// 59, 63–65 — the inversive strain, the interface blade, and the field
// that condenses them.

/// T2 — THE INVERSIVE STRAIN IS EXACT, SIGNED AND SCALE-FREE (59.3).
///
/// `μ` itself is irrational — its denominator wants `√(r_a² r_b²)` —
/// so the pair `(num, den2)` carries it and every question is a
/// comparison of cross products. What must hold: the three contact
/// sides come out right, the horizon test agrees with them, and a
/// global dilation moves nothing.
#[test]
fn the_strain_reads_contact_exactly_and_scale_free() {
    use crate::transducer::blade::{self, Contact, Strain};
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    // A body against ITSELF is `δ = −1`: `‖ΔP‖ = 0` and `num = −2r²`,
    // so `num² = 4r⁴ = den2` exactly.
    //
    // THIS ASSERTED `Through` AND WAS WRONG (65). `δ = −1` is INTERNAL
    // TANGENCY — a codimension-1 contact interface — and the enum it
    // was written against had only three arms, folding the whole lower
    // half of the spectrum into "interpenetrating". The self-pair is
    // precisely where the two labels are both defensible, so the
    // fixture passed while testing what was written rather than what is
    // true; `the_five_inversive_regimes_are_read_exactly` exhibits the
    // case that is not ambiguous.
    let a = blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade;
    let s = Strain::between(&a, &a).expect("a body has a centroid");
    assert_eq!(
        s.side(),
        Contact::TangentInternal,
        "a body against itself touches itself from inside"
    );
    assert!(s.num < q(0), "and its numerator is negative");

    // FAR APART. Two dominoes a long way from one another: `‖ΔP‖²`
    // dominates, `δ > 1`.
    let far = blade::shape_of(&[(0, 100, 0), (1, 100, 0)]).blade;
    let s_far = Strain::between(&a, &far).expect("two centroids");
    assert_eq!(s_far.side(), Contact::Apart, "distant bodies are separated");

    // THE HORIZON ORDERS THEM, and the ordering is the physics: a
    // distant pair is outside every horizon a near pair is inside.
    let near = blade::shape_of(&[(0, 2, 0), (1, 2, 0)]).blade;
    let s_near = Strain::between(&a, &near).expect("two centroids");
    for mu_c in [q(1), q(4), q(16)] {
        assert!(
            !(s_far.within(&mu_c) && !s_near.within(&mu_c)),
            "no horizon admits the far pair while refusing the near one"
        );
    }
    assert!(s_near.within(&q(16)), "a generous horizon admits the near pair");
    assert!(!s_far.within(&q(16)), "and still refuses the far one");

    // SCALE-FREE, which is Axiom C's whole point: `δ² = num²/den2` is
    // invariant under `x ↦ λx` because `num ↦ λ²num` and
    // `den2 ↦ λ⁴den2`. Doubling every coordinate must not move the
    // reading or the side.
    let a2 = blade::shape_of(&[(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)]).blade;
    let far2 = blade::shape_of(&[
        (0, 200, 0),
        (1, 200, 0),
        (0, 201, 0),
        (1, 201, 0),
    ])
    .blade;
    let scaled = Strain::between(&a2, &far2).expect("two centroids");
    assert_eq!(scaled.side(), s_far.side(), "a dilation does not change the side");

    // AND NO ROOT WAS TAKEN ANYWHERE: `den2` is a square and is never
    // rooted; it is strictly positive because `radius2` is.
    assert!(s.den2 > q(0), "the denominator squared is strictly positive");
    assert!(s_far.den2 > q(0));
}

