// LAW-COORDINATE v3 (proposal 78.173 §5, fixtures 1–6 on one-component
// lanes): the canonical order and its writing, TPoint's writing as the
// non-negative case, the sexagesimal reading as display, `within` on the
// caller's intervals, origins never compared, and a neighbour range on one
// component.

use super::bodies::*;
use crate::coordinate::*;
use crate::Q;
use num_bigint::BigInt;

/// The origin enclosed once, in the branch cut at `3:2/1:0/1:0/1:0/1:0`.
fn enclosed() -> Origin {
    Origin { enclosures: vec![Local { h: 3, lanes: on_s(q(2, 1)), k: vec![0] }], ..root() }
}

/// FIXTURE 1A, THE ORDER: origin, `h`, the lanes S, G, T, H by exact signed
/// value, `K`. The bodies written in their canonical order sort to it, and
/// `1/2` stands after `1/3` though its writing sorts before.
#[test]
fn coordinate_orders_by_origin_bell_lanes_then_orbits() {
    let r = root();
    let ordered = vec![
        body(&r, 5, on_s(q(-1, 2)), vec![1]),
        body(&r, 5, lanes(q(1, 3), q(-1, 1), q(0, 1), q(0, 1)), vec![9]),
        body(&r, 5, on_s(q(1, 3)), vec![1]),
        body(&r, 5, on_s(q(1, 3)), vec![2]),
        body(&r, 5, on_s(q(1, 2)), vec![1]),
        body(&r, 6, on_s(q(-7, 1)), vec![0]),
        body(&enclosed(), 0, on_s(q(0, 1)), vec![0]),
    ];
    let mut sorted = ordered.clone();
    sorted.reverse();
    sorted.sort();
    assert_eq!(sorted, ordered);
    assert!(ordered[4].written() < ordered[2].written(), "as text, 1/2 sorts before 1/3");
}

/// FIXTURE 1B, THE WRITING: `origin/h:S:G:T:H:K`, a one-component lane
/// bare, a negative lane signed, an enclosure written out between the root
/// and the body.
#[test]
fn coordinate_writes_origin_enclosures_and_signed_lanes() {
    let g = "6a".repeat(32);
    assert_eq!(
        body(&root(), 5, on_s(q(1, 3)), vec![1]).written(),
        format!("{g}.00000092/0000000000000005:1/3:0/1:0/1:0/1:0000000000000001")
    );
    assert_eq!(
        body(&root(), 5, on_s(q(-1, 2)), vec![1]).written(),
        format!("{g}.00000092/0000000000000005:-1/2:0/1:0/1:0/1:0000000000000001")
    );
    assert_eq!(
        body(&enclosed(), 0, on_s(q(0, 1)), vec![0]).written(),
        format!("{g}.00000092/0000000000000003:2/1:0/1:0/1:0/1:0000000000000000/0000000000000000:0/1:0/1:0/1:0/1:0000000000000000")
    );
}

/// FIXTURE 1C, TPOINT IS THE NON-NEGATIVE CASE: with no enclosure and
/// one-component non-negative lanes, the coordinate writes exactly TPoint's
/// fixed writing (78.152 TU-5; the strings of
/// `chronometry::tuniversal::tests::point`) and orders as TPoint orders
/// (`compare_writings_agrees_with_point_order`).
#[test]
fn coordinate_reads_tpoint_as_its_non_negative_case() {
    let max = Q::from_integer(BigInt::from(u64::MAX));
    let p = body(&root(), 40_961, lanes(q(3, 1), max, q(0, 1), q(17, 1)), vec![812]);
    assert_eq!(p.written(), format!("{}.00000092/000000000000a001:3/1:ffffffffffffffff/1:0/1:11/1:000000000000032c", "6a".repeat(32)));
    let p = body(&root(), 10, lanes(q(7, 30), q(0, 1), q(1, 900), q(2, 1)), vec![3]);
    assert_eq!(p.local.written(), "000000000000000a:7/1e:0/1:1/384:2/1:0000000000000003");
    let at = |s: Q| body(&root(), 5, on_s(s), vec![1]);
    let (half, third, tenth) = (at(q(1, 2)), at(q(1, 3)), at(q(1, 10)));
    assert!(third < half && tenth < third, "the point's order is the lane's value");
    assert!(tenth < body(&root(), 6, on_s(q(1, 10)), vec![1]), "by h");
}

