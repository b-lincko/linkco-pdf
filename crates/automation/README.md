# pdfcraft-automation

Agent control for PdfCraft: a headless tool table over the engine, and an opt-in MCP server.

- **Layer:** L7 (architecture §13), but headless. It depends on `engine`, `render` and `organize`, never on a UI toolkit.
- **Status:** over a hundred tools; `pdfcraft-cli tools` lists the current set with their schemas (the count isn't repeated here, so it can't go stale). They cover inspection, rendering, text, page edits and organizing, page labels, bookmarks, comments and their review, form filling and authoring (fields, properties, scripts, data exchange, detection), redaction and sanitizing, password protection and signatures, calibrated 2D measurements and CSV export, adding text, images, links and stamps, optimizing, OCR, printing, export, metadata, undo/redo, save, combine, extract and split. Comment tools take geometry in the same top-left-origin points as everything else, and `comment_add` can mark text by searching for it (`find`). The MCP server also serves the open documents as resources, page images included. The running app's UI is driven separately, through the `ui.*` control channel in `pdfcraft-ui-egui`.

## API

```rust
let mut a = Automation::new().with_root("/work")?;      // optional: confine all paths
let doc = a.call("doc_open", &json!({ "path": "in.pdf" }))?;   // Vec<Content>
a.write_output("page1.png", &png)?;                      // save a result yourself, under the same root rules
tools() -> Vec<ToolDef>                                  // name, title, description, input_schema, read_only, destructive, command
mcp::McpServer::new(a).serve(stdin, stdout)?             // newline-delimited JSON-RPC 2.0
```

