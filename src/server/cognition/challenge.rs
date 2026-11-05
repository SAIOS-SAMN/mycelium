// 78.125 — 2(c) THE CHALLENGE WINDOW on the court: a recorded fold challenged
// within `W` heights of its seal, reversed where it is falsified, and its
// finality read — recursive over every fold it nets.
//
//! The rulings of 2026-10-07: `W` a law parameter shared by a
//! network universe; falsification REVERSES (the bond's slash waits for
//! 2(e)); an arbitrage is discharged by a correction naming its holder, or
//! dissolves with a reversed edge; FINALITY IS RECURSIVE — a fold is final
//! only when its window has passed with nothing open and every fold it nets
//! is final, and a reversed child returns every ancestor that netted it to
//! provisional, its netting returned, to refile; AVAILABILITY — a demanded
//! prefix unpublished by its deadline falsifies the fold by default.

use super::ledger::{count, discharge_key, Standing};
use super::Court;
use crate::anastome::dialogue::challenge::{
    line_head, Challenge, ChallengeRefusal, Challenged, Entry, Finality, SignedChallenge,
};
use crate::anastome::dialogue::uplink::{ChildNet, FoldAt, Net, TierFold};
use crate::chronometry::{ActKind, Cite, Stamp};
use crate::network::nerve::Keyed;
use crate::transducer::cech::coefficient::{Coefficient, Lanes};
use num_rational::BigRational as Q;

/// The outcome bytes `Cite::Challenge` carries.
const DEMANDED: u8 = 0;
const UPHELD: u8 = 1;
/// FORMAT: outcome byte on `Cite::Challenge`.
const FALSIFIED: u8 = 2;
/// FORMAT: outcome byte on `Cite::Challenge`.
const DISCHARGED: u8 = 3;
/// FORMAT: outcome byte on `Cite::Challenge`.
const WITHDRAWN: u8 = 4;

fn zero() -> [Q; 4] {
    std::array::from_fn(|_| Q::from_integer(0.into()))
}

/// `v` added to what `party` is expected to hold.
fn bump(expected: &mut Keyed<[Q; 4]>, party: &[u8; 32], v: [Q; 4]) {
    let mut held = match expected.remove(party) {
        Some(h) => h,
        None => zero(),
    };
    for (h, x) in held.iter_mut().zip(v) {
        *h += x;
    }
    expected.insert(party, held);
}

/// The entries' own sums, rows and declared, added to `(rows, declared)` —
/// `None` where a sum passes its carrier: a line that cannot be summed.
fn summed(entries: &[Entry], so_far: (&Lanes, &Lanes)) -> Option<(Lanes, Lanes)> {
    let rows = Lanes::of(&entries.iter().flat_map(|e| e.rows.iter().copied()).collect::<Vec<_>>())?;
    let declared = Lanes::of(&entries.iter().flat_map(|e| e.declared.iter().copied()).collect::<Vec<_>>())?;
    Some((so_far.0.compose(&rows)?, so_far.1.compose(&declared)?))
}

/// A STREAMED RECONSTRUCTION in progress (the Concordance, 4.4: judged as a
/// proof is checked, never by re-running what led to it) — volatile, never
/// persisted (a restart opens it again): the fold as restated and proved
/// against its commitments, the line's head so far, and its own sums so far.
/// The line's entries are never held (LD0).
#[derive(Clone, Debug)]
pub(crate) struct Reconstruction {
    rows: Vec<Net>,
    declared: Vec<Net>,
    children: Vec<ChildNet>,
    head: [u8; 32],
    own_rows: Lanes,
    own_declared: Lanes,
}

impl Reconstruction {
    /// The reconstruction carried through `entries`, each step checked
    /// against the one before: the head stepped, the sums added — `None`
    /// where a sum passes its carrier.
    fn then(self, entries: &[Entry]) -> Option<Reconstruction> {
        let (own_rows, own_declared) = summed(entries, (&self.own_rows, &self.own_declared))?;
        Some(Reconstruction { head: line_head(self.head, entries), own_rows, own_declared, ..self })
    }
}

