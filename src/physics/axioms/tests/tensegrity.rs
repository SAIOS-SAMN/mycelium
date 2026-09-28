// A.10.1 — ORBITAL TENSEGRITY, read on exact constellations: the laws and the
// three theorems, each on engine values.

use crate::physics::tensegrity::{
    barycentre, carried, centred, consolidate, coupling, equilibrium, force, inversive, lifted, net_torque, spin_of, strain,
    torque, Constellation, Frame, Refusal, Star,
};
use crate::transducer::cga::{rotator, sphere_at, translator_at, Mv};
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

/// THEOREM 2 · COMMUTATIVE CO-PLANARITY. Two unit spheres standing on the
/// turn's own axis, the second posed by the slide along it: every coupling
/// commutes with the generator, so the torque vanishes term by term — the
/// constellation circulates coaxially, in stationary equilibrium, its strain
/// zero.
#[test]
fn t2_frames_on_the_axis_circulate_coaxially_in_equilibrium() {
    let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&[q(0, 1), q(0, 1), q(2, 1)])));
    let b_orbit = turn().grade(2);
    let v = coupling(&a, &b).grade(2);
    assert!(v.geo(&b_orbit).sub(&b_orbit.geo(&v)).is_zero(), "the coupling commutes with the generator");
    let c = Constellation { frames: vec![a, b], generator: turn() };
    assert!(net_torque(&c).is_zero(), "the torque vanishes");
    assert_eq!(equilibrium(&c), Ok(()), "stationary equilibrium");
    assert_eq!(strain(&c), Some(num_rational::BigRational::zero()), "its strain zero");
}

/// THEOREM 1 · BILATERAL INVERSIVE RECIPROCITY, as the algebra reads it. At an
/// equilibrium of frames in one pose the coupling is the pure tether, and it
/// transposes: `V_ji = s_i s_j = Ṽ_ij`, and `δ²` reads the same from both
/// frames. With the frames posed apart the transposition does NOT hold as
/// written — the pose and the tether do not commute — so the theorem stands
/// for the tether, the inversive distance symmetric at every equilibrium.
#[test]
fn t1_at_equilibrium_the_inversive_distance_is_symmetric_and_the_tether_transposes() {
    let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], one()));
    let c = Constellation { frames: vec![a.clone(), b.clone()], generator: turn() };
    assert_eq!(equilibrium(&c), Ok(()), "two frames on the axis in one pose: equilibrium");
    assert_eq!(inversive(&a, &b), inversive(&b, &a), "δ² symmetric, one well");
    assert!(coupling(&b, &a).projective_eq(&coupling(&a, &b).reverse()), "V_ji ∝ Ṽ_ij for the tether");
    let slide = translator_at(&[q(0, 1), q(0, 1), q(2, 1)]);
    let (a, b) = (frame([0, 0, 0], one()), frame([0, 0, 2], slide));
    let posed = Constellation { frames: vec![a.clone(), b.clone()], generator: turn() };
    assert_eq!(equilibrium(&posed), Ok(()), "posed apart along the axis: still equilibrium");
    assert_eq!(inversive(&a, &b), inversive(&b, &a), "δ² symmetric still");
    assert!(!coupling(&b, &a).projective_eq(&coupling(&a, &b).reverse()), "but the posed coupling does not transpose");
}

