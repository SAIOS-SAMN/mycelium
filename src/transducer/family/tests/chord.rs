// 78.96 — R3 RULED LUMINAL: a chord's Burgers vector is carried by its own
// translator, a null generator, so a ring of slides closes or extrapolates as
// a nilpotent theorem instead of standing unbound.

use crate::transducer::base::Site;
use num_traits::Zero;
use crate::transducer::family::chord::dispiration;
use crate::transducer::sentence::compose::{compose, Closure, Term};
use crate::transducer::sentence::theorem::{Certificate, Theorem};

fn foot(cells: &[(i64, i64)]) -> Vec<Site> {
    cells.iter().map(|&(r, c)| Site::from((r, c, 0))).collect()
}

/// The chord between two copies of a piece two columns apart is the
/// translator of their Burgers vector: its generator is null, so the chord
/// back is its exact inverse and two slides compose by adding their
/// displacements. A ring of three slides whose displacements sum to zero
/// closes flat; a ring that leaves a net slip is a NILPOTENT theorem, its
/// history one integer — never unbound, as the transvection read it.
#[test]
fn a_chords_slip_is_luminal_and_a_ring_of_slides_is_a_theorem() {
    let a = foot(&[(0, 0), (0, 1)]);
    let b = foot(&[(0, 2), (0, 3)]);
    let c = foot(&[(2, 2), (2, 3)]);
    let ab = dispiration(None, &a, &b).expect("a displacement is a chord").carriage;
    let ba = dispiration(None, &b, &a).expect("and back").carriage;
    let one = ab.geo(&ba);
    assert!(one.grade(2).is_zero() && !one.coeff(0).is_zero(), "the chord back is the exact inverse: {one:?}");
    let generator = ab.grade(2);
    assert!(!generator.is_zero() && generator.geo(&generator).is_zero(), "the slip's generator is null");
    assert!(dispiration(None, &a, &a).is_none(), "two footprints over one centre carry no slip");
    let word = crate::transducer::cga::Mv::scalar(num_rational::BigRational::from_integer(1.into())).widened(ab.n());
    let term = |t: &crate::transducer::cga::Mv| Term { word: word.clone(), transport: t.clone() };
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 12);
    // a → b → c → a: the displacements sum to zero, and the ring closes.
    let bc = dispiration(None, &b, &c).expect("a chord").carriage;
    let ca = dispiration(None, &c, &a).expect("a chord").carriage;
    let closed = compose(&[term(&ab), term(&bc), term(&ca)], &mut budget).expect("a sentence");
    assert_eq!(closed.closure, Closure::Closes(1), "slides that return close flat");
    // a → b → c, then c → b → a only part-way: a net slip remains, and the
    // ring is a nilpotent theorem, `Ωᵐ ∝ 1 + m·N`.
    let open = compose(&[term(&ab), term(&bc)], &mut budget).expect("a sentence");
    match Theorem::of(&open) {
        Ok(t) => assert!(matches!(t.certificate, Certificate::Nilpotent { .. }), "a net slip extrapolates: {:?}", open.closure),
        Err(why) => panic!("a ring of slides is a theorem, not {why:?}"),
    }
}
