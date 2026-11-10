// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! K3 (78.177 §4a(1), ruled 2026-11-06), MU-CLIENT §5.5 — A VERIFIED
//! CONVERGENCE IS PAID A POWC SHARE OUT OF THE INSTALL'S MINT, NEVER ADDED
//! TO IT. Two minds' different words, one section on one scene (K1–K2):
//! the minds whose words converged with the installed one are paid out of
//! its COMPOSITION PREMIUM, what is left after the base and use shares are
//! taken out (`settle_recursive`). Each is paid its share of the frame,
//! the share its own word's section takes of each pair's frame (O1,
//! `family::local::word_local`); where the shares together pass what the
//! premium holds on a lane, they split that lane in proportion to them,
//! exactly. Where no premium is left, each convergence is held as a claim
//! at its share and never minted.
//!
//! It is not a 47.3 second vantage: consilience (courts re-deriving) and
//! convergence (minds meeting) stay named apart (37.7).

use super::*;
use crate::core::q::Q;
use num_traits::Zero;

/// THE CLAIM A CONVERGENCE IS HELD AT where no premium is left: the
/// installed word's address, then the converging mind, written out.
pub fn convergence_claim(word: &[u8], mind: &[u8; 32]) -> crate::transducer::address::Address {
    let mut k = b"converged".to_vec();
    crate::transducer::address::put_key_bytes(&mut k, word);
    k.extend_from_slice(mind);
    crate::transducer::address::Address::from_bytes(&k)
}

/// EACH MIND'S PART OF `premium`, lane by lane: its share where the shares
/// together fit the lane, else the lane split in proportion to the shares.
/// The parts never sum past the premium on any lane.
pub fn converged_parts(premium: &Mu, converged: &[([u8; 32], Mu)]) -> Vec<([u8; 32], Mu)> {
    let total = Mu::sum(converged.iter().map(|(_, s)| s));
    converged
        .iter()
        .map(|(mind, share)| {
            let lanes: [Q; 4] = std::array::from_fn(|l| {
                let (s, t, p) = (share.lane(l), total.lane(l), premium.lane(l));
                match t > p {
                    // The shares pass the lane: each takes its proportion.
                    true => p * s / t,
                    false => s.clone(),
                }
            });
            // Every factor is a non-negative lane, so every lane is, and
            // no lane is refused (as `forfeit::forfeit` reads its lanes).
            let part = match Mu::of(lanes) {
                Ok(m) => m,
                Err(_) => Mu::zero(),
            };
            (*mind, part)
        })
        .collect()
}

impl Treasury {
    /// K3 — THE CONVERGING MINDS PAID OUT OF `premium`, the composition
    /// premium of the install at `escrow`: each minted its part as POWC.
    /// Where the premium is empty, each convergence is held as a claim at
    /// its share (`convergence_claim`), never minted. Answers what the minds
    /// were minted together, at most `premium` on every lane.
    pub fn pay_converging(&mut self, escrow: &[u8], premium: &Mu, converged: &[([u8; 32], Mu)], anchor: Anchor) -> Mu {
        let mut paid = Mu::zero();
        if premium.is_zero() {
            for (mind, share) in converged.iter().filter(|(_, s)| !s.is_zero()) {
                self.escrow_award(&convergence_claim(escrow, mind), vec![*mind], share.clone(), anchor);
            }
            return paid;
        }
        for (mind, part) in converged_parts(premium, converged) {
            if part.lanes().iter().all(Zero::is_zero) {
                continue;
            }
            let minted = self.mint_to(mind, part, anchor, Some(Arm::Powc));
            paid.step(&minted);
        }
        paid
    }
}
