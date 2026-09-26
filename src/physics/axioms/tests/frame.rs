// 68 — the frame reader: the odd invariant, the scale certificate, and
// the refusals of the oblique branch.

use crate::transducer::blade::sensor_plane as blade;
use num_traits::Zero;

/// THE ODD HARMONIC CLOSES THE 180° TURN, AND VANISHES EXACTLY WHERE
/// THE FREEDOM IS REAL (68).
///
/// `h₃` turns at `3φ` and is the first odd-order harmonic the tower
/// carries. It is zero precisely for a body with 2-fold symmetry,
/// because the 180° turn maps such a body to itself and sends
/// `h₃ ↦ −h₃` — so where it vanishes, the two candidate turns act
/// IDENTICALLY and there is no fact left to recover.
///
/// That is what makes `HalfOpen` honest rather than a shrug: the `ℤ₂`
/// gauge freedom is a property of the body, not of the reader.
#[test]
fn the_odd_harmonic_vanishes_exactly_on_a_two_fold_symmetric_body() {
    // A domino and an S-tetromino are 2-fold symmetric: h₃ = 0.
    for cells in [
        vec![(0, 0, 0), (1, 0, 0)],
        vec![(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)],
    ] {
        let b = blade::shape_of(&cells);
        assert!(
            blade::third_harmonic(&b.blade).is_none(),
            "a 180° turn maps it to itself, so h3 must vanish: {cells:?}"
        );
    }
    // An L-tetromino has no 2-fold symmetry, so h₃ is there to read
    // and the 180° turn is determined rather than left open.
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    assert!(blade::third_harmonic(&l.blade).is_some(), "chiral, no 2-fold");
}

