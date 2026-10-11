// 78.130 — CERTIFIED REALS, the network's standard math (the operator's
// split: standard math server-side, custom math client-side; the
// coordinator's request 2026-10-07): every transcendental a kernel needs, as
// a PROVEN RATIONAL ENCLOSURE — never a float.
//
//! An [`Encl`] is a closed interval `[lo, hi]` of exact rationals that is
//! proved to hold the true value. Every operation rounds OUTWARD to the
//! dyadic grid `2^-w` (`w` the caller's precision in bits), so denominators
//! stay powers of two and the same inputs give the same enclosure, bit for
//! bit, on every machine and thread count — nothing here reads a float, a
//! clock or a thread.
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

/// A PROVEN ENCLOSURE: `lo ≤ true value ≤ hi`, exact rationals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Encl {
    pub lo: Q,
    pub hi: Q,
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
    /// The exact value.
    pub fn exact(v: Q) -> Encl {
        Encl { lo: v.clone(), hi: v }
    }

    /// `[lo, hi]`, where `lo ≤ hi`.
    pub fn new(lo: Q, hi: Q) -> Option<Encl> {
        (lo <= hi).then_some(Encl { lo, hi })
    }

    /// `±r` about zero.
    fn pm(r: Q) -> Encl {
        Encl { lo: -r.clone(), hi: r }
    }

    /// Rounded OUTWARD to the grid `2^-bits`.
    pub fn round(&self, bits: u32) -> Encl {
        Encl { lo: down(&self.lo, bits), hi: up(&self.hi, bits) }
    }

    pub fn width(&self) -> Q {
        &self.hi - &self.lo
    }

    pub fn contains(&self, v: &Q) -> bool {
        &self.lo <= v && v <= &self.hi
    }

    /// `self < other`, decided only where the bounds separate.
    pub fn lt(&self, other: &Encl) -> Option<bool> {
        if self.hi < other.lo {
            Some(true)
        } else if self.lo >= other.hi {
            Some(false)
        } else {
            None
        }
    }

    pub fn neg(&self) -> Encl {
        Encl { lo: -self.hi.clone(), hi: -self.lo.clone() }
    }

    pub fn add(&self, o: &Encl) -> Encl {
        Encl { lo: &self.lo + &o.lo, hi: &self.hi + &o.hi }
    }

    pub fn sub(&self, o: &Encl) -> Encl {
        self.add(&o.neg())
    }

    pub fn mul(&self, o: &Encl) -> Encl {
        let p = [&self.lo * &o.lo, &self.lo * &o.hi, &self.hi * &o.lo, &self.hi * &o.hi];
        let [a, b, c, d] = p;
        Encl { lo: qmin(qmin(a.clone(), b.clone()), qmin(c.clone(), d.clone())), hi: qmax(qmax(a, b), qmax(c, d)) }
    }

    /// Times an exact rational.
    pub fn scale(&self, s: &Q) -> Encl {
        self.mul(&Encl::exact(s.clone()))
    }

    /// The square, tight where the interval straddles zero.
    pub fn square(&self) -> Encl {
        let (a, b) = (&self.lo * &self.lo, &self.hi * &self.hi);
        if self.lo.is_negative() && self.hi.is_positive() {
            Encl { lo: Q::zero(), hi: qmax(a, b) }
        } else {
            Encl { lo: qmin(a.clone(), b.clone()), hi: qmax(a, b) }
        }
    }

    /// `1 / self` — none where the interval holds zero.
    pub fn recip(&self) -> Option<Encl> {
        if self.lo <= Q::zero() && self.hi >= Q::zero() {
            return None;
        }
        Some(Encl { lo: self.hi.recip(), hi: self.lo.recip() })
    }

    pub fn div(&self, o: &Encl) -> Option<Encl> {
        Some(self.mul(&o.recip()?))
    }

    /// The largest magnitude it holds.
    pub fn abs_max(&self) -> Q {
        qmax(self.lo.abs(), self.hi.abs())
    }

    /// Intersected with `[lo, hi]` — what the function's range proves.
    fn clamp(&self, lo: &Q, hi: &Q) -> Encl {
        let a = qmax(self.lo.clone(), lo.clone());
        let b = qmin(self.hi.clone(), hi.clone());
        match a <= b {
            true => Encl { lo: a, hi: b },
            false => Encl { lo: lo.clone(), hi: hi.clone() },
        }
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
fn exp_point(v: &Q, w: u32) -> Encl {
    let half = Q::new(BigInt::one(), BigInt::from(2));
    let mut r = v.clone();
    let mut m = 0u32;
    while r.abs() > half {
        r /= q(2);
        m += 1;
    }
    let w2 = w + 24 + m;
    let tiny = stop_at(w2);
    let rr = Encl::exact(r);
    let mut term = Encl::exact(Q::one());
    let mut sum = term.clone();
    let mut k = 1i64;
    loop {
        term = term.mul(&rr).scale(&Q::new(BigInt::one(), BigInt::from(k))).round(w2);
        sum = sum.add(&term).round(w2);
        if term.abs_max() <= tiny {
            break;
        }
        k += 1;
    }
    let mut out = sum.add(&Encl::pm(term.abs_max())).round(w2);
    for _ in 0..m {
        out = out.square().round(w2);
    }
    out.round(w)
}

/// `e^x`: monotone, so its endpoints' enclosures bound it.
pub fn exp(x: &Encl, w: u32) -> Encl {
    Encl { lo: exp_point(&x.lo, w).lo, hi: exp_point(&x.hi, w).hi }
}

// ———————————————————————————————— ln ————————————————————————————————

/// `2·atanh(z) = 2 Σ z^{2j+1}/(2j+1)` for `0 ≤ z ≤ ⅓`. REMAINDER: after the
/// term `t_n`, the tail is `≤ t_n·z²/(1−z²) ≤ t_n/8`.
fn atanh2(z: &Q, w2: u32) -> Encl {
    let tiny = stop_at(w2);
    let z2 = Encl::exact(z * z);
    let mut pow = Encl::exact(z.clone());
    let mut sum = Encl::exact(Q::zero());
    let mut j = 0i64;
    loop {
        let term = pow.scale(&Q::new(BigInt::from(2), BigInt::from(2 * j + 1))).round(w2);
        sum = sum.add(&term).round(w2);
        if term.abs_max() <= tiny {
            return sum.add(&Encl::pm(term.abs_max())).round(w2);
        }
        pow = pow.mul(&z2).round(w2);
        j += 1;
    }
}

/// `ln 2 = 2·atanh(⅓)`.
pub fn ln2(w: u32) -> Encl {
    atanh2(&Q::new(BigInt::one(), BigInt::from(3)), w + 16).round(w)
}

/// `ln v` at an exact point `v > 0`: `v = 2^k·m`, `m ∈ [1, 2)`, and `ln m =
/// 2·atanh((m−1)/(m+1))` with `(m−1)/(m+1) < ⅓`.
fn ln_point(v: &Q, w: u32) -> Option<Encl> {
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
    Some(l2.scale(&q(k)).add(&lnm).round(w))
}

/// `ln x` for `x > 0`: monotone, by its endpoints.
pub fn ln(x: &Encl, w: u32) -> Option<Encl> {
    Some(Encl { lo: ln_point(&x.lo, w)?.lo, hi: ln_point(&x.hi, w)?.hi })
}

// ———————————————————————————————— π ————————————————————————————————

/// `atan(1/n) = Σ (−1)^j / ((2j+1)·n^{2j+1})`. REMAINDER: an alternating
/// series of decreasing terms — the tail is bounded by the next term,
/// which the last term bounds.
fn atan_inv(n: i64, w2: u32) -> Encl {
    let tiny = stop_at(w2);
    let nn = Encl::exact(Q::new(BigInt::one(), BigInt::from(n * n)));
    let mut pow = Encl::exact(Q::new(BigInt::one(), BigInt::from(n)));
    let mut sum = Encl::exact(Q::zero());
    let mut j = 0i64;
    loop {
        let mut term = pow.scale(&Q::new(BigInt::one(), BigInt::from(2 * j + 1)));
        if j % 2 == 1 {
            term = term.neg();
        }
        let term = term.round(w2);
        sum = sum.add(&term).round(w2);
        if term.abs_max() <= tiny {
            return sum.add(&Encl::pm(term.abs_max())).round(w2);
        }
        pow = pow.mul(&nn).round(w2);
        j += 1;
    }
}

/// `π = 16·atan(1/5) − 4·atan(1/239)` (Machin).
pub fn pi(w: u32) -> Encl {
    let w2 = w + 16;
    atan_inv(5, w2).scale(&q(16)).sub(&atan_inv(239, w2).scale(&q(4))).round(w)
}

// ———————————————————————————————— √ and powers ————————————————————————————————

/// `√x` for `x ≥ 0`: the integer square roots of the endpoints on the grid
/// `4^-w`, floored below and ceiled above — exact bounds.
pub fn sqrt(x: &Encl, w: u32) -> Option<Encl> {
    if x.lo.is_negative() {
        return None;
    }
    let g = Q::from_integer(pow2(2 * w));
    let lo_n = (&x.lo * &g).floor().to_integer();
    let hi_n = (&x.hi * &g).ceil().to_integer();
    let lo = lo_n.sqrt();
    let mut hi = hi_n.sqrt();
    if &hi * &hi < hi_n {
        hi += 1;
    }
    Some(Encl { lo: Q::new(lo, pow2(w)), hi: Q::new(hi, pow2(w)) })
}

/// `x^e` for `x > 0` and a rational `e`: `exp(e·ln x)`, monotone pieces.
pub fn powq(x: &Encl, e: &Q, w: u32) -> Option<Encl> {
    if !x.lo.is_positive() {
        return None;
    }
    let w2 = w + 32 + mag_bits(e);
    Some(exp(&ln(x, w2)?.scale(e).round(w2), w2).round(w))
}

// ———————————————————————————————— sin, cos ————————————————————————————————

/// `(sin x, cos x)`: reduced by the nearest multiple of `π/2`, then the
/// Taylor series of both carried on intervals. REMAINDER: truncated after
/// degree `N−1`, each series is off by at most `M^N/N!` (Lagrange), `M` the
/// reduced argument's largest magnitude.
pub fn sin_cos(x: &Encl, w: u32) -> (Encl, Encl) {
    let w2 = w + 32 + mag_bits(&x.abs_max());
    let (r, quadrant) = if x.abs_max() > Q::one() {
        let half_pi = pi(w2 + mag_bits(&x.abs_max())).scale(&Q::new(BigInt::one(), BigInt::from(2)));
        let mid = (&x.lo + &x.hi) / q(2);
        let k = (mid / &half_pi.lo).round().to_integer();
        let r = x.sub(&half_pi.scale(&Q::from_integer(k.clone()))).round(w2);
        (r, k.mod_floor(&BigInt::from(4)))
    } else {
        (x.clone(), BigInt::zero())
    };
    let tiny = stop_at(w2);
    let m = r.abs_max();
    let mut term = Encl::exact(Q::one());
    let mut bound = Q::one();
    let (mut s, mut c) = (Encl::exact(Q::zero()), Encl::exact(Q::zero()));
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
        term = term.mul(&r).scale(&Q::new(BigInt::one(), BigInt::from(k))).round(w2);
        bound = up(&(&bound * &m / q(k)), w2);
        if bound <= tiny {
            break;
        }
    }
    let rem = Encl::pm(bound);
    let (s, c) = (s.add(&rem), c.add(&rem));
    // sin(r + kπ/2), cos(r + kπ/2) by the quadrant `k mod 4`.
    let (s, c) = match u8::try_from(&quadrant) {
        Ok(1) => (c, s.neg()),
        Ok(2) => (s.neg(), c.neg()),
        Ok(3) => (c.neg(), s),
        _ => (s, c),
    };
    let one = Q::one();
    (s.clamp(&-one.clone(), &one).round(w), c.clamp(&-one.clone(), &one).round(w))
}

