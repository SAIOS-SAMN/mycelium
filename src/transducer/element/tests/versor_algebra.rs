// 16 — the versor algebra of the carrier: what a word spells, how two
// descriptions compose, and when a description has a solution.

use crate::core::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;
use num_traits::Zero;
use crate::transducer::germinate::germinate;
use super::common::*;

/// Congruence on the conformal orbit: translations, rotations,
/// reflections, and exact dilations of a support are carried by a
/// rational versor found in closed form; a genuinely different shape
/// refuses; the pullback reads a sibling's assignment onto the base at
/// any scale exactly.
#[test]
fn the_versor_identifies_conformal_images_and_pulls_back_sections() {
    let l = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2), (1, 0, 1, 2)];
    let shifted: Vec<_> = l.iter().map(|(r, c, b, a)| (r + 7, c + 3, *b, *a)).collect();
    let rotated = vec![(0i64, 0i64, 1u8, 2u8), (1, 0, 1, 2), (0, -1, 1, 2)];
    let bar = vec![(0i64, 0i64, 1u8, 2u8), (0, 1, 1, 2), (0, 2, 1, 2)];
    assert!(transition(&l, &shifted).is_some(), "translation is a versor");
    assert!(transition(&l, &rotated).is_some(), "rotation is a versor");
    assert!(transition(&l, &bar).is_none(), "distinct shapes refuse");
    // A 2× dilated L with other hues: the versor exists and the pullback
    // reads its assignment onto the base's three cells.
    let big: Vec<(i64, i64, u8, u8)> = l
        .iter()
        .flat_map(|&(r, c, _, _)| {
            [(2 * r, 2 * c), (2 * r, 2 * c + 1), (2 * r + 1, 2 * c), (2 * r + 1, 2 * c + 1)]
                .into_iter()
                .map(|(rr, cc)| (rr, cc, 5u8, 6u8))
        })
        .collect();
    let v = transition(&big, &l).expect("a dilation by 1/2 carries the big L onto the L");
    assert_eq!(pullback(&positions(&l), &big, &v), Some(vec![(5, 6); 3]));
    assert_ne!(positions(&l), positions(&shifted), "the raw lens sees only coordinates");
}

#[test]
fn compose_is_the_geometric_product() {
    let a = elem_from_versor(translator_versor(1, 0));
    let b = elem_from_versor(translator_versor(0, 2));
    let c = elem_from_versor(translator_versor(3, 0));
    let lhs = a.compose(&b).compose(&c);
    let rhs = a.compose(&b.compose(&c));
    assert!(
        same_versor(&lhs.versor().expect("lhs"), &rhs.versor().expect("rhs")),
        "compose is the geometric product, associative"
    );
    let ab = a.compose(&b).versor().expect("ab");
    let want = translator_versor(1, 0).geo(&translator_versor(0, 2));
    assert!(same_versor(&ab, &want));
}

#[test]
fn description_composed_with_its_solution_is_identity() {
    let g = elem_from_versor(translator_versor(3, -2));
    let sol = g.solution().expect("an invertible description has a solution");
    assert!(g.compose(&sol).is_identity(), "g ∘ g⁻¹ = e — the coboundary closes");
    assert!(sol.compose(&g).is_identity(), "g⁻¹ ∘ g = e");
}

#[test]
fn inverse_refuses_a_non_versor() {
    let mut e = Element::identity();
    e.action.spatial = Some(Versor::zero());
    assert!(e.solution().is_none(), "a non-versor has no inverse");
    assert!(e.inverse().is_none(), "a non-versor has no inverse");
}

#[test]
fn the_solution_inverts_the_described_motion() {
    let g = elem_from_versor(translator_versor(4, 4));
    let v = g.versor().expect("V");
    let inv = g.solution().expect("inverse").versor().expect("V⁻¹");
    let qn = |n: i64| Q::from(BigInt::from(n));
    let p = crate::transducer::cga::point(&qn(1), &qn(2), &qn(0));
    let moved = v.sandwich(&p);
    let back = inv.sandwich(&moved);
    assert_eq!(
        crate::transducer::cga::base(&back),
        crate::transducer::cga::base(&p),
        "g⁻¹ carries the image back to the origin point"
    );
}

