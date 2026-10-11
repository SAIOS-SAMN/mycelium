// 78.37(4), RM1 — THE MIND'S REMAINDERS BY WHERE THEY WERE PERCEIVED: the
// index the receptor reads, derived from the words' torsion records.

use crate::client::sdk::*;
use crate::transducer::address::Address;
use crate::transducer::element::{Element, Remainder};

/// RM1 — A CALLUS THE RECEPTOR RAISED: one class perceived at two
/// incomparable chart chains with one ray — the mind's own second vantage —
/// and the two words whose records carry the remainders. The voice stands it
/// (`Entity::stand_calluses`).
#[derive(Clone, Debug)]
pub struct Callus {
    pub class: Box<crate::transducer::sentence::ClassVector>,
    pub ray: Box<crate::transducer::alphabet::Rotor>,
    pub chains: [Vec<Vec<u8>>; 2],
    pub words: [Address; 2],
}

/// What the mind made of the calluses raised: stood; unspelled (no held
/// constructor spells the ray — its class docketed as a demand); a chain
/// whose top did not read; a form that did not mint. And what the receptor
/// could not read: an index key whose chain does not read (`malformed`), a
/// word the index names whose record carries no matching remainder
/// (`unrecorded`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CallusTally {
    pub stood: u64,
    pub unspelled: u64,
    pub untopped: u64,
    pub unminted: u64,
    pub malformed: u64,
    pub unrecorded: u64,
    /// RM1 (c) — remainders the receptor skipped as answered (by a court's
    /// seal, or re-spelled by this mind); re-fills made; re-fills refused
    /// because the word's record carried no open remainder there.
    pub answered: u64,
    pub refilled: u64,
    pub refill_refused: u64,
}

impl Epigenome {
    /// A WORD STANDS — resident or dialect: each remainder its torsion record
    /// carries is filed at its class ‖ residue's chart chain. ONE REMAINDER
    /// PER CLASS at a chart (78.37(4)): a second word whose remainder
    /// coincides with one standing is not filed over it and is counted. A
    /// remainder newly filed is first RECEIVED against every remainder of its
    /// class at any chart.
    pub(crate) fn file_remainders(&mut self, addr: &Address, e: &Element) {
        let mut keys = Vec::new();
        // Each remainder with the ray of the cycle it rides (PD1, law 12):
        // the ray is the holonomy cycle's generator, not a copy on the
        // remainder. Only holonomy cycles carry remainders (the court's
        // Congruent fill and the vortex collapse file them; `torse` files
        // none), so a remainder with no ray has no producer.
        for (r, ray) in e.remainder_rays() {
            let k = r.index_key();
            match self.remainder_fibers.get(&k) {
                Some(a) if a == addr => keys.push(k),
                Some(_) => self.remainders_coincident += 1,
                None => {
                    if let Some(ray) = ray {
                        self.receive(addr, r, ray);
                    }
                    self.remainder_fibers.insert(&k, addr.clone());
                    keys.push(k);
                }
            }
        }
        if !keys.is_empty() {
            self.remainder_keys.insert(addr.as_bytes(), keys);
        }
    }

    /// The element a word this mind holds is, resident or dialect.
    pub(crate) fn element_of(&self, addr: &[u8]) -> Option<&Element> {
        match self.resident.get(addr) {
            Some(w) => Some(w.held.element()),
            None => self.dialect.get(addr).map(|d| d.sentence.element()),
        }
    }

    /// RM1 — THE RECEPTOR: a remainder `r` of the word at `addr`, read
    /// against every remainder of its class standing at another chart — one
    /// prefix read under κ. A chain nested in the other is one vantage at a
    /// finer depth, counted; two incomparable chains are two vantages: one
    /// ray (projectively) raises a callus, once per κ per pair of chains;
    /// differing rays are DISCORDANT, counted, and no ray is chosen
    /// (checklist (r)).
    fn receive(&mut self, addr: &Address, r: &Remainder, r_ray: &crate::transducer::alphabet::Rotor) {
        // RM1 (c): an answered remainder is evidence, not a question.
        if self.answered(addr.as_bytes(), r) {
            self.calluses.answered += 1;
            return;
        }
        let prefix = Remainder::class_prefix(&r.class);
        let standing: Vec<(Vec<u8>, Address)> = match self.remainder_fibers.fiber_at(&prefix) {
            Some(f) => f.map(|(k, a)| (k, a.clone())).collect(),
            None => Vec::new(),
        };
        for (key, other) in standing {
            if other == *addr {
                continue;
            }
            let Some(chain) = Remainder::chain_of_key(&key) else {
                self.calluses.malformed += 1;
                continue;
            };
            let within = |a: &[Vec<u8>], b: &[Vec<u8>]| a.iter().all(|k| b.contains(k));
            if within(&r.at, &chain) || within(&chain, &r.at) {
                self.remainders_nested += 1;
                continue;
            }
            let found = self.element_of(other.as_bytes()).and_then(|e| {
                e.remainder_rays()
                    .find(|(o, _)| o.class == r.class && o.at == chain)
                    .and_then(|(o, ray)| Some((ray?.clone(), self.answered(other.as_bytes(), o))))
            });
            let Some((ray, answered)) = found else {
                // The index names a word whose record carries no remainder of
                // this class at this chain: the index and the record disagree.
                self.calluses.unrecorded += 1;
                continue;
            };
            if answered {
                self.calluses.answered += 1;
                continue;
            }
            if !ray.projective_eq(r_ray) {
                self.remainders_discordant += 1;
                continue;
            }
            // One callus per κ per pair of chains: the pair keyed in its
            // own byte order, so either arrival names it once.
            let (a, b) = (Self::chain_bytes(&r.at), Self::chain_bytes(&chain));
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let mut ck = prefix.clone();
            crate::transducer::address::put_key_bytes(&mut ck, &lo);
            crate::transducer::address::put_key_bytes(&mut ck, &hi);
            if self.calloused.contains(&ck) {
                continue;
            }
            self.calloused.insert(&ck, ());
            self.calluses_pending.push(Callus {
                class: r.class.clone(),
                ray: Box::new(r_ray.clone()),
                chains: [r.at.clone(), chain],
                words: [addr.clone(), other],
            });
        }
    }

