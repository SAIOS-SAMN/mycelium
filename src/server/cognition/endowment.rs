// 78.155b — THE HYPHA SPONSORS A UNIVERSE'S PACKAGE (ruled
// 2026-10-14: genesis stays earned-only; in the development sandbox the
// hypha's reserve sponsors new universes' founding packages until a market
// exists where μ is bought, staked and yields).
//
//! THE ENDOWMENT IS WHAT THIS COURT HAS EARNED — its own balance of fees and
//! tolls — and nothing is set aside for it by a chosen sum: a grant draws on
//! it at the act, refused by name where it falls short (`GrantRefusal::Short`),
//! never minted. The FOUNDING and GROWTH endowments are what was granted from
//! it, read off the grants (law 12).
//!
//! A GRANT answers a universe's MANIFEST: the first after its origin is its
//! FOUNDING grant, each later one a GROWTH grant, and a growth grant waits
//! until the universe's own ledger has applied the grant before it (one in
//! flight). What a grant moves is read off the field, never a constant. THE
//! FOUNDING GRANT is the physics: a manifest carrying `n` kinds of nuclei is
//! granted the court's READ PRICE for `n` units of gas on every lane — enough
//! to stake for its first crossing's coordinates. A GROWTH GRANT is the
//! mathematics (78.155c, ruled 2026-10-14): the measured discharge of
//! the closures the court judged in that universe since its previous grant
//! (`closed_since_grant`); none judged, no growth grant. Either is a transfer
//! from this court to the reserve, sealed as an act citing the manifest by
//! its hash.
//!
//! THE ELEMENT ACCOUNT is read off the grants too: a universe holds the
//! nuclei its granted manifests carry; the hypha's account is the negative
//! of every universe's, so the network's total is zero by construction — no
//! stock is declared anywhere.

use super::Court;
use crate::anastome::dialogue::endowment::{nuclei_hold, put_nuclei, take_manifest, EndowmentReading, GrantRead, GrantRefusal, Granted, Nuclei, SignedManifest, KIND_MANIFEST};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::court::admission::Locator;
use crate::economy::Mu;
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_count, take_key_count};

/// ONE GRANT THIS COURT MADE: the manifest it answered (signed, whole), its
/// hash, the μ moved, founding or growth, and the act's stamp and chain hash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HeldGrant {
    pub(crate) manifest: SignedManifest,
    pub(crate) hash: [u8; 32],
    pub(crate) mu: Mu,
    pub(crate) founding: bool,
    pub(crate) at: Stamp,
    pub(crate) receipt: [u8; 32],
}

impl HeldGrant {
    fn put(&self, out: &mut Vec<u8>) -> Option<()> {
        let m = &self.manifest.manifest;
        put_nuclei(out, &m.nuclei)?;
        out.extend_from_slice(&m.nonce);
        out.extend_from_slice(&self.manifest.sig);
        self.mu.put(out);
        out.push(u8::from(self.founding));
        self.at.put_canonical(out);
        out.extend_from_slice(&self.receipt);
        Some(())
    }

    fn take(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<HeldGrant> {
        let manifest = take_manifest(domain, b, p)?;
        let hash = manifest.manifest.hash()?;
        let mu = Mu::take_from(b, p)?;
        let founding = match *b.get(*p)? {
            0 => false,
            1 => true,
            _ => return None,
        };
        *p += 1;
        let at = Stamp::take_canonical(b, p)?;
        let receipt: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(HeldGrant { manifest, hash, mu, founding, at, receipt })
    }
}

/// The grants written: `[domains][(domain 32, [grants][grant …]) …]`. `None`
/// where a count passes its carrier.
pub(crate) fn put_grants(grants: &Keyed<Vec<HeldGrant>>) -> Option<Vec<u8>> {
    let mut v = Vec::new();
    put_key_count(&mut v, grants.len());
    for (domain, held) in grants.scoped() {
        v.extend_from_slice(&domain);
        put_key_count(&mut v, held.len());
        for g in held {
            g.put(&mut v)?;
        }
    }
    Some(v)
}

pub(crate) fn take_grants(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], Vec<HeldGrant>)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let held = (0..take_key_count(b, p)?).map(|_| HeldGrant::take(domain, b, p)).collect::<Option<Vec<_>>>()?;
            Some((domain, held))
        })
        .collect()
}

