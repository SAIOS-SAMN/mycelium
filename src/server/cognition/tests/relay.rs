// 78.99 (5) — THE RELAY: a spoken sentence carries the coordinates of the
// words its form teaches through the speech codec whole, and the court
// re-reads them on its own derivation of the form — agreeing where the
// coordinates are its own, reading the coordinate that moved where a
// transport carried them elsewhere.

use super::common::*;
use crate::anastome::dialogue::{self, Spoken};
use crate::cognition::*;
use crate::transducer::sentence::as_element_form;

/// The slide family's form, spoken as a sentence: its rings of seen rays are
/// nilpotent theorems, each teaching a word.
fn slide_sentence() -> Element {
    let slides = vec![
        (digits(&["1100000", "0000000", "0002000", "0002200"]), digits(&["1100000", "0000000", "0000020", "0000022"])),
        (digits(&["0000110", "0000000", "0200000", "0220000"]), digits(&["0000110", "0000000", "0002000", "0002200"])),
    ];
    let rule = crate::transducer::family::extract_rule(&slides, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("a rule");
    as_element_form(&rule.form, [1, 0, 0, 0]).expect("a tape")
}

/// THE COORDINATES TRAVEL WITH THE SPEECH AND THE COURT RE-READS THEM. The
/// speaker reads the words its sentence teaches; the speech frame carries
/// them beside the tape and decodes them whole; the court's own derivation
/// of the same form reads every one of them (agreed, none moved, none
/// unsaid). Carried by a quarter turn they stand on other holes: the court
/// reads each relayed coordinate as moved.
#[test]
fn the_relayed_coordinates_travel_whole_and_the_court_re_reads_them() {
    let e = slide_sentence();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let (coords, unread) = crate::transducer::sentence::rung::coordinates_taught(&e, &mut budget);
    assert!(!coords.is_empty(), "the slide's rings teach words with coordinates (unread {unread})");
    let frame = dialogue::encode_speech_spoken(
        &[9; 32],
        [7u8; 4],
        0,
        &[Spoken { tape: e.to_bytes().expect("the record writes"), standing: 0, contraction: None, coordinates: coords.clone() }],
    )
    .expect("the task frames");
    let act = dialogue::decode_speech(&frame).expect("the frame decodes");
    assert_eq!(act.coordinates, vec![coords.clone()], "the coordinates travel whole");
    let mut court = Kernel::new();
    court.reread_relay(&e, &coords, &mut budget);
    let t = court.winding_tally();
    assert_eq!((t.relay_agreed, t.relay_moved, t.relay_unsaid), (coords.len() as u64, 0, 0), "the court reads what was relayed: {t:?}");
    let quarter = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()))
        .add(&crate::transducer::cga::Mv::blade(crate::transducer::cga::E1 | crate::transducer::cga::E2, num_rational::BigRational::from_integer(1.into())))
        .widened(3);
    let carried: Vec<_> = coords.iter().map(|c| c.transported(&quarter).expect("a versor transports")).collect();
    let moved_before = court.winding_tally().relay_moved;
    court.reread_relay(&e, &carried, &mut budget);
    assert!(court.winding_tally().relay_moved > moved_before, "a transported coordinate reads as the coordinate that moved");
}
