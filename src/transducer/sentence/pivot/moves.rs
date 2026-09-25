// 12.2, A20 — THE FACES A PIVOT WRITES: a certified move applied to a
// form's face carriers, and its inverse, the court's un-pivot.

use super::*;

impl Form {
    /// Apply a certified move (A20 `by`) to this form: a write onto the
    /// chord face's transport, or a chart axis opened over the orbit that
    /// closes on the face (I6). Orbits are re-read by the caller.
    pub fn apply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.open_axis_on(face);
            }
            _ => apply_by(&mut self.faces, by),
        }
    }

    /// The inverse of [`Form::apply_move`]: the court's un-pivot (A20 step 3).
    /// `false` where the move has no inverse — a placement whose translator
    /// is on the light cone — and nothing was written (VS1's ruling, item 6:
    /// no inverse, no un-pivot, refused by name and never skipped).
    #[must_use]
    pub fn unapply_move(&mut self, by: &PivotBy) -> bool {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.close_axis_on(face);
                true
            }
            _ => unapply_by(&mut self.faces, by),
        }
    }
}

pub(crate) fn apply_by(faces: &mut Faces, by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // A WORD MOVE WRITES THE OVERLAP'S FACTOR AND ITS EDGE (AT1, ruled):
        // the factor is the one produced content a face carries, and the
        // derived word reads it (`face_word_on`); the edge is measured
        // content the move moves too, because an orbit's holonomy is read
        // from whichever carrier its path has, and a move that wrote one left
        // the class unmoved on every path with the other. Both are written
        // with the SAME composition, and `unapply_by` inverts both.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.at_mut(face) {
                match side {
                    // `g · w⁻¹`: the right factor gains `w` on its left, so it
                    // enters inverted as `… · w⁻¹`.
                    Side::Right => f.right = word.compose(&f.right).normal_form(),
                    Side::Left => f.left = word.compose(&f.left).normal_form(),
                }
                if let Some(e) = f.place.edge.as_mut() {
                    e.word = pivoted_face(&e.word, word, *side);
                }
            }
        }
        // A PIVOT MOVES THE OVERLAP'S PLACE: a translator on its placement,
        // a gauge on its hue map.
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.at_mut(face) {
                f.place.carriage = translator.geo(&f.place.carriage);
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.at_mut(face) {
                f.place.gauge = compose_gauge(&f.place.gauge, gauge);
            }
        }
    }
}

/// The inverse of [`apply_by`]. `false` where the move has no inverse (a
/// placement's translator on the light cone): it was the defect VS1 named —
/// the un-pivot silently skipped and the caller read a form it believed
/// restored — and it now refuses by name, writing nothing.
#[must_use]
pub(crate) fn unapply_by(faces: &mut Faces, by: &PivotBy) -> bool {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // The mirror of `apply_by`'s word arm, carrier for carrier — or a
        // trial leaves the edge composite moved on a candidate it refused.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.at_mut(face) {
                match side {
                    Side::Right => f.right = word.inverse().compose(&f.right).normal_form(),
                    Side::Left => f.left = word.inverse().compose(&f.left).normal_form(),
                }
                if let Some(e) = f.place.edge.as_mut() {
                    e.word = unpivoted_face(&e.word, word, *side);
                }
            }
        }
        PivotBy::Placement { face, translator } => {
            // A translator's reverse is its inverse (`T T̃ = 1`), so the
            // unapply restores the carriage exactly (78.32(1)).
            if !crate::transducer::cga::is_versor(translator) {
                return false;
            }
            let inv = translator.reverse();
            if let Some(f) = faces.at_mut(face) {
                f.place.carriage = inv.geo(&f.place.carriage);
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.at_mut(face) {
                let back = gauge_invert(gauge);
                f.place.gauge = compose_gauge(&f.place.gauge, &back);
            }
        }
    }
    true
}

pub fn pivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(&w.inverse()).normal_form(),
        Side::Left => w.compose(g).normal_form(),
    }
}

/// The face word before that move: the inverse of [`pivoted_face`].
pub fn unpivoted_face(g: &Word, w: &Word, side: Side) -> Word {
    match side {
        Side::Right => g.compose(w).normal_form(),
        Side::Left => w.inverse().compose(g).normal_form(),
    }
}
