// 78.130 — CERTIFIED REALS: every enclosure holds the true value — pinned
// against known constants to 2^-60 — inverse pairs close, P is monotone and
// tends to one, and the same inputs give the same enclosure on any thread.

use crate::core::certified::*;
use num_bigint::BigInt;
use num_rational::BigRational as Q;
use num_traits::{One, Zero};

const W: u32 = 64;

/// A decimal literal, exactly.
fn dec(s: &str) -> Q {
    let (int, frac) = match s.split_once('.') {
        Some((i, f)) => (i, f),
        None => (s, ""),
    };
    let digits: BigInt = format!("{int}{frac}").parse().expect("digits");
    let scale = BigInt::from(10).pow(u32::try_from(frac.len()).expect("short"));
    Q::new(digits, scale)
}

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn point(v: Q) -> Encl {
    Encl::exact(v)
}

/// A token wide enough for every series these tests run.
fn token() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 24)
}

/// The enclosure holds the reference (known to far more digits than `W`)
/// and is no wider than `2^-60`.
fn pins(e: &Encl, reference: &str, name: &str) {
    let r = dec(reference);
    let slack = Q::new(BigInt::one(), BigInt::from(10).pow(45));
    assert!(e.lo <= &r + &slack && &r - &slack <= e.hi, "{name}: {e:?} misses {reference}");
    assert!(e.width() <= Q::new(BigInt::one(), BigInt::one() << 60u32), "{name}: wider than 2^-60");
}

/// The constants (π, e, ln 2, √2, sin 1, cos 1, lnΓ(½), erf 1, P(1, 1)) are
/// enclosed to 2^-60, each enclosure holding the reference value.
#[test]
fn enclosure_pins_constants_within_two_pow_minus_sixty() {
    pins(&pi(W), "3.14159265358979323846264338327950288419716939937510", "π");
    pins(&exp(&point(Q::one()), W), "2.71828182845904523536028747135266249775724709369995", "e");
    pins(&ln2(W), "0.69314718055994530941723212145817656807550013436025", "ln 2");
    pins(&sqrt(&point(q(2, 1)), W).expect("√2"), "1.41421356237309504880168872420969807856967187537694", "√2");
    pins(&sin(&point(Q::one()), W), "0.84147098480789650665250232163029899962256306079837", "sin 1");
    pins(&cos(&point(Q::one()), W), "0.54030230586813971740093660744297660373231042061792", "cos 1");
    pins(&lgamma(&point(q(1, 2)), W).expect("lnΓ(½)"), "0.57236494292470008707171367567652935582364740645765", "lnΓ(½)");
    pins(&erf(&point(Q::one()), W, &mut token()).expect("erf 1"), "0.84270079294971486934122063508260925929606699796630", "erf 1");
    pins(
        &gamma_p(&point(Q::one()), &point(Q::one()), W, &mut token()).expect("P(1,1)"),
        "0.63212055882855767840447622983853913255418886896823",
        "P(1, 1) = 1 − 1/e",
    );
}

/// Inverse pairs close (`exp ∘ ln`, `ln ∘ exp`) and the identities hold:
/// `Γ(1) = Γ(2) = 1`, `Γ(x+1) = xΓ(x)`, `sin² + cos² = 1`, `sin π = 0` and
/// erf is odd.
#[test]
fn exp_inverts_ln_and_satisfies_identities() {
    let v = q(7, 3);
    assert!(exp(&ln(&point(v.clone()), W).expect("ln"), W).contains(&v), "exp ∘ ln");
    let u = q(-5, 2);
    assert!(ln(&exp(&point(u.clone()), W), W).expect("ln").contains(&u), "ln ∘ exp");
    assert!(lgamma(&point(Q::one()), W).expect("lnΓ 1").contains(&Q::zero()), "Γ(1) = 1");
    assert!(lgamma(&point(q(2, 1)), W).expect("lnΓ 2").contains(&Q::zero()), "Γ(2) = 1");
    // lnΓ(x+1) − lnΓ(x) = ln x.
    let x = q(17, 4);
    let step = lgamma(&point(&x + Q::one()), W).expect("").sub(&lgamma(&point(x.clone()), W).expect(""));
    let lnx = ln(&point(x), W).expect("");
    assert!(step.lt(&lnx) != Some(true) && lnx.lt(&step) != Some(true), "Γ(x+1) = xΓ(x)");
    // sin² + cos² = 1, through the reduction by π/2.
    let (s, c) = sin_cos(&point(q(10, 1)), W);
    assert!(s.square().add(&c.square()).contains(&Q::one()), "sin² + cos² = 1 at 10");
    assert!(sin(&pi(W), W).contains(&Q::zero()), "sin π = 0");
    // erf is odd.
    let e1 = erf(&point(q(3, 4)), W, &mut token()).expect("");
    assert_eq!(erf(&point(q(-3, 4)), W, &mut token()).expect(""), e1.neg(), "erf(−x) = −erf(x)");
    assert_eq!(point(q(1, 3)).lt(&point(q(1, 2))), Some(true));
    assert_eq!(pi(W).lt(&pi(W)), None, "undecided where the bounds overlap");
}

