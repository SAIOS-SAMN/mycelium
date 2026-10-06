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

/// 78.100 — THE ROOT READER'S REFUSALS FOR THE STROKE, in this process: how
/// many, the highest degree refused, and the widest coefficient (in bits) of
/// a polynomial whose roots the stroke ran out reading.
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

/// An integer polynomial's value at `y`, by Horner, ascending coefficients.
fn eval_int(h: &[num_bigint::BigInt], y: &num_bigint::BigInt) -> num_bigint::BigInt {
    h.iter().rev().fold(num_bigint::BigInt::from(0), |acc, c| acc * y + c)
}

/// 78.100, 78.102 — WHERE EVERY REAL ROOT OF AN INTEGER POLYNOMIAL STANDS, to
/// the integer, at any degree: each root bracketed `[lo, hi]` with `hi − lo`
/// zero (the root is the integer) or one (it lies strictly between two
/// consecutive integers, or — conservatively — a critical point does), and
/// every bracket inside the Cauchy bound `bound`.
///
/// RECURSIVE ON THE DERIVATIVE, exact throughout. The derivative's brackets
/// cut the line into runs on which `h` is monotone; each run is read at its
/// ends and, where the sign changes, bisected over the integers by the signs
/// of exact evaluations — as many steps as the bound has bits. A derivative's
/// bracket of width one may hide a root of `h` beside the critical point, so
/// it is kept as a bracket of `h` too: an over-reading never misses a root,
/// and the integers it holds are its two ends, read where they are used. No
/// integer is ever factored. `Err` where the stroke is spent, one spend per
/// evaluation (rule 10).
fn brackets(
    h: &[num_bigint::BigInt],
    bound: &num_bigint::BigInt,
    budget: &mut crate::physics::Budget,
) -> Result<Vec<(num_bigint::BigInt, num_bigint::BigInt)>, ()> {
    use num_bigint::{BigInt, Sign};
    // A constant (or nothing) has no root to bracket.
    if h.len() < 2 {
        return Ok(Vec::new());
    }
    // A LINE's root is `−h₀/h₁`, read exactly: its floor and its ceiling.
    if let [h0, h1] = h {
        let root = Q::new(-h0.clone(), h1.clone());
        return Ok(vec![(root.floor().to_integer(), root.ceil().to_integer())]);
    }
    let derivative: Vec<BigInt> = h.iter().enumerate().skip(1).map(|(i, c)| c * BigInt::from(i)).collect();
    let critical = brackets(&derivative, bound, budget)?;
    let mut out: Vec<(BigInt, BigInt)> = critical.iter().filter(|(lo, hi)| lo != hi).cloned().collect();
    // THE MONOTONE RUNS between the critical brackets, inside the bound.
    let mut ends: Vec<BigInt> = vec![-bound.clone()];
    for (lo, hi) in &critical {
        ends.push(lo.clone());
        ends.push(hi.clone());
    }
    ends.push(bound.clone());
    for run in ends.chunks(2) {
        let [lo, hi] = run else { continue };
        if lo > hi {
            continue;
        }
        if !(budget.spend() && budget.spend()) {
            return Err(());
        }
        let (slo, shi) = (eval_int(h, lo).sign(), eval_int(h, hi).sign());
        if slo == Sign::NoSign {
            out.push((lo.clone(), lo.clone()));
        }
        if shi == Sign::NoSign {
            out.push((hi.clone(), hi.clone()));
        }
        if slo == Sign::NoSign || shi == Sign::NoSign || slo == shi {
            continue;
        }
        // One sign change on a monotone run: one root between, read to an
        // integer or to the interval between two consecutive integers.
        let (mut a, mut b) = (lo.clone(), hi.clone());
        let unit = BigInt::from(1);
        let mut exact = None;
        while &b - &a > unit {
            if !budget.spend() {
                return Err(());
            }
            let mid: BigInt = (&a + &b) >> 1;
            match eval_int(h, &mid).sign() {
                Sign::NoSign => {
                    exact = Some(mid);
                    break;
                }
                s if s == slo => a = mid,
                _ => b = mid,
            }
        }
        match exact {
            Some(y) => out.push((y.clone(), y)),
            None => out.push((a, b)),
        }
    }
    Ok(out)
}

/// 78.100, 78.102, 78.63 (the wrong math) — THE INTEGER ROOTS `y` OF THE
/// MONIC SUBSTITUTE of an integer polynomial `a₀ + … + a_m xᵐ`, at any degree;
/// every rational root of the polynomial is `y / a_m`.
///
/// `y = a_m·x` turns `a_m^{m−1}·f(y / a_m)` into `yᵐ + Σ aᵢ a_m^{m−1−i} yⁱ`,
/// monic with integer coefficients, so its rational roots are integers (the
/// rational root theorem, its leading coefficient one), every one inside the
/// Cauchy bound `1 + max |gᵢ|`. They are read off [`brackets`] — each exact
/// bracket is a root, and each bracket's two ends are read, which holds every
/// integer a root could be. Trial division retired with this: it walked to
/// `√n`, some `2⁴²` steps for the 84-bit coefficients epoch 125 met, and a
/// degree cut-off stood where the algebra's own degree (`n + 2`) is the law.
fn monic_integer_roots(ints: &[num_bigint::BigInt], budget: &mut crate::physics::Budget) -> Result<Vec<num_bigint::BigInt>, ()> {
    use num_bigint::{BigInt, Sign};
    let Some(m) = ints.len().checked_sub(1).filter(|m| *m >= 1) else { return Ok(Vec::new()) };
    let am = &ints[m];
    // The monic substitute, ascending: g_i = a_i · a_m^{m−1−i}, g_m = 1.
    let g: Vec<BigInt> = (0..m).map(|i| &ints[i] * num_traits::pow(am.clone(), m - 1 - i)).chain([BigInt::from(1)]).collect();
    // The largest magnitude among the lower coefficients, folded: a reading,
    // never a pick by position.
    let bound: BigInt = g[..m].iter().fold(BigInt::from(0), |most: BigInt, c| most.max(c.abs())) + BigInt::from(1);
    let mut roots: Vec<BigInt> = Vec::new();
    for (lo, hi) in brackets(&g, &bound, budget)? {
        for y in [lo, hi] {
            if roots.contains(&y) {
                continue;
            }
            if !budget.spend() {
                return Err(());
            }
            if eval_int(&g, &y).sign() == Sign::NoSign {
                roots.push(y);
            }
        }
    }
    Ok(roots)
}

