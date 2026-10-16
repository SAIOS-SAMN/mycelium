// 24.1, MU-CLIENT §4.13 — THE SHED: what this client stops holding, by what
// still references it — never by a window of its clock.

use crate::client::sdk::*;

impl Epigenome {
    /// A working-atlas read of a stood word is a use, stamped with the
    /// mind's OWN K (45.4). The height beside it is standing extent and is
    /// left alone; the lifecycle is the mind's, and a mind that hears
    /// nothing still thinks.
    pub(crate) fn touch(&mut self, addr: &[u8], now_k: u64) {
        if let Some(d) = self.dialect.get_mut(addr) {
            d.last_used_k = now_k;
            #[expect(clippy::disallowed_methods, reason = "rule 11 — one per use: a u64 of events cannot pass 2⁶⁴")]
            let n = d.uses.saturating_add(1);
            d.uses = n;
        }
    }

    /// MU-CLIENT §4.13 — A DIALECT WORD STANDS WHILE SOMETHING LIVE REFERENCES
    /// IT. The dialect is the mind's own basis, not yet ratified into the
    /// genome: it exists to span the representations of its standing
    /// thoughts, its open demands and its unjudged sentences, and has no
    /// right of its own past them — the inverse of door 1, where a thought
    /// lapses with the word it cites. Each entry `referenced` does not hold,
    /// and that waits on no citation (a withheld sentence is unjudged until
    /// its citations arrive, `resolve_withheld`), is dropped. No idle window,
    /// no count, no clock. The genome never sheds.
    ///
    /// Returns the addresses shed, so the carriers that hang off a dialect
    /// word — its utterance receipt above all (S12.6a) — leave with it.
    pub(crate) fn shed_unreferenced(&mut self, referenced: &impl Fn(&[u8], &Dialect) -> bool) -> Vec<Vec<u8>> {
        let waits = |d: &Dialect| d.withheld && !d.missing.is_empty();
        let drop: Vec<Vec<u8>> = self.dialect.scoped().filter(|(a, d)| !waits(d) && !referenced(a, d)).map(|(a, _)| a).collect();
        for a in &drop {
            self.drop_dialect(a);
        }
        #[expect(clippy::disallowed_methods, reason = "rule 11 — one per resident entry shed: a u64 of events cannot pass 2⁶⁴")]
        let n = self.shed_n.saturating_add(drop.len() as u64);
        self.shed_n = n;
        drop
    }
}
