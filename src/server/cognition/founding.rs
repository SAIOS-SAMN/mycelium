// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-HOSTING §5a.9 B0, MU-RECURSION 78.169 6(c) — REPLICATION BY
//! RECONSTRUCTION: each domain's founding, tiers, reserve and origin read
//! back off the founding receipts this court holds, under each signer's signature.
//!
//! A back-filled receipt carries no signature of its own (47.2 step 3): it is
//! admitted by its chain, and its signer's signature reaches it only through
//! a signed frame at or above its place on the signer's line, whose
//! predecessor hashes link down to it (MU-HOSTING §5a.10: the hash links, it
//! never admits). So a founding receipt admitted by back-fill waits behind
//! the one signed-reach gate (`reach.rs`) until such a frame is admitted,
//! and a founding receipt admitted under its own signature is reconstructed
//! at once. Each citation is then applied by the founding hypha's own rules and
//! refusals (`Court::admit_domain`, `designate_reserve`, `record_origin`),
//! once by content: the same content held already changes nothing, and
//! other content is refused by name and held by its stamp
//! (`founding_refusals`).

use super::Court;
use crate::anastome::dialogue::domain::{DomainGenesis, Founded, OriginReading};
use crate::chronometry::{Cite, KStepReceipt, Stamp};

/// WHAT ONE FOUNDING CITATION CAME TO at this court: the reconstruction
/// events of MU-SCIENCE §5.0, read on the domain registries. Named for
/// the event, so that `Reconstructed<T>` (`court::admission`) names only
/// the reconstructed object (ROADMAP track 25, X0).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum FoundingEvent {
    /// The registry now holds what the citation states (a collapse).
    Applied,
    /// The registry held the same content already, reached by another
    /// receipt: nothing changed (a converse).
    Held,
    /// The registry holds other content: the founding hypha's own refusal,
    /// the registry unchanged (a collision).
    Refused(Founded),
}

impl Court {
    /// ONE RECEIPT UNDER ITS SIGNER'S SIGNATURE, reconstructed: each
    /// founding citation applied by the founding hypha's rules. A reserve or
    /// an origin naming a domain no founding here holds, and that the
    /// receipt does not found itself, waits in `unfounded` for that founding
    /// (the mesh's topological order), and is reconstructed when it lands.
    pub(crate) fn reconstruct_founding(&mut self, r: &KStepReceipt) {
        if let Some(domain) = self.unfounded_in(r) {
            let mut key = domain.to_vec();
            key.extend_from_slice(&Stamp::line_key(&r.stamp.vk, r.stamp.place()));
            self.unfounded.insert(key, r.clone());
            return;
        }
        let mut founded: Vec<[u8; 32]> = Vec::new();
        for c in &r.cites {
            let read = match c {
                Cite::Domain { domain, tiers } => {
                    let read = self.reconstruct_domain(*domain, tiers);
                    if read == FoundingEvent::Applied {
                        founded.push(*domain);
                    }
                    read
                }
                Cite::Reserve { domain, tier } => self.reconstruct_reserve(*domain, *tier),
                Cite::Origin { domain, kind, version, commitment } => {
                    self.reconstruct_origin(r, *domain, (*kind, *version, *commitment))
                }
                // 78.127 — a fold of a domain's reserve tier is its era
                // anchor wherever it was admitted: a redesignation pending
                // here holds from it, as on the court that sealed the fold.
                Cite::Uplink { tier, .. } | Cite::Chained { tier, .. } => {
                    if let Some(domain) = self.tier_domain.get(tier).copied() {
                        self.anchor_passed(tier, &domain);
                    }
                    continue;
                }
                _ => continue,
            };
            match read {
                FoundingEvent::Applied => self.mark_persist(),
                FoundingEvent::Held => {}
                FoundingEvent::Refused(why) => {
                    if !self.founding_refusals.contains(&r.stamp.key()) {
                        self.founding_refusals.insert(r.stamp.key(), why);
                    }
                }
            }
        }
        for domain in founded {
            self.hear_unfounded(&domain);
        }
    }

    /// THE REFUSAL A PEER'S FOUNDING RECEIPT MET HERE, by its stamp: the
    /// founding hypha's own name for why this court's registries kept other
    /// content. `None` where the receipt was applied, held the same content,
    /// or is not held here.
    pub fn founding_refusal(&self, at: &Stamp) -> Option<Founded> {
        self.founding_refusals.get(&at.key()).cloned()
    }

