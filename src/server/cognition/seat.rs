// 78.145 — THE SEAT RELAY: a court holds each domain's latest seat listing
// and the mail between a house and the minds seated in it, and forwards it.
// It judges nothing in-game: it checks only that a post is signed by whom it
// must be — the domain's sitting reserve for the house's posts, the named
// mind for a take or a call.
//
//! VOLATILE AND BOUNDED (a sink keeps its bound). Nothing here is persisted:
//! a restart holds no listing and no mail, and a reader whose `after` stands
//! past the relay's last sequence reads from the start. Per addressee, a
//! READ-MAIL's `after` acknowledges — and drops — everything at or below it,
//! and what is held never passes `MAIL_BOUND` bytes as a reply writes them:
//! the frame budget less the reply's count, so one READ-MAIL always carries a
//! whole box. Past it the oldest piece is dropped and counted (`dropped`); the
//! reader sees the gap in the sequence.

use crate::anastome::dialogue::seat::{Mail, Post, ReadMail, Refusal, Signer, SignedPost};
use crate::network::nerve::Keyed;

/// THE BOUND ON ONE ADDRESSEE'S MAIL, in reply bytes: the frame budget less
/// the reply's `[count u16]` — derived, not declared.
pub fn mail_bound() -> usize {
    crate::network::wire::frame_budget() - 2
}

/// The box a mind's mail is filed under, and a domain's (its house's).
const TO_MIND: u8 = 0;
const TO_HOUSE: u8 = 1;

fn box_key(to: u8, who: &[u8; 32]) -> [u8; 33] {
    let mut k = [0u8; 33];
    k[0] = to;
    k[1..].copy_from_slice(who);
    k
}

/// One addressee's mail in sequence order, and the bytes a reply takes for it.
#[derive(Default)]
pub struct Mailbox {
    held: std::collections::VecDeque<Mail>,
    bytes: usize,
}

/// THE RELAY'S CARRIERS.
#[derive(Default)]
pub struct SeatRelay {
    /// Each domain's latest SEATS: its epoch and its body (fields ‖ sig).
    listings: Keyed<(u64, Vec<u8>)>,
    /// Each addressee's mail.
    mail: Keyed<Mailbox>,
    /// The last sequence issued.
    seq: u64,
    /// Pieces dropped unread at the bound, since boot.
    pub dropped: u64,
}

impl SeatRelay {
    /// File a piece, keeping the box within its bound and its count within a
    /// reply's u16.
    fn file(&mut self, key: &[u8; 33], piece: Mail) {
        let len = piece.wire_len();
        if len > mail_bound() {
            self.dropped += 1;
            return;
        }
        if !self.mail.contains(key) {
            self.mail.insert(key, Mailbox::default());
        }
        let Some(b) = self.mail.get_mut(key) else { return };
        while b.bytes + len > mail_bound() || b.held.len() >= usize::from(u16::MAX) {
            let Some(old) = b.held.pop_front() else { break };
            b.bytes -= old.wire_len();
            self.dropped += 1;
        }
        b.bytes += len;
        b.held.push_back(piece);
    }

    /// Drop what `after` acknowledges in one box; what remains, in order.
    fn read_box(&mut self, key: &[u8; 33], after: u64) -> Vec<Mail> {
        let Some(b) = self.mail.get_mut(key) else { return Vec::new() };
        while b.held.front().is_some_and(|m| m.seq <= after) {
            if let Some(old) = b.held.pop_front() {
                b.bytes -= old.wire_len();
            }
        }
        let left: Vec<Mail> = b.held.iter().cloned().collect();
        if b.held.is_empty() {
            self.mail.remove(key);
        }
        left
    }

    /// The census: listings held, boxes, pieces of mail.
    pub fn census(&self) -> (usize, usize, usize) {
        (self.listings.len(), self.mail.len(), self.mail.scoped().map(|(_, b)| b.held.len()).sum())
    }

