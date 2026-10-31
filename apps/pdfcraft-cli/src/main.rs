//! pdfcraft-cli — headless PdfCraft.
//!
//! Usage: `pdfcraft-cli --help`, which prints `USAGE` below.
//!
//! `run` and `mcp` drive the same tool table (`pdfcraft-automation`). In `run`, values parse as
//! JSON when they can (`pages=[1,3]`, `degrees=90`) and are strings otherwise. A script runs its
//! steps in one session, so `doc_open` returns id 1, the next document id 2, and so on. A step's
//! `"out"` (like `--out`) saves its image there. With `--root DIR`, every path a tool or step
//! names, `"out"` included, must be inside DIR, and relative paths resolve inside it.
//!
//! The MCP server never starts on its own: it runs only when this command is launched (normally
//! by an agent configured to use it), talks only over stdio, and exits when stdin closes.
//!
//! `check` is the robustness harness: every file is opened, inspected and fully rendered in a
//! *separate child process* with a wall-clock timeout, so hangs, panics and aborts in any
//! dependency are observed and reported instead of taking the harness down.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

mod exit;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use pdfcraft_engine::export::ImageFormat;
use pdfcraft_render::{PageRenderer, RenderConfig, RenderRequest, RequestKind, inspect};

/// The `mcp` lines of [`USAGE`]: only a build with the `mcp` feature has the command.
#[cfg(feature = "mcp")]
macro_rules! mcp_usage {
    () => {
        "\
pdfcraft-cli mcp    [--root DIR] [--compact] [--profile P] [--approve TOOL …]
                                                           MCP server on stdin/stdout (opt-in)
                                                           --compact lists a core set of tools plus tool_search and tool_call
                                                           --profile read-only|project|destructive narrows what it may run
                                                           --approve a destructive tool by name, under --profile project
"
    };
}
#[cfg(not(feature = "mcp"))]
macro_rules! mcp_usage {
    () => {
        ""
    };
}

/// The full usage, printed by `--help` (also after a command, e.g. `render --help`).
const USAGE: &str = concat!(
    "\
pdfcraft-cli info   <file.pdf> [--password PW]            document summary as JSON
                                                          (includes any declared PDF/A or PDF/UA level)
pdfcraft-cli render <file.pdf> --page N [--dpi 96] --out x.png   (.png, .jpg, .tif or .pam)
pdfcraft-cli preview <file.pdf> [--page N] [--dpi 150 | --width PX] [--max-px 4096] --out x.png [--password PW]
pdfcraft-cli text   <file.pdf> [--page N]                  extracted text (pages separated by form feeds)
pdfcraft-cli edit   <in.pdf> --out out.pdf [--rotate 1,3:90] [--delete 2,4] [--move 5:1]
                      [--insert-blank 1] [--title T] [--author A] [--full]
pdfcraft-cli combine <a.pdf> <b.pdf> … --out combined.pdf
pdfcraft-cli extract <in.pdf> --pages 1,3,5 --out out.pdf
pdfcraft-cli split   <in.pdf> (--every N | --before 3,7) [--out-dir DIR]
pdfcraft-cli check  <files or dirs…> [--timeout 20] [--dpi 36] [--json out.json]
pdfcraft-cli tools                                       automation tools and their JSON Schemas
pdfcraft-cli schema                                  the tool table with its version [--schema-version V]
                                                           asks whether this build speaks V (exit 3 if not)
pdfcraft-cli run    <tool> [key=value …] [--root DIR] [--out image.png]
pdfcraft-cli run    --script steps.json [--root DIR]      [{\"tool\": \"doc_open\", \"args\": {…}}, …]
                      [--max-workers N] [--max-input-mib N] [--max-output-mib N] [--max-batch N] [--time-limit-ms N]
                                                           limits for this session (0 = no limit); limits_status reports them
",
    mcp_usage!(),
    "\
pdfcraft-cli ui     --control FILE <method> [key=value …] [--out shot.png]
                                                           drive a running app started with --control FILE

Every command takes --error-format=json, which reports a failure on stderr as one JSON line
({\"error\":{\"kind\":…,\"exit_code\":…,\"message\":…}}). Exit codes (stable, never renumbered):
0 done · 1 failed · 2 usage · 3 unsupported · 4 malformed document · 5 refused · 6 partial"
);

fn main() -> ExitCode {
    let (args, json_errors) = take_error_format(std::env::args().skip(1).collect());
    // `<command> --help` too; only right after the command, so a later `-h` (a value) still reaches it.
    let command = match args.get(1).map(String::as_str) {
        Some("--help" | "-h") => Some("--help"),
        _ => args.first().map(String::as_str),
    };
    let result = match command {
        Some("--help" | "-h" | "help") => stdout_line(format_args!("{USAGE}")),
        None => Err(usage(format!("usage:\n{USAGE}"))),
        Some("info") => info(&args[1..]),
        Some("render") => render(&args[1..]),
        Some("preview") => preview(&args[1..]),
        Some("text") => text(&args[1..]),
        Some("edit") => edit(&args[1..]),
        Some("combine") => combine(&args[1..]),
        Some("extract") => extract(&args[1..]),
        Some("split") => split(&args[1..]),
        Some("check") => check(&args[1..]),
        Some("check-one") => check_one(&args[1..]),
        Some("tools") => tools(),
        Some("schema") => schema(&args[1..]),
        Some("run") => run(&args[1..]),
        Some("ui") => ui(&args[1..]),
        #[cfg(feature = "mcp")]
        Some("mcp") => mcp(&args[1..]),
        #[cfg(not(feature = "mcp"))]
        Some("mcp") => Err(unsupported("mcp: this build leaves out the MCP server (built without the `mcp` feature)")),
        Some("--version") => version(),
        Some(other) => Err(usage(format!("unknown command '{other}'; see pdfcraft-cli --help"))),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(CliError::Stdout(e)) if e.kind() == std::io::ErrorKind::BrokenPipe => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e.report(json_errors)),
    }
}

/// Split the global `--error-format` option out of the arguments, before any command sees them.
///
/// It is accepted anywhere on the line because it decides how a failure is *reported*, not what
/// the command does. `json` turns failures into one JSON line on stderr; `text` is the default
/// and is accepted so a wrapper can be explicit. Anything else is left in place, where the
/// command's own option check refuses it.
fn take_error_format(args: Vec<String>) -> (Vec<String>, bool) {
    let mut kept = Vec::with_capacity(args.len());
    let mut json = false;
    let mut rest = args.into_iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--error-format=json" => json = true,
            "--error-format=text" => json = false,
            "--error-format" => match rest.next() {
                Some(value) if value == "json" => json = true,
                Some(value) if value == "text" => json = false,
                // Put the option back, so the command reports the value it cannot use.
                Some(value) => kept.extend([arg, value]),
                None => kept.push(arg),
            },
            _ => kept.push(arg),
        }
    }
    (kept, json)
}

// Keep stdout errors typed: a closed pipe is normal, but file and command errors still fail.
#[derive(Debug)]
enum CliError {
    /// A failure and the kind a script can act on (`exit`).
    Message(exit::Failure),
    Stdout(std::io::Error),
}

impl CliError {
    /// Report on stderr in the format asked for and return the exit code.
    fn report(&self, json: bool) -> u8 {
        match self {
            Self::Message(failure) => failure.report(json),
            Self::Stdout(error) => exit::Failure::new(exit::Kind::Failed, format!("stdout: {error}")).report(json),
        }
    }

    /// The same failure, reported as part of a larger piece of work that partly happened.
    fn into_partial(self) -> Self {
        match self {
            Self::Message(failure) => Self::Message(exit::Failure::new(exit::Kind::Partial, failure.message)),
            stdout => stdout,
        }
    }

    /// Prefix the message with what was being attempted — `step 2 (doc_open): …` — keeping the
    /// kind, so a script still sees why it failed.
    fn context(self, what: &str) -> Self {
        match self {
            Self::Message(failure) => Self::Message(exit::Failure::new(failure.kind, format!("{what}: {}", failure.message))),
            stdout => stdout,
        }
    }
}

/// The command line cannot be run as written: an unknown option, a missing argument, a value the
/// option does not accept. Nothing was written, so a script may fix the call and retry.
fn usage(message: impl Into<String>) -> CliError {
    CliError::Message(exit::Failure::new(exit::Kind::Usage, message))
}

/// The input is not a document that can be read.
fn malformed(message: impl Into<String>) -> CliError {
    CliError::Message(exit::Failure::new(exit::Kind::Malformed, message))
}

/// This build cannot do it at all (the tool does not exist, the feature was left out).
fn unsupported(message: impl Into<String>) -> CliError {
    CliError::Message(exit::Failure::new(exit::Kind::Unsupported, message))
}

/// Part of the work was done and reported; the rest failed.
fn partial(message: impl Into<String>) -> CliError {
    CliError::Message(exit::Failure::new(exit::Kind::Partial, message))
}

/// The tool declines to use something it cannot trust, such as a control file another user could
/// have written: the request is refused rather than carried out with it.
fn refused(message: impl Into<String>) -> CliError {
    CliError::Message(exit::Failure::new(exit::Kind::Refused, message))
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self::Message(exit::Failure::new(exit::Kind::Failed, message))
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        Self::Message(exit::Failure::new(exit::Kind::Failed, message))
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(failure) => write!(f, "{failure}"),
            Self::Stdout(error) => write!(f, "stdout: {error}"),
        }
    }
}

