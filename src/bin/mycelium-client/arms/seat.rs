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

use mycelium::anastome::dialogue::seat::{Arg, Record};
use mycelium::client::sdk::seat::{seat_fns, MindMail, SeatHold};
use mycelium::client::sdk::seat_mind::task::{grid_steps, solve_args, tasks_before, FN_SOLVE};
use mycelium::client::sdk::seat_mind::Decision;
use mycelium::sdk::Entity;
use std::sync::atomic::{AtomicBool, Ordering};

/// THE SEAT THIS VOICE SITS.
pub(crate) struct SeatArm {
    domain: [u8; 32],
    seat: u32,
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
    /// once.
    answered: mycelium::network::nerve::Keyed<()>,
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
}

impl SeatArms {
    /// `--seat <domain-hex> <seat-id>[,<seat-id>…]`, or no arm.
    pub(crate) fn of_args(args: &[String], key: ed25519_dalek::SigningKey) -> Result<Option<SeatArms>, String> {
        let Some(at) = args.iter().position(|a| a == "--seat") else { return Ok(None) };
        let (Some(d), Some(s)) = (args.get(at + 1), args.get(at + 2)) else {
            return Err("--seat takes <domain-hex> <seat-id>[,<seat-id>…]".to_string());
        };
        let domain = mycelium::court::identity::parse_hex32(d).ok_or_else(|| format!("--seat: {d} is not a 32-byte hex domain"))?;
        let mut arms = Vec::new();
        for one in s.split(',') {
            let seat = one.trim().parse::<u32>().map_err(|e| format!("--seat: {one} is not a seat id: {e}"))?;
            if !arms.iter().any(|a: &SeatArm| a.seat == seat) {
                arms.push(SeatArm::new(domain, seat, key.clone()));
            }
        }
        Ok(Some(SeatArms { arms, key, after: 0 }))
    }

