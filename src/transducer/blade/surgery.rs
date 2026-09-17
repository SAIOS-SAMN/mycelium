// 50.8, A6 — TOPOLOGICAL SURGERY MONOID: exact cell-complex cobordisms (Fill, Puncture, Merge, Split).
//
//! The conformal group SO(n+1, 1) consists exclusively of homeomorphisms
//! (preserving Betti numbers: Δβ₀ = 0, Δβ₁ = 0). A transformation that changes
//! genus or connectivity has no conformal versor.
//!
//! A surgery is an exact cell-complex cobordism acting on the Betti numbers
//! and the support. Surgeries compose as a graded monoid:
//! Δβ(S₁ ∘ S₂) = Δβ(S₁) + Δβ(S₂).
//!
//! Any non-homeomorphic transformation factors as Φ = V ∘ S, where S is the
//! topological surgery matching Betti numbers, and V is the conformal versor
//! aligning the resulting homeomorphic core.

use super::body::{shape_of, topology, Shape};
use crate::transducer::cga::Mv;

/// The elementary topological surgery operations on 2D and 3D cell complexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SurgeryKind {
    /// Capping 1-cycles by attaching 2-cells (Δβ₁ = -k, cavity fill).
    FillHole,
    /// Deleting interior 2-cells opening 1-cycles (Δβ₁ = +k, puncture).
    Puncture,
    /// Attaching 1-cells identifying 0-cycles (Δβ₀ = -k, bridge merge).
    BridgeMerge,
    /// Cutting 1-cells disconnecting components (Δβ₀ = +k, separation cut).
    SplitCut,
}

/// An exact topological surgery on a cell complex.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Surgery {
    pub kind: SurgeryKind,
    pub delta_betti: (i64, i64),
    pub support_delta: Vec<(i64, i64)>,
}

impl Surgery {
    /// Construct a hole-filling surgery with the given cavity cells.
    pub fn fill_hole(cavity_cells: Vec<(i64, i64)>) -> Self {
        let mut delta = cavity_cells;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::FillHole,
            delta_betti: (0, -1),
            support_delta: delta,
        }
    }

    /// Construct a puncture surgery with the given removed cells.
    pub fn puncture(hole_cells: Vec<(i64, i64)>) -> Self {
        let mut delta = hole_cells;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::Puncture,
            delta_betti: (0, 1),
            support_delta: delta,
        }
    }

    /// Construct a bridge-merge surgery identifying disjoint components.
    pub fn bridge_merge(bridge_cells: Vec<(i64, i64)>) -> Self {
        let mut delta = bridge_cells;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::BridgeMerge,
            delta_betti: (-1, 0),
            support_delta: delta,
        }
    }

    /// Construct a split-cut surgery dividing connected components.
    pub fn split_cut(cut_cells: Vec<(i64, i64)>) -> Self {
        let mut delta = cut_cells;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::SplitCut,
            delta_betti: (1, 0),
            support_delta: delta,
        }
    }

    /// Apply the surgery to a support, producing the modified cell set.
    pub fn apply(&self, cells: &[(i64, i64)]) -> Vec<(i64, i64)> {
        match self.kind {
            SurgeryKind::FillHole | SurgeryKind::BridgeMerge => {
                let mut out = cells.to_vec();
                out.extend_from_slice(&self.support_delta);
                out.sort_unstable();
                out.dedup();
                out
            }
            SurgeryKind::Puncture | SurgeryKind::SplitCut => {
                let mut out = Vec::with_capacity(cells.len());
                for c in cells {
                    if self.support_delta.binary_search(c).is_err() {
                        out.push(*c);
                    }
                }
                out
            }
        }
    }

    /// Monoid composition: S₁ ∘ S₂ sums Betti deltas and composes cell operations.
    pub fn compose(&self, other: &Self) -> Self {
        let mut delta = self.support_delta.clone();
        delta.extend_from_slice(&other.support_delta);
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: self.kind,
            delta_betti: (
                self.delta_betti.0 + other.delta_betti.0,
                self.delta_betti.1 + other.delta_betti.1,
            ),
            support_delta: delta,
        }
    }

    /// Detect surgery between two supports by exact Betti number analysis.
    pub fn detect(src: &[(i64, i64)], dst: &[(i64, i64)]) -> Option<Self> {
        let t_src = topology(src);
        let t_dst = topology(dst);
        let d_b0 = t_dst.0 as i64 - t_src.0 as i64;
        let d_b1 = t_dst.1 as i64 - t_src.1 as i64;

        if d_b0 == 0 && d_b1 == 0 {
            return None;
        }

        if d_b1 < 0 {
            // Hole filled: cells in dst not in src
            let mut diff = Vec::new();
            for c in dst {
                if src.binary_search(c).is_err() {
                    diff.push(*c);
                }
            }
            if !diff.is_empty() {
                return Some(Surgery {
                    kind: SurgeryKind::FillHole,
                    delta_betti: (d_b0, d_b1),
                    support_delta: diff,
                });
            }
        } else if d_b1 > 0 {
            // Puncture: cells in src not in dst
            let mut diff = Vec::new();
            for c in src {
                if dst.binary_search(c).is_err() {
                    diff.push(*c);
                }
            }
            if !diff.is_empty() {
                return Some(Surgery {
                    kind: SurgeryKind::Puncture,
                    delta_betti: (d_b0, d_b1),
                    support_delta: diff,
                });
            }
        } else if d_b0 < 0 {
            // Merge: cells in dst not in src connecting components
            let mut diff = Vec::new();
            for c in dst {
                if src.binary_search(c).is_err() {
                    diff.push(*c);
                }
            }
            return Some(Surgery {
                kind: SurgeryKind::BridgeMerge,
                delta_betti: (d_b0, d_b1),
                support_delta: diff,
            });
        } else if d_b0 > 0 {
            // Split: cells removed to disconnect
            let mut diff = Vec::new();
            for c in src {
                if dst.binary_search(c).is_err() {
                    diff.push(*c);
                }
            }
            return Some(Surgery {
                kind: SurgeryKind::SplitCut,
                delta_betti: (d_b0, d_b1),
                support_delta: diff,
            });
        }

        None
    }

    /// Factor a non-homeomorphic transformation into a composite Φ = V ∘ S.
    /// Returns the surgery S and conformal alignment versor V.
    pub fn factor(src: &Shape, dst: &Shape) -> Option<(Self, Mv)> {
        let surg = Self::detect(&src.blade.cells, &dst.blade.cells)?;
        let core_cells = surg.apply(&src.blade.cells);
        let core_shape = shape_of(&core_cells);
        let v = crate::transducer::blade::carry::versor_between(&core_shape, dst)?;
        Some((surg, v))
    }
}
