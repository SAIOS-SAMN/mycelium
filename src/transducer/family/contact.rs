// 21 step 6, 22 — CONTACT: a class whose placement differs across the
// family's frames but whose ring distance to the Frame is one value, and
// whose word orbits, is a contact word; its placement leaves the form.

use super::orbit::positions_of;
use super::perceive::{frame_of, Grid, Painted};
use super::*;
use crate::transducer::sentence::section_of_frames;
use crate::network::nerve::Keyed;
use crate::physics::Budget;
use crate::transducer::element;
use crate::transducer::sentence::{Form, Vertex};

/// WHAT MOVE 6 READS OFF THE PERCEPT (52.5): the classes whose placement
/// or hue map varies across frames, and the frames each class's vertices
/// are claimed on.
#[derive(Clone)]
pub(crate) struct Varying {
    pub(crate) place_varies: Keyed<()>,
    pub(crate) class_frames: Keyed<Vec<u16>>,
}

impl Varying {
    pub(crate) fn new() -> Varying {
        Varying { place_varies: Keyed::new(), class_frames: Keyed::new() }
    }
}

/// The ring distance every output frame reads from its painted sites to
/// the frame's edge, when it is one value on every frame.
fn one_ring(family: &[(Grid, Grid)]) -> Option<u8> {
    let mut rings: Option<u8> = None;
    for (_x, y) in family {
        let fy = frame_of(y);
        let outs: Vec<Painted> = crate::arc::painted_of_grid(y);
        let (Some(last_row), Some(last_col)) = (fy.0.checked_sub(1), fy.1.checked_sub(1)) else {
            return None;
        };
        let frame_anchor: Vec<(i64, i64)> = (0..fy.1 as i64)
            .map(|c| (0, c))
            .chain((0..fy.1 as i64).map(|c| (last_row as i64, c)))
            .chain((0..fy.0 as i64).map(|r| (r, 0)))
            .chain((0..fy.0 as i64).map(|r| (r, last_col as i64)))
            .collect();
        let bound = (fy.0.saturating_add(fy.1)) as u32;
        let r = ring_dist_peel(&positions_of(&outs), &frame_anchor, bound)?;
        match rings {
            None => rings = Some(r),
            Some(h) if h != r => return None,
            _ => {}
        }
    }
    rings
}

/// Does a contact word orbit on every input of the family (22)? A word
/// whose first step leaves the ring unchanged returns `NoOrbit`, and the
/// court refuses it (23.1 step 6); the voice does not mint it.
fn orbits_everywhere(v: &Vertex, family: &[(Grid, Grid)]) -> bool {
    let Vertex::Word { derivation, .. } = v else { return false };
    if derivation.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
        return false;
    }
    family.iter().all(|(x, _)| {
        let painted = crate::arc::painted_of_grid(x);
        let frame = (x.len(), crate::arc::grid_width(x));
        let anchor: Vec<(i64, i64)> = painted.iter().map(|&(r, c, _)| (r, c)).collect();
        !matches!(orbit(v, &painted, &anchor, frame), OrbitEnd::NoOrbit)
    })
}

/// Step 6 of chapter 21: a class whose placement differs across frames
/// becomes a contact word when the ring to the Frame is one value and the
/// word orbits; its members' carriages become the identity and its star
/// faces re-derive. A class that does not stays the sections the percept
/// filed (18.2). `false` when the stroke ended.
pub(crate) fn contact_named_vertices(
    form: &mut Form,
    family: &[(Grid, Grid)],
    varying: &Varying,
    sections: &mut Keyed<Keyed<()>>,
    budget: &mut Budget,
) -> bool {
    trace::trace("extract/6 contact", trace::Wrote::Nothing, Some(budget));
    let ring = one_ring(family);
    let mut touched = false;
    for (tk, ()) in varying.place_varies.scoped() {
        if !budget.spend() {
            return false;
        }
        let Some(on) = form.by_transport.get(&tk).cloned() else { continue };
        let Some(r) = ring else {
            // The sections the percept filed stand; the class is claimed on
            // the frames its members were measured on.
            if let Some(frames) = varying.class_frames.get(&tk) {
                sections.insert(&tk, section_of_frames(frames.iter().copied()));
            }
            continue;
        };
        for i in on {
            let Some(Vertex::Word { contact: None, .. }) = form.vertices.get(i as usize) else { continue };
            let Some(mut candidate) = form.vertices.get(i as usize).cloned() else { continue };
            if let Vertex::Word { contact, acts, .. } = &mut candidate {
                *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
                let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                for k in charts {
                    if let Some(a) = acts.get_mut(&k) {
                        *a.carriage = element::identity_versor();
                    }
                }
            }
            if !orbits_everywhere(&candidate, family) {
                if let Some(frames) = varying.class_frames.get(&tk) {
                    sections.insert(&tk, section_of_frames(frames.iter().copied()));
                }
                continue;
            }
            if let Some(v) = form.vertices.get_mut(i as usize) {
                *v = candidate;
                touched = true;
            }
        }
    }
    if touched {
        form.reindex();
        rederive_star_faces(form);
    }
    true
}
