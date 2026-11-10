// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ROADMAP track 26 S9b, MU-SENSING §3.7, MU-CLIENT §6a —
//! AXIOMATIC GRAVITY CLAIMS:
//! - `only_disjoint_plane_orbits_change_rate_under_axiomatic_gravity`
//! - `axiomatic_gravity_shears_the_orthogonal_class`

use super::super::*;
use super::fixtures::{atlas_holding, frame, plane, revolution_turning, three_charts};

/// CLAIM: only disjoint plane orbits change rate under axiomatic gravity
/// (MU-SENSING §3.7; ROADMAP track 26 S9b).
///
/// An orbit's plane is the bivector B of its elliptic mode. Two orbit
/// classes are orthogonal when B₁ · B₂ = 0 and B₁ ∧ B₂ ≠ 0, which on the
/// frame is Meeting::Disjoint. The shear on the orthogonal class is 78.46's
/// commuting part, the drag, which changes rate and not plane. Orbits that
/// are not in the orthogonal class (sharing an axis) take the anticommuting
/// part, the lift (a tilt), with zero drag, and do not change rate.
#[test]
fn only_disjoint_plane_orbits_change_rate_under_axiomatic_gravity() {
    let f = frame(4);
    let (_, charts) = three_charts();

    // Three orbits: local on e₊₁e₊₂, orthogonal on e₊₃e₊₄ (disjoint), and sharing on e₊₂e₊₃.
    let p_local = plane(&f, 1, 2);
    let p_disjoint = plane(&f, 3, 4);
    let p_sharing = plane(&f, 2, 3);

    assert_eq!(p_local.meeting(&p_disjoint).expect("frame stands"), Meeting::Disjoint);
    assert_eq!(p_local.meeting(&p_sharing).expect("frame stands"), Meeting::Sharing);

    let orbit_local = revolution_turning(&charts, &p_local, 30, 1);
    let orbit_disjoint = revolution_turning(&charts, &p_disjoint, 30, 1);
    let orbit_sharing = revolution_turning(&charts, &p_sharing, 30, 1);

    // 1. The 78.46 split on the disjoint plane gives pure drag (rate change) and zero lift (no tilt).
    let split_disjoint = plane_split(&p_local, &p_disjoint).expect("splits");
    assert!(!split_disjoint.drag.is_zero(), "disjoint plane experiences non-zero drag (rate change)");
    assert!(split_disjoint.lift.is_zero(), "disjoint plane experiences zero lift (no tilt)");

    // 2. The 78.46 split on the sharing plane gives zero drag (no rate change) and pure lift (tilt).
    let split_sharing = plane_split(&p_local, &p_sharing).expect("splits");
    assert!(split_sharing.drag.is_zero(), "sharing plane experiences zero drag (no rate change)");
    assert!(!split_sharing.lift.is_zero(), "sharing plane experiences non-zero lift (plane tilt)");

    // 3. Orbit rate shear: only the disjoint plane orbit changes rate.
    let rate_change_disjoint = orbit_rate_shear(&orbit_local, &orbit_disjoint).expect("valid frame");
    assert!(rate_change_disjoint.is_some(), "disjoint plane orbit changes rate under axiomatic gravity");

    let rate_change_sharing = orbit_rate_shear(&orbit_local, &orbit_sharing).expect("valid frame");
    assert!(rate_change_sharing.is_none(), "sharing plane orbit does not change rate under axiomatic gravity");
}

/// CLAIM: axiomatic gravity shears the orthogonal class
/// (MU-SENSING §3.7, MU-CLIENT §6a; ROADMAP track 26 S9b).
///
/// An attractor's local orbit shears the orthogonal orbit class (disjoint
/// turning planes). Where no orbit is held, nothing is produced and the
/// manifold is flat. Where an attractor holds a bound orbit, its shear on
/// an orbit of the orthogonal class is the commuting drag component,
/// leaving the orthogonal plane intact while changing its rate.
#[test]
fn axiomatic_gravity_shears_the_orthogonal_class() {
    let f = frame(4);
    let (lifted, charts) = three_charts();

    let p_local = plane(&f, 1, 2);
    let p_orthogonal = plane(&f, 3, 4);
    let p_sharing = plane(&f, 2, 3);

    let orbit_local = revolution_turning(&charts, &p_local, 30, 1);
    let orbit_orthogonal = revolution_turning(&charts, &p_orthogonal, 30, 1);
    let orbit_sharing = revolution_turning(&charts, &p_sharing, 30, 1);

    // 1. Where no orbit is held, nothing is produced: the manifold is flat.
    let flat_atlas = lifted.atlas();
    let flat_shears = flat_atlas.axiomatic_shear(&orbit_orthogonal).expect("reads");
    assert!(flat_shears.is_empty(), "flat atlas holds no orbit: no shear produced");

    // 2. When the attractor holds orbit_local, it shears the orthogonal class.
    let atlas = atlas_holding(&lifted, orbit_local);
    let shears = atlas.axiomatic_shear(&orbit_orthogonal).expect("reads");
    assert_eq!(shears.len(), 1, "shears the orthogonal orbit class");

    // The shear is the drag component, which aligns with the orthogonal plane's bivector.
    let drag = &shears[0];
    assert!(!drag.is_zero(), "shear drag is non-zero");
    let orthogonal_bivector = p_orthogonal.bivector().multivector();
    assert!(drag.projective_eq(orthogonal_bivector), "shear drag lies along the orthogonal plane");

    // 3. Orbits not in the orthogonal class (sharing planes) receive no shear from axiomatic gravity.
    let sharing_shears = atlas.axiomatic_shear(&orbit_sharing).expect("reads");
    assert!(sharing_shears.is_empty(), "sharing plane is not in the orthogonal class: no shear");
}
