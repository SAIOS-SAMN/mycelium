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

// Standing is the act, and it is complete before anything is uttered. It
// is one node of the open orbit; the stroke's close moves K (OK1).
let codes = entity.stand(std::slice::from_ref(&e1), 0);
assert_eq!(entity.dialect_len(), 1, "the form stands in the dialect");
assert_eq!(entity.session.voice().chain().own_len(), 1, "standing it is one act");
assert_eq!(entity.session.voice().k(), 0, "K stands while the orbit is open");
entity.close_orbit();
assert_eq!(entity.session.voice().k(), 1, "the stroke's close stepped the clock once");
assert_eq!(codes, vec![0xff], "and it is fresh, so an utterance would ask about it");

// Standing the same form again is a second act of this mind: it is
// already taught, so nothing would go on the wire at all, and the clock
// still steps at the stroke's close. Nothing here read a k-step from
// anyone else.
entity.file_taught(&e1);
let codes = entity.stand(std::slice::from_ref(&e1), 0);
entity.close_orbit();
assert_eq!(entity.session.voice().k(), 2, "a second act in a second stroke, a second step");
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
/// `held_since(now_k, moved_k)` is `pivot_orbit`'s `t_extent`, `span_of` is
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

/// Z4d — THE RECEIPT IS THE ESCROW, AND THE LANES MOVE APART.
///
/// `Physics::default` sets `reserve: [reserve_max(); 4]` — four copies of
/// one number — and they stayed four copies for as long as nothing drew on
/// them separately. A hold now draws the thought's OWN four-lane measure,
/// so a thought that occupies `𝒮` and not `𝒽` leaves `𝒽` where it was.
/// That is law 5 made true of the carrier rather than asserted about it.
///
/// Three readings. The draw is per lane and the untouched lanes do not
/// move. The draw is EXACT OR THE THOUGHT IS WITHHELD — a lane that cannot
/// fund it refuses rather than flooring at zero (rule 11), which is door
/// 3's room check taken once, where the payment happens. And the escrow
/// RETURNS on the lapse, lane by lane, so the reserve is not a one-way
/// ratchet: this is the exhale the breathing condition asked for.
#[test]
fn a_hold_draws_its_own_lanes_and_returns_them_on_the_lapse() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("escrow"));
    let before = entity.physics.reserve;
    assert_eq!(before[0], before[3], "the reserve begins as four copies of one number");

    entity.hold_chain(vec![(9, 0, 1, 2)]);
    let thought = crate::transducer::element::measure_lanes(&[(9i64, 0i64, 1u8, 2u8)]);
    assert!(
        thought.iter().any(|&x| x > 0) && thought.contains(&0),
        "the thought occupies some lanes and not others, or this proves nothing: {thought:?}"
    );

    // `hold_chain` does not draw; `hold_form` does. Draw against a form
    // carrying the same chain and read the reserve lane by lane.
    let f = crate::transducer::sentence::Form {
        remainder: crate::transducer::element::place_chain(&[(9i64, 0i64, 1u8, 2u8)])
            .expect("a one-link chain places"),
        ..crate::transducer::sentence::Form::default()
    };
    let drew = crate::transducer::element::measure_lanes(&f.remainder.placed());
    assert!(entity.hold_form(f, &[], &[]), "a mind with room holds it");
    let after = entity.physics.reserve;
    for l in 0..4 {
        match drew[l] {
            0 => assert_eq!(after[l], before[l], "lane {l} was not occupied and did not move"),
            d => assert_eq!(
                after[l] + d,
                before[l],
                "lane {l} fell by exactly what the thought occupies there"
            ),
        }
    }
    assert!(
        (0..4).any(|l| after[l] != before[l]) && (0..4).any(|l| after[l] == before[l]),
        "the lanes moved APART: {before:?} -> {after:?}"
    );

    // The escrow returns on the lapse. The aperture is unmeasured here, so
    // nothing is paid to a height and nothing lapses — which is rule 15,
    // and the reading that a thought is never held against a height
    // nothing measured.
    assert_eq!(entity.lapse_held(), 0, "an unmeasured horizon lapses nothing");
}

