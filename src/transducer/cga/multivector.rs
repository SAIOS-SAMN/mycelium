//! A4, BQ3, 71.2(3), (4) — THE MULTIVECTOR OF `Cl(n+1, 1)` OVER ℚ: the
//! conformal model of ℚⁿ, with `n` carried by the value and the geometric
//! product written once for every `n`.
//!
//! THE GENERATORS KEEP THEIR BITS AS `n` GROWS. `e₊` is bit 3 and `e₋` is
//! bit 4 at every `n`; the first three base axes are bits 0, 1, 2, and a
//! fourth base axis onward takes bit 5 onward ([`base_axis`]). So at `n ≤ 3`
//! every mask, the canonical order (the first non-zero coefficient in mask
//! order) and the thirty-two coefficients on the wire are what the fixed
//! `Cl(4,1)` carrier wrote, and the oracle it left (`tests/oracle.golden`)
//! is reproduced reading for reading. At `n = 4` there are `2⁶` blades and
//! the first thirty-two are the `n = 3` algebra's own.
//!
//! THE VALUE CARRIES ITS `n`, AND TWO VALUES OF DIFFERENT `n` COMPOSE IN THE
//! LARGER. Because no generator moves, a value of the smaller algebra is the
//! same value in the larger one: ℚⁿ ⊂ ℚᵐ with the extra coordinates zero,
//! an exact inclusion and never a reading of one as the other. A value's
//! `n` is the smallest algebra holding its generators: a scalar is `n = 0`,
//! an element of every algebra.
//!
//! THE METRIC IS THE SIGNATURE'S, NOT A LIST: `e₋` squares to −1 and every
//! other generator to +1, which is `Cl(n+1, 1)` for every `n`.

use super::group::{projective_ratio, weight_of};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

/// The first three base axes — row, column, frame at the corpus's `n = 3` —
/// and the two generators that close the conformal model, `e₊` (square +1)
/// and `e₋` (square −1). These are generators, not counts: a fourth base
/// axis is [`base_axis`]`(4)`.
pub const E1: usize = 0b00001;
pub const E2: usize = 0b00010;
pub const E3: usize = 0b00100;
pub const EP: usize = 0b01000;
pub const EM: usize = 0b10000;

/// The mask of base axis `i`, counted from 1: bits 0–2 for the first three,
/// and bit `i + 1` from the fourth on, so `e₊` and `e₋` never move.
pub fn base_axis(i: usize) -> usize {
    match i {
        1..=3 => 1 << (i - 1),
        _ => 1 << (i + 1),
    }
}

/// How many coefficient slots the carrier holds at `n`: the masks reach bit
/// `n + 1` once `n ≥ 3`, and bit 4 before that.
pub fn slots(n: u8) -> usize {
    1usize << (usize::from(n.max(3)) + 2)
}

/// The blades of `Cl(n+1, 1)`: `2^{n+2}`.
pub fn blades(n: u8) -> usize {
    1usize << (usize::from(n) + 2)
}

/// The unit pseudoscalar at `n`: every base axis, `e₊` and `e₋`.
pub fn pseudo(n: u8) -> usize {
    (1..=usize::from(n)).fold(EP | EM, |m, i| m | base_axis(i))
}

/// The smallest `n` whose algebra holds every generator of `mask`: the
/// highest base axis it names.
fn n_of_mask(mask: usize) -> u8 {
    let mut n = 0usize;
    let mut bit = 0usize;
    let mut rest = mask;
    while rest != 0 {
        if rest & 1 == 1 {
            let axis = match bit {
                0..=2 => bit + 1,
                3 | 4 => 0,
                b => b - 1,
            };
            n = n.max(axis);
        }
        rest >>= 1;
        bit += 1;
    }
    // A mask is a machine word, so its highest axis is below 64 and is a
    // `u8` exactly.
    n as u8
}

