// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-FIELD v1 §3.4 (proposal 78.171), MU-SENSING §3.7, ROADMAP track 26 S9b —
//! AXIOMATIC GRAVITY: the local orbit's shear on the orthogonal orbit class.
//!
//! Two orbit classes are orthogonal when their two elliptic planes satisfy
//! `B₁ · B₂ = 0` (no grade-0 inner part) and `B₁ ∧ B₂ ≠ 0`, which on the
//! field's frame is exactly [`Meeting::Disjoint`]: the planes share no unit.
//!
//! The shear on the orthogonal class is 78.46's commuting part, the drag
//! (`LiftDrag::drag`), which changes rate along the orbit and leaves its plane
//! where it is. Orbits that are not in the orthogonal class (sharing a unit)
//! take the anticommuting part, the lift (`LiftDrag::lift`), which tilts the
//! plane and is not this shear. Where no orbit is held, nothing is produced,
//! and the manifold is flat.

use super::atlas::Orbit;
use super::refusal::FieldRefusal;
use super::turning::{Meeting, Turning, TurningPlane};
use crate::transducer::cga::{lift_drag, LiftDrag, Mv};

/// THE PRIMARY TURNING PLANE OF `orbit`, when every transition turns on
/// one plane; `None` where the orbit turns on several planes or none.
pub fn primary_plane(orbit: &Orbit) -> Option<TurningPlane> {
    let mut planes = orbit.revolution().iter().flat_map(|t| t.motion.turnings()).map(Turning::plane);
    let first = planes.next()?.clone();
    planes.all(|p| p == &first).then_some(first)
}

/// THE 78.46 SPLIT OF `neighbour`'S PLANE AGAINST `local`'S PLANE: drag
/// (`B_∥`, commuting, the rate along the plane) and lift (`B_⊥`,
/// anticommuting, the tilt of the plane).
pub fn plane_split(local: &TurningPlane, neighbour: &TurningPlane) -> Result<LiftDrag, FieldRefusal> {
    local.bivector().product(neighbour.bivector()).map_err(FieldRefusal::Mismatched)?;
    lift_drag(local.bivector().multivector(), neighbour.bivector().multivector())
        .ok_or_else(|| FieldRefusal::Unbound { charts: Vec::new() })
}

/// AXIOMATIC GRAVITY SHEAR (MU-SENSING §3.7): the shear of `local` on
/// `neighbour`. Returns `Some(split)` where `neighbour` is in the orthogonal
/// class (disjoint planes), the shear being the commuting drag part of 78.46's
/// split that changes the rate along `neighbour` leaving its plane intact.
/// Returns `None` where `neighbour` is not in the orthogonal class (not
/// disjoint, taking lift rather than shear).
pub fn axiomatic_shear(local: &TurningPlane, neighbour: &TurningPlane) -> Result<Option<LiftDrag>, FieldRefusal> {
    let meeting = local.meeting(neighbour)?;
    let split = plane_split(local, neighbour)?;
    match meeting {
        Meeting::Disjoint => Ok(Some(split)),
        _ => Ok(None),
    }
}

/// THE RATE CHANGE EXPERIENCED BY AN ORBIT UNDER AXIOMATIC GRAVITY FROM
/// A LOCAL ORBIT (MU-SENSING §3.7): returns `Some(drag)` where the neighbour
/// orbit turns on a plane of the orthogonal class (disjoint from the local
/// orbit's plane), advancing its rate; and `None` where the neighbour orbit's
/// plane is not disjoint (its drag is zero, so its rate does not change).
pub fn orbit_rate_shear(local: &Orbit, neighbour: &Orbit) -> Result<Option<Mv>, FieldRefusal> {
    let (Some(p_local), Some(p_neighbour)) = (primary_plane(local), primary_plane(neighbour)) else {
        return Ok(None);
    };
    Ok(axiomatic_shear(&p_local, &p_neighbour)?.map(|s| s.drag))
}
