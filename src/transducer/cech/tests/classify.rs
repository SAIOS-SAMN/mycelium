// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6, 15.5 — WHAT A TRANSITION IS, read from its rotor and its
//! carriage by the ONE classifier both organs use (L23b).
//!
//! Every one is a claim the sensor-plane block alone could not make: the
//! block of a translation is the identity and the block of a dilation is a
//! scalar multiple of it, so a reader that classified by block answered
//! `identity` for both and filed nothing.

use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::{classify, Fill};
use crate::transducer::cga;
use crate::transducer::element::{identity_versor, rho_of, same_versor, Versor};
use crate::transducer::sentence::CycleType;

fn q(n: i64) -> crate::core::Q {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(n))
}

fn ident() -> Versor {
    identity_versor()
}

/// The quarter turn on the sensor plane, as a rotor of order four.
fn quarter() -> Rotor {
    let id = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("the sensor plane");
    crate::transducer::alphabet::Word::from_letters(&[(id, false)]).rotor()
}

#[test]
fn the_identity_is_flat_and_so_is_a_scaled_carriage() {
    assert!(
        matches!(classify(&Rotor::identity(), &ident(), CycleType::Identity), Fill::Filled(..)),
        "no rotation and no residue is flat"
    );
    // Versors act projectively on the cone (1.5), so a nonzero rational
    // multiple of the identity carriage is the same carriage.
    let scaled = cga::Mv::scalar(q(7));
    assert!(
        matches!(classify(&Rotor::identity(), &scaled, CycleType::Identity), Fill::Filled(..)),
        "a scaled identity carriage is the identity carriage"
    );
}

/// THE CLAIM THE BLOCK COULD NOT MAKE. A translation's sensor-plane block
/// is the identity, so a reader that classifies by block reads it as the
/// identity rotor and calls the seam flat; a family carrying one shape to
/// two separated copies produces exactly this transition.
#[test]
fn a_translation_stands_with_its_placement_and_is_not_flat() {
    let t = cga::translator(&q(3), &q(-2), &q(0));
    assert!(
        crate::transducer::element::base_rotor_of(&t).is_some_and(|r| r.is_identity()),
        "the rotor part of a translation is the identity: the placement is the carriage's, not the rotor's"
    );
    let Fill::Class(kappa, r) = classify(&Rotor::identity(), &t, CycleType::Identity) else {
        panic!("a translation is a standing class, never flat");
    };
    assert!(r.is_identity(), "its rotor part is the identity");
    assert!(
        !same_versor(&kappa.carriage, &ident()),
        "and the class carries the placement the block subtracted away"
    );
}

/// A dilation is likewise invisible to the block: it survives there only
/// as the scalar a peel divides out, so the block reads a pure scale as a
/// rotation by nothing.
#[test]
fn a_dilation_stands_with_its_scale() {
    let d = cga::dilator(&q(3));
    assert_eq!(rho_of(&d), Some(q(3)), "the dilation is read off the action");
    let Fill::Class(kappa, r) = classify(&Rotor::identity(), &d, CycleType::Identity) else {
        panic!("a dilation is a standing class");
    };
    assert!(r.is_identity(), "its rotor part is the identity");
    assert_eq!(rho_of(&kappa.carriage), Some(q(3)), "the scale rides on the class's carriage");
}

/// A quarter turn with NO residue is the one thing that files: a torsion
/// class at its own period.
#[test]
fn a_residue_free_quarter_turn_files_at_its_period() {
    let Fill::Congruent(g, ell) = classify(&quarter(), &ident(), CycleType::Identity) else {
        panic!("a residue-free finite rotation files");
    };
    assert_eq!(ell, 4, "a quarter turn closes at four");
    assert_eq!(g.order(), Some(Order::Finite(4)), "and the filed rotor says so itself");
    assert!(g.projective_eq(&quarter()), "and it is the rotor that caused it");
}

