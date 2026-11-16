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
//! MANIFEST       [ACT_DOMAIN][domain 32][0x80 0x00][17][count u32 LE]
//!                (3Z u32, 3Q u32, pattern: n u8 u32…, planes: n u8 (plane u16, K u64, phase ℚ, pole ℚ)…, count ℕ)…
//!                [nonce 32][crossing][sig 64]
//! MANIFEST-PAGE  [ACT_DOMAIN][domain 32][0x80 0x00][19][hash 32][total u32][offset u32][chunk]
//! READ-ENDOWMENT [ACT_DOMAIN][domain 32][0x80 0x00][18][offset u32 LE]
//!                answered by the page [hash 32][total u32 LE][offset u32 LE][chunk] (#332)
//! ```
//!
//! Signed bytes: `"mycelium manifest v4" ‖ domain ‖ the entries as written ‖
//! nonce`. (v1 keyed each entry by Z alone, retired 2026-11-13; v2 carried no
//! electron phase, retired 2026-11-14; v3 carried one phase share as a
//! multivector scalar and a u16 count, retired 2026-11-16.)
//!
//! 78.184 §8 (approved 2026-11-16): nucleons in whole thirds, so an opened
//! plane's `Z = F + Σℓ_k/3` is exact. The kernel's `pattern` separates
//! nuclides of equal thirds, and the court orders by it and never reads its
//! meaning. The phase state is the planes in their composition order, each
//! at rest when `R_p = −σ_p` (xylarium T24 per plane). The axis plane `0` is
//! optional, and where stated its pole is the nucleon split. A manifest
//! past one wire body is sent in pages, assembled before it is judged.

use super::atlas::{byte, head, key32};
use super::crossing_record::{crossed, Crossing};
use crate::chronometry::Stamp;
use crate::economy::Mu;
use num_bigint::BigUint;
use num_rational::BigRational as Q;

/// FORMAT: sub-kind byte of ACT_DOMAIN for a manifest.
pub const KIND_MANIFEST: u8 = 17;
/// FORMAT: sub-kind byte of ACT_DOMAIN for an endowment read.
pub const KIND_READ_ENDOWMENT: u8 = 18;
const MANIFEST: &[u8] = b"mycelium manifest v4";

/// 78.184 §8 — ONE TURNING PLANE of a nucleus's phase state: the kernel's
/// plane id (`0` the axis plane), its winding `K`, its fibre phase `R_p` and
/// its state pole `σ_p`, each exact.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Plane {
    pub plane: u16,
    pub k: u64,
    pub phase: Q,
    pub pole: Q,
}

/// ONE NUCLEUS (78.184 §3, §5, §8): its protons and neutrons in whole thirds
/// (`3Z`, `3Q`), the kernel's `pattern` counts that separate nuclides of
/// equal thirds, and its phase state, the planes in their composition order
/// (none: the identity, left to the kernel's physics). Ordered by that key.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nucleus {
    pub z3: u32,
    pub q3: u32,
    pub pattern: Vec<u32>,
    pub planes: Vec<Plane>,
}

impl Nucleus {
    /// A nucleus of whole `z` protons and `q` neutrons whose capture is left
    /// to its kernel's physics.
    pub fn of(z: u16, q: u32) -> Nucleus {
        Nucleus { z3: 3 * u32::from(z), q3: q.saturating_mul(3), pattern: Vec::new(), planes: Vec::new() }
    }

    /// The protons, exact: `3Z / 3`.
    pub fn z(&self) -> Q {
        Q::new(self.z3.into(), 3.into())
    }

    /// The neutrons, exact: `3Q / 3`.
    pub fn q(&self) -> Q {
        Q::new(self.q3.into(), 3.into())
    }

    /// 78.184 §6, §8 — THE NUCLEONS a nucleus holds, in thirds, `3A = 3Z +
    /// 3Q`: what a crossing conserves and an element account counts,
    /// whatever the kind.
    pub fn nucleons3(&self) -> u64 {
        u64::from(self.z3) + u64::from(self.q3)
    }

    /// 78.184 §7–§8 — THE TUBE'S IMAGE: the pole swap `(3Z, 3Q) ↦ (3Q, 3Z)`,
    /// and the phase state reversed (the versor's reverse): the planes in the
    /// opposite order, each phase and pole negated, each winding kept. `3A`
    /// is kept, and the image of an entry at rest is at rest.
    pub fn image(&self) -> Nucleus {
        let planes = self.planes.iter().rev().map(|p| Plane { plane: p.plane, k: p.k, phase: -p.phase.clone(), pole: -p.pole.clone() }).collect();
        Nucleus { z3: self.q3, q3: self.z3, pattern: self.pattern.clone(), planes }
    }

    /// Whether the phase state stands at rest (78.184 §8; xylarium T24 per
    /// plane): every plane's phase at its pole's antipole, `R_p = −σ_p`, and
    /// the axis plane's pole the nucleon split `(Z − Q)/(Z + Q)`. No planes
    /// is the identity, at rest.
    pub fn at_rest(&self) -> bool {
        self.planes.iter().all(|p| {
            p.phase == -p.pole.clone()
                && (p.plane != 0
                    || match crate::economy::elemental::push(&self.z(), &self.q()) {
                        Some(split) => p.pole == split,
                        None => false,
                    })
        })
    }
}

