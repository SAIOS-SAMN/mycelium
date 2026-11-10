// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE'S MANIFOLD (78.179 §1; MU-SENSING §3.1; ROADMAP track 26, #32 item 6):
//! the sample built as a manifold, one factor per phase, replacing the sample's
//! legacy "envelope".
//!
//! A phase's period is read from the content, never from the extent alone:
//! - **Circle:** a circle `S¹` of period `p` only where `p` is the least proper
//!   divisor of the extent `e` such that every site's symbol equals the symbol
//!   `p` steps on round the extent.
//! - **Interval:** otherwise an interval chart of extent `e`, with boundary.
//! - **Product:** the sample is the product of its phases. A sample whose every
//!   phase is a circle is a torus `Tⁿ`. Where an axis is nested (#176), it is a
//!   fibration.

use super::base::Site;
use super::sample::{AxisReading, Sample};

/// ONE PHASE OF A SAMPLE'S MANIFOLD (78.179 §1): either a circle `S¹` of
/// period `p` read from repeating content, or an interval chart of extent `e`
/// with boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseManifold {
    /// A circle `S¹` of period `p` (a proper divisor of extent `e`). The far
    /// seam glues to the near; site `s` is the turn `s/p` with winding `⌊s/p⌋`.
    Circle { period: usize },
    /// An interval chart of extent `e`, with boundary.
    Interval { extent: usize },
}

impl PhaseManifold {
    /// The extent of this phase.
    pub fn extent(&self) -> usize {
        match *self {
            PhaseManifold::Circle { period } => period,
            PhaseManifold::Interval { extent } => extent,
        }
    }

    /// Is this phase a circle `S¹`?
    pub fn is_circle(&self) -> bool {
        matches!(self, PhaseManifold::Circle { .. })
    }

    /// Is this phase an interval chart with boundary?
    pub fn is_interval(&self) -> bool {
        matches!(self, PhaseManifold::Interval { .. })
    }
}

/// AN AXIS IN A SAMPLE'S MANIFOLD: a single phase, or a fibration where an axis
/// is nested (#176; MU-SENSING §3.6).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AxisManifold {
    /// A single phase on this axis.
    One(PhaseManifold),
    /// A nested axis as a fibration: the coarse phase is the base, the fine
    /// phase is the fibre.
    Fibred {
        base: PhaseManifold,
        fibre: PhaseManifold,
    },
}

impl AxisManifold {
    /// Is this axis a fibration of levels?
    pub fn is_fibred(&self) -> bool {
        matches!(self, AxisManifold::Fibred { .. })
    }

    /// Is every phase on this axis a circle?
    pub fn is_all_circles(&self) -> bool {
        match self {
            AxisManifold::One(p) => p.is_circle(),
            AxisManifold::Fibred { base, fibre } => base.is_circle() && fibre.is_circle(),
        }
    }

    /// The combined extent (measure) along this axis, or `None` on overflow.
    pub fn measure(&self) -> Option<usize> {
        match self {
            AxisManifold::One(p) => Some(p.extent()),
            AxisManifold::Fibred { base, fibre } => base.extent().checked_mul(fibre.extent()),
        }
    }
}

/// THE SAMPLE'S MANIFOLD (78.179 §1; #32 item 6): the manifold the sample is
/// built as once lifted into the field.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleManifold {
    axes: Vec<AxisManifold>,
}

impl SampleManifold {
    /// A sample manifold with the given axis manifolds.
    pub fn new(axes: Vec<AxisManifold>) -> SampleManifold {
        SampleManifold { axes }
    }

    /// The count of axes (the sample's rank).
    pub fn rank(&self) -> usize {
        self.axes.len()
    }

    /// The axes of the manifold in canonical address order.
    pub fn axes(&self) -> &[AxisManifold] {
        &self.axes
    }

    /// Axis `axis`, or None if past the rank.
    pub fn axis(&self, axis: usize) -> Option<&AxisManifold> {
        self.axes.get(axis)
    }

    /// A fully periodic torus `Tⁿ` with the given periods (78.179 §1).
    pub fn torus(periods: &[usize]) -> SampleManifold {
        let axes = periods
            .iter()
            .map(|&p| AxisManifold::One(PhaseManifold::Circle { period: p }))
            .collect();
        SampleManifold { axes }
    }

    /// Is the manifold a torus `Tⁿ` (every phase is a circle `S¹`)?
    pub fn is_torus(&self) -> bool {
        !self.axes.is_empty() && self.axes.iter().all(|a| a.is_all_circles())
    }

