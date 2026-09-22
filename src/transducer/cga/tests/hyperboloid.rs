// 54.7 Axiom B — laminae on the sheet, and the frame's base point.

use super::common::q;
use crate::transducer::cga::multivector::one;
use crate::transducer::cga::*;
use num_traits::Signed;

/// 54.7 AXIOM B — A LAMINA IS A RATIONAL POINT OF `ℍ³`, and the
/// transition between two of them is a DIRECTED transvection.
///
/// Every assertion here is a blade or a product, never a magnitude.
#[test]
fn a_lamina_lands_on_the_sheet_and_transports_along_a_geodesic() {
    let m = q(4, 1);
    let a = hyperbolic_lamina(&q(0, 1), &q(0, 1), &q(0, 1), &m).expect("the pole");
    let b = hyperbolic_lamina(&q(1, 1), &q(0, 1), &q(0, 1), &m).expect("one step");
    let c = hyperbolic_lamina(&q(0, 1), &q(1, 1), &q(0, 1), &m).expect("one step, other axis");

    // ON THE SHEET, exactly — `v² = −1` with no root taken anywhere.
    for v in [&a, &b, &c] {
        assert_eq!(v.geo(v), Mv::scalar(-one()), "v² = −1 on the hyperboloid");
    }
    assert_eq!(a, hyperboloid_pole(), "u = 0 is the pole");
    // OFF THE CHART IS REFUSED, not clamped: `|u| ≥ 1`.
    assert!(hyperbolic_lamina(&q(4, 1), &q(0, 1), &q(0, 1), &m).is_none());

    // THE TRANSITION TRANSPORTS, and is not an involution.
    let r = geodesic_transvection(&a, &b).expect("a geodesic joins them");
    let w = is_versor(&r).expect("R R̃ is a positive rational scalar");
    assert!(w.is_positive(), "R R̃ = 2(1 − v_a·v_b) ≥ 4, got {w}");
    assert_eq!(r.sandwich(&a).scale(&(one() / &w)), b, "R v_a R̃ / R R̃ = v_b exactly");

    // THE PLANE IS CARRIED: ⟨R⟩₂ is the geodesic 2-blade, simple, and
    // HYPERBOLIC — `(v_a∧v_b)² = cosh²d − 1 > 0`.
    let plane = r.grade(2);
    assert!(!plane.is_zero(), "the generator is a blade, not a scalar");
    assert!(bivector_is_simple(&r), "one geodesic spans one plane");
    assert!(
        plane.geo(&plane).scalar_part().is_positive(),
        "the geodesic plane is timelike: a transvection, not a rotation"
    );

    // AND IT IS NOT ORDER TWO. `Ω = mn` on the null cone squares to a
    // scalar and merely swaps its endpoints; this propagates.
    assert!(
        !r.geo(&r).sub(&Mv::scalar(r.geo(&r).scalar_part())).is_zero()
            || !r.geo(&r).grade(2).is_zero()
            || {
                // If R² happened to be scalar, transporting twice would
                // return to the start — which a real transvection does
                // not do.
                let twice = r.geo(&r);
                let wt = is_versor(&twice).expect("a versor squared is a versor");
                twice.sandwich(&a).scale(&(one() / wt)) != a
            },
        "a transvection propagates; an involution returns"
    );

    // TWO AXES DO NOT COMMUTE — the curvature, measured.
    let rb = geodesic_transvection(&a, &b).expect("step one way");
    let rc = geodesic_transvection(&a, &c).expect("step the other");
    assert!(!rb.geo(&rc).projective_eq(&rc.geo(&rb)), "boosts do not commute on ℍ³");
}

/// A FRAME'S BASE POINT is its versor applied to the pole, back on the
/// sheet — one exact division, no root.
#[test]
fn a_frames_base_point_returns_to_the_sheet() {
    let m = q(4, 1);
    let a = hyperbolic_lamina(&q(1, 1), &q(0, 1), &q(0, 1), &m).expect("a lamina");
    let r = geodesic_transvection(&hyperboloid_pole(), &a).expect("a geodesic");
    let p = frame_base_point(&r).expect("the frame stands somewhere");
    assert_eq!(p.geo(&p), Mv::scalar(-one()), "and that somewhere is on the sheet");
    assert_eq!(p, a, "the transvection's own base point is where it carried the pole");
}
