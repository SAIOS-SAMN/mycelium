// 78.130 — CERTIFIED REALS, the network's standard math (the ruled
// split: standard math server-side, custom math client-side; requested
// 2026-10-07): every transcendental a kernel needs, as
// a PROVEN RATIONAL ENCLOSURE — never a float.
//
//! A real is [`Certified`]: [`Certified::Exact`] where its value is an exact
//! rational, [`Certified::Enclosed`] where it is not, held by an [`Encl`], an
//! interval `[lo, hi]` of exact rationals with `lo < hi` that is proved to
//! hold the true value. AN ENCLOSURE'S WIDTH IS STRICTLY POSITIVE (78.182
//! item 24, MU-DERIVATIONS D4.8 O1): a zero-width enclosure is a point and is
//! refused, so a point is always `Exact` (law 11, exact or refused). The
//! fields of an `Encl` are written only here, and only with `lo < hi`
//! (`encl_refuses_zero_width`); outside this module the struct cannot be
//! written:
//!
//! ```compile_fail,E0451
//! use num_rational::BigRational as Q;
//! fn point(v: Q) -> mycelium::core::certified::Encl {
//!     mycelium::core::certified::Encl { lo: v.clone(), hi: v }
//! }
//! ```
//!
//! The lawful path, which compiles, refuses the point:
//!
//! ```
//! use num_rational::BigRational as Q;
//! fn enclosure(lo: Q, hi: Q) -> Option<mycelium::core::certified::Encl> {
//!     mycelium::core::certified::Encl::new(lo, hi)
//! }
//! ```
//!
//! Every operation rounds OUTWARD to the dyadic grid `2^-w` (`w` the
//! caller's precision in bits), so denominators stay powers of two and the
//! same inputs give the same enclosure, bit for bit, on every machine and
//! thread count — nothing here reads a float, a clock or a thread. Bounds
//! that meet after any operation are answered `Exact`; a function at an
//! argument where its value is exactly rational (`exp 0 = 1`, `cos 0 = 1`,
//! `ln 1 = 0`, `√(9/4) = 3/2`, `lnΓ 1 = 0`) answers `Exact`.
//!
//! Each function names its remainder bound. A series is carried at `w`
//! plus guard bits, its terms outward-rounded; it stops when a term's bound
//! falls below `2^-(w₂-8)` — eight bits COARSER than the grid it rounds to,
//! since an outward-rounded term never falls below the grid itself — and
//! the remainder is added as `±` that bound. `lt` decides only on separated
//! bounds.

use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};

/// A PROVEN ENCLOSURE OF POSITIVE WIDTH: `lo ≤ true value ≤ hi`, `lo < hi`,
/// exact rationals. Built outside this module only by [`Encl::new`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encl {
    lo: Q,
    hi: Q,
}

/// A CERTIFIED REAL: its exact rational value, or a proven enclosure of
/// positive width where no exact value is held.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Certified {
    Exact(Q),
    Enclosed(Encl),
}

fn pow2(bits: u32) -> BigInt {
    BigInt::one() << bits
}

fn q(n: i64) -> Q {
    Q::from_integer(BigInt::from(n))
}

fn qmin(a: Q, b: Q) -> Q {
    if a <= b {
        a
    } else {
        b
    }
}

fn qmax(a: Q, b: Q) -> Q {
    if a >= b {
        a
    } else {
        b
    }
}

/// `q` rounded DOWN to the grid `2^-bits`.
fn down(v: &Q, bits: u32) -> Q {
    let s = pow2(bits);
    Q::new((v * Q::from_integer(s.clone())).floor().to_integer(), s)
}

/// `q` rounded UP to the grid `2^-bits`.
fn up(v: &Q, bits: u32) -> Q {
    let s = pow2(bits);
    Q::new((v * Q::from_integer(s.clone())).ceil().to_integer(), s)
}

/// The grid's unit, eight bits coarser: where a series stops.
fn stop_at(bits: u32) -> Q {
    Q::new(BigInt::one(), pow2(bits - 8))
}

impl Encl {
    /// `[lo, hi]`, where `lo < hi`; `lo = hi` is a point, refused (78.182
    /// item 24), and is written [`Certified::Exact`].
    pub fn new(lo: Q, hi: Q) -> Option<Encl> {
        (lo < hi).then_some(Encl { lo, hi })
    }

    pub fn lo(&self) -> &Q {
        &self.lo
    }

    pub fn hi(&self) -> &Q {
        &self.hi
    }

    /// `hi − lo`, strictly positive.
    pub fn width(&self) -> Q {
        &self.hi - &self.lo
    }

    pub fn contains(&self, v: &Q) -> bool {
        &self.lo <= v && v <= &self.hi
    }