/// NUCLEI PER NUCLEUS: ascending `(Z, Q)`, each once, every count a positive
/// integer of any size (a package is astronomical).
pub type Nuclei = Vec<(Nucleus, BigUint)>;

/// The nuclei well formed: not empty, strictly ascending by key, every
/// nucleus with a proton, every phase and pole in `[−1, 1]`, every count at
/// least one.
pub fn nuclei_hold(n: &Nuclei) -> bool {
    let zero = BigUint::from(0u8);
    let one = Q::from_integer(1.into());
    let within = |x: &Q| *x <= one && *x >= -one.clone();
    !n.is_empty()
        && n.windows(2).all(|w| matches!(w, [x, y] if x.0 < y.0))
        && n.iter().all(|(k, c)| k.z3 > 0 && k.planes.iter().all(|p| within(&p.phase) && within(&p.pole)) && *c > zero)
}

/// 78.184 §8 — an exact rational, compact: `[sign u8][numer: len u16, LE]
/// [denom: len u16, LE]`.
fn put_q(out: &mut Vec<u8>, x: &Q) -> Option<()> {
    use num_traits::Signed;
    out.push(u8::from(x.is_negative()));
    put_natural(out, &x.numer().abs().to_biguint()?)?;
    put_natural(out, &x.denom().to_biguint()?)
}

fn take_q(b: &[u8], p: &mut usize) -> Option<Q> {
    let negative = match *b.get(*p)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    *p += 1;
    let (n, d) = (take_natural(b, p)?, take_natural(b, p)?);
    if d == BigUint::from(0u8) {
        return None;
    }
    let x = Q::new(n.into(), d.into());
    // Lowest terms and a zero never signed: one writing per value.
    let written = Q::new(x.numer().clone(), x.denom().clone());
    use num_traits::Zero;
    match (negative, written.is_zero()) {
        (true, true) => None,
        (true, false) => Some(-written),
        (false, _) => Some(written),
    }
}

/// A whole number of any size: `[len u16][bytes LE]`.
fn put_natural(out: &mut Vec<u8>, n: &BigUint) -> Option<()> {
    let bytes = n.to_bytes_le();
    out.extend_from_slice(&u16::try_from(bytes.len()).ok()?.to_le_bytes());
    out.extend_from_slice(&bytes);
    Some(())
}

fn take_natural(b: &[u8], p: &mut usize) -> Option<BigUint> {
    let n = usize::from(u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?));
    *p += 2;
    let bytes = b.get(*p..*p + n)?;
    *p += n;
    // One writing per value: no trailing zero byte.
    if bytes.last() == Some(&0) {
        return None;
    }
    Some(BigUint::from_bytes_le(bytes))
}

/// 78.184 §8 — the nuclei written: `[n u32]` then each entry `[3Z u32][3Q
/// u32][pattern: n u8, u32…][planes: n u8, (plane u16, K u64, phase ℚ, pole
/// ℚ)…][count ℕ]`.
pub(crate) fn put_nuclei(out: &mut Vec<u8>, n: &Nuclei) -> Option<()> {
    out.extend_from_slice(&u32::try_from(n.len()).ok()?.to_le_bytes());
    for (k, c) in n {
        out.extend_from_slice(&k.z3.to_le_bytes());
        out.extend_from_slice(&k.q3.to_le_bytes());
        out.push(u8::try_from(k.pattern.len()).ok()?);
        for x in &k.pattern {
            out.extend_from_slice(&x.to_le_bytes());
        }
        out.push(u8::try_from(k.planes.len()).ok()?);
        for pl in &k.planes {
            out.extend_from_slice(&pl.plane.to_le_bytes());
            out.extend_from_slice(&pl.k.to_le_bytes());
            put_q(out, &pl.phase)?;
            put_q(out, &pl.pole)?;
        }
        put_natural(out, c)?;
    }
    Some(())
}

