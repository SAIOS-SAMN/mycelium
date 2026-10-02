// 78.74 — A TEST PER CLAUSE OF THE CAPACITOR AND ITS TORSION
// (`physics::capacitor`), and of the white-hole source it was derived from:
// the inversion of a refusal is a flow out of a point of its own manifold,
// kept inside it, every scale the manifold's own.

use crate::physics::capacitor::{shape_of, Carried, Fibre, Shape};
use crate::physics::fascicle::{Fascicle, Streamline};
use crate::transducer::cga::{self, Mv, E1, E2, E3};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// The fibre from the origin to (3, 4, 0) with masses `mi`, `mj`.
fn fibre(mi: i64, mj: i64) -> Fibre {
    let (pi, pj) = (cga::point_at(&[q(0, 1), q(0, 1), q(0, 1)]), cga::point_at(&[q(3, 1), q(4, 1), q(0, 1)]));
    Fibre::new(pi, pj, q(mi, 1), q(mj, 1)).expect("two pivots, two masses")
}

/// A twist about the fibre's axis: a turn in the plane of `e₃` and the
/// in-plane normal `4e₁ − 3e₂`, both through the two pivots.
fn twist() -> Mv {
    let normal = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    Mv::scalar(q(1, 1)).add(&Mv::blade(E3, q(1, 1)).geo(&normal).grade(2).scale(&q(1, 5)))
}

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// The dual sphere centred at `c` through the point `p`: `C − ½ r² e_∞`, its
/// radius read as the inner product of the two points (`−½ r² = C·P`), never
/// as a sum of squares.
fn sphere_through(c: &[BigRational], p: &[BigRational]) -> Mv {
    let (cc, pp) = (cga::point_at(c), cga::point_at(p));
    let r2_half = -cc.geo(&pp).scalar_part();
    cc.sub(&cga::e_inf().scale(&r2_half))
}

fn along(p: &[BigRational], u: &[BigRational], t: &BigRational) -> Vec<BigRational> {
    p.iter().zip(u).map(|(pi, ui)| pi + t * ui).collect()
}

/// THE MANIFOLD: Σ centred at (1, 1, 0) through P = (4, 5, 0) — radius 5,
/// read, never declared. `u` is tangent to Σ at P: `u · (P − c) = 0`.
fn manifold() -> (Mv, [BigRational; 3], [BigRational; 3]) {
    let c = [q(1, 1), q(1, 1), q(0, 1)];
    let p = [q(4, 1), q(5, 1), q(0, 1)];
    let u = [q(4, 1), q(-3, 1), q(0, 1)];
    (sphere_through(&c, &p), p, u)
}

/// INSIDE Σ: the point's inner product with Σ, read at the point's own
/// weight (`−X·e_∞`), is positive.
fn inside(x: &Mv, sigma: &Mv) -> bool {
    let w = -x.geo(&cga::e_inf()).scalar_part();
    (x.geo(sigma).scalar_part() / w).is_positive()
}

/// Four conformal points lie on one circle iff their outer product vanishes.
fn concyclic(w: &Mv, x: &Mv, y: &Mv, z: &Mv) -> bool {
    w.wedge_vector(1, x).wedge_vector(2, y).wedge_vector(3, z).is_zero()
}

