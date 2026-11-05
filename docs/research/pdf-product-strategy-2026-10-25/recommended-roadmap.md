# Detailed recommended roadmap

> Research recommendations, not delivered features. Priorities are this study's priorities, not the original checklist tiers.

Read [the strategy report](README.md) first for scope, definitions, caveats and sequencing. Source IDs refer to [sources.md](sources.md). A source supports the workflow rationale, not our current capability. Baseline status is the exact checklist mapping; unlinked enhancements are not asserted absent.

Desktop = Windows/macOS/Linux/FreeBSD application; Web = browser application; CLI = command line/MCP; Integrations = OS/storage connectors; Mobile and Service = proposed product surfaces.

P0 = non-negotiable safety/usability gate when that surface or workflow is offered; P1 = daily-work priority; P2 = advanced/segment need; P3 = future investment requiring validation.

Effort is relative only: S = localized change; M = one bounded subsystem; L = multiple subsystems; XL = new platform/service or major fidelity work. These are not estimates in days or delivery commitments.

**222 recommendations:** P0: 38, P1: 98, P2: 71, P3: 15.


## R Reliability

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| R01 | Save-integrity and crash-recovery release gate | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Corpus and fault injection | Disk-full interrupted-write and crash tests never silently replace the last valid file | SRC01 |
| R02 | Independent rendering-fidelity benchmark | P0 · Desktop Web CLI · L | view.render-pages: partial; view.render-transparency: partial; view.render-cid-fonts: partial | Licensed synthetic corpus | Text shapes transparency and drawings compared against named external oracles with reviewed differences | SRC01 SRC11 |
| R03 | Global memory and decoded-data budgets | P0 · Desktop Web CLI · XL | core.lazy-loading: planned; misc.performance-budgets: planned | Byte-source and cache accounting | Large hostile files stop within a documented budget without losing other open documents | SRC01 |
| R04 | Cancellable background-job centre | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Job scheduler | OCR print convert and batch jobs show stages can cancel safely and do not freeze editing | SRC09 |
| R05 | External-file change and save-conflict detection | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | File identity version tokens | Concurrent modification prompts for compare save-copy or explicit overwrite instead of silent loss | SRC04 SRC12 |
| R06 | Recoverable per-document operation failures | P0 · Desktop Web CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Transactions and diagnostics | One malformed page or failed export leaves other tabs and the original document usable | SRC01 |
| R07 | Open-save-reopen interoperability matrix | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Independent viewers and fixtures | Fonts forms comments layers encryption and signatures retain documented semantics across supported readers | SRC01 SRC06 |
| R08 | Performance budgets on ordinary office hardware | P0 · Desktop Web CLI · M | misc.performance-budgets: planned | Repeatable benchmark machines | Cold-open first-page scrolling search and job latency have published regression thresholds | SRC08 SRC11 |
| R09 | GPU fallback and mixed-DPI acceptance suite | P0 · Desktop · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Real Windows Linux macOS hardware | Old integrated GPUs remote desktop and mixed-DPI monitors remain usable or fall back clearly | SRC06 |
| R10 | Signed verified updates with rollback | P0 · Desktop Integrations · L | view.check-for-updates: partial | Release signing policy | Installer and update origin verified previous installer retained and updates remain explicit opt-in | SRC27 |
| R11 | Privacy-preserving diagnostic export | P1 · Desktop Web CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Redaction and user consent | A support bundle excludes document contents tokens full paths and personal data unless explicitly approved | SRC22 |
| R12 | Stable capability and limitation disclosure | P0 · Desktop Web CLI Integrations · S | proposal / enhancement; no exact parity mapping; verify scope before implementation | Audited feature inventory | README UI tools and release notes agree on supported operations and partial implementations | SRC01 |

## V Reading and daily UX

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| V01 | Search all open documents and local folders | P1 · Desktop CLI · L | view.advanced-search: partial; view.search-folder: planned | Background text extraction | Results identify file page snippet and progress and work on pages never viewed | SRC01 SRC06 |
| V02 | Search comments bookmarks and attachments metadata | P1 · Desktop Web CLI · M | view.find-in-comments-bookmarks: planned; comment.search-comments: partial | Unified query model | A single search can include each scope without confusing text hits with comment hits | SRC06 |
| V03 | Persistent local full-text search index | P2 · Desktop CLI · L | view.search-index: planned | Incremental index encryption | Changed files reindex and revoked or removed files disappear from results | SRC06 |
| V04 | Split view and tear-out document windows | P1 · Desktop · L | view.split-window: planned; view.multiple-windows: planned | Window state and shared document model | Two views stay consistent while their zoom and scroll remain independently controllable | SRC06 |
| V05 | Synchronized side-by-side comparison | P1 · Desktop Web · L | misc.compare-side-by-side: planned | Page pairing model | Old and new views scroll together with an obvious toggle and independently selectable text | SRC01 SRC11 |
| V06 | Pinned favorite and recent-tool shortcuts | P1 · Desktop Web · M | view.customize-quick-tools: planned; misc.custom-tools: planned | Command registry | Users pin common tools and reset layouts without losing document state | SRC06 |
| V07 | Task-first Home and command discovery | P1 · Desktop Web · M | view.home-view: shipped | Command metadata | Open merge compress sign OCR and share are discoverable without knowing internal feature names | SRC08 |
| V08 | Remember per-document reading state safely | P1 · Desktop Web · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Privacy-aware local storage | Page zoom layout panel state and bookmarks restore without persisting passwords | SRC16 |
| V09 | Accessible reflow and reading mode | P1 · Desktop Web · L | view.reflow: planned; a11y.reflow-tagged: planned | Logical text and tags | Tagged content reflows without altering the saved document and offers original-view fallback | SRC17 |
| V10 | High-contrast reading colours that survive annotation | P1 · Desktop Web · M | a11y.replace-page-colors: planned | Separate display and document state | Highlighting and saving never bake display-only colour changes into the PDF | SRC17 |
| V11 | RTL reading order across spreads and navigation | P1 · Desktop Web · L | edit.rtl-cjk-editing: planned | Bidi navigation policy | Arabic spreads bookmarks search and cursor movement follow a tested RTL/LTR policy | SRC17 |
| V12 | Actionable errors progress and inline help | P1 · Desktop Web CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Error taxonomy | Every blocked task explains cause safe alternatives and whether the original was changed | SRC08 |

