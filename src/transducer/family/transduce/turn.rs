// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TURN AND THE ENCLOSURE: a boundary on one circle of a frame, as an
//! element of `ℚ/ℤ` held by its index and the period it was read in.

use super::TransductionRefusal;
use std::cmp::Ordering;

/// A BOUNDARY TURN `index / period` on one circle, `0 ≤ index ≤ period`.
/// Both ends are kept: `0` and `period` are the two sides of the seam, held
/// apart because nothing wraps. Two turns are compared by cross-multiplying
/// the indices and periods, never by reducing either; `1/2` and `2/4` are
/// one turn.
#[derive(Clone, Copy, Debug)]
pub struct Turn {
    pub index: usize,
    pub period: usize,
}

impl Turn {
    pub const fn new(index: usize, period: usize) -> Turn {
        Turn { index, period }
    }

    /// `index · period′` against `index′ · period`, in a carrier both
    /// products fit: two `usize` multiply inside `u128`.
    fn cross(&self, other: &Turn) -> (u128, u128) {
        (self.index as u128 * other.period as u128, other.index as u128 * self.period as u128)
    }

    /// Is this turn one of the seam's two sides (`0` or the whole turn)?
    pub fn is_seam(&self) -> bool {
        self.index == 0 || self.index == self.period
    }

    /// PROJECT (D3.2 §E): the turn read on a circle of period `period` is
    /// the boundary `index · period / self.period`, exactly when that is an
    /// integer; otherwise the turn falls inside a compartment and is refused
    /// `OffSite`, never rounded.
    pub fn lands(&self, period: usize) -> Result<usize, TransductionRefusal> {
        let n = self.index as u128 * period as u128;
        let d = self.period as u128;
        if d == 0 || n % d != 0 {
            return Err(TransductionRefusal::OffSite { turn: *self, period });
        }
        usize::try_from(n / d).map_err(|_| TransductionRefusal::OffSite { turn: *self, period })
    }

    /// The turn's exact value as a rational, for the share of a frame
    /// (O1). Never an operand of identity.
    pub fn value(&self) -> crate::core::Q {
        crate::core::Q::new(self.index.into(), self.period.into())
    }
}

impl PartialEq for Turn {
    fn eq(&self, other: &Turn) -> bool {
        let (a, b) = self.cross(other);
        a == b
    }
}

impl Eq for Turn {}

impl PartialOrd for Turn {
    fn partial_cmp(&self, other: &Turn) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Turn {
    fn cmp(&self, other: &Turn) -> Ordering {
        let (a, b) = self.cross(other);
        a.cmp(&b)
    }
}

/// THE ENCLOSURE OF A FRAME: the periods of its two circles, rows then
/// columns. They are the only numbers the lift holds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Enclosure {
    pub rows: usize,
    pub cols: usize,
}

impl Enclosure {
    /// Is the re-period from `self` onto `to` the identity (each circle
    /// keeps its period)?
    pub fn is_same_periods(&self, to: &Enclosure) -> bool {
        self == to
    }
}
