// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TURN AND THE ENCLOSURE: a boundary on one phase of a frame, held by
//! its index and the period it was read in, and a motion composed of
//! turns, which keeps its winding (ℤ) with its phase; and the frame's `n`
//! phases.

use super::TransductionRefusal;
use crate::transducer::sample::{AxisReading, Sample};
use std::cmp::Ordering;

/// A TURN `index / period` on one phase. A boundary turn stands at
/// `0 ≤ index ≤ period`, both ends kept: `0` and `period` are the two sides
/// of the seam, held apart because nothing wraps. A composed motion
/// ([`Turn::plus`]) may stand past `period`: its winding, the whole turns
/// it holds, is kept with its phase and never reduced away. Two turns are
/// compared by cross-multiplying the indices and periods, never by reducing
/// either; `1/2` and `2/4` are one turn, and `1/1` is a whole turn, never
/// `0/1`.
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
    /// of period `h`, held as the pair, never reduced: its whole turns, if
    /// any, are kept.
    pub const fn of(r: usize, h: usize) -> Turn {
        Turn { index: r, period: h }
    }

    /// THE PHASE REVERSED: the turn `(h − r) / h` of the same period, the
    /// reflection that keeps the seam (`0 ↔ h`), for a boundary turn
    /// (`0 ≤ r ≤ h`).
    pub const fn reversed(&self) -> Turn {
        Turn { index: self.period - self.index, period: self.period }
    }

    /// THE COMPOSED MOTION (MU-SENSING §3.1; 78.171 amendment (c)):
    /// `r/h + r′/h′`, held on the least common multiple of the two periods,
    /// the period of the composed orbit (MU-ANTI-FLATTENING 12: an `lcm` of
    /// the periods of turns, never of two frames' extents). THE WINDING IS
    /// KEPT: the index is never reduced below its period, so a composed
    /// motion holds its whole turns (ℤ, [`Turn::winding`]) with its phase,
    /// and none is dropped. `None` where a period is 0 (no circle to turn
    /// on) or the composed period or index passes the machine word.
    pub fn plus(&self, other: &Turn) -> Option<Turn> {
        let (p, q) = (self.period, other.period);
        if p == 0 || q == 0 {
            return None;
        }
        let period = (p / num_integer::gcd(p, q)).checked_mul(q)?;
        let a = (self.index as u128).checked_mul((period / p) as u128)?;
        let b = (other.index as u128).checked_mul((period / q) as u128)?;
        let index = usize::try_from(a.checked_add(b)?).ok()?;
        Some(Turn { index, period })
    }

    /// THE WHOLE TURNS A MOTION HOLDS: `K = ⌊|θ|/360⌋` for its angle
    /// `θ = 360·r/h` degrees, that is `⌊r/h⌋`, read exactly with no angle
    /// formed. 0 for a boundary turn below the seam. `None` for a period of
    /// 0, which holds no turn.
    pub fn winding(&self) -> Option<usize> {
        self.index.checked_div(self.period)
    }

    /// THE PHASE ALONE: the turn less its whole turns, `(r mod h)/h`, on
    /// the same period. A place's residue reads it (a place on a phase has
    /// no winding of its own); a motion keeps its winding beside it.
    /// `None` for a period of 0.
    pub fn phase(&self) -> Option<Turn> {
        Some(Turn { index: self.index.checked_rem(self.period)?, period: self.period })
    }

    /// THE RESIDUE OF A PLACE, `self − other` in ℚ/ℤ: the phase of the sum
    /// with `other`'s inverse, `(h − (r mod h))/h`. The residue of a claimed
    /// place against a reconstructed one (MU-HOSTING §5a.10): a place on a
    /// phase has no winding, so the residue is a phase, whole exactly where
    /// the two agree. `None` as [`Turn::plus`].
    pub fn less(&self, other: &Turn) -> Option<Turn> {
        let h = other.period;
        let r = other.index.checked_rem(h)?;
        self.plus(&Turn { index: h - r, period: h })?.phase()
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

    /// X1 (#160, ruled 2026-11-06) — THE TURN WRITTEN: `r` then `h`, each an
    /// unsigned count (u64 LE), so the turn `r/h` is the exact rational
    /// they give, never reduced; then its winding `K = ⌊r/h⌋` beside it
    /// (u64 LE). `None` for a period of 0, which holds no turn.
    pub fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        let k = u64::try_from(self.winding()?).ok()?;
        for x in [u64::try_from(self.index).ok()?, u64::try_from(self.period).ok()?, k] {
            out.extend_from_slice(&x.to_le_bytes());
        }
        Some(())
    }

    /// The turn read back, through [`Turn::of`] alone: `None` where the
    /// period is 0, a count passes the machine word, or the winding written
    /// beside it is not the turn's own.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Turn> {
        let mut word = || -> Option<u64> {
            let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(v)
        };
        let (r, h, k) = (word()?, word()?, word()?);
        let t = Turn::of(usize::try_from(r).ok()?, usize::try_from(h).ok()?);
        (u64::try_from(t.winding()?).ok()? == k).then_some(t)
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
