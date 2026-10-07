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
}

fn put_nets(out: &mut Vec<u8>, nets: &[Net]) {
    crate::transducer::address::put_key_count(out, nets.len());
    for (who, lanes) in nets {
        out.extend_from_slice(who);
        for l in lanes {
            out.extend_from_slice(&l.to_le_bytes());
        }
    }
}

fn take_nets(b: &[u8], p: &mut usize) -> Option<Vec<Net>> {
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
    /// the rows' commitment.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = DOMAIN.to_vec();
        out.extend_from_slice(&self.tier);
        out.extend_from_slice(&self.prefix);
        out.extend_from_slice(&self.rows_hash());
        out
    }

    /// CONTINUITY: on every lane, the identities' nets sum exactly to the
    /// declared net emission (sources less sinks) — nothing made or lost
    /// inside the fold but where it is declared.
    pub fn continuity(&self) -> Result<(), Discontinuous> {
        if !ascending(&self.rows) || !ascending(&self.declared) {
            return Err(Discontinuous::Unordered);
        }
        let nets = sum(&self.rows).map_err(|lane| Discontinuous::Overflow { lane })?;
        let emitted = sum(&self.declared).map_err(|lane| Discontinuous::Overflow { lane })?;
        match (0..4).find(|&l| nets[l] != emitted[l]) {
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

/// `[ACT_UPLINK][tier 32][prefix 32][rows][sig 64]`.
pub fn encode_uplink(f: &SignedFold) -> Vec<u8> {
    let mut out = vec![super::ACT_UPLINK];
    out.extend_from_slice(&f.fold.tier);
    out.extend_from_slice(&f.fold.prefix);
    out.extend_from_slice(&f.fold.rows_bytes());
    out.extend_from_slice(&f.sig);
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
    (p == rest.len()).then_some(SignedFold { fold: TierFold { tier, prefix, rows, declared }, sig })
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
}

/// The court's reply: `[0][stamp][hash]` admitted; `[1]` unsigned;
/// `[2][why][lane]` discontinuous; `[3]` the line refused.
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
            };
            out.push(why);
            out.extend_from_slice(&lane.to_le_bytes()[..1]);
        }
        Uplinked::LineRefused => out.push(3),
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
                _ => return None,
            })
        }
        3 => Uplinked::LineRefused,
        _ => return None,
    };
    (p == b.len()).then_some(u)
}
