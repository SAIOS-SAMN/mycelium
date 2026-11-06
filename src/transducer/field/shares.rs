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
use super::Well;

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
