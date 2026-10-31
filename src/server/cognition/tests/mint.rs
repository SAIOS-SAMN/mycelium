// 26, E2 — WHAT MINTS AND WHAT DOES NOT: one vantage mints nothing until a
// witness re-derives; association with a listing is not a mint; a rule that
// speaks its family mints what it measured.

use super::common::*;
use crate::cognition::*;
use crate::transducer::germinate::germinate;

/// ONE VANTAGE DOCKETS: a sentence this court replayed is listed on the
/// book, escrowed as a claim, and Offered — and mints NOTHING. The second
/// vantage is a witness hypha that re-derives the same address and has room
/// (47.3); on its arrival the escrow releases and the deflation mints. No
/// number of voices substitutes for it, and this asserted a two-voice
/// quorum before convergence replaced consensus.
#[test]
fn confirm_peer_holds_mints_only_after_witness_rederives() {
let mut home = Court::new();
let mut peer = Court::new();
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
// it at speech (23.8), so the install measures what the sentence
// closes (E2).
assert!(home.family_of(&addr).is_some(), "the court bound the sentence to its task's family");
assert!(home.offered(&addr), "the court Offers a first-vantage germ");
assert!(home.treasury().escrow_of(&addr).is_some(), "the priced magnitude is the claim");
assert_eq!(home.treasury().minted().expect("minted adds up"), [0; 4], "the claim is not yet payment: nothing minted");
// 78.153: every stroke the docket's replay takes.
home.court_settle();
let spoken = home.provisional.get(&addr).cloned().expect("still provisional until a peer glues");
let mut cover = crate::network::nerve::Cover::new();
assert!(matches!(fetched(&mut peer, &spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
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
fn confirm_peer_holds_addrs_mints_nothing_for_unclosing_speech() {
let mut k = Court::new();
// A restatement that MOVES (one column): motionless, a σ alone is a colour
// word, which stands only by its family's evidence (78.53).
let w = crate::transducer::element::from_connection(
    &crate::transducer::alphabet::Word::identity(),
    &crate::transducer::element::translator_versor(1, 0),
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
fn bind_listing_task_mints_nothing_without_replay() {
let mut k = Court::new();
let family = vec![(
    digits(&["1100", "0000"]),
    digits(&["0011", "0000"]),
)];
let need = crate::transducer::family::refine::need_of(&family[0].0, &family[0].1).expect("both extents stand");
let crate::arc::Refined { delta: chain, frame, .. } =
    crate::arc::refine_delta(&family[0].0, &family[0].1, &mut crate::physics::Physics::default().stroke_n(need))
        .expect("coboundary");
assert!(k.intake(&chain, &frame), "the listing stands");
let listing = k.pending.scoped().next().expect("listed").1.address();
let task = [7u8; 32];
k.bind_listing_task(&listing, task);
k.bind_task_family(task, family);
// A restatement that MOVES (one column): motionless, a σ alone is a colour
// word, which stands only by its family's evidence (78.53).
let w = crate::transducer::element::from_connection(
    &crate::transducer::alphabet::Word::identity(),
    &crate::transducer::element::translator_versor(1, 0),
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
fn mint_sentence_mints_measured_deflation() {
let family = vec![
    (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
    (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
];
let rule = crate::transducer::family::extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal, 0).expect("mint");
assert!(spoken.is_sentence());
let mut expect = [0u64; 4];
for (x, y) in &family {
    let need = crate::transducer::family::refine::need_of(x, y).expect("both extents stand");
    let chain = crate::arc::refine_delta(x, y, &mut crate::physics::Physics::default().stroke_n(need))
        .expect("coboundary")
        .delta;
    let lanes = crate::transducer::element::measure_lanes(&chain);
    for i in 0..4 {
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): the fixture family's chain measures cannot pass u64")]
        let e = expect[i].saturating_add(lanes[i]);
        expect[i] = e;
    }
}
assert!(expect.iter().any(|&x| x > 0), "the family has a coboundary to close");
let decoded = crate::transducer::sentence::from_element_form(&spoken).expect("v2 form");
let read = crate::transducer::family::form_deflation(&decoded, &family, &crate::transducer::family::NoCrystal).0;
// 54a82051 (ruled: 𝒯 pays the odometer, never the wall clock): 𝒯 is
// the ground the closure covered — the null flux of the closing rule's
// motions, which the sentence's own magnitude carries — and the chain
// carries no 𝒯 of its own. S, G and H are the family's coboundary.
let t = crate::transducer::rewrite::LANE_TEMPORAL;
for l in (0..4).filter(|&l| l != t) {
    assert_eq!(read[l], expect[l], "lane {l}: the form read back from its tape measures the family's coboundary");
}
assert_eq!(read[t], spoken.magnitude()[t], "𝒯: the ground the closure covered, as the sentence carries it");
let mut k = Court::new();
let task = crate::registry::task_address(&crate::arc::Task {
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
    read,
    "mint is what the form measured over its family — the coboundary and the ground it covered — not a bound listing's magnitude"
);
}

/// §6, on a court that has actually moved value rather than a treasury in
/// isolation. This is the gate on 1.10: settlement paths may only be built
/// where conservation is checkable, and `enroll` used to insert bond that
/// was never issued, so the second equation failed at the first enrolment.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
#[test]
fn audit_conserves_across_genesis_bond_and_settlement() {
use crate::transducer::element::from_canonical;
let mut k = Court::new();
// Court::new funds the hypha at genesis and stakes its bond from that.
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

/// Y3 (50.5, E2, §8.1): A sentence of three word vertices and seven demands
/// mints on its closed sub-complex, and the amount is the deflation those
/// three closed and nothing else; a sentence of zero word vertices mints nothing.
#[test]
fn mint_sentence_mints_closed_subcomplex_deflation() {
    use crate::transducer::sentence::{Demand, Vertex};
    use crate::transducer::stalk;
    let mut x = vec![vec![0u8; 12]; 18];
    // Body 1: length 1 at (0, 0)
    x[0][0] = 1;
    // Body 2: length 2 at (0, 2), (0, 3)
    x[0][2] = 2;
    x[0][3] = 2;
    // Body 3: length 3 at (0, 5), (0, 6), (0, 7)
    x[0][5] = 3;
    x[0][6] = 3;
    x[0][7] = 3;

    let mut y_recoloured = vec![vec![0u8; 12]; 18];
    y_recoloured[0][0] = 4;
    y_recoloured[0][2] = 5;
    y_recoloured[0][3] = 5;
    y_recoloured[0][5] = 6;
    y_recoloured[0][6] = 6;
    y_recoloured[0][7] = 6;
    let pure_family = vec![(x.clone(), y_recoloured.clone())];
    let mut pf = crate::transducer::family::cipher_form(&x, &y_recoloured);
    // THREE BODIES OF ONE TRANSPORT CLASS — each recoloured in place — are
    // one word vertex acting on three charts (D7, 21 step 1), each member
    // carrying its own hue map.
    let pure_words: usize = pf
        .vertices
        .entries()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .map(|(_, v)| v.acts().map_or(0, |a| a.len()))
        .sum();
    assert_eq!(pure_words, 3, "three bodies recoloured: three members of the closed sub-complex");
    let (three_closed_deflation, _) = crate::transducer::family::form_deflation(
        &pf,
        &pure_family,
        &crate::transducer::family::NoCrystal,
    );
    assert!(
        three_closed_deflation.iter().any(|&d| d > 0),
        "three recolours achieve positive deflation"
    );

    let demand_shapes: Vec<Vec<(i64, i64)>> = vec![
        vec![(3, 0), (3, 1), (3, 2), (3, 3)],
        vec![(5, 0), (5, 1), (5, 2), (5, 3), (5, 4)],
        vec![(7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5)],
        vec![(9, 0), (9, 1), (9, 2), (9, 3), (9, 4), (9, 5), (9, 6)],
        vec![(11, 0), (11, 1), (11, 2), (11, 3), (11, 4), (11, 5), (11, 6), (11, 7)],
        vec![(13, 0), (13, 1), (13, 2), (13, 3), (13, 4), (13, 5), (13, 6), (13, 7), (13, 8)],
        vec![(15, 0), (15, 1), (15, 2), (15, 3), (15, 4), (15, 5), (15, 6), (15, 7), (15, 8), (15, 9)],
    ];
    let mut y_with_7_creations = y_recoloured.clone();
    for s in &demand_shapes {
        for &(r, c) in s {
            let (r, c) = (
                usize::try_from(r).expect("a literal row of the fixture's grid"),
                usize::try_from(c).expect("a literal column of the fixture's grid"),
            );
            y_with_7_creations[r][c] = 7;
        }
    }
    let family = vec![(x.clone(), y_with_7_creations.clone())];

    for s in &demand_shapes {
        let links: Vec<(i64, i64, u8, u8)> = s.iter().map(|&(r, c)| (r, c, 0u8, 7u8)).collect();
        let chart = stalk::invariant(&links);
        pf.put_vertex(Vertex::demand(Demand::Creation, chart, links)).expect("a demand has a coordinate");
    }
    pf.reindex();
    let rule = crate::transducer::sentence::rule_of_form(pf);

    let words_count: usize = rule
        .form
        .vertices
        .entries()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .map(|(_, v)| v.acts().map_or(0, |a| a.len()))
        .sum();
    let demands_count = rule.form.demand_vertices().count();
    assert_eq!((words_count, demands_count), (3, 7), "three members act, seven demands stand");

    let spoken = crate::transducer::family::mint_sentence(
        &rule,
        &family,
        &crate::transducer::family::NoCrystal,
        0,
    )
    .expect("a sentence of three word vertices and seven demands mints on its closed sub-complex");
    assert!(spoken.is_sentence());

    let mut zero_word_rule = rule.clone();
    let words: Vec<_> = zero_word_rule
        .form
        .vertices
        .entries()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .map(|(k, _)| k)
        .collect();
    assert!(zero_word_rule.form.retract_vertices(&words), "the Frame and the demands keep a depth");
    zero_word_rule.form.reindex();
    assert_eq!(
        zero_word_rule
            .form
            .vertices
            .entries()
            .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
            .count(),
        0
    );
    assert!(
        crate::transducer::family::mint_sentence(
            &zero_word_rule,
            &family,
            &crate::transducer::family::NoCrystal,
            0,
        )
        .is_none(),
        "a sentence of zero word vertices mints nothing"
    );

    let mut k = Court::new();
    let task = crate::registry::task_address(&crate::arc::Task {
        train: family.clone(),
        test: Vec::new(),
    })
    .expect("task identity");
    k.bind_task_family(&task, family);
    let addr = spoken.address();
    let codes = k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken));
    assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
    assert_eq!(
        k.treasury().minted().expect("minted adds up"),
        three_closed_deflation,
        "the amount is the deflation those three closed and nothing else"
    );
}

/// A SENTENCE IS TAUGHT AT ITS LEARNER'S EXPENSE (ruled
/// 2026-09-30). An installed sentence served on a syllabus page is paid
/// from the voice's balance, never from the court's reserve: the reserve
/// stands where it stood, the voice pays the sentence's gluing step to the
/// court, and a voice that cannot pay is deferred with its own shortfall.
#[test]
fn serve_syllabus_charges_learner_not_reserve() {
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
    ];
    let rule = crate::transducer::family::extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
    let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal, 0).expect("mint");
    let mut k = Court::new();
    let task = crate::registry::task_address(&crate::arc::Task { train: family.clone(), test: Vec::new() }).expect("a task");
    k.bind_task_family(&task, family);
    let addr = spoken.address();
    assert_eq!(k.perceive_speech([1; 32], task.as_bytes(), std::slice::from_ref(&spoken)), vec![crate::anastome::dialogue::SPEECH_STOOD]);
    k.rule_all_provisionals();
    let cover = k.genesis_cover().expect("measured");
    k.enroll([2; 32], cover);
    k.confirm_peer_holds_addrs(std::slice::from_ref(&addr), [2; 32]);
    let cost = k.installed_word(addr.as_bytes()).expect("installed").magnitude();
    assert!(cost.iter().any(|&c| c > 0), "the sentence carries a cost");
    // A VOICE THAT CANNOT PAY waits, and nothing moves.
    let poor = [9u8; 32];
    let reserve = k.physics.reserve.lanes();
    let page = k.serve_syllabus(&[addr.as_bytes().to_vec()], poor, 1 << 20);
    assert!(page.words.is_empty() && page.deferred.len() == 1, "a voice with nothing waits");
    assert_eq!(k.physics.reserve.lanes(), reserve, "and the reserve is untouched");
    let [d] = k.deferred_reading().try_into().expect("one deferral");
    assert_eq!(d.learner, Some([9; 8]), "the learner that fell short is named as its payer");
    // A VOICE THAT CAN PAY is taught, and pays the court.
    let rich = [8u8; 32];
    k.enroll(rich, cover);
    k.fund(rich, cost);
    let before = k.treasury().balance_mu(&rich);
    let court = k.treasury().balance_mu(&[0x11; 32]);
    let page = k.serve_syllabus(&[addr.as_bytes().to_vec()], rich, 1 << 20);
    assert_eq!(page.words.len(), 1, "the sentence is taught: cost {cost:?} balance {before:?} deferred {:?} missed {}", page.deferred, page.missed.len());
    assert_eq!(k.physics.reserve.lanes(), reserve, "from the learner's balance, never the court's reserve");
    let after = k.treasury().balance_mu(&rich);
    assert_eq!(std::array::from_fn::<u64, 4, _>(|l| before[l] - after[l]), cost, "the learner paid the sentence's step");
    let court_after = k.treasury().balance_mu(&[0x11; 32]);
    assert_eq!(std::array::from_fn::<u64, 4, _>(|l| court_after[l] - court[l]), cost, "to the court that taught it");
    assert!(k.treasury().audit().is_ok(), "the ledger conserves");
    // THE ECONOMY READS IT: one payment on the journal, and it is tuition
    // to this court.
    let economy = k.economy().expect("measured");
    let payment = crate::observe::FLOWS.iter().position(|f| *f == "payment").expect("a named flow");
    assert_eq!(economy.flows[payment], crate::observe::Flow { rows: 1, mu: cost }, "one payment, its vector once");
    assert_eq!(economy.tuition, crate::observe::Flow { rows: 1, mu: cost }, "paid to this court");
    // 78.60 — the learner held nothing on the sentence's chart from this
    // court: the sentence stood unanchored.
    let acts = k.acts_reading();
    let axis = |n: &str| crate::observe::SYLLABUS.iter().position(|a| *a == n).expect("a named axis");
    assert_eq!(acts.syllabus[axis("sentences: unanchored")], 1, "{:?}", acts.lines());
    assert_eq!(acts.syllabus[axis("sentences: refracted (held, Γ)")], 0);
}

/// 78.60 — Γ IS SOLVED BACK FROM THE TWO FORMS, never searched for: a
/// minted sentence is refracted by a translation of one column, and the
/// relation read off the held form and the refracted one is a refraction
/// whose Γ lands on the refracted form's address — it could travel as
/// (held, Γ). The sentence's words are unit rotors that move by the words'
/// carriages, so the placement half of Γ is what is solved here. A form
/// stands refracted from itself.
#[test]
fn relation_solves_refraction_from_two_forms() {
    use crate::transducer::sentence::{as_element_form, from_element_form, relation, Relation, Transport};
    let family = vec![
        (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
    ];
    let rule = crate::transducer::family::extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
    let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal, 0).expect("mint");
    let held = from_element_form(&spoken).expect("a form");
    let gamma = Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: crate::transducer::element::translator_versor(1, 0),
        gauge: Vec::new(),
    };
    let out = gamma.refract(&held).expect("Γ keeps the sensor plane");
    let address = |f: &crate::transducer::sentence::Form| as_element_form(f, [0, 0, 0, 1]).map(|e| e.address());
    assert!(address(&held).is_some() && address(&out).is_some(), "both forms are words");
    assert_ne!(address(&held), address(&out), "the refraction moved the sentence");
    let Relation::Refracted(gamma) = relation(&held, &out) else {
        panic!("solved back: {:?} (moved: {})", relation(&held, &out), address(&held) != address(&out));
    };
    assert_eq!(gamma.refract(&held).and_then(|g| address(&g)), address(&out), "the Γ solved lands where the refraction did");
    assert!(matches!(relation(&held, &held), Relation::Refracted(_)), "a form is its own refraction by the identity");
}

/// 78.61 C1 — A WORD IS LIFTED INTO PERCEPTION AND COMPARED THERE: the same
/// minted sentence written in two other atlases — translated by a column,
/// and turned 180° in (e₁, e₂) (an orientation reversal of the sensor plane
/// the exact one-Γ solve could not even spell) — lifts to the same
/// invariants as the original, and a sentence of another family leaves a
/// residue on both sides. The lift reads real paths (never agrees on
/// nothing).
#[test]
fn lift_compares_coherent_across_atlases() {
    use crate::transducer::sentence::{compare, from_element_form, lift, Transport};
    type Family = [(Vec<Vec<u8>>, Vec<Vec<u8>>)];
    let mint = |family: &Family| {
        let rule = crate::transducer::family::extract_rule(family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("extract");
        let spoken = crate::transducer::family::mint_sentence(&rule, family, &crate::transducer::family::NoCrystal, 0).expect("mint");
        from_element_form(&spoken).expect("a form")
    };
    let held = mint(&[
        (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
        (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
    ]);
    let budget = || crate::physics::Physics::default().stroke_n(1 << 20);
    let held_lift = lift(&held, &mut budget());
    assert!(held_lift.lanes.iter().all(|l| l.scoped().count() > 0), "the lift reads every lane: {} unread", held_lift.unread);
    let slid = Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: crate::transducer::element::translator_versor(1, 0),
        gauge: Vec::new(),
    }
    .refract(&held)
    .expect("the slide refracts");
    let flip = crate::transducer::alphabet::LetterId::turn_90((1, 2)).expect("a letter");
    let flipped = Transport::from_word(crate::transducer::alphabet::Word::from_letters(&[(flip, false), (flip, false)]))
        .refract(&held)
        .expect("the flip keeps the sensor plane");
    for (name, other) in [("translated", &slid), ("flipped", &flipped)] {
        let c = compare(&held_lift, &lift(other, &mut budget()));
        assert!(c.coherent(), "{name}: the same word in another atlas is coherent on all four lanes: {c:?}");
    }
    let other = mint(&[
        (digits(&["1000", "0000"]), digits(&["0000", "0001"])),
        (digits(&["0100", "0000"]), digits(&["0000", "0010"])),
    ]);
    let c = compare(&held_lift, &lift(&other, &mut budget()));
    assert!(!c.diverging().is_empty() && c.unread == (0, 0), "another family's sentence diverges on a lane, read whole: {c:?}");
    eprintln!("LIFT diverging lanes {:?}", c.diverging().iter().map(|&i| crate::transducer::sentence::LANES[i]).collect::<Vec<_>>());
}
