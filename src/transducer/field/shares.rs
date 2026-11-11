// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANASTOME §9.4 (ruled 2026-11-06; issue #149) — A HELD WHOLE'S
//! COHESION, READ: a well, a held percept or a held thought (one well at
//! two stages, 78.171 §2), read per plane from the frames its body stands
//! on (`carrier::Cohesion`). A reading of what it holds on every call (law
//! 12), never a field of the well and never one number.
//!
//! ```compile_fail,E0615
//! // The cohesion is read, never held: no well stores it.
//! let well = mycelium::transducer::field::Well::default();
//! let _ = well.cohesion;
//! ```

use super::carrier::{pole_plane, whole_turn, Cohesion, FieldRefusal};
use super::{Contemplation, Well};
use crate::core::Q;
use num_traits::{One, Signed, Zero};

impl Well {
    /// THE WELL'S COHESION, PER PLANE (the viscosity; #149, #249): its inflation's
    /// gauge ([`Well::inflation`]: `H` the charts its cover closed, `R` its
    /// sections' charts still open) read as the held share `H/(H + R) = 1/(1 + t)`
    /// on its membrane's pole plane and every turning plane of its bound orbit
    /// and local orbits. On each plane, the first-degree orbital shear is
    /// `Δθ_k = 360° · c_k = 360° / (1 + t_k)`. At the throat both shares are `½`;
    /// at the closed pole (nothing open) the cohesion is 1. Refused as the
    /// inflation and [`Cohesion::of`] refuse.
    pub fn cohesion(&self) -> Result<Cohesion, FieldRefusal> {
        let inflation = self.inflation()?;
        let body = self.body()?;
        let mut frames = vec![(pole_plane(body.membrane().plane())?, inflation.gauge)];
        for (_, local_orbit) in &self.orbits {
            if let Ok(rot) = local_orbit.holonomy() {
                if let Ok(planes) = rot.planes() {
                    for (k, frac) in planes {
                        let c_k = frac.abs().fract();
                        let c_k = if c_k.is_zero() && !frac.is_zero() { Q::one() } else { c_k };
                        let h_k = c_k.clone();
                        let r_k = Q::one() - &c_k;
                        let f = forms::passage::PassageState { hold: h_k, reach: r_k };
                        if !frames.iter().any(|(existing, _)| existing == &k) {
                            frames.push((k, f));
                        }
                    }
                }
            }
        }
        if let Ok(holo) = body.holonomy() {
            for t in holo.turnings() {
                let frac = (t.degrees() / whole_turn()).abs();
                let c_k = frac.fract();
                let c_k = if c_k.is_zero() && !t.degrees().is_zero() { Q::one() } else { c_k };
                let h_k = c_k.clone();
                let r_k = Q::one() - &c_k;
                let key = t.plane().key().clone();
                let f = forms::passage::PassageState { hold: h_k, reach: r_k };
                if !frames.iter().any(|(existing, _)| existing == &key) {
                    frames.push((key, f));
                }
            }
        }
        Cohesion::of(&frames)
    }
}

impl Contemplation {
    /// THE HELD FIELD'S COHESION, PER PLANE (MU-CLIENT §4.9, §7.1; #149, #249):
    /// one held share `c_k = H/(H + R)` on each distinct frame plane its held wells
    /// stand on, read across each well's membrane frame plane and turning planes.
    /// `None` when no well is held. Never stored and never summed into one number.
    pub fn cohesion(&self) -> Result<Option<Cohesion>, FieldRefusal> {
        let mut frames = Vec::new();
        for (_, well) in self.wells.scoped() {
            let Ok(c) = well.cohesion() else { continue };
            for (p, _) in c.planes() {
                if !frames.iter().any(|(existing, _)| existing == p) {
                    if let Some(c_k) = c.on(p) {
                        let h_k = c_k.clone();
                        let r_k = Q::one() - c_k;
                        let f = forms::passage::PassageState { hold: h_k, reach: r_k };
                        frames.push((p.clone(), f));
                    }
                }
            }
        }
        if frames.is_empty() {
            Ok(None)
        } else {
            Cohesion::of(&frames).map(Some)
        }
    }

    /// The cohesion of the specific thought at `address` ([`Well::cohesion`]),
    /// or `None` where no thought stands at `address`.
    pub fn cohesion_of(&self, address: &[u8]) -> Option<Result<Cohesion, FieldRefusal>> {
        self.wells.get(address).map(Well::cohesion)
    }
}
