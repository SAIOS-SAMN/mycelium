// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 37.7, A18, A20, 18.2 — GENERATOR ADJUNCTION, WORD CANDIDATES, AND ADMISSION:
//! what a mind adjoins when the basis reaches nothing, how candidates are
//! eliminated, and how acting words are filed.

use super::common::*;
use super::pivot_orbit::Hold;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element;
use crate::transducer::sentence::*;

/// 37.7 FROM AN ORBIT'S OWN RESIDUE — THE PIVOT ADJOINS WHAT NOTHING REACHED.
///
/// The basis reached no coordinate the orbit occupies, the class was
/// published on an earlier stroke and nothing answered, and the mesh has not
/// ruled it. Asking again asks the same question of the same basis. The move
/// left is to adjoin the rotor the orbit measured, which is the one
/// construction by which a mind gets a generator it was not given.
///
/// Both directions, because an adjunction on an orbit the basis CAN divide
/// would mint a duplicate of a word the mind already holds: an empty crystal
/// with the class published adjoins, and a crystal holding the word does
/// not.
#[test]
fn a_published_residue_no_generator_reaches_is_adjoined() {
    let q = order4();

    // PUBLISHED AND UNANSWERED, with nothing in the basis: adjoin.
    let mut f = chord_form(std::slice::from_ref(&q));
    // 54.7 — THE YIELD IS BOTH PLANES. `file_orbit` returns the labour the
    // filing earns, and that is `(ℓ₁ − 1) + (ℓ₂ − 1)` across the two
    // invariant planes of the double rotation, not `lcm − 1`. A quarter
    // turn is isoclinic here — `[4, 4]` — so it earns `3 + 3 = 6`, where
    // the envelope said `4`. The two planes are doing the work; the
    // envelope was never either plane's.
    assert_eq!(f.file_orbit(0), vec![6], "the class is published first, at its own spectrum");
    let snaps = f.pivot_orbit(
        [0x9a; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let Some(Snap::Adjoin { rotor, .. }) =
        snaps.iter().find(|s| matches!(s, Snap::Adjoin { .. }))
    else {
        panic!("a published residue nothing reaches is adjoined: {snaps:?}");
    };
    assert!(
        !rotor.is_identity(),
        "the adjoined rotor is the one the orbit measured, and it turns"
    );

    // THE BASIS REACHES IT: no adjunction, because the mind already holds
    // the word and minting a second is a duplicate generator.
    let mut g = chord_form(std::slice::from_ref(&q));
    // 54.7 — `(ℓ₁ − 1) + (ℓ₂ − 1)`: a quarter turn is isoclinic, `[4, 4]`,
    // so it earns `3 + 3 = 6` where the envelope said `4`.
    assert_eq!(g.file_orbit(0), vec![6]);
    let snaps = g.pivot_orbit(
        [0x9b; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        !snaps.iter().any(|s| matches!(s, Snap::Adjoin { .. })),
        "a residue the basis divides is not adjoined: {snaps:?}"
    );
}

/// 37.7 — A MINTED GENERATOR IS A WORD THE PIVOT CAN BE OFFERED.
///
/// `generator_form` turns a measured rotor the seventeen cannot spell into a
/// one-vertex closed form carrying `Derivation::adjoined`. That is the only
/// construction in the tree that makes a generator rather than receiving
/// one, and until it was wired to `Fill::Unspellable` nothing called it
/// except the court's demanded-prime path.
///
/// Minting is worth nothing unless the result is offerable, so this asserts
/// the whole chain on one rotor: the form carries the rotor at a vertex that
/// acts on the chart, `admit_words_acting` files it on the coordinates its
/// class occupies, and `on_axis_of` answers it there. A generator the index
/// cannot return is a word the mind holds and can never use — which is
/// exactly the state the fleet was measured in, `held 0` on every read.
#[test]
fn a_minted_generator_is_filed_and_answerable_on_its_own_coordinates() {
    use crate::transducer::sentence::{
        admit_words_acting, from_element_form, generator_form, on_axis_of, AxisIndex,
    };
    use crate::transducer::stalk::Invariant;
    let chart = Invariant::counts(&[1, 7, 4]);
    let rotor = order4().rotor();
    assert!(!rotor.is_identity(), "the rotor turns, or there is nothing to adjoin");

    let e = generator_form(&rotor, &chart, &[]).expect("a versor mints a generator");
    let f = from_element_form(&e).expect("the minted letter decodes to its form");
    assert!(
        f.is_closed(),
        "a letter is a Frame and one vertex, no faces and no orbits — closed vacuously"
    );

    // THE VERTEX CARRIES THE ROTOR, not a spelling of it: a `Word` travels
    // as its letter tape, so an adjoined generator that put the rotor in the
    // residue would reach the wire as the identity.
    // THE VERTEX IS READ AT ITS OWN ADDRESS (54.2). It was read at the
    // EMPTY address, because `generator_form` handed `word_on` an empty
    // one; a vertex now derives its address from its coordinates, so the
    // minted letter is found where it actually stands.
    let minted = f
        .vertices
        .iter()
        .find_map(|v| match v {
            crate::transducer::sentence::Vertex::Word { address, .. } => Some(address.clone()),
            _ => None,
        })
        .expect("the letter has a word vertex");
    let carried = crate::transducer::sentence::rotor_at_vertex(&f, minted.as_bytes());
    assert!(
        carried.is_some_and(|r| r.projective_eq(&rotor)),
        "the minted vertex carries the measured rotor"
    );

    // AND IT IS OFFERABLE. Filed where it acts, answered on every
    // coordinate its class occupies.
    let mut ix: AxisIndex = AxisIndex::new();
    admit_words_acting(&mut ix, &f);
    let occupied = ClassVector::of_word(&order4()).expect("class").coordinates();
    assert!(!occupied.is_empty(), "the rotor occupies a coordinate");
    for axis in &occupied {
        assert!(
            !on_axis_of(&ix, &chart, *axis).is_empty(),
            "the pivot can be offered the minted generator on {axis:?}"
        );
    }
}

/// 15.3, rule 11 — THE ELIMINATOR REFUSES ONLY WHAT THE ALGEBRA REFUSES.
///
/// Whether a held word can cancel an orbit's rotor is exact rational
/// arithmetic and needs no trial, no face write and no budget. Every
/// candidate used to pay a `faces.clone()`, an `orbits_of` walk and a full
/// `class_vectors_on` read to discover it.
///
/// THE TEST IS CONJUGACY. Composing the candidate onto the composite
/// directly is not the move the trial makes — the chord sits inside the
/// path product, `hw = A·e·B`, so position and traversal direction matter,
/// and a composition taken at the ends eliminates candidates that DO
/// discharge (measured: four fixtures went to `discharged 0`). What survives
/// the path's conjugation is the class: closing on the right means
/// `A·e·w⁻¹·B = 1`, so `w = B·A·e`, a cyclic permutation of `hw` and
/// therefore conjugate to it; on the left, conjugate to `hw⁻¹`.
///
/// Both directions are asserted, because an eliminator that refuses a real
/// discharge is worse than none: the word of the orbit's own class survives
/// and still discharges, and a word of a different class is refused without
/// a write.
#[test]
fn the_eliminator_keeps_every_word_that_can_cancel_and_drops_the_rest() {
    let q = order4();
    let half = order2();
    assert_ne!(
        q.rotor().class_key(),
        half.rotor().class_key(),
        "a quarter and a half turn are different classes, or this proves nothing"
    );

    // THE CLASS THAT CAN CANCEL still discharges, through the whole drive
    // and with the eliminator in the path.
    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_orbit(
        [0x7a; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { .. })),
        "the orbit's own class is not eliminated: {snaps:?}"
    );

    // A CLASS THAT CANNOT is refused, and the orbit files instead. The half
    // turn occupies an axis the quarter's orbit does not discharge on, so no
    // positioning of it closes the orbit and the algebra says so before any
    // face is written.
    let mut g = chord_form(std::slice::from_ref(&q));
    let snaps = g.pivot_orbit(
        [0x7b; 32],
        &Keyed::new(),
        &Hold(vec![half]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        !snaps.iter().any(|s| matches!(s, Snap::Discharged { .. })),
        "a word of another class discharges nothing: {snaps:?}"
    );
}

/// A20, R8, R12 — A WORD MOVE WRITES BOTH OF A FACE'S CARRIERS.
///
/// `holonomy_words_on` answers the EDGE COMPOSITE where a member path
/// carries one and the face word where it does not. `apply_by`'s word arm
/// wrote `f.word` alone, so on every path with an edge the class was read
/// from a carrier the move never touched and no `Word` candidate could ever
/// discharge.
///
/// Measured on the wire: `Placement` moves discharge and `Word` moves do
/// not — `axis Orientation(0) no discharge`, every occurrence, in every
/// window. `Placement` and `Hue` write `p.carriage` and `p.gauge`, which
/// `orbit_transport_on` composes and `ClassVector::read` takes; the word arm
/// wrote the one carrier the reading skips.
///
/// This fixture is an orbit whose path HAS an edge composite, which is the
/// case the old arm could not move: the quarter turn lives on the edge and
/// the face words are identity.
#[test]
fn a_word_move_moves_the_edge_composite_the_class_is_read_from() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::pivot::moves::{apply_by, unapply_by};
    use crate::transducer::stalk::Invariant;
    let chart = Invariant::counts(&[3, 5]);
    let pair = face_pair_key(&chart, &chart);
    let q = order4();
    // The two ends are two SITES: one chart each, so they are two vertices
    // rather than one named twice (54.2). The faces carry their own pair
    // key and their own edges, so the holonomy this test reads is unmoved.
    let word_vertex = |b: u8| {
        Vertex::word_on(
            Derivation::spelled(Word::identity()),
            Invariant::counts(&[u64::from(b)]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        )
    };
    let place = |edge: Option<Word>| FacePlace {
        carriage: element::identity_versor(),
        gauge: Vec::new(),
        edge: edge.map(|w| Transport {
            word: w,
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }),
    };
    let mut f = Form::default();
    f.vertices.push(word_vertex(1));
    f.vertices.push(word_vertex(2));
    for (i, j, edge) in [(0u16, 1u16, None), (0, 2, None), (1, 2, Some(q.clone()))] {
        let mut face = FormFace::new(i, j, Word::identity());
        face.at.insert(pair.clone(), place(edge));
        f.faces.push(face);
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let [lp, ..] = f.orbits.as_slice() else { panic!("one orbit") };
    let chord = lp.chord;
    let before = f.orbit_holonomy(lp).expect("one holonomy");
    assert!(
        !before.rotor().is_identity(),
        "the class is read off the edge composite, which turns"
    );

    // THE MOVE REACHES THE CARRIER, which is the whole claim: the old arm
    // wrote `f.word` and left this reading bit-identical.
    let by = PivotBy::Word { face: chord, word: Box::new(before.clone()), side: Side::Right };
    apply_by(&mut f.faces, &by);
    let read = |faces: &[FormFace], vs: &[Vertex]| {
        let orbits = Form::orbits_of(faces, vs);
        let [lp, ..] = orbits.as_slice() else { panic!("one orbit") };
        crate::transducer::sentence::pivot::drive::orbit_holonomy_on(faces, vs, lp)
            .expect("one holonomy")
    };
    let after = read(&f.faces, &f.vertices);
    assert!(
        !after.rotor().projective_eq(&before.rotor()),
        "the move moved the carrier the class is read from; it used to leave it untouched"
    );

    // AND THE CANCELLING DIRECTION ZEROES IT. The orbit runs the chord
    // backward, so its composite is the edge's inverse and the move that
    // closes it composes on the left.
    assert!(unapply_by(&mut f.faces, &by), "a word move inverts");
    let closing = PivotBy::Word { face: chord, word: Box::new(before.clone()), side: Side::Left };
    apply_by(&mut f.faces, &closing);
    let closed = read(&f.faces, &f.vertices);
    assert!(
        closed.rotor().is_identity(),
        "the composite cancels its own inverse on the carrier, got {:?}",
        closed.rotor()
    );
    assert!(unapply_by(&mut f.faces, &closing), "a word move inverts");

    // AND THE UNAPPLY RESTORED IT, carrier for carrier, or a refused
    // candidate leaves the edge composite moved for the next one.
    let restored = read(&f.faces, &f.vertices);
    assert!(
        restored.rotor().projective_eq(&before.rotor()),
        "the un-apply restores both carriers"
    );
}

/// 18.2, 12.6 — A WORD IS FILED UNDER WHAT IT ACTS BY, NOT UNDER ITS ROTOR
/// ALONE.
///
/// `admit_words` read `ClassVector::of_word`, which is the word at the
/// IDENTITY carriage and the IDENTITY hue. `Placement`, `Dilation` and
/// `Hue` are read off exactly those two coordinates, so three of the nine
/// axes of 12.6 held nothing however many words were admitted — while
/// `Pivot::candidates_for` asks `on_axis_of` for whichever axis the orbit
/// it is discharging stands on, those three among them. An orbit standing on
/// a placement was answered `0` not because the mind held no word that
/// places but because the filing could not say one did.
///
/// The vertex here carries the IDENTITY word and a translating carriage,
/// so its rotor occupies nothing at all: every coordinate it is filed
/// under comes from the assignment, and under the old reader it was filed
/// under none. That is the discrimination — a turning word would have been
/// filed on its plane axes either way and the fixture would have passed
/// without the change.
#[test]
fn a_word_is_filed_under_the_carriage_and_the_gauge_it_acts_by() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::{admit_words_acting, on_axis_of, retract_words_acting, AxisIndex};
    use crate::transducer::stalk::Invariant;
    let acting = Invariant::counts(&[1, 7, 4]);
    let gauge = vec![(1u8, 2u8), (2, 1)];
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        Derivation::spelled(Word::identity()),
        acting.clone(),
        Box::new(element::translator_versor(3, 5)),
        gauge.clone(),
        None,
    ));

    // THE ROTOR ALONE OCCUPIES NOTHING. This is the reader the admit used
    // and it is why the three assignment axes were unreachable.
    assert!(
        ClassVector::of_word(&Word::identity())
            .expect("the identity is a versor")
            .coordinates()
            .is_empty(),
        "an identity word at the identity assignment occupies no coordinate"
    );

    let mut ix: AxisIndex = AxisIndex::new();
    admit_words_acting(&mut ix, &f);

    for axis in [Axis::Placement, Axis::Hue] {
        let got = on_axis_of(&ix, &acting, axis);
        assert!(
            got.iter().any(|w| w.is_identity()),
            "the word is offered on {axis:?}, which its carriage and its gauge \
             put it on and its rotor never could; got {} words",
            got.len()
        );
    }
    // A dilation it does not carry is still empty: the assignment is read,
    // not assumed to occupy everything it could.
    assert!(
        on_axis_of(&ix, &acting, Axis::Dilation).is_empty(),
        "a pure translation carries no dilation residue"
    );

    // AND THE RETRACT MIRRORS IT. A retract that read the rotor alone would
    // compute a different coordinate set and empty nothing.
    retract_words_acting(&mut ix, &f);
    for axis in [Axis::Placement, Axis::Hue] {
        assert!(
            on_axis_of(&ix, &acting, axis).is_empty(),
            "what the admit filed at {axis:?} the retract empties"
        );
    }
    f.vertices.clear();
}

/// 18.2, A21 §2 — A WORD IS FILED WHERE IT ACTS, AND THE CHART IS A
/// PREFILTER.
///
/// `axis_admit` filed a sentence's words under `invariant_of(element)`,
/// which for a sentence is `chart_of_form` — `[2, rank, orbit classes]`, a
/// projection of what the FORM HOLDS. `candidates_for` asks under the charts
/// an orbit's face vertices act on — `[1, moduli]`, a statement about the BODY
/// A WORD ACTS ON. Both charts are well formed and answer different
/// questions, so the index was filed under one answer and read under the
/// other: measured across the fleet, `crystal.on_axis` returned 0 on 123 of
/// 123 reads, and 75 of 76 admits filed on no axis at all because the forms
/// admitted are closed and their words occupy nothing.
///
/// Two things are asserted, because either alone leaves the read empty. The
/// word is filed where its vertex ACTS, which is the question the pivot
/// asks. And the read COARSENS the chart until it is not empty, as
/// `generators_under` does — a stratum mismatch is a coarser question, not
/// an absent answer, and an exact keyed read could not tell those apart.
///
/// The chart never certifies. A word this returns is a candidate the trial
/// applies and re-reads off the written faces, refusing unless every member
/// path discharges the axis exactly.
#[test]
fn a_word_is_filed_where_it_acts_and_the_chart_coarsens_until_it_answers() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::{admit_words_acting, on_axis_of, retract_words_acting, AxisIndex};
    use crate::transducer::stalk::Invariant;
    let q = order4();
    let acting = Invariant::counts(&[1, 7, 4]);
    let mut f = Form::default();
    f.vertices.push(Vertex::word_on(
        Derivation::spelled(q.clone()),
        acting.clone(),
        Box::new(element::identity_versor()),
        Vec::new(),
        None,
    ));

    let occupied = ClassVector::of_word(&q).expect("q has a class").coordinates();
    assert!(
        !occupied.is_empty(),
        "a quarter turn occupies a coordinate, or there is nothing to file"
    );

    let mut ix: AxisIndex = AxisIndex::new();
    admit_words_acting(&mut ix, &f);

    // FILED WHERE IT ACTS: the chart the pivot asks under answers.
    for axis in &occupied {
        let got = on_axis_of(&ix, &acting, *axis);
        assert!(
            got.iter().any(|w| w.rotor().projective_eq(&q.rotor())),
            "the word is offered on {axis:?} at the chart its vertex acts on, got {} words",
            got.len()
        );
    }

    // AND THE CHART COARSENS. An orbit standing on a finer stratum than the
    // word was filed at asks a narrower question; the read widens it one
    // axis at a time rather than answering nothing.
    let finer = Invariant::counts(&[1, 7, 4, 9, 2]);
    for axis in &occupied {
        let got = on_axis_of(&ix, &finer, *axis);
        assert!(
            got.iter().any(|w| w.rotor().projective_eq(&q.rotor())),
            "a finer chart coarsens to the stratum that holds the word, {axis:?}"
        );
    }

    // AN AXIS THE WORD DOES NOT OCCUPY IS STILL EMPTY. The coarsening widens
    // the chart and never the axis, which is the coordinate itself.
    let unoccupied = (0..6u8)
        .map(Axis::Prime13)
        .find(|a| !occupied.contains(a))
        .expect("a quarter carries no prime-13 exponent on some plane");
    assert!(
        on_axis_of(&ix, &acting, unoccupied).is_empty(),
        "coarsening widens the chart, never the axis"
    );

    // AND THE RETRACT MIRRORS THE ADMIT, member for member.
    retract_words_acting(&mut ix, &f);
    for axis in &occupied {
        assert!(
            on_axis_of(&ix, &acting, *axis).is_empty(),
            "what the admit filed at {axis:?} the retract empties"
        );
    }
    f.vertices.clear();
}