/// 78.155c — the closures awaiting a growth grant written:
/// `[domains][(domain 32, μ⃗) …]`, in domain order, each μ⃗ four exact
/// rational lanes (`Mu::put`).
pub(crate) fn put_closed(closed: &Keyed<Mu>) -> Vec<u8> {
    let mut v = Vec::new();
    put_key_count(&mut v, closed.len());
    for (domain, mu) in closed.scoped() {
        v.extend_from_slice(&domain);
        mu.put(&mut v);
    }
    v
}

pub(crate) fn take_closed(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], Mu)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some((domain, Mu::take_from(b, p)?))
        })
        .collect()
}

/// Two nuclei counts summed per Z, exact, in ascending Z.
fn add_nuclei(a: &Nuclei, b: &Nuclei) -> Nuclei {
    let mut sum: Keyed<num_bigint::BigUint> = Keyed::new();
    for (z, c) in a.iter().chain(b) {
        let key = z.to_be_bytes();
        let now = match sum.get(&key) {
            Some(held) => held + c,
            None => c.clone(),
        };
        sum.insert(key, now);
    }
    sum.scoped().filter_map(|(k, c)| Some((u16::from_be_bytes(k.as_slice().try_into().ok()?), c.clone()))).collect()
}

impl Court {
    /// Whether `domain`'s own ledger has applied the grant at `hash`: an
    /// entry citing it among what it applied (as a crossing is applied).
    /// The grant's hash locates it (an address, MU-HOSTING §5a.10).
    fn grant_applied(&self, domain: &[u8; 32], hash: &[u8; 32]) -> bool {
        let grant = Locator::of_bytes(*hash);
        self.atlas
            .ledgers
            .get(domain)
            .is_some_and(|chain| chain.iter().any(|e| e.entry.applied.iter().any(|(h, _)| Locator::of_bytes(*h).locates(&grant))))
    }

