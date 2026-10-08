// 78.128 — A WORMHOLE BETWEEN DOMAINS: a mouth pair built by two domains'
// reserves (the ancients), the only way anything crosses between universes
// with no shared space; kept and tolled by its builders; its loops read as
// curvature (the coordinator's rulings 2026-10-07).
//
//! A MOUTH PAIR is offered by one domain's reserve tier and accepted by the
//! other's: X's black-hole mouth and Y's white-hole mouth, each placed in its
//! domain's chart by an exact versor. Open, a TRAVERSAL — a 2(b) handoff
//! citing it — carries matter or a word through it, either way, its flat toll
//! escrowed from the sending tier and paid to the KEEPER on acceptance. Either
//! reserve may CLOSE it: traversals in flight return, their tolls refunded.
//! The open mouths are edges between DOMAINS carrying each pair's chart
//! transition `to_frame · from_frame⁻¹`; a cycle of them whose holonomy is not
//! the identity is two routes disagreeing about how one universe lands in
//! another — curvature, RECORDED (served as loops), never refused.

use super::challenge::window_end;
use super::handoff::toll_key;
use super::Kernel;
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Loop, Mouth, SignedHandoff, State, TubeReading};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::transducer::address::{put_key_bytes, take_key_bytes};
use crate::transducer::cech::coefficient::{Coefficient, Moved};
use crate::transducer::cech::cover::{Gauge, Gauged};

/// ONE MOUTH PAIR HELD: the pair, where it stands (offered; `Accepted` open;
/// `Returned` refused, expired or closed), the height it opened at, and the
/// receipt that offered it.
#[derive(Clone, Debug)]
pub(crate) struct MouthHeld {
    pub(crate) mouth: Mouth,
    pub(crate) state: State,
    pub(crate) opened: Option<u64>,
    pub(crate) at: Vec<u8>,
}

impl MouthHeld {
    pub(crate) fn put(&self, v: &mut Vec<u8>) {
        // The flat writing: a tariff is stored beside it (`Handoffs::put`).
        v.extend_from_slice(&self.mouth.legacy_bytes());
        match self.state {
            State::Offered { until } => {
                v.push(0);
                v.extend_from_slice(&until.to_le_bytes());
            }
            State::Accepted => v.push(1),
            State::Returned => v.push(2),
        }
        match self.opened {
            None => v.push(0),
            Some(h) => {
                v.push(1);
                v.extend_from_slice(&h.to_le_bytes());
            }
        }
        put_key_bytes(v, &self.at);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<MouthHeld> {
        let mouth = Mouth::take(b, p)?;
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let tag = *b.get(*p)?;
        *p += 1;
        let state = match tag {
            0 => State::Offered { until: u64_at(b, p)? },
            1 => State::Accepted,
            2 => State::Returned,
            _ => return None,
        };
        let flag = *b.get(*p)?;
        *p += 1;
        let opened = match flag {
            0 => None,
            1 => Some(u64_at(b, p)?),
            _ => return None,
        };
        Some(MouthHeld { mouth, state, opened, at: take_key_bytes(b, p)? })
    }
}

/// 78.133 — A TOLL SPLIT AND HELD (the coordinator's rulings): a tariffed
/// traversal's charge, divided at acceptance into its exact parts — each
/// recipient's, the keeper's last — and held whole until the window `W` from
/// the acceptance closes (paid out, every part together) or the traversal's
/// OUT-side fold is reversed (back to the traveller, every part together).
/// The traverse act's stamp and receipt are what each part's rows cite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TollHeld {
    pub(crate) parts: Vec<([u8; 32], [u64; 4])>,
    pub(crate) until: u64,
    pub(crate) traverse: Stamp,
    pub(crate) receipt: [u8; 32],
}