## E Editing

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| E01 | Reliable existing-text editing with font diagnostics | P0 · Desktop Web CLI · XL | edit.text-edit: partial | Font mapping and layout preservation | Unsupported fonts are disclosed and edits do not move unrelated content | SRC01 SRC04 |
| E02 | Arabic and mixed-script editing with correct search/copy | P1 · Desktop Web CLI · XL | edit.rtl-cjk-editing: planned | Shaping Unicode font coverage | Bilingual text keeps logical order joining punctuation and searchable copy semantics | SRC17 |
| E03 | Complete CJK and complex-script editing coverage | P2 · Desktop Web CLI · XL | edit.rtl-cjk-editing: planned | Licensed fonts and shaping | Test corpus covers vertical text fallback fonts and regional variants with explicit limitations | SRC04 |
| E04 | Find and replace with preview | P1 · Desktop Web CLI · L | edit.find-replace: planned | Stable text ranges | User previews every replacement excludes protected content and undoes a batch atomically | SRC06 |
| E05 | Per-character rich formatting and paragraph controls | P1 · Desktop Web CLI · L | edit.line-paragraph-spacing: partial; edit.lists: planned; edit.text-super-subscript: planned | Text run model | Mixed font style lists spacing and super/subscript survive reopen | SRC01 |
| E06 | Rotated text editing and move/resize | P2 · Desktop Web CLI · L | edit.rotated-text-editing: planned | Transform-safe editing | Edits preserve text angle glyph placement and unrelated graphics | SRC06 |
| E07 | Crop replace and relink existing images | P1 · Desktop Web CLI · L | edit.image-crop: partial; edit.image-external-editor: planned | Shared resource isolation | Editing one occurrence does not alter another unless the user requests all occurrences | SRC04 |
| E08 | Arrange align distribute and group objects | P1 · Desktop Web CLI · L | edit.arrange-objects: planned; edit.align-distribute-objects: planned; edit.edit-object-tool: partial | Selection geometry | Object moves have guides keyboard alternatives and one predictable undo step | SRC06 |
| E09 | General vector object editing | P2 · Desktop Web CLI · XL | edit.vector-edit: planned; edit.object-properties: planned | Display-list/object references | Users safely recolour move scale and delete supported paths without modifying shared resources | SRC06 |
| E10 | Preserve document tags and reading order during edits | P1 · Desktop Web CLI · XL | edit.keep-tags-on-edit: planned | Structure-tree editing | Editing does not orphan tags or silently invalidate known accessibility structure | SRC17 SRC19 |
| E11 | Reusable headers footers watermarks and Bates profiles | P1 · Desktop Web CLI · M | edit.decoration-profiles: planned; edit.bates-numbering: partial; edit.header-footer-shrink: planned | Profile serialization | Preview shows conflicts duplicate numbering and content overlap before applying | SRC06 |
| E12 | Named snapshots and before/after edit inspection | P1 · Desktop Web CLI · L | core.history-panel: planned | History persistence policy | User can inspect named versions and restore a copy without overwriting signed originals | SRC04 |

## P Pages and assembly

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| P01 | Complete page-range selection and validation | P1 · Desktop Web CLI · S | organize.page-range-selection: partial | Shared range parser | Odd even reversed mixed-size and orientation selections have identical UI and CLI meanings | SRC08 |
| P02 | Drag or copy pages across open documents | P1 · Desktop Web · L | organize.drag-pages-between-documents: planned; organize.insert-by-drop: partial | Cross-document import | Thumbnails preview exact destination and preserve rotation labels and resources | SRC06 |
| P03 | Safe split naming and multi-file splitting | P1 · Desktop Web CLI · M | organize.split-naming: planned; organize.split-multiple-files: planned | Collision-safe writes | Outputs use bookmark templates and never overwrite a different input silently | SRC08 |
| P04 | Merge form-name conflict resolution | P0 · Desktop Web CLI · L | organize.field-name-conflicts: planned | Form identity model | A merge previews duplicate field names and offers independent fields or intentional shared values | SRC01 |
| P05 | Preserve tags bookmarks and internal links while merging | P1 · Desktop Web CLI · XL | organize.struct-tree-merge: planned | Structure and destination remapping | Merged documents retain resolvable links and explain unsupported structure | SRC17 |
| P06 | Resize pages and remove white margins | P1 · Desktop Web CLI · M | organize.change-page-size: planned; organize.remove-white-margins: planned | Page-box transforms | Page geometry content scale and print size are shown separately before applying | SRC08 |
| P07 | Attachment and associated-file management | P1 · Desktop Web CLI · L | organize.attachments-add: planned; organize.attachments-delete: planned; organize.associated-files: planned | Attachment safety | Users inspect add remove describe and export attachments with type warnings | SRC06 |
| P08 | Document-set manifest and cover/index generation | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Stable file and page IDs | A combined package has an index exact page counts source names and document hashes | SRC11 |
| P09 | Portfolio viewing and safe extraction | P2 · Desktop Web CLI · L | organize.portfolio-view: planned; organize.portfolio-extract: planned | Attachment sandbox | A portfolio opens as a file list without executing embedded content | SRC06 |
| P10 | Duplicate blank and missing-page detection | P2 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Image/text fingerprinting | Suggestions show confidence and require confirmation before deleting pages | SRC11 |

## C Comments and review

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| C01 | Interoperable annotation appearance regression suite | P0 · Desktop Web CLI · L | comment.appearance-generation: partial; comment.text-box: partial | Cross-viewer tests | Colours rotation transparency text wrapping and print appearances agree after reopening | SRC01 SRC14 |
| C02 | Reusable team markup and stamp libraries | P1 · Desktop Web CLI · L | comment.stamps-manage: planned; misc.custom-tools: planned | Portable profile format | Teams import export and version original licensed stamps and presets | SRC11 SRC14 |
| C03 | Advanced comment filtering and saved views | P1 · Desktop Web CLI · M | comment.search-comments: partial | Comment metadata | Filter by author status date page subject and assignment and export the visible set | SRC12 SRC15 |
| C04 | Comment status owner deadline and resolution | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Issue model | A comment can become an assigned issue with due date resolution evidence and history | SRC15 |
| C05 | Comment summaries with page context | P1 · Desktop Web CLI · M | comment.summarize: partial | Report generation | Summary includes source page labels thumbnails author date and status | SRC06 |
| C06 | Import selected comments from revised PDFs | P1 · Desktop Web CLI · L | comment.import-from-pdf: planned; comment.export-selected: planned | Stable annotation IDs | Round-trip preserves identifiers avoids duplicates and previews conflicting edits | SRC12 |
| C07 | Migrate comments to revised drawings | P1 · Desktop Web CLI · XL | comment.migrate: planned | Page matching and alignment | Anchors move only after review and uncertain mappings remain visibly unresolved | SRC11 |
| C08 | Comfortable pen input and palm rejection | P2 · Desktop Mobile · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Platform stylus wrappers | Pressure tilt latency eraser and palm-rejection behavior are tested on supported devices | SRC14 |
| C09 | Image/photo/audio comments with transcripts | P2 · Desktop Web Mobile · L | comment.image-comment: planned; comment.record-audio: planned | Attachment policy and optional TTS/STT | Media is size-bounded attributed and readable with a text alternative | SRC15 |
| C10 | Read-only final review export versus editable original | P1 · Desktop Web CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Copy/export policy | Review copies clearly state whether annotations are flattened and never replace the working original | SRC02 |

