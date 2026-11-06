// LAW-COORDINATE fixture 11b (proposal 78.173 §5, read and stated by
// netstratum 2026-11-06): the composition of turnings by the Cayley form,
// exact in the even subalgebra of `Cl(p, q)`. Plane `k`'s positive unit
// `e₊ₖ` (squaring to +1) is base axis `2k − 1`, its negative unit `e₋ₖ`
// (squaring to −1) base axis `2k`. Every motion here is claimed both ways:
// `motion(generator(R)) = R` and `R·R̃ = 1`.

use crate::turning::reading::{composed, degrees, on_plane, Regime};
use crate::turning::*;
use crate::Q;
use algebra::{base_axis, Clifford};

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// A frame unit: `(+1, k)` is `e₊ₖ`, `(-1, k)` is `e₋ₖ`.
fn unit(square: i8, k: usize) -> usize {
    match square {
        1 => base_axis(2 * k - 1),
        _ => base_axis(2 * k),
    }
    .expect("the algebra holds the unit")
}

/// The signature of planes `1..=3`: each negative unit.
fn signature() -> usize {
    (1..=3).map(|k| unit(-1, k)).fold(0, |s, g| s | g)
}

/// `c` times the product of the units, in the order written.
fn word(c: Q, units: &[(i8, usize)]) -> Clifford {
    let sig = signature();
    units.iter().fold(Clifford::blade(sig, 0, c).expect("a scalar"), |w, (s, k)| {
        w.product(&Clifford::blade(sig, unit(*s, *k), q(1, 1)).expect("a unit")).expect("one signature")
    })
}

/// The sum of the words.
fn sum(words: &[Clifford]) -> Clifford {
    words.iter().fold(word(q(0, 1), &[]), |s, w| s.sum(w).expect("one signature"))
}

// The units the fixtures name: `e₊₁`, `e₊₂`, `e₊₃`, `e₋₁`, `e₋₂`.
const P1: (i8, usize) = (1, 1);
const P2: (i8, usize) = (1, 2);
const P3: (i8, usize) = (1, 3);
const N1: (i8, usize) = (-1, 1);
const N2: (i8, usize) = (-1, 2);

/// `R` is a motion its generator carries back to, and a unit.
fn claim_motion(r: &Clifford) {
    let back = generator(r).expect("a generator");
    assert!(same(&motion(&back).expect("a motion"), r), "cayley(back(R)) = R");
    assert!(same(&r.product(&reversed(r).expect("reverse")).expect("product"), &word(q(1, 1), &[])), "R·R̃ = 1");
}

/// The masks of grade 4 a value holds.
fn volume(r: &Clifford) -> Vec<usize> {
    r.terms().map(|(m, _)| m).filter(|m| m.count_ones() == 4).collect()
}

/// FIXTURE 11b-i, PLANES SHARING A UNIT: `B₁ = ½e₊₁e₊₂`, `B₂ = ⅓e₊₂e₊₃`.
/// The composite gains a part on the third plane `e₊₁e₊₃`, the turning
/// between the two, of the opposite sign in the other order, and holds no
/// volume. `e₊₁e₊₃ = −e₊₃e₊₁` in v4's key order.
#[test]
fn planes_sharing_a_unit_gain_the_turning_between_them_and_no_volume() {
    let (b1, b2) = (word(q(1, 2), &[P1, P2]), word(q(1, 3), &[P2, P3]));
    let (r1, r2) = (motion(&b1).expect("a motion"), motion(&b2).expect("a motion"));
    assert!(same(&r1, &sum(&[word(q(3, 5), &[]), word(q(4, 5), &[P1, P2])])), "R₁ = 3/5 + 4/5 e₊₁e₊₂");
    assert!(same(&r2, &sum(&[word(q(4, 5), &[]), word(q(3, 5), &[P2, P3])])), "R₂ = 4/5 + 3/5 e₊₂e₊₃");
    let one_way = compose(&r1, &r2).expect("composes");
    let other_way = compose(&r2, &r1).expect("composes");
    let expect = |s: i64| sum(&[word(q(12, 25), &[]), word(q(16, 25), &[P1, P2]), word(q(s * 12, 25), &[P1, P3]), word(q(9, 25), &[P2, P3])]);
    assert!(same(&one_way, &expect(1)), "R₁R₂: +12/25 on e₊₁e₊₃");
    assert!(same(&other_way, &expect(-1)), "R₂R₁: −12/25 on e₊₁e₊₃");
    assert!(same(&one_way, &sum(&[word(q(12, 25), &[]), word(q(16, 25), &[P1, P2]), word(q(-12, 25), &[P3, P1]), word(q(9, 25), &[P2, P3])])), "−12/25 on e₊₃e₊₁");
    let back = |s: i64| sum(&[word(q(16, 37), &[P1, P2]), word(q(s * 12, 37), &[P1, P3]), word(q(9, 37), &[P2, P3])]);
    assert!(same(&generator(&one_way).expect("a generator"), &back(1)), "B(R₁R₂) = (16, 12, 9)/37");
    assert!(same(&generator(&other_way).expect("a generator"), &back(-1)), "B(R₂R₁) = (16, −12, 9)/37");
    assert!(same(&compose_rates(&b1, &b2).expect("composes"), &back(1)), "the rates composed");
    assert!(same(&one_way.difference(&other_way).expect("one signature"), &word(q(24, 25), &[P1, P3])), "R₁R₂ − R₂R₁ = 24/25 e₊₁e₊₃");
    assert!(volume(&one_way).is_empty() && volume(&other_way).is_empty(), "no grade-4 part");
    [&r1, &r2, &one_way, &other_way].into_iter().for_each(claim_motion);
}

