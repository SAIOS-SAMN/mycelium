// LAW-COORDINATE v1 (proposal 78.173 §5): the canonical order and its
// writing, TPoint's writing as the non-negative case, the sexagesimal
// reading at stated depths, `within` on and off a sector bound, origins
// never compared, and a neighbour range on one lane.

use crate::coordinate::*;
use crate::Q;
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// The fixtures' universe root: genesis `6a` × 32, epoch `0x92`.
fn root() -> Origin {
    Origin { genesis: [0x6a; 32], epoch: 0x92, enclosures: vec![] }
}

/// The body at `origin`, bell `h`, lanes `S:G:T:H`, `K`.
fn body(origin: &Origin, h: u64, lanes: [Q; 4], k: u64) -> Coordinate {
    Coordinate { origin: origin.clone(), local: Local { h, lanes, k } }
}

/// Lanes with `s` on S and nothing else.
fn on_s(s: Q) -> [Q; 4] {
    [s, q(0, 1), q(0, 1), q(0, 1)]
}

/// The origin enclosed once, in the branch cut at `3:2/1:0/1:0/1:0/1:0`.
fn enclosed() -> Origin {
    Origin { enclosures: vec![Local { h: 3, lanes: on_s(q(2, 1)), k: 0 }], ..root() }
}

/// FIXTURE 1A, THE ORDER: origin, `h`, the lanes S, G, T, H by exact signed
/// value, `K`. The bodies written in their canonical order sort to it, and
/// `1/2` stands after `1/3` though its writing sorts before.
#[test]
fn coordinate_orders_by_origin_bell_lanes_then_orbits() {
    let r = root();
    let ordered = vec![
        body(&r, 5, on_s(q(-1, 2)), 1),
        body(&r, 5, [q(1, 3), q(-1, 1), q(0, 1), q(0, 1)], 9),
        body(&r, 5, on_s(q(1, 3)), 1),
        body(&r, 5, on_s(q(1, 3)), 2),
        body(&r, 5, on_s(q(1, 2)), 1),
        body(&r, 6, on_s(q(-7, 1)), 0),
        body(&enclosed(), 0, on_s(q(0, 1)), 0),
    ];
    let mut sorted = ordered.clone();
    sorted.reverse();
    sorted.sort();
    assert_eq!(sorted, ordered);
    assert!(ordered[4].written() < ordered[2].written(), "as text, 1/2 sorts before 1/3");
}

/// FIXTURE 1B, THE WRITING: `origin/h:S:G:T:H:K`, a negative lane signed,
/// an enclosure written out between the root and the body.
#[test]
fn coordinate_writes_origin_enclosures_and_signed_lanes() {
    let g = "6a".repeat(32);
    assert_eq!(
        body(&root(), 5, on_s(q(1, 3)), 1).written(),
        format!("{g}.00000092/0000000000000005:1/3:0/1:0/1:0/1:0000000000000001")
    );
    assert_eq!(
        body(&root(), 5, on_s(q(-1, 2)), 1).written(),
        format!("{g}.00000092/0000000000000005:-1/2:0/1:0/1:0/1:0000000000000001")
    );
    assert_eq!(
        body(&enclosed(), 0, on_s(q(0, 1)), 0).written(),
        format!("{g}.00000092/0000000000000003:2/1:0/1:0/1:0/1:0000000000000000/0000000000000000:0/1:0/1:0/1:0/1:0000000000000000")
    );
}

/// FIXTURE 1C, TPOINT IS THE NON-NEGATIVE CASE: with no enclosure and non-negative
/// lanes, the coordinate writes exactly TPoint's fixed writing (78.152
/// TU-5; the strings of `chronometry::tuniversal::tests::point`) and orders
/// as TPoint orders (`compare_writings_agrees_with_point_order`).
#[test]
fn coordinate_reads_tpoint_as_its_non_negative_case() {
    let max = Q::from_integer(BigInt::from(u64::MAX));
    let p = body(&root(), 40_961, [q(3, 1), max, q(0, 1), q(17, 1)], 812);
    assert_eq!(p.written(), format!("{}.00000092/000000000000a001:3/1:ffffffffffffffff/1:0/1:11/1:000000000000032c", "6a".repeat(32)));
    let p = body(&root(), 10, [q(7, 30), q(0, 1), q(1, 900), q(2, 1)], 3);
    assert_eq!(p.local.written(), "000000000000000a:7/1e:0/1:1/384:2/1:0000000000000003");
    let at = |s: Q| body(&root(), 5, on_s(s), 1);
    let (half, third, tenth) = (at(q(1, 2)), at(q(1, 3)), at(q(1, 10)));
    assert!(third < half && tenth < third, "the point's order is the lane's value");
    assert!(tenth < body(&root(), 6, on_s(q(1, 10)), 1), "by h");
}

/// FIXTURE 2, THE SEXAGESIMAL READING at stated depths: the sector's index,
/// its exact bounds and its writing (display).
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

