// 75, 78.11, MD1 — THE MODES OF A GENERATOR: a bivector split into its
// commuting 2-blades, `m` of them read off the bivector, each with its plane
// and its class, exact over ℚ or refused by name.
//
//! A generator `B ∈ ⋀²ℚ^{n+1,1}` acts on vectors by `F(x) = ⟨x B⟩₁`, a map
//! skew to the metric, and `F²` is self-adjoint. Its SCALAR INVARIANTS AT
//! EVERY EXPONENT — the power sums `tr((F²)ᵏ)` for `k` up to the dimension,
//! gathered into `F²`'s characteristic polynomial by Faddeev–LeVerrier — are
//! what the two-part split read only at the square (`⟨B²⟩₀`, `⟨B²⟩₄`). Every
//! non-zero root `λ` of that polynomial is one mode's value, its eigenspace
//! the mode's plane (or planes, where modes share a value: an isoclinic
//! pair, held whole), and the generalised kernel carries at most one
//! parabolic mode — `Cl(n+1, 1)` has one timelike direction, so at most one
//! mode is not elliptic.
//!
//! EACH MODE IS `B` PROJECTED ONTO ITS OWN PLANES: the eigenspaces are
//! mutually orthogonal, so the outermorphism of the projector onto one
//! carries `B` to exactly its part there, with no cross term to drop. No
//! angle, no root of a mode's value, no approximation.
//!
//! EXACT OR REFUSED (rule 4, rule 11). A factor of the characteristic
//! polynomial with no rational root leaves the split undone over ℚ, and it is
//! refused by name with the factor's degree — never rounded onto a nearby
//! split. Every rational root is read by the rational root theorem at any
//! degree (MD1b), so three planes at `n = 4` read three values; only what is
//! left without one is refused.
//!
//! A MOTION's modes are read off its own action by [`super::motion`] (MR1)
//! with the same machinery; this file reads a GENERATOR's.

use super::multivector::{one, Mv};
use crate::network::nerve::coord::BivectorKind;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// ONE MODE OF A GENERATOR: the part of `B` on its own planes, its class by
/// the sign of its square, and how many planes it spans — two or more where
/// modes share a value, an isoclinic set held whole (no split among them is
/// chosen, checklist (r)).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mode {
    pub blade: Mv,
    pub kind: BivectorKind,
    pub planes: usize,
}

/// WHY A GENERATOR DID NOT SPLIT OVER ℚ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Undecomposed {
    /// The characteristic polynomial's non-zero part has a factor of this
    /// degree with no rational root: the modes' values lie in an extension.
    Irrational(usize),
    /// The eigenspaces do not span the base: a non-diagonalisable part other
    /// than the one parabolic mode the signature allows.
    Defective,
    /// BD1 — the stroke's budget was spent reading the rational roots of a
    /// factor of this degree (rule 10): refused by name, never run on.
    Unbounded(usize),
}

/// The generators of the algebra `B` lives in, as masks in ascending bit
/// order — the base axes, `e₊` and `e₋`, `n + 2` of them — so a product of
/// two in this order is the blade the carrier writes.
pub(super) fn generators(n: u8) -> Vec<usize> {
    let mut masks: Vec<usize> = (1..=usize::from(n)).map(super::base_axis).collect();
    masks.push(super::EP);
    masks.push(super::EM);
    masks.sort_unstable();
    masks
}

/// The polynomial with its vanishing leading coefficients dropped: its
/// degree, read off the coefficients themselves.
fn trim(mut p: Vec<Q>) -> Vec<Q> {
    while let [_, .., lead] = p.as_slice() {
        if !lead.is_zero() {
            break;
        }
        p.pop();
    }
    p
}

