// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ČECH GLUING — the manifold is charts (hyphas) glued on their overlaps by
//! ASYMMETRIC coboundaries. When two charts both learned a germ for the same
//! object, their views are related by a TRANSITION element; the transition is
//! antisymmetric by construction (`g_YX = g_XY⁻¹` — the inverse deduction). A
//! triangle of charts (a 2-simplex of the Čech nerve — mycelium's K₃ mesh) glues
//! into a single sheaf iff the COCYCLE closes: `g_ab ∘ g_bc ∘ g_ca = 1` (δg = 0).
//! A non-identity composite is CURVATURE (δσ ≠ 0) — the exact holonomy of a
//! dialect/equivocation the charts must reconcile. This is the rigorous form of
//! the mesh's cross-node consilience: consilient charts have vanishing holonomy;
//! an equivocation shows up as a nonzero obstruction the blind court adjudicates.

use crate::transducer::element::Element;

/// The TRANSITION carrying chart X's view to chart Y's, from each chart's germ for
/// the shared object: `g_XY = g_Y ∘ g_X⁻¹` (the change of view). Antisymmetric by
/// construction — `g_YX = g_XY⁻¹`. `None` if X's germ is not invertible (a
/// coboundary/residual chart has no group view-change; that overlap is handled by
/// the additive coboundary, not this group transition).
pub fn transition(g_x: &Element, g_y: &Element) -> Option<Element> {
    Some(g_y.compose(&g_x.solution()?))
}

/// The HOLONOMY around a triangle of charts — the composite transition
/// `g_ab ∘ g_bc ∘ g_ca`. Identity ⇔ the charts GLUE (one sheaf); otherwise it is
/// the exact obstruction (the equivocation) the charts must reconcile.
pub fn holonomy(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> Element {
    g_ab.compose(g_bc).compose(g_ca)
}

/// Does a triangle of charts GLUE — the Čech cocycle `δg = 0`? True iff the
/// holonomy around the overlap is trivial (the transitions are the coboundary of a
/// consistent global assignment). False = CURVATURE (δσ ≠ 0), a dialect.
pub fn glues(g_ab: &Element, g_bc: &Element, g_ca: &Element) -> bool {
    holonomy(g_ab, g_bc, g_ca).is_identity()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_transform;
    use crate::transducer::transform::solve_transform;

    // A few invertible motion germs to act as per-chart views.
    fn motion(dc: i64) -> Element {
        let a = [(0, 0, 1u8), (0, 1, 1), (1, 0, 1)];
        let b = [(0, dc, 1u8), (0, dc + 1, 1), (1, dc, 1)];
        from_transform(&solve_transform(&a, &b).unwrap())
    }

    /// A TRANSITION is antisymmetric: `g_XY ∘ g_YX = 1` (`g_YX = g_XY⁻¹`).
    #[test]
    fn a_transition_and_its_reverse_cancel() {
        let (gx, gy) = (motion(3), motion(6));
        let f = transition(&gx, &gy).unwrap();
        let r = transition(&gy, &gx).unwrap();
        assert!(f.compose(&r).is_identity(), "g_XY ∘ g_YX = 1 — asymmetric coboundary");
    }

    /// CONSILIENT charts glue trivially: if all three learned the SAME germ, every
    /// transition is the identity and the cocycle closes.
    #[test]
    fn consilient_charts_glue() {
        let g = motion(3);
        let id = transition(&g, &g).unwrap(); // g ∘ g⁻¹ = 1
        assert!(id.is_identity());
        assert!(glues(&id, &id, &id), "identical views → one sheaf");
    }

    /// A GLOBAL SECTION always glues: transitions built as `g_Y ∘ g_X⁻¹` from ANY
    /// per-chart germs are a coboundary, so δδ = 0 — the cocycle closes regardless
    /// of the germs. This is the consistent-assignment (no-dialect) case.
    #[test]
    fn a_global_section_always_glues() {
        let (ga, gb, gc) = (motion(3), motion(6), motion(9));
        let g_ab = transition(&ga, &gb).unwrap();
        let g_bc = transition(&gb, &gc).unwrap();
        let g_ca = transition(&gc, &ga).unwrap();
        assert!(glues(&g_ab, &g_bc, &g_ca), "a coboundary always satisfies δδ = 0");
    }

    /// An EQUIVOCATION is CURVATURE: transitions that do NOT come from a global
    /// section leave a nonzero holonomy — the cocycle fails, the dialect is caught.
    #[test]
    fn an_equivocation_shows_up_as_curvature() {
        // a,b agree and b,c agree (identity), but c,a carry a real view-change:
        // no global assignment reproduces this, so the triangle does not glue.
        let g = motion(3);
        let id = transition(&g, &g).unwrap();
        let disagree = transition(&motion(3), &motion(6)).unwrap(); // ≠ identity
        assert!(!disagree.is_identity(), "the disagreeing edge is nontrivial");
        assert!(!glues(&id, &id, &disagree), "a lone dialect edge → curvature (δσ ≠ 0)");
        assert!(!holonomy(&id, &id, &disagree).is_identity(), "the holonomy is the exact obstruction");
    }
}
