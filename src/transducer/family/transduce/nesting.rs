// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! WHICH FACTORINGS A FAMILY IS INVARIANT UNDER (MU-SENSING §3.6; ROADMAP
//! track 26 S4; ruled 2026-11-06 on #32). An axis is taken apart into nested
//! phases only where the family's relations are invariant under the
//! factoring; otherwise it stays one phase.
//!
//! 1. **Per pair.** A factoring is read on each pair's input and output,
//!    each at its own extent: the shape `a · b → a′ · b′` is stated per pair
//!    ([`PairNesting`]). A candidate is named across the family by its axis,
//!    its joint and its block counts on the two members ([`Candidate`]). The
//!    family keeps a candidate only where every pair admits it; a candidate
//!    one pair refuses is kept as that pair's residue ([`Unkept`]), never
//!    discarded.
//! 2. **Separators.** On a separated factoring (fine period `b + 1`), the
//!    separator is the symbol every site at the fine phase `b` holds: read,
//!    never declared ([`Reading::Separator`]).
//! 3. **Repeating blocks.** Two or more coarse blocks related by a motion of
//!    the fine phase, the identity included: which blocks, by which sense,
//!    is kept as content ([`BlockRelation`]), never a bare flag.
//! 4. **Motions per level.** The pair carried by the product of a motion of
//!    the coarse phase and a motion of the fine phase ([`LevelMotion`]), each
//!    kept or reversed and re-periodded onto the output's block count and
//!    block size, every other axis kept; held where that product speaks the
//!    input as the output exactly, with no residue.
//! 5. **Collapse.** A candidate collapses the axis's vortex when a reading
//!    of 2–4 holds for it on every pair and for no other kept candidate
//!    ([`Taken::Apart`]). Where several hold alike they stay superposed
//!    ([`Taken::Held`]): none is chosen, and the axis is not taken apart.
//!
//! A pair admits a candidate where some reading of 2–4 holds on it. Only
//! the readings that hold on every pair are the family's.

use super::{Factoring, Factorings, Joint, Sense};
use crate::transducer::sample::{AxisReading, Sample};

/// TWO BLOCKS OF ONE SAMPLE RELATED: block `to` is block `from` with its
/// fine phase carried in `sense`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockRelation {
    pub from: usize,
    pub to: usize,
    pub sense: Sense,
}

/// A MOTION PER LEVEL: the coarse phase and the fine phase each carried in
/// its sense.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LevelMotion {
    pub coarse: Sense,
    pub fine: Sense,
}

/// THE KIND OF A READING THAT HOLDS A FACTORING (points 2–4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reading {
    Separator,
    Repeat,
    Level,
}

/// A MEMBER OF A PAIR.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Member {
    Input,
    Output,
}

/// ONE PAIR READ UNDER ONE CANDIDATE: its two factorings, and what holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairNesting {
    /// The input's factoring, at its own extent.
    pub input: Factoring,
    /// The output's factoring, at its own extent.
    pub output: Factoring,
    /// The separator symbol each member shows, where both show one.
    pub separators: Option<(Vec<u8>, Vec<u8>)>,
    /// The block relations each member shows, where both show some.
    pub repeats: Option<(Vec<BlockRelation>, Vec<BlockRelation>)>,
    /// Every motion per level that carries the pair exactly.
    pub level: Vec<LevelMotion>,
}

impl PairNesting {
    /// The kinds of reading that hold on the pair, in kind order.
    pub fn readings(&self) -> Vec<Reading> {
        let mut out = Vec::new();
        if self.separators.is_some() {
            out.push(Reading::Separator);
        }
        if self.repeats.is_some() {
            out.push(Reading::Repeat);
        }
        if !self.level.is_empty() {
            out.push(Reading::Level);
        }
        out
    }
}

/// A CANDIDATE NAMED ACROSS THE FAMILY: the axis, the joint, and the block
/// counts on the input and on the output.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Candidate {
    pub axis: usize,
    pub joint: Joint,
    pub coarse: (usize, usize),
}

/// WHY A PAIR REFUSES A CANDIDATE, named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NestRefusal {
    /// The member's extent along the axis has no factoring of the
    /// candidate's joint and block count.
    Unfactored { member: Member, extent: usize },
    /// Both factor, and no reading of 2–4 holds on the pair.
    Unread,
}

/// A CANDIDATE EVERY PAIR ADMITS, with each pair's reading.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Held {
    pub candidate: Candidate,
    pub pairs: Vec<PairNesting>,
}

impl Held {
    /// THE FAMILY'S READINGS: the kinds that hold on every pair.
    pub fn readings(&self) -> Vec<Reading> {
        let mut pairs = self.pairs.iter().map(PairNesting::readings);
        let Some(first) = pairs.next() else { return Vec::new() };
        pairs.fold(first, |acc, r| acc.into_iter().filter(|k| r.contains(k)).collect())
    }
}