/// A reconstruction's key: the fold, then the filer — so no filer's publication can
/// reset another's.
fn reconstruction_key(fold: &[u8], filer: &[u8; 32]) -> Vec<u8> {
    let mut k = Vec::with_capacity(fold.len() + 40);
    crate::transducer::address::put_key_bytes(&mut k, fold);
    k.extend_from_slice(filer);
    k
}

/// RECONSTRUCTION (the Concordance, 4.4): the fold's OWN entries against what
/// it nets — on every identity and lane, `Σ own rows = rows − Σ rate ·
/// included rows` exactly, and `Σ own declared = declared`: conservation,
/// checked locally, nothing re-run. A parent never re-lists its children's
/// transfers; each child's are proved by its own records. The fold
/// reconstructs exactly where its defect is zero on every lane.
///
/// MU-DERIVATIONS D2.1 — THE DEFECT, the fold's measured lie on each lane:
///
/// ```text
/// D_l = Σ_w | want_{w,l} − have_{w,l} |  +  Σ_w | declared_{w,l} − own_declared_{w,l} |
/// want_{w,l} = rows_{w,l} − Σ_{c ∈ children} rate_{c,l} · included_{c,w,l}
/// ```
///
/// exact over ℚ, `have` the fold's own entries summed. Zero on every lane
/// exactly where the fold reconstructs. `None` where the declared nets
/// cannot be summed — no defect is measured, and the fold does not
/// reconstruct.
pub(crate) fn defect(rows: &[Net], declared: &[Net], children: &[ChildNet], own_rows: &Lanes, own_declared: &Lanes) -> Option<crate::economy::Mu> {
    use num_traits::Signed;
    let declared = Lanes::of(declared)?;
    // The expected own rows, exact rationals, per identity per lane.
    let mut expected: Keyed<[Q; 4]> = Keyed::new();
    for (who, v) in rows {
        bump(&mut expected, who, std::array::from_fn(|l| Q::from_integer(v[l].into())));
    }
    for c in children {
        for (who, v) in &c.included_rows {
            bump(&mut expected, who, std::array::from_fn(|l| -(Q::new(c.rate[l].0.into(), c.rate[l].1.into()) * Q::from_integer(v[l].into()))));
        }
    }
    for (who, v) in own_rows.nets() {
        bump(&mut expected, who, std::array::from_fn(|l| -Q::from_integer(v[l].into())));
    }
    // The declared, less the declared the entries carried.
    let mut declared_gap: Keyed<[Q; 4]> = Keyed::new();
    for (who, v) in declared.nets() {
        bump(&mut declared_gap, who, std::array::from_fn(|l| Q::from_integer(v[l].into())));
    }
    for (who, v) in own_declared.nets() {
        bump(&mut declared_gap, who, std::array::from_fn(|l| -Q::from_integer(v[l].into())));
    }
    let mut d = zero();
    for (_, gap) in expected.scoped().chain(declared_gap.scoped()) {
        for (t, g) in d.iter_mut().zip(gap.iter()) {
            *t += g.abs();
        }
    }
    crate::economy::Mu::of(d).ok()
}