/// T3 — THE STRAIN IS A BILINEAR FORM, NOT A PAIRWISE TEST.
///
/// `Strain::between` subtracts centroids and squares, which reads as a
/// pairwise computation and invites a pairwise walk. It is not one. On
/// the conformal representative `s = P − ½r²e_∞` the numerator is an
/// INNER PRODUCT:
///
/// ```text
///     s_a · s_b = ½(r_a² + r_b² − ‖P_a − P_b‖²)
///     num       = −2 (s_a · s_b)
/// ```
///
/// which is LINEAR IN EACH ARGUMENT. That is what T3 needs and what a
/// bucketing scheme would destroy: "which bodies lie within `A`'s
/// horizon" is a linear functional evaluated against the carrier, so a
/// keyed descent is possible exactly when the key's coarse strata are
/// level sets of that functional. Quantising `⌊log₂ r⌋` or `P mod 2^k`
/// answers it with a grid instead, which is `ring_of_support` at a
/// coarser pitch — and `⌊log₂⌋` is not even exact in ℚ.
///
/// The identity is asserted here so the linear form is a measured fact
/// before anything is built on it.
#[test]
fn the_strain_numerator_is_an_inner_product_of_conformal_spheres() {
    use crate::transducer::blade::{self, Strain};
    use crate::transducer::cga;
    let q = |n: i64| crate::core::Q::from_integer(n.into());
    let two = q(2);

    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(0, 5, 0), (1, 5, 0), (0, 6, 0)]).blade,
        blade::shape_of(&[(3, 3, 0)]).blade,
    ];

    for a in &bodies {
        for b in &bodies {
            let s = Strain::between(a, b).expect("both centroids read");
            let (pa, pb) = (a.centroid().expect("a"), b.centroid().expect("b"));
            let sa = cga::sphere(&pa.0, &pa.1, &pa.2, &a.radius2());
            let sb = cga::sphere(&pb.0, &pb.1, &pb.2, &b.radius2());
            // The scalar part of the geometric product IS the inner
            // product of two vectors.
            let dot = sa.geo(&sb).scalar_part();
            assert_eq!(
                s.num,
                -&two * &dot,
                "num = −2 (s_a · s_b): the strain is bilinear in the spheres"
            );
        }
    }
}