/// FIXTURE 2, THE SEXAGESIMAL READING, DISPLAY ONLY, at stated depths: the
/// sector's index, its bounds and its writing. `360/7` degrees reads
/// `51;25,42`, that is 51°25′42″.
#[test]
fn sector_reads_places_at_stated_depths() {
    let cases: [(Q, u32, i64, &str); 8] = [
        (q(7, 3), 0, 2, "2"),
        (q(7, 3), 1, 140, "2;20"),
        (q(7, 3), 2, 8_400, "2;20,00"),
        (q(1, 7), 3, 30_857, "0;08,34,17"),
        (q(360, 7), 2, 185_142, "51;25,42"),
        (q(-1, 3), 1, -20, "-1;40"),
        (q(-1, 7), 1, -9, "-1;51"),
        (q(1, 3) - q(1, 1_000_000_000_000_000), 6, 15_551_999_999, "0;19,59,59,59,59,59"),
    ];
    for (v, depth, index, written) in cases {
        let s = Sector::holding(&v, depth);
        assert_eq!(s.index, BigInt::from(index), "{v} at depth {depth}");
        assert_eq!(s.written(), written, "{v} at depth {depth}");
        assert!(s.lo() <= v && v < s.hi(), "{v} lies in [lo, hi)");
        assert_eq!(s.hi() - s.lo(), s.width());
    }
    assert_eq!(Sector::holding(&q(-1, 3), 1).lo(), q(-1, 3), "a value on a bound is the sector's lower bound");
    assert_eq!(Sector::holding(&q(-1, 7), 1).lo(), q(-3, 20));
}

/// FIXTURE 3, SECTORS NEST: each sector at depth `d + 1` lies within the
/// one at depth `d` that holds the same value, and is its child.
#[test]
fn sector_nests_in_its_parent() {
    for v in [q(1, 7), q(-22, 7), q(7, 3), q(0, 1), q(-1, 3)] {
        for d in 0..6 {
            let (coarse, fine) = (Sector::holding(&v, d), Sector::holding(&v, d + 1));
            assert!(coarse.lo() <= fine.lo() && fine.hi() <= coarse.hi(), "{v} at {d}");
            assert_eq!(fine.parent(), Some(coarse));
        }
        assert_eq!(Sector::holding(&v, 0).parent(), None);
    }
}

/// The bounds `[lo, hi)` on S's one component, at `origin`.
fn on_s_within(origin: &Origin, lo: Q, hi: Q) -> Bounds {
    Bounds::of(origin.clone(), Lane::S, vec![(on('S', 1), Interval { lo, hi })]).expect("one key")
}

