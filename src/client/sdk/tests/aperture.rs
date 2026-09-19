// 24.1, 24.2 — the aperture and the shed: what a mind holds, what it lets
// go, and that it sheds on its own K though the mesh never advances.

use super::common::*;
use crate::client::sdk::*;

/// Shed never removes a dialect entry a live idea glued, and never the genome.
#[test]
fn shed_spares_glued_dialect_and_the_genome() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let e2 = two_body_sentence([&[(1, 3), (1, 3)], &[(3, 5), (3, 5)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("shed"));
entity.epigenome.stand_dialect_at(e1.clone(), 0, 0);
entity.epigenome.stand_dialect_at(e2.clone(), 0, 0);
assert_eq!(entity.dialect_len(), 2);
let addr = e1.address();
entity.hold_chain(vec![(9, 0, 1, 2)]);
let idea = entity
    .contemplation
    .bubbles
    .scoped()
    .next()
    .map(|(k, _)| k)
    .expect("idea");
entity.contemplation.mark_glue(&idea, addr.clone());
let aperture = crate::physics::aperture_cycle() as u64;
// The first aperture after boot is unmeasured: nothing sheds while
// the stroke ring fills, however idle an entry is.
for tu in 1..aperture {
    entity.shed_at(tu);
    assert_eq!(entity.dialect_len(), 2, "an unmeasured aperture sheds nothing");
}
// The ring is full at this stroke: the aperture is measured, the
// idle unglued entry sheds, the glued one stands.
entity.shed_at(aperture + 1);
assert_eq!(entity.dialect_len(), 1, "the idle unglued entry sheds; the glued one stands");
assert!(entity.epigenome.working_word(&addr).is_some(), "the glued entry stands");
assert!(entity.epigenome.working_word(&e2.address()).is_none(), "the idle entry is gone");
assert_eq!(entity.epigenome.genome_len(), 0, "genome is untouched by shed");
}

/// 7.3, 24.1, Stage 12 S12.6a — THE MIND'S CARRIERS GROW BY THE SHED'S
/// RESIDUE, NOT BY THE STROKES. A voice that withholds a form with a new
/// body every stroke and sheds on its own K holds, at 4N strokes, no more
/// dialect and no more utterance receipts than at N plus one aperture: a
/// word idle for an aperture leaves, and its receipt leaves with it. The
/// receipt stayed, and `taught_up` grew by every word ever uttered.
#[test]
fn a_minds_carriers_grow_by_the_sheds_residue_not_by_the_strokes() {
use super::common::closed_turn;
let aperture = crate::physics::aperture_cycle() as u64;
let run = |strokes: u64| -> (usize, usize) {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("leak"));
    let form = closed_turn(None);
    for k in 1..=strokes {
        // A body no earlier stroke spoke: a bar one site longer each stroke.
        let bar: Vec<(i64, i64, u8, u8)> = (0..=k as i64).map(|c| (0, c, 1u8, 2u8)).collect();
        entity.withhold_form(form.clone(), &[], std::slice::from_ref(&bar), [1, 0, 0, 0]);
        entity.shed_at(k);
    }
    (entity.dialect_len(), entity.utterances_filed())
};
let n = 2 * aperture;
let (dialect_n, receipts_n) = run(n);
let (dialect_4n, receipts_4n) = run(4 * n);
assert!(
    dialect_4n <= dialect_n + aperture as usize,
    "dialect at 4N ({dialect_4n}) stays within an aperture of N ({dialect_n})"
);
assert!(
    receipts_4n <= receipts_n + aperture as usize,
    "utterance receipts at 4N ({receipts_4n}) stay within an aperture of N ({receipts_n}): a receipt leaves with its word"
);
}

