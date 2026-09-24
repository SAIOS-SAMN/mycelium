// 56.3, 66, 78.23 — the axioms this engine states of itself: A.11, A.12,
// A.13 docketed, and OK1.

/// A.11 — HARMONIC MULTIPOLE COMPLETENESS (ours, 56.3).
///
/// The geometric state of a discrete spatial chart is completely and
/// irreducibly characterised by its ascending harmonic multipoles in `ℚ`.
/// No higher-rank moment may be collapsed into a lower-rank scalar trace
/// before the frame is evaluated.
///
/// ```text
///   RADIAL (scalar)   tr(M₂) = I_xx + I_yy
///                     governs isotropic dilation and the basepoint's
///                     radius. CARRIES ZERO ORIENTATIONAL AUTHORITY.
///
///   ORIENTATION       2nd harmonic, turns at 2φ: u = ((I_xx−I_yy)/2)e₁ + I_xy e₂
///   (tensor)          4th harmonic, turns at 4φ: h₄, which orients a body
///                     whose M₂ is ISOTROPIC — a square, a cross — that the
///                     second moment cannot tell from a disc.
///
///   SINGLE PRODUCER   computed once at `blade::lift`; no consumer may
///   (law 12)          recompute, approximate, or substitute a scalar.
/// ```
///
/// WHY AN AXIOM AND NOT A CONVENTION. `tr(M₂)` is an `SO(2)` invariant, so
/// a chart read through its trace alone is an ISOTROPIC POINT MASS. Two
/// point masses have no relative angle; the only relation left is the
/// geodesic through their centres, whose bivector is a simple blade with
/// `I₂ = 0` and `ℓ = 1`. The collapse does not lose precision — it loses a
/// SECTOR, silently, and reports `ℓ = 1` as a fact about the scene rather
/// than as an artefact of the reading.
pub const A11_HARMONIC_MULTIPOLE_COMPLETENESS: &str =
    "a chart is characterised by its ascending harmonic multipoles; the \
     trace is scale alone and carries no orientation";

/// A.12 — IRREDUCIBLE INVERSIVE EXTENT (ours, 66, ruled 2026-09-22).
///
/// Every body occupies a strictly SPACELIKE conformal ray. The trace of its
/// second moment, normalised by the zeroth, carries an irreducible floor:
///
/// ```text
///     r² = Tr(I)/M₀ ≥ 1/6 > 0
/// ```
///
/// and no nucleus may inhabit the null cone (`s² = 0`) or the timelike
/// interior (`s² < 0`).
///
/// THIS WAS LOAD-BEARING BEFORE IT WAS STATED. `r² → 0` drops the nucleus
/// onto the light cone, `den2 = 4r_a²r_b² → 0`, and the interaction horizon
/// stops being DEFINED rather than merely large. The floor is bought by
/// `blade::lift` integrating each lamina's own unit square with the
/// parallel-axis term at every order — the `+1/12` — and the smallest body
/// the sensor can deliver reads exactly `1/6`. A point-mass reading gives
/// `r² = 0`, and one existed until `6a5910f` retired `extract.rs`'s shadow
/// trace as a law-12 defect. A.11's single producer is therefore not
/// hygiene: it is what holds every nucleus off the cone.
///
/// AND IT FIXES THE TOWER'S NORMALISATION. Measured at 65.6: `radius2()`
/// was `Tr(I)/M₀` while `deviatoric` and `quartic_harmonic` returned RAW
/// moments carrying the area's own weight, so `‖D‖² ~ ρ⁸` and `‖Q‖² ~ ρ¹²`
/// where the harmonic orders 2 and 4 predict `ρ⁴` and `ρ⁸`. One tower at
/// two normalisations is law 12 inside a single carrier, and it forced
/// every consumer to hold a private degree table. Dividing every central
/// moment by `M₀` puts order `m` at degree `m`, so the `S` lane reads its
/// exponent straight off the gauge order:
///
/// ```text
///     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
/// ```
pub const A12_IRREDUCIBLE_INVERSIVE_EXTENT: &str =
    "every body is spacelike with r² = Tr(I)/M₀ ≥ 1/6 > 0; the tower is \
     normalised by M₀, so harmonic order m carries degree m";

