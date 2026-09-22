//! 54.7 R3/R4 — the spin group over ℚ: reflection factorization, the
//! Cayley transform, the split of a generator, and the transvections of ℍ⁴.

use super::group::is_versor;
use super::multivector::{one, Mv, E1, E2, E3, EM, EP};
use super::planes::TRANSVECTION_PLANES;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// A VECTOR OF `ℚ^{4,1}` THAT REFLECTS: `v² ∈ ℚ*`, so `v` is invertible and
/// `x ↦ −v x v⁻¹` is a reflection in its orthogonal hyperplane.
///
/// A null vector (`v² = 0`) reflects nothing — it has no inverse — and is
/// refused rather than returned as a factor that would silently annihilate
/// the versor it was multiplied into (rule 11).
pub fn reflector_vector(
    x1: &BigRational,
    x2: &BigRational,
    x3: &BigRational,
    xp: &BigRational,
    xm: &BigRational,
) -> Option<Mv> {
    let v = Mv::blade(E1, x1.clone())
        .add(&Mv::blade(E2, x2.clone()))
        .add(&Mv::blade(E3, x3.clone()))
        .add(&Mv::blade(EP, xp.clone()))
        .add(&Mv::blade(EM, xm.clone()));
    // v² is read by the geometric product, never as Σxᵢ² (rule 11): the
    // metric's signs live in the product and nowhere else.
    let sq = v.geo(&v).scalar_part();
    (!sq.is_zero()).then_some(v)
}

/// THE FULL SPIN GROUP OVER ℚ, BY REFLECTION FACTORIZATION (54.7 R4).
///
/// ```text
///     Ω = v₁ v₂ ⋯ v_{2k} ,    vᵢ² ∈ ℚ*
/// ```
///
/// Cartan–Dieudonné: every element of the group is a product of an even
/// number of reflections, and this is how it is BUILT here rather than
/// parameterised.
///
/// WHY NOT `Ω = a + b·B` ON ONE BLADE. That spans `ℚ[B] ≅ ℚ ⊕ ℚ`, a
/// two-dimensional split-complex slice of a ten-dimensional bivector space,
/// and its generator is a SIMPLE bivector — rank one, so
///
/// ```text
///     I₂ = ⟨B ∧ B⟩₄ = 0
/// ```
///
/// identically. A simple generator turns in one plane and translates along
/// one track: no screw, no helicoidal coupling, no simultaneous rotation
/// and transvection. Forcing every motion through that form collapses the
/// coupled degrees of freedom into uncoupled one-dimensional coordinate
/// tracks, which is the Anti-Flattening Law's clause 5 exactly — restricted
/// dimensional capacity — one dimension up from collapsing `Λ²(ℝ⁴)` to a
/// single blade.
///
/// A product of reflections carries no such restriction: `⟨Ω⟩₂` is a
/// general bivector, `I₂` is free to be non-zero, and a screw displacement
/// is an ordinary element rather than something the representation cannot
/// hold.
///
/// The versor condition is CHECKED, not assumed: `Ω Ω̃` must be a non-zero
/// SCALAR, so `⟨Ω Ω̃⟩₂` and `⟨Ω Ω̃⟩₄` both vanish. `is_versor` reads exactly
/// that through the geometric product.
pub fn versor_of_reflections(vs: &[Mv]) -> Option<Mv> {
    if vs.is_empty() || vs.len() % 2 != 0 {
        return None;
    }
    let mut out = Mv::scalar(one());
    // index: the factors the caller handed, in the order they compose —
    // reflections do not commute, and the order is the motion.
    for v in vs {
        if v.geo(v).scalar_part().is_zero() {
            return None;
        }
        out = out.geo(v);
    }
    is_versor(&out)?;
    Some(out)
}

