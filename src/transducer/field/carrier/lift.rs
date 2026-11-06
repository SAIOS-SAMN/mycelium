// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §3.6–§3.7, ROADMAP track 26 S4a — THE SAMPLE LIFTED INTO THE
//! FIELD at any rank: each body on its chart, each boundary point held by its
//! phases in degrees on named turning planes, the winding kept; an axis taken apart.

use super::atlas::{Atlas, Chart};
use super::enclosing::Membrane;
use super::refusal::FieldRefusal;
use super::turning::{FramePlanes, Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::family::transduce::{Enclosed, Factoring, Turn};
use crate::transducer::sample::Sample;
use crate::transducer::stalk::ChartAxis;
use forms::coordinate::{Square, DEGREES_PER_TURN};
use num_bigint::BigInt;

/// A POINT OF THE FIELD (78.171 §2): one phase per axis, each a turning
/// exact in degrees on its named plane with its winding. No unit norm, no
/// squared radius and no coordinate on a reference surface: the phases are
/// the point.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub phases: Vec<Turning>,
}

/// A BODY LIFTED: the chart it stands on, its symbol (a fibre value,
/// compared by equality alone), and its boundary record, each band's least
/// and greatest corner as points of the field. A body is held by its
/// boundary, never by its interior sites.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiftedBody {
    pub chart: Chart,
    pub symbol: Vec<u8>,
    pub boundary: Vec<(Point, Point)>,
}

/// A SAMPLE LIFTED INTO THE FIELD (MU-SENSING §3.7, *the lift*): its frame
/// planes, the turning plane each axis's phase stands on, its membrane, and
/// its bodies.
///
/// THE PLANES. A sample of rank `n` is lifted onto `n + 1` frame planes:
/// one per axis and the lift's own, plane `n + 1`. Axis `i`'s phase turns on
/// `e₊ᵢe₊₍ₙ₊₁₎`, the turning of the axis into the lift's plane, so no axis is
/// privileged and every axis stands on a plane whose two units square
/// alike (FORMS-COORDINATE v4). At rank 1 that plane is `e₊₁e₊₂`. The
/// sample's membrane is the throat of the lift's plane: the sample encloses
/// what it holds.
///
/// THE PHASE ORIGIN (78.171 §8, the convention agreed 2026-11-06 with
/// netstratum docs/117 §11.6). Every axis keeps the one shared lift unit
/// `e₊₍ₙ₊₁₎`, and its phase is measured from it: phase 0 lies on
/// `e₊₍ₙ₊₁₎`, never on `e₊ᵢ`, and a phase `θ` is `cos θ·e₊₍ₙ₊₁₎ − sin θ·e₊ᵢ`.
/// A reversal of axis `i` is then the reflection in `e₊ᵢ` alone, and a
/// transposition `(i j)` the quarter turning on `e₊ᵢe₊ⱼ` then the flip of
/// `e₊ᵢ` (`carrier::motion_of`). No degree written by the lift moves.
#[derive(Clone, Debug)]
pub struct Lifted {
    pub frame: FramePlanes,
    pub planes: Vec<TurningPlane>,
    pub membrane: Membrane,
    pub bodies: Vec<LiftedBody>,
}

/// THE DEGREES OF THE BOUNDARY `s` ON A PHASE OF PERIOD `period`:
/// `360°·s/period`. The far side of the seam, `s = period`, is `360°`,
/// `K = 1`, never read back as `0°`.
fn degrees_of_boundary(s: usize, period: usize) -> Q {
    Q::new(BigInt::from(s) * BigInt::from(DEGREES_PER_TURN), BigInt::from(period))
}

/// The strides of canonical address order: the last axis fastest.
fn strides_of(periods: &[usize]) -> Vec<usize> {
    let mut strides = vec![1usize; periods.len()];
    for k in (1..periods.len()).rev() {
        if let (Some(&s), Some(&p)) = (strides.get(k), periods.get(k)) {
            if let Some(slot) = strides.get_mut(k - 1) {
                *slot = s * p;
            }
        }
    }
    strides
}

