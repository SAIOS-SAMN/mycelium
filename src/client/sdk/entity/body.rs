// 78.164, 78.145, 78.146 — THE BODY, A BOUNDARY: the seats this mind sits in
// a game's house. Inflow — the mail the house posts to the mind's key: TAKEN,
// PERCEPT, OUTCOME, and the listing of a seat's functions. Outflow — the
// mind's calls: its own call at each beat, and the solve of a task set
// before a seat.
//
//! One mind may sit many seats (barren-realms, 2026-10-09): they share the
//! mind's key and so one mailbox, read to its end page by page, each piece to
//! the seat it names. A SEAT IS TAKEN ONLY WHERE THE HOUSE'S LATEST LISTING
//! NAMES IT, once under that listing (MU-CLIENT §7.1, the operator
//! 2026-10-17): refused or unanswered, it waits for the house's next listing
//! epoch; a call refused NOT THE HOLDER lets it go to be taken again (78.148,
//! the stroke-count retry retired). The listing is read off the mind's own
//! relay at every aperture cycle, and sit-all (78.158a) sits every seat it
//! offers. A cursor on a restarted relay is no progress (v1.4).
//!
//! MU-CLIENT §5.1, §6 — THE SEAT'S OWN CARRIER: every exchange with the house
//! (a listing, a take, a page of mail, a call) is POSTED on the seat's carrier
//! (`ask_seat`) and answered where it lands (`take_seat_answers`), so a slow
//! house holds that carrier and never the mind. A task before a seat is
//! pulled on the court's carrier the same way. The seat's acts are the
//! mind's, made at its returns (MU-CLIENT §4.4(4)).

use crate::anastome::dialogue::seat::{MailRead, Refusal, SignedPost};
use crate::client::sdk::seat::{seat_fns, MindMail, MindRead, SeatHold, SeatName};
use crate::client::sdk::seat_mind::task::{decline_args, grid_steps, solve_args, tasks_before, FN_DECLINE, FN_SOLVE};
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
    /// The epoch of the house's latest listing where it names this seat;
    /// none where the latest listing read names it not, or none is read.
    listed: Option<u64>,
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
    /// THE TASKS READ AND NOT SOLVED, by task: the atlas root the reading
    /// stopped at. Pulled again only once the mind's language moved, as a
    /// waiting court scene rests (2026-10-22: seat n1 pulled one task by its
    /// citation 8 times at one court height, each reading stopping silently).
    rested: crate::network::nerve::Keyed<crate::transducer::address::Address>,
    /// THE TASKS WHOSE PULL READ NOTHING — the court deferred it, or its
    /// reply was no envelope — taken up again at the seat's next mail landing
    /// (the court's next event), let go when the house sets it aside. A
    /// pull lost there waited the house's whole patience (~100 min) before the
    /// task came round again (2026-10-23: court a's door deferring).
    unpulled: crate::network::nerve::Keyed<()>,
    /// The standing call: (fn, its arguments in the codec's canonical form,
    /// its outcome applied). A setting that stands is not posted again.
    standing: Option<(u16, Vec<u8>, bool)>,
    /// MU-CLIENT §7.1 — what the seat met, since the body sat it: a reading
    /// for the observer, never read back by cognition.
    met: SeatMet,
}

/// WHAT ONE SEAT MET (the observer's `now`): the percepts perceived at it,
/// the task records they set before it, the calls the mind posted there
/// (its own and its solves), and the beats it passed — no counsel, and no
/// read call dominating.
#[derive(Clone, Copy, Debug, Default)]
struct SeatMet {
    percepts: u64,
    tasks: u64,
    calls: u64,
    passed: u64,
}

