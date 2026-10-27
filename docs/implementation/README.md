# Implementation queue and working agreement

**Started:** 25 October 2026. **Stage:** basic. **Authority:** the user's instruction to add the full research scope and deliver one audited component at a time.

## The complete TODO

**[Open the live TODO list](TODO.md)** or filter [TODO.csv](TODO.csv).

The queue retains **all 222 research recommendations, all 384 partial/planned baseline entries, and all 23 originally out-of-scope entries for explicit scope review**. These records overlap; they are not 629 independent missing functions. Bounded implementation components are tracked separately and linked to their parent workstream. Already-shipped functions are improved or audited where needed, not blindly recreated.

The original [research report and workbook](../research/pdf-product-strategy-2026-10-25/README.md) remain a dated snapshot. Live status is in [progress.toml](progress.toml); the checked-in TODO files are generated from it. Historical research claims are never rewritten to pretend proposals shipped.

## Mandatory order

1. **Basic:** safety, reliability, accessibility and everyday workflows. Ordinarily research P0/P1 plus basic checklist gaps.
2. **Advanced:** professional/segment functionality, including an optional collaboration service and its mandatory safety gates.
3. **Future:** speculative features and new native mobile/platform investment, followed by the retained vendor-specific scope reviews.

A P0 gate means “mandatory when the affected workflow is offered,” not “build a cloud service before improving local saves.” Service-specific authorization/versioning gates are scheduled with that advanced service and **must pass before it is offered**. Existing CLI functionality is not deferred merely because it can also be reused by a future service. Native mobile components remain future; responsive browser work can be basic. No external subscription, provider, upload or release is enabled just by placing it on this list.

When a baseline gap is referenced by several recommendations, its stage is the earliest applicable recommendation stage. Unmapped gaps inherit their baseline tier (P0/P1 basic, P2 advanced, P3 future). Review a component's real dependencies before starting; record and resolve dependency conflicts rather than silently claiming an entire stage is complete.

## Batch authorization — 26 October 2026

The user revised the earlier per-component approval requirement: “ok finish all basic at once, make sure you audit it after coding it.” Continue the **basic** stage without approval stops between its components. Keep separate acceptance, post-code audit, fixes, re-audit and tests for each component; a batch may share fresh full CI when that exact verification commit includes every component's final code and regressions. Do not merge, publish, deploy or advance to advanced/future under this authorization. Blocked or unverified records stay open; this instruction is not a waiver of acceptance criteria or evidence. The earlier component stop notes below are historical and superseded for basic-stage continuation only.

## Per-component quality workflow — applies inside the batch

For **every** component:

1. **Scope:** choose one basic component with an explicit acceptance criterion, parent TODO IDs, affected surfaces, dependencies and exclusions. Reproduce a defect before fixing it where feasible.
2. **Create/implement:** complete the behavior, including the engine API and CLI/MCP tool and end-to-end test when adding a user-facing function. Reuse shared helpers; no duplicate or placeholder implementations.
3. **Audit:** review correctness, data loss, malformed input, resource limits, path/network security, platform behavior, accessibility, localization, interoperability and undo/save consequences as applicable.
4. **Fix:** resolve findings; do not knowingly push a broken function, introduce a panic or weaken a security boundary.
5. **Audit again:** repeat the review against the fix, its tests and its callers. New tests must fail on the unfixed behavior where practical.
6. **Test:** run component regressions and relevant format/clippy/workspace/wasm checks. A component is not complete based only on lint or an evidence path existing. State exactly which gates ran and which are blocked.
7. **Push:** commit to and push only `arena/25355b3c-linkco-pdf`. Keep unrelated work out of the commit. The push does not authorize merge, publication, deployment or a production-readiness claim.
8. **Verify GitHub CI:** examine relevant checks for that implementation commit. A pending/failed check is not a pass. If a regression is found, fix and re-audit it before marking completion; do not hide it by weakening the test.
9. **Record:** update component evidence, exact CI run links, limitations and parent status. A small component does **not** complete its whole workstream or related gap automatically.
10. **Continue within scope:** record verified results and continue the authorized basic batch. Ask before changing stage or taking an action requiring a separate user decision. A batch report must distinguish completed components from the unfinished basic queue.

