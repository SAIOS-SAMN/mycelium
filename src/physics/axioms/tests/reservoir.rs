// MU-CLIENT §4.5 — DISSIPATION AND REFORMATION, THE RESERVOIR'S TWO DOORS:
// on a beat of the field what never returns dissipates into `e_∅` at its own
// coordinates, and a motion landing there re-forms it; per lane, nothing is
// made or lost outside the two doors.

use crate::transducer::cech::orbit::{Field, Released, Thread};
use crate::transducer::cga::{self, Mv, E1, E2, E3};
use crate::transducer::family::pairing::{transport_order, FieldLine, FluxReading};
use crate::transducer::rewrite::{LANE_SHELL, LANE_SPATIAL, LANE_TEMPORAL};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn line(carriage: Mv, mass: u64) -> FieldLine {
    FieldLine { carriage, gauge: Vec::new(), mass, free: false, line: Thread::Free }
}

/// ONE VENT OF A MIND: the lines admitted, then the field's beat.
fn admit(field: &mut Field, lines: &[FieldLine]) -> FluxReading {
    let reading = transport_order(lines, &[], field, &mut crate::physics::Physics::default().stroke_n(1 << 16));
    field.beat();
    reading
}

fn turn(mask: usize) -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(mask, q(1, 1)))
}

/// A BOOST NEVER RETURNS, SO A BEAT DISSIPATES IT; A TURN RINGS. A gain opens
/// 𝒮 by 3, and a turn on another plane opens 𝒽. The field moved, so it beats
/// (MU-CLIENT §7.1 Phase 3): the gain moves into the reservoir whole — 𝒮's
/// open pressure falls by exactly 3, the reservoir holds 3 — while the open
/// turn rings on, through a turn that comes back on a third plane. The
/// gain's inverse lands on its plane: the reservoir re-forms it, the inverse
/// closes it, and the 3 is relieved — converged, nothing opened; the
/// reservoir holds none.
#[test]
fn beat_dissipates_boost_and_inverse_reforms_it() {
    let mut field = Field::new();
    let gain = cga::dilator(&q(2, 1));
    let first = admit(&mut field, &[line(gain.clone(), 3), line(turn(E1 | E3), 1)]);
    assert_eq!((first.opened[LANE_SPATIAL], first.opened[LANE_SHELL]), (3, 1), "the gain and the turn open");
    let back_and_forth = [line(turn(E1 | E2), 1), line(turn(E1 | E2).reverse(), 1)];
    let _ = admit(&mut field, &back_and_forth);
    let (open, _) = field.orbits.totals();
    assert_eq!(open[LANE_SPATIAL], 0, "the beat dissipated the gain's open strain");
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 3, "into the reservoir, whole");
    assert_eq!(open[LANE_SHELL], 1, "the open turn rings on");
    let reformed = admit(&mut field, &[line(cga::dilator(&q(1, 2)), 3)]);
    assert_eq!(reformed.opened, [0; 4], "nothing opened: it closed on what re-formed");
    assert_eq!(reformed.relieved[LANE_SPATIAL], 3, "the re-formed strain relieved");
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 0, "the reservoir holds none");
}

/// ON 𝒯 THE SAME: a slide opens its tube by 2, and the field beats on it: the
/// tube's open flux dissipates into the reservoir at its ray, the tube itself
/// frozen in. Another tube's flux cancels and that tube, at rest, is let go;
/// the reservoir still holds the 2. Flux on the first ray re-forms it and
/// closes on it, relieving the 2.
#[test]
fn beat_dissipates_tube_flux_and_ray_reforms_it() {
    let mut field = Field::new();
    let slide = |x: i64, y: i64| cga::translator_at(&[q(x, 1), q(y, 1), q(0, 1)]);
    let first = admit(&mut field, &[line(slide(1, 0), 2)]);
    assert_eq!(first.opened[LANE_TEMPORAL], 2, "the slide opens its tube");
    let _ = admit(&mut field, &[line(slide(0, 1), 1), line(slide(0, -1), 1)]);
    assert_eq!(field.tubes.reserved(), 2.into(), "the beat dissipated the open flux into the reservoir");
    let reformed = admit(&mut field, &[line(slide(1, 0), 2)]);
    assert_eq!(reformed.opened, [0; 4], "nothing opened");
    assert_eq!(reformed.relieved[LANE_TEMPORAL], 2, "the re-formed flux relieved");
    assert_eq!(field.tubes.reserved(), 0.into(), "the reservoir holds none");
}