fn stdout_line(line: std::fmt::Arguments<'_>) -> Result<(), CliError> {
    let mut stdout = std::io::stdout().lock();
    writeln!(stdout, "{line}").and_then(|()| stdout.flush()).map_err(CliError::Stdout)
}

fn version() -> Result<(), CliError> {
    stdout_line(format_args!("pdfcraft-cli {} (Linkco PDF Editor)", env!("CARGO_PKG_VERSION")))?;
    stdout_line(format_args!("Source:  {}", pdfcraft_engine::links::GITHUB))?;
    Ok(())
}

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

/// Every value given to `name` (`--approve a --approve b`, or `--approve a,b`), in order.
#[cfg(feature = "mcp")]
fn values<'a>(args: &'a [String], name: &str) -> Vec<&'a str> {
    let mut out = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if arg == name
            && let Some(value) = args.get(i + 1)
        {
            out.extend(value.split(',').map(str::trim).filter(|v| !v.is_empty()));
        }
    }
    out
}

/// A number given to `flag`, checked against `range`; a bad value is a usage error before any file
/// is touched.
fn number(args: &[String], flag_name: &str, range: std::ops::RangeInclusive<u64>) -> Result<Option<u64>, CliError> {
    let Some(value) = flag(args, flag_name) else { return Ok(None) };
    let n: u64 =
        value.parse().map_err(|_| usage(format!("{flag_name} needs a whole number from {} to {}, not {value}", range.start(), range.end())))?;
    if !range.contains(&n) {
        return Err(usage(format!("{flag_name} {n} is out of range: {} to {}", range.start(), range.end())));
    }
    Ok(Some(n))
}

/// The session's limits from its flags: workers, input and output size, batch size and a time
/// limit. Every one is opt-in, so a command that says nothing gets the defaults.
fn limits(args: &[String]) -> Result<pdfcraft_automation::Limits, CliError> {
    let mut limits = pdfcraft_automation::Limits::default();
    if let Some(n) = number(args, "--max-workers", 1..=1024)? {
        limits.workers = n as usize;
    }
    if let Some(n) = number(args, "--max-input-mib", 0..=1_048_576)? {
        limits.max_input_bytes = n.saturating_mul(pdfcraft_automation::MIB);
    }
    if let Some(n) = number(args, "--max-output-mib", 0..=1_048_576)? {
        limits.max_output_bytes = n.saturating_mul(pdfcraft_automation::MIB);
    }
    if let Some(n) = number(args, "--max-batch", 0..=1_000_000)? {
        limits.max_batch_items = n as usize;
    }
    if let Some(n) = number(args, "--time-limit-ms", 0..=86_400_000)? {
        limits.item_timeout = std::time::Duration::from_millis(n);
    }
    Ok(limits)
}

#[derive(Clone, Copy)]
struct OptionSpec {
    name: &'static str,
    takes_value: bool,
}

/// Refuse a long option `command` doesn't take (a misspelling) before any file is read.
fn validate_options(command: &str, args: &[String], specs: &[OptionSpec]) -> Result<(), CliError> {
    let mut i = 0;
    while let Some(arg) = args.get(i) {
        if let Some(name) = arg.strip_prefix("--") {
            let Some(spec) = specs.iter().find(|spec| spec.name == arg) else {
                return Err(usage(format!("{command}: unknown option --{name}")));
            };
            // A missing or malformed value is left to the command, which reports it in its own
            // terms (`text` and `extract` explain what a page value must be).
            i += if spec.takes_value { 2 } else { 1 };
        } else {
            i += 1;
        }
    }
    Ok(())
}

const INFO_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--password", takes_value: true }];
const TEXT_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--page", takes_value: true }, OptionSpec { name: "--password", takes_value: true }];
const COMBINE_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--out", takes_value: true }];
const EXTRACT_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--out", takes_value: true },
    OptionSpec { name: "--pages", takes_value: true },
    OptionSpec { name: "--password", takes_value: true },
];
const SPLIT_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--every", takes_value: true },
    OptionSpec { name: "--before", takes_value: true },
    OptionSpec { name: "--out-dir", takes_value: true },
    OptionSpec { name: "--password", takes_value: true },
];
const RENDER_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--page", takes_value: true },
    OptionSpec { name: "--dpi", takes_value: true },
    OptionSpec { name: "--out", takes_value: true },
    OptionSpec { name: "--password", takes_value: true },
];
const CHECK_ONE_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--dpi", takes_value: true }, OptionSpec { name: "--edit", takes_value: false }];
const CHECK_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--timeout", takes_value: true },
    OptionSpec { name: "--dpi", takes_value: true },
    OptionSpec { name: "--json", takes_value: true },
];
const RUN_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--root", takes_value: true },
    OptionSpec { name: "--script", takes_value: true },
    OptionSpec { name: "--out", takes_value: true },
    OptionSpec { name: "--max-workers", takes_value: true },
    OptionSpec { name: "--max-input-mib", takes_value: true },
    OptionSpec { name: "--max-output-mib", takes_value: true },
    OptionSpec { name: "--max-batch", takes_value: true },
    OptionSpec { name: "--time-limit-ms", takes_value: true },
];
const UI_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--control", takes_value: true }, OptionSpec { name: "--out", takes_value: true }];

