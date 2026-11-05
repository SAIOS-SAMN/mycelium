// LAW-PASSAGE v2 (proposal 78.172): what the forms shell keeps of the
// passage, the refusal of a moved reach and the share's two faces. The
// passage's own fixtures stand with the algebra (78.172 §5).

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

/// A plane's share `1/5` read from its other axis is `4/5`, and reading
/// back from the first axis returns the share, over `[0, 1]`.
#[test]
fn pass_share_equals_reading_from_other_axis() {
    assert_eq!(pass_share(&q(1, 5)), Ok(q(4, 5)));
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

/// A crossing through the throat at `2` that carries the reach `1` and
/// states the inversion `H²/R = 4` is refused `NotLawful`, naming `4`
/// against the reach `1` it carried in.
#[test]
fn verify_refuses_inversion_claim() {
    assert_eq!(
        Throat::at(q(2, 1)).verify(&q(1, 1), &q(4, 1)),
        Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: q(4, 1), reconstructed: q(1, 1) })))
    );
}

/// THE SQUARING CANDIDATE IS REFUSED WHEREVER IT MOVES THE REACH: the
/// inversion `H²/R` names the carried reach exactly where `R = ±H`, so a
/// crossing stating it is refused `NotLawful` with both readings everywhere
/// but there, and at the throat it states the unmoved reach.
#[test]
fn verify_refuses_squaring_candidate_off_throat() {
    for throat in throats() {
        for r in reaches().into_iter().filter(|r| *r != q(0, 1)) {
            let inversion = throat.reach() * throat.reach() / &r;
            match inversion == r {
                true => assert_eq!(throat.verify(&r, &inversion), Ok(r.clone()), "the inversion names R only where R = ±H"),
                false => assert_eq!(
                    throat.verify(&r, &inversion),
                    Err(PassageRefusal::NotLawful(Box::new(Breach { claimed: inversion.clone(), reconstructed: r.clone() })))
                ),
            }
        }
    }
}

/// A crossing that states the reach it carried in is admitted on that
/// reach.
#[test]
fn verify_admits_unmoved_reach() {
    for throat in throats() {
        for r in reaches() {
            assert_eq!(throat.verify(&r, &r), Ok(r.clone()));
        }
    }
}
