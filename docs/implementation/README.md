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

## One-component workflow — remember this between sessions

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
10. **Stop and ask:** summarize the completed component and ask the user whether to continue to the next. **Do not begin that next component without their confirmation.**

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

## Regeneration and verification

```sh
python3 docs/implementation/build_backlog.py
python3 docs/implementation/build_backlog.py --check
python3 -m unittest discover -s docs/implementation -p 'test_*.py' -v
python3 -m unittest discover -s docs/research/scripts -p 'test_*.py' -v
```

The generator validates the inventory counts, unique IDs, component references, stage order and confirmation gate. Marking a component `complete` requires recorded passing CI and run links. Source-specific acceptance and audit evidence remain necessary even when the generator is green. Source/generated CSV text is escaped to avoid spreadsheet formula execution.