/// `p mod q` over ℚ, `q` non-zero.
fn rem(p: &[Q], q: &[Q]) -> Vec<Q> {
    let mut r = trim(p.to_vec());
    let q = trim(q.to_vec());
    let lead = q[q.len() - 1].clone();
    while r.len() >= q.len() && !(r.len() == 1 && r[0].is_zero()) {
        let f = &r[r.len() - 1] / &lead;
        let shift = r.len() - q.len();
        for (i, c) in q.iter().enumerate() {
            r[shift + i] = &r[shift + i] - &f * c;
        }
        r = trim(r);
        if r.len() < q.len() {
            break;
        }
        if r.len() == q.len() && r[r.len() - 1].is_zero() {
            break;
        }
    }
    r
}

fn gcd(a: &[Q], b: &[Q]) -> Vec<Q> {
    let (mut a, mut b) = (trim(a.to_vec()), trim(b.to_vec()));
    while !(b.len() == 1 && b[0].is_zero()) {
        let r = rem(&a, &b);
        a = b;
        b = r;
    }
    a
}

/// `p / q` exactly, `q` dividing `p`.
fn quotient(p: &[Q], q: &[Q]) -> Vec<Q> {
    let mut r = trim(p.to_vec());
    let q = trim(q.to_vec());
    let lead = q[q.len() - 1].clone();
    let Some(span) = r.len().checked_sub(q.len()) else { return vec![Q::zero()] };
    let mut out = vec![Q::zero(); span + 1];
    while r.len() >= q.len() && !(r.len() == 1 && r[0].is_zero()) {
        let f = &r[r.len() - 1] / &lead;
        let shift = r.len() - q.len();
        out[shift] = f.clone();
        for (i, c) in q.iter().enumerate() {
            r[shift + i] = &r[shift + i] - &f * c;
        }
        r.pop();
        r = trim(r);
    }
    trim(out)
}

/// The positive divisors of `n ≠ 0`, from its prime factors by trial
/// division — the finite set the rational root theorem names. Each trial
/// spends one read of the stroke's budget (BD1, rule 10); `None` where the
/// budget is spent before the factorisation ends.
/// 78.100 — THE ROOT READER'S REFUSALS FOR THE STROKE, in this process: how
/// many, the highest degree refused, and the widest coefficient (in bits)
/// whose divisors were being walked when the stroke ran out. Trial division
/// walks up to `√n`, so a coefficient of `b` bits can ask `2^(b/2)` steps.
static ROOTS_REFUSED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static ROOTS_DEGREE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static ROOTS_BITS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn note_root_refusal(degree: usize, bits: u64) {
    use std::sync::atomic::Ordering::Relaxed;
    let _ = ROOTS_REFUSED.fetch_update(Relaxed, Relaxed, |n| n.checked_add(1));
    ROOTS_DEGREE.fetch_max(u64::try_from(degree).map_or(u64::MAX, |d| d), Relaxed);
    ROOTS_BITS.fetch_max(bits, Relaxed);
}

/// The root reader's refusals: `(refused, highest degree, widest bits)`.
pub fn root_refusals() -> (u64, u64, u64) {
    use std::sync::atomic::Ordering::Relaxed;
    (ROOTS_REFUSED.load(Relaxed), ROOTS_DEGREE.load(Relaxed), ROOTS_BITS.load(Relaxed))
}

fn divisors(n: &num_bigint::BigInt, budget: &mut crate::physics::Budget) -> Option<Vec<num_bigint::BigInt>> {
    use num_bigint::BigInt;
    let mut rest = n.abs();
    let mut out = vec![BigInt::from(1)];
    let mut p = BigInt::from(2);
    while &p * &p <= rest {
        if !budget.spend() {
            return None;
        }
        let mut power = 0usize;
        while (&rest % &p).is_zero() {
            rest /= &p;
            power += 1;
        }
        let base = out.clone();
        let mut factor = BigInt::from(1);
        for _ in 0..power {
            factor *= &p;
            out.extend(base.iter().map(|d| d * &factor));
        }
        p += 1;
    }
    if rest > BigInt::from(1) {
        let base = out.clone();
        out.extend(base.iter().map(|d| d * &rest));
    }
    Some(out)
}

