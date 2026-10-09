// 78.145, 78.146 — THE SEAT ARM: a voice started with `--seat <domain-hex>
// <seat-id>` takes that seat in the game's house, reads its mail each stroke,
// perceives each percept, chooses its own call, and learns from the outcomes.
//
//! THE MIND IN THE GAME (78.146). Each PERCEPT is perceived by the entity's
//! mind at the seat (`Entity::perceive_seat`): every record's non-key
//! arguments held as series over the house's beats, each quantity's change a
//! slide and a scale on its own line's flows, a returning series an orbit
//! with its exact period. Each OUTCOME marks the call it answers; the next
//! percept reads what that call moved. The CALL is the mind's own
//! (`SeatMind::decide`): the function whose read outcomes moved the records
//! the counsel's worth favours at least as far as every other, the counsel
//! only where the mind has read nothing or nothing dominates — and one typed
//! line per beat says which call, whether it followed the counsel, and why.
//!
//! 78.148 — THE SEAT IS HELD AGAIN WHEN IT IS LOST, AND ITS LISTING IS
//! PERCEIVED. A TAKEN refused, or a call refused NOT THE HOLDER (a house
//! restarted, the seat passed), lets the take go and the next stroke takes
//! again; a TAKE no TAKEN answers within a full APERTURE CYCLE of this
//! arm's own strokes (`physics::aperture_cycle`, the stroke count at which
//! a contact reads occluded) is posted again. After the house answers the
//! take — and again when a call is refused NO SUCH FUNCTION — the arm reads
//! the domain's listing and hands the mind its own seat's functions: its
//! repertoire, which with no counsel it calls with arguments of its own
//! (`seat_mind::own`).
//!
//! v1.4 — A SEAT IS NAMED BY (domain, sub-origin, seat): `--seat
//! <domain-hex>[/<sub-origin-hex>] <seat-id>[,…]`, the sub-origin absent for
//! the domain's own universe. The box is read at the relay's standing: a
//! cursor left on a relay that restarted (another epoch, or past its last
//! sequence) is read again from the start, never taken for progress. Each
//! percept's (TPoint, u) locates the seat's sub-instance, and its experienced
//! dilation rides the perceived line.
//!
//! 78.158a — SIT-ALL: `--seat <domain-hex>[/<sub-origin-hex>] all` sits every
//! seat the domain's latest listing offers, read again when a listing names
//! another epoch and at every full aperture cycle of strokes; a seat new to
//! the listing is taken, one gone from it is let go. And the SOLVE is the
//! mind's own call: its reference kept, the mind told it called, the task
//! answered only on an applied outcome, a refused rule not posted again
//! until its address changes; its mint refusal and its home check named.

use mycelium::anastome::dialogue::seat::{Arg, MailRead, Record};
use mycelium::client::sdk::seat::{seat_fns, MindMail, MindRead, SeatHold, SeatName};
use mycelium::client::sdk::seat_mind::task::{grid_steps, solve_args, tasks_before, FN_SOLVE};
use mycelium::client::sdk::seat_mind::Decision;
use mycelium::sdk::Entity;
use std::sync::atomic::{AtomicBool, Ordering};

