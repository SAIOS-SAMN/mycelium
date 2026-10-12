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

/// THE SHARED WORD, BY THE ALGEBRA, ON TWO CARRIERS. Both minds hold the
/// court-taught quarter turn; A says the half turn. B's carrier re-derives
/// it through B's own generator, A's reads the loop said → heard, B's holds
/// the same loop, and the thing A said is AGREED — while the test only
/// waits. A's carrier then speaks it to the court carrying B's HOLD: signed
/// by B, naming exactly the word A said.
#[test]
fn two_minds_form_a_shared_word_and_carry_it_to_the_court() {
    let (court, speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let dir = std::env::temp_dir().join(format!("mycelium-conversation-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Entity::open(&court, Kernel::named("speaker"));
    let mut b = Entity::open(&court, Kernel::named("hearer"));
    let taught = word(&[(q, false)]);
    for m in [&mut a, &mut b] {
        assert!(m.absorb_taught(&taught.to_bytes().expect("the record writes")).is_some(), "the court's word is learned");
        m.open_chatter(&dir).expect("the post opens");
    }
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
        a.price_hearings();
        b.price_hearings();
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

/// The demands on a mind's plan that carry a residue.
fn demands(m: &Entity) -> usize {
    m.contemplation
        .docket_cites()
        .iter()
        .filter(|c| matches!(c, crate::chronometry::Cite::Demand { residue: Some(_), .. }))
        .count()
}

/// 78.46, 78.114 — A HEARING THAT NEVER RETURNS IS A DEMAND, NOT AN
/// AGREEMENT. The hearer holds a CARRIAGE — a slide, a ring that never
/// returns — beside the quarter turn; A says the quarter turn. It seats in
/// the carrier's layer, and its echo `T·R·T⁻¹·R⁻¹` is itself a slide:
/// parabolic, of no order. The hearing's cell is OPEN, so the hearer HOLDs
/// nothing and the court hears nothing; the echo stands on the hearer's plan
/// as a demand whose residue is the echo — deviation fed to construction.
#[test]
fn an_echo_that_never_returns_is_a_demand_not_an_agreement() {
    use crate::transducer::element::translator_versor;
    let (court, speeches) = court();
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let dir = std::env::temp_dir().join(format!("mycelium-open-hearing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut a = Entity::open(&court, Kernel::named("speaker"));
    let mut b = Entity::open(&court, Kernel::named("hearer"));
    let turn = word(&[(q, false)]);
    let slide = from_connection(&Word::from_letters(&[]), &translator_versor(5, 0), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide");
    assert!(a.absorb_taught(&turn.to_bytes().expect("the record writes")).is_some(), "A learns the turn");
    for w in [&turn, &slide] {
        assert!(b.absorb_taught(&w.to_bytes().expect("the record writes")).is_some(), "B learns the turn and the slide");
    }
    for m in [&mut a, &mut b] {
        m.open_chatter(&dir).expect("the post opens");
    }
    let before = demands(&b);
    a.say(&turn);
    let mut demanded = false;
    for _ in 0..4_000_000u32 {
        a.price_hearings();
        b.price_hearings();
        if demands(&b) > before {
            demanded = true;
            break;
        }
        std::thread::yield_now();
    }
    assert!(demanded, "the open echo stands on the hearer's plan as a demand");
    for _ in 0..100_000u32 {
        a.price_hearings();
        b.price_hearings();
    }
    assert!(speeches.try_recv().is_err(), "an open hearing is no agreement: the court hears nothing");
    assert!(!a.chat.as_ref().is_some_and(|c| c.agreed(turn.address().as_bytes())), "A holds nothing agreed");
    let _ = std::fs::remove_dir_all(&dir);
}
