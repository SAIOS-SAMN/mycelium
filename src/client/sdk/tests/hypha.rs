// MU-CLIENT §5.1 — A SLOW COURT HOLDS ONLY ITS CARRIER: the field speaks and
// goes on while its court has not answered, and takes the answer where it
// lands.

use crate::anastome::dialogue;
use crate::client::sdk::*;
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::identity_versor;
use crate::transducer::element::residual::from_connection;
use crate::transducer::sentence::{At, RefusalKind};
use std::io::{Read, Write};

/// A COURT THAT ANSWERS ONLY WHEN TOLD: it reads each act, hands the speech
/// it carried to the test, and answers it — refusing every sentence as a
/// malformed complex — once the test releases it.
fn held_court() -> (String, std::sync::mpsc::Receiver<dialogue::SpeechAct>, std::sync::mpsc::Sender<()>) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the court listens");
    let addr = listener.local_addr().expect("an address").to_string();
    let (heard, speeches) = std::sync::mpsc::channel();
    let (release, released) = std::sync::mpsc::channel::<()>();
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
                let refused: Vec<dialogue::Adjudication> = match dialogue::decode_speech(&body) {
                    Some(act) => {
                        let refusals = act
                            .sentences
                            .iter()
                            .filter_map(|tape| crate::transducer::element::Element::from_bytes(tape, &mut 0))
                            .map(|e| dialogue::Adjudication::Refused { kind: RefusalKind::MalformedComplex, at: At::Tape(e.address()), receipt: [9; 32] })
                            .collect();
                        let _ = heard.send(act);
                        if released.recv().is_err() {
                            return;
                        }
                        refusals
                    }
                    None => Vec::new(),
                };
                let reply = crate::network::wire::Frame { lane, body: dialogue::encode_speech_reply(&refused).expect("a reply") };
                let _ = s.write_all(&reply.encode().expect("framed"));
            }
        }
    });
    (addr, speeches, release)
}

/// THE FIELD SPEAKS AND GOES ON. A mind speaks a word to a court that has
/// heard it and not answered: the speech has left the field (the court holds
/// it), the answer is in flight, and nothing of the court's verdict is in
/// the mind. Released, the court refuses it; the mind takes the answer at its
/// next admission and seals the word.
#[test]
fn a_slow_court_holds_only_its_carrier() {
    let (court, speeches, release) = held_court();
    let mut mind = Entity::open(&court, Kernel::named("unwaiting"));
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let said = from_connection(&Word::from_letters(&[(q, false)]), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection");
    assert!(matches!(mind.speak_word(&said), Ok(Utterance::Spoken)), "said, and the field goes on");
    let act = speeches.recv().expect("the speech reached the court");
    assert_eq!(act.voice, Kernel::named("unwaiting").voice(), "spoken by the mind");
    assert_eq!(mind.hypha.in_flight(), 1, "its answer is in flight");
    mind.take_answers();
    assert_eq!(mind.hypha.in_flight(), 1, "the court has not answered: nothing lands");
    assert!(mind.sealed_at(said.address().as_bytes()).is_none(), "and no verdict is in the mind");
    release.send(()).expect("the court is released");
    mind.settle();
    assert_eq!(mind.hypha.in_flight(), 0, "the answer landed and was taken");
    assert!(mind.sealed_at(said.address().as_bytes()).is_some(), "the court's refusal sealed the word");
}
