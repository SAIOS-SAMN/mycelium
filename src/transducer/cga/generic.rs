//! BQ3, 71.2(3), (4) — THE CARRIER OVER `Cl(n+1, 1)`: the multivector of the
//! conformal model of ℚⁿ, with `n` carried by the value and the product
//! written once for every `n`.
//!
//! THE GENERATORS KEEP THEIR BITS AS `n` GROWS. `e₊` is bit 3 and `e₋` is
//! bit 4 at every `n`; the first three base axes are bits 0, 1, 2, and a
//! fourth base axis onward takes bit 5 onward. So at `n ≤ 3` every mask is
//! the one the fixed carrier used, the canonical order (first non-zero
//! coefficient in mask order) is the same, and the wire writes the same
//! thirty-two coefficients; at `n = 4` the algebra has `2⁶` blades and the
//! first thirty-two are the `n = 3` algebra's own.
//!
//! THE VALUE CARRIES ITS `n`, AND TWO VALUES OF DIFFERENT `n` COMPOSE IN THE
//! LARGER. Because a generator never moves, a value of the smaller algebra
//! is the same value in the larger one: ℚⁿ ⊂ ℚᵐ with the extra coordinates
//! zero, which is an exact inclusion and not a reading of one as the other.
//! A scalar is `n = 0`, an element of every algebra.
//!
//! THE METRIC IS THE SIGNATURE'S, NOT A LIST: `e₋` squares to −1 and every
//! other generator to +1, which is `Cl(n+1, 1)` for every `n`.

use super::multivector::{EM, EP};
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

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

/// An exact multivector of `Cl(n+1, 1)`: its `n` and its coefficients over
/// ℚ, one per slot. Equality is projective, as the fixed carrier's.
#[derive(Debug, Clone)]
pub struct MvN {
    n: u8,
    c: Vec<BigRational>,
}

impl PartialEq for MvN {
    fn eq(&self, other: &Self) -> bool {
        self.projective_eq(other)
    }
}
impl Eq for MvN {}

fn half() -> BigRational {
    BigRational::new(BigInt::from(1), BigInt::from(2))
}

/// The parity of the transpositions that sort blade `a` before blade `b`.
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

/// The signature's sign on the generators two blades share: `e₋` squares to
/// −1 and every other generator to +1.
fn metric_sign(shared: usize) -> i64 {
    if (shared & EM) != 0 {
        -1
    } else {
        1
    }
}

impl MvN {
    /// The zero of `Cl(n+1, 1)`.
    pub fn zero(n: u8) -> Self {
        MvN { n, c: vec![BigRational::zero(); slots(n)] }
    }

    /// A scalar: `n = 0`, an element of every algebra.
    pub fn scalar(v: BigRational) -> Self {
        let mut m = MvN::zero(0);
        m.c[0] = v;
        m
    }

    /// One basis blade of `Cl(n+1, 1)` with coefficient `v`. `None` where the
    /// mask names a generator the algebra at `n` does not have.
    pub fn blade(n: u8, mask: usize, v: BigRational) -> Option<Self> {
        if mask & !pseudo(n) != 0 {
            return None;
        }
        let mut m = MvN::zero(n);
        *m.c.get_mut(mask)? = v;
        Some(m)
    }

    /// The same coefficients from a slice read in mask order, at `n`. `None`
    /// where the slice is longer than the algebra's slots or sets a slot the
    /// algebra has no blade for.
    pub fn from_coefficients(n: u8, cs: &[BigRational]) -> Option<Self> {
        if cs.len() > slots(n) {
            return None;
        }
        let mut m = MvN::zero(n);
        let p = pseudo(n);
        for (mask, v) in cs.iter().enumerate() {
            if !v.is_zero() && mask & !p != 0 {
                return None;
            }
            m.c[mask] = v.clone();
        }
        Some(m)
    }

    /// The `n` this value belongs to.
    pub fn n(&self) -> u8 {
        self.n
    }

