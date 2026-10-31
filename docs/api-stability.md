# The tool interface, and what will not change under you

This page is the promise a script or an agent can rely on. It is short on purpose: everything here
is checked by tests, and everything not here is free to change.

## Versions

There are two versions, and they answer different questions.

**`schema_version` (`major.minor`)** names the revision of the tool table: which tools exist, what
arguments they take, what fields they return. `pdfcraft-cli schema` prints the whole table with the
version at the top; the MCP server reports the same number in `initialize`
(`serverInfo.toolSchemaVersion`) and in `tools/list` (`_meta.schemaVersion`).

The version is bumped like this:

* **minor** — a tool is added, an argument is added and is optional, a result gains a field, a
  schema gains something a caller may ignore (a worked example, a description), a message gets
  better, `limits_status` or a similar report gains a counter. A caller that ignores what it does
  not know keeps working.
* **major** — a tool is removed or renamed, an optional argument becomes required, a documented
  result field is removed or changes meaning, or an existing tool starts doing something different
  to a document. Anything that breaks a caller that was working.

Ask the build whether it speaks your revision:

```
$ pdfcraft-cli schema --schema-version 1.1
{ "schema_version": "1.1", "asked": { "schema_version": "1.1", "compatible": true }, … }
```

A different major version is not compatible, and the command exits **3** (`unsupported`) while
still printing the document, so a script can check the version before it starts a job rather than
discovering the mismatch in the middle of one.

**Protocol version** is the MCP revision, negotiated at `initialize` from the versions the server
lists. It has nothing to do with `schema_version`.

## Exit codes

The numbers are part of the interface: `0` done, `1` failed, `2` usage, `3` unsupported,
`4` malformed, `5` refused, `6` partial. They are only ever added to, never reused, never
renumbered. `--error-format=json` reports a failure as exactly one line:

```
{"error":{"kind":"malformed","exit_code":4,"message":"in.pdf: not a PDF"}}
```

`partial` (6) means *some work happened*: inspect the results rather than retry the whole job.
`refused` (5) means the operation itself was declined — by the document's permissions, by the
session's permission profile, or by a resource limit — and nothing was changed.

## Failure kinds from the tools

The same vocabulary reaches the tool boundary. Every `ToolError` carries a kind
(`ToolError::kind()`), and the MCP server returns it as `structuredContent.error.kind` on a failed
call, so a client can branch on the kind instead of parsing the message:

| Kind | Raised when |
|---|---|
| `usage` | The arguments do not match the tool's schema, or a value is out of range. |
| `unsupported` | No tool has that name, or this build cannot do what was asked at all. |
| `refused` | The session's permission profile, or a resource limit, declines the call. Nothing was read and nothing was written. |
| `failed` | The tool ran and failed: an I/O error, a damaged document, an engine refusal. |

A kind's meaning is never reused for something else, and a new kind is an addition.

**Not typed yet:** `malformed` exists on the command line but not at the tool boundary — a damaged
document reaches an agent as `failed` with the engine's message, because the engine does not type
the difference. Adding it would be additive (a new kind a client may branch on) rather than a
break; it is not claimed here.

## Examples

Every tool that changes a document carries a worked example inside its schema (`examples`), and
the example is a call the test suite proves: it is valid against the schema, it changes the
document, and the change survives a save and a reopen. An example is documentation of a call that
works, not a promise that the values in it are the only ones worth using — replacing `doc: 1` with
your document is expected. Adding an example where there was none, or correcting one that was
wrong, is not a breaking change — the examples arrived with schema version 1.1.

## What is not part of the interface

The text of a message, the order of a list a tool did not promise to sort, the number of tools, and
the wording of a hint or an instruction. A script that matches on prose is relying on something
this page does not promise.

## How this is kept honest

* `crates/automation/tests/schema.rs` checks the versioned document against the tool table it
  describes: same tools, same schemas, unique names, no required argument missing from
  `properties`, and the documented kinds for every failure the tools can raise.
* `apps/pdfcraft-cli/tests/schema.rs` runs the real command: the document, the compatibility answer
  for the same and a different major version, and the usage error for a version that is not a
  version.
* `apps/pdfcraft-cli/tests/mcp_errors.rs` starts the real server and checks the same version in
  `initialize` and `tools/list`, and the kind on a refused, malformed and unknown call.
