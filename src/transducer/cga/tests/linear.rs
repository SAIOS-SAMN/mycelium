// 78.11, MR1 — a linear function of the vectors, held by its images: the
// laws it is read by, on the actions of the engine's own versors.

use super::common::q;
use crate::transducer::cga::*;

/// A (3, 4, 5) turn in `e₁e₂` and a translation, both in `Cl(4, 1)`: a
/// function is composed only with one of its own algebra.
fn motions() -> (Mv, Mv) {
    (rotator(&q(2, 1), &q(1, 1)).widened(3), translator_at(&[q(1, 2), q(-3, 1), q(2, 1)]))
}

/// CAYLEY–HAMILTON, read by the function itself: the characteristic
/// polynomial the traces of its powers give, applied to the function by
/// Horner's rule, sends every vector to zero.
#[test]
fn linear_function_satisfies_characteristic_polynomial() {
    let (turn, shift) = motions();
    for m in [turn, shift] {
        let r = LinearFunction::of_versor(&m).expect("a versor acts");
        assert!(r.polynomial(&r.characteristic()).is_zero(), "p(R) = 0 for {m:?}");
    }
}

/// THE TRACE IS CYCLIC: `tr(f ∘ g) = tr(g ∘ f)`, the vector derivative's
/// reading of each composite, though the two composites differ.
#[test]
fn trace_is_cyclic_on_composite() {
    let (turn, shift) = motions();
    let (f, g) = (LinearFunction::of_versor(&turn).expect("acts"), LinearFunction::of_versor(&shift).expect("acts"));
    assert!(!f.after(&g).minus(&g.after(&f)).is_zero(), "the two do not commute");
    assert_eq!(f.after(&g).trace(), g.after(&f).trace(), "and the two traces agree");
}

/// THE KERNEL OF `R − 1` IS WHAT THE MOTION HOLDS: a turn in `e₁e₂` holds
/// `e₃`, `e₊` and `e₋` and nothing more, and every vector of the kernel is
/// held by the action.
#[test]
fn kernel_of_is_fixed_flat_for_turn_less_one() {
    let (turn, _) = motions();
    let r = LinearFunction::of_versor(&turn).expect("acts");
    let fixed = r.shifted(&q(1, 1)).kernel();
    assert_eq!(fixed.len(), 3, "e₃, e₊, e₋");
    assert!(fixed.iter().all(|v| r.at(v).sub(v).is_zero()), "each held");
}

/// THE MOTION'S MODES FROM ITS OWN ACTION: the rays compose back to the
/// motion (T1), the rays' values the roots of the characteristic polynomial of
/// `(R + R⁻¹)/2`, and the (3, 4, 5) turn's value is `c = 3/5`, rational.
#[test]
fn motion_modes_composes_back_to_action() {
    let (turn, _) = motions();
    let modes = match motion_modes(&turn, &mut super::common::budget()) {
        Some(Ok(ms)) => ms,
        other => panic!("the modes read: {other:?}"),
    };
    assert_eq!(modes.iter().map(|m| m.c.clone()).collect::<Vec<_>>(), vec![q(3, 5)], "one turn, at 3/5");
    let product = modes.iter().fold(Mv::scalar(q(1, 1)), |acc, m| acc.geo(&m.ray));
    assert!(product.projective_eq(&turn), "the rays compose back to the turn");
}
