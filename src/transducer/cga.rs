//! CONFORMAL GEOMETRIC ALGEBRA (DIM-5, the volumetric body) — the cure for
//! dimensional volumetric collapse. The perception must not hand the engine a
//! flat 0-chain (a scatter of per-cell point-diffs, un-factorable); it must hand
//! it VOLUMETRIC BLADES — objects as spheres with body — so the alphabet's
//! starved versors (rotors) have something to sandwich.
//!
//! This is the full exact multivector algebra `Cl(3,1)` over ℚ (no floats, no
//! roots — refuse, never round, as `iae::rotor` does one dimension down). The
//! orthonormal basis is `e1,e2,e+,e-` with squares `(+,+,+,-)`; the CONFORMAL
//! null basis is derived:
//!   e_o = ½(e- − e+)   (origin),   e_∞ = e- + e+   (infinity)
//!   e_o² = e_∞² = 0,   e_o·e_∞ = −1.
//! A base point embeds on the null cone `X(x) = x + ½‖x‖² e_∞ + e_o` (so
//! `X·X = 0`, exact — the base lives ON the cone, distances are bilinear one
//! level up). A SPHERE is the off-cone blade `S = X(c) − ½ r² e_∞`, and
//! `X(p)·S = 0` iff p lies on it. The point is the sphere's degenerate r=0 case.
//!
//! The whole reason for the ascent: a rigid motion is a single VERSOR SANDWICH
//! `V σ Ṽ` — a translation `T = 1 − ½ d e_∞` moves a whole volume (centroid,
//! boundary, interior) coherently in one algebraic step, where the 0-skeleton
//! needed `2·area` independent point deletions/insertions. And a null point is
//! NOT invertible (`X·X = 0`), so the geometric quotient `Q = B·A⁻¹` — the
//! demand between two scene states — only EXISTS once the object has volume
//! (`S·S = r² ≠ 0`). Volume is what makes the demand factorable.
//!
//! Blades are indexed by a 4-bit mask over `{e1,e2,e+,e-}`; the algebra is 16
//! coefficients of `BigRational`. The geometric product is the canonical
//! reordering-sign × metric construction. `iae::conformal` carries the same
//! embedding in coordinate form (proven in `g_conformal`); this module is its
//! multivector completion, so the sandwich and the quotient become first-class.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Zero};

/// Basis-vector masks over `{e1, e2, e+, e-}`.
pub const E1: usize = 0b0001;
pub const E2: usize = 0b0010;
pub const EP: usize = 0b0100; // e+  (square +1)
pub const EM: usize = 0b1000; // e-  (square −1)
/// The unit pseudoscalar `e1 e2 e+ e-`.
pub const PSEUDO: usize = 0b1111;

/// An exact multivector of `Cl(3,1)`: 16 blade coefficients over ℚ.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mv {
    c: [BigRational; 16],
}

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
