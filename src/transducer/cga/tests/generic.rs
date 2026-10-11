// BQ3, 71.2(3) — THE CARRIER IS GENERIC IN n: the algebra at n = 4, with its
// own count of blades and planes, holding the n = 3 algebra inside it.
//
// The n = 3 values are proved by the oracle (`oracle.rs`), which now runs on
// this carrier against the golden file the fixed carrier wrote.

use crate::transducer::cga::{self, base_axis, blades, bivector_planes, pseudo, slots, Mv, EM, EP, MAX_N};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn ax(i: usize) -> usize {
    base_axis(i).expect("a fixture's axis within the algebra")
}

/// THE ALGEBRA'S BOUND IS NAMED, NEVER WRAPPED: an axis, a pseudoscalar, a
/// count of blades or slots past [`MAX_N`] is refused, where a shift past the
/// machine word once wrapped into another axis's bit in a release build.
#[test]
fn past_the_algebras_masks_nothing_is_made_and_nothing_wraps() {
    let last = usize::from(MAX_N);
    assert!(base_axis(last).is_some(), "the last axis the masks hold");
    assert_eq!(base_axis(last + 1), None, "one past it is refused");
    assert_eq!(base_axis(0), None, "there is no axis zero");
    assert_eq!(pseudo(MAX_N + 1), None);
    assert_eq!(blades(MAX_N + 1), None);
    assert_eq!(slots(MAX_N + 1), None);
    assert_eq!(slots(MAX_N), Some(1usize << (usize::from(MAX_N) + 2)));
    // The axes of every n are the masks base_axis makes, each once.
    let masks: Vec<usize> = cga::axes(MAX_N).collect();
    assert_eq!(masks.len(), last);
    assert!(masks.windows(2).all(|w| w[0] < w[1]), "distinct and ascending");
}

/// THE CARRIER HOLDS ONLY WHAT THE VALUE HAS: a blade of a large algebra is
/// one term, and the product reads the terms the two hold — `e₁e₄₀ · e₄₀ =
/// e₁` exactly, at an `n` whose dense carrier would have held 2⁴² slots.
#[test]
fn a_blade_of_a_large_algebra_is_one_term_and_multiplies_exactly() {
    let e40 = Mv::blade(ax(40), q(1, 1));
    assert_eq!((e40.n(), e40.term_count()), (40, 1));
    let e1_40 = Mv::blade(ax(1) | ax(40), q(3, 2));
    let back = e1_40.geo(&e40);
    assert_eq!(back.term_count(), 1);
    assert_eq!(back.coeff(ax(1)).clone(), q(3, 2), "e₁e₄₀ e₄₀ = e₁ — the axis squares to +1");
    assert!(back.narrowed().n() == 1, "and it narrows to the algebra it lives in");
    // Adding a value to its negative is the zero, holding no term.
    assert!(e40.sub(&e40).is_zero());
    assert_eq!(e40.sub(&e40).term_count(), 0);
}

/// THE WIRE'S `n` IS A CLAIM, NEVER AN ALLOCATION: a frame naming an
/// algebra of 2⁶² slots in a handful of bytes is refused before anything is
/// sized by it, and an `n` past the masks is refused outright — one forged
/// byte took a node's memory before. A value still round-trips whole.
#[test]
fn a_forged_n_on_the_wire_is_refused_before_anything_is_sized_by_it() {
    let forged = [60u8, 1, 1, 1, 1, 1];
    assert!(Mv::from_bytes(&forged, &mut 0usize).is_none(), "2^62 coefficients in six bytes: refused");
    let past = [MAX_N + 1, 0, 0, 0];
    assert!(Mv::from_bytes(&past, &mut 0usize).is_none(), "an n past the masks: refused");
    let v = Mv::blade(ax(1) | ax(4), q(-7, 3)).add(&Mv::scalar(q(2, 1)));
    let mut b = Vec::new();
    v.to_bytes(&mut b);
    let mut p = 0usize;
    let back = Mv::from_bytes(&b, &mut p).expect("it reads");
    assert_eq!(p, b.len());
    assert!(back.projective_eq(&v) && back.coeff(ax(1) | ax(4)) == v.coeff(ax(1) | ax(4)));
}

