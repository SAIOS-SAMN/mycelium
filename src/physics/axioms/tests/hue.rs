// 70.4 — hue is the gauge a body carries, and contact is the diagonal.

use crate::transducer::base::Site;
use crate::transducer::blade::{self, Compartment};

/// Compartments and sites as the sensor hands them over (71.2(1)).
fn compartments<T: Copy + Into<Compartment>>(v: &[T]) -> Vec<Compartment> {
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
    let scene = compartments(&[(0, 0, 0, 1u8), (1, 0, 0, 1), (0, 1, 0, 2), (1, 1, 0, 2)]);
    let comps = blade::components(&scene);
    assert_eq!(comps.len(), 1, "one object, whatever its colours: {comps:?}");
    assert_eq!(comps[0].len(), 4, "and it carries all four compartments");
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
    use crate::transducer::sentence::contact_cells;
    let scene = compartments(&[(0, 0, 0, 1u8), (1, 1, 0, 2)]);
    // Still two bodies: the diagonal is NOT in the component relation.
    assert_eq!(
        blade::components(&scene).len(),
        2,
        "a corner meeting must not merge them"
    );
    // And they ARE in contact, by the relation that carries contact.
    let positions = sites(&[(0, 0, 0), (1, 1, 0)]);
    assert_eq!(
        contact_cells(&positions).len(),
        1,
        "the corner meeting is one contact edge"
    );
    // The anti-diagonal is read too, and needs its own ordering.
    let anti = sites(&[(0, 1, 0), (1, 0, 0)]);
    assert_eq!(
        contact_cells(&anti).len(),
        1,
        "(r+1, c-1) is a corner meeting as much as (r+1, c+1)"
    );
    // A 4-adjacent pair is NOT a diagonal contact: interior, not contact.
    let side = sites(&[(0, 0, 0), (0, 1, 0)]);
    assert!(
        contact_cells(&side).is_empty(),
        "an edge-sharing pair is one body's interior, never a contact"
    );
}

/// Every site of the block `{−1, 0, 1}ⁿ` about the origin, sorted.
fn block(n: usize) -> Vec<Site> {
    let mut out: Vec<Vec<i64>> = vec![Vec::new()];
    for _ in 0..n {
        out = out.into_iter().flat_map(|p| (-1..=1i64).map(move |x| [p.clone(), vec![x]].concat())).collect();
    }
    let mut sites: Vec<Site> = out.into_iter().map(|a| Site::new(a).expect("a block site within the algebra")).collect();
    sites.sort_unstable();
    sites
}

/// C2, 70.6, BQ2 — A CUBE SHARES A CELL WITH EVERY NEIGHBOUR, AND THE CELL
/// IS READ OFF THE PAIR. Read about the centre of the block `{−1,0,1}ⁿ` at
/// `n = 3` and `n = 4`: each of the `3ⁿ − 1` neighbours comes back once,
/// carrying its offset, the axes it differs on, and the grade `n − m` of
/// the cell the two cubes share. The shared faces are the body's
/// ([`face_incidence`]); every other grade is contact ([`contact_cells`]);
/// and a cell lies across the frame axis exactly where the frame is among
/// its axes. Nothing here is a count of a neighbourhood written down: the
/// relation is asserted pair by pair.
///
/// [`face_incidence`]: crate::transducer::sentence::face_incidence
/// [`contact_cells`]: crate::transducer::sentence::contact_cells
#[test]
fn a_cube_shares_a_cell_of_its_own_grade_with_every_neighbour() {
    use crate::transducer::base::FRAME_AXIS;
    use crate::transducer::sentence::{contact_cells, face_incidence, shared_cells};
    for n in [3usize, 4] {
        let positions = block(n);
        let origin = Site::new(vec![0; n]).expect("the origin within the algebra");
        let centre = positions.binary_search(&origin).expect("the centre is in its block");
        let about: Vec<_> = shared_cells(&positions).into_iter().filter(|c| c.i == centre || c.j == centre).collect();
        assert_eq!(about.len() + 1, 3usize.pow(u32::try_from(n).expect("n is 3 or 4, the loop's literals")), "every neighbour once, at n = {n}");
        for c in &about {
            let (p, q) = (&positions[c.i], &positions[c.j]);
            let offset: Vec<i64> = q.axes().iter().zip(p.axes()).map(|(a, b)| a - b).collect();
            assert_eq!(offset, c.d, "the offset is the pair's own coordinates");
            let differ: Vec<usize> = (0..n).filter(|&i| offset[i] != 0).collect();
            assert_eq!(c.axes, differ, "the axes are the ones the two sites differ on");
            assert_eq!(c.grade, n - differ.len(), "the grade is n − m");
            assert_eq!(c.is_face(), c.grade == n - 1, "a face is the cell of grade n − 1");
            assert_eq!(c.crosses(FRAME_AXIS), offset[FRAME_AXIS] != 0, "across the frames iff the frame differs");
        }
        let faces = face_incidence(&positions);
        let contacts = contact_cells(&positions);
        for c in &about {
            let pair = (c.i, c.j);
            assert_eq!(faces.contains(&pair), c.is_face(), "a face feeds the body relation and nothing else");
            assert_eq!(contacts.contains(c), !c.is_face(), "every lower grade is contact");
        }
    }
}

/// 71.2(2), BQ2 — A CONTACT IS THE GRADE OF THE SHARED CELL. Two bodies
/// meeting at an edge within a frame, at an edge across two frames and at a
/// corner read grades 1, 1 and 0 at `n = 3`; lifted into `n = 4` by the
/// inclusion (the fourth coordinate zero), the same two sites share a cell
/// one grade higher, since the cubes now also agree on the new axis. The
/// grade is the contact's own, read off the pair, never a kind looked up.
#[test]
fn a_contact_is_the_grade_of_the_shared_cell() {
    use crate::transducer::sentence::contact_cells;
    let grade = |a: Vec<i64>, b: Vec<i64>| -> Vec<usize> {
        let site = |a: Vec<i64>| Site::new(a).expect("a site within the algebra");
        contact_cells(&[site(a), site(b)]).into_iter().map(|c| c.grade).collect()
    };
    assert_eq!(grade(vec![0, 0, 0], vec![1, 1, 0]), vec![1], "an edge within a frame is grade 1");
    assert_eq!(grade(vec![0, 0, 0], vec![0, 1, 1]), vec![1], "an edge across two frames is grade 1");
    assert_eq!(grade(vec![0, 0, 0], vec![1, 1, 1]), vec![0], "a corner is grade 0");
    assert_eq!(grade(vec![0, 0, 0, 0], vec![1, 1, 0, 0]), vec![2], "at n = 4 the same pair shares a 2-cell");
    assert_eq!(grade(vec![0, 0, 0, 0], vec![1, 1, 1, 0]), vec![1], "and the corner of n = 3 is an edge of n = 4");
    assert_eq!(grade(vec![0, 0, 0, 0], vec![1, 1, 1, 1]), vec![0], "the corner of n = 4 is grade 0");
    assert!(grade(vec![0, 0, 0], vec![0, 1, 0]).is_empty(), "a shared face is no contact: it is one body");
    assert!(grade(vec![0, 0, 0], vec![2, 2, 0]).is_empty(), "and cubes a cell apart share nothing");
}