    /// The same value in the algebra at `m ≥ n`: the inclusion ℚⁿ ⊂ ℚᵐ, the
    /// new slots zero. A value is never narrowed; asking for a smaller `m`
    /// returns it unchanged.
    pub fn widened(&self, m: u8) -> Self {
        if m <= self.n {
            return self.clone();
        }
        let mut c = self.c.clone();
        c.resize(slots(m), BigRational::zero());
        MvN { n: m, c }
    }

    /// The two operands in one algebra: the larger `n` of the two.
    fn common(&self, o: &Self) -> (Self, Self) {
        let n = self.n.max(o.n);
        (self.widened(n), o.widened(n))
    }

    pub fn coeff(&self, mask: usize) -> BigRational {
        match self.c.get(mask) {
            Some(v) => v.clone(),
            None => BigRational::zero(),
        }
    }

    pub fn scalar_part(&self) -> BigRational {
        self.c[0].clone()
    }

    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|x| x.is_zero())
    }

    /// A4: divide by |first non-zero|, and make that coefficient +1.
    pub fn canonical(&self) -> Option<Self> {
        let i = (0..self.c.len()).find(|&i| !self.c[i].is_zero())?;
        let mut r = self.scale(&(BigRational::one() / self.c[i].abs()));
        if r.c[i].is_negative() {
            r = r.scale(&-BigRational::one());
        }
        Some(r)
    }

    /// The canonical writing: the coefficients after `canonical`, the zero
    /// element as it is.
    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        match self.canonical() {
            Some(c) => c.to_bytes(out),
            None => self.to_bytes(out),
        }
    }

    /// `a = λ b` for `λ ∈ ℚ×` of either sign.
    pub fn projective_eq(&self, other: &Self) -> bool {
        projective_ratio(self, other).is_some_and(|l| !l.is_zero())
    }

    pub fn add(&self, o: &Self) -> Self {
        let (a, b) = self.common(o);
        MvN { n: a.n, c: a.c.iter().zip(&b.c).map(|(x, y)| x + y).collect() }
    }

    pub fn sub(&self, o: &Self) -> Self {
        let (a, b) = self.common(o);
        MvN { n: a.n, c: a.c.iter().zip(&b.c).map(|(x, y)| x - y).collect() }
    }

    pub fn scale(&self, s: &BigRational) -> Self {
        MvN { n: self.n, c: self.c.iter().map(|x| x * s).collect() }
    }

    /// THE GEOMETRIC PRODUCT, for every `n`: `result[i ⊕ j] += sign · metric ·
    /// a_i · b_j` over every pair of populated blades.
    pub fn geo(&self, o: &Self) -> Self {
        let (a, b) = self.common(o);
        let mut r = MvN::zero(a.n);
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

    /// The reverse: a blade of grade `g` takes the sign `(−1)^{g(g−1)/2}`.
    pub fn reverse(&self) -> Self {
        MvN {
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

    /// The grade-`g` part.
    pub fn grade(&self, g: u32) -> Self {
        MvN {
            n: self.n,
            c: self
                .c
                .iter()
                .enumerate()
                .map(|(i, v)| match (i as u32).count_ones() == g {
                    true => v.clone(),
                    false => BigRational::zero(),
                })
                .collect(),
        }
    }

    /// `A ∧ v` for a grade-`g` blade `A` and a vector `v`: the top grade of
    /// their product.
    pub fn wedge_vector(&self, g: u32, v: &Self) -> Self {
        self.geo(v).grade(g + 1)
    }

    /// `⟨M M⟩₀`, the metric norm of a vector.
    pub fn norm2(&self) -> BigRational {
        self.geo(self).scalar_part()
    }

    /// `M⁻¹ = M̃ / (M M̃)` where `M M̃` is a non-zero scalar; `None` otherwise.
    pub fn versor_inverse(&self) -> Option<Self> {
        let rev = self.reverse();
        let prod = self.geo(&rev);
        let s = prod.scalar_part();
        if s.is_zero() || !prod.sub(&MvN::scalar(s.clone())).is_zero() {
            return None;
        }
        Some(rev.scale(&(BigRational::one() / s)))
    }

    pub fn quotient(b: &Self, a: &Self) -> Option<Self> {
        Some(b.geo(&a.versor_inverse()?))
    }

    pub fn sandwich(&self, x: &Self) -> Self {
        self.geo(x).geo(&self.reverse())
    }

    /// THE CLASS INVARIANTS OF A TRANSITION, read off its grade-2 generator
    /// exactly as the fixed carrier reads them.
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

    pub fn holonomy_key(&self) -> Option<crate::network::nerve::coord::Holonomy> {
        let (i1, i2, kind) = self.class_invariants()?;
        Some(crate::network::nerve::coord::Holonomy { i1, i2, kind })
    }

    pub fn class_key(&self) -> Option<crate::transducer::address::Address> {
        use crate::network::nerve::Coordinate;
        let h = self.holonomy_key()?;
        Some(crate::transducer::address::Address::from_bytes(h.encode().as_bytes()))
    }

    /// Canonical coefficients written out behind the versor tag.
    pub fn key(&self) -> Option<crate::transducer::address::Address> {
        let c = self.canonical()?;
        let mut buf = vec![crate::transducer::address::TAG_VERSOR];
        c.to_bytes(&mut buf);
        Some(crate::transducer::address::Address::new(buf))
    }

    /// The key read back at `n`, which the reader knows from the address.
    pub fn from_key(k: &[u8], n: u8) -> Option<Self> {
        if crate::transducer::address::tag_of(k) != Some(crate::transducer::address::TAG_VERSOR) {
            return None;
        }
        let mut p = 1usize;
        MvN::from_bytes(k, &mut p, n)
    }

    /// Every slot's coefficient, `[sign][nlen][numerator][dlen][denominator]`,
    /// in mask order: the fixed carrier's thirty-two at `n ≤ 3`.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
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

    /// The inverse of [`MvN::to_bytes`] at `n`: `slots(n)` coefficients.
    pub fn from_bytes(b: &[u8], pos: &mut usize, n: u8) -> Option<Self> {
        let mut m = MvN::zero(n);
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
            let denom = BigInt::from_bytes_le(num_bigint::Sign::Plus, db);
            if denom.is_zero() {
                return None;
            }
            *slot = BigRational::new(BigInt::from_bytes_le(s, nb), denom);
        }
        Some(m)
    }

    /// `½`, for the constructors written on this carrier.
    pub fn half() -> BigRational {
        half()
    }
}

/// Is `m` a versor: `m m̃` a non-zero scalar? Returns that scalar.
pub fn is_versor(m: &MvN) -> Option<BigRational> {
    let prod = m.geo(&m.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&MvN::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s)
}

/// Projective equality `a = λ b`, returning `λ`: through `a b̃` where `b` is
/// not null, and by exact componentwise proportion where it is.
pub fn projective_ratio(a: &MvN, b: &MvN) -> Option<BigRational> {
    let bb = b.norm2();
    if bb.is_zero() {
        return proportional(a, b);
    }
    let prod = a.geo(&b.reverse());
    let s = prod.scalar_part();
    if s.is_zero() || !prod.sub(&MvN::scalar(s.clone())).is_zero() {
        return None;
    }
    Some(s / bb)
}

fn proportional(a: &MvN, b: &MvN) -> Option<BigRational> {
    let (a, b) = a.common(b);
    let mut lambda: Option<BigRational> = None;
    for (x, y) in a.c.iter().zip(&b.c) {
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

/// The bivector planes of `Cl(n+1, 1)`, GENERATED: the base planes
/// `eᵢ∧eⱼ`, then `eᵢ∧e₊`, then the luminal `eᵢ∧e₋`, then `e₊∧e₋`.
/// `C(n+1, 2) + (n+1)` of them; at `n = 3` the ten in the fixed order.
pub fn bivector_planes(n: u8) -> Vec<usize> {
    let n = usize::from(n);
    let mut out = Vec::new();
    for i in 1..=n {
        for j in (i + 1)..=n {
            out.push(base_axis(i) | base_axis(j));
        }
    }
    for i in 1..=n {
        out.push(base_axis(i) | EP);
    }
    for i in 1..=n {
        out.push(base_axis(i) | EM);
    }
    out.push(EP | EM);
    out
}
