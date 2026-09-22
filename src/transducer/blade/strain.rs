// 59.3, 64, 65 — THE STRAIN AND THE INTERFACE between two bodies: the
// inversive strain carried as a ratio, its five contact regimes, the whole
// product of two nuclei, and the field `condense` reads them into.

use super::*;

/// T2 — THE INVERSIVE STRAIN BETWEEN TWO BODIES, CARRIED AS A RATIO AND
/// NEVER EVALUATED (59.3, Axiom C).
///
/// Two bodies with centroid `P` and radius of gyration `r` are related by
/// the inversive distance of their spheres,
///
/// ```text
///     δ = (‖P_a − P_b‖² − r_a² − r_b²) / (2 r_a r_b)
/// ```
///
/// and the kinematic charge of the chord between them is `μ = (δ − 1)/2`,
/// which is `⟨R⟩₀²/⟨RR̃⟩₀ − 1` for `R = 1 − v_b v_a`. `μ = 0` is exact
/// tangency, `μ < 0` interpenetration, `μ > 0` separation.
///
/// `μ` IS NOT RATIONAL AND NEVER REACHES THE WIRE. The denominator wants
/// `r_a r_b = √(r_a² r_b²)`, and [`Blade::radius2`] carries `r²`, so law 1
/// would refuse the root as inexact on nearly every body. What IS rational
/// is the pair: the numerator, and the denominator SQUARED. Every question
/// the engine asks of the strain is a comparison, and a comparison of two
/// ratios is a comparison of cross products — which is the projective
/// discipline stated once and obeyed here.
///
/// `num`'S SIGN IS ORTHOGONALITY, NOT TANGENCY. A first reading of this
/// had `num = 0` as the contact locus; it is not. `num = 0` is `δ = 0`,
/// two spheres meeting at RIGHT ANGLES, while tangency is `δ = 1`, which
/// is `num = √den2`. Reading the sign of `num` as the contact side would
/// put the horizon in the wrong place by a whole unit of `δ`, so the side
/// is read by [`Strain::side`] and never off `num` alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Strain {
    /// `‖P_a − P_b‖² − r_a² − r_b²` — the sign is `δ`'s, so it says which
    /// side of ORTHOGONALITY the pair is on.
    pub num: BigRational,
    /// `4 r_a² r_b²`, strictly positive: `Blade::radius2` is `> 0` by
    /// construction, so the pair never carries a zero denominator.
    pub den2: BigRational,
}

/// WHERE TWO BODIES STAND RELATIVE TO CONTACT. The reading `μ` would give
/// if `μ` could be written down, taken without writing it down.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    /// `δ > 1` — disjoint exteriors, `d > r_a + r_b`.
    Apart,
    /// `δ = 1` — touching from OUTSIDE, `d = r_a + r_b`. Measure-zero over
    /// ℚ, so it is the locus a threshold is taken about and never the test
    /// itself (59.4).
    TangentExternal,
    /// `|δ| < 1` — secant, `|r_a − r_b| < d < r_a + r_b`. Self-dual under
    /// the antipodal map, with `δ = 0` (inversive orthogonality) as its
    /// fixed point.
    Intersecting,
    /// `δ = −1` — touching from INSIDE, `d = |r_a − r_b|`. A genuine
    /// codimension-1 contact interface, not overlap.
    TangentInternal,
    /// `δ < −1` — strictly nested, `d < |r_a − r_b|`.
    Contained,
}

impl Contact {
    /// THE ANTIPODAL INVOLUTION `σ: s ↦ −s`, which sends `δ ↦ −δ`.
    ///
    /// A conformal sphere and its negation are the same sphere with INSIDE
    /// AND OUTSIDE EXCHANGED, so the involution swaps `Apart` with
    /// `Contained` and the two tangencies with one another, and fixes
    /// `Intersecting`. `σ ∘ σ = id`, asserted in the axioms.
    pub fn antipodal(self) -> Contact {
        match self {
            Contact::Apart => Contact::Contained,
            Contact::TangentExternal => Contact::TangentInternal,
            Contact::Intersecting => Contact::Intersecting,
            Contact::TangentInternal => Contact::TangentExternal,
            Contact::Contained => Contact::Apart,
        }
    }

    /// IS THIS A CONTACT INTERFACE? Both tangencies are, and treating one
    /// as contact and the other as penetration is what lost the boundary's
    /// symmetry.
    pub fn is_tangent(self) -> bool {
        matches!(self, Contact::TangentExternal | Contact::TangentInternal)
    }
}

