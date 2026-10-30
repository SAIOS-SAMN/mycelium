// 78.119 — ACT_UPLINK: a kernel's settlement tier sends up its fold (78.62
// step 3, ruled 2026-10-07: folds are admitted, not every
// transition).
//
//! A TIER'S FOLD is its net positions per identity per lane over a prefix of
//! its own line — `μ` never summed across lanes — its declared sources and
//! sinks, and the hash of the prefix head it nets, signed by the tier. The
//! court that admits it is a verifier OUTSIDE the tier's delegation closure:
//! it checks the tier's signature and the fold's CONTINUITY — on every lane,
//! the identities' nets sum exactly to the declared net emission — and seals
//! the fold's coordinates on its line (`Cite::Uplink`, never a copy of its
//! rows, LD0). A dispute replays the cited prefix against the rows, which
//! the tier holds and this receipt commits to by hash.
//!
//! First slice: a single-level fold. Multi-tier chaining, matter handoffs
//! (two-sided, P2) and the challenge window are later rows.

use crate::chronometry::Stamp;

/// The signed bytes' domain, so a fold signature is never a signature on
/// anything else.
const DOMAIN: &[u8] = b"mycelium uplink fold v1";

/// One identity's net position per lane `S:G:T:H`, signed and exact.
pub type Net = ([u8; 32], [i128; 4]);

/// A TIER'S FOLD: who folded, the prefix head it nets, the net per identity
/// per lane, and the declared sources (positive) and sinks (negative) — each
/// list in its identities' own ascending order, each identity once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TierFold {
    pub tier: [u8; 32],
    pub prefix: [u8; 32],
    pub rows: Vec<Net>,
    pub declared: Vec<Net>,
    /// 78.123 B — the folds this fold nets (empty for a single-level fold,
    /// which writes and signs exactly as 78.119's).
    pub children: Vec<ChildNet>,
    /// 78.126 — the handoff sides this fold consumes, each by the handoff's
    /// id and its side (empty: the fold writes and signs as before).
    pub handoffs: Vec<([u8; 32], super::handoff::Side)>,
}

/// 78.138 — WHERE A FOLD STANDS ON THE COURT: the receipt that sealed it and
/// the path of child indices inside that receipt's fold — empty for the
/// fold the receipt sealed (a reserve's), `[i]` for the line it carried at
/// `i`, `[i, j]` for the line that line carried at `j`, never flattened. Its
/// key is the stamp's key then the path, two bytes an index — so a fold the
/// court sealed itself keys exactly as it did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldAt {
    pub stamp: Stamp,
    pub path: Vec<u16>,
}

impl FoldAt {
    /// The fold a receipt sealed.
    pub fn sealed(stamp: Stamp) -> FoldAt {
        FoldAt { stamp, path: Vec::new() }
    }

    /// The child at `i` of this fold.
    pub fn child(&self, i: u16) -> FoldAt {
        let mut path = self.path.clone();
        path.push(i);
        FoldAt { stamp: self.stamp, path }
    }

    pub fn key(&self) -> Vec<u8> {
        let mut k = self.stamp.key();
        for i in &self.path {
            k.extend_from_slice(&i.to_be_bytes());
        }
        k
    }

    pub fn from_key(b: &[u8]) -> Option<FoldAt> {
        let mut p = 0usize;
        let stamp = Stamp::take_canonical(b, &mut p)?;
        let rest = b.get(p..)?;
        if rest.len() % 2 != 0 {
            return None;
        }
        Some(FoldAt { stamp, path: rest.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect() })
    }

    /// The stamp, then the path's count and its indices.
    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        self.stamp.put_canonical(out);
        crate::transducer::address::put_key_count(out, self.path.len());
        for i in &self.path {
            out.extend_from_slice(&i.to_be_bytes());
        }
    }

    pub fn take_canonical(b: &[u8], p: &mut usize) -> Option<FoldAt> {
        let stamp = Stamp::take_canonical(b, p)?;
        let path = (0..crate::transducer::address::take_key_count(b, p)?)
            .map(|_| {
                let i = u16::from_be_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
                *p += 2;
                Some(i)
            })
            .collect::<Option<Vec<_>>>()?;
        Some(FoldAt { stamp, path })
    }
}

