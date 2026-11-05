// LAW-COORDINATE v3 (proposal 78.173 §5, fixtures 11–13 and 16): lanes as
// keyed vectors, H on two frame planes in degrees — the order and the
// writings, a range on one component and then a filter, a merger and
// fission per component — and the lane set: keys in canonical order, `K`
// per component, and a lane no ruled opening admits refused.

use super::bodies::*;
use crate::coordinate::*;
use crate::Q;

/// FIXTURE 11, A VECTOR LANE IS ORDERED BY ITS COMPONENTS, IN KEY ORDER:
/// with H = `(e_Z1Q1, e_Z2Q2)` in degrees, `b = (1/3, −1/2)` stands before
/// `a = (1/3, 7/4)`, which stands before `c = (1/2, 0)`. The exact writing
/// is `(1/3,7/4)`; the display reading at depth 0 is `0 × 1` and at depth
/// 1 `0;20 × 1;45` (0°20′ × 1°45′), each sector an exact interval. Against
/// the bounds `[0, 1) × [1, 2)` the caller states, `c` reads `Within` on
/// `e_Z1Q1` and `Below` on `e_Z2Q2`: outside as a whole.
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
    let (z1, z2) = (plane(('Z', 1), ('Q', 1)), plane(('Z', 2), ('Q', 2)));
    let box_ = Bounds::of(root(), Lane::H, vec![(z1.clone(), Interval { lo: q(0, 1), hi: q(1, 1) }), (z2.clone(), Interval { lo: q(1, 1), hi: q(2, 1) })]).expect("two keys");
    let sides = c.within(&box_).expect("one origin, one shape");
    assert_eq!(sides.sides, vec![(z1, Side::Within), (z2.clone(), Side::Below)]);
    assert_eq!(sides.whole(), Whole::Outside(vec![(z2, Side::Below)]));
    assert_eq!(a.within(&box_).map(|s| s.whole()), Ok(Whole::Within));
}

/// FIXTURE 12, A RANGE ON ONE COMPONENT, THEN A FILTER: of `a`, `b`, `c`
/// and `d = (2/5, 7/4)`, the range on H's `e_Z1Q1` over `[1/3, 1/2)` holds
/// `b`, `a`, `d` in canonical order, by two bisections; the filter on
/// `e_Z2Q2` over `[1, 2)` within it leaves `a` and `d`.
#[test]
fn range_on_one_component_then_filter() {
    let (a, b, c, d) = (planar(q(1, 3), q(7, 4)), planar(q(1, 3), q(-1, 2)), planar(q(1, 2), q(0, 1)), planar(q(2, 5), q(7, 4)));
    let mut held = Bodies::new();
    for x in [&a, &b, &c, &d] {
        held.hold(x.clone());
    }
    let probe = planar(q(0, 1), q(0, 1));
    let found = held.range(&probe, &Lane::H, &plane(('Z', 1), ('Q', 1)), &Interval { lo: q(1, 3), hi: q(1, 2) }).expect("H's first plane");
    assert_eq!(found.bodies, &[b.clone(), a.clone(), d.clone()]);
    assert_eq!(found.filter(&Lane::H, &plane(('Z', 2), ('Q', 2)), &Interval { lo: q(1, 1), hi: q(2, 1) }), vec![&a, &d]);
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
    let (z1, z2) = (plane(('Z', 1), ('Q', 1)), plane(('Z', 2), ('Q', 2)));
    let h = |l: &Lanes| l.get(&Lane::H).expect("H").clone();
    assert_eq!(h(&merger.point.local.lanes), Vector::of(vec![(z1.clone(), q(7, 9)), (z2.clone(), q(5, 12))]).expect("two planes"));
    assert_eq!(h(&merger.offset_a), Vector::of(vec![(z1.clone(), q(-4, 9)), (z2.clone(), q(4, 3))]).expect("two planes"));
    assert_eq!(h(&merger.offset_b), Vector::of(vec![(z1.clone(), q(2, 9)), (z2, q(-2, 3))]).expect("two planes"));
    for ((_, x), (_, y)) in h(&merger.offset_a).components().iter().zip(h(&merger.offset_b).components()) {
        assert_eq!(x + q(2, 1) * y, q(0, 1));
    }
    assert_eq!(merger.point.reading(&Lane::H, 1).expect("H").written(), "0;46 × 0;25");
    assert_eq!(fission(&merger, &q(1, 1), &q(2, 1)), Ok((a, e)));
    let one = body(&root(), 5, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0]);
    assert_eq!(
        merge(&planar(q(0, 1), q(0, 1)), &q(1, 1), &one, &q(1, 1)),
        Err(MotionRefusal::ShapesDiffer(Box::new(Shapes {
            lane: Lane::H,
            held: vec![plane(('Z', 1), ('Q', 1)), plane(('Z', 2), ('Q', 2))],
            asked: vec![plane(('Z', 1), ('Q', 1))]
        })))
    );
}

