use super::super::*;
use super::common::*;

/// MU-CLIENT §5.3, §5.5 — A MIND'S READING AND A COURT'S AGREEMENTS travel
/// whole, the agreements' deviations exact rationals lane by lane, and a mind's
/// reading stands under its own source.
#[test]
fn mind_and_agreements_read_back_exact() {
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let mind = Mind {
        k: 9,
        heard: 4,
        understood: 3,
        returned: 2,
        priced: [5, 1, 2, 1],
        demanded: 2,
        constructed: [1, 0, 1, 0, 1, 2, 1],
        flex: [2, 1, 0, 3, 1, 1, 7, 2, 4],
        bodies: [4, 1, 2],
        perspective: [9, 3, 4, 1, 1, 6, 3, 1, 2],
        disagree: [1, 0, 2, 0, 3, 1, 4, 2, 1],
        demanded_kinds: [0, 1, 2, 3, 4, 5],
        boosts: [7, 1, 2],
        withheld_tests: [3, 2],
        acts: [4, 0, 1, 2, 3, 1, 0, 1],
        asks: [5, 2, 1, 1, 1],
        seats: [7, 2, 1, 3, 0],
        mutual: [4, 1],
        lexicon: 12,
        grown: 5,
        vocabulary: 40,
        remembered: 17,
        answers: [6, 1, 2],
        unanswered: 4,
        awaiting: 3,
        vantages: [5, 2, 7],
        repeats: [4, 9],
        laws: [3, 1, 0, 2],
        locks: vec![(1, 1, 3), (2, 1, 1)],
        drifts: 2,
        shears: 5,
        escapes: 1,
        held: 3,
        agreed: 2,
        composed: 8,
        shared: 1,
        closed: [q(1, 2), q(0, 1), q(0, 1), q(3, 7)],
        open: [q(-1, 3), q(2, 1), q(5, 4), q(0, 1)],
        thoughts: 6,
        unions: 1,
        waiting: 0,
        holes: 3,
        scars: 2,
        frozen: 1,
        breath: [4, 3, 1, 2],
        tasks: vec![TaskSeen { citation: [0x7b; 32], taken: 1, reading: 0, perspectives: 3, stop: 15 }],
    };
    let agreements = Agreements {
        pairs: vec![Agreed { speaker: party(1), hearer: party(2), agreed: 3, whole: 2, deviation: [q(1, 2), q(0, 1), q(0, 1), q(0, 1)] }],
    };
    reads_back(&[Reading::Mind(Box::new(mind.clone())), Reading::Agreements(Box::new(agreements))]);
    let key = Source::Mind(party(3)).key(TAG_MIND);
    assert_eq!(Source::of_key(&key), Some((Source::Mind(party(3)), TAG_MIND)), "a mind's reading stands under the mind");
    assert!(mind.lines().iter().any(|l| l.contains("deviation closed 1/2:0:0:3/7")), "the deviation is printed exact");
    // A court task travels by its whole citation and is named on the console:
    // taken, read as a task, undetermined by its perspectives, sent.
    let named = mind.tasks[0].line();
    assert!(named.starts_with(&format!("task {}", "7b".repeat(32))), "the whole citation: {named}");
    assert!(named.contains("perspectives: undetermined") && named.ends_with("stopped: sent"), "{named}");
}
