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
//! the tension is a TETHER: condensed with its transition. A tether is a
//! FIBRE: where one already stands on the pair, the arriving transport
//! FLEXES it (`Complex::condense`) — their composite read by its modes
//! against the tether's own axis as stretch, bend, twist, fold and slide —
//! the fibre carries the new transport, the cells over it re-read, and the
//! flex's strain winds into the field on the tether's line. The field
//! decides what holds: a stretch's boost never returns, dissipates on a
//! beat and releases the line, and the tether drains — it BREAKS; a turn
//! rings until it returns. A hole over the tether pins it rigid.

use super::*;
use crate::transducer::cech::{Applied, Ascended, Capped, Condensed, Pivot, Punctured, Thawed};
use crate::transducer::element::Versor;

/// One flex counted on the mind's reading by what it did to the fibre —
/// rigid, torsion, field, lift, or no fibre to read it on (counted as lift:
/// work) — and, pinned, beside it.
fn note_flex(carried: Option<crate::physics::capacitor::Carried>, pinned: bool) {
    use crate::physics::capacitor::Carried;
    let slot = match carried {
        Some(Carried::Rigid) => 0,
        Some(Carried::Torsion) => 1,
        Some(Carried::Field) => 2,
        Some(Carried::Lift) | None => 3,
    };
    crate::observe::note_mind(|m| {
        m.flex[slot] += 1;
        m.flex[4] += u64::from(pinned);
    });
}