## F Forms

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| F01 | Predictable required-field validation and completion guide | P0 · Desktop Web · L | form.appearance-regeneration: partial; form.push-buttons: partial | Form validation model | Users can find missing fields keyboard through them and save a correct visible copy | SRC01 |
| F02 | Better automatic field detection and correction | P1 · Desktop Web CLI · L | form.auto-detect-fields: partial | OCR and geometry | Detected fields can be renamed resized typed aligned and reviewed before acceptance | SRC06 |
| F03 | Field layout tools and repeated grids | P1 · Desktop Web CLI · M | form.multiple-copies: planned; form.show-tab-numbers: planned | Form editor | Copy grids alignment and tab numbering prevent accidental field-name collisions | SRC06 |
| F04 | Encrypted local autofill profiles | P1 · Desktop Web · L | form.auto-complete: planned | OS keychain opt-in policy | Users select fields to fill can clear stored values and never sync them without consent | SRC01 SRC22 |
| F05 | Reusable bilingual business form templates | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Template management | Company-approved forms preserve Arabic/English labels date formats calculations and export values | SRC01 |
| F06 | Safe form submission with destination preview | P1 · Desktop Web CLI · L | form.submit-form: planned | Network policy and user approval | Data fields destination and attachment scope are displayed before any transmission | SRC22 SRC25 |
| F07 | Collect responses and export a clean dataset | P2 · Desktop Web CLI · XL | form.distribute-track: na | Optional hosted collection | A response has a stable template version consent timestamp validation and CSV/JSON export | SRC03 |
| F08 | Field locks and signature dependencies | P1 · Desktop Web CLI · L | form.props-signed: planned; sign.field-mdp: planned | Signature policy | Signing locks exactly the configured fields and later changes are explained | SRC23 |
| F09 | XFA capability detection flattening and fallback | P1 · Desktop Web CLI · XL | form.xfa-static: partial; form.xfa-dynamic: partial; form.xfa-flatten: planned | XFA layout and script support | Unsupported XFA is detected before editing and flattening has a visual preview | SRC01 |
| F10 | Safe script inspection debugging and trust controls | P2 · Desktop Web CLI · L | form.js-host: partial; form.js-debugger: planned; form.edit-all-javascripts: planned | Sandbox limits | Users see what scripts run can disable them and inspect failures without granting file/network access | SRC24 |

## O OCR and scanning

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| O01 | Arabic and bilingual OCR engine support | P1 · Desktop CLI · XL | ocr.ocr-languages: partial; ocr.ocr-engines: planned | Licensed model selection | Arabic and English CER/WER measured on a rights-cleared corpus including tables stamps and rotated scans | SRC07 SRC22 |
| O02 | Accented Latin and additional language packs | P1 · Desktop CLI · L | ocr.ocr-languages: partial | Language pack distribution | Language availability is truthful models verified by hash and mixed-language tests published | SRC07 |
| O03 | Rotation detection and deskew | P1 · Desktop Web CLI · L | ocr.rotation-detection: planned; ocr.deskew: planned | Image preprocessing | Preview shows correction and original scan remains available | SRC07 |
| O04 | Despeckle background shadow and perspective cleanup | P1 · Desktop Web CLI · L | ocr.despeckle: planned; ocr.background-removal: planned; ocr.edge-shadow-removal: planned; ocr.camera-perspective: planned | Image pipeline | Small characters stamps signatures and drawing lines are not erased without warning | SRC16 |
| O05 | Low-confidence OCR correction interface | P1 · Desktop Web CLI · L | ocr.correct-recognized-text: planned | OCR word confidence model | Users compare image and recognized text and edits preserve searchable geometry | SRC07 |
| O06 | True editable-text OCR output | P2 · Desktop Web CLI · XL | ocr.editable-text-output: planned | Layout reconstruction | Searchable-image and editable-output modes are clearly distinguished with fidelity warnings | SRC07 |
| O07 | Batch OCR queue with resume and language presets | P1 · Desktop CLI · L | misc.batch-processing: partial | Job centre checkpoints | A restart resumes finished files without duplicate text layers or overwritten originals | SRC09 |
| O08 | Native scanner acquisition and feeder support | P1 · Desktop · XL | ocr.scanner-acquire: planned; ocr.scan-presets: planned; ocr.scan-append: planned | WIA ICA SANE safe wrappers | Feeder duplex page size blank-page handling and cancellation pass real-device tests | SRC16 |
| O09 | Browser OCR with explicit resource and privacy limits | P2 · Web · XL | ocr.ocr-web: planned | Worker execution model delivery | UI reports local versus remote processing memory limits and unsupported devices before starting | SRC08 |
| O10 | Mobile camera scan to desktop handoff | P2 · Mobile Desktop · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Mobile app secure handoff | Multiple camera pages can be cropped ordered reviewed and transferred without public-link defaults | SRC16 |
| O11 | Scan compression and searchable-PDF size presets | P2 · Desktop Web CLI · L | ocr.adaptive-compression: planned; ocr.ocr-downsample: planned | Image quality evaluation | Size previews include OCR legibility warnings and never use unsafe lossy text substitution | SRC08 |
| O12 | OCR quality and duplicate-layer health report | P1 · Desktop Web CLI · M | ocr.searchable-image: partial | Text-layer inspection | Users see coverage language and suspect text alignment before relying on search or redaction | SRC07 |

