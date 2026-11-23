//! Export a PDF ▸ Image ▸ Export all images, and Edit ▸ Save image as: the images that pages
//! use, as files. JPEG images are written unchanged; other images, and JPEG images with
//! transparency, are decoded and written as PNG with their soft mask, stencil mask or colour
//! key as alpha. Images PdfCraft can't decode yet (JPEG 2000, JBIG2, CCITT, separations) are
//! reported, never silently left out.

use std::collections::HashSet;

use pdfcraft_cos::{Dict, Document, ObjRef, Object, Stream};

/// One image, ready to write.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtractedImage {
    /// 0-based page the image was first found on.
    pub page: usize,
    pub object: ObjRef,
    pub width: u32,
    pub height: u32,
    /// "jpg" or "png".
    pub extension: &'static str,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ImageExport {
    pub images: Vec<ExtractedImage>,
    /// Images left out: (page, object, why).
    pub skipped: Vec<(usize, ObjRef, String)>,
}

/// Images larger than this many pixels are skipped (memory).
const MAX_PIXELS: u64 = 100_000_000;

/// The images of `pages` (0-based; each image once, on the first page using it), skipping
/// those with fewer than `min_side` pixels on their shorter side.
pub fn extract_images(doc: &Document, pages: &[usize], min_side: u32) -> ImageExport {
    let all = pdfcraft_model::pages(doc);
    let mut out = ImageExport::default();
    let mut seen = HashSet::new();
    for &p in pages {
        let Some(page) = all.get(p) else { continue };
        let mut refs = Vec::new();
        if let Some(res) = page.dict.get(b"Resources").map(|r| doc.resolve(r)).and_then(|r| r.as_dict().cloned()) {
            collect(doc, &res, 0, &mut HashSet::new(), &mut refs);
        }
        for r in refs {
            if !seen.insert(r) {
                continue;
            }
            let obj = doc.get(r);
            let Object::Stream(s) = &*obj else { continue };
            let (w, h) = (s.dict.int(b"Width").unwrap_or(0), s.dict.int(b"Height").unwrap_or(0));
            if w <= 0 || h <= 0 || (w.min(h) as u64) < min_side as u64 {
                continue;
            }
            match image(doc, s) {
                Ok((extension, data)) => out.images.push(ExtractedImage { page: p, object: r, width: w as u32, height: h as u32, extension, data }),
                Err(why) => out.skipped.push((p, r, why)),
            }
        }
    }
    out
}

/// One image XObject as a file: JPEG as is, else PNG ("Save image as").
pub fn image_file(doc: &Document, image: ObjRef) -> Result<(&'static str, Vec<u8>), String> {
    match &*doc.get(image) {
        Object::Stream(s) if s.dict.name(b"Subtype") == Some(b"Image") => self::image(doc, s),
        _ => Err("not an image".into()),
    }
}

/// The image XObjects reachable from `res`, through form XObjects, in resource order.
fn collect(doc: &Document, res: &Dict, depth: usize, forms: &mut HashSet<ObjRef>, out: &mut Vec<ObjRef>) {
    if depth > 12 {
        return;
    }
    let Some(xobjects) = res.get(b"XObject").map(|x| doc.resolve(x)).and_then(|x| x.as_dict().cloned()) else { return };
    for (_, v) in xobjects.iter() {
        let Some(r) = v.as_ref() else { continue };
        let obj = doc.get(r);
        let Object::Stream(s) = &*obj else { continue };
        match s.dict.name(b"Subtype") {
            Some(b"Image") => out.push(r),
            Some(b"Form") if forms.insert(r) => {
                if let Some(inner) = s.dict.get(b"Resources").map(|x| doc.resolve(x)).and_then(|x| x.as_dict().cloned()) {
                    collect(doc, &inner, depth + 1, forms, out);
                }
            }
            _ => {}
        }
    }
}

