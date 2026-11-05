# Complete baseline gap register

Snapshot: `1d7f56366160011e5bf9e370646bf1b45ba5fc79`. These **384** entries comprise all **308 planned** and **76 partial** entries in the 833-item repository checklist. These are repository assertions, not fresh UI verification.

No out-of-scope (`na`) item is silently counted as missing. The 23 `na` entries and all 426 shipped entries remain visible in [the complete baseline CSV](baseline-features.csv). See the main report for known contradictions and commercial scope decisions.


## core (14)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| core.image-filters | Image codecs in the filters crate (DCT, JPX, JBIG2, CCITT) | partial / P0 | decoded through the hayro bootstrap for display; own codecs are M1.1/M1.2 |
| core.attachment-only-encryption | Attachment-only encryption (/EFF) opens without a password | partial / P1 | two corpus files still ask for a password (M1.6) |
| core.open-public-key | Open certificate-encrypted (public-key handler) documents | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.deterministic-save | Deterministic full-save mode | planned / P2 | M1.9 option |
| core.linearization | Linearized save (Fast Web View) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.lazy-loading | Lazy loading of very large (GB) files within a memory budget | planned / P0 | ByteSource (M1.5); files are buffered whole today |
| core.pdf20-utf8-strings | PDF 2.0 UTF-8 text strings | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.pdf20-associated-files | PDF 2.0 associated files (/AF) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.pdf20-dpart | PDF 2.0 document parts (DPart) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| core.pdf20-namespaces | PDF 2.0 structure namespaces | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.arlington-validation | Structural validation against the Arlington PDF model | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.history-panel | History panel with named snapshots | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| core.web-recovery | Crash recovery in the web app | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| core.usage-rights-preserve | Preserve usage-rights (UR3) signatures from Reader-extended files | planned / P3 | preserve only |