    /// The domain a receipt's reserve or origin names that no founding here
    /// holds and the receipt does not found itself, where there is one.
    fn unfounded_in(&self, r: &KStepReceipt) -> Option<[u8; 32]> {
        let founds = |d: &[u8; 32]| r.cites.iter().any(|c| matches!(c, Cite::Domain { domain, .. } if domain == d));
        r.cites.iter().find_map(|c| match c {
            Cite::Reserve { domain, .. } | Cite::Origin { domain, .. } => {
                (self.domains.get(domain).is_none() && !founds(domain)).then_some(*domain)
            }
            _ => None,
        })
    }

    /// The receipts that waited on `domain`'s founding, reconstructed now
    /// that it is held.
    fn hear_unfounded(&mut self, domain: &[u8; 32]) {
        let waiting: Vec<Vec<u8>> = match self.unfounded.fibre_at(domain) {
            Some(fibre) => fibre.map(|(key, _)| key).collect(),
            None => Vec::new(),
        };
        for key in waiting {
            if let Some(r) = self.unfounded.remove(&key) {
                self.reconstruct_founding(&r);
            }
        }
    }

    /// 78.120 — A DOMAIN'S GENESIS, by `admit_domain`'s refusals: the tier
    /// keys in the keys' one writing, no founding of the domain already, no tier
    /// another domain names.
    fn reconstruct_domain(&mut self, domain: [u8; 32], tiers: &[[u8; 32]]) -> FoundingEvent {
        match self.domains.get(&domain) {
            Some(held) if held.as_slice() == tiers => return FoundingEvent::Held,
            Some(_) => return FoundingEvent::Refused(Founded::AlreadyFounded),
            None => {}
        }
        if !(DomainGenesis { domain, tiers: tiers.to_vec() }).ordered() {
            return FoundingEvent::Refused(Founded::Unordered);
        }
        if tiers.iter().any(|t| self.tier_domain.get(t).is_some()) {
            return FoundingEvent::Refused(Founded::TierHeld);
        }
        self.hold_domain(domain, tiers.to_vec());
        FoundingEvent::Applied
    }

    /// 78.127 — A RESERVE DESIGNATED, by `designate_reserve`'s rule: a tier
    /// the domain's genesis names, the reserve at once where the domain has
    /// none, pending until the sitting reserve's next fold otherwise.
    fn reconstruct_reserve(&mut self, domain: [u8; 32], tier: [u8; 32]) -> FoundingEvent {
        let Some(tiers) = self.domains.get(&domain) else { return FoundingEvent::Refused(Founded::NotFounded) };
        if !tiers.contains(&tier) {
            return FoundingEvent::Refused(Founded::NotATier);
        }
        let held = match self.reserves.get(&domain) {
            Some(r) => *r,
            None => super::convert::Reserve::default(),
        };
        let mut next = held;
        match next.current {
            None => next.current = Some(tier),
            Some(_) => next.pending = Some(tier),
        }
        if next == held {
            return FoundingEvent::Held;
        }
        self.reserves.insert(domain, next);
        FoundingEvent::Applied
    }

    /// 78.132 — A UNIVERSE'S ORIGIN, by `record_origin`'s refusals: the
    /// domain founded, and no other origin held for it; held with the stamp
    /// and receipt of the act that sealed it, as on the founding hypha.
    fn reconstruct_origin(&mut self, r: &KStepReceipt, domain: [u8; 32], (kind, version, commitment): (u8, u8, [u8; 32])) -> FoundingEvent {
        if self.domains.get(&domain).is_none() {
            return FoundingEvent::Refused(Founded::NotFounded);
        }
        match self.origins.get(&domain) {
            Some(o) if (o.kind, o.version, o.commitment) == (kind, version, commitment) => return FoundingEvent::Held,
            Some(_) => return FoundingEvent::Refused(Founded::AlreadyOriginated),
            None => {}
        }
        self.origins.insert(domain, OriginReading { kind, version, commitment, at: r.stamp, receipt: r.chain_hash.bytes() });
        FoundingEvent::Applied
    }
}
