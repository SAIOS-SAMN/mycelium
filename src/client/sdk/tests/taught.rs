// 78.100 D1 — A WORD A THEOREM TAUGHT ENTERS THE LANGUAGE: the mind's
// sentence teaches it, and the voice transcribes it by the one transcriber a
// closed thought is read by — spelled, it is a word the voice can stand and
// speak.

use crate::client::sdk::*;
use crate::transducer::cga::{Mv, E2, E3};
use crate::transducer::sentence::compose::{compose, Term};

/// A 90° TURN TAUGHT, TRANSCRIBED, SPELLED. A sentence carrying the turn of
/// the `e₂e₃` plane and back closes at once and teaches the turn; the
/// voice's transcription reads it through its own letters — the seventeen
/// spell a 90° turn — and hands back a word to stand and speak, counted as
/// spelled on the sentences row.
///
/// The word the theorem teaches is handed in (`teaches`, the one teaching
/// rule `hear` files by), never taken from the process-wide queue: a suite
/// running every test at once drained this test's word into another's and
/// handed it another's (2026-10-07, red once in 1 414).
#[test]
fn transcribe_these_spells_taught_turn_as_word() {
    use crate::transducer::sentence::compose::teaches;
    use crate::transducer::sentence::rung::Taught;
    use crate::transducer::sentence::theorem::Theorem;
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("taught"));
    let one = num_rational::BigRational::from_integer(1.into());
    let w = Mv::scalar(one.clone()).add(&Mv::blade(E2 | E3, one)).widened(3);
    let mut budget = e.stroke();
    let s = compose(&[Term { word: w.clone(), transport: w.reverse() }], &mut budget).expect("a sentence");
    let theorem = Theorem::of(&s);
    let word = teaches(&s, &theorem).expect("the theorem teaches").clone();
    let period = match s.closure {
        crate::transducer::sentence::compose::Closure::Closes(l) => l,
        _ => 1,
    };
    let (words, _) = e.transcribe_these(vec![Taught { motion: word, period }], &mut budget);
    assert!(!words.is_empty(), "the taught turn is spelled and handed back as a word to stand and speak");
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
fn transcribe_these_reports_ineffable_word() {
    use crate::transducer::sentence::rung::Taught;
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("ineffable"));
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let w = Mv::scalar(q(4)).add(&Mv::blade(E2 | E3, q(1))).widened(3);
    let mut budget = e.stroke();
    let (words, ineffable) = e.transcribe_these(vec![Taught { motion: w, period: 1 }], &mut budget);
    assert_eq!(ineffable.len(), 1, "its motion reported, so its sentence is spoken and its letter made");
    assert!(words.is_empty(), "no unspelled word is handed back as speech");
}

/// 78.109 K1, C1 — THE MIND CONSTRUCTS ITS LETTER AND SAYS ITS WORD. The
/// `(8, 15, 17)` turn is ineffable; the mind constructs its residue —
/// spelled as constructors, certified (`cga::construct`) — and adjoins its
/// rotation in its local dialect; the same word taught again is said by the
/// mind's own adjoined letter — no court asked, no ruling waited for.
#[test]
fn construct_and_adjoin_makes_ineffable_word_sayable() {
    use crate::transducer::sentence::rung::Taught;
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("sovereign"));
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let w = Mv::scalar(q(4)).add(&Mv::blade(E2 | E3, q(1))).widened(3);
    let mut budget = e.stroke();
    let (_, ineffable) = e.transcribe_these(vec![Taught { motion: w.clone(), period: 1 }], &mut budget);
    let chart = crate::transducer::stalk::Invariant::counts(&[1]);
    // Constructing reads the residue's modes under the stroke they need,
    // read off the residue (78.111 C1b): the mind's own stroke suffices.
    for residue in &ineffable {
        e.construct_and_adjoin(residue, Some(&chart), &[], &mut budget);
    }
    let (_, again) = e.transcribe_these(vec![Taught { motion: w, period: 1 }], &mut budget);
    assert!(again.is_empty(), "no longer ineffable: the mind holds its letter, and says it");
}

/// 78.68 ROW 8 — A RESIDUE THAT TURNS NOTHING BECOMES A LETTER. A dilation
/// about a point — the light from a star at a point, the residue the minds
/// carry most — constructs (78.114), and stands as the mind's MOTION letter:
/// its rotor the identity, its carriage the whole motion. The mind keeps it,
/// so the cell that demanded it is answered by it from here on.
#[test]
fn construct_and_adjoin_keeps_dilation_as_motion_letter() {
    use crate::transducer::cga::{at_three::translator, dilator};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let mut e = Entity::open("127.0.0.1:1", Kernel::named("star"));
    let t = translator(&q(1), &q(0), &q(3));
    let star = t.geo(&dilator(&q(2))).geo(&t.reverse());
    assert!(!e.contemplation.holds_own_letter(&star), "not held before it is made");
    let chart = crate::transducer::stalk::Invariant::counts(&[1]);
    let mut budget = e.stroke();
    e.construct_and_adjoin(&star, Some(&chart), &[], &mut budget);
    assert!(e.contemplation.holds_own_letter(&star), "the mind keeps the motion letter it made");
    let letter = crate::transducer::sentence::motion_letter_form(&star, &chart, &[]).expect("a letter");
    let bare = crate::transducer::sentence::generator_form(&crate::transducer::alphabet::Rotor::identity(), &chart, &[])
        .expect("the identity's letter");
    assert_ne!(letter.address(), bare.address(), "the motion rides the letter's carriage: it is its coordinate");
}
