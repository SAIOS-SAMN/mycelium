// 42 — THE COURT: a verdict installs, excises or refuses, and every organ
// reaches it by replaying the word rather than by trusting a signature.

use super::common::*;
use crate::cognition::*;
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, Element};

/// THE BLIND REPLAY refutes every tamper axis and upholds honest speech —
/// the contradiction docket's whole force is this determinism.
#[test]
fn blind_replay_refutes_tampered_words() {
use crate::transducer::germinate::germinate;
use crate::verdict::blind_replay;
let w = element::from_residual(
    &germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap(),
);
assert!(blind_replay(&w).is_none(), "an honest derivation replays clean");
// Tamper 1: shift the chain off origin (framing violation).
let mut off = w.clone();
off.sub = off.sub.iter().map(|(r, c, b, a)| (r + 3, *c, *b, *a)).collect();
assert!(blind_replay(&off).is_some(), "un-centered chain is false speech");
// Tamper 2: inflate the claimed magnitude (yield fraud).
let mut fat = w.clone();
fat.mag = [999, 0, 0, 0];
assert!(blind_replay(&fat).is_some(), "inflated magnitude does not re-derive");
// Tamper 3: an empty chain claiming work.
let mut empty = w.clone();
empty.sub.clear();
assert!(blind_replay(&empty).is_some(), "no evidence, no word");
}

/// STEP 5 — PROVISIONAL UNTIL THE COURT SPEAKS: a two-voice install escrows
/// its consilience award (unfinalized speech earns nothing); the docket
/// exports the spoken word; the upheld verdict releases the award; a
/// slashing verdict excises the word and debits the voices into the pool.
#[test]
fn installs_are_provisional_until_verdict() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let chain = germinate(&a, &b).unwrap();
assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
// The install PAID NOTHING — the award sits in escrow.
assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "unfinalized speech earns nothing");
assert_eq!(k.treasury().balance_mu(&[2; 32]), [0; 4]);
let dockets = k.dockets();
assert_eq!(dockets.len(), 1, "the provisional word is on the docket");
let row = &dockets[0];
assert_eq!(row.voices.len(), 2, "both attesting voices ride the docket");
assert!(row.escrowed.iter().any(|&m| m > 0), "the escrowed award is real");
assert_eq!(row.spoken.address(), row.addr.clone(), "the docket carries the word as spoken");
let addr = &row.addr;
// UPHELD: finality releases the award — the voices are paid at last.
assert!(k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD));
assert!(k.treasury().balance_mu(&[1; 32]).iter().any(|&m| m > 0), "finality pays voice A");
assert!(k.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0), "finality pays voice B");
assert!(k.dockets().is_empty(), "the docket cleared");
assert!(!k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD), "a verdict lands once");
}

/// A slashing verdict (failed blind replay — false speech) excises the word
/// from the basis and debits the voices' earned balances into the pool.
#[test]
fn a_slashing_verdict_excises_and_debits() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let chain = germinate(&a, &b).unwrap();
assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
let addr = k.dockets()[0].addr.clone();
assert!(k.apply_verdict(&addr, crate::anastome::dialogue::RULING_SLASHED));
assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "nothing was ever paid");
assert!(k.dockets().is_empty());
}

/// Court organ: docketing a germ does not grow the basis. The stroke
/// puts a section on the ruled cover. A peer's fetch glues by replay;
/// a forged word is slashed with a reason. Signatures are not consulted.
#[test]
fn court_organ_glue_on_replay_not_on_signatures() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
let mut third = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
assert_eq!(home.basis_len(), 0);
home.court_stroke();
let spoken = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(
    peer.glue_fetched(spoken.clone(), [1; 32], [2; 32], &mut cover),
    Glue::Admitted
));
assert!(peer.basis_len() >= 1, "the word is language on the second organ");
assert!(matches!(
    third.glue_fetched(spoken.clone(), [2; 32], [3; 32], &mut cover),
    Glue::Admitted
));
let mut forged = spoken.clone();
forged.mag = [999, 0, 0, 0];
assert!(
    matches!(third.glue_fetched(forged, [2; 32], [3; 32], &mut cover), Glue::Slashed(_)),
    "never admit on a word that fails replay"
);
}

