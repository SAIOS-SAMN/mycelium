// LAW-COORDINATE v1 (proposal 78.173 §5, fixtures 7–10): the advance of the
// lanes in the bell and its composition, an event's bell, the merger with
// its offsets, and fission as the merger's inverse.

use crate::coordinate::*;
use crate::Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// The fixtures' universe root: genesis `6a` × 32, epoch `0x92`.
fn root() -> Origin {
    Origin { genesis: [0x6a; 32], epoch: 0x92, enclosures: vec![] }
}

/// The body at the root, bell `h`, lanes `S:G:T:H`, `K`.
fn body(h: u64, lanes: [Q; 4], k: u64) -> Coordinate {
    Coordinate { origin: root(), local: Local { h, lanes, k } }
}

/// Fixture 7's body: bell 10, `S = 1/3`, `H = 1/4`, `K = 0`.
fn moving() -> Coordinate {
    body(10, [q(1, 3), q(0, 1), q(0, 1), q(1, 4)], 0)
}

/// Fixture 7's rates: `(1/7, 0, 1/2, 1/5)`.
fn rates() -> Rates {
    Rates { lanes: [q(1, 7), q(0, 1), q(1, 2), q(1, 5)] }
}

/// FIXTURE 7, THE ADVANCE: from bell 10 to bell 40 each lane moves by its
/// rate times 30, `S = 1/3 + 30/7`, `T = 15`, `H = 25/4`, and `K` is the
/// whole turns of H, 6.
#[test]
fn advance_equals_closed_form_lanes() {
    let advanced = moving().advance(&rates(), 40).expect("a forward advance");
    assert_eq!(advanced, body(40, [q(1, 3) + q(30, 7), q(0, 1), q(15, 1), q(25, 4)], 6));
}

/// FIXTURE 7, THE ADVANCE COMPOSES: 10 → 22 → 40 reaches the point
/// 10 → 40 reaches.
#[test]
fn advance_agrees_with_two_step_advance() {
    let once = moving().advance(&rates(), 40).expect("a forward advance");
    let twice = moving().advance(&rates(), 22).and_then(|p| p.advance(&rates(), 40)).expect("two forward advances");
    assert_eq!(twice, once);
}

/// FIXTURE 7, NO BACKWARD ADVANCE: bell 9 from bell 10 is refused
/// `Backward`, naming both bells.
#[test]
fn advance_refuses_backward_bell() {
    assert_eq!(moving().advance(&rates(), 9), Err(MotionRefusal::Backward { held: 10, asked: 9 }));
}

/// `K` reads the whole turns of H in both senses: H advanced to `−9/4`
/// reads `K = 2`.
#[test]
fn advance_reads_orbits_in_both_senses() {
    let falling = Rates { lanes: [q(0, 1), q(0, 1), q(0, 1), q(-1, 2)] };
    let advanced = body(0, [q(0, 1), q(0, 1), q(0, 1), q(1, 4)], 0).advance(&falling, 5).expect("a forward advance");
    assert_eq!(advanced.lane(Lane::H), &q(-9, 4));
    assert_eq!(advanced.local.k, 2);
}

/// Fixture 8's body: bell 4, `S = 0`.
fn waiting() -> Coordinate {
    body(4, [q(0, 1), q(0, 1), q(0, 1), q(0, 1)], 0)
}

/// Fixture 8's rates: `r_S = 2/3`, `r_G = 0`.
fn s_rates() -> Rates {
    Rates { lanes: [q(2, 3), q(0, 1), q(0, 1), q(0, 1)] }
}

/// FIXTURE 8, AN EVENT'S BELL: S reaches 1 at `4 + 1/(2/3) = 11/2`, between
/// whole bells: S stands at `2/3` at bell 5 and at `4/3` at bell 6.
#[test]
fn bell_of_equals_first_degree_solution() {
    assert_eq!(waiting().bell_of(Lane::S, &q(1, 1), &s_rates()), EventBell::At(q(11, 2)));
    let at = |t: u64| waiting().advance(&s_rates(), t).expect("a forward advance").lane(Lane::S).clone();
    assert_eq!((at(5), at(6)), (q(2, 3), q(4, 3)));
}

/// FIXTURE 8, NONE ON AN UNMOVING LANE: with `r_G = 0`, G reaches 1 at no
/// bell.
#[test]
fn bell_of_refuses_unmoving_lane() {
    assert_eq!(waiting().bell_of(Lane::G, &q(1, 1), &s_rates()), EventBell::Unmoving);
}

/// FIXTURE 8, NONE BEHIND THE BELL: `v = −1` under `r_S = 2/3` lies behind
/// bell 4, at `5/2`, which no advance reaches.
#[test]
fn bell_of_refuses_value_behind_bell() {
    assert_eq!(waiting().bell_of(Lane::S, &q(-1, 1), &s_rates()), EventBell::Behind { bell: q(5, 2) });
}

