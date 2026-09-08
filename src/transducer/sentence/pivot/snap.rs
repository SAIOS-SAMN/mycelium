// A16, A20 — WHAT A PIVOT PRODUCES: the snapshot of one pivot, the
// discharge it certifies, and the cell it opened.
//
//! SEVERAL CERTIFIED DISCHARGES ARE A VORTEX. `select_discharge` compares
//! nothing: closure certifies a pivot after it is made, and no reading of
//! μ⃗ — no dominance, no minimum, no order on the lanes — chooses among the
//! candidates that discharged (12.6, 16.16 step 5, T13, T22, consolidated
//! 2026-09-08). What the vortex records is the OBSTRUCTION between its
//! cells: the lane reading and the two axes, which is why they are two
//! proofs and not one.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: Box<PivotCertificate> },
    /// SEVERAL pivots discharge: all are held as cells of the thought until
    /// a perception, a glue or a ruling collapses the superposition (12.6).
    /// `between` is the obstruction of each consecutive pair — the reading
    /// that says they are two proofs, recorded and never spent on a choice.
    Vortex { chord: usize, held: Vec<PivotCertificate>, between: Vec<Obstruction> },
    /// Several discharges with no bundle to measure μ⃗, so the obstruction
    /// has no lane reading in it — a third state, not an empty one.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: crate::transducer::address::Address },
}

impl Snap {
    /// Lane measure of an A16 snap. Discharged and Vortex read the
    /// certificate μ⃗ (before − after); TorsionFiled has no bundle μ⃗.
    pub fn lanes(&self) -> [u64; 4] {
        match self {
            Snap::Discharged { certificate, .. } => mu_deflation(&certificate.mu),
            Snap::Vortex { held, .. } => {
                let mut s = [0u64; 4];
                for c in held {
                    let m = mu_deflation(&c.mu);
                    for (t, d) in s.iter_mut().zip(&m) {
                        *t = t.saturating_add(*d);
                    }
                }
                s
            }
            Snap::TorsionFiled { .. } | Snap::Unmeasured { .. } | Snap::Closed { .. } | Snap::TorsionOpen { .. } | Snap::Demand { .. } => {
                [0; 4]
            }
        }
    }
}

pub(crate) fn mu_deflation(mu: &crate::transducer::sentence::form::Mu) -> [u64; 4] {
    match mu {
        crate::transducer::sentence::form::Mu::Measured { before, after } => {
            std::array::from_fn(|i| before[i].saturating_sub(after[i]))
        }
        crate::transducer::sentence::form::Mu::Unmeasured => [0; 4],
    }
}

/// WHAT SEPARATES TWO CERTIFIED DISCHARGES of one loop. A record, spent on
/// nothing: it is read when a collapse asks what the collapse resolved, and
/// it is never consulted to prefer a cell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obstruction {
    /// The axes the two pivots acted on. Two axes out of one loop are two
    /// directions, which is the whole reason both are kept.
    pub axes: (Axis, Axis),
    /// The lane reading of the second's μ⃗ against the first's, per lane.
    /// `None` when neither carries a measured μ⃗ — no bundle, so no reading,
    /// which is a third state and not four `Held`s.
    pub lanes: Option<[crate::economy::LaneMove; 4]>,
}

/// The obstruction between two certified discharges: what says they are two
/// proofs of one loop (23.9) rather than one proof twice.
pub(crate) fn obstruction(a: &PivotCertificate, b: &PivotCertificate) -> Obstruction {
    let lanes = match (&a.mu, &b.mu) {
        (Mu::Measured { after: x, .. }, Mu::Measured { after: y, .. }) => {
            Some(crate::economy::lane_moves(y, x))
        }
        _ => None,
    };
    Obstruction { axes: (a.axis, b.axis), lanes }
}

/// The three outcomes of a loop's trials. `One` is the single discharge;
/// `Vortex` is SEVERAL, held together with nothing chosen between them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(Box<PivotCertificate>),
    Vortex(Vec<PivotCertificate>),
    NoneDischarged,
}

/// A remainder cell's kind, read from the two hues. Hue 0 is void.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Created,
    Erased,
    Recoloured,
    Unchanged,
}

impl CellKind {
    pub fn of(before: u8, after: u8) -> CellKind {
        match (before == 0, after == 0) {
            (true, true) => CellKind::Unchanged,
            (true, false) => CellKind::Created,
            (false, true) => CellKind::Erased,
            (false, false) if before == after => CellKind::Unchanged,
            (false, false) => CellKind::Recoloured,
        }
    }
}

/// One candidate pivot: a held word that occupies the axis, written on the
/// chord on one side. Two candidates whose face words are one vertex are one.
pub(crate) struct Candidate {
    pub(crate) axis: Axis,
    pub(crate) by: PivotBy,
}

/// Hold a vortex cell by its after class, then by the discharging word's
/// rotor A1 key. Collapse reads `(class, r.key())`.
pub fn vortex_put(v: &mut Keyed<Keyed<PivotCertificate>>, c: PivotCertificate) {
    let after = c.after.clone();
    let k = c.by.move_key();
    v.nested_insert(after, k, c);
}

/// NOTHING IS COMPARED HERE. One certified discharge is the discharge;
/// several are a vortex, all of them held with their certificates. The μ⃗
/// of a candidate is not read in this function at all — it was read once,
/// inside the trial, to refuse a candidate that RAISED a lane, and a
/// candidate that survived that is certified. Choosing between two
/// certified discharges by any function of their lanes is a score
/// (12.6, 16.16 step 5, T13, T22; law 1).
pub(crate) fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    match cands {
        [] => Discharge::NoneDischarged,
        [c] => Discharge::One(Box::new(c.clone())),
        _ => Discharge::Vortex(cands.to_vec()),
    }
}

/// The obstruction between each consecutive pair of a vortex's cells. For
/// the two-cell vortex this is the pair; for more it is every adjacency in
/// the order they were tried, which is the fixed axis order of 12.6 and not
/// a ranking.
pub(crate) fn obstructions(cells: &[PivotCertificate]) -> Vec<Obstruction> {
    cells.windows(2).map(|w| obstruction(&w[0], &w[1])).collect()
}
