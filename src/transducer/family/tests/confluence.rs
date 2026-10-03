// 78.78 — CONFLUENCE, READ (`family/confluence.rs`): an unsaid output body is a
// creation, a single carriage or a confluence by the input bodies it stands
// on; a confluence's mass, centre, inverse generator and containment are read
// exactly.

use super::common::grid;
use crate::transducer::family::confluence::read_pair;

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// TWO INPUTS BRIDGED: two single cells, and an output bar joining them. The
/// bar stands on both: a confluence of two, its inputs contained, its mass
/// grown (2 → 5), its centre kept (the bar is centred between them) — the
/// spheres concentric, the generator a dilation with an exact ratio — and
/// the dilation does not carry two cells onto five.
#[test]
fn a_bridged_pair_reads_as_a_confluence_of_two() {
    let x = grid(&["10001"]);
    let y = grid(&["11111"]);
    let rem = vec![(0, 1, 0, 1), (0, 2, 0, 1), (0, 3, 0, 1)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    let [c] = p.confluences.as_slice() else { panic!("one confluence: {p:?}") };
    assert_eq!((c.inputs, c.mass_in, c.mass_out), (2, 2, 5));
    assert!(c.contains && c.centre_kept, "both inputs stand in the bar, which is centred between them");
    assert!(c.ratio.is_some(), "concentric: the dilation's ratio is read exactly");
    assert!(!c.reproduces, "a dilation carries two cells onto two, never onto five");
    assert_eq!(p.confluence_sites, 3, "the three unsaid sites are the confluence's");
}

/// CREATION AND SINGLE CARRIAGE: an output body on no input is a creation;
/// one on exactly one input is a single carriage.
#[test]
fn creation_and_single_carriage_are_told_apart() {
    let x = grid(&["100", "000"]);
    let y = grid(&["110", "002"]);
    let rem = vec![(0, 1, 0, 1), (1, 2, 0, 2)];
    let p = read_pair(&x, &y, &rem, &mut budget());
    assert_eq!((p.singles, p.single_sites), (1, 1), "the grown cell stands on its one input");
    assert_eq!((p.creations, p.creation_sites), (1, 1), "the new cell stands on none");
    assert!(p.confluences.is_empty());
}
