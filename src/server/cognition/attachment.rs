// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.185 (approved 2026-11-17) — THE COVER THIS COURT HOLDS: the genesis
// hyphas, the ground, and every hypha an admitted adjunction attached.
//
//! The genesis hyphas were minted in one frame, so each two overlap by the
//! identity motion. An attached hypha adds the overlaps its tethers stated.
//! An adjunction or a retraction is checked under `forms::cover` on that
//! nerve and sealed on this court's line. Nothing is registered: the
//! attached hyphas are read off the admitted adjunctions held here, and join
//! the boot's live set, so a restart never burns an attached hypha's stake.

use super::Court;
use crate::anastome::dialogue::adjunction::{put_tethers, take_tethers, Adjunction, CoverRefused, Covered, Retraction, Signed, TetherStated, KIND_ADJUNCTION, KIND_RETRACTION};
use crate::chronometry::{ActKind, Cite};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_count, take_key_count};
use algebra::Clifford;
use forms::cover::{CoverRefusal, Nerve, Overlap, Tether};

/// ONE ATTACHED HYPHA: the tethers its admitted adjunction stated, and the
/// height it was sealed at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Attached {
    pub(crate) tethers: Vec<TetherStated>,
    pub(crate) sealed: u64,
}

fn refused(r: CoverRefusal) -> CoverRefused {
    match r {
        CoverRefusal::Held(_) => CoverRefused::Held,
        CoverRefusal::NoNode(k) => CoverRefused::NoNode(k),
        CoverRefusal::OpensCycle { a, b } => CoverRefused::OpensCycle(a, b),
        CoverRefusal::Cocycle { a, b, .. } => CoverRefused::Cocycle(a, b),
        CoverRefusal::Untethered => CoverRefused::Untethered,
        CoverRefusal::Disconnects => CoverRefused::Disconnects,
        CoverRefusal::Motion(_) => CoverRefused::Motion,
    }
}

impl Court {
    /// The genesis hyphas this court's mesh was minted with: the cover's
    /// ground, held at boot (never on the wire).
    pub fn hold_genesis_hyphas(&mut self, hyphas: &[[u8; 32]]) {
        self.genesis_hyphas = hyphas.to_vec();
    }

    /// The hyphas attached by admitted adjunctions, for the boot's live set.
    pub fn attached_hyphas(&self) -> Vec<[u8; 32]> {
        self.attached.scoped().filter_map(|(k, _)| <[u8; 32]>::try_from(k.as_slice()).ok()).collect()
    }

    /// THE NERVE this court holds: the genesis hyphas, each two by the
    /// identity, and every attached hypha with its tethers' overlaps.
    pub(crate) fn nerve(&self) -> Nerve {
        let one = Clifford::of_conformal(&crate::transducer::element::identity_versor());
        let mut nerve = Nerve { nodes: self.genesis_hyphas.clone(), overlaps: Vec::new() };
        for (i, a) in self.genesis_hyphas.iter().enumerate() {
            for b in self.genesis_hyphas.iter().skip(i + 1) {
                nerve.overlaps.push(Overlap { a: *a, b: *b, g: one.clone() });
            }
        }
        for (k, att) in self.attached.scoped() {
            let Ok(n) = <[u8; 32]>::try_from(k.as_slice()) else { continue };
            nerve.nodes.push(n);
            nerve.overlaps.extend(att.tethers.iter().map(|t| Overlap { a: t.node, b: n, g: Clifford::of_conformal(&t.g) }));
        }
        nerve
    }