#[test]
fn reverse_germ_is_the_inverse_chain() {
    let a = [(0, 0, 1u8), (0, 1, 2), (1, 0, 3)];
    let b = [(0, 0, 1u8), (0, 1, 9), (1, 0, 3)];
    let g = from_canonical(&germinate(&a, &b).unwrap());
    assert!(g.solution().is_none(), "a residual has no versor inverse");
    let inv = g.inverse().expect("the reverse chain exists");
    let want: Vec<_> = g.sub().iter().map(|&(r, c, b, a)| (r, c, a, b)).collect();
    assert_eq!(inv.sub(), want.as_slice(), "inverse swaps before/after on the chain");
    let again = inv.inverse().expect("reverse of reverse");
    assert_eq!(again.sub(), g.sub(), "inverse ∘ inverse is the chain");
}

#[test]
fn a5_chain_canonical_idempotent_and_translation_free() {
    let a = vec![(3, 4, 1u8, 2u8), (3, 5, 1, 2)];
    let b = vec![(0, 1, 1u8, 2u8), (0, 0, 1, 2)];
    let ca = canonical_chain(&a).unwrap();
    let cb = canonical_chain(&b).unwrap();
    assert_eq!(ca, cb);
    assert_eq!(canonical_chain(&ca).unwrap(), ca);
    assert_eq!(from_canonical(&ca).address(), from_canonical(&cb).address());
}

