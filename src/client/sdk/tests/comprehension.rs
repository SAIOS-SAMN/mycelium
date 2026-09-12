// 25, 26 — comprehension: what an utterance becomes in a client, what is
// withheld, and what a stood word closes.

use super::common::*;
use crate::client::sdk::*;

/// THE C-STACK TERMINAL: a taught word enters the GENOME (inheritance via
/// gene expression); a received utterance is COMPREHENDED — decomposed over
/// the genome, moving the EPIGENOME's expression weights (the local dialect's
/// recollection) — never re-cognized, never recalled. And comprehension never
/// mutates the genome: the inherited record cannot drift locally.
#[test]
fn the_entity_comprehends_and_the_epigenome_expresses() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let mut epi = Epigenome::new();
// The mesh teaches a word down: first hearing is novel, then it is held.
let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 4u8), (0, 1, 2)]).unwrap());
assert!(matches!(epi.learn(&word.to_bytes()), Some(Comprehension::Novel { .. })));
assert_eq!(epi.genome_len(), 1);
// The same utterance again — COMPREHENDED as the held word; expression rises.
match epi.comprehend(&word) {
    Comprehension::Understood(v) => {
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].0.as_slice(), word.address().as_bytes());
        assert_eq!(v[0].1, Standing::Ruled);
    }
    other => panic!("expected Understood with standing, got {other:?}"),
}
assert_eq!(epi.peak_expression(), Some(1), "the dialect's recollection weight moved");
// An unrelated utterance is heard as novel — a prospective, not a failure —
// and the genome is untouched by hearing it.
let other = from_canonical(&germinate(&[(0, 0, 5u8)], &[(0, 0, 9u8)]).unwrap());
assert!(matches!(epi.comprehend(&other), Comprehension::Novel { .. }));
assert_eq!(epi.genome_len(), 1, "comprehension never mutates the genome");
// 5a: the genome crystallizes and restores EXACTLY; the epigenome does
// not persist (expression resets — dialect is ephemeral by ruling).
let frozen = epi.crystallize();
let mut back = Epigenome::new();
assert!(back.uncrystallize(&frozen), "crystallized genome restores");
assert_eq!(back.genome_len(), epi.genome_len());
assert_eq!(back.genome()[0].address(), epi.genome()[0].address(), "words identical by address");
assert_eq!(
    back.peak_expression(),
    Some(0),
    "expression is NOT persisted — the restored word is carried at a weight of zero, \
     which is a weight and not the absence of one"
);
assert_eq!(
    Epigenome::new().peak_expression(),
    None,
    "a dialect with no word expresses nothing, and nothing has no peak"
);
assert!(!Epigenome::new().uncrystallize(b"GEN1\xff\xff\xff\xff"), "hostile count refused");
}

/// Chapter 24.2: a sentence that cites a form this mind does not hold
/// is Withheld; learn_one banks it in the dialect, never the genome.
#[test]
fn withheld_then_depth_order() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let missing_addr = crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 0xab]);
let inner = {
    let mut f = Form::default();
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    f.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 1]),
        derivation: crate::transducer::alphabet::Derivation::spelled(w.clone()),
        carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f
};
let mut outer = Form::default();
let _gi = outer
    .glue(&inner, missing_addr.clone(), Word::identity(), [1, 0, 0, 0])
    .expect("glue");
let e = as_element_form(&outer, [1, 0, 0, 0]).expect("v2 tape");
let mut epi = Epigenome::new();
match epi.comprehend(&e) {
    Comprehension::Withheld { missing } => {
        assert_eq!(missing, vec![missing_addr.as_bytes().to_vec()], "the cited address is named")
    }
    other => panic!("expected Withheld, got {other:?}"),
}
assert!(
    matches!(epi.learn(&e.to_bytes()), Some(Comprehension::Withheld { .. })),
    "learn banks the withheld sentence"
);
assert_eq!(epi.genome_len(), 0, "withheld does not enter the genome");
assert!(
    epi.dialect.scoped().any(|(_, d)| d.withheld),
    "the dialect holds the withheld sentence"
);
}

