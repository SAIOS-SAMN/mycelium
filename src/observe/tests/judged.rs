use super::super::*;
use super::common::*;

/// THE JUDGED-SOLVE OBSERVATION (2026-10-24): each solve a court judged
/// stands under its task's citation and the voice that sent it, with the
/// last verdict and its height and a count per verdict; another voice's
/// solve of the same task stands apart; a code past the verdicts is not
/// recorded; and the reading travels whole.
#[test]
fn note_judged_solve_keeps_verdicts_by_task_and_voice() {
    use crate::anastome::dialogue::{SOLVE_BOUNTY, SOLVE_FLOOR, SOLVE_UNREAD};
    let cite = [0xe7; 32];
    note_judged_solve(cite, party(1), SOLVE_UNREAD, 50);
    note_judged_solve(cite, party(1), SOLVE_BOUNTY, 52);
    note_judged_solve(cite, party(2), SOLVE_FLOOR, 60);
    note_judged_solve(cite, party(2), 9, 61);
    let j = judged_solves();
    let one = j.solves.iter().find(|s| s.citation == cite && s.voice == party(1)).copied().expect("kept by task and voice");
    assert_eq!((one.verdict, one.height, one.times), (SOLVE_BOUNTY, 52, [0, 0, 1, 1]), "the last verdict and height stand; each verdict counted at its code");
    let two = j.solves.iter().find(|s| s.citation == cite && s.voice == party(2)).copied().expect("another voice apart");
    assert_eq!((two.verdict, two.height, two.times), (SOLVE_FLOOR, 60, [0, 1, 0, 0]), "a code past the verdicts is not recorded");
    reads_back(&[Reading::Judged(Box::new(j.clone()))]);
    assert!(j.lines().iter().any(|l| l.contains(&"e7".repeat(32)) && l.contains("last first at height 52")), "named whole");
}
