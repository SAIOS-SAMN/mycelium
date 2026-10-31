// 78.127 — 2(e) μ CONVERSION on the court: a domain's RESERVE tier, and game
// value exchanged against μ already credited to it — never minted by a rate
// (ruled 2026-10-07).
//
//! μ exists only as the hypha makes it, and moves only key to key, Σ = 0
//! (`Treasury::pay`, `hold`, `release_held`, `refund_held`). A domain's
//! RESERVE is the μ balance of the one tier its key designates. CONVERT-OUT
//! pays μ from the reserve to a holder against game value leaving (a G sink
//! on the in-game holder's identity); CONVERT-IN holds a holder's μ until the
//! reserve tier accepts it into the reserve against game value arriving (a G
//! source), or refuses, or it expires — then it is refunded. Each G side is
//! consumed by the reserve tier's fold, exactly once, like a handoff side.
//! The SLASH of a falsified fold draws from its domain's reserve: half the
//! challenger's bounty, half retired (POWR) — the hook is wired and moves
//! nothing until bonds price it.

use super::challenge::window_end;
use super::Court;
use crate::anastome::dialogue::domain::{Founded, SignedReservation};
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Side, SignedHandoff, State};
use crate::anastome::dialogue::uplink::Net;
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// The escrow key a tier's bond is held under.
pub(crate) fn bond_key(tier: &[u8; 32]) -> Vec<u8> {
    let mut k = b"bond".to_vec();
    k.extend_from_slice(tier);
    k
}

/// A DOMAIN'S RESERVE: the tier whose μ balance it is, and a redesignation
/// pending until that tier's next admitted fold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct Reserve {
    pub(crate) current: Option<[u8; 32]>,
    pub(crate) pending: Option<[u8; 32]>,
}

/// WHICH WAY μ MOVED.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Dir {
    /// μ out of the reserve; game value leaves (a G sink).
    Out,
    /// μ into the reserve; game value arrives (a G source).
    In,
}

/// ONE CONVERSION HELD while its G side is open.
#[derive(Clone, Debug)]
pub(crate) struct Conversion {
    pub(crate) dir: Dir,
    pub(crate) tier: [u8; 32],
    pub(crate) holder: [u8; 32],
    pub(crate) identity: [u8; 32],
    pub(crate) g: i128,
    pub(crate) mu: [u64; 4],
    pub(crate) state: State,
    pub(crate) side: Option<Vec<u8>>,
    /// The receipt that sealed it, by its stamp key — a resend's answer.
    pub(crate) at: Vec<u8>,
}

impl Conversion {
    /// The nets its G side carries into the reserve tier's fold.
    pub(crate) fn nets(&self) -> Vec<Net> {
        let g = match self.dir {
            Dir::Out => -self.g,
            Dir::In => self.g,
        };
        vec![(self.identity, [0, g, 0, 0])]
    }

    /// Which side of a fold's refs it is consumed as.
    pub(crate) fn side_kind(&self) -> Side {
        match self.dir {
            Dir::Out => Side::Out,
            Dir::In => Side::In,
        }
    }

    pub(crate) fn put(&self, v: &mut Vec<u8>) {
        v.push(match self.dir {
            Dir::Out => 0,
            Dir::In => 1,
        });
        for k in [&self.tier, &self.holder, &self.identity] {
            v.extend_from_slice(k);
        }
        v.extend_from_slice(&self.g.to_le_bytes());
        for m in self.mu {
            v.extend_from_slice(&m.to_le_bytes());
        }
        match self.state {
            State::Offered { until } => {
                v.push(0);
                v.extend_from_slice(&until.to_le_bytes());
            }
            State::Accepted => v.push(1),
            State::Returned => v.push(2),
        }
        match &self.side {
            None => v.push(0),
            Some(k) => {
                v.push(1);
                put_key_bytes(v, k);
            }
        }
        put_key_bytes(v, &self.at);
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Conversion> {
        let byte = |b: &[u8], p: &mut usize| -> Option<u8> {
            let x = *b.get(*p)?;
            *p += 1;
            Some(x)
        };
        let key = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        };
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let dir = match byte(b, p)? {
            0 => Dir::Out,
            1 => Dir::In,
            _ => return None,
        };
        let (tier, holder, identity) = (key(b, p)?, key(b, p)?, key(b, p)?);
        let g = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
        *p += 16;
        let mut mu = [0u64; 4];
        for m in &mut mu {
            *m = u64_at(b, p)?;
        }
        let state = match byte(b, p)? {
            0 => State::Offered { until: u64_at(b, p)? },
            1 => State::Accepted,
            2 => State::Returned,
            _ => return None,
        };
        let side = match byte(b, p)? {
            0 => None,
            1 => Some(take_key_bytes(b, p)?),
            _ => return None,
        };
        Some(Conversion { dir, tier, holder, identity, g, mu, state, side, at: take_key_bytes(b, p)? })
    }
}

