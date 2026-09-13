// 24.1 — the aperture over held thoughts: T room is measured in receipts,
// and the oldest bubble is the one that moves.

use crate::transducer::bubble::*;

#[test]
fn t_room_moves_the_oldest_bubble_to_cavity() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.set_now(50);
    c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
    assert_eq!(c.len(), 2);
    c.retire_on_t(1);
    assert_eq!(c.len(), 1, "T-room of one keeps the younger bubble");
    assert_eq!(c.cavity_len(), 1, "the older remainder is a cavity chart");
}

/// L7b, rule 5: a room of zero is no room. The aperture was floored at one
/// bubble, so a reserve with no T room still held a thought it could not
/// afford; the reserve decides, and zero means the last bubble goes too.
#[test]
fn a_room_of_zero_keeps_no_bubble() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    assert_eq!(c.len(), 1);
    c.retire_on_t(0);
    assert_eq!(c.len(), 0, "no room, no bubble");
    assert_eq!(c.cavity_len(), 1, "the remainder is kept as a cavity chart, never lost (law 7)");
}

/// L7b, law 9: the potential's T lane is the receipts since the idea's
/// charts last moved, and it is zero at the stroke it was held — not
/// floored to one so that a gate would pass. The pivot loop runs on the
/// idea regardless (25.1): a form with no remainder expands and files on
/// the very stroke it is held.
#[test]
fn a_held_idea_expands_at_zero_extent_and_its_potential_reads_zero_on_t() {
    use crate::transducer::family::NoCrystal;
    let mut c = Contemplation::new();
    c.set_now(7);
    c.hold(vec![(0, 0, 1, 2)]);
    let k = c.bubbles.scoped().next().map(|(k, _)| k).expect("held");
    let atlas = crate::transducer::stalk::Stalks::default();
    let mut budget = crate::physics::Physics::default().stroke_n(4);
    let (spent, _) = c
        .bubbles
        .get_mut(&k)
        .expect("held")
        .expand_with(&atlas, &|_| None, 7, &NoCrystal, &mut budget);
    let b = c.bubbles.get(&k).expect("still held");
    assert_eq!(b.potential[2], 0, "moved at K 7, read at K 7: zero receipts, zero on T");
    assert_eq!(
        b.potential,
        element::measure_lanes(&[(0, 0, 1, 2)]),
        "the other lanes are the remainder's own measure"
    );
    let _ = spent;
}

/// A FORM THAT SPEAKS SEVERAL ROTORS HAS SEVERAL SECTIONS, and the cover
/// sees all of them.
///
/// `Bubble::rotor` answers one rotor or absence, which is correct for
/// `rotor_between` — two bubbles have one transport or they have none — and
/// wrong for the cover, which asked it and RETURNED on the absence. So
/// every multi-rotor form was dropped from the expansion leaving no reading
/// at all: a filter whose non-matching arm says nothing, and the forms it
/// dropped are exactly the multi-section forms D7–D10 brought into
/// existence.
///
/// `sections` is keyed by rotor and carriage together, so a vertex acting
/// on two charts is two sections and two vertices holding one section are
/// one.
#[test]
fn a_form_speaking_two_rotors_carries_two_sections_and_no_single_rotor() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::sentence::{Form, Vertex};
    use crate::transducer::stalk::Invariant;
    let q01 = LetterId::quarter((0, 1)).expect("the sensor plane");
    let q02 = LetterId::quarter((0, 2)).expect("another plane");
    let mut f = Form::default();
    for (i, id) in [q01, q02].into_iter().enumerate() {
        f.vertices.push(Vertex::word_on(
            crate::transducer::address::Address::from_bytes([4u8, i as u8]),
            Derivation::spelled(Word::from_letters(&[(id, false)])),
            Invariant(vec![3, 5]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    let b = Bubble { form: Some(f), ..Default::default() };
    assert!(
        b.rotor().is_none(),
        "two rotors are not one rotor, and no representative is chosen (Y5)"
    );
    let sections = b.sections();
    assert_eq!(sections.len(), 2, "and the cover reads both of them, not neither");
    let mut seen = Vec::new();
    for (_, (r, _)) in sections.scoped() {
        assert!(
            !seen.iter().any(|h: &crate::transducer::alphabet::Rotor| h.projective_eq(r)),
            "the two sections are the two rotors, not one twice"
        );
        seen.push(r.clone());
    }
}
