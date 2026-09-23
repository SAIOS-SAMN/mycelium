// BQ5a, 72.1, 72.2 — a contact's edge is the refraction across it, read on
// the pair the topology names, each body against its own transition; no band
// decides whether.

use crate::transducer::blade::{self, Contact};
use crate::transducer::family::pairing::{contact_edge, take_contact_tally, ContactEdge};
use crate::transducer::sentence::Refraction;

fn body(cells: &[(i64, i64, i64)]) -> blade::Blade {
    blade::shape_of(cells).blade
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
    let relative = theta_b.geo(&theta_a.versor_inverse().expect("a turn inverts"));
    let ContactEdge::Formed(edge) = contact_edge((&a, Some(&a_after)), (&b, Some(&b))) else {
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
        matches!(contact_edge((&a, Some(&a)), (&bar, Some(&bar))), ContactEdge::Formed(_)),
        "yet their contact is read, never refused for unlikeness"
    );

    // A body whose own transition is a mirror has no pose; the contact stands
    // without an edge and the arm is named.
    let j = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)]);
    let j_after = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
    let _ = take_contact_tally();
    assert!(
        matches!(contact_edge((&j, Some(&j_after)), (&bar, Some(&bar))), ContactEdge::Refused),
        "a mirrored transition carries no refraction"
    );
    let t = take_contact_tally();
    assert_eq!((t.offered, t.mirrored, t.condensed), (1, 1, 0), "refused by its arm, never dropped");
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
    let ContactEdge::Held(Refraction::Coset(coset)) = contact_edge((&s, Some(&turned)), (&b, Some(&b))) else {
        panic!("a held pose is a coset edge");
    };
    assert_eq!(coset.len(), 2, "both candidates of the held ℤ₂");
    for (edge, theta) in coset.iter().zip(&candidates) {
        let relative = theta.versor_inverse().expect("a turn inverts");
        assert!(edge.carriage.projective_eq(&relative.geo(&v)), "each candidate is (θ_bθ_a⁻¹)·V");
    }
    assert!(!coset[0].carriage.projective_eq(&coset[1].carriage), "two candidates, two transports");
    let t = take_contact_tally();
    assert_eq!((t.half_open, t.cosets, t.condensed), (1, 1, 0), "a coset formed, no representative condensed");
}

/// 78.6(2) — A PIECE AT ONE FRAME CARRIES ITS TETHER: no transition, so no
/// pose is read and none is refused; the edge is `V = s_b s_a` with the
/// refraction unread by name — static — never the identity.
#[test]
fn a_static_contact_carries_its_tether_with_the_refraction_unread() {
    let a = body(&[(0, 0, 0), (1, 0, 0), (0, 1, 0)]);
    let b = body(&[(2, 1, 0), (3, 1, 0), (2, 2, 0)]);
    let v = blade::tether(&a, &b).expect("both nuclei read");
    let _ = take_contact_tally();
    let ContactEdge::Held(Refraction::Static(tether)) = contact_edge((&a, None), (&b, Some(&b))) else {
        panic!("a piece at one frame is a static edge");
    };
    assert_eq!(tether, v, "the tether, whole");
    let t = take_contact_tally();
    assert_eq!((t.statics, t.half_open, t.condensed, t.mirrored), (1, 0, 0, 0), "static by name, no pose asked");
}
