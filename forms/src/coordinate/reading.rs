// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COORDINATE v3 (proposal 78.173 §1) — THE READINGS OF A COORDINATE:
// where a body stands against exact intervals the caller states (`within`,
// a side per component), the neighbours on one component by bisection, and
// the sexagesimal reading, which is display only.

use super::lanes::{FormRefusal, Key, Lane, Vector};
use super::{Coordinate, CoordinateRefusal, Origin, Side, BASE};
use crate::Q;
use num_bigint::BigInt;
use num_traits::One;
use std::cmp::Ordering;

/// AN EXACT INTERVAL `[lo, hi)` THE CALLER STATES: the one thing `within`
/// and the ranges decide on. An interval with `hi ≤ lo` holds no value.
/// Ordered by `lo`, then `hi`, only so a list of bounds has one writing.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    pub lo: Q,
    pub hi: Q,
}

impl Interval {
    /// WHERE `value` STANDS against `[lo, hi)`, by exact comparison: below
    /// `lo`, within, or at or past `hi`.
    pub fn side(&self, value: &Q) -> Side {
        match (value.cmp(&self.lo), value.cmp(&self.hi)) {
            (Ordering::Less, _) => Side::Below,
            (_, Ordering::Less) => Side::Within,
            _ => Side::Above,
        }
    }
}

/// THE BOUNDS A CALLER STATES ON ONE LANE: at an origin, an interval per
/// component, keyed as the lane's components are, in canonical key order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bounds {
    origin: Origin,
    lane: Lane,
    intervals: Vec<(Key, Interval)>,
}

impl Bounds {
    /// THE BOUNDS `intervals` on `lane` at `origin`, put in canonical key
    /// order. Refused `KeyRepeated` where a key is stated twice.
    pub fn of(origin: Origin, lane: Lane, mut intervals: Vec<(Key, Interval)>) -> Result<Bounds, FormRefusal> {
        intervals.sort();
        let repeated = intervals.windows(2).find_map(|w| match w {
            [a, b] if a.0 == b.0 => Some(b.0.clone()),
            _ => None,
        });
        match repeated {
            Some(key) => Err(FormRefusal::KeyRepeated { key }),
            None => Ok(Bounds { origin, lane, intervals }),
        }
    }

    /// The origin the bounds are stated at.
    pub fn origin(&self) -> &Origin {
        &self.origin
    }

    /// The lane the bounds are stated on.
    pub fn lane(&self) -> &Lane {
        &self.lane
    }

    /// The intervals, in canonical key order.
    pub fn intervals(&self) -> &[(Key, Interval)] {
        &self.intervals
    }

    /// The keys the bounds are stated on: the shape they read.
    fn keys(&self) -> Vec<Key> {
        self.intervals.iter().map(|(k, _)| k.clone()).collect()
    }
}

/// WHERE A BODY STANDS AGAINST BOUNDS: a side per component, in canonical
/// key order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sides {
    pub sides: Vec<(Key, Side)>,
}

/// THE WHOLE READING OF SIDES: within only where every component is
/// within; otherwise outside, naming each component that is not, with its
/// side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Whole {
    Within,
    Outside(Vec<(Key, Side)>),
}

impl Sides {
    /// Within where every component is within; outside, with the
    /// components that are not, otherwise.
    pub fn whole(&self) -> Whole {
        let outside: Vec<(Key, Side)> = self.sides.iter().filter(|(_, s)| *s != Side::Within).cloned().collect();
        match outside.is_empty() {
            true => Whole::Within,
            false => Whole::Outside(outside),
        }
    }
}

impl Coordinate {
    /// WHERE THIS BODY STANDS AGAINST `bounds`: a side per component of the
    /// bounds' lane, each decided on the caller's exact interval, never on
    /// a sector. Refused `OriginsDiffer` where the bounds stand at another
    /// origin (the body's held, the bounds' asked), and `ShapesDiffer`
    /// where their keys are not the lane's.
    pub fn within(&self, bounds: &Bounds) -> Result<Sides, CoordinateRefusal> {
        if self.origin != bounds.origin {
            return Err(CoordinateRefusal::OriginsDiffer { held: Box::new(self.origin.clone()), asked: Box::new(bounds.origin.clone()) });
        }
        let held = self.lane(&bounds.lane).map(Vector::keys).into_iter().flatten().collect::<Vec<Key>>();
        let differ = || CoordinateRefusal::ShapesDiffer { lane: bounds.lane.clone(), held: held.clone(), asked: bounds.keys() };
        let vector = self.lane(&bounds.lane).ok_or_else(differ)?;
        match vector.keys() == bounds.keys() {
            true => Ok(Sides { sides: vector.components().iter().zip(&bounds.intervals).map(|((k, v), (_, i))| (k.clone(), i.side(v))).collect() }),
            false => Err(differ()),
        }
    }

