// 78.126 — 2(b) MATTER HANDOFFS on the court: an offer held in flight, its
// answer, and each side consumed by exactly one fold (ruled
// 2026-10-07).
//
//! The court holds an offer whole — its nets are what a consuming fold's
//! continuity and a replay read — from its seal until both sides are
//! consumed by folds read final (`settle_windows` lets it go then: a final
//! fold is never reversed, so no side can be un-consumed). The answer window
//! is the law's `W`, the one finality clock.

use super::challenge::window_end;
use super::Court;
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Offer, Side, SignedHandoff, State};
use crate::anastome::dialogue::uplink::{Net, SideRefusal, TierFold};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};

/// ONE HANDOFF HELD: its offer, the height it sealed at, where it stands,
/// and the stamp key of the fold that consumed each side, where one has.
#[derive(Clone, Debug)]
pub(crate) struct Held {
    pub(crate) offer: Offer,
    pub(crate) sealed: u64,
    pub(crate) state: State,
    pub(crate) out: Option<Vec<u8>>,
    pub(crate) inn: Option<Vec<u8>>,
}

/// THE HANDOFFS ON THIS COURT — persisted on its node (`LEAF_HANDOFFS`).
#[derive(Default)]
pub(crate) struct Handoffs {
    /// Each handoff by its id.
    pub(crate) held: Keyed<Held>,
    /// Each fold that consumed sides, by its stamp key: the sides — what a
    /// replay reads and a reversal un-consumes.
    pub(crate) consumed_by: Keyed<Vec<([u8; 32], Side)>>,
    /// Each held offer's receipt, by the handoff's id — so a resent offer is
    /// answered with its original receipt and nothing is sealed twice.
    pub(crate) offered_at: Keyed<Vec<u8>>,
    /// 78.127 — each conversion by its id, held while its G side is open.
    pub(crate) conversions: Keyed<super::convert::Conversion>,
    /// 78.128 — each wormhole's mouth pair by its id.
    pub(crate) mouths: Keyed<super::mouth::MouthHeld>,
    /// 78.128 — each traversal's mouth, by the handoff's id.
    pub(crate) through: Keyed<[u8; 32]>,
    /// 78.133 — each tariffed traversal's toll, split and held to its
    /// window's close, by the handoff's id.
    pub(crate) tolls: Keyed<super::mouth::TollHeld>,
    /// 78.135 — each tube's throat bank, by the mouth's id.
    pub(crate) throats: Keyed<super::mouth::Throat>,
}

/// The escrow key a traversal's toll is held under.
pub(crate) fn toll_key(id: &[u8; 32]) -> Vec<u8> {
    let mut k = b"toll".to_vec();
    k.extend_from_slice(id);
    k
}

/// The nets a side carries into its consuming fold: OUT, what leaves; IN
/// accepted, what arrives; IN returned, what left, come back.
fn side_nets(h: &Held, side: Side) -> Vec<Net> {
    match (side, h.state) {
        (Side::Out, _) => h.offer.out_nets.clone(),
        (Side::In, State::Accepted) => h.offer.in_nets.clone(),
        (Side::In, _) => h.offer.out_nets.iter().map(|(who, v)| (*who, v.map(|x| -x))).collect(),
    }
}

impl Handoffs {
    /// The nets a fold's consumed sides carried, as recorded — empty for a
    /// fold that consumed none.
    pub(crate) fn crossing(&self, fold: &[u8]) -> Vec<Net> {
        let Some(refs) = self.consumed_by.get(fold) else { return Vec::new() };
        refs.iter()
            .filter_map(|(id, side)| match (self.held.get(id), self.conversions.get(id)) {
                (Some(h), _) => Some(side_nets(h, *side)),
                (None, Some(c)) => Some(c.nets()),
                (None, None) => None,
            })
            .flatten()
            .collect()
    }

