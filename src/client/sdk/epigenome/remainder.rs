// 78.37(4), RM1 — THE MIND'S REMAINDERS BY WHERE THEY WERE PERCEIVED: the
// index the receptor reads, derived from the words' torsion records.

use crate::client::sdk::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

impl Epigenome {
    /// A WORD STANDS — resident or dialect: each remainder its torsion record
    /// carries is filed at its residue's chart chain ‖ κ. ONE REMAINDER PER
    /// CLASS at a chart (78.37(4)): a second word whose remainder coincides
    /// with one standing is not filed over it and is counted — two
    /// remainders of one class are the receptor's question, never an
    /// overwrite.
    pub(crate) fn file_remainders(&mut self, addr: &Address, e: &Element) {
        let mut keys = Vec::new();
        for k in e.remainders().map(|r| r.index_key()) {
            match self.remainder_fibers.get(&k) {
                Some(a) if a == addr => keys.push(k),
                Some(_) => self.remainders_coincident += 1,
                None => {
                    self.remainder_fibers.insert(&k, addr.clone());
                    keys.push(k);
                }
            }
        }
        if !keys.is_empty() {
            self.remainder_keys.insert(addr.as_bytes(), keys);
        }
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
        let probe = crate::transducer::element::Remainder {
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
