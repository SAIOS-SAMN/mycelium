// 78.69 — THE MASTER SENTENCE EQUATION: words and transports composed in
// order until the ring closes, every word carried into one chart, the closed
// sentence's open path a word of the next rung.

use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::compose::{compose, next_rung, Closure, Term};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn one() -> Mv {
    Mv::scalar(q(1))
}

/// A 90° turn on the plane `mask`: `1 + e_ij`.
fn turn(mask: usize) -> Mv {
    one().add(&Mv::blade(mask, q(1)))
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A WORD AND ITS INVERSE, carried by the identity, close at once: `Ω ~ 1`,
/// ℓ = 1. One 90° turn alone, carried back to itself, closes at its order:
/// four.
#[test]
fn a_sentence_closes_at_the_period_of_its_holonomy() {
    let r = turn(E1 | E2);
    let back = Term { word: r.reverse(), transport: one() };
    let s = compose(&[Term { word: r.clone(), transport: one() }, back], &mut stroke()).expect("a sentence");
    assert_eq!(s.closure, Closure::Closes(1), "a word and its inverse return at once");
    let single = compose(&[Term { word: r, transport: one() }], &mut stroke()).expect("a sentence");
    assert_eq!(single.closure, Closure::Closes(4), "a 90° turn returns at its fourth pass");
}

/// A TRANSLATION NEVER RETURNS: a sentence carried by a null transport is
/// open, its demand standing.
#[test]
fn a_translated_sentence_is_open() {
    let t = cga::translator_at(&[q(1), q(0), q(0)]);
    let s = compose(&[Term { word: one(), transport: t }], &mut stroke()).expect("a sentence");
    assert_eq!(s.closure, Closure::Unbound, "a translation carries and never returns");
    assert!(next_rung(&[s]).is_empty(), "an open sentence teaches no word");
}

/// CONJUGATION CARRIES EVERY WORD INTO THE FIRST CHART: the second word,
/// written in its own chart as `g W g̃`, is read in the first chart as `W`
/// itself — the same motion seen from two charts is one generator (§1).
#[test]
fn every_word_is_carried_into_the_first_chart() {
    let w = turn(E1 | E2);
    let g = turn(E2 | E3);
    let seen = g.geo(&w).geo(&g.reverse());
    let s = compose(&[Term { word: w.clone(), transport: g.clone() }, Term { word: seen, transport: g.reverse() }], &mut stroke())
        .expect("a sentence");
    assert!(s.carried[1].projective_eq(&w), "the second word, carried back, is the first word's motion");
    assert_eq!(s.rank, Ok(1), "one motion seen twice spans one dimension");
}

/// THE CURVATURE IS THE CARRIED WORDS' ALGEBRA: two turns on planes that
/// share an axis do not commute and close to so(3), rank 3.
#[test]
fn a_sentence_reads_the_rank_of_its_carried_words() {
    let s = compose(
        &[Term { word: turn(E1 | E2), transport: one() }, Term { word: turn(E2 | E3), transport: one() }],
        &mut stroke(),
    )
    .expect("a sentence");
    assert_eq!(s.rank, Ok(3), "[e₁₂, e₂₃] closes to so(3)");
}

/// THE RUNG RECURSES: a closed sentence's open path is a word of the next
/// rung, and that word composes into a sentence exactly as a word of this
/// rung does — here the 90° turn's sentence teaches the turn itself, and the
/// next rung's sentence of that word closes at four again.
#[test]
fn a_closed_sentence_teaches_a_word_of_the_next_rung() {
    let r = turn(E1 | E2);
    let s = compose(&[Term { word: r.clone(), transport: one() }], &mut stroke()).expect("a sentence");
    let words = next_rung(&[s]);
    assert_eq!(words.len(), 1, "one closed sentence, one word");
    assert!(words[0].projective_eq(&r), "the shortcut is the open path");
    let again = compose(&[Term { word: words[0].clone(), transport: one() }], &mut stroke()).expect("a sentence");
    assert_eq!(again.closure, Closure::Closes(4), "the next rung composes as this one does");
}
