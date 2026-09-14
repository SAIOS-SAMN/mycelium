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
    for (_, (_, r, _)) in sections.scoped() {
        assert!(
            !seen.iter().any(|h: &crate::transducer::alphabet::Rotor| h.projective_eq(r)),
            "the two sections are the two rotors, not one twice"
        );
        seen.push(r.clone());
    }
}

/// AN IDEA MEETS THE CHARTS IT STANDS ON, AND NO OTHERS. A Čech cell is the
/// overlap of two sections, and two sections overlap when they are sections
/// over one chart; a chart no vertex of this form acts on is not an overlap
/// with it.
///
/// `expand_with` reached the atlas through `children`: it took the idea's
/// chart, peeled up one stratum, and paged that stratum's SIBLINGS under a
/// budget, meeting whatever the trie held nearby. This fixture is the
/// difference. The idea's vertex stands on `[3, 5]`; `[3, 7]` is its sibling
/// under the same parent `[3]`, and holds a member. The sibling must not
/// enter the cover, because nothing of this idea is a section over it.
#[test]
fn an_idea_meets_its_own_charts_and_not_their_siblings() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::family::NoCrystal;
    use crate::transducer::sentence::{Form, Vertex};
    use crate::transducer::stalk::{Invariant, Stalks};
    let mine = Invariant(vec![3, 5]);
    let sibling = Invariant(vec![3, 7]);
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([4u8, 1]),
        Derivation::spelled(Word::from_letters(&[(q, false)])),
        mine.clone(),
        Box::new(element::identity_versor()),
        Vec::new(),
        None,
    ));
    let mut b = Bubble { form: Some(f), ..Default::default() };

    // Both charts hold a member, and both members read as sections.
    let mut atlas = Stalks::default();
    atlas.admit(&mine, b"on-my-chart");
    atlas.admit(&sibling, b"on-the-sibling");
    let section_at = |_: &[u8]| {
        Some((
            Word::from_letters(&[(q, false)]).rotor(),
            element::identity_versor(),
        ))
    };

    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let _ = b.expand_with(&atlas, &section_at, 0, &NoCrystal, &mut budget);
    assert!(
        b.cover.contains(mine.key().as_bytes()),
        "the chart this idea's vertex stands on IS met, so the reading is not empty"
    );
    assert!(
        !b.cover.contains(sibling.key().as_bytes()),
        "a sibling chart this idea holds no section over is not an overlap"
    );
}

/// RE-HOLDING AN UNMUTATED FORM IS NOT MOVEMENT, so a bubble ages.
///
/// `moved_k` is the PHASE CHANGE stamp — "TUniversal when this bubble's
/// charts last moved". `hold_form` rebuilt the whole bubble on every hold
/// and stamped `moved_k = now_k` each time; D8 re-holds a withheld form on
/// every stroke, so the birth coordinate was re-stamped every stroke and no
/// bubble could ever age. `held_since(now_k, moved_k)` was zero BY
/// CONSTRUCTION, `t_extent` perpetually zero, and lane 𝒯 could never
/// accumulate: 58 of 58 spoken magnitudes at 9a08fba carried 𝒯 = 0 while
/// `span_of` never once refused — it was answering `Some(0)` every time.
///
/// A form retained across an evaluation stroke has moved zero in
/// configuration space. The key IS the topology and the carriages (law 2),
/// so a bubble found under it is that same form unchanged.
#[test]
fn a_form_re_held_unchanged_keeps_its_phase_change_stamp_and_ages() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::sentence::{Form, Vertex};
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    let mut form = Form::default();
    form.vertices.push(Vertex::word_on(
        crate::transducer::address::Address::from_bytes([4u8, 9]),
        Derivation::spelled(Word::from_letters(&[(q, false)])),
        crate::transducer::stalk::Invariant(vec![3, 5]),
        Box::new(element::identity_versor()),
        Vec::new(),
        None,
    ));
    form.remainder = crate::transducer::element::PlacedChain::default();
    form.faces.push(form.face_of(0, 1).expect("the star face"));

    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold_form(&form, &[], std::slice::from_ref(&vec![(0i64, 0i64, 1u8, 2u8)]));
    let key = c.bubbles.scoped().next().map(|(k, _)| k).expect("held");
    assert_eq!(c.bubbles.get(&key).expect("held").moved_k, 10, "born at the stamp it entered on");
    // The same form re-held unchanged at a later TUniversal, as D8 re-holds
    // a withheld form every stroke.
    c.set_now(64);
    c.hold_form(&form, &[], std::slice::from_ref(&vec![(0i64, 0i64, 1u8, 2u8)]));
    let again = c.bubbles.get(&key).expect("still held");
    assert_eq!(again.moved_k, 10, "retention is not motion: the stamp is preserved");
    assert_eq!(
        crate::transducer::bubble::idea::held_since(c.now_k(), again.moved_k),
        Some(54),
        "so the bubble has aged 54 in TUniversal"
    );

    // And re-holding does not throw away what the bubble accumulated.
    assert_eq!(c.bubbles.len(), 1, "one form, one bubble, however often it is held");
}
