// 3.7, 16.20, A18, A20 — THE AXES OF A FORM: a chart axis opened over a
// held loop and closed again, the seed it re-derives from, and the row each
// loop contributes to the rank audit.

use super::*;

impl Form {
    /// The loop closing on `chord`, by index. A walk of this form's own
    /// loops — its boundary, never a search over anything else.
    pub(crate) fn loop_of_chord(&self, chord: u16) -> Option<usize> {
        for (i, lp) in self.loops.iter().enumerate() {
            if lp.chord == chord {
                return Some(i);
            }
        }
        None
    }

    /// The chord of the loop this loop's class was opened on, if any (I6).
    pub fn opened_axis_of(&self, lp: &Loop) -> Option<u16> {
        opened_axis_on(&self.faces, &self.vertices, &self.axes, lp)
    }

    /// Law 12: the opened axis is a theorem the loop proves — the loop is
    /// the one the relation names, and its holonomy is a versor, so the
    /// seed (A20 `OpenAxis`) re-derives from it. Otherwise the relation is
    /// a stored coordinate the form cannot re-derive, and the loop stays a
    /// defect.
    /// AN AXIS IS KEYED BY A CLASS, SO THE CLASS IS WHAT IDENTIFIES IT
    /// (rule 15, 54.5 as ruled 2026-09-20).
    ///
    /// This read `chord == lp.chord` beside the seed, and the chord is a
    /// FACE INDEX — a position, not a coordinate. `form_vertex_key` knows
    /// that and writes an opened axis as its CLASS alone; the decoder has
    /// nothing to put in the chord slot and writes `0`. So on every form
    /// that came out of an address the stored chord was a placeholder, and
    /// this compared it as though it were real: the axis re-derived only
    /// when the loop's chord happened to be face zero, and otherwise a form
    /// that opened an axis was refused for it.
    ///
    /// `opened_axis_of` already finds the axis BY THE LOOP'S OWN CLASS,
    /// which is content. Two loops of one class are one obstruction and one
    /// axis serves both — that is what keying by class means — so there is
    /// nothing left for an index to disambiguate. What remains is the real
    /// question: does this loop have a holonomy for the axis to be the
    /// transition OF (16.20).
    pub fn axis_rederives(&self, lp: &Loop, _chord: u16) -> bool {
        self.axis_seed_of(lp).is_some()
    }

    /// The seed of the axis a loop opens: its own holonomy versor (16.20),
    /// which is the EDGE COMPOSITE where its paths carry one. Read off
    /// `lp.holonomy` — the word — it seeded the axis from a coboundary that
    /// telescopes to the identity, so the seed was the identity versor and
    /// the axis it opened stood for nothing the loop turns by. See
    /// [`crate::transducer::sentence::pivot::drive::loop_holonomy_on`].
    pub fn axis_seed_of(&self, lp: &Loop) -> Option<Versor> {
        element::versor_of_word(&self.loop_holonomy(lp)?)
    }

    /// THE HOLONOMY EVERY MEMBER PATH OF THIS LOOP AGREES ON, on this form.
    /// One reader for the whole `OpenAxis` pipeline — the gate, the seed,
    /// the class the axis is keyed by, and the row the audit reads — so the
    /// writer and the reader cannot key one carrier by two objects, which is
    /// exactly the split R8 removed from `file_loop`.
    pub(crate) fn loop_holonomy(&self, lp: &Loop) -> Option<Word> {
        crate::transducer::sentence::pivot::drive::loop_holonomy_on(
            &self.faces,
            &self.vertices,
            lp,
        )
    }