impl Contemplation {
    /// 78.74 — A THOUGHT AS A PIVOT: the barycentre of its bodies' vertex
    /// centroids, each weighed by its compartments, as a conformal point, and
    /// the thought's mass. `None` for a thought with no perceived body (a
    /// word, a sentence): it has no place to pivot at.
    pub(crate) fn pivot_of(&self, key: &[u8]) -> Option<(crate::transducer::cga::Mv, num_rational::BigRational)> {
        let b = self.bubbles.get(key)?;
        let bodies: Vec<(Vec<num_rational::BigRational>, num_rational::BigRational)> = b
            .spores
            .scoped()
            .flat_map(|(_, shapes)| {
                shapes
                    .iter()
                    .filter_map(|s| {
                        let mass = num_rational::BigRational::from_integer(i64::try_from(s.blade.support.len()).ok()?.into());
                        let point = s.blade.vertex_point()?;
                        Some((crate::transducer::cga::base_at(&point, usize::from(point.n()))?, mass))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        let star = crate::physics::tensegrity::barycentre(&bodies)?;
        Some((crate::transducer::cga::point_at(&star.centre), star.mass))
    }

    /// 78.74 — THE TETHER AS A FIBRE between its two thoughts' pivots, read
    /// off their coordinates whenever it is asked, never held. `None` where
    /// either thought has no pivot or the two stand at one point.
    pub(crate) fn fibre_of(&self, a: &[u8], b: &[u8]) -> Option<crate::physics::capacitor::Fibre> {
        let ((pa, ma), (pb, mb)) = (self.pivot_of(a)?, self.pivot_of(b)?);
        crate::physics::capacitor::Fibre::new(pa, pb, ma, mb)
    }

    /// CONDENSE the tension between `a` and `b` into a tether carrying
    /// `rotor` and `carriage`, with `volume` per lane. Where one stands, the
    /// arriving transport FLEXES the fibre (`Complex::condense`), and what the
    /// flex did is read off the fibre by its split (78.74): torsion twists it
    /// and does NO WORK — nothing is wound into the field for it; a field
    /// (charge between its levels, the stretch) or a lift (its pivots moved,
    /// the reshape) does work, and its composite winds into the field on the
    /// tether's own line, where the field decides what holds — a stretch that
    /// never returns dissipates, releases the line, and the tether breaks. A
    /// tether whose thoughts hold no pivots is read as working. A new tether
    /// FORMS the shortcuts its pivots bind (`form_through`).
    pub(crate) fn tether(&mut self, a: &[u8], b: &[u8], rotor: Rotor, carriage: Versor, volume: [u64; 4]) -> Condensed {
        use crate::physics::capacitor::Carried;
        let pair = Pivot::Condense(a.to_vec(), b.to_vec(), Box::new(rotor), carriage);
        let condensed = match self.complex.apply(&pair, |_| volume, volume) {
            Applied::Condensed(c) => c,
            _ => Condensed::Absent,
        };
        match &condensed {
            Condensed::Flexed(c) => {
                let carried = self.carried_by(a, b, c);
                if carried != Some(Carried::Torsion) && carried != Some(Carried::Rigid) {
                    let key = crate::transducer::cech::simplex_key(&crate::transducer::cech::sorted_verts([a, b])).as_bytes().to_vec();
                    self.complex.flow_out(crate::transducer::cech::Flow { motion: c.as_ref().clone(), line: crate::transducer::cech::Thread::Wound(key) });
                }
                note_flex(carried, false);
            }
            Condensed::Pinned(c) => {
                let carried = self.carried_by(a, b, c);
                note_flex(carried, true);
            }
            Condensed::Tethered => self.form_through(a, b),
            _ => {}
        }
        condensed
    }

    /// What a motion does to the fibre between `a` and `b`, read off its
    /// modes (`Fibre::reads`): rigid, torsion, field or lift. `None` where
    /// there is no fibre or the motion's modes do not read.
    fn carried_by(&self, a: &[u8], b: &[u8], motion: &crate::transducer::cga::Mv) -> Option<crate::physics::capacitor::Carried> {
        let fibre = self.fibre_of(a, b)?;
        let mut stroke = crate::physics::Physics::default().stroke_n(crate::transducer::cga::modes_need(motion)?);
        let modes = crate::physics::capacitor::modes_of(motion, motion.n(), &mut stroke)?;
        Some(fibre.reads(&modes))
    }

    /// FROZEN-IN FLUX (MU-CLIENT §4.5): the tether on `line` carried by the
    /// field's flow `increment` — moved with it, its cells re-read, and the
    /// carriage read off its fibre as any flex is. Answers whether it moved.
    pub(crate) fn carry(&mut self, line: &[u8], increment: &crate::transducer::cga::Mv) -> bool {
        if !self.complex.carry(line, increment) {
            return false;
        }
        let carried = match self.complex.vertices_at(line).as_deref() {
            Some([a, b]) => self.carried_by(a, b, increment),
            _ => None,
        };
        note_flex(carried, false);
        crate::observe::note_mind(|m| m.flex[5] += 1);
        true
    }

    /// 78.74 — THE DOTS CONNECTED, BY THE FIBRATION: switches of fibres
    /// sharing a pivot compose, `S₂₃S₁₂` carrying `P₁` through `P₂` to `P₃`.
    /// The new tether `a–b` shares a pivot with every other tether at `a` or
    /// `b`, `y–z`. Where `x` and `z` are not yet tethered, `z` is placed on
    /// the fibre `x–y` by its projection (`Fibre::project`, the ratio of the
    /// two pivots' pulls on it): at or above the switch — `x` pulls `z` at
    /// least as hard as `y` does — the shortcut forms, carrying the path's
    /// composite transport `g_xz = g_xy ∘ g_yz`, so the triangle it closes
    /// is flat; below it, `z` belongs to `y`'s side and nothing forms. One
    /// step: a formed shortcut forms nothing further.
    fn form_through(&mut self, a: &[u8], b: &[u8]) {
        use crate::transducer::cech::coefficient::Coefficient;
        use crate::transducer::cech::cover::Cochain;
        let mut formed: Vec<(Vec<u8>, Vec<u8>, crate::transducer::cech::coefficient::Turned)> = Vec::new();
        for (x, y) in [(a, b), (b, a)] {
            for edge in self.complex.link(y, 1) {
                let Some(ends) = self.complex.vertices_at(&edge) else { continue };
                let Some(z) = ends.iter().find(|v| v.as_slice() != y) else { continue };
                if z.as_slice() == x || self.complex.get(&crate::transducer::cech::sorted_verts([x, z.as_slice()])).is_some() {
                    continue;
                }
                let (Some(xy), Some((pz, _))) = (self.fibre_of(x, y), self.pivot_of(z)) else { continue };
                if xy.project(&pz).is_none_or(|mu| mu < num_rational::BigRational::from_integer(1.into())) {
                    crate::observe::note_mind(|m| m.flex[8] += 1);
                    continue;
                }
                let path = <crate::transducer::cech::Complex as Cochain<crate::transducer::cech::coefficient::Turned>>::crossing(&self.complex, x, y)
                    .zip(<crate::transducer::cech::Complex as Cochain<crate::transducer::cech::coefficient::Turned>>::crossing(&self.complex, y, z))
                    .and_then(|(g, h)| g.compose(&h));
                if let Some(g) = path {
                    formed.push((x.to_vec(), z.clone(), g));
                }
            }
        }
        for (x, z, g) in formed {
            let volume = match self.complex.get(&crate::transducer::cech::sorted_verts([x.as_slice()])) {
                Some(s) => s.volume,
                None => [1, 0, 0, 0],
            };
            let pair = Pivot::Condense(x, z, Box::new(g.rotor), g.carriage);
            if let Applied::Condensed(Condensed::Tethered) = self.complex.apply(&pair, |_| volume, volume) {
                crate::observe::note_mind(|m| m.flex[7] += 1);
            }
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

    /// LET GO THE LINES THE FIELD RELEASED (MU-CLIENT §4.5, §4.13): each a
    /// thought, a tether or a cell whose tension relaxed in the field, with
    /// why. A THOUGHT's own line is read by its cause (`line::release_line`):
    /// a grounded thought winds on past its return, and every other release
    /// lets the thought go. A released CELL descends by THE CROSS
    /// (`Complex::descend`): its class kept as a hole through the puncture,
    /// its content handed down. A released TETHER's read cells cross first,
    /// highest degree first — a boundary relaxing no longer closes — and the
    /// tether drains where no class stands over it; a hole over it — a class
    /// kept, or a question still carried — pins it, and it stands condensed.
    /// A line no longer in the complex is gone already and nothing happens.
    pub(crate) fn release(&mut self, lines: &[(Vec<u8>, crate::transducer::cech::orbit::Released)]) {
        for (line, why) in lines {
            if self.release_line(line, *why) {
                continue;
            }
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
                    if let Applied::Drained(crate::transducer::cech::Drained::Vacuum | crate::transducer::cech::Drained::Cauterized(_)) =
                        self.complex.apply(&Pivot::Drain(line.clone()), |_| [0; 4], [0; 4])
                    {
                        crate::observe::note_mind(|m| m.flex[6] += 1);
                    }
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
    ///
    /// THE DIPOLE'S READ-BACK: at a hole the cross left, ascend is the
    /// `seal` — the class the throat kept returns to a chamber over the
    /// standing boundary. Where the boundary no longer closes while its
    /// vertices stand, the class cannot be read back and is SEALED BY COLD
    /// (ruled 2026-10-21: the vacuum's face of the throat;
    /// the vacuum holds the frozen class): frozen in place, the hole closed
    /// and its boundary unpinned, never left as a wound that pins for ever.
    /// At a class the vacuum holds, ascend is the THAW: the chamber returns
    /// where its boundary closes again. (Where a vertex's word has left, the
    /// boundary drains with it and the hole is cauterized by heat:
    /// `release_vertex`.)
    pub(crate) fn ascend(&mut self, cell: &[u8]) -> CoboundaryOutcome {
        if self.complex.is_frozen(cell) {
            return match self.complex.thaw(cell) {
                Thawed::Sealed(fill) => CoboundaryOutcome::Attached(fill),
                Thawed::Held => CoboundaryOutcome::Attached(Fill::Open),
                Thawed::Absent => CoboundaryOutcome::Absent,
            };
        }
        if self.complex.is_hole(cell) {
            return match self.seal(cell) {
                Capped::Sealed { cap, .. } => CoboundaryOutcome::Attached(cap),
                // The boundary was read and does not close, or no longer
                // stands as a boundary: either way the class is not read
                // back, and the vacuum holds it.
                Capped::Open => {
                    let _ = self.complex.freeze(cell);
                    CoboundaryOutcome::Attached(Fill::Open)
                }
                Capped::Absent => {
                    let _ = self.complex.freeze(cell);
                    CoboundaryOutcome::Absent
                }
            };
        }
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
