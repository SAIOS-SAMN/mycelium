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
//! at every hypha that holds its receipt.
//!
//! Issue #118 — the reading visits the signing events and nothing else. The
//! hot ledger keeps where each stands (`ReceiptChain::signing_stamps`,
//! `kernel‖stamp‖n`), and the events of this court's own folded line are
//! read once from each era as the reading first meets it
//! ([`FoldedSignings`]): a closed era returns nothing new. A crossing that
//! reads one kernel reads that kernel's fibre alone. The reading spends the
//! hold's stroke one read per event, and one per receipt of an era read for
//! the first time; a stroke spent answers `StrokeSpent`, never a reading cut
//! short. The reading itself is derived on every ask and never kept
//! (Concordance 1.13).

use super::Court;
use crate::chronometry::chain::signing::signing_key;
use crate::chronometry::{ActKind, Cite, Place, Stamp};
use crate::court::admission::Reconstructed;
use crate::network::nerve::Keyed;
use crate::physics::laws::clauses::{
    history, judge, read_alignment, register, Alignment, AlignmentRefusal, SigningEvent, Signing, SigningRefusal, Verdict,
};

/// Issue #118 — THE SIGNING EVENTS OF THIS COURT'S OWN FOLDED LINE, keyed
/// `kernel‖stamp‖n` as the hot ledger keys them: each event a copy of the
/// citation its archived receipt carries, read from the era once. Which
/// eras were read is written as coordinates: the root of the last one read
/// (the own line's eras stand in root order), how many were read, and the
/// mark of the own line's era tally when the last was read, so an ask with
/// no new era reads nothing of the archive.
#[derive(Debug, Default)]
pub(crate) struct FoldedSignings {
    events: Keyed<SigningEvent>,
    through: Option<Place>,
    eras: u64,
    mark: u64,
}

impl FoldedSignings {
    /// How many signing events were read from the folded line.
    pub(crate) fn len(&self) -> usize {
        self.events.len()
    }
}

/// THE EVENT A SIGNING CITATION IS, at the receipt `at` that carries it.
fn event_of(at: Stamp, c: &Cite) -> Option<SigningEvent> {
    match c {
        Cite::Signing { kernel, reserve, verdict } => Some(SigningEvent { at, kernel: *kernel, reserve: *reserve, verdict: (**verdict).clone() }),
        _ => None,
    }
}

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
    pub fn alignment(&mut self, only: Option<[u8; 32]>, reads: &mut crate::physics::Budget) -> Result<Alignment, AlignmentRefusal> {
        let held = register().map_err(|r| AlignmentRefusal::Unregistered { id: r.id })?;
        let ids: Vec<_> = held.into_iter().map(|r| r.clause.id).collect();
        let events = self.signing_events(only.as_ref(), reads)?;
        Ok(Alignment { kernels: read_alignment(&ids, &events, only), history: history() })
    }

    /// EVERY SIGNING EVENT THIS COURT HOLDS, `only`'s where one kernel is
    /// named, in the ledger's stamp order: its own folded line's, then every
    /// node held hot that a signature reached; each once, by its stamp and
    /// its place among the receipt's citations. One read per event.
    fn signing_events(&mut self, only: Option<&[u8; 32]>, reads: &mut crate::physics::Budget) -> Result<Vec<SigningEvent>, AlignmentRefusal> {
        self.read_folded_eras(reads)?;
        // `stamp‖n`: the ledger's stamp order, the key's tail past the kernel.
        let mut found: Keyed<SigningEvent> = Keyed::new();
        let folded = match only {
            Some(k) => self.folded_signings.events.fibre_at(k),
            None => Some(self.folded_signings.events.scoped()),
        };
        for (key, e) in folded.into_iter().flatten() {
            if !reads.spend() {
                return Err(AlignmentRefusal::StrokeSpent);
            }
            let Some(order) = key.get(32..) else { continue };
            found.insert(order, e.clone());
        }
        for (at, n) in self.chain.signing_stamps(only) {
            if !reads.spend() {
                return Err(AlignmentRefusal::StrokeSpent);
            }
            if self.unreached.contains(&Stamp::line_key(&at.vk, at.place())) {
                continue;
            }
            let Some(r) = self.chain.node(&at) else { continue };
            let Some(e) = usize::try_from(n).ok().and_then(|n| r.cites.get(n)).and_then(|c| event_of(at, c)) else { continue };
            let Some(order) = signing_key(&e.kernel, &at, n).get(32..).map(<[u8]>::to_vec) else { continue };
            found.insert(order, e);
        }
        Ok(found.scoped().map(|(_, e)| e.clone()).collect())
    }

    /// THE OWN FOLDED LINE'S SIGNING EVENTS BROUGHT UP TO THE ARCHIVE: each
    /// own era not read yet is read once, its receipts' signing citations
    /// filed by their coordinates. Nothing of the archive is read where no
    /// own era was filed since the last ask (the own era tally's mark), and
    /// only the eras past the last one read otherwise. An own era held below
    /// the last one read (the count disagrees) reads the folded line again
    /// from its first era. An era whose payload does not read is passed
    /// over, as every era reader passes it (`LineHolder::page_eras`). Each
    /// era is read whole or not at all: a stroke spent within one leaves it
    /// unread and answers `StrokeSpent`.
    fn read_folded_eras(&mut self, reads: &mut crate::physics::Budget) -> Result<(), AlignmentRefusal> {
        // No own era filed reads as the tally never filed: none, no mark.
        let tally = match self.era_tallies.get(&self.identity) {
            Some(t) => *t,
            None => crate::chronometry::LineTally::default(),
        };
        if tally.mark == self.folded_signings.mark {
            return Ok(());
        }
        let mut memo = std::mem::take(&mut self.folded_signings);
        let read = self.read_eras_into(&mut memo, tally.filed, reads);
        if read.is_ok() {
            memo.mark = tally.mark;
        }
        self.folded_signings = memo;
        read
    }

    /// The own eras past `memo`'s last, read into it; `filed` is how many
    /// own eras the archive holds.
    fn read_eras_into(&self, memo: &mut FoldedSignings, filed: u64, reads: &mut crate::physics::Budget) -> Result<(), AlignmentRefusal> {
        let past = |memo: &FoldedSignings| match memo.through {
            Some(t) => self.archive.partition_point(|e| e.root.at <= t),
            None => 0,
        };
        let fresh = self.archive.get(past(memo)..).into_iter().flatten().filter(|e| e.signer == self.identity).count() as u64;
        if memo.eras.checked_add(fresh) != Some(filed) {
            *memo = FoldedSignings::default();
        }
        let run = self.archive.get(past(memo)..).into_iter().flatten().filter(|e| e.signer == self.identity);
        for era in run {
            let mut events = Vec::new();
            for r in self.era_receipts(era).into_iter().flatten() {
                if !reads.spend() {
                    return Err(AlignmentRefusal::StrokeSpent);
                }
                if r.kind != ActKind::Domain {
                    continue;
                }
                for (n, c) in r.cites.iter().enumerate() {
                    if let Some(e) = event_of(r.stamp, c) {
                        events.push((signing_key(&e.kernel, &r.stamp, n as u64), e));
                    }
                }
            }
            for (key, e) in events {
                memo.events.insert(key, e);
            }
            memo.through = Some(era.root.at);
            memo.eras += 1;
        }
        Ok(())
    }
}
