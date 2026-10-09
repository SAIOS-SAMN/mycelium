// 78.154 — THE MAP AND THE LEDGER on the court (MU-ECONOMY §7.1, the
// operator's and the coordinator's rulings 2026-10-09; 78.156): a universe's
// LEDGER chain — its committed cycle `u` paired with the mesh's final point —
// and its MAP — its mouths and segments placed in its own space, each at the
// final point it was mapped at — posted by its DOMAIN RESERVE as sub-kinds of
// ACT_DOMAIN.
//
//! Sub-kinds 8–11 of ACT_DOMAIN are the universe's statements (78.139/140:
//! GEOMETRY, BASKET, JOIN, READ-UNIVERSE), so the LEDGER proposed as 8 and
//! READ-LEDGER as 9 take the next free numbers: LEDGER 12, READ-LEDGER 13,
//! MAP 14, READ-MAP 15, TRAVERSE 16 (`crossing.rs`). The ruled bytes are
//! otherwise exactly as ruled.

use crate::chronometry::tuniversal::TPoint;

/// ACT_DOMAIN's sub-kind mark (as `domain.rs` writes it).
const SUB_KIND: [u8; 2] = [0x80, 0x00];
pub const KIND_LEDGER: u8 = 12;
pub const KIND_READ_LEDGER: u8 = 13;
pub const KIND_MAP: u8 = 14;
pub const KIND_READ_MAP: u8 = 15;
pub const KIND_TRAVERSE: u8 = 16;

const LEDGER: &[u8] = b"mycelium domain ledger v1";
const MAP: &[u8] = b"mycelium domain map v1";

pub(crate) fn key32(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(k)
}

