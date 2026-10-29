# Live implementation TODO

> Generated from the full research inventory and `progress.toml`; do not edit statuses here.

[Workflow and completion rules](README.md) · [Filterable CSV](TODO.csv) · [Original research](../research/pdf-product-strategy-2026-10-25/README.md)

Order: **basic → advanced → future**. Implement → audit → fix → re-audit → test → push → verify CI → continue the user-authorized basic batch; ask before changing stage.

Scope: **222 recommendations + 384 existing gaps + 23 scope reviews**, plus bounded components. These overlap; they are not 629 distinct new functions. Existing shipped features are not marked missing. Complete a gap only when its own criteria are demonstrated, not just because a related component landed.

P0 is a safety priority, not a universal scheduling stage. Gates for a new collaboration service stay attached to that advanced service; native mobile work is future. Never launch such a surface before its safety gates pass.

**Active component:** B024 · **Active stage:** basic

## Basic (355 records)

ci_pending: 1, complete: 23, in_progress: 3, pending: 328

| ID | TODO | Priority | Status | Related tasks |
|---|---|---|---|---|
| B001 | [x] Reject paths as atomic-save staging filenames | P0 | complete | REC-R01 |
| B002 | [x] Audit and harden flush-before-replace in automation saves | P0 | complete | REC-R01 |
| B003 | [x] Create Unix atomic-save staging files with private permissions | P0 | complete | REC-R01 |
| B004 | [x] Deterministic atomic-save write and rename fault coverage | P0 | complete | REC-R01 |
| B005 | [x] Process-termination boundaries and safe retry for atomic saves | P0 | complete | REC-R01 |
| B006 | [x] Retry recovery autosaves until complete persistence succeeds | P0 | complete | REC-R01 |
| B007 | [x] Validate recovery keys and bind metadata to its entry filename | P0 | complete | REC-R01 |
| B008 | [x] Keep failed recovery offers retryable and serialize password prompts | P0 | complete | REC-R01 |
| B009 | [x] Bound recovery metadata reads | P0 | complete | REC-R01 |
| B010 | [x] Refuse to overwrite a file another program changed on disk | P0 | complete | REC-R05 |
| B011 | [x] Create the recovery folder private to its user | P0 | complete | REC-S08 |
| B012 | [x] Notice a file renamed, moved or deleted while it is open | P0 | complete | REC-R05 |
| B013 | [x] Keep the recovery folder within a disk budget | P0 | complete | REC-R01; REC-S08 |
| B014 | [x] Make the settings folder and its record of recent documents private | P0 | complete | REC-S08 |
| B015 | [x] Stable exit codes and machine-readable failures for the command line | P0 | complete | REC-B02 |
| B016 | [x] Account for every network path, and fail the build when the list drifts | P0 | complete | REC-S09; REC-S03 |
| B017 | [x] Fetch the OCR models from a source that can be pinned, and pin what it serves | P0 | complete | REC-T03; REC-T06 |
| B018 | [x] Keep same-named form fields apart when documents are combined | P0 | complete | REC-P04 |
| B019 | [x] Take the page's own copy of itself when it is redacted | P0 | complete | REC-S01 |
| B020 | [x] Say the same thing about what works everywhere a user reads it | P0 | complete | REC-R12 |
| B021 | [x] Copy what is on screen at the resolution it is shown at | P0 | complete | REC-R02 |
| B022 | [x] One malformed page or a failed export does not spoil the rest | P0 | complete | REC-R06 |
| B023 | [x] See what the other version changed before replacing it | P0 | complete | REC-R05 |
| B024 | [ ] Stop any background job, and see what every one of them is doing | P0 | ci_pending | REC-R04 |
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
| REC-R04 | [ ] Cancellable background-job centre | P0 | in_progress | '- |
| REC-R05 | [ ] External-file change and save-conflict detection | P0 | in_progress | '- |
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

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Do not consume an unsaved revision when selecting an autosave snapshot. Acknowledge only after PDF and metadata writes both succeed; retry failures without another edit, independently per document. Reject acknowledgements for newer/closed/clean documents and keep autosaved documents dirty. Cover real filesystem refusals, prior/new entries, encrypted/plain PDFs and fresh-app recovery after retry.
- Audit: Baseline selection advanced snapshot_generation before RecoveryStore::write; a failed write was not retried without another edit. A regression against the extracted actual selection method failed before the fix. Selection is now read-only; a private revision token and current-byte identity guard the engine acknowledgement, called only after the whole store write returns Ok. Re-audit checked all callers, dirty/undo/save/close behavior, per-document state, metadata-write ordering and unchanged encryption/storage paths.
- Verification Limit: Full local workspace Clippy/test/wasm gates remained dependency-blocked by the unavailable craft-libs checkout; remote Linux/Windows/wasm, native ARM64 and FreeBSD gates passed, including the actual engine and filesystem/app integration regressions. The new integration tests drop/reopen app instances, not real processes. Directory/rename refusals are controlled filesystem errors, not physical volume-full or power loss. RecoveryStore still writes PDF then JSON separately; pair-transactionality, concurrent sessions, hostile store contents, automatic orphan cleanup, native macOS/network filesystems and web recovery are outside this component.
- Ci: passed
- Notes: The user approved continuation after B005. B006 fixes a reproduced snapshot-selection bug; it does not implement orphan cleanup, a transactional PDF/metadata pair, web recovery or power-loss guarantees. All five replacement workflows passed at 7f77ac6. Stop and await explicit approval before another component.
- Local evidence: Five standalone tests of actual extracted engine selection/acknowledgement and UI autosave-loop methods passed, using lightweight document/store stubs (not full-engine or filesystem integration).
- Local evidence: Baseline actual selector failed the unacknowledged-retry regression.
- Local evidence: Three scratch mutation checks rejected selection-time consumption, stale acknowledgement and acknowledgement before store success.
- Local evidence: Standalone Clippy with warnings denied passed.
- Local evidence: cargo fmt --all -- --check passed.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40289115131
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40289115132
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40289115141
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40289115171
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40289115231

### B007: Validate recovery keys and bind metadata to its entry filename

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Reject path/stream/device/control/oversized recovery keys before read/write/remove filesystem access, including before write creates the store; keep generated and ordinary Unicode keys compatible. Require each discovered JSON key to match its filename before probing/returning its PDF. Invalid removal requests are no-ops. Preserve outside files and normal plain/encrypted recovery/discard.
- Audit: All key-derived read/write/remove paths share a private validator; list validates the key and corresponding JSON file before its PDF probe, allowing native canonical-equivalent filenames without accepting a different regular file. The policy bounds keys to 128 UTF-8 bytes before case-fold allocation, rejects portable separators/streams/controls/device aliases and preserves normal Unicode/hidden/generated keys. Re-audit found and fixed allocation-before-length-rejection; added canonical filename comparison to preserve normalizing-filesystem spellings; confirmed no change to original document path metadata, atomic-save or B006 acknowledgement ordering. Invalid JSON is cleaned only at the discovered store path.
- Verification Limit: Full local workspace gates remained dependency-blocked by the unavailable craft-libs checkout; remote Linux/Windows/wasm, native ARM64 and FreeBSD gates passed, including the actual recovery-store and full-app plain/encrypted recovery/discard regressions. The local harness uses actual store/write/staging code and real Serde/JSON at Cargo.lock versions, but omits the GUI/engine and stubs only portable-mode directory selection (not used by these tests). This is lexical key validation, not symlink/hardlink/hostile-parent or concurrent-replacement protection, metadata/original-path authenticity, resource-bounded JSON/PDF loading, transactional pair recovery, physical volume-full, power-loss or web recovery. Native macOS/network filesystems remain unverified.
- Ci: passed
- Notes: The user approved continuation after B006. Baseline actual-store tests reproduced outside reads, replacement/deletion via traversal keys and alias metadata impersonating another entry. This is lexical key safety and filename binding, not a complete recovery-store sandbox or metadata authenticity guarantee. All five replacement workflows passed at 6f19614. Stop and await explicit approval before another component.
- Local evidence: Five new regression tests failed before the fix against actual RecoveryStore, desktop atomic writer and staging source with real pinned Serde/JSON.
- Local evidence: 17 actual-source store/staging tests passed serially and in parallel, including six new store tests.
- Local evidence: Standalone Cargo Clippy with all targets and warnings denied passed.
- Local evidence: Four scratch mutations rejected unchecked keys, unchecked removal, filename-alias acceptance and directory creation before validation.
- Local evidence: cargo fmt --all -- --check passed.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40295728212
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40295728273
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40295728229
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40295728210
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40295728476

### B008: Keep failed recovery offers retryable and serialize password prompts

