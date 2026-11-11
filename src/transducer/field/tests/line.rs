// MU-CLIENT §4.13, §7.1 Phase 3 — A HELD THOUGHT IS ITS ORBIT: memory released
// by the orbit's physics, every claim run through the mind's own field — a
// perception released where its motion is let go, a grounded thought winding
// on past its return, and escrow conserved through a merge.

use crate::chronometry::Cite;
use crate::transducer::field::Contemplation;
use crate::transducer::family::{perceive_family, scene_form_on, SectionSample, NoCrystal};

fn digits(rows: &[&str]) -> SectionSample {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// Perceive `input → output` in `family`, held on `escrow`, as a voice holds
/// it, in the field thinking on this thread.
fn perceive(c: &mut Contemplation, input: &[&str], output: &[&str], family: &[u8], escrow: [u64; 4]) -> Vec<u8> {
    let (a, b) = (digits(input), digits(output));
    let frames = perceive_family(&[(a.clone(), b.clone())]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let f = scene_form_on(&NoCrystal, &mut budget, &a, &b, &frames).expect("the scene reads");
    c.hold_perceived(f, &frames, &Vec::new(), &[], escrow, family, &mut budget).expect("held").as_bytes().to_vec()
}

/// THE VENT, as a winding takes it: the field beats, what it let go is let
/// go, and the thoughts newly standing thread lines.
fn vent(c: &mut Contemplation) {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let Ok((beat, released)) = crate::transducer::mind::with_field(|f| (f.beat(), f.take_released())) else {
        panic!("a mind is thinking")
    };
    c.release(&released);
    c.thread_new(&mut budget, beat);
}

/// THE FIELD MOVED by a motion of no thought: a 90° turn on `e₁e₃`, free.
fn move_field() {
    use crate::transducer::cga::{Mv, E1, E3};
    let one = num_rational::BigRational::from_integer(1.into());
    let turn = Mv::scalar(one.clone()).add(&Mv::blade(E1 | E3, one));
    let line = crate::transducer::family::pairing::FieldLine { carriage: turn, gauge: Vec::new(), mass: 1, free: false, line: crate::transducer::cech::Thread::Free };
    let _ = crate::transducer::family::pairing::mark_flux(&[], std::slice::from_ref(&line), &mut crate::physics::Physics::default().stroke_n(1 << 16));
}

/// Two Ls three columns apart, each carried one column along: one scene of a
/// slide.
const SLIDE_IN: [&str; 3] = ["10001000", "10001000", "11001100"];
const SLIDE_OUT: [&str; 3] = ["01000100", "01000100", "01100110"];
const SLIDE_ON: [&str; 3] = ["00100010", "00100010", "00110011"];

/// A PERCEPTION OF A SLIDE IS RELEASED WHERE ITS MOTION IS LET GO, and its
/// scene's demand stands. The slide is null flux: its tube stands through the
/// beat of the vent it was perceived at, the perception threaded on it. The
/// second hour, perceived while the first stands, meets it; its slide lands
/// on the same ray and closes all that stood open there, and the first is
/// let go at the vent as that closure. The second finds nothing open to
/// thread: it stands until the field next moves, to be met — a vent with
/// nothing moved lets it go nowhere — and leaves at the beat of that motion.
/// Both gone, the thoughts' remainders are at the cavity, the escrow returned exactly,
/// the scene counted for the stroke's returns, while the escaping motion
/// between the hours stands on the docket under that scene.
#[test]
fn release_drops_slide_perception_where_motion_lets_go() {
    crate::transducer::raster::stand_on(0);
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        let first = perceive(&mut c, &SLIDE_IN, &SLIDE_OUT, b"the scene", [1, 0, 0, 0]);
        vent(&mut c);
        assert!(c.wells.get(&first).is_some(), "the first stands through the beat it was perceived at");
        let second = perceive(&mut c, &SLIDE_OUT, &SLIDE_ON, b"the scene", [1, 0, 0, 0]);
        let cavity = c.cavity_len();
        vent(&mut c);
        assert!(c.wells.get(&first).is_none(), "the first let go as its ray closed");
        vent(&mut c);
        assert!(c.wells.get(&second).is_some(), "nothing moved: no time passed, and it stands");
        move_field();
        vent(&mut c);
        assert!(c.wells.get(&second).is_none(), "the second, held by nothing, left at the field's next beat");
        assert!(c.cavity_len() > cavity, "the remainders at the cavity");
        assert_eq!(c.take_returned_escrow(), [2, 0, 0, 0], "the escrow returned, exactly");
        let scenes = c.take_released_scenes();
        assert!(!scenes.is_empty() && scenes.iter().all(|s| s == b"the scene"), "the scene counted: {scenes:?}");
        let demanded = c.docket_cites().into_iter().any(|cite| matches!(cite, Cite::Demand { residue: Some(_), .. }));
        assert!(demanded, "the escaping motion stands as a demand");
    });
}

