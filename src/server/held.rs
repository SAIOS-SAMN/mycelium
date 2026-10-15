// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE KERNEL LOCK, COUNTED (2026-10-08) AND METERED (78.153b): who blocks
// whom, and what each hold read. Every reflex worker waited on the kernel
// while the door deferred nearly every act; no reading said which holder
// kept it. A lock taken through `held!` names its site as the holder; an
// attempt that finds the lock taken charges ONE block to the site holding it
// at that moment. Exact counts, no clock (law 9): a site that blocks often
// is a site that holds long or often.
//
//! 78.153b — THE HOLD'S ONE STROKE. The lock audit (docs/LOCK-AUDIT.md)
//! found no path under the kernel metered end to end: the guard dereferenced
//! to `Kernel`, so any call under it was one keystroke from an unmetered
//! walk. A [`Held`] no longer dereferences: `held!` mints the hold's ONE
//! stroke at the take, and the only way in is [`Held::run`], which hands the
//! closure the kernel AND that stroke. An input-sized kernel entry point
//! takes `&mut Budget`, so a call under the guard spends the hold's reads or
//! does not compile; `Kernel::stroke` is the crate's, never the binary's.
//! The server binary and the library (`synthesis`, `backfill`) take the
//! kernel through this one guard, so the book counts every hold.

use crate::cognition::Kernel;
use crate::physics::Budget;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};

/// One site's counts: how often it took the lock, how often it had to wait,
/// and how often another site waited on it.
struct Site {
    at: &'static str,
    took: u64,
    waited: u64,
    blocked: u64,
}

struct Book {
    holder: Option<&'static str>,
    /// Which hold this is: every take of the kernel numbered in turn, so two
    /// readings of one hold are told from two holds of one site.
    hold: u64,
    /// 78.152 TU-7 — the final-point advances the door's shelf had counted
    /// when this hold was taken: a hold is measured in TUniversal, never in
    /// seconds.
    advances_at: u64,
    /// 78.153b — the waiters this hold has blocked so far.
    blocked_now: u64,
    sites: Vec<Site>,
}

static BOOK: Mutex<Book> = Mutex::new(Book { holder: None, hold: 0, advances_at: 0, blocked_now: 0, sites: Vec::new() });

/// The reader of the final-point advances a hold is stamped with: the
/// server's shelf, handed in once at boot. Read nowhere else; a process
/// that hands none (a library caller, a test) stamps zero.
static ADVANCES: OnceLock<fn() -> u64> = OnceLock::new();

/// Hand the book the reader of the final-point advances (78.152 TU-7); the
/// first reader handed stands.
pub fn read_advances_with(read: fn() -> u64) {
    let _ = ADVANCES.set(read);
}

fn advances() -> u64 {
    match ADVANCES.get() {
        Some(read) => read(),
        None => 0,
    }
}

/// THE HOLD STANDING NOW: its site and its number, or none. Read without the
/// kernel (the observer's door), so a court whose kernel one thread holds
/// past every waiter still says WHICH site holds it — hypha c 2026-10-09:
/// one thread spun 21 minutes under the kernel, thirteen waited, the journal
/// fell silent, and no reading named the site.
pub fn holding() -> Option<(&'static str, u64)> {
    let b = BOOK.lock().ok()?;
    b.holder.map(|h| (h, b.hold))
}

/// The hold standing now with the final-point advances counted at its take
/// (the watchdog's reading, 78.152 TU-7).
pub fn holding_since() -> Option<(&'static str, u64, u64)> {
    let b = BOOK.lock().ok()?;
    b.holder.map(|h| (h, b.hold, b.advances_at))
}

/// Every take of the kernel through `held!` since boot: the book's hold
/// number. A path that takes no kernel leaves it where it stood (78.152b).
pub fn holds() -> u64 {
    BOOK.lock().map_or(0, |b| b.hold)
}

thread_local! {
    /// The takes of the kernel THIS THREAD made through `held!`: what a path
    /// run on it took, apart from every other thread's holds in the process
    /// (the book counts them all).
    static HOLDS_HERE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Every take of the kernel through `held!` by THIS thread: a path that takes
/// no kernel leaves it where it stood, whatever other threads take beside it.
pub fn holds_here() -> u64 {
    HOLDS_HERE.with(|h| h.get())
}

/// THE STEP A LONG HOLD STANDS IN: a site that does several things under
/// one hold names each as it enters it, so a hold that stands across the
/// observer's questions says which step it stands in (hypha a, 2026-10-09:
/// the court loop held 20+ minutes, the site alone could not say whether in
/// the stroke or in the publish). One marker for the server and the library
/// (`observe::step`), so the court stroke names its own steps inside the
/// bin's.
pub fn step(name: &'static str) {
    crate::observe::step(name);
}

pub fn stepping() -> &'static str {
    crate::observe::stepping()
}

/// Threads blocked on the kernel through `held!` right now, and how many
/// such waits have been served since boot.
static WAITING: AtomicU64 = AtomicU64::new(0);
static SERVED: AtomicU64 = AtomicU64::new(0);

/// THE COURT LETS THE QUEUE IN BEFORE IT TAKES THE KERNEL AGAIN. The court
/// loop re-took the kernel the instant it let go — 14 806 times in five
/// minutes on hypha c — and a std mutex is not fair, so a reflex worker
/// woken by the release lost the race every time: the workers starved, the
/// pool filled, and the door deferred nearly every act (2026-10-08). Each
/// wait queued when this is called is served once before it returns; one
/// that queues after waits its turn behind the next stroke. Counts, no
/// clock.
pub fn yield_to_waiters() {
    let queued = WAITING.load(Ordering::Acquire);
    if queued == 0 {
        return;
    }
    // Past 2⁶⁴ served waits there is no turn to count to, and none is waited.
    let Some(until) = SERVED.load(Ordering::Acquire).checked_add(queued) else { return };
    while SERVED.load(Ordering::Acquire) < until && WAITING.load(Ordering::Acquire) > 0 {
        std::thread::yield_now();
    }
}

