// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE FRAME AS A FORM (the operator, 2026-10-21: "fix the grid and upgrade
//! to forms: forms bend, twist, stretch, expand, and contract"; both readings
//! ruled — frame maps are conformal forms, and every law form is read under
//! them). A reading moves between two observers' frames (A6) only by an
//! EXACT, INVERTIBLE conformal map of ℚⁿ, fixed before any joint read —
//! never by a floor on a grid, which rounds a reading off and cannot be
//! undone. A frame is composed of five moves:
//!
//! ```text
//!     STRETCH    x ↦ x + t                         a translation
//!     TWIST      x ↦ x − 2 (v·x)/(v·v) v            a reflection; two are a turn
//!     EXPAND     x ↦ ρ x,  ρ > 1                    a dilation outward
//!     CONTRACT   x ↦ x / ρ, ρ > 1                   a dilation inward
//!     BEND       x ↦ (x + b|x|²)/(1 + 2b·x + |b|²|x|²)   a transversion
//!     TURN       (plane, degrees)                     an ordered turn, held as its angle
//! ```
//!
//! THE TURN IN DEGREES (xylarium's and netstratum's rulings: angles in exact
//! degrees, no half-angle tangents, no rational-cosine rotors): held as its
//! plane and its angle, undone by its negation, composed in one plane by
//! adding degrees; it carries a position only where its cosine and sine are
//! both rational — a whole number of quarter turns — and is refused by name
//! everywhere else, never enclosed or rounded.
//!
//! Every move is exact over ℚ and has an exact inverse (a stretch by `−t`,
//! the same reflection, the other dilation, a bend by `−b`), so a frame's
//! inverse is its moves undone in reverse order and a reading carried there
//! and back returns bit for bit. A twist is held as its reflections
//! (Cartan–Dieudonné), never as a matrix carrying the action. A bend sends
//! the point at its pole to infinity: there it is refused by name, never
//! rounded.
//!
//! A FRAME ACTS ON A READING'S OWN COORDINATES, whatever the observer reads
//! them in — never on a chart laid over them. Read as Euclidean positions,
//! two rational reflections turn by an angle whose cosine is rational, which
//! is a rational number of degrees only at Niven's angles; so a frame does
//! not carry an arbitrary rational-degree turning of positions, and does not
//! claim to. A reading whose coordinates ARE angles in degrees (a sphere's
//! place as its polar share and phases, netstratum's) is turned EXACTLY by a
//! stretch on its phase coordinate — Δ° added — and mirrored by a twist on
//! it (θ ↦ −θ), with no stereographic chart; a bend is conformal only in
//! flat coordinates, and a universe whose geometry is not flat leaves it
//! out of its own frames (A7).

use crate::Q;
use num_traits::{One, Zero};

/// WHY A FRAME OR A READING CARRIED BY IT IS REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// A move's vector, or the reading, over a different number of axes.
    Unmatched,
    /// A dilation by a ratio not above one (an expand or a contract is named
    /// by its direction, its ratio always above one).
    Ratio,
    /// A reflection in the zero vector: no hyperplane.
    NoMirror,
    /// The reading stands at a bend's pole: it is carried to infinity.
    AtPole,
    /// A turn whose degrees carry a position only through an irrational
    /// cosine or sine (every angle but a whole quarter turn): refused by
    /// name, never enclosed or rounded. The turn itself stands exact.
    Inexact,
    /// A turn's plane names an axis the reading does not have, or one axis
    /// twice.
    NoPlane,
}

/// ONE MOVE OF A FRAME.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Move {
    Stretch(Vec<Q>),
    /// A reflection in the hyperplane normal to this vector; two make a turn.
    Twist(Vec<Q>),
    Expand(Q),
    Contract(Q),
    Bend(Vec<Q>),
    /// AN ORDERED TURN IN DEGREES (xylarium's "the flat retired",
    /// netstratum's "laws as angles"): in the plane of axes `(i, j)`, from
    /// `i` toward `j`, by exactly `degrees` — held as the angle, never as a
    /// rational-cosine rotor or a half-angle tangent. Consecutive turns in
    /// one plane compose by adding their degrees.
    Turn { plane: (usize, usize), degrees: Q },
}

