// 70.4 — hue is the gauge a body carries, and contact is the diagonal.

use crate::transducer::base::Site;
use crate::transducer::blade::{self, Lamina};

/// Laminae and sites as the sensor hands them over (71.2(1)).
fn laminae<T: Copy + Into<Lamina>>(v: &[T]) -> Vec<Lamina> {
    v.iter().map(|&t| t.into()).collect()
}
fn sites<T: Copy + Into<Site>>(v: &[T]) -> Vec<Site> {
    v.iter().map(|&t| t.into()).collect()
}

/// H1, 70.4 — A TWO-COLOURED OBJECT IS ONE BODY.
///
/// Chapter 20's `A BODY IS ONE HUE` shattered it into two, which is
/// monochrome perception: a body seen one colour at a time. The hue
/// is the gauge the body CARRIES — `Blade.gauge` holds it as sorted
/// `(hue, count)` relations — and not the body's identity.
#[test]
fn h1_a_two_coloured_object_is_one_body() {
    // A 2×2 block whose left column is hue 1 and right column hue 2.
    let scene = laminae(&[(0, 0, 0, 1u8), (1, 0, 0, 1), (0, 1, 0, 2), (1, 1, 0, 2)]);
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
    let scene = laminae(&[(0, 0, 0, 1u8), (1, 1, 0, 2)]);
    // Still two bodies: the diagonal is NOT in the component relation.
    assert_eq!(
        blade::components(&scene).len(),
        2,
        "a corner meeting must not merge them"
    );
    // And they ARE in contact, by the relation that carries contact.
    let positions = sites(&[(0, 0, 0), (1, 1, 0)]);
    assert_eq!(
        incidence_diagonal_3d(&positions).len(),
        1,
        "the corner meeting is one contact edge"
    );
    // The anti-diagonal is read too, and needs its own ordering.
    let anti = sites(&[(0, 1, 0), (1, 0, 0)]);
    assert_eq!(
        incidence_diagonal_3d(&anti).len(),
        1,
        "(r+1, c-1) is a corner meeting as much as (r+1, c+1)"
    );
    // A 4-adjacent pair is NOT a diagonal contact: interior, not contact.
    let side = sites(&[(0, 0, 0), (0, 1, 0)]);
    assert!(
        incidence_diagonal_3d(&side).is_empty(),
        "an edge-sharing pair is one body's interior, never a contact"
    );
}

/// C2, 70.6 — A CUBE SHARES A BOUNDARY WITH TWENTY OF ITS TWENTY-SIX
/// NEIGHBOURS WITHOUT SHARING A FACE. The six it shares a face with make a
/// body and are [`incidence_of_3d`]'s. The twelve it shares an edge with and
/// the eight it shares a corner with are contact, and each comes back once,
/// with the direction from the lesser site to the greater. The frame axis
/// is among them: sixteen of the twenty change `k`.
///
/// [`incidence_of_3d`]: crate::transducer::sentence::incidence_of_3d
#[test]
fn a_cube_shares_boundary_with_twenty_of_its_twenty_six_neighbours() {
    use crate::transducer::sentence::{incidence_diagonal_3d, incidence_of_3d};
    let mut positions: Vec<Site> = Vec::new();
    for dr in -1..=1i64 {
        for dc in -1..=1i64 {
            for dk in -1..=1i64 {
                positions.push(Site::from((dr, dc, dk)));
            }
        }
    }
    let centre = positions.binary_search(&Site::from((0, 0, 0))).expect("the centre is one of the 27");
    let touching = |pairs: Vec<(usize, usize)>| pairs.into_iter().filter(|&(i, j)| i == centre || j == centre).count();
    let shared: Vec<(usize, usize, Vec<i64>)> = incidence_diagonal_3d(&positions)
        .into_iter()
        .filter(|(i, j, _)| *i == centre || *j == centre)
        .collect();
    assert_eq!(shared.len(), 20, "twenty shared edges and corners, each once");
    assert_eq!(
        touching(incidence_of_3d(&positions)),
        6,
        "and the six shared faces are the body's, not contact"
    );
    // Every pair carries the offset from its first site to its second.
    for (i, j, d) in &shared {
        let (p, q) = (&positions[*i], &positions[*j]);
        let offset: Vec<i64> = q.axes().iter().zip(p.axes()).map(|(a, b)| a - b).collect();
        assert_eq!(&offset, d, "the direction is the adjacency's own coordinates");
    }
    let across = shared.iter().filter(|(_, _, d)| d.get(2) != Some(&0)).count();
    assert_eq!(across, 16, "eight edges and eight corners across the frames, four edges within one: {across}");
}
