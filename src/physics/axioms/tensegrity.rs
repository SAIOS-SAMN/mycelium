// A.10.1 — ORBITAL TENSEGRITY (ruled 2026-09-28;
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
/// (repair A), `B_ij = ⟨V_ij⟩₂`, `V_ij = θ_j θ̃_i` the pose coupling, the hour
/// carried in the pose (78.52 Row 1; `T_axial` retired)
/// the ATTITUDE coupling (78.51(4′)).
pub const A10_1_II_BALANCE: &str = "a constellation is in stationary equilibrium iff its net commutator torque vanishes";

/// LAW III · STRAIN, PLANE BY PLANE (78.62, ruled 2026-10-01). The torque is
/// split into its own commuting modes and an operation is read by how it
/// moves each plane's torque — the projective ratio of the plane's blade
/// after to before — never by `‖τ‖²/‖B‖²`, a positive-definite sum of squared
/// coefficients (A4 refused). A null torque reads as its parabolic mode.
pub const A10_1_III_STRAIN: &str = "the strain is the torque's modes, each plane read by how an operation moves it";

/// METRIC RIGIDITY (78.51(4′), at unit weight by 78.59). The separation
/// between frames is each pair's product at unit weight, `ŝ_i · ŝ_j`, kept
/// across a transition — against the step's own image where a step dilates
/// — never a squared ratio, never the planar commutator.
pub const A10_1_METRIC_RIGIDITY: &str = "every pair keeps its unit-weight product across the transition";

/// THE VILLARCEAU SUCCESSION (78.52 Row 1). A turn's hours wind on the
/// isoclinic rotor pairing it with the same turn in `e₃e₊`, centred on the
/// constellation's own centre, closing after the turn's own period; a step
/// with no turn winds on its own parameter; the identity is reflexive.
pub const A10_1_VILLARCEAU_SUCCESSION: &str =
    "the hours wind isoclinically on e1e2 and e3e+ and close after the turn's own period";

/// A.13 · MOMENTUM CARRIED (78.52 Row 1). `Π = M₂(B)` — the inertia acting on
/// the step's bivector at the centroid — is carried by an unforced orbit,
/// `Π_{m+1} = g Π_m g̃ / (g g̃)`; what is not carried is the external torque.
pub const A13_MOMENTUM_CARRIED: &str = "an unforced orbit carries its momentum; what it does not carry is external torque";

/// HYPHAL FASCICLES (78.52 Row 2). Every motion is a fascicle's versor
/// `V = s₂ s₁`, read at unit weight `V X Ṽ / (s₁² s₂²)`; its streamline is its
/// spheres' pair read in the algebra — the blade's geometric square negative
/// (elliptic), zero (parabolic), positive (hyperbolic); the slide is the
/// fascicle of parallel planes kissing at `e_∞`.
pub const R2_HYPHAL_FASCICLES: &str =
    "every motion is a fascicle's product, its streamline its spheres' pair read in the algebra; the slide the fascicle kissing at infinity";

/// THE STAR SETS THE SPIN (78.51(2)). A rigid step splits exactly against a
/// star into its orbit about the star and its spin about the body's own
/// centre; a star elsewhere splits it into another spin.
pub const A10_1_STAR_SETS_SPIN: &str = "a step splits against a star into an orbit about it and a spin about the body";

/// THE INTRINSIC AXIS (78.51(3)). The generator is made about the
/// constellation's own centre — a body's centroid, a scene's barycentre —
/// never an arbitrary origin; the hours wind on its succession (78.52).
pub const A10_1_INTRINSIC_AXIS: &str = "the generator turns about the constellation's own centre, its hours winding on its succession";

/// LAW IV · FORCE. An operation carrying frame `k` is read by how it moves the
/// torque plane by plane: admitted where it raises no plane (stabilizing or
/// kept), admitted with its rising planes docketed where it raises some and
/// lowers others (a tilt, 78.62 (b)); refused as `DestabilizingOperation`
/// where it raises some and lowers none, `SingularCollapse` where it crushes
/// two frames into one.
pub const A10_1_IV_FORCE: &str =
    "an operation is admitted where it raises the torque on no plane, or tilts it, and refused where it raises and lowers none";

/// THEOREM 1 · BILATERAL INVERSIVE RECIPROCITY. At equilibrium the couplings
/// read both ways, `V_ji ∝ Ṽ_ij` — the pose and the axial slide commuting on
/// the axis — and the pair reads the same both ways.
pub const A10_1_T1_RECIPROCITY: &str = "at equilibrium the inversive distance is symmetric and the couplings transpose";

/// THEOREM 2 · COMMUTATIVE CO-PLANARITY. Where every coupling commutes with
/// the generator, the constellation circulates coaxially: its torque vanishes
/// term by term, and the modes of the generator commute.
pub const A10_1_T2_COPLANARITY: &str = "couplings commuting with the generator circulate coaxially, the torque zero term by term";

/// THEOREM 3 · ANTI-MONOCULARITY. No operation contracting a constellation to
/// one frame is coherent: the contraction is refused, never filled.
pub const A10_1_T3_ANTI_MONOCULARITY: &str = "a contraction to one frame is refused, never coherent";
