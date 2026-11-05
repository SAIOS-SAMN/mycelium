// LAW-COORDINATE v3 (proposal 78.173 §5, fixtures 7–10 on one-component
// lanes, H in degrees): the advance of the lanes in the bell and its
// composition, an event's bell, the merger with its offsets, and fission as
// the merger's inverse.

use super::bodies::*;
use crate::coordinate::*;

/// The body at the root, bell `h`, lanes `S:G:T:H`, `K`.
fn at_root(h: u64, lanes: Lanes, k: Vec<u64>) -> Coordinate {
    body(&root(), h, lanes, k)
}

/// Fixture 7's body: bell 10, `S = 1/3`, `H = 90°` on `e_Z1Q1`, `K = 0`.
fn moving() -> Coordinate {
    at_root(10, lanes(q(1, 3), q(0, 1), q(0, 1), q(90, 1)), vec![0])
}

/// Fixture 7's rates: `(1/7, 0, 1/2, 72°)` per bell.
fn rates() -> Rates {
    Rates { lanes: lanes(q(1, 7), q(0, 1), q(1, 2), q(72, 1)) }
}

/// FIXTURE 7, THE ADVANCE: from bell 10 to bell 40 each component moves by
/// its rate times 30, `S = 1/3 + 30/7 = 97/21`, `T = 15`,
/// `θ = 90° + 72°·30 = 2250°`, and `K = ⌊2250/360⌋ = 6`.
#[test]
fn advance_equals_closed_form_lanes() {
    let advanced = moving().advance(&rates(), 40).expect("a forward advance");
    assert_eq!(advanced, at_root(40, lanes(q(97, 21), q(0, 1), q(15, 1), q(2250, 1)), vec![6]));
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

/// FIXTURE 7, RATES OF ANOTHER SHAPE ARE REFUSED: rates keyed on H's
/// `e_Z2Q2` against a body on `e_Z1Q1` are refused `ShapesDiffer`, naming
/// both shapes.
#[test]
fn advance_refuses_other_shape() {
    let [ks, kg, kt, kh] = keys();
    let other = plane(('Z', 2), ('Q', 2));
    let zero = || q(0, 1);
    let rates = Rates { lanes: Lanes::base(Vector::one(ks, zero()), Vector::one(kg, zero()), Vector::one(kt, zero()), Vector::one(other.clone(), zero())).expect("the base lanes") };
    assert_eq!(moving().advance(&rates, 11), Err(MotionRefusal::ShapesDiffer(Box::new(Shapes { lane: Lane::H, held: vec![kh], asked: vec![other] }))));
}

/// `K` reads the whole turns of H in both senses: `90°` falling `180°` a
/// bell for 5 bells stands at `−810°`, `K = 2`.
#[test]
fn advance_reads_orbits_in_both_senses() {
    let falling = Rates { lanes: lanes(q(0, 1), q(0, 1), q(0, 1), q(-180, 1)) };
    let advanced = at_root(0, lanes(q(0, 1), q(0, 1), q(0, 1), q(90, 1)), vec![0]).advance(&falling, 5).expect("a forward advance");
    assert_eq!(value(&advanced, &Lane::H), &q(-810, 1));
    assert_eq!(advanced.local.k, vec![2]);
}

/// Fixture 8's body: bell 4, `S = 0`.
fn waiting() -> Coordinate {
    at_root(4, lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0])
}

/// Fixture 8's rates: `r_S = 2/3`, `r_G = 0`.
fn s_rates() -> Rates {
    Rates { lanes: lanes(q(2, 3), q(0, 1), q(0, 1), q(0, 1)) }
}

/// FIXTURE 8, AN EVENT'S BELL: S reaches 1 at `4 + 1/(2/3) = 11/2`, between
/// whole bells: S stands at `2/3` at bell 5 and at `4/3` at bell 6.
#[test]
fn bell_of_equals_first_degree_solution() {
    assert_eq!(waiting().bell_of(&Lane::S, &on('S', 1), &q(1, 1), &s_rates()), Ok(EventBell::At(q(11, 2))));
    let at = |t: u64| value(&waiting().advance(&s_rates(), t).expect("a forward advance"), &Lane::S).clone();
    assert_eq!((at(5), at(6)), (q(2, 3), q(4, 3)));
}

