// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-DERIVATIONS D3.2 — THE TRANSDUCTION: two frames related body to body,
//! in turns, never through a common lattice, at any rank `n`
//! (MU-SENSING §3.1, §3.6; ROADMAP track 26, S4).
//!
//! LIFT → ACT → PROJECT. Past the sensor a sample of rank `n` is `n` phases,
//! one circle of turns `ℚ/ℤ` per axis with the axis's extent as its period:
//! a site boundary is the turn `i/p`, held as the pair (index, period) and
//! compared with another by integer cross-multiplication, never converted
//! to a point, never reduced to a common denominator. The seam `0 ≡ 1` of
//! each phase is held as the frame's boundary and never wrapped. No phase
//! is privileged: the canonical axis order writes addresses and orders the
//! reading, and is never a role. A body is held by its BOUNDARY: its bands,
//! the `n`-dimensional boxes of one symbol whose boundary turns stand on
//! every phase, never its interior sites. Two frames relate through the
//! incidence of the two frames' bands on the one turn circle of each phase: a
//! re-period of each phase (no versor carries an anisotropic scale), an
//! offset, and one of the `2ⁿ·n!` signed permutations of the phases.
//! Sites are written only at the two raster ends: the sensor's read, and
//! the projection that speaks onto an output frame.

mod enclosed;
mod factor;
mod ground;
mod manifold;
mod membrane;
mod motion;
mod nest;
mod nesting;
mod percept;
mod record;
mod refusal;
mod relation;
mod turn;

#[cfg(test)]
mod tests;

pub use enclosed::{Band, Enclosed};
pub use factor::{factorisations, factorisations_need, need_of_motion, Factored, Factorisation, Motion};
pub use ground::{Candidacy, Ground, Unground};
pub use manifold::{AxisManifold, PhaseManifold, SampleManifold};
pub use motion::{Generated, Sense, SignedPermutation};
pub use nest::{Factoring, Factorings, Joint};
pub use nesting::{nesting_of, AxisNesting, BlockRelation, Candidate, Held, LevelMotion, Member, NestRefusal, PairNesting, Reading, Taken, Unkept};
pub use percept::{PairFactored, PerceptRefusal, TaskPercept};
pub use record::{arcs, family_share, need, need_of, share_of_enclosure, Incidence, SiteChange, Transduction};
pub use refusal::TransductionRefusal;
pub use relation::{relation_of, PairRelation};
pub use turn::{Enclosure, Turn};