/// THE SAME ROTOR CARRIED ALONG A TRANSLATION DOES NOT FILE. A rotation
/// about a point away from the origin is `T_p R T_p⁻¹ = T(p − Rp)·R`: its
/// rotor part is `R` and its order is four, but the seam does not return
/// after four turns — it walks — so filing it at period four would put a
/// theorem on the simplex that the seam does not satisfy.
///
/// The tree gauge of `solve_gauge` is what conjugates the base-point
/// translation away; ungauged it stands, which is the honest reading of a
/// transition with no frame and not a missed fill.
#[test]
fn a_turn_carried_along_a_placement_stands_rather_than_filing() {
    let placed = cga::translator(&q(5), &q(0), &q(0));
    let Fill::Class(kappa, r) = classify(&quarter(), &placed, CycleType::Identity) else {
        panic!("a turn with a placement residue stands; it does not file at four");
    };
    assert_eq!(
        r.order(),
        Some(Order::Finite(4)),
        "the rotor part still has order four — it is the RESIDUE that refuses the fill"
    );
    assert!(
        !same_versor(&kappa.carriage, &ident()),
        "and the class carries the placement that refused it"
    );
}

/// A rotor the seventeen do not spell is 37.7's own arm, not a class of
/// the identity and not flat. `π₁₇` as a rotor: its arc is `15 + 8i`,
/// whose norm is `17²`, and no product of the alphabet's primes reaches it.
#[test]
fn a_rotor_the_alphabet_cannot_spell_is_its_own_arm() {
    let pi17 = Rotor::scalar(q(4)).add(&Rotor::blade(0b0011, q(1)));
    assert!(
        crate::transducer::alphabet::word_of_rotor(&pi17).is_none(),
        "the seventeen do not spell π₁₇"
    );
    let Fill::Unspellable(r) = classify(&pi17, &ident(), CycleType::Identity) else {
        panic!("an unspellable rotor is 37.7's arm, not a class and not a fill");
    };
    assert!(r.projective_eq(&pi17), "and it names the rotor that could not be spelled");
}

/// THE COVER'S OWN PATH. `classify` tells a placed transition from a flat
/// one; the cover reaches it through `extend`, and `extend` is only ever
/// as informative as the two sections it is handed.
///
/// `Section::at` sets the carriage to the identity by construction. With
/// both sections built that way, `extend`'s composed carriage is the
/// identity for every pair of charts, so `classify`'s `flat` is true by
/// construction and `Fill::Class` is unreachable on that path — not
/// nothing curved found, but nothing curved REPRESENTABLE.
///
/// The two halves of this test are the same two charts. One pair of
/// sections carries the placements the charts actually hold and stands; the
/// other is the same pair through `Section::at` and fills.
#[test]
fn sections_at_different_placements_stand_where_the_identity_carriage_fills() {
    use crate::transducer::address::Address;
    use crate::transducer::cech::{extend, Section, Simplex};
    let addr_a = Address::from_bytes([1u8, 1]);
    let addr_b = Address::from_bytes([1u8, 2]);
    let edge = Simplex {
        vertices: vec![addr_a.as_bytes().to_vec(), addr_b.as_bytes().to_vec()],
        coefficient: Rotor::identity(),
        carriage: ident(),
        volume: [0; 4],
        fill: Fill::Open,
    };
    assert_eq!(edge.degree(), 1, "an edge, so `extend` takes the simplex's own arm");

    // The charts as they are: one at the origin, one placed at (3, −2).
    let placed = Section {
        addr: addr_b.clone(),
        gamma: Rotor::identity(),
        carriage: cga::translator(&q(3), &q(-2), &q(0)),
    };
    let origin = Section { addr: addr_a.clone(), gamma: Rotor::identity(), carriage: ident() };
    let Fill::Class(kappa, _) = extend(&origin, &placed, &edge) else {
        panic!("two charts at different placements do not glue: that is the obstruction");
    };
    assert!(
        !same_versor(&kappa.carriage, &ident()),
        "and the standing class carries the placement between them"
    );

    // The same two charts through the constructor the cover used. The
    // placement is not lost in `extend` — it never arrives.
    let flattened_b = Section::at(addr_b, Rotor::identity());
    let flattened_a = Section::at(addr_a, Rotor::identity());
    assert!(
        matches!(extend(&flattened_a, &flattened_b, &edge), Fill::Filled(..)),
        "with the carriage dropped at construction the same two charts read as glued"
    );
}
