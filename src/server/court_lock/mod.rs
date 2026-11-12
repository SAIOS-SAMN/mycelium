// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE COURT LOCK, COUNTED (2026-10-08) AND METERED (78.153b): who blocks
// whom, and what each hold read. Every reflex worker waited on the court
// while the door deferred nearly every act; no reading said which holder
// kept it. A lock taken through `held!` names its site as the holder; an
// attempt that finds the lock taken charges ONE block to the site holding it
// at that moment. Exact counts, no clock (law 9): a site that blocks often
// is a site that holds long or often.
//
//! 78.153b — THE HOLD'S ONE STROKE. The lock audit (MU-RECURSION 78.153)
//! found no path under the court metered end to end: the guard dereferenced
//! to `Court`, so any call under it was one keystroke from an unmetered
//! walk. A [`CourtGuard`] no longer dereferences: `held!` mints the hold's ONE
//! stroke at the take, and the only way in is [`CourtGuard::run`], which hands the
//! closure the court AND that stroke. An input-sized court entry point
//! takes `&mut Budget`, so a call under the guard spends the hold's reads or
//! does not compile; `Court::stroke` is the crate's, never the binary's.
//! The server binary and the library (`synthesis`, `backfill`) take the
//! court through this one guard, so the book counts every hold.

use crate::cognition::Court;
use crate::physics::meter::{Bound, Overrun, StrokeMeter};
use crate::physics::Budget;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};

/// One site's counts: how often it took the lock, how often it had to wait,
/// and how often another site waited on it.
struct LockCounts {
    at: &'static str,
    took: u64,
    waited: u64,
    blocked: u64,
    /// 78.182 item 5 — waiters answered `Deferred` while this site held
    /// past its stroke.
    deferred: u64,
}

