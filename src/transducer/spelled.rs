// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE THE LETTERS SPELL (MU-SENSING §3.9, §5; ROADMAP track 26
//! S4d): a [`Sample`] admitted onto the alphabet's section, the rank the
//! seventeen letters are written on (`alphabet::ALPHABET_SAMPLE_RANK`) at
//! one byte per symbol. It replaces the planar raster `Vec<Vec<u8>>` the
//! word, chart and rule-language machinery read as `family::Grid`.
//!
//! - **A sample, never a raster.** The carrier holds the sample whole: its
//!   extents, its symbols in canonical address order and its place in its
//!   family. The rank is the sample's, and the alphabet admits it
//!   ([`SectionSample::of`]); a sample of another rank or a wider symbol is
//!   refused by name ([`SectionRefusal`]), never truncated onto the
//!   letters' planes.
//! - **A line** is the run of sites along the sample's last axis at one
//!   coordinate of every other axis: contiguous in canonical address order,
//!   so it is read in place. The axes are ordered canonically, and the
//!   order is never a role (MU-SENSING §3.6).
//! - **Content equality.** Two section samples are equal where the
//!   extents and the symbols are: the place is the sample's coordinate in its
//!   family, never part of what the letters read, so a spoken sample and
//!   the posted one it answers compare as content.

use super::alphabet::{alphabet_admits, ALPHABET_SAMPLE_RANK};
use super::sample::{Place, Sample, SampleRefusal};
use super::sentence::SectionRefusal;

/// WHY LINES ARE NO SAMPLE THE LETTERS SPELL, named: no sample at all
/// (an empty or ragged raster), or a sample off the alphabet's section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unspelled {
    /// The lines are no sample.
    Sample(SampleRefusal),
    /// The sample is off the alphabet's section.
    Section(SectionRefusal),
}

/// A SAMPLE ON THE ALPHABET'S SECTION: the rank the letters spell, one byte
/// per symbol, read by lines along its last axis.
#[derive(Clone, Debug)]
pub struct SectionSample {
    sample: Sample,
    extents: [usize; ALPHABET_SAMPLE_RANK],
}

impl PartialEq for SectionSample {
    fn eq(&self, other: &SectionSample) -> bool {
        self.extents == other.extents && self.symbols() == other.symbols()
    }
}

impl Eq for SectionSample {}

impl SectionSample {
    /// THE SAMPLE ON THE SECTION, or the refusal that names why it is off
    /// it: a rank the letters are not written on, or a symbol wider than
    /// one byte.
    pub fn of(sample: Sample) -> Result<SectionSample, SectionRefusal> {
        alphabet_admits(sample.rank()).map_err(SectionRefusal::Rank)?;
        if sample.width() != 1 {
            return Err(SectionRefusal::Width { width: sample.width() });
        }
        let extents = <[usize; ALPHABET_SAMPLE_RANK]>::try_from(sample.extents()).map_err(|_| SectionRefusal::Rank(super::alphabet::AlphabetRank { rank: sample.rank() }))?;
        Ok(SectionSample { sample, extents })
    }

    /// A SECTION SAMPLE OF `extents` WHOSE EVERY SITE HOLDS `symbol`: the
    /// canvas a writer lays before painting it, at an empty place (one the
    /// family has not written). Refused by the sample's name where an
    /// extent is zero or the sites overflow the machine word.
    pub fn filled(extents: [usize; ALPHABET_SAMPLE_RANK], symbol: u8) -> Result<SectionSample, SampleRefusal> {
        let mut sites = 1usize;
        for (axis, &e) in extents.iter().enumerate() {
            sites = sites.checked_mul(e).ok_or(SampleRefusal::SitesOverflow { axis })?;
        }
        let sample = Sample::new(extents.to_vec(), 1, vec![symbol; sites], Place::new(Vec::new()))?;
        Ok(SectionSample { sample, extents })
    }

    /// THIS SAMPLE'S EXTENTS WITH EVERY SITE HOLDING `symbol`: the canvas a
    /// writer lays on the frame it read, at the same place.
    pub fn cleared(&self, symbol: u8) -> SectionSample {
        let mut out = self.clone();
        out.sample.symbol_bytes_mut().fill(symbol);
        out
    }

    /// THE RASTER AT INTAKE AS A SECTION SAMPLE (MU-SENSING §2): its lines
    /// read through the raster bridge (`raster::sample_of`) at an empty
    /// place. A ragged or empty raster is refused by the sample's name.
    pub fn of_lines(lines: &[Vec<u8>]) -> Result<SectionSample, Unspelled> {
        let sample = super::raster::sample_unplaced(lines).map_err(Unspelled::Sample)?;
        SectionSample::of(sample).map_err(Unspelled::Section)
    }

