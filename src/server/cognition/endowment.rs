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
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_count, take_key_count};

/// ONE GRANT THIS COURT MADE: the manifest it answered (signed, whole), its
/// hash, the μ moved, founding or growth, and the act's stamp and chain hash.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct HeldGrant {
    pub(crate) manifest: SignedManifest,
    pub(crate) hash: [u8; 32],
    pub(crate) mu: [u64; 4],
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
        for x in self.mu {
            out.extend_from_slice(&x.to_le_bytes());
        }
        out.push(u8::from(self.founding));
        self.at.put_canonical(out);
        out.extend_from_slice(&self.receipt);
        Some(())
    }

    fn take(domain: [u8; 32], b: &[u8], p: &mut usize) -> Option<HeldGrant> {
        let manifest = take_manifest(domain, b, p)?;
        let hash = manifest.manifest.hash()?;
        let mut mu = [0u64; 4];
        for x in &mut mu {
            *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
        }
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
/// `[domains][(domain 32, lanes 4 × u64) …]`, in domain order.
pub(crate) fn put_closed(closed: &Keyed<[u64; 4]>) -> Vec<u8> {
    let mut v = Vec::new();
    put_key_count(&mut v, closed.len());
    for (domain, lanes) in closed.scoped() {
        v.extend_from_slice(&domain);
        for x in lanes {
            v.extend_from_slice(&x.to_le_bytes());
        }
    }
    v
}

pub(crate) fn take_closed(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], [u64; 4])>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let mut lanes = [0u64; 4];
            for x in &mut lanes {
                *x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
                *p += 8;
            }
            Some((domain, lanes))
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
    fn grant_applied(&self, domain: &[u8; 32], hash: &[u8; 32]) -> bool {
        self.atlas.ledgers.get(domain).is_some_and(|chain| chain.iter().any(|e| e.entry.applied.iter().any(|(h, _)| h == hash)))
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
        let Some(reserve) = self.reserve_of(&m.domain) else { return refuse(GrantRefusal::NotReserve) };
        let Some(bytes) = m.signed_bytes() else { return refuse(GrantRefusal::Malformed) };
        if !crate::court::identity::verify(&reserve, &bytes, &s.sig) {
            return refuse(GrantRefusal::Unsigned);
        }
        if !nuclei_hold(&m.nuclei) {
            return refuse(GrantRefusal::Malformed);
        }
        let Some(hash) = m.hash() else { return refuse(GrantRefusal::Malformed) };
        let held: Vec<HeldGrant> = match self.grants.get(&m.domain) {
            Some(g) => g.clone(),
            None => Vec::new(),
        };
        if held.iter().any(|g| g.hash == hash) {
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
            // per kind of nuclei the manifest counts.
            true => {
                let Some(price) = self.read_price() else { return refuse(GrantRefusal::Overflow) };
                let Ok(kinds) = u64::try_from(m.nuclei.len()) else { return refuse(GrantRefusal::Overflow) };
                let mut mu = [0u64; 4];
                for (x, p) in mu.iter_mut().zip(price) {
                    let Some(v) = p.checked_mul(kinds) else { return refuse(GrantRefusal::Overflow) };
                    *x = v;
                }
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
        if !crate::economy::lanes_short(&crate::economy::shortfall(&mu, &earned)).is_empty() {
            return refuse(GrantRefusal::Short);
        }
        let cite = Cite::Stated { domain: m.domain, kind: KIND_MANIFEST, body: hash };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], reserve, |_| vec![cite.clone()]) else { return refuse(GrantRefusal::LineRefused) };
        if self.treasury.pay(self.identity, reserve, mu, (r.stamp, r.chain_hash)).is_err() {
            return refuse(GrantRefusal::Short);
        }
        let mut now = held;
        now.push(HeldGrant { manifest: s.clone(), hash, mu, founding, at: r.stamp, receipt: r.chain_hash });
        self.grants.insert(m.domain, now);
        // The closures a grant answered are its own: the next growth grant
        // counts only those judged after it.
        self.closed_since_grant.remove(&m.domain);
        self.mark_persist();
        Granted::Sealed { mu, founding, at: r.stamp, receipt: r.chain_hash }
    }

    /// 78.155c — WHAT `domain`'S NEXT GROWTH GRANT MOVES: the measured
    /// discharge of the closures this court judged in that universe since its
    /// previous grant, exact per lane; refused by name where none was judged.
    pub fn growth_size(&self, domain: &[u8; 32]) -> Result<[u64; 4], GrantRefusal> {
        match self.closed_since_grant.get(domain).copied() {
            Some(c) if c.iter().any(|&x| x > 0) => Ok(c),
            _ => Err(GrantRefusal::NoNewClosures),
        }
    }

    /// 78.155b — WHAT THIS COURT CAN GRANT AND HAS GRANTED, read: its earned
    /// balance; the founding and growth grants made to every universe, per
    /// lane; `domain`'s grants with whether its ledger applied each; and the
    /// nuclei that universe holds by its grants. Sealing nothing. `None`
    /// where a total passes its carrier: refused whole, never capped.
    pub fn endowment_reading(&self, domain: &[u8; 32]) -> Option<EndowmentReading> {
        let mut r = EndowmentReading { earned: self.treasury.balance_mu(&self.identity), ..EndowmentReading::default() };
        for (_, held) in self.grants.scoped() {
            for g in held {
                let total = match g.founding {
                    true => &mut r.founding,
                    false => &mut r.growth,
                };
                *total = crate::core::lanes::lanes_add(*total, g.mu)?;
            }
        }
        if let Some(held) = self.grants.get(domain) {
            for g in held {
                r.grants.push(GrantRead { manifest: g.hash, mu: g.mu, founding: g.founding, applied: self.grant_applied(domain, &g.hash) });
                r.nuclei = add_nuclei(&r.nuclei, &g.manifest.manifest.nuclei);
            }
        }
        Some(r)
    }
}
