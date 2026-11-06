// ROADMAP track 25 X1, issue #160 — THE CROSSING RECORD: the four parts of
// a crossing written out and read back to exact values, and a record that
// carries only an address refused `AddressOnly`.

use crate::anastome::dialogue::crossing_record::{put_address_only, Clauses, Content, Crossing, CrossingRefusal, Motion, Place, TurnBody};
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

/// A body of two phases, its boundary breaking at quarter and half turns,
/// taking three eighths of the frame.
fn body() -> TurnBody {
    TurnBody { breakpoints: vec![vec![Turn::of(1, 4), Turn::of(3, 4)], vec![Turn::of(1, 2)]], extent: q(3, 8) }
}

fn crossing(motion: Motion) -> Crossing {
    Crossing {
        place: Place { at: fp(12), parents: vec![parent(10), parent(11)] },
        content: Content { before: vec![body()], after: vec![body(), body()], motion },
        holonomy: vec![rotator(&q(2, 1), &q(1, 1))],
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