    /// 78.155b — A MANIFEST GRANTED, or refused by name: the domain founded
    /// and originated; signed by its sitting reserve; its nuclei well formed;
    /// not granted before; a growth grant only once the last is applied in
    /// the universe's ledger. Its grant — the read price for one unit of gas
    /// per lane per kind of nuclei it carries — moves from this court's
    /// earned balance to the reserve on one sealed act, whole or refused.
    pub fn manifest(&mut self, s: &SignedManifest) -> Granted {
        let refuse = Granted::Refused;
        let m = &s.manifest;
        if self.domains.get(&m.domain).is_none() {
            return refuse(GrantRefusal::NotFounded);
        }
        if self.origins.get(&m.domain).is_none() {
            return refuse(GrantRefusal::NotAUniverse);
        }
        let Some(bytes) = m.signed_bytes() else { return refuse(GrantRefusal::Malformed) };
        // Track 24 B4 — a universe's act, admitted only from its sitting
        // reserve: another key of the domain, or a domain with no reserve,
        // is no event through its vertex.
        let reserve = match self.reserve_signed(&m.domain, &bytes, &s.sig) {
            Ok(r) => r,
            Err(true) => {
                return Granted::Boundary(crate::court::admission::Boundary::NotThroughCore {
                    act: crate::court::admission::UniverseAct::Manifest,
                    domain: m.domain,
                })
            }
            Err(false) => return refuse(GrantRefusal::Unsigned),
        };
        if let Err(why) = self.signatory_now(crate::court::admission::UniverseAct::Manifest, m.domain) {
            return Granted::Boundary(why);
        }
        if !nuclei_hold(&m.nuclei) {
            return refuse(GrantRefusal::Malformed);
        }
        let Some(hash) = m.hash() else { return refuse(GrantRefusal::Malformed) };
        let held: Vec<HeldGrant> = match self.grants.get(&m.domain) {
            Some(g) => g.clone(),
            None => Vec::new(),
        };
        // A RESEND, met by what is held: the manifest, checked above, is
        // located among the grants made already by its hash (an address,
        // MU-HOSTING §5a.10), and is never admitted on it.
        if held.iter().any(|g| Locator::of_bytes(g.hash).locates(&Locator::of_bytes(hash))) {
            return refuse(GrantRefusal::Granted);
        }
        // One in flight: every grant made to this universe applied in its
        // ledger before another is made.
        if !held.iter().all(|g| self.grant_applied(&m.domain, &g.hash)) {
            return refuse(GrantRefusal::InFlight);
        }
        let founding = held.is_empty();
        let mu = match founding {
            // THE FOUNDING GRANT, THE PHYSICS (ruled 2026-10-14): the
            // variety of matter — the read price for one unit of gas per lane
            // per kind of nuclei the manifest counts, `kinds · p_l` exactly
            // (MU-DERIVATIONS D2.3).
            true => {
                let Some(price) = self.read_price() else { return refuse(GrantRefusal::Overflow) };
                let kinds = crate::core::q::Q::from_integer(m.nuclei.len().into());
                let Some(mu) = price.scaled(&kinds) else { return refuse(GrantRefusal::Overflow) };
                mu
            }
            // 78.155c — A GROWTH GRANT, THE MATHEMATICS (ruled
            // 2026-10-14): the measured discharge of the closures the court
            // judged in this universe since its previous grant — the
            // endowment grows only as its minds develop traversable
            // mathematics. None judged: no growth grant. Never estimated
            // ahead of a closure.
            false => match self.growth_size(&m.domain) {
                Ok(c) => c,
                Err(why) => return refuse(why),
            },
        };
        let earned = self.treasury.balance_mu(&self.identity);
        if !mu.shortfall(&earned).is_zero() {
            return refuse(GrantRefusal::Short);
        }
        let cite = Cite::Stated { domain: m.domain, kind: KIND_MANIFEST, body: hash };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], reserve, |_| vec![cite.clone()]) else { return refuse(GrantRefusal::LineRefused) };
        if self.treasury.pay(self.identity, reserve, mu.clone(), (r.stamp, r.chain_hash.bytes())).is_err() {
            return refuse(GrantRefusal::Short);
        }
        let mut now = held;
        now.push(HeldGrant { manifest: s.clone(), hash, mu: mu.clone(), founding, at: r.stamp, receipt: r.chain_hash.bytes() });
        self.grants.insert(m.domain, now);
        // The closures a grant answered are its own: the next growth grant
        // counts only those judged after it.
        self.closed_since_grant.remove(&m.domain);
        self.mark_persist();
        Granted::Sealed { mu, founding, at: r.stamp, receipt: r.chain_hash.bytes() }
    }

    /// 78.155c — WHAT `domain`'S NEXT GROWTH GRANT MOVES: the measured
    /// discharge of the closures this court judged in that universe since its
    /// previous grant, exact per lane; refused by name where none was judged.
    pub fn growth_size(&self, domain: &[u8; 32]) -> Result<Mu, GrantRefusal> {
        match self.closed_since_grant.get(domain) {
            Some(c) if !c.is_zero() => Ok(c.clone()),
            _ => Err(GrantRefusal::NoNewClosures),
        }
    }

    /// 78.155b — WHAT THIS COURT CAN GRANT AND HAS GRANTED, read: its earned
    /// balance; the founding and growth grants made to every universe, per
    /// lane; `domain`'s grants with whether its ledger applied each; and the
    /// nuclei that universe holds by its grants. Sealing nothing. The totals
    /// are exact.
    pub fn endowment_reading(&self, domain: &[u8; 32]) -> Option<EndowmentReading> {
        let mut r = EndowmentReading { earned: self.treasury.balance_mu(&self.identity), ..EndowmentReading::default() };
        for (_, held) in self.grants.scoped() {
            for g in held {
                match g.founding {
                    true => r.founding.step(&g.mu),
                    false => r.growth.step(&g.mu),
                }
            }
        }
        if let Some(held) = self.grants.get(domain) {
            for g in held {
                r.grants.push(GrantRead { manifest: g.hash, mu: g.mu.clone(), founding: g.founding, applied: self.grant_applied(domain, &g.hash) });
                r.nuclei = add_nuclei(&r.nuclei, &g.manifest.manifest.nuclei);
            }
        }
        Some(r)
    }
}