/// The incomplete gamma `P` is monotone in `x` and tends to one, and erf far
/// out is proved by its own tail bound, never run.
#[test]
fn gamma_p_rises_to_one_and_erf_reads_tail() {
    let a = point(q(2, 1));
    let low = gamma_p(&a, &point(Q::one()), W, &mut token()).expect("");
    let high = gamma_p(&a, &point(q(3, 1)), W, &mut token()).expect("");
    assert_eq!(low.lt(&high), Some(true), "P increasing in x");
    let far = gamma_p(&point(Q::one()), &point(q(40, 1)), W, &mut token()).expect("");
    assert!(far.lo > Q::one() - Q::new(BigInt::one(), BigInt::one() << 50u32), "P(1, 40) = 1 − e^-40");
    assert!(far.hi <= Q::one());
    assert_eq!(gamma_p(&a, &point(Q::zero()), W, &mut token()).expect(""), point(Q::zero()));
    let big = erf(&point(q(101, 1)), W, &mut crate::physics::Physics::default().stroke_n(0)).expect("erf by its tail, no term read");
    assert!(big.contains(&Q::one()) && big.width() <= Q::new(BigInt::one(), BigInt::one() << 64u32), "erf 101 within 2^-64 of one");
    assert_eq!(erf(&point(q(-101, 1)), W, &mut token()).expect(""), big.neg());
}

/// D4.8 (a) — THE SERIES' TERMS ARE AT MOST THE CLOSED FORM: `P(1, 3)` at
/// `w₂ = W + 40 + 2·bits(3)` spends at most `max(0, ⌈2·3 − 1⌉ − 1) + w₂ − 8`
/// reads.
#[test]
fn gamma_p_terms_equal_closed_form() {
    let w2 = W + 40 + 2 * 2;
    let bound = gamma_p_terms(&Q::one(), &q(3, 1), w2).expect("counted");
    assert_eq!(bound, 4 + u64::from(w2) - 8);
    let mut t = token();
    let before = t.remaining();
    gamma_p(&point(Q::one()), &point(q(3, 1)), W, &mut t).expect("read");
    let spent = u64::try_from(before - t.remaining()).expect("small");
    assert!(spent >= 1 && spent <= bound, "{spent} of at most {bound}");
}

/// D4.8 (a) — A SHORT TOKEN IS REFUSED UP FRONT: fewer reads than the exact
/// bound refuses before a term is read, naming the need; no cap stands in
/// `x` (a thousand runs where ten thousand was the wall's tenth).
#[test]
fn gamma_p_refuses_short_budget_up_front() {
    let mut t = crate::physics::Physics::default().stroke_n(10);
    let reply = gamma_p(&point(Q::one()), &point(q(3, 1)), W, &mut t);
    assert!(matches!(reply, Err(SeriesRefusal::Unmetered { needed }) if needed > 10), "{reply:?}");
    assert_eq!(t.remaining(), 10, "no read spent: refused up front");
}

/// D4.8 (e) — `P(1, 1000)` IS INFORMATIVE: its prefactor `≈ 2⁻¹⁴³³` is taken on
/// a grid fine enough for the sum it multiplies, so the enclosure holds
/// `1 − e^{−1000}` and is within the result's own grid, never `[0, 1]`.
#[test]
fn gamma_p_width_is_within_grid_at_large_x() {
    let p = gamma_p(&point(Q::one()), &point(q(1000, 1)), W, &mut token()).expect("read");
    assert!(p.width() <= Q::new(BigInt::from(4), BigInt::one() << W), "width {} within 2^(2−w)", p.width());
    assert!(p.lo > Q::one() - Q::new(BigInt::one(), BigInt::one() << 60u32) && p.hi <= Q::one(), "holds 1 − e^-1000");
}

