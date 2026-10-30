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
//!
//! 78.125 — every admitted fold is RECORDED for its challenge window (2(c),
//! `cognition::challenge`): the span of its tier line it nets, what it
//! netted of each child, who netted it, and how it stands. The edges are
//! held in the order sealed; the gauge and the obstructions are re-derived
//! from them (law 12), so a reversed fold's edges leave and the potential is
//! solved again.

use super::Kernel;
use crate::anastome::dialogue::uplink::{Child, ChildRefusal, FoldAt, SignedFold, TierFold, Uplinked};
use crate::chronometry::{Cite, Stamp};
use crate::network::nerve::Keyed;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use crate::transducer::cech::coefficient::{Coefficient, Lanes, Rates};
use crate::transducer::cech::cover::{Gauge, Gauged};

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

/// ONE RATES EDGE, as sealed: the child fold's vertex, the parent's, the
/// rate, and the stamp key of the fold that sealed it.
#[derive(Clone, Debug)]
pub(crate) struct Edge {
    pub(crate) child: Vec<u8>,
    pub(crate) parent: Vec<u8>,
    pub(crate) rate: Rates,
    pub(crate) by: Vec<u8>,
}

/// ONE OBSTRUCTION: the edge that closed an arbitrage cycle, the cycle's
/// exact holonomy per lane, and the fold whose seal closed it.
#[derive(Clone, Debug)]
pub(crate) struct Obstruction {
    pub(crate) child: Vec<u8>,
    pub(crate) parent: Vec<u8>,
    pub(crate) holonomy: Rates,
    pub(crate) by: Vec<u8>,
}

impl Obstruction {
    /// Its key among the discharges: the closing fold, then the child.
    pub(crate) fn key(&self) -> Vec<u8> {
        discharge_key(&self.by, &self.child)
    }
}

pub(crate) fn discharge_key(by: &[u8], child: &[u8]) -> Vec<u8> {
    let mut k = Vec::with_capacity(by.len() + child.len() + 8);
    put_key_bytes(&mut k, by);
    k.extend_from_slice(child);
    k
}

/// HOW A RECORDED FOLD STANDS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Standing {
    Open,
    /// Falsified — by replay or by default — and reversed.
    Falsified,
    /// A fold it netted was reversed; its own netting returned. It must refile.
    Unwound,
}

impl Standing {
    fn byte(self) -> u8 {
        match self {
            Standing::Open => 0,
            Standing::Falsified => 1,
            Standing::Unwound => 2,
        }
    }

    fn of(b: u8) -> Option<Standing> {
        [Standing::Open, Standing::Falsified, Standing::Unwound].into_iter().find(|s| s.byte() == b)
    }
}

/// 78.125 — A FOLD RECORDED FOR ITS WINDOW: its coordinates, the span of
/// its tier line it nets (`from` → `prefix`), its children's commitment,
/// the height it sealed at, what it netted of each child (by the child's
/// stamp key), the folds that netted it, a demand's height, an upheld
/// prefix's content hash, and its standing.
#[derive(Clone, Debug)]
pub(crate) struct FoldRecord {
    pub(crate) tier: [u8; 32],
    pub(crate) domain: [u8; 32],
    pub(crate) from: [u8; 32],
    pub(crate) prefix: [u8; 32],
    pub(crate) rows: [u8; 32],
    pub(crate) children: Option<[u8; 32]>,
    pub(crate) sealed: u64,
    pub(crate) nets: Vec<(Vec<u8>, Lanes, Lanes)>,
    pub(crate) parents: Vec<Vec<u8>>,
    pub(crate) demanded: Option<u64>,
    pub(crate) upheld: Option<[u8; 32]>,
    pub(crate) standing: Standing,
}

