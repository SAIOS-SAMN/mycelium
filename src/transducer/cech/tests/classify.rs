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
    let t = cga::at_three::translator(&q(3), &q(-2), &q(0));
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
    let Fill::Congruent(kappa, g, ell) = classify(&quarter(), &ident(), CycleType::Identity) else {
        panic!("a residue-free finite rotation files");
    };
    assert!(
        !kappa.coordinates().is_empty(),
        "and it files WITH its class vector: 11.3 holds the orbit with it, and 3.9 reads \
         the candidate pivots off the axes it occupies — a period alone has none"
    );
    assert_eq!(ell, 4, "a quarter turn closes at four");
    assert_eq!(g.order(), Some(Order::Finite(4)), "and the filed rotor says so itself");
    assert!(g.projective_eq(&quarter()), "and it is the rotor that caused it");
}

/// A TURN ABOUT A POINT FILES; A SCREW STANDS. Both carry the same rotor
/// of order four and both carry a translation, so a reader that asked only
/// whether the carriage was flat filed neither — which is what it did, and
/// why no body that turned about its own centroid ever reached a period.
///
/// They are told apart by solving `(I − R) p = t` exactly. The first is
/// `T_p R T_p⁻¹` for a rational `p` this test recovers and checks: it
/// returns after four turns, about that point, so its period is four and
/// `p` is where it stands rather than a residue. The second translates
/// ALONG the turn's axis, and no point is fixed: `(I − R) p` is zero on
/// `e₃` for every `p`, the system is inconsistent, and the seam's orbit
/// really never closes.
#[test]
fn a_turn_about_a_point_files_and_a_screw_stands() {
    use crate::transducer::element::turn_base_point;
    let about_a_point = cga::at_three::translator(&q(5), &q(0), &q(0));
    let p = turn_base_point(&quarter(), &about_a_point).expect("a quarter turn has a base point");
    let Fill::Congruent(kappa, g, ell) =
        classify(&quarter(), &about_a_point, CycleType::Identity)
    else {
        panic!("a turn about a point files at its period");
    };
    assert_eq!(ell, 4, "a quarter turn about a point closes at four, wherever the point is");
    // The base point is not lost by filing: the class vector carries the
    // carriage `T(p − Rp)`, so `p` is re-solved from what was filed.
    assert!(
        crate::transducer::element::turn_base_point(&g, &kappa.carriage).is_some(),
        "and p is recoverable from the filed class, not discarded at the gate"
    );
    // AND THE PLACEMENT AXIS DOES NOT OCCUPY. This assertion stood the
    // other way up and asserted the defect: a base point is WHERE THE TURN
    // STANDS, so 3.9's translation word would move the frame and discharge
    // nothing. `classify` filing this orbit `Congruent` while `occupies`
    // called its carriage an obstruction was two readers of one carriage
    // disagreeing (law 12).
    assert!(
        !kappa.occupies(crate::transducer::sentence::Axis::Placement),
        "a turn's base point is a choice of origin, not a coordinate to discharge"
    );
    assert_eq!(g.order(), Some(Order::Finite(4)), "and the filed rotor says so itself");
    // `p − R p` is the translation the carriage holds: the point is solved
    // from the turn, not measured against it. The base's three axes sit at
    // `e₀, e₁, e₃` of the rotor chart (51.2), so `e₂` carries nothing.
    let embedded = [p[0].clone(), p[1].clone(), q(0), p[2].clone()];
    let turned = quarter().carry(&embedded).expect("the quarter turn carries the base");
    assert_eq!(&embedded[0] - &turned[0], q(5), "r: p − Rp is the carriage's own translation");
    assert_eq!(&embedded[1] - &turned[1], q(0), "c: and nothing is left over");
    assert_eq!(&embedded[3] - &turned[3], q(0), "k: the axis is untouched");

    let screw = cga::at_three::translator(&q(0), &q(0), &q(5));
    assert!(
        turn_base_point(&quarter(), &screw).is_none(),
        "a translation along the axis fixes no point: the system is inconsistent"
    );
    let Fill::Class(kappa, r) = classify(&quarter(), &screw, CycleType::Identity) else {
        panic!("a screw stands; it does not file at four");
    };
    assert_eq!(
        r.order(),
        Some(Order::Finite(4)),
        "its rotor part still has order four — it is the SLIP that refuses the fill"
    );
    assert!(
        !same_versor(&kappa.carriage, &ident()),
        "and the class carries the placement that refused it"
    );

    // 54.4 — AND THE SLIP IS THE COORDINATE, not the word "no".
    //
    // `turn_base_point` answering `None` was the whole of what a screw used
    // to say, so it filed as an undifferentiated free generator and how far
    // it slips per circuit was discarded — a measured vector thrown away
    // (law 7), and a vector written down as a status (rule 4).
    //
    // `screw_of` splits the transport exactly: the CENTRE on `Im(I − R)`,
    // where the bend holds a point still, and the TWIST on `ker(I − R)`,
    // the plane the rotor leaves pointwise fixed, along which no choice of
    // origin absorbs the translation. A screw is curvature on the first
    // tensored with torsion on the second, and both are returned.
    //
    // This screw turns a quarter on `e₀∧e₁` and translates by 5 along `e₃`
    // — the axis the rotor fixes — so the Burgers vector is exactly
    // `(0, 0, 5)` and the centre is the origin. Exact over ℚ: not near it,
    // it IS it.
    let split = crate::transducer::element::screw_of(&quarter(), &screw)
        .expect("a screw splits; it is the FIXED POINT that does not exist");
    assert_eq!(split.twist, [q(0), q(0), q(5)], "the slip per circuit, on the axis the rotor fixes");
    assert_eq!(split.centre, [q(0), q(0), q(0)], "and the bend turns about the origin");
    assert!(
        kappa.occupies(crate::transducer::sentence::Axis::Placement),
        "a slip occupies the placement axis — this is the obstruction a demand is for"
    );

    // A PURE TURN HAS NO SLIP, and the same reader says so: the two arms
    // are one function, so a fixed point cannot be found by one reader and
    // denied by another (law 12).
    let about = crate::transducer::element::screw_of(&quarter(), &about_a_point)
        .expect("a turn about a point splits too");
    assert_eq!(about.twist, [q(0), q(0), q(0)], "a turn that returns slips by nothing");
    assert_eq!(
        crate::transducer::element::turn_base_point(&quarter(), &about_a_point),
        Some(about.centre),
        "and `turn_base_point` IS the zero-twist case of the split"
    );

    // AND THE TWO TURNS ARE ONE CLASS. `T(p − Rp)·R = T_p R T_p⁻¹` is
    // CONJUGATE to `R`, so a quarter turn about the origin and the same
    // quarter turn about `p` stand at one conjugacy stratum. The old
    // placement byte wrote `0` for the first and `1` for the second and
    // split them into two class keys — a choice of origin promoted to
    // identity. Both write the zero Burgers vector now, and the split is
    // gone.
    let at_origin = crate::transducer::sentence::ClassVector::of_rotor(
        &quarter(),
        ident(),
        CycleType::Identity,
    )
    .expect("a quarter turn has a class");
    let about_p = crate::transducer::sentence::ClassVector::of_rotor(
        &quarter(),
        about_a_point.clone(),
        CycleType::Identity,
    )
    .expect("so does the same turn about a point");
    assert_eq!(
        at_origin.class_key().as_bytes()[..9],
        about_p.class_key().as_bytes()[..9],
        "the conjugacy stratum is the same, which it always was"
    );
    assert!(
        !at_origin.occupies(crate::transducer::sentence::Axis::Placement)
            && !about_p.occupies(crate::transducer::sentence::Axis::Placement),
        "and neither occupies the placement axis: a base point is where the turn stands"
    );
}

