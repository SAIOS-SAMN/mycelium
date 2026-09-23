// 59.3, 62, 72.1, 72.1b — THE TETHER between two bodies: the conformal motion
// their two nuclei's inversions compose, `V = s_b s_a`, carried whole, and the
// contact regime read off its class.

use super::*;

/// THE TETHER `V_ab = s_b s_a` (72.1, A.10 as restated; 72.1b's noun).
///
/// Each nucleus is a dual sphere with `s² = r² ≠ 0`, an invertible vector,
/// so it is a versor: the inversion in that sphere. Two nuclei are related by
/// the motion their two inversions compose, and that motion is carried WHOLE
/// and applied by the sandwich `V X Ṽ` to whatever crosses the interface. It
/// turns about the spheres' common circle where they intersect, is parabolic
/// where they touch, and is a boost or a dilation where they stand apart.
///
/// THIS REPLACES THE SPLIT (SQ1 item 1, 75). `Interface { strain, blade }`
/// took this same product and cut it by grade into a scalar pair — `num`,
/// `den2 = 4 r_a² r_b²` — and a 2-blade, and the scalar gated whether a
/// tether formed. The split was the flattening (72.1). Nothing here gates:
/// whether two bodies touch is topological (A.14, `contact_cells`), and the
/// tether is what a touching pair carries.
///
/// `None` where a centroid does not read back (rule 11).
pub fn tether(a: &Blade, b: &Blade) -> Option<Mv> {
    Some(b.nucleus()?.geo(&a.nucleus()?))
}

/// WHERE TWO BODIES STAND RELATIVE TO CONTACT — the regime of their tether,
/// read as a class of `V` where a side is asked, never carried and never a
/// gate (Axiom C as tightened; SQ1 item 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    /// `δ > 1` — disjoint exteriors: `V` hyperbolic, `⟨V⟩₀ < 0`.
    Apart,
    /// `δ = 1` — touching from OUTSIDE: `V` parabolic.
    TangentExternal,
    /// `|δ| < 1` — secant: `V` elliptic, turning about the common circle,
    /// with `δ = 0` (inversive orthogonality) its fixed point.
    Intersecting,
    /// `δ = −1` — touching from INSIDE: `V` parabolic.
    TangentInternal,
    /// `δ < −1` — strictly nested: `V` hyperbolic, `⟨V⟩₀ > 0`.
    Contained,
}

impl Contact {
    /// THE ANTIPODAL INVOLUTION `σ: s ↦ −s`, which sends `δ ↦ −δ`: a sphere
    /// and its negation are one sphere with inside and outside exchanged, so
    /// `Apart` swaps with `Contained`, the two tangencies with one another,
    /// and `Intersecting` is fixed. `σ ∘ σ = id`.
    pub fn antipodal(self) -> Contact {
        match self {
            Contact::Apart => Contact::Contained,
            Contact::TangentExternal => Contact::TangentInternal,
            Contact::Intersecting => Contact::Intersecting,
            Contact::TangentInternal => Contact::TangentExternal,
            Contact::Contained => Contact::Apart,
        }
    }

    /// Both tangencies are contact interfaces.
    pub fn is_tangent(self) -> bool {
        matches!(self, Contact::TangentExternal | Contact::TangentInternal)
    }

    /// THE REGIME OF A TETHER, exactly and with no root.
    ///
    /// `⟨V⟩₀ = s_a · s_b = (r_a² + r_b² − d²)/2` and `⟨VṼ⟩₀ = r_a² r_b²`, so
    /// `δ = −⟨V⟩₀ / √⟨VṼ⟩₀` — the projective reading of `V`'s class (Study's
    /// condition on a ray, 72.1). It is compared against `±1` by sign first
    /// and only then by the equal-degree comparison `⟨V⟩₀²` against
    /// `⟨VṼ⟩₀`, so no sign is lost and no root is taken. Scale-free: `λV`
    /// reads the same regime. `None` where `V` has no weight.
    pub fn of(v: &Mv) -> Option<Contact> {
        let zero = BigRational::zero();
        let s = v.scalar_part();
        let w = v.geo(&v.reverse()).scalar_part();
        if w <= zero {
            return None;
        }
        let s2 = &s * &s;
        Some(match (s2.cmp(&w), s < zero) {
            // `|δ| = 1`: parabolic. `δ = 1` where `⟨V⟩₀ < 0`.
            (std::cmp::Ordering::Equal, true) => Contact::TangentExternal,
            (std::cmp::Ordering::Equal, false) => Contact::TangentInternal,
            // `|δ| < 1`: elliptic.
            (std::cmp::Ordering::Less, _) => Contact::Intersecting,
            // `|δ| > 1`: hyperbolic, the side by `δ`'s sign, opposite `⟨V⟩₀`'s.
            (std::cmp::Ordering::Greater, true) => Contact::Apart,
            (std::cmp::Ordering::Greater, false) => Contact::Contained,
        })
    }
}
