// 39 — fixture constructors for the client's tests: a closed turn form and
// a two-body sentence.

/// THE FIXTURES' FRAME (AT1: a member holds its frames): every member a
/// client fixture stands, stands at frame 0.
const FRAME: u64 = 0;

pub fn closed_turn(contact: Option<crate::transducer::alphabet::Contact>) -> crate::transducer::sentence::Form {
use crate::network::nerve::coord;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{Form, Vertex};
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
let v = Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(w),
    crate::transducer::stalk::Invariant::default(),
    Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
    vec![(1, 2)],
    [FRAME],
    contact,
);
let c = f.stand_member(&v, FRAME).expect("a word vertex stands");
f.place_overlap(coord::ChartAt::Frame, c, None).expect("star");
f.orbits = crate::transducer::sentence::Form::orbits_of(&f.faces, &f.vertices);
f
}

/// TWO BODIES OF THE IDENTITY'S CLASS, each on its own gauge. AT1 (Q1): one
/// class is ONE vertex, and a member is keyed by its chart and carriage, so
/// two bodies with one chart and one carriage are one member whatever their
/// gauges. TWO BODIES ARE TWO SITES, and the site is the chart (the sentence
/// fixtures' `star_form` convention): body `i` stands on chart `[i + 1]`,
/// each overlapped from the Frame's chart.
pub fn two_body_sentence(gauges: [&[(u8, u8)]; 2]) -> crate::transducer::element::Element {
use crate::network::nerve::coord;
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, Vertex};
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
for (i, g) in gauges.iter().enumerate() {
    let j = u64::try_from(i).expect("a fixture star's body, one per gauge, a handful") + 1;
    let v = Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        crate::transducer::stalk::Invariant::counts(&[j]),
        Box::new(crate::transducer::element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        g.to_vec(),
        [FRAME],
        None,
    );
    let c = f.stand_member(&v, FRAME).expect("a word vertex stands");
    f.place_overlap(coord::ChartAt::Frame, c, None).expect("star");
}
f.orbits = Form::orbits_of(&f.faces, &f.vertices);
as_element_form(&f, [1, 0, 0, 0]).expect("v2 sentence")
}