/// NO BOUNDARY IS DECLARED: A FIBRE IS ITS OWN SWITCH. A fibre between pivots
/// `Pᵢ` (mass `mᵢ`) and `Pⱼ` (mass `mⱼ`) gives the vector `S = mⱼ Pᵢ − mᵢ Pⱼ`:
/// a real sphere (`S² > 0`) whose reflection swaps the fibre's two ends, and
/// whose points are where the two pivots' weighed inner products balance,
/// `mⱼ X·Pᵢ = mᵢ X·Pⱼ` — the fibre's own equipotential. It is flat (the
/// bisecting plane, through `e_∞`) only where the masses balance; unequal
/// masses curve it. Switches of fibres sharing a pivot compose: the
/// multi-pivot carriage `S₂₃ S₁₂` takes `P₁` through `P₂` to `P₃`, exactly.
#[test]
fn each_fibre_is_its_own_polarity_switch_and_pivots_compose() {
    let pivots = [[q(0, 1), q(0, 1), q(0, 1)], [q(3, 1), q(4, 1), q(0, 1)], [q(-2, 1), q(7, 1), q(1, 1)]];
    let masses = [q(2, 1), q(1, 1), q(5, 1)];
    let pts: Vec<Mv> = pivots.iter().map(|p| cga::point_at(p)).collect();
    let switch = |i: usize, j: usize| {
        Fibre::new(pts[i].clone(), pts[j].clone(), masses[i].clone(), masses[j].clone()).expect("a fibre").switch()
    };
    let s12 = switch(0, 1);
    assert!(s12.geo(&s12).scalar_part().is_positive(), "a real sphere");
    assert!(s12.sandwich(&pts[0]).projective_eq(&pts[1]) && s12.sandwich(&pts[1]).projective_eq(&pts[0]), "it swaps the fibre's ends");
    assert!(!s12.geo(&cga::e_inf()).scalar_part().is_zero(), "unequal masses: curved, not through infinity");
    assert!(fibre(3, 3).switch().geo(&cga::e_inf()).scalar_part().is_zero(), "equal masses: the flat bisector, the degenerate case");
    let s23 = switch(1, 2);
    assert!(s23.geo(&s12).sandwich(&pts[0]).projective_eq(&pts[2]), "P₁ through P₂ to P₃");
    let one = cga::point_at(&[q(1, 1), q(1, 1), q(0, 1)]);
    assert_eq!(Fibre::new(one.clone(), one, q(1, 1), q(1, 1)), None, "one pivot is no fibre");
}

/// THE FIBRE AS A CAPACITOR. Its equipotentials are the pencil `S(λ) = λPᵢ −
/// Pⱼ`, each level `λ X·Pᵢ = X·Pⱼ`. Two levels compose an exact versor
/// `S(λ₂)S(λ₁)` that fixes both pivots (source and sink), is a boost (its
/// generator squares positive — the field, never a turn), carries every
/// level `μ` to `μ·(λ₂/λ₁)²` — the step between potentials a rational
/// ratio — and moves each point along a circle through both pivots (a field
/// line).
#[test]
fn a_fibre_is_a_capacitor_whose_levels_step_by_rational_ratios() {
    let f = fibre(2, 1);
    let (pi, pj) = (f.pi.clone(), f.pj.clone());
    let level = |l: BigRational| f.level(&l);
    let (l1, l2) = (q(2, 1), q(3, 1));
    let v = f.step(&l1, &l2);
    assert!(cga::is_versor(&v), "exact");
    assert!(v.sandwich(&pi).projective_eq(&pi) && v.sandwich(&pj).projective_eq(&pj), "both pivots fixed");
    let b = v.grade(2);
    assert!(b.geo(&b).scalar_part().is_positive(), "a boost: the field between them");
    let mu = q(5, 7);
    let ratio = &l2 / &l1;
    assert!(v.sandwich(&level(mu.clone())).projective_eq(&level(mu * &ratio * &ratio)), "level μ to μ(λ₂/λ₁)²");
    let x = cga::point_at(&[q(1, 1), q(-2, 1), q(0, 1)]);
    assert!(concyclic(&pi, &pj, &x, &v.sandwich(&x)), "along a field line through both pivots");
}

