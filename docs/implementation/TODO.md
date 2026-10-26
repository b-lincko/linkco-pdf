# Live implementation TODO

> Generated from the full research inventory and `progress.toml`; do not edit statuses here.

[Workflow and completion rules](README.md) · [Filterable CSV](TODO.csv) · [Original research](../research/pdf-product-strategy-2026-10-25/README.md)

Order: **basic → advanced → future**. Implement → audit → fix → re-audit → test → push → verify CI → ask the user to continue.

Scope: **222 recommendations + 384 existing gaps + 23 scope reviews**, plus bounded components. These overlap; they are not 629 distinct new functions. Existing shipped features are not marked missing. Complete a gap only when its own criteria are demonstrated, not just because a related component landed.

P0 is a safety priority, not a universal scheduling stage. Gates for a new collaboration service stay attached to that advanced service; native mobile work is future. Never launch such a surface before its safety gates pass.

**Active component:** B006 · **Active stage:** basic

## Basic (337 records)

ci_pending: 1, complete: 5, in_progress: 1, pending: 330

| ID | TODO | Priority | Status | Related tasks |
|---|---|---|---|---|
| B001 | [x] Reject paths as atomic-save staging filenames | P0 | complete | REC-R01 |
| B002 | [x] Audit and harden flush-before-replace in automation saves | P0 | complete | REC-R01 |
| B003 | [x] Create Unix atomic-save staging files with private permissions | P0 | complete | REC-R01 |
| B004 | [x] Deterministic atomic-save write and rename fault coverage | P0 | complete | REC-R01 |
| B005 | [x] Process-termination boundaries and safe retry for atomic saves | P0 | complete | REC-R01 |
| B006 | [ ] Retry recovery autosaves until complete persistence succeeds | P0 | ci_pending | REC-R01 |
| GAP-a11y.app-screen-reader | [ ] Screen-reader support for the app (AccessKit) | P0 | pending | REC-A02 |
| GAP-a11y.keyboard-only | [ ] Full keyboard operation of the app | P0 | pending | REC-A01 |
| GAP-comment.appearance-generation | [ ] Generate correct appearances for every markup type | P0 | pending | REC-C01 |
| GAP-comment.text-box | [ ] Text box comment with rich text | P0 | pending | REC-C01 |
| GAP-core.image-filters | [ ] Image codecs in the filters crate (DCT, JPX, JBIG2, CCITT) | P0 | pending |  |
| GAP-core.lazy-loading | [ ] Lazy loading of very large (GB) files within a memory budget | P0 | pending | REC-R03; REC-W05 |
| GAP-edit.image-crop | [ ] Crop an image | P0 | pending | REC-E07 |
| GAP-edit.text-edit | [ ] Edit existing text in place | P0 | pending | REC-E01 |
| GAP-form.appearance-regeneration | [ ] Regenerate field appearances (auto-size, comb, multiline, password) | P0 | pending | REC-F01 |
| GAP-form.push-buttons | [ ] Push buttons run their actions | P0 | pending | REC-F01 |
| GAP-misc.cli-completeness | [ ] CLI covers every shipped command | P0 | pending | REC-B01 |
| GAP-misc.performance-budgets | [ ] Meet performance budgets on Tier-1 platforms | P0 | pending | REC-R03; REC-R08 |
| GAP-ocr.ocr-languages | [ ] OCR document language selection | P0 | pending | REC-O01; REC-O02 |
| GAP-ocr.searchable-image | [ ] Searchable image output (invisible text layer) | P0 | pending | REC-O12 |
| GAP-protect.redaction-properties | [ ] Redaction fill colour and overlay text | P0 | pending |  |
| GAP-sign.signature-properties | [ ] Signature properties (validity summary, signer, time) | P0 | pending | REC-D01 |
| GAP-sign.signatures-panel | [ ] Signatures panel with per-revision tree | P0 | pending | REC-D01 |
| GAP-view.deep-zoom-tiles | [ ] Tiled rendering keeps deep zoom sharp | P0 | pending |  |
| GAP-view.render-cid-fonts | [ ] CID fonts and CMaps incl. CJK without ToUnicode | P0 | pending | REC-R02 |
| GAP-view.render-image-codecs | [ ] JBIG2, JPEG 2000, CCITT and JPEG images | P0 | pending |  |
| GAP-view.render-pages | [ ] Render pages (text, vectors, images) | P0 | pending | REC-R02 |
| GAP-view.render-shadings-patterns | [ ] Shadings, gradients and tiling patterns | P0 | pending |  |
| GAP-view.render-transparency | [ ] Blend modes, soft masks and transparency groups | P0 | pending | REC-R02 |
| GAP-view.render-type3 | [ ] Type 3 fonts | P0 | pending |  |
| GAP-view.web-app | [ ] Run in the browser (WebAssembly) | P0 | pending | REC-W01 |
| REC-A01 | [ ] Keyboard-only complete everyday workflows | P0 | pending | a11y.keyboard-only |
| REC-A02 | [ ] Screen-reader labels focus and navigation audit | P0 | pending | a11y.app-screen-reader |
| REC-A08 | [ ] Large targets zoom and non-drag alternatives | P0 | pending | '- |
| REC-B02 | [ ] Versioned schemas stable exit codes and structured errors | P0 | pending | '- |
| REC-B03 | [ ] Dry-run and destructive-operation approval plan | P0 | pending | '- |
| REC-B07 | [ ] Bounded worker pools and resource controls | P0 | pending | '- |
| REC-B09 | [ ] MCP permission profiles and per-action approvals | P0 | pending | '- |
| REC-C01 | [ ] Interoperable annotation appearance regression suite | P0 | pending | comment.appearance-generation comment.text-box |
| REC-D01 | [ ] Plain-language signature status and signed-revision viewer | P0 | pending | sign.signatures-panel sign.signature-properties sign.compare-signed-version |
| REC-E01 | [ ] Reliable existing-text editing with font diagnostics | P0 | pending | edit.text-edit |
| REC-F01 | [ ] Predictable required-field validation and completion guide | P0 | pending | form.appearance-regeneration form.push-buttons |
| REC-H04 | [ ] Share preflight for secrets metadata signatures and forms | P0 | pending | '- |
| REC-P04 | [ ] Merge form-name conflict resolution | P0 | pending | organize.field-name-conflicts |
| REC-Q05 | [ ] Reliable mixed-size duplex tray and label printing | P0 | pending | print.paper-source-by-size print.page-setup |
| REC-R01 | [ ] Save-integrity and crash-recovery release gate | P0 | in_progress | '- |
| REC-R02 | [ ] Independent rendering-fidelity benchmark | P0 | pending | view.render-pages view.render-transparency view.render-cid-fonts |
| REC-R03 | [ ] Global memory and decoded-data budgets | P0 | pending | core.lazy-loading misc.performance-budgets |
| REC-R04 | [ ] Cancellable background-job centre | P0 | pending | '- |
| REC-R05 | [ ] External-file change and save-conflict detection | P0 | pending | '- |
| REC-R06 | [ ] Recoverable per-document operation failures | P0 | pending | '- |
| REC-R07 | [ ] Open-save-reopen interoperability matrix | P0 | pending | '- |
| REC-R08 | [ ] Performance budgets on ordinary office hardware | P0 | pending | misc.performance-budgets |
| REC-R09 | [ ] GPU fallback and mixed-DPI acceptance suite | P0 | pending | '- |
| REC-R10 | [ ] Signed verified updates with rollback | P0 | pending | view.check-for-updates |
| REC-R12 | [ ] Stable capability and limitation disclosure | P0 | pending | '- |
| REC-S01 | [ ] Verified redaction and sanitized share-copy gate | P0 | pending | '- |
| REC-S02 | [ ] Restricted view for untrusted PDFs | P0 | pending | protect.protected-view protect.attachment-trust |
| REC-S03 | [ ] Explicit document network access controls | P0 | pending | protect.url-access-control |
| REC-S08 | [ ] Secure temporary files thumbnails and local history | P0 | pending | '- |
| REC-S09 | [ ] Privacy dashboard with offline-only switch | P0 | pending | '- |
| REC-T01 | [ ] Signed installers and macOS notarization gate | P0 | pending | '- |
| REC-T06 | [ ] SBOM dependency policy and security response | P0 | pending | '- |
| REC-T09 | [ ] Published platform capability and release-support matrix | P0 | pending | '- |
| REC-W01 | [ ] Crash-safe web working copy and recovery | P0 | pending | core.web-recovery view.web-app |
| REC-W02 | [ ] Browser capability and support matrix | P0 | pending | '- |
| REC-W03 | [ ] Explicit download/save acknowledgement | P0 | pending | '- |
| REC-W07 | [ ] Harden existing same-origin host bridge | P0 | pending | '- |
| GAP-a11y.artifact-marking | [ ] Mark content as artifact | P1 | pending |  |
| GAP-a11y.make-accessible-action | [ ] Make Accessible guided action | P1 | pending | REC-A06 |
| GAP-a11y.order-panel | [ ] Order panel | P1 | pending | REC-A03 |
| GAP-a11y.reading-order-tool | [ ] Reading order tool | P1 | pending | REC-A03 |
| GAP-a11y.tag-properties | [ ] Tag properties (type, title, actual text, alternate text, language) | P1 | pending |  |
| GAP-a11y.tags-panel | [ ] Tags panel (new, delete, find, change to artifact) | P1 | pending | REC-A03 |
| GAP-comment.insert-text | [ ] Insert-text proposal (caret) | P1 | pending |  |
| GAP-comment.line-endings | [ ] All ten line-ending styles and dashed/cloudy borders | P1 | pending |  |
| GAP-comment.popups | [ ] Open and reposition comment pop-ups | P1 | pending |  |
| GAP-comment.search-comments | [ ] Search comments | P1 | pending | REC-V02; REC-C03 |
| GAP-comment.show-hide-comments | [ ] Show or hide all comments and pop-ups | P1 | pending |  |
| GAP-comment.summarize | [ ] Summarize comments into a new PDF | P1 | pending | REC-C05 |
| GAP-core.arlington-validation | [ ] Structural validation against the Arlington PDF model | P1 | pending |  |
| GAP-core.attachment-only-encryption | [ ] Attachment-only encryption (/EFF) opens without a password | P1 | pending |  |
| GAP-core.linearization | [ ] Linearized save (Fast Web View) | P1 | pending | REC-W05; REC-Q10 |
| GAP-core.pdf20-associated-files | [ ] PDF 2.0 associated files (/AF) | P1 | pending |  |
| GAP-core.pdf20-namespaces | [ ] PDF 2.0 structure namespaces | P1 | pending |  |
| GAP-core.pdf20-utf8-strings | [ ] PDF 2.0 UTF-8 text strings | P1 | pending |  |
| GAP-core.web-recovery | [ ] Crash recovery in the web app | P1 | pending | REC-W01 |
| GAP-create.export-docx | [ ] Export to Word (.docx) | P1 | pending | REC-X01 |
| GAP-create.export-html | [ ] Export to HTML | P1 | pending |  |
| GAP-create.export-ocr | [ ] Run OCR automatically when exporting scans | P1 | pending | REC-X09 |
| GAP-create.export-svg | [ ] Export to SVG | P1 | pending |  |
| GAP-create.export-xlsx | [ ] Export to Excel (.xlsx) with table detection | P1 | pending | REC-X02 |
| GAP-create.from-screenshot | [ ] Create PDF from a screen capture | P1 | pending |  |
| GAP-edit.action-goto | [ ] Go-to-page-view action (with named destinations) | P1 | pending |  |
| GAP-edit.action-javascript | [ ] Run-a-JavaScript action | P1 | pending |  |
| GAP-edit.align-distribute-objects | [ ] Align and distribute objects | P1 | pending | REC-E08 |
| GAP-edit.arrange-objects | [ ] Arrange objects (bring forward, send backward) | P1 | pending | REC-E08 |
| GAP-edit.bates-multi-file | [ ] Bates numbering across multiple files | P1 | pending |  |
| GAP-edit.bates-numbering | [ ] Bates numbering | P1 | pending | REC-E11 |
| GAP-edit.bates-remove | [ ] Remove Bates numbering | P1 | pending |  |
| GAP-edit.edit-object-tool | [ ] Edit object tool for any page object | P1 | pending | REC-E08 |
| GAP-edit.keep-tags-on-edit | [ ] Keep structure tags when editing | P1 | pending | REC-E10 |
| GAP-edit.line-paragraph-spacing | [ ] Line and paragraph spacing | P1 | pending | REC-E05 |
| GAP-edit.lists | [ ] Bulleted and numbered lists | P1 | pending | REC-E05 |
| GAP-edit.rtl-cjk-editing | [ ] Edit right-to-left and CJK text | P1 | pending | REC-V11; REC-E02; REC-E03 |
| GAP-edit.watermark-appearance | [ ] Watermark rotation, opacity, scale, position, print/screen visibility | P1 | pending |  |
| GAP-form.auto-detect-fields | [ ] Automatic form field detection | P1 | pending | REC-F02 |
| GAP-form.document-actions | [ ] Document actions (will close, will save, did save, will print, did print) | P1 | pending |  |
| GAP-form.props-signed | [ ] Field properties: signed (lock fields when signed) | P1 | pending | REC-F08 |
| GAP-form.submit-form | [ ] Submit form to a URL or email (with a prompt) | P1 | pending | REC-F06 |
| GAP-form.xfa-dynamic | [ ] View and fill dynamic XFA forms | P1 | pending | REC-F09 |
| GAP-form.xfa-formcalc | [ ] FormCalc scripting | P1 | pending |  |
| GAP-form.xfa-static | [ ] View and fill static XFA forms | P1 | pending | REC-F09 |
| GAP-misc.action-archive | [ ] Built-in action: archive documents | P1 | pending |  |
| GAP-misc.batch-processing | [ ] Batch processing over files and folders | P1 | pending | REC-O07; REC-B04 |
| GAP-misc.compare-side-by-side | [ ] Side-by-side compare view with synchronised scrolling | P1 | pending | REC-V05 |
| GAP-misc.custom-commands | [ ] Custom commands | P1 | pending | REC-B05 |
| GAP-misc.installers | [ ] Signed installers for macOS, Windows and Linux | P1 | pending |  |
| GAP-misc.localization | [ ] Localization (Fluent catalogs) | P1 | pending |  |
| GAP-misc.preferences-dialog | [ ] Preferences dialog generated from a schema | P1 | pending |  |
| GAP-misc.prefs-accessibility | [ ] Preferences: accessibility | P1 | pending |  |
| GAP-misc.prefs-color-management | [ ] Preferences: colour management | P1 | pending |  |
| GAP-misc.prefs-commenting | [ ] Preferences: commenting | P1 | pending |  |
| GAP-misc.prefs-documents | [ ] Preferences: documents (autosave interval, recent count) | P1 | pending |  |
| GAP-misc.prefs-forms | [ ] Preferences: forms | P1 | pending |  |
| GAP-misc.prefs-general | [ ] Preferences: general | P1 | pending |  |
| GAP-misc.prefs-identity | [ ] Preferences: identity | P1 | pending |  |
| GAP-misc.prefs-language | [ ] Preferences: language | P1 | pending | REC-A10 |
| GAP-misc.prefs-page-display | [ ] Preferences: page display | P1 | pending |  |
| GAP-misc.prefs-security | [ ] Preferences: security and trust | P1 | pending |  |
| GAP-misc.prefs-signatures | [ ] Preferences: signatures | P1 | pending |  |
| GAP-misc.user-docs | [ ] User documentation and keyboard reference | P1 | pending |  |
| GAP-misc.web-deploy | [ ] Hosted web app deployment | P1 | pending |  |
| GAP-ocr.background-removal | [ ] Background removal | P1 | pending | REC-O04 |
| GAP-ocr.camera-perspective | [ ] Enhance camera images (edges, perspective, contrast) | P1 | pending | REC-O04; REC-M03 |
| GAP-ocr.deskew | [ ] Deskew | P1 | pending | REC-O03 |
| GAP-ocr.despeckle | [ ] Despeckle | P1 | pending | REC-O04 |
| GAP-ocr.ocr-engines | [ ] Pluggable OCR engines (ocrs default; tesseract optional) | P1 | pending | REC-O01 |
| GAP-ocr.rotation-detection | [ ] Detect and fix page rotation | P1 | pending | REC-O03 |
| GAP-optimize.compatibility-level | [ ] Make compatible with a chosen PDF version | P1 | pending |  |
| GAP-optimize.compress-structure | [ ] Compress document structure (object streams) | P1 | pending |  |
| GAP-optimize.downsample-images | [ ] Downsample colour, grey and mono images | P1 | pending |  |
| GAP-optimize.flate-unencoded | [ ] Flate-encode unencoded streams; replace LZW with Flate | P1 | pending |  |
| GAP-optimize.flatten-transparency | [ ] Flatten transparency | P1 | pending |  |
| GAP-optimize.mono-compression | [ ] Monochrome compression (JBIG2, CCITT G4) | P1 | pending |  |
| GAP-optimize.optimizer | [ ] Advanced optimization (PDF Optimizer) | P1 | pending | REC-X08 |
| GAP-optimize.output-intents | [ ] Choose output intents and conditions | P1 | pending |  |
| GAP-optimize.pdfa-accessible-levels | [ ] PDF/A conformance levels a and u | P1 | pending |  |
| GAP-optimize.pdfa-convert | [ ] Save as PDF/A (1b, 2b, 3b, 4) | P1 | pending | REC-Q02 |
| GAP-optimize.pdfa-validate | [ ] Verify PDF/A compliance (1-4) | P1 | pending | REC-Q02 |
| GAP-optimize.pdfua-validate | [ ] Verify PDF/UA-1 and PDF/UA-2 | P1 | pending | REC-A09 |
| GAP-optimize.preflight-reports | [ ] Preflight reports (PDF, XML, text) | P1 | pending |  |
| GAP-optimize.preflight-results | [ ] Preflight results tree with object details | P1 | pending |  |
| GAP-optimize.preflight-single-checks | [ ] Single checks | P1 | pending |  |
| GAP-optimize.preflight-single-fixups | [ ] Single fixups | P1 | pending |  |
| GAP-optimize.recompress-images | [ ] Recompress images (JPEG, JPEG 2000, Flate) | P1 | pending |  |
| GAP-optimize.standards-panel | [ ] Standards panel showing declared conformance | P1 | pending |  |
| GAP-organize.bookmark-actions | [ ] Bookmark actions | P1 | pending |  |
| GAP-organize.field-name-conflicts | [ ] Rename or merge colliding field names when combining | P1 | pending | REC-P04 |
| GAP-organize.insert-by-drop | [ ] Insert by dropping files onto the page grid | P1 | pending | REC-P02 |
| GAP-organize.insert-from-clipboard | [ ] Insert pages from the clipboard | P1 | pending |  |
| GAP-organize.page-range-selection | [ ] Select by range, odd/even, landscape/portrait | P1 | pending | REC-P01 |
| GAP-organize.page-tab-order | [ ] Page tab order property | P1 | pending |  |
| GAP-organize.struct-tree-merge | [ ] Merge structure trees when combining | P1 | pending | REC-P05 |
| GAP-print.color-management | [ ] Colour management settings (working spaces, intents) | P1 | pending |  |
| GAP-print.color-warnings | [ ] Overprint and rich-black warnings | P1 | pending |  |
| GAP-print.fix-hairlines | [ ] Fix hairlines | P1 | pending |  |
| GAP-print.grayscale | [ ] Print in grayscale | P1 | pending | REC-Q06 |
| GAP-print.object-inspector | [ ] Object inspector | P1 | pending |  |
| GAP-print.page-setup | [ ] Page setup | P1 | pending | REC-Q05 |
| GAP-print.print-as-image | [ ] Print as image | P1 | pending | REC-Q06 |
| GAP-protect.attachment-trust | [ ] Block opening risky attachment types | P1 | pending | REC-S02 |
| GAP-protect.redact-pattern-locales | [ ] Pattern sets per locale | P1 | pending | REC-S04 |
| GAP-protect.redaction-codes | [ ] Redaction code sets (e.g. FOIA exemptions) | P1 | pending |  |
| GAP-protect.url-access-control | [ ] Control internet access from PDFs | P1 | pending | REC-S03 |
| GAP-sign.compare-signed-version | [ ] Compare signed version with the current one | P1 | pending | REC-D01 |
| GAP-sign.document-timestamp | [ ] Add a document timestamp (RFC 3161) | P1 | pending |  |
| GAP-sign.field-mdp | [ ] Field locks (FieldMDP) | P1 | pending | REC-F08; REC-D05 |
| GAP-sign.lock-after-signing | [ ] Lock document or fields after signing | P1 | pending | REC-D05 |
| GAP-sign.os-trust | [ ] Trust the operating system's root certificates | P1 | pending | REC-D02 |
| GAP-sign.signature-appearance | [ ] Configure signature appearance (text, graphic, name) | P1 | pending | REC-D06 |
| GAP-sign.timestamp-servers | [ ] Configure timestamp servers | P1 | pending | REC-D02 |
| GAP-sign.trust-import | [ ] Import trusted certificates and set trust | P1 | pending | REC-D02 |
| GAP-sign.verification-time | [ ] Verification time (signing time, timestamp, now) | P1 | pending |  |
| GAP-view.advanced-search | [ ] Advanced search window | P1 | pending | REC-V01 |
| GAP-view.multiple-windows | [ ] Multiple windows; drag a tab out into a window | P1 | pending | REC-V04 |
| GAP-view.search-folder | [ ] Search all PDFs in a folder | P1 | pending | REC-V01 |
| REC-A03 | [ ] Reading order Tags and Content remediation tools | P1 | pending | a11y.reading-order-tool a11y.tags-panel a11y.content-panel a11y.order-panel |
| REC-A05 | [ ] Accessible table editor and header associations | P1 | pending | a11y.table-editor |
| REC-A06 | [ ] Guided make-accessible workflow and reports | P1 | pending | a11y.make-accessible-action a11y.setup-assistant |
| REC-A09 | [ ] PDF/UA-1 and PDF/UA-2 validation adapters | P1 | pending | optimize.pdfua-validate |
| REC-A10 | [ ] Translation quality and non-Latin font coverage gates | P1 | pending | misc.prefs-language |
| REC-B01 | [ ] Complete tool coverage for shipped GUI workflows | P1 | pending | misc.cli-completeness |
| REC-B04 | [ ] Folder watch and recursive batch workflows | P1 | pending | misc.batch-processing |
| REC-B05 | [ ] Portable workflow presets and action library | P1 | pending | misc.action-wizard-manage misc.custom-commands |
| REC-B06 | [ ] Checkpoint resume retry and idempotency | P1 | pending | '- |
| REC-B10 | [ ] Machine-readable validation and evidence artifacts | P1 | pending | '- |
| REC-C02 | [ ] Reusable team markup and stamp libraries | P1 | pending | comment.stamps-manage misc.custom-tools |
| REC-C03 | [ ] Advanced comment filtering and saved views | P1 | pending | comment.search-comments |
| REC-C04 | [ ] Comment status owner deadline and resolution | P1 | pending | '- |
| REC-C05 | [ ] Comment summaries with page context | P1 | pending | comment.summarize |
| REC-C06 | [ ] Import selected comments from revised PDFs | P1 | pending | comment.import-from-pdf comment.export-selected |
| REC-C07 | [ ] Migrate comments to revised drawings | P1 | pending | comment.migrate |
| REC-C10 | [ ] Read-only final review export versus editable original | P1 | pending | '- |
| REC-D02 | [ ] Trusted roots and explicit timestamp configuration | P1 | pending | sign.trust-import sign.timestamp-servers sign.os-trust |
| REC-D05 | [ ] Signature field-lock and certification preview | P1 | pending | sign.lock-after-signing sign.field-mdp sign.preview-mode |
| REC-D06 | [ ] Configurable reusable signature appearances | P1 | pending | sign.signature-appearance |
| REC-D10 | [ ] Signature evidence package and verification report | P1 | pending | '- |
| REC-E02 | [ ] Arabic and mixed-script editing with correct search/copy | P1 | pending | edit.rtl-cjk-editing |
| REC-E04 | [ ] Find and replace with preview | P1 | pending | edit.find-replace |
| REC-E05 | [ ] Per-character rich formatting and paragraph controls | P1 | pending | edit.line-paragraph-spacing edit.lists edit.text-super-subscript |
| REC-E07 | [ ] Crop replace and relink existing images | P1 | pending | edit.image-crop edit.image-external-editor |
| REC-E08 | [ ] Arrange align distribute and group objects | P1 | pending | edit.arrange-objects edit.align-distribute-objects edit.edit-object-tool |
| REC-E10 | [ ] Preserve document tags and reading order during edits | P1 | pending | edit.keep-tags-on-edit |
| REC-E11 | [ ] Reusable headers footers watermarks and Bates profiles | P1 | pending | edit.decoration-profiles edit.bates-numbering edit.header-footer-shrink |
| REC-E12 | [ ] Named snapshots and before/after edit inspection | P1 | pending | core.history-panel |
| REC-F02 | [ ] Better automatic field detection and correction | P1 | pending | form.auto-detect-fields |
| REC-F03 | [ ] Field layout tools and repeated grids | P1 | pending | form.multiple-copies form.show-tab-numbers |
| REC-F04 | [ ] Encrypted local autofill profiles | P1 | pending | form.auto-complete |
| REC-F05 | [ ] Reusable bilingual business form templates | P1 | pending | '- |
| REC-F06 | [ ] Safe form submission with destination preview | P1 | pending | form.submit-form |
| REC-F08 | [ ] Field locks and signature dependencies | P1 | pending | form.props-signed sign.field-mdp |
| REC-F09 | [ ] XFA capability detection flattening and fallback | P1 | pending | form.xfa-static form.xfa-dynamic form.xfa-flatten |
| REC-H01 | [ ] One Share/Send workflow with explicit output type | P1 | pending | '- |
| REC-H02 | [ ] Send a copy through native email/share sheet | P1 | pending | '- |
| REC-H03 | [ ] Compress-to-email copy and page-range sharing | P1 | pending | '- |
| REC-H05 | [ ] Offline review package with PDF XFDF and manifest | P1 | pending | '- |
| REC-I01 | [ ] Extend existing Nextcloud host integration safely | P1 | pending | '- |
| REC-I04 | [ ] SMB WebDAV and local network share robustness | P1 | pending | '- |
| REC-I05 | [ ] Outlook email attachment workflow | P1 | pending | '- |
| REC-I06 | [ ] Explorer/Finder quick actions and thumbnails | P1 | pending | '- |
| REC-I07 | [ ] Company template and preset distribution | P1 | pending | '- |
| REC-N01 | [ ] Drawing-set register with sheet and revision metadata | P1 | pending | '- |
| REC-N02 | [ ] Aligned colour overlays and visual drawing comparison | P1 | pending | misc.compare-scanned misc.compare-filters |
| REC-N05 | [ ] Calibrated multi-viewport measurements and units | P1 | pending | comment.measure-scale comment.measure-distance |
| REC-N08 | [ ] RFIs submittals and punch-list issue pins | P1 | pending | '- |
| REC-N09 | [ ] Controlled transmittals and approval stamps | P1 | pending | '- |
| REC-O01 | [ ] Arabic and bilingual OCR engine support | P1 | pending | ocr.ocr-languages ocr.ocr-engines |
| REC-O02 | [ ] Accented Latin and additional language packs | P1 | pending | ocr.ocr-languages |
| REC-O03 | [ ] Rotation detection and deskew | P1 | pending | ocr.rotation-detection ocr.deskew |
| REC-O04 | [ ] Despeckle background shadow and perspective cleanup | P1 | pending | ocr.despeckle ocr.background-removal ocr.edge-shadow-removal ocr.camera-perspective |
| REC-O05 | [ ] Low-confidence OCR correction interface | P1 | pending | ocr.correct-recognized-text |
| REC-O07 | [ ] Batch OCR queue with resume and language presets | P1 | pending | misc.batch-processing |
| REC-O08 | [ ] Native scanner acquisition and feeder support | P1 | pending | ocr.scanner-acquire ocr.scan-presets ocr.scan-append |
| REC-O12 | [ ] OCR quality and duplicate-layer health report | P1 | pending | ocr.searchable-image |
| REC-P01 | [ ] Complete page-range selection and validation | P1 | pending | organize.page-range-selection |
| REC-P02 | [ ] Drag or copy pages across open documents | P1 | pending | organize.drag-pages-between-documents organize.insert-by-drop |
| REC-P03 | [ ] Safe split naming and multi-file splitting | P1 | pending | organize.split-naming organize.split-multiple-files |
| REC-P05 | [ ] Preserve tags bookmarks and internal links while merging | P1 | pending | organize.struct-tree-merge |
| REC-P06 | [ ] Resize pages and remove white margins | P1 | pending | organize.change-page-size organize.remove-white-margins |
| REC-P07 | [ ] Attachment and associated-file management | P1 | pending | organize.attachments-add organize.attachments-delete organize.associated-files |
| REC-P08 | [ ] Document-set manifest and cover/index generation | P1 | pending | '- |
| REC-Q02 | [ ] Complete PDF/A validation and conversion coverage | P1 | pending | optimize.pdfa-validate optimize.pdfa-convert |
| REC-Q06 | [ ] Print-as-image and raster grayscale fallback | P1 | pending | print.print-as-image print.grayscale |
| REC-R11 | [ ] Privacy-preserving diagnostic export | P1 | pending | '- |
| REC-S04 | [ ] Locale-aware PII search and partial redaction | P1 | pending | protect.redact-pattern-locales protect.redact-partial-words |
| REC-S05 | [ ] Batch folder redaction with reports | P1 | pending | protect.redact-folder protect.redact-filename-suffix |
| REC-S07 | [ ] Managed document security presets | P1 | pending | protect.security-policies |
| REC-T02 | [ ] MSI silent installation uninstall and managed update policy | P1 | pending | '- |
| REC-T03 | [ ] Portable and offline-install packages | P1 | pending | '- |
| REC-T04 | [ ] Administrator-managed security and provider settings | P1 | pending | '- |
| REC-T07 | [ ] Accessible onboarding and workflow guides | P1 | pending | '- |
| REC-T10 | [ ] Support/export compatibility repair assistant | P1 | pending | '- |
| REC-V01 | [ ] Search all open documents and local folders | P1 | pending | view.advanced-search view.search-folder |
| REC-V02 | [ ] Search comments bookmarks and attachments metadata | P1 | pending | view.find-in-comments-bookmarks comment.search-comments |
| REC-V04 | [ ] Split view and tear-out document windows | P1 | pending | view.split-window view.multiple-windows |
| REC-V05 | [ ] Synchronized side-by-side comparison | P1 | pending | misc.compare-side-by-side |
| REC-V06 | [ ] Pinned favorite and recent-tool shortcuts | P1 | pending | view.customize-quick-tools misc.custom-tools |
| REC-V07 | [ ] Task-first Home and command discovery | P1 | pending | view.home-view |
| REC-V08 | [ ] Remember per-document reading state safely | P1 | pending | '- |
| REC-V09 | [ ] Accessible reflow and reading mode | P1 | pending | view.reflow a11y.reflow-tagged |
| REC-V10 | [ ] High-contrast reading colours that survive annotation | P1 | pending | a11y.replace-page-colors |
| REC-V11 | [ ] RTL reading order across spreads and navigation | P1 | pending | edit.rtl-cjk-editing |
| REC-V12 | [ ] Actionable errors progress and inline help | P1 | pending | '- |
| REC-W05 | [ ] Progressive rendering and bandwidth-conscious opening | P1 | pending | core.linearization core.lazy-loading |
| REC-W06 | [ ] Print handoff with truthful browser limitations | P1 | pending | '- |
| REC-W08 | [ ] Mobile-responsive browser reading and review | P1 | pending | '- |
| REC-W09 | [ ] Background worker isolation and job throttling | P1 | pending | '- |
| REC-X01 | [ ] High-fidelity Word export with layout options | P1 | pending | create.export-docx |
| REC-X02 | [ ] PDF tables to Excel or CSV | P1 | pending | create.export-xlsx |
| REC-X04 | [ ] Office-to-PDF local conversion provider | P1 | pending | create.from-office |
| REC-X07 | [ ] Reusable per-format export presets | P1 | pending | create.export-settings |
| REC-X08 | [ ] Smart size-target compression with comparison preview | P1 | pending | optimize.optimizer optimize.optimizer-presets |
| REC-X09 | [ ] Automatic OCR-on-export for image-only pages | P1 | pending | create.export-ocr |
| GAP-a11y.content-panel | [ ] Content panel | P2 | pending | REC-A03 |
| GAP-a11y.reflow-tagged | [ ] Reflow tagged content | P2 | pending | REC-V09 |
| GAP-a11y.replace-page-colors | [ ] Replace page colours (high contrast) | P2 | pending | REC-V10 |
| GAP-a11y.setup-assistant | [ ] Accessibility setup assistant | P2 | pending | REC-A06 |
| GAP-a11y.table-editor | [ ] Table editor (headers, scope, spans) | P2 | pending | REC-A05 |
| GAP-comment.export-selected | [ ] Export selected comments | P2 | pending | REC-C06 |
| GAP-comment.import-from-pdf | [ ] Import comments from another PDF | P2 | pending | REC-C06 |
| GAP-comment.measure-distance | [ ] Distance measuring tool | P2 | pending | REC-N05 |
| GAP-comment.measure-scale | [ ] Scale ratio and calibration per viewport | P2 | pending | REC-N05 |
| GAP-comment.stamps-manage | [ ] Manage stamp categories and imports | P2 | pending | REC-C02 |
| GAP-core.history-panel | [ ] History panel with named snapshots | P2 | pending | REC-E12 |
| GAP-create.export-settings | [ ] Per-format export settings | P2 | pending | REC-X07 |
| GAP-create.from-office | [ ] Create PDF from Office documents (LibreOffice sidecar) | P2 | pending | REC-X04 |
| GAP-edit.decoration-profiles | [ ] Saved header/footer and watermark settings | P2 | pending | REC-E11 |
| GAP-edit.find-replace | [ ] Find and replace in edit mode | P2 | pending | REC-E04 |
| GAP-edit.header-footer-shrink | [ ] Shrink document to avoid overwriting content | P2 | pending | REC-E11 |
| GAP-edit.image-external-editor | [ ] Edit an image in an external application | P2 | pending | REC-E07 |
| GAP-edit.text-super-subscript | [ ] Superscript and subscript | P2 | pending | REC-E05 |
| GAP-form.auto-complete | [ ] Auto-complete from previous entries | P2 | pending | REC-F04 |
| GAP-form.multiple-copies | [ ] Create multiple copies of a field in a grid | P2 | pending | REC-F03 |
| GAP-form.show-tab-numbers | [ ] Show tab numbers | P2 | pending | REC-F03 |
| GAP-form.xfa-flatten | [ ] Flatten XFA forms | P2 | pending | REC-F09 |
| GAP-misc.action-wizard-manage | [ ] Import, export, copy and delete actions | P2 | pending | REC-B05 |
| GAP-misc.compare-filters | [ ] Filter changes by type (text, images, formatting) | P2 | pending | REC-N02 |
| GAP-misc.compare-scanned | [ ] Compare scanned documents via OCR | P2 | pending | REC-N02 |
| GAP-misc.custom-tools | [ ] Custom tool sets | P2 | pending | REC-V06; REC-C02 |
| GAP-ocr.correct-recognized-text | [ ] Correct recognized text (low-confidence suspects) | P2 | pending | REC-O05 |
| GAP-ocr.edge-shadow-removal | [ ] Edge shadow removal | P2 | pending | REC-O04 |
| GAP-ocr.scan-append | [ ] Append scans to an existing PDF or split into files | P2 | pending | REC-O08 |
| GAP-ocr.scan-presets | [ ] Scan presets | P2 | pending | REC-O08 |
| GAP-ocr.scanner-acquire | [ ] Scan from TWAIN/WIA/ICA/SANE scanners | P2 | pending | REC-O08 |
| GAP-optimize.optimizer-presets | [ ] Optimizer presets | P2 | pending | REC-X08 |
| GAP-organize.associated-files | [ ] Associated files with relationships (PDF/A-3, PDF 2.0) | P2 | pending | REC-P07 |
| GAP-organize.attachments-add | [ ] Add file attachments | P2 | pending | REC-P07 |
| GAP-organize.attachments-delete | [ ] Delete attachments | P2 | pending | REC-P07 |
| GAP-organize.change-page-size | [ ] Change page size | P2 | pending | REC-P06 |
| GAP-organize.drag-pages-between-documents | [ ] Drag pages between documents | P2 | pending | REC-P02 |
| GAP-organize.remove-white-margins | [ ] Remove white margins | P2 | pending | REC-P06 |
| GAP-organize.split-multiple-files | [ ] Split multiple files in one run | P2 | pending | REC-P03 |
| GAP-organize.split-naming | [ ] Name split parts after bookmarks; never overwrite | P2 | pending | REC-P03 |
| GAP-print.paper-source-by-size | [ ] Choose paper source by PDF page size | P2 | pending | REC-Q05 |
| GAP-protect.protected-view | [ ] Open untrusted files in a restricted view | P2 | pending | REC-S02 |
| GAP-protect.redact-folder | [ ] Search and redact across a folder | P2 | pending | REC-S05 |
| GAP-protect.redact-partial-words | [ ] Redact part of a match (e.g. all but the last four digits) | P2 | pending | REC-S04 |
| GAP-protect.security-policies | [ ] Local security policies (saved presets) | P2 | pending | REC-S07 |
| GAP-sign.preview-mode | [ ] Preview document mode before signing | P2 | pending | REC-D05 |
| GAP-view.check-for-updates | [ ] Help ▸ Check for updates | P2 | pending | REC-R10 |
| GAP-view.customize-quick-tools | [ ] Customise the quick-action toolbar and pinned tools | P2 | pending | REC-V06 |
| GAP-view.find-in-comments-bookmarks | [ ] Find in comments and bookmarks | P2 | pending | REC-V02 |
| GAP-view.reflow | [ ] Reflow text to the window width | P2 | pending | REC-V09 |
| GAP-view.split-window | [ ] Split window (two panes on one document) | P2 | pending | REC-V04 |
| GAP-comment.migrate | [ ] Migrate comments to a revised document | P3 | pending | REC-C07 |
| GAP-protect.redact-filename-suffix | [ ] Add a suffix to redacted file names | P3 | pending | REC-S05 |