/// 78.138 — WHICH FOLD A PARENT NETS: one already on the court (`Sealed`,
/// by where it stands), or a progenitor line's fold CARRIED INLINE — the
/// line's whole fold and its signature, its own children carried in it in
/// turn. Lines never touch the court: the ancients carry them (the
/// Concordance, 4.1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Child {
    Sealed(FoldAt),
    Carried(Box<SignedFold>),
}

/// 78.123 B — ONE FOLD A PARENT NETS: the child (78.138: sealed, or carried
/// inline); the child's own rows and declared (proved against its
/// commitment, or equal to the carried fold's); the part of each this parent
/// includes — a child netted by several parents is partitioned among them —
/// and the rate per lane the parent translates it at (strictly positive; by
/// the lexicon v2, S, T and H exactly 1, only G translates).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildNet {
    pub child: Child,
    pub rows: Vec<Net>,
    pub declared: Vec<Net>,
    pub included_rows: Vec<Net>,
    pub included_declared: Vec<Net>,
    pub rate: [(i128, i128); 4],
}

impl ChildNet {
    /// The commitment the child's receipt holds over its own rows.
    pub fn rows_hash(&self) -> [u8; 32] {
        let mut out = Vec::new();
        put_nets(&mut out, &self.rows);
        put_nets(&mut out, &self.declared);
        crate::chronometry::payload_hash(&out)
    }

    /// The child's own writing: a sealed child's stamp alone where its path
    /// is empty (78.123's writing, untagged); else tagged — `[0][fold at]`
    /// or `[1][the carried fold]`.
    fn put(&self, out: &mut Vec<u8>, tagged: bool) {
        match (&self.child, tagged) {
            (Child::Sealed(at), false) => at.stamp.put_canonical(out),
            (Child::Sealed(at), true) => {
                out.push(0);
                at.put_canonical(out);
            }
            (Child::Carried(f), _) => {
                out.push(1);
                put_fold(out, f);
            }
        }
        for nets in [&self.rows, &self.declared, &self.included_rows, &self.included_declared] {
            put_nets(out, nets);
        }
        for (n, d) in self.rate {
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
    }

    fn take(b: &[u8], p: &mut usize, tagged: bool) -> Option<ChildNet> {
        let child = match tagged {
            false => Child::Sealed(FoldAt::sealed(Stamp::take_canonical(b, p)?)),
            true => {
                let tag = *b.get(*p)?;
                *p += 1;
                match tag {
                    0 => Child::Sealed(FoldAt::take_canonical(b, p)?),
                    1 => Child::Carried(Box::new(take_fold(b, p)?)),
                    _ => return None,
                }
            }
        };
        let (rows, declared) = (take_nets(b, p)?, take_nets(b, p)?);
        let (included_rows, included_declared) = (take_nets(b, p)?, take_nets(b, p)?);
        let mut rate = [(1i128, 1i128); 4];
        for r in &mut rate {
            let n = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
            *p += 16;
            let d = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
            *p += 16;
            *r = (n, d);
        }
        Some(ChildNet { child, rows, declared, included_rows, included_declared, rate })
    }

    /// Whether this child's writing needs the tagged form: a carried fold,
    /// or a sealed one standing inside another's.
    fn needs_tag(&self) -> bool {
        match &self.child {
            Child::Sealed(at) => !at.path.is_empty(),
            Child::Carried(_) => true,
        }
    }
}

/// 78.138 — THE TAGGED CHILDREN'S MARK, where a count would stand: an
/// over-long spelling no count is ever written as, so an untagged block
/// (78.123's) reads as it did.
const TAGGED: [u8; 2] = [0x80, 0x00];

/// The children: untagged where every one is a fold the court sealed itself
/// (78.123's writing exactly), else the mark, then each tagged.
pub(crate) fn put_children(out: &mut Vec<u8>, children: &[ChildNet]) {
    let tagged = children.iter().any(ChildNet::needs_tag);
    if tagged {
        out.extend_from_slice(&TAGGED);
    }
    crate::transducer::address::put_key_count(out, children.len());
    for c in children {
        c.put(out, tagged);
    }
}

pub(crate) fn take_children(b: &[u8], p: &mut usize) -> Option<Vec<ChildNet>> {
    let tagged = b.get(*p..*p + 2) == Some(&TAGGED[..]);
    if tagged {
        *p += 2;
    }
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n).map(|_| ChildNet::take(b, p, tagged)).collect()
}

