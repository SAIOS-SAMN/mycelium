// MU-CLIENT §5.1, §5.3, §5.5 — TWO MINDS FORM A SHARED WORD, EACH ON ITS OWN
// CARRIER, and the speaker carries it to the court with the hearer's signed
// agreement.

use crate::anastome::dialogue;
use crate::client::sdk::chatter::{Chatter, Said};
use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::identity_versor;
use crate::transducer::element::residual::from_connection;
use std::io::{Read, Write};

/// A connection word spelled by `letters` at the identity carriage.
fn word(letters: &[(LetterId, bool)]) -> crate::transducer::element::Element {
    from_connection(&Word::from_letters(letters), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection")
}

/// A COURT THAT READS ONE SPEECH: it accepts connections until a speech
/// carrying an agreement arrives, answers each act with an empty speech
/// reply, and hands the agreed speech back.
fn court() -> (String, std::sync::mpsc::Receiver<dialogue::SpeechAct>) {
    court_keeping(|act| act.agreements.iter().any(Option::is_some))
}

/// [`court`], handing back every speech `keep` names.
fn court_keeping(keep: fn(&dialogue::SpeechAct) -> bool) -> (String, std::sync::mpsc::Receiver<dialogue::SpeechAct>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the court listens");
    let addr = listener.local_addr().expect("an address").to_string();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for conn in listener.incoming() {
            let Ok(mut s) = conn else { continue };
            loop {
                let mut head = [0u8; 5];
                if s.read_exact(&mut head).is_err() {
                    break;
                }
                let Some((lane, pointed)) = crate::network::wire::read_lane(head[0]) else { break };
                let len = u32::from_le_bytes([head[1], head[2], head[3], head[4]]) as usize;
                let mut body = vec![0u8; len];
                if s.read_exact(&mut body).is_err() {
                    break;
                }
                let Some((body, _)) = crate::network::wire::split_point(body, pointed) else { break };
                let reply = crate::network::wire::Frame { lane, body: dialogue::encode_speech_reply(&[]).expect("a reply") };
                let _ = s.write_all(&reply.encode().expect("framed"));
                if let Some(act) = dialogue::decode_speech(&body) {
                    if keep(&act) {
                        let _ = tx.send(act);
                    }
                }
            }
        }
    });
    (addr, rx)
}