/// A CANDIDATE SOME PAIR REFUSES: kept as those pairs' residue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unkept {
    pub candidate: Candidate,
    /// Each refusing pair's index in the family and why.
    pub refused: Vec<(usize, NestRefusal)>,
}

/// WHETHER THE AXIS IS TAKEN APART.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taken {
    /// No candidate is kept: the axis is one phase.
    Whole,
    /// Kept candidate `candidate` collapsed the vortex by reading `by`.
    Apart { candidate: usize, by: Reading },
    /// Candidates are kept and none collapses: held superposed, the axis
    /// not taken apart.
    Held,
}

/// ONE AXIS OF THE FAMILY READ FOR NESTING.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AxisNesting {
    pub axis: usize,
    /// The candidates every pair admits, in candidate order.
    pub kept: Vec<Held>,
    /// The candidates some pair refuses, each with its refusals.
    pub residue: Vec<Unkept>,
    pub taken: Taken,
}

/// THE NESTING OF EVERY AXIS OF A FAMILY'S TRAIN PAIRS, all of one rank
/// (the percept refuses two ranks before this is read). A family of no
/// pair reads no axis.
pub fn nesting_of(train: &[(Sample, Sample)]) -> Vec<AxisNesting> {
    // Every sample is of the family's one rank; the fold reads it from all.
    let rank = train.iter().map(|(x, _)| x.rank()).fold(0, usize::max);
    (0..rank).map(|axis| axis_nesting(train, axis)).collect()
}

/// The extent of `s` along `axis`, or 0 past its rank (no factoring).
fn extent(s: &Sample, axis: usize) -> usize {
    match s.extent(axis) {
        AxisReading::Read(e) => e,
        AxisReading::Unread => 0,
    }
}

/// The factoring of `e` of `joint` with `coarse` blocks, if any.
fn factoring_of(e: usize, joint: Joint, coarse: usize) -> Option<Factoring> {
    Factorings::of_axis(e).candidates().iter().copied().find(|f| f.joint() == joint && f.coarse() == coarse)
}

/// One axis: every candidate any pair's extents name, read on every pair.
fn axis_nesting(train: &[(Sample, Sample)], axis: usize) -> AxisNesting {
    let mut candidates: Vec<Candidate> = Vec::new();
    for (x, y) in train {
        let (fx, fy) = (Factorings::of_axis(extent(x, axis)), Factorings::of_axis(extent(y, axis)));
        for a in fx.candidates() {
            for b in fy.candidates().iter().filter(|b| b.joint() == a.joint()) {
                candidates.push(Candidate { axis, joint: a.joint(), coarse: (a.coarse(), b.coarse()) });
            }
        }
    }
    candidates.sort_unstable();
    candidates.dedup();
    let (mut kept, mut residue) = (Vec::new(), Vec::new());
    for c in candidates {
        let mut pairs = Vec::with_capacity(train.len());
        let mut refused = Vec::new();
        for (i, (x, y)) in train.iter().enumerate() {
            match pair_nesting(x, y, &c) {
                Ok(p) => pairs.push(p),
                Err(why) => refused.push((i, why)),
            }
        }
        match refused.is_empty() {
            true => kept.push(Held { candidate: c, pairs }),
            false => residue.push(Unkept { candidate: c, refused }),
        }
    }
    let taken = taken_of(&kept);
    AxisNesting { axis, kept, residue, taken }
}

/// Point 5: the one kept candidate a reading holds for and no other, or
/// none.
fn taken_of(kept: &[Held]) -> Taken {
    if kept.is_empty() {
        return Taken::Whole;
    }
    let readings: Vec<Vec<Reading>> = kept.iter().map(Held::readings).collect();
    let mut collapsing = readings.iter().enumerate().filter_map(|(i, own)| {
        let by = own.iter().copied().find(|r| readings.iter().enumerate().all(|(j, other)| j == i || !other.contains(r)))?;
        Some((i, by))
    });
    match (collapsing.next(), collapsing.next()) {
        (Some((candidate, by)), None) => Taken::Apart { candidate, by },
        _ => Taken::Held,
    }
}

/// Points 1–4 on one pair under one candidate.
fn pair_nesting(x: &Sample, y: &Sample, c: &Candidate) -> Result<PairNesting, NestRefusal> {
    let (ex, ey) = (extent(x, c.axis), extent(y, c.axis));
    let input = factoring_of(ex, c.joint, c.coarse.0).ok_or(NestRefusal::Unfactored { member: Member::Input, extent: ex })?;
    let output = factoring_of(ey, c.joint, c.coarse.1).ok_or(NestRefusal::Unfactored { member: Member::Output, extent: ey })?;
    let separators = separator(x, c.axis, &input).zip(separator(y, c.axis, &output));
    let (rx, ry) = (relations(x, c.axis, &input), relations(y, c.axis, &output));
    let repeats = (!rx.is_empty() && !ry.is_empty()).then_some((rx, ry));
    let senses = [Sense::Kept, Sense::Reversed];
    let level = senses
        .iter()
        .flat_map(|&coarse| senses.iter().map(move |&fine| LevelMotion { coarse, fine }))
        .filter(|m| carries(x, y, c.axis, &input, &output, *m))
        .collect();
    let p = PairNesting { input, output, separators, repeats, level };
    match p.readings().is_empty() {
        true => Err(NestRefusal::Unread),
        false => Ok(p),
    }
}

