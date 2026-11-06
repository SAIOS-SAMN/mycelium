// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COUPLING v1 (proposal 78.175, F1 and F2; netstratum docs/121 §9.1,
// §10.1) — THE DIPOLE PARCEL AND ITS VISCOSITY: a parcel's frames, its pole
// and its open share, one law, each a ratio of the frames.

use super::{share_of, CouplingRefusal};
use crate::Q;
use num_traits::{One, Signed, Zero};

/// A PARCEL'S FRAMES: `A` held (closed) and `Q` open, each not below zero
/// and not both zero. The pole stands on these frames, never on the
/// electrons' phase `R`, which is the parcel's phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parcel {
    held: Q,
    open: Q,
}

/// A DIPOLE: its pole `σ ∈ [−1, 1]`, closed against open, and its strength,
/// the open share `o ∈ [0, 1]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dipole {
    pub pole: Q,
    pub open: Q,
}

impl Parcel {
    /// The parcel of `held` (`A`) and `open` (`Q`) frames. Refused
    /// `NotFrames` where one is below zero or both are zero.
    pub fn of(held: Q, open: Q) -> Result<Parcel, CouplingRefusal> {
        match held.is_negative() || open.is_negative() || (held.is_zero() && open.is_zero()) {
            true => Err(CouplingRefusal::NotFrames { held: Box::new(held), open: Box::new(open) }),
            false => Ok(Parcel { held, open }),
        }
    }

    /// THE PARCEL OF A KIND: `Z` and `U`, held `A = W = 3Z − U` and open
    /// `Q = U`. An open-heavy kind (`σ < 0`) needs `U > 3Z/2`. Refused
    /// `NotFrames` where `U > 3Z` or `U < 0`.
    pub fn kind(z: &Q, u: &Q) -> Result<Parcel, CouplingRefusal> {
        Parcel::of(Q::from_integer(3.into()) * z - u, u.clone())
    }

    /// The held frames `A`.
    pub fn held(&self) -> &Q {
        &self.held
    }

    /// The open frames `Q`.
    pub fn open(&self) -> &Q {
        &self.open
    }

    /// The ratio `t = Q/A`; none at the open pole, `A = 0`.
    pub fn ratio(&self) -> Option<Q> {
        (!self.held.is_zero()).then(|| &self.open / &self.held)
    }

    /// THE OPEN SHARE `o = Q/(A + Q) = ν(t)`, the parcel's strength (its
    /// viscosity's share).
    pub fn open_share(&self) -> Result<Q, CouplingRefusal> {
        share_of(&self.open, &self.held)
    }

    /// THE DIPOLE THE FRAMES HOLD: the pole `σ = (A − Q)/(A + Q)`, closed
    /// against open, continuous in `[−1, 1]` (a ratio, which no metric
    /// enters), and the open share `o = Q/(A + Q)`.
    pub fn dipole(&self) -> Result<Dipole, CouplingRefusal> {
        let open = self.open_share()?;
        // (A − Q)/(A + Q) = 1 − 2·Q/(A + Q): the held share less the open.
        let pole = Q::one() - Q::from_integer(2.into()) * &open;
        Ok(Dipole { pole, open })
    }

    /// THE INVERSION: the frames exchanged, `t ↦ 1/t`, so `o ↦ 1 − o` and
    /// `σ ↦ −σ`.
    pub fn inverted(&self) -> Parcel {
        Parcel { held: self.open.clone(), open: self.held.clone() }
    }
}

impl Dipole {
    /// A dipole stated by its pole and open share, as a kernel carries one.
    /// Refused `NotDipole` with the pole outside `[−1, 1]` or the open share
    /// outside `[0, 1]`.
    pub fn stated(pole: Q, open: Q) -> Result<Dipole, CouplingRefusal> {
        match pole.abs() > Q::one() || open.is_negative() || open > Q::one() {
            true => Err(CouplingRefusal::NotDipole { pole: Box::new(pole), open: Box::new(open) }),
            false => Ok(Dipole { pole, open }),
        }
    }
}
