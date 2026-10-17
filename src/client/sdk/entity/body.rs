// 78.164, 78.145, 78.146 — THE BODY, A BOUNDARY: the seats this mind sits in
// a game's house. Inflow — the mail the house posts to the mind's key: TAKEN,
// PERCEPT, OUTCOME, and the listing of a seat's functions. Outflow — the
// mind's calls: its own call at each beat, and the solve of a task set
// before a seat.
//
//! One mind may sit many seats (barren-realms, 2026-10-09): they share the
//! mind's key and so one mailbox, read to its end page by page, each piece to
//! the seat it names. A seat lost — a TAKEN refused, a call refused NOT THE
//! HOLDER, a take unanswered a full aperture cycle of strokes — is taken again
//! (78.148). Sit-all (78.158a) sits every seat the domain's listing offers,
//! read again when a listing names another epoch and at every aperture
//! cycle. A cursor on a restarted relay is no progress (v1.4).
//!
//! MU-CLIENT §5.1, §6 — THE SEAT'S OWN CARRIER: every exchange with the house
//! (a listing, a take, a page of mail, a call) is POSTED on the seat's carrier
//! (`ask_seat`) and answered where it lands (`take_seat_answers`), so a slow
//! house holds that carrier and never the mind. A task before a seat is
//! pulled on the court's carrier the same way. The seat's acts are the
//! mind's, made at its returns (MU-CLIENT §4.4(4)).

use crate::anastome::dialogue::seat::{MailRead, Refusal, SignedPost};
use crate::client::sdk::seat::{seat_fns, MindMail, MindRead, SeatHold, SeatName};
use crate::client::sdk::seat_mind::task::{grid_steps, solve_args, tasks_before, FN_SOLVE};
use crate::client::sdk::seat_mind::Decision;
use crate::client::sdk::*;

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
    /// MU-CLIENT §7.1 — THE TASKS PASSED, by task: the charts its offer stood
    /// on when it coupled to nothing (`offer`). Pulled again only where it
    /// couples; let go where the house no longer sets it before the seat.
    passed: crate::network::nerve::Keyed<Vec<Vec<u8>>>,
    /// The standing call: (fn, its arguments in the codec's canonical form,
    /// its outcome applied). A setting that stands is not posted again.
    standing: Option<(u16, Vec<u8>, bool)>,
}

/// THE BODY: the seats sat, the key they are sat by, and the shared box's
/// cursor under the relay's epoch — with whether a page of it is on the
/// seat's carrier now, and how many pages this reading has asked.
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
    mailing: bool,
    pages: u64,
}

