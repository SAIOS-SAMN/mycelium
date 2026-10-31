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
fn compose_closes_at_holonomy_period() {
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
fn next_rung_teaches_translations_null_motion() {
    let t = cga::translator_at(&[q(1), q(0), q(0)]);
    let s = compose(&[Term { word: one(), transport: t.clone() }], &mut stroke()).expect("a sentence");
    assert_eq!(s.closure, Closure::Unbound, "a translation carries and never returns");
    // 78.104 — its word is the identity and its motion rides the transport:
    // the algebra closes over the holonomy too, so the slide spans one
    // direction (it read 0 while only the words were closed).
    assert_eq!(s.rank, Ok(1), "a ray's sentence spans its slide");
    let words = next_rung(&[s]);
    assert_eq!(words.len(), 1, "a nilpotent theorem teaches");
    assert!(words[0].projective_eq(&t), "its word is the null motion");
}

/// CONJUGATION CARRIES EVERY WORD INTO THE FIRST CHART: the second word,
/// written in its own chart as `g W g̃`, is read in the first chart as `W`
/// itself — the same motion seen from two charts is one generator (§1).
#[test]
fn compose_carries_every_word_into_first_chart() {
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
fn compose_reads_rank_of_carried_words() {
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
fn next_rung_teaches_closed_sentences_open_path() {
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
        refraction: None,
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
fn terms_of_crossing_composes_word_and_displacement() {
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
    assert_eq!(
        terms_of(&path_of(vec![crossing(true), unread])).err(),
        Some(crate::transducer::sentence::compose::TermUnread::StarUnread),
        "an unread crossing leaves the ring unread by its reason and its face — here a star's — never flat"
    );
    // A contact face with no transport read is a refraction refused.
    let mut refused = crossing(true);
    refused.transport = None;
    refused.whole = true;
    assert_eq!(
        terms_of(&path_of(vec![refused])).err(),
        Some(crate::transducer::sentence::compose::TermUnread::Unrefracted),
        "a contact whose refraction was refused reads unrefracted"
    );
    // A contact's carriage holds its refraction whole (BQ5a): that crossing is
    // its carriage alone — its word spells the same turn and is not counted
    // twice.
    let mut contact = crossing(true);
    contact.whole = true;
    let whole = terms_of(&path_of(vec![contact])).expect("read");
    assert!(omega_of(&whole).projective_eq(&tr.carriage), "a contact crossing is its whole carriage");
}

/// 78.108 (78.107 withdrawn) — A CONTACT THE MIND COULD NOT REFRACT IS
/// REFUSED ALIKE AT THE MIND AND AT THE COURT. Two members and a contact face
/// between them carrying an interface, no edge and no refraction: as the mind
/// holds it, the contact is a refused circuit (its construction reads it,
/// CT2); decoded from the wire it is the same form, and the court reads it
/// refused too — no static read is invented at the decode.
#[test]
fn sentences_of_refuses_unrefracted_contact_after_decode() {
    use super::common::{contact, stand_with_star, word_vertex_on};
    use crate::transducer::sentence::compose::{sentences_of, RingUnread, TermUnread};
    use crate::transducer::sentence::Form;
    let mut f = Form::default();
    let ca = stand_with_star(&mut f, &word_vertex_on(1, crate::transducer::alphabet::Word::identity(), crate::transducer::stalk::Invariant::counts(&[1])));
    let cb = stand_with_star(&mut f, &word_vertex_on(2, crate::transducer::alphabet::Word::identity(), crate::transducer::stalk::Invariant::counts(&[2])));
    let o = f.place_overlap(ca, cb, None).expect("a face between the two members");
    f.faces.at_mut(&o).expect("the face").interface = Some(contact(0, 3, 1));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    let rings_of = |f: &Form| -> Vec<crate::transducer::sentence::Orbit> {
        let cycles = Form::cycles_of(&f.faces, &f.vertices);
        cycles.kept.iter().chain(&cycles.vacua).cloned().collect()
    };
    let unrefracted = |read: &[(Vec<crate::network::nerve::coord::Overlap>, Result<crate::transducer::sentence::compose::Composition, RingUnread>)]| {
        read.iter().any(|(_, r)| matches!(r, Err(RingUnread::Transport(TermUnread::Unrefracted))))
    };
    let held = sentences_of(&f, &rings_of(&f), &mut stroke());
    assert!(unrefracted(&held), "as the mind holds it, the contact is refused — its construction reads it");
    let spoken = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("the form crosses");
    let heard = crate::transducer::sentence::from_element_form(&spoken).expect("and decodes at the court");
    let rings = rings_of(&heard);
    assert!(!rings.is_empty(), "the contact closes a ring with the two stars");
    let read = sentences_of(&heard, &rings, &mut stroke());
    assert!(unrefracted(&read), "the court reads it refused, as the mind does");
}

/// 78.108 — THE REFRACTION CROSSES AS THE EDGE DOES. A contact face whose
/// refraction is a held coset of two candidates crosses the wire and is
/// decoded with the same candidates, whole: the court composes every route
/// the voice composed, never the static read alone.
#[test]
fn from_element_form_keeps_contact_refraction_whole() {
    use super::common::{contact, stand_with_star, word_vertex_on};
    use crate::transducer::sentence::{Form, Refraction, Transport};
    let mut f = Form::default();
    let ca = stand_with_star(&mut f, &word_vertex_on(1, crate::transducer::alphabet::Word::identity(), crate::transducer::stalk::Invariant::counts(&[1])));
    let cb = stand_with_star(&mut f, &word_vertex_on(2, crate::transducer::alphabet::Word::identity(), crate::transducer::stalk::Invariant::counts(&[2])));
    let o = f.place_overlap(ca, cb, None).expect("a face between the two members");
    let candidate = |d: i64| Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: cga::translator_at(&[q(d), q(0), q(0)]),
        gauge: Vec::new(),
    };
    let coset = Refraction::Coset(vec![candidate(1), candidate(-1)]);
    {
        let face = f.faces.at_mut(&o).expect("the face");
        face.interface = Some(contact(0, 3, 1));
        face.place.refraction = Some(coset.clone());
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    let spoken = crate::transducer::sentence::as_element_form(&f, [1, 0, 0, 0]).expect("the form crosses");
    let heard = crate::transducer::sentence::from_element_form(&spoken).expect("and decodes");
    let face = heard.faces.at(&o).expect("the face crossed");
    assert_eq!(face.place.refraction.as_ref(), Some(&coset), "the held coset crossed whole");
    let cycles = Form::cycles_of(&heard.faces, &heard.vertices);
    let rings: Vec<crate::transducer::sentence::Orbit> = cycles.kept.iter().chain(&cycles.vacua).cloned().collect();
    let read = crate::transducer::sentence::compose::sentences_of(&heard, &rings, &mut stroke());
    assert!(read.iter().filter(|(_, r)| r.is_ok()).count() >= 2, "every route the coset opens is composed at the court");
}

/// 78.6, 78.102 — A CONTACT WITH NO ONE TRANSPORT IS READ BY ITS REFRACTION.
/// A STATIC contact is its tether `V`, the whole motion at one frame, crossed
/// either way. A HELD COSET is never one term — no representative is chosen
/// — so a single sequence of terms is refused (`HeldCoset`) and the composer
/// keeps every route.
#[test]
fn terms_of_reads_static_tether_refuses_held_coset() {
    use crate::transducer::sentence::compose::{omega_of, terms_of, TermUnread};
    use crate::transducer::sentence::{HeldPose, Refraction, Transport};
    let tether = cga::translator_at(&[q(1), q(1), q(0)]);
    let mut static_contact = crossing(true);
    static_contact.transport = None;
    static_contact.whole = true;
    static_contact.refraction = Some(Refraction::Static { tether: tether.clone(), poses: [HeldPose::OneFrame, HeldPose::OneFrame] });
    let terms = terms_of(&path_of(vec![static_contact.clone()])).expect("a static contact is read");
    assert!(omega_of(&terms).projective_eq(&tether), "its term is its tether");
    let mut back = static_contact;
    back.forward = false;
    let there_and_back = terms_of(&path_of(vec![crossing(true), back])).expect("read either way");
    assert!(!omega_of(&there_and_back).is_zero(), "crossed backward by the tether's reverse");
    let candidate = |d: i64| Transport {
        word: crate::transducer::alphabet::Word::identity(),
        carriage: cga::translator_at(&[q(d), q(0), q(0)]),
        gauge: Vec::new(),
    };
    let mut coset = crossing(true);
    coset.transport = None;
    coset.whole = true;
    coset.refraction = Some(Refraction::Coset(vec![candidate(1), candidate(-1)]));
    assert_eq!(terms_of(&path_of(vec![coset])).err(), Some(TermUnread::HeldCoset), "a held coset is never one term");
}