/// THE LEDGER TREES' STATE on this court — persisted on its node
/// (`LEAF_LEDGERS`, `LEAF_CHALLENGES`), the gauges and obstructions
/// re-derived from the edges.
#[derive(Default)]
pub(crate) struct Ledgers {
    /// Per admitted fold, keyed by its receipt's stamp: its sealed
    /// coordinates — dropped once a parent has netted all of it, so what is
    /// held is the folds with value still in flight.
    pub(crate) folds: Keyed<Sealed>,
    /// Per child, keyed by its receipt's stamp: what remains of its rows and
    /// of its declared, not yet included by a parent. Empty is consumed.
    pub(crate) remaining: Keyed<(Lanes, Lanes)>,
    /// 78.125 — every fold admitted since 2(c), keyed by its receipt's stamp.
    pub(crate) records: Keyed<FoldRecord>,
    /// 78.125 — per tier, the prefix head of its last admitted fold: where
    /// its next fold's span begins.
    pub(crate) heads: Keyed<[u8; 32]>,
    /// 78.125 — per tier, the stamp key of the fold that set its head:
    /// re-derived from the records wherever they are restored, never
    /// persisted (law 12).
    pub(crate) head_at: Keyed<Vec<u8>>,
    /// Per domain: its rates edges, in the order sealed.
    pub(crate) edges: Keyed<Vec<Edge>>,
    /// 78.125 — FINAL FOLDS, PRUNED TO A STUB: a recorded fold read final
    /// keeps only its sealed coordinates and whether it was upheld, and only
    /// while it has value in flight (still nettable) or a fold that netted it
    /// stands unfinished (whose finality reads it, and whose reversal would
    /// return parts to it). A final fold is never reversed — its window has
    /// closed and every fold it nets is final — so nothing else is needed.
    pub(crate) finals: Keyed<(Sealed, bool)>,
    /// 78.125 (ruled) — each fold whose self-demand was
    /// withdrawn, by its stamp key: the height of the withdrawal, from which
    /// its evidence window stands reopened `W` — once per fold.
    pub(crate) withdrawn: Keyed<u64>,
    /// 78.125 — each standing demand, by its fold's stamp key: the demand's
    /// receipt (its stamp key) and its filer — so the same filer's resend is
    /// answered with the original and nothing is sealed twice.
    pub(crate) demands: Keyed<(Vec<u8>, [u8; 32])>,
    /// 78.125 — the arbitrages booked, keyed by closing fold and child: the
    /// holder the gain accrues to and its amount on G.
    pub(crate) discharged: Keyed<([u8; 32], i128)>,
    /// Per domain: the rates gauge over its folds — re-derived, never kept.
    pub(crate) gauges: Keyed<Gauge<Rates>>,
    /// Per domain: every obstruction its edges close — re-derived.
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

fn key32(b: &[u8], p: &mut usize) -> Option<[u8; 32]> {
    let k: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
    *p += 32;
    Some(k)
}

fn take_u64(b: &[u8], p: &mut usize) -> Option<u64> {
    let v = u64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
    *p += 8;
    Some(v)
}

fn take_flag(b: &[u8], p: &mut usize) -> Option<bool> {
    let f = match *b.get(*p)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    *p += 1;
    Some(f)
}

impl Ledgers {
    /// What remains of the child at `key`, or — never included yet — all of it.
    fn left(&self, key: &[u8], rows: &Lanes, declared: &Lanes) -> (Lanes, Lanes) {
        match self.remaining.get(key) {
            Some(held) => held.clone(),
            None => (rows.clone(), declared.clone()),
        }
    }

    /// The obstructions standing open in a domain: closed, and not booked.
    pub(crate) fn open_in(&self, domain: &[u8; 32]) -> Vec<&Obstruction> {
        match self.obstructions.get(domain) {
            Some(all) => all.iter().filter(|o| !self.discharged.contains(&o.key())).collect(),
            None => Vec::new(),
        }
    }

    /// THE DOMAIN'S GAUGE AND OBSTRUCTIONS, solved again from its edges in
    /// the order sealed.
    pub(crate) fn rederive(&mut self, domain: &[u8; 32]) {
        let mut gauge = Gauge::default();
        let mut found = Vec::new();
        for e in self.edges.get(domain).into_iter().flatten() {
            if let Gauged::Obstructed(h) = gauge.edge(&e.child, &e.parent, &e.rate) {
                found.push(Obstruction { child: e.child.clone(), parent: e.parent.clone(), holonomy: h, by: e.by.clone() });
            }
        }
        self.gauges.insert(domain, gauge);
        self.obstructions.insert(domain, found);
    }