## Advanced (210 records)

pending: 210

| ID | TODO | Priority | Status | Related tasks |
|---|---|---|---|---|
| GAP-form.js-host | [ ] JavaScript engine with the Acrobat object model subset | P0 | pending | REC-F10 |
| REC-H11 | [ ] Version history and optimistic concurrency for shared files | P0 | pending | '- |
| GAP-a11y.autotag | [ ] Automatically tag a document | P1 | pending | REC-A04 |
| GAP-a11y.autotag-fields | [ ] Autotag form fields | P1 | pending | REC-A04 |
| GAP-core.open-public-key | [ ] Open certificate-encrypted (public-key handler) documents | P1 | pending | REC-Q08 |
| GAP-create.export-accessible-text | [ ] Export accessible text (from tags) | P1 | pending | REC-X06 |
| GAP-create.export-pptx | [ ] Export to PowerPoint (.pptx) | P1 | pending | REC-X03 |
| GAP-edit.vector-edit | [ ] Select, move, scale, recolour and delete vector paths | P1 | pending | REC-E09 |
| GAP-form.js-debugger | [ ] JavaScript debugger | P1 | pending | REC-F10 |
| GAP-ocr.editable-text-output | [ ] Editable text and images output | P1 | pending | REC-O06 |
| GAP-ocr.ocr-web | [ ] OCR in the browser | P1 | pending | REC-O09 |
| GAP-optimize.pdfx-convert | [ ] Save as PDF/X (1a, 3, 4) | P1 | pending | REC-Q03 |
| GAP-optimize.pdfx-validate | [ ] Verify PDF/X compliance | P1 | pending | REC-Q03 |
| GAP-optimize.preflight | [ ] Preflight analysis | P1 | pending | REC-Q01 |
| GAP-optimize.preflight-fix | [ ] Preflight analyze and fix | P1 | pending | REC-Q01 |
| GAP-optimize.preflight-profiles | [ ] Built-in preflight profiles | P1 | pending | REC-Q01 |
| GAP-print.convert-colors | [ ] Convert colours | P1 | pending | REC-Q07 |
| GAP-print.ink-manager | [ ] Ink manager | P1 | pending | REC-Q07 |
| GAP-print.output-preview | [ ] Output preview | P1 | pending | REC-Q04 |
| GAP-print.overprint-preview | [ ] Overprint preview | P1 | pending | REC-Q04 |
| GAP-print.printer-marks | [ ] Add printer marks (crop, bleed, registration, colour bars) | P1 | pending | REC-Q07 |
| GAP-print.separations | [ ] Separations preview with area coverage | P1 | pending | REC-Q04 |
| GAP-print.total-area-coverage | [ ] Total area coverage warning | P1 | pending | REC-Q07 |
| GAP-protect.certificate-encryption | [ ] Encrypt with certificates for recipients | P1 | pending | REC-S06 |
| GAP-protect.recipient-permissions | [ ] Per-recipient permissions for certificate security | P1 | pending | REC-S06 |
| GAP-sign.ltv | [ ] Long-term validation (DSS/VRI) | P1 | pending | REC-D03 |
| GAP-sign.ocsp-crl | [ ] Revocation checking (OCSP and CRL) | P1 | pending | REC-D03 |
| GAP-sign.pades-blta | [ ] PAdES B-LT and B-LTA | P1 | pending | REC-D03 |
| GAP-sign.pades-bt | [ ] PAdES B-T (signature timestamp) | P1 | pending | REC-D03 |
| GAP-sign.pkcs11 | [ ] Sign with PKCS#11 tokens and smart cards | P1 | pending | REC-D04 |
| GAP-sign.web-signing | [ ] Sign in the browser with a PKCS#12 file | P1 | pending | REC-D08 |
| GAP-a11y.create-tag-from-selection | [ ] Create a tag from a selection | P2 | pending |  |
| GAP-a11y.find-untagged | [ ] Find unmarked content, comments and links | P2 | pending |  |
| GAP-a11y.keyboard-selection-cursor | [ ] Always show the keyboard selection cursor | P2 | pending |  |
| GAP-a11y.read-aloud | [ ] Read out loud (platform TTS) | P2 | pending | REC-A07 |
| GAP-a11y.read-aloud-options | [ ] Read-aloud voice, pitch and speed | P2 | pending | REC-A07 |
| GAP-a11y.role-map | [ ] Role map editor | P2 | pending |  |
| GAP-comment.connector-lines | [ ] Connector lines between markups and pop-ups | P2 | pending |  |
| GAP-comment.image-comment | [ ] Add an image as a comment | P2 | pending | REC-C09 |
| GAP-comment.stamps-rotate | [ ] Rotate stamps with a handle | P2 | pending |  |
| GAP-core.deterministic-save | [ ] Deterministic full-save mode | P2 | pending | REC-Q10 |
| GAP-core.pdf20-dpart | [ ] PDF 2.0 document parts (DPart) | P2 | pending |  |
| GAP-create.export-eps | [ ] Export to EPS | P2 | pending |  |
| GAP-create.export-jpeg2000 | [ ] Export pages as JPEG 2000 | P2 | pending |  |
| GAP-create.export-ps | [ ] Export to PostScript | P2 | pending |  |
| GAP-create.export-xml | [ ] Export to XML | P2 | pending |  |
| GAP-create.from-html | [ ] Create PDF from an HTML file | P2 | pending | REC-X05 |
| GAP-create.from-markdown | [ ] Create PDF from Markdown | P2 | pending | REC-X06 |
| GAP-create.from-web-page | [ ] Create PDF from a web page URL | P2 | pending | REC-X05 |
| GAP-create.image-conversion-options | [ ] Image compression options when converting | P2 | pending |  |
| GAP-edit.action-launch | [ ] Open-a-file action | P2 | pending |  |
| GAP-edit.action-named | [ ] Execute-a-menu-item action (safe list) | P2 | pending |  |
| GAP-edit.action-set-ocg-state | [ ] Set-layer-visibility action | P2 | pending |  |
| GAP-edit.object-properties | [ ] Object properties (colour space, font, convert colour) | P2 | pending | REC-E09 |
| GAP-edit.remove-all-links | [ ] Remove all links | P2 | pending |  |
| GAP-edit.rotated-text-editing | [ ] Edit rotated text | P2 | pending | REC-E06 |
| GAP-edit.spell-check | [ ] Spell check in comments, form fields and edited text | P2 | pending |  |
| GAP-edit.spell-dictionaries | [ ] Spelling dictionaries per language and a user dictionary | P2 | pending |  |
| GAP-form.add-barcode-field | [ ] Barcode fields (PDF417, QR, Data Matrix) | P2 | pending |  |
| GAP-form.edit-all-javascripts | [ ] Edit all JavaScripts in one editor | P2 | pending | REC-F10 |
| GAP-form.props-selection-change | [ ] Field properties: selection change | P2 | pending |  |
| GAP-form.rich-text-fields | [ ] Rich-text field values | P2 | pending |  |
| GAP-form.xfa-data | [ ] XFA XML data import and export | P2 | pending |  |
| GAP-misc.action-javascript-step | [ ] Execute JavaScript step | P2 | pending |  |
| GAP-misc.action-prompt-steps | [ ] Prompt-user and instruction steps | P2 | pending |  |
| GAP-misc.action-publish-web | [ ] Built-in action: prepare for web publishing | P2 | pending |  |
| GAP-misc.admin-preferences | [ ] Admin-deployable preference defaults | P2 | pending |  |
| GAP-misc.compare-page-ranges | [ ] Compare page ranges | P2 | pending |  |
| GAP-misc.prefs-content-editing | [ ] Preferences: content editing | P2 | pending |  |
| GAP-misc.prefs-convert | [ ] Preferences: convert to and from PDF | P2 | pending |  |
| GAP-misc.prefs-full-screen | [ ] Preferences: full screen | P2 | pending |  |
| GAP-misc.prefs-measuring | [ ] Preferences: measuring | P2 | pending |  |
| GAP-misc.prefs-reading | [ ] Preferences: reading | P2 | pending |  |
| GAP-misc.prefs-search | [ ] Preferences: search | P2 | pending |  |
| GAP-misc.prefs-spelling | [ ] Preferences: spelling | P2 | pending |  |
| GAP-misc.prefs-units-guides | [ ] Preferences: units and guides | P2 | pending |  |
| GAP-misc.rtl-ui | [ ] Right-to-left user interface | P2 | pending |  |
| GAP-ocr.adaptive-compression | [ ] Adaptive (MRC) compression of scans | P2 | pending | REC-O11 |
| GAP-ocr.camera-document-type | [ ] Camera image document types (whiteboard, card, form) | P2 | pending |  |
| GAP-ocr.descreen | [ ] Descreen | P2 | pending |  |
| GAP-ocr.ocr-downsample | [ ] Downsample images during OCR | P2 | pending | REC-O11 |
| GAP-ocr.scan-pdfa | [ ] Make scans PDF/A compliant | P2 | pending |  |
| GAP-optimize.batch-optimize | [ ] Optimize multiple files | P2 | pending |  |
| GAP-optimize.discard-external-refs | [ ] Discard external cross-references | P2 | pending |  |
| GAP-optimize.discard-form-actions | [ ] Discard form submit, import and reset actions | P2 | pending |  |
| GAP-optimize.discard-hidden-layers | [ ] Discard hidden layers and flatten visible ones | P2 | pending |  |
| GAP-optimize.discard-search-index | [ ] Discard embedded search index | P2 | pending |  |
| GAP-optimize.optimize-content | [ ] Optimize page content streams | P2 | pending |  |
| GAP-optimize.pdfe | [ ] PDF/E | P2 | pending |  |
| GAP-optimize.pdfvt | [ ] PDF/VT | P2 | pending |  |
| GAP-optimize.preflight-custom-profiles | [ ] Create and edit custom profiles | P2 | pending |  |
| GAP-optimize.preflight-droplets | [ ] Preflight droplets and batch preflight | P2 | pending |  |
| GAP-optimize.preflight-profile-exchange | [ ] Import, export and lock profiles | P2 | pending |  |
| GAP-optimize.preflight-snap-view | [ ] Snap view of a flagged object | P2 | pending |  |
| GAP-optimize.remove-standards-id | [ ] Remove PDF/A or PDF/X identification | P2 | pending |  |
| GAP-optimize.unembed-fonts | [ ] Unembed fonts | P2 | pending |  |
| GAP-organize.attachments-description | [ ] Edit attachment descriptions | P2 | pending |  |
| GAP-organize.bookmark-appearance | [ ] Bookmark style and colour | P2 | pending |  |
| GAP-organize.combine-options | [ ] Combine options (file size, convert to PDF/A, include open files) | P2 | pending |  |
| GAP-organize.custom-properties | [ ] Custom document properties | P2 | pending |  |
| GAP-organize.insert-from-scanner | [ ] Insert pages from a scanner | P2 | pending |  |
| GAP-organize.insert-from-web | [ ] Insert pages from a web page | P2 | pending |  |
| GAP-organize.page-actions | [ ] Page open/close actions | P2 | pending |  |
| GAP-organize.page-transitions | [ ] Page transitions | P2 | pending |  |
| GAP-organize.portfolio-extract | [ ] Extract files from a portfolio | P2 | pending | REC-P09 |
| GAP-organize.portfolio-view | [ ] View PDF Portfolios as a file list | P2 | pending | REC-P09 |
| GAP-organize.print-presets | [ ] Print dialog presets in document properties | P2 | pending |  |
| GAP-organize.xmp-editor | [ ] Additional metadata (XMP) editor | P2 | pending |  |
| GAP-print.advanced-print-separations | [ ] Print separations and in-RIP separations | P2 | pending |  |
| GAP-print.flattener-preview | [ ] Flattener preview | P2 | pending |  |
| GAP-print.ink-aliases | [ ] Ink aliases and spot-to-process | P2 | pending |  |
| GAP-print.postscript-output | [ ] PostScript Level 3 output | P2 | pending |  |
| GAP-print.print-marks-bleeds | [ ] Marks and bleeds when printing | P2 | pending |  |
| GAP-print.print-with-comment-summary | [ ] Print with a comments summary | P2 | pending |  |
| GAP-print.simulate-paper-ink | [ ] Simulate paper colour and black ink | P2 | pending |  |
| GAP-protect.encrypt-attachments-only | [ ] Encrypt only file attachments | P2 | pending |  |
| GAP-protect.redact-ai-suggestions | [ ] Suggest sensitive content to redact via an AI provider | P2 | pending | REC-Z04 |
| GAP-sign.clear-signature | [ ] Clear an unsaved signature; clear all signature fields | P2 | pending |  |
| GAP-sign.qualified-labels | [ ] Show qualified signature/seal labels | P2 | pending |  |
| GAP-sign.seed-values | [ ] Signature seed values | P2 | pending |  |
| GAP-view.auto-scroll | [ ] Automatic scrolling | P2 | pending |  |
| GAP-view.copy-with-layout | [ ] Copy text keeping layout | P2 | pending |  |
| GAP-view.cursor-coordinates | [ ] Cursor coordinates | P2 | pending |  |
| GAP-view.destinations-panel | [ ] Named destinations panel | P2 | pending |  |
| GAP-view.dynamic-zoom | [ ] Dynamic zoom (drag to zoom) | P2 | pending |  |
| GAP-view.embedded-index | [ ] Embedded search index | P2 | pending |  |
| GAP-view.full-screen-presentation | [ ] Full-screen presentation options (auto-advance, loop, click to advance) | P2 | pending |  |
| GAP-view.grid | [ ] Grid and snap to grid | P2 | pending |  |
| GAP-view.guides | [ ] Guides | P2 | pending |  |
| GAP-view.highlight-current-bookmark | [ ] Highlight the current bookmark | P2 | pending |  |
| GAP-view.layers-reset | [ ] Reset layers to initial visibility; list for all or current pages | P2 | pending |  |
| GAP-view.line-weights | [ ] Show line weights | P2 | pending |  |
| GAP-view.loupe | [ ] Loupe magnifier tool | P2 | pending |  |
| GAP-view.page-gaps | [ ] Show or hide gaps between pages | P2 | pending |  |
| GAP-view.pan-and-zoom | [ ] Pan and zoom overview window | P2 | pending |  |
| GAP-view.pdfa-view-mode | [ ] Read-only view for PDF/A files with an Enable editing bar | P2 | pending |  |
| GAP-view.rulers | [ ] Rulers | P2 | pending |  |
| GAP-view.search-boolean | [ ] Boolean, proximity and stemming search | P2 | pending |  |
| GAP-view.search-index | [ ] Build and search a full-text index across files | P2 | pending | REC-V03 |
| GAP-view.show-page-boxes | [ ] Show art, trim and bleed boxes | P2 | pending |  |
| GAP-view.single-key-accelerators | [ ] Single-key tool accelerators | P2 | pending |  |
| GAP-view.spreadsheet-split | [ ] Spreadsheet split (four synchronised panes) | P2 | pending |  |
| REC-A04 | [ ] Automatic tagging with mandatory review | P2 | pending | a11y.autotag a11y.autotag-fields |
| REC-A07 | [ ] Read aloud with word/page tracking | P2 | pending | a11y.read-aloud a11y.read-aloud-options |
| REC-B08 | [ ] Optional authenticated service API and webhooks | P2 | pending | '- |
| REC-C08 | [ ] Comfortable pen input and palm rejection | P2 | pending | '- |
| REC-C09 | [ ] Image/photo/audio comments with transcripts | P2 | pending | comment.image-comment comment.record-audio |
| REC-D03 | [ ] Complete PAdES timestamp and long-term-validation workflows | P2 | pending | sign.pades-bt sign.ltv sign.pades-blta sign.ocsp-crl |
| REC-D04 | [ ] PKCS#11 tokens and smart-card support | P2 | pending | sign.pkcs11 |
| REC-D07 | [ ] Multi-recipient signing envelopes | P2 | pending | sign.request-esignatures |
| REC-D08 | [ ] Browser signing with strong key-handling policy | P2 | pending | sign.web-signing |
| REC-E03 | [ ] Complete CJK and complex-script editing coverage | P2 | pending | edit.rtl-cjk-editing |
| REC-E06 | [ ] Rotated text editing and move/resize | P2 | pending | edit.rotated-text-editing |
| REC-E09 | [ ] General vector object editing | P2 | pending | edit.vector-edit edit.object-properties |
| REC-F07 | [ ] Collect responses and export a clean dataset | P2 | pending | form.distribute-track |
| REC-F10 | [ ] Safe script inspection debugging and trust controls | P2 | pending | form.js-host form.js-debugger form.edit-all-javascripts |
| REC-H06 | [ ] Protected recipient-specific links with expiry/revocation | P2 | pending | misc.share-links |
| REC-H07 | [ ] Recipient roles and domain-limited guest review | P2 | pending | '- |
| REC-H08 | [ ] Web viewer for invited guests without editor installation | P2 | pending | '- |
| REC-H09 | [ ] Threaded reviews mentions and assigned tasks | P2 | pending | comment.shared-review comment.mentions-live-review |
| REC-H10 | [ ] Review dashboard deadlines reminders and digest | P2 | pending | comment.review-tracker |
| REC-H12 | [ ] Tamper-evident activity and approval event log | P2 | pending | '- |
| REC-H14 | [ ] Offline comments queue and reconnect reconciliation | P2 | pending | '- |
| REC-H15 | [ ] Self-hosted and approved-region sharing providers | P2 | pending | misc.share-links misc.document-cloud-storage |
| REC-H16 | [ ] Cloud-drive check-in/out and change notifications | P2 | pending | misc.connected-storage |
| REC-H17 | [ ] Preview-scoped project links and QR handoff | P2 | pending | '- |
| REC-H18 | [ ] Upload/request-files portal for suppliers | P2 | pending | '- |
| REC-H19 | [ ] Watermarked recipient view and download policy | P2 | pending | '- |
| REC-H20 | [ ] Retention legal holds deletion and evidence export | P2 | pending | '- |
| REC-I02 | [ ] OneDrive SharePoint and Microsoft Teams handoff | P2 | pending | misc.connected-storage |
| REC-I03 | [ ] Google Drive and Workspace handoff | P2 | pending | misc.connected-storage |
| REC-I08 | [ ] Power Automate and no-code connector | P2 | pending | '- |
| REC-I09 | [ ] DMS adapters for Aconex Procore or existing project system | P2 | pending | '- |
| REC-I10 | [ ] Accounting ERP and invoice-processing export | P2 | pending | '- |
| REC-N03 | [ ] Batch slip-sheeting with markup preservation | P2 | pending | '- |
| REC-N04 | [ ] Batch hyperlinks from sheet/detail references | P2 | pending | '- |
| REC-N06 | [ ] Count angle volume and formula takeoffs | P2 | pending | '- |
| REC-N07 | [ ] Quantity schedules and spreadsheet handoff | P2 | pending | '- |
| REC-N10 | [ ] As-built and O&M handover binder builder | P2 | pending | '- |
| REC-N11 | [ ] Site photo and location evidence linked to drawings | P2 | pending | '- |
| REC-N12 | [ ] Bilingual contract/tender comparison report | P2 | pending | '- |
| REC-O06 | [ ] True editable-text OCR output | P2 | pending | ocr.editable-text-output |
| REC-O09 | [ ] Browser OCR with explicit resource and privacy limits | P2 | pending | ocr.ocr-web |
| REC-O10 | [ ] Mobile camera scan to desktop handoff | P2 | pending | '- |
| REC-O11 | [ ] Scan compression and searchable-PDF size presets | P2 | pending | ocr.adaptive-compression ocr.ocr-downsample |
| REC-P09 | [ ] Portfolio viewing and safe extraction | P2 | pending | organize.portfolio-view organize.portfolio-extract |
| REC-P10 | [ ] Duplicate blank and missing-page detection | P2 | pending | '- |
| REC-Q01 | [ ] General preflight profiles and fixups | P2 | pending | optimize.preflight optimize.preflight-fix optimize.preflight-profiles |
| REC-Q03 | [ ] PDF/X and production print preflight | P2 | pending | optimize.pdfx-validate optimize.pdfx-convert |
| REC-Q04 | [ ] Output preview overprint and separation tools | P2 | pending | print.output-preview print.separations print.overprint-preview |
| REC-Q07 | [ ] Advanced colour conversion ink limits and print marks | P2 | pending | print.convert-colors print.ink-manager print.printer-marks print.total-area-coverage |
| REC-Q08 | [ ] Certificate-encrypted and legacy document compatibility | P2 | pending | core.open-public-key core.usage-rights-preserve |
| REC-Q09 | [ ] PDF/A-3 associated XML e-invoice packaging | P2 | pending | optimize.einvoice-pdfa3 |
| REC-Q10 | [ ] Deterministic PDF serialization and web linearization | P2 | pending | core.deterministic-save core.linearization |
| REC-S06 | [ ] Recipient-certificate encryption | P2 | pending | protect.certificate-encryption protect.recipient-permissions |
| REC-S10 | [ ] Purview sensitivity-label integration | P2 | pending | protect.mip-labels |
| REC-T05 | [ ] Optional SSO MFA and tenant provisioning for collaboration | P2 | pending | '- |
| REC-T08 | [ ] Consent-only product health metrics | P2 | pending | '- |
| REC-V03 | [ ] Persistent local full-text search index | P2 | pending | view.search-index |
| REC-W04 | [ ] Offline-capable installable web app | P2 | pending | '- |
| REC-W10 | [ ] Page/comment deep links tied to document revision | P2 | pending | '- |
| REC-X03 | [ ] PDF slides to PowerPoint | P2 | pending | create.export-pptx |
| REC-X05 | [ ] HTML and URL-to-PDF with isolated fetching | P2 | pending | create.from-html create.from-web-page |
| REC-X06 | [ ] Markdown and accessible text export/import | P2 | pending | create.from-markdown create.export-accessible-text |
| REC-X10 | [ ] Structured JSON and region-based extraction | P2 | pending | '- |
| REC-Z04 | [ ] AI-assisted PII marking with human apply step | P2 | pending | protect.redact-ai-suggestions |
| GAP-comment.record-audio | [ ] Sound comments | P3 | pending | REC-C09 |
| GAP-core.usage-rights-preserve | [ ] Preserve usage-rights (UR3) signatures from Reader-extended files | P3 | pending | REC-Q08 |
| GAP-optimize.einvoice-pdfa3 | [ ] Embed e-invoice XML in PDF/A-3 | P3 | pending | REC-Q09 |
| GAP-protect.mip-labels | [ ] Open Microsoft Purview-protected PDFs and apply labels | P3 | pending | REC-S10 |

