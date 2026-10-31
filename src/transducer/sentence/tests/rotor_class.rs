// 16 — THE ROTOR and ITS CLASS: what order a word has, what is filed as
// torsion, and what is refused rather than read as the zero class.

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Order, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

/// 12.6, 16.16 step 3, 3.7 (I6): an orbit whose class has a free part and
/// which no held word discharges OPENS A CHART AXIS seeded by its own
/// holonomy — the residual measured how narrow the cover was, and the
/// cover grew an axis along which it is a section. The orbit then reads
/// zero, the rank moves one unit from `free` into `axes`, the form coheres
/// on the refined cover, and it is never filed as torsion (T3). The
/// certificate replays on the court, which re-derives the seed from the
/// orbit; a tape whose axis was struck out is refused on that orbit.
#[test]
fn pivot_orbit_opens_axis_for_undischarged_free_class() {
    // AT1: measured torsion rides the overlap's EDGE — a pivot factor
    // written without a certificate is refused at replay (REFUSE_PIVOT_INEXACT).
    let mut f = edge_chord_form(&[infinite()]);
    let before = f.rank().expect("rank");
    assert_eq!((before.free, before.axes), (1, 0), "one free orbit, no axis yet");
    let snaps = f.pivot_orbit(
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
        panic!("the opening axis discharges the orbit once: {snaps:?}");
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
    assert!(f.coheres(), "the orbit is the transition on the opened axis");
    assert!(
        f.class_vector(0).is_some_and(|k| k.is_zero()),
        "and it reads as the zero vector"
    );

    // The court replays the opened axis from the tape and the orbit.
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("crystallizes");
    let back = crate::transducer::sentence::from_element_form(&e).expect("tape");
    assert_eq!(back.axes.len(), 1, "the axis rides the tape");
    assert_eq!(
        crate::court::verdict::blind_replay(&e, &mut super::common::budget()),
        None,
        "the court re-derives the seed from the orbit and the certificate re-reads"
    );
    let mut struck = back.clone();
    struck.axes = Keyed::new();
    let bad = as_element_form(&struck, [1, 0, 0, 0]).expect("tape without its axis");
    assert_eq!(
        crate::court::verdict::blind_replay(&bad, &mut super::common::budget()),
        Some(crate::court::verdict::REFUSE_ORBIT),
        "without the axis the orbit is a free class again, and it does not replay"
    );
}

/// Torsion is filed only when it is witnessed at its period; a claimed
/// period the generator does not have is refused by name.
#[test]
fn record_cycle_refuses_unwitnessed_period() {
    let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
    let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
    let mut t = element::from_canonical(&crate::transducer::germinate::germinate(&a, &b).unwrap());
    t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
    assert_eq!(t.torse(), 1, "the hue cycle is one torsion generator");
    let ws = crate::transducer::torsion_witness::witness_stored(&t);
    assert!(
        ws.iter().any(|(ell, w)| *ell == 3
            && matches!(w, crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed { period: 3, .. })),
        "period 3 is witnessed closed: {ws:?}"
    );
    // AN UNCLOSED CIRCUIT IS NOT BELIEVED — and no longer by the witness
    // after the fact: a period the generator does not have is a claim, and
    // the record refuses it by name where it is filed (PD1).
    let before = t.torsion().len();
    let claimed = crate::transducer::element::TorsionCycle {
        generator: crate::transducer::element::Generator::Hue(vec![(1, 9)]),
        period: 8,
        fibre: crate::transducer::element::FIBRE_SPATIAL,
        remainder: None,
    };
    assert_eq!(t.record_cycle(claimed), crate::transducer::element::Filed::Misspoken, "refused by name");
    assert_eq!(t.torsion().len(), before, "and nothing was filed to walk");
}

/// The rank's off-plane count is its own lane, counted once per class and
/// never folded into the free rank.
#[test]
fn rank_counts_offplane_as_own_lane() {
    // A ROTOR WITH NO INVARIANT PAIR PLANE. It was a `(0, 2)` 90° turn,
    // which was off-plane only because the speller could not see its
    // plane; it turns plane one and reads as torsion now. Two 90° turns on
    // planes sharing an axis leave no two-subset invariant, and that is
    // what the off-plane lane counts.
    let w = Word::from_letters(&[
        (alphabet::LetterId::turn_90((0, 1)).unwrap(), false),
        (alphabet::LetterId::turn_90((0, 2)).unwrap(), false),
    ]);
    assert!(w.plane_block().is_none(), "it moves the sensor plane");
    assert!(w.form().is_none(), "and turns no pair plane at all");
    // THE OFF-PLANE ROTOR RIDES THE EDGE, not the face word (54.4, as
    // ruled 2026-09-20). This stood on `chord_form`, which puts the rotor
    // in the face's WORD — and a word with no sensor action places no
    // member pair, which `edge_chord_form`'s own doc has said since it was
    // written: "an orbit with no member path has no class to read".
    //
    // It passed anyway because the cycle basis was built on the VERTEX
    // graph, where a face joins its two vertices whether or not it relates
    // any bodies, and `orbit_holonomy_on` then fell back to the word. Built
    // on the MEMBER complex the cycle does not exist, and that is correct:
    // no body can be carried across a face that places none.
    //
    // `edge_chord_form` is the carrier the codebase already names for this
    // — an identity face word, which DOES place its pair, with the
    // off-plane rotor on 50.3's edge transport — so the orbit is a cycle of
    // the member complex and its composite is off-plane where it belongs.
    let two = edge_chord_form(&[w.clone(), w.clone()]);
    let r = two.rank().expect("rank");
    assert_eq!(r.offplane, 1, "two chords of one off-plane class count once, got {r:?}");
    assert_eq!(r.free, 0, "plane free rank is empty, got {r:?}");
    assert_ne!(r.offplane, 2, "the old per-chord count is not a coordinate");
    assert_ne!(r.free, 2, "the old folded sum is not free");
    let mixed = edge_chord_form(&[infinite(), w]);
    let r = mixed.rank().expect("rank");
    assert_eq!(r.free, 1, "one plane free chord, got {r:?}");
    assert_eq!(r.offplane, 1, "one off-plane class, got {r:?}");
    assert_ne!(r.free, 2, "no component equals the old sum");
    assert_ne!(r.offplane, 2, "no component equals the old sum");
}

/// Chapter 23.1 step 4: a filed torsion chord replays; a chord that is
/// not the holonomy refuses the face; a missing torsion entry refuses
/// the orbit.
#[test]
fn blind_replay_replays_filed_torsion_chord() {
    // AT1: the measured torsion rides the chord's EDGE; an uncertified
    // pivot factor is refused at replay (REFUSE_PIVOT_INEXACT).
    let mut f0 = edge_chord_form(&[word_turn_90()]);
    let _ = f0.pivot_orbit(
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
    let keyed = crate::transducer::sentence::from_element_form(&e).expect("the address decodes");
    // THE CLASS AND THE CHORD ARE DERIVED, NOT FILED (54.5). This read
    // them off `keyed.torsion`, and a form recovered from its address files
    // nothing — the filing rides `Element::filing`, and the class and
    // period are theorems of the complex (`orbit_classes`).
    let (class, chord) = keyed
        .orbits
        .iter()
        .find_map(|lp| match keyed.orbit_classes(lp).as_slice() {
            // The least class the orbit derives; `orbit_classes` walks a
            // `Keyed`, so the order is the member paths' own (rule 15).
            [row, ..] => Some((row.class.clone(), lp.chord.clone())),
            [] => None,
        })
        .expect("the complex derives its own class");
    assert_eq!(
        torsion_orbit_rotor(&keyed, &class).and_then(|r| r.holonomy_key()),
        Some(class),
        "the chord face is the keyed representative"
    );
    assert!(
        keyed.faces.holds(&chord),
        "the carrier stores a live chord face"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&e, &mut super::common::budget()),
        None,
        "a filed torsion chord replays: the chord carries the holonomy"
    );
    let mut f = crate::transducer::sentence::from_element_form(&e).expect("v2 tape");
    let mag = e.magnitude();
    // AT1: the torsion chord is the overlap whose EDGE carries the measured
    // 90° turn, so the forger moves the edge's word.
    let Some(ci) = f.faces.entries().find(|(_, face)| face.has_edge()).map(|(o, _)| o) else {
        panic!("the filed form has a torsion chord");
    };
    if let Some(e) = f.faces.at_mut(&ci).and_then(|face| face.place.edge.as_mut()) {
        e.word = word_turn_180();
    }
    let bad = as_element_form(&f, mag).expect("tampered chord");
    // A TAMPERED CHORD NAMES A DIFFERENT WORD (54.5, as ruled 2026-09-20).
    // This expected `REFUSE_ORBIT` — "a chord whose holonomy is not the FILED
    // class refuses the orbit" — which prices geometry against an annotation.
    // Coherence asks the complex now, so a chord whose new holonomy is still
    // a compact resonance coheres, and the court says so honestly. What the
    // forger cannot do is keep the name: a face's word is in the key (A21).
    assert_ne!(
        bad.address(),
        e.address(),
        "a face's word is in the key, so moving it names another word"
    );
    // THE FORGERY IS NOT UNGUARDED, IT IS UNCONSTRUCTIBLE (54.5, as ruled
    // 2026-09-20) — the same reading `replay_form_replays_filed_torsion_without_taped_forest` already carries
    // one carrier up.
    //
    // This stripped `Form.torsion` and expected `REFUSE_ORBIT`: a filed
    // chord without its entry refuses. Two things have to be true for that
    // attack to exist, and neither is. A form recovered from its ADDRESS
    // files nothing to begin with, so there is no entry to remove; and
    // coherence asks `orbit_classes` rather than the filing, so removing one
    // would change no answer if there were.
    //
    // What a forger can still move is a FACE, and `face_rederives` reads
    // those — asserted above, where moving a chord's word names a different
    // word. There is nothing left here to claim a filing with.
    let mut gone = crate::transducer::sentence::from_element_form(&e).expect("the address decodes");
    assert!(gone.torsion.is_empty(), "a form off its address files nothing");
    gone.torsion = Keyed::new();
    let missing = as_element_form(&gone, mag).expect("torsion stripped");
    assert_eq!(
        missing.address(),
        e.address(),
        "stripping a filing changes no byte of the word: it was never in the key"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&missing, &mut super::common::budget()),
        None,
        "and the geometry replays as it did, because that is what replay reads"
    );
}