/// Fixture 9's bodies at bell 5: `a = (S 1, H 2)`, `K = 2`;
/// `b = (S 3, H 0)`, `K = 0`.
fn pair() -> (Coordinate, Coordinate) {
    (body(5, [q(1, 1), q(0, 1), q(0, 1), q(2, 1)], 2), body(5, [q(3, 1), q(0, 1), q(0, 1), q(0, 1)], 0))
}

/// FIXTURE 9, A MERGER WITH OFFSETS: `a` at holding 1 and `b` at holding 3
/// merge to `S = 5/2`, `H = 1/2`, `K = 0`, with offsets `a (S −3/2,
/// H 3/2)`, `b (S 1/2, H −1/2)`, whose holding-weighted sum is zero.
#[test]
fn merge_equals_holding_weighted_lanes() {
    let (a, b) = pair();
    let merger = merge(&a, &q(1, 1), &b, &q(3, 1)).expect("one origin, one bell, a holding");
    assert_eq!(merger.point, body(5, [q(5, 2), q(0, 1), q(0, 1), q(1, 2)], 0));
    assert_eq!(merger.offset_a, [q(-3, 2), q(0, 1), q(0, 1), q(3, 2)]);
    assert_eq!(merger.offset_b, [q(1, 2), q(0, 1), q(0, 1), q(-1, 2)]);
    for (lane, (oa, ob)) in Lane::ORDER.iter().zip(merger.offset_a.iter().zip(&merger.offset_b)) {
        assert_eq!(oa + q(3, 1) * ob, q(0, 1), "lane {lane:?}");
    }
}

/// FIXTURE 9, A POINT MERGED WITH ITSELF IS ITSELF, with zero offsets.
#[test]
fn merge_keeps_point_merged_with_itself() {
    let (a, _) = pair();
    let merger = merge(&a, &q(2, 1), &a, &q(5, 1)).expect("one origin, one bell, a holding");
    assert_eq!(merger.point, a);
    assert_eq!((merger.offset_a, merger.offset_b), (<[Q; 4]>::default(), <[Q; 4]>::default()));
}

/// FIXTURE 9, REFUSED ACROSS ORIGINS: a body of epoch `0x93` merged with
/// one of the root is refused `OriginsDiffer`, naming both.
#[test]
fn merge_refuses_two_origins() {
    let (a, b) = pair();
    let other = Origin { epoch: 0x93, ..root() };
    let b = Coordinate { origin: other.clone(), ..b };
    assert_eq!(merge(&a, &q(1, 1), &b, &q(3, 1)), Err(MotionRefusal::OriginsDiffer { held: Box::new(root()), asked: Box::new(other) }));
}

/// FIXTURE 9, REFUSED ACROSS BELLS: bells 5 and 6 are refused
/// `BellsDiffer`.
#[test]
fn merge_refuses_two_bells() {
    let (a, b) = pair();
    let b = Coordinate { local: Local { h: 6, ..b.local }, ..b };
    assert_eq!(merge(&a, &q(1, 1), &b, &q(3, 1)), Err(MotionRefusal::BellsDiffer { held: 5, asked: 6 }));
}

/// FIXTURE 9, REFUSED WHERE THE HOLDINGS SUM TO ZERO: holdings 1 and −1
/// are refused `NoHolding`.
#[test]
fn merge_refuses_cancelling_holdings() {
    let (a, b) = pair();
    assert_eq!(merge(&a, &q(1, 1), &b, &q(-1, 1)), Err(MotionRefusal::NoHolding { a: Box::new(q(1, 1)), b: Box::new(q(-1, 1)) }));
}

/// FIXTURE 10, FISSION IS THE INVERSE OF MERGER: the witness's merged
/// point, holdings and offsets give back `a` and `b` exactly.
#[test]
fn fission_inverts_merge() {
    let (a, b) = pair();
    let merger = merge(&a, &q(1, 1), &b, &q(3, 1)).expect("one origin, one bell, a holding");
    assert_eq!(fission(&merger, &q(1, 1), &q(3, 1)), Ok((a, b)));
}

/// FIXTURE 10, OFFSETS NO MERGER LEFT ARE REFUSED: the witness's offsets
/// read at holdings 1 and 1 sum to `(S −1, H 1)`, refused `Unbalanced`
/// with that residual; holdings summing to zero are refused `NoHolding`.
#[test]
fn fission_refuses_unbalanced_offsets() {
    let (a, b) = pair();
    let merger = merge(&a, &q(1, 1), &b, &q(3, 1)).expect("one origin, one bell, a holding");
    assert_eq!(fission(&merger, &q(1, 1), &q(1, 1)), Err(MotionRefusal::Unbalanced { residual: Box::new([q(-1, 1), q(0, 1), q(0, 1), q(1, 1)]) }));
    assert_eq!(fission(&merger, &q(2, 1), &q(-2, 1)), Err(MotionRefusal::NoHolding { a: Box::new(q(2, 1)), b: Box::new(q(-2, 1)) }));
}