## X Conversion and export

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| X01 | High-fidelity Word export with layout options | P1 · Desktop Web CLI · XL | create.export-docx: partial | Layout and table reconstruction | Reflow and page-preserving modes are explicit and multi-column/tabled examples are regression tested | SRC01 SRC04 |
| X02 | PDF tables to Excel or CSV | P1 · Desktop Web CLI · XL | create.export-xlsx: planned | Table detection numeric locale | Users preview rows headers merged cells decimals and units and can correct inferred boundaries | SRC01 SRC05 |
| X03 | PDF slides to PowerPoint | P2 · Desktop Web CLI · XL | create.export-pptx: planned | Slide object mapping | Users choose editable versus image-backed output and see limitations before export | SRC01 SRC08 |
| X04 | Office-to-PDF local conversion provider | P1 · Desktop CLI · XL | create.from-office: planned | Approved isolated sidecar | Macros are disabled fonts disclosed and output reviewed without silently uploading documents | SRC01 SRC08 |
| X05 | HTML and URL-to-PDF with isolated fetching | P2 · Desktop Web CLI · XL | create.from-html: planned; create.from-web-page: planned | Network sandbox CSS renderer | Network URLs redirects credentials and embedded resources follow explicit approved policy | SRC25 |
| X06 | Markdown and accessible text export/import | P2 · Desktop Web CLI · L | create.from-markdown: planned; create.export-accessible-text: planned | Logical reading order | Headings lists tables and links survive conversion with clear loss reports | SRC17 |
| X07 | Reusable per-format export presets | P1 · Desktop Web CLI · M | create.export-settings: planned | Preset schema | Resolution page ranges fonts layout and image handling are previewable reproducible and shareable | SRC06 |
| X08 | Smart size-target compression with comparison preview | P1 · Desktop Web CLI · L | optimize.optimizer: partial; optimize.optimizer-presets: planned | Size estimator quality metrics | A target size offers measured tradeoffs without falsely promising lossless compression | SRC08 |
| X09 | Automatic OCR-on-export for image-only pages | P1 · Desktop Web CLI · L | create.export-ocr: planned | OCR language support | Export detects pages lacking text and asks before OCR or remote processing | SRC07 |
| X10 | Structured JSON and region-based extraction | P2 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Schema and coordinate model | Extracted values retain source page region confidence and human-corrected values | SRC05 SRC28 |

## S Security and privacy

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| S01 | Verified redaction and sanitized share-copy gate | P0 · Desktop Web CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Independent extract/reopen checks | Removed content cannot be recovered through text images metadata attachments or incremental history within declared coverage | SRC01 |
| S02 | Restricted view for untrusted PDFs | P0 · Desktop Web CLI · XL | protect.protected-view: planned; protect.attachment-trust: planned | Process and capability isolation | Opening a file cannot silently launch executables transmit data or load remote resources | SRC25 SRC26 |
| S03 | Explicit document network access controls | P0 · Desktop Web CLI · L | protect.url-access-control: planned | Central network policy | Scripts links forms connectors and AI cannot bypass user or administrator restrictions | SRC22 SRC25 |
| S04 | Locale-aware PII search and partial redaction | P1 · Desktop Web CLI · L | protect.redact-pattern-locales: planned; protect.redact-partial-words: planned | Validated patterns | Qatar IDs phone numbers bank identifiers and bilingual names are suggestions requiring human confirmation | SRC22 |
| S05 | Batch folder redaction with reports | P1 · Desktop CLI · L | protect.redact-folder: planned; protect.redact-filename-suffix: planned | Job centre sanitized-copy writer | Every file records matches applied actions failures and a safe output name | SRC01 |
| S06 | Recipient-certificate encryption | P2 · Desktop Web CLI · XL | protect.certificate-encryption: planned; protect.recipient-permissions: planned | Public-key security handler | Each recipient is validated before encryption and unsupported readers receive clear guidance | SRC06 |
| S07 | Managed document security presets | P1 · Desktop Web CLI · L | protect.security-policies: planned | Admin profile policy | Passwords are not stored in plain text and policies explain what PDF permissions can and cannot enforce | SRC04 |
| S08 | Secure temporary files thumbnails and local history | P0 · Desktop Web CLI Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Threat model and storage lifecycle | Sensitive derivative files expire predictably and diagnostics exclude document content | SRC22 |
| S09 | Privacy dashboard with offline-only switch | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Network inventory | User can see and disable every document transmission path while local editing remains available | SRC22 SRC10 |
| S10 | Purview sensitivity-label integration | P2 · Desktop Web Integrations · XL | protect.mip-labels: planned | Approved Microsoft integration | Labels permissions and downstream sharing restrictions are preserved or unsupported behavior is blocked | SRC05 |

## D Signatures and trust

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| D01 | Plain-language signature status and signed-revision viewer | P0 · Desktop Web CLI · L | sign.signatures-panel: partial; sign.signature-properties: partial; sign.compare-signed-version: planned | Revision diff validation | Valid invalid unknown untrusted and modified-after-signing are distinct with evidence | SRC23 |
| D02 | Trusted roots and explicit timestamp configuration | P1 · Desktop CLI · L | sign.trust-import: partial; sign.timestamp-servers: planned; sign.os-trust: planned | Policy and platform stores | Trust-source and verification-time choices are visible and reproducible | SRC23 |
| D03 | Complete PAdES timestamp and long-term-validation workflows | P2 · Desktop CLI · XL | sign.pades-bt: partial; sign.ltv: partial; sign.pades-blta: partial; sign.ocsp-crl: partial | RFC3161 OCSP CRL external validation | Each claimed profile passes named independent tests and clearly reports offline evidence limits | SRC23 |
| D04 | PKCS#11 tokens and smart-card support | P2 · Desktop CLI · XL | sign.pkcs11: planned | Safe maintained wrapper and hardware | PIN errors cancellation supported token types and repeated signing pass hardware tests | SRC23 |
| D05 | Signature field-lock and certification preview | P1 · Desktop Web CLI · L | sign.lock-after-signing: planned; sign.field-mdp: planned; sign.preview-mode: planned | FieldMDP policy | User sees permitted future changes and warnings before signing | SRC23 |
| D06 | Configurable reusable signature appearances | P1 · Desktop Web CLI · M | sign.signature-appearance: partial | Appearance policy | Text graphic role date timezone and company details render consistently without misleading trust indicators | SRC03 |
| D07 | Multi-recipient signing envelopes | P2 · Desktop Web Mobile Service · XL | sign.request-esignatures: na | Identity consent storage service | Signing order roles expiry reminders rejection and completion evidence are auditable | SRC03 SRC23 |
| D08 | Browser signing with strong key-handling policy | P2 · Web · XL | sign.web-signing: planned | Web crypto compatibility threat model | Keys are not persisted or transmitted by default and memory/session handling is documented | SRC23 |
| D09 | Trusted service provider and remote signing adapters | P3 · Desktop Web Service · XL | sign.cloud-signatures: na; sign.eutl: planned | Legal review provider contracts | Signature level identity checks trust chain and provider jurisdiction are explicit | SRC23 |
| D10 | Signature evidence package and verification report | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Validation report schema | PDF original signed bytes certificates timestamps and transaction evidence remain distinguishable | SRC23 |