/// T4 — THE INTERFACE BETWEEN TWO NUCLEI, WHOLE (64, ruled 2026-09-22).
///
/// SCALAR COLLAPSE IS DISALLOWED. The geometric product of two nuclei is
///
/// ```text
///     s_a s_b = s_a · s_b  +  s_a ∧ s_b
/// ```
///
/// and a tether that carries only the first term has thrown the interaction
/// away. `s_a · s_b` is ONE rational: it says how close, and nothing about
/// where, in what plane, or through which pencil. `s_a ∧ s_b` is a 2-blade
/// of `⋀²(ℚ^{4,1})` — TEN independent rational coordinates — carrying the
/// radical hyperplane, the orientation of the interface, and the coaxal
/// pencil of spheres through the two bodies' common boundary.
///
/// So the scalar GATES and the blade IS THE INTERFACE. `condense` asks the
/// first whether a tether forms and hands the second to the 1-cell, which
/// enters `C¹` with its whole geometric orientation rather than as an
/// unoriented link.
///
/// A LONE TETHER IS ALWAYS SIMPLE, and that is a theorem rather than a
/// limitation: `(s_a ∧ s_b) ∧ (s_a ∧ s_b) = 0` identically, because a
/// repeated factor kills the wedge. So `I₂ = 0` on every isolated
/// interface, and the luminal screw can only appear where the NETWORK
/// composes — a relative motor coupling a turn with an off-plane
/// displacement, two tethers spanning four independent directions, or a
/// closed circuit accumulating non-coplanar bivectors (59.7, 60.4). The
/// blade is the local interface; the screw is the global torsion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interface {
    /// Grade 0. The gate: `within(μ_c)` decides whether a tether forms.
    pub strain: Strain,
    /// Grade 2. The coaxal pencil's own blade — the interface the
    /// transition refracts across, carried into `C¹` entire.
    pub blade: crate::transducer::cga::Mv,
}

impl Interface {
    /// The whole product of two nuclei, split by grade. `None` where a
    /// centroid does not read back (rule 11).
    pub fn between(a: &Blade, b: &Blade) -> Option<Interface> {
        let product = a.nucleus()?.geo(&b.nucleus()?);
        Some(Interface { strain: Strain::between(a, b)?, blade: product.grade(2) })
    }

    /// Is this interface inside the horizon? The SCALAR gates; the blade is
    /// what the tether then carries.
    pub fn within(&self, mu_c: &BigRational) -> bool {
        self.strain.within(mu_c)
    }
}

/// T4 — THE PAIRWISE STRAIN FIELD, EVALUATED EXACTLY.
///
/// THIS IS A FIELD, NOT A SEARCH. A search explores a space of
/// possibilities for one that fits an ad-hoc score; this computes the
/// interaction between every pair of bodies present, the way an N-body
/// force calculation does, and every value it produces is an exact rational
/// the physics defines. Rule 1's "never a pair loop over the pieces" was
/// written for INCIDENCE BY SHARED SITE, which a keyed occupancy answers in
/// one linear pass — there is no keyed occupancy that answers a continuous
/// strain, because the relation is not "share a cell".
///
/// The cost does not bind: the corpus's largest cover holds 37 charts, so
/// the field is 666 unordered pairs of exact rational products. Building a
/// tree, a bucket or a lattice to avoid them would cost a standing
/// constraint to optimise an arithmetic cost that does not exist.
///
/// Ordered `i < j` so each interface is computed once and the reading does
/// not depend on the order the bodies were offered.
pub fn condense(bodies: &[Blade], mu_c: &BigRational) -> Vec<(usize, usize, Interface)> {
    let mut out = Vec::new();
    for i in 0..bodies.len() {
        for j in i + 1..bodies.len() {
            let Some(face) = Interface::between(&bodies[i], &bodies[j]) else {
                continue;
            };
            if face.within(mu_c) {
                out.push((i, j, face));
            }
        }
    }
    out
}

