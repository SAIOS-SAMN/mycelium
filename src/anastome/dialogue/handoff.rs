// 78.126 — ACT_HANDOFF: matter, or a delivered word, crossing from one
// domain to another on one court (2(b), the coordinator's rulings
// 2026-10-07; 78.59 P2: matter does not net like money).
//
//! A unit of matter stands in exactly one place, so a crossing between
//! domains is no rate on an edge: it is a HANDOFF one court serialises, two
//! sides each consumed exactly once.
//!
//! - OFFER, signed by the sending tier: the nets leaving its domain (its
//!   identities, negative) and arriving in the other (the receiver's
//!   identities, positive), summing to zero on every lane, on S (matter)
//!   and H (a delivered word) only — G crosses by rates and μ, never by
//!   handoff, and T is reserved for contracts. Sealed, the matter is IN
//!   FLIGHT. The offer carries the sender's charter commitment (zero until
//!   charters exist), signed, unchecked.
//! - ACCEPT, by the receiving tier, whole or not at all, within `W` of the
//!   offer; REFUSE by the receiver; EXPIRY claimed by the sender past `W`.
//!   Refused or expired, the handoff is RETURNED.
//! - A fold consumes a SIDE: OUT by the sender, in any state after the
//!   offer; IN by the receiver once accepted, or by the sender once
//!   returned. Each side once; a second is a double-spend, refused.

use super::uplink::{put_nets, take_nets, Net};
use crate::chronometry::Stamp;

/// The signed bytes' domain.
const SIGNED: &[u8] = b"mycelium handoff v1";

/// THE LANES A HANDOFF MAY CARRY: S (matter) and H (a delivered word).
pub const CARRIED: [bool; 4] = [true, false, false, true];

/// WHICH SIDE OF A HANDOFF A FOLD CONSUMES: OUT — the sender's, its nets
/// leaving; IN — the receiver's once accepted (its nets arriving), or the
/// sender's once returned (what left, come back).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Out,
    In,
}

impl Side {
    pub fn byte(self) -> u8 {
        match self {
            Side::Out => 0,
            Side::In => 1,
        }
    }

    pub fn of(b: u8) -> Option<Side> {
        match b {
            0 => Some(Side::Out),
            1 => Some(Side::In),
            _ => None,
        }
    }
}

/// A HANDOFF OFFERED: who sends, who receives, what leaves and what arrives,
/// and the sender's charter commitment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Offer {
    pub from_tier: [u8; 32],
    pub to_tier: [u8; 32],
    pub out_nets: Vec<Net>,
    pub in_nets: Vec<Net>,
    pub charter: [u8; 32],
    /// The sender's nonce, so two like offers are two handoffs.
    pub nonce: [u8; 32],
}

/// WHY AN OFFER IS NOT A HANDOFF.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unbalanced {
    /// A list not in its identities' strictly ascending order.
    Unordered,
    /// A lane a handoff does not carry is nonzero (G or T).
    Lane { lane: usize },
    /// Out and in do not sum to zero on this lane: matter made or lost.
    Unconserved { lane: usize },
    /// Out holds a positive net or in a negative one: a side running backward.
    Backward { lane: usize },
    /// A sum passed its carrier.
    Overflow { lane: usize },
}

fn ascending(nets: &[Net]) -> bool {
    nets.windows(2).all(|w| matches!(w, [a, b] if a.0 < b.0))
}

impl Offer {
    /// The offer's one writing.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&self.from_tier);
        out.extend_from_slice(&self.to_tier);
        put_nets(&mut out, &self.out_nets);
        put_nets(&mut out, &self.in_nets);
        out.extend_from_slice(&self.charter);
        out.extend_from_slice(&self.nonce);
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Offer> {
        let key = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        };
        let (from_tier, to_tier) = (key(b, p)?, key(b, p)?);
        let (out_nets, in_nets) = (take_nets(b, p)?, take_nets(b, p)?);
        Some(Offer { from_tier, to_tier, out_nets, in_nets, charter: key(b, p)?, nonce: key(b, p)? })
    }

    /// THE HANDOFF'S ID: the hash of its one writing.
    pub fn id(&self) -> [u8; 32] {
        crate::chronometry::payload_hash(&self.bytes())
    }

    /// CONSERVATION across the boundary: each list ascending, only S and H
    /// carried, out running out and in running in, and on every lane out
    /// and in summing to exactly zero.
    pub fn conserves(&self) -> Result<(), Unbalanced> {
        if !ascending(&self.out_nets) || !ascending(&self.in_nets) {
            return Err(Unbalanced::Unordered);
        }
        let mut sum = [0i128; 4];
        for (nets, sign) in [(&self.out_nets, -1i128), (&self.in_nets, 1)] {
            for (_, v) in nets {
                for (l, x) in v.iter().enumerate() {
                    if !CARRIED[l] && *x != 0 {
                        return Err(Unbalanced::Lane { lane: l });
                    }
                    if x.signum() == -sign {
                        return Err(Unbalanced::Backward { lane: l });
                    }
                    sum[l] = sum[l].checked_add(*x).ok_or(Unbalanced::Overflow { lane: l })?;
                }
            }
        }
        match (0..4).find(|&l| sum[l] != 0) {
            Some(lane) => Err(Unbalanced::Unconserved { lane }),
            None => Ok(()),
        }
    }
}

