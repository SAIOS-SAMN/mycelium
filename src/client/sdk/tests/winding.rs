// MU-CLIENT §4.4(4), §4.12, §3.1, 78.167 D1 — THE WINDING CARRIER: what the
// mind says leaves at its returns; a scene waiting on comprehension lapses
// with its last thought; a meaning given whole is derived once the words
// beside it are known.

use super::common::*;
use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};

/// MU-CLIENT §4.4(4), §7.1 R2 — STOOD NOW, SAID AT THE RETURN OF THE THOUGHT
/// IT SPEAKS FOR, AND AT NO OTHER. Two fresh sentences spoken under two
/// scenes stand in the dialect at once — the mind's own act — and nothing
/// reaches the carrier facing the court. Said again before its return, a
/// sentence is the one thing held, never two. A thought of the first scene
/// comes round by the field's own motion (`bind_a_thought`: two hours of a
/// slide, the escaping relation bound by the drive); the winding reads that
/// return, K steps by exactly it, and the first scene's speech is posted —
/// the second's, whose scene no thought of has returned, is still held.
#[test]
fn a_speech_leaves_at_its_own_thoughts_return_and_no_other() {
    let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
    let e2 = two_body_sentence([&[(5, 6), (5, 6)], &[(7, 8), (7, 8)]]);
    let (one, two) = ([1u8; 32], [2u8; 32]);
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("winding"));
    e.speak_with(std::slice::from_ref(&e1), &[], &one, |_, _| {});
    e.speak_with(std::slice::from_ref(&e2), &[], &two, |_, _| {});
    assert_eq!(e.dialect_len(), 2, "both stand at once");
    assert_eq!((e.hypha.in_flight(), e.held_for_returns(), e.own_k()), (0, 2, 0), "nothing posted: each waits for its thought's return");
    // Said again before the return, it is the one thing held, never two
    // (MU-CLIENT §4.13: held by its coordinates, never by a count).
    e.speak_with(std::slice::from_ref(&e1), &[], &one, |_, _| {});
    assert_eq!(e.held_for_returns(), 2, "one speech held per sentence, its latest saying");
    let returned = bind_a_thought(&mut e, &one);
    assert!(returned > 0, "a thought of the first scene came round");
    assert_eq!(e.own_k(), returned, "K stepped by exactly the returns");
    assert_eq!(e.hypha.in_flight(), 1, "the first scene's speech posted at its thought's return");
    assert_eq!(e.held_for_returns(), 1, "the second scene's still waits for a thought of its own");
}

/// The scene a task's pairs make: one bar carried a column along.
fn scene() -> (crate::arc::Grid, crate::arc::Grid) {
    (vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0]], vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 0]])
}

fn envelope(id: [u8; 32]) -> crate::anastome::dialogue::TaskEnvelope {
    crate::anastome::dialogue::TaskEnvelope { id, train: vec![scene()], tests: Vec::new() }
}

/// A SCENE WAITING ON COMPREHENSION STANDS WITH ITS THOUGHTS. With no thought
/// perceived in it, it lapses at the mind's return; with one standing and the
/// mind's language unmoved since it was read, it waits as it was — nothing
/// new to comprehend it by.
#[test]
fn a_waiting_scene_lapses_with_its_last_thought() {
    let id = [7u8; 32];
    let mut alone = Entity::open("127.0.0.1:1", Kernel::named("alone"));
    let root = alone.epigenome.atlas_root();
    alone.comprehending.insert(id, (envelope(id), root));
    alone.comprehend_waiting();
    assert!(alone.comprehending.get(&id).is_none(), "no thought stands on it: it lapses with them");

    let mut held = Entity::open("127.0.0.1:1", Kernel::named("held"));
    let (a, b) = scene();
    let frames = crate::transducer::family::perceive_family(&[(a.clone(), b.clone())]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let f = crate::transducer::family::scene_form_on(&held.epigenome, &mut budget, &a, &b, &frames).expect("the scene reads");
    held.contemplation.hold_perceived(f, &frames, &Vec::new(), &[], [0; 4], &id, &mut budget).expect("held");
    assert!(held.contemplation.holds_family(&id), "a thought stands on the scene");
    let root = held.epigenome.atlas_root();
    held.comprehending.insert(id, (envelope(id), root));
    held.comprehend_waiting();
    assert!(held.comprehending.get(&id).is_some(), "it waits beside its thought");
}

/// 78.167 D1 — A MEANING GIVEN WHOLE IS DERIVED ONCE THE WORDS BESIDE IT ARE
/// KNOWN. A screw — a quarter turn carried along its axis — answered as an
/// atom is known as GIVEN. Knowing the turn alone leaves the slide as its
/// residue: still given. Knowing the slide too, it is reduced again at the
/// mind's return and DERIVED, one rung above the two.
#[test]
fn a_given_meaning_is_rederived_once_the_words_beside_it_are_known() {
    use crate::transducer::sentence::rung::{held_rung, learn_by, learn_given, re_reduce_given};
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let quarter = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &crate::transducer::element::identity_versor()).expect("a motion");
    let screw = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &along).expect("a motion");
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("given"));
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        assert!(learn_given(&screw), "the screw's meaning given whole");
        assert!(learn_by(&quarter, &[]), "the turn known");
        assert_eq!(re_reduce_given(), 0, "the turn alone leaves the slide: still given");
        assert!(learn_by(&along, &[]), "the slide known");
        assert_eq!(re_reduce_given(), 1, "turn and slide known: the screw is derived");
        assert_eq!(held_rung(&screw), Some(2), "one rung above the two it is made of");
    });
    mind.field = field;
}
