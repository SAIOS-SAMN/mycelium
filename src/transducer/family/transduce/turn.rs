// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TURN AND THE ENCLOSURE: a boundary on one phase of a frame, as an
//! element of `ℚ/ℤ` held by its index and the period it was read in; and
//! the frame's `n` phases.

use super::TransductionRefusal;
use crate::transducer::sample::{AxisReading, Sample};
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

/// THE ENCLOSURE OF A FRAME: the periods of its `n` phases, one circle of
/// turns per axis of its sample (MU-SENSING §3.1). They are the only
/// numbers the lift holds. The axes stand in canonical address order, which
/// orders them for writing alone: no axis is a role, and none is read
/// apart from the others (MU-SENSING §3.6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enclosure {
    periods: Vec<usize>,
}

impl Enclosure {
    /// AN ENCLOSURE OF THE GIVEN PERIODS, or `Empty` where there is no
    /// phase at all or a phase of period 0 (no site boundary but the seam).
    pub fn new(periods: Vec<usize>) -> Result<Enclosure, TransductionRefusal> {
        match periods.is_empty() || periods.contains(&0) {
            true => Err(TransductionRefusal::Empty),
            false => Ok(Enclosure { periods }),
        }
    }

    /// THE PERIODS A SAMPLE PRESENTS: one phase per axis, each the axis's
    /// extent, read through [`Sample::extent`] up to the rank. A sample
    /// presents at least one axis and no empty one (S0), so every sample
    /// has its enclosure.
    pub fn of(sample: &Sample) -> Enclosure {
        let periods = (0..sample.rank())
            .filter_map(|axis| match sample.extent(axis) {
                AxisReading::Read(e) => Some(e),
                AxisReading::Unread => None,
            })
            .collect();
        Enclosure { periods }
    }

    /// The count of phases: the sample's rank.
    pub fn rank(&self) -> usize {
        self.periods.len()
    }

    /// The periods, in canonical address order.
    pub fn periods(&self) -> &[usize] {
        &self.periods
    }

    /// The period of phase `axis`, or [`AxisReading::Unread`] past the
    /// rank: an unread phase has no period, never 0 (MU-SENSING §5).
    pub fn period(&self, axis: usize) -> AxisReading {
        match self.periods.get(axis) {
            Some(&p) => AxisReading::Read(p),
            None => AxisReading::Unread,
        }
    }

    /// THE FRAME'S MEASURE (78.170 O1, at rank `n`): the product of its
    /// periods, the count of its sites, never a product of two fixed
    /// extents. `None` past `usize`.
    pub fn measure(&self) -> Option<usize> {
        self.periods.iter().try_fold(1usize, |m, &p| m.checked_mul(p))
    }

    /// Is the re-period from `self` onto `to` the identity (each phase
    /// keeps its period)?
    pub fn is_same_periods(&self, to: &Enclosure) -> bool {
        self == to
    }

    /// THE RE-PERIOD OF EACH PHASE from `self` onto `to`, one exact ratio
    /// `p′/p` per axis in lowest terms, `(numerator, denominator)`. The
    /// turns do not change; only the enclosure does. `None` where the two
    /// ranks differ: a phase of one frame with none in the other has no
    /// ratio.
    pub fn reperiod(&self, to: &Enclosure) -> Option<Vec<(usize, usize)>> {
        if self.rank() != to.rank() {
            return None;
        }
        self.periods
            .iter()
            .zip(&to.periods)
            .map(|(&p, &q)| {
                let g = num_integer::gcd(q, p);
                Some((q.checked_div(g)?, p.checked_div(g)?))
            })
            .collect()
    }
}
