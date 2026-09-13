//! 39 — FIXTURES for the sentence's tests: the two rationals, a form
//! address, a closed word form, a star, and the words of each order.

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

pub fn form_addr(n: u8) -> crate::transducer::address::Address {
    crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, n])
}

pub fn closed_word_form(word: Word, contact: Option<alphabet::Contact>) -> Form {
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::new(Vec::new()),
        alphabet::Derivation::spelled(word.clone()),
        stalk::Invariant(vec![3, 5]),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        vec![(1, 2)],
        contact,
    ));
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star"),
        edge: None,
    });
    f.remainder = crate::transducer::element::PlacedChain::default();
    f.depth = f.depth_of();
    let k = form_vertex_key(&f);
    if let Some(Vertex::Word { address, .. }) = f.vertices.get_mut(1) {
        *address = k;
    }
    f.reindex();
    f
}

pub fn word_vertex(addr: u8, word: Word) -> Vertex {
    word_vertex_on(addr, word, stalk::Invariant::default())
}

/// The same fixture on a NAMED chart. The chart is the key the member is
/// filed under, so a test that wants a vertex on a particular chart says
/// so when it builds the vertex; it cannot reach in and rewrite the key
/// afterwards, which is the whole of what keying by content means (law 2).
pub fn word_vertex_on(addr: u8, word: Word, chart: stalk::Invariant) -> Vertex {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    Vertex::word_on(
        form_addr(addr),
        alphabet::Derivation::spelled(word),
        chart,
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    )
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
            edge: None,
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
            edge: None,
        });
        f.faces.push(FormFace {
            i: 0,
            j: b,
            transport: f.face_of(0, b).expect("star b"),
            edge: None,
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
