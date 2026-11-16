// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.184 §6 (ruled 2026-11-16) — μ FOLLOWS ITS NUCLEI ACROSS A TUBE.
//
//! The transport is multi-axial: a crossing has no direction, and each
//! universe at the tube signs the nuclei it releases and the nuclei it
//! takes in. The court applies the tube's image at the vertex, so neither
//! side states it, and conserves NUCLEONS, never kinds, since kinds fuse and
//! burn. Dissipation is admitted.
//!
//! ```text
//! account(d)  = Σ_grants A  + Σ A taken in  − Σ A released          A = Z + Q, in nucleons
//! carried_l   = Σ_released n · row(Z, Q)_l · share(d)               the μ held on what d releases
//! to taker t  = carried · A_taken(t) / A_released                   delivered share now, owed share as a carried allotment
//! to the pool = carried · (A_released − A_taken) / A_released       the connection tax (Concordance 5.8)
//! ```
//!
//! The delivered share `d` of the releaser's founding moves at once, reserve
//! to reserve; the owed share `1 − d` moves as a carried allotment, delivered
//! along the same T lane from the same `T₀` (`deliver_at`), so nothing is
//! delivered twice and nothing is lost to the crossing.

use super::Court;
use crate::anastome::dialogue::crossing::{CrossRefusal, Motion, Side};
use crate::anastome::dialogue::endowment::Nuclei;
use crate::core::q::Q;
use crate::economy::Mu;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// ONE CARRIED ALLOTMENT: the owed share of μ that crossed with its nuclei,
/// delivered to `to` (a sitting reserve, or the court itself for the pool)
/// along the T lane from the releaser's `T₀`, `delivered` the share already
/// delivered when it crossed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Carried {
    pub(crate) to_pool: bool,
    pub(crate) crossing: [u8; 32],
    pub(crate) allotment: Mu,
    pub(crate) t0: Option<Q>,
    pub(crate) delivered: Q,
}

/// `a − b` per lane, a lane past empty standing at zero: what a founding
/// keeps once a crossing carried `b` off it.
fn less(a: &Mu, b: &Mu) -> Mu {
    match a.draw(&b.capped(a)) {
        Ok(rest) => rest,
        Err(_) => Mu::zero(),
    }
}

/// The nucleons `Σ n · A` of `nuclei`, exact.
pub(crate) fn nucleons_of(nuclei: &Nuclei) -> BigUint {
    nuclei.iter().fold(BigUint::zero(), |s, (k, n)| s + n * BigUint::from(k.nucleons()))
}

impl Court {
    /// 78.184 §6 — A UNIVERSE'S ELEMENT ACCOUNT, in nucleons: every grant's
    /// nuclei, plus what it took in across tubes, less what it released.
    /// `None` where the flows read past the account (never on a court that
    /// admitted them).
    pub(crate) fn element_account(&self, domain: &[u8; 32]) -> Option<BigUint> {
        let granted = match self.grants.get(domain) {
            Some(held) => held.iter().fold(BigUint::zero(), |s, g| s + nucleons_of(&g.manifest.manifest.nuclei)),
            None => BigUint::zero(),
        };
        let (taken, released) = match self.nucleon_flow.get(domain) {
            Some((t, r)) => (t.clone(), r.clone()),
            None => (BigUint::zero(), BigUint::zero()),
        };
        let held = granted + taken;
        (held >= released).then(|| held - released)
    }

    /// The μ held on `nuclei` released by `domain`: `Σ n · row · share`.
    fn carried_of(&mut self, domain: &[u8; 32], nuclei: &Nuclei) -> Option<Mu> {
        let share = self.alignment_share(domain)?;
        let created = self.elemental_creation(nuclei).ok()?;
        Some(Mu::of_modes(created.scaled(&share)?))
    }

