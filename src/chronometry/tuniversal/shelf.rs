// 78.152 TU-7 — THE DOOR'S TUNIVERSAL SHELF: the greatest final point this
// door has seen, kept beside the 78.151 published reading and OUTSIDE the
// kernel, raised from the keepers' verified readings; and what speakers
// carried in, held apart as heard, never as final.

use super::point::TPoint;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Condvar, Mutex};

/// What the shelf holds under its lock.
#[derive(Debug, Default, Clone, Copy)]
struct Held {
    /// The greatest point final in this door's view (TU-3).
    final_point: Option<TPoint>,
    /// The greatest point a speaker CARRIED in on a frame (TU-7): a claim the
    /// door read, not a quorum's — never folded into the final point.
    heard: Option<TPoint>,
    /// Frames read at the door, counted: an act arriving is work.
    acts: u64,
}

/// THE SHELF. A watchdog or the court loop reads it without the kernel.
#[derive(Debug, Default)]
pub struct TShelf {
    held: Mutex<Held>,
    moved: Condvar,
    /// How many times the final point advanced: the count a hold is
    /// measured in (TU-7), read without the shelf's lock.
    advances: AtomicU64,
}

/// What a parked reader saw last: the advances and the acts.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Seen {
    pub advances: u64,
    pub acts: u64,
}

impl TShelf {
    pub fn new() -> TShelf {
        TShelf::default()
    }

    fn held(&self) -> Held {
        match self.held.lock() {
            Ok(h) => *h,
            Err(p) => *p.into_inner(),
        }
    }

    /// The greatest final point this door has seen; `None` before the first.
    pub fn final_point(&self) -> Option<TPoint> {
        self.held().final_point
    }

    /// The greatest point a speaker carried in; a claim, never final.
    pub fn heard(&self) -> Option<TPoint> {
        self.held().heard
    }

    /// The final point's advances, counted.
    pub fn advances(&self) -> u64 {
        self.advances.load(Ordering::Acquire)
    }

    /// What a reader sees now.
    pub fn seen(&self) -> Seen {
        Seen { advances: self.advances(), acts: self.held().acts }
    }

    /// A FINAL POINT raised (from the keepers' quorum, TU-3): held where it
    /// advances on the one held, every parked reader woken. `true` where it
    /// advanced.
    pub fn raise_final(&self, p: TPoint) -> bool {
        let raised = {
            let mut h = match self.held.lock() {
                Ok(h) => h,
                Err(e) => e.into_inner(),
            };
            match p.advances(h.final_point.as_ref()) {
                true => {
                    h.final_point = Some(p);
                    true
                }
                false => false,
            }
        };
        if raised {
            // One per advance in one process: a u64 is not passed.
            self.advances.fetch_add(1, Ordering::AcqRel);
            self.moved.notify_all();
        }
        raised
    }

    /// A FRAME READ AT THE DOOR, with the point its speaker carried: the act
    /// is counted and wakes a parked reader (work arrived); the point is held
    /// as heard where it advances on the heard one.
    pub fn heard_at_door(&self, point: Option<TPoint>) {
        {
            let mut h = match self.held.lock() {
                Ok(h) => h,
                Err(e) => e.into_inner(),
            };
            if let Some(n) = h.acts.checked_add(1) {
                h.acts = n;
            }
            if let Some(p) = point {
                if p.advances(h.heard.as_ref()) {
                    h.heard = Some(p);
                }
            }
        }
        self.moved.notify_all();
    }

    /// PARK UNTIL SOMETHING MOVED SINCE `seen` — the final point advanced or
    /// an act arrived — bounded by `floor`, the one scheduler floor
    /// (`physics::stroke_floor`, 78.152): the wait ends on the first motion;
    /// the floor is told to the scheduler and read by no one. Answers what it
    /// saw on waking.
    #[expect(
        clippy::disallowed_types,
        reason = "law 9, 78.152 — the scheduler floor bounds a park and is told to the scheduler in its own units; no reading is taken from it"
    )]
    pub fn park(&self, seen: Seen, floor: std::time::Duration) -> Seen {
        let guard = match self.held.lock() {
            Ok(h) => h,
            Err(e) => e.into_inner(),
        };
        let moved = |h: &Held| h.acts != seen.acts || self.advances() != seen.advances;
        if !moved(&guard) {
            let _ = self.moved.wait_timeout_while(guard, floor, |h| !moved(h));
        }
        self.seen()
    }
}