/// THE SEAT THIS VOICE SITS.
pub(crate) struct SeatArm {
    /// v1.4 — the seat's name: (domain, sub-origin, seat).
    at: SeatName,
    key: ed25519_dalek::SigningKey,
    /// 78.148 — the take posted, the house's answer, the strokes waited.
    hold: SeatHold,
    /// The last sequence read: the next read acknowledges it.
    after: u64,
    percepts: u64,
    calls: u64,
    /// The last task's extraction, as the task arm memoises its own.
    memo: mycelium::transducer::family::ExtractMemo,
    /// The tasks this seat has answered, by citation: a house repeats a task
    /// in every percept while it stands before the seat, and it is answered
    /// once. 78.158a U4 — answered is an APPLIED outcome, never a post.
    answered: mycelium::network::nerve::Keyed<()>,
    /// 78.158a U4 — THE SOLVES IN FLIGHT, by task citation: the call's
    /// reference (what the house's outcome names) and the rule's address. A
    /// task in flight is not posted again until its outcome comes, or the
    /// take is posted again (the calls of a lost take are answered by none).
    solving: mycelium::network::nerve::Keyed<([u8; 32], Vec<u8>)>,
    /// 78.158a U4 — THE SOLVES THE HOUSE REFUSED, by task citation: the
    /// refused rule's address and the atlas root it was extracted at. The
    /// same rule is not posted again; it is answered again when the rule's
    /// address changes, as `Entity::solve_refused` keeps the task path's.
    refused: mycelium::network::nerve::Keyed<(mycelium::transducer::address::Address, Vec<u8>)>,
    /// THE STANDING CALL: the last call posted, as (fn, its arguments in
    /// the seat codec's own canonical form), and whether its outcome came
    /// back applied. A decision equal to a standing applied call is not
    /// posted again — the setting stands (xylarium, 2026-10-09: 2 209
    /// applied FOCUS calls in 27 minutes, the same value again and again).
    standing: Option<(u16, Vec<u8>, bool)>,
}

/// THE MAIL A STROKE READS, in pages: to the end of the box, at most a full
/// aperture cycle of pages (`SeatHold::patience`). One page a stroke left a
/// mind behind a house that posts a percept a cycle: its TAKEN buried, the
/// seat re-taken every few seconds, and its task percepts dropped by the
/// box's bound before they were read (2026-10-09).
fn pages() -> u64 {
    SeatHold::patience()
}

/// ONE MIND, MANY SEATS (barren-realms, 2026-10-09: one key powering the
/// ancients' and progenitors' 24 seats, each its own player). The seats share
/// the mind's one key, so they share one mailbox: it is read ONCE a stroke and
/// each piece goes to the seat it names (domain and seat are on every piece).
/// Each seat keeps its own take, its own tasks answered, its own standing
/// call, and makes at most one call a stroke.
pub(crate) struct SeatArms {
    arms: Vec<SeatArm>,
    key: ed25519_dalek::SigningKey,
    /// The last sequence read from the shared box.
    after: u64,
    /// v1.4 — the relay's epoch the cursor was read under: another epoch is
    /// a restarted relay, and the cursor is read again from the start.
    epoch: Option<u64>,
    /// 78.158a SIT-ALL — `--seat <domain>[/<sub>] all`: the domain (and
    /// sub-origin) whose every listed seat this mind sits; `None` for an
    /// explicit id list.
    all: Option<([u8; 32], Option<[u8; 32]>)>,
    /// The listing's epoch the seats sat were read from, and whether a
    /// listing read since names another (the seats are read again).
    listed: Option<u64>,
    relist: bool,
    /// The strokes since the listing was last read for the seats: read
    /// again at a full aperture cycle (`SeatHold::patience`, a stroke count,
    /// never a clock), since a held seat reads its listing only after a take
    /// and a house that adds a world posts nothing else a mind reads.
    unlisted: u64,
}

