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

use super::carrier::{frame_plane, Cohesion, FieldRefusal};
use super::{Contemplation, Well};

impl Well {
    /// THE WELL'S COHESION, PER PLANE (the viscosity): its inflation's
    /// gauge ([`Well::inflation`]: `H` the charts its cover closed, `R` its
    /// sections' charts still open) read as the held share `H/(H + R)` on
    /// the plane it stands on, its membrane's frame plane. At the throat
    /// both shares are `½`; at the closed pole (nothing open) the cohesion
    /// is 1. Refused as the inflation and [`Cohesion::of`] refuse.
    pub fn cohesion(&self) -> Result<Cohesion, FieldRefusal> {
        let plane = frame_plane(self.body()?.membrane().plane())?;
        Cohesion::of(&[(plane, self.inflation()?.gauge)])
    }
}

impl Contemplation {
    /// THE HELD FIELD'S COHESION, PER PLANE (MU-CLIENT §4.9, §7.1): one held
    /// share `c_k = H/(H + R)` on each distinct frame plane its held wells
    /// stand on, read from each well's inflation on its membrane's frame plane.
    /// `None` when no well is held. Never stored and never summed into one
    /// number.
    pub fn cohesion(&self) -> Result<Option<Cohesion>, FieldRefusal> {
        let mut frames = Vec::new();
        for (_, well) in self.wells.scoped() {
            let body = match well.body() {
                Ok(b) => b,
                Err(_) => continue,
            };
            let plane = frame_plane(body.membrane().plane())?;
            let gauge = well.inflation()?.gauge;
            if frames.iter().any(|(p, _)| p == &plane) {
                continue;
            }
            frames.push((plane, gauge));
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
