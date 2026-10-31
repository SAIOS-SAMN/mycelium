// 78.92 (2) — ACT_WHY: why one act on this court's ledger stands, answered
// from the kernel's own ledger and the words it holds (`Court::why`).
//
//! A reading: nothing is admitted, no receipt is written and no book moves.
//!
//! THE DOOR NEVER WAITS ON COGNITION FOR IT. Where a worker is free the
//! question is judged there, holding the kernel as every act does. Where the
//! door is full, the question is LEFT for the court stroke (`serve_left`),
//! which answers it under the lock it already holds (`answer_left`), and the
//! next asking of the same question is served off that answer — the
//! observer's two questions, for one act.

use crate::door::Door;
use crate::transport::answer;
use mycelium::anastome::dialogue::why::{decode_why, encode_why_reply, Why, WhyAsk};
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use std::sync::Mutex;

/// The question the door left for the court stroke, the latest in the place
/// of any before it.
static LEFT: Mutex<Option<WhyAsk>> = Mutex::new(None);

/// The stroke's answer to the question left, served to its next asking.
static ANSWERED: Mutex<Option<(WhyAsk, Option<Why>)>> = Mutex::new(None);

pub(crate) fn judge(stream: &mut TcpStream, frame: &Frame, door: &Door) {
    let Some(ask) = decode_why(&frame.body) else { return };
    let why = match crate::held::held!(door.kernel) {
        // Stays on the kernel (78.153b): stamp→word and its Reasons are not
        // on the shelf.
        Ok(mut h) => h.run(|k, _| k.why(ask)),
        Err(_) => return,
    };
    answer(stream, Frame { lane: frame.lane, body: encode_why_reply(why.as_ref()) });
}

/// A WHY THAT REACHED A FULL DOOR, taken without the kernel: served off the
/// stroke's answer where the same question was answered, else left for the
/// next stroke and deferred as every act a full door holds back is. Answers
/// whether the frame was a why.
pub(crate) fn serve_left(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let Some(ask) = decode_why(&frame.body) else { return false };
    let answered = match ANSWERED.lock() {
        Ok(a) => a.as_ref().filter(|(asked, _)| *asked == ask).map(|(_, why)| encode_why_reply(why.as_ref())),
        Err(_) => None,
    };
    if let Some(body) = answered {
        answer(stream, Frame { lane: frame.lane, body });
        return true;
    }
    if let Ok(mut left) = LEFT.lock() {
        *left = Some(ask);
    }
    if let Ok(deficit) = door.mesh.deficit.lock().map(|d| *d) {
        let body = mycelium::anastome::dialogue::encode_deferral(mycelium::cognition::outcome_code(&mycelium::cognition::Outcome::GermPending), &deficit);
        answer(stream, Frame { lane: frame.lane, body });
    }
    true
}

/// THE COURT STROKE'S SIDE: the question the door left is answered under the
/// lock the stroke already holds, for its next asking.
pub(crate) fn answer_left(k: &mycelium::cognition::Court) {
    let Some(ask) = LEFT.lock().ok().and_then(|mut l| l.take()) else { return };
    let why = k.why(ask);
    if let Ok(mut a) = ANSWERED.lock() {
        *a = Some((ask, why));
    }
}