/// FINITE EVOLUTION OF A GENERATOR, EXACTLY OVER ℚ — the rational Cayley
/// transform (54.7 R3, on the operator's ruling).
///
/// ```text
///     Ω = (1 − B)(1 + B)⁻¹  ∈  Spin⁺(3,1)(ℚ)
/// ```
///
/// A BIVECTOR IS NOT A MOTION. `B ∈ ⋀²ℚ^{4,1} ≅ 𝔰𝔭𝔦𝔫(3,1)` is the
/// infinitesimal element — a rate of change at one point of the tangent
/// space — and an engine that transports by carrying `B` around is working
/// in the tangent space, where every transformation is a linear
/// approximation of the motion it stands for. That is where the flattening
/// happens: the mixed generators `eᵢ ∧ e₋` get handled as independent
/// Cartesian offsets and the translational shear decouples from the ambient
/// curvature.
///
/// The finite action is the group element, and on the quadric it is
/// hyperbolic parallel transport along a geodesic:
///
/// ```text
///     dp/dτ = ½(Bp − pB)   ⟹   p(τ) = Ω(τ) p(0) Ω̃(τ)
/// ```
///
/// WHY NOT `exp`. `exp(½τB) = cosh(τ/2) + B sinh(τ/2)` leaves ℚ for every
/// rational `τ` worth having, and the two ways round that — floating point,
/// or pinning `Ω` to one blade as `a + bB` — destroy exactness and the
/// grade-4 coupling respectively. Cayley leaves neither behind: it is two
/// geometric products and an inverse, all rational, and it carries a
/// GENERAL bivector, so `I₂ = ⟨B∧B⟩₄` survives the passage and a screw
/// stays a screw.
///
/// `None` where `1 + B` is not invertible — the transform's pole, which is
/// the rotation by π that Cayley genuinely cannot reach, refused rather
/// than approximated (rule 11).
pub fn cayley(b: &Mv) -> Option<Mv> {
    let bivector = b.grade(2);
    if !b.sub(&bivector).is_zero() {
        return None;
    }
    let one_mv = Mv::scalar(one());
    let plus = one_mv.add(&bivector);
    let minus = one_mv.sub(&bivector);
    let omega = minus.geo(&inverse_even(&plus)?);
    is_versor(&omega)?;
    Some(omega)
}

/// THE EXACT INVERSE OF AN EVEN ELEMENT `u = ⟨u⟩₀ + ⟨u⟩₂ + ⟨u⟩₄`.
///
/// `versor_inverse` is `ũ / (u ũ)` and needs `u ũ` to be a pure SCALAR —
/// which holds for a versor and fails for the very elements this exists
/// for. With `B` non-simple,
///
/// ```text
///     (1 + B)(1 + B)~ = 1 − B² = (1 − ⟨B²⟩₀) − ⟨B²⟩₄
/// ```
///
/// carries a grade-4 part, so `1 + B` is invertible in the algebra and is
/// NOT a versor. Reaching for the versor inverse there refuses every
/// coupled generator — which is the flattening returning by the back door:
/// the only motions that would pass are the simple ones.
///
/// The general inverse in `Cl(4,1)`: with `w = u ũ = s + Q`, `Q` a
/// 4-vector, take `w̄ = s − Q`. `Q` commutes with scalars and `Q²` is a
/// scalar in five dimensions, so `w w̄ = s² − Q²` is scalar, and
///
/// ```text
///     u⁻¹ = ũ w̄ / ⟨w w̄⟩₀
/// ```
///
/// verified by `u · ũ w̄ / ⟨w w̄⟩₀ = w w̄ / ⟨w w̄⟩₀ = 1`. Exact throughout,
/// and `None` where the denominator vanishes — a genuine non-invertible
/// element, refused rather than divided by (rule 4, rule 11).
pub fn inverse_even(u: &Mv) -> Option<Mv> {
    let rev = u.reverse();
    let w = u.geo(&rev);
    let bar = w.grade(0).sub(&w.grade(4));
    if !w.sub(&w.grade(0).add(&w.grade(4))).is_zero() {
        // `u ũ` carries a grade this closed form does not cover; refused
        // rather than silently inverted by a formula that does not apply.
        return None;
    }
    let d = w.geo(&bar);
    let ds = d.scalar_part();
    if ds.is_zero() || !d.sub(&Mv::scalar(ds.clone())).is_zero() {
        return None;
    }
    Some(rev.geo(&bar).scale(&(one() / ds)))
}

