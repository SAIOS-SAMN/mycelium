// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE MANIFOLD (78.179 §1; MU-SENSING §3.1): the geometric manifold an
//! observation lifts into, one factor per phase, replacing legacy "envelopes".
//!
//! A phase's period is read from the content, never from the extent alone:
//! - **Circle:** a circle `S¹` of period `p` where `p` is the least proper
//!   divisor of the extent `e` such that content repeats `p` steps on round the
//!   extent. The far seam glues to the near; site `s` is turn `s/p` with winding `⌊s/p⌋`.
//! - **Interval:** otherwise an interval chart of extent `e`, with boundary.
//! - **The manifold is the product of its phases.** A manifold whose every
//!   phase is a circle is a torus `Tⁿ`. Where an axis is nested, it is a
//!   fibration: coarse base and fine fibre.

use super::base::Site;
use super::sample::{AxisReading, Sample};

/// ONE PHASE OF A MANIFOLD: either a circle `S¹` of period `p` read from
/// repeating content, or an interval chart of extent `e` with boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    /// A circle `S¹` of period `p` (a proper divisor of extent `e`). The far
    /// seam glues to the near; site `s` is the turn `s/p` with winding `⌊s/p⌋`.
    Circle { period: usize },
    /// An interval chart of extent `e`, with boundary.
    Interval { extent: usize },
}

impl Phase {
    /// The extent of this phase.
    pub fn extent(&self) -> usize {
        match *self {
            Phase::Circle { period } => period,
            Phase::Interval { extent } => extent,
        }
    }

    /// Is this phase a circle `S¹`?
    pub fn is_circle(&self) -> bool {
        matches!(self, Phase::Circle { .. })
    }

    /// Is this phase an interval chart with boundary?
    pub fn is_interval(&self) -> bool {
        matches!(self, Phase::Interval { .. })
    }
}

/// AN AXIS IN A MANIFOLD: a single phase, or a fibration where an axis is nested
/// (#176; MU-SENSING §3.6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Axis {
    /// A single phase on this axis.
    One(Phase),
    /// A nested axis as a fibration: the coarse phase is the base, the fine
    /// phase is the fibre.
    Fibred {
        base: Phase,
        fibre: Phase,
    },
}

impl Axis {
    /// Is this axis a fibration of levels?
    pub fn is_fibred(&self) -> bool {
        matches!(self, Axis::Fibred { .. })
    }

    /// Is every phase on this axis a circle?
    pub fn is_all_circles(&self) -> bool {
        match self {
            Axis::One(p) => p.is_circle(),
            Axis::Fibred { base, fibre } => base.is_circle() && fibre.is_circle(),
        }
    }

    /// The combined extent (measure) along this axis, or `None` on overflow.
    pub fn measure(&self) -> Option<usize> {
        match self {
            Axis::One(p) => Some(p.extent()),
            Axis::Fibred { base, fibre } => base.extent().checked_mul(fibre.extent()),
        }
    }
}

/// THE MANIFOLD (78.179 §1): the open-dimensional manifold the mind navigates
/// and contemplates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifold {
    axes: Vec<Axis>,
}

impl Manifold {
    /// A manifold with the given axes.
    pub fn new(axes: Vec<Axis>) -> Manifold {
        Manifold { axes }
    }

    /// The count of axes (the manifold's rank / dimension).
    pub fn rank(&self) -> usize {
        self.axes.len()
    }

    /// The axes of the manifold in canonical address order.
    pub fn axes(&self) -> &[Axis] {
        &self.axes
    }

    /// Axis `axis`, or None if past the rank.
    pub fn axis(&self, axis: usize) -> Option<&Axis> {
        self.axes.get(axis)
    }

    /// A fully periodic torus `Tⁿ` with the given periods (78.179 §1).
    pub fn torus(periods: &[usize]) -> Manifold {
        let axes = periods
            .iter()
            .map(|&p| Axis::One(Phase::Circle { period: p }))
            .collect();
        Manifold { axes }
    }

    /// Is the manifold a torus `Tⁿ` (every phase is a circle `S¹`)?
    pub fn is_torus(&self) -> bool {
        !self.axes.is_empty() && self.axes.iter().all(|a| a.is_all_circles())
    }

    /// The manifold's measure: the product of extents across its axes.
    pub fn measure(&self) -> Option<usize> {
        self.axes.iter().try_fold(1usize, |acc, a| acc.checked_mul(a.measure()?))
    }