- Status: **complete**; parent(s): REC-R01.
- Acceptance: A read or parse failure leaves the recovery entry on offer and retryable after the store is repaired; only a successful open (or the password prompt that owns its bytes) consumes an offer. Recovering several encrypted snapshots must not overwrite a pending password prompt, and an unrelated document's prompt must not be replaced. Duplicate keys never open duplicate tabs.
- Audit: Focused harness of the actual recover() method: five of six tests failed before the fix (both read and open failures consumed offers, duplicate keys opened twice, an unrelated prompt was replaced, the second encrypted snapshot overwrote the first prompt). After the fix only successful opens or their pending prompt consume an offer; iteration stops while a prompt is pending; cancelled/failed metadata returns to the offer without duplicates; the dialog is re-shown for remaining offers. submit_password returns the association for failed or cancelled recovery and keeps it for wrong-password retries. Re-audit checked dialog sequencing, notification paths and that no view/undo/save behaviour changed.
- Verification Limit: Full local workspace gates remain blocked by the missing craft-libs checkout; the new full-app regressions are compile-verified patterns but execute in remote CI only. The loop harness uses the actual recover() source with stubbed document opening, so it is not GUI/engine integration evidence. Serializing prompts trades parallelism for correctness: recovering several encrypted snapshots now requires answering one prompt at a time. Not claimed: PDF-load resource bounds, transactional pair recovery, whole-process restart, power-loss durability or metadata authenticity.
- Ci: passed
- Notes: The user authorized implementing all remaining basic components as a batch with a post-code audit. The audit of the recovery flow reproduced three defects: failed opens removed the offer while the files stayed in the store, a second encrypted recovery overwrote the pending prompt so the first snapshot lost its offer and association, and recovery could replace an unrelated password prompt. All five replacement workflows passed at 9ddec76.
- Local evidence: Six actual-source recovery-loop tests pass; five failed before the fix.
- Local evidence: Standalone clippy-driver with warnings denied and cargo fmt --check pass.
- Local evidence: Four new full-app regressions cover repair-and-retry after read or parse failure, duplicate keys, sequential encrypted recovery with wrong/cancelled passwords, and an unrelated pending prompt.
- Local evidence: Two scratch mutations (no per-key consumption, no prompt serialization) each failed two tests.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231270
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231324
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231256
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231352
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231317

### B009: Bound recovery metadata reads

- Status: **complete**; parent(s): REC-R01.
- Acceptance: Listing the recovery store reads each metadata file within a fixed byte budget and rejects metadata whose name or original path exceeds a fixed length, cleaning those entries inside the store like other invalid metadata. Complete bounded entries, including long native names, still list and recover unchanged.
- Audit: Audited the discovery path after B008. Metadata was read whole into memory and deserialized without any length bound, so a corrupt or hostile entry could allocate arbitrarily, and unbounded name/path strings flowed into the recovery dialog and save path. Reads now open the file, take at most 64 KiB + 1 bytes, reject anything larger, and require name and path within 4,096 bytes before listing; rejected entries are removed only at their discovered path inside the store, consistent with existing incomplete-entry cleanup. Re-audit confirmed the PDF probe still happens after key/filename binding, that the document-bytes read is unchanged and still engine-limited, and that the byte cap is defense-in-depth while the acceptance check is what tests can observe. CI then showed the new store test depends on directory iteration order: an invalid metadata file removed only its JSON, leaving the paired snapshot until a later listing happened to visit it. Invalid metadata now takes its paired snapshot with it, which makes the existing cleanup deterministic and keeps the test assertion unchanged.
- Verification Limit: Full local workspace gates remain blocked by the missing craft-libs checkout. The bound applies to recovery metadata only: document PDF bytes, engine loading and rendering remain limited by existing engine budgets, not this component. Not claimed: metadata authenticity, transactional pairs, symlink/hostile-parent protection, power-loss durability or recovery of over-budget entries (they are cleaned up like other invalid metadata). Native macOS/network-filesystem behaviour remains unverified.
- Ci: passed
- Notes: Batch follow-up to the B007 key/filename work: discovery still used an unbounded std::fs::read for JSON and accepted unbounded name/path strings. Metadata is now capped at 64 KiB per file with name/path at 4,096 bytes; document bytes remain engine-limited. All five replacement workflows passed at 9ddec76.
- Local evidence: A new actual-source store test failed before the fix and passes now (18 store/staging tests total, serial).
- Local evidence: The test rejects a padded 64 KiB+1 metadata file and 4,097-byte name/path entries, keeps a 4,096-byte name, and checks rejected pairs are cleaned inside the store.
- Local evidence: Removing the post-read size and string checks was rejected by that test.
- Local evidence: Removing the read byte cap alone is not detectable in tests and is recorded as defense-in-depth, not as covered behavior.
- Local evidence: Standalone Clippy with warnings denied and cargo fmt --check pass.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231270
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231324
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231256
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231352
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40312231317

### B010: Refuse to overwrite a file another program changed on disk

- Status: **complete**; parent(s): REC-R05.
- Acceptance: Saving in place when the file's bytes differ from what the app loaded or last wrote must not write; the app keeps the other program's version, explains the conflict and offers Save As, Overwrite or Cancel. Unchanged files, new files, Save As to another path and the first save of a document without a prior stamp keep working without a prompt.
- Audit: Traced every write: desktop save_doc_to and automation write their own bytes; this component covers the desktop in-place path, which is the one that replaces a file the user has open. Stamps are taken from the bytes on disk at open and re-read after each successful write, so the comparison is against what the app itself last saw rather than a timestamp. Identical bytes rewritten by another program are not a conflict (nothing is lost) and a vanished file is not one either (writing recreates it). The prompt is a modal like the save prompt, defaults to the non-destructive choice, treats Escape or an outside click as Cancel, and is compiled out on wasm where the path save does not exist. Re-audit after wiring: the overwrite agreement is a one-shot flag consumed by the next save, Cancel clears both the prompt and the flag, and the reminder stamp is refreshed inside the same success branch that marks the document saved.
- Verification Limit: Full local workspace gates remain blocked by the missing craft-libs checkout, so the new full-app regressions execute in remote CI only. This is a check between two filesystem operations, not a lock or a transaction: another program can still write between the check and the rename. Automation saves, physical volume-full behaviour, power loss and hostile parents/links are outside this component. REC-R05 stays open for rename/delete/move detection, external-change notification while idle, and merge or reload choices.
- Ci: passed
- Notes: Bounded part of REC-R05 (external-file change and save-conflict detection), which stays open. Detection is content-based (length plus a streamed 64-bit hash of the loaded bytes), recorded when a file is opened from disk and again after every successful save. The batch audit for this stage found no existing check at all: save_doc_to wrote straight over the path. CI is pending.
- Local evidence: Three standalone tests of the new stamp module pass, including same-length-different-bytes detection and a 300 KiB file that crosses read-buffer boundaries.
- Local evidence: The stamp module passes rustc with warnings denied as a standalone test binary.
- Local evidence: cargo fmt --all --check passes.
- Local evidence: Full-app regressions cover unchanged in-place saves, a blocked save with the other version preserved, Overwrite replacing it, and Cancel keeping it with the conflict raised again on the next save (executed in CI only).
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330474
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330329
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330634
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330320
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330428

### B011: Create the recovery folder private to its user

- Status: **complete**; parent(s): REC-S08.
- Acceptance: The recovery folder is created with owner-only access on Unix (mode 0700 in the creation call, so no looser window exists), a folder an earlier version created looser is tightened when the app opens it, the snapshots and metadata inside are readable only by their owner, and a path that is a link or a file is refused instead of followed or written through. Other platforms keep their per-user profile defaults.
- Audit: Traced every writer into the recovery folder: RecoveryStore::write creates it, ReadDir/list and read_metadata only read, and remove is limited to validated entry paths. The two files per entry already inherit the staging file's 0600 through the rename, so the exposure was the directory listing (0755 under the usual umask), not the bytes; a test now pins both modes so a later change to the staging creation cannot silently reopen it. The helper refuses symlinks, dangling links and plain files via symlink_metadata before any creation, so a tampered path never becomes a write target and never changes the mode of a directory the app does not own. The mode is changed through an open handle rather than the path, so a swap between the check and the change cannot redirect it. Tightening only removes access, and its failure is ignored on the write path so a filesystem without Unix modes cannot turn a permission bit into a failed autosave; listing still works even when the folder cannot be tightened. Re-audit: the AlreadyExists branch covers a concurrent creator, non-Unix builds only verify that the folder exists, and the store documentation now states that a folder replaced by a link is still listed but not written through.
- Verification Limit: Full local workspace gates remain blocked by the missing craft-libs checkout, so the store regressions run in remote CI only. This is not a sandbox: replacing the folder or its parent between two filesystem operations is outside the component, and filesystems without Unix modes (FAT/exFAT, some network shares) keep their own behavior. Thumbnails, saved settings, printed spool output and local history are not covered, and REC-S08 stays open.
- Ci: passed
- Notes: Bounded part of REC-S08 (secure temporary files, thumbnails and local history), which stays open. The audit found that recovery snapshots were already written through the staging file, which is created 0600 (B003), but the folder itself was made with std::fs::create_dir_all and the process umask, so 0755 was normal and other local accounts could list the entry names. The new helper is used for the folder on every recovery write, and opening an existing store tightens it. Thumbnails, settings, printed output, the portable data folder and the automation side are untouched. CI is pending.
- Local evidence: Six standalone tests of the new helper pass: new folder 0700, existing 0755 folder tightened and kept usable, missing parents created private, symlink refused with its target left at 0755, dangling symlink refused without creating the target, file at the path refused.
- Local evidence: The helper compiles as a standalone test binary and cargo fmt --all --check passes.
- Local evidence: The two new store regressions (folder and snapshot modes, tightening on open, linked folder refused) execute in remote CI only.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330474
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330329
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330634
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330320
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40331330428