// THE PATH-ORDERED TRANSPORT AROUND A CLOSED CHAIN, `Ω_∂e² = 𝒫 ∏ Ω_e`,
// STOOD HERE AND IS RETIRED (54.7, ruled 2026-09-21).
//
// It was written, never called, and computed the boundary product along a
// SINGLE channel — the `Cl(4,1)` carriage alone. A transition of this
// complex has two halves, the orientation rotor in `Cl(4,0)` and the
// spatial transvection in `Spin⁺(3,1)(ℚ)`, and by the Cartan decomposition
// `Spin⁺(3,1) ≅ Spin(3) × exp(𝔭)` with `K ∩ exp(𝔭) = {1}` a transvection
// can never cancel a rotation. A boundary product taken on the carriage
// alone therefore reports a cell as flat while its orientation rotor holds
// a disclination.
//
// `cech::coboundary::solve_gauge` already solves `δγ = ω` on BOTH halves
// together and returns the 1-cocycle the tree cannot place, and
// `Complex::coboundary_on_link` closes what a simplex's faces determine.
// Two implementations of one operation is one too many, and the surviving
// one is the one that is correct about the group.

/// THE CANONICAL SPLIT OF A GENERATOR INTO COMMUTING PARTS (54.7, ruled
/// 2026-09-21).
///
/// ```text
///     B = B_ell + B_hyp,   [B_ell, B_hyp] = 0,
///     B_ell² = −a² < 0  (compact rotation),
///     B_hyp² = +b² > 0  (open transvection)
/// ```
///
/// WHY IT IS NEEDED. A toral period is the order of a ROTATION — a compact
/// invariant. A transvection generates a non-compact one-parameter subgroup
/// whose geodesic flow has no returning orbit, so a screw's acoustic pitch
/// is the frequency of its twist about the axis and NOT of its slip along
/// it. Reading a period off the whole generator asks a compact question of
/// a non-compact object; reading it off `B_ell` asks it of the only part
/// that can answer.
///
/// THE ALGEBRA, and it is exact. Writing `S = B_hyp − B_ell`, the two parts
/// commute and `B_ell B_hyp = ½⟨B²⟩₄`, so
///
/// ```text
///     B²  = ⟨B²⟩₀ + ⟨B²⟩₄        S²  = ⟨B²⟩₀ − ⟨B²⟩₄
/// ```
///
/// and `B_ell = (B − S)/2`, `B_hyp = (B + S)/2`. `S` is sought in the
/// commutative subalgebra `⟨1, Q⟩` with `Q = ⟨B²⟩₄`, `Q² = κ ∈ ℚ`:
/// `S = α + βQ` with `α² + β²κ = ⟨B²⟩₀` and `2αβ = −1`, which reduces to a
/// quadratic in `α²` solved by [`crate::transducer::alphabet::rational_sqrt`].
///
/// EXACT OR REFUSED (rule 4, rule 11). The discriminant need not be a
/// rational square, and where it is not the split does not exist over ℚ and
/// is refused — never rounded onto a nearby one. A SIMPLE generator splits
/// trivially: one part is `B`, the other is zero, and which is which is the
/// sign of `⟨B²⟩₀`.
pub fn bivector_split(m: &Mv) -> Option<(Mv, Mv)> {
    let b = m.grade(2);
    let bb = b.geo(&b);
    let i1 = bb.scalar_part();
    let q = bb.grade(4);
    if q.is_zero() {
        // SIMPLE: one part carries the whole blade and the other is zero.
        // Which is which is the SIGN of `⟨B²⟩₀` — a reading, not a
        // convention — and the PLANE travels with it either way.
        return match i1.is_negative() {
            true => Some((b, Mv::zero())),
            false => Some((Mv::zero(), b)),
        };
    }
    // `Q² = κ` is a scalar in five dimensions; refuse if this algebra says
    // otherwise rather than proceeding on an assumption.
    let qq = q.geo(&q);
    let kappa = qq.scalar_part();
    if !qq.sub(&Mv::scalar(kappa.clone())).is_zero() {
        return None;
    }
    // `S = B_hyp − B_ell` lies in the commutative subalgebra `⟨B, BQ⟩` —
    // NOT in `⟨1, Q⟩`, which is scalar-plus-grade-4 and could never be a
    // bivector. Writing `S = xB + yBQ` and expanding `S² = ⟨B²⟩₀ − ⟨B²⟩₄`:
    //
    //     u = x² + y²κ = (⟨B²⟩₀² + κ)/Δ ,   v = 2xy = −2⟨B²⟩₀/Δ
    //     x² = (u ± √(u² − κv²))/2 ,        y = v/(2x)
    //
    // with `Δ = ⟨B²⟩₀² − κ`. Exact or refused (rule 4): neither root need
    // be rational, and where one is not the split does not exist over ℚ.
    let two = BigRational::from_integer(num_bigint::BigInt::from(2));
    let delta = &i1 * &i1 - &kappa;
    if delta.is_zero() {
        return None;
    }
    let v = -(&two * &i1) / &delta;
    let u = (&i1 * &i1 + &kappa) / &delta;
    let inner = &u * &u - &kappa * &v * &v;
    let r = crate::transducer::alphabet::rational_sqrt(&inner)?;
    // Either root of `x²` gives `S` or `−S`; the one that matters is which
    // half comes out compact, and that is read below rather than chosen.
    let x2 = (&u + &r) / &two;
    let x2 = match x2.is_zero() {
        true => (&u - &r) / &two,
        false => x2,
    };
    let x = crate::transducer::alphabet::rational_sqrt(&x2)?;
    if x.is_zero() {
        return None;
    }
    let y = &v / (&two * &x);
    let s = b.scale(&x).add(&b.geo(&q).scale(&y));
    // VERIFIED AGAINST THE ALGEBRA, never assumed.
    if !s.geo(&s).sub(&Mv::scalar(i1).sub(&q)).is_zero() {
        return None;
    }
    let half = one() / &two;
    let (p, n) = (b.sub(&s).scale(&half), b.add(&s).scale(&half));
    // WHICH HALF IS COMPACT IS MEASURED. `B_ell² < 0` and `B_hyp² > 0`;
    // the sign of the root decided the labelling, so the labelling is read
    // back off the algebra instead of trusted.
    let (ell, hyp) = match p.geo(&p).scalar_part().is_negative() {
        true => (p, n),
        false => (n, p),
    };
    if !ell.geo(&ell).scalar_part().is_negative() && !ell.is_zero() {
        return None;
    }
    if !hyp.geo(&hyp).scalar_part().is_positive() && !hyp.is_zero() {
        return None;
    }
    // The parts must COMMUTE and re-sum, or this is not the split.
    if !ell.geo(&hyp).sub(&hyp.geo(&ell)).is_zero() {
        return None;
    }
    if !ell.add(&hyp).sub(&b).is_zero() {
        return None;
    }
    Some((ell, hyp))
}

