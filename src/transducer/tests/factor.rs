use crate::transducer::alphabet::{PlaneForm, Rotor, Word};
use crate::transducer::factor::*;
use crate::transducer::alphabet::LetterId;

fn w(ls: &[(LetterId, bool)]) -> Word {
    Word::from_letters(ls)
}
fn pf(word: &Word) -> PlaneForm {
    word.form().expect("a plane word")
}
fn budget(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}

/// A12 on the plane decides membership by Hermite normal form, and the
/// answer is a FACTORISATION — the exponents over the words the mind
/// holds — not merely a yes. Verified by re-derivation (step 4), since a
/// mind recognises what it can re-derive and nothing else.
#[test]
fn factor_on_plane_understands_span_by_hnf() {
    let q01 = LetterId::turn_90((0, 1)).expect("plane");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("plane");
    let held = [w(&[(q01, false)]), w(&[(p5, false)])];
    let basis: Vec<PlaneForm> = held.iter().map(pf).collect();
    let rotors: Vec<Rotor> = held.iter().map(|x| x.rotor()).collect();

    // q·p5·q — in the span, and not one of the held words.
    let g = w(&[(q01, false), (p5, false), (q01, false)]);
    match factor_on_plane(&pf(&g), &basis) {
        Factored::Understood(tape) => {
            assert!(!tape.is_empty(), "the identity is not the answer here");
            assert!(
                replays(&tape, &rotors, &g.rotor()),
                "the factorisation re-derives the word it claims"
            );
        }
        other => panic!("a word in the span is understood, got {other:?}"),
    }
}

/// Step 5: when membership fails the residue is the CANONICAL remainder
/// — unique for the lattice, so two minds holding the same words speak
/// the same demand — and the part that did factor is kept, not thrown
/// away with it.
#[test]
fn factor_on_plane_remainder_is_canonical() {
    let q01 = LetterId::turn_90((0, 1)).expect("plane");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("plane");
    let basis = [pf(&w(&[(q01, false)]))];
    let g = w(&[(q01, false), (p5, false)]);
    let a = factor_on_plane(&pf(&g), &basis);
    let b = factor_on_plane(&pf(&g), &basis);
    match (&a, &b) {
        (
            Factored::Novel { remainder: r1, .. },
            Factored::Novel { remainder: r2, .. },
        ) => {
            assert_eq!(r1.rot, r2.rot, "one lattice, one residue");
            assert!(r1.rot.iter().any(|&x| x != 0), "the prime is genuinely missing");
            // The unit turn IS held, so the demand must not ask for
            // one: slots 1..=6 are the 90° turns (MU-ALPHABET.md).
            assert!(
                (1..=6).all(|u| r1.rot[u] == 0),
                "it asks only for what is absent, not for what is held"
            );
        }
        _ => panic!("a word outside the span is Novel, got {a:?}"),
    }
}

/// A held word is understood however it is spelled: identity is the
/// vertex, not the tape (1.5). Four 90° turns are the identity
/// projectively, so a mind holding the 90° turn speaks them.
#[test]
fn factor_on_plane_understands_held_word_respelled() {
    let q01 = LetterId::turn_90((0, 1)).expect("plane");
    let held = [w(&[(q01, false)])];
    let basis: Vec<PlaneForm> = held.iter().map(pf).collect();
    let rotors: Vec<Rotor> = held.iter().map(|x| x.rotor()).collect();
    let spelled_long = w(&[(q01, false), (q01, false), (q01, false)]);
    match factor_on_plane(&pf(&spelled_long), &basis) {
        Factored::Understood(tape) => assert!(
            replays(&tape, &rotors, &spelled_long.rotor()),
            "three 90° turns are three of the one held"
        ),
        other => panic!("understood however spelled, got {other:?}"),
    }
}

/// Off the plane there is no HNF and no metric to say which partial is
/// nearer, so the traversal is budgeted and its states are A1 keys. An
/// exhausted budget is UNMEASURED, never Novel: a refusal is not a
/// demand (rule 11).
#[test]
fn factor_off_plane_reaches_or_refuses_unmeasured() {
    let q01 = LetterId::turn_90((0, 1)).expect("plane");
    let q12 = LetterId::turn_90((1, 2)).expect("plane");
    let held = [w(&[(q01, false)]), w(&[(q12, false)])];
    let rotors: Vec<Rotor> = held.iter().map(|x| x.rotor()).collect();
    let g = w(&[(q01, false), (q12, false)]);

    let mut b = budget(64);
    match factor_off_plane(&g.rotor(), &rotors, &mut b) {
        Factored::Understood(tape) => assert!(
            replays(&tape, &rotors, &g.rotor()),
            "the traversal's tape re-derives the word"
        ),
        other => panic!("a reachable word is understood, got {other:?}"),
    }

    // One unit cannot reach it: unmeasured, and the caller must not read
    // that as "the mind does not hold it".
    let mut short = budget(1);
    assert_eq!(
        factor_off_plane(&g.rotor(), &rotors, &mut short),
        Factored::Unmeasured,
        "an exhausted traversal refuses"
    );
}

/// The verification is real: a tape that does not re-derive is refused,
/// so `Understood` cannot be claimed by an algebra that got it wrong.
#[test]
fn replays_refuses_tape_that_does_not_rederive() {
    let q01 = LetterId::turn_90((0, 1)).expect("plane");
    let p5 = LetterId::pythagorean_5((0, 1)).expect("plane");
    let held = [w(&[(q01, false)]), w(&[(p5, false)])];
    let rotors: Vec<Rotor> = held.iter().map(|x| x.rotor()).collect();
    let g = w(&[(q01, false)]);
    assert!(replays(&[(0, 1)], &rotors, &g.rotor()), "the true tape replays");
    assert!(!replays(&[(1, 1)], &rotors, &g.rotor()), "a wrong tape does not");
    assert!(!replays(&[(9, 1)], &rotors, &g.rotor()), "an index off the basis does not");
}