/// THE INTEGER FORM `p`'s rational roots are read on: square-free
/// (`p / gcd(p, p′)`), the root at zero divided out (whether it was there),
/// the rest over ℚ, and the rest's denominators cleared and common content
/// divided out — the same roots, the smallest integers that carry them (a
/// square-free quotient leaves a content the monic substitute would raise to
/// the power of the degree).
fn integer_form(p: &[Q]) -> (bool, Vec<Q>, Vec<num_bigint::BigInt>) {
    use num_bigint::BigInt;
    let p = trim(p.to_vec());
    let derivative: Vec<Q> =
        p.iter().enumerate().skip(1).map(|(i, c)| c * Q::from_integer(BigInt::from(i))).collect();
    let free = quotient(&p, &gcd(&p, &derivative));
    let rest: Vec<Q> = free.iter().skip_while(|c| c.is_zero()).cloned().collect();
    let zero = rest.len() < free.len();
    let lcm = rest.iter().fold(BigInt::from(1), |l, c| num_integer::Integer::lcm(&l, c.denom()));
    let cleared: Vec<BigInt> = rest.iter().map(|c| (c * Q::from_integer(lcm.clone())).to_integer()).collect();
    let content = cleared.iter().fold(BigInt::from(0), |g, c| num_integer::Integer::gcd(&g, c));
    let ints: Vec<BigInt> = match content.sign() {
        num_bigint::Sign::NoSign => cleared,
        _ => cleared.iter().map(|c| c / &content).collect(),
    };
    (zero, rest, ints)
}

/// 78.111 C1b — THE STROKE `p`'s ROOTS NEED, read off `p` itself and never a
/// chosen number: an upper bound on every spend [`rational_roots`] makes on
/// it. On the monic substitute of degree `m`, inside its Cauchy bound `B`:
/// [`brackets`] at the derivative of degree `j` reads at most `C_{j−1} + 1`
/// monotone runs, each two ends and at most `bits(2B)` bisections, and
/// returns at most `C_j ≤ 3·C_{j−1} + 2` brackets (`C₁ = 1`: a line's root is
/// read, not searched); [`monic_integer_roots`] then reads each bracket's two
/// ends. A stroke of this size never refuses `p` as `Unbounded`. Counted
/// exactly; `None` where the need is past the stroke's carrier (rule 11).
pub(super) fn roots_need(p: &[Q]) -> Option<usize> {
    use num_bigint::BigInt;
    let (_, _, ints) = integer_form(p);
    let Some(m) = ints.len().checked_sub(1).filter(|m| *m >= 1) else { return Some(0) };
    let am = &ints[m];
    let g: Vec<BigInt> = (0..m).map(|i| &ints[i] * num_traits::pow(am.clone(), m - 1 - i)).collect();
    let bound: BigInt = g.iter().fold(BigInt::from(0), |most: BigInt, c| most.max(c.abs())) + BigInt::from(1);
    let per_run = BigInt::from((bound * BigInt::from(2)).bits()) + BigInt::from(2);
    let (spent, brackets) = (2..=m).fold((BigInt::from(0), BigInt::from(1)), |(spent, c), _| {
        (spent + (&c + BigInt::from(1)) * &per_run, c * BigInt::from(3) + BigInt::from(2))
    });
    usize::try_from(spent + brackets * BigInt::from(2)).ok()
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
    let (zero, mut rest, ints) = integer_form(p);
    let mut roots: Vec<Q> = if zero { vec![Q::zero()] } else { Vec::new() };
    if let ([a0, ..], [.., am]) = (ints.as_slice(), ints.as_slice()) {
        if ints.len() > 1 {
            let unbounded = Undecomposed::Unbounded(ints.len() - 1);
            // 78.100 — A REFUSAL FOR THE STROKE IS READ: its degree and the
            // width of the coefficients its search was walking.
            let refuse = || {
                note_root_refusal(ints.len() - 1, a0.bits().max(am.bits()));
                unbounded
            };
            // THE CANDIDATES, at any degree, by the monic substitute and exact
            // bisection — no integer factored (78.100, 78.102).
            let candidates: Vec<Q> = match monic_integer_roots(&ints, budget) {
                Ok(ys) => ys.into_iter().map(|y| Q::new(y, am.clone())).collect(),
                Err(()) => return Err(refuse()),
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
