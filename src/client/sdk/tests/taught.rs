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
/// letter of 5 or 13 composes) is transcribed; nothing is handed back to
/// speak as a word, and the transcription reports it, so the voice speaks the
/// perceived form that taught it as an open sentence (the letter carrier's
/// voice half). The word is handed in, never drained from the process-wide
/// queue other tests fill and drain at once.
#[test]
fn an_ineffable_taught_word_is_reported_for_its_sentence_to_be_spoken() {
    use crate::transducer::sentence::rung::Taught;
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("ineffable"));
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let w = Mv::scalar(q(4)).add(&Mv::blade(E2 | E3, q(1))).widened(3);
    let mut budget = e.stroke();
    let before = sentence_tally();
    let (words, ineffable) = e.transcribe_these(vec![Taught { motion: w, period: 1 }], &mut budget);
    assert!(sentence_tally().taught_ineffable > before.taught_ineffable, "counted ineffable");
    assert_eq!(ineffable.len(), 1, "its motion reported, so its sentence is spoken and its letter made");
    assert!(words.is_empty(), "no unspelled word is handed back as speech");
}

/// 78.109 K1, C1 — THE MIND CONSTRUCTS ITS LETTER AND SAYS ITS WORD. The
/// `(8, 15, 17)` turn is ineffable; the mind constructs its residue —
/// spelled as constructors, certified (`cga::construct`) — and adjoins its
/// rotation in its local dialect; the same word taught again is said by the
/// mind's own adjoined letter — no court asked, no ruling waited for.
#[test]
fn a_mind_adjoins_its_ineffable_word_and_says_it_at_once() {
    use crate::transducer::sentence::rung::Taught;
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("sovereign"));
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let w = Mv::scalar(q(4)).add(&Mv::blade(E2 | E3, q(1))).widened(3);
    let mut budget = e.stroke();
    let (_, ineffable) = e.transcribe_these(vec![Taught { motion: w.clone(), period: 1 }], &mut budget);
    let chart = crate::transducer::stalk::Invariant::counts(&[1]);
    let made = crate::transducer::cga::construction_outcomes();
    // Constructing reads the residue's modes under the stroke they need,
    // read off the residue (78.111 C1b): the mind's own stroke suffices.
    for residue in &ineffable {
        e.construct_and_adjoin(residue, Some(&chart), &mut budget);
    }
    let after = crate::transducer::cga::construction_outcomes();
    let at = |n: &str| crate::transducer::cga::CONSTRUCTIONS.iter().position(|x| *x == n).expect("named");
    assert!(after[at("constructed")] > made[at("constructed")], "the residue spells as constructors: {made:?} → {after:?}");
    let before = sentence_tally();
    let (_, again) = e.transcribe_these(vec![Taught { motion: w, period: 1 }], &mut budget);
    assert!(again.is_empty(), "no longer ineffable: the mind holds its letter");
    assert!(sentence_tally().taught_adjoined > before.taught_adjoined, "said by its own adjoined letter");
}
