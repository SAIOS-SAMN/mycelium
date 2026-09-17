// 21 step 2 — THE INCIDENCE OF A SCENE: the faces two supports name, and
// the hue carrier a speech indexes by.
//
//! A face exists because two bodies' supports meet or sit at ring distance
//! one, read off a keyed occupancy of the sites (rule 1); nothing here
//! pairs one body with another, and nothing ranks.

use super::*;

thread_local! {
    static FACE_WORK: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn note_face_work() {
    FACE_WORK.with(|c| c.set(c.get().saturating_add(1)));
}

#[cfg(test)]
pub(crate) fn reset_face_work() {
    FACE_WORK.with(|c| c.set(0));
}

#[cfg(test)]
pub(crate) fn face_work() -> u64 {
    FACE_WORK.with(|c| c.get())
}

/// Faces the bundle names: star from the Frame (when the form carries one),
/// plus pairs whose after-supports overlap or sit at ring distance one.
/// Occupancy is keyed by site; related pairs are a Keyed set.
///
/// COST IS THE SITES, NOT THE BODIES ON THEM. A site with k occupants
/// contributes k−1 faces, not k(k−1)/2 — see the chain below for why the
/// chords are not faces the complex is missing. Every face this writes is
/// counted through `note_face_work`, so the gauge in
/// `face_work_is_linear_in_occupied_sites` reads the whole cost and not the
/// part that was already linear.
///
/// The supports arrive KEYED BY MEMBER — a vertex AND the chart it stands at
/// there (D10) — which is the coordinate a face is actually between: 21 step 2
/// relates two BODIES, and a vertex covering several bodies on several charts is
/// several occupants of the sites they sit on. Keyed by the vertex alone, the
/// pairs this returns would name a relation between two vertices and leave
/// which of their bodies met to be guessed by whatever built the face.
///
/// The pairs are member pairs, so the face is created already carrying which
/// member of `i` relates to which member of `j`. NO PRODUCT OF TWO FIBRES is
/// taken anywhere: the occupancy is keyed by SITE and a site names its
/// occupants, so the chain over `windows(2)` yields incidences the scene has.
///
/// THE MEMBERS ARE READ FROM THE INDEX. `supports` IS the member index, and
/// walking it is reading it (rule 1); a count of the integers with a keyed
/// read inside finds nothing for a vertex with no support and says nothing
/// about one the index holds and the count does not reach.
pub(crate) fn star_and_site_faces(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<(i64, i64, i64)>>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut related: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    if let Some(frame) = frame {
        for (mk, _) in supports.scoped() {
            if mk == frame {
                continue;
            }
            related.insert(member_pair_key(frame, &mk), ());
        }
    }
    let mut occ: crate::network::nerve::Keyed<Vec<Vec<u8>>> = crate::network::nerve::Keyed::new();
    for (mk, supp) in supports.scoped() {
        if Some(mk.as_slice()) == frame {
            continue;
        }
        for &(r, c, f) in supp {
            note_face_work();
            let k = base_key(r, c, f);
            match occ.get_mut(&k) {
                // The members ascend in key order and a member is offered its
                // own support once, so a repeat can only be the one just
                // pushed; a walk of the site's occupants on every offer would
                // be k² comparisons on the same site that the chain below is
                // already paying k² for.
                Some(v) => match v.as_slice() {
                    [.., last] if *last == mk => {}
                    _ => v.push(mk.clone()),
                },
                None => {
                    occ.insert(k, vec![mk.clone()]);
                }
            }
        }
    }
    // A SITE'S OCCUPANTS ARE A SIMPLEX, NOT A CLIQUE. Every vertex on this
    // site meets every other ON IT, so the subcomplex they span is a simplex
    // — contractible, and every cycle inside it is a boundary. Writing all
    // k(k−1)/2 of those pairs as faces would hand `loops_of` a cycle basis
    // of the 1-skeleton with nothing to fill k(k−1)/2 − (k−1) loops that
    // bound by construction, and the pivot drive would clone the whole form
    // once per candidate against each one.
    //
    // The chain is that simplex's spanning tree: k−1 faces, the same
    // connectivity and the same components, and the only cycles that survive
    // are the ones spanning DISTINCT sites — which are the cycles that carry
    // a class. The occupants are read in the form's own vertex coordinate,
    // which is content order, so two minds holding one scene chain it alike.
    for (_k, members) in occ.scoped() {
        for w in members.windows(2) {
            let [a, b] = w else { continue };
            note_face_work();
            related.insert(member_pair_key(a, b), ());
        }
    }
    for (mk, supp) in supports.scoped() {
        if Some(mk.as_slice()) == frame {
            continue;
        }
        for (r, c, f) in ring_of_support(supp) {
            note_face_work();
            let Some(others) = occ.get(&base_key(r, c, f)) else {
                continue;
            };
            for u in others {
                if *u != mk {
                    related.insert(member_pair_key(&mk, u), ());
                }
            }
        }
    }
    related
        .scoped()
        .filter_map(|(k, _)| read_member_pair_key(&k))
        .collect()
}

/// Physical size of the hue carrier. Index by hue, never search.
pub const PALETTE: usize = 256;
pub type HueMap = [Option<u8>; PALETTE];

pub(crate) fn hue_map_from_pairs(pairs: &[(u8, u8)]) -> HueMap {
    let mut m = [None; PALETTE];
    for &(f, t) in pairs {
        m[f as usize] = Some(t);
    }
    m
}
