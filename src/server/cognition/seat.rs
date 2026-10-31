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
//!
//! v1.4 — THE RELAY'S EPOCH: a value drawn once when the relay is made, so
//! every hypha restart changes it; a sub-kind 8 read is answered with it and
//! the last sequence issued, and a reader tells a restart from progress.
//! Listings are held per (domain, sub-origin).

use crate::anastome::dialogue::seat::{Mail, MailRead, Post, ReadMail, Refusal, Signer, SignedPost};
use crate::network::nerve::Keyed;

/// THE BOUND ON ONE ADDRESSEE'S MAIL, in reply bytes: the frame budget less
/// the reply's head before its pieces — v1.4's `[1][epoch][seq][count]`,
/// which covers v1.3's `[count]` — derived, not declared.
pub fn mail_bound() -> usize {
    crate::network::wire::frame_budget() - MailRead::HEAD
}

/// A LISTING'S COORDINATES: `[0][domain]` in the domain's own universe,
/// `[1][domain][sub_origin]` in a sub-universe of it.
fn listing_key(domain: &[u8; 32], sub_origin: Option<&[u8; 32]>) -> Vec<u8> {
    let mut k = Vec::with_capacity(65);
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
    k
}

/// THE RELAY'S EPOCH, drawn from the operating system's randomness when the
/// relay is made: no clock, and no two boots alike. Never 0, which a reader
/// names when it holds no epoch.
fn draw_epoch() -> u64 {
    use rand_core::{OsRng, RngCore};
    match OsRng.next_u64() {
        0 => 1,
        e => e,
    }
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
    /// v1.4 — the relay's epoch: drawn when it is made (`SharedRelay`).
    epoch: u64,
    /// Each (domain, sub-origin)'s latest SEATS: its epoch and its body
    /// (fields ‖ sig), at `listing_key`.
    listings: Keyed<(u64, Vec<u8>)>,
    /// Each addressee's mail.
    mail: Keyed<Mailbox>,
    /// The last sequence issued.
    seq: u64,
    /// Pieces dropped unread at the bound, since boot.
    pub dropped: u64,
    /// 78.146 — THE EXEMPT READ'S WINDOW: per mind whose box holds mail, the
    /// relay sequence its last exempt read stood at. Held only while the box
    /// is, so it is bounded by the boxes.
    exempt_at: Keyed<u64>,
    /// THE POSTS SINCE BOOT, per domain with a sitting reserve here: filed
    /// by kind, refused by reason (`observe::Relay`). Bounded by the domains
    /// founded.
    posts: Keyed<([u64; 6], [u64; 3])>,
    /// Posts naming a domain with no reserve here, and seat frames that did
    /// not read far enough to name one: each one count.
    unknown: u64,
    unread: u64,
}

/// A post's kind, in `observe::POST_KINDS` order.
fn kind_of(p: &Post) -> usize {
    match p {
        Post::Seats(_) => 0,
        Post::Take(_) => 1,
        Post::Taken(_) => 2,
        Post::Percept(_) => 3,
        Post::Call(_) => 4,
        Post::Outcome(_) => 5,
    }
}

/// A refusal's place in `observe::POST_REFUSALS`; `None` for UnknownDomain,
/// which names no counted domain.
fn reason_of(r: Refusal) -> Option<usize> {
    match r {
        Refusal::Unsigned => Some(0),
        Refusal::NotTheHouse => Some(1),
        Refusal::Malformed => Some(2),
        Refusal::UnknownDomain => None,
    }
}

/// 78.146 — THE RELAY ON ITS OWN LOCK, so a door can serve a mind's read
/// without waiting on the court's cognition (`exempt_read`); the kernel holds
/// it and speaks to it through the same lock.
#[derive(Clone)]
pub struct SharedRelay(std::sync::Arc<std::sync::Mutex<SeatRelay>>);

impl Default for SharedRelay {
    /// A relay made with its epoch drawn: a restart is a new epoch.
    fn default() -> SharedRelay {
        SharedRelay(std::sync::Arc::new(std::sync::Mutex::new(SeatRelay { epoch: draw_epoch(), ..SeatRelay::default() })))
    }
}

impl SharedRelay {
    /// v1.4 — the relay's epoch and the last sequence it issued.
    pub fn standing(&self) -> (u64, u64) {
        let r = self.lock();
        (r.epoch, r.seq)
    }

