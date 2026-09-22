// 12.2, A20 — THE FACES A PIVOT WRITES: a certified move applied to a
// form's face carriers, and its inverse, the court's un-pivot.

use super::*;

impl Form {
    /// Apply a certified move (A20 `by`) to this form: a write onto the
    /// chord face's transport, or a chart axis opened over the loop that
    /// closes on the face (I6). Loops are re-walked by the caller.
    pub fn apply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.open_axis_on(*face);
            }
            _ => apply_by(&mut self.faces, by),
        }
    }

    /// The inverse of [`Form::apply_move`]: the court's un-pivot (A20 step 3).
    pub fn unapply_move(&mut self, by: &PivotBy) {
        match by {
            PivotBy::OpenAxis { face, .. } => {
                let _ = self.close_axis_on(*face);
            }
            _ => unapply_by(&mut self.faces, by),
        }
    }
}

pub(crate) fn apply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // A WORD MOVE WRITES BOTH OF A FACE'S CARRIERS, because a loop's
        // holonomy is read from whichever one its path has. `holonomy_words_on`
        // answers the EDGE COMPOSITE where a member path carries one and the
        // face word where it does not, so a move that wrote `f.word` alone
        // left the class unmoved on every path with an edge — the class was
        // read from a carrier the move never touched.
        //
        // Measured: `Placement` and `Hue` moves discharge on the wire and
        // `Word` moves do not — `axis Orientation(0) no discharge`, every
        // occurrence, across every window. Those two write `p.carriage` and
        // `p.gauge`, which `loop_transport_on` composes and `ClassVector::read`
        // takes; the word arm wrote the one carrier the reading skips. This is
        // the same split R8 removed from `file_loop` and R12 from the
        // `OpenAxis` pipeline, at the site that makes the move.
        //
        // Both are written with the SAME composition, so a path reading either
        // carrier sees the same move, and `unapply_by` inverts both.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = pivoted_face(&f.word, word, *side);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(e) = f.at.get_mut(&k).and_then(|p| p.edge.as_mut()) {
                        e.word = pivoted_face(&e.word, word, *side);
                    }
                }
            }
        }
        // A PIVOT MOVES THE FACE AT EVERY MEMBER PAIR. The move is a write on
        // the FACE — a translator on its placement, a gauge on its hue map —
        // and a face relating several pairs of bodies is moved at each of them
        // or the pivot moves one pair and leaves the rest where they were.
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.carriage = translator.geo(&p.carriage);
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, gauge);
                    }
                }
            }
        }
    }
}

pub(crate) fn unapply_by(faces: &mut [FormFace], by: &PivotBy) {
    match by {
        PivotBy::OpenAxis { .. } => {}
        // The mirror of `apply_by`'s word arm, carrier for carrier — or a
        // trial leaves the edge composite moved on a candidate it refused.
        PivotBy::Word { face, word, side } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                f.word = unpivoted_face(&f.word, word, *side);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(e) = f.at.get_mut(&k).and_then(|p| p.edge.as_mut()) {
                        e.word = unpivoted_face(&e.word, word, *side);
                    }
                }
            }
        }
        PivotBy::Placement { face, translator } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                if let Some(inv) = translator.versor_inverse() {
                    let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                    for k in pairs {
                        if let Some(p) = f.at.get_mut(&k) {
                            p.carriage = inv.geo(&p.carriage);
                        }
                    }
                }
            }
        }
        PivotBy::Hue { face, gauge } => {
            if let Some(f) = faces.get_mut(*face as usize) {
                let back = gauge_invert(gauge);
                let pairs: Vec<Vec<u8>> = f.at.scoped().map(|(k, _)| k).collect();
                for k in pairs {
                    if let Some(p) = f.at.get_mut(&k) {
                        p.gauge = compose_gauge(&p.gauge, &back);
                    }
                }
            }
        }
    }
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
