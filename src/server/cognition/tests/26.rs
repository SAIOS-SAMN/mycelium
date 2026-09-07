// 26 — THE MARKET: the five-act conversation, the docket and its room,
// and what does and does not mint.

use super::common::*;
use crate::cognition::*;
use crate::transducer::germinate::germinate;
use crate::transducer::element::{self, Element};

/// THE REFINEMENT BID (0x09): a bidder claims a word the rest of the
/// vocabulary already speaks; the kernel's deterministic replay confirms,
/// the word retires (compression), the bidder earns the yield. A bid on
/// an ESSENTIAL word is refused — nothing else derives it.
#[test]
fn a_refinement_bid_retires_a_derivable_word_and_pays_the_bidder() {
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
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
k.basis_insert(element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(4, 4, 8, 8)), [1; 32]);
k.basis_insert(from_residual(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
let derivable = k.basis()[0].address(); // its duplicate speaks it
// The bidder's sentence: the duplicate, cited at the identity.
let body = |w: &Element| crate::transducer::sentence::cited_pattern(w, 0).unwrap();
let sentence = vec![(k.basis()[1].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[1]))];
let paid = k.try_refine(&derivable, [5; 32], &sentence);
assert!(paid.is_some(), "the derivable word is a confirmed compression bid");
assert_eq!(k.basis_len(), 1, "the vocabulary compressed");
let essential = k.basis()[0].address();
let self_cite = vec![(essential.clone(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[0]))];
assert!(k.try_refine(&essential, [5; 32], &self_cite).is_none(), "an essential word is refused");
assert_eq!(k.basis_len(), 1, "refusal mutates nothing");
assert!(k.try_refine(&[3; 32], [5; 32], &sentence).is_none(), "an unknown address is refused");

// THE COURT'S BUSINESS STAYS THE COURT'S: a provisional word cannot be
// bid away, even if derivable.
k.basis_insert(
    element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(3, 3, 9, 9)),
    [1; 32],
);
let addr = k.basis()[k.basis_len() - 1].address();
k.provisional.insert(&addr, (*k.basis()[k.basis_len() - 1]).clone());
let sentence = vec![(k.basis()[0].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis()[0]))];
assert!(k.try_refine(&addr, [5; 32], &sentence).is_none(), "provisional words are refused");
assert_eq!(k.basis_len(), 2);
}

/// STEP 3's ECONOMIC CONVERSATION CLOSES: A speaks a novel PERCEPT (the germ
/// pends as a tape, not quoted); A's empty-cites demand lists the remainder
/// on the quote frontier; B independently attests; consilience installs,
/// the market CLEARS. {demand → quote → demand → settle}.
#[test]
fn the_five_act_conversation_closes_with_settlement() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "percept: voice A");
assert_eq!(k.quote().1.len(), 1, "corpus demand is quoted when the frontier is dry");
let chain = germinate(&a, &b).unwrap();
assert_eq!(k.attest(&chain, [1; 32]), Outcome::GermPending, "remainder-of-g / unreached demand");
let (surface, quoted) = k.quote();
assert_eq!(quoted.len(), 1, "the unreached/remainder demand is quoted as work");
assert!(surface.iter().any(|&s| s >= 1), "the open demand tensions the lanes it occupies");
let (work, mu) = &quoted[0];
assert!(
    (0..4).any(|l| work.magnitude()[l] > 0 && mu[l] > 0),
    "price is nonzero on a lane the word moves"
);
let chain = work.sub().to_vec();
let out = k.attest(&chain, [2; 32]);
assert_eq!(out, Outcome::GermPending, "consilience settles: {out:?}");
k.glue_local();
assert_eq!(k.voices_of(&k.basis()[0].address()).len(), 2, "the settle parties");
assert!(k.quote().1.is_empty(), "the market cleared");
}

