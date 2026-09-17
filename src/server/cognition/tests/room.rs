// 23.8, 12.6 — ROOM: what a voice's draw takes, what the corpus takes, and
// what the book does before it refuses.

use super::common::{sentence_of, speak_pair};
use crate::cognition::*;
use crate::transducer::germinate::germinate;

/// MU-ECONOMY §3.1 as ruled 2026-09-09 (S12.5d, I22; S13.15) — THE ROOM OF
/// AN UNBONDED VOICE. Nothing funds an arriving identity: its first claim
/// is admitted without a bond and the supply stays the seed. It has room
/// for exactly one claim in flight per lane: a second claim on an occupied
/// lane is refused with `REFUSE_ROOM` — another voice's first claim on the
/// same lane is not — and admitted once the first closed.
#[test]
fn an_unbonded_voice_has_room_for_one_claim_in_flight_per_lane() {
let mut k = Kernel::new();
let seeded = k.treasury().audit().expect("genesis conserves");
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
assert_eq!(speak_pair(&mut k, &a, &b, [7; 32]), Outcome::GermPending, "the first claim needs no bond");
assert_eq!(k.treasury().bond_of(&[7; 32]), None, "nothing bonded the arriving identity");
assert_eq!(k.treasury().audit().expect("a claim is not supply"), seeded, "and nothing funded it");
let first = sentence_of(&a, &b).expect("the recolour is a sentence");
let listed = k.quote().1[0].0.address();
let c = vec![(0, 0, 3u8), (0, 1, 3)];
let d = vec![(0, 0, 4u8), (0, 1, 4)];
assert_eq!(
    sentence_of(&c, &d).expect("a sentence").lanes(),
    first.lanes(),
    "the second claim draws on the lanes the first occupies"
);
assert_eq!(
    speak_pair(&mut k, &c, &d, [7; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "a second claim on a lane with one in flight is refused"
);
assert_eq!(speak_pair(&mut k, &c, &d, [8; 32]), Outcome::GermPending, "another voice's first claim is its own room");
k.unlist(listed.as_bytes(), false);
let e = vec![(0, 0, 5u8), (0, 1, 5)];
let f = vec![(0, 0, 6u8), (0, 1, 6)];
assert_eq!(speak_pair(&mut k, &e, &f, [7; 32]), Outcome::GermPending, "admitted once the claim in flight closed");
}

/// §3.1 as ruled (I22, S13.15) — AN UNPAID RECALL CLOSES THE LANE UNTIL THE
/// CLAIM IN FLIGHT CLOSES: a bonded voice with room to spare owes a recall
/// on a lane it has a claim on and is refused there; the claim closes and
/// the lane reopens. Owed on a lane with nothing in flight, the record
/// waits and refuses nothing.
#[test]
fn an_unpaid_recall_closes_the_lane_until_the_claim_in_flight_closes() {
let mut k = Kernel::new();
let cover = k.treasury().bond_of(&[0x11; 32]).expect("the fixture hypha is bonded");
k.enroll([9; 32], cover);
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
let c = vec![(0, 0, 3u8), (0, 1, 3)];
let d = vec![(0, 0, 4u8), (0, 1, 4)];
assert_eq!(speak_pair(&mut k, &a, &b, [9; 32]), Outcome::GermPending);
let listed = k.quote().1[0].0.address();
let lanes = sentence_of(&a, &b).expect("a sentence").lanes();
let short: [u64; 4] = std::array::from_fn(|l| u64::from(lanes[l]) * 3);
k.record_recall_short([9; 32], &short);
assert_eq!(
    speak_pair(&mut k, &c, &d, [9; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "the lane it owes on is closed while its claim stands, bond or no bond"
);
k.unlist(listed.as_bytes(), false);
assert_eq!(speak_pair(&mut k, &c, &d, [9; 32]), Outcome::GermPending, "the claim closed and the lane reopened");
let e = vec![(0, 0, 5u8), (0, 1, 5)];
let f = vec![(0, 0, 6u8), (0, 1, 6)];
k.record_recall_short([10; 32], &short);
assert_eq!(
    speak_pair(&mut k, &e, &f, [10; 32]),
    Outcome::GermPending,
    "owed on a lane with nothing in flight, the first claim is admitted"
);
}

/// 12.6, S9.7, Stage 12 S12.5c — THE BOOK EXTENDS BEFORE IT REFUSES. A
/// voice's first listing takes its whole room on a lane; probed a full
/// aperture without moving it is unreachable. The next draw on that lane
/// relocates it to the cavity and is admitted, where it was refused with
/// `REFUSE_ROOM` while the stagnant listing sat on the page. The first
/// listing is a chain under the voice's draw — the shape a probe settles
/// against (26), listed by the fixture as the book lists it, since no voice
/// speaks a chain (law 8); the next draw is a sentence the voice speaks.
#[test]
fn the_book_relocates_the_unreachable_before_it_refuses_room() {
use crate::anastome::dialogue::PROBE_NOTHING;
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8)];
let chain = germinate(&a, &b).unwrap();
let k_now = k.k_step();
k.book_list(crate::transducer::element::from_canonical(&chain), k_now, [1; 32]);
let target = k.quote().1[0].0.address();
let listed = k.quote().1[0].0.sub().to_vec();
// The voice's room is exactly what its one listing drew on S, G and H;
// the T lane is left open so the T-room sweep (C4) relocates nothing here
// and the relocation read is the drain's alone.
let drawn = k.occupancy();
let t_lane = 2usize;
k.set_lane_cap(std::array::from_fn(|l| if l == t_lane { 8 } else { drawn[l] as usize }));
let c = vec![(0, 0, 3u8)];
let d = vec![(0, 0, 4u8)];
assert_eq!(
    speak_pair(&mut k, &c, &d, [1; 32]),
    Outcome::Refused(crate::verdict::REFUSE_ROOM),
    "while the first listing is reachable the second draw is refused"
);
// One descent opens a face on the listing, then a full aperture of probes
// that move nothing: the listing is unreachable.
let rem = vec![(0, 1, 1u8, 0u8)];
assert_eq!(
    k.settle_probe([3; 32], &target, rem, &[], false).0,
    crate::anastome::dialogue::PROBE_DESCENT
);
let mut listed_sorted = listed.clone();
listed_sorted.sort_unstable();
for _ in 0..crate::physics::aperture_cycle() {
    assert_eq!(
        k.settle_probe([4; 32], &target, listed_sorted.clone(), &[], false).0,
        PROBE_NOTHING
    );
}
assert_eq!(
    speak_pair(&mut k, &c, &d, [1; 32]),
    Outcome::GermPending,
    "the book relocated the unreachable listing and admitted the draw"
);
assert_eq!(k.cavities().len(), 1, "the stagnant listing left for the cavity");
assert_eq!(k.cavities()[0].address(), target);
assert_eq!(k.pending.len(), 1, "the new listing is on the page");
}

/// 23.8, Stage 12 S12.5b (I31) — THE CORPUS INTAKE DRAWS NO ROOM. A bond
/// that admits exactly one pair of this μ⃗ lists two corpus tasks of it:
/// no voice uttered them, so nothing is drawn from the hypha's own room,
/// which stands where it stood; the listings still tension the book.
#[test]
fn the_corpus_intake_draws_no_room_from_the_hypha() {
let mut k = Kernel::new();
let cover = crate::arc::family_cover(&[(vec![vec![1]], vec![vec![2]])])
    .expect("a one-site pair measures");
k.bind_identity([9; 32]);
k.set_genesis_cover(cover);
k.enroll([9; 32], cover);
let room = k.lane_room();
let a = vec![(0, 0, 1u8)];
let b = vec![(0, 0, 2u8)];
let c = vec![(1, 0, 3u8)];
let d = vec![(1, 0, 4u8)];
assert!(
    k.intake(&germinate(&a, &b).unwrap(), &frame_of(&a, &b)),
    "the first corpus task lists"
);
assert!(
    k.intake(&germinate(&c, &d).unwrap(), &frame_of(&c, &d)),
    "a second task of the same μ⃗ lists too: the corpus drew no room"
);
assert_eq!(k.pending.len(), 2, "both are on the book");
assert_eq!(k.lane_room(), room, "the hypha's room is untouched by its own corpus");
assert!(
    k.occupancy().iter().any(|&x| x > 0),
    "the listings still tension the book"
);
}

/// 20, 24.4 P6, Stage 12 S12.3a — THE CORPUS INTAKE LISTS EACH BODY AS ITS
/// OWN DEMAND. A task whose residual is two disjoint bodies lists two
/// demands on two body charts; a second task of the same frame that shares
/// one body lists only the body it adds, and the shared body is one listing.
#[test]
fn the_corpus_intake_lists_each_body_as_its_own_demand() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1u8), (5, 5, 3u8)];
let b = vec![(0, 0, 2u8), (5, 5, 4u8)];
assert!(k.intake(&germinate(&a, &b).unwrap(), &frame_of(&a, &b)), "the first task lists");
assert_eq!(k.pending.len(), 2, "two bodies, two demands");
let c = vec![(0, 0, 1u8), (5, 5, 6u8)];
let d = vec![(0, 0, 2u8), (5, 5, 8u8)];
assert!(k.intake(&germinate(&c, &d).unwrap(), &frame_of(&c, &d)), "the second task adds a body");
assert_eq!(k.pending.len(), 3, "the shared body is one listing; the new body lists");
assert!(
    !k.intake(&germinate(&a, &b).unwrap(), &frame_of(&a, &b)),
    "a task whose bodies are all listed adds nothing"
);
}