## Future (88 records)

pending: 65, scope_review: 23

| ID | TODO | Priority | Status | Related tasks |
|---|---|---|---|---|
| SCOPE-comment.enable-reader-commenting | [ ] Enable commenting and measuring in Reader (usage rights) | N/A | scope_review |  |
| SCOPE-comment.mentions-live-review | [ ] '@mentions, presence and live co-review | N/A | scope_review | REC-H09; REC-H13 |
| SCOPE-comment.review-tracker | [ ] Review tracker | N/A | scope_review | REC-H10 |
| SCOPE-comment.shared-review | [ ] Send for comments / shared review | N/A | scope_review | REC-H09 |
| SCOPE-create.cloud-office-conversion | [ ] Cloud conversion from the macOS Office add-in | N/A | scope_review |  |
| SCOPE-edit.adobe-express-edits | [ ] Adobe Express image edits (remove background, generative fill) | N/A | scope_review |  |
| SCOPE-form.autofill-profile | [ ] Autofill collection synced to an Adobe account | N/A | scope_review |  |
| SCOPE-form.distribute-track | [ ] Distribute and track forms, collect responses | N/A | scope_review | REC-F07 |
| SCOPE-form.web-forms | [ ] Acrobat Sign web forms | N/A | scope_review |  |
| SCOPE-misc.adobe-account | [ ] Adobe account sign-in | N/A | scope_review |  |
| SCOPE-misc.adobe-ai-assistant | [ ] Adobe AI Assistant | N/A | scope_review |  |
| SCOPE-misc.adobe-express | [ ] Adobe Express integration and generated images | N/A | scope_review |  |
| SCOPE-misc.connected-storage | [ ] Connected third-party storage accounts in-app | N/A | scope_review | REC-H16; REC-I02; REC-I03 |
| SCOPE-misc.cross-device-sync | [ ] Sync signatures, stamps and preferences across devices | N/A | scope_review |  |
| SCOPE-misc.document-cloud-storage | [ ] Adobe Document Cloud storage | N/A | scope_review | REC-H15 |
| SCOPE-misc.flash-swf | [ ] Flash/SWF content | N/A | scope_review |  |
| SCOPE-misc.pdf-spaces | [ ] PDF Spaces | N/A | scope_review |  |
| SCOPE-misc.share-links | [ ] Share links and invitations | N/A | scope_review | REC-H06; REC-H15 |
| SCOPE-ocr.adobe-scan-sync | [ ] Adobe Scan mobile capture sync | N/A | scope_review |  |
| SCOPE-protect.rights-management | [ ] Server-based rights management (AEM/LiveCycle) | N/A | scope_review |  |
| SCOPE-sign.aatl | [ ] Adobe Approved Trust List | N/A | scope_review |  |
| SCOPE-sign.cloud-signatures | [ ] Cloud signature (CSC) digital IDs via Adobe | N/A | scope_review | REC-D09 |
| SCOPE-sign.request-esignatures | [ ] Request e-signatures (Acrobat Sign) | N/A | scope_review | REC-D07 |
| REC-M07 | [ ] Secure receive-from and send-to OS share sheets | P1 | pending | '- |
| GAP-a11y.alt-text-suggestions | [ ] Suggest alternate text via an AI provider | P2 | pending | REC-Z05 |
| GAP-comment.geospatial | [ ] Geospatial location and measuring | P2 | pending | REC-Z10 |
| GAP-misc.ai-ask | [ ] Ask questions with citations to pages | P2 | pending | REC-Z01 |
| GAP-misc.ai-provider | [ ] AI provider interface (local model or own endpoint), off by default | P2 | pending | REC-Z01 |
| GAP-misc.ai-summary | [ ] Summarize a document | P2 | pending | REC-Z02 |
| GAP-sign.eutl | [ ] EU Trusted Lists | P2 | pending | REC-D09 |
| REC-M02 | [ ] Offline project packs and explicit sync state | P2 | pending | '- |
| REC-M03 | [ ] Camera scanning batch crop and reorder | P2 | pending | ocr.camera-perspective |
| REC-M04 | [ ] Touch-first forms and on-site signatures | P2 | pending | '- |
| REC-M05 | [ ] Stylus eraser palm rejection and quick palettes | P2 | pending | '- |
| REC-M06 | [ ] Photo-backed punch inspections and checklists | P2 | pending | '- |
| REC-M08 | [ ] Biometric app lock and managed wipe | P2 | pending | '- |
| REC-M09 | [ ] QR-based drawing and equipment lookup | P2 | pending | '- |
| GAP-a11y.class-map | [ ] Class map editor | P3 | pending |  |
| GAP-create.distiller | [ ] PostScript/EPS to PDF (Distiller) | P3 | pending |  |
| GAP-create.job-options | [ ] PDF settings presets (job options) | P3 | pending |  |
| GAP-create.pdf-printer | [ ] Print to PDF from other applications | P3 | pending |  |
| GAP-create.translate | [ ] Translate documents | P3 | pending | REC-Z02 |
| GAP-edit.action-goto-embedded | [ ] Go to a target in an embedded file (GoToE) | P3 | pending |  |
| GAP-edit.action-media | [ ] Play sound / media / rich-media actions | P3 | pending |  |
| GAP-edit.action-thread | [ ] Read-an-article action | P3 | pending |  |
| GAP-form.folder-level-js | [ ] Folder-level JavaScript | P3 | pending |  |
| GAP-misc.3d-view | [ ] View 3D (U3D/PRC) content | P3 | pending | REC-Z09 |
| GAP-misc.ai-charts | [ ] Charts from document data | P3 | pending |  |
| GAP-misc.ai-conversational-edit | [ ] Natural-language editing through registry commands | P3 | pending | REC-Z06 |
| GAP-misc.ai-podcast | [ ] Generate an audio overview | P3 | pending |  |
| GAP-misc.ai-presentation | [ ] Generate a presentation from documents | P3 | pending |  |
| GAP-misc.contract-analysis | [ ] Contract key terms and multi-contract compare | P3 | pending | REC-Z03 |
| GAP-misc.legacy-multimedia | [ ] Legacy multimedia (screen annotations, renditions) | P3 | pending |  |
| GAP-misc.prefs-catalog | [ ] Preferences: catalog / index | P3 | pending |  |
| GAP-misc.rich-media-playback | [ ] Play video and sound via the platform | P3 | pending |  |
| GAP-misc.rich-media-poster | [ ] Show posters for rich media | P3 | pending |  |
| GAP-misc.rich-media-preserve | [ ] Preserve 3D, video and sound content | P3 | pending |  |
| GAP-misc.send-email | [ ] Send the file by email (system mail client) | P3 | pending |  |
| GAP-ocr.remove-halo | [ ] Remove halos | P3 | pending |  |
| GAP-optimize.lines-to-curves | [ ] Convert smooth lines to curves | P3 | pending |  |
| GAP-optimize.merge-image-fragments | [ ] Detect and merge image fragments | P3 | pending |  |
| GAP-optimize.pdfx5-reference-xobjects | [ ] Show PDF/X-5 reference XObjects | P3 | pending |  |
| GAP-optimize.preflight-audit-trail | [ ] Embed preflight audit trail | P3 | pending |  |
| GAP-organize.base-url | [ ] Base URL for relative links | P3 | pending |  |
| GAP-organize.page-templates | [ ] Page templates | P3 | pending |  |
| GAP-organize.portfolio-create | [ ] Create PDF Portfolios | P3 | pending |  |
| GAP-print.jdf | [ ] JDF job definitions (read-only) | P3 | pending |  |
| GAP-print.trap-presets | [ ] Trap presets (stored as data) | P3 | pending |  |
| GAP-protect.recipient-ldap | [ ] Find recipients in a directory (LDAP) | P3 | pending |  |
| GAP-protect.remove-hidden-on-close | [ ] Remove hidden information when closing or sending | P3 | pending |  |
| GAP-protect.security-envelope | [ ] Security envelopes | P3 | pending |  |
| GAP-view.articles-panel | [ ] Articles panel and article reading | P3 | pending |  |
| GAP-view.model-tree | [ ] 3D model tree panel | P3 | pending | REC-Z09 |
| REC-D09 | [ ] Trusted service provider and remote signing adapters | P3 | pending | sign.cloud-signatures sign.eutl |
| REC-H13 | [ ] Real-time annotation synchronization and presence | P3 | pending | comment.mentions-live-review |
| REC-M01 | [ ] Dedicated iOS/Android capture and review app | P3 | pending | '- |
| REC-M10 | [ ] Voice notes with optional local transcription | P3 | pending | '- |
| REC-Z01 | [ ] Local/BYO-provider document Q&A with page citations | P3 | pending | misc.ai-provider misc.ai-ask |
| REC-Z02 | [ ] Bilingual summaries and side-by-side translation | P3 | pending | misc.ai-summary create.translate |
| REC-Z03 | [ ] Contract risk and obligation extraction | P3 | pending | misc.contract-analysis |
| REC-Z05 | [ ] AI-assisted alt text and structure suggestions | P3 | pending | a11y.alt-text-suggestions |
| REC-Z06 | [ ] Natural-language actions through approved command plans | P3 | pending | misc.ai-conversational-edit |
| REC-Z07 | [ ] Semantic project search across authorized revisions | P3 | pending | '- |
| REC-Z08 | [ ] Drawing symbol search and assisted quantity takeoff | P3 | pending | '- |
| REC-Z09 | [ ] 3D PDF and BIM-linked inspection | P3 | pending | misc.3d-view view.model-tree |
| REC-Z10 | [ ] Geospatial drawings and field positioning | P3 | pending | comment.geospatial |
| REC-Z11 | [ ] Handwriting search and recognition | P3 | pending | '- |
| REC-Z12 | [ ] Immutable project closeout and evidence timeline | P3 | pending | '- |

