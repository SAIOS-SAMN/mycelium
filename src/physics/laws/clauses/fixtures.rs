// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-RELEASE §3a step 3 — THE JOINT-READ FIXTURES' INPUTS, as the proposals
// state them: LAW-PASSAGE v3's five (78.172 §5, adopted 2026-11-05, for the
// joint read) and LAW-COORDINATE v1's (78.173 §5), each an exact input.
//
//! The generators of the passage's fixtures are named by role, as the
//! fixtures write them: plane `k`'s positive unit `e₊ₖ` (squaring to +1,
//! the throat's unit) is base axis `2k − 1`, its negative unit `e₋ₖ`
//! (squaring to −1) base axis `2k`, beside the conformal pair's `e₋` (as
//! `forms`' own claims name them, `forms/src/tests/crossing.rs`). A
//! fixture's two passes are two entries, the second's input the first's
//! object, so each pass is read on its own. The face `δ = H/(H + R)` of
//! 78.172 §5's fixture 6 is a reading of fixture 1's frame states and has
//! no operation in `Fixture` or reading in `Object`: it is read in `forms`
//! alone. The coordinate's fixtures stand at the
//! root `R` (genesis `6a` × 32, epoch `0x92`). Fixture 1b's writings and
//! fixture 3's nesting (a claim over every depth, not an input and an
//! object) are read in `forms` alone, as are fixtures 7–10 (the advance,
//! an event's bell, the merger and fission): `Fixture` has no operation
//! for them and `Object` no reading of a merger or an event's bell, so
//! serving them is a new fixture and object tag on `ACT_LAWS` mode 7, a
//! wire change left to the clause's drafting (78.173 §6).

use super::register::Fixture;
use algebra::{base_axis, Clifford, EM};
use forms::coordinate::{Coordinate, Lane, Local, Origin};
use forms::Q;
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// Plane `k`'s positive unit `e₊ₖ`.
fn positive(k: usize) -> Option<usize> {
    base_axis(2 * k - 1)
}

/// Plane `k`'s negative unit `e₋ₖ`.
fn negative(k: usize) -> Option<usize> {
    base_axis(2 * k)
}

/// The signature of the frame planes `1..=m`: each negative unit
/// negative, beside `e₋`.
fn signature(m: usize) -> usize {
    (1..=m).filter_map(negative).fold(EM, |s, g| s | g)
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

/// The frame state `h·n_Z + r·n_Q = (h + r)·e₊₁ + (h − r)·e₋₁` on plane 1,
/// under the planes `1..=m`.
fn frame(m: usize, h: Q, r: Q) -> Option<Clifford> {
    sum(vec![word(m, &h + &r, &[positive(1)]), word(m, h - r, &[negative(1)])])
}

/// One crossing of the frame state at hold `h` and reach `r` on plane 1,
/// with `t` on `T`.
fn crossed(h: Q, r: Q, t: Q) -> Option<Fixture> {
    Some(Fixture::Crossed { positive: word(1, q(1, 1), &[positive(1)])?, negative: word(1, q(1, 1), &[negative(1)])?, frame: frame(1, h, r)?, flux: t })
}

/// The riding state's pass on plane 1, under the planes `1..=3`.
fn ridden(state: Clifford) -> Option<Fixture> {
    Some(Fixture::Ridden { positive: word(3, q(1, 1), &[positive(1)])?, negative: word(3, q(1, 1), &[negative(1)])?, state })
}

/// LAW-PASSAGE v3's five fixtures (78.172 §5), in order, each pass its own
/// entry: (1) span `3/5` across and back; (2) the throat; (3) a pole;
/// (4) the riding state's parity, across and back; (5) a general frame
/// state across and back. `None` where the algebra holds no generator a
/// fixture names.
pub fn passage_fixtures() -> Option<Vec<Fixture>> {
    // Fixture 4's even state, `q = 3 + 2·e₊₂e₊₃ + e₊₁e₋₂`, and `e₊₁·q =
    // 3·e₊₁ + 2·e₊₁e₊₂e₊₃ + e₋₂`, over three planes.
    let state = sum(vec![word(3, q(3, 1), &[]), word(3, q(2, 1), &[positive(2), positive(3)]), word(3, q(1, 1), &[positive(1), negative(2)])])?;
    let carried =
        sum(vec![word(3, q(3, 1), &[positive(1)]), word(3, q(2, 1), &[positive(1), positive(2), positive(3)]), word(3, q(1, 1), &[negative(2)])])?;
    Some(vec![
        crossed(q(4, 5), q(1, 5), q(0, 1))?,
        crossed(q(1, 5), q(4, 5), q(-6, 5))?,
        crossed(q(1, 2), q(1, 2), q(0, 1))?,
        crossed(q(4, 5), q(0, 1), q(0, 1))?,
        ridden(state)?,
        ridden(carried)?,
        crossed(q(2, 1), q(-1, 3), q(1, 1))?,
        crossed(q(-1, 3), q(2, 1), q(-11, 3))?,
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
