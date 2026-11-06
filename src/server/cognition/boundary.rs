// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// CONCORDANCE §0, 4.1, 6.2a; MU-HOSTING §5a.4–5a.6; ROADMAP track 24 B4 —
// THE TUBE READS ITS DOMAIN'S SIGNATORY: a universe's act crosses its
// universal core tube only while the domain's sitting reserve converses on
// every network clause of the register served, read from the signing events
// on the ledger (`Court::alignment`), never from a stored flag.
//
//! A network clause the reserve has not read, or collides on, refuses the
//! crossing `NotSignatory`, naming the clause and the reading (the residue
//! where it collides). A kernel-standing clause never refuses: a veto of one
//! bounds only the acts the clause governs (`UniverseAct::governed_by`),
//! refused `Bounded` and named; every other act crosses. An alignment that
//! cannot be read whole (`StrokeSpent`, a register unread) answers
//! `Unaligned`: the crossing waits on the reading, never decided on part of
//! it. A key's own acts never come here (6.2a, keys for a key's own acts).

use super::Court;
use crate::court::admission::{Boundary, UniverseAct};
use crate::physics::laws::clauses::{register, AlignmentRefusal, ClauseReading, Standing};

impl Court {
    /// THE TUBE'S READING OF `domain`'s SIGNATORY for its act `act`, on
    /// the hold's stroke `reads`: the sitting reserve's alignment, clause
    /// by clause of the register served. `NotThroughCore` where the domain
    /// has no sitting reserve: no act of it reaches the vertex.
    pub(crate) fn signatory(&mut self, act: UniverseAct, domain: [u8; 32], reads: &mut crate::physics::Budget) -> Result<(), Boundary> {
        let Some(reserve) = self.reserve_of(&domain) else { return Err(Boundary::NotThroughCore { act, domain }) };
        let unaligned = |why| Boundary::Unaligned { act, domain, why };
        let held = register().map_err(|r| unaligned(AlignmentRefusal::Unregistered { id: r.id }))?;
        let read = self.alignment(Some(reserve), reads).map_err(unaligned)?;
        let readings = match read.kernels.into_iter().find(|k| k.kernel == reserve) {
            Some(k) => k.clauses,
            None => Vec::new(),
        };
        for r in held {
            let id = r.clause.id;
            let reading = match readings.iter().find(|c| c.id == id) {
                Some(c) => c.reading.clone(),
                None => ClauseReading::Unread,
            };
            match (r.clause.standing, reading) {
                (_, ClauseReading::Converses { .. }) => {}
                (Standing::Network, reading) => return Err(Boundary::NotSignatory { act, domain, clause: id, reading: Box::new(reading) }),
                (Standing::Kernel, ClauseReading::Vetoed { .. }) if act.governed_by(&id.law) => return Err(Boundary::Bounded { act, domain, clause: id }),
                (Standing::Kernel, _) => {}
            }
        }
        Ok(())
    }

    /// [`Court::signatory`] on a stroke of its own, for the acts whose
    /// seal takes none yet (65.5: the stroke is minted at the act's seal,
    /// the reading's one read per signing event spent from it, #118).
    pub(crate) fn signatory_now(&mut self, act: UniverseAct, domain: [u8; 32]) -> Result<(), Boundary> {
        let mut reads = self.physics.stroke();
        self.signatory(act, domain, &mut reads)
    }
}