/// A docket draws the chain's μ⃗ from the uttering voice's remaining bond;
/// a short lane refuses and nothing is listed. A second voice's bond is
/// its own; the hypha's identity is not the drawer.
#[test]
fn docket_draws_room_and_refuses_when_short() {
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
let c = vec![(1, 0, 3u8)];
let d = vec![(1, 0, 4u8)];
let mut short = Kernel::new();
short.set_lane_cap([8, 1, 8, 8]);
let _ = short.ingest(&a, &b, [1; 32]);
assert_eq!(short.pending.len(), 1, "the first draw covers");
let _ = short.ingest(&c, &d, [1; 32]);
let _ = &short;
// ONE VANTAGE DOCKETS, so the first pair is listed on the first
// ingest rather than the second; the G lane is capped at 1 and the
// second pair's draw does not cover, so it is refused and never
// reaches the book.
assert_eq!(
    short.pending.len(),
    1,
    "a lane that does not cover the draw refuses, nothing further docketed"
);

let mut bonded = Kernel::new();
let cover = crate::arc::family_cover(&[(vec![vec![1]], vec![vec![2]])])
    .expect("one 1-cell pair measures");
bonded.bind_identity([9; 32]);
bonded.set_genesis_cover(cover);
bonded.enroll([1; 32], cover);
bonded.enroll([2; 32], cover);
let chain_ab = germinate(&a, &b).unwrap();
let chain_cd = germinate(&c, &d).unwrap();
assert_eq!(bonded.attest(&chain_ab, [1; 32]), Outcome::GermPending);
assert_eq!(bonded.pending.len(), 1, "the cover admits one pair");
assert_eq!(bonded.attest(&chain_cd, [1; 32]), Outcome::GermPending);
assert_eq!(
    bonded.pending.len(),
    1,
    "a second pair of the same μ⃗ waits until the first draw releases"
);
assert_eq!(bonded.attest(&chain_cd, [2; 32]), Outcome::GermPending);
assert_eq!(
    bonded.pending.len(),
    2,
    "a second voice's own bond covers its own draw"
);
}

/// A listing that stands an aperture of receipts behind the head
/// releases its draw; occupancy returns; nothing is slashed.
#[test]
fn docket_draw_releases_on_lapse() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(k.pending.len(), 1, "listed");
let occ = k.occupancy();
assert!(occ.iter().any(|&x| x > 0), "the draw occupies");
let lambda = crate::physics::aperture_cycle() as u64;
k.set_height(k.height().saturating_add(lambda));
k.court_stroke();
assert!(k.pending.is_empty(), "the listing lapsed");
assert_eq!(k.occupancy(), [0; 4], "the draw released");
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == "slash"),
    "lapse does not slash"
);
}

/// D12 — THE ESCROW-PRICED DOOR: a calm mesh keeps a free door; a
/// saturated mesh charges congestion (fee → the pool) and DEFERS a
/// speaker who can pay nothing on a fully exhausted lane — spam costs
/// exactly when strain is real, and paying members pass.
#[test]
fn the_door_charges_strain_and_defers_the_broke() {
let mut k = Kernel::new();
// Calm: the door is free — a normal percept adjudicates untouched.
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "calm door admits");
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == "fee"),
    "no fee on a calm mesh"
);
// Exhaust the metabolism entirely (every lane to zero reserve).
for _ in 0..16 {
    k.physics_mut().spend([true; 4]);
}
// A broke speaker (no earned balance) is DEFERRED at the door.
let a2 = vec![(5, 0, 2), (5, 1, 2)];
let b2 = vec![(5, 0, 6), (5, 1, 6)];
let broke = k.ingest(&a2, &b2, [9; 32]);
assert_eq!(broke, Outcome::GermPending, "saturated door defers the broke: {broke:?}");
assert!(k.basis().is_empty() || k.quote().1.iter().all(|(w, _)| w.sub() != [(0i64, 0i64, 2u8, 6u8)]),
    "the deferred demand was never adjudicated");
// A FUNDED speaker pays the congestion fee and passes the door.
k.treasury_mut().settle_subsidy([7; 32], [8, 8, 8, 8], (0, [0; 32]));
for _ in 0..16 {
    k.physics_mut().spend([true; 4]); // re-exhaust (the deferral relaxed some)
}
let _ = k.ingest(&a2, &b2, [7; 32]);
assert!(
    k.treasury().entries().iter().any(|e| e.kind == "transfer" && e.who == [7; 32]),
    "the funded speaker paid the door as a transfer"
);
}