/// WHAT IS ASKED OF A HANDOFF.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Handoff {
    Offer(Box<Offer>),
    /// The receiver takes it whole.
    Accept { id: [u8; 32] },
    /// The receiver refuses it: returned.
    Refuse { id: [u8; 32] },
    /// The sender claims it returned, its window passed unanswered.
    Expire { id: [u8; 32] },
    /// 78.127 — μ OUT of a domain's reserve, against game value leaving.
    ConvertOut(Box<ConvertOut>),
    /// 78.127 — μ INTO a domain's reserve, for game value arriving: offered
    /// by the holder, answered by the reserve tier (accept, refuse, expire).
    ConvertIn(Box<ConvertIn>),
    /// 78.128 — A WORMHOLE'S MOUTHS OFFERED by one domain's reserve to
    /// another's; accepted, the mouth pair is OPEN.
    Mouth(Box<Mouth>),
    /// 78.128 — a mouth pair CLOSED by either reserve; traversals in flight
    /// return.
    Close { id: [u8; 32] },
    /// 78.128 — a handoff THROUGH an open mouth pair: the only way matter or
    /// a word crosses between domains (universes with no shared space).
    Traverse { offer: Box<Offer>, mouth: [u8; 32] },
    /// 78.135 — the throat FED by its keeper: `mu_s` more μ-S banked, its
    /// mass grown by as much, its horizon re-derived wider. `seq` is the
    /// throat's count of bank acts so far, so a resend is never a second
    /// feed.
    Feed { mouth: [u8; 32], mu_s: u64, seq: u64 },
    /// 78.135 — the throat DRAWN DOWN by its keeper: `mu_s` of its bank
    /// returned to the faces that fed it, in proportion, its mass shrunk by
    /// as much — never to zero (closing is `Close`).
    Draw { mouth: [u8; 32], mu_s: u64, seq: u64 },
}

/// 78.128 — A WORMHOLE'S MOUTH PAIR (the coordinator's rulings 2026-10-07):
/// built by two domains' RESERVE tiers (the ancients), `from_tier` offering
/// and `to_tier` accepting; X's black-hole mouth and Y's white-hole mouth,
/// each placed in its domain's chart by an exact Cl(4,1) versor (a horizon
/// sphere carried onto a sphere); the KEEPER — one of the two — takes a flat
/// `toll` of μ per traversal. Two-way, like a lane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mouth {
    pub from_tier: [u8; 32],
    pub to_tier: [u8; 32],
    pub keeper: [u8; 32],
    pub from_frame: crate::transducer::cga::Mv,
    pub to_frame: crate::transducer::cga::Mv,
    pub toll: [u64; 4],
    pub nonce: [u8; 32],
    /// 78.133 — the per-unit toll and its split; `None` for a flat-toll
    /// mouth (78.128), whose toll is all the keeper's.
    pub tariff: Option<Tariff>,
    /// 78.134 — the TUBE this mouth pair is: its frames in `Spin⁺(3,1)`, its
    /// throat sized by its mass. `None` for a mouth offered before tubes —
    /// UNTETHERED, two free frames, kept as history.
    pub tube: Option<Tube>,
}

/// 78.134 — A WORMHOLE AS A TUBE between two places on curved space (the
/// coordinator's rulings, the operator's anti-flattening rule): each mouth
/// stands at its frame's base point on its domain's `ℍ³`, and its horizon is
/// the level set `{p : p·v = −c}` there. The throat's size is its MASS `m`
/// (units are mass), and `c = cosh(2m/R)` is DERIVED per end in that end's
/// space's curvature radius `R` — the upper bound of the certified
/// enclosure on a fixed grid (`sdk::physics::throat_level`), never stored
/// as the truth. One throat: one mass at both ends; the levels differ only
/// where the two spaces' curvatures do. A tube may SUPERSEDE an untethered
/// mouth, citing it; the old record stays.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tube {
    pub mass: (u64, u64),
    pub from_r: (u64, u64),
    pub to_r: (u64, u64),
    pub supersedes: Option<[u8; 32]>,
}

impl Tube {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (n, d) in [self.mass, self.from_r, self.to_r] {
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
        match &self.supersedes {
            None => out.push(0),
            Some(id) => {
                out.push(1);
                out.extend_from_slice(id);
            }
        }
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Tube> {
        let mut q = || -> Option<(u64, u64)> { Some((u64_at(b, p)?, u64_at(b, p)?)) };
        let (mass, from_r, to_r) = (q()?, q()?, q()?);
        let flag = *b.get(*p)?;
        *p += 1;
        let supersedes = match flag {
            0 => None,
            1 => Some(key32(b, p)?),
            _ => return None,
        };
        Some(Tube { mass, from_r, to_r, supersedes })
    }

    /// Mass and both curvature radii positive exact rationals.
    pub fn holds(&self) -> bool {
        [self.mass, self.from_r, self.to_r].iter().all(|(n, d)| *n > 0 && *d > 0)
    }
}

/// 78.134 — WHETHER A FRAME IS A MOTION OF CURVED SPACE: an element of
/// `Spin⁺(3,1)`, the isometries of `ℍ³` inside `Cl(4,1)` — even, a versor,
/// commuting with `e₊` (the axis that makes the sheet the sheet), and
/// orthochronous (it carries the pole onto the UPPER sheet).
pub fn is_spin_plus_31(f: &crate::transducer::cga::Mv) -> bool {
    use crate::transducer::cga::{is_versor, Mv, EM, EP};
    use num_traits::Signed;
    if f.is_zero() || !is_versor(f) {
        return false;
    }
    let even = [1u32, 3, 5].iter().all(|g| f.grade(*g).is_zero());
    let ep = Mv::blade(EP, num_rational::BigRational::from_integer(1.into()));
    let fixes_axis = f.geo(&ep).sub(&ep.geo(f)).is_zero();
    let upper = crate::transducer::cga::frame_base_point(f).is_some_and(|v| v.coeff(EM).is_positive());
    even && fixes_axis && upper
}

/// 78.133 — A MOUTH'S TARIFF (the coordinator's rulings): a wormhole carries
/// S and H only, so each pays on its own lane — `rate_s` μ-S per unit of
/// MASS leaving (`Σ|out S|`), `rate_h` μ-H per unit of INFORMATION leaving
/// (`Σ|out H|`), each an exact rational `(n, d)`; G (value, the one
/// translated lane) and T (duration) are never charged by a mouth. The SPLIT
/// — `(recipient, share)` in the recipients' ascending order, each share an
/// exact rational, summing to exactly one — divides each lane's charge alike.
/// Part of the mouth's id: a new split is a new mouth, never an edit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tariff {
    pub rate_s: (u64, u64),
    pub rate_h: (u64, u64),
    pub split: Vec<([u8; 32], (u64, u64))>,
}

