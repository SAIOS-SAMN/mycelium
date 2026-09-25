// 3.7, 16.20, A18, A20 — THE AXES OF A FORM: a chart axis opened over a
// held orbit and closed again, the seed it re-derives from, and the row each
// orbit contributes to the rank audit.

use super::*;

impl Form {
    /// The orbit closing on `chord`, by the chord's own overlap (AT1). A read
    /// of this form's own orbits — its boundary, never a search over
    /// anything else.
    pub(crate) fn orbit_of_chord(&self, chord: &coord::Overlap) -> Option<usize> {
        self.orbits.iter().position(|lp| &lp.chord == chord)
    }

    /// The class of the axis this orbit's class was opened on, if any (I6).
    pub fn opened_axis_of(&self, lp: &Orbit) -> Option<coord::Holonomy> {
        opened_axis_on(&self.faces, &self.vertices, &self.axes, lp)
    }

    /// Law 12: the opened axis is a theorem the orbit proves — the orbit is
    /// the one the relation names, and its holonomy is a versor, so the
    /// seed (A20 `OpenAxis`) re-derives from it. Otherwise the relation is
    /// a stored coordinate the form cannot re-derive, and the orbit stays a
    /// defect.
    ///
    /// AN AXIS IS KEYED BY A CLASS, SO THE CLASS IS WHAT IDENTIFIES IT
    /// (rule 15, 54.5 as ruled 2026-09-20): two orbits of one class are one
    /// obstruction and one axis serves both. What remains is the real
    /// question: does this orbit have a holonomy for the axis to be the
    /// transition OF (16.20).
    pub fn axis_rederives(&self, lp: &Orbit) -> bool {
        self.axis_seed_of(lp).is_some()
    }

    /// The seed of the axis an orbit opens: its own holonomy versor (16.20),
    /// which is the EDGE COMPOSITE where its paths carry one — never the
    /// word, which telescopes to the identity on a coboundary cycle. See
    /// [`crate::transducer::sentence::pivot::drive::orbit_holonomy_on`].
    pub fn axis_seed_of(&self, lp: &Orbit) -> Option<Versor> {
        element::versor_of_word(&self.orbit_holonomy(lp)?)
    }

    /// THE HOLONOMY EVERY MEMBER PATH OF THIS ORBIT AGREES ON, on this form.
    /// One reader for the whole `OpenAxis` pipeline — the gate, the seed,
    /// the class the axis is keyed by, and the row the audit reads.
    pub(crate) fn orbit_holonomy(&self, lp: &Orbit) -> Option<Word> {
        crate::transducer::sentence::pivot::drive::orbit_holonomy_on(&self.faces, &self.vertices, lp)
    }

    /// Open a chart axis over the orbit closing on `chord` (A20 `OpenAxis`).
    /// Refuses a chord with no orbit, a holonomy with no class, and a
    /// holonomy with no versor to seed the axis.
    pub(crate) fn open_axis_on(&mut self, chord: &coord::Overlap) -> Option<()> {
        let i = self.orbit_of_chord(chord)?;
        let lp = self.orbits.get(i)?;
        let class = self.orbit_holonomy(lp)?.rotor().holonomy_key()?;
        self.axis_seed_of(lp)?;
        self.axes.put(&class, Some(chord.clone()));
        Some(())
    }

    /// The inverse move: the axis opened over the orbit closing on `chord`
    /// leaves the form (the court's un-pivot, A20 step 3). Keyed by the same
    /// composite the opening used, or it removes nothing and the axis stands
    /// with no orbit claiming it.
    pub(crate) fn close_axis_on(&mut self, chord: &coord::Overlap) -> Option<()> {
        let i = self.orbit_of_chord(chord)?;
        let lp = self.orbits.get(i)?;
        let class = self.orbit_holonomy(lp)?.rotor().holonomy_key()?;
        self.axes.drop_at(&class)?;
        Some(())
    }

    /// The row one orbit contributes to the rank audit (A18): an opened
    /// axis, or what its holonomy is; `None` when the holonomy has no class.
    pub fn orbit_row_at(&self, idx: usize) -> Option<crate::transducer::sentence::OrbitRow> {
        orbit_row_on(&self.faces, &self.vertices, &self.axes, self.orbits.get(idx)?)
    }

    pub fn orbit_rows(&self) -> Option<Vec<crate::transducer::sentence::OrbitRow>> {
        (0..self.orbits.len()).map(|i| self.orbit_row_at(i)).collect()
    }

    /// A18 over this form's orbits. `None` when a holonomy has no class or
    /// the ℤ-rank overflows: unmeasured, and the caller refuses.
    pub fn rank(&self) -> Option<crate::transducer::sentence::Rank> {
        crate::transducer::sentence::rank_of_rows(&self.orbit_rows()?)
    }
}

/// THE AXIS CARRIER AND THE ORBIT, without the form between them. An orbit's
/// opened axis is a keyed read of the form's axis carrier at the orbit's own
/// class, so a trial holding that carrier reads it directly (I6). The class
/// is the edge composite's wherever a path carries one, which cannot be read
/// from the orbit alone — so the faces and vertices are the carrier.
pub(crate) fn opened_axis_on(
    faces: &Faces,
    vertices: &Vertices,
    axes: &Keyed<Option<coord::Overlap>, coord::Holonomy>,
    lp: &Orbit,
) -> Option<coord::Holonomy> {
    let class = crate::transducer::sentence::pivot::drive::orbit_holonomy_on(faces, vertices, lp)?
        .rotor()
        .holonomy_key()?;
    axes.holds(&class).then_some(class)
}

/// The row one orbit contributes to the rank audit (A18), read off the axis
/// carrier and the orbit's own holonomy — the edge composite, the same object
/// the axis is keyed by and the class vector is read from.
pub(crate) fn orbit_row_on(
    faces: &Faces,
    vertices: &Vertices,
    axes: &Keyed<Option<coord::Overlap>, coord::Holonomy>,
    lp: &Orbit,
) -> Option<crate::transducer::sentence::OrbitRow> {
    // The class the axis was opened under IS the row's coordinate, so it is
    // carried: `rank_of_rows` counts axes by class, and one opening over a
    // class shared by two orbits is one axis, not two.
    if let Some(class) = opened_axis_on(faces, vertices, axes, lp) {
        return Some(crate::transducer::sentence::OrbitRow::Opened(class));
    }
    // The row is the obstruction's. An orbit whose paths disagree has no
    // single holonomy and therefore no row (rule 11).
    crate::transducer::sentence::orbit_row(&crate::transducer::sentence::pivot::drive::orbit_holonomy_on(
        faces, vertices, lp,
    )?)
}
