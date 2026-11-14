// 78.155b — A UNIVERSE'S PACKAGE, AND THE GRANT THAT SPONSORS IT (ruled
// 2026-10-14: genesis stays earned-only; in the development sandbox
// the hypha sponsors new universes' founding packages until a market exists
// where μ is bought, staked and yields). Two sub-kinds of ACT_DOMAIN.
//
//! A MANIFEST (sub-kind 17) is a universe's request for a package: an exact
//! COUNT OF NUCLEI per nucleus `(Z protons, Q neutrons)`, the coupled
//! nucleon dipole (78.184 §3, ruled 2026-11-13: Z is coupled to Q, never
//! keyed by Z alone; MU-EQUATIONS §6.13), so isotopes stay apart. Each
//! kernel translates its own letters onto protons and neutrons at its
//! uplink; the hypha reads the pair, never sets it. Signed by the domain's SITTING RESERVE, as its
//! statements are. The court answers it with a GRANT: a transfer from its
//! own earned μ to the reserve, never a mint — the first manifest after the
//! origin is the FOUNDING grant, each later one a GROWTH grant. READ-ENDOWMENT
//! (sub-kind 18) reads what the court can grant and what it has granted.
//!
//! ```text
//! MANIFEST       [ACT_DOMAIN][domain 32][0x80 0x00][17][count u16 LE]
//!                (Z u16 LE, Q u32 LE, count: Mv scalar, an integer ≥ 1)… [nonce 32][sig 64]
//! READ-ENDOWMENT [ACT_DOMAIN][domain 32][0x80 0x00][18]
//! ```
//!
//! Signed bytes: `"mycelium manifest v2" ‖ domain ‖ the entries as written ‖
//! nonce`. (v1 keyed each entry by Z alone and summed the neutrons away,
//! retired 2026-11-13.)

use super::atlas::{byte, head, key32};
use super::crossing_record::{crossed, Crossing};
use crate::chronometry::Stamp;
use crate::economy::Mu;
use crate::transducer::cga::Mv;
use num_bigint::BigUint;
use num_rational::BigRational as Q;

/// FORMAT: sub-kind byte of ACT_DOMAIN for a manifest.
pub const KIND_MANIFEST: u8 = 17;
/// FORMAT: sub-kind byte of ACT_DOMAIN for an endowment read.
pub const KIND_READ_ENDOWMENT: u8 = 18;
const MANIFEST: &[u8] = b"mycelium manifest v2";

/// ONE NUCLEUS, the coupled nucleon dipole (78.184 §3): `z` protons and `q`
/// neutrons, ordered protons first.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nucleus {
    pub z: u16,
    pub q: u32,
}

/// NUCLEI PER NUCLEUS: ascending `(Z, Q)`, each once, every count a positive
/// integer of any size (a package is astronomical).
pub type Nuclei = Vec<(Nucleus, BigUint)>;

/// The nuclei well formed: not empty, `(Z, Q)` strictly ascending, every
/// nucleus with a proton, every count at least one.
pub fn nuclei_hold(n: &Nuclei) -> bool {
    let zero = BigUint::from(0u8);
    !n.is_empty() && n.windows(2).all(|w| matches!(w, [x, y] if x.0 < y.0)) && n.iter().all(|(k, c)| k.z > 0 && *c > zero)
}

pub(crate) fn put_nuclei(out: &mut Vec<u8>, n: &Nuclei) -> Option<()> {
    out.extend_from_slice(&u16::try_from(n.len()).ok()?.to_le_bytes());
    for (k, c) in n {
        out.extend_from_slice(&k.z.to_le_bytes());
        out.extend_from_slice(&k.q.to_le_bytes());
        Mv::scalar(Q::from_integer(num_bigint::BigInt::from(c.clone()))).to_bytes(out);
    }
    Some(())
}

/// The nuclei read whole: a count that is no positive integer is no count.
pub(crate) fn take_nuclei(b: &[u8], p: &mut usize) -> Option<Nuclei> {
    let n = usize::from(u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?));
    *p += 2;
    (0..n)
        .map(|_| {
            let z = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
            *p += 2;
            let neutrons = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            let count = Mv::from_bytes(b, p)?.scalar_part();
            if !count.is_integer() {
                return None;
            }
            let c = count.to_integer().to_biguint()?;
            Some((Nucleus { z, q: neutrons }, c))
        })
        .collect()
}

/// A UNIVERSE'S MANIFEST: the domain, its nuclei per Z, and a nonce that
/// makes two equal requests two manifests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub domain: [u8; 32],
    pub nuclei: Nuclei,
    pub nonce: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedManifest {
    pub manifest: Manifest,
    /// X1 (#160) — the crossing the manifest carries.
    pub crossing: Crossing,
    pub sig: [u8; 64],
}

