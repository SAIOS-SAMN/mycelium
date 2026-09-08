// 23, 42 — REPLAY: a spoken form is re-derived from its own tape, move by
// move, against the cover that resolves what it cites.
//
//! Every vertex re-derives from its tape, every face from its vertices,
//! every loop's holonomy to the identity, and every pivot certificate from
//! the axis it names. A form that does not is REFUSED with the move that
//! failed, never scored.

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
/// with no Glued vertex.
pub fn replay_form_with(
    spoken: &Element,
    resolve: &dyn Fn(&[u8]) -> Option<Form>,
    family: Option<&[(crate::arc::Grid, crate::arc::Grid)]>,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<&'static str> {
    let Some(f) = sentence::from_element_form(spoken) else {
        return Some(REFUSE_FORM_TAPE);
    };
    if f.depth != f.depth_of() {
        return Some(REFUSE_GLUED_DEPTH);
    }
    for v in &f.vertices {
        let Vertex::Glued { address, depth, .. } = v else {
            continue;
        };
        if *depth >= f.depth {
            return Some(REFUSE_GLUED_DEPTH);
        }
        let Some(cited) = resolve(address) else {
            return Some(REFUSE_GLUED_UNRULED);
        };
        if !cited.is_closed() {
            return Some(REFUSE_GLUED_UNRULED);
        }
        if cited.depth_of() != *depth {
            return Some(REFUSE_GLUED_DEPTH);
        }
        let frame_is_identity = match cited.vertices.first() {
            Some(fr @ Vertex::Frame) => fr.word_in(&cited).is_some_and(|w| w.is_identity()),
            _ => false,
        };
        if !frame_is_identity {
            return Some(REFUSE_GLUED_FRAME_GAUGED);
        }
    }
    if let Some(reason) = vertex_tape_and_moves(&f, family) {
        return Some(reason);
    }
    // Loops are derived, never trusted from the tape: re-walk the forest.
    let Some(taped_rank) = f.rank() else {
        return Some(REFUSE_RANK_LOST);
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
        if f.face_of(face.i, face.j).as_ref() == Some(&face.transport) {
            continue;
        }
        if !torsion_chord_carries(&walked, &by_chord, i, face) {
            return Some(REFUSE_FACE);
        }
    }
    if !form_loops_replay(&walked) {
        return Some(REFUSE_LOOP);
    }
    let Some(derived) = walked.rank() else {
        return Some(REFUSE_RANK_LOST);
    };
    if taped_rank.torsion < derived.torsion
        || taped_rank.free < derived.free
        || taped_rank.offplane < derived.offplane
        || taped_rank.axes < derived.axes
    {
        return Some(REFUSE_RANK_LOST);
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
        if rems.iter().any(|r| !r.is_empty()) || !crate::economy::descends(&[0; 4], &pay) {
            return Some(REFUSE_NO_DESCENT);
        }
    }
    replay_certificates(&walked, &by_chord)
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
                word,
                carriage,
                contact,
                ..
            } => {
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
        match lp.holonomy.order() {
            Order::Infinite => return false,
            Order::Finite(ell) => {
                let k = lp.holonomy.rotor().class_or_rotor_key();
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
    use crate::transducer::sentence::{rank_of_holonomies, unapply_by};
    for (_, cert) in f.certificates.fiber(crate::network::nerve::ROOT) {
        let face = cert.by.face();
        let Some(&li) = by_chord.get(&face_idx_key(face)) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let mut pre = f.clone();
        unapply_by(&mut pre.faces, &cert.by);
        pre.loops = Form::loops_of(&pre.faces, pre.vertices.len());
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
        let Some(rb) = rank_of_holonomies(pre.loops.iter().map(|l| &l.holonomy)) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        let Some(ra) = rank_of_holonomies(f.loops.iter().map(|l| &l.holonomy)) else {
            return Some(REFUSE_PIVOT_INEXACT);
        };
        if rb != cert.rank_before || ra != cert.rank_after {
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
    for (stored, witness) in toroid::witness_stored(spoken) {
        match witness {
            HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
            HolonomicLoopWitness::Closed { .. } => return Some("torsion period does not re-derive — the winding was misspoken"),
            HolonomicLoopWitness::Open { .. } => return Some("torsion cycle does not close — the period was a filing"),
        }
    }
    None
}