/// WHY A TARIFF IS NOT ONE.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Untariffed {
    /// A rate or a share with a zero denominator.
    ZeroDenominator,
    /// The recipients not strictly ascending: each once, in one writing.
    Unordered,
    /// The shares do not sum to exactly one.
    NotWhole,
    /// A flat toll beside the tariff: a mouth's toll is one law.
    Flat,
}

impl Tariff {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (n, d) in [self.rate_s, self.rate_h] {
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
        crate::transducer::address::put_key_count(&mut out, self.split.len());
        for (who, (n, d)) in &self.split {
            out.extend_from_slice(who);
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Tariff> {
        let rate_s = (u64_at(b, p)?, u64_at(b, p)?);
        let rate_h = (u64_at(b, p)?, u64_at(b, p)?);
        let n = crate::transducer::address::take_key_count(b, p)?;
        let split = (0..n).map(|_| Some((key32(b, p)?, (u64_at(b, p)?, u64_at(b, p)?)))).collect::<Option<Vec<_>>>()?;
        Some(Tariff { rate_s, rate_h, split })
    }

    /// The tariff well formed: every denominator nonzero, the recipients
    /// strictly ascending, the shares summing to exactly one.
    pub fn holds(&self) -> Result<(), Untariffed> {
        use num_rational::BigRational as Q;
        use num_traits::{One, Zero};
        let rates = [self.rate_s, self.rate_h];
        let fractions = rates.iter().chain(self.split.iter().map(|(_, s)| s));
        if fractions.into_iter().any(|(_, d)| *d == 0) {
            return Err(Untariffed::ZeroDenominator);
        }
        if !self.split.windows(2).all(|w| matches!(w, [a, b] if a.0 < b.0)) {
            return Err(Untariffed::Unordered);
        }
        let sum = self.split.iter().fold(Q::zero(), |s, (_, (n, d))| s + Q::new((*n).into(), (*d).into()));
        match sum == Q::one() {
            true => Ok(()),
            false => Err(Untariffed::NotWhole),
        }
    }

    /// THE CHARGE for a passage: on S `⌊rate_s · mass⌋`, on H `⌊rate_h ·
    /// information⌋`, G and T never — the remainder never charged, never
    /// rounded up against the traveller. `None` where a denominator is zero
    /// or a lane passes its carrier.
    pub fn charge(&self, offer: &Offer) -> Option<[u64; 4]> {
        let floor = |(n, d): (u64, u64), units: u64| -> Option<u64> {
            if d == 0 {
                return None;
            }
            u64::try_from(u128::from(n).checked_mul(u128::from(units))? / u128::from(d)).ok()
        };
        Some([floor(self.rate_s, leaving(offer, 0)?)?, 0, 0, floor(self.rate_h, leaving(offer, 3)?)?])
    }

    /// THE PARTS of a `charge`: each recipient but the `keeper` its `⌊share ·
    /// charge⌋` per lane, the keeper the rest — so the parts sum to the
    /// charge exactly, no dust. In the split's order, the keeper's part last;
    /// a recipient whose part is zero on every lane is left out. `None` where
    /// a share's floors pass the charge (shares not summing to one).
    pub fn parts(&self, keeper: &[u8; 32], charge: &[u64; 4]) -> Option<Vec<([u8; 32], [u64; 4])>> {
        let mut rest = *charge;
        let mut out = Vec::new();
        for (who, (n, d)) in &self.split {
            if who == keeper {
                continue;
            }
            let mut part = [0u64; 4];
            for ((p, c), r) in part.iter_mut().zip(charge).zip(rest.iter_mut()) {
                if *d == 0 {
                    return None;
                }
                *p = u64::try_from(u128::from(*n).checked_mul(u128::from(*c))? / u128::from(*d)).ok()?;
                *r = r.checked_sub(*p)?;
            }
            if part.iter().any(|m| *m > 0) {
                out.push((*who, part));
            }
        }
        if rest.iter().any(|m| *m > 0) {
            out.push((*keeper, rest));
        }
        Some(out)
    }
}

/// WHAT A PASSAGE CARRIES OUT on `lane`: `Σ|out|` — mass on S, information
/// on H. `None` where it passes its carrier or an out net runs backward.
pub fn leaving(offer: &Offer, lane: usize) -> Option<u64> {
    offer.out_nets.iter().try_fold(0u64, |s, (_, v)| s.checked_add(u64::try_from(v.get(lane)?.checked_neg()?).ok()?))
}

impl Mouth {
    /// The flat-toll mouth's one writing (78.128), what a held mouth is
    /// stored by; its tariff, where it has one, is stored beside it.
    pub fn legacy_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for k in [&self.from_tier, &self.to_tier, &self.keeper] {
            out.extend_from_slice(k);
        }
        self.from_frame.to_bytes(&mut out);
        self.to_frame.to_bytes(&mut out);
        for m in self.toll {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&self.nonce);
        out
    }