/// THE READING THE ENGINE ACTUALLY TAKES, from two bodies rather than a
/// hand-built carriage. `versor_coset` builds every member as
/// `T(c_b)·D(ρ)·R·T(−c_a)`, so a body turned a quarter about itself
/// arrives with a placement in its carriage ALWAYS — there is no path by
/// which such a transport reaches `classify` flat. That is the whole of
/// why no turning body ever filed a period: the carriage was never flat,
/// and `flat` was the only gate.
///
/// The two assertions here are the before and the after on one reading:
/// the carriage is NOT flat, and it files at four anyway, because the
/// translation it carries is the turn's own base point and solves.
///
/// Measured on this pair: three members carry the L onto its image, all
/// three with a placement, and none of them could file while `flat` was
/// the gate. Two file now — the quarter turn at four and a half turn at
/// two — and the third is a reflection the seventeen do not spell, which
/// is 37.7's arm and not a missed fill.
#[test]
fn a_body_turned_about_itself_files_though_its_carriage_is_not_flat() {
    use crate::transducer::blade::{shape_of, versor_coset_ordered};
    use crate::transducer::element::{base_rotor_of, dilator_of, translator_of};
    // An L, and the same L under `(r, c) → (c, −r)` carried back onto the
    // non-negative quadrant: a genuine quarter turn between two bodies.
    let l = vec![(0i64, 0i64, 0i64), (1, 0, 0), (2, 0, 0), (2, 1, 0)];
    let turned = vec![(0i64, 0i64, 0i64), (0, 1, 0), (0, 2, 0), (1, 0, 0)];
    let (a, b) = (shape_of(&l), shape_of(&turned));
    let members = versor_coset_ordered(&a, &b);
    assert!(!members.is_empty(), "a quarter turn carries the L onto its image");
    let mut read = 0usize;
    let mut placed = 0usize;
    let mut filed = 0usize;
    for v in &members {
        // `edge_of`'s own split: the rotation on the base, and `D·T` as
        // the carriage. Nothing here is built for the test.
        let (Some(r), Some(d), Some(t)) = (base_rotor_of(v), dilator_of(v), translator_of(v))
        else {
            continue;
        };
        read += 1;
        let carriage = d.geo(&t);
        if !same_versor(&carriage, &ident()) {
            placed += 1;
        }
        if matches!(classify(&r, &carriage, CycleType::Identity), Fill::Congruent(_, _, 4)) {
            filed += 1;
        }
    }
    assert_eq!(read, 3, "three members carry the L onto its image");
    assert_eq!(placed, read, "every one of them carries a placement: none is flat");
    assert_eq!(filed, 1, "and the quarter turn files at four, placement and all");
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
        carriage: cga::at_three::translator(&q(3), &q(-2), &q(0)),
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
