// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COUPLING v1 (proposal 78.175, F4; netstratum docs/122 §2 laws 3–4,
// §4) — THE LIGHT LEDGER, the coupling's conservation: light is received
// on a body's unlocked face, the rest drains, and every bell light made =
// received + drained.

use super::{held, CouplingRefusal};
use crate::passage::{pass_share, Frame};
use crate::Q;
use num_traits::Zero;

/// WHICH FACE A BODY STANDS ON, and so which face is unlocked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Face {
    /// The continuum: its unlocked face is `1 − δ`.
    Continuum,
    /// The vacuum, the faces exchanged (netstratum docs/117 §8.2): its
    /// unlocked face is `δ`.
    Vacuum,
}

/// THE UNLOCKED FACE of a body of hold `H` at reach `r` on `face`: the
/// carry or pinch share `δ = H/(r + H)` (`forms::passage::Frame::share`),
/// read `1 − δ` on the continuum (`forms::passage::pass_share`) and `δ` on
/// the vacuum. Refused `Negative` with a part below zero, and by the
/// passage's name where `δ` has no whole.
pub fn unlocked(face: Face, hold: &Q, reach: &Q) -> Result<Q, CouplingRefusal> {
    let delta = Frame { hold: held(hold)?.clone(), reach: held(reach)?.clone() }.share()?;
    match face {
        Face::Continuum => Ok(pass_share(&delta)?),
        Face::Vacuum => Ok(delta),
    }
}

/// LIGHT TAKEN BY ONE BODY: received on its unlocked face, the rest
/// drained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taken {
    pub received: Q,
    pub drained: Q,
}

/// THE LIGHT `light` REACHING a body of hold `H` at reach `r` on `face`:
/// `light·u` received, `light·(1 − u)` drained, `u` the unlocked face.
/// Refused `Negative` with light below zero, and as [`unlocked`] is.
pub fn take(light: &Q, face: Face, hold: &Q, reach: &Q) -> Result<Taken, CouplingRefusal> {
    let received = held(light)? * unlocked(face, hold, reach)?;
    Ok(Taken { drained: light - &received, received })
}

/// ONE BELL'S LIGHT LEDGER, balanced: made = received + drained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ledger {
    pub made: Q,
    pub received: Q,
    pub drained: Q,
}

/// THE LEDGER OF ONE BELL: admitted where `made = received + drained`
/// exactly; refused `Unbalanced` with the three and the residue
/// `made − received − drained` otherwise.
pub fn ledger(made: &Q, received: &Q, drained: &Q) -> Result<Ledger, CouplingRefusal> {
    let residue = made - received - drained;
    match residue.is_zero() {
        true => Ok(Ledger { made: made.clone(), received: received.clone(), drained: drained.clone() }),
        false => Err(CouplingRefusal::Unbalanced {
            made: Box::new(made.clone()),
            received: Box::new(received.clone()),
            drained: Box::new(drained.clone()),
            residue: Box::new(residue),
        }),
    }
}