Finish the relevant basic queue before advancing to advanced, then future. Do not automatically jump stages. Items blocked on credentials, legal decisions, hardware, proprietary services or architecture choices stay visibly blocked until resolved; absence of evidence is not completion. Intentional exclusions/waivers need an explicit user decision and a recorded reason, not a silent checkbox.

“Without errors” is the quality objective, not a promise that software has no unknown defects. Known failures, missing acceptance coverage and unsupported platform/workflow claims must remain visible. If a required gate cannot be run, stop short of claiming that component complete. Never push a known-failing or deliberately incomplete implementation as a completed function.

## Verified component: B001 — staging filename confinement

Parent: **REC-R01, save integrity**. Existing callers: desktop `write_atomically` and automation `write_atomic`, reached through the existing Save/`doc_save` functionality.

The shared staging helper already used exclusive creation, random suffixes and an attempt limit. The first audit demonstrated that a path-shaped `name` could nevertheless escape its selected directory if a matching intermediate directory existed. B001 rejects non-filenames **before truncation, suffix generation or filesystem access**. It also rejects Windows alternate-stream names while preserving legal Unix backslashes and Unicode filenames.

Acceptance includes both name formats, empty/dot/absolute/traversal/NUL inputs, long-path truncation cases, normal Unicode/hidden filenames and Windows-specific path/stream cases. Existing collision and attempt-limit behavior must remain unchanged.

**Not claimed:** full filesystem sandboxing against hostile parent-directory changes, complete crash/power-loss durability, scanner/printer/hardware acceptance, or completion of all R01. The live record distinguishes local tests from full CI. All B001 replacement gates passed at `da8b5de`; the user’s conditional continuation approval now permits B002.

Verification: main Linux/Windows/wasm CI, Windows ARM64, FreeBSD build/test/package, packaging lint and preview-handler checks all passed for `da8b5de` (links in `progress.toml`). Serial FreeBSD execution isolated the large-memory renderer fixtures; quiet child-harness output repaired a CUPS mock that otherwise acquired a serial libtest prefix. No production renderer/printer code, security budget, deadline or existing assertion was weakened.

## Verified component: B002 — flush before replacing automation output

Parent: **REC-R01, save integrity**. Automation's atomic writer previously wrote, closed and renamed its staging file without flushing. Three deterministic regressions reproduced the missing flush: replacement despite an injected flush failure for existing/new targets, and absence of a flush-before-replace callback.

The writer now performs **write → flush → close → rename**. A write/flush failure closes the descriptor, attempts staging cleanup and returns an error before the target is replaced or `doc_save` marks the document saved. Desktop and automation share the existing desktop flush helper verbatim, including the macOS `ENOTSUP`/`ENOTTY` plain-`fsync` fallback; no production fallback policy is broadened. Root checks, name forms and unconfined stream-compatible `write_output` are unchanged.

Local evidence: the actual automation/staging functions pass 18 standalone tests; the actual desktop/staging functions pass 16. Both standalone modules pass Clippy with warnings denied, and workspace formatting passes. End-to-end coverage adds failed-save/retry preservation and checks that full/incremental saves leave no staging files. A macOS-only automation regression mirrors the desktop's supported-volume fallback test.

**Verified:** all five workflows passed on `8191bbd`: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 test/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). This includes the new end-to-end save/retry test. The existing workflow skipped mutation fuzzing; no new fuzz execution is claimed. Full local Cargo gates were blocked by the missing `craft-libs` checkout. Native macOS/SMB/FUSE execution, including the macOS-only tests, remains unverified; fallback preservation was checked against the prior source. This component is not a parent-directory sync, a full power-loss guarantee, protection against hostile parent-directory replacement, or completion of REC-R01. The user approved the next basic component on 2026-10-25.