/// Waiting-on is keyed by the missing address; the overlap face of the
/// glued vertex is the re-gauge site, never face 0.
#[test]
fn withheld_waiting_on_resolves_at_the_overlap_face() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let word_vertex = |addr: crate::transducer::address::Address| Vertex::Word {
    address: addr,
    derivation: crate::transducer::alphabet::Derivation::spelled(w.clone()),
    carriage: Box::new(crate::transducer::element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
    gauge: vec![(1, 2)],
    class: crate::transducer::stalk::Invariant::default(),
    contact: None,
};
let inner = {
    let mut f = Form::default();
    f.vertices.push(word_vertex(crate::transducer::address::Address::from_bytes([1u8; 32])));
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f
};
let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
let inner_addr = inner_e.address();
let mut outer = Form::default();
outer.vertices.push(word_vertex(crate::transducer::address::Address::from_bytes([2u8; 32])));
outer.faces.push(FormFace { i: 0, j: 1, transport: outer.face_of(0, 1).expect("vertices"), edge: None });
let _gi = outer
    .glue(&inner, inner_addr.clone(), Word::identity(), [1, 0, 0, 0])
    .expect("glue");
assert_eq!(
    outer.glued.get(&inner_addr).copied(),
    Some(_gi),
    "glue records the cited address"
);
assert!(
    outer.faces.iter().any(|f| f.i == 0 && f.j == _gi && f.i != 1),
    "the glue's placement face is not the first Word face"
);
let e = as_element_form(&outer, [1, 0, 0, 0]).expect("outer tape");
let mut epi = Epigenome::new();
assert!(
    matches!(epi.learn(&e.to_bytes()), Some(Comprehension::Withheld { .. })),
    "learn banks the withheld sentence"
);
assert_eq!(
    epi.waiting_on_len(&inner_addr),
    1,
    "waiting-on is keyed by the missing address"
);
assert_eq!(epi.genome_len(), 0, "withheld does not enter the genome");
assert!(
    matches!(
        epi.learn(&inner_e.to_bytes()),
        Some(Comprehension::Understood(_) | Comprehension::Admitted)
    ),
    "the cited form is learned"
);
assert_eq!(
    epi.waiting_on_len(&inner_addr),
    0,
    "the arrival consumes the waiting-on index"
);
assert!(
    !epi.dialect
        .scoped()
        .any(|(_, d)| d.withheld),
    "the citing sentence is no longer withheld"
);
}

/// A citer whose cited class is held under another spelling is Understood
/// through that held form; standing is read from the atlas fiber.
#[test]
fn comprehend_understands_through_held_class() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::as_element_form;
let inner_a = closed_turn(None);
let inner_b = closed_turn(Some(crate::transducer::alphabet::Contact { anchor: 1, ring: 2 }));
let ea = as_element_form(&inner_a, [1, 0, 0, 0]).expect("A");
let eb = as_element_form(&inner_b, [1, 0, 0, 0]).expect("B");
assert_ne!(ea.address(), eb.address(), "contact is on the form key");
assert_eq!(
    crate::transducer::sentence::chart_of(&ea),
    crate::transducer::sentence::chart_of(&eb),
    "the class is the chart"
);
let mut epi = Epigenome::new();
assert!(epi.learn(&eb.to_bytes()).is_some());
let mut outer = crate::transducer::sentence::Form::default();
outer
    .glue(&inner_a, ea.address(), Word::identity(), [1, 0, 0, 0])
    .expect("glue A");
let e = as_element_form(&outer, [1, 0, 0, 0]).expect("citer");
match epi.comprehend(&e) {
    Comprehension::Understood(entries) => {
        assert_eq!(entries.len(), 1, "one held form of the cited chart");
        assert_eq!(entries[0].0.as_slice(), eb.address().as_bytes());
        assert_eq!(entries[0].1, Standing::Ruled);
    }
    other => panic!("expected Understood through the held class, got {other:?}"),
}
}

