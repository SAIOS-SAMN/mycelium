// 26, 50.9 — ACT_SPEECH: a voice stands sentences at the sentences' home court and
// is answered an adjudication per sentence.
//
//! The speaker's frame is handed to the court whole (`hear_frame`): every
//! tape decoded there, every sentence heard in the order sent, and the
//! reply one `Adjudication` per sentence — never an opaque byte. An act
//! the door deferred is answered the deferral frame (23.8), which carries
//! the lanes and nothing else.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::{outcome_code, Outcome};
use mycelium::network::wire::Frame;
use std::net::TcpStream;


/// A voice speaks sentences to the court that homes them (26).
pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let court = &door.court;

    let Some(act) = dialogue::decode_speech(&frame.body) else {
        return;
    };
    // THE HEARING IS THE TRANSITION THAT ADMITS: what it docketed, and what
    // the sentences waiting on a word carried, it offers when it returns.
    let (heard, offers, attested) = match crate::held::held!(court) {
        // OWED (78.153b): the hearing mints its strokes per sentence (the
        // lock audit, row 9); this hold's stroke is not yet threaded in.
        Ok(mut h) => h.run(|k, _| {
            let heard = k.hear_frame(&act);
            // MU-CLIENT §5.5 — agreed language: the other mind's signed
            // HOLD verified, its attestation sealed on the word.
            let attested = k.attest_agreements(&act);
            (heard, k.take_offers(), attested)
        }),
        Err(_) => return,
    };
    let short = |b: &[u8]| b.iter().take(4).map(|x| format!("{x:02x}")).collect::<String>();
    for a in &attested {
        mycelium::observe::note_agreement(&act.voice, &a.by, &a.deviation);
        let lanes: Vec<String> = a.deviation.iter().map(|d| d.to_string()).collect();
        eprintln!("  agreed: word {}… spoken by {}… attested by {}… deviation S:G:T:H {}", short(&a.word), short(&act.voice), short(&a.by), lanes.join(":"));
        // MU-CLIENT §5.5 — two constructions, one section, verified here.
        for w in &a.converged {
            eprintln!("  converged: word {}… of {}… and word {}… of {}… close one section on the words' scene", short(&a.word), short(&act.voice), short(w), short(&a.by));
        }
    }
    let body = match heard {
        Ok(adjudicated) => match dialogue::encode_speech_reply(&adjudicated) {
            Some(b) => b,
            None => {
                eprintln!("  speech refused: the reply does not fit its count");
                return;
            }
        },
        Err(lanes) => dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &lanes),
    };
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
    // The voice is answered first; the offers leave after, from this act.
    crate::offer::offer(court, &door.mesh, offers);
}
