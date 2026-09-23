// 18.2, 21 step 2, 50.3 — THE FACES OF A FORM, READ: the word between two
// vertices, what a face carries at each member pair, and the face a pair of
// vertices makes under a named word.

use super::*;

impl Form {

    /// THE WORD BETWEEN TWO VERTICES: `w_j ∘ w_i⁻¹`, read from their
    /// derivations. A vertex coordinate, so it is the same at every member
    /// pair of the face.
    pub fn face_word(&self, i: u16, j: u16) -> Option<Word> {
        let wa = self.vertices.get(i as usize)?.word_in(self)?;
        let wb = self.vertices.get(j as usize)?.word_in(self)?;
        Some(wb.compose(&wa.inverse()).normal_form())
    }

    /// WHAT THE FACE CARRIES AT ONE NAMED MEMBER PAIR: the relative carriage
    /// of the body on `cj` against the body on `ci` under the face's word, and
    /// the relative hue map between their two gauges. The edge is not derived
    /// here — 50.3's edge is measured on the union of the two bodies by
    /// whatever created the face, and re-deriving a placement must not erase
    /// it, so it is carried in by the caller.
    pub fn face_place(
        &self,
        i: u16,
        ci: &stalk::Invariant,
        j: u16,
        cj: &stalk::Invariant,
        edge: Option<Transport>,
    ) -> Option<FacePlace> {
        self.face_place_by(i, ci, j, cj, &self.face_word(i, j)?, edge)
    }

    /// The same placement UNDER A NAMED WORD. A section face (21 step 2, S12.2)
    /// carries a word the bundle spoke between these two ends that is NOT the
    /// coboundary of their placements, and its carriage is the relative under
    /// THAT word — computing it under the coboundary word would place the
    /// section by the face it is a section of.
    pub fn face_place_by(
        &self,
        i: u16,
        ci: &stalk::Invariant,
        j: u16,
        cj: &stalk::Invariant,
        word: &Word,
        edge: Option<Transport>,
    ) -> Option<FacePlace> {
        use crate::transducer::sentence::transport::{vertex_carriage_at, vertex_gauge_at};
        let va = self.vertices.get(i as usize)?;
        let vb = self.vertices.get(j as usize)?;
        let ca = vertex_carriage_at(va, self, ci)?;
        let cb = vertex_carriage_at(vb, self, cj)?;
        let wv = element::versor_of_word(word)?;
        let inv = wv.versor_inverse()?;
        let conjugated = wv.geo(&ca).geo(&inv);
        let carriage = cb.geo(&conjugated.versor_inverse()?);
        let gauge = relative_gauge(&vertex_gauge_at(va, ci), &vertex_gauge_at(vb, cj));
        Some(FacePlace { carriage, gauge, edge, refraction: None })
    }

    /// THE FACE BETWEEN TWO VERTICES, AT THE MEMBER PAIRS THE INCIDENCE NAMES.
    ///
    /// `pairs` is the incidence: which member of `i` relates to which member
    /// of `j`. It comes from the site occupancy that created the face
    /// (`named_faces`) or from the pairs a face already carries when
    /// its placements are re-derived. IT IS NEVER THE PRODUCT OF THE TWO
    /// FIBRES — a face exists because two BODIES' supports meet, and that
    /// incidence is between bodies, so it says which member relates to which.
    ///
    /// `None` when the word does not read; a member pair whose placement does
    /// not derive is left out of the fibre rather than given a versor it does
    /// not have (rule 11).
    pub fn face_at(
        &self,
        i: u16,
        j: u16,
        pairs: &[(stalk::Invariant, stalk::Invariant)],
    ) -> Option<FormFace> {
        self.face_at_by(i, j, pairs, &self.face_word(i, j)?)
    }

    /// The same face UNDER A NAMED WORD, for a section face: the word is the
    /// one the bundle spoke between these ends, and the placements are the
    /// relatives under it.
    pub fn face_at_by(
        &self,
        i: u16,
        j: u16,
        pairs: &[(stalk::Invariant, stalk::Invariant)],
        word: &Word,
    ) -> Option<FormFace> {
        crate::transducer::family::trace::trace("face_at", crate::transducer::family::trace::Wrote::Ordinal(j), None);
        let mut at = Keyed::new();
        for (ci, cj) in pairs {
            if let Some(p) = self.face_place_by(i, ci, j, cj, word, None) {
                at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
            }
        }
        note_unplaced(i, j, word, pairs.len(), at.len());
        Some(FormFace { i, j, word: word.clone(), at, pairs: None, interface: Vec::new() })
    }

