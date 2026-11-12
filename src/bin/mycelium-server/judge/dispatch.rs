use super::{challenge, domain, entries, equivocation, fold, handoff, laws, observe, probe, quote, receipts, seat, speech, syllabus, tick, uplink, wallet, why};
use crate::context::Context;
use crate::control::serve_control;
use crate::transport::answer;
use mycelium::anastome::dialogue;
use mycelium::cognition::{outcome_code, Outcome};
use mycelium::network::wire::Delimited;
use mycelium::physics::actions::{Action, Kind};
use mycelium::network::channel::Channel;
use std::sync::atomic::{AtomicU64, Ordering};

static REFUSE_COUNT: AtomicU64 = AtomicU64::new(0);

/// 78.151 — THE READS THAT TAKE NO COURT, answered where the envelope is read:
/// a page of a line or a fold root (`ACT_RECEIPTS`), an account
/// (`ACT_WALLET`) and the core clock (`ACT_LAWS` clock), off the court's
/// published reading; a seat listing, off the relay's own lock; and
/// (78.152b) a peer keeper's pushed ring (`ACT_TICK`), off the keepers' own
/// lock. None waits behind a worker the court holds. `false` where the
/// envelope is none of them: the caller judges it as before.
pub(crate) fn serve_published(stream: &mut Channel, delimited: &Delimited, context: &Context) -> bool {
    let tag = dialogue::act_tag(&delimited.body);
    let served = match tag.and_then(Action::of_wire) {
        Some(Action::Receipts) => {
            receipts::judge(stream, delimited, context);
            true
        }
        // 78.153b — a fold witness's journal page, off the reading.
        Some(Action::Entries) => {
            entries::judge(stream, delimited, context);
            true
        }
        // 78.154 (4) — a stake act is a write: it reaches the court.
        Some(Action::Wallet) if mycelium::anastome::dialogue::wallet::decode_wallet(&delimited.body).is_some() => {
            wallet::judge(stream, delimited, context);
            true
        }
        // 78.154 — READ-LEDGER off the reading; every other domain act writes.
        Some(Action::Domain) => domain::serve_ledger(stream, delimited, context),
        Some(Action::Laws) => {
            let clock = matches!(mycelium::physics::laws::codec::decode_laws_request(&delimited.body), Some(mycelium::physics::laws::codec::LawsRequest::Clock));
            if clock {
                laws::judge(stream, delimited, context);
            }
            clock
        }
        Some(Action::Seat) => seat::serve_listing(stream, delimited, context),
        Some(Action::Tick) => {
            tick::judge(stream, delimited, context);
            true
        }
        _ => false,
    };
    if let (true, Some(t)) = (served, tag) {
        mycelium::observe::note_served(t);
    }
    served
}

pub(crate) fn judge(stream: &mut Channel, delimited: Delimited, context: &Context) {
    if serve_control(stream, &delimited, context) {
        return;
    }
    // 78.94 — THE ENVELOPE IS READ AS AN ACTION OF THE PHYSICS: a byte that
    // names no action, or a court's own act (never asked for), is not
    // answered; a retired action is refused by its kind.
    let Some(action) = dialogue::act_tag(&delimited.body).and_then(Action::of_wire) else { return };
    if action.kind() == Kind::Retired {
        // 26, 50.6 seam 3: raw painted sites and coboundary chains do not
        // reach a court, nor do acts with no handler. Refused with the
        // reason, no height (this court read no book to answer with),
        // nothing taught.
        if delimited.body.len() < 5 {
            return;
        }
        let why = mycelium::verdict::REFUSE_NOT_A_SENTENCE;
        let c = REFUSE_COUNT.fetch_add(1, Ordering::Relaxed);
        if c == 0 || c.is_power_of_two() {
            eprintln!("  court refuse action {}: {why} (count={c})", String::from_utf8_lossy(action.name()));
        }
        let body = dialogue::encode_outcome(outcome_code(&Outcome::Refused(why)), 0, &[0; 4]);
        answer(stream, Delimited { body });
        return;
    }
    match action {
        Action::Speech => speech::judge(stream, &delimited, context),
        Action::Quote => quote::judge(stream, &delimited, context),
        Action::Digests => syllabus::digests(stream, &delimited, context),
        Action::Fetch => syllabus::fetch(stream, &delimited, context),
        Action::Probe => probe::judge(stream, &delimited, context),
        Action::Equivocation => equivocation::judge(stream, &delimited, context),
        Action::Receipts => receipts::judge(stream, &delimited, context),
        Action::Entries => entries::judge(stream, &delimited, context),
        Action::Fold => fold::judge(stream, &delimited, context),
        Action::Observe => observe::judge(stream, &delimited, context),
        Action::Laws => laws::judge(stream, &delimited, context),
        Action::Why => why::judge(stream, &delimited, context),
        Action::Wallet => wallet::judge(stream, &delimited, context),
        Action::Uplink => uplink::judge(stream, &delimited, context),
        Action::Domain => domain::judge(stream, &delimited, context),
        Action::Challenge => challenge::judge(stream, &delimited, context),
        Action::Handoff => handoff::judge(stream, &delimited, context),
        Action::Seat => seat::judge(stream, &delimited, context),
        // Answered where the envelope is read (`serve_published`); here only
        // where a caller judged it unread.
        Action::Tick => tick::judge(stream, &delimited, context),
        // The door's control actions (`serve_control`) answered above; any
        // other action has no judge here and is not answered.
        _ => {}
    }
}
