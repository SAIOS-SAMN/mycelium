// LAW-COORDINATE v3 (proposal 78.173 §5, fixtures 7, 10 and 13) over
// generated exact-ℚ inputs on vector lanes: the advance composes, and
// fission inverts the merger, per component.

use super::bodies::*;
use crate::coordinate::*;
use crate::Q;
use num_traits::{Signed, ToPrimitive};

/// Generated signed lanes: the `n`-th of a run of exact rationals of both
/// signs and several denominators, S on two axes and H on two frame planes
/// and the turning between them, in degrees.
fn lanes_of(n: i64) -> Lanes {
    let v = |c: Vec<(Key, Q)>| Vector::of(c).expect("distinct keys");
    let h = v(vec![
        (plane(('Z', 1), ('Q', 1)), q(5 * n - 11, 4) * q(360, 1)),
        (plane(('Z', 1), ('Z', 2)), q(7 - 2 * n, 3)),
        (plane(('Z', 2), ('Q', 2)), q(97 * n, 5)),
    ]);
    let s = v(vec![(on('S', 1), q(n - 3, 7)), (on('S', 2), q(1 - n, 9))]);
    Lanes::base(s, Vector::one(on('G', 1), q(2 * n + 1, 5)), Vector::one(on('T', 1), q(-n, 3)), h).expect("the base lanes")
}

/// The body at the root, bell `h`, `lanes`, with `K` the whole turns of each
/// H component.
fn at(h: u64, lanes: Lanes) -> Coordinate {
    let k = lanes
        .get(&Lane::H)
        .expect("H")
        .components()
        .iter()
        .map(|(_, theta)| (theta.abs() / q(360, 1)).floor().to_integer().to_u64().expect("K within u64"))
        .collect();
    body(&root(), h, lanes, k)
}

/// THE ADVANCE COMPOSES over generated bodies, rates and bells
/// `h ≤ a ≤ t`: `h → a → t` is `h → t`, per component.
#[test]
fn advance_agrees_with_two_step_advance_over_generated_cases() {
    for n in 0..12 {
        let p = at(3, lanes_of(n));
        let rates = Rates { lanes: lanes_of(7 - n) };
        for (a, t) in [(3, 3), (3, 9), (5, 9), (9, 9), (4, 31)] {
            let once = p.advance(&rates, t);
            let twice = p.advance(&rates, a).and_then(|m| m.advance(&rates, t));
            assert_eq!(twice, once, "case {n}, {a}, {t}");
            assert_eq!(once.map(|c| c.local.k.len()), Ok(3), "case {n}: a forward advance, K per H component");
        }
    }
}

/// FISSION INVERTS THE MERGER over generated pairs of bodies at one origin
/// and bell, at holdings of both signs whose sum is not zero: the merged
/// point, the holdings and the offsets give both bodies back, and the
/// holding-weighted sum of the offsets is zero on every component.
#[test]
fn fission_inverts_merge_over_generated_cases() {
    let holdings = [(q(1, 1), q(3, 1)), (q(2, 5), q(7, 3)), (q(-1, 2), q(3, 2)), (q(5, 1), q(-2, 1)), (q(1, 1), q(0, 1))];
    for n in 0..12 {
        let (a, b) = (at(17, lanes_of(n)), at(17, lanes_of(3 * n - 5)));
        for (w_a, w_b) in &holdings {
            let merger = merge(&a, w_a, &b, w_b).expect("one origin, one bell, a holding");
            for ((lane, oa), (_, ob)) in merger.offset_a.held().iter().zip(merger.offset_b.held()) {
                for ((key, x), (_, y)) in oa.components().iter().zip(ob.components()) {
                    assert_eq!(w_a * x + w_b * y, q(0, 1), "case {n}, {lane:?} {}", key.written());
                }
            }
            assert_eq!(fission(&merger, w_a, w_b), Ok((a.clone(), b.clone())), "case {n}, holdings {w_a}, {w_b}");
        }
    }
}