/// MU-CLIENT §5.3 — A QUESTION IS ANSWERED BY WHAT THE MIND KNOWS, AND THE
/// MEANING DERIVED. B knows the 90° turn of the sensor plane and remembers
/// the sound of a SCREW — that turn carried along the axis it turns about —
/// but does not know it as a word. A says the screw. B posits what it knows:
/// the turn covers the screw's elliptic mode, and the RESIDUE left is the
/// slide along the axis; B asks of that alone. A answers it whole (a slide
/// is one carriage, factored no further). B takes the slide, and the screw's
/// meaning is DERIVED: the turn with the slide, known from here on one rung
/// above both — where before the answer it was a word B had only heard.
#[test]
fn take_in_conversation_derives_meaning_from_posited_residue() {
    use crate::transducer::sentence::rung::{held_rung, known, learn_by};
    let (court, _speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let dir = std::env::temp_dir().join(format!("mycelium-question-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Entity::open(&court, Kernel::named("answerer"));
    let mut b = Entity::open(&court, Kernel::named("asker"));
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let quarter = conformal_of_word(&[(q, false)]);
    let screw = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &along).expect("a motion");
    b.remember_sound(&screw);
    let mut field = std::mem::take(&mut b.field);
    field.think(|| {
        assert!(learn_by(&quarter, &[]), "B knows the turn");
        let posited = crate::transducer::sentence::rung::posit(&screw).expect("the screw's modes read");
        assert!(posited.residue.projective_eq(&along), "what the turn leaves of the screw is the slide alone");
        assert!(matches!(posited.parts.as_slice(), [p] if p.projective_eq(&quarter)), "the turn is what B posits");
        assert!(!known(&screw), "the screw itself B does not know");
    });
    b.field = field;
    for m in [&mut a, &mut b] {
        m.open_chatter(&dir).expect("the post opens");
    }
    let said = from_connection(&Word::from_letters(&[(q, false)]), &along, &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection");
    a.say(&said, &[]);
    // MU-CLIENT §4.4(4) — what A says leaves when A's own field comes round.
    assert!(super::common::bind_a_thought(&mut a, &[]) > 0, "A's orbit returned");
    let derived = |b: &mut Entity| {
        let mut field = std::mem::take(&mut b.field);
        let read = field.think(|| (known(&screw), held_rung(&screw)));
        b.field = field;
        read
    };
    let mut read = (false, None);
    for _ in 0..400_000u32 {
        a.take_in_conversation();
        b.take_in_conversation();
        read = derived(&mut b);
        if read.0 {
            break;
        }
        std::thread::yield_now();
    }
    assert_eq!(read, (true, Some(2)), "the screw's meaning derived from the turn and the slide, a rung above both");
    let _ = std::fs::remove_dir_all(&dir);
}

/// MU-CLIENT §5.4, D4 — A DEMAND IS MET FROM THE DICTIONARY FIRST. A mind
/// that knows the 90° turn meets a demand for the 180° turn — its square —
/// composed whole: nothing constructed, it was a word of the dictionary all
/// along. It meets a demand for a SCREW in part: it posits the turn, only the
/// slide it lacked is constructed, and the screw is known from then on as the
/// turn with the slide, a rung above both.
#[test]
fn compose_or_construct_meets_demand_from_dictionary_first() {
    use crate::transducer::sentence::rung::{held_rung, known, learn_by};
    let (court, _speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let quarter = conformal_of_word(&[(q, false)]);
    let half = conformal_of_word(&[(q, false), (q, false)]);
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let screw = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &along).expect("a motion");
    let mut mind = Entity::open(&court, Kernel::named("composer"));
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        learn_by(&quarter, &[]);
        let chart = crate::transducer::stalk::Invariant::default();
        let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
        assert!(known(&half), "the 180° turn is the 90° turn's square: a word of the dictionary");
        mind.compose_or_construct(&half, Some(&chart), &mut budget);
        assert_eq!(held_rung(&half), None, "composed whole: nothing constructed or held anew");
        assert!(!known(&screw) && !known(&along), "neither the screw nor the slide is known");
        mind.compose_or_construct(&screw, Some(&chart), &mut budget);
        assert!(known(&along), "only the slide it lacked was constructed and is known");
        assert!(known(&screw), "and the screw is known");
        assert_eq!(held_rung(&screw), Some(2), "as the turn with the slide, a rung above both");
    });
    mind.field = field;
}

/// 78.166 1b — THE REMAINDER ASKED AGAIN. A mind that knows the 90° turn in
/// its own lexicon, and holds the slide the court ruled, meets a demand for
/// the turn carried along twice the slide. The lexicon posits the turn and
/// leaves the double slide; that remainder is no primitive — the court's
/// slide composes it — so the mind speaks it citing the slide at exponent 2,
/// constructs nothing, and knows the whole word from then on.
#[test]
fn compose_or_construct_speaks_remainder_ruled_words_compose() {
    use crate::transducer::sentence::rung::{known, learn_by};
    let (court, speeches) = court_keeping(|act| !act.sentences.is_empty());
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let chart = crate::transducer::stalk::Invariant::default();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let twice = crate::transducer::cga::translator_at(&[n(0), n(0), n(2)]);
    let quarter = conformal_of_word(&[(q, false)]);
    let screw = crate::transducer::element::conformal_of(&Word::from_letters(&[(q, false)]), &twice).expect("a motion");
    let slide = crate::transducer::sentence::motion_letter_form(&along, &chart, &[]).expect("the slide is a letter");
    let mut mind = Entity::open(&court, Kernel::named("remainder"));
    assert!(mind.absorb_taught(&slide.to_bytes().expect("the record writes")).is_some(), "the court's slide is learned");
    let mut field = std::mem::take(&mut mind.field);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let knows = field.think(|| {
        learn_by(&quarter, &[]);
        mind.compose_or_construct(&screw, Some(&chart), &mut budget);
        known(&screw)
    });
    mind.field = field;
    assert!(knows, "the whole word is known: the turn with the double slide");
    // MU-CLIENT §4.4(4) — what the mind composed leaves when its field comes round.
    assert!(super::common::bind_a_thought(&mut mind, &[]) > 0, "the mind's orbit returned");
    let mut act = None;
    for _ in 0..4_000_000u32 {
        if let Ok(heard) = speeches.try_recv() {
            act = Some(heard);
            break;
        }
        std::thread::yield_now();
    }
    let act = act.expect("the remainder's composition reaches the court");
    let [tape] = act.sentences.as_slice() else { panic!("one word") };
    let mut p = 0usize;
    let spoken = crate::transducer::element::Element::from_bytes(tape, &mut p).expect("the tape reads");
    let [cite] = spoken.receipts().derivations.as_slice() else { panic!("one derivation") };
    assert!(
        matches!(cite.generators.as_slice(), [(form, _, 2)] if *form == slide.address()),
        "the remainder spoken as the ruled slide at exponent 2: {:?}",
        cite.generators
    );
}

/// 78.166 1c — A CONJUGATE IS THE HELD WORD TRANSPORTED, NEVER A NEW LETTER.
/// The court ruled a letter for the 90° turn of the (0,1) plane. A mind
/// holding it meets a demand for the 90° turn of the (0,3) plane — the same
/// turn in another orientation, one class. It speaks one word citing the held
/// letter with the conjugator that carries it, `g · W · g̃`; the court's replay
/// composes it, and the same cite with the identity for its conjugator — a
/// claim the citation does not make good — is refused.
#[test]
fn compose_or_construct_speaks_conjugate_as_transported_word() {
    let (court, speeches) = court_keeping(|act| !act.sentences.is_empty());
    let chart = crate::transducer::stalk::Invariant::default();
    let q01 = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("a 90° turn"), false)]).rotor();
    let q03 = LetterId::turn_90((0, 3)).expect("a 90° turn");
    let residue = conformal_of_word(&[(q03, false)]);
    let letter = crate::transducer::sentence::generator_form(&q01, &chart, &[]).expect("the turn is a letter");
    let mut mind = Entity::open(&court, Kernel::named("conjugator"));
    assert!(mind.absorb_taught(&letter.to_bytes().expect("the record writes")).is_some(), "the court's letter is learned");
    let mut field = std::mem::take(&mut mind.field);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    field.think(|| mind.compose_or_construct(&residue, Some(&chart), &mut budget));
    mind.field = field;
    // MU-CLIENT §4.4(4) — what the mind composed leaves when its field comes round.
    assert!(super::common::bind_a_thought(&mut mind, &[]) > 0, "the mind's orbit returned");
    let mut act = None;
    for _ in 0..4_000_000u32 {
        if let Ok(heard) = speeches.try_recv() {
            act = Some(heard);
            break;
        }
        std::thread::yield_now();
    }
    let act = act.expect("the conjugate reaches the court");
    let [tape] = act.sentences.as_slice() else { panic!("one word") };
    let mut p = 0usize;
    let spoken = crate::transducer::element::Element::from_bytes(tape, &mut p).expect("the tape reads, its conjugator with it");
    let f = crate::transducer::sentence::from_element_form(&spoken).expect("a form");
    let env = spoken.receipts();
    let [cite] = env.derivations.as_slice() else { panic!("one derivation") };
    assert!(matches!(cite.generators.as_slice(), [(form, _, 1)] if *form == letter.address()), "citing the held letter once");
    assert!(cite.conjugator.is_some(), "with the conjugator that carries it");
    let held = crate::transducer::sentence::from_element_form(&letter).expect("the letter's form");
    let resolve = |a: &[u8]| (a == letter.address().as_bytes()).then(|| held.clone());
    assert!(crate::court::verdict::replay::derivations_compose_at(&f, env, &resolve).is_none(), "the court composes g · W · g̃");
    let mut wrong = env.clone();
    for d in &mut wrong.derivations {
        d.conjugator = Some(crate::transducer::alphabet::Rotor::identity());
    }
    assert!(crate::court::verdict::replay::derivations_compose_at(&f, &wrong, &resolve).is_some(), "a conjugator that does not carry it is refused");
}

