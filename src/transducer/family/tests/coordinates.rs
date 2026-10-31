// 78.99 (1), (5) — A WORD'S COORDINATES: its holes and windings read off its
// own modes, the word rebuilt from them, a power recognized through them, and
// the coordinates carried whole across the wire and through a chart
// transport.

use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::sentence::coordinates::{Coordinates, Winding};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn read(w: &Mv) -> Coordinates {
    Coordinates::of(w, &mut stroke()).expect("the word's modes read")
}

/// The 90° turn of the `e₁e₂` plane, as its rotor ray.
fn quarter() -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1))).widened(3)
}

/// A TURN WORD STANDS ON ONE HOLE, AT ITS TURN. The 90° turn winds its plane
/// at a quarter of 360°; its square stands on the same hole at a half, its
/// cube at three quarters — the side the other way — and each is the turn's
/// power, read off the windings. Every reading rebuilds its word.
#[test]
fn coordinates_of_turn_winds_one_hole() {
    let w = quarter();
    let one = read(&w);
    assert_eq!(one.holes.len(), 1, "one plane, one hole");
    let Winding::Turn { t, ell } = one.holes[0].winding.clone() else { panic!("a turn: {:?}", one.holes[0].winding) };
    assert_eq!(ell, 4, "a quarter of 360°");
    assert!(t == 1 || t == 3, "on one side or the other: {t}");
    assert!(one.word().projective_eq(&w), "the coordinates are the word");
    let two = read(&w.geo(&w));
    assert_eq!(two.holes[0].key, one.holes[0].key, "the square stands on the same hole");
    assert_eq!(two.holes[0].winding, Winding::Turn { t: 1, ell: 2 }, "at 180°");
    assert!(two.word().projective_eq(&w.geo(&w)));
    assert_eq!(two.power_of(&one), Some(2), "the square is the turn's power 2");
    let three = read(&w.geo(&w).geo(&w));
    let Winding::Turn { t: t3, ell: 4 } = three.holes[0].winding.clone() else { panic!("a 90° turn") };
    assert_eq!(t3, 4 - t, "the cube stands on the other side");
    assert_eq!(three.power_of(&one), Some(3));
    assert_eq!(one.power_of(&one), None, "a word is not its own power: it is recognized by its key");
}

/// A SLIDE COUNTS ALONG ITS HOLE. A translation's one mode is parabolic: its
/// winding is a count against the canonical generator, the slide by two the
/// slide by one counted twice; a slide along another axis stands on another
/// hole and is no power of the first.
#[test]
fn coordinates_of_slide_counts_along_hole() {
    let t1 = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let t2 = cga::translator_at(&[q(2, 1), q(0, 1), q(0, 1)]);
    let ty = cga::translator_at(&[q(0, 1), q(1, 1), q(0, 1)]);
    let (a, b, c) = (read(&t1), read(&t2), read(&ty));
    assert!(matches!(a.holes[0].winding, Winding::Count(_)), "a count: {:?}", a.holes[0].winding);
    assert!(a.word().projective_eq(&t1) && b.word().projective_eq(&t2), "the coordinates are the words");
    assert_eq!(b.power_of(&a), Some(2), "the slide by two is the slide by one, twice");
    assert_ne!(c.holes[0].key, a.holes[0].key, "another axis, another hole");
    assert_eq!(c.power_of(&a), None);
}

/// A DILATION IS NAMED: it never returns and does not condense (78.68), so
/// its hole holds its rational point and side, and the word rebuilds.
#[test]
fn coordinates_of_names_dilation_on_hole() {
    let d = cga::dilator(&q(2, 1));
    let c = read(&d);
    assert!(c.holes.iter().any(|h| matches!(h.winding, Winding::Named { .. })), "named: {:?}", c.holes);
    assert!(c.word().projective_eq(&d), "the coordinates are the word");
}

