// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE SAMPLE (MU-SENSING §2; ROADMAP track 26, S0): the sensor's only
//! input, of any rank. Its rank, extents, symbols and place are all data.
//!
//! A sample is a raster only here, at intake, and again where speech writes
//! an answer back into a source's form (MU-SENSING §2; MU-RECURSION 76.1;
//! MU-ANTI-FLATTENING clause 12). Between those two ends nothing holds it as
//! a raster: the lift reads it into phases and fibre values (MU-SENSING §3).
//!
//! - **Rank** is the count of extents, read from the source and never
//!   fixed in the code (MU-RECURSION 71).
//! - **Symbols** are one byte buffer at the source's own width, one symbol
//!   per site, compared by equality only (MU-SENSING §3.3). No symbol is
//!   read as a number, an order, a ground or a fibre meaning here.
//! - **Canonical address order**: sites are laid out with axis 0 slowest
//!   and the last axis fastest. The order serves addressing only and is
//!   never a role (MU-SENSING §3.6): which axis is space, sequence or value
//!   is a perspective's (MU-SENSING §4).
//! - **Unread dimensions stay unread** (MU-SENSING §5): an axis past the
//!   rank reads [`AxisReading::Unread`], never 0, and a site of another rank
//!   is refused, never padded.

use super::base::Site;

/// THE SAMPLE'S PLACE IN ITS FAMILY (MU-SENSING §2 (4)): a coordinate the
/// family carries, of whatever count the family gives it. It is never a
/// fixed axis index, and it names no role: a source's words for a place
/// (a pair's two members, a step, a view) end in the source's decoder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Place(Vec<i64>);

impl Place {
    /// A place from the family's coordinates for it.
    pub fn new(coordinates: Vec<i64>) -> Place {
        Place(coordinates)
    }

    /// The family's coordinates for the place.
    pub fn coordinates(&self) -> &[i64] {
        &self.0
    }
}

/// ONE SYMBOL OF A SAMPLE: the bytes of one site at the source's width.
/// A label whose meaning the sensor deduces (MU-SENSING §3.3), so two
/// symbols are compared by equality only: the type has no order and no
/// numeric reading.
///
/// ```compile_fail,E0369
/// use mycelium::transducer::sample::Symbol;
/// let (a, b) = (Symbol::new(&[1]), Symbol::new(&[2]));
/// let _ = a < b;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol<'a>(&'a [u8]);

impl<'a> Symbol<'a> {
    /// A symbol from its bytes, as a source writes it.
    pub fn new(bytes: &'a [u8]) -> Symbol<'a> {
        Symbol(bytes)
    }

    /// The symbol's width in bytes: the source's own.
    pub fn width(&self) -> usize {
        self.0.len()
    }

    /// The symbol's bytes, for the two raster ends alone: speech writing a
    /// sample back, and content written out as an address (A21). Inside
    /// the crate only; no reader takes them as a number.
    pub(crate) fn bytes(&self) -> &'a [u8] {
        self.0
    }
}

/// AN AXIS READ OR UNREAD (MU-SENSING §5): an axis of the sample reads its
/// extent; an axis past the sample's rank carries no value at all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisReading {
    /// The extent the source presents along the axis.
    Read(usize),
    /// An axis the sample does not present: unread, never 0.
    Unread,
}

/// WHY A SAMPLE IS REFUSED AT INTAKE, named with its cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleRefusal {
    /// No extent at all: a sample presents at least one axis.
    NoAxis,
    /// An axis of extent 0 presents no site.
    EmptyAxis { axis: usize },
    /// A symbol width of 0 bytes carries no symbol.
    NoWidth,
    /// The symbol buffer does not hold one symbol per site at the width.
    Length { expected: usize, found: usize },
    /// The product of the extents up to `axis` passes the machine word: the
    /// sites cannot be counted.
    SitesOverflow { axis: usize },
    /// The site count times the width passes the machine word.
    BytesOverflow { sites: usize, width: usize },
    /// A rank the base's site carrier cannot hold ([`Site::new`] refuses
    /// it), so no site of the sample could be addressed.
    RankPastSite { rank: usize },
}

/// WHY A SITE IS REFUSED BY A SAMPLE, named with its cause.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiteRefusal {
    /// The site's rank is not the sample's: a site of another rank is a
    /// different kind of thing, never padded or truncated.
    RankMismatch { sample: usize, site: usize },
    /// The site's coordinate on `axis` lies outside that axis's extent.
    Outside { axis: usize, coordinate: i64, extent: usize },
}

/// A SAMPLE (MU-SENSING §2): its extents (the rank is the extents' count), its
/// symbols at the source's width in canonical address order, and its place
/// in its family.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sample {
    extents: Vec<usize>,
    width: usize,
    symbols: Vec<u8>,
    place: Place,
}