## Component verification records

### B001: Reject paths as atomic-save staging filenames

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Reject traversal, absolute paths, empty/dot names and NUL before filesystem access or suffix consumption; reject Windows streams; preserve valid native Unicode/hidden filenames and both existing naming schemes.
- Audit: First audit reproduced a real escape from the selected directory. Added validation before truncation; second audit checked native filename semantics, Windows streams, call sites, side effects and existing collision/attempt behavior.
- Verification Limit: Local full workspace gates were unavailable because the craft-libs checkout was missing. Remote Linux/Windows/wasm, ARM64 and FreeBSD checks passed on the verification commit. macOS hardware acceptance, full durability and all of REC-R01 are not claimed.
- Ci: passed
- Local evidence: 7 staging tests passed on Linux using rustc 1.92.0-nightly.
- Local evidence: Production and test modules passed standalone clippy-driver with warnings denied.
- Local evidence: Two new regression tests failed before the fix and passed afterward.
- Local evidence: cargo fmt --all -- --check passed.
- Local evidence: CUPS fixture: existing default assertion and new regression reproduced failures with one harness thread before the fix; all 4 actual CUPS tests passed with both 1 and 2 threads after the fix.
- Local evidence: Actual spool module and extracted CUPS tests passed standalone clippy-driver with warnings denied.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40192697828
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40192697831
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40192697825
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40192697845
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40192697834

