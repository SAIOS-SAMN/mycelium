// 78.112 H1, H2 — HEARING: rings echoed through the rings larger than them,
// nested, the smallest inside the next larger, the largest on the outside,
// and the balance read.

use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::compose::{compose, Closure, Composition, Term};
use crate::transducer::sentence::hearing::{echo, nest, Echo, Seat};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn one() -> Mv {
    Mv::scalar(q(1))
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// One ring sounding `omega` once: its word carried by the identity.
fn ring(omega: Mv) -> Composition {
    compose(&[Term { word: omega, transport: one() }], &mut stroke()).expect("a ring")
}

/// A 90° turn on the plane `mask`, `1 + e_ij` (ℓ 4); the 180° turn is the
/// blade alone (ℓ 2).
fn turn_90(mask: usize) -> Mv {
    one().add(&Mv::blade(mask, q(1)))
}

/// TWO TURNS IN ONE PLANE NEST WHOLE AND BALANCE: the 180° turn returns
/// twice on one return of the 90° turn, its overtone; carried round it, it
/// comes back as itself — the echo is the identity. It seats whole inside,
/// the 90° turn is on the outside, and the rings are in harmonic balance.
#[test]
fn an_overtone_in_one_plane_seats_whole_and_balances() {
    let inner = ring(Mv::blade(E1 | E2, q(1)));
    let outer = ring(turn_90(E1 | E2));
    assert_eq!((inner.closure, outer.closure), (Closure::Closes(2), Closure::Closes(4)));
    let n = nest(&[&inner, &outer], &mut stroke());
    assert_eq!(n.seats, vec![Seat::Whole, Seat::Outer]);
    assert!(n.balanced, "every period divides the outer return, every seat whole");
}

/// A TURN IN A CROSSING PLANE SEATS WITH A TILT: the 180° turn on `e₂e₃`
/// divides the 90° turn on `e₁e₂`, but carried round it comes back on
/// another plane — its echo is a turn, a pitch shifted, and the rings are
/// out of balance.
#[test]
fn an_overtone_across_planes_seats_with_a_tilt_its_echo_a_turn() {
    let inner = ring(Mv::blade(E2 | E3, q(1)));
    let outer = ring(turn_90(E1 | E2));
    assert_eq!(echo(&inner.omega, &outer.omega, &mut stroke()), Echo::Turn);
    let n = nest(&[&inner, &outer], &mut stroke());
    assert_eq!(n.seats, vec![Seat::Tilted(vec![Echo::Turn]), Seat::Outer]);
    assert!(!n.balanced, "a tilted seat is the imbalance");
}

/// A RING THAT RETURNS SOUNDS INSIDE ONE THAT NEVER DOES: a 90° turn on
/// `e₂e₃` inside a translation along `e₁` — the carrier — commutes with it
/// and passes through whole.
#[test]
fn a_returning_ring_seats_whole_in_its_carrier() {
    let inner = ring(turn_90(E2 | E3));
    let carrier = compose(&[Term { word: one(), transport: cga::translator_at(&[q(1), q(0), q(0)]) }], &mut stroke())
        .expect("a carrier");
    assert_eq!(carrier.closure, Closure::Unbound);
    let n = nest(&[&carrier, &inner], &mut stroke());
    assert_eq!(n.seats, vec![Seat::Outer, Seat::Whole]);
    assert!(n.balanced);
}

/// A RING DIVIDING NO LARGER RING IS LEFT OVER: the 120° turn about the
/// diagonal, `(1 + e₁₂)(1 + e₂₃)`, returns at 3, which does not divide the
/// 90° turn's 4 — it stands outside, leftover, and the rings do not balance.
#[test]
fn a_ring_dividing_no_larger_ring_is_left_over() {
    let third = ring(turn_90(E1 | E2).geo(&turn_90(E2 | E3)));
    let outer = ring(turn_90(E1 | E2));
    assert_eq!((third.closure, outer.closure), (Closure::Closes(3), Closure::Closes(4)));
    let n = nest(&[&third, &outer], &mut stroke());
    assert_eq!(n.seats, vec![Seat::Leftover, Seat::Outer]);
    assert!(!n.balanced, "a leftover ring is outside the harmonic order");
}
