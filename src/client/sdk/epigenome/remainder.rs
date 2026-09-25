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

/// What the voice made of the calluses raised: stood; unspelled (no held
/// constructor spells the ray — its class docketed as a demand); a chain
/// whose top did not read; a word whose admitting stamp it did not hold; a
/// form or an act that did not mint.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CallusTally {
    pub stood: u64,
    pub unspelled: u64,
    pub untopped: u64,
    pub uncited: u64,
    pub unminted: u64,
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
        for r in e.remainders() {
            let k = r.index_key();
            match self.remainder_fibers.get(&k) {
                Some(a) if a == addr => keys.push(k),
                Some(_) => self.remainders_coincident += 1,
                None => {
                    self.receive(addr, r);
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
    fn element_of(&self, addr: &[u8]) -> Option<&Element> {
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
    fn receive(&mut self, addr: &Address, r: &Remainder) {
        let prefix = Remainder::class_prefix(&r.class);
        let standing: Vec<(Vec<u8>, Address)> = match self.remainder_fibers.fiber_at(&prefix) {
            Some(f) => f.map(|(k, a)| (k, a.clone())).collect(),
            None => Vec::new(),
        };
        for (key, other) in standing {
            if other == *addr {
                continue;
            }
            let Some(chain) = Remainder::chain_of_key(&key) else { continue };
            let within = |a: &[Vec<u8>], b: &[Vec<u8>]| a.iter().all(|k| b.contains(k));
            if within(&r.at, &chain) || within(&chain, &r.at) {
                self.remainders_nested += 1;
                continue;
            }
            let ray = self
                .element_of(other.as_bytes())
                .and_then(|e| e.remainders().find(|o| o.class == r.class && o.at == chain).map(|o| (*o.ray).clone()));
            let Some(ray) = ray else { continue };
            if !ray.projective_eq(&r.ray) {
                self.remainders_discordant += 1;
                continue;
            }
            // One callus per κ per pair of chains: the pair keyed in its
            // own byte order, so either arrival names it once.
            let (a, b) = (Self::chain_bytes(&r.at), Self::chain_bytes(&chain));
            let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
            let mut ck = prefix.clone();
            crate::transducer::address::put_bytes(&mut ck, &lo);
            crate::transducer::address::put_bytes(&mut ck, &hi);
            if self.calloused.contains(&ck) {
                continue;
            }
            self.calloused.insert(&ck, ());
            self.calluses_pending.push(Callus {
                class: r.class.clone(),
                ray: r.ray.clone(),
                chains: [r.at.clone(), chain],
                words: [addr.clone(), other],
            });
        }
    }

    fn chain_bytes(chain: &[Vec<u8>]) -> Vec<u8> {
        let mut out = Vec::new();
        for c in chain {
            crate::transducer::address::put_bytes(&mut out, c);
        }
        out
    }

    /// The calluses raised and not yet stood, taken.
    pub(crate) fn take_calluses(&mut self) -> Vec<Callus> {
        std::mem::take(&mut self.calluses_pending)
    }

    /// The word a held address is, for the voice's citation.
    pub(crate) fn held_element(&self, addr: &[u8]) -> Option<Element> {
        self.element_of(addr).cloned()
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
    }

    /// THE RECEPTOR'S READ: the words whose remainder of class `class` was
    /// perceived on the chart chain `at` — a keyed read, never a scan.
    pub fn remainders_at(&self, at: &[Vec<u8>], class: &crate::transducer::sentence::ClassVector) -> Vec<Address> {
        let probe = Remainder {
            class: Box::new(class.clone()),
            ray: Box::new(crate::transducer::alphabet::Rotor::identity()),
            at: at.to_vec(),
        };
        match self.remainder_fibers.get(&probe.index_key()) {
            Some(a) => vec![a.clone()],
            None => Vec::new(),
        }
    }

    /// Remainders indexed.
    pub fn remainders_len(&self) -> usize {
        self.remainder_fibers.len()
    }
}
