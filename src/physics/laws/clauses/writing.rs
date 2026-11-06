// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// A21, CONCORDANCE §0 — THE CLAUSE'S CONTENT WRITTEN OUT: every fixture's
// input and every object it reaches in exact bytes, so a clause's address is
// its content and never a digest of it.
//
//! Every count, mask and text length by its own coordinate
//! (`put_key_count`), every rational by its one codec (`put_q`). A value of
//! `Cl(p, q)` is written `[negatives][term count][(mask, coefficient) …]`,
//! its masks strictly ascending and every coefficient non-zero, so a value
//! has one writing and a reader refuses any other. A coordinate is written
//! `[genesis 32][epoch u32 LE][enclosure count][local …][local]`, a local
//! (`FORMS-COORDINATE` v4) `[h u64 LE][lane count][lane …][K count][K u64
//! LE …]`, a lane `[lane name][component count][(key, value) …]`, its keys
//! in canonical order, a key `[axis count][(letter byte, index) …]`, a
//! frame unit's letter its sign (`+` for `e₊ₖ`, `-` for `e₋ₖ`). A lane
//! name is its byte (S 0, G 1, T 2, H 3) or, for an opened lane, `4`, the
//! name's bytes and its law version (u16 LE); a reader admits only the
//! ruled openings. A reader that does not find what it needs answers
//! `None`, never a default; a lane set or a vector whose order is not the
//! canonical one is a second writing and reads as none.

use super::coupling::{put_coupling_fixture, put_coupling_object, take_coupling_fixture, take_coupling_object};
use super::register::{ConcordanceClause, ConcordanceClauseId, Fixture, Object, Standing};
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use crate::transducer::alphabet::{put_q, take_q};
use algebra::Clifford;
use forms::coordinate::{Axis, Bounds, Coordinate, CoordinateRefusal, Interval, Key, Lane, Lanes, Local, Openings, Origin, Sector, Side, Sides, Square, Vector};
use forms::Q;
use num_traits::Zero;

pub(super) fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
    let v = *b.get(*p)?;
    *p += 1;
    Some(v)
}

fn take_array<const N: usize>(b: &[u8], p: &mut usize) -> Option<[u8; N]> {
    let v: [u8; N] = b.get(*p..*p + N)?.try_into().ok()?;
    *p += N;
    Some(v)
}

/// A value of `Cl(p, q)`: its negative mask, then each term ascending by
/// mask.
pub fn put_clifford(out: &mut Vec<u8>, x: &Clifford) {
    put_key_count(out, x.negatives());
    put_key_count(out, x.terms().count());
    for (mask, c) in x.terms() {
        put_key_count(out, mask);
        put_q(out, c);
    }
}

/// A value read back, refused where its masks do not ascend strictly, a
/// coefficient is zero, or a mask names a generator the algebra does not
/// hold: one writing per value.
pub fn take_clifford(b: &[u8], p: &mut usize) -> Option<Clifford> {
    let negatives = take_key_count(b, p)?;
    let n = take_key_count(b, p)?;
    let mut value = Clifford::blade(negatives, 0, Q::zero()).ok()?;
    let mut previous: Option<usize> = None;
    for _ in 0..n {
        let mask = take_key_count(b, p)?;
        let c = take_q(b, p)?;
        if c.is_zero() || previous.is_some_and(|m| m >= mask) {
            return None;
        }
        previous = Some(mask);
        value = value.sum(&Clifford::blade(negatives, mask, c).ok()?).ok()?;
    }
    Some(value)
}

/// A key: its axes, each its letter's byte (a frame unit's sign, `+` or
/// `-`) and its index.
pub(super) fn put_key(out: &mut Vec<u8>, k: &Key) {
    put_key_count(out, k.axes().len());
    for a in k.axes() {
        let mut letter = [0u8; 4];
        put_key_bytes(out, a.letter().encode_utf8(&mut letter).as_bytes());
        put_key_count(out, a.index() as usize);
    }
}

