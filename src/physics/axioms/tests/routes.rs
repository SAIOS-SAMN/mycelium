// 79.1, 79.2, AX1 — a test per clause of the route axioms and theorems; the
// conjectures refused, never asserted.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::{self, Mv, E1, E2, E3, EM};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A step read backward: its reverse, a ray (78.32(1)) — `==` on `Mv` is
/// projective equality, so a route and its reversal compare as rays.
fn inv(g: &Mv) -> Mv {
    g.reverse()
}

/// A route's transport: its steps composed in order, `g_k ⋯ g₁`.
fn route(steps: &[&Mv]) -> Mv {
    steps.iter().fold(Mv::scalar(q(1, 1)), |acc, g| g.geo(&acc))
}

/// `H(r, s) = T(s)⁻¹ T(r)`.
fn loop_of(r: &Mv, s: &Mv) -> Mv {
    inv(s).geo(r)
}

fn one() -> Mv {
    Mv::scalar(q(1, 1))
}

/// Five steps that do not commute: a quarter turn, a translation, a boost, a
/// (3,4,5) turn on another plane, and a quarter turn about `(1, 2, 2)/3`.
fn steps() -> [Mv; 5] {
    let u = Mv::blade(E2 | E3, q(1, 3)).add(&Mv::blade(E1 | E3, q(-2, 3))).add(&Mv::blade(E1 | E2, q(2, 3)));
    [
        cga::rotator(&q(1, 1), &q(1, 1)),
        cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]),
        Mv::scalar(q(5, 1)).add(&Mv::blade(E1 | EM, q(4, 1))),
        cga::rotator_on_blade(&q(3, 1), &q(4, 1), E2 | E3),
        one().add(&u),
    ]
}

/// R1 — A ROUTE IS THE ORDERED PRODUCT OF ITS STEPS, THE REVERSED ROUTE ITS
/// INVERSE; transports are even, constructors odd with `v² ≠ 0`.
#[test]
fn r1_a_route_is_its_ordered_product_and_the_reverse_carries_the_inverse() {
    let [g1, g2, g3, ..] = steps();
    let t = route(&[&g1, &g2, &g3]);
    let back = route(&[&inv(&g3), &inv(&g2), &inv(&g1)]);
    assert_eq!(back.geo(&t), one(), "the reversed route undoes the route");
    assert_ne!(route(&[&g2, &g1, &g3]), t, "the order is the motion");
    for g in [&g1, &g2, &g3, &t] {
        assert!(g.grade(1).is_zero() && g.grade(3).is_zero() && g.grade(5).is_zero(), "a transport is even");
    }
    assert!(cga::is_constructor(&Mv::blade(E1, q(2, 1))), "a vector with v² ≠ 0 is a constructor");
    let null = Mv::blade(cga::EP, q(1, 1)).add(&Mv::blade(EM, q(1, 1)));
    assert!(!cga::is_constructor(&null), "a null vector reflects nothing");
    assert!(!cga::is_constructor(&g1), "an even element is no constructor");
}

/// R2 — CONSTRUCTORS BUILD: two reflections compose the tether `V = s_b s_a`,
/// and a generator carries a constructor to a constructor of the same
/// square.
#[test]
fn r2_constructors_build_generators_and_generators_carry_constructors() {
    let (a, b) = (Mv::blade(E1, q(1, 1)), Mv::blade(E1, q(1, 1)).add(&Mv::blade(E2, q(1, 1))));
    let v = cga::versor_of_reflections(&[b.clone(), a.clone()]).expect("two reflections compose a versor");
    assert_eq!(v, b.geo(&a), "the tether is the product of its constructors");
    for g in steps() {
        let w = cga::carry(&g, &a).expect("a generator carries a constructor");
        assert!(cga::is_constructor(&w), "to a constructor");
        assert_eq!(w.geo(&w).scalar_part(), a.geo(&a).scalar_part(), "of the same square");
    }
}

/// R3 — LOCALITY: a cell is the members standing at its witness, nothing
/// more; a member standing at another element is another cell's.
#[test]
fn r3_a_cell_is_the_members_standing_at_its_witness() {
    use crate::network::nerve::coord::Lamina;
    use crate::transducer::base::Site;
    let at = |x: i64| Lamina { site: Site::new(vec![x, 0, 0]) };
    let standing: Vec<(Lamina, Vec<usize>)> = vec![(at(0), vec![0, 1]), (at(2), vec![1, 2]), (at(4), vec![0, 1])];
    let cells = crate::transducer::cech::cells_over(
        standing.iter().map(|(l, m)| (l.clone(), m.as_slice())),
        |m| Some(vec![u8::try_from(m).expect("a member of the three-member standing list")]),
        |_| None,
    )
    .cells;
    let members: Vec<Vec<usize>> = cells.iter().map(|c| c.members.clone()).collect();
    assert_eq!(members, vec![vec![0, 1], vec![1, 2]], "two cells, each its own standing set");
    assert_eq!(cells[0].witnesses.len(), 2, "one cell standing at two elements has two witnesses");
}