    /// THE MEMBER PAIRS A FACE BETWEEN TWO VERTICES HAS where one endpoint
    /// stands at a single chart — the star case, and every face this tree makes
    /// outside a cover's meets. `None` where both carry several: the pairing is
    /// the scene's fact and this does not have it.
    pub fn face_pairs(&self, i: u16, j: u16) -> Option<Vec<(stalk::Invariant, stalk::Invariant)>> {
        let ci = self.vertices.get(i as usize)?.charts();
        let cj = self.vertices.get(j as usize)?.charts();
        match (ci.as_slice(), cj.as_slice()) {
            ([one], many) => Some(many.iter().map(|c| (one.clone(), c.clone())).collect()),
            (many, [one]) => Some(many.iter().map(|c| (c.clone(), one.clone())).collect()),
            _ => None,
        }
    }

    /// A FACE CARRYING A NAMED WORD, placed at the member pairs its two
    /// vertices name. This is what `FormFace::new` was for — a section face, or
    /// a chord — except that `new` recorded the word and left the placement
    /// empty, and a face with no placement contributes no member path to any
    /// orbit, so every orbit through it lost its carriage.
    pub fn face_carrying(&self, i: u16, j: u16, word: Word) -> Option<FormFace> {
        self.face_carrying_with_edge(i, j, word, None)
    }

    /// A FACE CARRYING A NAMED WORD AND AN EDGE TRANSPORT.
    pub fn face_carrying_with_edge(
        &self,
        i: u16,
        j: u16,
        word: Word,
        edge: Option<Transport>,
    ) -> Option<FormFace> {
        match self.face_pairs(i, j) {
            Some(pairs) => {
                crate::transducer::family::trace::trace("face_at", crate::transducer::family::trace::Wrote::Ordinal(j), None);
                let mut at = Keyed::new();
                for (ci, cj) in &pairs {
                    if let Some(p) = self.face_place_by(i, ci, j, cj, &word, edge.clone()) {
                        at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
                    }
                }
                note_unplaced(i, j, &word, pairs.len(), at.len());
                Some(FormFace { i, j, word, at, pairs: None, interface: Vec::new() })
            }
            None => {
                let mut f = FormFace::new(i, j, word);
                let ends = (
                    self.vertices.get(i as usize).and_then(|v| v.face_chart()),
                    self.vertices.get(j as usize).and_then(|v| v.face_chart()),
                );
                if let (Some(ci), Some(cj)) = &ends {
                    if let Some(p) = self.face_place_by(i, ci, j, cj, &f.word, edge) {
                        f.at.insert(crate::transducer::sentence::face_pair_key(ci, cj), p);
                    }
                }
                note_unplaced(i, j, &f.word, usize::from(matches!(ends, (Some(_), Some(_)))), f.at.len());
                Some(f)
            }
        }
    }

    /// THE STAR FACE OF A VERTEX, or any face one of whose endpoints stands at
    /// a single chart. There the incidence needs no reading: one side has one
    /// member, so each member of the other side relates to it and to nothing
    /// else. The Frame is the case that matters — every star face is
    /// Frame→vertex and the Frame stands at the root stratum alone.
    ///
    /// `None` where BOTH endpoints carry several members: the pairing is then
    /// a fact about the scene and this function does not have it. Callers with
    /// an incidence use `face_at`.
    pub fn face_of(&self, i: u16, j: u16) -> Option<FormFace> {
        self.face_at(i, j, &self.face_pairs(i, j)?)
    }

    pub fn delta1(&self) -> Vec<(u16, Word)> {
        self.orbits
            .iter()
            .enumerate()
            .map(|(i, lp)| (i as u16, lp.holonomy.clone()))
            .collect()
    }

}

/// §8.1, measurement only: a face whose word has no sensor action places
/// no member pair — `face_place_by` derives each pair's carriage through
/// `versor_of_word`, one plane by construction (51.2) — and every orbit
/// through it has no member path and no class. Said once per face, against
/// the pairs the incidence named.
fn note_unplaced(i: u16, j: u16, word: &Word, named: usize, placed: usize) {
    if named > 0 && placed == 0 && element::versor_of_word(word).is_none() {
        eprintln!("  face {i}-{j} placed no member pair: the word has no sensor action");
    }
}
