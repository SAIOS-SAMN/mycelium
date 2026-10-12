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
//! DRAIN AND DESCEND. At an orbit's close, a tether that carried no flux on
//! any lane over the whole measured aperture has relaxed: the cells standing
//! on relaxed tethers alone descend, their fills handed out, and the tethers
//! drain, their strain vented (`release_relaxed`). A vertex the mind lets go
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
        let key = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts([a, b])).as_bytes().to_vec();
        let condensed = match self.complex.apply(&pair, |_| volume, volume) {
            Applied::Condensed(Condensed::AlreadyTethered) if !self.complex.tethered_as(a, b, &rotor, &carriage) => {
                match self.complex.apply(&Pivot::Drain(key.clone()), |_| volume, volume) {
                    Applied::Drained(Drained::Pinned) => Condensed::AlreadyTethered,
                    _ => match self.complex.apply(&pair, |_| volume, volume) {
                        Applied::Condensed(c) => c,
                        _ => Condensed::Absent,
                    },
                }
            }
            Applied::Condensed(c) => c,
            _ => Condensed::Absent,
        };
        // A tether condensed now is measured from now.
        if condensed == Condensed::Tethered {
            self.tethered_at.insert(&key, self.now_k);
        }
        condensed
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

    /// DRAIN AND DESCEND WHERE THE TENSION RELAXED (MU-CLIENT §4.5, 78.37(3)),
    /// at an orbit's close over the measured aperture `[from, now]`. A tether
    /// that stood the whole aperture and carried no flux on any lane — this
    /// orbit or since `from` — has RELAXED; one a held ring stands on is held
    /// (the ring holds it). Every cell standing on relaxed tethers alone
    /// DESCENDS first, highest degree first, its fill handed out to the field;
    /// then each relaxed tether DRAINS, its strain vented. A tether beneath a
    /// cell whose boundary still carries flux is pinned there and stands. A
    /// tether read standing with no height of its own is measured from now:
    /// an unmeasured age releases nothing.
    pub(crate) fn release_relaxed(&mut self, from: u64) {
        use crate::transducer::cech::{simplex_key, sorted_verts};
        let held = self.held_edges();
        let tethers: Vec<Vec<u8>> = match self.complex.degrees.get(1) {
            Some(layer) => layer.scoped().map(|(k, _)| k).collect(),
            None => Vec::new(),
        };
        let mut relaxed: Keyed<()> = Keyed::new();
        for e in tethers {
            let Some(&at) = self.tethered_at.get(&e) else {
                self.tethered_at.insert(&e, self.now_k);
                continue;
            };
            let carried = self.flux.get(&e).is_some_and(|phi| phi.iter().any(|&p| p > 0)) || self.flowed_since(&e, from);
            if at <= from && !carried && !held.contains(&e) {
                relaxed.insert(&e, ());
            }
        }
        // The cells over relaxed tethers alone, read off each tether's
        // cofaces (the meet of its two stars), never a read of every cell.
        let mut cells: Keyed<usize> = Keyed::new();
        for (e, ()) in relaxed.scoped() {
            let Some(ends) = self.complex.vertices_at(&e) else { continue };
            let Some(cofaces) = self.complex.cofaces_of(&ends) else { continue };
            for (k, ()) in cofaces.scoped() {
                let Some(d) = self.complex.degree_of(&k).filter(|d| *d >= 2) else { continue };
                let Some(vs) = self.complex.vertices_at(&k) else { continue };
                let all_relaxed = vs.iter().enumerate().all(|(i, x)| {
                    vs[i + 1..].iter().all(|y| relaxed.contains(simplex_key(&sorted_verts([x, y])).as_bytes()))
                });
                if all_relaxed {
                    cells.insert(&k, d);
                }
            }
        }
        let mut cells: Vec<(usize, Vec<u8>)> = cells.scoped().map(|(k, d)| (*d, k)).collect();
        #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical coordinate: the degree, so every coface descends before its faces")]
        cells.sort_by_key(|(d, _)| std::cmp::Reverse(*d));
        for (_, k) in cells {
            let _ = self.complex.apply(&Pivot::Descend(k), |_| [0; 4], [0; 4]);
        }
        for (e, ()) in relaxed.scoped() {
            if !matches!(self.complex.apply(&Pivot::Drain(e.clone()), |_| [0; 4], [0; 4]), Applied::Drained(Drained::Pinned)) {
                self.tethered_at.remove(&e);
                self.flux_at.remove(&e);
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
