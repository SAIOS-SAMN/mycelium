// 26 — THE MARKET: the five-act conversation, the docket and its room, the
// door's strain, the probe, and the descent that opens a face.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element;
use crate::transducer::germinate::germinate;

/// STEP 3's ECONOMIC CONVERSATION CLOSES: A speaks a novel SENTENCE and the
/// court lists it with its demands (20 as ruled, 50.5); the listing is
/// quoted as work; A speaks it again, a use of the address that draws
/// nothing new; B independently speaks the same sentence; consilience
/// installs, the market CLEARS. {speech → quote → speech → settle}. A
/// second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's
/// witness.
#[test]
fn the_five_act_conversation_closes_with_settlement() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending, "voice A speaks the sentence");
assert_eq!(k.quote().1.len(), 1, "the sentence is quoted when the frontier is dry");
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending, "A again: a use of the listing, nothing drawn");
let (surface, quoted) = k.quote();
assert_eq!(quoted.len(), 1, "the open demand is quoted as work, once");
assert!(surface.iter().any(|&s| s >= 1), "the open demand tensions the lanes it occupies");
let (work, mu) = &quoted[0];
assert!(
    (0..4).any(|l| work.magnitude()[l] > 0 && mu[l] > 0),
    "price is nonzero on a lane the word moves"
);
let out = speak_pair(&mut k, &a, &b, [2; 32]);
assert_eq!(out, Outcome::GermPending, "consilience settles: {out:?}");
k.glue_local();
assert_eq!(k.voices_of(&k.basis()[0].address()).len(), 2, "the settle parties");
assert!(k.quote().1.is_empty(), "the market cleared");
}