    /// The held folds and remainders (`LEAF_LEDGERS`); `None` where it holds
    /// none. B′'s gauge and obstruction sections are retired: written empty,
    /// and a leaf that carries any is refused (they are re-derived now).
    pub(crate) fn put(&self) -> Option<Vec<u8>> {
        if self.folds.is_empty() && self.remaining.is_empty() {
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
        // The retired sections, empty.
        put_key_count(&mut v, 0);
        put_key_count(&mut v, 0);
        Some(v)
    }

    /// The held folds and remainders read back whole.
    pub(crate) fn take(b: &[u8], p: &mut usize) -> Option<Ledgers> {
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
        // B′'s retired sections. Its gauge steps are a spanning forest — each
        // step an edge the gauge joined by — so they are read back as edges,
        // sealed by no recorded fold (`by` empty: a fold admitted before 2(c)
        // has no window), and the potential they re-derive is the one held.
        // An obstruction held there closed a cycle the forest no longer
        // shows; it cannot be re-derived, and is refused rather than dropped.
        for _ in 0..take_key_count(b, p)? {
            let domain = key32(b, p)?;
            let steps = (0..take_key_count(b, p)?)
                .map(|_| {
                    let (child, parent) = (take_key_bytes(b, p)?, take_key_bytes(b, p)?);
                    Some(Edge { child, parent, rate: Rates::take_canonical(b, p)?, by: Vec::new() })
                })
                .collect::<Option<Vec<_>>>()?;
            out.edges.insert(domain, steps);
            out.rederive(&domain);
        }
        if take_key_count(b, p)? != 0 {
            return None;
        }
        Some(out)
    }

    /// The records, tier heads, edges and discharges (`LEAF_CHALLENGES`);
    /// `None` where none is held.
    pub(crate) fn put_challenges(&self) -> Option<Vec<u8>> {
        if self.records.is_empty()
            && self.heads.is_empty()
            && self.edges.is_empty()
            && self.discharged.is_empty()
            && self.finals.is_empty()
            && self.withdrawn.is_empty()
            && self.demands.is_empty()
        {
            return None;
        }
        let mut v = Vec::new();
        put_key_count(&mut v, self.records.len());
        for (k, r) in self.records.scoped() {
            put_key_bytes(&mut v, &k);
            for h in [&r.tier, &r.domain, &r.from, &r.prefix, &r.rows] {
                v.extend_from_slice(h);
            }
            match &r.children {
                None => v.push(0),
                Some(c) => {
                    v.push(1);
                    v.extend_from_slice(c);
                }
            }
            v.extend_from_slice(&r.sealed.to_le_bytes());
            put_key_count(&mut v, r.nets.len());
            for (child, rows, declared) in &r.nets {
                put_key_bytes(&mut v, child);
                rows.put_canonical(&mut v);
                declared.put_canonical(&mut v);
            }
            put_key_count(&mut v, r.parents.len());
            for parent in &r.parents {
                put_key_bytes(&mut v, parent);
            }
            match r.demanded {
                None => v.push(0),
                Some(h) => {
                    v.push(1);
                    v.extend_from_slice(&h.to_le_bytes());
                }
            }
            match &r.upheld {
                None => v.push(0),
                Some(h) => {
                    v.push(1);
                    v.extend_from_slice(h);
                }
            }
            v.push(r.standing.byte());
        }
        put_key_count(&mut v, self.heads.len());
        for (tier, head) in self.heads.scoped() {
            v.extend_from_slice(&tier);
            v.extend_from_slice(head);
        }
        put_key_count(&mut v, self.edges.len());
        for (domain, edges) in self.edges.scoped() {
            v.extend_from_slice(&domain);
            put_key_count(&mut v, edges.len());
            for e in edges {
                put_key_bytes(&mut v, &e.child);
                put_key_bytes(&mut v, &e.parent);
                e.rate.put_canonical(&mut v);
                put_key_bytes(&mut v, &e.by);
            }
        }
        put_key_count(&mut v, self.discharged.len());
        for (k, (holder, amount)) in self.discharged.scoped() {
            put_key_bytes(&mut v, &k);
            v.extend_from_slice(holder);
            v.extend_from_slice(&amount.to_le_bytes());
        }
        // The final stubs, after the sections 6ef3b165 wrote: a leaf ending
        // before them carries none.
        put_key_count(&mut v, self.finals.len());
        for (k, ((tier, prefix, rows), upheld)) in self.finals.scoped() {
            put_key_bytes(&mut v, &k);
            for h in [tier, prefix, rows] {
                v.extend_from_slice(h);
            }
            v.push(u8::from(*upheld));
        }
        // The withdrawals, after the stubs.
        put_key_count(&mut v, self.withdrawn.len());
        for (k, h) in self.withdrawn.scoped() {
            put_key_bytes(&mut v, &k);
            v.extend_from_slice(&h.to_le_bytes());
        }
        // The standing demands, after the withdrawals.
        put_key_count(&mut v, self.demands.len());
        for (k, (at, filer)) in self.demands.scoped() {
            put_key_bytes(&mut v, &k);
            put_key_bytes(&mut v, at);
            v.extend_from_slice(filer);
        }
        Some(v)
    }

    /// The records, heads, edges and discharges read back whole into `self`,
    /// and every domain's gauge re-derived.
    pub(crate) fn take_challenges(&mut self, b: &[u8], p: &mut usize) -> Option<()> {
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            let (tier, domain, from, prefix, rows) = (key32(b, p)?, key32(b, p)?, key32(b, p)?, key32(b, p)?, key32(b, p)?);
            let children = match take_flag(b, p)? {
                false => None,
                true => Some(key32(b, p)?),
            };
            let sealed = take_u64(b, p)?;
            let nets = (0..take_key_count(b, p)?)
                .map(|_| Some((take_key_bytes(b, p)?, Lanes::take_canonical(b, p)?, Lanes::take_canonical(b, p)?)))
                .collect::<Option<Vec<_>>>()?;
            let parents = (0..take_key_count(b, p)?).map(|_| take_key_bytes(b, p)).collect::<Option<Vec<_>>>()?;
            let demanded = match take_flag(b, p)? {
                false => None,
                true => Some(take_u64(b, p)?),
            };
            let upheld = match take_flag(b, p)? {
                false => None,
                true => Some(key32(b, p)?),
            };
            let standing = Standing::of(*b.get(*p)?)?;
            *p += 1;
            let record = FoldRecord { tier, domain, from, prefix, rows, children, sealed, nets, parents, demanded, upheld, standing };
            self.records.insert(&k, record);
        }
        for _ in 0..take_key_count(b, p)? {
            let (tier, head) = (key32(b, p)?, key32(b, p)?);
            self.heads.insert(tier, head);
        }
        let mut domains = Vec::new();
        for _ in 0..take_key_count(b, p)? {
            let domain = key32(b, p)?;
            let edges = (0..take_key_count(b, p)?)
                .map(|_| {
                    let (child, parent) = (take_key_bytes(b, p)?, take_key_bytes(b, p)?);
                    let rate = Rates::take_canonical(b, p)?;
                    Some(Edge { child, parent, rate, by: take_key_bytes(b, p)? })
                })
                .collect::<Option<Vec<_>>>()?;
            self.edges.insert(domain, edges);
            domains.push(domain);
        }
        for _ in 0..take_key_count(b, p)? {
            let k = take_key_bytes(b, p)?;
            let holder = key32(b, p)?;
            let amount = i128::from_le_bytes(b.get(*p..*p + 16)?.try_into().ok()?);
            *p += 16;
            self.discharged.insert(&k, (holder, amount));
        }
        // The final stubs — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = take_key_bytes(b, p)?;
                let sealed = (key32(b, p)?, key32(b, p)?, key32(b, p)?);
                let upheld = take_flag(b, p)?;
                self.finals.insert(&k, (sealed, upheld));
            }
        }
        // The withdrawals — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = take_key_bytes(b, p)?;
                let h = take_u64(b, p)?;
                self.withdrawn.insert(&k, h);
            }
        }
        // The standing demands — absent from a leaf written before them.
        if *p < b.len() {
            for _ in 0..take_key_count(b, p)? {
                let k = take_key_bytes(b, p)?;
                let at = take_key_bytes(b, p)?;
                let filer = key32(b, p)?;
                self.demands.insert(&k, (at, filer));
            }
        }
        for d in &domains {
            self.rederive(d);
        }
        // Each tier's head re-derived to the fold that set it: the latest
        // recorded fold of that tier standing at its head.
        let mut set_by: Keyed<(u64, Vec<u8>)> = Keyed::new();
        for (k, r) in self.records.scoped() {
            if self.heads.get(&r.tier) != Some(&r.prefix) {
                continue;
            }
            if set_by.get(&r.tier).is_none_or(|(h, _)| r.sealed >= *h) {
                set_by.insert(r.tier, (r.sealed, k));
            }
        }
        for (tier, (_, k)) in set_by.scoped() {
            self.head_at.insert(&tier, k.clone());
        }
        Some(())
    }
}

