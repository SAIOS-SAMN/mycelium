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

/// W — A WORD'S SECTION IS EVERY SITE ITS SPEECH CHANGED. The slide carries
/// the pair's whole before side — both bodies — and the answer holds both
/// where it carried them: its section is each site it painted and each it
/// vacated, and it is paid the pair's demand over them.
#[test]
fn word_local_section_is_every_changed_site() {
    let family = vec![(
        grid(&["000000", "110000", "000000", "020000"]),
        grid(&["000000", "001100", "000000", "000200"]),
    )];
    let read = word_local(&slide(0, 2), &family);
    let changed = sites(&[(1, 0), (1, 1), (1, 2), (1, 3), (3, 1), (3, 3)]);
    let [got] = read.sites.as_slice() else { panic!("one pair, one section") };
    let got: Vec<Vec<u8>> = got.scoped().map(|(k, _)| k).collect();
    let want: Vec<Vec<u8>> = changed.scoped().map(|(k, _)| k).collect();
    assert_eq!(got, want, "every painted and every vacated site of the whole before side moved");
    assert_eq!(read.pay, measure_sites(&family, &[changed]), "paid the pair's demand over its section");
    assert!(read.pay.iter().any(|&m| m > 0), "a word that says the pair is paid");
}

/// W — A WORD THAT SAYS SOMETHING FALSE ON A PAIR IS PAID NOTHING THERE, even
/// where part of what it said is right. One body slides two columns, the other
/// stands: the slide carries both, so it says the standing body moved, and the
/// answer holds it where it stood. Nothing of the pair is the word's — never a
/// choice of the body it happened to fit.
#[test]
fn word_local_pays_nothing_on_false_pair() {
    let family = vec![(
        grid(&["000000", "110000", "000000", "020000"]),
        grid(&["000000", "001100", "000000", "020000"]),
    )];
    let read = word_local(&slide(0, 2), &family);
    assert!(read.sites.iter().all(|s| s.is_empty()), "no site of a pair it spoke falsely on");
    assert_eq!(read.pay, [0; 4], "and nothing paid");
}

/// W — A WORD THAT CARRIES A BODY WRONG HAS NO SECTION THERE. The same slide
/// on a pair whose body moved one column: its image is not the answer, and it
/// closes nothing.
#[test]
fn word_local_closes_nothing_for_wrong_carriage() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0110", "0000"]))];
    let read = word_local(&slide(0, 2), &family);
    assert!(read.sites.iter().all(|s| s.is_empty()), "no site of a body carried wrong");
    assert_eq!(read.pay, [0; 4], "and nothing paid");
}

/// W, 78.149(c) — A WORD THAT IS NOT A CONNECTION NAMES NO BODY here: a
/// letter stands at no frame, and the local measure of anything else is the
/// form's, never this reader's.
#[test]
fn word_local_pays_letter_nothing() {
    let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane's 90° turn");
    let chart = crate::transducer::stalk::invariant(&[(0, 0, 1, 1)]);
    let letter = crate::transducer::sentence::generator_form(&crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor(), &chart, &[]).expect("a letter");
    let read = word_local(&letter, &family);
    assert_eq!(read.pay, [0; 4], "a letter is paid nothing on its own");
}

/// W, C — A FORM AND ITS WORD READ ONE SECTION: the family's rule (one body
/// slid, a form) closes on its own section exactly the sites the slide word
/// closes on its own, so the rule's section beyond the word's is empty — the
/// form reader and the word reader agree where the form says no more than the
/// word. (The sentence's gluing, read off its rings, is guarded at the court:
/// `server::cognition::tests::measure`.)
#[test]
fn form_local_agrees_with_word_local_section() {
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
fn beyond_is_difference_of_sites() {
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
