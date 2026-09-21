//! A4 — CONFORMAL GEOMETRIC ALGEBRA: the exact multivector algebra `Cl(4,1)`
//! over ℚ, the conformal model of the three-coordinate base `(r, c, k)`
//! (51.2), where a body is an invertible dual sphere and a rigid motion is one
//! versor sandwich, so the quotient between two scene states exists.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

/// Basis-vector masks over `{e1, e2, e3, e+, e-}`: the three base axes —
/// row, column, frame — and the two null-cone generators.
pub const E1: usize = 0b00001;
pub const E2: usize = 0b00010;
pub const E3: usize = 0b00100;
pub const EP: usize = 0b01000; // e+  (square +1)
pub const EM: usize = 0b10000; // e-  (square −1)
/// The unit pseudoscalar `e1 e2 e3 e+ e-`.
pub const PSEUDO: usize = 0b11111;
/// The blades of the algebra: `2⁵`.
pub const BLADES: usize = 32;
const MASK: usize = BLADES - 1;

/// An exact multivector of `Cl(4,1)`: 32 blade coefficients over ℚ.
/// Equality is projective (`λ ∈ ℚ×`); coefficient identity is [`Mv::canonical`].
#[derive(Debug, Clone)]
pub struct Mv {
    c: [BigRational; BLADES],
}

impl PartialEq for Mv {
    fn eq(&self, other: &Self) -> bool {
        self.projective_eq(other)
    }
}
impl Eq for Mv {}

fn zero() -> BigRational {
    BigRational::zero()
}
fn one() -> BigRational {
    BigRational::one()
}
fn half() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(2))
}

/// The parity of the number of transpositions needed to sort the concatenation
/// of blade `a` before blade `b` into canonical ascending order: +1 even, −1 odd.
fn reorder_sign(a: usize, b: usize) -> i64 {
    let mut hi = a >> 1;
    let mut swaps = 0u32;
    while hi != 0 {
        swaps += (hi & b).count_ones();
        hi >>= 1;
    }
    if swaps & 1 == 0 {
        1
    } else {
        -1
    }
}

/// The metric sign of squaring the shared generators of two blades: only `e-`
/// squares to −1; `e1, e2, e3, e+` square to +1. (No null generators in the
/// orthonormal basis — the null cone is DERIVED — so a blade product never
/// vanishes by metric, only by sign.)
fn metric_sign(shared: usize) -> i64 {
    if (shared & EM) != 0 {
        -1
    } else {
        1
    }
}

impl Mv {
    pub fn zero() -> Self {
        Self { c: std::array::from_fn(|_| zero()) }
    }

    pub fn scalar(v: BigRational) -> Self {
        let mut m = Self::zero();
        m.c[0] = v;
        m
    }

    /// A single basis blade with coefficient `v`.
    pub fn blade(mask: usize, v: BigRational) -> Self {
        let mut m = Self::zero();
        m.c[mask & MASK] = v;
        m
    }

    pub fn coeff(&self, mask: usize) -> &BigRational {
        &self.c[mask & MASK]
    }

