// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6 — THE MAGNITUDE A FAMILY PRESENTS: each train pair's lane counts,
//! joined lane by lane over the family, which is the genesis bond a court
//! stakes against a corpus (46, E1). Read on the samples at their own rank
//! `n` (MU-SENSING §2; ruled 2026-11-06 on #32, track 26 S4):
//! - S and G from the transduction's projected record
//!   (`Transduction::projected`, on one enclosure the coboundary link for
//!   link);
//! - H from the boundary by grade `n − 1` of the support before and after;
//!   a chain's length is no holonomy and is not added;
//! - a pair whose record does not read (its re-period does not land,
//!   `OffSite`, or its ranks differ) is held as a residue citing the pair
//!   and the refusal, and contributes nothing: it is never aligned at a
//!   corner and never dropped.
//!
//! The bond stays a count. The share of the frame is v0.5.0's (78.174,
//! MU-ECONOMY §12).

use super::transduce::{need_of, Ground, SiteChange, Transduction, TransductionRefusal};
use crate::transducer::base::Site;
use crate::transducer::sample::Sample;

/// A PAIR THE MAGNITUDE DOES NOT READ, held as a residue (ruled 2026-11-06
/// on #32): its index in the family's train and the transduction's refusal
/// naming why, such as `OffSite` with the turn and the period where the
/// re-period does not land.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unmeasured {
    pub pair: usize,
    pub refusal: TransductionRefusal,
}

/// THE MAGNITUDE OF A FAMILY: the lane-wise sum of the counts of its
/// measured pairs, and the pairs held as residue beside it, never folded
/// into the counts.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct FamilyCover {
    lanes: [u64; 4],
    measured: usize,
    residue: Vec<Unmeasured>,
}

impl FamilyCover {
    /// The bond: the lane-wise sum over the measured pairs; `None` where no
    /// pair is measured (an unmeasured family is no zero bond).
    pub fn bond(&self) -> Option<[u64; 4]> {
        (self.measured > 0).then_some(self.lanes)
    }

    /// The lane-wise sum over the measured pairs, zero where none is.
    pub fn lanes(&self) -> [u64; 4] {
        self.lanes
    }

    /// The pairs held as residue, in train order.
    pub fn residue(&self) -> &[Unmeasured] {
        &self.residue
    }
}

/// THE PROJECTED RECORD OF ONE PAIR at its own rank, under the stroke its
/// periods need: the sites of the output frame where the re-periodded input
/// says otherwise. The transduction's refusal where it does not read: two
/// ranks (`Rank`), a need past `usize` (`Spent`), a re-period that does not
/// land (`OffSite`).
pub fn pair_record(a: &Sample, b: &Sample) -> Result<Vec<SiteChange>, TransductionRefusal> {
    if a.rank() != b.rank() {
        return Err(TransductionRefusal::Rank { from: a.rank(), to: b.rank() });
    }
    let need = need_of(a, b).ok_or(TransductionRefusal::Spent { need: None })?;
    let mut stroke = crate::physics::Physics::default().stroke_n(need);
    Transduction::of(a, b, &mut stroke)?.projected()
}

/// THE LANE COUNTS OF A PROJECTED RECORD under a ground: S counts the sites
/// changed between ground and painted, G the sites changed otherwise (one
/// painted symbol to another, or one candidate of a vortex to another), as
/// `Transduction::lane_shares` reads them; T is not on the record; H is the
/// change of the support's boundary by grade `n − 1`, the difference of the
/// components and of the free faces of the sites painted before and of
/// those painted after. `None` past `u64`.
pub fn record_lanes(record: &[SiteChange], ground: &Ground<Vec<u8>>) -> Option<[u64; 4]> {
    let (mut spatial, mut gauge) = (0u64, 0u64);
    let (mut before, mut after) = (Vec::new(), Vec::new());
    for c in record {
        let (painted_b, painted_a) = (!ground.holds_symbol(c.before()), !ground.holds_symbol(c.after()));
        match painted_b != painted_a {
            true => spatial = spatial.checked_add(1)?,
            false => gauge = gauge.checked_add(1)?,
        }
        if painted_b {
            before.push(c.site().clone());
        }
        if painted_a {
            after.push(c.site().clone());
        }
    }
    let (sb, sa) = (boundary_of(before)?, boundary_of(after)?);
    let shell = sb.0.abs_diff(sa.0).checked_add(sb.1.abs_diff(sa.1))?;
    Some([spatial, gauge, 0, shell])
}

