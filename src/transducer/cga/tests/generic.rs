// BQ3, 71.2(3) — THE CARRIER IS GENERIC IN n: the algebra at n = 4, with its
// own count of blades and planes, holding the n = 3 algebra inside it.
//
// The n = 3 values are proved by the oracle (`oracle.rs`), which now runs on
// this carrier against the golden file the fixed carrier wrote.

use crate::transducer::cga::{self, base_axis, blades, bivector_planes, pseudo, Mv, EM, EP};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// THE ALGEBRA AT n = 4: `2⁶` blades, fifteen planes, the signature's
/// metric, the new axis at its own bit, and the n = 3 algebra inside it.
#[test]
fn the_algebra_at_n4_has_its_own_count_and_holds_n3_inside_it() {
    assert_eq!(blades(4), 64, "Cl(5,1) has 2^6 blades");
    assert_eq!(bivector_planes(4).len(), 15, "C(5,2) + 5 = 15 planes");
    assert_eq!(bivector_planes(3).len(), 10, "and the n = 3 row of the family is ten");
    let e4 = base_axis(4);
    assert_eq!(e4, 1 << 5, "the fourth base axis takes bit 5, so e+ and e- never move");
    let one = |mask| Mv::blade(mask, BigRational::one());
    assert_eq!(one(e4).n(), 4, "a blade on the fourth axis belongs to n = 4");
    assert_eq!(one(e4).norm2(), q(1, 1), "a base axis squares to +1");
    assert_eq!(one(EP).norm2(), q(1, 1), "e+ squares to +1");
    assert_eq!(one(EM).norm2(), q(-1, 1), "e- squares to -1");
    assert_eq!(one(EP | EM).n(), 0, "e+ and e- belong to every algebra");
    // The pseudoscalar is grade n + 2, and invertible.
    let i4 = one(pseudo(4));
    assert_eq!((pseudo(4) as u32).count_ones(), 6, "grade n + 2");
    assert!(crate::transducer::cga::is_versor(&i4), "the pseudoscalar is a versor, so invertible");
    // THE INCLUSION: an n = 3 value is the same value at n = 4.
    let a = Mv::blade(0b00011, q(2, 1));
    let b = one(e4 | EM);
    let ab = a.geo(&b);
    assert_eq!(ab.n(), 4, "a product lives in the larger algebra");
    assert_eq!(ab.coeff(0b00011 | e4 | EM).clone(), q(2, 1), "e1e2 · e4e- is the grade-4 blade, exactly");
    assert!(a.widened(4).projective_eq(&a), "widening moves no value");
    // A plane through the fourth axis turns and one through e- boosts.
    let turn = one(base_axis(1) | e4);
    let boost = one(e4 | EM);
    assert!(turn.geo(&turn).scalar_part() < BigRational::zero(), "e1e4 squares negative: harmonic");
    assert!(boost.geo(&boost).scalar_part() > BigRational::zero(), "e4e- squares positive: luminal");
    // The generated names read every plane at n = 4, the new axis by its
    // own index.
    let names: Vec<String> = bivector_planes(4).into_iter().map(cga::plane_name).collect();
    assert!(names.contains(&"e14".to_string()) && names.contains(&"e4m".to_string()), "{names:?}");
    // And the cyclotomic candidates widen with n: φ(ℓ) ≤ 5 admits no new
    // order over φ(ℓ) ≤ 4 (φ is even past 2), and φ(ℓ) ≤ 6 admits 7 and 9.
    assert_eq!(cga::cyclotomic_orders(3), vec![1, 2, 3, 4, 5, 6, 8, 10, 12]);
    assert_eq!(cga::cyclotomic_orders(4), cga::cyclotomic_orders(3));
    assert!(cga::cyclotomic_orders(5).contains(&7) && cga::cyclotomic_orders(5).contains(&9));
    // Audit N-5 — THE ORDER IS AN LCM: at d = 6 the eigenvalues of orders 3
    // and 5 fit (φ = 2 + 4) and close at 15, though φ(15) = 8 > 6; orders 4
    // and 5 close at 20 the same way. Order 21 needs φ = 2 + 6 and does not
    // fit.
    let at5 = cga::cyclotomic_orders(5);
    assert!(at5.contains(&15) && at5.contains(&20), "{at5:?}");
    assert!(!at5.contains(&21), "{at5:?}");
}

