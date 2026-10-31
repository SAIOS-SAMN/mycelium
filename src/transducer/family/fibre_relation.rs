use super::TrainPair;

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
/// court re-derives it from the grids it holds (`element::colour_word`),
/// so the word stands by evidence and never by a count of who said it.
pub fn fibre_relation(family: &[TrainPair]) -> Option<Vec<(u8, u8)>> {
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
    crate::arc::on_ground_of(family, || fibre_map_on_ground(family))
}

fn fibre_map_on_ground(family: &[TrainPair]) -> Option<Vec<(u8, u8)>> {
    let void = crate::arc::ground();
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (x, y) in family {
        // 78.56 — WHERE THE BASE STANDS STILL across a change of extent:
        // each input site at the output site the envelopes' dilation
        // carries it to, never aligned at a corner. A pair whose dilation
        // is irrational has no site that corresponds, and witnesses nothing.
        let Some(s) = crate::arc::envelope_dilation(x, y) else { continue };
        for (site, h) in crate::arc::painted_sites(x) {
            let Some(there) = crate::arc::carried_site(x, y, s, site) else { continue };
            let t = crate::arc::hue_at(y, there);
            if t == void {
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