    /// THE RELEASER'S DELIVERY: its founding's `T₀` and the share delivered.
    fn delivery_of(&self, domain: &[u8; 32]) -> (Option<Q>, Q) {
        match self.grants.get(domain).and_then(|g| g.iter().find(|g| g.founding)) {
            Some(g) => (g.t0.clone(), g.delivered.clone()),
            None => (None, Q::zero()),
        }
    }

    /// 78.184 §6 — THE MOTIONS CHECKED, at the tube between `a` and `b`, signed
    /// by `crossing`'s motion bytes: each motion a universe of the tube, at
    /// most one each, signed by its sitting reserve; every entry at rest
    /// (the image of a rest entry is at rest, so the taker's letters check
    /// as stated); no release past its account; nucleons taken at most those
    /// released. Answers the nucleons released and taken.
    pub(crate) fn check_motions(&self, t: &crate::anastome::dialogue::crossing::Traverse, a: [u8; 32], b: [u8; 32]) -> Result<(BigUint, BigUint), CrossRefusal> {
        let side = |d: &[u8; 32]| if *d == a { Side::A } else { Side::B };
        let mut seen: Vec<[u8; 32]> = Vec::new();
        let (mut released, mut taken) = (BigUint::zero(), BigUint::zero());
        for m in &t.motions {
            if (m.domain != a && m.domain != b) || seen.contains(&m.domain) {
                return Err(CrossRefusal::NotAtTube);
            }
            seen.push(m.domain);
            let who = side(&m.domain);
            let Some(bytes) = t.motion_bytes(&m.domain, &m.released, &m.taken) else { return Err(CrossRefusal::Overflow) };
            if !self.reserve_of(&m.domain).is_some_and(|r| crate::court::identity::verify(&r, &bytes, &m.sig)) {
                return Err(CrossRefusal::MotionUnsigned(who));
            }
            if !m.released.iter().chain(&m.taken).all(|(n, _)| n.at_rest()) {
                return Err(CrossRefusal::NotAtRest(who));
            }
            let out = nucleons_of(&m.released);
            if self.element_account(&m.domain).is_none_or(|held| out > held) {
                return Err(CrossRefusal::PastAccount(who));
            }
            released += out;
            taken += nucleons_of(&m.taken);
        }
        if taken > released {
            return Err(CrossRefusal::NotConserved);
        }
        Ok((released, taken))
    }