fn dot(a: &[Q], b: &[Q]) -> Q {
    a.iter().zip(b).fold(Q::zero(), |s, (x, y)| s + x * y)
}

impl Move {
    /// The move that undoes this one, exactly.
    pub fn inverse(&self) -> Move {
        match self {
            Move::Stretch(t) => Move::Stretch(t.iter().map(|x| -x.clone()).collect()),
            Move::Twist(v) => Move::Twist(v.clone()),
            Move::Expand(r) => Move::Contract(r.clone()),
            Move::Contract(r) => Move::Expand(r.clone()),
            Move::Bend(b) => Move::Bend(b.iter().map(|x| -x.clone()).collect()),
            Move::Turn { plane, degrees } => Move::Turn { plane: *plane, degrees: -degrees.clone() },
        }
    }

    /// The point `x` carried by this move.
    pub fn apply(&self, x: &[Q]) -> Result<Vec<Q>, Refused> {
        let matched = |v: &[Q]| match v.len() == x.len() {
            true => Ok(()),
            false => Err(Refused::Unmatched),
        };
        match self {
            Move::Stretch(t) => {
                matched(t)?;
                Ok(x.iter().zip(t).map(|(a, b)| a + b).collect())
            }
            Move::Twist(v) => {
                matched(v)?;
                let vv = dot(v, v);
                if vv.is_zero() {
                    return Err(Refused::NoMirror);
                }
                let k = Q::from_integer(2.into()) * dot(v, x) / vv;
                Ok(x.iter().zip(v).map(|(a, b)| a - &k * b).collect())
            }
            Move::Expand(r) | Move::Contract(r) => {
                if r <= &Q::one() {
                    return Err(Refused::Ratio);
                }
                let s = match self {
                    Move::Expand(_) => r.clone(),
                    _ => Q::one() / r,
                };
                Ok(x.iter().map(|a| a * &s).collect())
            }
            Move::Bend(b) => {
                matched(b)?;
                let xx = dot(x, x);
                let den = Q::one() + Q::from_integer(2.into()) * dot(b, x) + dot(b, b) * &xx;
                if den.is_zero() {
                    return Err(Refused::AtPole);
                }
                Ok(x.iter().zip(b).map(|(a, c)| (a + c * &xx) / &den).collect())
            }
            Move::Turn { plane: (i, j), degrees } => {
                if i == j || *i >= x.len() || *j >= x.len() {
                    return Err(Refused::NoPlane);
                }
                let (c, s) = quarter_turn(degrees).ok_or(Refused::Inexact)?;
                let mut out = x.to_vec();
                let (xi, xj) = (x[*i].clone(), x[*j].clone());
                out[*i] = &xi * &c - &xj * &s;
                out[*j] = &xi * &s + &xj * &c;
                Ok(out)
            }
        }
    }
}

/// The cosine and sine of a turn by `degrees`, where both are rational: a
/// whole number of quarter turns. `None` everywhere else (Niven: a
/// rational-degree angle has rational cosine and sine together only at the
/// quarter turns).
fn quarter_turn(degrees: &Q) -> Option<(Q, Q)> {
    let quarters = degrees / Q::from_integer(90.into());
    if !quarters.is_integer() {
        return None;
    }
    let four = num_bigint::BigInt::from(4);
    let k = ((quarters.to_integer() % &four) + &four) % &four;
    let (one, zero) = (Q::one(), Q::zero());
    match u8::try_from(k).ok()? {
        0 => Some((one, zero)),
        1 => Some((zero, one)),
        2 => Some((-one, zero)),
        3 => Some((zero, -one)),
        _ => None,
    }
}

