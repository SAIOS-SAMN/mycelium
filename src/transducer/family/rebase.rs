// 21 step 4 — THE REBASE: a vertex read in another's frame, where the whole
// bundle measures one relative placement between them.
//
//! Split from `extract.rs` at the §1.1 1200-line reading bound; the item is
//! moved, not rewritten (§1.6). It is step 4 of chapter 21's seven moves and
//! `extract_rule_with` calls it between the glue and the unfold.

use super::*;

/// Step 4 of chapter 21: along each named face (i, j) with `w_i` not the
/// identity, `j` is `i` read in `j`'s frame when, on every pair of the
/// bundle, the two bodies carry one linear word and differ only by their
/// relative placement `t = p_j − p_i`. Conjugation by a translation keeps
/// the linear block and moves the centre, so `t · w_i · t⁻¹` is realised
/// by `apply_form` as `w_i` at `p_i + t`; nothing is spelled for `t`.
pub(crate) fn rebase_named_faces(form: &mut Form) {
    use crate::network::nerve::coord::{ChartAt, Overlap, VertexAt};
    use crate::network::nerve::Coordinate;
    // THE OVERLAPS AS THEY STAND, taken once and grouped by the ordered pair
    // of vertices they join: the rebase moves vertices to new keys and adds
    // no overlap, so the pairs are read from this copy while the vertices
    // move under them. A pair is the old face — every member pair and every
    // frame the two vertices overlap at.
    let mut pairs: crate::network::nerve::Keyed<(VertexAt, VertexAt, Vec<Overlap>)> = crate::network::nerve::Keyed::new();
    for (o, _) in form.faces.entries() {
        let (ChartAt::At(_), ChartAt::At(_)) = (&o.from, &o.to) else { continue };
        let (a, b) = (o.from_vertex(), o.to_vertex());
        let mut k = Vec::new();
        crate::transducer::address::put_key_bytes(&mut k, a.encode().as_bytes());
        crate::transducer::address::put_key_bytes(&mut k, b.encode().as_bytes());
        match pairs.get_mut(&k) {
            Some((_, _, os)) => os.push(o),
            None => {
                pairs.insert(&k, (a, b, vec![o]));
            }
        }
    }
    let pairs: Vec<(VertexAt, VertexAt, Vec<Overlap>)> = pairs.scoped().map(|(_, p)| p.clone()).collect();
    // THE RELATIVES OF EVERY CLASS PAIR, read off the overlaps once before
    // any vertex is rebased, and read below by key per pair (rule 1).
    let relatives = relatives_by_class(&form.faces, &form.vertices);
    for (i, j, overlaps) in pairs {
        // A vertex already rebased has left its key; the pair no longer
        // stands, and a vertex that is not a Word is no frame to read in.
        let (Some(vi), Some(vj)) = (form.vertices.at(&i), form.vertices.at(&j)) else {
            continue;
        };
        let (Vertex::Word { derivation: di, .. }, Vertex::Word { .. }) = (vi, vj) else {
            continue;
        };
        let di = di.clone();
        if di.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
            continue;
        }
        let Some(tc_i) = transport_class(vi) else {
            continue;
        };
        let Some(tc_j) = transport_class(vj) else {
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
        for o in &overlaps {
            let (ChartAt::At(x), ChartAt::At(y)) = (&o.from, &o.to) else { continue };
            let (Some(a), Some(b)) = (vi.acting_at(&x.chart, x.frame), vj.acting_at(&y.chart, y.frame)) else {
                agree = false;
                break;
            };
            // `b ã`, the relative placement as a ray (78.32(1)).
            if !crate::transducer::cga::is_versor(&a.carriage) {
                agree = false;
                break;
            }
            let here = b.carriage.geo(&a.carriage.reverse());
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
        // The chord is the first overlap this pair crossed, in key order: it
        // is passed and never searched for (15.5).
        let [chord, ..] = overlaps.as_slice() else { continue };
        if constant {
            let _ = form.rebase(&i, &j, relative, chord);
            trace::trace("extract/4 rebased", trace::Wrote::Nothing, None);
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
    // THE STAR IS THE FRAME'S FIBRE: every overlap leaving the Frame's
    // chart, read at its tag (AT1). Each keeps its edge, its producer and its
    // interface — WHICH charts it joins is the scene's fact — and its place
    // is re-derived at its own coordinate; the word is derived on read and
    // has nothing to re-derive.
    for o in form.overlaps_from(&crate::network::nerve::coord::VertexAt::Frame) {
        trace::trace("extract/4 star-face", trace::Wrote::Nothing, None);
        let Some(edge) = form.faces.at(&o).map(|f| f.place.edge.clone()) else { continue };
        let Some(p) = form.face_place(&o.from, &o.to, edge) else { continue };
        if let Some(f) = form.faces.at_mut(&o) {
            let refraction = f.place.refraction.clone();
            f.place = p;
            f.place.refraction = refraction;
        }
    }
}
