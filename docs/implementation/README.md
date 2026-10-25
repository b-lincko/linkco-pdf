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

## Current component: B002 — verified; awaiting next-component approval

Parent: **REC-R01, save integrity**. Automation's atomic writer previously wrote, closed and renamed its staging file without flushing. Three deterministic regressions reproduced the missing flush: replacement despite an injected flush failure for existing/new targets, and absence of a flush-before-replace callback.

The writer now performs **write → flush → close → rename**. A write/flush failure closes the descriptor, attempts staging cleanup and returns an error before the target is replaced or `doc_save` marks the document saved. Desktop and automation share the existing desktop flush helper verbatim, including the macOS `ENOTSUP`/`ENOTTY` plain-`fsync` fallback; no production fallback policy is broadened. Root checks, name forms and unconfined stream-compatible `write_output` are unchanged.

Local evidence: the actual automation/staging functions pass 18 standalone tests; the actual desktop/staging functions pass 16. Both standalone modules pass Clippy with warnings denied, and workspace formatting passes. End-to-end coverage adds failed-save/retry preservation and checks that full/incremental saves leave no staging files. A macOS-only automation regression mirrors the desktop's supported-volume fallback test.

**Verified:** all five workflows passed on `8191bbd`: main Linux/Windows/wasm/consistency/advisory gates, native ARM64 test/install checks, FreeBSD build/test/package, packaging lint and preview-handler tests (links in `progress.toml`). This includes the new end-to-end save/retry test. The existing workflow skipped mutation fuzzing; no new fuzz execution is claimed. Full local Cargo gates were blocked by the missing `craft-libs` checkout. Native macOS/SMB/FUSE execution, including the macOS-only tests, remains unverified; fallback preservation was checked against the prior source. This component is not a parent-directory sync, a full power-loss guarantee, protection against hostile parent-directory replacement, or completion of REC-R01. **Stop here and obtain explicit approval before another component.**

## Regeneration and verification

```sh
python3 docs/implementation/build_backlog.py
python3 docs/implementation/build_backlog.py --check
python3 -m unittest discover -s docs/implementation -p 'test_*.py' -v
python3 -m unittest discover -s docs/research/scripts -p 'test_*.py' -v
```

The generator validates the inventory counts, unique IDs, component references, stage order and confirmation gate. Marking a component `complete` requires recorded passing CI and run links. Source-specific acceptance and audit evidence remain necessary even when the generator is green. Source/generated CSV text is escaped to avoid spreadsheet formula execution.