    /// 78.185 §2 — AN ADJUNCTION: signed by the hypha's key, its crossing
    /// verified, its link checked on the nerve (a simplex, the cocycle
    /// closed, a tether at rest), then sealed.
    pub fn adjoin(&mut self, s: &Signed<Adjunction>) -> Covered {
        let refuse = Covered::Refused;
        let hypha = s.act.hypha;
        let Some(bytes) = s.signed_bytes() else { return refuse(CoverRefused::Unsigned) };
        if !crate::court::identity::verify(&hypha, &bytes, &s.sig) {
            return refuse(CoverRefused::Unsigned);
        }
        if s.crossing.verify().is_err() {
            return refuse(CoverRefused::Crossing);
        }
        let link: Vec<Tether> = s.act.tethers.iter().map(|t| Tether { node: t.node, g: Clifford::of_conformal(&t.g), planes: t.planes.clone() }).collect();
        if let Err(why) = self.nerve().adjoin(hypha, &link) {
            return refuse(refused(why));
        }
        let cite = Cite::Stated { domain: hypha, kind: KIND_ADJUNCTION, body: crate::chronometry::payload_hash(&bytes) };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], hypha, |_| vec![cite.clone()]) else { return refuse(CoverRefused::LineRefused) };
        self.attached.insert(hypha, Attached { tethers: s.act.tethers.clone(), sealed: r.stamp.h });
        self.mark_persist();
        Covered::Sealed { at: r.stamp, receipt: r.chain_hash.bytes() }
    }

    /// 78.185 §4 — A RETRACTION: signed by the hypha's key, an attached hypha
    /// (a genesis hypha is the ground, never retracted), no claim standing
    /// on its stake, the nerve connected without it; its stake returned to
    /// its balance, then sealed. What it witnessed stays on the ledger.
    pub fn retract(&mut self, s: &Signed<Retraction>) -> Covered {
        let refuse = Covered::Refused;
        let hypha = s.act.hypha;
        let Some(bytes) = s.signed_bytes() else { return refuse(CoverRefused::Unsigned) };
        if !crate::court::identity::verify(&hypha, &bytes, &s.sig) {
            return refuse(CoverRefused::Unsigned);
        }
        if s.crossing.verify().is_err() {
            return refuse(CoverRefused::Crossing);
        }
        if self.genesis_hyphas.contains(&hypha) {
            return refuse(CoverRefused::Genesis);
        }
        if self.attached.get(&hypha).is_none() {
            return refuse(CoverRefused::Held);
        }
        if self.atlas.claims.fibre_at(&hypha).is_some_and(|mut f| f.next().is_some()) {
            return refuse(CoverRefused::Claimed);
        }
        if let Err(why) = self.nerve().retract(&hypha) {
            return refuse(refused(why));
        }
        let cite = Cite::Stated { domain: hypha, kind: KIND_RETRACTION, body: crate::chronometry::payload_hash(&bytes) };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], hypha, |_| vec![cite.clone()]) else { return refuse(CoverRefused::LineRefused) };
        if let Some(stake) = self.treasury.stake_of(&hypha) {
            let _ = self.treasury.unstake(hypha, stake, (r.stamp, r.chain_hash.bytes()));
        }
        self.attached.remove(&hypha);
        self.mark_persist();
        Covered::Sealed { at: r.stamp, receipt: r.chain_hash.bytes() }
    }
}

/// The attached hyphas written: `[n][(hypha 32, sealed u64, tethers) …]`.
pub(crate) fn put_attached(attached: &Keyed<Attached>) -> Option<Vec<u8>> {
    let mut v = Vec::new();
    put_key_count(&mut v, attached.len());
    for (k, a) in attached.scoped() {
        v.extend_from_slice(&k);
        v.extend_from_slice(&a.sealed.to_le_bytes());
        put_tethers(&mut v, &a.tethers)?;
    }
    Some(v)
}

/// The attached hyphas read whole (`put_attached`).
pub(crate) fn take_attached(b: &[u8], p: &mut usize) -> Option<Vec<([u8; 32], Attached)>> {
    (0..take_key_count(b, p)?)
        .map(|_| {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            let sealed = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
            *p += 8;
            Some((k, Attached { tethers: take_tethers(b, p)?, sealed }))
        })
        .collect()
}