### B002: Audit and harden flush-before-replace in automation saves

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Scope and reproduce durability/failure behavior before changing it; preserve the desktop's supported-volume flush fallback, clean up failed staging writes and keep original bytes intact.
- Audit: Baseline automation wrote, closed and renamed without a flush. Three injected ordering/failure regressions failed before the fix. Shared the desktop flush helper verbatim, including macOS ENOTSUP/ENOTTY fallback; second audit checked write/flush/close/rename order, error propagation before mark_saved, cleanup after descriptor close, root handling and dependency relocation.
- Verification Limit: Full local workspace gates were dependency-blocked; remote Linux/Windows/wasm, native ARM64 and FreeBSD gates passed, including new save/retry coverage. Native macOS/SMB/FUSE execution remains unverified; the fallback was moved verbatim and macOS-only coverage was added but not executed here. Parent-directory synchronization and hostile parent replacement remain outside scope.
- Ci: passed
- Notes: The user confirmed continuation on 2026-10-25; implementation began only after all B001 gates passed. B002 is implemented, audited and verified at 8191bbd. All replacement CI gates passed. Await explicit approval before the next component.
- Local evidence: 18 standalone automation/staging tests passed; three new regressions failed before the fix.
- Local evidence: 16 standalone desktop/staging tests passed.
- Local evidence: Standalone Clippy passed for the actual automation/desktop save functions and staging module.
- Local evidence: cargo fmt --all -- --check passed.
- Local evidence: Desktop flush helper moved verbatim; dependency version unchanged.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40195716756
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40195716771
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40195716697
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40195716751
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40195716706

