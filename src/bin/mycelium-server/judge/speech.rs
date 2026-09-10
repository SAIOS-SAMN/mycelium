// 26, 50.9 — ACT_SPEECH: a voice stands sentences at their home court and
// is answered an adjudication per sentence.
//
//! The speaker's frame is handed to the kernel whole (`hear_frame`): every
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
    let kernel = &door.kernel;

    let Some(act) = dialogue::decode_speech(&frame.body) else {
        return;
    };
    let heard = match kernel.lock() {
        Ok(mut k) => k.hear_frame(&act),
        Err(_) => return,
    };
    let body = match heard {
        Ok(adjudicated) => dialogue::encode_speech_reply(&adjudicated),
        Err(lanes) => dialogue::encode_deferral(outcome_code(&Outcome::GermPending), &lanes),
    };
    let reply = Frame { lane: frame.lane, body };
    answer(stream, reply);
}
