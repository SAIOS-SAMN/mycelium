// 78.75 IN n DIMENSIONS (`physics::outflow`): the white-hole source on grade
// one and two alone, at whatever dimension was observed — the Cl(4,1) source
// on n = 3, coefficient for coefficient, and on S⁶⁰ and S³⁶⁰ each plane's own
// outflow, read exactly.

use crate::physics::outflow::{keeping, outflow_planes, Bivector, NullPoint, Plane, OutflowRefusal, SourceN, Vector};
use crate::physics::source::{geoid_vector, Source};
use crate::transducer::cga::{self, Mv, E1, E2, E3, EM, EP};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// A Cl(4,1) vector as the n = 3 model's coordinates: e₁, e₂, e₃, e₊, e₋.
fn vector_of(m: &Mv) -> Vec<BigRational> {
    [E1, E2, E3, EP, EM].iter().map(|b| m.coeff(*b).clone()).collect()
}

/// A Cl(4,1) bivector as the n = 3 model's components, by axis pair.
fn bivector_of(m: &Mv) -> Bivector {
    let terms = m
        .grade(2)
        .terms()
        .map(|(mask, c)| {
            let bits: Vec<usize> = (0..5).filter(|b| mask & (1 << b) != 0).collect();
            (bits[0], bits[1], c.clone())
        })
        .collect();
    Bivector::new(3, terms).expect("five axes")
}

fn at(x: i64, y: i64, z: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1), q(z, 1)])
}

/// The geoid of five pivots on the sphere of radius 5 about the origin, read
/// off the pivots (as `tests/source.rs` reads it).
fn sigma() -> Mv {
    let pivots = vec![at(5, 0, 0), at(0, 5, 0), at(-3, 4, 0), at(0, 0, 5), at(3, 0, 4)];
    match crate::physics::capacitor::shape_of(&pivots, 3) {
        crate::physics::capacitor::Shape::Stable(round) => geoid_vector(&round).expect("a sphere"),
        other => panic!("five pivots on one sphere stand: {other:?}"),
    }
}

/// ON n = 3 THE ORGAN IS THE Cl(4,1) SOURCE. For a lift with parts that
/// keep the geoid and parts that do not, the part kept read on grade one and
/// two equals the part `source::keeping` reads on the whole algebra; and at
/// each pivot the source both open, the generators agree coefficient for
/// coefficient — or both refuse.
#[test]
fn source_n_equals_cl41_source_on_three_axes() {
    let s = sigma();
    let blade = |a: usize, b: usize, c: i64| Mv::blade(a, q(c, 1)).geo(&Mv::blade(b, q(1, 1)));
    let lift = blade(E1, E3, 1).add(&blade(E2, EP, 2)).add(&blade(E3, EM, -3)).add(&blade(E1, E2, 5));
    let k = crate::physics::source::keeping(&s, &lift).expect("σ is a sphere");
    let sn = Vector::new(vector_of(&s)).expect("a vector");
    let kn = keeping(&sn, &bivector_of(&lift)).expect("σ is a sphere");
    assert_eq!(kn, bivector_of(&k), "the part kept, read on two grades, is the whole algebra's");
    for p in [at(5, 0, 0), at(0, 5, 0), at(-3, 4, 0), at(0, 0, 5), at(3, 0, 4)] {
        let pn = NullPoint::new(vector_of(&p)).expect("a pivot is null");
        match (Source::of(&s, &p, &k), SourceN::of(&sn, &pn, &kn)) {
            (Some(a), Ok(b)) => assert_eq!(b.generator, bivector_of(&a.generator), "one source"),
            (None, Err(_)) => {}
            (a, b) => panic!("the two organs disagree on a pivot: {a:?} / {b:?}"),
        }
    }
}

/// A turn of every base plane at its own rate, `Σ λᵢ e₂ᵢ₋₁ ∧ e₂ᵢ`, on the
/// geoid `e₊` (it holds no `e₊`, so the whole turn keeps it), at the pivot
/// `x + |x| e₋` with `x` one on its first `m` base axes (`m` a square, so
/// `|x|` is whole).
fn turned(n: usize, m: usize, root: i64) -> (Vector, NullPoint, Bivector) {
    let mut sigma = vec![BigRational::zero(); n + 2];
    sigma[n] = BigRational::one();
    let mut p = vec![BigRational::zero(); n + 2];
    for a in p.iter_mut().take(m) {
        *a = BigRational::one();
    }
    p[n + 1] = q(root, 1);
    let rates = (0..n / 2).map(|i| (2 * i, 2 * i + 1, q(i as i64 + 1, 7))).collect();
    (Vector::new(sigma).expect("a vector"), NullPoint::new(p).expect("null: |x|² = root²"), Bivector::new(n, rates).expect("a turn"))
}