### B003: Create Unix atomic-save staging files with private permissions

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Request 0600 in the Unix exclusive-creation syscall before writing bytes, honor stricter umasks, preserve collision refusal and bytes/modes of planted files, and cover desktop plus headless Save/Save As/confined output. Do not change Windows ACL or unconfined stream behavior.
- Audit: Reproduced default Unix staging mode 0666 under an isolated umask 000 before the first byte. The fix supplies mode 0600 to OpenOptions::create_new, never a post-create chmod. Re-audit checked both naming schemes, restrictive umask behavior, collision metadata, child-filter receipts, unchanged Windows code path and both caller pipelines.
- Verification Limit: Local full workspace gates were dependency-blocked; remote Linux/Windows/wasm, native ARM64 and FreeBSD checks passed, including Unix permission and tool-level integration coverage. Native macOS execution and network filesystems remain unverified. This is a Unix creation-mode policy, not Windows ACL hardening, ownership/ACL preservation, parent-directory race protection or full power-loss durability.
- Ci: passed
- Notes: The user approved the next basic component on 2026-10-25 after B002 verification. New/replaced Unix atomic outputs intentionally retain private mode rather than copying broader sharing permissions; full ownership/ACL preservation is outside scope. All five replacement workflows passed on fc5fd97. Stop and await explicit approval before another component.
- Local evidence: Permission regression failed before the fix: observed 0666, expected 0600.
- Local evidence: 11 staging tests passed, including isolated child umasks 000/022/077/777 and unchanged collision permissions.
- Local evidence: 20 actual automation/staging standalone tests passed.
- Local evidence: 19 actual desktop/staging standalone tests passed.
- Local evidence: Standalone Clippy with warnings denied and non-test staging compilation passed.
- Local evidence: cargo fmt --all -- --check passed.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40200837989
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40200837981
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40200837971
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40200837965
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40200838063

