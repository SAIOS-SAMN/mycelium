// 24.1, 24.2, MU-CLIENT §4.13 — the shed: what a mind holds, and what it
// lets go where nothing live references it.

use super::common::*;
use crate::client::sdk::*;

/// MU-CLIENT §4.13 — A DIALECT WORD STANDS WHILE A THOUGHT GLUES IT, and one
/// nothing references is shed at once — no window, no clock; the genome
/// never sheds.
#[test]
fn shed_keeps_glued_dialect_word_and_genome() {
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let e2 = two_body_sentence([&[(1, 3), (1, 3)], &[(3, 5), (3, 5)]]);
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("shed"));
    let _ = entity.stand(&[e1.clone(), e2.clone()], 0, &[]);
    assert_eq!(entity.dialect_len(), 2);
    let addr = e1.address();
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    let idea = entity.contemplation.bubbles.scoped().next().map(|(k, _)| k).expect("idea");
    entity.contemplation.mark_glue(&idea, addr.clone());
    let _ = entity.shed();
    assert_eq!(entity.dialect_len(), 1, "the unreferenced word sheds; the glued one stands");
    assert!(entity.epigenome.working_word(&addr).is_some(), "the glued word stands");
    assert!(entity.epigenome.working_word(&e2.address()).is_none(), "the unreferenced word is gone");
    assert_eq!(entity.epigenome.genome_len(), 0, "the genome is untouched by the shed");
}

/// MU-CLIENT §4.13 — A WORD THE MIND SAID AND A COURT TAUGHT STANDS UNTIL
/// THE GENOME HOLDS IT: its receipt is what keeps it from being said twice,
/// so it is referenced while it is unratified, whatever its thoughts did.
#[test]
fn shed_keeps_taught_word_until_ratified() {
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("taught"));
    entity.remember_spoken(&e1);
    let _ = entity.shed();
    assert!(entity.epigenome.working_word(&e1.address()).is_some(), "taught and unratified, it stands");
    assert!(entity.already_taught(&e1), "and its receipt with it");
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
fn stand_acts_at_mind_k_and_moves_no_clock() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("clock"));
assert_eq!(entity.own_k(), 0, "a mind none of whose orbits returned seats at zero");

// Standing is the act, and it is complete before anything is uttered. It
// stands at the mind's K; only a return moves K (MU-CLIENT §4.4(4)).
let codes = entity.stand(std::slice::from_ref(&e1), 0, &[]);
assert_eq!(entity.dialect_len(), 1, "the form stands in the dialect");
assert!(!entity.close_cycle(0), "standing is no return: the stroke closes nothing");
assert_eq!(entity.own_k(), 0, "K stands");
assert!(entity.close_cycle(1), "an orbit returned: the stroke closes");
assert_eq!(entity.own_k(), 1, "the return stepped the clock once");
assert_eq!(codes, vec![0xff], "and it is fresh, so an utterance would ask about it");

// Standing the same form again is a USE of it (ruled 2026-10-14: a
// mind repeating itself is mumbling, not acting): it is already taught, so
// nothing would go on the wire at all. Nothing here read a k-step from
// anyone else.
entity.file_taught(&e1);
let codes = entity.stand(std::slice::from_ref(&e1), 0, &[]);
assert!(!entity.close_cycle(0), "a use returns nothing");
assert_eq!(entity.own_k(), 1, "the clock stands");
assert_eq!(
    codes,
    vec![crate::anastome::dialogue::SPEECH_STOOD],
    "a vertex already taught stands and goes nowhere"
);
}

/// THE MIND HOLDS TWO CLOCKS AND THEY ARE NOT ONE. `absorb_taught` states
/// it in a line: "k is the mind's K (law 9), tu is TUniversal; they are not
/// one clock." The mind's K counts the orbits it acted in (78.49).
/// TUniversal is the height of the court's receipt ledger, read off a reply
/// and only advancing (23.8).
///
/// Each carrier says which it wants, and two said one thing and were fed the
/// other. `Contemplation.now_k` reads "TUniversal the entity last stamped"
/// and `Bubble.moved_k` reads "TUniversal when this bubble's charts last
/// moved"; both were stamped from the mind's own K. Everything
/// downstream is a span between two of those stamps —
/// `held_since(now_k, moved_k)` is `pivot_orbit`'s `t_extent`, `span_of` is
/// lane 𝒯 of a minted word, `retire_on_t` is the T-room — so all three
/// measured a span of the wrong clock. A span in a mind's own K is not
/// comparable across voices; TUniversal is the shared coordinate.
#[test]
fn expand_ideas_stamps_contemplation_with_tuniversal() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("clocks"));
    // Two clocks, driven apart: an orbit this mind acted in, and a height
    // read off the mesh.
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let _ = entity.stand(std::slice::from_ref(&e1), 0, &[]);
    entity.close_cycle(1);
    let k = entity.own_k();
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
        "and it is not the mind's own K"
    );
}

