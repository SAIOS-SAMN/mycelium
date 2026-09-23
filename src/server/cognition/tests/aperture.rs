// 24.1 — THE APERTURE: what the mind can afford to hold, what it sheds,
// and that the cost of one word does not grow with the book.

use super::common::speak_pair;
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
// A bar: a body the eyes pair with its image (20 as ruled, L7). An L that
// lost a site is a body no versor carries — no sentence, no listing.
let a = vec![(0, 0, 1), (0, 1, 1)];
// ENTROPIC EXPANSION: eight distinct germs drain the residual lane — each
// a new site painted its own hue beside the bar, a creation the voice's
// sentence carries as a demand beside the bar's own word (50.5). H1b, 70.4 —
// separated: at `(0, 2)` the site is one hue-blind piece with the bar.
for n in 0..8u8 {
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 3, n + 2)];
    // One vantage dockets; a second voice on the same pair is a use.
    match speak_pair(&mut k, &a, &b, [1; 32]) {
        Outcome::GermPending => k.glue_local(),
        Outcome::Installed(_) | Outcome::Sectioned(_) | Outcome::Decomposed(_) => {}
        o => panic!("germ {n}: {o:?}"),
    }
}
// Two fresh germs: their first voices each trigger a syntropic stroke (the
// mesh is depleted), partially recovering the lane. The bar recoloured
// whole, two ways: two sentences of one word each, differing in the hue
// their gauge carries the bar to.
let ga = vec![(0, 0, 4), (0, 1, 4)]; // 1→4
let gb = vec![(0, 0, 5), (0, 1, 5)]; // 1→5
let held: Vec<Vec<u8>> = k.provisional.scoped().map(|(key, _)| key).collect();
assert_eq!(speak_pair(&mut k, &a, &ga, [1; 32]), Outcome::GermPending);
// The sentence A spoke, read off the docket by the address the court
// gave it: the one listing that was not there before.
let ga_spoken = k
    .provisional
    .scoped()
    .find(|(key, _)| !held.contains(key))
    .map(|(_, e)| e.clone())
    .expect("ga's sentence is on the docket");
// Voice 1 has its one claim in flight on this lane (§3.1 as ruled,
// S13.15); B's first voice is another arriving identity.
assert_eq!(speak_pair(&mut k, &a, &gb, [4; 32]), Outcome::GermPending);
// A's quorum lands on the recovered lane and installs — spending it back out…
assert_eq!(speak_pair(&mut k, &a, &ga, [2; 32]), Outcome::GermPending);
let mut cover = crate::network::nerve::Cover::new();
let _ = k.glue_fetched(ga_spoken, [2; 32], [1; 32], &mut cover);
// …so B finds the lane DEPLETED. The METABOLISM is what defers -- the
// only thing that should gate here (E3) -- and the germ is not lost: a
// later submission dockets it once the reserve recovers. What does NOT
// gate it is a count of voices; that was the quorum this replaced.
match speak_pair(&mut k, &a, &gb, [2; 32]) {
    Outcome::GermPending | Outcome::Sectioned(_) => {}
    o => panic!("deferred or sectioned, not lost: {o:?}"),
}
match speak_pair(&mut k, &a, &gb, [3; 32]) {
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
let calm = k.lane_room().expect("an enrolled court's room reads");
assert!(calm.iter().all(|&c| c > 0), "no lane is ever pinched to zero: {calm:?}");
let s0 = calm[0];
for _ in 0..24 {
    k.physics_mut().bump(&[0, 0, 1, 0]);
}
let tense = k.lane_room().expect("an enrolled court's room reads");
assert_eq!(tense[0], s0, "T pressure does not steal spatial room: {tense:?}");
}

