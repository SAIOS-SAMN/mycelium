// 78.73 — A TEST PER MALLEABILITY CLAUSE: every lane's elastic vent is its
// own orbit's return, a refused draw presses the lane it blocked, and the
// lanes' (μ, β, ε) read as exact rationals.

use crate::transducer::cech::orbit::{OrbitRegistry, Thread};

/// A mode admitted on no line of a complex.
const FREE: Thread = Thread::Free;
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The turn at `t` along the `e₁e₂` plane's flow, `1 + t·e₁e₂` — the
/// (3, 4, 5) turn at `t = 1/2`, of no finite order.
fn turn_at(t: BigRational) -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, t))
}

fn turn() -> Mv {
    turn_at(q(1, 2))
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

/// 78.73 (2), 78.75 — 𝒽'S VENT IS THE FLOW'S RETURN, WHATEVER THE AMOUNTS:
/// one plane is one orbit, and every turn admitted on it composes into the
/// point its flow has reached. The (3, 4, 5) turn — a rosette, no finite
/// order, never returning alone — twice flows away (t = 1/2, then 4/3); a
/// turn of ANOTHER amount, 4/9 back, brings it nearer (to t = 4/3 ⊕ −4/9 =
/// 24/43, `|1 − c|` from 32/25 down to 1152/2425), and the turn that is the remainder's own inverse
/// returns it, relieving all that stood open. Counted as ±1 windings of one
/// fixed angle, keyed by the angle, the three amounts stood on three orbits
/// and none could ever return the others.
#[test]
fn admit_mode_composes_turns_of_any_amount() {
    let mode = |t: BigRational| -> cga::MotionMode {
        let [m] = modes(&turn_at(t)).try_into().expect("one mode: one plane");
        m
    };
    let mut orbits = OrbitRegistry::new();
    let mut admit = |t: BigRational| orbits.admit_mode(&mode(t), 3, &FREE).map(|(l, a)| (l, moved(a))).expect("reads");
    assert_eq!(admit(q(1, 2)), (3, (3, 0, 0)), "on 𝒽: away from its return");
    assert_eq!(admit(q(1, 2)), (3, (3, 0, 0)), "further: the flow stands at 4/3");
    assert_eq!(admit(q(-4, 9)), (3, (0, 3, 3)), "another amount, back: nearer");
    // Where it stands now, and the turn that is its exact inverse.
    let p = cga::FlowPoint::at(&Mv::blade(E1 | E2, q(1, 1)), &q(4, 3)).expect("a plane");
    let back = cga::FlowPoint::at(&Mv::blade(E1 | E2, q(1, 1)), &q(-4, 9)).expect("a plane");
    let here = p.then(&back).and_then(|x| x.parameter()).expect("off the pole");
    assert_eq!(admit(-here), (3, (0, 3, 3)), "its inverse: returned, relieved whole");
    assert!(orbits.is_empty(), "a returned orbit is let go: the carrier keeps its bound");
    assert_eq!(orbits.totals(), ([0; 4], [0, 0, 0, 12]), "nothing open; twelve moved");
}

/// 78.73 (2) — A TURN AND ITS INVERSE RETURN AT ONCE: orientation is the
/// winding's sign, read against the plane's reference.
#[test]
fn admit_mode_returns_turn_and_inverse() {
    let [there] = modes(&turn()).try_into().expect("one mode");
    let [back] = modes(&turn().reverse()).try_into().expect("one mode");
    let mut orbits = OrbitRegistry::new();
    assert_eq!(orbits.admit_mode(&there, 2, &FREE).map(|(_, a)| moved(a)), Some((2, 0, 0)));
    assert_eq!(orbits.admit_mode(&back, 2, &FREE).map(|(_, a)| moved(a)), Some((0, 2, 2)), "returned: relieved whole");
    assert!(orbits.is_empty());
}

/// 78.73 (2), 78.75 — 𝒮'S VENT: dilations compose on one orbit by the dilations'
/// ratios — by two, then by three, then by a sixth returns.
#[test]
fn admit_mode_composes_dilations_and_returns_on_spatial() {
    let one = |r: BigRational| -> cga::MotionMode {
        let [m] = modes(&cga::dilator(&r)).try_into().expect("one mode");
        m
    };
    let mut orbits = OrbitRegistry::new();
    assert_eq!(orbits.admit_mode(&one(q(2, 1)), 5, &FREE).map(|(l, a)| (l, moved(a))), Some((0, (5, 0, 0))), "a boost is 𝒮");
    assert_eq!(orbits.admit_mode(&one(q(3, 1)), 5, &FREE).map(|(l, a)| (l, moved(a))), Some((0, (5, 0, 0))), "by three more: further");
    assert_eq!(orbits.admit_mode(&one(q(1, 6)), 5, &FREE).map(|(l, a)| (l, moved(a))), Some((0, (0, 5, 10))), "by a sixth: returned");
    assert!(orbits.is_empty());
}

/// 78.73 (2), (3) — 𝒢'S VENT AND STRAIN ON THE FIBRE: a swap applied twice
/// returns; a three-cycle stands open twice and returns on the third; a map
/// sending one hue two places is no map and is not admitted; the identity
/// carries no strain.
#[test]
fn admit_hues_returns_at_identity_composite() {
    let mut orbits = OrbitRegistry::new();
    let swap = [(1u8, 2u8), (2, 1)];
    assert_eq!(orbits.admit_hues(&swap, 4, &FREE).map(moved), Some((4, 0, 0)));
    assert_eq!(orbits.admit_hues(&swap, 4, &FREE).map(moved), Some((0, 4, 4)), "swapped back: returned");
    let cycle = [(1u8, 2u8), (2, 3), (3, 1)];
    let seen: Vec<_> = (0..3).map(|_| orbits.admit_hues(&cycle, 1, &FREE).map(moved)).collect();
    assert_eq!(seen, vec![Some((1, 0, 0)), Some((1, 0, 0)), Some((0, 1, 2))], "three hues displaced until the third returns all");
    assert_eq!(orbits.admit_hues(&[(1, 2), (1, 3)], 1, &FREE), None, "one hue two places: no map");
    assert_eq!(orbits.admit_hues(&[(4, 4)], 1, &FREE), None, "the identity: no strain");
    assert_eq!(orbits.totals(), ([0; 4], [0, 11, 0, 0]), "all on 𝒢, all returned");
}

/// 78.73 (1) — STAGNATION: a refused draw presses each short lane by the
/// flow it blocked, and nothing else; the reserve stands as it was.
#[test]
fn draw_presses_blocked_lane_on_refusal() {
    let mut p = crate::physics::Physics { reserve: crate::physics::Reserve::of([5, 0, 5, 1]), ..Default::default() };
    assert!(p.draw([1, 3, 0, 4]).is_err(), "𝒢 and 𝒽 cannot cover it");
    assert_eq!(p.pressure, [0, 3, 0, 3], "blocked: three on 𝒢, three past the one 𝒽 held");
    assert_eq!(p.reserve.lanes(), [5, 0, 5, 1], "all or nothing");
}

/// 78.73 (4) — THE MALLEABILITY LINE, EXACT: one generator on 𝒮 and 𝒢, one
/// refused draw on 𝒢: 𝒮 took all its stress plastically, 𝒢 half; lanes no
/// stress met are named unstressed, lanes nothing moved on unmoved.
#[test]
fn malleability_line_reads_exact_rationals_and_names_unread() {
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
