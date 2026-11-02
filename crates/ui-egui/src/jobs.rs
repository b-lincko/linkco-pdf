//! The background jobs the shell is running: what each one is, where it has got to, and how to
//! stop it.
//!
//! Six kinds of work leave the interface thread: optimizing a document, recognising text in one
//! document, recognising text in a folder of files, running an action over files, exporting, and
//! printing.
//! Each used to report itself in its own way — a private progress card for the optimizer, a toast
//! rewritten every few hundred milliseconds for the rest — and only two of them could be stopped
//! at all: the optimizer's Cancel button, and one method (`cancel_ocr`) that no control reached.
//!
//! This module holds what they share. A [`Cancel`] is a flag a worker checks between items, and
//! [`run_batch`] is the loop that checks it, so "stop" means the same thing everywhere: the item
//! in hand is finished (its file is written whole or not at all, by the atomic writer) and no
//! further item starts. [`Jobs`] is the list the card draws and the control channel reports, so a
//! person and an agent see the same jobs and can stop the same job.
//!
//! Bounded on purpose: a stop is cooperative and is answered between items, so a single item that
//! takes a long time (one huge page at 600 dpi) still has to finish. A job can also be handed to
//! something this app cannot call back — a print job the spooler has taken — and [`Jobs::hand_over`]
//! is that moment: the card withdraws its Cancel and the job centre refuses the stop, rather than
//! offering one that would be ignored. Nothing here kills a thread,
//! and a job whose worker has died is removed by the poll that was waiting for it, not by a
//! watchdog: the worker holds an [`Alive`] handle and drops it when it ends, so the poll can tell
//! "not finished yet" from "never going to say anything" (REC-R06).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Names a running job in the centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JobId(pub u64);

/// A cooperative stop request, shared with the thread that carries a job out.
///
/// Cloning shares one flag: the card, the control channel, a keyboard shortcut and the worker all
/// hold the same value, and asking to stop is idempotent and thread-safe.
#[derive(Clone, Debug, Default)]
pub struct Cancel(Arc<AtomicBool>);

impl Cancel {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ask the worker to stop at its next boundary. Any thread, any number of times.
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }

    /// Whether a stop has been asked for. Workers read this between items, never inside one.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Whether a job's worker is still running (REC-R06).
///
/// A worker reports by writing its outcome where the poll looks for it, and a poll cannot tell
/// "not there yet" from "never coming". A worker that panicked on one awkward page, or that
/// returned early, wrote nothing — so the card spun for ever, and the slot that keeps a second run
/// of the same kind from starting stayed closed for the life of the session, on a document that
/// was otherwise fine. This is the other half of that answer: the worker holds a [`Worker`] for as
/// long as it runs and drops it when it ends — a return and an unwind both drop it — so the poll
/// can tell the two apart and end the job itself when nothing is coming.
///
/// It is published as a handle rather than a deadline on purpose: a worker that is slow is not a
/// worker that is gone, and a job given up on after N seconds would end real work on a large
/// document. It says what is true — that the worker that was going to report has ended.
#[derive(Clone, Debug)]
pub struct Alive(Arc<AtomicBool>);

impl Alive {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }

    /// Whether the worker is still running: false once the [`Worker`] it holds was dropped.
    pub fn is_running(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    /// Whether the worker that holds a copy has ended without saying anything: it is gone, and
    /// `reported` — what the poll found in the place the worker writes its outcome — is false.
    /// Both halves are needed: a worker that ended properly leaves its report behind before its
    /// handle goes, and a worker that is still going has not ended.
    pub fn ended_without_report(&self, reported: bool) -> bool {
        !self.is_running() && !reported
    }

    /// The worker's end of this. One per worker, held for the length of the work, dropped at the
    /// end of it — and dropped by an unwind as well, which is the whole point.
    pub fn worker(&self) -> Worker {
        Worker(self.0.clone())
    }
}

impl Default for Alive {
    fn default() -> Self {
        Self::new()
    }
}

/// A worker's end of [`Alive`]: dropping it says the worker has ended, however it ended.
#[derive(Debug)]
pub struct Worker(Arc<AtomicBool>);

impl Drop for Worker {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

/// What kind of work a job is: the card groups by it and tests read it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Optimize,
    /// Recognizing text in the open document.
    Recognize,
    /// Recognizing text in a folder of files.
    RecognizeFiles,
    /// An action run over files.
    Action,
    Export,
    /// A job sent to a printer.
    Print,
}

impl Kind {
    /// A stable name for the control channel, so an agent does not depend on the variant order.
    pub fn name(self) -> &'static str {
        match self {
            Kind::Optimize => "optimize",
            Kind::Recognize => "recognize",
            Kind::RecognizeFiles => "recognize-files",
            Kind::Action => "action",
            Kind::Export => "export",
            Kind::Print => "print",
        }
    }
}

