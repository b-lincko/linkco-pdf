//! Export a PDF ▸ Image (PNG, JPEG, TIFF) and Text (Acrobat's Export a PDF tool, first formats).
//!
//! On the desktop the export runs on a worker thread and reports progress in the notice bar;
//! on the web it runs in place and downloads the files.

use std::sync::{Arc, Mutex};

use egui::{Align, Layout};
use pdfcraft_engine::export::{ExportSource, Exporter, ImageFormat};

use crate::marks_ui::PageRange;
use crate::theme::{self, Tokens};
use crate::{PdfCraftApp, widgets};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportKind {
    Image,
    Text,
    /// Export all images: the images pages use, as files.
    AllImages,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExportDraft {
    pub dpi: f64,
    pub range: PageRange,
    pub format: ImageFormat,
    /// Export all images ▸ skip images with fewer pixels than this on their shorter side.
    pub min_side: u32,
}

impl Default for ExportDraft {
    fn default() -> Self {
        Self { dpi: 150.0, range: PageRange::default(), format: ImageFormat::Png, min_side: 0 }
    }
}

/// Progress of a background export: (done, total, final message once finished).
pub type ExportStatus = Arc<Mutex<Option<(usize, usize, Option<String>)>>>;

pub(crate) fn body(ui: &mut egui::Ui, app: &mut PdfCraftApp, t: &Tokens, kind: ExportKind) -> (bool, bool) {
    let count = app.active_ids().and_then(|(_, id)| app.session.get(id)).map(|d| d.info.pages.len()).unwrap_or(0);
    let d = &mut app.export_draft;
    ui.label(
        egui::RichText::new(match kind {
            ExportKind::Image => tl!("Export to Image"),
            ExportKind::Text => tl!("Export to Text"),
            ExportKind::AllImages => tl!("Export All Images"),
        })
        .font(theme::semibold(18.0)),
    );
    ui.add_space(8.0);
    if kind == ExportKind::Image {
        ui.horizontal(|ui| {
            ui.label(tl!("Resolution"));
            egui::ComboBox::from_id_salt("export-dpi")
                .selected_text(crate::i18n::fmt(tl!("{dpi} pixels/inch"), &[("dpi", &d.dpi.to_string())]))
                .show_ui(ui, |ui| {
                    for dpi in [72.0, 96.0, 150.0, 300.0, 600.0] {
                        ui.selectable_value(&mut d.dpi, dpi, crate::i18n::fmt(tl!("{dpi} pixels/inch"), &[("dpi", &dpi.to_string())]));
                    }
                });
        });
        ui.horizontal(|ui| {
            ui.label(tl!("Format"));
            let mut quality = match d.format {
                ImageFormat::Jpeg { quality } => quality,
                _ => 85,
            };
            egui::ComboBox::from_id_salt("export-format").selected_text(d.format.label()).show_ui(ui, |ui| {
                for f in [ImageFormat::Png, ImageFormat::Jpeg { quality }, ImageFormat::Tiff] {
                    let on = std::mem::discriminant(&d.format) == std::mem::discriminant(&f);
                    if ui.selectable_label(on, f.label()).clicked() {
                        d.format = f;
                    }
                }
            });
            if let ImageFormat::Jpeg { .. } = d.format {
                ui.label(tl!("Quality"));
                if ui.add(egui::Slider::new(&mut quality, 10..=100)).changed() {
                    d.format = ImageFormat::Jpeg { quality };
                }
            }
        });
        ui.label(
            egui::RichText::new(crate::i18n::fmt(tl!("One {f} file per page, named after the document."), &[("f", d.format.label())]))
                .small()
                .color(t.text_faint),
        );
    } else if kind == ExportKind::AllImages {
        ui.horizontal(|ui| {
            ui.label(tl!("Exclude images smaller than"));
            let label = move |n: u32| if n == 0 { tl!("No limit").to_string() } else { format!("{n} pixels") };
            egui::ComboBox::from_id_salt("export-min").selected_text(label(d.min_side)).show_ui(ui, |ui| {
                for n in [0, 16, 32, 64, 128, 256] {
                    ui.selectable_value(&mut d.min_side, n, label(n));
                }
            });
        });
        ui.label(
            egui::RichText::new(tl!("Each image once, named after the document and page. JPEG images are saved unchanged; others as PNG."))
                .small()
                .color(t.text_faint),
        );
    } else {
        ui.label(egui::RichText::new(tl!("Plain text in reading order; pages are separated by form feeds.")).small().color(t.text_faint));
    }
    ui.add_space(6.0);
    ui.label(egui::RichText::new(tl!("Pages")).font(theme::semibold(12.5)));
    d.range.ui(ui, count);
    ui.add_space(12.0);
    let (mut apply, mut cancel) = (false, false);
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        let ok = !d.range.pages(count).is_empty();
        if ui.add_enabled_ui(ok, |ui| widgets::pill_button(ui, tl!("Export"), true)).inner.clicked() {
            apply = true;
        }
        if widgets::pill_button(ui, tl!("Cancel"), false).clicked() {
            cancel = true;
        }
    });
    (apply, cancel)
}

