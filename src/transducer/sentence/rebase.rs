// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.2, 12.2, 50.8 — REBASE, AND THE ORBIT THAT RETURNS TO ITS OWN
//! BASEPOINT. "Do to B what A did to itself" is a relation between TWO
//! vertices; an edge that comes back to the vertex it left is not an
//! outward step at all but an element of that vertex's STABILIZER, and it
//! is evaluated rather than skipped.
//!
//! Split from `chart.rs` at the §1.1 600-line reading bound; `rebase` and
//! `unfold` are moved, not rewritten (§1.6), and the orbit reader is new.

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
    /// first reader to ask for it does not come back — L20's loop, measured
    /// on `0607ce86.json`, where `Δ` is the identity because the two ends
    /// are one vertex and `c·c⁻¹ = 1`.
    ///
    /// The cycle is CLOSED by reading its boundary holonomy through the one
    /// classifier both organs use, and the vertex is returned unchanged:
    ///
    /// - `Δ` flat and trivial — zero curvature, zero shift. The rebase
    ///   absorbs the identity, `rebase(v, 1) ≡ v`, and the traversal
    ///   advances along the rest of the star. Nothing is filed: there is
    ///   nothing to fill.
    /// - `Δ` of finite order `ℓ` with no residue — the cycle carries
    ///   discrete holonomy and is FILLED by a 2-cell: the class is filed on
    ///   this form's own torsion carrier at period `ℓ`, on the chord that
    ///   closes it. That is the same `FiledLoop` a loop over a cover files
    ///   (50.8 case 3); the cycle becomes content instead of a walk.
    /// - anything else — the obstruction stands and nothing is invented for
    ///   it (rule 11). The vertex is unchanged either way.
    ///
    /// `chord` is the face the caller walked to get here — the face that
    /// closes the cycle. It is passed in and never searched for: the caller
    /// holds it, and a filed loop names a live face or it names nothing
    /// (15.5).
    ///
    /// The chain is WALKED, not just the one step: a `by` two hops away
    /// that comes back is the same cycle with a longer boundary.
    pub fn rebase(&mut self, by: u16, of: u16, carriage: Versor, chord: u16) -> Option<u16> {
        // A vertex that is not there is not a frame to read in. Read, never
        // compared against a length: absence is the refusal (rule 2).
        self.vertices.get(by as usize)?;
        if by == of || self.rebase_reaches(by, of) {
            self.close_stabilizer_orbit(of, &carriage, chord);
            return Some(of);
        }
        // A REBASED VERTEX KEEPS `of`'s CHART AND GAUGE, and `Rebased` holds
        // one of each: the vertex being rebased is single-chart today, so
        // its representative member IS its chart and gauge. D7b decides
        // what rebasing a multi-member vertex means; it is not this commit.
        let held = self.vertices.get(of as usize)?;
        let (Some(a), true) = (held.acting(), matches!(held, Vertex::Word { .. })) else {
            return None;
        };
        let (gauge, class) = (a.gauge.clone(), a.chart.clone());
        self.vertices[of as usize] = Vertex::Rebased {
            by,
            of,
            carriage: Box::new(carriage),
            gauge,
            class,
        };
        Some(of)
    }

    /// Does `from`'s rebase chain reach `target`? The walk carries the
    /// vertices it has VISITED, keyed, and stops when it meets one again —
    /// which is the same contraction `word_in` performs, not a step count
    /// standing in for one. A chain that revisits anything is closed, and a
    /// closed chain that a new rebase would join is a cycle whether or not
    /// `target` is the vertex it closes on.
    fn rebase_reaches(&self, from: u16, target: u16) -> bool {
        let mut at = from;
        let mut seen: Keyed<()> = Keyed::new();
        loop {
            if at == target {
                return true;
            }
            if seen.contains(&vertex_ix_key(at)) {
                return true;
            }
            seen.insert(vertex_ix_key(at), ());
            match self.vertices.get(at as usize) {
                Some(Vertex::Rebased { by, .. }) => at = *by,
                _ => return false,
            }
        }
    }

    /// THE 2-CELL THAT CLOSES A STABILIZER ORBIT (50.8 case 3, 12.2). `Δ`
    /// is the accumulated gauge around a cycle that returned to `of`. Its
    /// rotor is read by A22's peel — a composed carriage carries no tape,
    /// so there is no spelling to read off one — and the reading is
    /// `cech::classify`'s, the one classifier both organs use.
    ///
    /// Filing it here is what turns the walk into content: the engine does
    /// not step out of the loop, it EVALUATES the loop, and the loop
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
        let hue = CycleType::of(&vertex_gauge(v));
        let Fill::Congruent(g, ell) = crate::transducer::cech::classify(&word.rotor(), delta, hue)
        else {
            return;
        };
        let Some(class) = g.class_key() else { return };
        // `sigma` is EMPTY and that is the reading, not a placeholder: a
        // stabilizer orbit at one vertex permutes no hues between vertices,
        // so there is no generator of a hue cycle to record. The period is
        // the rotor's own order and the chord is the face that closed it.
        self.torsion.insert(
            class,
            crate::transducer::sentence::FiledLoop { sigma: Vec::new(), period: ell, chord },
        );
    }

    pub fn unfold(&mut self, measure: Measure) -> u16 {
        let idx = self.vertices.len() as u16;
        self.vertices.push(Vertex::Unfolded { measure });
        idx
    }
}