    /// LIFTS AN OBSERVATION INTO THE MANIFOLD (78.179 §1): one factor per
    /// phase.
    ///
    /// For each axis, tests proper divisors `p` of the extent `e` in ascending
    /// order. A phase is a circle `S¹` of period `p` where every site's symbol
    /// equals the symbol `p` steps on round the extent; otherwise an interval.
    pub fn lift_from(sample: &Sample) -> Manifold {
        let rank = sample.rank();
        let mut axes = Vec::with_capacity(rank);
        for axis in 0..rank {
            let extent = match sample.extent(axis) {
                AxisReading::Read(e) => e,
                AxisReading::Unread => 0,
            };
            let phase = detect_phase(sample, axis, extent);
            axes.push(Axis::One(phase));
        }
        Manifold { axes }
    }
}

/// Detects whether an axis is a circle of proper period `p`, or an interval of
/// extent `e`.
fn detect_phase(sample: &Sample, axis: usize, extent: usize) -> Phase {
    if extent <= 1 {
        return Phase::Interval { extent };
    }
    for p in 1..extent {
        if extent % p != 0 {
            continue;
        }
        if is_periodic_along_axis(sample, axis, extent, p) {
            return Phase::Circle { period: p };
        }
    }
    Phase::Interval { extent }
}

/// Checks if every site in the sample repeats with period `p` along `axis`.
fn is_periodic_along_axis(sample: &Sample, axis: usize, extent: usize, p: usize) -> bool {
    let check = || -> Option<bool> {
        for site in sample.sites() {
            let sym = sample.symbol_at(&site).ok()?;
            let mut target_axes = site.axes().to_vec();
            if let Some(coord) = target_axes.get_mut(axis) {
                let current = usize::try_from(*coord).ok()?;
                let next = current.checked_add(p)? % extent;
                *coord = i64::try_from(next).ok()?;
            }
            let target_site = Site::new(target_axes)?;
            let target_sym = sample.symbol_at(&target_site).ok()?;
            if sym != target_sym {
                return Some(false);
            }
        }
        Some(true)
    };
    match check() {
        Some(v) => v,
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::sample::Place;

    fn sample_1d(symbols: &[u8]) -> Sample {
        Sample::new(vec![symbols.len()], 1, symbols.to_vec(), Place::new(vec![]))
            .expect("valid 1d sample")
    }

    fn sample_2d(rows: usize, cols: usize, symbols: &[u8]) -> Sample {
        Sample::new(vec![rows, cols], 1, symbols.to_vec(), Place::new(vec![]))
            .expect("valid 2d sample")
    }

    #[test]
    fn periodic_content_becomes_circle() {
        let s = sample_1d(&[1, 2, 1, 2]);
        let m = Manifold::lift_from(&s);
        assert_eq!(m.rank(), 1);
        assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 2 }));
        assert!(m.is_torus(), "a 1D circle is a 1-torus T¹");
    }

    #[test]
    fn constant_content_becomes_circle_period_one() {
        let s = sample_1d(&[5, 5, 5]);
        let m = Manifold::lift_from(&s);
        assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 1 }));
    }

    #[test]
    fn non_repeating_content_becomes_interval() {
        let s = sample_1d(&[1, 2, 3, 4]);
        let m = Manifold::lift_from(&s);
        assert_eq!(m.axes()[0], Axis::One(Phase::Interval { extent: 4 }));
        assert!(!m.is_torus());
    }

    #[test]
    fn fully_periodic_2d_sample_is_torus_t2() {
        let s = sample_2d(2, 4, &[7, 8, 7, 8, 7, 8, 7, 8]);
        let m = Manifold::lift_from(&s);
        assert_eq!(m.rank(), 2);
        assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 1 }));
        assert_eq!(m.axes()[1], Axis::One(Phase::Circle { period: 2 }));
        assert!(m.is_torus(), "both phases circles => torus T²");
    }

    #[test]
    fn torus_constructor_creates_all_circle_phases() {
        let t = Manifold::torus(&[3, 5, 7]);
        assert_eq!(t.rank(), 3);
        assert!(t.is_torus(), "T³");
        assert_eq!(t.measure(), Some(105));
    }

    #[test]
    fn sample_lift_produces_manifold() {
        let s = sample_1d(&[1, 2, 1, 2]);
        let m = s.lift();
        assert_eq!(m.rank(), 1);
        assert_eq!(m.axes()[0], Axis::One(Phase::Circle { period: 2 }));
    }
}