fn wound(carriage: Mv, key: &[u8]) -> FieldLine {
    FieldLine { carriage, gauge: Vec::new(), mass: 1, free: false, line: Thread::Wound(key.to_vec()) }
}

/// MU-CLIENT §7.1 Phase 3 — THE FIELD BEATS WHEREVER IT MOVED, with no return
/// anywhere: a gain wound on a line is admitted alone and stands through the
/// beat of its own vent, its line held. A turn on another plane moves the
/// field again, and the beat after dissipates the gain into the reservoir
/// and lets its line go, by that cause.
#[test]
fn beat_dissipates_wound_boost_one_beat_later() {
    let mut field = Field::new();
    let _ = admit(&mut field, &[wound(cga::dilator(&q(2, 1)), b"the gain")]);
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 0, "it stands through the beat it was admitted in");
    assert!(field.take_released().is_empty(), "its line held");
    let _ = admit(&mut field, &[line(turn(E1 | E3), 1)]);
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 1, "dissipated whole at the next, nothing having returned");
    let released = field.take_released();
    assert_eq!(released, vec![(b"the gain".to_vec(), Released::Dissipated)], "its line let go, dissipated");
}

/// 78.43 — AN ORBIT CARRIED ITS WHOLE PERIOD WITHOUT RETURNING SEALS. On one
/// plane three 90° turns and a 180° turn: the motions that moved it have
/// orders 4, 4, 4 and 2, so its period is 4; after four transitions it
/// stands at 90°, not home. Its age complete, it seals: its line let go by
/// that cause, its strain held in the reservoir at its coordinates.
#[test]
fn beat_seals_orbit_carried_its_period_unreturned() {
    let mut field = Field::new();
    let half = Mv::blade(E1 | E2, q(1, 1));
    for step in [turn(E1 | E2), turn(E1 | E2), turn(E1 | E2), half] {
        let _ = admit(&mut field, &[wound(step, b"the ring")]);
    }
    assert_eq!(field.orbits.sealed(), 1, "sealed at its period");
    assert_eq!(field.orbits.len(), 0, "and no longer stands open");
    assert!(field.orbits.reserved()[LANE_SHELL] > 0, "its strain held in the reservoir");
    let released = field.take_released();
    assert_eq!(released, vec![(b"the ring".to_vec(), Released::Sealed)], "its line let go, sealed");
}

/// 78.43, §0.0 HARMONIC — A ROSETTE HAS NO PERIOD AND NEVER SEALS: the
/// (3,4,5) turn closes at no order the algebra admits, so however far it is
/// carried it neither returns nor seals; it rings on, its winding content,
/// its line held.
#[test]
fn beat_never_seals_rosette() {
    let mut field = Field::new();
    let rosette = Mv::scalar(q(2, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
    for _ in 0..8 {
        let _ = admit(&mut field, &[wound(rosette.clone(), b"the rosette")]);
    }
    assert_eq!((field.orbits.sealed(), field.orbits.len()), (0, 1), "it rings on, unsealed");
    assert!(field.take_released().is_empty(), "its line held");
}

/// MU-CLIENT §4.13, 78.168 C2 — THE COURT'S GRAVITY DRAGS AN ESCAPING ORBIT
/// IN. A turn wound on a line rings on through every beat with no deficit
/// standing; once the court's deficit presses its lane (𝒽), the next beat
/// drags it into the reservoir as a boost is, its line let go, dissipated. A
/// lane the deficit does not press beats as before: the turn on another lane
/// is untouched by a deficit on 𝒮 alone.
#[test]
fn beat_under_deficit_drags_ringing_orbit_on_its_lane() {
    let mut field = Field::new();
    let _ = admit(&mut field, &[wound(turn(E1 | E2), b"the thought")]);
    let _ = admit(&mut field, &[line(turn(E1 | E3), 1)]);
    assert!(field.take_released().is_empty(), "with no deficit it rings on");
    let _ = transport_order(&[line(cga::dilator(&q(2, 1)), 1)], &[], &mut field, &mut crate::physics::Physics::default().stroke_n(1 << 16));
    field.beat_under(&[1, 0, 0, 0]);
    assert!(field.take_released().is_empty(), "a deficit on 𝒮 alone does not press 𝒽");
    let _ = transport_order(&[line(cga::dilator(&q(3, 1)), 1)], &[], &mut field, &mut crate::physics::Physics::default().stroke_n(1 << 16));
    field.beat_under(&[0, 0, 0, 1]);
    assert_eq!(field.take_released(), vec![(b"the thought".to_vec(), Released::Dissipated)], "dragged in on 𝒽, its line let go");
    assert!(field.orbits.reserved()[LANE_SHELL] > 0, "its strain held in the reservoir");
}

/// MU-CLIENT §4.13, 78.43 — A RESERVOIR RECORD CONDENSES WHEN THE EON IT
/// ENTERED UNDER CLOSES. A gain dissipates while a 90° turn rings: the
/// record enters under that ringing orbit. However many beats the field
/// takes — slides on fresh rays, touching no orbit — the record stands while
/// the turn rings: no count of beats ages it. The turn's inverse brings it
/// home; the orbit it awaited has left, its eon has closed, and at that beat
/// the record condenses to nothing, counted.
#[test]
fn reserved_record_condenses_when_its_eon_closes() {
    let mut field = Field::new();
    let slide = |x: i64, y: i64| cga::translator_at(&[q(x, 1), q(y, 1), q(0, 1)]);
    let _ = admit(&mut field, &[line(turn(E1 | E3), 1), line(cga::dilator(&q(2, 1)), 1)]);
    let _ = admit(&mut field, &[line(slide(0, 1), 1)]);
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 1, "the gain dissipated, in the reservoir");
    for (x, y) in [(1, 1), (1, 2), (1, 3), (2, 3), (3, 4), (3, 5)] {
        let _ = admit(&mut field, &[line(slide(x, y), 1)]);
        assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 1, "it stands while the orbit it awaits rings");
    }
    let _ = admit(&mut field, &[line(turn(E1 | E3).reverse(), 1)]);
    assert_eq!(field.orbits.reserved()[LANE_SPATIAL], 0, "its eon closed, it condensed");
    assert_eq!(field.condensed[LANE_SPATIAL], 1, "the strain condensed, counted");
}

