// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COMPUTATIONAL PHYSICS — polytonic kinematic pressure + torsion, per lane,
//! never a monotone scalar (the #34 law). Pressure is the per-lane backlog of
//! unresolved demand (𝒮 spatial, 𝒢 gauge, 𝒯 temporal, 𝒽 shell); the metabolism
//! rate is derived per-lane from it, so no lane is ever traded against another.
//! Torsion is the obstruction reading: a demand that will not close is a PIVOT —
//! a torsion charge on the lane it resisted, the demand for a new fiber.

use crate::transducer::transform::Transform;

/// The four lanes a germ carries content on: 𝒮 (spatial motor ≠ id), 𝒢 (gauge
/// relabel ≠ ∅), 𝒯/𝒽 reserved (temporal/shell — filled as those fibers land).
pub fn lanes_of(g: &Transform) -> [bool; 4] {
    [!g.motor.is_identity(), !g.relabel.is_empty(), false, false]
}

/// Per-lane kinematic pressure + torsion. Arrays, never a summed scalar.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Physics {
    /// Open demand backlog per lane — the pressure.
    pub pressure: [u64; 4],
    /// Unclosable-residual (pivot) charge per lane — the torsion spectrum.
    pub torsion: [u64; 4],
}

impl Physics {
    /// A demand entered on these lanes — pressure rises.
    pub fn bump(&mut self, lanes: [bool; 4]) {
        for l in 0..4 {
            if lanes[l] {
                self.pressure[l] = self.pressure[l].saturating_add(1);
            }
        }
    }

    /// A demand closed on these lanes — pressure relieves.
    pub fn relieve(&mut self, lanes: [bool; 4]) {
        for l in 0..4 {
            if lanes[l] {
                self.pressure[l] = self.pressure[l].saturating_sub(1);
            }
        }
    }

    /// A pivot on `lane` — a torsion charge (the demand for a new fiber). An
    /// unattributable pivot charges all lanes (its resisting axis is unknown).
    pub fn pivot(&mut self, lanes: [bool; 4]) {
        let any = lanes.iter().any(|&b| b);
        for l in 0..4 {
            if lanes[l] || !any {
                self.torsion[l] = self.torsion[l].saturating_add(1);
            }
        }
    }

    /// THE METABOLISM RATE — per-lane, proportional to pressure. Polytonic: each
    /// lane's cadence is its own pressure, never one ranking across lanes.
    pub fn metabolism_rate(&self) -> [u64; 4] {
        self.pressure
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::transform::solve_transform;

    #[test]
    fn pressure_is_per_lane_and_relieves() {
        let mut p = Physics::default();
        p.bump([true, false, false, false]); // a spatial demand
        p.bump([false, true, false, false]); // a gauge demand
        assert_eq!(p.metabolism_rate(), [1, 1, 0, 0], "per-lane, not summed");
        p.relieve([true, false, false, false]);
        assert_eq!(p.metabolism_rate(), [0, 1, 0, 0]);
    }

    #[test]
    fn a_recolour_charges_the_gauge_lane() {
        let a = [(0, 0, 1u8), (0, 1, 1)];
        let b = [(0, 0, 2u8), (0, 1, 2)];
        let g = solve_transform(&a, &b).unwrap();
        assert_eq!(lanes_of(&g), [false, true, false, false], "pure recolour = gauge only");
    }

    #[test]
    fn a_pivot_is_a_torsion_charge() {
        let mut p = Physics::default();
        p.pivot([false, false, false, false]); // unattributable
        assert_eq!(p.torsion, [1, 1, 1, 1], "an unresolved residual charges torsion");
    }
}
