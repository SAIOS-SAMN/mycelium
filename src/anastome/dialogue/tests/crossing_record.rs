// ROADMAP track 25 X1, issue #160 — THE CROSSING RECORD: the four parts of
// a crossing written out and read back to exact values, and a record that
// carries only an address refused `AddressOnly`.

use crate::anastome::dialogue::crossing_record::{put_address_only, Body, Clauses, Content, Crossing, CrossingRefusal, Motion, Place, Ring, TurnBody};
use crate::physics::laws::TurnBreach;
use crate::transducer::cga::Mv;
use crate::chronometry::tuniversal::{Origin, TPoint};
use crate::chronometry::{Parent, Place as LinePlace, Stamp};
use crate::court::admission::Predecessor;
use crate::physics::laws::clauses::ConcordanceClauseId;
use crate::transducer::cga::rotator;
use crate::transducer::family::transduce::Turn;

fn q(n: i64, d: i64) -> crate::core::Q {
    crate::core::Q::new(n.into(), d.into())
}

fn fp(h: u64) -> TPoint {
    TPoint { origin: Origin { genesis: [9; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0].into(), k: h }
}

fn parent(h: u8) -> Parent {
    let k = u64::from(h);
    Parent { at: Stamp::new(k, [5; 32], LinePlace { k, i: 0 }), hash: Predecessor::of_bytes([h; 32]) }
}

/// A body of two phases standing at the origin `(0, 0)`, its boundary
/// breaking at quarter and half turns, taking three eighths of the frame.
fn body() -> Body {
    body_at(vec![Turn::of(0, 4), Turn::of(0, 2)])
}

fn body_at(origin: Vec<Turn>) -> Body {
    Body { origin, boundary: TurnBody { breakpoints: vec![vec![Turn::of(1, 4), Turn::of(3, 4)], vec![Turn::of(1, 2)]], extent: q(3, 8) } }
}

/// The ring through ticks 7 → 3 → 9, stated from 7: written from 3, the
/// least, the orientation kept (3 → 9 → 7). Its holonomy the (3,4,5) turn.
fn ring() -> Ring {
    Ring::new(vec![fp(7), fp(3), fp(9)], rotator(&q(2, 1), &q(1, 1))).expect("a ring")
}

fn crossing(motion: Motion) -> Crossing {
    Crossing {
        place: Place { at: fp(12), parents: vec![parent(10), parent(11)] },
        content: Content { frame: vec![4, 2], before: vec![body()], after: vec![body(), body()], motion },
        holonomy: vec![ring()],
        clauses: Clauses {
            read: vec![ConcordanceClauseId { law: "FORMS-PASSAGE".into(), version: 4 }],
            cited: vec![fp(11)],
            signings: vec![Stamp::new(3, [6; 32], LinePlace { k: 1, i: 2 })],
        },
    }
}

/// THE RECORD ROUND-TRIPS TO EXACT VALUES: the point and the past cone, the
/// bodies by their boundaries and extents, the motion (a versor, or turns
/// whose winding is kept: `5/4` reads back with its whole turn), the ring's
/// holonomy as a versor, and the clauses, cited points and signing stamps.
#[test]
fn crossing_round_trips_exact_values() {
    for motion in [Motion::Versor(rotator(&q(2, 1), &q(1, 1))), Motion::Turns(vec![Turn::of(5, 4), Turn::of(1, 3)])] {
        let c = crossing(motion.clone());
        let mut out = Vec::new();
        c.put(&mut out).expect("written");
        let back = Crossing::from_bytes(&out).expect("read whole");
        assert_eq!(back.place.at, c.place.at);
        assert_eq!(back.place.parents.len(), 2);
        assert!(back.place.parents.iter().zip(&c.place.parents).all(|(a, b)| a.at == b.at && Predecessor::cites(a, b)), "the past cone, by stamp and hash");
        assert_eq!(back.content, c.content, "bodies, extents and the motion exact");
        assert_eq!(back.holonomy, c.holonomy);
        assert_eq!(back.clauses, c.clauses);
        if let Motion::Turns(ts) = &back.content.motion {
            assert_eq!(ts[0].winding(), Some(1), "the winding is kept beside the phase");
        }
        let mut longer = out.clone();
        longer.push(0);
        assert_eq!(Crossing::from_bytes(&longer).err(), Some(CrossingRefusal::Malformed), "a byte left over");
    }
}

/// A RECORD WITHOUT CONTENT IS REFUSED `AddressOnly`: its place and nothing
/// where the object belongs. A turn written with a winding not its own does
/// not read.
#[test]
fn crossing_without_content_is_refused_address_only() {
    let mut out = Vec::new();
    put_address_only(&fp(12), &[parent(10)], &mut out);
    assert_eq!(Crossing::from_bytes(&out).err(), Some(CrossingRefusal::AddressOnly));
    let mut t = Vec::new();
    Turn::of(5, 4).put(&mut t).expect("written");
    t[16] = 0;
    assert!(Turn::take(&t, &mut 0).is_none(), "the winding written beside the turn must be its own");
}

/// A RING IS NAMED BY ITS VERTICES FROM THE LEAST: two statements that are
/// rotations of one cycle are one ring, and its writing reads back to it.
#[test]
fn ring_named_from_least_vertex_round_trips() {
    let r = ring();
    assert_eq!(r.vertices(), &[fp(3), fp(9), fp(7)], "from the least, the orientation kept");
    let rotated = Ring::new(vec![fp(9), fp(7), fp(3)], rotator(&q(2, 1), &q(1, 1))).expect("a ring");
    assert_eq!(rotated, r, "a rotation of the cycle is the one ring");
    let mut out = Vec::new();
    crossing(Motion::Turns(vec![Turn::of(1, 2)])).put(&mut out).expect("written");
    let back = Crossing::from_bytes(&out).expect("read whole");
    assert_eq!(back.holonomy, vec![r]);
    assert!(Ring::new(vec![fp(3), fp(9), fp(3)], rotator(&q(2, 1), &q(1, 1))).is_none(), "a vertex twice names no ring");
}

/// THE REVERSED RING IS ANOTHER NAME WITH THE INVERSE HOLONOMY: the same
/// vertices the other way round, from the least, and its holonomy composed
/// with the ring's is the identity.
#[test]
fn reversed_ring_has_inverse_holonomy() {
    let r = ring();
    let back = r.reversed().expect("an invertible holonomy");
    assert_eq!(back.vertices(), &[fp(3), fp(7), fp(9)]);
    assert_ne!(back, r, "a ring and its reverse are two names");
    assert!(back.holonomy.geo(&r.holonomy).sub(&Mv::scalar(q(1, 1))).is_zero(), "inverse holonomies");
    assert_eq!(back.reversed().expect("invertible"), r, "reversed twice, the ring again");
}

/// A WRITING THAT DOES NOT START AT ITS LEAST VERTEX IS REFUSED `RingStart`,
/// never read as another ring.
#[test]
fn ring_writing_off_least_start_is_refused() {
    let mut out = Vec::new();
    crossing(Motion::Turns(vec![Turn::of(1, 2)])).put(&mut out).expect("written");
    // The ring's first vertex is the point at tick 3; its writing stands
    // after the place, the content and the ring count. Swap it with the
    // second (tick 9) by rewriting the ring from 9.
    let mut ring_bytes = Vec::new();
    fp(3).put(&mut ring_bytes);
    let at = out.windows(ring_bytes.len()).position(|w| w == ring_bytes.as_slice()).expect("the least vertex written");
    let mut nine = Vec::new();
    fp(9).put(&mut nine);
    let mut forged = out.clone();
    forged[at..at + nine.len()].copy_from_slice(&nine);
    forged[at + nine.len()..at + 2 * nine.len()].copy_from_slice(&ring_bytes);
    assert_eq!(Crossing::from_bytes(&forged).err(), Some(CrossingRefusal::RingStart));
}

/// One body carried by `motion` over the frame `(4, 2)`: from the origin
/// `(0, 0)` onto `after`.
fn carried(motion: Vec<Turn>, after: Vec<Turn>) -> Crossing {
    let mut c = crossing(Motion::Turns(motion));
    c.content.before = vec![body()];
    c.content.after = vec![body_at(after)];
    c
}

/// THE RECORD IS VERIFIABLE FROM ITSELF ALONE (ruled 2026-11-06, #160): a
/// content that states no frame is refused `NoFrame`, and a body that
/// states no origin `NoPlace`, naming the body.
#[test]
fn crossing_without_frame_or_place_is_refused_by_name() {
    let mut c = carried(vec![Turn::of(1, 4), Turn::of(1, 2)], vec![Turn::of(1, 4), Turn::of(1, 2)]);
    assert_eq!(c.verify(), Ok(()), "a frame, an origin per body, carried");
    let mut no_frame = c.clone();
    no_frame.content.frame = Vec::new();
    assert_eq!(no_frame.verify(), Err(CrossingRefusal::NoFrame));
    no_frame.content.frame = vec![4, 0];
    assert_eq!(no_frame.verify(), Err(CrossingRefusal::NoFrame), "a phase of period 0 is no frame");
    c.content.after[0].origin = Vec::new();
    assert_eq!(c.verify(), Err(CrossingRefusal::NoPlace { body: 1 }), "counted through the bodies before, then after");
}

/// `verify_turns` RUNS ON THE RECORD: a motion stated as turns carries each
/// body's origin before onto its origin after over the record's own frame;
/// a turn the frame's periods do not hold, or an origin not carried, is
/// refused with its breach named.
#[test]
fn crossing_turns_are_read_by_the_conformal_law() {
    let off = carried(vec![Turn::of(1, 3), Turn::of(1, 2)], vec![Turn::of(1, 3), Turn::of(1, 2)]);
    assert!(matches!(off.verify(), Err(CrossingRefusal::Turns(b)) if matches!(b[..], [TurnBreach::LeavesPeriods { phase: 0, period: 4, .. }])));
    let uncarried = carried(vec![Turn::of(1, 4), Turn::of(1, 2)], vec![Turn::of(2, 4), Turn::of(1, 2)]);
    assert!(matches!(uncarried.verify(), Err(CrossingRefusal::Turns(b)) if matches!(b[..], [TurnBreach::NotCarried { body: 0, .. }])));
    let mut unpaired = carried(vec![Turn::of(1, 4), Turn::of(1, 2)], vec![Turn::of(1, 4), Turn::of(1, 2)]);
    unpaired.content.after.push(body());
    assert_eq!(unpaired.verify(), Err(CrossingRefusal::Unpaired { before: 1, after: 2 }));
}

/// THE REFUSAL RIDES THE BOUNDARY'S ANSWER: written and read back whole,
/// each breach with what it breaks the law by.
#[test]
fn crossing_refusal_round_trips_in_the_boundary() {
    use crate::court::admission::{Boundary, UniverseAct};
    let uncarried = carried(vec![Turn::of(1, 4), Turn::of(1, 2)], vec![Turn::of(2, 4), Turn::of(1, 2)]);
    let why = uncarried.verify().expect_err("not carried");
    for why in [why, CrossingRefusal::Absent, CrossingRefusal::NoPlace { body: 3 }, CrossingRefusal::Unpaired { before: 1, after: 2 }] {
        let b = Boundary::Crossing { act: UniverseAct::Map, domain: [7; 32], why };
        let mut out = Vec::new();
        b.put(&mut out);
        let mut p = 0;
        assert_eq!(Boundary::take(&out, &mut p), Some(b));
        assert_eq!(p, out.len());
    }
}