## Verified component: B003 — private Unix staging

Parent: **REC-R01, save integrity**. The audit reproduced a default-mode staging file with permissions `0666` under umask `000`, before its first byte was written. Under a usual `022` umask the same default permits `0644`. Hidden/random names do not prevent another user reading a file when filesystem permissions allow it.

Unix creation now requests `0600` in the exclusive-creation syscall, **not** via a later chmod. Stricter umasks remain effective. Both naming schemes, collision refusal, flush/close/rename order, root confinement and unconfined stream output are unchanged. Both desktop saves and headless `doc_save`/root-confined `write_output` use the shared helper.

**Compatibility policy:** new and replaced Unix atomic outputs keep the restrictive staging mode. Existing group/other sharing permissions, ownership and extended ACLs are not copied. Windows retains its existing access-control behavior. This is not a guarantee against administrators, filesystems that ignore Unix modes, hostile parent changes or power loss.

Regressions check both formats before the first byte under child-process umasks `000`, `022`, `077` and `777`. No parent/global umask mutation is used; per-child receipts prevent a false pass from an empty test filter. Planted-file bytes and mode remain unchanged on collision. Desktop save coverage and an automation Save/Save As/confined-output/reopen integration test cover callers. **Verified at `fc5fd97`:** all five replacement workflows passed: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 tests/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). Linux/FreeBSD coverage includes the permission regression and headless save/output/reopen integration test. Local tests/Clippy/formatting also passed; full local workspace gates remained dependency-blocked. Existing workflow policy skipped mutation fuzzing; no fuzz execution is claimed. Native macOS/network-filesystem execution remains unverified. REC-R01 remains open. The user approved B004 on 2026-10-26.

## Verified component: B004 — controlled write/rename faults

Parent: **REC-R01, save integrity**. This audit found a testability gap, not a newly reproduced data-loss defect: write/rename errors were already propagated, but post-staging rename-denial coverage depended on Windows read-only-file behavior. Both pipelines now have **private** write/flush/rename seams; production still supplies standard `write_all`, the unchanged shared flush policy and `std::fs::rename`. No tool argument, environment switch or public fault-control API was added.

Three portable tests per caller exercise: actual partial staging bytes followed by an injected disk-full error (no flush/rename); successful write/flush followed by an injected rename denial; and a writer that first returns `Interrupted`, then writes at most two bytes per call until `write_all` finishes. Both failure cases cover existing and new targets, unchanged original bytes/target absence, returned errors and staging removal. The rename test checks order and, on Windows, exclusive reopening to prove the original write handle was closed. Existing tool save/error/retry integration coverage remains in force.

Local evidence: 23 automation/staging and 22 desktop/staging standalone tests, direct Clippy with warnings denied, formatting and four scratch error-swallowing mutation checks passed. **Verified at `db25dbb`:** all five replacement workflows passed: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 test/install, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). Native Windows coverage passed the exclusive-reopen handle-closure probe. Full local workspace gates remained dependency-blocked. Existing workflow policy skipped mutation fuzzing; the scratch mutation checks above are not a fuzz run. These are controlled operation faults using real temporary files, not physical disk-full, process-kill or power-loss tests. Native macOS/network filesystems and cleanup failures caused by unlink denial remain outside this evidence. REC-R01 is still open. The user approved B005 on 2026-10-26.

## Verified component: B005 — process termination at save boundaries

Parent: **REC-R01, save integrity**. Test-only additions use B004's private I/O seams; production code, tool arguments and save behavior are unchanged. Each caller launches real children for three stages and both existing/new targets (12 child terminations across the two tests):

- **Partial staging write:** kill while the child holds the writing descriptor. The target stays original/absent, and the partial staging file remains.
- **Flushed, before rename:** kill after flushing and closing the staging file. The target stays original/absent, and a complete staged file remains.
- **Immediately after rename:** kill before the save function returns. The target contains complete new bytes and no staging file remains.

