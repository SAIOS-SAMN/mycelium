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
    /// orbit's class and the charts it stands on, as its cover.
    pub(crate) fn docket_demand(
        &mut self,
        class: &crate::transducer::sentence::ClassVector,
        charts: &[crate::transducer::stalk::Invariant],
    ) {
        let mut keys: Keyed<()> = Keyed::new();
        for ch in charts {
            keys.insert(ch.key().as_bytes(), ());
        }
        let cover = Some(keys.scoped().map(|(k, _)| k).collect());
        self.docket_put(Cite::Demand { class: Box::new(class.clone()), cover });
    }

    /// THIS ORBIT'S DOCKET, in its entries' own key order: what the close
    /// cites.
    pub fn docket_cites(&self) -> Vec<Cite> {
        self.docket.scoped().map(|(_, c)| c.clone()).collect()
    }

    /// THE CLOSE SEALED WITH THE DOCKET CITED: it becomes the next orbit's
    /// questions, and this orbit's docket starts empty.
    pub fn docket_sealed(&mut self) {
        self.docket_open = std::mem::take(&mut self.docket);
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
        }
    }
}
