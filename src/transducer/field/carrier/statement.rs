// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §0, LAW-FIELD v1 §5 (proposal 78.171) — THE FIELD STATED: an
//! atlas written as its objects (charts, attractors, laps, angles), never its
//! readings, and reconstructed through the carrier's own constructors.

use super::atlas::{Atlas, Attractor, Chart, Curvature, Orbit, Transition};
use super::enclosing::Membrane;
use super::holonomy::Motion;
use super::refusal::FieldRefusal;
use super::turning::{FramePlanes, Turning, TurningPlane};
use crate::core::Q;
use algebra::component::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use forms::coordinate::Square;
use num_bigint::BigInt;

/// WHY A STATED FIELD IS NOT ADMITTED: it does not reconstruct, or the
/// curvature the court derives from it differs from the curvature claimed,
/// each differing pair named, the claim beside the reconstruction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatementRefusal {
    Unreconstructed(FieldRefusal),
    Differs { pairs: Vec<(Option<Curvature>, Option<Curvature>)> },
}

/// A square's one byte in a statement.
fn square_byte(s: Square) -> u8 {
    match s {
        Square::Positive => 0,
        Square::Negative => 1,
    }
}

fn put_chart(out: &mut Vec<u8>, c: &Chart) {
    put_key_count(out, c.coordinates().len());
    c.coordinates().iter().for_each(|&x| put_key_count(out, x));
}

/// A turning written: its plane's square and two unit indices in the
/// plane's orientation, then its angle's numerator and denominator.
fn put_turning(out: &mut Vec<u8>, t: &Turning) {
    // A turning plane's key is two frame units of one square, by its
    // constructor; the first unit's square is the plane's.
    if let [a, b] = t.plane().key().axes() {
        out.push(a.square().map_or(0, square_byte));
        put_key_count(out, a.index() as usize);
        put_key_count(out, b.index() as usize);
    }
    put_key_bytes(out, &t.degrees().numer().to_signed_bytes_le());
    put_key_bytes(out, &t.degrees().denom().to_signed_bytes_le());
}

impl Atlas {
    /// THE FIELD STATED: the frame's plane count; the charts; then each
    /// attractor's centre, its membrane's plane and its lap, each step's
    /// two charts and its turnings (each plane's square and two indices,
    /// and its angle in degrees, exact). No reading is written: the
    /// holonomies, windings and orders are the reconstruction's to derive.
    pub fn put_statement(&self, out: &mut Vec<u8>) {
        put_key_count(out, self.frame().count() as usize);
        put_key_count(out, self.charts().len());
        self.charts().iter().for_each(|c| put_chart(out, c));
        put_key_count(out, self.attractors().len());
        for a in self.attractors() {
            put_chart(out, &a.centre);
            put_key_count(out, a.membrane.plane() as usize);
            put_key_count(out, a.orbit.lap().len());
            for t in a.orbit.lap() {
                put_chart(out, &t.from);
                put_chart(out, &t.to);
                put_key_count(out, t.motion.turnings().len());
                t.motion.turnings().iter().for_each(|x| put_turning(out, x));
            }
        }
    }
}

/// A reader over a statement, refusing `Malformed` at the byte it stopped.
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}

impl Reader<'_> {
    fn count(&mut self) -> Result<usize, FieldRefusal> {
        take_key_count(self.b, &mut self.p).ok_or(FieldRefusal::Malformed { at: self.p })
    }

    fn index(&mut self) -> Result<u32, FieldRefusal> {
        let at = self.p;
        u32::try_from(self.count()?).map_err(|_| FieldRefusal::Malformed { at })
    }

    fn integer(&mut self) -> Result<BigInt, FieldRefusal> {
        take_key_bytes(self.b, &mut self.p).map(|v| BigInt::from_signed_bytes_le(&v)).ok_or(FieldRefusal::Malformed { at: self.p })
    }

    fn chart(&mut self) -> Result<Chart, FieldRefusal> {
        let n = self.count()?;
        (0..n).map(|_| self.count()).collect::<Result<Vec<_>, _>>().map(Chart::of)
    }

    fn turning(&mut self, frame: &FramePlanes) -> Result<Turning, FieldRefusal> {
        let at = self.p;
        let square = match self.b.get(self.p) {
            Some(0) => Square::Positive,
            Some(1) => Square::Negative,
            _ => return Err(FieldRefusal::Malformed { at }),
        };
        self.p += 1;
        let (k, l) = (self.index()?, self.index()?);
        let plane = TurningPlane::of(frame, square, k, l)?;
        let (numer, denom) = (self.integer()?, self.integer()?);
        match denom == BigInt::from(0) {
            true => Err(FieldRefusal::Malformed { at }),
            false => Turning::of(plane, Q::new(numer, denom)),
        }
    }
}

/// THE FIELD RECONSTRUCTED FROM ITS STATEMENT: every plane, angle, motion,
/// lap and membrane rebuilt through the carrier's constructors, so a
/// motion is derived from its stated angle and a lap that does not close
/// is refused as [`Orbit::of`] refuses it. Refused `Malformed` at the byte
/// where the writing stops, or where bytes follow it.
pub fn reconstruct(stated: &[u8]) -> Result<Atlas, FieldRefusal> {
    let mut r = Reader { b: stated, p: 0 };
    let frame = FramePlanes::of(r.index()?)?;
    let charts = (0..r.count()?).map(|_| r.chart()).collect::<Result<Vec<_>, _>>()?;
    let mut atlas = Atlas::of(frame, charts);
    for _ in 0..r.count()? {
        let centre = r.chart()?;
        let membrane = Membrane::of(&frame, r.index()?)?;
        let mut lap = Vec::new();
        for _ in 0..r.count()? {
            let (from, to) = (r.chart()?, r.chart()?);
            let turnings = (0..r.count()?).map(|_| r.turning(&frame)).collect::<Result<Vec<_>, _>>()?;
            lap.push(Transition { from, to, motion: Motion::of(turnings)? });
        }
        atlas.hold(Attractor { centre, orbit: Orbit::of(lap)?, membrane })?;
    }
    match r.p == stated.len() {
        true => Ok(atlas),
        false => Err(FieldRefusal::Malformed { at: r.p }),
    }
}

/// THE COURT'S ADMISSION OF A STATED FIELD (MU-SENSING §0): the field
/// reconstructed from its statement, its curvature derived by the court,
/// and compared with the curvature the mind claims. Admitted with the
/// reconstruction where every attractor's reading agrees; refused
/// `Differs` with each pair that does not, or `Unreconstructed`. The
/// court never takes the claimed reading as its own.
pub fn admit(stated: &[u8], claimed: &[Curvature]) -> Result<Vec<Curvature>, StatementRefusal> {
    let reconstructed = reconstruct(stated).map_err(StatementRefusal::Unreconstructed)?.curvature();
    let steps = claimed.len().max(reconstructed.len());
    let pairs: Vec<(Option<Curvature>, Option<Curvature>)> = (0..steps)
        .map(|i| (claimed.get(i).cloned(), reconstructed.get(i).cloned()))
        .filter(|(c, r)| c != r)
        .collect();
    match pairs.is_empty() {
        true => Ok(reconstructed),
        false => Err(StatementRefusal::Differs { pairs }),
    }
}
