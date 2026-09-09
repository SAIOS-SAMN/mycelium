// 24.1 — THE APERTURE: what the mind can afford to hold, what it sheds,
// and that the cost of one word does not grow with the book.

use crate::cognition::*;

/// THE TWO-STROKE BREATHES: sustained germination depletes the reserve; a germ
/// on a depleted lane DEFERS (pending, never stranded) while the syntropic
/// strokes regenerate; a later submission dockets the deferred germ once the
/// lane re-earns expansion. The METABOLISM defers it; no count of voices does.
/// A second vantage here is `glue_local`'s two fixture keys (§3.4's
/// wire-identifier exception): this test proves nothing about 47.3's witness.
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
k.court_stroke(super::common::one_stroke_transport);
let hot = (
    k.field_hot_len(),
    k.pending.len(),
    k.heard_len(),
    k.dialect_len(),
    k.provisional.len(),
);
for _ in 0..16 {
    k.court_stroke(super::common::one_stroke_transport);
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

/// 24.1, 26 — THE OPEN-DEMAND ARM. When the lane a residual occupies is
/// depleted, germination DEFERS and the residual is listed on the open
/// market instead: the demand stands, quotable, until the reserve recovers.
/// Each of the three carriers charges for itself — a word of finite order
/// charges its order, an unclosed walk charges the letters it spelled, and a
/// pure residual chain charges the support its chain touches — and all three
/// are LISTED. Only an element with neither word nor chain is degenerate.
///
/// This arm had no test at all: a probe at its head ran zero times across
/// the whole suite while the corrections of F2r1 and F2r1c were written into
/// it (backtrack ledger, 2026-09-07). This test reaches it and takes the
/// PURE RESIDUAL CHAIN carrier — the one F2r1c corrected, and the one that
/// was being refused. The two tape carriers (a word of finite order, an
/// unclosed walk) are still unreached; the fixture that spells one under a
/// depleted lane is owed as F2r1d.
#[test]
fn a_depleted_lane_lists_the_demand_instead_of_germinating_it() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 7)];
    // Empty every lane: eight spends empty a full reserve, exactly.
    for _ in 0..8 {
        k.physics_mut().spend([true; 4]);
    }
    assert!(
        !crate::economy::lanes_short(&k.physics().reserve_shortfall([true; 4])).is_empty(),
        "the reserve is spent on some lane"
    );
    let before = k.field().len();
    let out = k.ingest(&a, &b, [1; 32]);
    assert_eq!(out, Outcome::GermPending, "a deferred demand still stands");
    assert!(
        k.field().len() > before,
        "the residual is LISTED on the open book, not refused: a pure \
         residual chain is the germination demand itself"
    );
}

/// 24.1, 18.2 — THE SENTENCE ARM. A sentence has no single spelling: it is
/// a nerve one level above a word, and its words are its loop holonomies.
/// Under a depleted lane it reaches the open-demand arm with no stored
/// cycle, and the arm charges the ORDERS OF THOSE LOOPS' CLASSES rather
/// than reading `letters()` — a question one level below the carrier — and
/// charging the whole nerve as a bare chain.
///
/// The form is a triangle, so it has a loop; a tree has none and charges
/// nothing, which is correct and is why this fixture is three vertices.
#[test]
fn a_depleted_lane_charges_a_sentence_by_its_loop_words() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    for tag in [2u8, 3u8, 4u8] {
        f.vertices.push(Vertex::Word {
            address: Address::from_bytes([tag; 32]),
            word: Word::identity(),
            carriage: Box::new(element::carriage(&one, &(zero.clone(), zero.clone()))),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
    }
    for (i, j) in [(0u16, 1u16), (1, 2), (0, 2)] {
        f.faces.push(FormFace { i, j, transport: f.face_of(i, j).expect("face") });
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    assert!(!f.loops.is_empty(), "a triangle closes a loop");
    assert!(!f.loop_words().is_empty(), "and the loop carries a holonomy word");
    let spoken = as_element_form(&f, [1, 1, 0, 0]).expect("a sentence tape");
    assert!(spoken.letters().is_none(), "a sentence is not a word: it has no letters");
    let decoded = spoken.form().expect("and it decodes to the nerve its words live in");
    let steps: u64 = decoded
        .loop_words()
        .iter()
        .map(|(w, _)| match w.order() {
            crate::transducer::alphabet::Order::Finite(p) => p as u64,
            crate::transducer::alphabet::Order::Infinite => w.len() as u64,
        })
        .sum();
    assert!(steps > 0, "the loops name a charge");

    let mut k = Kernel::new();
    // Admission is room (23.8, S9.6): a first-time voice is enrolled by
    // `reflex_open`, which this fixture bypasses to reach `commit_germ`
    // directly, so it enrols the voice itself; an unbonded voice has no
    // room and its act is refused before anything is charged.
    k.ensure_bonded([1; 32]);
    for _ in 0..8 {
        k.physics_mut().spend([true; 4]);
    }
    assert!(
        !crate::economy::lanes_short(&k.physics().reserve_shortfall([true; 4])).is_empty(),
        "the reserve is spent on some lane"
    );
    let before = k.physics().pressure;
    let language = k.language();
    let (form, qs) = reduce_germ(&language, (0, 0), &spoken);
    assert!(
        matches!(form, NormalForm::Irreducible(_)),
        "a sentence the language does not hold is irreducible, tape and all"
    );
    let out = k.commit_germ(
        Reduction {
            germ: spoken.clone(),
            form,
            qs,
            language,
            listing: spoken.address(),
            frontier: true,
        },
        [1; 32],
    );
    assert_eq!(out, Outcome::GermPending, "a deferred sentence still stands");
    let after = k.physics().pressure;
    assert!(
        after[0] > before[0],
        "the sentence is charged by its loops' orders, not by a chain it does not have: \
         {before:?} then {after:?}"
    );
}
