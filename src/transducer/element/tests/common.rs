//! 39 — FIXTURES for the element's tests: each one built through a
//! constructor the carrier itself exports.

use crate::core::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;

pub fn q(v: i64) -> Q {
    Q::from(BigInt::from(v))
}

pub fn elem_from_versor(v: Versor) -> Element {
    let mut e = Element::identity();
    e.action.spatial = Some(v);
    e
}

pub fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}