impl TollHeld {
    pub(crate) fn put(&self, v: &mut Vec<u8>) {
        crate::transducer::address::put_key_count(v, self.parts.len());
        for (who, mu) in &self.parts {
            v.extend_from_slice(who);
            for m in mu {
                v.extend_from_slice(&m.to_le_bytes());
            }
        }
        v.extend_from_slice(&self.until.to_le_bytes());
        self.traverse.put_canonical(v);
        v.extend_from_slice(&self.receipt);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<TollHeld> {
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let parts = (0..crate::transducer::address::take_key_count(b, p)?)
            .map(|_| {
                let who: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                let mut mu = [0u64; 4];
                for m in &mut mu {
                    *m = u64_at(b, p)?;
                }
                Some((who, mu))
            })
            .collect::<Option<Vec<_>>>()?;
        let until = u64_at(b, p)?;
        let traverse = Stamp::take_canonical(b, p)?;
        let receipt: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(TollHeld { parts, until, traverse, receipt })
    }
}

/// 78.135 — A THROAT'S BANK (the coordinator's rulings): the μ-S holding a
/// tube open, held per FACE THAT FED IT under `throat_key(mouth, feeder)` —
/// the offerer's `⌈m⌉` at the offer, the keeper's feeds after — the net
/// change its feeds and draws made to the mass, and its count of bank acts.
/// Drawn, each feeder gets back its share in proportion; closed, each its
/// whole hold — never retired: the universe is self-maintaining.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Throat {
    pub(crate) feeders: Vec<[u8; 32]>,
    pub(crate) adjusted: i128,
    pub(crate) acts: u64,
}

impl Throat {
    pub(crate) fn put(&self, v: &mut Vec<u8>) {
        crate::transducer::address::put_key_count(v, self.feeders.len());
        for f in &self.feeders {
            v.extend_from_slice(f);
        }
        v.extend_from_slice(&self.adjusted.to_le_bytes());
        v.extend_from_slice(&self.acts.to_le_bytes());
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Throat> {
        let feeders = (0..crate::transducer::address::take_key_count(b, p)?)
            .map(|_| {
                let f: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                *p += 32;
                Some(f)
            })
            .collect::<Option<Vec<_>>>()?;
        let adjusted = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
        *p += 16;
        let acts = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        *p += 8;
        Some(Throat { feeders, adjusted, acts })
    }
}

/// The escrow key one face's share of a throat's bank is held under.
pub(crate) fn throat_key(mouth: &[u8; 32], feeder: &[u8; 32]) -> Vec<u8> {
    let mut k = b"throat".to_vec();
    k.extend_from_slice(mouth);
    k.extend_from_slice(feeder);
    k
}

/// `⌈n/d⌉` for a positive exact mass, in μ-S.
fn ceil_mass((n, d): (u64, u64)) -> Option<u64> {
    (d > 0).then(|| n.div_ceil(d))
}

impl Kernel {
    /// 78.135 — A TUBE'S MASS NOW: its committed mass, plus what its feeds
    /// added and less what its draws took.
    pub(crate) fn mass_now(&self, mouth: &[u8; 32]) -> Option<num_rational::BigRational> {
        let t = self.handoffs.mouths.get(mouth)?.mouth.tube.clone()?;
        let (n, d) = t.mass;
        if d == 0 {
            return None;
        }
        let adjusted = self.handoffs.throats.get(mouth).map_or(0, |th| th.adjusted);
        Some(num_rational::BigRational::new(n.into(), d.into()) + num_rational::BigRational::from_integer(adjusted.into()))
    }

    /// 78.135 — the throat's bank per feeder, μ-S held now.
    fn bank_of(&self, mouth: &[u8; 32]) -> Vec<([u8; 32], u64)> {
        let Some(th) = self.handoffs.throats.get(mouth) else { return Vec::new() };
        th.feeders.iter().filter_map(|f| Some((*f, self.treasury.held_of(&throat_key(mouth, f))?.mu[0]))).collect()
    }

