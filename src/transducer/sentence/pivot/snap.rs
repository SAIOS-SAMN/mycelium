// A16, A20 — WHAT A PIVOT PRODUCES: the snapshot of one pivot, the
// discharge it certifies, and the cell it opened.
//
//! A `Discharge` that cannot be ordered against another is held as a vortex
//! and never chosen between (rule 15): descent certifies, it does not
//! select.

use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Snap {
    /// A chord the tree gauge closed: its class was trivial (T2), never rank.
    Closed { chord: usize, order: Order },
    /// A held loop discharged on one occupied axis by a certified pivot.
    Discharged { chord: usize, certificate: PivotCertificate },
    /// Several pivots discharge and none dominates on μ⃗: all are held as
    /// cells of the thought until a perception, a glue or a ruling collapses
    /// the superposition (12.6).
    Vortex { chord: usize, held: Vec<PivotCertificate> },
    /// Several discharges with no bundle to measure μ⃗.
    Unmeasured { chord: usize, held: Vec<PivotCertificate> },
    TorsionFiled { chord: usize, ell: u32 },
    TorsionOpen { chord: usize },
    Demand { chord: usize, key: crate::transducer::a21::Address },
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
                    for i in 0..4 {
                        s[i] = s[i].saturating_add(m[i]);
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

/// The three outcomes of selecting among certified discharges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(PivotCertificate),
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

/// Dominance certifies (law 5): the lane-wise minimum of the candidates'
/// μ⃗ is computed once; a candidate whose μ⃗ equals that minimum is the
/// discharge. Several that match, or none that match (incomparable lanes),
/// are a vortex. No pairwise pass.
pub(crate) fn select_discharge(cands: &[PivotCertificate]) -> Discharge {
    let measured: Vec<[u64; 4]> = cands
        .iter()
        .filter_map(|c| match c.mu {
            Mu::Measured { after, .. } => Some(after),
            Mu::Unmeasured => None,
        })
        .collect();
    debug_assert!(
        measured.is_empty() || measured.len() == cands.len(),
        "μ⃗ is measured once per loop"
    );
    match (measured.as_slice(), cands) {
        ([], []) => Discharge::NoneDischarged,
        ([], [c]) => Discharge::One(c.clone()),
        ([], _) => Discharge::Vortex(cands.to_vec()),
        (vals, _) => match crate::economy::dominant(vals) {
            Some(i) => Discharge::One(cands[i].clone()),
            None => Discharge::Vortex(cands.to_vec()),
        },
    }
}