/// A docket draws the chain's μ⃗ from the uttering voice's remaining bond;
/// a short lane refuses the act with REFUSE_ROOM, and nothing is listed,
/// drawn, escrowed or sealed (23.8, S9.6). A second voice's bond is its
/// own; the hypha's identity is not the drawer.
#[test]
fn a_short_lane_refuses_the_act_and_dockets_nothing() {
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
let c = vec![(1, 0, 3u8)];
let d = vec![(1, 0, 4u8)];
let mut short = Kernel::new();
short.set_lane_cap([8, 1, 8, 8]);
assert_eq!(speak_pair(&mut short, &a, &b, [1; 32]), Outcome::GermPending, "the first draw covers");
assert_eq!(short.pending.len(), 1, "the first pair is listed on one vantage");
let occupancy = short.occupancy();
let provisional = short.provisional.len();
let entries = short.treasury().entries().len();
let k = short.k_step();
// The G lane is capped at 1 and the second pair's draw does not cover:
// the act is refused with the reason, and the book, the room, the
// escrow and the receipt chain are exactly as they were.
assert_eq!(
    speak_pair(&mut short, &c, &d, [1; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "a lane that does not cover the draw refuses the act"
);
assert_eq!(short.pending.len(), 1, "nothing further docketed");
assert_eq!(short.occupancy(), occupancy, "nothing drawn");
assert_eq!(short.provisional.len(), provisional, "nothing provisional");
assert_eq!(short.treasury().entries().len(), entries, "nothing escrowed");
assert_eq!(short.k_step(), k, "no receipt sealed for a refused act");

let mut bonded = Kernel::new();
let cover = crate::arc::family_cover(&[(vec![vec![1]], vec![vec![2]])])
    .expect("a one-site pair measures");
bonded.bind_identity([9; 32]);
bonded.set_genesis_cover(cover);
bonded.enroll([1; 32], cover);
bonded.enroll([2; 32], cover);
assert_eq!(speak_pair(&mut bonded, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(bonded.pending.len(), 1, "the cover admits one pair");
assert_eq!(
    speak_pair(&mut bonded, &c, &d, [1; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "a second pair of the same μ⃗ is refused until the first draw releases"
);
assert_eq!(bonded.pending.len(), 1, "and it is not listed");
assert_eq!(speak_pair(&mut bonded, &c, &d, [2; 32]), Outcome::GermPending);
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
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(k.pending.len(), 1, "listed");
let occ = k.occupancy();
assert!(occ.iter().any(|&x| x > 0), "the draw occupies");
let lambda = crate::physics::aperture_cycle() as u64;
k.set_height(k.height().saturating_add(lambda));
k.court_stroke(one_stroke_transport);
assert!(k.pending.is_empty(), "the listing lapsed");
assert_eq!(k.occupancy(), [0; 4], "the draw released");
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == crate::economy::Kind::Slash),
    "lapse does not slash"
);
}

/// 47.5, Stage 11 I20 — Λ IS PER CHART. The lapse asks the caller with the
/// listing's chart (`invariant_of` of the pending element), the listing's
/// magnitude and this court's room — the three coordinates the Cover's
/// `home` reads (16.11) — and never with the listing's address, which the
/// router could only place by XOR distance. A caller that records what it
/// was asked is asked exactly those coordinates, once for the one pending
/// listing; a caller that has measured no transport lapses nothing.
#[test]
fn the_lapse_asks_lambda_with_the_listing_chart_magnitude_and_the_court_room() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
let listed = k
    .pending
    .scoped()
    .next()
    .map(|(_, (e, _))| e.clone())
    .expect("listed");
let want_chart = crate::transducer::stalk::invariant_of(&listed);
let want_mag = listed.magnitude();
let room = k.lane_room().expect("this court's room reads");
let want_room: [u64; 4] = std::array::from_fn(|l| room[l] as u64);
// INSIDE this court's own horizon, so what the pass proves is WHAT Λ IS
// ASKED WITH and not what an unmeasured answer does with it: past the
// horizon the listing now leaves on the court's own aperture (the
// stroke-duration ruling), which would empty the book before the
// assertions below could read it.
k.set_height(1);
let asked = std::cell::RefCell::new(Vec::new());
k.court_stroke(|chart, mag, room| {
    asked.borrow_mut().push((chart.clone(), *mag, *room));
    crate::physics::Lambda::Unmeasured
});
let asked = asked.into_inner();
assert_eq!(asked.len(), 1, "Λ is asked once, for the one pending listing: {asked:?}");
assert_eq!(asked[0].0, want_chart, "with the listing's chart, never its address");
assert_eq!(asked[0].1, want_mag, "with the listing's magnitude, so the occupied lanes route it");
assert_eq!(asked[0].2, want_room, "and with this court's room, so it is a candidate home");
assert_eq!(k.pending.len(), 1, "and inside the horizon the listing stands");
}

/// D12 — THE ESCROW-PRICED DOOR: a calm mesh keeps a free door; a
/// saturated mesh charges congestion (fee → the pool) and DEFERS a
/// speaker who can pay nothing on a fully exhausted lane — spam costs
/// exactly when strain is real, and paying members pass.
#[test]
fn the_door_charges_strain_and_defers_the_broke() {
let mut k = Kernel::new();
// Calm: the door is free — a sentence is heard untouched.
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 4), (0, 1, 4)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending, "calm door admits");
// The door fee is a BURN (`settle_door`), and a calm mesh writes none.
// This read `e.kind == "fee"`, which named a kind no writer in the tree
// has ever produced, so the assertion held whatever the door did. The
// typed `Kind` is what caught it: there is no `Fee` to compare against.
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == crate::economy::Kind::Burn),
    "no fee on a calm mesh"
);
// Exhaust the metabolism entirely (every lane to zero reserve).
for _ in 0..16 {
    k.physics_mut().spend([true; 4]);
}
// The door's reading is per lane (S9.6b): every lane is spent to
// nothing and a broke speaker pays nothing on any, so the reading names
// all four with the fee that went unpaid, the deficit itself.
let reading = k.door_deferral([9; 32]);
assert_eq!(
    reading,
    k.physics().reserve_deficit(),
    "a broke speaker on an exhausted mesh is deferred on every lane, by the deficit"
);
// A broke speaker (no earned balance) reads as deferred on every lane
// while the reserve stands at nothing. The reflex opening regenerates
// the lanes before it hears (S13.2), so the sentence is heard on the
// regenerated reserve, and the broke speaker paid nothing at the door.
let a2 = vec![(5, 0, 2), (5, 1, 2)];
let b2 = vec![(5, 0, 6), (5, 1, 6)];
let broke = speak_pair(&mut k, &a2, &b2, [9; 32]);
assert_eq!(broke, Outcome::GermPending, "heard on the regenerated lane: {broke:?}");
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == crate::economy::Kind::Burn && e.who == [9; 32]),
    "the broke speaker paid nothing at the door"
);
// A FUNDED speaker pays the congestion fee and passes the door.
k.treasury_mut().settle_subsidy([7; 32], [8, 8, 8, 8], (0, [0; 32]));
for _ in 0..16 {
    k.physics_mut().spend([true; 4]); // re-exhaust (the deferral relaxed some)
}
let _ = speak_pair(&mut k, &a2, &b2, [7; 32]);
// The fee is a BURN: §2 has no pool carrier for it to land in, so μ⃗
// that leaves the speaker's balance leaves the supply with it. This
// asserted a `transfer`, which is a payment to a payee, and there was
// none: the debit half was written and the credit half never existed.
assert!(
    k.treasury().entries().iter().any(|e| e.kind == crate::economy::Kind::Burn && e.who == [7; 32]),
    "the funded speaker paid the door out of balance"
);
assert!(
    k.treasury().audit().is_ok(),
    "and the ledger still conserves after it: {:?}",
    k.treasury().audit()
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
// A coboundary listed on an empty language: a chain listing under a
// voice's draw — the shape the corpus intake lists (23.8) and a probe
// settles against (26). No voice speaks a chain (law 8); the fixture
// lists it as the book does.
let c = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 1), (0, 3, 3)];
let d = vec![(0, 0, 2u8), (0, 1, 2), (0, 2, 1), (0, 3, 4)];
let chain = germinate(&c, &d).unwrap();
let k_now = k.k_step();
k.book_list(element::from_canonical(&chain), k_now, [1; 32]);
let target = k.quote().1[0].0.address();
let chain = k.quote().1[0].0.sub().to_vec();
assert_eq!(chain.len(), 3);
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
k.basis_insert(element::from_canonical(&germinate(&a, &b).unwrap()), [2; 32]);
let _word = k.basis()[0].address();
// The entity's reduction over the language: the word cited at the
// origin, the far link left as the residual.
let language = k.language();
let sentence = crate::transducer::sentence::deconstruct_at(&|i| language.get(i), &language.stalks, &|a| language.index.get(a).copied(), &|i| language.shape(i), &chain, None);
let (void, cites) = (sentence.residual.clone(), sentence.cites.clone());
assert_eq!(void, vec![(0, 3, 3, 4)]);
assert_eq!(cites.len(), 1);
let basis = k.basis();
assert_eq!(
    replay_at(&|i| basis.get(i).map(|w| w.element()), &cites, &chain),
    void,
    "the replay reproduces the residual"
);
let (code, paid, remaining) = k.settle_probe([3; 32], &target, void.clone(), &[], false);
assert_eq!(code, PROBE_DESCENT);
assert_eq!(paid, [0; 4], "a descent is measured, never paid: covering links opens no axis");
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