Phase-specific file barriers and a bounded readiness wait prevent empty test-filter or stale-barrier false passes. `Child::kill` performs actual termination, with Unix SIGKILL status asserted; a guard kills/reaps on parent failures as well. Pre-rename orphans retain private Unix mode. A subsequent real save retries suffixes `[7, 8]`: it skips an occupied orphan name and preserves its contents rather than clobbering it.

Local evidence: 24 automation/staging and 23 desktop/staging standalone tests pass with serial and parallel execution; direct Clippy and formatting pass. Four scratch mutations that corrupt the target in place or reuse a staging orphan are rejected. **Verified at `4b3a22b`:** all five replacement workflows passed: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 tests/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). The workspace unit suites include both new child-termination tests. Full local Cargo gates remained blocked by the missing `craft-libs` checkout. Existing workflow policy skipped mutation fuzzing; the scratch mutation checks are not a fuzz run. These are byte-level tests on synthetic payloads, not PDF conformance, full UI/CLI restart/session recovery, physical volume-full, power-loss or native macOS/network-filesystem certification. Abrupt termination can leave staging files; automatic orphan recovery/cleanup is not implemented here. REC-R01 remains open. Stop and obtain approval before another component.

## Verified component: B006 — retry failed recovery autosaves

Parent: **REC-R01, save integrity**. The audit reproduced a real state-management bug: `autosave_snapshots` advanced the persisted-generation marker before the frontend attempted disk I/O. When recovery storage failed, later autosaves skipped the unchanged unsaved revision. The regression against the extracted actual selector failed before the fix.

Selection is now read-only and remains pending until `acknowledge_autosave(&snapshot)`. A private revision token and byte-identity check prevent an older acknowledgement from hiding a newer edit or regressing a successful acknowledgement. Closed/clean documents are ignored, documents are acknowledged independently, and successful autosaves do **not** mark user changes saved. The frontend acknowledges only after `RecoveryStore::write` succeeds for **both** PDF and metadata.

Real integration regressions cover blocked store directories, with/without an earlier stored revision, plain/encrypted PDFs, repeated failure followed by retry without another edit, unchanged original files and recovery of complete edited PDFs in a fresh app instance. Separate metadata-rename denial cases prove that writing PDF bytes alone does not acknowledge persistence. Existing engine snapshot tests now explicitly acknowledge successful storage; new tests cover unacknowledged retry, stale acknowledgements, undo/save/close and independent documents.

Local evidence: five standalone tests of actual extracted engine/UI methods with lightweight document/store stubs pass, direct Clippy and formatting pass, and three scratch mutations are rejected. **Verified at `7f77ac6`:** all five replacement workflows passed: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 tests/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). The full workspace suites include the actual engine acknowledgement regressions and real filesystem failure/retry/fresh-app recovery tests for plain/encrypted PDFs. Full local Cargo gates remained dependency-blocked; the standalone tests are not full-engine or filesystem/app-integration evidence. Existing workflow policy skipped mutation fuzzing; the scratch checks are not a fuzz run. These are failure/retry fixes to existing autosave, not a new tool or user-visible control. No automation/root/atomic-write behavior changes. PDF/metadata pair-transactionality, whole-process restart, physical volume-full, power loss, concurrent sessions, hostile recovery-store contents, orphan cleanup and web/native-macOS/network-filesystem certification remain outside scope. REC-R01 stays open. Stop for approval before another component.

## Verified component: B007 — recovery key safety and filename binding

Parent: **REC-R01, save integrity**. The audit reproduced a concrete path-handling defect in the actual store: keys were joined directly into entry paths, and JSON could identify a different key from its own filename. With disposable fixtures, five regressions failed before the fix: traversal could read/replace/delete outside files, `clear` followed a forged outside key, and misnamed metadata could impersonate another entry.

