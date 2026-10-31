// 1.1, 1.2 — the seventeen letters, each with its inverse, and the words
// the dihedral motions spell.

use crate::transducer::alphabet::*;
use crate::transducer::peel::PeelNest;

/// THE DERIVED LAYOUT IS THE TABULATED ONE, coordinate for coordinate.
///
/// The alphabet's shape was written out in five places at once: the length
/// `17`, the plane table and its type `[_; 6]`, the `[_; 4]` beside it, the
/// slot bases `1`, `7` and `13` at every reader, the `p < 4` that guarded
/// the second prime, the `p == 0` that meant the sensor plane, the
/// `[i64; 12]` free row, the `1..=6` 90° sweep, the three complementary
/// slot pairs, and the relator count. Each of those is the same fact as the
/// rank and the two tables, said again where it could drift.
///
/// They are derived now, and this is what says the derivation reproduces
/// what was there: a chart of rank four has six pair planes in lexicographic
/// order, seventeen letters over them, and twelve relators.
#[test]
fn alphabet_layout_derives_tabulated_values() {
    assert_eq!(LETTER_RANK, 4, "the chart the letters occupy");
    assert_eq!(PAIR_PLANES, 6, "rank choose two");
    assert_eq!(
        SLOT_PLANES_SIX,
        [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)],
        "the two-subsets in lexicographic order, which is slot order"
    );
    assert_eq!(CAYLEY_ALPHABET_LEN, 17, "identity, six 90° turns, six (3,4,5), four (5,12,13)");
    assert_eq!(crate::transducer::sentence::FREE_ROW, 12, "two prime exponents per plane");

    // The slot bases, read back off the letters they name.
    assert_eq!(LetterId::turn_90((0, 1)).map(|l| l.raw()), Some(1));
    assert_eq!(LetterId::pythagorean_5((0, 1)).map(|l| l.raw()), Some(7));
    assert_eq!(LetterId::pythagorean_13((0, 1)).map(|l| l.raw()), Some(13));

    // The three named letters are readings of the sensor plane and its
    // flip, not positions someone counted to.
    assert_eq!(
        LetterId::SHEET,
        flip_plane(sensor_plane()).and_then(LetterId::turn_90).expect("the sensor plane flips"),
        "the sheet is the 90° turn on the sensor plane's flip"
    );
    assert_eq!(LetterId::PRIME_5, LetterId::pythagorean_5(sensor_plane()).expect("on the sensor"));
    assert_eq!(LetterId::PRIME_13, LetterId::pythagorean_13(sensor_plane()).expect("on the sensor"));

    // Two planes are complementary exactly when they share no axis, which
    // the complement reads off the plane rather than off a table of pairs.
    for &plane in &SLOT_PLANES_SIX {
        let other = complement_plane(plane).expect("every pair plane of rank four has a complement");
        assert!(plane.0 != other.0 && plane.0 != other.1 && plane.1 != other.0 && plane.1 != other.1);
        assert_eq!(complement_plane(other), Some(plane), "and the relation is symmetric");
    }
    assert_eq!(complement_plane((0, 1)), Some((2, 3)));
    assert_eq!(complement_plane((0, 2)), Some((1, 3)));
    assert_eq!(complement_plane((0, 3)), Some((1, 2)));

    // Six 90°-turn orders, three commuting pairs, one sheet, two primes.
    assert_eq!(relators().len(), 12);
    let commuting = relators()
        .iter()
        .filter(|r| r.kind == RelatorKind::PlaneCommute)
        .count();
    assert_eq!(commuting, 3, "each complementary pair stated once, not twice");
}

/// LAW VIII: seventeen letters, each a versor with its carried inverse
/// exact — `Φ Φ⁻¹ = 1` for all of them, and no eighteenth slot.
#[test]
fn letter_carries_exact_inverse_for_every_letter() {
    assert_eq!(LetterId::all().count(), 17);
    assert!(LetterId::try_new(17).is_none());
    for id in LetterId::all() {
        let phi = letter(id);
        assert!(phi.weight().is_some(), "letter {} is a versor", id.raw());
        assert_eq!(phi.geo(&letter_inverse(id)), Rotor::identity(), "letter {} carries its inverse", id.raw());
        let scaled = phi.scale(&qn(3, 2));
        assert!(phi.projective_eq(&scaled), "a positive weight is the same transport");
        assert!(phi.projective_eq(&phi.scale(&qn(-3, 2))), "a negative weight is the same transport");
        assert_eq!(phi.normalised(), scaled.normalised());
        assert_eq!(phi.normalised(), phi.scale(&qn(-1, 1)).normalised());
    }
}