/// PC1 — A TRANSLATION IS ONE PARABOLIC PLANE, NOT A TURN AND A BOOST. The
/// translator `1 − ½ d e_∞` carries `eᵢ ∧ e_∞`, which fills `eᵢe₊` and
/// `eᵢe₋` equally; epoch 97's census counted it on both and read every
/// slip as a turn on `e1p`. A boost on `e1m` alone is still a boost, and a
/// transversion `eᵢ ∧ e_o` is the other pole.
#[test]
fn a_translation_is_one_parabolic_plane_in_the_census() {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cga::Pole;
    let identity = Rotor::scalar(q(1, 1));
    let _ = cga::take_plane_tally();
    cga::note_transition(&identity, &cga::translator_at(&[q(2, 1), q(0, 1), q(0, 1)]));
    let t = cga::take_plane_tally();
    assert_eq!(t.count_parabolic(1, Pole::Infinity), 1, "one parabolic plane, e1∞");
    assert_eq!(t.count(base_axis(1) | EP), 0, "not a turn on e1p");
    assert_eq!(t.count(base_axis(1) | EM), 0, "not a boost on e1m");
    assert_eq!(t.occupied(), 1, "one plane in all: {}", t.line());
    // A pure boost `e1 ∧ e₋` is not null: it stays on its luminal plane.
    let _ = cga::take_plane_tally();
    let boost = Mv::scalar(q(2, 1)).add(&Mv::blade(base_axis(1) | EM, q(1, 1)));
    cga::note_transition(&identity, &boost);
    let t = cga::take_plane_tally();
    assert_eq!(t.count(base_axis(1) | EM), 1, "a boost is a boost");
    assert_eq!(t.count_parabolic(1, Pole::Infinity), 0);
    // `e1 ∧ e_o` — the other pole.
    let o = cga::occupancy(&Mv::blade(base_axis(1) | EM, q(1, 1)).sub(&Mv::blade(base_axis(1) | EP, q(1, 1))), 3);
    assert_eq!(o.parabolic, vec![(1, Pole::Origin)], "{o:?}");
    assert!(o.planes.is_empty(), "{o:?}");
}

/// 51.2, audit N-2 — THE CENSUS FILES A TURN UNDER THE PLANE IT TURNS. The
/// letters' `e0e3` is the base's (row, frame) plane, which the carriage
/// calls `e₁e₃`; `e1e3` is (column, frame), `e₂e₃`; the off-plane `e0e2`
/// lands on `e₁e₊`. A hand table filed these as `e₂e₃`, `e₂e₊` and `e₁e₃`.
#[test]
fn the_plane_census_reads_the_rotor_chart_through_the_alphabets_slots() {
    use crate::transducer::alphabet::{axis_of_slot, slot_of_axis, Rotor};
    for axis in 0..3 {
        let slot = slot_of_axis(axis).expect("the grid's three axes have slots");
        assert_eq!(axis_of_slot(slot), Some(axis), "a slot stands for its own axis");
    }
    assert_eq!(axis_of_slot(2), None, "e2 is the orientation character, no base axis");
    let cases: [(usize, usize); 3] = [
        (0b1001, base_axis(1) | base_axis(3)),
        (0b1010, base_axis(2) | base_axis(3)),
        (0b0101, base_axis(1) | EP),
    ];
    for (rotor_mask, plane) in cases {
        let _ = cga::take_plane_tally();
        let turn = Rotor::scalar(q(3, 5)).add(&Rotor::blade(rotor_mask, q(4, 5)));
        cga::note_transition(&turn, &Mv::scalar(BigRational::one()));
        let t = cga::take_plane_tally();
        assert_eq!(t.count(plane), 1, "rotor {rotor_mask:#06b} is filed as {}", cga::plane_name(plane));
        assert_eq!(t.occupied(), 1, "and under no other plane");
    }
}