/// IS THIS GENERATOR SIMPLE? `B ∧ B = 0` means `B` is a single blade — one
/// plane, one motion. `B ∧ B ≠ 0` means it is a SCREW: a rotation and a
/// transvection on orthogonal planes at once, which cannot be split into
/// independent planar projections without losing the coupling (51.2,
/// Anti-Flattening clause 5).
pub fn bivector_is_simple(m: &Mv) -> bool {
    let b = m.grade(2);
    b.geo(&b).grade(4).is_zero()
}

/// A GEODESIC TRANSVECTION OF ℍ⁴, EXACTLY OVER ℚ (54.7 R3).
///
/// SIMPLE BY CONSTRUCTION, and that is a statement about this particular
/// motion rather than a shape imposed on all of them: a boost along ONE
/// geodesic has a rank-one generator and `I₂ = 0` honestly. A motion that
/// couples planes is built with [`versor_of_reflections`], which places no
/// such restriction on `⟨Ω⟩₂`.
///
/// The motion generated by a hyperbolic bivector `B = eᵢ ∧ e₋`, `B² = +1`.
/// Its one-parameter form is `Ω = exp(½ τ B) = cosh(τ/2) + sinh(τ/2)·B`,
/// and `cosh` and `sinh` of a rational are NOT rational — so a transvection
/// parameterised by hyperbolic distance cannot be held exactly (rule 4,
/// rule 11), and this does not pretend to hold one.
///
/// It is written the way [`rotator`] already writes a rotation: the
/// UNNORMALISED even element `Ω = a + b·(eₐ ∧ e₋)` with `a, b ∈ ℚ`. The
/// sandwich carries the weight and homogeneity on the null cone absorbs
/// it, exactly as a rotation's is absorbed.
///
/// THE WEIGHT IS `Ω Ω̃`, TAKEN BY THE GEOMETRIC PRODUCT (rule 11). A first
/// cut wrote it as `a² − b²` — the components squared and subtracted, a
/// Pythagorean form on a pair of numbers. That is the flat two-dimensional
/// square rule 11 forbids, written inside the one function whose purpose is
/// to leave flatness behind: "fluid physics carried in squares is squares
/// of fluid". `is_versor` already computes `Ω Ω̃` through `geo` and
/// `reverse`, so the weight is a reading of the algebra and never an
/// arithmetic identity someone remembered for one blade.
///
/// The refusal it produces is the same one and means more: `Ω Ω̃ = 0` is the
/// light cone, where the element is a PARABOLIC limit rotation rather than
/// a transvection and has no inverse. `is_versor` also refuses an `Ω Ω̃`
/// that is not a pure scalar, which no hand-written norm can see at all.
///
/// `plane` is one of [`TRANSVECTION_PLANES`]; any other blade is not a
/// hyperbolic generator and is refused.
pub fn transvection(a: &BigRational, b: &BigRational, plane: usize) -> Option<Mv> {
    if !TRANSVECTION_PLANES.contains(&plane) {
        return None;
    }
    let omega = Mv::scalar(a.clone()).add(&Mv::blade(plane, b.clone()));
    is_versor(&omega)?;
    Some(omega)
}