/// EACH BODY'S CHART, read at the sensor once: its rank, its measure (the
/// sites it holds), and its boundary's faces of grade `n − 1` (a site's
/// side whose neighbour across it is off the sample or another body's).
/// Each is invariant under the field's signed permutations of the phases
/// and its translations; the class is coarsest first, each a count.
/// Refused `Malformed` with a count no `u64` coordinate holds.
fn charts_of(e: &Enclosed) -> Result<Vec<Chart>, FieldRefusal> {
    let periods = e.enclosure.periods();
    let strides = strides_of(periods);
    let sites: usize = periods.iter().product();
    let mut owner: Vec<Option<usize>> = vec![None; sites];
    for (band, b) in e.bands.iter().enumerate() {
        let mut offsets = vec![0usize];
        for (&(lo, hi), &stride) in b.spans.iter().zip(&strides) {
            offsets = offsets.iter().flat_map(|&o| (lo..hi).map(move |c| o + c * stride)).collect();
        }
        for o in offsets {
            if let Some(slot) = owner.get_mut(o) {
                *slot = e.body_of.get(band).copied();
            }
        }
    }
    let mut measure = vec![0usize; e.bodies];
    let mut faces = vec![0usize; e.bodies];
    for (o, body) in owner.iter().enumerate().filter_map(|(o, b)| Some((o, (*b)?))) {
        if let Some(m) = measure.get_mut(body) {
            *m += 1;
        }
        for (&p, &stride) in periods.iter().zip(&strides) {
            let c = (o / stride) % p;
            let before = (c > 0).then(|| owner.get(o - stride).copied().flatten()).flatten();
            let after = (c + 1 < p).then(|| owner.get(o + stride).copied().flatten()).flatten();
            let open = [before, after].iter().filter(|n| **n != Some(body)).count();
            if let Some(f) = faces.get_mut(body) {
                *f += open;
            }
        }
    }
    let rank = periods.len();
    let count = |x: usize| u64::try_from(x).map(ChartAxis::Count).map_err(|_| FieldRefusal::Malformed { at: x });
    measure.into_iter().zip(faces).map(|(m, f)| Ok(Chart::of(vec![count(rank)?, count(m)?, count(f)?]))).collect()
}

impl Lifted {
    /// THE SAMPLE LIFTED. Refused `PastFrame` where the rank's frame planes
    /// pass the algebra's masks.
    pub fn of(sample: &Sample) -> Result<Lifted, FieldRefusal> {
        let e = Enclosed::of(sample);
        let periods = e.enclosure.periods().to_vec();
        let rank = u32::try_from(periods.len()).map_err(|_| FieldRefusal::PastFrame { plane: u32::MAX })?;
        let own = rank.checked_add(1).ok_or(FieldRefusal::PastFrame { plane: rank })?;
        let frame = FramePlanes::of(own)?;
        let planes = (1..=rank).map(|i| TurningPlane::of(&frame, Square::Positive, i, own)).collect::<Result<Vec<_>, _>>()?;
        let membrane = Membrane::of(&frame, own)?;
        let charts = charts_of(&e)?;
        let corner = |pick: fn((usize, usize)) -> usize, spans: &[(usize, usize)]| -> Result<Point, FieldRefusal> {
            let phases = spans
                .iter()
                .zip(&periods)
                .zip(&planes)
                .map(|((&span, &p), plane)| Turning::of(plane.clone(), degrees_of_boundary(pick(span), p)))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Point { phases })
        };
        let mut bodies = Vec::with_capacity(e.bodies);
        for (body, chart) in charts.into_iter().enumerate() {
            let symbol = e.body_symbol(body).map(|s| s.bytes().to_vec()).ok_or(FieldRefusal::Malformed { at: body })?;
            let boundary = e
                .body_bands(body)
                .map(|(_, b)| Ok((corner(|s| s.0, &b.spans)?, corner(|s| s.1, &b.spans)?)))
                .collect::<Result<Vec<_>, FieldRefusal>>()?;
            bodies.push(LiftedBody { chart, symbol, boundary });
        }
        Ok(Lifted { frame, planes, membrane, bodies })
    }

    /// THE ATLAS THE LIFT STANDS ON: the bodies' charts on its frame, no
    /// attractor held, so flat. A sample holds no orbit; orbits are held
    /// over pairs and thoughts (ROADMAP track 26 S9).
    pub fn atlas(&self) -> Atlas {
        Atlas::of(self.frame, self.bodies.iter().map(|b| b.chart.clone()).collect())
    }
}

/// A PHASE TAKEN APART (MU-SENSING §3.6; 78.171 fixture 4): the site's
/// turnings at each level of the factoring, coarsest first, each of its own
/// level's period, on one plane.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nested {
    pub coarse: Turning,
    pub fine: Turning,
    factoring: Factoring,
}

impl Nested {
    /// SITE `s` OF A PHASE OF PERIOD `a·b` on `plane`, taken apart by
    /// `factoring`: `(⌊s/b⌋/a, (s mod b)/b)` as degrees. Refused `Malformed`
    /// with `s` where it is not a site of the phase.
    pub fn of(plane: &TurningPlane, factoring: Factoring, s: usize) -> Result<Nested, FieldRefusal> {
        let (c, f) = factoring.nested(s).ok_or(FieldRefusal::Malformed { at: s })?;
        let degrees = |t: Turn| t.value() * Q::from_integer(BigInt::from(DEGREES_PER_TURN));
        Ok(Nested { coarse: Turning::of(plane.clone(), degrees(c))?, fine: Turning::of(plane.clone(), degrees(f))?, factoring })
    }

    /// THE PHASE RECOMPOSED: `θ_coarse + θ_fine / a`, the site's turning on
    /// the one phase.
    pub fn recomposed(&self) -> Result<Turning, FieldRefusal> {
        let a = Q::from_integer(BigInt::from(self.factoring.coarse()));
        Turning::of(self.coarse.plane().clone(), self.coarse.degrees() + self.fine.degrees() / a)
    }
}
