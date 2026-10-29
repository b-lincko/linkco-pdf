//! View ▸ Zoom ▸ Marquee Zoom (drag a rectangle to fill the window with it; click to zoom in)
//! and Edit ▸ Take a Snapshot (drag a rectangle to copy that area as an image).
//!
//! A snapshot is rendered at the same scale the page is drawn on screen
//! ([`crate::canvas::render_scale`]: zoom × 96/72 × pixels per point), so the copy has one image
//! pixel for every screen pixel and nothing is resampled. Rendering it any smaller — which it did:
//! zoom × pixels per point, missing the 96/72 factor — makes a copy that looks soft however sharp
//! the page is on screen.

use egui::{Color32, CornerRadius, Pos2, Rect, Stroke};
use pdfcraft_render::{RenderConfig, RenderRequest, RequestKind, Tile};

use crate::canvas::{DocView, PageXform};
use crate::{PdfCraftApp, QuickTool};

/// The renderer draws one tile of at most this many device pixels a side (it clamps a larger
/// request), so a snapshot of a bigger area is copied at a lower scale rather than cropped.
const TILE_SIDE: f32 = 4096.0;
/// A snapshot is never smaller than this many device pixels per point (36 dpi), so one taken
/// while zoomed right out is still an image of the page rather than a few dozen pixels.
const MIN_SCALE: f32 = 0.5;

/// A finished gesture: (page, the rectangle on screen, the same in page view points
/// [x0, y0, x1, y1]); a plain click has an empty rectangle.
pub type Marquee = (usize, Rect, [f32; 4]);

/// Drag a rectangle on page `page` (Marquee Zoom, Snapshot).
pub(crate) fn page_input(ui: &egui::Ui, resp: &egui::Response, xf: &PageXform, page: usize, view: &mut DocView) {
    let pointer = ui.input(|i| i.pointer.hover_pos());
    let origin = ui.input(|i| i.pointer.press_origin());
    if pointer.is_some_and(|p| xf.rect.contains(p)) {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
    }
    if resp.drag_started()
        && let Some(o) = origin.filter(|o| xf.rect.contains(*o))
    {
        view.marquee = Some((page, o));
    }
    if let Some((p, start)) = view.marquee
        && p == page
        && let Some(end) = pointer
    {
        let end = Pos2::new(end.x.clamp(xf.rect.left(), xf.rect.right()), end.y.clamp(xf.rect.top(), xf.rect.bottom()));
        let r = Rect::from_two_pos(start, end);
        ui.painter().rect_stroke(r, CornerRadius::ZERO, Stroke::new(1.0, Color32::from_rgb(0x14, 0x73, 0xE6)), egui::StrokeKind::Middle);
        if resp.drag_stopped() {
            view.marquee = None;
            let (a, b) = (xf.screen_to_view(r.left_top()), xf.screen_to_view(r.right_bottom()));
            view.marquee_done = Some((page, r, [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)]));
        }
    }
    if resp.clicked()
        && let Some(p) = pointer.filter(|p| xf.rect.contains(*p))
    {
        let v = xf.screen_to_view(p);
        view.marquee_done = Some((page, Rect::from_min_size(p, egui::Vec2::ZERO), [v.0, v.1, v.0, v.1]));
    }
}

impl PdfCraftApp {
    /// Finish a marquee gesture for the current tool.
    pub(crate) fn finish_marquee(&mut self, index: usize, done: Marquee) {
        let (page, rect, view_rect) = done;
        match self.quick_tool {
            QuickTool::MarqueeZoom => {
                let view = &mut self.views[index];
                if rect.width() < 6.0 || rect.height() < 6.0 {
                    // A click zooms in one step around the point.
                    let z = (view.zoom * 1.5).min(64.0);
                    view.zoom_at(z, rect.center());
                } else {
                    view.zoom_to_rect(rect);
                }
            }
            QuickTool::Snapshot => {
                if view_rect[2] - view_rect[0] < 2.0 || view_rect[3] - view_rect[1] < 2.0 {
                    return;
                }
                match self.snapshot(index, page, view_rect) {
                    Ok((w, h)) => {
                        self.notify_fmt("The selected area has been copied ({w} × {h} pixels)", &[("w", &w.to_string()), ("h", &h.to_string())])
                    }
                    Err(e) => self.notify_fmt("Couldn't take the snapshot: {e}", &[("e", &e.to_string())]),
                }
            }
            _ => {}
        }
    }

