// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-RELEASE §3a step 3 — THE JOINT-READ FIXTURES' INPUTS, as the proposals
// state them: LAW-PASSAGE v3's five (78.172 §5, adopted 2026-11-05, for the
// joint read) and LAW-COORDINATE v4's (78.173 §5), each an exact input.
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
//! alone. The coordinate's fixtures stand at the root `R` (genesis `6a` ×
//! 32, epoch `0x92`), its one-component lanes keyed `e_S1`, `e_G1`, `e_T1`
//! and H's turning about axis 1, `e₊₂e₊₃` (`e_+2+3`), H's two-plane bodies
//! on the turnings about axes 1 and 2, `e₊₂e₊₃` and `e₊₃e₊₁`, in degrees
//! (`FORMS-COORDINATE` v4). Fixture 1b's writings and fixture 3's nesting (a
//! claim over every depth, not an input and an object) are read in `forms`
//! alone, as are fixtures 7–10 and 13–16 (the advance, an event's bell,
//! the merger and fission, the re-origin, dilation, the lane set):
//! `Fixture` has no operation for them and `Object` no reading of a merger,
//! an event's bell or a passage, so serving them is a new fixture and
//! object tag on `ACT_LAWS` mode 7, a wire change left to the clause's
//! drafting (78.173 §6).

use super::register::Fixture;
use algebra::{base_axis, Clifford, EM};
use forms::coordinate::{Axis, Bounds, Coordinate, Interval, Key, Lane, Lanes, Local, Origin, Square, Vector};
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

/// The key of the one axis `letter` `index`.
fn on(letter: char, index: u32) -> Option<Key> {
    Some(Key::axis(Axis::of(letter, index).ok()?))
}

/// The continuum's turning `e₊ₖe₊ₗ`, in that orientation (FORMS-COORDINATE
/// v4: H stands only on turning planes).
fn turning(k: u32, l: u32) -> Option<Key> {
    Key::generator(Axis::unit(Square::Positive, k).ok()?, Axis::unit(Square::Positive, l).ok()?).ok()
}

/// The one-component lanes `S:G:T:H`, keyed `e_S1`, `e_G1`, `e_T1` and
/// `e₊₂e₊₃`, with `s` on S, `g` on G, and T and H at 0.
fn lanes(s: Q, g: Q) -> Option<Lanes> {
    let one = |k: Option<Key>, v: Q| Some(Vector::one(k?, v));
    Lanes::base(one(on('S', 1), s)?, one(on('G', 1), g)?, one(on('T', 1), q(0, 1))?, one(turning(2, 3), q(0, 1))?).ok()
}

/// The lanes with `s` on S alone.
fn on_s(s: Q) -> Option<Lanes> {
    lanes(s, q(0, 1))
}

/// The body at `origin`, bell `h`, lanes, `K`.
fn body(origin: &Origin, h: u64, lanes: Option<Lanes>, k: u64) -> Option<Coordinate> {
    Some(Coordinate { origin: origin.clone(), local: Local { h, lanes: lanes?, k: vec![k] } })
}

/// The root enclosed once, in the branch cut at `3:2:0:0:0:0`.
fn enclosed() -> Option<Origin> {
    Some(Origin { enclosures: vec![Local { h: 3, lanes: on_s(q(2, 1))?, k: vec![0] }], ..root() })
}

/// The body at `R`, bell 5, S = G = T = 0 and H on the turnings about axes
/// 1 and 2, `e₊₂e₊₃` and `e₊₃e₊₁`, in degrees (fixtures 11 and 12).
fn planar(h1: Q, h2: Q) -> Option<Coordinate> {
    let h = Vector::of(vec![(turning(2, 3)?, h1), (turning(3, 1)?, h2)]).ok()?;
    let zero = |k: Option<Key>| Some(Vector::one(k?, q(0, 1)));
    let lanes = Lanes::base(zero(on('S', 1))?, zero(on('G', 1))?, zero(on('T', 1))?, h).ok()?;
    Some(Coordinate { origin: root(), local: Local { h: 5, lanes, k: vec![0, 0] } })
}

/// `1/3 − 10⁻¹⁵`, the value just below a bound.
fn below_third() -> Q {
    q(1, 3) - Q::new(BigInt::from(1), BigInt::from(10).pow(15))
}

