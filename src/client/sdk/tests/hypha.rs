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
fn speak_word_goes_on_while_court_is_slow() {
    let (court, speeches, release) = held_court();
    let mut mind = Mind::open(&court, Voice::named("unwaiting"));
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let said = from_connection(&Word::from_letters(&[(q, false)]), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection");
    assert!(matches!(mind.speak_word(&said), Ok(Utterance::Spoken)), "said, and the field goes on");
    // MU-CLIENT §4.4(4) — it leaves when the mind's own field comes round.
    assert!(super::common::bind_a_thought(&mut mind, &[]) > 0, "the mind's orbit returned");
    let act = speeches.recv().expect("the speech reached the court");
    assert_eq!(act.voice, Voice::named("unwaiting").id(), "spoken by the mind");
    assert_eq!(mind.hypha.in_flight(), 1, "its answer is in flight");
    mind.take_answers();
    assert_eq!(mind.hypha.in_flight(), 1, "the court has not answered: nothing lands");
    assert!(mind.sealed_at(said.address().as_bytes()).is_none(), "and no verdict is in the mind");
    release.send(()).expect("the court is released");
    mind.settle();
    assert_eq!(mind.hypha.in_flight(), 0, "the answer landed and was taken");
    assert!(mind.sealed_at(said.address().as_bytes()).is_some(), "the court's refusal sealed the word");
}

/// AN OCCLUDED WIRE IS READ AGAIN, NEVER ABANDONED. A mind whose court does
/// not answer fails its exchanges until the wire reads occluded; occluded,
/// it asks the court for nothing — and its next tick still sends one
/// discovery down the wire, so the impedance is read again rather than
/// standing forever on exchanges it no longer makes.
#[test]
fn wind_reads_occluded_wire_again() {
    let mut mind = Mind::open("127.0.0.1:1", Voice::named("occluded"));
    // It holds language, so nothing else makes its language due again.
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let taught = from_connection(&Word::from_letters(&[(q, false)]), &identity_versor(), &[], &Vec::new(), [1, 0, 0, 0]).expect("a connection");
    assert!(mind.absorb_taught(&taught.to_bytes().expect("the record writes")).is_some(), "the word is learned");
    for _ in 0..64 {
        if matches!(mind.transport(), TransportAperture::Occluded { .. }) {
            break;
        }
        mind.discover();
        mind.settle();
    }
    assert!(matches!(mind.transport(), TransportAperture::Occluded { .. }), "a court that never answers occludes the wire");
    let stop = std::sync::atomic::AtomicBool::new(false);
    for n in 0..3 {
        mind.settle();
        let _ = mind.wind(&stop);
        assert!(mind.hypha.in_flight() >= 1, "stroke {n}: the occluded mind still asks the nerve");
    }
}

/// MU-CLIENT §5.1, §6 — A SILENT HOUSE HOLDS ONLY THE SEAT'S CARRIERS. A mind
/// sits a seat in a house that takes every connection and never answers. Its
/// body's stroke returns at once — the take posted on the seat's own carrier,
/// the first page of its mail on the perceiving carrier, which the court holds
/// to its next event and so never stands in front of a take, solve or
/// declination (2026-10-23) — and nothing of it rides the court's carrier.
#[test]
fn body_tick_holds_silent_house_on_seat_carriers() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("the house listens");
    let addr = listener.local_addr().expect("an address").to_string();
    std::thread::spawn(move || {
        let mut held = Vec::new();
        // Taken, read, never answered.
        for s in listener.incoming().flatten() {
            held.push(s);
        }
    });
    let mut mind = Mind::open(&addr, Voice::named("seated"));
    let key = ed25519_dalek::SigningKey::from_bytes(&[7; 32]);
    mind.sit([5; 32], None, crate::client::sdk::entity::body::Sit::Seats(vec![1]), key);
    mind.body_tick();
    assert_eq!(mind.seat.in_flight(), 1, "the take waits on the seat's carrier");
    assert_eq!(mind.held.in_flight(), 1, "the first page waits on the perceiving carrier, apart from the take");
    assert_eq!(mind.hypha.in_flight(), 0, "the court's carrier carries none of it");
    mind.body_tick();
    assert_eq!(mind.held.in_flight(), 1, "a page in flight is not asked again: one reading of the box at a time");
}
