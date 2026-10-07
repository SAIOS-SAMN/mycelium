// 78.123 B — THE LEDGER TREE AS A COMPLEX: a domain's folds chained by the
// folds that net them, read on the generic cover (Phase A) with two
// coefficient groups — the RATES on cycles, the NETS at vertices.
//
//! A parent fold nets children already sealed on this court's line. Three
//! readings, each exact:
//!
//! - AGGREGATION, by the fold's own continuity (`TierFold::continuity`): its
//!   rows less `Σ rate · included rows` balance its declared, lane by lane.
//! - VERTEX CONSERVATION, per child: the parts its parents include sum to
//!   its own rows and declared, per identity per lane. The court holds what
//!   remains while parents are unsealed; a part of the wrong sign, or more
//!   than remains, is a double count and is refused. Value still in flight
//!   is an OPEN REMAINDER — a fact of finality, never a breach.
//! - THE RATES COCYCLE, per domain: each child → parent edge carries its
//!   rate per lane on a [`Gauge`] of [`Rates`], so a cycle of any length is
//!   read against the potential and never by walking the tree. A cycle whose
//!   holonomy is not one is ARBITRAGE, docketed with its exact gain per
//!   lane; while one stands open, a fold seals PROVISIONAL.

use super::Kernel;
use crate::anastome::dialogue::uplink::{ChildRefusal, SignedFold, Uplinked};
use crate::chronometry::Cite;
use crate::network::nerve::Keyed;
use crate::transducer::cech::coefficient::{Coefficient, Lanes, Rates};
use crate::transducer::cech::cover::{Gauge, Gauged};

/// One open obstruction: the child fold's vertex, the parent fold's whose
/// edge closed the cycle, and the cycle's exact holonomy per lane.
pub(crate) type Obstruction = (Vec<u8>, Vec<u8>, Rates);

/// AN ADMITTED FOLD BY ITS SEALED COORDINATES (B′): its tier, the prefix
/// head it nets, its rows' commitment — what its receipt cites, held so a
/// parent proves the child after its receipt has folded off the hot line.
pub(crate) type Sealed = ([u8; 32], [u8; 32], [u8; 32]);

/// A FOLD'S VERTEX on its domain's gauge (78.123 B′): its own coordinates,
/// `tier ‖ prefix` — the fold, never the tier, so a rate that moves between
/// epochs is two edges and no cycle, while one fold netted by two routes
/// meets itself at its own vertex.
pub(crate) fn vertex(tier: &[u8; 32], prefix: &[u8; 32]) -> Vec<u8> {
    let mut v = Vec::with_capacity(64);
    v.extend_from_slice(tier);
    v.extend_from_slice(prefix);
    v
}

/// THE LEDGER TREES' STATE on this court — persisted on its node
/// (`LEAF_LEDGERS`), never re-derived from a line that may have folded.
#[derive(Default)]
pub(crate) struct Ledgers {
    /// Per admitted fold, keyed by its receipt's stamp: its sealed
    /// coordinates — dropped once a parent has netted all of it, so what is
    /// held is the folds with value still in flight.
    pub(crate) folds: Keyed<Sealed>,
    /// Per child, keyed by its receipt's stamp: what remains of its rows and
    /// of its declared, not yet included by a parent. Empty is consumed.
    pub(crate) remaining: Keyed<(Lanes, Lanes)>,
    /// Per domain: the rates gauge over its folds.
    pub(crate) gauges: Keyed<Gauge<Rates>>,
    /// Per domain: the obstructions standing open.
    pub(crate) obstructions: Keyed<Vec<Obstruction>>,
}

/// Whether `part` fits inside `left` on every identity and lane: the same
/// sign and no greater magnitude — else the first lane it passes.
fn fits(part: &Lanes, left: &Lanes) -> Result<(), usize> {
    for (who, v) in part.nets() {
        let r = left.net_of(who);
        for (l, (x, y)) in v.iter().zip(r).enumerate() {
            let within = match x.signum() {
                1 => y >= *x,
                -1 => y <= *x,
                _ => true,
            };
            if !within {
                return Err(l);
            }
        }
    }
    Ok(())
}

impl Ledgers {
    /// What remains of the child at `key`, or — never included yet — all of it.
    fn left(&self, key: &[u8], rows: &Lanes, declared: &Lanes) -> (Lanes, Lanes) {
        match self.remaining.get(key) {
            Some(held) => held.clone(),
            None => (rows.clone(), declared.clone()),
        }
    }

    /// The obstructions standing open in a domain.
    pub(crate) fn open_in(&self, domain: &[u8; 32]) -> usize {
        self.obstructions.get(domain).map_or(0, Vec::len)
    }

