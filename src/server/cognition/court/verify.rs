// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// POWV — THE VERIFYING COURTS PAID OUT OF THE RELEASE THEY VERIFIED
// (MU-ECONOMY §5.1, `economy::verify`).
//
//! Every POUW release passes through here: the claim's base is what the
//! closer is owed — lane-wise the lesser of the escrow and the measured
//! discharge (78.149 (b)) — and the courts whose reconstruction verified it each take
//! a share of that base, per lane, read off this court's strain
//! against its reserve. The closer is minted the rest. Total issuance is
//! the base, exactly as before.

use crate::cognition::*;
use crate::economy::{Modes, Mu};

impl Court {
    /// THE VERIFICATION OF A RELEASE PAID to `organs`, the courts whose
    /// reconstruction verified it (this court, and the witness or peer that
    /// re-derived it where there is one): each is minted its share of
    /// `base`'s modes as POWV at `at`, the lane's factor scaling each mode
    /// on it (§5.1). Duplicate organs are one organ. Answers the modes the
    /// organs took together — none where this court is at rest or its
    /// surface cannot be read.
    pub(crate) fn pay_verification(&mut self, base: &Modes, organs: &[[u8; 32]], at: crate::economy::Anchor) -> Modes {
        let mut distinct: Vec<[u8; 32]> = Vec::with_capacity(organs.len());
        for o in organs {
            if !distinct.contains(o) {
                distinct.push(*o);
            }
        }
        let Some(strain) = crate::economy::yield_surface(&self.physics) else { return Modes::new() };
        let each = crate::economy::verification_modes(base, &strain, self.physics.capacity(), distinct.len());
        if each.is_empty() {
            return each;
        }
        // Each organ's share is at most `base / n` on its lane, so the sum
        // of n of them is at most `base`.
        let mut taken = Modes::new();
        for organ in &distinct {
            let _ = self.treasury.settle_verification(*organ, each.clone(), at);
            taken.add(&each);
        }
        if !taken.is_empty() {
            self.acts.pay("verifications", taken.reading());
        }
        taken
    }

    /// A CLAIM RELEASED WITH ITS VERIFIERS PAID: the base the closer is owed
    /// (the re-traversed closure's modes, `discharge`, capped lane-wise by
    /// the escrow), the verifying courts' POWV taken from it, and the rest
    /// released to the claim's voices as POUW (`Treasury::release_discharged`).
    /// `None` where `address` holds no escrow.
    pub(crate) fn release_verified(
        &mut self,
        address: &[u8],
        discharge: &Modes,
        organs: &[[u8; 32]],
        at: crate::economy::Anchor,
    ) -> Option<Mu> {
        let base = discharge.capped(&self.treasury.escrow_of(address)?.mu);
        let taken = self.pay_verification(&base, organs, at);
        // taken ≤ base mode by mode: the organs' shares sum to at most it.
        let mut rest = base;
        let _ = rest.take(&taken);
        self.treasury.release_discharged(address, &rest, at)
    }
}
