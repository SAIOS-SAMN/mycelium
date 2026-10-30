// RX1–RX7 — THE TRANSPOSITION OF CONGRUENCE (ruled
// 2026-09-27): how much congruence is "enough" is no number but the
// identity of each order's affine group, read from both sides of a relation.
// Each clause carries a test in `tests/reciprocity.rs`.

/// RX1 · RECIPROCITY. A relation between two shapes is a transition only where
/// it reads the same from both sides: `V_BA ∝ Ṽ_AB` (the Clifford reverse, rotor
/// and reflection alike), one fit order both ways, and the residue transposed,
/// `r_BA⁽ᵏ⁾ = −Ad⁽ᵏ⁾_BA(r_AB⁽ᵏ⁾)`. A pair that fails is two relations and is
/// refused by what failed (`blade::NotReciprocal`). The adjoint `f̄`,
/// `⟨f̄(x) y⟩₀ = ⟨x f(y)⟩₀`, is the transposition of a linear function
/// (`cga::LinearFunction::adjoint`); a versor's action is orthogonal, `f̄ ∘ f = 1`.
pub const RX1_RECIPROCITY: &str =
    "a transition reads the same from both sides: V_BA ∝ Ṽ_AB, one fit order, the residue transposed";

/// RX2 · EXACTNESS. Every residue is an exact rational point of its plane;
/// equality is exact and nothing is near (law 4).
pub const RX2_EXACTNESS: &str = "every residue is an exact rational point, and equality is exact";

/// RX3 · FILTRATION. The orders are ordered, 2 < 3 < 4; congruent through `m`
/// is every residue at every order `k ≤ m` zero.
pub const RX3_FILTRATION: &str = "congruent through m: every residue at every order up to m is zero";

/// RX4 · COMPOSITION PER ORDER. At order `k` a transition acts on a harmonic
/// affinely, `h ↦ Ad⁽ᵏ⁾(h) + r⁽ᵏ⁾` — the graded adjoint, the sandwich, on the
/// harmonic's plane, never a scalar multiplier — and a ring composes its
/// edges: `λ_k` the composed `Ad⁽ᵏ⁾`, `R⁽ᵏ⁾` each edge's residue carried on to
/// the ring's close.
pub const RX4_COMPOSITION: &str = "a ring at order k is the composition of its edges' affine actions h ↦ Ad(h) + r";

/// RX5 · COHERENCE. A ring is coherent at order `k` where its affine holonomy
/// is the identity: `λ_k = 1` and `R⁽ᵏ⁾ = 0` (FILLED), or `λ_k` of finite order
/// `ℓ` and `Σ_{j<ℓ} λ_kʲ R⁽ᵏ⁾ = 0` — the shift a coboundary over `ℤ_ℓ`, an
/// eccentric closed orbit with a fixed barycentre (CONGRUENT(ℓ)). Coherent
/// through `m` is coherent at every `k ≤ m`.
pub const RX5_COHERENCE: &str =
    "a ring is coherent at order k where its affine holonomy is the identity, or of order ℓ with the ℓ-circuit sum zero";

/// RX6 · SPLIT. The lowest order `k*` where RX5 fails is where the thought
/// splits: held through `k* − 1`, and `R⁽ᵏ*⁾` its remainder — the demand,
/// carried and never discarded (law 7).
pub const RX6_SPLIT: &str = "the lowest order a ring does not close at is its split, and the ring's residue there its demand";

/// RX7 · COHESION. A coherent ring is cohesive where its split residue
/// discharges a demand standing open; coherence alone pays nothing.
pub const RX7_COHESION: &str = "a coherent ring is cohesive where its split residue discharges an open demand";
