// BQ5a, 72.1, 72.2 — a contact's edge is the refraction across it, read on
// the pair the topology names, each body against its own transition; no band
// decides whether.

use crate::transducer::blade::{self, Contact};
use crate::transducer::family::pairing::{contact_edge, ContactEdge};
use crate::transducer::sentence::{HeldPose, Refraction};

fn body(cells: &[(i64, i64, i64)]) -> blade::Blade {
    blade::shape_of(cells).blade
}

/// A body's nucleus: the face's body pair carries it (BD2, 78.18(c)).
fn nucleus(b: &blade::Blade) -> crate::transducer::cga::Mv {
    b.nucleus().expect("the body's nucleus reads")
}

fn pose(before: &blade::Blade, after: &blade::Blade) -> crate::transducer::cga::Mv {
    match blade::relative_pose(before, after, &mut super::common::budget()) {
        Ok(blade::Pose::Turn(r)) => r,
        other => panic!("a determined turn: {other:?}"),
    }
}

/// THE EDGE IS `g = (θ_bθ_a⁻¹)·V`, CARRIED WHOLE, AND NOTHING GATES IT.
///
/// Two L-trominoes meet at ONE corner — outside the retired band that
/// decided whether a face was written, so the field refused them. Under the transition the first turns 90° and the second
/// stays: each body's own pose is read against itself, `θ_bθ_a⁻¹` is how the
/// one turned against the other, and the contact carries it composed with
/// the tether — in the carriage entire, never split by lane, never the
/// rigidity of the two bodies' union, and never a congruence asked of two
/// unlike pieces.
#[test]
fn contact_edge_carries_refraction_not_rigidity() {
    let a = body(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    let a_after = body(&[(0, 0, 0), (0, 1, 0), (-1, 0, 0)]);
    let b = body(&[(2, 1, 0), (3, 1, 0), (2, 2, 0)]);
    let v = blade::tether(&a, &b).expect("both nuclei read");
    assert_eq!(Contact::of(&v), Some(Contact::Apart), "a corner contact reads apart as a tether");
    let (theta_a, theta_b) = (pose(&a, &a_after), pose(&b, &b));
    let relative = theta_b.geo(&theta_a.reverse());
    let ContactEdge::Formed(edge) =
        contact_edge((&nucleus(&a), &nucleus(&b)), Some((&a, &a_after)), Some((&b, &b)), &mut super::common::budget())
    else {
        panic!("the band gates nothing: the contact carries its edge");
    };
    assert!(edge.carriage.projective_eq(&relative.geo(&v)), "the carriage is (θ_bθ_a⁻¹)·V, whole");
    assert!(
        !edge.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()),
        "the word reads the 90° turn one body made against the other"
    );

    // Two unlike touching bodies — no turn carries the one onto the other —
    // still each carry a refraction: each is read against itself.
    // (An asymmetric one: a bar's own 180° turn would hold its pose open.)
    let bar = body(&[(3, 0, 0), (4, 0, 0), (5, 0, 0), (5, 1, 0)]);
    assert!(blade::relative_pose(&a, &bar, &mut super::common::budget()).is_err(), "an L-tromino and an L-tetromino are not one turn apart");
    assert!(
        matches!(
            contact_edge((&nucleus(&a), &nucleus(&bar)), Some((&a, &a)), Some((&bar, &bar)), &mut super::common::budget()),
            ContactEdge::Formed(_)
        ),
        "yet the bodies' contact is read, never refused for unlikeness"
    );

    // A body whose own transition is a mirror is read orientation-reversed
    // (AL1 as amended): its pose is the odd versor, and the contact carries
    // its refraction where it used to be refused.
    let j = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let j_after = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
    let edge = contact_edge((&nucleus(&j), &nucleus(&bar)), Some((&j, &j_after)), Some((&bar, &bar)), &mut super::common::budget());
    assert!(
        matches!(edge, ContactEdge::Formed(_) | ContactEdge::Held(_)),
        "a mirrored transition carries its refraction"
    );
}

