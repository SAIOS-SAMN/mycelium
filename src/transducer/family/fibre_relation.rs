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
        // each input site at the output site the envelopes' dilation
        // carries it to, never aligned at a corner. A pair whose dilation
        // is irrational has no site that corresponds, and witnesses nothing.
        let Some(s) = envelope_dilation(x.sample(), y.sample()) else { continue };
        for (site, h) in crate::transducer::raster::painted_sites(x) {
            let Some(there) = carried_site(x, y, s, site) else { continue };
            let Some(t) = crate::transducer::raster::value_at(y, there) else { continue };
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

// ── THE ENVELOPES' DILATION (78.56), read by the fibre map above and by the
// frame's momentum (`raster::momentum_vanishes`). Moved from the former
// `arc` module unchanged; it squares the extents (r² = Σ eᵢ²/4), which
// law 11 refuses, and is listed as a defect in MU-SENSING §8 for the
// re-period of each phase (track 26, S4) to replace.

/// 78.56 — THE DILATION BETWEEN TWO ENVELOPES. A raster is enveloped by its
/// circumscribed sphere (centre the extent's midpoint, r² = Σ eᵢ²/4 over
/// its axes), and a change of extent is the dilation about the centres
/// carrying one envelope onto the other, by s = r_to / r_from. It is exact
/// only where r²_to / r²_from is a rational square: answered as `s = p/q`
/// in lowest terms, `None` where s is irrational (78.45: no approximation)
/// or an extent is empty. Two samples of one extent: 1/1. Read off the
/// samples' extents alone, at each sample's own rank.
pub(crate) fn envelope_dilation(from: &crate::transducer::sample::Sample, to: &crate::transducer::sample::Sample) -> Option<(i64, i64)> {
    let r2 = |g: &crate::transducer::sample::Sample| -> i64 { g.extents().iter().map(|&e| (e as i64) * (e as i64)).sum() };
    let (a, b) = (r2(to), r2(from));
    if a == 0 || b == 0 {
        return None;
    }
    let g = gcd_i64(a, b);
    let (a, b) = (a / g, b / g);
    let (p, q) = (exact_root(a)?, exact_root(b)?);
    Some((p, q))
}

/// Where a site of `from` stands in `to` under the envelopes' dilation
/// ([`envelope_dilation`], `s = p/q`), read in centred coordinates doubled
/// so every centre is an integer: `X = 2x − (e − 1)`. `None` where the
/// carried point is not a site of `to`: between sites, or outside it.
pub(crate) fn carried_site(
    from: &super::SectionSample,
    to: &super::SectionSample,
    (p, q): (i64, i64),
    site: [usize; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK],
) -> Option<[usize; crate::transducer::alphabet::ALPHABET_SAMPLE_RANK]> {
    use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
    use crate::transducer::raster::centred;
    let (e_from, e_to) = (from.extents(), to.extents());
    let (p, q) = (i128::from(p), i128::from(q));
    let mut out = [0usize; ALPHABET_SAMPLE_RANK];
    for axis in 0..ALPHABET_SAMPLE_RANK {
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
    Some(out)
}

/// The greatest common divisor of two positive integers.
fn gcd_i64(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// The exact square root of a non-negative integer, or `None` where it is
/// not a perfect square: Newton's iteration on the integers, which strictly
/// decreases from `n` to ⌊√n⌋ (O(log n) steps, bounded by its own descent,
/// rule 10), never a walk up from zero.
fn exact_root(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }
    if n < 2 {
        return Some(n);
    }
    let mut x = n;
    let mut next = (x + 1) / 2;
    while next < x {
        x = next;
        next = (x + n / x) / 2;
    }
    (x * x == n).then_some(x)
}
