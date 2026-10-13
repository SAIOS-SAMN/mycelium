// 78.112 H1, H2 — HEARING: rings echoed through the rings larger than them,
// nested, the smallest inside the next larger, the largest on the outside,
// and the balance read.

use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::compose::{compose, Closure, Composition, Term};
use crate::transducer::sentence::hearing::{echo, nest, orbit, Echo, Seat};
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

/// 78.112 H6 — LIGHT PERPENDICULAR TO THE PLANE IS A TWIST: a 90° turn on
/// `e₁e₂` carried by a ray along `e₃`. The ray commutes with the turn's plane
/// whole: all of it advances the ring along its axis, none crosses the
/// plane, and the pitch — the advance per elliptic return — is four times
/// the ray.
#[test]
fn light_perpendicular_to_the_plane_is_a_twist_all_advance() {
    let t = cga::translator_at(&[q(0), q(0), q(1)]);
    let r = compose(&[Term { word: turn_90(E1 | E2), transport: t.clone() }], &mut stroke()).expect("a ring");
    let met = orbit(&[&r], &mut stroke());
    let [m] = met.as_slice() else { panic!("one ray met: {met:?}") };
    let ray = crate::transducer::cayley::bivector_of(&t).expect("the ray's generator");
    assert_eq!(m.bounce, 4, "the turn returns at four");
    assert_eq!(m.advance, ray, "the light along the axis advances whole");
    assert!(m.crossing.is_zero(), "nothing crosses the plane");
    assert_eq!(m.pitch, ray.scale(&q(4)), "the advance per return");
}

/// H6 — THE LIGHT IN A SPACE IS ITS NULL MODE, NOT ITS WHOLE GENERATOR: a
/// space that turns as it carries (a twist, `T·R` about the carried axis)
/// has a generator that is not null as a whole, and a null mode that is the
/// light; asked whole, it met nothing. A DILATED carriage `D·T` has no null
/// mode at all — it is one dilation about a shifted centre, hyperbolic: the
/// light from a star at a point, not at infinity (read live 2026-10-06: every
/// space of the stalled task's rings is hyperbolic or the identity).
#[test]
fn the_light_in_a_space_is_its_null_mode_and_a_dilated_carriage_has_none() {
    let twist = cga::translator_at(&[q(0), q(0), q(1)]).geo(&turn_90(E1 | E2));
    let r = compose(&[Term { word: turn_90(E1 | E2), transport: twist }], &mut stroke()).expect("a ring");
    let met = orbit(&[&r], &mut stroke());
    assert!(!met.is_empty(), "the twist's light is met: {:?}", r.modes);
    assert!(met.iter().all(|m| !m.advance.is_zero()), "along the axis it advances the ring");
    let dilated = cga::dilator(&BigRational::new(2.into(), 1.into())).geo(&cga::translator_at(&[q(0), q(0), q(1)]));
    let rays = crate::transducer::cech::flux::null_fluxes(&dilated, &mut stroke()).expect("its modes read");
    assert!(rays.is_empty(), "a dilated carriage holds no null mode: one dilation about a shifted centre");
}

/// H6 — LIGHT IN THE PLANE CROSSES IT: the same turn carried by a ray along
/// `e₁`, which lies in its plane. The ray anticommutes with the plane: none
/// of it advances, all of it crosses — the impact.
#[test]
fn light_in_the_plane_crosses_it() {
    let t = cga::translator_at(&[q(1), q(0), q(0)]);
    let r = compose(&[Term { word: turn_90(E1 | E2), transport: t }], &mut stroke()).expect("a ring");
    let met = orbit(&[&r], &mut stroke());
    let [m] = met.as_slice() else { panic!("one ray met: {met:?}") };
    assert!(m.advance.is_zero(), "no light along an axis: {:?}", m.advance);
    assert!(!m.crossing.is_zero(), "the light crosses the plane");
}