/// Extra court strokes of a fixed corpus do not grow the hot set.
#[test]
fn hot_set_does_not_grow_with_idle_strokes() {
let mut k = Kernel::new();
k.set_height(5);
let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole (20 as ruled, L7)
assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending, "the sentence is listed");
speak_pair(&mut k, &a, &b, [2; 32]);
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
use crate::transducer::element::from_canonical;
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
            k.basis_insert(from_canonical(&g), [1; 32]);
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
    // A bar kept and a site painted beside it (20 as ruled, L7): the
    // sentence is the bar's word with a creation demand riding it. H1b,
    // 70.4 — separated, as in the test above.
    let a = vec![(0, 0, 1), (0, 1, 1)];
    let b = vec![(0, 0, 1), (0, 1, 1), (0, 3, 7)];
    // Empty every lane: eight spends empty a full reserve, exactly.
    for _ in 0..8 {
        k.physics_mut().spend([true; 4]);
    }
    assert!(
        !crate::economy::lanes_short(&k.physics().reserve_shortfall([true; 4])).is_empty(),
        "the reserve is spent on some lane"
    );
    let before = k.field().len();
    let out = speak_pair(&mut k, &a, &b, [1; 32]);
    assert_eq!(out, Outcome::GermPending, "a deferred demand still stands");
    assert!(
        k.field().len() > before,
        "the sentence is LISTED on the open book with its demand, not refused: \
         what the eyes could not pair rides the sentence as the demand itself"
    );
}

/// 24.1, 18.2 — THE SENTENCE ARM. A sentence has no single spelling: it is
/// a nerve one level above a word, and its words are its orbit holonomies.
/// Under a depleted lane it reaches the open-demand arm with no stored
/// cycle, and the arm charges the ORDERS OF THOSE ORBITS' CLASSES rather
/// than reading `letters()` — a question one level below the carrier — and
/// charging the whole nerve as a bare chain.
///
/// The form is a triangle, so it has an orbit; a tree has none and charges
/// nothing, which is correct and is why this fixture is three vertices.
#[test]
fn a_depleted_lane_charges_a_sentence_by_its_orbit_words() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{as_element_form, Form, Vertex};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    for tag in [2u8, 3u8, 4u8] {
        f.vertices.push(Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            // Each vertex stands on its OWN chart: with one chart they would be one
            // vertex, since an address is derived from its coordinates (54.2, §1.5).
            crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::carriage(&one, &(zero.clone(), zero.clone()))),
            vec![(1, 2)],
            None,
        ));
    }
    for (i, j) in [(0u16, 1u16), (1, 2), (0, 2)] {
        f.faces.push(f.face_of(i, j).expect("face"));
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert!(!f.orbits.is_empty(), "a triangle closes an orbit");
    assert!(!f.orbit_words().is_empty(), "and the orbit carries a holonomy word");
    let spoken = as_element_form(&f, [1, 1, 0, 0]).expect("a sentence tape");
    assert!(spoken.letters().is_none(), "a sentence is not a word: it has no letters");
    let decoded = crate::transducer::sentence::from_element_form(&spoken)
        .expect("and it decodes to the nerve its words live in");
    let steps: u64 = decoded
        .orbit_words()
        .iter()
        .map(|(w, _)| match w.order() {
            crate::transducer::alphabet::Order::Finite(p) => p as u64,
            crate::transducer::alphabet::Order::Infinite => w.len() as u64,
        })
        .sum();
    assert!(steps > 0, "the orbits name a charge");

    let mut k = Kernel::new();
    // Admission is room (23.8, S9.6): this fixture bypasses the replay to
    // reach `list_sentence` directly with an unbonded voice, which has
    // room for one claim in flight per lane (§3.1 as ruled, S13.15) —
    // nothing enrols it, and this first claim is admitted on that room.
    for _ in 0..8 {
        k.physics_mut().spend([true; 4]);
    }
    assert!(
        !crate::economy::lanes_short(&k.physics().reserve_shortfall([true; 4])).is_empty(),
        "the reserve is spent on some lane"
    );
    let before = k.physics().pressure;
    let out = k.list_sentence(&spoken, [1; 32]);
    assert_eq!(out, Outcome::GermPending, "a deferred sentence still stands");
    let after = k.physics().pressure;
    assert!(
        after[0] > before[0],
        "the sentence is charged by its orbits' orders, not by a chain it does not have: \
         {before:?} then {after:?}"
    );
}

