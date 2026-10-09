// 78.154 — THE MAP AND THE LEDGER on this court (MU-ECONOMY §7.1; 78.156):
// each universe's LEDGER chain, chained and sealed, never judged; its MAP of
// mouths and segments, posted by its reserve; the final points this court has
// held, which a crossing's freshness is counted on.
//
//! The court CHAINS and SEALS a ledger entry and never judges the state it
//! commits: both digests are stored and checked by no one here (Concordance
//! 1.13 — reconstructing the parent and its events against the root is the
//! universe's and the auditors' work). A point is FINAL IN THIS COURT'S VIEW
//! when it stands at the court's quorum-final point's origin at or below its
//! tick, and agrees with any final point this court held at that tick: the
//! door's shelf (78.152 TU-7) is read by the judge and handed in, and every
//! final point handed in is held on the LADDER — the advances a crossing's
//! freshness counts (78.156 §4) are the ladder's points between two ticks.

use super::Kernel;
use crate::anastome::dialogue::atlas::{
    dilation, local_record_id, local_throat_id, HeldEntry, HeldRecord, LedgerPage, LedgerRefusal, Ledgered, MapEntry, MapRecord, MapRefusal, Mapped, Place, Segment, SignedLedger, SignedMap,
    KIND_LEDGER, KIND_MAP,
};
use crate::anastome::dialogue::crossing::Edge;
use crate::chronometry::tuniversal::TPoint;
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use std::sync::Arc;

/// 78.154 (4) — A CLAIM DRAWN ON A STAKE: the fee held from it, the parts
/// it pays (each court and its part), the height its window closes at, and
/// the act it was drawn at — what each part's rows cite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Claim {
    pub(crate) parts: Vec<([u8; 32], [u64; 4])>,
    pub(crate) until: u64,
    pub(crate) at: Stamp,
    pub(crate) receipt: [u8; 32],
}

impl Claim {
    fn put(&self, v: &mut Vec<u8>) {
        put_key_count(v, self.parts.len());
        for (who, mu) in &self.parts {
            v.extend_from_slice(who);
            for m in mu {
                v.extend_from_slice(&m.to_le_bytes());
            }
        }
        v.extend_from_slice(&self.until.to_le_bytes());
        self.at.put_canonical(v);
        v.extend_from_slice(&self.receipt);
    }

    fn take(b: &[u8], p: &mut usize) -> Option<Claim> {
        use crate::anastome::dialogue::atlas::{key32, u64_at};
        let parts = (0..take_key_count(b, p)?)
            .map(|_| Some((key32(b, p)?, [u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?, u64_at(b, p)?])))
            .collect::<Option<Vec<_>>>()?;
        let until = u64_at(b, p)?;
        let at = Stamp::take_canonical(b, p)?;
        Some(Claim { parts, until, at, receipt: key32(b, p)? })
    }
}

/// THE ATLAS this court keeps — persisted on its own node (`LEAF_ATLAS`).
#[derive(Clone, Debug, Default)]
pub(crate) struct Atlas {
    /// Each universe's ledger chain in chain order, by its domain — shared
    /// with the published reading (78.151) until it grows.
    pub(crate) ledgers: Keyed<Arc<Vec<HeldEntry>>>,
    /// Each universe's map records, by `domain ‖ record id`.
    pub(crate) records: Keyed<HeldRecord>,
    /// Each crossing's reconciliation edge, by its hash.
    pub(crate) edges: Keyed<Edge>,
    /// THE LADDER: the final points this court has held, by `genesis ‖ epoch
    /// ‖ h` (big-endian) — ordered by tick within an origin.
    pub(crate) ladder: Keyed<TPoint>,
    /// Claims drawn on stakes, by `staker ‖ claim id`.
    pub(crate) claims: Keyed<Claim>,
    /// Stake acts taken, by `party ‖ nonce`: a resend moves nothing.
    pub(crate) spent: Keyed<()>,
    /// Moved at every entry sealed: the published reading's mark. Volatile.
    pub(crate) mark: u64,
}

fn ladder_key(p: &TPoint) -> Vec<u8> {
    let mut k = p.origin.genesis.to_vec();
    k.extend_from_slice(&p.origin.epoch.to_be_bytes());
    k.extend_from_slice(&p.h.to_be_bytes());
    k
}

