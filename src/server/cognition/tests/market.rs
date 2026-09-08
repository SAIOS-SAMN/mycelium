// 26 — THE MARKET: the five-act conversation, the docket and its room, the
// door's strain, the probe, and the descent that opens a face.

use super::common::*;
use crate::cognition::*;
use crate::transducer::element;
use crate::transducer::germinate::germinate;

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
assert_eq!(short.ingest(&a, &b, [1; 32]), Outcome::GermPending, "the first draw covers");
assert_eq!(short.pending.len(), 1, "the first pair is listed on one vantage");
let occupancy = short.occupancy();
let provisional = short.provisional.len();
let entries = short.treasury().entries().len();
let k = short.k_step();
// The G lane is capped at 1 and the second pair's draw does not cover:
// the act is refused with the reason, and the book, the room, the
// escrow and the receipt chain are exactly as they were.
assert_eq!(
    short.ingest(&c, &d, [1; 32]),
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
    .expect("one 1-cell pair measures");
bonded.bind_identity([9; 32]);
bonded.set_genesis_cover(cover);
bonded.enroll([1; 32], cover);
bonded.enroll([2; 32], cover);
let chain_ab = germinate(&a, &b).unwrap();
let chain_cd = germinate(&c, &d).unwrap();
assert_eq!(bonded.attest(&chain_ab, [1; 32]), Outcome::GermPending);
assert_eq!(bonded.pending.len(), 1, "the cover admits one pair");
assert_eq!(
    bonded.attest(&chain_cd, [1; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "a second pair of the same μ⃗ is refused until the first draw releases"
);
assert_eq!(bonded.pending.len(), 1, "and it is not listed");
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
k.court_stroke(one_stroke_transport);
assert!(k.pending.is_empty(), "the listing lapsed");
assert_eq!(k.occupancy(), [0; 4], "the draw released");
assert!(
    !k.treasury().entries().iter().any(|e| e.kind == crate::economy::Kind::Slash),
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
let Demand { voice: v, a: ra, b: rb } =
    decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
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
    ha.append_step([1; 32], [2; 32]);
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
