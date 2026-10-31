// 78.74 (draft, sections) — TWO GEOIDS OF ONE CLASS, RELATED THREE WAYS. A
// section of a class is one carriage of it: a body carried from its input
// pivot P to its output pivot Q, a fibre whose switch S = m_Q P − m_P Q is the
// white-hole construction's geoid (source at P, sink at Q). Two sections
// give two geoids, and they relate by:
//
// - CONGRUENCE — they overlay: the same weighed length and the same masses,
//   `S₁² = S₂²` (`S² = m_P m_Q d²`, read as `−2 m_P m_Q P·Q`, never a sum of
//   squares) — the second section is the first displaced: a copy.
// - COHERENCE — the sections' carriages are one motion up to torsion about the
//   fibre joining the sections' sources: the relative motion keeps both sources and
//   every level.
// - COHESION — entangled: the fascicle of the two geoids, `S₂ S₁`, binds them
//   when the geoids cut at an angle of finite order, so the bound pair
//   returns; apart (a boost) or at an angle of no finite order (a rosette),
//   nothing binds.

use crate::network::nerve::coord::BivectorKind;
use crate::physics::capacitor::{Carried, Fibre};
use crate::physics::fascicle::{Fascicle, Streamline};
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;
use num_traits::{Signed, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn at(x: i64, y: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1)])
}

/// A section's geoid: the switch of the fibre from where it was to where it
/// landed, weighed by the compartments at each end.
fn geoid(from: Mv, to: Mv, m_in: i64, m_out: i64) -> Fibre {
    Fibre::new(from, to, q(m_in, 1), q(m_out, 1)).expect("a section moves")
}

/// The bound pair's period, if the geoids' fascicle turns by an angle of
/// finite order.
fn binds(s1: &Mv, s2: &Mv) -> Option<u32> {
    let f = Fascicle { first: s1.clone(), second: s2.clone() };
    if f.streamline() != Some(Streamline::Elliptic) {
        return None;
    }
    let modes = match cga::motion_modes(&f.versor(), &mut crate::physics::Physics::default().stroke_n(1 << 16)) {
        Some(Ok(m)) => m,
        _ => return None,
    };
    let turns: Vec<_> = modes.iter().filter(|m| m.kind == BivectorKind::Elliptic).collect();
    match turns.as_slice() {
        [one] => one.order,
        _ => None,
    }
}

/// CONGRUENCE: two copies of one section, displaced, overlay — equal
/// weighed lengths and masses; a section moved further, or carrying another
/// mass, does not.
#[test]
fn switch_square_equal_for_congruent_sections() {
    let a = geoid(at(0, 0), at(3, 4), 2, 2);
    let copy = geoid(at(10, 1), at(13, 5), 2, 2);
    let further = geoid(at(10, 1), at(16, 9), 2, 2);
    let heavier = geoid(at(10, 1), at(13, 5), 3, 3);
    let s2 = |f: &Fibre| f.switch().geo(&f.switch()).scalar_part();
    assert_eq!(s2(&a), s2(&copy), "a copy: S² = m m' d² alike — congruent");
    assert_ne!(s2(&a), s2(&further), "moved further: not congruent");
    assert_ne!(s2(&a), s2(&heavier), "heavier: not congruent");
    assert_eq!(s2(&a), q(4 * 25, 1), "S² = m_P m_Q d², exact: 2·2·25");
}

/// COHERENCE: a section and its mirror across the line joining the two
/// sources are one motion up to torsion — the relative motion's flow, read
/// one dimension up where it is a flip, twists about the fibre joining the
/// sources and does no work; a section slid instead lifts a source.
#[test]
fn fibre_reads_coherent_sections_as_torsion() {
    let (p1, p2) = (at(0, 0), at(3, 4));
    let joining = Fibre::new(p1, p2, q(2, 1), q(2, 1)).expect("two sources");
    let v1 = Mv::scalar(q(1, 1));
    let mirror = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    let slid = cga::translator_at(&[q(1, 1), q(0, 1)]);
    let read = |m: &Mv| {
        let modes = crate::physics::capacitor::modes_of(m, 2, &mut crate::physics::Physics::default().stroke_n(1 << 16));
        joining.reads(&modes.expect("the modes read"))
    };
    assert_eq!(read(&v1.reverse().geo(&mirror)), Carried::Torsion, "coherent: the flip about the sources' line");
    assert_eq!(read(&v1.reverse().geo(&slid)), Carried::Lift, "not coherent: the slide moves a source");
}