/// T3 — THE STRAIN DEPENDS ONLY ON THE RELATIVE MOTOR (ruled
/// 2026-09-22).
///
/// A body's nucleus is `s = M s₀ M̃` for a motor `M ∈ Spin⁺(4,1)(ℚ)`,
/// and a versor is an orthogonal transformation of `ℚ^{4,1}`, so it
/// preserves the inner product. Therefore
///
/// ```text
///     (M s_a M̃) · (M s_b M̃) = s_a · s_b
/// ```
///
/// and `num = −2(s_a · s_b)` is a function of `M_a⁻¹ M_b` ALONE —
/// never of where either body sits absolutely. Absolute position is a
/// gauge artifact of the strain.
///
/// WHAT THAT COLLAPSES. A spatial index would answer "which bodies are
/// near me"; the spellable horizon answers "can my mind factor `g_ab`".
/// Since the relative motor IS `g_ab`, those are one question, and T3
/// is not an index bolted beside the physics — proximity is membership
/// of the relative motor in the alphabet's spellable neighbourhood.
///
/// Asserted here on a motor with BOTH halves — a turn composed with a
/// displacement — because a translation-only fixture would pass on the
/// commuting case and say nothing about the screw.
#[test]
fn the_strain_is_invariant_under_a_common_motor() {
    use crate::transducer::blade::{self, Strain};
    use crate::transducer::cga;
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(4, 7, 0), (5, 7, 0), (4, 8, 0)]).blade,
        blade::shape_of(&[(-3, 2, 0)]).blade,
    ];
    let nucleus = |b: &blade::Blade| {
        let p = b.centroid().expect("a centroid");
        cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
    };

    // A motor with a turn AND a displacement, so the two halves do not
    // commute and the test is not a translation identity in disguise.
    let motor = cga::translator(&q(11), &q(-4), &q(2)).geo(&cga::rotator(&q(1), &q(1)));

    // THE INVARIANCE IS PROJECTIVE, NOT ON THE NOSE, and a first
    // statement of it claimed otherwise. A versor preserves the inner
    // product only when it is ORTHOGONAL — `M M̃ = 1` — and in ℚ a
    // versor cannot be normalised without a square root, so every motor
    // here is a RAY. `rotator(1,1) = 1 + e₁₂` has `M M̃ = 2`, and the
    // sandwich scales the inner product by `λ² = 4`:
    //
    //     num ↦ λ² num,   den2 ↦ λ⁴ den2,   δ² = num²/den2 unchanged
    //
    // which is 57.2's ray exactly — "canonicalization is not a
    // homomorphism" — and is why Axiom C carries the ratio and never
    // evaluates it. The scale-free reading is invariant; the numerator
    // alone is not, and a design that keyed on `num` would key on the
    // gauge.
    let lambda = motor.geo(&motor.reverse()).scalar_part();
    assert_eq!(lambda, q(2), "the fixture's motor is a ray of norm 2, not a unit");
    for a in &bodies {
        for b in &bodies {
            let (sa, sb) = (nucleus(a), nucleus(b));
            let before = sa.geo(&sb).scalar_part();
            let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
            assert_eq!(
                after,
                &lambda * &lambda * &before,
                "a common motor scales the strain by its own norm squared"
            );
            // AND THE SCALE-FREE READING SURVIVES EXACTLY. `num²/den2`
            // is what the horizon is taken on, and it does not move.
            let (ra2, rb2) = (sa.geo(&sa).scalar_part(), sb.geo(&sb).scalar_part());
            let (ra2m, rb2m) = (
                motor.sandwich(&sa).geo(&motor.sandwich(&sa)).scalar_part(),
                motor.sandwich(&sb).geo(&motor.sandwich(&sb)).scalar_part(),
            );
            let delta2 = |n: &crate::core::Q, x: &crate::core::Q, y: &crate::core::Q| {
                let num = -q(2) * n;
                &num * &num / (q(4) * x * y)
            };
            assert_eq!(
                delta2(&before, &ra2, &rb2),
                delta2(&after, &ra2m, &rb2m),
                "δ² = num²/den2 is invariant under the motor, on the nose"
            );
        }
    }

    // AND THE SIGN CONVENTION THE WHOLE THING RESTS ON. `s² = +r² > 0`:
    // the nucleus is SPACELIKE, which is the dual-sphere chart where
    // contact is reachable. On the `v² = −1` hyperboloid `μ = sinh²(d/2)
    // ≥ 0` and tangency is unreachable — that is 59.2's finding, and a
    // statement that the nucleus has `s² = −r²` would put it back
    // there.
    for b in &bodies {
        let s = nucleus(b);
        let r2 = b.radius2();
        assert_eq!(s.geo(&s).scalar_part(), r2, "s² = +r², spacelike");
        assert!(r2 > q(0), "and every extracted body is invertible");
    }

    // A BODY AGAINST ITSELF IS THE IDENTITY MOTOR, and that is internal
    // tangency rather than "no relation": δ = −1, num = −2r², num² =
    // den2.
    let s = Strain::between(&bodies[0], &bodies[0]).expect("a centroid");
    assert_eq!(&s.num * &s.num, s.den2, "M_ab = 1 sits exactly on δ = −1");
}