/// 78.100, 78.63 (the wrong math) — THE INTEGER ROOTS `y` OF THE MONIC
/// SUBSTITUTE of an integer polynomial `a₀ + … + a_m xᵐ` of degree one to
/// three, every rational root of which is `y / a_m`.
///
/// `y = a_m·x` turns `a_m^{m−1}·f(y / a_m)` into `yᵐ + Σ aᵢ a_m^{m−1−i} yⁱ`,
/// monic with integer coefficients, so its rational roots are integers (the
/// rational root theorem, its leading coefficient one). They are found by
/// EXACT BISECTION, never by factoring an integer: the derivative splits the
/// line into runs on which the polynomial is monotone — its critical points
/// bracketed between integers by an exact integer square root — and each
/// run is bisected over the integers by the signs of exact evaluations,
/// about as many steps as the coefficients have bits. Trial division asked
/// up to `√n` divisions: on epoch 125 every refusal was a quadratic or a
/// cubic with coefficients of up to 84 bits, some `2⁴²` steps each.
///
/// `None` above degree three (the divisors' search stands there); `Some(Err)`
/// where the stroke is spent, one spend per evaluation (rule 10).
fn monic_integer_roots(
    ints: &[num_bigint::BigInt],
    budget: &mut crate::physics::Budget,
) -> Option<Result<Vec<num_bigint::BigInt>, ()>> {
    use num_bigint::BigInt;
    let m = ints.len().checked_sub(1)?;
    if !(1..=3).contains(&m) {
        return None;
    }
    let am = &ints[m];
    // The monic substitute, ascending: g_i = a_i · a_m^{m−1−i}, g_m = 1.
    let g: Vec<BigInt> = (0..m).map(|i| &ints[i] * num_traits::pow(am.clone(), m - 1 - i)).chain([BigInt::from(1)]).collect();
    let at = |y: &BigInt| g.iter().rev().fold(BigInt::from(0), |acc, c| acc * y + c);
    let sign = |v: BigInt| v.sign();
    // Every real root lies within the Cauchy bound `1 + max |g_i|`.
    // The largest magnitude among the lower coefficients, folded: a reading,
    // never a pick by position.
    let bound: BigInt = g[..m].iter().fold(BigInt::from(0), |most: BigInt, c| most.max(c.abs())) + BigInt::from(1);
    // THE MONOTONE RUNS, as integer intervals, and the few integers between
    // a critical point's floor and ceiling read on their own.
    let (mut runs, mut singles): (Vec<(BigInt, BigInt)>, Vec<BigInt>) = (Vec::new(), Vec::new());
    match m {
        1 => runs.push((-bound.clone(), bound.clone())),
        2 => {
            // g' = 2y + g₁, its root −g₁/2: floor and ceiling.
            let num = -&g[1];
            let (lo, hi) = (num_integer::Integer::div_floor(&num, &BigInt::from(2)), num_integer::Integer::div_ceil(&num, &BigInt::from(2)));
            runs.push((-bound.clone(), lo.clone()));
            runs.push((hi.clone(), bound.clone()));
        }
        _ => {
            // g' = 3y² + 2g₂y + g₁; its roots (−2g₂ ± √D)/6, D = 4g₂² − 12g₁.
            let d = BigInt::from(4) * &g[2] * &g[2] - BigInt::from(12) * &g[1];
            if d.sign() != num_bigint::Sign::Plus {
                runs.push((-bound.clone(), bound.clone()));
            } else {
                // √D ∈ [s, s + 1): each critical point bracketed by integers.
                let s = d.sqrt();
                let six = BigInt::from(6);
                let floor = |n: BigInt| num_integer::Integer::div_floor(&n, &six);
                let ceil = |n: BigInt| num_integer::Integer::div_ceil(&n, &six);
                let base = -BigInt::from(2) * &g[2];
                let unit = BigInt::from(1);
                let (k1, k2): (BigInt, BigInt) = (floor(&base - &s - &unit), ceil(&base - &s));
                let (k3, k4): (BigInt, BigInt) = (floor(&base + &s), ceil(&base + &s + &unit));
                runs.push((-bound.clone(), k1.clone()));
                runs.push((k2.clone(), k3.clone()));
                runs.push((k4.clone(), bound.clone()));
                // Between a critical point's floor and ceiling: at most one
                // integer each side (the bracket is a sixth wide).
                let mut y: BigInt = &k1 + &unit;
                while y < k2 {
                    singles.push(y.clone());
                    y += &unit;
                }
                let mut y: BigInt = &k3 + &unit;
                while y < k4 {
                    singles.push(y.clone());
                    y += &unit;
                }
            }
        }
    }
    let mut roots: Vec<BigInt> = Vec::new();
    for y in singles {
        if !budget.spend() {
            return Some(Err(()));
        }
        if at(&y).sign() == num_bigint::Sign::NoSign && !roots.contains(&y) {
            roots.push(y);
        }
    }
    for (mut lo, mut hi) in runs {
        if lo > hi {
            continue;
        }
        if !(budget.spend() && budget.spend()) {
            return Some(Err(()));
        }
        let (slo, shi) = (sign(at(&lo)), sign(at(&hi)));
        if slo == num_bigint::Sign::NoSign {
            if !roots.contains(&lo) {
                roots.push(lo.clone());
            }
            continue;
        }
        if shi == num_bigint::Sign::NoSign {
            if !roots.contains(&hi) {
                roots.push(hi.clone());
            }
            continue;
        }
        if slo == shi {
            continue;
        }
        // One sign change on a monotone run: one real root between, read to
        // an integer or to an interval between two consecutive integers.
        while &hi - &lo > BigInt::from(1) {
            if !budget.spend() {
                return Some(Err(()));
            }
            let mid: BigInt = (&lo + &hi) >> 1;
            let smid = sign(at(&mid));
            if smid == num_bigint::Sign::NoSign {
                if !roots.contains(&mid) {
                    roots.push(mid.clone());
                }
                break;
            }
            match smid == slo {
                true => lo = mid,
                false => hi = mid,
            }
        }
    }
    Some(Ok(roots))
}

