// 56.2 — A.4 refused on the algebra, and the light cone as the chart's edge.

use num_traits::Zero;

/// A.4 IS REFUSED BECAUSE THE METRIC SAYS SO. `e₋² = −1`, so the form
/// is indefinite and no positive-definite inner product exists on it.
/// Asserted on the algebra rather than on a comment, because the whole
/// hyperbolic sector depends on the sign.
#[test]
fn em_squares_negative_so_metric_is_indefinite() {
    use crate::transducer::cga;
    let em = crate::transducer::element::Versor::blade(cga::EM, crate::core::Q::from_integer(1.into()));
    let sq = em.geo(&em).scalar_part();
    assert!(
        sq < crate::core::Q::zero(),
        "e₋² must be negative or the boost sector collapses into rotation; got {sq}"
    );
}

/// THE CAYLEY CHART'S BOUNDARY IS THE LIGHT CONE, and it is refused
/// by name rather than by falling through a generic failure.
///
/// `B = u ∧ e₋` has `B² = |u|²`, so `1 − B²` vanishes exactly on the
/// unit sphere, and `|u| = tanh(θ/2)` puts that at `θ → ∞` — infinite
/// rapidity. Every finite boost lies strictly inside; the boundary is
/// luminal. A displacement one step inside the cone must still read.
#[test]
fn cayley_covers_finite_boosts_and_refuses_cone() {
    use crate::core::Q;
    use crate::transducer::cga;
    let q = |n: i64, d: i64| Q::new(n.into(), d.into());
    // The hyperbolic generators at the corpus's n = 3, named.
    let planes = cga::transvection_planes(3);
    let gen = |s: Q| crate::transducer::element::Versor::blade(planes[0], s);

    // Inside the cone: |u| = 1/2, a finite boost, carried exactly.
    assert!(
        cga::cayley(&gen(q(1, 2))).is_some(),
        "a finite rapidity is inside the Cayley chart"
    );
    // ON the cone: |u| = 1, θ → ∞. No finite versor represents it.
    assert!(
        cga::cayley(&gen(q(1, 1))).is_none(),
        "the unit sphere is infinite rapidity and has no finite versor"
    );
    // Just inside it, and still exact — the boundary is a point, not
    // a neighbourhood that swallows nearby readings.
    assert!(
        cga::cayley(&gen(q(99, 100))).is_some(),
        "the chart is open up to the cone, not merely near its centre"
    );
}