    /// 78.135 — A THROAT EMPTIED: every face's hold back to it, whole; the
    /// bank's record let go.
    fn empty_throat(&mut self, mouth: &[u8; 32], anchor: crate::economy::Anchor) {
        let Some(th) = self.handoffs.throats.remove(mouth) else { return };
        for f in &th.feeders {
            let _ = self.treasury.refund_held(&throat_key(mouth, f), anchor);
        }
    }

    /// 78.135 — ONE FACE'S SHARE SET to `mu_s` μ-S: its hold returned and
    /// held again at the new amount (none at zero). False where the face
    /// cannot hold it.
    fn set_share(&mut self, mouth: &[u8; 32], feeder: &[u8; 32], mu_s: u64, anchor: crate::economy::Anchor) -> bool {
        let key = throat_key(mouth, feeder);
        if self.treasury.held_of(&key).is_some() && self.treasury.refund_held(&key, anchor).is_err() {
            return false;
        }
        mu_s == 0 || self.treasury.hold(*feeder, &key, [mu_s, 0, 0, 0], anchor).is_ok()
    }

    /// 78.135 — A THROAT FED OR DRAWN by its keeper: in turn (`seq`), on an
    /// open tube. Fed, the keeper banks `mu_s` more μ-S and the mass grows;
    /// drawn, `mu_s` of the bank returns to its faces in proportion to what
    /// each holds — each its floor, the keeper (or the first face) the rest —
    /// and the mass shrinks, never to zero.
    pub(crate) fn bank_act(&mut self, h: &SignedHandoff) -> Handed {
        use HandoffRefusal as R;
        let refuse = Handed::Refused;
        let (mouth, mu_s, seq, feeding) = match h.handoff {
            Handoff::Feed { mouth, mu_s, seq } => (mouth, mu_s, seq, true),
            Handoff::Draw { mouth, mu_s, seq } => (mouth, mu_s, seq, false),
            _ => return refuse(R::Unknown),
        };
        let Some(held) = self.handoffs.mouths.get(&mouth).cloned() else { return refuse(R::Unknown) };
        if held.mouth.tube.is_none() {
            return refuse(R::Untethered);
        }
        if h.who != held.mouth.keeper {
            return refuse(R::NotTheTier);
        }
        if held.state != State::Accepted {
            return refuse(R::NoMouth);
        }
        if self.handoffs.throats.get(&mouth).map_or(0, |t| t.acts) != seq {
            return refuse(R::Stale);
        }
        if mu_s == 0 {
            return refuse(R::Starved);
        }
        let bank = self.bank_of(&mouth);
        let total: u128 = bank.iter().map(|(_, m)| u128::from(*m)).sum();
        let mut parts: Vec<([u8; 32], u64)> = Vec::new();
        if feeding {
            let short = crate::economy::shortfall(&[mu_s, 0, 0, 0], &self.treasury.balance_mu(&h.who));
            if !crate::economy::lanes_short(&short).is_empty() {
                return refuse(R::HolderShort);
            }
        } else {
            // Never to zero: the mass after must stay positive.
            let after = self.mass_now(&mouth).map(|m| m - num_rational::BigRational::from_integer(mu_s.into()));
            if !after.is_some_and(|m| num_traits::Signed::is_positive(&m)) || u128::from(mu_s) > total || total == 0 {
                return refuse(R::Starved);
            }
            let mut rest = mu_s;
            for (f, have) in &bank {
                let share = u64::try_from(u128::from(mu_s) * u128::from(*have) / total).ok();
                let Some(share) = share else { return refuse(R::Starved) };
                rest -= share;
                parts.push((*f, share));
            }
            // The rest to the keeper where it fed and has room, else to the
            // first face with room — every part within its face's hold.
            let room = |p: &([u8; 32], u64), b: &[([u8; 32], u64)]| b.iter().find(|(f, _)| *f == p.0).map_or(0, |(_, h)| h - p.1);
            let at = parts.iter().position(|p| p.0 == held.mouth.keeper && room(p, &bank) >= rest).or_else(|| parts.iter().position(|p| room(p, &bank) >= rest));
            let Some(at) = at else { return refuse(R::Starved) };
            parts[at].1 += rest;
        }
        let kind = if feeding { 14 } else { 15 };
        let cite = Cite::Handoff { id: mouth, kind };
        let Some(r) = self.seal_step(ActKind::Handoff, [0; 4], h.who, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let anchor = (r.stamp, r.chain_hash);
        let mut th = match self.handoffs.throats.get(&mouth) {
            Some(t) => t.clone(),
            None => Throat::default(),
        };
        if feeding {
            let had = bank.iter().find(|(f, _)| *f == h.who).map_or(0, |(_, m)| *m);
            let Some(now) = had.checked_add(mu_s) else { return refuse(R::Starved) };
            let _ = self.set_share(&mouth, &h.who, now, anchor);
            if !th.feeders.contains(&h.who) {
                th.feeders.push(h.who);
            }
            th.adjusted += i128::from(mu_s);
        } else {
            for (f, share) in &parts {
                let had = bank.iter().find(|(g, _)| g == f).map_or(0, |(_, m)| *m);
                let _ = self.set_share(&mouth, f, had - share, anchor);
            }
            th.adjusted -= i128::from(mu_s);
        }
        th.acts += 1;
        self.handoffs.throats.insert(mouth, th);
        self.mark_persist();
        Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id: mouth, state: State::Accepted }
    }