/// The lattice isometries are WORDS, not letters: rot90 is one 90°
/// letter, rot180 two, the column flip a 180° turn through the third
/// axis, and the transposition the product of the two — every one spelled by
/// the peel and re-derived exactly from its tape.
#[test]
fn spell_images_spells_dihedral_motions() {
    let rot90 = spell_images((0, -1), (1, 0)).expect("rot90 spells");
    assert_eq!(rot90.word.len(), 1);
    let rot180 = spell_images((-1, 0), (0, -1)).expect("rot180 spells");
    assert_eq!(rot180.word.len(), 2);
    let flip_cols = spell_images((1, 0), (0, -1)).expect("flip spells");
    assert_eq!(flip_cols.word.len(), 2);
    assert!(flip_cols.word.letters().iter().all(|(id, _)| plane_of(*id) == Some((1, 2))));
    let transpose = spell_images((0, 1), (1, 0)).expect("transpose spells");
    assert_eq!(transpose.word.len(), 3);
    let id = spell_images((1, 0), (0, 1)).expect("identity spells");
    assert!(id.word.is_empty());
    // The words re-derive the words' blocks and round-trip the wire.
    for sp in [&rot90, &rot180, &flip_cols, &transpose] {
        let mut b = Vec::new();
        sp.to_bytes(&mut b);
        let mut pos = 0;
        assert_eq!(Spelling::from_bytes(&b, &mut pos).as_ref(), Some(sp));
        assert_eq!(pos, b.len());
    }
}

/// Pythagorean rotations and dilations: the (3,4,5) letter's own
/// rotation spells as one letter at ρ = 1 and at ρ = 3; a rotation by a
/// prime outside the alphabet (17) stands as a remainder; a non-
/// similarity refuses.
#[test]
fn spell_peels_dilation_and_refuses_outside_primes() {
    let l5 = spell(&[[qn(-7, 25), qn(-24, 25)], [qn(24, 25), qn(-7, 25)]]).expect("(3,4,5)² spells");
    assert_eq!(l5.word.len(), 1);
    let l5_inv = spell(&[[qn(-7, 25), qn(24, 25)], [qn(-24, 25), qn(-7, 25)]]).expect("inverse spells");
    assert_eq!(l5_inv.word.letters()[0].0, l5.word.letters()[0].0);
    assert_ne!(l5_inv.word.letters()[0].1, l5.word.letters()[0].1);
    let scaled = spell(&[[qn(-21, 25), qn(-72, 25)], [qn(72, 25), qn(-21, 25)]]).expect("ρ = 3");
    assert_eq!(scaled.word, l5.word, "ρ is peeled for the carriage; the word is the rotation");
    // (3,4,5) itself is a half letter under the sandwich: not a word.
    let (nest, _) = spell_peel(&[[qn(3, 5), qn(-4, 5)], [qn(4, 5), qn(3, 5)]]).expect("a similarity");
    assert!(matches!(nest, PeelNest::Standing { .. }));
    // (8,15,17): a prime the alphabet does not hold.
    assert!(spell(&[[qn(8, 17), qn(-15, 17)], [qn(15, 17), qn(8, 17)]]).is_none());
    // A shear is not a similarity.
    assert!(spell(&[[q(1), q(1)], [q(0), q(1)]]).is_none());
    // (5,12,13)² composed with a 90° turn: two letters.
    let w = Word::from_letters(&[(LetterId::turn_90((0, 1)).unwrap(), false), (LetterId::pythagorean_13((0, 1)).unwrap(), true)]);
    let block = w.plane_block().expect("plane preserved");
    let sp = spell(&block).expect("spells back");
    assert_eq!(sp.word.plane_block(), Some(block));
    assert_eq!(sp.word.len(), 2);
}

/// Sheet conjugates a rotation to its inverse: S Q S Q⁻¹ is a 180°
/// turn, not the identity the commuting sum would claim.
#[test]
fn sheet_conjugation_is_not_abelian() {
    let sheet = spell_images((1, 0), (0, -1)).expect("column flip");
    let q = spell_images((0, -1), (1, 0)).expect("rot90");
    let w = sheet.word.compose(&q.word).compose(&sheet.word).compose(&q.word.inverse());
    let f = w.form().expect("the product preserves the plane");
    assert!(!f.is_identity(), "S Q S Q⁻¹ is a 180° turn, not 1");
    assert_eq!(w.order(), Order::Finite(2));
    let turn_180 = spell_images((-1, 0), (0, -1)).expect("rot180");
    assert!(w.same_form(&turn_180.word));
}
