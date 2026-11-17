// 78.185 — the adjunction and the retraction of the cover, the fixtures of
// §7: a nerve of three nodes and an entrant.

use crate::cover::{CoverRefusal, Nerve, Overlap, Tether};
use crate::turning::{compose, motion};
use crate::Q;
use algebra::{base_axis, Clifford};

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// The positive unit of plane `k`.
fn unit(k: usize) -> usize {
    base_axis(2 * k - 1).expect("the algebra holds the unit")
}

/// The signature of planes `1..=3`: each negative unit.
fn signature() -> usize {
    (1..=3).map(|k| base_axis(2 * k).expect("the unit")).fold(0, |s, g| s | g)
}

/// The motion of rate `c` on the plane of positive units `a` and `b`.
fn turning(c: Q, a: usize, b: usize) -> Clifford {
    let sig = signature();
    let rate = Clifford::blade(sig, 0, c)
        .expect("a scalar")
        .product(&Clifford::blade(sig, unit(a), q(1, 1)).expect("a unit"))
        .and_then(|w| w.product(&Clifford::blade(sig, unit(b), q(1, 1)).expect("a unit")))
        .expect("one signature");
    motion(&rate).expect("a motion")
}

fn key(b: u8) -> [u8; 32] {
    [b; 32]
}

/// Nodes a, b, c overlapping pairwise with consistent motions, and d
/// overlapping c alone.
fn nerve() -> (Nerve, Clifford, Clifford) {
    let (r1, r2) = (turning(q(1, 2), 1, 2), turning(q(1, 3), 2, 3));
    let ac = compose(&r1, &r2).expect("composed");
    let overlaps = vec![
        Overlap { a: key(1), b: key(2), g: r1.clone() },
        Overlap { a: key(2), b: key(3), g: r2.clone() },
        Overlap { a: key(1), b: key(3), g: ac },
        Overlap { a: key(3), b: key(4), g: r1.clone() },
    ];
    (Nerve { nodes: vec![key(1), key(2), key(3), key(4)], overlaps }, r1, r2)
}

/// AN ENTRANT LINKED AT REST TO TWO OVERLAPPING NODES IS ADJOINED, and its
/// retraction keeps the nerve whole. Off rest it is untethered; with the
/// readings disagreeing on its triangle the residue is named; linked to two
/// nodes that do not overlap it would open a cycle; and retracting the only
/// node d overlaps disconnects the nerve.
#[test]
fn an_adjunction_keeps_the_classes_and_a_retraction_the_nerve_whole() {
    let (n0, r1, r2) = nerve();
    let entrant = key(9);
    let rest = vec![(q(1, 3), q(-1, 3))];
    // a → n stated as a → b → n: g_an = g_ab · g_bn.
    let g_bn = r2.clone();
    let g_an = compose(&r1, &g_bn).expect("composed");
    let link = vec![Tether { node: key(1), g: g_an.clone(), planes: rest.clone() }, Tether { node: key(2), g: g_bn.clone(), planes: Vec::new() }];
    let n1 = n0.adjoin(entrant, &link).expect("adjoined: a simplex, the cocycle closed, a tether at rest");
    assert_eq!(n1.nodes.len(), 5);
    assert!(n1.retract(&entrant).is_ok(), "its retraction keeps the nerve whole");

    let off = vec![Tether { node: key(1), g: g_an.clone(), planes: vec![(q(1, 3), q(-1, 5))] }];
    assert!(matches!(n0.adjoin(entrant, &off), Err(CoverRefusal::Untethered)), "a tether off rest links nothing");

    let wrong = vec![Tether { node: key(1), g: r1.clone(), planes: rest.clone() }, Tether { node: key(2), g: g_bn, planes: Vec::new() }];
    assert!(matches!(n0.adjoin(entrant, &wrong), Err(CoverRefusal::Cocycle { .. })), "the readings disagree on (a, b, n): the residue named");

    let apart = vec![Tether { node: key(1), g: g_an, planes: rest.clone() }, Tether { node: key(4), g: r2, planes: Vec::new() }];
    assert!(matches!(n0.adjoin(entrant, &apart), Err(CoverRefusal::OpensCycle { a, b }) if a == key(1) && b == key(4)), "a and d do not overlap: a new cycle");

    assert!(matches!(n0.retract(&key(3)), Err(CoverRefusal::Disconnects)), "d overlaps c alone");
}
