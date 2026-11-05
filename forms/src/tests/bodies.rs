// The fixtures' shared inputs (proposal 78.173 §5): the root `R`, the keys
// of the one-component lanes, and bodies built from them.

use crate::coordinate::*;
use crate::Q;

pub fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// The fixtures' universe root: genesis `6a` × 32, epoch `0x92`.
pub fn root() -> Origin {
    Origin { genesis: [0x6a; 32], epoch: 0x92, enclosures: vec![] }
}

/// The axis `letter` `index`.
pub fn axis(letter: char, index: u32) -> Axis {
    Axis::of(letter, index).expect("a writable axis")
}

/// The key of the one axis `letter` `index`.
pub fn on(letter: char, index: u32) -> Key {
    Key::axis(axis(letter, index))
}

/// The generator of the plane of the axes `a` and `b`, in that order.
pub fn plane(a: (char, u32), b: (char, u32)) -> Key {
    Key::generator(axis(a.0, a.1), axis(b.0, b.1)).expect("two axes")
}

/// The one-component lanes' keys: S, G and T each on its own first axis
/// (`e_S1`, `e_G1`, `e_T1`), H on the frame plane `e_Z1Q1`.
pub fn keys() -> [Key; 4] {
    [on('S', 1), on('G', 1), on('T', 1), plane(('Z', 1), ('Q', 1))]
}

/// One-component lanes `S:G:T:H`, H in degrees.
pub fn lanes(s: Q, g: Q, t: Q, h: Q) -> Lanes {
    let [ks, kg, kt, kh] = keys();
    Lanes::base(Vector::one(ks, s), Vector::one(kg, g), Vector::one(kt, t), Vector::one(kh, h)).expect("the base lanes")
}

/// Lanes with `s` on S and nothing else.
pub fn on_s(s: Q) -> Lanes {
    lanes(s, q(0, 1), q(0, 1), q(0, 1))
}

/// The body at `origin`, bell `h`, `lanes`, `K` as stated.
pub fn body(origin: &Origin, h: u64, lanes: Lanes, k: Vec<u64>) -> Coordinate {
    Coordinate { origin: origin.clone(), local: Local { h, lanes, k } }
}

/// The one-component value of `lane` on `c`.
pub fn value<'a>(c: &'a Coordinate, lane: &Lane) -> &'a Q {
    let v = c.lane(lane).expect("a base lane");
    match v.components() {
        [(_, x)] => x,
        other => panic!("one component, not {other:?}"),
    }
}

/// Bodies at `R`, bell 5, S = G = T = 0, and H on the two frame planes
/// `e_Z1Q1` and `e_Z2Q2`, in degrees, `K` read per component.
pub fn planar(h1: Q, h2: Q) -> Coordinate {
    let [ks, kg, kt, _] = keys();
    let h = Vector::of(vec![(plane(('Z', 1), ('Q', 1)), h1), (plane(('Z', 2), ('Q', 2)), h2)]).expect("two planes");
    let lanes = Lanes::base(Vector::one(ks, q(0, 1)), Vector::one(kg, q(0, 1)), Vector::one(kt, q(0, 1)), h).expect("the base lanes");
    body(&root(), 5, lanes, vec![0, 0])
}
