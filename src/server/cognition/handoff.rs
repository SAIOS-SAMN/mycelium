// 78.126 — 2(b) MATTER HANDOFFS on the court: an offer held in flight, its
// answer, and each side consumed by exactly one fold (the coordinator's
// rulings 2026-10-07).
//
//! The court holds an offer whole — its nets are what a consuming fold's
//! continuity and a replay read — from its seal until both sides are
//! consumed by folds read final (`settle_windows` lets it go then: a final
//! fold is never reversed, so no side can be un-consumed). The answer window
//! is the law's `W`, the one finality clock.

use super::challenge::window_end;
use super::Kernel;
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
        refs.iter().filter_map(|(id, side)| self.held.get(id).map(|h| side_nets(h, *side))).flatten().collect()
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
        }
    }

    /// The state written by its own coordinates; `None` where none is held.
    pub(crate) fn put(&self) -> Option<Vec<u8>> {
        if self.held.is_empty() && self.consumed_by.is_empty() && self.offered_at.is_empty() {
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
        Some(out)
    }
}

impl Kernel {
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
        let id = h.handoff.id();
        let head = self.chain.head_height();
        let state = match &h.handoff {
            Handoff::Offer(o) => {
                if h.who != o.from_tier {
                    return refuse(R::NotTheTier);
                }
                if let Err(u) = o.conserves() {
                    return refuse(R::Unbalanced(u));
                }
                match (self.tier_domain.get(&o.from_tier), self.tier_domain.get(&o.to_tier)) {
                    (Some(x), Some(y)) if x != y => {}
                    _ => return refuse(R::Unbound),
                }
                if let Some(held) = self.handoffs.held.get(&id) {
                    // A RESEND (the coordinator's ruling): the same offer —
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
                let Some(held) = self.handoffs.held.get(&id) else { return refuse(R::Unknown) };
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
        let kind = match &h.handoff {
            Handoff::Offer(_) => 0,
            Handoff::Accept { .. } => 1,
            Handoff::Refuse { .. } => 2,
            Handoff::Expire { .. } => 3,
        };
        let cite = Cite::Handoff { id, kind };
        let Some(r) = self.seal_step(ActKind::Handoff, [0; 4], h.who, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let now = match (state, &h.handoff) {
            (None, Handoff::Offer(o)) => {
                let until = window_end(r.stamp.h);
                let held = Held { offer: o.as_ref().clone(), sealed: r.stamp.h, state: State::Offered { until }, out: None, inn: None };
                self.handoffs.held.insert(id, held);
                self.handoffs.offered_at.insert(id, r.stamp.key());
                State::Offered { until }
            }
            (Some(s), _) => {
                if let Some(held) = self.handoffs.held.get_mut(&id) {
                    held.state = s;
                }
                s
            }
            (None, _) => return refuse(R::Unknown),
        };
        self.mark_persist();
        Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state: now }
    }

    /// 78.126 — A HANDOFF'S STANDING: its state and whether each side is
    /// consumed. Sealing nothing.
    pub fn handoff_read(&self, id: &[u8; 32]) -> Option<(State, bool, bool)> {
        let h = self.handoffs.held.get(id)?;
        Some((h.state, h.out.is_some(), h.inn.is_some()))
    }

    /// THE SIDES A FOLD WOULD CONSUME, proved: each names a held handoff, is
    /// this fold's tier's, stands answerable, and is not consumed — once in
    /// this fold. Their nets, or the first side refused and why.
    pub(crate) fn sides_of(&self, fold: &TierFold) -> Result<Vec<Net>, (u32, SideRefusal)> {
        let mut nets = Vec::new();
        for (i, (id, side)) in fold.handoffs.iter().enumerate() {
            let index = super::ledger::count(i);
            if fold.handoffs[..i].contains(&(*id, *side)) {
                return Err((index, SideRefusal::Repeated));
            }
            let Some(h) = self.handoffs.held.get(id) else { return Err((index, SideRefusal::Unknown)) };
            let (owner, taken) = match (side, h.state) {
                (Side::Out, _) => (h.offer.from_tier, &h.out),
                (Side::In, State::Accepted) => (h.offer.to_tier, &h.inn),
                (Side::In, State::Returned) => (h.offer.from_tier, &h.inn),
                (Side::In, State::Offered { .. }) => return Err((index, SideRefusal::NotYet)),
            };
            if owner != fold.tier {
                return Err((index, SideRefusal::NotYours));
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
        }
        for k in &spent {
            self.handoffs.consumed_by.remove(k);
        }
        done.len() + spent.len()
    }
}