/// THE CENTRE IS FORMULATED, NEVER ASSUMED: three pivots fix the pivots' round
/// (`P₁∧P₂∧P₃`), and its centre is read exactly as the round reflecting
/// infinity (`C e_∞ C`). A 90° turn about that centre keeps the round — the
/// mass orbits on its manifold and returns there; the SAME 90° turn about
/// the origin (a frame's implicit centre) is of the same order 4 and moves
/// the round off itself. Order alone does not say a transport returns on
/// the manifold: the centre the three points formulate does.
#[test]
fn round_centre_is_where_turn_keeps_round() {
    let (p1, p2, p3) = (at(0, 0), at(4, 0), at(0, 2));
    let round = p1.wedge_vector(1, &p2).wedge_vector(2, &p3);
    let centre = round.geo(&cga::e_inf()).geo(&round);
    assert!(centre.grade(1).projective_eq(&at(2, 1)), "the circumcentre (2, 1), exact");
    let quarter = |c: &Mv| {
        let t = cga::translator_at(&[q(2, 1), q(1, 1)]);
        let r = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
        match c.projective_eq(&at(2, 1)) {
            true => t.geo(&r).geo(&t.reverse()),
            false => r,
        }
    };
    let about_centre = quarter(&at(2, 1));
    let about_origin = quarter(&at(0, 0));
    assert!(about_centre.sandwich(&round).projective_eq(&round), "about its centre: the round is kept");
    assert!(!about_origin.sandwich(&round).projective_eq(&round), "about the origin: the same order, the round moved");
}

fn p3(x: i64, y: i64, z: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1), q(z, 1)])
}

fn p3q(x: BigRational, y: BigRational, z: BigRational) -> Mv {
    cga::point_at(&[x, y, z])
}

fn vec3(x: i64, y: i64, z: i64) -> Mv {
    Mv::blade(E1, q(x, 1)).add(&Mv::blade(E2, q(y, 1))).add(&Mv::blade(crate::transducer::cga::E3, q(z, 1)))
}

/// The tilted frame: c = (1,1,1); u = (1,2,2) and w = (2,1,−2), orthogonal,
/// each of length 3; the axis a = (−2,2,−1) = (u × w)/3, also length 3.
fn frame() -> ([i64; 3], [i64; 3], [i64; 3], [i64; 3]) {
    ([1, 1, 1], [1, 2, 2], [2, 1, -2], [-2, 2, -1])
}

/// The 90° turn in the plane of `u` and `w` about `c`: `9 + u∧w` (|u∧w| = 9,
/// so tan(θ/2) = 1, exact), carried to `c` by the translator.
fn tilted_quarter() -> Mv {
    let (c, u, w, _) = frame();
    let b = vec3(u[0], u[1], u[2]).wedge_vector(1, &vec3(w[0], w[1], w[2]));
    let r = Mv::scalar(q(9, 1)).add(&b);
    let t = cga::translator_at(&[q(c[0], 1), q(c[1], 1), q(c[2], 1)]);
    t.geo(&r).geo(&t.reverse())
}