/// 78.138 — A CARRIED FOLD'S WRITING, inside its parent: the tier, prefix,
/// rows and signature as an uplink's, then its children and its handoff
/// sides, each counted (nothing left implicit inside another frame).
fn put_fold(out: &mut Vec<u8>, f: &SignedFold) {
    out.extend_from_slice(&f.fold.tier);
    out.extend_from_slice(&f.fold.prefix);
    out.extend_from_slice(&f.fold.rows_bytes());
    out.extend_from_slice(&f.sig);
    put_children(out, &f.fold.children);
    put_handoffs(out, &f.fold.handoffs);
}

fn take_fold(b: &[u8], p: &mut usize) -> Option<SignedFold> {
    let tier: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    let prefix: [u8; 32] = b.get(*p + 32..*p + 64)?.try_into().ok()?;
    *p += 64;
    let (rows, declared) = (take_nets(b, p)?, take_nets(b, p)?);
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    let children = take_children(b, p)?;
    let handoffs = take_handoffs(b, p)?;
    Some(SignedFold { fold: TierFold { tier, prefix, rows, declared, children, handoffs }, sig })
}

/// The fold and the tier's signature over [`TierFold::signed_bytes`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedFold {
    pub fold: TierFold,
    pub sig: [u8; 64],
}

/// WHY A FOLD IS NOT CONTINUOUS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Discontinuous {
    /// A list is not in its identities' ascending order, or names one twice:
    /// the rows have no canonical writing.
    Unordered,
    /// A lane's sum passes its carrier (rule 11): refused, never wrapped.
    Overflow { lane: usize },
    /// A lane's nets do not sum to its declared emission: value made or lost.
    Unbalanced { lane: usize },
    /// 78.123 B — a child's rate on this lane is not strictly positive, or
    /// translates a lane the lexicon keeps untranslated (S, T, H: exactly 1).
    Rate { lane: usize },
}

pub(crate) fn put_nets(out: &mut Vec<u8>, nets: &[Net]) {
    crate::transducer::address::put_key_count(out, nets.len());
    for (who, lanes) in nets {
        out.extend_from_slice(who);
        for l in lanes {
            out.extend_from_slice(&l.to_le_bytes());
        }
    }
}

pub(crate) fn take_nets(b: &[u8], p: &mut usize) -> Option<Vec<Net>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let mut lanes = [0i128; 4];
            for l in &mut lanes {
                *l = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
                *p += 16;
            }
            Some((who, lanes))
        })
        .collect()
}

fn ascending(nets: &[Net]) -> bool {
    nets.windows(2).all(|w| matches!(w, [a, b] if a.0 < b.0))
}

/// Per lane, the exact sum, or the lane it passed its carrier on.
fn sum(nets: &[Net]) -> Result<[i128; 4], usize> {
    let mut total = [0i128; 4];
    for (_, lanes) in nets {
        for (l, (t, v)) in total.iter_mut().zip(lanes).enumerate() {
            *t = t.checked_add(*v).ok_or(l)?;
        }
    }
    Ok(total)
}

