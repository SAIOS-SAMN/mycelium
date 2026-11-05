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
//! `[h u64 LE][S][G][T][H][K u64 LE]`. A reader that does not find what it
//! needs answers `None`, never a default.

use super::register::{ConcordanceClause, ConcordanceClauseId, Fixture, Object, Standing};
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use crate::transducer::alphabet::{put_q, take_q};
use algebra::Clifford;
use forms::coordinate::{Coordinate, CoordinateRefusal, Lane, Local, Origin, Sector, Side};
use forms::Q;
use num_traits::Zero;

fn take_u8(b: &[u8], p: &mut usize) -> Option<u8> {
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

fn put_local(out: &mut Vec<u8>, l: &Local) {
    out.extend_from_slice(&l.h.to_le_bytes());
    l.lanes.iter().for_each(|v| put_q(out, v));
    out.extend_from_slice(&l.k.to_le_bytes());
}

fn take_local(b: &[u8], p: &mut usize) -> Option<Local> {
    let h = u64::from_le_bytes(take_array(b, p)?);
    let lanes = [take_q(b, p)?, take_q(b, p)?, take_q(b, p)?, take_q(b, p)?];
    let k = u64::from_le_bytes(take_array(b, p)?);
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
fn lane_byte(lane: Lane) -> u8 {
    match lane {
        Lane::S => 0,
        Lane::G => 1,
        Lane::T => 2,
        Lane::H => 3,
    }
}

fn take_lane(b: &[u8], p: &mut usize) -> Option<Lane> {
    Lane::ORDER.get(usize::from(take_u8(b, p)?)).copied()
}

fn put_depth(out: &mut Vec<u8>, depth: u32) {
    out.extend_from_slice(&depth.to_le_bytes());
}

fn take_depth(b: &[u8], p: &mut usize) -> Option<u32> {
    Some(u32::from_le_bytes(take_array(b, p)?))
}

/// ONE FIXTURE'S INPUT: its tag (the fixture's operation, in [`Fixture`]'s
/// order), then its objects.
pub fn put_fixture(out: &mut Vec<u8>, f: &Fixture) {
    match f {
        Fixture::PassedTwice { v, q } | Fixture::Passed { v, q } => {
            out.push(match f {
                Fixture::PassedTwice { .. } => 0,
                _ => 1,
            });
            put_clifford(out, v);
            put_clifford(out, q);
        }
        Fixture::Commutator { a, b } => {
            out.push(2);
            put_clifford(out, a);
            put_clifford(out, b);
        }
        Fixture::Square { x } => {
            out.push(3);
            put_clifford(out, x);
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
        Fixture::Within { at, other, lane, depth } => {
            out.push(6);
            put_coordinate(out, at);
            put_coordinate(out, other);
            out.push(lane_byte(*lane));
            put_depth(out, *depth);
        }
        Fixture::Range { held, at, lane, lo, hi } => {
            out.push(7);
            put_bodies(out, held);
            put_coordinate(out, at);
            out.push(lane_byte(*lane));
            put_q(out, lo);
            put_q(out, hi);
        }
    }
}

pub fn take_fixture(b: &[u8], p: &mut usize) -> Option<Fixture> {
    Some(match take_u8(b, p)? {
        0 => Fixture::PassedTwice { v: take_clifford(b, p)?, q: take_clifford(b, p)? },
        1 => Fixture::Passed { v: take_clifford(b, p)?, q: take_clifford(b, p)? },
        2 => Fixture::Commutator { a: take_clifford(b, p)?, b: take_clifford(b, p)? },
        3 => Fixture::Square { x: take_clifford(b, p)? },
        4 => Fixture::Ordered { bodies: take_bodies(b, p)? },
        5 => Fixture::Sector { value: take_q(b, p)?, depth: take_depth(b, p)? },
        6 => Fixture::Within { at: take_coordinate(b, p)?, other: take_coordinate(b, p)?, lane: take_lane(b, p)?, depth: take_depth(b, p)? },
        7 => Fixture::Range { held: take_bodies(b, p)?, at: take_coordinate(b, p)?, lane: take_lane(b, p)?, lo: take_q(b, p)?, hi: take_q(b, p)? },
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
        Object::Bodies(bodies) => {
            out.push(1);
            put_bodies(out, bodies);
        }
        Object::Sector(s) => {
            out.push(2);
            put_depth(out, s.depth);
            put_q(out, &Q::from_integer(s.index.clone()));
        }
        Object::Side(s) => out.extend_from_slice(&[3, side_byte(*s)]),
        Object::Refused(CoordinateRefusal::OriginsDiffer { held, asked }) => {
            out.push(4);
            put_origin(out, held);
            put_origin(out, asked);
        }
    }
}

pub fn take_object(b: &[u8], p: &mut usize) -> Option<Object> {
    Some(match take_u8(b, p)? {
        0 => Object::Multivector(take_clifford(b, p)?),
        1 => Object::Bodies(take_bodies(b, p)?),
        2 => {
            let depth = take_depth(b, p)?;
            let index = take_q(b, p)?;
            Object::Sector(Sector { depth, index: index.is_integer().then(|| index.to_integer())? })
        }
        3 => Object::Side(side_of(take_u8(b, p)?)?),
        4 => Object::Refused(CoordinateRefusal::OriginsDiffer { held: Box::new(take_origin(b, p)?), asked: Box::new(take_origin(b, p)?) }),
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
