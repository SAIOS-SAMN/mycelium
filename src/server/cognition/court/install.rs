// 47.3, 13.9 — THE HOME INSTALL: what a second vantage installs, the
// citations an installed form notes, and the citers a slashed word
// re-dockets.
//
//! Split from `glue.rs` (MU-STANDARDS 1.1, one organ per file): the glue
//! converges two organs on one object there; the install that convergence
//! rests on is here.

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// 47.3: THE HOME INSTALLS on a second vantage. `witness` is the organ
    /// whose re-derivation the install rests on, and it is the author of the
    /// receipt this act seals (45.1): a book that cannot say which vantage
    /// an install stood on cannot re-derive the install.
    pub(crate) fn home_install(&mut self, addr: &[u8], witness: [u8; 32]) {
        let held = self.word_at(addr).cloned().or_else(|| self.provisional.get(addr).cloned());
        // L8y: the voice whose draw this address occupies is `drawers`,
        // and where `pending_take` already cleared it the speaker recorded
        // when the sentence was heard stands. Written as `[0; 32]`, every
        // word this court ruled had no speaker, so a use of it was credited
        // to the null identity and no speaker of a generator was ever paid
        // (26).
        let drawn = self
            .drawers
            .get(addr)
            .copied()
            .or_else(|| self.discoverer_of(addr));
        if self.basis.get(addr).is_none() {
            if let Some(w) = held.as_ref() {
                if !(w.is_connection() && w.letters().is_none()) {
                    match drawn {
                        Some(voice) => self.basis_push(w.clone(), voice),
                        // No draw: a word this court did not receive from a
                        // voice — a peer's glue, a restore. It has no
                        // speaker here, which is a fact and not a default
                        // (rule 11).
                        None => self.basis_push(w.clone(), [0; 32]),
                    }
                    self.fed_admit(addr, w.clone());
                }
            }
        }
        // 13.9: the home's install is its OWN word, ruled on a witness's
        // re-derivation, so it becomes this court's section on the chart.
        self.rule_own(addr);
        if let Some(w) = held {
            if w.is_rewrite() {
                if self.treasury.release_rewrite(addr).is_none() {
                    return;
                }
                self.retire_rewrite_inputs(&w);
            } else {
                let pay = self.verified_install_deflation(addr, &w);
                if pay.iter().all(|&x| x == 0) {
                    let _ = self.treasury.revoke_award(addr, false);
                    eprintln!(
                        "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                        addr[0], addr[1]
                    );
                } else if let Some(crate::economy::escrow::Escrowed { voices, anchor, .. }) =
                    self.treasury.release_award(addr)
                {
                    self.treasury.mint_claim(&voices, pay, anchor);
                }
            }
        } else {
            let _ = self.treasury.revoke_award(addr, false);
            eprintln!(
                "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                addr[0], addr[1]
            );
        }
        self.unlist(addr, false);
        self.provisional.remove(addr);
        self.court_upheld += 1;
        self.mark_persist();
        if let Some(w) = self.word_at(addr).cloned() {
            self.note_form_citations(addr, &w);
        }
        // 45.1: an install is an ADMITTED ACT, so it is a step of this
        // kernel's clock and a receipt in its chain. Without one the book
        // grew a ruled word at no position: nothing cited the peer receipts
        // admitted since the last step, and a reader replaying the chain
        // found the install nowhere in it.
        self.append_step(crate::chronometry::ActKind::Install, addr, witness);
        eprintln!("  court home installed {:02x}{:02x}… peer holds the stratum", addr[0], addr[1]);
    }

    pub(crate) fn note_form_citations(&mut self, addr: &[u8], w: &Element) {
        use crate::transducer::sentence::Vertex;
        let Some(f) = w.form() else {
            return;
        };
        for v in &f.vertices {
            let Vertex::Glued { address, .. } = v else {
                continue;
            };
            match self.cited_by.get_mut(address) {
                Some(bucket) => {
                    bucket.insert(addr, ());
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(addr, ());
                    self.cited_by.insert(address.clone(), bucket);
                }
            }
        }
        for (_, cert) in f.certificates.fiber(crate::network::nerve::ROOT) {
            let k = cert.key();
            match self.replay_memo.get_mut(addr) {
                Some(bucket) => {
                    bucket.insert(k, (true, self.chain.head_k()));
                }
                None => {
                    let mut bucket = crate::network::nerve::Keyed::new();
                    bucket.insert(k, (true, self.chain.head_k()));
                    self.replay_memo.insert(addr, bucket);
                }
            }
            eprintln!(
                "  court pivot certified {:02x}{:02x}… loop {} axis {} {:02x}{:02x}→{:02x}{:02x}",
                addr[0],
                addr[1],
                cert.chord,
                cert.axis.label(),
                cert.before[0],
                cert.before[1],
                cert.after[0],
                cert.after[1]
            );
        }
    }

    pub(crate) fn redocket_citers_of(&mut self, cited: &[u8]) {
        let Some(bucket) = self.cited_by.remove(cited) else {
            return;
        };
        let citers: Vec<Vec<u8>> = bucket
            .fiber(crate::network::nerve::ROOT)
            .map(|(k, _)| k)
            .collect();
        for c in citers {
            if let Some(w) = self.word_at(&c).cloned() {
                self.provisional.insert(c, w);
            }
        }
    }
}
