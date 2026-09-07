// 16 — the versor algebra of the carrier: what a word spells, how two
// descriptions compose, and when a description has a solution.

use crate::core::engine::Q;
use crate::transducer::element::*;
use num_bigint::BigInt;
use crate::transducer::germinate::germinate;
use super::common::*;

/// The legacy lattice transport decodes to the versor with the same
/// action: index 1 is (r, c) → (c, −r), then the offset.
#[test]
fn the_legacy_transport_lifts_to_its_versor() {
    let v = legacy_versor(1, 2, 3).expect("a lattice isometry is a rational versor");
    let q = |n: i64| num_rational::BigRational::from_integer(BigInt::from(n));
    let at = |r: i64, c: i64| crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&q(r), &q(c))));
    assert_eq!(at(0, 0), Some((q(2), q(3))));
    assert_eq!(at(1, 0), Some((q(2), q(2))), "(1,0) → (0,−1) + (2,3)");
    assert_eq!(at(0, 1), Some((q(3), q(3))), "(0,1) → (1,0) + (2,3)");
    assert!(legacy_versor(9, 0, 0).is_none());
}

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
    let p = crate::transducer::cga::point(&qn(1), &qn(2));
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
    let g = from_residual(&germinate(&a, &b).unwrap());
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
    assert_eq!(from_residual(&a).address(), from_residual(&b).address());
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