impl TierFold {
    /// The rows written canonically: the nets, then the declared.
    pub fn rows_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        put_nets(&mut out, &self.rows);
        put_nets(&mut out, &self.declared);
        out
    }

    /// THE ROWS' COMMITMENT — what the receipt cites and a party holding the
    /// rows proves them against.
    pub fn rows_hash(&self) -> [u8; 32] {
        crate::chronometry::payload_hash(&self.rows_bytes())
    }

    /// The bytes the tier signs: the domain, the tier, the prefix head and
    /// the rows' commitment — and, where it nets children, their commitment
    /// too (a fold with none signs exactly as 78.119's).
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = DOMAIN.to_vec();
        out.extend_from_slice(&self.tier);
        out.extend_from_slice(&self.prefix);
        out.extend_from_slice(&self.rows_hash());
        if let Some(c) = self.children_hash() {
            out.extend_from_slice(&c);
        }
        if let Some(h) = self.handoffs_hash() {
            out.extend_from_slice(&h);
        }
        out
    }

    /// 78.126 — THE HANDOFFS' COMMITMENT, where the fold consumes any: what
    /// its signature holds of them, under its own domain so it is never a
    /// children's commitment.
    pub fn handoffs_hash(&self) -> Option<[u8; 32]> {
        if self.handoffs.is_empty() {
            return None;
        }
        let mut h = b"handoffs".to_vec();
        put_handoffs(&mut h, &self.handoffs);
        Some(crate::chronometry::payload_hash(&h))
    }

    /// 78.123 B — THE CHILDREN'S COMMITMENT, where the fold nets any: what
    /// its signature and its receipt (`Cite::Chained`) hold of them.
    pub fn children_hash(&self) -> Option<[u8; 32]> {
        if self.children.is_empty() {
            return None;
        }
        let mut c = Vec::new();
        put_children(&mut c, &self.children);
        Some(crate::chronometry::payload_hash(&c))
    }

    /// CONTINUITY: on every lane, the identities' nets sum exactly to the
    /// declared net emission (sources less sinks) — nothing made or lost
    /// inside the fold but where it is declared. 78.123 B — a fold netting
    /// children balances what it includes, translated: on every lane, its
    /// rows less `Σ rate · included rows` sum exactly to its own declared
    /// (exact rationals; translation's rounding declared, never floored).
    pub fn continuity(&self) -> Result<(), Discontinuous> {
        self.continuity_with(&[])
    }

    /// 78.126 — CONTINUITY WITH THE HANDOFF SIDES THE FOLD CONSUMES: their
    /// nets — matter or words leaving (negative) or arriving (positive) across
    /// a domain's boundary — stand with its declared emission.
    pub fn continuity_with(&self, consumed: &[Net]) -> Result<(), Discontinuous> {
        use num_rational::BigRational as Q;
        if !ascending(&self.rows) || !ascending(&self.declared) {
            return Err(Discontinuous::Unordered);
        }
        let nets = sum(&self.rows).map_err(|lane| Discontinuous::Overflow { lane })?;
        let declared = sum(&self.declared).map_err(|lane| Discontinuous::Overflow { lane })?;
        let crossing = sum(consumed).map_err(|lane| Discontinuous::Overflow { lane })?;
        let mut emitted = [0i128; 4];
        for (l, e) in emitted.iter_mut().enumerate() {
            *e = declared[l].checked_add(crossing[l]).ok_or(Discontinuous::Overflow { lane: l })?;
        }
        let mut residual: [Q; 4] = std::array::from_fn(|l| Q::from_integer(nets[l].into()) - Q::from_integer(emitted[l].into()));
        for c in &self.children {
            for nets in [&c.rows, &c.declared, &c.included_rows, &c.included_declared] {
                if !ascending(nets) {
                    return Err(Discontinuous::Unordered);
                }
            }
            for (l, (n, d)) in c.rate.iter().enumerate() {
                let untranslated = l != 1 && (*n, *d) != (1, 1);
                if *n <= 0 || *d <= 0 || untranslated {
                    return Err(Discontinuous::Rate { lane: l });
                }
            }
            let included = sum(&c.included_rows).map_err(|lane| Discontinuous::Overflow { lane })?;
            for (l, r) in residual.iter_mut().enumerate() {
                let (n, d) = c.rate[l];
                *r -= Q::new(n.into(), d.into()) * Q::from_integer(included[l].into());
            }
        }
        match (0..4).find(|&l| !num_traits::Zero::is_zero(&residual[l])) {
            Some(lane) => Err(Discontinuous::Unbalanced { lane }),
            None => Ok(()),
        }
    }

    /// The fold signed by the tier's key — whose public half must be `tier`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedFold {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedFold { fold: self, sig }
    }
}

