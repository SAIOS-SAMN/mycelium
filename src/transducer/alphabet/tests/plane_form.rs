// 9, 9.1 — the plane form as the coefficient group, and conjugacy as an
// equality of coefficients rather than a search.

use crate::transducer::alphabet::*;

/// THE FORM RE-DERIVES THE ROTOR IT WAS READ FROM. `to_word` is the
/// inverse of `form`, so `form().to_word()` is the same group element,
/// which is what lets `normal_form` return it. A plane spelled with the
/// wrong handedness, or an orientation counted twice, breaks exactly here
/// and nowhere visible until a conjugation comes out inverted.
#[test]
fn a_plane_form_rebuilds_the_rotor_it_was_read_from() {
    let q = |p| Word::from_letters(&[(LetterId::turn_90(p).expect("a pair plane"), false)]);
    let cases: Vec<(&str, Word)> = vec![
        ("sensor 90° turn", q((0, 1))),
        ("off-sensor 90° turn", q((0, 2))),
        ("far 90° turn", q((2, 3))),
        ("sheet", PlaneForm::sheet_word()),
        ("sensor 180° turn", q((0, 1)).compose(&q((0, 1)))),
        ("double rotation", q((0, 1)).compose(&q((2, 3)))),
        // The RAW conjugate tape, before any normalisation: `rebase` runs
        // `normal_form` itself, so asserting on its output would only say
        // the normal form is idempotent.
        (
            "raw conjugate tape",
            Word::from_letters(&[
                (LetterId::turn_90((1, 2)).expect("sheet plane"), false),
                (LetterId::turn_90((0, 1)).expect("sensor plane"), false),
                (LetterId::turn_90((1, 2)).expect("sheet plane"), true),
            ]),
        ),
        ("conjugate", q((0, 1)).rebase(&q((1, 2)))),
        ("prime on the sensor", Word::from_letters(&[(LetterId::PRIME_5, false)])),
    ];
    for (name, w) in cases {
        let Some(f) = w.form() else {
            continue;
        };
        assert!(
            f.to_word().rotor().projective_eq(&w.rotor()),
            "{name}: the form rebuilds its own rotor"
        );
        assert!(
            w.normal_form().rotor().projective_eq(&w.rotor()),
            "{name}: and the normal form is the same element"
        );
    }
}

/// PlaneForm is the coefficient group: products and inverses are exact
/// and O(1); a sheet conjugates exponents.
#[test]
fn plane_form_is_the_coefficient_group() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let rot = Word::from_letters(&[(q, false)]).form().unwrap();
    let sheet = PlaneForm { orientation: true, rot: [0; CAYLEY_ALPHABET_LEN] };
    assert!(rot.compose(&rot.inverse()).is_identity());
    assert!(sheet.compose(&sheet).is_identity(), "a sheet has order 2");
    let conj = rot.conjugate_by(&sheet);
    assert_eq!(conj, rot.inverse(), "S R S⁻¹ = R⁻¹");
    assert_eq!(rot.compose(&sheet), sheet.compose(&rot.inverse()));
    assert_eq!(rot.key(), rot.to_word().form().unwrap().key());
    let w = Word::from_letters(&[(q, false)]);
    let r = w.rotor();
    assert!(r.geo(&r.reverse()).is_identity());
    let n = w.compose(&w.inverse()).normal_form();
    assert!(n.rotor().is_identity(), "normal form of a cancelling tape is 1");
}

/// §52.1: conjugacy key is invariant on the four conjugation lines.
#[test]
fn plane_conjugacy_four_lines() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false), (q, false)]).form().unwrap();
    let r2 = Word::from_letters(&[(q, false)]).form().unwrap();
    let sheet = PlaneForm { orientation: true, rot: [0; CAYLEY_ALPHABET_LEN] };
    let conj = |g: &PlaneForm, x: &PlaneForm| g.compose(&x.compose(&g.inverse()));
    assert_eq!(conj(&r2, &r).conjugacy_key(), r.conjugacy_key(), "(0,r') even");
    assert_eq!(conj(&sheet, &r).conjugacy_key(), r.conjugacy_key(), "(1,r'') even class of r and -r");
    let odd = r.compose(&sheet);
    assert_eq!(conj(&r2, &odd).conjugacy_key(), odd.conjugacy_key(), "(0,r') odd");
    assert_eq!(conj(&sheet, &odd).conjugacy_key(), odd.conjugacy_key(), "(1,r'') odd");
}

#[test]
fn conjugacy_key_iff_conjugate() {
    let q = LetterId::turn_90((0, 1)).unwrap();
    let r = Word::from_letters(&[(q, false)]).form().unwrap();
    let inv = r.inverse();
    assert!(r.conjugacy_eq(&inv), "r and -r are one even class");
    assert_eq!(r.conjugacy_key(), inv.conjugacy_key(), "r and -r are one even class");
    let sheet = PlaneForm { orientation: true, rot: [0; CAYLEY_ALPHABET_LEN] };
    let odd = r.compose(&sheet);
    let odd2 = inv.compose(&sheet);
    assert!(odd.conjugacy_eq(&odd2), "odd class is r mod 2");
    assert_eq!(odd.conjugacy_key(), odd2.conjugacy_key(), "odd class is r mod 2");
    assert!(!r.conjugacy_eq(&odd), "sheet splits the class");
    assert_ne!(r.conjugacy_key(), odd.conjugacy_key(), "sheet splits the class");
}