impl Sample {
    /// A SAMPLE FROM A DECODER'S OUTPUT, or the refusal that names why it
    /// is none. Every count is read with checked arithmetic; no bound is
    /// written here beyond the machine word and the site carrier's own.
    pub fn new(extents: Vec<usize>, width: usize, symbols: Vec<u8>, place: Place) -> Result<Sample, SampleRefusal> {
        if extents.is_empty() {
            return Err(SampleRefusal::NoAxis);
        }
        if let Some(axis) = extents.iter().position(|&e| e == 0) {
            return Err(SampleRefusal::EmptyAxis { axis });
        }
        if width == 0 {
            return Err(SampleRefusal::NoWidth);
        }
        let mut sites = 1usize;
        for (axis, &e) in extents.iter().enumerate() {
            sites = sites.checked_mul(e).ok_or(SampleRefusal::SitesOverflow { axis })?;
        }
        let expected = sites.checked_mul(width).ok_or(SampleRefusal::BytesOverflow { sites, width })?;
        if symbols.len() != expected {
            return Err(SampleRefusal::Length { expected, found: symbols.len() });
        }
        let rank = extents.len();
        if Site::new(vec![0; rank]).is_none() {
            return Err(SampleRefusal::RankPastSite { rank });
        }
        Ok(Sample { extents, width, symbols, place })
    }

    /// The sample's rank: the count of its extents, read from the data.
    pub fn rank(&self) -> usize {
        self.extents.len()
    }

    /// The extent along `axis`, or [`AxisReading::Unread`] past the rank.
    pub fn extent(&self, axis: usize) -> AxisReading {
        match self.extents.get(axis) {
            Some(&e) => AxisReading::Read(e),
            None => AxisReading::Unread,
        }
    }

    /// The source's symbol width in bytes.
    pub fn width(&self) -> usize {
        self.width
    }

    /// The sample's place in its family.
    pub fn place(&self) -> &Place {
        &self.place
    }

    /// THE SYMBOL AT `site`: refused `RankMismatch` where the site's rank is
    /// not the sample's, and `Outside` where a coordinate leaves its axis.
    /// The site's offset is read in canonical address order (axis 0
    /// slowest); each coordinate lies below its extent, so the offset lies
    /// below the site count the constructor counted.
    pub fn symbol_at(&self, site: &Site) -> Result<Symbol<'_>, SiteRefusal> {
        if site.n() != self.rank() {
            return Err(SiteRefusal::RankMismatch { sample: self.rank(), site: site.n() });
        }
        let mut offset = 0usize;
        for (axis, (&coordinate, &extent)) in site.axes().iter().zip(&self.extents).enumerate() {
            let c = usize::try_from(coordinate)
                .ok()
                .filter(|&c| c < extent)
                .ok_or(SiteRefusal::Outside { axis, coordinate, extent })?;
            offset = offset * extent + c;
        }
        let from = offset * self.width;
        Ok(Symbol(&self.symbols[from..from + self.width]))
    }

    /// THE SAMPLE'S SYMBOLS in canonical address order, axis 0 slowest: one
    /// per site, the order of [`Sample::sites`]. The sensor's one read of
    /// the raster.
    pub fn symbols(&self) -> impl Iterator<Item = Symbol<'_>> + '_ {
        self.symbols.chunks(self.width).map(Symbol)
    }

    /// THE SAMPLE'S FULL CONTENT WRITTEN OUT (A21), as the source gave it:
    /// the rank, each extent, the symbol width, the place's coordinates and
    /// the symbols in canonical address order, each written by its own
    /// coordinate (`address::put_key_count`, `put_key_bytes`), so the
    /// writing is injective. Nothing is read into it: no ground, no frame,
    /// no meaning (MU-SENSING §6).
    pub fn put_content(&self, out: &mut Vec<u8>) {
        use crate::transducer::address::{put_key_bytes, put_key_count};
        put_key_count(out, self.rank());
        for &e in &self.extents {
            put_key_count(out, e);
        }
        put_key_count(out, self.width);
        put_key_count(out, self.place.coordinates().len());
        for c in self.place.coordinates() {
            put_key_bytes(out, &c.to_le_bytes());
        }
        put_key_bytes(out, &self.symbols);
    }

    /// THE SAMPLE'S SITES in canonical address order, axis 0 slowest. Every
    /// coordinate is below its extent, which is below the buffer's length,
    /// so it stands in `i64`; and the constructor admitted only a rank the
    /// site carrier holds, so every address yields its site.
    pub fn sites(&self) -> impl Iterator<Item = Site> + '_ {
        let count = self.symbols.len() / self.width;
        (0..count).map_while(move |index| {
            let mut rest = index;
            let mut axes = vec![0i64; self.rank()];
            for (slot, &extent) in axes.iter_mut().zip(&self.extents).rev() {
                *slot = i64::try_from(rest % extent).ok()?;
                rest /= extent;
            }
            Site::new(axes)
        })
    }
}
