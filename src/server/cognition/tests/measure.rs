// MU-CLIENT §5.6 (W, E1) — THE COURT MEASURES A WORD ON THE SCENE IT WAS
// SPOKEN UNDER: the one reader of what a word can be escrowed or paid
// (`discharge_on`) decompresses a connection onto its bound family and reads
// its own section there; with no family it stands on no scene and measures
// nothing but the listing it stands for.

use crate::arc::Grid;
use crate::cognition::*;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{from_connection, translator_versor};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A WORD BOUND TO A FAMILY IS PAID ITS SECTION THERE. The slide that
/// carries one body of each pair right is measured, at the court's own one
/// reader, exactly as its local section reads — non-zero; the same word on no
/// family measures nothing (no listing stands for it); and a word that is no
/// conjugate carries no transport.
#[test]
fn a_word_bound_to_a_family_is_paid_its_section_there() {
    let court = Kernel::new();
    let family = vec![
        (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
    ];
    let slide = from_connection(&Word::identity(), &translator_versor(0, 2), &[], &Vec::new(), [1, 0, 0, 0]).expect("a slide spells");
    let addr = slide.address();
    let read = court.discharge_on(addr.as_bytes(), &slide, Some(&family));
    assert_eq!(read, crate::transducer::family::local::word_local(&slide, &family).pay, "the court reads the word's own section");
    assert!(read.iter().any(|&m| m > 0), "and it is paid there");
    assert_eq!(court.discharge_on(addr.as_bytes(), &slide, None), [0; 4], "on no scene, nothing");
    assert!(court.conjugate_of(&slide).is_none(), "a slide is no conjugate");
}