/// ONE ORBIT: the L turned 90° at a time about the plane's point (2, 2), each
/// carried one column in its output.
const HOURS: [([&str; 5], [&str; 5]); 4] = [
    (["010000", "010000", "011000", "000000", "000000"], ["001000", "001000", "001100", "000000", "000000"]),
    (["000000", "001110", "001000", "000000", "000000"], ["000000", "000111", "000100", "000000", "000000"]),
    (["000000", "000000", "001100", "000100", "000100"], ["000000", "000000", "000110", "000010", "000010"]),
    (["000000", "000000", "001000", "111000", "000000"], ["000000", "000000", "000100", "011100", "000000"]),
];

/// A GROUNDED THOUGHT WINDS ON PAST ITS RETURN (Q1, Q2 as ruled). The L's
/// four hours form one thought grounded on the 90° turn; its generator is
/// admitted on its own line once per hour, so at the fourth hour the field's
/// orbit returns with its winding. The thought is not let go there: it
/// stands, its first close made, and the line's return is read as a return.
#[test]
fn thread_new_winds_grounded_thought_past_return() {
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        for (input, output) in &HOURS {
            perceive(&mut c, input, output, b"the orbit", [0; 4]);
            vent(&mut c);
        }
        assert_eq!(c.closed_thoughts, 1, "the fourth hour closed its return: {}", c.cross_scene.line());
        let grounded: Vec<Vec<u8>> = c.wells.scoped().filter(|(_, b)| b.ground.is_some()).map(|(k, _)| k).collect();
        assert_eq!(grounded.len(), 1, "the grounded thought stands past its return");
    });
}

/// ESCROW IS CONSERVED THROUGH A MERGE. The L and the L turned 90°, each held
/// on its own escrow, merge into one thought, which holds exactly what the two
/// held — nothing re-drawn, nothing lost, nothing returned undrawn.
#[test]
fn hold_perceived_conserves_escrow_through_merge() {
    let mut field = crate::transducer::mind::MindField::default();
    field.think(|| {
        let mut c = Contemplation::new();
        perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"], b"the shape", [1, 0, 0, 2]);
        perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"], b"the shape", [0, 3, 0, 1]);
        let held: [u64; 4] = c.wells.scoped().fold([0; 4], |mut sum, (_, b)| {
            for (s, e) in sum.iter_mut().zip(b.escrow) {
                *s += e;
            }
            sum
        });
        let returned = c.take_returned_escrow();
        let total: [u64; 4] = std::array::from_fn(|l| held[l] + returned[l]);
        assert_eq!(total, [1, 3, 0, 3], "every unit held or returned, exactly once: held {held:?}, returned {returned:?}");
    });
}

