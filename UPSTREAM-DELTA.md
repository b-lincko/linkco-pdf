# Upstream delta — storytold/pdfcraft → b-lincko/linkco-pdf

Checked **2026-10-14** against `storytold/pdfcraft` `main` (`c1a41cc`), re-checked **2026-10-15**
against `9b6dab1`, and **2026-10-17** against `924c4da`. Sections 0.1–0.2 list what has landed
since and what we have taken.
This is the **functional** list: what a user or an agent can do upstream that they cannot do here.
Counts come from comparing command ids, automation tool ids, the engine's public API, crate lists,
UI panels and language catalogs — not from reading commit subjects.

| | |
|---|---|
| Our HEAD | `d21ec1b` (2026-10-15; the ports in §0.1 are already merged) |
| Closest upstream commit | `bbbe600` — *M10: Windows printing prints at actual size, off the UI thread* (2026-10-11) |
| Commits upstream since | **297** (291 non-merge) |
| Command ids | ours 182 → upstream **194** (**13 new**) |
| Automation tools | ours 93 → upstream **112** (**19 new**) |
| Crates | ours 30 → upstream 31 (**new: `crates/audit`**) |
| Right-hand panels | ours 10 → upstream 11 (**new: Tags**) |
| UI languages | ours 16 → upstream 21 (**new: el, kk, lt, sl, tr**) |
| Files changed upstream | 434 files, **+82,089 / −7,013**; 88 new files, none of them here |

## 0. Update — 17 Oct 2026: 45 commits since 9b6dab1

Upstream `main` moved `9b6dab1` -> **`924c4da`** (45 commits, 40 non-merge; 159 files,
**+8,587 / -875**) - a post-v0.8.0 batch of community PRs, nearly all dated 15-16 Oct.

**Printing (our area).**
- `980bd90` #1583: **4 x 6 in label paper and a Label layout** for shipping labels (`crates/print`
  +129, engine +14, the tool, the dialog). The only new print feature; the rest of upstream's print
  work is still the `craft-print` backend we are not taking.

**Fixes worth having.**
- `13e3154` fix(fonts): read the Annex D.2 alias codes as the ink they draw (+173/-20).
- `cccd55f` #1664: editing finds the pages of a damaged file that its readers find (+244/-13).
- `95a972a` #1562: preserve known unfiltered inline-image sample boundaries (+244/-1) - the same
  family as #1499, already queued.
- `41809cf` #1136/#1533: macOS Cmd+Q asks about unsaved changes (+65).
- `afa8e18` #1569: Wayland stops retaining Ctrl zoom for pixel-only wheel input (+114/-16).
- Forms: `e62d3db` #1588 (NCName field tags), `57c8de0` #1647 (XFDF text around XML comments),
  `53b9bc1` #1563 (carriage returns in CSV exports).
- `ab45a18` #1557: annotations stay readable under reading colours - only useful once #1270 lands.

**Performance.**
- `f65f733` #2436: faster sampled functions, image decodes across zoom levels and patch meshes;
  **vendored hayro/hayro-interpret patches 31-33** (+392/-235, 18 files under `vendor/`).
- `416c855` #1434: marker and link index scans deferred until first use (+247/-85).

**Larger features (a judgement call).**
- `02a4d81` #1452 keep selected form values through later edits (+322/-31); `d1bc200` #1539 grow
  text boxes and edit existing text; `7aac682` #1621 picture-menu actions; `10ad9b4` #1528 and
  `a790a04` #1486 added text selection and line breaks; `ae9fb42` #1549 edge scrolling and taking
  back a polygon's last point; `fcd7274` #1524 selectable list boxes; `cdf44d7` #1614 folding
  comment groups; `24f93e6` #1466 hide author-only hover previews; `3805be7` #1536 XDG
  configuration directory; `7461c29` #1448 `--automation-read-root`/`--automation-write-root` (we
  still pass one `--root` to the CLI and MCP).