/// R4 — CURVATURE IS THE BOUNDARY'S HOLONOMY, AND FLAT IS THE IDENTITY RAY:
/// two half turns on one plane compose to `−1`, which is flat; a boundary
/// that turns is not.
#[test]
fn r4_a_cell_is_flat_iff_its_boundary_is_the_identity_ray() {
    let half = Mv::blade(E1 | E2, q(1, 1));
    let boundary = route(&[&half, &half]);
    assert_eq!(boundary.scalar_part(), q(-1, 1), "the boundary composes to −1 exactly");
    assert_eq!(boundary, one(), "and −1 is the identity ray: flat");
    let [g1, ..] = steps();
    assert_ne!(route(&[&g1, &half]), one(), "a boundary that turns is curved");
}

/// R5 — BASING IS CONJUGATION BY THE TREE TRANSPORT: at the root it moves
/// nothing; elsewhere it moves the operator and keeps its class.
#[test]
fn r5_basing_conjugates_by_the_tree_transport() {
    let [g1, g2, g3, ..] = steps();
    let loop_t = route(&[&g1, &g2]);
    let based = |theta: &Mv| theta.geo(&loop_t).geo(&inv(theta));
    assert_eq!(based(&one()), loop_t, "at the root θ = 1");
    assert_ne!(based(&g3), loop_t, "elsewhere the operator moves");
    assert_eq!(based(&g3).class_invariants(), loop_t.class_invariants(), "and its class does not");
}

/// T1 — TWO ROUTES AGREE IFF THEIR LOOP IS THE IDENTITY RAY: a route and the
/// same motion factored through an extra step agree; a different motion
/// does not.
#[test]
fn t1_two_routes_agree_iff_their_loop_is_trivial() {
    let [g1, g2, g3, g4, _] = steps();
    let r = route(&[&g1, &g2]);
    let s = route(&[&g1, &g4, &inv(&g4), &g2]);
    assert_eq!(loop_of(&r, &s), one(), "the loop is trivial");
    assert_eq!(r, s, "so the routes agree");
    let t = route(&[&g1, &g3]);
    assert_ne!(loop_of(&r, &t), one(), "a loop that turns");
    assert_ne!(r, t, "so the routes disagree");
}

/// The square `a b c d` with its diagonal `a → c`: routes `r = a→b→c`,
/// `s = a→d→c`, cells `σ₁ = a→b→c→a` and `σ₂ = a→c→d→a`.
struct Square {
    r: Mv,
    s: Mv,
    f1: Mv,
    /// `σ₂` read at `c`, its own corner, and the lasso `γ = a → c` that
    /// carries it to `a`.
    f2_at_c: Mv,
    lasso: Mv,
}

fn square(ab: &Mv, bc: &Mv, ad: &Mv, dc: &Mv, ac: &Mv) -> Square {
    Square {
        r: route(&[ab, bc]),
        s: route(&[ad, dc]),
        f1: route(&[ab, bc, &inv(ac)]),
        f2_at_c: route(&[&inv(dc), &inv(ad), ac]),
        lasso: ac.clone(),
    }
}

/// T2 — DISCRETE NON-ABELIAN STOKES, WITH THE LASSO ORDER: the routes' loop
/// is the product of the two cells' curvatures, `σ₂`'s carried from its own
/// corner to the basepoint by its lasso, in the boundary's order — and not
/// in the other order, since the curvatures do not commute.
#[test]
fn t2_the_loop_is_the_lassoed_product_of_the_enclosed_curvatures() {
    let [g1, g2, g3, g4, g5] = steps();
    let sq = square(&g1, &g2, &g3, &g4, &g5);
    let f2 = inv(&sq.lasso).geo(&sq.f2_at_c).geo(&sq.lasso);
    let h = loop_of(&sq.r, &sq.s);
    assert_eq!(h, f2.geo(&sq.f1), "H(r, s) = (γ⁻¹ F₂ γ) · F₁, in the boundary's order");
    assert_ne!(h, sq.f1.geo(&f2), "the other order is another element: the cells do not commute");
    assert_ne!(sq.f1, one(), "and the cells are curved");
}

