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
    Discharged { chord: usize, certificate: Box<PivotCertificate> },
    /// Several pivots discharge and none dominates on μ⃗: all are held as
    /// cells of the thought until a perception, a glue or a ruling collapses
    /// the superposition (12.6). Each carries the READING that put it here.
    Vortex { chord: usize, held: Vec<Held> },
    /// Several discharges with no bundle to measure μ⃗.
    Unmeasured { chord: usize, held: Vec<Held> },
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
                for h in held {
                    let m = mu_deflation(&h.certificate.mu);
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

/// ONE CELL OF A VORTEX: a certified discharge, with the reading that kept
/// it in superposition instead of discharging it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Held {
    /// The certificate itself — the pivot stands, whatever the vortex does.
    pub certificate: PivotCertificate,
    /// The lanes on which this candidate's μ⃗ is ABOVE the lane-wise
    /// minimum of the cell, when there was a μ⃗ to read.
    ///
    /// Empty means it IS the minimum and is held because it is not alone
    /// there. Non-empty names the lanes that keep it from being the
    /// minimum — an incomparability, not a defeat, and the two are
    /// different facts that a bare list of candidates could not tell apart.
    /// `None` is an UNMEASURED cell: no bundle, so no μ⃗ and no reading.
    pub above_minimum: Option<Vec<usize>>,
}

/// The three outcomes of selecting among certified discharges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Discharge {
    One(Box<PivotCertificate>),
    Vortex(Vec<Held>),
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
        ([], [c]) => Discharge::One(Box::new(c.clone())),
        // No bundle, so no μ⃗ and no reading: the cell is unmeasured, which
        // is a different thing from measured and incomparable.
        ([], _) => Discharge::Vortex(
            cands
                .iter()
                .map(|c| Held { certificate: c.clone(), above_minimum: None })
                .collect(),
        ),
        (vals, _) => match crate::economy::dominant(vals) {
            Some(i) => Discharge::One(Box::new(cands[i].clone())),
            // 12.6: the vortex holds what does not dominate — and now it
            // holds WHY. The lane-wise minimum is computed once; each cell
            // carries the lanes on which it stands above that minimum, so
            // an incomparable cell (some lane above) is distinguishable
            // from a tied one (no lane above, and not alone).
            None => {
                // The minimum is FOLDED from the candidates themselves —
                // seeded by the first, never by a zero standing in for a
                // measurement that was not taken (rule 5). This arm's
                // `vals` is non-empty by construction: the empty case is
                // matched above.
                let min: [u64; 4] = match vals.split_first() {
                    Some((seed, rest)) => rest
                        .iter()
                        .fold(*seed, |acc, v| std::array::from_fn(|l| acc[l].min(v[l]))),
                    // No μ⃗ at all is no reading. The empty case is matched
                    // above, so this arm is unreachable; if it is ever
                    // reached the cells are held UNMEASURED rather than
                    // measured against a minimum that was invented here.
                    None => {
                        return Discharge::Vortex(
                            cands
                                .iter()
                                .map(|c| Held { certificate: c.clone(), above_minimum: None })
                                .collect(),
                        )
                    }
                };
                Discharge::Vortex(
                    cands
                        .iter()
                        .zip(vals)
                        .map(|(c, v)| Held {
                            certificate: c.clone(),
                            above_minimum: Some(
                                (0..4).filter(|&l| v[l] > min[l]).collect(),
                            ),
                        })
                        .collect(),
                )
            }
        },
    }
}