/// THE DISTINCT RATIONAL ROOTS of `p`, zero among them, exact — or the
/// degree of what remains when none of its roots is rational (MD1b, 78.12(1)).
///
/// `p` is taken square-free (`p / gcd(p, p′)`), so each value is read once.
/// Its rational roots are read by THE RATIONAL ROOT THEOREM: with the
/// denominators cleared to integer coefficients `a₀ … a_m`, every rational
/// root `r/s` in lowest terms has `r | a₀` and `s | a_m`, a finite set named
/// by the theorem, each candidate tested exactly and divided out. What is
/// left has no rational root: its values lie in an extension of ℚ, and it
/// is refused by its degree — never rounded onto a nearby split. The
/// factorisation spends the stroke's budget, and a spent budget refuses the
/// factor as `Unbounded` by its degree (BD1).
pub(super) fn rational_roots(p: &[Q], budget: &mut crate::physics::Budget) -> Result<Vec<Q>, Undecomposed> {
    use num_bigint::BigInt;
    let p = trim(p.to_vec());
    let derivative: Vec<Q> =
        p.iter().enumerate().skip(1).map(|(i, c)| c * Q::from_integer(BigInt::from(i))).collect();
    let free = quotient(&p, &gcd(&p, &derivative));
    let mut roots: Vec<Q> = Vec::new();
    // The root at zero divided out: the leading zero coefficients dropped.
    let mut rest: Vec<Q> = free.iter().skip_while(|c| c.is_zero()).cloned().collect();
    if rest.len() < free.len() {
        roots.push(Q::zero());
    }
    // Denominators cleared: every coefficient times their lcm is integral.
    let lcm = rest.iter().fold(BigInt::from(1), |l, c| num_integer::Integer::lcm(&l, c.denom()));
    let cleared: Vec<BigInt> = rest.iter().map(|c| (c * Q::from_integer(lcm.clone())).to_integer()).collect();
    // And their common content divided out — the same roots, the smallest
    // integers that carry them (a square-free quotient leaves a content the
    // monic substitute would raise to the power of the degree).
    let content = cleared.iter().fold(BigInt::from(0), |g, c| num_integer::Integer::gcd(&g, c));
    let ints: Vec<BigInt> = match content.sign() {
        num_bigint::Sign::NoSign => cleared,
        _ => cleared.iter().map(|c| c / &content).collect(),
    };
    if let ([a0, ..], [.., am]) = (ints.as_slice(), ints.as_slice()) {
        if ints.len() > 1 {
            let unbounded = Undecomposed::Unbounded(ints.len() - 1);
            // 78.100 — A REFUSAL FOR THE STROKE IS READ: its degree and the
            // width of the coefficients its search was walking.
            let refuse = || {
                note_root_refusal(ints.len() - 1, a0.bits().max(am.bits()));
                unbounded
            };
            // THE CANDIDATES, exact either way. Up to degree three (every
            // polynomial epoch 125 met) by the monic substitution and exact
            // bisection — no integer factored; above it by the divisors of
            // `a₀` and `a_m`, the rational root theorem's own set.
            let candidates: Vec<Q> = match monic_integer_roots(&ints, budget) {
                Some(Ok(ys)) => ys.into_iter().map(|y| Q::new(y, am.clone())).collect(),
                Some(Err(())) => return Err(refuse()),
                None => {
                    let (tops, bottoms) = (divisors(a0, budget).ok_or_else(refuse)?, divisors(am, budget).ok_or_else(refuse)?);
                    tops.iter()
                        .flat_map(|r| bottoms.iter().flat_map(move |s| [Q::new(r.clone(), s.clone()), -Q::new(r.clone(), s.clone())]))
                        .collect()
                }
            };
            for candidate in candidates {
                let value = rest.iter().rev().fold(Q::zero(), |acc, c| acc * &candidate + c);
                if value.is_zero() && !roots.contains(&candidate) {
                    rest = quotient(&rest, &[-candidate.clone(), one()]);
                    roots.push(candidate);
                }
            }
        }
    }
    match rest.len() {
        0 | 1 => {
            roots.sort();
            Ok(roots)
        }
        more => Err(Undecomposed::Irrational(more - 1)),
    }
}