/// An expressed word stands at the voice's K and TUniversal, so a shed
/// inside the aperture does not drop the mind's own speech.
#[test]
fn expressed_word_survives_shed_within_the_aperture() {
use crate::transducer::element::{canonical_chain, from_canonical};
let mut epi = Epigenome::new();
let residual = [(0i64, 0i64, 1u8, 2u8)];
assert!(epi.express(&residual, 7, 10), "the residual is claimed");
let addr = from_canonical(&canonical_chain(&residual).expect("one link is A5's chain")).address();
let d = epi.dialect.get(&addr).expect("stood");
assert_eq!(d.last_used_k, 7, "k is the voice's K");
assert_eq!(d.last_used_tu, 10, "tu is the standing extent in heights");
// The lifecycle is counted in the mind's OWN K (45.4), so the
// strokes that measure the aperture are K values, not heights.
let aperture = crate::physics::aperture_cycle() as u64;
let now = 7 + aperture - 1;
for k in 7..=now {
    epi.note_stroke(k);
}
let n = epi.shed(now);
assert_eq!(n, 0, "expressed within the aperture stands");
assert_eq!(epi.dialect_len(), 1);
assert!(epi.working_word(&addr).is_some());
}

/// The defect this coordinate change closes. Heights advance only on
/// admitted receipts (45.2), so a mind whose mesh admits nothing has a
/// TUniversal frozen at 0 — and a lifecycle read against it could never
/// fire, holding everything the mind ever stood for the life of the
/// process. A mind's own K advances on its own acts, so it reclaims
/// what it holds whether or not anyone answered it.
#[test]
fn a_mind_sheds_on_its_own_k_though_the_mesh_never_advances() {
use crate::transducer::element::{canonical_chain, from_canonical};
let mut epi = Epigenome::new();
let residual = [(0i64, 0i64, 1u8, 2u8)];
// Stood at the mind's K 1, with TUniversal frozen at 0 throughout:
// no receipt was ever admitted.
assert!(epi.express(&residual, 1, 0), "the residual is claimed");
let addr = from_canonical(&canonical_chain(&residual).expect("one link is A5's chain")).address();
assert_eq!(epi.dialect_len(), 1);
let aperture = crate::physics::aperture_cycle() as u64;
let now = 1 + aperture + 1;
for k in 1..=now {
    epi.note_stroke(k);
}
assert_eq!(epi.shed(now), 1, "unused past the aperture, it sheds");
assert_eq!(epi.dialect_len(), 0, "the mind reclaimed what it held");
assert!(epi.working_word(&addr).is_none());
}

/// 45.4 — THE MIND'S K IS ITS OWN ACTS, and no reply moves it. The test
/// above drives the epigenome with K values handed to it, so it proves the
/// shed's arithmetic and says nothing about where K comes from.
///
/// This one calls `stand` alone, which is 24.3's own act -- "perceive,
/// think, stand, utter" -- and reaches no wire at all. It spoke through
/// `speak` to a hypha that was not there, so it depended on a TCP connect
/// to a port nothing listened on failing, which §3.7 does not let a test
/// do ("what a test never does ... read the network"). Nothing about the
/// claim needed the wire: standing is complete before anything is uttered.
#[test]
fn standing_a_form_steps_the_minds_own_k() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("clock"));
assert_eq!(entity.session.voice().k(), 0, "a mind that has acted seats at zero");

// Standing is the act, and it is complete before anything is uttered.
let codes = entity.stand(std::slice::from_ref(&e1), 0);
assert_eq!(entity.dialect_len(), 1, "the form stands in the dialect");
assert_eq!(entity.session.voice().k(), 1, "standing it stepped the clock once");
assert_eq!(codes, vec![0xff], "and it is fresh, so an utterance would ask about it");

// Standing the same form again is a second act of this mind: it is
// already taught, so nothing would go on the wire at all, and the clock
// still steps. Nothing here read a k-step from anyone else.
entity.file_taught(&e1);
let codes = entity.stand(std::slice::from_ref(&e1), 0);
assert_eq!(entity.session.voice().k(), 2, "a second act, a second step");
assert_eq!(
    codes,
    vec![crate::anastome::dialogue::SPEECH_STOOD],
    "a vertex already taught stands and goes nowhere"
);
}

