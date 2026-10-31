// 78.70 — THE THEOREM: one proof certified to hold for every member of its
// family, each certificate re-checked finitely and its closed form read
// without enumerating the family.

use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::compose::{compose, Term};
use crate::transducer::sentence::theorem::{Certificate, Open, Theorem};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn one() -> Mv {
    Mv::scalar(q(1))
}

fn turn(mask: usize) -> Mv {
    one().add(&Mv::blade(mask, q(1)))
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn power(v: &Mv, m: u64) -> Mv {
    (0..m).fold(one(), |p, _| p.geo(v))
}

/// PERIODIC: a 90° turn's sentence closes at four, so every power is the
/// power mod four — read by the certificate and checked against the power
/// computed the long way for three whole periods.
#[test]
fn theorem_at_reads_periodic_power_by_period() {
    let r = turn(E1 | E2);
    let proof = compose(&[Term { word: r.clone(), transport: one() }], &mut stroke()).expect("a sentence");
    let t = Theorem::of(&proof).expect("it closes");
    assert!(matches!(t.certificate, Certificate::Periodic { ell: 4 }));
    assert!(t.check(&mut stroke()), "the certificate re-checks");
    for m in 0..12 {
        assert!(t.at(m).expect("a power").projective_eq(&power(&r, m)), "Ω^{m} by the period");
    }
}

/// NILPOTENT: a translation never returns but is null, so its family is a
/// line — `Ω^m ∝ 1 + m·B` — read with no product at all.
#[test]
fn theorem_at_reads_nilpotent_power_as_line() {
    let t_motion = cga::translator_at(&[q(1), q(2), q(0)]);
    let proof = compose(&[Term { word: one(), transport: t_motion.clone() }], &mut stroke()).expect("a sentence");
    let t = Theorem::of(&proof).expect("a null motion extrapolates");
    assert!(matches!(t.certificate, Certificate::Nilpotent { .. }));
    assert!(t.check(&mut stroke()));
    for m in 0..7 {
        assert!(t.at(m).expect("a power").projective_eq(&power(&t_motion, m)), "T^{m} = 1 + m·B");
    }
}

/// SATURATED: two turns sharing an axis close to so(3); every nested
/// bracket of any depth lies in the three-dimensional span, and the
/// certificate is that finite check.
#[test]
fn saturated_holds_every_nested_bracket() {
    let t = Theorem::saturated(&[turn(E1 | E2), turn(E2 | E3)], &mut stroke()).expect("closes");
    assert!(matches!(&t.certificate, Certificate::Saturated { basis } if basis.len() == 3));
    assert!(t.check(&mut stroke()));
    let a = Mv::blade(E1 | E2, q(1));
    let b = Mv::blade(E2 | E3, q(1));
    let br = |x: &Mv, y: &Mv| x.geo(y).sub(&y.geo(x));
    let deep = br(&br(&br(&a, &b), &a), &br(&b, &br(&a, &b)));
    assert!(t.holds(&deep), "a bracket four deep stays in the span");
    assert!(!t.holds(&Mv::blade(E1 | cga::EM, q(1))), "a boost is outside so(3)");
}

/// NORMAL CLOSURE AND HONEST REFUSAL: a theorem carried to another chart
/// keeps its certificate and still checks; a tampered certificate fails;
/// a motion with no finite certificate is held open by name.
#[test]
fn conjugated_checks_and_forged_certificate_fails() {
    let r = turn(E1 | E2);
    let proof = compose(&[Term { word: r, transport: one() }], &mut stroke()).expect("a sentence");
    let t = Theorem::of(&proof).expect("closes");
    let seen = t.conjugated(&turn(E2 | E3));
    assert!(seen.check(&mut stroke()), "the same period from another chart");
    let forged = Theorem { omega: t.omega.clone(), certificate: Certificate::Periodic { ell: 3 } };
    assert!(!forged.check(&mut stroke()), "a period the motion does not have fails");
    let boost = cga::cayley(&Mv::blade(E1 | cga::EM, BigRational::new(1.into(), 2.into()))).expect("a finite boost");
    let open = compose(&[Term { word: boost, transport: one() }], &mut stroke()).expect("a sentence");
    assert_eq!(Theorem::of(&open).err(), Some(Open::Unbound), "a boost never returns and is not null");
}