/// THE CONVERSATION: a listing is probed with a cited sentence. Graded
/// descent settles the reduction and the residual replaces the listing
/// under its frame; closure clears it and advances the lineage; a
/// sentence that names nothing settles nothing. The replay is linear in
/// the proof and the hypha runs no search.
#[test]
fn a_probe_settles_descent_then_closure_without_search() {
use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
use crate::transducer::sentence::replay_at;
let mut k = Kernel::new();
// A coboundary listed on an empty language (the residual path).
let c = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 1), (0, 3, 3)];
let d = vec![(0, 0, 2u8), (0, 1, 2), (0, 2, 1), (0, 3, 4)];
let chain = germinate(&c, &d).unwrap();
assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
let target = k.quote().1[0].0.address();
let chain = k.quote().1[0].0.sub().to_vec();
assert_eq!(chain.len(), 3);
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
k.basis_insert(element::from_residual(&germinate(&a, &b).unwrap()), [2; 32]);
let _word = k.basis()[0].address();
// The entity's reduction over the language: the word cited at the
// origin, the far cell left as the residual.
let language = k.language();
let sentence = crate::transducer::sentence::deconstruct_at(&|i| language.get(i), &language.stalks, &|a| language.index.get(a).copied(), &|i| language.shape(i), &chain, None);
let (void, cites) = (sentence.residual.clone(), sentence.cites.clone());
assert_eq!(void, vec![(0, 3, 3, 4)]);
assert_eq!(cites.len(), 1);
let basis = k.basis();
assert_eq!(
    replay_at(&|i| basis.get(i).map(|w| w.as_ref()), &cites, &chain),
    void,
    "the replay reproduces the residual"
);
let (code, paid, remaining) = k.settle_probe([3; 32], &target, void.clone(), &[], false);
assert_eq!(code, PROBE_DESCENT);
assert_eq!(paid, [0; 4], "a descent is measured, never paid: covering cells opens no axis");
assert_eq!(remaining, crate::transducer::element::measure_lanes(&void));
let listing = k.quote().1[0].0.clone();
assert_eq!(listing.address(), target, "the listing stays the body: a descent never re-lists a fragment");
assert_eq!(
    k.tension.get(&target).copied(),
    Some(crate::transducer::element::measure_lanes(&void)),
    "the body's remaining tension is the residual's measure, keyed by the listing"
);
// No descent settles nothing; closure clears the book.
let t2 = listing.address();
assert_eq!(k.settle_probe([3; 32], &t2, listing.sub().to_vec(), &[], false).0, PROBE_NOTHING);
assert_eq!(k.settle_probe([3; 32], &t2, Vec::new(), &[], false).0, PROBE_CLOSED);
assert!(k.quote().1.is_empty(), "the listing cleared");
}