    /// The sides a reversed fold consumed, open again.
    pub(crate) fn unconsume(&mut self, fold: &[u8]) {
        let Some(refs) = self.consumed_by.remove(fold) else { return };
        for (id, side) in refs {
            if let Some(h) = self.held.get_mut(&id) {
                match side {
                    Side::Out => h.out = None,
                    Side::In => h.inn = None,
                }
            }
            if let Some(c) = self.conversions.get_mut(&id) {
                c.side = None;
            }
        }
    }

    /// The state written by its own coordinates; `None` where none is held.
    pub(crate) fn put(&self) -> Option<Vec<u8>> {
        if self.held.is_empty()
            && self.consumed_by.is_empty()
            && self.offered_at.is_empty()
            && self.conversions.is_empty()
            && self.mouths.is_empty()
            && self.through.is_empty()
            && self.tolls.is_empty()
            && self.throats.is_empty()
        {
            return None;
        }
        let mut v = Vec::new();
        put_key_count(&mut v, self.held.len());
        for (id, h) in self.held.scoped() {
            v.extend_from_slice(&id);
            v.extend_from_slice(&h.offer.bytes());
            v.extend_from_slice(&h.sealed.to_le_bytes());
            match h.state {
                State::Offered { until } => {
                    v.push(0);
                    v.extend_from_slice(&until.to_le_bytes());
                }
                State::Accepted => v.push(1),
                State::Returned => v.push(2),
            }
            for side in [&h.out, &h.inn] {
                match side {
                    None => v.push(0),
                    Some(k) => {
                        v.push(1);
                        put_key_bytes(&mut v, k);
                    }
                }
            }
        }
        put_key_count(&mut v, self.consumed_by.len());
        for (k, refs) in self.consumed_by.scoped() {
            put_key_bytes(&mut v, &k);
            put_key_count(&mut v, refs.len());
            for (id, side) in refs {
                v.extend_from_slice(id);
                v.push(side.byte());
            }
        }
        // The offers' receipts, after the sections b1072ba0 wrote.
        put_key_count(&mut v, self.offered_at.len());
        for (id, at) in self.offered_at.scoped() {
            v.extend_from_slice(&id);
            put_key_bytes(&mut v, at);
        }
        // 78.127 — the conversions, after the offers' receipts.
        put_key_count(&mut v, self.conversions.len());
        for (id, c) in self.conversions.scoped() {
            v.extend_from_slice(&id);
            c.put(&mut v);
        }
        // 78.128 — the mouths and the traversals' mouths, after the conversions.
        put_key_count(&mut v, self.mouths.len());
        for (id, m) in self.mouths.scoped() {
            v.extend_from_slice(&id);
            m.put(&mut v);
        }
        put_key_count(&mut v, self.through.len());
        for (id, mouth) in self.through.scoped() {
            v.extend_from_slice(&id);
            v.extend_from_slice(mouth);
        }
        // 78.133 — the mouths' tariffs (beside the mouths' flat writing), then the
        // tolls held, after the traversals.
        let tariffed: Vec<(Vec<u8>, &crate::anastome::dialogue::handoff::Tariff)> =
            self.mouths.scoped().filter_map(|(id, m)| Some((id, m.mouth.tariff.as_ref()?))).collect();
        put_key_count(&mut v, tariffed.len());
        for (id, t) in tariffed {
            v.extend_from_slice(&id);
            v.extend_from_slice(&t.bytes());
        }
        put_key_count(&mut v, self.tolls.len());
        for (id, t) in self.tolls.scoped() {
            v.extend_from_slice(&id);
            t.put(&mut v);
        }
        // 78.134 — the mouths' curved tubes, beside the mouths' flat writing.
        let tubes: Vec<(Vec<u8>, &crate::anastome::dialogue::handoff::Tube)> =
            self.mouths.scoped().filter_map(|(id, m)| Some((id, m.mouth.tube.as_ref()?))).collect();
        let (curved, spaced): (Vec<_>, Vec<_>) = tubes.iter().partition(|(_, t)| t.curved());
        put_key_count(&mut v, curved.len());
        for (id, t) in curved {
            v.extend_from_slice(id);
            v.extend_from_slice(&t.bytes());
        }
        // 78.135 — the throats' banks, after the tubes.
        put_key_count(&mut v, self.throats.len());
        for (id, t) in self.throats.scoped() {
            v.extend_from_slice(&id);
            t.put(&mut v);
        }
        // 78.140 — the tubes with a torus end, then the core tubes by id.
        put_key_count(&mut v, spaced.len());
        for (id, t) in spaced {
            v.extend_from_slice(id);
            v.extend_from_slice(&t.bytes());
        }
        let cores: Vec<&Vec<u8>> = tubes.iter().filter(|(_, t)| t.core).map(|(id, _)| id).collect();
        put_key_count(&mut v, cores.len());
        for id in cores {
            v.extend_from_slice(id);
        }
        Some(v)
    }