    /// The mouth's one writing: the flat writing, then its tariff where it
    /// has one — so a flat mouth's id is what it was.
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = self.legacy_bytes();
        if let Some(t) = &self.tariff {
            out.extend_from_slice(&t.bytes());
        }
        // 78.134 — a tube's writing after, under its own tag, so no flat or
        // tariffed mouth's id moves.
        if let Some(t) = &self.tube {
            out.extend_from_slice(b"tube");
            out.extend_from_slice(&t.bytes());
        }
        out
    }

    /// 78.134 — a tube's frames both motions of curved space.
    pub fn frames_curved(&self) -> bool {
        is_spin_plus_31(&self.from_frame) && is_spin_plus_31(&self.to_frame)
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Mouth> {
        let (from_tier, to_tier, keeper) = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
        let from_frame = crate::transducer::cga::Mv::from_bytes(b, p)?;
        let to_frame = crate::transducer::cga::Mv::from_bytes(b, p)?;
        let mut toll = [0u64; 4];
        for m in &mut toll {
            *m = u64_at(b, p)?;
        }
        Some(Mouth { from_tier, to_tier, keeper, from_frame, to_frame, toll, nonce: key32(b, p)?, tariff: None, tube: None })
    }

    pub fn id(&self) -> [u8; 32] {
        let mut b = b"mouth".to_vec();
        b.extend_from_slice(&self.bytes());
        crate::chronometry::payload_hash(&b)
    }

    /// Each frame a versor, never zero: it carries a horizon sphere onto a
    /// sphere.
    pub fn frames_hold(&self) -> bool {
        [&self.from_frame, &self.to_frame].iter().all(|f| !f.is_zero() && crate::transducer::cga::is_versor(f))
    }
}

/// 78.127 — A CONVERSION OUT (2(e), the coordinator's rulings): signed by
/// the domain's RESERVE tier, the game value `g` (on G) its in-game holder
/// `identity` gives up, and the μ it receives on the court — `rate[l] · g` on
/// every μ lane, exact and integral — paid from the reserve to the holder's
/// key `holder`. A rate never mints μ: the reserve must hold it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertOut {
    pub tier: [u8; 32],
    pub holder: [u8; 32],
    pub identity: [u8; 32],
    pub g: i128,
    pub rate: [(u64, u64); 4],
    pub nonce: [u8; 32],
}

/// 78.127 — A CONVERSION IN, offered by the μ holder: its `mu`, held from its
/// balance until the reserve tier answers, for the game value `g` its
/// in-game `identity` receives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConvertIn {
    pub holder: [u8; 32],
    pub tier: [u8; 32],
    pub identity: [u8; 32],
    pub g: i128,
    pub mu: [u64; 4],
    pub nonce: [u8; 32],
}

fn key32(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(k)
}

