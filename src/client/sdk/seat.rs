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
pub fn mind_mail(me: [u8; 32], raw: Vec<Mail>) -> io::Result<Vec<MindMail>> {
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

/// 78.148 — A MIND'S HOLD ON ITS SEAT: whether its take is posted, whether
/// the house answered it held, and how many of the mind's own strokes it has
/// waited for that answer. The seat is held again when it is lost: a TAKEN
/// refused, or a call refused NOT THE HOLDER (a house restarted, the seat
/// passed), lets the take go and the next stroke takes again; a take no
/// TAKEN answers within [`SeatHold::patience`] strokes is let go and posted
/// again.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SeatHold {
    pub posted: bool,
    pub held: bool,
    pub waited: u64,
}

impl SeatHold {
    /// THE BOUND ON AN UNANSWERED TAKE: a full APERTURE CYCLE of the mind's
    /// own strokes (`physics::aperture_cycle`, the stroke count at which a
    /// contact's consecutive failures read as occlusion) — never a clock.
    pub fn patience() -> u64 {
        u64::from(crate::physics::aperture_cycle())
    }

    /// AT A STROKE: whether the take is to be posted now — never posted, let
    /// go, or (counted here) unanswered for a full aperture cycle.
    pub fn stroke(&mut self) -> bool {
        if self.posted && !self.held {
            self.waited = match self.waited.checked_add(1) {
                Some(w) => w,
                None => self.waited,
            };
            if self.waited >= Self::patience() {
                self.let_go();
            }
        }
        !self.posted
    }

    /// The take was posted: its answer awaited from here.
    pub fn take_posted(&mut self) {
        *self = SeatHold { posted: true, held: false, waited: 0 };
    }

    /// The take let go: the next stroke posts it again.
    pub fn let_go(&mut self) {
        *self = SeatHold::default();
    }

    /// THE HOUSE'S TAKEN: held — and the listing is to be read (true) — or
    /// refused and let go.
    pub fn taken(&mut self, ok: bool) -> bool {
        match ok {
            true => {
                *self = SeatHold { posted: true, held: true, waited: 0 };
                true
            }
            false => {
                self.let_go();
                false
            }
        }
    }