/// One more of a reading; a reading past its carrier stands.
fn met(slot: &mut u64) {
    let _ = crate::physics::metabolism::step_or_refuse(slot, 1);
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
    /// The relay's standing (epoch, last sequence) at the mind's last read:
    /// where it moves, something was filed, and the listing is read again.
    relay_seen: Option<(u64, u64)>,
    pages: u64,
    /// A listing read has landed since the body sat: mail is asked only
    /// after (`ask_mail`).
    listing_read: bool,
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
            listed: None,
            memo: crate::transducer::family::ExtractMemo::new(),
            answered: crate::network::nerve::Keyed::new(),
            solving: crate::network::nerve::Keyed::new(),
            refused: crate::network::nerve::Keyed::new(),
            passed: crate::network::nerve::Keyed::new(),
            rested: crate::network::nerve::Keyed::new(),
            unpulled: crate::network::nerve::Keyed::new(),
            standing: None,
            met: SeatMet::default(),
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
            relist: true,
            unlisted: 0,
            mailing: false,
            relay_seen: None,
            pages: 0,
            listing_read: false,
        });
    }

    /// THE BODY AT ADMISSION: what the seat's carrier landed taken; then the
    /// listing asked where it is due, each seat taken again where lost, and
    /// the shared box's first page asked where no reading of it is in flight.
    /// Nothing here waits on the house.
    pub(crate) fn body_tick(&mut self) {
        self.take_seat_answers();
        let Some(mut body) = self.board.body.take() else { return };
        // The house's listing, read off the mind's own relay: which seats it
        // names, under which epoch — what a take waits on (MU-CLIENT §7.1).
        if let Some(n) = body.unlisted.checked_add(1) {
            body.unlisted = n;
        }
        if body.relist || body.unlisted >= SeatHold::patience() {
            self.sit_listing(&mut body);
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

    /// MU-CLIENT §7.1 — EACH SEAT SAT, AS THE OBSERVER READS IT: whether it is
    /// held, the percepts perceived there, the task records set before it, its
    /// tasks passed uncoupled and standing, the calls posted and the beats
    /// passed. None where the body sits no seat.
    pub(crate) fn seat_lines(&self) -> Vec<String> {
        let Some(body) = self.board.body.as_ref() else { return Vec::new() };
        body.seats
            .iter()
            .map(|s| {
                format!(
                    "  seat {} · held {} · percepts {} · task records {} · tasks passed standing {} · calls posted {} · beats passed {}",
                    s.at.seat,
                    s.hold.held,
                    s.met.percepts,
                    s.met.tasks,
                    s.passed.len(),
                    s.met.calls,
                    s.met.passed
                )
            })
            .collect()
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
            // A task read and not solved rests until the language moves.
            let rests = |id: &[u8; 32]| seat.rested.get(id).is_some_and(|root| *root == self.epigenome.atlas_root());
            match task {
                Some(id) if uncoupled(&id) || rests(&id) => {}
                Some(id) if !seat.answered.contains(&id) && !seat.solving.contains(&id) && seat.hold.held => {
                    let _ = seat.passed.remove(&id);
                    self.pull_before(at, key, id)
                }
                // A TASK BEFORE A SEAT NOT YET HELD waits for the hold, taken
                // up again at a later landing (`unpulled`): a restarted mind
                // reads the house's percept before its re-take is answered,
                // and the task was skipped for good (2026-10-23: n4, percepts 1,
                // held false, nothing pulled).
                Some(id) if !seat.hold.held && !seat.answered.contains(&id) && !seat.solving.contains(&id) => {
                    seat.unpulled.insert(id, ());
                }
                Some(_) => {}
                None => self.beat(seat, &key),
            }
        }
        self.board.body = Some(body);
    }

    /// THE DOMAIN'S LATEST LISTING, asked of the mind's own relay; where it
    /// lands, each seat sat reads whether it names it and under which epoch
    /// (a take waits on it), and — 78.158a SIT-ALL — the seats it offers are
    /// sat and those it no longer offers let go. No listing names no seat.
    fn sit_listing(&mut self, body: &mut Body) {
        body.unlisted = 0;
        body.relist = false;
        let (domain, sub_origin) = (body.domain, body.sub_origin);
        self.ask_seat(move |s| s.list_seats(domain, sub_origin), |me, read: io::Result<Option<SignedPost>>| {
            // A listing that could not be read still opens the mail: the held
            // read is what wakes a resting mind, and it must never be withheld.
            let Ok(read) = read else {
                if let Some(b) = me.board.body.as_mut() {
                    b.listing_read = true;
                }
                return;
            };
            let named = read.as_ref().and_then(crate::client::sdk::seat::listed_seats);
            let Some(mut body) = me.board.body.take() else { return };
            if let (true, Some((_, listed))) = (body.all, named.as_ref()) {
                let sat: Vec<u32> = body.seats.iter().map(|s| s.at.seat).collect();
                let (take, let_go) = crate::client::sdk::seat::reseat(&sat, listed);
                body.seats.retain(|s| !let_go.contains(&s.at.seat));
                for seat in take {
                    body.seats.push(Seat::new(SeatName { domain: body.domain, sub_origin: body.sub_origin, seat }));
                }
            }
            for seat in &mut body.seats {
                seat.listed = named.as_ref().and_then(|(epoch, listed)| listed.contains(&seat.at.seat).then_some(*epoch));
            }
            body.listed = named.map(|(epoch, _)| epoch);
            body.listing_read = true;
            me.board.body = Some(body);
        });
    }

    /// THE SEAT TAKEN, where the house's latest listing names it and no take
    /// was posted or refused under that listing (`SeatHold::may_take`). A take
    /// the wire does not carry, the house refuses, or leaves unanswered waits
    /// for the house's next listing epoch; the calls of a take let go answer
    /// nothing.
    fn take_seat(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey) {
        let Some(epoch) = seat.listed.filter(|_| seat.hold.may_take(seat.listed)) else { return };
        seat.solving = crate::network::nerve::Keyed::new();
        seat.hold.take_posted(epoch);
        let (at, key) = (seat.at, key.clone());
        self.ask_seat(move |s| s.take(at, &key), move |_, _: io::Result<Result<u64, Refusal>>| {});
    }

    /// THE SHARED BOX'S NEXT PAGE, asked on the seat's carrier; it is read
    /// where it lands (`read_mail_page`). A body that sits every seat listed
    /// asks none before its first listing read has landed — whatever it named,
    /// so a listing that names nothing never leaves the mind without the read
    /// that wakes it: its seats stand only then, and
    /// a piece for a seat not yet sat is read past — the cursor beyond it,
    /// never seen again (2026-10-23: a restart read the house's percepts
    /// before its listing and lost them). The listing's landing wakes the
    /// mind, and its next tick asks.
    fn ask_mail(&mut self, body: &mut Body) {
        if body.all && !body.listing_read {
            return;
        }
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
        let (mail, standing_seq) = match read {
            Ok(Ok(MindRead { epoch, seq, mail })) => {
                // MU-CLIENT §7.1 — SOMETHING WAS FILED ON THE RELAY since the
                // mind last read it (another epoch, or a sequence it had not
                // seen): a house may have listed, so the listing is read again
                // now, never after a count of strokes — at rest the court's
                // events are the only strokes there are.
                if body.relay_seen != Some((epoch, seq)) {
                    body.relist = true;
                    body.relay_seen = Some((epoch, seq));
                }
                let standing = MailRead { epoch, seq, mail: Vec::new() };
                body.after = standing.cursor(body.epoch, body.after);
                body.epoch = Some(epoch);
                (mail, seq)
            }
            // A READ THE WIRE DID NOT CARRY IS ASKED AGAIN (2026-10-23): the
            // relay holds a read to the court's next event, which outlasts the
            // transport's deadline; a held read timed out here was dropped,
            // and the mind, blocked with no read outstanding, never saw the
            // percept the house filed next (n4: seat 1 held, percepts 0). The
            // renewal is paced by that deadline, never a spin.
            Err(_) => {
                self.ask_mail(&mut body);
                self.board.body = Some(body);
                return;
            }
            // A refusal names a fault in the read itself (unsigned,
            // malformed): asking again at once would only be refused again.
            Ok(Err(_)) => {
                self.board.body = Some(body);
                return;
            }
        };
        let more = !mail.is_empty();
        // AN EMPTY PAGE READS THE RELAY TO ITS STANDING (2026-10-22): the
        // relay holds a read only where nothing is filed past its cursor, and
        // its sequence counts every post, into any box — so a mind whose box
        // stayed empty kept a cursor below it, every read answered at once,
        // and five minds read ~2 400 times a second, the courts' doors taken
        // and their keepers' rings and digests turned away. Nothing in this
        // box stands past the cursor up to the standing: the cursor moves
        // there, and the next read is held to the court's next event.
        if !more {
            body.after = body.after.max(standing_seq);
        }
        let mut due: Vec<(SeatName, [u8; 32])> = Vec::new();
        for piece in mail {
            let (at, seq) = match &piece {
                MindMail::Taken(q, t) => (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }, *q),
                MindMail::Percept(q, p) => (SeatName { domain: p.domain, sub_origin: p.sub_origin, seat: p.seat }, *q),
                MindMail::Outcome(q, o) => (SeatName { domain: o.domain, sub_origin: o.sub_origin, seat: o.seat }, *q),
            };
            body.after = seq;
            // A piece for a seat this mind does not sit is read past.
            if let Some(seat) = body.seats.iter_mut().find(|s| s.at == at) {
                due.extend(self.read_piece(seat, piece).into_iter().map(|id| (at, id)));
            }
        }
        // A pull that read nothing is asked again at this landing — the
        // court's next event — never on a clock.
        for seat in &body.seats {
            for (k, ()) in seat.unpulled.scoped() {
                if let Ok(id) = <[u8; 32]>::try_from(k.as_slice()) {
                    if !due.contains(&(seat.at, id)) {
                        due.push((seat.at, id));
                    }
                }
            }
        }
        body.pages = match body.pages.checked_add(1) {
            Some(n) => n,
            None => body.pages,
        };
        // ONE READ ALWAYS OUTSTANDING: more mail is read on while this
        // reading's patience lasts; an empty page asks again at once, and the
        // relay holds that read to the court's next event — a post filed (a
        // listing, a take, a percept) or a keeper's reading — so a mind at rest
        // blocks on it and is woken by the house, never by a clock (2026-10-22:
        // a seat that rested with no read outstanding never saw the house list).
        match (more, body.pages < SeatHold::patience()) {
            (true, false) => {}
            (true, true) => self.ask_mail(&mut body),
            (false, _) => {
                body.pages = 0;
                self.ask_mail(&mut body);
            }
        }
        self.board.body = Some(body);
        // THE HOUSE'S PERCEPT IS ITS OWN EVENT (the auditor's ruling,
        // 2026-10-22, option (a)): each task it sets before a seat is taken up
        // where the mail lands, the body back on the board — never held for a
        // return of the mind's, which a resting seat mind (K 0, no feed, no
        // tension) never makes; the targets reached the house and no seat.
        for (at, id) in due {
            self.seat_act(at, Some(id));
        }
    }

    /// ONE PIECE OF MAIL PERCEIVED AT ITS SEAT: a TAKEN marks the hold (and
    /// the listing is asked), a PERCEPT is perceived by the mind — its beat's
    /// call held for the mind's return (MU-CLIENT §4.4(4)), the tasks before
    /// it answered, to be taken up where the mail lands — and an OUTCOME marks
    /// the call it answers.
    fn read_piece(&mut self, seat: &mut Seat, piece: MindMail) -> Vec<[u8; 32]> {
        let mut due = Vec::new();
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
                met(&mut seat.met.percepts);
                if let Ok(n) = u64::try_from(before.len()) {
                    let _ = crate::physics::metabolism::step_or_refuse(&mut seat.met.tasks, n);
                }
                // A task passed and no longer before the seat is let go: the
                // house set it aside. A counselled percept states no tasks.
                if !p.records.iter().any(|r| matches!(r, crate::anastome::dialogue::seat::Record::Counsel { .. })) {
                    let gone: Vec<Vec<u8>> = seat.passed.scoped().map(|(k, _)| k).filter(|k| !before.iter().any(|id| id.as_slice() == k.as_slice())).collect();
                    for k in gone {
                        let _ = seat.passed.remove(&k);
                    }
                    // So is a task rested, or unpulled, and no longer before it.
                    let gone: Vec<Vec<u8>> = seat.rested.scoped().map(|(k, _)| k).filter(|k| !before.iter().any(|id| id.as_slice() == k.as_slice())).collect();
                    for k in gone {
                        let _ = seat.rested.remove(&k);
                    }
                    let gone: Vec<Vec<u8>> = seat.unpulled.scoped().map(|(k, _)| k).filter(|k| !before.iter().any(|id| id.as_slice() == k.as_slice())).collect();
                    for k in gone {
                        let _ = seat.unpulled.remove(&k);
                    }
                }
                due.extend(before.into_iter().filter(|id| !seat.answered.contains(id) && !seat.solving.contains(id)));
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
        due
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
        let choice = match self.with_seat_mind(seat.at, |m| m.decide()) {
            Decision::Call(choice) => choice,
            Decision::NothingToCall => {
                met(&mut seat.met.passed);
                return;
            }
            Decision::Awaiting { .. } => return,
        };
        // The house's petitions are answered by their own acts, never by a
        // beat: a solve, or its declination.
        if choice.fn_id == FN_SOLVE || choice.fn_id == FN_DECLINE {
            return;
        }
        let mut encoded = Vec::new();
        let encodes = crate::anastome::dialogue::seat::put_args(&mut encoded, &choice.args).is_some();
        if encodes && seat.standing.as_ref().is_some_and(|(f, a, applied)| *applied && *f == choice.fn_id && *a == encoded) {
            return;
        }
        let (at, key, fn_id, args) = (seat.at, key.clone(), choice.fn_id, choice.args);
        met(&mut seat.met.calls);
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
            // A pull that read nothing waits for the next landing (`unpulled`).
            let unread = |me: &mut Entity| {
                me.with_body_seat(at, |seat| {
                    seat.unpulled.insert(id, ());
                })
            };
            let Ok(body) = body else { return unread(me) };
            if body.is_empty() {
                me.with_body_seat(at, |seat| {
                    let _ = seat.unpulled.remove(&id);
                    seat.answered.insert(id, ());
                });
                return;
            }
            let Some(env) = dialogue::decode_task_envelope(&body) else { return unread(me) };
            // EVERY TASK THE HOUSE SETS IS EVALUATED (the auditor's ruling,
            // 2026-10-23): a seat's attention is its house's, and a resting
            // seat's field couples to nothing — the coupling gate passed every
            // petition. The petition path itself is the gate (`solve_before`).
            let Some(mut b) = me.board.body.take() else { return };
            if let Some(seat) = b.seats.iter_mut().find(|s| s.at == at) {
                let _ = seat.unpulled.remove(&id);
                me.solve_before(seat, &key, &id, env.train, env.tests);
            }
            me.board.body = Some(b);
        });
    }

    /// 78.146 — A TASK BEFORE THE SEAT, landed: its rule extracted and spoken
    /// as the court's tasks are, the tests spoken through it, the sentence
    /// minted at its span and crossed to pairs it never saw, checked at home
    /// on the court-stamped words, then CALLED as fn 1 with the rule's tape,
    /// the outputs and its steps, on the seat's carrier. A rule the house
    /// refused is not posted again until its address changes. Read on the
    /// task's own ground (the auditor's ruling, 2026-10-18): the tests are
    /// spoken and the sentence minted where the demonstrations name it.
    ///
    /// Where the reading stopped is kept on the task's attribution, and a
    /// task read and not solved rests until the mind's language moves
    /// (`Seat::rested`).
    fn solve_before(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey, id: &[u8; 32], train: Vec<crate::transducer::family::TrainPair>, tests: Vec<crate::arc::Grid>) {
        use super::perceive::{Reading, Stop};
        let ground = crate::arc::ground_of(&train);
        let Some(stop) = crate::arc::on_ground(ground, || self.solve_before_on_ground(seat, key, id, train, tests)) else { return };
        self.note_task(id, Reading::Task, |t| {
            t.taken = true;
            t.stop = Some(stop);
        });
        if stop != Stop::Sent {
            seat.rested.insert(id, self.epigenome.atlas_root());
            // THE DECLINATION (the auditor's ruling, 2026-10-23): a reading
            // stopped short of a solve is said to the house, which frees the
            // seat at once — never left to its patience.
            if let Some(code) = stop.code() {
                let (at, key, args) = (seat.at, key.clone(), decline_args(id, code));
                met(&mut seat.met.calls);
                self.ask_seat(move |s| s.call(at, &key, FN_DECLINE, args), |_, _: io::Result<(Result<u64, Refusal>, [u8; 32])>| {});
            }
        }
    }

    /// Where the reading stopped; `None` where nothing was read (the seat
    /// is not held).
    fn solve_before_on_ground(&mut self, seat: &mut Seat, key: &ed25519_dalek::SigningKey, id: &[u8; 32], train: Vec<crate::transducer::family::TrainPair>, tests: Vec<crate::arc::Grid>) -> Option<super::perceive::Stop> {
        use super::perceive::Stop;
        if !seat.hold.held {
            return None;
        }
        let gen = self.epigenome.atlas_root();
        if seat.refused.get(id).is_some_and(|(root, _)| *root == gen) {
            return Some(Stop::RefusedBefore);
        }
        let frames = match seat.memo.holds(id, &gen) {
            true => crate::transducer::family::Frames::new(),
            false => crate::transducer::family::perceive_family(&train),
        };
        let mut budget = self.stroke_n(crate::transducer::family::extraction_spend(&train, &frames));
        let Some(rule) = crate::transducer::family::extract_rule_cached(&mut seat.memo, id, &train, gen, &mut budget, &self.epigenome, Some(&frames)) else {
            return Some(match budget.remaining() {
                0 => Stop::NoRuleUnread,
                _ => Stop::NoRule,
            });
        };
        let _ = self.speak_form(&rule, &train, &frames, id, &mut budget);
        let outs: Option<Vec<crate::arc::Grid>> = tests.iter().map(|x| crate::transducer::family::speak_rule(&rule, x, &self.epigenome).filter(|s| s.remainder.is_empty()).and_then(|s| s.grid())).collect();
        let Some(outs) = outs else { return Some(Stop::TestsUnspoken) };
        let Some(minted) = self.mint_solve(&rule, &train) else { return Some(Stop::Unminted) };
        let Ok(crossed) = crate::client::sdk::rule::crossing(&minted, &train, &self.epigenome) else { return Some(Stop::Uncrossed) };
        if seat.refused.get(id).is_some_and(|(_, addr)| addr.as_slice() == crossed.address().as_bytes()) {
            return Some(Stop::RefusedBefore);
        }
        // 78.157 §3a, 78.158a U2 — the home check on the court-stamped words;
        // unread at home goes to the court, which resumes it.
        let resolve = |a: &[u8]| self.epigenome.court_word(a).and_then(|e| e.form().cloned());
        match crate::client::sdk::rule::would_judge_with(&crossed, &train, &resolve, &self.epigenome, &mut budget) {
            Ok(()) | Err(crate::client::sdk::rule::NoRule::Unread) => {}
            Err(crate::client::sdk::rule::NoRule::CitesUnheld(_)) => return Some(Stop::CitesUnheld),
            Err(_) => return Some(Stop::Refuted),
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
        let Some(args) = solve_args(&crossed, &outs, &steps) else { return Some(Stop::Unsubmitted) };
        let (at, key, id, addr) = (seat.at, key.clone(), *id, crossed.address().as_bytes().to_vec());
        met(&mut seat.met.calls);
        let posted = args.clone();
        self.ask_seat(move |s| s.call(at, &key, FN_SOLVE, posted), move |me, called: io::Result<(Result<u64, Refusal>, [u8; 32])>| {
            if let Ok((Ok(_), reference)) = called {
                me.with_seat_mind(at, |m| m.called(FN_SOLVE, args, reference));
                me.with_body_seat(at, |seat| {
                    seat.solving.insert(id, (reference, addr));
                });
            }
        });
        Some(Stop::Sent)
    }
}
