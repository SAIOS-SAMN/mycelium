// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COMPUTATIONAL PHYSICS — polytonic kinematic pressure + torsion, per lane,
//! never a monotone scalar (the #34 law). Pressure is the per-lane backlog of
//! unresolved demand (𝒮 spatial, 𝒢 gauge, 𝒯 temporal, 𝒽 shell); the metabolism
//! rate is derived per-lane from it, so no lane is ever traded against another.
//! Torsion is the obstruction reading: a demand that will not close is a PIVOT —
//! a torsion charge on the lane it resisted, the demand for a new fiber.

// The four lanes a germ carries content on — 𝒮 (spatial), 𝒢 (gauge), residual (a
// germinated coboundary axis), 𝒽 (shell) — are read off the germ's exact per-lane
// support by `Element::lanes()` (transducer/element.rs), not derived here.

/// Per-lane kinematic pressure, torsion, and the SYNTROPIC/ENTROPIC metabolism.
/// Arrays, never a summed scalar (#34). The metabolism is the governing physics
/// of the hypha↔client exchange: syntropic events (settlements — order built,
/// negentropy) and entropic events (pivots/backlog — disorder) net per lane into
/// the order flux that paces serving and demand cadence.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Physics {
    /// Open demand backlog per lane — the pressure.
    pub pressure: [u64; 4],
    /// SYNTROPIC events per lane — settlements that built structure (negentropy).
    pub syntropic: [u64; 4],
    /// ENTROPIC events per lane — pivots/torsion, disorder the demand injected.
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

    /// The metabolism RELAXES — the entropic backlog DISSIPATES as the mesh works,
    /// so per-lane pressure is a RATE (unresolved demand per unit of work), not a
    /// one-way ratchet that saturates and stalls the governor. Called once per
    /// adjudication; steady-state pressure then equilibrates where the germination
    /// rate meets this relaxation — a bounded, self-limiting metabolism.
    pub fn relax(&mut self) {
        for l in 0..4 {
            self.pressure[l] = self.pressure[l].saturating_sub(1);
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

    /// A settlement built order on these lanes — a SYNTROPIC event (negentropy).
    pub fn syntropy(&mut self, lanes: [bool; 4]) {
        for l in 0..4 {
            if lanes[l] {
                self.syntropic[l] = self.syntropic[l].saturating_add(1);
            }
        }
    }

    /// THE METABOLISM RATE — per-lane serving cadence, proportional to pressure.
    /// Polytonic: each lane's rate is its own pressure, never one ranking across
    /// lanes. This is what paces the hypha's serving and the client's demand.
    pub fn metabolism_rate(&self) -> [u64; 4] {
        self.pressure
    }

    /// THE ORDER FLUX — per-lane syntropic minus entropic. Positive: the mesh is
    /// building structure on that lane (settling faster than it pivots);
    /// negative: it is losing ground (a lane demanding a new fiber). The dynamic
    /// entropic/syntropic metabolism (#34), reported hypha→client each exchange.
    pub fn order_flux(&self) -> [i64; 4] {
        std::array::from_fn(|l| self.syntropic[l] as i64 - self.torsion[l] as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_is_per_lane_and_relieves() {
        let mut p = Physics::default();
        p.bump([true, false, false, false]); // a spatial demand
        p.bump([false, true, false, false]); // a gauge demand
        assert_eq!(p.metabolism_rate(), [1, 1, 0, 0], "per-lane, not summed");
        p.relieve([true, false, false, false]);
        assert_eq!(p.metabolism_rate(), [0, 1, 0, 0]);
    }

    /// A pure recolour charges only the gauge lane — now read off the germ's exact
    /// per-lane support (`Element::lanes()`), the coordinate-exact carrier.
    #[test]
    fn a_recolour_charges_the_gauge_lane() {
        use crate::transducer::element::from_transform;
        use crate::transducer::transform::solve_transform;
        let g = from_transform(&solve_transform(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        assert_eq!(g.lanes(), [false, true, false, false], "pure recolour = gauge only");
    }

    #[test]
    fn a_pivot_is_a_torsion_charge() {
        let mut p = Physics::default();
        p.pivot([false, false, false, false]); // unattributable
        assert_eq!(p.torsion, [1, 1, 1, 1], "an unresolved residual charges torsion");
    }

    /// The metabolism is per-lane syntropic minus entropic — the order flux.
    #[test]
    fn the_metabolism_is_order_flux_per_lane() {
        let mut p = Physics::default();
        p.syntropy([true, false, false, false]); // built order on 𝒮
        p.syntropy([true, false, false, false]);
        p.pivot([false, true, false, false]); // disorder on 𝒢
        assert_eq!(p.order_flux(), [2, -1, 0, 0], "syntropic − entropic, never summed");
    }
}
