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
/// ground, whatever its code. (A dot of 4 stands beside the ring, so no one
/// body fills the canvas's inner seam and the ring frames no interior of
/// its own.)
#[test]
fn ground_deduced_not_symbol_zero() {
    let x = sample(&["3333333", "3111333", "3101333", "3111343", "3333333"]);
    let y = sample(&["3333333", "3222333", "3202333", "3222343", "3333333"]);
    let read = Ground::of(&[(x, y)]);
    assert!(!read.holds(&vec![0]), "0 is no ground: {read:?}");
    assert_eq!(read, one(3));
}

/// The symbol 0 is absent from one INPUT of the family: it is not ambient,
/// and no symbol is.
#[test]
fn ground_requires_presence_in_every_input() {
    let family = [
        (sample(&["000", "010", "000"]), sample(&["000", "020", "000"])),
        (sample(&["333", "343", "333"]), sample(&["333", "353", "333"])),
    ];
    let read = Ground::of(&family);
    assert!(!read.holds(&vec![0]), "absent from one input: {read:?}");
    assert_eq!(read, Ground::None);
}

/// The canvas 0 is in every input, and one output paints it over whole:
/// covering the ground is painting, and it stays the ground.
#[test]
fn ground_absent_from_an_output_is_still_ground() {
    let family = [
        (sample(&["000", "010", "000"]), sample(&["000", "020", "000"])),
        (sample(&["000", "010", "000"]), sample(&["222", "222", "222"])),
    ];
    assert_eq!(Ground::of(&family), one(0));
}

/// The symbol 0 stands inside the output, and the output's seam is no one
/// body (1s and 2s alternate on it): 0 meets no seam, outer or inner, and
/// encloses nothing there.
#[test]
fn ground_requires_meeting_every_seam() {
    let read = Ground::of(&[(sample(&["0000", "0120", "0000"]), sample(&["1212", "2002", "1212"]))]);
    assert!(!read.holds(&vec![0]), "0 meets no seam of the output: {read:?}");
    assert_eq!(read, Ground::None);
}

/// The output frames the canvas: a body of 8 covers its whole outer seam,
/// and 0 fills the interior inside it. The frame is the output's membrane
/// (peeled to read its inner seam), and 0, on that inner seam, is the
/// ground; the frame is not.
#[test]
fn framed_interior_is_ground_and_frame_is_membrane() {
    let x = sample(&["0000", "0000", "0000", "0000"]);
    let y = sample(&["8888", "8008", "8008", "8888"]);
    let levels = super::super::membrane::levels(&y);
    let [outer, inner, ..] = levels.as_slice() else { panic!("two levels: {levels:?}") };
    assert_eq!(outer.membrane, Some(vec![8]), "the frame is the membrane");
    assert_eq!(inner.seam, vec![vec![0]], "0 is the inner seam");
    let read = Ground::of(&[(x, y)]);
    assert_eq!(read, one(0));
    assert!(!read.holds(&vec![8]), "the membrane is never the ground");
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

/// The frame stands in every sample, input and output alike, so 8 is a
/// candidate on the outer seam; inside it, 0 meets the frame's entire
/// inner seam and a body (a dot of 1) stands on it in the input. The ground
/// is what bodies stand on: the framed interior joins the vortex beside
/// the frame, neither picked.
#[test]
fn always_framed_interior_holding_bodies_joins_the_vortex() {
    let x = sample(&["888888", "800008", "801008", "800008", "800008", "888888"]);
    let y = sample(&["888888", "800008", "800108", "800008", "800008", "888888"]);
    assert_eq!(Ground::of(&[(x, y)]), Ground::Vortex(vec![vec![8], vec![0]]));
}

/// The frame stands in every sample and nothing stands inside it: 0 fills
/// the interior, but no body stands on it, so it is no ground; the frame
/// is the one candidate.
#[test]
fn interior_holding_no_body_is_not_ground() {
    let x = sample(&["8888", "8008", "8008", "8888"]);
    let read = Ground::of(&[(x.clone(), x)]);
    assert!(!read.holds(&vec![0]), "nothing stands on 0: {read:?}");
    assert_eq!(read, one(8));
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