    /// Rounded OUTWARD to the grid `2^-bits`: `down(lo) ≤ lo < hi ≤ up(hi)`,
    /// so the width stays positive.
    pub fn round(&self, bits: u32) -> Encl {
        Encl { lo: down(&self.lo, bits), hi: up(&self.hi, bits) }
    }
}

impl Certified {
    /// `[lo, hi]` CHECKED: `Exact` where the bounds meet, `Enclosed` where
    /// `lo < hi`, refused where `lo > hi`.
    pub fn between(lo: Q, hi: Q) -> Option<Certified> {
        match lo.cmp(&hi) {
            std::cmp::Ordering::Less => Some(Certified::Enclosed(Encl { lo, hi })),
            std::cmp::Ordering::Equal => Some(Certified::Exact(lo)),
            std::cmp::Ordering::Greater => None,
        }
    }

    /// The bounds a site in this module computes, ordered by construction:
    /// where they meet, the value is `Exact`. Were a pair ever reversed, the
    /// interval between them is taken, never a narrower one.
    fn span(lo: Q, hi: Q) -> Certified {
        match lo.cmp(&hi) {
            std::cmp::Ordering::Less => Certified::Enclosed(Encl { lo, hi }),
            std::cmp::Ordering::Equal => Certified::Exact(lo),
            std::cmp::Ordering::Greater => Certified::Enclosed(Encl { lo: hi, hi: lo }),
        }
    }

    /// The lower bound: the value itself where exact.
    pub fn lo(&self) -> &Q {
        match self {
            Certified::Exact(v) => v,
            Certified::Enclosed(e) => &e.lo,
        }
    }

    /// The upper bound: the value itself where exact.
    pub fn hi(&self) -> &Q {
        match self {
            Certified::Exact(v) => v,
            Certified::Enclosed(e) => &e.hi,
        }
    }

    /// `±r` about zero; `Exact` zero where `r = 0`.
    fn pm(r: Q) -> Certified {
        Certified::span(-r.clone(), r)
    }

    /// Rounded OUTWARD to the grid `2^-bits`; an exact value stays exact.
    pub fn round(&self, bits: u32) -> Certified {
        match self {
            Certified::Exact(v) => Certified::Exact(v.clone()),
            Certified::Enclosed(e) => Certified::Enclosed(e.round(bits)),
        }
    }

    /// Both bounds rounded OUTWARD to the grid `2^-bits`, an exact value
    /// included: what keeps a series' denominators powers of two. A value on
    /// the grid stays exact.
    fn outward(&self, bits: u32) -> Certified {
        Certified::span(down(self.lo(), bits), up(self.hi(), bits))
    }

    /// `hi − lo`: zero where exact.
    pub fn width(&self) -> Q {
        self.hi() - self.lo()
    }

    pub fn contains(&self, v: &Q) -> bool {
        self.lo() <= v && v <= self.hi()
    }

    /// `self < other`, decided only where the bounds separate.
    pub fn lt(&self, other: &Certified) -> Option<bool> {
        if self.hi() < other.lo() {
            Some(true)
        } else if self.lo() >= other.hi() {
            Some(false)
        } else {
            None
        }
    }

    pub fn neg(&self) -> Certified {
        match self {
            Certified::Exact(v) => Certified::Exact(-v.clone()),
            Certified::Enclosed(e) => Certified::Enclosed(Encl { lo: -e.hi.clone(), hi: -e.lo.clone() }),
        }
    }

    pub fn add(&self, o: &Certified) -> Certified {
        Certified::span(self.lo() + o.lo(), self.hi() + o.hi())
    }

    pub fn sub(&self, o: &Certified) -> Certified {
        self.add(&o.neg())
    }

    pub fn mul(&self, o: &Certified) -> Certified {
        let p = [self.lo() * o.lo(), self.lo() * o.hi(), self.hi() * o.lo(), self.hi() * o.hi()];
        let [a, b, c, d] = p;
        Certified::span(qmin(qmin(a.clone(), b.clone()), qmin(c.clone(), d.clone())), qmax(qmax(a, b), qmax(c, d)))
    }

    /// Times an exact rational.
    pub fn scale(&self, s: &Q) -> Certified {
        self.mul(&Certified::Exact(s.clone()))
    }

    /// The square, tight where the interval straddles zero.
    pub fn square(&self) -> Certified {
        let (lo, hi) = (self.lo(), self.hi());
        let (a, b) = (lo * lo, hi * hi);
        if lo.is_negative() && hi.is_positive() {
            Certified::span(Q::zero(), qmax(a, b))
        } else {
            Certified::span(qmin(a.clone(), b.clone()), qmax(a, b))
        }
    }

    /// `1 / self` — none where the interval holds zero.
    pub fn recip(&self) -> Option<Certified> {
        if self.lo() <= &Q::zero() && self.hi() >= &Q::zero() {
            return None;
        }
        Some(Certified::span(self.hi().recip(), self.lo().recip()))
    }