/// FIXTURE 4, `within` ON THE CALLER'S INTERVAL: against `[1/3, 7/20)` the
/// lower bound is within, the upper bound above, a value `10⁻¹⁵` below the
/// bound below; against `[1/3, 1/3 + 60⁻⁶)` a value `1/(3·60⁶)` past `1/3`
/// is within; `2/5` is within `[0, 1)` and above `[1/3, 7/20)`. Each
/// reading is one side per component, within as a whole.
#[test]
fn within_decides_on_stated_intervals() {
    let r = root();
    let at = |s: Q| body(&r, 7, on_s(s), vec![4]);
    let (third, bound) = (q(1, 3), q(7, 20));
    let sixth = Q::new(BigInt::from(1), BigInt::from(3) * BigInt::from(60).pow(6));
    let width = Q::new(BigInt::from(1), BigInt::from(60).pow(6));
    let cases = [
        (third.clone(), third.clone(), bound.clone(), Side::Within),
        (bound.clone(), third.clone(), bound.clone(), Side::Above),
        (q(1, 3) - q(1, 1_000_000_000_000_000), third.clone(), bound.clone(), Side::Below),
        (q(1, 3) + sixth, third.clone(), q(1, 3) + width, Side::Within),
        (q(2, 5), q(0, 1), q(1, 1), Side::Within),
        (q(2, 5), third, bound, Side::Above),
    ];
    for (s, lo, hi, side) in cases {
        let read = at(s.clone()).within(&on_s_within(&r, lo, hi)).expect("one origin, one shape");
        assert_eq!(read.sides, vec![(on('S', 1), side)], "{s}");
        let whole = match side {
            Side::Within => Whole::Within,
            other => Whole::Outside(vec![(on('S', 1), other)]),
        };
        assert_eq!(read.whole(), whole);
    }
}

/// FIXTURE 4, A SHAPE NOT THE LANE'S IS REFUSED: bounds on `e_S2` against
/// S's `e_S1` are refused `ShapesDiffer`, naming both shapes.
#[test]
fn within_refuses_other_shape() {
    let r = root();
    let bounds = Bounds::of(r.clone(), Lane::S, vec![(on('S', 2), Interval { lo: q(0, 1), hi: q(1, 1) })]).expect("one key");
    assert_eq!(
        body(&r, 5, on_s(q(1, 3)), vec![0]).within(&bounds),
        Err(CoordinateRefusal::ShapesDiffer { lane: Lane::S, held: vec![on('S', 1)], asked: vec![on('S', 2)] })
    );
}

/// FIXTURE 5, ORIGINS ARE NEVER COMPARED: a reading between two origins is
/// refused `OriginsDiffer`, naming both, even where the lanes agree.
#[test]
fn within_refuses_two_origins() {
    let bounds = on_s_within(&enclosed(), q(1, 3), q(7, 20));
    assert_eq!(
        body(&root(), 5, on_s(q(1, 3)), vec![0]).within(&bounds),
        Err(CoordinateRefusal::OriginsDiffer { held: Box::new(root()), asked: Box::new(enclosed()) })
    );
}

/// Every body held whose test, read one by one, puts it in the range: at
/// `at`'s origin and bell, the lanes before `lane` equal, and the one
/// component of `lane` in `[lo, hi)`.
fn tested<'a>(held: &'a [Coordinate], at: &Coordinate, lane: &Lane, lo: &Q, hi: &Q) -> Vec<&'a Coordinate> {
    let n = at.local.lanes.place(lane).expect("a base lane");
    held.iter()
        .filter(|c| c.origin == at.origin && c.local.h == at.local.h && c.local.lanes.held()[..n] == at.local.lanes.held()[..n])
        .filter(|c| value(c, lane) >= lo && value(c, lane) < hi)
        .collect()
}

