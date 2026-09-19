// 23, 42 — REPLAY: a spoken form is re-derived from its own tape, move by
// move, against the cover that resolves what it cites.
//
//! Every vertex re-derives from its tape, every face from its vertices,
//! every loop's holonomy to the identity, and every pivot certificate from
//! the axis it names. A form that does not is REFUSED, never scored.

use super::body::{edges_rederive_family, undeclared_remainder, vertices_carry_family};
use super::*;

/// Chapter 23.1: replay a crystallized form from its tape and the forms it
/// cites. `resolve` is the hypha's ruled cover (a caller without one
/// resolves nothing, and every citation refuses): each cited form must be
/// ruled, closed, at the depth the tape claims and strictly below the
/// citer's, with an identity frame (chapter 32); every face re-derives from
/// its two vertices by M1, or is the chord of a loop whose holonomy it
/// carries and whose class is filed on torsion; every loop of the
/// recomputed spanning forest is projectively identity or filed finite
/// torsion. Descent is checked by the caller on the element.
pub fn replay_form(spoken: &Element, resolve: &dyn Fn(&[u8]) -> Option<Form>) -> Option<&'static str> {
    replay_form_with(spoken, resolve, None, &crate::transducer::family::NoCrystal)
}

/// Chapter 23.1 with the bound family, so rebase/unfold/orbit execute on
/// the trains when the kernel has them (step 6). Descent is measured
/// through `crystal` (the court's genome). `NoCrystal` is only for a tape
/// with no Glued vertex. The reason alone; `replay_form_at` carries the
/// coordinate.
pub fn replay_form_with(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<&'static str> {
    replay_form_at(spoken, resolve, family, crystal).map(|r| r.reason)
}

/// 50.2 step 2 at the court. Each cited `(form, vertex)` is TWO keyed
/// reads and no walk (law 3): the cover resolves the form by address, and
/// the form's own `by_address` resolves the vertex inside it. `Composes`
/// passes; `Differs` refuses at that vertex; `Unresolved` is the cited
/// form the court has not ruled, which takes `REFUSE_GLUED_UNRULED` on
/// the tape — the same deferral a `Glued` citation takes, so the sentence
/// is docketed and the syllabus serves the generator rather than the
/// voice being refused for speaking ahead of this court's cover.
/// DOES THIS FACE RE-DERIVE FROM ITS TWO VERTICES — the word once, and the
/// placement and hue map at every member pair the face records (D10).
///
/// The pairs are the face's own: re-deriving them from the vertices would take
/// the product of the two fibres, and which bodies a face relates is the
/// scene's fact, not something the court recomputes. A pair the two vertices no
/// longer relate does not re-derive, and the face refuses.
fn face_rederives(f: &Form, face: &crate::transducer::sentence::FormFace) -> bool {
    let (Some(vi), Some(vj)) = (
        f.vertices.get(face.i as usize),
        f.vertices.get(face.j as usize),
    ) else {
        return false;
    };
    if f.face_word(face.i, face.j).as_ref() != Some(&face.word) {
        return false;
    }
    for (pk, place) in face.at.scoped() {
        let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
            return false;
        };
        let (Some(ci), Some(cj)) = (vi.chart_keyed(&ik), vj.chart_keyed(&jk)) else {
            return false;
        };
        // The edge is evidence the court re-measures against the FAMILY
        // (`edges_rederive_family`), not something the vertices re-derive, so it
        // is carried in and compared by its presence alone here.
        match f.face_place(face.i, &ci, face.j, &cj, place.edge.clone()) {
            Some(p) if p == *place => {}
            _ => return false,
        }
    }
    true
}