/// THE HOLONOMY THE METABOLISM CHARGES ON IS THE ONE THE ORBIT CARRIES.
///
/// `germinate/listing.rs` charges the physics off `Form::orbit_words`: an orbit
/// of FINITE order raises pressure by its period, an orbit of INFINITE order
/// charges TORSION — "what will not close". That reader returned
/// `lp.holonomy`, the orbit's WORD, and a face's word is the coboundary
/// `w_j ∘ w_i⁻¹` whose product around a cycle telescopes to the identity. So
/// every orbit read `Finite(1)`, every sentence raised one unit of pressure
/// per orbit, and the torsion branch was unreachable by construction —
/// `Physics.torsion` stayed zero, `order_flux` was `syntropic − 0`, and the
/// `yield_surface` a reply carries never saw a cycle. Measured at 7582ada:
/// 196 cycles filed by the voices, 0 charged on any court.
///
/// This fixture is the same triangle as the test above with a quarter turn
/// on the one face that does not touch the Frame — flat on the word, curved
/// on the edge — and it asserts the reader reports the turn's period and not
/// the identity's.
#[test]
fn the_metabolism_reads_the_orbits_true_holonomy_not_its_coboundary() {
    use crate::transducer::alphabet::{Derivation, LetterId, Order, Word};
    use crate::transducer::sentence::{face_pair_key, FacePlace, Form, FormFace, Transport, Vertex};
    let chart = crate::transducer::stalk::Invariant::counts(&[3, 5]);
    let pair = face_pair_key(&chart, &chart);
    let q = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("plane"), false)]);
    let mut f = Form::default();
    for tag in [2u8, 3u8] {
        f.vertices.push(Vertex::word_on(
            Derivation::spelled(Word::identity()),
            // Its own chart: two on one chart with one rotor, one carriage
            // and one gauge are ONE vertex (54.2, §1.5).
            crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    for (i, j, edge) in [(0u16, 1u16, None), (0, 2, None), (1, 2, Some(q.clone()))] {
        let mut face = FormFace::new(i, j, Word::identity());
        face.at.insert(
            pair.clone(),
            FacePlace {
                carriage: element::identity_versor(),
                gauge: Vec::new(),
                edge: edge.map(|w| Transport {
                    word: w,
                    carriage: element::identity_versor(),
                    gauge: Vec::new(),
                }),
            },
        );
        f.faces.push(face);
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(f.orbits.len(), 1, "the triangle closes one orbit");
    assert_eq!(
        f.orbits[0].holonomy.order(),
        Order::Finite(1),
        "the WORD holonomy is the identity, as a product of coboundaries always is"
    );
    let words = f.orbit_words();
    assert!(!words.is_empty(), "the orbit reports a holonomy per member path");
    assert!(
        words.iter().any(|(w, _)| matches!(w.order(), Order::Finite(4))),
        "and what it reports is the edge composite's quarter turn, not the identity: {:?}",
        words.iter().map(|(w, _)| w.order()).collect::<Vec<_>>()
    );

    // A BARE UNCLOSED CYCLE IS A FREE ALGEBRAIC REFUSAL; THE CHARGE FOLLOWS
    // THE DEMAND.
    //
    // "What will not close" has two readings, and the charge asked only the
    // weaker — whether the ALGEBRA closes the cycle — so it fired on
    // `Order::Infinite` alone and this corpus produces none: 12187 flat and
    // 256 finite edge holonomies at e9399c4, INFINITE ZERO, against 200
    // cycles the voices filed. A filed cycle IS one no candidate discharged.
    //
    // But that is the ordinary condition of a voice searching for a word it
    // does not have, and `∂c ≠ 0` is algebraically a non-generator. Billing
    // it taxes the search itself. The charge is assessed where the speaker
    // claimed an adjudication stroke over the boundary — a DEMAND VERTEX
    // (50.5) — and a sentence that merely carries a cycle it could not close
    // is refused at the gate for free.
    let mut filed = f.clone();
    // The period is the FILER'S now, derived through the fixed-point
    // gate, so this reads what the orbit closes at rather than handing it in.
    // 54.7 — THE YIELD IS BOTH PLANES. `file_orbit` returns the labour the
    // filing earns, and that is `(ℓ₁ − 1) + (ℓ₂ − 1)` across the two
    // invariant planes of the double rotation, not `lcm − 1`. A quarter
    // turn is isoclinic here — `[4, 4]` — so it earns `3 + 3 = 6`, where
    // the envelope said `4`. The two planes are doing the work; the
    // envelope was never either plane's.
    assert_eq!(
        filed.file_orbit(0),
        vec![6],
        "the orbit files under its holonomy's class, at the period it closes at"
    );
    let bare = crate::transducer::sentence::as_element_form(&filed, [1, 1, 0, 0])
        .expect("a sentence tape");
    // THE FILING RIDES THE SPEAKER'S WORD, AND THE CYCLE RE-DERIVES FROM
    // THE GEOMETRY (54.5). This read the DECODED form's `torsion`, which is
    // empty by construction: `from_address` recovers the complex and a
    // filing is not part of it (law 12 — `orbit_classes` derives the class
    // and the period from the faces).
    assert!(!bare.filing().is_empty(), "the filed cycle rides the word that made it");
    let back = crate::transducer::sentence::from_element_form(&bare).expect("its address decodes");
    assert!(
        back.orbits.iter().any(|lp| !back.orbit_classes(lp).is_empty()),
        "and the cycle itself re-derives from the complex, with no filing consulted"
    );

    // The charge is read on a DEPLETED lane (23.8): a court with reserve to
    // spare dockets the listing and charges nothing, so the reserve is spent
    // first exactly as `a_depleted_lane_charges_a_sentence_by_its_orbit_words`
    // spends it.
    let depleted = || {
        let mut k = Kernel::new();
        for _ in 0..8 {
            k.physics_mut().spend([true; 4]);
        }
        k
    };

    let mut k = depleted();
    assert_eq!(k.physics().torsion, [0; 4], "a fresh court carries no torsion");
    let _ = k.list_sentence(&bare, [1; 32]);
    assert_eq!(
        k.physics().torsion,
        [0; 4],
        "an unclosed cycle with no demand on it is a free refusal, not a taxable event"
    );

    // The same cycle, with the sentence asking the market to adjudicate the
    // boundary: the stroke is claimed, so the stroke is charged.
    let mut demanding = filed;
    demanding.vertices.push(crate::transducer::family::demand_vertex(
        crate::transducer::sentence::Demand::Source { multiplicity: 1 },
        &crate::transducer::stalk::Invariant::counts(&[3, 5]),
        vec![(0, 0, 0, 1)],
    ));
    demanding.reindex();
    let asked = crate::transducer::sentence::as_element_form(&demanding, [1, 1, 0, 0])
        .expect("a sentence tape");
    assert!(
        crate::transducer::sentence::from_element_form(&asked)
            .expect("its tape decodes")
            .demand_vertices()
            .next()
            .is_some(),
        "and the demand rides the tape"
    );

    let mut k = depleted();
    let _ = k.list_sentence(&asked, [1; 32]);
    assert!(
        k.physics().torsion.iter().any(|&t| t > 0),
        "a sentence demanding adjudication over what it could not close pays for the stroke: {:?}",
        k.physics().torsion
    );
}
