// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! #147 (ruled 2026-11-06; MU-ANASTOME §9.2) — A SEAT'S SERIES LIFTED INTO
//! THE FIELD. Each locator's series is a sample of rank one along its series
//! index, lifted as any source is (MU-SENSING §2, §3.7; "sensing any
//! source, any rank"). No record type is assigned a plane, a flow or a
//! turning here: the field deduces whether a series turns, flows or holds.
//!
//! - A Q, a BigQ or a Count is an EXACT VALUE, written out as its one
//!   rational (`put_q`), so `Count(3)`, `Q 3/1` and `BigQ 3/1` are one
//!   value and lift as one symbol.
//! - A Flag, a Key or a Bytes run is a SYMBOL, written out by its own
//!   coordinates and compared by equality alone.
//! - An Mv is a MOTION, never a symbol: a versor series is lifted as its
//!   exact versors along the index (`Motions`), related through the
//!   algebra (the step carrying one onto the next, their composition, each
//!   step's conjugacy class), never by equality.
//!
//! A turning read off a lifted series keeps its winding (`Lifted::of`: the
//! far seam is 360° with `K = 1`). The lift is a reading, derived where it
//! is read and never held (Concordance 1.13).

use super::change::{carrier, quantity};
use super::series::Series;
use crate::anastome::dialogue::seat::Arg;
use crate::transducer::address::{put_key_bytes, Address};
use crate::transducer::alphabet::rotor::put_q;
use crate::transducer::cga::{Conjugacy, Mv};
use crate::transducer::field::carrier::{FieldRefusal, Lifted};
use crate::transducer::sample::{Place, Sample, SampleRefusal};
use num_rational::BigRational as Q;
use num_traits::One;

/// FORMAT: the written value's leading coordinate, an exact value.
const VALUE: u8 = 0;
/// FORMAT: the leading coordinate of a flag's symbol.
const FLAG: u8 = 1;
/// FORMAT: the leading coordinate of a key's symbol.
const KEY: u8 = 2;
/// FORMAT: the leading coordinate of a bytes run's symbol.
const BYTES: u8 = 3;

/// WHY A SERIES DOES NOT LIFT, named with what was read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiftRefusal {
    /// No observation is held.
    Empty,
    /// A versor series holds a value that is no versor at `at`, or a value
    /// series holds a versor at `at`: a motion is never written as a
    /// symbol beside values, nor a value as a motion.
    Mixed { at: usize },
    /// No exact versor carries observation `at` onto the next one
    /// (`carrier` refused): the step is named, never read as standing still.
    Uncarried { at: usize },
    /// The rank-one sample was refused at intake.
    Sample(SampleRefusal),
    /// The field's carrier refused the sample.
    Field(FieldRefusal),
}

/// ONE VALUE WRITTEN OUT by its coordinates: an exact value by its
/// rational, a symbol by its kind and its own bytes. Self-delimiting, so a
/// writing padded to a series' width is still read as one value. `None`
/// for a versor, which is a motion.
fn written(a: &Arg) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    match a {
        Arg::Mv(_) => return None,
        Arg::Flag(f) => out.extend_from_slice(&[FLAG, u8::from(*f)]),
        Arg::Key(k) => {
            out.push(KEY);
            out.extend_from_slice(k);
        }
        Arg::Bytes(b) => {
            out.push(BYTES);
            put_key_bytes(&mut out, b);
        }
        Arg::Q { .. } | Arg::BigQ { .. } | Arg::Count(_) => {
            out.push(VALUE);
            put_q(&mut out, &quantity(a)?);
        }
    }
    Some(out)
}

/// A SERIES' VALUES AS A SAMPLE OF RANK ONE along its series index: one
/// site per observation held, oldest first, each the value written out at
/// the series' width (the widest writing; a shorter one is followed by
/// zero bytes, which its own lengths end before).
pub fn series_sample(s: &Series) -> Result<Sample, LiftRefusal> {
    let writings = s.seen().enumerate().map(|(at, seen)| written(&seen.value).ok_or(LiftRefusal::Mixed { at })).collect::<Result<Vec<_>, _>>()?;
    #[expect(clippy::disallowed_methods, reason = "rule 4 — the widest writing is the carrier's byte width, a bound every site is written within; no value or symbol is chosen by it")]
    let width = writings.iter().map(Vec::len).max().ok_or(LiftRefusal::Empty)?;
    let mut symbols = Vec::with_capacity(width * writings.len());
    for w in &writings {
        symbols.extend_from_slice(w);
        symbols.resize(symbols.len() + width - w.len(), 0);
    }
    Sample::new(vec![writings.len()], width, symbols, Place::new(Vec::new())).map_err(LiftRefusal::Sample)
}

/// A VERSOR SERIES AS MOTIONS: its exact versors along the series index,
/// oldest first. Every relation it answers is the algebra's.
#[derive(Clone, Debug)]
pub struct Motions {
    pub versors: Vec<Mv>,
}

impl Motions {
    /// EACH STEP, oldest first: the exact versor `cᵢ` with `cᵢ · vᵢ = vᵢ₊₁`
    /// (`carrier`), refused `Uncarried` at the first step none carries.
    pub fn steps(&self) -> Result<Vec<Mv>, LiftRefusal> {
        self.versors.iter().zip(self.versors.iter().skip(1)).enumerate().map(|(at, (a, b))| carrier(a, b).ok_or(LiftRefusal::Uncarried { at })).collect()
    }

    /// THE STEPS COMPOSED, the latest on the left: `cₙ₋₁ ⋯ c₀`, the motion
    /// carrying the first versor onto the last. The scalar one where the
    /// series holds one observation.
    pub fn composed(&self) -> Result<Mv, LiftRefusal> {
        Ok(self.steps()?.iter().fold(Mv::scalar(Q::one()), |acc, c| c.geo(&acc)))
    }

    /// EACH STEP'S CONJUGACY CLASS, written as its coordinates (54.7; A21),
    /// oldest first: `None` for a step whose class the algebra does not read.
    pub fn classes(&self) -> Result<Vec<Option<Address>>, LiftRefusal> {
        Ok(self.steps()?.iter().map(|c| c.class_key()).collect())
    }
}

/// A SERIES LIFTED: values and symbols into the field, or versors as
/// motions.
#[derive(Clone, Debug)]
pub enum SeriesLift {
    Field(Lifted),
    Motions(Motions),
}

/// THE SERIES LIFTED (#147): a series whose every observation is a versor
/// as `Motions`, otherwise its rank-one sample by `Lifted::of`. Refused
/// `Mixed` where versors stand beside values.
pub fn lift(s: &Series) -> Result<SeriesLift, LiftRefusal> {
    let first = s.seen().next().ok_or(LiftRefusal::Empty)?;
    if !matches!(first.value, Arg::Mv(_)) {
        let sample = series_sample(s)?;
        return Lifted::of(&sample).map(SeriesLift::Field).map_err(LiftRefusal::Field);
    }
    let versors = s
        .seen()
        .enumerate()
        .map(|(at, seen)| match &seen.value {
            Arg::Mv(v) => Ok(v.clone()),
            _ => Err(LiftRefusal::Mixed { at }),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SeriesLift::Motions(Motions { versors }))
}