/// A SUPPORT'S BOUNDARY BY GRADE `n − 1`: its components under face contact
/// (`sentence::incidence_of_sites`), and its free faces, `2n` per site less
/// two per shared face. At rank 2 this is the planar signature's component
/// count and boundary (`candidates::signature`).
fn boundary_of(mut sites: Vec<Site>) -> Option<(u64, u64)> {
    sites.sort_unstable();
    sites.dedup();
    let faces = crate::transducer::sentence::incidence_of_sites(&sites);
    let mut roots = crate::transducer::sentence::components(sites.len(), &faces);
    roots.sort_unstable();
    roots.dedup();
    // Each site's `2n` faces at its own `n`.
    let sides = sites.iter().try_fold(0u64, |s, site| s.checked_add(u64::try_from(site.n().checked_mul(2)?).ok()?))?;
    let shared = u64::try_from(faces.len().checked_mul(2)?).ok()?;
    Some((u64::try_from(roots.len()).ok()?, sides.checked_sub(shared)?))
}

/// Lane-wise accumulation of one pair's counts into a bundle's. The pairs
/// of a bundle are DISJOINT SCENES, so a bundle's measure on a lane is the
/// sum of its pairs': chapter 12.6, "the lane-wise sum of the per-pair
/// pressures, each pair measured on its own residue, the pairs being
/// disjoint scenes; never a maximum over pairs". Lanes never meet: each
/// accumulates alone. `None` on overflow: a lane that cannot hold its own
/// sum is unmeasured, never capped (rule 11).
pub fn cover_join(cover: &mut [u64; 4], mag: &[u64; 4]) -> Option<()> {
    for l in 0..4 {
        cover[l] = cover[l].checked_add(mag[l])?;
    }
    Some(())
}

/// THE MAGNITUDE OF A FAMILY OF SAMPLES at any rank, under `ground`: each
/// pair's [`record_lanes`] joined lane by lane, each pair whose record does
/// not read held as [`Unmeasured`]. `None` where a lane overflows.
pub fn family_cover_of<'a>(train: impl IntoIterator<Item = (&'a Sample, &'a Sample)>, ground: &Ground<Vec<u8>>) -> Option<FamilyCover> {
    let mut cover = FamilyCover::default();
    for (pair, (a, b)) in train.into_iter().enumerate() {
        match pair_record(a, b) {
            Ok(record) => {
                cover_join(&mut cover.lanes, &record_lanes(&record, ground)?)?;
                cover.measured += 1;
            }
            Err(refusal) => cover.residue.push(Unmeasured { pair, refusal }),
        }
    }
    Some(cover)
}

/// [`family_cover_of`] on a posted family of any rank, under the ground its
/// own samples show (MU-SENSING §3.4).
pub fn family_cover(train: &[(Sample, Sample)]) -> Option<FamilyCover> {
    family_cover_of(train.iter().map(|(a, b)| (a, b)), &Ground::of(train))
}

/// [`family_cover_of`] on a family of the alphabet's section, under the
/// ground open on this thread, or the family's own where none is
/// (`raster::on_ground_of`).
pub fn section_cover(train: &super::Family) -> Option<FamilyCover> {
    let ground = crate::transducer::raster::on_ground_of(train, crate::transducer::raster::ground).map(|&b| Some(vec![b]));
    family_cover_of(train.iter().map(|(a, b)| (a.sample(), b.sample())), &ground)
}