/// A key read back: one axis, or a generator's two; refused where an axis
/// is unwritable or a generator names one axis twice.
pub(super) fn take_key(b: &[u8], p: &mut usize) -> Option<Key> {
    let n = take_key_count(b, p)?;
    let axes = (0..n)
        .map(|_| {
            let letter = String::from_utf8(take_key_bytes(b, p)?).ok()?;
            let index = u32::try_from(take_key_count(b, p)?).ok()?;
            match letter.chars().collect::<Vec<_>>().as_slice() {
                ['+'] => Axis::unit(Square::Positive, index).ok(),
                ['-'] => Axis::unit(Square::Negative, index).ok(),
                [c] => Axis::of(*c, index).ok(),
                _ => None,
            }
        })
        .collect::<Option<Vec<Axis>>>()?;
    match <[Axis; 2]>::try_from(axes) {
        Ok([a, c]) => Key::generator(a, c).ok(),
        Err(axes) => match <[Axis; 1]>::try_from(axes) {
            Ok([a]) => Some(Key::axis(a)),
            Err(_) => None,
        },
    }
}

/// A lane's name: its byte, or `4`, the opened lane's name and version.
fn put_lane(out: &mut Vec<u8>, lane: &Lane) {
    match lane {
        Lane::S => out.push(0),
        Lane::G => out.push(1),
        Lane::T => out.push(2),
        Lane::H => out.push(3),
        Lane::Opened { name, version } => {
            out.push(4);
            put_key_bytes(out, name.as_bytes());
            out.extend_from_slice(&version.to_le_bytes());
        }
    }
}

fn take_lane(b: &[u8], p: &mut usize) -> Option<Lane> {
    Some(match take_u8(b, p)? {
        0 => Lane::S,
        1 => Lane::G,
        2 => Lane::T,
        3 => Lane::H,
        4 => Lane::Opened { name: String::from_utf8(take_key_bytes(b, p)?).ok()?, version: u16::from_le_bytes(take_array(b, p)?) },
        _ => return None,
    })
}

fn put_vector(out: &mut Vec<u8>, v: &Vector) {
    put_key_count(out, v.components().len());
    for (k, x) in v.components() {
        put_key(out, k);
        put_q(out, x);
    }
}

/// A vector read back, refused where a key repeats or the keys are not in
/// canonical order: one writing per vector.
fn take_vector(b: &[u8], p: &mut usize) -> Option<Vector> {
    let n = take_key_count(b, p)?;
    let read = (0..n).map(|_| Some((take_key(b, p)?, take_q(b, p)?))).collect::<Option<Vec<(Key, Q)>>>()?;
    let v = Vector::of(read.clone()).ok()?;
    (v.components() == read.as_slice()).then_some(v)
}

fn put_local(out: &mut Vec<u8>, l: &Local) {
    out.extend_from_slice(&l.h.to_le_bytes());
    put_key_count(out, l.lanes.held().len());
    for (lane, v) in l.lanes.held() {
        put_lane(out, lane);
        put_vector(out, v);
    }
    put_key_count(out, l.k.len());
    l.k.iter().for_each(|k| out.extend_from_slice(&k.to_le_bytes()));
}

/// A place read back, its lane set admitted under the ruled openings and
/// refused where the lanes are not in canonical order.
fn take_local(b: &[u8], p: &mut usize) -> Option<Local> {
    let h = u64::from_le_bytes(take_array(b, p)?);
    let n = take_key_count(b, p)?;
    let read = (0..n).map(|_| Some((take_lane(b, p)?, take_vector(b, p)?))).collect::<Option<Vec<(Lane, Vector)>>>()?;
    let lanes = Lanes::of(read.clone(), &Openings::ruled()).ok()?;
    if lanes.held() != read.as_slice() {
        return None;
    }
    let m = take_key_count(b, p)?;
    let k = (0..m).map(|_| Some(u64::from_le_bytes(take_array(b, p)?))).collect::<Option<Vec<u64>>>()?;
    Some(Local { h, lanes, k })
}

fn put_origin(out: &mut Vec<u8>, o: &Origin) {
    out.extend_from_slice(&o.genesis);
    out.extend_from_slice(&o.epoch.to_le_bytes());
    put_key_count(out, o.enclosures.len());
    o.enclosures.iter().for_each(|l| put_local(out, l));
}