/// A count past u32 is past a reply's carrier: named at the wall, never
/// wrapped (rule 11).
#[expect(
    clippy::manual_unwrap_or,
    reason = "rule 11 — `unwrap_or` is the spelling this refuses; a count past u32 is past the reply's carrier and is named at the wall, never wrapped"
)]
pub(crate) fn count(n: usize) -> u32 {
    match u32::try_from(n) {
        Ok(c) => c,
        Err(_) => u32::MAX,
    }
}

/// 78.138 — WHERE A PLANNED CHILD WILL STAND: a fold already on the court
/// (its key), or one carried in the fold being admitted (its path inside it)
/// — the receipt's stamp is known only once the fold is sealed.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Place {
    On(Vec<u8>),
    In(Vec<u16>),
}

impl Place {
    /// Its key while planning: the two kinds never meet.
    fn plan_key(&self) -> Vec<u8> {
        match self {
            Place::On(k) => {
                let mut v = vec![0];
                v.extend_from_slice(k);
                v
            }
            Place::In(path) => {
                let mut v = vec![1];
                for i in path {
                    v.extend_from_slice(&i.to_be_bytes());
                }
                v
            }
        }
    }

    fn of_plan_key(k: &[u8]) -> Option<Place> {
        match k.split_first()? {
            (0, rest) => Some(Place::On(rest.to_vec())),
            (1, rest) if rest.len() % 2 == 0 => Some(Place::In(rest.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect())),
            _ => None,
        }
    }

