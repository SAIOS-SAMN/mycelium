// 78.145 — THE SEAT ARM (first cut): a voice started with `--seat <domain-hex>
// <seat-id>` takes that seat in the game's house, reads its mail each stroke,
// holds each percept as its perception, calls, and learns from the outcomes.
//
//! v1 CHOOSES BY THE HOUSE'S COUNSEL: where a percept carries a COUNSEL
//! record, the call is the one it names — the in-game mind's choice, followed,
//! the first counsel in the house's own record order. Without counsel the arm
//! calls nothing. OWED, named: the records → Cl(4,1) forms the entity can
//! absorb (until then a percept is held here as typed records and logged as
//! a digest — never painted into sites it is not); a choice of the mind's
//! own, read off those forms; learning from an outcome beyond counting it.

use mycelium::anastome::dialogue::seat::{Arg, Record};
use mycelium::client::sdk::seat::MindMail;
use mycelium::sdk::Entity;
use std::sync::atomic::{AtomicBool, Ordering};

/// THE SEAT THIS VOICE SITS AND WHAT IT HAS LEARNED THERE.
pub(crate) struct SeatArm {
    domain: [u8; 32],
    seat: u32,
    key: ed25519_dalek::SigningKey,
    /// Whether the take was posted, and the house's answer when read.
    took: bool,
    taken: Option<bool>,
    /// The last sequence read: the next read acknowledges it.
    after: u64,
    /// THE PERCEPTION: the last percept's records and the house's local
    /// time it stood at.
    perceived: Vec<Record>,
    local_time: Option<Arg>,
    percepts: u64,
    calls: u64,
    applied: u64,
    /// Refused outcomes, by the house's reason.
    refused: Vec<(u16, u64)>,
}

impl SeatArm {
    /// `--seat <domain-hex> <seat-id>`, or no arm.
    pub(crate) fn of_args(args: &[String], key: ed25519_dalek::SigningKey) -> Result<Option<SeatArm>, String> {
        let Some(at) = args.iter().position(|a| a == "--seat") else { return Ok(None) };
        let (Some(d), Some(s)) = (args.get(at + 1), args.get(at + 2)) else {
            return Err("--seat takes <domain-hex> <seat-id>".to_string());
        };
        let domain = mycelium::court::identity::parse_hex32(d).ok_or_else(|| format!("--seat: {d} is not a 32-byte hex domain"))?;
        let seat = s.parse::<u32>().map_err(|e| format!("--seat: {s} is not a seat id: {e}"))?;
        Ok(Some(SeatArm {
            domain,
            seat,
            key,
            took: false,
            taken: None,
            after: 0,
            perceived: Vec::new(),
            local_time: None,
            percepts: 0,
            calls: 0,
            applied: 0,
            refused: Vec::new(),
        }))
    }

    /// ONE STROKE AT THE SEAT: take it once; read the mail; hold each
    /// percept and follow its counsel; count each outcome.
    pub(crate) fn stroke(&mut self, entity: &Entity, tag: &str, stop: &AtomicBool) {
        let session = entity.relay();
        if !self.took {
            match session.take(self.domain, self.seat, &self.key) {
                Ok(Ok(seq)) => {
                    self.took = true;
                    eprintln!("[{tag}] seat {} take posted (seq {seq})", self.seat);
                }
                Ok(Err(why)) => eprintln!("[{tag}] seat {} take refused: {why:?}", self.seat),
                Err(e) => eprintln!("[{tag}] seat {} take unsent: {e}", self.seat),
            }
            return;
        }
        let mail = match session.mail(&self.key, self.after) {
            Ok(Ok(m)) => m,
            Ok(Err(why)) => {
                eprintln!("[{tag}] seat mail refused: {why:?}");
                return;
            }
            Err(e) => {
                eprintln!("[{tag}] seat mail unread: {e}");
                return;
            }
        };
        for piece in mail {
            if stop.load(Ordering::SeqCst) {
                return;
            }
            match piece {
                MindMail::Taken(seq, t) => {
                    self.after = seq;
                    self.taken = Some(t.ok);
                    eprintln!("[{tag}] seat {} taken {} (reason {})", t.seat, t.ok, t.reason);
                }
                MindMail::Percept(seq, p) => {
                    self.after = seq;
                    self.percepts += 1;
                    eprintln!("[{tag}] seat percept {} at {:?}: {}", p.seq, p.local_time, digest(&p.records));
                    self.perceived = p.records;
                    self.local_time = Some(p.local_time);
                    self.follow_counsel(entity, tag);
                }
                MindMail::Outcome(seq, o) => {
                    self.after = seq;
                    if o.applied {
                        self.applied += 1;
                    } else {
                        match self.refused.iter_mut().find(|(r, _)| *r == o.reason) {
                            Some((_, n)) => *n += 1,
                            None => self.refused.push((o.reason, 1)),
                        }
                    }
                    eprintln!(
                        "[{tag}] seat outcome applied {} reason {} | percepts {} calls {} applied {} refused {:?}",
                        o.applied, o.reason, self.percepts, self.calls, self.applied, self.refused
                    );
                }
            }
        }
    }

    /// v1: THE CALL THE HOUSE COUNSELS, where the seat is held and a counsel
    /// stands in the perception.
    fn follow_counsel(&mut self, entity: &Entity, tag: &str) {
        if self.taken != Some(true) {
            return;
        }
        let Some((fn_id, args)) = self.perceived.iter().find_map(|r| match r {
            Record::Counsel { fn_id, args, .. } => Some((*fn_id, args.clone())),
            Record::Plain { .. } => None,
        }) else {
            return;
        };
        match entity.relay().call(self.domain, self.seat, &self.key, fn_id, args) {
            Ok((Ok(seq), _)) => {
                self.calls += 1;
                eprintln!("[{tag}] seat call fn {fn_id} (the house's counsel) posted (seq {seq})");
            }
            Ok((Err(why), _)) => eprintln!("[{tag}] seat call fn {fn_id} refused: {why:?}"),
            Err(e) => eprintln!("[{tag}] seat call fn {fn_id} unsent: {e}"),
        }
    }
}

/// A PERCEPT'S TYPED DIGEST: its records by kind and their arguments by tag.
fn digest(records: &[Record]) -> String {
    let mut tags = [0u64; 6];
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
        "records {} kinds {kinds:?} counsel {counsel} | args Q {} Key {} Count {} Flag {} Mv {} BigQ {}",
        records.len(),
        tags[0],
        tags[1],
        tags[2],
        tags[3],
        tags[4],
        tags[5]
    )
}
