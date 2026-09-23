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
        alphabet::Derivation::spelled(word.clone()),
        stalk::Invariant::counts(&[3, 5]),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        vec![(1, 2)],
        contact,
    ));
    f.faces.push(f.face_of(0, 1).expect("star"));
    f.remainder = crate::transducer::element::PlacedChain::default();
    f.depth = f.depth_of();
    let k = form_vertex_key(&f);
    if let Some(Vertex::Word { address, .. }) = f.vertices.get_mut(1) {
        *address = k;
    }
    f.reindex();
    f
}

/// THE ONE MEMBER PAIR OF A FIXTURE'S FACE. Every vertex these fixtures build
/// stands at exactly one chart, so each of their faces relates exactly one pair
/// of bodies — this asserts that and hands back its key, so a test writing a
/// face's placement writes the pair the face HAS rather than a representative
/// of several (D10).
pub fn only_pair(face: &FormFace) -> Vec<u8> {
    let pairs: Vec<Vec<u8>> = face.at.scoped().map(|(k, _)| k).collect();
    let [pk] = pairs.as_slice() else {
        panic!("a fixture face relates one member pair, got {}", pairs.len())
    };
    pk.clone()
}

/// That pair's placement, read.
pub fn face_place(face: &FormFace) -> FacePlace {
    let pk = only_pair(face);
    face.at.get(&pk).expect("the pair just read").clone()
}

/// That pair's placement, written.
pub fn set_face_place(face: &mut FormFace, write: impl FnOnce(&mut FacePlace)) {
    let pk = only_pair(face);
    if let Some(p) = face.at.get_mut(&pk) {
        write(p);
    }
}

pub fn word_vertex(addr: u8, word: Word) -> Vertex {
    word_vertex_on(addr, word, stalk::Invariant::default())
}

/// The same fixture on a NAMED chart. The chart is the key the member is
/// filed under, so a test that wants a vertex on a particular chart says
/// so when it builds the vertex; it cannot reach in and rewrite the key
/// afterwards, which is the whole of what keying by content means (law 2).
/// `addr` USED TO NAME THE VERTEX AND NOW PLACES IT (54.2).
///
/// A vertex's address is derived from its coordinates, so two fixtures
/// handed different names but one rotor, one chart, one carriage and one
/// gauge are ONE vertex (§1.5). Extraction never emits two of them:
/// `extract_rule`'s `reads` is keyed by TRANSPORT CLASS, one `Vertex::Word`
/// per class.
///
/// `addr` NO LONGER NAMES THE VERTEX (54.2): a vertex's address is derived
/// from its coordinates, so two vertices with one rotor, one chart, one
/// carriage and one gauge are ONE vertex (§1.5), and extraction never
/// emits two — `reads` is keyed by transport class.
///
/// This builds ONE vertex and says nothing about distinctness; a fixture
/// that needs two says how they differ. [`chord_form`] does, on the chart,
/// because two ends of a chord are two SITES; separating them on the
/// carriage would put a placement component into every loop those tests
/// read, which is a different measurement, not a truer one.
pub fn word_vertex_on(addr: u8, word: Word, chart: stalk::Invariant) -> Vertex {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let _ = addr;
    Vertex::word_on(
        alphabet::Derivation::spelled(word),
        chart,
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        None,
    )
}