fn derivations_compose(f: &Form, resolve: &dyn Fn(&[u8]) -> Option<Form>) -> Option<Refusal> {
    use crate::transducer::alphabet::Verified;
    for v in &f.vertices {
        let Vertex::Word { address, derivation, .. } = v else {
            continue;
        };
        // THE CHART IN THE QUESTION (D10 item 10). A derivation is a vertex
        // coordinate, so the composition is asked once; what the refusal NAMES
        // is a chart, and the chart a vertex acting on several is refused at is
        // the one its derivation was read for — which is the vertex's whole
        // fibre, because the derivation is the same at each. A refusal at the
        // representative named one chart of several arbitrarily, so this names
        // every chart it acts on, one refusal each, and the first is the one
        // returned.
        // The refusal's chart is the LEAST the vertex stands at, and the slice
        // pattern says so: `charts()` yields them in the fibre's trie order,
        // which is the chart keys sorted, so the least is CONTENT and the same
        // on every node — not the one that happened to arrive first (rule 15).
        let charts = v.charts();
        let [class, ..] = charts.as_slice() else {
            continue;
        };
        // The ACTION at the cited vertex (L14). A citation of an adjoined
        // generator resolves here exactly as one of a spelled generator
        // does, because rotors compose whether or not the seventeen
        // spell them.
        let held = |form: &crate::transducer::address::Address,
                    vertex: &crate::transducer::address::Address| {
            let cited = resolve(form.as_bytes())?;
            sentence::rotor_at_vertex(&cited, vertex.as_bytes())
        };
        match derivation.verify_with(&held) {
            Verified::Composes => {}
            Verified::Differs => {
                return Some(Refusal {
                    reason: REFUSE_DERIVATION,
                    at: sentence::At::Vertex { address: address.clone(), chart: class.clone() },
                })
            }
            // The door (`hear_sentence`) decides the WAIT before any
            // replay runs, so an unruled cited FORM never reaches here.
            // What reaches here is a citation naming a vertex the ruled
            // form does not carry — the form resolved and the vertex was
            // not in it — which is a claim about the tape and a refusal
            // at the vertex that made it, not a deferral.
            Verified::Unresolved { .. } => {
                return Some(Refusal {
                    reason: REFUSE_DERIVATION,
                    at: sentence::At::Vertex { address: address.clone(), chart: class.clone() },
                })
            }
        }
    }
    None
}

/// 37.7 P2 — EVERY `RotorPrime` DEMAND CARRIES ITS PRIME. The witness is
/// the residue block the shells left (50.5 as amended); `π_p =
/// gcd_{ℤ[i]}(s + t·i, p)` must have norm `p`, or the demand names a
/// prime no witness of it carries and there is nothing to open. Refused
/// at the demand vertex, addressed by the demand's own key — which is
/// its coordinates written out (A21), so the coordinate is content.
fn witnesses_carry_their_primes(f: &Form) -> Option<Refusal> {
    use crate::transducer::sentence::Demand;
    for (_, kind, chart, _) in f.demand_vertices() {
        let Demand::RotorPrime { prime, witness, .. } = kind else {
            continue;
        };
        let carries = crate::core::gaussian::from_block(witness).is_some_and(|w| {
            crate::core::gaussian::prime_factor(&w, &num_bigint::BigInt::from(*prime)).is_some()
        });
        if !carries {
            return Some(Refusal {
                reason: REFUSE_LETTER_WITNESS,
                at: sentence::At::Vertex { address: kind.key(), chart: chart.clone() },
            });
        }
    }
    None
}