impl SignedManifest {
    /// THE BYTES THE SIGNATURE IS OVER: the manifest's own, then its
    /// crossing written out whole. The manifest's hash stays its own
    /// bytes'.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        crossed(self.manifest.signed_bytes()?, &self.crossing)
    }
}

impl Manifest {
    /// `None` where a count passes the carrier its length writes.
    pub fn signed_bytes(&self) -> Option<Vec<u8>> {
        let mut out = MANIFEST.to_vec();
        out.extend_from_slice(&self.domain);
        put_nuclei(&mut out, &self.nuclei)?;
        out.extend_from_slice(&self.nonce);
        Some(out)
    }

    /// THE MANIFEST'S HASH: the citation hash of exactly its signed bytes —
    /// what the grant and the universe's ledger cite.
    pub fn hash(&self) -> Option<[u8; 32]> {
        Some(crate::chronometry::payload_hash(&self.signed_bytes()?))
    }

    /// Signed by the domain's sitting reserve, with the crossing it carries
    /// (X1, #160).
    pub fn sign(self, crossing: Crossing, key: &ed25519_dalek::SigningKey) -> Option<SignedManifest> {
        use ed25519_dalek::Signer;
        let sig = key.sign(&crossed(self.signed_bytes()?, &crossing)?).to_bytes();
        Some(SignedManifest { manifest: self, crossing, sig })
    }
}

/// The MANIFEST act on the wire: `[nuclei][nonce 32][crossing][sig 64]`
/// after the head.
pub fn encode_manifest(s: &SignedManifest) -> Option<Vec<u8>> {
    let m = &s.manifest;
    let mut out = head(&m.domain, KIND_MANIFEST);
    put_manifest_body(&mut out, s)?;
    Some(out)
}

/// The manifest's body after its sub-kind, as the wire and the held grant
/// write it.
pub(crate) fn put_manifest_body(out: &mut Vec<u8>, s: &SignedManifest) -> Option<()> {
    put_nuclei(out, &s.manifest.nuclei)?;
    out.extend_from_slice(&s.manifest.nonce);
    s.crossing.put(out)?;
    out.extend_from_slice(&s.sig);
    Some(())
}

/// The MANIFEST's body after its sub-kind, read whole.
pub(crate) fn take_manifest(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<SignedManifest> {
    let nuclei = take_nuclei(b, p)?;
    let nonce = key32(b, p)?;
    let crossing = Crossing::take(b, p).ok()?;
    let sig: [u8; 64] = b.get(*p..*p + 64)?.try_into().ok()?;
    *p += 64;
    Some(SignedManifest { manifest: Manifest { domain, nuclei, nonce }, crossing, sig })
}

/// The READ-ENDOWMENT act on the wire.
pub fn encode_endowment_read(domain: [u8; 32]) -> Vec<u8> {
    head(&domain, KIND_READ_ENDOWMENT)
}

/// WHY A MANIFEST WAS NOT GRANTED, by name — nothing moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrantRefusal {
    /// No such domain is founded here.
    NotFounded,
    /// The domain has no origin: no universe to sponsor.
    NotAUniverse,
    /// The signature is no key's of the domain. (A domain with no sitting
    /// reserve, or a manifest another key of the domain signed, is the
    /// boundary's refusal, `Granted::Boundary`, track 24 B4.)
    Unsigned,
    /// The nuclei are empty, unordered, or a count is not a positive integer.
    Malformed,
    /// This manifest was granted already.
    Granted,
    /// 78.184 — the domain's kernel converses on none of the register's
    /// clauses (or its alignment does not read), so nothing is created for
    /// it, and no growth part stands beside.
    Unaligned,
    /// The domain's last grant is not yet applied in its ledger: one in flight.
    InFlight,
    /// What the court has earned does not cover the grant on a lane.
    Short,
    /// A price or a sum passes its carrier.
    Overflow,
    /// The court's line could not take the act.
    LineRefused,
    /// 78.155c — a growth grant with no closure judged in the universe since
    /// its previous grant: the endowment grows only with its mathematics.
    NoNewClosures,
}

impl GrantRefusal {
    pub fn byte(self) -> u8 {
        match self {
            GrantRefusal::NotFounded => 1,
            GrantRefusal::NotAUniverse => 2,
            GrantRefusal::Unsigned => 4,
            GrantRefusal::Malformed => 5,
            GrantRefusal::Granted => 6,
            GrantRefusal::InFlight => 7,
            GrantRefusal::Short => 8,
            GrantRefusal::Overflow => 9,
            GrantRefusal::LineRefused => 10,
            GrantRefusal::NoNewClosures => 11,
            GrantRefusal::Unaligned => 12,
        }
    }

    pub fn of(b: u8) -> Option<GrantRefusal> {
        [
            GrantRefusal::NotFounded,
            GrantRefusal::NotAUniverse,
            GrantRefusal::Unsigned,
            GrantRefusal::Malformed,
            GrantRefusal::Granted,
            GrantRefusal::InFlight,
            GrantRefusal::Short,
            GrantRefusal::Overflow,
            GrantRefusal::LineRefused,
            GrantRefusal::NoNewClosures,
            GrantRefusal::Unaligned,
        ]
        .into_iter()
        .find(|r| r.byte() == b)
    }
}

