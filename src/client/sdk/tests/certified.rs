// 78.130 — CERTIFIED REALS: every enclosure holds the true value — pinned
// against known constants to 2^-60 — inverse pairs close, P is monotone and
// tends to one, and the same inputs give the same enclosure on any thread.

use crate::client::sdk::certified::*;
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

/// The enclosure holds the reference (known to far more digits than `W`)
/// and is no wider than `2^-60`.
fn pins(e: &Encl, reference: &str, name: &str) {
    let r = dec(reference);
    let slack = Q::new(BigInt::one(), BigInt::from(10).pow(45));
    assert!(e.lo <= &r + &slack && &r - &slack <= e.hi, "{name}: {e:?} misses {reference}");
    assert!(e.width() <= Q::new(BigInt::one(), BigInt::one() << 60u32), "{name}: wider than 2^-60");
}

#[test]
fn the_constants_are_enclosed_to_two_to_the_minus_sixty() {
    pins(&pi(W), "3.14159265358979323846264338327950288419716939937510", "π");
    pins(&exp(&point(Q::one()), W), "2.71828182845904523536028747135266249775724709369995", "e");
    pins(&ln2(W), "0.69314718055994530941723212145817656807550013436025", "ln 2");
    pins(&sqrt(&point(q(2, 1)), W).expect("√2"), "1.41421356237309504880168872420969807856967187537694", "√2");
    pins(&sin(&point(Q::one()), W), "0.84147098480789650665250232163029899962256306079837", "sin 1");
    pins(&cos(&point(Q::one()), W), "0.54030230586813971740093660744297660373231042061792", "cos 1");
    pins(&lgamma(&point(q(1, 2)), W).expect("lnΓ(½)"), "0.57236494292470008707171367567652935582364740645765", "lnΓ(½)");
    pins(&erf(&point(Q::one()), W).expect("erf 1"), "0.84270079294971486934122063508260925929606699796630", "erf 1");
    pins(&gamma_p(&point(Q::one()), &point(Q::one()), W).expect("P(1,1)"), "0.63212055882855767840447622983853913255418886896823", "P(1, 1) = 1 − 1/e");
}

#[test]
fn inverse_pairs_close_and_identities_hold() {
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
    let e1 = erf(&point(q(3, 4)), W).expect("");
    assert_eq!(erf(&point(q(-3, 4)), W).expect(""), e1.neg(), "erf(−x) = −erf(x)");
    assert_eq!(point(q(1, 3)).lt(&point(q(1, 2))), Some(true));
    assert_eq!(pi(W).lt(&pi(W)), None, "undecided where the bounds overlap");
}

#[test]
fn the_incomplete_gamma_is_monotone_and_tends_to_one() {
    let a = point(q(2, 1));
    let low = gamma_p(&a, &point(Q::one()), W).expect("");
    let high = gamma_p(&a, &point(q(3, 1)), W).expect("");
    assert_eq!(low.lt(&high), Some(true), "P increasing in x");
    let far = gamma_p(&point(Q::one()), &point(q(40, 1)), W).expect("");
    assert!(far.lo > Q::one() - Q::new(BigInt::one(), BigInt::one() << 50u32), "P(1, 40) = 1 − e^-40");
    assert!(far.hi <= Q::one());
    assert_eq!(gamma_p(&a, &point(Q::zero()), W).expect(""), point(Q::zero()));
}

#[test]
fn the_same_inputs_give_the_same_enclosure_on_any_thread() {
    let here = (pi(W), erf(&point(q(5, 7)), W), lgamma(&point(q(3, 2)), W));
    let there = std::thread::spawn(|| (pi(W), erf(&point(q(5, 7)), W), lgamma(&point(q(3, 2)), W))).join().expect("thread");
    assert_eq!(here, there, "bit for bit");
}