/// The same replay, refusing WITH THE COORDINATE (50.9): the vertex that
/// does not carry its body, the face whose edge does not re-derive, the
/// chart with the links no demand declares; every other refusal is of the
/// whole tape.
pub fn replay_form_at(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<Refusal> {
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    // A SPOKEN word under replay: the court re-derives it from the tape it
    // was sent, which is the point of a replay, so the decode is named
    // (L16).
    let Some(f) = crate::transducer::sentence::from_element_form(spoken) else {
        return Some(tape(REFUSE_FORM_TAPE));
    };
    let f = &f;
    if f.depth != f.depth_of() {
        return Some(tape(REFUSE_GLUED_DEPTH));
    }
    for v in &f.vertices {
        let Vertex::Glued { address, depth, .. } = v else {
            continue;
        };
        if *depth >= f.depth {
            return Some(tape(REFUSE_GLUED_DEPTH));
        }
        let Some(cited) = resolve(address) else {
            return Some(tape(REFUSE_GLUED_UNRULED));
        };
        if !cited.is_closed() {
            return Some(tape(REFUSE_GLUED_UNRULED));
        }
        if cited.depth_of() != *depth {
            return Some(tape(REFUSE_GLUED_DEPTH));
        }
        // 18: A FORM OPENS AT ITS FRAME. Vertex zero is the frame by the
        // form's own shape, so the slice pattern reads the coordinate and
        // not the vertex that happened to be written first.
        let frame_is_identity = match cited.vertices.as_slice() {
            [fr @ Vertex::Frame, ..] => fr.word_in(&cited).is_some_and(|w| w.is_identity()),
            _ => false,
        };
        if !frame_is_identity {
            return Some(tape(REFUSE_GLUED_FRAME_GAUGED));
        }
    }
    // 50.2 step 2 — G COMPOSES THE DERIVATION. Every word vertex names
    // the generators it factored through; the court resolves each on its
    // own ruled cover, composes them with the residue, and compares the
    // product projectively (1.5) against the word the vertex carries. A
    // citation the cover does not hold is NOT a refusal: it is the
    // deferral a `Glued` citation already takes above — the sentence is
    // docketed and the syllabus serves the generator (21 step 3), one
    // mechanism at two levels.
    if let Some(refusal) = derivations_compose(f, resolve) {
        return Some(refusal);
    }
    // 37.7 P2: a demand that names a prime must carry a witness that
    // carries it. Read before anything is docketed, so `list_demands`
    // never opens a letter on a witness the arithmetic refuses.
    if let Some(refusal) = witnesses_carry_their_primes(f) {
        return Some(refusal);
    }
    // 23.1 step 3, 50.6 (L7): a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape — nothing in it acts.
    if !f.acts() {
        return Some(tape(REFUSE_NOT_A_SENTENCE));
    }
    if let Some(refusal) = vertex_tape_and_moves(f, family, spoken) {
        return Some(refusal);
    }
    // G steps 2–3 (50.2) on the court's copy of the family: every word
    // vertex carries its body onto its image, exactly (L7).
    if let Some(fam) = family {
        if let Some(refusal) = vertices_carry_family(f, fam) {
            return Some(refusal);
        }
    }
    // Loops are derived, never trusted from the tape: re-walk the forest.
    let Some(taped_rank) = f.rank() else {
        return Some(tape(REFUSE_RANK_LOST));
    };
    let mut walked = f.clone();
    walked.loops = Form::loops_of(&f.faces, &f.vertices);
    let by_chord = chord_loops(&walked);
    // Chapter 23.1 step 4: a face equals face_of(i, j), or it is the
    // Kruskal chord of a loop whose holonomy is what the chord carries
    // (tree faces of that loop are face_of). Filing of that class is
    // the loop check; a missing torsion entry is REFUSE_LOOP, a chord
    // that does not carry the holonomy is REFUSE_FACE.
    for (i, face) in f.faces.iter().enumerate() {
        // 50.3, 52.3: an edge transport of any order is a face. What it
        // carries is re-derived on the family below (`edges_rederive_family`),
        // and a period is never a gate.
        // THE FACE RE-DERIVES AT EVERY MEMBER PAIR IT CARRIES. The word is the
        // face's and re-derives once; the placement and hue map at each pair
        // re-derive from the two members that pair names, and a face whose
        // pairs the vertices no longer relate does not re-derive at all.
        if face_rederives(f, face) {
            continue;
        }
        if !torsion_chord_carries(&walked, &by_chord, i) {
            return Some(face_at(f, face, spoken));
        }
    }
    // 23.1 step 4, 50.3 (L7): every carried edge re-measured on the union
    // of its two bodies, on every pair of the family. BEFORE the loops,
    // because an edge that does not re-derive is a defect at a FACE and
    // the reading names it there; the loop reading over that same edge
    // would answer second-hand, as an unfiled class.
    if let Some(fam) = family {
        if let Some(refusal) = edges_rederive_family(&walked, fam) {
            return Some(refusal);
        }
    }
    if !form_loops_replay(&walked) {
        return Some(tape(REFUSE_LOOP));
    }
    let Some(derived) = walked.rank() else {
        return Some(tape(REFUSE_RANK_LOST));
    };
    // S9.2 — THE RANK RE-DERIVES OR THE WORD IS REFUSED. The court walks
    // the forest itself and compares the audit it derived against the one
    // the tape claims, coordinate by coordinate, and demands EQUALITY.
    //
    // A clause that refused only a tape claiming LESS than the walk on
    // some coordinate would pass a tape claiming MORE: structure the court
    // cannot derive, admitted because it points the convenient way. Rule
    // 11 is exact or refused, and a rank that does not re-derive is not
    // exact in either direction.
    if crate::transducer::sentence::Rank::moved(taped_rank, derived)
        != crate::transducer::sentence::RankMove::Equal
    {
        return Some(tape(REFUSE_RANK_LOST));
    }
    if let Some(fam) = family {
        // `Form.glued` is keyed by the cited address and `reindex` builds it,
        // so "does this form cite anything" is a read of that index and not a
        // walk of every vertex asking each whether it is a citation.
        let glued = !walked.glued.is_empty();
        let deflation_crystal: &dyn crate::transducer::family::Crystal = if glued {
            crystal
        } else {
            &crate::transducer::family::NoCrystal
        };
        let (pay, rems) =
            crate::transducer::family::form_deflation(&walked, fam, deflation_crystal);
        // S9.4 — CLOSURE IS THE CERTIFICATE. A remainder on any pair means
        // the form does not close the family it claims, and that is the
        // refusal. The μ⃗ it measured is NOT: a form that closes every pair
        // while measuring nothing has still closed them, and pricing is the
        // economy's reading, taken after the court's. Under 50.5 (L7) the
        // clause reads the CLOSED SUB-COMPLEX: a remainder link a demand
        // vertex names is what the sentence said it could not close — valid,
        // not closed — and only an undeclared remainder refuses, at the
        // chart of the links it read (50.9).
        let _ = pay;
        let undeclared = undeclared_remainder(&rems, &walked);
        if !undeclared.is_empty() {
            let chart = stalk::invariant(&undeclared);
            return Some(Refusal { reason: REFUSE_NOT_CLOSED, at: sentence::At::Chart(chart, undeclared) });
        }
    }
    replay_certificates(&walked, &by_chord).map(tape)
}

/// `REFUSE_FACE` at the face's two vertex addresses (50.9); a face reaching
/// a vertex with no address is refused at the tape.
fn face_at(f: &Form, face: &crate::transducer::sentence::FormFace, spoken: &Element) -> Refusal {
    let addr_of = |i: u16| match f.vertices.get(i as usize) {
        Some(Vertex::Word { address, .. }) | Some(Vertex::Glued { address, .. }) => Some(address.clone()),
        _ => None,
    };
    match (addr_of(face.i), addr_of(face.j)) {
        (Some(a), Some(b)) => Refusal { reason: REFUSE_FACE, at: sentence::At::Face(a, b) },
        _ => Refusal::of_tape(REFUSE_FACE, spoken),
    }
}

/// Chapter 23.1 steps 3 and 6: each Word vertex re-derives its rotor (M1)
/// and its affine; a Rebased whose word is the identity equals its base;
/// an Unfolded Remainder with nothing left does not move; an orbit whose
/// word is identity does not move. With a family, unfold and orbit execute
/// on the trains. The orbit's refusal is at the contact vertex with the
/// chart it acts on (50.9); every other refusal here is of the tape.
pub(crate) fn vertex_tape_and_moves(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    spoken: &Element,
) -> Option<Refusal> {
    use crate::transducer::family::{self, OrbitEnd};
    use crate::transducer::sentence::Measure;
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    for v in &f.vertices {
        match v {
            Vertex::Word { address, derivation, contact, .. } => {
                // EVERY MEMBER'S CARRIAGE must give a conformal versor, or
                // the tape carries a placement nothing can be spoken at.
                // One member today; the walk is the vertex's own fibre.
                let Some(acts) = v.acts() else {
                    return Some(tape(REFUSE_VERTEX_TAPE));
                };
                for (_, a) in acts.scoped() {
                    if element::conformal_of_rotor(&derivation.rotor(), &a.carriage).is_none() {
                        return Some(tape(REFUSE_VERTEX_TAPE));
                    }
                }
                if contact.is_some() {
                    let still = || match v.face_chart() {
                        Some(chart) => Refusal {
                            reason: REFUSE_ORBIT_STILL,
                            at: sentence::At::Vertex { address: address.clone(), chart },
                        },
                        None => tape(REFUSE_ORBIT_STILL),
                    };
                    if derivation.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
                        return Some(still());
                    }
                    if let Some(fam) = family {
                        for (x, _) in fam {
                            let painted = crate::arc::painted_of_grid(x);
                            // A grid with no rows has no width.
                            let frame = (x.len(), crate::arc::grid_width(x));
                            let anchor: Vec<(i64, i64)> = painted.iter().map(|&(r, c, _)| (r, c)).collect();
                            if matches!(family::orbit(v, &painted, &anchor, frame), OrbitEnd::NoOrbit) {
                                return Some(still());
                            }
                        }
                    }
                }
            }
            Vertex::Rebased { .. }
                if v.word_in(f).is_some_and(|w| w.is_identity()) => {
                    return Some(tape(REFUSE_REBASE_TRIVIAL));
                }
            Vertex::Unfolded { measure: Measure::Remainder } if f.remainder.is_empty() => {
                return Some(tape(REFUSE_UNFOLD_STILL));
            }
            Vertex::Unfolded { measure: Measure::Occupied } => {
                if let Some(fam) = family {
                    for (x, y) in fam {
                        if x == y {
                            return Some(tape(REFUSE_UNFOLD_STILL));
                        }
                    }
                }
            }
            // 50.5: a demand vertex names a kind that carries its
            // coordinates, or it is not a demand.
            Vertex::Demand { kind, .. } => {
                if let Err(why) = kind.typed() {
                    return Some(tape(why));
                }
            }
            _ => {}
        }
    }
    None
}

