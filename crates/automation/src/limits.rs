//! Bounded work: how much a session will read, write and do at once (REC-B07).
//!
//! Workers, quotas and a time limit are three different things that all answer the same question —
//! *how much of this machine may one session take* — and they are kept together here so that
//! `limits_status` can report one consistent picture and `pdfcraft-cli` can set them all with
//! flags that read alike.
//!
//! The defaults are deliberate: quotas are **off** (0 means no limit) and the time limit is none,
//! while the worker count has a real bound. A quota that silently refuses a legitimate 900 MiB save
//! is worse than no quota, and the point of REC-B07 is that the person running the session can
//! choose a bound for the machine in front of them — not that this build guesses one. The counts
//! below are how they check what the bound did.
//!
//! Nothing here is a sandbox: a limit stops a call from *starting* work that is too large, and the
//! audit records what was refused. A tool that is allowed still runs with the process's own memory
//! and file access.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::{Args, Automation, Result, ToolError, write_atomic};

/// One mebibyte, the unit the quotas are set in.
pub const MIB: u64 = 1 << 20;

/// What a session may do at once, and how big a thing it may move.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Worker threads for work this crate spreads out (text extraction today). At least 1.
    pub workers: usize,
    /// Largest file a tool will read, in bytes. 0 means no limit.
    pub max_input_bytes: u64,
    /// Largest file a tool will write, in bytes. 0 means no limit.
    pub max_output_bytes: u64,
    /// Most items one batch tool will take. 0 means no limit.
    pub max_batch_items: usize,
    /// How long a batch may spend before it stops starting new items. Zero means no limit.
    pub item_timeout: Duration,
}

impl Default for Limits {
    fn default() -> Self {
        Self { workers: default_workers(), max_input_bytes: 0, max_output_bytes: 0, max_batch_items: 0, item_timeout: Duration::ZERO }
    }
}

/// The worker count a session uses when nobody said: what the machine offers, capped so that a
/// large server does not have every core parsing the same file.
pub fn default_workers() -> usize {
    if cfg!(target_arch = "wasm32") {
        return 1;
    }
    std::thread::available_parallelism().map_or(1, |n| n.get()).clamp(1, 8)
}

/// What the limits have refused or let through since the session started. Per session, so two
/// sessions in one process (tests, the CLI's script runner) do not read each other's numbers.
#[derive(Debug, Default)]
pub struct Audit {
    checks: AtomicU64,
    inputs_refused: AtomicU64,
    outputs_refused: AtomicU64,
    batches_refused: AtomicU64,
    timed_out: AtomicU64,
    bytes_written: AtomicU64,
}

