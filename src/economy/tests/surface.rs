// 24.3, R3 — THE YIELD SURFACE a voice dwells on: the reserve deficit and
// the entropic flux, per lane, never the open-demand backlog.

use crate::economy::yield_surface;
use crate::physics::Physics;

#[test]
fn the_surface_is_the_deficit_and_the_entropic_flux_never_the_backlog() {
    let mut p = Physics::default();
    assert_eq!(yield_surface(&p), [0; 4], "a calm, full metabolism reads zero on every lane");
    p.bump(&[48_000, 28_000, 0, 53_000]);
    assert_eq!(yield_surface(&p), [0; 4], "a backlog of any size is not on the surface");
    p.spend([true, false, false, false]);
    let deficit = p.reserve_deficit();
    assert!(deficit[0] > 0, "one expansion spent the spatial lane");
    assert_eq!(yield_surface(&p), deficit, "the deficit is the surface, per lane");
    p.pivot([false, true, false, false]);
    let surface = yield_surface(&p);
    assert_eq!(surface[1], deficit[1] + 1, "an unrelieved pivot is one entropic unit on its lane");
    assert_eq!((surface[0], surface[2], surface[3]), (deficit[0], deficit[2], deficit[3]));
}
