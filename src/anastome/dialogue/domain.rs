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
    /// A domain's reserve read; nothing sealed.
    ReadReserve([u8; 32]),
    /// 78.129 — a tier's bond posted (topped up) or withdrawn by the reserve.
    Bond(SignedBond),
    /// 78.129 — a tier's bond read; nothing sealed.
    ReadBond { domain: [u8; 32], tier: [u8; 32] },
    /// 78.132 — a universe's origin recorded, once.
    Origin(SignedOrigin),
    /// 78.132 — a domain's origin read; nothing sealed.
    ReadOrigin([u8; 32]),
    /// 78.139 — a universe's curvature, basket or join, stated once by its
    /// sitting reserve.
    State(super::universe::SignedStatement),
    /// 78.139 — what a universe has stated, read; nothing sealed.
    ReadUniverse([u8; 32]),
    /// 78.154 — a universe's LEDGER entry, by its reserve.
    Ledger(Box<super::atlas::SignedLedger>),
    /// 78.154 — a universe's ledger after `u`, read; nothing sealed.
    ReadLedger { domain: [u8; 32], after: u64 },
    /// 78.154 — a MAP record posted by the universe's reserve.
    Map(Box<super::atlas::SignedMap>),
    /// 78.154 — a universe's map, read; nothing sealed.
    ReadMap([u8; 32]),
    /// 78.154 — a TRAVERSE: coordinates retrieved and the crossing mapped.
    Traverse(Box<super::crossing::SignedTraverse>),
}

const KIND_READ_RESERVE: u8 = 2;
const KIND_BOND: u8 = 3;
const KIND_UNBOND: u8 = 4;
const KIND_READ_BOND: u8 = 5;
const KIND_ORIGIN: u8 = 6;
const KIND_READ_ORIGIN: u8 = 7;

/// 78.132 — A UNIVERSE'S ORIGIN (the coordinator's ruling: one network
/// universe = one Big Bang = one domain, 78.86): the commitment — a 32-byte
/// hash of the universe's Big Bang record (its seed, laws as ratios, drawn
/// parameters, the parent bounce's mass and spin) — with the record's kind
/// and version. Signed by the domain's key, held ONCE per domain and never
/// edited (78.59 Ruling 2): a wrong origin is remedied by a new universe and
/// domain. A domain with an origin reads as a universe; its folds descend
/// from it by line precedence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Origin {
    pub domain: [u8; 32],
    pub kind: u8,
    pub version: u8,
    pub commitment: [u8; 32],
    /// 78.139 — version 2: the universe's curvature radius `R` in clear and
    /// its PARENT FACE — the core tube it bounced out of, or unwitnessed (a
    /// root). Present exactly where `version` is 2.
    pub universe: Option<super::universe::Genesis>,
}

const ORIGIN: &[u8] = b"mycelium universe origin v1";

impl Origin {
    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = ORIGIN.to_vec();
        out.extend_from_slice(&self.domain);
        out.extend_from_slice(&[self.kind, self.version]);
        out.extend_from_slice(&self.commitment);
        if let Some(u) = &self.universe {
            u.put(&mut out);
        }
        out
    }

    /// Signed by the domain's key, whose public half is `domain`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedOrigin {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedOrigin { origin: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedOrigin {
    pub origin: Origin,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][6][kind][version][commitment 32][sig 64]`.
pub fn encode_origin(o: &SignedOrigin) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&o.origin.domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_ORIGIN);
    out.extend_from_slice(&[o.origin.kind, o.origin.version]);
    out.extend_from_slice(&o.origin.commitment);
    if let Some(u) = &o.origin.universe {
        u.put(&mut out);
    }
    out.extend_from_slice(&o.sig);
    out
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][7]`.
pub fn encode_origin_read(domain: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_READ_ORIGIN);
    out
}

/// 78.132 — A UNIVERSE'S ORIGIN AS HELD: the record's kind, version and
/// commitment, and the stamp and receipt it was sealed under — what a holder
/// proves line precedence against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OriginReading {
    pub kind: u8,
    pub version: u8,
    pub commitment: [u8; 32],
    pub at: Stamp,
    pub receipt: [u8; 32],
}

