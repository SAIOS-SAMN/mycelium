// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A HOUSE THAT SEATS A MIND, on the SDK: the domain's sitting reserve lists
// one seat, answers the mind that takes it, sets the court's tasks before it
// one at a time as percepts, and answers each of the mind's calls with an
// outcome (78.145, 78.146; MU-HOSTING §5a.6; MU-CLIENT §6).
//
//! `seats <house home> <hypha address> [tasks]`
//!
//! 1. Identity — the house's Ed25519 key at `<house home>/voice.key`, the
//!    same file `house` reads, so the domain is that house's.
//! 2. The domain — founded on the key with its one tier the same key, and
//!    that tier designated the sitting reserve, each where it is not yet
//!    (the relay files only posts of a domain with a reserve, signed by it).
//! 3. The listing — seat 1, acting on the domain, its functions `FN_SOLVE`
//!    (the tape and the outputs as bytes) and `FN_DECLINE` (a citation and
//!    a stop's code).
//! 4. The mail — the house's box read at the relay's standing: a TAKE of
//!    seat 1 is answered TAKEN; the holder is then sent a PERCEPT setting
//!    one court task before the seat (record kind 4, `[Key(citation)]`),
//!    the task drawn by `ACT_TASK`. Each CALL is answered with an OUTCOME:
//!    `FN_SOLVE` and `FN_DECLINE` applied, another function refused
//!    `REASON_NO_SUCH_FUNCTION`, a caller not holding the seat refused
//!    `REASON_NOT_THE_HOLDER`. A declination sets the next task before the
//!    seat, up to `tasks` (default 8).
//!
//! The house judges nothing: the court judges the solve, which the seated
//! mind files under its own key and is paid itself (Concordance 6.2a). The
//! house prints, for the seat's `FN_SOLVE`, the line
//! `solved: task <citation> by mind <key>`, which the quickstart reads
//! against the court's judged reading.
//!
//! Exit 0 once a seated mind's `FN_SOLVE` is answered; 3 where the seat was
//! taken and every task set before it was declined (the seat's path whole,
//! no solve); 1 where the court gave no task, a post was refused, or the
//! hypha could not be reached; 2 on a usage refusal. The run is bounded by
//! its caller.

use mycelium_sdk::house::seat::{Arg, Call, MailRead, Outcome, Percept, Post, Record, Role, Seat, SeatFn, Seats, Take, Taken};
use mycelium_sdk::house::seat::{REASON_NOT_THE_HOLDER, REASON_NO_SUCH_FUNCTION};
use mycelium_sdk::house::{payload_hash, DomainGenesis, Founded, Reservation, FN_DECLINE, FN_SOLVE, KIND_TASK};
use mycelium_sdk::prelude::*;
use mycelium_sdk::task;
use mycelium_sdk::voice::hex32;
use std::process::ExitCode;

/// The one seat this house offers.
const SEAT: u32 = 1;

/// What the house holds of its seat and its tasks.
struct House {
    key: SigningKey,
    domain: [u8; 32],
    /// The mind holding seat 1, where one does.
    holder: Option<[u8; 32]>,
    /// The house's percept sequence: one more per percept sent.
    beat: u64,
    /// Tasks set before the seat, and how many may be.
    set: u64,
    tasks: u64,
    /// The task before the seat now.
    before: Option<[u8; 32]>,
}

/// What a call came to.
enum Answered {
    /// A seat's `FN_SOLVE` for this task, by this mind.
    Solved([u8; 32], [u8; 32]),
    /// A declination: the next task is due.
    Declined,
    /// Anything else answered.
    Other,
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let (Some(home), Some(endpoint)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: seats <house home> <hypha address> [tasks]");
        return ExitCode::from(2);
    };
    let tasks = match args.get(3).map(|t| t.parse::<u64>()) {
        None => 8,
        Some(Ok(n)) => n,
        Some(Err(e)) => {
            eprintln!("tasks: {e}");
            return ExitCode::from(2);
        }
    };
    if let Err(e) = std::fs::create_dir_all(home) {
        eprintln!("{home}: {e}");
        return ExitCode::from(2);
    }
    let (voice, key) = mycelium_sdk::voice::load_or_mint(&format!("{home}/voice.key"));
    let domain = key.verifying_key().to_bytes();
    println!("seats: house {}", hex32(&domain));
    let session = Session::open(endpoint, voice);
    if !founded_and_reserved(&session, &key) {
        return ExitCode::from(1);
    }
    let mut house = House { key, domain, holder: None, beat: 0, set: 0, tasks, before: None };
    if !house.list(&session) {
        return ExitCode::from(1);
    }
    house.serve(&session)
}

