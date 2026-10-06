// 78.100 D1 — A WORD A THEOREM TAUGHT ENTERS THE LANGUAGE: the mind's
// sentence teaches it, and the voice transcribes it by the one transcriber a
// closed thought is read by — spelled, it is a word the voice can stand and
// speak.

use crate::client::sdk::*;
use crate::transducer::cga::{Mv, E2, E3};
use crate::transducer::sentence::compose::{compose, Term};
use crate::transducer::sentence::rung::{hear, sentence_tally};

/// A 90° TURN TAUGHT, TRANSCRIBED, SPELLED. A sentence carrying the turn of
/// the `e₂e₃` plane and back closes at once and teaches the turn; the
/// voice's transcription reads it through its own letters — the seventeen
/// spell a 90° turn — and hands back a word to stand and speak, counted as
/// spelled on the sentences row.
#[test]
fn a_word_a_theorem_taught_is_transcribed_into_the_language() {
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("taught"));
    let one = num_rational::BigRational::from_integer(1.into());
    let w = Mv::scalar(one.clone()).add(&Mv::blade(E2 | E3, one)).widened(3);
    let mut budget = e.stroke();
    let s = compose(&[Term { word: w.clone(), transport: w.reverse() }], &mut budget).expect("a sentence");
    let before = sentence_tally();
    hear(&s, &mut budget);
    let (words, _) = e.transcribe_taught(&mut budget);
    let after = sentence_tally();
    assert!(after.taught_spelled > before.taught_spelled, "the taught turn is spelled: {after:?}");
    assert!(!words.is_empty(), "and handed back as a word to stand and speak");
    assert!(words.iter().all(|x| x.is_connection()), "each a connection, as speech is");
}

/// 78.102 — A TAUGHT WORD NO HELD LETTER SPELLS IS INEFFABLE, AND SAYS SO: the
/// `(8, 15, 17)` turn (rotor `4 + e₂₃`, the Gaussian prime 17, which no
/// letter of 5 or 13 composes) is taught and transcribed; nothing is handed
/// back to speak as a word, and the transcription reports the ineffable word
/// so the voice speaks the perceived form that taught it as an open sentence
/// (the letter carrier's voice half).
#[test]
fn an_ineffable_taught_word_is_reported_for_its_sentence_to_be_spoken() {
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("ineffable"));
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let w = Mv::scalar(q(4)).add(&Mv::blade(E2 | E3, q(1))).widened(3);
    let mut budget = e.stroke();
    let s = compose(&[Term { word: w.clone(), transport: w.reverse() }], &mut budget).expect("a sentence");
    let before = sentence_tally();
    hear(&s, &mut budget);
    let (words, ineffable) = e.transcribe_taught(&mut budget);
    assert!(sentence_tally().taught_ineffable > before.taught_ineffable, "counted ineffable");
    assert!(ineffable, "and reported, so its sentence is spoken");
    assert!(words.iter().all(|x| x.is_connection()), "no unspelled word is handed back as speech");
}