/// D4.8 (b) — THE ERF TAIL IS DERIVED, NOT CAPPED: `erf 101` at `w = 14 600`,
/// where the former `14 000` refused it, is read off its own bound.
#[test]
fn erf_tail_branch_is_derived_without_cap() {
    let w = 14_600;
    let e = erf(&point(q(101, 1)), w, &mut crate::physics::Physics::default().stroke_n(0)).expect("by its tail");
    assert!(e.contains(&Q::one()) && e.width() <= Q::new(BigInt::one(), BigInt::one() << w));
}

/// D4.8 (c) — A SPENT TOKEN ANSWERS THE SECTOR: `λ = 3`, `u` on `F(2)`'s
/// enclosure at the first round, a token too short for a second; the
/// quantile is 2 or 3, never a guess.
#[test]
fn poisson_quantile_answers_sector_on_spent_budget() {
    let l = q(3, 1);
    let w = 8;
    let f2 = exp(&point(-l.clone()), 200).scale(&q(17, 2));
    let u = f2.lo;
    let mut t = crate::physics::Physics::default().stroke_n(3);
    match poisson_quantile(&l, &u, w, &mut t) {
        Some(Quantile::Sector { lo, hi }) => assert_eq!((lo, hi), (2, 3)),
        Some(Quantile::At(k)) => assert!(k == 2 || k == 3, "decided {k}"),
        None => panic!("a step was undecided: its sector is the answer"),
    }
}

/// D4.8 (c) — A LARGE λ IS DECIDED: `λ = 500` at `w = 64`, the leading term
/// `e^{−500}` resolved relative to its own size, the median read near λ.
#[test]
fn poisson_quantile_decides_at_large_lambda() {
    let q500 = poisson_quantile(&q(500, 1), &q(1, 2), W, &mut token());
    assert!(matches!(q500, Some(Quantile::At(k)) if (480..=520).contains(&k)), "{q500:?}");
}

/// D4.8 (e) — `exp(100)` IS WITHIN ITS GRID: the value's own magnitude is in
/// the guard, so its width is the result's grid, not `2^{144 − 88}`.
#[test]
fn exp_width_is_within_grid_at_positive_argument() {
    let e = exp(&point(q(100, 1)), W);
    assert!(e.width() <= Q::new(BigInt::from(2), BigInt::one() << W), "width {}", e.width());
}

/// The same inputs give the same enclosure on any thread.
#[test]
fn enclosure_ignores_thread() {
    let here = (pi(W), erf(&point(q(5, 7)), W, &mut token()), lgamma(&point(q(3, 2)), W));
    let there = std::thread::spawn(|| (pi(W), erf(&point(q(5, 7)), W, &mut token()), lgamma(&point(q(3, 2)), W))).join().expect("thread");
    assert_eq!(here, there, "bit for bit");
}

/// Whether `e` lies strictly inside `(lo, hi)`.
fn within(e: &Encl, lo: Q, hi: Q) -> bool {
    e.lo > lo && e.hi < hi
}

/// The hyperbolic and inverse circular functions are enclosed: sinh, cosh
/// and tan pin the reference values, atan and acos meet the exact multiples of
/// π, and acos refuses outside [−1, 1].
#[test]
fn enclosure_pins_hyperbolic_and_inverse_circular_references() {
    pins(&sinh(&point(Q::one()), W), "1.17520119364380145688238185059560081515571798133409", "sinh 1");
    pins(&cosh(&point(Q::one()), W), "1.54308063481524377847790562075706168260152911236586", "cosh 1");
    pins(&tan(&point(Q::one()), W).expect("tan 1"), "1.55740772465490223050697480745836017308725077238152", "tan 1");
    assert!(cosh(&point(Q::zero()), W).contains(&Q::one()), "cosh 0 = 1");
    let quarter_pi = pi(W).scale(&q(1, 4));
    let a1 = atan(&point(Q::one()), W);
    assert!(a1.lt(&quarter_pi).is_none() && quarter_pi.lt(&a1).is_none(), "atan 1 = π/4");
    let half_pi = pi(W).scale(&q(1, 2));
    let pair = atan(&point(q(2, 1)), W).add(&atan(&point(q(1, 2)), W));
    assert!(pair.lt(&half_pi).is_none() && half_pi.lt(&pair).is_none(), "atan 2 + atan ½ = π/2");
    let a0 = acos(&point(Q::zero()), W).expect("acos 0");
    assert!(a0.lt(&half_pi).is_none() && half_pi.lt(&a0).is_none(), "acos 0 = π/2");
    let third_pi = pi(W).scale(&q(1, 3));
    let ah = acos(&point(q(1, 2)), W).expect("acos ½");
    assert!(ah.lt(&third_pi).is_none() && third_pi.lt(&ah).is_none(), "acos ½ = π/3");
    assert_eq!(acos(&point(q(-1, 1)), W), Some(pi(W)), "acos(−1) = π");
    assert!(acos(&point(q(3, 2)), W).is_none(), "outside [−1, 1]");
    assert!(logistic(&point(Q::zero()), W).expect("").contains(&q(1, 2)), "logistic 0 = ½");
}

