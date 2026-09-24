// BQ5a, 72.1, 72.2 — a contact's edge is the refraction across it, read on
// the pair the topology names, each body against its own transition; no band
// decides whether.

use crate::transducer::blade::{self, Contact};
use crate::transducer::family::pairing::{contact_edge, take_contact_tally, ContactEdge};
use crate::transducer::sentence::{HeldPose, Refraction};

fn body(cells: &[(i64, i64, i64)]) -> blade::Blade {
    blade::shape_of(cells).blade
}

/// A body's nucleus: the face's body pair carries it (BD2, 78.18(c)).
fn nucleus(b: &blade::Blade) -> crate::transducer::cga::Mv {
    b.nucleus().expect("the body's nucleus reads")
}

fn pose(before: &blade::Blade, after: &blade::Blade) -> crate::transducer::cga::Mv {
    match blade::relative_pose(before, after) {
        Ok(blade::Pose::Turn(r)) => r,
        other => panic!("a determined turn: {other:?}"),
    }
}

/// THE EDGE IS `g = (θ_bθ_a⁻¹)·V`, CARRIED WHOLE, AND NOTHING GATES IT.
///
/// Two L-trominoes meet at ONE corner, `δ² > 4` — outside the band
/// `1 < δ ≤ 2` that decided whether a face was written, so the field refused
/// them. Under the transition the first turns a quarter and the second
/// stays: each body's own pose is read against itself, `θ_bθ_a⁻¹` is how the
/// one turned against the other, and the contact carries it composed with
/// the tether — in the carriage entire, never split by lane, never the
/// rigidity of the two bodies' union, and never a congruence asked of two
/// unlike pieces.
#[test]
fn a_contact_edge_is_refraction_never_rigidity() {
    let _ = take_contact_tally();
    let a = body(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    let a_after = body(&[(0, 0, 0), (0, 1, 0), (-1, 0, 0)]);
    let b = body(&[(2, 1, 0), (3, 1, 0), (2, 2, 0)]);
    let v = blade::tether(&a, &b).expect("both nuclei read");
    assert_eq!(Contact::of(&v), Some(Contact::Apart), "a corner contact reads apart as a tether");
    let d2 = &v.scalar_part() * &v.scalar_part() / v.geo(&v.reverse()).scalar_part();
    assert!(d2 > crate::core::Q::from_integer(4.into()), "outside the retired band: δ² = {d2}");
    let (theta_a, theta_b) = (pose(&a, &a_after), pose(&b, &b));
    let relative = theta_b.geo(&theta_a.reverse());
    let ContactEdge::Formed(edge) =
        contact_edge((&nucleus(&a), &nucleus(&b)), Some((&a, &a_after)), Some((&b, &b)))
    else {
        panic!("the band gates nothing: the contact carries its edge");
    };
    assert!(edge.carriage.projective_eq(&relative.geo(&v)), "the carriage is (θ_bθ_a⁻¹)·V, whole");
    assert!(
        !edge.word.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()),
        "the word reads the quarter turn one body made against the other"
    );
    let t = take_contact_tally();
    assert_eq!((t.offered, t.apart, t.condensed), (1, 1, 1), "offered, read apart, condensed");

    // Two unlike touching bodies — no turn carries the one onto the other —
    // still carry their refraction: each is read against itself.
    // (An asymmetric one: a bar's own half turn would hold its pose open.)
    let bar = body(&[(3, 0, 0), (4, 0, 0), (5, 0, 0), (5, 1, 0)]);
    assert!(blade::relative_pose(&a, &bar).is_err(), "an L-tromino and an L-tetromino are not one turn apart");
    assert!(
        matches!(
            contact_edge((&nucleus(&a), &nucleus(&bar)), Some((&a, &a)), Some((&bar, &bar))),
            ContactEdge::Formed(_)
        ),
        "yet their contact is read, never refused for unlikeness"
    );

    // A body whose own transition is a mirror has no pose; the contact stands
    // without an edge and the arm is named.
    let j = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let j_after = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
    let _ = take_contact_tally();
    assert!(
        matches!(
            contact_edge((&nucleus(&j), &nucleus(&bar)), Some((&j, &j_after)), Some((&bar, &bar))),
            ContactEdge::Refused { poses: [Some(blade::NoPose::Mirrored), None] }
        ),
        "a mirrored transition carries no refraction, and the refusal names the member's arm"
    );
    let t = take_contact_tally();
    assert_eq!((t.offered, t.mirrored, t.condensed), (1, 1, 0), "refused by its arm, never dropped");
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
fn an_oblique_member_refuses_the_edge_and_raises_the_rotor_prime() {
    use crate::transducer::family::{oblique_rotor_prime, take_demand_tally};
    use crate::transducer::sentence::Demand;
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let image = body(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    let bar = body(&[(10, 0, 0), (11, 0, 0), (10, 1, 0)]);
    let _ = take_contact_tally();
    assert!(
        matches!(
            contact_edge((&nucleus(&l), &nucleus(&bar)), Some((&l, &image)), Some((&bar, &bar))),
            ContactEdge::Refused { poses: [Some(blade::NoPose::Oblique), None] }
        ),
        "a similarity at an unspelled angle carries no edge, and the refusal names the arm"
    );
    let t = take_contact_tally();
    assert_eq!((t.offered, t.oblique, t.condensed), (1, 1, 0), "refused oblique, never dropped");

    let _ = take_demand_tally();
    let Some(Demand::RotorPrime { prime, witness, .. }) = oblique_rotor_prime(&l, &image) else {
        panic!("the oblique member raises the rotor-prime demand");
    };
    assert_eq!(prime, 17, "the prime 4 + i lies over");
    let arc = crate::transducer::alphabet::arc_block(&15.into(), &8.into());
    assert_eq!(*witness, arc, "the witness is the arc of (4 + i)²");

    let s = body(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    assert!(oblique_rotor_prime(&s, &image).is_none(), "a different shape raises nothing");
    let d = take_demand_tally();
    assert_eq!((d.offered, d.minted, d.permanent), (2, 1, 1), "one minted, one refused permanent");
}

/// F5 — GATE 1 IS LABELLED BY THE ANSWER'S KIND, NEVER BY THE BRANCH: a pair
/// reading a turn of the base is no refusal and counts `turned`; an unlike
/// pair is P, `permanent`; an empty body is I, `bodiless`. None of them is
/// counted where another's label belongs.
#[test]
fn the_mints_first_gate_counts_each_answer_by_its_kind() {
    use crate::transducer::family::{oblique_rotor_prime, take_demand_tally};
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let quarter = body(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (-1, 2, 0)]);
    let s = body(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    let empty = blade::shape_of::<(i64, i64, i64)>(&[]).blade;
    let _ = take_demand_tally();
    assert!(blade::relative_pose(&l, &quarter).is_ok(), "the fixture: a quarter turn apart");
    for (a, b) in [(&l, &quarter), (&l, &s), (&empty, &l)] {
        assert!(oblique_rotor_prime(a, b).is_none(), "none of them is oblique");
    }
    let d = take_demand_tally();
    assert_eq!(
        (d.offered, d.turned, d.permanent, d.bodiless, d.held, d.unclassified, d.minted),
        (3, 1, 1, 1, 0, 0, 0),
        "each answer under its own kind: {}",
        d.line()
    );
}

/// CV1b F3 (R2), A.17 — THE POLE'S DEMANDS STAND ONCE, AND NONE LEAVES
/// UNNAMED: the rotor prime stands; the same demand reached again merges into
/// it; a kind with no coordinates to key it is `unkeyed`; a rotor prime whose
/// witness carries nothing is `untyped`. Each counted on the mint's tally.
#[test]
fn the_poles_demands_stand_once_and_every_other_is_named() {
    use crate::transducer::family::{oblique_rotor_prime, stand_demands, take_demand_tally};
    use crate::transducer::sentence::{Demand, Vertex};
    use crate::transducer::stalk::Invariant;
    let l = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let image = body(&[(0, 0, 0), (4, 1, 0), (8, 2, 0), (7, 6, 0)]);
    let rotor_prime = oblique_rotor_prime(&l, &image).expect("the oblique pair's demand");
    let Demand::RotorPrime { plane, prime, .. } = rotor_prime.clone() else { panic!("a rotor prime") };
    let empty = Demand::RotorPrime { plane, prime, witness: Box::default() };
    let chart = Invariant::default();
    let _ = take_demand_tally();
    let mut vertices: Vec<Vertex> = Vec::new();
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
    let t = take_demand_tally();
    assert_eq!((t.merged, t.unkeyed, t.untyped), (1, 1, 1), "every demand not stood, by its name");
}

/// 78.6(1) — A HELD `ℤ₂` RIDES THE EDGE AS A COSET: an S-tetromino against
/// its own quarter turn holds its pose open (its odd moment vanishes), so
/// its contact carries BOTH candidates `θ·V` — two distinct transports,
/// never a representative — and the tally names a coset formed.
#[test]
fn a_held_half_turn_rides_the_contact_edge_as_a_coset() {
    let s4 = [(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)];
    let s = body(&s4);
    let turned = body(&s4.map(|(r, c, k)| (-c, r, k)));
    let Ok(blade::Pose::HalfOpen(candidates)) = blade::relative_pose(&s, &turned) else {
        panic!("the S holds its half turn");
    };
    let b = body(&[(4, 0, 0), (5, 0, 0), (4, 1, 0)]);
    let v = blade::tether(&s, &b).expect("both nuclei read");
    let _ = take_contact_tally();
    let ContactEdge::Held(Refraction::Coset(coset)) =
        contact_edge((&nucleus(&s), &nucleus(&b)), Some((&s, &turned)), Some((&b, &b)))
    else {
        panic!("a held pose is a coset edge");
    };
    assert_eq!(coset.len(), 2, "both candidates of the held ℤ₂");
    for (edge, theta) in coset.iter().zip(&candidates) {
        let relative = theta.reverse();
        assert!(edge.carriage.projective_eq(&relative.geo(&v)), "each candidate is (θ_bθ_a⁻¹)·V");
    }
    assert!(!coset[0].carriage.projective_eq(&coset[1].carriage), "two candidates, two transports");
    let t = take_contact_tally();
    assert_eq!((t.half_open, t.cosets, t.condensed), (1, 1, 0), "a coset formed, no representative condensed");
}

/// 78.6(2), 78.18(a) — A PIECE AT ONE FRAME CARRIES ITS TETHER: the edge is
/// `V = s_b s_a` of the two BODIES' nuclei with the refraction unread by name
/// — static — never the identity; the member that did transit keeps its pose
/// on the edge by name and the one at one frame is named so, so a later
/// fibre can complete the refraction. No refraction is asked, and none
/// refused.
#[test]
fn a_static_contact_carries_its_tether_and_the_pose_it_read() {
    let a = body(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    let b = body(&[(2, 1, 0), (3, 1, 0), (2, 2, 0)]);
    let b_after = body(&[(2, 1, 0), (2, 2, 0), (1, 1, 0)]);
    let v = blade::tether(&a, &b).expect("both nuclei read");
    let _ = take_contact_tally();
    let ContactEdge::Held(Refraction::Static { tether, poses }) =
        contact_edge((&nucleus(&a), &nucleus(&b)), None, Some((&b, &b_after)))
    else {
        panic!("a piece at one frame is a static edge");
    };
    assert_eq!(tether, v, "the tether of the two bodies, whole");
    let [HeldPose::OneFrame, HeldPose::Read(theta_b)] = &poses else { panic!("the poses as read: {poses:?}") };
    assert!(theta_b[0].projective_eq(&pose(&b, &b_after)), "the transited end's pose, kept");
    let t = take_contact_tally();
    assert_eq!((t.statics, t.half_open, t.condensed, t.mirrored), (1, 0, 0, 0), "static by name, no refraction asked");
}

