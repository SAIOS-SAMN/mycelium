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
fn derivations_compose(f: &Form, resolve: &dyn Fn(&[u8]) -> Option<Form>) -> Option<Refusal> {
    use crate::transducer::alphabet::Verified;
    for v in &f.vertices {
        let Vertex::Word { address, derivation, class, .. } = v else {
            continue;
        };
        let held = |form: &crate::transducer::address::Address,
                    vertex: &crate::transducer::address::Address| {
            let cited = resolve(form.as_bytes())?;
            sentence::word_at_vertex(&cited, vertex.as_bytes())
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

/// The same replay, refusing WITH THE COORDINATE (50.9): the vertex that
/// does not carry its body, the face whose edge does not re-derive, the
/// chart with the cells no demand declares; every other refusal is of the
/// whole tape.
pub fn replay_form_at(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<Refusal> {
    let tape = |reason: &'static str| Refusal::of_tape(reason, spoken);
    let Some(f) = sentence::from_element_form(spoken) else {
        return Some(tape(REFUSE_FORM_TAPE));
    };
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
        let frame_is_identity = match cited.vertices.first() {
            Some(fr @ Vertex::Frame) => fr.word_in(&cited).is_some_and(|w| w.is_identity()),
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
    if let Some(refusal) = derivations_compose(&f, resolve) {
        return Some(refusal);
    }
    // 23.1 step 3, 50.6 (L7): a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape — nothing in it acts.
    if !f.acts() {
        return Some(tape(REFUSE_NOT_A_SENTENCE));
    }
    if let Some(reason) = vertex_tape_and_moves(&f, family) {
        return Some(tape(reason));
    }
    // G steps 2–3 (50.2) on the court's copy of the family: every word
    // vertex carries its body onto its image, exactly (L7).
    if let Some(fam) = family {
        if let Some(refusal) = vertices_carry_family(&f, fam) {
            return Some(refusal);
        }
    }
    // Loops are derived, never trusted from the tape: re-walk the forest.
    let Some(taped_rank) = f.rank() else {
        return Some(tape(REFUSE_RANK_LOST));
    };
    let mut walked = f.clone();
    walked.loops = Form::loops_of(&f.faces, f.vertices.len());
    let by_chord = chord_loops(&walked);
    // Chapter 23.1 step 4: a face equals face_of(i, j), or it is the
    // Kruskal chord of a loop whose holonomy is what the chord carries
    // (tree faces of that loop are face_of). Filing of that class is
    // the loop check; a missing torsion entry is REFUSE_LOOP, a chord
    // that does not carry the holonomy is REFUSE_FACE.
    for (i, face) in f.faces.iter().enumerate() {
        // 50.3: an edge transport is a stabilizer element or an exchange
        // of lattice bodies, both of finite order on the word; a word of
        // infinite order is neither (the membership on the bodies is read
        // with the family, L7).
        if let Some(edge) = &face.edge {
            if matches!(edge.word.order(), crate::transducer::alphabet::Order::Infinite) {
                return Some(face_refusal(&f, face));
            }
        }
        if f.face_of(face.i, face.j).as_ref() == Some(&face.transport) {
            continue;
        }
        if !torsion_chord_carries(&walked, &by_chord, i, face) {
            return Some(tape(REFUSE_FACE));
        }
    }
    if !form_loops_replay(&walked) {
        return Some(tape(REFUSE_LOOP));
    }
    // 23.1 step 4, 50.3 (L7): every carried edge re-measured on the union
    // of its two bodies, on every pair of the family.
    if let Some(fam) = family {
        if let Some(refusal) = edges_rederive_family(&walked, fam) {
            return Some(refusal);
        }
    }
    let Some(derived) = walked.rank() else {
        return Some(tape(REFUSE_RANK_LOST));
    };
    // S9.2 — THE RANK RE-DERIVES OR THE WORD IS REFUSED. The court walks
    // the forest itself and compares the audit it derived against the one
    // the tape claims, coordinate by coordinate, and demands EQUALITY.
    //
    // The old clause refused only a tape claiming LESS than the walk on
    // some coordinate, so a tape claiming MORE passed: structure the court
    // could not derive, admitted because it pointed the convenient way.
    // Rule 11 is exact or refused, and a rank that does not re-derive is
    // not exact in either direction.
    if taped_rank != derived {
        return Some(tape(REFUSE_RANK_LOST));
    }
    if let Some(fam) = family {
        let glued = walked
            .vertices
            .iter()
            .any(|v| matches!(v, Vertex::Glued { .. }));
        let descent_crystal: &dyn crate::transducer::family::Crystal = if glued {
            crystal
        } else {
            &crate::transducer::family::NoCrystal
        };
        let (pay, rems) = crate::transducer::family::form_descent(&walked, fam, descent_crystal);
        // S9.4 — CLOSURE IS THE CERTIFICATE. A remainder on any pair means
        // the form does not close the family it claims, and that is the
        // refusal. The μ⃗ it measured is NOT: a form that closes every pair
        // while measuring nothing has still closed them, and pricing is the
        // economy's reading, taken after the court's. Under 50.5 (L7) the
        // clause reads the CLOSED SUB-COMPLEX: a remainder cell a demand
        // vertex names is what the sentence said it could not close — valid,
        // not closed — and only an undeclared remainder refuses, at the
        // chart of the cells it read (50.9).
        let _ = pay;
        let undeclared = undeclared_remainder(&rems, &walked);
        if !undeclared.is_empty() {
            let chart = stalk::invariant(&undeclared);
            return Some(Refusal { reason: REFUSE_NOT_CLOSED, at: sentence::At::Chart(chart, undeclared) });
        }
    }
    replay_certificates(&walked, &by_chord).map(tape)
}

/// `REFUSE_EDGE` at a face's two vertex addresses; a face reaching a vertex
/// with no address is refused at the tape.
fn face_refusal(f: &Form, face: &crate::transducer::sentence::FormFace) -> Refusal {
    let addr_of = |i: u16| match f.vertices.get(i as usize) {
        Some(Vertex::Word { address, .. }) | Some(Vertex::Glued { address, .. }) => Some(address.clone()),
        _ => None,
    };
    match (addr_of(face.i), addr_of(face.j)) {
        (Some(a), Some(b)) => Refusal { reason: REFUSE_EDGE, at: sentence::At::Face(a, b) },
        _ => Refusal { reason: REFUSE_EDGE, at: sentence::At::Tape(sentence::form_vertex_key(f)) },
    }
}

/// Chapter 23.1 steps 3 and 6: each Word vertex re-derives its rotor (M1)
/// and its affine; a Rebased whose word is the identity equals its base;
/// an Unfolded Remainder with nothing left does not move; an orbit whose
/// word is identity does not move. With a family, unfold and orbit execute
/// on the trains.
pub(crate) fn vertex_tape_and_moves(
    f: &Form,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
) -> Option<&'static str> {
    use crate::transducer::family::{self, OrbitEnd};
    use crate::transducer::sentence::Measure;
    for v in &f.vertices {
        match v {
            Vertex::Word {
                derivation,
                carriage,
                contact,
                ..
            } => {
                let word = derivation.word();
                if element::conformal_of(word, carriage).is_none() {
                    return Some(REFUSE_VERTEX_TAPE);
                }
                if contact.is_some() {
                    if word.is_identity() {
                        return Some(REFUSE_ORBIT_STILL);
                    }
                    if let Some(fam) = family {
                        for (x, _) in fam {
                            let cells: Vec<_> = family::perceive(x).into_iter().flat_map(|b| b.cells).collect();
                            let frame = (x.len(), x.first().map(|r| r.len()).unwrap_or(0));
                            let anchor: Vec<(i64, i64)> = cells.iter().map(|&(r, c, _)| (r, c)).collect();
                            if matches!(family::orbit(v, &cells, &anchor, frame), OrbitEnd::NoOrbit) {
                                return Some(REFUSE_ORBIT_STILL);
                            }
                        }
                    }
                }
            }
            Vertex::Rebased { .. }
                if v.word_in(f).is_some_and(|w| w.is_identity()) => {
                    return Some(REFUSE_REBASE_TRIVIAL);
                }
            Vertex::Unfolded { measure: Measure::Remainder } if f.remainder.is_empty() => {
                return Some(REFUSE_UNFOLD_STILL);
            }
            Vertex::Unfolded { measure: Measure::Occupied } => {
                if let Some(fam) = family {
                    for (x, y) in fam {
                        if x == y {
                            return Some(REFUSE_UNFOLD_STILL);
                        }
                    }
                }
            }
            // 50.5: a demand vertex names a kind that carries its
            // coordinates, or it is not a demand.
            Vertex::Demand { kind, .. } => {
                if let Err(why) = kind.typed() {
                    return Some(why);
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

/// The face is the last face of a non-identity loop, every other face of
/// that loop is `face_of`, and the loop holonomy is the chord's word.
pub(crate) fn torsion_chord_carries(
    f: &Form,
    by_chord: &Keyed<u16>,
    face_idx: usize,
    face: &crate::transducer::sentence::FormFace,
) -> bool {
    let Some(&li) = by_chord.get(&face_idx_key(face_idx as u16)) else {
        return false;
    };
    let Some(lp) = f.loops.get(li as usize) else {
        return false;
    };
    if lp.holonomy.is_identity() {
        return false;
    }
    for &fi in &lp.faces {
        if fi == lp.chord {
            continue;
        }
        let Some(tf) = f.faces.get(fi as usize) else {
            return false;
        };
        if f.face_of(tf.i, tf.j).as_ref() != Some(&tf.transport) {
            return false;
        }
    }
    lp.holonomy.same_form(&face.transport.word)
        || lp.holonomy.same_form(&face.transport.word.inverse())
}

/// Identity loops, or finite open loops filed on the form's torsion carrier.
pub(crate) fn form_loops_replay(f: &Form) -> bool {
    use crate::transducer::alphabet::Order;
    for lp in &f.loops {
        if lp.holonomy.is_identity() {
            continue;
        }
        // An axis the form opened over this loop (I6): the holonomy is the
        // transition on that axis, and it replays only when the recorded
        // seed re-derives from the loop itself (law 12).
        if let Some(chord) = f.opened_axis_of(lp) {
            if f.axis_rederives(lp, chord) {
                continue;
            }
            return false;
        }
        match lp.holonomy.order() {
            Order::Infinite => return false,
            Order::Finite(ell) => {
                // No class, no filing: a holonomy that is not a versor is a
                // loop that does not replay, never one keyed by its bytes.
                let Some(k) = lp.holonomy.rotor().class_key() else {
                    return false;
                };
                match f.torsion.get(k.as_bytes()) {
                    Some(filed) if filed.period == ell => {}
                    _ => return false,
                }
            }
        }
    }
    true
}

pub(crate) fn face_idx_key(i: u16) -> Vec<u8> {
    i.to_le_bytes().to_vec()
}

/// A20: each certificate is the un-pivot of one chord face, re-walked.
/// The loop's class key and rank before and after must match the tape.
pub(crate) fn replay_certificates(f: &Form, by_chord: &Keyed<u16>) -> Option<&'static str> {
    for (_, cert) in f.certificates.fiber(crate::network::nerve::ROOT) {
        let face = cert.by.face();
        let Some(&li) = by_chord.get(&face_idx_key(face)) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let mut pre = f.clone();
        pre.unapply_move(&cert.by);
        pre.loops = Form::loops_of(&pre.faces, pre.vertices.len());
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
