// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2, §6 — SAMPLES ON THE WIRE (ROADMAP track 26, S5): the
// carrier a task envelope, a posted task and a solve's spoken outputs share.
//
//! A sample of any rank is written as its rank, its extents, its symbol
//! width, its place and its symbols:
//!
//! `[rank u16][extent u16 · rank][width u16][place count u16][coordinate
//! i64 · count][symbols, width · ∏ extents bytes]`
//!
//! The limits are derived, never chosen (MU-SENSING §6):
//! - the rank and each extent stand in the existing `u16` length field, and
//!   a value past it is refused, never widened ([`SampleWire::RankPastField`],
//!   [`SampleWire::ExtentPastField`]);
//! - the symbol width is the source's own, written once per sample;
//! - the symbol bytes `w · ∏ eᵢ` are counted with checked arithmetic and a
//!   count past the machine word is refused by name
//!   (`SampleRefusal::SitesOverflow`, `SampleRefusal::BytesOverflow`).
//!
//! A sample's place is its position in the family that carries it, `[split,
//! pair, member]` ([`position`]). Each decoder reads the place written and
//! refuses one that is not the sample's position
//! ([`SampleWire::PlaceOffPosition`]), so one family has one writing.
//!
//! A mind perceives the samples of any rank (`transduce::TaskPercept`). The
//! word machinery that spells a percept is the alphabet's, written on one
//! rank at one byte per symbol; its section is bridged here at the two ends
//! (MU-SENSING §2): [`TaskEnvelope::section`] and [`PostedTask::section`]
//! at intake, [`spoken_outputs`] at speech. A sample of another rank or
//! width is carried exactly, perceived, and refused by the section by name
//! (`sentence::SectionRefusal`: the alphabet's rank, or the width), never
//! truncated.

use crate::transducer::family::{Task, TrainPair};
use crate::transducer::spelled::SectionSample;
use crate::transducer::sample::{Place, Sample, SampleRefusal};
use crate::transducer::sentence::SectionRefusal;

/// WHY A SAMPLE IS NOT WRITTEN OR NOT READ, named with its cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleWire {
    /// The rank passes its `u16` field: a blob where a coordinate belongs,
    /// refused, never widened.
    RankPastField { rank: usize },
    /// An extent passes its `u16` field: refused, never widened.
    ExtentPastField { axis: usize, extent: usize },
    /// The symbol width passes its `u16` field.
    WidthPastField { width: usize },
    /// The place's coordinate count passes its `u16` field.
    PlacePastField { count: usize },
    /// The place written is not the sample's position in its family.
    PlaceOffPosition,
    /// A pair's index passes the place's `i64` coordinate.
    PositionPastField { pair: usize },
    /// A count of samples or pairs passes its one-byte field.
    CountPastField { count: usize },
    /// A withheld output's marker is neither `0` (absent) nor `1`
    /// (present).
    Marker { byte: u8 },
    /// The bytes end before the sample does, or bytes stand after the last.
    Short,
    /// The values read are no sample, refused by the sample's own name: no
    /// axis, an empty axis, no width, or the sites or the symbol bytes past
    /// the machine word.
    Sample(SampleRefusal),
}

/// A SAMPLE'S SPLIT IN A TASK: the train pairs, then the test pairs. The
/// order is the family's addressing order, the place's first coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Split {
    Train,
    Test,
}

/// A SAMPLE'S MEMBER IN ITS PAIR: the input, then the output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Member {
    Input,
    Output,
}

/// THE PLACE OF A SAMPLE AT `[split, pair, member]`: the coordinates a task
/// carries its samples at (`registry::task_address` reads the same).
/// Refused where the pair's index passes the place's coordinate.
pub fn position(split: Split, pair: usize, member: Member) -> Result<Place, SampleWire> {
    let split = match split {
        Split::Train => 0i64,
        Split::Test => 1i64,
    };
    let member = match member {
        Member::Input => 0i64,
        Member::Output => 1i64,
    };
    let at = i64::try_from(pair).map_err(|_| SampleWire::PositionPastField { pair })?;
    Ok(Place::new(vec![split, at, member]))
}