#[cfg(feature = "mcp")]
const MCP_OPTIONS: &[OptionSpec] = &[
    OptionSpec { name: "--root", takes_value: true },
    OptionSpec { name: "--compact", takes_value: false },
    OptionSpec { name: "--profile", takes_value: true },
    OptionSpec { name: "--approve", takes_value: true },
    OptionSpec { name: "--max-workers", takes_value: true },
    OptionSpec { name: "--max-input-mib", takes_value: true },
    OptionSpec { name: "--max-output-mib", takes_value: true },
    OptionSpec { name: "--max-batch", takes_value: true },
    OptionSpec { name: "--time-limit-ms", takes_value: true },
];

fn positional(args: &[String]) -> Vec<&str> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if a.starts_with("--") {
            skip = true;
            continue;
        }
        out.push(a.as_str());
    }
    out
}

fn read(path: &str) -> Result<Arc<Vec<u8>>, String> {
    std::fs::read(path).map(Arc::new).map_err(|e| format!("{path}: {e}"))
}

fn info(args: &[String]) -> Result<(), CliError> {
    validate_options("info", args, INFO_OPTIONS)?;
    let path = *positional(args).first().ok_or_else(|| usage("info: missing file"))?;
    let bytes = read(path)?;
    let password = flag(args, "--password");
    let info = inspect(bytes.clone(), password).map_err(|e| malformed(e.to_string()))?;
    // Dynamic XFA forms: say what laying the template out gives (what the app shows).
    let xfa_layout = (info.xfa == Some(pdfcraft_render::Xfa::Dynamic))
        .then(|| {
            let mut s = pdfcraft_engine::Session::new();
            let id = s.open("info.pdf", None, bytes.clone(), password).ok()?;
            let d = s.get(id)?;
            d.xfa.as_ref().map(|x| serde_json::json!({ "pages": x.pages, "fields": x.fields, "warnings": x.warnings }))
        })
        .flatten();
    // What the document declares as a standard, read the way the Standards panel reads it, so
    // `info` and the app agree. A file that declares nothing leaves these null and empty.
    let declared =
        pdfcraft_cos::Document::open_with_password(bytes.clone(), password).map(|doc| pdfcraft_preflight::declared(&doc)).unwrap_or_default();
    let json = serde_json::json!({
        "file": path,
        "pdf_version": info.pdf_version,
        "file_size": info.file_size,
        "pages": info.pages.len(),
        "first_page_pt": info.pages.first().map(|p| [p.width, p.height]),
        "title": info.title, "author": info.author, "producer": info.producer, "creator": info.creator,
        "encrypted": info.encrypted, "tagged": info.tagged, "javascript": info.has_javascript,
        "pdfa": declared.pdfa.as_ref().map(|(part, conformance)| format!("PDF/A-{part}{}", conformance.to_lowercase())),
        "pdfua": declared.pdfua,
        "output_intents": declared.output_intents,
        "xfa": info.xfa.map(|x| match x { pdfcraft_render::Xfa::Static => "static", pdfcraft_render::Xfa::Dynamic => "dynamic" }),
        "xfa_layout": xfa_layout,
        "bookmarks": info.outline.len(), "annotations": info.annotations.len(), "fields": info.fields.len(),
        "links": info.links.len(), "layers": info.layers.len(), "attachments": info.attachments.len(),
        "custom_properties": info.custom.len(),
        "page_labels": info.pages.iter().take(8).map(|p| p.label.clone()).collect::<Vec<_>>(),
        "warnings": info.warnings,
    });
    stdout_line(format_args!("{}", serde_json::to_string_pretty(&json).unwrap_or_default()))?;
    Ok(())
}

fn text(args: &[String]) -> Result<(), CliError> {
    validate_options("text", args, TEXT_OPTIONS)?;
    let path = *positional(args).first().ok_or_else(|| usage("text: missing file"))?;
    let mut selected_page = None;
    // Validate every supplied value before reading, retaining the first valid selection.
    let mut options = args.iter();
    while let Some(option) = options.next() {
        if option == "--page" {
            let page = options
                .next()
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|page| *page > 0)
                .ok_or_else(|| usage("bad --page: expected a positive page number"))?;
            selected_page.get_or_insert(page - 1);
        } else if option.starts_with("--") {
            // Match positional(): an option's operand is not itself another option.
            options.next();
        }
    }
    let password = flag(args, "--password");
    let bytes = read(path)?;
    let mut r = PageRenderer::new(bytes.clone(), RenderConfig { password: password.map(Arc::from), ..Default::default() });
    // A document the renderer could not open (wrong or missing password, unparseable file) has
    // no pages, and "extract every page" of nothing would print nothing and exit 0 (#132). Ask
    // `inspect` why instead, so a protected file fails the same way `info` does.
    if r.page_count() == 0 {
        let info = inspect(bytes, password).map_err(|e| malformed(format!("text: {e}")))?;
        if !info.pages.is_empty() {
            return Err(malformed("text: the document could not be parsed"));
        }
    }
    let pages: Vec<usize> = match selected_page {
        Some(page) => vec![page],
        None => (0..r.page_count()).collect(),
    };
    let mut failed: Vec<usize> = Vec::new();
    let mut printed = 0usize;
    for (n, p) in pages.iter().enumerate() {
        let out = r.render(RenderRequest { page: *p, kind: RequestKind::Text, tile: None, scale: 1.0, tag: 0 });
        if let Some(e) = out.error {
            let _ = writeln!(std::io::stderr().lock(), "page {}: {e}", p + 1);
            failed.push(p + 1);
            continue;
        }
        if n > 0 {
            stdout_line(format_args!("\u{c}"))?;
        }
        stdout_line(format_args!("{}", out.text.map(|t| t.plain_text()).unwrap_or_default()))?;
        printed += 1;
    }
    let unreadable = failed.iter().map(usize::to_string).collect::<Vec<_>>().join(", ");
    if failed.is_empty() {
        Ok(())
    } else if printed > 0 {
        // Some text reached the caller and some pages failed: a partial result, not a failure.
        Err(partial(format!("text: page(s) {unreadable} could not be read")))
    } else {
        // Nothing was printed, so nothing was half-done.
        Err(format!("text: page(s) {unreadable} could not be read").into())
    }
}

/// 1-based page list ("1,3,5") → 0-based indices.
fn page_list(s: &str) -> Result<Vec<usize>, CliError> {
    s.split(',')
        .map(|p| p.trim().parse::<usize>().ok().filter(|n| *n > 0).map(|n| n - 1).ok_or_else(|| usage(format!("bad page number {p:?}"))))
        .collect()
}

/// The options `edit` understands, for its "unknown option" message.
const EDIT_OPTIONS: &str = "--out, --password, --full, --rotate, --delete, --move, --insert-blank, --title, --author";

