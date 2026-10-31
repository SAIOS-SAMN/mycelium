// 78.167 A — WHAT A WRONG SPEECH IS, BY CAUSE: a perspective whose rule put
// cells in the wrong places is read cell by cell — a body erased that no
// vertex reached, a body displaced, cells never created, cells never cleared —
// and a speech names the bodies it did not reach.

use crate::arc::Grid;
use crate::client::sdk::entity::family::occupancy_causes;

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// EACH CAUSE ON ITS SLOT. The input holds a body at the top-left and one at
/// the bottom-right. The truth keeps the bottom-right body, moves nothing
/// else, and creates a cell on ground; the speech left the bottom-right body
/// unreached (ground), painted a cell the truth holds as ground, and left the
/// created cell ground.
#[test]
fn occupancy_causes_reads_wrong_speech_by_cause() {
    let x = grid(&["100", "000", "002"]);
    let y = grid(&["000", "030", "002"]);
    // The body at index 1 is the bottom-right one (perception reads bodies
    // in its own order: the first site met is the top-left body).
    let bodies = crate::transducer::family::perceive(&x);
    let br = bodies
        .iter()
        .position(|b| b.compartments.iter().any(|l| crate::transducer::family::on_grid(&l.site) == Some((2, 2))))
        .expect("the bottom-right body is perceived");
    let said = grid(&["100", "000", "000"]);
    let causes = occupancy_causes(&said, &x, &y, &[br]);
    assert_eq!(causes, vec![4, 5, 7], "erased (unreached and kept), uncreated (ground in, cell out), uncleared (painted where ground)");
    // The same wrong cell, the body reached: displaced.
    assert_eq!(occupancy_causes(&grid(&["000", "000", "000"]), &x, &grid(&["000", "000", "002"]), &[]), vec![6]);
    // Every cell agrees on occupancy: no cause here (the hues alone differ).
    assert!(occupancy_causes(&grid(&["100", "000", "005"]), &x, &grid(&["100", "000", "002"]), &[]).is_empty());
}

/// A SPEECH NAMES THE BODIES IT DID NOT REACH. A rule extracted from a family
/// that carries one shape is spoken on an input that also holds a body of a
/// shape the family never showed: that body is unreached, and named so —
/// never silently absent from the speech.
#[test]
fn speak_rule_names_unreached_bodies() {
    let family = vec![
        (grid(&["1000", "0000", "0000"]), grid(&["0000", "1000", "0000"])),
        (grid(&["0000", "0100", "0000"]), grid(&["0000", "0000", "0100"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let rule = crate::transducer::family::extract_rule(&family, &mut budget).expect("the family has a rule");
    let x = grid(&["0010", "0000", "2220"]);
    let speech = crate::transducer::family::speak::speak_rule(&rule, &x, &crate::transducer::family::NoCrystal).expect("it speaks");
    let bodies = crate::transducer::family::perceive(&x);
    let bar = bodies.iter().position(|b| b.compartments.len() == 3).expect("the bar is perceived");
    assert!(speech.unreached.contains(&bar), "the bar of a shape the family never showed is named unreached: {:?}", speech.unreached);
}

/// 78.167 B — A SPEECH THAT LEFT A BODY UNREACHED IS NEVER A SOLVE. The same
/// rule spoken on a test whose every body it reaches is the answer the court
/// is sent; spoken on a test holding a bar of a shape its family never
/// showed, the whole answer is withheld (the bar held as a question), never
/// sent with the bar silently erased.
#[test]
fn tests_spoken_withholds_speech_with_unreached_body() {
    use crate::client::sdk::*;
    let family = vec![
        (grid(&["1000", "0000", "0000"]), grid(&["0000", "1000", "0000"])),
        (grid(&["0000", "0100", "0000"]), grid(&["0000", "0000", "0100"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let rule = crate::transducer::family::extract_rule(&family, &mut budget).expect("the family has a rule");
    use crate::client::sdk::entity::family::tests_spoken;
    let e = Mind::open("127.0.0.1:1", Voice::named("withheld"));
    let (reached, _) = tests_spoken(&e.epigenome, &rule, &[grid(&["0010", "0000", "0000"])]);
    assert_eq!(reached.expect("every body reached: the answer stands"), vec![grid(&["0000", "0010", "0000"])]);
    let (unreached, held) = tests_spoken(&e.epigenome, &rule, &[grid(&["0010", "0000", "2220"])]);
    assert!(unreached.is_none(), "the bar unreached: nothing is sent");
    assert!(!held.is_empty(), "the bar is held as a question");
}