impl SeatArms {
    /// `--seat <domain-hex>[/<sub-origin-hex>] <seat-id>[,<seat-id>…]`, or
    /// `--seat <domain-hex>[/<sub-origin-hex>] all` (78.158a: every seat the
    /// domain's latest listing offers), or no arm.
    pub(crate) fn of_args(args: &[String], key: ed25519_dalek::SigningKey) -> Result<Option<SeatArms>, String> {
        let Some(at) = args.iter().position(|a| a == "--seat") else { return Ok(None) };
        let (Some(d), Some(s)) = (args.get(at + 1), args.get(at + 2)) else {
            return Err("--seat takes <domain-hex>[/<sub-origin-hex>] <seat-id>[,<seat-id>…] | all".to_string());
        };
        let (d, sub) = match d.split_once('/') {
            Some((d, sub)) => (d, Some(sub)),
            None => (d.as_str(), None),
        };
        let domain = mycelium::court::identity::parse_hex32(d).ok_or_else(|| format!("--seat: {d} is not a 32-byte hex domain"))?;
        let sub_origin = match sub {
            Some(x) => Some(mycelium::court::identity::parse_hex32(x).ok_or_else(|| format!("--seat: {x} is not a 32-byte hex sub-origin"))?),
            None => None,
        };
        let mut arms = Vec::new();
        if s == "all" {
            return Ok(Some(SeatArms { arms, key, after: 0, epoch: None, all: Some((domain, sub_origin)), listed: None, relist: true, unlisted: 0 }));
        }
        for one in s.split(',') {
            let seat = one.trim().parse::<u32>().map_err(|e| format!("--seat: {one} is not a seat id: {e}"))?;
            if !arms.iter().any(|a: &SeatArm| a.at.seat == seat) {
                arms.push(SeatArm::new(SeatName { domain, sub_origin, seat }, key.clone()));
            }
        }
        Ok(Some(SeatArms { arms, key, after: 0, epoch: None, all: None, listed: None, relist: false, unlisted: 0 }))
    }

    /// 78.158a SIT-ALL — THE SEATS SAT, READ FROM THE LISTING: every seat
    /// the domain's latest listing offers is sat (a new arm, which takes it
    /// this stroke), and every seat sat that the listing no longer offers is
    /// let go (its arm dropped: its mail is read past). Read at the first
    /// stroke, when a listing an arm read since names another epoch, and at
    /// every full aperture cycle of strokes. One typed line where the
    /// listing's epoch or the seats sat change.
    fn sit_listing(&mut self, entity: &mut Entity, tag: &str) {
        let Some((domain, sub_origin)) = self.all else { return };
        self.unlisted = 0;
        let listing = match entity.relay().list_seats(domain, sub_origin) {
            Ok(Some(s)) => s,
            Ok(None) => {
                if self.relist {
                    eprintln!("[{tag}] seat all: no listing held for the domain — read again next stroke");
                }
                self.relist = false;
                return;
            }
            Err(e) => {
                if self.relist {
                    eprintln!("[{tag}] seat all: listing unread: {e} — read again next stroke");
                }
                self.relist = false;
                return;
            }
        };
        self.relist = false;
        let Some((epoch, listed)) = mycelium::client::sdk::seat::listed_seats(&listing) else {
            return eprintln!("[{tag}] seat all: the listing is no SEATS");
        };
        let sat: Vec<u32> = self.arms.iter().map(|a| a.at.seat).collect();
        let (take, let_go) = mycelium::client::sdk::seat::reseat(&sat, &listed);
        self.arms.retain(|a| !let_go.contains(&a.at.seat));
        for seat in &take {
            self.arms.push(SeatArm::new(SeatName { domain, sub_origin, seat: *seat }, self.key.clone()));
        }
        if self.listed != Some(epoch) || !take.is_empty() || !let_go.is_empty() {
            eprintln!(
                "[{tag}] seat all: listing epoch {epoch} offers {} seats — sitting {}, taken {take:?}, let go {let_go:?}",
                listed.len(),
                self.arms.len()
            );
        }
        self.listed = Some(epoch);
    }