/// TILT AND AXIS (rotation on a 3D object is read about x, y and z).
/// Three pivots in a TILTED plane fix the pivots' circle and its centre exactly;
/// the turn that keeps the circle is the turn in the circle's own plane —
/// about the axis the three points formulate, through the centre along
/// u × w — and that axis is fixed pointwise. The same 90° in the x,y plane
/// about the same centre moves the circle.
#[test]
fn turn_keeps_tilted_circle_about_its_own_axis() {
    let (c, u, w, a) = frame();
    let at3 = |d: [i64; 3], s: i64| p3(c[0] + s * d[0], c[1] + s * d[1], c[2] + s * d[2]);
    let (p1, p2, p3_) = (at3(u, 1), at3(w, 1), at3(u, -1));
    let circle = p1.wedge_vector(1, &p2).wedge_vector(2, &p3_);
    let centre = circle.geo(&cga::e_inf()).geo(&circle).grade(1);
    assert!(centre.projective_eq(&p3(c[0], c[1], c[2])), "the centre (1, 1, 1), exact, off every coordinate plane");
    let turn = tilted_quarter();
    assert!(turn.sandwich(&circle).projective_eq(&circle), "the turn in its own tilted plane keeps the circle");
    assert!(turn.sandwich(&at3(a, 1)).projective_eq(&at3(a, 1)), "and fixes its axis, c + a");
    assert!(turn.sandwich(&at3(a, -2)).projective_eq(&at3(a, -2)), "pointwise along it");
    assert!(!turn.sandwich(&p1).projective_eq(&p1), "while the circle's own points move round it");
    let flat = {
        let t = cga::translator_at(&[q(c[0], 1), q(c[1], 1), q(c[2], 1)]);
        t.geo(&Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)))).geo(&t.reverse())
    };
    assert!(!flat.sandwich(&circle).projective_eq(&circle), "the x,y turn about the same centre moves the tilted circle");
}

/// STABILITY OF A NUCLEUS AND ITS SIGMA POINTS INSIDE THE GEOID. The geoid
/// Σ is the sphere four pivots fix (c ± u, c + w, c + a, all at distance 3
/// from c). Inside it stand the nucleus c and its six sigma points c ± u/3,
/// c ± w/3, c ± a/3 (each at distance 1). Read per mode:
/// - ELLIPTIC — the turn about the formulated axis: the sigma set is carried
///   onto itself, every distance to c kept, Σ kept, and four steps return:
///   STABLE.
/// - PARABOLIC — the white-hole source on Σ at P = c + u: every sigma point
///   stays inside Σ, but the nucleus moves: bounded, not invariant —
///   MARGINAL.
/// - HYPERBOLIC — the capacitor step on the fibre through c from c + u to
///   c − u: the nucleus is driven along the field line: UNSTABLE.
#[test]
fn sigma_points_stable_only_under_axial_turn() {
    let (c, u, w, a) = frame();
    let third = |x: i64| BigRational::new(x.into(), 3.into());
    let centre = p3(c[0], c[1], c[2]);
    let pt = |d: [i64; 3], s: i64| p3(c[0] + s * d[0], c[1] + s * d[1], c[2] + s * d[2]);
    let sig = |d: [i64; 3], s: i64| {
        p3q(q(c[0], 1) + third(s * d[0]), q(c[1], 1) + third(s * d[1]), q(c[2], 1) + third(s * d[2]))
    };
    let sigma: Vec<Mv> = vec![centre.clone(), sig(u, 1), sig(u, -1), sig(w, 1), sig(w, -1), sig(a, 1), sig(a, -1)];
    let geoid = pt(u, 1).wedge_vector(1, &pt(w, 1)).wedge_vector(2, &pt(u, -1)).wedge_vector(3, &pt(a, 1));
    assert!(!geoid.is_zero(), "four pivots fix the geoid");
    // Distance² to c, read as −2 X·C over the weights (exact, never a root).
    let d2 = |x: &Mv| {
        let wx = -x.geo(&cga::e_inf()).scalar_part();
        -q(2, 1) * x.geo(&centre).scalar_part() / wx
    };
    let inside = |x: &Mv| d2(x) < q(9, 1);
    assert!(sigma.iter().all(inside), "the nucleus and its sigma points stand inside the geoid");
    // THE GEOID AS ONE VECTOR (the inner-product form): σ, the dual of the
    // pivots' wedge, holds every pivot on it by ONE scalar each — `P·σ = 0`
    // — and reads every inner point on one side of it by that scalar's sign;
    // no grade is climbed per point.
    let pseudo = Mv::blade(crate::transducer::cga::E1 | crate::transducer::cga::E2 | crate::transducer::cga::E3 | crate::transducer::cga::EP | crate::transducer::cga::EM, q(1, 1));
    let sigma_vec = geoid.geo(&pseudo).grade(1);
    for p in [pt(u, 1), pt(w, 1), pt(u, -1), pt(a, 1), pt(w, -1), pt(a, -1)] {
        assert!(p.geo(&sigma_vec).scalar_part().is_zero(), "a pivot on the geoid: P·σ = 0, one scalar");
    }
    let side = |x: &Mv| x.geo(&sigma_vec).scalar_part().is_positive();
    assert!(sigma.iter().all(|s| side(s) == side(&centre)), "every inner point on the nucleus's side, by one sign");
    assert_ne!(side(&p3(9, 9, 9)), side(&centre), "a point outside on the other");

    // ELLIPTIC: stable.
    let turn = tilted_quarter();
    for s in &sigma {
        let img = turn.sandwich(s);
        assert!(sigma.iter().any(|t| t.projective_eq(&img)), "the sigma set is carried onto itself");
        assert_eq!(d2(&img), d2(s), "each distance to the nucleus kept");
    }
    assert!(turn.sandwich(&geoid).projective_eq(&geoid), "the geoid kept");
    let four = turn.geo(&turn).geo(&turn).geo(&turn);
    assert!(four.projective_eq(&Mv::scalar(q(1, 1))), "four steps return: periodic");

    // PARABOLIC: bounded, not invariant.
    let along = |d: [i64; 3], s: BigRational| -> Vec<BigRational> {
        (0..3).map(|i| q(c[i] + u[i], 1) + &s * q(d[i], 1)).collect()
    };
    let through = |centre_of: Vec<BigRational>| {
        let cc = cga::point_at(&centre_of);
        let r2_half = -cc.geo(&pt(u, 1)).scalar_part();
        cc.sub(&cga::e_inf().scale(&r2_half))
    };
    let source = crate::physics::fascicle::Fascicle { first: through(along(w, q(1, 1))), second: through(along(w, q(2, 1))) }.versor();
    for s in &sigma {
        assert!(inside(&source.sandwich(s)), "the source keeps every sigma point inside");
    }
    assert!(!source.sandwich(&centre).projective_eq(&centre), "but the nucleus moves: marginal");

    // HYPERBOLIC: unstable.
    let fibre = crate::physics::capacitor::Fibre::new(pt(u, 1), pt(u, -1), q(1, 1), q(1, 1)).expect("a fibre through c");
    let step = fibre.charge(&q(3, 2));
    let moved = step.sandwich(&centre);
    assert!(!moved.projective_eq(&centre), "the nucleus is driven along the field line: unstable");
    assert!(
        cga::point_at(&[q(c[0] + u[0], 1), q(c[1] + u[1], 1), q(c[2] + u[2], 1)]).wedge_vector(1, &centre).wedge_vector(2, &moved).wedge_vector(3, &pt(u, -1)).is_zero(),
        "along the circle through both pivots: the field line"
    );
}

