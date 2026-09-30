// A.10.1 — ORBITAL TENSEGRITY, read on exact constellations: the laws and the
// three theorems, each on engine values. The coupling is ATTITUDE, pose only;
// the separation is governed by metric rigidity (78.51(4′)); the hours wind
// on a Clifford torus and A.13's momentum is carried by an unforced orbit
// (78.52 Row 1).

use crate::physics::tensegrity::{
    barycentre, carried, coherence, consolidate, coupling, equilibrium, external_torque, force, hour, inversive,
    metric_defect, momentum, net_torque, rigid, spin_of, strain, succession, torque, Body, Constellation, Frame,
    Refusal, Star,
};
use crate::transducer::cga::{rotator, rotator_on_blade, sphere_at, translator_at, Mv, E1, E3};
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

/// The turn the orbit circulates under: 90° in `e₁e₂` about the `e₃` axis.
fn turn() -> Mv {
    rotator(&q(1, 1), &q(1, 1)).widened(3)
}

/// A unit sphere at `c`, posed by `pose`.
fn frame(c: [i64; 3], pose: Mv) -> Frame {
    Frame { nucleus: sphere_at(&c.map(|x| q(x, 1)), &q(1, 1)), pose }
}

fn one() -> Mv {
    Mv::scalar(num_rational::BigRational::one()).widened(3)
}

fn zero() -> num_rational::BigRational {
    num_rational::BigRational::zero()
}

fn at(c: [i64; 3]) -> Vec<num_rational::BigRational> {
    c.map(|x| q(x, 1)).to_vec()
}

/// THEOREM 2 · COMMUTATIVE CO-PLANARITY. Two unit spheres on the turn's own
/// axis, the second posed by the slide along it: the coupling commutes with
/// the generator, the torque vanishes term by term, the strain zero.
#[test]
fn t2_frames_on_the_axis_circulate_coaxially_in_equilibrium() {
    let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&at([0, 0, 2]))));
    let b_orbit = turn().grade(2);
    let v = coupling(&a, &b).grade(2);
    assert!(v.geo(&b_orbit).sub(&b_orbit.geo(&v)).is_zero(), "the coupling commutes with the generator");
    let c = Constellation { frames: vec![a, b], generator: turn() };
    assert!(net_torque(&c).is_zero(), "the torque vanishes");
    assert_eq!(equilibrium(&c), Ok(()), "stationary equilibrium");
    assert_eq!(strain(&c), Some(zero()), "its strain zero");
}

/// THEOREM 1 · BILATERAL INVERSIVE RECIPROCITY. The pose coupling transposes,
/// `V_ji = Ṽ_ij`, in one pose and posed apart; `δ²` reads the same from both
/// frames.
#[test]
fn t1_at_equilibrium_the_inversive_distance_is_symmetric_and_the_coupling_transposes() {
    for pose in [one(), translator_at(&at([0, 0, 2]))] {
        let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], pose));
        let c = Constellation { frames: vec![a.clone(), b.clone()], generator: turn() };
        assert_eq!(equilibrium(&c), Ok(()), "on the axis: equilibrium");
        assert_eq!(inversive(&a, &b), inversive(&b, &a), "δ² symmetric, one well");
        assert!(coupling(&b, &a).projective_eq(&coupling(&a, &b).reverse()), "V_ji ∝ Ṽ_ij");
    }
}

/// LAW II AND REPAIR B ON A NULL TORQUE: the second frame posed by a slide
/// ACROSS the axis. Its torque against the turn is NULL, `⟨τ²⟩₀ = 0`; the
/// coefficient norm reads it and law II refuses.
#[test]
fn law_ii_refuses_a_null_torque_and_the_coefficient_norm_reads_it() {
    let c = Constellation {
        frames: vec![frame([0, 0, 0], one()), frame([0, 0, 1], translator_at(&at([2, 0, 0])))],
        generator: turn(),
    };
    let tau = net_torque(&c);
    assert!(!tau.is_zero(), "the torque stands");
    assert!(tau.geo(&tau).scalar_part().is_zero(), "τ² has no scalar part: a null torque");
    assert!(strain(&c).is_some_and(|e| e > zero()), "the coefficient norm reads it");
    assert!(matches!(equilibrium(&c), Err(Refusal::UnbalancedTensegrity { .. })), "law II refuses it");
}

/// THE TORQUE (78.51(1)) is the commutator's half, `½ [B, G]`: zero along the
/// generator's axis, the lift across it, antisymmetric.
#[test]
fn the_torque_is_the_commutator_zero_along_the_axis() {
    assert!(torque(&translator_at(&at([0, 0, 2])), &turn()).is_zero(), "a slide along the axis exerts no torque");
    let across = translator_at(&at([2, 0, 0]));
    let tau = torque(&across, &turn());
    assert!(!tau.is_zero(), "a slide across it does");
    assert!(torque(&turn(), &across).add(&tau).is_zero(), "the torque is antisymmetric");
}

