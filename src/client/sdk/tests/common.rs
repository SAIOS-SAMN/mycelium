// Fixture constructors for the client's tests (MU-STANDARDS 3.3): a closed
// turn form and a two-body sentence.


pub fn closed_turn(contact: Option<crate::transducer::alphabet::Contact>) -> crate::transducer::sentence::Form {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{Form, FormFace, Vertex};
let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
let w = Word::from_letters(&[(q, false)]);
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
f.vertices.push(Vertex::Word {
    address: crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 1]),
    word: w,
    carriage: Box::new(crate::transducer::element::carriage(&one, &(zero.clone(), zero))),
    gauge: vec![(1, 2)],
    class: crate::transducer::stalk::Invariant::default(),
    contact,
});
f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
f.loops = crate::transducer::sentence::Form::loops_of(&f.faces, f.vertices.len());
f
}

pub fn two_body_sentence(gauges: [&[(u8, u8)]; 2]) -> crate::transducer::element::Element {
use crate::transducer::alphabet::Word;
use crate::transducer::sentence::{as_element_form, Form, FormFace, Vertex};
let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
let mut f = Form::default();
for (i, g) in gauges.iter().enumerate() {
    f.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, i as u8 + 1]),
        word: Word::identity(),
        carriage: Box::new(crate::transducer::element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        gauge: g.to_vec(),
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    let j = (i as u16) + 1;
    f.faces.push(FormFace {
        i: 0,
        j,
        transport: f.face_of(0, j).expect("star"),
    });
}
f.loops = Form::loops_of(&f.faces, f.vertices.len());
as_element_form(&f, [1, 0, 0, 0]).expect("v2 sentence")
}
