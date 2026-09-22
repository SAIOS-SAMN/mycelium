//! A4 — the multivector of `Cl(4,1)` over ℚ: its thirty-two blades, the
//! geometric product, and the canonical writing a key is built from.

use super::group::{is_versor, projective_ratio};
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
    pub(super) c: [BigRational; BLADES],
}

impl PartialEq for Mv {
    fn eq(&self, other: &Self) -> bool {
        self.projective_eq(other)
    }
}
impl Eq for Mv {}

pub(super) fn zero() -> BigRational {
    BigRational::zero()
}
pub(super) fn one() -> BigRational {
    BigRational::one()
}
pub(super) fn half() -> BigRational {
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