/// THE ALGEBRA AT n = 4: `2⁶` blades, fifteen planes, the signature's
/// metric, the new axis at its own bit, and the n = 3 algebra inside it.
#[test]
fn the_algebra_at_n4_has_its_own_count_and_holds_n3_inside_it() {
    assert_eq!(blades(4), Some(64), "Cl(5,1) has 2^6 blades");
    assert_eq!(bivector_planes(4).len(), 15, "C(5,2) + 5 = 15 planes");
    assert_eq!(bivector_planes(3).len(), 10, "and the n = 3 row of the family is ten");
    let e4 = ax(4);
    assert_eq!(e4, 1 << 5, "the fourth base axis takes bit 5, so e+ and e- never move");
    let one = |mask| Mv::blade(mask, BigRational::one());
    assert_eq!(one(e4).n(), 4, "a blade on the fourth axis belongs to n = 4");
    assert_eq!(one(e4).norm2(), q(1, 1), "a base axis squares to +1");
    assert_eq!(one(EP).norm2(), q(1, 1), "e+ squares to +1");
    assert_eq!(one(EM).norm2(), q(-1, 1), "e- squares to -1");
    assert_eq!(one(EP | EM).n(), 0, "e+ and e- belong to every algebra");
    // The pseudoscalar is grade n + 2, and invertible.
    let p4 = pseudo(4).expect("the pseudoscalar of Cl(5,1)");
    let i4 = one(p4);
    assert_eq!(p4.count_ones(), 6, "grade n + 2");
    assert!(crate::transducer::cga::is_versor(&i4), "the pseudoscalar is a versor, so invertible");
    // THE INCLUSION: an n = 3 value is the same value at n = 4.
    let a = Mv::blade(0b00011, q(2, 1));
    let b = one(e4 | EM);
    let ab = a.geo(&b);
    assert_eq!(ab.n(), 4, "a product lives in the larger algebra");
    assert_eq!(ab.coeff(0b00011 | e4 | EM).clone(), q(2, 1), "e1e2 · e4e- is the grade-4 blade, exactly");
    assert!(a.widened(4).projective_eq(&a), "widening moves no value");
    // A plane through the fourth axis turns and one through e- boosts.
    let turn = one(ax(1) | e4);
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
    use crate::transducer::cga::Pole;
    let slide = cga::translator_at(&[q(2, 1), q(0, 1), q(0, 1)]);
    let t = cga::occupancy(&slide, slide.n());
    assert_eq!(t.parabolic, vec![(1, Pole::Infinity)], "one parabolic plane, e1∞");
    assert!(t.planes.is_empty(), "not a turn on e1p, not a boost on e1m: {t:?}");
    // A pure boost `e1 ∧ e₋` is not null: it stays on its luminal plane.
    let boost = Mv::scalar(q(2, 1)).add(&Mv::blade(ax(1) | EM, q(1, 1)));
    let t = cga::occupancy(&boost, boost.n());
    assert_eq!(t.planes, vec![ax(1) | EM], "a boost is a boost");
    assert!(t.parabolic.is_empty());
    // `e1 ∧ e_o` — the other pole.
    let o = cga::occupancy(&Mv::blade(ax(1) | EM, q(1, 1)).sub(&Mv::blade(ax(1) | EP, q(1, 1))), 3);
    assert_eq!(o.parabolic, vec![(1, Pole::Origin)], "{o:?}");
    assert!(o.planes.is_empty(), "{o:?}");
}

/// 51.2 — EVERY GRID AXIS HAS ITS SLOT IN THE LETTERS' CHART, and the
/// orientation character `e2` stands for no base axis.
#[test]
fn the_alphabets_slots_stand_for_the_grids_axes() {
    use crate::transducer::alphabet::{axis_of_slot, slot_of_axis};
    for axis in 0..3 {
        let slot = slot_of_axis(axis).expect("the grid's three axes have slots");
        assert_eq!(axis_of_slot(slot), Some(axis), "a slot stands for its own axis");
    }
    assert_eq!(axis_of_slot(2), None, "e2 is the orientation character, no base axis");
}