    /// 78.133 — A TARIFF'S RECIPIENTS: the keeper, and the tier keys its own
    /// domain's genesis names (its progenitor lines) — never the traveller's
    /// domain, never a stranger.
    fn split_to_lines(&self, m: &Mouth, t: &crate::anastome::dialogue::handoff::Tariff) -> bool {
        let lines = self.tier_domain.get(&m.keeper).and_then(|d| self.domains.get(d));
        t.split.iter().all(|(who, _)| *who == m.keeper || lines.is_some_and(|tiers| tiers.contains(who)))
    }

    /// 78.133 — THE TOLLS WHOSE TRAVERSAL'S OUT SIDE a reversed fold consumed:
    /// every part back to the traveller together. Read before the fold's
    /// sides are opened again. Tolls refunded.
    pub(crate) fn unwind_tolls(&mut self, void: &[Vec<u8>], anchor: crate::economy::Anchor) -> usize {
        let hit: Vec<Vec<u8>> = self
            .handoffs
            .tolls
            .scoped()
            .map(|(k, _)| k)
            .filter(|id| self.handoffs.held.get(id).and_then(|h| h.out.as_ref()).is_some_and(|f| void.contains(f)))
            .collect();
        for id in &hit {
            let Ok(id32) = <[u8; 32]>::try_from(id.as_slice()) else { continue };
            if self.treasury.refund_held(&toll_key(&id32), anchor).is_ok() {
                self.handoffs.tolls.remove(id);
            }
        }
        hit.len()
    }

    /// 78.133 — THE TOLLS WHOSE WINDOW CLOSED, paid out: `W` from the
    /// acceptance passed, and the traversal's OUT-side fold — where one
    /// consumed it and is still recorded — read final (a withdrawal reopening
    /// it extends the hold). Every part together, its rows citing the
    /// traverse act. Tolls released.
    pub(crate) fn release_tolls(&mut self) -> usize {
        let head = self.chain.head_height();
        let due: Vec<(Vec<u8>, TollHeld)> = self
            .handoffs
            .tolls
            .scoped()
            .filter(|(id, t)| {
                let out = self.handoffs.held.get(id).and_then(|h| h.out.clone());
                let settled = match out {
                    Some(f) if self.ledgers.records.contains(&f) => Stamp::from_key(&f)
                        .is_some_and(|s| matches!(self.finality(&s), crate::anastome::dialogue::challenge::Finality::Final { .. })),
                    _ => true,
                };
                head >= t.until && settled
            })
            .map(|(k, t)| (k, t.clone()))
            .collect();
        let mut paid = 0;
        for (id, t) in due {
            let Ok(id32) = <[u8; 32]>::try_from(id.as_slice()) else { continue };
            if self.treasury.release_split(&toll_key(&id32), &t.parts, (t.traverse, t.receipt)).is_ok() {
                self.handoffs.tolls.remove(&id);
                paid += 1;
            }
        }
        paid
    }