pub fn sin(x: &Encl, w: u32) -> Encl {
    sin_cos(x, w).0
}

pub fn cos(x: &Encl, w: u32) -> Encl {
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
/// `X₀ = w₂/8 + 2` keeps the series' least term below the grid.
pub fn lgamma(x: &Encl, w: u32) -> Option<Encl> {
    if !x.lo.is_positive() {
        return None;
    }
    let w2 = w + 32 + mag_bits(&x.hi);
    let x0 = q(i64::from(w2 / 8 + 2));
    let shift = if x.lo < x0 { (&x0 - &x.lo).ceil().to_integer() } else { BigInt::zero() };
    let steps = usize::try_from(&shift).ok()?;
    let mut prod = Encl::exact(Q::one());
    for i in 0..steps {
        prod = prod.mul(&x.add(&Encl::exact(q(i64::try_from(i).ok()?)))).round(w2);
    }
    let y = x.add(&Encl::exact(Q::from_integer(shift))).round(w2);
    let ln_y = ln(&y, w2)?;
    let two_pi = pi(w2).scale(&q(2));
    let half = Q::new(BigInt::one(), BigInt::from(2));
    let mut s = y
        .sub(&Encl::exact(half.clone()))
        .mul(&ln_y)
        .sub(&y)
        .add(&ln(&two_pi, w2)?.scale(&half))
        .round(w2);
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
        let term = ypow.scale(&(&b[2 * k] / &denom)).round(w2);
        s = s.add(&term).round(w2);
        // The next term's bound, at the smallest y.
        let next_den = Q::from_integer(BigInt::from((2 * k + 2) * (2 * k + 1)));
        let next = up(&(b[2 * k + 2].abs() / next_den * ypow.mul(&y2_recip).abs_max()), w2);
        if next <= tiny {
            s = s.add(&Encl::pm(next));
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
    Some(out.round(w))
}

// ———————————————————————————————— P(a, x), erf ————————————————————————————————

/// `P(a, x)` at exact points `a > 0`, `x ≥ 0`: `x^a e^{-x} / Γ(a+1) · S`, `S =
/// Σ_{n≥0} x^n / ((a+1)…(a+n))`. REMAINDER: once `ρ = x/(a+n+1) ≤ ½`, the
/// tail after the term `t_n` is `≤ t_n·ρ/(1−ρ) ≤ t_n`, all terms positive.
fn gamma_p_point(a: &Q, x: &Q, w: u32) -> Option<Encl> {
    if !a.is_positive() || x.is_negative() {
        return None;
    }
    // The series takes O(x) terms: past the cap it is refused, never run
    // unbounded (the coordinator's ruling: the funnel's x stay well under).
    if x > &q(P_SERIES_CAP) {
        return None;
    }
    if x.is_zero() {
        return Some(Encl::exact(Q::zero()));
    }
    let w2 = w + 40 + 2 * mag_bits(x);
    let tiny = stop_at(w2);
    let half = Q::new(BigInt::one(), BigInt::from(2));
    let mut term = Encl::exact(Q::one());
    let mut sum = term.clone();
    let mut n = 1i64;
    loop {
        term = term.scale(&(x / (a + q(n)))).round(w2);
        sum = sum.add(&term).round(w2);
        let rho = x / (a + q(n + 1));
        if rho <= half && term.hi <= &sum.lo * &tiny {
            sum = sum.add(&Encl { lo: Q::zero(), hi: term.hi.clone() });
            break;
        }
        n += 1;
    }
    let a1 = Encl::exact(a + Q::one());
    let log_pre = ln_point(x, w2)?.scale(a).sub(&Encl::exact(x.clone())).sub(&lgamma(&a1, w2)?);
    let p = exp(&log_pre.round(w2), w2).mul(&sum);
    Some(p.clamp(&Q::zero(), &Q::one()).round(w))
}

/// The regularized lower incomplete gamma `P(a, x)` for `a > 0`, `x ≥ 0`:
/// increasing in `x` and decreasing in `a`, so bounded by its corners.
pub fn gamma_p(a: &Encl, x: &Encl, w: u32) -> Option<Encl> {
    if !a.lo.is_positive() || x.lo.is_negative() {
        return None;
    }
    let lo = gamma_p_point(&a.hi, &x.lo, w)?.lo;
    let hi = gamma_p_point(&a.lo, &x.hi, w)?.hi;
    Encl::new(lo, hi)
}

/// THE CAP on `P`'s series argument: `x ≤ 10⁴`, past which `P` is refused.
pub const P_SERIES_CAP: i64 = 10_000;

/// `erf v = sign(v)·P(½, v²)` at an exact point. Past the series cap (`v² >
/// 10⁴`), `1 − erf|v| = erfc|v| ≤ e^{−v²}/(|v|√π) < e^{−10⁴} < 2^{−w}` for
/// every `w < 14 000`: the enclosure `[1 − 2^{−w}, 1]`, proved, not run.
fn erf_point(v: &Q, w: u32) -> Option<Encl> {
    let v2 = v * v;
    if v2 > q(P_SERIES_CAP) {
        if w >= 14_000 {
            return None;
        }
        let near = Encl { lo: Q::one() - Q::new(BigInt::one(), pow2(w)), hi: Q::one() };
        return Some(if v.is_negative() { near.neg() } else { near });
    }
    let p = gamma_p_point(&Q::new(BigInt::one(), BigInt::from(2)), &v2, w)?;
    Some(if v.is_negative() { p.neg() } else { p })
}

/// `erf x`: increasing, by its endpoints.
pub fn erf(x: &Encl, w: u32) -> Option<Encl> {
    Encl::new(erf_point(&x.lo, w)?.lo, erf_point(&x.hi, w)?.hi)
}

// ———————————————————————————————— sinh, cosh, logistic ————————————————————————————————

/// `sinh x = (e^x − e^{−x})/2`: increasing, by its endpoints.
pub fn sinh(x: &Encl, w: u32) -> Encl {
    let w2 = w + 8;
    let at = |v: &Q| exp_point(v, w2).sub(&exp_point(&-v.clone(), w2)).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    Encl { lo: at(&x.lo).lo, hi: at(&x.hi).hi }.round(w)
}

/// `cosh x = (e^x + e^{−x})/2`: even, least at zero, increasing in `|x|`.
pub fn cosh(x: &Encl, w: u32) -> Encl {
    let w2 = w + 8;
    let at = |v: &Q| exp_point(v, w2).add(&exp_point(&-v.clone(), w2)).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    let far = at(&x.abs_max());
    let near = if x.lo.is_negative() && x.hi.is_positive() {
        Q::one()
    } else {
        at(&qmin(x.lo.abs(), x.hi.abs())).lo
    };
    Encl { lo: near, hi: far.hi }.round(w)
}

/// The logistic `1/(1 + e^{−x})`: increasing, by its endpoints.
pub fn logistic(x: &Encl, w: u32) -> Option<Encl> {
    let w2 = w + 8;
    let at = |v: &Q| Encl::exact(Q::one()).add(&exp_point(&-v.clone(), w2)).recip();
    Some(Encl { lo: at(&x.lo)?.lo, hi: at(&x.hi)?.hi }.round(w))
}

// ———————————————————————————————— atan, acos, tan ————————————————————————————————

/// `atan v` for `|v| ≤ 1`, by Euler's series `atan v = v/(1+v²) · Σ_k
/// ∏_{j≤k} (2j/(2j+1)) · y^k`, `y = v²/(1+v²) ≤ ½`. REMAINDER: the terms are
/// positive and each ratio is `≤ y ≤ ½`, so the tail after the term `t_n` is
/// `≤ t_n·y/(1−y) ≤ t_n`.
fn atan_small(v: &Q, w2: u32) -> Encl {
    let tiny = stop_at(w2);
    let v2 = v * v;
    let y = &v2 / (Q::one() + &v2);
    let lead = v / (Q::one() + &v2);
    let mut term = Encl::exact(Q::one());
    let mut sum = term.clone();
    let mut k = 1i64;
    loop {
        term = term.scale(&(Q::new(BigInt::from(2 * k), BigInt::from(2 * k + 1)) * &y)).round(w2);
        sum = sum.add(&term).round(w2);
        if term.abs_max() <= tiny {
            break;
        }
        k += 1;
    }
    sum.add(&Encl { lo: Q::zero(), hi: term.abs_max() }).scale(&lead).round(w2)
}

/// `atan v` at an exact point: for `|v| > 1`, `sign(v)·π/2 − atan(1/v)`.
fn atan_point(v: &Q, w: u32) -> Encl {
    let w2 = w + 24;
    if v.abs() <= Q::one() {
        return atan_small(v, w2).round(w);
    }
    let half_pi = pi(w2).scale(&Q::new(BigInt::one(), BigInt::from(2)));
    let inner = atan_small(&v.recip(), w2);
    let out = if v.is_positive() { half_pi.sub(&inner) } else { half_pi.neg().sub(&inner) };
    out.round(w)
}

/// `atan x`: increasing, by its endpoints.
pub fn atan(x: &Encl, w: u32) -> Encl {
    Encl { lo: atan_point(&x.lo, w).lo, hi: atan_point(&x.hi, w).hi }
}

/// `acos v` at an exact point `−1 ≤ v ≤ 1`: `acos v = 2·atan(√((1−v)/(1+v)))`;
/// `acos(−1) = π`, `acos 1 = 0`.
fn acos_point(v: &Q, w: u32) -> Option<Encl> {
    let w2 = w + 24;
    let one = Q::one();
    if v > &one || v < &-one.clone() {
        return None;
    }
    if v == &-one.clone() {
        return Some(pi(w));
    }
    let ratio = (&one - v) / (&one + v);
    let root = sqrt(&Encl::exact(ratio), w2)?;
    let a = Encl { lo: atan_point(&root.lo, w2).lo, hi: atan_point(&root.hi, w2).hi };
    Some(a.scale(&q(2)).round(w))
}

/// `acos x` for `x ⊂ [−1, 1]`: decreasing, by its endpoints.
pub fn acos(x: &Encl, w: u32) -> Option<Encl> {
    Encl::new(acos_point(&x.hi, w)?.lo, acos_point(&x.lo, w)?.hi)
}

/// `tan x = sin x / cos x` — none where the enclosure of `cos x` holds zero.
pub fn tan(x: &Encl, w: u32) -> Option<Encl> {
    let (s, c) = sin_cos(x, w + 16);
    Some(s.div(&c)?.round(w))
}

// ———————————————————————————————— the Planck tail ————————————————————————————————

/// `15/π⁴`: the black-body normalization.
fn planck_norm(w2: u32) -> Option<Encl> {
    let p = pi(w2);
    let p4 = p.square().square();
    Some(p4.recip()?.scale(&q(15)))
}

/// `F(v)` for `v ≥ 1`: `15/π⁴ · Σ_{n≥1} e^{−nv}·p_n(v)`, `p_n(v) = v³/n + 3v²/n² +
/// 6v/n³ + 6/n⁴`. REMAINDER: every term is positive and `p_n` is decreasing in
/// `n`, so the tail after `N` terms is `Σ_{n>N} e^{−nv}p_n ≤ p_{N+1}·Σ_{n>N}
/// e^{−nv} = e^{−(N+1)v}·p_{N+1} / (1 − e^{−v})` — the next term over `1 − e^{−v}`.
fn planck_far(v: &Q, w2: u32) -> Option<Encl> {
    let tiny = stop_at(w2);
    let e = exp_point(&-v.clone(), w2);
    let gap = Encl::exact(Q::one()).sub(&e).recip()?;
    let p = |n: i64| -> Q {
        let n = q(n);
        v * v * v / &n + q(3) * v * v / (&n * &n) + q(6) * v / (&n * &n * &n) + q(6) / (&n * &n * &n * &n)
    };
    let mut en = e.clone();
    let mut sum = Encl::exact(Q::zero());
    let mut n = 1i64;
    loop {
        sum = sum.add(&en.scale(&p(n))).round(w2);
        en = en.mul(&e).round(w2);
        let tail = en.scale(&p(n + 1)).mul(&gap);
        if tail.hi <= tiny {
            sum = sum.add(&Encl { lo: Q::zero(), hi: tail.hi });
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
fn planck_near(v: &Q, w2: u32) -> Option<Encl> {
    let tiny = stop_at(w2);
    let sixth = v / q(6);
    let mut b = bernoulli(32);
    let mut integral = Encl::exact(Q::zero());
    let mut vpow = v * v * v; // v^{k+3}
    let mut fact = BigInt::one(); // k!
    let mut k = 0usize;
    loop {
        if b.len() <= k {
            b = bernoulli(2 * k + 16);
        }
        let kq = Q::from_integer(BigInt::from(k + 3)) * Q::from_integer(fact.clone());
        integral = integral.add(&Encl::exact(&b[k] * &vpow / kq)).round(w2);
        k += 1;
        fact *= BigInt::from(k);
        vpow *= v;
        // The tail after K = k terms.
        let mut bound = q(4) * v * v * v / (Q::one() - &sixth);
        for _ in 0..=k {
            bound *= &sixth;
        }
        if up(&bound, w2) <= tiny {
            integral = integral.add(&Encl::pm(up(&bound, w2)));
            break;
        }
    }
    Some(Encl::exact(Q::one()).sub(&integral.mul(&planck_norm(w2)?)))
}

/// THE PLANCK TAIL `F(x) = 15/π⁴ · ∫_x^∞ t³/(e^t − 1) dt`: the fraction of
/// black-body energy above `x = hν/kT`. `F(0) = 1` exactly; `F` decreasing, so
/// bounded by its endpoints; `x ≥ 0`.
pub fn planck_tail(x: &Encl, w: u32) -> Option<Encl> {
    if x.lo.is_negative() {
        return None;
    }
    let w2 = w + 32 + mag_bits(&x.hi);
    let at = |v: &Q| -> Option<Encl> {
        if v.is_zero() {
            return Some(Encl::exact(Q::one()));
        }
        if v < &Q::one() {
            planck_near(v, w2)
        } else {
            planck_far(v, w2)
        }
    };
    let out = Encl::new(at(&x.hi)?.lo, at(&x.lo)?.hi)?;
    Some(out.clamp(&Q::zero(), &Q::one()).round(w))
}

// ———————————————————————————————— the Poisson quantile ————————————————————————————————

/// THE INVERSE POISSON CDF: the least `k` with `Σ_{j≤k} e^{−λ} λ^j/j! ≥ u`, for
/// an exact `λ > 0` and `0 ≤ u < 1`. Each step's CDF is an enclosure; a step
/// is decided only where its bounds separate from `u`, the precision raised
/// where they do not, up to `w + 256` bits — beyond that `u` sits on a step
/// edge to that precision, and the quantile is refused rather than guessed.
pub fn poisson_quantile(lambda: &Q, u: &Q, w: u32) -> Option<u64> {
    if !lambda.is_positive() || u.is_negative() || u >= &Q::one() {
        return None;
    }
    let mut wp = w + 32 + mag_bits(lambda);
    while wp <= w + 256 + mag_bits(lambda) {
        let mut term = exp_point(&-lambda.clone(), wp);
        let mut cdf = term.clone();
        let mut k = 0u64;
        let decided = loop {
            match (cdf.lo >= *u, cdf.hi < *u) {
                (true, _) => break Some(k),
                (_, true) => {}
                _ => break None,
            }
            k = k.checked_add(1)?;
            term = term.scale(&(lambda / Q::from_integer(BigInt::from(k)))).round(wp);
            cdf = cdf.add(&term).round(wp);
        };
        if decided.is_some() {
            return decided;
        }
        wp += 64;
    }
    None
}