/// 16.20, A20, I6, R8 — THE WHOLE `OpenAxis` PIPELINE READS ONE HOLONOMY,
/// and it is the edge composite.
///
/// `lp.holonomy` is the WORD. A face's word is the coboundary `w_j ∘ w_i⁻¹`
/// of its two vertices' words, and `∏ (w_j ∘ w_i⁻¹) ≡ 1` around any cycle
/// by algebra — an exact Čech coboundary, `δw = 0`. So every reader keyed to
/// it asked whether a boundary was non-trivial, which it never is.
///
/// The consequences were not one bug but four, each in a different organ:
/// `candidates_for`'s gate refused the widening on every coboundary orbit;
/// the seed was `versor_of_word(identity)`, an axis standing for nothing the
/// orbit turns by; `open_axis_on` filed under the identity's class; and
/// `opened_axis_on` looked the axis up under the identity's class, so the
/// audit could not see an opening that HAD been made. Measured at f3a76ec:
/// `opens 1` on 133 orbits, the single one whose word did not cancel, against
/// 92 torsion classes filed off the edge composite in the same five minutes.
///
/// This fixture is the disagreement itself: three coboundary faces, so the
/// word holonomy is the identity, and a quarter turn on the one face that
/// does not touch the Frame. Every site is asserted on that one orbit — the
/// keying invariant is total or it is not an invariant.
#[test]
fn the_open_axis_pipeline_keys_on_the_edge_composite_not_the_flat_word() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::{opened_axis_on, orbit_row_on};
    use crate::transducer::stalk::Invariant;
    let chart = Invariant::counts(&[3, 5]);
    let pair = face_pair_key(&chart, &chart);
    let q = order4();
    // The ends are two SITES: one chart each, so they are two vertices and
    // not one named twice (54.2). The faces carry their own pair key and
    // their own edges, so the holonomy this test reads is unmoved.
    let word_vertex = |b: u8| {
        Vertex::word_on(
            Derivation::spelled(Word::identity()),
            Invariant::counts(&[u64::from(b)]),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        )
    };
    let place = |edge: Option<Word>| FacePlace {
        carriage: element::identity_versor(),
        gauge: Vec::new(),
        edge: edge.map(|w| Transport {
            word: w,
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }),
    };
    let mut f = Form::default();
    f.vertices.push(word_vertex(1));
    f.vertices.push(word_vertex(2));
    for (i, j, edge) in [(0u16, 1u16, None), (0, 2, None), (1, 2, Some(q.clone()))] {
        let mut face = FormFace::new(i, j, Word::identity());
        face.at.insert(pair.clone(), place(edge));
        f.faces.push(face);
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let [lp, ..] = f.orbits.as_slice() else { panic!("three faces close one orbit") };
    let chord = lp.chord;

    // THE TWO CARRIERS DISAGREE, which is the precondition for the rest.
    assert!(
        lp.holonomy.rotor().is_identity(),
        "the word holonomy telescopes flat, as a product of coboundaries always does"
    );
    let composite = f.orbit_holonomy(lp).expect("the paths agree on one holonomy");
    assert!(
        !composite.rotor().is_identity(),
        "the edge composite carries the quarter the word cancelled"
    );

    // THE SEED comes from the composite, so it is a versor that turns.
    let seed = f.axis_seed_of(lp).expect("the composite seeds the axis");
    assert!(
        !element::same_versor(&seed, &element::identity_versor()),
        "seeded from the word this was the identity versor, standing for nothing"
    );

    // THE WRITER AND THE READER AGREE. An axis opened over this orbit is
    // found again by the reader that has to count it — keyed to the word,
    // `open_axis_on` filed under the identity's class and `opened_axis_on`
    // looked under it too, so the opening was invisible to the audit the one
    // time the classes differed.
    assert!(
        opened_axis_on(&f.faces, &f.vertices, &f.axes, lp).is_none(),
        "no axis is open yet"
    );
    let mut opened = f.clone();
    opened.open_axis_on(chord).expect("the axis opens over the composite's class");
    let [olp, ..] = opened.orbits.as_slice() else { panic!("one orbit") };
    assert_eq!(
        opened_axis_on(&opened.faces, &opened.vertices, &opened.axes, olp),
        Some(chord),
        "the reader finds the axis the writer filed"
    );
    assert_eq!(
        orbit_row_on(&opened.faces, &opened.vertices, &opened.axes, olp),
        Some(OrbitRow::Opened(
            composite.rotor().holonomy_key().expect("the composite has a class")
        )),
        "and the audit counts the orbit as opened under the composite's class, \
         not as the defect it was"
    );

    // AND THE INVERSE REMOVES WHAT THE OPENING WROTE. Keyed to the word,
    // `close_axis_on` removed nothing and the axis stood with no orbit
    // claiming it.
    opened.close_axis_on(chord).expect("the axis closes under the same class");
    let [clp, ..] = opened.orbits.as_slice() else { panic!("one orbit") };
    assert!(
        opened_axis_on(&opened.faces, &opened.vertices, &opened.axes, clp).is_none(),
        "the un-pivot removes the axis the pivot opened"
    );
}