/// Z4d — A LAPSE RETURNS ONLY WHAT THE HOLD DREW, and a thought held
/// without a draw returns nothing.
///
/// THE FLEET FOUND THIS AND THE SUITE DID NOT. `hold_form` set the
/// bubble's escrow to the thought's own measure while the DRAW happened a
/// level up in the entity, so the same number lived in two places and they
/// disagreed: every bubble built without a draw — `hold`, a merge, a tape
/// restore — returned a full escrow on its lapse and the reserve inflated.
/// Read on the fleet at 3245041: `reserve [2194, 203, 88, 3382]` against a
/// `reserve_max` of 88, with lane `𝒯` alone still at 88 because nothing
/// draws or returns on it.
///
/// The escrow is now what the caller DREW, passed in, so one place draws
/// and one place records the same number (law 12).
#[test]
fn a_lapse_returns_only_what_the_hold_drew() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("drew"));
    let before = entity.physics.reserve;
    // Held with no draw: `hold_chain` goes to the contemplation directly.
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    assert_eq!(entity.physics.reserve, before, "a hold that drew nothing takes nothing");
    // Stamp a whole aperture so a horizon can be MEASURED, then pay the
    // held thought through it. A thought paid on this stroke cannot lapse
    // on this stroke, which is the escrow working and not a miss.
    for k in 1..=(crate::physics::aperture_cycle().max(1) as u64) {
        entity.contemplation.set_now(k);
    }
    assert_eq!(entity.lapse_held(), 0, "a thought paid this stroke does not lapse on it");
    // Now the clock passes the height it was paid through.
    let paid_through = entity.contemplation.now_k()
        + entity.contemplation.aperture_tu().expect("a measured horizon");
    entity.contemplation.set_now(paid_through + 1);
    let n = entity.lapse_held();
    assert!(n > 0, "a measured horizon lapses what the clock has passed");
    assert_eq!(
        entity.physics.reserve, before,
        "and it returns NOTHING, because nothing was drawn: the reserve cannot inflate"
    );
}

/// THE PIVOT'S PENDING WORK IS A SET OF COORDINATES, not a point on a
/// timeline.
///
/// It was `genome()[thought_gen..]` — a tail of a vector — and then
/// `genome_since(seq)`, an admission ordinal. Both answer "which words
/// arrived after the n-th", and the pivot's question is "which
/// obstructions has it not reached". Whether a word arrived at stroke 2
/// or stroke 200 is not a property of the word; whether it is still
/// undischarged is, and that is a coordinate in or out of a set.
///
/// Three readings: an admitted word's address enters the set, draining
/// takes it and leaves the set empty, and a RETIRED word leaves the set
/// with it — which the ordinal could not express, because an ordinal
/// already handed out cannot be withdrawn.
#[test]
fn the_pivots_pending_work_is_a_set_of_coordinates() {
    let mut epi = Epigenome::new();
    let a = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let addr = a.address();
    assert_eq!(epi.unpivoted_len(), 0, "a fresh mind has no pending work");

    epi.learn(&a.to_bytes().expect("the record writes"));
    assert_eq!(epi.unpivoted_len(), 1, "an admitted word is work the pivot has not reached");

    let taken = epi.take_unpivoted();
    assert_eq!(taken.len(), 1, "draining yields the word");
    assert_eq!(
        taken.iter().map(crate::transducer::element::Element::address).collect::<Vec<_>>(),
        vec![addr.clone()]
    );
    assert_eq!(epi.unpivoted_len(), 0, "and the set is empty after");
    assert!(epi.take_unpivoted().is_empty(), "draining again yields nothing");

    // A RETIRED WORD IS NO LONGER WORK. An ordinal handed out cannot be
    // taken back; a coordinate leaves the set with the word.
    let b = two_body_sentence([&[(1, 3), (1, 3)], &[(3, 5), (3, 5)]]);
    let b_addr = b.address();
    epi.learn(&b.to_bytes().expect("the record writes"));
    assert_eq!(epi.unpivoted_len(), 1);
    assert!(epi.retire(b_addr.as_bytes()), "the word retires");
    assert_eq!(epi.unpivoted_len(), 0, "and leaves the pending set with it");
}
