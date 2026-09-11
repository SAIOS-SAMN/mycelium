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

/// NO GAUGE. Every target in this file is a bare plane versor measured
/// with no hue map beside it, so the `𝒢` lane is unoccupied — which is the
/// truth about these targets, not a placeholder. The claims here are about
/// the presentation on `𝒽`, which is the only lane they occupy.
const NO_GAUGE: &[(u8, u8)] = &[];

/// THE CIPHER'S CASE, kept as the boundary it is: a mind that holds
/// nothing has no presentation to factor through, so the word is the
/// residue A22 peeled from the identity and the derivation cites nobody.
#[test]
fn a_voice_that_holds_nothing_spells_from_the_identity() {
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let d = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(d.is_spelled(), "no citation, because nothing is held");
    assert!(d.rotor().projective_eq(&quarter(2).rotor()), "and it reaches the vertex");
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
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
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
        d.rotor().projective_eq(&quarter(2).rotor()),
        "and the derivation composes to the same vertex the cipher reached"
    );
}

/// 1.5 and 13.10 on the wire: two minds whose presentations differ read
/// one measured versor as two derivations and ONE vertex. The address
/// cannot tell them apart and must not; the derivations can.
#[test]
fn two_presentations_read_one_versor_as_two_derivations_of_one_vertex() {
    let v = element::versor_of_word(&quarter(2)).expect("a plane versor");
    let a = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    let b = factor_versor(&Holds::of(quarter(3), 3), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert_ne!(a.generators, b.generators, "two presentations, two proofs");
    assert!(
        a.rotor().projective_eq(&b.rotor()),
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
    let a = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    let bare = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    // Re-read by the first mind after the second: no root was consulted
    // and no entry was overwritten, because there is no entry.
    let again = factor_versor(&Holds::of(quarter(1), 1), &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the peel closes");
    assert!(bare.is_spelled(), "the mind that holds nothing still spells");
    assert_eq!(a.generators, again.generators, "and the mind that holds one still cites it");
}

/// A generator the ALPHABET DOES NOT SPELL, as a court adjoins one
/// (37.7): a word vertex that cites nothing, spells nothing, and carries
/// `π_p = a + b·i` as the rotor `a + b·e01`. Its block's arc is `π_p²`,
/// of norm `p²`, which is the action a versor carrying that prime stands
/// on.
fn letter_of(a: i64, b: i64, tag: u8) -> (Address, Form) {
    use crate::transducer::alphabet::{Derivation, Rotor};
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let rotor = Rotor::scalar(q(a)).add(&Rotor::blade(0b0011, q(b)));
    let address = Address::from_bytes([TAG_FORM, tag]);
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address,
        derivation: Derivation::adjoined(rotor),
        carriage: Box::new(element::identity_versor()),
        gauge: Vec::new(),
        class: Invariant::default(),
        contact: None,
    });
    // No star face: the face would be the letter's own action, which the
    // seventeen cannot spell (L11 (2/2)).
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    (Address::from_bytes([TAG_FORM, tag]), f)
}

impl Holds {
    /// A mind taught an adjoined generator, filling the same index a
    /// taught form fills.
    fn of_letter(a: i64, b: i64, tag: u8) -> Holds {
        let (address, form) = letter_of(a, b, tag);
        let mut index = crate::transducer::sentence::GeneratorIndex::new();
        crate::transducer::sentence::admit_generators(&mut index, &address, &form);
        Holds { index, form }
    }
}

/// The versor whose sensor block is the arc `s + t·i`.
fn versor_of_arc(s: i64, t: i64) -> crate::transducer::element::Versor {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    // A rotor `x + y·e01` has block arc `(x² − y²) + 2xy·i`, so the arc
    // `s + t·i` is reached from its square root in ℤ[i].
    let r = Rotor::scalar(q(s)).add(&Rotor::blade(0b0011, q(t)));
    element::versor_of_word(&crate::transducer::alphabet::Word::formed(r))
        .expect("a plane versor")
}

/// L11 (3/3) — THE THIRD FACT: a mind taught a generator the seventeen
/// cannot spell DERIVES THROUGH IT. A12 over letter exponents cannot see
/// 17 however many letters the mind holds; the ring divides it out of the
/// sensor block exactly and cites it with its multiplicity.
///
/// The versor here is `π_17` itself as a rotor, whose block arc is
/// `π_17²`: the generator divides it once and nothing is left, so the
/// residue is the identity.
#[test]
fn a_mind_taught_an_adjoined_generator_derives_through_it() {
    let held = Holds::of_letter(4, 1, 7);
    let v = versor_of_arc(4, 1);
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it where the peel could not");
    assert!(!d.is_spelled(), "it cites the letter it was taught");
    assert_eq!(d.generators.len(), 1, "one generator");
    assert_eq!(
        d.generators.first().map(|(_, _, e)| *e),
        Some(1),
        "applied once: the target IS the generator"
    );
    assert!(d.residue.is_identity(), "and nothing is left for the alphabet");
}

/// The same generator at multiplicity two: `π_17²` as a rotor, whose
/// block arc is `π_17⁴`. The ring divides twice and the exponent says so.
#[test]
fn the_ring_cites_a_generator_at_its_full_multiplicity() {
    let held = Holds::of_letter(4, 1, 7);
    // (4 + i)² = 15 + 8i.
    let v = versor_of_arc(15, 8);
    let d = factor_versor(&held, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it");
    assert_eq!(
        d.generators.first().map(|(_, _, e)| *e),
        Some(2),
        "full multiplicity, divided out to exhaustion"
    );
    assert!(d.residue.is_identity(), "and nothing is left over");
}

/// A mind that does NOT hold the letter still cannot say it: the ring has
/// nothing to divide by, and the demand the peel typed stands. This is
/// what makes the two tests above a fact about the LANGUAGE and not about
/// the arithmetic.
#[test]
fn a_mind_without_the_letter_still_demands_it() {
    let v = versor_of_arc(4, 1);
    let demanded = factor_versor(&NoCrystal, &mut budget(), &v, NO_GAUGE)
        .expect("a linear block")
        .expect_err("the seventeen cannot spell it");
    assert!(
        matches!(demanded, crate::transducer::sentence::Demand::RotorPrime { prime: 17, .. }),
        "and the demand names the prime: {demanded:?}"
    );
}

/// A court holding the letter COMPOSES the citation (L14): the resolver
/// answers the generator's action, so a citation of a word the alphabet
/// cannot spell verifies exactly as one it can.
#[test]
fn a_court_holding_the_letter_composes_a_citation_of_it() {
    use crate::transducer::alphabet::Verified;
    let held = Holds::of_letter(4, 1, 7);
    let (letter_addr, letter_form) = letter_of(4, 1, 7);
    let d = factor_versor(&held, &mut budget(), &versor_of_arc(15, 8), NO_GAUGE)
        .expect("a linear block")
        .expect("the ring reaches it");
    let cover = |form: &Address, vertex: &Address| -> Option<crate::transducer::alphabet::Rotor> {
        match form == &letter_addr {
            true => crate::transducer::sentence::rotor_at_vertex(&letter_form, vertex.as_bytes()),
            false => None,
        }
    };
    assert_eq!(d.verify_with(&cover), Verified::Composes, "the court re-derives and agrees");
    assert_eq!(
        d.verify_with(&|_: &Address, _: &Address| None),
        Verified::Unresolved { form: letter_addr.clone(), vertex: letter_addr },
        "a court that has not ruled it waits, as it does for any citation"
    );
}

/// The chart read reaches it: a letter is filed at its own versor chart
/// and a versor carrying that prime finds it, because `generators_under`
/// peels prefixes and the two charts share one. Asserted, because the
/// whole ring branch rests on the mind being ABLE to offer the letter.
#[test]
fn a_versor_carrying_the_prime_finds_the_letter_at_its_chart() {
    use crate::transducer::alphabet::Rotor;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let held = Holds::of_letter(4, 1, 7);
    let target = Rotor::scalar(q(15)).add(&Rotor::blade(0b0011, q(8)));
    let chart = crate::transducer::sentence::versor_chart_of(&target).expect("a chart");
    assert_eq!(
        Crystal::generators_under(&held, &chart, &mut budget()).len(),
        1,
        "the mind offers the letter to a versor whose norm the prime divides"
    );
}