    /// Open a chart axis over the loop closing on `chord` (A20 `OpenAxis`).
    /// Refuses a chord with no loop, a holonomy with no class, and a
    /// holonomy with no versor to seed the axis.
    pub(crate) fn open_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = self.loop_holonomy(lp)?.rotor().holonomy_key()?;
        self.axis_seed_of(lp)?;
        self.axes.put(&class, chord);
        Some(())
    }

    /// The inverse move: the axis opened over the loop closing on `chord`
    /// leaves the form (the court's un-pivot, A20 step 3). Keyed by the same
    /// composite the opening used, or it removes nothing and the axis stands
    /// with no loop claiming it.
    pub(crate) fn close_axis_on(&mut self, chord: u16) -> Option<()> {
        let i = self.loop_of_chord(chord)?;
        let lp = self.loops.get(i)?;
        let class = self.loop_holonomy(lp)?.rotor().holonomy_key()?;
        self.axes.drop_at(&class)?;
        Some(())
    }

    /// The row one loop contributes to the rank audit (A18): an opened
    /// axis, or what its holonomy is; `None` when the holonomy has no class.
    pub fn loop_row_at(&self, idx: usize) -> Option<crate::transducer::sentence::LoopRow> {
        loop_row_on(&self.faces, &self.vertices, &self.axes, self.loops.get(idx)?)
    }

    pub fn loop_rows(&self) -> Option<Vec<crate::transducer::sentence::LoopRow>> {
        (0..self.loops.len()).map(|i| self.loop_row_at(i)).collect()
    }

    /// A18 over this form's loops. `None` when a holonomy has no class or
    /// the ℤ-rank overflows: unmeasured, and the caller refuses.
    pub fn rank(&self) -> Option<crate::transducer::sentence::Rank> {
        crate::transducer::sentence::rank_of_rows(&self.loop_rows()?)
    }
}

/// THE AXIS CARRIER AND THE LOOP, without the form between them. A loop's
/// opened axis is a keyed read of the form's axis carrier at the loop's own
/// class, so a trial holding that carrier reads it directly (I6).
/// THE FACES AND VERTICES ARE THE CARRIER, not decoration on the signature.
/// The axis is keyed by the loop's holonomy CLASS, and a loop's holonomy is
/// its edge composite wherever a path carries one — which cannot be read
/// from `lp` alone, because `lp.holonomy` is the word and the word
/// telescopes to the identity on any coboundary cycle. Read there, this
/// answered `axes.get(identity_class)` for every loop on the corpus and the
/// opening was invisible to the audit that has to count it.
pub(crate) fn opened_axis_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &Keyed<u16, coord::Holonomy>,
    lp: &Loop,
) -> Option<u16> {
    let class = crate::transducer::sentence::pivot::drive::loop_holonomy_on(faces, vertices, lp)?
        .rotor()
        .holonomy_key()?;
    axes.at(&class).copied()
}

/// The row one loop contributes to the rank audit (A18), read off the axis
/// carrier and the loop's own holonomy — the edge composite, the same object
/// the axis is keyed by and the class vector is read from.
pub(crate) fn loop_row_on(
    faces: &[FormFace],
    vertices: &[Vertex],
    axes: &Keyed<u16, coord::Holonomy>,
    lp: &Loop,
) -> Option<crate::transducer::sentence::LoopRow> {
    // The class the axis was opened under IS the row's coordinate, so it is
    // carried: `rank_of_rows` counts axes by class, and one opening over a
    // class shared by two loops is one axis, not two.
    if opened_axis_on(faces, vertices, axes, lp).is_some() {
        let class = crate::transducer::sentence::pivot::drive::loop_holonomy_on(faces, vertices, lp)?
            .rotor()
            .holonomy_key()?;
        return Some(crate::transducer::sentence::LoopRow::Opened(class));
    }
    // The row is the obstruction's, so it is read off the obstruction the
    // rest of this pipeline names. A loop whose paths disagree has no single
    // holonomy and therefore no row (rule 11), which is the same refusal
    // `loop_row` makes for a holonomy that is not a versor.
    crate::transducer::sentence::loop_row(&crate::transducer::sentence::pivot::drive::loop_holonomy_on(
        faces, vertices, lp,
    )?)
}