/// 25 DOOR 3 — A THOUGHT THE ROOM CANNOT HOLD IS WITHHELD, NEVER EVICTED.
///
/// Room is `boundary_measure` against the voice's reserve, LANE BY LANE
/// (law 5): the open thoughts' own four-lane measure, never the thoughts' sum and
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
fn hold_form_withholds_without_room_never_evicts() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("room"));
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    let held_now = entity.contemplation.len();
    assert!(held_now > 0, "a mind with room holds the thought");
    // Spend every lane: a reserve of zero is no room on any lane.
    entity.physics.reserve = crate::physics::Reserve::of([0; 4]);
    let second = crate::transducer::sentence::Form {
        remainder: crate::transducer::element::place_chain(&[(4i64, 4i64, 7u8, 8u8)])
            .expect("a one-link chain places"),
        ..crate::transducer::sentence::Form::default()
    };
    assert!(
        !entity.room_for(&second),
        "a lane with no room has no room for a thought that occupies one"
    );
    let mut b = entity.stroke();
    assert!(
        !entity.hold_form(second, &[], &[], &mut b),
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
fn hold_form_draws_own_lanes_and_release_returns_them() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("escrow"));
    let before = entity.physics.reserve.lanes();
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
    let mut b = entity.stroke();
    assert!(entity.hold_form(f, &[], &[], &mut b), "a mind with room holds it");
    let after = entity.physics.reserve.lanes();
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

    // MU-CLIENT §7.1 Phase 3 — THE ESCROW RETURNS ONCE, EXACTLY, when the
    // thought is released by its orbit: the reserve stands where it stood
    // before the hold, lane by lane.
    let held: Vec<Vec<u8>> = entity.contemplation.bubbles.scoped().filter(|(_, b)| b.escrow == drew).map(|(k, _)| k).collect();
    let [key] = held.as_slice() else { panic!("one thought holds the draw: {}", held.len()) };
    entity.contemplation.release_thought(key);
    entity.physics.reserve.restore(entity.contemplation.take_returned_escrow());
    assert_eq!(entity.physics.reserve.lanes(), before, "released, it returned exactly what it drew");
    assert_eq!(entity.contemplation.take_returned_escrow(), [0; 4], "once");
}

/// Z4d, MU-CLIENT §7.1 Phase 3 — A RELEASE RETURNS ONLY WHAT THE HOLD DREW,
/// and a thought held without a draw returns nothing: escrow is never
/// restored undrawn, so the reserve cannot inflate. A remainder held alone
/// draws nothing and holds nothing in escrow; released, it returns nothing.
#[test]
fn release_thought_returns_only_what_hold_drew() {
    let mut entity = Entity::open("127.0.0.1:1", Kernel::named("drew"));
    let before = entity.physics.reserve.clone();
    entity.hold_chain(vec![(9, 0, 1, 2)]);
    assert_eq!(entity.physics.reserve, before, "a hold that drew nothing takes nothing");
    let held: Vec<Vec<u8>> = entity.contemplation.bubbles.scoped().map(|(k, _)| k).collect();
    let [key] = held.as_slice() else { panic!("one thought held: {}", held.len()) };
    assert_eq!(entity.contemplation.bubbles.get(key).map(|b| b.escrow), Some([0; 4]), "it holds nothing in escrow");
    entity.contemplation.release_thought(key);
    assert!(entity.contemplation.bubbles.get(key).is_none(), "released");
    entity.physics.reserve.restore(entity.contemplation.take_returned_escrow());
    assert_eq!(entity.physics.reserve, before, "and it returns NOTHING, because nothing was drawn");
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
fn take_unpivoted_drains_set_of_coordinates() {
    let mut epi = Epigenome::new();
    let a = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let addr = a.address();
    assert_eq!(epi.unpivoted_len(), 0, "a fresh mind has no pending work");

    epi.learn(&a.to_bytes().expect("the record writes"), &mut super::common::budget());
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
    epi.learn(&b.to_bytes().expect("the record writes"), &mut super::common::budget());
    assert_eq!(epi.unpivoted_len(), 1);
    assert!(epi.retire(b_addr.as_bytes()), "the word retires");
    assert_eq!(epi.unpivoted_len(), 0, "and leaves the pending set with it");
}
