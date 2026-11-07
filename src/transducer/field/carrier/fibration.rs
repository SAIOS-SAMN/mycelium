// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ROADMAP track 26 S7 stage 4 (the rest of S4a), MU-SENSING §3.6–§3.7 —
//! NESTING AS A FIBRATION OF LEVELS where the family is invariant. An axis
//! the family's nesting takes apart (`transduce::nesting`, `Taken::Apart`)
//! adds a level, a field within the field: the coarse phase is the base,
//! one point per block, and over each base point the fibre is the block's
//! own field, lifted with its own atlas (78.170 O5, a cover read as `a`
//! sub-enclosures). An axis the nesting keeps whole, or holds superposed,
//! is no level. The tower is never flattened into one product.

use super::lift::Lifted;
use super::refusal::FieldRefusal;
use super::turning::{Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::family::transduce::{AxisNesting, Factoring, Member, Taken};
use crate::transducer::sample::Sample;
use forms::coordinate::DEGREES_PER_TURN;
use num_bigint::BigInt;

/// ONE LEVEL OF THE TOWER: the axis taken apart, its factoring, the base
/// (the coarse phase of each block, in degrees on the axis's own plane) and
/// the fibre over each base point (the block's sites, the separator left
/// out, lifted as a sample of its own).
#[derive(Clone, Debug)]
pub struct Level {
    pub axis: usize,
    pub factoring: Factoring,
    pub base: Vec<Turning>,
    pub fibres: Vec<Lifted>,
}

/// A SAMPLE LIFTED AS A FIBRATION: the whole lift, and one level per axis
/// taken apart, in axis order.
#[derive(Clone, Debug)]
pub struct Fibred {
    pub whole: Lifted,
    pub levels: Vec<Level>,
}

/// BLOCK `i` OF `sample` ALONG `axis` UNDER `factoring`: its sites in
/// canonical order, the axis's extent the block's. Refused `Malformed`
/// with the axis where the factoring is not the axis's extent.
fn block_of(sample: &Sample, axis: usize, factoring: &Factoring, i: usize) -> Result<Sample, FieldRefusal> {
    if sample.extents().get(axis) != Some(&factoring.extent()) {
        return Err(FieldRefusal::Malformed { at: axis });
    }
    let first = factoring.site(i, 0).ok_or(FieldRefusal::Malformed { at: i })?;
    let block = factoring.block();
    let mut symbols = Vec::new();
    for (site, symbol) in sample.sites().zip(sample.symbols()) {
        let at = site.axis(axis).and_then(|c| usize::try_from(c).ok()).ok_or(FieldRefusal::Malformed { at: axis })?;
        if at >= first && at < first + block {
            symbols.extend_from_slice(symbol.bytes());
        }
    }
    let mut extents = sample.extents().to_vec();
    if let Some(e) = extents.get_mut(axis) {
        *e = block;
    }
    Sample::new(extents, sample.width(), symbols, sample.place().clone()).map_err(|_| FieldRefusal::Malformed { at: i })
}

impl Level {
    /// THE LEVEL OF `sample` TAKEN APART ON `axis` BY `factoring`, the base
    /// point of block `i` the coarse turn `360°·i/a` on `plane`. Refused as
    /// a block and its lift refuse.
    pub fn of(sample: &Sample, plane: &TurningPlane, axis: usize, factoring: Factoring) -> Result<Level, FieldRefusal> {
        let coarse = factoring.coarse();
        let whole = BigInt::from(DEGREES_PER_TURN);
        let mut base = Vec::with_capacity(coarse);
        let mut fibres = Vec::with_capacity(coarse);
        for i in 0..coarse {
            base.push(Turning::of(plane.clone(), Q::new(BigInt::from(i) * &whole, BigInt::from(coarse)))?);
            fibres.push(Lifted::of(&block_of(sample, axis, &factoring, i)?)?);
        }
        Ok(Level { axis, factoring, base, fibres })
    }
}

impl Fibred {
    /// THE SAMPLE AS ITS FAMILY'S NESTING TAKES IT APART: `sample` is pair
    /// `pair`'s `member` of the family `nesting` was read on. Each axis the
    /// nesting takes apart (`Taken::Apart`) is a level by the kept
    /// candidate's factoring on that pair's member; an axis kept whole or
    /// held superposed is none. Refused `Malformed` with the pair where
    /// the kept candidate holds no reading of it, and as the lift refuses.
    pub fn of(sample: &Sample, nesting: &[AxisNesting], pair: usize, member: Member) -> Result<Fibred, FieldRefusal> {
        let whole = Lifted::of(sample)?;
        let mut levels = Vec::new();
        for n in nesting {
            let Taken::Apart { candidate, .. } = n.taken else { continue };
            let held = n.kept.get(candidate).ok_or(FieldRefusal::Malformed { at: candidate })?;
            let read = held.pairs.get(pair).ok_or(FieldRefusal::Malformed { at: pair })?;
            let factoring = match member {
                Member::Input => read.input,
                Member::Output => read.output,
            };
            let plane = whole.planes.get(n.axis).ok_or(FieldRefusal::Malformed { at: n.axis })?;
            levels.push(Level::of(sample, plane, n.axis, factoring)?);
        }
        Ok(Fibred { whole, levels })
    }
}