impl Court {
    /// 78.125 — A CHALLENGE TAKEN: the filer's signature, the fold's record,
    /// its window — then the demand, the replay, the default or the
    /// correction, each sealed by `Cite::Challenge`.
    pub fn challenge(&mut self, c: &SignedChallenge) -> Challenged {
        use ChallengeRefusal as R;
        let refuse = Challenged::Refused;
        if !crate::court::identity::verify(&c.who, &c.challenge.signed_bytes(&c.who), &c.sig) {
            return refuse(R::Unsigned);
        }
        let fold = c.challenge.fold();
        let key = fold.key();
        let Some(rec) = self.ledgers.records.get(&key).cloned() else { return refuse(R::NoRecord) };
        if rec.standing != Standing::Open {
            return refuse(R::Closed);
        }
        let head = self.chain.head_height();
        let window_open = self.evidence_open(&key, rec.sealed, head);
        let deadline_open = rec.demanded.is_some_and(|d| !self.window_closed(d, head));
        match &c.challenge {
            Challenge::Withdraw { demand, .. } => {
                if c.who != rec.tier {
                    return refuse(R::NotTheTier);
                }
                if self.ledgers.withdrawn.contains(&key) {
                    return refuse(R::Withdrawn);
                }
                if rec.upheld.is_some() {
                    return refuse(R::Published);
                }
                let Some(demanded) = rec.demanded else { return refuse(R::NotDemanded) };
                // THE DEMAND IS THE TIER'S OWN: its receipt, at the height the
                // demand stands at, cites this fold's demand, and its
                // operator is the tier's (`operator_hash(voice, court_head)`)
                // — an external challenger's demand never withdraws.
                let own = crate::chronometry::operator_hash(&rec.tier, &self.court_head);
                let is_it = self.chain.node(demand).is_some_and(|n| {
                    n.stamp.h == demanded
                        && n.operator_hash == own
                        && n.cites.iter().any(|x| matches!(x, Cite::Challenge { fold: f, kind: 0, outcome: DEMANDED, .. } if *f == fold))
                });
                if !is_it {
                    return refuse(R::NotTheDemand);
                }
                let Some(r) = self.seal_challenge(c, WITHDRAWN, [0; 32]) else { return refuse(R::LineRefused) };
                if let Some(held) = self.ledgers.records.get_mut(&key) {
                    held.demanded = None;
                }
                self.ledgers.demands.remove(&key);
                self.ledgers.withdrawn.insert(&key, r.stamp.h);
                Challenged::Reopened { at: r.stamp, receipt: r.chain_hash.bytes(), closes: self.window_close(r.stamp.h).ok() }
            }
            Challenge::Demand { .. } => {
                if rec.upheld.is_some() {
                    return refuse(R::Published);
                }
                if rec.demanded.is_some() {
                    // A RESEND by the same filer (as ruled) is
                    // answered with the original demand; another filer's is
                    // refused — one demand per fold.
                    let original = self
                        .ledgers
                        .demands
                        .get(&key)
                        .filter(|(_, filer)| *filer == c.who)
                        .and_then(|(at, _)| Stamp::from_key(at))
                        .and_then(|at| Some((at, self.chain.node(&at)?.chain_hash.bytes())));
                    return match original {
                        Some((at, receipt)) => Challenged::Demanded { at, receipt, deadline: self.window_close(at.h).ok() },
                        None => refuse(R::Demanded),
                    };
                }
                if !window_open {
                    return refuse(R::Closed);
                }
                let Some(r) = self.seal_challenge(c, DEMANDED, [0; 32]) else { return refuse(R::LineRefused) };
                if let Some(held) = self.ledgers.records.get_mut(&key) {
                    held.demanded = Some(r.stamp.h);
                }
                self.ledgers.demands.insert(&key, (r.stamp.key(), c.who));
                Challenged::Demanded { at: r.stamp, receipt: r.chain_hash.bytes(), deadline: self.window_close(r.stamp.h).ok() }
            }
            Challenge::Evidence { .. } | Challenge::Open { .. } | Challenge::Part { .. } => {
                if rec.upheld.is_some() {
                    return refuse(R::Published);
                }
                // Within the window, or within a standing demand's deadline.
                if !(window_open || deadline_open) {
                    return refuse(R::Closed);
                }
                let k = reconstruction_key(&key, &c.who);
                let proves = |rows: &[Net], declared: &[Net], children: &[ChildNet]| {
                    let shown = TierFold {
                        tier: rec.tier,
                        prefix: rec.prefix,
                        rows: rows.to_vec(),
                        declared: declared.to_vec(),
                        children: children.to_vec(),
                        handoffs: Vec::new(),
                    };
                    shown.rows_hash() == rec.rows && shown.children_hash() == rec.children
                };
                // 78.126 — what the fold's consumed handoff sides carried stands
                // with its declared: its line declares the crossing too.
                let crossing = self.handoffs.crossing(&key);
                let declaring = |declared: &[Net]| -> Vec<Net> { declared.iter().chain(&crossing).copied().collect() };
                let rebuilt = match &c.challenge {
                    // The whole prefix in one act.
                    Challenge::Evidence { rows, declared, children, entries, .. } => {
                        if !proves(rows, declared, children) {
                            return refuse(R::NotTheFold);
                        }
                        let declared = declaring(declared);
                        Reconstruction { rows: rows.clone(), declared, children: children.clone(), head: rec.from, own_rows: Lanes::identity(), own_declared: Lanes::identity() }
                            .then(entries)
                    }
                    // A streamed publication opened: the fold proved, the line
                    // at the span's start; nothing sealed.
                    Challenge::Open { rows, declared, children, .. } => {
                        if !proves(rows, declared, children) {
                            return refuse(R::NotTheFold);
                        }
                        let declared = declaring(declared);
                        let opened = Reconstruction { rows: rows.clone(), declared, children: children.clone(), head: rec.from, own_rows: Lanes::identity(), own_declared: Lanes::identity() };
                        self.reconstructions.insert(&k, opened);
                        return Challenged::Streaming { head: rec.from };
                    }
                    Challenge::Part { entries, .. } => {
                        let Some(open) = self.reconstructions.remove(&k) else { return refuse(R::NotOpened) };
                        open.then(entries)
                    }
                    Challenge::Demand { .. } | Challenge::Default { .. } | Challenge::Correction { .. } | Challenge::Withdraw { .. } => {
                        return refuse(R::NotOpened)
                    }
                };
                // A line that cannot be summed is no evidence.
                let Some(rebuilt) = rebuilt else { return refuse(R::NotThePrefix) };
                if rebuilt.head != rec.prefix {
                    return match c.challenge {
                        // A part continues; the whole prefix must reach the cited head.
                        Challenge::Part { .. } => {
                            let at = rebuilt.head;
                            self.reconstructions.insert(&k, rebuilt);
                            Challenged::Streaming { head: at }
                        }
                        _ => refuse(R::NotThePrefix),
                    };
                }
                // THE LINE REACHED THE CITED HEAD: judged once, sealed once,
                // the head its content address.
                let lie = defect(&rebuilt.rows, &rebuilt.declared, &rebuilt.children, &rebuilt.own_rows, &rebuilt.own_declared);
                if lie.as_ref().is_some_and(crate::economy::Mu::is_zero) {
                    let Some(r) = self.seal_challenge(c, UPHELD, rec.prefix) else { return refuse(R::LineRefused) };
                    if let Some(held) = self.ledgers.records.get_mut(&key) {
                        held.upheld = Some(rec.prefix);
                    }
                    self.mark_persist();
                    return Challenged::Upheld { at: r.stamp, receipt: r.chain_hash.bytes(), prefix: rec.prefix };
                }
                // The exposure is read before reversal moves it; a defect that
                // cannot be measured stands as the whole exposure.
                let exposure = self.exposure_of(&key);
                let lie = match lie {
                    Some(d) => d,
                    None => exposure.clone(),
                };
                let Some(r) = self.seal_challenge(c, FALSIFIED, rec.prefix) else { return refuse(R::LineRefused) };
                let unwound = self.reverse(&key, (r.stamp, r.chain_hash.bytes()));
                // 78.129, D2.1–D2.2 — the slash: the bond against the
                // exposure, its bounty the defect against the exposure.
                self.slash(&rec.tier, c.who, &exposure, &lie, (r.stamp, r.chain_hash.bytes()));
                Challenged::Falsified { at: r.stamp, receipt: r.chain_hash.bytes(), unwound: count(unwound) }
            }
            Challenge::Default { .. } => {
                if rec.upheld.is_some() {
                    return refuse(R::Published);
                }
                let Some(demanded) = rec.demanded else { return refuse(R::NotDemanded) };
                if !self.window_closed(demanded, head) {
                    return refuse(R::NotDue);
                }
                // A default published nothing: every net the fold asserted
                // stands unproved, so its defect is its whole exposure.
                let exposure = self.exposure_of(&key);
                let Some(r) = self.seal_challenge(c, FALSIFIED, [0; 32]) else { return refuse(R::LineRefused) };
                let unwound = self.reverse(&key, (r.stamp, r.chain_hash.bytes()));
                self.slash(&rec.tier, c.who, &exposure, &exposure, (r.stamp, r.chain_hash.bytes()));
                Challenged::Falsified { at: r.stamp, receipt: r.chain_hash.bytes(), unwound: count(unwound) }
            }
            Challenge::Correction { child, holder, amount, .. } => {
                if c.who != rec.tier {
                    return refuse(R::NotTheTier);
                }
                if !window_open {
                    return refuse(R::Closed);
                }
                let k = discharge_key(&key, child);
                let open = self.ledgers.open_in(&rec.domain).iter().any(|o| o.key() == k);
                if !open {
                    return refuse(R::NoObstruction);
                }
                let mut booked = Vec::new();
                crate::transducer::address::put_key_bytes(&mut booked, child);
                booked.extend_from_slice(holder);
                booked.extend_from_slice(&amount.to_le_bytes());
                let Some(r) = self.seal_challenge(c, DISCHARGED, crate::chronometry::payload_hash(&booked)) else {
                    return refuse(R::LineRefused);
                };
                self.ledgers.discharged.insert(&k, (*holder, *amount));
                self.mark_persist();
                Challenged::Discharged { at: r.stamp, receipt: r.chain_hash.bytes() }
            }
        }
    }

