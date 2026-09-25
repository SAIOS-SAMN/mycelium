// 24.3, R3 — THE YIELD SURFACE a voice dwells on: the reserve deficit and
// the entropic flux, per lane, never the open-demand backlog.

use crate::economy::yield_surface;
use crate::physics::Physics;

#[test]
fn the_surface_is_the_deficit_and_the_entropic_flux_never_the_backlog() {
    let mut p = Physics::default();
    assert_eq!(yield_surface(&p), Some([0; 4]), "a calm, full metabolism reads zero on every lane");
    p.bump(&[48_000, 28_000, 0, 53_000]);
    assert_eq!(yield_surface(&p), Some([0; 4]), "a backlog of any size is not on the surface");
    p.spend([true, false, false, false]);
    let deficit = p.reserve_deficit();
    assert!(deficit[0] > 0, "one expansion spent the spatial lane");
    assert_eq!(yield_surface(&p), Some(deficit), "the deficit is the surface, per lane");
    p.pivot([false, true, false, false]);
    let surface = yield_surface(&p).expect("the surface reads");
    assert_eq!(surface[1], deficit[1] + 1, "an unrelieved pivot is one entropic unit on its lane");
    assert_eq!((surface[0], surface[2], surface[3]), (deficit[0], deficit[2], deficit[3]));
}

/// STD1 — THE ORDER FLUX IS EXACT PAST 2⁶³, AND A SURFACE PAST ITS CARRIER
/// IS REFUSED. A torsion counter above `i64::MAX` wrapped negative under the
/// `as i64` the flux was read through; in `i128` it reads whole. A lane whose
/// deficit plus that excess passes a `u64` has no surface, never a pinned one.
#[test]
fn the_flux_is_exact_and_a_surface_past_its_carrier_is_refused() {
    let mut p = Physics::default();
    p.torsion[3] = u64::MAX;
    assert_eq!(p.order_flux()[3], -i128::from(u64::MAX), "the whole counter, never wrapped");
    assert_eq!(yield_surface(&p), Some([0, 0, 0, u64::MAX]), "a full reserve owes no deficit: the excess alone");
    p.spend([false, false, false, true]);
    assert!(p.reserve_deficit()[3] > 0, "the holonomic lane is spent");
    assert_eq!(yield_surface(&p), None, "deficit plus excess passes the carrier: refused");
}
