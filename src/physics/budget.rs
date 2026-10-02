// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Rule 10 — THE READ TOKEN: what bounds a loop that does not read a
//! boundary.
//!
//! Split from `physics.rs` at the §1.1 600-line reading bound; the items
//! are moved, not rewritten (§1.6).
//!
//! IT DOES NOT DIVIDE, AND TWO ATTEMPTS TO MAKE IT ARE RECORDED HERE SO A
//! THIRD IS NOT WRITTEN. `cover_share()` gave a sub-read a constant — half
//! of `spend_unit` — and hid starvation behind it: measured over twelve
//! tasks, seven covers came back unmeasured at the machine's own stroke
//! and THE SAME SEVEN came back unmeasured at a stroke of 1 048 576,
//! because both handed the cover the same number. `Budget::halve` replaced
//! it with half of whatever the caller held, which scales but puts a
//! chosen proportion where the constant had been — the same class of
//! invented number, one step quieter, and rule 11 refuses the class and
//! not the value.
//!
//! A sub-read spends the caller's token and is atomic by rollback of its
//! RESULT: it returns a value only where it ran to the end, so a caller
//! cannot read half a measurement. The reads it spent are spent, and the
//! cost shows in `exhausted` and in the sub-read's own unmeasured count,
//! where it can be read. What turns that cost into strokes is a cursor
//! (L20), not a divisor: the sub-read resumes where it stopped, as
//! `stroke_fills` already resumes from `fill_cursor` under S13.3.

/// Linear token for one stroke's keyed reads. Non-Copy, non-Clone: a
/// function that wants more than a fiber must take a Budget in its
/// signature. Constructed only by [`super::Physics::stroke`].
#[derive(Debug)]
pub struct Budget {
    pub(super) remaining: usize,
}

impl Budget {
    /// Reads still granted on this token.
    pub fn remaining(&self) -> usize {
        self.remaining
    }

    /// Spend one read. False when the token is exhausted.
    pub fn spend(&mut self) -> bool {
        if self.remaining == 0 {
            return false;
        }
        self.remaining -= 1;
        true
    }
}

/// THE ANISOTROPIC RESERVE — attention as a four-lane vector, not a tick
/// (65, ruled 2026-09-22).
///
/// `Budget` counts calls to `spend()`, which is a count of IMPLEMENTATION
/// EVENTS and not of physical work. That is why a stroke calibrated against
/// it decays silently: purging the ring stencil removed meet-yielding
/// sites, the same deformation cost fewer `spend()` calls, and
/// `tests/cover.rs`'s `STROKE = 4` stopped forcing the resume it was
/// written to prove. A constant denominated in an implementation detail
/// tracks the implementation, and every refactor falsifies it without
/// touching the physics.
///
/// AND IT IS A SCALAR COLLAPSE. Law 5 reads per lane, and attention is
/// ALREADY a four-lane quantity in this engine — `Bubble.escrow` is "the
/// μ⃗ drawn from the voice's reserve to keep it, per lane", and "a mind
/// rich on `𝒽` and spent on `𝒮` holds turns and withholds placements". A
/// single integer can express neither half of that: a mind bankrupt on
/// scale burns a global counter on an expensive dilation attempt and is
/// preempted out of turning and sliding, which it could still afford.
///
/// ```text
///     A⃗ = (S, G, T, H) = (scale, gauge/rotor, translation, hue/fibre)
/// ```
///
/// A DRAW IS A VECTOR AND IT REFUSES PER LANE. `draw` deducts a whole
/// `[u64; 4]` or deducts nothing, and it refuses naming WHICH lane is
/// depleted — so a mind does not "run out of time", it exhausts specific
/// degrees of freedom and goes on working in the ones it still holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reserve {
    lanes: [u64; 4],
    /// THE RESERVE'S OWN LEDGER: what it gave, per lane; what came back;
    /// and how many draws it refused. Carried by the reserve itself so its
    /// flux reads wherever it lives — the thread-local ledger (`note_draw`)
    /// sees only its own thread, and a court's reserve is drawn on many.
    drawn: [u64; 4],
    restored: [u64; 4],
    refused: u64,
}

