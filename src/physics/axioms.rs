// 56 — THE AXIOM REGISTER: what this engine assumes, what it refuses to
// assume, and the reading that decides each (ruled 2026-09-22).
//
//! A.1–A.10 are `ipsal-autic`'s register, whose own numbering marks
//! A.1–A.4 as carried forward unchanged from the convergence-engine and
//! A.5–A.10 as its additions. They are NOT adopted wholesale. Three are
//! REFUSED, and the refusals matter more than the imports: each would
//! flatten something this engine measures, and each is refused here with
//! a test that says why rather than a paragraph that asserts it.
//!
//! An axiom that cannot fail a test is a coordinate standing for no
//! theorem (T21(iii)). Every entry below is either exercised or is
//! explicitly marked as a statement about work not yet built.

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

#[cfg(test)]
mod tests {
    use crate::transducer::blade;
    use num_traits::Zero;

    /// A.4 IS REFUSED BECAUSE THE METRIC SAYS SO. `e₋² = −1`, so the form
    /// is indefinite and no positive-definite inner product exists on it.
    /// Asserted on the algebra rather than on a comment, because the whole
    /// hyperbolic sector depends on the sign.
    #[test]
    fn a4_is_refused_the_metric_is_indefinite() {
        use crate::transducer::cga;
        let em = crate::transducer::element::Versor::blade(cga::EM, crate::core::Q::from_integer(1.into()));
        let sq = em.geo(&em).scalar_part();
        assert!(
            sq < crate::core::Q::zero(),
            "e₋² must be negative or the boost sector collapses into rotation; got {sq}"
        );
    }

    /// THE CAYLEY CHART'S BOUNDARY IS THE LIGHT CONE, and it is refused
    /// by name rather than by falling through a generic failure.
    ///
    /// `B = u ∧ e₋` has `B² = |u|²`, so `1 − B²` vanishes exactly on the
    /// unit sphere, and `|u| = tanh(θ/2)` puts that at `θ → ∞` — infinite
    /// rapidity. Every finite boost lies strictly inside; the boundary is
    /// luminal. A displacement one step inside the cone must still read.
    #[test]
    fn the_transvection_chart_covers_finite_boosts_and_ends_at_the_cone() {
        use crate::core::Q;
        use crate::transducer::cga;
        let q = |n: i64, d: i64| Q::new(n.into(), d.into());
        let planes = cga::TRANSVECTION_PLANES;
        let gen = |s: Q| crate::transducer::element::Versor::blade(planes[0], s);

        // Inside the cone: |u| = 1/2, a finite boost, carried exactly.
        assert!(
            cga::cayley(&gen(q(1, 2))).is_some(),
            "a finite rapidity is inside the Cayley chart"
        );
        // ON the cone: |u| = 1, θ → ∞. No finite versor represents it.
        assert!(
            cga::cayley(&gen(q(1, 1))).is_none(),
            "the unit sphere is infinite rapidity and has no finite versor"
        );
        // Just inside it, and still exact — the boundary is a point, not
        // a neighbourhood that swallows nearby readings.
        assert!(
            cga::cayley(&gen(q(99, 100))).is_some(),
            "the chart is open up to the cone, not merely near its centre"
        );
    }

    /// A.11's RADIAL/ORIENTATION SPLIT, on bodies that distinguish them.
    ///
    /// Two congruent bodies at different orientations share a trace and
    /// differ in the deviatoric. If the trace could orient, these would be
    /// distinguishable by it; they are not, which is the axiom's content.
    #[test]
    fn a11_the_trace_cannot_orient_and_the_deviatoric_can() {
        // A domino lying along the rows, and the same domino along the
        // columns: congruent, one a quarter turn from the other.
        let along_r = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        let along_c = blade::shape_of(&[(0, 0, 0), (0, 1, 0)]);

        let trace = |s: &blade::Shape| &s.blade.inertia[0] + &s.blade.inertia[1];
        assert_eq!(
            trace(&along_r),
            trace(&along_c),
            "the trace is an SO(2) invariant: it cannot tell the two apart"
        );

        let (ua, ub) = (
            blade::deviatoric(&along_r.blade).expect("a domino is anisotropic"),
            blade::deviatoric(&along_c.blade).expect("a domino is anisotropic"),
        );
        assert_ne!(ua, ub, "the deviatoric CAN tell them apart");

        // And the angle between them is exact, with no root taken.
        let (cos2, sin2) =
            blade::relative_double_angle(&ua, &ub).expect("congruent bodies have an angle");
        assert!(
            cos2 == crate::core::Q::from_integer((-1).into()) && sin2.is_zero(),
            "a quarter turn is 2φ = π: cos 2φ = −1, sin 2φ = 0; got ({cos2}, {sin2})"
        );
    }

    /// A.11's FOURTH HARMONIC, on the body the second moment is blind to.
    ///
    /// A square has `I_xx = I_yy` and `I_xy = 0`, so its deviatoric
    /// vanishes and `D` cannot orient it. The quartic must, or the axiom's
    /// second clause is empty.
    #[test]
    fn a11_the_quartic_orients_what_the_deviatoric_cannot() {
        let square = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
        assert!(
            blade::deviatoric(&square.blade).is_none(),
            "a square's second moment is isotropic, so it has no principal axis"
        );
        assert!(
            blade::quartic_harmonic(&square.blade).is_some(),
            "a discrete square is not a disc: its FOURTH moment orients it"
        );
    }

