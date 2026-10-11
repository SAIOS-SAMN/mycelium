// 62.1, MU-CLIENT §4.5 — THE MIND'S QUESTIONS ARE HOLES IN ITS OWN COMPLEX:
// the puncture / seal axis of the board, driven by what the transport reads.
//
//! A ring the transport holds at holonomy ≠ 1 (`close_ring`) is ASCENDED —
//! its cell filled through the one fill path — and where the cell reads a
//! CLASS, curvature that does not return, it is PUNCTURED: the class stays in
//! the mind's complex as a hole its boundary winds about, filed by its class
//! key. That hole is the question the mind carries.
//!
//! The hole is SEALED when the mind comes to hold what it asked: the ring
//! collapses (two arrivals agree around it), or a discharge answers its class
//! (`docket_discharged`). The cap re-ascends over the standing boundary at
//! the volume the hole was read with, and the class it kept is killed — the
//! mind COMPREHENDS. A cap whose boundary does not close leaves the hole
//! standing. Nothing here is scheduled or counted: each pivot is the reading
//! that drove it, and its outcome is returned whole.

use super::*;
use crate::transducer::cech::Capped;

impl Contemplation {
    /// PUNCTURE A HELD RING'S CELL where it reads a class: ascended first
    /// through the one fill path (`fill_meet`, which settles a decoherence
    /// pending there), then opened into a hole keeping the class. A cell that
    /// fills flat or closes at a period asks nothing and stands as it filled.
    pub(crate) fn ask(&mut self, cell: &[u8]) {
        let _ = self.fill_meet(cell);
        let Some(Fill::Class(..)) = self.complex.simplex(cell).map(|s| &s.fill) else { return };
        if let cech::Punctured::Opened(Fill::Class(kappa, _)) = self.complex.puncture(cell) {
            self.questions.nested_insert(kappa.class_key().as_bytes(), cell, ());
        }
    }

    /// SEAL A HOLE: its cap re-ascended over its standing boundary at the
    /// volume it was read with (read, never written). Where the cap seals,
    /// the class it kept leaves the questions and the cap's reading settles
    /// what was pending on the cell; otherwise the hole stands as it was.
    pub(crate) fn answer_hole(&mut self, cell: &[u8]) -> Capped {
        let Some(volume) = self.complex.hole_volume(cell) else { return Capped::Absent };
        let capped = self.complex.seal(cell, |_| volume);
        if let Capped::Sealed { cap, killed } = &capped {
            if let Fill::Class(kappa, _) = killed {
                self.unfile_question(kappa.class_key().as_bytes(), cell);
            }
            self.settle_decoherence(cell, &CoboundaryOutcome::Attached(cap.clone()));
        }
        capped
    }

    /// A DISCHARGE ANSWERED `class_key`: every hole keeping that class is
    /// sealed — read at the class's own key, never by walking the holes. An
    /// entry whose hole no longer stands (its vertex was vented, its holes
    /// capped there) leaves.
    pub(crate) fn answer_questions(&mut self, class_key: &[u8]) {
        let cells: Vec<Vec<u8>> = match self.questions.get(class_key) {
            Some(q) => q.scoped().map(|(k, ())| k).collect(),
            None => return,
        };
        for cell in cells {
            if let Capped::Absent = self.answer_hole(&cell) {
                if self.complex.hole_volume(&cell).is_none() {
                    self.unfile_question(class_key, &cell);
                }
            }
        }
    }

    fn unfile_question(&mut self, class_key: &[u8], cell: &[u8]) {
        if let Some(q) = self.questions.get_mut(class_key) {
            q.remove(cell);
            if q.is_empty() {
                self.questions.remove(class_key);
            }
        }
    }
}
