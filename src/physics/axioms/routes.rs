// 79.1, 79.2, AX1 — the route axioms and theorems: one circuit against many
// routes (resolution, 1→n) and many routes to one class (convergence, n→1).
// Each clause carries a test in `tests/routes.rs`; the conjectures enter as
// refused tests, never as assertions.

/// R1 · CARRIER. A transport is an even versor of `Spin(n+1,1)(ℚ)` read as a
/// ray; a constructor is a rational vector `v`, `v² ∈ ℚ*`, odd. A route's
/// transport is the ordered product of its steps, `T(r) = g_k ⋯ g₁`; the
/// reversed route carries the inverse.
pub const R1_CARRIER: &str =
    "a transport is an even rational versor read as a ray, a constructor an odd vector with v² ≠ 0; \
     T(r) is the ordered product of the steps and the reversed route carries its inverse";

/// R2 · CONSTRUCTORS BUILD. Every generator is a product of constructors
/// (54.7 R4); `Ω v Ω̃` is a constructor again.
pub const R2_CONSTRUCTORS_BUILD: &str =
    "every generator is a product of constructors, and a generator carries a constructor to a constructor";

/// R3 · LOCALITY. Routes are compared inside one cell — the members standing
/// at one witness (NB1) — a finite complex, never across the whole form.
pub const R3_LOCALITY: &str = "routes are compared inside one cell: the members standing at one witness";

/// R4 · CURVATURE IS HOLONOMY. A 2-cell's curvature is the transport of its
/// ordered boundary, `F(σ) = T(∂σ)`; `σ` is flat iff `F(σ)` is the identity
/// ray (`±1` one ray, EQ1).
pub const R4_CURVATURE_IS_HOLONOMY: &str =
    "F(σ) = T(∂σ), and σ is flat iff F(σ) is the identity ray, ±1 one ray";

/// R5 · BASING. Every loop is carried to the basepoint by the tree
/// transport, `ρ(ℓ) = θ T(ℓ) θ⁻¹` (Axiom A, 54.7).
pub const R5_BASING: &str = "every loop is based by the tree transport: ρ(ℓ) = θ T(ℓ) θ⁻¹";

/// T1 · TWO ROUTES. `H(r, s) = T(s)⁻¹ T(r)` is the holonomy of `s⁻¹ r`, and
/// `T(r) ∼ T(s)` iff `H(r, s) ∼ 1`.
pub const T1_TWO_ROUTES: &str = "two routes agree iff their loop's holonomy is the identity ray";

/// T2 · DISCRETE NON-ABELIAN STOKES. Where `s⁻¹ r` bounds cells `σ₁ … σ_m`,
/// `H(r, s)` is the product of their curvatures, each carried to the
/// basepoint by its lasso, in the boundary's order; flat cells give
/// agreeing routes. THE CONVERSE FAILS on a relator: non-trivial curvatures
/// multiply to the identity, so agreeing routes never prove the cells flat.
pub const T2_DISCRETE_STOKES: &str =
    "H(r, s) is the lassoed product of the enclosed curvatures in boundary order; the converse fails on a relator";

/// C1 · 1→n, RESOLUTION. (a) `H(rᵢ, rⱼ) = Hⱼ⁻¹ Hᵢ`: `n` readings against one
/// reference give every pair; (b) the content is route-independent iff
/// every `Hᵢ ∼ 1`; (c) the circuits' actions generate a ℚ-algebra whose rank
/// is the cell's capacity; (d) within one generator the same shape is the
/// mode split.
pub const C1_RESOLUTION: &str =
    "n readings against one reference give every pair; route independence iff each is the identity; \
     the rank of the actions' algebra is the capacity; the mode split is the same shape";

/// C2 · n→1, CONVERGENCE. (a) Conjugation preserves `I₁`, `I₂` and the kind,
/// so minds basing one circuit at their own points agree on its class;
/// (b) many constructor chains compose to one versor — synonyms; (c) the
/// convergence of replays on the factorization is open.
pub const C2_CONVERGENCE: &str =
    "conjugation preserves the class, so based readings converge on it; many constructor chains compose to one versor";

/// THE CONJECTURES, by name, each held open by the row that would decide it
/// and entered as a refused test.
pub const ROUTE_CONJECTURES: [(&str, &str); 3] = [
    (
        "the held constructors produce a second route inside every single-route cell",
        "CT2: built from held reflections, T1 on the pair; where none exists the wall is permanent",
    ),
    (
        "the constructor orbit closes iff every mode is elliptic of finite order",
        "CS1, CG1: refused read per generator — two generators each of finite elliptic modes need not close",
    ),
    (
        "a sign difference between two routes is the held ℤ₂",
        "T1 reads rays and is blind to the sign; the ℤ₂ is read by the coset (78.6), not by T1",
    ),
];