    /// 78.133 — the μ held in toll escrow toward `tier`, per lane: its parts
    /// not yet released, so its receipts are not final before release.
    /// `None` where the sum passes its carrier.
    pub fn toll_held_for(&self, tier: &[u8; 32]) -> Option<[u64; 4]> {
        let mut held = [0u64; 4];
        for (_, t) in self.handoffs.tolls.scoped() {
            for (_, mu) in t.parts.iter().filter(|(who, _)| who == tier) {
                for (h, m) in held.iter_mut().zip(mu) {
                    *h = h.checked_add(*m)?;
                }
            }
        }
        Some(held)
    }

    /// 78.133 — whether a toll is held on `fold`: it consumed the OUT side of
    /// a traversal whose parts are not yet released.
    pub fn fold_holds_toll(&self, fold: &Stamp) -> bool {
        let key = fold.key();
        self.handoffs.tolls.scoped().any(|(id, _)| self.handoffs.held.get(&id).and_then(|h| h.out.as_ref()) == Some(&key))
    }

    /// 78.128 — WHETHER TWO DOMAINS ARE CONTIGUOUS — one physical universe,
    /// one shared chart — so a handoff between them needs no mouth. The hook
    /// a later act fills (two domain keys declaring it); none is, so none are.
    pub(crate) fn contiguous(&self, _x: &[u8; 32], _y: &[u8; 32]) -> bool {
        false
    }

    /// A traversal's mouth: OPEN, and joining the two domains its handoff
    /// crosses between, in either direction.
    pub(crate) fn through(&self, mouth: &[u8; 32], x: &[u8; 32], y: &[u8; 32]) -> Result<(), HandoffRefusal> {
        let Some(m) = self.handoffs.mouths.get(mouth) else { return Err(HandoffRefusal::NoMouth) };
        if m.state != State::Accepted {
            return Err(HandoffRefusal::NoMouth);
        }
        let ends = (self.tier_domain.get(&m.mouth.from_tier).copied(), self.tier_domain.get(&m.mouth.to_tier).copied());
        match ends {
            (Some(a), Some(b)) if (a == *x && b == *y) || (a == *y && b == *x) => Ok(()),
            _ => Err(HandoffRefusal::NotTheMouth),
        }
    }

    /// A traversal's toll, answered: to its mouth's KEEPER where the handoff
    /// was accepted, back to the sending tier otherwise. Nothing where no
    /// toll is held.
    pub(crate) fn settle_toll(&mut self, id: &[u8; 32], accepted: bool, anchor: crate::economy::Anchor) {
        let Some(mouth) = self.handoffs.through.get(id).copied() else { return };
        let key = toll_key(id);
        if self.treasury.held_of(&key).is_none() {
            return;
        }
        let Some(m) = self.handoffs.mouths.get(&mouth).map(|m| m.mouth.clone()) else {
            let _ = self.treasury.refund_held(&key, anchor);
            return;
        };
        if !accepted {
            let _ = self.treasury.refund_held(&key, anchor);
            return;
        }
        // 78.133 — a TARIFFED toll is split at acceptance and HELD to the
        // window's close; a flat toll (78.128) is the keeper's at once.
        let Some(tariff) = &m.tariff else {
            let _ = self.treasury.release_held(&key, m.keeper, anchor);
            return;
        };
        let charge = self.treasury.held_of(&key).map(|h| h.mu);
        let traverse = self.handoffs.offered_at.get(id).and_then(|k| Stamp::from_key(k)).and_then(|at| Some((at, self.chain.node(&at)?.chain_hash)));
        match (charge.and_then(|c| tariff.parts(&m.keeper, &c)), traverse) {
            (Some(parts), Some((at, receipt))) => {
                let held = TollHeld { parts, until: window_end(anchor.0.h), traverse: at, receipt };
                self.handoffs.tolls.insert(id, held);
            }
            // Parts that do not read are no toll: back to the traveller.
            _ => {
                let _ = self.treasury.refund_held(&key, anchor);
            }
        }
    }