#[test]
fn carriage_holds_rho_once() {
    let two = Q::from(BigInt::from(2));
    let car = crate::transducer::cga::dilator(&two);
    let w = from_connection(&crate::transducer::alphabet::Word::identity(), &car, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells");
    let letters = w.letters().expect("letters");
    let d = dilator_of(&letters.carriage).expect("dilator");
    assert!(same_versor(&d, &car), "ρ lives in the carriage");
    let key = w.address();
    // form_key is M1 ‖ carriage ‖ σ ‖ chain: no second ρ beside the carriage.
    assert!(key.as_bytes().len() > 1);
}

/// A4, I12: A CARRIAGE `D(ρ) · T(t)` DECOMPOSES EXACTLY AT EVERY ρ. The
/// origin's image under the sandwich is `X(ρ t)`, so the translator read off
/// it alone was `T(ρ t)` and the "dilator" `V · T(ρ t)⁻¹` kept a translation
/// inside it: at ρ = 2 and t = (3, 5) the placement residue read (6, 10)
/// and no pure `D · T` passed `chart.rs`'s identity test on the dilator. ρ
/// is read off the versor's action on `e_∞`, `t` off the origin divided by
/// it; the two recompose to the carriage, and a reflection does not change
/// the scale it carries.
#[test]
fn a_carriage_decomposes_into_its_dilator_and_translator_at_every_rho() {
    use crate::transducer::cga;
    let q = |n: i64, d: i64| Q::new(BigInt::from(n), BigInt::from(d));
    for (rho, t1, t2) in [(q(1, 1), q(3, 1), q(5, 1)), (q(2, 1), q(3, 1), q(5, 1)), (q(1, 3), q(-2, 1), q(7, 2))] {
        let car = carriage(&rho, &(t1.clone(), t2.clone()));
        assert_eq!(rho_of(&car), Some(rho.clone()), "ρ is read off the action on e_∞");
        let t = translator_of(&car).expect("translator");
        assert!(
            same_versor(&t, &cga::translator(&t1, &t2, &q(0, 1))),
            "the translator is t, not ρ·t, at ρ = {rho}"
        );
        let d = dilator_of(&car).expect("dilator");
        assert!(same_versor(&d, &cga::dilator(&rho)), "the dilator is D(ρ) and carries no translation");
        assert!(same_versor(&d.geo(&t), &car), "and D · T is the carriage again");
    }
    // A rotor in the versor changes neither reading: it fixes the origin
    // and scales e_∞ by its own norm, which the ratio divides out.
    let rot = cga::rotator(&q(3, 1), &q(4, 1));
    let v = carriage(&q(2, 1), &(q(1, 1), q(0, 1))).geo(&rot);
    assert_eq!(rho_of(&v), Some(q(2, 1)), "ρ through a rotor");
    assert!(same_versor(&translator_of(&v).expect("t"), &cga::translator(&q(1, 1), &q(0, 1), &q(0, 1))));
    // A reflection is the orientation character, never a scale.
    let flipped = v.geo(&cga::reflector());
    assert_eq!(rho_of(&flipped), Some(q(2, 1)), "ρ through a reflection is ρ");
    // The identity carriage reads as no translation and no dilation.
    assert!(same_versor(&translator_of(&identity_versor()).expect("t"), &identity_versor()));
    assert!(same_versor(&dilator_of(&identity_versor()).expect("d"), &identity_versor()));
}

/// 51.2, A4 — THE BRIDGE IS EXACT ON THE BASE, BOTH WAYS. Every letter on a
/// base plane — `(0,1)`, `(0,3)`, `(1,3)` — acts on the base points as its
/// rotor acts on the base vectors, and the rotor is read back whole off the
/// versor, through a carriage as well as bare; the sheet, the orientation
/// character, acts as the reflection it is and reads back; a letter that
/// turns `e₂` into a base axis has no base action and is refused, never
/// projected.
#[test]
fn the_base_action_of_a_rotor_is_exact_and_its_inverse_reads_it_back() {
    use crate::transducer::alphabet::{LetterId, Rotor, Word, SLOT_PLANES_SIX};
    use crate::transducer::cga;
    let q = |n: i64| Q::from(BigInt::from(n));
    let unit = |k: usize| -> [Q; 4] { std::array::from_fn(|i| if i == k { q(1) } else { q(0) }) };
    let point_of = |x: &[Q; 4]| cga::point(&x[0], &x[1], &x[3]);
    let agrees = |rotor: &Rotor, v: &Versor| -> bool {
        [0usize, 1, 3].iter().all(|&k| {
            let Some(img) = rotor.carry(&unit(k)) else { return false };
            let Some((x1, x2, x3)) = cga::base(&v.sandwich(&point_of(&unit(k)))) else { return false };
            img[2].is_zero() && img[0] == x1 && img[1] == x2 && img[3] == x3
        })
    };
    let car = carriage(&q(2), &(q(3), q(5)));
    let mut on_base = 0usize;
    for &plane in &SLOT_PLANES_SIX {
        let letters = [LetterId::quarter(plane), LetterId::pythagorean_5(plane), LetterId::pythagorean_13(plane)];
        for id in letters.into_iter().flatten() {
            let rotor = Word::from_letters(&[(id, false)]).rotor();
            let touches_e2 = plane.0 == 2 || plane.1 == 2;
            match versor_of_word(&Word::from_letters(&[(id, false)])) {
                None => assert!(touches_e2, "a letter on a base plane has a base action: {plane:?}"),
                Some(v) => {
                    assert!(!touches_e2, "a letter turning e₂ into the base has no base action: {plane:?}");
                    on_base += 1;
                    assert!(agrees(&rotor, &v), "the versor acts as the rotor on the base: {plane:?}");
                    let back = base_rotor_of(&v).expect("the rotor reads back");
                    assert!(back.projective_eq(&rotor), "and is the rotor, whole: {plane:?}");
                    let placed = base_rotor_of(&car.geo(&v)).expect("through a carriage too");
                    assert!(placed.projective_eq(&rotor), "the carriage does not change the rotor read: {plane:?}");
                }
            }
        }
    }
    assert_eq!(on_base, 8, "three base planes: three quarters, three (3,4,5) rotors, two (5,12,13) rotors (51.6)");
    // The sheet: two quarters on (1,2), the half turn that flips e₁ and e₂,
    // which the base reads as the reflection of its column axis.
    let sheet = crate::transducer::alphabet::PlaneForm::sheet_word();
    let v = versor_of_word(&sheet).expect("the orientation character acts on the base");
    assert!(agrees(&sheet.rotor(), &v), "as the reflection it is on the base");
    assert!(base_rotor_of(&v).is_some_and(|r| r.projective_eq(&sheet.rotor())), "and reads back");
}
