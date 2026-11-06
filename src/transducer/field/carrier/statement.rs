// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! MU-SENSING §0, LAW-FIELD v1 §5 (proposal 78.171) — THE FIELD STATED: an
//! atlas written as its objects (charts, attractors, laps, angles), never its
//! readings, and reconstructed through the carrier's own constructors.

use super::atlas::{Atlas, Attractor, Chart, Curvature, Orbit, Transition};
use super::enclosing::Membrane;
use super::holding::{ExpansionRing, WellBody, WellLayer, WellStage};
use super::holonomy::Motion;
use super::refusal::FieldRefusal;
use super::turning::{FramePlanes, Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::stalk::ChartAxis;
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

/// A chart written: its coordinate count, then each coordinate as the
/// invariant writes it (`ChartAxis::key_bytes`: its kind, then its exact
/// value), so a ratio is written whole and read back exactly.
fn put_chart(out: &mut Vec<u8>, c: &Chart) {
    put_key_count(out, c.coordinates().len());
    c.coordinates().iter().for_each(|x| out.extend(x.key_bytes()));
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

    fn axis(&mut self) -> Result<ChartAxis, FieldRefusal> {
        let at = self.p;
        ChartAxis::take_axis(self.b, &mut self.p).ok_or(FieldRefusal::Malformed { at })
    }

    fn chart(&mut self) -> Result<Chart, FieldRefusal> {
        let n = self.count()?;
        (0..n).map(|_| self.axis()).collect::<Result<Vec<_>, _>>().map(Chart::of)
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

impl Reader<'_> {
    /// An atlas as [`Atlas::put_statement`] writes it, rebuilt through the
    /// carrier's constructors.
    fn atlas(&mut self) -> Result<Atlas, FieldRefusal> {
        let frame = FramePlanes::of(self.index()?)?;
        let charts = (0..self.count()?).map(|_| self.chart()).collect::<Result<Vec<_>, _>>()?;
        let mut atlas = Atlas::of(frame, charts);
        for _ in 0..self.count()? {
            let centre = self.chart()?;
            let membrane = Membrane::of(&frame, self.index()?)?;
            let mut lap = Vec::new();
            for _ in 0..self.count()? {
                let (from, to) = (self.chart()?, self.chart()?);
                let turnings = (0..self.count()?).map(|_| self.turning(&frame)).collect::<Result<Vec<_>, _>>()?;
                lap.push(Transition { from, to, motion: Motion::of(turnings)? });
            }
            atlas.hold(Attractor { centre, orbit: Orbit::of(lap)?, membrane })?;
        }
        Ok(atlas)
    }

    /// A stage as [`WellBody::put_statement`] writes it: the layer's byte,
    /// then the ring's.
    fn stage(&mut self) -> Result<WellStage, FieldRefusal> {
        let at = self.p;
        let layer = match self.b.get(at) {
            Some(0) => WellLayer::Perception,
            Some(1) => WellLayer::Thought,
            _ => return Err(FieldRefusal::Malformed { at }),
        };
        let ring = match self.b.get(at + 1) {
            Some(0) => ExpansionRing::Expanding,
            Some(1) => ExpansionRing::Rested,
            _ => return Err(FieldRefusal::Malformed { at: at + 1 }),
        };
        self.p = at + 2;
        Ok(WellStage { layer, ring })
    }

    /// Nothing past the writing: refused `Malformed` at the first byte
    /// that follows it.
    fn ended(&self) -> Result<(), FieldRefusal> {
        match self.p == self.b.len() {
            true => Ok(()),
            false => Err(FieldRefusal::Malformed { at: self.p }),
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
    let atlas = r.atlas()?;
    r.ended()?;
    Ok(atlas)
}

impl WellBody {
    /// THE WELL STATED (ROADMAP track 26 S7 stage 2): its own atlas stated
    /// (its planes, its charts, its one attractor: centre, membrane plane
    /// and lap), then its stage, the layer's byte (`0` perception, `1`
    /// thought) and the ring's (`0` expanding, `1` rested). No reading is
    /// written: the holonomy, precession and inflation are the court's to
    /// derive. Refused as [`WellBody::atlas`] refuses.
    pub fn put_statement(&self, out: &mut Vec<u8>) -> Result<(), FieldRefusal> {
        self.atlas()?.put_statement(out);
        out.push(match self.stage().layer {
            WellLayer::Perception => 0,
            WellLayer::Thought => 1,
        });
        out.push(match self.stage().ring {
            ExpansionRing::Expanding => 0,
            ExpansionRing::Rested => 1,
        });
        Ok(())
    }
}

/// A STATED WELL RECONSTRUCTED: its atlas rebuilt as [`reconstruct`]
/// rebuilds one, its one attractor taken as the well's, and its stage read.
/// Refused `Malformed` where the atlas holds other than one attractor (at
/// the byte after the atlas), where the stage does not decode, or where
/// bytes follow it.
pub fn reconstruct_well(stated: &[u8]) -> Result<WellBody, FieldRefusal> {
    let mut r = Reader { b: stated, p: 0 };
    let atlas = r.atlas()?;
    let [attractor] = atlas.attractors() else {
        return Err(FieldRefusal::Malformed { at: r.p });
    };
    let stage = r.stage()?;
    r.ended()?;
    WellBody::held(*atlas.frame(), attractor.clone(), stage)
}

/// THE COURT'S ADMISSION OF A STATED WELL: the well reconstructed, its
/// curvature (the precession of its local orbit per lap, or the refusal
/// that reading meets, `Loop` for a flat well) derived by the court and
/// compared with the curvature the mind claims. Admitted with the
/// reconstruction on a match; refused `Differs` with the pair, or
/// `Unreconstructed`.
pub fn admit_well(stated: &[u8], claimed: &Curvature) -> Result<WellBody, StatementRefusal> {
    let body = reconstruct_well(stated).map_err(StatementRefusal::Unreconstructed)?;
    let derived = Curvature { centre: body.centre().clone(), holonomy: body.precession() };
    match &derived == claimed {
        true => Ok(body),
        false => Err(StatementRefusal::Differs { pairs: vec![(Some(claimed.clone()), Some(derived))] }),
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