/// The JPEG codestream a `/DCTDecode` stream carries, with the general-purpose filters recorded in
/// front of that codec decoded off it. `chain` is `Stream::filters()`; the last entry is the codec.
///
/// `None` when the bytes in front cannot be read: another image codec (which is itself the thing
/// to export, not a wrapper), a damaged stage, or data that turns out not to be a JPEG at all. The
/// caller reports that rather than writing a file whose extension lies about it.
fn peel_jpeg(s: &Stream, chain: &[(pdfcraft_filters::Filter, pdfcraft_filters::Params)]) -> Option<Vec<u8>> {
    let (_, front) = chain.split_last()?;
    if front.iter().any(|(f, _)| f.is_image_codec()) {
        return None;
    }
    let bytes = if front.is_empty() {
        s.raw.to_vec()
    } else {
        // A wrapper around compressed data must not be allowed to expand without bound: the
        // decoded JPEG can be larger than the stream that held it, but not indefinitely.
        // Strict, not tolerant: a half-decoded wrapper would hand over a truncated JPEG, and a
        // file that opens partway is worse than a refusal the caller can report.
        let cap = s.raw.len().saturating_mul(8).max(1 << 20);
        pdfcraft_filters::decode(front, &s.raw, cap).ok()?
    };
    // SOI: the one thing every JPEG file begins with. A chain that decodes to anything else is not
    // a JPEG under a wrapper, and exporting it as `.jpg` would be a lie in the filename.
    bytes.starts_with(&[0xFF, 0xD8]).then_some(bytes)
}

