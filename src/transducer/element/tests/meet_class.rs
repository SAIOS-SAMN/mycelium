// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 12.6, 15.5 — WHAT A MEET'S TRANSITION IS, read on the versor.
//!
//! Every claim here is one the sensor-plane block could not make. The
//! block of a translation is the identity and the block of a dilation is a
//! scalar multiple of it, so a reader that classified Γ by its block alone
//! answered `identity` for both and filed nothing; and a rotation about
//! any point but the origin has a block that mis-orders it.

use crate::transducer::alphabet::{Order, PLANE_COL, PLANE_ROW};
use crate::transducer::cga;
use crate::transducer::element::{
    dilator_of, identity_versor, meet_class, rho_of, sensor_plane_block, translator_of, MeetClass,
};
use crate::transducer::sentence::CycleType;

fn q(n: i64) -> crate::core::Q {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(n))
}

/// A quarter turn about the origin, as a versor: the rotor `1 + e_rc`
/// normalised away — `cga::rotator(c, s)` takes the half-angle's cosine
/// and sine parts projectively, so `(1, 1)` is the eighth-turn rotor whose
/// square is the quarter turn.
fn quarter() -> crate::transducer::element::Versor {
    cga::rotator(&q(1), &q(1))
}

#[test]
fn the_identity_is_flat_and_so_is_a_scaled_identity() {
    assert!(matches!(meet_class(&identity_versor(), CycleType::Identity), Some(MeetClass::Flat)));
    // Versors act projectively on the cone (1.5), so a nonzero rational
    // multiple of the identity is the same transport and is also flat.
    let scaled = crate::transducer::cga::Mv::scalar(q(7));
    assert!(matches!(meet_class(&scaled, CycleType::Identity), Some(MeetClass::Flat)));
}

/// THE CLAIM THE BLOCK COULD NOT MAKE. A translation's sensor-plane block
/// is the identity, so the deleted `rotor_of_transition` read it as the
/// identity rotor and refused it as flat — measured on the corpus, where a
/// family carrying one shape to two separated copies produced exactly this
/// Γ and filed nothing while reporting no obstruction either.
#[test]
fn a_translation_stands_with_its_placement_and_is_not_flat() {
    let t = cga::translator(&q(3), &q(-2));
    let block = sensor_plane_block(&t).expect("a translator has a block");
    assert_eq!(block, [[q(1), q(0)], [q(0), q(1)]], "the block cannot see a translation at all");

    let Some(MeetClass::Standing { kappa }) = meet_class(&t, CycleType::Identity) else {
        panic!("a translation is a standing class, never flat");
    };
    // The placement residue is carried, which is what an arriving
    // generator of the conjugate class would have to match.
    assert!(
        !crate::transducer::element::same_versor(&kappa.carriage, &identity_versor()),
        "the class carries the placement the block subtracted away"
    );
}

/// A dilation is likewise invisible to the block: it survives there only
/// as the scalar a peel divides out, so the block reads a pure scale as a
/// rotation by nothing.
#[test]
fn a_dilation_stands_with_its_scale() {
    let d = cga::dilator(&q(3));
    assert_eq!(rho_of(&d), Some(q(3)), "the dilation is read off the action");
    let Some(MeetClass::Standing { kappa }) = meet_class(&d, CycleType::Identity) else {
        panic!("a dilation is a standing class");
    };
    assert_eq!(rho_of(&kappa.carriage), Some(q(3)), "the scale rides on the class's carriage");
}

/// A quarter turn about the ORIGIN has no residue, so it is the one thing
/// that files: a torsion class at its period.
#[test]
fn a_quarter_turn_about_the_origin_files_at_its_period() {
    let g = quarter();
    assert!(
        crate::transducer::element::same_versor(
            &translator_of(&g).expect("t"),
            &identity_versor()
        ),
        "a rotation about the origin has no placement residue"
    );
    assert!(
        crate::transducer::element::same_versor(&dilator_of(&g).expect("d"), &identity_versor()),
        "and no dilation residue"
    );
    let Some(MeetClass::Finite { ell, rotor }) = meet_class(&g, CycleType::Identity) else {
        panic!("a residue-free finite rotation files");
    };
    assert_eq!(ell, 4, "a quarter turn closes at four");
    assert_eq!(rotor.order(), Some(Order::Finite(4)), "and the filed rotor says so itself");
    assert!(
        crate::transducer::alphabet::rotor_block_on(&rotor, (PLANE_ROW, PLANE_COL)).is_some(),
        "the filed rotor acts on the sensor plane"
    );
}

/// A ROTATION ABOUT A POINT AWAY FROM THE ORIGIN is `T_p R T_p⁻¹ =
/// T(p − Rp)·R`. Its rotor part is `R` and its order is four, but it
/// carries a placement residue, so it does NOT file here: filing it at
/// period four would put a theorem on the form that the seam does not
/// satisfy, because the seam does not return after four turns — it walks.
///
/// This is precisely what the tree gauge removes. Once a cycle's sections
/// are gauge-fixed, the base-point translation is conjugated away and the
/// same seam reads `Finite { ell: 4 }`. Until then it stands, which is the
/// honest reading of an ungauged transition and not a missed fill.
#[test]
fn a_turn_about_a_point_carries_its_placement_and_does_not_file_ungauged() {
    let p = cga::translator(&q(5), &q(0));
    let g = p.geo(&quarter()).geo(&p.versor_inverse().expect("a translator inverts"));
    let t = translator_of(&g).expect("the placement residue is readable");
    assert!(
        !crate::transducer::element::same_versor(&t, &identity_versor()),
        "p − Rp is not zero for a quarter turn about (5, 0)"
    );
    assert!(
        matches!(meet_class(&g, CycleType::Identity), Some(MeetClass::Standing { .. })),
        "ungauged, a turn about a point stands; the tree gauge is what lets it file"
    );
}