/// THE 180° TURN IS NOW READ ACROSS A CHANGE OF SCALE (68).
///
/// The cell test was gated on `support.len()` equality, so a pair of
/// different size was refused a reading and returned mod `π` —
/// a cell count standing in for scale, which is the `half-open` group.
/// `φ = z₃·conj(z₂)` is scale-free, so the turn between a body and its
/// dilate is DETERMINED, not left open.
#[test]
fn the_turn_180_is_determined_between_a_body_and_its_dilate() {
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let l2 = blade::shape_of(&[
        (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
        (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
        (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
        (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
    ]);
    assert_ne!(l.blade.support.len(), l2.blade.support.len(), "four against sixteen");
    let frame = blade::relative_rotor(&l.blade, &l2.blade).expect("a turn");
    assert!(
        matches!(frame, blade::Pose::Turn(_)),
        "determined across a dilation, not HalfOpen: {frame:?}"
    );
}

/// THE QUARTIC BRANCH OFFERS ALL FOUR TURNS, NOT TWO (68).
///
/// A moment of azimuthal order `m` turns at `m·φ`, so the same
/// `(wedge, dot)` means different things at `m = 2` and `m = 4`. At
/// `m = 4`, `wedge = 0` and `dot > 0` is `4φ ≡ 0`, which is EVERY
/// 90° turn — but the reader tried `[0, 2]` and refused the other
/// two, reporting `Incongruent` on exactly the bodies the quartic
/// exists to orient: those isotropic at second order.
///
/// A square is `C₄`, so every 90° turn carries it onto itself and
/// all four must be reachable.
#[test]
fn a_quartic_oriented_body_reaches_every_turn_90() {
    let sq = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
    assert!(blade::deviatoric(&sq.blade).is_none(), "isotropic at order 2");
    assert!(blade::quartic_harmonic(&sq.blade).is_some(), "the quartic orients it");
    let pose = blade::relative_rotor(&sq.blade, &sq.blade);
    assert!(
        matches!(&pose, Ok(blade::Pose::HalfOpen(c)) if c.len() == 4 && c.iter().all(|r| r.parity() == Some(false))),
        "a C4 body carries onto itself by its four turns, the coset held and none chosen: {pose:?}"
    );
}

/// DG1, 78.38(C) — A TRANSITION AT A RATIONAL TURN NO LATTICE TURN LISTS IS
/// CARRIED. `z ↦ (3 + 4i)·z`, built by the algebra — the rotor `2 + e₁₂`
/// under `D(5)` — carries the L's sites onto sites; the point tower certifies
/// it, the second harmonic's point is `cos 2φ = −7/25`, and the exact
/// half-angle step (`2(1 + C) = 36/25`) reads `φ` back as the map's own
/// rotor. The square base's four turns refused it as `Oblique`.
#[test]
fn a_pythagorean_site_map_is_carried_at_its_rational_turn() {
    use crate::transducer::cga;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let map = cga::dilator(&q(5)).geo(&cga::rotator(&q(2), &q(1)));
    let image: Vec<crate::transducer::base::Site> = l
        .blade
        .support
        .iter()
        .map(|s| {
            let p = blade::carry(&map, s.axes()).expect("a finite image");
            crate::transducer::base::Site::new(
                p.iter().map(|x| i64::try_from(x.to_integer()).expect("a site of the lattice")).collect(),
            )
        })
        .collect();
    let image = blade::shape_of(&image);
    assert_eq!(blade::dilation(&l.blade, &image.blade), Ok(blade::Dilation::Sites), "a site map");
    let pose = blade::relative_rotor(&l.blade, &image.blade).expect("carried at its rational turn");
    assert!(matches!(pose, blade::Pose::Turn(_)), "determined, the odd harmonic settling the 180° turn: {pose:?}");
    assert!(pose.rotor().projective_eq(&cga::rotator(&q(2), &q(1))), "the map's own rotor, read back: {pose:?}");
}

/// 55.3, 68 — THE CHIRAL L IS CARRIED ONTO ITS 90° TURN BY COVARIANCE:
/// one member, even, and the verification is the tower's — the centre, the
/// second moment, the odd and quartic harmonics — never a lamina landing.
#[test]
fn the_chiral_l_is_carried_onto_its_turn_by_the_towers_covariance() {
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let turned = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 0, 0)]);
    let pose = blade::relative_rotor(&l.blade, &turned.blade).expect("a turn");
    assert!(matches!(pose, blade::Pose::Turn(_)), "h₃ settles the 180° turn: {pose:?}");
    assert_eq!(pose.rotor().parity(), Some(false), "a turn, not a mirror");
    let v = blade::versor_between(&l, &turned).expect("the transition");
    assert!(
        blade::carries_by_covariance(&v, &l.blade, &turned.blade, blade::Tower::Region),
        "the region's tower is carried by it"
    );
    assert!(
        !blade::carries_by_covariance(&v, &turned.blade, &l.blade, blade::Tower::Region),
        "and not the other way: a 90° turn is not its own inverse"
    );
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
    let refused = blade::relative_rotor(&l.blade, &s.blade).err();
    assert!(
        matches!(refused, Some(blade::NoPose::Incongruent(blade::Incongruence::Order { .. }))),
        "an invariant separates them on both towers, so no vocabulary reaches them: {refused:?}"
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
    // And the odd gate is what READS it (AL1 as amended): the pose is the
    // odd versor, the pair read orientation-reversed, never a turn.
    let pose = blade::relative_rotor(&l.blade, &j.blade).expect("a mirror is read, never refused");
    assert_eq!(pose.rotor().parity(), Some(true), "the odd gate reads the reflection: {pose:?}");
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
    let pose = blade::relative_rotor(&j.blade, &l2.blade).expect("improper across a dilation, and read");
    assert_eq!(pose.rotor().parity(), Some(true), "its pose is the odd versor, read across the scale: {pose:?}");
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
    let refused = blade::relative_rotor(&d2.blade, &l3.blade).err();
    assert!(
        matches!(refused, Some(blade::NoPose::Incongruent(blade::Incongruence::Order { .. }))),
        "chirality permits it; both towers refuse it; the towers win: {refused:?}"
    );
}

/// 78.38 (C) — A SITE MAP IS A DILATION ON THE POINT TOWER, AND THE
/// REGION TOWER REFUSES IT FOR THE SUBSTRATE'S REASON.
///
/// `z ↦ (4 + i)·z` carries the L-tetromino's four sites onto four sites
/// of the lattice: one shape at two scales and an angle the alphabet does
/// not spell. The lamina tower integrates each lamina's own unit square —
/// A.12's floor, which no similarity of the lattice scales — so it
/// refuses; the point tower is exact under the map and certifies. The
/// relation is read on the tower that certified: one ray for `Z`, the arc
/// of `4 + i`, and the pose left open as `Oblique`.
#[test]
fn a_site_map_certifies_on_the_point_tower_and_reads_oblique() {
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let image = blade::shape_of(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    assert_eq!(blade::dilation(&l.blade, &image.blade), Ok(blade::Dilation::Sites), "the sites certify, the region does not");
    // The other similarity of the lattice: a filled 2×2 blow-up is exact on
    // the region and not on the sites, so the region tower certifies it.
    let l2 = blade::shape_of(&[
        (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
        (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
        (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
        (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
    ]);
    assert_eq!(blade::dilation(&l.blade, &l2.blade), Ok(blade::Dilation::Region), "the blow-up certifies on the region");
    let za = blade::chirality_on(&l.blade, blade::Tower::Sites).expect("a chiral body");
    let zb = blade::chirality_on(&image.blade, blade::Tower::Sites).expect("its image is chiral too");
    assert!((&za.0 * &zb.1 - &za.1 * &zb.0).is_zero(), "one ray for Z on the point tower");
    assert!(&za.0 * &zb.0 + &za.1 * &zb.1 > num_rational::BigRational::zero(), "the same ray, not its opposite");
    // The arc is the DOUBLE angle's (`z` turns at `2φ`), so a site turn by
    // `4 + i` reads `(4 + i)² = 15 + 8i`: primitive, and its gcd with 17 in
    // `ℤ[i]` is `4 + i` itself, of norm exactly 17.
    let arc = blade::oblique_arc(&l.blade, &image.blade).expect("the arc of the turn");
    assert_eq!(arc, (num_bigint::BigInt::from(15), num_bigint::BigInt::from(8)), "the arc of (4 + i)²");
    assert_eq!(
        blade::relative_rotor(&l.blade, &image.blade).err(),
        Some(blade::NoPose::Oblique),
        "a similarity at an angle the alphabet does not spell is left open"
    );
}

/// 78.38 (C) — INCONGRUENT ONLY WHERE BOTH TOWERS REFUSE, NAMING THE ORDER
/// AT WHICH EACH DISAGREES.
///
/// The S-tetromino is a genuinely different shape from the L's image: no
/// similarity carries one onto the other, and each tower names the first
/// harmonic order at which they part.
#[test]
fn a_different_shape_is_incongruent_at_its_order_on_both_towers() {
    let image = blade::shape_of(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    let s = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    let two = blade::TowerRefusal::At(blade::Harmonic::Two);
    assert_eq!(blade::dilation(&image.blade, &s.blade), Err((two, two)), "both towers refuse at the deviatoric");
    assert_eq!(
        blade::relative_rotor(&image.blade, &s.blade).err(),
        Some(blade::NoPose::Incongruent(blade::Incongruence::Order { region: two, sites: two })),
        "the refusal names the order from each tower"
    );
}

/// 78.38 (C), R1 — A BODY WITH NO POINTS ON THE PLANE IS REFUSED BY THAT NAME.
///
/// Asked on a plane off the bodies' base, neither tower carries a scale for
/// them, so each refuses as `Unplaced` before any order is read — never as a
/// difference of shape at the deviatoric.
#[test]
fn a_plane_off_the_base_is_unplaced_on_both_towers() {
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let off = (0, l.blade.n() + 1);
    let unplaced = blade::TowerRefusal::Unplaced;
    assert_eq!(
        crate::transducer::blade::dilation(&l.blade, &l.blade, off),
        Err((unplaced, unplaced)),
        "no scale on either tower, refused by its own name"
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
    let unlike = |r: &Option<blade::NoPose>| {
        matches!(r, Some(blade::NoPose::Incongruent(blade::Incongruence::Order { .. })))
    };
    // The improper pair is read as its odd versor (AL1 as amended), so it
    // leaves the refusals entirely; the unlike pairs stay refused apart.
    assert!(
        matches!(read, [None, ref s, ref b, None] if unlike(s) && unlike(b)),
        "read orientation-reversed, unlike, unlike, and a turn across a dilation: {read:?}"
    );
    let mirror = blade::relative_rotor(&l.blade, &j.blade).expect("the improper pair is read");
    assert_eq!(mirror.rotor().parity(), Some(true), "as the odd versor: {mirror:?}");
}

/// 78.38 (C), clarified — THE CERTIFICATE IS ASKED BEFORE THE TURN BRANCH.
///
/// A domino and a three-cell bar lie along one axis and are both 2-fold
/// symmetric, so the aligned branch reads only their angle — `wedge = 0`,
/// `h₃ = 0` — and held them `HalfOpen`, a turn between two shapes no
/// similarity relates. Neither tower certifies them, and no reader past the
/// angle would refuse them, so the refusal comes first, naming the order
/// from each tower.
#[test]
fn an_aligned_unlike_pair_is_refused_before_the_turn_branch() {
    let dom = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]);
    let bar = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
    assert!(blade::third_harmonic(&dom.blade).is_none() && blade::third_harmonic(&bar.blade).is_none(), "2-fold, h3 = 0");
    let (two, four) = (blade::TowerRefusal::At(blade::Harmonic::Two), blade::TowerRefusal::At(blade::Harmonic::Four));
    assert_eq!(blade::dilation(&dom.blade, &bar.blade), Err((two, four)), "the region parts at 2, the sites at 4");
    assert_eq!(
        blade::relative_rotor(&dom.blade, &bar.blade).err(),
        Some(blade::NoPose::Incongruent(blade::Incongruence::Order { region: two, sites: four })),
        "unlike, not a held 180° turn"
    );
}
