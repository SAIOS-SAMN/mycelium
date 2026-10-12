// 78.69, 78.75 — A MIND SPEAKS ITS RINGS BY THE MASTER EQUATION: a real
// percept's rings composed and named, the rung recursing on closed sentences,
// and a sentence heard again recognized — comprehension, live.

use super::common::grid;
use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::compose::{compose, terms_of, Closure, Term};
use crate::transducer::sentence::rung::{hear, lexicon};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The alphabet's (3, 4, 5) letter on the plane `a ∧ b`, as its rotor ray:
/// `3 + 4 e_ab` (the letter `3/5 + 4/5 e_ab`).
fn turn(a: usize, b: usize) -> Mv {
    Mv::scalar(q(3, 1)).add(&Mv::blade(a | b, q(4, 1)))
}

/// A REAL PERCEPT'S RINGS, COMPOSED AND NAMED: every ring of a slide family
/// is composed by the master equation, and each reads OPEN — it never
/// returns — with the same modes as the circuit's former carriage-only
/// composite. 78.97: its seen edges are luminal rays, so the one mode is
/// PARABOLIC and the ring is a nilpotent theorem (the next test), where the
/// transvection of 54.7 R3 had read it a boost, unbound with no theorem.
#[test]
fn a_percepts_rings_are_composed_and_each_named() {
    let slides = vec![
        (grid(&["1100000", "0000000", "0002000", "0002200"]), grid(&["1100000", "0000000", "0000020", "0000022"])),
        (grid(&["0000110", "0000000", "0200000", "0220000"]), grid(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    let form = rule.form;
    let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
    let rings: Vec<_> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    assert!(!rings.is_empty(), "the slide's form holds rings");
    let mut read = 0;
    for lp in &rings {
        let Some(paths) = crate::transducer::sentence::circuit::paths_carried(&form.faces, &form.vertices, lp) else { continue };
        for (_, p) in paths.scoped() {
            let terms = terms_of(p).expect("every crossing of the slide's rings reads");
            let s = compose(&terms, &mut stroke()).expect("a sentence");
            assert_eq!(s.closure, Closure::Unbound, "the dislocation never returns: open, by name");
            let kinds = |m: &Mv| match cga::motion_modes(m, &mut stroke()) {
                Some(Ok(ms)) => ms.iter().map(|x| x.kind).collect::<Vec<_>>(),
                other => panic!("modes read: {other:?}"),
            };
            let old = p.edge.as_ref().map(|e| e.carriage.geo(&p.face.carriage)).expect("an edge half");
            assert_eq!(kinds(&s.omega), kinds(&old), "the one product reads the composite's modes");
            read += 1;
        }
    }
    assert!(read > 0, "rings were read");
}

/// 78.97 — A CHORD IS HEARD, NOT TRAVERSED. On the same slide family's real
/// percept, the composer leaves every ring that crosses a chord (an overlap
/// between two sections' pieces) as HEARD and composes the rest from their
/// rays alone; no ring is both.
#[test]
fn a_ring_through_a_chord_is_heard_and_the_rest_are_rays() {
    use crate::transducer::sentence::compose::{is_chord, sentences_of, RingUnread};
    let slides = vec![
        (grid(&["1100000", "0000000", "0002000", "0002200"]), grid(&["1100000", "0000000", "0000020", "0000022"])),
        (grid(&["0000110", "0000000", "0200000", "0220000"]), grid(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    let form = rule.form;
    let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
    let rings: Vec<_> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    let (mut heard, mut rays) = (0, 0);
    for (steps, read) in sentences_of(&form, &rings, &mut stroke()) {
        match steps.iter().any(is_chord) {
            true => {
                assert_eq!(read.err(), Some(RingUnread::Heard), "a chord's ring is heard");
                heard += 1;
            }
            false => {
                assert_ne!(read.err(), Some(RingUnread::Heard), "a ring of rays is never heard");
                rays += 1;
            }
        }
    }
    assert!(rays > 0, "the slide's form holds rings of rays: heard {heard}");
}

/// 78.97 — A RING OF SEEN RAYS CONDENSES. Each seen edge's displacement is
/// its translator, a null generator, so the slide family's rings of rays are
/// NILPOTENT theorems — `Ωᵐ ∝ 1 + m·N`, one integer times one direction —
/// where the boost left them unbound demands; and a nilpotent theorem teaches
/// its open path as a word, spelled (the slide turns nothing).
#[test]
fn a_ring_of_seen_rays_is_a_nilpotent_theorem_and_teaches_a_word() {
    use crate::transducer::sentence::compose::sentences_of;
    use crate::transducer::sentence::theorem::{Certificate, Theorem};
    let slides = vec![
        (grid(&["1100000", "0000000", "0002000", "0002200"]), grid(&["1100000", "0000000", "0000020", "0000022"])),
        (grid(&["0000110", "0000000", "0200000", "0220000"]), grid(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    let form = rule.form;
    let cycles = crate::transducer::sentence::Form::cycles_of(&form.faces, &form.vertices);
    let rings: Vec<_> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    let mut theorems = 0;
    for (_, read) in sentences_of(&form, &rings, &mut stroke()) {
        let Ok(s) = read else { continue };
        let t = Theorem::of(&s).expect("a ring of rays is a theorem");
        assert!(matches!(t.certificate, Certificate::Nilpotent { .. }), "a slide condenses: {:?}", s.closure);
        hear(&s, &mut stroke());
        theorems += 1;
    }
    assert!(theorems > 0, "the slide's rings were read");
}

/// 78.68, 78.99 — A SENTENCE THAT NEVER RETURNS AND DOES NOT CONDENSE IS
/// NAMED AND HELD. A dilation by 2 is hyperbolic: its sentence is open, no
/// certificate stands, and its holonomy is held as a word by name.
#[test]
fn a_hyperbolic_sentence_is_named_and_held() {
    let one = Mv::scalar(q(1, 1));
    let d = cga::dilator(&q(2, 1));
    let s = compose(&[Term { word: d, transport: one }], &mut stroke()).expect("a sentence");
    assert!(!s.closure.closes(), "a dilation never returns: {:?}", s.closure);
    hear(&s, &mut stroke());
    assert!(
        crate::transducer::sentence::rung::held_rung(&s.omega).is_some(),
        "its holonomy is held as a word, by name"
    );
}

/// MU-CLIENT §4.2 — TWO MINDS ON ONE THREAD KEEP TWO FIELDS. A mind hears a
/// dilation's sentence in its own field: it holds the word and is taught it,
/// and a cover it parks stays with it. A second mind thinking on the same
/// thread holds none of it and was taught nothing. The first, thinking again,
/// still holds the word, its teaching already taken, its cover still parked.
#[test]
fn two_minds_on_one_thread_keep_two_fields() {
    use crate::transducer::family::cursor;
    use crate::transducer::mind::MindField;
    use crate::transducer::sentence::rung::{held_rung, take_taught};
    let d = cga::dilator(&q(3, 1));
    let s = compose(&[Term { word: d, transport: Mv::scalar(q(1, 1)) }], &mut stroke()).expect("a sentence");
    let (mut a, mut b) = (MindField::default(), MindField::default());
    a.think(|| {
        hear(&s, &mut stroke());
        assert!(held_rung(&s.omega).is_some(), "the first mind holds the word");
        assert_eq!(take_taught().len(), 1, "and is taught it");
        cursor::park(b"a cover", cursor::Pending::new());
    });
    b.think(|| {
        assert!(held_rung(&s.omega).is_none(), "the second mind holds nothing the first heard");
        assert!(take_taught().is_empty(), "and was taught nothing");
        assert_eq!(cursor::parked(), 0, "nor holds the first's cover");
    });
    a.think(|| {
        assert!(held_rung(&s.omega).is_some(), "the first mind still holds its word");
        assert!(take_taught().is_empty(), "its teaching taken once");
        assert!(cursor::take(b"a cover").is_some(), "its cover parked with it");
    });
}

/// 78.99 (1) — COMPREHENSION BY COORDINATES. A sentence teaches the 90° turn
/// of the `e₁e₃` plane, held with its hole and its winding; a second teaches
/// the turn's square — a new key, but on the same hole at twice the winding —
/// and it is recognized as the held word's power: counted, never held.
#[test]
fn a_held_words_power_is_recognized_by_its_coordinates() {
    let w = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E3, q(1, 1)));
    let w2 = w.geo(&w);
    let said = |x: &Mv| compose(&[Term { word: x.clone(), transport: x.reverse() }], &mut stroke()).expect("a sentence");
    hear(&said(&w), &mut stroke());
    hear(&said(&w2), &mut stroke());
    assert!(crate::transducer::sentence::rung::held_rung(&w2).is_none(), "the square is the held turn's power: never held");
}

/// THE RUNG RECURSES, AND RE-HEARING IS COMPREHENSION. A sentence carrying the
/// (3, 4, 5) turn `W` and back closes at once; its shortcut `W` is held as a
/// word of rung 1, spelled. A second sentence whose first term IS that word
/// closes too, and its shortcut is held a rung higher. Heard again, the first
/// sentence re-derives its shortcut — recognized, held once.
#[test]
fn the_rung_recurses_and_a_sentence_heard_again_is_recognized() {
    let w = turn(E1, E2);
    let v = turn(E2, E3);
    let one = Mv::scalar(q(1, 1));
    let first = compose(&[Term { word: w.clone(), transport: w.reverse() }], &mut stroke()).expect("a sentence");
    assert_eq!(first.closure, Closure::Closes(1), "there and back: closed");
    assert!(first.path.projective_eq(&w), "its shortcut is the turn");
    hear(&first, &mut stroke());
    let rung_of_w = crate::transducer::sentence::rung::held_rung(&w);
    assert!(rung_of_w >= Some(1), "held at rung 1 (or already held)");
    let second = compose(
        &[Term { word: w.clone(), transport: one }, Term { word: v.clone(), transport: v.geo(&w).reverse() }],
        &mut stroke(),
    )
    .expect("a sentence");
    assert_eq!(second.closure, Closure::Closes(1), "a sentence of the held word, closed");
    assert!(second.words[0].projective_eq(&w), "its first term's word is the held word");
    hear(&second, &mut stroke());
    assert!(
        crate::transducer::sentence::rung::held_rung(&second.path) >= Some(2),
        "its shortcut stands a rung higher: the grammar on its own closures"
    );
    let held = lexicon();
    hear(&first, &mut stroke());
    assert_eq!(crate::transducer::sentence::rung::held_rung(&w), rung_of_w, "heard again: re-derived, recognized");
    let now = lexicon();
    assert_eq!(now, held, "and never held twice");
}

/// 78.105 N1 — A HELD WORD IS RECOGNIZED WHATEVER SPACE SURROUNDS IT. A
/// sentence teaches the turn `w`, held at its rung; a new sentence says `w`
/// again with a slide as the space after it. Its term's word is `w` and its
/// space the slide — read apart, the word is the held one and the sentence
/// stands a rung above it. The fused product `slide · w` is no held word:
/// read as one, the space hid the word.
#[test]
fn a_held_word_is_recognized_whatever_space_surrounds_it() {
    use crate::transducer::sentence::rung::rung_in_lexicon;
    let w = turn(E1, E3);
    let first = compose(&[Term { word: w.clone(), transport: w.reverse() }], &mut stroke()).expect("a sentence");
    hear(&first, &mut stroke());
    let slide = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let again = compose(&[Term { word: w.clone(), transport: slide.clone() }], &mut stroke()).expect("a sentence");
    assert!(again.words[0].projective_eq(&w), "the word is said, its space the slide");
    assert!(rung_in_lexicon(&again).expect("read") >= 2, "the held word lifts the sentence a rung, its space notwithstanding");
}

/// 78.100 D4 — DEPTH IS READ BY CITATION. A sentence teaches a turn of the
/// `e₂e₃` plane — `½ + e₂₃`, its point `cos θ = −3/5`, a rosette named by
/// that point — held at its rung. A later sentence teaches a screw — that
/// turn and a slide along its axis. Deconstructed by its
/// motion, the screw cites the held turn on the turn's hole and leaves the
/// slide as its residue, so it is new and held — one rung above the turn it
/// cites, though no word of its own sentence was held.
#[test]
fn a_word_stands_a_rung_above_the_words_it_cites() {
    use crate::transducer::sentence::rung::held_rung;
    let half = q(1, 2);
    let third = Mv::scalar(half.clone()).add(&Mv::blade(E2 | E3, q(1, 1)));
    let taught = compose(&[Term { word: third.clone(), transport: third.reverse() }], &mut stroke()).expect("a sentence");
    hear(&taught, &mut stroke());
    let below = held_rung(&third).expect("the turn is held");
    let slide = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let screw = third.geo(&slide);
    let again = compose(&[Term { word: screw.clone(), transport: screw.reverse() }], &mut stroke()).expect("a sentence");
    hear(&again, &mut stroke());
    let rung = held_rung(&screw).expect("the screw is new in its residue, and held");
    assert!(rung > below, "it stands above the word it cites: {rung} over {below}");
}
