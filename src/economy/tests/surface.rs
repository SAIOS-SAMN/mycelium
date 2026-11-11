// 24.3, R3 — THE YIELD SURFACE a voice dwells on: the reserve deficit and
// the entropic flux, per lane, never the open-demand backlog.

use crate::economy::yield_surface;
use crate::physics::Physics;

/// The yield surface is the reserve deficit and the entropic flux per lane,
/// never the open-demand backlog.
#[test]
fn yield_surface_reads_deficit_and_flux_not_backlog() {
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

/// MU-DERIVATIONS D2.3 — A UNIT OF GAS AT REST COSTS ONE μ: the read price
/// of a court with no strain is one on every lane.
#[test]
fn read_price_is_one_at_rest() {
    assert_eq!(crate::economy::read_price(&[0; 4], 496), Some(crate::economy::Mu::counts([1; 4])));
}

/// AT A FULL DEFICIT (`y = R`) A UNIT OF GAS COSTS TWO, and a share of the
/// reserve prices exactly between: `y = 1` against `R = 3` is 4/3.
#[test]
fn read_price_is_two_at_full_deficit() {
    use crate::core::q::Q;
    let q = |n: i64, d: i64| Q::new(n.into(), d.into());
    let price = crate::economy::read_price(&[496, 0, 1, 248], 496).expect("a reserve");
    assert_eq!(price, crate::economy::Mu::of([q(2, 1), q(1, 1), q(497, 496), q(3, 2)]).expect("non-negative"));
    let thirds = crate::economy::read_price(&[1, 0, 0, 0], 3).expect("a reserve");
    assert_eq!(thirds.lane(0), q(4, 3));
    assert_eq!(crate::economy::read_price(&[0; 4], 0), None, "no reserve, no price");
}

/// THE PRICE IS SCALE-FREE: two hosts at the same relative strain price a
/// unit of gas alike, whatever the absolute size of their reserves.
#[test]
fn read_price_is_scale_free() {
    let small = crate::economy::read_price(&[8, 4, 0, 16], 16).expect("a reserve");
    let large = crate::economy::read_price(&[248, 124, 0, 496], 496).expect("a reserve");
    assert_eq!(small, large);
}

/// STD1 — THE ORDER FLUX IS EXACT PAST 2⁶³, AND A SURFACE PAST ITS CARRIER
/// IS REFUSED. A torsion counter above `i64::MAX` wrapped negative under the
/// `as i64` the flux was read through; in `i128` it reads whole. A lane whose
/// deficit plus that excess passes a `u64` has no surface, never a pinned one.
#[test]
fn yield_surface_refuses_past_carrier_and_flux_is_exact() {
    let mut p = Physics::default();
    p.torsion[3] = u64::MAX;
    assert_eq!(p.order_flux()[3], -i128::from(u64::MAX), "the whole counter, never wrapped");
    assert_eq!(yield_surface(&p), Some([0, 0, 0, u64::MAX]), "a full reserve owes no deficit: the excess alone");
    p.spend([false, false, false, true]);
    assert!(p.reserve_deficit()[3] > 0, "the holonomic lane is spent");
    assert_eq!(yield_surface(&p), None, "deficit plus excess passes the carrier: refused");
}