impl OriginReading {
    pub fn put(&self, out: &mut Vec<u8>) {
        out.extend_from_slice(&[self.kind, self.version]);
        out.extend_from_slice(&self.commitment);
        self.at.put_canonical(out);
        out.extend_from_slice(&self.receipt);
    }

    pub fn take(b: &[u8], p: &mut usize) -> Option<OriginReading> {
        let (kind, version) = (*b.get(*p)?, *b.get(*p + 1)?);
        *p += 2;
        let commitment: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        let at = Stamp::take_canonical(b, p)?;
        let receipt: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(OriginReading { kind, version, commitment, at, receipt })
    }
}

/// `[0]` no origin; `[1][kind][version][commitment 32][stamp][receipt 32]`.
pub fn encode_origin_reading(r: Option<&OriginReading>) -> Vec<u8> {
    match r {
        None => vec![0],
        Some(r) => {
            let mut out = vec![1];
            r.put(&mut out);
            out
        }
    }
}

/// The reading read whole: `Some(None)` where the domain has no origin.
pub fn decode_origin_reading(b: &[u8]) -> Option<Option<OriginReading>> {
    match b.split_first()? {
        (0, []) => Some(None),
        (1, rest) => {
            let mut p = 0;
            let r = OriginReading::take(rest, &mut p)?;
            (p == rest.len()).then_some(Some(r))
        }
        _ => None,
    }
}

/// 78.129 — A TIER'S BOND (the coordinator's ruling): posted by the domain's
/// RESERVE for one tier it backs — μ the ancients hold, escrowed under the
/// tier's name — topped up by posting again (`mu: Some`), or WITHDRAWN
/// (`mu: None`) once every fold of the tier is final. The tier that signed a
/// lie is the one at stake: a falsified fold slashes half its bond.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bond {
    pub domain: [u8; 32],
    pub reserve: [u8; 32],
    pub tier: [u8; 32],
    pub mu: Option<[u64; 4]>,
    pub nonce: [u8; 32],
}

const BOND: &[u8] = b"mycelium bond v1";

impl Bond {
    fn kind(&self) -> u8 {
        match self.mu {
            Some(_) => KIND_BOND,
            None => KIND_UNBOND,
        }
    }

    fn body(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.reserve);
        out.extend_from_slice(&self.tier);
        if let Some(mu) = self.mu {
            for m in mu {
                out.extend_from_slice(&m.to_le_bytes());
            }
        }
        out.extend_from_slice(&self.nonce);
        out
    }

    pub fn signed_bytes(&self) -> Vec<u8> {
        let mut out = BOND.to_vec();
        out.push(self.kind());
        out.extend_from_slice(&self.domain);
        out.extend_from_slice(&self.body());
        out
    }

    /// Signed by the reserve tier, whose public half is `reserve`.
    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedBond {
        use ed25519_dalek::Signer;
        let sig = key.sign(&self.signed_bytes()).to_bytes();
        SignedBond { bond: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedBond {
    pub bond: Bond,
    pub sig: [u8; 64],
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][3|4][reserve 32][tier 32][mu 4×u64 — 3 only][nonce 32][sig 64]`.
pub fn encode_bond(b: &SignedBond) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&b.bond.domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(b.bond.kind());
    out.extend_from_slice(&b.bond.body());
    out.extend_from_slice(&b.sig);
    out
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][5][tier 32]`.
pub fn encode_bond_read(domain: [u8; 32], tier: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_READ_BOND);
    out.extend_from_slice(&tier);
    out
}

/// A bond as read: its μ per lane (zero where none is posted).
pub fn encode_bond_reading(mu: &[u64; 4]) -> Vec<u8> {
    mu.iter().flat_map(|m| m.to_le_bytes()).collect()
}

pub fn decode_bond_reading(b: &[u8]) -> Option<[u64; 4]> {
    if b.len() != 32 {
        return None;
    }
    let mut mu = [0u64; 4];
    for (m, c) in mu.iter_mut().zip(b.chunks_exact(8)) {
        *m = u64::from_le_bytes(c.try_into().ok()?);
    }
    Some(mu)
}

/// `[ACT_DOMAIN][domain 32][0x80 0x00][2]`.
pub fn encode_reserve_read(domain: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_DOMAIN];
    out.extend_from_slice(&domain);
    out.extend_from_slice(&SUB_KIND);
    out.push(KIND_READ_RESERVE);
    out
}