/// THE COMPOSITE GEODESIC MOTION ALONG EVERY AXIS OF ℍ⁴ — one transvection
/// per generator of [`TRANSVECTION_PLANES`], composed by the geometric
/// product in the generators' own order.
///
/// Unlike the conformal translators `eᵢ ∧ e_∞`, which commute because that
/// reading is of a flat lattice base (51.2), these do NOT commute: their
/// commutator is a rotation scaled by the sectional curvature. The failure
/// to commute IS the curvature, which is why the order is the algebra's and
/// not a caller's, and why `K` is read off a remainder rather than carried
/// as a constant (51.2, Anti-Flattening clause 7).
pub fn transvection_along(axes: [(&BigRational, &BigRational); 4]) -> Option<Mv> {
    let mut out = Mv::scalar(one());
    // index: this algebra's own hyperbolic generators, in the order the
    // blades are numbered — the carrier's declaration, never a search.
    for (i, plane) in TRANSVECTION_PLANES.iter().enumerate() {
        let (a, b) = axes.get(i)?;
        out = out.geo(&transvection(a, b, *plane)?);
    }
    is_versor(&out)?;
    Some(out)
}

/// THE HOLONOMY OF A GEODESIC BIGON: `Ω₁ Ω₂ Ω₁⁻¹ Ω₂⁻¹`, the group
/// commutator of two transvections.
///
/// On a flat base this is the identity — that is what "translators commute"
/// means. Here it is a ROTATION whose magnitude is the sectional curvature
/// of the plane the two boosts span (51.2). Returning it is how the
/// curvature is READ off the geometry rather than asserted as a value:
/// [`Mv::class_invariants`] on the result says which bivector type the
/// commutator is, and an elliptic answer on two hyperbolic generators is
/// the curvature being non-zero, measured.
pub fn transvection_commutator(a: &Mv, b: &Mv) -> Option<Mv> {
    let ai = a.versor_inverse()?;
    let bi = b.versor_inverse()?;
    let c = a.geo(b).geo(&ai).geo(&bi);
    is_versor(&c)?;
    Some(c)
}