    /// View ▸ Zoom ▸ Fit Visible on the current page: find where the page has ink (a quick
    /// low-resolution render) and zoom so that spans the window's width. A blank page fits the
    /// width.
    pub fn fit_visible(&mut self, index: usize) -> Result<(), String> {
        let view = &self.views[index];
        let page = view.current;
        let doc = self.session.get(view.id).ok_or("no document")?;
        let config = RenderConfig { password: doc.password.as_deref().map(std::sync::Arc::from), ..RenderConfig::default() };
        let mut r = pdfcraft_render::PageRenderer::new(doc.bytes.clone(), config);
        // About 800 px along the longer side is plenty to find the margins.
        let size = doc.info.pages.get(page).map_or(792.0, |p| p.width.max(p.height));
        let out = r.render(RenderRequest { page, kind: RequestKind::Pixels, tile: None, scale: (800.0 / size).clamp(0.1, 4.0), tag: 0 });
        if let Some(e) = out.error {
            return Err(e);
        }
        let rot = view.rotation;
        let view = &mut self.views[index];
        match ink_bounds(out.width as usize, out.height as usize, &out.rgba) {
            Some([u0, v0, u1, v1]) => {
                // View space → the rotated view on screen.
                let rotate = |u: f32, v: f32| match rot {
                    90 => (1.0 - v, u),
                    180 => (1.0 - u, 1.0 - v),
                    270 => (v, 1.0 - u),
                    _ => (u, v),
                };
                let (a, b) = (rotate(u0, v0), rotate(u1, v1));
                if !view.fit_content(page, [a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)]) {
                    view.fit = crate::canvas::Fit::Width;
                }
            }
            None => view.fit = crate::canvas::Fit::Width,
        }
        Ok(())
    }

    /// Render `view_rect` (page view points) of `page` at the scale it is drawn on screen and copy
    /// it to the clipboard (and keep it in `last_snapshot`).
    pub fn snapshot(&mut self, index: usize, page: usize, view_rect: [f32; 4]) -> Result<(u32, u32), String> {
        let view = &self.views[index];
        let doc = self.session.get(view.id).ok_or("no document")?;
        let ppp = self.ctx.as_ref().map_or(2.0, |c| c.pixels_per_point());
        let (w_pt, h_pt) = (view_rect[2] - view_rect[0], view_rect[3] - view_rect[1]);
        if !(w_pt.is_finite() && h_pt.is_finite() && view_rect.iter().all(|v| v.is_finite())) || w_pt < 1.0 || h_pt < 1.0 {
            return Err("the selected area is too small to copy".into());
        }
        let scale = snapshot_scale(view.zoom, ppp, w_pt, h_pt);
        let tile = Tile {
            x: (view_rect[0] * scale).floor().max(0.0) as u32,
            y: (view_rect[1] * scale).floor().max(0.0) as u32,
            w: ((w_pt * scale).ceil().max(1.0) as u32).min(TILE_SIDE as u32),
            h: ((h_pt * scale).ceil().max(1.0) as u32).min(TILE_SIDE as u32),
        };
        let config = RenderConfig { password: doc.password.as_deref().map(std::sync::Arc::from), ..RenderConfig::default() };
        let mut r = pdfcraft_render::PageRenderer::new(doc.bytes.clone(), config);
        let out = r.render(RenderRequest { page, kind: RequestKind::Pixels, tile: Some(tile), scale, tag: 0 });
        if let Some(e) = out.error {
            return Err(e);
        }
        let (w, h) = (out.width, out.height);
        #[cfg(not(target_arch = "wasm32"))]
        if self.system_clipboard {
            let img = arboard::ImageData { width: w as usize, height: h as usize, bytes: std::borrow::Cow::Borrowed(&out.rgba) };
            arboard::Clipboard::new().and_then(|mut c| c.set_image(img)).map_err(|e| e.to_string())?;
        }
        self.last_snapshot = Some((w, h, out.rgba));
        Ok((w, h))
    }
}

