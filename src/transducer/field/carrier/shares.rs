// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-ANASTOME §9.4 (ruled 2026-11-06; issue #149), LAW-COUPLING v1 F2
//! (proposal 78.175), LAW-PASSAGE v3 (78.172) — THE COHESION OF A HELD
//! WHOLE, PER PLANE: on each plane that holds a frame state `H·n_Z + R·n_Q`
//! the held share `c_k = H/(H + R) = 1 − ν(t) = ν(1/t)`, `t = R/H`, the
//! viscosity; its complement `ν(t)`, the open share, is the fluidity. Both
//! are `½` at the throat, and the passage's pole swap exchanges them.
//!
//! A READING, NEVER STORED AND NEVER A SCALAR. It is read from the frames
//! the held whole stands on, every call (law 12), as a keyed vector over
//! its planes (`forms::coordinate::Vector`, keyed by the plane's two axes
//! as the coordinate form writes them). Its components are never summed
//! into one another nor averaged: the reading offers no total.
//!
//! ```compile_fail,E0599
//! // No total: the cohesion is per plane, never one number.
//! fn total(c: &mycelium::transducer::field::carrier::Cohesion) -> mycelium::core::Q {
//!     c.sum()
//! }
//! ```

use super::refusal::FieldRefusal;
use crate::core::Q;
use forms::coordinate::{Axis, FormRefusal, Key, Plane, Square, Vector};
use forms::passage::Frame;
use num_traits::One;

/// THE COHESION OF A HELD WHOLE (the viscosity; MU-ANASTOME §9.4): one
/// held share per plane that holds a frame state, keyed by the plane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cohesion {
    planes: Vector,
}

impl Cohesion {
    /// THE COHESION OF `frames`, each a frame state keyed by the plane it
    /// stands on: per plane `H/(H + R)` (`Frame::share`). A plane is named
    /// by its two axes, a turning plane (`e_+k+l`) or one frame plane's own
    /// pair (`e_+k-k`), where the frame's poles stand. Refused
    /// `NotTurningPlane` with the coordinate form's refusal for a key of
    /// one axis or of two axes that hold no frame, or a plane named twice;
    /// and `Passage` where a frame holds no share.
    pub fn of(frames: &[(Key, Frame)]) -> Result<Cohesion, FieldRefusal> {
        let mut components = Vec::with_capacity(frames.len());
        for (key, frame) in frames {
            match key.plane() {
                Some(Plane::Turning(_) | Plane::Frame) => {}
                Some(plane) => return Err(FieldRefusal::NotTurningPlane(FormRefusal::NotTurningPlane { key: key.clone(), plane })),
                None => return Err(FieldRefusal::NotTurningPlane(FormRefusal::NotGenerator { key: key.clone() })),
            }
            components.push((key.clone(), frame.share().map_err(FieldRefusal::Passage)?));
        }
        Vector::of(components).map(|planes| Cohesion { planes }).map_err(FieldRefusal::NotTurningPlane)
    }

    /// The held share on each plane, in the keys' canonical order.
    pub fn planes(&self) -> &[(Key, Q)] {
        self.planes.components()
    }

    /// The held share on `plane`, where the whole holds a frame there.
    pub fn on(&self, plane: &Key) -> Option<&Q> {
        self.planes.get(plane)
    }

    /// THE FLUIDITY, PER PLANE: each plane's open share `1 − c_k = ν(t)`,
    /// keyed as the cohesion is and never summed.
    pub fn fluidity(&self) -> Vec<(Key, Q)> {
        self.planes().iter().map(|(k, c)| (k.clone(), Q::one() - c)).collect()
    }

    /// THE THROAT TRAVERSAL STATE on plane `k` (78.176):
    /// - `Balanced` where `c_k == 1/2` (the throat boundary);
    /// - `TooCohesive` where `c_k > 1/2` (sticks at the throat);
    /// - `TooFluid` where `c_k < 1/2` (tears at the throat).
    pub fn traversal_state_on(&self, plane: &Key) -> Option<TraversalState> {
        let c = self.on(plane)?;
        let half = Q::new(num_bigint::BigInt::from(1), num_bigint::BigInt::from(2));
        match () {
            _ if *c == half => Some(TraversalState::Balanced),
            _ if *c > half => Some(TraversalState::TooCohesive),
            _ => Some(TraversalState::TooFluid),
        }
    }

    /// FIRST-DEGREE PLANAR ORBITAL SHEAR `Δθ_k = 360° · c_k = 360° / (1 + t_k)`.
    /// On a turning plane between attractors, reach is first-degree (t = r/M)
    /// and the turn count is 1 + t (never (1 + t)^2). Eliminates the 1/r^2
    /// ultraviolet singularity (bounded at 360° as r -> 0) and infrared
    /// dark matter anomaly (first-degree shear preserves orbital hold).
    pub fn planar_shear_on(&self, plane: &Key) -> Option<Q> {
        let c = self.on(plane)?;
        Some(super::turning::whole_turn() * c)
    }
}

/// THROAT TRAVERSAL STATE on a turning plane (LAW-PASSAGE, 78.172; 78.176):
/// a whole too cohesive sticks at the throat, one too fluid tears; balanced
/// traversal occurs at the throat where viscosity and fluidity agree (c = d = 1/2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalState {
    /// Both shares agree at 1/2: smooth traversal under the passage involution t ↦ 1/t.
    Balanced,
    /// Held share c_k > 1/2: viscosity dominates, the whole sticks at the throat.
    TooCohesive,
    /// Open share d_k > 1/2 (c_k < 1/2): fluidity dominates, the whole tears at the throat.
    TooFluid,
}

/// FRAME PLANE `k`'S KEY, its own two units `e₊ₖe₋ₖ` (`e_+k-k`), where the
/// carrier's frame states stand. Refused `NotTurningPlane` with the
/// coordinate form's refusal where `k` is 0.
pub fn frame_plane(k: u32) -> Result<Key, FieldRefusal> {
    let unit = |square| Axis::unit(square, k).map_err(FieldRefusal::NotTurningPlane);
    Key::generator(unit(Square::Positive)?, unit(Square::Negative)?).map_err(FieldRefusal::NotTurningPlane)
}
