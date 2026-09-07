// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! NETWORK — the validated wire substrate, below the sovereignty boundary.
//! Framing/transport may hold maps as cold indices; routing state stays
//! Vec-of-relations, never digest-keyed cognition. Three seams: `wire`
//! (binary frames, MTU budget, 4-lane stratum multiplex), `nerve` (Čech
//! peer table + per-lane defect flux + invariant fingerprints), `mesh`.
//! Germ admission is NOT here and is not a count: it is convergence, and
//! it lives in `court::convergence` where the re-derivations are recorded.

pub mod wire;
pub mod nerve;
pub mod mesh;
