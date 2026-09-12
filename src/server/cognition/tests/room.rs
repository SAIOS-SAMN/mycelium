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
    .expect("one 1-cell pair measures");
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