/// A field's value in its `u16`, or the refusal `past` names.
fn u16_of(n: usize, past: SampleWire) -> Result<[u8; 2], SampleWire> {
    u16::try_from(n).map(u16::to_le_bytes).map_err(|_| past)
}

/// ONE SAMPLE WRITTEN, or refused by name with nothing written.
pub fn push_sample(out: &mut Vec<u8>, s: &Sample) -> Result<(), SampleWire> {
    let mut framed = Vec::new();
    framed.extend_from_slice(&u16_of(s.rank(), SampleWire::RankPastField { rank: s.rank() })?);
    for (axis, extent) in s.extents().iter().copied().enumerate() {
        framed.extend_from_slice(&u16_of(extent, SampleWire::ExtentPastField { axis, extent })?);
    }
    framed.extend_from_slice(&u16_of(s.width(), SampleWire::WidthPastField { width: s.width() })?);
    let place = s.place().coordinates();
    framed.extend_from_slice(&u16_of(place.len(), SampleWire::PlacePastField { count: place.len() })?);
    for c in place {
        framed.extend_from_slice(&c.to_le_bytes());
    }
    for symbol in s.symbols() {
        framed.extend_from_slice(symbol.bytes());
    }
    out.append(&mut framed);
    Ok(())
}

/// One `u16` field read, or `Short`.
fn take_u16(b: &[u8], p: &mut usize) -> Result<usize, SampleWire> {
    crate::transducer::address::take_u16_count(b, p).ok_or(SampleWire::Short)
}

/// ONE SAMPLE READ BACK, exactly, or refused by name. Every count is
/// checked against the bytes that remain before anything is allocated, and
/// the symbol bytes `w · ∏ eᵢ` are counted with checked arithmetic.
pub fn read_sample(b: &[u8], p: &mut usize) -> Result<Sample, SampleWire> {
    let remaining = |p: usize| b.len().checked_sub(p).ok_or(SampleWire::Short);
    let rank = take_u16(b, p)?;
    // Two bytes per extent: a rank the bytes cannot hold is short.
    if rank > remaining(*p)? / 2 {
        return Err(SampleWire::Short);
    }
    let mut extents = Vec::with_capacity(rank);
    for _ in 0..rank {
        extents.push(take_u16(b, p)?);
    }
    let width = take_u16(b, p)?;
    let count = take_u16(b, p)?;
    // Eight bytes per coordinate.
    if count > remaining(*p)? / 8 {
        return Err(SampleWire::Short);
    }
    let mut place = Vec::with_capacity(count);
    for _ in 0..count {
        let bytes: [u8; 8] = b.get(*p..*p + 8).and_then(|s| s.try_into().ok()).ok_or(SampleWire::Short)?;
        place.push(i64::from_le_bytes(bytes));
        *p += 8;
    }
    let mut sites = 1usize;
    for (axis, &e) in extents.iter().enumerate() {
        sites = sites.checked_mul(e).ok_or(SampleWire::Sample(SampleRefusal::SitesOverflow { axis }))?;
    }
    let bytes = sites.checked_mul(width).ok_or(SampleWire::Sample(SampleRefusal::BytesOverflow { sites, width }))?;
    if bytes > remaining(*p)? {
        return Err(SampleWire::Short);
    }
    let symbols = b.get(*p..*p + bytes).ok_or(SampleWire::Short)?.to_vec();
    *p += bytes;
    Sample::new(extents, width, symbols, Place::new(place)).map_err(SampleWire::Sample)
}

/// [`read_sample`], refused where the place read is not the sample's
/// position.
fn read_sample_at(b: &[u8], p: &mut usize, split: Split, pair: usize, member: Member) -> Result<Sample, SampleWire> {
    let at = position(split, pair, member)?;
    let s = read_sample(b, p)?;
    match *s.place() == at {
        true => Ok(s),
        false => Err(SampleWire::PlaceOffPosition),
    }
}