/// T2's CONVERSE FAILS ON A RELATOR: every edge flat but the diagonal, which
/// turns — both cells curved, their curvatures inverse, and the two routes
/// agree. Agreement never proves the enclosed cells flat.
#[test]
fn t2_converse_fails_on_a_relator() {
    let [.., g5] = steps();
    let sq = square(&one(), &one(), &one(), &one(), &g5);
    assert_eq!(loop_of(&sq.r, &sq.s), one(), "the routes agree");
    assert_ne!(sq.f1, one(), "while σ₁ is curved");
    assert_ne!(sq.f2_at_c, one(), "and σ₂ is curved");
    let f2 = inv(&sq.lasso).geo(&sq.f2_at_c).geo(&sq.lasso);
    assert_eq!(f2.geo(&sq.f1), one(), "their curvatures cancel: a relator");
}

/// C1 (a, b) — ONE REFERENCE GIVES EVERY PAIR, AND INDEPENDENCE IS EACH
/// READING TRIVIAL: `H(r₁, r₂) = H₂⁻¹ H₁` with no pairwise read.
#[test]
fn c1_one_reference_gives_every_pair() {
    let [g1, g2, g3, g4, g5] = steps();
    let r0 = route(&[&g1, &g2]);
    let (r1, r2) = (route(&[&g3, &g2]), route(&[&g4, &g5]));
    let (h1, h2) = (loop_of(&r1, &r0), loop_of(&r2, &r0));
    assert_eq!(loop_of(&r1, &r2), inv(&h2).geo(&h1), "every pair from the star");
    let same = route(&[&g1, &g4, &inv(&g4), &g2]);
    assert_eq!(loop_of(&same, &r0), one(), "a route agreeing with the reference reads trivial");
    assert_ne!(h1, one(), "and one that does not, does not: the content depends on the route");
}

/// C1 (c) — THE CAPACITY IS THE RANK OF THE ALGEBRA THE ACTIONS GENERATE:
/// the identity circuit generates the unit alone; one quarter turn
/// `ℚ[x]/((x − 1)(x² + 1))`, rank three; two quarter turns on crossing
/// planes the octahedral group's algebra, `M₃(ℚ) ⊕ ℚ`, rank ten, and they
/// are coupled.
#[test]
fn c1_capacity_is_the_rank_of_the_actions_algebra() {
    let read = |hs: Vec<Mv>| {
        use crate::network::nerve::coord::{ChartAt, Overlap, VertexAt};
        use crate::transducer::cech::CellCircuit;
        // Each circuit its own ring over its own overlap: the Frame's chart
        // to a member on chart `[i + 1]` (AT1: a face is an overlap).
        let overlap = |i: u64| Overlap {
            from: ChartAt::Frame,
            to: ChartAt::at(VertexAt::Frame, crate::transducer::stalk::Invariant::counts(&[i + 1]), 0),
        };
        let circuits: Vec<CellCircuit> = hs
            .into_iter()
            .zip(0u64..)
            .map(|(h, i)| CellCircuit { ring: CellCircuit::ring_of(&[(overlap(i), true)]), holonomy: Ok(h), hue: Some(Vec::new()) })
            .collect();
        crate::transducer::cech::read_cell(1, None, None, &circuits, &[], &mut stroke())
    };
    assert_eq!(read(vec![one()]).capacity, Ok(1), "the identity: the unit alone");
    let quarter = cga::rotator(&q(1, 1), &q(1, 1));
    let a = read(vec![quarter.clone()]);
    assert_eq!((a.capacity, a.coupled), (Ok(3), 0), "one quarter turn: rank three");
    let other = cga::rotator_on_blade(&q(1, 1), &q(1, 1), E2 | E3);
    let b = read(vec![quarter, other]);
    assert_eq!((b.capacity, b.coupled), (Ok(10), 1), "two crossing quarter turns: rank ten, coupled");
}

/// C1 (d) — THE SAME SHAPE INSIDE ONE GENERATOR: a motion's mode rays are
/// routes of their own, and they compose back to the motion.
#[test]
fn c1_the_mode_split_is_the_same_shape() {
    let [g1, g2, ..] = steps();
    let screw = g1.geo(&cga::translator_at(&[q(0, 1), q(0, 1), q(1, 1)]));
    let Some(Ok(modes)) = cga::motion_modes(&screw, &mut stroke()) else { panic!("the screw splits") };
    assert_eq!(modes.iter().fold(one(), |acc, m| acc.geo(&m.ray)), screw, "the rays compose to the motion");
    let _ = g2;
}