    pub fn div(&self, o: &Certified) -> Option<Certified> {
        Some(self.mul(&o.recip()?))
    }

    /// The largest magnitude it holds.
    pub fn abs_max(&self) -> Q {
        qmax(self.lo().abs(), self.hi().abs())
    }

    /// Intersected with `[lo, hi]` — what the function's range proves.
    fn clamp(&self, lo: &Q, hi: &Q) -> Certified {
        let a = qmax(self.lo().clone(), lo.clone());
        let b = qmin(self.hi().clone(), hi.clone());
        match a <= b {
            true => Certified::span(a, b),
            false => Certified::span(lo.clone(), hi.clone()),
        }
    }
}

/// AN INCREASING FUNCTION BY ITS ENDPOINTS: `f(lo).lo` to `f(hi).hi`; an
/// exact argument is read once, at its own point.
fn increasing(x: &Certified, f: impl Fn(&Q) -> Certified) -> Certified {
    match x {
        Certified::Exact(v) => f(v),
        Certified::Enclosed(e) => Certified::span(f(&e.lo).lo().clone(), f(&e.hi).hi().clone()),
    }
}

/// [`increasing`] for a function that may refuse a point.
fn rising(x: &Certified, f: impl Fn(&Q) -> Option<Certified>) -> Option<Certified> {
    match x {
        Certified::Exact(v) => f(v),
        Certified::Enclosed(e) => Some(Certified::span(f(&e.lo)?.lo().clone(), f(&e.hi)?.hi().clone())),
    }
}

/// A DECREASING FUNCTION BY ITS ENDPOINTS: `f(hi).lo` to `f(lo).hi`, checked;
/// an exact argument is read once, at its own point.
fn falling(x: &Certified, f: impl Fn(&Q) -> Option<Certified>) -> Option<Certified> {
    match x {
        Certified::Exact(v) => f(v),
        Certified::Enclosed(e) => Certified::between(f(&e.hi)?.lo().clone(), f(&e.lo)?.hi().clone()),
    }
}

/// WHERE AN EXACT RATIONAL `r` STANDS AGAINST A REAL `x`, decided past any
/// grid (MU-DERIVATIONS D1.2 Phase A): `x`'s enclosure `f(w)` is refined —
/// `w` doubled from the bits that resolve `r` itself — until it separates
/// from `r`: `Less` where `r < x`, `Greater` where `r > x`, `Equal` only where
/// `x` is exact and meets `r`. For an irrational `x` the refinement ends at
/// a precision bounded by `r`'s own size (`x`'s irrationality measure is
/// finite: for `π`, `|π − p/q| > q^{−7.103}` past a finite `q₀`,
/// Zeilberger–Zudilin), so no grid stands and none is capped. `None` only
/// where the precision would pass its `u32` carrier: undecided, named.
pub fn separates(r: &Q, f: impl Fn(u32) -> Certified) -> Option<std::cmp::Ordering> {
    let size = |z: &BigInt| u32::try_from(z.bits()).ok();
    let mut w = size(r.numer())?.checked_add(size(r.denom())?)?.checked_add(2)?;
    loop {
        let x = f(w);
        if r < x.lo() {
            return Some(std::cmp::Ordering::Less);
        }
        if r > x.hi() {
            return Some(std::cmp::Ordering::Greater);
        }
        if let Certified::Exact(_) = x {
            return Some(std::cmp::Ordering::Equal);
        }
        w = w.checked_mul(2)?;
    }
}

/// `bits` of a rational's magnitude: enough guard to carry it.
fn mag_bits(v: &Q) -> u32 {
    let n = v.abs().ceil().to_integer();
    u32::try_from(n.bits()).map_or(u32::MAX >> 4, |b| b)
}

// ———————————————————————————————— exp ————————————————————————————————

/// `e^v` at an exact point: halved `m` times to `|r| ≤ ½`, Taylor in `r`,
/// squared back `m` times. REMAINDER: after the term `t_n`, the tail is
/// `Σ_{j>n} |r|^j/j! ≤ |t_n|·(|r|/(n+1))/(1 − |r|/(n+2)) ≤ |t_n|/3`.
fn exp_point(v: &Q, w: u32) -> Certified {
    let half = Q::new(BigInt::one(), BigInt::from(2));
    let mut r = v.clone();
    let mut m = 0u32;
    while r.abs() > half {
        r /= q(2);
        m += 1;
    }
    // D4.8 (e): the squarings amplify the relative width by `2^m`, and the
    // absolute width of `e^v` carries the value's own magnitude, `v·log₂e <
    // 3v/2` bits for `v > 0`; both are read off `v`.
    let lift = match v.is_positive() {
        true => u32::try_from((v * q(3) / q(2)).ceil().to_integer()).map_or(u32::MAX >> 4, |b| b),
        false => 0,
    };
    let w2 = w + 24 + m + lift;
    let tiny = stop_at(w2);
    let rr = Certified::Exact(r);
    let mut term = Certified::Exact(Q::one());
    let mut sum = term.clone();
    let mut k = 1i64;
    loop {
        term = term.mul(&rr).scale(&Q::new(BigInt::one(), BigInt::from(k))).outward(w2);
        sum = sum.add(&term).outward(w2);
        if term.abs_max() <= tiny {
            break;
        }
        k += 1;
    }
    let mut out = sum.add(&Certified::pm(term.abs_max())).outward(w2);
    for _ in 0..m {
        out = out.square().outward(w2);
    }
    out.outward(w)
}