/// TORSION ON THE CAPACITOR: any generator `G` acting on the fibre `B = Pᵢ∧Pⱼ`
/// splits exactly — `B² = β²` is a rational scalar, so `BGB/β²` is exact — into
/// - the FIELD `cB`, `c = ⟨GB⟩₀/⟨BB⟩₀`: charge moved between levels;
/// - the TORSION `T`, commuting with `B` and annihilating both pivots
///   (`Pᵢ·T = Pⱼ·T = 0`): it fixes every level of the fibre — it does no work
///   across them — and carries what it moves about the fibre's axis;
/// - the LIFT `L`, anticommuting with `B`: it moves the pivots — the only
///   part that reshapes the capacitor.
///
/// A twist about the axis (a turn in the plane of two spheres through both
/// pivots) composed with the field is a loxodrome: one boost mode, one turn
/// mode, both pivots fixed, every level kept.
#[test]
fn torsion_splits_from_the_field_and_does_no_work_across_its_levels() {
    let f = fibre(2, 1);
    let (pi, pj) = (f.pi.clone(), f.pj.clone());
    let b = f.blade().clone();
    assert!(b.geo(&b).scalar_part().is_positive(), "the fibre is a boost's plane");
    let level = |l: BigRational| f.level(&l);
    // A generic generator: a turn, a slide, and a twist, mixed.
    let g = Mv::blade(E1 | E2, q(1, 1))
        .add(&Mv::blade(E1, q(1, 1)).geo(&cga::e_inf()).grade(2).scale(&q(2, 3)))
        .add(&Mv::blade(E3, q(1, 1)).geo(&Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)))).grade(2));
    let split = f.split(&g);
    let (field, torsion, lift) = (b.scale(&split.field), split.torsion.clone(), split.lift.clone());
    assert!(split.does_work(), "a field and a lift: work");
    assert_eq!(field.add(&torsion).add(&lift), g, "the three parts are G, exactly");
    let comm = |x: &Mv, y: &Mv| x.geo(y).sub(&y.geo(x));
    let anti = |x: &Mv, y: &Mv| x.geo(y).add(&y.geo(x));
    assert!(comm(&torsion, &b).is_zero() && comm(&field, &b).is_zero(), "field and torsion commute with the fibre");
    assert!(anti(&lift, &b).grade(2).is_zero() && !lift.is_zero(), "the lift anticommutes, and is there");
    assert!(torsion.geo(&b).scalar_part().is_zero(), "torsion has no field part");
    assert!(!torsion.is_zero() && !field.is_zero(), "and all three parts are there: the reading is not vacuous");
    let dot = |p: &Mv, t: &Mv| p.geo(t).sub(&t.geo(p)).grade(1);
    assert!(dot(&pi, &torsion).is_zero() && dot(&pj, &torsion).is_zero(), "torsion fixes both pivots");
    for l in [q(1, 1), q(2, 1), q(5, 7)] {
        assert!(dot(&level(l), &torsion).is_zero(), "and every level: no work across them");
    }
    assert!(!dot(&pi, &lift).is_zero(), "the lift moves a pivot");
    // The twist, finite and exact: a turn about the axis, composed with a
    // level step, is a loxodrome — one boost, one turn.
    let axis_turn = twist();
    assert!(!f.split(&axis_turn).does_work(), "the twist alone: torsion, no work");
    let step = f.step(&q(2, 1), &q(3, 1));
    assert!(axis_turn.geo(&step).projective_eq(&step.geo(&axis_turn)), "the twist commutes with the step");
    for l in [q(1, 1), q(5, 7)] {
        assert!(axis_turn.sandwich(&level(l.clone())).projective_eq(&level(l)), "the twist keeps each level");
    }
    let modes = match cga::motion_modes(&axis_turn.geo(&step), &mut crate::physics::Physics::default().stroke_n(1 << 16)) {
        Some(Ok(m)) => m,
        other => panic!("the loxodrome's modes read: {other:?}"),
    };
    let kinds: Vec<_> = modes.iter().map(|m| m.kind).collect();
    use crate::network::nerve::coord::BivectorKind;
    assert!(kinds.contains(&BivectorKind::Hyperbolic) && kinds.contains(&BivectorKind::Elliptic), "a boost and a turn: {kinds:?}");
}