/// 78.112 H7b — HEARING WINDS THE CROSSING AS TORSION, AND ONLY THE CROSSING:
/// the ring the light crosses winds the versor its crossing derives; the
/// pure twist, all advance, winds nothing.
#[test]
fn hearing_winds_the_crossing_and_a_pure_twist_winds_nothing() {
    use crate::transducer::sentence::hearing::hear;
    let crossed = compose(&[Term { word: turn_90(E1 | E2), transport: cga::translator_at(&[q(1), q(0), q(0)]) }], &mut stroke())
        .expect("a ring");
    let met = orbit(&[&crossed], &mut stroke());
    let [m] = met.as_slice() else { panic!("one ray met") };
    let expected = crate::transducer::cayley::versor_of_bivector(&m.crossing).expect("the crossing derives a versor");
    assert_eq!(hear(&[&crossed], &mut stroke()).wound, Some(expected), "the impact, wound");
    let twist = compose(&[Term { word: turn_90(E1 | E2), transport: cga::translator_at(&[q(0), q(0), q(1)]) }], &mut stroke())
        .expect("a ring");
    assert_eq!(hear(&[&twist], &mut stroke()).wound, None, "all advance: nothing wound");
}

/// A ring's size, read off its own coordinates on the stroke its modes need.
fn size_of(m: &Mv) -> crate::transducer::sentence::hearing::Size {
    let need = cga::modes_need(m).expect("its modes read");
    let coords = crate::transducer::sentence::coordinates::Coordinates::of(m, &mut crate::physics::Physics::default().stroke_n(need)).expect("its coordinates read");
    crate::transducer::sentence::hearing::Size::of_coordinates(&coords).expect("a size")
}

/// `m` heard against a mind that remembers hearing the one ring `heard`.
fn heard_against(m: &Mv, heard: &Mv) -> crate::transducer::sentence::hearing::Echoed {
    crate::transducer::sentence::hearing::hear_across(m, size_of(m), vec![(size_of(heard), vec![(b"heard".to_vec(), heard)])].into_iter())
}

/// 78.112 H5 — A WORD HEARD ACROSS MINDS IS SEATED IN THE HEARER'S RINGS.
/// The hearer heard the 90° turn of the grid plane (ℓ 4). The speaker's 180°
/// turn of that plane (ℓ 2, dividing 4) seats in it WHOLE: understood whole,
/// carried as it is, the loop still. The 180° turn of a crossing plane seats
/// with a TILT: carried round the ring, the loop said → heard is exactly the
/// echo `Ω_L Ω_s Ω_L⁻¹ Ω_s⁻¹`, and it is a turn. A turn whose point is no
/// Niven point never returns: a carrier, larger than every returning ring —
/// the OUTER ring, understood as it is; so is a slide.
#[test]
fn a_word_heard_across_minds_is_seated_in_the_hearers_rings() {
    use crate::transducer::sentence::hearing::Seat;
    let quarter = turn_90(E1 | E2);
    let half = Mv::blade(E1 | E2, q(1));
    let heard = heard_against(&half, &quarter);
    assert_eq!(heard.seat, Seat::Whole, "the 180° turn sounds inside the 90° turn whole");
    assert!(heard.carried.as_ref().is_some_and(|c| c.projective_eq(&half)), "carried as it is: the loop still");
    let crossing = Mv::blade(E1 | E3, q(1));
    let tilted = heard_against(&crossing, &quarter);
    assert!(matches!(&tilted.seat, Seat::Tilted(e) if e == &vec![Echo::Turn]), "a crossing plane seats with a tilt: {:?}", tilted.seat);
    let carried = tilted.carried.expect("carried round the ring");
    let l = turn_90(E1 | E2);
    let e = l.geo(&crossing).geo(&l.reverse()).geo(&crossing.reverse());
    assert!(carried.geo(&crossing.reverse()).projective_eq(&e), "the loop said → heard is the echo");
    let rosette = Mv::scalar(BigRational::new((-1).into(), 2.into())).add(&Mv::blade(E1 | E2, q(1)));
    assert_eq!(heard_against(&rosette, &quarter).seat, Seat::Outer, "c = −3/5: no return, a carrier");
    let slide = cga::translator_at(&[q(1), q(0), q(0)]);
    let outer = heard_against(&slide, &quarter);
    assert_eq!(outer.seat, Seat::Outer, "the carrier is larger than every returning ring");
    assert!(outer.carried.is_some_and(|c| c.projective_eq(&slide)));
}