/// A MOTOR OF NEGATIVE NORM DOES NOT FLIP THE CONTACT SIDE (ruled
/// 2026-09-22, and left open as an argument until measured).
///
/// `Cl(4,1)` is INDEFINITE — `e₋² = −1` — so a versor built from an odd
/// number of timelike reflections has `λ = M M̃ < 0`, which cannot
/// happen in a definite signature. 64.5 left it open whether the side
/// of the strain survives such a motor, because the sign of `num` is
/// what `Strain::side` splits on before squaring.
///
/// It does survive, and here is why: the sandwich `M x M̃` is `λ` times
/// an orthogonal map, so the inner product picks up `λ²` — and `λ² > 0`
/// for every `λ ∈ ℚ×`, negative ones included. The SIDE is therefore an
/// absolute geometric invariant even through an indefinite reflection
/// channel, while `num` itself remains gauge.
#[test]
fn a_negative_norm_motor_scales_the_strain_and_never_flips_its_side() {
    use crate::transducer::blade;
    use crate::transducer::cga::{self, Mv, E1, EM};
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    // `e₁e₋`: one spacelike and one timelike direction, so
    // `M M̃ = e₁ e₋ e₋ e₁ = e₁ (−1) e₁ = −1`.
    let motor = Mv::blade(E1, q(1)).geo(&Mv::blade(EM, q(1)));
    let lambda = motor.geo(&motor.reverse()).scalar_part();
    assert_eq!(lambda, q(-1), "an indefinite signature admits a NEGATIVE versor norm");
    assert!(lambda < q(0), "which a definite signature never could");

    let bodies = [
        blade::shape_of(&[(0, 0, 0), (1, 0, 0)]).blade,
        blade::shape_of(&[(0, 9, 0), (1, 9, 0)]).blade,
        blade::shape_of(&[(0, 1, 0)]).blade,
    ];
    let nucleus = |b: &blade::Blade| {
        let p = b.centroid().expect("a centroid");
        cga::sphere(&p.0, &p.1, &p.2, &b.radius2())
    };

    for a in &bodies {
        for b in &bodies {
            let (sa, sb) = (nucleus(a), nucleus(b));
            let before = sa.geo(&sb).scalar_part();
            let after = motor.sandwich(&sa).geo(&motor.sandwich(&sb)).scalar_part();
            assert_eq!(
                after,
                &lambda * &lambda * &before,
                "the inner product picks up λ², never λ"
            );
            // AND THAT IS THE WHOLE POINT: `λ² > 0` even for `λ < 0`,
            // so the sign the side is read from cannot invert.
            assert_eq!(
                before > q(0),
                after > q(0),
                "a negative-norm motor does not move the contact side"
            );
        }
    }
    assert!(&lambda * &lambda > q(0), "λ² is positive for every λ in ℚ×");
}