/// Descent records a remainder face and a holonomic cell-count, stagnation
/// counts only unmoved occupied lanes, and a full aperture of failed
/// probes sheds the listing as a cavity.
#[test]
fn descent_opens_a_face_and_drain_sheds_the_unmoved() {
use crate::anastome::dialogue::PROBE_DESCENT;
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8)];
let chain = germinate(&a, &b).unwrap();
assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
let target = k.quote().1[0].0.address();
let listed = k.quote().1[0].0.sub().to_vec();
let rem = vec![(0, 1, 1u8, 0u8)];
let before = k.tension.get(&target).copied().unwrap_or_else(|| k.quote().1[0].0.magnitude());
let after = crate::transducer::element::measure_lanes(&rem);
assert_ne!(after, before, "the residual is a proper descent");
let (code, _, remaining) = k.settle_probe([3; 32], &target, rem.clone(), &[], false);
assert_eq!(code, PROBE_DESCENT);
assert_eq!(remaining, after);
let listing = k.quote().1[0].0.clone();
assert_eq!(listing.address(), target, "the listing stays the body");
assert_eq!(listing.magnitude()[3], rem.len() as u64, "holonomic demand is the remainder cell count");
let rem_inv = crate::transducer::stalk::invariant(&rem);
assert_eq!(k.face_of(&target).cloned(), Some(rem_inv.clone()));
assert!(
    k.listing_atlas.fiber(&rem_inv).iter().any(|a| a.as_slice() == target.as_bytes()),
    "the remainder chart admits the listing"
);
let base_inv = crate::transducer::stalk::invariant(&listed);
assert_eq!(
    k.listing_atlas.chart_of(&target).cloned().as_ref(),
    Some(&base_inv),
    "base moduli are never written"
);
let s = k.stagnant_of(&target);
for l in 0..4 {
    if before[l] > 0 && after[l] == before[l] {
        assert!(s[l] >= 1, "unmoved occupied lane {l} stagnates: {s:?}");
    } else {
        assert_eq!(s[l], 0, "a moved or empty lane {l} resets: {s:?}");
    }
}
let mut listed_sorted = listed.clone();
listed_sorted.sort_unstable();
let kappa = crate::physics::aperture_cycle();
for _ in 0..kappa {
    assert_eq!(
        k.settle_probe([4; 32], &target, listed_sorted.clone(), &[], false).0,
        crate::anastome::dialogue::PROBE_NOTHING
    );
}
let s = k.stagnant_of(&target);
let t = k.tension.get(&target).copied().unwrap();
assert!(
    (0..4).all(|l| t[l] == 0 || s[l] >= kappa),
    "every occupied lane sat a full cycle: tension {t:?} stagnant {s:?} kappa {kappa}"
);
k.drain_now();
assert!(k.quote().1.is_empty(), "the unreachable listing left the page");
assert_eq!(k.cavities().len(), 1, "the cavity is the relocated element");
assert_eq!(k.cavities()[0].address(), target);
assert_eq!(k.cavities()[0].magnitude()[3], rem.len() as u64);
assert!(
    k.listing_atlas.fiber(&rem_inv).iter().any(|a| a.as_slice() == target.as_bytes()),
    "drain relocates: the listing stays on the atlas"
);
assert_eq!(
    k.listing_atlas.chart_of(&target).cloned().as_ref(),
    Some(&base_inv),
    "base moduli stay on the cavity chart"
);
assert!(
    k.field_cold.get(&target).is_some(),
    "the cold field is readable by key"
);
assert!(k.cavity_book.get(&target).is_some(), "the element is keyed on the cavity book");
let listing = k.field_cold.get(&target).expect("relocated field is cold beside CAV1");
let mut seen = Vec::new();
for e in &listing.entries {
    assert!(!seen.contains(&e.chart), "a chart appears once per listing");
    seen.push(e.chart.clone());
}
let bytes = k.snapshot();
let mut k2 = Kernel::new();
assert!(k2.restore(&bytes), "the field persists beside DEF1");
assert!(k2.cavity_book.get(&target).is_some(), "a restart keeps the cavity");
assert!(
    k2.field_cold.get(&target).is_some_and(|f| !f.entries.is_empty()),
    "a restart does not forget which angles were probed"
);
}

/// THE PROOF: on the wire the judge replays; a proven percept commits its
/// sentence, an unproven one is deferred, never searched.
#[test]
fn the_wire_judge_replays_proofs_and_defers_the_unproven() {
let k = std::sync::Mutex::new(Kernel::new());
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
{
    let mut g = k.lock().unwrap();
    g.ingest(&a, &b, [1; 32]);
    g.ingest(&a, &b, [2; 32]);
    g.glue_local();
}
{
    let g = k.lock().unwrap();
    assert!(!g.basis().is_empty(), "the pair form stands");
}
let (ca, cb) = perceive(&a, &b);
let frame = frame_of(&a, &b);
let out = crate::synthesis::adjudicate(&k, &ca, &cb, [3; 32], &frame, &[]);
assert!(
    matches!(out, Outcome::Decomposed(ref w) if w == &vec![0]),
    "the held pair form is the word, got {out:?}"
);
let a2 = vec![(0, 0, 1u8), (0, 1, 1), (1, 0, 7)];
let b2 = vec![(0, 0, 2u8), (0, 1, 2), (1, 0, 8)];
let (ca2, cb2) = perceive(&a2, &b2);
let frame2 = frame_of(&a2, &b2);
let out = crate::synthesis::adjudicate(&k, &ca2, &cb2, [3; 32], &frame2, &[]);
assert!(
    matches!(out, Outcome::GermPending | Outcome::Decomposed(_)),
    "a different pair is its own form or residual, got {out:?}"
);
}