/// `e^x`: monotone, so its endpoints' enclosures bound it; `e^0 = 1` exact.
pub fn exp(x: &Certified, w: u32) -> Certified {
    increasing(x, |v| match v.is_zero() {
        true => Certified::Exact(Q::one()),
        false => exp_point(v, w),
    })
}

// ———————————————————————————————— ln ————————————————————————————————

/// `2·atanh(z) = 2 Σ z^{2j+1}/(2j+1)` for `0 ≤ z ≤ ⅓`. REMAINDER: after the
/// term `t_n`, the tail is `≤ t_n·z²/(1−z²) ≤ t_n/8`.
fn atanh2(z: &Q, w2: u32) -> Certified {
    let tiny = stop_at(w2);
    let z2 = Certified::Exact(z * z);
    let mut pow = Certified::Exact(z.clone());
    let mut sum = Certified::Exact(Q::zero());
    let mut j = 0i64;
    loop {
        let term = pow.scale(&Q::new(BigInt::from(2), BigInt::from(2 * j + 1))).outward(w2);
        sum = sum.add(&term).outward(w2);
        if term.abs_max() <= tiny {
            return sum.add(&Certified::pm(term.abs_max())).outward(w2);
        }
        pow = pow.mul(&z2).outward(w2);
        j += 1;
    }
}

/// `ln 2 = 2·atanh(⅓)`.
pub fn ln2(w: u32) -> Certified {
    atanh2(&Q::new(BigInt::one(), BigInt::from(3)), w + 16).outward(w)
}

/// `ln v` at an exact point `v > 0`: `v = 2^k·m`, `m ∈ [1, 2)`, and `ln m =
/// 2·atanh((m−1)/(m+1))` with `(m−1)/(m+1) < ⅓`.
fn ln_point(v: &Q, w: u32) -> Option<Certified> {
    if !v.is_positive() {
        return None;
    }
    let nb = i64::try_from(v.numer().bits()).ok()?;
    let db = i64::try_from(v.denom().bits()).ok()?;
    let mut k = nb - db;
    let scale = |k: i64| -> Option<Q> {
        let p = Q::from_integer(pow2(u32::try_from(k.abs()).ok()?));
        Some(if k >= 0 { p } else { p.recip() })
    };
    let mut m = v / scale(k)?;
    while m >= q(2) {
        m /= q(2);
        k += 1;
    }
    while m < Q::one() {
        m *= q(2);
        k -= 1;
    }
    let w2 = w + 24 + u32::try_from(k.unsigned_abs().max(1)).ok()?.ilog2() + 1;
    let z = (&m - Q::one()) / (&m + Q::one());
    let lnm = atanh2(&z, w2);
    let l2 = atanh2(&Q::new(BigInt::one(), BigInt::from(3)), w2 + 64);
    Some(l2.scale(&q(k)).add(&lnm).outward(w))
}

/// `ln x` for `x > 0`: monotone, by its endpoints; `ln 1 = 0` exact.
pub fn ln(x: &Certified, w: u32) -> Option<Certified> {
    rising(x, |v| match v.is_one() {
        true => Some(Certified::Exact(Q::zero())),
        false => ln_point(v, w),
    })
}

// ———————————————————————————————— π ————————————————————————————————

/// `atan(1/n) = Σ (−1)^j / ((2j+1)·n^{2j+1})`. REMAINDER: an alternating
/// series of decreasing terms — the tail is bounded by the next term,
/// which the last term bounds.
fn atan_inv(n: i64, w2: u32) -> Certified {
    let tiny = stop_at(w2);
    let nn = Certified::Exact(Q::new(BigInt::one(), BigInt::from(n * n)));
    let mut pow = Certified::Exact(Q::new(BigInt::one(), BigInt::from(n)));
    let mut sum = Certified::Exact(Q::zero());
    let mut j = 0i64;
    loop {
        let mut term = pow.scale(&Q::new(BigInt::one(), BigInt::from(2 * j + 1)));
        if j % 2 == 1 {
            term = term.neg();
        }
        let term = term.outward(w2);
        sum = sum.add(&term).outward(w2);
        if term.abs_max() <= tiny {
            return sum.add(&Certified::pm(term.abs_max())).outward(w2);
        }
        pow = pow.mul(&nn).outward(w2);
        j += 1;
    }
}

