// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! RECOGNITION — the unified geometric orbit test (Phase-1 inversion).
//!
//! This replaces the flat `sensory_verdict` reflex (four `*_cells` table
//! lookups). Recognition is ONE group action: a demand body is recognised as a
//! transform of a known body iff a versor `M` carries one onto the other —
//! `∃ M s.t. M σ_demand M̃ = σ_known` —
//! judged by TOPOLOGICAL CLOSURE (the sphere lands exactly on the target — the
//! residual coboundary closes), NOT byte-replay. Exact-ℚ throughout; no digest,
//! no bucket. The base fiber is translation (matching `field`'s m=2 displacement
//! fiber); rotation and scale are ADDED fibers as dimensional resolution demands
//! (the D4 and dilation arms), never new lookup tables.

use crate::transducer::blade::Blade;
use crate::transducer::cga::{self, Mv};

/// Recognise body `a` → body `b` as a pure TRANSLATION: build the translator
/// versor from the exact centroid displacement and accept iff it CLOSES —
/// `T σ_a T̃ = σ_b` exactly (topological soundness, the body coincides). `None`
/// is an honest refusal: the motion is not a translation (it carries a rotation
/// or scale the base fiber cannot express — that is the demand for a new fiber,
/// not a wrong answer).
pub fn recognize_translation(a: &Blade, b: &Blade) -> Option<Mv> {
    let (ax, ay) = a.centroid()?;
    let (bx, by) = b.centroid()?;
    let t = cga::translator(&(bx - ax), &(by - ay));
    (t.sandwich(&a.spatial) == b.spatial).then_some(t)
}

/// Recognise a whole scene `a → b` as ONE coherent rigid translation — the
/// orbit test. Bodies correspond by their rigid-motion invariant (zero spatial
/// search); the candidate versor comes from the first corresponded pair and must
/// CLOSE on every pair. `Some(M)` is recognition of a single motion carrying the
/// entire scene; `None` means no single translation closes it (the residual is
/// genuine — a germ or a higher-fiber transform is owed, per Law V, not
/// suppressed).
pub fn recognize_scene(a: &[Blade], b: &[Blade]) -> Option<Mv> {
    if a.is_empty() || a.len() != b.len() {
        return None;
    }
    // Correspond by invariant orbit: sort each side by signature, require the
    // ensembles' invariant sequences to match, then pair positionally.
    let mut ai: Vec<usize> = (0..a.len()).collect();
    let mut bi: Vec<usize> = (0..b.len()).collect();
    ai.sort_by(|&x, &y| a[x].invariant().cmp(&a[y].invariant()));
    bi.sort_by(|&x, &y| b[x].invariant().cmp(&b[y].invariant()));
    for k in 0..a.len() {
        if a[ai[k]].invariant() != b[bi[k]].invariant() {
            return None; // the ensembles are not the same bodies up to rigid motion
        }
    }
    // One versor must carry every corresponded body — a single coherent motion.
    let t = recognize_translation(&a[ai[0]], &b[bi[0]])?;
    for k in 0..a.len() {
        if t.sandwich(&a[ai[k]].spatial) != b[bi[k]].spatial {
            return None;
        }
    }
    Some(t)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::blade::{lift, transduce};

    /// A translated body is recognised: the translator closes exactly.
    #[test]
    fn a_translation_is_recognised() {
        let a = lift(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)], 0);
        let b = lift(&[(0, 3, 1), (0, 4, 1), (1, 3, 1), (1, 4, 1)], 1);
        let m = recognize_translation(&a, &b).expect("a pure translation is recognised");
        assert_eq!(m.sandwich(&a.spatial), b.spatial, "the body lands exactly");
    }

    /// A body that changed SHAPE (not a rigid translate) is honestly refused —
    /// the coboundary does not close; the residual stands (Law V).
    #[test]
    fn a_shape_change_is_refused() {
        let a = lift(&[(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)], 0); // 2×2 square
        let b = lift(&[(0, 0, 1), (0, 1, 1), (0, 2, 1), (0, 3, 1)], 1); // 1×4 bar
        assert!(recognize_translation(&a, &b).is_none(), "not a translation — refused");
    }

    /// A whole scene translated coherently is recognised as ONE motion.
    #[test]
    fn a_coherent_scene_translation_is_one_motion() {
        let a = transduce(&[(0, 0, 1), (5, 5, 2)], 0);
        let b = transduce(&[(2, 3, 1), (7, 8, 2)], 1); // both shifted by (2,3)
        assert!(recognize_scene(&a, &b).is_some(), "one coherent translation");
    }

    /// Two bodies shifted by DIFFERENT vectors is not a single rigid motion —
    /// no one versor closes both, so recognition refuses (the residual is real).
    #[test]
    fn incoherent_shifts_refuse() {
        let a = transduce(&[(0, 0, 1), (5, 5, 2)], 0);
        let b = transduce(&[(2, 3, 1), (9, 9, 2)], 1); // different shifts
        assert!(recognize_scene(&a, &b).is_none(), "no single motion — refused");
    }
}