/// [`push_sample`], refused where the sample's place is not its position:
/// a writer never sends a writing its reader refuses.
fn push_sample_at(out: &mut Vec<u8>, s: &Sample, split: Split, pair: usize, member: Member) -> Result<(), SampleWire> {
    match *s.place() == position(split, pair, member)? {
        true => push_sample(out, s),
        false => Err(SampleWire::PlaceOffPosition),
    }
}

/// A count behind one byte, or refused past it.
fn put_count(out: &mut Vec<u8>, count: usize) -> Result<(), SampleWire> {
    crate::transducer::address::put_u8_count(out, count).ok_or(SampleWire::CountPastField { count })
}

/// A count behind one byte read back.
fn take_count(b: &[u8], p: &mut usize) -> Result<usize, SampleWire> {
    let n = usize::from(*b.get(*p).ok_or(SampleWire::Short)?);
    *p += 1;
    Ok(n)
}

/// THE TRAIN PAIRS written: `[n u8][(input, output)·n]`, each at its
/// position in the train split.
pub(crate) fn push_train(out: &mut Vec<u8>, train: &[(Sample, Sample)]) -> Result<(), SampleWire> {
    put_count(out, train.len())?;
    for (i, (x, y)) in train.iter().enumerate() {
        push_sample_at(out, x, Split::Train, i, Member::Input)?;
        push_sample_at(out, y, Split::Train, i, Member::Output)?;
    }
    Ok(())
}

/// The train pairs read back, each at its position.
pub(crate) fn read_train(b: &[u8], p: &mut usize) -> Result<Vec<(Sample, Sample)>, SampleWire> {
    let n = take_count(b, p)?;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let x = read_sample_at(b, p, Split::Train, i, Member::Input)?;
        let y = read_sample_at(b, p, Split::Train, i, Member::Output)?;
        out.push((x, y));
    }
    Ok(out)
}

/// SAMPLES OF ONE MEMBER OF THE TEST SPLIT written: `[n u8][sample·n]`,
/// sample `i` at `[test, i, member]` — the test inputs an envelope serves,
/// or the outputs a solve speaks.
pub(crate) fn push_tests(out: &mut Vec<u8>, samples: &[Sample], member: Member) -> Result<(), SampleWire> {
    put_count(out, samples.len())?;
    for (i, s) in samples.iter().enumerate() {
        push_sample_at(out, s, Split::Test, i, member)?;
    }
    Ok(())
}

/// The samples of one member of the test split read back.
pub(crate) fn read_tests(b: &[u8], p: &mut usize, member: Member) -> Result<Vec<Sample>, SampleWire> {
    let n = take_count(b, p)?;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        out.push(read_sample_at(b, p, Split::Test, i, member)?);
    }
    Ok(out)
}

/// 78.146, TRACK 26 S5 — THE OUTPUTS AS THE SOLVE ACT AND THE SEAT'S SOLVE
/// CALL WRITE THEM: `[n u8][sample·n]`, output `i` at `[test, i, output]`.
pub fn encode_samples(outputs: &[Sample]) -> Result<Vec<u8>, SampleWire> {
    let mut out = Vec::new();
    push_tests(&mut out, outputs, Member::Output)?;
    Ok(out)
}

/// The outputs read back whole, or refused by name: nothing after the
/// last.
pub fn decode_samples(b: &[u8]) -> Result<Vec<Sample>, SampleWire> {
    let mut p = 0usize;
    let out = read_tests(b, &mut p, Member::Output)?;
    match p == b.len() {
        true => Ok(out),
        false => Err(SampleWire::Short),
    }
}

/// A section sample as the sample at its position, refused by name.
fn placed(g: &SectionSample, split: Split, pair: usize, member: Member) -> Result<Sample, SampleWire> {
    Ok(g.sample().placed(position(split, pair, member)?))
}