/// WHICH LANE REFUSED A DRAW. Not a boolean: "the mind is out of
/// attention" and "the mind cannot afford THIS, on the scale lane" are
/// different facts, and only the second tells a caller what it may still
/// attempt (rule 11).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Depleted {
    /// The lane that could not cover the draw. Where several could not, it
    /// is the first in `(S, G, T, H)` order, which is content order and
    /// not a priority.
    pub lane: usize,
    /// What was asked on that lane, and what was held.
    pub asked: u64,
    pub held: u64,
}

impl Reserve {
    pub fn of(lanes: [u64; 4]) -> Reserve {
        Reserve { lanes, drawn: [0; 4], restored: [0; 4], refused: 0 }
    }

    pub fn lanes(&self) -> [u64; 4] {
        self.lanes
    }

    /// THE LEDGER: drawn and restored per lane, and draws refused.
    pub fn ledger(&self) -> ([u64; 4], [u64; 4], u64) {
        (self.drawn, self.restored, self.refused)
    }

    /// A lane total stepped, or left where it was past its carrier (the
    /// global ledger counts that refusal).
    fn book(total: &mut [u64; 4], by: &[u64; 4]) {
        for (t, b) in total.iter_mut().zip(by) {
            crate::physics::metabolism::step_or_refuse(t, *b);
        }
    }

    /// Is this reserve spent on every lane? A mind with nothing left
    /// anywhere yields; one with anything left on any lane does not.
    pub fn is_spent(&self) -> bool {
        self.lanes.iter().all(|&l| l == 0)
    }

    /// DRAW A VECTOR, ALL OR NOTHING. The whole deduction lands or none of
    /// it does, so a refused action leaves the reserve exactly as it was
    /// and a caller can attempt a different lane without having paid for
    /// the one it could not afford.
    ///
    /// AN ACTION WITH A ZERO PROJECTION ON A DEPLETED LANE IS AFFORDABLE.
    /// That is the whole content of the anisotropy: preemption happens only
    /// where the action needs a lane that is gone, never because some other
    /// lane ran out.
    pub fn draw(&mut self, cost: [u64; 4]) -> Result<(), Depleted> {
        for (i, (&want, &held)) in cost.iter().zip(self.lanes.iter()).enumerate() {
            if want > held {
                // D2 — the refusal is the metabolic reading, counted on the
                // lane that refused it before the return (PHY1).
                crate::physics::metabolism::note_refused(i);
                self.refused += 1;
                return Err(Depleted { lane: i, asked: want, held });
            }
        }
        for (l, c) in self.lanes.iter_mut().zip(cost.iter()) {
            *l -= c;
        }
        crate::physics::metabolism::note_draw(&cost);
        Self::book(&mut self.drawn, &cost);
        Ok(())
    }

    /// Return a draw to the reserve — the conjugate of `draw`, because an
    /// escrow that can only be spent is the ratchet one level down (62).
    /// A return past the lane's carrier is refused and counted on the
    /// ledger (STD1, rule 11), never pinned at a ceiling.
    pub fn restore(&mut self, cost: [u64; 4]) {
        for (l, c) in self.lanes.iter_mut().zip(cost.iter()) {
            crate::physics::metabolism::step_or_refuse(l, *c);
        }
        crate::physics::metabolism::note_restore(&cost);
        Self::book(&mut self.restored, &cost);
    }

    /// DRAIN: take what each lane holds of `cost`, up to what it holds — the
    /// spend of work already done, where the work was not gated on the draw
    /// (an expansion's charge, a tick). What was taken passes the ledger as
    /// drawn; what a lane could not give is returned, per lane, and is the
    /// shortfall the caller reads. Never a draw past empty (rule 11).
    pub fn drain(&mut self, cost: [u64; 4]) -> [u64; 4] {
        let taken: [u64; 4] = std::array::from_fn(|l| cost[l].min(self.lanes[l]));
        for (l, t) in self.lanes.iter_mut().zip(&taken) {
            *l -= t;
        }
        crate::physics::metabolism::note_draw(&taken);
        Self::book(&mut self.drawn, &taken);
        std::array::from_fn(|l| cost[l] - taken[l])
    }
}