/// The distinct non-zero rational roots of `p`: a generator's mode values.
fn nonzero_roots(p: &[Q], budget: &mut crate::physics::Budget) -> Result<Vec<Q>, Undecomposed> {
    Ok(rational_roots(p, budget)?.into_iter().filter(|r| !r.is_zero()).collect())
}

/// `B` carried by the outermorphism of `P`: every `e_k ∧ e_l` to
/// `P e_k ∧ P e_l`.
fn project(b: &Mv, p: &super::LinearFunction) -> Mv {
    let masks = p.masks();
    let images = p.images();
    let mut out = Mv::zero();
    for k in 0..masks.len() {
        for l in (k + 1)..masks.len() {
            let c = b.coeff(masks[k] | masks[l]);
            if c.is_zero() {
                continue;
            }
            out = out.add(&images[k].geo(&images[l]).grade(2).scale(c));
        }
    }
    out
}

/// THE SPECTRAL PROJECTORS OF A FUNCTION, AS POLYNOMIALS IN IT: one per
/// value in `values` onto that value's space along every other, and the rest
/// — the generalised space at `root` where one is named — as the identity
/// less them. For a value `λⱼ` the projector is
///
/// ```text
///     Pⱼ(x) = (x − r)^d Π_{i≠j} (x − λᵢ)  /  ((λⱼ − r)^d Π_{i≠j} (λⱼ − λᵢ))
/// ```
///
/// with `r` the root whose generalised space carries a nilpotent part (its
/// factor raised to the dimension `d` kills it) — `1` at every `λⱼ`, `0` at
/// every other value and on the root's whole space. Exact over ℚ, no change of
/// basis and no inverse. Each value's space must be the function's own there
/// (checked by the caller), which is what makes these the projectors.
pub(super) fn spectral_projectors(
    f: &super::LinearFunction,
    values: &[Q],
    root: Option<&Q>,
) -> Vec<super::LinearFunction> {
    let d = f.masks().len();
    let linear = |c: &Q| vec![-c.clone(), one()];
    let times = |p: &[Q], q: &[Q]| -> Vec<Q> {
        let mut out = vec![Q::zero(); p.len() + q.len() - 1];
        for (i, a) in p.iter().enumerate() {
            for (j, b) in q.iter().enumerate() {
                out[i + j] += a * b;
            }
        }
        out
    };
    values
        .iter()
        .enumerate()
        .map(|(j, lj)| {
            let mut poly = vec![one()];
            let mut norm = one();
            if let Some(r) = root {
                for _ in 0..d {
                    poly = times(&poly, &linear(r));
                    norm *= lj - r;
                }
            }
            for (i, li) in values.iter().enumerate() {
                if i != j {
                    poly = times(&poly, &linear(li));
                    norm *= lj - li;
                }
            }
            let poly: Vec<Q> = poly.iter().map(|c| c / &norm).collect();
            f.polynomial(&poly)
        })
        .collect()
}

