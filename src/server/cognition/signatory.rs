// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, MU-HOSTING §5a.4, ROADMAP track 24 B2 — THE SIGNATORY
// RECORD: each kernel's signing act, judged by reconstruction, recorded as
// an event on the court's receipt chain; and the alignment reading derived
// from those events, never stored beside them.
//
//! A signing act whose signature reconstructs (`Reconstructed::signing`) is
//! judged clause by clause against the register (`clauses::judge`) and its
//! verdict is sealed on the court's line as a `Domain` act citing
//! `Cite::Signing`: the kernel's key, the domain it is the sitting reserve
//! of (none where none), and the verdict, admitted or refused by name. A
//! refused act is recorded as an admitted one is: a `NotSignatory` is a
//! collision held as a residue naming both sides (Concordance A7), never
//! discarded. An act whose signature does not verify (`Malformed`,
//! `Unsigned`) is no kernel's act and is recorded nowhere; nor is a verdict
//! the register cannot give (`Unregistered`).
//!
//! The alignment reading reads every signing event the court holds: its own
//! line, hot and in its eras, and every peer's receipt that a signature
//! reached (a receipt held unreached is read by no reader, MU-HOSTING
//! §5a.9). A peer's signing receipt stays hot past its signer's fold
//! (`shed_folded_lines`), so an act judged at any hypha of the mesh reads
//! at every hypha that holds its receipt. The walk spends the hold's stroke
//! one read per receipt; a stroke spent answers `StrokeSpent`, never a
//! reading cut short.

use super::Court;
use crate::chronometry::{ActKind, Cite, KStepReceipt, Stamp};
use crate::court::admission::Reconstructed;
use crate::network::nerve::Keyed;
use crate::physics::laws::clauses::{
    history, judge, read_alignment, register, Alignment, AlignmentRefusal, SigningEvent, Signing, SigningRefusal, Verdict,
};

impl Court {
    /// A KERNEL'S SIGNING ACT HEARD (`ACT_LAWS` mode 8): read whole, its
    /// signature reconstructed, judged against the register, and its
    /// verdict sealed on this court's line. The verdict is answered only
    /// once sealed; a verdict the line refuses is answered `Unrecorded`.
    pub fn hear_signing(&mut self, b: &[u8]) -> Verdict {
        let held = register().map_err(|r| SigningRefusal::Unregistered { id: r.id })?;
        let s = Signing::from_bytes(b)?;
        let signed = Reconstructed::signing(&s)?;
        let verdict = judge(&held, &signed);
        let kernel = s.kernel_key;
        let cite = Cite::Signing { kernel, reserve: self.reserve_domain(&kernel), verdict: Box::new(verdict.clone()) };
        match self.seal_step(ActKind::Domain, [0; 4], kernel, |_| vec![cite.clone()]) {
            Some(_) => {
                self.mark_persist();
                verdict
            }
            None => Err(SigningRefusal::Unrecorded),
        }
    }

    /// The domain whose sitting reserve `kernel` is, where it is one.
    pub fn reserve_domain(&self, kernel: &[u8; 32]) -> Option<[u8; 32]> {
        self.tier_domain.get(kernel).copied().filter(|d| self.reserve_of(d) == Some(*kernel))
    }

    /// THE ALIGNMENT READING (`ACT_LAWS` mode 9): every kernel's, or
    /// `only`'s, derived from the signing events this court holds, with the
    /// signings of the past beside it as history.
    pub fn alignment(&self, only: Option<[u8; 32]>, reads: &mut crate::physics::Budget) -> Result<Alignment, AlignmentRefusal> {
        let held = register().map_err(|r| AlignmentRefusal::Unregistered { id: r.id })?;
        let ids: Vec<_> = held.into_iter().map(|r| r.clause.id).collect();
        let events = self.signing_events(reads)?;
        Ok(Alignment { kernels: read_alignment(&ids, &events, only), history: history() })
    }

    /// EVERY SIGNING EVENT THIS COURT HOLDS, in the ledger's stamp order:
    /// its own folded line read from its eras, then every node held hot that
    /// a signature reached; each once, by its stamp and its place among the
    /// receipt's citations.
    fn signing_events(&self, reads: &mut crate::physics::Budget) -> Result<Vec<SigningEvent>, AlignmentRefusal> {
        let mut found: Keyed<SigningEvent> = Keyed::new();
        let mut file = |r: &KStepReceipt, reads: &mut crate::physics::Budget| -> Result<(), AlignmentRefusal> {
            if !reads.spend() {
                return Err(AlignmentRefusal::StrokeSpent);
            }
            if r.kind != ActKind::Domain {
                return Ok(());
            }
            for (n, c) in r.cites.iter().enumerate() {
                let Cite::Signing { kernel, reserve, verdict } = c else { continue };
                let mut key = r.stamp.key();
                key.extend_from_slice(&(n as u64).to_be_bytes());
                found.insert(key, SigningEvent { at: r.stamp, kernel: *kernel, reserve: *reserve, verdict: (**verdict).clone() });
            }
            Ok(())
        };
        for e in self.archive.iter().filter(|e| e.signer == self.identity) {
            // An era whose payload does not read is passed over, as every
            // era reader passes it (`LineHolder::page_eras`).
            for r in self.era_receipts(e).into_iter().flatten() {
                file(&r, reads)?;
            }
        }
        for r in self.chain.nodes() {
            if self.unreached.contains(&Stamp::line_key(&r.stamp.vk, r.stamp.place())) {
                continue;
            }
            file(&r, reads)?;
        }
        Ok(found.scoped().map(|(_, e)| e.clone()).collect())
    }
}
