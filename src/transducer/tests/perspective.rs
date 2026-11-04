// MU-SENSING §4, §3.6 — the perspective: the roles of a sample's axes are a
// perspective's, read from the rank, and no axis is privileged by the code
// (ROADMAP track 26 S4b).

use crate::transducer::alphabet::{alphabet_admits, sensor_plane, slot_plane, AlphabetRank, ALPHABET_SAMPLE_RANK};
use crate::transducer::perspective::{family_place_axis, family_plane, Perspective, Role};

/// The default perspective of a site of a family's base of `n` coordinates
/// reads the sample's `n − 1` axes as space and the sample's place in its
/// family as the sequence, after them: the place is a coordinate the family
/// carries, never a fixed axis index.
#[test]
fn of_family_site_reads_space_then_place() {
    for n in 1..=5usize {
        let p = Perspective::of_family_site(n);
        assert_eq!(p.rank(), n);
        assert_eq!(p.space_axes(), (0..n - 1).collect::<Vec<_>>(), "every sample axis is space at n = {n}");
        assert_eq!(p.place_axis(), Some(n - 1), "the place follows the sample's axes at n = {n}");
        assert_eq!(family_place_axis(n), Some(n - 1));
    }
    assert_eq!(Perspective::of_family_site(0).place_axis(), None, "a base of no coordinate has no place");
}

/// The sensor's plane is the first two space axes a perspective assigns;
/// a sample of one space axis has none, and none is invented.
#[test]
fn plane_refuses_fewer_than_two_space_axes() {
    assert_eq!(family_plane(3), Some((0, 1)), "a rank-2 sample and its place");
    assert_eq!(family_plane(4), Some((0, 1)), "a rank-3 sample: its first two space axes");
    assert_eq!(family_plane(2), None, "a rank-1 sample has no plane");
    assert_eq!(Perspective::of_sample(1).plane(), None);
    let read_as_value = Perspective::new(vec![Role::Value, Role::Space, Role::Space]);
    assert_eq!(read_as_value.plane(), Some((1, 2)), "the plane is the perspective's, not axes 0 and 1");
}

/// Relabelling the axes moves each role with its axis, so a perspective
/// relabelled by a permutation and relabelled back is the one it was; a
/// relabelling that is no permutation is refused.
#[test]
fn relabelled_moves_roles_with_axes() {
    let p = Perspective::new(vec![Role::Space, Role::Value, Role::Sequence]);
    let q = p.relabelled(&[2, 0, 1]).expect("a permutation");
    assert_eq!(q.roles(), &[Role::Sequence, Role::Space, Role::Value]);
    assert_eq!(q.relabelled(&[1, 2, 0]), Some(p.clone()), "the inverse relabelling restores it");
    assert_eq!(p.relabelled(&[0, 0, 1]), None, "an axis read twice is no relabelling");
    assert_eq!(p.relabelled(&[0, 1]), None, "an axis left out is no relabelling");
}

/// The alphabet's sensor plane is the plane the default perspective of the
/// alphabet's own base assigns, each axis at its slot (MU-SENSING §9 on
/// MU-ALPHABET §2): the const reading and the perspective agree.
#[test]
fn sensor_plane_agrees_with_default_perspective() {
    let assigned = crate::transducer::perspective::alphabet_base_plane().and_then(slot_plane);
    assert_eq!(assigned, Some(sensor_plane()));
}

/// A sample of a rank the letters are not written on is refused by name
/// where only the alphabet limits it (`AlphabetRank`), never truncated to
/// the letters' planes; the rank the letters spell is admitted.
#[test]
fn rank_beyond_the_alphabet_is_refused_by_name() {
    assert_eq!(alphabet_admits(ALPHABET_SAMPLE_RANK), Ok(()));
    let beyond = ALPHABET_SAMPLE_RANK + 1;
    assert_eq!(alphabet_admits(beyond), Err(AlphabetRank { rank: beyond }));
    assert_eq!(alphabet_admits(1), Err(AlphabetRank { rank: 1 }), "a rank below is not spelled either");
}

/// D2.5 O3: the aperture counts the vertices of the unit cell of the base
/// the founding motions act on, the alphabet's, `2ⁿ` at its `n`, never a
/// task's rank and never a fixed frame axis.
#[test]
fn strokes_per_reserve_equals_vertices_of_alphabet_base_cell() {
    let n = crate::transducer::alphabet::ALPHABET_BASE_N;
    let vertices = (0..n).fold(1u64, |cell, _| cell * 2);
    assert_eq!(crate::physics::STROKES_PER_RESERVE, vertices);
    assert_eq!(n, crate::transducer::alphabet::LETTER_RANK - 1, "the base is the letters' chart less the orientation character");
}