/// THE SPEECH END (MU-SENSING §2): the outputs a mind spoke on the
/// alphabet's section, each the sample at its place `[test, i, output]`.
pub fn spoken_outputs(outputs: &[SectionSample]) -> Result<Vec<Sample>, SampleWire> {
    outputs.iter().enumerate().map(|(i, g)| placed(g, Split::Test, i, Member::Output)).collect()
}

/// A SAMPLE ON THE ALPHABET'S SECTION (`spelled::SectionSample::of`): the
/// sample of the rank the letters spell (`alphabet::alphabet_admits`) at
/// one byte per symbol, held whole with its place. A sample of another
/// rank is refused by the alphabet's name, one of another width by its
/// width: carried exactly on the wire and perceived at its rank, never
/// truncated onto the section.
pub fn section_of(s: &Sample) -> Result<SectionSample, SectionRefusal> {
    SectionSample::of(s.clone())
}

/// A REGISTERED TASK AS THE REGISTRY HANDS IT OVER (wire v1.2; track 26
/// S5): the family to learn from and the inputs to answer, as samples of
/// any rank, never the answers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskEnvelope {
    /// The task's citation (wire v1.2) — what a solve names it by.
    pub id: [u8; 32],
    /// The TRAIN pairs: the family the rule is extracted from, each pair's
    /// input and output at `[train, i, input]` and `[train, i, output]`.
    pub train: Vec<(Sample, Sample)>,
    /// The TEST inputs, input `i` at `[test, i, input]`. The outputs for
    /// these inputs are what a solve claims; the registry never sends them.
    pub tests: Vec<Sample>,
}

/// A TASK ENVELOPE ON THE ALPHABET'S SECTION (MU-SENSING §2): the
/// envelope's samples as the section's carrier the word machinery spells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TaskRasters {
    pub id: [u8; 32],
    pub train: Vec<TrainPair>,
    pub tests: Vec<SectionSample>,
}

impl TaskEnvelope {
    /// THE ENVELOPE OF A TASK HELD ON THE SECTION: its train pairs and test
    /// inputs, each a sample at its position. The registry serves posted
    /// samples (`PostedTask::envelope`).
    pub fn of_rasters(id: [u8; 32], train: &[TrainPair], tests: &[SectionSample]) -> Result<TaskEnvelope, SampleWire> {
        let mut pairs = Vec::with_capacity(train.len());
        for (i, (x, y)) in train.iter().enumerate() {
            pairs.push((placed(x, Split::Train, i, Member::Input)?, placed(y, Split::Train, i, Member::Output)?));
        }
        let tests = tests.iter().enumerate().map(|(i, x)| placed(x, Split::Test, i, Member::Input)).collect::<Result<_, _>>()?;
        Ok(TaskEnvelope { id, train: pairs, tests })
    }

    /// THE ENVELOPE ON THE ALPHABET'S SECTION, or the refusal naming the
    /// first sample off it ([`section_of`]).
    pub fn section(&self) -> Result<TaskRasters, SectionRefusal> {
        let train = self.train.iter().map(|(x, y)| Ok((section_of(x)?, section_of(y)?))).collect::<Result<_, SectionRefusal>>()?;
        let tests = self.tests.iter().map(section_of).collect::<Result<_, SectionRefusal>>()?;
        Ok(TaskRasters { id: self.id, train, tests })
    }
}

/// A TASK AS A SOURCE POSTS IT (MU-SENSING §6, §7; track 26 S1, S5): the
/// train pairs, and the test pairs with each output withheld by the court
/// (absent where the source holds none), as samples of any rank.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostedTask {
    pub train: Vec<(Sample, Sample)>,
    pub test: Vec<(Sample, Option<Sample>)>,
}

