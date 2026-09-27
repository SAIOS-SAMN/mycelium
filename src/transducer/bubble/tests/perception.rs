// N4, 0.7 — PERCEPTIONS ARE NOT THOUGHTS: a held perception persists through
// the unglued door, and a thought is the transition that relates two of them.

use crate::transducer::bubble::Contemplation;
use crate::transducer::family::{scene_form, Grid, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// Perceive the scene `input → output` and hold its form, as a voice does.
fn perceive(c: &mut Contemplation, input: &[&str], output: &[&str]) {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let f = scene_form(&NoCrystal, &mut budget, &grid(input), &grid(output)).expect("the scene reads");
    c.hold_form(f, &Vec::new(), &[], [0; 4], &mut budget);
}

/// Every held idea read through its first expansion: nothing is dirty.
fn expanded(c: &mut Contemplation) {
    let keys: Vec<Vec<u8>> = c.bubbles.scoped().map(|(k, _)| k).collect();
    for k in keys {
        if let Some(b) = c.bubbles.get_mut(&k) {
            b.dirty = false;
        }
    }
}

/// One L body, turned 90°, 180° and carried unturned in three scenes: three
/// perceptions of one shape, each pair related by the transition between
/// their turns — three CONGRUENT thoughts closing one cycle. Expanded and
/// shed, every perception is still held: the unglued door acts on thoughts.
#[test]
fn held_perceptions_persist_and_meet_by_their_transitions() {
    let l = ["1000", "1000", "1100", "0000"];
    let mut c = Contemplation::new();
    perceive(&mut c, &l, &["1110", "1000", "0000", "0000"]);
    perceive(&mut c, &l, &["1100", "0100", "0100", "0000"]);
    perceive(&mut c, &l, &["0000", "0100", "0100", "0110"]);
    expanded(&mut c);
    assert_eq!(c.shed_unglued(), 0, "no perception passes the unglued door");
    let m = c.meet_graph();
    assert_eq!(m.vertices, 3, "the three perceptions held");
    assert_eq!(m.edges[1], 3, "each pair meets by a congruent transition");
    assert_eq!(m.cycle_rank, 1, "and the three close one cycle");
}

/// A PERCEPTION OF TWO BODIES TURNED APART has no one rotor, and it still
/// meets a perception of one of them: the transition on the chart the two
/// share, between the one body each holds there.
#[test]
fn a_perception_of_bodies_turned_apart_meets_on_the_chart_it_shares() {
    let mut c = Contemplation::new();
    // An L turned 90° and a bar carried unturned, in one scene.
    perceive(
        &mut c,
        &["100000", "100011", "110000", "000000"],
        &["111000", "100000", "000011", "000000"],
    );
    perceive(&mut c, &["1000", "1000", "1100", "0000"], &["1100", "0100", "0100", "0000"]);
    expanded(&mut c);
    assert_eq!(c.shed_unglued(), 0, "both perceptions held");
    let m = c.meet_graph();
    assert_eq!(m.vertices, 2);
    assert_eq!(m.edges.iter().sum::<u64>(), 1, "one thought: the L's transition between them");
}