pub(super) fn kind_of(blade: &Mv) -> BivectorKind {
    let s = blade.geo(blade).scalar_part();
    match (blade.is_zero(), s.is_negative(), s.is_positive()) {
        (true, _, _) => BivectorKind::Trivial,
        (false, true, _) => BivectorKind::Elliptic,
        (false, _, true) => BivectorKind::Hyperbolic,
        _ => BivectorKind::Parabolic,
    }
}

/// A MODE'S OWN CLOSURE UNDER A MOTION (75, 78.11 link 3): the least order
/// `ℓ` of the algebra's cyclotomic set at which `motion^ℓ` fixes every
/// vector of the mode's plane — the plane read as the image of the mode's
/// own action, `⟨e_k B_j⟩₁`. Cayley is not multiplicative over commuting
/// parts, so a mode's closure is read on the motion's own action and never
/// off `cayley(B_j)`. `None` for a parabolic or hyperbolic mode, open by
/// name, and for an elliptic mode that closes at no order the algebra
/// admits: a rosette.
pub fn mode_order(motion: &Mv, mode: &Mode) -> Option<u32> {
    if mode.kind != BivectorKind::Elliptic {
        return None;
    }
    let n = motion.n().max(mode.blade.n());
    let plane: Vec<Mv> = generators(n)
        .into_iter()
        .map(|m| Mv::blade(m, one()).geo(&mode.blade).grade(1))
        .filter(|v| !v.is_zero())
        .collect();
    order_on(motion, &plane)
}

/// The least order of the algebra's cyclotomic set at which `motion^ℓ` fixes
/// every vector of `plane`, read on the motion's own action; `None` where no
/// admitted order closes it, or the motion does not invert.
pub(super) fn order_on(motion: &Mv, plane: &[Mv]) -> Option<u32> {
    let n = plane.iter().fold(motion.n(), |n, v| n.max(v.n()));
    // `Ωᵏ x Ω̃ᵏ = wᵏ · Ωᵏ x Ω⁻ᵏ`: the reverse carries the action, and the
    // weight's power is the eigenvalue a closing power holds each vector at
    // (78.28(2)) — never a division.
    let w = super::weight_of(motion)?;
    let inverse = motion.reverse();
    let mut wk = w.clone();
    let orders = super::cyclotomic_orders(n);
    let bound = orders.iter().copied().fold(1, u32::max);
    // MR1b — AN ODD VERSOR ACTS AS `x ↦ −Ω x Ω⁻¹`, so its `k`-th power
    // carries the sign `(−1)^k`: the action closes where `Ωᵏ x Ω⁻ᵏ` is
    // that sign times `x`, never where the bare sandwich is.
    let odd = super::motion::is_odd(motion);
    let (mut acc, mut acc_inv) = (motion.clone(), inverse.clone());
    for k in 1..=bound {
        let flip = odd && k % 2 == 1;
        let closes = |v: &Mv| {
            let image = acc.geo(v).geo(&acc_inv);
            let held = v.scale(&wk);
            match flip {
                true => image.add(&held).is_zero(),
                false => image.sub(&held).is_zero(),
            }
        };
        if orders.contains(&k) && plane.iter().all(closes) {
            return Some(k);
        }
        acc = acc.geo(motion);
        acc_inv = inverse.geo(&acc_inv);
        wk = &wk * &w;
    }
    None
}

