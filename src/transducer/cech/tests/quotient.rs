// 78.104 — THE QUOTIENT READING: a motion's generators inside the held
// algebra, or outside it by the generators' coset, exactly and with no complement
// chosen.

use crate::transducer::cech::quotient::{grow, quotient};
use crate::transducer::cga::{translator_at, Mv, E1, E2, E3, EM};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The (3, 4, 5) turn on the plane `mask`, as its rotor ray `2 + e_ab`.
fn turn(mask: usize) -> Mv {
    Mv::scalar(q(2)).add(&Mv::blade(mask, q(1)))
}

/// A boost on `e₁ ∧ e₋`: `5 + 3 e₁e₋`, its square's scalar `25 − 9`.
fn boost() -> Mv {
    Mv::scalar(q(5)).add(&Mv::blade(E1 | EM, q(3)))
}

/// A held algebra grown from the given motions, in the given order.
fn held(motions: &[Mv]) -> Vec<Mv> {
    motions.iter().fold(Vec::new(), |h, m| grow(&h, m, &mut stroke()).expect("grows"))
}

/// A MOTION THE HELD ALGEBRA SPANS IS INSIDE; ONE IT DOES NOT IS OUTSIDE. Two
/// turns on planes sharing an axis close to so(3) (rank 3), so a turn on the
/// third plane — never itself held — is inside. A boost leaves so(3): it is
/// outside, and its coset is read.
#[test]
fn quotient_spanned_inside_unspanned_outside() {
    let h = held(&[turn(E1 | E2), turn(E2 | E3)]);
    assert_eq!(h.len(), 3, "[e₁₂, e₂₃] closes to so(3)");
    let third = quotient(&h, &turn(E1 | E3), &mut stroke()).expect("read");
    assert!(third.spanned(), "e₁₃ lies in so(3) though never held");
    let b = quotient(&h, &boost(), &mut stroke()).expect("read");
    assert!(!b.spanned(), "a boost leaves so(3)");
    assert_eq!(b.outside.len(), 1, "one generator, one coset");
    assert!(!b.outside[0].residue.is_empty(), "its coset has a coordinate");
}

/// THE COSET'S COORDINATE IS THE COSET'S ALONE: the same algebra grown in two
/// orders gives one residue, and two motions that differ by something held
/// are one class — the two residues agree.
#[test]
fn quotient_coset_independent_of_order_and_held() {
    let one = held(&[turn(E1 | E2), turn(E2 | E3)]);
    let other = held(&[turn(E2 | E3), turn(E1 | E2)]);
    let a = quotient(&one, &boost(), &mut stroke()).expect("read");
    let b = quotient(&other, &boost(), &mut stroke()).expect("read");
    assert_eq!(a.outside[0].residue, b.outside[0].residue, "the order the algebra grew in does not move the coset");
    // The boost's generator plus a held generator: the same coset.
    let shifted = Mv::scalar(q(1)).add(&Mv::blade(E1 | EM, q(3)).add(&Mv::blade(E1 | E2, q(2))));
    let c = quotient(&one, &shifted, &mut stroke()).expect("read");
    assert_eq!(c.outside.len(), 1);
    assert_eq!(c.outside[0].residue, a.outside[0].residue, "differing by what is held, one class");
}

/// NULL-SAFE: a seen ray's generator is null (no metric complement exists on
/// it). Against nothing held it is outside; once held, the same slide twice
/// as far is inside, read with no metric.
#[test]
fn quotient_reads_null_slide_without_metric() {
    let t = translator_at(&[q(1), q(0), q(0)]);
    assert!(!quotient(&[], &t, &mut stroke()).expect("read").spanned(), "nothing held: outside");
    let h = held(&[t]);
    let twice = quotient(&h, &translator_at(&[q(2), q(0), q(0)]), &mut stroke()).expect("read");
    assert!(twice.spanned(), "the same slide further is inside");
    let across = quotient(&h, &translator_at(&[q(0), q(1), q(0)]), &mut stroke()).expect("read");
    assert!(!across.spanned(), "a slide on another axis is outside");
}

/// The identity has no generators: nothing inside, nothing outside.
#[test]
fn quotient_identity_has_no_generators() {
    let r = quotient(&[], &Mv::scalar(q(1)), &mut stroke()).expect("read");
    assert!(r.inside.is_empty() && r.spanned());
}