    /// Whether a fold's evidence window stands open at `head`: the window
    /// opened at its seal, or — its self-demand withdrawn — the window the
    /// withdrawal opened, while either has not closed.
    pub(crate) fn evidence_open(&self, key: &[u8], sealed: u64, head: u64) -> bool {
        !self.window_closed(sealed, head) || self.ledgers.withdrawn.get(key).is_some_and(|h| !self.window_closed(*h, head))
    }

    /// The opening of the window a fold's finality waits on: a standing
    /// demand's, else an open withdrawal's, else its seal's.
    fn governing_window(&self, key: &[u8], sealed: u64, demanded: Option<u64>, head: u64) -> u64 {
        if let Some(d) = demanded {
            return d;
        }
        match self.ledgers.withdrawn.get(key) {
            Some(h) if !self.window_closed(*h, head) => *h,
            _ => sealed,
        }
    }

    /// MU-DERIVATIONS D2.1 — THE EXPOSURE of the fold at `key`: the value its
    /// commitment moved on each lane, the quantity reversal returns, read
    /// before reversal moves it.
    ///
    /// ```text
    /// own(k) = remaining[k] ∘ Σ_{p ∈ parents(k)} included_p(k)        (rows, declared) as admitted
    /// X_l    = Σ_{(w,n) ∈ own(k).rows} n_l⁺  +  Σ_{(w,n) ∈ own(k).declared} |n_l|
    ///          + Σ_{(c, rows, decl) ∈ rec.nets} ( Σ_{(w,n) ∈ rows} n_l⁺ + Σ_{(w,n) ∈ decl} |n_l| )
    /// ```
    ///
    /// Rows move key to key and sum to zero, so their positive part is the
    /// value transferred, with no `½`; a declared flow is one-sided, so its
    /// absolute value is its value. Summed exactly: no carrier to pass.
    pub(crate) fn exposure_of(&self, key: &[u8]) -> crate::economy::Mu {
        use num_traits::Signed;
        let Some(rec) = self.ledgers.records.get(key) else { return crate::economy::Mu::zero() };
        let (mut rows, mut declared): (Keyed<[Q; 4]>, Keyed<[Q; 4]>) = (Keyed::new(), Keyed::new());
        let gather = |into: &mut Keyed<[Q; 4]>, nets: &[Net]| {
            for (who, v) in nets {
                bump(into, who, std::array::from_fn(|l| Q::from_integer(v[l].into())));
            }
        };
        if let Some((r, d)) = self.ledgers.remaining.get(key) {
            gather(&mut rows, r.nets());
            gather(&mut declared, d.nets());
        }
        for p in &rec.parents {
            let Some(parent) = self.ledgers.records.get(p) else { continue };
            for (_, r, d) in parent.nets.iter().filter(|(child, ..)| child.as_slice() == key) {
                gather(&mut rows, r.nets());
                gather(&mut declared, d.nets());
            }
        }
        let mut x = zero();
        let positive = |x: &mut [Q; 4], v: &[Q; 4]| {
            for (t, n) in x.iter_mut().zip(v.iter()) {
                if n.is_positive() {
                    *t += n;
                }
            }
        };
        let absolute = |x: &mut [Q; 4], v: &[Q; 4]| {
            for (t, n) in x.iter_mut().zip(v.iter()) {
                *t += n.abs();
            }
        };
        for (_, v) in rows.scoped() {
            positive(&mut x, v);
        }
        for (_, v) in declared.scoped() {
            absolute(&mut x, v);
        }
        for (_, r, d) in &rec.nets {
            for (_, v) in r.nets() {
                positive(&mut x, &std::array::from_fn(|l| Q::from_integer(v[l].into())));
            }
            for (_, v) in d.nets() {
                absolute(&mut x, &std::array::from_fn(|l| Q::from_integer(v[l].into())));
            }
        }
        // A sum of positive parts and absolute values is never negative.
        match crate::economy::Mu::of(x) {
            Ok(m) => m,
            Err(_) => crate::economy::Mu::zero(),
        }
    }