/// 78.127 — A DOMAIN'S RESERVE AS READ: the tier whose balance it is now,
/// a redesignation pending until that tier's next fold, and the reserve's μ
/// balance per lane.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReserveReading {
    pub current: Option<[u8; 32]>,
    pub pending: Option<[u8; 32]>,
    pub balance: [u64; 4],
}

pub fn encode_reserve_reading(r: &ReserveReading) -> Vec<u8> {
    let mut out = Vec::new();
    for t in [r.current, r.pending] {
        match t {
            None => out.push(0),
            Some(t) => {
                out.push(1);
                out.extend_from_slice(&t);
            }
        }
    }
    for m in r.balance {
        out.extend_from_slice(&m.to_le_bytes());
    }
    out
}

pub fn decode_reserve_reading(b: &[u8]) -> Option<ReserveReading> {
    let mut p = 0;
    let mut tier = || -> Option<Option<[u8; 32]>> {
        let flag = *b.get(p)?;
        p += 1;
        match flag {
            0 => Some(None),
            1 => {
                let t: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
                p += 32;
                Some(Some(t))
            }
            _ => None,
        }
    };
    let (current, pending) = (tier()?, tier()?);
    let mut balance = [0u64; 4];
    for m in &mut balance {
        *m = u64::from_le_bytes(b.get(p..p + 8)?.try_into().ok()?);
        p += 8;
    }
    (p == b.len()).then_some(ReserveReading { current, pending, balance })
}