/// The file for one image: its JPEG data as is, or a PNG. An image with transparency (a soft
/// mask, a stencil `/Mask` or a colour-key `/Mask`) is always a PNG with that transparency as
/// alpha, JPEG images included, so cut-out images keep their transparent background.
fn image(doc: &Document, s: &Stream) -> Result<(&'static str, Vec<u8>), String> {
    // JPEG data is a file of its own once the filters in front of `/DCTDecode` are peeled off
    // (§7.4: the chain runs in order, so those stages come before the codec). `FlateDecode` in
    // front of a JPEG is common — producers compress an already-compressed stream — and the JPEG
    // itself is still intact underneath.
    let chain = s.filters();
    // The codecs this build cannot decode are named first, so their reason is the one reported
    // rather than a peeling failure (`/Crypt` is dropped from `chain` — it is applied on load).
    match chain.last().map(|(f, _)| f.name()) {
        Some("JPXDecode") => return Err("JPEG 2000 images can't be exported yet".into()),
        Some("JBIG2Decode") => return Err("JBIG2 images can't be exported yet".into()),
        Some("CCITTFaxDecode") => return Err("CCITT fax images can't be exported yet".into()),
        _ => {}
    }
    // `coded` says a JPEG decoder is needed to read the samples (true whether or not the codec is
    // wrapped). `as_is_ok` says the stream's own bytes are already the JPEG file, which holds only
    // when nothing in front of the codec has to be peeled.
    let coded = chain.last().is_some_and(|(f, _)| *f == pdfcraft_filters::Filter::Dct);
    let data = if coded {
        match peel_jpeg(s, &chain) {
            Some(bytes) => bytes,
            None => return Err("the JPEG data inside these filters can't be recovered".into()),
        }
    } else {
        s.raw.to_vec()
    };
    let as_is_ok = coded && chain.len() == 1;
    let as_is = || Ok(("jpg", data.clone()));
    let (w, h) = (s.dict.int(b"Width").unwrap_or(0), s.dict.int(b"Height").unwrap_or(0));
    let Some((w, h)) = size(w, h) else {
        return match () {
            _ if as_is_ok => as_is(),
            _ if w <= 0 || h <= 0 => Err("the image has no size".into()),
            _ => Err(format!("{w} × {h} is too large to export")),
        };
    };
    let mask = s.dict.get(b"ImageMask").is_some_and(|m| matches!(&*doc.resolve(m), Object::Bool(true)));
    // A stencil mask (`ImageMask true`) has no /SMask or /Mask of its own (8.9.6.2).
    let mut alpha = if mask { None } else { soft_mask(doc, s, w, h).or_else(|| stencil_mask(doc, s, w, h)) };
    let key = if mask { None } else { colour_key(doc, s) };
    if as_is_ok && alpha.is_none() && key.is_none() {
        return as_is();
    }
    let mut space = if mask { Space::Gray } else { space(doc, s.dict.get(b"ColorSpace"))? };
    let (samples, bpc) = if coded {
        // A JPEG decoder gives grey or RGB (CMYK and YCCK are converted); a JPEG that doesn't
        // decode, or an indexed one (not valid), is written as is, without its transparency.
        let Some((samples, decoded)) = jpeg_samples(&data, &space, w, h) else { return as_is() };
        space = decoded;
        (samples, 8)
    } else {
        let data = s.decoded().map_err(|e| e.to_string())?;
        let bpc = if mask { 1 } else { s.dict.int(b"BitsPerComponent").unwrap_or(8) };
        let bpc = match bpc {
            1 => 1,
            2 => 2,
            4 => 4,
            8 => 8,
            16 => 16,
            other => return Err(format!("{other} bits per component isn't supported")),
        };
        (unpack(&data, w, h, space.components(), bpc).ok_or("the image data is shorter than its size says")?, bpc)
    };
    let n = space.components();
    if alpha.is_none() {
        alpha = key.and_then(|k| keyed_alpha(&samples, n, bpc, &k));
    }
    if as_is_ok && alpha.is_none() {
        return as_is();
    }
    // Decode arrays: only inversion ([1 0] per component) is honoured, the common case. A JPEG
    // decoder has already applied an Adobe JPEG's own inversion.
    let invert = s
        .dict
        .get(b"Decode")
        .map(|d| doc.resolve(d))
        .and_then(|d| d.as_array().map(|a| a.first().and_then(Object::as_f64) > a.get(1).and_then(Object::as_f64)));
    let invert = (invert.unwrap_or(false) && !coded) ^ mask; // a stencil mask paints its 0 samples
    let max = (1u32 << bpc.min(8)) - 1;
    let mut rgb = Vec::with_capacity(w * h * 3);
    for px in samples.chunks_exact(n) {
        let v = |i: usize| {
            let x = (px.get(i).copied().unwrap_or(0) as u32 * 255 / max) as u8;
            if invert { 255 - x } else { x }
        };
        match &space {
            Space::Gray => rgb.extend_from_slice(&[v(0); 3]),
            Space::Rgb => rgb.extend_from_slice(&[v(0), v(1), v(2)]),
            Space::Cmyk => rgb.extend_from_slice(&cmyk(v(0), v(1), v(2), v(3))),
            Space::Indexed(base, table) => {
                let i = px.first().copied().unwrap_or(0) as usize;
                let k = base.components();
                let e = table.get(i * k..i * k + k).unwrap_or(&[0, 0, 0, 0][..k]);
                rgb.extend_from_slice(&match **base {
                    Space::Gray => [e[0]; 3],
                    Space::Cmyk => cmyk(e[0], e[1], e[2], e[3]),
                    _ => [e[0], e[1], e[2]],
                });
            }
        }
    }
    if let (Some(a), Some(matte)) = (alpha.as_deref(), matte(doc, s, &space)) {
        unpremultiply(&mut rgb, a, matte);
    }
    png(w as u32, h as u32, &rgb, alpha.as_deref()).map(|p| ("png", p))
}

/// Width and height as sizes, when both are positive and the image is at most `MAX_PIXELS`.
fn size(w: i64, h: i64) -> Option<(usize, usize)> {
    let (w, h) = (usize::try_from(w).ok()?, usize::try_from(h).ok()?);
    let pixels = (w as u64).checked_mul(h as u64)?;
    (w > 0 && h > 0 && pixels <= MAX_PIXELS).then_some((w, h))
}