/// Apply page and metadata edits through the engine and save (incrementally unless `--full`).
///
/// Every argument is parsed before the file is opened: an option this command does not know
/// (a typo like `--rotat`) is an error, not something to skip — skipping it would write an
/// unchanged copy and exit 0, which looks like a successful edit (#133).
fn edit(args: &[String]) -> Result<(), CliError> {
    use pdfcraft_engine::{Edit, Session};
    let mut path: Option<&str> = None;
    let mut out: Option<&str> = None;
    let mut password: Option<&str> = None;
    let mut full = false;
    let mut edits = Vec::new();
    let mut i = 0;
    while let Some(arg) = args.get(i).map(String::as_str) {
        // Options that take no value, and the input file.
        match arg {
            "--full" => {
                full = true;
                i += 1;
                continue;
            }
            a if a.starts_with("--") => {}
            a => {
                if path.is_some() {
                    return Err(format!("edit: unexpected argument {a:?} (one input file, then options)").into());
                }
                path = Some(a);
                i += 1;
                continue;
            }
        }
        let value = args.get(i + 1).map(String::as_str).ok_or_else(|| format!("edit: {arg} needs a value"))?;
        match arg {
            "--out" => out = Some(value),
            "--password" => password = Some(value),
            "--rotate" => {
                let (pages, deg) = value.split_once(':').ok_or_else(|| usage("--rotate PAGES:DEGREES"))?;
                edits.push(Edit::RotatePages { pages: page_list(pages)?, degrees: deg.parse().map_err(|_| usage("bad degrees"))? });
            }
            "--delete" => edits.push(Edit::DeletePages { pages: page_list(value)? }),
            "--move" => {
                let (pages, to) = value.split_once(':').ok_or_else(|| usage("--move PAGES:TO"))?;
                let to: usize = to.parse().map_err(|_| usage("bad target"))?;
                edits.push(Edit::MovePages {
                    pages: page_list(pages)?,
                    to: to.checked_sub(1).ok_or_else(|| usage("--move target must be at least 1"))?,
                });
            }
            "--insert-blank" => {
                let at: usize = value.parse().map_err(|_| usage("bad position"))?;
                edits.push(Edit::InsertBlankPage {
                    at: at.checked_sub(1).ok_or_else(|| usage("--insert-blank must be at least 1"))?,
                    width: 612.0,
                    height: 792.0,
                });
            }
            "--title" => edits.push(Edit::SetInfo { key: "Title".into(), value: value.into() }),
            "--author" => edits.push(Edit::SetInfo { key: "Author".into(), value: value.into() }),
            other => return Err(format!("edit: unknown option {other:?} (expected one of: {EDIT_OPTIONS})").into()),
        }
        i += 2;
    }
    let path = path.ok_or_else(|| usage("edit: missing file"))?;
    let out = out.ok_or_else(|| usage("edit: missing --out"))?;
    let mut session = Session::new();
    let id = session.open(path, Some(path.to_string()), read(path)?, password).map_err(|e| malformed(e.to_string()))?;
    for e in edits {
        session.apply(id, e).map_err(|e| e.to_string())?;
    }
    let bytes = if full { session.save_full_bytes(id) } else { session.save_bytes(id) }.map_err(|e| e.to_string())?;
    std::fs::write(out, bytes.as_slice()).map_err(|e| format!("{out}: {e}").into())
}

fn file_stem(path: &str) -> String {
    Path::new(path).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| path.to_string())
}

fn combine(args: &[String]) -> Result<(), CliError> {
    validate_options("combine", args, COMBINE_OPTIONS)?;
    let out = flag(args, "--out").ok_or_else(|| usage("combine: missing --out"))?;
    let inputs = positional(args);
    if inputs.len() < 2 {
        return Err(usage("combine: give at least two input files"));
    }
    let sources = inputs.iter().map(|p| Ok((file_stem(p), read(p)?))).collect::<Result<Vec<_>, String>>()?;
    let bytes = pdfcraft_engine::Session::new().combine(&sources).map_err(|e| e.to_string())?;
    std::fs::write(out, bytes.as_slice()).map_err(|e| format!("{out}: {e}").into())
}

fn extract(args: &[String]) -> Result<(), CliError> {
    validate_options("extract", args, EXTRACT_OPTIONS)?;
    let path = *positional(args).first().ok_or_else(|| usage("extract: missing file"))?;
    let out = flag(args, "--out").ok_or_else(|| usage("extract: missing --out"))?;
    let pages = page_list(flag(args, "--pages").ok_or_else(|| usage("extract: missing --pages"))?)?;
    let mut session = pdfcraft_engine::Session::new();
    let id = session.open(path, None, read(path)?, flag(args, "--password")).map_err(|e| malformed(e.to_string()))?;
    let bytes = session.extract(id, &pages).map_err(|e| e.to_string())?;
    std::fs::write(out, bytes.as_slice()).map_err(|e| format!("{out}: {e}").into())
}

fn split(args: &[String]) -> Result<(), CliError> {
    validate_options("split", args, SPLIT_OPTIONS)?;
    use pdfcraft_engine::SplitBy;
    let path = *positional(args).first().ok_or_else(|| usage("split: missing file"))?;
    let by = match (flag(args, "--every"), flag(args, "--before")) {
        (Some(n), None) => SplitBy::PageCount(n.parse().map_err(|_| usage("bad --every"))?),
        (None, Some(list)) => SplitBy::Before(page_list(list)?),
        _ => return Err(usage("split: give either --every N or --before PAGES")),
    };
    let dir = PathBuf::from(flag(args, "--out-dir").unwrap_or("."));
    let mut session = pdfcraft_engine::Session::new();
    let id = session.open(path, None, read(path)?, flag(args, "--password")).map_err(|e| malformed(e.to_string()))?;
    let stem = file_stem(path);
    let parts = session.split(id, &by).map_err(|e| e.to_string())?;
    // An output folder that does not exist yet is created (#249); one that can't be is reported
    // as the folder's problem, not as the first part's.
    std::fs::create_dir_all(&dir).map_err(|e| format!("split: --out-dir {}: {e}", dir.display()))?;
    for (a, b, bytes) in parts {
        let name = dir.join(if a == b { format!("{stem}-p{a}.pdf") } else { format!("{stem}-p{a}-{b}.pdf") });
        std::fs::write(&name, bytes.as_slice()).map_err(|e| format!("{}: {e}", name.display()))?;
        stdout_line(format_args!("{}", name.display()))?;
    }
    Ok(())
}