fn put_handoffs(out: &mut Vec<u8>, refs: &[([u8; 32], super::handoff::Side)]) {
    crate::transducer::address::put_key_count(out, refs.len());
    for (id, side) in refs {
        out.extend_from_slice(id);
        out.push(side.byte());
    }
}

fn take_handoffs(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], super::handoff::Side)>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let id: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            let side = super::handoff::Side::of(*b.get(*p + 32)?)?;
            *p += 33;
            Some((id, side))
        })
        .collect()
}

/// `[ACT_UPLINK][tier 32][prefix 32][rows][sig 64]`, and — 78.123 B, only
/// where the fold nets children or consumes handoffs — `[count][child …]`
/// after the signature, then — 78.126, only where it consumes handoffs —
/// `[count][(id, side) …]`; so a single-level fold's bytes are 78.119's
/// exactly and a chained one's 78.123's.
pub fn encode_uplink(f: &SignedFold) -> Vec<u8> {
    let mut out = vec![super::ACT_UPLINK];
    out.extend_from_slice(&f.fold.tier);
    out.extend_from_slice(&f.fold.prefix);
    out.extend_from_slice(&f.fold.rows_bytes());
    out.extend_from_slice(&f.sig);
    if !f.fold.children.is_empty() || !f.fold.handoffs.is_empty() {
        put_children(&mut out, &f.fold.children);
    }
    if !f.fold.handoffs.is_empty() {
        put_handoffs(&mut out, &f.fold.handoffs);
    }
    out
}

/// The fold a frame carries, read whole: a byte left over is no fold.
pub fn decode_uplink(b: &[u8]) -> Option<SignedFold> {
    let rest = b.strip_prefix(&[super::ACT_UPLINK])?;
    let mut p = 0;
    let tier: [u8; 32] = rest.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let prefix: [u8; 32] = rest.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let rows = take_nets(rest, &mut p)?;
    let declared = take_nets(rest, &mut p)?;
    let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    let children = match p == rest.len() {
        true => Vec::new(),
        false => take_children(rest, &mut p)?,
    };
    let handoffs = match p == rest.len() {
        true => Vec::new(),
        false => take_handoffs(rest, &mut p)?,
    };
    // ONE WRITING: an empty children block stands only before handoffs, and
    // an empty handoffs block never — what does not re-encode to these very
    // bytes is no fold.
    let fold = TierFold { tier, prefix, rows, declared, children, handoffs };
    let signed = SignedFold { fold, sig };
    (p == rest.len() && encode_uplink(&signed) == b).then_some(signed)
}

/// 78.123 B — WHY A CHILD WAS NOT NETTED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChildRefusal {
    /// Its stamp names no admitted fold on this court's line.
    NotOnLine,
    /// Its rows do not prove against the commitment its receipt holds.
    Mismatch,
    /// It was folded by a tier of another domain.
    ForeignDomain,
    /// What this parent includes passes what the child has left, on this
    /// lane: a part of opposite sign or greater magnitude — a double count.
    OverIncluded { lane: usize },
    /// 78.138 — a carried line's signature does not verify over its fold.
    Unsigned,
    /// 78.138 — a carried line has no bond of its own posted: each answers
    /// for its own work (as ruled).
    Unbonded,
    /// 78.138 — a carried line's own fold is not continuous, consumes a side
    /// it may not, or nets a child it may not.
    Broken,
    /// 78.138 — a carried line's fold over its line's current head: an empty
    /// span claiming nets.
    Repeat,
}