/// THE DOMAIN FOUNDED AND ITS RESERVE DESIGNATED, each where it is not yet.
fn founded_and_reserved(session: &Session, key: &SigningKey) -> bool {
    let domain = key.verifying_key().to_bytes();
    match session.found_domain(&DomainGenesis { domain, tiers: vec![domain] }.sign(key)) {
        Ok(Founded::Sealed { at, .. }) => println!("seats: domain sealed at height {}", at.h),
        Ok(Founded::AlreadyFounded) => println!("seats: domain founded already"),
        Ok(other) => {
            println!("seats: domain refused, {other:?}");
            return false;
        }
        Err(e) => {
            eprintln!("seats: domain: {e}");
            return false;
        }
    }
    if matches!(session.reserve_reading(domain), Ok(r) if r.current == Some(domain)) {
        return true;
    }
    match session.reserve(&Reservation { domain, tier: domain, nonce: payload_hash(&domain) }.sign(key)) {
        Ok(f) => println!("seats: reserve {f:?}"),
        Err(e) => {
            eprintln!("seats: reserve: {e}");
            return false;
        }
    }
    matches!(session.reserve_reading(domain), Ok(r) if r.current == Some(domain))
}

impl House {
    /// One post signed by the house and filed; whether the relay filed it.
    fn post(&self, session: &Session, post: Post) -> bool {
        let Some(signed) = post.sign(&self.key) else {
            println!("seats: a post that does not write");
            return false;
        };
        match session.seat_post(&signed) {
            Ok(Ok(_)) => true,
            Ok(Err(why)) => {
                println!("seats: the relay refused a post, {why:?}");
                false
            }
            Err(e) => {
                println!("seats: a post did not reach the relay: {e}");
                false
            }
        }
    }

    /// THE LISTING: seat 1 and its two functions.
    fn list(&self, session: &Session) -> bool {
        // Each argument's tag read off an argument of its kind.
        let (bytes, key, count) = (Arg::Bytes(Vec::new()).tag(), Arg::Key([0; 32]).tag(), Arg::Count(0).tag());
        let fns = vec![SeatFn { id: FN_SOLVE, tags: vec![bytes, bytes] }, SeatFn { id: FN_DECLINE, tags: vec![key, count] }];
        let seat = Seat { seat: SEAT, role: Role(0), on: self.domain, beat: Arg::Q { num: 1, den: 1 }, fns };
        let listed = self.post(session, Post::Seats(Seats { domain: self.domain, sub_origin: None, epoch: 1, seats: vec![seat] }));
        if listed {
            println!("seats: seat {SEAT} listed");
        }
        listed
    }

    /// THE HOUSE'S BOX, read until a seated mind's solve is answered or no
    /// task is left to set.
    fn serve(&mut self, session: &Session) -> ExitCode {
        let (mut after, mut epoch) = (0u64, 0u64);
        loop {
            let read = match session.read_mail(&self.key, after, epoch) {
                Ok(Ok(r)) => r,
                Ok(Err(why)) => {
                    println!("seats: the relay refused the house's read, {why:?}");
                    return ExitCode::from(1);
                }
                // A read held past the transport's deadline is asked again;
                // a hypha not reached ends the run.
                Err(e) if matches!(e.kind(), std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock) => continue,
                Err(e) => {
                    println!("seats: the house's read did not reach the relay: {e}");
                    return ExitCode::from(1);
                }
            };
            // An empty page moves the cursor to the relay's standing, so the
            // next read is held to the court's next event.
            let MailRead { epoch: e, seq, mail } = read;
            epoch = e;
            if mail.is_empty() {
                after = seq;
            }
            for piece in mail {
                // The relay serves the box in sequence order: each piece
                // read moves the cursor past it.
                after = piece.seq;
                let Some(signed) = piece.post() else { continue };
                let reference = signed.reference();
                let answered = match (signed.post, reference) {
                    (Post::Take(t), _) => {
                        self.take(session, &t);
                        Answered::Other
                    }
                    (Post::Call(c), Some(r)) => self.call(session, &c, r),
                    _ => Answered::Other,
                };
                match answered {
                    Answered::Solved(task, mind) => {
                        println!("solved: task {} by mind {}", hex32(&task), hex32(&mind));
                        return ExitCode::SUCCESS;
                    }
                    Answered::Declined if self.set >= self.tasks => {
                        println!("seats: {} tasks set, each declined", self.set);
                        return ExitCode::from(3);
                    }
                    Answered::Declined if !self.set_task(session) => return ExitCode::from(1),
                    Answered::Declined | Answered::Other => {}
                }
            }
        }
    }