    /// The state written by its own coordinates; `None` where it holds
    /// nothing, so a court with no chained fold writes as it did.
    pub(crate) fn put(&self) -> Option<Vec<u8>> {
        use crate::transducer::address::{put_key_bytes, put_key_count};
        if self.folds.is_empty() && self.remaining.is_empty() && self.gauges.is_empty() && self.obstructions.is_empty() {
            return None;
        }
        let mut v = Vec::new();
        put_key_count(&mut v, self.folds.len());
        for (k, (tier, prefix, rows)) in self.folds.scoped() {
            put_key_bytes(&mut v, &k);
            for h in [tier, prefix, rows] {
                v.extend_from_slice(h);
            }
        }
        put_key_count(&mut v, self.remaining.len());
        for (k, (rows, declared)) in self.remaining.scoped() {
            put_key_bytes(&mut v, &k);
            rows.put_canonical(&mut v);
            declared.put_canonical(&mut v);
        }
        put_key_count(&mut v, self.gauges.len());
        for (domain, gauge) in self.gauges.scoped() {
            v.extend_from_slice(&domain);
            let steps = gauge.steps();
            put_key_count(&mut v, steps.len());
            for (at, parent, g) in &steps {
                put_key_bytes(&mut v, at);
                put_key_bytes(&mut v, parent);
                g.put_canonical(&mut v);
            }
        }
        put_key_count(&mut v, self.obstructions.len());
        for (domain, open) in self.obstructions.scoped() {
            v.extend_from_slice(&domain);
            put_key_count(&mut v, open.len());
            for (child, parent, h) in open {
                put_key_bytes(&mut v, child);
                put_key_bytes(&mut v, parent);
                h.put_canonical(&mut v);
            }
        }
        Some(v)
    }

    /// The state read back whole from its writing.
    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Ledgers> {
        use crate::transducer::address::{take_key_bytes, take_key_count};
        let key32 = |b: &[u8], p: &mut usize| -> Option<[u8; 32]> {
            let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
            *p += 32;
            Some(k)
        };
        let mut out = Ledgers::default();
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            let sealed = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
            out.folds.insert(&k, sealed);
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            let rows = Lanes::take_canonical(b, p)?;
            let declared = Lanes::take_canonical(b, p)?;
            out.remaining.insert(&k, (rows, declared));
        }
        for _ in 0..take_key_count(b, p)? {
            let domain = key32(b, p)?;
            let mut gauge = Gauge::default();
            for _ in 0..take_key_count(b, p)? {
                let at = take_key_bytes(b, p)?;
                let parent = take_key_bytes(b, p)?;
                gauge.hold_step(&at, parent, Rates::take_canonical(b, p)?);
            }
            out.gauges.insert(domain, gauge);
        }
        for _ in 0..take_key_count(b, p)? {
            let domain = key32(b, p)?;
            let open = (0..take_key_count(b, p)?)
                .map(|_| Some((take_key_bytes(b, p)?, take_key_bytes(b, p)?, Rates::take_canonical(b, p)?)))
                .collect::<Option<Vec<_>>>()?;
            out.obstructions.insert(domain, open);
        }
        Some(out)
    }
}

impl Kernel {
    /// AN ADMITTED FOLD'S SEALED COORDINATES (B′): held from its admission
    /// until a parent nets all of it — whatever the hot line has folded —
    /// or, for a fold admitted before the court held them, read off its
    /// receipt while the hot line still holds it.
    fn sealed_fold(&self, at: &crate::chronometry::Stamp) -> Option<Sealed> {
        if let Some(held) = self.ledgers.folds.get(&at.key()) {
            return Some(*held);
        }
        self.chain.node(at)?.cites.iter().find_map(|x| match x {
            Cite::Uplink { tier, prefix, rows } | Cite::Chained { tier, prefix, rows, .. } => Some((*tier, *prefix, *rows)),
            _ => None,
        })
    }