    pub fn scalar_part(&self) -> BigRational {
        self.c[0].clone()
    }

    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|x| x.is_zero())
    }

    /// A4: divide by |first nonzero|, negate so that coefficient is +1.
    pub fn canonical(&self) -> Option<Mv> {
        let i = (0..BLADES).find(|&i| !self.c[i].is_zero())?;
        let mut r = self.scale(&(one() / self.c[i].abs()));
        if r.c[i].is_negative() {
            r = r.scale(&-one());
        }
        Some(r)
    }

    /// THE CANONICAL WRITING (A4, 17.2): the coefficients after `canonical`,
    /// so a versor and every `λ·versor` write the same bytes and a key built
    /// from them is content rather than one chosen spelling of it. The zero
    /// element has no `canonical` and is not a versor; its zeros are already
    /// its only writing, so it is written as it is.
    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        match self.canonical() {
            Some(c) => c.to_bytes(out),
            None => self.to_bytes(out),
        }
    }

    /// `a = λ b` for `λ ∈ ℚ×` of either sign.
    pub fn projective_eq(&self, other: &Mv) -> bool {
        projective_ratio(self, other).is_some_and(|l| !l.is_zero())
    }

    /// Canonical coefficients written out (A21). None when the element is
    /// zero (no canonical form).
    pub fn key(&self) -> Option<crate::transducer::address::Address> {
        let n = self.canonical()?;
        let mut buf = vec![crate::transducer::address::TAG_VERSOR];
        n.to_bytes(&mut buf);
        Some(crate::transducer::address::Address::new(buf))
    }

    pub fn from_key(k: &[u8]) -> Option<Mv> {
        if crate::transducer::address::tag_of(k) != Some(crate::transducer::address::TAG_VERSOR) {
            return None;
        }
        let mut p = 1usize;
        Mv::from_bytes(k, &mut p)
    }

    pub fn add(&self, o: &Self) -> Self {
        Self { c: std::array::from_fn(|i| &self.c[i] + &o.c[i]) }
    }

    pub fn sub(&self, o: &Self) -> Self {
        Self { c: std::array::from_fn(|i| &self.c[i] - &o.c[i]) }
    }

    pub fn scale(&self, s: &BigRational) -> Self {
        Self { c: std::array::from_fn(|i| &self.c[i] * s) }
    }

    /// THE GEOMETRIC PRODUCT — the whole algebra. `result[i⊕j] += sign · metric ·
    /// a_i · b_j` over every pair of populated blades.
    pub fn geo(&self, o: &Self) -> Self {
        let mut r = Self::zero();
        for i in 0..BLADES {
            if self.c[i].is_zero() {
                continue;
            }
            for j in 0..BLADES {
                if o.c[j].is_zero() {
                    continue;
                }
                let sign = reorder_sign(i, j) * metric_sign(i & j);
                let term = &self.c[i] * &o.c[j];
                let out = i ^ j;
                if sign >= 0 {
                    r.c[out] = &r.c[out] + &term;
                } else {
                    r.c[out] = &r.c[out] - &term;
                }
            }
        }
        r
    }

    /// The reverse `~M`: blade of grade g gets sign `(−1)^{g(g−1)/2}`.
    pub fn reverse(&self) -> Self {
        Self {
            c: std::array::from_fn(|i| {
                let g = (i as u32).count_ones();
                if (g * (g.wrapping_sub(1)) / 2) & 1 == 0 {
                    self.c[i].clone()
                } else {
                    -self.c[i].clone()
                }
            }),
        }
    }

    /// THE OUTER PRODUCT `A ∧ v` FOR A BLADE `A` AND A VECTOR `v` (54.7 B8).
    ///
    /// `⟨A v⟩_{g+1}` — the geometric product's top grade. It is the whole of
    /// the wedge when `A` is a homogeneous blade of grade `g`, which is the
    /// only case a round is built from: a round is `p₀ ∧ p₁ ∧ … ∧ pₘ` and
    /// each step wedges one more null point onto a blade.
    ///
    /// GRASSMANN TERMINATES IT AND NO GUARD DOES. `v ∧ v = 0`, so a point
    /// already on the round contributes nothing; a point conformally
    /// dependent on the round gives zero the same way; and `⋀⁶(ℚ^{4,1}) ≡
    /// 0`, so a sixth point vanishes identically. Dimensional exhaustion is
    /// exterior-algebra arithmetic, never a `grade > MAX` branch.
    ///
    /// `Cl(4,1)` is this tree's conformal algebra — five generators
    /// `{e1, e2, e3, e+, e-}`, 32 blades, the conformal model of 3-space —
    /// so the ladder of rounds is: grade 2 a point pair (0-sphere), grade 3
    /// a circle (1-sphere), grade 4 a sphere (2-sphere), grade 5 the
    /// pseudoscalar, grade 6 identically zero.
    pub fn wedge_vector(&self, g: u32, v: &Self) -> Self {
        self.geo(v).grade(g + 1)
    }

    /// THE CONJUGACY INVARIANTS OF A TRANSITION, READ OFF ITS GRADE-2
    /// GENERATOR (54.7, as ruled 2026-09-21).
    ///
    /// A transition is `Ω = R · T(τ)` and its generator is the bivector
    /// `B = ⟨Ω⟩₂`. In `Cl(4,1)` that bivector is NOT one kind of thing:
    ///
    /// ```text
    ///     B = B_ell + B_hyp ,   B_ell = Σ b_ij (eᵢ∧eⱼ)   1 ≤ i<j ≤ 3
    ///                           B_hyp = Σ b_i₋ (eᵢ∧e₋)   1 ≤ i ≤ 3
    /// ```
    ///
    /// `B_ell` generates an ELLIPTIC rotation stabilising a timelike
    /// geodesic; `B_hyp` generates a HYPERBOLIC transvection, a pure
    /// geodesic translation; a null `B` generates a PARABOLIC limit
    /// rotation stabilising a point on the asymptotic sphere. The three are
    /// not conjugate to one another, and the sign of
    ///
    /// ```text
    ///     I₁ = ⟨B²⟩₀ = ‖B_ell‖² − ‖B_hyp‖²
    /// ```
    ///
    /// is what tells them apart.
    ///
    /// WHY THIS CANNOT BE DONE IN `Cl(4,0)`. `Rotor::class_invariants` reads
    /// `c[0]` and `c[15]` — grade 0 and grade 4 — and discards all six
    /// bivector components. That is not an oversight there: `Cl(4,0)` is
    /// POSITIVE DEFINITE, every bivector squares negative, every plane is
    /// carried to every other by a conjugation, and the conjugacy classes of
    /// `Spin(4)` genuinely are the two angles alone. The planes are the
    /// rotation's position, not its class, so reading grade 2 in `Cl(4,0)`
    /// would add nothing.
    ///
    /// It adds everything here. Under the signature `(+,+,+,+,−)` the sign
    /// of `I₁` is a real coordinate, and a rotation, a boost and a limit
    /// rotation are three classes rather than one. `Cl(4,0)` could not hold
    /// that distinction because it has no absolute to be hyperbolic about —
    /// which is why the transition's two halves must be composed into ONE
    /// element of `Spin(4,1)` before the class is taken, rather than a rotor
    /// classified in a flat algebra beside a carriage classified in a curved
    /// one.
    ///
    /// ```text
    ///     I₂ = ‖⟨B²⟩₄‖²      the screw coupling, 2(B_ell · B_hyp) squared
    /// ```
    ///
    /// `I₂ = 0` is a planar rotation or a pure transvection; `I₂ ≠ 0` is a
    /// helicoidal screw, the two halves coupled.
    ///
    /// EXACT AND SCALE-FREE. `B ↦ λB` sends `⟨B²⟩₀ ↦ λ²⟨B²⟩₀` and
    /// `w = ΩΩ̃ ↦ λ²w`, so `I₁/w` is invariant; `‖⟨B²⟩₄‖² ↦ λ⁴‖·‖²`, so
    /// `I₂/w²` is. Both are quadratic in `B`, so neither depends on the
    /// ray's sign — `−Ω` and `Ω` are one transition and read alike. `None`
    /// when `Ω` is not a versor, which is a refusal and never a zero class
    /// (rule 11).
    pub fn class_invariants(
        &self,
    ) -> Option<(BigRational, BigRational, crate::network::nerve::coord::BivectorKind)> {
        let w = is_versor(self)?;
        if w.is_zero() {
            return None;
        }
        let b = self.grade(2);
        let bb = b.geo(&b);
        let i1 = bb.scalar_part() / &w;
        let quad = bb.grade(4);
        let i2 = quad.geo(&quad.reverse()).scalar_part() / (&w * &w);
        // THE SIGN IS THE TYPE, and it is the reading this whole function
        // exists for. Computed, not assumed — with `eᵢ² = +1` for
        // `i ∈ {1,2,3,+}` and `e₋² = −1`, all anticommuting:
        //
        //   ELLIPTIC   B = e₁∧e₂ :  B² = e₁e₂e₁e₂ = −e₁e₁e₂e₂ = −1  < 0
        //   HYPERBOLIC B = eᵢ∧e₋ :  B² = eᵢe₋eᵢe₋ = −eᵢeᵢe₋e₋ = +1  > 0
        //   PARABOLIC  B = −½ d e_∞ : e_∞² = e₋² + e₊² = 0 and d ⊥ e_∞,
        //              so B² = −¼ d² e_∞² = 0
        //
        // A first cut had elliptic and hyperbolic the wrong way round. The
        // parabolic case is the one that names the absolute: the CONFORMAL
        // translator `T(d) = 1 − ½ d e_∞` has a NULL bivector and fixes the
        // point at infinity, which is what makes the flat embedding the
        // wrong geometry for a manifold with curvature. On the hyperboloid
        // there is no `e_∞` to fix, and a geodesic translation is a true
        // transvection `eᵢ∧e₋` with `B² = +1`.
        // 54.7 — TRIVIAL IS NOT PARABOLIC, and collapsing them put every
        // identity loop in the same class as every null shear.
        //
        //   TRIVIAL     B = 0            — the identity, Ω = 1, turns nothing
        //   ELLIPTIC    ⟨B²⟩₀ < 0        — a rotation, eᵢ∧eⱼ or eᵢ∧e₊
        //   HYPERBOLIC  ⟨B²⟩₀ > 0        — a transvection, eᵢ∧e₋
        //   PARABOLIC   B ≠ 0, ⟨B²⟩₀ = 0 — a genuine null shear on e₊∧e₋
        //
        // The first three are readings of a motion; the fourth is too, and
        // the identity is the absence of one. `I₁ = 0` alone cannot tell
        // the last two apart, so the bivector itself is asked.
        use crate::network::nerve::coord::BivectorKind;
        let kind = if b.is_zero() {
            BivectorKind::Trivial
        } else if i1.is_negative() {
            BivectorKind::Elliptic
        } else if i1.is_positive() {
            BivectorKind::Hyperbolic
        } else {
            BivectorKind::Parabolic
        };
        Some((i1, i2, kind))
    }

    /// 54.7 R4 — THE HOLONOMY CLASS OF A MOTION, AS THE COORDINATE THE
    /// TORSION AND AXIS CARRIERS ANSWER.
    ///
    /// The same bytes as [`Mv::class_key`], declared as `coord::Holonomy`.
    /// This is the producer that replaces `Rotor::class_key` beneath
    /// `Rotor::holonomy_key`: the outer type is unchanged, so every carrier
    /// typed in R5 — `Form.torsion`, `Form.axes`, `Element.filing`,
    /// `Idea.torsion` — keeps its signature and its call sites.
    ///
    /// WHAT CHANGES IS WHAT THE CLASS CAN SAY. In `Cl(4,0)` the metric is
    /// positive definite, so for every bivector `B = B⁺ + B⁻`,
    ///
    /// ```text
    ///     ⟨B²⟩₀ = −(‖B⁺‖² + ‖B⁻‖²) ≤ 0
    /// ```
    ///
    /// with equality only at `B = 0`. A `Cl(4,0)` rotor therefore has
    /// exactly one bivector type — elliptic — and a holonomy class read
    /// from it cannot distinguish a rotation from a geodesic translation,
    /// because it cannot represent the translation at all. Here `⋀²ℚ^{4,1}`
    /// is ten-dimensional and the trichotomy is a reading: `eᵢ∧eⱼ` and
    /// `eᵢ∧e₊` elliptic (`I₁ < 0`), `eᵢ∧e₋` hyperbolic transvections
    /// (`I₁ > 0`), `e₊∧e₋` parabolic (`I₁ = 0`).
    pub fn holonomy_key(&self) -> Option<crate::network::nerve::coord::Holonomy> {
        let (i1, i2, kind) = self.class_invariants()?;
        Some(crate::network::nerve::coord::Holonomy { i1, i2, kind })
    }

    /// The class as canonical coordinates (A21, law 2): the two invariants
    /// written out and the bivector's type, never a digest.
    /// The class as canonical coordinates — written by the coordinate
    /// itself, so the bytes here and the bytes a carrier is keyed by are
    /// one construction and not two agreeing (rule 12).
    pub fn class_key(&self) -> Option<crate::transducer::address::Address> {
        use crate::network::nerve::Coordinate;
        let h = self.holonomy_key()?;
        Some(crate::transducer::address::Address::from_bytes(
            h.encode().as_bytes(),
        ))
    }

    /// The grade-`g` part.
    pub fn grade(&self, g: u32) -> Self {
        Self {
            c: std::array::from_fn(|i| {
                if (i as u32).count_ones() == g {
                    self.c[i].clone()
                } else {
                    zero()
                }
            }),
        }
    }

    /// The scalar `⟨A B̃⟩₀`-style inner-square. For a grade-1 vector this is the
    /// metric norm `v·v`.
    pub fn norm2(&self) -> BigRational {
        self.geo(self).scalar_part()
    }

    /// The inverse `M⁻¹ = M̃ / (M M̃)` when `M M̃` is a nonzero SCALAR (true for
    /// versors and for non-null vectors — spheres). `None` when it is not a
    /// scalar or is zero: a NULL point (`X·X = 0`) has no inverse — the demand
    /// `B·A⁻¹` cannot form until the object has volume. Honest, never rounded.
    pub fn versor_inverse(&self) -> Option<Self> {
        let rev = self.reverse();
        let prod = self.geo(&rev);
        let s = prod.scalar_part();
        if s.is_zero() {
            return None; // null / volumeless: no inverse — the demand cannot form
        }
        if !prod.sub(&Mv::scalar(s.clone())).is_zero() {
            return None; // M M̃ is not a pure scalar: not a simple versor/vector
        }
        Some(rev.scale(&(one() / s)))
    }

    /// The geometric quotient `Q = B · A⁻¹` — the demand between two states.
    /// `None` when A is non-invertible (a null, volumeless body).
    pub fn quotient(b: &Self, a: &Self) -> Option<Self> {
        Some(b.geo(&a.versor_inverse()?))
    }

    /// The versor SANDWICH `V x Ṽ` — the exact action of a rigid motion on a
    /// whole volume in one step.
    pub fn sandwich(&self, x: &Self) -> Self {
        self.geo(x).geo(&self.reverse())
    }
}

