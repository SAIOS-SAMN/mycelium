// 23.8, 26 — THE DISPATCH: which act a frame is, and which judge it goes to.
//
//! The percept, the gossiped percept, the demand and the routed demand are
//! retired acts (law 8, ruled 2026-09-10): a court hears sentences and
//! nothing else, so a frame carrying one is refused at the door with
//! `REFUSE_NOT_A_SENTENCE` and nothing of it reaches the court. An act
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
pub(crate) mod challenge;
pub(crate) mod handoff;
pub(crate) mod seat;
pub(crate) mod tick;
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

/// 78.151 — THE READS THAT TAKE NO COURT, answered where the frame is read:
/// a page of a line or a fold root (`ACT_RECEIPTS`), an account
/// (`ACT_WALLET`) and the core clock (`ACT_LAWS` clock), off the court's
/// published reading; a seat listing, off the relay's own lock; and
/// (78.152b) a peer keeper's pushed ring (`ACT_TICK`), off the keepers' own
/// lock. None waits behind a worker the court holds. `false` where the
/// frame is none of them: the caller judges it as before.
pub(crate) fn serve_published(stream: &mut TcpStream, frame: &Frame, door: &Door) -> bool {
    let tag = dialogue::act_tag(&frame.body);
    let served = match tag.and_then(Action::of_wire) {
        Some(Action::Receipts) => {
            receipts::judge(stream, frame, door);
            true
        }
        // 78.153b — a fold witness's journal page, off the reading.
        Some(Action::Rows) => {
            rows::judge(stream, frame, door);
            true
        }
        // 78.154 (4) — a stake act is a write: it reaches the court.
        Some(Action::Wallet) if mycelium::anastome::dialogue::wallet::decode_wallet(&frame.body).is_some() => {
            wallet::judge(stream, frame, door);
            true
        }
        // 78.154 — READ-LEDGER off the reading; every other domain act writes.
        Some(Action::Domain) => domain::serve_ledger(stream, frame, door),
        Some(Action::Laws) => {
            let clock = matches!(mycelium::physics::laws::codec::decode_laws_request(&frame.body), Some(mycelium::physics::laws::codec::LawsRequest::Clock));
            if clock {
                laws::judge(stream, frame, door);
            }
            clock
        }
        Some(Action::Seat) => seat::serve_listing(stream, frame, door),
        Some(Action::Tick) => {
            tick::judge(stream, frame, door);
            true
        }
        _ => false,
    };
    if let (true, Some(t)) = (served, tag) {
        mycelium::observe::note_served(t);
    }
    served
}

pub(crate) fn judge(stream: &mut TcpStream, frame: Frame, door: &Door) {
    if serve_control(stream, &frame, door) {
        return;
    }
    // 78.94 — THE FRAME IS READ AS AN ACTION OF THE PHYSICS: a byte that
    // names no action, or a court's own act (never asked for), is not
    // answered; a retired action is refused by its kind.
    let Some(action) = dialogue::act_tag(&frame.body).and_then(Action::of_wire) else { return };
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
        Action::Challenge => challenge::judge(stream, &frame, door),
        Action::Handoff => handoff::judge(stream, &frame, door),
        Action::Seat => seat::judge(stream, &frame, door),
        // Answered where the frame is read (`serve_published`); here only
        // where a caller judged it unread.
        Action::Tick => tick::judge(stream, &frame, door),
        // The door's control actions (`serve_control`) answered above; any
        // other action has no judge here and is not answered.
        _ => {}
    }
}