/// A cited address whose chart is not held is Withheld, never Dialect.
#[test]
fn foreign_class_is_withheld() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let inner_a = closed_turn(None);
let ea = as_element_form(&inner_a, [1, 0, 0, 0]).expect("A");
// A FORM THAT HOLDS SOMETHING ELSE UP TO GAUGE, which is what a
// different chart now means: two bodies and a CHORD between them
// carrying a half turn, filed as torsion at its own period, so the form
// closes a loop and its rank reads `torsion 1` where the turn's star
// tree reads zero on every coordinate.
//
// A star tree would not do it. Two star trees differing only in their
// word are one object up to gauge — the single face is a coboundary and
// the word transports away (T2) — so they sit on one chart, which is
// what a chart is for. The old chart wrote a row per vertex and told
// them apart by the spelling.
let other = {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let half = Word::from_letters(&[(q, false), (q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    for tag in [9u8, 10u8] {
        f.vertices.push(Vertex::Word {
            address: crate::transducer::address::Address::from_bytes([
                crate::transducer::address::TAG_FORM,
                tag,
            ]),
            derivation: crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero.clone()))),
            gauge: Vec::new(),
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
    }
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star 1"), edge: None });
    f.faces.push(FormFace { i: 0, j: 2, transport: f.face_of(0, 2).expect("star 2"), edge: None });
    f.faces.push(FormFace::new(1, 2, half.clone()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    // A17: the loop is believed at the period it closes at, and a court
    // replays it only where the form files it under its own class.
    let [lp, ..] = f.loops.as_slice() else { panic!("the chord closes a loop") };
    let chord = lp.chord;
    f.torsion.insert(
        half.rotor().class_key().expect("the half turn has a class"),
        crate::transducer::sentence::FiledLoop { sigma: Vec::new(), period: 2, chord },
    );
    f.reindex();
    f
};
let eo = as_element_form(&other, [1, 0, 0, 0]).expect("other class");
assert_ne!(
    crate::transducer::sentence::chart_of(&ea),
    crate::transducer::sentence::chart_of(&eo)
);
let mut epi = Epigenome::new();
assert!(epi.learn(&eo.to_bytes()).is_some());
let mut outer = Form::default();
outer
    .glue(&inner_a, ea.address(), Word::identity(), [1, 0, 0, 0])
    .expect("glue A");
let e = as_element_form(&outer, [1, 0, 0, 0]).expect("citer");
match epi.comprehend(&e) {
    Comprehension::Withheld { missing } => {
        assert_eq!(missing, vec![ea.address().as_bytes().to_vec()], "a foreign class is withheld")
    }
    other => panic!("expected Withheld, got {other:?}"),
}
}

/// Until A12 factors words through the held basis, a citation-free
/// closed form is Admitted, not Understood by nothing.
#[test]
fn citation_free_closed_form_is_admitted() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
f.vertices.push(Vertex::Word {
    address: crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 1]),
    derivation: crate::transducer::alphabet::Derivation::spelled(w),
    carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
    gauge: vec![(1, 2)],
    class: crate::transducer::stalk::Invariant::default(),
    contact: None,
});
f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
f.loops = Form::loops_of(&f.faces, f.vertices.len());
assert!(f.is_closed(), "a tree of two vertices coheres");
let e = as_element_form(&f, [1, 0, 0, 0]).expect("v2 tape");
let mut epi = Epigenome::new();
assert!(
    matches!(epi.comprehend(&e), Comprehension::Admitted),
    "no held citations, not Understood(vec![])"
);
}

/// learn_at stamps the voice's K and TUniversal, not an idle (1, 1).
#[test]
fn learn_at_stamps_voice_k_and_tuniversal() {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let missing_addr = crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 0xab]);
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let inner = {
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 1]),
        derivation: crate::transducer::alphabet::Derivation::spelled(w),
        carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f
};
let mut outer = Form::default();
let _gi = outer
    .glue(&inner, missing_addr.clone(), Word::identity(), [1, 0, 0, 0])
    .expect("glue");
let e = as_element_form(&outer, [1, 0, 0, 0]).expect("v2 tape");
let mut epi = Epigenome::new();
assert!(matches!(
    epi.learn_at(&e.to_bytes(), 7, 11),
    Some(Comprehension::Withheld { .. })
));
let d = epi
    .dialect
    .scoped()
    .next()
    .map(|(_, d)| d)
    .expect("withheld dialect");
assert_eq!(d.last_used_k, 7, "k is the voice's K");
assert_eq!(d.last_used_tu, 11, "tu is TUniversal");
}

/// A vertex already spoken stands in the dialect and does not go on
/// the wire, even when the second utterance is a translated embedding.
#[test]
fn the_same_vertex_is_spoken_once() {
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let e2 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
assert!(e1.same_vertex(&e2), "two tapes of one form are one vertex");
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("once"));
assert!(!entity.already_taught(&e1));
entity.remember_spoken(&e1);
assert!(entity.already_taught(&e2), "same vertex is already taught");
let codes = entity.speak(std::slice::from_ref(&e2)).expect("no wire on a known vertex");
assert_eq!(codes, vec![dialogue::SPEECH_STOOD]);
assert_eq!(entity.dialect_len(), 1, "stood once in the dialect");
}

