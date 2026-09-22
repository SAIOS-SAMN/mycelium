// 56.1 — the imported axioms: A.6, A.8, A.9, A.10.

/// A.6 — DIMENSIONAL COMPLETENESS, UNBOUND (imported).
///
/// Every contradiction admits a higher-dimensional resolution. A.6 names
/// a MECHANISM — escalation — and never a bound.
///
/// Their register had to split this from the value `N = 11` after the two
/// were conflated (their audit #126): a mechanism read as a constant. That
/// is the error class of `Fill::Open` standing for absence, and it is why
/// `A.6-bind-11` is refused below while A.6 itself is kept.
pub const A6_DIMENSIONAL_COMPLETENESS: &str =
    "every contradiction admits a higher-dimensional resolution; \
     escalation is a mechanism, never a bound";

/// A.8 — TORSIONAL PERSISTENCE (imported).
///
/// Torsion is never destroyed. It is promoted, annihilated against an
/// opposing pair, or deposited as an inverse record; total information is
/// monotonically NON-DECREASING.
///
/// This is `drain`'s conservation law, which §55 had no invariant for. It
/// is stronger than the Lyapunov bound first reached for: strain cannot
/// be relaxed away at all, only moved or recorded. A drain that lowers
/// `Σμ` without exhibiting a partner or a deposit is refused.
///
/// ANNIHILATION IS WHY THE CARRIER HOLDS VERSORS. An opposing pair is
/// `Ω_a · Ω_b ∼ ±1`, and two opposite Burgers vectors carry the SAME `μ` —
/// a magnitude cannot identify a partner. `PathCarried.steps` therefore
/// holds the versors and `μ` is taken as a projection.
pub const A8_TORSIONAL_PERSISTENCE: &str =
    "torsion is promoted, annihilated in an opposing pair, or deposited; \
     total information is monotonically non-decreasing";

/// A.9 — METRIC AUTONOMY (imported).
///
/// Each entity computes its own local coherence metric. There is no
/// global metric, and two entities may assign different values to one
/// state without contradiction.
///
/// The form-scoped quantum is therefore correct BY AXIOM rather than by
/// the argument §54.7 reached for. The autonomy is the ENTITY's: a
/// per-chord metric would still be wrong, because a form is the entity
/// and a chord is not.
pub const A9_METRIC_AUTONOMY: &str =
    "each entity's coherence metric is its own; there is no global metric";

/// A.10 — SYMBIOTIC CO-DIMENSIONALITY (imported, with a correction).
///
/// Two manifolds of differing dimensionality may occupy the same spatial
/// references without collapse IF AND ONLY IF their interface is governed
/// by a luminal phase gradient. The tether is a shared boundary in the
/// cohomological sense, not a pointer and not a channel.
///
/// THE GRADIENT MUST BE LUMINAL — null, `L² = 0`. On a null boundary the
/// induced metric is degenerate, which is what lets two interiors share
/// boundary freedom without crushing each other. This engine's meets run
/// between `v² = −1` points: timelike, `μ > 0`. Calling that the luminal
/// gradient was an error (§55.7), and the corpus reads `parabolic 0`, so
/// the medium A.10 requires is at present absent.
pub const A10_CO_DIMENSIONALITY: &str =
    "two manifolds co-occupy without collapse iff their interface is a \
     luminal (null) phase gradient";