/// THE VILLARCEAU SUCCESSION (78.52 Row 1). The hour rotor of the 90° turn
/// pairs it with the same turn in `e₃e₊`: the two planes commute, the rotor
/// closes after the turn's own period — four hours — and a body's four hours
/// are four distinct nuclei, the fifth its first again. No ruler, no step.
#[test]
fn the_hours_wind_isoclinically_and_close_after_the_turn_s_period() {
    let (r12, r3p) = (rotator(&q(1, 1), &q(1, 1)), rotator_on_blade(&q(1, 1), &q(1, 1), E3 | crate::transducer::cga::EP));
    assert!(r12.geo(&r3p).sub(&r3p.geo(&r12)).is_zero(), "the two planes commute");
    let r = succession(&turn(), &at([0, 0, 0])).expect("the turn winds");
    let mut power = one();
    for _ in 0..4 {
        power = r.geo(&power);
    }
    assert!(power.projective_eq(&one()), "the hours close after four");
    let s = frame([1, 0, 0], one()).nucleus;
    let hours: Vec<Frame> = (0..4).map(|m| hour(&s, &r, m)).collect();
    for (i, a) in hours.iter().enumerate() {
        for b in hours.iter().skip(i + 1) {
            assert!(!a.nucleus.geo(&b.nucleus).grade(2).is_zero(), "each hour its own nucleus");
        }
    }
    assert!(hour(&s, &r, 4).nucleus.projective_eq(&s), "the fifth hour is the first");
}

/// A SLIDE WINDS ON ITS OWN PARAMETER; THE IDENTITY WINDS ON NOTHING. A pure
/// slide's hour rotor is the slide itself; the identity has no succession —
/// it is reflexive, decided before any orbit is asked.
#[test]
fn a_slide_winds_on_its_own_parameter_and_the_identity_on_nothing() {
    let slide = translator_at(&at([1, 0, 0]));
    assert!(succession(&slide, &at([0, 0, 0])).is_some_and(|r| r.projective_eq(&slide)), "the slide is its own hour");
    assert_eq!(succession(&one(), &at([0, 0, 0])), None, "the identity has no succession");
}

/// THE INTRINSIC AXIS, TIER 1. A body at `(1,0,0)` of extent `r² = 2`, its
/// hours on the succession centred at its own centroid, the hour rotor its
/// generator: in equilibrium. Against the turn about the ORIGIN the same
/// frames are refused — an arbitrary origin's torque.
///
/// THE TORUS'S UNIT: a body of extent exactly `r² = 1` about the centre is
/// the torus's pole, `∝ e₊`; the half period (hour 2 of 4) carries it to
/// `−e₊`, its own ray — a collapse the unit length of `e₃e₊` makes.
#[test]
fn tier_1_a_body_winding_about_its_own_centroid_balances_and_about_the_origin_does_not() {
    let s = sphere_at(&at([1, 0, 0]), &q(2, 1));
    let r = succession(&turn(), &at([1, 0, 0])).expect("the turn winds");
    let unit = frame([1, 0, 0], one()).nucleus;
    assert!(hour(&unit, &r, 2).nucleus.projective_eq(&unit), "the unit sphere is the torus's pole");
    let frames: Vec<Frame> = (0..3).map(|m| hour(&s, &r, m)).collect();
    let own = Constellation { frames: frames.clone(), generator: r };
    assert_eq!(equilibrium(&own), Ok(()), "about its own centroid it balances");
    assert_eq!(strain(&own), Some(zero()));
    let trapped = Constellation { frames, generator: turn() };
    assert!(matches!(equilibrium(&trapped), Err(Refusal::UnbalancedTensegrity { .. })), "about the origin it does not");
}

/// TIER 2 · THE SYNCHRONOUS BINARY. An equal binary about its barycentre,
/// hour 0 at rest and hour 1 on the succession centred there: balanced, and
/// the turn keeps the pair's inversive distance.
#[test]
fn tier_2_a_synchronous_binary_about_its_barycentre_balances_and_is_rigid() {
    let star = barycentre(&[(at([1, 0, 0]), q(1, 1)), (at([-1, 0, 0]), q(1, 1))]).expect("the barycentre");
    assert_eq!(star.centre, at([0, 0, 0]));
    let r = succession(&turn(), &star.centre).expect("the turn winds");
    let bodies = [frame([1, 0, 0], one()).nucleus, frame([-1, 0, 0], one()).nucleus];
    let frames: Vec<Frame> = (0..2).flat_map(|m| bodies.iter().map(move |s| (s.clone(), m))).map(|(s, m)| hour(&s, &r, m)).collect();
    let binary = Constellation { frames, generator: r };
    assert_eq!(equilibrium(&binary), Ok(()), "the attitudes balance");
    let moved: Vec<(Mv, Mv)> = bodies.iter().map(|s| (s.clone(), turn().sandwich(s))).collect();
    assert_eq!(rigid(&moved), Ok(()), "the turn keeps the pair's inversive distance");
}

