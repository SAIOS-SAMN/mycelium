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
    pub fn class_invariants(&self) -> Option<(BigRational, BigRational, u8)> {
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
        let ty = if i1.is_negative() {
            0
        } else if i1.is_positive() {
            1
        } else {
            2
        };
        Some((i1, i2, ty))
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
    pub fn holonomy_key(
        &self,
    ) -> Option<crate::network::nerve::Key<crate::network::nerve::coord::Holonomy>> {
        crate::network::nerve::Key::from_address(&self.class_key()?).ok()
    }

    /// The class as canonical coordinates (A21, law 2): the two invariants
    /// written out and the bivector's type, never a digest.
    pub fn class_key(&self) -> Option<crate::transducer::address::Address> {
        let (i1, i2, ty) = self.class_invariants()?;
        let mut buf = vec![crate::transducer::address::TAG_CLASS];
        crate::transducer::alphabet::put_q(&mut buf, &i1);
        crate::transducer::alphabet::put_q(&mut buf, &i2);
        buf.push(ty);
        Some(crate::transducer::address::Address::new(buf))
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
pub fn translator(d1: &BigRational, d2: &BigRational, d3: &BigRational) -> Mv {
    let d = Mv::blade(E1, d1.clone()).add(&Mv::blade(E2, d2.clone())).add(&Mv::blade(E3, d3.clone()));
    let d_einf = d.geo(&e_inf());
    Mv::scalar(one()).sub(&d_einf.scale(&half()))
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
