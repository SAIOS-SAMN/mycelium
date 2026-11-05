// LAW-COORDINATE v1 (proposal 78.173 §5, fixtures 7 and 10) over generated
// exact-ℚ inputs: the advance composes, and fission inverts the merger.

use crate::coordinate::*;
use crate::Q;
use num_traits::Signed;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// Generated signed lanes: the `n`-th of a run of exact rationals of both
/// signs and several denominators.
fn lanes(n: i64) -> [Q; 4] {
    [q(n - 3, 7), q(2 * n + 1, 5), q(-n, 3), q(5 * n - 11, 4)]
}

/// The body at the root, bell `h`, `lanes`, with `K` the whole turns of H.
fn body(h: u64, lanes: [Q; 4]) -> Coordinate {
    let k = lanes[Lane::H.place()].abs().floor().to_integer().try_into().expect("K within u64");
    Coordinate { origin: Origin { genesis: [0x6a; 32], epoch: 0x92, enclosures: vec![] }, local: Local { h, lanes, k } }
}

/// THE ADVANCE COMPOSES over generated bodies, rates and bells
/// `h ≤ a ≤ t`: `h → a → t` is `h → t`.
#[test]
fn advance_agrees_with_two_step_advance_over_generated_cases() {
    for n in 0..12 {
        let p = body(3, lanes(n));
        let rates = Rates { lanes: lanes(7 - n) };
        for (a, t) in [(3, 3), (3, 9), (5, 9), (9, 9), (4, 31)] {
            let once = p.advance(&rates, t);
            let twice = p.advance(&rates, a).and_then(|m| m.advance(&rates, t));
            assert_eq!(twice, once, "case {n}, {a}, {t}");
            assert!(once.is_ok(), "case {n}: a forward advance");
        }
    }
}

/// FISSION INVERTS THE MERGER over generated pairs of bodies at one origin
/// and bell, at holdings of both signs whose sum is not zero: the merged
/// point, the holdings and the offsets give both bodies back, and the
/// holding-weighted sum of the offsets is zero on every lane.
#[test]
fn fission_inverts_merge_over_generated_cases() {
    let holdings = [(q(1, 1), q(3, 1)), (q(2, 5), q(7, 3)), (q(-1, 2), q(3, 2)), (q(5, 1), q(-2, 1)), (q(1, 1), q(0, 1))];
    for n in 0..12 {
        let (a, b) = (body(17, lanes(n)), body(17, lanes(3 * n - 5)));
        for (w_a, w_b) in &holdings {
            let merger = merge(&a, w_a, &b, w_b).expect("one origin, one bell, a holding");
            for (lane, (oa, ob)) in Lane::ORDER.iter().zip(merger.offset_a.iter().zip(&merger.offset_b)) {
                assert_eq!(w_a * oa + w_b * ob, q(0, 1), "case {n}, lane {lane:?}");
            }
            assert_eq!(fission(&merger, w_a, w_b), Ok((a.clone(), b.clone())), "case {n}, holdings {w_a}, {w_b}");
        }
    }
}