fn origin_key(p: &TPoint) -> Vec<u8> {
    let mut k = p.origin.genesis.to_vec();
    k.extend_from_slice(&p.origin.epoch.to_be_bytes());
    k
}

pub(crate) fn record_key(domain: &[u8; 32], id: &[u8; 32]) -> Vec<u8> {
    let mut k = domain.to_vec();
    k.extend_from_slice(id);
    k
}

/// The id a mouth's record is held under on its universe's map.
pub(crate) fn mouth_id(domain: &[u8; 32], mouth: &[u8; 32], side: u8) -> [u8; 32] {
    MapRecord::Mouth { place: (*mouth, side), freshness: 0 }.id(domain)
}

/// 78.154 — A MAPPED PLACE, ONE READING FOR BOTH KINDS (a 78.128 mouth's
/// side, a local throat's end) on the map of the universe it is asked of:
/// its exact place in that universe's space where it reads, the keeper of
/// its throat, its freshness `N`, and the record held (its mapped point).
#[derive(Clone, Debug)]
pub(crate) struct MappedPlace {
    pub(crate) place: Option<crate::transducer::cga::Mv>,
    pub(crate) keeper: [u8; 32],
    pub(crate) freshness: u64,
    pub(crate) held: HeldRecord,
}

impl Atlas {
    pub(crate) fn is_empty(&self) -> bool {
        self.ledgers.is_empty() && self.records.is_empty() && self.edges.is_empty() && self.ladder.is_empty() && self.claims.is_empty() && self.spent.is_empty()
    }

    /// `[ledgers][records][edges][ladder][claims][spent]`, each counted, each
    /// by its own key.
    pub(crate) fn put(&self) -> Vec<u8> {
        let mut v = Vec::new();
        put_key_count(&mut v, self.ledgers.len());
        for (domain, chain) in self.ledgers.scoped() {
            v.extend_from_slice(&domain);
            put_key_count(&mut v, chain.len());
            for e in chain.iter() {
                e.put(&mut v);
            }
        }
        put_key_count(&mut v, self.records.len());
        for (k, r) in self.records.scoped() {
            put_key_bytes(&mut v, &k);
            r.put(&mut v);
        }
        put_key_count(&mut v, self.edges.len());
        for (k, e) in self.edges.scoped() {
            put_key_bytes(&mut v, &k);
            e.put(&mut v);
        }
        put_key_count(&mut v, self.ladder.len());
        for (_, p) in self.ladder.scoped() {
            p.put(&mut v);
        }
        put_key_count(&mut v, self.claims.len());
        for (k, c) in self.claims.scoped() {
            put_key_bytes(&mut v, &k);
            c.put(&mut v);
        }
        put_key_count(&mut v, self.spent.len());
        for (k, _) in self.spent.scoped() {
            put_key_bytes(&mut v, &k);
        }
        v
    }

    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Atlas> {
        use crate::anastome::dialogue::atlas::key32;
        let mut a = Atlas::default();
        for _ in 0..take_key_count(b, p)? {
            let domain = key32(b, p)?;
            let chain = (0..take_key_count(b, p)?).map(|_| HeldEntry::take(domain, b, p)).collect::<Option<Vec<_>>>()?;
            a.ledgers.insert(domain, Arc::new(chain));
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            a.records.insert(k, HeldRecord::take(b, p)?);
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            a.edges.insert(k, Edge::take(b, p)?);
        }
        for _ in 0..take_key_count(b, p)? {
            let t = TPoint::take(b, p)?;
            a.ladder.insert(ladder_key(&t), t);
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            a.claims.insert(k, Claim::take(b, p)?);
        }
        for _ in 0..take_key_count(b, p)? {
            a.spent.insert(take_key_bytes(b, p)?, ());
        }
        Some(a)
    }

    /// A universe's latest entry.
    pub(crate) fn head(&self, domain: &[u8; 32]) -> Option<&HeldEntry> {
        self.ledgers.get(domain)?.iter().next_back()
    }