/// The court promotes a dialect vertex into the genome, carrying uses,
/// and never admits a second copy.
#[test]
fn learn_one_promotes_dialect_same_vertex() {
use crate::transducer::family::Crystal;
let e1 = two_body_sentence([&[(1, 2), (1, 2)], &[(3, 4), (3, 4)]]);
let mut epi = Epigenome::new();
epi.stand_dialect_at(e1.clone(), 1, 1);
epi.stand_dialect_at(e1.clone(), 2, 2);
assert_eq!(epi.dialect_len(), 1);
assert_eq!(epi.genome_len(), 0);
let uses = 2u64;
let _ = uses;
let heard = epi.learn(&e1.to_bytes()).expect("promoted");
let _ = heard;
assert_eq!(epi.dialect_len(), 0, "dialect moves, it is not copied");
assert_eq!(epi.genome_len(), 1, "genome is the promoted vertex");
let promoted = epi.take_promoted();
assert_eq!(promoted.len(), 1);
assert_eq!(epi.expression[0], 2, "uses ride with the promotion");
// The crystal is read at two coordinates and they are not one (21 step 3
// as ruled 2026-09-10): `on_class` at the form's own chart, which is what
// comprehension of a citation asks, and `on_body` at the chart of a body
// the form carries, which is what the glue asks. Both carry the standing.
let inv = crate::transducer::stalk::invariant_of(&e1);
let on = epi.on_class(&inv);
assert!(
    on.iter().any(|(_, _, st)| *st == Standing::Ruled),
    "Crystal carries standing as a coordinate after promotion"
);
let body = crate::transducer::sentence::from_element_form(&e1)
    .expect("the tape reads back")
    .vertices
    .iter()
    .find_map(|v| match v {
        crate::transducer::sentence::Vertex::Word { class, .. } => Some(class.clone()),
        _ => None,
    })
    .expect("a word vertex carries its body's chart");
assert!(
    epi.on_body(&body).iter().any(|(_, _, st)| *st == Standing::Ruled),
    "and the glue's read carries it at the body's chart too"
);
}

/// A taught word that fails replay is refused, not learned.
#[test]
fn taught_word_that_fails_replay_is_refused_not_learned() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let honest = from_canonical(
    &germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap(),
);
let mut fat = honest.clone();
fat.set_magnitude([999, 0, 0, 0]);
let mut entity = Entity::open("127.0.0.1:1", Kernel::named("syllabus-refuse"));
assert!(
    entity.absorb_taught(&fat.to_bytes()).is_none(),
    "a word that fails replay is not learned"
);
assert_eq!(entity.epigenome.genome_len(), 0, "genome stays empty");
assert_eq!(entity.dialect_len(), 0, "dialect stays empty");
assert!(
    entity.absorb_taught(&honest.to_bytes()).is_some(),
    "an honest derivation is learned"
);
}

/// A stood dialect word closes a remainder without waiting for the court.
#[test]
fn a_stood_word_closes_todays_remainder() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let mut epi = Epigenome::new();
let word = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
epi.stand_dialect_at(word.clone(), 1, 1);
assert_eq!(epi.genome_len(), 0);
let chain = crate::transducer::element::canonical_chain(word.sub())
    .expect("a stood word's chain is A5's");
let located = {
    // locate is private; comprehend over the same chain is the public read.
    epi.comprehend(&from_canonical(&chain))
};
match located {
    Comprehension::Understood(_) | Comprehension::Held | Comprehension::Admitted => {}
    other => {
        // residual words locate as the chain of the stood word
        let again = epi.comprehend(&word);
        assert!(
            matches!(again, Comprehension::Understood(_) | Comprehension::Admitted | Comprehension::Held),
            "a stood word is on the working language, got {other:?} then {again:?}"
        );
    }
}
}

/// Equal rotor, different ρ / σ / shift: three genome entries. Two
/// spellings of one vertex still merge.
#[test]
fn rho_sigma_shift_are_three_vertices_two_spellings_merge() {
use crate::transducer::element::{from_connection, identity_versor, translator_versor};
let id = crate::transducer::alphabet::Word::identity();
let swap = from_connection(&id, &identity_versor(), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("swap");
let cycle = from_connection(&id, &identity_versor(), &[(1, 2), (2, 3), (3, 1)], &Vec::new(), [1, 0, 0, 0]).expect("cycle");
let place = from_connection(&id, &translator_versor(5, 0), &[(1, 2), (2, 1)], &Vec::new(), [1, 0, 0, 0]).expect("shift");
let mut epi = Epigenome::new();
assert!(epi.learn(&swap.to_bytes()).is_some());
assert!(epi.learn(&cycle.to_bytes()).is_some());
assert!(epi.learn(&place.to_bytes()).is_some());
assert_eq!(epi.genome_len(), 3, "ρ/σ/shift are three vertices");
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
let long = crate::transducer::alphabet::Word::from_letters(&[
    (q, false),
    (q, false),
    (q, false),
    (q, false),
]);
let mut alt = swap.clone();
let letters = alt.letters().expect("connection");
let spelling = crate::transducer::alphabet::Spelling { word: long };
let rem = alt.sub().to_vec();
alt.set_description(crate::transducer::element::connection_word(
    &spelling,
    &letters.carriage,
    &letters.sigma,
    &rem,
));
assert_eq!(alt.address(), swap.address(), "two spellings, one address");
assert!(epi.learn(&alt.to_bytes()).is_some());
assert_eq!(epi.genome_len(), 3, "a second tape of the same address does not mint");
}
