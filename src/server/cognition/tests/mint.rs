// 26, E2 — WHAT MINTS AND WHAT DOES NOT: one vantage mints nothing until a
// witness re-derives; association with a listing is not a mint; a rule that
// speaks its family mints the descent it measured.

use super::common::*;
use crate::cognition::*;
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, Element};

/// ONE VANTAGE DOCKETS: a sentence this court replayed is listed on the
/// book, escrowed as a claim, and Offered — and mints NOTHING. The second
/// vantage is a witness hypha that re-derives the same address and has room
/// (47.3); on its arrival the escrow releases and the deflation mints. No
/// number of voices substitutes for it, and this asserted a two-voice
/// quorum before convergence replaced consensus.
#[test]
fn one_vantage_dockets_escrows_and_mints_nothing_until_a_witness_rederives() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
// The whole body recoloured: one hue, one body, one word that closes its
// pair. What mints on release is the deflation a sentence closes (E2);
// a sentence of demands closes nothing and mints nothing (50.5).
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)];
assert_eq!(speak_pair(&mut home, &a, &b, [1; 32]), Outcome::GermPending);
assert!(!home.pending.is_empty(), "the listing stays on the book");
assert_eq!(home.provisional.len(), 1, "the sentence is on the docket");
let addr = home.provisional.scoped().next().unwrap().0.clone();
// The voice bound its task's family and the court bound the sentence to
// it at speech (23.8), so the install measures the descent the sentence
// closes (E2).
assert!(home.family_of(&addr).is_some(), "the court bound the sentence to its task's family");
assert!(home.offered(&addr), "the court Offers a first-vantage germ");
assert!(home.treasury().escrow_of(&addr).is_some(), "the priced magnitude is the claim");
assert_eq!(home.treasury().minted().expect("minted adds up"), [0; 4], "the claim is not yet payment: nothing minted");
home.court_stroke(one_stroke_transport);
let spoken = home.provisional.get(&addr).cloned().expect("still provisional until a peer glues");
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(
    peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover),
    Glue::Admitted
));
let cover = home.genesis_cover().expect("measured");
home.enroll([2; 32], cover); // 47.3: a witness with room
let bond = home.treasury().bond_total();
home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
assert!(home.basis_len() >= 1, "the home installs after the peer glues");
assert!(home.provisional.is_empty(), "the docket clears on install");
assert!(home.treasury().escrow_of(&addr).is_none(), "the claim is released");
assert!(
    home.treasury().minted().expect("minted adds up").iter().any(|&x| x > 0),
    "release mints the priced vector"
);
assert_eq!(home.treasury().bond_total(), bond, "mint does not debit the stake bond");
}

/// A stood sentence that closes no listing returns its escrow and mints nothing.
#[test]
fn speech_that_closes_nothing_mints_none() {
let mut k = Kernel::new();
let w = crate::transducer::element::from_connection(
    &crate::transducer::alphabet::Word::identity(),
    &crate::transducer::element::identity_versor(),
    &[(1, 2), (2, 1)],
    &Vec::new(),
    [1, 1, 0, 0],
)
.unwrap();
let addr = w.address();
let codes = k.perceive_speech([1; 32], &[0; 32], std::slice::from_ref(&w));
assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
assert!(k.treasury().escrow_of(&addr).is_some());
k.rule_all_provisionals();
// 47.3: the witness is a vantage with room. Bond it.
let cover = k.genesis_cover().expect("measured");
k.enroll([2; 32], cover);
k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
assert!(k.treasury().escrow_of(&addr).is_none(), "escrow returned");
assert_eq!(
    k.treasury().minted().expect("minted adds up"),
    [0; 4],
    "a restatement that closes nothing pays nothing"
);
}