    /// AN OUTCOME READ: refused NOT THE HOLDER lets the take go; refused NO
    /// SUCH FUNCTION asks the listing again (true).
    pub fn outcome(&mut self, applied: bool, reason: u16) -> bool {
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
pub fn seat_fns(listing: &SignedPost, seat: u32) -> Option<(u64, Vec<SeatFn>)> {
    let Post::Seats(seats) = &listing.post else { return None };
    let mine = seats.seats.iter().find(|s| s.seat == seat)?;
    Some((seats.epoch, mine.fns.clone()))
}

fn seat_key(domain: &[u8; 32], seat: u32) -> Vec<u8> {
    let mut k = domain.to_vec();
    k.extend_from_slice(&seat.to_be_bytes());
    k
}

impl super::Entity {
    /// The session this entity speaks through — the seat surface's mouth.
    pub fn relay(&self) -> &super::Session {
        &self.session
    }

    /// 78.146 — THE MIND AT A SEAT it sits, written: taken from its
    /// coordinate (begun where it was not) and put back there.
    pub fn with_seat_mind<R>(&mut self, domain: [u8; 32], seat: u32, f: impl FnOnce(&mut super::seat_mind::SeatMind) -> R) -> R {
        let key = seat_key(&domain, seat);
        let mut mind = match self.seats.remove(&key) {
            Some(m) => m,
            None => super::seat_mind::SeatMind::new(domain, seat),
        };
        let r = f(&mut mind);
        self.seats.insert(&key, mind);
        r
    }

    /// The mind at a seat, where it sits one.
    pub fn seat_mind_at(&self, domain: [u8; 32], seat: u32) -> Option<&super::seat_mind::SeatMind> {
        self.seats.get(&seat_key(&domain, seat))
    }

    /// 78.146 — A PERCEPT AT A SEAT, perceived: held on its series, and a
    /// call's effect read where its outcome came. Reading what a call did is
    /// the mind learning — an act of this orbit (78.49).
    pub fn perceive_seat(&mut self, p: &Percept) -> super::seat_mind::Perceived {
        let reading = self.with_seat_mind(p.domain, p.seat, |m| m.perceive(p));
        if reading.learned.is_some() {
            self.acted = true;
        }
        reading
    }
}

impl super::Session {
    /// One signed post, answered by its sequence or the refusal by name.
    pub fn seat_post(&self, s: &SignedPost) -> io::Result<Result<u64, Refusal>> {
        let reply = self.speak(encode_post(s).ok_or_else(unwritten)?)?;
        decode_post_reply(&reply).ok_or_else(|| undecoded("post"))
    }

    /// A DOMAIN'S SEATS as its house last listed them, with the house's
    /// signature (`SignedPost::signed_by` the reserve's key verifies it);
    /// `None` where none is listed.
    pub fn list_seats(&self, domain: [u8; 32]) -> io::Result<Option<SignedPost>> {
        let reply = self.speak(encode_read_listing(&domain))?;
        decode_listing_reply(&reply).ok_or_else(|| undecoded("listing"))
    }

    /// A MIND TAKES A SEAT, signed by its own key.
    pub fn take(&self, domain: [u8; 32], seat: u32, key: &SigningKey) -> io::Result<Result<u64, Refusal>> {
        let mind = key.verifying_key().to_bytes();
        let s = Post::Take(Take { domain, seat, mind, nonce: nonce() }).sign(key).ok_or_else(unwritten)?;
        self.seat_post(&s)
    }

    /// A SEATED MIND CALLS: answered by the relay's sequence, with the call's
    /// reference — what the house's outcome will name.
    pub fn call(&self, domain: [u8; 32], seat: u32, key: &SigningKey, fn_id: u16, args: Vec<Arg>) -> io::Result<(Result<u64, Refusal>, [u8; 32])> {
        let mind = key.verifying_key().to_bytes();
        let s = Post::Call(Call { domain, seat, mind, nonce: nonce(), fn_id, args }).sign(key).ok_or_else(unwritten)?;
        let reference = s.reference().ok_or_else(unwritten)?;
        Ok((self.seat_post(&s)?, reference))
    }

    /// The raw mail past `after` (acknowledging everything at or below it).
    pub fn read_mail(&self, key: &SigningKey, after: u64) -> io::Result<Result<Vec<Mail>, Refusal>> {
        let reply = self.speak(encode_read_mail(&ReadMail::sign(key, after, nonce())))?;
        decode_mail_reply(&reply).ok_or_else(|| undecoded("mail"))
    }

    /// A MIND'S MAIL past `after`, typed. A piece that does not read as a
    /// TAKEN, PERCEPT or OUTCOME naming this mind — which the relay never
    /// serves — is an error by name, never skipped.
    pub fn mail(&self, key: &SigningKey, after: u64) -> io::Result<Result<Vec<MindMail>, Refusal>> {
        let me = key.verifying_key().to_bytes();
        match self.read_mail(key, after)? {
            Ok(raw) => mind_mail(me, raw).map(Ok),
            Err(why) => Ok(Err(why)),
        }
    }

    /// THE HOUSE LISTS ITS SEATS at an epoch; the latest epoch replaces.
    pub fn post_seats(&self, house: &SigningKey, seats: Seats) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Seats(seats).sign(house).ok_or_else(unwritten)?)
    }

    /// THE HOUSE'S MAIL past `after`: the takes and calls addressed to its
    /// domain, typed.
    pub fn house_mail(&self, house: &SigningKey, after: u64) -> io::Result<Result<Vec<HouseMail>, Refusal>> {
        let raw = match self.read_mail(house, after)? {
            Ok(m) => m,
            Err(why) => return Ok(Err(why)),
        };
        let mut out = Vec::new();
        for m in raw {
            let s = m.post().ok_or_else(|| undecoded("mail piece"))?;
            out.push(match &s.post {
                Post::Take(t) => HouseMail::Take(m.seq, t.clone()),
                Post::Call(c) => HouseMail::Call { seq: m.seq, call: c.clone(), reference: s.reference().ok_or_else(unwritten)? },
                _ => return Err(undecoded("mail piece for a mind")),
            });
        }
        Ok(Ok(out))
    }

    /// THE HOUSE ANSWERS A TAKE.
    pub fn post_taken(&self, house: &SigningKey, t: Taken) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Taken(t).sign(house).ok_or_else(unwritten)?)
    }

    /// THE HOUSE PERCEIVES FOR A SEATED MIND.
    pub fn post_percept(&self, house: &SigningKey, p: Percept) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Percept(p).sign(house).ok_or_else(unwritten)?)
    }

    /// THE HOUSE ANSWERS A CALL with its outcome.
    pub fn post_outcome(&self, house: &SigningKey, o: Outcome) -> io::Result<Result<u64, Refusal>> {
        self.seat_post(&Post::Outcome(o).sign(house).ok_or_else(unwritten)?)
    }
}
