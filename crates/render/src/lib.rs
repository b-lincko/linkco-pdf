//! pdfcraft-render — page rasterization and document inspection.
//!
//! **Bootstrap status (see ADR-0004):** rasterization goes straight through the `hayro` crate
//! (hayro-interpret + vello_cpu). Inspection uses the lazy `cos` reader with a read-only lopdf
//! object adapter and retains `lopdf` loading for compatibility repairs. The renderer and that
//! adapter are replaced by `model` and the DisplayList design in M1–M2. The public API is what the engine and UI rely on, so
//! the swap stays internal to this crate.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod inspect;
mod pixels;
mod raster;
mod structure;
pub mod text;

pub use inspect::{
    Annotation, Attachment, AttachmentSource, DestView, DocInfo, Field, FieldKind, FontInfo, Layer, LayerOp, Link, LinkTarget, OutlineItem, PageInfo,
    Xfa, attachment_data, inspect, pretty_date,
};
pub use pixels::Pixels;
pub use raster::{
    MAX_PIXELS, MAX_SIDE, PageRenderer, RenderConfig, RenderPool, RenderRequest, RenderStats, RenderWarning, RenderedPage, RequestKind, Retiring,
    Tile, device_pixels, effective_scale,
};
pub use text::{PageText, TextGlyph};

/// Errors surfaced to the user when a document cannot be opened.
///
/// The variants say *whose* problem the failure is, because a caller has to act differently on
/// each: `Invalid` is the file (it is not a document, or is too damaged to be one), `NeedsPassword`
/// and `WrongPassword` are the caller's to fix, `Empty` is a real document with nothing in it,
/// `Unsupported` is this build's limit, and `Internal` is ours to fix — nothing about the file is
/// being claimed when that one is raised.
#[derive(Debug, thiserror::Error)]
pub enum OpenError {
    #[error("the file is not a readable PDF: {0}")]
    Invalid(String),
    #[error("the document is protected by a password")]
    NeedsPassword,
    #[error("the password is incorrect")]
    WrongPassword,
    #[error("the document has no pages")]
    Empty,
    #[error("{0}")]
    Unsupported(String),
    #[error("reading it failed unexpectedly: {0}")]
    Internal(String),
}