pub(crate) fn chord_loops(f: &Form) -> Keyed<u16> {
    let mut by_chord = Keyed::new();
    for (i, lp) in f.loops.iter().enumerate() {
        by_chord.insert(face_idx_key(lp.chord), i as u16);
    }
    by_chord
}

/// 23.1 step 4 — A FACE THAT IS NOT `face_of` IS THE KRUSKAL CHORD of a
/// loop whose every other face is. The pivot writes the chord's transport
/// and nothing else, so the chord is exactly where a form's own word
/// differs from the coboundary of its two vertices.
///
/// THE CHORD IS A FACE LIKE ANY OTHER and carries its edge like any other.
/// What the loop then reads — flat, filed at its period, or not evaluated
/// at all — is [`form_loops_replay`]'s, on the loop's EDGE holonomy (50.3).
/// Asking for the TRANSPORT composite here instead, matched against the
/// chord's own word, would be a reading that telescopes over words that
/// commute: it would certify a loop the voice never evaluated and demand a
/// filing under a class nothing files.
pub(crate) fn torsion_chord_carries(f: &Form, by_chord: &Keyed<u16>, face_idx: usize) -> bool {
    let Some(&li) = by_chord.get(&face_idx_key(face_idx as u16)) else {
        return false;
    };
    let Some(lp) = f.loops.get(li as usize) else {
        return false;
    };
    for &fi in &lp.faces {
        if fi == lp.chord {
            continue;
        }
        let Some(tf) = f.faces.get(fi as usize) else {
            return false;
        };
        if !face_rederives(f, tf) {
            return false;
        }
    }
    true
}

