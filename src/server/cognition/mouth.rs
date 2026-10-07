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
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Loop, Mouth, SignedHandoff, State};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::transducer::address::{put_key_bytes, take_key_bytes};
use crate::transducer::cech::coefficient::{Coefficient, Turned};
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
        v.extend_from_slice(&self.mouth.bytes());
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

impl Kernel {
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
        let keeper = self.handoffs.mouths.get(&mouth).map(|m| m.mouth.keeper);
        let _ = match (accepted, keeper) {
            (true, Some(k)) => self.treasury.release_held(&key, k, anchor),
            _ => self.treasury.refund_held(&key, anchor),
        };
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
                State::Offered { until }
            }
            (_, 1) => {
                if let Some(held) = self.handoffs.mouths.get_mut(&id) {
                    held.state = State::Accepted;
                    held.opened = Some(r.stamp.h);
                }
                State::Accepted
            }
            (_, 8) => {
                if let Some(held) = self.handoffs.mouths.get_mut(&id) {
                    held.state = State::Returned;
                }
                // Traversals in flight through it return, their tolls back.
                let in_flight: Vec<[u8; 32]> = self
                    .handoffs
                    .through
                    .scoped()
                    .filter(|(_, m)| **m == id)
                    .filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok())
                    .filter(|k| self.handoffs.held.get(k).is_some_and(|t| matches!(t.state, State::Offered { .. })))
                    .collect();
                for t in in_flight {
                    if let Some(held) = self.handoffs.held.get_mut(&t) {
                        held.state = State::Returned;
                    }
                    self.settle_toll(&t, false, anchor);
                }
                State::Returned
            }
            _ => {
                if let Some(held) = self.handoffs.mouths.get_mut(&id) {
                    held.state = State::Returned;
                }
                State::Returned
            }
        };
        self.mark_persist();
        Handed::Sealed { at: r.stamp, receipt: r.chain_hash, id, state }
    }

    /// 78.128 — THE MOUTH LOOPS: the open mouth pairs as edges between
    /// domains, in the order they opened, each carrying its chart transition
    /// `to_frame · from_frame⁻¹`; every pair that closes a cycle of domains
    /// whose holonomy is not the identity, with that holonomy. Read, never
    /// stored (law 12); never a refusal — curvature, recorded.
    pub fn mouth_loops(&self) -> Vec<Loop> {
        // In the order they opened: keyed by the opening height, then the id.
        let mut open: crate::network::nerve::Keyed<([u8; 32], Mouth)> = crate::network::nerve::Keyed::new();
        for (k, m) in self.handoffs.mouths.scoped() {
            let (Some(h), Ok(id)) = (m.opened, <[u8; 32]>::try_from(k.as_slice())) else { continue };
            let mut at = h.to_be_bytes().to_vec();
            at.extend_from_slice(&id);
            open.insert(at, (id, m.mouth.clone()));
        }
        let mut gauge: Gauge<Turned> = Gauge::default();
        let mut loops = Vec::new();
        for (_, (id, m)) in open.scoped() {
            let id = *id;
            let (Some(x), Some(y)) = (self.tier_domain.get(&m.from_tier), self.tier_domain.get(&m.to_tier)) else { continue };
            let transition = Turned { rotor: crate::transducer::alphabet::Rotor::identity(), carriage: m.to_frame.geo(&m.from_frame.reverse()) };
            if let Gauged::Obstructed(h) = gauge.edge(x, y, &transition) {
                if !h.is_identity() {
                    loops.push(Loop { mouth: id, holonomy: h.carriage });
                }
            }
        }
        loops
    }
}