/// The request a frame carries, read whole, or none.
pub fn decode_domain_request(b: &[u8]) -> Option<DomainRequest> {
    let rest = b.strip_prefix(&[super::ACT_DOMAIN])?;
    if rest.get(32..34) != Some(&SUB_KIND[..]) {
        return decode_domain(b).map(DomainRequest::Genesis);
    }
    let domain: [u8; 32] = rest.get(..32)?.try_into().ok()?;
    let mut p = 34;
    let key = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
        let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(k)
    };
    match *rest.get(p)? {
        KIND_READ_RESERVE => return (rest.len() == p + 1).then_some(DomainRequest::ReadReserve(domain)),
        KIND_READ_ORIGIN => return (rest.len() == p + 1).then_some(DomainRequest::ReadOrigin(domain)),
        KIND_ORIGIN => {
            p += 1;
            let (kind, version) = (*rest.get(p)?, *rest.get(p + 1)?);
            p += 2;
            let commitment = key(rest, &mut p)?;
            // 78.139 — version 2 carries the universe's curvature and parent.
            let universe = match version {
                2 => Some(super::universe::Genesis::take(rest, &mut p)?),
                _ => None,
            };
            let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            let origin = Origin { domain, kind, version, commitment, universe };
            return (p == rest.len()).then_some(DomainRequest::Origin(SignedOrigin { origin, sig }));
        }
        // 78.139 — a universe's statements (geometry, basket, join) and its read.
        super::universe::KIND_READ_UNIVERSE => return (rest.len() == p + 1).then_some(DomainRequest::ReadUniverse(domain)),
        k @ (super::universe::KIND_GEOMETRY | super::universe::KIND_BASKET | super::universe::KIND_JOIN) => {
            p += 1;
            let statement = super::universe::Statement::take(k, domain, rest, &mut p)?;
            let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            return (p == rest.len()).then_some(DomainRequest::State(super::universe::SignedStatement { statement, sig }));
        }
        KIND_READ_BOND => {
            p += 1;
            let tier = key(rest, &mut p)?;
            return (p == rest.len()).then_some(DomainRequest::ReadBond { domain, tier });
        }
        k @ (KIND_BOND | KIND_UNBOND) => {
            p += 1;
            let (reserve, tier) = (key(rest, &mut p)?, key(rest, &mut p)?);
            let mu = match k {
                KIND_BOND => {
                    let mut mu = [0u64; 4];
                    for m in &mut mu {
                        *m = u64::from_le_bytes(rest.get(p..p + 8)?.try_into().ok()?);
                        p += 8;
                    }
                    Some(mu)
                }
                _ => None,
            };
            let nonce = key(rest, &mut p)?;
            let sig: [u8; 64] = rest.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            let bond = Bond { domain, reserve, tier, mu, nonce };
            return (p == rest.len()).then_some(DomainRequest::Bond(SignedBond { bond, sig }));
        }
        // 78.154 — the ledger, the map and the traverse, and their reads.
        super::atlas::KIND_LEDGER => {
            p += 1;
            let s = super::atlas::take_ledger(domain, rest, &mut p)?;
            return (p == rest.len()).then_some(DomainRequest::Ledger(Box::new(s)));
        }
        super::atlas::KIND_READ_LEDGER => {
            p += 1;
            let after = super::atlas::u64_at(rest, &mut p)?;
            return (p == rest.len()).then_some(DomainRequest::ReadLedger { domain, after });
        }
        super::atlas::KIND_MAP => {
            p += 1;
            let s = super::atlas::take_map(domain, rest, &mut p)?;
            return (p == rest.len()).then_some(DomainRequest::Map(Box::new(s)));
        }
        super::atlas::KIND_READ_MAP => return (rest.len() == p + 1).then_some(DomainRequest::ReadMap(domain)),
        super::atlas::KIND_TRAVERSE => {
            p += 1;
            let s = super::crossing::take_traverse(domain, rest, &mut p)?;
            return (p == rest.len()).then_some(DomainRequest::Traverse(Box::new(s)));
        }
        KIND_RESERVE => {}
        _ => return None,
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
    /// 78.129 — a bond posted or withdrawn: sealed, and the tier's bond now.
    Bonded { at: Stamp, receipt: [u8; 32], bond: [u64; 4] },
    /// 78.129 — the bond's signer is not the domain's reserve tier.
    NotReserve,
    /// 78.129 — the reserve holds less μ than the bond posts.
    Short,
    /// 78.129 — a fold of the tier is not yet final: no bond withdrawn while
    /// one is slashable.
    Outstanding,
    /// 78.129 — no bond stands for the tier.
    Unbonded,
    /// 78.132 — the origin sealed: the domain reads as a universe from here.
    Originated { at: Stamp, receipt: [u8; 32] },
    /// 78.132 — the domain holds its origin already; never an edit.
    AlreadyOriginated,
    /// 78.139 — a universe's statement sealed: its curvature, its basket, or
    /// its join to its parent's core.
    Stated { at: Stamp, receipt: [u8; 32] },
    /// 78.139 — stated already, once; never an edit.
    AlreadyStated,
    /// 78.139 — the tube named is not an open core tube whose far face is
    /// this domain.
    NotACore,
    /// 78.139 — the domain has no origin yet: not a universe.
    NotAUniverse,
    /// 78.139 — the statement is not well formed: a curvature not positive,
    /// a basket empty, unordered or with a mass not positive, or an origin
    /// whose version and fields disagree.
    Malformed,
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
        Founded::Bonded { at, receipt, bond } => {
            out.push(9);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out.extend_from_slice(&encode_bond_reading(bond));
        }
        Founded::NotReserve => out.push(10),
        Founded::Short => out.push(11),
        Founded::Outstanding => out.push(12),
        Founded::Unbonded => out.push(13),
        Founded::AlreadyOriginated => out.push(14),
        Founded::Originated { at, receipt } => {
            out.push(15);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
        }
        Founded::Stated { at, receipt } => {
            out.push(16);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
        }
        Founded::AlreadyStated => out.push(17),
        Founded::NotACore => out.push(18),
        Founded::NotAUniverse => out.push(19),
        Founded::Malformed => out.push(20),
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
        9 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            let bond = decode_bond_reading(b.get(p + 32..p + 64)?)?;
            p += 64;
            Founded::Bonded { at, receipt, bond }
        }
        10 => Founded::NotReserve,
        11 => Founded::Short,
        12 => Founded::Outstanding,
        13 => Founded::Unbonded,
        14 => Founded::AlreadyOriginated,
        15 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Founded::Originated { at, receipt }
        }
        16 => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Founded::Stated { at, receipt }
        }
        17 => Founded::AlreadyStated,
        18 => Founded::NotACore,
        19 => Founded::NotAUniverse,
        20 => Founded::Malformed,
        _ => return None,
    };
    (p == b.len()).then_some(f)
}
