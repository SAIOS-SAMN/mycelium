// 78.71 — THE FLUID AXIOMS: the lanes read as a field on the conformal
// sphere, never as lengths on a lattice. Each clause carries a test in
// `tests/fluid.rs`.

/// F1 · A NULL GENERATOR HAS NO LENGTH. A translation's generator
/// `d ∧ e_∞` squares to zero; on the conformal sphere it is a rotation about
/// the point at infinity. Conjugation can take it to any other on its ray,
/// so it has no invariant size: only its ray and its ratios to the
/// generators on that ray.
pub const F1_NULL_HAS_NO_LENGTH: &str =
    "a null generator has no conjugation-invariant length: only its ray and its ratios on that ray";

/// F2 · FLUX BY STOKES. 𝒯 is the null part of a closed cell's curvature —
/// the parabolic modes of its holonomy `F(σ) = T(∂σ)` (R4) — and nothing a
/// single transport's endpoint says.
pub const F2_FLUX_BY_STOKES: &str = "𝒯 is the parabolic part of a closed cell's curvature, read by its modes";

/// F3 · FROZEN IN. The flow carries a flux tube whole: conjugating every
/// flux of a tube by one motion keeps them on one ray and keeps the fluxes'
/// ratios, so a tube's windings are invariants of the flow (Alfvén, exact).
pub const F3_FROZEN_IN: &str = "conjugation carries a tube whole: one ray, the same ratios, the same windings";

/// F4 · THE QUANTUM IS READ. A tube's quantum is the generator of the
/// ℤ-module its fluxes' ratios span — `gcd` of the numerators over `lcm` of
/// the denominators — so every flux is an integer winding of it; a finer
/// flux refines it, never refused, and nothing declares it.
pub const F4_QUANTUM_READ: &str = "a tube's quantum is the generator of its fluxes' ℤ-module; every flux winds it a whole number of times";

/// F5 · NO LATTICE ON THE SPHERE. A motion's lanes are read by its own
/// modes — a turn on 𝒽, a boost or dilation on 𝒮, a null rotation on 𝒯 —
/// never by where it carries the origin: an endpoint's straight chord reads a
/// turn about any other centre as a translation (§76.1, 78.71).
pub const F5_NO_LATTICE_ON_THE_SPHERE: &str =
    "a motion's lanes are its modes, never the chord its image of the origin draws";
