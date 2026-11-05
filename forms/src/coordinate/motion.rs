// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// LAW-COORDINATE v3 (proposal 78.173 §1, fixtures 7–10 and 13–15) — THE
// MOTION OF A COORDINATE, per component: the closed-form advance on the
// universe's own bell, an event's bell, the merger with its offsets and
// fission, the passage across a throat (re-origin) and the flow of two
// pushes.

use super::lanes::{Key, Lane, Lanes, Shapes};
use super::{Coordinate, Local, Origin, DEGREES_PER_TURN};
use crate::passage::{pass_share, Frame, PassageRefusal, Poles};
use crate::Q;
use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};
use std::cmp::Ordering;

/// THE LANES' RATES: each component's change per bell, keyed as the body's
/// components are; H's in degrees per bell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rates {
    pub lanes: Lanes,
}

/// WHY AN ADVANCE, A MERGER, A FISSION, A PASSAGE OR A FLOW IS REFUSED, by
/// name and with what was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MotionRefusal {
    /// An advance to a bell before the body's: there is no backward
    /// advance. The bell held and the bell asked.
    Backward { held: u64, asked: u64 },
    /// A merger of bodies at two origins: origins are never compared. The
    /// origin held and the origin asked, as written.
    OriginsDiffer { held: Box<Origin>, asked: Box<Origin> },
    /// A merger of bodies at two bells. The bell held and the bell asked.
    BellsDiffer { held: u64, asked: u64 },
    /// Two lane sets of different shapes (rates against a body, two bodies
    /// merged, offsets against a point): the lane and the keys each holds.
    ShapesDiffer(Box<Shapes>),
    /// No component `key` on `lane`, where a reading names one.
    NoComponent { lane: Lane, key: Key },
    /// Holdings whose sum is zero: no holding-weighted coordinate. The two
    /// holdings.
    NoHolding { a: Box<Q>, b: Box<Q> },
    /// Offsets whose holding-weighted sum is not zero on every component:
    /// no merger left them. The sum, per lane and component.
    Unbalanced { residual: Box<Lanes> },
    /// Whole turns of an H component past `K`'s carrier, a `u64`. The
    /// component's key and the whole turns read.
    OrbitsPastCarrier { key: Key, turns: BigInt },
    /// A passage whose frame state's span is not the body's S component:
    /// the S held and the span read from the frame state.
    SpanDiffers { held: Box<Q>, read: Box<Q> },
    /// A push outside `(−1, 1)`: the value as presented.
    NotPush(Box<Q>),
    /// The passage refused the frame state or its share, by its own name.
    Passage(Box<PassageRefusal>),
}

impl From<PassageRefusal> for MotionRefusal {
    fn from(r: PassageRefusal) -> MotionRefusal {
        MotionRefusal::Passage(Box::new(r))
    }
}

impl From<Shapes> for MotionRefusal {
    fn from(s: Shapes) -> MotionRefusal {
        MotionRefusal::ShapesDiffer(Box::new(s))
    }
}

/// `K` READ FROM H, PER COMPONENT: the whole turns of each turning in both
/// senses, `K_c = ⌊|θ_c| / 360⌋`, in key order; refused
/// `OrbitsPastCarrier` where one passes `K`'s carrier.
pub(crate) fn orbits(lanes: &Lanes) -> Result<Vec<u64>, MotionRefusal> {
    let turn = Q::from_integer(BigInt::from(DEGREES_PER_TURN));
    lanes
        .turnings()
        .map(|(key, theta)| {
            let turns = (theta.abs() / &turn).floor().to_integer();
            turns.to_u64().ok_or_else(|| MotionRefusal::OrbitsPastCarrier { key: key.clone(), turns })
        })
        .collect()
}

/// THE BELL AT WHICH A COMPONENT REACHES A VALUE (78.173 fixture 8), a
/// named reading: reached at an exact bell, or none, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventBell {
    /// Reached at this exact bell, at or after the body's: `h + (v − x)/r`.
    /// It may fall between whole bells.
    At(Q),
    /// None: the component's rate is 0, so it never moves to the value.
    Unmoving,
    /// None: `(v − x)/r < 0`, the component stood at the value only before
    /// the body's bell, at `bell`, which no advance reaches.
    Behind { bell: Q },
}

/// WHERE A PASSAGE ACTS ON THE BODY: the key of S's component that holds the
/// throat plane's span, and the key of T's component that takes its flux.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Through {
    pub s: Key,
    pub t: Key,
}