    fn chain_bytes(chain: &[Vec<u8>]) -> Vec<u8> {
        let mut out = Vec::new();
        for c in chain {
            crate::transducer::address::put_key_bytes(&mut out, c);
        }
        out
    }

    /// The calluses raised and not yet stood, taken.
    pub(crate) fn take_calluses(&mut self) -> Vec<Callus> {
        std::mem::take(&mut self.calluses_pending)
    }

    /// A WORD LEFT, where it stands neither resident nor dialect: its
    /// remainders leave the index by the keys it filed.
    pub(crate) fn drop_remainders(&mut self, addr: &[u8]) {
        if self.genome.contains(addr) || self.dialect.contains(addr) {
            return;
        }
        if let Some(keys) = self.remainder_keys.remove(addr) {
            for k in keys {
                self.remainder_fibers.remove(&k);
            }
        }
        // Its re-fill marks leave with it: they stand under its address.
        let mut word = Vec::new();
        crate::transducer::address::put_key_bytes(&mut word, addr);
        let marks: Vec<Vec<u8>> = match self.refilled.fiber_at(&word) {
            Some(f) => f.map(|(k, _)| k).collect(),
            None => Vec::new(),
        };
        for k in marks {
            self.refilled.remove(&k);
        }
    }

    /// THE RECEPTOR'S READ: the words whose remainder of class `class` was
    /// perceived on the chart chain `at` — a keyed read, never a scan.
    #[cfg(test)]
    pub(crate) fn remainders_at(&self, at: &[Vec<u8>], class: &crate::transducer::sentence::ClassVector) -> Vec<Address> {
        let probe = Remainder { class: Box::new(class.clone()), at: at.to_vec(), answered: None };
        match self.remainder_fibers.get(&probe.index_key()) {
            Some(a) => vec![a.clone()],
            None => Vec::new(),
        }
    }

    /// Remainders indexed.
    #[cfg(test)]
    pub(crate) fn remainders_len(&self) -> usize {
        self.remainder_fibers.len()
    }

    /// RM1 (c) — THE OPEN REMAINDERS OF A CLASS: one prefix read under κ,
    /// each word, its chain and its ray, answered ones left as evidence.
    pub(crate) fn open_remainders_of(
        &self,
        class: &crate::transducer::sentence::ClassVector,
    ) -> Vec<(Address, Vec<Vec<u8>>, crate::transducer::alphabet::Rotor)> {
        let prefix = Remainder::class_prefix(class);
        let Some(fiber) = self.remainder_fibers.fiber_at(&prefix) else { return Vec::new() };
        let hits: Vec<(Vec<u8>, Address)> = fiber.map(|(k, a)| (k, a.clone())).collect();
        let mut out = Vec::new();
        for (key, addr) in hits {
            let Some(chain) = Remainder::chain_of_key(&key) else { continue };
            let open = self.element_of(addr.as_bytes()).and_then(|e| {
                e.remainder_rays()
                    .find(|(o, _)| *o.class == *class && o.at == chain && !self.answered(addr.as_bytes(), o))
                    .and_then(|(_, ray)| ray.cloned())
            });
            if let Some(ray) = open {
                out.push((addr, chain, ray));
            }
        }
        out
    }

    /// A remainder's key among this mind's re-fills: the word, then the
    /// remainder's own key.
    fn refill_key(addr: &[u8], r: &Remainder) -> Vec<u8> {
        let mut k = Vec::new();
        crate::transducer::address::put_key_bytes(&mut k, addr);
        crate::transducer::address::put_key_bytes(&mut k, &r.index_key());
        k
    }

    /// IS A REMAINDER ANSWERED (78.49): on its record by a court's seal, or
    /// re-spelled by this mind.
    pub(crate) fn answered(&self, addr: &[u8], r: &Remainder) -> bool {
        r.answered.is_some() || self.refilled.contains(&Self::refill_key(addr, r))
    }

    /// RM1 (c), 78.49 — THIS MIND RE-SPELLED a held word's open remainder of
    /// `class` at `at` by the word `spelling`: marked, by the word that
    /// spells it, in the mind — the record is left to a court's seal. `false`
    /// where the word's record carries no open remainder there.
    pub(crate) fn mark_refilled(
        &mut self,
        addr: &[u8],
        class: &crate::transducer::sentence::ClassVector,
        at: &[Vec<u8>],
        spelling: &Address,
    ) -> bool {
        let open = self.element_of(addr).and_then(|e| {
            e.remainder_rays().find(|(o, _)| *o.class == *class && o.at == at && !self.answered(addr, o)).map(|(o, _)| o.clone())
        });
        match open {
            Some(r) => {
                self.refilled.insert(Self::refill_key(addr, &r), spelling.clone());
                true
            }
            None => false,
        }
    }
}
