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

/// Plane `k`'s frame unit of square `square`: `e₊ₖ` or `e₋ₖ`.
pub fn unit(square: Square, k: u32) -> Axis {
    Axis::unit(square, k).expect("a plane's unit")
}

/// The generator of the units `a` and `b`, each `(square, plane)`, in that
/// order.
pub fn units(a: (Square, u32), b: (Square, u32)) -> Key {
    Key::generator(unit(a.0, a.1), unit(b.0, b.1)).expect("two units")
}

/// The continuum's turning `e₊ₖe₊ₗ`, in that orientation.
pub fn turning(k: u32, l: u32) -> Key {
    units((Square::Positive, k), (Square::Positive, l))
}

/// The one-component lanes' keys: S, G and T each on its own first axis
/// (`e_S1`, `e_G1`, `e_T1`), H on the turning about axis 1, `e₊₂e₊₃`
/// (`e_+2+3`).
pub fn keys() -> [Key; 4] {
    [on('S', 1), on('G', 1), on('T', 1), turning(2, 3)]
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

/// The turnings about axes 1 and 2, `e₊₂e₊₃` (`e_+2+3`) and `e₊₃e₊₁`
/// (`e_+3+1`), in their key order.
pub fn about() -> (Key, Key) {
    (turning(2, 3), turning(3, 1))
}

/// Bodies at `R`, bell 5, S = G = T = 0, and H on the turnings about axes
/// 1 and 2, `e₊₂e₊₃` and `e₊₃e₊₁`, in degrees, `K` read per component.
pub fn planar(h1: Q, h2: Q) -> Coordinate {
    let [ks, kg, kt, _] = keys();
    let (z1, z2) = about();
    let h = Vector::of(vec![(z1, h1), (z2, h2)]).expect("two planes");
    let lanes = Lanes::base(Vector::one(ks, q(0, 1)), Vector::one(kg, q(0, 1)), Vector::one(kt, q(0, 1)), h).expect("the base lanes");
    body(&root(), 5, lanes, vec![0, 0])
}
