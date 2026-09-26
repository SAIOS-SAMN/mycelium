//! 39 — FIXTURES for the sentence's tests: the two rationals, a form
//! address, a closed word form, a star, and the words of each order.

use crate::network::nerve::coord;
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

/// BQ5b — A CONTACT'S INTERFACE between two unit spheres centred on the
/// first axis at `a` and `b`, at the contact's `grade`: the one producer,
/// `Interface::between`, over two real nuclei.
pub fn contact(a: i64, b: i64, grade: u8) -> Interface {
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let sa = crate::transducer::cga::sphere_at(&[q(a), q(0), q(0)], &q(1));
    let sb = crate::transducer::cga::sphere_at(&[q(b), q(0), q(0)], &q(1));
    Interface::between(&sa, &sb, grade).expect("two spheres tether")
}

pub fn form_addr(n: u8) -> crate::transducer::address::Address {
    crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, n])
}

/// THE FIXTURES' FRAME: every member a fixture stands, stands at frame 0 —
/// one frame, the single pair a fixture scene is (AT1: a member holds its
/// frames, and a fixture writes them as it writes its member pairs).
pub const FRAME: u64 = 0;

/// A word vertex stood on the form at [`FRAME`], with its star from the
/// Frame's chart (Q-B1): the member's chart back.
pub fn stand_with_star(f: &mut Form, v: &Vertex) -> coord::ChartAt {
    let c = f.stand_member(v, FRAME).expect("a word vertex stands");
    f.place_overlap(coord::ChartAt::Frame, c.clone(), None).expect("star");
    c
}

/// A CHORD between two charts carrying `w`: the overlap placed, and `w`
/// written as the pivot writes it — the left factor (AT1, ruled: produced
/// content, written only by the pivot's Word move) — so its derived word is
/// `w · (w_b w_a⁻¹)`, which is `w` between two ends of one word.
pub fn chord(f: &mut Form, a: &coord::ChartAt, b: &coord::ChartAt, w: Word) -> coord::Overlap {
    let o = f.place_overlap(a.clone(), b.clone(), None).expect("a chord between two placed members");
    f.apply_move(&PivotBy::Word { face: o.clone(), word: Box::new(w), side: Side::Left });
    o
}

/// A chord whose EDGE carries the transport, its factors the identity.
pub fn edge_chord(f: &mut Form, a: &coord::ChartAt, b: &coord::ChartAt, edge: Transport) -> coord::Overlap {
    f.place_overlap(a.clone(), b.clone(), Some(edge)).expect("a chord between two placed members")
}

/// The chart of a form's one member on `chart` at [`FRAME`], by the class
/// of the vertex it stands at.
pub fn chart_of(v: &Vertex, chart: stalk::Invariant) -> coord::ChartAt {
    coord::ChartAt::at(v.at().expect("a fixture vertex has a coordinate"), chart, FRAME)
}

/// The coordinate of a form's one vertex of kind Word (a fixture with one
/// word class).
pub fn only_word(f: &Form) -> coord::VertexAt {
    let words: Vec<coord::VertexAt> = f
        .vertices
        .entries()
        .filter(|(_, v)| matches!(v, Vertex::Word { .. }))
        .map(|(k, _)| k)
        .collect();
    let [w] = words.as_slice() else { panic!("a fixture with one word class, got {}", words.len()) };
    w.clone()
}

pub fn closed_word_form(word: Word, contact: Option<alphabet::Contact>) -> Form {
    let mut f = Form::default();
    let v = Vertex::word_on(
        alphabet::Derivation::spelled(word.clone()),
        stalk::Invariant::counts(&[3, 5]),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        vec![(1, 2)],
        [FRAME],
        contact,
    );
    let c = stand_with_star(&mut f, &v);
    f.remainder = crate::transducer::element::PlacedChain::default();
    f.depth = f.depth_of().expect("a fixture's depth reads");
    let k = form_vertex_key(&f);
    if let Some(Vertex::Word { address, .. }) = f.vertices.at_mut(&c.vertex()) {
        *address = k;
    }
    f.reindex();
    f
}

