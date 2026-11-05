// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE THE LETTERS SPELL (MU-SENSING §3.6, §3.9, §4, §5; ROADMAP
//! track 26 S4d): a [`Sample`] admitted onto the alphabet's section, the
//! rank the seventeen letters are written on
//! (`alphabet::ALPHABET_SAMPLE_RANK`) at one byte per symbol, read through
//! its perspective. It replaced the planar raster `Vec<Vec<u8>>` the word,
//! chart and rule-language machinery read as `family::Grid` (stage 1), and
//! its readers read sites through the perspective's plane, never lines
//! (stage 2).
//!
//! - **A sample, never a raster.** The carrier holds the sample whole: its
//!   extents, its symbols in canonical address order and its place in its
//!   family. The rank is the sample's, and the alphabet admits it
//!   ([`SectionSample::of`]); a sample of another rank or a wider symbol is
//!   refused by name ([`SectionRefusal`]), never truncated onto the
//!   letters' planes.
//! - **Read through a perspective.** The sample's default perspective
//!   (`Perspective::of_sample`) names the plane the letters are read on
//!   (`Perspective::plane`). A site of the section ([`SectionSite`]) is the
//!   sample's coordinate vector restricted to that plane, in the plane's
//!   order. The section is admitted only where the plane spans every axis
//!   of the sample, so no coordinate is invented for an unread axis and
//!   none is dropped (MU-SENSING §5).
//! - **No line and no raster inside.** Readers walk the sites
//!   ([`SectionSample::sites`]), read one site ([`SectionSample::at`]) and
//!   write one site in place ([`SectionSample::paint`]). The raster is
//!   only the speech end toward a source's form ([`SectionSample::raster`])
//!   and the intake end ([`SectionSample::of_lines`]).
//! - **Content equality.** Two section samples are equal where the
//!   extents and the symbols are: the place is the sample's coordinate in its
//!   family, never part of what the letters read, so a spoken sample and
//!   the posted one it answers compare as content.

use super::alphabet::{alphabet_admits, AlphabetRank, ALPHABET_BASE_N, ALPHABET_SAMPLE_RANK};
use super::base::Site;
use super::perspective::{family_place_axis, family_plane, Perspective};
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

/// A SITE OF THE SECTION (MU-SENSING §3.6, §4): a sample's coordinate
/// vector restricted by its perspective to the plane the letters are read
/// on, one coordinate per axis of that plane in the plane's order. No
/// coordinate is a row or a column; the plane is the perspective's.
///
/// A coordinate may lie off the sample (a carried or neighbouring site is
/// read before it is known to land): such a site carries no symbol, never
/// the ground written in for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SectionSite([i64; ALPHABET_SAMPLE_RANK]);

impl SectionSite {
    /// A site of the section from its coordinates along the perspective's
    /// plane, in the plane's order.
    pub fn on_plane(coordinates: [i64; ALPHABET_SAMPLE_RANK]) -> SectionSite {
        SectionSite(coordinates)
    }

    /// The coordinates along the perspective's plane, in the plane's order.
    pub fn coordinates(&self) -> [i64; ALPHABET_SAMPLE_RANK] {
        self.0
    }

    /// The coordinates as indices of the sample's sites, or `None` where a
    /// coordinate is negative (a site off every sample).
    pub fn indices(&self) -> Option<[usize; ALPHABET_SAMPLE_RANK]> {
        let mut out = [0usize; ALPHABET_SAMPLE_RANK];
        for (slot, &c) in out.iter_mut().zip(&self.0) {
            *slot = usize::try_from(c).ok()?;
        }
        Some(out)
    }

    /// A site of the section from indices of the sample's sites; `None`
    /// where an index passes `i64`.
    pub fn of_indices(indices: [usize; ALPHABET_SAMPLE_RANK]) -> Option<SectionSite> {
        let mut out = [0i64; ALPHABET_SAMPLE_RANK];
        for (slot, &c) in out.iter_mut().zip(&indices) {
            *slot = i64::try_from(c).ok()?;
        }
        Some(SectionSite(out))
    }

    /// THE SITE MOVED BY `by` along the plane, coordinate by coordinate;
    /// `None` where a coordinate would leave `i64`.
    pub fn moved(&self, by: [i64; ALPHABET_SAMPLE_RANK]) -> Option<SectionSite> {
        let mut out = self.0;
        for (slot, d) in out.iter_mut().zip(by) {
            *slot = slot.checked_add(d)?;
        }
        Some(SectionSite(out))
    }

