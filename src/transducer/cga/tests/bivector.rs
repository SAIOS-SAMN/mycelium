// 54.7 R3/R4 — the bivector trichotomy, the split, Cayley, the screw, and
// the curvature read off two boosts.

use super::common::{budget, q};
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::multivector::one;
use crate::transducer::cga::*;
use num_traits::{Signed, Zero};

/// The hyperbolic generators at the corpus's n = 3, named: the fixtures
/// below are read in `Cl(4,1)`.
static TRANSVECTION_PLANES: std::sync::LazyLock<Vec<usize>> =
    std::sync::LazyLock::new(|| transvection_planes(3));

/// 54.7 R3/R4 — THE BIVECTOR TRICHOTOMY IS A READING, AND `Cl(4,0)`
/// CANNOT MAKE IT.
///
/// Computed, never asserted. The three types are distinguished by the
/// sign of `I₁ = ⟨B²⟩₀/w`, and the identity is distinguished from a
/// null shear by `B` itself, which is the split the engine had collapsed.
#[test]
fn the_four_bivector_readings_are_distinct_and_exact() {
    use crate::network::nerve::coord::BivectorKind;
    let kind = |m: &Mv| m.class_invariants().map(|(_, _, k)| k);

    // ELLIPTIC: a 180° turn on e₁e₂. B² = −1 < 0.
    let rot = rotator(&q(0, 1), &q(1, 1));
    assert_eq!(kind(&rot), Some(BivectorKind::Elliptic));

    // HYPERBOLIC: a geodesic transvection on EVERY generator of ℍ⁴,
    // `e₊∧e₋` included. B² = +1 > 0. There is NO element of Cl(4,0)
    // with this reading — its metric is positive definite, so
    // ⟨B²⟩₀ ≤ 0 for every bivector it has.
    for &plane in TRANSVECTION_PLANES.iter() {
        let tv = transvection(&q(2, 1), &q(1, 1), plane)
            .expect("off the light cone, this is a transvection");
        assert_eq!(
            kind(&tv),
            Some(BivectorKind::Hyperbolic),
            "generator {plane:05b} is a boost of hyperbolic 4-space"
        );
    }
    assert_eq!(TRANSVECTION_PLANES.len(), 4, "ℍ⁴ has four boosts, not three");

    // TRIVIAL: B = 0. The identity turns nothing.
    assert_eq!(kind(&Mv::scalar(one())), Some(BivectorKind::Trivial));

    // PARABOLIC: the generator is built on `e_∞`, where `e_∞² = 0` —
    // NOT on `e₊∧e₋`, which is the fourth boost above. The conformal
    // translator is exactly this type, which is what makes it the
    // flatness R3 removes.
    let t = translator(&q(1, 1), &q(0, 1), &q(0, 1));
    assert_eq!(kind(&t), Some(BivectorKind::Parabolic), "T(d) fixes the point at infinity");
    assert!(e_inf().geo(&e_inf()).is_zero(), "e_∞² = e₋² + e₊² = 0");

    // And an element on the light cone `Ω Ω̃ = 0` has no inverse, so it
    // is refused rather than handed back as a versor. The weight is the
    // geometric product's, never `a² − b²` (rule 11).
    assert!(transvection(&q(1, 1), &q(1, 1), TRANSVECTION_PLANES[0]).is_none());
    assert!(transvection(&q(1, 1), &q(0, 1), E1 | E2).is_none(), "e₁e₂ is not a hyperbolic generator");
}