A private shared path helper now validates keys **before** read/write/remove filesystem access, including before creating the store directory. Keys are portable identifiers of 1–128 UTF-8 bytes, not paths: separators, streams, controls, Windows device aliases, unsupported filename punctuation and leading spaces/trailing spaces/dots are rejected. Generated time/PID/document IDs, normal Unicode identifiers and hidden/dotted identifiers remain valid. Invalid reads/writes return `InvalidInput`; the existing void `remove` API ignores invalid keys. Discovery requires the JSON filename to identify the same key-derived JSON file (native canonical-equivalent paths are allowed) **before** probing the corresponding PDF. Invalid JSON entries are removed only at their discovered path inside the store, under the existing incomplete-entry cleanup policy.

Local evidence: **17 actual-source tests** (six new recovery-store plus 11 staging) pass serially and in parallel with the real desktop atomic writer and real Serde/JSON at Cargo.lock versions; standalone Cargo Clippy with warnings denied and formatting pass. Five tests failed against the baseline; four scratch mutations are rejected. Only portable-mode directory selection is stubbed in the harness, and none of these tests use it. This is not GUI/engine integration evidence. A full-app regression adds plain/encrypted recovery and discard with forged traversal/alias metadata alongside a valid edited PDF, and checks outside bytes remain intact. **Verified at `6f19614`:** all five replacement workflows passed: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 tests/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). These workspace suites include the six new store regressions and full-app plain/encrypted recovery/discard coverage. Full local workspace gates remained dependency-blocked. Existing workflow policy skipped mutation fuzzing; the four scratch checks are not a fuzz run.

This fixes existing recovery-store behavior, not a new user control or automation tool. It is **lexical key validation and filename binding**, not protection against symlinks/hardlinks, hostile parents or concurrent directory/file replacement. Metadata contents and original-document paths are not authenticated; JSON/PDF loading is not newly resource-bounded. Pair-transactionality, automatic staging-orphan recovery, physical volume-full, power loss, web recovery and native macOS/network-filesystem certification remain outside this component. REC-R01 stays open. Stop for approval before another component.

## Verified component: B008 — retryable recovery offers and serialized password prompts

Parent: **REC-R01, save integrity**. The post-code audit of the recovery flow reproduced three defects. A failed read or a PDF the engine rejected still removed every requested key from the offer, so a damaged snapshot disappeared from the dialog until restart even though its files stayed in the store. Recovering more than one encrypted snapshot replaced the pending password prompt, so the earlier entry was already gone from the offer and its recovery association was lost. Recovery could also replace an unrelated document's pending password prompt.

Now only a successful open — or the password prompt that owns that document's bytes — consumes an offer. A read/open failure leaves the entry retryable, iteration stops while a prompt is pending so later entries stay on offer, and the dialog is re-shown when work remains. Cancelled or failed password recovery returns the association to the offer, deduplicated; wrong-password retries keep it. Duplicate requested keys no longer open duplicate tabs.

Local evidence: six tests of the actual `recover()` method pass, five of which failed before the fix; four new full-app regressions cover repair-and-retry after read/parse failure, duplicate keys, sequential encrypted recovery with wrong and cancelled passwords, and an unrelated pending prompt. Standalone Clippy and formatting pass. **Verified at `9ddec76`:** all five replacement workflows passed: main Linux/Windows Clippy and tests, wasm32, licences/advisories, native ARM64, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). Two intermediate commits failed the Clippy step on the new regressions; the lint fixes are in `35e3417`/`9ddec76` and the failures are recorded. Full local workspace gates remain dependency-blocked by the craft-libs checkout. The trade-off is explicit: encrypted snapshots are recovered one prompt at a time. PDF-load bounds, transactional pairs, power-loss durability and metadata authenticity remain open.

## Verified component: B009 — bounded recovery metadata

Parent: **REC-R01, save integrity**. Follow-up to B007: discovery still read each `<key>.json` whole with `std::fs::read` and accepted unbounded `name`/`path` strings, so a corrupt or hostile entry could allocate without limit and overlong strings flowed into the dialog and save path.

