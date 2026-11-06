// LAW-COORDINATE v4 (proposal 78.173 §5, fixtures 11–13 and 16): lanes as
// keyed vectors, H on two turning planes in degrees — the order and the
// writings, a range on one component and then a filter, a merger and
// fission per component — and the lane set: keys in canonical order, `K`
// per component, H on turning planes alone, and a lane no ruled opening
// admits refused.

use super::bodies::*;
use crate::coordinate::*;
use crate::Q;

/// FIXTURE 11, A VECTOR LANE IS ORDERED BY ITS COMPONENTS, IN KEY ORDER:
/// with H = `(e₊₂e₊₃, e₊₃e₊₁)` in degrees, `b = (1/3, −1/2)` stands before
/// `a = (1/3, 7/4)`, which stands before `c = (1/2, 0)`. The exact writing
/// is `(1/3,7/4)`; the display reading at depth 0 is `0 × 1` and at depth
/// 1 `0;20 × 1;45` (0°20′ × 1°45′), each sector an exact interval. Against
/// the bounds `[0, 1) × [1, 2)` the caller states, `c` reads `Within` on
/// `e₊₂e₊₃` and `Below` on `e₊₃e₊₁`: outside as a whole.
#[test]
fn vector_lane_orders_writes_and_reads_per_component() {
    let (a, b, c) = (planar(q(1, 3), q(7, 4)), planar(q(1, 3), q(-1, 2)), planar(q(1, 2), q(0, 1)));
    assert!(b < a && a < c, "the first component, then the second breaking the tie");
    let h = a.lane(&Lane::H).expect("H");
    assert_eq!(h.written(), "(1/3,7/4)");
    assert_eq!(a.local.written(), "0000000000000005:0/1:0/1:0/1:(1/3,7/4):(0000000000000000,0000000000000000)");
    assert_eq!(a.reading(&Lane::H, 0).expect("H").written(), "0 × 1");
    let reading = a.reading(&Lane::H, 1).expect("H");
    assert_eq!(reading.written(), "0;20 × 1;45");
    let bounds: Vec<(Q, Q)> = reading.sectors.iter().map(|(_, s)| (s.lo(), s.hi())).collect();
    assert_eq!(bounds, [(q(1, 3), q(7, 20)), (q(7, 4), q(53, 30))]);
    assert_eq!((Sector::holding(&q(-1, 2), 0).written(), Sector::holding(&q(-1, 2), 1).written()), ("-1".to_string(), "-1;30".to_string()));
    let (z1, z2) = about();
    let box_ =Bounds::of(root(), Lane::H, vec![(z1.clone(), Interval { lo: q(0, 1), hi: q(1, 1) }), (z2.clone(), Interval { lo: q(1, 1), hi: q(2, 1) })]).expect("two keys");
    let sides = c.within(&box_).expect("one origin, one shape");
    assert_eq!(sides.sides, vec![(z1, Side::Within), (z2.clone(), Side::Below)]);
    assert_eq!(sides.whole(), Whole::Outside(vec![(z2, Side::Below)]));
    assert_eq!(a.within(&box_).map(|s| s.whole()), Ok(Whole::Within));
}

/// FIXTURE 12, A RANGE ON ONE COMPONENT, THEN A FILTER: of `a`, `b`, `c`
/// and `d = (2/5, 7/4)`, the range on H's `e₊₂e₊₃` over `[1/3, 1/2)` holds
/// `b`, `a`, `d` in canonical order, by two bisections; the filter on
/// `e₊₃e₊₁` over `[1, 2)` within it leaves `a` and `d`.
#[test]
fn range_on_one_component_then_filter() {
    let (a, b, c, d) = (planar(q(1, 3), q(7, 4)), planar(q(1, 3), q(-1, 2)), planar(q(1, 2), q(0, 1)), planar(q(2, 5), q(7, 4)));
    let mut held = Bodies::new();
    for x in [&a, &b, &c, &d] {
        held.hold(x.clone());
    }
    let probe = planar(q(0, 1), q(0, 1));
    let (z1, z2) = about();
    let found = held.range(&probe, &Lane::H, &z1, &Interval { lo: q(1, 3), hi: q(1, 2) }).expect("H's first plane");
    assert_eq!(found.bodies, &[b.clone(), a.clone(), d.clone()]);
    assert_eq!(found.filter(&Lane::H, &z2, &Interval { lo: q(1, 1), hi: q(2, 1) }), vec![&a, &d]);
}