/// WHAT THE COURT ANSWERED.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Uplinked {
    /// Sealed on the court's line: the receipt's stamp and chain hash — the
    /// fold's receipt id.
    Admitted { at: Stamp, receipt: [u8; 32] },
    /// The tier's signature does not verify over the fold.
    Unsigned,
    /// The fold is not continuous, and why.
    Discontinuous(Discontinuous),
    /// The court's line could not take it.
    LineRefused,
    /// 78.120 — the tier key is named by no domain's genesis on this court.
    Unbound,
    /// 78.123 B — a fold netting children, sealed: its receipt id; whether
    /// it seals PROVISIONAL (an obstruction stands open in its domain's
    /// cover, so finality does not pass over it); the obstructions standing
    /// open; and how many of its children still have value in flight.
    Chained { at: Stamp, receipt: [u8; 32], provisional: bool, obstructions: u32, open_remainders: u32 },
    /// 78.123 B — the child at `index` was not netted, and why.
    ChildRefused { index: u32, why: ChildRefusal },
    /// 78.126 — the handoff side at `index` was not consumed, and why.
    HandoffRefused { index: u32, why: SideRefusal },
    /// 78.125 (as ruled) — a fold over its tier's current
    /// head whose commitments differ from the fold admitted there: an empty
    /// span claiming nets. (A fold byte-identical to it is answered with
    /// its original receipt, sealing nothing.)
    Repeat,
    /// 78.138 — only the ancients uplink: the folding tier is not its
    /// domain's reserve. A line's fold is carried in its reserve's.
    NotReserve,
    /// 78.154a — the fold's CONNECTION_TAX flow cannot be paid, and why.
    Tax(TaxRefusal),
}

/// 78.154a — THE CONNECTION TAX'S FLOW KIND on a tier fold: a DECLARED row
/// under this reserved coordinate is the universe's dissipation returning
/// to the hypha — a sink, on the existing lanes S, G, T, H exactly as
/// dissipated (no new lane: a lane names a physical dimension, the tax is a
/// purpose) — paid from the domain's reserve to the court's recycled pool
/// when the fold is admitted: a transfer, never a mint and never a burn.
pub const CONNECTION_TAX: [u8; 32] = *b"mycelium flow: connection tax v1";

/// 78.154a — WHY A FOLD'S CONNECTION TAX WAS NOT PAID (the fold is refused
/// whole).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaxRefusal {
    /// A lane of the flow is a source: the tax only leaves a universe.
    Source { lane: usize },
    /// The reserve's μ balance lacks it, on some lane.
    Short,
    /// A carried line declares it: the tax is paid from the reserve alone.
    Carried,
}

/// 78.126 — WHY A FOLD MAY NOT CONSUME A HANDOFF SIDE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideRefusal {
    /// No handoff of that id on this court.
    Unknown,
    /// The side is not this fold's tier's: OUT is the sender's; IN the
    /// receiver's once accepted, the sender's once returned.
    NotYours,
    /// IN while the handoff is still offered: neither accepted nor returned.
    NotYet,
    /// Consumed already — a double-spend.
    Consumed,
    /// Named twice in this fold.
    Repeated,
}

impl SideRefusal {
    const ALL: [SideRefusal; 5] = [SideRefusal::Unknown, SideRefusal::NotYours, SideRefusal::NotYet, SideRefusal::Consumed, SideRefusal::Repeated];

    fn byte(self) -> u8 {
        match self {
            SideRefusal::Unknown => 0,
            SideRefusal::NotYours => 1,
            SideRefusal::NotYet => 2,
            SideRefusal::Consumed => 3,
            SideRefusal::Repeated => 4,
        }
    }
}