    /// 78.128 — A MOUTH ACT: the pair offered (by the sending reserve), its
    /// answer (the other reserve accepts or refuses within `W`; the offerer
    /// claims expiry past it), or its close (either reserve).
    pub(crate) fn mouth_act(&mut self, h: &SignedHandoff) -> Handed {
        use HandoffRefusal as R;
        let refuse = Handed::Refused;
        let id = h.handoff.id();
        let head = self.chain.head_height();
        let kind = match &h.handoff {
            Handoff::Mouth(m) => {
                if let Some(held) = self.handoffs.mouths.get(&id) {
                    // A resend: its original receipt, nothing sealed twice.
                    return match Stamp::from_key(&held.at).and_then(|at| Some((at, self.chain.node(&at)?.chain_hash))) {
                        Some((at, receipt)) => Handed::Sealed { at, receipt, id, state: held.state },
                        None => refuse(R::Offered),
                    };
                }
                if h.who != m.from_tier {
                    return refuse(R::NotTheTier);
                }
                let (dx, dy) = (self.tier_domain.get(&m.from_tier).copied(), self.tier_domain.get(&m.to_tier).copied());
                let (Some(dx), Some(dy)) = (dx, dy) else { return refuse(R::Unbound) };
                if dx == dy {
                    return refuse(R::Unbound);
                }
                // The builders are the ancients: both ends their domains' reserves.
                if self.reserve_of(&dx) != Some(m.from_tier) || self.reserve_of(&dy) != Some(m.to_tier) {
                    return refuse(R::NotReserve);
                }
                if m.keeper != m.from_tier && m.keeper != m.to_tier {
                    return refuse(R::BadKeeper);
                }
                if !m.frames_hold() {
                    return refuse(R::Degenerate);
                }
                // 78.134 — A MOUTH IS A TUBE between two places on curved
                // space, never two free frames: its frames motions of `ℍ³`,
                // its mass and curvatures positive, and a mouth it
                // supersedes an open untethered one of the same pair.
                let Some(t) = &m.tube else { return refuse(R::Untethered) };
                if !t.holds() {
                    return refuse(R::BadTube);
                }
                if !m.frames_curved() {
                    return refuse(R::NotIsometry);
                }
                if let Some(old) = &t.supersedes {
                    let same_pair = |o: &Mouth| (o.from_tier, o.to_tier) == (m.from_tier, m.to_tier) || (o.from_tier, o.to_tier) == (m.to_tier, m.from_tier);
                    let open_legacy = self.handoffs.mouths.get(old).is_some_and(|h| h.mouth.tube.is_none() && h.state == State::Accepted && same_pair(&h.mouth));
                    if !open_legacy {
                        return refuse(R::NotSupersedable);
                    }
                }
                // 78.133 — a tariff well formed, no flat toll beside it, and
                // its shares to the keeper and its progenitor lines only.
                if let Some(t) = &m.tariff {
                    if let Err(why) = t.holds() {
                        return refuse(R::Untariffed(why));
                    }
                    if m.toll.iter().any(|x| *x > 0) {
                        return refuse(R::Untariffed(crate::anastome::dialogue::handoff::Untariffed::Flat));
                    }
                    if !self.split_to_lines(m, t) {
                        return refuse(R::NotALine);
                    }
                }
                // 78.135 — the throat held open by matter set aside: the
                // offerer banks `⌈m⌉` μ-S, or the tube is not offered.
                let Some(bank) = ceil_mass(t.mass) else { return refuse(R::BadTube) };
                let short = crate::economy::shortfall(&[bank, 0, 0, 0], &self.treasury.balance_mu(&h.who));
                if !crate::economy::lanes_short(&short).is_empty() {
                    return refuse(R::HolderShort);
                }
                7
            }
            Handoff::Close { .. } => {
                let Some(held) = self.handoffs.mouths.get(&id) else { return refuse(R::Unknown) };
                if h.who != held.mouth.from_tier && h.who != held.mouth.to_tier {
                    return refuse(R::NotTheTier);
                }
                if held.state != State::Accepted {
                    return refuse(R::Answered);
                }
                8
            }
            Handoff::Accept { .. } | Handoff::Refuse { .. } | Handoff::Expire { .. } => {
                let Some(held) = self.handoffs.mouths.get(&id) else { return refuse(R::Unknown) };
                let State::Offered { until } = held.state else { return refuse(R::Answered) };
                let expiring = matches!(h.handoff, Handoff::Expire { .. });
                let filer = if expiring { held.mouth.from_tier } else { held.mouth.to_tier };
                if h.who != filer {
                    return refuse(R::NotTheTier);
                }
                if expiring != (head >= until) {
                    return refuse(R::Window);
                }
                match h.handoff {
                    Handoff::Accept { .. } => 1,
                    Handoff::Refuse { .. } => 2,
                    _ => 3,
                }
            }
            _ => return refuse(R::Unknown),
        };
        let cite = Cite::Handoff { id, kind };
        let Some(r) = self.seal_step(ActKind::Handoff, [0; 4], h.who, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let anchor = (r.stamp, r.chain_hash);
        let state = match (&h.handoff, kind) {
            (Handoff::Mouth(m), _) => {
                let until = window_end(r.stamp.h);
                let held = MouthHeld { mouth: m.as_ref().clone(), state: State::Offered { until }, opened: None, at: r.stamp.key() };
                self.handoffs.mouths.insert(id, held);
                // 78.135 — the offerer's bank, the throat's first face.
                if let Some(bank) = m.tube.as_ref().and_then(|t| ceil_mass(t.mass)) {
                    if self.set_share(&id, &h.who, bank, anchor) {
                        self.handoffs.throats.insert(id, Throat { feeders: vec![h.who], adjusted: 0, acts: 0 });
                    }
                }
                State::Offered { until }
            }
            (_, 1) => {
                let mut superseded = None;
                if let Some(held) = self.handoffs.mouths.get_mut(&id) {
                    held.state = State::Accepted;
                    held.opened = Some(r.stamp.h);
                    superseded = held.mouth.tube.as_ref().and_then(|t| t.supersedes);
                }
                // 78.134 — the untethered mouth a tube supersedes closes as
                // it opens: what is in flight through it returns; its record
                // stays, cited by the tube.
                if let Some(old) = superseded {
                    self.close_mouth(&old, anchor);
                }
                State::Accepted
            }
            (_, 8) => {
                self.close_mouth(&id, anchor);
                State::Returned
            }
            _ => {
                if let Some(held) = self.handoffs.mouths.get_mut(&id) {
                    held.state = State::Returned;
                }
                // Refused or expired: the offerer's bank back to it.
                self.empty_throat(&id, anchor);
                State::Returned
            }
        };
        self.mark_persist();
        Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state }
    }