/// A JPEG's samples (8 bits): grey for a grey image, else RGB, with the space they are in.
fn jpeg_samples(data: &[u8], space: &Space, w: usize, h: usize) -> Option<(Vec<u8>, Space)> {
    if matches!(space, Space::Indexed(..)) {
        return None;
    }
    let img = image::load_from_memory_with_format(data, image::ImageFormat::Jpeg).ok()?;
    if img.width() as usize != w || img.height() as usize != h {
        return None;
    }
    Some(match space {
        Space::Gray => (img.into_luma8().into_raw(), Space::Gray),
        _ => (img.into_rgb8().into_raw(), Space::Rgb),
    })
}

/// The colour-key `/Mask` (8.9.6.4): a [min max] pair per colour component, in sample values.
fn colour_key(doc: &Document, s: &Stream) -> Option<Vec<(u32, u32)>> {
    let m = doc.resolve(s.dict.get(b"Mask")?);
    let a = m.as_array()?;
    let v: Vec<u32> = a.iter().map(|o| doc.resolve(o).as_f64().map(|f| f.clamp(0.0, 65_535.0) as u32)).collect::<Option<_>>()?;
    (!v.is_empty() && v.len().is_multiple_of(2)).then(|| v.as_chunks::<2>().0.iter().map(|&[lo, hi]| (lo, hi)).collect())
}

/// Alpha from a colour key: pixels whose every component falls in its range are transparent.
fn keyed_alpha(samples: &[u8], n: usize, bpc: usize, key: &[(u32, u32)]) -> Option<Vec<u8>> {
    if key.len() != n {
        return None;
    }
    // 16-bit samples keep their high byte (see `unpack`), so compare the ranges' high bytes.
    let shift = if bpc == 16 { 8 } else { 0 };
    Some(
        samples
            .chunks_exact(n)
            .map(|px| {
                let hidden = px.iter().zip(key).all(|(&c, &(lo, hi))| (lo >> shift..=hi >> shift).contains(&(c as u32)));
                if hidden { 0 } else { 255 }
            })
            .collect(),
    )
}

/// The `/Matte` colour of the image's soft mask (11.6.5.3), for grey and RGB images: the colour
/// the image samples were pre-blended with.
fn matte(doc: &Document, s: &Stream, space: &Space) -> Option<Vec<u8>> {
    let r = s.dict.get(b"SMask")?.as_ref()?;
    let obj = doc.get(r);
    let Object::Stream(m) = &*obj else { return None };
    let a = doc.resolve(m.dict.get(b"Matte")?);
    let v: Vec<u8> =
        a.as_array()?.iter().map(|o| doc.resolve(o).as_f64().map(|f| (f.clamp(0.0, 1.0) * 255.0).round() as u8)).collect::<Option<_>>()?;
    match (space, v.len()) {
        (Space::Gray, 1) => Some(vec![v[0]; 3]),
        (Space::Rgb, 3) => Some(v),
        _ => None,
    }
}

/// Undo a pre-blend with `matte`: c = m + (c′ − m) / α.
fn unpremultiply(rgb: &mut [u8], alpha: &[u8], matte: Vec<u8>) {
    for (px, &a) in rgb.as_chunks_mut::<3>().0.iter_mut().zip(alpha) {
        if a == 0 {
            continue;
        }
        for (c, &m) in px.iter_mut().zip(&matte) {
            let v = m as i32 + (*c as i32 - m as i32) * 255 / a as i32;
            *c = v.clamp(0, 255) as u8;
        }
    }
}

#[derive(Clone, Debug)]
enum Space {
    Gray,
    Rgb,
    Cmyk,
    /// Base space and lookup table (one byte per base component).
    Indexed(Box<Space>, Vec<u8>),
}

impl Space {
    fn components(&self) -> usize {
        match self {
            Space::Gray | Space::Indexed(..) => 1,
            Space::Rgb => 3,
            Space::Cmyk => 4,
        }
    }
}