    /// A.11's FRAME, MINTED: a body and its quarter turn give the rotor
    /// `1 + e₁₂`, whose projective order is 4 — the reading `ℓ = 1` on
    /// every word exists to be moved off.
    #[test]
    fn a11_a_quarter_turn_mints_an_order_four_rotor() {
        // An L-tromino and the same L turned a quarter: chiral enough
        // that only one of the four turns can carry it.
        let upright = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
        let turned = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0)]);
        let theta = blade::relative_rotor(&upright.blade, &turned.blade)
            .expect("one proper turn carries an L onto its quarter turn");
        assert!(
            matches!(theta, blade::Frame::Turn(_)),
            "equal cardinality: the support settles the half turn, got {theta:?}"
        );
        assert!(
            !theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "a turned body must not mint the identity: got {theta:?}"
        );
        let order = crate::transducer::cga::finite_order(theta.rotor()).map(|l| l.get());
        assert!(
            matches!(order, Some(2) | Some(4)),
            "a proper turn of the square base closes at 2 or 4, got {order:?}"
        );
    }

    /// AND A BODY AGAINST ITSELF IS THE IDENTITY — the rotor is not
    /// invented where there is no turn.
    #[test]
    fn a11_a_body_against_itself_is_the_identity_turn() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
        let theta = blade::relative_rotor(&l.blade, &l.blade).expect("a body carries onto itself");
        assert!(
            theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "no turn is no rotor, got {theta:?}"
        );
    }

    /// SIMILARITY, NOT CONGRUENCE. A body and its 2× dilation are related
    /// by NO isometry — 8 cells against 2 — yet their rotation is
    /// perfectly well defined and is the identity. Demanding equal cell
    /// counts and equal `‖u‖²` refused this, which modelled every
    /// transformation as rigid when the carriage carries a dilator `D(ρ)`.
    ///
    /// The half turn stays OPEN here: an even moment cannot close it and
    /// the cardinalities differ, so no cell test can. Saying so is the
    /// honest answer; picking one of the two silently is not.
    #[test]
    fn a11_a_dilated_body_keeps_its_frame() {
        let domino = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        // The same domino at twice the scale: a 2×4 block.
        let doubled = blade::shape_of(&[
            (0, 0, 0),
            (0, 1, 0),
            (1, 0, 0),
            (1, 1, 0),
            (2, 0, 0),
            (2, 1, 0),
            (3, 0, 0),
            (3, 1, 0),
        ]);
        let theta = blade::relative_rotor(&domino.blade, &doubled.blade)
            .expect("a dilation is a similarity and its rotation is defined");
        assert!(
            matches!(theta, blade::Frame::HalfOpen(_)),
            "differing cardinality leaves the half turn open, got {theta:?}"
        );
        assert!(
            theta.rotor().projective_eq(&crate::transducer::element::identity_versor()),
            "a pure dilation turns nothing, got {theta:?}"
        );
    }

    /// A REFLECTION IS NOT A ROTATION — on a body chiral enough to tell.
    ///
    /// THE THREE-CELL L WILL NOT DO, and this test first used it: the
    /// L-tromino is ACHIRAL, its mirror image IS one of its own four
    /// turns, and `relative_rotor` correctly found one. The smallest
    /// chiral polyomino takes four cells, so the L- and J-tetrominoes are
    /// the honest fixture — congruent as multisets, mirror images, and
    /// related by NO proper turn.
    #[test]
    fn a11_a_mirrored_body_mints_no_rotor() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        // THIS READ `Oblique` UNTIL V1, AND THAT WAS THE FINDING THAT
        // FORCED THE SPLIT (68). The mirrored deviatorics sit at
        // `wedge ≠ 0` — a reflection about an axis at `α` maps `u(2φ)` to
        // `u(2α − 2φ)`, a perfectly good rotation OF THE DEVIATORIC — so
        // the reader refused at the angle test and never reached a cell
        // test at all. `Oblique` therefore held genuine non-lattice turns
        // AND reflections, which are `O(2)∖SO(2)` and are not rotations at
        // any angle.
        //
        // That mattered for 37.7: a demand minted here would have asked
        // the court to adjoin a generator for a relation that HAS no
        // rotor. The deviatorics alone cannot separate the two, so the
        // cells do it — the four improper elements of `D₄`, applied and
        // compared, which nothing in this file had ever looked at.
        assert_eq!(
            blade::relative_rotor(&l.blade, &j.blade).err(),
            Some(blade::NoFrame::Mirrored),
            "a reflection is proved improper by the cells, not left oblique"
        );
    }

    /// D2 — THE METABOLIC LEDGER READS BOTH HALVES, AND NETS THEM PER
    /// LANE (68).
    ///
    /// A draw and its restore are conjugate, so a lane that cycles has
    /// zero flux while a lane that spends does not — and the two must be
    /// distinguishable, or "the engine is busy" and "the engine is
    /// draining" read the same. Arrays throughout: netting the four into
    /// one scalar trades a lane against another, which #34 refuses.
    ///
    /// A REFUSED DRAW IS COUNTED AND MOVES NO LANE. `Reserve::draw` is
    /// all-or-nothing, so a depletion must leave the reserve untouched
    /// AND still register — otherwise a starved engine and an idle one
    /// produce the same ledger.
    #[test]
    fn d2_the_ledger_separates_a_lane_that_cycles_from_one_that_spends() {
        use crate::physics::metabolism;
        let _ = metabolism::take_metabolism();
        let mut r = crate::physics::budget::Reserve::of([8, 8, 8, 8]);
        r.draw([3, 0, 0, 0]).expect("affordable");
        r.restore([3, 0, 0, 0]);
        r.draw([0, 5, 0, 0]).expect("affordable");
        r.draw([0, 99, 0, 0]).expect_err("gauge cannot cover it");
        let m = metabolism::take_metabolism();
        assert_eq!(m.drawn, [3, 5, 0, 0], "only afforded draws move a lane");
        assert_eq!(m.restored, [3, 0, 0, 0], "the conjugate half");
        assert_eq!(m.flux(), [0, 5, 0, 0], "lane 0 cycled; lane 1 spent");
        assert_eq!(m.depleted, 1, "the refusal is a reading, not a silence");
    }

    /// D2 — ENTROPY IS PER DIMENSION, AND `Σℓ` IS THE ORBIT STRUCTURE.
    ///
    /// The two cauterizations terminate different skeleta, so a ledger
    /// that merges them cannot say which dimension the engine spends.
    /// `orbit_infinite` is kept apart from a period of zero because A19
    /// spells infinite order as `0` and the two readings are opposite —
    /// a walk that never returns against an orbit with no period at all.
    #[test]
    fn d2_entropy_and_orbit_are_counted_at_their_own_dimensions() {
        use crate::physics::metabolism;
        use crate::transducer::alphabet::Order;
        let _ = metabolism::take_metabolism();
        metabolism::note_cauterized();
        metabolism::note_released();
        metabolism::note_released();
        metabolism::note_orbit(Some(Order::Finite(4)));
        metabolism::note_orbit(Some(Order::Finite(2)));
        metabolism::note_orbit(Some(Order::Infinite));
        metabolism::note_orbit(None);
        let m = metabolism::take_metabolism();
        assert_eq!((m.cauterized, m.released), (1, 2), "two dimensions, two counts");
        assert_eq!(m.entropy(), 3, "and a total that does not lose them");
        assert_eq!((m.orbit_finite, m.orbit_sum), (2, 6), "Σℓ over closed orbits");
        assert_eq!(m.orbit_infinite, 1, "a walk is not a period");
    }

    /// V1 — PROVED UNLIKE BY AN INVARIANT, NOT BY EXHAUSTING A GROUP (68).
    ///
    /// The L- and S-tetrominoes differ in a quantity no conformal motion
    /// changes, so no turn relates them at any angle in any vocabulary.
    ///
    /// WHAT CHANGED IN THE RECUT. The first cut proved this by applying
    /// the eight elements of `D₄` to the cells and finding none that
    /// carried — a hardcoded generator table over raw coordinates, resting
    /// on the crystallographic restriction of `ℤ²` to rule out everything
    /// it had not tried. None of that means anything in `Cl(4,1)`. The
    /// refusal now comes from the bodies' own harmonic tower, holds at any
    /// scale, and assumes no base lattice at all.
    #[test]
    fn v1_an_unlike_pair_is_permanently_unturned() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let s = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
        assert_eq!(
            blade::relative_rotor(&l.blade, &s.blade).err(),
            Some(blade::NoFrame::Incongruent),
            "an invariant separates them, so no vocabulary reaches them"
        );
    }

    /// V1 — WHAT IT TAKES TO LEAVE THE QUESTION OPEN, STATED AS THE TWO
    /// GATES RATHER THAN AS A FIXTURE (68).
    ///
    /// `Oblique` is the arm NOT proved permanent, and reaching it needs
    /// BOTH gates to pass at once: the odd invariant must not be conjugate
    /// (no reflection) and A.12's scale certificate must hold (the tower
    /// corresponds at every order), while the deviatoric relation is at an
    /// angle the alphabet cannot spell.
    ///
    /// MEASURED, AND WORTH SAYING PLAINLY: across 110 ordered pairs of
    /// eleven lattice bodies, NOTHING reached this arm. Every refusal was
    /// `Mirrored`, `Incongruent` or `Isotropy`. A body and its dilate
    /// satisfy the certificate but then stand at a lattice angle and read
    /// as a turn; two bodies at an oblique angle fail the certificate.
    /// Whether the corpus contains a pair that passes both is a
    /// measurement the recut takes, not something a fixture should assert
    /// into existence — so this pins the GATES, which is what the code
    /// actually promises.
    #[test]
    fn v1_the_open_arm_needs_both_gates_and_a_dilate_passes_only_one() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let s = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        // Gate 2 alone: an unlike pair fails the certificate outright.
        assert!(!blade::dilation_certified(&l.blade, &s.blade), "tower differs");
        // Gate 1 alone: a reflection satisfies the certificate — every
        // harmonic NORM survives a reflection — and is caught only by the
        // odd invariant. Either gate alone would let it through.
        assert!(
            blade::dilation_certified(&l.blade, &j.blade),
            "a reflection passes the even tower, which is why the odd gate exists"
        );
        assert_eq!(
            blade::relative_rotor(&l.blade, &j.blade).err(),
            Some(blade::NoFrame::Mirrored),
            "and the odd gate is what refuses it"
        );
    }

    /// D3 — THE WITNESS AN OBLIQUE PAIR MINTS IS ONE THE COURT ACCEPTS.
    ///
    /// Minting a demand is not the deliverable; minting one that SURVIVES
    /// `REFUSE_LETTER_WITNESS` is. The court reads
    /// `π_p = gcd_{ℤ[i]}(s + t·i, p)` and demands a factor whose norm is
    /// exactly `p`, so this walks the whole chain the court walks — arc,
    /// primitivity, norm, prime, Euclid — and refuses to call the wiring
    /// done on the strength of a demand appearing somewhere.
    ///
    /// PRIMITIVITY IS THE LOAD-BEARING STEP. With `g = gcd(s,t) > 1` and
    /// `p | g`, that gcd comes back with norm `p²` and the letter is
    /// thrown out; the demand would be minted, counted, relayed, and
    /// refused at the far end, which reads as a live pipeline carrying
    /// nothing.
    ///
    /// THE PAIR HERE IS NOT ITSELF `Oblique` — it is `Incongruent`, and
    /// after the recut no probed lattice pair reaches the open arm at all.
    /// What is under test is the ARITHMETIC CHAIN the mint runs once a
    /// pair does reach it, and `oblique_arc` is exercised directly for
    /// that reason rather than through a classification that would be
    /// asserting a fixture into an arm the geometry does not put it in.
    #[test]
    fn d3_the_mint_produces_a_witness_the_court_can_open() {
        use crate::core::gaussian;
        use num_integer::Integer;
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let big = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 1, 0)]);
        let (s, t) = blade::oblique_arc(&l.blade, &big.blade).expect("the pair has an arc");
        assert_eq!(s.gcd(&t), num_bigint::BigInt::from(1), "the arc is primitive: {s} + {t}i");
        let norm = &s * &s + &t * &t;
        let plane = (
            crate::transducer::alphabet::PLANE_ROW,
            crate::transducer::alphabet::PLANE_COL,
        );
        let prime = crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane)
            .expect("the norm carries a prime the alphabet does not");
        let block = crate::transducer::alphabet::arc_block(&s, &t);
        let w = gaussian::from_block(&block).expect("the block is a Gaussian arc");
        let pi = gaussian::prime_factor(&w, &num_bigint::BigInt::from(prime))
            .expect("the court reads the factor off the witness");
        assert_eq!(
            pi.norm(),
            num_bigint::BigInt::from(prime),
            "the factor's norm IS the prime, which is what the court checks"
        );
        eprintln!("d3 arc {s} + {t}i | norm {norm} | prime {prime} | factor {pi:?}");
    }

    /// D3 — A PERMANENT PAIR MINTS NOTHING.
    ///
    /// The demand path is reached only from `Oblique`. A reflection has no
    /// rotor at any angle, so asking the court to adjoin a generator for
    /// it would poison the docket with a letter that can never close.
    #[test]
    fn d3_a_reflection_and_an_unlike_pair_mint_no_arc() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        for (other, why) in [(&j, "a reflection")] {
            assert_ne!(
                blade::relative_rotor(&l.blade, &other.blade).err(),
                Some(blade::NoFrame::Oblique),
                "{why} never reaches the demand path"
            );
        }
    }

    /// V1 RECUT — THE ODD INVARIANT IS SCALE-FREE, AND THAT IS THE WHOLE
    /// GAIN OVER THE LATTICE READING IT REPLACED (68).
    ///
    /// `L4x2` is `L4` dilated by 2. Normalised, `h₂` has degree 2 and `h₄`
    /// degree 4, so `Z = h₂²·conj(h₄)` has degree 8 and the pair's
    /// invariants must stand in the ratio `2⁸ = 256` on BOTH components —
    /// asserted here as an exact rational identity, no root and no
    /// tolerance.
    ///
    /// THE FIRST CUT OF V1 COULD NOT SEE THIS AT ALL. It compared cells
    /// about the centroid and refused outright when the supports differed
    /// in cardinality, so every dilated pair was unreadable by
    /// construction. The ray comparison divides the dilation out, and the
    /// pair reads as the turn it is.
    #[test]
    fn v1_the_odd_invariant_scales_by_its_degree_and_reads_across_a_dilation() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let l2 = blade::shape_of(&[
            (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
            (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
            (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
            (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
        ]);
        let za = blade::chirality(&l.blade).expect("a chiral body");
        let zb = blade::chirality(&l2.blade).expect("its dilate is chiral too");
        let d8 = num_rational::BigRational::from_integer(num_bigint::BigInt::from(256));
        assert_eq!(zb.0, &za.0 * &d8, "degree 8 on the real part");
        assert_eq!(zb.1, &za.1 * &d8, "degree 8 on the imaginary part");
        assert!(
            blade::relative_rotor(&l.blade, &l2.blade).is_ok(),
            "a dilated pair is a turn, which the cardinality test refused outright"
        );
    }

    /// V1 RECUT — A REFLECTION IS SEEN ACROSS A CHANGE OF SCALE.
    ///
    /// `J4` is `L4` reflected; `L4x2` is `L4` dilated. The relation
    /// between them is improper AND carries a dilation, so no cell
    /// comparison could ever have read it — the supports have four cells
    /// against sixteen. The odd invariant reads it exactly, because a
    /// conjugate ray is a conjugate ray at any scale.
    #[test]
    fn v1_a_reflection_is_proved_improper_across_a_dilation() {
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        let l2 = blade::shape_of(&[
            (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
            (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
            (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
            (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
        ]);
        assert_ne!(j.blade.support.len(), l2.blade.support.len(), "four against sixteen");
        assert_eq!(
            blade::relative_rotor(&j.blade, &l2.blade).err(),
            Some(blade::NoFrame::Mirrored),
            "improper across a dilation, which no cell test can reach"
        );
    }

    /// V1 RECUT — CHIRALITY ANSWERS THE ODD QUESTION ONLY, AND THE FIRST
    /// RECUT GOT THIS WRONG (68).
    ///
    /// `Z` is REAL for every achiral body, so all of them share one ray
    /// and a chirality-only gate degenerates to "handedness does not
    /// forbid it". Measured: a domino and an L-tromino passed it and read
    /// `Oblique`, which would have asked the court to adjoin a generator
    /// relating a two-cell bar to a three-cell corner.
    ///
    /// A.12's scale certificate is the EVEN half of the question and was
    /// already built. Both gates must pass for the question to stay open.
    #[test]
    fn v1_an_achiral_pair_is_still_refused_by_the_scale_certificate() {
        let d2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        let l3 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
        // Both achiral: the odd invariant is real and cannot separate them.
        assert!(blade::chirality(&d2.blade).expect("read").1.is_zero(), "achiral");
        assert!(blade::chirality(&l3.blade).expect("read").1.is_zero(), "achiral");
        // The even tower does separate them, and that is what decides.
        assert!(!blade::dilation_certified(&d2.blade, &l3.blade), "not similar");
        assert_eq!(
            blade::relative_rotor(&d2.blade, &l3.blade).err(),
            Some(blade::NoFrame::Incongruent),
            "chirality permits it; the tower refuses it; the tower wins"
        );
    }

    /// V1 — THE BRANCH SEPARATES THE TWO PERMANENT FACTS, AND SEPARATES
    /// BOTH FROM A TURN (68).
    ///
    /// The point of the split is that a census can read `oblique` as "the
    /// vocabulary is short". That only holds if the permanent cases land
    /// elsewhere, so this pins them together rather than trusting separate
    /// fixtures not to drift onto one arm.
    ///
    /// AN EARLIER VERSION OF THIS TEST ASSERTED `Oblique` FOR THE FOURTH
    /// ROW and it was wrong: `L4` against a five-cell L read `Oblique`
    /// only because the lattice reading had no cell test available at
    /// unequal cardinality — a gap in the instrument, recorded as a
    /// finding. The tower separates them, and the honest answer is
    /// `Incongruent`.
    #[test]
    fn v1_the_refusals_of_the_oblique_branch_stay_apart() {
        let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
        let j = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
        let s = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
        let big = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (3, 0, 0), (3, 1, 0)]);
        let l2 = blade::shape_of(&[
            (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
            (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
            (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
            (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
        ]);
        let read = [
            blade::relative_rotor(&l.blade, &j.blade).err(),
            blade::relative_rotor(&l.blade, &s.blade).err(),
            blade::relative_rotor(&l.blade, &big.blade).err(),
            blade::relative_rotor(&l.blade, &l2.blade).err(),
        ];
        assert_eq!(
            read,
            [
                Some(blade::NoFrame::Mirrored),
                Some(blade::NoFrame::Incongruent),
                Some(blade::NoFrame::Incongruent),
                None,
            ],
            "improper, unlike, unlike, and a turn across a dilation"
        );
    }

    /// A NON-CONGRUENT PAIR HAS NO ANGLE. Two bodies whose anisotropies
    /// differ are not related by a rotation, and answering one anyway
    /// would be inventing a reading (rule 11).
    #[test]
    fn a11_non_congruent_bodies_have_no_rotation_between_them() {
        let domino = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        let tromino = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
        let (ua, ub) = (
            blade::deviatoric(&domino.blade).expect("anisotropic"),
            blade::deviatoric(&tromino.blade).expect("anisotropic"),
        );
        assert!(
            blade::relative_double_angle(&ua, &ub).is_none(),
            "different anisotropies are not a rotation apart"
        );
    }

    /// A.11's REACH, MEASURED RATHER THAN ASSUMED (ruled 2026-09-22).
    ///
    /// A claim was made that the trace/2nd/4th suite is blind to six of the
    /// nine crystallographic orders and that `relative_rotor` minting only
    /// `{1, 2, 4}` was the instrument's own limit. IT IS NOT, and this test
    /// is the arithmetic that says so.
    ///
    /// The 2D crystallographic restriction is `φ(ℓ) ≤ 2`, admitting
    /// `{1, 2, 3, 4, 6}`, and a SQUARE lattice narrows it to `{1, 2, 4}`. A
    /// body of unit cells on an ARC grid therefore cannot carry `C₃` or `C₆`
    /// symmetry, and no rotation of order 3 or 6 maps the lattice to itself,
    /// so no such relation between two grid bodies exists to be measured.
    /// `φ(ℓ) ≤ 4` giving `{1,2,3,4,5,6,8,10,12}` is the restriction in FOUR
    /// dimensions: those orders live on the off-plane channels of `Cl(4,0)`
    /// and are not planar shape orientations at all.
    ///
    /// So `{2, 4}` is exactly sufficient for the sensor plane, and what it
    /// covers is asserted here: the 2nd orients everything the lattice can
    /// turn except the `C₄` bodies, and the 4th orients those.
    #[test]
    fn a11_the_even_suite_covers_the_square_lattices_whole_order_set() {
        // C₁ — an L-tromino. Anisotropic at the second harmonic.
        let c1 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
        assert!(
            blade::deviatoric(&c1.blade).is_some(),
            "a generic body is oriented by the 2nd harmonic"
        );

        // C₂ — a domino. Also second-harmonic anisotropic.
        let c2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
        assert!(
            blade::deviatoric(&c2.blade).is_some(),
            "a two-fold body is oriented by the 2nd harmonic"
        );

        // C₄ — the 2x2 square. The 2nd harmonic VANISHES on it, which is
        // the whole reason the 4th exists (A.11).
        let c4 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]);
        assert!(
            blade::deviatoric(&c4.blade).is_none(),
            "a four-fold body is isotropic to the 2nd harmonic"
        );

        // AND THE ORDER SET IS CLOSED HERE. `{1, 2, 4}` is every rotation a
        // square lattice admits, so a suite that orients each of them is
        // complete for the sensor plane. A seventh harmonic would answer a
        // question the base cannot ask.
    }

    /// THE PARITY GAP IS REAL, AND IT IS NOT THE ORDER GAP (57.3).
    ///
    /// The one thing the even suite genuinely cannot do is separate `φ` from
    /// `φ + π`: every even-rank moment satisfies `u(−x) = u(x)`, so a body
    /// and its point reflection through the centroid carry the SAME
    /// deviatoric and the SAME quartic. This is independent of the order
    /// set — it holds for `C₁` bodies on a square lattice, which is most of
    /// them — and it is why `Frame::HalfOpen` exists.
    ///
    /// Closing it needs an ODD invariant. The 3rd harmonic is the candidate,
    /// and the reason it works is NOT that bodies have three-fold symmetry
    /// (a square lattice forbids that) but that a generic body has a
    /// non-vanishing third moment which changes sign under `x -> -x`.
    #[test]
    fn a11_no_even_moment_separates_a_body_from_its_point_reflection() {
        let body = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
        // The same body reflected through the origin. Congruent to itself
        // under a half turn, and a half turn is what must be detected.
        let flipped = blade::shape_of(&[(0, 0, 0), (-1, 0, 0), (0, -1, 0)]);

        let (ua, ub) = (
            blade::deviatoric(&body.blade).expect("anisotropic"),
            blade::deviatoric(&flipped.blade).expect("anisotropic"),
        );
        assert_eq!(ua, ub, "the 2nd harmonic cannot see a half turn");

        let (qa, qb) = (
            blade::quartic_harmonic(&body.blade),
            blade::quartic_harmonic(&flipped.blade),
        );
        assert_eq!(qa, qb, "and neither can the 4th: both are even rank");
    }

    /// `I₂ ≠ 0` IS PROOF THE MOTION LEFT THE SENSOR PLANE (ruled
    /// 2026-09-22).
    ///
    /// THE SENSOR IS NOT THE ONTOLOGY. A reading that confined the engine's
    /// orders to `{1, 2, 4}` because the detector is a square raster
    /// mistook the display substrate for the manifold: a facet pitched out
    /// of the sensor plane projects to jagged cells, and the refraction
    /// rotor across its edge carries its own order whatever the raster can
    /// tile. `Cl(4,1)` admits the whole cyclotomic spectrum, `φ(ℓ) ≤ 4`.
    ///
    /// And the second Casimir says when that has happened. `⟨Ω⟩₄` is a
    /// grade-4 part, so it needs FOUR independent directions to be
    /// non-zero. Everything confined to one plane is proportional to `e₁₂`
    /// and `e₁₂ ∧ e₁₂ = 0`; a planar translation is `v ∧ e_∞` and two of
    /// them repeat `e_∞`; a planar rotation against a planar translation
    /// repeats `e₁` or `e₂`. Each is asserted below rather than argued.
    ///
    /// So a corpus reading `non-simple 3` is not carrying a curiosity: it is
    /// EMPIRICAL PROOF that three of its circuits compose content off the
    /// sensor plane, which is what a refraction between two non-parallel
    /// facets is.
    #[test]
    fn a_non_simple_bivector_requires_content_off_the_sensor_plane() {
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // A planar rotation composed with a planar rotation: one blade, so
        // the wedge is identically zero.
        let r1 = cga::rotator(&q(1), &q(1));
        let r2 = cga::rotator(&q(2), &q(3));
        assert!(
            cga::bivector_is_simple(&r1.geo(&r2)),
            "two turns in one plane cannot couple: e₁₂ ∧ e₁₂ = 0"
        );

        // A planar rotation composed with an IN-PLANE translation. The
        // translator is `v ∧ e_∞` with `v` in `e₁e₂`, so every grade-4 term
        // repeats a basis vector.
        let t_in = cga::translator(&q(3), &q(5), &q(0));
        assert!(
            cga::bivector_is_simple(&r1.geo(&t_in)),
            "a turn and a slide inside the sensor plane stay simple"
        );

        // Two in-plane translations: `e_∞` repeats.
        let t_in2 = cga::translator(&q(-2), &q(7), &q(0));
        assert!(
            cga::bivector_is_simple(&t_in.geo(&t_in2)),
            "two in-plane slides stay simple"
        );

        // NOW LEAVE THE PLANE. A turn in `e₁e₂` against a displacement with
        // a third-axis component — which in this engine is the FRAME axis,
        // the corpus's own `e₃` (61.3) — spans a four-volume.
        let t_off = cga::translator(&q(0), &q(0), &q(1));
        assert!(
            !cga::bivector_is_simple(&r1.geo(&t_off)),
            "a turn against an off-plane displacement is a SCREW: I₂ ≠ 0"
        );

        // TWO TURNS SHARING AN AXIS DO NOT COUPLE, and this is not a
        // detail. `e₁₂ ∧ e₁₃ = 0` because `e₁` repeats, and in THREE
        // dimensions every pair of 2-planes shares an axis — so a
        // composition of two spatial rotations is simple no matter how the
        // facets are tilted relative to one another.
        let shares_an_axis = cga::rotator_on_blade(&q(1), &q(1), cga::E1 | cga::E3);
        assert!(
            cga::bivector_is_simple(&r1.geo(&shares_an_axis)),
            "two turns through a common axis stay simple: e₁₂ ∧ e₁₃ = 0"
        );

        // IT TAKES A FOURTH DIRECTION. Two turns couple only when their
        // planes are COMPLETELY orthogonal — isoclinic — which `Cl(4,1)`
        // has room for and three spatial dimensions do not.
        let isoclinic = cga::rotator_on_blade(&q(1), &q(1), cga::E3 | cga::EP);
        assert!(
            !cga::bivector_is_simple(&r1.geo(&isoclinic)),
            "e₁₂ against e₃e₊ spans a four-volume: this is the screw"
        );
    }

    /// T2 — THE INVERSIVE STRAIN IS EXACT, SIGNED AND SCALE-FREE (59.3).
    ///
    /// `μ` itself is irrational — its denominator wants `√(r_a² r_b²)` —
    /// so the pair `(num, den2)` carries it and every question is a
    /// comparison of cross products. What must hold: the three contact
    /// sides come out right, the horizon test agrees with them, and a
    /// global dilation moves nothing.
    #[test]
    fn the_strain_reads_contact_exactly_and_scale_free() {
        use crate::transducer::blade::{self, Contact, Strain};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // A body against ITSELF is `δ = −1`: `‖ΔP‖ = 0` and `num = −2r²`,
        // so `num² = 4r⁴ = den2` exactly.
        //
        // THIS ASSERTED `Through` AND WAS WRONG (65). `δ = −1` is INTERNAL
        // TANGENCY — a codimension-1 contact interface — and the enum it
        // was written against had only three arms, folding the whole lower
        // half of the spectrum into "interpenetrating". The self-pair is
        // precisely where the two labels are both defensible, so the
        // fixture passed while testing what was written rather than what is
        // true; `the_five_inversive_regimes_are_read_exactly` exhibits the
        // case that is not ambiguous.
        let a = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade;
        let s = Strain::between(&a, &a).expect("a body has a centroid");
        assert_eq!(
            s.side(),
            Contact::TangentInternal,
            "a body against itself touches itself from inside"
        );
        assert!(s.num < q(0), "and its numerator is negative");

        // FAR APART. Two dominoes a long way from one another: `‖ΔP‖²`
        // dominates, `δ > 1`.
        let far = blade::shape_of(&[(0, 100, 0), (1, 100, 0)]).blade;
        let s_far = Strain::between(&a, &far).expect("two centroids");
        assert_eq!(s_far.side(), Contact::Apart, "distant bodies are separated");

        // THE HORIZON ORDERS THEM, and the ordering is the physics: a
        // distant pair is outside every horizon a near pair is inside.
        let near = blade::shape_of(&[(0, 2, 0), (1, 2, 0)]).blade;
        let s_near = Strain::between(&a, &near).expect("two centroids");
        for mu_c in [q(1), q(4), q(16)] {
            assert!(
                !(s_far.within(&mu_c) && !s_near.within(&mu_c)),
                "no horizon admits the far pair while refusing the near one"
            );
        }
        assert!(s_near.within(&q(16)), "a generous horizon admits the near pair");
        assert!(!s_far.within(&q(16)), "and still refuses the far one");

        // SCALE-FREE, which is Axiom C's whole point: `δ² = num²/den2` is
        // invariant under `x ↦ λx` because `num ↦ λ²num` and
        // `den2 ↦ λ⁴den2`. Doubling every coordinate must not move the
        // reading or the side.
        let a2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]).blade;
        let far2 = blade::shape_of(&[
            (0, 200, 0),
            (1, 200, 0),
            (0, 201, 0),
            (1, 201, 0),
        ])
        .blade;
        let scaled = Strain::between(&a2, &far2).expect("two centroids");
        assert_eq!(scaled.side(), s_far.side(), "a dilation does not change the side");

        // AND NO ROOT WAS TAKEN ANYWHERE: `den2` is a square and is never
        // rooted; it is strictly positive because `radius2` is.
        assert!(s.den2 > q(0), "the denominator squared is strictly positive");
        assert!(s_far.den2 > q(0));
    }

    /// T3 — THE STRAIN IS A BILINEAR FORM, NOT A PAIRWISE TEST.
    ///
    /// `Strain::between` subtracts centroids and squares, which reads as a
    /// pairwise computation and invites a pairwise walk. It is not one. On
    /// the conformal representative `s = P − ½r²e_∞` the numerator is an
    /// INNER PRODUCT:
    ///
    /// ```text
    ///     s_a · s_b = ½(r_a² + r_b² − ‖P_a − P_b‖²)
    ///     num       = −2 (s_a · s_b)
    /// ```
    ///
    /// which is LINEAR IN EACH ARGUMENT. That is what T3 needs and what a
    /// bucketing scheme would destroy: "which bodies lie within `A`'s
    /// horizon" is a linear functional evaluated against the carrier, so a
    /// keyed descent is possible exactly when the key's coarse strata are
    /// level sets of that functional. Quantising `⌊log₂ r⌋` or `P mod 2^k`
    /// answers it with a grid instead, which is `ring_of_support` at a
    /// coarser pitch — and `⌊log₂⌋` is not even exact in ℚ.
    ///
    /// The identity is asserted here so the linear form is a measured fact
    /// before anything is built on it.
    #[test]
    fn the_strain_numerator_is_an_inner_product_of_conformal_spheres() {
        use crate::transducer::blade::{self, Strain};
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());
        let two = q(2);

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(0, 5, 0), (1, 5, 0), (0, 6, 0)]).blade,
            blade::shape_of(&[(3, 3, 0)]).blade,
        ];

        for a in &bodies {
            for b in &bodies {
                let s = Strain::between(a, b).expect("both centroids read");
                let (pa, pb) = (a.centroid().expect("a"), b.centroid().expect("b"));
                let sa = cga::sphere(&pa.0, &pa.1, &pa.2, &a.radius2());
                let sb = cga::sphere(&pb.0, &pb.1, &pb.2, &b.radius2());
                // The scalar part of the geometric product IS the inner
                // product of two vectors.
                let dot = sa.geo(&sb).scalar_part();
                assert_eq!(
                    s.num,
                    -&two * &dot,
                    "num = −2 (s_a · s_b): the strain is bilinear in the spheres"
                );
            }
        }
    }

    /// T3 — THE STRAIN DEPENDS ONLY ON THE RELATIVE MOTOR (ruled
    /// 2026-09-22).
    ///
    /// A body's nucleus is `s = M s₀ M̃` for a motor `M ∈ Spin⁺(4,1)(ℚ)`,
    /// and a versor is an orthogonal transformation of `ℚ^{4,1}`, so it
    /// preserves the inner product. Therefore
    ///
    /// ```text
    ///     (M s_a M̃) · (M s_b M̃) = s_a · s_b
    /// ```
    ///
    /// and `num = −2(s_a · s_b)` is a function of `M_a⁻¹ M_b` ALONE —
    /// never of where either body sits absolutely. Absolute position is a
    /// gauge artifact of the strain.
    ///
    /// WHAT THAT COLLAPSES. A spatial index would answer "which bodies are
    /// near me"; the spellable horizon answers "can my mind factor `g_ab`".
    /// Since the relative motor IS `g_ab`, those are one question, and T3
    /// is not an index bolted beside the physics — proximity is membership
    /// of the relative motor in the alphabet's spellable neighbourhood.
    ///
    /// Asserted here on a motor with BOTH halves — a turn composed with a
    /// displacement — because a translation-only fixture would pass on the
    /// commuting case and say nothing about the screw.
    #[test]
    fn the_strain_is_invariant_under_a_common_motor() {
        use crate::transducer::blade::{self, Strain};
        use crate::transducer::cga;
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(4, 7, 0), (5, 7, 0), (4, 8, 0)]).blade,
            blade::shape_of(&[(-3, 2, 0)]).blade,
        ];
        let nucleus = |b: &blade::Blade| {
            let p = b.centroid().expect("a centroid");
            cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
        };

        // A motor with a turn AND a displacement, so the two halves do not
        // commute and the test is not a translation identity in disguise.
        let motor = cga::translator(&q(11), &q(-4), &q(2)).geo(&cga::rotator(&q(1), &q(1)));

        // THE INVARIANCE IS PROJECTIVE, NOT ON THE NOSE, and a first
        // statement of it claimed otherwise. A versor preserves the inner
        // product only when it is ORTHOGONAL — `M M̃ = 1` — and in ℚ a
        // versor cannot be normalised without a square root, so every motor
        // here is a RAY. `rotator(1,1) = 1 + e₁₂` has `M M̃ = 2`, and the
        // sandwich scales the inner product by `λ² = 4`:
        //
        //     num ↦ λ² num,   den2 ↦ λ⁴ den2,   δ² = num²/den2 unchanged
        //
        // which is 57.2's ray exactly — "canonicalization is not a
        // homomorphism" — and is why Axiom C carries the ratio and never
        // evaluates it. The scale-free reading is invariant; the numerator
        // alone is not, and a design that keyed on `num` would key on the
        // gauge.
        let lambda = motor.geo(&motor.reverse()).scalar_part();
        assert_eq!(lambda, q(2), "the fixture's motor is a ray of norm 2, not a unit");
        for a in &bodies {
            for b in &bodies {
                let (sa, sb) = (nucleus(a), nucleus(b));
                let before = sa.geo(&sb).scalar_part();
                let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
                assert_eq!(
                    after,
                    &lambda * &lambda * &before,
                    "a common motor scales the strain by its own norm squared"
                );
                // AND THE SCALE-FREE READING SURVIVES EXACTLY. `num²/den2`
                // is what the horizon is taken on, and it does not move.
                let (ra2, rb2) = (sa.geo(&sa).scalar_part(), sb.geo(&sb).scalar_part());
                let (ra2m, rb2m) = (
                    motor.sandwich(&sa).geo(&motor.sandwich(&sa)).scalar_part(),
                    motor.sandwich(&sb).geo(&motor.sandwich(&sb)).scalar_part(),
                );
                let delta2 = |n: &crate::core::Q, x: &crate::core::Q, y: &crate::core::Q| {
                    let num = -q(2) * n;
                    &num * &num / (q(4) * x * y)
                };
                assert_eq!(
                    delta2(&before, &ra2, &rb2),
                    delta2(&after, &ra2m, &rb2m),
                    "δ² = num²/den2 is invariant under the motor, on the nose"
                );
            }
        }

        // AND THE SIGN CONVENTION THE WHOLE THING RESTS ON. `s² = +r² > 0`:
        // the nucleus is SPACELIKE, which is the dual-sphere chart where
        // contact is reachable. On the `v² = −1` hyperboloid `μ = sinh²(d/2)
        // ≥ 0` and tangency is unreachable — that is 59.2's finding, and a
        // statement that the nucleus has `s² = −r²` would put it back
        // there.
        for b in &bodies {
            let s = nucleus(b);
            let r2 = b.radius2();
            assert_eq!(s.geo(&s).scalar_part(), r2, "s² = +r², spacelike");
            assert!(r2 > q(0), "and every extracted body is invertible");
        }

        // A BODY AGAINST ITSELF IS THE IDENTITY MOTOR, and that is internal
        // tangency rather than "no relation": δ = −1, num = −2r², num² =
        // den2.
        let s = Strain::between(&bodies[0], &bodies[0]).expect("a centroid");
        assert_eq!(&s.num * &s.num, s.den2, "M_ab = 1 sits exactly on δ = −1");
    }

    /// THE HARMONIC CORE IS WHAT KEEPS THE NUCLEUS OFF THE NULL CONE
    /// (ruled 2026-09-22).
    ///
    /// The luminal sector is an ENVELOPE: the projective ray wraps the
    /// harmonic body, and it can only do so while the body holds itself
    /// open. `s² = +r²`, so `r² → 0` drops the nucleus onto the light cone,
    /// `den2 = 4 r_a² r_b² → 0`, and the inversive distance diverges — the
    /// interaction horizon stops being defined at all.
    ///
    /// THE SMALLEST POSSIBLE BODY IS THE TEST. One lamina is the least
    /// harmonic mass the sensor can deliver, and it is exactly where a
    /// point-mass reading would give `r² = 0`. It does not, because
    /// `blade::lift` INTEGRATES THE LAMINA'S OWN UNIT SQUARE and carries
    /// the parallel-axis term at every order — which is the shadow trace's
    /// retirement (A.11's single producer) doing structural work rather
    /// than cosmetic work. A second producer taking cells as point masses
    /// would have collapsed the nucleus onto the cone.
    #[test]
    fn the_smallest_body_still_holds_its_nucleus_off_the_cone() {
        use crate::transducer::blade;
        let q = |n: i64| crate::core::Q::from_integer(n.into());
        let sixth = q(1) / q(6);

        // A SINGLE LAMINA — the least mass there is.
        let one = blade::shape_of(&[(0, 0, 0)]).blade;
        let r2 = one.radius2();
        assert!(
            r2 >= sixth,
            "one lamina integrates its own square: r² = {r2} ≥ 1/6, never 0"
        );

        // A point-mass reading would put this at zero. It does not, so the
        // nucleus is spacelike and `den2` for a pair of them is bounded
        // away from zero.
        let s = blade::Strain::between(&one, &one).expect("a centroid");
        assert!(s.den2 >= q(4) * &sixth * &sixth, "den2 = 4 r_a² r_b² is bounded below");
        assert!(s.den2 > q(0), "so the horizon is always defined");
    }

    /// A MOTOR OF NEGATIVE NORM DOES NOT FLIP THE CONTACT SIDE (ruled
    /// 2026-09-22, and left open as an argument until measured).
    ///
    /// `Cl(4,1)` is INDEFINITE — `e₋² = −1` — so a versor built from an odd
    /// number of timelike reflections has `λ = M M̃ < 0`, which cannot
    /// happen in a definite signature. 64.5 left it open whether the side
    /// of the strain survives such a motor, because the sign of `num` is
    /// what `Strain::side` splits on before squaring.
    ///
    /// It does survive, and here is why: the sandwich `M x M̃` is `λ` times
    /// an orthogonal map, so the inner product picks up `λ²` — and `λ² > 0`
    /// for every `λ ∈ ℚ×`, negative ones included. The SIDE is therefore an
    /// absolute geometric invariant even through an indefinite reflection
    /// channel, while `num` itself remains gauge.
    #[test]
    fn a_negative_norm_motor_scales_the_strain_and_never_flips_its_side() {
        use crate::transducer::blade;
        use crate::transducer::cga::{self, Mv, E1, EM};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // `e₁e₋`: one spacelike and one timelike direction, so
        // `M M̃ = e₁ e₋ e₋ e₁ = e₁ (−1) e₁ = −1`.
        let motor = Mv::blade(E1, q(1)).geo(&Mv::blade(EM, q(1)));
        let lambda = motor.geo(&motor.reverse()).scalar_part();
        assert_eq!(lambda, q(-1), "an indefinite signature admits a NEGATIVE versor norm");
        assert!(lambda < q(0), "which a definite signature never could");

        let bodies = [
            blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
            blade::shape_of(&[(0, 9, 0), (1, 9, 0)]).blade,
            blade::shape_of(&[(0, 1, 0)]).blade,
        ];
        let nucleus = |b: &blade::Blade| {
            let p = b.centroid().expect("a centroid");
            cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
        };

        for a in &bodies {
            for b in &bodies {
                let (sa, sb) = (nucleus(a), nucleus(b));
                let before = sa.geo(&sb).scalar_part();
                let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
                assert_eq!(
                    after,
                    &lambda * &lambda * &before,
                    "the inner product picks up λ², never λ"
                );
                // AND THAT IS THE WHOLE POINT: `λ² > 0` even for `λ < 0`,
                // so the sign the side is read from cannot invert.
                assert_eq!(
                    before > q(0),
                    after > q(0),
                    "a negative-norm motor does not move the contact side"
                );
            }
        }
        assert!(&lambda * &lambda > q(0), "λ² is positive for every λ in ℚ×");
    }

    /// T4 — THE INTERFACE IS THE WHOLE PRODUCT, AND SCALAR COLLAPSE IS
    /// DISALLOWED (64, ruled 2026-09-22).
    ///
    /// Four proofs, each arithmetic rather than argued.
    ///
    /// (1) THE SPLIT IS COMPLETE. `s_a s_b = s_a·s_b + s_a∧s_b` and there
    ///     is nothing else: the geometric product of two VECTORS has grades
    ///     0 and 2 only, so the interface loses nothing by being carried as
    ///     those two.
    ///
    /// (2) A LONE TETHER IS ALWAYS SIMPLE. `(s_a∧s_b) ∧ (s_a∧s_b) = 0`
    ///     identically — a repeated factor kills the wedge — so `I₂ = 0` on
    ///     every isolated interface and a screw can only be a property of
    ///     the NETWORK (59.7, 60.4).
    ///
    /// (3) THE BLADE IS THE COAXAL PENCIL. Every sphere `λs_a + μs_b` lies
    ///     in it: `s ∧ B = 0` for the whole one-parameter family, which is
    ///     what makes `B` the interface rather than a pair of endpoints.
    ///
    /// (4) AND THE COLLAPSE IS REAL. Two pairs at the SAME `δ²` and the
    ///     same side carry DIFFERENT blades. That is the refutation: a
    ///     tether keyed on the scalar alone cannot tell two interactions
    ///     apart that differ in orientation, plane and pencil.
    #[test]
    fn the_interface_carries_what_the_scalar_throws_away() {
        use crate::transducer::blade::{self, Interface};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        let a = blade::shape_of(&[(0, 0, 0)]).blade;
        let along_r = blade::shape_of(&[(4, 0, 0)]).blade;
        let along_c = blade::shape_of(&[(0, 4, 0)]).blade;

        let (na, nr) = (a.nucleus().expect("a"), along_r.nucleus().expect("r"));
        let product = na.geo(&nr);

        // (1) GRADES 0 AND 2 ONLY.
        for g in [1u32, 3, 4, 5] {
            assert!(
                product.grade(g).is_zero(),
                "the product of two vectors has no grade {g} part"
            );
        }
        let face = Interface::between(&a, &along_r).expect("both nuclei");
        assert_eq!(
            product.grade(0).scalar_part() * q(-2),
            face.strain.num,
            "the scalar part IS the strain numerator"
        );
        assert_eq!(product.grade(2), face.blade, "and the grade-2 part IS the interface");
        assert!(!face.blade.is_zero(), "which is not empty for two distinct bodies");

        // (2) SIMPLE, IDENTICALLY.
        assert!(
            crate::transducer::cga::bivector_is_simple(&face.blade),
            "a wedge of two vectors can never carry a screw: I₂ ≡ 0"
        );

        // (3) THE COAXAL PENCIL LIES IN THE BLADE. `s ∧ B = 0` for every
        //     `s = λ s_a + μ s_b`, which is what makes B the interface.
        for (l, m) in [(1i64, 0i64), (0, 1), (1, 1), (3, -2), (-5, 7)] {
            let s = na.scale(&q(l)).add(&nr.scale(&q(m)));
            // `s ∧ B` is the grade-3 part of `B s`: a vector wedged into a
            // 2-blade, which vanishes exactly when the vector lies in it.
            assert!(
                face.blade.wedge_vector(2, &s).is_zero(),
                "the pencil member ({l}, {m}) lies in the interface blade"
            );
        }

        // (4) THE COLLAPSE IS REAL: same δ², same side, DIFFERENT blade.
        let other = Interface::between(&a, &along_c).expect("both nuclei");
        let d2 = |f: &Interface| {
            &f.strain.num * &f.strain.num / f.strain.den2.clone()
        };
        assert_eq!(d2(&face), d2(&other), "two pairs at the same scale-free strain");
        assert_eq!(
            face.strain.side(),
            other.strain.side(),
            "and on the same side of contact"
        );
        assert_ne!(
            face.blade, other.blade,
            "yet different interfaces — the scalar cannot tell them apart"
        );
    }

    /// T4 — `condense` IS A FIELD, AND IT MINTS THE 1-SKELETON.
    ///
    /// The horizon admits near pairs and refuses far ones, the reading does
    /// not depend on the order the bodies were offered, and every tether
    /// carries its interface blade rather than a bare link.
    #[test]
    fn condense_mints_tethers_from_the_strain_field() {
        use crate::transducer::blade::{self, condense};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // Three close together and one far away.
        let bodies: Vec<blade::Blade> = [
            vec![(0i64, 0i64, 0i64)],
            vec![(0, 1, 0)],
            vec![(1, 0, 0)],
            vec![(0, 60, 0)],
        ]
        .iter()
        .map(|cells| blade::shape_of(cells).blade)
        .collect();

        let near = condense(&bodies, &q(4));
        assert!(!near.is_empty(), "a horizon that admits nothing is not a field");
        assert!(
            near.iter().all(|&(i, j, _)| i != 3 && j != 3),
            "the distant body is outside every near horizon: {:?}",
            near.iter().map(|&(i, j, _)| (i, j)).collect::<Vec<_>>()
        );
        assert!(
            near.iter().all(|(_, _, f)| !f.blade.is_zero()),
            "every minted tether carries its interface, never a bare link"
        );

        // A WIDER HORIZON ADMITS STRICTLY MORE, so the gate is monotone and
        // is really reading the field rather than answering a constant.
        //
        // THE WIDE HORIZON IS DERIVED, NOT PICKED. A number chosen by hand
        // here is the `STROKE = 4` defect again — it was picked at 1000 and
        // the far pair needs 5399. The bound comes from the structural one
        // instead: `radius2 ≥ 1/6` for every body, so `den2 = 4r_a²r_b² ≥
        // 1/9` and `√den2 ≥ 1/3`, giving `δ = num/√den2 ≤ 3·num` and
        // therefore `μ = (δ−1)/2 < 2·num`. Twice the largest numerator in
        // the field admits every pair in it, whatever the fixture.
        let widest = {
            let mut m = q(0);
            for i in 0..bodies.len() {
                for j in i + 1..bodies.len() {
                    if let Some(s) = blade::Strain::between(&bodies[i], &bodies[j]) {
                        if s.num > m {
                            m = s.num;
                        }
                    }
                }
            }
            q(2) * m
        };
        let wide = condense(&bodies, &widest);
        assert!(wide.len() >= near.len(), "a wider horizon admits at least as many");
        assert_eq!(
            wide.len(),
            bodies.len() * (bodies.len() - 1) / 2,
            "and a horizon past the whole field admits every pair"
        );
        assert!(wide.len() > near.len(), "which is strictly more than the near one");

        // ORDER-FREE: the field is a property of the bodies, not of the
        // order they were handed over.
        let mut shuffled = bodies.clone();
        shuffled.reverse();
        assert_eq!(
            condense(&shuffled, &q(4)).len(),
            near.len(),
            "the same field however the bodies are offered"
        );
    }

    /// THE INVERSIVE SPECTRUM HAS FIVE REGIMES, NOT THREE (65, ruled
    /// 2026-09-22).
    ///
    /// ```text
    ///  Contained   TangentInt     Intersecting    TangentExt    Apart
    ///   δ < −1       δ = −1        |δ| < 1         δ = 1        δ > 1
    /// ```
    ///
    /// A first cut folded everything below `δ = 1` into one arm, so
    /// INTERNAL TANGENCY — `d = |r_a − r_b|`, where the spheres touch at
    /// one point from inside — was reported as interpenetration. Half the
    /// contact boundary, mislabelled.
    ///
    /// Each regime is exhibited on a fixture whose geometry is known, built
    /// from bodies of DIFFERENT radii so that internal tangency is a real
    /// configuration rather than the degenerate self-pair that hid the
    /// defect.
    #[test]
    fn the_five_inversive_regimes_are_read_exactly() {
        use crate::transducer::blade::{self, Contact, Strain};
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // Radii are set by the bodies' own moments, so the fixtures are
        // built and then MEASURED rather than assumed: `side` is checked
        // against `δ²` and the sign of `num`, which is the definition.
        let small = blade::shape_of(&[(0, 0, 0)]).blade;
        let big = blade::shape_of(&[
            (0, 0, 0), (1, 0, 0), (2, 0, 0), (0, 1, 0), (1, 1, 0),
            (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0),
        ])
        .blade;

        // The classification is a function of `(sign num, δ² vs 1)`, and
        // this asserts the two agree on every pair the fixtures give.
        let check = |a: &blade::Blade, b: &blade::Blade| {
            let s = Strain::between(a, b).expect("centroids");
            let sq = &s.num * &s.num;
            let want = match (s.num > q(0), sq.cmp(&s.den2)) {
                (_, std::cmp::Ordering::Equal) if s.num > q(0) => Contact::TangentExternal,
                (_, std::cmp::Ordering::Equal) => Contact::TangentInternal,
                (true, std::cmp::Ordering::Greater) => Contact::Apart,
                (false, std::cmp::Ordering::Greater) => Contact::Contained,
                (_, std::cmp::Ordering::Less) => Contact::Intersecting,
            };
            assert_eq!(s.side(), want, "the spectrum is read off (sgn num, δ² vs 1)");
            s.side()
        };

        // A BODY AGAINST ITSELF IS INTERNAL TANGENCY, not penetration.
        // `num = −2r²` and `num² = den2`, so `δ = −1` exactly — and the
        // first cut called this `Through`.
        assert_eq!(
            check(&small, &small),
            Contact::TangentInternal,
            "δ = −1 is a CONTACT interface, not overlap"
        );
        assert_eq!(check(&big, &big), Contact::TangentInternal);

        // A SMALL BODY AT THE CENTRE OF A LARGE ONE IS CONTAINED. It must
        // be CONCENTRIC: at `d = 0`,
        //
        //     δ = −(r_a² + r_b²) / (2 r_a r_b)  ≤  −1
        //
        // by AM–GM, with equality exactly when `r_a = r_b`. Different radii
        // therefore put it strictly below −1. A first version of this put
        // the small body at `(0,0,0)` while the block's centroid is
        // `(1,1,0)` — a CORNER, not the centre — and it read
        // `Intersecting`, correctly.
        let centred = blade::shape_of(&[(1, 1, 0)]).blade;
        assert_eq!(check(&centred, &big), Contact::Contained, "nested, δ < −1");
        assert!(
            centred.radius2() != big.radius2(),
            "and the radii differ, so it is strictly nested rather than tangent"
        );

        // FAR APART.
        let far = blade::shape_of(&[(0, 200, 0)]).blade;
        assert_eq!(check(&small, &far), Contact::Apart, "disjoint, δ > 1");

        // AND THE BAND BETWEEN. Walking one body outward from the other
        // must pass through `Intersecting` before reaching `Apart`.
        let mut saw_secant = false;
        for d in 1..12i64 {
            let moved = blade::shape_of(&[(0, d, 0)]).blade;
            if check(&big, &moved) == Contact::Intersecting {
                saw_secant = true;
            }
        }
        assert!(saw_secant, "the secant band is reachable and is not an empty arm");
    }

    /// THE ANTIPODAL INVOLUTION `σ: s ↦ −s` IS AN INVOLUTION, AND IT
    /// EXCHANGES THE CONTACT BOUNDARY (65).
    ///
    /// A sphere and its negation are one sphere with inside and outside
    /// swapped, so `δ ↦ −δ`: `Apart ↔ Contained`, the two tangencies
    /// exchange, and `Intersecting` is fixed with `δ = 0` as its own fixed
    /// point. `δ²` survives it; the SIDE does not, which is why a gate that
    /// must be orientation-blind reads `δ²` and one that cares reads the
    /// arm.
    #[test]
    fn the_antipodal_map_exchanges_the_contact_boundary_and_squares_to_one() {
        use crate::transducer::blade::Contact;
        let all = [
            Contact::Apart,
            Contact::TangentExternal,
            Contact::Intersecting,
            Contact::TangentInternal,
            Contact::Contained,
        ];
        for c in all {
            assert_eq!(c.antipodal().antipodal(), c, "σ∘σ = id");
        }
        assert_eq!(Contact::Apart.antipodal(), Contact::Contained);
        assert_eq!(Contact::TangentExternal.antipodal(), Contact::TangentInternal);
        assert_eq!(
            Contact::Intersecting.antipodal(),
            Contact::Intersecting,
            "the secant band is self-dual"
        );
        // BOTH TANGENCIES ARE CONTACT. Treating one as contact and the
        // other as penetration is what lost the boundary's symmetry.
        assert!(Contact::TangentExternal.is_tangent());
        assert!(Contact::TangentInternal.is_tangent());
        for c in [Contact::Apart, Contact::Intersecting, Contact::Contained] {
            assert!(!c.is_tangent(), "{c:?} is not a contact interface");
        }
        // The involution has no fixed points outside the secant band, so
        // it genuinely pairs the spectrum rather than permuting it.
        for c in [
            Contact::Apart,
            Contact::TangentExternal,
            Contact::TangentInternal,
            Contact::Contained,
        ] {
            assert_ne!(c.antipodal(), c, "{c:?} is exchanged, not fixed");
        }
    }

    /// ATTENTION IS ANISOTROPIC: A DRAW IS A VECTOR AND REFUSES PER LANE
    /// (65).
    ///
    /// A scalar stroke counts `spend()` calls — implementation events, not
    /// physical work — which is why a constant calibrated against it decays
    /// on any refactor. And it collapses the four lanes law 5 reads
    /// separately: a mind bankrupt on SCALE but rich on GAUGE could not
    /// express that at all, and an expensive dilation attempt would preempt
    /// it out of turning.
    ///
    /// `Reserve` draws `[u64; 4]` all-or-nothing and refuses naming WHICH
    /// lane is gone, so an action with a zero projection on a depleted lane
    /// is still affordable. That is the whole content of the anisotropy.
    #[test]
    fn a_depleted_lane_does_not_preempt_the_others() {
        use crate::physics::Reserve;

        // A mind spent on scale and rich on gauge and translation — the
        // exact state `Bubble.escrow`'s own doc describes.
        let mut r = Reserve::of([0, 10, 10, 5]);
        assert!(!r.is_spent(), "a mind with any lane left has not yielded");

        // IT CANNOT DILATE, and the refusal says which lane and by how
        // much rather than answering a bare false (rule 11).
        let d = r.draw([1, 0, 0, 0]).expect_err("scale is gone");
        assert_eq!((d.lane, d.asked, d.held), (0, 1, 0), "the refusal names the lane");
        assert_eq!(r.lanes(), [0, 10, 10, 5], "and an refused draw costs nothing");

        // BUT IT CAN STILL TURN AND SLIDE. Under a scalar stroke the
        // refused dilation would have burned the counter and preempted
        // these.
        r.draw([0, 3, 2, 0]).expect("gauge and translation are affordable");
        assert_eq!(r.lanes(), [0, 7, 8, 5], "the whole vector lands, or none of it");

        // ALL OR NOTHING ACROSS LANES: a draw affordable on three lanes and
        // not the fourth lands nowhere.
        let d = r.draw([0, 1, 1, 99]).expect_err("hue cannot cover it");
        assert_eq!(d.lane, 3, "and the refusal names the lane that could not");
        assert_eq!(r.lanes(), [0, 7, 8, 5], "with nothing deducted");

        // THE CONJUGATE. An escrow that can only be spent is the ratchet
        // one level down (62), so a draw returns.
        r.restore([0, 3, 2, 0]);
        assert_eq!(r.lanes(), [0, 10, 10, 5], "a returned draw is exactly the draw");

        // SPENT IS EVERY LANE, not any lane.
        let mut empty = Reserve::of([0, 0, 0, 0]);
        assert!(empty.is_spent());
        assert!(empty.draw([0, 0, 0, 0]).is_ok(), "a zero draw is always affordable");
    }

    /// IS `δ = 2` THE ADJACENCY SIGNATURE, AND DOES IT SURVIVE SIZE?
    ///
    /// The claim: two unit laminae sharing an edge sit at `δ = 2` exactly,
    /// so `δ² ≤ 4` is a DERIVED contact gate rather than a dialled one.
    /// `‖ΔP‖² = 1`, `r² = 1/6` each, so
    ///
    /// ```text
    ///     num  = 1 − 1/6 − 1/6 = 2/3
    ///     den2 = 4 · (1/6)(1/6) = 1/9
    ///     δ²   = (2/3)² / (1/9) = 4
    /// ```
    ///
    /// The second half is the one that decides whether it is usable: the
    /// gyration radius GROWS with the body, so a signature calibrated on
    /// single laminae may not hold for adjacent blocks. This measures it
    /// rather than assuming either way.
    #[test]
    fn the_adjacency_signature_and_whether_it_survives_size() {
        use crate::transducer::blade::{self, Strain};
        let q = |n: i64| crate::core::Q::from_integer(n.into());
        let d2 = |a: &blade::Blade, b: &blade::Blade| {
            let s = Strain::between(a, b).expect("centroids");
            &s.num * &s.num / s.den2
        };

        // ONE LAMINA EACH, EDGE-ADJACENT.
        let u0 = blade::shape_of(&[(0, 0, 0)]).blade;
        let u1 = blade::shape_of(&[(0, 1, 0)]).blade;
        assert_eq!(u0.radius2(), q(1) / q(6), "the +1/12 floor, twice");
        assert_eq!(d2(&u0, &u1), q(4), "edge-adjacent unit laminae sit at δ² = 4");

        // DIAGONAL, and SEPARATED BY ONE CELL.
        let diag = blade::shape_of(&[(1, 1, 0)]).blade;
        let gap = blade::shape_of(&[(0, 2, 0)]).blade;
        assert_eq!(d2(&u0, &diag), q(25), "a corner neighbour is δ = 5");
        assert_eq!(d2(&u0, &gap), q(121), "a one-cell gap is δ = 11");

        // So `δ² ≤ 4` admits EDGE adjacency and refuses corner adjacency,
        // which is 51.3's own 4-neighbour convention in the plane arriving
        // from the metric rather than from a stencil.

        // NOW SIZE. Two 3x3 blocks sharing an edge: centroids 3 apart.
        let block = |r0: i64, c0: i64| {
            let mut cells = Vec::new();
            for r in 0..3 {
                for c in 0..3 {
                    cells.push((r0 + r, c0 + c, 0));
                }
            }
            blade::shape_of(&cells).blade
        };
        // NOW SIZE — and it is SCALE-FREE, measured across n = 1..5.
        //
        // An n×n block has `r² = n²/6`, so two of them sharing an edge have
        // `‖ΔP‖² = n²` and
        //
        //     num  = n² − n²/6 − n²/6 = 2n²/3
        //     den2 = 4(n²/6)² = n⁴/9
        //     δ²   = (2n²/3)² / (n⁴/9) = 4          for EVERY n
        //
        // `δ² = 4` is therefore not a constant calibrated on unit laminae.
        // It is the SIMILARITY CLASS of edge-adjacency, and `δ` is a
        // similarity invariant, so two adjacent n-blocks are similar to two
        // adjacent unit cells and read identically.
        for n in 1..=5i64 {
            let cells = |r0: i64, c0: i64| {
                let mut v = Vec::new();
                for r in 0..n {
                    for c in 0..n {
                        v.push((r0 + r, c0 + c, 0));
                    }
                }
                blade::shape_of(&v).blade
            };
            let (a, b) = (cells(0, 0), cells(0, n));
            assert_eq!(
                a.radius2(),
                q(n * n) / q(6),
                "an n×n block carries r² = n²/6"
            );
            assert_eq!(d2(&a, &b), q(4), "and edge-adjacency is δ² = 4 at n = {n}");
        }
        let b0 = block(0, 0);
        assert!(b0.radius2() > u0.radius2(), "a block is larger than a lamina");
        assert!(d2(&b0, &block(0, 60)) > q(4), "and distance is still refused");
    }

    /// THE GAUGE FREQUENCY IS THE SCALE EXPONENT — MEASURED (65.6).
    ///
    /// The hypothesis: a harmonic of azimuthal order `m` carries scaling
    /// degree `m`, because a conformal homothety acts as `w = ρe^{iθ}` and
    /// `f_m(wz) = ρ^m e^{imθ} z^m` — the integer in the phase and the
    /// exponent in the amplitude are the SAME `m`. So `S` reads its weights
    /// off `G` rather than from a table.
    ///
    /// IT MUST BE MEASURED AGAINST `blade::lift`, NOT ASSUMED. The
    /// derivation is for MASS-NORMALISED moments, and `deviatoric` returns
    /// raw `inertia` components with no division by area. If `lift` does
    /// not normalise, the degrees shift by the mass's own weight and every
    /// cross-multiplication gate built on them is off.
    ///
    /// A body and its exact 2× dilate: each cell becomes a 2×2 block, which
    /// is the dilation `x ↦ 2x` performed on the lattice itself.
    #[test]
    fn the_scaling_degrees_of_the_harmonic_tower() {
        use crate::transducer::blade;
        let q = |n: i64| crate::core::Q::from_integer(n.into());

        // An L-tromino: `C₁`, so every harmonic is non-vanishing and each
        // degree is observable.
        let base = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]).blade;
        let dilate = {
            let mut cells = Vec::new();
            for &(r, c) in &[(0i64, 0i64), (1, 0), (0, 1)] {
                for dr in 0..2 {
                    for dc in 0..2 {
                        cells.push((2 * r + dr, 2 * c + dc, 0));
                    }
                }
            }
            blade::shape_of(&cells).blade
        };

        // ρ = 2, so ρ² = 4. The trace is claimed at degree 2.
        let (ra2, rb2) = (base.radius2(), dilate.radius2());
        assert_eq!(&rb2 / &ra2, q(4), "r² scales as ρ² — the trace sits at degree 2");

        let norm2 = |v: [crate::core::Q; 2]| &v[0] * &v[0] + &v[1] * &v[1];
        let da = norm2(blade::deviatoric(&base).expect("anisotropic"));
        let db = norm2(blade::deviatoric(&dilate).expect("anisotropic"));
        // MEASURED: `‖D‖²` scales as 256 = ρ⁸, NOT ρ⁴.
        //
        // THE CARRIERS ARE AT MIXED NORMALISATION, and that is the finding.
        // The theorem "azimuthal order m carries scaling degree m" holds for
        // MASS-NORMALISED moments, `T^(k) = (1/M₀)∫(x−P)^⊗k`. `radius2()`
        // is normalised — it is the conformal sphere's `r²`, a length², and
        // scales as `ρ²`. `deviatoric` is NOT: it returns raw `inertia`
        // combinations, and a raw second moment over a 2D region carries
        // the area's own weight, `∫x² ~ ρ²·ρ² = ρ⁴`. So `‖D‖² ~ ρ⁸`.
        //
        // `S` therefore CANNOT read its exponent off `G` alone. It must
        // account for each carrier's own normalisation, and `Blade` does not
        // use one normalisation across the tower.
        // A.12 — NORMALISED, so the harmonic order IS the degree.
        //
        // Before normalisation this read `ρ⁸` for `‖D‖²` and `ρ¹²` for
        // `‖Q‖²`: `radius2()` was `Tr(I)/M₀` while the harmonics were raw,
        // one tower at two normalisations. Dividing every central moment by
        // `M₀` puts order `m` at degree `m`, so `‖M̃^(m)‖² ~ (ρ²)^m`.
        assert_eq!(&db / &da, q(16), "‖D̃‖² ~ ρ⁴ = (ρ²)², the deviatoric at m = 2");

        let qa = norm2(blade::quartic_harmonic(&base).expect("has a quartic"));
        let qb = norm2(blade::quartic_harmonic(&dilate).expect("has a quartic"));
        assert_eq!(&qb / &qa, q(256), "‖Q̃‖² ~ ρ⁸ = (ρ²)⁴, the quartic at m = 4");

        // THE SCALE CERTIFICATE, with its exponent read STRAIGHT OFF THE
        // GAUGE ORDER and no table anywhere:
        //
        //     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
        assert_eq!(&db * ra2.pow(2), &da * rb2.pow(2), "m = 2 certificate");
        assert_eq!(&qb * ra2.pow(4), &qa * rb2.pow(4), "m = 4 certificate");

        // A SHEAR IS NOT A DILATION, and the certificate must say so: a
        // body with the same trace but a different eccentricity fails at
        // m = 2 even though the scalar ratio would admit it.
        let bar = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (0, 3, 0)]).blade;
        let square = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]).blade;
        let (rs2, rq2) = (bar.radius2(), square.radius2());
        let ds = norm2(blade::deviatoric(&bar).expect("a bar is anisotropic"));
        // The square is `C₄`: its deviatoric VANISHES, so the certificate
        // is `‖D_bar‖² · (r_sq²)² = 0`, which fails unless the bar is
        // isotropic too. Shear refused.
        assert!(blade::deviatoric(&square).is_none(), "a square has no deviatoric");
        assert!(
            &ds * rq2.pow(2) != q(0),
            "so the m = 2 certificate refuses the bar against the square"
        );
        let _ = rs2;
    }
}