/// `π = 16·atan(1/5) − 4·atan(1/239)` (Machin).
pub fn pi(w: u32) -> Certified {
    let w2 = w + 16;
    atan_inv(5, w2).scale(&q(16)).sub(&atan_inv(239, w2).scale(&q(4))).outward(w)
}

// ———————————————————————————————— √ and powers ————————————————————————————————

/// `√x` for `x ≥ 0`: the integer square roots of the endpoints on the grid
/// `4^-w`, floored below and ceiled above — exact bounds. At an exact
/// argument whose numerator and denominator are both perfect squares, the
/// root is that rational, `Exact`.
pub fn sqrt(x: &Certified, w: u32) -> Option<Certified> {
    if x.lo().is_negative() {
        return None;
    }
    if let Some(root) = exact_root(x, 2) {
        return Some(Certified::Exact(root));
    }
    let g = Q::from_integer(pow2(2 * w));
    let lo_n = (x.lo() * &g).floor().to_integer();
    let hi_n = (x.hi() * &g).ceil().to_integer();
    let lo = lo_n.sqrt();
    let mut hi = hi_n.sqrt();
    if &hi * &hi < hi_n {
        hi += 1;
    }
    Some(Certified::span(Q::new(lo, pow2(w)), Q::new(hi, pow2(w))))
}

/// THE `n`-TH ROOT OF AN EXACT NON-NEGATIVE RATIONAL, where it is one: the
/// integer roots of the reduced numerator and denominator, read back by
/// their power. `None` for an enclosure, or where either is no `n`-th power.
fn exact_root(x: &Certified, n: u32) -> Option<Q> {
    let Certified::Exact(v) = x else { return None };
    if v.is_negative() || n == 0 {
        return None;
    }
    let (a, b) = (v.numer().nth_root(n), v.denom().nth_root(n));
    (&a.pow(n) == v.numer() && &b.pow(n) == v.denom()).then(|| Q::new(a, b))
}

/// `x^e` for `x > 0` and a rational `e`: `exp(e·ln x)`, monotone pieces. At
/// an exact `x` whose `e`-th power is rational — `e` an integer, or `x` a
/// perfect power of `e`'s denominator — the power is that rational, `Exact`.
pub fn powq(x: &Certified, e: &Q, w: u32) -> Option<Certified> {
    if !x.lo().is_positive() {
        return None;
    }
    if let Some(root) = u32::try_from(e.denom()).ok().and_then(|d| exact_root(x, d)) {
        if let Ok(p) = i32::try_from(e.numer()) {
            return Some(Certified::Exact(root.pow(p)));
        }
    }
    let w2 = w + 32 + mag_bits(e);
    Some(exp(&ln(x, w2)?.scale(e).outward(w2), w2).outward(w))
}

// ———————————————————————————————— sin, cos ————————————————————————————————

/// `(sin x, cos x)`: reduced by the nearest multiple of `π/2`, then the
/// Taylor series of both carried on intervals. REMAINDER: truncated after
/// degree `N−1`, each series is off by at most `M^N/N!` (Lagrange), `M` the
/// reduced argument's largest magnitude. `sin 0 = 0` and `cos 0 = 1`, exact.
pub fn sin_cos(x: &Certified, w: u32) -> (Certified, Certified) {
    if x == &Certified::Exact(Q::zero()) {
        return (Certified::Exact(Q::zero()), Certified::Exact(Q::one()));
    }
    let w2 = w + 32 + mag_bits(&x.abs_max());
    let (r, quadrant) = if x.abs_max() > Q::one() {
        let half_pi = pi(w2 + mag_bits(&x.abs_max())).scale(&Q::new(BigInt::one(), BigInt::from(2)));
        let mid = (x.lo() + x.hi()) / q(2);
        let k = (mid / half_pi.lo()).round().to_integer();
        let r = x.sub(&half_pi.scale(&Q::from_integer(k.clone()))).outward(w2);
        (r, k.mod_floor(&BigInt::from(4)))
    } else {
        (x.clone(), BigInt::zero())
    };
    let tiny = stop_at(w2);
    let m = r.abs_max();
    let mut term = Certified::Exact(Q::one());
    let mut bound = Q::one();
    let (mut s, mut c) = (Certified::Exact(Q::zero()), Certified::Exact(Q::zero()));
    let mut k = 0i64;
    loop {
        // term = r^k / k!, signed into its series.
        match k % 4 {
            0 => c = c.add(&term),
            1 => s = s.add(&term),
            2 => c = c.sub(&term),
            _ => s = s.sub(&term),
        }
        k += 1;
        term = term.mul(&r).scale(&Q::new(BigInt::one(), BigInt::from(k))).outward(w2);
        bound = up(&(&bound * &m / q(k)), w2);
        if bound <= tiny {
            break;
        }
    }
    let rem = Certified::pm(bound);
    let (s, c) = (s.add(&rem), c.add(&rem));
    // sin(r + kπ/2), cos(r + kπ/2) by the quadrant `k mod 4`.
    let (s, c) = match u8::try_from(&quadrant) {
        Ok(1) => (c, s.neg()),
        Ok(2) => (s.neg(), c.neg()),
        Ok(3) => (c.neg(), s),
        _ => (s, c),
    };
    let one = Q::one();
    (s.clamp(&-one.clone(), &one).outward(w), c.clamp(&-one.clone(), &one).outward(w))
}