    /// ONE STROKE AT EVERY SEAT: each seat takes (again, where lost); the
    /// shared box is read to its end once; each piece goes to its seat; then
    /// each seat answers its tasks and makes its one call.
    pub(crate) fn stroke(&mut self, entity: &mut Entity, tag: &str, stop: &AtomicBool) {
        // 78.158a SIT-ALL — the seats read from the listing first: at the
        // first stroke, when a listing read since names another epoch, and
        // at a full aperture cycle of strokes.
        if self.all.is_some() {
            if let Some(n) = self.unlisted.checked_add(1) {
                self.unlisted = n;
            }
            if self.relist || self.unlisted >= SeatHold::patience() {
                self.sit_listing(entity, tag);
            }
        }
        for arm in &mut self.arms {
            arm.take_step(entity, tag);
        }
        let mut beats = vec![0u64; self.arms.len()];
        let mut tasks: Vec<Vec<[u8; 32]>> = vec![Vec::new(); self.arms.len()];
        for _ in 0..pages() {
            // 0 is the epoch a reader names where it holds none.
            let held_epoch = self.epoch.map_or(0, |e| e);
            let mail = match entity.relay().mail(&self.key, self.after, held_epoch) {
                Ok(Ok(MindRead { epoch, seq, mail: m })) => {
                    // v1.4 — a cursor on a restarted relay is no progress:
                    // what it read was a whole box from the start, and the
                    // cursor stands where that read leaves it.
                    let standing = MailRead { epoch, seq, mail: Vec::new() };
                    let cursor = standing.cursor(self.epoch, self.after);
                    if cursor != self.after {
                        eprintln!("[{tag}] seat mail: the relay restarted under the cursor (epoch {epoch:016x}, seq {seq}, cursor {}) — read from the start", self.after);
                        self.after = cursor;
                    }
                    self.epoch = Some(epoch);
                    m
                }
                Ok(Err(why)) => {
                    eprintln!("[{tag}] seat mail refused: {why:?}");
                    break;
                }
                Err(e) => {
                    eprintln!("[{tag}] seat mail unread: {e}");
                    break;
                }
            };
            if mail.is_empty() {
                break;
            }
            for piece in mail {
                if stop.load(Ordering::SeqCst) {
                    return;
                }
                let (at, seq) = match &piece {
                    MindMail::Taken(q, t) => (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }, *q),
                    MindMail::Percept(q, p) => (SeatName { domain: p.domain, sub_origin: p.sub_origin, seat: p.seat }, *q),
                    MindMail::Outcome(q, o) => (SeatName { domain: o.domain, sub_origin: o.sub_origin, seat: o.seat }, *q),
                };
                self.after = seq;
                // A piece for a seat this mind does not sit is read past.
                let Some(i) = self.arms.iter().position(|a| a.at == at) else { continue };
                let seen = match (self.arms.get_mut(i), beats.get_mut(i), tasks.get_mut(i)) {
                    (Some(arm), Some(b), Some(t)) => arm.read_piece(entity, tag, piece, b, t),
                    _ => None,
                };
                // 78.158a SIT-ALL — a listing an arm read at another epoch
                // than the seats were sat from: the seats are read again.
                if self.all.is_some() && seen.is_some() && seen != self.listed {
                    self.relist = true;
                }
            }
        }
        for ((arm, b), t) in self.arms.iter_mut().zip(beats).zip(tasks) {
            for id in t {
                if !arm.answered.contains(&id) && !arm.solving.contains(&id) {
                    arm.solve_before(entity, tag, &id);
                }
            }
            if b > 0 {
                arm.beat(entity, tag);
            }
        }
    }
}

impl SeatArm {
    fn new(at: SeatName, key: ed25519_dalek::SigningKey) -> SeatArm {
        SeatArm {
            at,
            key,
            hold: SeatHold::default(),
            after: 0,
            percepts: 0,
            calls: 0,
            memo: mycelium::transducer::family::ExtractMemo::new(),
            answered: mycelium::network::nerve::Keyed::new(),
            solving: mycelium::network::nerve::Keyed::new(),
            refused: mycelium::network::nerve::Keyed::new(),
            standing: None,
        }
    }