/// 54.7 — THE CANONICAL SPLIT, verified by the algebra and not asserted.
///
/// A screw's pitch is the frequency of its TWIST; its slip along the
/// axis is open and sounds nothing. The split is what lets a period be
/// asked of the part that can answer.
#[test]
fn a_screw_splits_into_commuting_compact_and_open_parts() {
    // A COUPLED generator: a rotation in e₁e₂ (a² = 1) and a boost on
    // e₃e₋ at half magnitude (b² = ¼). Worked by hand:
    //   ⟨B²⟩₀ = −1 + ¼ = −¾ ,  Q = e₁e₂e₃e₋ ,  κ = Q² = −1
    //   disc = 9/16 + 1 = 25/16 ,  √disc = 5/4
    //   b² = (−¾ + 5/4)/2 = ¼ ,  a² = ¼ + ¾ = 1
    let screw = Mv::blade(E1 | E2, q(1, 1)).add(&Mv::blade(E3 | EM, q(1, 2)));
    assert!(!bivector_is_simple(&screw), "the generator couples two planes");
    let (ell, hyp) = bivector_split(&screw, &mut budget()).expect("the split is exact over ℚ here");

    // THE BLADES, not their magnitudes: each part carries its own PLANE
    // and that is the whole point. `e₁e₂` is the twist, `e₃e₋` the slip.
    assert_eq!(ell, Mv::blade(E1 | E2, q(1, 1)), "the compact part IS the rotation plane");
    assert_eq!(hyp, Mv::blade(E3 | EM, q(1, 2)), "the open part IS the boost plane");
    assert!(ell.geo(&ell).scalar_part().is_negative(), "B_ell² < 0");
    assert!(hyp.geo(&hyp).scalar_part().is_positive(), "B_hyp² > 0");
    assert!(ell.geo(&hyp).sub(&hyp.geo(&ell)).is_zero(), "[B_ell, B_hyp] = 0");
    assert!(ell.add(&hyp).sub(&screw.grade(2)).is_zero(), "B_ell + B_hyp = B");
    assert!(bivector_is_simple(&ell) && bivector_is_simple(&hyp), "each part turns in ONE plane");

    // A SIMPLE generator carries the whole blade in one part, and WHICH
    // is the sign of ⟨B²⟩₀ — a reading, not a convention.
    let rot = Mv::blade(E1 | E2, q(1, 1));
    let (e3, h3) = bivector_split(&rot, &mut budget()).expect("a rotation splits");
    assert!(h3.is_zero() && e3 == rot, "a rotation is wholly compact, plane intact");
    let boost = Mv::blade(TRANSVECTION_PLANES[0], q(1, 1));
    let (e4, h4) = bivector_split(&boost, &mut budget()).expect("a boost splits");
    assert!(e4.is_zero() && h4 == boost, "a transvection is wholly open, plane intact");

    // THE COMPACT PART IS THE ONLY ONE THAT CAN SOUND, and evolving it
    // proves which: the twist is elliptic, the slip hyperbolic.
    use crate::network::nerve::coord::BivectorKind;
    let turn = cayley(&ell).expect("the compact part evolves");
    assert_eq!(turn.class_invariants().map(|(_, _, k)| k), Some(BivectorKind::Elliptic));
    let slip = cayley(&hyp).expect("the open part evolves");
    assert_eq!(slip.class_invariants().map(|(_, _, k)| k), Some(BivectorKind::Hyperbolic));
    assert!(e4.is_zero(), "a pure dislocation has no compact part to sound");
}

/// 54.7 R3 — A GENERATOR IS NOT A MOTION, and Cayley is the finite
/// action over ℚ.
///
/// `exp(½τB)` leaves the rationals for every `τ` worth having, and the
/// two ways round it — floating point, or pinning `Ω` to one blade —
/// lose exactness and the grade-4 coupling respectively. This measures
/// that Cayley loses neither: a SCREW generator evolves to a screw
/// versor, `I₂ ≠ 0` on both sides.
#[test]
fn cayley_evolves_a_generator_to_a_finite_versor_and_keeps_the_coupling() {
    use crate::network::nerve::coord::BivectorKind;
    // A SIMPLE hyperbolic generator evolves to a hyperbolic motion.
    let boost_gen = Mv::blade(TRANSVECTION_PLANES[0], q(1, 3));
    let boost = cayley(&boost_gen).expect("1 + B is invertible off the pole");
    assert_eq!(
        boost.class_invariants().map(|(_, _, k)| k),
        Some(BivectorKind::Hyperbolic),
        "the finite action of a boost generator is a boost"
    );
    assert!(bivector_is_simple(&boost), "one geodesic stays one geodesic");

    // A COUPLED generator — a rotation plane and a boost plane at once —
    // evolves to a screw, and `I₂` survives the passage. Under
    // `Ω = a + bB` this motion has no representation at all.
    let screw_gen = Mv::blade(E1 | E2, q(1, 2)).add(&Mv::blade(E3 | EM, q(1, 5)));
    assert!(!bivector_is_simple(&screw_gen), "the generator couples two planes");
    let screw = cayley(&screw_gen).expect("1 + B is invertible");
    assert!(!bivector_is_simple(&screw), "and so does its finite action");
    let (_, i2, _) = screw.class_invariants().expect("a versor has invariants");
    assert!(!i2.is_zero(), "I₂ = ⟨B∧B⟩₄ survives evolution, got {i2}");

    // The transform's pole is refused, never approximated.
    assert!(cayley(&Mv::scalar(one())).is_none(), "a scalar is not a generator");

    // AND A LOOP'S HOLONOMY IS THE ORDERED PRODUCT OF ITS EDGES. A
    // closed chain of transports that returns is flat; one that does
    // not is the obstruction the cell traps.
    let flat = boost.geo(&boost.reverse());
    assert!(flat.projective_eq(&Mv::scalar(one())), "Ω Ω̃ is the identity ray: a flat cell");
    let held = boost.geo(&screw);
    assert!(!held.projective_eq(&Mv::scalar(one())), "a cell that traps an obstruction");
    assert!(
        !boost.geo(&screw).projective_eq(&screw.geo(&boost)),
        "the product is path-ORDERED: transport does not commute"
    );
}