pub fn sin(x: &Certified, w: u32) -> Certified {
    sin_cos(x, w).0
}

pub fn cos(x: &Certified, w: u32) -> Certified {
    sin_cos(x, w).1
}

// ———————————————————————————————— lgamma ————————————————————————————————

/// The Bernoulli numbers `B_0..=B_m`, exact: `B_m = −1/(m+1)·Σ_{k<m}
/// C(m+1, k)·B_k`.
fn bernoulli(m: usize) -> Vec<Q> {
    let mut b: Vec<Q> = Vec::with_capacity(m + 1);
    for n in 0..=m {
        if n == 0 {
            b.push(Q::one());
            continue;
        }
        let mut acc = Q::zero();
        let mut binom = BigInt::one(); // C(n+1, 0)
        for (k, bk) in b.iter().enumerate() {
            acc += Q::from_integer(binom.clone()) * bk;
            binom = binom * BigInt::from(n + 1 - k) / BigInt::from(k + 1);
        }
        b.push(-acc / Q::from_integer(BigInt::from(n + 1)));
    }
    b
}

/// `ln Γ(x)` for `x > 0`: shifted to `y = x + N ≥ X₀` by `ln Γ(x) = ln Γ(y) −
/// ln(x(x+1)…(x+N−1))`, then Stirling's series `(y−½)ln y − y + ½ln 2π +
/// Σ B_{2k}/(2k(2k−1)y^{2k−1})`. REMAINDER: for real `y > 0` the error is
/// bounded by the first omitted term, `|B_{2n+2}|/((2n+2)(2n+1)·y^{2n+1})`;
/// `X₀ = w₂/8 + 2` keeps the series' least term below the grid. `ln Γ(1) =
/// ln Γ(2) = 0`, exact.
pub fn lgamma(x: &Certified, w: u32) -> Option<Certified> {
    if !x.lo().is_positive() {
        return None;
    }
    if x == &Certified::Exact(Q::one()) || x == &Certified::Exact(q(2)) {
        return Some(Certified::Exact(Q::zero()));
    }
    let w2 = w + 32 + mag_bits(x.hi());
    let x0 = q(i64::from(w2 / 8 + 2));
    let shift = if x.lo() < &x0 { (&x0 - x.lo()).ceil().to_integer() } else { BigInt::zero() };
    let steps = usize::try_from(&shift).ok()?;
    let mut prod = Certified::Exact(Q::one());
    for i in 0..steps {
        prod = prod.mul(&x.add(&Certified::Exact(q(i64::try_from(i).ok()?)))).outward(w2);
    }
    let y = x.add(&Certified::Exact(Q::from_integer(shift))).outward(w2);
    let ln_y = ln(&y, w2)?;
    let two_pi = pi(w2).scale(&q(2));
    let half = Q::new(BigInt::one(), BigInt::from(2));
    let mut s = y
        .sub(&Certified::Exact(half.clone()))
        .mul(&ln_y)
        .sub(&y)
        .add(&ln(&two_pi, w2)?.scale(&half))
        .outward(w2);
    let tiny = stop_at(w2);
    // The powers of 1/y kept EXACT: rounded to the grid, a power near the
    // grid's unit would be dominated by its own rounding, and the Bernoulli
    // numbers would grow that error into the sum.
    let y_recip = y.recip()?;
    let y2_recip = y_recip.square();
    let mut ypow = y_recip.clone(); // y^{-(2k-1)}
    let mut b = bernoulli(8);
    let mut k = 1usize;
    loop {
        if b.len() <= 2 * k + 2 {
            b = bernoulli(4 * k + 8);
        }
        let denom = Q::from_integer(BigInt::from(2 * k * (2 * k - 1)));
        let term = ypow.scale(&(&b[2 * k] / &denom)).outward(w2);
        s = s.add(&term).outward(w2);
        // The next term's bound, at the smallest y.
        let next_den = Q::from_integer(BigInt::from((2 * k + 2) * (2 * k + 1)));
        let next = up(&(b[2 * k + 2].abs() / next_den * ypow.mul(&y2_recip).abs_max()), w2);
        if next <= tiny {
            s = s.add(&Certified::pm(next));
            break;
        }
        if next >= term.abs_max() && k > 2 {
            // The asymptotic series turned before the grid: X₀ too small.
            return None;
        }
        ypow = ypow.mul(&y2_recip);
        k += 1;
    }
    let out = if steps == 0 { s } else { s.sub(&ln(&prod, w2)?) };
    Some(out.outward(w))
}