    /// A mouth pair CLOSED: returned, the traversals in flight through it
    /// returned, their tolls back, and — 78.135 — its throat's bank back to
    /// the faces that fed it, never retired.
    fn close_mouth(&mut self, id: &[u8; 32], anchor: crate::economy::Anchor) {
        if let Some(held) = self.handoffs.mouths.get_mut(id) {
            held.state = State::Returned;
        }
        self.empty_throat(id, anchor);
        let in_flight: Vec<[u8; 32]> = self
            .handoffs
            .through
            .scoped()
            .filter(|(_, m)| *m == id)
            .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
            .filter(|k| self.handoffs.held.get(k).is_some_and(|t| matches!(t.state, State::Offered { .. })))
            .collect();
        for t in in_flight {
            if let Some(held) = self.handoffs.held.get_mut(&t) {
                held.state = State::Returned;
            }
            self.settle_toll(&t, false, anchor);
        }
    }

    /// 78.134 — A MOUTH PAIR'S TUBE, derived and never stored: each mouth's
    /// place on its domain's `ℍ³` (its frame's base point), each end's
    /// horizon level from the throat's mass in that end's curvature. `None`
    /// for an untethered mouth or one whose readings do not hold.
    pub fn tube_of(&self, mouth: &[u8; 32]) -> Option<TubeReading> {
        let m = &self.handoffs.mouths.get(mouth)?.mouth;
        let t = m.tube.as_ref()?;
        let q = |(n, d): (u64, u64)| (d > 0).then(|| num_rational::BigRational::new(n.into(), d.into()));
        // 78.135 — the mass NOW: fed and drawn, the horizon re-derived.
        let mass = self.mass_now(mouth)?;
        let numer = u64::try_from(mass.numer()).ok()?;
        let denom = u64::try_from(mass.denom()).ok()?;
        Some(TubeReading {
            from_place: crate::transducer::cga::frame_base_point(&m.from_frame)?,
            to_place: crate::transducer::cga::frame_base_point(&m.to_frame)?,
            from_level: crate::client::sdk::physics::throat_level(&mass, &q(t.from_r)?)?,
            to_level: crate::client::sdk::physics::throat_level(&mass, &q(t.to_r)?)?,
            mass: (numer, denom),
        })
    }

