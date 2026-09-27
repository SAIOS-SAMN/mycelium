// 78.37, TP1 — THE ORBIT'S DOCKET: the classes its rings held and the demands
// its pivots left, cited on the orbit's close so a restart keeps the plan,
// and taken by the next orbit outermost first.
//
//! An entry is a citation by coordinates, never a copy (78.22): a held class
//! (`Cite::Class`) or a demand (`Cite::Demand`), each with its COVER — the
//! charts its thoughts stand on. Entries are WRITTEN in their own key order
//! (rule 15, nothing chosen). The next orbit TAKES them by the partial order
//! of containment: an entry whose cover contains another's wraps it, so its
//! answer transports to every ring beneath; the outermost first, key order
//! among incomparables. An entry whose cover did not read has its enclosure
//! owed and wraps nothing and is wrapped by nothing — taken in key order.

use super::*;
use crate::chronometry::Cite;

/// 78.37 — WHAT THE DOCKET HOLDS: this orbit's entries not yet cited, the
/// next orbit's questions (the last close's docket, or the one a boot
/// re-seeded), the entries of those whose enclosure is owed, and entries that
/// could not be written (a coordinate past its wall, LD0).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DocketReading {
    pub pending: u64,
    pub open: u64,
    pub owed: u64,
    pub unwritten: u64,
    /// R1 — entries answered by name since the contemplation began.
    pub answered: u64,
    /// Thoughts the class taker marked, since the contemplation began.
    pub taken: u64,
    /// 78.48(9) — open entries folded at a close: a whole orbit stood and no
    /// held idea on any chart of the cover, since the contemplation began.
    pub lapsed: u64,
}

/// An entry's own coordinates, its key in the docket.
fn entry_key(c: &Cite) -> Option<Vec<u8>> {
    let mut k = Vec::new();
    c.put_canonical(&mut k)?;
    Some(k)
}

/// Does `a`'s cover strictly contain `b`'s? Both must read.
fn wraps(a: &Cite, b: &Cite) -> bool {
    match (a.cover(), b.cover()) {
        (Some(x), Some(y)) => x.len() > y.len() && y.iter().all(|c| x.contains(c)),
        _ => false,
    }
}

impl Contemplation {
    /// FILE AN ENTRY on this orbit's docket, once by its own coordinates.
    pub(crate) fn docket_put(&mut self, c: Cite) {
        match entry_key(&c) {
            Some(k) => {
                self.docket.insert(&k, c);
            }
            None => self.docket_unwritten += 1,
        }
    }

    /// 78.37 — CS2's DEMAND ARM'S DEMAND onto the docket: the undischarged
    /// orbit's class, the charts it stands on as its cover, and (78.47) its
    /// residue as its canonical ray — the coordinate its constructors are
    /// derived from by whoever asks.
    pub(crate) fn docket_demand(
        &mut self,
        class: &crate::transducer::sentence::ClassVector,
        charts: &[crate::transducer::stalk::Invariant],
        residue: Option<&crate::transducer::cga::Mv>,
    ) {
        let mut keys: Keyed<()> = Keyed::new();
        for ch in charts {
            keys.insert(ch.key().as_bytes(), ());
        }
        let cover = Some(keys.scoped().map(|(k, _)| k).collect());
        let residue = residue.and_then(crate::transducer::cga::Mv::canonical);
        self.docket_put(Cite::Demand { class: Box::new(class.clone()), residue, cover });
    }

    /// THE WHOLE PLAN, in its entries' own key order: the open questions
    /// carried forward and this orbit's new entries — what the close cites,
    /// so the last Orbit act is the plan (R1).
    pub fn docket_cites(&self) -> Vec<Cite> {
        let mut all: Keyed<Cite> = Keyed::new();
        for (k, c) in self.docket_open.scoped().chain(self.docket.scoped()) {
            all.insert(&k, c.clone());
        }
        all.scoped().map(|(_, c)| c.clone()).collect()
    }

    /// THE CLOSE SEALED WITH THE PLAN CITED: this orbit's entries MERGE into
    /// the open questions, by their coordinates, and nothing leaves by
    /// replacement (R1).
    pub fn docket_sealed(&mut self) {
        for (k, c) in std::mem::take(&mut self.docket).scoped() {
            self.docket_open.insert(&k, c.clone());
        }
        self.docket_to_take = true;
    }

    /// THE CLASS TAKER, at the first stroke after a seal or a re-seed: in
    /// `docket_order()`, each held class's thoughts — those standing on its
    /// cover's charts — are marked dirty with its rotor in their horizon, so
    /// the expand ring reaches them first. The horizon's own order is
    /// untouched and nothing reorders the pivot's reading. An entry whose
    /// cover did not read has no thoughts to mark (its enclosure owed). The
    /// DEMAND taker is owed to CV4/SP2.
    pub(crate) fn take_docket(&mut self) {
        if !std::mem::take(&mut self.docket_to_take) {
            return;
        }
        for entry in self.docket_order() {
            let Cite::Class { rotor, cover: Some(charts), .. } = entry else { continue };
            let mut thoughts: Keyed<()> = Keyed::new();
            for chart in &charts {
                for (id, _) in self.by_chart.link(chart) {
                    if let Some(addr) = Contemplation::ordinal_of_key(&id).and_then(|o| self.address_at(o)) {
                        thoughts.insert(addr, ());
                    }
                }
            }
            let keys: Vec<Vec<u8>> = thoughts.scoped().map(|(k, _)| k).collect();
            for k in keys {
                let Some(b) = self.bubbles.get_mut(&k) else { continue };
                horizon_put(&mut b.horizon, rotor.as_ref().clone(), Standing::Held);
                b.dirty = true;
                self.dirty.insert(&k, ());
                self.docket_taken += 1;
            }
        }
    }

