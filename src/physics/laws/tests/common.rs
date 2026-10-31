use super::super::*;
use crate::physics::tensegrity::carried_body;
use crate::transducer::cga::{rotator, sphere_at};

pub(super) fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

pub(super) fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A body at `centre` with a second moment on its first axis.
pub(super) fn body(centre: [i64; 3]) -> Body {
    Body {
        centre: centre.iter().map(|&c| q(c)).collect(),
        mass: q(1),
        moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
    }
}

/// The (3,4,5) turn in (e₁, e₂) — `2 + e₁₂`, a rational rotor, no angle.
pub(super) fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// Two bodies carried by `motion` exactly: nucleus (a unit sphere — a point
/// is null and its inversive distance unread) by the sandwich, body by the
/// rigid carriage.
pub(super) fn carried(motion: &Mv) -> Vec<(BodyState, BodyState)> {
    [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|&c| {
            let b = body(c);
            let nucleus = sphere_at(&b.centre, &q(1));
            let after = BodyState { nucleus: motion.sandwich(&nucleus), body: carried_body(&b, motion).expect("a rigid carriage") };
            (BodyState { nucleus, body: b }, after)
        })
        .collect()
}