/// FIXTURE 13, A MERGER AND FISSION PER COMPONENT: `a = (1/3, 7/4)` holding
/// 1 and `e = (1, −1/4)` holding 2 merge to `(7/9, 5/12)`, with offsets
/// `a (−4/9, 4/3)` and `e (2/9, −2/3)`, whose holding-weighted sum is zero
/// on each component; the display reading at depth 1 is `0;46 × 0;25`, and
/// fission gives back `a` and `e`. Bodies whose H is keyed differently are
/// refused `ShapesDiffer`.
#[test]
fn merge_and_fission_per_component() {
    let (a, e) = (planar(q(1, 3), q(7, 4)), planar(q(1, 1), q(-1, 4)));
    let merger = merge(&a, &q(1, 1), &e, &q(2, 1)).expect("one origin, one bell, a holding");
    let (z1, z2) = about();
    let h = |l: &Lanes| l.get(&Lane::H).expect("H").clone();
    assert_eq!(h(&merger.point.local.lanes), Vector::of(vec![(z1.clone(), q(7, 9)), (z2.clone(), q(5, 12))]).expect("two planes"));
    assert_eq!(h(&merger.offset_a), Vector::of(vec![(z1.clone(), q(-4, 9)), (z2.clone(), q(4, 3))]).expect("two planes"));
    assert_eq!(h(&merger.offset_b), Vector::of(vec![(z1.clone(), q(2, 9)), (z2.clone(), q(-2, 3))]).expect("two planes"));
    for ((_, x), (_, y)) in h(&merger.offset_a).components().iter().zip(h(&merger.offset_b).components()) {
        assert_eq!(x + q(2, 1) * y, q(0, 1));
    }
    assert_eq!(merger.point.reading(&Lane::H, 1).expect("H").written(), "0;46 × 0;25");
    assert_eq!(fission(&merger, &q(1, 1), &q(2, 1)), Ok((a, e)));
    let one = body(&root(), 5, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
    assert_eq!(
        merge(&planar(q(0, 1), q(0, 1)), &q(1, 1), &one, &q(1, 1)),
        Err(MotionRefusal::ShapesDiffer(Box::new(Shapes { lane: Lane::H, held: vec![z1.clone(), z2], asked: vec![z1] })))
    );
}

/// FIXTURE 16, THE KEYS AND `K` PER COMPONENT: H stated as `e₊₃e₊₁ 1080°`,
/// `e₊₂e₊₃ 400°` and τ₁₂ `e₊₁e₊₂ −30°` is held in the keys' written order
/// `e_+1+2`, `e_+2+3`, `e_+3+1`, written `(-1e/1,190/1,438/1)`, and its
/// advance by zero bells reads `K = (0, 1, 3)`, `⌊|θ_c|/360⌋`.
#[test]
fn keys_order_and_orbits_per_component() {
    let ((z1, z2), tau) = (about(), turning(1, 2));
    let h = Vector::of(vec![(z2.clone(), q(1080, 1)), (z1.clone(), q(400, 1)), (tau.clone(), q(-30, 1))]).expect("three planes");
    assert_eq!(h.keys(), [tau.clone(), z1.clone(), z2.clone()]);
    assert_eq!(h.keys().iter().map(Key::written).collect::<Vec<_>>(), ["e_+1+2", "e_+2+3", "e_+3+1"]);
    assert_eq!(h.written(), "(-1e/1,190/1,438/1)");
    let zero = |k: Key| Vector::one(k, q(0, 1));
    let base = |h: Vector| Lanes::base(zero(on('S', 1)), zero(on('G', 1)), zero(on('T', 1)), h).expect("the base lanes");
    let still = Vector::of(vec![(z1, q(0, 1)), (tau, q(0, 1)), (z2, q(0, 1))]).expect("three planes");
    let p = body(&root(), 0, base(h), vec![]).advance(&Rates { lanes: base(still) }, 0).expect("an advance by no bells");
    assert_eq!(p.local.k, vec![0, 1, 3]);
    assert!(p.local.written().ends_with(":(-1e/1,190/1,438/1):(0000000000000000,0000000000000001,0000000000000003)"));
}

/// FIXTURE 16, H STANDS ONLY ON TURNING PLANES (netstratum docs/117 §10.3):
/// the continuum's `e₊₂e₊₃` and the vacuum's `e₋₂e₋₃` key H; frame plane
/// 1's own `e₊₁e₋₁` and the mixed pair `e₊₁e₋₂` are boosts, refused on H
/// `NotTurningPlane` naming `Frame` and `Mixed`, and admitted on S; axes
/// named by letters, `e_Z1Z2`, name no square and are refused on H
/// `Unsigned`. A unit of plane 0 is unwritable.
#[test]
fn h_keys_are_turning_planes() {
    let (pos, neg) = (Square::Positive, Square::Negative);
    let zero = |k: Key| Vector::one(k, q(0, 1));
    let with_h = |h: Key| Lanes::base(zero(on('S', 1)), zero(on('G', 1)), zero(on('T', 1)), zero(h));
    let vacuum = units((neg, 2), (neg, 3));
    assert_eq!(vacuum.written(), "e_-2-3");
    assert_eq!(vacuum.plane(), Some(Plane::Turning(neg)));
    assert_eq!(turning(2, 3).plane(), Some(Plane::Turning(pos)));
    assert!(with_h(turning(2, 3)).is_ok() && with_h(vacuum).is_ok(), "the continuum's and the vacuum's turnings key H");
    let (frame, mixed, named) = (units((pos, 1), (neg, 1)), units((pos, 1), (neg, 2)), plane(('Z', 1), ('Z', 2)));
    assert_eq!(frame.written(), "e_+1-1");
    assert_eq!(with_h(frame.clone()), Err(FormRefusal::NotTurningPlane { key: frame.clone(), plane: Plane::Frame }));
    assert_eq!(with_h(mixed.clone()), Err(FormRefusal::NotTurningPlane { key: mixed.clone(), plane: Plane::Mixed }));
    assert_eq!(with_h(named.clone()), Err(FormRefusal::NotTurningPlane { key: named, plane: Plane::Unsigned }));
    let boosts = Vector::of(vec![(frame, q(3, 5)), (mixed, q(-1, 7))]).expect("two boost planes");
    assert!(Lanes::base(boosts, zero(on('G', 1)), zero(on('T', 1)), zero(turning(2, 3))).is_ok(), "S's keys may name boost planes");
    assert_eq!(Axis::unit(pos, 0), Err(FormRefusal::AxisUnwritable { letter: '+', index: 0 }));
}

/// FIXTURE 16, A LANE NO RULED OPENING ADMITS IS REFUSED: a fifth lane `W`
/// at law version 1 is refused `LaneNotOpened`, naming it, under the
/// network's ruled openings (none yet). Under an opening stated for the
/// claim, the same set is admitted and `W` stands after H.
#[test]
fn unopened_lane_is_refused() {
    let w = Lane::Opened { name: "W".to_string(), version: 1 };
    let zero = |k: Key| Vector::one(k, q(0, 1));
    let set = || {
        vec![
            (w.clone(), zero(on('W', 1))),
            (Lane::S, zero(on('S', 1))),
            (Lane::G, zero(on('G', 1))),
            (Lane::T, zero(on('T', 1))),
            (Lane::H, zero(turning(2, 3))),
        ]
    };
    assert_eq!(Openings::ruled().opened(), &[]);
    assert_eq!(Lanes::of(set(), &Openings::ruled()), Err(FormRefusal::LaneNotOpened { lane: w.clone() }));
    let opened = Lanes::of(set(), &Openings::stated(vec![w.clone()])).expect("an opened lane");
    let order: Vec<Lane> = opened.held().iter().map(|(l, _)| l.clone()).collect();
    assert_eq!(order, [Lane::S, Lane::G, Lane::T, Lane::H, w]);
}

/// THE LANE SET'S OTHER REFUSALS, each by name: a base lane missing, a lane
/// twice, an H component keyed by one axis, a key twice or one plane in
/// both orientations, a generator of one axis, an unwritable axis.
#[test]
fn lane_set_refuses_by_name() {
    let zero = |k: Key| Vector::one(k, q(0, 1));
    let (s, g, t, h) = (zero(on('S', 1)), zero(on('G', 1)), zero(on('T', 1)), zero(turning(2, 3)));
    let ruled = Openings::ruled();
    assert_eq!(Lanes::of(vec![(Lane::S, s.clone()), (Lane::G, g.clone()), (Lane::T, t.clone())], &ruled), Err(FormRefusal::LaneMissing { lane: Lane::H }));
    assert_eq!(
        Lanes::of(vec![(Lane::S, s.clone()), (Lane::S, s.clone()), (Lane::G, g.clone()), (Lane::T, t.clone()), (Lane::H, h)], &ruled),
        Err(FormRefusal::LaneRepeated { lane: Lane::S })
    );
    assert_eq!(Lanes::base(s, g, t, zero(on('Z', 1))), Err(FormRefusal::NotGenerator { key: on('Z', 1) }));
    assert_eq!(Vector::of(vec![(on('S', 1), q(1, 1)), (on('S', 1), q(2, 1))]), Err(FormRefusal::KeyRepeated { key: on('S', 1) }));
    let both = Vector::of(vec![(turning(2, 3), q(1, 1)), (turning(3, 2), q(2, 1))]);
    assert!(matches!(both, Err(FormRefusal::KeyRepeated { .. })), "one plane in both orientations");
    assert_eq!(Key::generator(axis('Z', 1), axis('Z', 1)), Err(FormRefusal::NotPlane { axis: axis('Z', 1) }));
    assert_eq!(Axis::of('z', 1), Err(FormRefusal::AxisUnwritable { letter: 'z', index: 1 }));
    assert_eq!(Axis::of('Z', 0), Err(FormRefusal::AxisUnwritable { letter: 'Z', index: 0 }));
}