Metadata reads now stop after 64 KiB + 1 bytes and entries larger than 64 KiB are rejected; `name` and a present `path` must be at most 4,096 bytes. Rejected entries are cleaned up only at their discovered path inside the store, like other invalid metadata. Complete bounded entries — including a 4,096-byte native name — still list.

Local evidence: a new actual-source store test failed before the fix (padded 64 KiB+1 metadata, 4,097-byte name and path) and passes now, with 18 store/staging tests green and Clippy/formatting clean. Removing the acceptance checks is rejected by that test; removing only the read byte cap is not observable in tests and is recorded as defense-in-depth. **Verified at `9ddec76`** by the same five workflows; the store regression runs in the Linux, Windows and ARM64 suites and FreeBSD builds/tests the workspace. This bounds metadata only: PDF bytes and engine loading keep their existing limits, and authenticity, transactions, symlink/hostile-parent safety and power-loss durability remain open.

## Batch component: B010 — external file changes block a silent overwrite

Parent: **REC-R05, external-file change and save-conflict detection** (bounded; R05 stays open). There was no check at all: `save_doc_to` wrote over the document's path whatever had happened to that file meanwhile, so a version another program wrote disappeared without a word.

The app now stamps a file's contents (length plus a streamed 64-bit hash) when it opens a document from disk and again after each successful save. Saving in place compares the stamp with the file: if the bytes differ, nothing is written and a modal explains the conflict, offering **Save As…** (primary), **Cancel**, and **Overwrite**. Identical bytes are not a conflict, and a missing file is not one either (writing recreates it). The prompt is compiled out on wasm, where the in-place path save does not exist.

Local evidence: three standalone stamp tests pass (same-length different bytes; a 300 KiB file crossing read buffers; missing/unreadable paths), the module compiles with warnings denied, and formatting passes. The three full-app regressions (unchanged save, blocked save, overwrite, cancel-then-ask-again) pass on Linux and Windows. **Verified at `7a72435` by all five workflows.**

Not claimed: this is a check between two filesystem operations, not a lock — another program can still write between the check and the rename. Automation saves, rename/delete/move detection, idle-time notification and merge/reload choices remain open in REC-R05.

## Batch component: B011 — the recovery folder is private to its user

Parent: **REC-S08, secure temporary files, thumbnails and local history** (bounded; S08 stays open). Recovery snapshots were already written through the staging file, which is created `0600` before the first byte (B003). The folder holding them was not: `create_dir_all` plus the process umask produced `0755`, so another local account could list the entry names of work the user had not saved yet.

`pdfcraft_platform::private_dir::create_private_dir` now creates the folder with mode `0700` in the creation call itself, tightens a looser folder when the app opens the store, and refuses a path that is a link, a dangling link or a file instead of following or writing through it. The mode is changed through an open handle, so a swap between the check and the change cannot redirect it. Restricting a folder further is the only change it makes, and a failure to tighten is ignored on the write path so a filesystem without Unix modes cannot turn a permission bit into a failed autosave. Windows and macOS keep their per-user profile defaults.

Local evidence: six standalone helper tests pass (new folder, loosened folder tightened and still usable, missing parents, symlink, dangling symlink, file at the path), and formatting passes. The two store regressions (modes of the folder and both files; tightening on open; linked folder refused) pass on Linux and Windows, and the helper tests also run on ARM64 and FreeBSD. **Verified at `7a72435` by all five workflows.**

Not claimed: this is not a sandbox — replacing the folder or its parent between two filesystem operations is outside the component. Thumbnails, saved settings, printed output, local history, the portable data folder and the automation side are untouched, so REC-S08 stays open.

## Batch component: B012 — noticing a file renamed, moved or deleted while it is open

Parent: **REC-R05, external-file change and save-conflict detection** (second bounded part; R05 stays open). B010 stopped a save from overwriting a file another program changed, but only when the user saved. A file renamed or deleted while the document sat open stayed invisible, and the app remembered only a content stamp, so there was no way to tell a rename from a deletion.

