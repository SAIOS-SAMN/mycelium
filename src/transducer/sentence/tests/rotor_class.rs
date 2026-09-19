// 16 — THE ROTOR and ITS CLASS: what order a word has, what is filed as
// torsion, and what is refused rather than read as the zero class.

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Order, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

/// 12.6, 16.16 step 3, 3.7 (I6): a loop whose class has a free part and
/// which no held word discharges OPENS A CHART AXIS seeded by its own
/// holonomy — the residual measured how narrow the cover was, and the
/// cover grew an axis along which it is a section. The loop then reads
/// zero, the rank moves one unit from `free` into `axes`, the form coheres
/// on the refined cover, and it is never filed as torsion (T3). The
/// certificate replays on the court, which re-derives the seed from the
/// loop; a tape whose axis was struck out is refused on that loop.
#[test]
fn an_infinite_class_no_word_discharges_opens_an_axis_and_is_never_filed() {
    let mut f = chord_form(&[infinite()]);
    let before = f.rank().expect("rank");
    assert_eq!((before.free, before.axes), (1, 0), "one free loop, no axis yet");
    let snaps = f.pivot_loop(
        [0x11; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let discharged: Vec<&PivotCertificate> = snaps
        .iter()
        .filter_map(|s| match s {
            Snap::Discharged { certificate, .. } => Some(certificate.as_ref()),
            _ => None,
        })
        .collect();
    let [certificate] = discharged[..] else {
        panic!("the opening axis discharges the loop once: {snaps:?}");
    };
    assert_eq!(certificate.axis, Axis::Open, "on the opening axis");
    assert_eq!(
        certificate.rank_move,
        RankMove::Discharged { from: RankAxis::Free },
        "one unit left free for axes"
    );
    assert!(matches!(certificate.by, PivotBy::OpenAxis { .. }), "the move is the opened axis");
    assert!(
        !snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { .. } | Snap::Demand { .. })),
        "neither filed nor demanded: {snaps:?}"
    );
    assert!(f.torsion.is_empty(), "an infinite class is never a large-ℓ torsion cycle");
    assert_eq!(f.axes.len(), 1, "the form carries the opened axis");
    let after = f.rank().expect("rank");
    assert_eq!((after.free, after.axes), (0, 1), "the free unit is an axis now");
    assert!(f.coheres(), "the loop is the transition on the opened axis");
    assert!(
        f.class_vector(0).is_some_and(|k| k.is_zero()),
        "and it reads as the zero vector"
    );

    // The court replays the opened axis from the tape and the loop.
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("crystallizes");
    let back = crate::transducer::sentence::from_element_form(&e).expect("tape");
    assert_eq!(back.axes.len(), 1, "the axis rides the tape");
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "the court re-derives the seed from the loop and the certificate re-walks"
    );
    let mut struck = back.clone();
    struck.axes = Keyed::new();
    let bad = as_element_form(&struck, [1, 0, 0, 0]).expect("tape without its axis");
    assert_eq!(
        crate::court::verdict::blind_replay(&bad),
        Some(crate::court::verdict::REFUSE_LOOP),
        "without the axis the loop is a free class again, and it does not replay"
    );
}

#[test]
fn torsion_filed_only_when_witnessed_at_period() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = element::from_canonical(&crate::transducer::germinate::germinate(&a, &b).unwrap());
    t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    assert_eq!(t.torse(), 1, "the hue cycle is one torsion generator");
    let ws = crate::transducer::torsion_witness::witness_stored(&t);
    assert!(
        ws.iter().any(|(ell, w)| *ell == 3
            && matches!(w, crate::transducer::torsion_witness::HolonomicLoopWitness::Closed { period: 3, .. })),
        "period 3 is witnessed closed: {ws:?}"
    );
    t.record_torsion(vec![(1, 9)], 8, crate::transducer::element::FIBER_SPATIAL);
    let ws = crate::transducer::torsion_witness::witness_stored(&t);
    assert!(
        ws.iter().any(|(ell, w)| *ell == 8
            && matches!(w, crate::transducer::torsion_witness::HolonomicLoopWitness::Open { .. })),
        "an unclosed walk is not believed: {ws:?}"
    );
}