/// FIXTURE 11b-ii, DISJOINT PLANES: `B₁ = ½e₊₁e₊₂`, `B₂ = ⅓e₋₁e₋₂`. The
/// motions commute and the composite keeps the volume `12/25
/// e₊₁e₊₂e₋₁e₋₂`; its generator reads `4/7` and `3/7` per plane, never the
/// rates `½` and `⅓` it was composed from: `a(1 + c²)/(1 − a²c²)` and
/// `c(1 + a²)/(1 − a²c²)` at `a = ½`, `c = ⅓`.
#[test]
fn disjoint_planes_commute_and_keep_the_volume() {
    let (b1, b2) = (word(q(1, 2), &[P1, P2]), word(q(1, 3), &[N1, N2]));
    let (r1, r2) = (motion(&b1).expect("a motion"), motion(&b2).expect("a motion"));
    let one_way = compose(&r1, &r2).expect("composes");
    let expect = sum(&[word(q(12, 25), &[]), word(q(16, 25), &[P1, P2]), word(q(9, 25), &[N1, N2]), word(q(12, 25), &[P1, P2, N1, N2])]);
    assert!(same(&one_way, &expect), "R₁R₂ with the volume");
    assert!(same(&compose(&r2, &r1).expect("composes"), &expect), "R₂R₁ = R₁R₂");
    let (a, c) = (q(1, 2), q(1, 3));
    let den = q(1, 1) - &a * &a * &c * &c;
    let (on_first, on_second) = (&a * (q(1, 1) + &c * &c) / &den, &c * (q(1, 1) + &a * &a) / &den);
    assert_eq!((on_first.clone(), on_second.clone()), (q(4, 7), q(3, 7)));
    let back = generator(&one_way).expect("a generator");
    assert!(same(&back, &sum(&[word(on_first, &[P1, P2]), word(on_second, &[N1, N2])])), "B = 4/7 e₊₁e₊₂ + 3/7 e₋₁e₋₂");
    assert!(!same(&back, &b1.sum(&b2).expect("one signature")), "a per-plane reading does not give back ½ and ⅓");
    // The per-plane readings recomposed plane by plane are another motion:
    // `(33 + 56e₊₁e₊₂)/65 · (20 + 21e₋₁e₋₂)/29`, never `R₁R₂`.
    let piecewise = compose(&motion(&word(q(4, 7), &[P1, P2])).expect("a motion"), &motion(&word(q(3, 7), &[N1, N2])).expect("a motion")).expect("composes");
    let read_apart = sum(&[word(q(660, 1885), &[]), word(q(1120, 1885), &[P1, P2]), word(q(693, 1885), &[N1, N2]), word(q(1176, 1885), &[P1, P2, N1, N2])]);
    assert!(same(&piecewise, &read_apart) && !same(&piecewise, &expect), "the plane-by-plane recomposition loses the motion");
    assert!(same(&motion(&back).expect("a motion"), &expect), "the generator held whole keeps it, volume and all");
    [&r1, &r2, &one_way].into_iter().for_each(claim_motion);
}

/// FIXTURE 11b-iii, ONE PLANE: on the elliptic `e₊₁e₊₂`, `½` then `⅓` is
/// `1`; on the hyperbolic `e₊₁e₋₁`, `½` then `⅓` is `5/7`, the flow law.
/// The closed forms and the general law agree.
#[test]
fn one_plane_composes_by_its_regime() {
    for (plane, regime, expect) in [(word(q(1, 1), &[P1, P2]), Regime::Elliptic, q(1, 1)), (word(q(1, 1), &[P1, N1]), Regime::Hyperbolic, q(5, 7))] {
        assert_eq!(Regime::of(&plane), Ok(regime));
        assert_eq!(composed(regime, &q(1, 2), &q(1, 3)), Ok(expect.clone()));
        let rate = |b: Q| word(b, &[]).product(&plane).expect("one signature");
        let general = compose_rates(&rate(q(1, 2)), &rate(q(1, 3))).expect("composes");
        assert!(same(&general, &rate(expect)), "{regime:?}: the general law reads the closed form");
    }
}

