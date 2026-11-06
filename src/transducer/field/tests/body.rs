// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.171 §2 (a well), MU-SENSING §3.7, ROADMAP track 26 S7 stage 2 — THE
// WELL STANDS ON THE FIELD'S CARRIER: its centre an attractor, its membrane
// its own enclosure, its stage a coordinate, its volume and potential
// readings per lane, and its body stated for a court.

use crate::transducer::field::carrier::*;
use crate::transducer::field::*;
use crate::transducer::sentence::tests::fixtures::link;
use crate::transducer::stalk::Invariant;
use forms::coordinate::Square;
use forms::passage::{Place, Pole};

/// One thought held from a remainder of two links, at K `now`.
fn held_at(now: u64) -> (Contemplation, Vec<u8>) {
    let mut c = Contemplation::new();
    c.set_now(now);
    c.hold(vec![link(0, 0, 1, 2), link(0, 1, 1, 2)]);
    let k = c.wells.scoped().next().map(|(k, _)| k).expect("one well held");
    (c, k)
}

/// A lap through the well's centre and two further charts, each step a
/// turning by `degrees` on `e₊₁e₊₂` of the well's planes.
fn lap_about(centre: &Chart, degrees: i64) -> Orbit {
    let planes = Well::planes().expect("the well's planes");
    let plane = TurningPlane::of(&planes, Square::Positive, 1, 2).expect("e₊₁e₊₂");
    let charts = [centre.clone(), Chart::of_invariant(&Invariant::counts(&[1])), Chart::of_invariant(&Invariant::counts(&[2]))];
    let step = |i: usize| {
        let turning = Turning::of(plane.clone(), crate::core::Q::from_integer(degrees.into())).expect("a turning");
        Transition { from: charts[i].clone(), to: charts[(i + 1) % 3].clone(), motion: Motion::of(vec![turning]).expect("a motion") }
    };
    Orbit::of((0..3).map(step).collect()).expect("the lap closes")
}

/// A WELL STANDS ON ITS ATTRACTOR: its body's one attractor is at the chart
/// its remainder stands on, in its own atlas. Holding no orbit it is flat,
/// its precession refused `Loop`; holding a lap of three 30° steps it
/// precesses by 90° on `e₊₁e₊₂` per lap, read from the orbit, never stored.
#[test]
fn well_stands_on_its_attractor() {
    let (mut c, k) = held_at(0);
    let b = c.wells.get(&k).expect("held");
    let centre = Chart::of_invariant(&crate::transducer::stalk::invariant(&b.remainder));
    let body = b.body().expect("the well stands on the carrier");
    assert_eq!(body.centre(), &centre, "the centre is the remainder's chart");
    let atlas = body.atlas().expect("its own atlas");
    let [attractor] = atlas.attractors() else { panic!("one attractor, the well's") };
    assert_eq!(attractor.centre, centre);
    assert_eq!(b.precession(), Err(FieldRefusal::Loop { charts: vec![centre.clone()] }), "flat: no orbit held");
    let b = c.wells.get_mut(&k).expect("held");
    b.orbit = Some(lap_about(&centre, 30));
    let h = b.precession().expect("an orbit precesses");
    let angle = h.angle().expect("one plane");
    assert_eq!((angle.plane().key().written(), angle.degrees()), ("e_+1+2".to_string(), &crate::core::Q::from_integer(90.into())));
    assert_eq!(b.holonomy(), Ok(h), "the holonomy per lap is the precession read");
}