    /// THE SITE READ FROM `origin`: each coordinate less the origin's along
    /// the same axis of the plane; `None` where a coordinate would leave
    /// `i64`.
    pub fn offset_from(&self, origin: SectionSite) -> Option<SectionSite> {
        let mut out = self.0;
        for (slot, o) in out.iter_mut().zip(origin.0) {
            *slot = slot.checked_sub(o)?;
        }
        Some(SectionSite(out))
    }

    /// THE SPAN OF SITES ALONG THE PLANE: their least corner (the least
    /// coordinate on each axis of the plane) and, per axis, one past the
    /// farthest offset from it, the extents of the least sample that holds
    /// every site read from the corner. `None` for no site, or where an
    /// offset passes the machine word.
    pub fn span_of(sites: impl Iterator<Item = SectionSite> + Clone) -> Option<(SectionSite, [usize; ALPHABET_SAMPLE_RANK])> {
        let mut least: Option<[i64; ALPHABET_SAMPLE_RANK]> = None;
        for s in sites.clone() {
            match least.as_mut() {
                None => least = Some(s.0),
                Some(l) => {
                    for (at, c) in l.iter_mut().zip(s.0) {
                        if c < *at {
                            *at = c;
                        }
                    }
                }
            }
        }
        let corner = SectionSite(least?);
        let mut extents = [0usize; ALPHABET_SAMPLE_RANK];
        for s in sites {
            for (e, o) in extents.iter_mut().zip(s.offset_from(corner)?.indices()?) {
                let past = o.checked_add(1)?;
                if past > *e {
                    *e = past;
                }
            }
        }
        Some((corner, extents))
    }

    /// WRITE THE SITE: each coordinate along the plane, in the plane's
    /// order, eight bytes little-endian; the bytes a planar pair `(a, b)`
    /// was written in.
    pub fn put_canonical(&self, buf: &mut Vec<u8>) {
        for x in self.0 {
            buf.extend_from_slice(&x.to_le_bytes());
        }
    }

    /// READ A SITE [`put_canonical`](Self::put_canonical) wrote at `*p`,
    /// advancing `*p`; `None` where the bytes end first.
    pub fn take_canonical(b: &[u8], p: &mut usize) -> Option<SectionSite> {
        let mut out = [0i64; ALPHABET_SAMPLE_RANK];
        for slot in out.iter_mut() {
            let end = p.checked_add(std::mem::size_of::<i64>())?;
            *slot = i64::from_le_bytes(b.get(*p..end)?.try_into().ok()?);
            *p = end;
        }
        Some(SectionSite(out))
    }

    /// THE SITE ON THE FAMILY'S BASE (MU-SENSING §4): the plane's
    /// coordinates on the family site's plane and the sample's place in its
    /// family on the place axis, under the default perspective of a site of
    /// the alphabet's base (`Perspective::of_family_site`). `None` where
    /// that perspective names no plane or no place.
    pub fn on_family(&self, place: i64) -> Option<Site> {
        let (a, b) = family_plane(ALPHABET_BASE_N)?;
        let k = family_place_axis(ALPHABET_BASE_N)?;
        let mut axes = vec![0i64; ALPHABET_BASE_N];
        for (&axis, &c) in [a, b].iter().zip(&self.0) {
            *axes.get_mut(axis)? = c;
        }
        *axes.get_mut(k)? = place;
        Site::new(axes)
    }

    /// THE SITE OF THE SECTION A SITE OF THE FAMILY'S BASE STANDS AT: its
    /// coordinates on the family site's plane under the default perspective
    /// (`perspective::family_plane`); every other axis rides on the pose.
    /// `None` for a base whose perspective holds no plane.
    pub fn of_family(site: &Site) -> Option<SectionSite> {
        let (a, b) = family_plane(site.n())?;
        let mut out = [0i64; ALPHABET_SAMPLE_RANK];
        for (slot, axis) in out.iter_mut().zip([a, b]) {
            *slot = site.axis(axis)?;
        }
        Some(SectionSite(out))
    }
}