/// MU-CLIENT §5.4, D4 — A COMPOSITION OF RULED WORDS IS SPOKEN TO THE COURT.
/// The court ruled a SLIDE, a motion letter; a mind holding it meets a demand
/// for the slide carried twice as far. The ruled words compose it — the slide
/// squared, exact on the whole motion — so nothing is constructed: the mind
/// speaks ONE word whose derivation cites the slide at exponent 2, with no
/// peer to agree it (it mints no dimension). The court's replay composes the
/// citation against the slide it holds and admits it; the same word citing
/// the slide once is refused.
#[test]
fn compose_or_construct_speaks_ruled_composition_to_court() {
    let (court, speeches) = court_keeping(|act| !act.sentences.is_empty());
    let n = |v: i64| num_rational::BigRational::from_integer(v.into());
    let chart = crate::transducer::stalk::Invariant::default();
    let along = crate::transducer::cga::translator_at(&[n(0), n(0), n(1)]);
    let twice = crate::transducer::cga::translator_at(&[n(0), n(0), n(2)]);
    let slide = crate::transducer::sentence::motion_letter_form(&along, &chart, &[]).expect("the slide is a letter");
    let mut mind = Entity::open(&court, Kernel::named("composer"));
    assert!(mind.absorb_taught(&slide.to_bytes().expect("the record writes")).is_some(), "the court's slide is learned");
    let mut field = std::mem::take(&mut mind.field);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    field.think(|| mind.compose_or_construct(&twice, Some(&chart), &mut budget));
    mind.field = field;
    // MU-CLIENT §4.4(4) — what the mind composed leaves when its field comes round.
    assert!(super::common::bind_a_thought(&mut mind, &[]) > 0, "the mind's orbit returned");
    let mut act = None;
    for _ in 0..4_000_000u32 {
        if let Ok(heard) = speeches.try_recv() {
            act = Some(heard);
            break;
        }
        std::thread::yield_now();
    }
    let act = act.expect("the composition reaches the court");
    let [tape] = act.sentences.as_slice() else { panic!("one word") };
    let mut p = 0usize;
    let spoken = crate::transducer::element::Element::from_bytes(tape, &mut p).expect("the tape reads");
    let f = crate::transducer::sentence::from_element_form(&spoken).expect("a form");
    assert!(f.faces.is_empty(), "a word, not a sentence");
    let env = spoken.receipts();
    let [cite] = env.derivations.as_slice() else { panic!("one derivation: {:?}", env.derivations) };
    assert!(
        matches!(cite.generators.as_slice(), [(form, _, 2)] if *form == slide.address()),
        "citing the ruled slide at exponent 2: {:?}",
        cite.generators
    );
    let held = crate::transducer::sentence::from_element_form(&slide).expect("the slide's form");
    let resolve = |a: &[u8]| (a == slide.address().as_bytes()).then(|| held.clone());
    assert!(crate::court::verdict::replay::derivations_compose_at(&f, env, &resolve).is_none(), "the court composes it from the slide it holds");
    let mut forged = env.clone();
    for d in &mut forged.derivations {
        d.generators = d.generators.iter().map(|(f, v, _)| (f.clone(), v.clone(), 1)).collect();
    }
    assert!(crate::court::verdict::replay::derivations_compose_at(&f, &forged, &resolve).is_some(), "citing the slide once composes something else: refused");
}

