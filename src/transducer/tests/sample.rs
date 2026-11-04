// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2, §5; ROADMAP track 26 S0 — the sample as the sensor's only
// input: rank read from data, unread dimensions unread, symbols by equality.

use crate::transducer::base::Site;
use crate::transducer::cga::MAX_N;
use crate::transducer::sample::*;

/// A sample of `extents` at `width` whose buffer counts up byte by byte, so
/// every site's symbol differs from every other's (the fixtures stay below
/// one byte's span).
fn counted(extents: &[usize], width: usize) -> Sample {
    let len = extents.iter().product::<usize>() * width;
    let symbols = (0..len).map(|b| u8::try_from(b).expect("below a byte")).collect();
    Sample::new(extents.to_vec(), width, symbols, Place::new(vec![0])).expect("the sample stands")
}

fn site(axes: &[i64]) -> Site {
    Site::new(axes.to_vec()).expect("a site of the base")
}

/// THE RANK IS THE SAMPLE'S OWN (MU-SENSING §2 (1)): samples of rank 1, 2, 3
/// and 4 each report the count of the sample's extents, and every site
/// each yields carries that many coordinates. No rank is written in the sensor.
#[test]
fn sensor_reads_rank_from_sample() {
    for extents in [vec![3], vec![2, 3], vec![2, 1, 2], vec![1, 2, 1, 3]] {
        let s = counted(&extents, 1);
        assert_eq!(s.rank(), extents.len());
        assert!(s.sites().all(|x| x.n() == extents.len()));
        assert_eq!(s.sites().count(), extents.iter().product::<usize>());
    }
}

/// AN UNREAD DIMENSION CARRIES NO VALUE (MU-SENSING §5): an axis past the
/// rank reads `Unread`, never an extent of 0 or 1, and a site of another
/// rank is refused by its rank, never padded with 0 or truncated.
#[test]
fn unread_dimension_carries_no_value() {
    let s = counted(&[2, 3], 1);
    assert_eq!(s.extent(0), AxisReading::Read(2));
    assert_eq!(s.extent(1), AxisReading::Read(3));
    assert_eq!(s.extent(2), AxisReading::Unread);
    assert_eq!(s.symbol_at(&site(&[0, 0, 0])), Err(SiteRefusal::RankMismatch { sample: 2, site: 3 }));
    assert_eq!(s.symbol_at(&site(&[0])), Err(SiteRefusal::RankMismatch { sample: 2, site: 1 }));
}

/// A SITE OUTSIDE AN EXTENT IS REFUSED with its axis, coordinate and
/// extent: past the far end, and below the near end.
#[test]
fn symbol_at_refuses_site_outside_extent() {
    let s = counted(&[2, 3], 1);
    assert_eq!(s.symbol_at(&site(&[1, 3])), Err(SiteRefusal::Outside { axis: 1, coordinate: 3, extent: 3 }));
    assert_eq!(s.symbol_at(&site(&[-1, 0])), Err(SiteRefusal::Outside { axis: 0, coordinate: -1, extent: 2 }));
}

/// SYMBOLS ARE COMPARED BY EQUALITY ONLY (MU-SENSING §3.3): equal bytes are
/// one symbol, and any difference of bytes or of width is another. That
/// `Symbol` has no order is the `compile_fail` doc test on the type.
#[test]
fn symbols_compare_by_equality_only() {
    assert_eq!(Symbol::new(&[1, 2]), Symbol::new(&[1, 2]));
    assert_ne!(Symbol::new(&[1, 2]), Symbol::new(&[2, 1]));
    assert_ne!(Symbol::new(&[1]), Symbol::new(&[1, 0]));
}