fn render(args: &[String]) -> Result<(), CliError> {
    validate_options("render", args, RENDER_OPTIONS)?;
    let path = *positional(args).first().ok_or_else(|| usage("render: missing file"))?;
    let page: usize = flag(args, "--page").unwrap_or("1").parse().map_err(|_| usage("bad --page"))?;
    let page_index = page.checked_sub(1).ok_or_else(|| usage("--page must be at least 1"))?;
    let dpi: f32 = flag(args, "--dpi").unwrap_or("96").parse().map_err(|_| usage("bad --dpi"))?;
    let out = flag(args, "--out").ok_or_else(|| usage("render: missing --out (.png, .jpg, .tif or .pam)"))?;
    // The file is what its name says (#248): a `.png` used to get a netpbm PAM stream.
    let format = match Path::new(out).extension().and_then(|e| e.to_str()).map(str::to_ascii_lowercase).as_deref() {
        Some("png") => Some(ImageFormat::Png),
        Some("jpg" | "jpeg") => Some(ImageFormat::Jpeg { quality: 90 }),
        Some("tif" | "tiff") => Some(ImageFormat::Tiff),
        Some("pam") => None,
        _ => return Err(format!("render: --out {out}: use a .png, .jpg, .tif or .pam name").into()),
    };
    let mut r = PageRenderer::new(
        read(path)?,
        RenderConfig { password: flag(args, "--password").map(Arc::from), reject_oversize: true, ..Default::default() },
    );
    let p = r.render(RenderRequest { page: page_index, kind: RequestKind::Pixels, tile: None, scale: dpi / 72.0, tag: 0 });
    if let Some(e) = p.error {
        return Err(e.into());
    }
    let bytes = match format {
        // With `reject_oversize` the page is drawn at `--dpi` or not at all, so the file can say so.
        Some(f) => pdfcraft_engine::export::encode_image(p.width, p.height, &p.rgba, f, f64::from(dpi))?,
        None => {
            // PAM (netpbm RGB_ALPHA): the raw premultiplied pixels, for tools that read it.
            let mut pam = format!("P7\nWIDTH {}\nHEIGHT {}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n", p.width, p.height).into_bytes();
            pam.extend_from_slice(&p.rgba);
            pam
        }
    };
    std::fs::write(out, bytes).map_err(|e| format!("{out}: {e}"))?;
    let _ = writeln!(std::io::stderr().lock(), "rendered page {page} at {dpi} dpi: {}×{} px in {} ms", p.width, p.height, p.millis);
    for warning in &p.warnings {
        match warning {
            pdfcraft_render::RenderWarning::ContentTruncated => {
                let _ = writeln!(std::io::stderr().lock(), "warning: page {page}: content past the page's safety budget was skipped");
            }
        }
    }
    Ok(())
}

/// Fast single-page preview helper used by the Windows File Explorer Preview Handler (`IPreviewHandler`).
/// Emits a single structured `STATUS\t...` line to stdout so the preview host can distinguish
/// valid PDFs, password-protected PDFs, empty PDFs, and damaged files without opening the main editor.
fn preview(args: &[String]) -> Result<(), CliError> {
    let path = *positional(args).first().ok_or_else(|| usage("preview: missing file"))?;
    let page: usize = flag(args, "--page").unwrap_or("1").parse::<usize>().unwrap_or(1).max(1);
    let dpi: f32 = flag(args, "--dpi").unwrap_or("150").parse::<f32>().unwrap_or(150.0).clamp(36.0, 300.0);
    // `--width PX` asks for a raster exactly as wide as the preview pane needs (the Explorer handler
    // passes its pane width × zoom), overriding `--dpi`; `--max-px` caps the longer side so a poster-
    // sized page can't allocate hundreds of megabytes in the preview host.
    let width_px: Option<f32> = flag(args, "--width").and_then(|w| w.parse::<f32>().ok()).filter(|w| w.is_finite() && *w >= 1.0);
    let max_px: f32 = flag(args, "--max-px").and_then(|m| m.parse::<f32>().ok()).filter(|m| m.is_finite()).unwrap_or(4096.0).clamp(64.0, 8192.0);
    let out = flag(args, "--out").ok_or_else(|| usage("preview: missing --out <file.png>"))?;
    let password = flag(args, "--password");

    let bytes = match read(path) {
        Ok(b) => b,
        Err(e) => {
            stdout_line(format_args!("STATUS\tIO_ERROR\t0\t{}", e.replace(['\r', '\n', '\t'], " ")))?;
            return Ok(());
        }
    };

    let info = match inspect(bytes.clone(), password) {
        Ok(i) => i,
        Err(pdfcraft_render::OpenError::NeedsPassword | pdfcraft_render::OpenError::WrongPassword) => {
            stdout_line(format_args!(
                "STATUS\tPASSWORD_REQUIRED\t0\tThis PDF document is password-protected. Open it in Linkco PDF Editor to enter the password."
            ))?;
            return Ok(());
        }
        Err(e) => {
            stdout_line(format_args!("STATUS\tINVALID_PDF\t0\t{}", e.to_string().replace(['\r', '\n', '\t'], " ")))?;
            return Ok(());
        }
    };

    let total_pages = info.pages.len();
    if total_pages == 0 {
        stdout_line(format_args!("STATUS\tEMPTY_PDF\t0\tThis PDF document contains no pages."))?;
        return Ok(());
    }

    let clamped_page = page.min(total_pages);
    let scale = info.pages.get(clamped_page - 1).map_or(dpi / 72.0, |pi| preview_scale(pi.width, pi.height, dpi, width_px, max_px));
    let mut r = PageRenderer::new(bytes, RenderConfig { password: password.map(Arc::from), ..Default::default() });
    let p = r.render(RenderRequest { page: clamped_page - 1, kind: RequestKind::Pixels, tile: None, scale, tag: 0 });
    if let Some(e) = p.error {
        stdout_line(format_args!("STATUS\tRENDER_ERROR\t{total_pages}\t{}", e.replace(['\r', '\n', '\t'], " ")))?;
        return Ok(());
    }

    let png_bytes = pdfcraft_engine::export::encode_image(p.width, p.height, &p.rgba, ImageFormat::Png, f64::from(scale) * 72.0)?;
    std::fs::write(out, png_bytes).map_err(|e| format!("{out}: {e}"))?;
    stdout_line(format_args!("STATUS\tOK\t{total_pages}\t{clamped_page}\t{}\t{}", p.width, p.height))?;
    Ok(())
}

/// Render scale (pixels per point) for `preview`: `--width` wins over `--dpi`, and the longer side
/// of the raster never exceeds `max_px`. Degenerate page sizes fall back to the `--dpi` scale.
fn preview_scale(page_w: f32, page_h: f32, dpi: f32, width_px: Option<f32>, max_px: f32) -> f32 {
    let base = dpi / 72.0;
    if !(page_w.is_finite() && page_h.is_finite()) || page_w <= 0.0 || page_h <= 0.0 {
        return base;
    }
    let wanted = width_px.map_or(base, |w| w / page_w);
    let cap = max_px / page_w.max(page_h);
    wanted.min(cap).clamp(0.05, 300.0 / 72.0 * 4.0)
}

/// Child-process body for `check`: prints one JSON line.
fn check_one(args: &[String]) -> Result<(), CliError> {
    validate_options("check-one", args, CHECK_ONE_OPTIONS)?;
    let path = *positional(args).first().ok_or_else(|| usage("check-one: missing file"))?;
    let dpi: f32 = flag(args, "--dpi").unwrap_or("36").parse().map_err(|_| usage("bad --dpi"))?;
    let start = Instant::now();
    let bytes = read(path)?;
    let (status, pages, failed, detail, warnings) = match inspect(bytes.clone(), None) {
        Err(e) => (format!("open-error:{}", short(&e.to_string())), 0, 0, e.to_string(), 0),
        Ok(info) => {
            let mut r = PageRenderer::new(bytes, RenderConfig::default());
            let mut failed = Vec::new();
            for i in 0..info.pages.len().min(500) {
                let p = r.render(RenderRequest { page: i, kind: RequestKind::Pixels, tile: None, scale: dpi / 72.0, tag: 0 });
                if let Some(e) = r.render(RenderRequest { page: i, kind: RequestKind::Text, tile: None, scale: 1.0, tag: 0 }).error {
                    failed.push(format!("p{} text: {e}", i + 1));
                }
                if let Some(e) = p.error {
                    failed.push(format!("p{}: {e}", i + 1));
                }
            }
            let status = if failed.is_empty() { "ok".to_string() } else { "page-errors".to_string() };
            (status, info.pages.len(), failed.len(), failed.join(" | "), info.warnings.len())
        }
    };
    // `--edit` (fuzzing): also run our own object layer end to end, with no panic safety net,
    // so a panic in cos shows up as a crash of this process.
    if args.iter().any(|a| a == "--edit") {
        edit_round_trip(&std::fs::read(path).map_err(|e| e.to_string())?);
    }
    let line = serde_json::json!({ "file": path, "status": status, "pages": pages, "failed_pages": failed, "warnings": warnings,
        "ms": start.elapsed().as_millis() as u64, "detail": detail.chars().take(400).collect::<String>() });
    stdout_line(format_args!("{line}"))?;
    Ok(())
}