/// A PAINTED SITE OF THE SECTION (MU-SENSING §3.3, §3.6): a site of the
/// section and the symbol the sample holds there, one byte, where that
/// symbol is not the scene's ground. Ordered by the site, then the symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PaintedSite {
    /// Where the symbol stands on the section.
    pub site: SectionSite,
    /// The symbol painted there.
    pub symbol: u8,
}

impl PaintedSite {
    /// The site `site` painted `symbol`.
    pub fn new(site: SectionSite, symbol: u8) -> PaintedSite {
        PaintedSite { site, symbol }
    }

    /// The site of the section whose coordinates along the perspective's
    /// plane are `coordinates`, painted `symbol`.
    pub fn on_plane(coordinates: [i64; ALPHABET_SAMPLE_RANK], symbol: u8) -> PaintedSite {
        PaintedSite { site: SectionSite(coordinates), symbol }
    }
}

/// A SAMPLE ON THE ALPHABET'S SECTION: the rank the letters spell, one byte
/// per symbol, read through its perspective's plane.
#[derive(Clone, Debug)]
pub struct SectionSample {
    sample: Sample,
    extents: [usize; ALPHABET_SAMPLE_RANK],
    perspective: Perspective,
    /// The sample's axes the perspective's plane reads, in the plane's
    /// order; together they are every axis of the sample.
    plane: [usize; ALPHABET_SAMPLE_RANK],
}

impl PartialEq for SectionSample {
    fn eq(&self, other: &SectionSample) -> bool {
        self.extents == other.extents && self.symbols() == other.symbols()
    }
}

impl Eq for SectionSample {}

impl SectionSample {
    /// THE SAMPLE ON THE SECTION, or the refusal that names why it is off
    /// it: a rank the letters are not written on, a symbol wider than one
    /// byte, or a perspective whose plane does not span the sample's axes.
    pub fn of(sample: Sample) -> Result<SectionSample, SectionRefusal> {
        alphabet_admits(sample.rank()).map_err(SectionRefusal::Rank)?;
        if sample.width() != 1 {
            return Err(SectionRefusal::Width { width: sample.width() });
        }
        let off = || SectionRefusal::Rank(AlphabetRank { rank: sample.rank() });
        let extents = <[usize; ALPHABET_SAMPLE_RANK]>::try_from(sample.extents()).map_err(|_| off())?;
        let perspective = Perspective::of_sample(sample.rank());
        let (a, b) = perspective.plane().ok_or_else(off)?;
        let plane = <[usize; ALPHABET_SAMPLE_RANK]>::try_from([a, b].as_slice()).map_err(|_| off())?;
        // The plane spans every axis: each axis of the sample once.
        let mut spanned = [false; ALPHABET_SAMPLE_RANK];
        for &axis in &plane {
            match spanned.get_mut(axis) {
                Some(slot) if !*slot => *slot = true,
                _ => return Err(off()),
            }
        }
        Ok(SectionSample { sample, extents, perspective, plane })
    }