#[test]
fn rank_offplane_is_its_own_lane() {
    // A ROTOR WITH NO INVARIANT PAIR PLANE. It was a `(0, 2)` quarter,
    // which was off-plane only because the speller could not see its
    // plane; it turns plane one and reads as torsion now. Two quarters on
    // planes sharing an axis leave no two-subset invariant, and that is
    // what the off-plane lane counts.
    let w = Word::from_letters(&[
        (alphabet::LetterId::quarter((0, 1)).unwrap(), false),
        (alphabet::LetterId::quarter((0, 2)).unwrap(), false),
    ]);
    assert!(w.plane_block().is_none(), "it moves the sensor plane");
    assert!(w.form().is_none(), "and turns no pair plane at all");
    let two = chord_form(&[w.clone(), w.clone()]);
    let r = two.rank().expect("rank");
    assert_eq!(r.offplane, 1, "two chords of one off-plane class count once, got {r:?}");
    assert_eq!(r.free, 0, "plane free rank is empty, got {r:?}");
    assert_ne!(r.offplane, 2, "the old per-chord count is not a coordinate");
    assert_ne!(r.free, 2, "the old folded sum is not free");
    let mixed = chord_form(&[infinite(), w]);
    let r = mixed.rank().expect("rank");
    assert_eq!(r.free, 1, "one plane free chord, got {r:?}");
    assert_eq!(r.offplane, 1, "one off-plane class, got {r:?}");
    assert_ne!(r.free, 2, "no component equals the old sum");
    assert_ne!(r.offplane, 2, "no component equals the old sum");
}

/// Chapter 23.1 step 4: a filed torsion chord replays; a chord that is
/// not the holonomy refuses the face; a missing torsion entry refuses
/// the loop.
#[test]
fn filed_torsion_chord_replays_or_refuses() {
    let mut f0 = chord_form(&[order4()]);
    let _ = f0.pivot_loop(
        [0xca; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(f0.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
    let e = as_element_form(&f0, [1, 0, 0, 0]).expect("the filed form crystallizes");
    assert!(
        e.torsion().is_empty(),
        "v2 does not fill the element torsion section"
    );
    assert!(
        carries_torsion(&e),
        "the form carries the filed cycle"
    );
    let keyed = crate::transducer::sentence::from_element_form(&e).expect("v2 tape");
    let (class, chord) = keyed
        .torsion
        .scoped()
        .next()
        .map(|(k, v)| (k, v.chord))
        .expect("filed class");
    assert_eq!(
        torsion_loop_rotor(&keyed, &class).and_then(|r| r.class_key()),
        Some(crate::transducer::address::Address::from(class)),
        "the chord face is the keyed representative"
    );
    assert!(
        keyed.faces.get(chord as usize).is_some(),
        "the carrier stores a live chord face"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "a filed torsion chord replays: the chord carries the holonomy"
    );
    let mut f = crate::transducer::sentence::from_element_form(&e).expect("v2 tape");
    let mag = e.magnitude();
    let Some(ci) = f
        .faces
        .iter()
        .position(|face| f.face_of(face.i, face.j).as_ref() != Some(face))
    else {
        panic!("the filed form has a torsion chord");
    };
    f.faces[ci].word = order2();
    let bad = as_element_form(&f, mag).expect("tampered chord");
    assert_eq!(
        crate::court::verdict::blind_replay(&bad),
        Some(crate::court::verdict::REFUSE_LOOP),
        "a chord whose holonomy is not the filed class refuses the loop"
    );
    let mut gone = crate::transducer::sentence::from_element_form(&e).expect("v2 tape");
    gone.torsion = Keyed::new();
    let missing = as_element_form(&gone, mag).expect("torsion stripped");
    assert_eq!(
        crate::court::verdict::blind_replay(&missing),
        Some(crate::court::verdict::REFUSE_LOOP),
        "a filed chord without its torsion entry refuses the loop"
    );
}

#[test]
fn horizon_seeded_infinite_is_not_demanded() {
    let mut f = chord_form(&[infinite()]);
    let r = infinite().rotor();
    let mut horizon = crate::network::nerve::Keyed::new();
    horizon.insert(r.class_key().expect("infinite class"), (r, Order::Infinite, Standing::Held));
    let snaps = f.pivot_loop(
        [0x5e; 32],
        &horizon,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
        "a held infinite class is not spoken again: {snaps:?}"
    );
}

#[test]
fn offplane_word_refuses_to_act_on_a_translation() {
    let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 2)).unwrap(), false)]);
    assert!(w.plane_block().is_none(), "a (0,2) quarter leaves the sensor plane");
    let t = (q1(), q0());
    assert!(
        apply_plane_block(&w, &t).is_none(),
        "an off-plane word does not act as the identity on a translation"
    );
    let tr = Transport {
        word: w.clone(),
        carriage: element::identity_versor(),
        gauge: Vec::new(),
    };
    assert!(
        tr.inverse().is_some(),
        "an off-plane holonomy is a rotor like any other: inverse is the carriage inverse"
    );
    assert!(
        tr.refract(&Form::default()).is_none(),
        "refraction refuses a word that moves the sensor plane"
    );
}

