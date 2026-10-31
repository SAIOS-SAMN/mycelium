// A21, rule 6 — THE STRATIFICATION OF A CLASS KEY: that a rotor's
// conjugacy class IS the class vector's first coordinate, and that a
// carrier which knows its own coordinates refuses a wrong depth and a
// foreign key instead of answering empty.

use crate::network::nerve::Stratified;
use crate::transducer::alphabet::{LetterId, Rotor, Word};
use crate::transducer::element;
use crate::transducer::sentence::{ClassVector, CycleType};

fn turn_90() -> Word {
    Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("the sensor plane"), false)])
}

/// THE CLAIM THE VORTEX COLLAPSE RESTS ON, asserted rather than trusted.
///
/// `ClassVector::class_key` writes the conjugacy invariants first, and
/// `Rotor::class_key` writes exactly those with the same tag and the same
/// sign byte. Two commits keyed a carrier by one and read it with the
/// other, and both found nothing, because both used an EXACT read where
/// the second is the first's depth-one projection.
///
/// The schema parses the boundary out of the bytes the writer wrote, so
/// this is a fact about the two writers and not about a length anyone
/// remembered: if either changes its framing, this fails here instead of
/// silently emptying a fiber on a fleet.
#[test]
fn class_key_leads_with_rotor_class() {
    let w = turn_90();
    let cv = ClassVector::of_word(&w).expect("a 90° turn has a class vector");
    let vector_key = cv.class_key();
    let rotor_key = w.rotor().class_key().expect("a 90° turn has a class");

    let schema = ClassVector::class_schema();
    let end = schema
        .boundary(vector_key.as_bytes(), 1)
        .expect("the class vector spells its first coordinate");
    assert_eq!(
        vector_key.as_bytes().get(..end),
        Some(rotor_key.as_bytes()),
        "the rotor's conjugacy class is the class vector's first coordinate, byte for byte"
    );

    // And the whole key parses to its own end: five coordinates, nothing
    // past the last. The placement status is the fifth and is written
    // LAST, so the conjugacy stratum every court asks by stays depth one.
    schema
        .whole(vector_key.as_bytes())
        .expect("a class key spells every coordinate and stops");
    assert_eq!(schema.depth(), 5, "conjugacy, plane, hue, carriage, placement");
}

/// A COARSE COORDINATE IS A FIBER AND THE WHOLE PATH IS AN EXACT READ, and
/// the carrier says which is which rather than letting a caller ask for one
/// and receive the other's empty answer.
///
/// Two orbits of one conjugacy class standing at different carriages are one
/// stratum and two entries. Read at depth one both answer; read whole, each
/// answers itself.
#[test]
fn fiber_at_depth_answers_every_class_vector_under_stratum() {
    let w = turn_90();
    let here = ClassVector::read(&w, element::identity_versor(), CycleType::Identity)
        .expect("a class vector at the origin");
    let moved = ClassVector::read(&w, element::translator_versor(1, 2), CycleType::Identity)
        .expect("the same class carried elsewhere");
    let (hk, mk) = (here.class_key(), moved.class_key());
    assert_ne!(hk, mk, "two carriages are two class vectors, or this proves nothing");

    let mut held: Stratified<u8> = Stratified::new(ClassVector::class_schema());
    held.insert(hk.as_bytes(), 1).expect("a whole path is bound");
    held.insert(mk.as_bytes(), 2).expect("and so is the other");

    let under: Vec<Vec<u8>> = held
        .fiber_at_depth(hk.as_bytes(), 1)
        .expect("depth one is the conjugacy stratum")
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        under.len(),
        2,
        "one conjugacy class, two class vectors, and the stratum answers both"
    );

    assert_eq!(held.get(hk.as_bytes()).expect("a whole path"), Some(&1));
    assert_eq!(held.get(mk.as_bytes()).expect("a whole path"), Some(&2));
}

/// A KEY OF ANOTHER SYSTEM IS REFUSED, NOT ANSWERED EMPTY (rule 11).
///
/// This is the whole defect in one assertion. A chart key and a class key
/// are both `Address` bytes, and `Keyed` answered `None` to a chart key
/// read against a class-keyed carrier — indistinguishable from "this class
/// is not held". The schema fails at the coordinate it fails at and names
/// the system.
#[test]
fn stratified_get_refuses_foreign_key_by_name() {
    let mut held: Stratified<u8> = Stratified::new(ClassVector::class_schema());
    let chart = crate::transducer::stalk::Invariant::counts(&[1, 3, 5]).key();

    let refused = held.get(chart.as_bytes()).expect_err("a chart is not a class vector");
    assert_eq!(refused.system, "class vector");
    assert_eq!(refused.segment, "conjugacy invariants", "it fails at the first coordinate");

    held.insert(chart.as_bytes(), 1)
        .expect_err("and a foreign key cannot be bound either");
}

/// THE TWO DEPTHS THAT ARE NOT PROJECTIONS ARE REFUSED, each pointing at
/// the read it actually is: depth zero is the whole carrier, and the full
/// depth is an exact read. A site asking for a projection cannot receive
/// either by accident.
#[test]
fn fiber_at_depth_refuses_depth_zero_and_whole_path() {
    let w = turn_90();
    let cv = ClassVector::of_word(&w).expect("a class vector");
    let key = cv.class_key();
    let mut held: Stratified<u8> = Stratified::new(ClassVector::class_schema());
    held.insert(key.as_bytes(), 1).expect("bound");

    held.fiber_at_depth(key.as_bytes(), 0)
        .err()
        .expect("depth zero is `scoped`");
    held.fiber_at_depth(key.as_bytes(), 5)
        .err()
        .expect("the whole path is `get`");
    assert_eq!(held.scoped().count(), 1, "and the whole carrier is one entry");
}

/// A ROTOR AND ITS INVERSE ARE ONE CONJUGACY STRATUM. `class_invariants`
/// reads the ray-corrected scalar and pseudoscalar squared over the weight,
/// and reversion preserves both grades while the weight divides out, so one
/// fiber answers both. The eliminator in the pivot drive branches on the
/// side of the chord to compute these separately; this says the two
/// branches name one coordinate.
#[test]
fn class_key_is_shared_by_rotor_and_inverse() {
    let w = turn_90();
    let r = w.rotor();
    let inv = r.reverse();
    assert!(!r.projective_eq(&inv), "q and q⁻¹ are two elements");
    assert_eq!(
        r.class_key(),
        inv.class_key(),
        "and one conjugacy class, so one stratum answers both"
    );
    let _: Rotor = inv;
}