/// The scale a snapshot of a `w`×`h` point area renders at, in device pixels per point.
///
/// Exactly the scale the page is drawn on screen, so that the copy is one image pixel per screen
/// pixel: [`crate::canvas::render_scale`] is `zoom * PT * pixels_per_point`, and a copy rendered at
/// `zoom * pixels_per_point` (what this used to do) held 3/4 of the pixels of the same area on
/// screen and was stretched back up wherever it was pasted — the soft snapshot.
///
/// It is lowered only for an area that would not fit one renderer tile ([`TILE_SIDE`] a side),
/// so a very large selection is copied whole at a lower resolution rather than cropped.
fn snapshot_scale(zoom: f32, ppp: f32, w: f32, h: f32) -> f32 {
    let screen = zoom * crate::canvas::PT * ppp;
    if !screen.is_finite() || screen <= 0.0 {
        return MIN_SCALE;
    }
    // A non-finite or empty side cannot happen (the caller checks), but never divide by it.
    let longest = if w.is_finite() && h.is_finite() { w.max(h).max(1.0) } else { return screen.max(MIN_SCALE) };
    screen.max(MIN_SCALE).min(TILE_SIDE / longest)
}

/// The normalised bounds [u0, v0, u1, v1] of the pixels that aren't (near) white, if any.
pub(crate) fn ink_bounds(w: usize, h: usize, rgba: &[u8]) -> Option<[f32; 4]> {
    if w == 0 || h == 0 || rgba.len() < w * h * 4 {
        return None;
    }
    let ink = |p: &[u8]| p[3] > 16 && (p[0] < 240 || p[1] < 240 || p[2] < 240);
    let (mut x0, mut y0, mut x1, mut y1) = (w, h, 0, 0);
    for (y, row) in rgba.chunks_exact(w * 4).take(h).enumerate() {
        for (x, p) in row.as_chunks::<4>().0.iter().enumerate() {
            if ink(p) {
                x0 = x0.min(x);
                x1 = x1.max(x + 1);
                y0 = y0.min(y);
                y1 = y1.max(y + 1);
            }
        }
    }
    (x1 > x0 && y1 > y0).then(|| [x0 as f32 / w as f32, y0 as f32 / h as f32, x1 as f32 / w as f32, y1 as f32 / h as f32])
}

#[cfg(test)]
mod tests {
    /// One image pixel per screen pixel: the copy must not be resampled, which is what made
    /// snapshots look soft while the page on screen was sharp.
    #[test]
    fn a_snapshot_is_rendered_at_the_scale_the_page_is_drawn_at() {
        for (zoom, ppp) in [(1.0, 1.0), (1.0, 2.0), (0.5, 1.0), (2.0, 1.5), (4.0, 3.0)] {
            let want = zoom * crate::canvas::PT * ppp;
            assert!((super::snapshot_scale(zoom, ppp, 120.0, 80.0) - want).abs() < 1e-6, "zoom {zoom}, {ppp} px/point: want {want}");
        }
        // The old formula (zoom × pixels per point) was 96/72 short: a third fewer pixels.
        assert!((super::snapshot_scale(1.0, 1.0, 120.0, 80.0) / 1.0 - 4.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn a_large_area_is_copied_at_a_lower_scale_rather_than_cropped() {
        // 2000 × 1500 points at 8 device pixels per point would be 16000 × 12000: the renderer
        // draws at most 4096 pixels a side, so the scale comes down until both sides fit.
        let scale = super::snapshot_scale(6.0, 1.0, 2000.0, 1500.0);
        assert!((2000.0 * scale - super::TILE_SIDE).abs() < 1.0, "{}", 2000.0 * scale);
        assert!(1500.0 * scale <= super::TILE_SIDE + 1.0);
    }

    #[test]
    fn a_snapshot_never_goes_below_half_a_pixel_per_point() {
        assert_eq!(super::snapshot_scale(0.08, 1.0, 600.0, 800.0), super::MIN_SCALE);
        // A degenerate zoom cannot produce a zero, NaN or infinite scale.
        for zoom in [0.0, -3.0, f32::NAN, f32::INFINITY] {
            let scale = super::snapshot_scale(zoom, 2.0, 600.0, 800.0);
            assert!(scale.is_finite() && scale >= super::MIN_SCALE, "zoom {zoom}: {scale}");
        }
    }

    #[test]
    fn ink_bounds_finds_the_marked_area() {
        let (w, h) = (10, 20);
        let mut px = vec![255u8; w * h * 4];
        for (x, y) in [(2, 5), (7, 15)] {
            px[(y * w + x) * 4] = 0;
        }
        assert_eq!(super::ink_bounds(w, h, &px), Some([0.2, 0.25, 0.8, 0.8]));
        assert_eq!(super::ink_bounds(w, h, &vec![255u8; w * h * 4]), None);
    }
}