**Not for us.** M12 accessibility (`b7976ef` #1496, `a6124bc` #1463, `afa5099` #1451), the redaction
audit (`9dfc91e` #2662), tests/CI/docs/translations, and `5fdd820` #1895 (validated signature
display and snapshot tab reuse: 38 files, +993, in UI we have diverged from). Upstream's own
duplicate-key clean-ups (#2414 zh-hant, #2419 parity) found nothing here: our catalogs and
`acrobat-features.toml` have no duplicate keys.

## 0.1 Update — 15 Oct 2026: what landed since the check above

Upstream `main` moved `c1a41cc` → **`d3a91c5`** (10 commits, 14–15 Oct). Two headline things:
**PdfCraft v0.8.0 was released**, and a new **external-oracle / synthetic-fixture test
infrastructure** landed. Nothing new for the *user* in the last 24 h beyond the release tag.

1. **v0.8.0 released** (tag `v0.8.0`, 2026-10-14 15:21 UTC): **77 PRs** since v0.7.1. The release
   commit is only a version bump (0.7.1 → 0.8.0); its content is sections 1–6 above **plus** these,
   which I had not listed before:
   - F1 + F2: **exact standard-14 metrics** and code-to-Unicode completeness
   - fix(render): Type 3 text ignores rendering modes other than invisible (hayro patch 27)
   - fix(edit): flattening form fields also drops the ones whose widgets are on no page
   - fix: allow links beneath highlight annotations
   - M7: keep a neighbouring line when replacing text (#1316)
   - Detect rectangular form fields drawn with closed line paths; fix single-click freehand drawing
   - Fill & Sign: no comment hover popup over placed text (#1504)
   - Issue #1058: an update keeps the desktop shortcut only if it is still there
   - Name the failing option and page count when a chained edit page is out of range (#1550)
   - M0: label local builds in `--version`, About and the package release
   - fix(xfdf): preserve quoted empty CSV values; Retire inherited resources;
     Lab/Bradford adaptation by LMS white-point ratios
   - M1: **a save that doesn't reopen is refused before it replaces the file** (#1698 — our `c1a41cc`)
   - fix(ui): a file version found before a save is not applied over it (#1802)
   - M10: sync automation saves before replacing the destination (#1804)
   - M3: **drag tabs to reorder and Ctrl+Tab to switch** (#1537, #1542 — our `5e12f5f`)
   - **Pick up craft-print's native Windows printing** (#1107, PR #1132)
   - F3: one documented font-substitution chain, reporting what it chose and why (PR #1622)
   - A PR template, with the review practice and external-PR bar written down

2. **New crate `crates/oracle`** — the external programs PdfCraft checks itself against (qpdf,
   `pdftoppm`, `pdftotext`), with one rule: **a missing oracle is a declared skip or a failure,
   never a silent pass** (`or_skip!`). Test-support layer, dev-dependency only, so the standalone
   bottom crates can use it too. Motivation: `cos/tests/encryption.rs::qpdf_agrees` had *never*
   executed on a runner while the README claimed saved output was "independently checked with qpdf".

3. **New crate `crates/testkit`** — **one** way to write a synthetic PDF fixture
   (`Pdf::new().obj("…").stream("…", b"…").build()`), replacing 55 files that hand-rolled the same
   serialiser 111 times. No workspace dependencies, so `cos`/`filters`/`crypt`/`geom` can use it
   without a cycle.

4. **New gate `cargo xtask render-oracle`** — compares page 1 of every corpus PDF against Poppler's
   `pdftoppm`. Score = share of pixels with no near match in the other renderer's 3×3 neighbourhood
   (forgives antialiasing and subpixel placement, not missing or wrong content).
   **First reading, 250 files at 72 dpi, tolerance 8: 229 compared, median 0.89 %, mean 2.16 %,
   p90 5.35 %, 25 files within 0.1 %.** It reports rather than gates.

5. **`crates/render/tests/jbig2_invariants.rs`** — the 96 `bitmap-*.pdf` conformance fixtures must
   all decode to the same picture. **Ours produces 1 distinct image (correct); Poppler produces 8**
   (blank pages for the halftone-skip and symbol fixtures, and TPGRON refinement mis-decoded).
   Conclusion recorded upstream: *Poppler is not a usable oracle for JBIG2*.

6. **CI now installs and requires the oracles** (`qpdf`, `poppler-utils`, plus
   `PDFCRAFT_ORACLES_REQUIRED=1` on Linux), so the qpdf-backed tests run on every CI run instead of
   skipping; `parity/acrobat-features.toml` updated to say so.

7. **Text-oracle fix**: a document that forbids copying is skipped instead of scoring us 0
   (bug1815476.pdf). **Oracle decoder**: fixed-size pixel chunks (#2342).

8. **`docs/gaps.md` gap 12 rewritten** with the first measured rendering numbers, how to read the
   score (the worst file of the 250, `bug1552113.pdf` at 84 %, was *Poppler's* defect), and what is
   still missing: Acrobat as the reference, form calculation and signature validation, an interop
   suite (pdf.js, MuPDF), and 15 of 250 files that can't be compared because the two renderers round
   the page box differently.

9. **In flight, not on `main` yet**: `fix/winansi-soft-hyphen` (14 Oct), `fix/redact-scanned-pdf`
   (14 Oct), `ci/direct-timeout-limits-20261015` (15 Oct).

**What it means for us.** The last day's work is test infrastructure, so it adds no user-facing
feature — but it changes behaviour worth copying: with `crates/oracle`, a missing `qpdf` on CI is a
**failure**, not a silent pass, and `crates/testkit` is a dev-only crate we could adopt cheaply the
next time we write a PDF fixture. The v0.8.0 changelog (77 PRs) is also the cleanest statement of
everything we are behind on.

## 0.2 Ported into this repo — 17 Oct 2026

Taken one task id per commit, each with upstream's tests. Our printing path (PowerShell spooler and
`render_print_sheets`) is **kept**: upstream moved theirs into a `craft-print` crate, and swapping it
needs real Windows verification, so only the print *fixes* were ported.

| Commit | Upstream | What it fixes or adds |
|---|---|---|
| `e8ca761` | `a4c53f7` (#1308/#1411) | Current page prints the page on screen, not the page labelled with its number |
| `cd31142` | `2796f3d` (#1335/#1376) | A poster scale that saturates the tile counts is refused, not overflowed |
| `8e76461` | `f036d38` (#599/#1123) | ARCH/ANSI/large-ISO paper presets, Match page at 100%, PDF scale readout |
| `11255c7` | `4812e49` (#1226) | CUPS printer instances get their driver options |
| `a915820` | `c0c40ac` (#981) | macOS: saves to SMB/FUSE volumes fall back to a plain fsync |
| `d21ec1b` | `87b448b` (#1339) | An incremental save with no objects writes a valid xref |
| `c287cb0` | `9c2fc4f` (#1849) | A6 in the Paper menu and `doc_print`, so our paper list matches upstream's |
| `8eb6efa` | `980bd90` (#1583) | 4 × 6 in label paper, `media=` for the sheet the pages were laid out on, and a Label layout that cuts a label to its printed area and turns it upright |
| `95bc12d` | `95a972a` (#1562) | Unfiltered inline images use their sample length: pixels that look like `EI` are no longer cut short |
| `04aedac` | `cccd55f` (#1664) | Editing a file whose page tree is lost addresses the pages the reader finds by scanning |
| `ffbd600` | `afa8e18` (#1569) | Wayland: a pixel-only wheel no longer keeps Ctrl held, so zooming stops sticking |
| `de54549` | `e62d3db` (#1588) | Exported XML field tags stay inside the NCName grammar |
| `f7ff940` | `57c8de0` (#1647) | XFDF field text survives XML comments and processing instructions |
| `96b94b1` | `53b9bc1` (#1563) | Form CSV export quotes carriage returns |

Still queued from the older batch: validate-before-replace (`c1a41cc`), #1630, #1306, #1512, #1477,
#1640, #1499, #1500; then the medium features #1243 (web printing), #1270 (reading colours), #463
(units), #1537 (tabs). From the 17 Oct batch: `02a4d81` (#1452), `d1bc200` (#1539), `7aac682`
(#1621), `10ad9b4` (#1528), `a790a04` (#1486), `ae9fb42` (#1549), `fcd7274` (#1524), `cdf44d7`
(#1614), `24f93e6` (#1466), `3805be7` (#1536), `7461c29` (#1448), `ab45a18` (#1557).

Three from the selected batch were **not** taken, each for its own reason:

- **`41809cf` (#1533, macOS Cmd+Q asks about unsaved changes).** Needs cargo: it adds a git
  dependency (`craft-quit` from `storytold/craft-libs` at `f58bc1a`, while we pin that repo at
  `5c874d51` for `craft-ui`), and its `Cargo.lock` entry can't be resolved or written by hand here.
- **`f65f733` (#2436, faster sampled functions, image decodes and patch meshes).** It patches
  `vendor/hayro-interpret`, which has diverged: upstream's `cache.rs` is 279 lines against our 161
  and its `x_object.rs` 1213 against our 984. Only `encode.rs` and `function/type0.rs` still match.
  Taking it means syncing our vendored hayro-interpret first — a job of its own.
- **`416c855` (#1434, defer marker and link scans).** Portable in principle, but it turns
  `Document::marks/added/links` from fields into lazy accessors across 14 files (engine, five UI
  files, four UI test files) and adds a `Scope::Links` refresh path. Left for a session with a build.

Deliberately **not** ported: the `craft-print` migration, camera/Enhance, PDF/UA auto-tagging and the
Tags panel, the batch CLI, `crates/oracle` + `crates/testkit`, `crates/audit`, PowerPoint export,
the M12 accessibility work (`b7976ef`, `a6124bc`, `afa5099`), the redaction audit (`9dfc91e`) and
`5fdd820` (#1895, UI we have diverged from).

## 1. New end-user functions (command ids we don't have)

| Command | What the user gets |
|---|---|
| `a11y.autotag` | **Auto-tag a text-only PDF** in one action, with a reviewable preview of the proposed structure (#1440, #1383) |
| `a11y.tags` | **Tags panel**: list structure tags, move them in reading order, edit tag properties (#1216, #1482, #1229) |
| `create.multiple_separate` | **Create one PDF per input file** from Combine/Add files (instead of one merged file) |
| `export.pptx` | **Export to PowerPoint** (#1137/#1162) |
| `ocr.enhance_camera` | **Enhance camera image**: find the page's edges in a photo, warp it straight, clean it up (lighting/contrast/specks), choose colour / greyscale / black-and-white (#1027, #1038) |
| `protect.unlock` | **Remove a password** / unlock a protected document you can open |
| `view.continuous` | **Continuous scrolling** as its own checkbox, independent of the base page layout (#752/#896) |
| `view.next_tab`, `view.previous_tab` | **Cycle through document tabs** (`5e12f5f`) |
| `help.website`, `help.github`, `help.discord`, `help.app_page` | New **Help ▸** destinations (app page, GitHub, Discord) |

## 2. New automation / CLI capabilities

New tools (callable through `pdfcraft-cli run` and MCP):

| Tool | Purpose |
|---|---|
| `doc_batch` | **Process files and folders**: convert, OCR and optimize over whole trees (`recursive`, `directories`), numbered outputs, per-file results (#1605) |
| `page_enhance_camera` | Enhance camera image, headless (corners or automatic edge detection, mode + level) |
| `print_readiness` | **Print readiness check**: effective image resolution where each image is drawn, page boxes and bleed, output intents, non-embedded fonts, PDF/A-2b structural result (verification only) |
| `doc_audit_redactions` | **Audit redactions** for content that is only visually hidden (paint-order and colour aware, fail-closed) — backed by the new `crates/audit` crate (#110) |
| `accessibility_autotag_preview` / `accessibility_autotag_apply` | Preview and apply automatic tagging |
| `accessibility_tags`, `accessibility_move_tag`, `accessibility_set_tag_property` | Read and edit the structure tree headlessly |
| `accessibility_review_record`, `…_load`, `…_save`, `…_html` | Accessibility review record: save human review decisions with the document, export a before/after HTML summary (#1204, #1234) |
| `view_reading_colors` | **Reading colours** (Acrobat's Replace Document Colors): off, black-on-white, white-on-black, yellow/ green-on-black or custom (#897/#1270) |
| `text_fonts`, `text_font_import`, `text_font_files` | List and load the document's fonts while editing text |
| `text_edit_layout` | Measure a text draft (layout feedback for text editing) |
| `image_hit` | Hit-test page artwork: which images/Forms contain a point, respecting clipping and holes |
| `field_conflicts` (parameter) | Combine: choose `rename` or `merge` when field names collide |

CLI: new **`batch`** subcommand (`apps/pdfcraft-cli/src/batch.rs`) and `check` tests; upstream also
**dropped** the CLI `preview` subcommand (we still ship it).

## 3. New engine capabilities (public API we don't have)

From `crates/engine/src/lib.rs` (109 → 134 public functions):

- **Protection**: `can_unlock`, `unlock`, `allows_copying`
- **Opening**: `prepare_open`, `commit_open`, `open_context`, `reload` (background open + reload when the file changes on disk, #459)
- **Combine**: `combine_each` (separate outputs), `combine_field_conflicts`, `combine_grouped_reporting`, `combine_unlocked_reporting`
- **Accessibility**: `tags`, `undo_label`, `redo_label`
- **Reading**: `reading_colors`, `set_reading_colors`
- **Fonts / text**: `refresh_fonts`, `typed_signature_graphic`, `xfa_scripts_off`
- **Audit**: `audit_redactions`
- **Misc**: `saved_bytes`, `measurement_scale_or` (units preference, #432/#463), `as_pdf`

New crate: **`crates/audit`** (redaction audit engine).

## 4. New UI surfaces and settings

- **Tags** right-hand panel (structure tree, tag properties, move in reading order).
- **Help ▸** app page / GitHub / Discord.
- **Units preference** (mm, cm, in, pt, defaulting to the system region) (#432/#463).
- **Reading colours** and **continuous scrolling** as independent settings.
- **Tab context menu** (#1266), **tab reorder and cycle**.
- **Wayland**: GNOME-style title bar drawn in-app (#1530) and window controls; Linux GPU choice (#1088), server DPI (#1150).
- **Animation effects** follow Windows' setting, with a preference (#1235).
- **File watching**: reload a document when its file changes on disk; opening moved off the UI thread (`open_worker.rs`).
- **Camera UI** (Scan & OCR ▸ Enhance), **page-range selection** in Organize Pages (#1092/#1174),
  **"Other colour…" picker with recent custom colours** in comments (#1193),
  **IME composition preserved** in command search (#1252), **mouse-wheel scrolling while selecting text** (#1514).

## 5. Languages

Upstream adds **Lithuanian** (#1401), **Turkish** (#1057), **Kazakh** (#955), **Greek** and **Slovenian**
catalogs, plus completed Czech (#1066) and refined Greek (#1262); the i18n diff alone is
**+18,161 / −805** across 22 files. They also changed how lower layers hand sentences over for
translation (#478), so our catalogs would need that too.

## 6. User-visible behaviour we still have the old (buggy) version of

Print (our area):
- **Current page prints the page labelled with that number**, not the page being viewed (#1308/#1411) — our
  `PrintDraft::settings` still passed `current_page+1` as a label-parsed range — **ported 15 Oct** (`e8ca761`).
- Only **6 paper presets**; upstream has **18** (ARCH A–E1, ANSI C–E, A0–A2) (#599/#1123). **Ported 15 Oct** (`8e76461`).
- No **CUPS printer instances** (`queue/instance` defaults, #1226). **Ported 15 Oct** (`11255c7`).
- No **printing from the web build** (#1243) — our gap 4.
- Posters: upstream refuses huge posters at the sheet limit (#1335/#1376). **Ported 15 Oct** (`cd31142`).

Correctness / safety:
- **Save**: upstream validates the produced PDF before replacing the file (`c1a41cc`), falls back to a plain
  fsync where a volume refuses the full flush (#981/#1626), and writes a valid xref on an incremental save
  with no objects (#1339/#1424).
- **Security**: 10 audit findings in redaction, signature validation and untrusted-input handling (`b642e32`),
  3 more in redaction/signature validation (#1631), sanitised attachment save filenames (#1114/#1630),
  `ui.click`/`ui.drag` refuse overflowing coordinates (#1306), rotation normalisation before adding (#1512).
- **Text**: TrueType font without an encoding read as WinAnsi (#1477), doubled narrow letters in condensed text
  (#1467), Arabic text order in copy/search/edit (#1369), Arabic ActualText ligatures (#1473), Hebrew and
  Devanagari/Indic editing (#521, #1033, #1380), shared Type 3 glyph resources on extraction (#1500).
- **Rendering / images**: filtered soft masks keep thin strokes (#1649), CMYK JPEGs keep their colours on export
  (#1640), packed TIFF-predictor row boundaries (#1499), Lab `/WhitePoint` (#1203).
- **Search / comments**: bounded page-text cache with partial results (#1492), undrawn comments and accurate
  search hits kept (#1442), a skipped accessibility rule stays with its document (#1301).
- **Forms / XFA**: positioned objects by `anchorType` (#1637), signing a field whose `/V` is null (#1328),
  refuse signing a field whose `/V` is not a signature (#1402), one script thread per event (#1355).
- **Shell**: a closed right panel stays closed when opening a document (#1590), Combine flags the same
  converted file added twice (#1303), absolute GoToR paths ask first (#1364), Windows launcher paths with
  spaces (#1456), macOS starts on wgpu on Intel Metal (#1627).

## 7. Where we diverge — things upstream *removed* that we still have

- **Windows printing.** Upstream replaced the whole self-rendered path with the shared
  **`craft-print`** crate (`git = "https://github.com/storytold/craft-libs"`, rev `f03ec27ab2c405969428512178f746183cd55ca0`).
  Their `crates/print/src/spool.rs` is now a thin adapter; `windows_print.ps1` is **gone**, and so are
  `render_print_sheets`, `render_imposed_sheets`, `print_max_dpi` and `allows_high_quality_printing`,
  which **we still have**. (We already depend on `craft-ui` from the same public repo, so `craft-print` is
  reachable for us.) Practical effect: our Print dialog renders sheets itself through PowerShell; theirs hands
  the print-ready PDF to a native spooler.
- **CLI `preview`** subcommand: removed upstream, still in ours.
- **Our own addition**: the **“Print both sides”** button in the print preview (`566a3a1`) — upstream has
  nothing like it (`grep -c "Print both sides"` on their `print_ui.rs` → 0).
- **Linkco identity**: `build.py`, `installer.py`, `packaging/installer.iss`, `RELEASE_NOTES.md`, brand
  assets, About/contributor credits.

## 8. Conflicts to expect when syncing

- `crates/ui-egui/src/i18n/*` (22 files, +18,161/−805).
- `crates/ui-egui/src/print_ui.rs` (+470/−598 against ours) and `crates/print/*` (rewritten; `spool.rs`
  became an adapter).
- Branding-touched files: `crates/ui-egui/src/{lib,chrome,home,dialogs,canvas,js_ui,ocr_ui,portable,recovery}.rs`,
  `crates/engine/src/{lib,commands,links}.rs`, `crates/sign/*`, `apps/pdfcraft*`.
- `vendor/` (31 files changed upstream: hayro patches — Turkish Ğ/İ/Ş glyphs #1409, Lab/Bradford, gradients).

## 9. How this was measured (and how to refresh it)

```bash
git fetch --filter=blob:none --no-tags https://github.com/storytold/pdfcraft.git \
  '+refs/heads/*:refs/remotes/upstream/*'
git diff --stat HEAD upstream/main -- crates apps xtask vendor
# functional counts
git show upstream/main:crates/engine/src/commands.rs | grep -oE '"[a-z][a-z0-9]+\.[a-z0-9._-]+"' | sort -u
git show upstream/main:crates/automation/src/tools.rs | grep -oE 't\("[a-z_]+", "[^"]+"'
git show upstream/main:crates/engine/src/lib.rs | grep -oE 'pub fn [a-z_0-9]+' | sort -u
```

**Caveats.** Upstream history is non-linear and our repo is one squashed commit with Linkco branding, so
the base (`bbbe600`) is an estimate from best tree match (1113 of 1238 common paths byte-identical) — verify
per item before backporting. We also carry a little content *newer* than that base (parts of the M10
Windows-printing work besides the ones listed in §7), so "missing" means "not in our tree", not "safe to copy".