/// 54.7, Anti-Flattening clause 5 — A SCREW IS REACHABLE, which is what
/// the single-blade form `Ω = a + bB` made impossible.
///
/// That form spans `ℚ[B] ≅ ℚ ⊕ ℚ`, a two-dimensional split-complex
/// slice, and its generator is rank one — so `I₂ = ⟨B∧B⟩₄ = 0`
/// identically, no matter what `a` and `b` are. Every motion built that
/// way turns in one plane and slides along one track. Reflection
/// factorization has no such ceiling, and this MEASURES the difference:
/// a simple boost reads `I₂ = 0`, a coupled motion reads `I₂ ≠ 0`.
#[test]
fn a_coupled_screw_has_a_non_zero_second_casimir() {
    let z = q(0, 1);
    let v = |a: (i64, i64), b: (i64, i64), c: (i64, i64), p: (i64, i64), m: (i64, i64)| {
        reflector_vector(&q(a.0, a.1), &q(b.0, b.1), &q(c.0, c.1), &q(p.0, p.1), &q(m.0, m.1))
            .expect("a non-null vector reflects")
    };
    // Two reflection pairs on orthogonal planes: the first pair turns in
    // e₁e₂, the second boosts along e₃e₋. Their product couples them.
    let screw = versor_of_reflections(&[
        v((1, 1), (0, 1), (0, 1), (0, 1), (0, 1)),
        v((1, 1), (1, 1), (0, 1), (0, 1), (0, 1)),
        v((0, 1), (0, 1), (1, 1), (0, 1), (0, 1)),
        v((0, 1), (0, 1), (2, 1), (0, 1), (1, 1)),
    ])
    .expect("four reflections compose to a versor");
    assert!(
        !bivector_is_simple(&screw),
        "B ∧ B ≠ 0: the generator couples two planes, which a single blade cannot"
    );
    let (_, i2, _) = screw.class_invariants().expect("a versor has invariants");
    assert!(!i2.is_zero(), "I₂ = ⟨B∧B⟩₄ is non-zero on a screw, got {i2}");

    // A single-generator boost is simple, honestly: one geodesic, one
    // plane. The point is that the REPRESENTATION no longer forces it.
    let boost = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("a boost");
    assert!(bivector_is_simple(&boost), "one geodesic has a rank-one generator");
    let (_, i2_boost, _) = boost.class_invariants().expect("a versor has invariants");
    assert_eq!(i2_boost, z, "I₂ = 0 on a simple motion, which is a reading and not a ceiling");

    // An odd number of reflections is not in the spin group, and a null
    // vector reflects nothing — both refused rather than multiplied in.
    assert!(versor_of_reflections(&[v((1, 1), (0, 1), (0, 1), (0, 1), (0, 1))]).is_none());
    assert!(reflector_vector(&z, &z, &z, &q(1, 1), &q(1, 1)).is_none(), "e₊+e₋ is null");
}

/// 51.2, Anti-Flattening clause 7 — TRANSVECTIONS DO NOT COMMUTE, and
/// the failure IS the curvature. On a flat base the commutator of two
/// translations is the identity; here it is a rotation, and that is
/// read off the algebra rather than asserted as a value.
#[test]
fn two_boosts_do_not_commute_and_their_commutator_is_a_rotation() {
    use crate::network::nerve::coord::BivectorKind;
    let a = transvection(&q(2, 1), &q(1, 1), TRANSVECTION_PLANES[0]).expect("boost on e₁e₋");
    let b = transvection(&q(3, 1), &q(1, 1), TRANSVECTION_PLANES[1]).expect("boost on e₂e₋");
    assert!(!a.geo(&b).projective_eq(&b.geo(&a)), "ΩₐΩ_b ≠ Ω_bΩₐ: the base is not flat");
    let c = transvection_commutator(&a, &b).expect("the commutator is a versor");
    assert!(!c.projective_eq(&Mv::scalar(one())), "a non-zero curvature is a non-trivial commutator");
    assert_eq!(
        c.class_invariants().map(|(_, _, k)| k),
        Some(BivectorKind::Elliptic),
        "the commutator of two boosts is a ROTATION scaled by the sectional curvature"
    );
    // The conformal translators, by contrast, commute — which is the
    // statement that THAT reading is of a flat lattice (51.2).
    let (t1, t2) = (
        translator(&q(1, 1), &q(0, 1), &q(0, 1)),
        translator(&q(0, 1), &q(1, 1), &q(0, 1)),
    );
    assert!(t1.geo(&t2).projective_eq(&t2.geo(&t1)), "e_i ∧ e_∞ commute");
}

/// EVERY ROTOR LIFTS ELLIPTIC, and that is the statement that `Cl(4,0)`
/// has no transvection — not a defect of the lift (54.7 R4).
#[test]
fn a_lifted_rotor_is_always_elliptic_or_trivial() {
    use crate::network::nerve::coord::BivectorKind;
    use crate::transducer::alphabet::{LetterId, Word};
    for plane in [(0, 1), (0, 2), (1, 3), (2, 3)] {
        let Some(letter) = LetterId::turn_90(plane) else { continue };
        let w = Word::from_letters(&[(letter, false)]);
        let Some((_, _, kind)) = w.rotor().class_invariants() else { continue };
        assert!(
            matches!(kind, BivectorKind::Elliptic | BivectorKind::Trivial),
            "a Cl(4,0) rotor on {plane:?} read as {kind:?}; its bivector space is spacelike"
        );
    }
}
