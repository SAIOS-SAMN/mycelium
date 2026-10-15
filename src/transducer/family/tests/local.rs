// MU-CLIENT §5.6 (W, C, S) — THE LOCAL DISCHARGE, claimed by the
// decompression itself: a word closes exactly the sites its own letters carry
// right, a body it carries wrong is no part of it, a letter names no body,
// and a sentence's gluing is the set of sites it closes beyond its words'.

use crate::arc::Grid;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{from_connection, translator_versor};
use crate::transducer::family::local::{form_local, measure_sites, word_local, Local};
use crate::transducer::family::{extract_rule, site_key, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

fn sites(at: &[(i64, i64)]) -> Keyed<()> {
    let mut k = Keyed::new();
    for &(r, c) in at {
        k.insert(site_key(r, c), ());
    }
    k
}

/// The slide by `(r, c)`, spelled as a connection: no turn, carried there.
fn slide(r: i64, c: i64) -> crate::transducer::element::Element {
    from_connection(&Word::identity(), &translator_versor(r, c), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells")
}

/// W — A WORD CLOSES THE BODY ITS LETTERS CARRY RIGHT, AND NO MORE. One body
/// slides two columns; another stands. The slide carries both: the first
/// lands where the answer holds it and is the word's section — the sites it
/// painted and the sites it vacated — while the second lands where the answer
/// holds nothing of it and is no part of the section. The section's pay is
/// the pair's demand over those sites.
#[test]
fn a_word_closes_the_body_its_letters_carry_right() {
    let family = vec![(
        grid(&["0000", "1100", "0000", "0200"]),
        grid(&["0000", "0011", "0000", "0200"]),
    )];
    let read = word_local(&slide(0, 2), &family);
    let carried = sites(&[(1, 0), (1, 1), (1, 2), (1, 3)]);
    let [got] = read.sites.as_slice() else { panic!("one pair, one section") };
    let got: Vec<Vec<u8>> = got.scoped().map(|(k, _)| k).collect();
    let want: Vec<Vec<u8>> = carried.scoped().map(|(k, _)| k).collect();
    assert_eq!(got, want, "the carried body's painted and vacated sites, and only those");
    assert_eq!(read.pay, measure_sites(&family, &[carried]), "paid the pair's demand over its section");
    assert!(read.pay.iter().any(|&m| m > 0), "a word that closes a body is paid");
}

/// W — A WORD THAT CARRIES A BODY WRONG HAS NO SECTION THERE. The same slide
/// on a pair whose body moved one column: its image is not the answer, and it
/// closes nothing.
#[test]
fn a_word_carrying_a_body_wrong_closes_nothing() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0110", "0000"]))];
    let read = word_local(&slide(0, 2), &family);
    assert!(read.sites.iter().all(|s| s.is_empty()), "no site of a body carried wrong");
    assert_eq!(read.pay, [0; 4], "and nothing paid");
}

/// W, 78.149(c) — A WORD THAT IS NOT A CONNECTION NAMES NO BODY here: a
/// letter stands at no frame, and the local measure of anything else is the
/// form's, never this reader's.
#[test]
fn a_letter_names_no_body() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane's quarter turn");
    let chart = crate::transducer::stalk::invariant(&[(0, 0, 1, 1)]);
    let letter = crate::transducer::sentence::generator_form(&crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor(), &chart, &[]).expect("a letter");
    let read = word_local(&letter, &family);
    assert_eq!(read.pay, [0; 4], "a letter is paid nothing on its own");
}

/// S — A SENTENCE THAT DOES NO MORE THAN ITS WORDS EARNS NO GLUING: the
/// family's rule closes the slid body, the slide closes the same sites, and
/// the rule's sites beyond the word's are none.
#[test]
fn a_sentence_doing_no_more_than_its_word_earns_no_gluing() {
    let family = vec![
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
    ];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the slide extracts");
    let whole = form_local(&rule.form, &family, &NoCrystal, &Word::identity());
    assert!(whole.pay.iter().any(|&m| m > 0), "the rule closes the family on its section");
    let word = word_local(&slide(0, 2), &family);
    let glued = whole.beyond(&Local::union(&[word], family.len()), &family);
    assert!(glued.sites.iter().all(|s| s.is_empty()), "no site the rule closes that its word does not");
    assert_eq!(glued.pay, [0; 4], "so no premium");
}

/// S, C — BEYOND IS A DIFFERENCE OF SETS, never of sums: a section beyond
/// itself is empty, beyond nothing it is whole, and what it pays is the
/// demand over the sites left.
#[test]
fn beyond_is_a_difference_of_sites() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let all = sites(&[(0, 0), (0, 1), (0, 2), (0, 3)]);
    let half = sites(&[(0, 2), (0, 3)]);
    let whole = Local { pay: measure_sites(&family, std::slice::from_ref(&all)), sites: vec![all] };
    let part = Local { pay: measure_sites(&family, std::slice::from_ref(&half)), sites: vec![half] };
    assert!(whole.beyond(&whole, &family).sites.iter().all(|s| s.is_empty()), "beyond itself, nothing");
    assert_eq!(whole.beyond(&Local::union(&[], 1), &family).pay, whole.pay, "beyond nothing, whole");
    let left = whole.beyond(&part, &family);
    assert_eq!(left.pay, measure_sites(&family, &[sites(&[(0, 0), (0, 1)])]), "the sites left, measured");
}