fn take_origin(b: &[u8], p: &mut usize) -> Option<Origin> {
    let genesis = take_array(b, p)?;
    let epoch = u32::from_le_bytes(take_array(b, p)?);
    let n = take_key_count(b, p)?;
    let enclosures = (0..n).map(|_| take_local(b, p)).collect::<Option<Vec<_>>>()?;
    Some(Origin { genesis, epoch, enclosures })
}

/// A coordinate: its origin written out, then its place.
pub fn put_coordinate(out: &mut Vec<u8>, c: &Coordinate) {
    put_origin(out, &c.origin);
    put_local(out, &c.local);
}

pub fn take_coordinate(b: &[u8], p: &mut usize) -> Option<Coordinate> {
    Some(Coordinate { origin: take_origin(b, p)?, local: take_local(b, p)? })
}

fn put_bodies(out: &mut Vec<u8>, bodies: &[Coordinate]) {
    put_key_count(out, bodies.len());
    bodies.iter().for_each(|c| put_coordinate(out, c));
}

fn take_bodies(b: &[u8], p: &mut usize) -> Option<Vec<Coordinate>> {
    let n = take_key_count(b, p)?;
    (0..n).map(|_| take_coordinate(b, p)).collect()
}

/// A lane's byte: its place in the canonical order S, G, T, H.
fn put_interval(out: &mut Vec<u8>, i: &Interval) {
    put_q(out, &i.lo);
    put_q(out, &i.hi);
}

fn take_interval(b: &[u8], p: &mut usize) -> Option<Interval> {
    Some(Interval { lo: take_q(b, p)?, hi: take_q(b, p)? })
}

/// Bounds: the origin, the lane, then each key and its interval in
/// canonical key order.
fn put_bounds(out: &mut Vec<u8>, bounds: &Bounds) {
    put_origin(out, bounds.origin());
    put_lane(out, bounds.lane());
    put_key_count(out, bounds.intervals().len());
    for (k, i) in bounds.intervals() {
        put_key(out, k);
        put_interval(out, i);
    }
}

/// Bounds read back, refused where a key repeats or the keys are not in
/// canonical order.
fn take_bounds(b: &[u8], p: &mut usize) -> Option<Bounds> {
    let origin = take_origin(b, p)?;
    let lane = take_lane(b, p)?;
    let n = take_key_count(b, p)?;
    let read = (0..n).map(|_| Some((take_key(b, p)?, take_interval(b, p)?))).collect::<Option<Vec<(Key, Interval)>>>()?;
    let bounds = Bounds::of(origin, lane, read.clone()).ok()?;
    (bounds.intervals() == read.as_slice()).then_some(bounds)
}

fn put_keys(out: &mut Vec<u8>, keys: &[Key]) {
    put_key_count(out, keys.len());
    keys.iter().for_each(|k| put_key(out, k));
}

fn take_keys(b: &[u8], p: &mut usize) -> Option<Vec<Key>> {
    let n = take_key_count(b, p)?;
    (0..n).map(|_| take_key(b, p)).collect()
}

fn put_depth(out: &mut Vec<u8>, depth: u32) {
    out.extend_from_slice(&depth.to_le_bytes());
}

fn take_depth(b: &[u8], p: &mut usize) -> Option<u32> {
    Some(u32::from_le_bytes(take_array(b, p)?))
}

