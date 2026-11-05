// A9, MU-DERIVATIONS D3.3 — the carrying set: every term of the closure that
// carries a footprint, keyed and never cut, read on a metered token that
// refuses the whole set when spent.

use crate::transducer::anchor::*;
use crate::transducer::sentence::tests::fixtures::sites;
use crate::transducer::spelled::{PaintedSite, SectionSite};

/// A scene of four hues in four 2×2 blocks, each block its own component.
fn quilt() -> Vec<PaintedSite> {
    (0..4).flat_map(|r| (0..4).map(move |c| PaintedSite::on_plane([r, c], u8::try_from(1 + 2 * (r / 2) + c / 2).expect("a hue")))).collect()
}

/// The site at the origin of the plane.
fn origin() -> SectionSite {
    SectionSite::on_plane([0, 0])
}

fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

/// THE CARRYING SET IS THE CLOSURE'S CARRIERS, EVERY ONE: each term held
/// carries the footprint, and the set is exactly the candidates whose
/// evaluation carries it, one per key.
#[test]
fn carrying_set_count_equals_closure() {
    let scene = quilt();
    let fp = sites(&[(0, 0)]);
    let set = derive_carrying_all(&fp, &scene, &mut stroke(1 << 20)).expect("read");
    for (_, t) in set.scoped() {
        assert!(t.eval(&scene).contains(&origin()), "{t:?} carries the footprint");
    }
    let mut keys: Vec<Vec<u8>> = candidates(&fp, &scene)
        .into_iter()
        .filter(|t| t.eval(&scene).contains(&origin()))
        .map(|t| t.key().as_bytes().to_vec())
        .collect();
    keys.sort_unstable();
    keys.dedup();
    assert_eq!(set.len(), keys.len(), "every carrier of the closure, one per key");
}

/// THE THIRTY-THIRD TERM IS KEPT: a footprint the closure carries with more
/// than thirty-two terms keeps them all, and two such evidences bind every
/// one of them in common — nothing past a count is cut.
#[test]
fn carrying_set_keeps_thirty_third_term() {
    let scene = quilt();
    let fp = sites(&[(0, 0)]);
    let set = derive_carrying_all(&fp, &scene, &mut stroke(1 << 20)).expect("read");
    assert!(set.len() > 32, "the fixture's closure carries more than thirty-two terms: {}", set.len());
    let family = vec![(fp.clone(), scene.clone()), (fp, scene)];
    let bound = crate::court::verdict::derive_pi(&family, &[], &mut stroke(1 << 22)).expect("read");
    assert_eq!(bound.len(), set.len(), "every common term held");
}

/// A SPENT TOKEN REFUSES THE WHOLE SET: a token too short for the closure
/// returns no partial set, naming how many terms it read.
#[test]
fn carrying_set_refuses_spent_token_whole() {
    let scene = quilt();
    let fp = sites(&[(0, 0)]);
    let reply = derive_carrying_all(&fp, &scene, &mut stroke(3));
    assert!(matches!(reply, Err(CarryingRefusal::Unread { terms_read }) if terms_read < candidates(&fp, &scene).len()), "{reply:?}");
}