// ---- the conformal null basis, embeddings, and versors ----

/// `e_o = ½(e- − e+)` — the origin (null).
pub fn e_o() -> Mv {
    Mv::blade(EM, half()).sub(&Mv::blade(EP, half()))
}

/// `e_∞ = e- + e+` — the point at infinity (null).
pub fn e_inf() -> Mv {
    Mv::blade(EM, one()).add(&Mv::blade(EP, one()))
}

/// Embed a base point on the null cone: `X(x) = x + ½‖x‖² e_∞ + e_o`, with
/// `x = x1 e1 + x2 e2 + x3 e3` — row, column, frame.
pub fn point(x1: &BigRational, x2: &BigRational, x3: &BigRational) -> Mv {
    let x2n = x1 * x1 + x2 * x2 + x3 * x3;
    let mut m = Mv::blade(E1, x1.clone()).add(&Mv::blade(E2, x2.clone())).add(&Mv::blade(E3, x3.clone()));
    m = m.add(&e_inf().scale(&(half() * x2n)));
    m.add(&e_o())
}

/// A CHART BASE POINT ON THE NULL CONE OF A NON-EUCLIDEAN SPACE (54.7 B8,
/// as ruled 2026-09-21).
///
/// ```text
///     p(x) = x₁e₁ + x₂e₂ + x₃e₃ + x₀e₋ + R e₊
/// ```
///
/// with `x` on the hyperboloid `⟨x,x⟩_{3,1} = x₁² + x₂² + x₃² − x₀² = −R²`,
/// `x₀ > 0`. Then, since `e₊² = +1` and `e₋² = −1`,
///
/// ```text
///     p² = (x₁² + x₂² + x₃²) − x₀² + R² = ⟨x,x⟩_{3,1} + R² = 0
/// ```
///
/// so every chart base point is strictly null, and the inner product of two
/// of them is the NON-EUCLIDEAN invariant:
///
/// ```text
///     pᵢ · pⱼ = ⟨xᵢ,xⱼ⟩_{3,1} + R² = R²(1 − cosh(d_ℍ(xᵢ,xⱼ)/R))
/// ```
///
/// WHY NOT [`point`]. That is the flat embedding `x + ½‖x‖²e_∞ + e_o`: it
/// introduces a degenerate point at infinity and a Cartesian norm, which
/// puts a privileged planar boundary into a manifold whose sectional
/// curvature is not zero. Its translations are the parabolic `T(a) = 1 +
/// ½a e_∞`; here there is no `e_∞` generator to build one from, and a
/// geodesic motion is a hyperbolic transvection — a mixed bivector `eᵢ ∧
/// e₋` — with elliptic rotations `eᵢ ∧ eⱼ` stabilising a timelike axis and
/// parabolic limit rotations on null bivectors. `Isom(ℍ³) ≅ Spin⁺(3,1) ⊂
/// Spin(4,1)`.
///
/// EXACT OR REFUSED (law 4, rule 11). `x₀` is a COORDINATE of the point and
/// is never solved for: `x₀² = ‖x‖² + R²` has no rational root in general,
/// and a chart whose four coordinates do not satisfy the hyperboloid
/// condition exactly is refused here rather than rounded onto it.
pub fn hyperbolic_point(
    x1: &BigRational,
    x2: &BigRational,
    x3: &BigRational,
    x0: &BigRational,
    r: &BigRational,
) -> Option<Mv> {
    if x1 * x1 + x2 * x2 + x3 * x3 - x0 * x0 + r * r != zero() {
        return None;
    }
    if *x0 <= zero() {
        return None;
    }
    Some(
        Mv::blade(E1, x1.clone())
            .add(&Mv::blade(E2, x2.clone()))
            .add(&Mv::blade(E3, x3.clone()))
            .add(&Mv::blade(EM, x0.clone()))
            .add(&Mv::blade(EP, r.clone())),
    )
}

/// The dual sphere `S = X(c) − ½ r² e_∞`: a point at squared-distance `r2` from
/// `c`. `r2 = 0` recovers the point (a volumeless sphere). Off-cone iff r²≠0.
pub fn sphere(c1: &BigRational, c2: &BigRational, c3: &BigRational, r2: &BigRational) -> Mv {
    point(c1, c2, c3).sub(&e_inf().scale(&(half() * r2)))
}

/// The translation versor `T(d) = 1 − ½ d e_∞`, `d = d1 e1 + d2 e2 + d3 e3`.
/// Acts by sandwich: `T X(x) T̃ = X(x + d)`, up to the homogeneous scale.
/// Translators commute (51.2): the base is a lattice.
///
/// 54.7 R3 — THIS IS THE PARABOLIC PINNING, and it is what the hyperbolic
/// unification replaces. Its generator is `B = −½ d e_∞` with `e_∞² = 0`,
/// so `B² = 0`: a null bivector fixing the point at infinity, which is a
/// privileged planar boundary in a manifold whose sectional curvature is
/// not zero. [`transvection`] is the geodesic motion that has no such
/// fixed point.
pub fn translator(d1: &BigRational, d2: &BigRational, d3: &BigRational) -> Mv {
    let d = Mv::blade(E1, d1.clone()).add(&Mv::blade(E2, d2.clone())).add(&Mv::blade(E3, d3.clone()));
    let d_einf = d.geo(&e_inf());
    Mv::scalar(one()).sub(&d_einf.scale(&half()))
}

/// THE FOUR HYPERBOLIC GENERATORS `eₐ ∧ e₋`, the mixed bivectors `Cl(4,0)`
/// has none of. Each squares to `+1`:
///
/// ```text
///     (eₐe₋)² = eₐe₋eₐe₋ = −eₐeₐe₋e₋ = −(+1)(−1) = +1
/// ```
///
/// FOUR, NOT THREE. A first cut declared `e₁∧e₋, e₂∧e₋, e₃∧e₋` and dropped
/// `e₊∧e₋`. `Spin(4,1)` is the isometry group of HYPERBOLIC 4-SPACE whose
/// boundary at infinity is the three-coordinate base (51.2), and ℍ⁴ is four
/// dimensional: its boosts are `eₐ ∧ e₋` for every spacelike `a`, and `e₊`
/// is spacelike (`e₊² = +1`). Declaring three of four restricts dimensional
/// capacity, which the Anti-Flattening Law clause 5 refuses by name — the
/// same defect as collapsing `Λ²(ℝ⁴)` to one blade, one dimension up.
pub const TRANSVECTION_PLANES: [usize; 4] = [E1 | EM, E2 | EM, E3 | EM, EP | EM];

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

/// THE PATH-ORDERED TRANSPORT AROUND A CLOSED CHAIN OF EDGES.
///
/// ```text
///     Ω_∂e² = 𝒫 ∏ Ω_e
/// ```
///
/// An edge of the complex is not an annotation carrying a bivector label;
/// it is a FINITE transport operator, and the curvature a 2-cell traps is
/// the ordered product of the transitions around its attaching map. The
/// order is the path's — the product does not commute, and that is the
/// content.
///
/// `Ω_∂e² = 1` is a flat cell. Anything else is an obstruction the cell
/// holds: a Burgers transvection where the composite reads hyperbolic, a
/// curvature rotation where it reads elliptic — which
/// [`Mv::class_invariants`] answers and nothing here decides.
pub fn transport_around(edges: &[Mv]) -> Option<Mv> {
    let mut out = Mv::scalar(one());
    // index: the edges of one attaching map, in the order the boundary
    // walks them — the cell's own declaration, never a search.
    for e in edges {
        is_versor(e)?;
        out = out.geo(e);
    }
    is_versor(&out)?;
    Some(out)
}

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

