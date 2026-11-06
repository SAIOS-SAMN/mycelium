// MU-RELEASE §3a — the register serves LAW-COUPLING v1's fixtures as the
// clause `FORMS-COUPLING` v1 at kernel standing, and the hypha's
// reconstruction of each reaches the object 78.175 §5 states.

use crate::physics::laws::clauses::*;
use forms::coordinate::{Axis, Key, Square};
use forms::coupling::{Dipole, Field, Ledger, Meets, Pull, Taken};
use forms::Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

fn turning(k: u32, l: u32) -> Key {
    Key::generator(Axis::unit(Square::Positive, k).expect("a unit"), Axis::unit(Square::Positive, l).expect("a unit")).expect("a turning")
}

/// The objects the coupling clause reaches, in the fixtures' order.
fn reached() -> (Standing, Vec<CouplingObject>) {
    let held = register().expect("the register stands");
    let r = held.into_iter().find(|r| r.clause.id.law == forms::coupling::LAW_ID).expect("the coupling clause");
    let objects = r
        .reached
        .into_iter()
        .map(|o| match o {
            Object::Coupling(c) => c,
            other => panic!("a coupling object, not {other:?}"),
        })
        .collect();
    (r.clause.standing, objects)
}

/// 78.175 §5 REACHED, in order: the six parcels' dipoles; hydrogen and the
/// kind `(4/3, 1)` founded on three planes, hydrogen bonded on `e₊₁e₊₂`
/// and peeled back; the nine pulls on frames (the lock 749/3649, the split
/// 719/3643, alone, a throat neighbour, Δ 45° 739/3647, 319/1619,
/// 3940299/3940300, the pure Z pole as neighbour and as body, both
/// unmoved); the three neighbour readings; light on each face; the two
/// ledgers that close and the one refused with residue 1. The clause
/// stands at kernel standing.
#[test]
fn register_reaches_stated_coupling_objects() {
    let (standing, objects) = reached();
    assert_eq!(standing, Standing::Kernel);
    let dipole = |p: (i64, i64), o: (i64, i64)| CouplingObject::Dipole(Dipole { pole: q(p.0, p.1), open: q(o.0, o.1) });
    let frames = |planes: [(i64, i64, i64, i64); 3]| {
        let keys = [turning(2, 3), turning(3, 1), turning(1, 2)];
        CouplingObject::Frames(keys.into_iter().zip(planes).map(|(k, (a, b, c, d))| (k, q(a, b), q(c, d))).collect())
    };
    let pulled = |field: Field, gap: Q, push: Q, rate: Q| CouplingObject::Pull { field, pull: Pull { gap, push, rate } };
    let held = |f: Q| Field::Held { holding: q(1, 1), field: f };
    let meeting = |reach: Q, meets: Meets| CouplingObject::Meeting(forms::coupling::Meeting { separation: q(1, 1), reach, meets });
    let hydrogen = frames([(2, 3, 1, 3); 3]);
    let stated = vec![
        dipole((1, 2), (1, 4)),
        dipole((1, 3), (1, 3)),
        dipole((0, 1), (1, 2)),
        dipole((-1, 1), (1, 1)),
        dipole((1, 1), (0, 1)),
        dipole((-1, 3), (2, 3)),
        hydrogen.clone(),
        frames([(1, 1, 1, 3); 3]),
        frames([(2, 3, 1, 3), (2, 3, 1, 3), (1, 1, 0, 1)]),
        hydrogen,
        pulled(held(q(-4, 27)), q(1, 3), q(4, 729), q(749, 3649)),
        pulled(held(q(2, 27)), q(1, 3), q(-2, 729), q(719, 3643)),
        pulled(Field::Alone, q(1, 3), q(0, 1), q(1, 5)),
        pulled(held(q(0, 1)), q(1, 3), q(0, 1), q(1, 5)),
        pulled(held(q(-2, 27)), q(1, 3), q(2, 729), q(739, 3647)),
        pulled(held(q(2, 27)), q(1, 3), q(-1, 324), q(319, 1619)),
        pulled(held(q(-1, 1)), q(-19800, 19801), q(19800, 19801), q(3_940_299, 3_940_300)),
        pulled(held(q(0, 1)), q(1, 3), q(0, 1), q(1, 5)),
        pulled(held(q(-4, 27)), q(1, 3), q(0, 1), q(1, 5)),
        meeting(q(1, 1), Meets::Bound),
        meeting(q(9, 10), Meets::Apart),
        meeting(q(3, 2), Meets::Within),
        CouplingObject::Taken(Taken { received: q(6, 1), drained: q(2, 1) }),
        CouplingObject::Taken(Taken { received: q(2, 1), drained: q(6, 1) }),
        CouplingObject::Ledger(Ledger { made: q(8, 1), received: q(6, 1), drained: q(2, 1) }),
        CouplingObject::Ledger(Ledger { made: q(8, 1), received: q(2, 1), drained: q(6, 1) }),
        CouplingObject::Unbalanced { made: q(8, 1), received: q(6, 1), drained: q(1, 1), residue: q(1, 1) },
    ];
    assert_eq!(objects, stated);
}