/// A PASSAGE READ ON THE COORDINATE: the body past it, the frame state past
/// the pole swap (hold and reach exchanged, the whole `H + R` kept), and
/// the face past it, `1 − δ`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Passed {
    pub body: Coordinate,
    pub frame: Frame,
    pub share: Q,
}

impl Coordinate {
    /// THE ADVANCE TO BELL `t` (78.173 fixture 7): each component
    /// `x(t) = x(h) + r·(t − h)` under its rate, exactly, at the same
    /// origin, on the universe's own bell (TUniversal stamps and never
    /// drives); H's in degrees, `θ(t) = θ(h) + ω·(t − h)`, with `K` re-read
    /// per component. It composes: `h → a → t` is `h → t` for
    /// `h ≤ a ≤ t`. Refused `Backward` for `t < h`, `ShapesDiffer` where the
    /// rates are not keyed as the body's components, and
    /// `OrbitsPastCarrier`.
    pub fn advance(&self, rates: &Rates, t: u64) -> Result<Coordinate, MotionRefusal> {
        let span = t.checked_sub(self.local.h).ok_or(MotionRefusal::Backward { held: self.local.h, asked: t })?;
        let span = Q::from_integer(BigInt::from(span));
        let lanes = self.local.lanes.zip(&rates.lanes, |x, r| x + r * &span)?;
        let k = orbits(&lanes)?;
        Ok(Coordinate { origin: self.origin.clone(), local: Local { h: t, lanes, k } })
    }

    /// THE BELL AT WHICH COMPONENT `key` OF `lane` REACHES `value` under
    /// `rates` (78.173 fixture 8): `h + (v − x)/r`, exact, solved in first
    /// degree with no period assumed. None, by name, where the rate is 0 or
    /// the value lies behind the body's bell. Refused `NoComponent` where
    /// the body or the rates hold no such component.
    pub fn bell_of(&self, lane: &Lane, key: &Key, value: &Q, rates: &Rates) -> Result<EventBell, MotionRefusal> {
        let none = || MotionRefusal::NoComponent { lane: lane.clone(), key: key.clone() };
        let x = self.lane(lane).and_then(|v| v.get(key)).ok_or_else(none)?;
        let rate = rates.lanes.get(lane).and_then(|v| v.get(key)).ok_or_else(none)?;
        Ok(match rate.cmp(&Q::zero()) {
            Ordering::Equal => EventBell::Unmoving,
            _ => {
                let interval = (value - x) / rate;
                let bell = Q::from_integer(BigInt::from(self.local.h)) + &interval;
                match interval.cmp(&Q::zero()) {
                    Ordering::Less => EventBell::Behind { bell },
                    _ => EventBell::At(bell),
                }
            }
        })
    }

    /// A PASSAGE OF THE BODY THROUGH THE THROAT OF `poles`, its frame state
    /// `frame` (78.173 fixture 14; LAW-PASSAGE v3, `Poles::cross`), into
    /// `into`. The frame goes by the pole swap: hold and reach exchanged,
    /// `H + R` kept, the face `δ ↦ 1 − δ`. Within one enclosure (`into` is
    /// the body's origin) the passage is that face exchange alone, and the
    /// body is unchanged. Across a throat it is the re-origin: the body
    /// stands at `into`, S's component `through.s` reverses (`|S|` kept)
    /// and T's component `through.t` gains `−2·span`. Refused as the
    /// passage refuses the frame state or its share, `SpanDiffers` where
    /// the span is not the body's S, and `NoComponent`.
    pub fn pass(&self, poles: &Poles, frame: &Frame, through: &Through, into: &Origin) -> Result<Passed, MotionRefusal> {
        let share = pass_share(&frame.share()?)?;
        let state = poles.state(frame)?;
        match into == &self.origin {
            true => {
                let past = poles.read(&poles.swap(&state)?)?;
                Ok(Passed { body: self.clone(), frame: past, share })
            }
            false => {
                let none = |lane: Lane, key: &Key| MotionRefusal::NoComponent { lane, key: key.clone() };
                let held = self.lane(&Lane::S).and_then(|v| v.get(&through.s)).ok_or_else(|| none(Lane::S, &through.s))?;
                let flux = self.lane(&Lane::T).and_then(|v| v.get(&through.t)).ok_or_else(|| none(Lane::T, &through.t))?;
                if held != &frame.span() {
                    return Err(MotionRefusal::SpanDiffers { held: Box::new(held.clone()), read: Box::new(frame.span()) });
                }
                let crossed = poles.cross(&state, flux)?;
                let lanes = self
                    .local
                    .lanes
                    .with(&Lane::S, &through.s, crossed.span.clone())
                    .and_then(|l| l.with(&Lane::T, &through.t, crossed.flux.clone()))
                    .ok_or_else(|| none(Lane::T, &through.t))?;
                let body = Coordinate { origin: into.clone(), local: Local { h: self.local.h, lanes, k: self.local.k.clone() } };
                Ok(Passed { body, frame: poles.read(&crossed.frame)?, share })
            }
        }
    }
}