/// AXIOM B — A LAMINA LANDS ON THE HYPERBOLOID SHEET, EXACTLY OVER ℚ
/// (54.7, ruled 2026-09-21).
///
/// ```text
///     v(u) = (2u₁, 2u₂, 2u₃, 1+|u|²) / (1−|u|²),     u = (r,c,k)/M
/// ```
///
/// Rational stereographic projection from the pole `(0,0,0,1)`. Every
/// component is a rational polynomial in `u`, so there is no square root
/// and no Diophantine refusal:
///
/// ```text
///     v² = [4|u|² − (1+|u|²)²] / (1−|u|²)² = −(1−|u|²)²/(1−|u|²)² = −1
/// ```
///
/// THE SHEET, NOT THE CONE. `v² = −1` is the INTERIOR `ℍ³`, where a lamina
/// and a member frame actually sit; the null cone `p² = 0` is the ideal
/// boundary `∂_∞ℍ³`, the asymptotic horizon of infinite geodesic rays. A
/// first cut put laminae on the cone, and there
/// [`geodesic_transvection`] degenerates: `p_a² = 0` collapses the
/// transport step and the operator carries nothing.
///
/// `M` is the LATTICE QUANTUM: an integer with `M² > r²+c²+k²`, which keeps
/// `|u| < 1` so the denominator never vanishes and the point stays on the
/// upper sheet. It sets the unit of one lattice step and nothing else — the
/// curvature is still read off a remainder, never carried as a constant
/// (51.2, 51.11). `None` where `M` does not bound the point: that lamina is
/// outside this chart and says so rather than being rounded into it.
pub fn hyperbolic_lamina(r: &BigRational, c: &BigRational, k: &BigRational, m: &BigRational) -> Option<Mv> {
    if m.is_zero() {
        return None;
    }
    let (u1, u2, u3) = (r / m, c / m, k / m);
    let n2 = &u1 * &u1 + &u2 * &u2 + &u3 * &u3;
    let denom = one() - &n2;
    if denom.is_zero() || denom.is_negative() {
        // `|u| ≥ 1` is off this chart — outside the sheet's stereographic
        // image, refused rather than projected onto its edge (rule 11).
        return None;
    }
    let two = BigRational::from_integer(num_bigint::BigInt::from(2));
    let s = one() / &denom;
    Some(
        Mv::blade(E1, &two * &u1 * &s)
            .add(&Mv::blade(E2, &two * &u2 * &s))
            .add(&Mv::blade(E3, &two * &u3 * &s))
            .add(&Mv::blade(EM, (one() + &n2) * &s)),
    )
}

/// THE POLE OF THE SHEET — `v₀ = e₋`, the point `u = 0` maps to, and the
/// reference a frame's own base point is obtained from by its sandwich.
pub fn hyperboloid_pole() -> Mv {
    Mv::blade(EM, one())
}

/// AXIOM B's TRANSITION — the directed geodesic transvection carrying
/// `v_a` to `v_b` on the sheet, exactly over ℚ (54.7).
///
/// ```text
///     R = 1 − v_b v_a  ∈  Cl⁰(3,1)(ℚ)
/// ```
///
/// VERIFIED BY THE PRODUCT, not asserted. With `v_a² = v_b² = −1`:
///
/// ```text
///     R R̃ = 1 − 2(v_a·v_b) + v_b v_a² v_b = 2(1 − v_a·v_b) ≥ 4 > 0
///     R v_a R̃ = 2(1 − v_a·v_b) v_b        ⟹  R v_a R̃ / R R̃ = v_b
/// ```
///
/// so the norm is a strictly positive rational — invertible with no square
/// root — and the transport is exact, forward-directed, and needs no
/// projective sign fix.
///
/// WHY NOT `Ω = m n` WITH `m = v_b+v_a`, `n = v_b−v_a`. On null rays those
/// are orthogonal (`m·n = p_b² − p_a² = 0`), so they anticommute and
/// `Ω² = −m²n² = 4s² ∈ ℚ*`: an INVOLUTION of order two, a half-turn that
/// swaps its endpoints. Edges built from it would oscillate `v_a ↔ v_b`
/// rather than propagate along the geodesic, and every scalar check would
/// still pass.
///
/// THE PLANE IS CARRIED, not a magnitude: `⟨R⟩₂ = v_a ∧ v_b` is the
/// geodesic 2-blade, with
/// `(v_a∧v_b)² = (v_a·v_b)² − v_a²v_b² = cosh²d − 1 > 0` — hyperbolic, and
/// `d` is never formed (rule 1).
pub fn geodesic_transvection(va: &Mv, vb: &Mv) -> Option<Mv> {
    let on_sheet = |v: &Mv| {
        let s = v.geo(v);
        s.scalar_part() == -one() && s.sub(&Mv::scalar(s.scalar_part())).is_zero()
    };
    if !on_sheet(va) || !on_sheet(vb) {
        return None;
    }
    let r = Mv::scalar(one()).sub(&vb.geo(va));
    // `R R̃ = 2(1 − v_a·v_b)` must be a non-zero pure scalar, which is what
    // `is_versor` reads — and it is ≥ 4 on the sheet, so the only way this
    // refuses is a vector that is not where it claimed to be.
    is_versor(&r)?;
    Some(r)
}

/// THE BASE POINT A FRAME STANDS AT: its own versor applied to the pole,
/// renormalised onto the sheet.
///
/// `v = V v₀ Ṽ` has `v² = −(V Ṽ)²`, so dividing by the weight puts it back
/// on `v² = −1` — one exact rational division, no root.
pub fn frame_base_point(v: &Mv) -> Option<Mv> {
    let w = is_versor(v)?;
    if w.is_zero() {
        return None;
    }
    let p = v.sandwich(&hyperboloid_pole());
    let out = p.scale(&(one() / w));
    let s = out.geo(&out);
    (s.scalar_part() == -one() && s.sub(&Mv::scalar(s.scalar_part())).is_zero()).then_some(out)
}

/// THE TORAL PERIOD OF A MOTION OF `Cl(4,1)` — the acoustic reading (54.7).
///
/// A toral period is the order of a ROTATION: a compact invariant. So it is
/// asked of the COMPACT PART of the generator and of nothing else.
///
/// WHY NOT OF THE WHOLE MOTION. `Rotor::toral_period` reads the `Cl(4,0)`
/// alphabet chart, where every letter is a Euclidean rotation and therefore
/// every word is compact and periodic — 545 of 545 corpus cycles read as
/// CLOSING while 541 of them classified Hyperbolic. That reader interrogates
/// the ALPHABET; this one interrogates the GEOMETRY.
///
/// AN OPEN ORBIT HAS TWO CAUSES AND BOTH ARE READ HERE (the Niven trap):
///
///   1. `B_ell = 0` — no rotation at all. A pure Burgers dislocation slips
///      along a geodesic and never returns.
///   2. `B_ell ≠ 0` and the winding is IRRATIONAL. `cos θ` is rational by
///      construction for a rational rotor, and Niven's theorem says a
///      rational cosine with `θ/π ∈ ℚ` forces `cos θ ∈ {0, ±½, ±1}`. The
///      `(3,4,5)` rotor has `cos θ = −7/25`, which is rational and not one
///      of those — so its orbit is dense on the circle and closes never.
///      `half_angle` is exactly that test and answers `None` there.
///
/// So `B_ell ≠ 0` does NOT imply a finite period, and a finite period is
/// always one of `{1, 2, 3, 4, 6}` — the crystallographic restriction,
/// arriving from ℚ rather than from a convention. A period of 5 or 7 is
/// not merely absent; it is unreachable.
///
/// `None` is the projective boundary `[1:0]`: an open orbit, charged as
/// shear rather than sounding a pitch.
pub fn compact_period(m: &Mv) -> Spectrum {
    use crate::transducer::sentence::ToralPeriod;
    let Some((ell, _hyp)) = bivector_split(m) else {
        // THE SPLIT DOES NOT EXIST OVER ℚ. The commuting eigenplanes live
        // in `ℚ(√Δ)` and this is an algebraic obstruction, NOT an open
        // orbit — reporting it as one would be a refusal counted as a
        // measurement (rule 11).
        return Spectrum::Undecomposed;
    };
    if ell.is_zero() {
        // No compact part: a pure transvection, open by construction.
        return Spectrum::Open;
    }
    // THE COMPACT PART MUST TURN IN ONE PLANE for a single period to name
    // it. An isoclinic double rotation needs a second split, which this
    // does not perform — refused rather than reported as one plane's order.
    if !bivector_is_simple(&ell) {
        return Spectrum::Isoclinic;
    }
    let (Some(omega), ) = (cayley(&ell), ) else {
        return Spectrum::Undecomposed;
    };
    let Some(w) = is_versor(&omega) else {
        return Spectrum::Undecomposed;
    };
    let s = omega.scalar_part();
    // `cos²(θ/2) = s²/w`, which is `half_angle`'s own reading — the one
    // writer of Niven's five rationals in this tree (rule 12).
    let Some((k, denom)) = crate::transducer::alphabet::half_angle(&s, &w) else {
        // NIVEN: the cosine is rational and is not one of the five, so the
        // winding is irrational and the orbit is dense. Genuinely open,
        // with a non-zero compact part — the trap, read correctly.
        return Spectrum::Open;
    };
    // `θ = 2kπ/m`, so the order is `m / gcd(k, m)`; `k = 0` is no turn.
    let order = match k {
        0 => 1,
        _ => denom / gcd_u32(k, denom),
    };
    let Some(plane) = std::num::NonZeroU32::new(order) else {
        return Spectrum::Undecomposed;
    };
    // ONE PLANE TURNS AND THE ORTHOGONAL ONE IS IDLE, which is a period of
    // one and not an absence: an idle plane closes trivially and earns no
    // yield, where absence would say the factor turns forever.
    Spectrum::Closes(ToralPeriod { plane_1: Some(plane), plane_2: std::num::NonZeroU32::new(1) })
}