/// MU-CLIENT §5.3 — A VANTAGE IS WHERE A WORD LANDS IN THE MIND'S OWN HOLES.
/// A mind that knows the 90° turn reads the 180° turn through it (its square)
/// — that turn is its vantage; the turn itself it reads through no other word
/// of its own: no vantage. Two vantages are distinct only where the hearer
/// read through words of its own that are not the speaker's: the same words
/// are one derivation seen twice, and a hearer with none stood nowhere.
#[test]
fn vantage_reads_word_through_minds_own_words() {
    use crate::transducer::sentence::rung::{distinct_vantage, learn_by, vantage};
    let (court, _speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let quarter = conformal_of_word(&[(q, false)]);
    let half = conformal_of_word(&[(q, false), (q, false)]);
    let mut mind = Entity::open(&court, Kernel::named("observer"));
    let mut field = std::mem::take(&mut mind.field);
    field.think(|| {
        assert!(vantage(&half).is_empty(), "knowing nothing, no vantage");
        learn_by(&quarter, &[]);
        let v = vantage(&half);
        assert!(matches!(v.as_slice(), [w] if w.projective_eq(&quarter)), "the 180° turn read through the 90° turn: {v:?}");
        assert!(vantage(&quarter).is_empty(), "the turn itself through no other word of its own");
        assert!(distinct_vantage(&[], &v), "a hearer reading through its own words, the speaker through none: two vantages");
        assert!(!distinct_vantage(&v, &v), "the same words: one derivation seen twice");
        assert!(!distinct_vantage(&v, &[]), "a hearer with no vantage stood nowhere of its own");
    });
    mind.field = field;
}

/// THE SHARED WORD, BY THE ALGEBRA, ON TWO CARRIERS (MU-CLIENT §4.9). Both
/// minds have HEARD the 180° turn before (each remembers its sound), and the
/// hearer KNOWS it from a vantage of its own — the 90° turn it knows, whose
/// square it is — while A offers it from none; A says it. To B it
/// sounds familiar and is a word it knows — understood — and what it
/// sounded like is a sound A has heard too: MUTUAL. The thing A said is
/// AGREED while the test only waits, and A's carrier speaks it to the court
/// carrying B's HOLD: signed by B, naming exactly the word A said.
#[test]
fn say_carries_shared_word_to_court_with_hold() {
    use crate::transducer::sentence::rung::{known, learn_by};
    let (court, speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let dir = std::env::temp_dir().join(format!("mycelium-conversation-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Entity::open(&court, Kernel::named("speaker"));
    let mut b = Entity::open(&court, Kernel::named("hearer"));
    let taught = word(&[(q, false)]);
    let half = conformal_of_word(&[(q, false), (q, false)]);
    for m in [&mut a, &mut b] {
        assert!(m.absorb_taught(&taught.to_bytes().expect("the record writes")).is_some(), "the court's word is learned");
        m.remember_sound(&half);
        m.open_chatter(&dir).expect("the post opens");
    }
    let mut field = std::mem::take(&mut b.field);
    let quarter = conformal_of_word(&[(q, false)]);
    field.think(|| {
        assert!(learn_by(&quarter, &[]), "B knows the 90° turn");
        assert!(known(&half), "and so the 180° turn, its square: B's own vantage");
    });
    b.field = field;
    let said = word(&[(q, false), (q, false)]);
    a.say(&said, &[]);
    // MU-CLIENT §4.4(4) — what A says leaves when A's own field comes round.
    assert!(super::common::bind_a_thought(&mut a, &[]) > 0, "A's orbit returned");
    // Each mind's field prices what its conversation understood, as its tick
    // does at admission; the court hears once both fields converged.
    let mut act = None;
    for _ in 0..4_000_000u32 {
        if let Ok(heard) = speeches.try_recv() {
            act = Some(heard);
            break;
        }
        a.take_in_conversation();
        b.take_in_conversation();
        std::thread::yield_now();
    }
    let act = act.expect("the agreed speech reaches the court");
    let agreed = |m: &Entity| m.chat.as_ref().is_some_and(|c| c.agreed(said.address().as_bytes()));
    assert!(agreed(&a), "A holds what it said as agreed language");
    assert_eq!(act.voice, Kernel::named("speaker").voice(), "spoken by the mind that said it");
    let [tape] = act.sentences.as_slice() else { panic!("one sentence") };
    let mut p = 0usize;
    let spoken = crate::transducer::element::Element::from_bytes(tape, &mut p).expect("the tape reads");
    assert_eq!(spoken.address(), said.address(), "the thing said");
    let [Some(frame)] = act.agreements.as_slice() else { panic!("carrying one agreement") };
    let Some(Chatter { speaker, said: Said::Hold { said: held, deviation, .. }, .. }) = Chatter::open(frame) else { panic!("a signed HOLD") };
    assert_eq!(speaker, Kernel::named("hearer").voice(), "signed by the mind that agreed");
    assert_eq!(held.as_slice(), said.address().as_bytes(), "naming exactly the word");
    let zero = num_rational::BigRational::from_integer(0.into());
    assert!(deviation.iter().all(|d| *d == zero), "heard whole: no strain on any lane: {deviation:?}");
    let _ = std::fs::remove_dir_all(&dir);
}

/// MU-CLIENT §4.9 — DOES THIS SOUND LIKE SOMETHING THE MIND HAS HEARD: a ring read
/// against what the mind remembers hearing, never a vocabulary. With nothing heard
/// a 180° turn is a NEW sound, on the outside. Having heard a ring of order
/// three (the turn carrying `e₁ → e₂ → e₃`, `½(1 + e₂e₃ + e₃e₁ + e₁e₂)`), the
/// 180° turn's period divides nothing larger heard: LEFTOVER, still a new
/// sound. Having heard a slide — the carrier every returning ring sounds
/// inside — the 180° turn sounds inside it, its echo turned: TILTED, a
/// familiar sound. Having heard the 180° turn itself, it is WHOLE. A ring
/// heard twice is held once.
#[test]
fn sound_of_seats_ring_by_resonance_with_heard() {
    use crate::transducer::cga::{Mv, E1, E2, E3};
    use crate::transducer::sentence::hearing::Seat;
    let (court, _speeches) = court();
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let half = conformal_of_word(&[(LetterId::turn_90((0, 1)).expect("a 90° turn"), false); 2]);
    let third = Mv::scalar(q(1, 2))
        .add(&Mv::blade(E2 | E3, q(1, 2)))
        .add(&Mv::blade(E3 | E1, q(1, 2)))
        .add(&Mv::blade(E1 | E2, q(1, 2)));
    let slide = crate::transducer::element::translator_versor(5, 0);
    let sounded = |heard: &[&Mv], ring: &Mv| {
        let mut mind = Entity::open(&court, Kernel::named("listener"));
        for h in heard {
            mind.remember_sound(h);
        }
        mind.sound_of(ring).map(|(e, _)| e.seat)
    };
    assert_eq!(sounded(&[], &half), Some(Seat::Outer), "nothing heard: a new sound");
    assert_eq!(sounded(&[&third], &half), Some(Seat::Leftover), "a larger sound it divides not: still new");
    assert!(matches!(sounded(&[&slide], &half), Some(Seat::Tilted(_))), "inside the carrier, its echo turned: familiar");
    assert_eq!(sounded(&[&half], &half), Some(Seat::Whole), "heard before: whole");
    let mut mind = Entity::open(&court, Kernel::named("listener"));
    mind.remember_sound(&half);
    mind.remember_sound(&half);
    assert_eq!(mind.remembered.scoped().map(|(_, m)| m.len()).sum::<usize>(), 1, "a ring heard twice is held once");
}

/// A word spelled by `letters`, as its whole motion at the identity carriage.
fn conformal_of_word(letters: &[(LetterId, bool)]) -> crate::transducer::cga::Mv {
    crate::transducer::element::conformal_of(&Word::from_letters(letters), &identity_versor()).expect("a motion")
}

/// MU-CLIENT §5.3 — A QUESTION IS ANSWERED BY WHAT THE WORD IS MADE OF. A
/// word of two 90° turns, asked about, factors into its two letters, and
/// the factors compose back to exactly the word; one 90° turn factors
/// into nothing smaller and comes back whole — an atom.
#[test]
fn factors_of_answers_question_by_word_factors() {
    use crate::client::sdk::entity::chatter::factors_of;
    use crate::transducer::element::conformal_of;
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let two = conformal_of(&Word::from_letters(&[(q, false), (q, false)]), &identity_versor()).expect("a motion");
    let factors = factors_of(&two, &[], b"word");
    assert_eq!(factors.len(), 2, "two letters: {factors:?}");
    let composed = factors.iter().fold(crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into())), |acc, f| acc.geo(&f.motion));
    assert!(composed.projective_eq(&two), "the factors compose back to the word");
    let one = conformal_of(&Word::from_letters(&[(q, false)]), &identity_versor()).expect("a motion");
    let atom = factors_of(&one, &[(1, 2)], b"word");
    assert!(matches!(atom.as_slice(), [a] if a.motion.projective_eq(&one) && a.gauge == vec![(1, 2)]), "an atom comes back whole: {atom:?}");
}

/// ONE STROKE PRICES WHAT ITS BUDGET HOLDS (ruled 2026-10-25). One more of
/// the field's work queued than a stroke's budget reads: the stroke prices
/// its budget's worth and leaves the rest waiting, and the mind owes strokes
/// meanwhile; the next stroke prices what is left and owes none for it.
#[test]
fn price_hearings_prices_one_budget_rest_wait() {
    use crate::client::sdk::entity::chatter::FieldWork;
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("pricing"));
    let field = mind.pricing_for_test();
    let reads = crate::physics::Physics::default().stroke().remaining();
    assert!(reads > 0, "a stroke reads something");
    for _ in 0..=reads {
        field.send(FieldWork::Hear { speaker: [1; 32], said: Vec::new(), words: Vec::new(), evidence: None }).expect("queued");
    }
    assert_eq!(mind.price_hearings(), reads, "its budget's worth priced");
    assert!(mind.board.hearings_wait && mind.owes_strokes(), "one left: it waits, and strokes are owed");
    assert_eq!(mind.price_hearings(), 1, "the one left priced");
    assert!(!mind.board.hearings_wait && !mind.owes_strokes(), "nothing waits");
}

/// THE TASK FIRST (ruled 2026-10-26): while a court's task waits on the
/// board, or is asked for, no hearing is priced — the work waits, and
/// strokes are owed; with the pipeline clear, it is priced.
#[test]
fn price_hearings_defers_while_task_waits() {
    use crate::client::sdk::entity::chatter::FieldWork;
    use crate::client::sdk::entity::offer::Offer;
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("task first"));
    let field = mind.pricing_for_test();
    field.send(FieldWork::Hear { speaker: [1; 32], said: Vec::new(), words: Vec::new(), evidence: None }).expect("queued");
    let train = vec![(vec![vec![1u8, 0], vec![0, 0]], vec![vec![0u8, 1], vec![0, 0]])];
    let offer = Offer::of(&train);
    mind.board.court.task = Some((crate::anastome::dialogue::TaskEnvelope { id: [2; 32], train, tests: Vec::new() }, offer));
    assert_eq!(mind.price_hearings(), 0, "a task on the board: no hearing priced");
    assert!(mind.board.hearings_wait && mind.owes_strokes(), "the hearing waits, and strokes are owed");
    mind.board.court.task = None;
    mind.board.court.pulling = true;
    assert_eq!(mind.price_hearings(), 0, "a task asked for: no hearing priced");
    mind.board.court.pulling = false;
    assert_eq!(mind.price_hearings(), 1, "the pipeline clear: the hearing is priced");
    assert!(!mind.board.hearings_wait, "nothing waits");
}

