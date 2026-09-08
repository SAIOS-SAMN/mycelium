// Fixture constructors for the sentence's tests (MU-STANDARDS 3.3): the two
// rationals, a form address, a closed word form, a star, and the words of
// each order the class tests need.

use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

pub fn q0() -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(0))
}

pub fn q1() -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(1))
}

pub fn form_addr(n: u8) -> crate::transducer::a21::Address {
    crate::transducer::a21::Address::from_bytes([crate::transducer::a21::TAG_FORM, n])
}

pub fn closed_word_form(word: Word, contact: Option<alphabet::Contact>) -> Form {
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::a21::Address::new(Vec::new()),
        word: word.clone(),
        carriage: Box::new(element::carriage(&q1(), &(q0(), q1()))),
        gauge: vec![(1, 2)],
        class: stalk::Invariant(vec![3, 5]),
        contact,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star"),
    });
    f.remainder.clear();
    f.depth = f.depth_of();
    let k = form_vertex_key(&f);
    if let Some(Vertex::Word { address, .. }) = f.vertices.get_mut(1) {
        *address = k;
    }
    f.reindex();
    f
}

pub fn word_vertex(addr: u8, word: Word) -> Vertex {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    Vertex::Word {
        address: form_addr(addr),
        word,
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: vec![(1, 2)],
        class: stalk::Invariant::default(),
        contact: None,
    }
}

pub fn star_form(n_words: usize, extra: Vec<(u16, u16, Word)>) -> Form {
    let mut f = Form::default();
    for i in 0..n_words {
        f.vertices.push(word_vertex(i as u8 + 1, Word::identity()));
        let j = (i as u16) + 1;
        f.faces.push(FormFace {
            i: 0,
            j,
            transport: f.face_of(0, j).expect("star"),
        });
    }
    for (i, j, w) in extra {
        f.faces.push(FormFace::new(i, j, w));
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    f
}

pub fn order2() -> Word {
    alphabet::PlaneForm {
        orientation: true,
        rot: [0; alphabet::CAYLEY_ALPHABET_LEN],
    }
    .to_word()
}

pub fn order4() -> Word {
    Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)])
}

pub fn infinite() -> Word {
    Word::from_letters(&[(alphabet::LetterId::pythagorean_5((0, 1)).unwrap(), false)])
}

pub fn chord_form(chords: &[Word]) -> Form {
    let mut f = Form::default();
    for (ci, w) in chords.iter().enumerate() {
        let a = (ci * 2 + 1) as u16;
        let b = (ci * 2 + 2) as u16;
        f.vertices.push(word_vertex(a as u8, Word::identity()));
        f.vertices.push(word_vertex(b as u8, Word::identity()));
        f.faces.push(FormFace {
            i: 0,
            j: a,
            transport: f.face_of(0, a).expect("star a"),
        });
        f.faces.push(FormFace {
            i: 0,
            j: b,
            transport: f.face_of(0, b).expect("star b"),
        });
        f.faces.push(FormFace::new(a, b, w.clone()));
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    f
}

pub fn coboundary_chord_form() -> Form {
    let r = order4();
    let mut f = Form::default();
    f.vertices.push(word_vertex(1, Word::identity()));
    f.vertices.push(word_vertex(2, Word::identity()));
    f.faces.push(FormFace::new(0, 1, r.clone()));
    f.faces.push(FormFace::new(0, 2, Word::identity()));
    f.faces.push(FormFace::new(1, 2, r.inverse()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.reindex();
    f
}

pub fn cert(axis: Axis, mu: [u64; 4]) -> PivotCertificate {
    PivotCertificate {
        chord: 0,
        axis,
        before: Word::identity().class_key().expect("identity"),
        after: Word::identity().class_key().expect("identity"),
        rank_move: RankMove::Equal,
        mu: Mu::Measured {
            before: [0; 4],
            after: mu,
        },
        by: PivotBy::Word {
            face: 0,
            word: Word::identity(),
            side: Side::Right,
        },
    }
}