/// Binding a listing to a task does not mint: the form must speak the trains.
#[test]
fn association_with_a_listing_does_not_mint() {
let mut k = Kernel::new();
let family = vec![(
    digits(&["1100", "0000"]),
    digits(&["0011", "0000"]),
)];
let crate::arc::Refined { delta: chain, frame, .. } =
    crate::arc::refine_delta(&family[0].0, &family[0].1).expect("coboundary");
assert!(k.intake(&chain, &frame), "the listing stands");
let listing = k.pending.scoped().next().expect("listed").1.0.address();
let task = [7u8; 32];
k.bind_listing_task(&listing, task);
k.bind_task_family(task, family);
let w = crate::transducer::element::from_connection(
    &crate::transducer::alphabet::Word::identity(),
    &crate::transducer::element::identity_versor(),
    &[(1, 2), (2, 1)],
    &Vec::new(),
    [4, 4, 4, 4],
)
.unwrap();
let addr = w.address();
assert_ne!(addr, listing, "the restatement is not the listing");
let codes = k.perceive_speech([1; 32], &task, std::slice::from_ref(&w));
assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
k.bind_listing_task(&addr, task);
k.rule_all_provisionals();
// 47.3: the witness is a vantage with room. Bond it.
let cover = k.genesis_cover().expect("measured");
k.enroll([2; 32], cover);
k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
assert_eq!(
    k.treasury().minted().expect("minted adds up"),
    [0; 4],
    "a bound listing is not a mint without replay"
);
}

/// A rule that speaks every train pair mints the coboundary it closed.
#[test]
fn a_rule_that_speaks_the_family_mints_measured_descent() {
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
];
let rule = crate::transducer::family::extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal).expect("mint");
assert!(spoken.is_sentence());
let mut expect = [0u64; 4];
for (x, y) in &family {
    let chain = crate::arc::refine_delta(x, y).expect("coboundary").delta;
    let lanes = crate::transducer::element::measure_lanes(&chain);
    for i in 0..4 {
        expect[i] = expect[i].saturating_add(lanes[i]);
    }
}
assert!(expect.iter().any(|&x| x > 0), "the family has descent");
let decoded = crate::transducer::sentence::from_element_form(&spoken).expect("v2 form");
assert_eq!(
    crate::transducer::family::form_deflation(&decoded, &family, &crate::transducer::family::NoCrystal).0,
    expect,
    "the form read back from its tape measures the family's descent"
);
let mut k = Kernel::new();
let task = crate::registry::task_id(&crate::arc::Task {
    train: family.clone(),
    test: Vec::new(),
})
.expect("a grid pair's chain is A5's, so the task has an identity");
k.bind_task_family(&task, family);
let addr = spoken.address();
let codes = k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken));
assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
k.rule_all_provisionals();
// 47.3: the witness is a vantage with room. Bond it.
let cover = k.genesis_cover().expect("measured");
k.enroll([2; 32], cover);
k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
assert_eq!(
    k.treasury().minted().expect("minted adds up"),
    expect,
    "mint is the measured coboundary, not a bound listing's magnitude"
);
}

