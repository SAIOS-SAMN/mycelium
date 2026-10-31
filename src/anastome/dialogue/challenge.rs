// 78.125 — ACT_CHALLENGE: a sealed fold challenged within its window (2(c),
// ruled 2026-10-07), and its finality read.
//
//! A fold's rows stay with its tier (LD0); its receipt commits to them. A
//! challenge makes the commitment answerable, in four kinds:
//!
//! - DEMAND (availability): a challenger asks the tier to publish the prefix
//!   its fold nets. The tier has `W` heights from the demand; unanswered,
//!   the fold is falsified by default.
//! - EVIDENCE (replay): the prefix itself — the tier line's entries from the
//!   tier's previous admitted fold to this one's prefix head — with the
//!   fold's own rows, declared and children, put to the court by anyone. The
//!   court checks the entries chain to the cited head and the fold to its
//!   commitments, and re-derives: the fold's OWN entries must sum to its rows
//!   less what it nets of its children, translated, and to its declared.
//!   Upheld, the prefix's content hash is sealed; else the fold is falsified.
//! - DEFAULT: a demand's deadline passed unanswered, claimed.
//! - CORRECTION: the tier whose fold closed an arbitrage cycle books the
//!   gain, naming its holder and amount on G, within its fold's window.
//!
//! A TIER LINE (canonical, so replay is possible): entries `{rows,
//! declared}`, `head₀` the zero hash, `headₙ = H("mycelium tier line v1" ‖
//! headₙ₋₁ ‖ entry)`.
//!
//! The FINALITY of a fold is read, never stored, and sealing nothing.

use super::uplink::{put_children, put_nets, take_children, take_nets, ChildNet, FoldAt, Net};
use crate::chronometry::Stamp;

/// The tier line's hash domain.
pub const LINE_DOMAIN: &[u8] = b"mycelium tier line v1";

/// The signed bytes' domain: a challenge's signature is never another's.
const SIGNED: &[u8] = b"mycelium challenge v1";

/// ONE ENTRY OF A TIER LINE: a batch of transfers by its nets per identity
/// per lane, and the sources and sinks it declares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub rows: Vec<Net>,
    pub declared: Vec<Net>,
}

impl Entry {
    fn put(&self, out: &mut Vec<u8>) {
        put_nets(out, &self.rows);
        put_nets(out, &self.declared);
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Entry> {
        Some(Entry { rows: take_nets(b, p)?, declared: take_nets(b, p)? })
    }
}

/// THE HEAD a tier line reaches from `from` through `entries`.
pub fn line_head(from: [u8; 32], entries: &[Entry]) -> [u8; 32] {
    entries.iter().fold(from, |head, e| {
        let mut b = LINE_DOMAIN.to_vec();
        b.extend_from_slice(&head);
        e.put(&mut b);
        crate::chronometry::payload_hash(&b)
    })
}

fn put_entries(out: &mut Vec<u8>, entries: &[Entry]) {
    crate::transducer::address::put_key_count(out, entries.len());
    for e in entries {
        e.put(out);
    }
}

fn take_entries(b: &[u8], p: &mut usize) -> Option<Vec<Entry>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n).map(|_| Entry::take(b, p)).collect()
}

/// WHAT IS CHALLENGED, AND HOW. 78.138 — a fold is named by where it stands
/// (`FoldAt`): a reserve's by its receipt, a line's by its reserve's
/// receipt and its place inside.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Challenge {
    /// The tier must publish the prefix the fold nets.
    Demand { fold: FoldAt },
    /// The prefix, and the fold's own rows, declared and children — proved
    /// against its commitments — put to RECONSTRUCTION (the Concordance,
    /// 4.4): the line's entries stepped to the committed head, the entries' nets
    /// conserved, each step checked against the one before.
    Evidence { fold: FoldAt, rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet>, entries: Vec<Entry> },
    /// A demand's deadline passed unanswered: the fold falsified by default.
    Default { fold: FoldAt },
    /// The arbitrage the fold's edge from `child` closed, booked: its gain
    /// on G, `amount`, a declared source to the named `holder`.
    Correction { fold: FoldAt, child: Vec<u8>, holder: [u8; 32], amount: i128 },
    /// A STREAMED PUBLICATION opened: the fold's own rows, declared and
    /// children, proved against its commitments; the line follows in parts.
    Open { fold: FoldAt, rows: Vec<Net>, declared: Vec<Net>, children: Vec<ChildNet> },
    /// The next entries of a streamed publication, continuing its head. The
    /// part whose head reaches the cited prefix completes the reconstruction.
    Part { fold: FoldAt, entries: Vec<Entry> },
    /// 78.125 (as ruled) — A SELF-DEMAND WITHDRAWN by the
    /// fold's own tier, named by the demand's receipt stamp: before any
    /// default is claimed, once per fold, reopening the evidence window for
    /// `W` from the withdrawal — sealed, a public record that the tier asked
    /// for its own second chance.
    Withdraw { fold: FoldAt, demand: Stamp },
}