/// ONE FIXTURE'S INPUT: its tag (the fixture's operation), then its
/// objects. Tags 0–3 named LAW-PASSAGE v2's operations (the product `v·q`
/// passed twice and once, the commutator, the square); they are retired
/// with `FORMS-PASSAGE` v3 and read as none. The pole swap's crossing is
/// tag 8 and the riding state's pass tag 9; LAW-COUPLING's fixtures are
/// tag 12, each its operation's byte then its values (`super::coupling`).
pub fn put_fixture(out: &mut Vec<u8>, f: &Fixture) {
    match f {
        Fixture::Crossed { positive, negative, frame, flux } => {
            out.push(8);
            put_clifford(out, positive);
            put_clifford(out, negative);
            put_clifford(out, frame);
            put_q(out, flux);
        }
        Fixture::Ridden { positive, negative, state } => {
            out.push(9);
            put_clifford(out, positive);
            put_clifford(out, negative);
            put_clifford(out, state);
        }
        Fixture::Ordered { bodies } => {
            out.push(4);
            put_bodies(out, bodies);
        }
        Fixture::Sector { value, depth } => {
            out.push(5);
            put_q(out, value);
            put_depth(out, *depth);
        }
        Fixture::Within { at, bounds } => {
            out.push(10);
            put_coordinate(out, at);
            put_bounds(out, bounds);
        }
        Fixture::Range { held, at, lane, key, interval } => {
            out.push(11);
            put_bodies(out, held);
            put_coordinate(out, at);
            put_lane(out, lane);
            put_key(out, key);
            put_interval(out, interval);
        }
        Fixture::Coupling(c) => {
            out.push(12);
            put_coupling_fixture(out, c);
        }
    }
}

/// One fixture read back by its tag. Tags 6 and 7 named
/// `FORMS-COORDINATE` v2's `within` on a sector at a depth and its range on
/// one lane; they are retired with v3 (`within` on stated bounds, tag 10;
/// a range on one component, tag 11) and read as none.
pub fn take_fixture(b: &[u8], p: &mut usize) -> Option<Fixture> {
    Some(match take_u8(b, p)? {
        4 => Fixture::Ordered { bodies: take_bodies(b, p)? },
        5 => Fixture::Sector { value: take_q(b, p)?, depth: take_depth(b, p)? },
        10 => Fixture::Within { at: take_coordinate(b, p)?, bounds: take_bounds(b, p)? },
        11 => Fixture::Range { held: take_bodies(b, p)?, at: take_coordinate(b, p)?, lane: take_lane(b, p)?, key: take_key(b, p)?, interval: take_interval(b, p)? },
        8 => Fixture::Crossed { positive: take_clifford(b, p)?, negative: take_clifford(b, p)?, frame: take_clifford(b, p)?, flux: take_q(b, p)? },
        9 => Fixture::Ridden { positive: take_clifford(b, p)?, negative: take_clifford(b, p)?, state: take_clifford(b, p)? },
        12 => Fixture::Coupling(take_coupling_fixture(b, p)?),
        _ => return None,
    })
}

fn side_byte(s: Side) -> u8 {
    match s {
        Side::Below => 0,
        Side::Within => 1,
        Side::Above => 2,
    }
}

fn side_of(b: u8) -> Option<Side> {
    match b {
        0 => Some(Side::Below),
        1 => Some(Side::Within),
        2 => Some(Side::Above),
        _ => None,
    }
}

/// ONE OBJECT A FIXTURE REACHES: its tag, then the object.
pub fn put_object(out: &mut Vec<u8>, o: &Object) {
    match o {
        Object::Multivector(x) => {
            out.push(0);
            put_clifford(out, x);
        }
        Object::Crossing { frame, span, flux } => {
            out.push(5);
            put_clifford(out, frame);
            put_q(out, span);
            put_q(out, flux);
        }
        Object::Bodies(bodies) => {
            out.push(1);
            put_bodies(out, bodies);
        }
        Object::Sector(s) => {
            out.push(2);
            put_depth(out, s.depth);
            put_q(out, &Q::from_integer(s.index.clone()));
        }
        Object::Sides(s) => {
            out.push(6);
            put_key_count(out, s.sides.len());
            for (k, side) in &s.sides {
                put_key(out, k);
                out.push(side_byte(*side));
            }
        }
        Object::Refused(CoordinateRefusal::OriginsDiffer { held, asked }) => {
            out.push(4);
            put_origin(out, held);
            put_origin(out, asked);
        }
        Object::Refused(CoordinateRefusal::ShapesDiffer { lane, held, asked }) => {
            out.push(7);
            put_lane(out, lane);
            put_keys(out, held);
            put_keys(out, asked);
        }
        Object::Refused(CoordinateRefusal::NoComponent { lane, key }) => {
            out.push(8);
            put_lane(out, lane);
            put_key(out, key);
        }
        Object::Coupling(c) => {
            out.push(9);
            put_coupling_object(out, c);
        }
    }
}

