// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-RELEASE §3a steps 3–5 — LAW-COORDINATE FIXTURE 11b AS A CONCORDANCE
// CLAUSE (proposal 78.173 §5, read and stated by netstratum 2026-11-06;
// `FORMS-TURNING` v1, at kernel standing): each fixture's input, the
// value it reaches through `forms::turning`, and the inputs written out in
// exact bytes.
//
//! A fixture is one operation of the composition law on exact values of
//! `Cl(p, q)`: a rate's motion (`motion`), a motion's generator
//! (`generator`), two motions composed (`compose`), and two rates composed
//! (`compose_rates`). Each reaches a value, served as the register's
//! `Object::Multivector`, so no object tag is new. The motions composed are
//! stated as the fixture writes them, never computed by the hypha before
//! serving. Plane `k`'s positive unit `e₊ₖ` (squaring to +1) is base axis
//! `2k − 1` and its negative unit `e₋ₖ` (squaring to −1) base axis `2k`.
//! The refusals, the closed forms on one plane and the certified degree
//! sectors are read in `forms` alone (`forms/src/tests/turning.rs`).

use super::writing::{put_clifford, take_clifford, take_u8};
use algebra::{base_axis, Clifford};
use forms::turning::{compose, compose_rates, generator, motion, same};
use forms::Q;

/// ONE FIXTURE 11b INPUT, exact.
#[derive(Clone, Debug)]
pub enum TurningFixture {
    /// The motion of the rate `rate`, `(1 + B)(1 − B)⁻¹`.
    Motion { rate: Clifford },
    /// The generator of the motion `motion`, `(R − 1)(R + 1)⁻¹`.
    Generator { motion: Clifford },
    /// Two motions composed, `first·then`.
    Composed { first: Clifford, then: Clifford },
    /// Two rates composed: the generator of their motions' product.
    Rates { first: Clifford, then: Clifford },
}

impl PartialEq for TurningFixture {
    fn eq(&self, o: &TurningFixture) -> bool {
        match (self, o) {
            (TurningFixture::Motion { rate }, TurningFixture::Motion { rate: r }) => same(rate, r),
            (TurningFixture::Generator { motion }, TurningFixture::Generator { motion: m }) => same(motion, m),
            (TurningFixture::Composed { first, then }, TurningFixture::Composed { first: f, then: t })
            | (TurningFixture::Rates { first, then }, TurningFixture::Rates { first: f, then: t }) => same(first, f) && same(then, t),
            _ => false,
        }
    }
}

impl Eq for TurningFixture {}

/// THE HYPHA'S RECONSTRUCTION OF ONE FIXTURE through `forms::turning`;
/// `None` where the law refuses the input.
pub fn reached(f: &TurningFixture) -> Option<Clifford> {
    match f {
        TurningFixture::Motion { rate } => motion(rate).ok(),
        TurningFixture::Generator { motion } => generator(motion).ok(),
        TurningFixture::Composed { first, then } => compose(first, then).ok(),
        TurningFixture::Rates { first, then } => compose_rates(first, then).ok(),
    }
}

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// A frame unit's mask: `e₊ₖ` for `positive`, else `e₋ₖ`.
fn unit(positive: bool, k: usize) -> Option<usize> {
    match positive {
        true => base_axis(2 * k - 1),
        false => base_axis(2 * k),
    }
}

/// The signature of the frame planes `1..=3`: each negative unit.
fn signature() -> Option<usize> {
    (1..=3).try_fold(0, |s, k| Some(s | unit(false, k)?))
}

/// `c` times the product of the units `(positive, k)`, in the order
/// written.
fn word(c: Q, units: &[(bool, usize)]) -> Option<Clifford> {
    let sig = signature()?;
    units.iter().try_fold(Clifford::blade(sig, 0, c).ok()?, |w, (p, k)| w.product(&Clifford::blade(sig, unit(*p, *k)?, q(1, 1)).ok()?).ok())
}

/// The sum of the words `(c, units)`.
fn sum(words: &[(Q, &[(bool, usize)])]) -> Option<Clifford> {
    words.iter().try_fold(word(q(0, 1), &[])?, |s, (c, units)| s.sum(&word(c.clone(), units)?).ok())
}

