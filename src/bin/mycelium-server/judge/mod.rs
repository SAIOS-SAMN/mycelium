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
pub(crate) mod uplink;
pub(crate) mod wallet;
pub(crate) mod domain;
pub(crate) mod why;

use crate::door::{serve_control, Door};
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::{outcome_code, Outcome};
use mycelium::network::wire::Frame;
use mycelium::physics::actions::{Action, Kind};
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, Ordering};

static REFUSE_COUNT: AtomicU64 = AtomicU64::new(0);

pub(crate) fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    if serve_control(stream, &frame, door) {
        return;
    }
    // 78.94 — THE FRAME IS READ AS AN ACTION OF THE PHYSICS: a byte that
    // names no action, or a court's own act (never asked for), is not
    // answered; a retired action is refused by its kind.
    let Some(action) = dialogue::act_tag(&frame.body).and_then(Action::of_tag) else { return };
    if action.kind() == Kind::Retired {
        // 26, 50.6 seam 3: raw painted sites and coboundary chains do not
        // reach a court, nor do acts with no handler. Refused with the
        // reason, no height (this court read no book to answer with),
        // nothing taught.
        if frame.body.len() < 5 {
            return;
        }
        let why = mycelium::verdict::REFUSE_NOT_A_SENTENCE;
        let c = REFUSE_COUNT.fetch_add(1, Ordering::Relaxed);
        if c == 0 || c.is_power_of_two() {
            eprintln!("  court refuse action {}: {why} (count={c})", String::from_utf8_lossy(action.name()));
        }
        let body = dialogue::encode_outcome(outcome_code(&Outcome::Refused(why)), 0, &[0; 4]);
        answer(stream, Frame { lane: frame.lane, body });
        return;
    }
    match action {
        Action::Speech => speech::judge(stream, &frame, door),
        Action::Quote => quote::judge(stream, &frame, door),
        Action::Digests => syllabus::digests(stream, &frame, door),
        Action::Fetch => syllabus::fetch(stream, &frame, door),
        Action::Probe => probe::judge(stream, &frame, door),
        Action::Equivocation => equivocation::judge(stream, &frame, door),
        Action::Receipts => receipts::judge(stream, &frame, door),
        Action::Rows => rows::judge(stream, &frame, door),
        Action::Fold => fold::judge(stream, &frame, door),
        Action::Observe => observe::judge(stream, &frame, door),
        Action::Laws => laws::judge(stream, &frame, door),
        Action::Why => why::judge(stream, &frame, door),
        Action::Wallet => wallet::judge(stream, &frame, door),
        Action::Uplink => uplink::judge(stream, &frame, door),
        Action::Domain => domain::judge(stream, &frame, door),
        // The door's control actions (`serve_control`) answered above; any
        // other action has no judge here and is not answered.
        _ => {}
    }
}
