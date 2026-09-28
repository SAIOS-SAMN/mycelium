// A.10.1 — ORBITAL TENSEGRITY, read on exact constellations: the laws and the
// three theorems, each on engine values. The coupling is ATTITUDE — relative
// pose and axial slide — and the separation is governed by metric rigidity
// (the amendment of 2026-09-28, 78.51(4′)).

use crate::physics::tensegrity::{
    barycentre, carried, centred, consolidate, coupling, equilibrium, force, inversive, lifted, net_torque, rigid, spin_of,
    strain, torque, Constellation, Frame, Refusal, Star,
};
use crate::transducer::cga::{rotator, rotator_on_blade, sphere_at, translator_at, Mv};
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

/// THEOREM 2 · COMMUTATIVE CO-PLANARITY. Two unit spheres standing on the
/// turn's own axis, the second posed by the slide along it: every coupling
/// commutes with the generator, so the torque vanishes term by term — the
/// constellation circulates coaxially, in stationary equilibrium, its strain
/// zero.
#[test]
fn t2_frames_on_the_axis_circulate_coaxially_in_equilibrium() {
    let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&[q(0, 1), q(0, 1), q(2, 1)])));
    let b_orbit = turn().grade(2);
    let v = coupling(&a, &b).map(|v| v.grade(2));
    assert!(v.is_some_and(|v| v.geo(&b_orbit).sub(&b_orbit.geo(&v)).is_zero()), "the coupling commutes with the generator");
    let c = Constellation { frames: vec![a, b], generator: turn() };
    assert!(net_torque(&c).is_some_and(|t| t.is_zero()), "the torque vanishes");
    assert_eq!(equilibrium(&c), Ok(()), "stationary equilibrium");
    assert_eq!(strain(&c), Some(zero()), "its strain zero");
}

/// THEOREM 1 · BILATERAL INVERSIVE RECIPROCITY. At an equilibrium on the axis
/// the attitude coupling transposes, `V_ji ∝ Ṽ_ij` — in one pose, and posed
/// apart along the axis, for the pose and the axial slide commute — and `δ²`
/// reads the same from both frames.
#[test]
fn t1_at_equilibrium_the_inversive_distance_is_symmetric_and_the_coupling_transposes() {
    let slide = translator_at(&[q(0, 1), q(0, 1), q(2, 1)]);
    for pose in [one(), slide] {
        let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], pose));
        let c = Constellation { frames: vec![a.clone(), b.clone()], generator: turn() };
        assert_eq!(equilibrium(&c), Ok(()), "on the axis: equilibrium");
        assert_eq!(inversive(&a, &b), inversive(&b, &a), "δ² symmetric, one well");
        let (ab, ba) = (coupling(&a, &b), coupling(&b, &a));
        assert!(matches!((ab, ba), (Some(ab), Some(ba)) if ba.projective_eq(&ab.reverse())), "V_ji ∝ Ṽ_ij");
    }
}

/// LAW II AND REPAIR B ON A NULL TORQUE: two frames an hour apart on the
/// axis, the second posed by a slide ACROSS it. Its attitude coupling carries
/// the transverse slide, whose torque against the turn is NULL: `⟨τ²⟩₀ = 0`,
/// the indefinite form blind to it. The coefficient norm reads it (repair B):
/// the strain is positive, and law II refuses the constellation.
#[test]
fn law_ii_refuses_a_null_torque_and_the_coefficient_norm_reads_it() {
    let across = translator_at(&[q(2, 1), q(0, 1), q(0, 1)]);
    let c = Constellation { frames: vec![frame([0, 0, 0], one()), frame([0, 0, 1], across)], generator: turn() };
    let tau = net_torque(&c).expect("placed");
    assert!(!tau.is_zero(), "the torque stands");
    assert!(tau.geo(&tau).scalar_part().is_zero(), "τ² has no scalar part: a null torque");
    assert!(strain(&c).is_some_and(|e| e > zero()), "the coefficient norm reads it");
    assert!(matches!(equilibrium(&c), Err(Refusal::UnbalancedTensegrity { .. })), "law II refuses it");
}

/// THE TORQUE (78.51(1)) is the commutator's half, `½ [B, G]`: zero for a
/// coupling along the generator's axis (the slide along `e₃` against the turn
/// in `e₁e₂`), and for a coupling across it the lift, which the turn's own
/// commutator reverses: `[G, B] = −[B, G]`.
#[test]
fn the_torque_is_the_commutator_zero_along_the_axis() {
    let along = translator_at(&[q(0, 1), q(0, 1), q(2, 1)]);
    assert!(torque(&along, &turn()).is_zero(), "a slide along the axis exerts no torque");
    let across = translator_at(&[q(2, 1), q(0, 1), q(0, 1)]);
    let tau = torque(&across, &turn());
    assert!(!tau.is_zero(), "a slide across it does");
    assert!(torque(&turn(), &across).add(&tau).is_zero(), "the torque is antisymmetric");
}