/// FIXTURE 6, A RANGE ON ONE COMPONENT: at the root and bell 5, S over
/// `[1/2, 9/10)` holds exactly the bodies at `S = 1/2, 3/5, 7/10, 4/5` of
/// the twenty at `S = i/10`, and the three more at `S = 1/2` with `G = −1,
/// 1/4, 2`; the body at bell 6 and the body of another epoch are not in
/// it. On G at `S = 1/2`, `[0, 1)` holds `G = 0` and `G = 1/4`. Each range
/// is exactly the bodies a test of every body returns.
#[test]
fn range_on_one_lane_returns_exactly_the_tested_bodies() {
    let r = root();
    let mut bodies = Bodies::new();
    for i in 0..20 {
        bodies.hold(body(&r, 5, on_s(q(i, 10)), vec![0]));
    }
    bodies.hold(body(&r, 6, on_s(q(1, 2)), vec![0]));
    bodies.hold(body(&Origin { epoch: 0x93, ..root() }, 5, on_s(q(1, 2)), vec![0]));
    for g in [q(-1, 1), q(1, 4), q(2, 1)] {
        bodies.hold(body(&r, 5, lanes(q(1, 2), g, q(0, 1), q(0, 1)), vec![0]));
    }
    let at = body(&r, 5, on_s(q(0, 1)), vec![0]);
    let span = Interval { lo: q(1, 2), hi: q(9, 10) };
    let found = bodies.range(&at, &Lane::S, &on('S', 1), &span).expect("S's component");
    let s: Vec<Q> = found.bodies.iter().map(|c| value(c, &Lane::S).clone()).collect();
    assert_eq!(s, [q(1, 2), q(1, 2), q(1, 2), q(1, 2), q(3, 5), q(7, 10), q(4, 5)]);
    assert_eq!(found.bodies.iter().collect::<Vec<_>>(), tested(bodies.held(), &at, &Lane::S, &span.lo, &span.hi));
    let at_half = body(&r, 5, on_s(q(1, 2)), vec![0]);
    let on_g = bodies.range(&at_half, &Lane::G, &on('G', 1), &Interval { lo: q(0, 1), hi: q(1, 1) }).expect("G's component");
    let g: Vec<Q> = on_g.bodies.iter().map(|c| value(c, &Lane::G).clone()).collect();
    assert_eq!(g, [q(0, 1), q(1, 4)]);
    let empty = bodies.range(&at, &Lane::S, &on('S', 1), &Interval { lo: q(9, 10), hi: q(1, 2) }).expect("S's component");
    assert!(empty.bodies.is_empty(), "an empty span holds no body");
    assert_eq!(
        bodies.range(&at, &Lane::S, &on('S', 2), &span).map(|n| n.bodies.len()),
        Err(CoordinateRefusal::NoComponent { lane: Lane::S, key: on('S', 2) })
    );
}

/// FIXTURE 6, THE COST IS A BISECTION: over every span tried the range is
/// the tested bodies, and each of the range's two bisections over `L`
/// bodies takes at most the binary digits of `L` probes, a bound that grows
/// by two (log₂ 4) from `N` to `4N` bodies, never with the count.
#[test]
fn range_probes_grow_by_bisection_steps() {
    let r = root();
    let held = |n: i64| {
        let mut b = Bodies::new();
        for i in 0..n {
            b.hold(body(&r, 5, on_s(q(i, n)), vec![0]));
            b.hold(body(&r, 6, on_s(q(i, n)), vec![0]));
        }
        b
    };
    let at = body(&r, 5, on_s(q(0, 1)), vec![0]);
    let (small, large) = (held(64), held(256));
    let digits = |b: &Bodies| usize::BITS - b.held().len().leading_zeros();
    assert_eq!(digits(&large), digits(&small) + 2, "4N holds two binary digits more than N");
    for (lo, hi) in [(q(0, 1), q(1, 1)), (q(1, 3), q(7, 20)), (q(-1, 2), q(1, 8)), (q(9, 10), q(2, 1))] {
        let span = Interval { lo: lo.clone(), hi: hi.clone() };
        let a = small.range(&at, &Lane::S, &on('S', 1), &span).expect("S's component");
        let b = large.range(&at, &Lane::S, &on('S', 1), &span).expect("S's component");
        assert_eq!(a.bodies.iter().collect::<Vec<_>>(), tested(small.held(), &at, &Lane::S, &lo, &hi));
        assert_eq!(b.bodies.iter().collect::<Vec<_>>(), tested(large.held(), &at, &Lane::S, &lo, &hi));
        assert!(a.probes <= 2 * digits(&small), "{lo}..{hi}: {} probes at N", a.probes);
        assert!(b.probes <= 2 * digits(&large), "{lo}..{hi}: {} probes at 4N", b.probes);
    }
}