/// Adopt writes a field entry and a deflation so the stroke can rule
/// a client-minted connection. Ruled listings do not starve later ones.
#[test]
fn adopt_reaches_the_docket_and_stroke_skips_ruled() {
use crate::transducer::element::from_connection;
fn conn(shift: i64) -> Element {
    from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::translator_versor(shift, 0),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("spells")
}
let mut k = Kernel::new();
let cap = crate::physics::Physics::default().stroke().remaining();
let mut addrs = Vec::new();
for i in 0..(cap + 1) {
    let w = conn(i as i64 + 1);
    let addr = w.address();
    assert!(matches!(k.install_adopted(&w, [1; 32]), Adoption::Installed));
    assert!(k.field().get(&addr).is_some(), "adopt writes a field entry");
    addrs.push(addr);
}
let ruled_n = |k: &Kernel, addrs: &[crate::transducer::address::Address]| {
    addrs
        .iter()
        .filter(|a| {
            k.word_at(a).is_some_and(|w| k.ruled_have(&crate::transducer::stalk::invariant_of(w)).iter().any(|h| h.as_slice() == a.as_bytes()))
        })
        .count()
};
k.court_stroke();
assert_eq!(ruled_n(&k, &addrs), cap, "one stroke rules a spend of unruled listings, not already-ruled ones");
k.court_stroke();
assert_eq!(ruled_n(&k, &addrs), cap + 1, "later listings are not starved by already-ruled ones");
}

/// The home installs when a peer's ruled digest holds the stratum.
#[test]
fn home_installs_when_peer_ruled_root_gains_the_address() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
assert_eq!(home.basis_len(), 0);
home.rule_all_provisionals();
let spoken = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
assert!(peer.basis_len() >= 1);
let cover = home.genesis_cover().expect("measured");
home.enroll([2; 32], cover); // 47.3: a witness with room
home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
assert!(home.basis_len() >= 1, "the home's own word is language once a peer holds the stratum");
assert!(home.provisional.is_empty(), "the provisional leaves the home");
}

/// A peer have-list that names a ruled provisional is the second organ:
/// the home installs without waiting for a digest-root match.
#[test]
fn home_installs_on_a_have_list_naming_a_ruled_provisional() {
let mut home = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
let addr = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0.clone();
home.court_stroke();
assert!(home.provisional.contains(&addr), "still provisional until the second organ");
// 47.3: a witness is a second VANTAGE with room, not merely a second
// key. An unbonded peer has none and does not converge the object.
home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
assert!(home.basis_len() == 0, "a witness without room is not a vantage");
let cover = home.genesis_cover().expect("measured");
home.enroll([2; 32], cover);
home.confirm_peer_holds_addrs(&[Address::from(addr.as_slice())], [2; 32]);
assert!(home.basis_len() >= 1, "the witness re-derives the address and it installs");
assert!(home.provisional.is_empty(), "the docket clears on install");
}

/// 23.1 step 7 on the court's own stroke (I3): a docketed form is replayed
/// against the family it is bound to, so a form that leaves a remainder on
/// a pair of that family is slashed with `REFUSE_NOT_CLOSED` where the
/// stroke rules, and the same form bound to the family it closes is ruled.
/// Before this the stroke replayed with no family, so closure was checked
/// only at speech and never again.
#[test]
fn the_court_stroke_refuses_a_form_that_leaves_a_remainder_on_its_family() {
use crate::transducer::family::{extract_rule, mint_sentence, NoCrystal};
let grid = |rows: &[&str]| -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
};
let (x, y) = (grid(&["1"]), grid(&["2"]));
let one_pair = vec![(x.clone(), y.clone())];
let rule = extract_rule(&one_pair).expect("a recolour is a rule");
let word = mint_sentence(&rule, &one_pair, &NoCrystal).expect("it closes its own pair");
let addr = word.address();
let wider = vec![(x, y), (grid(&["10"]), grid(&["23"]))];

let mut refusing = Kernel::new();
let task = [7u8; 32];
refusing.bind_task_family(task, wider);
refusing.docket_sentence(&word, [1; 32]);
refusing.speech_task.insert(&addr, Address::from_bytes(task));
assert!(refusing.provisional.contains(&addr), "docketed");
refusing.court_stroke();
assert_eq!(refusing.court_slashed, 1, "the stroke refuses the form on the family it does not close");
assert!(!refusing.provisional.contains(&addr), "and it leaves the docket");
assert!(!refusing.ruled_contains(&addr), "never ruled");

let mut ruling = Kernel::new();
ruling.bind_task_family(task, one_pair);
ruling.docket_sentence(&word, [1; 32]);
ruling.speech_task.insert(&addr, Address::from_bytes(task));
ruling.court_stroke();
assert_eq!(ruling.court_slashed, 0, "the form closes every pair of the family it is bound to");
assert!(ruling.ruled_contains(&addr), "and the stroke rules it");
}