/// LAW II AND REPAIR B ON A NULL TORQUE: one unit sphere off the axis and its
/// image under the orbit's own turn. The torque does NOT vanish — the two
/// frames' pencil is not coaxial with the turn — and it is NULL: `⟨τ²⟩₀ = 0`,
/// the indefinite form blind to it. The coefficient norm reads it (repair B):
/// the strain is positive, and law II refuses the constellation.
#[test]
fn law_ii_refuses_a_null_torque_and_the_coefficient_norm_reads_it() {
    let a = frame([1, 0, 0], one());
    let b = carried(&a, &turn());
    let c = Constellation { frames: vec![a, b], generator: turn() };
    let tau = net_torque(&c);
    assert!(!tau.is_zero(), "the torque stands");
    assert!(tau.geo(&tau).scalar_part().is_zero(), "τ² has no scalar part: a null torque");
    assert!(strain(&c).is_some_and(|e| e > num_rational::BigRational::zero()), "the coefficient norm reads it");
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
/// against the turn about the ORIGIN are refused — the origin's lever arm, an
/// arbitrary origin's torque (rule 15), not the body's.
#[test]
fn tier_1_a_body_spinning_about_its_own_centroid_balances_and_about_the_origin_does_not() {
    let c = [q(1, 1), q(0, 1), q(0, 1)];
    let own = centred(&turn(), &c);
    let hours = [frame([1, 0, 0], one()), frame([1, 0, 0], own.clone())];
    let stacked: Vec<Frame> = hours.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect();
    let spun = Constellation { frames: stacked.clone(), generator: own };
    assert_eq!(equilibrium(&spun), Ok(()), "the spin about its own centroid balances");
    assert_eq!(strain(&spun), Some(num_rational::BigRational::zero()));
    let trapped = Constellation { frames: stacked, generator: turn() };
    assert!(matches!(equilibrium(&trapped), Err(Refusal::UnbalancedTensegrity { .. })), "about the origin it does not");
}

/// TIER 2 AS THE ALGEBRA READS IT: an equal binary, co-rotating about its
/// barycentre at the origin, each body at its own place through two hours.
/// The chord between the two bodies stands ACROSS the spin axis, and a chord
/// across the axis never commutes with the turn about it: the torque stands
/// and the binary is refused as unbalanced — the barycentre does not cancel
/// it. (78.51(4): the multi-body balance the ruling expected is not what the
/// coupling `V_ij` reads.)
#[test]
fn tier_2_a_binary_about_its_barycentre_is_refused_its_chord_across_the_axis() {
    let star = barycentre(&[(vec![q(1, 1), q(0, 1), q(0, 1)], q(1, 1)), (vec![q(-1, 1), q(0, 1), q(0, 1)], q(1, 1))]);
    assert_eq!(star.as_ref().map(|s| s.centre.clone()), Some(vec![q(0, 1), q(0, 1), q(0, 1)]), "the barycentre");
    let g = centred(&turn(), &[q(0, 1), q(0, 1), q(0, 1)]);
    let mut frames = Vec::new();
    for (m, pose) in [(0u64, one()), (1, g.clone())] {
        for c in [[1, 0, 0], [-1, 0, 0]] {
            frames.push(lifted(&frame(c, pose.clone()), m));
        }
    }
    let binary = Constellation { frames, generator: g };
    assert!(matches!(equilibrium(&binary), Err(Refusal::UnbalancedTensegrity { .. })), "the binary is refused");
    assert!(strain(&binary).is_some_and(|e| e > num_rational::BigRational::zero()));
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
/// A slide off the axis pulls the frame off it: the strain RISES (`5/2`), the
/// coefficient norm ordering what the indefinite square could not, and law II
/// refuses it as unbalanced, carrying the torque it left.
#[test]
fn law_iv_the_turn_is_conservative_and_the_slide_off_the_axis_raises_the_strain() {
    let c = Constellation {
        frames: vec![frame([0, 0, 0], one()), frame([0, 0, 2], translator_at(&[q(0, 1), q(0, 1), q(2, 1)]))],
        generator: turn(),
    };
    assert_eq!(force(&c, 1, &turn()), Some(num_rational::BigRational::zero()), "the turn about the axis exerts nothing");
    let mut kept = c.clone();
    kept.frames[1] = carried(&kept.frames[1], &turn());
    assert_eq!(consolidate(&c, &kept), Ok(()), "and is consolidated");
    let off = translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    assert_eq!(force(&c, 1, &off), Some(q(5, 2)), "the slide off the axis: the strain rises");
    let mut after = c.clone();
    after.frames[1] = carried(&after.frames[1], &off);
    assert!(
        matches!(consolidate(&c, &after), Err(Refusal::UnbalancedTensegrity { net_torque }) if !net_torque.is_zero()),
        "yet it unbalances the constellation, and law II refuses it"
    );
}

/// THE HOURS ARE HEIGHTS (78.51): a body on the turn's own axis, held through
/// two hours at heights 0 and 1, balances; the same body OFF the axis at
/// `(1,0,0)`, its hours as seen along the orbit about the origin, does not —
/// the lift alone does not place a body on the axis it turns about.
#[test]
fn the_hours_lifted_balance_on_the_axis_and_not_off_it() {
    let on = [frame([0, 0, 0], one()), frame([0, 0, 0], turn())];
    let on = Constellation { frames: on.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect(), generator: turn() };
    assert_eq!(equilibrium(&on), Ok(()), "on the axis: balanced");
    let a = frame([1, 0, 0], one());
    let seen = [a.clone(), carried(&a, &turn())];
    let off = Constellation { frames: seen.iter().zip(0u64..).map(|(f, m)| lifted(f, m)).collect(), generator: turn() };
    assert!(matches!(equilibrium(&off), Err(Refusal::UnbalancedTensegrity { .. })), "off it: refused");
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