    /// AN ENTRY ANSWERED, BY NAME: it leaves the docket, open or pending, and
    /// is counted. The only way an entry leaves (R1).
    fn docket_answer(&mut self, key: &[u8]) {
        let left = self.docket_open.remove(key).is_some() | self.docket.remove(key).is_some();
        if left {
            self.docket_answered += 1;
        }
    }

    /// A HELD RING'S ENTRY, indexed by the ring's closing edge and its
    /// 2-simplices (volatile), so a collapse or a re-fill there answers it.
    pub(crate) fn docket_ring(&mut self, handles: &[Vec<u8>], entry: &Cite) {
        let Some(k) = entry_key(entry) else { return };
        for h in handles {
            self.docket_rings.insert(h, k.clone());
        }
    }

    /// A RING COLLAPSED OR A SIMPLEX RE-FILLED at `handle`: the class entry
    /// held there, where there is one, is answered.
    pub(crate) fn docket_ring_closed(&mut self, handle: &[u8]) {
        if let Some(k) = self.docket_rings.remove(handle) {
            self.docket_answer(&k);
        }
    }

    /// AN ORBIT DISCHARGED whose class key is `class_key`: a held word now
    /// spells it, and every entry of that class — held or demanded, carried
    /// in-process or re-seeded — is answered. One rule for both kinds.
    pub(crate) fn docket_discharged(&mut self, class_key: &[u8]) {
        let answered: Vec<Vec<u8>> = self
            .docket_open
            .scoped()
            .chain(self.docket.scoped())
            .filter(|(_, c)| {
                matches!(c, Cite::Class { class, .. } | Cite::Demand { class, .. } if class.class_key().as_bytes() == class_key)
            })
            .map(|(k, _)| k)
            .collect();
        // RM1 (c) — a DEMAND answered opens the re-fill's door for its
        // class: the entity re-spells that class's open remainders at the
        // close.
        for k in &answered {
            let demand = match (self.docket_open.get(k), self.docket.get(k)) {
                (Some(Cite::Demand { class, .. }), _) | (_, Some(Cite::Demand { class, .. })) => Some((**class).clone()),
                _ => None,
            };
            if let Some(class) = demand {
                self.demands_answered.push(class);
            }
        }
        for k in answered {
            self.docket_answer(&k);
        }
    }

    /// 78.48(9) — THE DOCKET'S FOLD, before the close cites it: an OPEN entry
    /// — one that has stood a whole orbit, re-seeded ones among them — whose
    /// cover names no chart a held idea stands on has carried no flux for
    /// that orbit: nothing in the mind can answer it through a thought, a
    /// free face (TC1), and it lapses, counted. This orbit's new entries are
    /// given their orbit; an entry whose cover did not read is kept, its
    /// enclosure owed (its charts cannot be asked). The plan is then bounded
    /// by the charts the live ideas stand on, never by what was ever asked.
    pub fn docket_fold(&mut self) {
        let lapsing: Vec<Vec<u8>> = self
            .docket_open
            .scoped()
            .filter(|(_, c)| match c.cover() {
                Some(charts) => !charts.iter().any(|ch| self.by_chart.link(ch).next().is_some()),
                None => false,
            })
            .map(|(k, _)| k)
            .collect();
        for k in lapsing {
            self.docket_open.remove(&k);
            self.docket_lapsed += 1;
        }
    }

    /// A RESTART KEEPS THE PLAN: the restored chain's last close's docket
    /// re-seeds the next orbit's questions. Only its docket entries are read.
    pub fn docket_reseed(&mut self, cites: &[Cite]) {
        self.docket_open = Keyed::new();
        for c in cites.iter().filter(|c| matches!(c, Cite::Class { .. } | Cite::Demand { .. })) {
            if let Some(k) = entry_key(c) {
                self.docket_open.insert(&k, c.clone());
            }
        }
        self.docket_to_take = true;
    }

    /// THE ORDER THE NEXT ORBIT TAKES ITS QUESTIONS IN: outermost first by
    /// containment of covers, key order among incomparables.
    pub fn docket_order(&self) -> Vec<Cite> {
        let mut left: Vec<Cite> = self.docket_open.scoped().map(|(_, c)| c.clone()).collect();
        let mut out = Vec::with_capacity(left.len());
        while !left.is_empty() {
            // The entries no remaining entry wraps, in key order.
            let (outer, rest): (Vec<Cite>, Vec<Cite>) =
                left.iter().cloned().partition(|c| !left.iter().any(|o| wraps(o, c)));
            out.extend(outer);
            left = rest;
        }
        out
    }

    /// 78.37 — the docket's reading.
    pub fn docket_reading(&self) -> DocketReading {
        DocketReading {
            pending: self.docket.len() as u64,
            open: self.docket_open.len() as u64,
            owed: self.docket_open.scoped().filter(|(_, c)| c.cover().is_none()).count() as u64,
            unwritten: self.docket_unwritten,
            answered: self.docket_answered,
            taken: self.docket_taken,
            lapsed: self.docket_lapsed,
        }
    }
}