    /// THE SEAT TAKEN (again, where it was lost or went unanswered a full
    /// aperture cycle of strokes).
    fn take_step(&mut self, entity: &mut Entity, tag: &str) {
        let was_posted = self.hold.posted;
        if self.hold.stroke() {
            if was_posted {
                eprintln!("[{tag}] seat {} take let go: no TAKEN within an aperture cycle of {} strokes — taken again", self.at.seat, SeatHold::patience());
            }
            // The calls of a take let go are answered by none: their solves
            // are no longer in flight (78.158a U4).
            self.solving = mycelium::network::nerve::Keyed::new();
            match entity.relay().take(self.at, &self.key) {
                Ok(Ok(seq)) => {
                    self.hold.take_posted();
                    eprintln!("[{tag}] seat {} take posted (seq {seq})", self.at.seat);
                }
                Ok(Err(why)) => eprintln!("[{tag}] seat {} take refused: {why:?}", self.at.seat),
                Err(e) => eprintln!("[{tag}] seat {} take unsent: {e}", self.at.seat),
            }
        }
    }

    /// ONE PIECE OF MAIL READ: a TAKEN marks the hold, a PERCEPT is perceived
    /// (its tasks gathered, its beat counted), an OUTCOME marks its call.
    /// Returns the epoch of a listing it read (78.158a sit-all).
    fn read_piece(&mut self, entity: &mut Entity, tag: &str, piece: MindMail, beats: &mut u64, tasks: &mut Vec<[u8; 32]>) -> Option<u64> {
        {
            match piece {
                MindMail::Taken(seq, t) => {
                    self.after = seq;
                    eprintln!("[{tag}] seat {} taken {} (reason {})", t.seat, t.ok, t.reason);
                    if (SeatName { domain: t.domain, sub_origin: t.sub_origin, seat: t.seat }) != self.at {
                        return None;
                    }
                    match self.hold.taken(t.ok) {
                        true => return self.read_listing(entity, tag),
                        false => eprintln!("[{tag}] seat {} take let go: TAKEN refused (reason {}) — taken again next stroke", self.at.seat, t.reason),
                    }
                }
                MindMail::Percept(seq, p) => {
                    self.after = seq;
                    self.percepts += 1;
                    eprintln!("[{tag}] seat percept {} at {:?}: {}", p.seq, p.local_time, digest(&p.records));
                    let reading = entity.perceive_seat(&p);
                    eprintln!("[{tag}] seat perceived {reading}");
                    // 78.146 — every percept is a beat; a task before the seat
                    // is answered as well (once, after the box is read). A
                    // house that cites its standing task on every percept
                    // left the mind no beat at all, so it made no call of its
                    // own (barren-realms, 2026-10-09).
                    let before = tasks_before(&p);
                    *beats += 1;
                    for id in before {
                        if !tasks.contains(&id) {
                            tasks.push(id);
                        }
                    }
                }
                MindMail::Outcome(seq, o) => {
                    self.after = seq;
                    let answered = entity.with_seat_mind(self.at, |m| m.outcome(&o));
                    // The standing call's outcome: applied, it stands.
                    if let (Some(f), Some((sf, _, applied))) = (answered, self.standing.as_mut()) {
                        if f == *sf {
                            *applied = o.applied;
                        }
                    }
                    eprintln!(
                        "[{tag}] seat outcome applied {} reason {} for fn {answered:?} | percepts {} calls {}",
                        o.applied, o.reason, self.percepts, self.calls
                    );
                    // 78.158a U4 — A SOLVE'S OUTCOME: applied, the task is
                    // answered; refused, the rule is remembered against it
                    // and the task is answered again when the rule changes.
                    let solved = self.solving.scoped().find(|(_, (r, _))| *r == o.call).map(|(k, (_, a))| (k, a.clone()));
                    if let Some((id, addr)) = solved {
                        let _ = self.solving.remove(&id);
                        match o.applied {
                            true => {
                                self.answered.insert(&id, ());
                                let _ = self.refused.remove(&id);
                                eprintln!("[{tag}] seat task {} solve applied: answered", crate::speak::fam_id(&id));
                            }
                            false => {
                                self.refused.insert(&id, (entity.epigenome.atlas_root(), addr));
                                eprintln!(
                                    "[{tag}] seat task {} solve refused (reason {}): answered again when the rule's address changes",
                                    crate::speak::fam_id(&id),
                                    o.reason
                                );
                            }
                        }
                    }
                    // 78.148 — a refusal is information: the seat lost, or
                    // the listing moved.
                    let held = self.hold.held;
                    if self.hold.outcome(o.applied, o.reason) {
                        return self.read_listing(entity, tag);
                    } else if held && !self.hold.held {
                        eprintln!("[{tag}] seat {} take let go: a call refused NOT THE HOLDER — taken again next stroke", self.at.seat);
                    }
                }
            }
        }
        None
    }

