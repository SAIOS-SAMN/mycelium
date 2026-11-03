// MU-SENSING §3.4; ROADMAP track 26 S2 — THE GROUND, DEDUCED: a symbol is
// a candidate exactly when it is ambient (in every sample), enclosing (it
// meets every sample's seam) and never carried as a body; one candidate is
// the ground, several are held as a vortex, none reads no ground. No
// symbol is the ground by default, by count or by order.

use super::sample;
use crate::transducer::family::transduce::Ground;

fn one(symbol: u8) -> Ground<Vec<u8>> {
    Ground::One(vec![symbol])
}

/// The canvas is 3: ambient, meeting every seam, held in place. The symbol
/// 0 stands inside the body of 1s and meets no seam, so it is not the
/// ground, whatever its code.
#[test]
fn ground_deduced_not_symbol_zero() {
    let x = sample(&["33333", "31113", "31013", "31113", "33333"]);
    let y = sample(&["33333", "32223", "32023", "32223", "33333"]);
    assert_eq!(Ground::of(&[(x, y)]), one(3));
}

/// The canvas 0 is absent from one sample of the family (a pair whose
/// output is painted whole): it is not ambient, and no symbol is.
#[test]
fn ground_requires_presence_in_every_sample() {
    let family = [
        (sample(&["000", "010", "000"]), sample(&["000", "020", "000"])),
        (sample(&["000", "010", "000"]), sample(&["222", "222", "222"])),
    ];
    let read = Ground::of(&family);
    assert!(!read.holds(&vec![0]), "absent from one sample: {read:?}");
    assert_eq!(read, Ground::None);
}

/// The symbol 0 is in every sample, but in the output it stands only
/// inside a ring of 1s and meets no seam: it encloses nothing there.
#[test]
fn ground_requires_meeting_every_seam() {
    let read = Ground::of(&[(sample(&["000", "010", "000"]), sample(&["111", "101", "111"]))]);
    assert!(!read.holds(&vec![0]), "0 meets no seam of the output: {read:?}");
    assert_eq!(read, Ground::None);
}

/// On a canvas of 7, a block of 0 moves one site along a phase, and a
/// single 0 stays in the corner: 0 is ambient and enclosing, but a body of
/// it is carried elsewhere, so it carries structure and is no ground.
#[test]
fn ground_carried_as_a_body_is_not_ground() {
    let x = sample(&["07777", "77777", "70077", "70077", "77777"]);
    let y = sample(&["07777", "77777", "77007", "77007", "77777"]);
    assert_eq!(Ground::of(&[(x, y)]), one(7));
}

/// A body painted onto the ground (the 1 grows by a site) covers some of
/// the ground's sites; the ground carries no body, and stays the ground.
#[test]
fn painting_onto_the_ground_keeps_it_ground() {
    let x = sample(&["0000", "0100", "0000"]);
    let y = sample(&["0000", "0110", "0000"]);
    assert_eq!(Ground::of(&[(x, y)]), one(0));
}

/// Two symbols each ambient, enclosing and held in place: both are held,
/// in the order the family's samples meet them, and neither is picked.
#[test]
fn several_candidates_are_held_as_a_vortex() {
    let x = sample(&["0055", "0055"]);
    let read = Ground::of(&[(x.clone(), x)]);
    assert_eq!(read, Ground::Vortex(vec![vec![0], vec![5]]));
    assert!(read.holds(&vec![0]) && read.holds(&vec![5]), "every candidate is read as ground");
    assert!(read.symbol().is_err(), "a writer is refused: no candidate is picked");
}

/// Every symbol is a single site that moves: each is carried as a body, so
/// no candidate stands and every symbol is a body.
#[test]
fn no_candidate_reads_no_ground() {
    let read = Ground::of(&[(sample(&["12", "34"]), sample(&["21", "43"]))]);
    assert_eq!(read, Ground::None);
    assert!(!read.holds(&vec![1]) && !read.holds(&vec![0]), "no symbol is read as ground");
}
