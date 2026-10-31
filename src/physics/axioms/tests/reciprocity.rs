// RX1–RX3 — the transposition of congruence, read on the engine's own
// transitions: every exact transition reads the same from both sides.

use crate::transducer::blade::sensor_plane as blade;
use crate::transducer::blade::{certify, reciprocal, transitions_between, Harmonic, Transition};
use crate::transducer::cga::{rotator, translator_at, LinearFunction, Mv};

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

fn coset(a: &blade::Shape, b: &blade::Shape) -> Vec<Transition> {
    transitions_between(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the pair is carried")
}

fn l() -> blade::Shape {
    blade::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)])
}

/// The L blown up twice: each compartment a 2×2 block.
fn l_blown_up() -> blade::Shape {
    blade::shape_of(&[
        (0, 0, 0), (0, 1, 0), (1, 0, 0), (1, 1, 0),
        (2, 0, 0), (2, 1, 0), (3, 0, 0), (3, 1, 0),
        (4, 0, 0), (4, 1, 0), (5, 0, 0), (5, 1, 0),
        (4, 2, 0), (4, 3, 0), (5, 2, 0), (5, 3, 0),
    ])
}

/// RX1's TRANSPOSITION `f̄`: `⟨f̄(x) y⟩₀ = ⟨x f(y)⟩₀` on every pair of
/// generators, and a versor's action is orthogonal, `f̄ ∘ f = 1`.
#[test]
fn adjoint_is_transposition_and_versor_action_orthogonal() {
    let m = translator_at(&[q(1, 2), q(-3, 1), q(2, 1)]).geo(&rotator(&q(2, 1), &q(1, 1)).widened(3));
    let f = LinearFunction::of_versor(&m).expect("a versor acts");
    let ft = f.adjoint();
    for &a in f.masks() {
        for &b in f.masks() {
            let (x, y) = (Mv::blade(a, q(1, 1)), Mv::blade(b, q(1, 1)));
            assert_eq!(ft.at(&x).geo(&y).scalar_part(), x.geo(&f.at(&y)).scalar_part(), "⟨f̄(x) y⟩ = ⟨x f(y)⟩");
        }
    }
    assert!(ft.after(&f).minus(&LinearFunction::identity(3)).is_zero(), "f̄ ∘ f = 1");
}

/// T1 — EVERY EXACT TRANSITION IS RECIPROCAL: each member of the coset from
/// `a` to `b` is read back from `b`'s side — its reverse a member of the coset
/// from `b` to `a`, one order, the residue transposed — for a turn, and for a
/// blow-up whose fit leaves a residue at order four.
#[test]
fn reciprocal_reads_every_transition_from_both_sides() {
    let turned = blade::shape_of(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (1, 0, 0)]);
    for (a, b) in [(l(), turned), (l(), l_blown_up())] {
        let (ab, ba) = (coset(&a, &b), coset(&b, &a));
        for t in &ab {
            let back = reciprocal(t, &ba, 3).expect("read the same from b's side");
            assert!(back.versor.projective_eq(&t.versor.reverse()), "V_BA ∝ Ṽ_AB");
        }
    }
}

/// T2 — THE FIT ORDER IS SYMMETRIC, and the blow-up fits through three both
/// ways, its residue at four transposed.
#[test]
fn certify_fit_is_symmetric_in_order() {
    let (a, b) = (l(), l_blown_up());
    let (ab, ba) = (
        certify(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE).expect("fit"),
        certify(&b.blade, &a.blade, crate::transducer::base::GRID_PLANE).expect("fit"),
    );
    assert_eq!(ab.through, Harmonic::Three);
    assert_eq!(ab.through, ba.through, "m_AB = m_BA");
    let [t] = coset(&a, &b).try_into().expect("one member");
    let ba = coset(&b, &a);
    let back = reciprocal(&t, &ba, 3).expect("reciprocal");
    let on_grid = |x: &Transition| -> Vec<[num_rational::BigRational; 2]> {
        x.residue
            .iter()
            .filter(|r| r.order == Harmonic::Four && r.plane == crate::transducer::base::GRID_PLANE)
            .map(|r| r.vector.clone())
            .collect()
    };
    assert_eq!(on_grid(&t), vec![[q(-1, 4), q(0, 1)]], "the blow-up's order-four residue, S₄(2)/2² = −1/4");
    // Read back at the scale 1/2: −Ad⁽⁴⁾(r) = −(1/2)⁴ · (−1/4) = 1/64.
    assert_eq!(on_grid(back), vec![[q(1, 64), q(0, 1)]], "transposed onto the L's side");
}
