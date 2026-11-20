// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §3.1, §3.7; MU-ANTI-FLATTENING 12 — A SECTION READ AS TURNS:
//! the chart of one section's support, each site a turn in ℚ/ℤ of the
//! section's own periods, with no conformal lift.
//!
//! The section lifts by enclosure: its span along the plane is its frame,
//! one phase per plane axis of the span's extent, and a site at offset `s`
//! from the span's least corner on a phase of period `p` is the turn `s/p`
//! (`Turn::of`). The chart is read on Sⁿ (78.188) from the support the turns
//! land on (`Turn::lands`): its `n` and its Betti numbers, and the one hue
//! fraction a hue-blind section carries. The second moments are read in the
//! frame's own gauge, a turn's offset from the phases' mean turn times the
//! phase's period, each compartment with its round's extent (A4); they are a
//! reading, not a chart axis. A chart only locates (71.2(6)), and its axes
//! are those [`super::harmonics::sphere_axes`] lays out.

use crate::transducer::base::Site;
use crate::transducer::family::transduce::{Enclosure, Turn};
use crate::transducer::spelled::{PaintedSite, SectionSite};
use crate::transducer::stalk::ChartAxis;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// ONE SECTION AS TURNS: the section's enclosure (one period per axis of
/// the plane, the extent of its span) and each site's turns, one per phase,
/// in the order the sites were handed.
#[derive(Clone, Debug)]
pub struct SectionTurns {
    enclosure: Enclosure,
    sites: Vec<Vec<Turn>>,
}

impl SectionTurns {
    /// LIFT A SECTION BY ENCLOSURE: the span of its sites is its frame, and
    /// each site the turn of its offset from the span's least corner on
    /// each phase. `None` for no site (no frame to enclose), or where an
    /// offset passes the machine word.
    pub fn of(sites: &[SectionSite]) -> Option<SectionTurns> {
        let (corner, extents) = SectionSite::span_of(sites.iter().copied())?;
        let enclosure = Enclosure::new(extents.to_vec()).ok()?;
        let sites = sites
            .iter()
            .map(|s| Some(s.offset_from(corner)?.indices()?.iter().zip(extents).map(|(&r, h)| Turn::of(r, h)).collect()))
            .collect::<Option<Vec<Vec<Turn>>>>()?;
        Some(SectionTurns { enclosure, sites })
    }

    /// The section's enclosure: one period per phase.
    pub fn enclosure(&self) -> &Enclosure {
        &self.enclosure
    }

    /// Each site's turns, one per phase.
    pub fn turns(&self) -> &[Vec<Turn>] {
        &self.sites
    }

    /// THE SUPPORT THE TURNS LAND ON, on the family's base: each turn landed
    /// on its phase's period, read at the sensor as the compartment of the
    /// section's scene (the place `0`), sorted and unique. `None` where a
    /// turn lands on no site of its period.
    pub fn support(&self) -> Option<Vec<Site>> {
        let mut support = self
            .sites
            .iter()
            .map(|turns| {
                let mut at = [0usize; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK];
                for ((slot, t), &p) in at.iter_mut().zip(turns).zip(self.enclosure.periods()) {
                    *slot = t.lands(p).ok()?;
                }
                let site = SectionSite::of_indices(at)?;
                Some(crate::transducer::blade::Compartment::from(PaintedSite::new(site, 0)).site)
            })
            .collect::<Option<Vec<Site>>>()?;
        support.sort_unstable();
        support.dedup();
        Some(support)
    }

    /// THE SECOND MOMENTS ON THE PLANE, in the frame's gauge: per phase
    /// pair `(k, l)`, `Σ (t_k − t̄_k)·p_k · (t_l − t̄_l)·p_l` over the sites,
    /// with each compartment's round extent on the diagonal of a phase its
    /// base reads as space (`round_extent`). `t̄` is the mean turn, a share
    /// along the phase's line (ℚ), and `p` the phase's period: the turn
    /// read in the frame's own gauge. Rows in phase order.
    pub fn moments(&self, n: usize, plane: (usize, usize)) -> Vec<Vec<Q>> {
        let count = q(self.sites.len());
        let periods: Vec<Q> = self.enclosure.periods().iter().map(|&p| q(p)).collect();
        let means: Vec<Q> = (0..periods.len())
            .map(|k| self.sites.iter().filter_map(|t| t.get(k)).fold(Q::zero(), |s, t| s + t.value()) / &count)
            .collect();
        let offsets: Vec<Vec<Q>> = self
            .sites
            .iter()
            .map(|t| t.iter().zip(&means).zip(&periods).map(|((t, m), p)| (t.value() - m) * p).collect())
            .collect();
        let axes = [plane.0, plane.1];
        (0..periods.len())
            .map(|k| {
                (0..periods.len())
                    .map(|l| {
                        let mut m = offsets.iter().fold(Q::zero(), |s, d| s + &d[k] * &d[l]);
                        if k == l {
                            m += super::body::round_extent(n, axes[k]) * &count;
                        }
                        m
                    })
                    .collect()
            })
            .collect()
    }

    /// THE CHART OF THE SECTION on Sⁿ (78.188; A21 as ruled 2026-09-18),
    /// read from its turns: the base's `n`, the Betti numbers of the support
    /// the turns land on, then the one hue fraction a hue-blind section
    /// carries. `None` where the support's topology is refused or its base
    /// holds no plane (the section's plane is what its turns are read on).
    pub fn chart(&self) -> Option<Vec<ChartAxis>> {
        let support = self.support()?;
        let n = support.iter().map(Site::n).fold(0, usize::max);
        crate::transducer::perspective::family_plane(n)?;
        let betti = super::topology(&support)?;
        // One hue covers the whole section: its fraction is the whole.
        Some(super::harmonics::sphere_axes(u64::try_from(n).ok()?, &betti, &[<Q as num_traits::One>::one()]))
    }
}

fn q(v: usize) -> Q {
    Q::from_integer(BigInt::from(v))
}
