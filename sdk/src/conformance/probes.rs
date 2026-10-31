// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// THE OBJECTS THE CONFORMANCE READ SPEAKS: two transitions (a lawful turn
// and a still height), a universe's genesis and origin, a manifest, and the
// seats a house lists — each exact, the same at both ends of every codec.

use mycelium::anastome::dialogue::domain::Origin;
use mycelium::anastome::dialogue::endowment::{Manifest, Nuclei};
use mycelium::anastome::dialogue::seat::{Arg, Post, Role, Seat, SeatFn, Seats, TAG_COUNT};
use mycelium::anastome::dialogue::universe::{Genesis, Geometry, Parent};
use mycelium::physics::laws::{BodyState, Transition};
use mycelium::physics::tensegrity::{carried_body, Body};
use mycelium::transducer::cga::{rotator, sphere_at, Mv};
use num_rational::BigRational as Q;

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

/// The (3,4,5) turn in (e₁, e₂): `2 + e₁₂`, a rational rotor.
fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// Two bodies carried by the turn exactly: each nucleus a unit sphere moved
/// by the sandwich, each body by its rigid carriage.
fn carried() -> Option<Vec<(BodyState, BodyState)>> {
    [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|c| {
            let body = Body {
                centre: c.iter().map(|x| q(*x)).collect(),
                mass: q(1),
                moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
            };
            let nucleus = sphere_at(&body.centre, &q(1));
            let after = BodyState { nucleus: turn().sandwich(&nucleus), body: carried_body(&body, &turn())? };
            Some((BodyState { nucleus, body }, after))
        })
        .collect()
}

/// A LAWFUL TRANSITION (the turn carrying two bodies, heights 7 → 8) and a
/// STILL one (the same at 8 → 8, refused on time alone). `None` where the
/// turn carries no body rigidly, which it does.
pub(crate) fn transitions() -> Option<[Transition; 2]> {
    let lawful = Transition { motion: turn(), heights: (7, 8), bodies: carried()?, hues: Vec::new(), fluxes: Vec::new() };
    let still = Transition { heights: (8, 8), ..lawful.clone() };
    Some([lawful, still])
}

/// A UNIVERSE'S GENESIS: a curved space of radius 1, a root unwitnessed.
pub(crate) fn genesis() -> Genesis {
    Genesis { geometry: Geometry::Curved((1, 1)), parent: Parent::Unwitnessed }
}

/// A UNIVERSE'S ORIGIN at version 2, committing to the citation hash of the
/// record it names.
pub(crate) fn origin(domain: [u8; 32]) -> Origin {
    let commitment = mycelium::chronometry::payload_hash(b"mycelium conformance universe");
    Origin { domain, kind: 0, version: 2, commitment, universe: Some(genesis()) }
}

/// A MANIFEST of one kind of nuclei, one hydrogen nucleus, under a nonce.
pub(crate) fn manifest(domain: [u8; 32]) -> Manifest {
    let nuclei: Nuclei = vec![(1, num_bigint::BigUint::from(1u8))];
    Manifest { domain, nuclei, nonce: mycelium::chronometry::payload_hash(b"mycelium conformance manifest") }
}

/// THE SEATS A HOUSE LISTS at epoch 0: one seat of role 0 acting on the
/// domain's key, beating once per unit, offering one function of one count.
pub(crate) fn seats(domain: [u8; 32]) -> Post {
    let seat = Seat { seat: 0, role: Role(0), on: domain, beat: Arg::Q { num: 1, den: 1 }, fns: vec![SeatFn { id: 0, tags: vec![TAG_COUNT] }] };
    Post::Seats(Seats { domain, sub_origin: None, epoch: 0, seats: vec![seat] })
}
