// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! NETWORK — the validated wire substrate, below the sovereignty boundary.
//! Framing/transport may hold maps as cold indices; the germ-admission decision
//! (consensus) and routing state stay Vec-of-relations, never digest-keyed
//! cognition. Three seams: `wire` (binary frames, MTU budget, 4-lane stratum
//! multiplex), `consensus` (QUORUM=2 two-voice germ admission), `nerve` (Čech
//! peer table + per-lane defect flux + invariant fingerprints).

pub mod wire;
pub mod consensus;
pub mod nerve;
