// 78.120 — ACT_DOMAIN: a domain founded on a court (2(d), 78.62's GENESIS act;
// the operator's word 2026-10-07: "proceed").
//
//! A DOMAIN'S GENESIS names the domain's key and the tier keys that settle for
//! it, signed by the domain's key and sealed on the court's line
//! (`Cite::Domain`, keys only — coordinates). From then on a tier's fold
//! (`ACT_UPLINK`) is admitted only from a key a sealed genesis names: a
//! fold's signature means the domain stands behind the tier. A domain is
//! founded once; granting and revoking tier keys are later rows.

use crate::chronometry::Stamp;

/// The signed bytes' domain, so a genesis signature is never a signature on
/// anything else.
///
/// 78.122 — KEPT AS FIRST SIGNED: the network's "world" became "domain" (the
/// shared lexicon), and every name moved but this one — a domain founded
/// before the rename was signed over these bytes, and a re-founding signed
/// over others would not verify.
const DOMAIN: &[u8] = b"mycelium world genesis v1";

/// A DOMAIN'S GENESIS: its key, and its tier keys in their own ascending
/// order, each once.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainGenesis {
    pub domain: [u8; 32],
    pub tiers: Vec<[u8; 32]>,
}

/// The genesis and the domain key's signature over [`DomainGenesis::signed_bytes`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedDomain {
    pub genesis: DomainGenesis,
    pub sig: [u8; 64],
}

/// Keys written canonically: their count, then each.
pub(crate) fn put_keys(out: &mut Vec<u8>, keys: &[[u8; 32]]) {
    crate::transducer::address::put_key_count(out, keys.len());
    for k in keys {
        out.extend_from_slice(k);
    }
}

/// Keys read back one by one, never reserved by the count.
pub(crate) fn take_keys(b: &[u8], p: &mut usize) -> Option<Vec<[u8; 32]>> {
    let n = crate::transducer::address::take_key_count(b, p)?;
    (0..n)
        .map(|_| {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        })
        .collect()
}

impl DomainGenesis {
    /// Whether the tier keys stand in their one writing: ascending, each once.
    pub fn ordered(&self) -> bool {
        self.tiers.windows(2).all(|w| matches!(w, [a, b] if a < b))
    }

    /// The bytes the domain signs: the domain, the domain, its tier keys.
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = DOMAIN.to_vec();
        out.extend_from_slice(&self.domain);
        put_keys(&mut out, &self.tiers);
        out
    }

    /// The genesis signed by the domain's key — whose public half is `domain`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedDomain {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedDomain { genesis: self, sig }
    }
}

/// `[ACT_DOMAIN][domain 32][tiers][sig 64]`.
pub fn encode_domain(w: &SignedDomain) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&w.genesis.domain);
    put_keys(&mut out, &w.genesis.tiers);
    out.extend_from_slice(&w.sig);
    out
}

/// The genesis a frame carries, read whole: a byte left over is none.
pub fn decode_domain(b: &[u8]) -> Option<SignedDomain> {
    let rest = b.strip_prefix(&[super::ACT_DOMAIN])?;
    let mut p = 0;
    let domain: [u8; 32] = rest.get(p..p + 32)?.try_into().ok()?;
    p += 32;
    let tiers = take_keys(rest, &mut p)?;
    let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    (p == rest.len()).then_some(SignedDomain { genesis: DomainGenesis { domain, tiers }, sig })
}

/// 78.127 — THE DOMAIN'S RESERVE TIER DESIGNATED (2(e), the coordinator's
/// rulings): the one tier of the domain whose μ balance on this court is its
/// RESERVE — the only tier that converts game value against μ, and the one a
/// falsified fold's slash draws from. Signed by the domain's key; a domain
/// with none takes it at once, a redesignation from the reserve tier's next
/// admitted fold (its era anchor), never mid-era.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reservation {
    pub domain: [u8; 32],
    pub tier: [u8; 32],
    pub nonce: [u8; 32],
}

const RESERVE: &[u8] = b"mycelium domain reserve v1";

/// A SUB-KIND OF ACT_DOMAIN, marked where a genesis writes its tier count by
/// a count no genesis can write — `0x80 0x00`, an over-long spelling the key
/// count refuses — so every genesis frame reads as it did, and an older
/// reader refuses a designation by name.
const SUB_KIND: [u8; 2] = [0x80, 0x00];
const KIND_RESERVE: u8 = 1;

