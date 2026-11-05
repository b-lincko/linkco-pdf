# Release readiness — 2026-10-24

**Status: release candidate under validation, not approved for production.**

This hardening batch imports 23 selected upstream changes from
`storytold/pdfcraft` through `c894928e`; it is not a full synchronization of upstream.
The broader project remains alpha in [ROADMAP.md](../ROADMAP.md). Passing CI is
necessary, but does not establish rendering fidelity or suitability for every production workflow.

## Included in this batch

- Parser/resource hardening: large dictionary indexing, bounded filter chains, long-number
  recovery, Extended XMP chunk limits, large-real serialization and doubled-sign parsing.
- Rendering/text: shading alpha at paint time and column reading order across paragraph gaps.
- Signing: bounded DSS evidence decoding, strict CMS signed attributes, macOS issuer chains
  and a clearer Windows provider-loading error. Independent encryption test vectors are included.
- Forms/XFA: long FormCalc unit names, WordNum rounding limits, ISO time seconds,
  month/weekday names, hidden unnamed subform objects and JavaScript string escapes.
- Interchange/automation: zero XFDF annotation flags, indirect initial-view booleans,
  failure propagation from information-removal actions and the US #10 envelope description.
- Security: `ctutils 0.4.5`, `crypto-bigint 0.7.6`, `rustls-webpki 0.103.16`
  (RUSTSEC-2026-0382) and non-yanked `clipboard-win 5.4.3`.

The earlier Tools tab-strip port (#2244) remains reverted after the Linux find regression.
The linear-time `printd` patch (`2e9b247f`) mentioned in commit `f59f06d`'s message
was **not** applied; that commit contains 14 changes, not the 15 listed in its message.

## Release gates

Before publishing, verify the exact release commit, not an earlier green commit:

- [ ] CI: formatting, all-target Clippy, Windows tests and Linux tests with craft-fonts.
- [ ] CI: dependency licences, advisories, bans and source policy.
- [ ] CI: layering, attributed assets, CLI without MCP and WebAssembly checks.
- [ ] Platform workflows: Windows ARM64, preview handlers, FreeBSD and packaging lint.
- [ ] Release pipeline: build installers, verify signatures/notarization and retain checksums.
- [ ] Smoke-test the packaged app on each supported target: open/render, find text on
      unvisited pages, edit/save/reopen, undo/redo, forms, sign/verify and print/duplex.
- [ ] Test representative documents on copies; confirm no data loss or unacceptable
      rendering changes. Preserve a known-good installer and document backups for rollback.

CI evidence is attached to [PR #2](https://github.com/b-lincko/linkco-pdf/pull/2).
The dependency audit now publishes failures as annotations rather than hiding them in
container logs. Security policy is not weakened to make the check pass.

## Limits of this validation

There is no complete Cargo toolchain in the agent workspace, so compilation and tests run
in GitHub Actions; local checks include rustfmt, patch whitespace, manifest parsing and
SHA-256/licence-file verification of all 246 attributed repository assets.

The new macOS Keychain test is intentionally ignored because it modifies the user's
keychain search list temporarily. Run it explicitly on an isolated macOS test account;
it is not covered by Linux/Windows CI. No production release has been published or deployed
as part of this work. Signing secrets, release-environment approval and real printer/token
hardware validation remain release-owner responsibilities.