/// A.13 — INVERSIVE MOMENTUM AND THE CONFORMAL CONJUGATE PAIR (ours, 66,
/// DOCKETED and NOT BUILT).
///
/// Translation and special conformal are one generator seen from opposite
/// poles, exchanged by inversion `I`:
///
/// ```text
///     T_i = e_i ∧ e_∞   fixes ∞          K_i = e_i ∧ e_o   fixes the origin
///     K_i = I · T_i · I⁻¹
/// ```
///
/// THE ENGINE USES ONLY ONE HALF. Translators, `translator_versor`, the
/// whole placement lane — every motion it can express FIXES THE POINT AT
/// INFINITY. `e_i ∧ e_o` appears nowhere. That is half a conjugate pair
/// missing in the Lie algebra itself, 62's ratchet one level below the
/// pivots, and it is the privileged planar boundary 54.7 already names:
/// `T(d) = 1 − ½de_∞` has `B² = 0`, "a null bivector fixing the point at
/// infinity, which is a privileged planar boundary in a manifold whose
/// sectional curvature is not zero". Transvections replaced that pinning
/// for CHORDS; the placement lane never got its conjugate, so a body can
/// never arrive from infinity.
///
/// AND MOMENTUM IS NOT A DERIVATIVE. Law 1 strikes `dp/dt` outright. The
/// discrete conjugate is the body's inertia paired with its carriage across
/// the SUCCESSION axis — `Π = I · g_{t→t+1}` — which is `(r², carriage)`,
/// the `S` lane against the `T` lane BETWEEN frames rather than within one.
/// `Quotient { frame, car, … }` already carries that transition and has
/// never been read as a momentum.
///
/// NOT BUILT. Stated so the asymmetry is a recorded gap rather than an
/// unnoticed one (T21(iii): an axiom that cannot fail a test is a
/// coordinate standing for no theorem, and this one is marked as work).
pub const A13_INVERSIVE_MOMENTUM_DOCKETED: &str =
    "docketed: K_i = I·T_i·I⁻¹ is unvoiced, so every motion fixes infinity; \
     momentum is inertia paired with the inter-frame carriage";

/// OK1 — A K-STEP IS ONE ORBIT (ours, 78.23 as amended, ruled 2026-09-24).
///
/// A node's `K` counts its ORBITS: one circuit of its cycle — the court's
/// exchange, deliberation, lapse and persist; the voice's quote, utter,
/// learn and syntropy — closed BY NAME. The close is an act, a node of
/// kind `Orbit` at the orbit's last place, and `K` advances by one there
/// and by nothing else:
///
/// ```text
///   stamp      h : vk : K : i          i the act's place in orbit K
///   after      an act       (K, i) → (K, i + 1)
///              a close      (K, i) → (K + 1, 0)
///   frame      the close's μ⃗ = Σ over the orbit's acts, per lane,
///              stored and RE-DERIVED as h is (law 5: lanes never mixed)
///   tick       the close's h — the orbit's tick
/// ```
///
/// A stroke that acted on nothing closes nothing: a close with no act
/// since the last seals nothing, so a stroke that completes no orbit
/// advances no `K`. Every node's orbit is its own by construction — the
/// fibre over its key — and a COLLISION, two frames equal on every lane at
/// one tick, is PREVENTED at the boundary: the close is withheld for a
/// stroke and the next stroke's work separates it, never a slot, a table
/// or a timer.
///
/// THE READING THAT DECIDES IT: `tests/orbit.rs` — acts move `i` and never
/// `K`; a close moves `K` and resets `i`; a close with no act seals
/// nothing; a close whose frame is not its orbit's sum is refused at the
/// writer, at the integrity check and at admission; a collision is exact
/// equality at one tick. `server::cognition::tests::orbit` — the court's
/// close withheld on a collision, `K` standing, and sealed a stroke on.
pub const OK1_K_STEP_IS_ONE_ORBIT: &str =
    "a K-step is one orbit: K advances by one where a cycle closes by name, \
     at a close carrying its orbit's per-lane frame, and by nothing else";