fn space(doc: &Document, cs: Option<&Object>) -> Result<Space, String> {
    let Some(cs) = cs else { return Err("the image has no colour space".into()) };
    let cs = doc.resolve(cs);
    let unsupported = |n: &[u8]| Err(format!("{} images can't be exported yet", String::from_utf8_lossy(n)));
    match &*cs {
        Object::Name(n) => match n.as_slice() {
            b"DeviceGray" | b"G" | b"CalGray" => Ok(Space::Gray),
            b"DeviceRGB" | b"RGB" | b"CalRGB" => Ok(Space::Rgb),
            b"DeviceCMYK" | b"CMYK" => Ok(Space::Cmyk),
            other => unsupported(other),
        },
        Object::Array(a) => match a.first().and_then(Object::as_name) {
            Some(b"ICCBased") => match a.get(1).map(|s| doc.resolve(s)).as_deref() {
                Some(Object::Stream(icc)) => match icc.dict.int(b"N") {
                    Some(1) => Ok(Space::Gray),
                    Some(3) => Ok(Space::Rgb),
                    Some(4) => Ok(Space::Cmyk),
                    _ => Err("an ICC colour space with an unusual number of components".into()),
                },
                _ => Err("a damaged ICC colour space".into()),
            },
            Some(b"CalGray") => Ok(Space::Gray),
            Some(b"CalRGB") => Ok(Space::Rgb),
            Some(b"Indexed" | b"I") => {
                let base = space(doc, a.get(1))?;
                if matches!(base, Space::Indexed(..)) {
                    return Err("nested indexed colour spaces aren't valid".into());
                }
                let table = match a.get(3).map(|t| doc.resolve(t)).as_deref() {
                    Some(Object::String(s)) => s.bytes.clone(),
                    Some(Object::Stream(t)) => t.decoded().map_err(|e| e.to_string())?,
                    _ => return Err("an indexed colour space without a lookup table".into()),
                };
                Ok(Space::Indexed(Box::new(base), table))
            }
            Some(other) => unsupported(other),
            None => Err("a damaged colour space".into()),
        },
        _ => Err("a damaged colour space".into()),
    }
}

/// Samples as one byte each (16-bit samples keep their high byte).
fn unpack(data: &[u8], w: usize, h: usize, n: usize, bpc: usize) -> Option<Vec<u8>> {
    let row_bits = w * n * bpc;
    let row = row_bits.div_ceil(8);
    if data.len() < row * h {
        return None;
    }
    let mut out = Vec::with_capacity(w * h * n);
    for y in 0..h {
        let r = &data[y * row..(y + 1) * row];
        match bpc {
            8 => out.extend_from_slice(&r[..w * n]),
            16 => out.extend(r.as_chunks::<2>().0.iter().take(w * n).map(|c| c[0])),
            _ => {
                let per = 8 / bpc;
                let m = (1u8 << bpc) - 1;
                for i in 0..w * n {
                    let shift = 8 - bpc * (i % per + 1);
                    out.push((r[i / per] >> shift) & m);
                }
            }
        }
    }
    Some(out)
}

fn cmyk(c: u8, m: u8, y: u8, k: u8) -> [u8; 3] {
    let f = |x: u8| ((255 - x as u32) * (255 - k as u32) / 255) as u8;
    [f(c), f(m), f(y)]
}

/// The soft mask (`/SMask`) as 8-bit alpha, scaled to the image's size when it has another.
fn soft_mask(doc: &Document, s: &Stream, w: usize, h: usize) -> Option<Vec<u8>> {
    let r = s.dict.get(b"SMask")?.as_ref()?;
    let obj = doc.get(r);
    let Object::Stream(m) = &*obj else { return None };
    gray_plane(doc, m, w, h)
}

