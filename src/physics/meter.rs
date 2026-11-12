// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.182 item 5 (#221) — THE STROKE METER: a hold's stroke and the reads
//! spent under it, read across threads without a lock.
//!
//! A hold is metered in reads (78.153b): it is taken with one stroke, and
//! every read granted on the holding thread is counted ([`super::spent_here`]).
//! That count was the thread's own, so a waiter could not read how far the
//! hold it waited behind had gone, and a patience counted in the waiter's
//! own polls measured nothing about that hold (#221, the `reflex-patience`
//! finding). The meter carries the hold's `(stroke, spent)` in atomics: the
//! holding thread adds each read it is granted, and a waiter reads both
//! with no lock. A hold that has spent past its stroke is OVERRUN; a waiter
//! that finds it so is answered at once, and the holder is flagged: its next
//! metered read is refused, as a spent token refuses one, so its work ends
//! by its own named refusal and never with a wrong answer.
//!
//! A waiter that finds no overrun parks on the one GATE until a release or
//! an overrun moves it: the gate counts releases, and a read that passes a
//! stroke wakes it.

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex};

/// A hold's stroke against the reads spent under it, when the reads have
/// passed the stroke: what a waiter behind the hold is told.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Overrun {
    /// The hold's number, as the court-lock book counts takes.
    pub hold: u64,
    /// The holder, as the caller numbers its holders (the court-lock
    /// book's site).
    pub holder: u64,
    /// Reads granted on the holding thread since the take.
    pub spent: u64,
    /// The reads the hold's stroke granted at the take.
    pub stroke: u64,
}

/// ONE HELD SLOT'S METER. `slot` names the lock it meters (the lock's
/// address; zero while free). A stroke of `u64::MAX` stands while unbound,
/// so a reading taken across a release never reads an overrun.
pub struct StrokeMeter {
    slot: AtomicUsize,
    hold: AtomicU64,
    holder: AtomicU64,
    stroke: AtomicU64,
    spent: AtomicU64,
    preempt: AtomicBool,
}

impl Default for StrokeMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl StrokeMeter {
    pub const fn new() -> Self {
        StrokeMeter {
            slot: AtomicUsize::new(0),
            hold: AtomicU64::new(0),
            holder: AtomicU64::new(0),
            stroke: AtomicU64::new(u64::MAX),
            spent: AtomicU64::new(0),
            preempt: AtomicBool::new(false),
        }
    }

    /// CLAIM THIS METER for the slot `slot`, hold `hold` by `holder`, with a
    /// stroke of `stroke` reads, and bind it to this thread; none where
    /// another slot holds it.
    pub fn claim(&'static self, slot: usize, hold: u64, holder: u64, stroke: u64) -> Option<Bound> {
        if slot == 0 || self.slot.compare_exchange(0, slot, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        self.spent.store(0, Ordering::Release);
        self.preempt.store(false, Ordering::Release);
        self.hold.store(hold, Ordering::Release);
        self.holder.store(holder, Ordering::Release);
        self.stroke.store(stroke, Ordering::Release);
        let before = BOUND.with(|b| b.replace(Some(self)));
        Some(Bound { meter: self, before })
    }

    /// The overrun of the hold on `slot`, if this meter holds that slot and
    /// its reads have passed its stroke. Read without a lock: the slot and
    /// the hold are read again after the counts, so a reading that straddles
    /// a release or a new claim reads nothing.
    pub fn overrun(&self, slot: usize) -> Option<Overrun> {
        if slot == 0 || self.slot.load(Ordering::Acquire) != slot {
            return None;
        }
        let hold = self.hold.load(Ordering::Acquire);
        let holder = self.holder.load(Ordering::Acquire);
        let stroke = self.stroke.load(Ordering::Acquire);
        let spent = self.spent.load(Ordering::Acquire);
        let same = self.slot.load(Ordering::Acquire) == slot && self.hold.load(Ordering::Acquire) == hold;
        (same && spent > stroke).then_some(Overrun { hold, holder, spent, stroke })
    }

    /// Flag the holder for preemption: its next metered read is refused.
    pub fn preempt(&self) {
        self.preempt.store(true, Ordering::Release);
    }

    /// One read granted on the holding thread: counted, and the gate woken
    /// on the read that passes the stroke.
    fn count(&self) {
        let n = self.spent.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        if n == self.stroke.load(Ordering::Acquire).wrapping_add(1) {
            GATE.notify();
        }
    }
}

/// A CLAIMED METER, bound to the holding thread: released once, by the
/// hold that claimed it, before the lock is let go.
pub struct Bound {
    meter: &'static StrokeMeter,
    /// The meter this thread stood under before the claim, restored at the
    /// release (a hold of a second slot inside a first).
    before: Option<&'static StrokeMeter>,
}

impl Bound {
    /// LET THE SLOT GO: the reading reset (an unbound meter never reads an
    /// overrun), the meter freed, and the thread's binding restored. The
    /// lock itself is let go after; [`released`] then tells the waiters.
    pub fn release(self) {
        BOUND.with(|b| b.set(self.before));
        let m = self.meter;
        m.stroke.store(u64::MAX, Ordering::Release);
        m.spent.store(0, Ordering::Release);
        m.preempt.store(false, Ordering::Release);
        m.slot.store(0, Ordering::Release);
    }
}

/// THE GATE a waiter parks on: the releases counted so far, and the one
/// condition every release and every overrun wakes.
struct Gate {
    released: Mutex<u64>,
    moved: Condvar,
}

impl Gate {
    fn notify(&self) {
        // Taken and let go before the wake, so a waiter between its reading
        // and its wait is woken, never missed.
        drop(self.released.lock());
        self.moved.notify_all();
    }
}

static GATE: Gate = Gate { released: Mutex::new(0), moved: Condvar::new() };

/// The releases counted so far: read before a waiter's try, so a release
/// between the try and the wait is not missed.
pub fn releases() -> u64 {
    match GATE.released.lock() {
        Ok(r) => *r,
        Err(e) => *e.into_inner(),
    }
}

/// A lock was let go: counted, and every waiter woken.
pub fn released() {
    let mut r = match GATE.released.lock() {
        Ok(r) => r,
        Err(e) => e.into_inner(),
    };
    *r = r.wrapping_add(1);
    drop(r);
    GATE.moved.notify_all();
}

/// WAIT for a release past `seen`, or until `overrun` reads true.
pub fn wait(seen: u64, overrun: &dyn Fn() -> bool) {
    let guard = match GATE.released.lock() {
        Ok(r) => r,
        Err(e) => e.into_inner(),
    };
    drop(GATE.moved.wait_while(guard, |r| *r == seen && !overrun()));
}

thread_local! {
    /// The meter of the hold this thread stands in, if any.
    static BOUND: std::cell::Cell<Option<&'static StrokeMeter>> = const { std::cell::Cell::new(None) };
}

/// Whether the hold this thread stands in was flagged for preemption.
pub(super) fn preempted_here() -> bool {
    BOUND.with(|b| b.get().is_some_and(|m| m.preempt.load(Ordering::Acquire)))
}

/// One read granted on this thread, counted on the meter it stands under.
pub(super) fn count_here() {
    BOUND.with(|b| {
        if let Some(m) = b.get() {
            m.count();
        }
    });
}
