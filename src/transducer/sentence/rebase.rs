// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.2, 12.2, 50.8 — REBASE, AND THE ORBIT THAT RETURNS TO ITS OWN
//! BASEPOINT. "Do to B what A did to itself" is a relation between TWO
//! vertices; an edge that comes back to the vertex it left is not an
//! outward step at all but an element of that vertex's STABILIZER, and it
//! is evaluated rather than skipped.

use super::*;
use crate::network::nerve::{coord, Keyed};
use crate::transducer::element::Versor;

impl Form {
    /// Replace vertex `of` by `by` read in `of`'s frame (3.2). `of`'s own
    /// dilation, hue map and charts are kept on the Rebased vertex; only its
    /// word is now `by`'s. A vertex that is not a `Word` is left as it is.
    /// Returns the rebased vertex's coordinate — its NEW key (AT1, ruled:
    /// `tag ‖ class(by) ‖ class(of)`), under which every overlap at it is
    /// re-filed ([`Form::rekey_vertex`]).
    ///
    /// A REBASE THAT RETURNS TO ITS OWN BASEPOINT IS AN ELEMENT OF THE
    /// STABILIZER. When `by` is `of`, or `by`'s own rebase chain comes back
    /// to `of`, the edge closes a 1-cycle at `of` and `carriage` is the
    /// accumulated gauge `Δ` around that cycle: the cycle is CLOSED by
    /// reading its boundary holonomy through the one classifier both organs
    /// use, and the vertex is returned unchanged (see
    /// `close_stabilizer_orbit`).
    ///
    /// `chord` is the overlap the caller crossed to get here — the one that
    /// closes the cycle. It is passed in and never searched for (15.5).
    pub fn rebase(&mut self, by: &coord::VertexAt, of: &coord::VertexAt, carriage: Versor, chord: &coord::Overlap) -> Option<coord::VertexAt> {
        // A vertex that is not there is not a frame to read in (rule 2).
        self.vertices.at(by)?;
        if by == of || self.rebase_reaches(by, of) {
            self.close_stabilizer_orbit(of, &carriage, chord);
            return Some(of.clone());
        }
        // A REBASED VERTEX KEEPS EVERY CHART `of` ACTED ON, with the relative
        // at each, and every frame each member stood on (D10, AT1). Where `of`
        // acts on several charts the relative differs at each and is
        // re-derived per chart, `t_c = C_of,c · C_by⁻¹`; THE BASE MUST STAND
        // AT ONE CHART, or the relative has no one member of the base to be
        // taken against and the rebase refuses (rule 11).
        let base_charts = self.vertices.at(by)?.charts();
        let [base_chart] = base_charts.as_slice() else {
            return None;
        };
        let base_inv = crate::transducer::sentence::transport::vertex_carriage_at(self.vertices.at(by)?, self, base_chart)
            .filter(crate::transducer::cga::is_versor)
            .map(|c| c.reverse());
        let held = self.vertices.at(of)?;
        let class = held.class()?;
        let Vertex::Word { acts: held_acts, .. } = held else {
            return None;
        };
        let single = held_acts.len() == 1;
        let mut acts: Keyed<Acting, coord::Section> = Keyed::new();
        for (_ck, a) in held_acts.scoped() {
            let relative = match (single, &base_inv) {
                (true, _) => carriage.clone(),
                (false, Some(inv)) => a.carriage.geo(inv),
                (false, None) => continue,
            };
            // THE MEMBER NAMES ITSELF (54.7 R1, law 12).
            let member = Acting {
                chart: a.chart.clone(),
                carriage: Box::new(relative),
                gauge: a.gauge.clone(),
                frames: a.frames.clone(),
            };
            acts.put(&member.key(), member);
        }
        if acts.is_empty() {
            return None;
        }
        let rebased = Vertex::Rebased { by: by.clone(), class, acts };
        let at = rebased.at()?;
        self.vertices.drop_at(of);
        self.vertices.put(&at, rebased);
        self.rekey_vertex(of, &at);
        Some(at)
    }