/// A HEARING WAITS AT MOST ACROSS ONE RETURN AT THE DOOR TOO (MU-CLIENT §4.13,
/// ruled 2026-10-26): one the task pipeline held unpriced crosses the first
/// return and lapses unread at the second — the buffer is empty, and nothing
/// is owed for it.
#[test]
fn lapse_unpriced_lapses_hearing_at_second_return() {
    use crate::client::sdk::entity::chatter::FieldWork;
    let mut mind = Entity::open("127.0.0.1:1", Kernel::named("lapsing hearing"));
    let field = mind.pricing_for_test();
    field.send(FieldWork::Hear { speaker: [1; 32], said: Vec::new(), words: Vec::new(), evidence: None }).expect("queued");
    mind.board.court.pulling = true;
    assert_eq!(mind.price_hearings(), 0, "held: the task is asked for");
    assert_eq!(mind.lapse_unpriced(), 0, "the first return: it crosses");
    assert_eq!(mind.board.unpriced_crossed.len(), 1, "waiting across one return");
    assert_eq!(mind.lapse_unpriced(), 1, "the second return: it lapses unread");
    assert!(mind.board.unpriced.is_empty() && mind.board.unpriced_crossed.is_empty(), "the buffer is empty");
    assert_eq!(mind.price_hearings(), 0, "nothing left to price");
    assert!(!mind.board.hearings_wait, "nothing owed for it");
}