/// THE INTRINSIC AXIS (78.51(3)), TIER 1. A body at `(1,0,0)` held through
/// two hours, stacked at its own centre on the spin axis, posed by the turn
/// about its own centroid: in equilibrium, its strain zero. The same frames
/// against the turn about the ORIGIN are refused — the pose about the body
/// and the turn about the origin do not commute: an arbitrary origin's
/// torque (rule 15), not the body's.
#[test]
fn tier_1_a_body_spinning_about_its_own_centroid_balances_and_about_the_origin_does_not() {
    let c = [q(1, 1), q(0, 1), q(0, 1)];
    let own = centred(&turn(), &c);
    let hours = [frame([1, 0, 0], one()), frame([1, 0, 0], own.clone())];
    let stacked: Vec<Frame> = hours.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect();
    let spun = Constellation { frames: stacked.clone(), generator: own };
    assert_eq!(equilibrium(&spun), Ok(()), "the spin about its own centroid balances");
    assert_eq!(strain(&spun), Some(zero()));
    let trapped = Constellation { frames: stacked, generator: turn() };
    assert!(matches!(equilibrium(&trapped), Err(Refusal::UnbalancedTensegrity { .. })), "about the origin it does not");
}

/// TIER 2 · THE SYNCHRONOUS BINARY (78.51(4′)). An equal binary about its
/// barycentre at the origin, each body at its own place, hour 0 at rest and
/// hour 1 turned by the turn about the barycentre: every relative pose is a
/// power of the generator and every axial slide lies along its axis, so the
/// attitude torque vanishes; and the turn keeps the pair's inversive
/// distance, so the binary is rigid. It consolidates.
#[test]
fn tier_2_a_synchronous_binary_about_its_barycentre_balances_and_is_rigid() {
    let star = barycentre(&[(vec![q(1, 1), q(0, 1), q(0, 1)], q(1, 1)), (vec![q(-1, 1), q(0, 1), q(0, 1)], q(1, 1))]);
    assert_eq!(star.as_ref().map(|s| s.centre.clone()), Some(vec![q(0, 1), q(0, 1), q(0, 1)]), "the barycentre");
    let g = centred(&turn(), &[q(0, 1), q(0, 1), q(0, 1)]);
    let mut frames = Vec::new();
    for (m, pose) in [(0u64, one()), (1, g.clone())] {
        for c in [[1, 0, 0], [-1, 0, 0]] {
            frames.push(lifted(&frame(c, pose.clone()), m));
        }
    }
    let binary = Constellation { frames, generator: g.clone() };
    assert_eq!(equilibrium(&binary), Ok(()), "the attitudes balance");
    assert_eq!(strain(&binary), Some(zero()));
    let bodies: Vec<(Mv, Mv)> =
        [[1, 0, 0], [-1, 0, 0]].iter().map(|c| frame(*c, one()).nucleus).map(|s| (s.clone(), g.sandwich(&s))).collect();
    assert_eq!(rigid(&bodies), Ok(()), "the turn keeps the pair's inversive distance");
}

/// TIER 2 · WHAT REFUSES. A body TUMBLING out of the plane — posed at hour 1
/// by a turn in `e₁e₃` while its partner turns in `e₁e₂` — leaves an attitude
/// torque: refused as unbalanced, the precession its content. Two bodies
/// DRIFTING APART — `(±1,0,0)` to `(±2,0,0)` — change their inversive
/// distance: refused as non-rigid, `Δδ²` its content.
#[test]
fn tier_2_a_tumbling_body_is_unbalanced_and_a_drifting_pair_is_not_rigid() {
    let tumble = rotator_on_blade(&q(1, 1), &q(1, 1), crate::transducer::cga::E1 | crate::transducer::cga::E3);
    let frames =
        vec![lifted(&frame([1, 0, 0], one()), 0), lifted(&frame([-1, 0, 0], one()), 0), lifted(&frame([1, 0, 0], tumble), 1)];
    let c = Constellation { frames, generator: turn() };
    assert!(matches!(equilibrium(&c), Err(Refusal::UnbalancedTensegrity { .. })), "the tumble precesses");
    let s = |x: i64| frame([x, 0, 0], one()).nucleus;
    let drift = [(s(1), s(2)), (s(-1), s(-2))];
    assert!(matches!(rigid(&drift), Err(Refusal::NonRigid { i: 0, j: 1, ref delta }) if !delta.is_zero()), "the drift is not rigid");
}

