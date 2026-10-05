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

/// A TRANSLATION NEVER RETURNS, BUT IT CONDENSES: a sentence carried by a
/// null transport is open, and — a nilpotent theorem, `Ωᵐ ∝ 1 + m·N` — it
/// teaches its holonomy, the one null motion (78.97, the one rule of 78.100
/// D2). A sentence whose modes do not read teaches nothing.
#[test]
fn a_translated_sentence_is_open_and_teaches_its_null_motion() {
    let t = cga::translator_at(&[q(1), q(0), q(0)]);
    let s = compose(&[Term { word: one(), transport: t.clone() }], &mut stroke()).expect("a sentence");
    assert_eq!(s.closure, Closure::Unbound, "a translation carries and never returns");
    let words = next_rung(&[s]);
    assert_eq!(words.len(), 1, "a nilpotent theorem teaches");
    assert!(words[0].projective_eq(&t), "its word is the null motion");
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

/// A crossing of one face carrying the (3, 4, 5) turn in the `r, c` plane and
/// a displacement, as the circuit records it.
fn crossing(forward: bool) -> crate::transducer::sentence::circuit::CarriedTransition {
    use crate::transducer::alphabet::{LetterId, Word};
    let word = Word::from_letters(&[(LetterId::pythagorean_5((0, 1)).expect("a pair plane"), true)]);
    let carriage = cga::translator_at(&[q(2), q(-1), q(0)]);
    crate::transducer::sentence::circuit::CarriedTransition {
        face: crate::network::nerve::coord::Overlap {
            from: crate::network::nerve::coord::ChartAt::Frame,
            to: crate::network::nerve::coord::ChartAt::Frame,
        },
        forward,
        transport: Some(crate::transducer::sentence::Transport { word, carriage, gauge: Vec::new() }),
        whole: false,
    }
}

fn path_of(transitions: Vec<crate::transducer::sentence::circuit::CarriedTransition>) -> crate::transducer::sentence::circuit::PathCarried {
    use crate::transducer::alphabet::Word;
    let flat = || crate::transducer::sentence::Transport { word: Word::identity(), carriage: one(), gauge: Vec::new() };
    crate::transducer::sentence::circuit::PathCarried {
        face: flat(),
        edge: Some(flat()),
        transitions,
        edge_refused: None,
        edge_lost_at: None,
        refracted: None,
    }
}

/// 78.75 — A RING'S CROSSING IS ONE MOTION: its term is the word's motion
/// `A(w)` and the displacement `c`, so the crossing composes to `c·A(w)` —
/// the member's own conformal realisation (`conformal_of`), the word kept,
/// never dropped as the carriage-only product dropped it. Crossed against
/// the face it is the exact inverse: forward then back returns the ring to
/// the identity, ℓ = 1. And the circuit's one reader (`composite`) is this
/// product.
#[test]
fn a_rings_crossing_is_one_motion_word_and_displacement_together() {
    use crate::transducer::sentence::compose::{omega_of, terms_of};
    let one_way = path_of(vec![crossing(true)]);
    let terms = terms_of(&one_way).expect("every transport read");
    let tr = crossing(true).transport.expect("held");
    let motion = crate::transducer::element::conformal_of(&tr.word, &tr.carriage).expect("a base action");
    assert!(omega_of(&terms).projective_eq(&motion), "the crossing is c·A(w), one motion");
    assert!(!omega_of(&terms).projective_eq(&tr.carriage), "the word is not dropped");
    assert_eq!(one_way.composite().map(|m| m.projective_eq(&motion)), Some(true), "the circuit's one reader is the master equation's product");
    let there_and_back = path_of(vec![crossing(true), crossing(false)]);
    let s = compose(&terms_of(&there_and_back).expect("read"), &mut stroke()).expect("a sentence");
    assert_eq!(s.closure, Closure::Closes(1), "a crossing and its reverse: the identity");
    let mut unread = crossing(true);
    unread.transport = None;
    assert!(terms_of(&path_of(vec![crossing(true), unread])).is_none(), "an unread crossing leaves the ring unread, never flat");
    // A contact's carriage holds its refraction whole (BQ5a): that crossing is
    // its carriage alone — its word spells the same turn and is not counted
    // twice.
    let mut contact = crossing(true);
    contact.whole = true;
    let whole = terms_of(&path_of(vec![contact])).expect("read");
    assert!(omega_of(&whole).projective_eq(&tr.carriage), "a contact crossing is its whole carriage");
}
