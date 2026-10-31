// 21 step 6, 22 — CONTACT: a class whose placement differs across the
// family's frames but whose ring distance to the Frame is one value, and
// whose word orbits, is a contact word; its placement leaves the form.

use super::orbit::positions_of;
use super::perceive::{frame_of, Grid, Painted};
use super::*;
use crate::network::nerve::Keyed;
use crate::physics::Budget;
use crate::transducer::element;
use crate::transducer::sentence::{Form, Vertex};

/// WHAT MOVE 6 READS OFF THE PERCEPT (52.5): the classes whose placement
/// or hue map varies across frames.
///
/// `class_frames` STOOD HERE — the frames each class's members were measured
/// on, written into the sections at class depth when the class stayed
/// sections. Each member holds its frame (AT1, ruled) and the class's
/// section is the union read off them ([`Form::sections`]), so the class
/// frames had a second carrier that had to agree with the first (law 12).
#[derive(Clone)]
pub(crate) struct Varying {
    /// TRANSPORT CLASS → the class's placement varies across frames.
    /// Written from `TransportClass::key` and read against `by_transport`,
    /// so it answers that coordinate and says so (54.7 R5).
    pub(crate) place_varies: Keyed<(), coord::Transport>,
}

impl Varying {
    pub(crate) fn new() -> Varying {
        Varying { place_varies: Keyed::new() }
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
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a resident grid's frame dimensions cannot sum past a 64-bit usize")]
        let bound = fy.0.saturating_add(fy.1);
        // The ring is read in the grid's count; the contact word spells it
        // in a byte, so a ring past it is no contact word — refused, never
        // wrapped onto a nearer ring.
        let r = u8::try_from(ring_dist_peel(&positions_of(&outs), &frame_anchor, bound)?).ok()?;
        match rings {
            None => rings = Some(r),
            Some(h) if h != r => return None,
            _ => {}
        }
    }
    rings
}

/// Does a contact word move the body on every input of the family (22)? A
/// word whose motion fixes the body as a set stands still, and the court
/// refuses it (23.1 step 6); the voice does not mint it.
fn orbits_everywhere(v: &Vertex, family: &[(Grid, Grid)]) -> bool {
    let Vertex::Word { derivation, .. } = v else { return false };
    if derivation.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
        return false;
    }
    family.iter().all(|(x, _)| moves(v, &crate::arc::painted_of_grid(x)))
}

/// Step 6 of chapter 21: a class whose placement differs across frames
/// becomes a contact word when the ring to the Frame is one value and the
/// word orbits; its members' carriages become the identity and its star
/// faces re-derive. A class that does not stays the sections the percept
/// filed (18.2) — its members' frames, which the class's section is read
/// off. `false` when the stroke ended.
pub(crate) fn contact_named_vertices(
    form: &mut Form,
    family: &[(Grid, Grid)],
    varying: &Varying,
    budget: &mut Budget,
) -> bool {
    trace::trace("extract/6 contact", trace::Wrote::Nothing, Some(budget));
    let ring = one_ring(family);
    let mut touched = false;
    // THE KEY STAYS A TRANSPORT COORDINATE ACROSS THE WHOLE BODY (54.7 R5).
    // `scoped` hands out loose bytes, and a key that leaves a typed carrier
    // as bytes can be filed into any other; `entries` is the same walk with
    // the coordinate still on it, so all three reads below are checked.
    for (tk, ()) in varying.place_varies.entries() {
        if !budget.spend() {
            return false;
        }
        let Some(on) = form.by_transport.at(&tk).cloned() else { continue };
        let Some(r) = ring else { continue };
        for at in on {
            let Some(Vertex::Word { contact: None, .. }) = form.vertices.at(&at) else { continue };
            let Some(mut candidate) = form.vertices.at(&at).cloned() else { continue };
            if let Vertex::Word { contact, acts, address, derivation } = &mut candidate {
                *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
                // SECTIONS, not charts (54.7 R5) — `chart ‖ carriage` — and
                // the key IS the carriage, so a member whose carriage becomes
                // the identity is filed again under its new key; two sections
                // of one chart that both become the identity are one member
                // standing on the frames of both.
                let held: Vec<crate::transducer::sentence::Acting> = acts.entries().map(|(_, a)| a.clone()).collect();
                let mut rekeyed: Keyed<crate::transducer::sentence::Acting, coord::Section> = Keyed::new();
                for mut a in held {
                    *a.carriage = element::identity_versor();
                    let k = a.key();
                    if let Some(prior) = rekeyed.drop_at(&k) {
                        for (f, ()) in prior.frames.scoped() {
                            a.frames.insert(&f, ());
                        }
                    }
                    rekeyed.put(&k, a);
                }
                *acts = rekeyed;
                // The address is derived from what the vertex now holds (54.2).
                *address = crate::transducer::sentence::word_vertex_address(derivation, acts, *contact);
            }
            if !orbits_everywhere(&candidate, family) {
                continue;
            }
            form.vertices.put(&at, candidate);
            touched = true;
        }
    }
    if touched {
        form.reindex();
        rederive_star_faces(form);
    }
    true
}
