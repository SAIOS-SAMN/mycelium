// 78.145 — THE SEAT SURFACE, FROM A KERNEL: a mind lists a domain's seats,
// takes one, reads its mail (taken, percepts, outcomes) and calls; a house —
// its domain's sitting reserve — lists its seats, reads its mail (takes and
// calls) and answers. Every act through the raw act door (`Session::speak`)
// with `dialogue::seat`'s codec, so the bytes a kernel sends are the bytes
// the relay holds.
//
//! The mail is held on the hypha a post reached, volatile: a house and the
//! minds seated in it speak to the same hypha (owed: routing a domain's mail
//! across the nerve). A reply that does not decode is an error by name.

use crate::anastome::dialogue::seat::*;
use ed25519_dalek::SigningKey;
use rand_core::{OsRng, RngCore};
use std::io;

fn undecoded(what: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("the seat {what} reply did not decode"))
}

fn unwritten() -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, "the seat post cannot be written (a denominator, or a length past its carrier)")
}

fn nonce() -> [u8; 32] {
    let mut n = [0u8; 32];
    OsRng.fill_bytes(&mut n);
    n
}

/// WHAT A MIND READS IN ITS MAIL, typed: the house's answer to its take, a
/// percept, an outcome — each with the relay's sequence.
#[derive(Clone, Debug, PartialEq)]
pub enum MindMail {
    Taken(u64, Taken),
    Percept(u64, Percept),
    Outcome(u64, Outcome),
}

/// v1.4 — A MIND'S MAIL AT THE RELAY'S STANDING: the relay's epoch, its last
/// sequence, and the pieces typed.
#[derive(Clone, Debug, PartialEq)]
pub struct MindRead {
    pub epoch: u64,
    pub seq: u64,
    pub mail: Vec<MindMail>,
}

/// WHAT A HOUSE READS IN ITS MAIL: a take and a call, each with its
/// sequence; a call with its reference, which the outcome names.
#[derive(Clone, Debug, PartialEq)]
pub enum HouseMail {
    Take(u64, Take),
    Call { seq: u64, call: Call, reference: [u8; 32] },
}

/// A MIND'S RAW MAIL TYPED: a piece that does not read as a TAKEN, PERCEPT
/// or OUTCOME naming `me` — which the relay never serves — is an error by
/// name, never skipped.
pub(crate) fn mind_mail(me: [u8; 32], raw: Vec<Mail>) -> io::Result<Vec<MindMail>> {
    let mut out = Vec::new();
    for m in raw {
        let post = m.post().ok_or_else(|| undecoded("mail piece"))?.post;
        if post.mind() != Some(me) {
            return Err(undecoded("mail piece addressed to another"));
        }
        out.push(match post {
            Post::Taken(t) => MindMail::Taken(m.seq, t),
            Post::Percept(p) => MindMail::Percept(m.seq, p),
            Post::Outcome(o) => MindMail::Outcome(m.seq, o),
            _ => return Err(undecoded("mail piece for a house")),
        });
    }
    Ok(out)
}

/// 78.148, MU-CLIENT §7.1 — A MIND'S HOLD ON ITS SEAT, BY THE HOUSE'S OWN
/// EVENTS (the operator, 2026-10-17: a stroke-count retry flooded a house
/// that refused every take, ~360 takes a second, its TAKENs dropped at the
/// relay's bound): whether its take is posted, whether the house answered
/// it held, and the listing epoch it last posted or was refused under. A
/// seat is taken only where the house's latest listing NAMES it, and once
/// under that listing: refused, or unanswered, it waits for the house's next
/// listing epoch — never a count of the mind's strokes. A call refused NOT
/// THE HOLDER (the house restarted, the seat passed) lets the take go, and
/// the seat may be taken again under the listing standing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SeatHold {
    pub posted: bool,
    pub held: bool,
    pub under: Option<u64>,
}

impl SeatHold {
    /// THE BOUND ON A READING OF THE MIND'S OWN RELAY (its mail's pages, its
    /// listing read again): a full APERTURE CYCLE of the mind's own strokes
    /// (`physics::aperture_cycle`). Never a bound on what is sent a house.
    pub(crate) fn patience() -> u64 {
        u64::from(crate::physics::aperture_cycle())
    }

    /// WHETHER THE TAKE IS TO BE POSTED, the house's latest listing naming
    /// this seat under epoch `listed` (none: the listing names it not, or no
    /// listing is read): not held, and not already posted or refused under
    /// that epoch.
    pub(crate) fn may_take(&self, listed: Option<u64>) -> bool {
        listed.is_some_and(|e| !self.held && self.under != Some(e))
    }