/// STABILIZED, NOT DECLARED: a sphere through every pivot is orthogonal to
/// every fibre's switch whatever the masses (`Σ·Pᵢ = Σ·Pⱼ = 0`), so all of
/// them carry it onto itself — the shape the pivots hold still. Four pivots
/// fix it (`P₁∧P₂∧P₃∧P₄`, its dual); a fifth off it leaves no common shape,
/// read as the five points' outer product not vanishing.
#[test]
fn the_pivots_own_sphere_is_the_shape_every_switch_keeps() {
    let pivots = [[q(1, 1), q(0, 1), q(0, 1)], [q(0, 1), q(1, 1), q(0, 1)], [q(-1, 1), q(0, 1), q(0, 1)], [q(0, 1), q(0, 1), q(1, 1)]];
    let masses = [q(2, 1), q(1, 1), q(5, 1), q(3, 7)];
    let pts: Vec<Mv> = pivots.iter().map(|p| cga::point_at(p)).collect();
    let held = pts[0].wedge_vector(1, &pts[1]).wedge_vector(2, &pts[2]).wedge_vector(3, &pts[3]);
    assert!(!held.is_zero(), "four pivots in general position hold one sphere");
    for i in 0..4 {
        for j in (i + 1)..4 {
            let s = pts[i].scale(&masses[j]).sub(&pts[j].scale(&masses[i]));
            assert!(s.sandwich(&held).projective_eq(&held), "fibre {i}{j} keeps the pivots' sphere");
        }
    }
    let fifth = cga::point_at(&[q(2, 1), q(2, 1), q(2, 1)]);
    assert!(!held.wedge_vector(4, &fifth).is_zero(), "a fifth pivot off it: no common round shape stands");
    // The organ reads the same, and never calls an untested round stable:
    // four pivots only determine the sphere; a fifth on it makes it stand; a
    // fifth off it, emergent; three, open.
    assert_eq!(shape_of(&pts, 3), Shape::Determined(held.clone()), "four pivots: determined, not yet tested");
    let mut on_it = pts.clone();
    on_it.push(cga::point_at(&[q(0, 1), q(-1, 1), q(0, 1)]));
    assert_eq!(shape_of(&on_it, 3), Shape::Stable(held.clone()), "a fifth on it: the sphere stands");
    let mut more = pts.clone();
    more.push(fifth);
    assert_eq!(shape_of(&more, 3), Shape::Emergent, "a fifth off it: emergent");
    assert_eq!(shape_of(&pts[..3], 3), Shape::Open, "three cannot fix a sphere");
    // On the grid's plane (n = 2): three points determine a circle, a fourth
    // on it stands, a fourth off it does not.
    let on = |x: i64, y: i64| cga::point_at(&[q(x, 1), q(y, 1)]);
    assert!(matches!(shape_of(&[on(5, 0), on(0, 5), on(-5, 0)], 2), Shape::Determined(_)), "three: determined");
    assert!(matches!(shape_of(&[on(5, 0), on(0, 5), on(-5, 0), on(3, 4)], 2), Shape::Stable(_)), "concyclic");
    assert_eq!(shape_of(&[on(5, 0), on(0, 5), on(-5, 0), on(3, 3)], 2), Shape::Emergent, "not");
    assert_eq!(shape_of(&[on(5, 0), on(5, 0), on(0, 5)], 2), Shape::Open, "a repeated pivot fixes nothing new");
}

/// 78.74 — HOW A FINITE MOTION TREATS A FIBRE, read on the versor itself: the
/// identity carries it rigidly; the twist about its axis, and the mirror
/// across its line (an odd versor — the grid's flip), keep both pivots and
/// every level: torsion; a level step keeps the pivots and moves the levels:
/// field; a slide moves a pivot: lift.
#[test]
fn a_motion_carries_a_fibre_rigidly_by_torsion_by_field_or_by_lift() {
    let f = fibre(2, 1);
    assert_eq!(f.carried_by(&Mv::scalar(q(3, 1))), Carried::Rigid);
    assert_eq!(f.carried_by(&twist()), Carried::Torsion, "the twist about the axis");
    let mirror = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    assert_eq!(f.carried_by(&mirror), Carried::Torsion, "the flip across the fibre's line");
    assert_eq!(f.carried_by(&f.step(&q(2, 1), &q(3, 1))), Carried::Field, "a level step");
    assert_eq!(f.carried_by(&cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)])), Carried::Lift, "a slide");
}

