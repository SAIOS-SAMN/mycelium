// 23.8, 50.5 (L7) — G at the court: the court binds the pair it adjudicated
// as the sentence's family, lists each demand vertex by kind, and the
// stroke replays the sentence on that pair — refusing it for an undeclared
// remainder and never for the one its demands name.

use super::common::*;
use crate::cognition::*;

/// A percept-born sentence is bound to the pair the court adjudicated, so
/// its install measures what it closes there and mints — with nothing
/// bound by hand.
#[test]
fn the_court_binds_the_pair_it_adjudicated_as_the_sentences_family() {
    let mut home = Kernel::new();
    let mut peer = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
    assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    let addr = home.provisional.fiber(crate::network::nerve::ROOT).next().expect("docketed").0.clone();
    assert!(home.family_of(&addr).is_some_and(|f| f.len() == 1), "the pair is the sentence's family");
    home.court_stroke(one_stroke_transport);
    assert!(home.ruled_contains(&addr), "the stroke replays the sentence on its pair and rules it");
    let spoken = home.provisional.get(&addr).cloned().expect("provisional until a witness");
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
    let cover = home.genesis_cover().expect("measured");
    home.enroll([2; 32], cover);
    home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
    assert!(home.basis_len() >= 1, "the home installs on the witness");
    assert!(
        home.treasury().minted().expect("minted adds up").iter().any(|&x| x > 0),
        "the install mints the descent the sentence closes on the pair the court bound"
    );
}

/// A bar recoloured with a cell created beside it lists one sentence and
/// one `creation` demand under it, and the stroke rules the sentence: the
/// created cell is the remainder its demand names, valid and not closed.
#[test]
fn the_court_lists_each_demand_vertex_and_rules_the_closed_sub_complex() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 2), (0, 1, 2), (1, 2, 3)];
    assert_eq!(k.demands_listed(), 0);
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(k.demands_listed(), 1, "one demand vertex listed under the sentence");
    assert_eq!(k.provisional.len(), 1, "one sentence on the docket, never one listing per demand");
    k.court_stroke(one_stroke_transport);
    assert_eq!(k.court_slashed, 0, "the declared remainder does not refuse the sentence");
    let addr = k.provisional.fiber(crate::network::nerve::ROOT).next().expect("still docketed").0.clone();
    assert!(k.ruled_contains(&addr), "the closed sub-complex is ruled");
}

/// 50.5: a percept the eyes cannot open at all — an L that became a bar,
/// no body pair admitted — is a percept with no sentence and no listing.
#[test]
fn a_percept_the_eyes_cannot_pair_lists_nothing() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::Pivot, "no sentence, no listing");
    assert!(k.pending.is_empty(), "nothing on the book");
    assert!(k.provisional.is_empty(), "nothing on the docket");
    assert_eq!(k.basis_len(), 0);
}