/// THE SYMBOL WIDTH IS THE SOURCE'S (MU-SENSING §6): at widths 1, 2 and 4
/// the symbols read over the sites in canonical address order are the
/// buffer, chunk by chunk, each of the source's width.
#[test]
fn symbol_width_is_the_sources() {
    for width in [1, 2, 4] {
        let symbols: Vec<u8> = (0..6 * width).map(|b| u8::try_from(b).expect("below a byte")).collect();
        let s = Sample::new(vec![2, 3], width, symbols.clone(), Place::new(vec![0])).expect("the sample stands");
        assert_eq!(s.width(), width);
        let read: Vec<Symbol<'_>> = s.sites().map(|x| s.symbol_at(&x).expect("a site of the sample")).collect();
        let chunks: Vec<Symbol<'_>> = symbols.chunks_exact(width).map(Symbol::new).collect();
        assert_eq!(read, chunks);
        assert!(read.iter().all(|y| y.width() == width));
    }
}

/// THE CANONICAL ADDRESS ORDER WALKS AXIS 0 SLOWEST: at extents `2 × 3` the
/// sites run along the last axis first.
#[test]
fn sites_walk_axis_zero_slowest() {
    let s = counted(&[2, 3], 1);
    let walked: Vec<Site> = s.sites().collect();
    let expected: Vec<Site> = [[0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2]].iter().map(|a| site(a)).collect();
    assert_eq!(walked, expected);
}

/// THE PLACE IS THE FAMILY'S COORDINATE, kept as given (MU-SENSING §2 (4)).
#[test]
fn sample_keeps_place() {
    let s = Sample::new(vec![1], 1, vec![7], Place::new(vec![3, -1])).expect("the sample stands");
    assert_eq!(s.place().coordinates(), &[3, -1]);
}

/// A sample with no extent presents no axis.
#[test]
fn sample_refuses_no_axis() {
    assert_eq!(Sample::new(Vec::new(), 1, Vec::new(), Place::new(vec![0])), Err(SampleRefusal::NoAxis));
}

/// An axis of extent 0 is refused with its axis.
#[test]
fn sample_refuses_empty_axis() {
    assert_eq!(Sample::new(vec![2, 0, 3], 1, Vec::new(), Place::new(vec![0])), Err(SampleRefusal::EmptyAxis { axis: 1 }));
}

/// A width of 0 bytes carries no symbol.
#[test]
fn sample_refuses_no_width() {
    assert_eq!(Sample::new(vec![2], 0, Vec::new(), Place::new(vec![0])), Err(SampleRefusal::NoWidth));
}

/// A buffer short or long of one symbol per site at the width is refused
/// with both lengths.
#[test]
fn sample_refuses_mismatched_length() {
    assert_eq!(
        Sample::new(vec![2, 2], 2, vec![0; 7], Place::new(vec![0])),
        Err(SampleRefusal::Length { expected: 8, found: 7 })
    );
    assert_eq!(
        Sample::new(vec![2, 2], 2, vec![0; 9], Place::new(vec![0])),
        Err(SampleRefusal::Length { expected: 8, found: 9 })
    );
}

/// Extents whose product passes the machine word are refused at the axis
/// where the count passes it: checked arithmetic, never a cap.
#[test]
fn sample_refuses_uncountable_sites() {
    assert_eq!(
        Sample::new(vec![usize::MAX, 2], 1, Vec::new(), Place::new(vec![0])),
        Err(SampleRefusal::SitesOverflow { axis: 1 })
    );
}

/// A site count whose bytes at the width pass the machine word is refused
/// with the count and the width.
#[test]
fn sample_refuses_uncountable_bytes() {
    assert_eq!(
        Sample::new(vec![usize::MAX], 2, Vec::new(), Place::new(vec![0])),
        Err(SampleRefusal::BytesOverflow { sites: usize::MAX, width: 2 })
    );
}

/// A rank the base's site carrier refuses is refused at intake, so every
/// sample admitted addresses its sites; the bound is the carrier's own.
#[test]
fn sample_refuses_rank_past_site() {
    let rank = usize::from(MAX_N) + 1;
    assert_eq!(
        Sample::new(vec![1; rank], 1, vec![0], Place::new(vec![0])),
        Err(SampleRefusal::RankPastSite { rank })
    );
}