impl Challenge {
    /// The kind's byte on the wire; 4 is the finality read.
    pub fn kind(&self) -> u8 {
        match self {
            Challenge::Demand { .. } => 0,
            Challenge::Evidence { .. } => 1,
            Challenge::Default { .. } => 2,
            Challenge::Correction { .. } => 3,
            Challenge::Open { .. } => 6,
            Challenge::Part { .. } => 7,
            Challenge::Withdraw { .. } => 9,
        }
    }

    /// The fold challenged.
    pub fn fold(&self) -> FoldAt {
        match self {
            Challenge::Demand { fold }
            | Challenge::Evidence { fold, .. }
            | Challenge::Default { fold }
            | Challenge::Correction { fold, .. }
            | Challenge::Open { fold, .. }
            | Challenge::Part { fold, .. }
            | Challenge::Withdraw { fold, .. } => fold.clone(),
        }
    }

    fn put_body(&self, out: &mut Vec<u8>) {
        self.fold().put_canonical(out);
        match self {
            Challenge::Demand { .. } | Challenge::Default { .. } => {}
            Challenge::Evidence { rows, declared, children, entries, .. } => {
                put_nets(out, rows);
                put_nets(out, declared);
                put_children(out, children);
                put_entries(out, entries);
            }
            Challenge::Open { rows, declared, children, .. } => {
                put_nets(out, rows);
                put_nets(out, declared);
                put_children(out, children);
            }
            Challenge::Part { entries, .. } => put_entries(out, entries),
            Challenge::Withdraw { demand, .. } => demand.put_canonical(out),
            Challenge::Correction { child, holder, amount, .. } => {
                crate::transducer::address::put_key_bytes(out, child);
                out.extend_from_slice(holder);
                out.extend_from_slice(&amount.to_le_bytes());
            }
        }
    }

    fn take_body(kind: u8, b: &[u8], p: &mut usize) -> Option<Challenge> {
        let fold = FoldAt::take_canonical(b, p)?;
        Some(match kind {
            0 => Challenge::Demand { fold },
            1 => Challenge::Evidence {
                fold,
                rows: take_nets(b, p)?,
                declared: take_nets(b, p)?,
                children: take_children(b, p)?,
                entries: take_entries(b, p)?,
            },
            2 => Challenge::Default { fold },
            3 => {
                let child = crate::transducer::address::take_key_bytes(b, p)?;
                let holder: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                let amount = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
                *p += 16;
                Challenge::Correction { fold, child, holder, amount }
            }
            6 => Challenge::Open { fold, rows: take_nets(b, p)?, declared: take_nets(b, p)?, children: take_children(b, p)? },
            7 => Challenge::Part { fold, entries: take_entries(b, p)? },
            9 => Challenge::Withdraw { fold, demand: Stamp::take_canonical(b, p)? },
            _ => return None,
        })
    }

    /// The bytes the filer signs: the domain, the filer, the kind, the body.
    pub fn signed_bytes(&self, who: &[u8; 32]) -> Vec<u8> {
        let mut out = SIGNED.to_vec();
        out.extend_from_slice(who);
        out.push(self.kind());
        self.put_body(&mut out);
        out
    }

