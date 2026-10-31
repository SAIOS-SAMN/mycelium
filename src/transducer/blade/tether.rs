// 59.3, 62, 72.1, 72.1b — THE TETHER between two bodies: the conformal motion
// the inversions in the two nuclei compose, `V = s_b s_a`, carried whole, and the
// contact regime read off its class.

use super::*;

/// THE TETHER `V_ab = s_b s_a` (72.1, A.10 as restated; 72.1b's noun).
///
/// Each nucleus is a dual sphere with `s² = r² ≠ 0`, an invertible vector,
/// so it is a versor: the inversion in that sphere. Two nuclei are related by
/// the motion the two nuclei's inversions compose, and that motion is carried WHOLE
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
    Some(tether_of(&a.nucleus()?, &b.nucleus()?))
}

/// THE TETHER BETWEEN TWO NUCLEI, `V = s_b s_a` — the one product
/// [`tether`] and the face's interface both read (BQ5b, law 12).
pub fn tether_of(sa: &Mv, sb: &Mv) -> Mv {
    sb.geo(sa)
}

/// WHERE TWO BODIES STAND RELATIVE TO CONTACT — the regime of the bodies' tether,
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

    /// THE REGIME OF A TETHER, read off its ONE MODE (MR1, 78.28), exactly
    /// and with no root.
    ///
    /// `V = s_b s_a` is a product of two vectors, so its plane `B = ⟨V⟩₂ =
    /// s_a ∧ s_b` is simple and MR1's split of it is `B` itself: one mode,
    /// whose class is the sign of `B²` — elliptic below zero, parabolic at
    /// it, hyperbolic above. That is the regime: the spheres cross, touch,
    /// or stand apart, and on the hyperbolic side `⟨V⟩₀ = s_a · s_b`'s sign
    /// says whether one holds the other. It read `⟨V⟩₀²` against `V Ṽ`,
    /// which is `⟨V⟩₀² − B²` — the same regime by a weight formed to decide
    /// it (VA1). Scale-free: `λV` reads the same regime. `None` where `V`
    /// does not act as a motion, and where one sphere is imaginary
    /// (`⟨V⟩₀² ≤ B²`), which has no real regime.
    pub fn of(v: &Mv) -> Option<Contact> {
        if !crate::transducer::cga::is_versor(v) {
            return None;
        }
        let zero = BigRational::zero();
        let s = v.scalar_part();
        let b = v.grade(2);
        let mode = b.geo(&b).scalar_part();
        // THE WEIGHT'S SIGN, read in the algebra (78.59): `V Ṽ = s² − B B`
        // for a scalar and one simple blade, so a tether of no positive weight
        // reads no contact — never `s · s` compared as a squared scalar.
        if v.geo(&v.reverse()).scalar_part() <= zero {
            return None;
        }
        Some(match (mode.cmp(&zero), s < zero) {
            // Parabolic: the spheres touch, from outside where `⟨V⟩₀ < 0`.
            (std::cmp::Ordering::Equal, true) => Contact::TangentExternal,
            (std::cmp::Ordering::Equal, false) => Contact::TangentInternal,
            // Elliptic: they cross.
            (std::cmp::Ordering::Less, _) => Contact::Intersecting,
            // Hyperbolic: apart, or one holding the other, by `⟨V⟩₀`'s sign.
            (std::cmp::Ordering::Greater, true) => Contact::Apart,
            (std::cmp::Ordering::Greater, false) => Contact::Contained,
        })
    }
}