/// C2 (a) — CONJUGATION PRESERVES THE CLASS: minds basing one circuit at
/// their own points read one class.
#[test]
fn c2_based_readings_converge_on_the_class() {
    let [g1, g2, g3, g4, g5] = steps();
    let circuit = route(&[&g1, &g2, &g4]);
    let class = circuit.class_invariants();
    for theta in [g3, g5, route(&[&g2, &g4])] {
        let based = theta.geo(&circuit).geo(&inv(&theta));
        assert_eq!(based.class_invariants(), class, "(I₁, I₂, kind) survive the basing");
    }
}

/// C2 (b) — MANY CONSTRUCTOR CHAINS, ONE VERSOR: reflecting in `e₁` then
/// `e₁ + e₂`, and in `e₁ + e₂` then `e₂`, compose one quarter turn —
/// synonyms, one theorem with two proofs.
#[test]
fn c2_many_constructor_chains_compose_one_versor() {
    let (a, b, c) = (Mv::blade(E1, q(1, 1)), Mv::blade(E1, q(1, 1)).add(&Mv::blade(E2, q(1, 1))), Mv::blade(E2, q(1, 1)));
    let first = cga::versor_of_reflections(&[a, b.clone()]).expect("a versor");
    let second = cga::versor_of_reflections(&[b, c]).expect("a versor");
    assert_eq!(first, second, "two chains, one ray");
    assert_eq!(first, cga::rotator(&q(1, 1), &q(1, 1)), "the quarter turn");
}

/// CONJECTURE (79.2), REFUSED AS READ PER GENERATOR: two quarter turns whose
/// axes meet at `cos θ = 2/3` — each generator's every mode elliptic of
/// order four — generate no finite group, so the constructors' orbit does
/// not close: under a stroke it refuses as `Unbounded`, never `Closed`.
/// "Every mode elliptic of finite order" must be read on the group, not on
/// its generators, and stays a conjecture.
#[test]
fn conjecture_the_orbit_closure_is_refused_read_per_generator() {
    let [g1, .., g5] = steps();
    for g in [&g1, &g5] {
        let Some(Ok(ms)) = cga::motion_modes(g, &mut stroke()) else { panic!("a quarter turn splits") };
        assert!(ms.iter().all(|m| m.kind == BivectorKind::Elliptic && m.order == Some(4)), "every mode elliptic of order four");
    }
    // A small stroke: the orbit's rays grow in their coefficients at every
    // carrying, so the refusal is read early and cheaply.
    let orbit = cga::constructor_orbit(&[Mv::blade(E3, q(1, 1))], &[g1, g5], &mut crate::physics::Physics::default().stroke_n(256));
    assert!(matches!(orbit, cga::ConstructorOrbit::Unbounded { .. }), "the orbit does not close: {orbit:?}");
}

/// CONJECTURE (79.2), REFUSED: T1 READS RAYS AND IS BLIND TO THE SIGN. Two
/// routes whose transports differ by `−1` read agreeing, though the sign is
/// there to be read: the held `ℤ₂` is the coset's reading (78.6), not T1's.
#[test]
fn conjecture_the_sign_between_two_routes_is_not_read_by_t1() {
    let [g1, g2, ..] = steps();
    let half = Mv::blade(E1 | E2, q(1, 1));
    let r = route(&[&g1, &g2]);
    let s = route(&[&g1, &g2, &half, &half]);
    assert!(s.add(&r).is_zero(), "the transports differ by −1 exactly");
    assert_eq!(loop_of(&r, &s), one(), "and T1 reads them agreeing");
}

/// THE CONJECTURES ARE HELD OPEN IN THE REGISTER, each by the row that would
/// decide it; none is stated among the theorems.
#[test]
fn the_conjectures_are_held_open_by_name() {
    use crate::physics::axioms::{C1_RESOLUTION, C2_CONVERGENCE, ROUTE_CONJECTURES, T1_TWO_ROUTES, T2_DISCRETE_STOKES};
    let owed: Vec<&str> = ROUTE_CONJECTURES.iter().map(|(_, row)| *row).collect();
    assert!(owed.iter().all(|row| !row.is_empty()), "every conjecture names what would decide it");
    let theorems = [T1_TWO_ROUTES, T2_DISCRETE_STOKES, C1_RESOLUTION, C2_CONVERGENCE];
    for (c, _) in ROUTE_CONJECTURES {
        assert!(!theorems.contains(&c), "a conjecture is never a theorem's statement");
    }
}