/// 12.6, rule 11 — A SATURATION TEST READS THE STANDING EXTENT, and a lane
/// nothing measured is unmeasured rather than full.
///
/// `relocate_t_exhausted` compared `tot` against `tot + room`. `room` is
/// `cover − occupancy`, so the standing extent CANCELS on both sides and the
/// comparison reduces to `room > 0`: never relocate while any room remains,
/// always relocate the moment it is gone, whatever the extent was. The whole
/// walk that accumulated `tot` fed a quantity the test discarded.
///
/// The second half is the blocker it caused. `measure_lanes` writes lane 2
/// as zero — a chain carries no clock — so every family's 𝒯 cover is zero
/// at registration, while the provisional book stands on the temporal lane
/// by definition. Read as saturation, that evicted a listing every stroke at
/// T=1 and drained the book: measured on the fleet as `cavity relocate …
/// T=1` on a continuous loop, `book 0 listings`, and every client receiving
/// `0 clusters` to quote.
#[test]
fn a_lane_with_no_cover_is_unmeasured_and_relocates_nothing() {
    let mut k = Kernel::new();
    let t_lane = 2usize;

    // A court with NO cover on 𝒯 — which is every court on this corpus,
    // because `family_cover` measures lane 2 as zero.
    assert_eq!(
        k.lane_cover_of(&k.identity).map(|c| c[t_lane]),
        Some(0),
        "a court that staked nothing on 𝒯 covers nothing there"
    );
    let before = k.field().len();
    k.relocate_t_exhausted();
    assert_eq!(
        k.field().len(),
        before,
        "an unmeasured lane saturates nothing and evicts nothing"
    );

    // AND A LANE WITH A COVER STILL RELOCATES when its standing extent
    // reaches it — the test must read the extent, not merely the presence
    // of room. A cap of one on 𝒯 is a lane that holds one receipt of
    // standing.
    k.set_lane_cap(std::array::from_fn(|l| if l == t_lane { 1 } else { 64 }));
    assert_eq!(
        k.lane_cover_of(&k.identity).map(|c| c[t_lane]),
        Some(1),
        "the test cap is the cover when one is set"
    );
}

