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

// ── TRACK 26 S6: A SHIFT OF PERSPECTIVE IS A SENSOR MOTION; PERSPECTIVES
// OF ONE SAMPLE ARE A COVER (MU-SENSING §4; MU-CLIENT 24.4) ─────────────────

use crate::transducer::family::transduce::{Sense, SignedPermutation};
use crate::transducer::perspective::cover::{Agreement, Cocycle, Contact, Cover};
use crate::transducer::perspective::SensorMotion;
use crate::transducer::sample::{Place, Sample};

/// A rank-3 sample of extents 2 × 3 × 2 with three symbols, placed.
fn block() -> Sample {
    let symbols = vec![1, 1, 2, 2, 3, 3, 1, 2, 2, 2, 3, 1];
    Sample::new(vec![2, 3, 2], 1, symbols, Place::new(vec![0, 1, 0])).expect("a sample")
}

/// A shift of perspective moves the sensor and never the data: the frame read
/// under the shifted perspective is the frame read before, moved by the
/// sensor's motion; the posted sample's content (its address) is what it
/// was; shifts compose as the motions do; and the turn from one perspective
/// to the other is the motion.
#[test]
fn perspective_shift_is_a_sensor_motion() {
    let s = block();
    let mut before = Vec::new();
    s.put_content(&mut before);
    let p = Perspective::of_sample(3);
    for g in SignedPermutation::all(3) {
        let q = p.shifted(&SensorMotion::Pose(g.clone())).expect("a pose of the rank");
        let moved = g.moved(&p.frame(&s).expect("read")).expect("a pose lands on its own periods");
        assert_eq!(q.frame(&s).expect("read"), moved, "the reading is the sensor's motion of the reading");
        assert_eq!(p.turn_to(&q), Ok(g.clone()), "the turn between the perspectives is the motion");
        assert_eq!(q.read(&s).expect("read").place(), s.place(), "the reading keeps the sample's place");
        for h in SignedPermutation::all(3).step_by(7) {
            let twice = q.shifted(&SensorMotion::Pose(h.clone())).expect("a pose");
            let once = p.shifted(&SensorMotion::Pose(g.then(&h).expect("one rank"))).expect("a pose");
            assert_eq!(twice, once, "shifts compose as the motions do");
        }
    }
    let mut after = Vec::new();
    s.put_content(&mut after);
    assert_eq!(before, after, "the sample is never rewritten");
    let off = SignedPermutation::identity(2);
    assert!(p.shifted(&SensorMotion::Pose(off)).is_err(), "a motion of another rank is refused by name");
    let recast = p.shifted(&SensorMotion::Recast { axis: 1, role: Role::Sequence }).expect("an axis read");
    assert_eq!(recast.roles(), &[Role::Space, Role::Sequence, Role::Space]);
    assert_eq!(recast.pose(), p.pose(), "a recast keeps the pose");
    assert!(p.shifted(&SensorMotion::Recast { axis: 3, role: Role::Value }).is_err(), "an axis the perspective does not read");
}

/// Two perspectives of one sample are compared as a cover: perspectives
/// that differ by a pose read the same bodies (congruent), the turn between
/// them the transition; a perspective that reads an axis as a sequence
/// holds no contact along it, and the residue is exactly those contacts,
/// held and named; the transitions close around every triple.
#[test]
fn two_perspectives_of_one_sample_compared_as_a_cover() {
    let s = block();
    let p = Perspective::of_sample(3);
    let turn = SignedPermutation::new(vec![(2, Sense::Reversed), (0, Sense::Kept), (1, Sense::Reversed)]).expect("a signed permutation");
    let turned = p.shifted(&SensorMotion::Pose(turn.clone())).expect("a pose");
    // Read axis 1 of the turned perspective is the sample's axis 0.
    let sequenced = turned.shifted(&SensorMotion::Recast { axis: 1, role: Role::Sequence }).expect("an axis read");
    let cover = Cover::of(&s, vec![p.clone(), turned, sequenced]).expect("one rank");
    let a = cover.overlap(0, 1).expect("an overlap");
    assert_eq!(a.turn, turn);
    assert_eq!(a.agreement, Agreement::Congruent, "a pose reads the same bodies");
    let b = cover.overlap(0, 2).expect("an overlap");
    let along_zero: Vec<Contact> = cover.contacts(0).expect("read").into_iter().filter(|c| c.axis == 0).collect();
    assert!(!along_zero.is_empty(), "the fixture holds contacts along axis 0");
    assert_eq!(b.agreement, Agreement::Residue { first_only: along_zero, second_only: Vec::new() }, "the residue is the contacts along the axis read as a sequence");
    assert_eq!(cover.cocycle(0, 1, 2), Some(Cocycle::Closes), "the transitions close around the triple");
    assert_eq!(cover.cocycle(2, 0, 1), Some(Cocycle::Closes));
    let other = Sample::new(vec![2, 2], 1, vec![1, 1, 1, 1], Place::new(Vec::new())).expect("a sample");
    assert!(Cover::of(&other, vec![p]).is_err(), "a perspective of another rank is refused by name");
}

/// The frame fixing is a perspective: the task it returns is the posted
/// task read through the pose the moments name, and the posted task is not
/// rewritten.
#[test]
fn frame_fixing_is_a_perspective_reading() {
    crate::transducer::raster::stand_on(0);
    let train = vec![(vec![vec![4, 4, 0], vec![4, 0, 0], vec![0, 0, 0]], vec![vec![5, 5, 0], vec![5, 0, 0], vec![0, 0, 0]])];
    let task = crate::transducer::family::task_of_lines(train, Vec::new());
    let posted = task.clone();
    let perspective = crate::transducer::raster::frame_fixing(&task);
    let fixed = crate::transducer::raster::frame_fixed(task);
    for ((a, b), (fa, fb)) in posted.train.iter().zip(&fixed.train) {
        assert_eq!(perspective.read(a.sample()).as_ref(), Ok(fa.sample()), "the input read through the perspective");
        assert_eq!(perspective.read(b.sample()).as_ref(), Ok(fb.sample()), "the output read through the perspective");
    }
    assert_ne!(perspective.pose(), &SignedPermutation::identity(2), "the moments name a pose for this scene");
}