/// A glued form's descent is measured through the court genome as Crystal.
#[test]
fn glued_form_replays_descent_through_the_court_genome() {
use crate::transducer::family::{
    apply_form, extract_rule_with, mint_sentence, pair_form, Crystal, Grid, Speech,
};
use crate::transducer::sentence::{as_element_form, Form, Vertex};
use crate::transducer::stalk::{Invariant, Standing};
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0011", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0011"])),
];
let held_form = pair_form(&family[0].0, &family[0].1, None);
let held_e = as_element_form(&held_form, [1, 0, 0, 0]).expect("inner tape");
struct Held {
    e: Element,
    f: Form,
}
impl Crystal for Held {
    fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
        vec![(self.e.clone(), self.f.clone(), Standing::Held)]
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)> {
        (addr == self.e.address().as_bytes()).then(|| (self.e.clone(), self.f.clone()))
    }
    fn on_axis(
        &self,
        _chart: &Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        let mut index = crate::transducer::sentence::AxisIndex::new();
        crate::transducer::sentence::admit_words(
            &mut index,
            &Invariant::default(),
            &crate::transducer::sentence::words_of_form(&self.f),
        );
        crate::transducer::sentence::on_axis_of(&index, &Invariant::default(), axis)
    }
}
let cover = Held {
    e: held_e.clone(),
    f: held_form,
};
let rule = extract_rule_with(&family, &cover).expect("glues");
assert!(
    rule.form
        .vertices
        .iter()
        .any(|v| matches!(v, Vertex::Glued { .. })),
    "the rule cites the held form"
);
let spoken = mint_sentence(&rule, &family, &cover).expect("mint through crystal");
let mut k = Kernel::new();
let inner_addr = held_e.address();
assert_eq!(
    k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&held_e)),
    vec![crate::anastome::dialogue::SPEECH_STOOD]
);
k.rule_all_provisionals();
k.confirm_peer_holds_addrs(std::slice::from_ref(&inner_addr), [2; 32]);
let task = [9u8; 32];
k.bind_task_family(task, family);
let codes = k.perceive_speech([1; 32], &task, std::slice::from_ref(&spoken));
assert_eq!(
    codes,
    vec![crate::anastome::dialogue::SPEECH_STOOD],
    "glued descent replays through the court genome, not NoCrystal"
);
}

/// 47.3, 13.9: A WITNESS GLUES AN OFFER AS THE HOME, NOT AS ITSELF. The
/// Čech cell of a converging pair has two vertices, one per vantage. Gluing
/// with `peer == self_id` writes the offered word into the section this
/// court's own reading is read back from, so the court compares the form
/// with itself and agrees every time — an agreement manufactured by writing
/// the answer into the place the question is asked.
#[test]
fn a_witness_glues_an_offer_at_the_homes_vantage() {
    let home_vk = [1u8; 32];
    let witness_vk = [2u8; 32];
    let mut home = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    assert_eq!(home.ingest(&a, &b, home_vk), Outcome::GermPending);
    assert_eq!(home.ingest(&a, &b, [3; 32]), Outcome::GermPending);
    home.court_stroke();
    let spoken =
        home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().1.clone();
    let inv = crate::transducer::stalk::invariant_of(&spoken);
    let offered = spoken.address();

    let mut witness = Kernel::new();
    let mut cover = crate::network::nerve::Cover::new();
    assert!(matches!(
        witness.glue_fetched(spoken.clone(), home_vk, witness_vk, &mut cover),
        Glue::Admitted
    ));
    let cx = witness.complexes().get(&inv.key()).expect("the class has a complex");
    assert_eq!(
        cx.sections.get(home_vk.as_slice()).map(|s| s.addr.clone()),
        Some(offered.clone()),
        "the offered form is the HOME's section"
    );
    assert!(
        cx.sections.get(witness_vk.as_slice()).is_none(),
        "and is not this court's own reading, which it has not made"
    );

    // The old read, kept here so the difference is a measurement and not a
    // claim: gluing as oneself files the home's word under this court's own
    // vantage, and the cell has one vertex where 13.9 needs two.
    let mut confused = Kernel::new();
    let mut cover = crate::network::nerve::Cover::new();
    let _ = confused.glue_fetched(spoken, witness_vk, witness_vk, &mut cover);
    let cx = confused.complexes().get(&inv.key()).expect("the class has a complex");
    assert_eq!(
        cx.sections.get(witness_vk.as_slice()).map(|s| s.addr.clone()),
        Some(offered),
        "one vantage: the court holds the offer as its own re-derivation"
    );
    assert!(cx.sections.get(home_vk.as_slice()).is_none(), "and the home has no section at all");
}
