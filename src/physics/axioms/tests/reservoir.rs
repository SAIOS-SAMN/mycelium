// MU-CLIENT §4.5 — DISSIPATION AND REFORMATION, THE RESERVOIR'S TWO DOORS:
// on a beat of the field what never returns dissipates into `e_∅` at its own
// coordinates, and a motion landing there re-forms it; per lane, nothing is
// made or lost outside the two doors.

use crate::transducer::cech::orbit::{Field, Thread};
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

fn admit(field: &mut Field, lines: &[FieldLine]) -> FluxReading {
    transport_order(lines, &[], field, &mut crate::physics::Physics::default().stroke_n(1 << 16))
}

fn turn(mask: usize) -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(mask, q(1, 1)))
}

/// A BOOST NEVER RETURNS, SO A BEAT DISSIPATES IT; A TURN RINGS. A gain opens
/// 𝒮 by 3, and a turn on another plane opens 𝒽. A turn that comes back on a
/// third plane is a beat: the gain moves into the reservoir whole — 𝒮's open
/// pressure falls by exactly 3, the reservoir holds 3 — while the open turn
/// rings on. The gain's inverse lands on its plane: the reservoir re-forms
/// it, the inverse closes it, and the 3 is relieved — converged, nothing
/// opened; the reservoir holds none.
#[test]
fn a_beat_dissipates_what_never_returns_and_motion_reforms_it() {
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

/// ON 𝒯 THE SAME: a slide opens its tube by 2; another tube's flux cancels —
/// a beat — and the first tube's open flux dissipates into the reservoir at
/// its ray, the tube itself frozen in. Flux on that ray re-forms it and
/// closes on it, relieving the 2.
#[test]
fn a_tubes_open_flux_dissipates_on_a_beat_and_reforms_on_its_ray() {
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
