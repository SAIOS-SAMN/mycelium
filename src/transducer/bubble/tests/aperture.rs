// 24.1 — the aperture over held thoughts: T room is measured in receipts,
// and the oldest bubble is the one that moves.

use crate::transducer::bubble::*;

/// Every bubble the contemplation holds, by address — what a door is
/// handed. `retire_on_t` picked ONE by age against a count; the doors are
/// handed the set that lapsed and retire it whole.
fn all_keys(c: &Contemplation) -> Vec<Vec<u8>> {
    c.bubbles.scoped().map(|(k, _)| k).collect()
}

/// 25, Z4b — NO HOLD IS RETIRED BY AGE OR BY A COUNT.
///
/// THIS TEST ASSERTED THE OPPOSITE AND ITS CLAIM IS FALSE UNDER THE RULING.
/// It was `t_room_moves_the_oldest_bubble_to_cavity`: two bubbles, a room
/// of one, and the OLDER retired. `retire_on_t` is deleted and both halves
/// of what it did are refused — a count against a bound is rule 2's and
/// the bound was the box's RAM (rule 5, `reserve_max` probed at 88), and
/// picking the oldest is rule 4's, a selection by a scalar.
///
/// What replaces it is the three doors, and none of them reads an age or a
/// count: a hold lapses with what it cites, is retired at its promotion,
/// and a lane with no room WITHHOLDS rather than evicting. So two bubbles
/// nothing has lapsed or promoted both stand, however old either is.
#[test]
fn no_hold_is_retired_by_age_or_by_a_count() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.set_now(50);
    c.hold(vec![(5, 5, 3, 4), (5, 6, 3, 4)]);
    assert_eq!(c.len(), 2, "two thoughts stand");
    // Time passes and nothing lapses: age is not a door.
    c.set_now(5_000);
    assert_eq!(c.len(), 2, "age alone retires nothing");
    assert_eq!(c.cavity_len(), 0, "and nothing was evicted to the cavity");
    // The door retires what it is handed, as a set, and the remainders are
    // kept exactly as a retirement always kept them (law 7).
    let keys = all_keys(&c);
    assert_eq!(c.lapse_with(&keys), 2, "the door retires the set it is handed");
    assert_eq!(c.len(), 0);
    assert_eq!(c.cavity_len(), 2, "both remainders are kept, neither lost");
}

/// 24.1, law 7, rule 1 — A RETIRED REMAINDER IS FILED UNDER ITS CHART, and
/// the drain reads at that chart instead of copying the carrier.
///
/// `retire_on_t` writes one cavity entry per retirement, every stroke, and
/// the only drain is `pivot_on`'s. Keyed by the bubble alone that drain
/// could not ASK: it would clone every entry, key and whole remainder,
/// re-derive each one's chart, and keep the few that matched — a carrier
/// that grows on the hot path and drains on a coincidence.
///
/// The chart leads the key and the bubble follows, so two remainders retired
/// at one chart stay two (law 7) and the entries a chart can re-hold are the
/// subtree under it.
#[test]
fn a_cavity_entry_is_read_back_at_its_own_chart_and_two_at_one_chart_are_two() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    c.set_now(20);
    c.hold(vec![(9, 9, 5, 6), (9, 10, 5, 6)]);
    assert_eq!(c.len(), 2, "two bubbles held");

    // Both retire, and both are kept: law 7 loses no remainder, and two
    // remainders are two entries however their charts compare.
    // The CLAIM here is the cavity's key shape and is unchanged; only the
    // trigger is, from `retire_on_t`'s age pick to the door's set.
    let keys = all_keys(&c);
    c.lapse_with(&keys);
    assert_eq!(c.len(), 0, "the door retired both");
    assert_eq!(c.cavity_len(), 2, "both remainders are kept, neither collapsed");
    assert_eq!(
        c.cavity_links(),
        4,
        "and the links they hold are the links they had, not a count of entries"
    );

    // THE KEY IS THE CHART, so the entries a chart can re-hold are a keyed
    // read under it — which is what the drain reads, and what a key that
    // was the bubble's could not answer.
    //
    // Both remainders are two adjacent links, so they are the SAME SHAPE and
    // share a chart: the fibre answers both, and the bubble suffix is what
    // keeps them two entries rather than one overwriting the other.
    let two_link = crate::transducer::stalk::invariant(&[(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2)]);
    let under = c
        .cavity_keys_under(&two_link)
        .expect("a chart names a fibre of the cavity");
    assert_eq!(
        under.len(),
        2,
        "one chart, two remainders, two entries — the suffix keeps them apart"
    );

    // A REMAINDER OF ANOTHER SHAPE STANDS AT ANOTHER CHART, and the prefix
    // separates them. Without this the read above would pass on a cavity
    // that answered everything to everything.
    c.set_now(30);
    c.hold(vec![(2, 2, 7, 8), (2, 3, 7, 8), (3, 2, 7, 8)]);
    let keys = all_keys(&c);
    c.lapse_with(&keys);
    assert_eq!(c.cavity_len(), 3, "three remainders kept");
    let three_link =
        crate::transducer::stalk::invariant(&[(2i64, 2i64, 7u8, 8u8), (2, 3, 7, 8), (3, 2, 7, 8)]);
    assert_ne!(
        three_link.key(),
        two_link.key(),
        "three links in an L is not two in a row, or this proves nothing"
    );
    assert_eq!(
        c.cavity_keys_under(&three_link).expect("a fibre").len(),
        1,
        "the L's chart answers the L alone"
    );
    assert_eq!(
        c.cavity_keys_under(&two_link).expect("a fibre").len(),
        2,
        "and the pair's chart still answers the pair"
    );
}