### B012: Notice a file renamed, moved or deleted while it is open

- Status: **complete**; parent(s): REC-R05.
- Acceptance: An open document whose file is renamed or moved inside its folder, deleted, or rewritten by another program says so while the user is working, once per new situation, without writing, moving or reopening anything; an untouched file raises nothing, and saving keeps working on the document afterwards.
- Audit: Traced what the app remembered about a file: only a content stamp, taken from the bytes it had just read or written, so after a rename there was nothing to recognise the file by and no way to tell a rename from a deletion. The record now also holds the file's identity (device and inode on Unix) and its modification time. The poll is ordered cheapest first: one metadata call; if the size and time still match and this is not the deep pass, nothing is read; otherwise the contents are compared, which is the same comparison the save uses, so a replacement holding identical bytes is not reported and one holding different bytes is a conflict whether or not the name is the same. Identity is consulted only when the path holds no regular file, and only inside the document's own folder, so a file moved elsewhere is reported as gone instead of chased. Nothing is reported while a dialog or prompt is open, and state is compared with the last reported one so an idle window does not fill with toasts. Re-audit found that size plus modification time alone can hide a same-length rewrite inside one timestamp tick, so the deep pass compares contents every 60 seconds as well as on every explicit check, which then forced the distinction between the cheap and deep paths into the tests. The web build has no path-based save, so its tick is a no-op.
- Verification Limit: Full local workspace gates remain blocked: this checkout cannot resolve the craft-libs git dependency or the crates.io packages offline, so the full-app regressions run in remote CI only. Identity needs Unix metadata; elsewhere a rename is reported as a missing file. The poll is a check between two filesystem operations, not a lock, and it reads the document file (never the whole disk) at most every three seconds for metadata and every sixty seconds for contents. Merge or reload choices, automation saves and notification while the app is closed remain open in REC-R05.
- Ci: passed
- Notes: The second bounded part of REC-R05 (external-file change and save-conflict detection), which stays open for merge/reload choices and automation saves. B010 made a save refuse to overwrite a changed file, but only when the user saved: a file renamed or deleted in the meantime stayed invisible. This adds a 3-second poll of the open documents' files with a 60-second content confirmation, plus 16 translated notices.
- Local evidence: Nine standalone tests pass: an untouched file is fine and is not read again, a different length and a same-length rewrite are both changes, identical bytes rewritten are not a change, a deleted file is gone, a renamed file is found by identity, a replacement is reported by its contents, and a missing folder is not a conflict.
- Local evidence: The module and its dependency compile together as a standalone test binary with clippy warnings denied.
- Local evidence: cargo fmt --all --check passes.
- Local evidence: The two full-app regressions (rename then deletion reported once each while the document stays open and still saves; an idle document reporting an external write) execute in remote CI only.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467873226
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872809
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872868
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872891
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872916

### B013: Keep the recovery folder within a disk budget

- Status: **complete**; parent(s): REC-R01, REC-S08.
- Acceptance: The recovery folder stays within fixed entry and byte budgets by deleting the least recently saved snapshots and their metadata; the snapshot just written and the newest snapshot are never deleted, so the folder is never emptied and a snapshot larger than the whole budget is still kept; a folder within budget is untouched and the same folder always produces the same decision.
- Audit: Traced every way an entry is created and removed: write replaces a pair atomically, remove deletes a validated pair, clear empties the folder, and list cleans leftovers. Nothing bounded growth, and the folder's total size was never read. Pruning therefore reads only the metadata files (bounded to 64 KiB each by B009), pairs each with its snapshot's size, and asks the pure budget module what to delete, so the decision is testable without a filesystem. Metadata is deleted before its snapshot, so an interrupted prune leaves a snapshot without metadata, which the next listing cleans, rather than an entry promising bytes that are gone. Pruning runs only after the metadata write succeeded and its own failure is ignored: losing the new snapshot over housekeeping would be worse than an over-budget folder. Ordering is by recorded save time with ties broken by key, so listing order cannot change the outcome. Selection never deletes the entry just written or the newest entry, and never empties the folder: a single snapshot over budget is the user's only copy of unsaved work, and is removed by the next save or clean quit anyway. Re-audit: the zero-budget and overflowing-size cases exposed a rule that could delete every entry, so the newest is now always kept and a test pins it; a same-timestamp folder is pinned to a key order so the decision is deterministic.
- Verification Limit: Full local workspace gates remain blocked: this checkout cannot resolve the craft-libs git dependency or the crates.io packages offline, so the store regression runs in remote CI only. The budget is fixed rather than configurable in Preferences; only the recovery folder is pruned, not thumbnails, the portable data folder or local history elsewhere in REC-S08. Nothing here makes the PDF/metadata pair transactional or claims power-loss durability, so REC-R01 stays open.
- Ci: passed
- Notes: Bounded part of REC-R01 (save-integrity and crash recovery) and REC-S08 (local history hygiene), both of which stay open. The audit found that snapshots were only removed when the user saved, discarded or quit cleanly, so a crash plus weeks of not opening the app, or a long session with many large documents, left snapshots nothing would ever look at again, with no ceiling on the folder at all. Recovery now prunes after each successful metadata write.
- Local evidence: Seven standalone tests of the budget module pass: within budget, entry count, byte count, the just-written entry, a never-emptied folder, deterministic ordering and saturating sizes.
- Local evidence: The budget module compiles as a standalone test binary with warnings denied.
- Local evidence: cargo fmt --all --check passes.
- Local evidence: The store regression (two-entry/2,500-byte budget: the oldest is evicted with its metadata, the fourth snapshot larger than the budget is kept alone) executes in remote CI only.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467873226
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872809
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872868
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872891
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40467872916

### B014: Make the settings folder and its record of recent documents private

