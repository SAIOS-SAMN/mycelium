// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE AS A MANIFOLD (ruled 2026-11-06 on #32, the envelope; MU-SENSING
//! §3.1, §3.6, §3.7). The sensor lifts each axis of a sample into a phase of
//! its own period (the envelope, `Enclosure`); this module reads what that
//! envelope is, as a manifold:
//!
//! - **A phase with a period is a circle** ([`PhaseManifold::Circle`]): the sample's
//!   symbols repeat along the axis with a period `p` that divides the
//!   extent, so the far seam glues onto the near one and the phase closes;
//!   `p`, the least such, is the circle's circumference.
//! - **A phase without one is an interval chart with boundary**
//!   ([`PhaseManifold::Interval`]): its seam is held, never wrapped (§3.1).
//! - **The sample is their product** ([`SampleManifold`]), or a fibration
//!   where an axis is taken apart (#176): the coarse phase the base and the
//!   fine phase the fibre, each read circle or interval by the same reading
//!   on the blocks ([`AxisManifold::Fibred`]).
//! - **A fully periodic sample is a torus** `Tⁿ` ([`SampleManifold::torus`]),
//!   its periods the circles' circumferences.
//!
//! The manifold is a reading of the sample, computed when asked and never
//! stored; no period is fixed in the code, and no threshold decides a
//! circle: a period holds on every site or it is no period. The torus is the
//! reading of the sample's own phases, never a reference surface the sample
//! is measured against (§3.7), and the membrane stays the only sphere. The
//! perspectives' cover is the atlas of this manifold, and its cocycle the
//! gluing (`perspective::cover`).

use super::Factoring;
use crate::transducer::base::Site;
use crate::transducer::sample::{AxisReading, Sample};

/// ONE PHASE READ AS A MANIFOLD OF DIMENSION ONE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseManifold {
    /// A circle `S¹` of circumference `period`: the symbols repeat with it
    /// along the phase and the far seam glues onto the near one.
    Circle { period: usize },
    /// An interval chart of `extent` sites with its two seams as boundary.
    Interval { extent: usize },
}

/// ONE AXIS OF THE SAMPLE AS A FACTOR OF ITS MANIFOLD.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisManifold {
    /// The axis is one phase.
    One(PhaseManifold),
    /// The axis is taken apart (MU-SENSING §3.6): a fibration whose base is
    /// the coarse phase over the blocks and whose fibre is the fine phase
    /// within a block.
    Fibred { base: PhaseManifold, fibre: PhaseManifold },
}

/// A SAMPLE'S MANIFOLD: one factor per axis, in canonical axis order. A
/// reading of the sample's envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SampleManifold {
    pub axes: Vec<AxisManifold>,
}

impl SampleManifold {
    /// THE PRODUCT OF THE SAMPLE'S PHASES: each axis a circle where the
    /// sample's symbols repeat along it with a period dividing its extent,
    /// else an interval.
    pub fn of(sample: &Sample) -> SampleManifold {
        let axes = (0..sample.rank()).map(|axis| AxisManifold::One(phase_of(sample, axis, extent(sample, axis), Some))).collect();
        SampleManifold { axes }
    }

    /// THE MANIFOLD WITH AXES TAKEN APART: an axis named with the factoring
    /// it is taken apart by (`nested`, one per axis or none) is a fibration
    /// of its coarse phase over its fine phase; the rest are as
    /// [`SampleManifold::of`] reads them. A factoring whose extent is not
    /// the axis's own is no factoring of it, and the axis stays one phase.
    pub fn nested(sample: &Sample, nested: &[Option<Factoring>]) -> SampleManifold {
        let axes = (0..sample.rank())
            .map(|axis| {
                let e = extent(sample, axis);
                match nested.get(axis).copied().flatten().filter(|f| f.extent() == e) {
                    Some(f) => AxisManifold::Fibred { base: phase_of(sample, axis, f.coarse(), |i| f.site(i, 0)), fibre: fibre_of(sample, axis, &f) },
                    None => AxisManifold::One(phase_of(sample, axis, e, Some)),
                }
            })
            .collect();
        SampleManifold { axes }
    }

