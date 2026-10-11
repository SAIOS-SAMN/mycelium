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
//! THE CARRIER HOLDS THE BLADES THE VALUE HAS, AND NO OTHERS. A value is its
//! non-zero terms, each a blade mask and an exact coefficient, in mask
//! order; a slot it does not hold is zero. Nothing is sized by `2^{n+2}`: a
//! single blade of a large algebra is one term, and the geometric product
//! runs over the terms the two values hold. The algebra is bounded by its
//! masks — machine words — at [`MAX_N`], and a generator past it is refused
//! where it would be made ([`base_axis`]), never shifted into a wrong mask.
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

/// THE LARGEST `n` THE ALGEBRA HOLDS: base axis `i ≥ 4` is bit `i + 1` of a
/// machine-word mask, and the slots of `Cl(n+1, 1)` are `2^{n+2}` masks, so
/// the last axis whose algebra's masks are all words is `usize::BITS − 3`.
#[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — usize::BITS − 3 is below 256 on every target")]
pub const MAX_N: u8 = (usize::BITS - 3) as u8;

/// The mask of base axis `i`, counted from 1: bits 0–2 for the first three,
/// and bit `i + 1` from the fourth on, so `e₊` and `e₋` never move. `None`
/// for axis 0 (there is none) and past [`MAX_N`]: an axis the algebra's
/// masks cannot hold is refused, never wrapped into another axis's bit.
pub fn base_axis(i: usize) -> Option<usize> {
    match i {
        0 => None,
        1..=3 => Some(1 << (i - 1)),
        _ if i <= usize::from(MAX_N) => Some(1 << (i + 1)),
        _ => None,
    }
}

/// THE BASE AXES OF `Cl(n+1, 1)`, each by its mask, in axis order: those
/// the algebra's masks hold (every one, for `n ≤ MAX_N`).
pub fn axes(n: u8) -> impl Iterator<Item = usize> + Clone {
    (1..=usize::from(n)).map_while(base_axis)
}

/// How many masks the algebra at `n` spans on the wire: the masks reach bit
/// `n + 1` once `n ≥ 3`, and bit 4 before that. `None` past [`MAX_N`].
pub fn slots(n: u8) -> Option<usize> {
    if n > MAX_N {
        return None;
    }
    1usize.checked_shl(u32::from(n.max(3)) + 2)
}

/// The blades of `Cl(n+1, 1)`: `2^{n+2}`. `None` past [`MAX_N`].
pub fn blades(n: u8) -> Option<usize> {
    if n > MAX_N {
        return None;
    }
    1usize.checked_shl(u32::from(n) + 2)
}

/// The unit pseudoscalar at `n`: every base axis, `e₊` and `e₋`. `None` past
/// [`MAX_N`].
pub fn pseudo(n: u8) -> Option<usize> {
    (1..=usize::from(n)).try_fold(EP | EM, |m, i| Some(m | base_axis(i)?))
}

/// The smallest `n` whose algebra holds every generator of `mask`: the
/// highest base axis it names.
#[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the algebra's range: an axis of a machine-word blade mask, below usize::BITS ≤ 64")]
fn n_of_mask(mask: usize) -> u8 {
    // The highest set bit is the highest generator; e₊, e₋ name no axis.
    let mut rest = mask & !(EP | EM);
    let mut n = 0usize;
    let mut bit = 0usize;
    while rest != 0 {
        if rest & 1 == 1 {
            n = match bit {
                0..=2 => bit + 1,
                b => b - 1,
            };
        }
        rest >>= 1;
        bit += 1;
    }
    // A mask is a machine word, so its highest axis is below 64 and is a
    // `u8` exactly.
    n as u8
}

/// An exact multivector of `Cl(n+1, 1)`: its `n` and its non-zero terms, each
/// a blade mask and a coefficient over ℚ, strictly ascending by mask.
/// Equality is projective (`λ ∈ ℚ×`); coefficient identity is
/// [`Mv::canonical`].
#[derive(Debug, Clone)]
pub struct Mv {
    n: u8,
    c: Vec<(usize, BigRational)>,
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

/// The zero a blade the value does not hold reads as.
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

/// The grade of a blade: how many generators its mask names.
fn grade_of(mask: usize) -> u32 {
    mask.count_ones()
}

impl Mv {
    /// A value from terms in any order, at `n`: like masks summed, zeros
    /// dropped, the rest in mask order.
    fn of_terms(n: u8, mut terms: Vec<(usize, BigRational)>) -> Self {
        terms.sort_unstable_by_key(|(m, _)| *m);
        let mut c: Vec<(usize, BigRational)> = Vec::with_capacity(terms.len());
        for (m, v) in terms {
            match c.last_mut() {
                Some((last, sum)) if *last == m => *sum += v,
                _ => c.push((m, v)),
            }
        }
        c.retain(|(_, v)| !v.is_zero());
        Self { n, c }
    }