## A Accessibility

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| A01 | Keyboard-only complete everyday workflows | P0 · Desktop Web · L | a11y.keyboard-only: planned | Focus audit | Open search annotate fill save and print work without a mouse with no keyboard traps | SRC17 |
| A02 | Screen-reader labels focus and navigation audit | P0 · Desktop Web · L | a11y.app-screen-reader: partial | AccessKit/browser semantics | Named screen readers can identify controls fields errors progress and document navigation | SRC17 |
| A03 | Reading order Tags and Content remediation tools | P1 · Desktop Web CLI · XL | a11y.reading-order-tool: planned; a11y.tags-panel: planned; a11y.content-panel: planned; a11y.order-panel: planned | Structure editing model | A human can repair sequence inspect artifacts and validate the result | SRC18 SRC19 |
| A04 | Automatic tagging with mandatory review | P2 · Desktop Web CLI · XL | a11y.autotag: planned; a11y.autotag-fields: planned | Layout semantics | Suggested headings tables forms and artifacts are inspectable undoable and never labelled automatically compliant | SRC18 SRC19 |
| A05 | Accessible table editor and header associations | P1 · Desktop Web CLI · XL | a11y.table-editor: planned | Structure table model | Users repair spans header scope and reading order with independent validation | SRC18 |
| A06 | Guided make-accessible workflow and reports | P1 · Desktop Web CLI · L | a11y.make-accessible-action: planned; a11y.setup-assistant: planned | Checker remediation links | Report separates passed failed skipped unsupported and needs-human-review findings | SRC19 |
| A07 | Read aloud with word/page tracking | P2 · Desktop Web Mobile · L | a11y.read-aloud: planned; a11y.read-aloud-options: planned | Opt-in platform TTS | Voice speed language and reading order are adjustable without uploading text by default | SRC17 |
| A08 | Large targets zoom and non-drag alternatives | P0 · Desktop Web Mobile · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Responsive widgets | Controls remain usable at high UI scaling and every drag action has an accessible alternative | SRC17 |
| A09 | PDF/UA-1 and PDF/UA-2 validation adapters | P1 · Desktop CLI · L | optimize.pdfua-validate: planned | External oracle integration | Named profile tool version and machine-versus-human limits appear in every result | SRC18 SRC19 SRC29 |
| A10 | Translation quality and non-Latin font coverage gates | P1 · Desktop Web Mobile · L | misc.prefs-language: partial | Licensed fonts native reviewers | Critical labels errors form values and filenames display correctly in supported languages | SRC17 |

## N Engineering and contracting

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| N01 | Drawing-set register with sheet and revision metadata | P1 · Desktop Web CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Project document model | Sheet number discipline revision status and superseded copies are visible and filterable | SRC11 |
| N02 | Aligned colour overlays and visual drawing comparison | P1 · Desktop Web CLI · XL | misc.compare-scanned: planned; misc.compare-filters: planned | Registration transform | Users manually confirm alignment and review differences without treating scan noise as design changes | SRC11 SRC13 |
| N03 | Batch slip-sheeting with markup preservation | P2 · Desktop CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Drawing matching comment migration | Replacement previews matched unmatched and ambiguous sheets and retains an immutable old revision | SRC11 SRC13 |
| N04 | Batch hyperlinks from sheet/detail references | P2 · Desktop CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Sheet registry OCR | Links target the correct issued revision with an ambiguity report | SRC13 |
| N05 | Calibrated multi-viewport measurements and units | P1 · Desktop Web CLI · L | comment.measure-scale: partial; comment.measure-distance: partial | Scale metadata | Each measurement retains calibration source units and viewport; exported totals are reproducible | SRC13 |
| N06 | Count angle volume and formula takeoffs | P2 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Measurement geometry | Counts and quantities show formulas scale units rounding and manual corrections | SRC13 |
| N07 | Quantity schedules and spreadsheet handoff | P2 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Takeoff schema | CSV/XLSX values reconcile with selected markups and formulas are preserved or disclosed | SRC13 |
| N08 | RFIs submittals and punch-list issue pins | P1 · Desktop Web Mobile Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Issue model project permissions | Issue links include drawing revision owner due date status photos and closeout evidence | SRC11 SRC15 |
| N09 | Controlled transmittals and approval stamps | P1 · Desktop Web CLI Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Manifest and approval model | An issued package records purpose recipient exact revision hash and acknowledgement separately from certificate signing | SRC11 |
| N10 | As-built and O&M handover binder builder | P2 · Desktop CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Metadata bookmarks manifests | Deliverables include searchable index discipline sections equipment records and missing-document checks | SRC11 |
| N11 | Site photo and location evidence linked to drawings | P2 · Desktop Web Mobile Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Consent mobile capture | Photo timestamp location consent and exact drawing revision remain attached through issue resolution | SRC15 |
| N12 | Bilingual contract/tender comparison report | P2 · Desktop Web CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Arabic text extraction aligned diff | Changes in clauses quantities dates and parties link back to original pages for human review | SRC01 SRC11 |