/// Fixture 6's held bodies: `R/5:i/10` for `i = 0..19`, `R/6:1/2`, the same
/// body at epoch `0x93`, and `R/5:1/2:g` for `g = −1, 1/4, 2`.
fn held() -> Option<Vec<Coordinate>> {
    let r = root();
    let mut held: Vec<Option<Coordinate>> = (0..20).map(|i| body(&r, 5, on_s(q(i, 10)), 0)).collect();
    held.push(body(&r, 6, on_s(q(1, 2)), 0));
    held.push(body(&Origin { epoch: 0x93, ..root() }, 5, on_s(q(1, 2)), 0));
    held.extend([q(-1, 1), q(1, 4), q(2, 1)].into_iter().map(|g| body(&r, 5, lanes(q(1, 2), g), 0)));
    held.into_iter().collect()
}

/// The bounds `[lo, hi)` on S's one component, at `origin`.
fn on_s_within(origin: &Origin, lo: Q, hi: Q) -> Option<Bounds> {
    Bounds::of(origin.clone(), Lane::S, vec![(on('S', 1)?, Interval { lo, hi })]).ok()
}

/// LAW-COORDINATE v3's fixtures (78.173 §5) with an input and an object:
/// the order (1a, its seven bodies given in reverse), the sexagesimal
/// reading at the eight stated depths (2, display), `within` on the
/// caller's intervals (4), origins never compared (5), the two ranges on one
/// component (6), the order of a two-plane H (11, given in reverse), and a
/// range on one of its components (12). `None` where the form refuses an
/// input it builds.
pub fn coordinate_fixtures() -> Option<Vec<Fixture>> {
    let r = root();
    let mut ordered = vec![
        body(&r, 5, on_s(q(-1, 2)), 1)?,
        body(&r, 5, lanes(q(1, 3), q(-1, 1)), 9)?,
        body(&r, 5, on_s(q(1, 3)), 1)?,
        body(&r, 5, on_s(q(1, 3)), 2)?,
        body(&r, 5, on_s(q(1, 2)), 1)?,
        body(&r, 6, on_s(q(-7, 1)), 0)?,
        body(&enclosed()?, 0, on_s(q(0, 1)), 0)?,
    ];
    ordered.reverse();
    let mut fixtures = vec![Fixture::Ordered { bodies: ordered }];
    let sectors = [(q(7, 3), 0), (q(7, 3), 1), (q(7, 3), 2), (q(1, 7), 3), (q(360, 7), 2), (q(-1, 3), 1), (q(-1, 7), 1), (below_third(), 6)];
    fixtures.extend(sectors.into_iter().map(|(value, depth)| Fixture::Sector { value, depth }));
    // `1/(3·60⁶)` past `1/3`, inside `[1/3, 1/3 + 60⁻⁶)`.
    let (past, width) = (q(1, 139_968_000_000), q(1, 46_656_000_000));
    let within = [
        (q(1, 3), q(1, 3), q(7, 20)),
        (q(7, 20), q(1, 3), q(7, 20)),
        (below_third(), q(1, 3), q(7, 20)),
        (q(1, 3) + past, q(1, 3), q(1, 3) + width),
        (q(2, 5), q(0, 1), q(1, 1)),
        (q(2, 5), q(1, 3), q(7, 20)),
    ];
    for (s, lo, hi) in within {
        fixtures.push(Fixture::Within { at: body(&r, 7, on_s(s), 4)?, bounds: on_s_within(&r, lo, hi)? });
    }
    fixtures.push(Fixture::Within { at: body(&r, 5, on_s(q(1, 3)), 0)?, bounds: on_s_within(&enclosed()?, q(1, 3), q(7, 20))? });
    let span = |lo: Q, hi: Q| Interval { lo, hi };
    fixtures.push(Fixture::Range { held: held()?, at: body(&r, 5, on_s(q(0, 1)), 0)?, lane: Lane::S, key: on('S', 1)?, interval: span(q(1, 2), q(9, 10)) });
    fixtures.push(Fixture::Range { held: held()?, at: body(&r, 5, on_s(q(1, 2)), 0)?, lane: Lane::G, key: on('G', 1)?, interval: span(q(0, 1), q(1, 1)) });
    let (a, b, c, d) = (planar(q(1, 3), q(7, 4))?, planar(q(1, 3), q(-1, 2))?, planar(q(1, 2), q(0, 1))?, planar(q(2, 5), q(7, 4))?);
    fixtures.push(Fixture::Ordered { bodies: vec![c.clone(), a.clone(), b.clone()] });
    fixtures.push(Fixture::Range {
        held: vec![a, b, c, d],
        at: planar(q(0, 1), q(0, 1))?,
        lane: Lane::H,
        key: turning(2, 3)?,
        interval: span(q(1, 3), q(1, 2)),
    });
    Some(fixtures)
}
