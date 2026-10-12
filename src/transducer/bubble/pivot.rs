// MU-CLIENT §4.5 — THE MIND'S PIVOTS ON ITS OWN COMPLEX: every change of
// dimension the contemplation makes is one of the six pivots, applied at its
// coordinate through `Complex::apply`, its outcome returned whole.
//
//! ASCEND. A cell the climb minted over standing faces is sealed by its own
//! vertices through the ascend pivot — never filled around it — so a cell
//! whose boundary lost a face, carries a remainder, or stands at a hole
//! refuses by name. A tether's own reading (an edge's fill, which decides a
//! merge) is no pivot and is read as it is.
//!
//! DRAIN AND DESCEND ARE THE FIELD'S EVENTS. A tether condensed or a face
//! sealed winds its strain into the mind's field on its line, threading the
//! tubes and orbits its modes land on. Where an orbit RETURNS (harmonic: its
//! tension vented) or a tube's net flux CANCELS (luminal: it carries no
//! field), the field releases every line threading it, and the mind lets
//! those lines go (`release`): a released tether's read cells DESCEND first
//! (holonomic: a boundary relaxing no longer closes), their fills handed out,
//! and the tether DRAINS; each unwinds its strain back into the field. A hole
//! over the tether — a question the mind still carries — pins it and it
//! stands. Nothing is polled, stamped or windowed. A vertex the mind lets go
//! releases its whole star the same way (`Complex::vent_vertex`).
//!
//! CONDENSE. Two held thoughts the transport relates stand in tension, and
//! the tension is a TETHER: condensed with its transition. Where a tether
//! already stands on the pair carrying the same transport, nothing changes.
//! Where the tension re-formed on another transport, the standing tether
//! DRAINS first — its strain vented into the field, its class cauterized out
//! where it carried one — and the new one condenses: the conjugate pair,
//! never an overwrite. A tether pinned beneath a read cell does not drain
//! (63.2): the read cell's class stands on it, and the new transport is not
//! written over it.

use super::*;
use crate::transducer::cech::{Applied, Ascended, Capped, Condensed, Drained, Pivot, Punctured};
use crate::transducer::element::Versor;

impl Contemplation {
    /// CONDENSE the tension between `a` and `b` into a tether carrying
    /// `rotor` and `carriage`, with `volume` per lane; a standing tether on
    /// another transport drains first. Answers what the condense did.
    pub(crate) fn tether(&mut self, a: &[u8], b: &[u8], rotor: Rotor, carriage: Versor, volume: [u64; 4]) -> Condensed {
        let pair = Pivot::Condense(a.to_vec(), b.to_vec(), Box::new(rotor.clone()), carriage.clone());
        match self.complex.apply(&pair, |_| volume, volume) {
            Applied::Condensed(Condensed::AlreadyTethered) if !self.complex.tethered_as(a, b, &rotor, &carriage) => {
                let key = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts([a, b])).as_bytes().to_vec();
                match self.complex.apply(&Pivot::Drain(key), |_| volume, volume) {
                    Applied::Drained(Drained::Pinned) => Condensed::AlreadyTethered,
                    _ => match self.complex.apply(&pair, |_| volume, volume) {
                        Applied::Condensed(c) => c,
                        _ => Condensed::Absent,
                    },
                }
            }
            Applied::Condensed(c) => c,
            _ => Condensed::Absent,
        }
    }

    /// PUNCTURE a sealed cell into a hole keeping its class.
    pub(crate) fn puncture(&mut self, cell: &[u8]) -> Punctured {
        match self.complex.apply(&Pivot::Puncture(cell.to_vec()), |_| [0; 4], [0; 4]) {
            Applied::Punctured(p) => p,
            _ => Punctured::Absent,
        }
    }

    /// SEAL a hole: its cap re-ascended over its standing boundary at the
    /// volume the hole was read with (read, never written).
    pub(crate) fn seal(&mut self, hole: &[u8]) -> Capped {
        let Some(volume) = self.complex.hole_volume(hole) else { return Capped::Absent };
        match self.complex.apply(&Pivot::Seal(hole.to_vec()), |_| volume, volume) {
            Applied::Capped(c) => c,
            _ => Capped::Absent,
        }
    }

    /// LET GO THE LINES THE FIELD RELEASED (MU-CLIENT §4.5): each a tether or
    /// a cell whose tension relaxed in the field. A released CELL descends,
    /// its fill handed out. A released TETHER's read cells descend first,
    /// highest degree first — a boundary relaxing no longer closes — and the
    /// tether drains; a hole over it pins it, and it stands. A line no longer
    /// in the complex is gone already and nothing happens.
    pub(crate) fn release(&mut self, lines: &[Vec<u8>]) {
        for line in lines {
            match self.complex.degree_of(line) {
                Some(1) => {
                    let cofaces = self.complex.vertices_at(line).and_then(|ends| self.complex.cofaces_of(&ends));
                    let mut cells: Vec<(usize, Vec<u8>)> = match cofaces {
                        Some(c) => c.scoped().filter_map(|(k, ())| self.complex.degree_of(&k).filter(|d| *d >= 2).map(|d| (d, k))).collect(),
                        None => Vec::new(),
                    };
                    #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical coordinate: the degree, so every coface descends before its faces")]
                    cells.sort_by_key(|(d, _)| std::cmp::Reverse(*d));
                    for (_, k) in cells {
                        let _ = self.complex.apply(&Pivot::Descend(k), |_| [0; 4], [0; 4]);
                    }
                    let _ = self.complex.apply(&Pivot::Drain(line.clone()), |_| [0; 4], [0; 4]);
                }
                Some(d) if d >= 2 => {
                    let _ = self.complex.apply(&Pivot::Descend(line.clone()), |_| [0; 4], [0; 4]);
                }
                _ => {}
            }
        }
    }

    /// ASCEND a cell of the meet graph by its own vertices: its standing
    /// boundary sealed through the one fill, at the volume the climb minted
    /// it with (read, never written). A boundary a face has left, a remainder
    /// among its faces, or a hole there refuses by name — read as no outcome.
    /// Answers what the fill attached, as the meet graph reads it.
    pub(crate) fn ascend(&mut self, cell: &[u8]) -> CoboundaryOutcome {
        let Some((vertices, volume)) = self.complex.simplex(cell).map(|s| (s.vertices.clone(), s.volume)) else {
            return CoboundaryOutcome::Absent;
        };
        match self.complex.apply(&Pivot::Ascend(vertices), |_| volume, volume) {
            Applied::Ascended(Ascended::Sealed(fill)) => CoboundaryOutcome::Attached(fill),
            Applied::Ascended(Ascended::Open) => CoboundaryOutcome::Attached(Fill::Open),
            Applied::Ascended(Ascended::AlreadySealed) => CoboundaryOutcome::AlreadyFilled,
            _ => CoboundaryOutcome::Absent,
        }
    }
}
