// 24.1 — THE APERTURE: what the mind can afford to hold, what it sheds,
// and that the cost of one word does not grow with the book.

use crate::cognition::*;
use crate::cognition::reflex::action_potential;

/// THE TWO-STROKE BREATHES: sustained germination depletes the reserve; a germ
/// on a depleted lane DEFERS (pending, never stranded) while the syntropic
/// strokes regenerate; a later submission dockets the deferred germ once the
/// lane re-earns expansion. The METABOLISM defers it; no count of voices does.
#[test]
fn a_depleted_lane_defers_germination_until_the_reserve_recovers() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
// ENTROPIC EXPANSION: eight distinct residual germs drain the residual lane.
for n in 0..8u8 {
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, n + 2)];
    // One vantage dockets; a second voice on the same pair is a use.
    match k.ingest(&a, &b, [1; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
        o => panic!("germ {n}: {o:?}"),
    }
}
// Two fresh germs: their first voices each trigger a syntropic stroke (the
// mesh is depleted), partially recovering the lane.
// Distinct single-transition residuals (unique hue pairs, so their tallies
// are distinct — see the Step-2 note on residual-substance geometry).
let ga = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
let gb = vec![(0, 0, 1), (0, 1, 5), (1, 0, 1)]; // (0,1): 1→5
assert_eq!(k.ingest(&a, &ga, [1; 32]), Outcome::GermPending);
assert_eq!(k.ingest(&a, &gb, [1; 32]), Outcome::GermPending);
// A's quorum lands on the recovered lane and installs — spending it back out…
assert_eq!(k.ingest(&a, &ga, [2; 32]), Outcome::GermPending);
let ga_spoken = k
    .provisional
    .fiber(crate::network::nerve::ROOT)
    .map(|(_, e)| e.clone())
    .find(|e| e.sub().iter().any(|c| c.2 == 1 && c.3 == 4))
    .expect("ga's residual is on the docket");
let mut cover = crate::network::nerve::Cover::new();
let _ = k.glue_fetched(ga_spoken, [2; 32], [1; 32], &mut cover);
// …so B finds the lane DEPLETED. The METABOLISM is what defers -- the
// only thing that should gate here (E3) -- and the germ is not lost: a
// later submission dockets it once the reserve recovers. What does NOT
// gate it is a count of voices; that was the quorum this replaced.
match k.ingest(&a, &gb, [2; 32]) {
    Outcome::GermPending | Outcome::Sectioned(_) => {}
    o => panic!("deferred or sectioned, not lost: {o:?}"),
}
match k.ingest(&a, &gb, [3; 32]) {
    Outcome::GermPending => k.glue_local(),
    Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
    o => panic!("deferral complete: {o:?}"),
}
}

/// Room is per-lane from bond and reserve; ticks on T do not steal
/// S, G, or H. Eviction is by ACTION POTENTIAL, never FIFO.
#[test]
fn market_capacity_is_thermodynamic_and_eviction_is_by_tension() {
let mut k = Kernel::new();
k.bind_identity([1; 32]);
k.enroll([1; 32], [8, 8, 1, 8]);
let calm = k.lane_room();
assert!(calm.iter().all(|&c| c > 0), "no lane is ever pinched to zero: {calm:?}");
let s0 = calm[0];
for _ in 0..24 {
    k.physics_mut().bump(&[0, 0, 1, 0]);
}
let tense = k.lane_room();
assert_eq!(tense[0], s0, "T pressure does not steal spatial room: {tense:?}");
// Action potential: a VALUABLE stale listing outranks a worthless
// fresh one — eviction is argmin tension, not argmax age (anti-FIFO).
let rich_old = action_potential(&[0, 0, 900, 0], 64);
let poor_new = action_potential(&[0, 0, 3, 0], 0);
assert!(rich_old > poor_new, "value dominates age: {rich_old} vs {poor_new}");
// And dwell genuinely decays tension at equal value.
let fresh = action_potential(&[0, 0, 40, 0], 0);
let stale = action_potential(&[0, 0, 40, 0], 128);
assert!(fresh > stale, "dwell cools a listing: {fresh} vs {stale}");
}

/// Extra court strokes of a fixed corpus do not grow the hot set.
#[test]
fn hot_set_does_not_grow_with_idle_strokes() {
let mut k = Kernel::new();
k.set_height(5);
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
k.ingest(&a, &b, [1; 32]);
k.ingest(&a, &b, [2; 32]);
k.court_stroke();
let hot = (
    k.field_hot_len(),
    k.pending.len(),
    k.heard_len(),
    k.dialect_len(),
    k.provisional.len(),
);
for _ in 0..16 {
    k.court_stroke();
}
assert_eq!(
    (
        k.field_hot_len(),
        k.pending.len(),
        k.heard_len(),
        k.dialect_len(),
        k.provisional.len(),
    ),
    hot,
    "idle strokes do not grow the hot set"
);
}

/// COST PER OBSERVATION. The test that tells the two architectures apart:
/// a base that is MODIFIED absorbs one more word at a cost independent of
/// how much it already holds; a base that is APPENDED TO pays for its own
/// size on every observation, and grows once per input forever.
///
/// Admitting a word rebuilt the whole language snapshot — the word vector
/// copied, the index trie deep-cloned, N fresh `OnceLock`s allocated and
/// every lifted shape thrown away — so the Nth admit cost O(N) and the
/// fleet's footprint grew quadratically in what it had learned. This is the
/// N vs 4N counter of the enumeration audit, applied to a write.
#[test]
fn admitting_a_word_costs_the_same_at_n_and_4n() {
use crate::network::nerve::work;
use crate::transducer::element::from_residual;
use crate::transducer::germinate::germinate;
let admit_after = |n: usize| -> u64 {
    let mut k = Kernel::new();
    // Distinct SUPPORTS, not distinct positions: A5 centres a chain to
    // its min corner, so words that differ only by placement are one
    // word (1.5) and the base would not grow at all.
    let fill = |i: usize, k: &mut Kernel| {
        let a: Vec<(i64, i64, u8)> = (0..=i as i64).map(|c| (0, c, 1u8)).collect();
        let b: Vec<(i64, i64, u8)> = (0..=i as i64).map(|c| (0, c, 2u8)).collect();
        if let Some(g) = germinate(&a, &b) {
            k.basis_insert(from_residual(&g), [1; 32]);
        }
    };
    for i in 0..n {
        fill(i, &mut k);
    }
    let held = k.basis_len();
    work::reset();
    fill(n + 1, &mut k);
    let cost = work::total();
    assert!(k.basis_len() > held, "the word was admitted at n={n}");
    cost
};
let at_n = admit_after(8);
let at_4n = admit_after(32);
assert!(at_n > 0, "the admit is measured");
assert_eq!(
    at_n, at_4n,
    "one observation costs the same into a large base as a small one: \
     {at_n} at N, {at_4n} at 4N"
);
}