    /// 78.184 §6 — THE μ MOVED WITH THE NUCLEI, the crossing sealed at
    /// `anchor`. Each releaser's carried μ is divided by nucleons: the
    /// takers' parts and the pool's. The delivered share moves now (reserve
    /// to reserve, or into the recycled pool); the owed share moves as a
    /// carried allotment. Refused `Uncovered` before anything moves where a
    /// releaser's reserve cannot cover its delivered share.
    pub(crate) fn carry_motions(&mut self, motions: &[Motion], a: [u8; 32], crossing: [u8; 32], released: &BigUint, anchor: crate::economy::Anchor) -> Result<(), CrossRefusal> {
        if released.is_zero() {
            return Ok(());
        }
        let total = Q::from_integer(released.clone().into());
        let takers: Vec<([u8; 32], Q)> = motions.iter().map(|m| (m.domain, Q::from_integer(nucleons_of(&m.taken).into()) / &total)).collect();
        let kept = takers.iter().fold(Q::zero(), |s, (_, f)| s + f);
        let lost = Q::one() - kept;
        // Every releaser's part read first, so nothing moves on a refusal.
        let mut parts = Vec::new();
        for m in motions.iter().filter(|m| !m.released.is_empty()) {
            let who = if m.domain == a { Side::A } else { Side::B };
            let Some(carried) = self.carried_of(&m.domain, &m.released) else { return Err(CrossRefusal::Overflow) };
            let (t0, d) = self.delivery_of(&m.domain);
            let Some(now) = carried.scaled(&d) else { return Err(CrossRefusal::Overflow) };
            let Some(reserve) = self.reserve_of(&m.domain) else { return Err(CrossRefusal::Uncovered(who)) };
            if !now.shortfall(&self.treasury.balance_mu(&reserve)).is_zero() {
                return Err(CrossRefusal::Uncovered(who));
            }
            parts.push((m.domain, reserve, carried, now, t0, d));
        }
        for (domain, reserve, carried, now, t0, d) in parts {
            // The releaser's founding gives up what crossed: its allotment
            // by the whole carried μ, its delivered μ by the share moved.
            if let Some(mut held) = self.grants.get(&domain).cloned() {
                if let Some(g) = held.iter_mut().find(|g| g.founding) {
                    g.allotment = less(&g.allotment, &carried);
                    g.mu = less(&g.mu, &now);
                }
                self.grants.insert(domain, held);
            }
            let owed = less(&carried, &now);
            for (to, f) in takers.iter().filter(|(_, f)| !f.is_zero()) {
                let Some(to_reserve) = self.reserve_of(to) else { continue };
                if let Some(paid) = now.scaled(f) {
                    let _ = self.treasury.pay(reserve, to_reserve, paid, anchor);
                }
                if let Some(allot) = owed.scaled(f) {
                    self.push_carried(*to, Carried { to_pool: false, crossing, allotment: allot, t0: t0.clone(), delivered: d.clone() });
                }
            }
            if !lost.is_zero() {
                if let Some(taxed) = now.scaled(&lost) {
                    self.pay_connection_tax(reserve, taxed, anchor);
                }
                if let Some(allot) = owed.scaled(&lost) {
                    let court = self.identity;
                    self.push_carried(court, Carried { to_pool: true, crossing, allotment: allot, t0: t0.clone(), delivered: d.clone() });
                }
            }
        }
        for m in motions {
            let mut flow = match self.nucleon_flow.get(&m.domain) {
                Some(f) => f.clone(),
                None => (BigUint::zero(), BigUint::zero()),
            };
            flow.0 += nucleons_of(&m.taken);
            flow.1 += nucleons_of(&m.released);
            self.nucleon_flow.insert(m.domain, flow);
        }
        self.mark_persist();
        Ok(())
    }

    fn push_carried(&mut self, to: [u8; 32], c: Carried) {
        let mut held = match self.carried.get(&to) {
            Some(v) => v.clone(),
            None => Vec::new(),
        };
        held.push(c);
        self.carried.insert(to, held);
    }
}

fn put_q(out: &mut Vec<u8>, q: &Q) {
    crate::transducer::cga::Mv::scalar(q.clone()).to_bytes(out);
}

fn take_q(b: &[u8], p: &mut usize) -> Option<Q> {
    Some(crate::transducer::cga::Mv::from_bytes(b, p)?.scalar_part())
}

fn put_count(out: &mut Vec<u8>, n: &BigUint) {
    put_q(out, &Q::from_integer(n.clone().into()));
}

fn take_count(b: &[u8], p: &mut usize) -> Option<BigUint> {
    let q = take_q(b, p)?;
    match q.is_integer() {
        true => q.to_integer().to_biguint(),
        false => None,
    }
}

/// 78.184 §6 — the elements written: `[parties][(party 32, [n][carried …])
/// …][domains][(domain 32, taken, released) …]`.
pub(crate) fn put_elements(carried: &crate::network::nerve::Keyed<Vec<Carried>>, flow: &crate::network::nerve::Keyed<(BigUint, BigUint)>) -> Vec<u8> {
    use crate::transducer::address::put_key_count;
    let mut v = Vec::new();
    put_key_count(&mut v, carried.len());
    for (party, held) in carried.scoped() {
        v.extend_from_slice(&party);
        put_key_count(&mut v, held.len());
        for c in held {
            v.push(u8::from(c.to_pool));
            v.extend_from_slice(&c.crossing);
            c.allotment.put(&mut v);
            match &c.t0 {
                None => v.push(0),
                Some(t) => {
                    v.push(1);
                    put_q(&mut v, t);
                }
            }
            put_q(&mut v, &c.delivered);
        }
    }
    put_key_count(&mut v, flow.len());
    for (domain, (taken, released)) in flow.scoped() {
        v.extend_from_slice(&domain);
        put_count(&mut v, taken);
        put_count(&mut v, released);
    }
    v
}