## H Sharing and collaboration

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| H01 | One Share/Send workflow with explicit output type | P1 · Desktop Web Mobile · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Export intent model | User chooses attachment sanitized copy review link or signing request and sees privacy consequences | SRC02 |
| H02 | Send a copy through native email/share sheet | P1 · Desktop Web Mobile Integrations · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Platform handoff | Correct current saved bytes are attached with a clear unsaved-changes decision | SRC02 |
| H03 | Compress-to-email copy and page-range sharing | P1 · Desktop Web CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Export and compression presets | Original is unchanged selected page count size and omitted attachments are previewed | SRC08 |
| H04 | Share preflight for secrets metadata signatures and forms | P0 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Inspection policy | Copy warns about hidden information unresolved redactions attachments form data and signature invalidation | SRC22 SRC23 |
| H05 | Offline review package with PDF XFDF and manifest | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Stable IDs package format | Recipients can review without an account and returned comments import without silent duplication | SRC12 |
| H06 | Protected recipient-specific links with expiry/revocation | P2 · Desktop Web Mobile Service · XL | misc.share-links: na | Identity access-control storage | New access is denied immediately after revocation and token/preview/download endpoints use the same policy | SRC02 |
| H07 | Recipient roles and domain-limited guest review | P2 · Web Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Authorization model | Viewer commenter approver and owner permissions are enforced on the server not only in UI | SRC02 SRC12 |
| H08 | Web viewer for invited guests without editor installation | P2 · Web Mobile Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Responsive accessible viewer | Invitation sign-in expiry permissions and supported browsers have tested guest flows | SRC02 |
| H09 | Threaded reviews mentions and assigned tasks | P2 · Desktop Web Mobile Service · XL | comment.shared-review: na; comment.mentions-live-review: na | Identity notification service | Threads stay attached to the exact document revision and users can resolve or reopen issues | SRC02 SRC15 |
| H10 | Review dashboard deadlines reminders and digest | P2 · Desktop Web Service · L | comment.review-tracker: na | Notification preferences | Users distinguish sent delivered opened commented approved and overdue states with known tracking limits | SRC02 SRC03 |
| H11 | Version history and optimistic concurrency for shared files | P0 · Desktop Web Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Version tokens immutable blobs | An old client cannot overwrite a newer version and a merge/conflict choice preserves both copies | SRC04 SRC12 |
| H12 | Tamper-evident activity and approval event log | P2 · Desktop Web CLI Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Trusted time append-only storage | Actor action document hash and time are exportable with retention limits and integrity verification | SRC23 |
| H13 | Real-time annotation synchronization and presence | P3 · Desktop Web Mobile Service · XL | comment.mentions-live-review: na | Durable event log offline queues | Concurrent annotation edits converge or expose conflicts without dropping an acknowledged edit | SRC12 SRC14 |
| H14 | Offline comments queue and reconnect reconciliation | P2 · Desktop Web Mobile Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Stable annotations revisions | Disconnected reviews replay exactly once and edits to superseded files are flagged | SRC12 |
| H15 | Self-hosted and approved-region sharing providers | P2 · Desktop Web CLI Service · XL | misc.share-links: na; misc.document-cloud-storage: na | Operations governance | Administrator controls storage region backup keys retention and deletion visibility | SRC10 SRC22 |
| H16 | Cloud-drive check-in/out and change notifications | P2 · Desktop Web Integrations · XL | misc.connected-storage: na | OAuth provider adapter | Permissions are least-privilege and provider conflicts never silently discard local edits | SRC04 SRC27 |
| H17 | Preview-scoped project links and QR handoff | P2 · Desktop Web Mobile Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Short-lived authorization | QR codes contain no credentials in logs and respect the same expiry and recipient policy | SRC02 |
| H18 | Upload/request-files portal for suppliers | P2 · Web Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Upload isolation quotas | Guests upload only to their assigned request with malware handling quotas and receipt manifest | SRC11 |
| H19 | Watermarked recipient view and download policy | P2 · Web Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Server rendering authorization | View controls never claim to prevent screenshots or copying of already downloaded files | SRC02 |
| H20 | Retention legal holds deletion and evidence export | P2 · Desktop Web Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Governance model | Deletion covers defined active derivatives while retention backups and holds are disclosed | SRC22 SRC23 |

## W Web application

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| W01 | Crash-safe web working copy and recovery | P0 · Web · XL | core.web-recovery: planned; view.web-app: partial | IndexedDB/OPFS quota handling | Reload crash quota exhaustion and private mode never falsely report a durable save | SRC08 |
| W02 | Browser capability and support matrix | P0 · Web · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Automated browser tests | Unsupported signatures printing file APIs and OCR are detected and explained before work starts | SRC08 |
| W03 | Explicit download/save acknowledgement | P0 · Web · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Host/file API abstraction | User knows whether edits are only in memory downloaded or saved to a connected host | SRC08 |
| W04 | Offline-capable installable web app | P2 · Web · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Versioned cache and storage policy | Offline availability is explicit app updates do not invalidate unsaved working copies | SRC08 |
| W05 | Progressive rendering and bandwidth-conscious opening | P1 · Web · XL | core.linearization: planned; core.lazy-loading: planned | Range-capable byte source | First-page work can start before full download without bypassing access checks | SRC08 |
| W06 | Print handoff with truthful browser limitations | P1 · Web · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Browser print/export pathway | Page size scale duplex driver limits and fallback download are explained | SRC08 |
| W07 | Harden existing same-origin host bridge | P0 · Web Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Versioned host protocol | Origin checks save acknowledgements conflict tokens and document identity prevent cross-file overwrite | SRC25 |
| W08 | Mobile-responsive browser reading and review | P1 · Web Mobile · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Touch layouts accessibility | Small screens expose comments search forms and save without hidden controls | SRC16 SRC17 |
| W09 | Background worker isolation and job throttling | P1 · Web · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Worker model resource budgets | Heavy documents keep the UI responsive and cancellation cleans up memory | SRC08 |
| W10 | Page/comment deep links tied to document revision | P2 · Web Service · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Persistent revision IDs | A shared location opens the right revision page label and comment or reports it missing | SRC02 |

## M Mobile and tablet

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| M01 | Dedicated iOS/Android capture and review app | P3 · Mobile · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Mobile product decision | Platform targets maintenance budget accessibility and offline policy approved before implementation | SRC16 |
| M02 | Offline project packs and explicit sync state | P2 · Mobile Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Encrypted storage sync protocol | Users know which revision is available offline and cannot unknowingly upload stale replacements | SRC12 |
| M03 | Camera scanning batch crop and reorder | P2 · Mobile · L | ocr.camera-perspective: planned | Camera permissions | Capture works without public upload and produces an ordered searchable handoff | SRC16 |
| M04 | Touch-first forms and on-site signatures | P2 · Mobile Web · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Responsive form and signature UI | Required fields signature intent and final-copy save work with one-handed/touch use | SRC16 |
| M05 | Stylus eraser palm rejection and quick palettes | P2 · Mobile Desktop · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Native pen APIs | Supported pen buttons pressure gestures and accessibility fallbacks are tested | SRC14 |
| M06 | Photo-backed punch inspections and checklists | P2 · Mobile Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Issue model | Inspection findings keep checklist version location photo and responsible party | SRC15 |
| M07 | Secure receive-from and send-to OS share sheets | P1 · Mobile Integrations · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | OS document providers | App receives the selected copy with correct filename type and ownership | SRC16 |
| M08 | Biometric app lock and managed wipe | P2 · Mobile · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | OS keystore MDM policy | Lock protects local sensitive caches and remote wipe never claims to erase exported copies | SRC22 |
| M09 | QR-based drawing and equipment lookup | P2 · Mobile Web Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Document registry access policy | Code resolves authorized current/as-built revision with clear offline fallback | SRC11 |
| M10 | Voice notes with optional local transcription | P3 · Mobile Desktop · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Consent speech model | Recording permission language confidence and data processing location are visible | SRC15 |