    /// A TAKE ANSWERED: held where seat 1 is free or already this mind's,
    /// and the first task set before the holder.
    fn take(&mut self, session: &Session, t: &Take) {
        let ok = t.seat == SEAT && self.holder.is_none_or(|h| h == t.mind);
        let reason = if ok { 0 } else { REASON_NOT_THE_HOLDER };
        let taken = Taken { domain: self.domain, sub_origin: None, seat: t.seat, mind: t.mind, ok, reason };
        if !self.post(session, Post::Taken(taken)) {
            return;
        }
        println!("seats: seat {} taken by mind {}: {}", t.seat, hex32(&t.mind), if ok { "held" } else { "refused" });
        if ok && self.holder.replace(t.mind).is_none() {
            let _ = self.set_task(session);
        }
    }

    /// THE NEXT COURT TASK SET BEFORE THE HOLDER, as a percept; whether one
    /// was set.
    fn set_task(&mut self, session: &Session) -> bool {
        let Some(mind) = self.holder else { return false };
        let reply = match session.speak(task::encode_task_request(&self.domain)) {
            Ok(r) => r,
            Err(e) => {
                println!("seats: the court gave no task: {e}");
                return false;
            }
        };
        let Some(envelope) = task::decode_task(&reply) else {
            println!("seats: the court gave no task ({} bytes)", reply.len());
            return false;
        };
        self.set += 1;
        self.beat += 1;
        let cite = envelope.id;
        let records = vec![Record::Plain { kind: KIND_TASK, args: vec![Arg::Key(cite)] }];
        let Ok(time) = i64::try_from(self.beat) else { return false };
        let percept = Percept { domain: self.domain, sub_origin: None, seat: SEAT, mind, local_time: Arg::Q { num: time, den: 1 }, seq: self.beat, records, point: None };
        let sent = self.post(session, Post::Percept(percept));
        if sent {
            self.before = Some(cite);
            println!("seats: task {} set before seat {SEAT} ({} of {})", hex32(&cite), self.set, self.tasks);
        }
        sent
    }

    /// A CALL ANSWERED with its outcome.
    fn call(&mut self, session: &Session, c: &Call, reference: [u8; 32]) -> Answered {
        let holds = c.seat == SEAT && self.holder == Some(c.mind);
        // A declination's arguments: the task's citation, then the stop.
        let cites = |args: &[Arg]| match args {
            [Arg::Key(k), ..] => Some(*k),
            _ => None,
        };
        let (applied, reason, answered) = match (holds, c.fn_id) {
            (false, _) => (false, REASON_NOT_THE_HOLDER, Answered::Other),
            (true, FN_SOLVE) => match self.before {
                Some(task) => (true, 0, Answered::Solved(task, c.mind)),
                None => (false, REASON_NO_SUCH_FUNCTION, Answered::Other),
            },
            (true, FN_DECLINE) => match (cites(&c.args), self.before) {
                (Some(k), Some(task)) if k == task => (true, 0, Answered::Declined),
                _ => (true, 0, Answered::Other),
            },
            (true, _) => (false, REASON_NO_SUCH_FUNCTION, Answered::Other),
        };
        let deed = match &answered {
            Answered::Solved(task, _) => *task,
            _ => [0; 32],
        };
        let outcome = Outcome { domain: self.domain, sub_origin: None, seat: c.seat, mind: c.mind, call: reference, applied, reason, deed };
        if !self.post(session, Post::Outcome(outcome)) {
            return Answered::Other;
        }
        let stop = match c.args.get(1) {
            Some(Arg::Count(n)) if c.fn_id == FN_DECLINE => format!(", stop {n}"),
            _ => String::new(),
        };
        println!("seats: call fn {} by mind {}: {}{stop}", c.fn_id, hex32(&c.mind), if applied { "applied" } else { "refused" });
        answered
    }
}