/// The reserves written: `[count][(domain 32, current?, pending?) …]`.
pub(crate) fn put_reserves(v: &mut Vec<u8>, reserves: &Keyed<Reserve>) {
    put_key_count(v, reserves.len());
    for (domain, r) in reserves.scoped() {
        v.extend_from_slice(&domain);
        for t in [r.current, r.pending] {
            match t {
                None => v.push(0),
                Some(t) => {
                    v.push(1);
                    v.extend_from_slice(&t);
                }
            }
        }
    }
}

pub(crate) fn take_reserves(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], Reserve)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let mut tier = || -> Option<Option<[u8; 32]>> {
                let flag = *b.get(*p)?;
                *p += 1;
                match flag {
                    0 => Some(None),
                    1 => {
                        let t: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                        *p += 32;
                        Some(Some(t))
                    }
                    _ => None,
                }
            };
            let (current, pending) = (tier()?, tier()?);
            Some((domain, Reserve { current, pending }))
        })
        .collect()
}

/// 78.132 — the origins written: `[count][(domain 32, origin) …]`.
pub(crate) fn put_origins(v: &mut Vec<u8>, origins: &Keyed<crate::anastome::dialogue::domain::OriginReading>) {
    put_key_count(v, origins.len());
    for (domain, o) in origins.scoped() {
        v.extend_from_slice(&domain);
        o.put(v);
    }
}

pub(crate) fn take_origins(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], crate::anastome::dialogue::domain::OriginReading)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some((domain, crate::anastome::dialogue::domain::OriginReading::take(b, p)?))
        })
        .collect()
}

impl Court {
    /// 78.127 — A DOMAIN'S RESERVE TIER DESIGNATED: the domain's signature,
    /// the domain founded here, the tier one its genesis names. A domain with
    /// no reserve takes it at once; otherwise it is pending until the current
    /// reserve tier's next admitted fold — its era anchor, never mid-era.
    pub fn designate_reserve(&mut self, s: &SignedReservation) -> Founded {
        let r = &s.reservation;
        // 78.136 — THE DOMAIN DESIGNATES ONCE, at founding; every succession
        // after is signed by the SITTING reserve — the trust passes to the
        // ancients who arise, never back to the founder's key alone.
        let signer = match self.reserve_of(&r.domain) {
            Some(sitting) => sitting,
            None => r.domain,
        };
        if !crate::court::identity::verify(&signer, &r.signed_bytes(), &s.sig) {
            return Founded::Unsigned;
        }
        let Some(tiers) = self.domains.get(&r.domain) else { return Founded::NotFounded };
        if !tiers.contains(&r.tier) {
            return Founded::NotATier;
        }
        let cite = Cite::Reserve { domain: r.domain, tier: r.tier };
        let Some(sealed) = self.seal_step(ActKind::Domain, [0; 4], r.domain, |_| vec![cite.clone()]) else { return Founded::LineRefused };
        let mut held = match self.reserves.get(&r.domain) {
            Some(r) => *r,
            None => Reserve::default(),
        };
        let now = held.current.is_none();
        match now {
            true => held.current = Some(r.tier),
            false => held.pending = Some(r.tier),
        }
        self.reserves.insert(r.domain, held);
        self.mark_persist();
        Founded::Reserved { at: sealed.stamp, receipt: sealed.chain_hash, now }
    }