pub(crate) fn u64_at(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

pub(crate) fn byte(b: &[u8], p: &mut usize) -> Option<u8> {
    let x = *b.get(*p)?;
    *p += 1;
    Some(x)
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][kind]`.
pub(crate) fn head(domain: &[u8; 32], kind: u8) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(kind);
    out
}

/// 78.154 (3) — the crossings a receiver APPLIED, committed in its next
/// entry: each crossing's hash and the `u` it was transduced at.
pub type Applied = Vec<([u8; 32], u64)>;

fn put_applied(out: &mut Vec<u8>, applied: &Applied) {
    crate::transducer::address::put_key_count(out, applied.len());
    for (c, u) in applied {
        out.extend_from_slice(c);
        out.extend_from_slice(&u.to_le_bytes());
    }
}

fn take_applied(b: &[u8], p: &mut usize) -> Option<Applied> {
    (0..crate::transducer::address::take_key_count(b, p)?).map(|_| Some((key32(b, p)?, u64_at(b, p)?))).collect()
}

/// 78.154 — ONE LEDGER ENTRY of a universe: its committed cycle `u`, its
/// state COMMITMENT and EVENTS digest (`commitment.rs`; Concordance 1.13),
/// the final point it was made under, the previous entry's hash (zero
/// first), the crossings it applied (78.154 (3); empty for most), and a
/// nonce. Signed by the domain's RESERVE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LedgerEntry {
    pub domain: [u8; 32],
    pub u: u64,
    pub commitment: [u8; 32],
    pub events: [u8; 32],
    pub point: TPoint,
    pub parent: [u8; 32],
    pub applied: Applied,
    pub nonce: [u8; 32],
}

impl LedgerEntry {
    /// `"mycelium domain ledger v1" ‖ domain ‖ u ‖ commitment ‖ events ‖
    /// point ‖ parent ‖ nonce`, and the applied list after it only where it
    /// is not empty.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = LEDGER.to_vec();
        out.extend_from_slice(&self.domain);
        out.extend_from_slice(&self.u.to_le_bytes());
        out.extend_from_slice(&self.commitment);
        out.extend_from_slice(&self.events);
        self.point.put(&mut out);
        out.extend_from_slice(&self.parent);
        out.extend_from_slice(&self.nonce);
        if !self.applied.is_empty() {
            put_applied(&mut out, &self.applied);
        }
        out
    }

    /// THE ENTRY'S HASH — what its successor names as `parent`: the
    /// citation hash of exactly its signed bytes.
    pub fn hash(&self) -> [u8; 32] {
        crate::chronometry::payload_hash(&self.signed_bytes())
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedLedger {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedLedger { entry: self, sig }
    }

    /// The body as stored and served: `[u][commitment][events][point]
    /// [parent][nonce][applied]`.
    pub fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.u.to_le_bytes());
        out.extend_from_slice(&self.commitment);
        out.extend_from_slice(&self.events);
        self.point.put(out);
        out.extend_from_slice(&self.parent);
        out.extend_from_slice(&self.nonce);
        put_applied(out, &self.applied);
    }

    pub fn take(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<LedgerEntry> {
        let u = u64_at(b, p)?;
        let (commitment, events) = (key32(b, p)?, key32(b, p)?);
        let point = TPoint::take(b, p)?;
        let (parent, nonce) = (key32(b, p)?, key32(b, p)?);
        Some(LedgerEntry { domain, u, commitment, events, point, parent, applied: take_applied(b, p)?, nonce })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedLedger {
    pub entry: LedgerEntry,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][12][u u64 LE][commitment 32][events
/// 32][cited final TPoint 84][parent entry 32][nonce 32][sig 64]`, then —
/// only where it applied crossings — `[count][(crossing 32, u_applied u64
/// LE) …]`, signed.
pub fn encode_ledger(s: &SignedLedger) -> Vec<u8> {
    let e = &s.entry;
    let mut out = head(&e.domain, KIND_LEDGER);
    out.extend_from_slice(&e.u.to_le_bytes());
    out.extend_from_slice(&e.commitment);
    out.extend_from_slice(&e.events);
    e.point.put(&mut out);
    out.extend_from_slice(&e.parent);
    out.extend_from_slice(&e.nonce);
    out.extend_from_slice(&s.sig);
    if !e.applied.is_empty() {
        put_applied(&mut out, &e.applied);
    }
    out
}

/// The entry after the sub-kind byte, read whole; an empty applied list is
/// never written, so a count of zero is no entry.
pub(crate) fn take_ledger(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedLedger> {
    let u = u64_at(b, p)?;
    let (commitment, events) = (key32(b, p)?, key32(b, p)?);
    let point = TPoint::take(b, p)?;
    let (parent, nonce) = (key32(b, p)?, key32(b, p)?);
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    let applied = match *p < b.len() {
        true => Some(take_applied(b, p)?).filter(|a| !a.is_empty())?,
        false => Vec::new(),
    };
    Some(SignedLedger { entry: LedgerEntry { domain, u, commitment, events, point, parent, applied, nonce }, sig })
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][13][after u u64 LE]`.
pub fn encode_ledger_read(domain: [u8; 32], after: u64) -> Vec<u8> {
    let mut out = head(&domain, KIND_READ_LEDGER);
    out.extend_from_slice(&after.to_le_bytes());
    out
}

/// WHY A LEDGER ENTRY WAS NOT SEALED (the ruled codes 1–6; 7–9 added).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LedgerRefusal {
    /// 1 — signed by a key of the domain, but not its sitting reserve; or
    /// the domain has no reserve.
    NotReserve,
    /// 2 — no such domain founded on this court.
    UnknownDomain,
    /// 3 — the parent is not the chain's head: a fork, or a first entry
    /// naming a parent.
    Parent,
    /// 4 — `u` not strictly above the head's.
    NotAbove,
    /// 5 — the cited point is not final in this court's view, or above it.
    NotFinal,
    /// 6 — the signature verifies under no key of the domain.
    Unsigned,
    /// 7 — the cited point stands below the head entry's (h only rises).
    Behind,
    /// 8 — an applied crossing is not one pending into this universe, or
    /// was applied above this entry's `u`.
    Applied,
    /// 9 — the court's line could not take it.
    LineRefused,
}

impl LedgerRefusal {
    const ALL: [LedgerRefusal; 9] = [
        LedgerRefusal::NotReserve,
        LedgerRefusal::UnknownDomain,
        LedgerRefusal::Parent,
        LedgerRefusal::NotAbove,
        LedgerRefusal::NotFinal,
        LedgerRefusal::Unsigned,
        LedgerRefusal::Behind,
        LedgerRefusal::Applied,
        LedgerRefusal::LineRefused,
    ];

    pub fn code(self) -> u8 {
        match self {
            LedgerRefusal::NotReserve => 1,
            LedgerRefusal::UnknownDomain => 2,
            LedgerRefusal::Parent => 3,
            LedgerRefusal::NotAbove => 4,
            LedgerRefusal::NotFinal => 5,
            LedgerRefusal::Unsigned => 6,
            LedgerRefusal::Behind => 7,
            LedgerRefusal::Applied => 8,
            LedgerRefusal::LineRefused => 9,
        }
    }

    pub fn of(code: u8) -> Option<LedgerRefusal> {
        LedgerRefusal::ALL.into_iter().find(|r| r.code() == code)
    }
}

/// WHAT THE COURT ANSWERED an entry: sealed — the entry's hash, the height
/// it was sealed at and the court's final point then — or refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ledgered {
    Sealed { entry: [u8; 32], h: u64, point: TPoint },
    Refused(LedgerRefusal),
}

/// `[1][entry hash 32][sealed h u64 LE][court's final TPoint 84]` or
/// `[0][reason u8]`.
pub fn encode_ledger_reply(r: &Ledgered) -> Vec<u8> {
    match r {
        Ledgered::Sealed { entry, h, point } => {
            let mut out = vec![1];
            out.extend_from_slice(entry);
            out.extend_from_slice(&h.to_le_bytes());
            point.put(&mut out);
            out
        }
        Ledgered::Refused(why) => vec![0, why.code()],
    }
}

pub fn decode_ledger_reply(b: &[u8]) -> Option<Ledgered> {
    match b {
        [0, code] => Some(Ledgered::Refused(LedgerRefusal::of(*code)?)),
        [1, rest @ ..] => {
            let mut p = 0;
            let entry = key32(rest, &mut p)?;
            let h = u64_at(rest, &mut p)?;
            let point = TPoint::take(rest, &mut p)?;
            (p == rest.len()).then_some(Ledgered::Sealed { entry, h, point })
        }
        _ => None,
    }
}

/// 78.154 — AN ENTRY AS THE COURT HOLDS IT: the entry, its hash, the height
/// it was sealed at and the court's final point then; and, served, the
/// DILATION of the segment it closes — `Δu/Δh` from its parent, reduced
/// exactly; `None` for the first entry or where `h` did not move (78.156 §2).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeldEntry {
    pub entry: LedgerEntry,
    pub hash: [u8; 32],
    pub sealed: u64,
    pub court: TPoint,
    pub dilation: Option<(u64, u64)>,
}

impl HeldEntry {
    pub fn put(&self, out: &mut Vec<u8>) {
        self.entry.put(out);
        out.extend_from_slice(&self.hash);
        out.extend_from_slice(&self.sealed.to_le_bytes());
        self.court.put(out);
        match self.dilation {
            None => out.push(0),
            Some((n, d)) => {
                out.push(1);
                out.extend_from_slice(&n.to_le_bytes());
                out.extend_from_slice(&d.to_le_bytes());
            }
        }
    }

    pub fn take(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<HeldEntry> {
        let entry = LedgerEntry::take(domain, b, p)?;
        let hash = key32(b, p)?;
        let sealed = u64_at(b, p)?;
        let court = TPoint::take(b, p)?;
        let dilation = match byte(b, p)? {
            0 => None,
            1 => Some((u64_at(b, p)?, u64_at(b, p)?)),
            _ => return None,
        };
        Some(HeldEntry { entry, hash, sealed, court, dilation })
    }
}

/// THE DILATION of a segment, `Δu/Δh` reduced; `None` where `h` did not
/// move or a difference does not read.
pub fn dilation(u: (u64, u64), h: (u64, u64)) -> Option<(u64, u64)> {
    let du = u.1.checked_sub(u.0)?;
    let dh = h.1.checked_sub(h.0).filter(|d| *d > 0)?;
    let g = num_integer::Integer::gcd(&du, &dh);
    Some((du / g, dh / g))
}

/// A READ-LEDGER ANSWER: the entries after `u`, in chain order, as many as
/// fit one frame, and whether more stand.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LedgerPage {
    pub entries: Vec<HeldEntry>,
    pub more: bool,
}

/// `[more u8][count][held entry …]`.
pub fn encode_ledger_page(page: &LedgerPage) -> Vec<u8> {
    let mut out = vec![u8::from(page.more)];
    crate::transducer::address::put_key_count(&mut out, page.entries.len());
    for e in &page.entries {
        e.put(&mut out);
    }
    out
}

pub fn decode_ledger_page(domain: [u8; 32], b: &[u8]) -> Option<LedgerPage> {
    let mut p = 0;
    let more = match byte(b, &mut p)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    let entries = (0..crate::transducer::address::take_key_count(b, &mut p)?).map(|_| HeldEntry::take(domain, b, &mut p)).collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(LedgerPage { entries, more })
}

/// 78.154 — WHAT A SEGMENT IS: a road inside the universe, a core's road, or
/// a wormhole's THROAT (its two mouths' sides).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Segment {
    Tube,
    Core,
    Throat,
}

impl Segment {
    pub fn byte(self) -> u8 {
        match self {
            Segment::Tube => 0,
            Segment::Core => 1,
            Segment::Throat => 2,
        }
    }

    pub fn of(b: u8) -> Option<Segment> {
        match b {
            0 => Some(Segment::Tube),
            1 => Some(Segment::Core),
            2 => Some(Segment::Throat),
            _ => None,
        }
    }
}

/// A MAPPED PLACE: a 78.128 mouth pair by its id, or a LOCAL THROAT by its
/// derived mouth id (`local_throat_id`), and its side (0 and 1 its two ends;
/// a mouth pair's 0 the offering end, 1 the accepting end).
pub type Place = ([u8; 32], u8);

const LOCAL_THROAT: &str = "mycelium local throat v1";

/// 78.154 — A LOCAL THROAT'S MOUTH ID, derived and never posted:
/// `derive_key("mycelium local throat v1", domain ‖ throat key)`.
pub fn local_throat_id(domain: &[u8; 32], throat: &[u8; 32]) -> [u8; 32] {
    let mut m = domain.to_vec();
    m.extend_from_slice(throat);
    blake3::derive_key(LOCAL_THROAT, &m)
}

/// The id a local throat's record is held under, from its derived mouth id:
/// the citation hash of `"map record" ‖ domain ‖ [2] ‖ local id`.
pub fn local_record_id(domain: &[u8; 32], local: &[u8; 32]) -> [u8; 32] {
    let mut b = b"map record".to_vec();
    b.extend_from_slice(domain);
    b.push(2);
    b.extend_from_slice(local);
    crate::chronometry::payload_hash(&b)
}

/// 78.154 — ONE MAP RECORD. A MOUTH: the mouth pair and its side in this
/// universe, and its FRESHNESS bound `N` (78.156 §4: how many final-point
/// advances behind a crossing's point this universe's latest entry may
/// stand). Its place in the universe's own space is read off the mouth's
/// tube (78.140), never restated. A SEGMENT: its kind, its two places, its
/// span in geometrized megaparsecs exact, and — a throat — its keeper (the
/// road toll itself is in-universe, never here). A LOCAL THROAT: an
/// in-universe throat (a progenitor tube, core to galaxy, a long haul) —
/// its throat key, its keeper (the progenitor line's key, carried inline),
/// its two ends' FRAMES and its `N`; both ends stand in the posting
/// universe, each place read off its frame in the universe's declared
/// geometry. No 78.128 act and no court bond: the map record is all of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapRecord {
    Mouth { place: Place, freshness: u64 },
    Segment { kind: Segment, a: Place, b: Place, span: (u64, u64), keeper: Option<[u8; 32]> },
    LocalThroat { throat: [u8; 32], keeper: [u8; 32], frames: (crate::transducer::cga::Mv, crate::transducer::cga::Mv), freshness: u64 },
}

fn put_place(out: &mut Vec<u8>, (m, side): &Place) {
    out.extend_from_slice(m);
    out.push(*side);
}

fn take_place(b: &[u8], p: &mut usize) -> Option<Place> {
    Some((key32(b, p)?, byte(b, p)?))
}

impl MapRecord {
    pub fn put(&self, out: &mut Vec<u8>) {
        match self {
            MapRecord::Mouth { place, freshness } => {
                out.push(0);
                put_place(out, place);
                out.extend_from_slice(&freshness.to_le_bytes());
            }
            MapRecord::Segment { kind, a, b, span, keeper } => {
                out.push(1);
                out.push(kind.byte());
                put_place(out, a);
                put_place(out, b);
                out.extend_from_slice(&span.0.to_le_bytes());
                out.extend_from_slice(&span.1.to_le_bytes());
                match keeper {
                    None => out.push(0),
                    Some(k) => {
                        out.push(1);
                        out.extend_from_slice(k);
                    }
                }
            }
            MapRecord::LocalThroat { throat, keeper, frames, freshness } => {
                out.push(2);
                out.extend_from_slice(throat);
                out.extend_from_slice(keeper);
                frames.0.to_bytes(out);
                frames.1.to_bytes(out);
                out.extend_from_slice(&freshness.to_le_bytes());
            }
        }
    }

    /// A local throat's two ends read off its frames in `space`; `None`
    /// for any other record, or where a frame is no motion of that space.
    pub fn local_places(&self, space: &crate::anastome::dialogue::universe::Geometry) -> Option<(crate::transducer::cga::Mv, crate::transducer::cga::Mv)> {
        use crate::anastome::dialogue::tube::place_in;
        match self {
            MapRecord::LocalThroat { frames: (f0, f1), .. } => Some((place_in(space, f0)?, place_in(space, f1)?)),
            MapRecord::Mouth { .. } | MapRecord::Segment { .. } => None,
        }
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<MapRecord> {
        Some(match byte(b, p)? {
            0 => MapRecord::Mouth { place: take_place(b, p)?, freshness: u64_at(b, p)? },
            1 => {
                let kind = Segment::of(byte(b, p)?)?;
                let (a, bb) = (take_place(b, p)?, take_place(b, p)?);
                let span = (u64_at(b, p)?, u64_at(b, p)?);
                let keeper = match byte(b, p)? {
                    0 => None,
                    1 => Some(key32(b, p)?),
                    _ => return None,
                };
                MapRecord::Segment { kind, a, b: bb, span, keeper }
            }
            2 => {
                let (throat, keeper) = (key32(b, p)?, key32(b, p)?);
                let f0 = crate::transducer::cga::Mv::from_bytes(b, p)?;
                let f1 = crate::transducer::cga::Mv::from_bytes(b, p)?;
                MapRecord::LocalThroat { throat, keeper, frames: (f0, f1), freshness: u64_at(b, p)? }
            }
            _ => return None,
        })
    }

    /// THE RECORD'S ID: its universe, and a mouth by its place, a segment by
    /// its kind and its two places, a local throat by its derived mouth id
    /// (`local_record_id`) — what a later act supersedes, and what a route
    /// names (one throat mapped by both universes is two records).
    pub fn id(&self, domain: &[u8; 32]) -> [u8; 32] {
        let mut b = b"map record".to_vec();
        b.extend_from_slice(domain);
        match self {
            MapRecord::Mouth { place, .. } => {
                b.push(0);
                put_place(&mut b, place);
            }
            MapRecord::Segment { kind, a, b: bb, .. } => {
                b.push(1);
                b.push(kind.byte());
                put_place(&mut b, a);
                put_place(&mut b, bb);
            }
            MapRecord::LocalThroat { throat, .. } => return local_record_id(domain, &local_throat_id(domain, throat)),
        }
        crate::chronometry::payload_hash(&b)
    }
}

/// 78.154 — A MAP ACT: the domain, the final point it maps at, the record,
/// a nonce; signed by the domain's RESERVE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapAct {
    pub domain: [u8; 32],
    pub point: TPoint,
    pub record: MapRecord,
    pub nonce: [u8; 32],
}

impl MapAct {
    /// `"mycelium domain map v1" ‖ domain ‖ point ‖ record ‖ nonce`.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = MAP.to_vec();
        out.extend_from_slice(&self.domain);
        self.point.put(&mut out);
        self.record.put(&mut out);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedMap {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedMap { act: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedMap {
    pub act: MapAct,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][14][point 84][record][nonce 32][sig
/// 64]`; a record `[0][mouth 32][side u8][N u64 LE]` or `[1][kind u8][mouth
/// 32][side u8][mouth 32][side u8][span n u64][span d u64][0 | 1 keeper 32]`
/// or a local throat `[2][throat key 32][keeper 32][frame_0 Mv][frame_1
/// Mv][N u64 LE]`.
pub fn encode_map(s: &SignedMap) -> Vec<u8> {
    let mut out = head(&s.act.domain, KIND_MAP);
    s.act.point.put(&mut out);
    s.act.record.put(&mut out);
    out.extend_from_slice(&s.act.nonce);
    out.extend_from_slice(&s.sig);
    out
}

pub(crate) fn take_map(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedMap> {
    let point = TPoint::take(b, p)?;
    let record = MapRecord::take(b, p)?;
    let nonce = key32(b, p)?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    Some(SignedMap { act: MapAct { domain, point, record, nonce }, sig })
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][15]`.
pub fn encode_map_read(domain: [u8; 32]) -> Vec<u8> {
    head(&domain, KIND_READ_MAP)
}

/// WHY A MAP ACT WAS NOT SEALED (1, 2, 5, 6 as the ledger's).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapRefusal {
    NotReserve,
    UnknownDomain,
    NotFinal,
    Unsigned,
    /// 7 — no open tube of that id on this court.
    NoMouth,
    /// 8 — the side named does not stand in this universe.
    NotThisSide,
    /// 9 — a segment's end is not a mapped place of this universe (a
    /// throat: its own side), or a throat's two ends are not one mouth's two
    /// sides.
    Unmapped,
    /// 10 — a span not a positive exact rational, a side past 1, or a
    /// segment joining a place to itself.
    Malformed,
    /// 11 — a throat whose keeper is not its mouth's keeper, or a road that
    /// names one.
    Keeper,
    /// 12 — mapped at a point below the record's last mapping.
    Behind,
    /// 13 — the court's line could not take it.
    LineRefused,
    /// 14 — a local throat's frame is zero or no versor.
    NotVersor,
    /// 15 — a local throat in a universe that declared no geometry.
    NoGeometry,
    /// 16 — a local throat's end does not read a place in its universe's
    /// geometry (its frame no motion of that space).
    Unplaced,
}

impl MapRefusal {
    const ALL: [MapRefusal; 14] = [
        MapRefusal::NotReserve,
        MapRefusal::UnknownDomain,
        MapRefusal::NotFinal,
        MapRefusal::Unsigned,
        MapRefusal::NoMouth,
        MapRefusal::NotThisSide,
        MapRefusal::Unmapped,
        MapRefusal::Malformed,
        MapRefusal::Keeper,
        MapRefusal::Behind,
        MapRefusal::LineRefused,
        MapRefusal::NotVersor,
        MapRefusal::NoGeometry,
        MapRefusal::Unplaced,
    ];

    pub fn code(self) -> u8 {
        match self {
            MapRefusal::NotReserve => 1,
            MapRefusal::UnknownDomain => 2,
            MapRefusal::NotFinal => 5,
            MapRefusal::Unsigned => 6,
            MapRefusal::NoMouth => 7,
            MapRefusal::NotThisSide => 8,
            MapRefusal::Unmapped => 9,
            MapRefusal::Malformed => 10,
            MapRefusal::Keeper => 11,
            MapRefusal::Behind => 12,
            MapRefusal::LineRefused => 13,
            MapRefusal::NotVersor => 14,
            MapRefusal::NoGeometry => 15,
            MapRefusal::Unplaced => 16,
        }
    }

    pub fn of(code: u8) -> Option<MapRefusal> {
        MapRefusal::ALL.into_iter().find(|r| r.code() == code)
    }
}

/// WHAT THE COURT ANSWERED a map act.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mapped {
    Sealed { record: [u8; 32], h: u64, point: TPoint },
    Refused(MapRefusal),
}

/// `[1][record id 32][sealed h u64 LE][court's final TPoint 84]` or `[0][reason]`.
pub fn encode_map_reply(r: &Mapped) -> Vec<u8> {
    match r {
        Mapped::Sealed { record, h, point } => {
            let mut out = vec![1];
            out.extend_from_slice(record);
            out.extend_from_slice(&h.to_le_bytes());
            point.put(&mut out);
            out
        }
        Mapped::Refused(why) => vec![0, why.code()],
    }
}

pub fn decode_map_reply(b: &[u8]) -> Option<Mapped> {
    match b {
        [0, code] => Some(Mapped::Refused(MapRefusal::of(*code)?)),
        [1, rest @ ..] => {
            let mut p = 0;
            let record = key32(rest, &mut p)?;
            let h = u64_at(rest, &mut p)?;
            let point = TPoint::take(rest, &mut p)?;
            (p == rest.len()).then_some(Mapped::Sealed { record, h, point })
        }
        _ => None,
    }
}

/// 78.154 — A RECORD AS MAPPED: the record, the final point it was mapped
/// at, and the height its act sealed at — kept by the court that sealed it,
/// the court a traversal's map fee pays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeldRecord {
    pub record: MapRecord,
    pub point: TPoint,
    pub sealed: u64,
}

impl HeldRecord {
    pub fn put(&self, out: &mut Vec<u8>) {
        self.record.put(out);
        self.point.put(out);
        out.extend_from_slice(&self.sealed.to_le_bytes());
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<HeldRecord> {
        Some(HeldRecord { record: MapRecord::take(b, p)?, point: TPoint::take(b, p)?, sealed: u64_at(b, p)? })
    }
}

/// ONE RECORD AS READ: the record held and its exact places in its
/// universe's own space, derived at the read and never stored beside it — a
/// mouth's one place off its tube (78.140: a point of the curved space or of
/// the torus's fundamental domain), a local throat's two ends off its
/// frames in the universe's declared geometry (side 0, then side 1); none
/// for a segment, or where a place does not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapEntry {
    pub held: HeldRecord,
    pub places: Vec<crate::transducer::cga::Mv>,
}

/// A UNIVERSE'S MAP as read: every record, by its id's order — `[count]
/// [(held record, n u8, place × n) …]`, `n` at most 2 (0 or 1 is the v1
/// writing exactly).
pub fn encode_map_reading(records: &[MapEntry]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::transducer::address::put_key_count(&mut out, records.len());
    for r in records {
        r.held.put(&mut out);
        let n = match r.places.len() {
            0 => 0u8,
            1 => 1,
            _ => 2,
        };
        out.push(n);
        for m in r.places.iter().take(usize::from(n)) {
            m.to_bytes(&mut out);
        }
    }
    out
}

pub fn decode_map_reading(b: &[u8]) -> Option<Vec<MapEntry>> {
    let mut p = 0;
    let all = (0..crate::transducer::address::take_key_count(b, &mut p)?)
        .map(|_| {
            let held = HeldRecord::take(b, &mut p)?;
            let n = byte(b, &mut p)?;
            if n > 2 {
                return None;
            }
            let places = (0..n).map(|_| crate::transducer::cga::Mv::from_bytes(b, &mut p)).collect::<Option<Vec<_>>>()?;
            Some(MapEntry { held, places })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(all)
}