struct Book {
    holder: Option<&'static str>,
    /// Which hold this is: every take of the court numbered in turn, so two
    /// readings of one hold are told from two holds of one site.
    hold: u64,
    /// 78.152 TU-7 — the final-point advances the door's shelf had counted
    /// when this hold was taken: a hold is measured in TUniversal, never in
    /// seconds.
    advances_at: u64,
    /// 78.153b — the waiters this hold has blocked so far.
    blocked_now: u64,
    sites: Vec<LockCounts>,
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
/// court (the observer's door), so a court whose state one thread holds
/// past every waiter still says WHICH site holds it — hypha c 2026-10-09:
/// one thread spun 21 minutes under the court, thirteen waited, the journal
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

/// Every take of the court through `held!` since boot: the book's hold
/// number. A path that takes no court leaves it where it stood (78.152b).
pub fn holds() -> u64 {
    BOOK.lock().map_or(0, |b| b.hold)
}

thread_local! {
    /// The takes of the court THIS THREAD made through `held!`: what a path
    /// run on it took, apart from every other thread's holds in the process
    /// (the book counts them all).
    static HOLDS_HERE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Every take of the court through `held!` by THIS thread: a path that takes
/// no court leaves it where it stood, whatever other threads take beside it.
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

/// Threads blocked on the court through `held!` right now, and how many
/// such waits have been served since boot.
static WAITING: AtomicU64 = AtomicU64::new(0);
static SERVED: AtomicU64 = AtomicU64::new(0);

/// THE COURT LETS THE QUEUE IN BEFORE IT TAKES THE COURT AGAIN. The court
/// loop re-took the court the instant it let go — 14 806 times in five
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

/// The site `at`'s counts, made where the book has none, and `f` run on
/// them; answers the site's place in the book, the number a meter names its
/// holder by.
fn with_site(book: &mut Book, at: &'static str, f: impl FnOnce(&mut LockCounts)) -> usize {
    if let Some(i) = book.sites.iter().position(|s| s.at == at) {
        f(&mut book.sites[i]);
        return i;
    }
    let mut s = LockCounts { at, took: 0, waited: 0, blocked: 0, deferred: 0 };
    f(&mut s);
    book.sites.push(s);
    book.sites.len() - 1
}

/// 78.182 item 5 (#221) — THE METERS OF THE HELD SLOTS: a chain of meters
/// a hold claims a free one of for its lock, and a waiter finds its lock's
/// by the lock's address, both without a lock. The chain grows by one meter
/// where every meter is claimed (each link set once, never taken back), so
/// it is as long as the most courts this process ever held at once: one in
/// a server, as many as run beside each other in a test binary. No width is
/// chosen.
struct MeterLink {
    meter: StrokeMeter,
    next: OnceLock<&'static MeterLink>,
}

static METERS: MeterLink = MeterLink { meter: StrokeMeter::new(), next: OnceLock::new() };

/// Every meter of the chain, from the first.
fn meters() -> impl Iterator<Item = &'static StrokeMeter> {
    std::iter::successors(Some(&METERS), |l| l.next.get().copied()).map(|l| &l.meter)
}

/// A free meter claimed for `slot`: the first free one of the chain, or a
/// new link where every one is claimed.
fn claim(slot: usize, hold: u64, holder: u64, stroke: u64) -> Option<Bound> {
    // A lock's address is never zero, the free slot's mark: no meter is
    // claimed for it, and the chain is not grown for it.
    if slot == 0 {
        return None;
    }
    let mut link: &'static MeterLink = &METERS;
    loop {
        if let Some(b) = link.meter.claim(slot, hold, holder, stroke) {
            return Some(b);
        }
        link = link.next.get_or_init(|| Box::leak(Box::new(MeterLink { meter: StrokeMeter::new(), next: OnceLock::new() })));
    }
}

/// The lock's address: the slot its meter is claimed for.
fn slot_of(m: &Mutex<Court>) -> usize {
    std::ptr::from_ref(m) as usize
}

/// The meter claimed for `slot`, if a hold claimed one.
fn meter_of(slot: usize) -> Option<(&'static StrokeMeter, Overrun)> {
    meters().find_map(|m| m.overrun(slot).map(|o| (m, o)))
}

/// WHY A TAKE OF THE COURT WAS REFUSED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// The lock is poisoned: a holder panicked under it.
    Poisoned,
    /// 78.182 item 5 — the hold this take waited behind had spent past its
    /// stroke: the take is answered at once, never blocked, and the holder
    /// is flagged for preemption at its next metered read. `holder` is the
    /// site holding the court, as the book names it.
    Deferred { holder: Option<&'static str>, spent: u64, stroke: u64, hold: u64 },
}

thread_local! {
    /// The takes THIS thread was answered `Deferred`: a reflex worker reads
    /// it across a judgement, so an act its judge left unanswered for a
    /// deferral is answered with the deferral (`mycelium-server`).
    static DEFERRED_HERE: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// The takes this thread was answered `Deferred` since it started.
pub fn deferred_here() -> u64 {
    DEFERRED_HERE.with(std::cell::Cell::get)
}

/// THE COURT, HELD BY ONE NAMED SITE, WITH THE HOLD'S ONE STROKE. Never a
/// `Deref` to the court: [`CourtGuard::run`] is the way in, and it hands the
/// stroke beside the court, so an input-sized read under the lock spends
/// it (78.153b).
pub struct CourtGuard<'a> {
    /// Released in field order after `Drop::drop`: the meter is let go
    /// first (in `drop`), then this lock, then `wake` tells the waiters.
    guard: MutexGuard<'a, Court>,
    stroke: Budget,
    at: &'static str,
    /// This thread's granted reads at the take (`physics::spent_here`).
    spent_at: u64,
    /// The meter this hold claimed, released before the lock.
    meter: Option<Bound>,
    _wake: Wake,
}

/// Dropped after the lock: every release is told to the waiters, metered
/// or not, so a waiter parked on a release never misses one.
struct Wake;

impl Drop for Wake {
    fn drop(&mut self) {
        crate::physics::meter::released();
    }
}

impl CourtGuard<'_> {
    /// THE ONLY WAY IN: the court and the hold's stroke, together. A read
    /// that walks what grows spends `reads`; a stroke spent ends it with its
    /// unread named (`ReconstructionOutcome::Unread`, a deferral, a page cut short) —
    /// never a wrong answer.
    pub fn run<R>(&mut self, f: impl FnOnce(&mut Court, &mut Budget) -> R) -> R {
        f(&mut self.guard, &mut self.stroke)
    }

    /// The reads the hold's stroke still grants.
    pub fn remaining(&self) -> usize {
        self.stroke.remaining()
    }

    /// LET GO OF THE COURT, KEEP THE STROKE: the hold's stroke handed on
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

impl Drop for CourtGuard<'_> {
    fn drop(&mut self) {
        let read = crate::physics::spent_here().checked_sub(self.spent_at);
        if let Ok(mut b) = BOOK.lock() {
            let blocked = b.blocked_now;
            if read == Some(0) && blocked >= unmetered_bound() {
                eprintln!("  unmetered hold {} (hold {}, step {}) read nothing and blocked {blocked}", self.at, b.hold, stepping());
            }
            b.holder = None;
        }
        if let Some(m) = self.meter.take() {
            m.release();
        }
    }
}