/// TIER 2 · WHAT REFUSES. A body TUMBLING out of the plane at its second hour
/// leaves an attitude torque; a pair DRIFTING APART is not rigid.
#[test]
fn tier_2_a_tumbling_body_is_unbalanced_and_a_drifting_pair_is_not_rigid() {
    let r = succession(&turn(), &at([0, 0, 0])).expect("the turn winds");
    let tumble = rotator_on_blade(&q(1, 1), &q(1, 1), E1 | E3);
    let frames = vec![frame([1, 0, 0], one()), frame([-1, 0, 0], one()), carried(&frame([1, 0, 0], one()), &tumble)];
    let c = Constellation { frames, generator: r };
    assert!(matches!(equilibrium(&c), Err(Refusal::UnbalancedTensegrity { .. })), "the tumble precesses");
    let s = |x: i64| frame([x, 0, 0], one()).nucleus;
    let drift = [(s(1), s(2)), (s(-1), s(-2))];
    assert!(matches!(rigid(&drift), Err(Refusal::NonRigid { i: 0, j: 1, ref delta }) if !delta.is_zero()), "the drift is not rigid");
}

/// 78.57 · THE COHERENCE FORM IS READ WHOLE, EACH DEFECT IN ITS OWN GRADE,
/// NEVER SUMMED. A transition that tumbles (Δω), drifts apart (Δδ²) and
/// grows a body on its step (τ_ext) carries all three defects at once — the
/// form files each blade, where a gate that stopped at the first would have
/// told the mind one third of what is missing. The synchronous binary,
/// turned rigidly with its momentum carried, closes: every component zero.
#[test]
fn the_coherence_form_files_every_defect_blade_and_closes_only_at_zero() {
    let r = succession(&turn(), &at([0, 0, 0])).expect("the turn winds");
    let tumble = rotator_on_blade(&q(1, 1), &q(1, 1), E1 | E3);
    let frames = vec![frame([1, 0, 0], one()), frame([-1, 0, 0], one()), carried(&frame([1, 0, 0], one()), &tumble)];
    let c = Constellation { frames, generator: r };
    let mut defect = coherence(&c, &c).expect("the frames stand and the strain does not rise");
    let s = |x: i64| frame([x, 0, 0], one()).nucleus;
    defect.metric.extend(metric_defect(&[(s(1), s(2)), (s(-1), s(-2))]));
    let d = |a: i64, b: i64| vec![vec![q(a, 1), zero()], vec![zero(), q(b, 1)]];
    let before = Body { centre: vec![q(1, 1), zero()], mass: q(4, 1), moments: d(2, 1) };
    let grown = Body { centre: vec![zero(), q(-1, 1)], mass: q(4, 1), moments: d(3, 1) };
    let g = rotator(&q(1, 1), &q(1, 1));
    defect.momentum.extend(external_torque(&before, &grown, &g).filter(|t| !t.is_zero()));
    assert_eq!(
        (defect.torsion.len(), defect.metric.len(), defect.momentum.len()),
        (1, 1, 1),
        "three defects, three blades: {defect:?}"
    );
    assert!(!defect.closes(), "the form does not close");
    // THE BINARY CLOSES: balanced, rigid, its momentum carried.
    let star = barycentre(&[(at([1, 0, 0]), q(1, 1)), (at([-1, 0, 0]), q(1, 1))]).expect("the barycentre");
    let r = succession(&turn(), &star.centre).expect("the turn winds");
    let bodies = [frame([1, 0, 0], one()).nucleus, frame([-1, 0, 0], one()).nucleus];
    let frames: Vec<Frame> = (0..2).flat_map(|m| bodies.iter().map(move |s| (s.clone(), m))).map(|(s, m)| hour(&s, &r, m)).collect();
    let binary = Constellation { frames, generator: r };
    let mut closed = coherence(&binary, &binary).expect("the binary stands");
    closed.metric.extend(metric_defect(&bodies.iter().map(|s| (s.clone(), turn().sandwich(s))).collect::<Vec<_>>()));
    let after = Body { centre: vec![zero(), q(-1, 1)], mass: q(4, 1), moments: d(1, 2) };
    closed.momentum.extend(external_torque(&before, &after, &g).filter(|t| !t.is_zero()));
    assert!(closed.closes(), "every component zero, exactly: the constellation locks: {closed:?}");
}