fn u64_at(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn i128_at(b: &[u8], p: &mut usize) -> Option<i128> {
    let v = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
    *p += 16;
    Some(v)
}

impl ConvertOut {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for k in [&self.tier, &self.holder, &self.identity] {
            out.extend_from_slice(k);
        }
        out.extend_from_slice(&self.g.to_le_bytes());
        for (n, d) in self.rate {
            out.extend_from_slice(&n.to_le_bytes());
            out.extend_from_slice(&d.to_le_bytes());
        }
        out.extend_from_slice(&self.nonce);
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<ConvertOut> {
        let (tier, holder, identity) = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
        let g = i128_at(b, p)?;
        let mut rate = [(0u64, 1u64); 4];
        for r in &mut rate {
            *r = (u64_at(b, p)?, u64_at(b, p)?);
        }
        Some(ConvertOut { tier, holder, identity, g, rate, nonce: key32(b, p)? })
    }

    /// Its id: the hash of its one writing, under its own domain.
    pub fn id(&self) -> [u8; 32] {
        let mut b = b"convert out".to_vec();
        b.extend_from_slice(&self.bytes());
        crate::chronometry::payload_hash(&b)
    }

    /// The μ it pays on each lane, `rate · g` — `None` where a rate has a
    /// zero denominator, `g` is not positive, or a lane is not an exact
    /// integer that a lane carries.
    pub fn mu(&self) -> Option<[u64; 4]> {
        let g = u128::try_from(self.g).ok().filter(|g| *g > 0)?;
        let mut mu = [0u64; 4];
        for (m, (n, d)) in mu.iter_mut().zip(self.rate) {
            let num = u128::from(n).checked_mul(g)?;
            let d = u128::from(d);
            if d == 0 || num % d != 0 {
                return None;
            }
            *m = u64::try_from(num / d).ok()?;
        }
        Some(mu)
    }
}

impl ConvertIn {
    pub fn bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for k in [&self.holder, &self.tier, &self.identity] {
            out.extend_from_slice(k);
        }
        out.extend_from_slice(&self.g.to_le_bytes());
        for m in self.mu {
            out.extend_from_slice(&m.to_le_bytes());
        }
        out.extend_from_slice(&self.nonce);
        out
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<ConvertIn> {
        let (holder, tier, identity) = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
        let g = i128_at(b, p)?;
        let mut mu = [0u64; 4];
        for m in &mut mu {
            *m = u64_at(b, p)?;
        }
        Some(ConvertIn { holder, tier, identity, g, mu, nonce: key32(b, p)? })
    }

    pub fn id(&self) -> [u8; 32] {
        let mut b = b"convert in".to_vec();
        b.extend_from_slice(&self.bytes());
        crate::chronometry::payload_hash(&b)
    }
}

impl Handoff {
    fn kind(&self) -> u8 {
        match self {
            Handoff::Offer(_) => 0,
            Handoff::Accept { .. } => 1,
            Handoff::Refuse { .. } => 2,
            Handoff::Expire { .. } => 3,
            Handoff::ConvertOut(_) => 5,
            Handoff::ConvertIn(_) => 6,
            // 78.133 — a tariffed mouth is its own kind, so a flat mouth's
            // writing reads as it did.
            Handoff::Mouth(m) if m.tube.is_some() => KIND_TUBE,
            Handoff::Mouth(m) if m.tariff.is_some() => KIND_TARIFFED_MOUTH,
            Handoff::Mouth(_) => 7,
            Handoff::Close { .. } => 8,
            Handoff::Traverse { .. } => 9,
            Handoff::Feed { .. } => KIND_FEED,
            Handoff::Draw { .. } => KIND_DRAW,
        }
    }

    fn put_body(&self, out: &mut Vec<u8>) {
        match self {
            // 78.134 — a tube: the flat writing, the tube, then its tariff
            // flagged.
            Handoff::Mouth(m) if m.tube.is_some() => {
                out.extend_from_slice(&m.legacy_bytes());
                if let Some(t) = &m.tube {
                    out.extend_from_slice(&t.bytes());
                }
                match &m.tariff {
                    None => out.push(0),
                    Some(t) => {
                        out.push(1);
                        out.extend_from_slice(&t.bytes());
                    }
                }
            }
            Handoff::Offer(o) => out.extend_from_slice(&o.bytes()),
            Handoff::Accept { id } | Handoff::Refuse { id } | Handoff::Expire { id } | Handoff::Close { id } => out.extend_from_slice(id),
            Handoff::ConvertOut(c) => out.extend_from_slice(&c.bytes()),
            Handoff::ConvertIn(c) => out.extend_from_slice(&c.bytes()),
            Handoff::Mouth(m) => out.extend_from_slice(&m.bytes()),
            Handoff::Traverse { offer, mouth } => {
                out.extend_from_slice(&offer.bytes());
                out.extend_from_slice(mouth);
            }
            Handoff::Feed { mouth, mu_s, seq } | Handoff::Draw { mouth, mu_s, seq } => {
                out.extend_from_slice(mouth);
                out.extend_from_slice(&mu_s.to_le_bytes());
                out.extend_from_slice(&seq.to_le_bytes());
            }
        }
    }

    /// The handoff it names. A traversal is the handoff its offer is.
    pub fn id(&self) -> [u8; 32] {
        match self {
            Handoff::Offer(o) | Handoff::Traverse { offer: o, .. } => o.id(),
            Handoff::Accept { id } | Handoff::Refuse { id } | Handoff::Expire { id } | Handoff::Close { id } => *id,
            Handoff::ConvertOut(c) => c.id(),
            Handoff::ConvertIn(c) => c.id(),
            Handoff::Mouth(m) => m.id(),
            Handoff::Feed { mouth, .. } | Handoff::Draw { mouth, .. } => *mouth,
        }
    }

    /// The bytes the filer signs.
    pub fn signed_bytes(&self, who: &[u8; 32]) -> Vec<u8> {
        let mut out = SIGNED.to_vec();
        out.extend_from_slice(who);
        out.push(self.kind());
        self.put_body(&mut out);
        out
    }

    pub fn sign(self, key: &ed25519_dalek::SigningKey) -> SignedHandoff {
        use ed25519_dalek::Signer;
        let who = key.verifying_key().to_bytes();
        let sig = key.sign(&self.signed_bytes(&who)).to_bytes();
        SignedHandoff { who, handoff: self, sig }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SignedHandoff {
    pub who: [u8; 32],
    pub handoff: Handoff,
    pub sig: [u8; 64],
}

/// WHERE A HANDOFF STANDS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// Offered, in flight, awaiting the receiver until `until` (a height).
    Offered { until: u64 },
    Accepted,
    Returned,
}

/// WHAT A FRAME ASKS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HandoffRequest {
    Signed(SignedHandoff),
    /// A handoff's standing, read; nothing sealed.
    Read([u8; 32]),
    /// 78.128 — the open mouth loops whose holonomy is not the identity:
    /// curvature recorded; nothing sealed.
    Loops,
    /// 78.134 — a mouth pair's tube, read; nothing sealed.
    TubeRead([u8; 32]),
}

const KIND_READ: u8 = 4;
const KIND_LOOPS: u8 = 10;
const KIND_TARIFFED_MOUTH: u8 = 11;
const KIND_TUBE: u8 = 12;
const KIND_TUBE_READ: u8 = 13;
const KIND_FEED: u8 = 14;
const KIND_DRAW: u8 = 15;

/// `[ACT_HANDOFF][13][mouth 32]`.
pub fn encode_tube_read(mouth: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_HANDOFF, KIND_TUBE_READ];
    out.extend_from_slice(&mouth);
    out
}

/// 78.134 — A TUBE AS READ, derived and never stored: each mouth's place on
/// its domain's `ℍ³` (its frame's base point, `v² = −1`), each end's horizon
/// level `c` from the throat's mass in that end's curvature, and the mass.
/// A traversal entering at level `from_level` emerges at `to_level`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TubeReading {
    pub from_place: crate::transducer::cga::Mv,
    pub to_place: crate::transducer::cga::Mv,
    pub from_level: num_rational::BigRational,
    pub to_level: num_rational::BigRational,
    pub mass: (u64, u64),
}