/// MU-CLIENT §4.13 — A TUBE WITH NOTHING OPEN HOLDS NO LINE. A slide opens
/// its tube; the same slide wound on a line closes on it, nothing left open,
/// and that line is let go as the closure; a third, wound on another line,
/// lands on the tube with nothing open and is let go the same way — never
/// held by a tube that carries no open flux.
#[test]
fn transport_order_lets_go_line_on_closed_tube() {
    let mut field = Field::new();
    let slide = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let _ = transport_order(&[line(slide.clone(), 1)], &[], &mut field, &mut budget);
    let _ = transport_order(&[wound(slide.clone(), b"the closure")], &[], &mut field, &mut budget);
    assert_eq!(field.take_released(), vec![(b"the closure".to_vec(), Released::Returned)], "closed, let go");
    let _ = transport_order(&[wound(slide, b"too late")], &[], &mut field, &mut budget);
    assert_eq!(field.take_released(), vec![(b"too late".to_vec(), Released::Returned)], "nothing open to hold it");
    assert_eq!(field.tubes.threaded().1, 0, "no line stands on the tube");
}

/// MU-CLIENT §4.13 — A LINE THREADS WHAT ITS MOTION STANDS ON, AND IS LET GO
/// BY ITS CAUSE: a turn admitted free rings; a line threaded on that turn's
/// motion is held by its orbit, moving nothing; the turn's inverse brings the
/// orbit home and the threaded line is let go as returned. A gain's orbit
/// holds a line through the beat it was admitted in, and lets it go,
/// dissipated, at the next; dissipated, it holds nothing to thread.
#[test]
fn thread_holds_line_until_its_orbit_releases_it() {
    let mut field = Field::new();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 16);
    let _ = admit(&mut field, &[line(turn(E1 | E2), 1)]);
    assert_eq!(field.thread(&turn(E1 | E2), &[], b"the thought", &mut budget), Some(true), "held by the ringing orbit");
    let _ = admit(&mut field, &[line(turn(E1 | E2).reverse(), 1)]);
    let released = field.take_released();
    assert_eq!(released, vec![(b"the thought".to_vec(), Released::Returned)], "let go as its orbit returned");
    let _ = admit(&mut field, &[line(cga::dilator(&q(2, 1)), 1)]);
    assert_eq!(field.thread(&cga::dilator(&q(2, 1)), &[], b"the gain's", &mut budget), Some(true), "held through its first beat");
    let _ = admit(&mut field, &[line(turn(E1 | E3), 1)]);
    assert_eq!(field.take_released(), vec![(b"the gain's".to_vec(), Released::Dissipated)], "let go, dissipated, at the next");
    assert_eq!(field.thread(&cga::dilator(&q(2, 1)), &[], b"too late", &mut budget), Some(false), "a dissipated orbit holds nothing");
}
