// 78.164, 78.145, 78.146 — THE BODY, A BOUNDARY: the seats this mind sits in
// a game's house. Inflow — the mail the house posts to the mind's key: TAKEN,
// PERCEPT, OUTCOME, and the listing of a seat's functions. Outflow — the
// mind's calls: its own call at each beat, and the solve of a task set
// before a seat.
//
//! One mind may sit many seats (barren-realms, 2026-10-09): they share the
//! mind's key and so one mailbox, read once a tick to its end, each piece to
//! the seat it names. A seat lost — a TAKEN refused, a call refused NOT THE
//! HOLDER, a take unanswered a full aperture cycle of ticks — is taken again
//! (78.148). Sit-all (78.158a) sits every seat the domain's listing offers,
//! read again when a listing names another epoch and at every aperture
//! cycle. A cursor on a restarted relay is no progress (v1.4).

use crate::anastome::dialogue::seat::MailRead;
use crate::client::sdk::seat::{seat_fns, MindMail, MindRead, SeatHold, SeatName};
use crate::client::sdk::seat_mind::task::{grid_steps, solve_args, tasks_before, FN_SOLVE};
use crate::client::sdk::seat_mind::Decision;
use crate::client::sdk::*;
use std::sync::atomic::{AtomicBool, Ordering};

/// WHICH SEATS THE BODY SITS: these, or every seat the domain lists.
pub enum Sit {
    Seats(Vec<u32>),
    All,
}

/// ONE SEAT SAT.
struct Seat {
    at: SeatName,
    hold: SeatHold,
    memo: crate::transducer::family::ExtractMemo,
    /// The tasks answered, by citation: answered is an APPLIED outcome.
    answered: crate::network::nerve::Keyed<()>,
    /// The solves in flight, by task: the call's reference and the rule's
    /// address; not posted again until the outcome answers or the take is
    /// posted again.
    solving: crate::network::nerve::Keyed<([u8; 32], Vec<u8>)>,
    /// The solves the house refused, by task: the refused rule's address and
    /// the atlas root it was extracted at.
    refused: crate::network::nerve::Keyed<(crate::transducer::address::Address, Vec<u8>)>,
    /// The standing call: (fn, its arguments in the codec's canonical form,
    /// its outcome applied). A setting that stands is not posted again.
    standing: Option<(u16, Vec<u8>, bool)>,
}

/// THE BODY: the seats sat, the key they are sat by, and the shared box's
/// cursor under the relay's epoch.
pub(crate) struct Body {
    seats: Vec<Seat>,
    key: ed25519_dalek::SigningKey,
    domain: [u8; 32],
    sub_origin: Option<[u8; 32]>,
    all: bool,
    after: u64,
    epoch: Option<u64>,
    listed: Option<u64>,
    relist: bool,
    unlisted: u64,
}

impl Seat {
    fn new(at: SeatName) -> Seat {
        Seat {
            at,
            hold: SeatHold::default(),
            memo: crate::transducer::family::ExtractMemo::new(),
            answered: crate::network::nerve::Keyed::new(),
            solving: crate::network::nerve::Keyed::new(),
            refused: crate::network::nerve::Keyed::new(),
            standing: None,
        }
    }
}

impl Entity {
    /// SIT A GAME'S SEATS: `sit` in `domain` (and its sub-origin, v1.4), by
    /// `key`, this mind's own.
    pub fn sit(&mut self, domain: [u8; 32], sub_origin: Option<[u8; 32]>, sit: Sit, key: ed25519_dalek::SigningKey) {
        let (seats, all) = match sit {
            Sit::Seats(ids) => {
                let mut seats: Vec<Seat> = Vec::new();
                for seat in ids {
                    if !seats.iter().any(|s| s.at.seat == seat) {
                        seats.push(Seat::new(SeatName { domain, sub_origin, seat }));
                    }
                }
                (seats, false)
            }
            Sit::All => (Vec::new(), true),
        };
        self.board.body = Some(Body { seats, key, domain, sub_origin, all, after: 0, epoch: None, listed: None, relist: all, unlisted: 0 });
    }