/// Open with cos, touch every object, edit the catalog, save incrementally and in full (both
/// output styles), and reopen each result. Errors are fine; panics, hangs and aborts are bugs.
fn edit_round_trip(bytes: &[u8]) {
    use pdfcraft_cos::{Document, Object, SaveOptions, write_full, write_incremental};
    let Ok(mut doc) = Document::open(Arc::new(bytes.to_vec())) else { return };
    for num in doc.object_numbers().into_iter().take(20_000) {
        if let Ok(o) = doc.try_get(num)
            && let Object::Stream(s) = &*o
        {
            let _ = s.decoded();
        }
    }
    if let Some(root) = doc.root() {
        let _ = doc.update_dict(root, |d| d.set(b"PdfCraftFuzz".to_vec(), Object::Bool(true)));
    }
    let classic = SaveOptions { object_streams: false, ..SaveOptions::default() };
    for out in [write_incremental(&doc, &SaveOptions::default()), write_full(&doc, &SaveOptions::default()), write_full(&doc, &classic)]
        .into_iter()
        .flatten()
    {
        if let Ok(again) = Document::open(Arc::new(out)) {
            let _ = again.root().map(|r| again.get(r));
        }
    }
}

fn short(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).take(4).collect::<Vec<_>>().join("-").to_lowercase()
}

fn collect(paths: &[&str]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    while let Some(p) = stack.pop() {
        if p.is_dir() {
            if let Ok(rd) = std::fs::read_dir(&p) {
                stack.extend(rd.flatten().map(|e| e.path()));
            }
        } else if p.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("pdf")) {
            out.push(p);
        }
    }
    out.sort();
    out
}

fn check(args: &[String]) -> Result<(), CliError> {
    validate_options("check", args, CHECK_OPTIONS)?;
    let files = collect(&positional(args));
    if files.is_empty() {
        return Err(usage("check: no PDF files found"));
    }
    let timeout = Duration::from_secs(flag(args, "--timeout").unwrap_or("20").parse().map_err(|_| usage("bad --timeout"))?);
    let dpi = flag(args, "--dpi").unwrap_or("36").to_string();
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut results = Vec::new();
    let mut counts: std::collections::BTreeMap<String, usize> = Default::default();
    for (n, f) in files.iter().enumerate() {
        let r = run_child(&exe, f, &dpi, timeout);
        let status = r["status"].as_str().unwrap_or("?").to_string();
        *counts.entry(status.split(':').next().unwrap_or("?").to_string()).or_default() += 1;
        if status != "ok" {
            eprintln!(
                "[{}/{}] {status:<28} {}  {}",
                n + 1,
                files.len(),
                f.display(),
                r["detail"].as_str().unwrap_or("").chars().take(160).collect::<String>()
            );
        }
        results.push(r);
    }
    eprintln!("\nchecked {} files: {:?}", files.len(), counts);
    if let Some(out) = flag(args, "--json") {
        std::fs::write(out, serde_json::to_string_pretty(&results).unwrap_or_default()).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn run_child(exe: &Path, file: &Path, dpi: &str, timeout: Duration) -> serde_json::Value {
    let file_s = file.to_string_lossy().to_string();
    let child = Command::new(exe).args(["check-one", &file_s, "--dpi", dpi]).stdout(Stdio::piped()).stderr(Stdio::null()).spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => return serde_json::json!({ "file": file_s, "status": "harness-error", "detail": e.to_string() }),
    };
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut out = String::new();
                if let Some(mut so) = child.stdout.take() {
                    use std::io::Read;
                    let _ = so.read_to_string(&mut out);
                }
                return match serde_json::from_str::<serde_json::Value>(out.trim()) {
                    Ok(v) => v,
                    Err(_) => serde_json::json!({ "file": file_s, "status": "crash", "detail": format!("child exited with {status}") }),
                };
            }
            Ok(None) if start.elapsed() > timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return serde_json::json!({ "file": file_s, "status": "timeout", "detail": format!("exceeded {}s", timeout.as_secs()) });
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(e) => return serde_json::json!({ "file": file_s, "status": "harness-error", "detail": e.to_string() }),
        }
    }
}

// ---- automation --------------------------------------------------------------------------------

fn automation(args: &[String]) -> Result<pdfcraft_automation::Automation, CliError> {
    let a = pdfcraft_automation::Automation::new().with_limits(limits(args)?);
    match flag(args, "--root") {
        Some(root) => a.with_root(root).map_err(|e| usage(format!("--root {root}: {e}"))),
        None => Ok(a),
    }
}

/// A tool failure in the script's terms: a tool that does not exist is not something this build
/// can do, arguments that don't fit its schema are a bad call, and anything else is the tool
/// itself failing.
fn tool_failure(error: pdfcraft_automation::ToolError) -> CliError {
    use pdfcraft_automation::ToolError;
    match error {
        ToolError::UnknownTool(name) => unsupported(format!("unknown tool {name:?} (see `pdfcraft-cli tools`)")),
        ToolError::InvalidArgs(message) => usage(message),
        ToolError::Failed(message) => CliError::Message(exit::Failure::new(exit::Kind::Failed, message)),
        ToolError::Refused(message) => refused(message),
    }
}

fn tools() -> Result<(), CliError> {
    let list: Vec<serde_json::Value> = pdfcraft_automation::tools().iter().map(pdfcraft_automation::schema::tool_json).collect();
    stdout_line(format_args!("{}", serde_json::to_string_pretty(&list).unwrap_or_default()))?;
    Ok(())
}

/// The options `schema` understands.
const SCHEMA_OPTIONS: &[OptionSpec] = &[OptionSpec { name: "--schema-version", takes_value: true }];