/// An overlap's placement, written (AT1: one member pair per overlap; it is
/// read as `face.place`).
pub fn set_face_place(face: &mut FormFace, write: impl FnOnce(&mut FacePlace)) {
    write(&mut face.place);
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
/// carriage would put a placement component into every orbit those tests
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
        [FRAME],
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
///
/// AT1 (Q1): spokes of one word are ONE vertex — its class — with a member
/// per spoke's chart, and the star is an overlap from the Frame's chart to
/// each. `extra` names chords by spoke number, `0` the Frame's chart.
pub fn star_form(n_words: usize, extra: Vec<(u16, u16, Word)>) -> Form {
    let mut f = Form::default();
    let mut charts = vec![coord::ChartAt::Frame];
    for i in 0..n_words {
        let j = u16::try_from(i).expect("a star fixture's spoke, n_words small") + 1;
        let v = word_vertex_on(
            u8::try_from(j).expect("a star fixture's spoke, n_words small"),
            Word::identity(),
            stalk::Invariant::counts(&[u64::from(j)]),
        );
        charts.push(stand_with_star(&mut f, &v));
    }
    // A CHORD CARRIES ITS PLACEMENTS as well as its word, or the orbit it
    // closes has no member path and no carriage (D10).
    for (i, j, w) in extra {
        let (a, b) = (&charts[usize::from(i)], &charts[usize::from(j)]);
        chord(&mut f, a, b, w);
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

pub fn word_turn_180() -> Word {
    alphabet::PlaneForm {
        orientation: true,
        rot: [0; alphabet::CAYLEY_ALPHABET_LEN],
    }
    .to_word()
}

pub fn word_turn_90() -> Word {
    Word::from_letters(&[(alphabet::LetterId::turn_90((0, 1)).unwrap(), false)])
}

pub fn infinite() -> Word {
    Word::from_letters(&[(alphabet::LetterId::pythagorean_5((0, 1)).unwrap(), false)])
}

pub fn chord_form(chords: &[Word]) -> Form {
    let mut f = Form::default();
    for (ci, w) in chords.iter().enumerate() {
        let a = u16::try_from(ci * 2 + 1).expect("a chord fixture's end, a few chords");
        let b = u16::try_from(ci * 2 + 2).expect("a chord fixture's end, a few chords");
        // TWO ENDS ARE TWO SITES, and the site is the chart. They used to
        // be one vertex twice — same rotor, same chart, same carriage,
        // same gauge — distinguished only by the address each was handed,
        // which is identity by fiat (§1.5) and a form extraction cannot
        // produce. The chord's own transport is unchanged by this: the
        // chart is where a body stands, not how it is carried.
        let (a8, b8) = (
            u8::try_from(a).expect("a chord fixture's end, a few chords"),
            u8::try_from(b).expect("a chord fixture's end, a few chords"),
        );
        let ca = stand_with_star(&mut f, &word_vertex_on(a8, Word::identity(), stalk::Invariant::counts(&[u64::from(a)])));
        let cb = stand_with_star(&mut f, &word_vertex_on(b8, Word::identity(), stalk::Invariant::counts(&[u64::from(b)])));
        chord(&mut f, &ca, &cb, w.clone());
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

/// A chord whose EDGE carries the word (50.3): the face's own word is the
/// identity and the orbit's holonomy is the stabilizer composite. This is
/// the carrier an off-plane rotor stands on — a face word with no sensor
/// action places no member pair, and an orbit with no member path has no
/// class to read.
pub fn edge_chord_form(edges: &[Word]) -> Form {
    let mut f = Form::default();
    for (ci, w) in edges.iter().enumerate() {
        let a = u16::try_from(ci * 2 + 1).expect("an edge fixture's end, a few edges");
        let b = u16::try_from(ci * 2 + 2).expect("an edge fixture's end, a few edges");
        // TWO ENDS ARE TWO SITES (§1.5), for `chord_form`'s reason and at
        // the site that repair missed: one rotor, one chart, one carriage
        // and one gauge is ONE vertex, and a form holding it twice keys to
        // a form holding it once.
        let (a8, b8) = (
            u8::try_from(a).expect("an edge fixture's end, a few edges"),
            u8::try_from(b).expect("an edge fixture's end, a few edges"),
        );
        let ca = stand_with_star(&mut f, &word_vertex_on(a8, Word::identity(), stalk::Invariant::counts(&[u64::from(a)])));
        let cb = stand_with_star(&mut f, &word_vertex_on(b8, Word::identity(), stalk::Invariant::counts(&[u64::from(b)])));
        edge_chord(&mut f, &ca, &cb, Transport::from_word(w.clone()));
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

pub fn coboundary_chord_form() -> Form {
    let r = word_turn_90();
    let mut f = Form::default();
    // TWO ENDS ARE TWO SITES (§1.5), as in `chord_form`: two members of the
    // identity's class, each overlapped from the Frame's chart.
    let c1 = f.stand_member(&word_vertex_on(1, Word::identity(), stalk::Invariant::counts(&[1])), FRAME).expect("member 1");
    let c2 = f.stand_member(&word_vertex_on(2, Word::identity(), stalk::Invariant::counts(&[2])), FRAME).expect("member 2");
    let charts = [coord::ChartAt::Frame, c1, c2];
    for (i, j, w) in [(0usize, 1usize, r.clone()), (0, 2, Word::identity()), (1, 2, r.inverse())] {
        chord(&mut f, &charts[i], &charts[j], w);
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    f
}

/// An overlap for a fixture that needs one by name and holds no form: the
/// Frame's chart to the identity's member on chart `[1]` at [`FRAME`].
pub fn some_overlap() -> coord::Overlap {
    coord::Overlap {
        from: coord::ChartAt::Frame,
        to: chart_of(&word_vertex_on(1, Word::identity(), stalk::Invariant::counts(&[1])), stalk::Invariant::counts(&[1])),
    }
}

pub fn cert(axis: Axis, mu: [u64; 4]) -> PivotCertificate {
    PivotCertificate {
        chord: some_overlap(),
        axis,
        before: Word::identity().class_key().expect("identity"),
        after: Word::identity().class_key().expect("identity"),
        rank_move: RankMove::Equal,
        mu: Mu::Measured {
            before: [0; 4],
            after: mu,
        },
        by: PivotBy::Word {
            face: some_overlap(),
            word: Box::new(Word::identity()),
            side: Side::Right,
        },
    }
}