    /// 78.148 — THE LISTING PERCEIVED: the domain's latest SEATS read, and
    /// this seat's functions — id and argument tags — handed to the mind as
    /// its repertoire. One typed line. Returns the listing's epoch where it
    /// was read and is SEATS (78.158a: a listing naming no seat here is still
    /// an epoch read — the seat may be gone from it).
    fn read_listing(&mut self, entity: &mut Entity, tag: &str) -> Option<u64> {
        let listing = match entity.relay().list_seats(self.at.domain, self.at.sub_origin) {
            Ok(Some(s)) => s,
            Ok(None) => {
                eprintln!("[{tag}] seat listing: none held for the domain");
                return None;
            }
            Err(e) => {
                eprintln!("[{tag}] seat listing unread: {e}");
                return None;
            }
        };
        let Some((epoch, fns)) = seat_fns(&listing, self.at.seat) else {
            eprintln!("[{tag}] seat listing names no seat {}", self.at.seat);
            return mycelium::client::sdk::seat::listed_seats(&listing).map(|(e, _)| e);
        };
        entity.with_seat_mind(self.at, |m| m.perceive_listing(&fns));
        let named: Vec<(u16, &[u8])> = fns.iter().map(|f| (f.id, f.tags.as_slice())).collect();
        eprintln!("[{tag}] seat {} listing perceived at epoch {epoch}: fns (id, arg tags) {named:?}", self.at.seat);
        Some(epoch)
    }

    /// THE MIND'S CALL AT THIS BEAT, where the seat is held: decided, posted,
    /// and its reference held for the outcome — one typed line.
    fn beat(&mut self, entity: &mut Entity, tag: &str) {
        if !self.hold.held {
            return;
        }
        let decision = entity.with_seat_mind(self.at, |m| m.decide());
        eprintln!("[{tag}] seat beat {decision}");
        let Decision::Call(choice) = decision else { return };
        // A SOLVE ANSWERS ITS TASK, never a beat: fn 1 is posted by
        // `solve_before` with the task's own tape, so a beat that would call
        // it again with an older solve's arguments makes no call.
        if choice.fn_id == FN_SOLVE {
            eprintln!("[{tag}] seat beat makes no call: fn {FN_SOLVE} answers a task, never a beat");
            return;
        }
        // THE STANDING CALL: the same function with the same arguments, its
        // last outcome applied, is not posted again — the setting stands.
        let mut encoded = Vec::new();
        let encodes = mycelium::anastome::dialogue::seat::put_args(&mut encoded, &choice.args).is_some();
        if encodes && self.standing.as_ref().is_some_and(|(f, a, applied)| *applied && *f == choice.fn_id && *a == encoded) {
            eprintln!("[{tag}] seat call fn {} stands: its last outcome applied, not posted again", choice.fn_id);
            return;
        }
        match entity.relay().call(self.at, &self.key, choice.fn_id, choice.args.clone()) {
            Ok((Ok(seq), reference)) => {
                self.calls += 1;
                self.standing = encodes.then_some((choice.fn_id, encoded, false));
                entity.with_seat_mind(self.at, |m| m.called(choice.fn_id, choice.args, reference));
                eprintln!("[{tag}] seat call fn {} posted (seq {seq})", choice.fn_id);
            }
            Ok((Err(why), _)) => eprintln!("[{tag}] seat call fn {} refused: {why:?}", choice.fn_id),
            Err(e) => eprintln!("[{tag}] seat call fn {} unsent: {e}", choice.fn_id),
        }
    }
}