/// THE MODES OF `b`'s bivector part (MD1). Each mode's blade, class and
/// planes; the modes commute and re-sum to `B`, verified before they are
/// returned. The generalised kernel's part is listed first where it is not
/// zero (at most one parabolic mode), then the elliptic and hyperbolic modes
/// by their value, ascending. An empty list is the zero generator.
pub fn modes(b: &Mv, budget: &mut crate::physics::Budget) -> Result<Vec<Mode>, Undecomposed> {
    let bivector = b.grade(2);
    if bivector.is_zero() {
        return Ok(Vec::new());
    }
    let n = bivector.n();
    let d = generators(n).len();
    // `F(x) = ⟨x B⟩₁`, the generator's action on the vectors, and `F²`.
    let f = super::LinearFunction::of(n, |x| x.geo(&bivector));
    let a = f.after(&f);
    let values = nonzero_roots(&a.characteristic(), budget)?;
    // The generalised kernel `ker(Aᵈ)`, and each value's space `ker(A − λ)`.
    let kernel = a.power(d).kernel();
    let eigen: Vec<Vec<Mv>> = values.iter().map(|l| a.shifted(l).kernel()).collect();
    if kernel.len() + eigen.iter().map(Vec::len).sum::<usize>() != d {
        return Err(Undecomposed::Defective);
    }
    // THE PROJECTORS AS POLYNOMIALS IN `A`: one per value, and the
    // generalised kernel's as the identity less them.
    let per_value = spectral_projectors(&a, &values, Some(&Q::zero()));
    let onto_kernel = per_value.iter().fold(super::LinearFunction::identity(n), |acc, p| acc.minus(p));
    let spaces: Vec<(Option<&Q>, usize, &super::LinearFunction)> = std::iter::once((None, kernel.len(), &onto_kernel))
        .chain(values.iter().zip(&eigen).zip(&per_value).map(|((l, s), p)| (Some(l), s.len(), p)))
        .collect();
    let mut out: Vec<Mode> = Vec::new();
    for (value, width, p) in spaces {
        let blade = project(&bivector, p);
        if blade.is_zero() {
            continue;
        }
        // Planes the mode spans: its eigenspace's dimension halved, or one
        // for the parabolic part of the kernel.
        let planes = match value {
            Some(_) => width / 2,
            None => 1,
        };
        out.push(Mode { kind: kind_of(&blade), blade, planes });
    }
    // VERIFIED AGAINST THE ALGEBRA, never assumed: a one-plane mode is
    // simple, the modes re-sum to `B`, and they commute pairwise.
    if out.iter().any(|m| m.planes == 1 && !m.blade.geo(&m.blade).grade(4).is_zero()) {
        return Err(Undecomposed::Defective);
    }
    let sum = out.iter().fold(Mv::zero(), |acc, m| acc.add(&m.blade));
    if !sum.sub(&bivector).is_zero() {
        return Err(Undecomposed::Defective);
    }
    for (i, x) in out.iter().enumerate() {
        for y in out.iter().skip(i + 1) {
            if !x.blade.geo(&y.blade).sub(&y.blade.geo(&x.blade)).is_zero() {
                return Err(Undecomposed::Defective);
            }
        }
    }
    Ok(out)
}