    /// A VERTEX THAT CHANGED ITS COORDINATE TAKES ITS OVERLAPS WITH IT: every
    /// overlap with an end at `old` is re-filed with that end at `new`, and
    /// every carrier naming one of them by its key — the bodies a contact
    /// runs between, a filed orbit's chord, an opened axis's chord — follows.
    /// Nothing is renumbered; the keys are the coordinates, and the
    /// coordinate moved.
    pub(crate) fn rekey_vertex(&mut self, old: &coord::VertexAt, new: &coord::VertexAt) {
        let chart = |c: &coord::ChartAt| match c {
            coord::ChartAt::At(m) if &m.vertex == old => coord::ChartAt::at(new.clone(), m.chart.clone(), m.frame),
            other => other.clone(),
        };
        for o in self.overlaps_at(old) {
            let moved = coord::Overlap { from: chart(&o.from), to: chart(&o.to) };
            if let Some(f) = self.faces.drop_at(&o) {
                self.faces.put(&moved, f);
            }
            if let Some(b) = self.face_bodies.drop_at(&o) {
                self.face_bodies.put(&moved, b);
            }
            let filed: Vec<coord::Holonomy> =
                self.torsion.entries().filter(|(_, f)| f.chord.overlap() == Some(&o)).map(|(k, _)| k).collect();
            for k in filed {
                if let Some(f) = self.torsion.at_mut(&k) {
                    f.chord = crate::transducer::sentence::Closes::On(moved.clone());
                }
            }
            let axes: Vec<coord::Holonomy> =
                self.axes.entries().filter(|(_, c)| c.as_ref() == Some(&o)).map(|(k, _)| k).collect();
            for k in axes {
                self.axes.put(&k, Some(moved.clone()));
            }
        }
    }

    /// Does `from`'s rebase chain reach `target`? The traversal carries the
    /// vertices it has VISITED, by their coordinates, and stops when it meets
    /// one again: a chain that revisits anything is closed.
    fn rebase_reaches(&self, from: &coord::VertexAt, target: &coord::VertexAt) -> bool {
        let mut at = from.clone();
        let mut seen: Keyed<(), coord::VertexAt> = Keyed::new();
        loop {
            if &at == target || seen.holds(&at) {
                return true;
            }
            seen.put(&at, ());
            match self.vertices.at(&at) {
                Some(Vertex::Rebased { by, .. }) => at = by.clone(),
                _ => return false,
            }
        }
    }

    /// THE 2-SIMPLEX THAT CLOSES A STABILIZER ORBIT (50.8 case 3, 12.2). `Δ`
    /// is the accumulated gauge around a cycle that returned to `of`. Its
    /// rotor is read by A22's peel and classified by `cech::classify`; a
    /// congruent class is filed on the form's torsion at the period its
    /// rotor's order reads, on the chord that closed it. A chord that names
    /// no live overlap files nothing.
    fn close_stabilizer_orbit(&mut self, of: &coord::VertexAt, delta: &Versor, chord: &coord::Overlap) {
        use crate::transducer::cech::Fill;
        if !self.faces.holds(chord) {
            return;
        }
        let Some(word) = crate::transducer::family::peel_versor(delta).and_then(|r| r.ok()) else {
            return;
        };
        let Some(v) = self.vertices.at(of) else {
            return;
        };
        // THE ORBIT IS CLASSIFIED AT EVERY CHART THE VERTEX STANDS AT, each
        // congruent one filed under its own class.
        let mut filed = Vec::new();
        for chart in v.charts() {
            let hue = CycleType::of(&vertex_gauge_at(v, &chart));
            let Fill::Congruent(_, g, _ell) = crate::transducer::cech::classify(&word.rotor(), delta, hue) else {
                continue;
            };
            let Some(class) = g.holonomy_key() else { continue };
            filed.push((class, g.toral_period()));
        }
        for (class, periods) in filed {
            // `sigma` is EMPTY and that is the reading: a stabilizer orbit at
            // one vertex permutes no hues between vertices.
            self.torsion.put(
                &class,
                crate::transducer::sentence::FiledOrbit {
                    sigma: Vec::new(),
                    periods,
                    chord: crate::transducer::sentence::Closes::On(chord.clone()),
                },
            );
        }
    }

    pub fn unfold(&mut self, measure: Measure) -> Option<coord::VertexAt> {
        self.put_vertex(Vertex::Unfolded { measure })
    }
}