    /// Its key once the fold is sealed at `stamp`.
    fn at(&self, stamp: Stamp) -> Vec<u8> {
        match self {
            Place::On(k) => k.clone(),
            Place::In(path) => FoldAt { stamp, path: path.clone() }.key(),
        }
    }
}

/// 78.138 — ONE ADMISSION PLANNED before anything is sealed: what remains of
/// each child after this netting, the rates edges and the cycles they close
/// (by the netting fold's path), and every fold to hold — the reserve's and
/// each line carried in it, children before parents — with its nets.
#[derive(Default)]
struct Plan {
    left: Keyed<(Lanes, Lanes)>,
    edges: Vec<PlannedEdge>,
    closes: Vec<PlannedEdge>,
    holds: Vec<PlannedHold>,
    heads: Keyed<[u8; 32]>,
    sides: Vec<([u8; 32], crate::anastome::dialogue::handoff::Side)>,
}

/// A planned rates edge, or the cycle it closes: the child vertex, the
/// parent vertex, the rate (or the holonomy), and the netting fold's path.
type PlannedEdge = (Vec<u8>, Vec<u8>, Rates, Vec<u16>);

/// A planned fold to hold: its path, the fold, and its nets by where each
/// child will stand.
type PlannedHold = (Vec<u16>, TierFold, Vec<(Place, Lanes, Lanes)>);

impl Kernel {
    /// AN ADMITTED FOLD'S SEALED COORDINATES (B′): held from its admission
    /// until a parent nets all of it — whatever the hot line has folded —
    /// or, for a fold admitted before the court held them, read off its
    /// receipt while the hot line still holds it (78.138: a carried line is
    /// always held, never read off a receipt it has none of).
    pub(crate) fn sealed_fold(&self, at: &FoldAt) -> Option<Sealed> {
        let key = at.key();
        if let Some(held) = self.ledgers.folds.get(&key) {
            return Some(*held);
        }
        // 78.138 — a carried line has no receipt of its own to read: netted
        // whole, it still stands by its record, or its final stub.
        if let Some(r) = self.ledgers.records.get(&key) {
            return Some((r.tier, r.prefix, r.rows));
        }
        if let Some((sealed, _)) = self.ledgers.finals.get(&key) {
            return Some(*sealed);
        }
        if !at.path.is_empty() {
            return None;
        }
        self.chain.node(&at.stamp)?.cites.iter().find_map(|x| match x {
            Cite::Uplink { tier, prefix, rows } | Cite::Chained { tier, prefix, rows, .. } => Some((*tier, *prefix, *rows)),
            _ => None,
        })
    }

    /// 78.125 — A FOLD ADMITTED, RECORDED: held by its sealed coordinates,
    /// recorded for its window with the span of its tier line it nets (from
    /// its tier's last head), linked as a parent of each child it netted,
    /// and its tier's head moved to its prefix.
    pub(crate) fn hold_admitted(&mut self, at: Stamp, fold: &TierFold, domain: [u8; 32], nets: Vec<(Vec<u8>, Lanes, Lanes)>) {
        self.hold_admitted_at(at.key(), at.h, fold, domain, nets);
    }

    /// The same, at a fold's key — 78.138: a carried line's, inside its
    /// reserve's receipt, sealed at that receipt's height.
    fn hold_admitted_at(&mut self, key: Vec<u8>, sealed: u64, fold: &TierFold, domain: [u8; 32], nets: Vec<(Vec<u8>, Lanes, Lanes)>) {
        let rows = fold.rows_hash();
        self.ledgers.folds.insert(&key, (fold.tier, fold.prefix, rows));
        // A tier's first fold spans its line from the zero head.
        let from = self.ledgers.heads.get(&fold.tier).map_or([0; 32], |h| *h);
        for (child, ..) in &nets {
            if let Some(r) = self.ledgers.records.get_mut(child) {
                r.parents.push(key.clone());
            }
        }
        let record = FoldRecord {
            tier: fold.tier,
            domain,
            from,
            prefix: fold.prefix,
            rows,
            children: fold.children_hash(),
            sealed,
            nets,
            parents: Vec::new(),
            demanded: None,
            upheld: None,
            standing: Standing::Open,
        };
        self.ledgers.records.insert(&key, record);
        self.ledgers.heads.insert(fold.tier, fold.prefix);
        self.ledgers.head_at.insert(fold.tier, key);
    }