/// A FRAME: its moves, applied in order. The empty frame is the identity.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Frame {
    moves: Vec<Move>,
}

impl Frame {
    /// The identity frame.
    pub fn identity() -> Frame {
        Frame::default()
    }

    /// This frame, then `m`.
    pub fn then(mut self, m: Move) -> Frame {
        self.moves.push(m);
        self
    }

    /// This frame, then every move of `later`.
    pub fn compose(mut self, later: &Frame) -> Frame {
        self.moves.extend(later.moves.iter().cloned());
        self
    }

    /// The frame that undoes this one: its moves undone, in reverse order.
    pub fn inverse(&self) -> Frame {
        Frame { moves: self.moves.iter().rev().map(Move::inverse).collect() }
    }

    /// The moves, as held.
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }

    /// A READING CARRIED INTO THIS FRAME, exactly, move by move —
    /// consecutive turns in one plane composed first by adding their
    /// degrees, so a run that sums to whole quarter turns carries exactly
    /// even where its parts would not.
    pub fn carry(&self, x: &[Q]) -> Result<Vec<Q>, Refused> {
        self.composed().iter().try_fold(x.to_vec(), |p, m| m.apply(&p))
    }

    /// The moves with each run of turns in one plane composed into one turn.
    fn composed(&self) -> Vec<Move> {
        let mut out: Vec<Move> = Vec::with_capacity(self.moves.len());
        for m in &self.moves {
            match (out.last_mut(), m) {
                (Some(Move::Turn { plane: p, degrees: d }), Move::Turn { plane, degrees }) if p == plane => *d += degrees,
                _ => out.push(m.clone()),
            }
        }
        out
    }
}

/// A LAW FORM READ IN A FRAME: every form whose reading is a point of ℚⁿ is
/// read in any observer's frame by carrying that reading — never a grid.
pub trait ReadIn {
    /// The form's reading in its own frame.
    fn reading_point(&self) -> Vec<Q>;

