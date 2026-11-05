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
///
/// THE GUARD OF MU-ANTI-FLATTENING 12 (`turn_has_one_constructor`): a
/// site's lift has one constructor, [`Turn::of`], and the index and the
/// period are private, so no reader takes a turn apart into a grid index.
/// A turn is read back on a period only by landing it there
/// ([`Turn::lands`]), exact or refused `OffSite`. Neither a turn written
/// by its fields nor a field read compiles:
///
/// ```compile_fail,E0451
/// use mycelium::transducer::family::transduce::Turn;
/// let _ = Turn { index: 1, period: 2 };
/// ```
///
/// ```compile_fail,E0616
/// use mycelium::transducer::family::transduce::Turn;
/// let _ = Turn::of(1, 2).index;
/// ```
///
/// The constructor, which compiles:
///
/// ```
/// use mycelium::transducer::family::transduce::Turn;
/// assert_eq!(Turn::of(1, 2), Turn::of(2, 4));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Turn {
    index: usize,
    period: usize,
}

impl Turn {
    /// THE TURN `r / h` (MU-ANTI-FLATTENING 12): index `r` read on a phase
    /// of period `h`, an element of ℚ/ℤ held as the pair, never reduced.
    pub const fn of(r: usize, h: usize) -> Turn {
        Turn { index: r, period: h }
    }

    /// THE PHASE REVERSED: the turn `(h − r) / h` of the same period, the
    /// reflection that keeps the seam (`0 ↔ h`), for a boundary turn
    /// (`0 ≤ r ≤ h`).
    pub const fn reversed(&self) -> Turn {
        Turn { index: self.period - self.index, period: self.period }
    }

    /// THE SUM IN ℚ/ℤ (MU-SENSING §3.1): `r/h + r′/h′` mod 1, held on the
    /// least common multiple of the two periods, the period of the composed
    /// orbit (MU-ANTI-FLATTENING 12: an `lcm` of the periods of turns, never
    /// of two frames' extents). As a motion the seam's two sides are one
    /// point of ℚ/ℤ, so the sum's index lies below its period. `None` where
    /// a period is 0 (no circle to turn on) or the composed period passes
    /// the machine word.
    pub fn plus(&self, other: &Turn) -> Option<Turn> {
        let (p, q) = (self.period, other.period);
        if p == 0 || q == 0 {
            return None;
        }
        let period = (p / num_integer::gcd(p, q)).checked_mul(q)?;
        let a = (self.index as u128).checked_mul((period / p) as u128)?;
        let b = (other.index as u128).checked_mul((period / q) as u128)?;
        let index = usize::try_from(a.checked_add(b)? % period as u128).ok()?;
        Some(Turn { index, period })
    }

    /// THE DIFFERENCE IN ℚ/ℤ, `self − other` mod 1: the sum with `other`'s
    /// inverse, `(h − (r mod h))/h`. The residue of a claimed turn against
    /// a reconstructed one (MU-HOSTING §5a.10). `None` as [`Turn::plus`].
    pub fn less(&self, other: &Turn) -> Option<Turn> {
        let h = other.period;
        let r = other.index.checked_rem(h)?;
        self.plus(&Turn { index: h - r, period: h })
    }

    /// Is the turn whole, the zero of ℚ/ℤ (`h` divides `r`)? A period of
    /// 0 holds no turn, so none of it is whole.
    pub fn is_whole(&self) -> bool {
        self.index.checked_rem(self.period) == Some(0)
    }

    /// `index · period′` against `index′ · period`, in a carrier both
    /// products fit: two `usize` multiply inside `u128`.
    fn cross(&self, other: &Turn) -> (u128, u128) {
        (self.index as u128 * other.period as u128, other.index as u128 * self.period as u128)
    }

    /// Is this turn one of the seam's two sides (`0` or the whole turn)?
    #[cfg(test)]
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