/// A18, T21 (iii) — AN OPENING'S RANK IS READ OVER EVERY ROW, because the
/// axis carrier is keyed by CLASS and not by orbit.
///
/// `axes.insert(class, face)` files the opening under the orbit's holonomy
/// class, and `opened_axis_on` answers `Opened` for ANY orbit whose composite
/// class equals it. So a second orbit of the same class is opened by the same
/// write, and a rank-after that substituted only the trialled orbit's row
/// certified a move the form does not make. The drive catches the mismatch
/// at `apply_pivot` and holds the orbit — which is right, and is also a
/// certificate that should never have been written.
///
/// Measured at 551c609, fifteen minutes: 11 `axis Open discharged` against 6
/// `held, the applied pivot did not make the move its certificate claims`.
/// The 6 appeared in the same window the widening started firing at rate —
/// the defect was always there and nothing reached it while the gate was
/// keyed to the flat word.
///
/// A `Word`, `Placement` or `Hue` move writes one face's transport, so
/// exactly one orbit's holonomy moves and the substitution is the whole truth
/// for those. This asserts the certificate against the form for both kinds.
///
/// AND THE SAME HOLDS FOR EVERY MOVE, not only an opening — see
/// `a_word_moves_certified_rank_counts_the_siblings_it_moved` below, which is
/// the `Word` arm of the identical reading.
#[test]
fn an_openings_certified_rank_is_the_rank_the_form_moves_by() {
    let q = order4();
    let f = chord_form(&[q.clone(), q.clone()]);
    assert_eq!(f.orbits.len(), 2, "two chords of one class close two orbits");
    let [a, b] = f.orbits.as_slice() else { panic!("two orbits") };
    assert_eq!(
        f.orbit_holonomy(a).and_then(|h| h.rotor().class_key()),
        f.orbit_holonomy(b).and_then(|h| h.rotor().class_key()),
        "the two orbits carry ONE class, which is what the axis is keyed by"
    );

    let rows = f.orbit_rows().expect("rows");
    let rank_before = rank_of_rows(&rows).expect("rank");
    // Orbit 0 by the slice pattern, not by `first()`: the index IS content
    // here, because `trial` is asked about that orbit and no other (rule 15).
    let [lp, ..] = f.orbits.as_slice() else { panic!("two orbits") };
    let chord = lp.chord;
    let seed = f.axis_seed_of(lp).expect("the composite seeds the axis");
    let cand = Candidate {
        axis: Axis::Open,
        by: PivotBy::OpenAxis { face: chord, seed },
    };
    let prior = crate::transducer::sentence::pivot::drive::Prior {
        rank_before,
        crystal: &crate::transducer::family::NoCrystal,
        bundle: None,
        t_extent: 0,
        mu_before: None,
    };
    // ONE AXIS OVER A CLASS TWO ORBITS SHARE IS ONE STEP. The axis carrier is
    // keyed by class, so this write opens both orbits; counted by class, as
    // every coordinate of A18 is, that is `torsion −1, axes +1` and the
    // certificate states it. Counted by orbit it would read `[-2, 0, 0, +2]`
    // and T21 (iii) would refuse an elementary attachment as non-elementary.
    let mut f = f;
    let before = f.rank().expect("rank");
    let cert = f
        .trial(0, &cand, &f.class_vector(0).expect("class"), &prior)
        .expect("one axis over one class is one step");
    assert_eq!(
        cert.rank_move,
        RankMove::Discharged { from: RankAxis::Torsion },
        "the shared class leaves torsion once and lands on axes once"
    );
    f.apply_pivot(&cert);
    assert_eq!(
        f.rank().map(|r| Rank::moved(before, r)),
        Some(cert.rank_move),
        "the applied opening makes the move its certificate claims"
    );
    // AND IT KILLED BOTH REPRESENTATIVES, which is what opening by class
    // means: neither orbit is a defect any more.
    for i in 0..f.orbits.len() {
        assert!(
            f.class_vector(i).is_some_and(|k| k.is_zero()),
            "orbit {i} of the opened class reads as a coordinate, not a defect"
        );
    }

    // AND A ONE-ORBIT FORM CERTIFIES THE SAME ONE STEP.
    let mut one = chord_form(std::slice::from_ref(&q));
    let rows = one.orbit_rows().expect("rows");
    let rank_before = rank_of_rows(&rows).expect("rank");
    let [lp, ..] = one.orbits.as_slice() else { panic!("one orbit") };
    let cand = Candidate {
        axis: Axis::Open,
        by: PivotBy::OpenAxis {
            face: lp.chord,
            seed: one.axis_seed_of(lp).expect("seed"),
        },
    };
    let prior = crate::transducer::sentence::pivot::drive::Prior {
        rank_before,
        crystal: &crate::transducer::family::NoCrystal,
        bundle: None,
        t_extent: 0,
        mu_before: None,
    };
    let cert = one
        .trial(0, &cand, &one.class_vector(0).expect("class"), &prior)
        .expect("one orbit of one class opens in one unit");
    one.apply_pivot(&cert);
    assert_eq!(
        one.rank().map(|r| Rank::moved(rank_before, r)),
        Some(cert.rank_move),
        "the applied opening makes the move its certificate claims"
    );
}