/// ONE PLANE'S MOTION IN CLOSED FORM: `((1 − b²) + 2b·e)/(1 + b²)` on an
/// elliptic plane and `((1 + b²) + 2b·e)/(1 − b²)` on a hyperbolic one, for
/// rates of either sign, and the composition of every pair agreeing with
/// the general law.
#[test]
fn closed_forms_on_one_plane_agree_with_the_general_law() {
    let rates = [q(1, 2), q(-1, 3), q(2, 7), q(5, 3), q(0, 1)];
    for (plane, sign) in [(word(q(1, 1), &[P2, P3]), -1), (word(q(1, 1), &[P2, N2]), 1)] {
        let regime = Regime::of(&plane).expect("a plane");
        for b in &rates {
            let bb = b * b;
            let (scalar, den) = (q(1, 1) + Q::from_integer(sign.into()) * &bb, q(1, 1) - Q::from_integer(sign.into()) * &bb);
            let Ok(r) = on_plane(b, &plane) else {
                assert!(den == q(0, 1), "refused only at the pole");
                continue;
            };
            assert!(same(&r, &sum(&[word(&scalar / &den, &[]), word(q(2, 1) * b / &den, &[]).product(&plane).expect("one signature")])), "{regime:?} at {b}");
            claim_motion(&r);
            for c in &rates {
                let rate = |x: &Q| word(x.clone(), &[]).product(&plane).expect("one signature");
                match (composed(regime, b, c), compose_rates(&rate(b), &rate(c))) {
                    (Ok(x), Ok(general)) => assert!(same(&general, &rate(&x)), "{regime:?}: {b} then {c}"),
                    (Err(a), Err(g)) => assert_eq!(a, g, "{regime:?}: {b} then {c} refused alike"),
                    (a, g) => panic!("{regime:?}: {b} then {c}: {a:?} against {g:?}"),
                }
            }
        }
    }
}

/// THE REFUSALS BY NAME: `R = −1` has no generator; a hyperbolic rate of
/// `1` names no motion (`1 − B` is a zero divisor); an odd value is not a
/// motion, nor an even value that is not a unit; a rate with a scalar part
/// is not a generator.
#[test]
fn inverses_that_do_not_exist_are_refused_by_name() {
    assert_eq!(generator(&word(q(-1, 1), &[])).map(|_| ()), Err(TurningRefusal::NoGenerator));
    assert_eq!(motion(&word(q(1, 1), &[P1, N1])).map(|_| ()), Err(TurningRefusal::NoMotion));
    assert_eq!(composed(Regime::Elliptic, &q(1, 1), &q(1, 1)), Err(TurningRefusal::NoGenerator), "1 then 1 is R = −1");
    assert_eq!(composed(Regime::Hyperbolic, &q(1, 1), &q(1, 3)), Err(TurningRefusal::NoMotion));
    assert_eq!(generator(&word(q(1, 1), &[P1])).map(|_| ()), Err(TurningRefusal::NotEven { masks: vec![unit(1, 1)] }));
    assert!(matches!(generator(&word(q(2, 1), &[])), Err(TurningRefusal::NotUnit { .. })));
    assert_eq!(motion(&sum(&[word(q(1, 1), &[]), word(q(1, 2), &[P1, P2])])).map(|_| ()), Err(TurningRefusal::NotGenerator { masks: vec![0] }));
    assert_eq!(Regime::of(&word(q(2, 1), &[P1, P2])), Err(TurningRefusal::NotGenerator { masks: vec![unit(1, 1) | unit(1, 2)] }));
}

/// DEGREES ARE A READING ONLY, `tan(θ/4) = b`, as certified sectors: `b = 1`
/// is `180°` exactly; `½` reads `106;15,36` and `⅓` reads `73;44,23`, which
/// compose to `1` (`180°`); `2` reads `253;44,23` (`360° − 4·atan(½)`), and
/// `−½` reads `-107;44,23` (−106°15′36″, the sector below it).
#[test]
fn degrees_read_as_certified_sectors() {
    let read = |b: Q, depth| degrees(&b, depth).expect("a sector").written();
    assert_eq!(read(q(1, 1), 2), "180;00,00");
    assert_eq!(read(q(0, 1), 1), "0;00");
    assert_eq!(read(q(1, 2), 2), "106;15,36");
    assert_eq!(read(q(1, 3), 2), "73;44,23");
    assert_eq!(read(q(2, 1), 2), "253;44,23");
    assert_eq!(read(q(-1, 2), 2), "-107;44,23");
}
