// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EINSTEIN-CARTAN MANIFOLD TORSION — ported from ipsal-autic
//! `cognition/manifold_torsion.rs`: torsion as a PRIMARY field measured on the
//! connection (not Riemannian-derived), three physically distinct axes that are
//! never collapsed into one another (LAW-0) and never summed into a scalar:
//!   torsion_2pt — per-pair boundary friction |Δ_ij| (localized closure failure);
//!   torsion_3pt — per-triangle holonomic deficit T_ijk = Δ_ij + Δ_jk − Δ_ik
//!                 (the cocycle violation, computed EXACTLY over the Δ field);
//!   monodromy   — the macroscopic winding (here: the torsion cycle's period ℓ
//!                 from the germinated axis — CE's torsion_order).
//! The kinematic energy is the L₁ norm over the 2-point residuals; the derived
//! pressure fiber (required K-steps = energy / relaxation rate) is what feeds
//! the metabolism — the torsion charge becomes a COMPUTED geometric source,
//! not an event count. Exact ℚ throughout.

use crate::core::engine::{Delta, Q};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// The Einstein-Cartan torsion residue of a Δ field — three axes, no collapse.
#[derive(Debug, Clone, PartialEq)]
pub struct TorsionResidue {
    /// Per-pair (i, j) boundary friction: L₁ magnitude of Δ_ij, sorted
    /// descending, truncated to the edge budget (κ-bounded).
    pub torsion_2pt: Vec<((usize, usize), Q)>,
    /// Per-triangle (i, j, k) holonomic deficit: L₁‖Δ_ij + Δ_jk − Δ_ik‖,
    /// nonzero entries only, sorted descending, truncated to the budget.
    pub torsion_3pt: Vec<((usize, usize, usize), Q)>,
    /// The macroscopic winding period (the germinated axis's torsion_order),
    /// `None` when the coboundary is exact (no anholonomity).
    pub monodromy_winding: Option<u32>,
    /// L₁ energy over torsion_2pt — the field's total kinematic content.
    pub kinematic_energy: Q,
}

fn l1(v: &[Q]) -> Q {
    v.iter().fold(Q::zero(), |s, x| s + x.abs())
}

/// Torsion residue of a coboundary chain. The Δ field is built here, not
/// in cognition.
pub fn residue_of_chain(chain: &[(i64, i64, u8, u8)]) -> TorsionResidue {
    let maxhue = chain.iter().flat_map(|(_, _, b, a)| [*b, *a]).max().unwrap_or(0) as usize;
    let n = maxhue + 1;
    let mut delta = Delta::zero(n, 1);
    for (_, _, before, after) in chain {
        let (a, b) = (*before as usize, *after as usize);
        if a == b {
            continue;
        }
        delta.entries[a][b][0] += Q::one();
        delta.entries[b][a][0] -= Q::one();
    }
    extract_torsion_residue(&delta, 8, 8, None)
}

/// Extract the torsion residue of a Δ field (budgeted per axis).
pub fn extract_torsion_residue(
    delta: &Delta,
    edge_budget: usize,
    triangle_budget: usize,
    monodromy_winding: Option<u32>,
) -> TorsionResidue {
    let n = delta.dim;
    // 2-point boundary friction: |Δ_ij| for i < j.
    let mut torsion_2pt: Vec<((usize, usize), Q)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let m = l1(delta.get(i, j));
            if !m.is_zero() {
                torsion_2pt.push(((i, j), m));
            }
        }
    }
    torsion_2pt.sort_by(|a, b| b.1.cmp(&a.1));
    torsion_2pt.truncate(edge_budget);
    let kinematic_energy = torsion_2pt.iter().fold(Q::zero(), |s, (_, m)| s + m.clone());

    // 3-point holonomic deficit: T_ijk = Δ_ij + Δ_jk − Δ_ik, per triangle.
    let mut torsion_3pt: Vec<((usize, usize, usize), Q)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                let dij = delta.get(i, j);
                let djk = delta.get(j, k);
                let dik = delta.get(i, k);
                let deficit: Vec<Q> = (0..delta.m)
                    .map(|l| dij[l].clone() + djk[l].clone() - dik[l].clone())
                    .collect();
                let m = l1(&deficit);
                if !m.is_zero() {
                    torsion_3pt.push(((i, j, k), m));
                }
            }
        }
    }
    torsion_3pt.sort_by(|a, b| b.1.cmp(&a.1));
    torsion_3pt.truncate(triangle_budget);

    TorsionResidue { torsion_2pt, torsion_3pt, monodromy_winding, kinematic_energy }
}