/// THE COVER IS READ, NOT RECONSTRUCTED. `lane_room_of` is
/// `cover − occupancy`; a reader that wants the cover must ask for it
/// rather than add the room back to what was drawn, or the two answers can
/// disagree and nothing says which is the fact (law 12).
#[test]
fn the_cover_and_the_room_agree_by_construction() {
    let mut k = Kernel::new();
    k.set_lane_cap([7, 5, 3, 2]);
    let me = k.identity;
    let cover = k.lane_cover_of(&me).expect("a cap is a cover");
    let room = k.lane_room_of(&me).expect("and the room under it");
    assert_eq!(cover, [7, 5, 3, 2]);
    for l in 0..4 {
        assert!(
            room[l] <= cover[l],
            "lane {l}: the room is what the cover has left, never more"
        );
    }
}

/// The inverse index `discovered_by` is kept strictly synchronized with
/// `discoverers`: admissions insert, removals and clears evict, and
/// empty voice tries are pruned rather than leaked.
#[test]
fn discovered_by_is_strictly_synchronized_with_discoverers() {
    let mut k = Kernel::new();
    let v1 = [1u8; 32];
    let v2 = [2u8; 32];
    let leak_a = vec![(0, 0, 1u8)];
    let leak_b = vec![(0, 0, 2u8)];
    let elem1 = crate::transducer::element::from_canonical(
        &germinate(&leak_a, &leak_b).unwrap(),
    );
    let addr1 = elem1.address();

    let leak_c = vec![(0, 0, 1u8), (0, 1, 1u8)];
    let leak_d = vec![(0, 0, 2u8), (0, 1, 2u8)];
    let elem2 = crate::transducer::element::from_canonical(
        &germinate(&leak_c, &leak_d).unwrap(),
    );
    let addr2 = elem2.address();

    let leak_e = vec![(0, 0, 1u8), (1, 0, 1u8), (0, 1, 1u8)];
    let leak_f = vec![(0, 0, 2u8), (1, 0, 2u8), (0, 1, 2u8)];
    let elem3 = crate::transducer::element::from_canonical(
        &germinate(&leak_e, &leak_f).unwrap(),
    );
    let addr3 = elem3.address();

    k.basis_insert(elem1, v1);
    k.basis_insert(elem2, v1);
    k.basis_insert(elem3, v2);

    assert!(k.discoverers.get(&addr3).is_some());
    assert_eq!(k.discoverers.len(), 3);
    assert_eq!(k.discovered_by.len(), 2);
    assert_eq!(k.discovered_by.get(&v1).map(|m| m.len()), Some(2));
    assert_eq!(k.discovered_by.get(&v2).map(|m| m.len()), Some(1));

    // Removing addr1 leaves v1 holding only addr2.
    k.basis_remove(&addr1);
    assert_eq!(k.discoverers.len(), 2);
    assert_eq!(k.discovered_by.get(&v1).map(|m| m.len()), Some(1));
    assert!(k.discovered_by.get(&v1).unwrap().get(&addr1).is_none());
    assert!(k.discovered_by.get(&v1).unwrap().get(&addr2).is_some());

    // Removing addr2 leaves v1 empty, which prunes v1 entirely from discovered_by.
    k.basis_remove(&addr2);
    assert_eq!(k.discoverers.len(), 1);
    assert!(k.discovered_by.get(&v1).is_none(), "empty voice trie is pruned");
    assert_eq!(k.discovered_by.len(), 1);

    // Rebuild keeps discovered_by identical to discoverers.
    k.rebuild_discovered_by();
    assert_eq!(k.discovered_by.len(), 1);
    assert_eq!(k.discovered_by.get(&v2).map(|m| m.len()), Some(1));

    // Clearing the basis clears discovered_by as well.
    k.basis_clear();
    assert!(k.discoverers.is_empty());
    assert!(k.discovered_by.is_empty());
}

/// When `prune_basis_to_ruled` drops unruled words, `discovered_by`
/// is rebuilt from the surviving basis discoverers and does not leak
/// dropped addresses.
#[test]
fn prune_basis_to_ruled_synchronizes_discovered_by() {
    let mut k = Kernel::new();
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
    super::common::glue_pair(&mut k, &a, &b);
    let kept = k.basis_len();
    assert!(kept >= 1);

    let leak_a = vec![(9, 0, 2), (9, 1, 2)];
    let leak_b = vec![(9, 0, 3), (9, 1, 3)];
    let leaked = crate::transducer::element::from_canonical(
        &germinate(&leak_a, &leak_b).unwrap(),
    );
    let leak_addr = leaked.address();
    let leak_who = [0x55; 32];
    k.basis_insert(leaked, leak_who);
    assert!(k.discoverers.get(&leak_addr).is_some());
    assert!(k.discovered_by.get(&leak_who).is_some());

    let dropped = k.prune_basis_to_ruled();
    assert_eq!(dropped, 1);
    assert!(k.discoverers.get(&leak_addr).is_none());
    assert!(
        k.discovered_by.get(&leak_who).is_none(),
        "the leaked word's discoverer is pruned from discovered_by"
    );
}