/// INFLECTION MOVES THE HOLES; THE SIGNATURE IS WHAT EVERY NODE HEARS
/// (78.100). Carried by a chart transport `Γ`, a word's coordinates are those
/// of `Γ W Γ̃`, read directly — its holes moved onto other planes — while its
/// harmonic signature is unchanged: the tones are conjugation-invariant and
/// the planes are not among them. A power of the word sounds otherwise. The
/// signature travels whole.
#[test]
fn transported_moves_holes_keeps_signature() {
    use crate::transducer::sentence::coordinates::Signature;
    let w = quarter().geo(&cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]));
    let c = read(&w);
    let gamma = Mv::scalar(q(1, 1)).add(&Mv::blade(E2 | E3, q(1, 1))).widened(3);
    let moved = c.transported(&gamma).expect("a versor transports");
    let direct = read(&gamma.sandwich(&w));
    assert_eq!(moved, direct, "inflection is the transport applied to the holes");
    assert_ne!(moved.holes.iter().map(|h| &h.key).collect::<Vec<_>>(), c.holes.iter().map(|h| &h.key).collect::<Vec<_>>(), "the planes moved");
    assert_eq!(moved.signature(), c.signature(), "the signature did not: a conjugate sounds the same");
    let quarter_sig = read(&quarter()).signature();
    assert_ne!(read(&quarter().geo(&quarter())).signature(), quarter_sig, "the turn's square sounds otherwise");
    let mut wire = Vec::new();
    c.signature().put(&mut wire);
    let mut pos = 0;
    let back = Signature::take(&wire, &mut pos).expect("the signature reads back");
    assert_eq!((pos, back), (wire.len(), c.signature()), "whole");
}

/// A lexicon holding `held`, each filed under its holes by its own key.
fn index(held: &[(&[u8], &Coordinates)]) -> crate::transducer::sentence::coordinates::HoleIndex {
    let mut ix = crate::transducer::sentence::coordinates::HoleIndex::default();
    for (key, c) in held {
        ix.file(key, c);
    }
    ix
}

/// 78.100 D3 — A WORD IS DECONSTRUCTED BY ITS MOTION, MODE BY MODE, INTO
/// WHAT IS HELD AND ITS RESIDUE. The 90° turn is held. Its square is every
/// mode cited — the turn at exponent 2 — and derived: grammar, never held
/// twice. A screw — the turn and a slide along its axis, two commuting modes
/// — cites the turn on the turn's hole and leaves the slide as its residue;
/// the cited mode and the residue multiply back to the screw exactly. A word
/// on no held hole is all residue.
#[test]
fn deconstruct_splits_word_into_cited_and_residue() {
    let turn = read(&quarter());
    let ix = index(&[(b"turn", &turn)]);
    let held = |k: &[u8]| (k == b"turn").then_some(&turn);

    let square = ix.deconstruct(&read(&quarter().geo(&quarter())), held);
    assert!(square.derived(), "every mode cited: {square:?}");
    assert_eq!(square.cites[0].exponent, 2, "the turn, twice");

    let slide = cga::translator_at(&[q(0, 1), q(0, 1), q(1, 1)]);
    let screw = quarter().geo(&slide);
    let c = read(&screw);
    assert_eq!(c.holes.len(), 2, "a screw stands on two holes: {c:?}");
    let d = ix.deconstruct(&c, held);
    assert_eq!(d.cites.len(), 1, "the turn's mode is cited: {d:?}");
    assert_eq!((d.cites[0].word.as_slice(), d.cites[0].exponent), (b"turn".as_slice(), 1));
    assert_eq!(d.residue.len(), 1, "the slide is the residue");
    assert_eq!(d.residue[0].kind, crate::network::nerve::coord::BivectorKind::Parabolic, "a null slide");
    let cited = turn.holes.iter().find(|h| h.key == d.cites[0].hole).expect("the cited hole").ray();
    let rebuilt = cited.geo(&Coordinates { holes: d.residue.clone() }.word());
    assert!(rebuilt.projective_eq(&c.word()), "the cited mode and the residue multiply back to the screw");

    let alone = ix.deconstruct(&read(&slide), held);
    assert!(alone.cites.is_empty() && alone.residue.len() == 1, "a word on no held hole is all residue");
}