    /// The relay, locked. A poisoned lock still holds a whole relay — every
    /// write to it completes under the lock or not at all — and the relay is
    /// volatile, so it is read on.
    pub fn lock(&self) -> std::sync::MutexGuard<'_, SeatRelay> {
        match self.0.lock() {
            Ok(g) => g,
            Err(p) => p.into_inner(),
        }
    }

    pub fn census(&self) -> (usize, usize, usize) {
        self.lock().census()
    }

    /// 78.145, 78.151 — A DOMAIN'S LATEST SEATS (fields ‖ sig), in its own
    /// universe or (v1.4) at a sub-origin, or none, read on the relay's own
    /// lock: a listing is relay state, never the court's, so its read takes
    /// no kernel.
    pub fn listing(&self, domain: &[u8; 32], sub_origin: Option<&[u8; 32]>) -> Option<Vec<u8>> {
        self.lock().listings.get(&listing_key(domain, sub_origin)).map(|(_, b)| b.clone())
    }

    pub fn held_bytes(&self, to_house: bool, who: &[u8; 32]) -> usize {
        self.lock().held_bytes(to_house, who)
    }

    pub fn dropped(&self) -> u64 {
        self.lock().dropped
    }

    /// A seat frame that did not read far enough to name a domain.
    pub fn note_unread(&self) {
        let mut r = self.lock();
        r.unread = r.unread.wrapping_add(1);
    }

    /// 78.145 — THE RELAY'S READING for the observer, read on its own lock.
    pub fn reading(&self) -> crate::observe::Relay {
        self.lock().reading()
    }

    /// 78.146 — A MIND'S READ, EXEMPT FROM THE DOOR'S DEFERRAL: reading one's
    /// own mail spends nothing. Served from the mind's own box alone (a
    /// house's box is read through the court, which knows its reserve), and
    /// BOUNDED: one exempt read per reader per window of its own mail — a
    /// reader whose box holds a piece filed since its last exempt read.
    /// `None` where it is not exempt (no box, or the window spent): the
    /// door's ordinary rule answers it.
    /// WHETHER ANYTHING WAS FILED PAST THE READER'S CURSOR: the relay's last
    /// sequence beyond what the read acknowledges, on the relay's epoch — a
    /// read with nothing past it waits for the court's next event (the
    /// server's held read). Any box counts: a house reads its own besides its
    /// key's.
    pub fn filed_past(&self, r: &ReadMail) -> bool {
        let relay = self.lock();
        relay.seq > r.after_on(relay.epoch, relay.seq)
    }

    pub fn exempt_read(&self, r: &ReadMail) -> Option<Result<MailRead, Refusal>> {
        if !r.verified() {
            return Some(Err(Refusal::Unsigned));
        }
        let mut relay = self.lock();
        let key = box_key(TO_MIND, &r.reader);
        // The window is the reader's own: the newest piece in its box.
        let newest = relay.mail.get(&key).and_then(|b| b.held.back()).map(|m| m.seq)?;
        if relay.exempt_at.get(&key).is_some_and(|at| *at >= newest) {
            return None;
        }
        relay.exempt_at.insert(key, newest);
        let seq = relay.seq;
        let after = r.after_on(relay.epoch, seq);
        let mut out = Vec::new();
        let mut bytes = 0usize;
        for m in relay.read_box(&key, after) {
            if bytes + m.wire_len() > mail_bound() || out.len() >= usize::from(u16::MAX) {
                break;
            }
            bytes += m.wire_len();
            out.push(m);
        }
        Some(Ok(MailRead { epoch: relay.epoch, seq, mail: out }))
    }
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
            self.exempt_at.remove(key);
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

    /// One post counted against its domain: filed under its kind, or
    /// refused for its reason.
    fn count(&mut self, domain: &[u8; 32], filed: Option<usize>, refused: Option<usize>) {
        if !self.posts.contains(domain) {
            self.posts.insert(domain, ([0; 6], [0; 3]));
        }
        let Some((f, r)) = self.posts.get_mut(domain) else { return };
        if let Some(n) = filed.and_then(|i| f.get_mut(i)) {
            *n = n.wrapping_add(1);
        }
        if let Some(n) = refused.and_then(|i| r.get_mut(i)) {
            *n = n.wrapping_add(1);
        }
    }

    /// 78.145 — THE RELAY'S READING: its standing, what it holds, and each
    /// counted domain's posts.
    pub fn reading(&self) -> crate::observe::Relay {
        let as_u64 = |n: usize| match u64::try_from(n) {
            Ok(v) => v,
            Err(_) => u64::MAX,
        };
        let (listings, boxes, pieces) = self.census();
        let bytes: usize = self.mail.scoped().map(|(_, b)| b.bytes).sum();
        let domains = self
            .posts
            .scoped()
            .filter_map(|(k, (filed, refused))| Some(crate::observe::RelayPosts { domain: k.as_slice().try_into().ok()?, filed: *filed, refused: *refused }))
            .collect();
        crate::observe::Relay {
            epoch: self.epoch,
            seq: self.seq,
            listings: as_u64(listings),
            boxes: as_u64(boxes),
            pieces: as_u64(pieces),
            bytes: as_u64(bytes),
            dropped: self.dropped,
            unknown: self.unknown,
            unread: self.unread,
            domains,
        }
    }
}