- Status: **complete**; parent(s): REC-S08.
- Acceptance: The settings folder is created owner-only on Unix, a settings file (app.ron) or log folder an earlier launch left readable is tightened to owner-only at the next launch without rewriting it, a link at the settings path is left alone rather than followed, a missing settings file is not an error, and the portable data folder is created owner-only from the start. Other platforms keep their per-user profile defaults, and a mode that cannot be set never stops the app.
- Audit: Started from every file the app keeps that names a document: app.ron (recent files and preferences), logs, recovery snapshots, new digital IDs. Recovery was handled in B011; the settings folder and the logs were not, and portable mode created its whole data folder with create_dir_all. The settings file is written by eframe at exit, so its mode cannot be chosen at creation: the mode is therefore tightened at the next launch, which is stated as the trade-off rather than hidden, and the folder itself is created owner-only now, so a first-time user's reload, logs and settings are never exposed. The mode is changed through an open handle after confirming by symlink_metadata that the path is a regular file, so a link planted there is neither followed nor changed and a swap between check and change cannot redirect it. Contents are never rewritten, and a failure is logged rather than fatal: an app that cannot set a mode must still open documents. Portable mode's own write probe still runs after the folder is created, so a read-only medium still turns portable mode off. Re-audit: the log subfolder is tightened only when it exists (the logger creates it later), the Windows and macOS builds compile the same module with a no-op, and nothing user-facing is added, so no translation changes.
- Verification Limit: Full local workspace gates remain blocked: this checkout cannot resolve the craft-libs git dependency or the crates.io packages offline, so the app-shell wiring and the portable assertion run in remote CI only. The settings file is tightened one launch after it is written, not at creation, because the framework owns that write. Non-Unix platforms keep profile defaults, and Unix modes do not protect against another process running as the same user. Thumbnails, printed output and the rest of REC-S08 stay open.
- Ci: passed
- Notes: Bounded part of REC-S08 (secure temporary files, thumbnails and local history), which stays open. The audit found that the settings folder was created by the framework and the portable folder by create_dir_all, both with the process umask: app.ron holds the recent-document list with full paths and the log files name documents, so 0755/0644 let another local account read what this person had open. B011 covered only the recovery folder.
- Local evidence: Six standalone tests of the settings module pass (fresh folder 0700, an existing 0644 settings file tightened to 0600 with its contents unchanged, a 0755 log folder tightened, a symlink left alone with its target's mode unchanged, a missing settings file not an error, and an unwritable folder not stopping protection).
- Local evidence: The module compiles as a standalone test binary together with the platform folder helper with warnings denied.
- Local evidence: cargo fmt --all --check passes.
- Local evidence: The portable-mode assertion (the data folder is 0700) runs with the existing portable tests in remote CI.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40479225078
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40479224982
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40479225262
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40479225181
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40479224958
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40472751178
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40472751157
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40472751141
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40472751124
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40472751134

### B015: Stable exit codes and machine-readable failures for the command line

- Status: **complete**; parent(s): REC-B02.
- Acceptance: Every failure the command line reports carries one documented kind with its own exit code (1 failed, 2 usage, 3 unsupported, 4 malformed, 5 refused, 6 partial) that is never renumbered, --error-format=json writes exactly one parseable JSON line to standard error with that kind, code and message, a message that contains quotes or control characters cannot break the JSON, the default prose line stays byte-identical, and a failure after earlier work (a script step, some pages of text) is reported as partial rather than failed.
- Audit: Started from every failure path in the command: the option check, required arguments, option values, document opens through the renderer and the engine, automation tool errors and the control-file reader. The kinds follow what the failure actually means, not where it happened: a bad invocation is usage even when it is a page number, a document that will not open is malformed, a tool name this build does not have is unsupported, and the control file the tool refuses to trust is refused. Two existing behaviours needed a decision rather than a rewrite: a page that could not be read after other pages were printed is partial only when text actually reached stdout (with nothing printed it stays failed, so the existing reading of that case does not change), and a script step that fails after earlier steps ran is partial because their work stands. The envelope is built by hand with the JSON escaping rules, one line, so no message can add a line or end the string; it also keeps the module dependency-free, which is why it can be tested on its own. Re-audit: the prose default is unchanged byte-for-byte (an existing test asserts the exact line), the broken-pipe special case still exits 0, the exit code comes from the failure rather than from the report succeeding, and --error-format is stripped before any command sees it so no command has to know about it.
- Verification Limit: The command-line crate cannot be built in this checkout: it depends on the engine and on the craft-libs git dependency, which need the network. The unit tests of the exit module were run standalone with a local rustc; everything that touches the real commands, the new end-to-end test and both updated expectations (a bad option value now exits 2) run in remote CI only. Not claimed: versioned automation schemas, a `refused` code for a document that forbids an operation (a wrong password still reports malformed, since the engine does not type the difference), or the same codes for the MCP server, which reports errors inside the protocol.
- Ci: passed
- Notes: Bounded part of REC-B02 (versioned schemas, stable exit codes and structured errors), which stays open: the automation JSON Schemas are not versioned here, and the tool schemas themselves are unchanged. Before this, every failure exited 1 and its reason was prose, so a script had to parse English to tell a mistyped option from a damaged file.
- Local evidence: Six standalone tests of the exit module pass with warnings denied (one documented code per kind, no shared codes, the envelope fields, a message with quotes/backslashes/newlines/control characters, only the JSON-required escapes, and the display form).
- Local evidence: cargo fmt --all --check passes.
- Local evidence: The command-line crate, its new unit tests and the new end-to-end test compile and run in remote CI (the crate depends on the engine and its vendored dependencies, which cannot be built offline here).
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40499132125
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40499132161
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40499184176
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40499132083
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40499132082

### B016: Account for every network path, and fail the build when the list drifts

- Status: **complete**; parent(s): REC-S09, REC-S03.
- Acceptance: Every place the editor or the command line can move bytes off the computer, or hand an address to another program, is listed with what limits it; the list is checked against the source, so a file that gains network code without an entry, a listed file that loses the code its entry names, and a crate that gains an HTTP client nobody explained all fail the check; and the update check is proven reachable only from the menu command, with no check at start.
- Audit: The audit started from the README's privacy claim and asked what the code actually does: it scanned every product source folder for socket, HTTP-client, browser-launch and URL-handoff code, and read each hit. Five places exist. One can leave the machine: Help ▸ Check for updates, one HTTPS GET to the GitHub API, reachable only from that command through a closure the app installs and nothing else mentions (the script enforces that: the function may appear exactly once, on the line that installs it). Two are loopback: the opt-in --control listener on 127.0.0.1 with a random port and a token from an owner-only file, and the CLI that connects to it after refusing a control file that is a link, another user's, or readable by others. One hands an address to the browser, never fetching it here, and only after the document's own address has passed engine::links::document_url and the user has confirmed the full address. One only parses a printer address as an IPv4 host. Two things looked like egress and are not: rustls_cng in the sign crate is the Windows certificate store, not a TLS client, and the sign crate's timestamp authority is a trait the caller supplies — the app supplies none. Re-audit: comment lines are stripped before scanning (a comment about the webbrowser crate is not a path off the machine) while trailing comments are kept, so the cost of an oddity is a false alarm and not a silent gap; test code is scanned too, since a test that opens a socket is still code in the repository; dev-dependencies and build-dependencies are excluded from the HTTP-client rule because neither ships, and that exclusion is itself tested.
- Verification Limit: Static, source-level evidence: it proves what the code in this repository can do, not what a future dependency does at run time, and it is not a runtime sandbox. The browser (or mail program) that receives a confirmed address is outside it, as are the operating system's own services. No dashboard or offline-only switch is claimed, since the audit found no automatic request to switch off — a future automatic network feature has to be added to the inventory, and the check will demand its restraint before it can pass. Thumbnails, spooler content and the rest of REC-S09 stay open.
- Ci: passed
- Notes: Bounded part of REC-S09 (privacy dashboard with an offline-only switch) and REC-S03 (explicit document network access controls), both of which stay open: there is no dashboard and no switch yet, because the audit found there is nothing to switch — the app makes no request of its own. What the README has claimed since the beginning ('no background network requests; the only activity is the optional update check') is now checked by CI rather than asserted.
- Local evidence: The checker passes on this checkout: five places accounted for, each naming real code.
- Local evidence: 19 new unit tests pass: the real checkout is covered, and a fixture tree proves that an unlisted networking file, a place that lost its code, a place that lost one of its tokens, a new HTTP client, a removed one, a startup update check, an update check not installed as the source, an unknown kind, an empty restraint, a duplicate entry and an unexplained dependency each fail the check, while a comment-only mention and a dev-dependency do not.
- Local evidence: The whole docs test suite CI runs passes locally: 29 tests.
- Local evidence: cargo fmt --all --check passes (no Rust changed).
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40518858485
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40518858529
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40518858497
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40518858783
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40518858661

### B017: Fetch the OCR models from a source that can be pinned, and pin what it serves

- Status: **complete**; parent(s): REC-T03, REC-T06.
- Acceptance: Release packaging gets the two OCR models again, from a source whose bytes a maintainer can check without trusting whatever a bucket serves that day: the SHA-256 pinned in ATTRIBUTION.toml comes from the publisher's own record of the file, a mismatch stops the download, the engine loads the format the files are in, an installation that still has the previous format keeps working, and every place that names the model files (packaging scripts, workflow greps, the MSI test) names the current one.
- Audit: Started from the failing step: `cargo xtask models` asks for `text-detection.rten` and the host answers 403. Upstream's announcement (issue #282) deprecates the `.rten` variants, says they were 'temporarily made unavailable at their original URLs' as a brownout, and points consumers at the ONNX models; the ocrs 0.13.1 release's own download script fetches `text-detection.onnx` and `text-recognition.onnx` from the same bucket. The decision (the user's) is to migrate. Two things then had to be true at once, and only one could be had from the bucket: the files must exist, and their SHA-256 must be known before they ship. The bucket's contents cannot be hashed from this workspace at all, so pinning a number guessed from it would have been a coin toss dressed up as supply-chain verification. The publisher's model card on Hugging Face carries the same models with git-lfs pointers, whose `oid sha256` is the SHA-256 of the file: that is a hash a maintainer can re-read at review time, and the two files were pinned from there (detection c15df092…, recognition 3774cdfe…). Consequences the audit found and fixed in the same change: `ocrs` and `rten` were built with the `.rten` format only, so the ONNX files would not have loaded (both features are now on, and the `.rten` support is kept so an installation that still has the old files keeps working, which is also what the new model lookup does: current format first, previous format second, and one file of a pair is not a pair); the FreeBSD and release workflows grepped the packages for `models/*.rten`, and `packaging/windows/test-msi.ps1` counted `.rten` rows in the MSI's file table, so both would have failed a correct package; and `Cargo.lock` gains `rten-onnx` (checksum read from the crates.io index, no other new package). The download check now reports the hash the server actually served, so the next time a host changes its bytes the log says what to pin instead of only that the pin is wrong, and `cargo xtask models --print-hashes` fetches to a temporary folder and prints size and SHA-256 without touching the project, which is the deliberate way to re-pin. Re-audit: the manifest is valid TOML with 64-hex hashes and https URLs, the model files keep their canonical local names while the card's own file names stay in the source line, the licence and its hash are unchanged, `ATTRIBUTION.md` was regenerated from the manifest rather than hand-written, and no test, deadline or budget was relaxed.
- Verification Limit: The workspace cannot be built here (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs git dependency needs the network), so nothing in this change was compiled, linked or run locally: the Rust edits, the Cargo.lock edit and the feature selection are verified by CI or not at all. The pinned hashes are the publisher's own LFS content hashes for the files on the model card; that the bucket serves the same bytes could not be checked from here, and if it does not the download stops with a mismatch instead of shipping different models. Model behaviour is not re-baselined: the ONNX files are the models the `.rten` files were converted from, but this change does not claim identical OCR output. Where packages are published, how a package is signed and the rest of REC-T03 and REC-T06 stay open.
- Ci: passed
- Notes: Bounded part of REC-T03 (portable and offline-install packages) and REC-T06 (dependency and provenance policy), both of which stay open: this fixes how the OCR models are fetched and pinned, not how they are chosen or where packages are published. It exists because upstream deprecated the `.rten` model files on 2026-10-27 (robertknight/ocrs#282) and the host that served them answers HTTP 403, which had stopped release packaging for every commit; the user chose to migrate to the ONNX models rather than drop OCR or host the old files.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: ATTRIBUTION.toml parses, and both model entries carry 64-hex sha256 and https URLs (checked with Python's tomllib against the same rules xtask validates).
- Local evidence: ATTRIBUTION.md's two model rows are regenerated from the manifest in the generator's own column order and escaping.
- Local evidence: The docs suite that CI runs passes locally (29 tests).
- Local evidence: No packaging script, workflow step or ATTRIBUTION entry still names a `.rten` file except the deliberate previous-format fallback.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40644285341
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40644285428
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40644285315
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40644285360

### B018: Keep same-named form fields apart when documents are combined

- Status: **complete**; parent(s): REC-P04.
- Acceptance: Combining documents never leaves two top-level fields with the same fully-qualified name: a field whose name the merged form already uses is renamed deterministically ("name", then "name (2)", "name (3)"…, skipping suffixes the form already holds), each field keeps its own value and its own widgets, a name that does not collide is left exactly as it was (so extract, split, page moves and a single-file combine do not rewrite anything), and a name in UTF-16 or PDFDocEncoding is compared as text and written back as a text string.
- Audit: Started by reading register_fields, the one place a combined document's /AcroForm /Fields is built: it appended every copied top-level field and never looked at the names already in the list. ISO 32000-2 §12.7.3.2 makes that a defect rather than a taste question — a field's fully-qualified name is its /T below its ancestors', and two fields with the same fully-qualified name are the same field — so combining two files that each have a field called `name` produced a document in which both widgets share one value, and whichever edit came last won. The fix renames rather than merges, because merging would silently drop one file's answer; the suffix scheme is the one this file already uses for duplicate attachment names (" (2)"), so the two conflicts behave alike. Three details came out of the audit. Only fields actually being added are renamed (the existing-ref guard runs first), so a page move inside one document, an extraction, a split and a single-file combine keep every name byte-identical — the fidelity the old behaviour had. Names are compared as decoded text, not raw bytes, so a UTF-16BE `/T` and a PDFDocEncoding spelling of the same name are recognised as the same name; the renamed value keeps the original's hex/literal form and is written with PdfString::text, so a non-ASCII name becomes a proper text string instead of ASCII bytes glued onto a UTF-16 name. And the search skips suffixes the form already uses (a file that already has `name (2)` keeps it), with a saturating counter so the loop cannot wrap; it terminates because each candidate is distinct and the taken list is finite. Unnamed fields (a widget that is its own field with no /T) are left alone, since readers do not merge those by name. Re-audit: the automation catalogue's doc_combine description now states the rule, because the renamed names are what a script reads back; the parity record moves from planned to partial with the four new tests cited; and nothing was changed in /V, /Kids or widget parenting, so values and appearances travel exactly as before.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so neither the new tests nor clippy have been run here: the Rust changes are verified by CI or not at all. Not claimed: choosing the new field name or merging two fields interactively, showing the rename in the interface, renaming inside a single document (there is no conflict there), or any change to field values, widgets or appearance.
- Ci: passed
- Notes: Bounded part of REC-P04 (merge form-name conflict resolution), which stays open: the conflict is resolved automatically and deterministically, but the user cannot choose the new name, merge the two fields or see the rename in the interface. The data-loss path was real: two fields with one fully-qualified name are one field to every reader.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: parity/acrobat-features.toml still parses, and each cited evidence entry names a test function that exists in the file it points at.
- Local evidence: The docs suite CI runs passes locally (29 tests).
- Local evidence: Four new unit tests in crates/organize: two files with the same field name, three files with a pre-existing "name (2)" in the mix, a single-document extract/split-duplicate/single-file combine that must not rename, and a non-ASCII name that must stay a text string.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40655257449
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40655257492
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40655257413
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40655257397
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40655257504

### B019: Take the page's own copy of itself when it is redacted

- Status: **complete**; parent(s): REC-S01.
- Acceptance: Every page a redaction touches loses the page-level data that still describes it as it was before — its thumbnail image (/Thumb) and the producing application's private data (/PieceInfo) — while a page no mark touched keeps both; the counts are reported; and the saved file, which an applied redaction forces to be a full rewrite, contains neither.
- Audit: Started from the module's own promise — content under the marks is removed 'for good' — and asked what a reader could still show after a redaction. The glyph, image, vector, annotation, field, tag and old-revision paths were already handled (each has a test, and applying a redaction calls require_full_save so a write_incremental becomes a full rewrite). Two page-level objects were not: /Thumb is an image of the page as it was, and /PieceInfo is the producing application's private data, which may contain its own copy of the page description; both are attached to the page dictionary, both are copied into the saved file by the full rewrite while the key is present, and neither can be reached by a mark. They are now removed from every page the operation touched, and counted in the report (thumbnails, private_data). Three decisions came out of the audit. The removal is per redacted page rather than document-wide, because a page nobody marked still has a correct thumbnail and its own private data — tested with a two-page document where only the first page is marked. It is unconditional: a thumbnail is exactly the picture the redaction exists to destroy, so leaving it to a separate optimisation would be a footgun (readers regenerate thumbnails; the file loses a cache). And it happens after the annotation pass on the same page object, so a page whose annotations were rewritten cannot keep a stale copy. Re-audit: the counts are per page and saturate nothing; both keys are optional in ISO 32000-2 (§12.5.3 /Thumb, §14.5 /PieceInfo) so a missing one is not an error and no shape is assumed; the key names are matched on the page dictionary only, so an /XObject named Thumb in resources is untouched; and the parity record for applying redactions now states it, with the two new tests cited.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so none of the Rust — production or test — was compiled or run here; it is verified by CI or not at all. Not claimed: XFA packets, document /Info and XMP metadata, or attachments, which a redaction does not remove and a user can clear with Sanitize document; a share-copy report that states what was covered; and any change to the appearance of the redacted page.
- Ci: passed
- Notes: Bounded part of REC-S01 (verified redaction and a sanitized share copy), which stays open: the content under the marks, the tags that described it and the next save were already handled, and this closes the copy of the page that sat beside the content, reachable by no mark. The audit asked what else in a document is a picture or a description of a page as it was, and found two things redaction ignored: /Thumb, an image every reader shows as the page's thumbnail (so a hover or a file manager revealed what the black box covered), and /PieceInfo, the producing application's private data, which can hold its own description of the page. Both are optional caches no reader needs, so a redacted page loses them and pages without marks keep theirs. Still open in REC-S01: XFA packets, document /Info and XMP (a redacted phrase can also live in the title), attachments a user did not ask to remove, and a share-copy report that lists what was and was not covered.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: parity/acrobat-features.toml parses, and each cited evidence entry names a test function that exists in the file it points at.
- Local evidence: The docs suite CI runs passes locally (29 tests).
- Local evidence: Three new tests: a redacted page loses both keys and the markers inside their objects while a page without marks keeps its own; the same through the automation tools (open, mark by find, apply, save), reading the saved bytes; and the per-page split of the counts.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40682881690
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40682881658
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40682881592
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40682881581
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40682881590

### B020: Say the same thing about what works everywhere a user reads it

- Status: **complete**; parent(s): REC-R12.
- Acceptance: The README's recommended-tool table and the interface's own tool catalogue name the same tools, in the same order, under the catalogue's labels; every command the README prints is in the command registry; every tool group the catalogue does not call Ready is listed with the catalogue's own count of working tools and its own milestone; a tool the interface calls planned is not a command that already runs nor a feature the parity inventory records as shipped; and CI fails when any of these drift apart.
- Audit: Started from the recommendation's acceptance line — "README UI tools and release notes agree on supported operations and partial implementations" — and from the ledger's warning that this was a proposal with no exact parity mapping, so the first job was to bound it. The audit read the four surfaces and compared them mechanically: `README.md` (Recommended Tools), `crates/engine/src/catalog.rs` (23 groups, 145 items, each with an `Availability`), `crates/engine/src/commands.rs` (181 registered commands) and `crates/ui-egui/src/home.rs` (the `RECOMMENDED` cards), with `parity/acrobat-features.toml` (833 records) as the fifth, independent opinion on what shipped. Four disagreements came out, listed in the notes. Five decisions came out of fixing them. (1) The README table is now machine-checkable: each row names its tool group as a code span and exactly one command, so the check can bind a row to a group and a command to the registry, and prose stays prose. (2) The check parses the real catalogue rather than a copy, and raises `LookupError` when the parse stops matching the file (a renamed struct, a missing file), because a checker that quietly reads zero groups passes forever. (3) The group-level `Planned` convention — "this group completes in Mx; items marked Ready work today" — was already implemented (`panels.rs` prints exactly that banner) and was documented rather than changed; what was wrong was two items inside a Ready group claiming to be planned. (4) The new README section "What is not finished yet" is a table of counts, so a user can see that Edit a PDF is 13 of 14 tools rather than read a claim of completeness; its numbers come from the catalogue and the check compares all three cells. (5) The catalogue-versus-registry rule lives in Rust next to the existing `every_ready_catalogue_item_is_a_registered_command`, and the catalogue-versus-parity rule lives in Python because that inventory is TOML. Re-audit: every rule has a fixture test that breaks exactly that rule in a throwaway copy of the real five files (21 drift cases plus the gate), so a rule that stops firing fails the suite; the rules were then run against the clean tree, which passes; and the boundaries were re-checked by hand — the README's remaining prose about capabilities (the Features list) still matches the catalogue's Ready items, and nothing here claims Acrobat parity, which `parity/acrobat-features.toml` scores separately.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so the three Rust changes — two catalogue items moving from Planned to Ready, the Home card array gaining `redact`, and the new converse test — were not compiled or run here; they are verified by CI or not at all. The Python side was run. Not claimed: generated or checked release notes; per-item readiness in the README; a machine-readable capability manifest; any change to how the interface reports a planned group (only the two stale items were corrected); or Acrobat parity for any Ready tool. One stale artefact remains and is recorded rather than fixed: `docs/images/linkco-home-dashboard.png` still shows the eight-card dashboard with hand-written blurbs and a v0.3.0 footer, so it predates both the current card rendering and the ninth card; rendering a new one needs a build and a display, neither of which this checkout has.
- Ci: passed
- Notes: Bounded part of REC-R12 (stable capability and limitation disclosure), which stays open. What this closes is the contradiction inside the repository: three places tell a user what the build can do — the README's Recommended Tools table, the tool catalogue the interface draws, and the Home cards — and nothing compared them. The audit found four disagreements: the README named `organize`, `edit`, `export`, `fill_sign` and `protect` in a column headed Primary Commands, but those are catalogue group ids, not commands; one row named `file.reduce_size`, a command that has never existed (the group is `compress`, the commands are `optimize.reduce` and `optimize.advanced`); redaction, five Ready tools and a flagship of the product, was on neither the README table nor the Home dashboard; and the two annotation items `Cloud` and `Stamp` were marked `Planned("M5")` while their commands were registered, reachable from the interface (the cloud tool in `crates/ui-egui/src/comments.rs`, the stamp palette) and recorded as shipped in `parity/acrobat-features.toml` with UI tests — working tools telling users they were coming in a milestone. All four are fixed, and the agreement is now checked rather than intended: `docs/implementation/check_disclosure.py` (run by the docs step in CI, 22 unit tests) reads the real files and fails on drift, and `crates/engine/src/commands.rs` holds the catalogue-versus-registry promise in both directions. Still open in REC-R12: release notes are per-release history and are not generated or checked; the README discloses readiness per tool group, not per tool; there is no machine-readable capability manifest for a support desk or a partner to read.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: python3 docs/implementation/check_disclosure.py --check passes on this checkout, and --summary lists all 23 tool groups.
- Local evidence: python3 -m unittest discover -s docs/implementation -p 'test_*.py' passes: 51 tests, 22 of them new (one gate on the real files, 21 drift cases).
- Local evidence: python3 docs/implementation/build_backlog.py --check passes after regeneration.
- Local evidence: Each README row's claim was checked by hand against the items of its group in catalog.rs.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40702890770
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40702890909
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40702890816
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40702890916
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40702890720

### B021: Copy what is on screen at the resolution it is shown at

- Status: **complete**; parent(s): REC-R02.
- Acceptance: A snapshot of a region is rendered at exactly the scale the page is drawn on screen (zoom × 96/72 × pixels per point), so the copied image holds one pixel per screen pixel and is never resampled; a selection too large for the renderer's tile limit (4096 pixels a side) is copied whole at a lower scale instead of being cropped; and the size the editor reports is the size of the image it copied.
- Audit: Started from a report that the pixels of a snapshot are blurry, and asked which surfaces raster a page and at what scale. Six did: the canvas (`canvas::render_scale`: zoom × PT × pixels per point), the print preview (`print_ui`, page-on-sheet scale × pane scale × pixels per point), the accessibility dialog's figure picture (380 × pixels per point over the figure's width), image export and the command line (dpi / 72), the automation `page_render` (dpi / 72) and the snapshot. Five of the six were device-exact; the snapshot was the exception, and its own comment said 'at the current zoom', which is what the code did rather than what the user sees. The arithmetic is a 4/3 shortfall at every zoom and every screen scale. Three decisions came out of the audit. (1) The scale is now the canvas's, written once as `snapshot_scale` and compared against `canvas::PT` in a test, so the two cannot drift apart again. (2) The renderer draws a tile of at most 4096 pixels a side and silently clamps anything larger, which would have cropped a big selection down to its top-left corner; the scale is now reduced to fit that tile, so the whole selection is copied. (3) The arbitrary 8-pixels-per-point ceiling is gone: it made a deeply zoomed-in snapshot softer than the screen, and the tile limit is the real constraint. Re-audit: the size the editor reports comes from the renderer's own output width and height, so a lowered scale is reported honestly; the start corner still floors to a whole pixel so the copy does not drift by a fraction; a degenerate zoom or rectangle returns a finite scale or an error rather than NaN (three unit tests); and the UI test now measures the copy against the area's size on screen instead of only checking that something was copied. The other five surfaces were re-read and left alone.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so neither the change nor its tests were compiled or run here; they are verified by CI or not at all. Not claimed: any change to the rendering of the page on screen (it was already device-exact), to colour management, or to the clipboard's own colour handling; a fidelity benchmark against a reference renderer; or a regenerated README screenshot — `docs/images/capture.sh` now captures at 2× without the resampling `--width` used to apply, but the committed images were made by an older build and replacing them needs a machine that can run the renderer.
- Ci: passed
- Notes: Bounded part of REC-R02 (independent rendering-fidelity benchmark), which stays open. The defect was in one line: `Edit ▸ Take a Snapshot` rendered the selection at `zoom × pixels per point`, while the canvas draws the page at `zoom × 96/72 × pixels per point` (`canvas::render_scale`). Every snapshot therefore held three quarters of the pixels the same area showed on screen — a copy that is soft the moment anything scales it back up to the size it came from, however sharp the page looked. Two smaller faults were in the same function: the scale was capped at 8 pixels per point, so a snapshot taken deeply zoomed in was coarser than the screen, and the area was only bounded by a 64-megapixel error rather than by the 4096-pixel tile the renderer actually draws, so a selection over that (about 34 × 34 inches at 120 dpi) was refused instead of copied. The snapshot now uses the canvas's own device scale, comes down only as far as one renderer tile requires, and copies a large selection whole at the lower resolution. Still open in REC-R02: a fidelity benchmark against a reference renderer, colour-management checks, and the tests that would show a rendering regression rather than a copying one.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: The three new unit tests in crates/ui-egui/src/zoom_snap.rs assert the scale equals canvas::render_scale for five zoom/screen-scale pairs, that a 2000 × 1500 point selection is fitted to one 4096-pixel tile, and that a degenerate zoom never yields a zero, NaN or infinite scale.
- Local evidence: crates/ui-egui/tests/viewer.rs::marquee_zoom_and_snapshot now asserts the copied image is as wide and as tall as the same area on screen (within a pixel and a half), which is the assertion that fails against the old formula.
- Local evidence: python3 docs/implementation/build_backlog.py --check passes, and python3 -m unittest discover -s docs/implementation -p 'test_*.py' passes (51 tests).
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40824850924
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40824850797
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40824850836
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40824850820
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40824850900

### B022: One malformed page or a failed export does not spoil the rest

- Status: **complete**; parent(s): REC-R06.
- Acceptance: A page the renderer refuses says why on the page, states how often it has been tried, and is retried on a short backoff a bounded number of times instead of being either given up on at the first error or asked for again for ever; an edit, a rotation or a zoom tries it again; only that page is reported, and neighbouring pages, thumbnails, print previews and other open documents keep drawing; and an export that fails reports why, leaves the document open and unchanged, and the next export to a real folder works.
- Audit: Started from the recommendation's acceptance line — one malformed page or a failed export leaves other tabs and the original document usable — and asked what the interface does with a failure today. `DocView.errors` was a `HashMap<usize, String>`: set once by `receive_result`, cleared only by `page_changed` (an edit) or `invalidate_content`, and used in three other places as a permanent 'do not ask again' flag. Four defects came out. (1) A failure with a passing cause was permanent: the page stayed blank, with its error box, until the document was reopened — the one recovery the reader had was to close the file. (2) The field's own comment said 'never re-requested', but the demand loop in `draw` pushed the page's raster every frame and only the queue's de-duplication (`if queue != last_queue`) prevented a render storm: a fragile invariant that nothing tested. (3) Thumbnails, print previews and organize-grid renders were blocked for that page for ever, so a page that failed at one zoom never even got a thumbnail. (4) Nothing told the reader the page had been tried more than once. Five decisions came out of the audit. A failure is a record rather than a string. The retries are bounded (two, after 1 s and 4 s) because a genuinely broken page must not cost a render per frame. Only the document view's own demand — a whole-page raster or a tile — can clear a failure when its tag changes; thumbnails and print previews are gated by the same schedule but never clear it, because two kinds of request with different tags would otherwise keep reviving each other for ever. Rotation clears the failure explicitly, because rotating the view does not change the render scale and so would not otherwise retry. And the page says how often it was tried, so a repeated failure is visible rather than mysterious. Re-audit: `waiting_since` is dropped when a failure is recorded, so the 'Still rendering' hint cannot contradict an error; the automatic attempts are counted in the record and stop at three (`attempts` includes the first failure), tested at the boundaries; the export side was checked too — the atomic writer already leaves the target intact — and a new automation test proves a failed export leaves the document open, unchanged and exportable again. Three more findings came out of the re-audit and are fixed with it. A retry that succeeds must clear the record, or the error card would keep covering a page that has drawn — `receive_result` now drops a failed page's record when a whole-page or tile render comes back, and a thumbnail that comes back says nothing about the page itself. A failure reported by a demand other than the one that failed first must not restart the schedule, or a page failing as a page and as a thumbnail in turn could keep reviving itself; only the recorded demand is counted. And the retry needs no new command: zoom and rotate are the existing ways to change the demand, the card names them, and the agent control channel already reports `page_errors` per document. The state machine is driven directly rather than through a synthetic file: hayro substitutes A4 for a degenerate page box (`base_dimensions`), so a refusal cannot be produced from a PDF the UI would accept, and the renderer has no other refusal a synthetic file can reach. A first attempt at a shell test with a zero `/MediaBox` page asserted an error that never came, which is how that was found; it was replaced by a DocView test with real `RenderRequest`s and `RenderedPage`s. Not changed: a renderer panic inside a worker is still handled by worker replacement, which was already tested.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so none of the Rust — production or test — was compiled or run here; it is verified by CI or not at all. The Python side was run. Not claimed: a job centre that lists and cancels work in progress; per-document rollback beyond the atomic save; a diagnostics bundle; recovery from a renderer panic beyond the existing worker replacement; a UI-level test of a renderer refusal, because hayro substitutes A4 for a degenerate page box and the renderer has no other refusal a synthetic file can reach, so the state machine is driven directly with real requests and results instead of through the shell; and any claim that a transient failure is always recovered — the retries are bounded on purpose, and the page then says so.
- Ci: passed
- Notes: Bounded part of REC-R06 (recoverable per-document operation failures), which stays open. What this closes is the page that was broken for the life of the tab: a renderer refusal was stored as a bare string and cleared only by an edit, so a failure with a passing cause — a retired worker, a brief allocation failure, a page that lost its race with a resize — left the page blank until the document was reopened, while the comment on the field said it was 'never re-requested' and the demand loop went on demanding it every frame. A failure is now a record: the reason, the demand that failed, how many attempts it has had and when the next is due. It is retried after 1 s and 4 s and then waits for the demand to change, so a page that is genuinely broken costs two extra renders rather than one per frame, and thumbnails and print previews — which used to be blocked for that page for ever — follow the same schedule. Changing the view (a zoom, a rotation), editing the page or reopening it tries again, and the page says how often it was tried. Still open in REC-R06: a job centre that lists work in progress and cancels it, per-document transaction rollback beyond the atomic save, a diagnostics bundle, and recovery from a renderer panic inside a worker beyond the existing worker replacement.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: crates/ui-egui/src/canvas.rs::tests::a_failed_page_is_retried_twice_and_then_waits_for_the_view_to_change checks the schedule at its boundaries (not due before 1 s, due at 1 s, then 4 s, then never).
- Local evidence: crates/ui-egui/src/canvas.rs::tests::a_refused_page_is_retried_twice_then_waits_for_a_new_demand drives a real DocView: sixty frames of demand inside the first second ask for nothing, the two retries are admitted and counted, a spent failure is not admitted even at t = 100 000 s, a new demand is a fresh attempt, and a render that comes back removes the card.
- Local evidence: crates/automation/tests/automation.rs::a_failed_export_leaves_the_document_open_and_usable exports into a path that is a file (the error is reported), checks the open documents are unchanged, then exports into a real folder and finds the PNG.
- Local evidence: python3 docs/implementation/build_backlog.py --check passes, and python3 -m unittest discover -s docs/implementation -p 'test_*.py' passes (51 tests).
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40843250000
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40843250166
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40843250147
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40843250027
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40843250254

### B023: See what the other version changed before replacing it

- Status: **complete**; parent(s): REC-R05.
- Acceptance: When the file of an open document changed on disk, the save-conflict prompt says what the other version holds — how many pages and when it was written, read from the file itself — and offers to open that version beside the document and show the differences with the existing compare, so the reader chooses between comparing, saving a copy and overwriting with the facts in front of them; comparing writes nothing and leaves the document unsaved, and its copy cannot be saved over the original; a version too large or too damaged to open is refused with a reason instead of freezing the window; and the automation control channel reports both the change and the held save.
- Audit: Started from the recommendation's acceptance line — concurrent modification prompts for compare, save-copy or explicit overwrite instead of silent loss — and asked which of the three the interface actually offers. Two were there. Overwrite and Save As… were bounded by B010, and Cancel writes nothing; the poll added by B012 notices a rewritten, renamed or deleted file while the document sits idle. Compare was not: the prompt named the situation and left the reader to guess its size. The audit also found that the parity table had no entry for any of the three components, that the control channel reported neither the disk state nor the held save, and that SaveConflict carried only the document and path, so the prompt had nothing to say about the other version. Six decisions came out of it. The facts are read from the file at the moment the conflict is raised, not per frame, because a prompt is raised once per save attempt. They are bounded by READ_LIMIT (32 MiB) with a capped read that stops one byte past the limit, so a huge file is never loaded to discover that it is huge; a version that cannot be parsed still gets a prompt, with no page count claimed for it. Compare opens the other version with no path, so it cannot become an accidental second way to overwrite the original. The comparison is the existing doc.compare path — the same one the Compare Files dialog runs — with the file on disk as the older version and this document as the newer one, so the panel, the shading and the report behave exactly as elsewhere. The held save is dropped, not re-attempted: comparing must not write, and the next save asks again. The disk state and the held save are exposed on the control channel, which is the automation surface this feature can have without inventing a headless duplicate of a modal. Re-audit: the facts are re-read on every raise, so a second attempt after another program wrote again describes the new bytes; a rotation of state — compare, then cancel, then overwrite — leaves no stale comparison pointing at a closed document (the copy is an ordinary tab the user can close); the password prompt a pathless encrypted copy would raise is cancelled and reported instead of stacking two dialogs; and the five new strings are in all sixteen catalogs with the catalogs' SHA-256s re-recorded in ATTRIBUTION.toml. A last finding is about the audit trail itself: `cargo xtask parity` is not one of the five workflows, so nothing has been validating the feature table. Run by hand, it rejected five evidence references B021 and B022 had written as `path.rs::tests::fn`, which the validator splits at the first `::`; those are corrected to the `path.rs::fn` form the rest of the file uses. It also reports three `print.*` features citing tests that no longer exist under those names (`crates/print/src/tests.rs::windows_printers_carry_one_default_and_keep_their_names`, `::the_print_script_renders_the_job_and_reaches_the_spooler`, `::the_windows_job_is_the_environment_of_the_print_script`). That drift predates this component and is left alone rather than repointed at a test whose claim would have to be guessed; the finding is recorded here instead.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so none of the Rust — production or test — was compiled or run here; it is verified by CI or not at all. The Python side was run. Not claimed: the CLI's conflict check, a web-build equivalent, identity-based rename detection off Unix, Acrobat parity for the prompt's wording or options (the table's entry for this feature marks it partial and says so), and any guarantee that a comparison of two very large documents is quick.
- Ci: passed
- Notes: Bounded part of REC-R05 (external-file change and save-conflict detection), which stays open for the command line. B010 already refused to overwrite a file another program changed, offering Save As, Cancel or Overwrite, and B012 already polled the open files and reported a rewritten, renamed or deleted one. What neither did was the third option the recommendation names: a comparison. The prompt said a file changed, not what changed, so the reader decided between losing their work and losing the other program's work without seeing either. The prompt now states the other version's page count and when it was written, and Compare opens that version beside the document and runs the app's own text and visual compare, marking the differences on this document's pages. The copy is opened without a path, so it cannot be saved over the file it came from; the deferred save is dropped rather than performed, so the document stays unsaved and saving again asks the same question. Both the facts and the compare read the file at most up to 32 MiB (conflict::READ_LIMIT) and through the parser's own catch-unwind, so a hostile or huge file is refused with a sentence rather than stalling the window. The facts also go to the agent control channel: each document reports its disk state (changed, renamed to a path, or gone) and a top-level save_conflict reports the held save with the other version's pages and bytes. Still open in REC-R05: the CLI's pdfcraft-cli edit --out writes with std::fs::write and checks nothing, so a command line edit can still replace a file another program wrote; the web build has no local path to watch; and the identity-based rename notice is Unix-only.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: The three new unit tests in crates/ui-egui/src/conflict.rs read a real two-page file (page count, size and modification time), show that an unparseable file is still described without claiming pages, and prove the capped read stops at the cap rather than loading the file.
- Local evidence: crates/ui-egui/src/editing.rs::the_other_version_opens_under_a_name_that_names_it keeps the copy's title recognisable and its extension intact.
- Local evidence: crates/ui-egui/tests/conflict.rs::the_other_version_can_be_compared_before_anything_is_written drives a real conflict through the shell: the prompt knows the other version has 3 pages, clicking Compare opens it, the compare runs with the disk version as the older document, nothing is written and the document stays unsaved.
- Local evidence: crates/ui-egui/tests/control.rs::a_file_that_changed_on_disk_is_reported_with_the_held_save reads the disk state and the held save back through the control channel.
- Local evidence: python3 docs/implementation/build_backlog.py --check passes, and python3 -m unittest discover -s docs/implementation -p 'test_*.py' passes (51 tests).
- Local evidence: All 247 recorded asset SHA-256s match their files, including the sixteen edited catalogs.
- Local evidence: Every evidence reference this component adds was checked against the function it names, and every command and tool it lists against the two registries the parity validator reads.
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40870833445
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40870833461
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40870833439
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40870833434
- CI run: https://github.com/b-lincko/linkco-pdf/actions/runs/40870833451

### B024: Stop any background job, and see what every one of them is doing

- Status: **ci_pending**; parent(s): REC-R04.
- Acceptance: While the app is working on something that leaves the interface thread, one card at the bottom of the window names each job, says what it is doing now and how far it has got, and offers a Cancel that stops that job and no other; a stop is safe — the item in hand finishes and its file is written whole or not at all, no further item starts, and the card says Cancelling… until the worker reports — and it is answered the same way by the optimizer, by text recognition on one document, by text recognition on a folder of files, by an action run over files, and by an export; the same jobs and the same stop are on the agent control channel, so a script sees what the person sees and stops the job the person would; and nothing that runs off the interface thread can freeze editing while it runs.
- Audit: Started from the recommendation's done-when — OCR, print, convert and batch jobs show stages, can cancel safely, and do not freeze editing — and asked, for each of the five kinds of work that leave the interface thread, three questions: does the reader see a stage, can they stop it, and can it hold up editing. The answers were uneven, and the audit named why. The optimizer had a card and a Cancel; text recognition had a cancel flag (`cancel_ocr`) that nothing in the interface ever called, so an OCR run could not be stopped at all; export and action runs reported into the toast, a single slot shared with every other notice, so a save confirmation hid the progress of the job that was running; the folder-based recognizer had no progress reporting beyond a lock the poll read. The four stop flags were four separate designs (an AtomicBool here, a `cancelled` field there, a closure returning false there), which is how the unevenness happened. Five decisions came out of that. One stop type (Cancel, an Arc around one AtomicBool) is shared by the card, the control channel and the worker, so a stop asked from a screen reader, from a script or from the card is one stop. One batch loop (run_batch) answers a stop the same way for every job: the item in hand finishes, no further item starts, and the outcome says whether anything was given up — so a stop that arrives after the last item is a finished batch, not a cancellation, and the summary does not claim otherwise. A stop is cooperative and answered between items, because an item is one page's recognition or one file's write and a job that cannot be interrupted must not pretend it can; nothing kills a thread. A job is registered before its worker starts and removed by the poll that was already waiting for it, so no watchdog is needed and a job that finishes in the same call (tests and automation run inline) never leaves a row behind. And the card is the report: the same list is drawn on screen and serialized on the control channel, so an agent cannot see a job a person does not, or stop a different one. Re-audit found four faults of this component's own, all fixed here: the export registered its job before the folder picker returned, so dismissing the picker would leave a ghost row with no worker — registration moved inside the branch that knows the folder; the Host trait grew the two new methods twice (a duplicate block from the same edit) — the copy is gone; ActionRun carried a stale Default derive after it gained a job id, which cannot default — the derive and the stray doc comment are gone; and the folder recognition's summary counted only files it wrote while the wording said "in {n} files", which is now the same number the loop reports. Two smaller corrections: a stopped export now says so in the sentence the toast already wraps in "to {dir}", and cancel_ocr stops the folder batch as well as the single document, since both are recognitions the reader started. Three more came from CI and from the new tests. The export test showed that a stop between pages reported the count from before the page in hand, so the card said 0 of 2 after one file had been written: the count is now stored before the stop is answered. The Czech catalog test refuses leading or trailing whitespace in a translation, and the stopped-after note begins with a space because it is a suffix, so that entry is not in the Czech catalog, which covers the menus only; the other fifteen carry it. And the first version of the card test clicked the second of two identical Cancel labels, which stopped nothing at all, so the test now drives the stop through the same call the row makes and clicks the one Cancel that is left, which is also the stronger claim. The audit also noted a drift it did not cause: docs/localization.md still describes the catalogs as holding 2,000-2,140 entries and names 2,047 for Ukrainian, 2,000 for Brazilian Portuguese and 2,137 for Arabic, while the sixteen files now hold 2,215-2,233 each, so those figures have been stale for some components and no test reads them; the counts are left as written rather than replaced by numbers this component would have to keep current by hand.
- Verification Limit: The workspace cannot be built in this checkout (rten 0.26 needs rustc 1.94, the local toolchain is 1.88, and the craft-libs dependency needs the network), so none of the Rust — production or test — was compiled or run here; it is verified by CI or not at all. The Python side was run. Not claimed: print jobs and the Word/HTML/RTF conversions, which still run on the interface thread and are therefore still open in REC-R04; a job queue, since each kind still runs one at a time; any promise that stopping is immediate — an item in hand (one page at 600 dpi, one large file) finishes first, by design; and Acrobat parity for the card's wording or placement, which the feature table's entry marks partial and says so.
- Notes: Bounded part of REC-R04 (the cancellable background-job centre), which stays open for print jobs and for the Word, HTML and RTF conversions. Before this component the five kinds of background work each reported themselves in their own way, and only one of them could be stopped: the optimizer had a private card with a Cancel button, text recognition had a cancel flag but no button anywhere that reached it, and the other three rewrote a toast every few hundred milliseconds, which any ordinary notice then replaced. There is now one job centre (crates/ui-egui/src/jobs.rs): a job id, a cooperative stop request shared with the worker, the list the card draws and the control channel reports, and one batch loop every multi-item job runs, so a stop means the same thing everywhere. The card lists up to three jobs with their own eased progress bars (further ones are counted), the Cancel belongs to its own row and is withdrawn once a stop is asked for, and the toast is lifted above the card instead of over it. The jobs are also on the control channel: ui.state reports each job's id, kind, name, stage, done and total, fraction and whether a stop was asked for, and ui.jobs with {"cancel": id} asks one to stop and answers with the list. Four new strings (the folder-recognition name, the file stage, the stopped-after note and the counted remainder above three jobs) are in all sixteen catalogs, and the page stage reuses the catalogs' existing Page {p} of {n}.
- Local evidence: cargo fmt --all -- --check passes.
- Local evidence: crates/ui-egui/src/jobs.rs::a_stop_request_is_shared_idempotent_and_never_goes_back proves one flag behind every clone, and that asking twice does nothing new.
- Local evidence: crates/ui-egui/src/jobs.rs::a_batch_stops_before_the_next_item_and_keeps_what_it_finished stops during the second of four items and shows that exactly two started, both finished, and the outcome says the batch was cancelled.
- Local evidence: crates/ui-egui/src/jobs.rs::a_batch_stopped_before_it_starts_touches_nothing, ::a_batch_records_each_failure_and_carries_on, ::a_stop_that_arrives_after_the_last_item_is_not_a_cancellation and ::progress_is_reported_before_each_item cover the other three edges of the stop contract and the progress the card is given.
- Local evidence: crates/ui-egui/src/jobs.rs::the_centre_lists_what_is_running_and_forgets_what_finished registers two jobs, stages one, refuses a NaN fraction, cancels one and proves the other untouched, then refuses to cancel a finished job.
- Local evidence: crates/ui-egui/src/optimize_ui.rs::the_progress_card_shows_the_stage_and_cancels drives the card through a real shell: the row reads "Optimize PDF…: Optimizing… image 4 of 10" at the stage's own 0.275 fraction, Cancel asks the worker's token and the row becomes "Optimize PDF…: Cancelling…" with no second Cancel, and the row goes when the worker reports.
- Local evidence: crates/ui-egui/tests/ui.rs::the_jobs_card_lists_each_job_and_cancels_only_its_own lists two concurrent jobs, clicks the second row's Cancel, and proves the first job's worker was not asked to stop, that the stopped row says Cancelling… and offers no second Cancel, and that finishing it removes only its own row.
- Local evidence: crates/ui-egui/tests/control.rs::jobs_are_listed_and_stopped_one_at_a_time reads the jobs back through ui.state and ui.jobs, stops one of two by id, and gets an error rather than silence for a job that is not running and for a cancel that is not an id.
- Local evidence: crates/ui-egui/src/export_ui.rs::a_stop_between_pages_keeps_the_page_in_hand_and_starts_no_other stops an export while its first page is being written and shows that the first page's PNG was written whole, the second page never started, and the message and the card both say 1 of 2.
- Local evidence: crates/ui-egui/src/export_ui.rs::a_stop_before_the_first_page_writes_nothing shows a stop that arrives before the first page writes no file at all.
- Local evidence: python3 docs/implementation/build_backlog.py --check passes, and python3 -m unittest discover -s docs/implementation -p 'test_*.py' passes.
- Local evidence: All 247 recorded asset SHA-256s match their files, including the sixteen edited catalogs.