/// `n` SPOKES ARE `n` SITES (§1.5, 54.5 — the repair `chord_form` already
/// carries, applied at the site it was missed).
///
/// Every spoke was `word_vertex(i + 1, Word::identity())`, and since 54.2
/// made `addr` non-naming those are the SAME vertex: one rotor, one chart,
/// one carriage, one gauge. §1.5 says they are one vertex, and
/// `form_vertex_key` agreed — it wrote them through a trie keyed by each
/// one's coordinates and emitted ONE record — while the tape wrote three.
/// So every reading taken through the tape saw a star with `n` spokes and
/// the same form's ADDRESS named a star with one, and nothing compared the
/// two until the decoder moved.
///
/// The chart is what separates them, for `chord_form`'s reason: the chart
/// is where a body stands, not how it is carried, so the star's transports
/// are untouched.
pub fn star_form(n_words: usize, extra: Vec<(u16, u16, Word)>) -> Form {
    let mut f = Form::default();
    for i in 0..n_words {
        let j = (i as u16) + 1;
        f.vertices.push(word_vertex_on(
            j as u8,
            Word::identity(),
            stalk::Invariant::counts(&[u64::from(j)]),
        ));
        f.faces.push(f.face_of(0, j).expect("star"));
    }
    // A CHORD CARRIES ITS PLACEMENTS as well as its word, or the loop it
    // closes has no member path and no carriage (D10).
    for (i, j, w) in extra {
        f.faces.push(f.face_carrying(i, j, w).expect("a chord between two placed vertices"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
        // TWO ENDS ARE TWO SITES, and the site is the chart. They used to
        // be one vertex twice — same rotor, same chart, same carriage,
        // same gauge — distinguished only by the address each was handed,
        // which is identity by fiat (§1.5) and a form extraction cannot
        // produce. The chord's own transport is unchanged by this: the
        // chart is where a body stands, not how it is carried.
        f.vertices.push(word_vertex_on(a as u8, Word::identity(), stalk::Invariant::counts(&[u64::from(a)])));
        f.vertices.push(word_vertex_on(b as u8, Word::identity(), stalk::Invariant::counts(&[u64::from(b)])));
        f.faces.push(f.face_of(0, a).expect("star a"));
        f.faces.push(f.face_of(0, b).expect("star b"));
        f.faces.push(f.face_carrying(a, b, w.clone()).expect("a chord between two bodies"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

/// A chord whose EDGE carries the word (50.3): the face's own word is the
/// identity and the loop's holonomy is the stabilizer composite. This is
/// the carrier an off-plane rotor stands on — a face word with no sensor
/// action places no member pair, and a loop with no member path has no
/// class to read.
pub fn edge_chord_form(edges: &[Word]) -> Form {
    let mut f = Form::default();
    for (ci, w) in edges.iter().enumerate() {
        let a = (ci * 2 + 1) as u16;
        let b = (ci * 2 + 2) as u16;
        // TWO ENDS ARE TWO SITES (§1.5), for `chord_form`'s reason and at
        // the site that repair missed: one rotor, one chart, one carriage
        // and one gauge is ONE vertex, and a form holding it twice keys to
        // a form holding it once.
        f.vertices.push(word_vertex_on(a as u8, Word::identity(), stalk::Invariant::counts(&[u64::from(a)])));
        f.vertices.push(word_vertex_on(b as u8, Word::identity(), stalk::Invariant::counts(&[u64::from(b)])));
        f.faces.push(f.face_of(0, a).expect("star a"));
        f.faces.push(f.face_of(0, b).expect("star b"));
        f.faces.push(
            f.face_carrying_with_edge(a, b, Word::identity(), Some(Transport::from_word(w.clone())))
                .expect("a chord between two bodies"),
        );
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

pub fn coboundary_chord_form() -> Form {
    let r = order4();
    let mut f = Form::default();
    // TWO ENDS ARE TWO SITES (§1.5), as in `chord_form`.
    f.vertices.push(word_vertex_on(1, Word::identity(), stalk::Invariant::counts(&[1])));
    f.vertices.push(word_vertex_on(2, Word::identity(), stalk::Invariant::counts(&[2])));
    for (i, j, w) in [(0u16, 1u16, r.clone()), (0, 2, Word::identity()), (1, 2, r.inverse())] {
        f.faces.push(f.face_carrying(i, j, w).expect("two placed vertices and the Frame"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
            word: Box::new(Word::identity()),
            side: Side::Right,
        },
    }
}