/// A1: a (0,2) quarter to the fourth power is projectively identity and
/// closed to coheres, pivot_loop, is_cauterized, and the court. The
/// sensor-plane abelianisation does not see it.
#[test]
fn offplane_quarter_to_the_fourth_is_closed_to_all_four_readers() {
    let q = alphabet::LetterId::quarter((0, 2)).expect("quarter");
    let w = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    assert!(w.is_identity(), "Φ⁴ is projectively 1");
    let mut f = Form::default();
    let empty = |addr: u8| Vertex::word_on(
        form_addr(addr),
        alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q0()))),
        Vec::new(),
        None,
    );
    f.vertices.push(empty(1));
    f.vertices.push(empty(2));
    f.faces.push(f.face_of(0, 1).expect("star 1"));
    f.faces.push(f.face_of(0, 2).expect("star 2"));
    f.faces.push(f.face_carrying(1, 2, w).expect("a chord between two bodies"));
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    f.reindex();
    assert!(f.coheres(), "coheres reads projective identity");
    assert!(
        f.is_cauterized(),
        "is_cauterized reads projective identity"
    );
    assert!(
        f.loops.iter().all(|lp| lp.holonomy.is_identity()),
        "the chord holonomy is A1 identity"
    );
    let snaps = f.pivot_loop(
        [0xa1; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !snaps.iter().any(|s| matches!(
            s,
            Snap::Demand { .. } | Snap::TorsionOpen { .. } | Snap::TorsionFiled { .. }
        )),
        "pivot_loop does not demand a projective identity: {snaps:?}"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("the identity chord crystallizes");
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "the court reads projective identity"
    );
}

#[test]
fn rotor_class_is_read_before_the_plane_form() {
    let w = order4();
    let rk = w.rotor().class_key().expect("quarter is a versor");
    assert_eq!(
        w.class_key(),
        Some(rk.clone()),
        "Word::class_key is the rotor conjugacy, not the plane form"
    );
    let cv = ClassVector::of_word(&w).expect("quarter");
    let kb = cv.class_key();
    assert!(
        kb.as_bytes().starts_with(rk.as_bytes()),
        "a prefix of the class vector is the rotor's conjugacy stratum"
    );
    assert!(cv.occupies(Axis::Unit(0)));
    assert!(!cv.occupies(Axis::OffPlane), "a plane word is not an off-plane fallback");
    let other = order2();
    assert!(
        !w.same_class(&other),
        "distinct rotor classes are distinct even when both have a plane form"
    );
}

