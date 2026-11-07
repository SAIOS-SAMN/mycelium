// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ROADMAP track 26 S7 stage 4 (the rest of S4a), MU-SENSING §3.9 — A
//! FAMILY'S PAIRS LIFTED AS TRANSITIONS between their bodies' charts: the
//! factorisation's motion, a signed permutation of the phases and the
//! re-period of each moved phase, written as a motion of the field.
//!
//! THE LIFT'S CONVENTION (78.171 §8, agreed 2026-11-06 with netstratum
//! docs/117 §11.6): every axis keeps the one shared lift unit `e₊₍ₙ₊₁₎` and
//! its phase is measured from it (`Lifted`). On that convention every
//! signed permutation lifts whole, and none is refused:
//!
//! - **A reversal of axis `i`** (`t ↦ −t`) is the reflection in `e₊ᵢ`
//!   alone, `x ↦ −e₊ᵢ·x·e₊ᵢ`, every other axis fixed. The transduction's
//!   reversal writes the boundary `p − s` (`Turn::reversed`), so phase `θ`
//!   reads `360° − θ`: the reflection, then one whole turning on the axis's
//!   own plane, which is the identity as a motion and carries the winding
//!   the transduction writes (the far seam is `360°`, `K = 1`, never `0°`).
//!   The two stand as their own generators in the word, never folded: the
//!   flip counts `{e₊ᵢ: 1}` on P, and the turning carries `K = 1` (confirmed
//!   on #134, 2026-11-07; 78.171 §8).
//! - **A transposition `(i j)`** is the quarter turning on `e₊ᵢe₊ⱼ`, then
//!   the flip of `e₊ᵢ`.
//! - **A re-period** of output axis `j` is `RePeriod` from the period the
//!   phase carries to the output's.

use super::atlas::Chart;
use super::holonomy::Motion;
use super::lift::Lifted;
use super::motions::{FieldMotion, Generator, Unit};
use super::refusal::FieldRefusal;
use super::turning::{FramePlanes, Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::family::transduce::{Enclosed, Enclosure, Factorisation, Motion as Transduced, Sense, TransductionRefusal};
use crate::transducer::sample::Sample;
use forms::coordinate::{Square, DEGREES_PER_TURN};
use num_bigint::BigInt;

/// A PAIR'S BODY CARRIED AS A TRANSITION: the chart of the input's body,
/// the chart of the output's body it is carried onto, and the motion of the
/// field that carries it (the factorisation's motion, lifted).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairTransition {
    pub from: Chart,
    pub to: Chart,
    pub motion: FieldMotion,
}

/// Frame plane `k`'s unit `e₊ₖ`, for axis `k − 1`.
fn axis_unit(axis: usize) -> Result<Unit, FieldRefusal> {
    let plane = u32::try_from(axis + 1).map_err(|_| FieldRefusal::PastFrame { plane: u32::MAX })?;
    Ok(Unit { square: Square::Positive, plane })
}

/// The turning by `degrees` on `e₊ₖe₊ₗ` of `frame`, as one generator.
fn turn(frame: &FramePlanes, k: u32, l: u32, degrees: i64) -> Result<Generator, FieldRefusal> {
    let plane = TurningPlane::of(frame, Square::Positive, k, l)?;
    Ok(Generator::Turn(Motion::of(vec![Turning::of(plane, Q::from_integer(BigInt::from(degrees)))?])?))
}

/// The machine count `x` as a frame plane index.
fn plane_index(x: usize) -> Result<u32, FieldRefusal> {
    u32::try_from(x).map_err(|_| FieldRefusal::PastFrame { plane: u32::MAX })
}