    /// Signed by the filer, whose key is `who`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedChallenge {
        use ed25519_dalek::Signer;
        let who = key.verifying_key().to_bytes();
        let sig = key.sign(&self.signed_bytes(&who)).to_bytes();
        SignedChallenge { who, challenge: self, sig }
    }
}

/// A CHALLENGE AND ITS FILER'S SIGNATURE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedChallenge {
    pub who: [u8; 32],
    pub challenge: Challenge,
    pub sig: [u8; 64],
}

/// WHAT A FRAME ASKS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ChallengeRequest {
    Signed(SignedChallenge),
    /// A fold's finality, read; nothing sealed.
    Finality(FoldAt),
    /// A domain's open obstructions, read; nothing sealed.
    Obstructions([u8; 32]),
    /// A tier's line head — where its next fold's span begins — read;
    /// nothing sealed.
    LineHead([u8; 32]),
}

const KIND_FINALITY: u8 = 4;
const KIND_OBSTRUCTIONS: u8 = 5;
const KIND_LINE_HEAD: u8 = 8;

/// `[ACT_CHALLENGE][8][tier 32]`.
pub fn encode_line_head_request(tier: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_CHALLENGE, KIND_LINE_HEAD];
    out.extend_from_slice(&tier);
    out
}

/// 78.133 — A TIER'S LINE HEAD AS READ: the head, the fold that set it where
/// one did (none: the zero head, no fold recorded), and the μ HELD toward
/// the tier in toll escrow per lane — receipts not final before release.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineHead {
    pub head: [u8; 32],
    /// The fold that set it — 78.138: where it stands (a line's is carried
    /// inside its reserve's receipt).
    pub at: Option<FoldAt>,
    pub held: [u64; 4],
}

/// `[head 32][0 | 1 stamp][held 4×u64]`.
pub fn encode_line_head_reply(r: &LineHead) -> Vec<u8> {
    let mut out = r.head.to_vec();
    match &r.at {
        None => out.push(0),
        Some(s) => {
            out.push(1);
            s.put_canonical(&mut out);
        }
    }
    for m in r.held {
        out.extend_from_slice(&m.to_le_bytes());
    }
    out
}

pub fn decode_line_head_reply(b: &[u8]) -> Option<LineHead> {
    let head: [u8; 32] = b.get(..32)?.try_into().ok()?;
    let mut p = 33;
    let at = match *b.get(32)? {
        0 => None,
        1 => Some(FoldAt::take_canonical(b, &mut p)?),
        _ => return None,
    };
    let mut held = [0u64; 4];
    for m in &mut held {
        *m = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    (p == b.len()).then_some(LineHead { head, at, held })
}

/// AN OPEN ARBITRAGE, as served: the fold whose seal closed the cycle, the
/// edge's child and parent fold vertices (`tier ‖ prefix`), and the cycle's
/// exact holonomy per lane — what a correction names (fold and child) and
/// prices (the gain the holonomy measures, on the flows the tier holds).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenObstruction {
    pub fold: FoldAt,
    pub child: Vec<u8>,
    pub parent: Vec<u8>,
    pub holonomy: crate::transducer::cech::coefficient::Rates,
}

/// `[ACT_CHALLENGE][5][domain 32]`.
pub fn encode_obstructions_request(domain: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_CHALLENGE, KIND_OBSTRUCTIONS];
    out.extend_from_slice(&domain);
    out
}

/// `[count][(fold stamp, child, parent, holonomy) …]`.
pub fn encode_obstructions_reply(open: &[OpenObstruction]) -> Vec<u8> {
    use crate::transducer::cech::coefficient::Coefficient;
    let mut out = Vec::new();
    crate::transducer::address::put_key_count(&mut out, open.len());
    for o in open {
        o.fold.put_canonical(&mut out);
        crate::transducer::address::put_key_bytes(&mut out, &o.child);
        crate::transducer::address::put_key_bytes(&mut out, &o.parent);
        o.holonomy.put_canonical(&mut out);
    }
    out
}