impl PostedTask {
    /// A section task's samples, each at its position; an absent test
    /// output stays absent.
    pub fn of_rasters(task: &Task) -> Result<PostedTask, SampleWire> {
        let mut train = Vec::with_capacity(task.train.len());
        for (i, (x, y)) in task.train.iter().enumerate() {
            train.push((placed(x, Split::Train, i, Member::Input)?, placed(y, Split::Train, i, Member::Output)?));
        }
        let mut test = Vec::with_capacity(task.test.len());
        for (i, (x, y)) in task.test.iter().enumerate() {
            let output = match y {
                None => None,
                Some(y) => Some(placed(y, Split::Test, i, Member::Output)?),
            };
            test.push((placed(x, Split::Test, i, Member::Input)?, output));
        }
        Ok(PostedTask { train, test })
    }

    /// THE POSTED TASK ON THE ALPHABET'S SECTION: an absent output stays
    /// absent. Refused naming the first sample off the section
    /// ([`section_of`]).
    pub fn section(&self) -> Result<Task, SectionRefusal> {
        let train = self.train.iter().map(|(x, y)| Ok((section_of(x)?, section_of(y)?))).collect::<Result<_, SectionRefusal>>()?;
        let test = self
            .test
            .iter()
            .map(|(x, y)| {
                let output = match y {
                    Some(y) => Some(section_of(y)?),
                    None => None,
                };
                Ok((section_of(x)?, output))
            })
            .collect::<Result<_, SectionRefusal>>()?;
        Ok(Task { train, test })
    }

    /// THE SAMPLES THAT ARE THE TASK'S IDENTITY (MU-SENSING §6.1): the
    /// train pairs' inputs and outputs and the test inputs, in address
    /// order. The withheld outputs are the court's and no part of it.
    pub fn posted(&self) -> impl Iterator<Item = &Sample> + '_ {
        let train = self.train.iter().flat_map(|(x, y)| [x, y]);
        train.chain(self.test.iter().map(|(x, _)| x))
    }

    /// THE ENVELOPE THE POSTED TASK SERVES under its citation `id` (track
    /// 26 S4c): the train pairs and the test inputs as posted, at any rank
    /// and width, never a withheld output. No sample passes through the
    /// alphabet's section on the way out.
    pub fn envelope(&self, id: [u8; 32]) -> TaskEnvelope {
        TaskEnvelope { id, train: self.train.clone(), tests: self.test.iter().map(|(x, _)| x.clone()).collect() }
    }
}

/// THE POSTED TASK WRITTEN: the train pairs, then `[nx u8]` and per test
/// pair its input and `[0 | 1 u8]` with the withheld output where there is
/// one. Refused by name where a sample passes a field.
pub fn encode_posted_samples(task: &PostedTask) -> Result<Vec<u8>, SampleWire> {
    let mut out = Vec::new();
    push_train(&mut out, &task.train)?;
    put_count(&mut out, task.test.len())?;
    for (i, (x, y)) in task.test.iter().enumerate() {
        push_sample_at(&mut out, x, Split::Test, i, Member::Input)?;
        match y {
            Some(y) => {
                out.push(1);
                push_sample_at(&mut out, y, Split::Test, i, Member::Output)?;
            }
            None => out.push(0),
        }
    }
    Ok(out)
}

/// The posted task read back whole, exactly, or refused by name: nothing
/// after the last pair, and a marker other than `0` or `1` is refused.
pub fn decode_posted_samples(b: &[u8]) -> Result<PostedTask, SampleWire> {
    let mut p = 0usize;
    let train = read_train(b, &mut p)?;
    let n = take_count(b, &mut p)?;
    let mut test = Vec::with_capacity(n);
    for i in 0..n {
        let x = read_sample_at(b, &mut p, Split::Test, i, Member::Input)?;
        let marker = *b.get(p).ok_or(SampleWire::Short)?;
        p += 1;
        let y = match marker {
            0 => None,
            1 => Some(read_sample_at(b, &mut p, Split::Test, i, Member::Output)?),
            byte => return Err(SampleWire::Marker { byte }),
        };
        test.push((x, y));
    }
    match p == b.len() {
        true => Ok(PostedTask { train, test }),
        false => Err(SampleWire::Short),
    }
}