/// `schema`: the whole tool table, as one document with the version that names it (REC-B02).
///
/// `--schema-version V` asks a caller's question the other way round: does this build speak the
/// revision I was written against? It answers `compatible` — whether V has the same major number,
/// which is what the version promises (see `crates/automation/src/schema.rs`) — and exits 3
/// (`unsupported`) when it does not, so a script can refuse to start rather than discover the
/// mismatch halfway through a job.
fn schema(args: &[String]) -> Result<(), CliError> {
    validate_options("schema", args, SCHEMA_OPTIONS)?;
    let mine = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    let mut document = pdfcraft_automation::schema::schema_document();
    if let Some(asked) = flag(args, "--schema-version") {
        if !positional(args).is_empty() {
            return Err(usage("schema: --schema-version takes no other arguments"));
        }
        let Some(wanted) = pdfcraft_automation::schema::parse_version(asked) else {
            return Err(usage(format!("schema: {asked:?} is not a version like {mine}; versions are major.minor")));
        };
        let (major, _) = pdfcraft_automation::schema::parse_version(mine).unwrap_or((0, 0));
        // The same major: additive changes only, so the caller's expectations still hold.
        let compatible = wanted.0 == major;
        document["asked"] = serde_json::json!({ "schema_version": asked, "compatible": compatible });
        stdout_line(format_args!("{}", serde_json::to_string_pretty(&document).unwrap_or_default()))?;
        if !compatible {
            return Err(unsupported(format!(
                "schema: this build speaks schema {mine}, which is not compatible with {asked} (a different major version)"
            )));
        }
        return Ok(());
    }
    if let Some(extra) = positional(args).first() {
        return Err(usage(format!("schema: unexpected argument {extra:?}")));
    }
    stdout_line(format_args!("{}", serde_json::to_string_pretty(&document).unwrap_or_default()))?;
    Ok(())
}

/// Print a tool's result: JSON as JSON; images go to `--out` (or are summarised). The image is
/// written like a tool's own output, so `--root` confines it too.
fn print_output(auto: &pdfcraft_automation::Automation, content: Vec<pdfcraft_automation::Content>, out: Option<&str>) -> Result<(), CliError> {
    for c in content {
        match c {
            pdfcraft_automation::Content::Json(v) => stdout_line(format_args!("{}", serde_json::to_string_pretty(&v).unwrap_or_default()))?,
            pdfcraft_automation::Content::Png { data, width, height } => match out {
                Some(path) => {
                    let written = auto.write_output(path, &data).map_err(|e| e.to_string())?;
                    stdout_line(format_args!("{}", serde_json::json!({ "image": written.to_string_lossy(), "width": width, "height": height })))?;
                }
                None => stdout_line(format_args!(
                    "{}",
                    serde_json::json!({ "image": "png", "width": width, "height": height, "bytes": data.len(), "hint": "pass --out file.png to save it" })
                ))?,
            },
        }
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), CliError> {
    validate_options("run", args, RUN_OPTIONS)?;
    let mut auto = automation(args)?;
    if let Some(script) = flag(args, "--script") {
        let text = std::fs::read_to_string(script).map_err(|e| format!("{script}: {e}"))?;
        let steps: Vec<serde_json::Value> = serde_json::from_str(&text).map_err(|e| usage(format!("{script}: {e}")))?;
        for (i, step) in steps.iter().enumerate() {
            // Steps before this one already ran, so their work stands: the failure is partial
            // (`exit::Kind::Partial`), which tells a script to look at what was done rather than
            // to assume nothing happened.
            run_step(&mut auto, step, i).map_err(|e| if i > 0 { e.into_partial() } else { e })?;
        }
        return Ok(());
    }
    let tool = *positional(args).first().ok_or_else(|| usage("run: missing tool name (see `pdfcraft-cli tools`)"))?;
    let mut obj = serde_json::Map::new();
    for kv in positional(args).iter().skip(1) {
        let (k, v) = kv.split_once('=').ok_or_else(|| usage(format!("run: expected key=value, got {kv:?}")))?;
        let value = serde_json::from_str(v).unwrap_or_else(|_| serde_json::Value::String(v.to_string()));
        obj.insert(k.to_string(), value);
    }
    let content = auto.call(tool, &serde_json::Value::Object(obj)).map_err(tool_failure)?;
    print_output(&auto, content, flag(args, "--out"))
}

/// One step of a `--script`, with the step number in every message.
fn run_step(auto: &mut pdfcraft_automation::Automation, step: &serde_json::Value, i: usize) -> Result<(), CliError> {
    let tool = step["tool"].as_str().ok_or_else(|| usage(format!("step {}: missing \"tool\"", i + 1)))?;
    let content = auto.call(tool, &step["args"]).map_err(|e| tool_failure(e).context(&format!("step {} ({tool})", i + 1)))?;
    print_output(auto, content, step["out"].as_str()).map_err(|e| e.context(&format!("step {} ({tool})", i + 1)))
}

#[cfg(feature = "mcp")]
fn mcp(args: &[String]) -> Result<(), CliError> {
    validate_options("mcp", args, MCP_OPTIONS)?;
    let compact = args.iter().any(|a| a == "--compact");
    let profile = match flag(args, "--profile") {
        Some(name) => pdfcraft_automation::Profile::parse(name)
            .ok_or_else(|| usage(format!("mcp: --profile {name} is not one of {}", pdfcraft_automation::Profile::NAMES.join(", "))))?,
        None => pdfcraft_automation::Profile::default(),
    };
    let mut policy = pdfcraft_automation::Policy::new(profile);
    for name in values(args, "--approve") {
        policy.approve(name).map_err(|e| usage(format!("mcp: --approve {e}")))?;
    }
    let session = automation(args)?.with_policy(policy).map_err(|e| usage(format!("mcp: {e}")))?;
    let mut server = pdfcraft_automation::mcp::McpServer::new(session).with_compact(compact);
    let _ = writeln!(
        std::io::stderr().lock(),
        "pdfcraft-cli: MCP server on stdio (protocol {}, {} profile{}); close stdin to stop",
        pdfcraft_automation::mcp::PROTOCOL_VERSIONS[0],
        profile.name(),
        if compact { ", compact tool list" } else { "" }
    );
    server
        .serve(std::io::stdin().lock(), std::io::stdout().lock())
        .map_err(|e| CliError::Message(exit::Failure::new(exit::Kind::Failed, e.to_string())))
}

// ---- UI control channel client -----------------------------------------------------------------

/// One request to a running app's control channel (`pdfcraft --control FILE`).
fn ui(args: &[String]) -> Result<(), CliError> {
    validate_options("ui", args, UI_OPTIONS)?;
    use std::io::{BufRead, BufReader, Write as _};
    let file = flag(args, "--control").ok_or_else(|| usage("ui: missing --control FILE (start the app with `pdfcraft --control FILE`)"))?;
    let info = read_control_file(file)?;
    let port = info["port"].as_u64().ok_or_else(|| malformed(format!("{file}: no port")))?;
    // Refused rather than truncated, which would quietly reach some other port.
    let port = u16::try_from(port).map_err(|_| malformed(format!("{file}: {port} is not a port number")))?;
    let token = info["token"].as_str().ok_or_else(|| malformed(format!("{file}: no token")))?;
    let pos = positional(args);
    let method =
        *pos.first().ok_or_else(|| usage("ui: missing method (state, inspect, click, drag, type, key, command, commands, set, open, screenshot)"))?;
    let method = if method.starts_with("ui.") { method.to_string() } else { format!("ui.{method}") };
    let mut params = serde_json::Map::new();
    for kv in pos.iter().skip(1) {
        let (k, v) = kv.split_once('=').ok_or_else(|| usage(format!("ui: expected key=value, got {kv:?}")))?;
        params.insert(k.to_string(), serde_json::from_str(v).unwrap_or_else(|_| serde_json::Value::String(v.to_string())));
    }
    let stream =
        std::net::TcpStream::connect(("127.0.0.1", port)).map_err(|e| format!("can't reach the app on port {port}: {e} (is it still running?)"))?;
    stream.set_read_timeout(Some(Duration::from_secs(40))).map_err(|e| e.to_string())?;
    let mut write = stream.try_clone().map_err(|e| e.to_string())?;
    let mut lines = BufReader::new(stream).lines();
    let mut rpc = |id: u64, method: &str, params: serde_json::Value| -> Result<serde_json::Value, String> {
        writeln!(write, "{}", serde_json::json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params })).map_err(|e| e.to_string())?;
        let line = lines.next().ok_or("the app closed the connection")?.map_err(|e| e.to_string())?;
        let reply: serde_json::Value = serde_json::from_str(&line).map_err(|e| e.to_string())?;
        match reply.get("error") {
            Some(e) => Err(e["message"].as_str().unwrap_or("error").to_string()),
            None => Ok(reply["result"].clone()),
        }
    };
    rpc(1, "auth", serde_json::json!({ "token": token }))?;
    let mut result = rpc(2, &method, serde_json::Value::Object(params))?;
    if let (Some(out), Some(data)) = (flag(args, "--out"), result.get("png_base64").and_then(|d| d.as_str())) {
        use base64::Engine as _;
        let png = base64::engine::general_purpose::STANDARD.decode(data).map_err(|e| e.to_string())?;
        std::fs::write(out, png).map_err(|e| format!("{out}: {e}"))?;
        result["png_base64"] = serde_json::json!(format!("written to {out}"));
    }
    stdout_line(format_args!("{}", serde_json::to_string_pretty(&result).unwrap_or_default()))?;
    Ok(())
}

