// 21 step 4 — THE REBASE: a vertex read in another's frame, where the whole
// bundle measures one relative placement between them.
//
//! Split from `extract.rs` at the §1.1 1200-line reading bound; the item is
//! moved, not rewritten (§1.6). It is step 4 of chapter 21's seven moves and
//! `extract_rule_with` calls it between the glue and the unfold.

use super::*;
use crate::transducer::sentence::vertex_ix_key;

/// Step 4 of chapter 21: along each named face (i, j) with `w_i` not the
/// identity, `j` is `i` read in `j`'s frame when, on every pair of the
/// bundle, the two bodies carry one linear word and differ only by their
/// relative placement `t = p_j − p_i`. Conjugation by a translation keeps
/// the linear block and moves the centre, so `t · w_i · t⁻¹` is realised
/// by `apply_form` as `w_i` at `p_i + t`; nothing is spelled for `t`.
pub(crate) fn rebase_named_faces(form: &mut Form) {
    // THE FACES AS THEY STAND, taken once: the rebase writes vertices and
    // adds no face, so the faces are read from this copy while the vertices
    // move under them.
    let named: Vec<FormFace> = form.faces.clone();
    // THE RELATIVES OF EVERY CLASS PAIR, read off the faces once before any
    // vertex is rebased, and read below by key per face (rule 1).
    let relatives = relatives_by_class(&named, &form.vertices);
    // The face's own index IS the chord it closes on where it closes: this
    // is a clone of `form.faces` and the rebase adds no face, so the two
    // agree throughout. A caller that walked a face knows which one; it is
    // passed and never searched for (15.5).
    for (fi, face) in named.iter().enumerate() {
        let (i, j) = (face.i as usize, face.j as usize);
        if matches!(form.vertices.get(j), Some(Vertex::Rebased { .. })) {
            continue;
        }
        let (Some(vi), Some(vj)) = (form.vertices.get(i), form.vertices.get(j)) else {
            continue;
        };
        let (Vertex::Word { derivation: di, .. }, Vertex::Word { .. }) = (vi, vj) else {
            continue;
        };
        let di = di.clone();
        if di.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
            continue;
        }
        let Some(tc_i) = transport_class(&form.vertices[i]) else {
            continue;
        };
        let Some(tc_j) = transport_class(&form.vertices[j]) else {
            continue;
        };
        // THE RELATIVE IS READ PER CHART PAIR, at the pairs THIS FACE relates
        // (D10 item 9). It was `vi.carriage_of()` against `vj.carriage_of()` —
        // two representatives, which on a face between multi-chart vertices
        // compares one body's placement against a body on a different chart and
        // calls the difference the relative.
        //
        // A rebase asserts ONE relative for the whole vertex, so it fires only
        // where every pair this face relates agrees on it AND every pair form
        // measures that same relative between the two classes. Where the pairs
        // disagree there is no one relative to rebase by (rule 11).
        let mut relative: Option<Versor> = None;
        let mut agree = true;
        for (pk, _) in face.at.scoped() {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(cci), Some(ccj)) = (vi.chart_keyed(&ik), vj.chart_keyed(&jk)) else {
                agree = false;
                break;
            };
            let (Some(a), Some(b)) = (vi.acting_on(&cci), vj.acting_on(&ccj)) else {
                agree = false;
                break;
            };
            let Some(inv) = a.carriage.versor_inverse() else {
                agree = false;
                break;
            };
            let here = b.carriage.geo(&inv);
            match &relative {
                None => relative = Some(here),
                Some(held) if !element::same_versor(held, &here) => {
                    agree = false;
                    break;
                }
                Some(_) => {}
            }
        }
        let (true, Some(relative)) = (agree, relative) else {
            continue;
        };
        // EVERY MEMBER PAIR MEASURES THE SAME RELATIVE, at every face this
        // form records between the two classes. The incidence is the face,
        // so the bodies compared are the bodies the scene relates and never
        // a product of the two classes' placements.
        let constant = relatives
            .get(&class_pair_key(&tc_i, &tc_j))
            .is_some_and(|rs| !rs.is_empty() && rs.scoped().all(|(_, (_, r))| element::same_versor(r, &relative)));
        if constant {
            let _ = form.rebase(face.i, face.j, relative, fi as u16);
            trace::trace("extract/4 rebased", trace::Wrote::Ordinal(face.j), None);
        }
    }
    trace::trace("extract/4 reindex", trace::Wrote::Nothing, None);
    form.reindex();
    rederive_star_faces(form);
}

/// THE STAR FACES RE-DERIVED FROM THEIR VERTICES after a move that moved a
/// vertex's placement — the rebase, the contact. The member pairs each face
/// already carries are kept with their edges: WHICH pairs a face relates is
/// the scene's fact, and the placement at each is re-derived at its own
/// coordinate; a face left as it was would carry a placement the vertex no
/// longer has, which the court refuses as a face that does not re-derive.
pub(crate) fn rederive_star_faces(form: &mut Form) {
    for (_, &fi) in form.star.scoped() {
        let Some(face) = form.faces.get(fi as usize) else {
            continue;
        };
        let (i, j) = (face.i, face.j);
        trace::trace("extract/4 star-face", trace::Wrote::Ordinal(j), None);
        // THE RE-DERIVATION KEEPS THE MEMBER PAIRS THE FACE ALREADY CARRIES
        // and keeps their edges: the rebase moved the vertices' placements, so
        // the carriage at each pair is stale, but WHICH pairs this face relates
        // is the scene's fact and re-deriving it from the vertices would take
        // the product of their fibres. The pairs are read off the face and the
        // placement at each is re-derived at its own coordinate.
        let pairs: Vec<(Vec<u8>, Option<crate::transducer::sentence::Transport>)> = face
            .at
            .scoped()
            .map(|(k, p)| (k, p.edge.clone()))
            .collect();
        let mut at = crate::network::nerve::Keyed::new();
        for (pk, edge) in pairs {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(ci), Some(cj)) = (
                form.vertices.get(i as usize).and_then(|v| v.chart_keyed(&ik)),
                form.vertices.get(j as usize).and_then(|v| v.chart_keyed(&jk)),
            ) else {
                continue;
            };
            if let Some(p) = form.face_place(i, &ci, j, &cj, edge) {
                at.insert(&pk, p);
            }
        }
        if let Some(word) = form.face_word(i, j) {
            form.faces[fi as usize].word = word;
            form.faces[fi as usize].at = at;
        }
    }
}