/// The export's name in the card: the title the dialog already uses, so no new string is needed.
fn export_job_name(kind: ExportKind) -> String {
    match kind {
        ExportKind::Image => tl!("Export to Image").to_string(),
        ExportKind::Text => tl!("Export to Text").to_string(),
        ExportKind::AllImages => tl!("Export All Images").to_string(),
    }
}

/// Write the files (`name`, bytes) produced for `pages`, reporting progress.
#[allow(clippy::too_many_arguments)]
fn run(
    src: ExportSource,
    kind: ExportKind,
    dpi: f64,
    format: ImageFormat,
    min_side: u32,
    pages: Vec<usize>,
    stem: String,
    mut sink: impl FnMut(&str, Vec<u8>) -> Result<(), String>,
    status: &ExportStatus,
    lang: crate::i18n::Lang,
    cancel: &crate::jobs::Cancel,
) -> String {
    // May run on a worker thread: messages are drawn in the UI's language.
    crate::i18n::set_current(lang);
    let total = pages.len();
    let set = |done: usize, msg: Option<String>| {
        if let Ok(mut s) = status.lock() {
            *s = Some((done, total, msg));
        }
    };
    // A stop is answered between pages and images, like every other batch (see crate::jobs).
    // The wrapper below appends " to {dir}", so this reads as one sentence.
    let stopped = |done: usize| {
        crate::i18n::fmt(tl!("Export stopped after {done} of {total} files"), &[("done", &done.to_string()), ("total", &total.to_string())])
    };
    if kind == ExportKind::AllImages {
        set(0, None);
        let out = match pdfcraft_engine::export::extract_images(&src, &pages, min_side) {
            Ok(o) => o,
            Err(e) => return crate::i18n::fmt(tl!("Export stopped: {e}"), &[("e", &e.to_string())]),
        };
        for (k, img) in out.images.iter().enumerate() {
            if cancel.is_cancelled() {
                return stopped(k);
            }
            set(k, None);
            if let Err(e) = sink(&pdfcraft_engine::export::image_file_name(&stem, img, k + 1), img.data.clone()) {
                return crate::i18n::fmt(tl!("Export stopped: {e}"), &[("e", &e.to_string())]);
            }
        }
        let n = out.images.len();
        let mut msg = if n == 1 {
            crate::i18n::fmt(tl!("Exported 1 image"), &[])
        } else {
            crate::i18n::fmt(tl!("Exported {n} images"), &[("n", &n.to_string())])
        };
        if !out.skipped.is_empty() {
            msg.push_str(&crate::i18n::fmt(
                tl!(" ({s} not exported: {first})"),
                &[("s", &out.skipped.len().to_string()), ("first", &out.skipped[0].2)],
            ));
        }
        return msg;
    }
    let mut ex = Exporter::from_source(src);
    match kind {
        // Handled above.
        ExportKind::AllImages => String::new(),
        ExportKind::Image => {
            for (k, p) in pages.iter().enumerate() {
                if cancel.is_cancelled() {
                    return stopped(k);
                }
                set(k, None);
                let result = ex.image(*p, dpi, format).and_then(|img| sink(&format!("{stem}_page_{}.{}", p + 1, format.extension()), img));
                if let Err(e) = result {
                    return crate::i18n::fmt(tl!("Export stopped: {e}"), &[("e", &e.to_string())]);
                }
            }
            let mut msg = if total == 1 {
                crate::i18n::fmt(tl!("Exported 1 image"), &[])
            } else {
                crate::i18n::fmt(tl!("Exported {n} images"), &[("n", &total.to_string())])
            };
            // Pages too large for the renderer at the chosen resolution are drawn at the most it
            // allows: say so, rather than leave a smaller image unexplained.
            let lowered: Vec<f64> = pages.iter().map(|p| ex.dpi_used(*p, dpi)).filter(|used| *used < dpi.clamp(18.0, 1200.0) - 0.5).collect();
            if let Some(lowest) = lowered.iter().copied().reduce(f64::min) {
                msg.push_str(&crate::i18n::fmt(
                    tl!(" ({n} at {dpi} dpi, the largest size they can be drawn at)"),
                    &[("n", &lowered.len().to_string()), ("dpi", &format!("{}", lowest.floor()))],
                ));
            }
            msg
        }
        ExportKind::Text => {
            set(0, None);
            match ex.text_of(&pages).and_then(|text| sink(&format!("{stem}.txt"), text.into_bytes())) {
                Ok(()) => {
                    if total == 1 {
                        crate::i18n::fmt(tl!("Exported the text of 1 page"), &[])
                    } else {
                        crate::i18n::fmt(tl!("Exported the text of {n} pages"), &[("n", &total.to_string())])
                    }
                }
                Err(e) => crate::i18n::fmt(tl!("Export stopped: {e}"), &[("e", &e.to_string())]),
            }
        }
    }
}