    /// 78.125 (ruled) — A FOLD OVER ITS TIER'S CURRENT
    /// HEAD, answered without sealing: byte-identical to the fold admitted
    /// there — the same prefix, rows, children and handoff sides — it is a
    /// RESEND, answered with that fold's original receipt; otherwise a
    /// REPEAT, refused (an empty span claiming nets). `None`: the fold
    /// advances its tier's line. A retry is safe, and no fold is held twice.
    pub(crate) fn repeat_of(&self, f: &TierFold, domain: &[u8; 32]) -> Option<Uplinked> {
        if self.ledgers.heads.get(&f.tier) != Some(&f.prefix) {
            return None;
        }
        let original = || -> Option<Uplinked> {
            let key = self.ledgers.head_at.get(&f.tier)?;
            let rec = self.ledgers.records.get(key)?;
            let sides = self.handoffs.consumed_by.get(key).map_or(&[][..], Vec::as_slice);
            if rec.rows != f.rows_hash() || rec.children != f.children_hash() || sides != f.handoffs.as_slice() {
                return None;
            }
            let at = FoldAt::from_key(key)?.stamp;
            let receipt = self.chain.node(&at)?.chain_hash;
            Some(match rec.children {
                None => Uplinked::Admitted { at, receipt },
                Some(_) => {
                    let open = self.ledgers.open_in(domain).len();
                    let in_flight = rec
                        .nets
                        .iter()
                        .filter(|(child, ..)| self.ledgers.remaining.get(child).is_some_and(|(r, d)| !r.is_identity() || !d.is_identity()))
                        .count();
                    Uplinked::Chained { at, receipt, provisional: open > 0, obstructions: count(open), open_remainders: count(in_flight) }
                }
            })
        };
        Some(match original() {
            Some(resent) => resent,
            None => Uplinked::Repeat,
        })
    }

    /// 78.125 — A TIER'S LINE HEAD: where the court will replay its next
    /// fold's span from — its last recorded fold's prefix, or the zero head —
    /// and the stamp of the fold that set it. A client that lost its own head
    /// resyncs by it before it folds.
    pub fn line_head(&self, tier: &[u8; 32]) -> ([u8; 32], Option<FoldAt>) {
        let head = self.ledgers.heads.get(tier).map_or([0; 32], |h| *h);
        let at = self.ledgers.head_at.get(tier).and_then(|k| FoldAt::from_key(k));
        (head, at)
    }