    fn seal_challenge(&mut self, c: &SignedChallenge, outcome: u8, evidence: [u8; 32]) -> Option<crate::chronometry::KStepReceipt> {
        let cite = Cite::Challenge { fold: c.challenge.fold(), kind: c.challenge.kind(), outcome, evidence };
        let r = self.seal_step(ActKind::Challenge, [0; 4], c.who, |_| vec![cite.clone()])?;
        self.mark_persist();
        Some(r)
    }

    /// REVERSAL: the falsified fold and, transitively, every fold that netted
    /// it — each one's netting returned to its children's remainders, its
    /// edges out of its domain's gauge (solved again), its bookings gone, and
    /// it voided from netting; the tolls riding on its OUT sides refunded
    /// (78.133). The count of ancestors unwound.
    fn reverse(&mut self, key: &[u8], anchor: crate::economy::Anchor) -> usize {
        let mut stack = vec![(key.to_vec(), Standing::Falsified)];
        let mut void: Vec<Vec<u8>> = Vec::new();
        let mut domains: Vec<[u8; 32]> = Vec::new();
        while let Some((k, standing)) = stack.pop() {
            let Some(rec) = self.ledgers.records.get_mut(&k) else { continue };
            if rec.standing != Standing::Open {
                continue;
            }
            rec.standing = standing;
            let (nets, parents, domain) = (rec.nets.clone(), rec.parents.clone(), rec.domain);
            for (child, rows, declared) in nets {
                let back = match self.ledgers.remaining.get(&child) {
                    Some((r, d)) => (r.compose(&rows), d.compose(&declared)),
                    None => (Some(rows), Some(declared)),
                };
                if let (Some(r), Some(d)) = back {
                    self.ledgers.remaining.insert(&child, (r, d));
                }
                // Nettable again: by its record, or by its final stub.
                let sealed = match (self.ledgers.records.get(&child), self.ledgers.finals.get(&child)) {
                    (Some(c), _) => Some((c.tier, c.prefix, c.rows)),
                    (None, Some((s, _))) => Some(*s),
                    (None, None) => None,
                };
                if let Some(sealed) = sealed {
                    self.ledgers.folds.insert(&child, sealed);
                }
            }
            stack.extend(parents.into_iter().map(|p| (p, Standing::Unwound)));
            if !domains.contains(&domain) {
                domains.push(domain);
            }
            void.push(k);
        }
        // 78.133 — every part of a toll whose OUT side a voided fold consumed,
        // back to the traveller together — read before the sides reopen.
        self.unwind_tolls(&void, anchor);
        for k in &void {
            self.ledgers.folds.remove(k);
            self.ledgers.remaining.remove(k);
            // 78.126 — the handoff sides it consumed are open again.
            self.handoffs.unconsume(k);
        }
        for d in &domains {
            if let Some(edges) = self.ledgers.edges.get_mut(d) {
                edges.retain(|e| !void.contains(&e.by));
            }
            self.ledgers.rederive(d);
        }
        let booked: Vec<Vec<u8>> = self
            .ledgers
            .discharged
            .scoped()
            .map(|(k, _)| k)
            .filter(|k| void.iter().any(|v| k.starts_with(&discharge_key(v, &[]))))
            .collect();
        for k in &booked {
            self.ledgers.discharged.remove(k);
        }
        self.mark_persist();
        // The falsified fold first; the rest are the ancestors unwound.
        void.iter().skip(1).count()
    }