/// The Planck tail is one at zero exactly, and its near and far series
/// meet at one.
#[test]
fn planck_tail_is_one_at_zero_and_agrees_with_series() {
    assert_eq!(planck_tail(&point(Q::zero()), W), Some(point(Q::one())), "F(0) = 1 exactly");
    // Small x: 1 − 15/π⁴·(x³/3 − x⁴/8 + …), at x = 1/10 ≈ 0.999953.
    let small = planck_tail(&point(q(1, 10)), W).expect("");
    assert!(within(&small, q(99994, 100000), q(99996, 100000)), "{small:?}");
    // The near series (x < 1) and the far series (x ≥ 1) agree: F' at 1 is
    // −15/π⁴ · 1/(e − 1) ≈ −0.0896, so F(0.99) − F(1) ≈ 0.000896.
    let below = planck_tail(&point(q(99, 100)), W).expect("");
    let at = planck_tail(&point(Q::one()), W).expect("");
    assert!(within(&below.sub(&at), q(88, 100000), q(91, 100000)), "the two series meet at 1");
    // Large x: the first term 15/π⁴·e^{−x}(x³+3x²+6x+6) dominates.
    let far = planck_tail(&point(q(30, 1)), W).expect("");
    // The reference at 128 bits: e^{-30}'s grid, scaled by ~4600, would
    // otherwise swamp what it checks.
    let first = exp(&point(q(-30, 1)), 128).scale(&q(27000 + 2700 + 180 + 6, 1)).scale(&q(15, 1)).mul(&pi(128).square().square().recip().expect(""));
    // The rest is the second term, ~e^{-30}/2 ≈ 2^-44 of the first; the grid
    // adds 2^-64 absolute (F(30) ≈ 4·10⁻¹⁰, so absolute, not relative).
    let gap = far.sub(&first).abs_max();
    let allowed = &first.hi / Q::from_integer(BigInt::one() << 40u32) + Q::new(BigInt::one(), BigInt::one() << 60u32);
    assert!(gap < allowed, "the first term, to 2^-40 of itself and the grid");
    let order = planck_tail(&Encl::new(q(2, 1), q(3, 1)).expect(""), W).expect("");
    assert!(order.lo < order.hi && order.lo > Q::zero(), "an interval of x");
}

/// The Poisson quantile reads the CDF's steps, and refuses `u = 1`.
#[test]
fn poisson_quantile_reads_cdf_steps() {
    // λ = 3: CDF(0) ≈ 0.0498, CDF(1) ≈ 0.1991, CDF(2) ≈ 0.4232, CDF(3) ≈ 0.6472, CDF(5) ≈ 0.9161.
    let l = q(3, 1);
    let at = |u: &Q| poisson_quantile(&l, u, W, &mut token());
    assert_eq!(at(&Q::zero()), Some(Quantile::At(0)));
    assert_eq!(at(&q(1, 10)), Some(Quantile::At(1)));
    assert_eq!(at(&q(1, 2)), Some(Quantile::At(3)));
    assert_eq!(at(&q(9, 10)), Some(Quantile::At(5)));
    assert_eq!(at(&Q::one()), None, "u < 1");
}

/// D1.2 PHASE A — THE REFINEMENT ENDS ON A RATIONAL RADIUS: `π` refined past
/// every rational it is set against, each on its true side — `3` and
/// `333/106` below, `22/7` and `355/113` above (the last within `3·10⁻⁷` of
/// it) — and an exact value meets itself as `Equal`.
#[test]
fn separates_terminates_on_rational_radius() {
    use std::cmp::Ordering::{Equal, Greater, Less};
    for (r, side) in [(q(3, 1), Less), (q(333, 106), Less), (q(22, 7), Greater), (q(355, 113), Greater)] {
        assert_eq!(separates(&r, pi), Some(side), "{r}");
    }
    assert_eq!(separates(&q(1, 3), |_| Encl::exact(q(1, 3))), Some(Equal));
}