/// An exact multivector of `Cl(n+1, 1)`: its `n` and one coefficient over ℚ
/// per slot. Equality is projective (`λ ∈ ℚ×`); coefficient identity is
/// [`Mv::canonical`].
#[derive(Debug, Clone)]
pub struct Mv {
    n: u8,
    c: Vec<BigRational>,
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

/// The zero a slot outside a value's algebra reads as: that value has no
/// coefficient there, which is zero in every larger algebra holding it.
static ZERO: std::sync::LazyLock<BigRational> = std::sync::LazyLock::new(BigRational::zero);

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

/// The signature's sign on the generators two blades share: only `e₋`
/// squares to −1. No generator of the orthonormal basis is null — the null
/// cone is derived — so a blade product never vanishes by metric, only by
/// sign.
fn metric_sign(shared: usize) -> i64 {
    if (shared & EM) != 0 {
        -1
    } else {
        1
    }
}

impl Mv {
    /// The zero: `n = 0`, an element of every algebra.
    pub fn zero() -> Self {
        Self::zero_at(0)
    }

    /// The zero of `Cl(n+1, 1)`.
    pub fn zero_at(n: u8) -> Self {
        Self { n, c: vec![zero(); slots(n)] }
    }

    pub fn scalar(v: BigRational) -> Self {
        let mut m = Self::zero();
        m.c[0] = v;
        m
    }

    /// A single basis blade with coefficient `v`, in the smallest algebra
    /// holding its generators.
    pub fn blade(mask: usize, v: BigRational) -> Self {
        let mut m = Self::zero_at(n_of_mask(mask));
        m.c[mask] = v;
        m
    }

    /// The `n` this value belongs to: the smallest algebra holding it.
    pub fn n(&self) -> u8 {
        self.n
    }

    /// How many coefficient slots the value holds.
    pub fn slot_count(&self) -> usize {
        self.c.len()
    }

    /// The same value in the algebra at `m ≥ n`: the inclusion ℚⁿ ⊂ ℚᵐ, the
    /// new slots zero. A value is never narrowed.
    pub fn widened(&self, m: u8) -> Self {
        if m <= self.n {
            return self.clone();
        }
        let mut c = self.c.clone();
        c.resize(slots(m), zero());
        Self { n: m, c }
    }

    /// The two operands in one algebra: the larger `n` of the two.
    pub(super) fn common(&self, o: &Self) -> (Self, Self) {
        let n = self.n.max(o.n);
        (self.widened(n), o.widened(n))
    }

    pub fn coeff(&self, mask: usize) -> &BigRational {
        match self.c.get(mask) {
            Some(v) => v,
            None => &ZERO,
        }
    }