`crates/ui-egui/src/disk_watch.rs` now remembers the file's identity (device and inode) and modification time beside the content stamp, and polls the open documents' files every three seconds, comparing contents once a minute and on every explicit check. It reports three situations once each: another program wrote the file (“…changed on disk…”), the file was moved or renamed inside its folder (“…was moved or renamed to…”), and the file is gone (“…is no longer at…; it was moved to another folder or deleted.”). Nothing is written, moved or reopened, nothing is said while a dialog is open, and an unchanged file is not even re-read.

Local evidence: nine standalone tests pass (untouched, different length, same-length rewrite, identical rewrite, deleted, renamed by identity, replaced file, missing folder) and the module clears clippy with warnings denied plus formatting. Two full-app regressions — rename then delete reported once each while the document stays open and still saves, and an idle document reporting an external write — run in CI, where both Windows jobs and Linux pass. **Verified at `9546a81` by all five workflows.**

Not claimed: identity needs Unix metadata, so elsewhere a rename is reported as a missing file; the poll reads only the document's own file and its folder, never the disk; this is a check between two filesystem operations, not a lock. Merge/reload choices, automation saves and notification while the app is closed stay open in REC-R05.

## Batch component: B013 — the recovery folder stays within a disk budget

Parent: **REC-R01** (save integrity and crash recovery) and **REC-S08** (local history hygiene), both still open. Snapshots were only deleted when the user saved, discarded or quit cleanly, and nothing measured the folder, so a crash followed by weeks of not opening the app, or a long session with many large scans, left snapshots nothing would ever read again.

`crates/ui-egui/src/recovery_budget.rs` is the decision, with no filesystem in it: stay within 200 entries and 512 MiB, deleting the least recently saved first, ties broken by key. The store asks it after every successful metadata write, reading each metadata file (already bounded to 64 KiB by B009) and its snapshot's size. The snapshot just written and the newest snapshot are never deleted and the folder is never emptied — a single snapshot larger than the whole budget may be the user's only copy of unsaved work, and the next save or clean quit removes it. Metadata goes before its snapshot, so an interrupted prune leaves a leftover the next listing cleans rather than an entry promising missing bytes. Housekeeping failures are ignored: an over-budget folder is better than a lost snapshot.

Local evidence: seven standalone decision tests pass (within budget, entry count, byte count, the just-written entry, never emptied, deterministic ordering, saturating sizes) and formatting passes. The store regression (a two-entry/2,500-byte budget evicting the oldest pair, then keeping a single over-budget snapshot) runs in CI. **Verified at `9546a81` by all five workflows.**

Not claimed: the budget is fixed rather than a preference, and only the recovery folder is pruned — thumbnails, the portable data folder and other local history stay in REC-S08. This does not make the snapshot/metadata pair transactional, so REC-R01 stays open.

## Batch component: B014 — the settings folder and its record of recent documents

Parent: **REC-S08, secure temporary files, thumbnails and local history** (bounded; S08 stays open). `app.ron` holds the preferences *and* the recent-document list — file names and full paths of what this person had open — and the log files in `logs/` name documents too. The settings folder came from the framework and the portable data folder from `create_dir_all`, both with the process umask, so `0755`/`0644` let another local account read that history. B011 had covered only the recovery folder.

`crates/ui-egui/src/settings_privacy.rs` runs once per launch, before anything uses the folder: it creates or re-tightens the folder `0700` (the same helper as the recovery folder), tightens an existing `logs/` the same way, and sets an existing `app.ron` to `0600` through an open handle after confirming the path is a regular file — a link there is left alone. Contents are never rewritten. The portable data folder is now created owner-only from the start. A mode that cannot be set is logged and never stops the app.

The settings file is written by the framework at exit, so its mode cannot be chosen at creation; the next launch tightens it. That one-launch gap is the documented trade-off — the folder itself is private from the first launch.

