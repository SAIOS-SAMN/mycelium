// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.184 §5 (ruled 2026-11-14), MU-EQUATIONS §6.13 — DELIVERY PER FINAL
// POINT: a grant's allotment delivered along TUniversal's temporal plane.
//
//! The network never gives a universe a rate (MU-CHRONOMETRY §2), so the
//! beat is read off the network's own clock. The T lane is the temporal
//! plane, cumulative verified null flux. Read with the faces law, with `T₀`
//! the T lane at the grant's first final point with `T > 0`:
//!
//! ```text
//! d_T(h) = 1 − T₀ / T(h)              the share delivered by final point h
//! beat   = d_T(h) − d_T(h_prev)       minted at each final-point advance
//! ```
//!
//! So delivery never exceeds the allotment, half arrives when the network
//! has travelled as far again as at the founding (`T(h) = 2T₀`), and a quiet
//! network delivers nothing.

use super::Court;
use crate::chronometry::tuniversal::TPoint;
use crate::core::q::Q;
use crate::economy::Mu;
use num_traits::{One, Zero};

/// The T lane's index in a point's lanes (S, G, T, H).
const T_LANE: usize = 2;

impl Court {
    /// THE FINAL POINT `fin` ADVANCED: every grant still delivering takes its
    /// beat. A grant with no `T₀` takes the first `T > 0` it meets as its
    /// `T₀` and delivers nothing that bell. Each beat is minted to the
    /// domain's sitting reserve, standing at the grant's own act. Answers
    /// the deliveries made, `(domain, μ)`.
    pub fn deliver_at(&mut self, fin: &TPoint) -> Vec<([u8; 32], Mu)> {
        let t = fin.lanes.lane(T_LANE);
        let mut out = Vec::new();
        if t <= Q::zero() {
            return out;
        }
        let domains: Vec<[u8; 32]> = self.grants.scoped().map(|(k, _)| k).filter_map(|k| <[u8; 32]>::try_from(k.as_slice()).ok()).collect();
        for domain in domains {
            let Some(reserve) = self.reserve_of(&domain) else { continue };
            let Some(mut held) = self.grants.get(&domain).cloned() else { continue };
            let mut moved = false;
            for g in held.iter_mut() {
                if g.allotment.is_zero() || g.delivered >= Q::one() {
                    continue;
                }
                let Some(t0) = g.t0.clone() else {
                    g.t0 = Some(t.clone());
                    moved = true;
                    continue;
                };
                let d = Q::one() - &t0 / &t;
                if d <= g.delivered {
                    continue;
                }
                let beat = &d - &g.delivered;
                let Some(due) = g.allotment.scaled(&beat) else { continue };
                let minted = self.treasury.mint_to(reserve, due, (g.at, g.receipt), None);
                g.mu = g.mu.add(&minted);
                g.delivered = d;
                moved = true;
                out.push((domain, minted));
            }
            if moved {
                self.grants.insert(domain, held);
                self.mark_persist();
            }
        }
        out
    }
}