/// The court's reply: `[0][stamp][hash]` admitted; `[1]` unsigned;
/// `[2][why][lane]` discontinuous; `[3]` the line refused; `[4]` unbound.
pub fn encode_uplink_reply(u: &Uplinked) -> Vec<u8> {
    let mut out = Vec::new();
    match u {
        Uplinked::Admitted { at, receipt } => {
            out.push(0);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
        }
        Uplinked::Unsigned => out.push(1),
        Uplinked::Discontinuous(d) => {
            out.push(2);
            let (why, lane) = match d {
                Discontinuous::Unordered => (0u8, 0usize),
                Discontinuous::Overflow { lane } => (1, *lane),
                Discontinuous::Unbalanced { lane } => (2, *lane),
                Discontinuous::Rate { lane } => (3, *lane),
            };
            out.push(why);
            out.extend_from_slice(&lane.to_le_bytes()[..1]);
        }
        Uplinked::LineRefused => out.push(3),
        Uplinked::Unbound => out.push(4),
        Uplinked::Chained { at, receipt, provisional, obstructions, open_remainders } => {
            out.push(5);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out.push(u8::from(*provisional));
            out.extend_from_slice(&obstructions.to_le_bytes());
            out.extend_from_slice(&open_remainders.to_le_bytes());
        }
        Uplinked::ChildRefused { index, why } => {
            out.push(6);
            out.extend_from_slice(&index.to_le_bytes());
            let (code, lane) = match why {
                ChildRefusal::NotOnLine => (0u8, 0usize),
                ChildRefusal::Mismatch => (1, 0),
                ChildRefusal::ForeignDomain => (2, 0),
                ChildRefusal::OverIncluded { lane } => (3, *lane),
                ChildRefusal::Unsigned => (4, 0),
                ChildRefusal::Unbonded => (5, 0),
                ChildRefusal::Broken => (6, 0),
                ChildRefusal::Repeat => (7, 0),
            };
            out.push(code);
            out.extend_from_slice(&lane.to_le_bytes()[..1]);
        }
        Uplinked::HandoffRefused { index, why } => {
            out.push(7);
            out.extend_from_slice(&index.to_le_bytes());
            out.push(why.byte());
        }
        Uplinked::Repeat => out.push(8),
        Uplinked::NotReserve => out.push(9),
        Uplinked::Tax(why) => {
            out.push(10);
            let (code, lane) = match why {
                TaxRefusal::Source { lane } => (0u8, *lane),
                TaxRefusal::Short => (1, 0),
                TaxRefusal::Carried => (2, 0),
            };
            out.push(code);
            out.extend_from_slice(&lane.to_le_bytes()[..1]);
        }
    }
    out
}

/// The reply read whole, or none.
pub fn decode_uplink_reply(b: &[u8]) -> Option<Uplinked> {
    let mut p = 1;
    let [tag, ..] = b else { return None };
    let u = match *tag {
        0 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Uplinked::Admitted { at, receipt }
        }
        1 => Uplinked::Unsigned,
        2 => {
            let (why, lane) = (*b.get(1)?, usize::from(*b.get(2)?));
            p = 3;
            Uplinked::Discontinuous(match why {
                0 => Discontinuous::Unordered,
                1 => Discontinuous::Overflow { lane },
                2 => Discontinuous::Unbalanced { lane },
                3 => Discontinuous::Rate { lane },
                _ => return None,
            })
        }
        3 => Uplinked::LineRefused,
        4 => Uplinked::Unbound,
        5 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            let provisional = match *b.get(p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            p += 1;
            let obstructions = u32::from_le_bytes(b.get(p..p + 4)?.try_into().ok()?);
            p += 4;
            let open_remainders = u32::from_le_bytes(b.get(p..p + 4)?.try_into().ok()?);
            p += 4;
            Uplinked::Chained { at, receipt, provisional, obstructions, open_remainders }
        }
        6 => {
            let index = u32::from_le_bytes(b.get(1..5)?.try_into().ok()?);
            let (code, lane) = (*b.get(5)?, usize::from(*b.get(6)?));
            p = 7;
            let why = match code {
                0 => ChildRefusal::NotOnLine,
                1 => ChildRefusal::Mismatch,
                2 => ChildRefusal::ForeignDomain,
                3 => ChildRefusal::OverIncluded { lane },
                4 => ChildRefusal::Unsigned,
                5 => ChildRefusal::Unbonded,
                6 => ChildRefusal::Broken,
                7 => ChildRefusal::Repeat,
                _ => return None,
            };
            Uplinked::ChildRefused { index, why }
        }
        7 => {
            let index = u32::from_le_bytes(b.get(1..5)?.try_into().ok()?);
            let code = *b.get(5)?;
            p = 6;
            Uplinked::HandoffRefused { index, why: SideRefusal::ALL.into_iter().find(|r| r.byte() == code)? }
        }
        8 => Uplinked::Repeat,
        9 => Uplinked::NotReserve,
        10 => {
            let (code, lane) = (*b.get(1)?, usize::from(*b.get(2)?));
            p = 3;
            Uplinked::Tax(match code {
                0 => TaxRefusal::Source { lane },
                1 => TaxRefusal::Short,
                2 => TaxRefusal::Carried,
                _ => return None,
            })
        }
        _ => return None,
    };
    (p == b.len()).then_some(u)
}
