//! A4 — CONFORMAL GEOMETRIC ALGEBRA: the exact multivector algebra `Cl(3,1)`
//! over ℚ, where a body is an invertible dual sphere and a rigid motion is one
//! versor sandwich, so the quotient between two scene states exists.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Signed, Zero};

/// Basis-vector masks over `{e1, e2, e+, e-}`.
pub const E1: usize = 0b0001;
pub const E2: usize = 0b0010;
pub const EP: usize = 0b0100; // e+  (square +1)
pub const EM: usize = 0b1000; // e-  (square −1)
/// The unit pseudoscalar `e1 e2 e+ e-`.
pub const PSEUDO: usize = 0b1111;

/// An exact multivector of `Cl(3,1)`: 16 blade coefficients over ℚ.
/// Equality is projective (`λ ∈ ℚ×`); coefficient identity is [`Mv::canonical`].
#[derive(Debug, Clone)]
pub struct Mv {
    c: [BigRational; 16],
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
/// squares to −1; `e1,e2,e+` square to +1. (No null generators in the
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
        m.c[mask & 0xF] = v;
        m
    }

    pub fn coeff(&self, mask: usize) -> &BigRational {
        &self.c[mask & 0xF]
    }

    pub fn scalar_part(&self) -> BigRational {
        self.c[0].clone()
    }

    pub fn is_zero(&self) -> bool {
        self.c.iter().all(|x| x.is_zero())
    }

    /// A4: divide by |first nonzero|, negate so that coefficient is +1.
    pub fn canonical(&self) -> Option<Mv> {
        let i = (0..16).find(|&i| !self.c[i].is_zero())?;
        let mut r = self.scale(&(one() / self.c[i].abs()));
        if r.c[i].is_negative() {
            r = r.scale(&-one());
        }
        Some(r)
    }

    /// THE CANONICAL WRITING (A4, 17.2): the coefficients after `canonical`,
    /// so a versor and every `λ·versor` write the same bytes and a key built
    /// from them is content rather than one chosen spelling of it.
    ///
    /// Every key and address in the tree that carried a versor wrote its
    /// raw coefficients — `class_key`, `vertex_address`, `form_key_bytes` —
    /// so a carriage and its negation, one projective element, were two
    /// keys. `same_vertex` named this as 17.2's open item and every
    /// comparison downstream of those keys inherited it. This is the one
    /// writer that closes it.
    ///
    /// The zero element has no `canonical` and is not a versor; its sixteen
    /// zeros are already its only writing, so it is written as it is — a
    /// fact about zero, not a default standing in for a canonical form.
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
        for i in 0..16 {
            if self.c[i].is_zero() {
                continue;
            }
            for j in 0..16 {
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

/// Embed a base point on the null cone: `X(x) = x + ½‖x‖² e_∞ + e_o`.
pub fn point(x1: &BigRational, x2: &BigRational) -> Mv {
    let x2n = x1 * x1 + x2 * x2;
    let mut m = Mv::blade(E1, x1.clone()).add(&Mv::blade(E2, x2.clone()));
    m = m.add(&e_inf().scale(&(half() * x2n)));
    m.add(&e_o())
}

/// The dual sphere `S = X(c) − ½ r² e_∞`: a point at squared-distance `r2` from
/// `c`. `r2 = 0` recovers the point (a volumeless sphere). Off-cone iff r²≠0.
pub fn sphere(c1: &BigRational, c2: &BigRational, r2: &BigRational) -> Mv {
    point(c1, c2).sub(&e_inf().scale(&(half() * r2)))
}

/// The translation versor `T(d) = 1 − ½ d e_∞`, `d = d1 e1 + d2 e2`. Acts by
/// sandwich: `T X(x) T̃ = X(x + d)`, up to the homogeneous scale.
pub fn translator(d1: &BigRational, d2: &BigRational) -> Mv {
    let d = Mv::blade(E1, d1.clone()).add(&Mv::blade(E2, d2.clone()));
    let d_einf = d.geo(&e_inf());
    Mv::scalar(one()).sub(&d_einf.scale(&half()))
}

/// The Euclidean rotation versor in the `e1e2` plane as an EXACT even
/// multivector `s + b·e1e2` (unnormalized, as `iae::rotor`: the sandwich carries
/// the scale `s²+b²`, absorbed by homogeneity on the cone). The quarter turn is
/// `s=0, b=1` — the hole a rational chart cannot reach, an ordinary citizen here.
pub fn rotator(s: &BigRational, b: &BigRational) -> Mv {
    Mv::scalar(s.clone()).add(&Mv::blade(E1 | E2, b.clone()))
}

/// Read the base point `(x1, x2)` back out of a homogeneous conformal point or
/// sphere: the scale is `α = −(P·e_∞)`, then `x = (P_e1, P_e2)/α`. `None` at
/// infinity (`α = 0`).
pub fn base(p: &Mv) -> Option<(BigRational, BigRational)> {
    let alpha = -p.geo(&e_inf()).scalar_part();
    if alpha.is_zero() {
        return None;
    }
    Some((p.coeff(E1) / &alpha, p.coeff(E2) / &alpha))
}

/// Does base point `p` lie on dual sphere `S`? `X(p)·S = 0`, exact.
pub fn on_sphere(p: &Mv, s: &Mv) -> bool {
    p.geo(s).scalar_part().is_zero()
}

/// THE BOUNDARY PROJECTION `∂σ = σ ⌟ I⁻¹` — the dual contraction against the
/// pseudoscalar, the bridge from the volumetric body to its cellular
/// boundary/2-chain (which then enters the Mayer–Vietoris / L2 coboundary
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
/// settled against the embedding itself (the form that carries X(1,0) to
/// X(ρ,0)), never by a chosen sign.
pub fn dilator(rho: &BigRational) -> Mv {
    let e = e_plane();
    let plus = Mv::scalar(rho + one()).add(&e.scale(&(rho - one())));
    let target = point(rho, &zero());
    let probe = plus.sandwich(&point(&one(), &zero()));
    match projective_ratio(&probe, &target) {
        Some(_) => plus,
        None => Mv::scalar(rho + one()).sub(&e.scale(&(rho - one()))),
    }
}

/// THE REFLECTOR across the e2 axis (rows negated): the odd versor `e1`.
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
    for mask in 0..16 {
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
    /// Serialize the 16 exact coefficients: per coefficient `[sign u8][nlen
    /// u32][numerator][dlen u32][denominator]`, little-endian magnitudes.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        for mask in 0..16 {
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
        for mask in 0..16 {
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
        let moved = d.sandwich(&point(&q(2, 1), &q(-4, 1)));
        assert_eq!(base(&moved), Some((q(3, 1), q(-6, 1))), "X(2,−4) → X(3,−6) at ρ = 3/2");
        let r = reflector();
        let flipped = r.sandwich(&point(&q(2, 1), &q(5, 1)));
        assert_eq!(base(&flipped), Some((q(-2, 1), q(5, 1))));
        let s = sphere(&q(1, 1), &q(1, 1), &q(5, 1));
        assert_eq!(projective_ratio(&s.scale(&q(7, 3)), &s), Some(q(7, 3)));
        assert!(projective_ratio(&s, &sphere(&q(1, 1), &q(1, 1), &q(6, 1))).is_none());
        let composed = translator(&q(1, 1), &q(0, 1)).geo(&d);
        let mut bytes = Vec::new();
        composed.to_bytes(&mut bytes);
        let mut pos = 0usize;
        assert_eq!(Mv::from_bytes(&bytes, &mut pos), Some(composed));
    }

    #[test]
    fn a4_projective_eq_either_sign_and_key() {
        let t = translator(&q(2, 1), &q(0, 1));
        assert!(t.projective_eq(&t.scale(&q(-3, 2))));
        assert_eq!(t.canonical().unwrap().key(), t.scale(&q(-3, 2)).canonical().unwrap().key());
        assert_eq!(t, t.scale(&q(5, 1)), "PartialEq is projective");
        let u = translator(&q(3, 1), &q(0, 1));
        assert_ne!(t.key(), u.key());
    }
}