/// T4 — THE INTERFACE IS THE WHOLE PRODUCT, AND SCALAR COLLAPSE IS
/// DISALLOWED (64, ruled 2026-09-22).
///
/// Four proofs, each arithmetic rather than argued.
///
/// (1) THE SPLIT IS COMPLETE. `s_a s_b = s_a·s_b + s_a∧s_b` and there
///     is nothing else: the geometric product of two VECTORS has grades
///     0 and 2 only, so the interface loses nothing by being carried as
///     those two.
///
/// (2) A LONE TETHER IS ALWAYS SIMPLE. `(s_a∧s_b) ∧ (s_a∧s_b) = 0`
///     identically — a repeated factor kills the wedge — so `I₂ = 0` on
///     every isolated interface and a screw can only be a property of
///     the NETWORK (59.7, 60.4).
///
/// (3) THE BLADE IS THE COAXAL PENCIL. Every sphere `λs_a + μs_b` lies
///     in it: `s ∧ B = 0` for the whole one-parameter family, which is
///     what makes `B` the interface rather than a pair of endpoints.
///
/// (4) AND THE COLLAPSE IS REAL. Two pairs at the SAME `δ²` and the
///     same side carry DIFFERENT blades. That is the refutation: a
///     tether keyed on the scalar alone cannot tell two interactions
///     apart that differ in orientation, plane and pencil.
#[test]
fn the_interface_carries_what_the_scalar_throws_away() {
    use crate::transducer::blade::{self, Interface};
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    let a = blade::shape_of(&[(0, 0, 0)]).blade;
    let along_r = blade::shape_of(&[(4, 0, 0)]).blade;
    let along_c = blade::shape_of(&[(0, 4, 0)]).blade;

    let (na, nr) = (a.nucleus().expect("a"), along_r.nucleus().expect("r"));
    let product = na.geo(&nr);

    // (1) GRADES 0 AND 2 ONLY.
    for g in [1u32, 3, 4, 5] {
        assert!(
            product.grade(g).is_zero(),
            "the product of two vectors has no grade {g} part"
        );
    }
    let face = Interface::between(&a, &along_r).expect("both nuclei");
    assert_eq!(
        product.grade(0).scalar_part() * q(-2),
        face.strain.num,
        "the scalar part IS the strain numerator"
    );
    assert_eq!(product.grade(2), face.blade, "and the grade-2 part IS the interface");
    assert!(!face.blade.is_zero(), "which is not empty for two distinct bodies");

    // (2) SIMPLE, IDENTICALLY.
    assert!(
        crate::transducer::cga::bivector_is_simple(&face.blade),
        "a wedge of two vectors can never carry a screw: I₂ ≡ 0"
    );

    // (3) THE COAXAL PENCIL LIES IN THE BLADE. `s ∧ B = 0` for every
    //     `s = λ s_a + μ s_b`, which is what makes B the interface.
    for (l, m) in [(1i64, 0i64), (0, 1), (1, 1), (3, -2), (-5, 7)] {
        let s = na.scale(&q(l)).add(&nr.scale(&q(m)));
        // `s ∧ B` is the grade-3 part of `B s`: a vector wedged into a
        // 2-blade, which vanishes exactly when the vector lies in it.
        assert!(
            face.blade.wedge_vector(2, &s).is_zero(),
            "the pencil member ({l}, {m}) lies in the interface blade"
        );
    }

    // (4) THE COLLAPSE IS REAL: same δ², same side, DIFFERENT blade.
    let other = Interface::between(&a, &along_c).expect("both nuclei");
    let d2 = |f: &Interface| {
        &f.strain.num * &f.strain.num / f.strain.den2.clone()
    };
    assert_eq!(d2(&face), d2(&other), "two pairs at the same scale-free strain");
    assert_eq!(
        face.strain.side(),
        other.strain.side(),
        "and on the same side of contact"
    );
    assert_ne!(
        face.blade, other.blade,
        "yet different interfaces — the scalar cannot tell them apart"
    );
}

/// T4 — `condense` IS A FIELD, AND IT MINTS THE 1-SKELETON.
///
/// The horizon admits near pairs and refuses far ones, the reading does
/// not depend on the order the bodies were offered, and every tether
/// carries its interface blade rather than a bare link.
#[test]
fn condense_mints_tethers_from_the_strain_field() {
    use crate::transducer::blade::{self, condense};
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    // Three close together and one far away.
    let bodies: Vec<blade::Blade> = [
        vec![(0i64, 0i64, 0i64)],
        vec![(0, 1, 0)],
        vec![(1, 0, 0)],
        vec![(0, 60, 0)],
    ]
    .iter()
    .map(|cells| blade::shape_of(cells).blade)
    .collect();

    let near = condense(&bodies, &q(4));
    assert!(!near.is_empty(), "a horizon that admits nothing is not a field");
    assert!(
        near.iter().all(|&(i, j, _)| i != 3 && j != 3),
        "the distant body is outside every near horizon: {:?}",
        near.iter().map(|&(i, j, _)| (i, j)).collect::<Vec<_>>()
    );
    assert!(
        near.iter().all(|(_, _, f)| !f.blade.is_zero()),
        "every minted tether carries its interface, never a bare link"
    );

    // A WIDER HORIZON ADMITS STRICTLY MORE, so the gate is monotone and
    // is really reading the field rather than answering a constant.
    //
    // THE WIDE HORIZON IS DERIVED, NOT PICKED. A number chosen by hand
    // here is the `STROKE = 4` defect again — it was picked at 1000 and
    // the far pair needs 5399. The bound comes from the structural one
    // instead: `radius2 ≥ 1/6` for every body, so `den2 = 4r_a²r_b² ≥
    // 1/9` and `√den2 ≥ 1/3`, giving `δ = num/√den2 ≤ 3·num` and
    // therefore `μ = (δ−1)/2 < 2·num`. Twice the largest numerator in
    // the field admits every pair in it, whatever the fixture.
    let widest = {
        let mut m = q(0);
        for i in 0..bodies.len() {
            for j in i + 1..bodies.len() {
                if let Some(s) = blade::Strain::between(&bodies[i], &bodies[j]) {
                    if s.num > m {
                        m = s.num;
                    }
                }
            }
        }
        q(2) * m
    };
    let wide = condense(&bodies, &widest);
    assert!(wide.len() >= near.len(), "a wider horizon admits at least as many");
    assert_eq!(
        wide.len(),
        bodies.len() * (bodies.len() - 1) / 2,
        "and a horizon past the whole field admits every pair"
    );
    assert!(wide.len() > near.len(), "which is strictly more than the near one");

    // ORDER-FREE: the field is a property of the bodies, not of the
    // order they were handed over.
    let mut shuffled = bodies.clone();
    shuffled.reverse();
    assert_eq!(
        condense(&shuffled, &q(4)).len(),
        near.len(),
        "the same field however the bodies are offered"
    );
}