    pub fn scalar_part(&self) -> BigRational {
        self.c[0].clone()
    }

    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|x| x.is_zero())
    }

    /// A4: divide by |first nonzero|, negate so that coefficient is +1.
    pub fn canonical(&self) -> Option<Mv> {
        let i = (0..self.c.len()).find(|&i| !self.c[i].is_zero())?;
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
        let (a, b) = self.common(o);
        Self { n: a.n, c: a.c.iter().zip(&b.c).map(|(x, y)| x + y).collect() }
    }

    pub fn sub(&self, o: &Self) -> Self {
        let (a, b) = self.common(o);
        Self { n: a.n, c: a.c.iter().zip(&b.c).map(|(x, y)| x - y).collect() }
    }

    pub fn scale(&self, s: &BigRational) -> Self {
        Self { n: self.n, c: self.c.iter().map(|x| x * s).collect() }
    }

    /// THE GEOMETRIC PRODUCT — the whole algebra, for every `n`. `result[i⊕j]
    /// += sign · metric · a_i · b_j` over every pair of populated blades.
    pub fn geo(&self, o: &Self) -> Self {
        let (a, b) = self.common(o);
        let mut r = Self::zero_at(a.n);
        for (i, x) in a.c.iter().enumerate() {
            if x.is_zero() {
                continue;
            }
            for (j, y) in b.c.iter().enumerate() {
                if y.is_zero() {
                    continue;
                }
                let sign = reorder_sign(i, j) * metric_sign(i & j);
                let term = x * y;
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
            n: self.n,
            c: self
                .c
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    let g = (i as u32).count_ones();
                    if (g * (g.wrapping_sub(1)) / 2) & 1 == 0 {
                        v.clone()
                    } else {
                        -v.clone()
                    }
                })
                .collect(),
        }
    }

    /// The grade involution `M̂`: blade of grade g gets sign `(−1)^g`. On a
    /// vector it is the negation; on an even element the identity.
    pub fn involute(&self) -> Self {
        Self {
            n: self.n,
            c: self
                .c
                .iter()
                .enumerate()
                .map(|(i, v)| match (i as u32).count_ones() & 1 {
                    0 => v.clone(),
                    _ => -v.clone(),
                })
                .collect(),
        }
    }

    /// THE TWISTED ADJOINT `M̂ x M̃` (78.28(1)): the action of an even or odd
    /// versor on `x`, up to its weight. For an even `M` it is the sandwich;
    /// for an odd `M` the sandwich negated, which is what a reflection does
    /// to the vector it acts on — so a fixed vector of an odd motion is read
    /// at the same weight as an even one's, never at its negative.
    pub fn twisted(&self, x: &Self) -> Self {
        self.involute().geo(x).geo(&self.reverse())
    }

    /// Occupied grades all even, or all odd: the parity a versor has. The
    /// zero element occupies none and has no parity.
    pub fn parity(&self) -> Option<bool> {
        let mut odd = None;
        for (i, v) in self.c.iter().enumerate() {
            if v.is_zero() {
                continue;
            }
            let o = (i as u32).count_ones() & 1 == 1;
            match odd {
                None => odd = Some(o),
                Some(p) if p != o => return None,
                Some(_) => {}
            }
        }
        odd
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
    /// dependent on the round gives zero the same way; and past grade
    /// `n + 2` the exterior algebra of `ℚ^{n+1,1}` is identically zero, so
    /// one point too many vanishes. Dimensional exhaustion is exterior-
    /// algebra arithmetic, never a `grade > MAX` branch.
    ///
    /// At the corpus's `n = 3` the ladder of rounds is: grade 2 a point pair
    /// (0-sphere), grade 3 a circle (1-sphere), grade 4 a sphere (2-sphere),
    /// grade 5 the pseudoscalar, grade 6 identically zero.
    pub fn wedge_vector(&self, g: u32, v: &Self) -> Self {
        self.geo(v).grade(g + 1)
    }

    /// THE CONJUGACY INVARIANTS OF A TRANSITION, READ OFF ITS GRADE-2
    /// GENERATOR (54.7, as ruled 2026-09-21).
    ///
    /// A transition is `Ω = R · T(τ)` and its generator is the bivector
    /// `B = ⟨Ω⟩₂`. In `Cl(n+1,1)` that bivector is NOT one kind of thing:
    ///
    /// ```text
    ///     B = B_ell + B_hyp ,   B_ell = Σ b_ij (eᵢ∧eⱼ)
    ///                           B_hyp = Σ b_i₋ (eᵢ∧e₋)
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
    /// `Spin(4)` genuinely are the two angles alone.
    ///
    /// It adds everything here. Under a signature with `e₋² = −1` the sign
    /// of `I₁` is a real coordinate, and a rotation, a boost and a limit
    /// rotation are three classes rather than one — which is why the
    /// transition's two halves must be composed into ONE element before the
    /// class is taken.
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
    /// `I₂/w²` is. `None` when `Ω` is not a versor, which is a refusal and
    /// never a zero class (rule 11).
    pub fn class_invariants(
        &self,
    ) -> Option<(BigRational, BigRational, crate::network::nerve::coord::BivectorKind)> {
        // `I₁/w`, `I₂/w²`: scale-free ratios read as the class (Axiom C).
        let w = weight_of(self)?;
        if w.is_zero() {
            return None;
        }
        let b = self.grade(2);
        let bb = b.geo(&b);
        let i1 = bb.scalar_part() / &w;
        let quad = bb.grade(4);
        let i2 = quad.geo(&quad.reverse()).scalar_part() / (&w * &w);
        // THE SIGN IS THE TYPE. With `eᵢ² = +1` for every generator but
        // `e₋² = −1`, all anticommuting:
        //
        //   ELLIPTIC   B = e₁∧e₂ :  B² = −1  < 0
        //   HYPERBOLIC B = eᵢ∧e₋ :  B² = +1  > 0
        //   PARABOLIC  B = −½ d e_∞ : e_∞² = 0 and d ⊥ e_∞, so B² = 0
        //
        // TRIVIAL IS NOT PARABOLIC (54.7): the identity turns nothing, and
        // `I₁ = 0` alone cannot tell the two apart, so the bivector itself
        // is asked.
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
    /// TORSION AND AXIS CARRIERS ANSWER. The same bytes as
    /// [`Mv::class_key`], declared as `coord::Holonomy`.
    pub fn holonomy_key(&self) -> Option<crate::network::nerve::coord::Holonomy> {
        let (i1, i2, kind) = self.class_invariants()?;
        Some(crate::network::nerve::coord::Holonomy { i1, i2, kind })
    }

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
            n: self.n,
            c: self
                .c
                .iter()
                .enumerate()
                .map(|(i, v)| match (i as u32).count_ones() == g {
                    true => v.clone(),
                    false => zero(),
                })
                .collect(),
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
    /// The same value in the smallest algebra holding its non-zero
    /// coefficients: the inverse of [`Mv::widened`], exact, since every slot
    /// it drops is zero. The zero narrows to `n = 0`.
    pub fn narrowed(&self) -> Self {
        let n = self
            .c
            .iter()
            .enumerate()
            .filter(|(_, v)| !v.is_zero())
            .fold(0u8, |n, (mask, _)| {
                let m = n_of_mask(mask);
                if m > n {
                    m
                } else {
                    n
                }
            });
        let mut c = self.c.clone();
        c.truncate(slots(n));
        Self { n, c }
    }

    /// THE VALUE ON THE WIRE, ITS `n` FIRST (71.2(1), BQ1): the `n` of the
    /// smallest algebra holding it, one byte, then [`Mv::put_coefficients`]
    /// at that `n`. A value and its widening are one value, so they write
    /// one run of bytes; a reader takes the `n` off the bytes and never
    /// assumes one.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        let m = self.narrowed();
        out.push(m.n);
        m.put_coefficients(out);
    }

    /// Every slot's exact coefficient: per coefficient `[sign u8][nlen u32]
    /// [numerator][dlen u32][denominator]`, little-endian magnitudes, in
    /// mask order. At `n ≤ 3` that is thirty-two, the fixed carrier's
    /// coefficients exactly.
    pub fn put_coefficients(&self, out: &mut Vec<u8>) {
        for v in &self.c {
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

    /// The inverse of [`Mv::to_bytes`]: the value's `n`, then its
    /// coefficients at that `n`. `None` where the bytes do not hold them.
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Mv> {
        let n = *b.get(*pos)?;
        *pos += 1;
        Mv::coefficients_at(b, pos, n)
    }

    /// The inverse of [`Mv::put_coefficients`] at `n`: `slots(n)`
    /// coefficients.
    pub fn coefficients_at(b: &[u8], pos: &mut usize, n: u8) -> Option<Mv> {
        let mut m = Mv::zero_at(n);
        for slot in m.c.iter_mut() {
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
            *slot = BigRational::new(numer, denom);
        }
        Some(m)
    }
}