    /// A SECTION SAMPLE OF `extents` (one per axis of the plane, in the
    /// plane's order) WHOSE EVERY SITE HOLDS `symbol`: the canvas a writer
    /// lays before painting it, at an empty place (one the family has not
    /// written). Refused by the sample's name where an extent is zero or the
    /// sites overflow the machine word.
    pub fn filled(extents: [usize; ALPHABET_SAMPLE_RANK], symbol: u8) -> Result<SectionSample, Unspelled> {
        let mut sites = 1usize;
        for (axis, &e) in extents.iter().enumerate() {
            sites = sites.checked_mul(e).ok_or(Unspelled::Sample(SampleRefusal::SitesOverflow { axis }))?;
        }
        let sample = Sample::new(extents.to_vec(), 1, vec![symbol; sites], Place::new(Vec::new())).map_err(Unspelled::Sample)?;
        SectionSample::of(sample).map_err(Unspelled::Section)
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

    /// The perspective the section is read through.
    pub fn perspective(&self) -> &Perspective {
        &self.perspective
    }

    /// THE EXTENTS ALONG THE PERSPECTIVE'S PLANE, in the plane's order: the
    /// extents every reader of a site of the section reads its coordinates
    /// against.
    pub fn extents(&self) -> [usize; ALPHABET_SAMPLE_RANK] {
        let mut out = [0usize; ALPHABET_SAMPLE_RANK];
        for (slot, &axis) in out.iter_mut().zip(&self.plane) {
            // Every axis of the plane is an axis of the sample (`of`).
            if let Some(&e) = self.extents.get(axis) {
                *slot = e;
            }
        }
        out
    }

    /// Every symbol in canonical address order, one byte each.
    pub fn symbols(&self) -> &[u8] {
        self.sample.symbol_bytes()
    }

    /// THE OFFSET OF A SITE OF THE SECTION in canonical address order, or
    /// `None` where a coordinate leaves its axis: each coordinate of the
    /// plane is written onto the sample axis the plane reads it on.
    pub(crate) fn offset_of(&self, site: SectionSite) -> Option<usize> {
        let mut canonical = [0i64; ALPHABET_SAMPLE_RANK];
        for (&axis, &c) in self.plane.iter().zip(&site.0) {
            *canonical.get_mut(axis)? = c;
        }
        let mut offset = 0usize;
        for (&c, &e) in canonical.iter().zip(&self.extents) {
            let c = usize::try_from(c).ok().filter(|&c| c < e)?;
            offset = offset.checked_mul(e)?.checked_add(c)?;
        }
        Some(offset)
    }

    /// THE SECTION'S SITE AT AN OFFSET in canonical address order: the
    /// sample's coordinate vector, axis 0 slowest, restricted to the plane.
    /// Every index lies below its extent, below the buffer's length, so it
    /// stands in `i64` and every offset of the buffer yields its site.
    fn site_at(&self, mut offset: usize) -> Option<SectionSite> {
        let mut canonical = [0usize; ALPHABET_SAMPLE_RANK];
        for (slot, &e) in canonical.iter_mut().zip(&self.extents).rev() {
            *slot = offset % e;
            offset /= e;
        }
        let mut out = [0i64; ALPHABET_SAMPLE_RANK];
        for (slot, &axis) in out.iter_mut().zip(&self.plane) {
            *slot = i64::try_from(*canonical.get(axis)?).ok()?;
        }
        Some(SectionSite(out))
    }

    /// EVERY SITE OF THE SECTION WITH ITS SYMBOL, in canonical address
    /// order: the sensor's one walk of the sample.
    pub fn sites(&self) -> impl Iterator<Item = (SectionSite, u8)> + '_ {
        self.symbols().iter().enumerate().map_while(|(offset, &h)| Some((self.site_at(offset)?, h)))
    }

    /// THE SYMBOL AT `site`, or `None` where a coordinate leaves its axis:
    /// a site off the sample carries no symbol, never the ground written in
    /// for it.
    pub fn at(&self, site: SectionSite) -> Option<u8> {
        self.symbols().get(self.offset_of(site)?).copied()
    }

    /// WRITE `symbol` AT `site`, in place; `None` (nothing written) where a
    /// coordinate leaves its axis.
    pub fn paint(&mut self, site: SectionSite, symbol: u8) -> Option<()> {
        let offset = self.offset_of(site)?;
        *self.sample.symbol_bytes_mut().get_mut(offset)? = symbol;
        Some(())
    }

    /// THE SECTION SAMPLE AS THE RASTER A SOURCE'S FORM WRITES (MU-SENSING
    /// §2): the speech end of the bridge, the runs along the last axis in
    /// canonical address order. No reader of the section reads it.
    pub fn raster(&self) -> Vec<Vec<u8>> {
        self.symbols().chunks(self.extents[ALPHABET_SAMPLE_RANK - 1]).map(<[u8]>::to_vec).collect()
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
        self.raster() == *lines
    }
}

#[cfg(test)]
impl SectionSample {
    /// A FIXTURE'S SAMPLE WITH EVERY SYMBOL MAPPED by `f`, written in place
    /// at the same sites.
    pub(crate) fn mapped(&self, f: impl Fn(u8) -> u8) -> SectionSample {
        let mut out = self.clone();
        for h in out.sample.symbol_bytes_mut() {
            *h = f(*h);
        }
        out
    }

    /// A FIXTURE'S WRITE of `symbol` at `coordinates` along the plane; a
    /// site off the sample is a defect of the fixture.
    pub(crate) fn set(&mut self, coordinates: [i64; ALPHABET_SAMPLE_RANK], symbol: u8) {
        self.paint(SectionSite::on_plane(coordinates), symbol).expect("a fixture writes a site of the sample");
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
