// 78.139 — A UNIVERSE ON THE COURT (the rulings on (i-b), with: a galactic
// core is a larger wormhole, every horizon has its far face). What a universe states of itself — its geometry (78.140: curved or
// a torus), its parent face,
// its ancients' basket, its join to its parent's core — held once each and
// never edited; and the core tubes that join universes.
//
//! A domain READS AS A UNIVERSE once it has an origin. Its origin of version
//! 2 states its geometry and its parent face: the core tube it
//! bounced out of — open, a core tube, its far end in this domain — or
//! unwitnessed (a root). Its SITTING RESERVE states, once each: its
//! geometry where the origin did not (a v1 origin; legacy domains), its
//! ancients' survival basket, and — a root — its JOIN to the parent's core
//! once found. Existing domains keep working (a ruled grace): they
//! originate before joining a core tube or converting.

use super::Court;
use crate::anastome::dialogue::domain::{Founded, Origin};
use crate::anastome::dialogue::universe::{basket_holds, Geometry, Genesis, Parent, SignedStatement, Stated, Universe, UniverseReading, KIND_GEOMETRY, KIND_JOIN};
use crate::anastome::dialogue::handoff::State;
use crate::chronometry::{ActKind, Cite};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_count, take_key_count};

/// The universes written: `[count][(domain 32, universe) …]`.
pub(crate) fn put_universes(v: &mut Vec<u8>, universes: &Keyed<Universe>) {
    put_key_count(v, universes.len());
    for (domain, u) in universes.scoped() {
        v.extend_from_slice(&domain);
        u.put(v);
    }
}

pub(crate) fn take_universes(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], Universe)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let domain: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some((domain, Universe::take(b, p)?))
        })
        .collect()
}

impl Court {
    fn universe_of(&self, domain: &[u8; 32]) -> Universe {
        match self.universes.get(domain) {
            Some(u) => u.clone(),
            None => Universe::default(),
        }
    }

    /// 78.140 — the geometry a domain has stated, if it has.
    pub(crate) fn geometry_of(&self, domain: &[u8; 32]) -> Option<&Geometry> {
        self.universes.get(domain)?.geometry.as_ref()
    }

    /// Whether the domain `tier` stands in has stated its ancients' basket.
    pub(crate) fn basket_stated_for(&self, tier: &[u8; 32]) -> bool {
        self.tier_domain.get(tier).and_then(|d| self.universes.get(d)).is_some_and(|u| !u.basket.is_empty())
    }

    /// Whether `tube` is an OPEN CORE TUBE whose far end stands in `domain`.
    pub(crate) fn core_into(&self, tube: &[u8; 32], domain: &[u8; 32]) -> Option<crate::anastome::dialogue::handoff::Tube> {
        let held = self.handoffs.mouths.get(tube)?;
        let t = held.mouth.tube.as_ref().filter(|t| t.core)?;
        let far = self.tier_domain.get(&held.mouth.to_tier)?;
        (held.state == State::Accepted && far == domain).then(|| t.clone())
    }

    /// 78.139 — WHAT AN ORIGIN STATES OF ITS UNIVERSE, read before it seals:
    /// version 2 exactly where it carries a geometry and a parent face; the
    /// geometry well formed and not stated already; a parent core an open
    /// core tube into this domain, its far end's space this geometry. The citations
    /// it adds, each by its coordinates.
    pub(crate) fn origin_universe(&self, o: &Origin) -> Result<Vec<Cite>, Founded> {
        let Some(u) = &o.universe else {
            return match o.version {
                2 => Err(Founded::Malformed),
                _ => Ok(Vec::new()),
            };
        };
        if o.version != 2 || !u.geometry.holds() {
            return Err(Founded::Malformed);
        }
        if self.geometry_of(&o.domain).is_some() {
            return Err(Founded::AlreadyStated);
        }
        let mut cites = vec![Cite::Stated { domain: o.domain, kind: KIND_GEOMETRY, body: Stated::Geometry(u.geometry.clone()).hash() }];
        if let Parent::Core(tube) = u.parent {
            let Some(t) = self.core_into(&tube, &o.domain) else { return Err(Founded::NotACore) };
            if t.to_space != u.geometry {
                return Err(Founded::Malformed);
            }
            cites.push(Cite::Stated { domain: o.domain, kind: KIND_JOIN, body: tube });
        }
        Ok(cites)
    }

    /// The universe an origin of version 2 stated, held.
    pub(crate) fn hold_genesis(&mut self, domain: [u8; 32], g: &Genesis) {
        let mut u = self.universe_of(&domain);
        u.geometry = Some(g.geometry.clone());
        u.parent = Some(g.parent);
        self.universes.insert(domain, u);
    }

    /// 78.139 — A UNIVERSE'S STATEMENT, by its SITTING RESERVE, once each:
    /// its geometry (well formed; where none is stated), its ancients' basket
    /// (well formed; where none is stated), or its JOIN — an originated root
    /// naming an open core tube into it, its far space its geometry where one
    /// is stated. Sealed by its coordinates; never an edit.
    pub fn state(&mut self, s: &SignedStatement) -> Founded {
        let st = &s.statement;
        if self.domains.get(&st.domain).is_none() {
            return Founded::NotFounded;
        }
        let Some(reserve) = self.reserve_of(&st.domain) else { return Founded::NotReserve };
        if !crate::court::identity::verify(&reserve, &st.signed_bytes(), &s.sig) {
            return Founded::Unsigned;
        }
        let mut u = self.universe_of(&st.domain);
        let body = match &st.stated {
            Stated::Geometry(g) => {
                if !g.holds() {
                    return Founded::Malformed;
                }
                if u.geometry.is_some() {
                    return Founded::AlreadyStated;
                }
                u.geometry = Some(g.clone());
                st.stated.hash()
            }
            Stated::Basket(b) => {
                if !basket_holds(b) {
                    return Founded::Malformed;
                }
                if !u.basket.is_empty() {
                    return Founded::AlreadyStated;
                }
                u.basket = b.clone();
                st.stated.hash()
            }
            Stated::Join(tube) => {
                if self.origins.get(&st.domain).is_none() {
                    return Founded::NotAUniverse;
                }
                if matches!(u.parent, Some(Parent::Core(_))) {
                    return Founded::AlreadyStated;
                }
                let Some(t) = self.core_into(tube, &st.domain) else { return Founded::NotACore };
                if u.geometry.as_ref().is_some_and(|g| *g != t.to_space) {
                    return Founded::Malformed;
                }
                u.parent = Some(Parent::Core(*tube));
                *tube
            }
        };
        let cite = Cite::Stated { domain: st.domain, kind: st.stated.kind(), body };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], reserve, |_| vec![cite.clone()]) else { return Founded::LineRefused };
        self.universes.insert(st.domain, u);
        self.mark_persist();
        Founded::Stated { at: r.stamp, receipt: r.chain_hash }
    }

    /// 78.139 — WHAT A UNIVERSE HAS STATED, as read. Sealing nothing.
    pub fn universe_reading(&self, domain: &[u8; 32]) -> UniverseReading {
        let u = self.universe_of(domain);
        UniverseReading { originated: self.origins.get(domain).is_some(), geometry: u.geometry, parent: u.parent, basket: u.basket }
    }
}
