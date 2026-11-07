// MU-CLIENT §3.2, §4.2, §4.11 (#143) — THE ALGEBRA HELD PER CLASS in the
// mind's field: it grows only where a ring's generator lies outside the held
// span, and two minds align on the class, each reading it on its own planes.

use crate::transducer::cech::class_algebra::{class_of, ClassAlgebras};
use crate::transducer::cga::{Mv, E1, E2, E3};
use crate::transducer::mind::{with_field, MindField};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The (3, 4, 5) turn on the plane `mask`, as its rotor ray `2 + e_ab`.
fn turn(mask: usize) -> Mv {
    Mv::scalar(q(2)).add(&Mv::blade(mask, q(1)))
}

/// `Γ X Γ⁻¹`, with `Γ = 1 + e₂₃` (the quarter turn carrying `e₂` onto
/// `e₃`) and `Γ⁻¹ = (1 − e₂₃)/2`.
fn conjugate(x: &Mv) -> Mv {
    let g = Mv::scalar(q(1)).add(&Mv::blade(E2 | E3, q(1)));
    let g_inv = Mv::scalar(q(1)).sub(&Mv::blade(E2 | E3, q(1))).scale(&BigRational::new(1.into(), 2.into()));
    g.geo(x).geo(&g_inv)
}

/// THE ALGEBRA PER CLASS GROWS ONLY WHERE A GENERATOR LIES OUTSIDE THE HELD
/// SPAN. The (3, 4, 5) turns on `e₁₂`, `e₂₃` and `e₁₃` are one class. The
/// first ring heard opens the class's algebra. The same ring heard again
/// lies inside, and nothing is closed. A turn of the class on a plane sharing
/// an axis lies outside, and the class's algebra closes to `so(3)`. The
/// third plane, never heard, is then inside. What lies outside is read per
/// axis as its coset.
#[test]
fn class_algebra_grows_only_where_a_generator_lies_outside_the_held_span() {
    let class = class_of(&turn(E1 | E2)).expect("a rotor's class");
    assert_eq!(class_of(&turn(E2 | E3)), Some(class.clone()), "one turn on another plane: one class");
    let mut held = ClassAlgebras::new();
    let first = held.hear(&turn(E1 | E2), &mut stroke()).expect("heard");
    assert_eq!(first.class, class);
    assert_eq!(first.quotient.outside.len(), 1, "nothing held: the generator is outside");
    assert!(!first.quotient.outside[0].residue.is_empty(), "read by its coset's coordinate");
    assert_eq!(held.basis(&class).len(), 1);

    let again = held.hear(&turn(E1 | E2), &mut stroke()).expect("heard");
    assert!(again.quotient.spanned(), "the same ring lies inside");
    assert_eq!(held.basis(&class).len(), 1, "inside: the algebra does not grow");

    let across = held.hear(&turn(E2 | E3), &mut stroke()).expect("heard");
    assert_eq!(across.quotient.outside.len(), 1);
    assert_eq!(held.basis(&class).len(), 3, "[e₁₂, e₂₃] closes to so(3)");

    let third = held.hear(&turn(E1 | E3), &mut stroke()).expect("heard");
    assert!(third.quotient.spanned(), "e₁₃ lies in so(3) though never heard");
    assert_eq!(held.basis(&class).len(), 3);
    assert_eq!(held.classes(), 1, "one class heard");
}

/// TWO MINDS ALIGN ON THE CLASS, EACH READING IT ON ITS OWN PLANES. One mind
/// takes in a circuit `W`. Another takes in the same word in another frame,
/// `W′ = Γ W Γ⁻¹`. Both file it under one class, through the mind's own door
/// (`mark_flux`). Each holds the algebra on its own plane, and the two
/// bases differ. The conjugating element `Γ` carries the first mind's basis
/// into the second's span: it is the translation between the two
/// perspectives.
#[test]
fn two_minds_align_on_the_class_each_reading_it_on_its_own_planes() {
    let w = turn(E1 | E2);
    let w_other = conjugate(&w);
    let heard = |ring: &Mv| {
        let mut mind = MindField::default();
        mind.think(|| {
            let _ = crate::transducer::family::pairing::mark_flux(&[(ring.clone(), 1)], &[], &mut stroke());
            match with_field(|f| f.classes.clone()) {
                Ok(classes) => classes,
                Err(_) => panic!("the mind's field stands while it thinks"),
            }
        })
    };
    let (a, b) = (heard(&w), heard(&w_other));
    let class = class_of(&w).expect("a rotor's class");
    assert_eq!(class_of(&w_other), Some(class.clone()), "one class across frames");
    let (on_a, on_b) = (a.basis(&class), b.basis(&class));
    assert_eq!((on_a.len(), on_b.len()), (1, 1), "each mind holds the class's algebra");
    assert_ne!(on_a, on_b, "each on its own planes");
    let carried: Vec<Mv> = on_a.iter().map(conjugate).collect();
    let closed = crate::transducer::cech::lie_close(on_b, &carried, &mut stroke()).expect("closes");
    assert_eq!(closed.len(), on_b.len(), "Γ carries one basis into the other's span");
}
