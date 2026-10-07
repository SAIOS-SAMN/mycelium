// 78.119 — ACT_UPLINK: a kernel's settlement tier sends up its fold (78.62
// step 3, the operator's ruling 2026-10-07: "admit folds, not every
// transition").
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
}

/// 78.123 B — ONE FOLD A PARENT NETS: the child's receipt on this court's
/// line; the child's own rows and declared (proved against the commitment
/// its receipt holds); the part of each this parent includes — a child
/// netted by several parents is partitioned among them — and the rate per
/// lane the parent translates it at (strictly positive; by the lexicon v2,
/// S, T and H exactly 1, only G translates).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChildNet {
    pub at: Stamp,
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

    fn put(&self, out: &mut Vec<u8>) {
        self.at.put_canonical(out);
        for nets in [&self.rows, &self.declared, &self.included_rows, &self.included_declared] {
            put_nets(out, nets);
        }
        for (n, d) in self.rate {
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
    }

    fn take(b: &[u8], p: &mut usize) -> Option<ChildNet> {
        let at = Stamp::take_canonical(b, p)?;
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
        Some(ChildNet { at, rows, declared, included_rows, included_declared, rate })
    }
}

pub(crate) fn put_children(out: &mut Vec<u8>, children: &[ChildNet]) {
    crate::transducer::address::put_key_count(out, children.len());
    for c in children {
        c.put(out);
    }
}

pub(crate) fn take_children(b: &[u8], p: &mut usize) -> Option<Vec<ChildNet>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n).map(|_| ChildNet::take(b, p)).collect()
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
        out
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
        use num_rational::BigRational as Q;
        if !ascending(&self.rows) || !ascending(&self.declared) {
            return Err(Discontinuous::Unordered);
        }
        let nets = sum(&self.rows).map_err(|lane| Discontinuous::Overflow { lane })?;
        let emitted = sum(&self.declared).map_err(|lane| Discontinuous::Overflow { lane })?;
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

/// `[ACT_UPLINK][tier 32][prefix 32][rows][sig 64]`, and — 78.123 B, only
/// where the fold nets children — `[count][child …]` after the signature, so
/// a single-level fold's bytes are 78.119's exactly.
pub fn encode_uplink(f: &SignedFold) -> Vec<u8> {
    let mut out = vec![super::ACT_UPLINK];
    out.extend_from_slice(&f.fold.tier);
    out.extend_from_slice(&f.fold.prefix);
    out.extend_from_slice(&f.fold.rows_bytes());
    out.extend_from_slice(&f.sig);
    if !f.fold.children.is_empty() {
        put_children(&mut out, &f.fold.children);
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
        false => {
            let n = crate::transducer::address::take_key_count(rest, &mut p)?;
            let children = (0..n).map(|_| ChildNet::take(rest, &mut p)).collect::<Option<Vec<_>>>()?;
            if children.is_empty() {
                return None;
            }
            children
        }
    };
    (p == rest.len()).then_some(SignedFold { fold: TierFold { tier, prefix, rows, declared, children }, sig })
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
                _ => return None,
            };
            Uplinked::ChildRefused { index, why }
        }
        _ => return None,
    };
    (p == b.len()).then_some(u)
}