/// The elements read whole (`put_elements`).
#[expect(clippy::type_complexity, reason = "the leaf's two keyed parts, read in the order written")]
pub(crate) fn take_elements(b: &[u8], p: &mut usize) -> Option<(Vec<([u8; 32], Vec<Carried>)>, Vec<([u8; 32], (BigUint, BigUint))>)> {
    use crate::transducer::address::take_key_count;
    let key = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
        let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(k)
    };
    let carried = (0..take_key_count(b, p)?)
        .map(|_| {
            let party = key(b, p)?;
            let held = (0..take_key_count(b, p)?)
                .map(|_| {
                    let to_pool = match *b.get(*p)? {
                        0 => false,
                        1 => true,
                        _ => return None,
                    };
                    *p += 1;
                    let crossing = key(b, p)?;
                    let allotment = Mu::take_from(b, p)?;
                    let t0 = match *b.get(*p)? {
                        0 => {
                            *p += 1;
                            None
                        }
                        1 => {
                            *p += 1;
                            Some(take_q(b, p)?)
                        }
                        _ => return None,
                    };
                    let delivered = take_q(b, p)?;
                    Some(Carried { to_pool, crossing, allotment, t0, delivered })
                })
                .collect::<Option<Vec<_>>>()?;
            Some((party, held))
        })
        .collect::<Option<Vec<_>>>()?;
    let flow = (0..take_key_count(b, p)?).map(|_| Some((key(b, p)?, (take_count(b, p)?, take_count(b, p)?)))).collect::<Option<Vec<_>>>()?;
    Some((carried, flow))
}

impl Court {
    /// 78.184 §6 — THE CARRIED ALLOTMENTS' BEAT at final point `t` (the T
    /// lane): each delivers `(d − delivered) · allotment` from the
    /// releaser's `T₀`, minted to the party's sitting reserve or, for the
    /// pool, minted to this court and recycled. Answers the deliveries.
    pub(crate) fn deliver_carried(&mut self, t: &Q, anchor: crate::economy::Anchor) -> Vec<([u8; 32], Mu)> {
        let mut out = Vec::new();
        let parties: Vec<[u8; 32]> = self.carried.scoped().map(|(k, _)| k).filter_map(|k| <[u8; 32]>::try_from(k.as_slice()).ok()).collect();
        for party in parties {
            let Some(mut held) = self.carried.get(&party).cloned() else { continue };
            let mut moved = false;
            for c in held.iter_mut() {
                let Some(t0) = c.t0.clone() else {
                    c.t0 = Some(t.clone());
                    moved = true;
                    continue;
                };
                let d = Q::one() - &t0 / t;
                if d <= c.delivered || c.delivered >= Q::one() {
                    continue;
                }
                let Some(due) = c.allotment.scaled(&((&d - &c.delivered) / (Q::one() - &c.delivered))) else { continue };
                c.allotment = less(&c.allotment, &due);
                c.delivered = d;
                moved = true;
                let to = match c.to_pool {
                    true => self.identity,
                    false => match self.reserve_of(&party) {
                        Some(r) => r,
                        None => continue,
                    },
                };
                let minted = self.treasury.mint_to(to, due, anchor, None);
                if c.to_pool {
                    self.pay_connection_tax(to, minted.clone(), anchor);
                }
                out.push((party, minted));
            }
            if moved {
                self.carried.insert(party, held);
                self.mark_persist();
            }
        }
        out
    }
}
