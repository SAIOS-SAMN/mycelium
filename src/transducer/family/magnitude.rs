// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6 — THE MAGNITUDE A FAMILY PRESENTS: each train pair's μ⃗ (the
//! coboundary's lane measure), joined lane by lane over the family, which
//! is the genesis bond a court stakes against a corpus (46, E1). Moved from
//! the former `arc` module unchanged (ROADMAP track 26, S1). The pairs are
//! read as planar chains (`raster::painted_of`), a defect named in
//! MU-SENSING §8 for samples of rank `n` to replace (track 26, S4).

use super::Family;
use crate::transducer::raster::painted_of;

/// The μ⃗ of one train pair: the coboundary's lane measure. T is not on the chain.
/// None when the coboundary is unmeasured.
pub fn pair_magnitude(a: &[Vec<u8>], b: &[Vec<u8>]) -> Option<[u64; 4]> {
    let ca = painted_of(a);
    let cb = painted_of(b);
    crate::transducer::germinate::germinate(&ca, &cb)
        .map(|g| crate::transducer::element::measure_lanes(&g))
}

/// Lane-wise accumulation of one pair's μ⃗ into a bundle's. The pairs of a
/// bundle are DISJOINT SCENES, so a bundle's measure on a lane is the sum of
/// its pairs': chapter 12.6, "the lane-wise sum of the per-pair pressures,
/// each pair measured by `measure_lanes` on its own residue, the pairs
/// being disjoint scenes; never a maximum over pairs". A maximum reports a
/// bundle as no larger than its largest pair, which is how a genesis bond
/// measured from a corpus came out too small to cover a single pair of it.
/// Lanes never meet: each accumulates alone. `None` on overflow: a lane
/// that cannot hold its own sum is unmeasured, never capped (rule 11).
pub fn cover_join(cover: &mut [u64; 4], mag: &[u64; 4]) -> Option<()> {
    for l in 0..4 {
        cover[l] = cover[l].checked_add(mag[l])?;
    }
    Some(())
}

/// Genesis bond of one family: the lane-wise measure of its train pairs,
/// which is the work the family presents. The train is the family's
/// boundary. `None` when no pair is measured or a lane overflows.
pub fn family_cover(train: &Family) -> Option<[u64; 4]> {
    let mut cover = [0u64; 4];
    let mut any = false;
    for (a, b) in train {
        let Some(mag) = pair_magnitude(a, b) else {
            continue;
        };
        if mag.iter().all(|&x| x == 0) {
            continue;
        }
        any = true;
        cover_join(&mut cover, &mag)?;
        // H carries the pair's own coboundary extent, accumulated with the
        // rest: the same disjoint-scene rule as every other lane.
        let chain = crate::transducer::germinate::germinate(&painted_of(a), &painted_of(b))?;
        cover[3] = cover[3].checked_add(chain.len() as u64)?;
    }
    any.then_some(cover)
}
