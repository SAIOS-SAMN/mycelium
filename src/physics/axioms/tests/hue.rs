// 70.4 — hue is the gauge a body carries, and contact is the diagonal.

use crate::transducer::blade;

/// H1, 70.4 — A TWO-COLOURED OBJECT IS ONE BODY.
///
/// Chapter 20's `A BODY IS ONE HUE` shattered it into two, which is
/// monochrome perception: a body seen one colour at a time. The hue
/// is the gauge the body CARRIES — `Blade.gauge` holds it as sorted
/// `(hue, count)` relations — and not the body's identity.
#[test]
fn h1_a_two_coloured_object_is_one_body() {
    // A 2×2 block whose left column is hue 1 and right column hue 2.
    let scene: Vec<crate::transducer::blade::Lamina> =
        vec![(0, 0, 0, 1), (1, 0, 0, 1), (0, 1, 0, 2), (1, 1, 0, 2)];
    let comps = blade::components(&scene);
    assert_eq!(comps.len(), 1, "one object, whatever its colours: {comps:?}");
    assert_eq!(comps[0].len(), 4, "and it carries all four laminae");
    // The colours are not lost — they ride in the gauge.
    let body = blade::lift(&comps[0]);
    assert_eq!(body.gauge.len(), 2, "two hues carried as gauge: {:?}", body.gauge);
}

/// H2, 70.4 — INTER-BODY CONTACT IS THE DIAGONAL, AND IT MUST NOT
/// MERGE WHAT IT RELATES.
///
/// Under H1 two distinct bodies can never be 4-adjacent — they would
/// be one body — so 4-adjacency is strictly interior and contact needs
/// its own relation. The corner meeting is exactly what 4-adjacency
/// leaves available.
///
/// THE LOAD-BEARING HALF IS THE SEPARATION: if the diagonal entered
/// the component relation it would swallow the contact it exposes.
#[test]
fn h2_two_bodies_meet_at_a_corner_without_becoming_one() {
    use crate::transducer::sentence::incidence_diagonal_3d;
    let scene: Vec<crate::transducer::blade::Lamina> =
        vec![(0, 0, 0, 1), (1, 1, 0, 2)];
    // Still two bodies: the diagonal is NOT in the component relation.
    assert_eq!(
        blade::components(&scene).len(),
        2,
        "a corner meeting must not merge them"
    );
    // And they ARE in contact, by the relation that carries contact.
    let positions: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (1, 1, 0)];
    assert_eq!(
        incidence_diagonal_3d(&positions).len(),
        1,
        "the corner meeting is one contact edge"
    );
    // The anti-diagonal is read too, and needs its own ordering.
    let anti: Vec<(i64, i64, i64)> = vec![(0, 1, 0), (1, 0, 0)];
    assert_eq!(
        incidence_diagonal_3d(&anti).len(),
        1,
        "(r+1, c-1) is a corner meeting as much as (r+1, c+1)"
    );
    // A 4-adjacent pair is NOT a diagonal contact: interior, not contact.
    let side: Vec<(i64, i64, i64)> = vec![(0, 0, 0), (0, 1, 0)];
    assert!(
        incidence_diagonal_3d(&side).is_empty(),
        "an edge-sharing pair is one body's interior, never a contact"
    );
}