    /// 78.127 — A DOMAIN'S RESERVE AS READ: its tier now, a redesignation
    /// pending, and the reserve's μ balance. Sealing nothing.
    pub fn reserve_reading(&self, domain: &[u8; 32]) -> crate::anastome::dialogue::domain::ReserveReading {
        let held = match self.reserves.get(domain) {
            Some(r) => *r,
            None => Reserve::default(),
        };
        let balance = match held.current {
            Some(t) => self.treasury.balance_mu(&t),
            None => [0; 4],
        };
        crate::anastome::dialogue::domain::ReserveReading { current: held.current, pending: held.pending, balance }
    }

    /// 78.132 — A UNIVERSE'S ORIGIN RECORDED: the domain's signature, the
    /// domain founded here, and no origin held for it yet — once, never an
    /// edit (78.59 Ruling 2). Sealed on the line by its coordinates; every
    /// fold of the domain after it descends from it by line precedence.
    pub fn record_origin(&mut self, s: &crate::anastome::dialogue::domain::SignedOrigin) -> Founded {
        let o = &s.origin;
        if !crate::court::identity::verify(&o.domain, &o.signed_bytes(), &s.sig) {
            return Founded::Unsigned;
        }
        if self.domains.get(&o.domain).is_none() {
            return Founded::NotFounded;
        }
        if self.origins.get(&o.domain).is_some() {
            return Founded::AlreadyOriginated;
        }
        // 78.139 — version 2 states the universe's curvature and parent face.
        let mut cites = vec![Cite::Origin { domain: o.domain, kind: o.kind, version: o.version, commitment: o.commitment }];
        match self.origin_universe(o) {
            Ok(more) => cites.extend(more),
            Err(refused) => return refused,
        }
        let Some(sealed) = self.seal_step(ActKind::Domain, [0; 4], o.domain, |_| cites.clone()) else { return Founded::LineRefused };
        if let Some(u) = &o.universe {
            self.hold_genesis(o.domain, u);
        }
        let held = crate::anastome::dialogue::domain::OriginReading {
            kind: o.kind,
            version: o.version,
            commitment: o.commitment,
            at: sealed.stamp,
            receipt: sealed.chain_hash,
        };
        self.origins.insert(o.domain, held);
        self.mark_persist();
        Founded::Originated { at: sealed.stamp, receipt: sealed.chain_hash }
    }

    /// 78.132 — A DOMAIN'S ORIGIN AS HELD, or none. Sealing nothing.
    pub fn origin_of(&self, domain: &[u8; 32]) -> Option<crate::anastome::dialogue::domain::OriginReading> {
        self.origins.get(domain).copied()
    }

    /// The tier whose μ balance is a domain's reserve now.
    pub fn reserve_of(&self, domain: &[u8; 32]) -> Option<[u8; 32]> {
        self.reserves.get(domain)?.current
    }

    /// Whether `tier` is the SITTING RESERVE — the ancients' house — of a
    /// founded domain.
    pub fn is_ancients_house(&self, tier: &[u8; 32]) -> bool {
        self.tier_domain.get(tier).is_some_and(|d| self.reserve_of(d) == Some(*tier))
    }

    /// WHETHER A KEY IS AN INNER SEAT OF ITS DOMAIN: a tier the domain itself
    /// registered that is not its sitting reserve — the ancients' house.
    ///
    /// A READING, NEVER A GATE (ruled 2026-10-14: any holder of a key may
    /// solve). The court authenticates a KEY, and whoever holds it
    /// is served and paid: keeping the house's key is the ancients' charge,
    /// and a tier reaching the hypha directly is the game kernel's to answer,
    /// with game-kernel repercussions. The court cannot know a tier — only
    /// what a domain declared of its own keys — so a refusal here bound only
    /// the kernels that would have relayed anyway. What it does is DETECT:
    /// the door counts the solves an inner seat signed, so a kernel, or a
    /// stipulation ruled later, has the reading to act on.
    pub fn inner_seat(&self, signer: &[u8; 32]) -> bool {
        self.tier_domain.get(signer).is_some_and(|d| self.reserve_of(d) != Some(*signer))
    }