/// A WORD OFF THE SENSOR PLANE OCCUPIES ITS OWN PLANE, and `OffPlane` is
/// what is left when no pair plane is turned at all.
///
/// `Word::form` spelled the sensor block alone, so a quarter on `(0, 2)`
/// had no form and fell to `OffPlane` — the one coordinate that survives a
/// missing plane block. It turns `(0, 2)`, that is a coordinate the
/// carrier has always had a seat for, and it is occupied now.
///
/// `OffPlane` narrows with it: it was "the sensor plane is not preserved"
/// and is now "no pair plane is turned", which is a rotor with no
/// invariant plane at all rather than one whose plane is simply not the
/// sensor's.
#[test]
fn a_word_off_the_sensor_plane_occupies_its_own_plane() {
    let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 2)).unwrap(), false)]);
    assert!(w.plane_block().is_none(), "it moves the sensor plane, as it always did");
    let f = w.form().expect("and it turns (0, 2), which the carrier has a seat for");
    assert!(!f.is_identity(), "a quarter turn is not the identity of the plane group");
    let cv = ClassVector::of_word(&w).expect("a quarter is a versor");
    assert!(
        cv.occupies(Axis::Unit(1)),
        "the (0, 2) quarter occupies the unit axis of plane one, got {:?}",
        cv.coordinates()
    );
    assert!(
        !cv.occupies(Axis::OffPlane),
        "and it is not off-plane: it has a plane, and it is not the sensor's"
    );
    assert!(
        !cv.occupies(Axis::Orientation(0)) && !cv.occupies(Axis::Unit(0)),
        "the sensor's own slots stay empty, not a fallback identity"
    );
    assert_eq!(w.class_key(), w.rotor().class_key());
    assert!(w.same_form(&w));

    // A ROTOR THAT TURNS NO PAIR PLANE is what `OffPlane` now reads. Two
    // quarters on planes sharing an axis leave no two-subset invariant.
    let mixed = Word::from_letters(&[
        (alphabet::LetterId::quarter((0, 1)).unwrap(), false),
        (alphabet::LetterId::quarter((0, 2)).unwrap(), false),
    ]);
    assert!(mixed.form().is_none(), "no pair plane is turned by it");
    let cv = ClassVector::of_word(&mixed).expect("still a versor, so still a class");
    assert!(
        cv.occupies(Axis::OffPlane),
        "and THAT is off-plane, got {:?}",
        cv.coordinates()
    );
}

#[test]
fn orientation_is_the_third_axis_half_turn() {
    let s = alphabet::PlaneForm::sheet_word();
    assert_eq!(s.len(), 2, "the 1.2 relator is two quarter turns, S = q²");
    let f = s.form().expect("the half-turn keeps the sensor plane");
    assert!(f.orientation, "det/ρ² = −1 is the orientation character");
    let cv = ClassVector::of_word(&s).expect("half-turn");
    assert_eq!(
        cv.coordinates(),
        vec![Axis::Orientation(0)],
        "orientation occupies Orientation(0), never a stored flag, got {:?}",
        cv.coordinates()
    );
    let q = order4();
    let conj = s.compose(&q).compose(&s).compose(&q.inverse());
    assert!(
        !conj.form().expect("plane").is_identity(),
        "S R S⁻¹ = R⁻¹, not the commuting identity"
    );
}

#[test]
fn a_loop_residue_is_the_translator_of_its_holonomy_versor() {
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        set_face_place(fa, |p| p.gauge.clear());
    }
    let chord = f.loops[0].chord as usize;
    set_face_place(&mut f.faces[chord], |p| {
        p.carriage = element::carriage(&q1(), &(q1(), q0()));
    });
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    let kappa = f.class_vector(0).expect("loop");
    assert!(kappa.occupies(Axis::Placement));
    let t = element::translator_of(&kappa.carriage).expect("translator");
    assert!(
        !element::same_versor(&t, &element::identity_versor()),
        "the residue is the translator, not a pair"
    );
    assert!(!kappa.occupies(Axis::Dilation));
}

#[test]
fn a_non_versor_holonomy_is_refused_not_zero_class() {
    let w = Word::formed(alphabet::Rotor::zero());
    assert!(w.class_key().is_none(), "a non-versor is never a class");
    assert!(ClassVector::of_word(&w).is_none());
    assert!(ClassVector::read(&w, element::identity_versor(), CycleType::Identity).is_none());
    let rk = alphabet::Rotor::zero().key();
    assert_eq!(rk.tag(), Some(crate::transducer::address::TAG_ROTOR));
    assert_ne!(w.class_key(), Some(rk), "Word::class_key does not fall back to TAG_ROTOR");
}
