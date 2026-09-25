// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.2, 12.2, 50.8 — REBASE, AND THE ORBIT THAT RETURNS TO ITS OWN
//! BASEPOINT. "Do to B what A did to itself" is a relation between TWO
//! vertices; an edge that comes back to the vertex it left is not an
//! outward step at all but an element of that vertex's STABILIZER, and it
//! is evaluated rather than skipped.

use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::element::Versor;

impl Form {
    /// Replace vertex `of` by `by` read in `of`'s frame (3.2). `of`'s own
    /// dilation, hue map and chart are kept on the Rebased vertex; only
    /// its word is now `by`'s. A vertex that is not a `Word` is left as it
    /// is, and `of` is returned either way.
    ///
    /// A REBASE THAT RETURNS TO ITS OWN BASEPOINT IS AN ELEMENT OF THE
    /// STABILIZER. When `by` is `of`, or `by`'s own rebase chain comes back
    /// to `of`, the edge closes a 1-cycle at `of` and `carriage` is the
    /// accumulated gauge `Δ` around that cycle. Writing `Rebased { by: of }`
    /// would define the vertex's word in terms of its own word, and the
    /// first reader to ask for it would not come back (L20).
    ///
    /// The cycle is CLOSED by reading its boundary holonomy through the one
    /// classifier both organs use, and the vertex is returned unchanged:
    ///
    /// - `Δ` flat and trivial — zero curvature, zero shift. The rebase
    ///   absorbs the identity, `rebase(v, 1) ≡ v`, and the traversal
    ///   advances along the rest of the star. Nothing is filed: there is
    ///   nothing to fill.
    /// - `Δ` of finite order `ℓ` with no residue — the cycle carries
    ///   discrete holonomy and is FILLED by a 2-simplex: the class is filed on
    ///   this form's own torsion carrier at period `ℓ`, on the chord that
    ///   closes it. That is the same `FiledOrbit` an orbit over a cover files
    ///   (50.8 case 3); the cycle becomes content instead of a traversal.
    /// - anything else — the obstruction stands and nothing is invented for
    ///   it (rule 11). The vertex is unchanged either way.
    ///
    /// `chord` is the face the caller crossed to get here — the face that
    /// closes the cycle. It is passed in and never searched for: the caller
    /// holds it, and a filed orbit names a live face or it names nothing
    /// (15.5).
    ///
    /// The chain is TRAVERSED, not just the one step: a `by` two hops away
    /// that comes back is the same cycle with a longer boundary.
    pub fn rebase(&mut self, by: u16, of: u16, carriage: Versor, chord: u16) -> Option<u16> {
        // A vertex that is not there is not a frame to read in. Read, never
        // compared against a length: absence is the refusal (rule 2).
        self.vertices.get(by as usize)?;
        if by == of || self.rebase_reaches(by, of) {
            self.close_stabilizer_orbit(of, &carriage, chord);
            return Some(of);
        }
        // A REBASED VERTEX KEEPS EVERY CHART `of` ACTED ON, with the relative at
        // each. It kept one — `of`'s representative member — so rebasing a
        // vertex acting on two charts dropped the second body at the one move
        // whose whole job is to REPLACE a vertex, leaving nothing to read the
        // dropped member back from (D10).
        //
        // `carriage` is the relative the CALLER measured, `C_of · C_by⁻¹`
        // between one member of each. Where `of` acts on several charts the
        // relative differs at each, so it is re-derived here per chart from the
        // two vertices this vertex names: `t_c = C_of,c · C_by⁻¹`. The caller's
        // value is what the single-chart case gives and is used there, so a
        // caller that measured a relative gets the one it measured.
        //
        // THE BASE MUST STAND AT ONE CHART. A fibre keyed by `of`'s chart has no
        // room to say which member of the base each relative was taken against,
        // so a multi-member base refuses (rule 11) and the vertex stays as it
        // was — which is what `rebase`'s contract already does for a non-Word,
        // and `rebase_named_faces` reads `None` as "this face does not rebase".
        let base_charts = self.vertices.get(by as usize)?.charts();
        let [base_chart] = base_charts.as_slice() else {
            return None;
        };
        let base_inv = crate::transducer::sentence::transport::vertex_carriage_at(
            self.vertices.get(by as usize)?,
            self,
            base_chart,
        )
        .filter(crate::transducer::cga::is_versor)
        .map(|c| c.reverse());
        let held = self.vertices.get(of as usize)?;
        let Vertex::Word { acts: held_acts, .. } = held else {
            return None;
        };
        let single = held_acts.len() == 1;
        let mut acts: Keyed<Acting, crate::network::nerve::coord::Section> = Keyed::new();
        for (_ck, a) in held_acts.scoped() {
            let relative = match (single, &base_inv) {
                // One chart: the caller's own measurement, unchanged.
                (true, _) => carriage.clone(),
                // Several: the relative at THIS chart, `C_of,c · C_by⁻¹`.
                (false, Some(inv)) => a.carriage.geo(inv),
                // No base placement to measure against is not a relative of
                // zero; the member carries none and is not written (rule 11).
                (false, None) => continue,
            };
            // THE MEMBER NAMES ITSELF (54.7 R1, law 12). This inserted
            // under the SOURCE member's key `ck` while building an `Acting`
            // with a different carriage — the relative `t = C_of,chart ·
            // C_by⁻¹`. While the key was the chart alone the two agreed by
            // accident; once the key is `chart ‖ carriage` it files a
            // section under another section's name.
            let member = Acting {
                chart: a.chart.clone(),
                carriage: Box::new(relative),
                gauge: a.gauge.clone(),
            };
            acts.put(&member.key(), member);
        }
        if acts.is_empty() {
            return None;
        }
        self.vertices[of as usize] = Vertex::Rebased { by, of, acts };
        Some(of)
    }