impl SeatArm {
    /// 78.146 — A TASK BEFORE THE SEAT: fetched by its id, its rule extracted
    /// and minted as the task arm does (the form spoken, the tests spoken
    /// through it, the sentence minted at its span, crossed to pairs it never
    /// saw, replayed at home on the train), then CALLED as fn 1 with the
    /// rule's tape, the outputs and — a pure motion and hue map — its steps.
    /// No rule minted, no call, and the line says why.
    fn solve_before(&mut self, entity: &mut Entity, tag: &str, id: &[u8; 32]) {
        let no_call = |why: &str| eprintln!("[{tag}] seat task {} no call: {why}", crate::speak::fam_id(id));
        if !self.hold.held {
            return no_call("the seat is not held");
        }
        let (train, tests) = match entity.pull_task_by_id(id) {
            Ok(Some(env)) => (env.train, env.tests),
            Ok(None) => {
                self.answered.insert(id, ());
                return no_call("the registry holds no such task");
            }
            Err(e) => return no_call(&format!("the task was not fetched: {e}")),
        };
        let gen = entity.epigenome.atlas_root();
        // 78.158a U4 — a rule the house refused for this task, at the atlas
        // root it was extracted at, is not extracted again until the
        // language moves; then the rule's address is read against it below.
        if self.refused.get(id).is_some_and(|(root, _)| *root == gen) {
            return no_call("withheld again: the house refused this task's rule and the language has not moved");
        }
        let frames = match self.memo.holds(id, &gen) {
            true => mycelium::transducer::family::Frames::new(),
            false => mycelium::transducer::family::perceive_family(&train),
        };
        let mut budget = entity.stroke_n(mycelium::transducer::family::extraction_spend(&train, &frames));
        let Some(rule) = mycelium::transducer::family::extract_rule_cached(&mut self.memo, id, &train, gen, &mut budget, &entity.epigenome, Some(&frames)) else {
            return no_call("extract withheld");
        };
        let _ = crate::speak::speak_form(entity, tag, &rule, &train, &frames, id, &mut budget);
        let outs: Option<Vec<mycelium::arc::Grid>> = tests
            .iter()
            .map(|x| mycelium::transducer::family::speak_rule(&rule, x, &entity.epigenome).filter(|s| s.remainder.is_empty()).and_then(|s| s.grid()))
            .collect();
        let Some(outs) = outs else { return no_call("a test is not spoken whole by the rule") };
        // 78.158a U1 — what is not minted says why: the span unmeasured, or
        // the mint's refusal named whole with the form's demand kinds.
        let Some(deliberation) = entity.span_of(&rule.form) else {
            return no_call("no sentence minted: the span is unmeasurable");
        };
        let minted = match mycelium::transducer::family::mint_sentence_why(&rule, &train, &entity.epigenome, deliberation.span()) {
            Ok(w) => w,
            Err(why) => return no_call(&format!("no sentence minted: {}", mycelium::transducer::family::mint_refusal_line(why, &rule.form))),
        };
        let crossed = match mycelium::sdk::rule::crossing(&minted, &train, &entity.epigenome) {
            Ok(e) => e,
            Err(why) => return no_call(&format!("the rule does not cross: {why:?}")),
        };
        if self.refused.get(id).is_some_and(|(_, addr)| addr.as_slice() == crossed.address().as_bytes()) {
            return no_call("withheld again: the house refused this rule for the task, and its address has not changed");
        }
        // 78.157 §3a — the home check resolves the COURT-STAMPED words only,
        // on this solve's stroke; 78.158a U2 — unread at home is unjudged,
        // and the solve goes to the court, which resumes it.
        let resolve = |a: &[u8]| entity.epigenome.court_word(a).and_then(|e| e.form().cloned());
        match mycelium::sdk::rule::would_judge_with(&crossed, &train, &resolve, &entity.epigenome, &mut budget) {
            Ok(()) => {}
            Err(mycelium::sdk::rule::NoRule::Unread) => eprintln!(
                "[{tag}] seat task {} unjudged at home: {} — posted for the court to resume",
                crate::speak::fam_id(id),
                mycelium::verdict::REFUSE_UNREAD
            ),
            Err(mycelium::sdk::rule::NoRule::CitesUnheld(a)) => {
                return no_call(&format!("cites unheld: the rule cites {} no court admitted", crate::speak::fam_id(&a)));
            }
            Err(why) => return no_call(&format!("refuted at home: {why:?}")),
        }
        let shapes = |g: &mycelium::arc::Grid| match g.as_slice() {
            [row, ..] => (g.len(), row.len()),
            [] => (0, 0),
        };
        // The shapes the D4 kind is read on: the family's first pair, in the
        // registry's own order (its envelope is the statement).
        let steps = match train.as_slice() {
            [(a, b), ..] => match grid_steps(&crossed, shapes(a), shapes(b)) {
                Ok(s) => s,
                Err(why) => {
                    eprintln!("[{tag}] seat task {} rule is no pure motion/hue: {why:?}", crate::speak::fam_id(id));
                    Vec::new()
                }
            },
            [] => Vec::new(),
        };
        let Some(args) = solve_args(&crossed, &outs, &steps) else { return no_call("the rule's tape or the outputs pass their carrier") };
        // 78.158a U4 — THE SOLVE IS THE MIND'S OWN CALL: its reference kept,
        // the mind told it called (so the outcome reads against fn 1, never
        // `fn None`), and the task in flight until the outcome answers it.
        match entity.relay().call(self.at, &self.key, FN_SOLVE, args.clone()) {
            Ok((Ok(seq), reference)) => {
                self.calls += 1;
                entity.with_seat_mind(self.at, |m| m.called(FN_SOLVE, args, reference));
                self.solving.insert(id, (reference, crossed.address().as_bytes().to_vec()));
                eprintln!("[{tag}] seat task {} call fn {FN_SOLVE} posted (seq {seq}): {} outputs, steps {steps:?}", crate::speak::fam_id(id), outs.len());
            }
            Ok((Err(why), _)) => eprintln!("[{tag}] seat task call refused: {why:?}"),
            Err(e) => eprintln!("[{tag}] seat task call unsent: {e}"),
        }
    }
}

/// A PERCEPT'S TYPED DIGEST: its records by kind and their arguments by tag.
fn digest(records: &[Record]) -> String {
    let mut tags = [0u64; 7];
    let mut count = |args: &[Arg]| {
        for a in args {
            if let Some(n) = tags.get_mut(usize::from(a.tag())) {
                *n += 1;
            }
        }
    };
    let mut kinds: Vec<u16> = Vec::new();
    let mut counsel = 0u64;
    for r in records {
        match r {
            Record::Plain { kind, args } => {
                kinds.push(*kind);
                count(args);
            }
            Record::Counsel { args, worth, cost, .. } => {
                counsel += 1;
                count(args);
                count(std::slice::from_ref(worth));
                count(std::slice::from_ref(cost));
            }
        }
    }
    format!(
        "records {} kinds {kinds:?} counsel {counsel} | args Q {} Key {} Count {} Flag {} Mv {} BigQ {} Bytes {}",
        records.len(),
        tags[0],
        tags[1],
        tags[2],
        tags[3],
        tags[4],
        tags[5],
        tags[6]
    )
}