/// Take the court at site `at` with the hold's one stroke (`Court::stroke`,
/// the court's own metabolism); refused where the lock is poisoned, or
/// `Deferred` where the hold it waits behind has spent past its stroke
/// (78.182 item 5).
pub fn lock<'a>(m: &'a Mutex<Court>, at: &'static str) -> Result<CourtGuard<'a>, Refused> {
    let slot = slot_of(m);
    let mut seen = crate::physics::meter::releases();
    let (mut guard, waited) = match m.try_lock() {
        Ok(g) => (g, false),
        Err(TryLockError::Poisoned(_)) => return Err(Refused::Poisoned),
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
                            eprintln!("  court lock {} blocked others {} (took {} waited {})", s.at, s.blocked, s.took, s.waited);
                        }
                    });
                }
            }
            WAITING.fetch_add(1, Ordering::AcqRel);
            let taken = wait_for(m, slot, &mut seen);
            WAITING.fetch_sub(1, Ordering::AcqRel);
            SERVED.fetch_add(1, Ordering::AcqRel);
            (taken?, true)
        }
    };
    let mut hold = 0;
    let mut site = 0;
    if let Ok(mut b) = BOOK.lock() {
        b.holder = Some(at);
        b.advances_at = advances();
        b.blocked_now = 0;
        // A hold numbered past 2⁶⁴ takes is not numbered again; the site
        // still names it.
        if let Some(n) = b.hold.checked_add(1) {
            b.hold = n;
        }
        hold = b.hold;
        site = with_site(&mut b, at, |s| {
            s.took += 1;
            if waited {
                s.waited += 1;
            }
        });
    }
    HOLDS_HERE.with(|h| h.set(h.get().wrapping_add(1)));
    let stroke = guard.stroke();
    // A stroke past 2⁶⁴ reads is never passed: it meters as unbounded.
    let limit = match u64::try_from(stroke.remaining()) {
        Ok(n) => n,
        Err(_) => u64::MAX,
    };
    // A book past 2⁶⁴ sites names its holder by no place: the meter stands
    // unclaimed, and its waiters wait as for an unmetered hold.
    let meter = match u64::try_from(site) {
        Ok(site) => claim(slot, hold, site, limit),
        Err(_) => None,
    };
    Ok(CourtGuard { guard, stroke, at, spent_at: crate::physics::spent_here(), meter, _wake: Wake })
}

/// 78.182 item 5 — A WAIT THAT READS THE HOLD IT WAITS BEHIND. The waiter
/// tries the lock; while it is held, it reads the holder's meter without a
/// lock, and a hold spent past its stroke answers it `Deferred` at once and
/// flags the holder. Otherwise it parks until a release or an overrun.
fn wait_for<'a>(m: &'a Mutex<Court>, slot: usize, seen: &mut u64) -> Result<MutexGuard<'a, Court>, Refused> {
    loop {
        match m.try_lock() {
            Ok(g) => return Ok(g),
            Err(TryLockError::Poisoned(_)) => return Err(Refused::Poisoned),
            Err(TryLockError::WouldBlock) => {}
        }
        if let Some((meter, o)) = meter_of(slot) {
            meter.preempt();
            DEFERRED_HERE.with(|d| d.set(d.get().wrapping_add(1)));
            let holder = BOOK.lock().ok().and_then(|mut b| {
                let i = usize::try_from(o.holder).ok()?;
                let s = b.sites.get_mut(i)?;
                s.deferred += 1;
                if s.deferred.is_power_of_two() {
                    eprintln!("  court lock {} held past its stroke (spent {} of {}): deferred {}", s.at, o.spent, o.stroke, s.deferred);
                }
                Some(s.at)
            });
            return Err(Refused::Deferred { holder, spent: o.spent, stroke: o.stroke, hold: o.hold });
        }
        crate::physics::meter::wait(*seen, &|| meter_of(slot).is_some());
        *seen = crate::physics::meter::releases();
    }
}

/// `held!(court)` — `court.lock()` counted at this file and line, with the
/// hold's one stroke.
macro_rules! held {
    ($m:expr) => {
        $crate::server::court_lock::lock(&$m, concat!(file!(), ":", line!()))
    };
}
pub(crate) use held;

#[cfg(test)]
mod tests;