/// Descent records a remainder FACE, stagnation counts only unmoved
/// occupied lanes, and a full aperture of failed probes sheds the listing
/// as a cavity.
///
/// THE LISTING'S MAGNITUDE STAYS A FUNCTION OF ITS CHAIN. A holonomic
/// demand written as the remainder's link count would be a tally of links
/// in the lane that carries boundary topology and the rotor's own
/// coordinates (16.16, 42.6), a second carrier for a fact the field
/// already holds (law 12), and a magnitude `blind_replay` refuses: it
/// re-derives a residual germ's lanes from its own chain and refuses a
/// word claiming more than the chain supports, so the court would slash
/// its own listing on the next circuit and every listing a voice made
/// progress on would be destroyed by the court's own write. The assertion
/// here is that property: after a descent the listing still re-derives
/// from its chain.
#[test]
fn descent_opens_a_face_and_drain_sheds_the_unmoved() {
use crate::anastome::dialogue::PROBE_DESCENT;
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8)];
let chain = germinate(&a, &b).unwrap();
assert!(k.intake(&chain, &frame_of(&a, &b)), "the corpus intake lists the body");
let target = k.quote().1[0].0.address();
let listed = k.quote().1[0].0.sub().to_vec();
let rem = vec![(0, 1, 1u8, 0u8)];
let before = match k.tension.get(&target) {
    Some(t) => *t,
    None => k.quote().1[0].0.magnitude(),
};
let after = crate::transducer::element::measure_lanes(&rem);
assert_ne!(after, before, "the residual is a proper descent");
let (code, _, remaining) = k.settle_probe([3; 32], &target, rem.clone(), &[], false);
assert_eq!(code, PROBE_DESCENT);
assert_eq!(remaining, after);
let listing = k.quote().1[0].0.clone();
assert_eq!(listing.address(), target, "the listing stays the body");
// THE LISTING'S MEASURE STILL RE-DERIVES FROM ITS OWN CHAIN, which is
// what the court checks on every circuit. A descent is recorded on the
// FIELD, read below as the remainder face and the remainder chart; it
// does not rewrite the word.
assert_eq!(
    listing.base_magnitude(),
    crate::transducer::element::measure_lanes(&listed),
    "a descent leaves the listing's measure a function of its chain"
);
assert_eq!(
    crate::court::verdict::blind_replay(&listing),
    None,
    "so the court's own next circuit does not slash it"
);
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
// The cavity is the listing itself, so its measure is still its chain's.
assert_eq!(
    k.cavities()[0].base_magnitude(),
    crate::transducer::element::measure_lanes(&listed),
    "relocating a listing does not rewrite what it measured"
);
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

