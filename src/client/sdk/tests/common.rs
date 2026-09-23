// 39 — fixture constructors for the client's tests: a closed turn form and
// a two-body sentence.


pub fn closed_turn(contact: Option<crate::transducer::alphabet::Contact>) -> crate::transducer::sentence::Form {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{Form, Vertex};
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
f.vertices.push(Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(w),
    crate::transducer::stalk::Invariant::default(),
    Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
    vec![(1, 2)],
    contact,
));
f.faces.push(f.face_of(0, 1).expect("vertices"));
f.orbits = crate::transducer::sentence::Form::orbits_of(&f.faces, &f.vertices);
f
}

pub fn two_body_sentence(gauges: [&[(u8, u8)]; 2]) -> crate::transducer::element::Element {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, Vertex};
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
for (i, g) in gauges.iter().enumerate() {
    f.vertices.push(Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        crate::transducer::stalk::Invariant::default(),
        Box::new(crate::transducer::element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        g.to_vec(),
        None,
    ));
    let j = (i as u16) + 1;
    f.faces.push(f.face_of(0, j).expect("star"));
}
f.orbits = Form::orbits_of(&f.faces, &f.vertices);
as_element_form(&f, [1, 0, 0, 0]).expect("v2 sentence")
}