/// FIXTURE 4, `within` ON AND OFF A BOUND: against the sector at depth 1
/// holding `1/3`, `[1/3, 7/20)`, the lower bound is within, the upper bound
/// above, a value `10⁻¹⁵` below the bound below; at depth 6 a value
/// `1/(3·60⁶)` past `1/3` is within; `2/5` is within at depth 0 and above
/// at depth 1.
#[test]
fn within_decides_on_exact_sector_bounds() {
    let r = root();
    let other = body(&r, 5, on_s(q(1, 3)), 0);
    let at = |s: Q| body(&r, 7, on_s(s), 4);
    assert_eq!(at(q(1, 3)).within(&other, Lane::S, 1), Ok(Side::Within));
    assert_eq!(at(q(7, 20)).within(&other, Lane::S, 1), Ok(Side::Above));
    assert_eq!(at(q(1, 3) - q(1, 1_000_000_000_000_000)).within(&other, Lane::S, 1), Ok(Side::Below));
    let sixth = Q::new(BigInt::from(1), BigInt::from(3) * BigInt::from(BASE).pow(6));
    assert_eq!(at(q(1, 3) + sixth).within(&other, Lane::S, 6), Ok(Side::Within));
    assert_eq!(at(q(2, 5)).within(&other, Lane::S, 0), Ok(Side::Within));
    assert_eq!(at(q(2, 5)).within(&other, Lane::S, 1), Ok(Side::Above));
    assert_eq!(at(q(2, 5)).within(&other, Lane::G, 3), Ok(Side::Within), "both rest at 0 on G");
}

/// FIXTURE 5, ORIGINS ARE NEVER COMPARED: a reading between two origins is
/// refused `OriginsDiffer`, naming both, even where the lanes agree.
#[test]
fn within_refuses_two_origins() {
    let (a, b) = (body(&root(), 5, on_s(q(1, 3)), 0), body(&enclosed(), 5, on_s(q(1, 3)), 0));
    assert_eq!(
        a.within(&b, Lane::S, 1),
        Err(CoordinateRefusal::OriginsDiffer { held: Box::new(root()), asked: Box::new(enclosed()) })
    );
}

/// Every body held whose test, read one by one, puts it in the range.
fn tested<'a>(held: &'a [Coordinate], at: &Coordinate, lane: Lane, lo: &Q, hi: &Q) -> Vec<&'a Coordinate> {
    let n = lane.place();
    held.iter()
        .filter(|c| c.origin == at.origin && c.local.h == at.local.h && c.local.lanes[..n] == at.local.lanes[..n])
        .filter(|c| c.lane(lane) >= lo && c.lane(lane) < hi)
        .collect()
}

/// FIXTURE 6, A RANGE ON ONE LANE: at the root and bell 5, S over
/// `[1/2, 9/10)` holds exactly the bodies at `S = 1/2, 3/5, 7/10, 4/5` of
/// the twenty at `S = i/10`, and the three more at `S = 1/2` with `G = −1,
/// 1/4, 2`; the body at bell 6 and the body of another epoch are not in it. On G at `S = 1/2`, `[0, 1)` holds `G = 0` and
/// `G = 1/4`. Each range is exactly the bodies a test of every body
/// returns.
#[test]
fn range_on_one_lane_returns_exactly_the_tested_bodies() {
    let r = root();
    let mut bodies = Bodies::new();
    for i in 0..20 {
        bodies.hold(body(&r, 5, on_s(q(i, 10)), 0));
    }
    bodies.hold(body(&r, 6, on_s(q(1, 2)), 0));
    bodies.hold(body(&Origin { epoch: 0x93, ..root() }, 5, on_s(q(1, 2)), 0));
    for g in [q(-1, 1), q(1, 4), q(2, 1)] {
        bodies.hold(body(&r, 5, [q(1, 2), g, q(0, 1), q(0, 1)], 0));
    }
    let at = body(&r, 5, on_s(q(0, 1)), 0);
    let found = bodies.range(&at, Lane::S, &q(1, 2), &q(9, 10));
    let s: Vec<Q> = found.bodies.iter().map(|c| c.lane(Lane::S).clone()).collect();
    assert_eq!(s, [q(1, 2), q(1, 2), q(1, 2), q(1, 2), q(3, 5), q(7, 10), q(4, 5)]);
    assert_eq!(found.bodies.iter().collect::<Vec<_>>(), tested(bodies.held(), &at, Lane::S, &q(1, 2), &q(9, 10)));
    let at_half = body(&r, 5, on_s(q(1, 2)), 0);
    let on_g = bodies.range(&at_half, Lane::G, &q(0, 1), &q(1, 1));
    let g: Vec<Q> = on_g.bodies.iter().map(|c| c.lane(Lane::G).clone()).collect();
    assert_eq!(g, [q(0, 1), q(1, 4)]);
    assert!(bodies.range(&at, Lane::S, &q(9, 10), &q(1, 2)).bodies.is_empty(), "an empty span holds no body");
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
            b.hold(body(&r, 5, on_s(q(i, n)), 0));
            b.hold(body(&r, 6, on_s(q(i, n)), 0));
        }
        b
    };
    let at = body(&r, 5, on_s(q(0, 1)), 0);
    let (small, large) = (held(64), held(256));
    let digits = |b: &Bodies| usize::BITS - b.held().len().leading_zeros();
    assert_eq!(digits(&large), digits(&small) + 2, "4N holds two binary digits more than N");
    for (lo, hi) in [(q(0, 1), q(1, 1)), (q(1, 3), q(7, 20)), (q(-1, 2), q(1, 8)), (q(9, 10), q(2, 1))] {
        let (a, b) = (small.range(&at, Lane::S, &lo, &hi), large.range(&at, Lane::S, &lo, &hi));
        assert_eq!(a.bodies.iter().collect::<Vec<_>>(), tested(small.held(), &at, Lane::S, &lo, &hi));
        assert_eq!(b.bodies.iter().collect::<Vec<_>>(), tested(large.held(), &at, Lane::S, &lo, &hi));
        assert!(a.probes <= 2 * digits(&small), "{lo}..{hi}: {} probes at N", a.probes);
        assert!(b.probes <= 2 * digits(&large), "{lo}..{hi}: {} probes at 4N", b.probes);
    }
}