impl Audit {
    fn count(counter: &AtomicU64) {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// A session's limits and the record of what they did.
#[derive(Debug)]
pub struct Budget {
    limits: Limits,
    audit: Audit,
}

impl Default for Budget {
    fn default() -> Self {
        Self::new(Limits::default())
    }
}

impl Budget {
    pub fn new(limits: Limits) -> Self {
        // A zero worker count is not a limit, it is a mistake: clamp it to the one thread the work
        // can always use.
        Self { limits: Limits { workers: limits.workers.max(1), ..limits }, audit: Audit::default() }
    }

    pub fn limits(&self) -> &Limits {
        &self.limits
    }

    /// Refuse an input file over the limit, before it is read.
    pub(crate) fn check_input(&self, bytes: u64) -> Result<()> {
        Audit::count(&self.audit.checks);
        let max = self.limits.max_input_bytes;
        if max == 0 || bytes <= max {
            return Ok(());
        }
        Audit::count(&self.audit.inputs_refused);
        Err(ToolError::Refused(format!(
            "refused: this file is {}, over this session's input limit of {} (--max-input-mib); nothing was read and nothing was changed",
            human(bytes),
            human(max)
        )))
    }

    /// Refuse an output file over the limit, before it is written.
    pub(crate) fn check_output(&self, bytes: usize) -> Result<()> {
        Audit::count(&self.audit.checks);
        let max = self.limits.max_output_bytes;
        if max == 0 || bytes as u64 <= max {
            return Ok(());
        }
        Audit::count(&self.audit.outputs_refused);
        Err(ToolError::Refused(format!(
            "refused: the result would be {}, over this session's output limit of {} (--max-output-mib); nothing was written",
            human(bytes as u64),
            human(max)
        )))
    }

    /// Refuse a batch with more items than the session allows, before any of them is read.
    pub(crate) fn check_batch(&self, items: usize) -> Result<()> {
        Audit::count(&self.audit.checks);
        let max = self.limits.max_batch_items;
        if max == 0 || items <= max {
            return Ok(());
        }
        Audit::count(&self.audit.batches_refused);
        Err(ToolError::Refused(format!(
            "refused: {items} items is over this session's batch limit of {max} (--max-batch); nothing was read and nothing was written"
        )))
    }

    /// Write a tool's output, counting it and enforcing the output limit first.
    pub(crate) fn write(&self, path: &Path, bytes: &[u8]) -> Result<()> {
        self.check_output(bytes.len())?;
        write_atomic(path, bytes)?;
        self.audit.bytes_written.fetch_add(bytes.len() as u64, Ordering::Relaxed);
        Ok(())
    }

    /// Has `started` been running longer than the time limit? A batch asks this between items, so
    /// an item already in hand is finished (as a stop is, in the job centre) rather than abandoned.
    pub(crate) fn timed_out(&self, started: Instant) -> bool {
        let limit = self.limits.item_timeout;
        if limit.is_zero() || started.elapsed() <= limit {
            return false;
        }
        Audit::count(&self.audit.timed_out);
        true
    }

    /// `limits_status`: what is set, and what it has done.
    pub(crate) fn json(&self) -> Value {
        let l = &self.limits;
        json!({
            "limits": {
                "workers": l.workers,
                "max_input_bytes": l.max_input_bytes,
                "max_output_bytes": l.max_output_bytes,
                "max_batch_items": l.max_batch_items,
                "item_timeout_ms": l.item_timeout.as_millis(),
            },
            "audit": {
                "checks": self.audit.checks.load(Ordering::Relaxed),
                "inputs_refused": self.audit.inputs_refused.load(Ordering::Relaxed),
                "outputs_refused": self.audit.outputs_refused.load(Ordering::Relaxed),
                "batches_refused": self.audit.batches_refused.load(Ordering::Relaxed),
                "batches_stopped_by_time": self.audit.timed_out.load(Ordering::Relaxed),
                "bytes_written": self.audit.bytes_written.load(Ordering::Relaxed),
            },
            "note": "0 means no limit. A refused call changes nothing; a batch stopped by the time limit keeps the items it finished.",
        })
    }
}

/// Bytes in the units a person reads them in.
fn human(bytes: u64) -> String {
    if bytes >= MIB && bytes.is_multiple_of(MIB) {
        format!("{} MiB", bytes / MIB)
    } else if bytes >= 1024 && bytes.is_multiple_of(1024) {
        format!("{} KiB", bytes / 1024)
    } else {
        format!("{bytes} bytes")
    }
}

impl Automation {
    /// Set the session's limits (replacing the defaults).
    pub fn with_limits(mut self, limits: Limits) -> Self {
        self.resources = std::sync::Arc::new(Budget::new(limits));
        self
    }

    /// What this session's limits are and what they have done.
    pub fn resources(&self) -> &Budget {
        &self.resources
    }

    /// Read one batch input: resolve it, answer the input quota from its size, then read it. A
    /// per-file failure comes back as text, which is how a batch reports the files it gave up on.
    pub(crate) fn read_input(&self, path: &str) -> std::result::Result<Vec<u8>, String> {
        let src = self.resolve(path, false).map_err(|e| e.to_string())?;
        let size = std::fs::metadata(&src).map_err(|e| format!("{}: {e}", src.display()))?.len();
        self.resources.check_input(size).map_err(|e| e.to_string())?;
        std::fs::read(&src).map_err(|e| format!("{}: {e}", src.display()))
    }

    /// `limits_status`: the limits and the audit, for an agent or a script.
    pub(crate) fn limits_status(&self, _a: &Args) -> Result<Value> {
        Ok(self.resources.json())
    }
}