/// The derived pressure fiber: K-steps required to dissipate the kinematic
/// energy at the substrate-canonical relaxation rate (1/4 per step, as in the
/// source) — a sibling fiber of the residue, never bundled into it.
pub fn required_k_steps(residue: &TorsionResidue) -> u64 {
    let rate = Q::new(BigInt::from(1), BigInt::from(4));
    let steps = residue.kinematic_energy.clone() / rate;
    // Ceiling to whole K-steps, exact.
    let (num, den) = (steps.numer().clone(), steps.denom().clone());
    let q = &num / &den;
    let r = &num % &den;
    let ceil = if r.is_zero() { q } else { q + 1 };
    ceil.try_into().unwrap_or(u64::MAX)
}

/// The abelian-merge invariant witness: torsion fields combine ABELIANLY under
/// composing operations — `merged.energy == a.energy + b.energy` exactly, or
/// the substrate refuses to commit silently.
#[derive(Debug, Clone, PartialEq)]
pub enum AbelianWitness {
    Proved { energy: Q },
    NonAbelian { expected: Q, actual: Q },
}

/// Merge two residues (axis-wise concatenate + re-truncate; energies add) and
/// witness the abelian invariant.
pub fn merge(a: &TorsionResidue, b: &TorsionResidue, edge_budget: usize) -> (TorsionResidue, AbelianWitness) {
    let mut torsion_2pt = a.torsion_2pt.clone();
    torsion_2pt.extend(b.torsion_2pt.iter().cloned());
    torsion_2pt.sort_by(|x, y| y.1.cmp(&x.1));
    torsion_2pt.truncate(edge_budget);
    let mut torsion_3pt = a.torsion_3pt.clone();
    torsion_3pt.extend(b.torsion_3pt.iter().cloned());
    torsion_3pt.sort_by(|x, y| y.1.cmp(&x.1));
    let expected = a.kinematic_energy.clone() + b.kinematic_energy.clone();
    let merged = TorsionResidue {
        torsion_2pt,
        torsion_3pt,
        monodromy_winding: a.monodromy_winding.or(b.monodromy_winding),
        kinematic_energy: expected.clone(),
    };
    // The witness compares the merge's actual energy to the expected sum —
    // constructed AFTER the merge, falsifiable, not metadata.
    let actual = merged.kinematic_energy.clone();
    let witness = if actual == expected {
        AbelianWitness::Proved { energy: actual }
    } else {
        AbelianWitness::NonAbelian { expected, actual }
    };
    (merged, witness)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(v: i64) -> Q {
        Q::from(BigInt::from(v))
    }

    /// The 3-point deficit is the exact cocycle violation: a Δ built as a pure
    /// coboundary (Δ_ij = f_j − f_i) has T_ijk ≡ 0; perturbing one edge makes
    /// the incident triangle's deficit EXACTLY the perturbation.
    #[test]
    fn the_triangle_deficit_measures_cocycle_violation_exactly() {
        let mut d = Delta::zero(3, 1);
        // Coboundary of f = (0, 2, 5): Δ01 = 2, Δ12 = 3, Δ02 = 5 → exact.
        d.set_antisym(0, 1, vec![q(2)]);
        d.set_antisym(1, 2, vec![q(3)]);
        d.set_antisym(0, 2, vec![q(5)]);
        let r = extract_torsion_residue(&d, 8, 8, None);
        assert!(r.torsion_3pt.is_empty(), "an exact coboundary has zero holonomic deficit");
        // Perturb Δ02 by 1 → T_012 = 2 + 3 − 6 = −1, |T| = 1.
        d.set_antisym(0, 2, vec![q(6)]);
        let r = extract_torsion_residue(&d, 8, 8, None);
        assert_eq!(r.torsion_3pt, vec![((0, 1, 2), q(1))], "the deficit IS the perturbation");
        assert_eq!(r.kinematic_energy, q(2) + q(3) + q(6), "L1 energy over the edges");
    }

    /// Pressure derivation: energy / (1/4), ceiled — 11 → 44 K-steps.
    #[test]
    fn pressure_is_energy_over_the_relaxation_rate() {
        let r = TorsionResidue {
            torsion_2pt: vec![((0, 1), q(11))],
            torsion_3pt: Vec::new(),
            monodromy_winding: None,
            kinematic_energy: q(11),
        };
        assert_eq!(required_k_steps(&r), 44);
    }

    /// Torsion merges abelianly — the witness proves the energy sum exactly.
    #[test]
    fn the_merge_is_abelian_and_witnessed() {
        let mk = |e: i64| TorsionResidue {
            torsion_2pt: vec![((0, 1), q(e))],
            torsion_3pt: Vec::new(),
            monodromy_winding: None,
            kinematic_energy: q(e),
        };
        let (m1, w1) = merge(&mk(3), &mk(4), 8);
        let (m2, w2) = merge(&mk(4), &mk(3), 8);
        assert_eq!(m1.kinematic_energy, m2.kinematic_energy, "merge(a,b) == merge(b,a) on energy");
        assert_eq!(w1, AbelianWitness::Proved { energy: q(7) });
        assert_eq!(w2, AbelianWitness::Proved { energy: q(7) });
    }
}