/// The nuclei read whole.
pub(crate) fn take_nuclei(b: &[u8], p: &mut usize) -> Option<Nuclei> {
    let u32_at = |b: &[u8], p: &mut usize| -> Option<u32> {
        let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
        *p += 4;
        Some(v)
    };
    let n = u32_at(b, p)?;
    (0..n)
        .map(|_| {
            let (z3, q3) = (u32_at(b, p)?, u32_at(b, p)?);
            let np = *b.get(*p)?;
            *p += 1;
            let pattern = (0..np).map(|_| u32_at(b, p)).collect::<Option<Vec<_>>>()?;
            let nl = *b.get(*p)?;
            *p += 1;
            let planes = (0..nl)
                .map(|_| {
                    let plane = u16::from_le_bytes(b.get(*p..*p + 2)?.try_into().ok()?);
                    *p += 2;
                    let k = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                    *p += 8;
                    Some(Plane { plane, k, phase: take_q(b, p)?, pole: take_q(b, p)? })
                })
                .collect::<Option<Vec<_>>>()?;
            let c = take_natural(b, p)?;
            Some((Nucleus { z3, q3, pattern, planes }, c))
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

/// The READ-ENDOWMENT act on the wire, asking its first page.
pub fn encode_endowment_read(domain: [u8; 32]) -> Vec<u8> {
    encode_endowment_read_at(domain, 0)
}

/// #332 — THE READ-ENDOWMENT act asking the page at byte `offset` of the
/// encoded reading: `[head][offset u32 LE]`.
pub fn encode_endowment_read_at(domain: [u8; 32], offset: u32) -> Vec<u8> {
    let mut out = head(&domain, KIND_READ_ENDOWMENT);
    out.extend_from_slice(&offset.to_le_bytes());
    out
}

/// #332 — ONE PAGE OF AN ENDOWMENT READING. The reading is encoded whole
/// ([`encode_endowment_reading`]) and served in pages that fit the wire's
/// one budget. Each page carries the whole reading's hash, so a reader
/// assembles only pages of one reading, and a reading that changed between
/// pages is read again from the first. A founding of many kinds makes a
/// reading past the budget: each distinct `Z/Q` is its own boost mode
/// (MU-EQUATIONS §6.13), and netstratum's reading of about 1,500 kinds was
/// 789,851 bytes against 786,432 (2026-11-14).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndowmentPage {
    pub hash: [u8; 32],
    pub total: u32,
    pub offset: u32,
    pub chunk: Vec<u8>,
}

/// The bytes one page carries: the wire's body budget less the delimiter's
/// own headroom (`DELIMITED_MAX` over `BODY_BUDGET`), which holds the
/// page's head and the door's expression.
pub const ENDOWMENT_PAGE: usize = crate::network::wire::BODY_BUDGET - u16::MAX as usize;

/// The page of `whole` at `offset`: `[hash 32][total u32 LE][offset u32 LE]
/// [chunk]`. `None` where `offset` stands past the reading or a length
/// passes its carrier.
pub fn encode_endowment_page(whole: &[u8], offset: u32) -> Option<Vec<u8>> {
    let total = u32::try_from(whole.len()).ok()?;
    let start = usize::try_from(offset).ok()?;
    if start > whole.len() {
        return None;
    }
    let end = whole.len().min(start + ENDOWMENT_PAGE);
    let mut out = crate::chronometry::payload_hash(whole).to_vec();
    out.extend_from_slice(&total.to_le_bytes());
    out.extend_from_slice(&offset.to_le_bytes());
    out.extend_from_slice(whole.get(start..end)?);
    Some(out)
}

/// The page offset a READ-ENDOWMENT act asks: its last four bytes. `None`
/// for a body shorter than an offset.
pub fn decode_domain_offset(ask: &[u8]) -> Option<u32> {
    let at = ask.len().checked_sub(4)?;
    Some(u32::from_le_bytes(ask.get(at..)?.try_into().ok()?))
}

/// A page read whole.
pub fn decode_endowment_page(b: &[u8]) -> Option<EndowmentPage> {
    let hash: [u8; 32] = b.get(0..32)?.try_into().ok()?;
    let total = u32::from_le_bytes(b.get(32..36)?.try_into().ok()?);
    let offset = u32::from_le_bytes(b.get(36..40)?.try_into().ok()?);
    Some(EndowmentPage { hash, total, offset, chunk: b.get(40..)?.to_vec() })
}

/// #332 — THE READING ASSEMBLED FROM ITS PAGES: `speak` answers the reply to
/// one page's ask. Pages are asked from offset 0 until the reading is whole;
/// every page must carry the first page's hash and continue at its offset,
/// and the whole must re-derive that hash, or the reading is read again from
/// the first page, once. `None` where it does not assemble or decode.
pub fn read_endowment(domain: [u8; 32], mut speak: impl FnMut(Vec<u8>) -> Option<Vec<u8>>) -> Option<EndowmentReading> {
    for _ in 0..2 {
        let mut whole: Vec<u8> = Vec::new();
        let mut hash: Option<[u8; 32]> = None;
        let assembled = loop {
            let offset = u32::try_from(whole.len()).ok()?;
            let page = decode_endowment_page(&speak(encode_endowment_read_at(domain, offset))?)?;
            if hash.is_some_and(|h| h != page.hash) || page.offset != offset {
                break false;
            }
            hash = Some(page.hash);
            if page.chunk.is_empty() && whole.len() < usize::try_from(page.total).ok()? {
                break false;
            }
            whole.extend_from_slice(&page.chunk);
            if whole.len() >= usize::try_from(page.total).ok()? {
                break crate::chronometry::payload_hash(&whole) == page.hash;
            }
        };
        if assembled {
            return decode_endowment_reading(&whole);
        }
    }
    None
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
    /// 78.184 §5 — an entry states a phase `R ≠ 0` off its rest point
    /// `R = −(Z − Q)/(Z + Q)` (`Nucleus::at_rest`).
    NotAtRest,
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
            GrantRefusal::NotAtRest => 13,
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
            GrantRefusal::NotAtRest,
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