    /// THE SEXAGESIMAL READING of `lane` at `depth`, for display: the
    /// sector holding each component.
    pub fn reading(&self, lane: &Lane, depth: u32) -> Option<Reading> {
        let v = self.lane(lane)?;
        Some(Reading { sectors: v.components().iter().map(|(k, x)| (k.clone(), Sector::holding(x, depth))).collect() })
    }
}

/// ONE SECTOR OF THE SEXAGESIMAL READING, DISPLAY ONLY: at `depth`, the
/// interval `[index·60⁻ᵈᵉᵖᵗʰ, (index + 1)·60⁻ᵈᵉᵖᵗʰ)` holding a value. No
/// decision reads it: `within` and the ranges take the caller's
/// [`Interval`]. The value read is never replaced by it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sector {
    pub depth: u32,
    pub index: BigInt,
}

/// THE DISPLAY OF A SECTOR: its lower bound as a signed whole part and
/// `depth` places, each in `0..60`, so that `lo = whole + Σ placeᵢ·60⁻ⁱ`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Places {
    pub whole: BigInt,
    pub places: Vec<BigInt>,
}

/// `60ᵈᵉᵖᵗʰ`, exactly.
fn scale(depth: u32) -> BigInt {
    BigInt::from(BASE).pow(depth)
}

impl Sector {
    /// THE SECTOR AT `depth` HOLDING `value`: the one integer `m` with
    /// `m·60⁻ᵈ ≤ value < (m + 1)·60⁻ᵈ`, fixed by exact comparison.
    pub fn holding(value: &Q, depth: u32) -> Sector {
        let index = (value * Q::from_integer(scale(depth))).floor().to_integer();
        Sector { depth, index }
    }

    /// The sector's width, `60⁻ᵈᵉᵖᵗʰ`.
    pub fn width(&self) -> Q {
        Q::new(BigInt::one(), scale(self.depth))
    }

    /// The lower bound, held by the sector.
    pub fn lo(&self) -> Q {
        Q::new(self.index.clone(), scale(self.depth))
    }

    /// The upper bound, not held by the sector.
    pub fn hi(&self) -> Q {
        Q::new(&self.index + BigInt::one(), scale(self.depth))
    }

    /// The sector one depth coarser that holds this one; `None` at depth 0,
    /// the whole unit, which no coarser sector reads.
    pub fn parent(&self) -> Option<Sector> {
        Some(Sector::holding(&self.lo(), self.depth.checked_sub(1)?))
    }

    /// THE PLACES, for display: the whole part (the integer at or below the
    /// lower bound) and `depth` places in `0..60`, coarsest first.
    pub fn places(&self) -> Places {
        let s = scale(self.depth);
        let whole = Q::new(self.index.clone(), s.clone()).floor().to_integer();
        let mut rest = &self.index - &whole * &s;
        let mut places = Vec::new();
        for i in (0..self.depth).rev() {
            let unit = scale(i);
            places.push(&rest / &unit);
            rest %= unit;
        }
        Places { whole, places }
    }

    /// THE WRITING, for display only: the whole part in decimal, then `;`
    /// and the places in two decimal digits each, separated by `,` — the
    /// sexagesimal notation, `0;08,34,17` for `1/7` at depth 3. An H
    /// component in degrees reads directly as degrees;minutes,seconds
    /// (`51;25,42` is 51°25′42″). A sector at depth 0 writes its whole part
    /// alone.
    pub fn written(&self) -> String {
        let p = self.places();
        match p.places.is_empty() {
            true => p.whole.to_string(),
            false => {
                let places: Vec<String> = p.places.iter().map(|d| format!("{d:02}")).collect();
                format!("{};{}", p.whole, places.join(","))
            }
        }
    }
}

/// THE SEXAGESIMAL READING OF A LANE, DISPLAY ONLY: the sector holding each
/// component, in canonical key order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub sectors: Vec<(Key, Sector)>,
}

impl Reading {
    /// The writing, for display only: each sector's writing, joined by
    /// ` × ` (`0;20 × 1;45`); a one-component reading is its sector's.
    pub fn written(&self) -> String {
        let parts: Vec<String> = self.sectors.iter().map(|(_, s)| s.written()).collect();
        parts.join(" × ")
    }
}