    /// 78.125 — THE WINDOWS SETTLED, at an orbit's close (the court's own
    /// cycle; never a height interval). What the court keeps of the ledger
    /// trees is bounded by what is still live:
    /// - a streamed replay whose fold can take no more evidence leaves;
    /// - a record read FINAL is pruned to its stub (coordinates, upheld) —
    ///   a final fold is never reversed, so nothing else is read again;
    /// - a stub leaves once the fold has no value in flight and no recorded
    ///   (unfinished) fold nets it;
    /// - an empty remainder — the guard against netting a consumed child
    ///   from the full — leaves once nothing names the fold and the hot line
    ///   no longer holds its receipt.
    ///
    /// What stays is the folds within the folds' windows and the open
    /// remainders: the bound 78.123 ruled. The count of entries it let go.
    pub(crate) fn settle_windows(&mut self) -> usize {
        let head = self.chain.head_height();
        let mut gone = 0usize;
        // Reconstructions whose fold takes no more evidence.
        let stale: Vec<Vec<u8>> = self
            .reconstructions
            .scoped()
            .map(|(k, _)| k)
            .filter(|k| {
                let Some(fold) = crate::transducer::address::take_key_bytes(k, &mut 0) else { return true };
                let live = self.ledgers.records.get(&fold).is_some_and(|r| {
                    let window = self.evidence_open(&fold, r.sealed, head) || r.demanded.is_some_and(|d| !self.window_closed(d, head));
                    r.standing == Standing::Open && r.upheld.is_none() && window
                });
                !live
            })
            .collect();
        for k in &stale {
            self.reconstructions.remove(k);
        }
        // Final records, to stubs.
        let mut memo: Keyed<Finality> = Keyed::new();
        let keys: Vec<Vec<u8>> = self.ledgers.records.scoped().map(|(k, _)| k).collect();
        let finals: Vec<(Vec<u8>, bool)> = keys
            .into_iter()
            .filter_map(|k| match self.finality_of(&k, head, &mut memo) {
                Finality::Final { upheld } => Some((k, upheld)),
                _ => None,
            })
            .collect();
        for (k, upheld) in finals {
            if let Some(r) = self.ledgers.records.remove(&k) {
                self.ledgers.finals.insert(&k, ((r.tier, r.prefix, r.rows), upheld));
                gone += 1;
            }
        }
        // Stubs no longer needed: not nettable, named by no unfinished fold,
        // and off the hot line — while the hot line holds its receipt, a fold
        // with no record and no stub reads PRE-LINE, so its stub stays and it
        // reads final (the seam :7520 found: a let-go stub misread pre-line).
        let mut named: Keyed<()> = Keyed::new();
        for (_, r) in self.ledgers.records.scoped() {
            for (child, ..) in &r.nets {
                named.insert(child, ());
            }
        }
        let spent: Vec<Vec<u8>> = self
            .ledgers
            .finals
            .scoped()
            .map(|(k, _)| k)
            .filter(|k| {
                !self.ledgers.folds.contains(k) && !named.contains(k) && FoldAt::from_key(k).and_then(|s| self.chain.node(&s.stamp)).is_none()
            })
            .collect();
        for k in &spent {
            self.ledgers.finals.remove(k);
            gone += 1;
        }
        // Empty remainders nothing names and the hot line no longer holds.
        let empty: Vec<Vec<u8>> = self
            .ledgers
            .remaining
            .scoped()
            .filter(|(k, (r, d))| {
                r.is_identity()
                    && d.is_identity()
                    && !self.ledgers.records.contains(k)
                    && !self.ledgers.finals.contains(k)
                    && FoldAt::from_key(k).and_then(|s| self.chain.node(&s.stamp)).is_none()
            })
            .map(|(k, _)| k)
            .collect();
        for k in &empty {
            self.ledgers.remaining.remove(k);
            gone += 1;
        }
        // 78.133 — tolls whose window closed, paid out (before the handoffs
        // they ride on are let go).
        gone += self.release_tolls();
        // 78.154 (4) — and the map fees claimed on stakes, paid whole.
        gone += self.release_claims();
        // 78.126 — handoffs both of whose sides final folds consumed.
        gone += self.settle_handoffs();
        // Demands and withdrawals of folds no longer recorded.
        let records = &self.ledgers.records;
        let stale: Vec<Vec<u8>> = self.ledgers.demands.scoped().map(|(k, _)| k).filter(|k| !records.contains(k)).collect();
        let spent: Vec<Vec<u8>> = self.ledgers.withdrawn.scoped().map(|(k, _)| k).filter(|k| !records.contains(k)).collect();
        for k in &stale {
            self.ledgers.demands.remove(k);
        }
        for k in &spent {
            self.ledgers.withdrawn.remove(k);
        }
        gone += stale.len() + spent.len();
        // 78.125 (law version 5) — the closes no held window reads.
        gone += self.settle_closes();
        if gone > 0 {
            self.mark_persist();
        }
        gone + stale.len()
    }

