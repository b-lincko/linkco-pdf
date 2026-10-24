# Upstream changes our repo does not have

Source: [storytold/pdfcraft](https://github.com/storytold/pdfcraft).

| | |
|---|---|
| Window measured | `88dad98` → `86730ed1` |
| Dates | 2026-10-21 22:05 → 2026-10-24 02:05 (~2 days) |
| Commits (non-merge) | **734** |
| Files changed | 511, **+76,140 / −7,425** |
| Version | 0.11.0 → **0.13.0** (two releases: `39844f0` 0.12.0, `53199fb` 0.13.0) |
| Our head when compared | `abf30c3` |

**How to read this.** Sections A–B are *verified* — each item was checked against our tree and is
absent. Sections C–E are the commit window: every commit in it is upstream work we have not taken,
but individual commits were not each diffed against our tree, so a few may duplicate something we
already carry independently.

## A. Structural gaps (verified missing)

**Crates: upstream 33, ours 30 — three we do not have**

| Crate | What upstream's own docs say it is |
|---|---|
| `crates/audit` | **audit a document for *faux redactions*** — opaque rectangles painted over text that is still extractable underneath, plus redaction annotations marked but never applied. A properly applied redaction removes the text, so it never flags |
| `crates/oracle` | **external oracle processes** (`qpdf` and friends) that PdfCraft checks its own output against. The point is the policy: a missing oracle is a declared skip or a failure, **never a silent pass** — upstream found its owner-password tests had never actually executed on CI while the README claimed qpdf checked them |
| `crates/testkit` | **one dependency-free way to write synthetic PDF fixtures**, replacing 55 files that had hand-rolled the same serialiser (number objects, record offsets, emit an xref table and trailer) 111 times over |

**Languages: upstream 33, ours 16 — 17 missing.** Ours: ar, bg, cs, de, es, fr, hu, it, ja, pl,
pt-br, ru, te, uk, zh-hans, zh-hant.

| Missing | | |
|---|---|---|
| ca (Catalan) | ckb (Sorani Kurdish) | el (Greek) |
| fi (Finnish) | hi (Hindi) | id (Indonesian) |
| kk (Kazakh) | ko (Korean) | lo (Lao) |
| lt (Lithuanian) | nb (Norwegian Bokmål) | ro (Romanian) |
| sl (Slovenian) | sq (Albanian) | th (Thai) |
| tr (Turkish) | vi (Vietnamese) |  |

**New source modules — all ten confirmed absent from our tree**

| File | What it adds |
|---|---|
| `crates/edit/src/text_session.rs` | **a text editing session for one paragraph (#2512)** — opens the page once, lays every draft out from that reading on a copy of the document as it was opened (so no draft sees another's output), and commits into the document it was opened on, refusing if that document changed since. This is the infrastructure our deferred text-editing work has been missing |
| `vendor/hayro-cmap/` | **new vendored crate** — CMap tables for CJK text, brotli-compressed assets |
| `crates/edit/src/background.rs` | a page's background colour: what Edit a PDF ▸ Background sets, and what producers emit, for exports that keep a page colour (#2992) |
| `crates/cos/src/budget.rs` | resource budgets when parsing untrusted files |
| `crates/automation/src/layers.rs` | layers automation API |
| `crates/engine/src/stamp_preview.rs` | stamp previews |
| `crates/fonts/src/cjk_scripts.rs` | CJK script detection |
| `crates/ui-egui/src/macos_association.rs` | macOS file-type association |
| `docs/form-performance.md` | form performance notes |
| `docs/page-display.md` | page display modes |

## B. New commands (17, verified absent)

Diffed the command registry against `crates/ui-egui/src/commands.rs`: upstream 154 ids, ours 137.

- `a11y.autotag`
- `a11y.autotag_fields`
- `a11y.find_untagged`
- `a11y.tags`
- `create.multiple_separate`
- `edit.copy`
- `export.pptx`
- `file.word_count`
- `form.calculation_order`
- `ocr.enhance_camera`
- `protect.unlock`
- `view.continuous`
- `view.next_tab`
- `view.next_view`
- `view.page_gaps`
- `view.previous_tab`
- `view.previous_view`

## C. Feature work — 181 milestone commits

These carry the Acrobat-parity milestones (M1–M14; there is no M13 in this window).

### M1 — 1 commits

- `34e4aac2` keep roadmap status blockquotes valid  
  <sub>docs, +2</sub>

### M2 — 9 commits

- `70704ff2` text extraction finishes a column before the next one across a paragraph gap (#3658)  
  <sub>render, +264</sub>
- `b67a8bf2` a post table version with swapped halves doesn't make a font unusable  
  <sub>render, +66</sub>
- `1451325a` preserve word spaces around TOC leaders  
  <sub>render, +118</sub>
- `ebc7b6d8` text extraction keeps doubled letters in text condensed to 20 Tz and below  
  <sub>render, +49</sub>
- `b88c8d79` the two Identity-H ToUnicode rules point at each other  
  <sub>fonts, +7</sub>
- `7f9d86c9` doc comments for the Identity-H ToUnicode paths  
  <sub>vendor:hayro-interpret, +19</sub>
- `978c77c5` the Identity-H ToUnicode map leaves out DEL and C1 controls  
  <sub>fonts, +24</sub>
- `96db0e3b` Type 0 fonts with /ToUnicode /Identity-H read codes as Unicode  
  <sub>fonts, +115</sub>
- `ff5a2fc9` retain clipping text in filtered extraction  
  <sub>render, +40</sub>

### M3 — 15 commits

- `ac41758c` a Launch link that names a PDF opens it in PdfCraft (ISO 32000-2 §12.6.4.6)  
  <sub>render, +174</sub>
- `1f78e2f3` resize Pages inspector thumbnails  
  <sub>ui-egui, +351</sub>
- `93945aea` pin the reported OS in the panel snapshot harness  
  <sub>ui-egui, +11</sub>
- `14471d76` panel snapshots run from one exhaustive setup  
  <sub>ui-egui, +214</sub>
- `b0e75150` snapshot tests of every side panel  
  <sub>ui-egui, +458</sub>
- `941fbe7c` choose overlapping comment hover by annotation index  
  <sub>ui-egui, +59</sub>
- `929455ad` fix multiline annotation hover bounds and paint order  
  <sub>ui-egui, +186</sub>
- `604529e9` let Escape close an open menu before it clears the selection  
  <sub>ui-egui, +26</sub>
- `f6255c0e` numbered glyph names read as nothing again, not as a StandardEncoding guess (#3169, #3170)  
  <sub>vendor:hayro-interpret, +63</sub>
- `3fff309d` a link followed in full screen cancels the click's pending page turn  
  <sub>ui-egui, +39</sub>
- `14e17ece` in full screen a double-click selects a word, and full screen selects the Select tool  
  <sub>ui-egui, +104</sub>
- `1e2e7cc7` full screen shows one fitted page at a time, as Acrobat's Full Screen Mode  
  <sub>ui-egui, +235</sub>
- `0b731453` extract text from glyph names outside the Adobe Glyph List (#3169, #3170)  
  <sub>vendor:hayro-interpret, +121</sub>
- `92990b2e` layer_set shows or hides a layer, and page_render draws with the document's view settings  
  <sub>automation, +166</sub>
- `f976adeb` the "Save changes?" prompt opens on Save and moves between buttons by keyboard  
  <sub>ui-egui, +221</sub>

### M4 — 8 commits

- `e6b97925` bookmarks show and set their title colour and style (ISO 32000-2 Tables 151, 152)  
  <sub>ui-egui, +563</sub>
- `946a77cf` a new bookmark goes to where the reader is  
  <sub>ui-egui, +600</sub>
- `37454d9c` document indirect initial-view boolean handling  
  <sub>docs, +5</sub>
- `4bcae6dd` record indirect initial-view preference coverage  
  <sub>docs, +5</sub>
- `3d32ffd2` resolve indirect initial-view boolean preferences  
  <sub>organize, +49</sub>
- `33268824` use fixed-size slices in named destination regressions  
  <sub>automation, +4</sub>
- `2448074d` assert redo survives destination-budget refusal  
  <sub>automation, +8</sub>
- `031532b8` bound named-destination pruning during page deletion  
  <sub>organize, +269</sub>

### M5 — 14 commits

- `20a395c0` sticky notes draw the icon they name (ISO 32000-2 Table 175)  
  <sub>annot, +209</sub>
- `8af35de4` stamp_custom places a picture fitted inside a rect (#4077)  
  <sub>automation, +166</sub>
- `6411cc80` a Replace text comment is named Replace text, not Strikethrough  
  <sub>ui-egui, +11</sub>
- `4cd64681` drag a line's ends (and a polygon's corners) to change its length and angle  
  <sub>ui-egui, +400</sub>
- `ad453129` doc comments for the functions around this change  
  <sub>forms, +6</sub>
- `465bd0a7` doc comments for the functions this change touches  
  <sub>forms, +8</sub>
- `0ca7fb9b` a button caption's characters its font's encoding can't hold are reported  
  <sub>forms, +22</sub>
- `24a45db0` leave measuring form text in its own font to #2900  
  <sub>forms, +12</sub>
- `c0250b59` form text is measured in its own font, not always Helvetica  
  <sub>forms, +10</sub>
- `6b4e396a` a push button's caption uses the font, size and color of its /DA  
  <sub>forms, +73</sub>
- `99c78557` a stamp grows and shrinks with its handles before release  
  <sub>ui-egui, +468</sub>
- `62cde99a` reveal sidebar comments at the current zoom  
  <sub>ui-egui, +68</sub>
- `b97e5919` year-first slash dates (yyyy/mm/dd, yyyy/m/d) for Fill & Sign and headers  
  <sub>engine, +16</sub>
- `4f3f2fe1` batch Comments panel thread and page lookups  
  <sub>ui-egui, +219</sub>

### M6 — 14 commits

- `ba012449` calculations read their notation only from a comment, and a regex after a condition as a regex  
  <sub>forms, +218</sub>
- `8f292c66` scripts see xfa_version, and xfa_installed in XFA forms  
  <sub>js, +73</sub>
- `81c08be3` a field without /TU shows no tooltip of its own  
  <sub>ui-egui, +3</sub>
- `33150203` test the read-only and blank-/TU tooltip cases  
  <sub>ui-egui, +27</sub>
- `7ddb6226` hovering a form field shows its tooltip  
  <sub>ui-egui, +36</sub>
- `526d87d8` formattedValue formats only a value of the ISO date's own shape  
  <sub>xfa, +11</sub>
- `21b62671` a display picture with month or weekday names leaves formattedValue raw  
  <sub>xfa, +17</sub>
- `6a8ea287` doc comments on the functions the formattedValue change touches  
  <sub>xfa, +45</sub>
- `7176ebde` XFA formattedValue shows a date in the field's display picture  
  <sub>xfa, +174</sub>
- `39d0b953` treat imported MaxLen zero as unlimited  
  <sub>docs, +144</sub>
- `0ba01a05` optimize AcroForm JavaScript snapshots and diffs  
  <sub>engine, +609</sub>
- `7504799e` align secured-form highlight tests after rebase  
  <sub>docs, +8</sub>
- `2556a3ac` the parity notes say button labels follow their /DA  
  <sub>parity, +4</sub>
- `bf2eb470` detected form fields get no border or fill  
  <sub>engine, +43</sub>

### M7 — 26 commits

- `e3f86940` redaction audit skips covered-text assembly when copying is withheld  
  <sub>audit, +34</sub>
- `458c20d3` opening a paragraph leaves the page as it is until it changes  
  <sub>ui-egui, +231</sub>
- `af333266` sample a moved picture from where the fresh raster has it  
  <sub>ui-egui, +155</sub>
- `bef4e28a` a dragged page image leaves no outline where it was  
  <sub>ui-egui, +13</sub>
- `6495c08a` a text editing session for one paragraph  
  <sub>edit, +135</sub>
- `f632c1b0` a resized page image stays blanked where the raster still has it  
  <sub>ui-egui, +44</sub>
- `764439bb` hold an applied paragraph's look until every tile is drawn, and not after a turn  
  <sub>ui-egui, +25</sub>
- `08513ad9` keep a paragraph open with its typing when its edit fails  
  <sub>ui-egui, +64</sub>
- `1f335a2b` an added picture resized by a corner shows only at its new size  
  <sub>ui-egui, +107</sub>
- `7a7d39f0` keep a moved or resized page image in place until its page is drawn again  
  <sub>ui-egui, +146</sub>
- `f7fc2094` keep an applied paragraph's look until its page is drawn again  
  <sub>ui-egui, +125</sub>
- `68c85bce` open the paragraph editor without a fade-in  
  <sub>ui-egui, +83</sub>
- `475f375f` keep a word on its line when its spacing differs by less than a thousandth of an em  
  <sub>edit, +33</sub>
- `6beecdac` Header & footer dialog: Shrink the page content checkbox  
  <sub>ui-egui, +102</sub>
- `5bf48e0f` doc_header_footer takes shrink  
  <sub>automation, +41</sub>
- `01149dea` header and footer can shrink the page content out of their way  
  <sub>edit, +166</sub>
- `e9c53ea7` keep a behind watermark under one on top on an empty page  
  <sub>edit, +37</sub>
- `5beaf88a` paint a new background over older ones  
  <sub>edit, +51</sub>
- `3909bebe` a paragraph number keeps naming the paragraph text_paragraphs listed  
  <sub>automation, +303</sub>
- `9194ec45` the paragraph editor draws on its page only  
  <sub>ui-egui, +121</sub>
- `2d974867` keep roadmap status blockquotes valid  
  <sub>docs, +2</sub>
- `66981f1e` normalize added-image rotations before addition  
  <sub>automation, +119</sub>
- `a3cbb855` let a list marker open an item and a hanging indent continue it  
  <sub>edit, +86</sub>
- `0871543f` rotate an existing page image by any angle  
  <sub>ui-egui, +202</sub>
- `e0ef0983` drop the fallback fonts a text edit leaves unused  
  <sub>edit, +177</sub>
- `2109c678` show a shaped run in one text object instead of one per glyph  
  <sub>edit, +121</sub>

### M8 — 4 commits

- `fd2e13f3` XFA instance changes reserve their effect before changing the snapshot  
  <sub>js, +63</sub>
- `e9fc4cad` applying redactions drops structure-tree references to the comments and fields it removes  
  <sub>redact, +118</sub>
- `1c338b3e` a phone number takes one slash, right after its area code  
  <sub>redact, +8</sub>
- `73aea6e0` phone pattern finds slashed and labelled unseparated numbers  
  <sub>redact, +115</sub>

### M9 — 10 commits

- `f46268fb` visible signatures read upright on rotated pages  
  <sub>sign, +190</sub>
- `1b247a48` sign_list lists a hidden or NoView signature field as not visible  
  <sub>sign, +27</sub>
- `4ac31b3f` stop the side-by-side frame as soon as either half has closed  
  <sub>ui-egui, +6</sub>
- `f945eff6` closing a half of a side-by-side comparison keeps the app running  
  <sub>ui-egui, +47</sub>
- `b9414025` require eContentType and at most one certificates and CRLs field  
  <sub>sign, +14</sub>
- `f96e440a` check the bundle's SignedData shape and refuse trailing bytes  
  <sub>sign, +33</sub>
- `ddff6b64` refuse an unclosed PEM block; review follow-ups  
  <sub>sign, +11</sub>
- `0a56784d` read .p7b/.p7c certificate bundles for trust import  
  <sub>sign, +159</sub>
- `0fe2c070` read DocMDP permissions from the signed revision  
  <sub>sign, +58</sub>
- `4a35a1cb` rewriting a certified form's XFA data is a form fill  
  <sub>sign, +90</sub>

### M10 — 13 commits

- `4825b67d` odd and even pages are page numbers, as Acrobat counts them (#3984)  
  <sub>print, +50</sub>
- `09668ce8` a swapped post 2.0 table's glyph names serve /Differences  
  <sub>vendor:hayro-interpret, +33</sub>
- `d5002807` a turned fill or clip covers the page only if its corners do  
  <sub>edit, +92</sub>
- `ca0e0b08` a fill whose second rectangle cuts a hole is no page background  
  <sub>edit, +30</sub>
- `8707e87a` give the #2806 export tests' pages a background after rebasing  
  <sub>export, +5</sub>
- `d85d49a4` Export to Word keeps the page background colour  
  <sub>edit, +328</sub>
- `49fa1194` glyph names with lowercase hex digits map to nothing  
  <sub>render, +31</sub>
- `d0051fe6` a visibility expression naming an object that isn't a listed group is malformed  
  <sub>vendor:hayro-interpret, +33</sub>
- `ecb990c8` glyph names: a uni part with a surrogate group maps to nothing; numbered names read under a suffix  
  <sub>vendor:hayro-interpret, +29</sub>
- `36cfc856` dynamicRender is read only from markup whose closing tags match  
  <sub>render, +19</sub>
- `428cbc08` doc_print takes duplex "printer", leaving two-sided to the printer  
  <sub>automation, +54</sub>
- `3fbfa4e2` Two-sided starts at Printer default, so the printer's own duplex applies  
  <sub>ui-egui, +113</sub>
- `089acf71` keep dotted-leader contents entries together  
  <sub>edit, +114</sub>

### M11 — 3 commits

- `6765ae0b` Convert to PDF/A needs the owner's permission on an encrypted document (#3921)  
  <sub>engine, +44</sub>
- `3086ee7a` reject hostile optimizer image dimensions  
  <sub>optimize, +75</sub>
- `26051871` remove PDF/A or PDF/X identification (optimize.remove-standards-id)  
  <sub>organize, +160</sub>

### M12 — 41 commits

- `c0a513ee` scripts read a date in its data picture as the ISO date  
  <sub>xfa, +62</sub>
- `c7505930` accept Identity CJK Unicode mappings in accessibility checks  
  <sub>a11y, +268</sub>
- `572e01a7` the wrapping test checks the text stays within its room  
  <sub>xfa, +6</sub>
- `f9daeeb1` doc comments for the functions around this change  
  <sub>xfa, +24</sub>
- `15eef03e` doc comments for the functions around this change  
  <sub>xfa, +28</sub>
- `ee916731` doc comments for the functions around this change  
  <sub>xfa, +8</sub>
- `ec14687c` a draw without text or a width keeps the width it has room for  
  <sub>xfa, +17</sub>
- `a3b94202` doc comments, and words keeps its own  
  <sub>xfa, +5</sub>
- `5cd7e816` doc comments for the functions this change touches  
  <sub>xfa, +20</sub>
- `24b55275` doc comments for the functions this change touches  
  <sub>xfa, +25</sub>
- `15ab8c7a` doc comments for the functions this change touches  
  <sub>xfa, +28</sub>
- `35e04743` showing an XFA subform leaves its unnamed hidden objects hidden  
  <sub>xfa, +73</sub>
- `37f43e94` an XFA button's box is its whole field  
  <sub>xfa, +17</sub>
- `da38e0da` an XFA button without a height is as tall as its caption  
  <sub>xfa, +26</sub>
- `32bb14b9` an XFA label or button without a width is as wide as its text  
  <sub>xfa, +87</sub>
- `6dc8ad93` spaces alone stay on an XFA text line  
  <sub>xfa, +11</sub>
- `dd782bca` a non-interactive XFA field is measured at its text's horizontal scale  
  <sub>xfa, +45</sub>
- `b2f40e04` a space at the end of a line doesn't count when fitting XFA text  
  <sub>xfa, +71</sub>
- `162a8916` XFA text honors its horizontal font scale  
  <sub>xfa, +128</sub>
- `42e63703` full screen in two-page view fits the spread it shows  
  <sub>ui-egui, +74</sub>
- `3ed0dff8` full screen survives a reload and starts after the document's own initial view  
  <sub>ui-egui, +54</sub>
- `8f0557d2` an XFA script that fails partway keeps what it changed before the error  
  <sub>engine, +72</sub>
- `097c452c` a date its data picture can't hold stays ISO in the data  
  <sub>xfa, +25</sub>
- `38d3a6c8` each XFA date field uses its own data picture, and only a whole-date one  
  <sub>xfa, +222</sub>
- `d16c94a7` follow rewritten figure stream in engine regression  
  <sub>engine, +7</sub>
- `93f90ef7` XFA dates are written and read in the field's data picture  
  <sub>xfa, +164</sub>
- `e3d095f6` isolate decorative edits of shared content streams  
  <sub>a11y, +104</sub>
- `9e0937c3` the Visual filter only hides shading, since visual differences are never listed: say so  
  <sub>ui-egui, +14</sub>
- `9f536c9e` Compare panel filters text changes and visual differences (gap 21)  
  <sub>ui-egui, +185</sub>
- `4119359c` retain accepted whitespace when structured XFA data cannot sync  
  <sub>engine, +54</sub>
- `d6f872b6` avoid white flashes while reading pages render  
  <sub>ui-egui, +69</sub>
- `6425df50` reconcile current accessibility and XFA roadmap status  
  <sub>docs, +3</sub>
- `02fe96c5` reconcile current accessibility and XFA roadmap status  
  <sub>docs, +3</sub>
- `d7437606` reconcile current accessibility and XFA roadmap status  
  <sub>docs, +3</sub>
- `1946bcda` keep roadmap status blockquotes valid  
  <sub>docs, +2</sub>
- `607354ff` keep roadmap status blockquotes valid  
  <sub>docs, +2</sub>
- `bf542a76` keep roadmap status blockquotes valid  
  <sub>docs, +2</sub>
- `282ce566` the page side of a measuring scale takes a unit, so 1:50 is 1 mm = 50 mm  
  <sub>measure, +251</sub>
- `97d286b2` reflow growable XFA text after form edits  
  <sub>engine, +219</sub>
- `f06bebd5` dispatch repeated XFA print buttons by their own field  
  <sub>xfa, +95</sub>
- `a1466486` enforce XFA rich-text maxLength  
  <sub>xfa, +83</sub>

### M14 — 23 commits

- `1f881176` update ctutils to 0.4.5 for RUSTSEC-2026-0385  
  <sub>other, +4</sub>
- `adea2c90` recover existing macOS PDF associations after open and save  
  <sub>ui-egui, +639</sub>
- `b4f762ce` complete the French interface catalog (267 entries)  
  <sub>ui-egui, +266</sub>
- `071771f0` check_raster refuses a raster of no pixels itself, before any encoder  
  <sub>engine, +16</sub>
- `87648be1` JPEG and TIFF export refuse a raster of the wrong length instead of panicking  
  <sub>engine, +58</sub>
- `57d41762` Italian catalog: the 8-character digital ID password message  
  <sub>ui-egui, +5</sub>
- `eaed3409` the Preferences OK button stays in reach below the scrolling settings  
  <sub>ui-egui, +136</sub>
- `8ea90498` update Brazilian Portuguese parity notes  
  <sub>parity, +2</sub>
- `4eaba2bb` complete Brazilian Portuguese catalog  
  <sub>ui-egui, +316</sub>
- `4d2e4015` correct two drifted comments and cite why candidates are optional  
  <sub>ui-egui, +17</sub>
- `79cfdc00` apply the dead-path rule to families, not just the last resort  
  <sub>ui-egui, +22</sub>
- `4e69ca14` name the last resort and keep the pins honest  
  <sub>ui-egui, +34</sub>
- `5fc66c45` refresh the Spanish catalog's ATTRIBUTION entry and date the parity figures  
  <sub>docs, +25</sub>
- `1688bd40` check the Arial last resort only where it is installed  
  <sub>ui-egui, +32</sub>
- `4ac70042` Spanish catalog covers the bundled key union  
  <sub>ui-egui, +197</sub>
- `21dc64ea` skip the loader tests when installed faces are switched off  
  <sub>ui-egui, +6</sub>
- `19c3c8b1` find Apple's Devanagari faces in the folders current macOS uses  
  <sub>ui-egui, +160</sub>
- `b4bfe2b2` complete and review the Czech catalog  
  <sub>ui-egui, +357</sub>
- `aad803ca` add Catalan interface and date localization  
  <sub>ui-egui, +2840</sub>
- `d2fc6c1b` Polish says what a signature says, and the rest of the interface  
  <sub>ui-egui, +133</sub>
- `05179596` clarify Korean redaction and PDF terminology  
  <sub>ui-egui, +155</sub>
- `bd740ee9` align French terms with prepress and Acrobat vocabulary (70 entries)  
  <sub>ui-egui, +143</sub>
- `73fa64b4` complete the French interface catalog (267 entries)  
  <sub>ui-egui, +267</sub>

## D. Fixes — 282 commits, by area

### render — 29

- `743b3861` fix(render): draw device colour in the resources' /DefaultGray, /DefaultRGB and /DefaultCMYK (vendored hayro-interpret patch 38)
- `3b378707` Fix named properties in forms without resources
- `d9eb39d6` fix(render): draw each ICC profile through itself when two have the same stream dictionary (vendored hayro-interpret patch 40)
- `26273847` fix(render): images and nested groups knock out in a knockout group too
- `0f870c7b` fix(forms): read dynamicRender only at config/acrobat/acrobat7, in UTF-8 or UTF-16
- `fe3e07c2` fix(forms): an XFA form whose config forbids dynamic rendering is static
- `88735a70` fix(render): draw comma-below letters in non-embedded standard fonts (refs #1666)
- `d2bcda45` fix(render): knockout transparency groups knock out
- `421a0dd9` fix(render): mesh triangles that share an edge leave no seam
- `39b96ccf` fix(render): a visibility expression decides what an OCMD shows
- `f99966a1` fix(render): hit-test selection against the glyph quad, not its bounding box
- `785d1072` fix(render): pattern-filled stencil masks take the fill paint at the opacity once
- `88360885` fix(render): blend modes over an empty part of the page keep their colour
- `aaa4b99e` fix(render): spend the text budget on a rejected conversion
- `426a6097` fix(render): keep link addresses and attachment text within 16 MiB
- `9175291a` fix(render): draw missing spacing glyphs in Japanese CID substitutes as spaces
- `4ff9bd66` fix(render): cover all color spaces, padding and align /L policy with content crate
- `4362449f` fix(render): find an embedded file by key without the listing's budget
- `dfde9078` fix(render): list a repeated annotation once and cap what the listing keeps
- `48990720` fix(forms): read dynamicRender only from the configuration packet itself
- `d7b4d576` fix(render): emit clip-only text (7 Tr) to devices for text extraction
- `3df8a638` fix(render): a knockout clear follows a shading's /BBox and skips images that don't decode
- `7a73c2ec` fix(render): keep embedded-file keys within the text budget
- `c82dc91a` fix(render): list an annotation reached through a reference chain once
- `a0a66b98` fix(render): read an annotation attachment's Contents only when it names it
- `a0e82da8` fix(render): bound the replacement text copied into redraws
- `1cf6ca55` fix(render): a stencil mask is drawn at the fill opacity once
- `071130d4` fix(render): list links and attachments after the comment budget fills
- `8bcf83fa` fix(render): spend the raw length of text that converts shorter

### ui-egui — 26

- `91ce75ec` fix(ui): on-page text editors leave the mode tabs their pointer
- `7ec435eb` fix(print): Properties… reads CUPS driver options off the GUI thread, with a deadline for lpoptions
- `d7f8ba8b` fix(forms): Tab and Space reach check boxes and radio buttons, and a click focuses them (fixes #2223)
- `b387802f` fix(comments): Add a text box takes a drag that sizes the box (part of #2669)
- `3468c854` fix(print): disable Print on documents whose security forbids printing (part of #2669)
- `2a3dc472` fix(print): restore previous paper size after leaving Label mode
- `f4ee74db` fix(comments): the note editor takes the keyboard once, so V and H pick tools again
- `3d567bd7` fix(ui): mode tabs keep the hand over a page scrolled under them
- `6be95345` fix(print): the options read ends at its deadline and a dead read wakes the dialog
- `021812b6` fix(ui): the password prompt takes typing on Linux again
- `b913c285` fix(ui): a second file dialog waits for the open one instead of being refused (fixes #2150)
- `fefdccc8` fix(viewer): preserve gap toggles in organize and comparison views
- `7169160d` fix(ui): keep colour popups open when "+" opens the custom colour picker
- `7e68ef44` fix(combine): let the cards zoom to 500%
- `cdb1010f` fix(edit): a paragraph's width handle rewraps it from its whole handle, not just the inner half
- `d6a4602f` fix(ui): Korean (Hangul) file names and tab titles draw as boxes
- `0b77f4cb` fix(ui): Fill & Sign font size resizes the selected text
- `f536ac7a` fix(sign): fill typed signature outlines with non-zero winding
- `437b691e` fix(ui): the Read mode command leaves Edit as the Edit tab does
- `954c5142` fix(ui): the message bar shows a signature problem before "Certified by"
- `15a0b696` fix(comments): the eraser's size sets how wide it rubs
- `3261b6dd` fix(ui): show the physical page position under a labelled page box
- `c6f58baa` fix(comments): the eraser rubs exactly its preview, at any zoom or view rotation
- `aa5473e0` fix(ui): keep a closed right panel closed when a PDF asks to open on its outline
- `563a719f` fix(ui): the page tooltip stays in range on an empty document
- `b59b91f0` fix(print): Vietnamese for the printer-options loading note

### edit — 22

- `963609d7` fix(edit): draw the Cyrillic the Japanese faces lack with the bundled Inter face
- `a37a908f` fix(edit): page content, forms and image records share per-page limits
- `ed47f713` fix: read the writing mode leniently and join vertical glyph runs into one line
- `ed6ccdae` fix: compare the effective gs state, not the history of gs names, when grouping rows
- `ad9e5d2b` fix(edit): keep a one-row centred title centred when it grows
- `4e8e17c7` fix(edit): a header never takes over a font name the page already draws with
- `7a466ff5` fix(edit): prune a field again when a shorter path reaches it past the depth cutoff
- `9b5624af` fix: Issue #4017. Flattening removes empty fields it cannot draw
- `dc7a195d` fix(edit): prune each form-field node once after flattening
- `54152aed` fix(edit): refuse a mark on top of over-deep content before changing the page
- `fdcfe16b` fix(edit): bound the text one page's extraction can expand into
- `1ce4965c` fix: read the last array of a surplus-operand TJ, not the first
- `14b39783` fix(edit): a header reuses a font name only for its very own dictionary
- `b979d4d9` fix(audit): audit the glyphs on each page held, not their ActualText
- `c9090b00` fix(edit): refuse to put content on top when marked-content depth is unknown
- `ba024eb0` fix(edit): flatten on a copy so a page refused part way changes nothing
- `c38cdbf0` fix(edit): require a meaningful size difference for the wider tolerance
- `fe2ece46` fix(edit): keep the overlap guard for the wider footnote-marker tolerance
- `75207dc8` fix(edit): setting a paragraph's width no longer panics on text larger than a page
- `f1a89a48` fix(edit): wrap added text once per write, not once per line
- `84778107` fix(edit): keep a superscript footnote marker in its paragraph's row
- `c8883138` fix(edit): editor font enumeration shares one decode budget

### cos — 21

- `4e94aa1c` fix(cos): find objects a broken cross-reference table leaves out (fixes #1628)
- `733e38bf` fix(cos): a decode budget shared by every stream one walk reads
- `60d2d1e3` fix(cos): put inherited crypt-filter selectors under the new protection
- `2c4645de` fix(cos): check object-stream member numbers and offsets
- `57ab6fcb` fix(cos): the renumbering check follows a shifted header (refs #1628)
- `8444a08c` fix(cos): a repaired xref table fills only the numbers the damage leaves out (refs #1628)
- `a374d165` fix(cos): index the keys of a large dictionary
- `b1224557` fix(cos): find endstream once per document, not once per stream
- `58dc10c2` fix(cos): read a damaged real's numeric prefix in one pass
- `f63f5096` fix(cos): compare each section's table with a repeated hybrid stream
- `92e713fe` fix(cos): a /Filter that is not a name does not decode as unfiltered
- `a08d974a` fix(cos): encrypt the /Contents of an annotation that carries a /ByteRange
- `196be22b` fix(cos): read a hybrid cross-reference stream once per chain
- `4a7d556a` fix(cos): write whole reals past the 32-bit integer range as reals
- `ea4939cd` fix(cos): a strict decode charged to a DecodeBudget
- `986a64bc` fix(cos): cap the objects one parsed object may build
- `73d15cd8` fix(cos): count strings at every depth against the object caches
- `735c80c6` fix(cos): DecodeBudget decodes within a per-stream bound too
- `062270da` fix(cos): fill xref gaps on the compact cross-reference index (refs #1628)
- `92ab0e93` fix(cos): a spent decode budget reports its whole allowance
- `b72c730a` fix(cos): give the protect example a fresh key on every run

### sign — 16

- `343f1b05` fix(sign): signed attributes must carry the message digest and the data content type
- `a887405e` fix(sign): don't report valid when a DSS cap hides revocation evidence
- `bc6cfbd6` fix(sign): render Unicode characters in visible signatures using bundled Inter font
- `4f56e181` fix(sign): one content type and one message digest in the signed attributes
- `4e9d8f9b` fix(sign): require the time-stamping purpose for trusted timestamps
- `ffe99c19` fix(sign): eContentType must be data, and both content types must be OIDs
- `63b4af8b` fix(sign): embed the issuers of macOS Keychain identities
- `77ee1c8a` fix(sign): read DSS revocation evidence one stream at a time within caps
- `faab42d4` fix(sign): authorize an embedded OCSP responder by signature, not by name
- `ae791d66` fix(sign): a time-stamp response must carry the requested nonce and policy
- `d9cb7c2f` fix(sign): don't build chains through a CA with unprocessed critical extensions
- `fbada004` fix(sign): treat a changed trailer /Root after signing as not permitted
- `5162da1b` fix(sign): a widget counts as signed only when its /V resolves to a dictionary
- `5b813ce9` fix(sign): explain 0xC0000225 when a token's key provider can't find the key
- `07524f5e` fix(sign): explain 0x8009001D when a store key's provider fails to load
- `cb226fb0` fix(sign): the key description counts the modulus bits like the warning

### redact — 16

- `822179cc` fix(redact): remove hidden data from every object that owns it
- `dad86bf9` fix(redact,sign,cos): close high-severity findings from the agent scan
- `7db37911` fix(redact): clear descriptions above removed content and annotations
- `2cdb3950` fix(redact): compare marked content without a copy per enclosing MCID
- `be64a60c` fix(redact): unlink removed XObjects from surviving forms' resources
- `f1c56ef8` fix(redact): count a name tree's shared nodes once
- `f42fb0ad` fix(forms): a deleted field leaves the calculation order
- `241c5512` fix(redact): charge repeated name-tree kids to the node budget
- `39ee93fe` fix(redact,sign): address review — pop-up replies, signed-role deletions, widget role from signed revision
- `d2e7ce72` fix(redact): form entries and page content share one budget per page
- `0d216199` fix(redact): apply redactions on a copy so a failure leaves the document as it was
- `3fe9fe21` fix(redact): remove the widgets flattening keeps when removing form fields
- `750bbe27` fix(redact): clean the structure tree after removing hidden content
- `cccd435a` fix(redact): bound image decoding by the declared raster size
- `ee010fbb` fix(redact): read content strictly before sanitizing or redacting it
- `94563649` fix(redact): name the owner sweep's holder list for clippy

### engine — 11

- `895ccb90` fix(engine): guard undo and redo refresh and roll back on failure
- `22aa6d5f` fix(redact): place glyphs with the inherited and ExtGState-set text state
- `77480498` fix(engine): low-resolution printing prints only the chosen content
- `467c8556` fix(engine): encode each image once for Office export
- `6ca4431b` fix(redact): check every input content stream, whatever it calls itself
- `aa1c13f7` fix(engine): list a page truncated for both forms and pictures once
- `23a032ef` fix(engine): a reload keeps reading colours and hidden comments
- `28ab616e` fix(engine): print as low-resolution pictures when only that is allowed
- `f2f86455` fix(engine): read long underscore runs in one pass
- `73f11b2a` fix(engine): fail an action whose information-removal step fails
- `30300ae7` fix(engine): compare only documents that allow copying their text

### xfa — 10

- `1e2ac1d8` fix(xfa): copy embedded field values out only as far as the text budget
- `bac0e610` fix(xfa): bound the bytes a data import copies out of its file
- `1dbfa251` fix(xfa): bound the script text the live form's events hold
- `a07ad60d` fix(xfa): queue each added data instance once per pass
- `ee333376` fix(xfa): bound the name bytes of data nodes a write creates
- `34124371` fix(xfa): bound the embedded field text one text block expands
- `17f5d80f` fix(xfa): break a long word in linear time
- `306627ee` fix(xfa): read a packet stream the XFA array lists again only once
- `2e1c029f` fix(xfa): every flowed subform instance spends the layout budget
- `668d8f77` fix(xfa): every drawn subform instance spends the layout budget

### xfdf — 9

- `b1196c5c` fix(xfdf): bound what an FDF import copies out of repeated references
- `63f7932d` fix(xfdf): fall back to the /Rect when an imported redaction mark has a zero-area quad
- `d335ff37` fix(xfdf): a field name full of periods no longer overflows the stack on export
- `d2662223` fix(xfdf): draw every imported redaction mark's review outline, opaque
- `8a8dab6c` fix(xfdf): keep merged spreadsheet cells from starting a formula
- `8adf8e2d` fix(xfdf): read an indirect /F when deciding a comment is locked against import
- `f0e3880c` fix(xfdf): never let a same-named imported comment replace a form widget
- `93436d41` fix(xfdf): escape paired line-ending names in exported XFDF
- `25609702` fix(xfdf): use an imported redaction mark's /Rect only when it is four finite numbers

### automation — 9

- `ab9a5009` Fix #3507: preserve content boundaries when merging image fragments
- `ca8ebb0b` fix(automation): tools on a file the viewer reads fewer pages of answer instead of panicking
- `25eb5688` fix(forms): preserve content stream comment boundaries
- `acacf948` fix(automation): hide_text_block resolves its number like text_edit, and a merged paragraph is not followed
- `5dc64fac` fix(automation): keep page text from tools when copying is not allowed
- `0ca7ed07` fix(automation): a page whose paragraph listing was dropped at the cap is refused, not read by position
- `a62986b5` fix: honor single-page redaction selection
- `570e6c05` fix(automation): an emptied paragraph does not pass its number to an overlapping neighbour
- `f0494753` fix(print): list the US #10 envelope in doc_print's paper description

### js — 8

- `72fd2a1b` fix(js): bound the text an XFA script's effects and values hold
- `a5cfc3ee` fix(js): cap a form script's alerts, console lines and requests
- `e5411513` fix(js): XFA presence and access change the snapshot only when their effect is kept
- `a525cf1d` fix(js): refuse code compiled from strings that skipped the script checks
- `2ece498f` fix(js): charge both copies of a written XFA value before keeping either
- `2e9b247f` fix(js): read printd literals in linear time
- `d67c4b30` fix(js): an XFA value past the effect cap leaves the field unchanged
- `c32c2ce0` Fix FormCalc Str() printing small negative numbers as "-0"

### forms — 8

- `da351cda` fix(forms): cache field appearances drawn for display, redraw only what changed
- `b661ba45` fix(forms): draw fields the file leaves to the viewer, for display only
- `bb8ea65f` fix(forms): script streams share one decode budget per reading
- `fbd9911d` fix(forms): renaming a nested field checks the full name it would take
- `c73eb76e` fix(forms): draw the widget border inside /Tx BMC so Acrobat keeps it on focus changes
- `63843811` fix(forms): read each widget once when enumerating fields
- `5473d354` fix(forms): skip a field of an unread kind before it claims shared widgets
- `2d5c5054` fix(forms): merge a calculation's read names with a set

### organize — 8

- `cbcdca77` fix(organize): imports find the objects they added when new numbers fill gaps
- `9c9bbe2a` fix(organize): page content is joined within one decode budget
- `cab1dc3e` fix(organize): read each name-tree node once when importing
- `d39b55dd` fix(organize): register imported form fields in linear time
- `da288517` fix(organize): resume the field-rename suffix search where it stopped
- `1f6ce5ec` fix(organize): analyse each form once when pruning unused XObjects
- `59dbdee4` fix(organize): copy annotation actions with one shared reference map
- `cb1ddc89` fix(organize): find XMP attributes in one pass over the packet

### audit — 8

- `65ee18d4` fix(audit): bound cover-line comparisons and covered text
- `3fce90e9` fix(audit): withhold covered text when copying is not permitted
- `4c7ba85b` fix(audit): resolve an indirect /Subtype before skipping an object
- `b5cddc88` fix(audit): page content and forms share one decode budget per page
- `3e306c22` fix(audit): compare a cover with the start of a merged text line
- `3e16c71a` fix(audit): count only the covered text a finding reports against the output bound
- `747fb2df` fix(audit): count each word box a line has against the comparison bound
- `cc1086a8` fix(audit): keep the pending path across Q

### print — 7

- `14894363` fix(print): preserve existing appearances without Subtype
- `69109cd4` fix(print): reuse shared appearance promotions
- `8c8f56a2` fix(print): page content and label forms share one decode budget per page
- `24701fe9` fix(print): list the source pages once per imposition
- `08579962` fix(print): offer the US #10 envelope as a paper size
- `2430d53f` fix(print): read the text after a form the budget refuses
- `e3847ba0` fix(print): pass the lpoptions limit outside the stand-in command in the CUPS test

### annot — 7

- `1695106f` fix(annot): list a repeated link once and cap its web-address text
- `c2e96de0` fix(annot): extend each page's annotation list once when adding many links
- `05d09de1` fix(annot): read a shared page tree Kids array once
- `02f0f739` fix(annot): count web-address text after its UTF-8 conversion
- `cf565db3` fix(annot): a comment listed many times gets one display appearance
- `6394779b` fix(annot): check a whole link batch before creating any link object
- `e07942c8` fix(annot): cap the bumps of a cloudy outline

### content — 7

- `b7207887` fix(content): end inline images where the renderer ends them
- `0a30b4e0` fix(preflight): read content colour one operator at a time
- `9451e065` fix(content): recognize ~>EI terminator for ASCII85 inline images and cover filter arrays
- `3c64f931` fix(content): skip a long unreadable number whole
- `07aea983` fix(content): splice copies only the pieces a range touches
- `49456777` fix(content): restrict '>EI' terminator detection to ASCIIHex filter
- `d1b0293a` fix(content): accept inline image EI without preceding whitespace

### fonts — 7

- `75c42758` fix(fonts): look up CID and width ranges by binary search
- `05df6ee4` fix(fonts): decode each candidate font program once, within its size
- `5609bcbe` fix(fonts): wrap text measuring each word once
- `8da1164f` fix(fonts): decode Encoding and ToUnicode CMaps within 16 MiB
- `1cc392e5` fix(fonts): cut ToUnicode destinations to the CMap limit
- `7a33a85d` fix(fonts): embedded sfnt reads can share one decode budget
- `7eaa2079` fix(fonts): let auto-deref pass the candidate objects to the program match

### vendor:hayro-syntax — 7

- `f9d37c08` fix(render): use unfiltered inline image byte length before EI heuristic
- `a6b2ad7f` fix(render): resolved JP2 palette colours keep 8 bits whatever the index precision
- `01af6cba` fix(syntax): bound inline-image lookahead by content window
- `e1aeb00b` fix(render): apply the JP2 palette of a JPX image with no /ColorSpace
- `04697ad4` fix(content): an inline image ends at its own EI when later text holds a non-ASCII byte
- `857fef3b` fix(render): say that an unresolvable /ColorSpace also gets the JP2 palette
- `312a97d2` fix(render): restore the test's closing brace lost when rebasing onto main

### create — 6

- `9188cf59` fix(create): don't decode a JPEG carrying thousands of Extended XMP chunks
- `7e9f5a44` fix(create): bound a TIFF's decoded pages together, and read one when one is wanted
- `cad0fe45` fix(create): refuse a GIF screen past 4096x4096 and convert its canvas in place
- `4aad36cf` fix(create): refuse an indexed colour space whose base is indexed before following it
- `83d8eb08` fix(create): decode an indexed lookup table within a small cap
- `c704c108` fix(create): keep the TIFF total budget in range on 32-bit targets

### other — 5

- `811a3a84` fix(xtask): wait for Chrome child processes to settle PDF after launcher exits
- `e93f11e8` fix(packaging): use [[ tests and name positional parameters in the scripts (SonarCloud shelldre:S7688, S7679, S7682, S131)
- `19e95dd2` Fix stale translation-catalog SHA-256 entries after the rebase
- `80b7d215` fix: add generic font-family fallbacks in demo (SonarCloud css:S4649)
- `0b670caf` fix(deps): bump crypto-bigint to 0.7.6 (RUSTSEC-2026-0387)

### optimize — 5

- `0b6adb70` fix(optimize): page content and forms share one decode budget per page
- `6b625853` fix(optimize): bound the search for fragment runs on a page
- `cfda9223` fix(optimize): keep a destination referenced only through an indirect /Dest
- `dba51fc8` fix(optimize): refuse a JPEG whose own size differs from its dictionary's
- `78a562bc` fix(optimize): bound a merged fragment image's height before sizing its buffer

### apps — 4

- `06c4a970` fix(cli): info, text, render and audit take the owner password of R2–R4 files
- `72582843` fix(cli): bad option values name the option and the value, before any file is read
- `857c4913` fix(app): pdfcraft --help prints usage instead of opening the app (#4076)
- `0de54658` fix(windows): present Direct3D 12 through a composition visual, so resizing doesn't stretch

### preflight — 4

- `90982ba9` fix(preflight): check the actions an allowed action chains to
- `adab150a` fix(preflight): find annotation pages from one pass over the pages
- `62b54fce` fix(preflight): check the actions in an indirect /Next array
- `853a9ae1` fix(preflight): decode an output-intent profile within 16 MiB

### a11y — 4

- `060381c7` fix(a11y): join and splice page content streams with cross-stream rules
- `38211588` fix(a11y): find every figure's bounds on a page in one pass
- `110b7c67` fix(a11y): read the enclosing marked content in one step per operator
- `13d9c9bf` fix(a11y): page content and forms share one decode budget per page

### measure — 4

- `a16661ac` fix(measure): list a repeated measurement once and cap what a listing keeps
- `240d051b` fix(measure): read viewports within bounds when finding a scale
- `b6cb1517` fix(measure): read a stream's XObject resources once when snapping
- `a34697a8` fix(measure): a failed content decode uses up the snap byte budget

### vendor:hayro-interpret — 2

- `0c748e34` fix(render): bound xref streams, JBIG2 referred-to counts, saved graphics states and /W2 runs
- `3b6d344e` fix(render): find comma-below letters by character (refs #1666)

### platform — 2

- `bac52a6d` fix(save): preserve macOS extended attributes
- `c3bc8816` fix(platform): a saved file keeps its group along with its mode

### vendor:hayro — 2

- `bae69819` fix(render): the device decides whether a knockout group's image clears its rectangle
- `82023e2f` fix(render): the one-cell tiling renderer starts outside any knockout group

### ci — 1

- `487907ad` fix(links): resolve remote document paths with craft-docpath's resolve_absolute

### parity — 1

- `8555d6f1` fix(parity): cite the renamed renderer-check test and drop fixed redaction gaps

### export — 1

- `fcedb156` fix(export): find font size and colour slots through a map

### model — 1

- `6905acad` fix(model): bound the inherited values a page walk copies

### crypt — 1

- `1127a660` fix(crypt): resolve crypt filter methods once per handler

### filters — 1

- `e2c71b60` fix(filters): refuse a filter chain longer than eight

### oracle — 1

- `b57e2f68` fix(oracle): chunks_exact with a constant size trips clippy 1.99

### vendor:lopdf — 1

- `9d31cf9f` fix(docs): correct spelling of "Formatting" in comments

## E. Everything else — 271 commits

### ui-egui — 88

- `1ff8fe48` i18n: add Indonesian and Vietnamese interfaces
- `644348dd` i18n: add Hindi and Korean interfaces
- `0a78f31f` Add Finnish interface and date language
- `a30a0ab7` G14: Romanian (ro) interface catalog
- `8d99a5ce` Translations: a missing string never fails a test; xtask ci prints the gaps
- `14fcc82a` i18n: add Central Kurdish (Sorani) as an interface language
- `d5bbf60e` i18n: add Thai (th) interface language
- `c699e88c` i18n: add Albanian (sq) interface language
- `4164c0e3` i18n: add Lao (lo) translation support
- `7137c41f` feat(viewer): add optional gapless continuous page display
- `86730ed1` Tests: UI tests wait for background work by time, not frame count (#4186)
- `df591ff8` Rail: scroll the panel buttons above the page navigation in short windows
- `0121f534` Sync with main: the 80 new strings in all four new catalogs
- `a047c6ba` chore: resolve SonarCloud maintainability issues
- `d9306ab2` Integration: catalogues cover the r2 batch's new literals
- `fcdf2a21` Layers panel: Reset to Initial Visibility
- `e85fca66` Integration: text field table labels in the batch's catalogues (upstream #3479 × r2 catalogues)
- `ad35bd27` Add files via upload
- `afee39e6` Integration: English for the keys the new catalogs lack
- `44e8eeb2` Cancel stale Organize page drags after document changes
- `1a12b009` Preferences: always use my default zoom and page layout (fixes #1618)
- `a1acf528` Print: remember selected paper size across restarts
- `305a5676` Open: programmatic callers see focus vs open vs failure
- `cb7ad709` i18n(fr): drop the rewording of existing rows
- `57e531cc` G14: Romanian: translate the 64 catalog rows still in English
- `49f3125e` Restore the Telugu system-font fallback the i18n rebase dropped
- `6882ce34` Ctrl+V pastes a picture from the system clipboard onto the page (fixes #2494)
- `a5ef6b76` Wake idle windows for pending recovery snapshots
- `1c3cb9d1` feat(forms): nudge the selected field with the arrow keys in Prepare a form
- `299af117` i18n(th): translate the 80 strings added upstream since 0.8.0
- `b248dffd` Hebrew file names: draw them with an installed Hebrew face when the build has none (part of #586)
- `92062541` Open Save As and export dialogs in the document's own folder
- `f2e33db1` Regression tests for #246, #843 and #2028
- `1298aa28` Print: keep the picked printer for the next print
- `ea0c1ae1` Review: a selected line or polygon moved by its body is drawn where it is going
- `280f2439` Preferences: field highlights on by default, with a toggle in Appearance
- `ca562716` Preferences: list the languages alphabetically by their own name
- `215162b4` i18n: add Hindi, Indonesian, Korean and Vietnamese interfaces
- `95a769ee` Open: opening an already-open file focuses its tab
- `e3afcaaa` Integration: #2210 fill the 46 keys Finnish lacks with English text
- `10b768ff` Test idle autosave after a second edit
- `0a77db06` Forms: a radio group's read-only button is left unhighlighted on its own
- `56b316a7` Forms: a radio group's read-only button can't be chosen by a click
- `c7fbcd3f` Open: focus edge cases - vanished file, encrypted prompt, pathless documents
- `9e19d47e` Edit images: rotate knob is grabbable, flips below at the top, label readable
- `8341a721` Integration: mode-tab hand tests use the tabs the mode bar has (#2240 × #3010, #3022)
- `51a5d4ef` Integration: #3819 move name_sort_key above the test module
- `7eb4e5d5` test(comments): the eraser preview is as wide as it rubs while held
- `b99c774f` Integration: serialize GPU rendering in the ui tests (batch render tests × lavapipe)
- `6ee8329e` test(ui): the reduced-copy regression saves a real PDF and reopens its text
- `0d0cce1b` Forms: Highlight fields marks only the fields that can be filled
- `46417784` Integration: the live line-resize test follows lines across rotated pages (#3520 × #3781)
- `3a1c0293` ko: unify redaction-mark, flatten, Verify and Collapse terminology
- `4242e7ed` Test bringing the window forward for a new file while it is still preparing
- `deca1e03` Control channel: wait after a failed accept instead of retrying at once
- `4de9459f` Give the print preview's grayscale tag a bit of its own
- `de25be16` test: the off-page paragraph test compares the gap with the same pixels without Edit text
- `a7160cf4` Drop: dropping an already-open document focuses its tab
- `2f208a28` Keep Auto font size in the field properties Appearance tab
- `38e5c4bb` Bring the window forward when the running app is handed files
- `30bafb49` ui: add vertical spacing around panel header separators to prevent clipping
- `3e180944` i18n(ui): carry the two new signature wordings in the catalogues that must cover them
- `4bd08a75` Integration: Albanian signature labels all translated (#3101 × r2 catalogues)
- `d5bde67e` Ctrl+V doesn't paste a picture while preparing a form
- `a5e16bf5` Open Recent: focus an already-open file through the shared lookup
- `f295be1e` Tests: field highlights are on by default, so the toggle reads "Hide field highlights"
- `b15fe916` test: the image-drag pixel test waits for the page raster
- `9a2d2cfd` Integration: panel snapshots for the assembled batch (#3525 × #2607, #2731, #3828)
- `b5ba1760` test: the paragraph editor is drawn at full colour from the click's own frame
- `1acb402e` Integration: #2452 Polish for the two snapshot notices; English for five panel labels
- `262ae579` i18n: Lithuanian and Vietnamese for the new signature messages
- `3f3b9130` Integration: the dead-read print test waits for the preview rasters before checking idle (#2108 × print preview)
- `3d6f3fa3` Integration: #2070 revision and signed-version views opened in full screen present one page at a time
- `3bf32f9d` Integration: #3878 use a Thai letter as the undrawable character in the failed-edit test
- `0b621a94` Integration: cargo fmt
- `ad28a507` js_ui: put the preferences_body doc comment back on preferences_body
- `c9aff059` Integration: #3864 one doc comment each for page_raster_out_of_date and page_out_of_date
- `6840b0fc` Tests: wait for the vanished file's tab before deleting it
- `50ae733d` Integration: Alt+arrow leaves a selected field in place for Calculation order (#2621 × #3733)
- `c5fca1ee` i18n: translate the plain-file-name validation literal in id, hi, ko, nl, da, sv and fi
- `d6a27cd7` i18n: restore the upstream Vietnamese entry dropped during the main merge
- `40949476` Integration: cargo fmt
- `5237a542` Integration: #3325 the rail's page navigation panel is not resizable
- `b7af1536` Integration: #3931 move the screenshot doc comment back to compare_panel_screenshot
- `77eb328c` Forms: doc comments for the functions this change touches
- `5397a4af` Document the new-file OS open test
- `022a970f` Integration: #2173's restored-paper test lets the dialog relabel its button before clicking Save as PDF
- `17bf5e9c` Integration: #3603 drop the second Spanish Font entry

### other — 33

- `53199fb4` Release 0.13.0
- `39844f06` Release 0.12.0
- `c0c8dc57` Integration: ATTRIBUTION hashes
- `c4a6dd6b` Assets: don't hash translation catalogs in ATTRIBUTION.toml
- `a2597f92` Integration: ATTRIBUTION hashes
- `045c8b39` CI: nextest, line tables only, one wasm check
- `2cf91e3c` Packaging: pin the signing client and appimagetool, and check their hashes
- `ddc63d70` FreeBSD: package the aarch64 cross build (craft-common-actions#39)
- `35b869a2` Integration: ATTRIBUTION hashes
- `a074458c` Integration: ATTRIBUTION hashes
- `d578bd74` Integration: ATTRIBUTION hashes
- `5486100d` chore: refresh catalogue hashes after rebasing onto main
- `7cdde6c9` chore(assets): record the changed translation catalogues in ATTRIBUTION.toml
- `86739c11` Integration: ATTRIBUTION hashes
- `3dcd3a7f` Integration: page-gap labels in the seven new catalogs (#3101 × #2210, #2079, #2141, #2227, #2505, #2714, #3498)
- `fb1f72e9` Attribute the vendored cmaps.brotli as a repository asset
- `2e4d9a40` chore(deps): update rustls to 0.23.46
- `bd9030b2` chore(sonar): exclude vendored crates and the spec-mandated MD5/RIPEMD-160 (SonarCloud rust:S4790)
- `0dc4f923` Integration: #3101 page-gap labels in the Indonesian, Hindi and Korean catalogs
- `7848fc40` Integration: #2141 ATTRIBUTION entry for the Thai catalog
- `756a2f6f` i18n: add the Zoom percentage-string to Hindi, Indonesian and Korean
- `8293832a` Integration: Norwegian Bokmål page-gap labels (upstream #1836 × #3101)
- `faa7e6ac` Integration: ATTRIBUTION hashes
- `2990f0bc` Integration: ATTRIBUTION hashes
- `687425fa` ATTRIBUTION.toml: new SHA-256 for the es, lt and vi catalogs
- `5b61047e` i18n: re-verify and update the translation-catalog SHA-256 entries after the merge
- `aef42bc9` Integration: cargo fmt
- `8026e850` contributors: add my own entry (brian-moon)
- `faef5792` README: the app icon above the title
- `9ca29278` docs: credit MatrixMagician in the About window by their GitHub profile name
- `e6794983` chore: contributors entry for Lcoding15
- `be2f96a8` contributors: credit Guillem Hermida by real name
- `02b9a692` chore(contributors): add Bode512's people.toml entry (sync_github_name)

### edit — 12

- `20cfb67d` Lay out vertical (Identity-V) text as columns in the edit layer
- `57c6e476` Keep text rows in other layers, gs states or Tz out of a paragraph
- `fecfc774` feat(edit): Header & Footer can be drawn in Times or Courier
- `5eae8d6e` Edit text: keep a line together across subsets of the same font
- `f980b34e` Text and image reading charge a page's content and its forms to one decode budget
- `1d76fcbc` Edit text: tests for re-encoded subsets and five-letter prefixes
- `4a618e8d` Edit text: do not group or join by an empty font name
- `631ca00b` The image scan skips a path segment before any m instead of panicking
- `25e273a3` Pair /ActualText correctly past 256 levels of marked content
- `37b6c530` refactor(edit): images read page content through content_streams
- `4fc56fd3` Reading blocks documents that over-budget page content is an error and truncation covers forms
- `55e32759` Integration: subset fallback test names the face the build leads with (#2609 × #1760)

### render — 12

- `1b056db5` Reject rounded page rasters beyond the pixel budget
- `4ce3e4f2` render: draw device shadings in the resources' default colour spaces
- `3c366d5e` Render: radial luminosity fades have regression coverage
- `645a8ca2` Render: a five-point polyline is no longer painted as its bounding box
- `b744866d` Shading patterns use the stroke or fill alpha current at paint time
- `88e7a1ce` test(render): a shared-edge pixel keeps the colour at its corner (#3035 review)
- `77112959` test(render): the non-separable blend modes keep their colour over an empty page too
- `1812aafb` perf(render): read PDFCRAFT_TEXT_DEBUG once instead of per glyph
- `8708cbb9` style(render): rustfmt the comma-below test (refs #1666)
- `02440799` style: cargo fmt
- `2a93b315` test: the knockout fixture's nested form declares its real /Length
- `cf1305fa` Format the shading alpha test

### cos — 11

- `2ec501fa` test(cos): property tests for save fidelity, plus credit entry
- `0c09000f` A decode charges the page's decode budget for every filter stage's output
- `0ae289c7` A failed strict decode charges the page's decode budget for what it decoded
- `227b4bfc` Issue #2365: a rebuilt file keeps the /Info and /ID of its cross-reference stream
- `0e8ebf00` Issue #2366: a trailer whose /Prev is a reference still reaches the earlier sections
- `e336b48a` cos: a rebuild prefers a trailer whose /Root is the catalog (#2365 review)
- `7fb6ab0f` test(cos): write the qpdf oracle files into a private new directory
- `d5047aa4` test(cos): build the private temp directory without an unused mut on Windows
- `801980ba` cos: the /Prev repair note names the reference's own generation
- `79d84352` Integration: #2220 look up set-aside object numbers in a set
- `3a11f39e` chore(cos): allow the boxed dictionary key index under clippy

### docs — 8

- `3463d0e0` Integration: #3323 drop docs/form-maxlen-compatibility.md
- `580ef9f9` docs: drop the Korean review notes from the tree
- `fb05fda2` Integration: ROADMAP.md, docs/roadmap.md, docs/gaps.md merged line by line
- `8d469f9b` docs(roadmap): record #2662, the audit's one-word cover
- `f1e5aa1f` Integration: progress log newest first
- `b8b09d31` docs: remove shipped Vietnamese from M14 missing lists
- `6973d78b` docs: merge the duplicated catalog paragraph in localization.md
- `1646fc78` docs(claude): check for existing issues and PRs before starting a fix

### a11y — 7

- `7322091d` Accessibility: set a table's Summary and a header cell's Scope (#4103, #4104)
- `9d5bcc07` Accessibility check asks for a manual check where page content ran past the decode budget
- `0f5e7410` Follow a chain of references to the tagged flag, as the inspector does
- `4e1457ed` Marking a tag as an artifact refuses content past the page's decode budget
- `bfc74873` Tagged PDF: resolve /MarkInfo /Marked when it is an indirect reference
- `089882f3` Marking a figure decorative refuses content past the page's decode budget
- `f02b9981` Format the accessibility tests

### redact — 7

- `92c979c5` redact: refuse marks with no readable area instead of skipping them
- `c2c4db7e` redact: count shared outline items once so the walk ends
- `e2a67716` Redaction charges a page's content streams and its forms to one decode budget
- `b97b3832` redact: count inline outline items against the total budget
- `01aa6bcc` docs(redact): describe the overlay, text state, description and inline image rules
- `4d64ca8e` docs(redact): say which objects hidden-information removal reaches
- `71e03461` docs(redact): say that a mark with no readable area fails apply

### parity — 7

- `95263631` Build each crate's integration tests as one binary
- `09e32698` Parity: an already-open file focuses its tab on every open path
- `9f063cee` docs: gap 2 and view.render-cid-fonts for /ToUnicode /Identity-H
- `693eae1d` docs(parity): cite the audit's new tests and record its bounds and copy restriction
- `17dbc885` Revise evidence and notes in acrobat-features.toml
- `b1759657` Parity: limit the drop focus claim to path-bearing drops
- `942be185` parity: form.fill-sign-date has 14 presets now

### vendor:hayro-interpret — 7

- `9c2f4e35` perf(render): look CFF glyph names up in a map (vendored hayro-interpret patch)
- `9a997cf1` A rectangle is a traversal, not four corners visited
- `ec6a8166` Integration: #3020 number its hayro-interpret patch 48 (40 is #2987's)
- `e3fc8293` A rectangle's corners are equal, not nearly equal
- `c3c0bea0` Integration: #2977 number its hayro-interpret patch 46 (35 is #3296's)
- `85385107` Mark the alpha changes as PdfCraft patches
- `e22767f7` Integration: #2974 knockout erase pass reads form resources through Resources::for_form (#3637 made them optional)

### fonts — 6

- `cf0f8060` Allow OFL typefaces, Noto CJK and Source Han included (AGENTS.md §1.1)
- `a61bd92f` Fonts: measure a remapped standard-14 code by the glyph it draws
- `61632e48` Treat CRLF as a single line break in fonts::wrap
- `3e968c8a` chore(a11y, fonts): satisfy clippy in the CID width ranges and a test helper
- `b6c81567` Integration: cargo fmt
- `ab815b50` Stop Std14::recognise mapping sans-serif font names to Times

### vendor:hayro-cmap — 6

- `bf27cb25` Vendor and patch hayro-cmap for tolerant CMap parsing
- `e7045042` hayro-cmap patch: fix guard, entry alignment and array drain
- `69c87d2f` Integration: #3195 keep CMap entries read before a stream ends inside a block
- `dd64f179` hayro-cmap patch: an unresolved usecmap with no mappings reports failure
- `f6521cf6` hayro-cmap patch: tests for the unresolved-usecmap failure guard
- `e08950c8` hayro-cmap patch: truncated metadata-only streams report failure

### ci — 5

- `d1756e48` CI: shared gate workflow; nightly fuzzing in its own workflow
- `aea5e78e` release: the git-sources gate moves into craft-common-actions (storytold/craft-common-actions#78)
- `f4b5a909` ci(deps): bump Swatinem/rust-cache
- `05ac6a91` ci: read-only token for the CI workflow
- `a2ec27b4` ci(sonar): don't cancel superseded runs on main, so coverage is published

### vendor:README.md — 5

- `71efe25d` docs: one row per crate in the vendor README table
- `ee092caf` Integration: #3827 revert the unrelated vendor/README.md hunk
- `1227ab72` docs: consolidate hayro-syntax vendor record
- `c4fa66ec` Integration: #3668 vendor/README.md: describe the delimiter after EI in hayro-syntax patch (4)
- `7a290760` docs: patch (38)'s regression in the vendor README's Test column

### vendor:hayro-syntax — 5

- `3af35002` Issue #2366: a /Prev reference to an object holding the offset reads that offset
- `c351febe` hayro-syntax: a trailer /Prev written as a reference reaches the earlier sections (PdfCraft patch 22)
- `b85af272` Accept a delimiter after EI when ending an inline image
- `766ff9d0` Render: a doubled sign is one number, so a cm with it still has a matrix
- `386c92c0` Render: skipping a number covers the signs reading accepts

### sign — 4

- `7c473123` style(sign): rustfmt the new tests
- `30331b81` test(sign): serialize the Keychain search-list tests
- `737f2f72` test(sign): a small RSA key's signature verifies with the short-key note
- `8e08d266` Integration: #1679 test that a Japanese label beside a Latin Extended name is refused, not drawn in the wrong glyphs

### js — 4

- `5157e042` Keep seconds in FormCalc Time2Num/IsoTime2Num for ISO 8601 times
- `31373819` Zero-pad required integer digits in FormCalc Format()
- `3ee6a83e` Accept long unit names in FormCalc UnitValue
- `3246587e` Return empty WordNum result when cents rounding carries to 1e12

### audit — 4

- `1cb113f1` Audit: find opaque square annotations over text
- `1390ea41` Audit: a Square the renderer never draws is not a cover
- `e0e7d446` Cover audit charges each form entry to the page's form allowance
- `68c14c9d` docs(audit): the comparison bound counts words

### xfa — 4

- `a4cdbb62` XFA: decode BMP and TIFF pictures (fixes #1644)
- `84ee69b5` Render month and weekday names in XFA date patterns
- `947f0f29` XFA: turn TIFF pictures by their Orientation tag
- `f84eeaed` Integration: a non-interactive XFA field is measured at the width it is drawn (#3515 × #3521)

### print — 4

- `a9320efd` Print ▸ Properties…: the driver's options in the UI language
- `1fb0ab98` Integration: cargo fmt
- `1dac21c5` Print: PPD localization helpers are CUPS-only outside tests (Windows dead-code error)
- `7fbac38d` Integration: #2647 the lpoptions deadline test passes the language

### forms — 4

- `b2150f3f` Forms: measure a field's own simple font, not Helvetica, when laying out its text (fixes #2894)
- `1cd36bfc` Decode JavaScript hex and control escapes in AF script strings
- `4125bddb` Parse a bare trailing "a" as AM in AcroForm date/time parsing
- `64e915a1` Integration: #3543 merge the duplicated doc comment on button_appearance

### automation — 3

- `85da56ed` test(automation): pin the content_list guard and form_set_props' refusal
- `c9ac28a0` test(automation): give sign_document a real digital ID and document in the hostile-arguments sweep
- `eb6a4a2d` Integration: #1830 standards_remove_id is not the standards.pdfa command

### engine — 3

- `2250841f` Integration: #2714 Lao date names and drop the stray Preferences... key
- `844e3578` Integration: #2624 doc comment of xfa_scripts_initialize_calculate_validate_toggle_and_add_rows back on its test
- `35b5abd5` test(engine): show the truncated-page picture exports within a sufficient budget

### vendor:hayro — 3

- `fe7169c8` Renderer: sample a mesh shading over its triangles alone, not the whole path
- `b85b0d2d` Integration: #2974 number its hayro patch 22 (19 is #3296's)
- `ded4bee6` Integration: #2974 tiling early return in set_paint returns PaintSetup::Clip (#3296's return type)

### vendor:lopdf — 3

- `1c126150` lopdf: a rebuild takes the trailer from a cross-reference stream (PdfCraft patch 6)
- `d75d440a` lopdf: a rebuild prefers a trailer whose /Root is the catalog (#2365 review)
- `a2bc0d35` lopdf: a rebuilt trailer's /Root must match its generation (#2365 review)

### xfdf — 2

- `12c66f71` Keep zero annotation flags through XFDF export/import
- `b174b12b` Reject non-hex characters in XFDF colour parsing

### filters — 2

- `03366c3c` A counted decode bounds all filter stages together and counts output before the predictor
- `11d2a576` Filters README lists decode_counted

### crypt — 2

- `cdbce8ac` crypt: authenticate against encryption numbers we did not compute
- `ecaa2980` crypt: say how the quoted vectors reproduce, and name what nothing verifies

### annot — 2

- `a0c10626` Fill & Sign marks stay upright on rotated pages
- `7420af67` Draw a note without an appearance as a fixed-size icon at the corner of its Rect

### organize — 2

- `15f92032` Page-tree walks in annot, forms and organize skip repeated children
- `586246d6` organize: check the page's own MediaBox values in the walk test

### optimize — 1

- `3dbdc13b` Image resolution walk charges each form entry to the page's form allowance

### model — 1

- `735fb8e6` docs(filters, model): record the filter chain and inherited copy limits

### apps — 1

- `caeaac5c` Integration: render test expects the RGB PNG of an opaque page (#2825 × #2425)

### export — 1

- `1b73c11e` Integration: #2994 put the part() doc comment back above part

### measure — 1

- `be223888` Join /Contents pieces with a space in measure snap

### content — 1

- `07f3c974` Keep filtered inline images whole when their data contains EI

