use super::TrainPair;
use crate::transducer::element::Reperiod;

/// 78.53 — THE FAMILY'S FIBRE RELATION: the section transition the colour
/// fibre undergoes where the base stands still. At every site that stands
/// in both the input and the output of a train pair (neither the void),
/// the hue there goes to the hue there; over every pair, that is one map or
/// none, lifted unitarily (78.58, `element::lift_fibre`): a hue standing
/// still is no move and is not written, but it keeps its target, and every
/// other hue merged into a target rotates its distinction into the
/// reservoir. `None` where the map is not a function (a hue sent two ways),
/// where it moves no hue, or where no site stands in both scenes. Read in
/// the detector's own channels
/// — an absolute boundary (78.56 II) — so the colour words minds share name
/// the same colours for every mind.
///
/// The ONE reader of it: the mind mints its colour word from it and the
/// court re-derives it from the samples it holds (`element::colour_word`),
/// so the word stands by evidence and never by a count of who said it.
pub fn fibre_relation(family: &[TrainPair]) -> Option<crate::transducer::element::FibreRelation> {
    // 78.58 — the function the scenes showed, a hue standing still
    // included, lifted unitarily: a merge rotates its distinction into
    // the reservoir. Dropping the still hues first read a hue recoloured
    // into one that stayed as an injection, and minted a merge as if
    // nothing were lost.
    crate::transducer::element::lift_fibre(&fibre_map(family)?)
}

/// THE MAP THE SCENES SHOW of the hue fibre where the base stands still,
/// before any lift: each hue at each site standing in both scenes to the hue
/// there, a hue standing still included — the map a perception's faces
/// carry, and so the coordinate the field files a 𝒢 orbit under
/// (MU-CLIENT §7.1, the corpus met where the mind's orbits stand). `None`
/// where a hue goes two ways.
pub fn fibre_map(family: &[TrainPair]) -> Option<Vec<(u8, u8)>> {
    crate::transducer::raster::on_ground_of(family, || fibre_map_on_ground(family))
}

fn fibre_map_on_ground(family: &[TrainPair]) -> Option<Vec<(u8, u8)>> {
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (x, y) in family {
        // 78.56 — WHERE THE BASE STANDS STILL across a change of extent:
        // each input site at the output site the re-period of each phase
        // carries it to, about the centres, never aligned at a corner. A
        // pair of two ranks has no site that corresponds, and witnesses
        // nothing.
        let Some(r) = Reperiod::of_extents(x.sample().extents(), y.sample().extents()) else { continue };
        for p in crate::transducer::raster::painted_sites(x) {
            let h = p.symbol;
            let Some(there) = carried_site(x, y, &r, p.site) else { continue };
            let Some(t) = y.at(there) else { continue };
            if crate::transducer::raster::is_ground(t) {
                continue;
            }
            match map.iter().find(|(f, _)| *f == h) {
                Some(&(_, seen)) if seen != t => return None,
                Some(_) => {}
                None => map.push((h, t)),
            }
        }
    }
    Some(map)
}

// ── THE RE-PERIOD OF EACH PHASE (MU-SENSING §3.1; track 26 S4), read by the
// fibre map above and by the frame's momentum (`raster::momentum_vanishes`)
// through the one measured carrier, `element::Reperiod`. It replaces the
// envelopes' dilation, which squared the extents (r² = Σ eᵢ²/4) and is
// refused by law 11: two samples relate by the re-period of each phase,
// `p′ᵢ/pᵢ`, never by one scale read off a sphere.

/// Where a site of `from` stands in `to` under the re-period of each phase
/// (`Reperiod`, `p′ᵢ/pᵢ`), read in centred coordinates doubled so every
/// centre is an integer: `X = 2x − (e − 1)`, carried to `X·p′/p`. `None`
/// where the carried point is not a site of `to` (between sites, or outside
/// it), where an input period is empty, or where the re-period is not of
/// the plane's rank.
pub(crate) fn carried_site(
    from: &super::SectionSample,
    to: &super::SectionSample,
    reperiod: &Reperiod,
    site: crate::transducer::spelled::SectionSite,
) -> Option<crate::transducer::spelled::SectionSite> {
    use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
    use crate::transducer::raster::centred;
    let (e_from, e_to) = (from.extents(), to.extents());
    let site = site.coordinates();
    let mut out = [0usize; ALPHABET_SAMPLE_RANK];
    for axis in 0..ALPHABET_SAMPLE_RANK {
        let &(p_in, p_out) = reperiod.periods().get(axis)?;
        let (p, q) = (i128::try_from(p_out).ok()?, i128::try_from(p_in).ok()?);
        if q == 0 {
            return None;
        }
        let scaled = centred(site[axis], e_from[axis]).checked_mul(p)?;
        if scaled % q != 0 {
            return None;
        }
        let doubled = scaled / q + (e_to[axis] as i128 - 1);
        if doubled % 2 != 0 || doubled < 0 {
            return None;
        }
        let at = usize::try_from(doubled / 2).ok()?;
        if at >= e_to[axis] {
            return None;
        }
        out[axis] = at;
    }
    crate::transducer::spelled::SectionSite::of_indices(out)
}