## B CLI and automation

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| B01 | Complete tool coverage for shipped GUI workflows | P1 · CLI Desktop Web · L | misc.cli-completeness: partial | Command registry audit | Every claimed headless feature has schema examples and end-to-end save/reopen tests | SRC28 |
| B02 | Versioned schemas stable exit codes and structured errors | P0 · CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | API compatibility policy | Scripts can distinguish unsupported malformed refused partial and failed outcomes without parsing prose | SRC28 |
| B03 | Dry-run and destructive-operation approval plan | P0 · CLI Desktop · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Transaction planning | Users inspect targeted paths pages metadata and irreversible changes before execution | SRC26 |
| B04 | Folder watch and recursive batch workflows | P1 · CLI Desktop · XL | misc.batch-processing: partial | File-event dedupe job scheduler | Partial writes file loops exclusions and duplicate events cannot cause duplicate destructive processing | SRC09 |
| B05 | Portable workflow presets and action library | P1 · CLI Desktop Web · L | misc.action-wizard-manage: planned; misc.custom-commands: planned | Versioned workflow schema | Imported recipes validate dependencies permissions and output locations before running | SRC09 |
| B06 | Checkpoint resume retry and idempotency | P1 · CLI Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Job state and output manifests | Retries never duplicate signatures pages emails or external side effects | SRC28 |
| B07 | Bounded worker pools and resource controls | P0 · CLI Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Shared scheduler | Concurrency input/output quotas and timeouts can be configured and audited | SRC26 |
| B08 | Optional authenticated service API and webhooks | P2 · CLI Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Explicit service deployment approval | No listener starts by default and every request has auth authorization quotas and idempotency semantics | SRC28 SRC26 |
| B09 | MCP permission profiles and per-action approvals | P0 · CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Tool-boundary enforcement | Read-only project-scoped and destructive profiles cannot be escalated by document instructions | SRC24 SRC26 |
| B10 | Machine-readable validation and evidence artifacts | P1 · CLI · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Report schema | Reports name input hash tool/model version settings limitations output hashes and failures | SRC19 SRC28 |

## I Integrations

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| I01 | Extend existing Nextcloud host integration safely | P1 · Web Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Same-origin bridge version policy | Host permission changes conflict resolution and save failure are tested end to end | SRC04 |
| I02 | OneDrive SharePoint and Microsoft Teams handoff | P2 · Desktop Web Integrations · XL | misc.connected-storage: na | OAuth Graph approval | Minimal delegated scopes and check-in/out behavior match enterprise policy | SRC04 SRC27 |
| I03 | Google Drive and Workspace handoff | P2 · Desktop Web Integrations · XL | misc.connected-storage: na | OAuth provider policy | Open/save/share respects provider file permissions and conflict versions | SRC04 |
| I04 | SMB WebDAV and local network share robustness | P1 · Desktop CLI Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Atomic-write fallback policy | Offline reconnect permissions and failed rename never silently destroy a shared file | SRC27 |
| I05 | Outlook email attachment workflow | P1 · Desktop Integrations · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Email adapter | Reply with marked-up copy or controlled link never sends stale unsaved bytes | SRC02 |
| I06 | Explorer/Finder quick actions and thumbnails | P1 · Desktop Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Sandboxed shell extension | Merge convert print and redact-copy actions have clear confirmation and crash isolation | SRC06 |
| I07 | Company template and preset distribution | P1 · Desktop CLI Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Signed configuration package | Admin-managed originals can be updated while user customizations stay separate | SRC27 |
| I08 | Power Automate and no-code connector | P2 · Service Integrations · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Authenticated service API | Connector exposes bounded approved operations with retries and structured outcomes | SRC28 |
| I09 | DMS adapters for Aconex Procore or existing project system | P2 · Desktop Web Integrations · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Customer-selected integration contract | Correct document revision status role and transmittal metadata round-trip without duplicate records | SRC11 |
| I10 | Accounting ERP and invoice-processing export | P2 · CLI Service Integrations · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Validated extraction schema | Invoice references amounts taxes dates and source evidence are human-approved before posting | SRC05 SRC28 |

## T Deployment and support

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| T01 | Signed installers and macOS notarization gate | P0 · Desktop Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Release credentials secure build | A production release fails closed if mandatory signatures hashes or notarization are absent | SRC27 |
| T02 | MSI silent installation uninstall and managed update policy | P1 · Desktop Integrations · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | IT packaging matrix | Install repair upgrade rollback uninstall and associations pass managed deployment tests | SRC27 |
| T03 | Portable and offline-install packages | P1 · Desktop CLI Integrations · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Dependency/model packaging | An offline install includes required models fonts licences and verification checksums | SRC06 |
| T04 | Administrator-managed security and provider settings | P1 · Desktop Web CLI Integrations · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Policy schema trust source | Admin can disable cloud AI scripting external links updates and credentials storage without hiding errors | SRC04 SRC05 |
| T05 | Optional SSO MFA and tenant provisioning for collaboration | P2 · Web Service Integrations · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Hosted team service | SSO recovery deprovisioning roles and tenant isolation are security tested; local editor stays account-free | SRC05 SRC27 |
| T06 | SBOM dependency policy and security response | P0 · Desktop Web CLI Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Release provenance | Every artifact has an SBOM known-vulnerability policy security contact and patch process | SRC04 |
| T07 | Accessible onboarding and workflow guides | P1 · Desktop Web Mobile · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | User tests localized help | A novice completes common tasks without mandatory tours accounts or document uploads | SRC08 SRC17 |
| T08 | Consent-only product health metrics | P2 · Desktop Web Service · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Privacy budget retention | Metrics never contain document text or filenames; administrators and users can opt out | SRC22 |
| T09 | Published platform capability and release-support matrix | P0 · Desktop Web CLI Integrations · M | proposal / enhancement; no exact parity mapping; verify scope before implementation | Named supported targets | Each release states supported OS/browser versions signing state feature limits and end-of-support policy | SRC27 |
| T10 | Support/export compatibility repair assistant | P1 · Desktop Web CLI · L | proposal / enhancement; no exact parity mapping; verify scope before implementation | Diagnosis rules | Assistant identifies missing fonts damaged structure unsupported features and safe copy/fallback options | SRC06 |