    /// The entries after `u`, in chain order, as many as fit `budget` bytes
    /// written; and whether more stand.
    pub(crate) fn page(chain: Option<&Arc<Vec<HeldEntry>>>, after: u64, budget: usize) -> LedgerPage {
        let Some(chain) = chain else { return LedgerPage::default() };
        let from = chain.partition_point(|e| e.entry.u <= after);
        let mut page = LedgerPage::default();
        let mut used = 0usize;
        for e in chain.get(from..).into_iter().flatten() {
            let mut w = Vec::new();
            e.put(&mut w);
            match used.checked_add(w.len()) {
                Some(n) if n <= budget => {
                    used = n;
                    page.entries.push(e.clone());
                }
                _ => {
                    page.more = true;
                    break;
                }
            }
        }
        page
    }

    /// THE ADVANCES between two final points: the ladder's points of their
    /// one origin above `from`'s tick and at or below `to`'s. `None` where
    /// they stand at two origins (a re-originated clock is no distance).
    pub(crate) fn advances(&self, from: &TPoint, to: &TPoint) -> Option<u64> {
        if from.origin != to.origin {
            return None;
        }
        let fiber = self.ladder.fiber_at(&origin_key(to))?;
        let n = fiber.filter(|(_, p)| p.h > from.h && p.h <= to.h).count();
        u64::try_from(n).ok()
    }
}

impl Kernel {
    /// A FINAL POINT HANDED IN from the door's shelf, held on the ladder.
    pub(crate) fn note_final(&mut self, p: TPoint) {
        let k = ladder_key(&p);
        if !self.atlas.ladder.contains(&k) {
            self.atlas.ladder.insert(k, p);
            self.mark_persist();
        }
    }

    /// WHETHER `p` IS FINAL IN THIS COURT'S VIEW, `fin` the door's final
    /// point now (held on the ladder first).
    pub(crate) fn final_in_view(&mut self, p: &TPoint, fin: Option<TPoint>) -> bool {
        let Some(f) = fin else { return false };
        self.note_final(f);
        let held = self.atlas.ladder.get(&ladder_key(p));
        p.origin == f.origin && p.h <= f.h && held.is_none_or(|h| h == p)
    }

    /// The ladder let go below every universe's latest entry: a point no
    /// universe's freshness can count from again. The newest point stays.
    fn prune_ladder(&mut self) {
        let latest: Vec<TPoint> = self.atlas.ledgers.scoped().filter_map(|(_, c)| c.iter().next_back().map(|e| e.entry.point)).collect();
        // The ladder's key order is its points' order within an origin.
        let newest = self.atlas.ladder.scoped().map(|(k, _)| k).last();
        let gone: Vec<Vec<u8>> = self
            .atlas
            .ladder
            .scoped()
            .filter(|(k, p)| Some(k) != newest.as_ref() && !latest.is_empty() && latest.iter().all(|l| l.origin == p.origin && l.h > p.h))
            .map(|(k, _)| k)
            .collect();
        for k in &gone {
            self.atlas.ladder.remove(k);
        }
    }

    /// The signer of a domain's act: its SITTING RESERVE, or the refusal —
    /// `NotReserve` where another key of the domain (its founder, a tier)
    /// signed it or it has no reserve, `Unsigned` where no key of it did.
    fn reserve_signed(&self, domain: &[u8; 32], bytes: &[u8], sig: &[u8; 64]) -> Result<[u8; 32], bool> {
        let reserve = self.reserve_of(domain);
        if let Some(r) = reserve.filter(|r| crate::court::identity::verify(r, bytes, sig)) {
            return Ok(r);
        }
        let tiers = self.domains.get(domain).cloned().into_iter().flatten();
        let of_domain = std::iter::once(*domain).chain(tiers).any(|k| crate::court::identity::verify(&k, bytes, sig));
        Err(of_domain || reserve.is_none())
    }