impl Strain {
    /// The strain between two bodies, or `None` where a centroid does not
    /// read back (rule 11: refused, never defaulted).
    pub fn between(a: &Blade, b: &Blade) -> Option<Strain> {
        let (a1, a2, a3) = a.centroid()?;
        let (b1, b2, b3) = b.centroid()?;
        let (d1, d2, d3) = (&a1 - &b1, &a2 - &b2, &a3 - &b3);
        let sep2 = &d1 * &d1 + &d2 * &d2 + &d3 * &d3;
        let (ra2, rb2) = (a.radius2(), b.radius2());
        let four = BigRational::from_integer(BigInt::from(4));
        Some(Strain { num: sep2 - &ra2 - &rb2, den2: four * ra2 * rb2 })
    }

    /// `δ` COMPARED AGAINST A RATIONAL, EXACTLY AND WITHOUT A ROOT.
    ///
    /// `δ ≤ k` is `num ≤ k·√den2`. Both sides are compared by SIGN first
    /// and only then by squaring, because squaring loses the sign and a
    /// test that squares first answers the wrong question on a negative
    /// numerator. `√den2 > 0` always, which is what makes the case split
    /// total.
    fn delta_at_most(&self, k: &BigRational) -> bool {
        let zero = BigRational::from_integer(BigInt::from(0));
        match (self.num > zero, *k > zero) {
            // Both positive: `num ≤ k√den2 ⟺ num² ≤ k²·den2`.
            (true, true) => &self.num * &self.num <= k * k * &self.den2,
            // A positive numerator is never below a non-positive bound.
            (true, false) => false,
            // A non-positive numerator is always below a positive bound.
            (false, true) => true,
            // Both non-positive: the inequality REVERSES under squaring.
            (false, false) => &self.num * &self.num >= k * k * &self.den2,
        }
    }

    /// Is this pair inside the horizon `μ ≤ μ_c`? `μ = (δ−1)/2`, so the
    /// bound on `δ` is `2μ_c + 1` and the comparison is the one above.
    pub fn within(&self, mu_c: &BigRational) -> bool {
        let one = BigRational::from_integer(BigInt::from(1));
        let two = BigRational::from_integer(BigInt::from(2));
        self.delta_at_most(&(two * mu_c + one))
    }

    /// WHERE THE PAIR STANDS ON THE INVERSIVE SPECTRUM — five regimes, read
    /// exactly and without a root.
    ///
    /// ```text
    ///  Contained   TangentInt     Intersecting    TangentExt    Apart
    ///   δ < −1       δ = −1        |δ| < 1         δ = 1        δ > 1
    ///  ─────────]──────|──────[───────●───────]──────|──────[─────────
    ///                              δ = 0
    ///                           (orthogonal)
    /// ```
    ///
    /// A FIRST CUT HAD THREE ARMS AND COLLAPSED A CONTACT LOCUS. It read
    /// `Tangent` only for `δ = 1` and folded everything below into
    /// `Through`, so `δ = −1` — INTERNAL TANGENCY, where the spheres touch
    /// at one point from inside, `d = |r_a − r_b|` — was reported as
    /// interpenetration. That is a scalar collapse committed in an enum
    /// rather than in a metric, and it is exactly half the contact boundary
    /// mislabelled.
    ///
    /// The fixture hid it: a body against ITSELF sits at `δ = −1`, where
    /// "touching from inside" and "fully overlapping" are both defensible,
    /// so the assertion passed while testing what was written rather than
    /// what is true. For two DIFFERENT bodies at `d = |r_a − r_b|` they are
    /// not the same statement at all.
    ///
    /// THE COMPARISONS ARE `δ` AGAINST `±1`, and both go through
    /// `delta_at_most`, which splits on sign before squaring — so the
    /// spectrum is read with no root anywhere and no sign lost.
    pub fn side(&self) -> Contact {
        let one = BigRational::from_integer(BigInt::from(1));
        let zero = BigRational::from_integer(BigInt::from(0));
        // `|δ| = 1` exactly: `num² = den2`. The SIGN of `num` then says
        // which of the two tangencies it is, because `√den2 > 0` always.
        if &self.num * &self.num == self.den2 {
            return match self.num > zero {
                true => Contact::TangentExternal,
                false => Contact::TangentInternal,
            };
        }
        match (self.delta_at_most(&one), self.delta_at_most(&-one)) {
            // δ ≤ −1 and not equal to it, so δ < −1.
            (_, true) => Contact::Contained,
            // −1 < δ ≤ 1 and not equal to it, so |δ| < 1.
            (true, false) => Contact::Intersecting,
            // δ > 1.
            (false, false) => Contact::Apart,
        }
    }
}