/// `[0]` no tube; `[1][from place][to place][from level][to level][mass n d]`,
/// each level written as a scalar multivector — the canonical rational.
pub fn encode_tube_reading(r: Option<&TubeReading>) -> Vec<u8> {
    let Some(r) = r else { return vec![0] };
    let mut out = vec![1];
    r.from_place.to_bytes(&mut out);
    r.to_place.to_bytes(&mut out);
    crate::transducer::cga::Mv::scalar(r.from_level.clone()).to_bytes(&mut out);
    crate::transducer::cga::Mv::scalar(r.to_level.clone()).to_bytes(&mut out);
    out.extend_from_slice(&r.mass.0.to_le_bytes());
    out.extend_from_slice(&r.mass.1.to_le_bytes());
    out
}

pub fn decode_tube_reading(b: &[u8]) -> Option<Option<TubeReading>> {
    let (flag, rest) = b.split_first()?;
    match flag {
        0 => return rest.is_empty().then_some(None),
        1 => {}
        _ => return None,
    }
    let mut p = 0;
    let from_place = crate::transducer::cga::Mv::from_bytes(rest, &mut p)?;
    let to_place = crate::transducer::cga::Mv::from_bytes(rest, &mut p)?;
    let from_level = crate::transducer::cga::Mv::from_bytes(rest, &mut p)?.scalar_part();
    let to_level = crate::transducer::cga::Mv::from_bytes(rest, &mut p)?.scalar_part();
    let mass = (u64_at(rest, &mut p)?, u64_at(rest, &mut p)?);
    (p == rest.len()).then_some(Some(TubeReading { from_place, to_place, from_level, to_level, mass }))
}

/// `[ACT_HANDOFF][10]`.
pub fn encode_loops_read() -> Vec<u8> {
    vec![super::ACT_HANDOFF, KIND_LOOPS]
}

/// 78.128 — ONE MOUTH LOOP: the mouth pair whose opening closed a cycle of
/// domains, and the cycle's exact holonomy (its frame transitions composed
/// in path order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loop {
    pub mouth: [u8; 32],
    pub holonomy: crate::transducer::cga::Mv,
    /// 78.134 — whether every mouth of the cycle is a tube; a cycle through
    /// an untethered mouth (two free frames) reads its holonomy as history,
    /// not as an isometry of curved space.
    pub tethered: bool,
}

/// `[count][(mouth 32, holonomy, tethered u8) …]`.
pub fn encode_loops_reply(loops: &[Loop]) -> Vec<u8> {
    let mut out = Vec::new();
    crate::transducer::address::put_key_count(&mut out, loops.len());
    for l in loops {
        out.extend_from_slice(&l.mouth);
        l.holonomy.to_bytes(&mut out);
        out.push(u8::from(l.tethered));
    }
    out
}

pub fn decode_loops_reply(b: &[u8]) -> Option<Vec<Loop>> {
    let mut p = 0;
    let n = crate::transducer::address::take_key_count(b, &mut p)?;
    let loops = (0..n)
        .map(|_| {
            let mouth = key32(b, &mut p)?;
            let holonomy = crate::transducer::cga::Mv::from_bytes(b, &mut p)?;
            let tethered = match *b.get(p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            p += 1;
            Some(Loop { mouth, holonomy, tethered })
        })
        .collect::<Option<Vec<_>>>()?;
    (p == b.len()).then_some(loops)
}

/// `[ACT_HANDOFF][kind][who 32][body][sig 64]`.
pub fn encode_handoff(h: &SignedHandoff) -> Vec<u8> {
    let mut out = vec![super::ACT_HANDOFF, h.handoff.kind()];
    out.extend_from_slice(&h.who);
    h.handoff.put_body(&mut out);
    out.extend_from_slice(&h.sig);
    out
}

/// `[ACT_HANDOFF][4][id 32]`.
pub fn encode_handoff_read(id: [u8; 32]) -> Vec<u8> {
    let mut out = vec![super::ACT_HANDOFF, KIND_READ];
    out.extend_from_slice(&id);
    out
}

pub fn decode_handoff_request(b: &[u8]) -> Option<HandoffRequest> {
    let [act, kind, ..] = b else { return None };
    if *act != super::ACT_HANDOFF {
        return None;
    }
    let mut p = 2;
    let id = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
        let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
        *p += 32;
        Some(k)
    };
    let request = match *kind {
        KIND_READ => HandoffRequest::Read(id(b, &mut p)?),
        KIND_LOOPS => HandoffRequest::Loops,
        KIND_TUBE_READ => HandoffRequest::TubeRead(id(b, &mut p)?),
        k => {
            let who = id(b, &mut p)?;
            let handoff = match k {
                0 => Handoff::Offer(Box::new(Offer::take(b, &mut p)?)),
                1 => Handoff::Accept { id: id(b, &mut p)? },
                2 => Handoff::Refuse { id: id(b, &mut p)? },
                3 => Handoff::Expire { id: id(b, &mut p)? },
                5 => Handoff::ConvertOut(Box::new(ConvertOut::take(b, &mut p)?)),
                6 => Handoff::ConvertIn(Box::new(ConvertIn::take(b, &mut p)?)),
                7 => Handoff::Mouth(Box::new(Mouth::take(b, &mut p)?)),
                KIND_TARIFFED_MOUTH => {
                    let mut m = Mouth::take(b, &mut p)?;
                    m.tariff = Some(Tariff::take(b, &mut p)?);
                    Handoff::Mouth(Box::new(m))
                }
                k @ (KIND_FEED | KIND_DRAW) => {
                    let mouth = id(b, &mut p)?;
                    let (mu_s, seq) = (u64_at(b, &mut p)?, u64_at(b, &mut p)?);
                    match k {
                        KIND_FEED => Handoff::Feed { mouth, mu_s, seq },
                        _ => Handoff::Draw { mouth, mu_s, seq },
                    }
                }
                KIND_TUBE => {
                    let mut m = Mouth::take(b, &mut p)?;
                    m.tube = Some(Tube::take(b, &mut p)?);
                    let flag = *b.get(p)?;
                    p += 1;
                    m.tariff = match flag {
                        0 => None,
                        1 => Some(Tariff::take(b, &mut p)?),
                        _ => return None,
                    };
                    Handoff::Mouth(Box::new(m))
                }
                8 => Handoff::Close { id: id(b, &mut p)? },
                9 => {
                    let offer = Box::new(Offer::take(b, &mut p)?);
                    Handoff::Traverse { offer, mouth: id(b, &mut p)? }
                }
                _ => return None,
            };
            let sig: [u8; 64] = b.get(p..p + 64)?.try_into().ok()?;
            p += 64;
            HandoffRequest::Signed(SignedHandoff { who, handoff, sig })
        }
    };
    (p == b.len()).then_some(request)
}