/// The coordinate of `site` along `axis` as an index, or `None`.
fn at_axis(site: &crate::transducer::base::Site, axis: usize) -> Option<usize> {
    site.axis(axis).and_then(|c| usize::try_from(c).ok())
}

/// Point 2: the one symbol every separator site holds, or `None` where the
/// factoring abuts or the separator sites hold two symbols.
fn separator(s: &Sample, axis: usize, f: &Factoring) -> Option<Vec<u8>> {
    if f.joint() != Joint::Separated {
        return None;
    }
    let mut held: Option<Vec<u8>> = None;
    for site in s.sites() {
        if at_axis(&site, axis)? % f.fine() != f.block() {
            continue;
        }
        let symbol = s.symbol_at(&site).ok()?.bytes().to_vec();
        match &held {
            Some(h) if *h != symbol => return None,
            Some(_) => {}
            None => held = Some(symbol),
        }
    }
    held
}

/// Point 3: every pair of blocks `from < to` related by a sense of the fine
/// phase; a block of one site has the one sense.
fn relations(s: &Sample, axis: usize, f: &Factoring) -> Vec<BlockRelation> {
    let senses: &[Sense] = match f.block() {
        1 => &[Sense::Kept],
        _ => &[Sense::Kept, Sense::Reversed],
    };
    let mut out = Vec::new();
    for from in 0..f.coarse() {
        for to in from + 1..f.coarse() {
            for &sense in senses {
                if blocks_related(s, axis, f, from, to, sense) {
                    out.push(BlockRelation { from, to, sense });
                }
            }
        }
    }
    out
}

/// The fine coordinate `j` of a block of `b` sites carried in `sense`.
fn sensed(j: usize, b: usize, sense: Sense) -> usize {
    match sense {
        Sense::Kept => j,
        Sense::Reversed => b - 1 - j,
    }
}

/// Whether block `to` is block `from` with its fine phase in `sense`, site
/// for site, every other coordinate kept.
fn blocks_related(s: &Sample, axis: usize, f: &Factoring, from: usize, to: usize, sense: Sense) -> bool {
    s.sites().all(|site| {
        let Some(c) = at_axis(&site, axis) else { return false };
        let (i, j) = (c / f.fine(), c % f.fine());
        if i != from || j >= f.block() {
            return true;
        }
        let there = f.site(to, sensed(j, f.block(), sense)).and_then(|t| i64::try_from(t).ok()).and_then(|t| site.with_axis(axis, t));
        match there {
            Some(t) => s.symbol_at(&site) == s.symbol_at(&t),
            None => false,
        }
    })
}

/// Point 4: whether the motion per level `m` speaks `x` as `y` exactly:
/// every other axis of one extent and kept; the output's block `i′` reads
/// the input's block `⌊i′·a/a′⌋` and its fine site `j′` the input's
/// `⌊j′·b/b′⌋`, each then in its sense, where `a` divides `a′` and `b`
/// divides `b′` (each level held or stretched onto its own sites); a
/// separator site reads the input's separator.
fn carries(x: &Sample, y: &Sample, axis: usize, fx: &Factoring, fy: &Factoring, m: LevelMotion) -> bool {
    let others_alike = (0..x.rank()).all(|k| k == axis || x.extent(k) == y.extent(k));
    let (a, a2, b, b2) = (fx.coarse(), fy.coarse(), fx.block(), fy.block());
    if !others_alike || x.width() != y.width() || a2 % a != 0 || b2 % b != 0 {
        return false;
    }
    let sep = separator(x, axis, fx);
    y.sites().all(|site| {
        let Some(c) = at_axis(&site, axis) else { return false };
        let (i2, j2) = (c / fy.fine(), c % fy.fine());
        let Ok(spoken) = y.symbol_at(&site) else { return false };
        if j2 >= fy.block() {
            return sep.as_deref() == Some(spoken.bytes());
        }
        let i = sensed(i2 / (a2 / a), a, m.coarse);
        let j = sensed(j2 / (b2 / b), b, m.fine);
        let source = fx.site(i, j).and_then(|t| i64::try_from(t).ok()).and_then(|t| site.with_axis(axis, t));
        source.and_then(|t| x.symbol_at(&t).ok()) == Some(spoken)
    })
}