/// FIXTURE 8, NONE ON AN UNMOVING COMPONENT: with `r_G = 0`, G reaches 1
/// at no bell.
#[test]
fn bell_of_refuses_unmoving_lane() {
    assert_eq!(waiting().bell_of(&Lane::G, &on('G', 1), &q(1, 1), &s_rates()), Ok(EventBell::Unmoving));
}

/// FIXTURE 8, NONE BEHIND THE BELL: `v = −1` under `r_S = 2/3` lies behind
/// bell 4, at `5/2`, which no advance reaches; a component not held is
/// refused `NoComponent`.
#[test]
fn bell_of_refuses_value_behind_bell() {
    assert_eq!(waiting().bell_of(&Lane::S, &on('S', 1), &q(-1, 1), &s_rates()), Ok(EventBell::Behind { bell: q(5, 2) }));
    assert_eq!(
        waiting().bell_of(&Lane::S, &on('S', 2), &q(1, 1), &s_rates()),
        Err(MotionRefusal::NoComponent { lane: Lane::S, key: on('S', 2) })
    );
}

/// Fixture 9's bodies at bell 5: `a = (S 1, H 720°)`, `K = 2`;
/// `b = (S 3, H 0°)`, `K = 0`.
fn pair() -> (Coordinate, Coordinate) {
    (at_root(5, lanes(q(1, 1), q(0, 1), q(0, 1), q(720, 1)), vec![2]), at_root(5, lanes(q(3, 1), q(0, 1), q(0, 1), q(0, 1)), vec![0]))
}

/// FIXTURE 9, A MERGER WITH OFFSETS: `a` at holding 1 and `b` at holding 3
/// merge to `S = 5/2`, `H = 180°`, `K = 0`, with offsets `a (S −3/2,
/// H 540°)`, `b (S 1/2, H −180°)`, whose holding-weighted sum is zero.
#[test]
fn merge_equals_holding_weighted_lanes() {
    let (a, b) = pair();
    let merger = merge(&a, &q(1, 1), &b, &q(3, 1)).expect("one origin, one bell, a holding");
    assert_eq!(merger.point, at_root(5, lanes(q(5, 2), q(0, 1), q(0, 1), q(180, 1)), vec![0]));
    assert_eq!(merger.offset_a, lanes(q(-3, 2), q(0, 1), q(0, 1), q(540, 1)));
    assert_eq!(merger.offset_b, lanes(q(1, 2), q(0, 1), q(0, 1), q(-180, 1)));
    for (lane, (_, oa)) in Lane::BASE.iter().zip(merger.offset_a.held()) {
        let ob = merger.offset_b.get(lane).expect("a base lane");
        let mut sum = oa.components().iter().zip(ob.components()).map(|((_, x), (_, y))| x + q(3, 1) * y);
        assert!(sum.all(|s| s == q(0, 1)), "lane {lane:?}");
    }
}

/// FIXTURE 9, A POINT MERGED WITH ITSELF IS ITSELF, with zero offsets.
#[test]
fn merge_keeps_point_merged_with_itself() {
    let (a, _) = pair();
    let merger = merge(&a, &q(2, 1), &a, &q(5, 1)).expect("one origin, one bell, a holding");
    assert_eq!(merger.point, a);
    let zero = lanes(q(0, 1), q(0, 1), q(0, 1), q(0, 1));
    assert_eq!((merger.offset_a, merger.offset_b), (zero.clone(), zero));
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
/// read at holdings 1 and 1 sum to `(S −1, H 360°)`, refused `Unbalanced`
/// with that residual; holdings summing to zero are refused `NoHolding`.
#[test]
fn fission_refuses_unbalanced_offsets() {
    let (a, b) = pair();
    let merger = merge(&a, &q(1, 1), &b, &q(3, 1)).expect("one origin, one bell, a holding");
    assert_eq!(
        fission(&merger, &q(1, 1), &q(1, 1)),
        Err(MotionRefusal::Unbalanced { residual: Box::new(lanes(q(-1, 1), q(0, 1), q(0, 1), q(360, 1))) })
    );
    assert_eq!(fission(&merger, &q(2, 1), &q(-2, 1)), Err(MotionRefusal::NoHolding { a: Box::new(q(2, 1)), b: Box::new(q(-2, 1)) }));
}