/// FIXTURE 16, THE KEYS AND `K` PER COMPONENT: H stated as `e_Z2Q2 1080°`,
/// `e_Z1Q1 400°` and τ₁₂ `e_Z1Z2 −30°` is held in the keys' written order
/// `e_Z1Q1`, `e_Z1Z2`, `e_Z2Q2`, written `(190/1,-1e/1,438/1)`, and its
/// advance by zero bells reads `K = (1, 0, 3)`, `⌊|θ_c|/360⌋`.
#[test]
fn keys_order_and_orbits_per_component() {
    let (z1, tau, z2) = (plane(('Z', 1), ('Q', 1)), plane(('Z', 1), ('Z', 2)), plane(('Z', 2), ('Q', 2)));
    let h = Vector::of(vec![(z2.clone(), q(1080, 1)), (z1.clone(), q(400, 1)), (tau.clone(), q(-30, 1))]).expect("three planes");
    assert_eq!(h.keys(), [z1.clone(), tau.clone(), z2.clone()]);
    assert_eq!(h.keys().iter().map(Key::written).collect::<Vec<_>>(), ["e_Z1Q1", "e_Z1Z2", "e_Z2Q2"]);
    assert_eq!(h.written(), "(190/1,-1e/1,438/1)");
    let zero = |k: Key| Vector::one(k, q(0, 1));
    let base = |h: Vector| Lanes::base(zero(on('S', 1)), zero(on('G', 1)), zero(on('T', 1)), h).expect("the base lanes");
    let still = Vector::of(vec![(z1, q(0, 1)), (tau, q(0, 1)), (z2, q(0, 1))]).expect("three planes");
    let p = body(&root(), 0, base(h), vec![]).advance(&Rates { lanes: base(still) }, 0).expect("an advance by no bells");
    assert_eq!(p.local.k, vec![1, 0, 3]);
    assert!(p.local.written().ends_with(":(190/1,-1e/1,438/1):(0000000000000001,0000000000000000,0000000000000003)"));
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
            (Lane::H, zero(plane(('Z', 1), ('Q', 1)))),
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
    let (s, g, t, h) = (zero(on('S', 1)), zero(on('G', 1)), zero(on('T', 1)), zero(plane(('Z', 1), ('Q', 1))));
    let ruled = Openings::ruled();
    assert_eq!(Lanes::of(vec![(Lane::S, s.clone()), (Lane::G, g.clone()), (Lane::T, t.clone())], &ruled), Err(FormRefusal::LaneMissing { lane: Lane::H }));
    assert_eq!(
        Lanes::of(vec![(Lane::S, s.clone()), (Lane::S, s.clone()), (Lane::G, g.clone()), (Lane::T, t.clone()), (Lane::H, h)], &ruled),
        Err(FormRefusal::LaneRepeated { lane: Lane::S })
    );
    assert_eq!(Lanes::base(s, g, t, zero(on('Z', 1))), Err(FormRefusal::NotGenerator { key: on('Z', 1) }));
    assert_eq!(Vector::of(vec![(on('S', 1), q(1, 1)), (on('S', 1), q(2, 1))]), Err(FormRefusal::KeyRepeated { key: on('S', 1) }));
    let reversed = plane(('Q', 1), ('Z', 1));
    let both = Vector::of(vec![(plane(('Z', 1), ('Q', 1)), q(1, 1)), (reversed, q(2, 1))]);
    assert!(matches!(both, Err(FormRefusal::KeyRepeated { .. })), "one plane in both orientations");
    assert_eq!(Key::generator(axis('Z', 1), axis('Z', 1)), Err(FormRefusal::NotPlane { axis: axis('Z', 1) }));
    assert_eq!(Axis::of('z', 1), Err(FormRefusal::AxisUnwritable { letter: 'z', index: 1 }));
    assert_eq!(Axis::of('Z', 0), Err(FormRefusal::AxisUnwritable { letter: 'Z', index: 0 }));
}