/// A MANIFEST ANSWERED: granted — the μ per lane moved to the reserve,
/// whether it was the founding grant, and the act's stamp and chain hash —
/// or refused by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Granted {
    Sealed { mu: Mu, founding: bool, at: Stamp, receipt: [u8; 32] },
    Refused(GrantRefusal),
    /// Track 24 B4 — refused at the universal core tube, by name.
    Boundary(crate::court::admission::Boundary),
}

/// `[0][μ⃗][founding u8][stamp][receipt 32]`, `[1][refusal u8]` or
/// `[2][boundary]`, the μ⃗ four exact rational lanes (`Mu::put`).
pub fn encode_grant_reply(g: &Granted) -> Vec<u8> {
    match g {
        Granted::Sealed { mu, founding, at, receipt } => {
            let mut out = vec![0u8];
            mu.put(&mut out);
            out.push(u8::from(*founding));
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out
        }
        Granted::Refused(r) => vec![1, r.byte()],
        Granted::Boundary(why) => {
            let mut out = vec![2];
            why.put(&mut out);
            out
        }
    }
}

pub fn decode_grant_reply(b: &[u8]) -> Option<Granted> {
    let mut p = 0usize;
    match byte(b, &mut p)? {
        0 => {
            let mu = Mu::take_from(b, &mut p)?;
            let founding = match byte(b, &mut p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt = key32(b, &mut p)?;
            (p == b.len()).then_some(Granted::Sealed { mu, founding, at, receipt })
        }
        1 => {
            let r = GrantRefusal::of(byte(b, &mut p)?)?;
            (p == b.len()).then_some(Granted::Refused(r))
        }
        2 => {
            let why = crate::court::admission::Boundary::take(b, &mut p)?;
            (p == b.len()).then_some(Granted::Boundary(why))
        }
        _ => None,
    }
}

/// ONE GRANT, AS READ: the manifest it answered (by hash), the μ moved,
/// founding or growth, and whether the universe's ledger has applied it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrantRead {
    pub manifest: [u8; 32],
    pub mu: Mu,
    pub founding: bool,
    pub applied: bool,
}

/// WHAT THE COURT CAN GRANT AND HAS GRANTED: its earned μ per lane (the
/// endowment), the founding and growth grants made to every universe per
/// lane, this domain's grants, and the nuclei this universe holds by its
/// grants — the hypha's account is the negative of every universe's. Every
/// amount is exact (`Mu`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EndowmentReading {
    pub earned: Mu,
    pub founding: Mu,
    pub growth: Mu,
    pub grants: Vec<GrantRead>,
    pub nuclei: Nuclei,
}

/// `[earned μ⃗][founding μ⃗][growth μ⃗][n u16]([manifest 32][μ⃗][founding u8][applied u8])…[nuclei]`,
/// each μ⃗ four exact rational lanes (`Mu::put`).
pub fn encode_endowment_reading(r: &EndowmentReading) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    r.earned.put(&mut out);
    r.founding.put(&mut out);
    r.growth.put(&mut out);
    out.extend_from_slice(&u16::try_from(r.grants.len()).ok()?.to_le_bytes());
    for g in &r.grants {
        out.extend_from_slice(&g.manifest);
        g.mu.put(&mut out);
        out.push(u8::from(g.founding));
        out.push(u8::from(g.applied));
    }
    if r.nuclei.is_empty() {
        out.extend_from_slice(&0u16.to_le_bytes());
    } else {
        put_nuclei(&mut out, &r.nuclei)?;
    }
    Some(out)
}

pub fn decode_endowment_reading(b: &[u8]) -> Option<EndowmentReading> {
    let mut p = 0usize;
    let (earned, founding, growth) = (Mu::take_from(b, &mut p)?, Mu::take_from(b, &mut p)?, Mu::take_from(b, &mut p)?);
    let n = usize::from(u16::from_le_bytes(b.get(p..p + 2)?.try_into().ok()?));
    p += 2;
    let flag = |b: &[u8], p: &mut usize| -> Option<bool> {
        match byte(b, p)? {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    };
    let grants = (0..n)
        .map(|_| {
            let manifest = key32(b, &mut p)?;
            let mu = Mu::take_from(b, &mut p)?;
            let founding = flag(b, &mut p)?;
            let applied = flag(b, &mut p)?;
            Some(GrantRead { manifest, mu, founding, applied })
        })
        .collect::<Option<Vec<_>>>()?;
    let nuclei = take_nuclei(b, &mut p)?;
    (p == b.len()).then_some(EndowmentReading { earned, founding, growth, grants, nuclei })
}