/// THE LOOPS THE FORM EVALUATED, each flat or filed at its period.
///
/// The reading is the loop's EDGE HOLONOMY (50.3) — the composite of the
/// transitions between the bodies' own frames — and it is the same reader
/// the voice classifies with. The TRANSPORT composite is a different
/// object: it telescopes around a cycle over words that commute, and every
/// word a pair form composes is a sensor-plane word, so read on it a
/// cover's loop is flat however curved the scene is, while the class the
/// voice actually files — under the edge holonomy's key — is never looked
/// for.
///
/// A LOOP THE FORM DID NOT EVALUATE IS NOT REPLAYED. `edge_holonomy`
/// answers `None` where a remainder edge lies anywhere on the cycle (13.9
/// step 7), and a loop with no transition to read carries no obligation
/// (rule 11: that is a refusal to read, not a flat reading).
pub(crate) fn form_loops_replay(f: &Form) -> bool {
    for lp in &f.loops {
        // THE TRANSPORT COMPOSITE — the pivot's obstruction, discharged on
        // an axis or filed under its own class (12.6, A17).
        if !loop_reading_replays(f, lp, &lp.holonomy) {
            return false;
        }
        // THE EDGE HOLONOMY — the cover's (50.3). A loop the form did not
        // evaluate, a remainder edge anywhere on it, answers `None` and
        // carries no obligation: that is a refusal to read (13.9 step 7),
        // not a flat reading, and `pair.rs` classifies nothing there either.
        // EVERY EVALUATED MEMBER PATH must replay its own reading. A loop
        // running through two pairs of bodies carries an obligation at each,
        // and one path replaying said nothing about the others.
        let Some(es) = f.edge_holonomies(lp) else {
            continue;
        };
        for (_, e) in es.scoped() {
            if !loop_reading_replays(f, lp, &e.word) {
                return false;
            }
        }
    }
    true
}

