// LAW-PASSAGE v1 (proposal 78.172): the linear passage `R ↦ 2H − R`, its
// six joint-read fixtures (§5) and the claims of the algebra.

use crate::passage::*;
use crate::Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// Exact signed reaches on one line, past the throat on either side, two
/// of them at a throat.
fn reaches() -> Vec<Q> {
    vec![q(0, 1), q(1, 2), q(-7, 3), q(5, 2), q(11, 4), q(-1, 1), q(13, 5), q(3, 2), q(2, 1)]
}

/// Throats at exact signed reaches, the zero throat among them.
fn throats() -> Vec<Throat> {
    [q(3, 2), q(7, 3), q(1, 1), q(0, 1), q(-5, 6), q(2, 1)].into_iter().map(Throat::at).collect()
}

/// FIXTURE 1: through the throat at `3/2`, the reach `1/2` passes to `5/2`,
/// the reflection of the reach about the throat.
#[test]
fn pass_equals_reflection_about_throat() {
    assert_eq!(Throat::at(q(3, 2)).pass(&q(1, 2)), q(5, 2));
}

/// FIXTURE 2: through the same throat the reach `5/2` passes back to `1/2`;
/// and over every throat and reach, two passages return the reach, so the
/// reverse crossing is the same law.
#[test]
fn pass_is_involution() {
    let h = Throat::at(q(3, 2));
    assert_eq!(h.pass(&q(5, 2)), q(1, 2), "fixture 2");
    for throat in throats() {
        for r in reaches() {
            assert_eq!(throat.pass(&throat.pass(&r)), r, "2H − (2H − R) = R");
        }
    }
}

/// FIXTURE 3: a reach at the throat `7/3` stays at `7/3`; every throat is
/// fixed by its own passage, and no other reach is.
#[test]
fn pass_fixes_throat() {
    let h = Throat::at(q(7, 3));
    assert_eq!(h.pass(&q(7, 3)), q(7, 3), "fixture 3");
    for throat in throats() {
        assert_eq!(&throat.pass(throat.reach()), throat.reach(), "H is fixed");
        for r in reaches().into_iter().filter(|r| r != throat.reach()) {
            assert_ne!(throat.pass(&r), r, "only H is fixed");
        }
    }
}

/// FIXTURE 4: the share `1/5` passes to `4/5`, the reflection about ½, and
/// the share's passage is an involution on `[0, 1]`.
#[test]
fn pass_share_equals_reflection_about_half() {
    assert_eq!(pass_share(&q(1, 5)), Ok(q(4, 5)), "fixture 4");
    for d in [q(0, 1), q(1, 2), q(3, 7), q(1, 1)] {
        let there = pass_share(&d).expect("a share");
        assert_eq!(pass_share(&there), Ok(d), "δ ↦ 1 − δ returns δ");
    }
}

/// A value outside `[0, 1]` is no share, and is refused `NotShare` with it.
#[test]
fn pass_share_refuses_value_outside_unit_interval() {
    for v in [q(-1, 5), q(6, 5)] {
        assert_eq!(pass_share(&v), Err(PassageRefusal::NotShare(Box::new(v.clone()))));
    }
}

/// FIXTURE 5: a crossing through the throat at `2` that carries the reach
/// `1` and states the inversion `H²/R = 4` is refused `NotLawful`, naming
/// `4` against the passage `3`.
#[test]
fn verify_refuses_inversion_claim() {
    assert_eq!(
        Throat::at(q(2, 1)).verify(&q(1, 1), &q(4, 1)),
        Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: q(4, 1), reconstructed: q(3, 1) })))
    );
}

/// THE SQUARING CANDIDATE IS REFUSED WHEREVER IT MOVES THE REACH: the
/// inversion `H²/R` agrees with `2H − R` exactly where `(H − R)² = 0`, the
/// throat itself, so a crossing stating it off the throat is refused
/// `NotLawful` with both readings, and at the throat it states the fixed
/// point the passage admits.
#[test]
fn verify_refuses_squaring_candidate_off_throat() {
    for throat in throats() {
        for r in reaches().into_iter().filter(|r| *r != q(0, 1)) {
            let inversion = throat.reach() * throat.reach() / &r;
            match &r == throat.reach() {
                true => assert_eq!(throat.verify(&r, &inversion), Ok(r.clone()), "at the throat both fix H"),
                false => assert_eq!(
                    throat.verify(&r, &inversion),
                    Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: inversion.clone(), reconstructed: throat.pass(&r) })))
                ),
            }
        }
    }
}

/// A crossing that states the passage is admitted on the reconstructed
/// object.
#[test]
fn verify_admits_stated_passage() {
    for throat in throats() {
        for r in reaches() {
            assert_eq!(throat.verify(&r, &throat.pass(&r)), Ok(throat.pass(&r)));
        }
    }
}

/// FIXTURE 6: through the throat at `1` the reach `3` passes to `−1`, past
/// the centre and signed, never clamped; the reverse, `−1`, passes back to
/// `3`.
#[test]
fn pass_keeps_sign_past_twice_throat() {
    let h = Throat::at(q(1, 1));
    assert_eq!(h.pass(&q(3, 1)), q(-1, 1), "fixture 6");
    assert_eq!(h.pass(&q(-1, 1)), q(3, 1), "the reverse");
}