    /// A fold admitted: where its tier is its domain's reserve and a
    /// redesignation is pending, the anchor has passed and the new reserve
    /// holds from here.
    pub(crate) fn anchor_passed(&mut self, tier: &[u8; 32], domain: &[u8; 32]) {
        let Some(held) = self.reserves.get_mut(domain) else { return };
        if held.current == Some(*tier) {
            if let Some(next) = held.pending.take() {
                held.current = Some(next);
                self.mark_persist();
            }
        }
    }

    /// Whether `who`'s balance holds `mu` on every lane.
    fn covers(&self, who: &[u8; 32], mu: &[u64; 4]) -> bool {
        crate::economy::lanes_short(&crate::economy::shortfall(mu, &self.treasury.balance_mu(who))).is_empty()
    }

    /// The reserve tier of the domain `tier` stands in, where `tier` is it.
    fn is_reserve(&self, tier: &[u8; 32]) -> bool {
        self.tier_domain.get(tier).and_then(|d| self.reserve_of(d)) == Some(*tier)
    }

    /// 78.127 — A CONVERSION ACT: out (the reserve tier pays μ against game
    /// value leaving) or in (a holder offers μ against game value arriving),
    /// or an answer to an offer in (accept, refuse, expire).
    pub(crate) fn convert(&mut self, h: &SignedHandoff) -> Handed {
        use HandoffRefusal as R;
        let refuse = Handed::Refused;
        let id = h.handoff.id();
        let head = self.chain.head_height();
        // A RESEND is answered with its original receipt: nothing twice.
        if matches!(h.handoff, Handoff::ConvertOut(_) | Handoff::ConvertIn(_)) {
            if let Some(c) = self.handoffs.conversions.get(&id) {
                return match Stamp::from_key(&c.at).and_then(|at| Some((at, self.chain.node(&at)?.chain_hash))) {
                    Some((at, receipt)) => Handed::Sealed { at, receipt, id, state: c.state },
                    None => refuse(R::Offered),
                };
            }
        }
        match &h.handoff {
            Handoff::ConvertOut(c) => {
                if h.who != c.tier || !self.is_reserve(&c.tier) {
                    return refuse(R::NotReserve);
                }
                // 78.139 — the reserve unit is the ancients' basket: none
                // converted until it is stated.
                if !self.basket_stated_for(&c.tier) {
                    return refuse(R::NoBasket);
                }
                let Some(mu) = c.mu() else { return refuse(R::Inexact) };
                if mu.iter().all(|m| *m == 0) {
                    return refuse(R::Inexact);
                }
                // THE RESERVE HOLDS IT — a rate never mints μ.
                if !self.covers(&c.tier, &mu) {
                    return refuse(R::ReserveShort);
                }
                let Some(r) = self.seal_handoff(h, id, 5) else { return refuse(R::LineRefused) };
                if self.treasury.pay(c.tier, c.holder, mu, (r.stamp, r.chain_hash)).is_err() {
                    return refuse(R::ReserveShort);
                }
                let held = Conversion { dir: Dir::Out, tier: c.tier, holder: c.holder, identity: c.identity, g: c.g, mu, state: State::Accepted, side: None, at: r.stamp.key() };
                self.hold_conversion(id, held);
                Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state: State::Accepted }
            }
            Handoff::ConvertIn(c) => {
                if h.who != c.holder {
                    return refuse(R::NotTheTier);
                }
                if !self.is_reserve(&c.tier) {
                    return refuse(R::NotReserve);
                }
                if !self.basket_stated_for(&c.tier) {
                    return refuse(R::NoBasket);
                }
                if c.g <= 0 || c.mu.iter().all(|m| *m == 0) {
                    return refuse(R::Inexact);
                }
                if !self.covers(&c.holder, &c.mu) {
                    return refuse(R::HolderShort);
                }
                let Some(r) = self.seal_handoff(h, id, 6) else { return refuse(R::LineRefused) };
                if self.treasury.hold(c.holder, &id, c.mu, (r.stamp, r.chain_hash)).is_err() {
                    return refuse(R::HolderShort);
                }
                let until = window_end(r.stamp.h);
                let held = Conversion {
                    dir: Dir::In,
                    tier: c.tier,
                    holder: c.holder,
                    identity: c.identity,
                    g: c.g,
                    mu: c.mu,
                    state: State::Offered { until },
                    side: None,
                    at: r.stamp.key(),
                };
                self.hold_conversion(id, held);
                Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state: State::Offered { until } }
            }
            Handoff::Accept { .. } | Handoff::Refuse { .. } | Handoff::Expire { .. } => {
                let Some(held) = self.handoffs.conversions.get(&id).cloned() else { return refuse(R::Unknown) };
                let State::Offered { until } = held.state else { return refuse(R::Answered) };
                let expiring = matches!(h.handoff, Handoff::Expire { .. });
                let filer = if expiring { held.holder } else { held.tier };
                if h.who != filer {
                    return refuse(R::NotTheTier);
                }
                if expiring != (head >= until) {
                    return refuse(R::Window);
                }
                let kind = match h.handoff {
                    Handoff::Accept { .. } => 1,
                    Handoff::Refuse { .. } => 2,
                    _ => 3,
                };
                let Some(r) = self.seal_handoff(h, id, kind) else { return refuse(R::LineRefused) };
                let anchor = (r.stamp, r.chain_hash);
                let state = match kind {
                    1 => match self.treasury.release_held(&id, held.tier, anchor) {
                        Ok(_) => State::Accepted,
                        Err(_) => return refuse(R::HolderShort),
                    },
                    _ => {
                        let _ = self.treasury.refund_held(&id, anchor);
                        State::Returned
                    }
                };
                if let Some(c) = self.handoffs.conversions.get_mut(&id) {
                    c.state = state;
                }
                self.mark_persist();
                Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state }
            }
            Handoff::Offer(_)
            | Handoff::Mouth(_)
            | Handoff::Close { .. }
            | Handoff::Traverse { .. }
            | Handoff::Feed { .. }
            | Handoff::Draw { .. } => refuse(R::Unknown),
        }
    }

    fn seal_handoff(&mut self, h: &SignedHandoff, id: [u8; 32], kind: u8) -> Option<crate::chronometry::KStepReceipt> {
        let cite = Cite::Handoff { id, kind };
        self.seal_step(ActKind::Handoff, [0; 4], h.who, |_| vec![cite.clone()])
    }

    fn hold_conversion(&mut self, id: [u8; 32], c: Conversion) {
        self.handoffs.conversions.insert(id, c);
        self.mark_persist();
    }

    /// 78.129 — A TIER'S BOND as it stands: the μ its domain's reserve holds
    /// escrowed in its name (zero where none is posted).
    pub fn bond_of(&self, tier: &[u8; 32]) -> [u64; 4] {
        self.treasury.held_of(&bond_key(tier)).map_or([0; 4], |h| h.mu)
    }

    /// Whether the tier that sealed this fold stands bonded — what a finality
    /// read carries so a reader weighs an unbacked fold.
    pub fn fold_bonded(&self, fold: &crate::anastome::dialogue::uplink::FoldAt) -> bool {
        let key = fold.key();
        let tier = match (self.ledgers.records.get(&key), self.ledgers.finals.get(&key)) {
            (Some(r), _) => Some(r.tier),
            (None, Some(((tier, ..), _))) => Some(*tier),
            (None, None) => self.sealed_fold(fold).map(|(tier, ..)| tier),
        };
        tier.is_some_and(|t| self.bond_of(&t).iter().any(|m| *m > 0))
    }

    /// 78.129 — A TIER'S BOND POSTED OR WITHDRAWN (as
    /// ruled): signed by its domain's RESERVE, for a tier its genesis names.
    /// Posted, the reserve escrows `mu` more in the tier's name (a top-up of
    /// what it holds; a bond an earlier reserve posted returns to it, and the
    /// new reserve backs the whole). Withdrawn, it returns to the reserve —
    /// only once every fold of the tier is final: no bond leaves while a fold
    /// it backs can still be falsified.
    pub fn bond(&mut self, s: &crate::anastome::dialogue::domain::SignedBond) -> Founded {
        let b = &s.bond;
        if !crate::court::identity::verify(&b.reserve, &b.signed_bytes(), &s.sig) {
            return Founded::Unsigned;
        }
        let Some(tiers) = self.domains.get(&b.domain) else { return Founded::NotFounded };
        if !tiers.contains(&b.tier) {
            return Founded::NotATier;
        }
        if self.reserve_of(&b.domain) != Some(b.reserve) {
            return Founded::NotReserve;
        }
        let key = bond_key(&b.tier);
        let held = self.treasury.held_of(&key).cloned();
        let kept = held.as_ref().filter(|h| h.payer == b.reserve).map_or([0; 4], |h| h.mu);
        let after = match b.mu {
            Some(mu) => {
                if !self.covers(&b.reserve, &mu) {
                    return Founded::Short;
                }
                let mut total = [0u64; 4];
                for (t, (k, m)) in total.iter_mut().zip(kept.iter().zip(mu)) {
                    let Some(sum) = k.checked_add(m) else { return Founded::Short };
                    *t = sum;
                }
                total
            }
            None => {
                if held.is_none() {
                    return Founded::Unbonded;
                }
                if self.tier_outstanding(&b.tier) {
                    return Founded::Outstanding;
                }
                [0; 4]
            }
        };
        let cite = Cite::Bond { domain: b.domain, tier: b.tier, mu: after };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], b.reserve, |_| vec![cite.clone()]) else { return Founded::LineRefused };
        let anchor = (r.stamp, r.chain_hash);
        if held.is_some() {
            let _ = self.treasury.refund_held(&key, anchor);
        }
        if after.iter().any(|m| *m > 0) && self.treasury.hold(b.reserve, &key, after, anchor).is_err() {
            return Founded::Short;
        }
        self.mark_persist();
        Founded::Bonded { at: r.stamp, receipt: r.chain_hash, bond: after }
    }

    /// Whether a fold of the tier is not yet final — one its bond still backs.
    fn tier_outstanding(&self, tier: &[u8; 32]) -> bool {
        let keys: Vec<Vec<u8>> = self.ledgers.records.scoped().filter(|(_, r)| r.tier == *tier && r.standing == super::ledger::Standing::Open).map(|(k, _)| k).collect();
        keys.iter().any(|k| {
            crate::anastome::dialogue::uplink::FoldAt::from_key(k).is_none_or(|at| !matches!(self.finality(&at), crate::anastome::dialogue::challenge::Finality::Final { .. }))
        })
    }

    /// 78.129 — THE SLASH OF A FALSIFIED FOLD: half the tier's bond, on each
    /// lane, floored — the odd unit stays bonded — forfeited as ruled: half
    /// to the challenger whose replay or default falsified it (the bounty: a
    /// challenge needs a reason to be filed), half RETIRED from supply (POWR),
    /// an odd unit to retirement. Drawn from whoever posted the bond; nothing
    /// where the tier stands unbonded.
    pub(crate) fn slash(&mut self, tier: &[u8; 32], challenger: [u8; 32], anchor: crate::economy::Anchor) -> [u64; 4] {
        let key = bond_key(tier);
        let Some(held) = self.treasury.held_of(&key).cloned() else { return [0; 4] };
        let cut: [u64; 4] = std::array::from_fn(|l| held.mu[l] / 2);
        if cut.iter().all(|m| *m == 0) {
            return cut;
        }
        let bounty: [u64; 4] = std::array::from_fn(|l| cut[l] / 2);
        let retired: [u64; 4] = std::array::from_fn(|l| cut[l] - bounty[l]);
        let kept: [u64; 4] = std::array::from_fn(|l| held.mu[l] - cut[l]);
        if self.treasury.refund_held(&key, anchor).is_err() {
            return [0; 4];
        }
        if bounty.iter().any(|m| *m > 0) {
            let _ = self.treasury.pay(held.payer, challenger, bounty, anchor);
        }
        let _ = self.treasury.burn(held.payer, retired, anchor);
        if kept.iter().any(|m| *m > 0) {
            let _ = self.treasury.hold(held.payer, &key, kept, anchor);
        }
        self.mark_persist();
        cut
    }
}