    /// The same blades with each coefficient mapped; zeros dropped.
    fn map_terms(&self, f: impl Fn(usize, &BigRational) -> BigRational) -> Self {
        let c = self.c.iter().map(|(m, v)| (*m, f(*m, v))).filter(|(_, v)| !v.is_zero()).collect();
        Self { n: self.n, c }
    }

    /// The zero: `n = 0`, an element of every algebra.
    pub fn zero() -> Self {
        Self::zero_at(0)
    }

    /// The zero of `Cl(n+1, 1)`.
    pub fn zero_at(n: u8) -> Self {
        Self { n, c: Vec::new() }
    }

    pub fn scalar(v: BigRational) -> Self {
        Self::of_terms(0, vec![(0, v)])
    }

    /// A single basis blade with coefficient `v`, in the smallest algebra
    /// holding its generators.
    pub fn blade(mask: usize, v: BigRational) -> Self {
        Self::of_terms(n_of_mask(mask), vec![(mask, v)])
    }

    /// The `n` this value belongs to: the smallest algebra holding it.
    pub fn n(&self) -> u8 {
        self.n
    }

    /// THE TERMS THE VALUE HOLDS: each blade mask and its non-zero
    /// coefficient, ascending by mask. A blade not here is zero.
    pub fn terms(&self) -> impl Iterator<Item = (usize, &BigRational)> {
        self.c.iter().map(|(m, v)| (*m, v))
    }

    /// How many blades the value holds.
    pub fn term_count(&self) -> usize {
        self.c.len()
    }

    /// THE RAY IN ITS CANONICAL COORDINATES: each blade the canonical value
    /// holds, its mask whole (a machine word, below 2⁶³ within [`MAX_N`]),
    /// and its exact coefficient. Empty for the zero, which has no ray.
    pub fn ray(&self) -> Vec<(u64, BigRational)> {
        match self.canonical() {
            Some(c) => c.c.into_iter().map(|(m, v)| (m as u64, v)).collect(),
            None => Vec::new(),
        }
    }

    /// The same value in the algebra at `m ≥ n`: the inclusion ℚⁿ ⊂ ℚᵐ. A
    /// value is never narrowed here.
    pub fn widened(&self, m: u8) -> Self {
        Self { n: self.n.max(m), c: self.c.clone() }
    }

    pub fn coeff(&self, mask: usize) -> &BigRational {
        match self.c.binary_search_by_key(&mask, |(m, _)| *m) {
            Ok(i) => &self.c[i].1,
            Err(_) => &ZERO,
        }
    }

    pub fn scalar_part(&self) -> BigRational {
        self.coeff(0).clone()
    }

    pub fn is_zero(&self) -> bool {
        self.c.is_empty()
    }

    /// A4: divide by |first nonzero|, negate so that coefficient is +1.
    pub fn canonical(&self) -> Option<Mv> {
        let (_, first) = self.c.first()?;
        let mut r = self.scale(&(one() / first.abs()));
        if r.c.first().is_some_and(|(_, v)| v.is_negative()) {
            r = r.scale(&-one());
        }
        Some(r)
    }

    /// THE CANONICAL WRITING (A4, 17.2): the coefficients after `canonical`.
    /// It CARRIES the value between minds; it identifies nothing — two
    /// derivations of one motion are one by the algebra (`projective_eq`,
    /// their coordinates), never by these bytes. The zero element has no
    /// `canonical` and is written as it is.
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
        let n = self.n.max(o.n);
        Self::of_terms(n, self.c.iter().chain(&o.c).cloned().collect())
    }

    pub fn sub(&self, o: &Self) -> Self {
        let n = self.n.max(o.n);
        let negated = o.c.iter().map(|(m, v)| (*m, -v.clone()));
        Self::of_terms(n, self.c.iter().cloned().chain(negated).collect())
    }

    pub fn scale(&self, s: &BigRational) -> Self {
        self.map_terms(|_, v| v * s)
    }

    /// THE GEOMETRIC PRODUCT — the whole algebra, for every `n`. `result[i⊕j]
    /// += sign · metric · a_i · b_j` over every pair of terms the two hold.
    pub fn geo(&self, o: &Self) -> Self {
        let n = self.n.max(o.n);
        let mut terms = Vec::with_capacity(self.c.len() * o.c.len());
        for (i, x) in &self.c {
            for (j, y) in &o.c {
                let term = x * y;
                let sign = reorder_sign(*i, *j) * metric_sign(i & j);
                terms.push((i ^ j, if sign >= 0 { term } else { -term }));
            }
        }
        Self::of_terms(n, terms)
    }

    /// The reverse `~M`: blade of grade g gets sign `(−1)^{g(g−1)/2}`.
    pub fn reverse(&self) -> Self {
        self.map_terms(|m, v| {
            let g = grade_of(m);
            match (g * g.saturating_sub(1) / 2) & 1 {
                0 => v.clone(),
                _ => -v.clone(),
            }
        })
    }