impl PdfCraftApp {
    /// Start exporting the active document with the dialog's settings.
    pub(crate) fn start_export(&mut self, kind: ExportKind) {
        // What's typed in a form field is part of the document (#166).
        if !self.commit_form_typing() {
            return;
        }
        let Some((_, id)) = self.active_ids() else { return };
        let Some(doc) = self.session.get(id) else { return };
        let src = doc.export_source();
        let stem = doc.name.trim_end_matches(".pdf").trim_end_matches(".PDF").to_string();
        let pages = self.export_draft.range.pages(src.pages);
        let dpi = self.export_draft.dpi;
        let format = self.export_draft.format;
        let min_side = self.export_draft.min_side;
        let status: ExportStatus = Arc::new(Mutex::new(Some((0, pages.len(), None))));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let start = move |app: &mut Self, dir: std::path::PathBuf, inline: bool| {
                // Registered here, not before: until the folder is chosen there is nothing to stop.
                let (job, cancel) = app.jobs.begin(crate::jobs::Kind::Export, export_job_name(kind), pages.len());
                app.export_job = Some(job);
                let st = status.clone();
                let shown = dir.display().to_string();
                let lang = crate::i18n::current();
                let stop = cancel.clone();
                let work = move || {
                    let sink = |name: &str, bytes: Vec<u8>| {
                        crate::editing::write_atomically(&dir.join(name).to_string_lossy(), &bytes).map_err(|e| format!("{name}: {e}"))
                    };
                    let msg = run(src, kind, dpi, format, min_side, pages, stem, sink, &st, lang, &stop);
                    if let Ok(mut s) = st.lock() {
                        let (done, total) = s.as_ref().map_or((0, 0), |(d, t, _)| (*d, *t));
                        let done_msg = crate::i18n::fmt(tl!("{msg} to {dir}"), &[("msg", &msg), ("dir", &shown)]);
                        *s = Some((done.max(total), total, Some(done_msg)));
                    }
                };
                if inline {
                    work(); // tests and automation: synchronous
                } else {
                    std::thread::Builder::new().name("pdfcraft-export".into()).spawn(work).ok();
                }
                app.export_status = Some(status);
            };
            match self.export_dir_override.clone() {
                Some(d) => start(self, d.into(), true),
                None => {
                    let dialog = rfd::AsyncFileDialog::new().set_title(tl!("Choose a folder for the exported files").to_string());
                    self.ask_one(crate::pickers::Ask::Folder(dialog), None, move |app, dir| start(app, dir, false));
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let (job, cancel) = self.jobs.begin(crate::jobs::Kind::Export, export_job_name(kind), pages.len());
            self.export_job = Some(job);
            let msg = run(
                src,
                kind,
                dpi,
                format,
                min_side,
                pages,
                stem,
                |name, bytes| crate::editing::download(name, &bytes),
                &status,
                crate::i18n::current(),
                &cancel,
            );
            if let Ok(mut s) = status.lock() {
                *s = Some((0, 0, Some(msg)));
            }
            self.export_status = Some(status);
        }
    }

    /// Show export progress, and the result once it is done.
    pub(crate) fn poll_export(&mut self) {
        let Some(st) = self.export_status.clone() else { return };
        let snapshot = st.lock().ok().and_then(|s| s.clone());
        let job = self.export_job;
        match snapshot {
            Some((_, _, Some(msg))) => {
                self.export_status = None;
                self.export_job = None;
                if let Some(job) = job {
                    self.jobs.finish(job);
                }
                self.notify(msg);
            }
            Some((done, total, None)) => {
                if let Some(job) = job {
                    let stage = crate::i18n::fmt(tl!("Page {p} of {n}"), &[("p", &done.to_string()), ("n", &total.max(1).to_string())]);
                    self.jobs.stage(job, stage, if total == 0 { 0.0 } else { done as f32 / total as f32 });
                }
            }
            _ => {
                self.export_status = None;
                self.export_job = None;
                if let Some(job) = job {
                    self.jobs.finish(job);
                }
            }
        }
        if let Some(ctx) = &self.ctx {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two empty pages: enough for the export to name and draw, and nothing that needs a font.
    const FIXTURE: &[u8] = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 /MediaBox [0 0 200 100] >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R >> endobj\n4 0 obj << /Type /Page /Parent 2 0 R >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";

    fn source() -> ExportSource {
        ExportSource {
            bytes: std::sync::Arc::new(FIXTURE.to_vec()),
            config: pdfcraft_render::RenderConfig::default(),
            pages: 2,
            sizes: vec![(200.0, 100.0), (200.0, 100.0)],
        }
    }

    /// The export answers a stop between pages: the page in hand is written whole, the next one
    /// is not started, and the message says how far it got (in the sentence the toast wraps).
    #[test]
    fn a_stop_between_pages_keeps_the_page_in_hand_and_starts_no_other() {
        let stop = crate::jobs::Cancel::new();
        let mut written = Vec::new();
        let mut sink_stop = stop.clone();
        let status: ExportStatus = std::sync::Arc::new(std::sync::Mutex::new(Some((0, 2, None))));
        let msg = run(
            source(),
            ExportKind::Image,
            72.0,
            ImageFormat::Png,
            0,
            vec![0, 1],
            "doc".into(),
            |name, bytes| {
                assert!(bytes.starts_with(b"\x89PNG"), "{name} is not a page image");
                written.push(name.to_string());
                // The reader presses Cancel while the first page is being written.
                sink_stop.cancel();
                Ok(())
            },
            &status,
            crate::i18n::current(),
            &stop,
        );
        assert_eq!(written, vec!["doc_page_1.png"], "no page starts after the stop");
        assert_eq!(msg, "Export stopped after 1 of 2 files");
        let (done, total, _) = status.lock().unwrap().clone().unwrap();
        assert_eq!((done, total), (1, 2), "the card is told how far it got");
    }

    /// A stop that arrives before the first page writes nothing at all.
    #[test]
    fn a_stop_before_the_first_page_writes_nothing() {
        let stop = crate::jobs::Cancel::new();
        stop.cancel();
        let status: ExportStatus = std::sync::Arc::new(std::sync::Mutex::new(Some((0, 2, None))));
        let mut written = 0;
        let msg = run(
            source(),
            ExportKind::Image,
            72.0,
            ImageFormat::Png,
            0,
            vec![0, 1],
            "doc".into(),
            |_, _| {
                written += 1;
                Ok(())
            },
            &status,
            crate::i18n::current(),
            &stop,
        );
        assert_eq!(written, 0);
        assert_eq!(msg, "Export stopped after 0 of 2 files");
    }
}