/// `sinh x = (e^x − e^{−x})/2`: increasing, by its endpoints.
pub fn sinh(x: &Certified, w: u32) -> Certified {
    let w2 = w + 8;
    let at = |v: &Q| exp_point(v, w2).sub(&exp_point(&-v.clone(), w2)).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    increasing(x, at).outward(w)
}

/// `cosh x = (e^x + e^{−x})/2`: even, least at zero, increasing in `|x|`.
pub fn cosh(x: &Certified, w: u32) -> Certified {
    let w2 = w + 8;
    let at = |v: &Q| exp_point(v, w2).add(&exp_point(&-v.clone(), w2)).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    if let Certified::Exact(v) = x {
        return at(v).outward(w);
    }
    let far = at(&x.abs_max());
    let near = if x.lo().is_negative() && x.hi().is_positive() {
        Q::one()
    } else {
        at(&qmin(x.lo().abs(), x.hi().abs())).lo().clone()
    };
    Certified::span(near, far.hi().clone()).outward(w)
}

/// The logistic `1/(1 + e^{−x})`: increasing, by its endpoints.
pub fn logistic(x: &Certified, w: u32) -> Option<Certified> {
    let w2 = w + 8;
    let at = |v: &Q| Certified::Exact(Q::one()).add(&exp_point(&-v.clone(), w2)).recip();
    Some(rising(x, at)?.outward(w))
}

// ———————————————————————————————— atan, acos, tan ————————————————————————————————

/// `atan v` for `|v| ≤ 1`, by Euler's series `atan v = v/(1+v²) · Σ_k
/// ∏_{j≤k} (2j/(2j+1)) · y^k`, `y = v²/(1+v²) ≤ ½`. REMAINDER: the terms are
/// positive and each ratio is `≤ y ≤ ½`, so the tail after the term `t_n` is
/// `≤ t_n·y/(1−y) ≤ t_n`.
fn atan_small(v: &Q, w2: u32) -> Certified {
    let tiny = stop_at(w2);
    let v2 = v * v;
    let y = &v2 / (Q::one() + &v2);
    let lead = v / (Q::one() + &v2);
    let mut term = Certified::Exact(Q::one());
    let mut sum = term.clone();
    let mut k = 1i64;
    loop {
        term = term.scale(&(Q::new(BigInt::from(2 * k), BigInt::from(2 * k + 1)) * &y)).outward(w2);
        sum = sum.add(&term).outward(w2);
        if term.abs_max() <= tiny {
            break;
        }
        k += 1;
    }
    sum.add(&Certified::span(Q::zero(), term.abs_max())).scale(&lead).outward(w2)
}

/// `atan v` at an exact point: for `|v| > 1`, `sign(v)·π/2 − atan(1/v)`.
fn atan_point(v: &Q, w: u32) -> Certified {
    let w2 = w + 24;
    if v.abs() <= Q::one() {
        return atan_small(v, w2).outward(w);
    }
    let half_pi = pi(w2).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    let inner = atan_small(&v.recip(), w2);
    let out = if v.is_positive() { half_pi.sub(&inner) } else { half_pi.neg().sub(&inner) };
    out.outward(w)
}

/// `atan x`: increasing, by its endpoints.
pub fn atan(x: &Certified, w: u32) -> Certified {
    increasing(x, |v| atan_point(v, w))
}

/// `acos v` at an exact point `−1 ≤ v ≤ 1`: `acos v = 2·atan(√((1−v)/(1+v)))`;
/// `acos(−1) = π`, `acos 1 = 0`.
fn acos_point(v: &Q, w: u32) -> Option<Certified> {
    let w2 = w + 24;
    let one = Q::one();
    if v > &one || v < &-one.clone() {
        return None;
    }
    if v == &-one.clone() {
        return Some(pi(w));
    }
    let ratio = (&one - v) / (&one + v);
    let root = sqrt(&Certified::Exact(ratio), w2)?;
    let a = increasing(&root, |v| atan_point(v, w2));
    Some(a.scale(&q(2)).outward(w))
}

/// `acos x` for `x ⊂ [−1, 1]`: decreasing, by its endpoints.
pub fn acos(x: &Certified, w: u32) -> Option<Certified> {
    falling(x, |v| acos_point(v, w))
}

