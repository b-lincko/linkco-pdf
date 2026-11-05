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
$ pdfcraft-cli schema --schema-version 1.4
{ "schema_version": "1.4", "asked": { "schema_version": "1.4", "compatible": true }, … }
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
| `malformed` | A file named in the call is not a document that can be read: the parser rejects it — not a PDF at all, or damaged past what it can repair. Nothing about the call was wrong, and retrying it as written cannot help. |
| `refused` | The session's permission profile, or a resource limit, declines the call. Nothing was read and nothing was written. |
| `failed` | The tool ran and failed for any other reason: an I/O error, a document that is locked or has no pages, an engine refusal. |

A kind's meaning is never reused for something else, and a new kind is an addition: a client must
treat a kind it does not know as `failed` rather than assume it cannot happen within a major version.

`malformed` is not a guess from the message. The engine's own reason for refusing to open a file
decides it (`OpenError`): bytes that are not a readable document are `malformed`, while a document
that is locked by a password, one that holds no pages, one whose encryption this build cannot read,
and a failure of ours are all `failed`, each with a message that says which.

A batch plan reports the same reason before the run, as `targets[].problem` on a file it could not
open: `malformed`, `password`, `empty`, `unsupported` or `internal` — the plan's word for the
state, with only the first being a `malformed` call. So a plan and the run that follows it cannot
disagree about a file. The field arrived with schema version 1.3.

Still not typed: the per-file errors of a batch that ran (`action_run`, `ocr_recognize_files`) are
plain messages in the result list, not kinds; the call as a whole succeeds with some files reported.

## Approving a destructive batch

`batch_plan` writes nothing and hands back two things: the call that would carry the plan out
(`approve`), and an `approval` with a `token` — a digest of the batch the plan describes. Pass that
token as `approve_token` on the call, and the run carries out **the batch the token names**, or
refuses with `refused` and writes nothing:

* the same folder (a token is bound to the folder's path, not to how it was spelled);
* the same list (a file added to a folder that was walked, a file removed, other `paths`);
* the same settings (an action's steps, a recognition run's resolution, language and text-page
  handling);
* no output that has appeared since the plan said that name was free.

A call without a token is the call it always was: the argument is optional, and a run that is given
none answers `approved: false` so a caller can tell the two apart. The token is not a secret and not
a permission — it cannot let a call past the session's profile, and it does not decide whether a
batch *should* run. It answers one question, "is this the batch that was approved", for a plan
somebody read.

Showing that plan to a person and asking is the interface's job, and since B035 the interface does
it: Action Wizard and Recognize Text in Files plan the batch when the results folder is chosen, show
what it would write and what it would replace, and start nothing until that is answered — and the
answer is checked against the batch as it is at that moment, so a folder that changed under the
question is asked about again rather than run. That prompt is the app's; no tool, argument or
schema version changed with it.

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