/// 12.6, 16.16 step 3, A17, I6 — REDUCE, PUBLISH, THEN WIDEN.
///
/// A pivot is a REDUCTION: it divides the residual by a generator the mind
/// holds. Where the basis holds no word on the coordinate there is nothing
/// to divide by, every contraction candidate is terminal by algebra, and
/// asking again next stroke asks the same question of the same basis. The
/// move out of that is the reduction's inverse — widen the cover so the
/// class is re-read as a coordinate of the form rather than a defect in it
/// — but it is the THIRD move, not the second.
///
/// The second is 12.6 step 6: file the class, which puts it to the mesh for
/// a mind that CAN divide it. Widening first buries the defect under a
/// wider cover and asks nobody, and it also pre-empts the filing outright —
/// a discharging expansion wins `select_discharge` and the orbit never
/// reaches the filing arm. Measured: offered on irreducibility alone, six
/// fixtures that file a finite holonomy against an empty crystal stopped
/// filing and the whole filed-torsion path went dead.
///
/// `kappa.free_part()` alone gated the widening, and a free part is ONE way
/// a cover can be too narrow, not the only one. A quarter turn is finite
/// order — torsion, no prime exponent, not off-plane — so it has no free
/// part and never saw the move at all. On the fleet at 769a17c every orbit
/// with no candidate occupied `[Unit(0)]`, 60 of 60, every crystal read
/// answered 0, 123 of 123, and not one had a free part.
///
/// Every arm of the sequence is asserted, because the gate is the CRYSTAL'S
/// OWN ANSWER and the FORM'S OWN FILING, never a trial's failure.
#[test]
fn an_unanswered_irreducible_residual_is_widened_and_a_reducible_or_unasked_one_is_not() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let kappa = f.class_vector(0).expect("the chord closes one orbit");
    assert!(
        !kappa.free_part(),
        "a quarter turn is torsion: no prime exponent and not off-plane, so \
         the free-part gate alone never offers it the widening — {:?}",
        kappa.coordinates()
    );

    // FIRST: the basis divides it. There is something to reduce by, so the
    // contraction is offered and the widening is not.
    let unpublished: Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy> = Keyed::new();
    let reducible = f.candidates_for(0, &kappa, &Hold(vec![q.clone()]), false, &unpublished);
    assert!(
        reducible.iter().any(|c| c.axis != Axis::Open),
        "a held word on the coordinate is offered as a contraction: {:?}",
        reducible.iter().map(|c| c.axis).collect::<Vec<_>>()
    );
    assert!(
        !reducible.iter().any(|c| c.axis == Axis::Open),
        "a residual the basis can divide is not widened: {:?}",
        reducible.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // SECOND: nothing divides it and it has not been put to the mesh. The
    // move is to file, not to widen, so no candidate is offered at all and
    // the drive falls through to 12.6 step 6.
    let unasked =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, false, &unpublished);
    assert!(
        !unasked.iter().any(|c| c.axis == Axis::Open),
        "an unpublished residual is filed, not widened: {:?}",
        unasked.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // THIRD: the class has been filed — put to the mesh — and a stroke has
    // passed with nothing coming back. Now there is no move left but the
    // reduction's inverse. The filing is read as the PREVIOUS stroke's,
    // which is what the drive hands down: a class filed by this stroke is
    // not a demand anyone has had a stroke to answer.
    // 54.7 — THE YIELD IS BOTH PLANES. `file_orbit` returns the labour the
    // filing earns, and that is `(ℓ₁ − 1) + (ℓ₂ − 1)` across the two
    // invariant planes of the double rotation, not `lcm − 1`. A quarter
    // turn is isoclinic here — `[4, 4]` — so it earns `3 + 3 = 6`, where
    // the envelope said `4`. The two planes are doing the work; the
    // envelope was never either plane's.
    assert_eq!(
        f.file_orbit(0),
        vec![6],
        "the finite holonomy files under its class at four"
    );
    let published = f.torsion.clone();
    let asked =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, false, &published);
    assert!(
        asked.iter().any(|c| c.axis == Axis::Open),
        "a published, unanswered, irreducible residual is widened: {:?}",
        asked.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // AND THE HORIZON STILL WITHHOLDS IT. A class the mesh already holds is
    // not this mind's to open an axis over (A19): it waits for the form.
    let on_the_horizon =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, true, &published);
    assert!(
        !on_the_horizon.iter().any(|c| c.axis == Axis::Open),
        "a class the mesh holds is not widened: {:?}",
        on_the_horizon.iter().map(|c| c.axis).collect::<Vec<_>>()
    );
}