/// THE REFINEMENT BID (0x09): a bidder claims a word the rest of the
/// vocabulary already speaks; the kernel's deterministic replay confirms,
/// the word retires (compression), the bidder earns the yield. A bid on
/// an ESSENTIAL word is refused — nothing else derives it.
#[test]
fn a_refinement_bid_retires_a_derivable_word_and_pays_the_bidder() {
use crate::transducer::element::from_canonical;
let mut k = Kernel::new();
k.enroll([9; 32], crate::economy::UNIT_BOND); // capitalize the pool
// Two stale duplicates of the same deformation germinated far apart —
// identical substance after centering (what an import or a
// pre-consolidation era leaves). Neither is provisional: this is
// final vocabulary carrying a redundancy.
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
// Two words, one centered chain: the first carries a frame fibre (a
// distinct address, since a keyed basis holds one word per address)
// and the second speaks its chain exactly.
k.basis_insert(element::from_canonical_framed(&germinate(&a, &rot).unwrap(), &crate::arc::Frame::new(4, 4, 8, 8)), [1; 32]);
k.basis_insert(from_canonical(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
let derivable = k.basis()[0].address(); // its duplicate speaks it
// The bidder's sentence: the duplicate, cited at the identity.
let body = |w: &Element| crate::transducer::sentence::cited_pattern(w, 0).unwrap();
let sentence = vec![crate::anastome::dialogue::CiteWire {
    addr: k.basis()[1].address(),
    standing: 0,
    transport: element::identity_versor(),
    gauge: 0,
    body: body(&k.basis()[1]),
}];
let paid = k.try_refine(&derivable, [5; 32], &sentence);
assert!(paid.is_some(), "the derivable word is a confirmed compression bid");
assert_eq!(k.basis_len(), 1, "the vocabulary compressed");
let essential = k.basis()[0].address();
let self_cite = vec![crate::anastome::dialogue::CiteWire {
    addr: essential.clone(),
    standing: 0,
    transport: element::identity_versor(),
    gauge: 0,
    body: body(&k.basis()[0]),
}];
assert!(k.try_refine(&essential, [5; 32], &self_cite).is_none(), "an essential word is refused");
assert_eq!(k.basis_len(), 1, "refusal mutates nothing");
assert!(k.try_refine(&[3; 32], [5; 32], &sentence).is_none(), "an unknown address is refused");

// THE COURT'S BUSINESS STAYS THE COURT'S: a provisional word cannot be
// bid away, even if derivable.
k.basis_insert(
    element::from_canonical_framed(&germinate(&a, &rot).unwrap(), &crate::arc::Frame::new(3, 3, 9, 9)),
    [1; 32],
);
let addr = k.basis()[k.basis_len() - 1].address();
k.provisional.insert(&addr, k.basis()[k.basis_len() - 1].element().clone());
let sentence = vec![crate::anastome::dialogue::CiteWire {
    addr: k.basis()[0].address(),
    standing: 0,
    transport: element::identity_versor(),
    gauge: 0,
    body: body(&k.basis()[0]),
}];
assert!(k.try_refine(&addr, [5; 32], &sentence).is_none(), "provisional words are refused");
assert_eq!(k.basis_len(), 2);
}

/// §6, on a kernel that has actually moved value rather than a treasury in
/// isolation. This is the gate on 1.10: settlement paths may only be built
/// where conservation is checkable, and `enroll` used to insert bond that
/// was never issued, so the second equation failed at the first enrolment.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn the_kernel_conserves_across_genesis_bond_and_a_settlement() {
use crate::transducer::element::from_canonical;
let mut k = Kernel::new();
// Kernel::new funds the hypha at genesis and stakes its bond from that.
let seeded = k.treasury().audit().expect("genesis and its bond conserve");
assert!(seeded.iter().any(|&x| x > 0), "the hypha is funded");
assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "seeded is not earned");

let a = vec![(0, 0, 1), (0, 1, 2)];
let b = vec![(0, 0, 5), (0, 1, 2)];
k.basis_insert(from_canonical(&germinate(&a, &b).expect("coboundary")), [1; 32]);
k.treasury().audit().expect("an install conserves");

// Two voices docket the same residual, then the court rules on it.
// E1: an escrow holds μ⃗ that does not exist yet, so it is a claim on
// issuance and not supply (§3.1, §6). A ruling discharges it into a
// mint; a revocation makes it vanish with nothing ever issued. Counting
// it as issued reported value the mesh had not created, and the audit
// failed here at the first docket.
let _ = speak_pair(&mut k, &a, &b, [1; 32]);
let _ = speak_pair(&mut k, &a, &b, [2; 32]);
let docketed = k.treasury().audit().expect("a claim is not yet supply");
assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "no claim has been ruled");

// NOTHING FUNDS AN ARRIVING IDENTITY (§3.1 as ruled 2026-09-09, S13.15).
// Supply used to grow from the seed to three times it across these two
// ingests: `ensure_bonded` funded every arriving identity at the genesis
// cover so it could bond into room — a faucet, free μ⃗ per key, since
// identities are free (47.3). Speaking needs no bond (an escrow is a
// claim, E1); bond buys room, and an unbonded voice has room for one
// claim in flight per lane. The seed is the supply.
assert_eq!(docketed, seeded, "no allocation per arriving identity: the seed is the supply");
assert_eq!(k.treasury().bond_of(&[1; 32]), None, "the arriving voice is unbonded");
assert_eq!(k.treasury().bond_of(&[2; 32]), None);

k.glue_local();
let held = k.treasury().audit().expect("a glue conserves");
let addrs: Vec<Address> = k.dockets().iter().map(|d| d.addr.clone()).collect();
for a in addrs {
    let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
}
let after = k.treasury().audit().expect("a verdict conserves");
assert!(
    after.iter().zip(held.iter()).all(|(x, y)| x >= y),
    "a ruling mints and never unmakes: {held:?} then {after:?}"
);
}