/// Law 7: A RETIREMENT LOSES NO REMAINDER, whichever door took the bubble.
///
/// Was `a_room_of_zero_keeps_no_bubble`, which asserted that a reserve with
/// no T room evicts the last thought. That claim is false under the ruling
/// — door 3 WITHHOLDS a new thought when a lane is spent and never evicts a
/// held one, because eviction chooses. What survives from it, and is
/// asserted here, is law 7: the retired remainder is kept as a cavity chart
/// and never lost.
#[test]
fn a_retired_remainder_is_kept_as_a_cavity_chart() {
    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
    assert_eq!(c.len(), 1);
    let keys = all_keys(&c);
    c.lapse_with(&keys);
    assert_eq!(c.len(), 0, "the door retired it");
    assert_eq!(c.cavity_len(), 1, "the remainder is kept as a cavity chart, never lost (law 7)");
}

/// L7b, law 9: the potential's T lane is the receipts since the idea's
/// charts last moved, and it is zero at the stroke it was held — not
/// floored to one so that a gate would pass. The pivot orbit runs on the
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
    let spent = c
        .bubbles
        .get_mut(&k)
        .expect("held")
        .expand_with(&atlas, &|_| None, 7, &NoCrystal, &mut budget, &mut Default::default())
        .spent;
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
/// wrong for the cover: a cover that asked it and returned on the absence
/// would drop every multi-rotor form from the expansion with no reading at
/// all, a filter whose non-matching arm says nothing.
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
    for id in [q01, q02] {
        f.vertices.push(Vertex::word_on(
            Derivation::spelled(Word::from_letters(&[(id, false)])),
            Invariant::counts(&[3, 5]),
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

/// 17.2, A4 — A SECTION IS KEYED BY THE CARRIAGE RAY. Two vertices holding
/// one rotor on one chart with carriages `c` and `−c` hold one section; raw
/// coefficients keyed them as two.
#[test]
fn two_vertices_whose_carriages_differ_by_sign_hold_one_section() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::sentence::{Form, Vertex};
    use crate::transducer::stalk::Invariant;
    let q01 = LetterId::quarter((0, 1)).expect("the sensor plane");
    let c = element::translator_versor(1, 2);
    let neg = c.scale(&num_rational::BigRational::from_integer(num_bigint::BigInt::from(-1)));
    let mut f = Form::default();
    for car in [c, neg] {
        f.vertices.push(Vertex::word_on(
            Derivation::spelled(Word::from_letters(&[(q01, false)])),
            Invariant::counts(&[3, 5]),
            Box::new(car),
            Vec::new(),
            None,
        ));
    }
    let b = Bubble { form: Some(f), ..Default::default() };
    assert_eq!(b.sections().len(), 1, "one carriage ray on one chart is one section, not one per sign");
}

/// EXPANSION 4 — A RULED WORD THAT IS ONE CANDIDATE OF A HELD VORTEX
/// COLLAPSES IT ON PIVOT, AND THE SYMMETRY IS CARRIED OUT. A vortex holds
/// two certified discharges of one orbit; when a word whose rotor is one
/// candidate arrives (a ruled word, learned), `pivot_on` collapses the
/// vortex by that candidate and the difference `w_a⁻¹·w_b` — a rotation
/// the mind measured (37.7) — rides out as an adjunction candidate rather
/// than being forgotten with the loser.
///
/// THE VORTEX IS HELD UNDER THE ORBIT'S AFTER-CLASS, which here is the zero
/// class — both candidates closed the orbit — and the arriving word's rotor
/// has a class of its own. The collapse is read at the CANDIDATE's key, so
/// the after-class is not a coordinate the word has to know; read at the
/// rotor's class it would find nothing.
#[test]
fn a_learned_candidate_collapses_the_vortex_and_its_symmetry_is_carried_out() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::sentence::{
        Axis, ClassVector, Form, Mu, PivotBy, PivotCertificate, RankMove, Side, Vertex,
    };
    use crate::transducer::stalk::Invariant;
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    let a = Word::from_letters(&[(q, false)]);
    let b = Word::from_letters(&[(q, true)]);
    let symmetry = a.inverse().compose(&b).normal_form().rotor();
    assert!(!symmetry.is_identity(), "q⁻¹·q⁻¹ is a half turn, not the identity");

    // A form with one face for the certificate to name.
    let mut f = Form::default();
    for w in [a.clone(), b.clone()] {
        f.vertices.push(Vertex::word_on(
            Derivation::spelled(w),
            Invariant::counts(&[3, 5]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
    }
    f.faces.push(f.face_carrying(1, 2, a.clone()).expect("a chord between two placed vertices"));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();

    // The vortex: two candidates that discharge ONE orbit, so they share
    // that orbit's class vector as their `before` and differ only in the
    // move. Both closed it, so the class after is the zero class — which
    // is why filing under `after` put every closed vortex in the tree
    // under one key and left nothing for a deciding word to find.
    let before = ClassVector::of_word(&a)
        .expect("a quarter turn has a class vector")
        .class_key();
    let after = ClassVector::zero().class_key();
    assert_ne!(after, before, "a closed orbit's class is not the class it stood in");
    let cert = |w: &Word| PivotCertificate {
        chord: 0,
        axis: Axis::Open,
        before: before.clone(),
        after: after.clone(),
        rank_move: RankMove::Equal,
        mu: Mu::Unmeasured,
        by: PivotBy::Word { face: 0, word: Box::new(w.clone()), side: Side::Right },
    };
    let mut vortex = crate::network::nerve::Keyed::new();
    for c in [cert(&a), cert(&b)] {
        crate::transducer::sentence::vortex_put(&mut vortex, c);
    }
    // The vortex is the FORM's carrier; a bubble mirrors it from the form
    // it takes (`take_form`), so it is written where the pivot reads it.
    f.vortex = vortex.clone();

    // The arriving word: the candidate `a`, LEARNED AS A FORM — a generator
    // form, as a ruled letter arrives from the syllabus — whose chart is a
    // form chart and shares no prefix with the bubble's cover of body
    // charts. The collapse is read at the candidate's own key, so the
    // bubble is found whatever it covers and whatever class the orbit was
    // left in.
    let word = crate::transducer::sentence::generator_form(&a.rotor(), &Invariant::counts(&[3, 5]), &[])
        .expect("a rotor mints a generator");
    let mut cover = crate::network::nerve::Keyed::new();
    cover.insert(Invariant::counts(&[1, 3, 5]).key(), ());
    let key = crate::transducer::bubble::idea::idea_key(&f);
    let bubble = Bubble { form: Some(f), vortex, cover, ..Default::default() };
    let mut c = Contemplation::new();
    c.admit_incidence(&std::sync::Arc::from(&key[..]), &bubble);
    c.bubbles.insert(&key, bubble);
    // THE STRATUM IS THE COORDINATE, and it is the depth-one projection of
    // the class vector the candidate hangs under — not a second key written
    // to agree with it.
    let stratum = a.rotor().class_key().expect("a quarter turn has a class");
    let end = ClassVector::class_schema()
        .boundary(before.as_bytes(), 1)
        .expect("the class vector spells its first coordinate");
    assert_eq!(
        before.as_bytes().get(..end),
        Some(stratum.as_bytes()),
        "the deciding word's class IS the orbit class's first coordinate"
    );
    // Same claim, read through the ordinal the index now holds (Z4 item 2).
    let filed_here = c.by_vortex.get(stratum.as_bytes()).is_some_and(|m| {
        m.scoped()
            .filter_map(|(k, _)| Contemplation::ordinal_of_key(&k))
            .filter_map(|o| c.address_at(o))
            .any(|a| a == key.as_bytes())
    });
    assert!(filed_here, "the vortex is filed at its conjugacy stratum");
    assert!(
        c.by_vortex.get(after.as_bytes()).is_none(),
        "and not at the class the move would leave the orbit in"
    );

    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let (_, _, _, _, adjoined) = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    let [(rotor, _)] = adjoined.as_slice() else {
        panic!("one symmetry rides out of the collapse, got {}", adjoined.len())
    };
    assert!(rotor.projective_eq(&symmetry), "the symmetry is w_a⁻¹·w_b");
    assert!(
        c.bubbles.get(&key).is_some_and(|b| b.vortex.is_empty()),
        "and the vortex collapsed"
    );
    assert!(
        c.by_vortex.get(stratum.as_bytes()).is_none_or(|m| !m.contains(&key)),
        "and its index entry left with it"
    );
}

/// AN IDEA MEETS THE CHARTS IT STANDS ON, AND NO OTHERS. A Čech simplex is
/// the overlap of two sections, and two sections overlap when they are
/// sections over one chart; a chart no vertex of this form acts on is not
/// an overlap with it.
///
/// This fixture is the difference between incidence in the sheaf and
/// adjacency in the trie. The idea's vertex stands on `[3, 5]`; `[3, 7]` is
/// its sibling under the same parent `[3]`, and holds a member. The sibling
/// must not enter the cover, because nothing of this idea is a section over
/// it.
#[test]
fn an_idea_meets_its_own_charts_and_not_their_siblings() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::family::NoCrystal;
    use crate::transducer::sentence::{Form, Vertex};
    use crate::transducer::stalk::{Invariant, Stalks};
    let mine = Invariant::counts(&[3, 5]);
    let sibling = Invariant::counts(&[3, 7]);
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
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
    let _ = b.expand_with(&atlas, &section_at, 0, &NoCrystal, &mut budget, &mut Default::default());
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
/// charts last moved". D8 re-holds a withheld form on every stroke, so a
/// `hold_form` that rebuilt the whole bubble and stamped `moved_k = now_k`
/// each time would re-stamp the birth coordinate every stroke and no bubble
/// could ever age: `held_since(now_k, moved_k)` zero BY CONSTRUCTION,
/// `t_extent` perpetually zero, lane 𝒯 never accumulating, and `span_of`
/// answering `Some(0)` every time.
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
        Derivation::spelled(Word::from_letters(&[(q, false)])),
        crate::transducer::stalk::Invariant::counts(&[3, 5]),
        Box::new(element::identity_versor()),
        Vec::new(),
        None,
    ));
    form.remainder = crate::transducer::element::PlacedChain::default();
    form.faces.push(form.face_of(0, 1).expect("the star face"));

    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold_form(form.clone(), &[], std::slice::from_ref(&vec![(0i64, 0i64, 1u8, 2u8)]), [0; 4]);
    let key = c.bubbles.scoped().next().map(|(k, _)| k).expect("held");
    assert_eq!(c.bubbles.get(&key).expect("held").moved_k, 10, "born at the stamp it entered on");
    // The same form re-held unchanged at a later TUniversal, as D8 re-holds
    // a withheld form every stroke.
    c.set_now(64);
    c.hold_form(form.clone(), &[], std::slice::from_ref(&vec![(0i64, 0i64, 1u8, 2u8)]), [0; 4]);
    let again = c.bubbles.get(&key).expect("still held");
    assert_eq!(again.moved_k, 10, "retention is not motion: the stamp is preserved");
    assert_eq!(
        crate::transducer::bubble::idea::held_since(c.now_k(), again.moved_k),
        Some(54),
        "so the bubble has aged 54 in TUniversal"
    );

    // And re-holding does not throw away what the bubble accumulated.
    assert_eq!(c.bubbles.len(), 1, "one form, one bubble, however often it is held");

    // A SPAN OF ZERO HAS TWO CAUSES AND THEY ARE DIFFERENT FACTS: a form held
    // and not yet aged, and a form never held at all. Reported as one `0`
    // the two are unreadable on the wire, with no way to tell which path
    // produced it.
    use crate::transducer::bubble::contemplation::Deliberation;
    assert_eq!(
        c.span_of(&form),
        Some(Deliberation::Held(54)),
        "a held form reports the extent it deliberated"
    );
    let mut other = Form::default();
    other.vertices.push(Vertex::word_on(
        Derivation::spelled(Word::from_letters(&[(q, false)])),
        crate::transducer::stalk::Invariant::counts(&[9, 9]),
        Box::new(element::identity_versor()),
        Vec::new(),
        None,
    ));
    assert_eq!(
        c.span_of(&other),
        Some(Deliberation::Fresh),
        "a form this contemplation never held is fresh, not a held form aged zero"
    );
    assert_eq!(Deliberation::Fresh.span(), 0, "and it stood over no receipts");
}

/// LINEAGE SURVIVES MUTATION: the entry follows the form it holds.
///
/// A bubble's key is its form's address, and a pivot REWRITES THE FORM IN
/// PLACE — `expand_all` takes the slot by the key it entered with and
/// `expand_with` calls `take_form` inside it. Left where it was, the bubble
/// would keep its inception stamp and everything it had built while its key
/// stopped matching the form it now held, and every reader looking it up by
/// the current form would miss it: `span_of` answering "never held" for a
/// bubble that has been deliberating for thousands of receipts, and
/// `hold_form` inserting a SECOND bubble for the same lineage while the
/// first is orphaned under a key nothing computes any more. A form with an
/// open remainder speaks only once it closes, and closing mutates it.
#[test]
fn a_bubble_rekeyed_by_a_mutation_keeps_its_inception() {
    use crate::transducer::alphabet::{Derivation, LetterId, Word};
    use crate::transducer::bubble::contemplation::Deliberation;
    use crate::transducer::sentence::{Form, Vertex};
    let q = LetterId::quarter((0, 1)).expect("the sensor plane");
    // The forms must differ on a coordinate the KEY carries. A vertex's own
    // address is deliberately not one of them (1.5: two minds whose
    // derivations differ reach one address), so they differ by the chart the
    // vertex acts on — which is what a rebase or a refinement moves.
    let build = |chart: Vec<u64>| {
        let mut f = Form::default();
        f.vertices.push(Vertex::word_on(
            Derivation::spelled(Word::from_letters(&[(q, false)])),
            crate::transducer::stalk::Invariant::counts(&chart),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        ));
        f.faces.push(f.face_of(0, 1).expect("the star face"));
        f
    };
    let before = build(vec![3, 5]);
    let after = build(vec![7, 11]);
    let rem = vec![(0i64, 0i64, 1u8, 2u8)];

    let mut c = Contemplation::new();
    c.set_now(10);
    c.hold_form(before.clone(), &[], std::slice::from_ref(&rem), [0; 4]);
    let old_key = crate::transducer::bubble::idea::idea_key(&before);
    let new_key = crate::transducer::bubble::idea::idea_key(&after);
    assert_ne!(old_key, new_key, "the mutation is a different form, so a different key");

    // The form mutates in place, as a pivot rewrites it, and the entry
    // follows.
    c.set_now(90);
    c.rekey(old_key.as_bytes(), new_key.as_bytes());
    assert!(c.bubbles.get(old_key.as_bytes()).is_none(), "no orphan is left behind");
    assert_eq!(c.bubbles.len(), 1, "one lineage, one bubble");
    assert_eq!(
        c.bubbles.get(new_key.as_bytes()).expect("moved").moved_k,
        10,
        "and it carries the inception it deliberated from, not the mutation's stamp"
    );
    assert_eq!(
        c.span_of(&after),
        Some(Deliberation::Held(80)),
        "so the resolved form reports the true extent of its deliberation"
    );
}