    /// THE BODY'S TICK: the seats read from the listing where they are due,
    /// each seat taken again where lost, the shared box read to its end and
    /// each piece perceived at its seat, then every task set before a seat
    /// solved and every seat's beat answered by the mind's own call.
    pub(crate) fn body_tick(&mut self, stop: &AtomicBool) {
        let Some(mut body) = self.board.body.take() else { return };
        if body.all {
            if let Some(n) = body.unlisted.checked_add(1) {
                body.unlisted = n;
            }
            if body.relist || body.unlisted >= SeatHold::patience() {
                self.sit_listing(&mut body);
            }
        }
        for seat in &mut body.seats {
            self.take_seat(seat, &body.key);
        }
        let mut beats = vec![0u64; body.seats.len()];
        let mut tasks: Vec<Vec<[u8; 32]>> = vec![Vec::new(); body.seats.len()];
        'pages: for _ in 0..SeatHold::patience() {
            let held_epoch = match body.epoch {
                Some(e) => e,
                None => 0,
            };
            let mail = match self.relay().mail(&body.key, body.after, held_epoch) {
                Ok(Ok(MindRead { epoch, seq, mail })) => {
                    let standing = MailRead { epoch, seq, mail: Vec::new() };
                    body.after = standing.cursor(body.epoch, body.after);
                    body.epoch = Some(epoch);
                    mail
                }
                _ => break,
            };
            if mail.is_empty() {
                break;
            }
            for piece in mail {
                if stop.load(Ordering::SeqCst) {
                    break 'pages;
                }
                let (at, seq) = match &piece {
                    MindMail::Taken(q, t) => (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }, *q),
                    MindMail::Percept(q, p) => (SeatName { domain: p.domain, sub_origin: p.sub_origin, seat: p.seat }, *q),
                    MindMail::Outcome(q, o) => (SeatName { domain: o.domain, sub_origin: o.sub_origin, seat: o.seat }, *q),
                };
                body.after = seq;
                // A piece for a seat this mind does not sit is read past.
                let Some(i) = body.seats.iter().position(|s| s.at == at) else { continue };
                let seen = match (body.seats.get_mut(i), beats.get_mut(i), tasks.get_mut(i)) {
                    (Some(seat), Some(b), Some(t)) => self.read_piece(seat, piece, b, t),
                    _ => None,
                };
                if body.all && seen.is_some() && seen != body.listed {
                    body.relist = true;
                }
            }
        }
        for ((seat, b), t) in body.seats.iter_mut().zip(beats).zip(tasks) {
            for id in t {
                if !seat.answered.contains(&id) && !seat.solving.contains(&id) {
                    self.solve_before(seat, &body.key, &id);
                }
            }
            if b > 0 {
                self.beat(seat, &body.key);
            }
        }
        self.board.body = Some(body);
    }

    /// 78.158a SIT-ALL — the seats the domain's latest listing offers are
    /// sat, and those it no longer offers let go.
    fn sit_listing(&mut self, body: &mut Body) {
        body.unlisted = 0;
        body.relist = false;
        let Ok(Some(listing)) = self.relay().list_seats(body.domain, body.sub_origin) else { return };
        let Some((epoch, listed)) = crate::client::sdk::seat::listed_seats(&listing) else { return };
        let sat: Vec<u32> = body.seats.iter().map(|s| s.at.seat).collect();
        let (take, let_go) = crate::client::sdk::seat::reseat(&sat, &listed);
        body.seats.retain(|s| !let_go.contains(&s.at.seat));
        for seat in take {
            body.seats.push(Seat::new(SeatName { domain: body.domain, sub_origin: body.sub_origin, seat }));
        }
        body.listed = Some(epoch);
    }

    /// THE SEAT TAKEN (again, where it was lost or went unanswered a full
    /// aperture cycle of ticks); the calls of a take let go answer nothing.
    fn take_seat(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey) {
        if !seat.hold.stroke() {
            return;
        }
        seat.solving = crate::network::nerve::Keyed::new();
        if let Ok(Ok(_)) = self.relay().take(seat.at, key) {
            seat.hold.take_posted();
        }
    }

    /// ONE PIECE OF MAIL PERCEIVED AT ITS SEAT: a TAKEN marks the hold (and
    /// the listing is perceived), a PERCEPT is perceived by the mind (its
    /// tasks gathered, its beat counted), an OUTCOME marks the call it
    /// answers. Answers the epoch of a listing it read.
    fn read_piece(&mut self, seat: &mut Seat, piece: MindMail, beats: &mut u64, tasks: &mut Vec<[u8; 32]>) -> Option<u64> {
        match piece {
            MindMail::Taken(_, t) => {
                if (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }) == seat.at && seat.hold.taken(t.ok) {
                    return self.read_listing(seat);
                }
            }
            MindMail::Percept(_, p) => {
                let _ = self.perceive_seat(&p);
                *beats += 1;
                for id in tasks_before(&p) {
                    if !tasks.contains(&id) {
                        tasks.push(id);
                    }
                }
            }
            MindMail::Outcome(_, o) => {
                let answered = self.with_seat_mind(seat.at, |m| m.outcome(&o));
                if let (Some(f), Some((sf, _, applied))) = (answered, seat.standing.as_mut()) {
                    if f == *sf {
                        *applied = o.applied;
                    }
                }
                let solved = seat.solving.scoped().find(|(_, (r, _))| *r == o.call).map(|(k, (_, a))| (k, a.clone()));
                if let Some((id, addr)) = solved {
                    let _ = seat.solving.remove(&id);
                    match o.applied {
                        true => {
                            seat.answered.insert(&id, ());
                            let _ = seat.refused.remove(&id);
                        }
                        false => {
                            seat.refused.insert(&id, (self.epigenome.atlas_root(), addr));
                        }
                    }
                }
                if seat.hold.outcome(o.applied, o.reason) {
                    return self.read_listing(seat);
                }
            }
        }
        None
    }

    /// 78.148 — THE LISTING PERCEIVED: this seat's functions handed to the
    /// mind as its repertoire. Answers the listing's epoch.
    fn read_listing(&mut self, seat: &Seat) -> Option<u64> {
        let Ok(Some(listing)) = self.relay().list_seats(seat.at.domain, seat.at.sub_origin) else { return None };
        let Some((epoch, fns)) = seat_fns(&listing, seat.at.seat) else {
            return crate::client::sdk::seat::listed_seats(&listing).map(|(e, _)| e);
        };
        self.with_seat_mind(seat.at, |m| m.perceive_listing(&fns));
        Some(epoch)
    }

    /// THE MIND'S CALL AT THIS BEAT, where the seat is held: a solve answers
    /// its task and never a beat; a setting that stands applied is not
    /// posted again.
    fn beat(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey) {
        if !seat.hold.held {
            return;
        }
        let Decision::Call(choice) = self.with_seat_mind(seat.at, |m| m.decide()) else { return };
        if choice.fn_id == FN_SOLVE {
            return;
        }
        let mut encoded = Vec::new();
        let encodes = crate::anastome::dialogue::seat::put_args(&mut encoded, &choice.args).is_some();
        if encodes && seat.standing.as_ref().is_some_and(|(f, a, applied)| *applied && *f == choice.fn_id && *a == encoded) {
            return;
        }
        if let Ok((Ok(_), reference)) = self.relay().call(seat.at, key, choice.fn_id, choice.args.clone()) {
            seat.standing = encodes.then_some((choice.fn_id, encoded, false));
            self.with_seat_mind(seat.at, |m| m.called(choice.fn_id, choice.args, reference));
        }
    }

    /// 78.146 — A TASK BEFORE THE SEAT: fetched by its id, its rule extracted
    /// and spoken as the court's tasks are, the tests spoken through it, the
    /// sentence minted at its span and crossed to pairs it never saw, checked
    /// at home on the court-stamped words, then CALLED as fn 1 with the
    /// rule's tape, the outputs and its steps. A rule the house refused is
    /// not posted again until its address changes.
    fn solve_before(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey, id: &[u8; 32]) {
        if !seat.hold.held {
            return;
        }
        let (train, tests) = match self.pull_task_by_id(id) {
            Ok(Some(env)) => (env.train, env.tests),
            Ok(None) => {
                seat.answered.insert(id, ());
                return;
            }
            Err(_) => return,
        };
        let gen = self.epigenome.atlas_root();
        if seat.refused.get(id).is_some_and(|(root, _)| *root == gen) {
            return;
        }
        let frames = match seat.memo.holds(id, &gen) {
            true => crate::transducer::family::Frames::new(),
            false => crate::transducer::family::perceive_family(&train),
        };
        let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(&train, &frames));
        let Some(rule) = crate::transducer::family::extract_rule_cached(&mut seat.memo, id, &train, gen, &mut budget, &self.epigenome, Some(&frames)) else { return };
        let _ = self.speak_form(&rule, &train, &frames, id, &mut budget);
        let outs: Option<Vec<crate::arc::Grid>> = tests.iter().map(|x| crate::transducer::family::speak_rule(&rule, x, &self.epigenome).filter(|s| s.remainder.is_empty()).and_then(|s| s.grid())).collect();
        let Some(outs) = outs else { return };
        let Some(minted) = self.mint_solve(&rule, &train) else { return };
        let Ok(crossed) = crate::client::sdk::rule::crossing(&minted, &train, &self.epigenome) else { return };
        if seat.refused.get(id).is_some_and(|(_, addr)| addr.as_slice() == crossed.address().as_bytes()) {
            return;
        }
        // 78.157 §3a, 78.158a U2 — the home check on the court-stamped words;
        // unread at home goes to the court, which resumes it.
        let resolve = |a: &[u8]| self.epigenome.court_word(a).and_then(|e| e.form().cloned());
        match crate::client::sdk::rule::would_judge_with(&crossed, &train, &resolve, &self.epigenome, &mut budget) {
            Ok(()) | Err(crate::client::sdk::rule::NoRule::Unread) => {}
            Err(_) => return,
        }
        let shapes = |g: &crate::arc::Grid| match g.as_slice() {
            [row, ..] => (g.len(), row.len()),
            [] => (0, 0),
        };
        let steps = match train.as_slice() {
            [(a, b), ..] => match grid_steps(&crossed, shapes(a), shapes(b)) {
                Ok(s) => s,
                Err(_) => Vec::new(),
            },
            [] => Vec::new(),
        };
        let Some(args) = solve_args(&crossed, &outs, &steps) else { return };
        if let Ok((Ok(_), reference)) = self.relay().call(seat.at, key, FN_SOLVE, args.clone()) {
            self.with_seat_mind(seat.at, |m| m.called(FN_SOLVE, args, reference));
            seat.solving.insert(id, (reference, crossed.address().as_bytes().to_vec()));
        }
    }
}
