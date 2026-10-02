// 78.73 — A TEST PER MALLEABILITY CLAUSE: every lane's elastic vent is its
// own orbit's return, a refused draw presses the lane it blocked, and the
// lanes' (μ, β, ε) read as exact rationals.

use crate::transducer::cech::orbit::OrbitRegistry;
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A 90° turn about the origin, unnormalized.
fn turn() -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)))
}

fn modes(m: &Mv) -> Vec<cga::MotionMode> {
    match cga::motion_modes(m, &mut stroke()) {
        Some(Ok(modes)) => modes,
        other => panic!("the modes read: {other:?}"),
    }
}

/// (open, closed, relieved) of one admission.
fn moved(a: crate::transducer::cech::flux::Admitted) -> (u64, u64, u64) {
    (a.opened, a.closed, a.relieved)
}

/// 78.73 (2) — 𝒽'S VENT IS THE TURN'S RETURN: four 90° turns in one plane
/// open twice, close once, and the fourth returns the orbit, relieving all
/// that stood open; the orbit is let go.
#[test]
fn four_quarter_turns_open_close_and_return() {
    let [m] = modes(&turn()).try_into().expect("one mode: one plane");
    let mut orbits = OrbitRegistry::new();
    let seen: Vec<(usize, (u64, u64, u64))> = (0..4).map(|_| orbits.admit_mode(&m, 3).map(|(l, a)| (l, moved(a))).expect("reads")).collect();
    assert_eq!(seen, vec![(3, (3, 0, 0)), (3, (3, 0, 0)), (3, (0, 3, 3)), (3, (0, 3, 3))], "on 𝒽: away, away, nearer, returned");
    assert_eq!(orbits.totals(), ([0; 4], [0, 0, 0, 12]), "nothing left open; twelve moved");
    assert!(orbits.is_empty(), "a returned orbit is let go: the carrier keeps its bound");
}

/// 78.73 (2) — A TURN AND ITS INVERSE RETURN AT ONCE: orientation is the
/// winding's sign, read against the plane's reference.
#[test]
fn a_turn_and_its_inverse_return() {
    let [there] = modes(&turn()).try_into().expect("one mode");
    let [back] = modes(&turn().reverse()).try_into().expect("one mode");
    let mut orbits = OrbitRegistry::new();
    assert_eq!(orbits.admit_mode(&there, 2).map(|(_, a)| moved(a)), Some((2, 0, 0)));
    assert_eq!(orbits.admit_mode(&back, 2).map(|(_, a)| moved(a)), Some((0, 2, 2)), "returned: relieved whole");
    assert!(orbits.is_empty());
}

/// 78.73 (2) — 𝒮'S VENT: a dilation by two and one by a half return.
#[test]
fn a_dilation_and_its_inverse_return_on_spatial() {
    let [grow] = modes(&cga::dilator(&q(2, 1))).try_into().expect("one mode");
    let [shrink] = modes(&cga::dilator(&q(1, 2))).try_into().expect("one mode");
    let mut orbits = OrbitRegistry::new();
    assert_eq!(orbits.admit_mode(&grow, 5).map(|(l, a)| (l, moved(a))), Some((0, (5, 0, 0))), "a boost is 𝒮");
    assert_eq!(orbits.admit_mode(&shrink, 5).map(|(l, a)| (l, moved(a))), Some((0, (0, 5, 5))));
    assert!(orbits.is_empty());
}

/// 78.73 (2), (3) — 𝒢'S VENT AND STRAIN ON THE FIBRE: a swap applied twice
/// returns; a three-cycle stands open twice and returns on the third; a map
/// sending one hue two places is no map and is not admitted; the identity
/// carries no strain.
#[test]
fn a_hue_orbit_returns_when_its_composite_is_the_identity() {
    let mut orbits = OrbitRegistry::new();
    let swap = [(1u8, 2u8), (2, 1)];
    assert_eq!(orbits.admit_hues(&swap, 4).map(moved), Some((4, 0, 0)));
    assert_eq!(orbits.admit_hues(&swap, 4).map(moved), Some((0, 4, 4)), "swapped back: returned");
    let cycle = [(1u8, 2u8), (2, 3), (3, 1)];
    let seen: Vec<_> = (0..3).map(|_| orbits.admit_hues(&cycle, 1).map(moved)).collect();
    assert_eq!(seen, vec![Some((1, 0, 0)), Some((1, 0, 0)), Some((0, 1, 2))], "three hues displaced until the third returns all");
    assert_eq!(orbits.admit_hues(&[(1, 2), (1, 3)], 1), None, "one hue two places: no map");
    assert_eq!(orbits.admit_hues(&[(4, 4)], 1), None, "the identity: no strain");
    assert_eq!(orbits.totals(), ([0; 4], [0, 11, 0, 0]), "all on 𝒢, all returned");
}

/// 78.73 (1) — STAGNATION: a refused draw presses each short lane by the
/// flow it blocked, and nothing else; the reserve stands as it was.
#[test]
fn a_refused_draw_presses_the_lane_it_blocked() {
    let mut p = crate::physics::Physics { reserve: crate::physics::Reserve::of([5, 0, 5, 1]), ..Default::default() };
    assert!(p.draw([1, 3, 0, 4]).is_err(), "𝒢 and 𝒽 cannot cover it");
    assert_eq!(p.pressure, [0, 3, 0, 3], "blocked: three on 𝒢, three past the one 𝒽 held");
    assert_eq!(p.reserve.lanes(), [5, 0, 5, 1], "all or nothing");
}

/// 78.73 (4) — THE MALLEABILITY LINE, EXACT: one generator on 𝒮 and 𝒢, one
/// refused draw on 𝒢: 𝒮 took all its stress plastically, 𝒢 half; lanes no
/// stress met are named unstressed, lanes nothing moved on unmoved.
#[test]
fn malleability_reads_exact_rationals_and_names_no_reading() {
    let mut p = crate::physics::Physics { reserve: crate::physics::Reserve::of([9, 0, 9, 9]), ..Default::default() };
    p.plastic([true, true, false, false]);
    let _ = p.draw([0, 1, 0, 0]);
    let zero = || (num_bigint::BigInt::from(0), num_bigint::BigInt::from(0));
    let elastic = [zero(), zero(), (1.into(), 4.into()), zero()];
    assert_eq!(
        p.malleability_line(&elastic),
        "μ [1, 1/2, unstressed, unstressed] β [0, 1/2, unstressed, unstressed] ε [unmoved, unmoved, 1/4, unmoved]"
    );
}