/// THE SOURCE AT P ON Σ, KEPT IN Σ. Two spheres through P with centres on
/// Σ's tangent at P are orthogonal to Σ (`s·Σ = 0`) and kiss at P. Their
/// fascicle is parabolic and exact; it fixes P and not the ambient infinity;
/// it carries Σ onto itself and the inside of Σ inside — the flow emanates
/// from P and never leaves the manifold; its streamlines are circles through
/// P (horocycles of the ball Σ bounds).
#[test]
fn a_source_on_the_manifolds_own_infinity_stays_inside_it() {
    let (sigma, p, u) = manifold();
    let f = Fascicle { first: sphere_through(&along(&p, &u, &q(1, 1)), &p), second: sphere_through(&along(&p, &u, &q(2, 1)), &p) };
    for s in [&f.first, &f.second] {
        assert!(s.geo(&sigma).scalar_part().is_zero(), "each sphere is orthogonal to Σ");
    }
    assert_eq!(f.streamline(), Some(Streamline::Parabolic), "they kiss at P");
    let v = f.versor();
    assert!(cga::is_versor(&v), "exact");
    let pt = cga::point_at(&p);
    assert!(v.sandwich(&pt).projective_eq(&pt), "P is fixed: the source");
    assert!(!v.sandwich(&cga::e_inf()).projective_eq(&cga::e_inf()), "the ambient infinity is not");
    assert!(v.sandwich(&sigma).projective_eq(&sigma), "Σ is carried onto itself");
    let x = cga::point_at(&[q(2, 1), q(1, 3), q(0, 1)]);
    assert!(inside(&x, &sigma), "x is in the manifold");
    let (x1, x2) = (v.sandwich(&x), v.sandwich(&v.sandwich(&x)));
    assert!(inside(&x1, &sigma) && inside(&x2, &sigma), "and stays in it");
    assert!(!x1.projective_eq(&x), "x moved");
    assert!(concyclic(&pt, &x, &x1, &x2), "along a circle through P");
    let off = cga::point_at(&[q(0, 1), q(0, 1), q(5, 1)]);
    assert!(!concyclic(&pt, &x, &x1, &off), "not vacuous");
    let b = v.grade(2);
    assert!(!b.is_zero() && b.geo(&b).is_zero(), "its generator is null: no length, no lattice");
}

/// THE POLARITY SWITCH, ENCAPSULATED: reflection in Σ itself turns the
/// manifold inside out — inside to outside and back — and the source
/// commutes with it (its spheres are orthogonal to Σ), so the inward well
/// and the outward flow are one flow seen from either side of Σ, with P its
/// source on both. Σ's own points stand.
#[test]
fn reflection_in_the_manifold_switches_polarity_and_keeps_the_source() {
    let (sigma, p, u) = manifold();
    let f = Fascicle { first: sphere_through(&along(&p, &u, &q(1, 1)), &p), second: sphere_through(&along(&p, &u, &q(2, 1)), &p) };
    let v = f.versor();
    let x = cga::point_at(&[q(2, 1), q(1, 3), q(0, 1)]);
    let mirrored = sigma.sandwich(&x);
    assert!(inside(&x, &sigma) && !inside(&mirrored, &sigma), "inside goes outside: the polarity switched");
    assert!(sigma.sandwich(&mirrored).projective_eq(&x), "and back: an involution");
    assert!(sigma.geo(&v).projective_eq(&v.geo(&sigma)), "the source commutes with the switch");
    let pt = cga::point_at(&p);
    assert!(sigma.sandwich(&pt).projective_eq(&pt), "P, on Σ, stands");
}

/// THE SLIVERS, IN Σ's OWN SCALE: inversion in the sphere at P with Σ's own
/// radius opens Σ into a plane (P to infinity); a slide along that plane by
/// `t·u`, carried back, is a source at P that keeps Σ. They compose by their
/// rational parameter — 1/7 then 2/5 is 19/35 — and every one keeps Σ.
#[test]
fn slivers_in_the_manifolds_own_scale_compose_and_keep_it() {
    let (sigma, p, u) = manifold();
    let pt = cga::point_at(&p);
    // The sphere at P with Σ's radius: `−½ r² = C·P` read off Σ's centre.
    let c = cga::point_at(&[q(1, 1), q(1, 1), q(0, 1)]);
    let open = pt.add(&cga::e_inf().scale(&c.geo(&pt).scalar_part()));
    assert!(open.sandwich(&sigma).geo(&cga::e_inf()).scalar_part().is_zero(), "Σ opened into a plane: it passes through infinity");
    let sliver = |t: BigRational| {
        let d: Vec<BigRational> = u.iter().map(|ui| &t * ui).collect();
        open.geo(&cga::translator_at(&d)).geo(&open.reverse())
    };
    let composed = sliver(q(1, 7)).geo(&sliver(q(2, 5)));
    assert!(composed.projective_eq(&sliver(q(1, 7) + q(2, 5))), "1/7 then 2/5 is 19/35");
    for t in [q(1, 7), q(2, 5), q(-3, 2)] {
        let k = sliver(t);
        assert!(k.sandwich(&pt).projective_eq(&pt), "each fixes P");
        assert!(k.sandwich(&sigma).projective_eq(&sigma), "each keeps Σ");
    }
}
