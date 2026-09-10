// 50.2 step 2 — F FACTORS THROUGH THE PRESENTATION. What a voice says
// depends on what it holds; a voice that holds nothing spells from the
// identity, which is the cipher's case and no longer the only one.

use crate::transducer::address::{Address, TAG_FORM};
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element;
use crate::transducer::family::{factor_versor, Crystal, NoCrystal};
use crate::transducer::sentence::{Form, FormFace, Vertex};
use crate::transducer::stalk::Invariant;

fn quarter(n: usize) -> Word {
    let q = LetterId::quarter((0, 1)).expect("the quarter turn on the sensor plane");
    Word::from_letters(&vec![(q, false); n])
}

/// A closed one-word sentence carrying `w`: a generator as the language
/// holds one since L5 — a `Word` vertex of a FORM, never a v1 connection.
fn sentence_of(w: Word, tag: u8) -> (Address, Form) {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let address = Address::from_bytes([TAG_FORM, tag]);
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address,
        derivation: crate::transducer::alphabet::Derivation::spelled(w),
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: Vec::new(),
        class: Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("star"), edge: None });
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    (Address::from_bytes([TAG_FORM, tag]), f)
}

/// A mind that holds one form, answering `generators_under` through the
/// SAME index the epigenome uses — `admit_generators` filling it and the
/// keyed subtree read draining it — so what this fixture proves about the
/// read is what the live mind does (§3.4).
struct Holds {
    index: crate::transducer::sentence::GeneratorIndex,
    form: Form,
}

impl Holds {
    fn of(w: Word, tag: u8) -> Holds {
        let (address, form) = sentence_of(w, tag);
        let mut index = crate::transducer::sentence::GeneratorIndex::new();
        crate::transducer::sentence::admit_generators(&mut index, &address, &form);
        Holds { index, form }
    }
}

impl Crystal for Holds {
    fn on_class(&self, _c: &Invariant) -> Vec<(crate::transducer::element::Element, Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn on_body(&self, _c: &Invariant) -> Vec<(crate::transducer::element::Element, Form, crate::transducer::stalk::Standing)> {
        Vec::new()
    }
    fn speak_form(
        &self,
        f: &Form,
        x: &crate::transducer::family::Grid,
        placement: &Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }
    fn by_address(&self, _a: &[u8]) -> Option<(crate::transducer::element::Element, Form)> {
        None
    }
    fn on_axis(&self, _c: &Invariant, _axis: crate::transducer::sentence::Axis) -> Vec<Word> {
        Vec::new()
    }
    fn generators_under(
        &self,
        chart: &Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(Address, Address, Word)> {
        crate::transducer::sentence::generators_under(&self.index, chart, budget)
            .into_iter()
            .filter_map(|(form, vertex)| {
                let w = crate::transducer::sentence::word_at_vertex(&self.form, vertex.as_bytes())?;
                Some((form, vertex, w))
            })
            .collect()
    }
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke()
}

/// THE CIPHER'S CASE, kept as the boundary it is: a mind that holds
/// nothing has no presentation to factor through, so the word is the
/// residue A22 peeled from the identity and the derivation cites nobody.
#[test]
fn a_voice_that_holds_nothing_spells_from_the_identity() {
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let d = factor_versor(&NoCrystal, &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(d.is_spelled(), "no citation, because nothing is held");
    assert!(d.word().rotor().projective_eq(&quarter(2).rotor()), "and it reaches the vertex");
}

/// 50.2 step 2, the whole claim of this row: the SAME measured versor,
/// read by a mind that holds the quarter turn, is a derivation THROUGH
/// that generator. A22 from the identity is a function of the versor
/// alone; this is not, and that is the difference between a language and
/// a cipher.
#[test]
fn a_voice_that_holds_a_generator_derives_through_it() {
    let held = Holds::of(quarter(1), 1);
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let d = factor_versor(&held, &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(!d.is_spelled(), "the mind factored through what it holds");
    assert_eq!(d.generators.len(), 1, "one generator, applied twice");
    assert_eq!(
        d.generators.first().map(|(f, _, _)| f.clone()),
        Some(Address::from_bytes([TAG_FORM, 1])),
        "cited by the address of the form the mind was taught"
    );
    assert!(d.residue.is_identity(), "the generator reached the whole word");
    assert!(
        d.word().rotor().projective_eq(&quarter(2).rotor()),
        "and the derivation composes to the same vertex the cipher reached"
    );
}

/// 1.5 and 13.10 on the wire: two minds whose presentations differ read
/// one measured versor as two derivations and ONE vertex. The address
/// cannot tell them apart and must not; the derivations can.
#[test]
fn two_presentations_read_one_versor_as_two_derivations_of_one_vertex() {
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let a = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    let b = factor_versor(&Holds::of(quarter(3), 3), &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    assert_ne!(a.generators, b.generators, "two presentations, two proofs");
    assert!(
        a.word().rotor().projective_eq(&b.word().rotor()),
        "one vertex, one address, one ledger entry"
    );
}

/// Rule 6 on the memo, asserted where it belongs — on the OUTPUT, not on
/// a key. The percept is memoised on sensor bytes and the derivation is
/// re-run per mind, so the same two grids read by two minds give two
/// derivations with no cache root anywhere in it. A memo that keyed the
/// presentation would let a digest decide whether F re-derives.
#[test]
fn one_memoised_percept_gives_each_mind_its_own_derivation() {
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let a = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    let bare = factor_versor(&NoCrystal, &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    // Re-read by the first mind after the second: no root was consulted
    // and no entry was overwritten, because there is no entry.
    let again = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(bare.is_spelled(), "the mind that holds nothing still spells");
    assert_eq!(a.generators, again.generators, "and the mind that holds one still cites it");
}
