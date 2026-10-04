// 23.8, 26 — THE DISPATCH: which act a frame is, and which judge it goes to.
//
//! The percept, the gossiped percept, the demand and the routed demand are
//! retired acts (law 8, ruled 2026-09-10): a court hears sentences and
//! nothing else, so a frame carrying one is refused at the door with
//! `REFUSE_NOT_A_SENTENCE` and nothing of it reaches the kernel. An act
//! this dispatch does not know is not answered.

pub(crate) mod equivocation;
pub(crate) mod fold;
pub(crate) mod laws;
pub(crate) mod observe;
pub(crate) mod offer;
pub(crate) mod probe;
pub(crate) mod quote;
pub(crate) mod receipts;
pub(crate) mod rows;
pub(crate) mod speech;
pub(crate) mod syllabus;
pub(crate) mod wallet;
pub(crate) mod why;

use crate::door::{serve_control, Door};
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::{outcome_code, Outcome};
use mycelium::network::wire::Frame;
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, Ordering};

static REFUSE_COUNT: AtomicU64 = AtomicU64::new(0);

pub(crate) fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    if serve_control(stream, &frame, door) {
        return;
    }
    let Some((&kind, _)) = frame.body.split_first() else { return };
    match kind {
        dialogue::ACT_SPEECH => speech::judge(stream, &frame, door),
        dialogue::ACT_QUOTE => quote::judge(stream, &frame, door),
        dialogue::ACT_DIGESTS => syllabus::digests(stream, &frame, door),
        dialogue::ACT_FETCH => syllabus::fetch(stream, &frame, door),
        dialogue::ACT_PROBE => probe::judge(stream, &frame, door),
        dialogue::ACT_EQUIVOCATION => equivocation::judge(stream, &frame, door),
        dialogue::ACT_RECEIPTS => receipts::judge(stream, &frame, door),
        dialogue::ACT_ROWS => rows::judge(stream, &frame, door),
        dialogue::ACT_FOLD => fold::judge(stream, &frame, door),
        dialogue::ACT_OBSERVE => observe::judge(stream, &frame, door),
        dialogue::ACT_LAWS => laws::judge(stream, &frame, door),
        dialogue::ACT_WHY => why::judge(stream, &frame, door),
        dialogue::ACT_WALLET => wallet::judge(stream, &frame, door),
        dialogue::ACT_PERCEPT | dialogue::ACT_GOSSIP | dialogue::ACT_DEMAND | dialogue::ACT_DEMAND_ROUTED => {
            // 26, 50.6 seam 3: raw painted sites and coboundary chains do
            // not reach a court. Refused with the reason, no height (this
            // court read no book to answer with), nothing taught.
            if frame.body.len() < 5 {
                return;
            }
            let why = mycelium::verdict::REFUSE_NOT_A_SENTENCE;
            let c = REFUSE_COUNT.fetch_add(1, Ordering::Relaxed);
            if c == 0 || c.is_power_of_two() {
                eprintln!("  court refuse act {kind:#04x}: {why} (count={c})");
            }
            let body = dialogue::encode_outcome(outcome_code(&Outcome::Refused(why)), 0, &[0; 4]);
            answer(stream, Frame { lane: frame.lane, body });
        }
        _ => {}
    }
}
