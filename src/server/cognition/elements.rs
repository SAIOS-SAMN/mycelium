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
//! account(d)  = Σ_grants 3A + Σ 3A taken in − Σ 3A released       3A = 3Z + 3Q, nucleons in thirds (78.184 §8)
//! f           = A_released(d) / account(d)                           the share of d's nuclei that leaves
//! carried     = f · (μ on d's nuclei: delivered and owed)            whatever its origin, never re-derived
//! to taker t  = carried · A_taken(t) / A_released                   delivered share now, owed share as carried allotments
//! to the pool = carried · (A_released − A_taken) / A_released       the connection tax (Concordance 5.8)
//! ```
//!
//! μ IS HELD ON THE NUCLEI, never re-derived from the holder (2026-11-16):
//! a universe's μ on nuclei is what its founding delivered plus what it
//! carried in, and what both still owe. A release carries its nucleon share
//! of all of it, so out and back returns the same μ less what each pass
//! dissipated: no traversal is a free loop. Each owed share keeps its own
//! source's `T₀` and delivered share, delivered along the same T lane
//! (`deliver_carried`), so nothing is delivered twice.

use super::Court;
use crate::anastome::dialogue::crossing::{CrossRefusal, Motion, Side};
use crate::anastome::dialogue::endowment::Nuclei;
use crate::core::q::Q;
use crate::economy::Mu;
use num_bigint::BigUint;
use num_traits::{One, Zero};

/// ONE CARRIED ALLOTMENT: μ that crossed with its nuclei, held for the party
/// it delivers to (a domain's sitting reserve, or this court for the pool):
/// `allotment` what is still owed, delivered along the T lane from its
/// source's `T₀`; `delivered` the share of the source already delivered;
/// `mu` what this record has delivered so far, held on the nuclei.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Carried {
    pub(crate) to_pool: bool,
    pub(crate) crossing: [u8; 32],
    pub(crate) allotment: Mu,
    pub(crate) t0: Option<Q>,
    pub(crate) delivered: Q,
    pub(crate) mu: Mu,
}

/// An exact scaling that passed its carrier reads as nothing moved.
trait OrZero {
    fn unwrap_or_else_zero(self) -> Mu;
}

impl OrZero for Option<Mu> {
    fn unwrap_or_else_zero(self) -> Mu {
        match self {
            Some(m) => m,
            None => Mu::zero(),
        }
    }
}

/// `a − b` per lane, a lane past empty standing at zero: what a founding
/// keeps once a crossing carried `b` off it.
fn less(a: &Mu, b: &Mu) -> Mu {
    match a.draw(&b.capped(a)) {
        Ok(rest) => rest,
        Err(_) => Mu::zero(),
    }
}

