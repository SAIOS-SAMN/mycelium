// 78.75 — THE FLOW, a test per clause: a point on one plane's flow composes by
// the geometric product in the plane, one law for every kind with the
// metric's sign deciding (the turn's tangent addition, the boost's, and the
// null flow's plain addition — the slivers); the pole is held; the return
// and its nearness are exact; and a mode's point is its own ray.

use crate::transducer::cga::multivector::one;
use crate::transducer::cga::*;
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn plane(a: usize, b: usize) -> Mv {
    Mv::blade(a, one()).geo(&Mv::blade(b, one()))
}

/// The three kinds of plane, read off the metric's sign: `e₁e₂` turns
/// (`B² = −1`), `e₁e₋` boosts (`+1`), `e₁e_∞` is null (`0`).
fn planes() -> [(Mv, FlowKind); 3] {
    [
        (plane(E1, E2), FlowKind::Turn),
        (plane(E1, EM), FlowKind::Boost),
        (Mv::blade(E1, one()).geo(&e_inf()).grade(2), FlowKind::Null),
    ]
}

/// ONE LAW: two points `t₁`, `t₂` of one flow compose to `(t₁ + t₂)/(1 +
/// k t₁t₂)`, `k = B²` — and that is the geometric product of the two points' versors,
/// exactly, on every kind. On the null plane it is plain addition: 1/7 then
/// 2/5 is 19/35, the white-hole slivers' own law.
#[test]
fn flow_point_then_composes_by_metric_law() {
    let (t1, t2) = (q(1, 7), q(2, 5));
    for (b, kind) in planes() {
        let (p1, p2) = (FlowPoint::at(&b, &t1).expect("a plane"), FlowPoint::at(&b, &t2).expect("a plane"));
        assert_eq!(p1.kind(), kind);
        let p = p1.then(&p2).expect("one plane");
        assert!(p.versor().projective_eq(&p1.versor().geo(&p2.versor())), "{kind:?}: the composition is the product");
        let k = p1.square().clone();
        let law = (&t1 + &t2) / (one() + &k * &t1 * &t2);
        assert_eq!(p.parameter(), Some(law.clone()), "{kind:?}: (t₁ + t₂)/(1 + k t₁t₂)");
        if kind == FlowKind::Null {
            assert_eq!(law, q(19, 35), "null: plain addition — 1/7 then 2/5 is 19/35");
        }
    }
}

/// THE RETURN AND ITS NEARNESS, EXACT: a point and its own backward return to
/// the identity on every kind; `c` is the action's value on the plane — read
/// back against MR1's `c` off the motion's own action — and `|1 − c|` falls
/// as the flow nears its return.
#[test]
fn flow_point_returns_exactly_and_reads_nearness() {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    for (b, kind) in planes() {
        let p = FlowPoint::at(&b, &q(3, 4)).expect("a plane");
        assert!(!p.returned(), "{kind:?}: off its return");
        assert!(p.then(&p.back()).expect("one plane").returned(), "{kind:?}: forward then back is the identity");
        let near = FlowPoint::at(&b, &q(1, 4)).expect("a plane");
        if kind != FlowKind::Null {
            let modes = match motion_modes(&p.versor(), &mut budget) {
                Some(Ok(m)) => m,
                other => panic!("{kind:?}: modes {other:?}"),
            };
            let [mode] = modes.as_slice() else { panic!("{kind:?}: one mode, {modes:?}") };
            assert_eq!(p.c(), Some(mode.c.clone()), "{kind:?}: the flow's c is the action's own");
            assert_eq!(mode.flow().and_then(|m| m.against(p.plane())), Some(p.clone()), "{kind:?}: the mode's point is the flow point");
        }
        assert!(near.distance() < p.distance(), "{kind:?}: nearer the return, nearer by the exact measure");
        assert_eq!(FlowPoint::origin(&b).and_then(|o| o.distance()), Some(BigRational::from_integer(0.into())), "{kind:?}: the origin is the return");
    }
}

/// THE POLE IS HELD, NEVER REFUSED: on a turning plane the point `(0 : 1)` is
/// the 180° turn — no parameter, `c = −1` — and twice it is the full turn,
/// returned (the ray `−1`). A ROSETTE never returns: the (3, 4, 5) turn's
/// point composed with itself again and again stays exact and off its
/// return, its numbers growing — content, never capped.
#[test]
fn flow_point_holds_pole_and_rosette_never_returns() {
    let b = plane(E1, E2);
    let half = FlowPoint::of_motion(&b).expect("the plane's own blade");
    assert_eq!(half.parameter(), None, "the 180° turn: no finite parameter");
    assert_eq!(half.c(), Some(-one()), "c = −1");
    assert!(half.then(&half).expect("one plane").returned(), "twice round 180°: returned");
    // tan(θ/2) = 1/2: cos θ = 3/5, no finite order (Niven).
    let r = FlowPoint::at(&b, &q(1, 2)).expect("a plane");
    assert_eq!(r.c(), Some(q(3, 5)));
    let mut at = r.clone();
    for _ in 0..12 {
        at = at.then(&r).expect("one plane");
        assert!(!at.returned(), "a rosette never returns");
    }
    assert!(at.parameter().is_some_and(|t| t.denom() > &num_bigint::BigInt::from(1000)), "its point grows, exact: {at:?}");
}

/// A FLIP IS A TURN ONE DIMENSION UP: the mirror across a line of the plane,
/// unfolded, is even, acts on every point of the base as the mirror does, and
/// lies on a flow — at the 180° pole of the plane the mirror's normal makes
/// with the new axis. An even motion unfolds to itself.
#[test]
fn unfold_lifts_flip_to_turn_one_dimension_up() {
    let mirror = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    let turned = unfold(&mirror, 2).expect("a versor");
    assert_eq!(turned.n(), 3);
    for x in [point_at(&[q(1, 1), q(2, 1)]), point_at(&[q(-3, 2), q(5, 7)])] {
        assert!(turned.sandwich(&x).projective_eq(&mirror.sandwich(&x)), "on the base, exactly the mirror");
    }
    let p = FlowPoint::of_motion(&turned).expect("one plane");
    assert_eq!((p.kind(), p.parameter()), (FlowKind::Turn, None), "a turn, at its 180° pole");
    let even = translator_at(&[q(1, 1), q(0, 1)]);
    assert_eq!(unfold(&even, 2), Some(even), "an even motion is its own");
}