    /// The manifold's measure: the product of extents across its axes.
    pub fn measure(&self) -> Option<usize> {
        self.axes.iter().try_fold(1usize, |acc, a| acc.checked_mul(a.measure()?))
    }

    /// READ A SAMPLE'S MANIFOLD FROM ITS CONTENT (78.179 §1): one factor per
    /// phase.
    ///
    /// For each axis, tests proper divisors `p` of the extent `e` in ascending
    /// order. A phase is a circle `S¹` of period `p` where every site's symbol
    /// equals the symbol `p` steps on round the extent; otherwise an interval.
    pub fn of(sample: &Sample) -> SampleManifold {
        let rank = sample.rank();
        let mut axes = Vec::with_capacity(rank);
        for axis in 0..rank {
            let extent = match sample.extent(axis) {
                AxisReading::Read(e) => e,
                AxisReading::Unread => 0,
            };
            let phase = detect_phase_manifold(sample, axis, extent);
            axes.push(AxisManifold::One(phase));
        }
        SampleManifold { axes }
    }
}

/// Detects whether an axis is a circle of proper period `p`, or an interval of
/// extent `e`.
fn detect_phase_manifold(sample: &Sample, axis: usize, extent: usize) -> PhaseManifold {
    if extent <= 1 {
        return PhaseManifold::Interval { extent };
    }
    // Proper divisors in ascending order: 1 <= p < extent
    for p in 1..extent {
        if extent % p != 0 {
            continue;
        }
        if is_periodic_along_axis(sample, axis, extent, p) {
            return PhaseManifold::Circle { period: p };
        }
    }
    PhaseManifold::Interval { extent }
}

/// Checks if every site in the sample repeats with period `p` along `axis`.
fn is_periodic_along_axis(sample: &Sample, axis: usize, extent: usize, p: usize) -> bool {
    let check = || -> Option<bool> {
        for site in sample.sites() {
            let sym = sample.symbol_at(&site).ok()?;
            // Target site shifted by p steps on round the extent
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
        // Repeated sequence [1, 2, 1, 2]: extent 4, period 2
        let s = sample_1d(&[1, 2, 1, 2]);
        let m = SampleManifold::of(&s);
        assert_eq!(m.rank(), 1);
        assert_eq!(
            m.axes()[0],
            AxisManifold::One(PhaseManifold::Circle { period: 2 })
        );
        assert!(m.is_torus(), "a 1D circle is a 1-torus T¹");
    }

    #[test]
    fn constant_content_becomes_circle_period_one() {
        // Constant sequence [5, 5, 5]: extent 3, period 1
        let s = sample_1d(&[5, 5, 5]);
        let m = SampleManifold::of(&s);
        assert_eq!(
            m.axes()[0],
            AxisManifold::One(PhaseManifold::Circle { period: 1 })
        );
    }

    #[test]
    fn non_repeating_content_becomes_interval() {
        // [1, 2, 3, 4] has no proper divisor period
        let s = sample_1d(&[1, 2, 3, 4]);
        let m = SampleManifold::of(&s);
        assert_eq!(
            m.axes()[0],
            AxisManifold::One(PhaseManifold::Interval { extent: 4 })
        );
        assert!(!m.is_torus());
    }

    #[test]
    fn fully_periodic_2d_sample_is_torus_t2() {
        // 2x4 grid:
        // row 0: 7, 8, 7, 8
        // row 1: 7, 8, 7, 8
        // Rows are identical (period 1 along axis 0), cols repeat with period 2 (along axis 1)
        let s = sample_2d(2, 4, &[7, 8, 7, 8, 7, 8, 7, 8]);
        let m = SampleManifold::of(&s);
        assert_eq!(m.rank(), 2);
        assert_eq!(
            m.axes()[0],
            AxisManifold::One(PhaseManifold::Circle { period: 1 })
        );
        assert_eq!(
            m.axes()[1],
            AxisManifold::One(PhaseManifold::Circle { period: 2 })
        );
        assert!(m.is_torus(), "both phases circles => torus T²");
    }

    #[test]
    fn torus_constructor_creates_all_circle_phases() {
        let t = SampleManifold::torus(&[3, 5, 7]);
        assert_eq!(t.rank(), 3);
        assert!(t.is_torus(), "T³");
        assert_eq!(t.measure(), Some(105));
    }
}