    /// The take was posted under the listing epoch `epoch`: its answer
    /// awaited, or the next epoch.
    pub(crate) fn take_posted(&mut self, epoch: u64) {
        *self = SeatHold { posted: true, held: false, under: Some(epoch) };
    }

    /// The take let go: it may be posted again under the listing standing.
    pub(crate) fn let_go(&mut self) {
        *self = SeatHold::default();
    }

    /// THE HOUSE'S TAKEN: held — and the listing is to be read (true) — or
    /// refused, the seat waiting for the house's next listing epoch.
    pub(crate) fn taken(&mut self, ok: bool) -> bool {
        self.posted = false;
        self.held = ok;
        if ok {
            self.posted = true;
        }
        ok
    }

    /// AN OUTCOME READ: refused NOT THE HOLDER lets the take go; refused NO
    /// SUCH FUNCTION asks the listing again (true).
    pub(crate) fn outcome(&mut self, applied: bool, reason: u16) -> bool {
        match (applied, reason) {
            (false, REASON_NOT_THE_HOLDER) => {
                self.let_go();
                false
            }
            (false, REASON_NO_SUCH_FUNCTION) => true,
            _ => false,
        }
    }
}

/// 78.148 — ONE SEAT'S FUNCTIONS in a listing, with the listing's epoch:
/// the repertoire a mind at that seat perceives. `None` where the post is no
/// SEATS or names no such seat.
pub(crate) fn seat_fns(listing: &SignedPost, seat: u32) -> Option<(u64, Vec<SeatFn>)> {
    let Post::Seats(seats) = &listing.post else { return None };
    let mine = seats.seats.iter().find(|s| s.seat == seat)?;
    Some((seats.epoch, mine.fns.clone()))
}

/// 78.158a SIT-ALL — EVERY SEAT A LISTING OFFERS, in the house's own
/// listing order, with the listing's epoch. `None` where the post is no
/// SEATS.
pub(crate) fn listed_seats(listing: &SignedPost) -> Option<(u64, Vec<u32>)> {
    let Post::Seats(seats) = &listing.post else { return None };
    let mut ids: Vec<u32> = Vec::new();
    for s in &seats.seats {
        if !ids.contains(&s.seat) {
            ids.push(s.seat);
        }
    }
    Some((seats.epoch, ids))
}

/// 78.158a SIT-ALL — A LISTING READ AGAINST THE SEATS SAT: the seats it
/// offers that are not sat (to take, in the listing's order) and the seats
/// sat that it no longer offers (to let go, in the order they were sat).
pub(crate) fn reseat(sat: &[u32], listed: &[u32]) -> (Vec<u32>, Vec<u32>) {
    let take = listed.iter().filter(|s| !sat.contains(s)).copied().collect();
    let let_go = sat.iter().filter(|s| !listed.contains(s)).copied().collect();
    (take, let_go)
}

/// A SEAT'S COORDINATES, as (domain, sub-origin, seat) name it (v1.4):
/// `[0][domain][seat BE]` in the domain's own universe, `[1][domain][sub
/// origin][seat BE]` in a sub-universe of it.
fn seat_key(domain: &[u8; 32], sub_origin: Option<&[u8; 32]>, seat: u32) -> Vec<u8> {
    let mut k = Vec::with_capacity(69);
    match sub_origin {
        None => {
            k.push(0);
            k.extend_from_slice(domain);
        }
        Some(s) => {
            k.push(1);
            k.extend_from_slice(domain);
            k.extend_from_slice(s);
        }
    }
    k.extend_from_slice(&seat.to_be_bytes());
    k
}

/// v1.4 — A SEAT'S NAME: its domain, the sub-universe it stands in (none, the
/// domain's own), and its id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SeatName {
    pub domain: [u8; 32],
    pub sub_origin: Option<[u8; 32]>,
    pub seat: u32,
}

impl super::Entity {
    /// 78.146 — THE MIND AT A SEAT it sits, written: taken from its
    /// coordinate (begun where it was not) and put back there.
    pub(crate) fn with_seat_mind<R>(&mut self, at: SeatName, f: impl FnOnce(&mut super::seat_mind::SeatMind) -> R) -> R {
        let key = seat_key(&at.domain, at.sub_origin.as_ref(), at.seat);
        let mut mind = match self.seats.remove(&key) {
            Some(m) => m,
            None => super::seat_mind::SeatMind::scoped(at.domain, at.sub_origin, at.seat),
        };
        let r = f(&mut mind);
        self.seats.insert(&key, mind);
        r
    }

