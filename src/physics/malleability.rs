// 78.73 — MALLEABILITY: what a lane does under stress, read as exact
// rationals on every lane alike.
//
//! A medium under stress deforms ELASTICALLY (it returns: an orbit or tube
//! closes, `cech::orbit`), PLASTICALLY (it keeps the new shape: a generator
//! enters the language) or FRACTURES (it fails at yield: a draw refused).
//! Per lane `ℓ`:
//!
//! - `μ_ℓ = plastic / (plastic + fracture)` — malleability;
//! - `β_ℓ = fracture / (plastic + fracture)` — brittleness, `μ + β = 1`;
//! - `ε_ℓ = open / transport` — the elastic strain still standing.
//!
//! Each is a ratio of counts the lane's own books keep, so the lanes compare
//! with no common unit. `0/0` is no reading and is named — `unstressed`, no
//! stress met the lane; `unmoved`, nothing moved on it — never written as a
//! number.

use super::{metabolism, Depleted, Physics};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

/// `a / (a + b)` exact, or `None` at `0/0`.
fn share(a: u64, b: u64) -> Option<BigRational> {
    let whole = BigInt::from(a) + BigInt::from(b);
    (!whole.is_zero()).then(|| BigRational::new(a.into(), whole))
}

fn shown(r: Option<BigRational>, none: &str) -> String {
    match r {
        Some(r) => r.to_string(),
        None => none.to_string(),
    }
}

impl Physics {
    /// 78.73 (1) — A DRAW, AND ITS STAGNATION: the vector drawn all or
    /// nothing; where a lane cannot cover it, the flow it blocked — what the
    /// draw asked past what the lane held — presses on that lane. A refused
    /// draw counted on the ledger and on no pressure read the lane under the
    /// most stress as calm (b's 𝒢: refused 92, pressure 0).
    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a lane that covered its share blocked nothing, named at the arm"
    )]
    pub fn draw(&mut self, cost: [u64; 4]) -> Result<(), Depleted> {
        let held = self.reserve.lanes();
        let drawn = self.reserve.draw(cost);
        if drawn.is_err() {
            let blocked: [u64; 4] = std::array::from_fn(|l| match cost[l].checked_sub(held[l]) {
                Some(past) => past,
                None => 0,
            });
            self.bump(&blocked);
        }
        drawn
    }

    /// 78.73 — A GENERATOR ENTERED THE LANGUAGE on these lanes: the medium
    /// took the stress by changing shape. One event per lane it occupies.
    pub fn plastic(&mut self, lanes: [bool; 4]) {
        for (p, on) in self.plastic.iter_mut().zip(lanes) {
            if on {
                metabolism::step_or_refuse(p, 1);
            }
        }
    }

    /// 78.73 (4) — THE LANES' MALLEABILITY LINE: `μ`, `β` and `ε` per lane,
    /// each exact or named as no reading. Fracture is the reserve's own
    /// refusals per lane; the elastic terms are the mind's field's
    /// (`Field::elastic`).
    pub fn malleability_line(&self, elastic: &[(BigInt, BigInt); 4]) -> String {
        let (_, _, fracture) = self.reserve.ledger();
        let mu: Vec<String> = (0..4).map(|l| shown(share(self.plastic[l], fracture[l]), "unstressed")).collect();
        let beta: Vec<String> = (0..4).map(|l| shown(share(fracture[l], self.plastic[l]), "unstressed")).collect();
        let eps: Vec<String> = elastic
            .iter()
            .map(|(open, transport)| shown((!transport.is_zero()).then(|| BigRational::new(open.clone(), transport.clone())), "unmoved"))
            .collect();
        format!("μ [{}] β [{}] ε [{}]", mu.join(", "), beta.join(", "), eps.join(", "))
    }
}