/// Most a control file may hold. The app writes under 100 bytes.
const CONTROL_FILE_MAX: u64 = 64 * 1024;

const CONTROL_FILE_ADVICE: &str =
    "Start the app with a control file in a folder only you can write, such as `pdfcraft --control ~/.pdfcraft-control.json`";

/// Read the file written by `pdfcraft --control FILE`.
///
/// It names the port that receives the token and every command, typed text included, so it must
/// be the user's own. In a shared folder such as `/tmp`, another local user can create the file
/// first; the app then can't write it, and the CLI would talk to the other user's listener.
/// Refused: a symbolic link, anything but a regular file, and on Unix a file owned by someone
/// else or open to other users (the app writes it with mode 600).
fn read_control_file(file: &str) -> Result<serde_json::Value, CliError> {
    use std::io::Read as _;
    let at_path = std::fs::symlink_metadata(file).map_err(|e| format!("{file}: {e}"))?;
    if at_path.file_type().is_symlink() {
        return Err(refused(format!("{file}: is a symbolic link, which could point at another user's file. {CONTROL_FILE_ADVICE}")));
    }
    if !at_path.is_file() {
        return Err(refused(format!("{file}: not a regular file. {CONTROL_FILE_ADVICE}")));
    }
    let f = std::fs::File::open(file).map_err(|e| format!("{file}: {e}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        // Checked on the open file, so nothing swapped in after `symlink_metadata` gets through.
        let opened = f.metadata().map_err(|e| format!("{file}: {e}"))?;
        if (opened.dev(), opened.ino()) != (at_path.dev(), at_path.ino()) {
            return Err(refused(format!("{file}: was replaced while being opened. {CONTROL_FILE_ADVICE}")));
        }
        if let Some(problem) = control_file_problem(opened.uid(), opened.mode(), current_uid()) {
            return Err(refused(format!("{file}: {problem}. {CONTROL_FILE_ADVICE}")));
        }
    }
    let mut text = String::new();
    f.take(CONTROL_FILE_MAX.saturating_add(1)).read_to_string(&mut text).map_err(|e| format!("{file}: {e}"))?;
    if u64::try_from(text.len()).unwrap_or(u64::MAX) > CONTROL_FILE_MAX {
        return Err(refused(format!("{file}: too large for a control file (over {} KiB)", CONTROL_FILE_MAX / 1024)));
    }
    serde_json::from_str(&text).map_err(|e| malformed(format!("{file}: {e}")))
}

/// Why user `me` can't trust a control file with this owner and mode, if they can't.
#[cfg(any(test, unix))]
fn control_file_problem(owner: u32, mode: u32, me: u32) -> Option<String> {
    if owner != me {
        return Some(format!("owned by another user (uid {owner}; you are uid {me}), who may be the one listening for your commands"));
    }
    let permissions = mode & 0o7777;
    (permissions & 0o077 != 0).then(|| format!("other users have access to it (mode {permissions:o}; the app writes it with mode 600)"))
}

/// The effective user id: the owner of the files this process creates, and so of a control file
/// the app wrote for this user. std has no `geteuid` and this crate forbids `unsafe`; rustix's is
/// safe and can't fail.
#[cfg(unix)]
fn current_uid() -> u32 {
    rustix::process::geteuid().as_raw()
}

#[cfg(test)]
mod tests {
    use super::{control_file_problem, preview_scale};

    #[test]
    fn preview_scale_honours_width_dpi_and_the_pixel_cap() {
        // Letter page, 612×792 pt.
        assert!((preview_scale(612.0, 792.0, 144.0, None, 4096.0) - 2.0).abs() < 1e-4);
        // `--width` overrides `--dpi`: 1224 px wide ⇒ 2 px per point.
        assert!((preview_scale(612.0, 792.0, 72.0, Some(1224.0), 4096.0) - 2.0).abs() < 1e-4);
        // The longer side is capped: 792 pt × scale ≤ 1000 px.
        let capped = preview_scale(612.0, 792.0, 300.0, Some(10_000.0), 1000.0);
        assert!(792.0 * capped <= 1000.0 + 1e-3, "{capped}");
        // A poster-sized page (A0) at 150 dpi stays within the cap.
        let a0 = preview_scale(2384.0, 3370.0, 150.0, None, 4096.0);
        assert!(3370.0 * a0 <= 4096.0 + 1e-3, "{a0}");
        // Degenerate sizes fall back to the dpi scale.
        assert!((preview_scale(0.0, 792.0, 72.0, Some(500.0), 4096.0) - 1.0).abs() < 1e-4);
        assert!((preview_scale(f32::NAN, 792.0, 72.0, None, 4096.0) - 1.0).abs() < 1e-4);
    }

    #[test]
    fn a_control_file_must_be_yours_and_private() {
        // Regular file type bits (0o100000) don't matter, only the owner and the permission bits.
        assert_eq!(control_file_problem(1000, 0o100600, 1000), None);
        assert_eq!(control_file_problem(1000, 0o600, 1000), None);
        assert_eq!(control_file_problem(1000, 0o400, 1000), None);
        let other = control_file_problem(1001, 0o600, 1000).unwrap_or_default();
        assert!(other.contains("owned by another user") && other.contains("1001"), "{other}");
        // Root gets no exception: a file a user planted is still theirs.
        assert!(control_file_problem(1000, 0o600, 0).is_some_and(|p| p.contains("owned by another user")));
        for mode in [0o644, 0o640, 0o604, 0o660, 0o606, 0o620, 0o602, 0o610, 0o601, 0o100666] {
            let open = control_file_problem(1000, mode, 1000).unwrap_or_default();
            assert!(open.contains("other users"), "{mode:o}: {open}");
        }
        assert!(control_file_problem(1000, 0o100644, 1000).unwrap_or_default().contains("mode 644"));
    }
}