    /// The sample whole, with its place.
    pub fn sample(&self) -> &Sample {
        &self.sample
    }

    /// The sample whole, given up.
    pub fn into_sample(self) -> Sample {
        self.sample
    }

    /// The sample's extents, one per axis, in canonical axis order.
    pub fn extents(&self) -> [usize; ALPHABET_SAMPLE_RANK] {
        self.extents
    }

    /// The count of sites in one line: the extent of the last axis.
    fn line_extent(&self) -> usize {
        self.extents[ALPHABET_SAMPLE_RANK - 1]
    }

    /// Every symbol in canonical address order, one byte each.
    pub fn symbols(&self) -> &[u8] {
        self.sample.symbol_bytes()
    }

    /// THE LINES, in canonical address order: each the run of sites along
    /// the last axis.
    pub fn lines(&self) -> std::slice::Chunks<'_, u8> {
        self.symbols().chunks(self.line_extent())
    }

    /// The line at `coordinate` of the first axis, or `None` past it.
    pub fn line(&self, coordinate: usize) -> Option<&[u8]> {
        let w = self.line_extent();
        self.symbols().get(coordinate.checked_mul(w)?..coordinate.checked_add(1)?.checked_mul(w)?)
    }

    /// The line at `coordinate` of the first axis, for a writer that paints
    /// it in place, or `None` past it.
    pub fn line_mut(&mut self, coordinate: usize) -> Option<&mut [u8]> {
        let w = self.line_extent();
        self.sample.symbol_bytes_mut().get_mut(coordinate.checked_mul(w)?..coordinate.checked_add(1)?.checked_mul(w)?)
    }

    /// THE SYMBOL AT `site` (one coordinate per axis), or `None` where a
    /// coordinate leaves its axis: a site off the sample carries no symbol,
    /// never the ground written in for it.
    pub fn at(&self, site: [i64; ALPHABET_SAMPLE_RANK]) -> Option<u8> {
        let mut offset = 0usize;
        for (&c, &e) in site.iter().zip(&self.extents) {
            let c = usize::try_from(c).ok().filter(|&c| c < e)?;
            offset = offset * e + c;
        }
        self.symbols().get(offset).copied()
    }

    /// THE SECTION SAMPLE AS THE RASTER A SOURCE'S FORM WRITES (MU-SENSING
    /// §2): the speech end of the bridge, line by line.
    pub fn raster(&self) -> Vec<Vec<u8>> {
        self.lines().map(<[u8]>::to_vec).collect()
    }
}

// ── TEST FIXTURES (MU-SENSING §7): a fixture's lines are read at intake
// through the raster bridge, as a source's decoder reads its own. A fixture
// whose lines are no sample on the section is a defect of the fixture.

#[cfg(test)]
impl From<Vec<Vec<u8>>> for SectionSample {
    fn from(lines: Vec<Vec<u8>>) -> SectionSample {
        SectionSample::of_lines(&lines).expect("a fixture's lines are a sample on the section")
    }
}

#[cfg(test)]
impl FromIterator<Vec<u8>> for SectionSample {
    fn from_iter<I: IntoIterator<Item = Vec<u8>>>(lines: I) -> SectionSample {
        SectionSample::from(lines.into_iter().collect::<Vec<Vec<u8>>>())
    }
}

#[cfg(test)]
impl PartialEq<Vec<Vec<u8>>> for SectionSample {
    fn eq(&self, lines: &Vec<Vec<u8>>) -> bool {
        self.lines().eq(lines.iter().map(Vec::as_slice))
    }
}

/// A FIXTURE'S PAIR OF LINES, as a source writes it before intake.
#[cfg(test)]
pub(crate) type LinePair = (Vec<Vec<u8>>, Vec<Vec<u8>>);

/// A FIXTURE'S FAMILY of line pairs, each pair read at intake.
#[cfg(test)]
pub(crate) fn pairs_of(pairs: Vec<LinePair>) -> Vec<(SectionSample, SectionSample)> {
    pairs.into_iter().map(|(x, y)| (SectionSample::from(x), SectionSample::from(y))).collect()
}

impl std::ops::Index<usize> for SectionSample {
    type Output = [u8];

    /// The line at `coordinate` of the first axis; a coordinate past it is
    /// a defect of the reader, as a slice's index is.
    fn index(&self, coordinate: usize) -> &[u8] {
        let w = self.line_extent();
        &self.symbols()[coordinate * w..(coordinate + 1) * w]
    }
}

impl std::ops::IndexMut<usize> for SectionSample {
    fn index_mut(&mut self, coordinate: usize) -> &mut [u8] {
        let w = self.line_extent();
        &mut self.sample.symbol_bytes_mut()[coordinate * w..(coordinate + 1) * w]
    }
}
