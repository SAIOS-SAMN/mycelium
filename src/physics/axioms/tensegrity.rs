// A.10.1 — ORBITAL TENSEGRITY (ruled 2026-09-28, the operator's word;
// MU-RECURSION 78.51): coherence is the stationary equilibrium of a
// constellation of at least two co-present congruent frames. The physics is
// `physics::tensegrity`; each clause carries a test in `tests/tensegrity.rs`.

/// A.10.1 · THE AXIOM. Coherence is an equilibrium condition of a multi-frame
/// constellation, never an intrinsic property of an isolated frame: an orbit
/// is coherent iff it holds at least two co-present congruent frames whose
/// mutual inversive tethers balance the circulation generator's strain under
/// exact Clifford commutation over ℚ.
pub const A10_1_ORBITAL_TENSEGRITY: &str =
    "coherence is the stationary equilibrium of at least two co-present congruent frames whose tethers balance the generator";

/// LAW I · MINIMUM CO-PRESENCE. Fewer than two frames carry no coupling and
/// cannot evaluate coherence: refused as `MonocularOrbit`.
pub const A10_1_I_CO_PRESENCE: &str = "an orbit of fewer than two co-present frames is refused as monocular";

/// THE TORQUE (78.51(1)). A coupling's torque on a generator is `½[B, G]`,
/// 78.46's lift: zero exactly where `B` turns about `G`'s own axis.
pub const A10_1_TORQUE: &str = "a coupling's torque on a generator is half their commutator, zero along the generator's axis";

/// LAW II · TENSEGRITY BALANCE. Stationary equilibrium iff the net torque
/// `τ = Σ_{i<j} ½[B_ij, B_orbit]` vanishes coefficient by coefficient over ℚ
/// (repair A), `B_ij = ⟨V_ij⟩₂`, `V_ij = (θ_j θ̃_i)(s_j s_i)` (A.10's tether
/// under A.16).
pub const A10_1_II_BALANCE: &str = "a constellation is in stationary equilibrium iff its net commutator torque vanishes";

/// LAW III · STRAIN. `ℰ = ‖τ‖² / ‖B_orbit‖²`, the coefficient norm on the
/// orthonormal blades (repair B): positive-definite, zero exactly where the
/// torque is — a null torque is read.
pub const A10_1_III_STRAIN: &str = "the strain is the torque's coefficient norm over the generator's";

/// THE STAR SETS THE SPIN (78.51(2)). A rigid step splits exactly against a
/// star into its orbit about the star and its spin about the body's own
/// centre; a star elsewhere splits it into another spin.
pub const A10_1_STAR_SETS_SPIN: &str = "a step splits against a star into an orbit about it and a spin about the body";

/// THE INTRINSIC AXIS (78.51(3)). The generator is made about the
/// constellation's own centre — a body's centroid, a scene's barycentre —
/// never an arbitrary origin; the hours stand at their heights on its axis.
pub const A10_1_INTRINSIC_AXIS: &str = "the generator turns about the constellation's own centre, its hours at their heights on its axis";

/// LAW IV · FORCE. An operation carrying frame `k` exerts `ΔE = ℰ(𝒞') − ℰ(𝒞)`:
/// admitted where it does not raise the strain and the constellation it
/// leaves stands in equilibrium; refused as `DestabilizingOperation` where it
/// raises it, `SingularCollapse` where it crushes two frames into one.
pub const A10_1_IV_FORCE: &str =
    "an operation is admitted where it raises no strain and leaves the constellation in equilibrium";

/// THEOREM 1 · BILATERAL INVERSIVE RECIPROCITY. At equilibrium the couplings
/// read both ways, `V_ji ∝ Ṽ_ij` up to the tethers' reverse, and `δ²_ij = δ²_ji`.
pub const A10_1_T1_RECIPROCITY: &str = "at equilibrium the inversive distance is symmetric and the couplings transpose";

/// THEOREM 2 · COMMUTATIVE CO-PLANARITY. Where every coupling commutes with
/// the generator, the constellation circulates coaxially: its torque vanishes
/// term by term, and the modes of the generator commute.
pub const A10_1_T2_COPLANARITY: &str = "couplings commuting with the generator circulate coaxially, the torque zero term by term";

/// THEOREM 3 · ANTI-MONOCULARITY. No operation contracting a constellation to
/// one frame is coherent: the contraction is refused, never filled.
pub const A10_1_T3_ANTI_MONOCULARITY: &str = "a contraction to one frame is refused, never coherent";