    /// ONE STROKE AT EVERY SEAT: each seat takes (again, where lost); the
    /// shared box is read to its end once; each piece goes to its seat; then
    /// each seat answers its tasks and makes its one call.
    pub(crate) fn stroke(&mut self, entity: &mut Entity, tag: &str, stop: &AtomicBool) {
        for arm in &mut self.arms {
            arm.take_step(entity, tag);
        }
        let mut beats = vec![0u64; self.arms.len()];
        let mut tasks: Vec<Vec<[u8; 32]>> = vec![Vec::new(); self.arms.len()];
        for _ in 0..pages() {
            let mail = match entity.relay().mail(&self.key, self.after) {
                Ok(Ok(m)) => m,
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
                let (domain, seat, seq) = match &piece {
                    MindMail::Taken(q, t) => (t.domain, t.seat, *q),
                    MindMail::Percept(q, p) => (p.domain, p.seat, *q),
                    MindMail::Outcome(q, o) => (o.domain, o.seat, *q),
                };
                self.after = seq;
                // A piece for a seat this mind does not sit is read past.
                let Some(i) = self.arms.iter().position(|a| a.domain == domain && a.seat == seat) else { continue };
                if let (Some(arm), Some(b), Some(t)) = (self.arms.get_mut(i), beats.get_mut(i), tasks.get_mut(i)) {
                    arm.read_piece(entity, tag, piece, b, t);
                }
            }
        }
        for ((arm, b), t) in self.arms.iter_mut().zip(beats).zip(tasks) {
            for id in t {
                if !arm.answered.contains(&id) {
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
    fn new(domain: [u8; 32], seat: u32, key: ed25519_dalek::SigningKey) -> SeatArm {
        SeatArm {
            domain,
            seat,
            key,
            hold: SeatHold::default(),
            after: 0,
            percepts: 0,
            calls: 0,
            memo: mycelium::transducer::family::ExtractMemo::new(),
            answered: mycelium::network::nerve::Keyed::new(),
            standing: None,
        }
    }

    /// THE SEAT TAKEN (again, where it was lost or went unanswered a full
    /// aperture cycle of strokes).
    fn take_step(&mut self, entity: &mut Entity, tag: &str) {
        let was_posted = self.hold.posted;
        if self.hold.stroke() {
            if was_posted {
                eprintln!("[{tag}] seat {} take let go: no TAKEN within an aperture cycle of {} strokes — taken again", self.seat, SeatHold::patience());
            }
            match entity.relay().take(self.domain, self.seat, &self.key) {
                Ok(Ok(seq)) => {
                    self.hold.take_posted();
                    eprintln!("[{tag}] seat {} take posted (seq {seq})", self.seat);
                }
                Ok(Err(why)) => eprintln!("[{tag}] seat {} take refused: {why:?}", self.seat),
                Err(e) => eprintln!("[{tag}] seat {} take unsent: {e}", self.seat),
            }
        }
    }

    /// ONE PIECE OF MAIL READ: a TAKEN marks the hold, a PERCEPT is perceived
    /// (its tasks gathered, its beat counted), an OUTCOME marks its call.
    fn read_piece(&mut self, entity: &mut Entity, tag: &str, piece: MindMail, beats: &mut u64, tasks: &mut Vec<[u8; 32]>) {
        {
            match piece {
                MindMail::Taken(seq, t) => {
                    self.after = seq;
                    eprintln!("[{tag}] seat {} taken {} (reason {})", t.seat, t.ok, t.reason);
                    if t.domain != self.domain || t.seat != self.seat {
                        return;
                    }
                    match self.hold.taken(t.ok) {
                        true => self.read_listing(entity, tag),
                        false => eprintln!("[{tag}] seat {} take let go: TAKEN refused (reason {}) — taken again next stroke", self.seat, t.reason),
                    }
                }
                MindMail::Percept(seq, p) => {
                    self.after = seq;
                    self.percepts += 1;
                    eprintln!("[{tag}] seat percept {} at {:?}: {}", p.seq, p.local_time, digest(&p.records));
                    let reading = entity.perceive_seat(&p);
                    eprintln!("[{tag}] seat perceived {reading}");
                    // 78.146 — a task before the seat is answered (once, after
                    // the box is read); a percept with none is a beat.
                    let before = tasks_before(&p);
                    if before.is_empty() {
                        *beats += 1;
                    }
                    for id in before {
                        if !tasks.contains(&id) {
                            tasks.push(id);
                        }
                    }
                }
                MindMail::Outcome(seq, o) => {
                    self.after = seq;
                    let answered = entity.with_seat_mind(self.domain, self.seat, |m| m.outcome(&o));
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
                    // 78.148 — a refusal is information: the seat lost, or
                    // the listing moved.
                    let held = self.hold.held;
                    if self.hold.outcome(o.applied, o.reason) {
                        self.read_listing(entity, tag);
                    } else if held && !self.hold.held {
                        eprintln!("[{tag}] seat {} take let go: a call refused NOT THE HOLDER — taken again next stroke", self.seat);
                    }
                }
            }
        }
    }

    /// 78.148 — THE LISTING PERCEIVED: the domain's latest SEATS read, and
    /// this seat's functions — id and argument tags — handed to the mind as
    /// its repertoire. One typed line.
    fn read_listing(&mut self, entity: &mut Entity, tag: &str) {
        let listing = match entity.relay().list_seats(self.domain) {
            Ok(Some(s)) => s,
            Ok(None) => return eprintln!("[{tag}] seat listing: none held for the domain"),
            Err(e) => return eprintln!("[{tag}] seat listing unread: {e}"),
        };
        let Some((epoch, fns)) = seat_fns(&listing, self.seat) else {
            return eprintln!("[{tag}] seat listing names no seat {}", self.seat);
        };
        entity.with_seat_mind(self.domain, self.seat, |m| m.perceive_listing(&fns));
        let named: Vec<(u16, &[u8])> = fns.iter().map(|f| (f.id, f.tags.as_slice())).collect();
        eprintln!("[{tag}] seat {} listing perceived at epoch {epoch}: fns (id, arg tags) {named:?}", self.seat);
    }

    /// THE MIND'S CALL AT THIS BEAT, where the seat is held: decided, posted,
    /// and its reference held for the outcome — one typed line.
    fn beat(&mut self, entity: &mut Entity, tag: &str) {
        if !self.hold.held {
            return;
        }
        let decision = entity.with_seat_mind(self.domain, self.seat, |m| m.decide());
        eprintln!("[{tag}] seat beat {decision}");
        let Decision::Call(choice) = decision else { return };
        // THE STANDING CALL: the same function with the same arguments, its
        // last outcome applied, is not posted again — the setting stands.
        let mut encoded = Vec::new();
        let encodes = mycelium::anastome::dialogue::seat::put_args(&mut encoded, &choice.args).is_some();
        if encodes && self.standing.as_ref().is_some_and(|(f, a, applied)| *applied && *f == choice.fn_id && *a == encoded) {
            eprintln!("[{tag}] seat call fn {} stands: its last outcome applied, not posted again", choice.fn_id);
            return;
        }
        match entity.relay().call(self.domain, self.seat, &self.key, choice.fn_id, choice.args.clone()) {
            Ok((Ok(seq), reference)) => {
                self.calls += 1;
                self.standing = encodes.then_some((choice.fn_id, encoded, false));
                entity.with_seat_mind(self.domain, self.seat, |m| m.called(choice.fn_id, choice.args, reference));
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
        let Some(minted) = entity.span_of(&rule.form).and_then(|d| mycelium::transducer::family::mint_sentence(&rule, &train, &entity.epigenome, d.span())) else {
            return no_call("no sentence minted (an unmeasured span, or the mint refused)");
        };
        let crossed = match mycelium::sdk::rule::crossing(&minted, &train, &entity.epigenome) {
            Ok(e) => e,
            Err(why) => return no_call(&format!("the rule does not cross: {why:?}")),
        };
        let resolve = |a: &[u8]| entity.epigenome.working_word(a).and_then(|e| e.form().cloned());
        if let Err(why) = mycelium::sdk::rule::would_judge(&crossed, &train, &resolve, &entity.epigenome) {
            return no_call(&format!("refuted at home: {why:?}"));
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
        match entity.relay().call(self.domain, self.seat, &self.key, FN_SOLVE, args) {
            Ok((Ok(seq), _)) => {
                self.calls += 1;
                self.answered.insert(id, ());
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