/// H2 ACROSS MINDS — THE NESTING DECIDES, NOT THE RINGS' NUMBER. A ring of
/// size 2 heard where the only larger layer is of size 3 divides none of it:
/// LEFTOVER, not understood, carried nowhere. Heard where a layer of size 4
/// stands it seats there; and with no larger layer at all it is the outer
/// ring.
#[test]
fn a_ring_dividing_no_larger_layer_is_leftover() {
    use crate::transducer::sentence::hearing::{hear_across, Size};
    let word = Mv::blade(E1 | E2, q(1));
    let ring = turn_90(E1 | E2);
    let layer = |s: Size| vec![(s, vec![(b"ring".to_vec(), &ring)])];
    let left = hear_across(&word, Size::Returns(2), layer(Size::Returns(3)).into_iter());
    assert_eq!((left.seat, left.carried), (Seat::Leftover, None));
    let seated = hear_across(&word, Size::Returns(2), layer(Size::Returns(4)).into_iter());
    assert_eq!(seated.seat, Seat::Whole);
    let alone = hear_across(&word, Size::Returns(2), Vec::new().into_iter());
    assert_eq!(alone.seat, Seat::Outer);
}

/// 78.112 H5, 78.73 — AN ECHO'S DEVIATION IS EXACT, PER LANE, AND GRADED.
/// The identity strains nothing. Turns on 𝒽 strain `(1 − c)/2`: the 180°
/// turn 1, the 90° turn ½, the turn whose ray is `2 + e₁₂` (c = 3/5) ⅕, the
/// slighter `3 + e₁₂` (c = 4/5) ⅒ — each perturbation its own exact amount,
/// never a yes or a no, and on 𝒽 alone. A gain strains 𝒮 alone; a slide 𝒯
/// alone, by its count.
#[test]
fn an_echos_deviation_is_exact_per_lane_and_graded() {
    use crate::transducer::sentence::hearing::strain;
    use crate::transducer::sentence::presentation::{LANE_H, LANE_S, LANE_T};
    let r = |n: i64, d: i64| BigRational::new(n.into(), d.into());
    let zero = r(0, 1);
    assert!(strain(&one()).expect("reads").iter().all(|x| *x == zero), "the identity strains nothing");
    for (ray, expected) in [
        (Mv::blade(E1 | E2, q(1)), r(1, 1)),
        (turn_90(E1 | E2), r(1, 2)),
        (Mv::scalar(q(2)).add(&Mv::blade(E1 | E2, q(1))), r(1, 5)),
        (Mv::scalar(q(3)).add(&Mv::blade(E1 | E2, q(1))), r(1, 10)),
    ] {
        let s = strain(&ray).expect("reads");
        assert_eq!(s[LANE_H], expected, "a turn's strain on 𝒽");
        assert!(s.iter().enumerate().all(|(l, x)| l == LANE_H || *x == zero), "on 𝒽 alone: {s:?}");
    }
    let gain = strain(&cga::dilator(&r(2, 1))).expect("reads");
    assert!(gain[LANE_S] > zero && gain.iter().enumerate().all(|(l, x)| l == LANE_S || *x == zero), "a gain strains 𝒮 alone: {gain:?}");
    let slide = strain(&cga::translator_at(&[q(3), q(0), q(0)])).expect("reads");
    assert!(slide[LANE_T] > zero && slide.iter().enumerate().all(|(l, x)| l == LANE_T || *x == zero), "a slide strains 𝒯 alone: {slide:?}");
}