impl super::Court {
    /// 78.145 — A SEAT POST: checked for its signer, then held (a listing)
    /// or filed for its addressee (a take or a call to the domain's house; a
    /// taken, percept or outcome to the mind it names). Answered by the
    /// sequence it was given, or the refusal by name — and counted on the
    /// relay's reading either way.
    pub fn seat_post(&mut self, s: &SignedPost) -> Result<u64, Refusal> {
        let domain = s.post.domain();
        let Some(house) = self.reserve_of(&domain) else {
            let mut relay = self.seats.lock();
            relay.unknown = relay.unknown.wrapping_add(1);
            return Err(Refusal::UnknownDomain);
        };
        let checked = match s.post.signer() {
            Signer::House if !s.signed_by(&house) => Err(Refusal::NotTheHouse),
            Signer::Mind(m) if !s.signed_by(&m) => Err(Refusal::Unsigned),
            _ => s.body().ok_or(Refusal::Malformed),
        };
        let mut guard = self.seats.lock();
        let relay = &mut *guard;
        let body = match checked {
            Ok(body) => body,
            Err(why) => {
                relay.count(&domain, None, reason_of(why));
                return Err(why);
            }
        };
        relay.count(&domain, Some(kind_of(&s.post)), None);
        relay.seq += 1;
        let seq = relay.seq;
        let sub = s.post.sub();
        match &s.post {
            Post::Seats(l) => {
                // The latest epoch replaces, per (domain, sub-origin); an
                // older one is acknowledged and not held.
                let key = listing_key(&domain, l.sub_origin.as_ref());
                let held = relay.listings.get(&key).map(|(e, _)| *e);
                if held.is_none_or(|e| l.epoch >= e) {
                    relay.listings.insert(key, (l.epoch, body));
                }
            }
            Post::Take(_) | Post::Call(_) => relay.file(&box_key(TO_HOUSE, &domain), Mail { seq, sub, body }),
            Post::Taken(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
            Post::Percept(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
            Post::Outcome(t) => relay.file(&box_key(TO_MIND, &t.mind), Mail { seq, sub, body }),
        }
        Ok(seq)
    }

    /// 78.145 — A DOMAIN'S LATEST SEATS (fields ‖ sig), in its own universe
    /// or (v1.4) at a sub-origin, or none.
    pub fn seat_listing(&self, domain: &[u8; 32], sub_origin: Option<&[u8; 32]>) -> Option<Vec<u8>> {
        self.seats.listing(domain, sub_origin)
    }

    /// 78.146 — the relay on its own lock, for a door's exempt read.
    pub fn seat_relay(&self) -> SharedRelay {
        self.seats.clone()
    }

    /// 78.145 — A READER'S MAIL past `after`, in sequence order: a mind's
    /// own, and, where the reader is a domain's sitting reserve, that
    /// domain's house mail. `after` acknowledges and drops everything at or
    /// below it; an `after` past the last sequence reads from the start (the
    /// relay restarted under it). v1.4: answered with the relay's epoch and
    /// last sequence, read in the same hold as the pieces.
    pub fn seat_mail(&mut self, r: &ReadMail) -> Result<MailRead, Refusal> {
        if !r.verified() {
            return Err(Refusal::Unsigned);
        }
        let house_of = match self.tier_domain.get(&r.reader).copied() {
            Some(d) if self.reserve_of(&d) == Some(r.reader) => Some(d),
            _ => None,
        };
        let mut relay = self.seats.lock();
        let (epoch, seq) = (relay.epoch, relay.seq);
        let after = r.after_on(epoch, seq);
        let to_mind = relay.read_box(&box_key(TO_MIND, &r.reader), after);
        let house = match house_of {
            Some(d) => relay.read_box(&box_key(TO_HOUSE, &d), after),
            None => Vec::new(),
        };
        drop(relay);
        // Two runs, each in sequence order, merged by sequence: the relay's
        // own order, never a measurement. The reply holds both boxes, so
        // where both are near the bound it is cut at the frame budget and the
        // rest stands for the next read.
        let mut out = Vec::with_capacity(to_mind.len() + house.len());
        let (mut a, mut b) = (to_mind.into_iter().peekable(), house.into_iter().peekable());
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
        Ok(MailRead { epoch, seq, mail: out })
    }
}
