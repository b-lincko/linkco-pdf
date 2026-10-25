# Linkco PDF — market requirements, missing capabilities and product strategy

**Research date:** 25 October 2026
**Repository baseline:** `1d7f56366160011e5bf9e370646bf1b45ba5fc79`
**Purpose:** a decision-ready inventory, not an implementation change or a production certification.

## Executive recommendation

**Make Linkco the dependable, offline-first, bilingual PDF workbench for contracting and business document workflows.** Do not compete first on the largest number of menu items or on an AI chat box. Compete on correct output, safe saves, good printing, easy daily tasks, drawing revision control, and controlled exchange of documents.

The recommended order is:

1. **Establish trust:** rendering/save fidelity, resource limits, secure redaction, predictable signatures, accessible operation, truthful capability claims and signed release artifacts.
2. **Remove daily friction:** reliable search, text fixes, Arabic/English handling, OCR, common conversions, page assembly, print presets and cancellable jobs.
3. **Make exchange useful without requiring a cloud:** share a safe copy, prepare an email-sized PDF, exchange offline review packages and create controlled transmittals.
4. **Add optional team services:** authenticated review links, version control, assignments, approvals, retention and signing envelopes. Prefer existing storage/identity integrations before building a new general-purpose cloud drive.
5. **Differentiate for contracting:** drawing registers, aligned overlays, calibrated quantities, revision-aware markups, RFIs/submittals and handover binders.
6. **Invest later in evaluated intelligence:** citation-backed local/BYO-provider Q&A, bilingual extraction and assisted review. Never let document content authorize a transmission or destructive operation.

This is a **recommendation inferred from the product, competitor workflows and the company's stated use cases**, not a measured market-share or willingness-to-pay finding. No interviews, customer survey or competitor hands-on benchmark was performed.

### Deliverables and how to use them

| File | Use |
|---|---|
| **[Excel workbook](Linkco-PDF-Product-Strategy.xlsx)** | Filterable Read me, Recommendations, Baseline 833, Gaps 384 and Sources sheets |
| [Detailed recommendations](recommended-roadmap.md) | **222** recommendations with priority, application surface, baseline mapping, dependencies, relative effort and acceptance criterion |
| [Complete existing gap register](existing-gaps.md) | All **308 planned + 76 partial = 384** unfinished checklist entries, with their recorded limitations |
| [Complete baseline CSV](baseline-features.csv) | All **833** checklist entries, including shipped/out-of-scope entries, command/tool IDs and evidence references |
| [Roadmap CSV](recommended-roadmap.csv) / [gap CSV](existing-gaps.csv) | Import into a backlog or spreadsheet |
| [Sources and limitations](sources.md) / [source CSV](sources.csv) | **29** primary/vendor/standards/security references and what each does and does not establish |

**Do not add 222 to 384:** recommendations overlap existing gaps, strengthen existing features and introduce new product surfaces. The counts are different views of the work, not independent feature totals. “Complete” means complete extraction of this pinned checklist plus the explicitly scoped recommendation inventory; it does not mean every possible future PDF feature has been discovered.

---

## 1. Scope: what “each app” means

Only the `b-lincko/linkco-pdf` repository was audited. There are three product applications under `apps/`: `pdfcraft` (desktop), `pdfcraft-web` and `pdfcraft-cli`. The CLI is also the entry point for opt-in MCP automation. The repository additionally contains OS integration/packaging code and an existing same-origin browser host bridge.

| Surface | Exists here? | Proposed role |
|---|---|---|
| Desktop — Windows, macOS, Linux, FreeBSD | Yes; platform-specific capability varies | Full offline workbench: edit, assemble, review, sign, OCR, print and prepare controlled copies |
| Web/browser | Yes; WebAssembly, shared UI and host integration | Fast reader/reviewer/form workflow with explicit save/recovery and browser capability limits |
| CLI/automation/MCP | Yes | Repeatable, bounded processing and evidence reports; no network listener by default |
| OS/storage integrations | Yes in part: shell preview/thumbnail/file opening and browser-host integration | Safe handoffs to email, local/network drives and approved document stores |
| Native mobile/tablet companion | **Not an application in `apps/`** | Optional future capture, field review, forms, signatures and offline project packs |
| Collaboration/sharing service | **Not an established standalone product application in this checkout** | Optional new service or provider integration, requiring identity, authorization, operations and retention design |

If “each app” was intended to include other Linkco products, those repositories were not supplied and no claim about their missing features is made here. This report is for the PDF product family.

### Method and confidence

- Parsed every checklist entry; preserved its exact status, notes, tier, evidence, commands and tools.
- Examined the README, architecture/gap/parity documents and the OCR, export, compare, accessibility, preflight, automation, web-host and UI modules.
- Checked that all referenced evidence **file paths** exist at the pinned snapshot. This does not prove a named test exists or passes.
- Researched official competitor feature/help pages, W3C/standards documentation, OWASP and relevant government guidance. Selected full pages were fetched; sources based only on search extracts are explicitly labelled.
- Sampled public upstream issues for qualitative user pain. They are neither a representative survey nor confirmed defects in this fork.
- Did **not** run a new GUI session, Cargo tests, hardware lab or document-fidelity benchmark during this research task. Earlier CI results are not a substitute for the proposed acceptance tests.

Status language throughout:

- **Repository-shipped:** checklist says shipped; not freshly certified here.
- **Partial:** existing implementation with a recorded limitation; improve rather than duplicate it.
- **Planned:** checklist says not implemented; verify against current code before estimating.
- **Out of scope (`na`):** an upstream/cloud/legacy item excluded by existing product policy; not automatically a missing requirement.
- **Proposal/unmapped:** a suggested enhancement without an exact checklist match; not an assertion that all related functionality is absent.
- **Conflict:** documentation/code/checklist disagree; require a targeted demonstration before claiming either complete support or complete absence.

---

## 2. What the market expects — evidence, not hype

### 2.1 Competitive workflow benchmarks

These are **advertised capabilities**, not an independently tested feature ranking. Edition, platform, region and service terms matter.

| Benchmark | What the published offering demonstrates | Implication for Linkco |
|---|---|---|
| Adobe Acrobat | Mainstream editing, Office conversion, page organization, forms/signatures; advanced OCR/redaction; shared review links and tracked signature requests. [18](https://www.adobe.com/acrobat/plans.html) [3](https://helpx.adobe.com/acrobat/desktop/share-and-review-documents/share-documents/share-pdfs.html) [4](https://helpx.adobe.com/acrobat/desktop/e-sign-documents/request-e-signatures/send-for-signing.html) | An editor needs a coherent document lifecycle, not just individual PDF operations. |
| Foxit | Editing, OCR, comparison, forms; connected DMS, check-in/out, version history, integrations and administrative controls are advertised. [26](https://www.foxit.com/pdf-editor/) | Teams will compare safe version handling and existing-stack integration as well as editing breadth. |
| Nitro | Editing/conversion, data extraction, enterprise SSO/domain controls and managed deployment/integrations. [2](https://www.gonitro.com/pdf) [3](https://www.gonitro.com/hubfs/DeploymentGuide_NamedUserLicensing_May2026latestversion.pdf?hsLang=en) | IT acceptance, structured data output and deployment are product functions, not afterthoughts. |
| PDF-XChange | Dense desktop toolset, advanced OCR, forms, document comparison, portable/MSI/ARM64 packages and perpetual licensing. [2](https://www.pdf-xchange.com/product/pdf-xchange-editor) [1](https://www.pdf-xchange.com/knowledgebase/351-How-do-I-OCR-documents-with-the-PDF-XChange-family-of-products) | Strong offline power-user workflows are a viable benchmark; distinguish searchable-image OCR from editable OCR. |
| Smallpdf | Task-first browser tools: merge/split/compress, Office/image conversion, OCR, forms, protection and AI utilities. [3](https://smallpdf.com/pdf-tools) | Common tasks must be obvious, fast to start and clear about output and processing location. |
| iLovePDF | Chained workflows, API/no-code automation, offline options and regional processing controls. [4](https://www.ilovepdf.com/blog/automate-pdf-tasks-with-ilovepdf-workflows) [1](https://www.ilovepdf.com/blog/api-no-code-pdf-automation-ilovepdf) [1](https://www.ilovepdf.com/blog/choose-where-your-pdf-files-are-processed) | Reusable task sequences and control over data movement can matter more than adding another tool. |
| Bluebeam | Drawing registers/revisions, overlays, slip-sheeting, measurements, linked issues, transmittals, and separate live sessions versus document storage. [11](https://www.bluebeam.com/workflows/drawing-and-document-management/) [1](https://www.bluebeam.com/pricing/) [3](https://support.bluebeam.com/revu/subscription/studio-and-bluebeam-cloud-comparison-guide.html) | For engineering/contracting, revision correctness, quantities and accountable review are high-value differentiation. |
| Drawboard | Pen-focused markup, live/asynchronous collaboration, drawing-pinned issues, ownership and due dates. [3](https://www.drawboard.com/pdf/product-tour) [3](https://www.drawboard.com/projects/tour) | Field review needs comfortable pen/touch interaction plus a way to close issues, not just create annotations. |
| PDF Expert | Mobile form filling, signing, redaction and Pencil-based use. [1](https://pdfexpert.com/ios/features) | A mobile companion should be optimized for capture/review, not a cramped copy of the desktop UI. |
| Docusign | Recipient identity verification, transaction audit trails and completion evidence. [1](https://www.docusign.com/products/electronic-signature/features) | Sending a signature request is an identity/evidence workflow, distinct from placing a signature image or creating a local digital signature. |

**Interpretation:** the mainstream baseline is view/edit/organize/convert/OCR/forms/sign/protect/print; business differentiation comes from reliability, integration, controlled exchange and repeatable work. Specialized AEC differentiation comes from drawing revisions, takeoff and traceable issue closure. This interpretation is a product hypothesis grounded in the offerings above, not a statistical claim about all buyers.

### 2.2 Segment-specific requirements

The needs below are **hypotheses to validate with representative users**:

| Audience | Job to be done | Basic must-have for that audience | Valuable differentiator |
|---|---|---|---|
| Office/admin/HR | Assemble, fill, approve and send routine documents | Search, page assembly, simple edits, forms, signatures, email-sized copies, dependable print | Templates, local autofill, reusable action sequences |
| Contracting/engineering/document control | Work from the correct drawing/tender revision | Clear sheet/revision identity, A3/A4/large-format print, measurements, comments, compare, issue record | Slip sheets, overlays, transmittals, RFIs/submittals, handover binders |
| Finance/procurement | Extract and verify invoices/quotations | Accurate OCR, tables, amounts/dates, redaction, auditability | Excel/CSV export, source-linked extraction, ERP handoff |
| Legal/compliance | Preserve evidence and remove sensitive material | Verified redaction, revision-aware signatures, stable export, retention controls | Folder redaction, certificate/token support, contract comparison |
| Site teams | Inspect and report away from a desk | Responsive mobile review, offline files, current revision indication, photos, easy markup | Assigned punch items, camera scanning, QR equipment/document lookup |
| IT/security | Deploy, control and support the application | Signed packages, silent install, update policy, offline operation, clear data flows | Managed policies, optional SSO for services, approved storage and support evidence |
| Users with disabilities | Complete the same everyday tasks independently | Keyboard-only workflows, screen-reader semantics, visible focus, scale/contrast | Reflow, read-aloud and guided document accessibility remediation |

Do not label takeoff, 3D viewing or a collaboration subscription “basic” for every reader. Conversely, Arabic/English correctness and drawing fidelity can be purchasing gates for Linkco's intended workflows even if they are not universal requirements for every PDF customer.

### 2.3 Regulation and procurement: separate three obligations

1. **Application accessibility:** the UI itself must be operable. Use WCAG 2.2 AA as the web design target and a mapped native accessibility test plan. Keyboard access, unobscured focus, non-drag alternatives and target sizing are concrete criteria; not every criterion has the same level or exceptions. [1](https://www.w3.org/TR/WCAG22/)
2. **Accessible output documents:** document tags, language, reading order, tables and meaningful alternatives require their own tools and evaluation. PDF/UA-2 concerns accessible PDF 2.0; a checker cannot judge every semantic property. veraPDF states that its PDF/UA validation covers machine-verifiable checks. [10](https://pdfa.org/iso-14289-2-pdf-ua-2-the-gold-standard-for-accessibility-in-pdf-2-0-has-arrived/) [6](https://docs.verapdf.org/validation/)
3. **Applicable law and contracts:** the European Accessibility Act applies to enumerated products/services from 28 June 2025 with specific exemptions/transitions; it is **not** a rule that every PDF everywhere must carry a PDF/UA badge. [1](https://commission.europa.eu/strategy-and-policy/policies/justice-and-fundamental-rights/disability/european-accessibility-act-eaa_en) [2](https://eur-lex.europa.eu/EN/legal-content/summary/accessibility-of-products-and-services.html)

For Qatar-based workflows, NCSA identifies Law No. 13 of 2016 on personal-data privacy protection. That supports privacy-aware processing, access control and accountable data handling; this report does **not** infer a blanket Qatar-only hosting rule or a universal cross-border ban. Determine legal basis, special-data handling, contracts, retention and transfer requirements with qualified counsel before deploying a sharing or AI service. [1](https://ncsa.gov.qa/en/pages/personal-data-privacy-personal-data-privacy-protection-law)

Company ISO certifications, a secure transport connection, an automated check, or a signature image do **not** by themselves certify this application, a cloud service, a document's legal status or full accessibility.

---

## 3. Actual baseline: what exists, what is missing, what is uncertain

### 3.1 Inventory counts

The machine-readable baseline is `parity/acrobat-features.toml` at the pinned commit.

| Checklist status | Count | Meaning in this study |
|---|---:|---|
| Shipped | 426 | Existing capability with cited repository evidence; not a fresh usability/fidelity certification |
| Partial | 76 | Improve/finish what exists |
| Planned | 308 | Recorded as not started; verify current implementation before estimating |
| Not applicable/out of scope | 23 | Mostly vendor-cloud/legacy entries, not automatically required |
| **Total** | **833** | **810 in-scope entries**, not 833 missing features |

Of the original checklist's 252 P0 entries, **226 are marked shipped, 23 partial and 3 planned**. The three planned P0 items are bounded lazy loading, complete keyboard operation and performance budgets. These are the original checklist tiers, distinct from the research recommendation priorities.

Do not translate these numbers directly into “production readiness” or customer satisfaction percentages. Some partial labels reflect implementation ownership (for example using the bootstrap renderer), not a demonstrated failure in every rendering case. Improving an existing renderer with measured fidelity may deliver more value than rewriting it solely to change the label.

### 3.2 Existing strengths — do not build them twice

The repository already has a substantial base: file open/save and recovery mechanisms; page thumbnails/navigation; merge/split/reorder/rotate/crop; comments and XFDF; form filling/authoring; drawn/typed/image signatures; local certificate signing and validation work; password protection/redaction/sanitization; Word/HTML/RTF/text/image export paths; OCR text-layer generation; measurements; action/CLI/MCP tooling; print imposition/duplex; and OS preview/thumbnail integration. These are **existing implementation areas**, not a claim that all modes, documents or platforms are complete.

Specific source anchors include:

- `crates/automation/README.md` — tool/API coverage and path confinement.
- `crates/ocr/src/lib.rs` — current engine, English/unaccented-Latin alphabet and invisible text layer.
- `crates/export/src/lib.rs` — flowing paragraph/image/table export model.
- `crates/compare/src/lib.rs` and `crates/ui-egui/src/compare_ui.rs` — text comparison, visual-region state and comparison panel/report.
- `crates/preflight/src/lib.rs` — object-graph PDF/A-2b/3b checks and limited conversion fixes.
- `crates/a11y/README.md` — 32-rule checker, manual-check states and missing remediation tools.
- `apps/pdfcraft-web/src/main.rs` — URL opens, unload warning and same-origin host bridge.
- `crates/sign/src/` — certificate/CMS/timestamp/revocation code; profile support still requires careful validation.

### 3.3 Important “basic missing” or incomplete workflows

| Need | Actual evidence | Work required |
|---|---|---|
| Reliable existing text editing | `edit.text-edit` partial | Finish fidelity/font behavior and publish supported/unsupported cases rather than add another edit mode. |
| Arabic/mixed-script editing | README claims Arabic shaping; `edit.rtl-cjk-editing` is planned | Reconcile new-text rendering versus existing-text editing, logical copy/search, form fields and UI bidi; do not call Arabic entirely absent. |
| Search beyond the active document | `view.advanced-search` partial; folder/index features planned | Search all open files, comments/bookmarks and authorized local folders with progress/cancellation. |
| Find-and-replace | `edit.find-replace` planned | Preview changes and preserve layout/protected content. |
| OCR beyond English | `ocr.ocr-languages` partial; source exposes only `en` | Add Arabic/bilingual and accented-Latin models, licensing, evaluations and language selection. |
| Reliable scan cleanup | Deskew/rotation/acquisition checklist items planned | Audit preprocessing claims; add reviewable cleanup and actual scanner workflows where needed. |
| Office conversion | Word export partial; XLSX/PPTX export and Office import planned | Prioritize Word quality, Excel tables and safe local Office-to-PDF; distinguish each direction. |
| Comparison side-by-side | README advertises synchronized comparison; checklist says planned; reviewed UI describes differences on the newer document | Demonstrate the exact workflow, then reconcile documentation and add synchronized panes if not supported. Existing comparison is not absent. |
| Conflict-safe page assembly | Form-name collisions and structure-tree merge planned | Avoid fields becoming unintentionally linked and preserve tags/destinations when combining. |
| Convenient sharing | Cloud-sharing/review entries are `na`; synced local folders/host handoffs already exist | Build safe copy/email/offline-review first; treat hosted links as an explicit optional scope expansion. |
| Accessible everyday use | Keyboard-only planned; screen-reader support partial | Test complete workflows, not labels alone. |
| Comfortable personalization | Toolbar/pinned tools planned; preferences partial | Finish quick actions, saved tool settings and discoverable workspace defaults. |
| Very large files | Lazy loading and performance budgets planned | Introduce measured global budgets and incremental input; avoid one huge file freezing unrelated work. |
| Web durability | Web app partial; recovery planned | Distinguish in-memory, locally persisted, downloaded and host-saved copies. |
| Managed release experience | Existing packages and workflows, but release-readiness document still has gates | Enforce signing, capability disclosure, install/upgrade/rollback tests and support policy for production releases. |

### 3.4 Documentation claims needing targeted verification

| Claim area | Why it needs verification | Safe product language until verified |
|---|---|---|
| “Full OCR/scan” | Current model is English/unaccented Latin; scanner acquisition and editable OCR remain planned | “Searchable-image OCR with documented model/language limits.” |
| “Full PDF/A/standards support” | Preflight currently targets selected object-graph checks/fixes for PDF/A-2b/3b; font embedding/content checks remain limitations | Name exact part/profile and unsupported checks; do not use a universal compliance badge. |
| “Full PAdES B-T/B-LT/B-LTA” | README is broader than several partial signing checklist entries | Name tested profiles/providers and explain offline evidence/trust limitations. |
| “Complete accessibility” | Checker is present; tag/order/table remediation is mostly planned | “Accessibility checks with manual-review states,” not automatic certification. |
| “Complete Arabic/RTL” | Rendering, translation, navigation, copy/search and existing-text editing are different capabilities | Publish a task-by-task bilingual support matrix. |
| “Barcode fields / HTML creation / comparison” | README and checklist use different scopes/statuses | Demonstrate exact create/read/edit surface and update one authoritative capability inventory. |

Before turning a `planned` entry into a ticket, inspect the current UI/API and resolve these conflicts. This prevents paying for duplicated work and prevents promising unavailable functionality.

---

## 4. Non-negotiable functions and quality gates

“Must have” has two meanings: a basic business job must be possible, and a safety property must hold whenever a feature is offered. A hosted link is optional; **authorization on that link is not optional once it exists**.

| Non-negotiable | Minimum acceptance gate |
|---|---|
| Open and render faithfully | Supported fonts, transparency, engineering vectors, forms and images have an independently reviewed regression corpus; unsupported content is reported. |
| Save without data loss | Atomic writes where supported; explicit fallback on network shares; disk-full/crash/concurrent-write tests; preserve the last valid copy. |
| Preserve document semantics | Do not silently strip unknown objects, signatures, attachments, tags, layers or form values during unrelated edits. |
| Navigate/search/copy | Search works on unvisited pages; page labels and extracted/copied text are correct for the advertised scripts. |
| Organize pages | Merge/split/reorder/rotate/extract are predictable, undoable before save and collision-safe. |
| Make basic text/image fixes | Supported operations preserve unrelated layout; substitutions and unsupported cases are visible. |
| Annotate and fill forms | Markups and values remain visible in another reader and when printed; keyboard operation works. |
| Distinguish signatures | Drawn mark, approval event, certificate signature and remote signing envelope have different labels and trust meanings. |
| Redact for real | Remove source content and recoverable history within declared coverage; verify sanitized outputs independently; never represent a rectangle overlay as redaction. |
| Print correctly | Actual size/fit, orientation, mixed sizes, selected ranges, duplex and driver options have device-level acceptance tests. |
| Remain responsive | Bounded work, cancellation, progress and job isolation for hostile/large documents. |
| Protect confidential data | Local operation without mandatory account/cloud; explicit opt-in before external processing; secrets never in logs. |
| Be accessible | Everyday tasks complete by keyboard; screen-reader semantics, focus and scaling tested on named platforms. |
| Handle failures honestly | Errors say what failed, whether bytes changed and how to recover; unsupported is never reported as successful. |
| Distribute trustworthily | Verified build provenance/signing appropriate to platform, dependency checks, security contact and rollback procedure. |

**Release rule:** block a production release for known data loss, recoverable “redacted” content, false signature-validity status, tenant/file authorization bypass, or silent content corruption. A fast green test suite cannot waive those gates.

---

## 5. Improvements for day-to-day functionality and usability

### 5.1 Design principles

- **Task names over internal modes:** “Combine documents”, “Prepare a safe copy”, “Recognize text”, “Compare revisions”. Keep advanced controls available through progressive disclosure.
- **One active document identity:** show filename, dirty state, revision/source and save destination; do not let a late download switch the document a user is editing.
- **Before/after previews:** page assembly, redaction, OCR, conversion, compression and printing must expose the exact affected pages/output.
- **Visible state changes:** save completion, background work, cancellation, offline status, upload, link permissions and signature changes need explicit feedback.
- **Reversible until intentionally finalized:** support undo/snapshot/copy workflows; warn before irreversible redaction, flattening or signature-invalidating edits.
- **Good defaults with remembered presets:** A4/A3/labels, actual-size versus fit, duplex, OCR language, author identity and export naming; do not store credentials as ordinary preferences.
- **Preserve the agreed print interaction:** “Print both sides” remains an icon + text on/off toggle in the preview footer beside sheet navigation (Off ↔ LongEdge), visibly selected when on. Do not turn it into a three-state cycle or front/back preview switch; any advanced short-edge option belongs separately.
- **Keyboard and touch equivalents:** no task should require precision dragging; test high DPI, large text, Arabic strings and small windows.
- **No surprise network action:** opening a PDF, selecting a tool or reading a summary must not silently start uploads, scripts or account registration.

### 5.2 Daily workflow acceptance scenarios

| Workflow | Desired end-to-end path | What to test |
|---|---|---|
| Tender package | Open inputs → OCR scans → reorder/bookmark → inspect → sanitize copy → compress → transmittal | Correct source order, no lost attachments, accurate page index and exact issued-file hash |
| Drawing revision | Open old/new → pair sheets → align overlay → inspect changes → assign issues → issue current set | Wrong scale/misalignment warnings, unresolved mappings and superseded-sheet visibility |
| Form/approval | Fill → validate → sign/approve → save copy → email or controlled link | Required fields, copy semantics, signature status and correct attachment bytes |
| Invoice/quotation | OCR → select table → review numbers/units → Excel/CSV → approve ERP handoff | Arabic/English text, decimal separators, totals and source evidence |
| Routine editing | Find clause → edit text → preview/reopen → compare → save new version | Substitution warnings, no movement of unrelated content, preserved links/tags |
| Site inspection | Download approved pack → work offline → markup/photo → assign issue → reconnect | Offline revision identity, conflict reconciliation, no lost acknowledged edits |
| Printing | Choose pages/size/scale → select duplex/tray → preview → spool/cancel | A3/A4 mixes, 4×6 labels, odd-page duplex and actual-size calibration |
| Accessible delivery | Inspect → fix tags/order/tables → manual review → named-profile validation → report | Correct logical reading order and no false “all checks passed” when checks were skipped |

### 5.3 Proposed success measurements — not existing performance claims

Run a pilot on a specified Windows office laptop, representative large drawings, supported browser/device classes and rights-cleared bilingual samples. Suggested initial targets:

- At least **90% unassisted completion** of a defined ten-task everyday suite; measure time-on-task and failure reasons, not just satisfaction scores.
- At least **30% fewer interactions** for the selected top repetitive workflows compared with the baseline UI, without hiding required safety decisions.
- **Zero observed data-loss/corruption or authorization failures** in the release acceptance suite; this is a gate, not proof that no unknown bugs exist.
- Establish measured p50/p95 budgets for first-page display, input latency, search and job cancellation by file class; approve numeric thresholds after baseline measurement instead of inventing one target for every PDF.
- Measure OCR CER/WER by language/document class; extraction accuracy for amounts/dates; false-positive/false-negative redaction suggestions; and human correction time. Do not quote an untested “99% accuracy”.
- Test print scale with a physical reference on selected printers, not only a screenshot of the preview.
- Track recoverable error rate, retries, stale-version conflicts and unresolved review items through opt-in, content-free telemetry or pilot observation.

---

## 6. Functions to add for sharing

### 6.1 Four separate products hidden inside the word “share”

1. **Send a file copy:** native email/share sheet, selected pages, compression, sanitized copy, flattening options. No cloud required.
2. **Exchange review:** embedded comments/XFDF plus an offline review package or an authenticated hosted reviewer. Needs stable document/annotation IDs and conflict handling.
3. **Request approval/signature:** recipients, roles, intent, order, expiry, reminders, identity checks and completion evidence. An approval stamp alone is not a cryptographic signature.
4. **Manage project documents:** current revision, check-in/out or optimistic concurrency, access control, transmittals, retention, audit and rollback. This is more than attaching a file to a message.

The market evidence distinguishes shared document review, signing requests and project/version management rather than treating them as one checkbox. [3](https://helpx.adobe.com/acrobat/desktop/share-and-review-documents/share-documents/share-pdfs.html) [4](https://helpx.adobe.com/acrobat/desktop/e-sign-documents/request-e-signatures/send-for-signing.html) [3](https://support.bluebeam.com/revu/subscription/studio-and-bluebeam-cloud-comparison-guide.html) [1](https://www.docusign.com/products/electronic-signature/features)

### 6.2 Complete sharing backlog

The detailed specification is **H01–H20** in [the recommendation catalog](recommended-roadmap.md). In order:

1. Unified Share/Send intent selector.
2. Native email/share-sheet copy.
3. Selected-page/email-size export.
4. Share preflight: metadata, attachments, unresolved redaction, signatures, form data.
5. Offline PDF/XFDF review package with revision/hash manifest.
6. Recipient-specific links, expiry and revocation.
7. Viewer/commenter/approver/owner roles; domain-restricted invitations.
8. Accessible browser guest viewer.
9. Threaded comments, mentions, assignment and resolution.
10. Review dashboard, deadlines, reminders and digests.
11. Immutable versions and conflict-safe shared saving.
12. Tamper-evident activity/approval event log.
13. Real-time annotations and presence — **later**, after durable async review works.
14. Offline review queue and reconnect reconciliation.
15. Self-hosted/approved-region sharing providers.
16. Cloud-drive check-in/out and change notifications.
17. Revision-aware page/comment links and QR handoff.
18. Supplier upload/request-files portal.
19. Recipient watermark and download policy.
20. Retention, legal holds, deletion and evidence export.

### 6.3 Recommended architecture

**Phase A — no new service:** ShareIntent → copy/export policy → preflight → user preview → local artifact/manifest → OS email/share handoff. Keep the working original unchanged.

**Phase B — provider adapter:** connect to an approved existing store through a versioned adapter with least-privilege permissions. The current same-origin web host bridge is a foundation to extend, not a reason to claim there is no integration. Persist provider file ID and version token separately from local filename.

**Phase C — optional review service:** immutable document revisions/object hashes, transactional metadata, explicit tenant/project membership, server-side role enforcement, short-lived sessions/link authorization, append-only review events, background job queues and retention lifecycle. Store comment bodies and derivative render/text data under the same authorization policy as the PDF.

**Phase D — live collaboration:** synchronize annotations first. Do not attempt unrestricted simultaneous binary PDF editing as the first collaboration milestone. Establish durable event delivery, conflict resolution and offline replay before presence indicators.

**Minimum failure/security tests:** forward an invitation to the wrong person; revoke access while a tab is open; reuse an expired token; cross-tenant page/thumbnail/text fetch; replace a newer shared revision from an old client; retry upload after timeout; synchronize an offline edit to a superseded document; delete a document with cached derivatives; request a malicious URL; upload a decompression bomb. URL fetches/connectors need explicit SSRF protections; document-triggered actions are untrusted. [6](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html) [5](https://cheatsheetseries.owasp.org/cheatsheets/MCP_Security_Cheat_Sheet.html)

### 6.4 Promises we must not make

- Revoking a link prevents **future authorized retrieval**; it does not erase an already downloaded copy or screenshot.
- “Disable download”, PDF permission flags, flattening and watermarks are not strong DRM and do not guarantee confidentiality after disclosure.
- TLS/server-at-rest encryption is not automatically end-to-end encryption. True E2EE changes search, server rendering, malware scanning, recovery and key-sharing architecture; select a threat model before marketing it.
- An “opened” event does not prove a named person read or understood a document. Tracking is incomplete when clients block events or documents are forwarded/offline.
- Retention for signature evidence, ordinary temporary conversion files and backups may differ; disclose lifecycle by data type.
- An immutable local event file without protected signing keys/trusted time is not automatically an independently trustworthy audit trail.
- No silent upload to AI or a sharing provider. Service choice, data location, retention and costs must be visible before transmission.

---

## 7. Application-by-application investment plan

Every item below maps to the full catalog, where effort and acceptance criteria are supplied.

| Application | Basic / day-to-day improvements | Advanced additions | Future opportunities |
|---|---|---|---|
| **Desktop** | Reliability R01–R12; reading V01–V12; text E01/E02/E04; OCR O01–O05; page safety P03–P05; printer Q05/Q06; local sharing H01–H05 | Drawing register/overlays N01/N02; quantities N05–N07; accessible remediation A03/A05; advanced signatures D03/D04; batch B04–B06 | Evaluated local AI Z01–Z06; symbol search Z08; 3D/geospatial only if paid demand supports Z09/Z10 |
| **Web** | Recovery/save W01–W03; browser capabilities W02; accessible mobile review W08; host bridge W07; performance W09 | Authenticated review H06–H12; page links W10; explicit browser OCR W09/O09; host storage I01–I03 | Offline-installable PWA W04; controlled live annotations H13; local browser models subject to device limits |
| **CLI/MCP** | Stable contracts/errors B02; dry-run B03; bounded resources B07; permission profiles B09; coverage B01 | Folder watchers B04; portable actions B05; resume/idempotency B06; evidence B10 | Opt-in authenticated service API B08 and enterprise connectors I08/I10; no listener by default |
| **OS/storage integrations** | Correct current-copy handoff H02/I05; shell isolation I06; network-drive resilience I04; signed deployment T01–T03 | Cloud provider check-in/out I02/I03; admin presets I07; project DMS I09 | Selected ERP/no-code adapters I08/I10; prioritize actual installed customer systems |
| **Mobile/tablet — proposed** | First validate responsive web review and capture/transfer M03/M04/M07 | Offline packs M02; pen M05; inspections M06; device security M08 | Native apps M01 after usage/maintenance case; voice/transcription M10 and field context M09 |
| **Team service — proposed** | No launch before authorization/versioning/privacy gates H04/H07/H11/H20 and service policies | Review, approval, request-files, evidence, SSO and regional/self-host options | Real-time co-review, remote signing and semantic project search after an operational model is funded |

Do not split the product into separate “merge app”, “compress app”, and “sign app” unless users validate that packaging. Share one engine/document model and expose focused workflows on each surface. The current architecture already shares substantial engine/UI code; preserve that advantage.

---

## 8. Advanced functions worth adding

The complete detailed list is in the workbook/catalog; the highest-value clusters are:

- **Engineering/document control:** sheet register, discipline/revision metadata, alignment-aware overlays, slip sheets, markup migration, detail hyperlinks, calibrated multi-viewport quantities, RFIs/submittals, controlled transmittals and O&M/as-built binders (**N01–N12**).
- **Business extraction/conversion:** high-quality Word, table-to-XLSX/CSV, safe local Office import, region-based structured extraction and validated ERP handoff (**X01–X10, I10**).
- **Security and trust:** protected view, locale-aware PII suggestions, folder redaction, certificate encryption, managed policy, token signing and explicit profile evidence (**S01–S10, D01–D10**).
- **Accessibility remediation:** order/tags/content panels, accessible table editing, reviewed autotagging, named PDF/UA profiles and external validation (**A03–A09**).
- **Print and standards:** independent PDF/A validation, configurable preflight/fixups, PDF/X, output/separation/overprint previews, accurate mixed-tray/label printing and advanced colour controls (**Q01–Q10**).
- **High-volume automation:** resumable jobs, manifests, folder watches, schema-stable CLI/API, provider integrations and no-code workflows (**B01–B10, I01–I10**).

Dependencies matter: complete structured text/table extraction before promising robust spreadsheet output; stable identities/revision mapping before comment migration; immutable versions and authorization before live review; profile validation before compliance badges.

---

## 9. Future functions — useful only with evidence and guardrails

**Z01–Z12** specify the proposed future portfolio: local/BYO-provider cited Q&A, bilingual summaries/translation, contract obligations, assisted PII marking, accessibility suggestions, approved natural-language action plans, permission-aware semantic search, symbol/takeoff assistance, 3D/BIM links, geospatial workflows, handwriting recognition and verified closeout timelines.

Rank future work by saved correction/review time, not demo appeal. Begin with read-only assistance and source citations. A generated presentation, podcast or decorative image is lower priority than truthful document extraction, signability, current drawing revision and safe output.

PDFs, retrieved documents and tool outputs can contain malicious instructions. AI suggestions must not acquire authority to read arbitrary folders, call connectors, sign, send or redact. Enforce permissions outside the model, validate every tool argument, require action-specific approval and constrain data access. Keyword filtering alone is not a security boundary. [1](https://cheatsheetseries.owasp.org/cheatsheets/LLM_Prompt_Injection_Prevention_Cheat_Sheet.html) [5](https://cheatsheetseries.owasp.org/cheatsheets/MCP_Security_Cheat_Sheet.html)

For every AI capability, require: documented model/provider/location, opt-in and admin-disable, rights-cleared evaluation sets, language/complex-layout tests, source revision/page/region, confidence/abstention, human review, cost/resource limits and no training/data-retention claim without contractual evidence.

**Defer unless demand is demonstrated:** rich-media playback, broad 3D editing, obscure print-production formats, new account/billing infrastructure for offline users, a replacement for every DMS/ERP, unrestricted real-time content editing and automatic legal/compliance judgments.

---

## 10. Prioritization, dependencies and delivery sequence

### Priority and effort definitions

The **222 recommendations** comprise **38 P0, 98 P1, 71 P2 and 15 P3** items. These are feature/workstream candidates, not 222 ready-sized tickets.

- **P0:** safety/usability/release gate whenever the affected workflow/surface is offered. Some apply only if a future service is launched; they do not imply the current app already has an authorization flaw.
- **P1:** repeated everyday work or an important target-audience gap.
- **P2:** professional/segment capability with a strong dependency or narrower audience.
- **P3:** future/new-platform investment requiring demand validation.
- **S/M/L/XL:** relative scope only; XL includes new services/platforms and major fidelity work. No estimate in calendar days or agent-hours is claimed.

Suggested triage formula after interviews: **frequency × user impact × reach × evidence confidence**, then apply a hard safety/compliance gate and divide non-gating work by delivery/maintenance cost. Do not allow a scoring formula to rank a known data-loss fix below cosmetic work.

### First 15 workstreams to validate and scope

1. Capability-claim reconciliation and one authoritative support matrix — **R12/T09**.
2. Save/crash/conflict correctness, including network shares — **R01/R05/I04**.
3. Rendering/interoperability/performance corpus and budgets — **R02/R03/R07/R08**.
4. Redaction/sanitized-copy verification and restricted document actions — **S01/S02/S03**.
5. Accessible everyday keyboard/screen-reader/scale workflows — **A01/A02/A08**.
6. Real-device A4/A3/labels/mixed-size duplex acceptance — **Q05/Q06**.
7. Bilingual editing/search/copy and Arabic OCR feasibility — **E02/V11/O01**.
8. Search all open files/folders/comments and preserve reading state — **V01/V02/V08**.
9. Cancellable background work and resumable batch actions — **R04/O07/B06**.
10. Safe page assembly: field-name and tag/link conflicts — **P04/P05**.
11. Word quality, table-to-Excel/CSV and safe Office import — **X01/X02/X04**.
12. Unified safe-copy/email/offline-review sharing — **H01–H05**.
13. Drawing register/aligned compare/transmittal pilot — **N01/N02/N09**.
14. Signing/trust evidence clarity and release signing — **D01/D02/T01**.
15. Governed storage integration and asynchronous review pilot — **I01/I02/H06–H12**, only after service policy approval.

### Sequenced roadmap — gates, not promised dates

| Stage | Deliver | Exit gate |
|---|---|---|
| **0 — Validate** | Role interviews, task observation, rights-cleared corpus, capability reconciliation, baseline measurements | Top workflows and platform/language scope agreed; no unverified feature marketed as complete |
| **1 — Dependable daily product** | Relevant P0 safety/accessibility/release gates; targeted print/search/edit/OCR fixes | Production acceptance suite, packaged-app tests, known limits and rollback process approved |
| **2 — Office and engineering productivity** | Bilingual workflows, conversion/assembly, templates, local sharing, revision register and compare pilot | Users complete agreed real jobs with measurable improvement and acceptable output fidelity |
| **3 — Controlled teams** | Existing-storage adapters, async review, roles/versioning, approvals/transmittals, retention | Security/privacy/operations review plus representative external-party pilot; recovery/restore demonstrated |
| **4 — Field and intelligence** | Mobile/offline investment and evaluated AI/advanced AEC functions | Demonstrated demand, model/hardware evaluation, maintenance capacity and agreed unit economics |

Implementation decisions still needing an owner: hosted versus self-hosted service; initial storage/identity provider; mandatory signing jurisdictions/trust providers; native mobile investment; supported OCR languages; licensing of optional sidecars/models; and support/retention commitments.

---

## 11. What research remains before calling this a market-validated roadmap

### Qualitative evidence sampled

The public upstream open-issue sample includes printer discovery [#160](https://github.com/storytold/pdfcraft/issues/160), printing [#3032](https://github.com/storytold/pdfcraft/issues/3032), missing Chinese UI glyphs [#826](https://github.com/storytold/pdfcraft/issues/826), RTL spreads [#2339](https://github.com/storytold/pdfcraft/issues/2339), RTL bookmarks/comments [#3220](https://github.com/storytold/pdfcraft/issues/3220), custom label sizes [#2826](https://github.com/storytold/pdfcraft/issues/2826), browser OCR [#103](https://github.com/storytold/pdfcraft/issues/103), accented-Latin OCR [#524](https://github.com/storytold/pdfcraft/issues/524), reading-colour persistence [#3443](https://github.com/storytold/pdfcraft/issues/3443) and older-GPU startup [#4134](https://github.com/storytold/pdfcraft/issues/4134).

These were found through GitHub issue search sorted by comment activity. Titles/open state are signals, **not proof the same defect exists at this fork's baseline**; reports can be old, duplicated or already fixed by a port. Do not use comment counts as market demand or convert an issue title directly into a bug finding.

### Proposed validation programme

- Interview/observe **12–18 users** spanning document control, engineers/site staff, admin/HR, finance/procurement, legal and IT; include Arabic/English users and assistive-technology users.
- Obtain permission to analyze representative samples or build synthetic equivalents: tenders, A3/A1 drawings, mixed scans, invoices, bilingual forms, signed/encrypted files, large binders and unusual labels.
- Ask users to demonstrate their last ten real document jobs, the workaround used, failure cost, frequency and exchange destination. Avoid asking only “would you like feature X?”.
- Run the same task set in Linkco and the tools users already own. Compare completion, correctness, correction time, clicks, print output and recovery—not vendor feature lists alone.
- Validate a local/offline review package before funding a service; validate one storage/identity integration before adding five.
- Pilot proposed advanced functions with named owners and acceptance criteria; require evidence of reduced rework or measurable time saved.
- Revisit priorities when users contradict this report. Product decisions should not be locked to a static competitive checklist.

No market size, pricing elasticity, customer preference percentages, competitor OCR accuracy or guaranteed ROI is invented in this study.

---

## 12. Maintenance and reproducibility

The existing gap lists and workbook are generated from a fixed source commit. The recommendation input is maintained separately from product parity so proposals do not become false shipped claims.

```sh
# Standard-library CSV/Markdown generation and consistency checks:
python3 docs/research/scripts/build_product_inventory.py

# Optional workbook generation (openpyxl 3.1.5 in an isolated Python environment):
python3 docs/research/scripts/build_product_inventory.py --xlsx

# Research-package consistency tests (openpyxl enables the workbook test):
python3 -m unittest discover -s docs/research/scripts -p 'test_*.py' -v
```

The workbook is a contributor-original documentation asset registered in `ATTRIBUTION.toml`. Regeneration is byte-stable with openpyxl 3.1.5; when changing research inputs, re-verify its provenance, update its SHA-256 and regenerate attribution with `cargo xtask assets --write` before committing.

Validation for this research package: generator checks and five consistency tests passed, including workbook rows/filters/text cells, ID/source references, exact gap coverage and local Markdown links. All 247 registered asset hashes and licence-file paths were checked. Rust/Cargo tooling was unavailable locally during this documentation task; the Rust format/clippy/test and native acceptance suites were **not rerun locally**, and no product implementation was changed.

Inputs: pinned `parity/acrobat-features.toml`, `../scripts/recommendations_2026_10_25.psv`, and `sources.csv`. The generator validates IDs, counts, source references and baseline evidence paths. CSV cells starting with formula-trigger characters receive a protective apostrophe; XLSX cells are explicitly text. The dated snapshot must not silently change when product code evolves.

**Bottom line:** finish and prove the functions people already depend on, make the common jobs effortless, add safe exchange before live collaboration, and build contracting-specific revision/evidence workflows before low-value feature proliferation. The linked catalogs provide the complete scoped list needed to turn that strategy into a backlog.