/// ONE VANTAGE DOCKETS: a percept this court re-derived is listed on the
/// book, escrowed as a claim, and Offered — and mints NOTHING. The second
/// vantage is a witness hypha that re-derives the same address and has room
/// (47.3); on its arrival the escrow releases and the deflation mints. No
/// number of voices substitutes for it, and this asserted a two-voice
/// quorum before convergence replaced consensus.
#[test]
fn one_vantage_dockets_escrows_and_mints_nothing_until_a_witness_rederives() {
let mut home = Kernel::new();
let mut peer = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
assert!(!home.pending.is_empty(), "the listing stays on the book");
assert_eq!(home.provisional.len(), 1, "the residual is on the docket");
let addr = home.provisional.fiber(crate::network::nerve::ROOT).next().unwrap().0.clone();
assert!(home.offered(&addr), "the court Offers a first-vantage germ");
assert!(home.treasury().escrow_of(&addr).is_some(), "the priced magnitude is the claim");
assert_eq!(home.treasury().minted().expect("minted adds up"), [0; 4], "the claim is not yet payment: nothing minted");
home.court_stroke();
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
let (chain, _, frame) = crate::arc::refine_delta(&family[0].0, &family[0].1).expect("coboundary");
assert!(k.intake(&chain, &frame), "the listing stands");
let listing = k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address();
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
let rule = crate::transducer::family::extract_rule(&family).expect("extract");
let spoken = crate::transducer::family::mint_sentence(&rule, &family, &crate::transducer::family::NoCrystal).expect("mint");
assert!(spoken.is_sentence());
let mut expect = [0u64; 4];
for (x, y) in &family {
    let (chain, _, _) = crate::arc::refine_delta(x, y).expect("coboundary");
    let lanes = crate::transducer::element::measure_lanes(&chain);
    for i in 0..4 {
        expect[i] = expect[i].saturating_add(lanes[i]);
    }
}
assert!(expect.iter().any(|&x| x > 0), "the family has descent");
let decoded = crate::transducer::sentence::from_element_form(&spoken).expect("v2 form");
assert_eq!(
    crate::transducer::family::form_descent(&decoded, &family, &crate::transducer::family::NoCrystal).0,
    expect,
    "the form read back from its tape measures the family's descent"
);
let mut k = Kernel::new();
let task = crate::registry::task_id(&crate::arc::Task {
    train: family.clone(),
    test: Vec::new(),
});
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

/// A repeated (vk, address, K) is answered from the docket: same reply,
/// no re-docket, no re-charge. A later K is a use; another voice is
/// consilience.
#[test]
fn repeated_demand_is_answered_not_redocketed() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
let chain = germinate(&a, &b).unwrap();
let first = k.attest(&chain, [1; 32]);
assert_eq!(first, Outcome::GermPending, "one voice lists, does not install");
let n = k.pending.len();
assert_eq!(n, 1, "the first hearing dockets the residual");
let pressure = k.physics().pressure;
let occupancy = k.occupancy();
let second = k.attest(&chain, [1; 32]);
assert_eq!(second, first, "the docket answers the same receipt");
assert_eq!(k.pending.len(), n, "nothing re-docketed");
assert_eq!(k.physics().pressure, pressure, "nothing re-charged");
assert_eq!(k.occupancy(), occupancy, "room is not drawn again");
let used = k.attest_at(&chain, [1; 32], &[], 1);
assert_eq!(used, Outcome::GermPending, "a later K is a use, not a duplicate");
assert_eq!(k.pending.len(), n, "a use of a listed address does not re-docket");
let other = k.attest(&chain, [2; 32]);
assert_eq!(other, Outcome::GermPending, "another voice is consilience, not a duplicate");
}

/// The demand codec roundtrips.
#[test]
fn the_demand_codec_roundtrips() {
let (a, b) = (scene(2, 2), scene(2, 5));
let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
assert_eq!((v, ra, rb), ([9; 32], a, b));
}

