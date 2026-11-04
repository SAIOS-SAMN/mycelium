// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE LINK AT RANK `n` (MU-SENSING §2, §3.9; ROADMAP track 26 S4b): a
//! change at one site of a sample of any rank, with the two symbols it
//! carries there at the source's own width. The residue of a factorisation
//! is a set of links; a court derives one and compares it with a claim's
//! (`transduce::factor`).
//!
//! The word, chart and field carriers spell chains on the alphabet's
//! section (`SectionLink`), the sample of the rank the seventeen letters are
//! written on. A link enters them only through [`Link::section`], which
//! refuses another rank or a wider symbol by name: the alphabet is the
//! only thing that limits it there (MU-ALPHABET §2).

use super::SectionLink;
use crate::transducer::alphabet::{alphabet_admits, AlphabetRank};
use crate::transducer::base::Site;

/// A LINK: a site of rank `n` (its coordinates in canonical axis order) and
/// the symbols before and after the change, each at the source's width,
/// compared by equality alone. No coordinate is a row or a column, and none
/// is privileged: the rank is the site's.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Link {
    /// Where the change stands, of the sample's rank.
    pub site: Site,
    /// The symbol before the change, at the source's width.
    pub before: Vec<u8>,
    /// The symbol after the change, at the source's width.
    pub after: Vec<u8>,
}

/// WHY A LINK HAS NO LINK ON THE ALPHABET'S SECTION, named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SectionRefusal {
    /// The site's rank is not the one the letters spell.
    Rank(AlphabetRank),
    /// A symbol wider than the section's one byte.
    Width { width: usize },
}

impl Link {
    /// A link at `site` carrying `before` to `after`.
    pub fn new(site: Site, before: Vec<u8>, after: Vec<u8>) -> Link {
        Link { site, before, after }
    }

    /// The link's rank: its site's count of coordinates.
    pub fn rank(&self) -> usize {
        self.site.n()
    }

    /// A SITE CHANGE OF A TRANSDUCTION AS A LINK, at the change's own rank
    /// and width.
    pub fn of_change(c: &crate::transducer::family::transduce::SiteChange) -> Link {
        Link { site: c.site().clone(), before: c.before().bytes().to_vec(), after: c.after().bytes().to_vec() }
    }

    /// A LINK OF THE ALPHABET'S SECTION READ AS A LINK: its two coordinates
    /// as a site of rank 2, its two bytes as symbols of width 1.
    pub fn of_section(&(a, b, before, after): &SectionLink) -> Link {
        Link { site: Site::of([a, b]), before: vec![before], after: vec![after] }
    }

    /// THE LINK ON THE ALPHABET'S SECTION, or the refusal that names why it
    /// has none: a rank the letters do not spell ([`alphabet_admits`]), or
    /// a symbol wider than one byte. Never truncated onto the section.
    pub fn section(&self) -> Result<SectionLink, SectionRefusal> {
        alphabet_admits(self.rank()).map_err(SectionRefusal::Rank)?;
        let byte = |s: &[u8]| match s {
            &[b] => Ok(b),
            _ => Err(SectionRefusal::Width { width: s.len() }),
        };
        match self.site.axes() {
            &[a, b] => Ok((a, b, byte(&self.before)?, byte(&self.after)?)),
            _ => Err(SectionRefusal::Rank(AlphabetRank { rank: self.rank() })),
        }
    }

    /// THE LINK WRITTEN OUT (A21): its site (`n` first), then each symbol
    /// by its own count and bytes, so the writing is injective across ranks
    /// and widths.
    pub fn put(&self, out: &mut Vec<u8>) {
        use crate::transducer::address::put_key_bytes;
        self.site.put(out);
        put_key_bytes(out, &self.before);
        put_key_bytes(out, &self.after);
    }

    /// The inverse of [`Link::put`]: `None` where the bytes do not hold a
    /// site and its two symbols whole.
    pub fn take(b: &[u8], p: &mut usize) -> Option<Link> {
        use crate::transducer::address::take_key_bytes;
        let site = Site::take(b, p)?;
        let before = take_key_bytes(b, p)?;
        let after = take_key_bytes(b, p)?;
        Some(Link { site, before, after })
    }
}