/// THE BODIES A KERNEL HOLDS, in the canonical order: the ordered key is
/// the index, so a neighbour is found by bisection on it. The key routes and
/// never identifies.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bodies {
    held: Vec<Coordinate>,
}

/// THE BODIES OF ONE RANGE ON ONE COMPONENT, and the probes the two
/// bisections that found it took.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Neighbours<'a> {
    pub bodies: &'a [Coordinate],
    pub probes: u32,
}

impl<'a> Neighbours<'a> {
    /// THE BODIES OF THE RANGE WHOSE COMPONENT `key` OF `lane` LIES IN
    /// `interval`: a filter within the range, read body by body over the
    /// range alone.
    pub fn filter(&self, lane: &Lane, key: &Key, interval: &Interval) -> Vec<&'a Coordinate> {
        self.bodies.iter().filter(|c| c.lane(lane).and_then(|v| v.get(key)).is_some_and(|v| interval.side(v) == Side::Within)).collect()
    }
}

/// The first place in `held` whose body does not stand before the bound,
/// by bisection, and the probes it took.
fn bisect(held: &[Coordinate], before: impl Fn(&Coordinate) -> bool) -> (usize, u32) {
    let (mut lo, mut hi, mut probes) = (0usize, held.len(), 0u32);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        probes += 1;
        match held.get(mid).is_some_and(&before) {
            true => lo = mid + 1,
            false => hi = mid,
        }
    }
    (lo, probes)
}

impl Bodies {
    /// No bodies.
    pub fn new() -> Bodies {
        Bodies { held: Vec::new() }
    }

    /// The bodies, in the canonical order.
    pub fn held(&self) -> &[Coordinate] {
        &self.held
    }

    /// HOLD A BODY at its place in the canonical order, after any body at
    /// the same coordinate; returns the place.
    pub fn hold(&mut self, body: Coordinate) -> usize {
        let at = self.held.partition_point(|c| c <= &body);
        self.held.insert(at, body);
        at
    }

    /// THE NEIGHBOURS OF `at` ON COMPONENT `key` OF `lane` OVER `interval`:
    /// the bodies at `at`'s origin and bell whose lanes before `lane`, and
    /// whose components of `lane` keyed before `key`, equal `at`'s, and
    /// whose component `key` lies in `[lo, hi)`. In the canonical order
    /// they stand together, so two bisections find them; no body outside
    /// the range is read past the probes, and any other component is a
    /// filter within them ([`Neighbours::filter`]). Refused `NoComponent`
    /// where `at` holds no component `key` on `lane`.
    pub fn range(&self, at: &Coordinate, lane: &Lane, key: &Key, interval: &Interval) -> Result<Neighbours<'_>, CoordinateRefusal> {
        let none = || CoordinateRefusal::NoComponent { lane: lane.clone(), key: key.clone() };
        let n = at.local.lanes.place(lane).ok_or_else(none)?;
        let vector = at.local.lanes.get(lane).ok_or_else(none)?;
        let c = vector.components().binary_search_by(|(k, _)| k.cmp(key)).map_err(|_| none())?;
        let (lanes_before, keys_before) = (at.local.lanes.held().get(..n).ok_or_else(none)?, vector.components().get(..c).ok_or_else(none)?);
        let key_of = |x: &Coordinate, v: &Q| -> Ordering {
            let theirs = x.local.lanes.held();
            let head = (&x.origin, x.local.h, theirs.get(..n));
            match head.cmp(&(&at.origin, at.local.h, Some(lanes_before))) {
                Ordering::Equal => {
                    let target: Vec<(Key, Q)> = keys_before.iter().cloned().chain([(key.clone(), v.clone())]).collect();
                    let comps = theirs.get(n).map(|(l, w)| (l, w.components()));
                    comps.cmp(&Some((lane, target.as_slice())))
                }
                other => other,
            }
        };
        let (start, p) = bisect(&self.held, |c| key_of(c, &interval.lo) == Ordering::Less);
        let (end, q) = bisect(&self.held, |c| key_of(c, &interval.hi) == Ordering::Less);
        // An empty span (`hi ≤ lo`) ends at or before it starts: no body.
        let bodies = match self.held.get(start..end) {
            Some(b) => b,
            None => &[],
        };
        Ok(Neighbours { bodies, probes: p + q })
    }
}
