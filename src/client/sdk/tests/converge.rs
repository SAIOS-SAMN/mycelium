// MU-CLIENT §5.5 — REAL CONVERGENCE, claimed by the decompression itself: two
// different words that change the same sites of one scene converge; one word
// read twice does not; a word false on its scene is refuted; a word leaving
// takes its scene with it.

use crate::client::sdk::entity::converge::{Evident, Scenes};
use crate::transducer::alphabet::{LetterId, Word};
use crate::transducer::element::{from_connection, translator_versor, Element};

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The slide by `(r, c)`: no turn, carried there.
fn slide(r: i64, c: i64) -> Element {
    from_connection(&Word::identity(), &translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// The 180° turn of the sensor plane, carried by `(r, c)`: a compartment is
/// the unit square at its site, so the square at `(i, j)` lands at
/// `(r − 1 − i, c − 1 − j)`.
fn turn_180(r: i64, c: i64) -> Element {
    let q = LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let word = Word::from_letters(&[(q, false), (q, false)]);
    from_connection(&word, &translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a 180° turn spells")
}

/// TWO CONSTRUCTIONS, ONE CONCLUSION. On `11·· → ··11` the slide by two
/// columns and the 180° turn about the row's middle are different vertices,
/// and each changes exactly the four sites of the row: the hearer that spoke
/// the slide on the scene reads the turn true there and converged with its
/// own. The slide heard back is the same vertex — one object read twice, no
/// convergence.
#[test]
fn two_different_words_with_one_section_converge() {
    let scene = [7u8; 32];
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let (mine, theirs) = (slide(0, 2), turn_180(1, 4));
    assert!(!mine.same_vertex(&theirs), "two constructions, not one");
    let mut s = Scenes::default();
    s.hold(&scene, &family, &mine);
    assert_eq!(s.read(&scene, &theirs, &family, [9; 32]), Evident::True, "the 180° turn is true on the scene");
    assert_eq!(s.converged.len(), 1, "and converges with the slide: one section, two orbits");
    assert_eq!(s.read(&scene, &mine, &family, [9; 32]), Evident::True);
    assert_eq!(s.converged.len(), 1, "the slide heard back is one vertex read twice: no convergence");
}

/// A WORD FALSE ON ITS SCENE IS REFUTED, and converges with nothing: the
/// slide by one row paints the row below, where the answer holds nothing.
/// The slide by one column is TRUE there on a smaller section — the two sites
/// it changed are right — and converges with nothing either: another section.
#[test]
fn a_word_false_on_its_scene_is_refuted() {
    let scene = [7u8; 32];
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let mut s = Scenes::default();
    s.hold(&scene, &family, &slide(0, 2));
    assert_eq!(s.read(&scene, &slide(1, 0), &family, [9; 32]), Evident::Refuted);
    assert_eq!(s.read(&scene, &slide(0, 1), &family, [9; 32]), Evident::True, "true where it changed a site, on a smaller section");
    assert!(s.converged.is_empty());
}

/// A WORD LEAVING TAKES ITS PLACE ON ITS SCENE, and a scene with no word
/// left leaves.
#[test]
fn a_word_leaving_takes_its_scene_with_it() {
    let scene = [7u8; 32];
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let word = slide(0, 2);
    let mut s = Scenes::default();
    s.hold(&scene, &family, &word);
    assert_eq!(s.held(), (1, 1));
    s.forget(word.address().as_bytes());
    assert_eq!(s.held(), (0, 0));
    assert!(s.family(&scene).is_none());
}