/// A.13 · THE MOMENTUM IS CARRIED BY AN UNFORCED ORBIT. A body of mass 4 at
/// `(1,0)`, its moments `diag(2, 1)`, turned 90° about the origin to `(0,−1)`,
/// its moments turned with it to `diag(1, 2)`: `Π_after = g Π_before g̃`, the
/// external torque zero. The same body GROWN on the way — its moments
/// `diag(3, 1)` — is not carried: the external torque stands.
#[test]
fn a13_an_unforced_orbit_carries_its_momentum_and_a_forced_one_leaves_a_torque() {
    let d = |a: i64, b: i64| vec![vec![q(a, 1), zero()], vec![zero(), q(b, 1)]];
    let before = Body { centre: vec![q(1, 1), zero()], mass: q(4, 1), moments: d(2, 1) };
    let after = Body { centre: vec![zero(), q(-1, 1)], mass: q(4, 1), moments: d(1, 2) };
    let g = rotator(&q(1, 1), &q(1, 1));
    assert!(!momentum(&before, &g.grade(2)).is_zero(), "the body has momentum");
    assert!(external_torque(&before, &after, &g).is_some_and(|t| t.is_zero()), "the orbit carries it");
    let grown = Body { moments: d(3, 1), ..after };
    assert!(external_torque(&before, &grown, &g).is_some_and(|t| !t.is_zero()), "a grown body leaves an external torque");
}

/// THE STAR SETS THE SPIN (78.51(2)).
#[test]
fn the_star_s_place_sets_the_spin_the_step_leaves() {
    let (before, after) = (at([1, 0, 0]), at([0, -1, 0]));
    let step = turn();
    let p = |x: &[num_rational::BigRational]| crate::transducer::cga::point_at(x);
    assert!(step.sandwich(&p(&before)).projective_eq(&p(&after)), "the step carries the centre");
    let star = |c: [i64; 3]| Star { centre: at(c), mass: q(1, 1) };
    let locked = spin_of(&step, &star([0, 0, 0]), &before, &after);
    assert!(locked.as_ref().is_some_and(|s| s.projective_eq(&one())), "about the origin the body is locked");
    let spin = spin_of(&step, &star([1, -1, 0]), &before, &after);
    assert!(spin.as_ref().is_some_and(|s| !s.projective_eq(&one())), "about (1,−1,0) it spins");
    assert!(spin.as_ref().is_some_and(|s| s.sandwich(&p(&before)).projective_eq(&p(&before))), "about its own centre");
    assert_eq!(spin_of(&step, &star([2, 0, 0]), &before, &after), None, "no orbit of a star it did not keep its distance from");
}

/// LAW IV · FORCE. The turn about the axis exerts nothing; a slide across it
/// raises the strain and law II refuses it.
#[test]
fn law_iv_the_turn_is_conservative_and_the_slide_across_the_axis_raises_the_strain() {
    let c = Constellation {
        frames: vec![frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&at([0, 0, 2])))],
        generator: turn(),
    };
    assert_eq!(force(&c, 1, &turn()), Some(zero()), "the turn about the axis exerts nothing");
    let mut kept = c.clone();
    kept.frames[1] = carried(&kept.frames[1], &turn());
    assert_eq!(consolidate(&c, &kept), Ok(()), "and is consolidated");
    let off = translator_at(&at([1, 0, 0]));
    assert!(force(&c, 1, &off).is_some_and(|f| f > zero()), "the slide across the axis: the strain rises");
    let mut after = c.clone();
    after.frames[1] = carried(&after.frames[1], &off);
    assert!(
        matches!(consolidate(&c, &after), Err(Refusal::UnbalancedTensegrity { net_torque }) if !net_torque.is_zero()),
        "and law II refuses it"
    );
}

/// THEOREM 3 · ANTI-MONOCULARITY, AND THE SINGULAR COLLAPSE.
#[test]
fn t3_a_contraction_to_one_frame_is_refused_never_coherent() {
    let a = frame([0, 0, 0], one());
    let alone = Constellation { frames: vec![a.clone()], generator: turn() };
    assert_eq!(equilibrium(&alone), Err(Refusal::MonocularOrbit));
    let crushed = Constellation { frames: vec![a.clone(), a.clone()], generator: turn() };
    assert_eq!(equilibrium(&crushed), Err(Refusal::SingularCollapse));
    let coherent = Constellation { frames: vec![a, frame([0, 0, 2], translator_at(&at([0, 0, 2])))], generator: turn() };
    assert_eq!(consolidate(&coherent, &alone), Err(Refusal::MonocularOrbit), "the contraction refused");
}