Local evidence: six standalone tests pass (fresh folder, existing `0644` file tightened with contents unchanged, `0755` log folder tightened, symlink left alone, missing file, unwritable folder) and formatting passes. The portable-mode assertion that its data folder is `0700` runs with the existing portable tests in CI. **CI is pending, blocked externally.** Two attempts (`2409227`, `04f3d57`) agree: CI (ubuntu, Windows, wasm, licences — including this component's new portable `0700` assertion), packaging lint and the preview handler pass; the Windows ARM64 job builds the ARM64 release and passes its PE checks and native tests; the FreeBSD job passes its build and full test steps. Both packaging jobs then fail in `cargo xtask models`, because the upstream OCR model host answers HTTP 403 for `text-detection.rten` — the same 403 comes back from this workspace, while the same URL worked earlier the same morning. No MSI, zip or tarball is produced for any commit, so release packaging is broken for the whole project until the host is fixed or the models are re-sourced; the model URL was deliberately left alone.

Not claimed: one launch of exposure for `app.ron` written by the framework; non-Unix platforms keep profile defaults; Unix modes do not stop another process running as the same user. Thumbnails, printed output and the rest of REC-S08 stay open.

## Batch component: B015 — stable exit codes and machine-readable failures

Parent: **REC-B02** (bounded; the versioned automation schemas stay open). Until now every command-line failure exited `1` and explained itself in English, so a script that wanted to tell a mistyped option from a damaged document had to parse prose. `apps/pdfcraft-cli/src/exit.rs` now names six kinds — `failed` 1, `usage` 2, `unsupported` 3, `malformed` 4, `refused` 5, `partial` 6 — with the numbers documented as stable and never renumbered.

`main` classifies each failure where it happens: the option check and every required argument and option value are `usage`, a document the renderer or the engine cannot open is `malformed`, a tool this build does not have is `unsupported`, and the control file the `ui` command refuses to trust is `refused`. Two cases are judged by what was actually done rather than where the failure arose: `text` reports `partial` only when some text reached stdout before pages failed (nothing printed stays `failed`), and a `--script` step that fails after earlier steps ran is `partial`, because their work stands.

`--error-format=json` is stripped before any command sees it and writes exactly one JSON line to standard error: `{"error":{"kind":"malformed","exit_code":4,"message":"…"}}`. The escaping is the JSON one and nothing else, so a file name with quotes cannot add a line or end the string. The default prose line is byte-identical to what it was, and a broken pipe on stdout still exits 0.

Local evidence: six standalone tests of the exit module pass with warnings denied, and formatting is clean. Everything that runs the real commands — including the new end-to-end test and the two updated expectations (an unusable option value now exits 2 instead of 1) — needs the engine and its dependencies, so it was verified remotely at `03b00c2`: ubuntu and windows built the crate, clippy passed with warnings denied, and the whole workspace test suite ran, the new tests included. Packaging lint and the preview handler passed too, and the Windows ARM64 and FreeBSD jobs passed their builds and tests. **CI stays pending** only because both packaging jobs stop in `cargo xtask models` — the upstream OCR model host returns HTTP 403 for `text-detection.rten` (the same external block recorded on B014), so no MSI, zip or tarball is produced for any commit.

Not claimed: versioned automation schemas; `refused` for a document that forbids an operation (a wrong password still reports `malformed`, since the engine does not type the difference); the MCP server, which reports errors inside its protocol.

## Regeneration and verification

```sh
python3 docs/implementation/build_backlog.py
python3 docs/implementation/build_backlog.py --check
python3 -m unittest discover -s docs/implementation -p 'test_*.py' -v
python3 -m unittest discover -s docs/research/scripts -p 'test_*.py' -v
```

The generator validates the inventory counts, unique IDs, component references, stage order and confirmation gate. Marking a component `complete` requires recorded passing CI and run links. Source-specific acceptance and audit evidence remain necessary even when the generator is green. Source/generated CSV text is escaped to avoid spreadsheet formula execution.