    /// 78.154 — A LEDGER ENTRY CHAINED AND SEALED: the domain founded here,
    /// signed by its sitting reserve, its point final in this court's view
    /// and not below the head's, its parent the head (zero first), its `u`
    /// above the head's, and every crossing it applied pending into it. The
    /// court never judges what it commits.
    pub fn ledger(&mut self, s: &SignedLedger, fin: Option<TPoint>) -> Ledgered {
        use LedgerRefusal as R;
        let refuse = Ledgered::Refused;
        let e = &s.entry;
        if self.domains.get(&e.domain).is_none() {
            return refuse(R::UnknownDomain);
        }
        let reserve = match self.reserve_signed(&e.domain, &e.signed_bytes(), &s.sig) {
            Ok(r) => r,
            Err(true) => return refuse(R::NotReserve),
            Err(false) => return refuse(R::Unsigned),
        };
        if !self.final_in_view(&e.point, fin) {
            return refuse(R::NotFinal);
        }
        let head = self.atlas.head(&e.domain).cloned();
        let expected = head.as_ref().map_or([0; 32], |h| h.hash);
        if e.parent != expected {
            return refuse(R::Parent);
        }
        if head.as_ref().is_some_and(|h| e.u <= h.entry.u) {
            return refuse(R::NotAbove);
        }
        if head.as_ref().is_some_and(|h| h.entry.point.origin == e.point.origin && e.point.h < h.entry.point.h) {
            return refuse(R::Behind);
        }
        for (i, (c, ua)) in e.applied.iter().enumerate() {
            let pending = self.atlas.edges.get(c).is_some_and(|x| x.to == e.domain && x.u_applied.is_none());
            let twice = e.applied.iter().take(i).any(|(d, _)| d == c);
            if !pending || twice || *ua > e.u {
                return refuse(R::Applied);
            }
        }
        let hash = e.hash();
        let cite = Cite::Stated { domain: e.domain, kind: KIND_LEDGER, body: hash };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], reserve, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let Some(court) = fin else { return refuse(R::NotFinal) };
        let dilation = head.as_ref().filter(|h| h.entry.point.origin == e.point.origin).and_then(|h| dilation((h.entry.u, e.u), (h.entry.point.h, e.point.h)));
        let held = HeldEntry { entry: e.clone(), hash, sealed: r.stamp.h, court, dilation };
        match self.atlas.ledgers.get_mut(&e.domain) {
            Some(chain) => Arc::make_mut(chain).push(held),
            None => {
                self.atlas.ledgers.insert(e.domain, Arc::new(vec![held]));
            }
        }
        // 78.154 (3) — where the receiver applied each crossing: the edge
        // reads its residual from here, never before.
        for (c, ua) in &e.applied {
            if let Some(edge) = self.atlas.edges.get_mut(c) {
                edge.u_applied = Some(*ua);
            }
        }
        self.atlas.mark = self.atlas.mark.wrapping_add(1);
        self.prune_ladder();
        self.mark_persist();
        Ledgered::Sealed { entry: hash, h: r.stamp.h, point: court }
    }

    /// 78.154 — A UNIVERSE'S LEDGER after `u`, as many entries as fit one
    /// frame. Sealing nothing.
    pub fn ledger_page(&self, domain: &[u8; 32], after: u64) -> LedgerPage {
        Atlas::page(self.atlas.ledgers.get(domain), after, ledger_budget())
    }

    /// 78.156 §2 — A UNIVERSE'S DILATION per entry segment, `Δu/Δh` exact,
    /// in chain order (`None` where `h` did not move or the clock
    /// re-originated). A reading; the hypha records, never steers.
    pub fn dilations(&self, domain: &[u8; 32]) -> Vec<(u64, Option<(u64, u64)>)> {
        self.atlas.ledgers.get(domain).map_or(Vec::new(), |c| c.iter().map(|e| (e.entry.u, e.dilation)).collect())
    }

    /// 78.154 — the reconciliation edge of a crossing, as held.
    pub fn crossing_edge(&self, crossing: &[u8; 32]) -> Option<Edge> {
        self.atlas.edges.get(crossing).cloned()
    }

    /// The mouth pair `mouth` open as a tube, and the domain its `side`
    /// stands in.
    pub(crate) fn mouth_side(&self, mouth: &[u8; 32], side: u8) -> Option<(crate::anastome::dialogue::handoff::Mouth, [u8; 32])> {
        let held = self.handoffs.mouths.get(mouth)?;
        if held.state != crate::anastome::dialogue::handoff::State::Accepted || held.mouth.tube.is_none() {
            return None;
        }
        let tier = match side {
            0 => held.mouth.from_tier,
            1 => held.mouth.to_tier,
            _ => return None,
        };
        Some((held.mouth.clone(), *self.tier_domain.get(&tier)?))
    }

    /// A LOCAL THROAT `local` (its derived mouth id) on `domain`'s map, as
    /// held.
    pub(crate) fn local_throat(&self, domain: &[u8; 32], local: &[u8; 32]) -> Option<&HeldRecord> {
        let held = self.atlas.records.get(&record_key(domain, &local_record_id(domain, local)))?;
        match held.record {
            MapRecord::LocalThroat { .. } => Some(held),
            MapRecord::Mouth { .. } | MapRecord::Segment { .. } => None,
        }
    }

    /// 78.154 — THE MAPPED PLACE `(id, side)` on `domain`'s map, either kind:
    /// a mouth pair's side mapped there (its place off the tube, its keeper
    /// the mouth's), or a local throat's end (its place off its frame in the
    /// universe's declared geometry, its keeper inline). `None` where it is
    /// not mapped there.
    pub(crate) fn mapped_place(&self, domain: &[u8; 32], (id, side): &Place) -> Option<MappedPlace> {
        if *side > 1 {
            return None;
        }
        if let Some(held) = self.atlas.records.get(&record_key(domain, &mouth_id(domain, id, *side))) {
            let MapRecord::Mouth { freshness, .. } = held.record else { return None };
            let mouth = &self.handoffs.mouths.get(id)?.mouth;
            let place = mouth.places().map(|(f, t)| match side {
                0 => f,
                _ => t,
            });
            return Some(MappedPlace { place, keeper: mouth.keeper, freshness, held: held.clone() });
        }
        let held = self.local_throat(domain, id)?;
        let MapRecord::LocalThroat { keeper, freshness, .. } = held.record else { return None };
        let place = self.geometry_of(domain).and_then(|g| held.record.local_places(g)).map(|(p0, p1)| match side {
            0 => p0,
            _ => p1,
        });
        Some(MappedPlace { place, keeper, freshness, held: held.clone() })
    }

    /// THE UNIVERSE A PLACE STANDS IN: a mouth pair's side by its tier's
    /// domain (an open tube), else a local throat of `local_to`'s map — whose
    /// two ends both stand in it.
    pub(crate) fn place_domain(&self, local_to: &[u8; 32], (id, side): &Place) -> Option<[u8; 32]> {
        match self.mouth_side(id, *side) {
            Some((_, d)) => Some(d),
            None => (*side <= 1 && self.local_throat(local_to, id).is_some()).then_some(*local_to),
        }
    }

    /// 78.154 — A MAP RECORD POSTED by the universe's sitting reserve at a
    /// final point: a MOUTH of an open tube on its side in this universe;
    /// a road (tube, core) between two of its mapped places, no keeper; a
    /// THROAT between one mouth's two sides, its own side mapped, its keeper
    /// the mouth's; a LOCAL THROAT, each frame a versor whose place reads in
    /// the universe's declared geometry (its two ends then mapped places of
    /// this universe, a throat segment between them kept by its keeper).
    /// Superseded only by a later act on the same id.
    pub fn map(&mut self, s: &SignedMap, fin: Option<TPoint>) -> Mapped {
        use MapRefusal as R;
        let refuse = Mapped::Refused;
        let a = &s.act;
        if self.domains.get(&a.domain).is_none() {
            return refuse(R::UnknownDomain);
        }
        let reserve = match self.reserve_signed(&a.domain, &a.signed_bytes(), &s.sig) {
            Ok(r) => r,
            Err(true) => return refuse(R::NotReserve),
            Err(false) => return refuse(R::Unsigned),
        };
        if !self.final_in_view(&a.point, fin) {
            return refuse(R::NotFinal);
        }
        let mapped = |k: &Kernel, place: &Place| k.mapped_place(&a.domain, place).is_some();
        match &a.record {
            MapRecord::Mouth { place, .. } => {
                if place.1 > 1 {
                    return refuse(R::Malformed);
                }
                let Some((_, d)) = self.mouth_side(&place.0, place.1) else { return refuse(R::NoMouth) };
                if d != a.domain {
                    return refuse(R::NotThisSide);
                }
            }
            MapRecord::LocalThroat { frames: (f0, f1), .. } => {
                if [f0, f1].iter().any(|f| f.is_zero() || !crate::transducer::cga::is_versor(f)) {
                    return refuse(R::NotVersor);
                }
                let Some(space) = self.geometry_of(&a.domain) else { return refuse(R::NoGeometry) };
                if a.record.local_places(space).is_none() {
                    return refuse(R::Unplaced);
                }
            }
            MapRecord::Segment { kind, a: x, b: y, span, keeper } => {
                if !crate::anastome::dialogue::universe::positive(*span) || x.1 > 1 || y.1 > 1 || x == y {
                    return refuse(R::Malformed);
                }
                match kind {
                    // A local throat's own throat: its two ends, both of this
                    // universe and mapped with it, its keeper the record's.
                    Segment::Throat if x.0 == y.0 && self.local_throat(&a.domain, &x.0).is_some() => {
                        if self.mapped_place(&a.domain, x).map(|m| m.keeper) != *keeper {
                            return refuse(R::Keeper);
                        }
                    }
                    Segment::Throat => {
                        if x.0 != y.0 {
                            return refuse(R::Unmapped);
                        }
                        let (Some((m, dx)), Some((_, dy))) = (self.mouth_side(&x.0, x.1), self.mouth_side(&y.0, y.1)) else { return refuse(R::NoMouth) };
                        let own = match (dx == a.domain, dy == a.domain) {
                            (true, _) => x,
                            (false, true) => y,
                            (false, false) => return refuse(R::NotThisSide),
                        };
                        if !mapped(self, own) {
                            return refuse(R::Unmapped);
                        }
                        if *keeper != Some(m.keeper) {
                            return refuse(R::Keeper);
                        }
                    }
                    Segment::Tube | Segment::Core => {
                        if !mapped(self, x) || !mapped(self, y) {
                            return refuse(R::Unmapped);
                        }
                        if keeper.is_some() {
                            return refuse(R::Keeper);
                        }
                    }
                }
            }
        }
        let id = a.record.id(&a.domain);
        let key = record_key(&a.domain, &id);
        if self.atlas.records.get(&key).is_some_and(|h| h.point.origin == a.point.origin && a.point.h < h.point.h) {
            return refuse(R::Behind);
        }
        let cite = Cite::Stated { domain: a.domain, kind: KIND_MAP, body: crate::chronometry::payload_hash(&a.signed_bytes()) };
        let Some(r) = self.seal_step(ActKind::Domain, [0; 4], reserve, |_| vec![cite.clone()]) else { return refuse(R::LineRefused) };
        let Some(court) = fin else { return refuse(R::NotFinal) };
        self.atlas.records.insert(key, HeldRecord { record: a.record.clone(), point: a.point, sealed: r.stamp.h });
        self.mark_persist();
        Mapped::Sealed { record: id, h: r.stamp.h, point: court }
    }

    /// 78.154 — A UNIVERSE'S MAP as read: every record, each mouth with its
    /// place in the universe's own space read off its tube, each local
    /// throat with its two ends' places read off its frames — every place
    /// through the one mapped-place reading. Sealing nothing.
    pub fn map_reading(&self, domain: &[u8; 32]) -> Vec<MapEntry> {
        let Some(fiber) = self.atlas.records.fiber_at(domain) else { return Vec::new() };
        let place = |p: &Place| self.mapped_place(domain, p).and_then(|m| m.place);
        fiber
            .map(|(_, held)| {
                let places = match &held.record {
                    MapRecord::Mouth { place: p, .. } => place(p).into_iter().collect(),
                    MapRecord::LocalThroat { throat, .. } => {
                        let local = local_throat_id(domain, throat);
                        match (place(&(local, 0)), place(&(local, 1))) {
                            (Some(p0), Some(p1)) => vec![p0, p1],
                            _ => Vec::new(),
                        }
                    }
                    MapRecord::Segment { .. } => Vec::new(),
                };
                MapEntry { held: held.clone(), places }
            })
            .collect()
    }
}

/// The bytes a ledger page may take: the frame's body less its header.
pub(crate) fn ledger_budget() -> usize {
    match crate::network::wire::frame_budget().checked_sub(64) {
        Some(b) => b,
        None => 0,
    }
}