/// A18, T21 (iii) — A WORD MOVE'S CERTIFIED RANK COUNTS THE SIBLINGS IT
/// MOVED, because no move a pivot makes is confined to one orbit.
///
/// `trial` substituted row `idx` alone for `Word`, `Placement` and `Hue`,
/// on the ground that such a move writes one face's transport so exactly one
/// orbit's holonomy moves. The premise fails wherever two orbits SHARE that
/// face: the write is on the face, and every orbit through it reads the
/// carrier that moved. R13 found this for `OpenAxis` and fixed only that
/// arm; R16 made it unmissable for `Word`, because a word move now writes
/// the EDGE transports, which is exactly where a sibling orbit reads its
/// class from.
///
/// MEASURED at d1d84ca, the first soak with the whole block live: 50
/// occurrences of `axis Orientation(0) refused: rank moved [1, 0, 0, 0]` —
/// a torsion class appearing with none leaving, which is the signature of a
/// sibling row counted at its old value.
///
/// The fixture is two chords sharing the Frame's star faces, so a write on
/// one chord's face is read by both orbits. The assertion is the one the
/// drive makes at `apply_pivot`: whatever the trial certifies, the form
/// makes.
#[test]
fn a_word_moves_certified_rank_counts_the_siblings_it_moved() {
    let q = order4();
    let f = chord_form(&[q.clone(), q.clone()]);
    assert_eq!(f.orbits.len(), 2, "two chords close two orbits");
    let rows = f.orbit_rows().expect("rows");
    let rank_before = rank_of_rows(&rows).expect("rank");
    let [lp, ..] = f.orbits.as_slice() else { panic!("two orbits") };
    let chord = lp.chord;
    let kappa = f.class_vector(0).expect("class");

    // Every candidate the drive would offer on this orbit, trialled exactly as
    // the drive trials it — and every certificate any of them writes must
    // state the move the form actually makes.
    let prior = crate::transducer::sentence::pivot::drive::Prior {
        rank_before,
        crystal: &Hold(vec![q.clone()]),
        bundle: None,
        t_extent: 0,
        mu_before: None,
    };
    let unpublished: Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy> = Keyed::new();
    let cands = f.candidates_for(0, &kappa, &Hold(vec![q.clone()]), false, &unpublished);
    assert!(
        cands.iter().any(|c| matches!(c.by, PivotBy::Word { .. })),
        "the held quarter is offered as a word move on a shared face"
    );
    let mut certified = 0usize;
    for cand in &cands {
        let Some(cert) = f.trial(0, cand, &kappa, &prior) else {
            continue;
        };
        certified += 1;
        let mut applied = f.clone();
        applied.apply_pivot(&cert);
        assert_eq!(
            applied.rank().map(|r| Rank::moved(rank_before, r)),
            Some(cert.rank_move),
            "axis {:?}: the applied move is the move the certificate claims, \
             siblings on the shared face included",
            cand.axis
        );
    }
    assert!(
        certified > 0,
        "at least one candidate certifies, or this asserts nothing: chord {chord}"
    );
}