/// `tan x = sin x / cos x` — none where the enclosure of `cos x` holds zero.
pub fn tan(x: &Certified, w: u32) -> Option<Certified> {
    let (s, c) = sin_cos(x, w + 16);
    Some(s.div(&c)?.outward(w))
}

// ———————————————————————————————— the Planck tail ————————————————————————————————

/// `15/π⁴`: the black-body normalization.
fn planck_norm(w2: u32) -> Option<Certified> {
    let p = pi(w2);
    let p4 = p.square().square();
    Some(p4.recip()?.scale(&q(15)))
}

/// `F(v)` for `v ≥ 1`: `15/π⁴ · Σ_{n≥1} e^{−nv}·p_n(v)`, `p_n(v) = v³/n + 3v²/n² +
/// 6v/n³ + 6/n⁴`. REMAINDER: every term is positive and `p_n` is decreasing in
/// `n`, so the tail after `N` terms is `Σ_{n>N} e^{−nv}p_n ≤ p_{N+1}·Σ_{n>N}
/// e^{−nv} = e^{−(N+1)v}·p_{N+1} / (1 − e^{−v})` — the next term over `1 − e^{−v}`.
fn planck_far(v: &Q, w2: u32) -> Option<Certified> {
    let tiny = stop_at(w2);
    let e = exp_point(&-v.clone(), w2);
    let gap = Certified::Exact(Q::one()).sub(&e).recip()?;
    let p = |n: i64| -> Q {
        let n = q(n);
        v * v * v / &n + q(3) * v * v / (&n * &n) + q(6) * v / (&n * &n * &n) + q(6) / (&n * &n * &n * &n)
    };
    let mut en = e.clone();
    let mut sum = Certified::Exact(Q::zero());
    let mut n = 1i64;
    loop {
        sum = sum.add(&en.scale(&p(n))).outward(w2);
        en = en.mul(&e).outward(w2);
        let tail = en.scale(&p(n + 1)).mul(&gap);
        if tail.hi() <= &tiny {
            sum = sum.add(&Certified::span(Q::zero(), tail.hi().clone()));
            break;
        }
        n += 1;
    }
    Some(sum.mul(&planck_norm(w2)?))
}

/// `F(v)` for `0 ≤ v < 1`: `1 − 15/π⁴·∫_0^v t³/(e^t−1) dt`, and `t³/(e^t−1) =
/// t²·Σ B_k t^k/k!`, so the integral is `Σ_k B_k v^{k+3}/((k+3)·k!)`.
/// REMAINDER: `|B_k|/k! ≤ 4/(2π)^k < 4/6^k`, so the tail after `K` terms is
/// `≤ 4v³·(v/6)^{K+1}/(1 − v/6)`.
fn planck_near(v: &Q, w2: u32) -> Option<Certified> {
    let tiny = stop_at(w2);
    let sixth = v / q(6);
    let mut b = bernoulli(32);
    let mut integral = Certified::Exact(Q::zero());
    let mut vpow = v * v * v; // v^{k+3}
    let mut fact = BigInt::one(); // k!
    let mut k = 0usize;
    loop {
        if b.len() <= k {
            b = bernoulli(2 * k + 16);
        }
        let kq = Q::from_integer(BigInt::from(k + 3)) * Q::from_integer(fact.clone());
        integral = integral.add(&Certified::Exact(&b[k] * &vpow / kq)).outward(w2);
        k += 1;
        fact *= BigInt::from(k);
        vpow *= v;
        // The tail after K = k terms.
        let mut bound = q(4) * v * v * v / (Q::one() - &sixth);
        for _ in 0..=k {
            bound *= &sixth;
        }
        if up(&bound, w2) <= tiny {
            integral = integral.add(&Certified::pm(up(&bound, w2)));
            break;
        }
    }
    Some(Certified::Exact(Q::one()).sub(&integral.mul(&planck_norm(w2)?)))
}

/// THE PLANCK TAIL `F(x) = 15/π⁴ · ∫_x^∞ t³/(e^t − 1) dt`: the fraction of
/// black-body energy above `x = hν/kT`. `F(0) = 1` exactly; `F` decreasing, so
/// bounded by its endpoints; `x ≥ 0`.
pub fn planck_tail(x: &Certified, w: u32) -> Option<Certified> {
    if x.lo().is_negative() {
        return None;
    }
    let w2 = w + 32 + mag_bits(x.hi());
    let at = |v: &Q| -> Option<Certified> {
        if v.is_zero() {
            return Some(Certified::Exact(Q::one()));
        }
        if v < &Q::one() {
            planck_near(v, w2)
        } else {
            planck_far(v, w2)
        }
    };
    let out = falling(x, at)?;
    Some(out.clamp(&Q::zero(), &Q::one()).outward(w))
}