    /// 78.123 B — A FOLD THAT NETS CHILDREN, admitted once its signature,
    /// its binding and its aggregation have read (`admit_uplink`). Each
    /// child: a fold already on this court — its rows proving against its
    /// commitment — or (78.138) a progenitor line's fold CARRIED INLINE, read
    /// here in full (its signature, its domain, its own bond, its span, its
    /// sides, its continuity, and its own children in turn); of the same
    /// domain, and with included parts that fit what remains of it. Any
    /// refusal refuses the whole fold, by the top child's index. Then the
    /// rates edges read on the domain's gauge, ONE receipt sealed — the
    /// reserve's, `Cite::Chained`, whose children's commitment holds every
    /// carried fold — and every fold held: each line under the receipt's
    /// stamp and its path, children before parents.
    pub(crate) fn admit_chained(&mut self, f: &SignedFold, domain: [u8; 32]) -> Uplinked {
        let mut plan = Plan::default();
        let mut gauge = match self.ledgers.gauges.get(&domain) {
            Some(g) => g.clone(),
            None => Gauge::default(),
        };
        let nets = match self.plan_children(&f.fold, &[], domain, &mut plan, &mut gauge) {
            Ok(nets) => nets,
            Err(refused) => return refused,
        };
        let Some(children) = f.fold.children_hash() else { return Uplinked::LineRefused };
        let rows = f.fold.rows_hash();
        let cite = Cite::Chained { tier: f.fold.tier, prefix: f.fold.prefix, rows, children };
        let Some(r) = self.seal_step(crate::chronometry::ActKind::Uplink, [0; 4], f.fold.tier, |_| vec![cite.clone()]) else {
            return Uplinked::LineRefused;
        };
        let stamp = r.stamp;
        let open_remainders = plan.left.scoped().filter(|(_, (r, d))| !r.is_identity() || !d.is_identity()).count();
        // Every carried line held, children before parents, then the
        // reserve's own fold: each a fold that may be netted in its turn and
        // challengeable for the receipt's window.
        for (path, fold, line_nets) in std::mem::take(&mut plan.holds) {
            let key = Place::In(path).at(stamp);
            let resolved = line_nets.into_iter().map(|(p, rows, declared)| (p.at(stamp), rows, declared)).collect();
            self.hold_admitted_at(key.clone(), stamp.h, &fold, domain, resolved);
            self.consume_sides(&key, &fold.handoffs);
        }
        for (k, v) in plan.left.scoped() {
            let Some(key) = Place::of_plan_key(&k).map(|p| p.at(stamp)) else { continue };
            // A child netted whole leaves the held folds; its empty remainder
            // stays, so it is never netted again from the full.
            if v.0.is_identity() && v.1.is_identity() {
                self.ledgers.folds.remove(&key);
            }
            self.ledgers.remaining.insert(&key, v.clone());
        }
        let at = |path: Vec<u16>| FoldAt { stamp, path }.key();
        let mut held: Vec<Edge> = self.ledgers.edges.remove(&domain).into_iter().flatten().collect();
        held.extend(plan.edges.into_iter().map(|(child, parent, rate, by)| Edge { child, parent, rate, by: at(by) }));
        self.ledgers.edges.insert(domain, held);
        self.ledgers.gauges.insert(domain, gauge);
        let found = plan.closes.into_iter().map(|(child, parent, holonomy, by)| Obstruction { child, parent, holonomy, by: at(by) });
        match self.ledgers.obstructions.get_mut(&domain) {
            Some(all) => all.extend(found),
            None => {
                self.ledgers.obstructions.insert(domain, found.collect());
            }
        }
        let resolved = nets.into_iter().map(|(p, rows, declared)| (p.at(stamp), rows, declared)).collect();
        self.hold_admitted(stamp, &f.fold, domain, resolved);
        self.mark_persist();
        let open = self.ledgers.open_in(&domain).len();
        Uplinked::Chained {
            at: stamp,
            receipt: r.chain_hash,
            provisional: open > 0,
            obstructions: count(open),
            open_remainders: count(open_remainders),
        }
    }

    /// 78.138 — THE CHILDREN OF THE FOLD AT `path` (inside the fold being
    /// admitted) PLANNED: each read, its included parts fitted against what
    /// remains of it, its rates edge laid on the gauge — a carried line read
    /// whole and its own children planned in turn. The fold's nets, by where
    /// each child will stand; or the refusal, by the top child's index.
    fn plan_children(&self, fold: &TierFold, path: &[u16], domain: [u8; 32], plan: &mut Plan, gauge: &mut Gauge<Rates>) -> Result<Vec<(Place, Lanes, Lanes)>, Uplinked> {
        let mut nets = Vec::with_capacity(fold.children.len());
        let parent = vertex(&fold.tier, &fold.prefix);
        for (i, c) in fold.children.iter().enumerate() {
            // A nested refusal is named at the top child it sits under.
            let top = match (path, u32::try_from(i)) {
                ([t, ..], _) => u32::from(*t),
                ([], Ok(n)) => n,
                ([], Err(_)) => u32::MAX,
            };
            let refuse = |why| Uplinked::ChildRefused { index: top, why };
            let Ok(i16) = u16::try_from(i) else { return Err(refuse(ChildRefusal::NotOnLine)) };
            let (place, sealed) = match &c.child {
                Child::Sealed(at) => {
                    let Some(sealed) = self.sealed_fold(at) else { return Err(refuse(ChildRefusal::NotOnLine)) };
                    if sealed.2 != c.rows_hash() {
                        return Err(refuse(ChildRefusal::Mismatch));
                    }
                    (Place::On(at.key()), sealed)
                }
                Child::Carried(line) => {
                    let mut here = path.to_vec();
                    here.push(i16);
                    self.plan_carried(line, c, &here, domain, plan, gauge, top)?;
                    (Place::In(here), (line.fold.tier, line.fold.prefix, line.fold.rows_hash()))
                }
            };
            if self.tier_domain.get(&sealed.0) != Some(&domain) {
                return Err(refuse(ChildRefusal::ForeignDomain));
            }
            let (Some(rows), Some(declared)) = (Lanes::of(&c.rows), Lanes::of(&c.declared)) else {
                return Err(refuse(ChildRefusal::Mismatch));
            };
            let (Some(inc_rows), Some(inc_declared)) = (Lanes::of(&c.included_rows), Lanes::of(&c.included_declared)) else {
                return Err(refuse(ChildRefusal::OverIncluded { lane: 0 }));
            };
            let plan_key = place.plan_key();
            let (r, d) = match (plan.left.get(&plan_key), &place) {
                (Some(x), _) => x.clone(),
                (None, Place::On(key)) => self.ledgers.left(key, &rows, &declared),
                (None, Place::In(_)) => (rows, declared),
            };
            if let Err(lane) = fits(&inc_rows, &r).and_then(|()| fits(&inc_declared, &d)) {
                return Err(refuse(ChildRefusal::OverIncluded { lane }));
            }
            let after = (
                inc_rows.inverse().and_then(|n| r.compose(&n)),
                inc_declared.inverse().and_then(|n| d.compose(&n)),
            );
            let (Some(r2), Some(d2)) = after else { return Err(refuse(ChildRefusal::OverIncluded { lane: 0 })) };
            plan.left.insert(&plan_key, (r2, d2));
            // THE RATES EDGE on the domain's gauge, read before the seal so
            // the seal knows whether it is provisional.
            let Some(rate) = Rates::of(c.rate) else {
                return Err(Uplinked::Discontinuous(crate::anastome::dialogue::uplink::Discontinuous::Rate { lane: 1 }));
            };
            let child = vertex(&sealed.0, &sealed.1);
            if let Gauged::Obstructed(h) = gauge.edge(&child, &parent, &rate) {
                plan.closes.push((child.clone(), parent.clone(), h, path.to_vec()));
            }
            plan.edges.push((child, parent.clone(), rate, path.to_vec()));
            nets.push((place, inc_rows, inc_declared));
        }
        Ok(nets)
    }