## Q Advanced PDF and print

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| Q01 | General preflight profiles and fixups | P2 · Desktop CLI · XL | optimize.preflight: planned; optimize.preflight-fix: planned; optimize.preflight-profiles: planned | Standards rules and reports | Users select a named profile inspect findings and preview potentially appearance-changing fixes | SRC19 |
| Q02 | Complete PDF/A validation and conversion coverage | P1 · Desktop CLI · XL | optimize.pdfa-validate: partial; optimize.pdfa-convert: partial | Font embedding color handling external validator | Each supported part/level passes an independent validator and unsupported conversions fail honestly | SRC19 |
| Q03 | PDF/X and production print preflight | P2 · Desktop CLI · XL | optimize.pdfx-validate: planned; optimize.pdfx-convert: planned | ICC profiles print-domain expertise | Output-intent fonts bleed transparency and overprint are validated per named profile | SRC06 |
| Q04 | Output preview overprint and separation tools | P2 · Desktop · XL | print.output-preview: planned; print.separations: planned; print.overprint-preview: planned | Color-managed renderer | Preview identifies spot inks coverage and output intent without changing content | SRC06 |
| Q05 | Reliable mixed-size duplex tray and label printing | P0 · Desktop CLI · L | print.paper-source-by-size: planned; print.page-setup: planned | Driver capability model hardware | A4/A3 labels odd page counts long/short-edge and actual-size jobs pass printer acceptance tests | SRC01 SRC11 |
| Q06 | Print-as-image and raster grayscale fallback | P1 · Desktop CLI · L | print.print-as-image: planned; print.grayscale: partial | Raster pipeline | Users see DPI spool size and fidelity tradeoffs before bypassing a failing vector path | SRC06 |
| Q07 | Advanced colour conversion ink limits and print marks | P2 · Desktop CLI · XL | print.convert-colors: planned; print.ink-manager: planned; print.printer-marks: planned; print.total-area-coverage: planned | Color engine licensed profiles | Conversions have soft-proof preview and preserve or disclose spot-color changes | SRC06 |
| Q08 | Certificate-encrypted and legacy document compatibility | P2 · Desktop Web CLI · XL | core.open-public-key: planned; core.usage-rights-preserve: planned | Security handler preservation | Unsupported encryption fails safely and existing rights/signatures are not silently removed | SRC01 |
| Q09 | PDF/A-3 associated XML e-invoice packaging | P2 · Desktop CLI · L | optimize.einvoice-pdfa3: planned | Country-specific schema validator | Embedded invoice data matches visible totals and a jurisdiction-specific validator reports its scope | SRC19 |
| Q10 | Deterministic PDF serialization and web linearization | P2 · Desktop CLI · XL | core.deterministic-save: planned; core.linearization: planned | Stable serialization layout | Identical normalized inputs can reproduce hashes and range opening works with authenticated delivery | SRC08 |

## Z Future differentiation

| ID | Function | Priority / surface / effort | Baseline mapping | Dependency | Acceptance criterion | Basis |
|---|---|---|---|---|---|---|
| Z01 | Local/BYO-provider document Q&A with page citations | P3 · Desktop Web CLI · XL | misc.ai-provider: planned; misc.ai-ask: planned | Provider policy retrieval isolation | Answers link exact revision page passages and can say insufficient evidence | SRC04 SRC24 |
| Z02 | Bilingual summaries and side-by-side translation | P3 · Desktop Web CLI · XL | misc.ai-summary: planned; create.translate: planned | Arabic evaluation human review | Original wording remains visible and names amounts dates clauses and uncertainty are flagged | SRC04 |
| Z03 | Contract risk and obligation extraction | P3 · Desktop Web CLI · XL | misc.contract-analysis: planned | Legal-domain evaluation | Output is a review aid not legal advice and every obligation cites source evidence | SRC05 SRC24 |
| Z04 | AI-assisted PII marking with human apply step | P2 · Desktop Web CLI · XL | protect.redact-ai-suggestions: planned | PII evaluation redaction verification | Suggestions never transmit data or permanently redact until explicitly approved | SRC04 SRC24 |
| Z05 | AI-assisted alt text and structure suggestions | P3 · Desktop Web CLI · XL | a11y.alt-text-suggestions: planned | Semantic review workflow | Human can accept edit reject and audit every suggestion without automatic compliance claims | SRC18 SRC19 SRC24 |
| Z06 | Natural-language actions through approved command plans | P3 · Desktop Web CLI · XL | misc.ai-conversational-edit: planned | Command planner permission boundary | A user previews exact pages operations outputs and irreversible effects before execution | SRC24 SRC26 |
| Z07 | Semantic project search across authorized revisions | P3 · Desktop Web Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | ACL-aware encrypted indexes | Search respects current permissions and distinguishes current from superseded documents | SRC24 |
| Z08 | Drawing symbol search and assisted quantity takeoff | P3 · Desktop CLI · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Visual model calibrated geometry | Counts include confidence visual locations and correction controls; never used as certified quantities unchecked | SRC13 |
| Z09 | 3D PDF and BIM-linked inspection | P3 · Desktop Web · XL | misc.3d-view: planned; view.model-tree: planned | Safe U3D/PRC support and customer demand | Model opening is sandboxed and links retain original drawing/document evidence | SRC13 |
| Z10 | Geospatial drawings and field positioning | P3 · Desktop Mobile · XL | comment.geospatial: planned | Coordinate reference systems consent | Accuracy projection units and unavailable positioning are explicit | SRC11 |
| Z11 | Handwriting search and recognition | P3 · Desktop Mobile · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Licensed language models stylus corpus | Original ink is preserved recognized text is searchable and confidence correction is available | SRC14 |
| Z12 | Immutable project closeout and evidence timeline | P3 · Desktop Web CLI Service · XL | proposal / enhancement; no exact parity mapping; verify scope before implementation | Signed manifests retention trusted timestamps | Every deliverable maps to an issued revision evidence history and verification procedure without claiming automatic legal admissibility | SRC11 SRC23 |