/// ONE READING OF ONE LOOP: flat, the transition on an axis the form
/// opened, or filed as torsion under its own class at its own period.
///
/// It is called twice per loop because a loop HAS two obstructions and
/// neither replaces the other (`Form::edge_class_vector`): the transport
/// composite the pivot discharges, and the edge holonomy the cover files.
/// A court that read the first alone would never look for the class a
/// cover actually files — keyed by the edge holonomy — and the transport
/// composite telescopes over the sensor-plane words a pair form composes,
/// which read flat however curved the scene is.
fn loop_reading_replays(
    f: &Form,
    lp: &crate::transducer::sentence::Loop,
    word: &crate::transducer::alphabet::Word,
) -> bool {
    if word.is_identity() {
        return true;
    }
    // An axis the form opened over this loop (I6): the holonomy is the
    // transition on that axis, and it replays only when the recorded
    // seed re-derives from the loop itself (law 12).
    if let Some(chord) = f.opened_axis_of(lp) {
        return f.axis_rederives(lp, chord);
    }
    // Filed under its class at the period the court re-derives — `0` for
    // infinite order, A19's spelling of it (52.3). No class, no filing: a
    // holonomy that is not a versor is a loop that does not replay, never
    // one keyed by its bytes.
    let Some(k) = word.rotor().class_key() else {
        return false;
    };
    let ell = word.order().to_u32();
    matches!(f.torsion.get(k.as_bytes()), Some(filed) if filed.period == ell)
}

pub(crate) fn face_idx_key(i: u16) -> Vec<u8> {
    i.to_le_bytes().to_vec()
}

/// A20: each certificate is the un-pivot of one chord face, re-walked.
/// The loop's class key and rank before and after must match the tape.
pub(crate) fn replay_certificates(f: &Form, by_chord: &Keyed<u16>) -> Option<&'static str> {
    // ONE FORM'S certificates: the carrier hangs off the form being
    // replayed, and every certificate on it is re-walked or the tape is
    // refused -- there is no subset of them that would do.
    for (_, cert) in f.certificates.scoped() {
        let face = cert.by.face();
        let Some(&li) = by_chord.get(&face_idx_key(face)) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let mut pre = f.clone();
        pre.unapply_move(&cert.by);
        pre.loops = Form::loops_of(&pre.faces, &pre.vertices);
        // An opened axis (I6) is seeded by the loop's own holonomy versor
        // (16.20): the court re-derives the seed from the loop it re-walked
        // and refuses a certificate whose seed is anything else (law 12).
        if let crate::transducer::sentence::PivotBy::OpenAxis { seed, .. } = &cert.by {
            let rederived = pre
                .loops
                .get(li as usize)
                .and_then(|lp| pre.axis_seed_of(lp));
            if !rederived.is_some_and(|v| element::same_versor(&v, seed)) {
                return Some(REFUSE_PIVOT_INEXACT);
            }
        }
        let Some(kappa_before) = pre.class_vector(li as usize) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        if kappa_before.class_key() != cert.before {
            return Some(REFUSE_PIVOT_INEXACT);
        }
        let Some(kappa_after) = f.class_vector(li as usize) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        if kappa_after.class_key() != cert.after || kappa_after.occupies(cert.axis) {
            return Some(REFUSE_PIVOT_INEXACT);
        }
        let Some(rb) = pre.rank() else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let Some(ra) = f.rank() else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        // S9.1 — THE COURT RE-DERIVES THE MOVE. The certificate claims what
        // the pivot did to the rank; the court walks the form before and
        // after for itself and computes the same move. The two absolute
        // audits it used to compare against are the court's own here, so
        // carrying them on the tape only invited a second comparison.
        if crate::transducer::sentence::Rank::moved(rb, ra) != cert.rank_move {
            return Some(REFUSE_PIVOT_INEXACT);
        }
    }
    None
}

/// Every torsion cycle the word speaks is walked from its base assignment.
/// A period is believed only as a witnessed closed loop; an open walk names
/// the anholonomity the label was filing over, and a closure at another
/// period is a winding the word misspoke. Replay was Δ = 0 on the chain;
/// this is ∮ dX = 0 on the fibre.
pub(crate) fn witness_torsion(spoken: &Element) -> Option<&'static str> {
    for (stored, witness) in torsion_witness::witness_stored(spoken) {
        match witness {
            HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
            HolonomicLoopWitness::Closed { .. } => return Some("torsion period does not re-derive — the winding was misspoken"),
            HolonomicLoopWitness::Open { .. } => return Some("torsion cycle does not close — the period was a filing"),
        }
    }
    None
}