/// WHY A HANDOFF ACT WAS NOT TAKEN.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandoffRefusal {
    Unsigned,
    /// The offer does not conserve, by why.
    Unbalanced(Unbalanced),
    /// The sender (offer, expiry) or receiver (accept, refuse) is not the filer.
    NotTheTier,
    /// A tier no domain on this court names; or both tiers in one domain.
    Unbound,
    /// The same offer sealed already.
    Offered,
    /// No handoff of this id.
    Unknown,
    /// Answered already — accepted or returned.
    Answered,
    /// Accepted or refused past its window; or expiry claimed before it closed.
    Window,
    LineRefused,
    /// 78.127 — a conversion by, or to, a tier that is not its domain's
    /// designated reserve.
    NotReserve,
    /// 78.127 — the reserve holds less μ than the conversion pays, on a lane.
    ReserveShort,
    /// 78.127 — `rate · g` is not an exact integer on every lane, or `g` is
    /// not positive: rounding is the ancients' to declare on G.
    Inexact,
    /// 78.127 — the holder holds less μ than it offers.
    HolderShort,
    /// 78.128 — a handoff between domains citing no OPEN mouth pair: domains
    /// share no space, and nothing crosses without a physical connection.
    NoMouth,
    /// 78.128 — a mouth frame zero or no versor: no horizon sphere carried.
    Degenerate,
    /// 78.128 — a keeper that is neither of the pair's two reserves.
    BadKeeper,
    /// 78.128 — the traversal's tiers are not in the mouth's two domains.
    NotTheMouth,
    /// 78.133 — the tariff is not one, by why; or a tariffed mouth carries a
    /// flat toll beside it.
    Untariffed(Untariffed),
    /// 78.133 — a share to a key that is neither the keeper nor one of its
    /// progenitor lines.
    NotALine,
    /// 78.134 — a tube's frame is not a motion of curved space (not in
    /// `Spin⁺(3,1)`).
    NotIsometry,
    /// 78.134 — a tube's mass or a curvature radius not a positive exact
    /// rational.
    BadTube,
    /// 78.134 — a superseding tube citing no mouth, a mouth already a tube
    /// or closed, or another pair's.
    NotSupersedable,
    /// 78.134 — a mouth offered without its tube: two free frames, refused
    /// (those offered before tubes stand as history).
    Untethered,
    /// 78.135 — a draw that would leave the throat no mass (closing is
    /// `Close`), or more than its bank holds.
    Starved,
    /// 78.135 — a bank act whose `seq` is not the throat's count: a resend,
    /// or out of order.
    Stale,
}

/// THE COURT'S ANSWER.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Handed {
    /// Sealed: the receipt, the handoff's id and its state now.
    Sealed { at: Stamp, receipt: [u8; 32], id: [u8; 32], state: State },
    Refused(HandoffRefusal),
}

fn put_state(out: &mut Vec<u8>, s: &State) {
    match s {
        State::Offered { until } => {
            out.push(0);
            out.extend_from_slice(&until.to_le_bytes());
        }
        State::Accepted => out.push(1),
        State::Returned => out.push(2),
    }
}

fn take_state(b: &[u8], p: &mut usize) -> Option<State> {
    let tag = *b.get(*p)?;
    *p += 1;
    Some(match tag {
        0 => {
            let until = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            State::Offered { until }
        }
        1 => State::Accepted,
        2 => State::Returned,
        _ => return None,
    })
}

