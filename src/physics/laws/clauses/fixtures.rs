// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-RELEASE §3a step 3 — THE JOINT-READ FIXTURES' INPUTS, as the proposals
// state them: LAW-PASSAGE v2's four (78.172 §5, agreed 2026-11-05) and
// LAW-COORDINATE v1's (78.173 §5), each an exact input.
//
//! The generators of the passage's fixtures are named by role, as the
//! fixtures write them: plane `k`'s closed pole `e_Zk` (squaring to +1) is
//! base axis `2k − 1`, its open pole `e_Qk` (squaring to −1) base axis
//! `2k`, beside the conformal pair's `e₋` (as `forms`' own claims name them,
//! `forms/src/tests/crossing.rs`). The coordinate's fixtures stand at the
//! root `R` (genesis `6a` × 32, epoch `0x92`). Fixture 1b's writings and
//! fixture 3's nesting (a claim over every depth, not an input and an
//! object) are read in `forms` alone.

use super::register::Fixture;
use algebra::{base_axis, Clifford, EM};
use forms::coordinate::{Coordinate, Lane, Local, Origin};
use forms::Q;
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// Plane `k`'s closed pole `e_Zk`.
fn closed(k: usize) -> Option<usize> {
    base_axis(2 * k - 1)
}

/// Plane `k`'s open pole `e_Qk`.
fn open(k: usize) -> Option<usize> {
    base_axis(2 * k)
}

/// The signature of the frame planes `1..=m`: each open pole negative,
/// beside `e₋`.
fn signature(m: usize) -> usize {
    (1..=m).filter_map(open).fold(EM, |s, g| s | g)
}

/// `c` times the product of the generators `gens`, in the order written,
/// under the signature of the planes `1..=m`; `None` for a generator the
/// algebra does not hold.
fn word(m: usize, c: Q, gens: &[Option<usize>]) -> Option<Clifford> {
    let sig = signature(m);
    gens.iter().try_fold(Clifford::blade(sig, 0, c).ok()?, |w, g| w.product(&Clifford::blade(sig, (*g)?, q(1, 1)).ok()?).ok())
}

/// The sum of `words`, under one signature.
fn sum(words: Vec<Option<Clifford>>) -> Option<Clifford> {
    let mut it = words.into_iter();
    let first = it.next()??;
    it.try_fold(first, |s, w| s.sum(&w?).ok())
}

/// The direction `h·e_Zk + r·e_Qk` of plane `k`, under the planes `1..=m`.
fn direction(m: usize, k: usize, h: Q, r: Q) -> Option<Clifford> {
    sum(vec![word(m, h, &[closed(k)]), word(m, r, &[open(k)])])
}

/// LAW-PASSAGE v2's four fixtures (78.172 §5), in order: the horizon
/// one-way, even to odd, two boosts leaving a turning, a null pair. `None`
/// where the algebra holds no generator a fixture names.
pub fn passage_fixtures() -> Option<Vec<Fixture>> {
    // Fixture 2's state, `q = 3 + 2·e_Z2Z3 + e_Z1Q2`, over three planes.
    let state = sum(vec![word(3, q(3, 1), &[]), word(3, q(2, 1), &[closed(2), closed(3)]), word(3, q(1, 1), &[closed(1), open(2)])])?;
    Some(vec![
        Fixture::PassedTwice { v: direction(3, 1, q(1, 1), q(1, 1))?, q: state.clone() },
        Fixture::Passed { v: direction(3, 1, q(1, 1), q(2, 1))?, q: state },
        Fixture::Commutator { a: word(2, q(1, 1), &[closed(1), open(1)])?, b: word(2, q(1, 1), &[closed(1), open(2)])? },
        Fixture::Square { x: sum(vec![word(2, q(1, 1), &[closed(1), closed(2)]), word(2, q(1, 1), &[open(1), closed(2)])])? },
    ])
}

/// The root `R`: genesis `6a` × 32, epoch `0x92`.
fn root() -> Origin {
    Origin { genesis: [0x6a; 32], epoch: 0x92, enclosures: vec![] }
}

/// The root enclosed once, in the branch cut at `3:2:0:0:0:0`.
fn enclosed() -> Origin {
    Origin { enclosures: vec![Local { h: 3, lanes: on_s(q(2, 1)), k: 0 }], ..root() }
}