/// One running job: what it is, where it has got to, and how to stop it.
#[derive(Clone, Debug)]
pub struct Job {
    pub id: JobId,
    pub kind: Kind,
    /// The job's name in the reader's language ("Reduce File Size", "Recognize text", the
    /// action's own name).
    pub label: String,
    /// What it is doing now, in the reader's language ("page 3 of 40", "image 4 of 10").
    pub stage: String,
    /// From 0 to 1, as far as the job can say.
    pub fraction: f32,
    pub done: usize,
    pub total: usize,
    /// A stop was asked for and the worker has not reported back yet.
    pub stopping: bool,
    /// Whether a stop can still be answered. False once the job has been handed to something
    /// outside this app that cannot call it back (see [`Jobs::hand_over`]).
    pub stoppable: bool,
    cancel: Cancel,
}

impl Job {
    /// The stop request for this job, for a worker that carries it out.
    pub fn cancel_token(&self) -> Cancel {
        self.cancel.clone()
    }
}

/// The running jobs, oldest first.
#[derive(Debug, Default)]
pub struct Jobs {
    next: u64,
    running: Vec<Job>,
}

impl Jobs {
    /// Register a job that is about to start, and hand back its handle and its stop request.
    ///
    /// Registering before the worker starts is what makes a job visible in the frame that begins
    /// it: a job that finishes immediately (the inline path in tests and automation) is registered
    /// and removed again in the same call, never leaving a card behind.
    pub fn begin(&mut self, kind: Kind, label: String, total: usize) -> (JobId, Cancel) {
        self.next = self.next.saturating_add(1);
        let id = JobId(self.next);
        let cancel = Cancel::new();
        self.running.push(Job {
            id,
            kind,
            label,
            stage: String::new(),
            fraction: 0.0,
            done: 0,
            total,
            stopping: false,
            stoppable: true,
            cancel: cancel.clone(),
        });
        (id, cancel)
    }

    /// The running jobs, oldest first.
    pub fn jobs(&self) -> &[Job] {
        &self.running
    }

    /// Whether anything is running.
    pub fn is_empty(&self) -> bool {
        self.running.is_empty()
    }

    /// One job, if it is still running.
    pub fn get(&self, id: JobId) -> Option<&Job> {
        self.running.iter().find(|j| j.id == id)
    }

    /// Say what the job is doing now, with its own idea of how far along it is.
    pub fn stage(&mut self, id: JobId, stage: impl Into<String>, fraction: f32) {
        if let Some(job) = self.running.iter_mut().find(|j| j.id == id) {
            job.stage = stage.into();
            job.fraction = if fraction.is_finite() { fraction.clamp(0.0, 1.0) } else { 0.0 };
        }
    }

    /// Say how many items of `total` are done; the stage sentence is the caller's.
    pub fn count(&mut self, id: JobId, done: usize, total: usize) {
        let fraction = if total == 0 { 0.0 } else { (done as f32 / total as f32).clamp(0.0, 1.0) };
        if let Some(job) = self.running.iter_mut().find(|j| j.id == id) {
            job.done = done;
            job.total = total;
            job.fraction = fraction;
        }
    }

    /// Remove a finished job (`false` when it was not running). Its card goes with it.
    pub fn finish(&mut self, id: JobId) -> bool {
        match self.running.iter().position(|j| j.id == id) {
            Some(at) => {
                self.running.remove(at);
                true
            }
            None => false,
        }
    }

    /// Ask the job's worker to stop. `false` when no such job is running (already finished), and
    /// when the job has been handed over and no stop can be answered any more (see [`Self::hand_over`]).
    pub fn cancel(&mut self, id: JobId) -> bool {
        match self.running.iter_mut().find(|j| j.id == id && j.stoppable) {
            Some(job) => {
                job.stopping = true;
                job.cancel.cancel();
                true
            }
            None => false,
        }
    }

    /// Ask every job that can still be stopped to stop; returns how many were asked.
    pub fn cancel_all(&mut self) -> usize {
        let mut asked = 0;
        for job in &mut self.running {
            if job.stoppable {
                job.stopping = true;
                job.cancel.cancel();
                asked += 1;
            }
        }
        asked
    }

    /// The job has been handed to something outside this app that cannot call it back — for a
    /// print, the system spooler has taken it. From now on the card offers no Cancel for it and
    /// [`Self::cancel`] answers `false`, because a stop that cannot happen is not a stop: a
    /// button that did nothing would be worse than no button. A stop that was asked for a moment
    /// too late is cleared with it, so the card does not say Cancelling… about a job that is
    /// going ahead. `false` when no such job is running.
    pub fn hand_over(&mut self, id: JobId) -> bool {
        match self.running.iter_mut().find(|j| j.id == id) {
            Some(job) => {
                job.stoppable = false;
                job.stopping = false;
                true
            }
            None => false,
        }
    }
}