/// THE STAR SETS THE SPIN (78.51(2)). One observed step — the turn about the
/// origin carrying a body's centre from `(1,0,0)` to `(0,−1,0)` — split against
/// three stars. At the origin, the whole step is the orbit and the spin the
/// identity: the body is locked. At `(1,−1,0)`, the orbit about that star
/// turns the other way and the spin is what is left, a turn about the body's
/// own centre that fixes it. At `(2,0,0)` the body did not keep its distance:
/// no orbit of that star carries it.
#[test]
fn the_star_s_place_sets_the_spin_the_step_leaves() {
    let (before, after) = ([q(1, 1), q(0, 1), q(0, 1)], [q(0, 1), q(-1, 1), q(0, 1)]);
    let step = turn();
    let at = |x: &[num_rational::BigRational]| crate::transducer::cga::point_at(x);
    assert!(step.sandwich(&at(&before)).projective_eq(&at(&after)), "the step carries the centre");
    let star = |c: [i64; 3]| Star { centre: c.map(|x| q(x, 1)).to_vec(), mass: q(1, 1) };
    let locked = spin_of(&step, &star([0, 0, 0]), &before, &after);
    assert!(locked.as_ref().is_some_and(|s| s.projective_eq(&one())), "about the origin the body is locked");
    let spin = spin_of(&step, &star([1, -1, 0]), &before, &after);
    let p = at(&before);
    assert!(spin.as_ref().is_some_and(|s| !s.projective_eq(&one())), "about (1,−1,0) it spins");
    assert!(spin.as_ref().is_some_and(|s| s.sandwich(&p).projective_eq(&p)), "about its own centre");
    assert_eq!(spin_of(&step, &star([2, 0, 0]), &before, &after), None, "no orbit of a star it did not keep its distance from");
}

/// LAW IV · FORCE, UNDER REPAIR B. On the coaxial equilibrium the generator's
/// own turn carries a frame about its axis and exerts nothing — conservative.
/// A slide across the axis poses the frame off it: the strain RISES, the
/// coefficient norm ordering what the indefinite square could not, and law II
/// refuses it as unbalanced, carrying the torque it left.
#[test]
fn law_iv_the_turn_is_conservative_and_the_slide_across_the_axis_raises_the_strain() {
    let c = Constellation {
        frames: vec![frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&[q(0, 1), q(0, 1), q(2, 1)]))],
        generator: turn(),
    };
    assert_eq!(force(&c, 1, &turn()), Some(zero()), "the turn about the axis exerts nothing");
    let mut kept = c.clone();
    kept.frames[1] = carried(&kept.frames[1], &turn());
    assert_eq!(consolidate(&c, &kept), Ok(()), "and is consolidated");
    let off = translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    assert!(force(&c, 1, &off).is_some_and(|f| f > zero()), "the slide across the axis: the strain rises");
    let mut after = c.clone();
    after.frames[1] = carried(&after.frames[1], &off);
    assert!(
        matches!(consolidate(&c, &after), Err(Refusal::UnbalancedTensegrity { net_torque }) if !net_torque.is_zero()),
        "and law II refuses it"
    );
}

/// THE HOURS ARE HEIGHTS (78.51): a body OFF the axis at `(1,0,0)`, its hours
/// as seen along the orbit about the origin, lifted to heights 0 and 1 — in
/// SYNCHRONOUS LOCK with the turn, its relative pose a power of it and its
/// slide along the axis: balanced. The same body turned in `e₁e₃` at its
/// second hour tumbles: refused.
#[test]
fn the_hours_lifted_in_synchronous_lock_balance_and_a_tumble_does_not() {
    let a = frame([1, 0, 0], one());
    let seen = [a.clone(), carried(&a, &turn())];
    let locked = Constellation { frames: seen.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect(), generator: turn() };
    assert_eq!(equilibrium(&locked), Ok(()), "in lock: balanced");
    let tumble = rotator_on_blade(&q(1, 1), &q(1, 1), crate::transducer::cga::E1 | crate::transducer::cga::E3);
    let tumbled = [a.clone(), carried(&a, &tumble)];
    let off = Constellation { frames: tumbled.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect(), generator: turn() };
    assert!(matches!(equilibrium(&off), Err(Refusal::UnbalancedTensegrity { .. })), "tumbling: refused");
}

/// THEOREM 3 · ANTI-MONOCULARITY, AND THE SINGULAR COLLAPSE. One frame alone
/// is refused as monocular; two frames on one nucleus are one frame crushed
/// twice, refused as a collapse; and a contraction of a coherent
/// constellation to one frame is refused, never admitted.
#[test]
fn t3_a_contraction_to_one_frame_is_refused_never_coherent() {
    let a = frame([0, 0, 0], one());
    let alone = Constellation { frames: vec![a.clone()], generator: turn() };
    assert_eq!(equilibrium(&alone), Err(Refusal::MonocularOrbit));
    let crushed = Constellation { frames: vec![a.clone(), a.clone()], generator: turn() };
    assert_eq!(equilibrium(&crushed), Err(Refusal::SingularCollapse));
    let coherent = Constellation {
        frames: vec![a, frame([0, 0, 2], translator_at(&[q(0, 1), q(0, 1), q(2, 1)]))],
        generator: turn(),
    };
    assert_eq!(consolidate(&coherent, &alone), Err(Refusal::MonocularOrbit), "the contraction refused");
}