fn with_site(book: &mut Book, at: &'static str, f: impl FnOnce(&mut Site)) {
    if let Some(s) = book.sites.iter_mut().find(|s| s.at == at) {
        f(s);
        return;
    }
    let mut s = Site { at, took: 0, waited: 0, blocked: 0 };
    f(&mut s);
    book.sites.push(s);
}

/// THE KERNEL, HELD BY ONE NAMED SITE, WITH THE HOLD'S ONE STROKE. Never a
/// `Deref` to the kernel: [`Held::run`] is the way in, and it hands the
/// stroke beside the kernel, so an input-sized read under the lock spends
/// it (78.153b).
pub struct Held<'a> {
    guard: MutexGuard<'a, Kernel>,
    stroke: Budget,
    at: &'static str,
    /// This thread's granted reads at the take (`physics::spent_here`).
    spent_at: u64,
}

impl Held<'_> {
    /// THE ONLY WAY IN: the kernel and the hold's stroke, together. A read
    /// that walks what grows spends `reads`; a stroke spent ends it with its
    /// unread named (`Replayed::Unread`, a deferral, a page cut short) —
    /// never a wrong answer.
    pub fn run<R>(&mut self, f: impl FnOnce(&mut Kernel, &mut Budget) -> R) -> R {
        f(&mut self.guard, &mut self.stroke)
    }

    /// The reads the hold's stroke still grants.
    pub fn remaining(&self) -> usize {
        self.stroke.remaining()
    }

    /// LET GO OF THE KERNEL, KEEP THE STROKE: the hold's stroke handed on
    /// whole, for work paced by the court's metabolism off the lock — an
    /// offer round, a fold round, a catch-up's pages on the wire.
    pub fn into_stroke(mut self) -> Budget {
        self.stroke.hand_over()
    }
}

/// THE UNMETERED-HOLD BOUND: a hold that read nothing while this many
/// waiters queued behind it is named when it lets go — one aperture of
/// strokes (`physics::aperture_cycle`), the same span the watchdog reads a
/// standing hold against; never a tuned count.
fn unmetered_bound() -> u64 {
    u64::from(crate::physics::aperture_cycle())
}

impl Drop for Held<'_> {
    fn drop(&mut self) {
        let read = crate::physics::spent_here().checked_sub(self.spent_at);
        if let Ok(mut b) = BOOK.lock() {
            let blocked = b.blocked_now;
            if read == Some(0) && blocked >= unmetered_bound() {
                eprintln!("  unmetered hold {} (hold {}, step {}) read nothing and blocked {blocked}", self.at, b.hold, stepping());
            }
            b.holder = None;
        }
    }
}

/// Take the kernel at site `at` with the hold's one stroke (`Kernel::stroke`,
/// the court's own metabolism); `Err(())` where the lock is poisoned, as
/// `lock()` refuses.
#[expect(clippy::result_unit_err, reason = "`held!` keeps `lock()`'s shape at every site: a poisoned lock is the one refusal, and it carries nothing to name")]
pub fn lock<'a>(m: &'a Mutex<Kernel>, at: &'static str) -> Result<Held<'a>, ()> {
    let (mut guard, waited) = match m.try_lock() {
        Ok(g) => (g, false),
        Err(TryLockError::Poisoned(_)) => return Err(()),
        Err(TryLockError::WouldBlock) => {
            if let Ok(mut b) = BOOK.lock() {
                if let Some(n) = b.blocked_now.checked_add(1) {
                    b.blocked_now = n;
                }
                let blocker = b.holder;
                if let Some(h) = blocker {
                    with_site(&mut b, h, |s| {
                        s.blocked += 1;
                        if s.blocked.is_power_of_two() {
                            eprintln!("  kernel lock {} blocked others {} (took {} waited {})", s.at, s.blocked, s.took, s.waited);
                        }
                    });
                }
            }
            WAITING.fetch_add(1, Ordering::AcqRel);
            let g = m.lock();
            WAITING.fetch_sub(1, Ordering::AcqRel);
            SERVED.fetch_add(1, Ordering::AcqRel);
            (g.map_err(|_| ())?, true)
        }
    };
    if let Ok(mut b) = BOOK.lock() {
        b.holder = Some(at);
        b.advances_at = advances();
        b.blocked_now = 0;
        // A hold numbered past 2⁶⁴ takes is not numbered again; the site
        // still names it.
        if let Some(n) = b.hold.checked_add(1) {
            b.hold = n;
        }
        with_site(&mut b, at, |s| {
            s.took += 1;
            if waited {
                s.waited += 1;
            }
        });
    }
    HOLDS_HERE.with(|h| h.set(h.get().wrapping_add(1)));
    let stroke = guard.stroke();
    Ok(Held { guard, stroke, at, spent_at: crate::physics::spent_here() })
}

/// `held!(kernel)` — `kernel.lock()` counted at this file and line, with the
/// hold's one stroke.
macro_rules! held {
    ($m:expr) => {
        $crate::server::held::lock(&$m, concat!(file!(), ":", line!()))
    };
}
pub(crate) use held;

#[cfg(test)]
mod tests;