    /// The grade involution `M̂`: blade of grade g gets sign `(−1)^g`. On a
    /// vector it is the negation; on an even element the identity.
    pub fn involute(&self) -> Self {
        self.map_terms(|m, v| match grade_of(m) & 1 {
            0 => v.clone(),
            _ => -v.clone(),
        })
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
        for (m, _) in &self.c {
            let o = grade_of(*m) & 1 == 1;
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
        let c = self.c.iter().filter(|(m, _)| grade_of(*m) == g).cloned().collect();
        Self { n: self.n, c }
    }

    /// The scalar `⟨A B̃⟩₀`-style inner-square. For a grade-1 vector this is the
    /// metric norm `v·v`.
    pub fn norm2(&self) -> BigRational {
        self.geo(self).scalar_part()
    }

    // `versor_inverse` (`M̃ / (M M̃)`) and `quotient` (`B·A⁻¹`) stood here:
    // inversion as an operation (78.32(1), VA1-inverse). A versor acts
    // projectively and every reading of what it carries is by proportion,
    // so a site that composed with an inverse composes with the held
    // element and its free reverse, the quotient `B·A⁻¹` is the ray `B Ã`,
    // a site that asked whether an element inverts asks whether it is a
    // versor (`is_versor`), and a site whose quotient's SCALE is content
    // divides by [`super::group::weight_of`] and says so in its comment.

    /// The versor SANDWICH `V x Ṽ` — the exact action of a rigid motion on a
    /// whole volume in one step.
    pub fn sandwich(&self, x: &Self) -> Self {
        self.geo(x).geo(&self.reverse())
    }
}

impl Mv {
    /// The same value in the smallest algebra holding its terms: the inverse
    /// of [`Mv::widened`], exact. The zero narrows to `n = 0`.
    pub fn narrowed(&self) -> Self {
        let n = self.c.iter().map(|(m, _)| n_of_mask(*m)).max().map_or(0, |n| n);
        Self { n, c: self.c.clone() }
    }

    /// THE VALUE ON THE WIRE, ITS `n` FIRST (71.2(1), BQ1): the `n` of the
    /// smallest algebra holding it, one byte, then [`Mv::put_coefficients`]
    /// at that `n`. It carries the value; it never identifies it.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        let m = self.narrowed();
        out.push(m.n);
        m.put_coefficients(out);
    }

    /// Every mask's exact coefficient over the algebra at the value's `n`,
    /// zero where it holds no term: per coefficient `[sign u8][nlen]
    /// [numerator][dlen][denominator]`, each length LEB128 (ruled (a)),
    /// little-endian magnitudes, in mask order. At `n ≤ 3` that is
    /// thirty-two, the fixed carrier's coefficients exactly.
    ///
    /// The span is `slots(n)`, defined for every `n` a value can hold: its
    /// masks are made by [`base_axis`], which refuses past [`MAX_N`], by the
    /// generator constants, by products of masks already held, and by
    /// [`Mv::from_bytes`], which refuses an `n` past it.
    pub fn put_coefficients(&self, out: &mut Vec<u8>) {
        let span = slots(self.n).map_or(0, |s| s);
        let mut at = self.c.iter().peekable();
        for mask in 0..span {
            let v = match at.peek() {
                Some((m, v)) if *m == mask => {
                    at.next();
                    v
                }
                _ => &*ZERO,
            };
            let (ns, nb) = v.numer().to_bytes_le();
            let db = v.denom().to_bytes_le().1;
            out.push(match ns {
                num_bigint::Sign::Minus => 2,
                num_bigint::Sign::NoSign => 0,
                num_bigint::Sign::Plus => 1,
            });
            // Each length by its own coordinate (STD1 3c-2, ruled (a)).
            crate::transducer::address::put_key_bytes(out, &nb);
            crate::transducer::address::put_key_bytes(out, &db);
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
    /// coefficients, each read off the bytes before the next. The `n` is the
    /// writer's claim, never trusted to size anything: past [`MAX_N`] it is
    /// refused, and a span the remaining bytes cannot hold (every coefficient
    /// takes at least three) is refused before a coefficient is read. Only
    /// the non-zero coefficients are kept.
    pub fn coefficients_at(b: &[u8], pos: &mut usize, n: u8) -> Option<Mv> {
        let span = slots(n)?;
        if span > b.len().checked_sub(*pos)? / 3 {
            return None;
        }
        let mut c = Vec::new();
        for mask in 0..span {
            let sign = *b.get(*pos)?;
            *pos += 1;
            let nb = crate::transducer::address::take_key_bytes(b, pos)?;
            let db = crate::transducer::address::take_key_bytes(b, pos)?;
            let s = match sign {
                2 => num_bigint::Sign::Minus,
                1 => num_bigint::Sign::Plus,
                0 => num_bigint::Sign::NoSign,
                _ => return None,
            };
            let numer = BigInt::from_bytes_le(s, &nb);
            let denom = BigInt::from_bytes_le(num_bigint::Sign::Plus, &db);
            if denom.is_zero() {
                return None;
            }
            let v = BigRational::new(numer, denom);
            if !v.is_zero() {
                c.push((mask, v));
            }
        }
        Some(Mv { n, c })
    }
}