pub fn decode_obstructions_reply(b: &[u8]) -> Option<Vec<OpenObstruction>> {
    use crate::transducer::cech::coefficient::{Coefficient, Rates};
    let mut p = 0;
    let n = crate::transducer::address::take_key_count(b, &mut p)?;
    let open = (0..n)
        .map(|_| {
            let fold = FoldAt::take_canonical(b, &mut p)?;
            let child = crate::transducer::address::take_key_bytes(b, &mut p)?;
            let parent = crate::transducer::address::take_key_bytes(b, &mut p)?;
            Some(OpenObstruction { fold, child, parent, holonomy: Rates::take_canonical(b, &mut p)? })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(open)
}

/// `[ACT_CHALLENGE][kind][who 32][body][sig 64]`.
pub fn encode_challenge(c: &SignedChallenge) -> Vec<u8> {
    let mut out = vec![super::ACT_CHALLENGE, c.challenge.kind()];
    out.extend_from_slice(&c.who);
    c.challenge.put_body(&mut out);
    out.extend_from_slice(&c.sig);
    out
}

/// `[ACT_CHALLENGE][4][fold stamp]`.
pub fn encode_finality_request(fold: FoldAt) -> Vec<u8> {
    let mut out = vec![super::ACT_CHALLENGE, KIND_FINALITY];
    fold.put_canonical(&mut out);
    out
}

/// The request a frame carries, read whole, or none.
pub fn decode_challenge_request(b: &[u8]) -> Option<ChallengeRequest> {
    let [act, kind, ..] = b else { return None };
    if *act != super::ACT_CHALLENGE {
        return None;
    }
    let mut p = 2;
    let request = match *kind {
        KIND_FINALITY => ChallengeRequest::Finality(FoldAt::take_canonical(b, &mut p)?),
        KIND_OBSTRUCTIONS => {
            let domain: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            ChallengeRequest::Obstructions(domain)
        }
        KIND_LINE_HEAD => {
            let tier: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            ChallengeRequest::LineHead(tier)
        }
        k => {
            let who: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            let challenge = Challenge::take_body(k, b, &mut p)?;
            let sig: [u8; 64] = b.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            ChallengeRequest::Signed(SignedChallenge { who, challenge, sig })
        }
    };
    (p == b.len()).then_some(request)
}

/// WHY A CHALLENGE WAS NOT TAKEN.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChallengeRefusal {
    /// The filer's signature does not verify.
    Unsigned,
    /// No fold of this court's ledger trees stands at the stamp — or it was
    /// sealed before the tier-line law, grandfathered: never demanded, never
    /// defaulted.
    NoRecord,
    /// The fold's window (and any demand's deadline) has passed, or it was
    /// falsified or unwound already.
    Closed,
    /// The prefix is published already: a demand asks for nothing.
    Published,
    /// A default claimed where no demand stands.
    NotDemanded,
    /// A default claimed before the demand's deadline.
    NotDue,
    /// The entries do not chain from the tier's previous head to the cited one.
    NotThePrefix,
    /// The rows, declared or children do not prove against the fold's commitments.
    NotTheFold,
    /// A correction filed by a key other than the closing fold's tier.
    NotTheTier,
    /// No open arbitrage closed at that fold from that child.
    NoObstruction,
    /// The act could not be sealed.
    LineRefused,
    /// A demand stands already: its deadline is the one the tier answers to.
    Demanded,
    /// A part of a publication this filer never opened for this fold.
    NotOpened,
    /// A demand withdrawn already: a fold's second chance is one.
    Withdrawn,
    /// The stamp names no receipt of a demand on this fold by its own tier:
    /// only a self-demand is withdrawn, and only by the tier.
    NotTheDemand,
}

impl ChallengeRefusal {
    const ALL: [ChallengeRefusal; 15] = [
        ChallengeRefusal::Unsigned,
        ChallengeRefusal::NoRecord,
        ChallengeRefusal::Closed,
        ChallengeRefusal::Published,
        ChallengeRefusal::NotDemanded,
        ChallengeRefusal::NotDue,
        ChallengeRefusal::NotThePrefix,
        ChallengeRefusal::NotTheFold,
        ChallengeRefusal::NotTheTier,
        ChallengeRefusal::NoObstruction,
        ChallengeRefusal::LineRefused,
        ChallengeRefusal::Demanded,
        ChallengeRefusal::NotOpened,
        ChallengeRefusal::Withdrawn,
        ChallengeRefusal::NotTheDemand,
    ];

    fn byte(self) -> u8 {
        match self {
            ChallengeRefusal::Unsigned => 0,
            ChallengeRefusal::NoRecord => 1,
            ChallengeRefusal::Closed => 2,
            ChallengeRefusal::Published => 3,
            ChallengeRefusal::NotDemanded => 4,
            ChallengeRefusal::NotDue => 5,
            ChallengeRefusal::NotThePrefix => 6,
            ChallengeRefusal::NotTheFold => 7,
            ChallengeRefusal::NotTheTier => 8,
            ChallengeRefusal::NoObstruction => 9,
            ChallengeRefusal::LineRefused => 10,
            ChallengeRefusal::Demanded => 11,
            ChallengeRefusal::NotOpened => 12,
            ChallengeRefusal::Withdrawn => 13,
            ChallengeRefusal::NotTheDemand => 14,
        }
    }

    fn of(b: u8) -> Option<ChallengeRefusal> {
        ChallengeRefusal::ALL.into_iter().find(|r| r.byte() == b)
    }
}

/// THE COURT'S ANSWER TO A CHALLENGE.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Challenged {
    /// The demand sealed; the tier must publish before `deadline` (a height).
    Demanded { at: Stamp, receipt: [u8; 32], deadline: u64 },
    /// The replay upheld the fold; the line's head it reached — the
    /// prefix's content address, the chain of every entry — sealed.
    Upheld { at: Stamp, receipt: [u8; 32], prefix: [u8; 32] },
    /// A streamed publication taken so far, nothing sealed: its line stands
    /// at `head`; the next part continues it.
    Streaming { head: [u8; 32] },
    /// A self-demand withdrawn, sealed: the evidence window reopened until
    /// `until` (a height).
    Reopened { at: Stamp, receipt: [u8; 32], until: u64 },
    /// The fold falsified — by replay or by default — and reversed, with
    /// `unwound` ancestors that netted it returned to provisional.
    Falsified { at: Stamp, receipt: [u8; 32], unwound: u32 },
    /// The arbitrage booked: the obstruction discharged.
    Discharged { at: Stamp, receipt: [u8; 32] },
    Refused(ChallengeRefusal),
}