/// THE INVERSIVE SPECTRUM HAS FIVE REGIMES, NOT THREE (65, ruled
/// 2026-09-22).
///
/// ```text
///  Contained   TangentInt     Intersecting    TangentExt    Apart
///   δ < −1       δ = −1        |δ| < 1         δ = 1        δ > 1
/// ```
///
/// A first cut folded everything below `δ = 1` into one arm, so
/// INTERNAL TANGENCY — `d = |r_a − r_b|`, where the spheres touch at
/// one point from inside — was reported as interpenetration. Half the
/// contact boundary, mislabelled.
///
/// Each regime is exhibited on a fixture whose geometry is known, built
/// from bodies of DIFFERENT radii so that internal tangency is a real
/// configuration rather than the degenerate self-pair that hid the
/// defect.
#[test]
fn the_five_inversive_regimes_are_read_exactly() {
    use crate::transducer::blade::{self, Contact, Strain};
    let q = |n: i64| crate::core::Q::from_integer(n.into());

    // Radii are set by the bodies' own moments, so the fixtures are
    // built and then MEASURED rather than assumed: `side` is checked
    // against `δ²` and the sign of `num`, which is the definition.
    let small = blade::shape_of(&[(0, 0, 0)]).blade;
    let big = blade::shape_of(&[
        (0, 0, 0), (1, 0, 0), (2, 0, 0), (0, 1, 0), (1, 1, 0),
        (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0),
    ])
    .blade;

    // The classification is a function of `(sign num, δ² vs 1)`, and
    // this asserts the two agree on every pair the fixtures give.
    let check = |a: &blade::Blade, b: &blade::Blade| {
        let s = Strain::between(a, b).expect("centroids");
        let sq = &s.num * &s.num;
        let want = match (s.num > q(0), sq.cmp(&s.den2)) {
            (_, std::cmp::Ordering::Equal) if s.num > q(0) => Contact::TangentExternal,
            (_, std::cmp::Ordering::Equal) => Contact::TangentInternal,
            (true, std::cmp::Ordering::Greater) => Contact::Apart,
            (false, std::cmp::Ordering::Greater) => Contact::Contained,
            (_, std::cmp::Ordering::Less) => Contact::Intersecting,
        };
        assert_eq!(s.side(), want, "the spectrum is read off (sgn num, δ² vs 1)");
        s.side()
    };

    // A BODY AGAINST ITSELF IS INTERNAL TANGENCY, not penetration.
    // `num = −2r²` and `num² = den2`, so `δ = −1` exactly — and the
    // first cut called this `Through`.
    assert_eq!(
        check(&small, &small),
        Contact::TangentInternal,
        "δ = −1 is a CONTACT interface, not overlap"
    );
    assert_eq!(check(&big, &big), Contact::TangentInternal);

    // A SMALL BODY AT THE CENTRE OF A LARGE ONE IS CONTAINED. It must
    // be CONCENTRIC: at `d = 0`,
    //
    //     δ = −(r_a² + r_b²) / (2 r_a r_b)  ≤  −1
    //
    // by AM–GM, with equality exactly when `r_a = r_b`. Different radii
    // therefore put it strictly below −1. A first version of this put
    // the small body at `(0,0,0)` while the block's centroid is
    // `(1,1,0)` — a CORNER, not the centre — and it read
    // `Intersecting`, correctly.
    let centred = blade::shape_of(&[(1, 1, 0)]).blade;
    assert_eq!(check(&centred, &big), Contact::Contained, "nested, δ < −1");
    assert!(
        centred.radius2() != big.radius2(),
        "and the radii differ, so it is strictly nested rather than tangent"
    );

    // FAR APART.
    let far = blade::shape_of(&[(0, 200, 0)]).blade;
    assert_eq!(check(&small, &far), Contact::Apart, "disjoint, δ > 1");

    // AND THE BAND BETWEEN. Walking one body outward from the other
    // must pass through `Intersecting` before reaching `Apart`.
    let mut saw_secant = false;
    for d in 1..12i64 {
        let moved = blade::shape_of(&[(0, d, 0)]).blade;
        if check(&big, &moved) == Contact::Intersecting {
            saw_secant = true;
        }
    }
    assert!(saw_secant, "the secant band is reachable and is not an empty arm");
}