/// MU-CLIENT §4.13, MU-SENSING §3.7, §3.9: `cech::orbit` orbits attach to the
/// `Well`s whose lines thread them in admission order (`Well::orbits`). A
/// single-line orbit stands attached on its well and refuses
/// `NotRevolution { at: 1 }` until a second well threads the same orbit,
/// forming a closed revolution with readable holonomy, precession, and
/// orthogonal-plane rate shear (`Well::local_shear`). When `Field::beat_under`
/// beats with court pressure (`deficit`) on that orbit's lane, the ringing
/// orbit is dragged into the reservoir and its line is released
/// (`Released::Dissipated`), deforming the well it faces (releasing the
/// thought to the cavity at its chart and returning its escrow), while a well
/// on an unpressed lane keeps its attached local orbit intact.
#[test]
fn court_pressure_deforms_the_well_it_faces() {
    use crate::transducer::cech::orbit::Field;
    use crate::transducer::cech::Thread;
    use crate::transducer::cga::{self, Mv, E1, E2, E3, EP};
    use crate::transducer::field::carrier::{FieldRefusal, RotorPrecession};
    use crate::transducer::field::SectionLink;
    use crate::transducer::rewrite::LANE_SHELL;
    use crate::transducer::spelled::SectionSite;
    use num_rational::BigRational;

    let mut c = Contemplation::new();
    let r1 = vec![SectionLink { site: SectionSite::on_plane([0, 0]), before: 1, after: 2 }];
    let r2 = vec![
        SectionLink { site: SectionSite::on_plane([1, 0]), before: 2, after: 3 },
        SectionLink { site: SectionSite::on_plane([1, 1]), before: 2, after: 3 },
    ];
    let r3 = vec![
        SectionLink { site: SectionSite::on_plane([2, 0]), before: 3, after: 4 },
        SectionLink { site: SectionSite::on_plane([2, 1]), before: 3, after: 4 },
        SectionLink { site: SectionSite::on_plane([2, 2]), before: 3, after: 4 },
    ];
    c.hold(r1.clone());
    c.hold(r2.clone());
    c.hold(r3.clone());
    let k1 = crate::transducer::field::well::remainder_address(&crate::transducer::element::place_chain(&r1).expect("placed")).as_bytes().to_vec();
    let k2 = crate::transducer::field::well::remainder_address(&crate::transducer::element::place_chain(&r2).expect("placed")).as_bytes().to_vec();
    let k3 = crate::transducer::field::well::remainder_address(&crate::transducer::element::place_chain(&r3).expect("placed")).as_bytes().to_vec();
    if let Some(w) = c.wells.get_mut(&k1) {
        w.escrow = [0, 0, 0, 5];
    }
    if let Some(w) = c.wells.get_mut(&k2) {
        w.escrow = [0, 0, 0, 7];
    }

    let mut field = Field::new();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let one = BigRational::from_integer(1.into());
    let two = BigRational::from_integer(2.into());
    let turn_12 = Mv::scalar(two.clone()).add(&Mv::blade(E1 | E2, one.clone()));
    let modes_12 = cga::motion_modes(&turn_12, &mut budget).expect("modes").expect("exact");
    assert_eq!(modes_12.len(), 1);

    // 1. Single-line orbit on well 1: attached to well 1, refuses `NotRevolution { at: 1 }`.
    field.orbits.admit_mode(&modes_12[0], 1, &Thread::Wound(k1.clone())).expect("admitted");
    c.sync_field(&mut field);
    let w1 = c.wells.get(&k1).expect("well 1 stands");
    assert_eq!(w1.local_orbits().len(), 1, "single-line orbit attached to well 1");
    assert_eq!(w1.local_precessions(), Err(FieldRefusal::NotRevolution { at: 1 }));

    // 2. Second line (`k2`) winds onto the same orbit in admission order:
    // the two-well revolution closes and reads `RotorPrecession::Advances`.
    field.orbits.admit_mode(&modes_12[0], 1, &Thread::Wound(k2.clone())).expect("admitted");
    // Well 3 threads a turning on an orthogonal plane (`E3 | EP`) so `local_shear`
    // between well 1 (`E1 | E2`) and well 3 (`E3 | EP`) reads the orthogonal drag.
    let turn_3p = Mv::scalar(two).add(&Mv::blade(E3 | EP, one));
    let modes_3p = cga::motion_modes(&turn_3p, &mut budget).expect("modes").expect("exact");
    field.orbits.admit_mode(&modes_3p[0], 1, &Thread::Wound(k3.clone())).expect("admitted");
    c.sync_field(&mut field);

    let (w1, w2, w3) = (c.wells.get(&k1).expect("w1"), c.wells.get(&k2).expect("w2"), c.wells.get(&k3).expect("w3"));
    let rev = w1.local_orbits()[0].1.revolution().expect("two-line revolution closes");
    assert_eq!((&rev[0].from, &rev[0].to), (&w1.centre(), &w2.centre()), "step 0 runs w1 -> w2 in admission order");
    assert_eq!((&rev[1].from, &rev[1].to), (&w2.centre(), &w1.centre()), "step 1 closes w2 -> w1");
    assert!(matches!(w1.local_precessions().expect("bound")[0].1, RotorPrecession::Advances(_)));
    let shears = w1.local_shear(w3).expect("orthogonal shear");
    assert_eq!(shears.len(), 1, "orthogonal orbit planes exert non-zero rate drag");
    assert!(!shears[0].is_zero());

    // 3. First beat with zero court deficit: ringing orbits stay in the field
    // and transition out of `in_motion`.
    assert!(field.beat_under(&[0; 4]), "field beats on prior admissions");
    c.sync_field(&mut field);
    assert!(c.wells.get(&k1).is_some() && c.wells.get(&k2).is_some(), "ringing wells stand without court pressure");

    // 4. Touch only a spatial boost on a free line so the field moves, then
    // beat under court pressure on `LANE_SHELL`: the court's deficit drags the
    // ringing shell orbits into the reservoir and deforms the wells they face.
    let boost = cga::transvection(&BigRational::from_integer(2.into()), &BigRational::from_integer(1.into()), E1 | cga::EM)
        .expect("boost");
    let boost_modes = cga::motion_modes(&boost, &mut budget).expect("modes").expect("exact");
    field.orbits.admit_mode(&boost_modes[0], 1, &Thread::Free).expect("spatial boost moves field");
    let mut deficit = [0u64; 4];
    deficit[LANE_SHELL] = 1;
    let cavity_before = c.cavity_len();
    assert!(field.beat_under(&deficit), "field beats under court pressure on shell lane");
    c.sync_field(&mut field);

    assert!(c.wells.get(&k1).is_none() && c.wells.get(&k2).is_none() && c.wells.get(&k3).is_none(), "pressed wells released");
    assert!(c.cavity_len() > cavity_before, "deformed wells' remainders retired to the cavity at their charts");
    assert_eq!(c.take_returned_escrow(), [0, 0, 0, 12], "escrow conserved upon court-pressure release");
}