/// R79: FACE WORDS AND EDGE TRANSPORTS ADMIT BIDIRECTIONALLY ON MEMBER CHARTS.
///
/// An edge or face between member charts A and B was previously filed only under
/// A, leaving B without access to the transport, and was skipped entirely if its
/// rotor was the identity — dropping all placement, dilation and hue transports.
/// With bidirectional admission, both A and B are indexed with the direct and
/// inverse transports, and non-rotor carriages are filed on their proper axes.
#[test]
fn face_words_and_edge_transports_admit_bidirectionally_on_member_charts() {
    use crate::transducer::sentence::{
        admit_words_acting, on_axis_of, retract_words_acting, AxisIndex, FacePlace, FormFace,
    };
    use crate::transducer::stalk::Invariant;

    let chart_a = Invariant::counts(&[1, 7, 4]);
    let chart_b = Invariant::counts(&[1, 7, 5]);
    let q = order4();

    let mut f = Form::default();
    let mut face = FormFace::new(0, 1, q.clone());
    let edge_transport = Transport {
        word: Word::identity(),
        carriage: element::translator_versor(2, 4),
        gauge: Vec::new(),
    };
    let place = FacePlace {
        carriage: element::translator_versor(1, 1),
        gauge: Vec::new(),
        edge: Some(edge_transport),
    };
    face.at.insert(
        crate::transducer::sentence::face_pair_key(&chart_a, &chart_b),
        place,
    );
    f.faces.push(face);

    let mut ix = AxisIndex::new();
    admit_words_acting(&mut ix, &f);

    // Both charts receive the edge placement (direct and inverse carriage)
    assert!(
        !on_axis_of(&ix, &chart_a, Axis::Placement).is_empty(),
        "chart A holds the edge placement transport"
    );
    assert!(
        !on_axis_of(&ix, &chart_b, Axis::Placement).is_empty(),
        "chart B holds the inverse edge placement transport"
    );

    // Both charts receive the face word turning coordinates
    let occupied = ClassVector::of_word(&q).expect("class").coordinates();
    for axis in &occupied {
        assert!(
            !on_axis_of(&ix, &chart_a, *axis).is_empty(),
            "chart A holds the turning face word on {axis:?}"
        );
        assert!(
            !on_axis_of(&ix, &chart_b, *axis).is_empty(),
            "chart B holds the inverse turning face word on {axis:?}"
        );
    }

    // Retract clears both charts
    retract_words_acting(&mut ix, &f);
    assert!(
        on_axis_of(&ix, &chart_a, Axis::Placement).is_empty(),
        "retract clears chart A placement"
    );
    assert!(
        on_axis_of(&ix, &chart_b, Axis::Placement).is_empty(),
        "retract clears chart B placement"
    );
    for axis in &occupied {
        assert!(
            on_axis_of(&ix, &chart_a, *axis).is_empty(),
            "retract clears chart A on {axis:?}"
        );
        assert!(
            on_axis_of(&ix, &chart_b, *axis).is_empty(),
            "retract clears chart B on {axis:?}"
        );
    }
}