/// THE ANTIPODAL INVOLUTION `σ: s ↦ −s` IS AN INVOLUTION, AND IT
/// EXCHANGES THE CONTACT BOUNDARY (65).
///
/// A sphere and its negation are one sphere with inside and outside
/// swapped, so `δ ↦ −δ`: `Apart ↔ Contained`, the two tangencies
/// exchange, and `Intersecting` is fixed with `δ = 0` as its own fixed
/// point. `δ²` survives it; the SIDE does not, which is why a gate that
/// must be orientation-blind reads `δ²` and one that cares reads the
/// arm.
#[test]
fn the_antipodal_map_exchanges_the_contact_boundary_and_squares_to_one() {
    use crate::transducer::blade::Contact;
    let all = [
        Contact::Apart,
        Contact::TangentExternal,
        Contact::Intersecting,
        Contact::TangentInternal,
        Contact::Contained,
    ];
    for c in all {
        assert_eq!(c.antipodal().antipodal(), c, "σ∘σ = id");
    }
    assert_eq!(Contact::Apart.antipodal(), Contact::Contained);
    assert_eq!(Contact::TangentExternal.antipodal(), Contact::TangentInternal);
    assert_eq!(
        Contact::Intersecting.antipodal(),
        Contact::Intersecting,
        "the secant band is self-dual"
    );
    // BOTH TANGENCIES ARE CONTACT. Treating one as contact and the
    // other as penetration is what lost the boundary's symmetry.
    assert!(Contact::TangentExternal.is_tangent());
    assert!(Contact::TangentInternal.is_tangent());
    for c in [Contact::Apart, Contact::Intersecting, Contact::Contained] {
        assert!(!c.is_tangent(), "{c:?} is not a contact interface");
    }
    // The involution has no fixed points outside the secant band, so
    // it genuinely pairs the spectrum rather than permuting it.
    for c in [
        Contact::Apart,
        Contact::TangentExternal,
        Contact::TangentInternal,
        Contact::Contained,
    ] {
        assert_ne!(c.antipodal(), c, "{c:?} is exchanged, not fixed");
    }
}