/// CV1b F3, 78.38 — AN OBLIQUE MEMBER REFUSES THE EDGE AND RAISES THE DEMAND.
///
/// The L-tetromino's own transition is `z ↦ (4 + i)·z`: one shape at two
/// scales, certified as a dilation on the point tower, at an angle the
/// alphabet does not spell. The contact carries no edge and names the arm
/// (`Oblique`, D); the pole reads the member through D3's three gates and
/// raises `RotorPrime` over 17 with the arc of `(4 + i)²` as witness. Beside
/// it, an S-tetromino against the image is a different shape, refused at
/// its order, and raises nothing.
#[test]
fn oblique_rotor_prime_raises_demand_where_edge_refuses() {
    use crate::transducer::family::oblique_rotor_prime;
    use crate::transducer::sentence::Demand;
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let image = body(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    let bar = body(&[(10, 0, 0), (11, 0, 0), (10, 1, 0)]);
    assert!(
        matches!(
            contact_edge((&nucleus(&l), &nucleus(&bar)), Some((&l, &image)), Some((&bar, &bar)), &mut super::common::budget()),
            ContactEdge::Refused { poses: [Some(blade::NoPose::Oblique), None] }
        ),
        "a similarity at an unspelled angle carries no edge, and the refusal names the arm"
    );

    let Some(Demand::RotorPrime { prime, witness, .. }) = oblique_rotor_prime(&l, &image, &mut super::common::budget()) else {
        panic!("the oblique member raises the rotor-prime demand");
    };
    assert_eq!(prime, 17, "the prime 4 + i lies over");
    let arc = crate::transducer::alphabet::arc_block(&15.into(), &8.into());
    assert_eq!(*witness, arc, "the witness is the arc of (4 + i)²");

    let s = body(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    assert!(oblique_rotor_prime(&s, &image, &mut super::common::budget()).is_none(), "a different shape raises nothing");
}

/// F5 — GATE 1 PASSES THE OBLIQUE ARM ALONE: a pair reading a turn of the
/// base is no refusal, an unlike pair is P, an empty body is I; none mints.
#[test]
fn oblique_rotor_prime_passes_oblique_arm_alone() {
    use crate::transducer::family::oblique_rotor_prime;
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let turn_90 = body(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (-1, 2, 0)]);
    let s = body(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    let empty = blade::shape_of::<(i64, i64, i64)>(&[]).blade;
    assert!(blade::relative_pose(&l, &turn_90, &mut super::common::budget()).is_ok(), "the fixture: a 90° turn apart");
    for (a, b) in [(&l, &turn_90),(&l, &s), (&empty, &l)] {
        assert!(oblique_rotor_prime(a, b, &mut super::common::budget()).is_none(), "none of them is oblique");
    }
}

/// CV1b F3 (R2), A.17 — THE POLE'S DEMANDS STAND ONCE: the rotor prime
/// stands; the same demand reached again merges into it; a kind with no
/// coordinates to key it, and a rotor prime whose witness carries nothing,
/// do not stand.
#[test]
fn stand_demands_stands_rotor_prime_once() {
    use crate::transducer::family::{oblique_rotor_prime, stand_demands};
    use crate::transducer::sentence::{Demand, Vertices};
    use crate::transducer::stalk::Invariant;
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let image = body(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    let rotor_prime = oblique_rotor_prime(&l, &image, &mut super::common::budget()).expect("the oblique pair's demand");
    let Demand::RotorPrime { plane, prime, .. } = rotor_prime.clone() else { panic!("a rotor prime") };
    let empty = Demand::RotorPrime { plane, prime, witness: Box::default() };
    let chart = Invariant::default();
    let mut vertices = Vertices::new();
    stand_demands(
        &mut vertices,
        vec![
            (rotor_prime.clone(), chart.clone(), Vec::new()),
            (rotor_prime, chart.clone(), Vec::new()),
            (Demand::Creation, chart.clone(), Vec::new()),
            (empty, chart, Vec::new()),
        ],
    );
    assert_eq!(vertices.len(), 1, "the rotor prime stands, once");
}

/// 78.6(1) — A HELD `ℤ₂` RIDES THE EDGE AS A COSET: an S-tetromino against
/// its own 90° turn holds its pose open (its odd moment vanishes), so
/// its contact carries BOTH candidates `θ·V` — two distinct transports,
/// never a representative — and the tally names a coset formed.
#[test]
fn contact_edge_holds_turn_180_as_coset() {
    let s4 = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)];
    let s = body(&s4);
    let turned = body(&s4.map(|(r, c, k)| (-c, r, k)));
    let Ok(blade::Pose::Open180(candidates)) = blade::relative_pose(&s, &turned, &mut super::common::budget()) else {
        panic!("the S holds its 180° turn");
    };
    let b = body(&[(4, 0, 0), (5, 0, 0), (4, 1, 0)]);
    let v = blade::tether(&s, &b).expect("both nuclei read");
    let ContactEdge::Held(Refraction::Coset(coset)) =
        contact_edge((&nucleus(&s), &nucleus(&b)), Some((&s, &turned)), Some((&b, &b)), &mut super::common::budget())
    else {
        panic!("a held pose is a coset edge");
    };
    assert_eq!(coset.len(), 2, "both candidates of the held ℤ₂");
    for (edge, theta) in coset.iter().zip(&candidates) {
        let relative = theta.reverse();
        assert!(edge.carriage.projective_eq(&relative.geo(&v)), "each candidate is (θ_bθ_a⁻¹)·V");
    }
    assert!(!coset[0].carriage.projective_eq(&coset[1].carriage), "two candidates, two transports");
}

/// 78.6(2), 78.18(a) — A PIECE AT ONE FRAME CARRIES ITS TETHER: the edge is
/// `V = s_b s_a` of the two BODIES' nuclei with the refraction unread by name
/// — static — never the identity; the member that did transit keeps its pose
/// on the edge by name and the one at one frame is named so, so a later
/// fibre can complete the refraction. No refraction is asked, and none
/// refused.
#[test]
fn contact_edge_static_carries_tether_and_pose() {
    let a = body(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    let b = body(&[(2, 1, 0), (3, 1, 0), (2, 2, 0)]);
    let b_after = body(&[(2, 1, 0), (2, 2, 0), (1, 1, 0)]);
    let v = blade::tether(&a, &b).expect("both nuclei read");
    let ContactEdge::Held(Refraction::Static { tether, poses }) =
        contact_edge((&nucleus(&a), &nucleus(&b)), None, Some((&b, &b_after)), &mut super::common::budget())
    else {
        panic!("a piece at one frame is a static edge");
    };
    assert_eq!(tether, v, "the tether of the two bodies, whole");
    let [HeldPose::OneFrame, HeldPose::Read(theta_b)] = &poses else { panic!("the poses as read: {poses:?}") };
    assert!(theta_b[0].projective_eq(&pose(&b, &b_after)), "the transited end's pose, kept");
}

/// 78.71 — EVERY LANE READ AS A FIELD: each body's transport through the rule
/// books its own modes by the mass it carried — a turn on 𝒽, a dilation on
/// 𝒮, a hue map on 𝒢 — and its null part as flux on 𝒯 in its tube's quanta,
/// never a chord's length and never twice. Two bodies translated along one
/// ray, one and two units: the first opens the tube by its four, the second
/// closes two quanta of four on it, relieving the four — twelve transported.
/// The turn, the dilation and the hue map each open a separate orbit (78.73).
#[test]
fn transport_order_reads_every_lane_as_field() {
    use crate::transducer::cga::{self, Mv, E1, E2};
    use crate::transducer::family::pairing::transport_order;
    let q = |n: i64, d: i64| num_rational::BigRational::new(n.into(), d.into());
    let mut b = crate::physics::Physics::default().stroke_n(1 << 16);
    let turn = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
    let dilate = cga::dilator(&q(2, 1));
    let slide = |x: i64| cga::translator_at(&[q(x, 1), q(0, 1), q(0, 1)]);
    let swap = vec![(1u8, 2u8), (2, 1)];
    let one = Mv::scalar(q(1, 1));
    let line = |carriage: Mv, gauge: Vec<(u8, u8)>, mass: u64| crate::transducer::family::pairing::FieldLine {
        carriage,
        gauge,
        mass,
        free: false,
        line: crate::transducer::cech::orbit::Thread::Free,
    };
    let transports = vec![
        line(turn.clone(), Vec::new(), 3),
        line(dilate, Vec::new(), 5),
        line(one, swap, 7),
        line(slide(1), Vec::new(), 4),
        line(slide(2), Vec::new(), 4),
    ];
    // 78.74 — the same turn in a frame whose fibres did no work presses
    // nothing: carried as torsion, by its mass.
    let mut quiet = crate::transducer::cech::orbit::Field::new();
    let free = transport_order(&[crate::transducer::family::pairing::FieldLine { free: true, ..line(turn, Vec::new(), 3) }], &[], &mut quiet, &mut b);
    assert_eq!((free.opened[3], free.torsion[3], quiet.orbits.len()), (0, 3, 0), "a free turn: no pressure, no orbit, torsion 3");
    let mut field = crate::transducer::cech::orbit::Field::new();
    let r = transport_order(&transports, &[], &mut field, &mut b);
    // 78.73: a first deformation on each orbit stands open — pressure by the
    // mass it carried — and builds no order until the orbit comes back.
    assert_eq!(r.opened, [5, 7, 4, 3], "the dilation on 𝒮, the hue map on 𝒢, the first slide on 𝒯, the turn on 𝒽");
    assert_eq!(r.order, [0, 0, 8, 0], "only the second slide closed: on its tube");
    assert_eq!(
        (r.opened[2], r.order[2], r.relieved[2], r.transport[2]),
        (4, 8, 4, 12),
        "two slides on one ray: opened by four, closed by two quanta of four, 𝒯 in quanta, never a length"
    );
    assert_eq!((field.tubes.len(), field.orbits.len()), (1, 3), "one ray, one tube; three orbits open");
}

/// 78.68 — A CONFORMAL CARRIAGE'S LANES: a translator carries a
/// displacement, 𝒯; a dilator is 𝒮; a turn about the origin is 𝒽 — the
/// rotor's lane on 42.6's table (78.69) — and carries no displacement. And why the lanes are read off the conformal
/// carriage and never its transvection (51.2): the transvection the same
/// displacement realises reads as a scale about another point, its
/// displacement gone.
#[test]
fn carriage_lanes_reads_lane_transported() {
    use crate::transducer::cga;
    use crate::transducer::family::pairing::carriage_lanes;
    let mut b = crate::physics::Physics::default().stroke_n(1 << 16);
    let one = num_rational::BigRational::from_integer(1.into());
    let zero = num_rational::BigRational::from_integer(0.into());
    // Half a unit: the transvection's Cayley chart ends at |u| = 1, the
    // light cone (56), so a unit displacement has no finite boost.
    let half = num_rational::BigRational::new(1.into(), 2.into());
    let t = cga::translator_at(&[half, zero.clone(), zero]);
    assert_eq!(carriage_lanes(&t, &mut b), [false, false, true, false], "a translation is 𝒯");
    let v = crate::transducer::element::transvection_of(&t).expect("a finite displacement inside the cone");
    assert!(
        !carriage_lanes(&v, &mut b)[crate::transducer::rewrite::LANE_TEMPORAL],
        "its transvection no longer reads the displacement, which is why the motor is read"
    );
    let d = cga::dilator(&num_rational::BigRational::from_integer(2.into()));
    assert_eq!(carriage_lanes(&d, &mut b), [true, false, false, false], "a dilation is 𝒮");
    let r = cga::Mv::scalar(one.clone()).add(&cga::Mv::blade(cga::E1 | cga::E2, one));
    assert_eq!(carriage_lanes(&r, &mut b), [false, false, false, true], "a turn about the origin is 𝒽, the rotor's lane (42.6)");
}