/// A FOLD'S FINALITY, read at the court's head.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Finality {
    /// Its window passed with nothing open, and every fold it nets is final:
    /// `upheld` where its prefix was replayed and upheld, else unchallenged.
    Final { upheld: bool },
    /// Its window, or a demand's deadline, runs to this height.
    Provisional { until: u64 },
    /// Its time passed, but an obstruction, a fold it nets, or an unclaimed
    /// default keeps it open.
    Blocked,
    /// Falsified by replay or default, and reversed.
    Falsified,
    /// A fold it netted was reversed: it must refile.
    Unwound,
    /// No record of it on this court's ledger trees.
    Unrecorded,
    /// Sealed before the tier-line law (78.125): no line to replay, so
    /// GRANDFATHERED — final, unchallenged, pre-line; demands refused.
    PreLine,
}

pub fn encode_challenge_reply(c: &Challenged) -> Vec<u8> {
    let mut out = Vec::new();
    let sealed = |out: &mut Vec<u8>, tag: u8, at: &Stamp, receipt: &[u8; 32]| {
        out.push(tag);
        at.put_canonical(out);
        out.extend_from_slice(receipt);
    };
    match c {
        Challenged::Demanded { at, receipt, deadline } => {
            sealed(&mut out, 0, at, receipt);
            out.extend_from_slice(&deadline.to_le_bytes());
        }
        Challenged::Upheld { at, receipt, prefix } => {
            sealed(&mut out, 1, at, receipt);
            out.extend_from_slice(prefix);
        }
        Challenged::Falsified { at, receipt, unwound } => {
            sealed(&mut out, 2, at, receipt);
            out.extend_from_slice(&unwound.to_le_bytes());
        }
        Challenged::Discharged { at, receipt } => sealed(&mut out, 3, at, receipt),
        Challenged::Refused(why) => out.extend_from_slice(&[4, why.byte()]),
        Challenged::Streaming { head } => {
            out.push(5);
            out.extend_from_slice(head);
        }
        Challenged::Reopened { at, receipt, until } => {
            sealed(&mut out, 6, at, receipt);
            out.extend_from_slice(&until.to_le_bytes());
        }
    }
    out
}