/// A SEAT'S ACT'S COORDINATES, held for the mind's return: the seat (its
/// domain, sub-origin and id), and the task a solve answers — one beat's call
/// a seat, one solve a task, whatever was perceived before the return.
fn seat_act_key(at: &SeatName, task: Option<&[u8; 32]>) -> Vec<u8> {
    let mut k = vec![3u8];
    k.extend_from_slice(&at.domain);
    match &at.sub_origin {
        Some(s) => {
            k.push(1);
            k.extend_from_slice(s);
        }
        None => k.push(0),
    }
    k.extend_from_slice(&at.seat.to_le_bytes());
    if let Some(id) = task {
        k.extend_from_slice(id);
    }
    k
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
            passed: crate::network::nerve::Keyed::new(),
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
        self.board.body = Some(Body {
            seats,
            key,
            domain,
            sub_origin,
            all,
            after: 0,
            epoch: None,
            listed: None,
            relist: all,
            unlisted: 0,
            mailing: false,
            pages: 0,
        });
    }

    /// THE BODY AT ADMISSION: what the seat's carrier landed taken; then the
    /// listing asked where it is due, each seat taken again where lost, and
    /// the shared box's first page asked where no reading of it is in flight.
    /// Nothing here waits on the house.
    pub(crate) fn body_tick(&mut self) {
        self.take_seat_answers();
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
        if !body.mailing {
            body.pages = 0;
            self.ask_mail(&mut body);
        }
        self.board.body = Some(body);
    }

    /// The seat at `at`, where the body still sits it.
    fn with_body_seat(&mut self, at: SeatName, f: impl FnOnce(&mut Seat)) {
        if let Some(seat) = self.board.body.as_mut().and_then(|b| b.seats.iter_mut().find(|s| s.at == at)) {
            f(seat);
        }
    }

    /// THE SEAT'S ACT AT THE MIND'S RETURN: a solve for the task `task` before
    /// the seat, or the call its beat names, made on the seat as it stands
    /// now — nothing where the seat is no longer sat or the task was answered
    /// meanwhile.
    fn seat_act(&mut self, at: SeatName, task: Option<[u8; 32]>) {
        let Some(mut body) = self.board.body.take() else { return };
        let key = body.key.clone();
        if let Some(seat) = body.seats.iter_mut().find(|s| s.at == at) {
            // MU-CLIENT §7.1 — a task passed is pulled again only where its
            // offer now couples to the mind.
            let uncoupled = |id: &[u8; 32]| seat.passed.get(id).is_some_and(|charts| !self.couples(id, charts));
            match task {
                Some(id) if uncoupled(&id) => {}
                Some(id) if !seat.answered.contains(&id) && !seat.solving.contains(&id) && seat.hold.held => {
                    let _ = seat.passed.remove(&id);
                    self.pull_before(at, key, id)
                }
                Some(_) => {}
                None => self.beat(seat, &key),
            }
        }
        self.board.body = Some(body);
    }

    /// 78.158a SIT-ALL — the domain's latest listing asked; where it lands,
    /// the seats it offers are sat and those it no longer offers let go.
    fn sit_listing(&mut self, body: &mut Body) {
        body.unlisted = 0;
        body.relist = false;
        let (domain, sub_origin) = (body.domain, body.sub_origin);
        self.ask_seat(move |s| s.list_seats(domain, sub_origin), |me, read: io::Result<Option<SignedPost>>| {
            let Ok(Some(listing)) = read else { return };
            let Some((epoch, listed)) = crate::client::sdk::seat::listed_seats(&listing) else { return };
            let Some(mut body) = me.board.body.take() else { return };
            let sat: Vec<u32> = body.seats.iter().map(|s| s.at.seat).collect();
            let (take, let_go) = crate::client::sdk::seat::reseat(&sat, &listed);
            body.seats.retain(|s| !let_go.contains(&s.at.seat));
            for seat in take {
                body.seats.push(Seat::new(SeatName { domain: body.domain, sub_origin: body.sub_origin, seat }));
            }
            body.listed = Some(epoch);
            me.board.body = Some(body);
        });
    }

    /// THE SEAT TAKEN (again, where it was lost or went unanswered a full
    /// aperture cycle of strokes); the calls of a take let go answer nothing.
    fn take_seat(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey) {
        if !seat.hold.stroke() {
            return;
        }
        seat.solving = crate::network::nerve::Keyed::new();
        // POSTED FROM THE SENDING: a take on the carrier is a take awaited, its
        // strokes counted against the hold's patience; one the wire or the
        // house refuses is let go where that answer lands, and taken again.
        seat.hold.take_posted();
        let (at, key) = (seat.at, key.clone());
        self.ask_seat(move |s| s.take(at, &key), move |me, taken: io::Result<Result<u64, Refusal>>| {
            if !matches!(taken, Ok(Ok(_))) {
                me.with_body_seat(at, |seat| {
                    if !seat.hold.held {
                        seat.hold.let_go();
                    }
                });
            }
        });
    }

    /// THE SHARED BOX'S NEXT PAGE, asked on the seat's carrier; it is read
    /// where it lands (`read_mail_page`).
    fn ask_mail(&mut self, body: &mut Body) {
        body.mailing = true;
        let held_epoch = match body.epoch {
            Some(e) => e,
            None => 0,
        };
        let (key, after) = (body.key.clone(), body.after);
        self.ask_seat(move |s| s.mail(&key, after, held_epoch), |me, read| me.read_mail_page(read));
    }

    /// A PAGE OF THE SHARED BOX, landed: each piece perceived at the seat it
    /// names, and the next page asked while this reading's patience lasts and
    /// the box gave mail.
    fn read_mail_page(&mut self, read: io::Result<Result<MindRead, Refusal>>) {
        let Some(mut body) = self.board.body.take() else { return };
        body.mailing = false;
        let mail = match read {
            Ok(Ok(MindRead { epoch, seq, mail })) => {
                let standing = MailRead { epoch, seq, mail: Vec::new() };
                body.after = standing.cursor(body.epoch, body.after);
                body.epoch = Some(epoch);
                mail
            }
            _ => {
                self.board.body = Some(body);
                return;
            }
        };
        let more = !mail.is_empty();
        for piece in mail {
            let (at, seq) = match &piece {
                MindMail::Taken(q, t) => (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }, *q),
                MindMail::Percept(q, p) => (SeatName { domain: p.domain, sub_origin: p.sub_origin, seat: p.seat }, *q),
                MindMail::Outcome(q, o) => (SeatName { domain: o.domain, sub_origin: o.sub_origin, seat: o.seat }, *q),
            };
            body.after = seq;
            // A piece for a seat this mind does not sit is read past.
            if let Some(seat) = body.seats.iter_mut().find(|s| s.at == at) {
                self.read_piece(seat, piece);
            }
        }
        body.pages = match body.pages.checked_add(1) {
            Some(n) => n,
            None => body.pages,
        };
        if more && body.pages < SeatHold::patience() {
            self.ask_mail(&mut body);
        }
        self.board.body = Some(body);
    }

    /// ONE PIECE OF MAIL PERCEIVED AT ITS SEAT: a TAKEN marks the hold (and
    /// the listing is asked), a PERCEPT is perceived by the mind — its beat's
    /// call and the tasks before it held for the mind's return (MU-CLIENT
    /// §4.4(4)) — and an OUTCOME marks the call it answers.
    fn read_piece(&mut self, seat: &mut Seat, piece: MindMail) {
        match piece {
            MindMail::Taken(_, t) => {
                if (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }) == seat.at && seat.hold.taken(t.ok) {
                    self.read_listing(seat.at);
                }
            }
            MindMail::Percept(_, p) => {
                let _ = self.perceive_seat(&p);
                let at = seat.at;
                let before = tasks_before(&p);
                // A task passed and no longer before the seat is let go: the
                // house set it aside. A counselled percept states no tasks.
                if !p.records.iter().any(|r| matches!(r, crate::anastome::dialogue::seat::Record::Counsel { .. })) {
                    let gone: Vec<Vec<u8>> = seat.passed.scoped().map(|(k, _)| k).filter(|k| !before.iter().any(|id| id.as_slice() == k.as_slice())).collect();
                    for k in gone {
                        let _ = seat.passed.remove(&k);
                    }
                }
                for id in before {
                    if !seat.answered.contains(&id) && !seat.solving.contains(&id) {
                        self.hold_for_return(&seat_act_key(&at, Some(&id)), &[], move |me| me.seat_act(at, Some(id)));
                    }
                }
                // A seat's call is the mind's own: it leaves at the mind's
                // next return (§6 — seat play from orbits comes after the
                // soak).
                self.hold_for_return(&seat_act_key(&at, None), &[], move |me| me.seat_act(at, None));
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
                    self.read_listing(seat.at);
                }
            }
        }
    }

    /// 78.148 — THE LISTING ASKED: where it lands, this seat's functions are
    /// handed to the mind as its repertoire, and a listing of another epoch
    /// than the body last sat by is read again for sit-all.
    fn read_listing(&mut self, at: SeatName) {
        self.ask_seat(move |s| s.list_seats(at.domain, at.sub_origin), move |me, read: io::Result<Option<SignedPost>>| {
            let Ok(Some(listing)) = read else { return };
            let epoch = match seat_fns(&listing, at.seat) {
                Some((epoch, fns)) => {
                    me.with_seat_mind(at, |m| m.perceive_listing(&fns));
                    Some(epoch)
                }
                None => crate::client::sdk::seat::listed_seats(&listing).map(|(e, _)| e),
            };
            if let Some(body) = me.board.body.as_mut() {
                if body.all && epoch.is_some() && epoch != body.listed {
                    body.relist = true;
                }
            }
        });
    }

    /// THE MIND'S CALL AT THIS BEAT, where the seat is held: a solve answers
    /// its task and never a beat; a setting that stands applied is not posted
    /// again. Posted on the seat's carrier; where it lands, the setting stands
    /// and the seat's mind hears what it called.
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
        let (at, key, fn_id, args) = (seat.at, key.clone(), choice.fn_id, choice.args);
        let posted = args.clone();
        self.ask_seat(move |s| s.call(at, &key, fn_id, posted), move |me, called: io::Result<(Result<u64, Refusal>, [u8; 32])>| {
            if let Ok((Ok(_), reference)) = called {
                me.with_body_seat(at, |seat| seat.standing = encodes.then_some((fn_id, encoded, false)));
                me.with_seat_mind(at, |m| m.called(fn_id, args, reference));
            }
        });
    }

    /// 78.146 — A TASK BEFORE THE SEAT, pulled by its citation on the court's
    /// carrier, and OFFERED where it lands (MU-CLIENT §7.1, `offer`): solved
    /// where it couples to the mind (`solve_before`), passed where it couples
    /// to nothing — its charts kept, so it is pulled again only where they
    /// couple.
    fn pull_before(&mut self, at: SeatName, key: ed25519_dalek::SigningKey, id: [u8; 32]) {
        let voice = self.hypha.kernel().voice();
        let act = dialogue::encode_task_by_id(&voice, &id);
        self.ask(move |s| s.speak_keyed(&id, act), move |me, body: io::Result<Vec<u8>>| {
            let Ok(body) = body else { return };
            if body.is_empty() {
                me.with_body_seat(at, |seat| {
                    seat.answered.insert(id, ());
                });
                return;
            }
            let Some(env) = dialogue::decode_task_envelope(&body) else { return };
            let offer = super::offer::Offer::of(&env.train);
            let couples = me.couples(&id, &offer.charts);
            let Some(mut b) = me.board.body.take() else { return };
            if let Some(seat) = b.seats.iter_mut().find(|s| s.at == at) {
                match couples {
                    true => me.solve_before(seat, &key, &id, env.train, env.tests),
                    false => {
                        seat.passed.insert(id, offer.charts);
                        me.passed(super::offer::PASSED_SEAT);
                    }
                }
            }
            me.board.body = Some(b);
        });
    }

    /// 78.146 — A TASK BEFORE THE SEAT, landed: its rule extracted and spoken
    /// as the court's tasks are, the tests spoken through it, the sentence
    /// minted at its span and crossed to pairs it never saw, checked at home
    /// on the court-stamped words, then CALLED as fn 1 with the rule's tape,
    /// the outputs and its steps, on the seat's carrier. A rule the house
    /// refused is not posted again until its address changes.
    fn solve_before(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey, id: &[u8; 32], train: Vec<crate::transducer::family::TrainPair>, tests: Vec<crate::arc::Grid>) {
        if !seat.hold.held {
            return;
        }
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
        let (at, key, id, addr) = (seat.at, key.clone(), *id, crossed.address().as_bytes().to_vec());
        let posted = args.clone();
        self.ask_seat(move |s| s.call(at, &key, FN_SOLVE, posted), move |me, called: io::Result<(Result<u64, Refusal>, [u8; 32])>| {
            if let Ok((Ok(_), reference)) = called {
                me.with_seat_mind(at, |m| m.called(FN_SOLVE, args, reference));
                me.with_body_seat(at, |seat| {
                    seat.solving.insert(id, (reference, addr));
                });
            }
        });
    }
}