/// What a batch did before it stopped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BatchOutcome<E> {
    /// Items that finished without an error.
    pub done: usize,
    /// What each failed item said, in the order they failed.
    pub failed: Vec<E>,
    /// A stop was asked for and at least one item was not started because of it.
    pub cancelled: bool,
}

impl<E> BatchOutcome<E> {
    /// How many items were offered to the loop, started or not.
    pub fn seen(&self) -> usize {
        self.done + self.failed.len()
    }
}

/// Run `each` over `items`, stopping before the next item once `cancel` is asked for.
///
/// Every batch uses this loop, so a stop means the same thing everywhere: the item in hand
/// finishes — its file is written whole or not at all, by the atomic writer — and no further item
/// starts. `report` is called before each item with how many have finished, for the card.
///
/// A stop that arrives after the last item finished is not a cancellation: nothing was given up,
/// so the outcome says the batch completed.
pub fn run_batch<T, E>(
    cancel: &Cancel,
    items: impl IntoIterator<Item = T>,
    mut report: impl FnMut(usize),
    mut each: impl FnMut(T) -> Result<(), E>,
) -> BatchOutcome<E> {
    let mut outcome = BatchOutcome { done: 0, failed: Vec::new(), cancelled: false };
    for item in items {
        if cancel.is_cancelled() {
            outcome.cancelled = true;
            break;
        }
        report(outcome.done);
        match each(item) {
            Ok(()) => outcome.done += 1,
            Err(e) => outcome.failed.push(e),
        }
    }
    outcome
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stop_request_is_shared_idempotent_and_never_goes_back() {
        let cancel = Cancel::new();
        let held = cancel.clone();
        assert!(!cancel.is_cancelled());
        held.cancel();
        assert!(cancel.is_cancelled(), "the clone and the original are one flag");
        cancel.cancel();
        assert!(cancel.is_cancelled(), "asking twice is harmless");
        assert!(!Cancel::new().is_cancelled(), "a fresh request is not set");
    }

    #[test]
    fn the_centre_lists_what_is_running_and_forgets_what_finished() {
        let mut jobs = Jobs::default();
        assert!(jobs.is_empty());
        let (a, token) = jobs.begin(Kind::Recognize, "Recognize text".into(), 40);
        let (b, _) = jobs.begin(Kind::Export, "Export".into(), 3);
        assert_ne!(a, b, "each job gets its own id");
        assert_eq!(jobs.jobs().len(), 2);
        jobs.count(a, 10, 40);
        jobs.stage(a, "page 11 of 40", 0.25);
        let job = jobs.get(a).expect("still running");
        assert_eq!((job.done, job.total), (10, 40));
        assert_eq!(job.stage, "page 11 of 40");
        assert!((job.fraction - 0.25).abs() < 1e-6, "{job:?}");
        // A fraction that is not a number never reaches the bar.
        jobs.stage(b, "counting", f32::NAN);
        assert_eq!(jobs.get(b).map(|j| j.fraction), Some(0.0));
        // Cancelling one leaves the other alone.
        assert!(jobs.cancel(a));
        assert!(jobs.get(a).is_some_and(|j| j.stopping));
        assert!(token.is_cancelled(), "the worker's own handle is the one that was set");
        assert!(jobs.get(b).is_some_and(|j| !j.stopping));
        // Finishing removes it; cancelling it afterwards is refused rather than silently ignored.
        assert!(jobs.finish(a), "the optimization was running");
        assert!(!jobs.cancel(a), "a finished job cannot be cancelled");
        assert_eq!(jobs.jobs().len(), 1);
        assert_eq!(jobs.cancel_all(), 1);
        assert!(jobs.get(b).is_some_and(|j| j.stopping));
        assert!(jobs.finish(b), "the export was running");
        assert!(jobs.is_empty());
    }

    /// A print job the spooler has taken cannot be called back: the centre stops offering the stop
    /// instead of recording one that would be ignored.
    #[test]
    fn a_handed_over_job_refuses_the_stop_and_keeps_no_stopping_state() {
        let mut jobs = Jobs::default();
        let (print, token) = jobs.begin(Kind::Print, "Print to Front Desk".into(), 0);
        let (other, _) = jobs.begin(Kind::Export, "Export".into(), 3);
        assert!(jobs.get(print).is_some_and(|j| j.stoppable), "a fresh job can be stopped");
        // While the sheets are being rendered the stop is real.
        assert!(jobs.cancel(print));
        assert!(token.is_cancelled(), "the worker holds the same flag");
        // The spooler takes the job: the stop is withdrawn, and the state that was set before it
        // is cleared so the card does not say Cancelling… for ever.
        assert!(jobs.hand_over(print));
        assert!(!jobs.cancel(print), "a stop that cannot happen is refused");
        assert!(jobs.get(other).is_some_and(|j| j.stoppable && !j.stopping), "the other job is untouched");
        assert_eq!(jobs.cancel_all(), 1, "only the job that can be stopped is asked");
        assert!(jobs.get(other).is_some_and(|j| j.stopping));
        assert!(jobs.get(print).is_some_and(|j| !j.stoppable && !j.stopping), "a late stop is not shown as one");
        assert!(!jobs.hand_over(crate::jobs::JobId(99)), "no such job");
    }

    #[test]
    fn a_batch_stops_before_the_next_item_and_keeps_what_it_finished() {
        let cancel = Cancel::new();
        let mut started = Vec::new();
        let outcome = run_batch(
            &cancel,
            vec!["a", "b", "c", "d"],
            |_| {},
            |item| {
                started.push(item);
                if started.len() == 2 {
                    // The reader asks to stop while the second item is in hand: it finishes.
                    cancel.cancel();
                }
                Ok::<(), String>(())
            },
        );
        assert_eq!(started, vec!["a", "b"], "no item starts after the stop");
        assert_eq!(outcome.done, 2);
        assert!(outcome.cancelled);
        assert_eq!(outcome.seen(), 2);
    }

    #[test]
    fn a_batch_stopped_before_it_starts_touches_nothing() {
        let cancel = Cancel::new();
        cancel.cancel();
        let mut touched = 0;
        let outcome = run_batch(
            &cancel,
            vec![1, 2, 3],
            |_| {},
            |_| {
                touched += 1;
                Ok::<(), String>(())
            },
        );
        assert_eq!(touched, 0, "a stop that arrives first means no work at all");
        assert_eq!(outcome.done, 0);
        assert!(outcome.cancelled);
    }

    #[test]
    fn a_worker_is_running_until_its_own_handle_is_dropped() {
        let alive = Alive::new();
        let held = alive.clone();
        assert!(alive.is_running(), "nothing has ended yet");
        let worker = held.worker();
        assert!(alive.is_running(), "having a handle is not having ended");
        drop(worker);
        assert!(!alive.is_running(), "the worker ended, and the copy the card holds says so");
        // A fresh one is a fresh worker, not the end of the old one.
        assert!(Alive::new().is_running());
    }

    #[test]
    fn a_worker_that_unwinds_has_ended_too() {
        // The case that made this type necessary: a panic on one awkward page used to leave the
        // card spinning and the next run of that kind refused for the life of the session.
        let alive = Alive::new();
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe({
            let alive = alive.clone();
            move || {
                let _worker = alive.worker();
                panic!("the page could not be rendered");
            }
        }));
        std::panic::set_hook(hook);
        assert!(unwound.is_err(), "the panic is the test's own");
        assert!(!alive.is_running(), "an unwind drops the worker's handle like a return does");
    }

    #[test]
    fn a_batch_records_each_failure_and_carries_on() {
        let cancel = Cancel::new();
        let outcome = run_batch(&cancel, vec![1, 2, 3, 4], |_| {}, |n| if n % 2 == 0 { Err(format!("item {n} broke")) } else { Ok(()) });
        assert_eq!(outcome.done, 2, "the failures do not stop the others");
        assert_eq!(outcome.failed, vec!["item 2 broke".to_string(), "item 4 broke".to_string()]);
        assert!(!outcome.cancelled);
        assert_eq!(outcome.seen(), 4);
    }

    #[test]
    fn a_stop_that_arrives_after_the_last_item_is_not_a_cancellation() {
        let cancel = Cancel::new();
        let outcome = run_batch(&cancel, vec![1, 2], |_| {}, |_| Ok::<(), String>(()));
        cancel.cancel();
        assert_eq!(outcome.done, 2);
        assert!(!outcome.cancelled, "everything was done, so nothing was given up");
    }

    #[test]
    fn progress_is_reported_before_each_item() {
        let cancel = Cancel::new();
        let mut reported = Vec::new();
        let outcome = run_batch(&cancel, vec![1, 2, 3], |done| reported.push(done), |n| if n == 2 { Err(()) } else { Ok(()) });
        assert_eq!(reported, vec![0, 1, 1], "the count is what had finished, not what was started");
        assert_eq!(outcome.done, 2);
        assert_eq!(outcome.failed.len(), 1);
    }
}