    /// 78.125 — A DOMAIN'S OPEN ARBITRAGES, as served: each by the fold that
    /// closed it, its edge's fold vertices and its exact holonomy.
    pub fn open_obstructions(&self, domain: &[u8; 32]) -> Vec<crate::anastome::dialogue::challenge::OpenObstruction> {
        self.ledgers
            .open_in(domain)
            .into_iter()
            .filter_map(|o| {
                Some(crate::anastome::dialogue::challenge::OpenObstruction {
                    fold: FoldAt::from_key(&o.by)?,
                    child: o.child.clone(),
                    parent: o.parent.clone(),
                    holonomy: o.holonomy.clone(),
                })
            })
            .collect()
    }

    /// 78.125 — A FOLD'S FINALITY at this court's head: its window, any
    /// demand's deadline, its open obstructions, and — recursively — every
    /// fold it nets. Read, never stored; it seals nothing.
    pub fn finality(&self, fold: &FoldAt) -> Finality {
        let mut memo: Keyed<Finality> = Keyed::new();
        self.finality_of(&fold.key(), self.chain.head_height(), &mut memo)
    }

    fn finality_of(&self, key: &[u8], head: u64, memo: &mut Keyed<Finality>) -> Finality {
        if let Some(f) = memo.get(key) {
            return *f;
        }
        let read = self.finality_read(key, head, memo);
        memo.insert(key, read);
        read
    }