/// FIXTURE 11b's INPUTS (78.173 §5), in order: (i) planes sharing a unit,
/// `½e₊₁e₊₂` and `⅓e₊₂e₊₃`, their motions, both products and both
/// generators read back; (ii) disjoint planes, `½e₊₁e₊₂` and `⅓e₋₁e₋₂`,
/// the motion of the second, both products and the generator; (iii) one
/// plane, `½` then `⅓` on the elliptic `e₊₁e₊₂` and on the hyperbolic
/// `e₊₁e₋₁`. `None` where the algebra holds no unit a fixture names.
pub fn turning_fixtures() -> Option<Vec<TurningFixture>> {
    let (p1, p2, p3, n1, n2) = ((true, 1), (true, 2), (true, 3), (false, 1), (false, 2));
    let r1 = sum(&[(q(3, 5), &[]), (q(4, 5), &[p1, p2])])?;
    let r2 = sum(&[(q(4, 5), &[]), (q(3, 5), &[p2, p3])])?;
    let shared = |s: i64| sum(&[(q(12, 25), &[]), (q(16, 25), &[p1, p2]), (q(s * 12, 25), &[p1, p3]), (q(9, 25), &[p2, p3])]);
    let r2_apart = sum(&[(q(4, 5), &[]), (q(3, 5), &[n1, n2])])?;
    let apart = sum(&[(q(12, 25), &[]), (q(16, 25), &[p1, p2]), (q(9, 25), &[n1, n2]), (q(12, 25), &[p1, p2, n1, n2])])?;
    Some(vec![
        TurningFixture::Motion { rate: word(q(1, 2), &[p1, p2])? },
        TurningFixture::Motion { rate: word(q(1, 3), &[p2, p3])? },
        TurningFixture::Composed { first: r1.clone(), then: r2.clone() },
        TurningFixture::Composed { first: r2, then: r1.clone() },
        TurningFixture::Generator { motion: shared(1)? },
        TurningFixture::Generator { motion: shared(-1)? },
        TurningFixture::Motion { rate: word(q(1, 3), &[n1, n2])? },
        TurningFixture::Composed { first: r1.clone(), then: r2_apart.clone() },
        TurningFixture::Composed { first: r2_apart, then: r1 },
        TurningFixture::Generator { motion: apart },
        TurningFixture::Rates { first: word(q(1, 2), &[p1, p2])?, then: word(q(1, 3), &[p1, p2])? },
        TurningFixture::Rates { first: word(q(1, 2), &[p1, n1])?, then: word(q(1, 3), &[p1, n1])? },
    ])
}

/// ONE FIXTURE'S WRITING: its operation's byte (0 a motion, 1 a generator,
/// 2 two motions composed, 3 two rates composed), then its values.
pub fn put_turning_fixture(out: &mut Vec<u8>, f: &TurningFixture) {
    match f {
        TurningFixture::Motion { rate } => {
            out.push(0);
            put_clifford(out, rate);
        }
        TurningFixture::Generator { motion } => {
            out.push(1);
            put_clifford(out, motion);
        }
        TurningFixture::Composed { first, then } => {
            out.push(2);
            put_clifford(out, first);
            put_clifford(out, then);
        }
        TurningFixture::Rates { first, then } => {
            out.push(3);
            put_clifford(out, first);
            put_clifford(out, then);
        }
    }
}

/// One fixture read back by its operation's byte.
pub fn take_turning_fixture(b: &[u8], p: &mut usize) -> Option<TurningFixture> {
    Some(match take_u8(b, p)? {
        0 => TurningFixture::Motion { rate: take_clifford(b, p)? },
        1 => TurningFixture::Generator { motion: take_clifford(b, p)? },
        2 => TurningFixture::Composed { first: take_clifford(b, p)?, then: take_clifford(b, p)? },
        3 => TurningFixture::Rates { first: take_clifford(b, p)?, then: take_clifford(b, p)? },
        _ => return None,
    })
}