/// WHAT A MOTION'S ACOUSTIC READING IS — and the three ways there is none,
/// kept apart (rule 11).
///
/// A first cut returned `Option<ToralPeriod>` and every `None` was counted
/// as an open orbit: 545 of 545 "open", which folded a genuine dense
/// winding together with two algebraic refusals and reported the sum as a
/// measurement. An orbit that does not close and a split that does not
/// exist are different facts about different things.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spectrum {
    /// The compact part closes at a crystallographic order.
    Closes(crate::transducer::sentence::ToralPeriod),
    /// The orbit genuinely never returns: either there is no compact part
    /// (a pure dislocation) or its winding is irrational (Niven). This is
    /// the projective boundary `[1:0]`, charged as shear.
    Open,
    /// `√Δ ∉ ℚ`: the commuting eigenplanes live in `ℚ(√Δ)` and the motion
    /// cannot be decomposed into rational simple parts without extending
    /// the field. An obstruction, not an orbit.
    Undecomposed,
    /// The compact part turns in two planes at once. It has a spectrum;
    /// this reader does not take it, and says so rather than reporting one
    /// plane's order as the whole.
    Isoclinic,
}

fn gcd_u32(a: u32, b: u32) -> u32 {
    match b {
        0 => a.max(1),
        _ => gcd_u32(b, a % b),
    }
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

/// The Euclidean rotation versor in the `e1e2` plane as an EXACT even
/// multivector `s + b·e1e2` (unnormalized: the sandwich carries the scale
/// `s²+b²`, absorbed by homogeneity on the cone). The quarter turn is
/// `s=0, b=1` — the hole a rational chart cannot reach, an ordinary citizen here.
pub fn rotator(s: &BigRational, b: &BigRational) -> Mv {
    rotator_on_blade(s, b, E1 | E2)
}

/// A rotation versor on any bivector blade of the algebra: the three base
/// planes `e1e2`, `e1e3`, `e2e3` are the rotor chart's `(0,1)`, `(0,3)`,
/// `(1,3)` (51.2).
pub fn rotator_on_blade(s: &BigRational, b: &BigRational, blade: usize) -> Mv {
    Mv::scalar(s.clone()).add(&Mv::blade(blade, b.clone()))
}

/// Read the base point `(x1, x2, x3)` back out of a homogeneous conformal
/// point or sphere: the scale is `α = −(P·e_∞)`, then `x = (P_e1, P_e2,
/// P_e3)/α`. `None` at infinity (`α = 0`).
pub fn base(p: &Mv) -> Option<(BigRational, BigRational, BigRational)> {
    let alpha = -p.geo(&e_inf()).scalar_part();
    if alpha.is_zero() {
        return None;
    }
    Some((p.coeff(E1) / &alpha, p.coeff(E2) / &alpha, p.coeff(E3) / &alpha))
}

/// Does base point `p` lie on dual sphere `S`? `X(p)·S = 0`, exact.
pub fn on_sphere(p: &Mv, s: &Mv) -> bool {
    p.geo(s).scalar_part().is_zero()
}

/// THE BOUNDARY PROJECTION `∂σ = σ ⌟ I⁻¹` — the dual contraction against the
/// pseudoscalar, the bridge from the volumetric body to its boundary
/// 2-chain on the cube complex (which then enters the Mayer–Vietoris / L2 coboundary
/// sequence for settlement). Here realized as the geometric product with the
/// inverse pseudoscalar; the caller reads the grade it needs.
pub fn boundary_dual(sigma: &Mv) -> Option<Mv> {
    let i = Mv::blade(PSEUDO, one());
    Some(sigma.geo(&i.versor_inverse()?))
}

// ---- the rational conformal group Spin(4,1)(ℚ): dilation, reflection, tests ----

/// The bivector `E = e_o ∧ e_∞` (the dilation generator's plane).
fn e_plane() -> Mv {
    // e_o e_∞ = e_o·e_∞ + e_o∧e_∞ = −1 + E.
    e_o().geo(&e_inf()).add(&Mv::scalar(one()))
}

/// THE DILATOR by an exact rational ρ > 0, as an UNNORMALIZED versor
/// `(ρ+1) ± (ρ−1) E`: a positive scalar multiple of exp(±½ ln ρ E), so the
/// sandwich carries points on the cone to their ρ-scaled images up to the
/// homogeneous scale, with no logarithm and no root. The orientation of E is
/// settled against the embedding itself (the form that carries X(1,0,0) to
/// X(ρ,0,0)), never by a chosen sign.
pub fn dilator(rho: &BigRational) -> Mv {
    let e = e_plane();
    let plus = Mv::scalar(rho + one()).add(&e.scale(&(rho - one())));
    let target = point(rho, &zero(), &zero());
    let probe = plus.sandwich(&point(&one(), &zero(), &zero()));
    match projective_ratio(&probe, &target) {
        Some(_) => plus,
        None => Mv::scalar(rho + one()).sub(&e.scale(&(rho - one()))),
    }
}

/// THE REFLECTOR across the plane `e1 = 0` (rows negated): the odd versor
/// `e1`, whose sandwich sends `X(x1, x2, x3)` to `X(−x1, x2, x3)` up to the
/// homogeneous scale.
pub fn reflector() -> Mv {
    Mv::blade(E1, one())
}

/// Is `m` a versor: `m m̃` a nonzero scalar? Returns that scalar.
pub fn is_versor(m: &Mv) -> Option<BigRational> {
    let prod = m.geo(&m.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&Mv::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s)
}

/// PROJECTIVE EQUALITY of two blades: `a = λ b` for some λ ∈ ℚ×. The test is
/// `a b̃` having a scalar part only, nonzero; returns λ. This is the exact
/// closed-form check replay and verification run — never a wedge, never a
/// rasterization.
pub fn projective_ratio(a: &Mv, b: &Mv) -> Option<BigRational> {
    let bb = b.norm2();
    if bb.is_zero() {
        // A null blade: `a b̃` has no scalar part at all, so the reverse
        // product decides nothing here. Exact componentwise proportionality
        // is what projective equality is for these.
        return proportional(a, b);
    }
    let prod = a.geo(&b.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&Mv::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s / bb)
}

/// Exact componentwise proportionality `a = λ b` (λ ≠ 0), for null blades
/// where the reverse product carries no scalar.
fn proportional(a: &Mv, b: &Mv) -> Option<BigRational> {
    let mut lambda: Option<BigRational> = None;
    for mask in 0..BLADES {
        let (x, y) = (a.coeff(mask), b.coeff(mask));
        match (x.is_zero(), y.is_zero()) {
            (true, true) => continue,
            (true, false) | (false, true) => return None,
            (false, false) => {
                let l = x / y;
                match &lambda {
                    None => lambda = Some(l),
                    Some(k) if *k == l => {}
                    Some(_) => return None,
                }
            }
        }
    }
    lambda
}

impl Mv {
    /// Serialize the 32 exact coefficients: per coefficient `[sign u8][nlen
    /// u32][numerator][dlen u32][denominator]`, little-endian magnitudes.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        for mask in 0..BLADES {
            let v = &self.c[mask];
            let (ns, nb) = v.numer().to_bytes_le();
            let db = v.denom().to_bytes_le().1;
            out.push(match ns {
                num_bigint::Sign::Minus => 2,
                num_bigint::Sign::NoSign => 0,
                num_bigint::Sign::Plus => 1,
            });
            out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
            out.extend_from_slice(&nb);
            out.extend_from_slice(&(db.len() as u32).to_le_bytes());
            out.extend_from_slice(&db);
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Mv> {
        let mut m = Mv::zero();
        for mask in 0..BLADES {
            let sign = *b.get(*pos)?;
            *pos += 1;
            let nl = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            let nb = b.get(*pos..*pos + nl)?;
            *pos += nl;
            let dl = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
            *pos += 4;
            let db = b.get(*pos..*pos + dl)?;
            *pos += dl;
            let s = match sign {
                2 => num_bigint::Sign::Minus,
                1 => num_bigint::Sign::Plus,
                _ => num_bigint::Sign::NoSign,
            };
            let numer = BigInt::from_bytes_le(s, nb);
            let denom = BigInt::from_bytes_le(num_bigint::Sign::Plus, db);
            if denom.is_zero() {
                return None;
            }
            m.c[mask] = BigRational::new(numer, denom);
        }
        Some(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(n: i64, d: i64) -> BigRational {
        BigRational::new(BigInt::from(n), BigInt::from(d))
    }

    /// 54.7 R3/R4 — THE BIVECTOR TRICHOTOMY IS A READING, AND `Cl(4,0)`
    /// CANNOT MAKE IT.
    ///
    /// Computed, never asserted. The three types are distinguished by the
    /// sign of `I₁ = ⟨B²⟩₀/w`, and the identity is distinguished from a
    /// null shear by `B` itself, which is the split the engine had collapsed.
    #[test]
    fn the_four_bivector_readings_are_distinct_and_exact() {
        use crate::network::nerve::coord::BivectorKind;
        let kind = |m: &Mv| m.class_invariants().map(|(_, _, k)| k);

        // ELLIPTIC: a quarter turn on e₁e₂. B² = −1 < 0.
        let rot = rotator(&q(0, 1), &q(1, 1));
        assert_eq!(kind(&rot), Some(BivectorKind::Elliptic));

        // HYPERBOLIC: a geodesic transvection on EVERY generator of ℍ⁴,
        // `e₊∧e₋` included. B² = +1 > 0. There is NO element of Cl(4,0)
        // with this reading — its metric is positive definite, so
        // ⟨B²⟩₀ ≤ 0 for every bivector it has.
        for plane in TRANSVECTION_PLANES {
            let tv = transvection(&q(2, 1), &q(1, 1), plane)
                .expect("off the light cone, this is a transvection");
            assert_eq!(
                kind(&tv),
                Some(BivectorKind::Hyperbolic),
                "generator {plane:05b} is a boost of hyperbolic 4-space"
            );
        }
        assert_eq!(TRANSVECTION_PLANES.len(), 4, "ℍ⁴ has four boosts, not three");

        // TRIVIAL: B = 0. The identity turns nothing.
        assert_eq!(kind(&Mv::scalar(one())), Some(BivectorKind::Trivial));

        // PARABOLIC: the generator is built on `e_∞`, where `e_∞² = 0` —
        // NOT on `e₊∧e₋`, which is the fourth boost above. The conformal
        // translator is exactly this type, which is what makes it the
        // flatness R3 removes.
        let t = translator(&q(1, 1), &q(0, 1), &q(0, 1));
        assert_eq!(kind(&t), Some(BivectorKind::Parabolic), "T(d) fixes the point at infinity");
        assert!(e_inf().geo(&e_inf()).is_zero(), "e_∞² = e₋² + e₊² = 0");

        // And an element on the light cone `Ω Ω̃ = 0` has no inverse, so it
        // is refused rather than handed back as a versor. The weight is the
        // geometric product's, never `a² − b²` (rule 11).
        assert!(transvection(&q(1, 1), &q(1, 1), TRANSVECTION_PLANES[0]).is_none());
        assert!(transvection(&q(1, 1), &q(0, 1), E1 | E2).is_none(), "e₁e₂ is not a hyperbolic generator");
    }

    /// 54.7 — THE CANONICAL SPLIT, verified by the algebra and not asserted.
    ///
    /// A screw's pitch is the frequency of its TWIST; its slip along the
    /// axis is open and sounds nothing. The split is what lets a period be
    /// asked of the part that can answer.
    #[test]
    fn a_screw_splits_into_commuting_compact_and_open_parts() {
        // A COUPLED generator: a rotation in e₁e₂ (a² = 1) and a boost on
        // e₃e₋ at half magnitude (b² = ¼). Worked by hand:
        //   ⟨B²⟩₀ = −1 + ¼ = −¾ ,  Q = e₁e₂e₃e₋ ,  κ = Q² = −1
        //   disc = 9/16 + 1 = 25/16 ,  √disc = 5/4
        //   b² = (−¾ + 5/4)/2 = ¼ ,  a² = ¼ + ¾ = 1
        let screw = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(E3 | EM, q(1, 2)));
        assert!(!bivector_is_simple(&screw), "the generator couples two planes");
        let (ell, hyp) = bivector_split(&screw).expect("the split is exact over ℚ here");

        // THE BLADES, not their magnitudes: each part carries its own PLANE
        // and that is the whole point. `e₁e₂` is the twist, `e₃e₋` the slip.
        assert_eq!(ell, Mv::blade(E1 | E2, q(1, 1)), "the compact part IS the rotation plane");
        assert_eq!(hyp, Mv::blade(E3 | EM, q(1, 2)), "the open part IS the boost plane");
        assert!(ell.geo(&ell).scalar_part().is_negative(), "B_ell² < 0");
        assert!(hyp.geo(&hyp).scalar_part().is_positive(), "B_hyp² > 0");
        assert!(ell.geo(&hyp).sub(&hyp.geo(&ell)).is_zero(), "[B_ell, B_hyp] = 0");
        assert!(ell.add(&hyp).sub(&screw.grade(2)).is_zero(), "B_ell + B_hyp = B");
        assert!(bivector_is_simple(&ell) && bivector_is_simple(&hyp), "each part turns in ONE plane");

        // A SIMPLE generator carries the whole blade in one part, and WHICH
        // is the sign of ⟨B²⟩₀ — a reading, not a convention.
        let rot = Mv::blade(E1 | E2, q(1, 1));
        let (e3, h3) = bivector_split(&rot).expect("a rotation splits");
        assert!(h3.is_zero() && e3 == rot, "a rotation is wholly compact, plane intact");
        let boost = Mv::blade(TRANSVECTION_PLANES[0], q(1, 1));
        let (e4, h4) = bivector_split(&boost).expect("a boost splits");
        assert!(e4.is_zero() && h4 == boost, "a transvection is wholly open, plane intact");

        // THE COMPACT PART IS THE ONLY ONE THAT CAN SOUND, and evolving it
        // proves which: the twist is elliptic, the slip hyperbolic.
        use crate::network::nerve::coord::BivectorKind;
        let turn = cayley(&ell).expect("the compact part evolves");
        assert_eq!(turn.class_invariants().map(|(_, _, k)| k), Some(BivectorKind::Elliptic));
        let slip = cayley(&hyp).expect("the open part evolves");
        assert_eq!(slip.class_invariants().map(|(_, _, k)| k), Some(BivectorKind::Hyperbolic));
        assert!(e4.is_zero(), "a pure dislocation has no compact part to sound");
    }

    /// 54.7 AXIOM B — A LAMINA IS A RATIONAL POINT OF `ℍ³`, and the
    /// transition between two of them is a DIRECTED transvection.
    ///
    /// Every assertion here is a blade or a product, never a magnitude.
    #[test]
    fn a_lamina_lands_on_the_sheet_and_transports_along_a_geodesic() {
        let m = q(4, 1);
        let a = hyperbolic_lamina(&q(0, 1), &q(0, 1), &q(0, 1), &m).expect("the pole");
        let b = hyperbolic_lamina(&q(1, 1), &q(0, 1), &q(0, 1), &m).expect("one step");
        let c = hyperbolic_lamina(&q(0, 1), &q(1, 1), &q(0, 1), &m).expect("one step, other axis");

        // ON THE SHEET, exactly — `v² = −1` with no root taken anywhere.
        for v in [&a, &b, &c] {
            assert_eq!(v.geo(v), Mv::scalar(-one()), "v² = −1 on the hyperboloid");
        }
        assert_eq!(a, hyperboloid_pole(), "u = 0 is the pole");
        // OFF THE CHART IS REFUSED, not clamped: `|u| ≥ 1`.
        assert!(hyperbolic_lamina(&q(4, 1), &q(0, 1), &q(0, 1), &m).is_none());

        // THE TRANSITION TRANSPORTS, and is not an involution.
        let r = geodesic_transvection(&a, &b).expect("a geodesic joins them");
        let w = is_versor(&r).expect("R R̃ is a positive rational scalar");
        assert!(w.is_positive(), "R R̃ = 2(1 − v_a·v_b) ≥ 4, got {w}");
        assert_eq!(r.sandwich(&a).scale(&(one() / &w)), b, "R v_a R̃ / R R̃ = v_b exactly");

        // THE PLANE IS CARRIED: ⟨R⟩₂ is the geodesic 2-blade, simple, and
        // HYPERBOLIC — `(v_a∧v_b)² = cosh²d − 1 > 0`.
        let plane = r.grade(2);
        assert!(!plane.is_zero(), "the generator is a blade, not a scalar");
        assert!(bivector_is_simple(&r), "one geodesic spans one plane");
        assert!(
            plane.geo(&plane).scalar_part().is_positive(),
            "the geodesic plane is timelike: a transvection, not a rotation"
        );

        // AND IT IS NOT ORDER TWO. `Ω = mn` on the null cone squares to a
        // scalar and merely swaps its endpoints; this propagates.
        assert!(
            !r.geo(&r).sub(&Mv::scalar(r.geo(&r).scalar_part())).is_zero()
                || !r.geo(&r).grade(2).is_zero()
                || {
                    // If R² happened to be scalar, transporting twice would
                    // return to the start — which a real transvection does
                    // not do.
                    let twice = r.geo(&r);
                    let wt = is_versor(&twice).expect("a versor squared is a versor");
                    twice.sandwich(&a).scale(&(one() / wt)) != a
                },
            "a transvection propagates; an involution returns"
        );

        // TWO AXES DO NOT COMMUTE — the curvature, measured.
        let rb = geodesic_transvection(&a, &b).expect("step one way");
        let rc = geodesic_transvection(&a, &c).expect("step the other");
        assert!(!rb.geo(&rc).projective_eq(&rc.geo(&rb)), "boosts do not commute on ℍ³");
    }

    /// A FRAME'S BASE POINT is its versor applied to the pole, back on the
    /// sheet — one exact division, no root.
    #[test]
    fn a_frames_base_point_returns_to_the_sheet() {
        let m = q(4, 1);
        let a = hyperbolic_lamina(&q(1, 1), &q(0, 1), &q(0, 1), &m).expect("a lamina");
        let r = geodesic_transvection(&hyperboloid_pole(), &a).expect("a geodesic");
        let p = frame_base_point(&r).expect("the frame stands somewhere");
        assert_eq!(p.geo(&p), Mv::scalar(-one()), "and that somewhere is on the sheet");
        assert_eq!(p, a, "the transvection's own base point is where it carried the pole");
    }

    /// 54.7 — THE NIVEN TRAP: an open orbit has TWO causes, and reading
    /// only the first would report a dense winding as a closed circuit.
    #[test]
    fn a_period_is_crystallographic_or_it_is_infinite() {
        // A pure transvection has no compact part: open, and no pitch.
        let boost = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("a boost");
        assert_eq!(compact_period(&boost), Spectrum::Open, "a dislocation slips and never returns");

        // A quarter turn closes at a crystallographic order, which is ℚ's
        // doing and not a convention.
        let quarter = rotator(&q(0, 1), &q(1, 1));
        let Spectrum::Closes(p) = compact_period(&quarter) else { panic!("a quarter turn closes") };
        let ell = p.plane_1.expect("the turning plane has an order").get();
        assert_eq!(ell, 2, "s = 0 is the half-angle π/2, so the turn is by π");
        assert!(matches!(ell, 1 | 2 | 3 | 4 | 6), "Niven allows only these, got {ell}");

        // THE TRAP ITSELF. `cos θ = (s²−b²)/(s²+b²) = −7/25` is RATIONAL and
        // is not one of Niven's five, so `θ/π` is irrational: the rotor is
        // purely elliptic — a non-zero compact part — and its orbit is dense
        // and closes never. `B_ell ≠ 0` does not imply a finite period.
        let pyth = rotator(&q(3, 1), &q(4, 1));
        let (e, h) = bivector_split(&pyth).expect("a simple rotation splits");
        assert!(!e.is_zero(), "the (3,4,5) rotor HAS a compact part");
        assert!(h.is_zero(), "and no open part at all");
        assert_eq!(
            compact_period(&pyth),
            Spectrum::Open,
            "yet cos θ = −7/25 winds densely: OPEN, not undecomposed — the \
             split exists and the orbit still never closes"
        );
    }

    /// 54.7 R3 — A GENERATOR IS NOT A MOTION, and Cayley is the finite
    /// action over ℚ.
    ///
    /// `exp(½τB)` leaves the rationals for every `τ` worth having, and the
    /// two ways round it — floating point, or pinning `Ω` to one blade —
    /// lose exactness and the grade-4 coupling respectively. This measures
    /// that Cayley loses neither: a SCREW generator evolves to a screw
    /// versor, `I₂ ≠ 0` on both sides.
    #[test]
    fn cayley_evolves_a_generator_to_a_finite_versor_and_keeps_the_coupling() {
        use crate::network::nerve::coord::BivectorKind;
        // A SIMPLE hyperbolic generator evolves to a hyperbolic motion.
        let boost_gen = Mv::blade(TRANSVECTION_PLANES[0], q(1, 3));
        let boost = cayley(&boost_gen).expect("1 + B is invertible off the pole");
        assert_eq!(
            boost.class_invariants().map(|(_, _, k)| k),
            Some(BivectorKind::Hyperbolic),
            "the finite action of a boost generator is a boost"
        );
        assert!(bivector_is_simple(&boost), "one geodesic stays one geodesic");

        // A COUPLED generator — a rotation plane and a boost plane at once —
        // evolves to a screw, and `I₂` survives the passage. Under
        // `Ω = a + bB` this motion has no representation at all.
        let screw_gen = Mv::blade(E1 | E2, q(1, 2)).add(&Mv::blade(E3 | EM, q(1, 5)));
        assert!(!bivector_is_simple(&screw_gen), "the generator couples two planes");
        let screw = cayley(&screw_gen).expect("1 + B is invertible");
        assert!(!bivector_is_simple(&screw), "and so does its finite action");
        let (_, i2, _) = screw.class_invariants().expect("a versor has invariants");
        assert!(!i2.is_zero(), "I₂ = ⟨B∧B⟩₄ survives evolution, got {i2}");

        // The transform's pole is refused, never approximated.
        assert!(cayley(&Mv::scalar(one())).is_none(), "a scalar is not a generator");

        // AND A LOOP'S HOLONOMY IS THE ORDERED PRODUCT OF ITS EDGES. A
        // closed chain of transports that returns is flat; one that does
        // not is the obstruction the cell traps.
        let inv = boost.versor_inverse().expect("a versor inverts");
        let flat = transport_around(&[boost.clone(), inv]).expect("a closed transport");
        assert!(flat.projective_eq(&Mv::scalar(one())), "Ω Ω⁻¹ = 1 is a flat cell");
        let held = transport_around(&[boost.clone(), screw.clone()]).expect("a transport");
        assert!(!held.projective_eq(&Mv::scalar(one())), "a cell that traps an obstruction");
        assert!(
            !boost.geo(&screw).projective_eq(&screw.geo(&boost)),
            "the product is path-ORDERED: transport does not commute"
        );
    }

    /// 54.7, Anti-Flattening clause 5 — A SCREW IS REACHABLE, which is what
    /// the single-blade form `Ω = a + bB` made impossible.
    ///
    /// That form spans `ℚ[B] ≅ ℚ ⊕ ℚ`, a two-dimensional split-complex
    /// slice, and its generator is rank one — so `I₂ = ⟨B∧B⟩₄ = 0`
    /// identically, no matter what `a` and `b` are. Every motion built that
    /// way turns in one plane and slides along one track. Reflection
    /// factorization has no such ceiling, and this MEASURES the difference:
    /// a simple boost reads `I₂ = 0`, a coupled motion reads `I₂ ≠ 0`.
    #[test]
    fn a_coupled_screw_has_a_non_zero_second_casimir() {
        let z = q(0, 1);
        let v = |a: (i64, i64), b: (i64, i64), c: (i64, i64), p: (i64, i64), m: (i64, i64)| {
            reflector_vector(&q(a.0, a.1), &q(b.0, b.1), &q(c.0, c.1), &q(p.0, p.1), &q(m.0, m.1))
                .expect("a non-null vector reflects")
        };
        // Two reflection pairs on orthogonal planes: the first pair turns in
        // e₁e₂, the second boosts along e₃e₋. Their product couples them.
        let screw = versor_of_reflections(&[
            v((1, 1), (0, 1), (0, 1), (0, 1), (0, 1)),
            v((1, 1), (1, 1), (0, 1), (0, 1), (0, 1)),
            v((0, 1), (0, 1), (1, 1), (0, 1), (0, 1)),
            v((0, 1), (0, 1), (2, 1), (0, 1), (1, 1)),
        ])
        .expect("four reflections compose to a versor");
        assert!(
            !bivector_is_simple(&screw),
            "B ∧ B ≠ 0: the generator couples two planes, which a single blade cannot"
        );
        let (_, i2, _) = screw.class_invariants().expect("a versor has invariants");
        assert!(!i2.is_zero(), "I₂ = ⟨B∧B⟩₄ is non-zero on a screw, got {i2}");

        // A single-generator boost is simple, honestly: one geodesic, one
        // plane. The point is that the REPRESENTATION no longer forces it.
        let boost = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("a boost");
        assert!(bivector_is_simple(&boost), "one geodesic has a rank-one generator");
        let (_, i2_boost, _) = boost.class_invariants().expect("a versor has invariants");
        assert_eq!(i2_boost, z, "I₂ = 0 on a simple motion, which is a reading and not a ceiling");

        // An odd number of reflections is not in the spin group, and a null
        // vector reflects nothing — both refused rather than multiplied in.
        assert!(versor_of_reflections(&[v((1, 1), (0, 1), (0, 1), (0, 1), (0, 1))]).is_none());
        assert!(reflector_vector(&z, &z, &z, &q(1, 1), &q(1, 1)).is_none(), "e₊+e₋ is null");
    }

    /// 51.2, Anti-Flattening clause 7 — TRANSVECTIONS DO NOT COMMUTE, and
    /// the failure IS the curvature. On a flat base the commutator of two
    /// translations is the identity; here it is a rotation, and that is
    /// read off the algebra rather than asserted as a value.
    #[test]
    fn two_boosts_do_not_commute_and_their_commutator_is_a_rotation() {
        use crate::network::nerve::coord::BivectorKind;
        let a = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("boost on e₁e₋");
        let b = transvection(&q(3, 1), &q(1, 1), TRANSVECTION_PLANES[1]).expect("boost on e₂e₋");
        assert!(!a.geo(&b).projective_eq(&b.geo(&a)), "ΩₐΩ_b ≠ Ω_bΩₐ: the base is not flat");
        let c = transvection_commutator(&a, &b).expect("the commutator is a versor");
        assert!(!c.projective_eq(&Mv::scalar(one())), "a non-zero curvature is a non-trivial commutator");
        assert_eq!(
            c.class_invariants().map(|(_, _, k)| k),
            Some(BivectorKind::Elliptic),
            "the commutator of two boosts is a ROTATION scaled by the sectional curvature"
        );
        // The conformal translators, by contrast, commute — which is the
        // statement that THAT reading is of a flat lattice (51.2).
        let (t1, t2) = (
            translator(&q(1, 1), &q(0, 1), &q(0, 1)),
            translator(&q(0, 1), &q(1, 1), &q(0, 1)),
        );
        assert!(t1.geo(&t2).projective_eq(&t2.geo(&t1)), "e_i ∧ e_∞ commute");
    }

    /// EVERY ROTOR LIFTS ELLIPTIC, and that is the statement that `Cl(4,0)`
    /// has no transvection — not a defect of the lift (54.7 R4).
    #[test]
    fn a_lifted_rotor_is_always_elliptic_or_trivial() {
        use crate::network::nerve::coord::BivectorKind;
        use crate::transducer::alphabet::{LetterId, Word};
        for plane in [(0, 1), (0, 2), (1, 3), (2, 3)] {
            let Some(letter) = LetterId::quarter(plane) else { continue };
            let w = Word::from_letters(&[(letter, false)]);
            let Some((_, _, kind)) = w.rotor().class_invariants() else { continue };
            assert!(
                matches!(kind, BivectorKind::Elliptic | BivectorKind::Trivial),
                "a Cl(4,0) rotor on {plane:?} read as {kind:?}; its bivector space is spacelike"
            );
        }
    }

    /// The dilator scales points exactly with no root or logarithm; the
    /// reflector negates rows; both are versors; projective equality reads
    /// the scalar ratio and refuses non-proportional blades; bytes round-trip.
    #[test]
    fn rational_versors_act_exactly_and_are_verified_in_closed_form() {
        let d = dilator(&q(3, 2));
        assert!(is_versor(&d).is_some(), "the dilator is a versor");
        let moved = d.sandwich(&point(&q(2, 1), &q(-4, 1), &q(6, 1)));
        assert_eq!(base(&moved), Some((q(3, 1), q(-6, 1), q(9, 1))), "X(2,−4,6) → X(3,−6,9) at ρ = 3/2");
        let r = reflector();
        let flipped = r.sandwich(&point(&q(2, 1), &q(5, 1), &q(7, 1)));
        assert_eq!(base(&flipped), Some((q(-2, 1), q(5, 1), q(7, 1))));
        let s = sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(5, 1));
        assert_eq!(projective_ratio(&s.scale(&q(7, 3)), &s), Some(q(7, 3)));
        assert!(projective_ratio(&s, &sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(6, 1))).is_none());
        let composed = translator(&q(1, 1), &q(0, 1), &q(2, 1)).geo(&d);
        let mut bytes = Vec::new();
        composed.to_bytes(&mut bytes);
        let mut pos = 0usize;
        assert_eq!(Mv::from_bytes(&bytes, &mut pos), Some(composed));
    }

    #[test]
    fn a4_projective_eq_either_sign_and_key() {
        let t = translator(&q(2, 1), &q(0, 1), &q(0, 1));
        assert!(t.projective_eq(&t.scale(&q(-3, 2))));
        assert_eq!(t.canonical().unwrap().key(), t.scale(&q(-3, 2)).canonical().unwrap().key());
        assert_eq!(t, t.scale(&q(5, 1)), "PartialEq is projective");
        let u = translator(&q(3, 1), &q(0, 1), &q(0, 1));
        assert_ne!(t.key(), u.key());
    }

    /// 51.2 — THE THIRD COORDINATE IS A BASE COORDINATE: translators along
    /// the three axes commute, a rotation in the `e1e3` plane carries the
    /// row axis onto the frame axis exactly, and a body in one frame at
    /// `k = 0` is carried there and nowhere else by a plane motion.
    #[test]
    fn the_frame_axis_is_a_base_coordinate() {
        let (z, o) = (q(0, 1), q(1, 1));
        let t1 = translator(&q(2, 1), &z, &z);
        let t3 = translator(&z, &z, &q(5, 1));
        assert!(t1.geo(&t3).projective_eq(&t3.geo(&t1)), "translators commute (51.2)");
        assert_eq!(
            base(&t1.geo(&t3).sandwich(&point(&z, &z, &z))),
            Some((q(2, 1), z.clone(), q(5, 1))),
            "and compose by adding the displacements"
        );
        let quarter_13 = rotator_on_blade(&o, &o, E1 | E3);
        let moved = base(&quarter_13.sandwich(&point(&o, &z, &z))).expect("a rotation keeps the point finite");
        assert!(moved.0.is_zero() && moved.1.is_zero() && !moved.2.is_zero(), "the row axis turns onto the frame axis: {moved:?}");
        let plane = rotator(&o, &o).geo(&translator(&q(3, 1), &q(-1, 1), &z));
        let carried = base(&plane.sandwich(&point(&q(4, 1), &q(4, 1), &z))).expect("finite");
        assert!(carried.2.is_zero(), "a plane motion keeps k = 0: {carried:?}");
    }
}