## view (40)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| view.render-pages | Render pages (text, vectors, images) | partial / P0 | hayro bootstrap renderer; own DisplayList devices in M2.6. Pages with /UserUnit are shown at their size in points, as Acrobat reads the value |
| view.render-shadings-patterns | Shadings, gradients and tiling patterns | partial / P0 | via hayro bootstrap |
| view.render-transparency | Blend modes, soft masks and transparency groups | partial / P0 | via hayro bootstrap |
| view.render-type3 | Type 3 fonts | partial / P0 | hayro bootstrap with vendored patches |
| view.render-cid-fonts | CID fonts and CMaps incl. CJK without ToUnicode | partial / P0 | CID-to-Unicode tables are M2.4; a non-embedded Adobe-Japan1 font is drawn in a craft-fonts Japanese face (Mincho or Gothic by name; BIZ UDMincho for Mincho, which covers half-width katakana) when built with craft-fonts; non-embedded GB1, CNS1 and Korea1 fonts still draw nothing (no allowed face) |
| view.render-image-codecs | JBIG2, JPEG 2000, CCITT and JPEG images | partial / P0 | hayro bootstrap |
| view.deep-zoom-tiles | Tiled rendering keeps deep zoom sharp | partial / P0 | capped at 8192 px per side (M3.3) |
| view.page-gaps | Show or hide gaps between pages | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.auto-scroll | Automatic scrolling | partial / P2 | Linux-only middle-button vertical autoscroll in the document and Organize Pages; Windows and macOS pan 1:1 while the middle button is held (Grabbing cursor, no latch). On every target the middle button never reaches the page tools, so it scrolls instead of drawing, selecting, cropping or moving. Click to toggle, release keeps scrolling on, and a Chromium-style nonlinear curve increases speed with vertical screen distance from the initial click. A 15-point dead zone pauses immediately. Continuous redraws use elapsed frame time and preserve fractional movement, tested at 30/60/120/144 fps. Cancels on another middle click, Escape, other pointer/wheel input, focus loss, tab changes and dialogs. Driven headlessly through the opt-in UI control channel (ui.click/ui.move/ui.drag). Keyboard-started timed scrolling is not implemented. |
| view.dynamic-zoom | Dynamic zoom (drag to zoom) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.loupe | Loupe magnifier tool | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.pan-and-zoom | Pan and zoom overview window | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.full-screen-presentation | Full-screen presentation options (auto-advance, loop, click to advance) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.reflow | Reflow text to the window width | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.multiple-windows | Multiple windows; drag a tab out into a window | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| view.split-window | Split window (two panes on one document) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.spreadsheet-split | Spreadsheet split (four synchronised panes) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.highlight-current-bookmark | Highlight the current bookmark | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.layers-reset | Reset layers to initial visibility; list for all or current pages | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.destinations-panel | Named destinations panel | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.articles-panel | Articles panel and article reading | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| view.model-tree | 3D model tree panel | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| view.find-in-comments-bookmarks | Find in comments and bookmarks | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.advanced-search | Advanced search window | partial / P1 | in the current document, as a side panel; searching other open documents, folders and indexes is still to come |
| view.search-folder | Search all PDFs in a folder | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| view.search-boolean | Boolean, proximity and stemming search | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.search-index | Build and search a full-text index across files | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.embedded-index | Embedded search index | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.copy-with-layout | Copy text keeping layout | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.rulers | Rulers | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.grid | Grid and snap to grid | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.guides | Guides | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.line-weights | Show line weights | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.cursor-coordinates | Cursor coordinates | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.check-for-updates | Help ▸ Check for updates | partial / P2 | asks GitHub Releases on request and offers the release page. By the owner's decision there is no check at start and no in-place install (Acrobat has both) |
| view.single-key-accelerators | Single-key tool accelerators | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.customize-quick-tools | Customise the quick-action toolbar and pinned tools | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.pdfa-view-mode | Read-only view for PDF/A files with an Enable editing bar | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.show-page-boxes | Show art, trim and bleed boxes | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| view.web-app | Run in the browser (WebAssembly) | partial / P0 | web build works; a late startup ?file= download opens in a background tab after the user starts working, preserving the active document and Save target (#171). IndexedDB/OPFS recent list and download flow are M3.10 |

## edit (32)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| edit.text-edit | Edit existing text in place | partial / P0 | click a paragraph box and retype it in place (font reused or Helvetica substituted, colour and spacing kept, rewrapped); a page's /Contents pieces are parsed joined, so a line whose operands and operator sit in different pieces is listed and edited (lines and paragraphs still don't join across pieces; Edit image parses its pieces joined too (#3294)); drag a box to move it or its right-edge handle to rewrap it (edit.text-move-resize); per-character formatting changes are next |
| edit.text-super-subscript | Superscript and subscript | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.line-paragraph-spacing | Line and paragraph spacing | partial / P1 | line spacing (1, 1.15, 1.5, 2 × size or any multiple); space between paragraphs is not adjustable yet |
| edit.lists | Bulleted and numbered lists | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.find-replace | Find and replace in edit mode | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.rtl-cjk-editing | Edit right-to-left and CJK text | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.rotated-text-editing | Edit rotated text | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.keep-tags-on-edit | Keep structure tags when editing | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.image-crop | Crop an image | partial / P0 | for added images: trim each side, clipped |
| edit.image-external-editor | Edit an image in an external application | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.arrange-objects | Arrange objects (bring forward, send backward) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.align-distribute-objects | Align and distribute objects | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.vector-edit | Select, move, scale, recolour and delete vector paths | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.edit-object-tool | Edit object tool for any page object | partial / P1 | Same-page mixed selection (#844): Shift/Ctrl/Command-click toggles, empty-space marquee (additive with modifiers), drag previews and one atomic undo step for existing paragraphs, Image/Form XObjects and added text/images. Source glyph bytes, fonts and kerning retained; unselected text and shared Form resources stay put. Generation-scoped references with optional automation guard; count and geometry bounded; unsupported vertical/clipping text refused without partial changes. Still missing: arbitrary paths, inline images, cross-page movement, group resizing, arrange/align and a general object editor. See docs/editing-multiple-objects.md. |
| edit.object-properties | Object properties (colour space, font, convert colour) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.remove-all-links | Remove all links | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.action-goto | Go-to-page-view action (with named destinations) | partial / P1 | go to a page (Fit view); named destinations and zoom pending |
| edit.action-launch | Open-a-file action | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.action-named | Execute-a-menu-item action (safe list) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.action-set-ocg-state | Set-layer-visibility action | partial / P2 | Links and push buttons run it: ON, OFF and Toggle in order, by optional content group, and with /PreserveRB (the default) a layer turned on turns off the rest of its /RBGroups group. Like the Layers panel it changes the view, not the document (no undo step, not saved). Not yet: creating the action (Link Properties, a field's Actions), bookmarks and other triggers, /Next chains, JavaScript ocg.state, and the Layers panel keeping radio-button groups itself. |
| edit.action-javascript | Run-a-JavaScript action | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.action-thread | Read-an-article action | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| edit.action-goto-embedded | Go to a target in an embedded file (GoToE) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| edit.action-media | Play sound / media / rich-media actions | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| edit.header-footer-shrink | Shrink document to avoid overwriting content | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.watermark-appearance | Watermark rotation, opacity, scale, position, print/screen visibility | partial / P1 | rotation, opacity, behind/on top; no position offsets, scale or print/screen-only variants in the UI yet |
| edit.decoration-profiles | Saved header/footer and watermark settings | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.bates-numbering | Bates numbering | partial / P1 | Bates ▸ Add opens Header & Footer with a Bates number; no multi-file batch yet |
| edit.bates-multi-file | Bates numbering across multiple files | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.bates-remove | Remove Bates numbering | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| edit.spell-check | Spell check in comments, form fields and edited text | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| edit.spell-dictionaries | Spelling dictionaries per language and a user dictionary | planned / P2 | No additional limitation recorded; requires code/UI verification. |

## organize (30)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| organize.insert-from-clipboard | Insert pages from the clipboard | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| organize.insert-from-scanner | Insert pages from a scanner | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.insert-from-web | Insert pages from a web page | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.insert-by-drop | Insert by dropping files onto the page grid | partial / P1 | Desktop: PDFs, images and .txt files dropped on the Organize page grid are inserted at the gap under the pointer, in order, and selected (elsewhere a dropped file still opens as a document). While the files hover, a bar at the gap and a + badge follow the pointer; on macOS the pointer is read from AppKit, because the windowing layer sends no moves during a file drag. Missing: on Windows and Linux there is no live position, so the grid is only outlined and the files go to the gap the pointer is at right after the drop (or the end after a second); the web build (dropped files open as documents). Covered by simulated drop events; a real drag from Finder is not in the automated tests. |
| organize.drag-pages-between-documents | Drag pages between documents | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.page-range-selection | Select by range, odd/even, landscape/portrait | partial / P1 | Organize ▸ Select: all, odd, even, landscape, portrait; ranges via the page tools |
| organize.split-naming | Name split parts after bookmarks; never overwrite | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.split-multiple-files | Split multiple files in one run | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.combine-options | Combine options (file size, convert to PDF/A, include open files) | partial / P2 | Include open files: Add open documents (with their unsaved changes). Missing: file-size options and converting the result to PDF/A. |
| organize.field-name-conflicts | Rename or merge colliding field names when combining | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| organize.struct-tree-merge | Merge structure trees when combining | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| organize.remove-white-margins | Remove white margins | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.change-page-size | Change page size | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.page-transitions | Page transitions | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.page-tab-order | Page tab order property | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| organize.page-actions | Page open/close actions | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.page-templates | Page templates | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| organize.bookmark-appearance | Bookmark style and colour | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.bookmark-actions | Bookmark actions | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| organize.attachments-add | Add file attachments | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.attachments-delete | Delete attachments | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.attachments-description | Edit attachment descriptions | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.associated-files | Associated files with relationships (PDF/A-3, PDF 2.0) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.portfolio-view | View PDF Portfolios as a file list | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.portfolio-extract | Extract files from a portfolio | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.portfolio-create | Create PDF Portfolios | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| organize.custom-properties | Custom document properties | partial / P2 | Through the tools: doc_set_info sets or clears an /Info entry under any name the spec does not use (names checked as Acrobat does: no standard name, no empty name, no spaces or delimiters, at most 127 bytes), and doc_info lists them under custom, apart from the standard entries. Not yet: the Custom tab in Document Properties, and mirroring them into XMP (pdfx:) as Acrobat does |
| organize.xmp-editor | Additional metadata (XMP) editor | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.print-presets | Print dialog presets in document properties | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| organize.base-url | Base URL for relative links | planned / P3 | No additional limitation recorded; requires code/UI verification. |

## comment (19)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| comment.appearance-generation | Generate correct appearances for every markup type | partial / P0 | appearances for every type PdfCraft creates (highlight multiplies; clouds, callouts, stamps, polygons, carets); rich text not yet |
| comment.insert-text | Insert-text proposal (caret) | partial / P1 | caret at a click; not yet snapped to the text insertion point |
| comment.text-box | Text box comment with rich text | partial / P0 | Helvetica text with colour, size and alignment; /RC (XHTML) and /DS are written and kept in step with edits; per-run formatting (bold/italic spans) is not yet |
| comment.record-audio | Sound comments | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| comment.image-comment | Add an image as a comment | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.line-endings | All ten line-ending styles and dashed/cloudy borders | partial / P1 | All ten /LE styles (None, Square, Circle, Diamond, OpenArrow, ClosedArrow, Butt, ROpenArrow, RClosedArrow, Slash) are drawn on lines, polylines and callouts. comment_add endings and Comment properties set them. Dashed borders are drawn. Cloudy borders are still polygon-only. |
| comment.search-comments | Search comments | partial / P1 | search by author, text and type in the panel; not yet in the Find bar |
| comment.show-hide-comments | Show or hide all comments and pop-ups | partial / P1 | hide/show all comments (fields and links still draw); per-type hiding not yet |
| comment.popups | Open and reposition comment pop-ups | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| comment.connector-lines | Connector lines between markups and pop-ups | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.stamps-manage | Manage stamp categories and imports | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.stamps-rotate | Rotate stamps with a handle | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.export-selected | Export selected comments | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.import-from-pdf | Import comments from another PDF | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| comment.summarize | Summarize comments into a new PDF | partial / P1 | Comments only layout, sorted by page, author, date or type; the layouts with the document and connector lines not yet |
| comment.migrate | Migrate comments to a revised document | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| comment.measure-distance | Distance measuring tool | partial / P2 | Two-point distance annotations with saved numeric captions and rectilinear scales. Partial: only single-unit decimal formats; compound (ft-in) and fractional formats are not supported (listed as unsupported, data kept), no leader lines or caption placement options, geospatial scales unsupported, and interop with Acrobat-authored measurements (reading theirs, Acrobat reading ours) is untested. |
| comment.measure-scale | Scale ratio and calibration per viewport | partial / P2 | Persistent rectangular viewports with last-containing-viewport precedence, numeric scale and two-point calibration. First vertex selects the scale; prior annotations retain theirs. Partial: single-unit decimal rectilinear scales only; compound (ft-in), fractional and differing distance units can't be authored or imported, no viewport editing/removal UI, and interop with Acrobat's scale ratio dialog and its saved viewports is untested. |
| comment.geospatial | Geospatial location and measuring | planned / P2 | No additional limitation recorded; requires code/UI verification. |

## form (21)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| form.push-buttons | Push buttons run their actions | partial / P0 | Reset form, Named (print, page navigation), URI, Go to, and any JavaScript (run in the sandboxed engine); submit explained, not sent |
| form.appearance-regeneration | Regenerate field appearances (auto-size, comb, multiline, password) | partial / P0 | auto-size, comb, multiline, password, quadding, /MK colours and /MK /R rotation (0, 90, 180, 270); approximate Helvetica metrics |
| form.rich-text-fields | Rich-text field values | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.auto-complete | Auto-complete from previous entries | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.submit-form | Submit form to a URL or email (with a prompt) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| form.add-barcode-field | Barcode fields (PDF417, QR, Data Matrix) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.props-selection-change | Field properties: selection change | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.props-signed | Field properties: signed (lock fields when signed) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| form.multiple-copies | Create multiple copies of a field in a grid | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.show-tab-numbers | Show tab numbers | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.auto-detect-fields | Automatic form field detection | partial / P1 | Underscore runs, rules, empty boxes, small squares (check boxes) and rows of two or more touching cells of one size (one comb text field per row, the comb flag set and the character limit the number of cells; drawn forms only); runs when Prepare a form starts on a document without fields, and from Prepare a form ▸ Detect form fields at any time (existing fields are skipped). Not yet: comb rows read from a scan, tables of cells, scanned forms (needs OCR first), radio groups. |
| form.document-actions | Document actions (will close, will save, did save, will print, did print) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| form.js-host | JavaScript engine with the Acrobat object model subset | partial / P0 | boa (pure Rust) with the forms object model: event, getField/Field (value, readonly, display, colours, items, check boxes), app, util.printf/printd/printx, console, display, color; no annotations, layers or dialogs (app.response returns null) yet. |
| form.js-debugger | JavaScript debugger | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| form.edit-all-javascripts | Edit all JavaScripts in one editor | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| form.folder-level-js | Folder-level JavaScript | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| form.xfa-static | View and fill static XFA forms | partial / P1 | the AcroForm fields are shown and filled as in any form; on open their values come from the XFA datasets (what another viewer saved) by the fields' Designer names (recorded as a document warning), and every edit merges the values back into the datasets, keeping data no field binds to, so Acrobat shows the same values; values that can't be written are warnings. Missing: no template semantics (no scripts, calculations or hidden subforms), explicit bind refs are not followed (#60) |
| form.xfa-dynamic | View and fill dynamic XFA forms | partial / P1 | laid out from the template on open by the xfa crate (positioned, tb, lr-tb, table/row, page masters, breaks, repeated header rows, captions, rich text, JPEG/PNG/GIF images) into real pages and AcroForm fields (text, password, date with AFDate formats, check boxes, radio groups, drop-downs and list boxes with shown/saved item pairs and multi-select, signature fields, image fields showing their picture (pictures decoded once per document, at most 20 million pixels, and reused across re-layouts), buttons with reset/print/save/URL), filled from the datasets packet (including rows another viewer added), saved as an incremental revision with the values merged into the datasets so Adobe's viewers show them; doc_info reports xfa_layout. XFA JavaScript and FormCalc (form.xfa-formcalc) run in the js crate's XFA object model (xfa.form, SOM resolution, rawValue, presence, access, instance managers, xfa.host, xfa.layout, xfa.event): initialize and calculate on open, exit, validate and calculate after a field changes, click for buttons (through js_run and the app), with rows added and removed, subforms shown and hidden, and the form laid out again, all as one undo step and kept in the file. Hostile scripts are bounded per event (merged and capped effects, messages and console lines, lower loop limits and a time budget on open and calculate, one datasets rewrite per event, print/save/links/focus from clicks only, a runaway script abandoned and the form's scripts turned off) and what open-time scripts changed is noted. Missing: loops inside nested calls are only stopped by abandoning their thread, and script memory is not capped; the change event per keystroke; xfa.template, connections and data descriptions; a hidden instance of a repeating subform hides them all; choosing a new picture for an image field; barcodes are drawn as their text, not bars; keep rules; metrics are approximate and Acrobat fidelity is eyeballed on real forms, not measured (#60) |
| form.xfa-formcalc | FormCalc scripting | partial / P1 | a native FormCalc interpreter (crates/js/src/formcalc.rs) over the same form table, effects and budgets as the XFA JavaScript model: expressions and FormCalc coercions, var, if/while/for/foreach/func, accessors ($, $host, $form, $record, $layout, $event, xfa., .., [n], [*], _name instance managers) and the arithmetic, logical, string, date/time, financial and unit built-ins. Bounded: steps and loop iterations, statement and expression nesting, operator/accessor chain length, strings (1 MiB; messages and thrown errors clipped to 4 KiB, field values and calculate results to 64 KiB, noted; copying and comparing long strings is charged to the step budget, so it ends on wasm, which has no clock), lists (each object once), clamped dates, the instance cap, and a deadline the script itself honours after the engine stops waiting. Missing: locale-aware pictures beyond simple number and date ones, Format/Parse time and text pictures, Get/Post/Put (refused), $template/$connectionSet, user-defined functions across scripts, a cap on a script's total memory (each string and list is capped, but many variables can each hold one), real Uuid() values (deterministic, from the step count) and the wall clock on wasm (Date() reads 1970-01-01 and Time() midnight there), and checking against Acrobat on real Designer forms beyond the fixtures |
| form.xfa-data | XFA XML data import and export | partial / P2 | the datasets packet inside the PDF: read on open (values, dates in ISO form, check and radio states, extra rows for repeating subforms) and rewritten after every form edit from the fields' SOM paths (/PCSom), with areas left out of data paths as Adobe binds them. Missing: importing or exporting a standalone XML/XDP data file, explicit bind refs, data descriptions |
| form.xfa-flatten | Flatten XFA forms | planned / P2 | No additional limitation recorded; requires code/UI verification. |

## protect (18)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| protect.encrypt-attachments-only | Encrypt only file attachments | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| protect.certificate-encryption | Encrypt with certificates for recipients | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| protect.recipient-permissions | Per-recipient permissions for certificate security | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| protect.recipient-ldap | Find recipients in a directory (LDAP) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| protect.security-policies | Local security policies (saved presets) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| protect.mip-labels | Open Microsoft Purview-protected PDFs and apply labels | planned / P3 | optional Windows plug-in |
| protect.redact-pattern-locales | Pattern sets per locale | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| protect.redact-partial-words | Redact part of a match (e.g. all but the last four digits) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| protect.redact-folder | Search and redact across a folder | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| protect.redaction-properties | Redaction fill colour and overlay text | partial / P0 | fill colour or none; overlay text with font, size or auto-size, colour, alignment and repeat, or redaction codes (protect.redaction-codes) |
| protect.redaction-codes | Redaction code sets (e.g. FOIA exemptions) | partial / P1 | built-in U.S. FOIA (5 U.S.C. 552(b)) and U.S. Privacy Act (552a) sets; picked codes become the overlay text, in the set's order. Missing: Edit Codes, custom code sets, import/export of code sets |
| protect.redact-filename-suffix | Add a suffix to redacted file names | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| protect.redact-ai-suggestions | Suggest sensitive content to redact via an AI provider | planned / P2 | local or user-provided model; off by default |
| protect.remove-hidden-on-close | Remove hidden information when closing or sending | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| protect.attachment-trust | Block opening risky attachment types | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| protect.url-access-control | Control internet access from PDFs | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| protect.protected-view | Open untrusted files in a restricted view | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| protect.security-envelope | Security envelopes | planned / P3 | legacy |

## sign (22)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| sign.pkcs11 | Sign with PKCS#11 tokens and smart cards | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.signature-appearance | Configure signature appearance (text, graphic, name) | partial / P1 | text items (name, date, reason, location, DN, labels); no graphics or saved appearances yet |
| sign.lock-after-signing | Lock document or fields after signing | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.field-mdp | Field locks (FieldMDP) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.document-timestamp | Add a document timestamp (RFC 3161) | partial / P1 | timestamp_document appends a standalone /ETSI.RFC3161 signature dictionary over the whole file; validation discovers and verifies these (imprint, token signature, TSA cert validity) and reports the trusted time. Missing: automation tool and timestamp-server configuration (transport is caller-injected). |
| sign.timestamp-servers | Configure timestamp servers | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.pades-bt | PAdES B-T (signature timestamp) | partial / P1 | Sign with an embedded RFC 3161 token (sign_with_timestamp): imprint and TSA signature verified at signing and validation time, trusted time reported. Missing: timestamp-server configuration and a fetcher (transport is caller-injected), the standalone document timestamp, automation tool. |
| sign.ltv | Long-term validation (DSS/VRI) | partial / P1 | dss::embed merges caller-supplied evidence into the catalog /DSS with /VRI entries per signature, deduplicating blobs, as a permitted incremental change; revocation.rs verifies the embedded CRLs and OCSP responses (signature, responder identity, CertID, window) and a verified revocation invalidates the signature. Missing: fetching evidence, automation tool. |
| sign.pades-blta | PAdES B-LT and B-LTA | partial / P1 | The B-LT/B-LTA chain works with caller-supplied evidence: sign, embed DSS (B-LT), then append a document timestamp over the result (B-LTA); earlier signatures stay valid with the store classified as a permitted change. Missing: OCSP/CRL verification of the evidence, re-timestamping automation. |
| sign.ocsp-crl | Revocation checking (OCSP and CRL) | partial / P1 | RFC 5280 CRLs and RFC 6960 OCSP responses are parsed and verified (issuer/responder signature, delegated responder's OCSP-signing EKU, CertID hash match, validity window) and checked against the DSS during validation. Missing: network fetching of fresh evidence and the OS trust store. |
| sign.signatures-panel | Signatures panel with per-revision tree | partial / P0 | per-signature tree with details and actions; no per-revision page outline yet |
| sign.signature-properties | Signature properties (validity summary, signer, time) | partial / P0 | validity summary, signer, issuer, time, algorithm in the panel; no separate properties dialog |
| sign.trust-import | Import trusted certificates and set trust | partial / P1 | import .cer/.pem/.der/.p12 through the tool; per-certificate trust settings not yet |
| sign.eutl | EU Trusted Lists | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| sign.os-trust | Trust the operating system's root certificates | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.verification-time | Verification time (signing time, timestamp, now) | partial / P1 | Validation prefers the verified RFC 3161 timestamp time over /M when checking certificate validity and revocation, but only when the TSA chains to the trust store; an untrusted token is reported as an unverified timestamp and its time is not used. Missing: an explicit verification-time option (verify at a chosen date / now) in the validation API. |
| sign.compare-signed-version | Compare signed version with the current one | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| sign.clear-signature | Clear an unsaved signature; clear all signature fields | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| sign.seed-values | Signature seed values | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| sign.preview-mode | Preview document mode before signing | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| sign.qualified-labels | Show qualified signature/seal labels | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| sign.web-signing | Sign in the browser with a PKCS#12 file | planned / P1 | No additional limitation recorded; requires code/UI verification. |

## ocr (21)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| ocr.ocr-languages | OCR document language selection | partial / P0 | Language choice exists, but the models read only the Latin alphabet without accents (English). |
| ocr.searchable-image | Searchable image output (invisible text layer) | partial / P0 | Invisible text (Tr 3) fitted to each word box; the image is not deskewed or downsampled (exact mode only). |
| ocr.editable-text-output | Editable text and images output | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.ocr-downsample | Downsample images during OCR | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.rotation-detection | Detect and fix page rotation | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.deskew | Deskew | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.despeckle | Despeckle | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.background-removal | Background removal | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.descreen | Descreen | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.remove-halo | Remove halos | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| ocr.edge-shadow-removal | Edge shadow removal | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.camera-perspective | Enhance camera images (edges, perspective, contrast) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.camera-document-type | Camera image document types (whiteboard, card, form) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.correct-recognized-text | Correct recognized text (low-confidence suspects) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.adaptive-compression | Adaptive (MRC) compression of scans | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.scan-pdfa | Make scans PDF/A compliant | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.ocr-engines | Pluggable OCR engines (ocrs default; tesseract optional) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.ocr-web | OCR in the browser | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| ocr.scanner-acquire | Scan from TWAIN/WIA/ICA/SANE scanners | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.scan-presets | Scan presets | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| ocr.scan-append | Append scans to an existing PDF or split into files | planned / P2 | No additional limitation recorded; requires code/UI verification. |

## create (22)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| create.from-markdown | Create PDF from Markdown | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.from-screenshot | Create PDF from a screen capture | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.from-html | Create PDF from an HTML file | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.from-web-page | Create PDF from a web page URL | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.from-office | Create PDF from Office documents (LibreOffice sidecar) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.image-conversion-options | Image compression options when converting | partial / P2 | Page sizing: embedded resolution (72 dpi fallback), fixed 72 dpi, or custom 1–1200 dpi; pixels are not resampled. Compression options are still absent. Windows image context menu opens the same chooser; other OS context-menu integration not yet. |
| create.pdf-printer | Print to PDF from other applications | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| create.distiller | PostScript/EPS to PDF (Distiller) | planned / P3 | deferred: a PostScript interpreter is a large project |
| create.job-options | PDF settings presets (job options) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| create.export-docx | Export to Word (.docx) | partial / P1 | Flowing Word document: paragraphs, Heading 1/2 from larger text, bold/italic, text colour (fill as sRGB; gray/CMYK converted naively, colour spaces via cs/sc read as black), images inline, page breaks, and detected tables with spans/empty cells (a row whose blocks run on into each other, such as a link inside a sentence or a superscript, or overlap as a paragraph's lines do, is text and not a table row, #2806); checked with macOS textutil and opened in Microsoft Word (Windows). Characters XML forbids are dropped; the page size stays within Word's 22 in. Not yet: columns, fonts, colour per run within a paragraph, exact layout mode. |
| create.export-xlsx | Export to Excel (.xlsx) with table detection | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.export-pptx | Export to PowerPoint (.pptx) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.export-jpeg2000 | Export pages as JPEG 2000 | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.export-html | Export to HTML | partial / P1 | Single HTML file with headings, paragraphs (with text colour), inline images, and detected tables with spans/empty cells (a row whose blocks run on into each other, such as a link inside a sentence or a superscript, or overlap as a paragraph's lines do, is text and not a table row, #2806). Not yet: multi-page folder output, CSS from fonts. |
| create.export-accessible-text | Export accessible text (from tags) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.export-xml | Export to XML | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.export-svg | Export to SVG | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.export-ps | Export to PostScript | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.export-eps | Export to EPS | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.export-ocr | Run OCR automatically when exporting scans | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| create.export-settings | Per-format export settings | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| create.translate | Translate documents | planned / P3 | through an optional AiProvider; never Adobe's cloud |

## optimize (43)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| optimize.compatibility-level | Make compatible with a chosen PDF version | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.optimizer | Advanced optimization (PDF Optimizer) | partial / P1 | Images, Discard Objects, Discard User Data, Clean Up (also drops images and forms the pages list but never draw, such as deleted images); runs in the background with a progress bar (per image) and Cancel; no Fonts or Transparency panels, no space audit |
| optimize.downsample-images | Downsample colour, grey and mono images | partial / P1 | colour and grayscale, bicubic, effective resolution measured where drawn; monochrome not yet |
| optimize.recompress-images | Recompress images (JPEG, JPEG 2000, Flate) | partial / P1 | JPEG (quality steps) and ZIP; no JPEG 2000 |
| optimize.mono-compression | Monochrome compression (JBIG2, CCITT G4) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.unembed-fonts | Unembed fonts | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.flatten-transparency | Flatten transparency | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.discard-search-index | Discard embedded search index | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.discard-form-actions | Discard form submit, import and reset actions | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.discard-external-refs | Discard external cross-references | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.discard-hidden-layers | Discard hidden layers and flatten visible ones | partial / P2 | hidden layer content removed; visible layers not flattened |
| optimize.merge-image-fragments | Detect and merge image fragments | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| optimize.lines-to-curves | Convert smooth lines to curves | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| optimize.compress-structure | Compress document structure (object streams) | partial / P1 | on by default for full saves; no Optimizer option yet |
| optimize.flate-unencoded | Flate-encode unencoded streams; replace LZW with Flate | partial / P1 | unencoded streams; LZW not yet re-encoded |
| optimize.optimize-content | Optimize page content streams | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.optimizer-presets | Optimizer presets | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.batch-optimize | Optimize multiple files | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight | Preflight analysis | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-fix | Preflight analyze and fix | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-profiles | Built-in preflight profiles | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-single-checks | Single checks | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-single-fixups | Single fixups | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-custom-profiles | Create and edit custom profiles | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-profile-exchange | Import, export and lock profiles | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-results | Preflight results tree with object details | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-snap-view | Snap view of a flagged object | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-reports | Preflight reports (PDF, XML, text) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-droplets | Preflight droplets and batch preflight | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.preflight-audit-trail | Embed preflight audit trail | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfa-validate | Verify PDF/A compliance (1-4) | partial / P1 | PDF/A-2b/3b structural rules: identification, encryption, output intent (present, and RGB/CMYK to match DeviceRGB/DeviceCMYK use), fonts embedded, annotations, actions, images, embedded files. Not yet: content-stream operator checks, transparency details, PDF/A-1 and -4, a veraPDF comparison. |
| optimize.pdfa-convert | Save as PDF/A (1b, 2b, 3b, 4) | partial / P1 | Fixes XMP, output intent (generated sRGB ICC profile, only for documents without DeviceCMYK; CMYK documents need a user-chosen CMYK output intent, not yet offered), forbidden actions, annotation flags, interpolation, encryption, AFRelationship. Not yet: embedding fonts, converting device CMYK, flattening transparency for PDF/A-1. |
| optimize.pdfa-accessible-levels | PDF/A conformance levels a and u | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfx-validate | Verify PDF/X compliance | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfx-convert | Save as PDF/X (1a, 3, 4) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.output-intents | Choose output intents and conditions | partial / P1 | sRGB output intent added and listed; no choice of other conditions yet. |
| optimize.pdfe | PDF/E | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfvt | PDF/VT | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfua-validate | Verify PDF/UA-1 and PDF/UA-2 | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| optimize.standards-panel | Standards panel showing declared conformance | partial / P1 | Declared PDF/A, PDF/UA and output intents shown in the PDF/A dialog and returned by pdfa_verify. |
| optimize.remove-standards-id | Remove PDF/A or PDF/X identification | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| optimize.pdfx5-reference-xobjects | Show PDF/X-5 reference XObjects | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| optimize.einvoice-pdfa3 | Embed e-invoice XML in PDF/A-3 | planned / P3 | No additional limitation recorded; requires code/UI verification. |

## print (24)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| print.paper-source-by-size | Choose paper source by PDF page size | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.print-with-comment-summary | Print with a comments summary | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.grayscale | Print in grayscale | partial / P1 | asks the printer for monochrome (lp -o, and the Windows page settings as the job's); the PDF itself stays in colour |
| print.print-as-image | Print as image | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.page-setup | Page setup | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.postscript-output | PostScript Level 3 output | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.output-preview | Output preview | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.separations | Separations preview with area coverage | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.total-area-coverage | Total area coverage warning | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.color-warnings | Overprint and rich-black warnings | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.object-inspector | Object inspector | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.simulate-paper-ink | Simulate paper colour and black ink | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.overprint-preview | Overprint preview | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.convert-colors | Convert colours | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.ink-manager | Ink manager | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.ink-aliases | Ink aliases and spot-to-process | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.printer-marks | Add printer marks (crop, bleed, registration, colour bars) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.fix-hairlines | Fix hairlines | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.flattener-preview | Flattener preview | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.color-management | Colour management settings (working spaces, intents) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| print.advanced-print-separations | Print separations and in-RIP separations | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.print-marks-bleeds | Marks and bleeds when printing | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| print.trap-presets | Trap presets (stored as data) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| print.jdf | JDF job definitions (read-only) | planned / P3 | No additional limitation recorded; requires code/UI verification. |

## a11y (23)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| a11y.app-screen-reader | Screen-reader support for the app (AccessKit) | partial / P0 | AccessKit labels on all custom widgets; full audit in M14.3 |
| a11y.keyboard-only | Full keyboard operation of the app | planned / P0 | No additional limitation recorded; requires code/UI verification. |
| a11y.autotag | Automatically tag a document | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.autotag-fields | Autotag form fields | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.reading-order-tool | Reading order tool | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.tags-panel | Tags panel (new, delete, find, change to artifact) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.tag-properties | Tag properties (type, title, actual text, alternate text, language) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.role-map | Role map editor | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.class-map | Class map editor | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| a11y.order-panel | Order panel | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.content-panel | Content panel | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.create-tag-from-selection | Create a tag from a selection | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.artifact-marking | Mark content as artifact | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.find-untagged | Find unmarked content, comments and links | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.table-editor | Table editor (headers, scope, spans) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.alt-text-suggestions | Suggest alternate text via an AI provider | planned / P2 | optional provider; off by default |
| a11y.make-accessible-action | Make Accessible guided action | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| a11y.setup-assistant | Accessibility setup assistant | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.read-aloud | Read out loud (platform TTS) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.read-aloud-options | Read-aloud voice, pitch and speed | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.replace-page-colors | Replace page colours (high contrast) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.reflow-tagged | Reflow tagged content | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| a11y.keyboard-selection-cursor | Always show the keyboard selection cursor | planned / P2 | No additional limitation recorded; requires code/UI verification. |

## misc (55)

| Feature ID | Function | Repository status / tier | Exact limitation notes |
|---|---|---|---|
| misc.compare-side-by-side | Side-by-side compare view with synchronised scrolling | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.compare-filters | Filter changes by type (text, images, formatting) | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.compare-page-ranges | Compare page ranges | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.compare-scanned | Compare scanned documents via OCR | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.action-wizard-manage | Import, export, copy and delete actions | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.action-archive | Built-in action: archive documents | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.action-publish-web | Built-in action: prepare for web publishing | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.batch-processing | Batch processing over files and folders | partial / P1 | Actions and OCR run over chosen files; whole folders (with subfolders) not yet. |
| misc.action-prompt-steps | Prompt-user and instruction steps | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.action-javascript-step | Execute JavaScript step | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.custom-commands | Custom commands | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.custom-tools | Custom tool sets | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.cli-completeness | CLI covers every shipped command | partial / P0 | tools grow as features land |
| misc.ai-provider | AI provider interface (local model or own endpoint), off by default | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-summary | Summarize a document | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-ask | Ask questions with citations to pages | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-conversational-edit | Natural-language editing through registry commands | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-presentation | Generate a presentation from documents | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-podcast | Generate an audio overview | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.ai-charts | Charts from document data | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.contract-analysis | Contract key terms and multi-contract compare | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.rich-media-preserve | Preserve 3D, video and sound content | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.rich-media-poster | Show posters for rich media | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.rich-media-playback | Play video and sound via the platform | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.3d-view | View 3D (U3D/PRC) content | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.legacy-multimedia | Legacy multimedia (screen annotations, renditions) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.preferences-dialog | Preferences dialog generated from a schema | partial / P1 | Preferences dialog with Interface language, Documents and view (persistent default workspace with session override), Identity, Fill & Sign (flatten on save, off by default) and JavaScript; other categories still live in their tools, and it isn't generated from a schema yet. |
| misc.prefs-general | Preferences: general | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-page-display | Preferences: page display | partial / P1 | Preferences ▸ Documents and view: Default page display and Default zoom (Fit to width, Zoom to page level or a percentage) for every new tab; a PDF that asks for a layout or zoom gets it. Missing: Acrobat's resolution, rendering and page-content options |
| misc.prefs-documents | Preferences: documents (autosave interval, recent count) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-commenting | Preferences: commenting | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-forms | Preferences: forms | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-identity | Preferences: identity | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-full-screen | Preferences: full screen | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-search | Preferences: search | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-signatures | Preferences: signatures | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-security | Preferences: security and trust | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-spelling | Preferences: spelling | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-language | Preferences: language | partial / P1 | Shortcut modifiers and key names use a dedicated catalog context across toolbar, menus, palette, Help and tooltips, with English fallback and macOS symbols preserved. Preferences switches Auto (system language)/English/Japanese/Simplified Chinese/Traditional Chinese/Czech/Brazilian Portuguese/German/Spanish/French/Telugu/Hungarian/Ukrainian/Italian/Arabic and persists; ui.set language auto\|en\|ja\|zh-hans\|zh-hant\|cs\|pt-br\|de\|es\|fr\|te\|hu\|uk\|it\|ar and ui.state expose it. TSV catalogs per language (same system as PhotoCraft); every dialog, panel and notice goes through tl!. Japanese, Traditional Chinese, Simplified Chinese, German, Spanish, French and Hungarian cover commands, dialogs, panels, shortcuts and history labels; Czech and Brazilian Portuguese the menus. French locales (fr, fr_FR, fr_CA, fr_BE) select the French catalog, German locales (de, de_DE, de_AT, de_CH) the German one, Hungarian locales (hu, hu_HU) the Hungarian one. German search hints reserve shortcut space, Home cards bound translated text, certificate Subject has its own context, and inserted tool names keep their capitalization. Engine/OS error details stay untranslated. Auto detects the Windows display language after explicitly named supported locale overrides; neutral C/POSIX launcher locales are skipped on Windows, with Chinese script/region mapping and English fallback. Arabic (ar and every ar-* locale) covers commands, dialogs and panels with the six CLDR plural forms; its labels are put into display order when the catalog loads, because egui joins Arabic letters but lays words out left to right. Missing: a right-to-left layout (Arabic labels sit in the left-to-right layout, a wrapped Arabic paragraph breaks at the wrong end, and text typed into a field is not reordered), Czech and Brazilian Portuguese dialogs, Chinese and Telugu font delivery in releases (the pinned craft-fonts revision has an Arabic face; its Noto Sans CJK SC is never embedded, AGENTS.md §1.1), a full localization audit. |
| misc.prefs-reading | Preferences: reading | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-accessibility | Preferences: accessibility | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-color-management | Preferences: colour management | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-measuring | Preferences: measuring | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-units-guides | Preferences: units and guides | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-content-editing | Preferences: content editing | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-convert | Preferences: convert to and from PDF | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.prefs-catalog | Preferences: catalog / index | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.admin-preferences | Admin-deployable preference defaults | planned / P2 | No additional limitation recorded; requires code/UI verification. |
| misc.localization | Localization (Fluent catalogs) | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.rtl-ui | Right-to-left user interface | planned / P2 | Only a first step exists: Arabic and Hebrew file names and paths in the tab strip, the recent-files list and Properties have glyphs (a craft-fonts Arab face, or an installed font on desktop) and are shown in display order (crates/ui-egui/src/bidi.rs). Missing: multi-line text (bookmarks, comments, search results), text input, a mirrored layout and a right-to-left interface language. |
| misc.installers | Signed installers for macOS, Windows and Linux | planned / P1 | Windows MSI creates a Start menu shortcut and an optional (default on) desktop shortcut and confirms full-UI completion with Finish (#143); packaging/windows/test-msi.ps1 checks the compiled MSI and windows-arm64.yml checks installation/removal. PDF files use a dedicated embedded document icon when PdfCraft is chosen as the default reader (confirmed in Windows Explorer); first-page thumbnails remain unsupported. Signed installers across platforms remain planned. |
| misc.web-deploy | Hosted web app deployment | planned / P1 | No additional limitation recorded; requires code/UI verification. |
| misc.performance-budgets | Meet performance budgets on Tier-1 platforms | planned / P0 | No additional limitation recorded; requires code/UI verification. |
| misc.send-email | Send the file by email (system mail client) | planned / P3 | No additional limitation recorded; requires code/UI verification. |
| misc.user-docs | User documentation and keyboard reference | planned / P1 | No additional limitation recorded; requires code/UI verification. |
