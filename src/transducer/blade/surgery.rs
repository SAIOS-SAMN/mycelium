// 50.8, A6 — TOPOLOGICAL SURGERY MONOID: exact cube-complex cobordisms (Fill, Puncture, Merge, Split).
//
//! The conformal group SO(n+1, 1) consists exclusively of homeomorphisms
//! (preserving Betti numbers: Δβ₀ = 0, Δβ₁ = 0). A transformation that changes
//! genus or connectivity has no conformal versor.
//!
//! A surgery is an exact cube-complex cobordism acting on the Betti numbers
//! and the support. Surgeries compose as a graded monoid:
//! Δβ(S₁ ∘ S₂) = Δβ(S₁) + Δβ(S₂).
//!
//! Any non-homeomorphic transformation factors as Φ = V ∘ S, where S is the
//! topological surgery matching Betti numbers, and V is the conformal versor
//! aligning the resulting homeomorphic core. The support is the stack, on
//! the sites of the base (51.3, 71); a support whose topology is not read at
//! its `n` has no surgery detected on it (38.2, BQ4).

use super::body::{shape_of, topology, Shape};
use crate::transducer::base::Site;
use crate::transducer::cga::Mv;

/// The elementary topological surgery operations on the cube complex of a
/// support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SurgeryKind {
    /// Capping 1-cycles by adding the compartments of a hole (Δβ₁ = -k, fill).
    FillHole,
    /// Deleting interior compartments, opening 1-cycles (Δβ₁ = +k, puncture).
    Puncture,
    /// Adding compartments that join components (Δβ₀ = -k, bridge merge).
    BridgeMerge,
    /// Removing compartments that disconnect components (Δβ₀ = +k, separation cut).
    SplitCut,
}

/// An exact topological surgery on a cube complex.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Surgery {
    pub kind: SurgeryKind,
    pub delta_betti: (i64, i64),
    pub support_delta: Vec<Site>,
}

impl Surgery {
    /// Construct a hole-filling surgery from the sites of the hole.
    pub fn fill_hole(hole: Vec<Site>) -> Self {
        let mut delta = hole;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::FillHole,
            delta_betti: (0, -1),
            support_delta: delta,
        }
    }

    /// Construct a puncture surgery from the sites removed.
    pub fn puncture(removed: Vec<Site>) -> Self {
        let mut delta = removed;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::Puncture,
            delta_betti: (0, 1),
            support_delta: delta,
        }
    }

    /// Construct a bridge-merge surgery from the sites of the bridge.
    pub fn bridge_merge(bridge: Vec<Site>) -> Self {
        let mut delta = bridge;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::BridgeMerge,
            delta_betti: (-1, 0),
            support_delta: delta,
        }
    }

    /// Construct a split-cut surgery from the sites of the cut.
    pub fn split_cut(cut: Vec<Site>) -> Self {
        let mut delta = cut;
        delta.sort_unstable();
        delta.dedup();
        Surgery {
            kind: SurgeryKind::SplitCut,
            delta_betti: (1, 0),
            support_delta: delta,
        }
    }

    /// Apply the surgery to a support, producing the modified support.
    pub fn apply(&self, support: &[Site]) -> Vec<Site> {
        match self.kind {
            SurgeryKind::FillHole | SurgeryKind::BridgeMerge => {
                let mut out = support.to_vec();
                out.extend_from_slice(&self.support_delta);
                out.sort_unstable();
                out.dedup();
                out
            }
            SurgeryKind::Puncture | SurgeryKind::SplitCut => {
                let mut out = Vec::with_capacity(support.len());
                for c in support {
                    if self.support_delta.binary_search(c).is_err() {
                        out.push(c.clone());
                    }
                }
                out
            }
        }
    }

    /// Monoid composition: S₁ ∘ S₂ sums Betti deltas and unites the supports moved.
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
    pub fn detect(src: &[Site], dst: &[Site]) -> Option<Self> {
        let t_src = topology(src)?;
        let t_dst = topology(dst)?;
        // β₀ and β₁ at the support's own `n`; a base with no β₁ (n < 2)
        // has no hole to fill or open.
        let beta = |t: &[u64], k: usize| t.get(k).map_or(0, |b| *b as i64);
        let d_b0 = beta(&t_dst, 0) - beta(&t_src, 0);
        let d_b1 = beta(&t_dst, 1) - beta(&t_src, 1);

        if d_b0 == 0 && d_b1 == 0 {
            return None;
        }
        // The sites of one support the other does not hold.
        let missing = |from: &[Site], held: &[Site]| -> Vec<Site> {
            from.iter().filter(|c| held.binary_search(c).is_err()).cloned().collect()
        };

        if d_b1 < 0 {
            // Hole filled: sites in dst not in src
            let diff = missing(dst, src);
            if !diff.is_empty() {
                return Some(Surgery {
                    kind: SurgeryKind::FillHole,
                    delta_betti: (d_b0, d_b1),
                    support_delta: diff,
                });
            }
        } else if d_b1 > 0 {
            // Puncture: sites in src not in dst
            let diff = missing(src, dst);
            if !diff.is_empty() {
                return Some(Surgery {
                    kind: SurgeryKind::Puncture,
                    delta_betti: (d_b0, d_b1),
                    support_delta: diff,
                });
            }
        } else if d_b0 < 0 {
            // Merge: sites in dst not in src connecting components
            return Some(Surgery {
                kind: SurgeryKind::BridgeMerge,
                delta_betti: (d_b0, d_b1),
                support_delta: missing(dst, src),
            });
        } else if d_b0 > 0 {
            // Split: sites removed to disconnect
            return Some(Surgery {
                kind: SurgeryKind::SplitCut,
                delta_betti: (d_b0, d_b1),
                support_delta: missing(src, dst),
            });
        }

        None
    }

    /// Factor a non-homeomorphic transformation into a composite Φ = V ∘ S.
    /// Returns the surgery S and conformal alignment versor V.
    pub fn factor(src: &Shape, dst: &Shape, budget: &mut crate::physics::Budget) -> Option<(Self, Mv)> {
        let surg = Self::detect(&src.blade.support, &dst.blade.support)?;
        let core = surg.apply(&src.blade.support);
        let core_shape = shape_of(&core);
        let v = crate::transducer::blade::carry::versor_between(&core_shape, dst, budget).ok()?;
        Some((surg, v))
    }
}