/// An infinite class already held on the horizon is not demanded again.
#[test]
fn pivot_orbit_skips_demand_for_horizon_class() {
    let mut f = chord_form(&[infinite()]);
    let r = infinite().rotor();
    let mut horizon = crate::network::nerve::Keyed::new();
    horizon.insert(r.class_key().expect("infinite class"), (r, Order::Infinite, Standing::Held));
    let snaps = f.pivot_orbit(
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

/// An off-plane word refuses to act on a translation, and refraction
/// refuses it.
#[test]
fn apply_plane_block_refuses_offplane_word() {
    let w = Word::from_letters(&[(alphabet::LetterId::turn_90((0, 2)).unwrap(), false)]);
    assert!(w.plane_block().is_none(), "a (0,2) 90° turn leaves the sensor plane");
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
        element::same_versor(&tr.reverse().carriage, &tr.carriage.reverse()),
        "an off-plane holonomy is a rotor like any other: its reverse is the carriage's reverse"
    );
    assert!(
        tr.refract(&Form::default()).is_none(),
        "refraction refuses a word that moves the sensor plane"
    );
}

/// A1: a (0,2) 90° turn to the fourth power is projectively identity and
/// closed to coheres, pivot_orbit, is_cauterized, and the court. The
/// sensor-plane abelianisation does not see it.
#[test]
fn offplane_turn_90_fourth_power_closes_for_every_reader() {
    let q = alphabet::LetterId::turn_90((0, 2)).expect("90° turn");
    let w = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
    assert!(w.is_identity(), "Φ⁴ is projectively 1");
    let mut f = Form::default();
    // Each empty vertex stands on its own chart: two with one rotor, one
    // chart, one carriage and one gauge would be ONE vertex (54.2, §1.5).
    let empty = |addr: u8| Vertex::word_on(
        alphabet::Derivation::spelled(Word::identity()),
        stalk::Invariant::counts(&[u64::from(addr)]),
        Box::new(element::carriage(&q1(), &(q0(), q0()))),
        Vec::new(),
        [FRAME],
        None,
    );
    // AT1: the two are one vertex of the identity's class with a member on
    // each chart (Q1), each starred from the Frame.
    let a = stand_with_star(&mut f, &empty(1));
    let b = stand_with_star(&mut f, &empty(2));
    // The measured word rides the chord's EDGE (an uncertified factor is
    // refused at replay).
    edge_chord(&mut f, &a, &b, Transport::from_word(w));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    assert!(f.coheres(), "coheres reads projective identity");
    assert!(
        f.is_cauterized(),
        "is_cauterized reads projective identity"
    );
    assert!(
        f.orbits
            .iter()
            .all(|lp| orbit_word_on(&f.faces, &f.vertices, lp).is_some_and(|h| h.is_identity())),
        "the chord holonomy is A1 identity"
    );
    let snaps = f.pivot_orbit(
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
        "pivot_orbit does not demand a projective identity: {snaps:?}"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("the identity chord crystallizes");
    assert_eq!(
        crate::court::verdict::blind_replay(&e, &mut super::common::budget()),
        None,
        "the court reads projective identity"
    );
}

/// The rotor class is read before the plane form: a word's class key is its
/// rotor conjugacy and the class vector's leading stratum.
#[test]
fn class_key_reads_rotor_conjugacy_first() {
    let w = word_turn_90();
    let rk = w.rotor().class_key().expect("a 90° turn is a versor");
    assert_eq!(
        w.class_key(),
        Some(rk.clone()),
        "Word::class_key is the rotor conjugacy, not the plane form"
    );
    let cv = ClassVector::of_word(&w).expect("90° turn");
    let kb = cv.class_key();
    assert!(
        kb.as_bytes().starts_with(rk.as_bytes()),
        "a prefix of the class vector is the rotor's conjugacy stratum"
    );
    assert!(cv.occupies(Axis::Unit(0)));
    assert!(!cv.occupies(Axis::OffPlane), "a plane word is not an off-plane fallback");
    let other = word_turn_180();
    assert!(
        !w.same_class(&other),
        "distinct rotor classes are distinct even when both have a plane form"
    );
}

/// A WORD OFF THE SENSOR PLANE OCCUPIES ITS OWN PLANE, and `OffPlane` is
/// what is left when no pair plane is turned at all.
///
/// `Word::form` spelled the sensor block alone, so a 90° turn on `(0, 2)`
/// had no form and fell to `OffPlane` — the one coordinate that survives a
/// missing plane block. It turns `(0, 2)`, that is a coordinate the
/// carrier has always had a seat for, and it is occupied now.
///
/// `OffPlane` narrows with it: it was "the sensor plane is not preserved"
/// and is now "no pair plane is turned", which is a rotor with no
/// invariant plane at all rather than one whose plane is simply not the
/// sensor's.
#[test]
fn of_word_occupies_own_plane_off_sensor() {
    let w = Word::from_letters(&[(alphabet::LetterId::turn_90((0, 2)).unwrap(), false)]);
    assert!(w.plane_block().is_none(), "it moves the sensor plane, as it always did");
    let f = w.form().expect("and it turns (0, 2), which the carrier has a seat for");
    assert!(!f.is_identity(), "a 90° turn is not the identity of the plane group");
    let cv = ClassVector::of_word(&w).expect("a 90° turn is a versor");
    assert!(
        cv.occupies(Axis::Unit(1)),
        "the (0, 2) 90° turn occupies the unit axis of plane one, got {:?}",
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
    // 90° turns on planes sharing an axis leave no two-subset invariant.
    let mixed = Word::from_letters(&[
        (alphabet::LetterId::turn_90((0, 1)).unwrap(), false),
        (alphabet::LetterId::turn_90((0, 2)).unwrap(), false),
    ]);
    assert!(mixed.form().is_none(), "no pair plane is turned by it");
    let cv = ClassVector::of_word(&mixed).expect("still a versor, so still a class");
    assert!(
        cv.occupies(Axis::OffPlane),
        "and THAT is off-plane, got {:?}",
        cv.coordinates()
    );
}

/// Orientation is the axis of the 180° turn: the sheet word occupies
/// Orientation(0), and conjugating a turn by it inverts the turn.
#[test]
fn of_word_occupies_orientation_for_turn_180() {
    let s = alphabet::PlaneForm::sheet_word();
    assert_eq!(s.len(), 2, "the 1.2 relator is two 90° turns, S = q²");
    let f = s.form().expect("the 180°-turn keeps the sensor plane");
    assert!(f.orientation, "det/ρ² = −1 is the orientation character");
    let cv = ClassVector::of_word(&s).expect("180°-turn");
    assert_eq!(
        cv.coordinates(),
        vec![Axis::Orientation(0)],
        "orientation occupies Orientation(0), never a stored flag, got {:?}",
        cv.coordinates()
    );
    let q = word_turn_90();
    let conj = s.compose(&q).compose(&s).compose(&q.inverse());
    assert!(
        !conj.form().expect("plane").is_identity(),
        "S R S⁻¹ = R⁻¹, not the commuting identity"
    );
}

/// An orbit's residue is the translator of its holonomy versor.
#[test]
fn class_vector_reads_residue_as_translator() {
    let mut f = chord_form(&[Word::identity()]);
    let overlaps: Vec<_> = f.faces.entries().map(|(o, _)| o).collect();
    for o in &overlaps {
        if let Some(fa) = f.faces.at_mut(o) {
            set_face_place(fa, |p| p.gauge.clear());
        }
    }
    let chord = f.orbits[0].chord.clone();
    set_face_place(f.faces.at_mut(&chord).expect("the chord's face"), |p| {
        p.carriage = element::carriage(&q1(), &(q1(), q0()));
    });
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let kappa = f.class_vector(0).expect("orbit");
    assert!(kappa.occupies(Axis::Placement));
    let t = element::translator_of(&kappa.carriage).expect("translator");
    assert!(
        !element::same_versor(&t, &element::identity_versor()),
        "the residue is the translator, not a pair"
    );
    assert!(!kappa.occupies(Axis::Dilation));
}

/// A non-versor holonomy is refused, never read as the zero class.
#[test]
fn class_key_refuses_non_versor() {
    let w = Word::formed(alphabet::Rotor::zero());
    assert!(w.class_key().is_none(), "a non-versor is never a class");
    assert!(ClassVector::of_word(&w).is_none());
    assert!(ClassVector::read(&w, element::identity_versor(), CycleType::Identity).is_none());
    let rk = alphabet::Rotor::zero().key();
    assert_eq!(rk.tag(), Some(crate::transducer::address::TAG_ROTOR));
    assert_ne!(w.class_key(), Some(rk), "Word::class_key does not fall back to TAG_ROTOR");
}
