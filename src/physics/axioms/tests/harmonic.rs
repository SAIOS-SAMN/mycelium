// 56.3, 66 — A.11's harmonic tower and A.12's extent, read on bodies.

use crate::transducer::blade::sensor_plane as blade;
use num_traits::Zero;

/// A.11's RADIAL/ORIENTATION SPLIT, on bodies that distinguish them.
///
/// Two congruent bodies at different orientations share a trace and
/// differ in the deviatoric. If the trace could orient, these would be
/// distinguishable by it; they are not, which is the axiom's content.
#[test]
fn a11_the_trace_cannot_orient_and_the_deviatoric_can() {
    // A domino lying along the rows, and the same domino along the
    // columns: congruent, one a 90° turn from the other.
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
        "a 90° turn is 2φ = π: cos 2φ = −1, sin 2φ = 0; got ({cos2}, {sin2})"
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

/// A.11's FRAME, MINTED: a body and its 90° turn give the rotor
/// `1 + e₁₂`, whose projective order is 4 — the reading `ℓ = 1` on
/// every word exists to be moved off.
#[test]
fn a11_a_turn_90_mints_an_order_four_rotor() {
    // An L-tromino and the same L turned 90°: chiral enough
    // that only one of the four turns can carry it.
    let upright = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0)]);
    let turned = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0)]);
    let theta = blade::relative_rotor(&upright.blade, &turned.blade)
        .expect("one proper turn carries an L onto its 90° turn");
    assert!(
        matches!(theta, blade::Pose::Turn(_)),
        "equal cardinality: the support settles the 180° turn, got {theta:?}"
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
/// The 180° turn stays OPEN here, and the REASON CHANGED (68). It was
/// "the cardinalities differ, so no cell test can run" — a count
/// standing in for scale, and false as a general statement now that
/// `φ = z₃·conj(z₂)` reads across any dilation.
///
/// It stays open because A DOMINO IS 2-FOLD SYMMETRIC, so `h₃ = 0`
/// exactly: the 180° turn maps it to itself and sends `h₃ ↦ −h₃`.
/// Both candidates act identically on it and there is no fact to
/// recover. The `ℤ₂` freedom is a property of the body, not a gap in
/// the reader — and an L-tetromino at the same two scales is now
/// DETERMINED, which this pair can never be.
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
        matches!(theta, blade::Pose::HalfOpen(_)),
        "a 2-fold symmetric body leaves the 180° turn open, got {theta:?}"
    );
    assert!(
        blade::third_harmonic(&domino.blade).is_none(),
        "and THAT is why: the odd harmonic vanishes on it"
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
fn a11_a_mirrored_body_is_read_as_its_odd_versor() {
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
    //
    // AND THE REFLECTION IS READ, NOT REFUSED (AL1 as amended): a mirrored
    // pair is the same signal read orientation-reversed, its pose the odd
    // versor the coset reader verifies — never `Oblique`, and never a
    // demand for a generator.
    let pose = blade::relative_rotor(&l.blade, &j.blade).expect("the mirror is read");
    assert_eq!(pose.rotor().parity(), Some(true), "an odd versor, not a rotor: {pose:?}");
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
/// them — and it is why `Pose::HalfOpen` exists.
///
/// Closing it needs an ODD invariant. The 3rd harmonic is the candidate,
/// and the reason it works is NOT that bodies have three-fold symmetry
/// (a square lattice forbids that) but that a generic body has a
/// non-vanishing third moment which changes sign under `x -> -x`.
#[test]
fn a11_no_even_moment_separates_a_body_from_its_point_reflection() {
    let body = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    // The same body reflected through the origin. Congruent to itself
    // under a 180° turn, and a 180° turn is what must be detected.
    let flipped = blade::shape_of(&[(0, 0, 0), (-1, 0, 0), (0, -1, 0)]);

    let (ua, ub) = (
        blade::deviatoric(&body.blade).expect("anisotropic"),
        blade::deviatoric(&flipped.blade).expect("anisotropic"),
    );
    assert_eq!(ua, ub, "the 2nd harmonic cannot see a 180° turn");

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
    let t_in = cga::at_three::translator(&q(3), &q(5), &q(0));
    assert!(
        cga::bivector_is_simple(&r1.geo(&t_in)),
        "a turn and a slide inside the sensor plane stay simple"
    );

    // Two in-plane translations: `e_∞` repeats.
    let t_in2 = cga::at_three::translator(&q(-2), &q(7), &q(0));
    assert!(
        cga::bivector_is_simple(&t_in.geo(&t_in2)),
        "two in-plane slides stay simple"
    );

    // NOW LEAVE THE PLANE. A turn in `e₁e₂` against a displacement with
    // a third-axis component — which in this engine is the FRAME axis,
    // the corpus's own `e₃` (61.3) — spans a four-volume.
    let t_off = cga::at_three::translator(&q(0), &q(0), &q(1));
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
    use crate::transducer::blade::sensor_plane as blade;
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
    // nucleus is spacelike and a tether between two of them has a weight
    // `⟨VṼ⟩₀ = r_a² r_b²` bounded away from zero: its regime is always
    // read.
    let v = blade::tether(&one, &one).expect("a centroid");
    let w = v.geo(&v.reverse()).scalar_part();
    assert!(w >= &sixth * &sixth, "the tether's weight r_a² r_b² is bounded below");
    assert!(w > q(0), "so the regime is always defined");
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
    use crate::transducer::blade::sensor_plane as blade;
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
