// 42 — THE KERNEL'S READS: the small accessors a caller outside the
// book asks the court for. Split from `kernel.rs` at the §1.1 600-line
// reading bound when the awaiting docket was added (21 step 3); the
// items are moved, not rewritten (§1.6).

use super::*;

impl Kernel {
    pub(crate) fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// The root over the ruled cover. Measured here, where it is asked
    /// for, rather than rebuilt on every ruling (see `refresh_ruled_root`).
    pub fn ruled_root(&mut self) -> [u8; 32] {
        self.measure_ruled_root()
    }

    pub fn court_federation_len(&self) -> usize {
        self.court_federation.len()
    }

    /// The demand vertices listed so far (50.5, L7): a counter read at the
    /// wire and by the tests, derived from the listings, never a gate.
    pub fn demands_listed(&self) -> u64 {
        self.court_demands_listed
    }

    pub fn offered(&self, addr: &[u8]) -> bool {
        self.offered.get(addr).is_some()
    }

    pub fn dialect_len(&self) -> usize {
        self.cocycle_at.len()
    }

    pub fn dialect_chain_len(&self) -> usize {
        self.dialect_chain.len()
    }

    pub fn heard_len(&self) -> usize {
        self.heard.len()
    }

    pub fn field_hot_len(&self) -> usize {
        self.field.len()
    }

    pub fn field_cold_len(&self) -> usize {
        self.field_cold.len()
    }

    pub fn settlements_hot_len(&self) -> usize {
        self.settlements.len()
    }

    pub fn settlements_cold_len(&self) -> usize {
        self.settlements_cold.len()
    }

}