/// One object read back by its tag. Tag 3 named `FORMS-COORDINATE` v2's
/// one side; it is retired with v3 (a side per component, tag 6) and reads
/// as none.
pub fn take_object(b: &[u8], p: &mut usize) -> Option<Object> {
    Some(match take_u8(b, p)? {
        0 => Object::Multivector(take_clifford(b, p)?),
        1 => Object::Bodies(take_bodies(b, p)?),
        2 => {
            let depth = take_depth(b, p)?;
            let index = take_q(b, p)?;
            Object::Sector(Sector { depth, index: index.is_integer().then(|| index.to_integer())? })
        }
        4 => Object::Refused(CoordinateRefusal::OriginsDiffer { held: Box::new(take_origin(b, p)?), asked: Box::new(take_origin(b, p)?) }),
        5 => Object::Crossing { frame: take_clifford(b, p)?, span: take_q(b, p)?, flux: take_q(b, p)? },
        6 => {
            let n = take_key_count(b, p)?;
            Object::Sides(Sides { sides: (0..n).map(|_| Some((take_key(b, p)?, side_of(take_u8(b, p)?)?))).collect::<Option<Vec<_>>>()? })
        }
        7 => Object::Refused(CoordinateRefusal::ShapesDiffer { lane: take_lane(b, p)?, held: take_keys(b, p)?, asked: take_keys(b, p)? }),
        8 => Object::Refused(CoordinateRefusal::NoComponent { lane: take_lane(b, p)?, key: take_key(b, p)? }),
        9 => Object::Coupling(take_coupling_object(b, p)?),
        _ => return None,
    })
}

/// A CLAUSE'S NAME: its law id, then its version.
pub fn put_clause_id(out: &mut Vec<u8>, id: &ConcordanceClauseId) {
    put_key_bytes(out, id.law.as_bytes());
    out.extend_from_slice(&id.version.to_le_bytes());
}

pub fn take_clause_id(b: &[u8], p: &mut usize) -> Option<ConcordanceClauseId> {
    let law = String::from_utf8(take_key_bytes(b, p)?).ok()?;
    Some(ConcordanceClauseId { law, version: u16::from_le_bytes(take_array(b, p)?) })
}

/// THE CLAUSE'S CONTENT ADDRESS, written out: its name, then each fixture's
/// input in order.
pub fn put_content(out: &mut Vec<u8>, id: &ConcordanceClauseId, fixtures: &[Fixture]) {
    put_clause_id(out, id);
    put_key_count(out, fixtures.len());
    fixtures.iter().for_each(|f| put_fixture(out, f));
}

pub fn take_content(b: &[u8], p: &mut usize) -> Option<(ConcordanceClauseId, Vec<Fixture>)> {
    let id = take_clause_id(b, p)?;
    let n = take_key_count(b, p)?;
    let fixtures = (0..n).map(|_| take_fixture(b, p)).collect::<Option<Vec<_>>>()?;
    Some((id, fixtures))
}

/// A CLAUSE AS SERVED: its name, its standing, then its fixtures.
pub fn put_clause(out: &mut Vec<u8>, c: &ConcordanceClause) {
    put_clause_id(out, &c.id);
    out.push(c.standing.byte());
    put_key_count(out, c.fixtures.len());
    c.fixtures.iter().for_each(|f| put_fixture(out, f));
}

pub fn take_clause(b: &[u8], p: &mut usize) -> Option<ConcordanceClause> {
    let id = take_clause_id(b, p)?;
    let standing = Standing::of(take_u8(b, p)?)?;
    let n = take_key_count(b, p)?;
    let fixtures = (0..n).map(|_| take_fixture(b, p)).collect::<Option<Vec<_>>>()?;
    Some(ConcordanceClause { id, standing, fixtures })
}