/// A stencil `/Mask` (an image mask stream, 8.9.6.3) as 8-bit alpha: its 1 samples (0 with
/// `/Decode [1 0]`) hide the image.
fn stencil_mask(doc: &Document, s: &Stream, w: usize, h: usize) -> Option<Vec<u8>> {
    let r = s.dict.get(b"Mask")?.as_ref()?;
    let obj = doc.get(r);
    let Object::Stream(m) = &*obj else { return None };
    Some(gray_plane(doc, m, w, h)?.into_iter().map(|v| 255 - v).collect())
}

/// A one-component mask image as 8-bit values (its `/Decode` inversion applied), scaled to
/// `w` × `h`. `None` when it can't be decoded (JPEG 2000, JBIG2 and CCITT masks).
fn gray_plane(doc: &Document, m: &Stream, w: usize, h: usize) -> Option<Vec<u8>> {
    let (mw, mh) = size(m.dict.int(b"Width")?, m.dict.int(b"Height")?)?;
    let stencil = m.dict.get(b"ImageMask").is_some_and(|v| matches!(&*doc.resolve(v), Object::Bool(true)));
    // A JPEG mask is read through a decoder, whether or not a filter wraps the codec: the wrapper
    // is peeled first, exactly as `image` does it, so a mask inside `/FlateDecode` is not lost.
    let chain = m.filters();
    let mut px = match chain.last().map(|(f, _)| f.name()) {
        Some("DCTDecode") => {
            // `peel_jpeg` takes the codec off the end itself; handing it the bytes of a wrapper
            // would give the decoder a Flate stream and quietly produce nothing.
            let data = peel_jpeg(m, &chain)?;
            let img = image::load_from_memory_with_format(&data, image::ImageFormat::Jpeg).ok()?.into_luma8();
            (img.width() as usize == mw && img.height() as usize == mh).then(|| img.into_raw())?
        }
        // Another image codec: the sample path below would decode its bytes as raw samples.
        Some("JPXDecode" | "JBIG2Decode" | "CCITTFaxDecode") => return None,
        _ => {
            let bpc = if stencil { 1 } else { m.dict.int(b"BitsPerComponent").unwrap_or(8) };
            let bpc = match bpc {
                1 => 1,
                2 => 2,
                4 => 4,
                8 => 8,
                16 => 16,
                _ => return None,
            };
            let max = (1u32 << bpc.min(8)) - 1;
            unpack(&m.decoded().ok()?, mw, mh, 1, bpc)?.into_iter().map(|v| (v as u32 * 255 / max) as u8).collect()
        }
    };
    let invert = m
        .dict
        .get(b"Decode")
        .map(|d| doc.resolve(d))
        .and_then(|d| d.as_array().map(|a| a.first().and_then(Object::as_f64) > a.get(1).and_then(Object::as_f64)))
        .unwrap_or(false);
    if invert {
        px.iter_mut().for_each(|v| *v = 255 - *v);
    }
    if (mw, mh) == (w, h) {
        return Some(px);
    }
    let img = image::GrayImage::from_raw(mw as u32, mh as u32, px)?;
    Some(image::imageops::resize(&img, w as u32, h as u32, image::imageops::FilterType::Triangle).into_raw())
}

fn png(w: u32, h: u32, rgb: &[u8], alpha: Option<&[u8]>) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, w, h);
        enc.set_depth(png::BitDepth::Eight);
        enc.set_compression(png::Compression::Fast);
        let data: Vec<u8> = match alpha {
            Some(a) => {
                enc.set_color(png::ColorType::Rgba);
                rgb.as_chunks::<3>().0.iter().zip(a).flat_map(|(c, a)| [c[0], c[1], c[2], *a]).collect()
            }
            None => {
                enc.set_color(png::ColorType::Rgb);
                rgb.to_vec()
            }
        };
        let mut wr = enc.write_header().map_err(|e| e.to_string())?;
        wr.write_image_data(&data).map_err(|e| e.to_string())?;
    }
    Ok(out)
}