    /// 78.138 — A PROGENITOR LINE CARRIED INLINE, read whole before anything
    /// is sealed: its signature over its own fold, a tier of THIS domain, a
    /// bond of ITS OWN posted (each answers for its own work, as
    /// ruled), a span that advances its line, the handoff sides
    /// it consumes (once in all this admission), its continuity, its rows as
    /// its parent states them, and its own children planned in turn. Planned
    /// to be held at `path`, after its children. Refused by the top child's
    /// index — a refusal inside its own children passed through as it is.
    #[expect(clippy::too_many_arguments, reason = "the planner's state is threaded, never held: the fold, its parent's statement of it, where it stands, the domain, the plan, the gauge and the index it is named by")]
    fn plan_carried(&self, line: &SignedFold, c: &crate::anastome::dialogue::uplink::ChildNet, path: &[u16], domain: [u8; 32], plan: &mut Plan, gauge: &mut Gauge<Rates>, top: u32) -> Result<(), Uplinked> {
        let refuse = |why| Uplinked::ChildRefused { index: top, why };
        let fold = &line.fold;
        if !crate::court::identity::verify(&fold.tier, &fold.signed_bytes(), &line.sig) {
            return Err(refuse(ChildRefusal::Unsigned));
        }
        if self.tier_domain.get(&fold.tier) != Some(&domain) {
            return Err(refuse(ChildRefusal::ForeignDomain));
        }
        if self.bond_of(&fold.tier).iter().all(|m| *m == 0) {
            return Err(refuse(ChildRefusal::Unbonded));
        }
        if fold.rows != c.rows || fold.declared != c.declared {
            return Err(refuse(ChildRefusal::Mismatch));
        }
        // Its span advances its line: over the head the court holds for it
        // (or this admission's last fold of it), never an empty span.
        let head = match plan.heads.get(&fold.tier) {
            Some(h) => *h,
            None => self.ledgers.heads.get(&fold.tier).map_or([0; 32], |h| *h),
        };
        if head == fold.prefix {
            return Err(refuse(ChildRefusal::Repeat));
        }
        if fold.handoffs.iter().any(|s| plan.sides.contains(s)) {
            return Err(refuse(ChildRefusal::Broken));
        }
        let crossing = self.sides_of(fold).map_err(|_| refuse(ChildRefusal::Broken))?;
        // Its own continuity, named as a top fold's is.
        fold.continuity_with(&crossing).map_err(Uplinked::Discontinuous)?;
        let nets = self.plan_children(fold, path, domain, plan, gauge)?;
        plan.sides.extend(fold.handoffs.iter().copied());
        plan.heads.insert(fold.tier, fold.prefix);
        plan.holds.push((path.to_vec(), fold.clone(), nets));
        Ok(())
    }
}
