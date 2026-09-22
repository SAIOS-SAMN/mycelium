// 56.2 — the refused axioms, each with the flattening it would bring.

/// A.4 — RATIONAL INNER PRODUCT, "positive-definite" (REFUSED).
///
/// False in this algebra. `Cl(4,1)` is indefinite, `e₋² = −1`, and the
/// sheet is `−t² + x² + y² + z² = −1`. Positive-definiteness would flatten
/// every boost generator into an elliptic rotation and erase the
/// non-compact sector — which is the sector this corpus entirely occupies.
pub const A4_POSITIVE_DEFINITE_REFUSED: &str =
    "refused: the metric is indefinite, e₋² = −1";

/// A.7 — HOLONOMIC INVARIANT as written (REFUSED).
///
/// "Closed paths always return to their topological starting class modulo
/// torsion", valued in `H¹(X, ℤ)` — the abelian, torsion-only picture. The
/// hyperbolic sector never returns: `cosh(kβ) > 1` for every `k ≥ 1`,
/// which is the measured `closes 0`. A.7 holds for the compact half alone,
/// and importing it whole asserts a closure measured to be absent.
pub const A7_ABELIAN_RETURN_REFUSED: &str =
    "refused for the non-compact sector: cosh(kβ) > 1 forbids return";

/// A.6-bind-11 — `N = 11` EXACTLY (REFUSED).
///
/// Sourced to an architecture document; their register is honest that it
/// is a constitutional BINDING and not a theorem of the algebra. A
/// hardcoded generator is refused whatever register it arrives in.
pub const A6_BIND_11_REFUSED: &str =
    "refused: a dimension count from an architecture document is not a theorem";