    /// The form's reading carried into `frame`.
    fn read_in(&self, frame: &Frame) -> Result<Vec<Q>, Refused> {
        frame.carry(&self.reading_point())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(n: i64, d: i64) -> Q {
        Q::new(n.into(), d.into())
    }

    fn v(xs: &[(i64, i64)]) -> Vec<Q> {
        xs.iter().map(|&(n, d)| q(n, d)).collect()
    }

    /// EVERY MOVE IS UNDONE EXACTLY: carried by a move and by its inverse, a
    /// reading returns bit for bit — stretch, twist, expand, contract, bend.
    #[test]
    fn every_move_is_undone_exactly() {
        let x = v(&[(3, 7), (-2, 5), (1, 1)]);
        let moves = [
            Move::Stretch(v(&[(1, 2), (0, 1), (-4, 3)])),
            Move::Twist(v(&[(1, 1), (2, 1), (2, 1)])),
            Move::Expand(q(5, 2)),
            Move::Contract(q(7, 3)),
            Move::Bend(v(&[(1, 3), (-1, 4), (0, 1)])),
        ];
        for m in moves {
            let there = m.apply(&x).expect("carried");
            assert_eq!(m.inverse().apply(&there), Ok(x.clone()), "{m:?} undone");
        }
    }

    /// A FRAME AND ITS INVERSE ARE THE IDENTITY: five moves composed, a
    /// reading carried there and back returns exactly; the composite of a
    /// frame with its inverse carries every reading to itself.
    #[test]
    fn a_frame_and_its_inverse_are_the_identity() {
        let frame = Frame::identity()
            .then(Move::Bend(v(&[(1, 5), (1, 7)])))
            .then(Move::Twist(v(&[(3, 1), (4, 1)])))
            .then(Move::Twist(v(&[(1, 1), (0, 1)])))
            .then(Move::Expand(q(9, 4)))
            .then(Move::Stretch(v(&[(-1, 3), (2, 1)])))
            .then(Move::Contract(q(3, 2)));
        for x in [v(&[(1, 2), (1, 3)]), v(&[(-5, 2), (7, 11)]), v(&[(0, 1), (0, 1)])] {
            let there = frame.carry(&x).expect("carried");
            assert_eq!(frame.inverse().carry(&there), Ok(x.clone()), "there and back");
            assert_eq!(frame.clone().compose(&frame.inverse()).carry(&x), Ok(x), "the composite is the identity");
        }
    }

    /// TWO TWISTS ARE A TURN: reflections in two mirrors at a right angle
    /// turn the plane by 180°, so a point goes to its antipode; each twist
    /// keeps every distance from the origin.
    #[test]
    fn two_twists_are_a_turn() {
        let half_turn = Frame::identity().then(Move::Twist(v(&[(1, 1), (0, 1)]))).then(Move::Twist(v(&[(0, 1), (1, 1)])));
        let x = v(&[(3, 5), (-2, 7)]);
        assert_eq!(half_turn.carry(&x), Ok(v(&[(-3, 5), (2, 7)])), "the antipode");
        let one = Move::Twist(v(&[(2, 1), (1, 1)])).apply(&x).expect("reflected");
        assert_eq!(dot(&one, &one), dot(&x, &x), "a twist keeps the distance");
    }

    /// AN ORDERED TURN IN DEGREES: held exactly as its angle, undone by its
    /// negation; a quarter turn carries a position exactly; a turn by 30°
    /// alone is refused by name — no cosine of it is rational — while three
    /// of them in one plane compose to 90° and carry exactly.
    #[test]
    fn a_turn_is_held_in_degrees_and_carried_where_exact() {
        let x = v(&[(3, 5), (-2, 7), (1, 1)]);
        let quarter = Move::Turn { plane: (0, 1), degrees: q(90, 1) };
        assert_eq!(quarter.apply(&x), Ok(v(&[(2, 7), (3, 5), (1, 1)])), "a quarter turn from axis 0 toward axis 1");
        assert_eq!(quarter.inverse().apply(&quarter.apply(&x).expect("turned")), Ok(x.clone()), "undone by its negation");
        let thirty = Move::Turn { plane: (0, 1), degrees: q(30, 1) };
        assert_eq!(thirty.apply(&x), Err(Refused::Inexact), "30° alone: refused, never rounded");
        let three = Frame::identity().then(thirty.clone()).then(thirty.clone()).then(thirty);
        assert_eq!(three.carry(&x), quarter.apply(&x), "three turns of 30° in one plane compose to 90°, exactly");
        assert_eq!(Move::Turn { plane: (1, 1), degrees: q(90, 1) }.apply(&x), Err(Refused::NoPlane));
    }

    /// REFUSED BY NAME: a bend's pole carried to infinity, a mirror of zero, a
    /// ratio not above one, axes unmatched — never rounded.
    #[test]
    fn what_no_frame_carries_is_refused_by_name() {
        // The pole of a bend by b is x = −b/|b|².
        assert_eq!(Move::Bend(v(&[(1, 2), (0, 1)])).apply(&v(&[(-2, 1), (0, 1)])), Err(Refused::AtPole));
        assert_eq!(Move::Twist(v(&[(0, 1), (0, 1)])).apply(&v(&[(1, 1), (1, 1)])), Err(Refused::NoMirror));
        assert_eq!(Move::Expand(q(1, 1)).apply(&v(&[(1, 1)])), Err(Refused::Ratio));
        assert_eq!(Move::Contract(q(1, 2)).apply(&v(&[(1, 1)])), Err(Refused::Ratio));
        assert_eq!(Move::Stretch(v(&[(1, 1)])).apply(&v(&[(1, 1), (1, 1)])), Err(Refused::Unmatched));
    }
}
