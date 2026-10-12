// MU-RECURSION §0.0 — THE FOUNDING PRINCIPLES: what the mathematics is
// derived from. Every axiom of this register, every law the court serves and
// every pivot a mind makes is read from one of the three, and each carries a
// test in `tests/founding.rs` that could fail.
//
//! THE PHYSICS FIRST, THE MATHEMATICS FROM IT. A motion of `Cl(4,1)` over ℚ
//! splits exactly into modes (MR1), and a mode is one of three kinds. Each
//! kind is a principle:
//!
//! - **HARMONIC** — the elliptic mode: a turn, which RETURNS. Its order is read
//!   off it (Niven: 1, 2, 3, 4, 6 among rational turns), or it has none and
//!   never returns (a rosette, whose winding is content). Return is the
//!   elastic vent, the period, the class that comes back.
//! - **LUMINAL** — the parabolic mode: a null generator, `B² = 0`, on the
//!   light cone. It has no length, only its ray and its ratios on it; flux
//!   along it is frozen into its tube and conserved by the flow.
//! - **HOLONOMIC** — the closed loop: what a circuit of transports composes to
//!   is its content. Moving the loop's base point conjugates it and changes
//!   none of its modes, so the class is the loop's, never the frame's.
//!
//! The hyperbolic mode (a boost or a dilation) is the luminal one off the
//! cone: two null rays, the field's expansion between them.
//!
//! TWO COROLLARIES BIND EVERY READER:
//!
//! - **EVERY DIMENSION IS AN EXPONENT.** A grade is the number of factors in a
//!   product of orthogonal generators; a cell's degree is its vertices less
//!   one; a base of `n` axes has a unit cell of `2ⁿ` vertices. Nothing caps a
//!   degree: the complex climbs as far as its faces stand (A.6).
//! - **NO CONSTANT.** A number in the physics is read off the field or the
//!   geometry, or it is undefined physics, named in [`OPEN_PHYSICS`] until it
//!   is derived. A bare number standing for a reading is refused.

/// HARMONIC — a turn returns at its read order, or never.
pub const HARMONIC_RETURN: &str =
    "an elliptic mode returns: its order is read off it (Niven's 1, 2, 3, 4, 6), or it has none and never returns, its winding content";

/// LUMINAL — a null generator has no length; its flux is frozen in.
pub const LUMINAL_NULL_FLUX: &str =
    "a parabolic mode is null: B² = 0, no length, only its ray and ratios; conjugation keeps it null and its flux on its tube";

/// HOLONOMIC — a loop's composite is its content, invariant under its base.
pub const HOLONOMIC_CONTENT: &str =
    "a closed loop's holonomy is its content: moving its base point conjugates it, and its modes — its class — are unchanged";

/// AN ORBIT IS A RETURN (harmonic): a mode's flow on its plane, composed
/// exactly, returning where its point is the identity (78.73, 78.75). A tick
/// is the host handing a mind a turn, and a node's `K` is a coordinate of its
/// ledger (OK1): neither is an orbit, and no orbit is counted by either.
pub const ORBIT_IS_A_RETURN: &str =
    "an orbit is a mode's flow on its plane, returning where its point is the identity; a tick is a turn of the host and K a ledger coordinate, never an orbit";

/// EVERY DIMENSION IS AN EXPONENT.
pub const DIMENSION_IS_EXPONENT: &str =
    "a grade is the count of orthogonal factors in a product; a cell's degree its vertices less one; a base of n axes has 2ⁿ vertices in its unit cell; no degree is capped";

/// NO CONSTANT.
pub const NO_CONSTANT: &str =
    "a number in the physics is read off the field or the geometry, or it is undefined physics named until it is derived";

/// UNDEFINED PHYSICS, NAMED: each number the tree still declares where a
/// reading belongs, with what it stands for and what will derive it.
pub const OPEN_PHYSICS: [(&str, &str); 2] = [
    (
        "CHALLENGE_WINDOW = 1024",
        "law parameter of version 4, shared by every kernel of a network universe (78.125); its derivation — the heights a fold's \
         challenge needs to reach every keeper and return — is a law amendment, read with the coordinator",
    ),
    (
        "THROAT_GRID = 64",
        "the 2^-64 grid a throat's cosh horizon is enclosed on (78.134): an approximation; the throat is a Cl(4,1) round carried by \
         its bank's modes, fit by meet, and the grid retires with it",
    ),
];