### B004: Deterministic atomic-save write and rename fault coverage

- Status: **complete**; parent(s): REC-R01.
- Acceptance: For both automation and desktop atomic writers, inject partial-write disk-full and post-flush rename failures for existing/new targets; preserve original bytes or absence of new targets, return errors and remove staging. Verify recoverable Interrupted/short writes finish before replacement, and on Windows verify the staging writer is closed before rename. Keep fault controls private and production I/O unchanged.
- Audit: Prior code already propagated write/rename errors, but portable post-staging failure tests were missing (the existing real rename-denial fixtures were Windows-only). Introduced private I/O seams with unchanged standard operations in production. Re-audit checked exact call order, descriptor scope, unchanged cleanup/error mapping, failure cases for existing/new outputs, retry semantics and no public runtime fault switches.
- Verification Limit: Local workspace gates were blocked by the missing craft-libs checkout; remote Linux/Windows/wasm, native ARM64 and FreeBSD gates passed, including Windows exclusive-handle reopening checks. The new tests inject operation errors against real staging files; they do not physically fill a volume, kill a process, simulate power loss or validate native macOS/network filesystems. Cleanup remains best-effort when unlink itself fails; broader recovery/metadata/parent-directory guarantees remain outside this component.
- Ci: passed
- Notes: The user approved continuation on 2026-10-26. This closes a deterministic coverage gap, not a newly reproduced production data-loss defect. It does not complete the broad REC-R01 release gate. All five replacement workflows passed at db25dbb. Stop and await explicit approval before another component.
- Local evidence: 23 actual automation/staging standalone tests passed.
- Local evidence: 22 actual desktop/staging standalone tests passed.
- Local evidence: Four scratch mutation checks rejected swallowed write/rename errors in both save pipelines.
- Local evidence: Standalone Clippy with warnings denied passed for both actual save pipelines.
- Local evidence: cargo fmt --all -- --check passed.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40274000272
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40274000472
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40274000276
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40274000403
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40274000545

