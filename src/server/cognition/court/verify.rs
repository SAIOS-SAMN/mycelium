// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// POWV — THE VERIFYING COURTS PAID OUT OF THE RELEASE THEY VERIFIED
// (MU-ECONOMY §5.1, `economy::verify`).
//
//! Every POUW release passes through here: the claim's base is what the
//! closer is owed — lane-wise the lesser of the escrow and the measured
//! discharge (78.149 (b)) — and the courts whose replay verified it take
//! their share of that base, per lane, read off this court's strain
//! against its reserve. The closer is minted the rest. Total issuance is
//! the base, exactly as before.

use crate::cognition::*;

impl Kernel {
    /// THE VERIFICATION OF A RELEASE PAID to `organs`, the courts whose
    /// replay verified it (this court, and the witness or peer that
    /// re-derived it where there is one): each is minted its share of
    /// `base` as POWV at `at`. Duplicate organs are one organ. Answers what
    /// the organs took together, per lane — zero where this court is at
    /// rest or its surface cannot be read.
    pub(crate) fn pay_verification(&mut self, base: [u64; 4], organs: &[[u8; 32]], at: crate::economy::Anchor) -> [u64; 4] {
        let mut distinct: Vec<[u8; 32]> = Vec::with_capacity(organs.len());
        for o in organs {
            if !distinct.contains(o) {
                distinct.push(*o);
            }
        }
        let Some(strain) = crate::economy::yield_surface(&self.physics) else { return [0; 4] };
        let each = crate::economy::verification_share(&base, &strain, crate::physics::reserve_max(), distinct.len());
        if each.iter().all(|&m| m == 0) {
            return [0; 4];
        }
        let mut taken = [0u64; 4];
        for organ in &distinct {
            let paid = self.treasury.settle_verification(*organ, each, at);
            for (t, p) in taken.iter_mut().zip(&paid) {
                // Each organ's share is at most `base / n` on its lane, so the
                // sum of n of them is at most `base`: it cannot pass the lane.
                *t += p;
            }
        }
        if taken.iter().any(|&m| m > 0) {
            self.acts.pay("verifications", taken);
        }
        taken
    }

    /// A CLAIM RELEASED WITH ITS VERIFIERS PAID: the base the closer is owed
    /// (the escrow capped lane-wise at `discharge`), the verifying courts'
    /// POWV taken from it, and the rest released to the claim's voices as
    /// POUW (`Treasury::release_discharged`). `None` where `addr` holds no
    /// escrow.
    pub(crate) fn release_verified(
        &mut self,
        addr: &[u8],
        discharge: [u64; 4],
        organs: &[[u8; 32]],
        at: crate::economy::Anchor,
    ) -> Option<[u64; 4]> {
        let held = self.treasury.escrow_of(addr)?.mu;
        let above = crate::economy::shortfall(&held, &discharge);
        let base: [u64; 4] = std::array::from_fn(|l| held[l] - above[l]);
        let taken = self.pay_verification(base, organs, at);
        let rest: [u64; 4] = std::array::from_fn(|l| base[l] - taken[l]);
        self.treasury.release_discharged(addr, rest, at)
    }
}