    /// The state read back whole.
    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Handoffs> {
        let id = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        };
        let u64_at = |b: &[u8], p: &mut usize| -> Option<u64> {
            let x = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some(x)
        };
        let mut out = Handoffs::default();
        for _ in 0..take_key_count(b, p)? {
            let key = id(b, p)?;
            let offer = Offer::take(b, p)?;
            let sealed = u64_at(b, p)?;
            let tag = *b.get(*p)?;
            *p += 1;
            let state = match tag {
                0 => State::Offered { until: u64_at(b, p)? },
                1 => State::Accepted,
                2 => State::Returned,
                _ => return None,
            };
            let mut side = || -> Option<Option<Vec<u8>>> {
                let flag = *b.get(*p)?;
                *p += 1;
                match flag {
                    0 => Some(None),
                    1 => Some(Some(take_key_bytes(b, p)?)),
                    _ => None,
                }
            };
            let (o, i) = (side()?, side()?);
            out.held.insert(key, Held { offer, sealed, state, out: o, inn: i });
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            let refs = (0..take_key_count(b, p)?)
                .map(|_| {
                    let h = id(b, p)?;
                    let side = Side::of(*b.get(*p)?)?;
                    *p += 1;
                    Some((h, side))
                })
                .collect::<Option<Vec<_>>>()?;
            out.consumed_by.insert(&k, refs);
        }
        // The offers' receipts — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                let at = take_key_bytes(b, p)?;
                out.offered_at.insert(k, at);
            }
        }
        // The conversions — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.conversions.insert(k, super::convert::Conversion::take(b, p)?);
            }
        }
        // The mouths and traversals — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.mouths.insert(k, super::mouth::MouthHeld::take(b, p)?);
            }
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.through.insert(k, id(b, p)?);
            }
        }
        // 78.133 — the tariffs and the tolls held — absent from a leaf
        // written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                let tariff = crate::anastome::dialogue::handoff::Tariff::take(b, p)?;
                out.mouths.get_mut(&k)?.mouth.tariff = Some(tariff);
            }
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.tolls.insert(k, super::mouth::TollHeld::take(b, p)?);
            }
        }
        // 78.134 — the tubes — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                let tube = crate::anastome::dialogue::handoff::Tube::take(b, p, false)?;
                out.mouths.get_mut(&k)?.mouth.tube = Some(tube);
            }
        }
        // 78.135 — the throats' banks — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.throats.insert(k, super::mouth::Throat::take(b, p)?);
            }
        }
        // 78.140 — the tubes with a torus end and the core tubes — absent
        // from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                let tube = crate::anastome::dialogue::handoff::Tube::take(b, p, true)?;
                out.mouths.get_mut(&k)?.mouth.tube = Some(tube);
            }
            for _ in 0..take_key_count(b, p)? {
                let k = id(b, p)?;
                out.mouths.get_mut(&k)?.mouth.tube.as_mut()?.core = true;
            }
        }
        Some(out)
    }
}