pub fn decode_challenge_reply(b: &[u8]) -> Option<Challenged> {
    let [tag, ..] = b else { return None };
    let mut p = 1;
    match *tag {
        4 => {
            return match b {
                [4, why] => Some(Challenged::Refused(ChallengeRefusal::of(*why)?)),
                _ => None,
            }
        }
        5 => return Some(Challenged::Streaming { head: b.get(1..)?.try_into().ok()? }),
        _ => {}
    }
    let at = Stamp::take_canonical(b, &mut p)?;
    let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let c = match *tag {
        0 => {
            let deadline = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
            p += 8;
            Challenged::Demanded { at, receipt, deadline }
        }
        1 => {
            let prefix: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Challenged::Upheld { at, receipt, prefix }
        }
        2 => {
            let unwound = u32::from_le_bytes(b.get(p..p + 4)?.try_into().ok()?);
            p += 4;
            Challenged::Falsified { at, receipt, unwound }
        }
        3 => Challenged::Discharged { at, receipt },
        6 => {
            let until = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
            p += 8;
            Challenged::Reopened { at, receipt, until }
        }
        _ => return None,
    };
    (p == b.len()).then_some(c)
}

/// 78.133 — A FOLD'S FINALITY AS READ: the finality; whether its tier stands
/// BONDED (78.129); whether a toll is HELD on it (it consumed a traversal's
/// OUT side whose parts are not yet released).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FinalityReading {
    pub finality: Finality,
    pub bonded: bool,
    pub toll_held: bool,
}

/// `[finality][bonded u8][toll held u8]`.
pub fn encode_finality_reading(r: &FinalityReading) -> Vec<u8> {
    let mut out = encode_finality_reply(&r.finality, r.bonded);
    out.push(u8::from(r.toll_held));
    out
}

pub fn decode_finality_reading(b: &[u8]) -> Option<FinalityReading> {
    let (toll_held, rest) = match b.split_last()? {
        (0, rest) => (false, rest),
        (1, rest) => (true, rest),
        _ => return None,
    };
    let (finality, bonded) = decode_finality_reply(rest)?;
    Some(FinalityReading { finality, bonded, toll_held })
}

/// A fold's finality and — 78.129 — whether its tier stands BONDED (a bond
/// posted by its domain's reserve), so a reader weighs an unbacked fold:
/// `[finality][bonded u8]`.
pub fn encode_finality_reply(f: &Finality, bonded: bool) -> Vec<u8> {
    let mut out = match f {
        Finality::Final { upheld } => vec![0, u8::from(*upheld)],
        Finality::Provisional { until } => {
            let mut out = vec![1];
            out.extend_from_slice(&until.to_le_bytes());
            out
        }
        Finality::Blocked => vec![2],
        Finality::Falsified => vec![3],
        Finality::Unwound => vec![4],
        Finality::Unrecorded => vec![5],
        Finality::PreLine => vec![6],
    };
    out.push(u8::from(bonded));
    out
}

pub fn decode_finality_reply(b: &[u8]) -> Option<(Finality, bool)> {
    let (bonded, b) = match b.split_last()? {
        (0, rest) => (false, rest),
        (1, rest) => (true, rest),
        _ => return None,
    };
    let f = match b {
        [0, 0] => Finality::Final { upheld: false },
        [0, 1] => Finality::Final { upheld: true },
        [1, rest @ ..] => Finality::Provisional { until: u64::from_le_bytes(rest.try_into().ok()?) },
        [2] => Finality::Blocked,
        [3] => Finality::Falsified,
        [4] => Finality::Unwound,
        [5] => Finality::Unrecorded,
        [6] => Finality::PreLine,
        _ => return None,
    };
    Some((f, bonded))
}