/// THE FACTORISATION'S MOTION LIFTED (MU-SENSING §3.9, the motion): the
/// signed permutation of `motion`, taken from a frame of enclosure `from`
/// and re-periodded onto `motion.onto`, as one motion of the field on
/// `frame`, the frame a lift of rank `n` stands on (`n + 1` planes).
///
/// The word, first acting first: each source axis read reversed is
/// reflected in its unit and turned one whole turn on its plane; the
/// transpositions then carry each source axis to the output axis that
/// reads it; each output axis whose period moves is re-periodded. Refused
/// `Transduction` where the motion's rank is not the enclosure's, and as
/// the frame refuses its planes.
pub fn motion_of(frame: &FramePlanes, from: &Enclosure, motion: &Transduced) -> Result<FieldMotion, FieldRefusal> {
    let permutation = &motion.permutation;
    let moved = permutation.enclosure(from).map_err(FieldRefusal::Transduction)?;
    let rank = permutation.rank();
    if motion.onto.rank() != rank {
        return Err(FieldRefusal::Transduction(TransductionRefusal::Rank { from: rank, to: motion.onto.rank() }));
    }
    let own = plane_index(rank + 1)?;
    let whole = i64::from(DEGREES_PER_TURN);
    let mut word = FieldMotion::identity();
    for &(source, sense) in permutation.images() {
        if sense == Sense::Reversed {
            let k = axis_unit(source)?;
            word = word.then(&FieldMotion::of(Generator::Flip(k))).then(&FieldMotion::of(turn(frame, k.plane, own, whole)?));
        }
    }
    // Where each source axis stands as the transpositions carry it: the
    // content of slot `p` is source axis `at[p]`.
    let mut at: Vec<usize> = (0..rank).collect();
    for (j, &(source, _)) in permutation.images().iter().enumerate() {
        let Some(q) = at.iter().position(|&a| a == source) else { return Err(FieldRefusal::Malformed { at: j }) };
        if q != j {
            let (lo, hi) = (j.min(q), j.max(q));
            let (i, l) = (plane_index(lo + 1)?, plane_index(hi + 1)?);
            word = word.then(&FieldMotion::of(turn(frame, i, l, whole / 4)?)).then(&FieldMotion::of(Generator::Flip(axis_unit(lo)?)));
            at.swap(j, q);
        }
    }
    for (axis, (&carried, &onto)) in moved.periods().iter().zip(motion.onto.periods()).enumerate() {
        if carried != onto {
            let period = |p: usize| u64::try_from(p).map_err(|_| FieldRefusal::Malformed { at: p });
            word = word.then(&FieldMotion::of(Generator::RePeriod { axis, from: period(carried)?, to: period(onto)? }));
        }
    }
    Ok(word)
}

/// THE PAIR `(x, y)` LIFTED AS TRANSITIONS UNDER ITS FACTORISATION `f`:
/// for each body of the moved frame, the transition from its chart to the
/// chart of the output body it is carried onto, by the lifted motion. A body
/// the motion does not carry whole keeps its refusal (`Mixed`, split or
/// merged), never dropped. The moved frame's charts are the input's, since
/// a chart is invariant under the signed permutations of the phases.
/// Refused as [`motion_of`] and the lift refuse, and `Transduction` where
/// the motion does not speak `x`.
pub fn transitions_of(x: &Sample, y: &Sample, f: &Factorisation) -> Result<Vec<Result<PairTransition, TransductionRefusal>>, FieldRefusal> {
    let source = Lifted::of(x)?;
    let from = Enclosure::of(x);
    let motion = motion_of(&source.frame, &from, &f.motion)?;
    let permutation = &f.motion.permutation;
    let own = permutation.enclosure(&from).map_err(FieldRefusal::Transduction)?;
    let moved = permutation.spoken_onto(&Enclosed::of(x), &own).map_err(FieldRefusal::Transduction)?;
    let (moved, output) = (Lifted::of(&moved)?, Lifted::of(y)?);
    Ok(f.carried
        .iter()
        .enumerate()
        .map(|(k, carried)| {
            let to = (*carried)?;
            match (moved.bodies.get(k), output.bodies.get(to)) {
                (Some(a), Some(b)) => Ok(PairTransition { from: a.chart.clone(), to: b.chart.clone(), motion: motion.clone() }),
                _ => Err(TransductionRefusal::Oblique),
            }
        })
        .collect())
}