/// A WELL'S MEMBRANE IS ITS OWN ENCLOSURE: the throat of the last of its
/// planes (the lift's own), standing at the throat. Its inflation stands
/// at the open pole, infinity, only inside it: the same state outside every
/// enclosure is refused `Unenclosed`. A thought holding no section reads at
/// the closed pole: its cover closed, nothing reaching.
#[test]
fn well_membrane_is_its_enclosure() {
    let (c, k) = held_at(0);
    let b = c.wells.get(&k).expect("held");
    let body = b.body().expect("stands");
    let membrane = body.membrane();
    assert_eq!(membrane.plane(), Well::planes().expect("planes").count(), "its own plane");
    assert_eq!(membrane.place(), Ok(Place::Throat));
    let q = |n: usize| crate::core::Q::from_integer(n.into());
    let open = body.inflation(q(0), q(1)).expect("enclosed");
    assert_eq!(open.place, Place::Pole(Pole::Open));
    let state = at_pole(membrane.poles(), Pole::Open, q(1)).expect("a pole state");
    assert_eq!(stand(&state, membrane.poles(), Labels::Standard, None), Err(FieldRefusal::Unenclosed { pole: Pole::Open }));
    let read = b.inflation().expect("a gauge reading");
    assert_eq!(read.place, Place::Pole(Pole::Closed), "{read:?}");
    assert_eq!(read.gauge.hold, q(b.cover.len()));
}

/// A WELL'S VOLUME AND POTENTIAL ARE READINGS PER LANE: the volume the
/// lanes of what it holds, moving when what it holds moves with nothing
/// written; the potential's T lane the receipts since its charts moved, read
/// at the K asked, and no reading where that K is behind the mark.
#[test]
fn well_volume_and_potential_are_readings_per_lane() {
    let (mut c, k) = held_at(3);
    let own = vec![link(0, 0, 1, 2), link(0, 1, 1, 2)];
    let b = c.wells.get(&k).expect("held");
    assert_eq!(b.volume(), Some(element::measure_lanes(&own)));
    let at_three = b.potential(3).expect("read at K 3");
    assert_eq!(at_three[2], 0, "moved at K 3, read at K 3");
    assert_eq!(b.potential(10).map(|p| p[2]), Some(7), "seven receipts later");
    assert_eq!(b.potential(2), None, "a K behind the mark measures no extent");
    let pair = vec![link(4, 4, 0, 5)];
    let b = c.wells.get_mut(&k).expect("held");
    b.remainders.insert(crate::transducer::family::pair_index_key(0), pair.clone());
    let both = crate::core::lanes::lanes_add(element::measure_lanes(&own), element::measure_lanes(&pair));
    assert_eq!(b.volume(), both, "the reading follows what the well holds");
}

/// A WELL'S STAGE IS A COORDINATE, NOT A FLAG: a remainder held alone is a
/// thought on the ring; expanded to rest it is a thought rested; a class
/// arriving puts it back on the ring. Perception stands before thought.
#[test]
fn well_stage_is_a_coordinate_not_a_flag() {
    use crate::transducer::family::NoCrystal;
    let (mut c, k) = held_at(0);
    assert_eq!(c.wells.get(&k).map(|b| b.stage), Some(WellStage::thought()));
    let atlas = crate::transducer::stalk::Stalks::default();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    let b = c.wells.get_mut(&k).expect("held");
    let _ = b.expand_with(&atlas, &|_| None, 0, &NoCrystal, &mut budget, &mut Default::default());
    assert_eq!(b.stage, WellStage { layer: WellLayer::Thought, ring: ExpansionRing::Rested });
    assert_eq!(b.stage.reopened(), WellStage::thought());
    assert!(WellStage::perceived() < WellStage::thought(), "one well at two stages, perception first");
    assert!(WellStage::thought() < WellStage::default(), "on the ring before rested");
}

/// A MIND'S WELL IS STATED AND RECONSTRUCTED BY A COURT: the statement
/// writes the body and no reading; the court derives the precession and the
/// stage from it and admits the mind's claim on a match.
#[test]
fn mind_well_stated_and_reconstructed_by_a_court() {
    let (mut c, k) = held_at(0);
    let b = c.wells.get_mut(&k).expect("held");
    b.orbit = Some(lap_about(&b.centre(), 30));
    let mut stated = Vec::new();
    b.put_statement(&mut stated).expect("stated");
    let claimed = Curvature { centre: b.centre(), holonomy: b.precession() };
    let court = admit_well(&stated, &claimed).expect("the court admits");
    assert_eq!(court.stage(), b.stage, "the stage reconstructed");
    assert_eq!(court.centre(), &b.centre(), "at the well's chart");
}