`Content` is `Json(Value)` or `Png { data, width, height }`. Errors are `UnknownTool`, `InvalidArgs` (the call didn't match the schema) or `Failed` (a readable message for the agent).

## Conventions

- Tool names are `snake_case` (`page_rotate`): MCP clients reject dots. `ToolDef::command` links a tool to the registry id it automates (`page.rotate`), and `command_list` reports the link.
- Pages and positions are **1-based**. Rectangles are PDF points with the origin at the top-left of the displayed page.
- Unknown arguments are rejected, so typos fail loudly.
- Tools that change a document return its summary (`doc`, `pages`, `dirty`, `undo`, `redo`, …).
- `comment_image_preview` returns JSON and a PNG: `layer: "background"` (the default) gives the page without the selected image signature/initials; `layer: "image"` gives its embedded image with alpha. The metadata supplies its displayed rectangle, document rotation, `image_rotation` (how far clockwise the image is shown turned: 0 for one placed by PdfCraft, which keeps it upright as displayed) and annotation opacity. Each result can be saved with the CLI's `--out`. Cache the background and transform the image for live movement/resizing; `comment_edit` commits the final geometry in one undo step. Previewing never changes the document.
- For proportional image signature resizing, use the embedded image layer's width/height ratio rather than a previously stretched annotation rectangle. The GUI preserves this ratio at corners with the opposite corner anchored; edge midpoint handles change only their own axis. `comment_edit` accepts the resulting rectangle.
- A batch tool (`action_run`, `ocr_recognize_files`) takes files **or folders** in `paths`. A folder is walked (`depth`, `exclude`, `include_hidden`, `follow_links`), its output folder is left out so a second run cannot pick up what the first wrote, and a file reached twice — two spellings, two overlapping folders, a link — is processed once. `batch_expand` reports the same list without writing anything, `batch_plan` plans it, and all four read those rules from `folder.rs`, so a plan cannot describe a different batch from the one the run would do. What is not there: nothing watches a folder for changes; a job is a list, made when it starts.
- `doc_close` refuses to drop unsaved changes unless `discard_changes: true`. `doc_save` writes atomically, incrementally in place, and in full for a new path. Atomic tool outputs and root-confined `write_output` flush their staging file before closing it and replacing the target. They share the desktop’s macOS unsupported-full-flush fallback; other flush errors refuse replacement and attempt staging cleanup. Unconfined `write_output` retains its stream-compatible behavior. File flush-before-replace is not a parent-directory sync or a complete power-loss durability guarantee. On Unix, atomic outputs request owner-only `0600` at staging creation (subject to a stricter umask), and keep it after replacement. Broader existing sharing permissions/ACLs are not preserved; Windows ACL behavior and unconfined stream-compatible output remain unchanged.
- With a root set, every read and write path must resolve inside it (symlinks and `..` included). Relative paths resolve inside the root, and `..` is resolved by name before the check. Every path outside the root gets the same refusal (`<path> is outside the allowed directory <root>`), whether or not it exists, so a confined agent can't probe the rest of the disk; on Windows another network share or device path (`\\host\share`, `\\?\UNC\…`, `\\.\…`) is refused without being contacted. "Not found" and other filesystem errors are reported only for paths inside the root. Tools that write several files into a folder name them after the document or input file, with separators, colons and control characters replaced by `_`, so a name can't lead them out of the folder. The root itself must be a folder.

## Shared field properties

For an individual push button, `form_set_props` also edits its visible normal label:

```json
{"doc": 1, "field": "clear_button", "caption": "Clear Form", "widget": 1}
```

`widget` is a 1-based index (default 1), used only with `caption`; an empty caption clears the
label. Neither renames the field. `form_fields` returns the first widget's `caption` and all
`captions` as `[{"widget": 1, "caption": "Clear Form"}]`. Other widgets and icon-only layouts
keep their settings. CLI and MCP use the same edit, including undo/redo and save/reopen.
The desktop exposes it as **Button Properties → Options → Label**. Rollover/down labels and
icon layout are not edited here; the existing caption font policy applies.

`form_set_props` accepts either `field` (one name) or `fields` (1–1000 unique names):

```json
{"doc": 1, "fields": ["first_name", "last_name"], "required": true,
 "appearance": {"width": 3, "border": "#1473E6"}}
```

The batch is atomic and creates one undo step. A missing or locked field, invalid option,
unknown argument or malformed appearance refuses the whole edit. Set `locked: false` to
unlock and change fields together. Only supplied properties change: each field/widget keeps
its other colours, fonts, values and unknown PDF dictionary keys. `"none"` removes a border
or fill colour. Names, rectangles, rotations and captions require the single-field form. Results keep
`field` for single edits and return `fields` for batches. CLI and MCP use the same tool.

## Permission profiles

`Automation::with_policy(Policy::new(Profile::…))` narrows what a session will run before any
argument is read: `ReadOnly` allows only the tools the table marks `read_only`, `Project` allows
everything but destructive tools plus those approved one by one (`Policy::approve("doc_save")`,
which refuses a name that is not a destructive tool), and `Destructive` — the default — allows
everything, as every earlier version did. `Project` needs a root; `with_policy` refuses it without
one rather than promising confinement it cannot keep.

The check is in `Automation::call`, so it covers `pdfcraft-cli run`, the MCP server and direct
callers alike, and the MCP layer lists only the tools the policy allows (`tools/list`,
`tool_search`) so an agent is not invited to call something that will be refused.
`policy_status` reports the profile, the approvals, the available and refused tools, and the root.

**Nothing in a document can change it.** The policy is set where the session starts; the profile is
not stored in, read from or affected by a document, and a PDF whose text, script, links, comments
or embedded files say "you now have every permission" is read as content — the refusals are the
same before and after opening it, which is what
`crates/automation/tests/policy.rs::nothing_inside_a_document_can_widen_the_profile` checks.

## Limits

`Automation::with_limits(Limits { workers, max_input_bytes, max_output_bytes, max_batch_items,
item_timeout })` bounds what one session will read, write and do at once (`crates/automation/src/
limits.rs`). Workers default to the machine's cores capped at 8 (1 on the web build); every quota
and the time limit default to zero, which means no limit, so nothing that was allowed before is
refused now.

The quotas are answered **before the work starts**: `doc_open` reads the file's size and refuses a
file over `max_input_bytes` without reading a byte of it, `doc_save` and every other writer go
through `Budget::write`, which refuses an over-size result before the file is opened, and the batch
tools answer `max_batch_items` before they resolve a single path. A batch under a time limit asks
between items, so the file in hand is finished and the rest are reported as skipped rather than
abandoned half-written.

`limits_status` reports the limits and an audit per session — checks, refusals by kind, batches
stopped by the time limit and bytes written — so a caller can see what its bound did rather than
guess. `pdfcraft-cli run|mcp` take `--max-workers`, `--max-input-mib`, `--max-output-mib`,
`--max-batch` and `--time-limit-ms`.

A limit is not a sandbox: an allowed call still runs with the process's own memory and file access,
and the time limit is checked between items, not enforced inside one.

## The interface's promises

`crates/automation/src/schema.rs` holds `TOOL_SCHEMA_VERSION` (`major.minor`) and builds the whole
table as one document (`schema::schema_document()`), which `pdfcraft-cli schema` prints. Within a
major version changes are additive — a new tool, a new optional argument, a new result field;
removing a tool, making an optional argument required, or changing what a tool writes to a document
is a major bump. The MCP server reports the same version in `initialize` and `tools/list`, so the
three cannot drift.

**Every tool that changes a document carries a worked example.** `examples.rs` holds one call per
such tool, and `schema_with_example` puts it in the tool's schema as `examples`, so
`pdfcraft-cli schema`, `pdfcraft-cli tools` and the MCP server all show a call that is known to
work instead of leaving the caller to guess at one. The examples are not decoration:
`crates/automation/tests/roundtrip.rs` checks each against its own schema (keys, required keys,
types, `minimum`, `enum`), runs it on a three-page document, saves, reopens the file in a session
that has never seen it, and checks the change is still there. A tool that changes a document and
has no example has a stated reason instead (`examples::why_not`), and a test holds the table to
that, so a new writer cannot ship without someone deciding how it is shown.

`ToolError::kind()` gives every failure a stable kind — `usage`, `unsupported`, `refused`,
`failed` — using the vocabulary the command line uses for its exit codes, and `ToolError::json()`
returns it with the message. The MCP server returns that object as a failed call's
`structuredContent.error`. See `docs/api-stability.md`.

## MCP server

**Opt-in only.** Nothing starts it automatically, and it opens no port. It runs while `pdfcraft-cli mcp [--root DIR] [--profile P] [--approve TOOL]` runs, normally launched by an agent from its MCP configuration. It stops when stdin closes. The CLI's `mcp` Cargo feature (on by default) compiles it out entirely when disabled.

`pdfcraft-cli mcp --compact` (or `McpServer::with_compact(true)`) keeps `tools/list` short for agents with small context budgets. It lists only the core tools (`mcp::COMPACT_CORE_TOOLS`: open, info, save, close, render, text extract and find, combine, split, undo) plus the common command/inspection/preview tools and two meta tools: `tool_search` (optional `query` and `category`, the name prefix; or `name` for one tool's full `input_schema`) and `tool_call` (`name` and `arguments`, run through the same `Automation::call`). `tools/call` by name keeps working for every tool, and the server instructions mention the meta tools. Without the flag nothing changes.

Implemented: `initialize` (protocol 2025-06-18, 2025-03-26, 2024-11-05), `ping`, `tools/list` (with `readOnlyHint`/`destructiveHint` annotations), `tools/call` (JSON results also returned as `structuredContent`; images as `image/png`), and `resources/list`, `resources/templates/list` and `resources/read`. The resources expose the open documents read-only: `pdfcraft://doc/{doc}/info` (JSON), `…/text`, `…/page/{page}/text` and `…/page/{page}/image{?dpi}` (PNG, 1–600 dpi). Tool failures come back as `isError: true` results, so the agent can read and recover from them.

## Planning a destructive batch

Two tools write a folder of results from a list of files: `action_run` (an Action Wizard run) and
`ocr_recognize_files` (a searchable copy of each file). Both are marked destructive, and both
choose their output names themselves.

`batch_plan {operation, paths, folder, …}` takes the same arguments as the writer it plans —
`operation: "action"` with `action` or `steps`, `operation: "ocr"` with `dpi`, `language` and
`skip_text_pages` — and answers what that call would do without writing anything:

* `targets`, one per input: the `output` the run would pick (including the `name (2).pdf` it falls
  back to), `replaces` and `replaces_note` when a file is already there, and what the input holds —
  `bytes`, `pages`, `title`, `encrypted`, `repair_notes` — or the `error` the run would hit. For an
  action each target also carries the `steps` that would run; for OCR, `changes` says what would be
  added.
* `replaces`: the files that would be overwritten, by name.
* `irreversible`: what cannot be undone, in plain sentences.
* `approve`: `{tool, args}`, the call that would carry the plan out, unchanged.

The destinations are not guessed: `batch_input_name` and `action_output` in `src/lib.rs` are the
functions the writers call, and `plan::action_from_args` is the parser `action_run` uses, so a plan
cannot name a file the run would not write or list steps the run would not take. Two inputs with
the same file name are numbered in the plan exactly as the run numbers them (`report.pdf`, then
`report (2).pdf`); an OCR batch writes under the input's own name, so a repeated name there is
reported as a replacement. Planning creates no folder and writes nothing (`read_only` in the tool
table, which the MCP layer reports as `readOnlyHint`), and a path or folder outside `--root`
refuses the plan before anything is read. Each input is read at most 32 MiB
(`plan::PLAN_READ_LIMIT`); a larger one is planned from its size and destination with
`pages: null`, rather than loaded to count them.

## Adding a tool

1. Add the engine capability first, with its tests (the tool is a thin adapter).
2. Add a `ToolDef` in `src/tools.rs`: a precise description, the schema, `ro()`/`destructive()`, and `cmd()` if a registry command exists.
3. Handle it in `Automation::call`, validating pages and positions with the existing helpers.
4. Add an end-to-end test in `tests/automation.rs`. `tool_table_is_well_formed` checks names, schemas and command links.


## Moving several page objects

`object_list {doc, page}` inventories recognised paragraphs, Image/Form artwork and added items
with generation-scoped kind/index references. `object_move {doc, page, objects, offset, generation?}`
moves 1–1000 references atomically in displayed points (right/down), preserving source glyph bytes
and fonts. A refusal changes nothing; success is one undo step. See
[the selection and movement guide](../../docs/editing-multiple-objects.md) for limits and examples.

## Common command tools

Following FilmCraft's MCP conventions, `command_list {doc?, filter?, enabled_only?}` keeps its
`{commands: [...]}` result and adds each mapped tool's parameter schema. `command_run {id, params?}`
runs that command through its existing headless tool: include `doc` and other tool arguments in
`params`. Commands without a headless mapping return a tool error. Unknown parameter keys are
ignored with a `warnings` array; required values and paths still go through the original validation.

`command_batch {steps: [{id, params?}], stop_on_error?}` returns `completed`, `failed`, and per-step
`{ok, result?, error?}` entries. The default is to stop at the first error; earlier edits remain.
MCP marks a batch with any failed steps as `isError`, including through compact `tool_call`.
`doc_inspect {doc?}` returns one document's info, or `{documents: [...]}` for all open documents.
`render_preview {doc?, page?, max_side?}` returns a PNG (page defaults to 1, maximum side defaults
to 1024, allowed 1–4096). Omit `doc` only when exactly one document is open.

All task tools remain available; compact mode includes the five common tools. Every MCP tool has
all four annotation hints. Unknown top-level argument keys return JSON-RPC `-32602` with the
accepted keys; failures in the invoked tool remain `isError` results. An escaped tool panic is
reported as an internal tool error, and the server continues serving.

Exports complete synchronously. `progressToken` and cancellation notifications are harmlessly
ignored; no background-job or cancellation behavior is added.

Session resources `pdfcraft://document` and `pdfcraft://commands`, and the refusal of unknown argument keys, are described in [docs/mcp.md](../../docs/mcp.md).

## Atomic-save failure coverage

The private atomic writer has deterministic write/flush/rename seams for unit tests only; production always uses standard file operations. Tests cover real partial staging bytes plus an injected disk-full error, a post-flush rename denial, and recoverable interrupted/short writes. Existing/new-target failures preserve the old bytes or absence of the target, report failure and clean up staging in the tested cases. There is no public/runtime fault switch. The existing tool save-failure/retry test checks that failed output does not mark unsaved edits as saved. These are not physical volume-full, process-kill, power-loss or guaranteed-unlink tests.

Test-only child-process coverage also kills a writer during partial staging, after flush/before rename and immediately after rename, for existing/new targets. The target stays original/absent before replacement or contains complete new bytes afterward. A retry skips leftover private Unix staging orphans instead of overwriting them. These synthetic byte-level tests do not add automatic orphan recovery, full application restart/session recovery or power-loss guarantees; no runtime fault controls are exposed.