    fn finality_read(&self, key: &[u8], head: u64, memo: &mut Keyed<Finality>) -> Finality {
        let Some(rec) = self.ledgers.records.get(key) else {
            // A recorded fold read final and pruned to its stub.
            if let Some((_, upheld)) = self.ledgers.finals.get(key) {
                return Finality::Final { upheld: *upheld };
            }
            // A fold sealed before the tier-line law has no record and no
            // line to replay: grandfathered (as ruled).
            return match FoldAt::from_key(key).and_then(|at| self.sealed_fold(&at)) {
                Some(_) => Finality::PreLine,
                None => Finality::Unrecorded,
            };
        };
        match rec.standing {
            Standing::Falsified => return Finality::Falsified,
            Standing::Unwound => return Finality::Unwound,
            Standing::Open => {}
        }
        // A withdrawn self-demand reopened the window: provisional through it.
        let mut open = self.evidence_open(key, rec.sealed, head);
        let pending = match (rec.demanded, rec.upheld) {
            (Some(d), None) => {
                if self.window_closed(d, head) {
                    // The default is due and unclaimed: never final.
                    return Finality::Blocked;
                }
                open = true;
                Some(d)
            }
            _ => None,
        };
        if open {
            let opened = self.governing_window(key, rec.sealed, pending, head);
            return Finality::Provisional { opened, closed: self.window_close(opened).ok() };
        }
        if self.ledgers.open_in(&rec.domain).iter().any(|o| o.by == key) {
            return Finality::Blocked;
        }
        let children: Vec<Vec<u8>> = rec.nets.iter().map(|(c, ..)| c.clone()).collect();
        let upheld = rec.upheld.is_some();
        for c in &children {
            if !matches!(self.finality_of(c, head, memo), Finality::Final { .. } | Finality::PreLine) {
                return Finality::Blocked;
            }
        }
        Finality::Final { upheld }
    }
}