    /// The pieces held for one addressee (the bound's reading).
    pub fn held_bytes(&self, to_house: bool, who: &[u8; 32]) -> usize {
        let key = box_key(if to_house { TO_HOUSE } else { TO_MIND }, who);
        self.mail.get(&key).map_or(0, |b| b.bytes)
    }
}

impl super::Kernel {
    /// 78.145 — A SEAT POST: checked for its signer, then held (a listing)
    /// or filed for its addressee (a take or a call to the domain's house; a
    /// taken, percept or outcome to the mind it names). Answered by the
    /// sequence it was given, or the refusal by name.
    pub fn seat_post(&mut self, s: &SignedPost) -> Result<u64, Refusal> {
        let domain = s.post.domain();
        let house = self.reserve_of(&domain).ok_or(Refusal::UnknownDomain)?;
        match s.post.signer() {
            Signer::House if !s.signed_by(&house) => return Err(Refusal::NotTheHouse),
            Signer::Mind(m) if !s.signed_by(&m) => return Err(Refusal::Unsigned),
            _ => {}
        }
        let body = s.body().ok_or(Refusal::Malformed)?;
        let relay = &mut self.seats;
        relay.seq += 1;
        let seq = relay.seq;
        let sub = s.post.sub();
        match &s.post {
            Post::Seats(l) => {
                // The latest epoch replaces; an older one is acknowledged and
                // not held.
                let held = relay.listings.get(&domain).map(|(e, _)| *e);
                if held.is_none_or(|e| l.epoch >= e) {
                    relay.listings.insert(domain, (l.epoch, body));
                }
            }
            Post::Take(_) | Post::Call(_) => relay.file(&box_key(TO_HOUSE, &domain), Mail { seq, sub, body }),
            Post::Taken(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
            Post::Percept(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
            Post::Outcome(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
        }
        Ok(seq)
    }

    /// 78.145 — A DOMAIN'S LATEST SEATS (fields ‖ sig), or none.
    pub fn seat_listing(&self, domain: &[u8; 32]) -> Option<Vec<u8>> {
        self.seats.listings.get(domain).map(|(_, b)| b.clone())
    }

    /// 78.145 — A READER'S MAIL past `after`, in sequence order: a mind's
    /// own, and, where the reader is a domain's sitting reserve, that
    /// domain's house mail. `after` acknowledges and drops everything at or
    /// below it; an `after` past the last sequence reads from the start (the
    /// relay restarted under it).
    pub fn seat_mail(&mut self, r: &ReadMail) -> Result<Vec<Mail>, Refusal> {
        if !r.verified() {
            return Err(Refusal::Unsigned);
        }
        let after = if r.after > self.seats.seq { 0 } else { r.after };
        let mine = self.seats.read_box(&box_key(TO_MIND, &r.reader), after);
        let house = match self.tier_domain.get(&r.reader).copied() {
            Some(d) if self.reserve_of(&d) == Some(r.reader) => self.seats.read_box(&box_key(TO_HOUSE, &d), after),
            _ => Vec::new(),
        };
        // Two runs, each in sequence order, merged by sequence: the relay's
        // own order, never a measurement. The reply holds both boxes, so
        // where both are near the bound it is cut at the frame budget and the
        // rest stands for the next read.
        let mut out = Vec::with_capacity(mine.len() + house.len());
        let (mut a, mut b) = (mine.into_iter().peekable(), house.into_iter().peekable());
        let mut bytes = 0usize;
        loop {
            let take_a = match (a.peek(), b.peek()) {
                (Some(x), Some(y)) => x.seq < y.seq,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => break,
            };
            let next = if take_a { a.next() } else { b.next() };
            let Some(m) = next else { break };
            if bytes + m.wire_len() > mail_bound() || out.len() >= usize::from(u16::MAX) {
                break;
            }
            bytes += m.wire_len();
            out.push(m);
        }
        Ok(out)
    }
}