/// THE MIND HOLDS TWO CLOCKS AND THEY ARE NOT ONE. `absorb_taught` states
/// it in a line: "k is the voice's K (law 9), tu is TUniversal; they are not
/// one clock." `Seat::k` is `chain.next_step_index()` — this voice's own
/// receipt index, advancing only when THIS voice seals a receipt. TUniversal
/// is the height of the receipt ledger, read off a reply and only advancing
/// (23.8).
///
/// Each carrier says which it wants, and two said one thing and were fed the
/// other. `Contemplation.now_k` reads "TUniversal the entity last stamped"
/// and `Bubble.moved_k` reads "TUniversal when this bubble's charts last
/// moved"; both were stamped from `session.voice().k()`. Everything
/// downstream is a span between two of those stamps —
/// `held_since(now_k, moved_k)` is `pivot_loop`'s `t_extent`, `span_of` is
/// lane 𝒯 of a minted word, `retire_on_t` is the T-room — so all three
/// measured a span of the wrong clock. A span in a voice's own K is not
/// comparable across voices; TUniversal is the shared coordinate.
///
/// `shed` is the deliberate opposite and stays: 45.4 rules that a mind sheds
/// by its own receipt index and NEVER by the mesh's height, because heights
/// advance only on admitted receipts, so a fleet refusing work could not
/// reclaim what it held.
#[test]
fn the_contemplation_is_stamped_with_tuniversal_and_the_shed_with_the_voices_own_k() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("clocks"));
    // Two clocks, driven apart: a receipt this voice sealed, and a height
    // read off the mesh.
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let _ = entity.stand(std::slice::from_ref(&e1), 0);
    let k = entity.session.voice().k();
    entity.advance_height(500);
    assert_ne!(k, entity.now_tu(), "the fixture separates them: {k} against 500");
    assert_eq!(entity.now_tu(), 500, "TUniversal is the height read off the mesh");

    // The contemplation takes TUniversal.
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    let _ = entity.expand_ideas();
    assert_eq!(
        entity.contemplation.now_k(),
        entity.now_tu(),
        "the contemplation's clock is the one its own field doc names"
    );
    assert_ne!(
        entity.contemplation.now_k(),
        k,
        "and it is not the voice's receipt index"
    );
}

/// 25 DOOR 3 — A THOUGHT THE ROOM CANNOT HOLD IS WITHHELD, NEVER EVICTED.
///
/// Room is `boundary_measure` against the voice's reserve, LANE BY LANE
/// (law 5): the open thoughts' own four-lane measure, never their sum and
/// never a count of bubbles. `retire_on_t` read a bubble count against
/// `reserve_max`, which is the box's RAM at boot — probed at 88 on this
/// machine — and it is deleted.
///
/// The three readings are the door. With room, the thought is held. With a
/// lane spent, the new thought is NOT held — and, the reading that matters,
/// every thought already held is still there. Eviction chooses: whichever
/// held thought went would be picked by age, size or arrival, and rules 4
/// and 15 refuse all three. What the withheld thought gets instead is its
/// demand standing on the court's docket, which `withhold_form` does
/// whether or not there was room.
#[test]
fn a_thought_the_room_cannot_hold_is_withheld_not_evicted() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("room"));
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    let held_now = entity.contemplation.len();
    assert!(held_now > 0, "a mind with room holds the thought");
    // Spend every lane: a reserve of zero is no room on any lane.
    entity.physics.reserve = [0; 4];
    let second = crate::transducer::sentence::Form {
        remainder: crate::transducer::element::place_chain(&[(4i64, 4i64, 7u8, 8u8)])
            .expect("a one-link chain places"),
        ..crate::transducer::sentence::Form::default()
    };
    assert!(
        !entity.room_for(&second),
        "a lane with no room has no room for a thought that occupies one"
    );
    assert!(
        !entity.hold_form(second, &[], &[]),
        "a lane with no room withholds the new thought"
    );
    assert_eq!(
        entity.contemplation.len(),
        held_now,
        "and NOTHING already held is evicted: eviction chooses, and rules 4 and 15 refuse it"
    );
}