### B005: Process-termination boundaries and safe retry for atomic saves

- Status: **complete**; parent(s): REC-R01.
- Acceptance: For automation and desktop save pipelines, kill real children during a partial staging write, after flush/before rename and immediately after rename, for existing/new targets. Before replacement keep old bytes/target absence; after replacement keep complete new bytes. Verify private Unix orphans survive pre-rename kills and retry skips them without clobbering. Reap children on assertion/timeout paths. No production fault switch or behavior change.
- Audit: Added only cfg(test) cases using B004 private seams. Real children signal phase-specific fresh-file barriers before the parent calls Child::kill and wait; Unix status must report SIGKILL. Re-audit added a reaping guard that does not kill an already reaped child, rejects stale/empty-filter barriers and checks retry collision behavior plus Unix orphan mode. Production code and safety deadlines are unchanged.
- Verification Limit: Local workspace gates were dependency-blocked by the missing craft-libs checkout; remote Linux/Windows/wasm, native ARM64 and FreeBSD gates passed, including the new child-termination unit tests in both callers. The tests use local-filesystem synthetic byte payloads, not PDF semantic/conformance checks or whole-application recovery. Pre-rename kills deliberately leave private staging orphans; no automatic orphan cleanup/recovery is added. Physical volume-full, power loss, parent-directory durability, native macOS/network filesystems and restart/session recovery remain outside this component.
- Ci: passed
- Notes: The user approved the next basic component on 2026-10-26 after B004 verification. This is controlled process-kill coverage with synthetic bytes, not full application crash recovery, physical disk-full or power-loss certification. All five replacement workflows passed at 4b3a22b. Stop and await explicit approval before another component.
- Local evidence: 24 actual automation/staging standalone tests passed.
- Local evidence: 23 actual desktop/staging standalone tests passed.
- Local evidence: Each new test exercises three phases for existing/new targets: 12 real child terminations across both callers.
- Local evidence: Four scratch mutation checks rejected in-place target corruption and unsafe orphan reuse in both pipelines.
- Local evidence: Standalone Clippy with warnings denied passed for both actual save pipelines.
- Local evidence: cargo fmt --all -- --check passed.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40279282248
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40279282322
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40279282211
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40279282228
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40279282235

### B006: Retry recovery autosaves until complete persistence succeeds

- Status: **ci_pending**; parent(s): REC-R01.
- Acceptance: Do not consume an unsaved revision when selecting an autosave snapshot. Acknowledge only after PDF and metadata writes both succeed; retry failures without another edit, independently per document. Reject acknowledgements for newer/closed/clean documents and keep autosaved documents dirty. Cover real filesystem refusals, prior/new entries, encrypted/plain PDFs and fresh-app recovery after retry.
- Audit: Baseline selection advanced snapshot_generation before RecoveryStore::write; a failed write was not retried without another edit. A regression against the extracted actual selection method failed before the fix. Selection is now read-only; a private revision token and current-byte identity guard the engine acknowledgement, called only after the whole store write returns Ok. Re-audit checked all callers, dirty/undo/save/close behavior, per-document state, metadata-write ordering and unchanged encryption/storage paths.
- Verification Limit: Full local workspace Clippy/test/wasm gates are blocked by the unavailable craft-libs checkout; fresh platform CI must run the real engine and filesystem/app integration tests. The new integration tests drop/reopen app instances, not real processes. Directory/rename refusals are controlled filesystem errors, not physical volume-full or power loss. RecoveryStore still writes PDF then JSON separately; pair-transactionality, concurrent sessions, hostile store contents, automatic orphan cleanup, native macOS/network filesystems and web recovery are outside this component.
- Ci: pending
- Notes: The user approved continuation after B005. B006 fixes a reproduced snapshot-selection bug; it does not implement orphan cleanup, a transactional PDF/metadata pair, web recovery or power-loss guarantees. Stop for explicit approval after verification.
- Local evidence: Five standalone tests of actual extracted engine selection/acknowledgement and UI autosave-loop methods passed, using lightweight document/store stubs (not full-engine or filesystem integration).
- Local evidence: Baseline actual selector failed the unacknowledged-retry regression.
- Local evidence: Three scratch mutation checks rejected selection-time consumption, stale acknowledgement and acknowledgement before store success.
- Local evidence: Standalone Clippy with warnings denied passed.
- Local evidence: cargo fmt --all -- --check passed.
