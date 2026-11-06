// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! PAST THE ALPHABET'S SECTION, READINGS GO THROUGH SECTION SAMPLES (ruled
//! 2026-11-06 on #32 (4); MU-SENSING §5, MU-RECURSION 71.2 (3)). The
//! seventeen letters are written on one rank (`alphabet::LETTER_RANK`, a
//! sample of `ALPHABET_SAMPLE_RANK`), and no letter is written by hand for a
//! plane a larger rank opens: such a plane receives a letter only by the
//! presentation's growth (a replayed generator, germination, a court
//! admitting a letter for a prime, 37.7), or not at all. A sample of a rank
//! past the section is never truncated onto the letters' planes; the word
//! machinery reads it through its section samples, one per plane of two of
//! its axes and per coordinate of the rest ([`sections_of`]), each a sample
//! the alphabet admits.
//!
//! A reading, computed when asked: the sample handed in is not rewritten,
//! and each section names the plane it lies in and where it stands on the
//! other axes.

use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
use crate::transducer::base::Site;
use crate::transducer::sample::{AxisReading, Sample};
use crate::transducer::sentence::SectionRefusal;
use crate::transducer::spelled::SectionSample;

/// ONE SECTION OF A SAMPLE: the plane of two of its axes it lies in, in
/// canonical order, the coordinate it stands at on every other axis (axis,
/// coordinate), and the section sample itself, at the sample's place.
#[derive(Clone, Debug)]
pub struct Section {
    pub plane: [usize; ALPHABET_SAMPLE_RANK],
    pub at: Vec<(usize, usize)>,
    pub sample: SectionSample,
}

/// EVERY SECTION OF `sample` on the alphabet's section: for each plane of two
/// of its axes, in canonical order, and each coordinate of the remaining
/// axes, in canonical address order, the slice through it read as a section
/// sample. A sample of the section's own rank is its one section. Refused
/// by the section's own name where the sample has fewer axes than the
/// section or a symbol wider than its byte; a sample is never padded.
pub fn sections_of(sample: &Sample) -> Result<Vec<Section>, SectionRefusal> {
    let n = sample.rank();
    if n < ALPHABET_SAMPLE_RANK {
        return Err(SectionRefusal::Rank(crate::transducer::alphabet::AlphabetRank { rank: n }));
    }
    if sample.width() != 1 {
        return Err(SectionRefusal::Width { width: sample.width() });
    }
    let extents: Vec<usize> = (0..n)
        .map(|axis| match sample.extent(axis) {
            AxisReading::Read(e) => e,
            AxisReading::Unread => 0,
        })
        .collect();
    let mut out = Vec::new();
    for a in 0..n {
        for b in a + 1..n {
            let rest: Vec<usize> = (0..n).filter(|&k| k != a && k != b).collect();
            for at in coordinates(&rest, &extents) {
                // The sites standing at `at`, walked in the sample's own
                // canonical order: with `a < b` that is the slice's order
                // too, its last axis fastest.
                let on = |site: &Site| at.iter().all(|&(k, c)| site.axis(k).and_then(|v| usize::try_from(v).ok()) == Some(c));
                let symbols: Vec<u8> = sample.sites().zip(sample.symbols()).filter(|(s, _)| on(s)).flat_map(|(_, y)| y.bytes().to_vec()).collect();
                // A slice of a sample is a sample: its extents are two of the
                // sample's own and its symbols one per site of them.
                let Ok(slice) = Sample::new(vec![extents[a], extents[b]], 1, symbols, sample.place().clone()) else { continue };
                out.push(Section { plane: [a, b], at, sample: SectionSample::of(slice)? });
            }
        }
    }
    Ok(out)
}

/// Every coordinate of the axes `rest` within their extents, in canonical
/// address order (the last axis fastest); one empty coordinate where `rest`
/// is empty.
fn coordinates(rest: &[usize], extents: &[usize]) -> Vec<Vec<(usize, usize)>> {
    let mut out: Vec<Vec<(usize, usize)>> = vec![Vec::new()];
    for &k in rest {
        let e = extents.get(k).copied().unwrap_or(0);
        out = out.into_iter().flat_map(|prefix| (0..e).map(move |c| [prefix.as_slice(), &[(k, c)]].concat())).collect();
    }
    out
}