impl Court {
    /// 78.126 — A HANDOFF ACT TAKEN: the filer's signature, then the offer
    /// (sender; conserving, on S and H only; both tiers bound, to different
    /// domains), the acceptance or refusal (receiver, within the window), or
    /// the expiry (sender, past it) — each sealed by `Cite::Handoff`.
    pub fn handoff(&mut self, h: &SignedHandoff) -> Handed {
        use HandoffRefusal as R;
        let refuse = Handed::Refused;
        if !crate::court::identity::verify(&h.who, &h.handoff.signed_bytes(&h.who), &h.sig) {
            return refuse(R::Unsigned);
        }
        let id = h.handoff.citation();
        // 78.127 — a conversion, or an answer to one, is the convert path.
        let answers_conversion = matches!(h.handoff, Handoff::Accept { .. } | Handoff::Refuse { .. } | Handoff::Expire { .. })
            && self.handoffs.conversions.contains(&id);
        if matches!(h.handoff, Handoff::ConvertOut(_) | Handoff::ConvertIn(_)) || answers_conversion {
            return self.convert(h);
        }
        // 78.128 — a mouth pair, its close, or an answer to its offer.
        let answers_mouth = matches!(h.handoff, Handoff::Accept { .. } | Handoff::Refuse { .. } | Handoff::Expire { .. })
            && self.handoffs.mouths.contains(&id);
        if matches!(h.handoff, Handoff::Mouth(_) | Handoff::Close { .. }) || answers_mouth {
            return self.mouth_act(h);
        }
        // 78.135 — a throat fed or drawn.
        if matches!(h.handoff, Handoff::Feed { .. } | Handoff::Draw { .. }) {
            return self.bank_act(h);
        }
        let head = self.chain.head_height();
        let state = match &h.handoff {
            Handoff::ConvertOut(_)
            | Handoff::ConvertIn(_)
            | Handoff::Mouth(_)
            | Handoff::Close { .. }
            | Handoff::Feed { .. }
            | Handoff::Draw { .. } => return refuse(R::UnknownId),
            Handoff::Offer(o) | Handoff::Traverse { offer: o, .. } => {
                if h.who != o.from_tier {
                    return refuse(R::NotTheTier);
                }
                if let Err(u) = o.conserves() {
                    return refuse(R::Unbalanced(u));
                }
                let (Some(x), Some(y)) = (self.tier_domain.get(&o.from_tier).copied(), self.tier_domain.get(&o.to_tier).copied()) else {
                    return refuse(R::Unbound);
                };
                // 78.136 — within one domain only along a ROAD: a traversal
                // through a mouth joining two places of it.
                if x == y && !matches!(h.handoff, Handoff::Traverse { .. }) {
                    return refuse(R::Unbound);
                }
                // 78.128 — DOMAINS SHARE NO SPACE: nothing crosses between
                // them but through an OPEN mouth pair joining these two, in
                // either direction (ruled: physics, not
                // policy) — unless the two are declared contiguous (a later
                // act; none is).
                match &h.handoff {
                    Handoff::Traverse { mouth, .. } => {
                        if let Err(why) = self.through(mouth, &x, &y) {
                            return refuse(why);
                        }
                    }
                    _ if self.contiguous(&x, &y) => {}
                    _ => return refuse(R::NoMouth),
                }
                if let Some(held) = self.handoffs.held.get(&id) {
                    // A RESEND (as ruled): the same offer —
                    // its id is its content — answered with its original
                    // receipt and its state now; nothing sealed twice.
                    let original = self.handoffs.offered_at.get(&id).and_then(|k| Stamp::from_key(k)).and_then(|at| Some((at, self.chain.node(&at)?)));
                    return match original {
                        Some((at, node)) => Handed::Sealed { at, receipt: node.chain_hash, id, state: held.state },
                        None => refuse(R::Offered),
                    };
                }
                None
            }
            Handoff::Accept { .. } | Handoff::Refuse { .. } | Handoff::Expire { .. } => {
                let Some(held) = self.handoffs.held.get(&id) else { return refuse(R::UnknownId) };
                let State::Offered { until } = held.state else { return refuse(R::Answered) };
                let expiring = matches!(h.handoff, Handoff::Expire { .. });
                let filer = if expiring { held.offer.from_tier } else { held.offer.to_tier };
                if h.who != filer {
                    return refuse(R::NotTheTier);
                }
                // Answered within the window; expired only past it.
                if expiring != (head >= until) {
                    return refuse(R::Window);
                }
                Some(match h.handoff {
                    Handoff::Accept { .. } => State::Accepted,
                    _ => State::Returned,
                })
            }
        };
        // 78.128 — a traversal's toll is escrowed from the sending tier before
        // anything seals: the payer must hold it.
        // 78.133 — a tariffed mouth charges the passage per unit: mass on S,
        // information on H, floored; a flat mouth its flat toll.
        let toll = match &h.handoff {
            Handoff::Traverse { mouth, offer } => match self.handoffs.mouths.get(mouth).map(|m| &m.mouth) {
                Some(m) => match &m.tariff {
                    Some(t) => match t.charge(offer) {
                        Some(c) => Some(c),
                        None => return refuse(R::Untariffed(crate::anastome::dialogue::handoff::Untariffed::ZeroDenominator)),
                    },
                    None => Some(m.toll),
                },
                None => None,
            },
            _ => None,
        };
        if let (Some(toll), Handoff::Traverse { offer, .. }) = (toll, &h.handoff) {
            let short = crate::economy::shortfall(&toll, &self.treasury.balance_mu(&offer.from_tier));
            if !crate::economy::lanes_short(&short).is_empty() {
                return refuse(R::HolderShort);
            }
        }
        let kind = match &h.handoff {
            Handoff::Offer(_) => 0,
            Handoff::Accept { .. } => 1,
            Handoff::Refuse { .. } => 2,
            Handoff::Expire { .. } => 3,
            Handoff::Traverse { .. } => 9,
            Handoff::ConvertOut(_)
            | Handoff::ConvertIn(_)
            | Handoff::Mouth(_)
            | Handoff::Close { .. }
            | Handoff::Feed { .. }
            | Handoff::Draw { .. } => return refuse(R::UnknownId),
        };
        let cite = Cite::Handoff { id, kind };
        let Some(r) = self.seal_step(ActKind::Handoff, [0; 4], h.who, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let anchor = (r.stamp, r.chain_hash);
        let now = match (state, &h.handoff) {
            (None, Handoff::Offer(o) | Handoff::Traverse { offer: o, .. }) => {
                let until = window_end(r.stamp.h);
                let held = Held { offer: o.as_ref().clone(), sealed: r.stamp.h, state: State::Offered { until }, out: None, inn: None };
                self.handoffs.held.insert(id, held);
                self.handoffs.offered_at.insert(id, r.stamp.key());
                if let Handoff::Traverse { mouth, .. } = &h.handoff {
                    self.handoffs.through.insert(id, *mouth);
                    if let Some(toll) = toll.filter(|t| t.iter().any(|m| *m > 0)) {
                        let _ = self.treasury.hold(o.from_tier, &toll_key(&id), toll, anchor);
                    }
                }
                State::Offered { until }
            }
            (Some(s), _) => {
                if let Some(held) = self.handoffs.held.get_mut(&id) {
                    held.state = s;
                }
                // 78.128 — the toll: to the keeper on acceptance, back to the
                // payer otherwise.
                self.settle_toll(&id, s == State::Accepted, anchor);
                s
            }
            (None, _) => return refuse(R::UnknownId),
        };
        self.mark_persist();
        Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state: now }
    }

