// MU-CLIENT §5.1, §5.3, §5.5 — TWO MINDS FORM A SHARED WORD ON THEIR OWN
// CARRIERS, and the speaker carries it to the court with the hearer's signed
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
                    if act.agreements.iter().any(Option::is_some) {
                        let _ = tx.send(act);
                    }
                }
            }
        }
    });
    (addr, rx)
}

/// THE SHARED WORD, BY THE ALGEBRA, ON TWO CARRIERS (MU-CLIENT §4.9). Both
/// minds have HEARD the 180° turn before (each remembers its sound), and
/// the hearer KNOWS it as a word (its lexicon holds it); A says it. To B it
/// sounds familiar and is a word it knows — understood — and what it
/// sounded like is a sound A has heard too: MUTUAL. The thing A said is
/// AGREED while the test only waits, and A's carrier speaks it to the court
/// carrying B's HOLD: signed by B, naming exactly the word A said.
#[test]
fn two_minds_form_a_shared_word_and_carry_it_to_the_court() {
    use crate::transducer::sentence::rung::{hear_word, Recognition};
    let (court, speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
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
    field.think(|| {
        assert_eq!(hear_word(&half), Recognition::New, "B comes to know the word");
        assert_eq!(hear_word(&half), Recognition::Recognized, "and knows it after");
    });
    b.field = field;
    let said = word(&[(q, false), (q, false)]);
    a.say(&said);
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

/// MU-CLIENT §4.9 — DOES THIS SOUND LIKE SOMETHING I HAVE HEARD: a ring read
/// against what the mind remembers hearing, never a vocabulary. With nothing heard
/// a 180° turn is a NEW sound, on the outside. Having heard a ring of order
/// three (the turn carrying `e₁ → e₂ → e₃`, `½(1 + e₂e₃ + e₃e₁ + e₁e₂)`), the
/// half turn's period divides nothing larger heard: LEFTOVER, still a new
/// sound. Having heard a slide — the carrier every returning ring sounds
/// inside — the half turn sounds inside it, its echo turned: TILTED, a
/// familiar sound. Having heard the half turn itself, it is WHOLE. A ring
/// heard twice is held once.
#[test]
fn a_ring_sounds_familiar_by_resonance_with_what_was_heard() {
    use crate::transducer::cga::{Mv, E1, E2, E3};
    use crate::transducer::sentence::hearing::Seat;
    let (court, _speeches) = court();
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let half = conformal_of_word(&[(LetterId::turn_90((0, 1)).expect("a quarter turn"), false); 2]);
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
/// word of two quarter turns, asked about, factors into its two letters, and
/// the factors compose back to exactly the word; one quarter turn factors
/// into nothing smaller and comes back whole — an atom.
#[test]
fn a_question_is_answered_by_the_words_factors() {
    use crate::client::sdk::entity::chatter::factors_of;
    use crate::transducer::element::conformal_of;
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let two = conformal_of(&Word::from_letters(&[(q, false), (q, false)]), &identity_versor()).expect("a motion");
    let factors = factors_of(&two, &[], b"word");
    assert_eq!(factors.len(), 2, "two letters: {factors:?}");
    let composed = factors.iter().fold(crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into())), |acc, f| acc.geo(&f.motion));
    assert!(composed.projective_eq(&two), "the factors compose back to the word");
    let one = conformal_of(&Word::from_letters(&[(q, false)]), &identity_versor()).expect("a motion");
    let atom = factors_of(&one, &[(1, 2)], b"word");
    assert!(matches!(atom.as_slice(), [a] if a.motion.projective_eq(&one) && a.gauge == vec![(1, 2)]), "an atom comes back whole: {atom:?}");
}
