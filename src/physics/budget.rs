// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Rule 10 — THE READ TOKEN: what bounds a loop that does not walk a
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