    /// Does `from`'s rebase chain reach `target`? The traversal carries the
    /// vertices it has VISITED, keyed, and stops when it meets one again —
    /// which is the same contraction `word_in` performs, not a step count
    /// standing in for one. A chain that revisits anything is closed, and a
    /// closed chain that a new rebase would join is a cycle whether or not
    /// `target` is the vertex it closes on.
    fn rebase_reaches(&self, from: u16, target: u16) -> bool {
        let mut at = from;
        let mut seen: Keyed<(), crate::network::nerve::coord::VertexIx> = Keyed::new();
        loop {
            if at == target {
                return true;
            }
            if seen.holds(&vertex_ix_key(at)) {
                return true;
            }
            seen.put(&vertex_ix_key(at), ());
            match self.vertices.get(at as usize) {
                Some(Vertex::Rebased { by, .. }) => at = *by,
                _ => return false,
            }
        }
    }

    /// THE 2-SIMPLEX THAT CLOSES A STABILIZER ORBIT (50.8 case 3, 12.2). `Δ`
    /// is the accumulated gauge around a cycle that returned to `of`. Its
    /// rotor is read by A22's peel — a composed carriage carries no tape,
    /// so there is no spelling to read off one — and the reading is
    /// `cech::classify`'s, the one classifier both organs use.
    ///
    /// Filing it here is what turns the traversal into content: the engine does
    /// not step out of the orbit, it EVALUATES the orbit, and the orbit
    /// becomes a class the form carries. A chord that names no live face
    /// files nothing.
    fn close_stabilizer_orbit(&mut self, of: u16, delta: &Versor, chord: u16) {
        use crate::transducer::cech::Fill;
        if self.faces.get(chord as usize).is_none() {
            return;
        }
        let Some(word) = crate::transducer::family::peel_versor(delta).and_then(|r| r.ok()) else {
            return;
        };
        let Some(v) = self.vertices.get(of as usize) else {
            return;
        };
        // THE ORBIT IS CLASSIFIED AT EVERY CHART THE VERTEX STANDS AT. The hue
        // cycle is a member coordinate, so a vertex acting on several charts
        // closes a cycle whose `𝒢` reading is each of theirs, and each
        // congruent one is filed under its own class. Reading one member's
        // gauge filed one class and dropped the rest.
        for chart in v.charts() {
            let hue = CycleType::of(&vertex_gauge_at(v, &chart));
            let Fill::Congruent(_, g, _ell) =
                crate::transducer::cech::classify(&word.rotor(), delta, hue)
            else {
                continue;
            };
            let Some(class) = g.holonomy_key() else { continue };
            // `sigma` is EMPTY and that is the reading, not a placeholder: a
            // stabilizer orbit at one vertex permutes no hues between
            // vertices, so there is no generator of a hue cycle to record. The
            // period is the rotor's own order and the chord is the face that
            // closed it.
            self.torsion.put(
                &class,
                crate::transducer::sentence::FiledOrbit {
                    sigma: Vec::new(),
                    periods: g.toral_period(),
                    chord,
                },
            );
        }
    }

    pub fn unfold(&mut self, measure: Measure) -> u16 {
        #[expect(clippy::cast_possible_truncation, reason = "AT1 debt — a slot number of the form's Vec (the Unfolded vertex about to be pushed); the atlas row replaces the slot by the chart's coordinates")]
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Unfolded { measure });
        idx
    }
}