    /// 78.126 — A HANDOFF'S STANDING: its state and whether each side is
    /// consumed. Sealing nothing.
    pub fn handoff_read(&self, id: &[u8; 32]) -> Option<(State, bool, bool)> {
        if let Some(c) = self.handoffs.conversions.get(id) {
            let taken = c.side.is_some();
            return Some(match c.side_kind() {
                Side::Out => (c.state, taken, false),
                Side::In => (c.state, false, taken),
            });
        }
        let h = self.handoffs.held.get(id)?;
        Some((h.state, h.out.is_some(), h.inn.is_some()))
    }

    /// THE SIDES A FOLD WOULD CONSUME, proved: each names a held handoff, is
    /// this fold's tier's, stands answerable, and is not consumed — once in
    /// this fold. The sides' nets, or the first side refused and why.
    pub(crate) fn sides_of(&self, fold: &TierFold) -> Result<Vec<Net>, (u32, SideRefusal)> {
        let mut nets = Vec::new();
        for (i, (id, side)) in fold.handoffs.iter().enumerate() {
            let index = super::ledger::count(i);
            if fold.handoffs[..i].contains(&(*id, *side)) {
                return Err((index, SideRefusal::Repeated));
            }
            // 78.127 — a conversion's G side: the reserve tier's, OUT for μ
            // paid out, IN for μ accepted in — never while an offer in stands.
            if let Some(c) = self.handoffs.conversions.get(id) {
                if c.tier != fold.tier || c.side_kind() != *side {
                    return Err((index, SideRefusal::NotThisTier));
                }
                if c.state != State::Accepted {
                    return Err((index, SideRefusal::NotYet));
                }
                if c.side.is_some() {
                    return Err((index, SideRefusal::Consumed));
                }
                nets.extend(c.nets());
                continue;
            }
            let Some(h) = self.handoffs.held.get(id) else { return Err((index, SideRefusal::UnknownHandoff)) };
            let (owner, taken) = match (side, h.state) {
                (Side::Out, _) => (h.offer.from_tier, &h.out),
                (Side::In, State::Accepted) => (h.offer.to_tier, &h.inn),
                (Side::In, State::Returned) => (h.offer.from_tier, &h.inn),
                (Side::In, State::Offered { .. }) => return Err((index, SideRefusal::NotYet)),
            };
            if owner != fold.tier {
                return Err((index, SideRefusal::NotThisTier));
            }
            if taken.is_some() {
                return Err((index, SideRefusal::Consumed));
            }
            nets.extend(side_nets(h, *side));
        }
        Ok(nets)
    }