/// The lanes `S:G:T:H` with `s` on S and `g` on G.
fn lanes(s: Q, g: Q) -> [Q; 4] {
    [s, g, q(0, 1), q(0, 1)]
}

/// The lanes with `s` on S alone.
fn on_s(s: Q) -> [Q; 4] {
    lanes(s, q(0, 1))
}

/// The body at `origin`, bell `h`, lanes, `K`.
fn body(origin: &Origin, h: u64, lanes: [Q; 4], k: u64) -> Coordinate {
    Coordinate { origin: origin.clone(), local: Local { h, lanes, k } }
}

/// `1/3 − 10⁻¹⁵`, the value just below a sector bound.
fn below_third() -> Q {
    q(1, 3) - Q::new(BigInt::from(1), BigInt::from(10).pow(15))
}

/// Fixture 6's held bodies: `R/5:i/10` for `i = 0..19`, `R/6:1/2`, the same
/// body at epoch `0x93`, and `R/5:1/2:g` for `g = −1, 1/4, 2`.
fn held() -> Vec<Coordinate> {
    let r = root();
    let mut held: Vec<Coordinate> = (0..20).map(|i| body(&r, 5, on_s(q(i, 10)), 0)).collect();
    held.push(body(&r, 6, on_s(q(1, 2)), 0));
    held.push(body(&Origin { epoch: 0x93, ..root() }, 5, on_s(q(1, 2)), 0));
    held.extend([q(-1, 1), q(1, 4), q(2, 1)].into_iter().map(|g| body(&r, 5, lanes(q(1, 2), g), 0)));
    held
}

/// LAW-COORDINATE v1's fixtures (78.173 §5) with an input and an object:
/// the order (1a, its seven bodies given in reverse), the sexagesimal
/// reading at the eight stated depths (2), `within` on and off a bound (4),
/// origins never compared (5), and the two ranges on one lane (6).
pub fn coordinate_fixtures() -> Vec<Fixture> {
    let r = root();
    let mut ordered = vec![
        body(&r, 5, on_s(q(-1, 2)), 1),
        body(&r, 5, lanes(q(1, 3), q(-1, 1)), 9),
        body(&r, 5, on_s(q(1, 3)), 1),
        body(&r, 5, on_s(q(1, 3)), 2),
        body(&r, 5, on_s(q(1, 2)), 1),
        body(&r, 6, on_s(q(-7, 1)), 0),
        body(&enclosed(), 0, on_s(q(0, 1)), 0),
    ];
    ordered.reverse();
    let mut fixtures = vec![Fixture::Ordered { bodies: ordered }];
    let sectors = [(q(7, 3), 0), (q(7, 3), 1), (q(7, 3), 2), (q(1, 7), 3), (q(360, 7), 2), (q(-1, 3), 1), (q(-1, 7), 1), (below_third(), 6)];
    fixtures.extend(sectors.into_iter().map(|(value, depth)| Fixture::Sector { value, depth }));
    let other = body(&r, 5, on_s(q(1, 3)), 0);
    let sixth = Q::new(BigInt::from(1), BigInt::from(3) * BigInt::from(forms::coordinate::BASE).pow(6));
    let within = [(q(1, 3), 1), (q(7, 20), 1), (below_third(), 1), (q(1, 3) + sixth, 6), (q(2, 5), 0), (q(2, 5), 1)];
    fixtures.extend(within.into_iter().map(|(s, depth)| Fixture::Within { at: body(&r, 7, on_s(s), 4), other: other.clone(), lane: Lane::S, depth }));
    fixtures.push(Fixture::Within { at: other.clone(), other: body(&enclosed(), 5, on_s(q(1, 3)), 0), lane: Lane::S, depth: 1 });
    fixtures.push(Fixture::Range { held: held(), at: body(&r, 5, on_s(q(0, 1)), 0), lane: Lane::S, lo: q(1, 2), hi: q(9, 10) });
    fixtures.push(Fixture::Range { held: held(), at: body(&r, 5, on_s(q(1, 2)), 0), lane: Lane::G, lo: q(0, 1), hi: q(1, 1) });
    fixtures
}