impl Reservation {
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = RESERVE.to_vec();
        for k in [&self.domain, &self.tier, &self.nonce] {
            out.extend_from_slice(k);
        }
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedReservation {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedReservation { reservation: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedReservation {
    pub reservation: Reservation,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][1][tier 32][nonce 32][sig 64]`.
pub fn encode_reservation(r: &SignedReservation) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&r.reservation.domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_RESERVE);
    out.extend_from_slice(&r.reservation.tier);
    out.extend_from_slice(&r.reservation.nonce);
    out.extend_from_slice(&r.sig);
    out
}

/// WHAT AN ACT_DOMAIN FRAME ASKS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DomainRequest {
    Genesis(SignedDomain),
    Reserve(SignedReservation),
}

/// The request a frame carries, read whole, or none.
pub fn decode_domain_request(b: &[u8]) -> Option<DomainRequest> {
    let rest = b.strip_prefix(&[super::ACT_DOMAIN])?;
    if rest.get(32..34) != Some(&SUB_KIND[..]) {
        return decode_domain(b).map(DomainRequest::Genesis);
    }
    let domain: [u8; 32] = rest.get(..32)?.try_into().ok()?;
    let mut p = 34;
    if *rest.get(p)? != KIND_RESERVE {
        return None;
    }
    p += 1;
    let mut key = || -> Option<[u8; 32]> {
        let k: [u8; 32] = rest.get(p..p + 32)?.try_into().ok()?;
        p += 32;
        Some(k)
    };
    let (tier, nonce) = (key()?, key()?);
    let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
    p += 64;
    (p == rest.len()).then_some(DomainRequest::Reserve(SignedReservation { reservation: Reservation { domain, tier, nonce }, sig }))
}

/// WHAT THE COURT ANSWERED a domain's genesis.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Founded {
    /// Sealed on the court's line: the receipt's stamp and chain hash.
    Sealed { at: Stamp, receipt: [u8; 32] },
    /// The domain's signature does not verify over the genesis.
    Unsigned,
    /// The tier keys are not ascending, each once.
    Unordered,
    /// This domain is founded on this court already.
    AlreadyFounded,
    /// A tier key is named by another domain's genesis already: a tier
    /// settles for one domain.
    TierHeld,
    /// The court's line could not take it.
    LineRefused,
    /// 78.127 — the reserve designated: sealed, and whether it holds now (a
    /// domain's first) or from its current reserve tier's next fold.
    Reserved { at: Stamp, receipt: [u8; 32], now: bool },
    /// 78.127 — the tier is not one the domain's genesis names.
    NotATier,
    /// 78.127 — no such domain is founded on this court.
    NotFounded,
}

/// `[0][stamp][hash]` sealed; `[1]` unsigned; `[2]` unordered; `[3]` already
/// founded; `[4]` a tier held; `[5]` the line refused.
pub fn encode_domain_reply(f: &Founded) -> Vec<u8> {
    let mut out = Vec::new();
    match f {
        Founded::Sealed { at, receipt } => {
            out.push(0);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
        }
        Founded::Unsigned => out.push(1),
        Founded::Unordered => out.push(2),
        Founded::AlreadyFounded => out.push(3),
        Founded::TierHeld => out.push(4),
        Founded::LineRefused => out.push(5),
        Founded::Reserved { at, receipt, now } => {
            out.push(6);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out.push(u8::from(*now));
        }
        Founded::NotATier => out.push(7),
        Founded::NotFounded => out.push(8),
    }
    out
}

/// The reply read whole, or none.
pub fn decode_domain_reply(b: &[u8]) -> Option<Founded> {
    let [tag, ..] = b else { return None };
    let mut p = 1;
    let f = match *tag {
        0 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Founded::Sealed { at, receipt }
        }
        1 => Founded::Unsigned,
        2 => Founded::Unordered,
        3 => Founded::AlreadyFounded,
        4 => Founded::TierHeld,
        5 => Founded::LineRefused,
        6 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            let now = match *b.get(p + 32)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            p += 33;
            Founded::Reserved { at, receipt, now }
        }
        7 => Founded::NotATier,
        8 => Founded::NotFounded,
        _ => return None,
    };
    (p == b.len()).then_some(f)
}
