// MU-CLIENT §5.4, 78.99 — A WORD READ BY ITS COORDINATES: every mode on its
// own law, and the word the product of its mode-words.

use crate::transducer::cga::{self, Mv, E1, E2};
use crate::transducer::sentence::word::{read, Law};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// The 90° turn of the `e₁e₂` plane, `1 + e₁₂`.
fn quarter() -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)))
}

/// EVERY MODE STANDS ON ITS OWN LAW: a turn of finite order is periodic, a
/// slide nilpotent, a turn of no finite order a rosette, a dilation a boost —
/// each read off the word's own hole, and each word of one mode reads as that
/// one mode-word, itself.
#[test]
fn every_mode_stands_on_its_own_law() {
    let laws = |m: &Mv| read(m).expect("its coordinates read").modes.iter().map(|w| w.law).collect::<Vec<_>>();
    assert_eq!(laws(&quarter()), vec![Law::Periodic], "a 90° turn returns at 4");
    assert_eq!(laws(&cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)])), vec![Law::Nilpotent], "a slide: one null motion per traversal");
    let rosette = Mv::scalar(q(-1, 2)).add(&Mv::blade(E1 | E2, q(1, 1)));
    assert_eq!(laws(&rosette), vec![Law::Rosette], "a turn at a point of no finite order turns forever");
    assert_eq!(laws(&cga::dilator(&q(2, 1))), vec![Law::Boost], "a dilation: a gain that never returns");
    let one = read(&quarter()).expect("reads");
    assert!(one.modes[0].word.projective_eq(&quarter()), "a word of one mode is its own mode-word");
}

/// A MIND THAT IS TAUGHT A WORD KNOWS ITS MODE-WORDS: its own perception
/// closing a sentence whose word is a screw teaches it the screw, and the
/// turn and the slide the screw is made of are words of its dictionary from
/// then on — in a mind it had never known them before.
#[test]
fn a_mind_taught_a_word_knows_its_mode_words() {
    use crate::transducer::sentence::compose::{compose, Term};
    use crate::transducer::sentence::rung::{hear, known};
    let along = cga::translator_at(&[q(0, 1), q(0, 1), q(1, 1)]);
    let screw = along.geo(&quarter());
    let one = Mv::scalar(q(1, 1));
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        assert!(!known(&quarter()) && !known(&along), "a fresh mind knows neither");
        let s = compose(&[Term { word: screw.clone(), transport: one.clone() }], &mut crate::physics::Physics::default().stroke_n(1 << 16)).expect("a ring");
        hear(&s, &mut crate::physics::Physics::default().stroke_n(1 << 16));
        assert!(known(&screw), "taught the screw");
        assert!(known(&quarter()), "and the turn it is made of");
        assert!(known(&along), "and the slide");
    });
}

/// A WORD IS THE PRODUCT OF ITS MODE-WORDS: a screw — the 90° turn carried
/// along the axis it turns about — reads as two, a periodic turn and a
/// nilpotent slide, and their product, in either order, is the screw.
#[test]
fn a_word_is_the_product_of_its_mode_words() {
    let along = cga::translator_at(&[q(0, 1), q(0, 1), q(1, 1)]);
    let screw = along.geo(&quarter());
    let r = read(&screw).expect("the screw's coordinates read");
    let laws: Vec<Law> = r.modes.iter().map(|m| m.law).collect();
    assert!(laws.len() == 2 && laws.contains(&Law::Periodic) && laws.contains(&Law::Nilpotent), "a turn and a slide: {laws:?}");
    let [a, b] = r.modes.as_slice() else { panic!("two modes") };
    assert!(a.word.geo(&b.word).projective_eq(&screw), "the product of its mode-words is the screw");
    assert!(b.word.geo(&a.word).projective_eq(&screw), "in either order: its modes commute");
    assert!(r.modes.iter().any(|m| m.word.projective_eq(&quarter())), "one mode-word is the 90° turn itself");
    assert!(r.modes.iter().any(|m| m.word.projective_eq(&along)), "the other the slide itself");
}