/// Two hyphas at different K relocate the same standing listing at the
/// same TUniversal: T-room, never a stroke count.
#[test]
fn t_room_relocates_at_tuniversal_independent_of_k() {
let list = |k: &mut Kernel, extra: bool| -> Address {
    let mut a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let mut b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
    if extra {
        a.push((1, 1, 1));
        b.push((1, 1, 5));
    }
    let _ = k.ingest(&a, &b, [1; 32]);
    k.pending.fiber(crate::network::nerve::ROOT).next().expect("listed").1.0.address()
};
let mut ha = Kernel::new();
let mut hb = Kernel::new();
ha.set_lane_cap([64, 64, 1, 64]);
hb.set_lane_cap([64, 64, 1, 64]);
ha.set_height(10);
hb.set_height(10);
let first_a = list(&mut ha, false);
let first_b = list(&mut hb, false);
assert_eq!(first_a, first_b);
// Advance K on ha only — TUniversal stays the clock we set.
for _ in 0..8 {
    ha.append_step(&[1; 32], [2; 32]);
}
assert!(ha.k_step() > hb.k_step());
ha.set_height(40);
hb.set_height(40);
let _ = list(&mut ha, true);
let _ = list(&mut hb, true);
assert!(
    ha.cavity_book.contains(&first_a),
    "ha relocates the older listing on T"
);
assert!(
    hb.cavity_book.contains(&first_b),
    "hb relocates the same listing at the same TUniversal"
);
}

/// §6, on a kernel that has actually moved value rather than a treasury in
/// isolation. This is the gate on 1.10: settlement paths may only be built
/// where conservation is checkable, and `enroll` used to insert bond that
/// was never issued, so the second equation failed at the first enrolment.
#[test]
fn the_kernel_conserves_across_genesis_bond_and_a_settlement() {
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
// Kernel::new funds the hypha at genesis and stakes its bond from that.
let seeded = k.treasury().audit().expect("genesis and its bond conserve");
assert!(seeded.iter().any(|&x| x > 0), "the hypha is funded");
assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "seeded is not earned");

let a = vec![(0, 0, 1), (0, 1, 2)];
let b = vec![(0, 0, 5), (0, 1, 2)];
k.basis_insert(from_residual(&germinate(&a, &b).expect("coboundary")), [1; 32]);
k.treasury().audit().expect("an install conserves");

// Two voices docket the same residual, then the court rules on it.
// E1: an escrow holds μ⃗ that does not exist yet, so it is a claim on
// issuance and not supply (§3.1, §6). A ruling discharges it into a
// mint; a revocation makes it vanish with nothing ever issued. Counting
// it as issued reported value the mesh had not created, and the audit
// failed here at the first docket.
let _ = k.ingest(&a, &b, [1; 32]);
let _ = k.ingest(&a, &b, [2; 32]);
let docketed = k.treasury().audit().expect("a claim is not yet supply");
assert_eq!(k.treasury().minted().expect("minted"), [0; 4], "no claim has been ruled");

// PINNED, with its number. Supply grew from the seed to three times it
// across two ingests: `ensure_bonded` funds EVERY arriving identity at
// the genesis cover so it can bond into room. Identities are free
// (47.3), so that is a faucet — free μ⃗ per key — and it was invisible
// while `enroll` conjured bond instead of being funded.
//
// §4 says an unbonded voice "may learn, speak, and be paid; every claim
// is escrowed against its own deflation", and under E1 an escrow is a
// claim rather than a transfer, so speaking needs no bond at all. What
// bond buys is ROOM — throughput, the rate the metabolism governs
// (§3.3) — and the question §4 leaves open is what room an unbonded
// voice has. Until that is ruled, this asserts the faucet exists rather
// than hiding it.
assert_eq!(
    docketed,
    std::array::from_fn(|l| seeded[l] * 3),
    "one genesis allocation per arriving identity — a faucet, not a mint"
);

k.glue_local();
let held = k.treasury().audit().expect("a glue conserves");
let addrs: Vec<Address> = k.dockets().iter().map(|(a, _, _, _, _, _)| a.clone()).collect();
for a in addrs {
    let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
}
let after = k.treasury().audit().expect("a verdict conserves");
assert!(
    after.iter().zip(held.iter()).all(|(x, y)| x >= y),
    "a ruling mints and never unmakes: {held:?} then {after:?}"
);
}
