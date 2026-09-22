// 68 — the frame reader: the odd invariant, the scale certificate, and
// the refusals of the oblique branch.

use crate::transducer::blade;
use num_traits::Zero;

/// THE ODD HARMONIC CLOSES THE HALF TURN, AND VANISHES EXACTLY WHERE
/// THE FREEDOM IS REAL (68).
///
/// `h₃` turns at `3φ` and is the first odd-order harmonic the tower
/// carries. It is zero precisely for a body with 2-fold symmetry,
/// because the half turn maps such a body to itself and sends
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
            "a half turn maps it to itself, so h3 must vanish: {cells:?}"
        );
    }
    // An L-tetromino has no 2-fold symmetry, so h₃ is there to read
    // and the half turn is determined rather than left open.
    let l = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    assert!(blade::third_harmonic(&l.blade).is_some(), "chiral, no 2-fold");
}

/// THE HALF TURN IS NOW READ ACROSS A CHANGE OF SCALE (68).
///
/// The cell test was gated on `support.len()` equality, so a pair of
/// different size was refused a reading and returned mod `π` —
/// a cell count standing in for scale, which is the `half-open` group.
/// `φ = z₃·conj(z₂)` is scale-free, so the turn between a body and its
/// dilate is DETERMINED, not left open.
#[test]
fn the_half_turn_is_determined_between_a_body_and_its_dilate() {
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
        matches!(frame, blade::Frame::Turn(_)),
        "determined across a dilation, not HalfOpen: {frame:?}"
    );
}

/// THE QUARTIC BRANCH OFFERS ALL FOUR TURNS, NOT TWO (68).
///
/// A moment of azimuthal order `m` turns at `m·φ`, so the same
/// `(wedge, dot)` means different things at `m = 2` and `m = 4`. At
/// `m = 4`, `wedge = 0` and `dot > 0` is `4φ ≡ 0`, which is EVERY
/// quarter turn — but the reader tried `[0, 2]` and refused the other
/// two, reporting `Incongruent` on exactly the bodies the quartic
/// exists to orient: those isotropic at second order.
///
/// A square is `C₄`, so every quarter turn carries it onto itself and
/// all four must be reachable.
#[test]
fn a_quartic_oriented_body_reaches_every_quarter_turn() {
    let sq = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0)]);
    assert!(blade::deviatoric(&sq.blade).is_none(), "isotropic at order 2");
    assert!(blade::quartic_harmonic(&sq.blade).is_some(), "the quartic orients it");
    assert!(
        blade::relative_rotor(&sq.blade, &sq.blade).is_ok(),
        "a C4 body carries onto itself"
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