fn refusal_bytes(r: &HandoffRefusal) -> [u8; 2] {
    match r {
        HandoffRefusal::Unsigned => [0, 0],
        HandoffRefusal::Unbalanced(u) => match u {
            Unbalanced::Unordered => [1, 0],
            Unbalanced::Lane { lane } => [2, lane_byte(*lane)],
            Unbalanced::Unconserved { lane } => [3, lane_byte(*lane)],
            Unbalanced::Backward { lane } => [4, lane_byte(*lane)],
            Unbalanced::Overflow { lane } => [5, lane_byte(*lane)],
        },
        HandoffRefusal::NotTheTier => [6, 0],
        HandoffRefusal::Unbound => [7, 0],
        HandoffRefusal::Offered => [8, 0],
        HandoffRefusal::Unknown => [9, 0],
        HandoffRefusal::Answered => [10, 0],
        HandoffRefusal::Window => [11, 0],
        HandoffRefusal::LineRefused => [12, 0],
        HandoffRefusal::NotReserve => [13, 0],
        HandoffRefusal::ReserveShort => [14, 0],
        HandoffRefusal::Inexact => [15, 0],
        HandoffRefusal::HolderShort => [16, 0],
        HandoffRefusal::NoMouth => [17, 0],
        HandoffRefusal::Degenerate => [18, 0],
        HandoffRefusal::BadKeeper => [19, 0],
        HandoffRefusal::NotTheMouth => [20, 0],
        HandoffRefusal::Untariffed(u) => [
            21,
            match u {
                Untariffed::ZeroDenominator => 0,
                Untariffed::Unordered => 1,
                Untariffed::NotWhole => 2,
                Untariffed::Flat => 3,
            },
        ],
        HandoffRefusal::NotALine => [22, 0],
        HandoffRefusal::NotIsometry => [23, 0],
        HandoffRefusal::BadTube => [24, 0],
        HandoffRefusal::NotSupersedable => [25, 0],
        HandoffRefusal::Untethered => [26, 0],
        HandoffRefusal::Starved => [27, 0],
        HandoffRefusal::Stale => [28, 0],
    }
}

#[expect(
    clippy::manual_unwrap_or,
    reason = "rule 11 — `unwrap_or` is the spelling this refuses; a lane past a byte is named at the wall, never wrapped"
)]
fn lane_byte(lane: usize) -> u8 {
    match u8::try_from(lane) {
        Ok(l) => l,
        Err(_) => u8::MAX,
    }
}

fn refusal_of(code: u8, lane: u8) -> Option<HandoffRefusal> {
    let lane = usize::from(lane);
    Some(match code {
        0 => HandoffRefusal::Unsigned,
        1 => HandoffRefusal::Unbalanced(Unbalanced::Unordered),
        2 => HandoffRefusal::Unbalanced(Unbalanced::Lane { lane }),
        3 => HandoffRefusal::Unbalanced(Unbalanced::Unconserved { lane }),
        4 => HandoffRefusal::Unbalanced(Unbalanced::Backward { lane }),
        5 => HandoffRefusal::Unbalanced(Unbalanced::Overflow { lane }),
        6 => HandoffRefusal::NotTheTier,
        7 => HandoffRefusal::Unbound,
        8 => HandoffRefusal::Offered,
        9 => HandoffRefusal::Unknown,
        10 => HandoffRefusal::Answered,
        11 => HandoffRefusal::Window,
        12 => HandoffRefusal::LineRefused,
        13 => HandoffRefusal::NotReserve,
        14 => HandoffRefusal::ReserveShort,
        15 => HandoffRefusal::Inexact,
        16 => HandoffRefusal::HolderShort,
        17 => HandoffRefusal::NoMouth,
        18 => HandoffRefusal::Degenerate,
        19 => HandoffRefusal::BadKeeper,
        20 => HandoffRefusal::NotTheMouth,
        21 => HandoffRefusal::Untariffed(match lane {
            0 => Untariffed::ZeroDenominator,
            1 => Untariffed::Unordered,
            2 => Untariffed::NotWhole,
            3 => Untariffed::Flat,
            _ => return None,
        }),
        22 => HandoffRefusal::NotALine,
        23 => HandoffRefusal::NotIsometry,
        24 => HandoffRefusal::BadTube,
        25 => HandoffRefusal::NotSupersedable,
        26 => HandoffRefusal::Untethered,
        27 => HandoffRefusal::Starved,
        28 => HandoffRefusal::Stale,
        _ => return None,
    })
}

pub fn encode_handoff_reply(h: &Handed) -> Vec<u8> {
    let mut out = Vec::new();
    match h {
        Handed::Sealed { at, receipt, id, state } => {
            out.push(0);
            at.put_canonical(&mut out);
            out.extend_from_slice(receipt);
            out.extend_from_slice(id);
            put_state(&mut out, state);
        }
        Handed::Refused(r) => {
            out.push(1);
            out.extend_from_slice(&refusal_bytes(r));
        }
    }
    out
}

pub fn decode_handoff_reply(b: &[u8]) -> Option<Handed> {
    let mut p = 1;
    let h = match b.first_chunk::<1>()? {
        [0] => {
            let at = Stamp::take_canonical(b, &mut p)?;
            let receipt: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            let id: [u8; 32] = b.get(p..p + 32)?.try_into().ok()?;
            p += 32;
            Handed::Sealed { at, receipt, id, state: take_state(b, &mut p)? }
        }
        [1] => {
            let (code, lane) = (*b.get(1)?, *b.get(2)?);
            p = 3;
            Handed::Refused(refusal_of(code, lane)?)
        }
        _ => return None,
    };
    (p == b.len()).then_some(h)
}

/// A handoff's standing as read: `[0]` none, or `[1][state][out consumed
/// u8][in consumed u8]`.
pub fn encode_read_reply(read: Option<(State, bool, bool)>) -> Vec<u8> {
    match read {
        None => vec![0],
        Some((s, out, inn)) => {
            let mut v = vec![1];
            put_state(&mut v, &s);
            v.extend_from_slice(&[u8::from(out), u8::from(inn)]);
            v
        }
    }
}

pub fn decode_read_reply(b: &[u8]) -> Option<Option<(State, bool, bool)>> {
    let mut p = 1;
    let read = match b.first_chunk::<1>()? {
        [0] => None,
        [1] => {
            let s = take_state(b, &mut p)?;
            let flag = |x: u8| match x {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            };
            let (out, inn) = (flag(*b.get(p)?)?, flag(*b.get(p + 1)?)?);
            p += 2;
            Some((s, out, inn))
        }
        _ => return None,
    };
    (p == b.len()).then_some(read)
}