/// EACH PLANE ITS OWN OUTFLOW, AT ANY OBSERVED DIMENSION. On S⁶⁰ (61 base
/// axes: thirty planes and an axis) and S³⁶⁰ (361: a hundred and eighty and
/// an axis), the source a many-plane turn opens has on plane `i` exactly its
/// rate times the pivot's square radius there, `λᵢ (P₂ᵢ₋₁² + P₂ᵢ²)`, and its
/// direction there is the pivot's in-plane position turned 90° on, scaled by
/// `λᵢ` — so every plane the pivot stands in turns at its own phase, and a
/// plane it holds nothing of reads zero.
#[test]
fn outflow_planes_reads_each_plane_at_any_dimension() {
    for (n, m, root) in [(61usize, 49usize, 7i64), (361, 361, 19)] {
        let (sigma, p, l) = turned(n, m, root);
        let planes = outflow_planes(&sigma, &p, &l).expect("a source");
        assert_eq!(planes.len(), n / 2, "every base plane, the odd axis paired with none");
        let one = |a: usize| q(i64::from(a < m), 1);
        for (i, f) in planes.iter().enumerate() {
            assert_eq!(f.plane, Plane { first: 2 * i + 1, second: 2 * i + 2 });
            let held = [2 * i, 2 * i + 1].iter().filter(|a| **a < m).count() as i64;
            let rate = q(i as i64 + 1, 7);
            assert_eq!(f.outflow, &rate * q(held, 1), "plane {} of n = {n}", i + 1);
            // The turn carries (P₂ᵢ₋₁, P₂ᵢ) to (−λP₂ᵢ, λP₂ᵢ₋₁): a quarter of
            // 360° ahead of the pivot in its plane.
            assert_eq!(f.direction, (-&rate * one(2 * i + 1), &rate * one(2 * i)), "plane {} of n = {n}: where the matter leaves", i + 1);
        }
        let src = SourceN::of(&sigma, &p, &keeping(&sigma, &l).expect("kept")).expect("a source");
        assert!(src.generator.contracted_by(p.vector()).expect("same n").is_zero(), "it fixes the pivot");
        assert!(src.generator.contracted_by(&sigma).expect("same n").is_zero(), "and keeps the geoid");
    }
}

/// THE LAW REFUSES BY NAME, never by a value: a zero point; a non-null one; a
/// geoid of square zero; a pivot off the geoid; a turn whose planes hold
/// nothing of the pivot; an axis past the coordinates; two dimensions mixed.
#[test]
fn source_n_refuses_non_null_point_and_still_plane() {
    let (sigma, p, l) = turned(61, 49, 7);
    let zero = vec![BigRational::zero(); 63];
    assert_eq!(NullPoint::new(zero.clone()), Err(OutflowRefusal::NotNull));
    let mut off = zero.clone();
    off[0] = BigRational::one();
    assert_eq!(NullPoint::new(off), Err(OutflowRefusal::NotNull), "e₁ is no null point");
    let mut light = zero.clone();
    (light[61], light[62]) = (BigRational::one(), BigRational::one());
    let light = Vector::new(light).expect("a vector");
    assert_eq!(keeping(&light, &l), Err(OutflowRefusal::NoSphere), "e₊ + e₋ squares to zero: a point, not a sphere");
    let mut tilted = zero.clone();
    // 1 + 4 + 4 + 16 − 25 = 0: null, with an e₊ part.
    (tilted[0], tilted[1], tilted[2], tilted[61], tilted[62]) = (q(1, 1), q(2, 1), q(2, 1), q(4, 1), q(5, 1));
    let tilted = NullPoint::new(tilted).expect("null");
    assert_eq!(SourceN::of(&sigma, &tilted, &l), Err(OutflowRefusal::OffGeoid), "on e₊ the pivot's e₊ part puts it off");
    let far = Bivector::new(61, vec![(59, 60, BigRational::one())]).expect("the last plane");
    assert_eq!(SourceN::of(&sigma, &p, &far), Err(OutflowRefusal::Still), "a plane holding nothing of the pivot moves it not");
    assert_eq!(Bivector::new(61, vec![(0, 63, BigRational::one())]), Err(OutflowRefusal::Axis));
    assert_eq!(keeping(&Vector::new(vec![BigRational::one(); 5]).expect("n = 3"), &l), Err(OutflowRefusal::Dimension));
}
