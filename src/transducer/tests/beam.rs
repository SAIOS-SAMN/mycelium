use crate::transducer::anchor::Selection;
use crate::transducer::beam::*;

/// One coherent domino vs. the same volume scattered: topology tells
/// them apart (β₀, boundary), volume cannot.
#[test]
fn signature_separates_equal_volumes_by_topology() {
    let scene = vec![(0, 0, 1), (0, 1, 1), (5, 5, 1), (7, 7, 2)];
    let coherent = signature(&[(0, 0), (0, 1)], &scene);
    let scattered = signature(&[(0, 0), (5, 5)], &scene);
    assert_eq!(coherent.sites, scattered.sites, "equal volume");
    assert_eq!(coherent.components, 1);
    assert_eq!(scattered.components, 2);
    assert_eq!(coherent.boundary, 6);
    assert_eq!(scattered.boundary, 8);
    assert_eq!(mismatch(&coherent, &coherent), [0; 4], "self-resonance is exact zero");
    assert!(mismatch(&coherent, &scattered)[0] > 0, "β₀ axis names the difference");
}

/// A candidate beaten on every lane is out; arrival order decides
/// nothing. One survivor is a discharge.
#[test]
fn beam_minima_drops_candidate_beaten_on_every_lane() {
    // Scene: a hue-1 domino and a lone hue-2 site.
    let scene = vec![(0, 0, 1), (0, 1, 1), (4, 4, 2)];
    let footprint = vec![(0i64, 0i64), (0, 1)];
    // Candidate A selects EVERYTHING painted (carries fp, but drags the
    // alien site); candidate B selects exactly the hue-1 support, so B
    // is no worse on any lane and better on some.
    let a = Selection::Hue(255);
    let b = Selection::Hue(1);
    assert_eq!(a.eval(&scene).len(), 3);
    assert_eq!(b.eval(&scene).len(), 2);
    let witnesses = vec![(footprint, scene)];
    // A is listed FIRST: arrival order would keep it.
    assert_eq!(
        beam_minima(&[a, b.clone()], &witnesses),
        vec![b.clone()],
        "the dominated candidate leaves and arrival order decides nothing"
    );
    assert_eq!(beam_minima(std::slice::from_ref(&b), &[]), vec![b]);
    assert_eq!(beam_minima(&[], &[]), Vec::new());
}

/// TWO INCOMPARABLE READINGS ARE HELD, not ranked. Each beats the other
/// on a different lane, so neither is out — and the lexicographic sort
/// this replaced would have returned one of them and dropped the other,
/// which is the vortex destroyed at the moment it forms.
#[test]
fn beam_minima_holds_incomparable_candidates() {
    // Two scenes. On the first, hue 1 is the exact footprint and hue 2
    // misses; on the second the footprint is hue 2 and hue 1 misses. The
    // worst pair governs each candidate, so each carries a mismatch the
    // other does not, on different content.
    let s1 = vec![(0, 0, 1), (0, 1, 1), (4, 4, 2)];
    let s2 = vec![(0, 0, 2), (0, 1, 2), (7, 7, 1)];
    let witnesses = vec![
        (vec![(0i64, 0i64), (0, 1)], s1),
        (vec![(0i64, 0i64), (0, 1)], s2),
    ];
    let one = Selection::Hue(1);
    let two = Selection::Hue(2);
    let held = beam_minima(&[one.clone(), two.clone()], &witnesses);
    assert_eq!(
        held.len(),
        2,
        "neither reading beats the other on every lane, so both stand: {held:?}"
    );
    assert!(held.contains(&one) && held.contains(&two));
}
