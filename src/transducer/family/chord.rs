// 54.7 — the transport a chord carries: the dispiration between two
// sections, and the Burgers vector between their footprints on the sheet.
// 78.96 (R3 ruled LUMINAL, 2026-10-04): the Burgers vector is carried by its
// own conformal translator, a null generator, never by a boost.

use crate::transducer::base::Site;

/// THE BURGERS VECTOR BETWEEN TWO FOOTPRINTS, carried LUMINALLY (54.7 R3
/// as ruled 2026-10-04, 78.96).
///
/// The footprint's basepoint is its centre of mass — equivariant,
/// `p(g·D) = g·p(D)`, the intrinsic representative (a lexicographic
/// minimum was extrinsic, rule 15) — and the displacement is
/// `τ = p(D_b) − p(D_a)`, a difference, so it does not depend on where the
/// scene was drawn.
///
/// HOW IT WAS CARRIED, AND WHY IT NO LONGER IS. 54.7 lifted both centres
/// onto the hyperbolic sheet at the form's quantum `M(X)` and transported
/// between them by Axiom B's transition `R = 1 − v_b v_a`, a geodesic
/// TRANSVECTION — a boost. A boost never returns: every ring that crossed a
/// chord read unbound, on every family (slide, flip, swap, turn — 78.75),
/// so no sentence a mind composed ever closed, no shortcut was taught, and
/// no word was built from a path (78.95's reading at epoch 121).
///
/// A pure displacement's own generator is NULL: `τ ∧ e_∞`, with
/// `(τ ∧ e_∞)² = 0`. Its versor is the conformal translator
/// `T(τ) = 1 − ½ τ e_∞`, and on that one direction composition is plain
/// addition — `T(d₁)T(d₂) = T(d₁ + d₂)`, `T(τ)^m = T(m·τ)` (78.68's
/// parabolic row, 78.75's `k = B² = 0`). So a ring of slides either closes
/// flat (`Σ τ = 0`) or extrapolates as a NILPOTENT theorem,
/// `Ωᵐ ∝ 1 + m·N`, its history one integer: luminal, as 78.68 names the
/// carriage's translation. No chart and no quantum are needed — the null
/// generator is exact on the base axes — so `form_chart` and the sheet
/// retire with the boost.
///
/// The R3 objection stands where it was raised and nowhere else: reading
/// EVERY edge's motor as a translator (`edge_of`) forced `I₁ = 0` on edges
/// that turn and synthesised parabolic cycles. This is the chord's Burgers
/// vector alone — two copies of one piece, apart by a displacement and
/// nothing else — whose true generator is null; `edge_of` keeps its own
/// reading.
///
/// `None` where the two footprints share a centre, since a zero
/// displacement is not a dislocation and none is invented (rule 11).
fn basepoint_translation(da: &[Site], db: &[Site]) -> Option<crate::transducer::sentence::Transport> {
    // THE INTRINSIC CENTRE of a support domain: its centre of mass, exact
    // over ℚ and invariant under the order the compartments arrive in, on every
    // axis the sites have (BQ4). A footprint mixing two bases has no centre
    // and is counted as unfooted.
    let centre = |d: &[Site]| -> Option<Vec<num_rational::BigRational>> {
        let [first, ..] = d else { return None };
        let n = num_bigint::BigInt::from(u64::try_from(d.len()).ok()?);
        let mut sum: Vec<num_bigint::BigInt> = vec![num_bigint::BigInt::from(0); first.n()];
        for s in d {
            if s.n() != sum.len() {
                return None;
            }
            for (acc, a) in sum.iter_mut().zip(s.axes()) {
                *acc += num_bigint::BigInt::from(*a);
            }
        }
        Some(sum.into_iter().map(|s| num_rational::BigRational::new(s, n.clone())).collect())
    };
    let (a, b) = match (centre(da), centre(db)) {
        (Some(a), Some(b)) if a.len() == b.len() => (a, b),
        _ => {
            crate::transducer::family::cover::tally(|t| t.tau_unfooted += 1);
            return None;
        }
    };
    if a == b {
        // THE TWO ENDS MET: two footprints sharing a centre stand over ONE
        // base point, so they are two sections of one fibre and their
        // difference is VERTICAL. There is no horizontal displacement for a
        // 1-cell to carry, so none is invented (rule 11); what differs
        // between them is higher-moment shape, a 2-cell's business.
        crate::transducer::family::cover::tally(|t| t.tau_met += 1);
        return None;
    }
    let tau: Vec<num_rational::BigRational> = b.iter().zip(&a).map(|(y, x)| y - x).collect();
    Some(crate::transducer::sentence::Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: crate::transducer::cga::translator_at(&tau),
        gauge: Vec::new(),
    })
}

/// THE DEFECT A CHORD CARRIES — a DISPIRATION, not one of its halves.
///
/// A first cut wrote `measured.or_else(transvection)`: the rotational
/// reading OR the translational one, whichever was present. That is the
/// projection into uncoupled sub-actions — a chord whose two sections
/// differ in BOTH carriage and footprint has a rotation about an axis
/// and a slip along it at once, and choosing one discards the coupling
/// that makes it helicoidal.
///
/// ```text
///     Ω_ab = (φ_b φ_a⁻¹) · T(τ),   τ = p(D_b) − p(D_a)
/// ```
///
/// The composite's generator is non-simple where a turn and a slip along
/// its axis are both present, so `I₂ = ⟨B∧B⟩₄ ≠ 0` and the screw survives.
/// Where the carriages agree it degenerates to a pure Volterra dislocation —
/// a uniform Burgers vector, now luminal — and where the footprints agree it
/// is the measured rotation alone. Each is a reading of what the two
/// sections differ in, never a shape imposed on all of them.
pub(super) fn dispiration(
    measured: Option<&crate::transducer::sentence::Transport>,
    da: &[Site],
    db: &[Site],
) -> Option<crate::transducer::sentence::Transport> {
    let slip = basepoint_translation(da, db);
    crate::transducer::family::cover::tally(|t| {
        t.chords_offered += 1;
        if slip.is_some() {
            t.tau_active += 1;
        }
    });
    match (measured, slip) {
        (None, s) => s,
        (Some(m), None) => Some(m.clone()),
        // BOTH: compose, and the product is the helicoidal transport.
        (Some(m), Some(s)) => Some(crate::transducer::sentence::Transport {
            word: m.word.clone(),
            carriage: m.carriage.geo(&s.carriage),
            gauge: m.gauge.clone(),
        }),
    }
}