/// The nucleons of `nuclei` in thirds, `Σ n · 3A` (78.184 §8), exact.
pub(crate) fn nucleons_of(nuclei: &Nuclei) -> BigUint {
    nuclei.iter().fold(BigUint::zero(), |s, (k, n)| s + n * BigUint::from(k.nucleons3()))
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

    /// THE μ ON `domain`'s NUCLEI, by source: each founding and each carried
    /// record, with what it delivered (held by the reserve), what it still
    /// owes, its `T₀` and its delivered share.
    fn sources_of(&self, domain: &[u8; 32]) -> Vec<(Mu, Mu, Option<Q>, Q)> {
        let mut out = Vec::new();
        for g in self.grants.get(domain).into_iter().flatten().filter(|g| g.founding) {
            let owed = g.allotment.scaled(&(Q::one() - &g.delivered)).unwrap_or_else_zero();
            out.push((g.mu.clone(), owed, g.t0.clone(), g.delivered.clone()));
        }
        for c in self.carried.get(domain).into_iter().flatten() {
            out.push((c.mu.clone(), c.allotment.clone(), c.t0.clone(), c.delivered.clone()));
        }
        out
    }

    /// Every source on `domain`'s nuclei kept at `1 − f`: the share that
    /// stays when `f` of its nucleons leave.
    fn keep_sources(&mut self, domain: &[u8; 32], f: &Q) {
        let stay = Q::one() - f;
        if let Some(mut held) = self.grants.get(domain).cloned() {
            for g in held.iter_mut().filter(|g| g.founding) {
                g.allotment = g.allotment.scaled(&stay).unwrap_or_else_zero();
                g.mu = g.mu.scaled(&stay).unwrap_or_else_zero();
            }
            self.grants.insert(*domain, held);
        }
        if let Some(mut held) = self.carried.get(domain).cloned() {
            for c in held.iter_mut() {
                c.allotment = c.allotment.scaled(&stay).unwrap_or_else_zero();
                c.mu = c.mu.scaled(&stay).unwrap_or_else_zero();
            }
            self.carried.insert(*domain, held);
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
            let out = nucleons_of(&m.released);
            let Some(account) = self.element_account(&m.domain).filter(|n| !n.is_zero()) else { return Err(CrossRefusal::PastAccount(who)) };
            let f = Q::from_integer(out.into()) / Q::from_integer(account.into());
            let sources = self.sources_of(&m.domain);
            let now = sources.iter().fold(Mu::zero(), |s, (held, ..)| s.add(&held.scaled(&f).unwrap_or_else_zero()));
            let Some(reserve) = self.reserve_of(&m.domain) else { return Err(CrossRefusal::Uncovered(who)) };
            if !now.shortfall(&self.treasury.balance_mu(&reserve)).is_zero() {
                return Err(CrossRefusal::Uncovered(who));
            }
            parts.push((m.domain, reserve, f, now, sources));
        }
        for (domain, reserve, f, now, sources) in parts {
            self.keep_sources(&domain, &f);
            for (to, share) in takers.iter().filter(|(_, x)| !x.is_zero()) {
                let Some(to_reserve) = self.reserve_of(to) else { continue };
                let paid = now.scaled(share).unwrap_or_else_zero();
                let _ = self.treasury.pay(reserve, to_reserve, paid, anchor);
                for (held, owed, t0, d) in &sources {
                    let part = |x: &Mu| x.scaled(&(&f * share)).unwrap_or_else_zero();
                    self.push_carried(*to, Carried { to_pool: false, crossing, allotment: part(owed), t0: t0.clone(), delivered: d.clone(), mu: part(held) });
                }
            }
            if !lost.is_zero() {
                self.pay_connection_tax(reserve, now.scaled(&lost).unwrap_or_else_zero(), anchor);
                let court = self.identity;
                for (_, owed, t0, d) in &sources {
                    let allot = owed.scaled(&(&f * &lost)).unwrap_or_else_zero();
                    self.push_carried(court, Carried { to_pool: true, crossing, allotment: allot, t0: t0.clone(), delivered: d.clone(), mu: Mu::zero() });
                }
            }
        }
        for m in motions {
            // The crossing read on the court's own log: what each universe
            // released and took, in thirds, and its account after.
            let (out, inn) = (nucleons_of(&m.released), nucleons_of(&m.taken));
            let mut flow = match self.nucleon_flow.get(&m.domain) {
                Some(f) => f.clone(),
                None => (BigUint::zero(), BigUint::zero()),
            };
            flow.0 += &inn;
            flow.1 += &out;
            self.nucleon_flow.insert(m.domain, flow);
            let account = self.element_account(&m.domain);
            eprintln!("  court crossed {:02x}{:02x}… at {:02x}{:02x}…: released {out} taken {inn} thirds, account {account:?}", crossing[0], crossing[1], m.domain[0], m.domain[1]);
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
            c.mu.put(&mut v);
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
                    let mu = Mu::take_from(b, p)?;
                    Some(Carried { to_pool, crossing, allotment, t0, delivered, mu })
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
                if !c.to_pool {
                    c.mu = c.mu.add(&minted);
                }
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