    /// 78.146 — A PERCEPT AT A SEAT, perceived: held on its series, and a
    /// call's effect read where its outcome came. Reading what a call did is
    /// the mind learning — an act, which moves no clock (K moves by the
    /// mind's returns, MU-CLIENT §4.4(4)).
    pub(crate) fn perceive_seat(&mut self, p: &Percept) -> super::seat_mind::Perceived {
        let at = SeatName { domain: p.domain, sub_origin: p.sub_origin, seat: p.seat };
        self.with_seat_mind(at, |m| m.perceive(p))
    }
}

impl super::Session {
    /// One signed post, answered by its sequence or the refusal by name.
    pub(crate) fn seat_post(&self, s: &SignedPost) -> io::Result<Result<u64, Refusal>> {
        let reply = self.speak(encode_post(s).ok_or_else(unwritten)?)?;
        decode_post_reply(&reply).ok_or_else(|| undecoded("post"))
    }

    /// A DOMAIN'S SEATS as its house last listed them — in its own universe,
    /// or (v1.4) at a sub-origin — with the house's signature
    /// (`SignedPost::signed_by` the reserve's key verifies it); `None` where
    /// none is listed.
    pub(crate) fn list_seats(&self, domain: [u8; 32], sub_origin: Option<[u8; 32]>) -> io::Result<Option<SignedPost>> {
        let reply = self.speak(encode_read_listing(&domain, sub_origin.as_ref()))?;
        decode_listing_reply(&reply, sub_origin.as_ref()).ok_or_else(|| undecoded("listing"))
    }

    /// A MIND TAKES A SEAT, signed by its own key.
    pub(crate) fn take(&self, at: SeatName, key: &SigningKey) -> io::Result<Result<u64, Refusal>> {
        let mind = key.verifying_key().to_bytes();
        let s = Post::Take(Take { domain: at.domain, sub_origin: at.sub_origin, seat: at.seat, mind, nonce: nonce() }).sign(key).ok_or_else(unwritten)?;
        self.seat_post(&s)
    }

    /// A SEATED MIND CALLS: answered by the relay's sequence, with the call's
    /// reference — what the house's outcome will name.
    pub(crate) fn call(&self, at: SeatName, key: &SigningKey, fn_id: u16, args: Vec<Arg>) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        let mind = key.verifying_key().to_bytes();
        let c = Call { domain: at.domain, sub_origin: at.sub_origin, seat: at.seat, mind, nonce: nonce(), fn_id, args };
        let s = Post::Call(c).sign(key).ok_or_else(unwritten)?;
        let reference = s.reference().ok_or_else(unwritten)?;
        Ok((self.seat_post(&s)?, reference))
    }

    /// The raw mail past `after` (acknowledging everything at or below it),
    /// read at the relay's standing (v1.4): `after` as read under relay
    /// `epoch` (0 where none is held), answered with the relay's epoch and
    /// last sequence beside the pieces, so a cursor left on a restarted relay
    /// is told from progress (`MailRead::cursor`) and acknowledges nothing
    /// there.
    pub(crate) fn read_mail(&self, key: &SigningKey, after: u64, epoch: u64) -> io::Result<Result<MailRead, Refusal>> {
        let reply = self.speak(encode_read_mail(&ReadMail::sign_standing(key, after, epoch, nonce())))?;
        decode_mail_reply_standing(&reply).ok_or_else(|| undecoded("mail"))
    }

    /// A MIND'S MAIL past `after` (read under relay `epoch`), typed, with the
    /// relay's standing (epoch, last sequence). A piece that does not read as
    /// a TAKEN, PERCEPT or OUTCOME naming this mind — which the relay never
    /// serves — is an error by name, never skipped.
    pub(crate) fn mail(&self, key: &SigningKey, after: u64, epoch: u64) -> io::Result<Result<MindRead, Refusal>> {
        let me = key.verifying_key().to_bytes();
        match self.read_mail(key, after, epoch)? {
            Ok(read) => mind_mail(me, read.mail).map(|mail| Ok(MindRead { epoch: read.epoch, seq: read.seq, mail })),
            Err(why) => Ok(Err(why)),
        }
    }

}
