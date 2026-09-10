// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! Rule 10 — THE READ TOKEN: what bounds a loop that does not walk a
//! boundary, how one token divides into two, and how the unspent half
//! comes back.
//!
//! Split from `physics.rs` at the §1.1 600-line reading bound; the items
//! are moved, not rewritten (§1.6).

/// Linear token for one stroke's keyed reads. Non-Copy, non-Clone: a
/// function that wants more than a fiber must take a Budget in its
/// signature. Constructed by [`super::Physics::stroke`] and by
/// [`Budget::share`], which splits one token into two and never creates
/// reads.
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

    /// WITHDRAW A SHARE, so a sub-read is ATOMIC: it completes within `n`
    /// or it contributes nothing, and either way the caller's own reads
    /// are untouched by it. Exact or refused (rule 11) — a token with
    /// fewer than `n` reads left does not hand back a smaller share,
    /// because a smaller share would let the sub-read end early and be
    /// mistaken for one that ran to completion. A share of no reads
    /// completes nothing and is refused for the same reason.
    ///
    /// `n` must not be a number read off the SCENE. A share sized by what
    /// is being looked at is content deciding how much work content gets,
    /// which is the shape rule 4 refuses. Half of the caller's own
    /// remaining reads is not that — see [`Budget::halve`].
    pub fn share(&mut self, n: usize) -> Option<Budget> {
        if n == 0 || self.remaining < n {
            return None;
        }
        self.remaining -= n;
        Some(Budget { remaining: n })
    }

    /// HALF OF WHAT IS LEFT, as a share. This is how a stroke DIVIDES:
    /// the sub-read gets half of what the caller still has, so it scales
    /// with the stroke, and the caller keeps the other half whatever the
    /// sub-read does with its own.
    ///
    /// A CONSTANT SHARE IS NOT A DIVISION, and this replaces one that was.
    /// `cover_share()` was half of `spend_unit` — a fixed size, so the
    /// sub-read got the same reads however large the stroke. Measured over
    /// twelve tasks: seven covers came back unmeasured at the machine's
    /// own stroke, and the SAME seven came back unmeasured at a stroke of
    /// 1 048 576 reads, because both handed the cover `spend_unit()/2`.
    /// Every cover with work to do was starved identically at both, and no
    /// large-stroke run could tell a reading from a starvation.
    ///
    /// The half is of the BUDGET, which is the caller's own state, and not
    /// of anything read off the scene. A token with one read left halves
    /// to zero and refuses, because a share of no reads cannot complete
    /// anything and would return unmeasured for free.
    pub fn halve(&mut self) -> Option<Budget> {
        self.share(self.remaining / 2)
    }

    /// Return a share's UNSPENT reads. The share is consumed here, so a
    /// share cannot be refunded twice and cannot be used after refund —
    /// the linearity that makes [`Budget`] non-Clone holds across the
    /// split as well.
    pub fn refund(&mut self, unspent: Budget) {
        self.remaining += unspent.remaining;
    }
}

#[cfg(test)]
mod tests {
    use super::super::Physics;

    /// A share is withdrawn exactly or refused, its unspent reads come
    /// back, and no read is created or lost across the split.
    #[test]
    fn a_share_is_exact_and_conserves_reads() {
        let mut p = Physics::default();
        let mut b = p.stroke_n(10);
        assert!(b.share(11).is_none(), "a share larger than the token is refused, not shrunk");
        assert_eq!(b.remaining(), 10, "a refused share withdraws nothing");
        let mut s = b.share(4).expect("four of ten");
        assert_eq!((b.remaining(), s.remaining()), (6, 4), "the reads moved, none were made");
        assert!(s.spend() && s.spend());
        b.refund(s);
        assert_eq!(b.remaining(), 8, "two spent inside the share, eight left on the token");
        let exact = b.share(8).expect("the whole remainder is a legal share");
        assert_eq!(b.remaining(), 0, "a token may give all of itself away");
        b.refund(exact);
        assert_eq!(b.remaining(), 8, "and take it all back unspent");
        assert!(b.share(0).is_none(), "a share of no reads completes nothing and is refused");
    }

    /// A halved share SCALES WITH THE STROKE, which a constant share does
    /// not. `cover_share()` was a constant of the physics and starved the
    /// same seven covers at the machine's stroke and at 1 048 576 reads
    /// alike; halving gives the sub-read more exactly when the caller has
    /// more, and always leaves the caller half.
    #[test]
    fn a_halved_share_scales_with_the_stroke_and_leaves_half_behind() {
        let mut p = Physics::default();
        for n in [10usize, 1000, 1 << 20] {
            let mut b = p.stroke_n(n);
            let s = b.halve().expect("a token of two or more reads halves");
            assert_eq!(s.remaining(), n / 2, "the share is half of what was left");
            assert_eq!(b.remaining(), n - n / 2, "and the caller keeps the rest");
            b.refund(s);
            assert_eq!(b.remaining(), n, "nothing is made or lost by the split");
        }
        let mut one = p.stroke_n(1);
        assert!(one.halve().is_none(), "one read halves to none and refuses");
        assert_eq!(one.remaining(), 1, "a refused halving withdraws nothing");
    }
}
