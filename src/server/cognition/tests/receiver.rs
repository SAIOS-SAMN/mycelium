// 23.8, 50.5 (L7, L8p) — G at the court: a spoken sentence is bound to its
// task's family, its demand vertices are listed by kind, and the stroke
// replays the sentence on that family — refusing it for an undeclared
// remainder and never for the one its demands name.

use super::common::*;
use crate::cognition::*;

/// A spoken sentence is bound to the family of the task it was spoken for
/// (23.8): the voice binds its task, the court binds the sentence to it at
/// speech, and the install measures what the sentence closes there and
/// mints — nothing is bound by the test.
#[test]
fn speak_pair_binds_sentence_to_task_family_and_mints() {
    let mut home = Court::new();
    let mut peer = Court::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    assert_eq!(speak_pair(&mut home, &a, &b, [1; 32]), Outcome::GermPending);
    let addr = home.provisional.scoped().next().expect("docketed").0.clone();
    assert!(home.family_of(&addr).is_some_and(|f| f.len() == 1), "the task's pair is the sentence's family");
    // 78.153: every stroke the docket's replay takes.
    home.court_settle();
    assert!(home.ruled_contains(&addr), "the stroke replays the sentence on its pair and rules it");
    let spoken = home.provisional.get(&addr).cloned().expect("provisional until a witness");
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(fetched(&mut peer, &spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
    let cover = home.genesis_cover().expect("measured");
    home.enroll([2; 32], cover);
    home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
    assert!(home.basis_len() >= 1, "the home installs on the witness");
    assert!(
        home.treasury().minted().expect("minted adds up").iter().any(|&x| x > 0),
        "the install mints what the sentence closes on the pair the court bound"
    );
}

/// A bar recoloured with a site painted beside it lists one sentence and
/// one `creation` demand under it, and the stroke rules the sentence: the
/// painted site is the remainder its demand names, valid and not closed.
#[test]
fn court_settle_lists_demands_and_rules_closed_subcomplex() {
    let mut k = Court::new();
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2), (1, 2, 3)];
    assert_eq!(k.demands_listed(), 0);
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.demands_listed(), 1, "one demand vertex listed under the sentence");
    assert_eq!(k.provisional.len(), 1, "one sentence on the docket, never one listing per demand");
    // 78.153: every stroke the docket's replay takes.
    k.court_settle();
    assert_eq!(k.court_slashed, 0, "the declared remainder does not refuse the sentence");
    let addr = k.provisional.scoped().next().expect("still docketed").0.clone();
    assert!(k.ruled_contains(&addr), "the closed sub-complex is ruled");
}

/// 50.5: a pair the eyes cannot open at all — an L that became a bar, no
/// body pair admitted — is a pair the voice has no sentence for: nothing
/// is spoken, nothing listed.
#[test]
fn speak_pair_speaks_nothing_for_unopened_pair() {
    let mut k = Court::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)];
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::Pivot, "no sentence, no listing");
    assert!(k.pending.is_empty(), "nothing on the book");
    assert!(k.provisional.is_empty(), "nothing on the docket");
    assert_eq!(k.basis_len(), 0);
}