/// A MERGER (78.173 fixtures 9 and 13): the merged point, and each body's
/// offset from it per component, `x_a − x` and `x_b − x`. The
/// holding-weighted sum of the offsets is zero on every component; with the
/// holdings they give both bodies back ([`fission`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Merger {
    pub point: Coordinate,
    pub offset_a: Lanes,
    pub offset_b: Lanes,
}

/// THE MERGER OF `a` HOLDING `w_a` AND `b` HOLDING `w_b` (78.173 fixtures
/// 9 and 13): at their one origin and one bell, each component
/// `x = (w_a·x_a + w_b·x_b)/(w_a + w_b)`, `K` re-read per H component, and
/// each body's offset kept: no residual is discarded. Refused
/// `OriginsDiffer`, `BellsDiffer`, `NoHolding` where `w_a + w_b = 0`, and
/// `ShapesDiffer` where the bodies' components are keyed differently.
pub fn merge(a: &Coordinate, w_a: &Q, b: &Coordinate, w_b: &Q) -> Result<Merger, MotionRefusal> {
    if a.origin != b.origin {
        return Err(MotionRefusal::OriginsDiffer { held: Box::new(a.origin.clone()), asked: Box::new(b.origin.clone()) });
    }
    if a.local.h != b.local.h {
        return Err(MotionRefusal::BellsDiffer { held: a.local.h, asked: b.local.h });
    }
    let total = w_a + w_b;
    if total.is_zero() {
        return Err(MotionRefusal::NoHolding { a: Box::new(w_a.clone()), b: Box::new(w_b.clone()) });
    }
    let lanes = a.local.lanes.zip(&b.local.lanes, |x, y| (w_a * x + w_b * y) / &total)?;
    let offset_a = a.local.lanes.zip(&lanes, |x, m| x - m)?;
    let offset_b = b.local.lanes.zip(&lanes, |x, m| x - m)?;
    let k = orbits(&lanes)?;
    Ok(Merger { point: Coordinate { origin: a.origin.clone(), local: Local { h: a.local.h, lanes, k } }, offset_a, offset_b })
}

/// FISSION, THE INVERSE OF MERGER (78.173 fixtures 10 and 13): the merged
/// point and each offset give back the bodies, `x + offset` per component,
/// at the point's origin and bell, each `K` re-read from its H. Refused
/// `NoHolding` where `w_a + w_b = 0`, `ShapesDiffer`, and `Unbalanced`
/// where the holding-weighted sum of the offsets is not zero, since no
/// merger at these holdings left them.
pub fn fission(merger: &Merger, w_a: &Q, w_b: &Q) -> Result<(Coordinate, Coordinate), MotionRefusal> {
    if (w_a + w_b).is_zero() {
        return Err(MotionRefusal::NoHolding { a: Box::new(w_a.clone()), b: Box::new(w_b.clone()) });
    }
    let residual = merger.offset_a.zip(&merger.offset_b, |oa, ob| w_a * oa + w_b * ob)?;
    if residual.held().iter().any(|(_, v)| v.components().iter().any(|(_, r)| !r.is_zero())) {
        return Err(MotionRefusal::Unbalanced { residual: Box::new(residual) });
    }
    let body = |offset: &Lanes| -> Result<Coordinate, MotionRefusal> {
        let lanes = merger.point.local.lanes.zip(offset, |x, o| x + o)?;
        let k = orbits(&lanes)?;
        Ok(Coordinate { origin: merger.point.origin.clone(), local: Local { h: merger.point.local.h, lanes, k } })
    };
    Ok((body(&merger.offset_a)?, body(&merger.offset_b)?))
}

/// THE FLOW OF TWO PUSHES (78.173 fixture 15): `x ⊕ y = (x + y)/(1 + x·y)`,
/// exact, for pushes in `(−1, 1)`, so `1 + x·y > 0`. A product of two
/// pushes, never a square read as a quantity. Refused `NotPush` with the
/// first push outside `(−1, 1)`.
pub fn flow(x: &Q, y: &Q) -> Result<Q, MotionRefusal> {
    match [x, y].into_iter().find(|p| p.abs() >= Q::one()) {
        Some(p) => Err(MotionRefusal::NotPush(Box::new(p.clone()))),
        None => Ok((x + y) / (Q::one() + x * y)),
    }
}