/// COHESION: two geoids that cut at 90° entangle into a 180° turn about
/// the pair's common round — period 2, bound; geoids that cut at another angle of
/// finite order bind at its period; two that stand apart breathe (a boost)
/// and never bind.
#[test]
fn fascicle_binds_geoids_cut_at_finite_angle() {
    // Equal masses: each geoid is its fibre's bisecting plane. The planes
    // x = 1 and y = 1 cut at 90°.
    let vertical = geoid(at(0, 0), at(2, 0), 1, 1).switch();
    let horizontal = geoid(at(0, 0), at(0, 2), 1, 1).switch();
    assert_eq!(binds(&vertical, &horizontal), Some(2), "90°: entangled into a 180° turn, period 2");
    // The planes x = 1 and x = y cut at 45°: a 90° turn, period 4.
    let diagonal = geoid(at(0, 0), at(1, -1), 1, 1).switch();
    let bound = binds(&geoid(at(0, 0), at(2, 0), 1, 1).switch(), &diagonal);
    assert!(matches!(bound, Some(4)), "45°: a 90° turn, period 4: {bound:?}");
    // Two concentric geoids stand apart: the pair's fascicle breathes, unbound.
    let inner = geoid(at(0, 0), at(1, 0), 1, 3).switch();
    let outer = geoid(at(0, 0), at(1, 0), 1, 5).switch();
    let apart = Fascicle { first: inner.clone(), second: outer.clone() }.streamline();
    assert_ne!(apart, Some(Streamline::Elliptic), "apart: no cut");
    assert_eq!(binds(&inner, &outer), None, "nothing binds");
}