    /// 78.123 B — A FOLD THAT NETS CHILDREN, admitted once its signature,
    /// its binding and its aggregation have read (`admit_uplink`). Each
    /// child: a fold sealed on this court's line (`Cite::Uplink` or
    /// `Cite::Chained`) whose rows prove against its receipt's commitment,
    /// of the same domain, and whose included parts fit what remains of it.
    /// Any refusal refuses the whole fold, by the child's index. Then the
    /// rates edges read on the domain's gauge, the fold sealed by
    /// `Cite::Chained`, and the remainders and gauge kept.
    pub(crate) fn admit_chained(&mut self, f: &SignedFold, domain: [u8; 32]) -> Uplinked {
        let mut left: Keyed<(Lanes, Lanes)> = Keyed::new();
        let mut vertices: Vec<Sealed> = Vec::with_capacity(f.fold.children.len());
        for (i, c) in f.fold.children.iter().enumerate() {
            let index = match u32::try_from(i) {
                Ok(n) => n,
                Err(_) => return Uplinked::ChildRefused { index: u32::MAX, why: ChildRefusal::NotOnLine },
            };
            let refuse = |why| Uplinked::ChildRefused { index, why };
            let Some(sealed) = self.sealed_fold(&c.at) else { return refuse(ChildRefusal::NotOnLine) };
            let (child_tier, _, rows_hash) = sealed;
            vertices.push(sealed);
            if rows_hash != c.rows_hash() {
                return refuse(ChildRefusal::Mismatch);
            }
            if self.tier_domain.get(&child_tier) != Some(&domain) {
                return refuse(ChildRefusal::ForeignDomain);
            }
            let (Some(rows), Some(declared)) = (Lanes::of(&c.rows), Lanes::of(&c.declared)) else {
                return refuse(ChildRefusal::Mismatch);
            };
            let (Some(inc_rows), Some(inc_declared)) = (Lanes::of(&c.included_rows), Lanes::of(&c.included_declared)) else {
                return refuse(ChildRefusal::OverIncluded { lane: 0 });
            };
            let key = c.at.key();
            let (r, d) = match left.get(&key) {
                Some(x) => x.clone(),
                None => self.ledgers.left(&key, &rows, &declared),
            };
            if let Err(lane) = fits(&inc_rows, &r).and_then(|()| fits(&inc_declared, &d)) {
                return refuse(ChildRefusal::OverIncluded { lane });
            }
            let after = (
                inc_rows.inverse().and_then(|n| r.compose(&n)),
                inc_declared.inverse().and_then(|n| d.compose(&n)),
            );
            let (Some(r2), Some(d2)) = after else { return refuse(ChildRefusal::OverIncluded { lane: 0 }) };
            left.insert(&key, (r2, d2));
        }
        // THE RATES EDGES on the domain's gauge, read before the seal so the
        // seal knows whether it is provisional.
        let mut gauge = match self.ledgers.gauges.get(&domain) {
            Some(g) => g.clone(),
            None => Gauge::default(),
        };
        let mut found: Vec<Obstruction> = Vec::new();
        let parent = vertex(&f.fold.tier, &f.fold.prefix);
        for (c, (tier, prefix, _)) in f.fold.children.iter().zip(&vertices) {
            let Some(rate) = Rates::of(c.rate) else {
                return Uplinked::Discontinuous(crate::anastome::dialogue::uplink::Discontinuous::Rate { lane: 1 });
            };
            let child = vertex(tier, prefix);
            if let Gauged::Obstructed(h) = gauge.edge(&child, &parent, &rate) {
                found.push((child, parent.clone(), h));
            }
        }
        let Some(children) = f.fold.children_hash() else { return Uplinked::LineRefused };
        let rows = f.fold.rows_hash();
        let cite = Cite::Chained { tier: f.fold.tier, prefix: f.fold.prefix, rows, children };
        let Some(r) = self.seal_step(crate::chronometry::ActKind::Uplink, [0; 4], f.fold.tier, |_| vec![cite.clone()]) else {
            return Uplinked::LineRefused;
        };
        // This fold may be netted in its turn: held by its sealed coordinates.
        self.ledgers.folds.insert(r.stamp.key(), (f.fold.tier, f.fold.prefix, rows));
        let open_remainders = left.scoped().filter(|(_, (r, d))| !r.is_identity() || !d.is_identity()).count();
        for (k, v) in left.scoped() {
            // A child netted whole leaves the held folds; its empty remainder
            // stays, so its receipt, while the hot line still holds it, is
            // never netted again from the full.
            if v.0.is_identity() && v.1.is_identity() {
                self.ledgers.folds.remove(&k);
            }
            self.ledgers.remaining.insert(&k, v.clone());
        }
        self.ledgers.gauges.insert(domain, gauge);
        if !found.is_empty() {
            match self.ledgers.obstructions.get_mut(&domain) {
                Some(open) => open.append(&mut found),
                None => {
                    self.ledgers.obstructions.insert(domain, found);
                }
            }
        }
        self.mark_persist();
        let open = self.ledgers.open_in(&domain);
        #[expect(
            clippy::manual_unwrap_or,
            reason = "rule 11 — `unwrap_or` is the spelling this refuses; a count past u32 is past the reply's carrier and is named at the wall, never wrapped"
        )]
        let count = |n: usize| match u32::try_from(n) {
            Ok(c) => c,
            Err(_) => u32::MAX,
        };
        Uplinked::Chained {
            at: r.stamp,
            receipt: r.chain_hash,
            provisional: open > 0,
            obstructions: count(open),
            open_remainders: count(open_remainders),
        }
    }
}