    /// 78.128 — THE MOUTH LOOPS: the open mouth pairs as edges between
    /// domains, in the order they opened, each carrying its chart transition
    /// `to_frame · from_frame⁻¹` — 78.134: a motion alone, no rotor slot;
    /// every pair that closes a cycle of domains whose holonomy is not the
    /// identity, with that holonomy, TETHERED where the tubes alone close the
    /// same cycle to the same holonomy (an isometry of curved space), else a
    /// cycle through an untethered mouth, history. Read, never stored (law
    /// 12); never a refusal — curvature, recorded.
    pub fn mouth_loops(&self) -> Vec<Loop> {
        // In the order they opened: keyed by the opening height, then the id.
        let mut open: crate::network::nerve::Keyed<([u8; 32], Mouth)> = crate::network::nerve::Keyed::new();
        for (k, m) in self.handoffs.mouths.scoped() {
            let (Some(h), Ok(id)) = (m.opened, <[u8; 32]>::try_from(k.as_slice())) else { continue };
            if m.state != State::Accepted {
                continue;
            }
            let mut at = h.to_be_bytes().to_vec();
            at.extend_from_slice(&id);
            open.insert(at, (id, m.mouth.clone()));
        }
        let (mut all, mut tubes): (Gauge<Moved>, Gauge<Moved>) = (Gauge::default(), Gauge::default());
        let mut loops = Vec::new();
        for (_, (id, m)) in open.scoped() {
            let id = *id;
            let (Some(x), Some(y)) = (self.tier_domain.get(&m.from_tier), self.tier_domain.get(&m.to_tier)) else { continue };
            let transition = Moved { motion: m.to_frame.geo(&m.from_frame.reverse()) };
            let in_tubes = match m.tube.is_some() {
                true => Some(tubes.edge(x, y, &transition)),
                false => None,
            };
            if let Gauged::Obstructed(h) = all.edge(x, y, &transition) {
                if !h.is_identity() {
                    let tethered = matches!(in_tubes, Some(Gauged::Obstructed(t)) if crate::transducer::element::same_versor(&t.motion, &h.motion));
                    loops.push(Loop { mouth: id, holonomy: h.motion, tethered });
                }
            }
        }
        loops
    }
}