    /// THE TORUS `Tⁿ` THE SAMPLE IS, where every phase is a circle: the
    /// circles' circumferences, axis by axis (a fibred axis gives its base's
    /// and then its fibre's). `None` where some phase is an interval: the
    /// sample is then a product with boundary, never a torus.
    pub fn torus(&self) -> Option<Vec<usize>> {
        let mut periods = Vec::new();
        for a in &self.axes {
            let phases = match a {
                AxisManifold::One(p) => vec![*p],
                AxisManifold::Fibred { base, fibre } => vec![*base, *fibre],
            };
            for p in phases {
                match p {
                    PhaseManifold::Circle { period } => periods.push(period),
                    PhaseManifold::Interval { .. } => return None,
                }
            }
        }
        Some(periods)
    }

    /// The count of the manifold's dimensions: one per phase, a fibred axis
    /// two.
    pub fn dimension(&self) -> usize {
        self.axes.iter().map(|a| if let AxisManifold::Fibred { .. } = a { 2 } else { 1 }).sum()
    }
}

/// The extent of `s` along `axis`, or 0 past its rank.
fn extent(s: &Sample, axis: usize) -> usize {
    match s.extent(axis) {
        AxisReading::Read(e) => e,
        AxisReading::Unread => 0,
    }
}

/// THE PHASE OF `axis` READ OVER `count` STEPS: step `k` stands at the axis
/// coordinate `at(k)` (plus the fine offset, for a block); the least period
/// `p` dividing `count`, below it, with every site's symbol equal to the one
/// `p` steps on (taken round the count), makes a circle; none, an interval
/// of `count`.
fn phase_of(sample: &Sample, axis: usize, count: usize, at: impl Fn(usize) -> Option<usize>) -> PhaseManifold {
    let repeats = |p: usize| {
        sample.sites().all(|site| {
            let Some(c) = site.axis(axis).and_then(|c| usize::try_from(c).ok()) else { return false };
            // The step this coordinate stands at, and its offset past it.
            let Some((k, off)) = (0..count).rev().find_map(|k| at(k).filter(|&s| s <= c).map(|s| (k, c - s))) else { return true };
            let Some(there) = at((k + p) % count).and_then(|s| i64::try_from(s + off).ok()) else { return false };
            same(sample, &site, axis, there)
        })
    };
    match (1..count).filter(|p| count % p == 0).find(|&p| repeats(p)) {
        Some(period) => PhaseManifold::Circle { period },
        None => PhaseManifold::Interval { extent: count },
    }
}

/// THE FIBRE OF A FACTORING: the least period `q` dividing the block size,
/// below it, with every block site's symbol equal to the one `q` on within
/// its block; none, an interval of the block.
fn fibre_of(sample: &Sample, axis: usize, f: &Factoring) -> PhaseManifold {
    let b = f.block();
    let repeats = |q: usize| {
        sample.sites().all(|site| {
            let Some(c) = site.axis(axis).and_then(|c| usize::try_from(c).ok()) else { return false };
            let Some((i, j)) = (0..f.coarse()).find_map(|i| (0..b).find(|&j| f.site(i, j) == Some(c)).map(|j| (i, j))) else { return true };
            let Some(there) = f.site(i, (j + q) % b).and_then(|s| i64::try_from(s).ok()) else { return false };
            same(sample, &site, axis, there)
        })
    };
    match (1..b).filter(|q| b % q == 0).find(|&q| repeats(q)) {
        Some(period) => PhaseManifold::Circle { period },
        None => PhaseManifold::Interval { extent: b },
    }
}

/// Whether `site` and the site at coordinate `there` on `axis` hold one
/// symbol.
fn same(sample: &Sample, site: &Site, axis: usize, there: i64) -> bool {
    match site.with_axis(axis, there) {
        Some(other) => sample.symbol_at(site).ok() == sample.symbol_at(&other).ok(),
        None => false,
    }
}