    /// The sides a sealed fold consumed, marked — each by the fold's stamp
    /// key, recorded for its replay and its reversal.
    pub(crate) fn consume_sides(&mut self, fold: &[u8], refs: &[([u8; 32], Side)]) {
        if refs.is_empty() {
            return;
        }
        for (id, side) in refs {
            if let Some(h) = self.handoffs.held.get_mut(id) {
                match side {
                    Side::Out => h.out = Some(fold.to_vec()),
                    Side::In => h.inn = Some(fold.to_vec()),
                }
            }
            if let Some(c) = self.handoffs.conversions.get_mut(id) {
                c.side = Some(fold.to_vec());
            }
        }
        self.handoffs.consumed_by.insert(fold, refs.to_vec());
    }

    /// 78.126 — THE HANDOFFS SETTLED with the windows: a handoff both of
    /// whose sides were consumed by folds no longer recorded (final, never
    /// reversed) is let go, and so is what those folds consumed. Entries let
    /// go.
    pub(crate) fn settle_handoffs(&mut self) -> usize {
        let records = &self.ledgers.records;
        let settled = |k: &Option<Vec<u8>>| k.as_ref().is_some_and(|f| !records.contains(f));
        let done: Vec<Vec<u8>> = self.handoffs.held.scoped().filter(|(_, h)| settled(&h.out) && settled(&h.inn)).map(|(k, _)| k).collect();
        let spent: Vec<Vec<u8>> = self.handoffs.consumed_by.scoped().map(|(k, _)| k).filter(|k| !records.contains(k)).collect();
        for k in &done {
            self.handoffs.held.remove(k);
            self.handoffs.offered_at.remove(k);
            self.handoffs.through.remove(k);
        }
        // 78.127 — a conversion whose side a final fold consumed, or returned.
        let converted: Vec<Vec<u8>> = self
            .handoffs
            .conversions
            .scoped()
            .filter(|(_, c)| settled(&c.side) || c.state == State::Returned)
            .map(|(k, _)| k)
            .collect();
        for k in &spent {
            self.handoffs.consumed_by.remove(k);
        }
        for k in &converted {
            self.handoffs.conversions.remove(k);
        }
        done.len() + spent.len() + converted.len()
    }
}