/// Two hyphas at different K relocate the same standing listing at the
/// same TUniversal: T-room, never a stroke count.
#[test]
fn t_room_relocates_at_tuniversal_independent_of_k() {
// The L recoloured whole to `hue` (20 as ruled, L7). Two hues are two
// sentences — the sentence is the rule, and a body's shape is quotiented
// from its key (1.5), so the same recolour of another shape would be the
// same listing and draw nothing new.
let list = |k: &mut Kernel, hue: u8| -> Address {
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, hue), (0, 1, hue), (1, 0, hue)];
    let _ = speak_pair(k, &a, &b, [1; 32]);
    k.pending.scoped().next().expect("listed").1.0.address()
};
let mut ha = Kernel::new();
let mut hb = Kernel::new();
ha.set_lane_cap([64, 64, 1, 64]);
hb.set_lane_cap([64, 64, 1, 64]);
ha.set_height(10);
hb.set_height(10);
let first_a = list(&mut ha, 4);
let first_b = list(&mut hb, 4);
assert_eq!(first_a, first_b);
// Advance K on ha only — TUniversal stays the clock we set.
for _ in 0..8 {
    ha.append_step(crate::chronometry::ActKind::List, [1; 32], [2; 32]);
}
assert!(ha.k_step() > hb.k_step());
ha.set_height(40);
hb.set_height(40);
let _ = list(&mut ha, 5);
let _ = list(&mut hb, 5);
assert!(
    ha.cavity_book.contains(&first_a),
    "ha relocates the older listing on T"
);
assert!(
    hb.cavity_book.contains(&first_b),
    "hb relocates the same listing at the same TUniversal"
);
}

/// 23.8, 45.2, Stage 12 S12.4a (I39) — THE QUOTE REPLY CARRIES THE LEDGER
/// HEIGHT. The book's height is the court's chain head, never the count of
/// listings written: the height moves with the ledger while the book
/// generation stands, and the quote's market view carries that height.
#[test]
fn the_quote_reply_height_is_the_chain_head_height_not_the_book_generation() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
assert_eq!(k.pending.len(), 1, "one listing written");
assert_eq!(k.book_height(), k.height(), "the book's height is the chain's head height");
let before = k.height();
k.set_height(before + 40);
assert_eq!(
    k.book_height(),
    before + 40,
    "the height moves with the ledger and not with the listings written"
);
assert_eq!(k.pending.len(), 1, "the book generation did not move it");
assert_eq!(
    k.market_open().head_height,
    before + 40,
    "the quote's market view carries the same height"
);
}
