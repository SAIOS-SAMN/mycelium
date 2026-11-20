// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.189 — THE ATLAS ON Sⁿ: the charts of a field's bodies read as open
//! caps of the sphere `Sⁿ = ℝⁿ ∪ {∞}`, the conformal model `Cl(n+1, 1)`(ℚ)
//! already carries. A body's cap is bounded by its nucleus (the round
//! `c + ½(c² − r²)e∞ + e₀`, ruled on #134). Nothing here reads a lattice.
//!
//! - **Where two charts meet** is the sign of their inversive product
//!   (`cga::containment`, EN1): on each other (cut or tangent), one inside
//!   the other, or apart. Every conformal map of Sⁿ keeps that reading,
//!   because it keeps the inversive product's sign; no grid, size or place
//!   enters it.
//! - **The transitions.** On each pair of charts that meet, the motion that
//!   carries one body's readings onto the other's, `g_ab`, stated by the
//!   physics (the versor between the bodies' shapes, `blade::versor_between`),
//!   never chosen here. A pair that meets with no motion stated is held
//!   unread, never dropped. These transitions are the atlas's connection
//!   (GLOSSARY, Atlas).
//! - **The gluing.** On every triangle of charts that pairwise meet, the
//!   circuit `a → b → c → a` composes to `g̃_ac · g_bc · g_ab`. Where that is
//!   a scalar, the three charts glue: the coordinates agree wherever the
//!   charts overlap (the cocycle). Otherwise the composite is the
//!   triangle's holonomy, kept whole as content (MU-TOPOLOGY §4), never
//!   removed as an error.
//!
//! Every pair and every triangle read spends one read of the stroke
//! (78.153); a stroke spent ends the reading as `None`, never a partial
//! atlas read as whole.

use crate::physics::Budget;
use crate::transducer::cga::{containment, Containment, Mv};

/// WHERE TWO CHARTS MEET on Sⁿ, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Meeting {
    /// The caps overlap: the reading of the pair's inversive product
    /// (`On`, `Inside` or `Wraps`).
    Overlap(Containment),
    /// Each cap outside the other: no overlap, no transition.
    Apart,
    /// An imaginary or flat round: no meeting reads.
    Unread,
}

/// THE MEETING OF TWO CHARTS, read off their rounds alone.
pub fn meeting(a: &Mv, b: &Mv) -> Meeting {
    match containment(a, b) {
        Containment::Apart => Meeting::Apart,
        Containment::Unread => Meeting::Unread,
        c => Meeting::Overlap(c),
    }
}

/// ONE EDGE OF THE ATLAS: charts `a < b` that overlap, how they meet, and
/// the motion carrying `a`'s readings onto `b`'s where the physics stated
/// one.
#[derive(Clone, Debug)]
pub struct Transition {
    pub a: usize,
    pub b: usize,
    pub meeting: Containment,
    pub g: Option<Mv>,
}

/// THE ATLAS'S OVERLAPS: the charts held, by index, and an edge per pair
/// that overlaps.
#[derive(Clone, Debug, Default)]
pub struct Overlaps {
    pub charts: usize,
    pub edges: Vec<Transition>,
}

impl Overlaps {
    /// The transition on the pair `(a, b)`, `a < b`, where the charts overlap.
    pub fn edge(&self, a: usize, b: usize) -> Option<&Transition> {
        self.edges.iter().find(|t| t.a == a && t.b == b)
    }
}

/// THE OVERLAPS OF THE CHARTS bounded by `rounds`: one read per pair.
/// `motion(a, b)` states the motion carrying `a`'s readings onto `b`'s; it
/// is asked only where the two overlap. `None` where the stroke ran out.
pub fn overlaps(rounds: &[Mv], mut motion: impl FnMut(usize, usize) -> Option<Mv>, budget: &mut Budget) -> Option<Overlaps> {
    let mut edges = Vec::new();
    for a in 0..rounds.len() {
        for b in a + 1..rounds.len() {
            if !budget.spend() {
                return None;
            }
            if let Meeting::Overlap(c) = meeting(&rounds[a], &rounds[b]) {
                edges.push(Transition { a, b, meeting: c, g: motion(a, b) });
            }
        }
    }
    Some(Overlaps { charts: rounds.len(), edges })
}

/// HOW A TRIANGLE OF CHARTS GLUES.
#[derive(Clone, Debug, PartialEq)]
pub enum Gluing {
    /// The circuit composes to a scalar: the coordinates agree on the
    /// overlaps.
    Glued,
    /// The circuit's composite, `g̃_ac · g_bc · g_ab`, kept whole.
    Holonomy(Mv),
    /// An edge of the triangle carries no stated motion.
    Unread,
}

/// ONE TRIANGLE: three charts that pairwise overlap, and its gluing.
#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub gluing: Gluing,
}

/// EVERY TRIANGLE OF THE ATLAS and its gluing: one read per triangle.
/// `None` where the stroke ran out.
pub fn gluing(atlas: &Overlaps, budget: &mut Budget) -> Option<Vec<Triangle>> {
    let one = Mv::scalar(num_rational::BigRational::from_integer(1.into()));
    let mut out = Vec::new();
    for ab in &atlas.edges {
        for bc in atlas.edges.iter().filter(|e| e.a == ab.b) {
            let Some(ac) = atlas.edge(ab.a, bc.b) else { continue };
            if !budget.spend() {
                return None;
            }
            let reading = match (&ab.g, &bc.g, &ac.g) {
                (Some(g_ab), Some(g_bc), Some(g_ac)) => {
                    let around = g_ac.reverse().geo(g_bc).geo(g_ab);
                    match around.projective_eq(&one) {
                        true => Gluing::Glued,
                        false => Gluing::Holonomy(around),
                    }
                }
                _ => Gluing::Unread,
            };
            out.push(Triangle { a: ab.a, b: ab.b, c: bc.b, gluing: reading });
        }
    }
    Some(out)
}
