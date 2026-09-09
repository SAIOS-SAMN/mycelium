// 45.9 — PHASE IS A READING: the four coordinates are read off the receipt
// kinds and the cover at the window's ends, and nothing stores them.

use crate::chronometry::{ActKind, ReceiptChain};
use crate::cognition::court::phase::{phase_over, CoverReading, Phase};
use crate::cognition::*;
use crate::network::nerve::Keyed;

fn reading(strata: &[(&[u8], usize)], sections: &[(&[u8], usize)]) -> CoverReading {
    let mut r = CoverReading::default();
    for (k, n) in strata {
        r.strata.insert(k, *n);
    }
    for (k, n) in sections {
        r.sections.insert(k, *n);
    }
    r
}

/// Comprehension counts the settlements against the listings; congruence
/// the charts with more than one section against the strata, and the
/// installs that cite a lineage; growth the words adjoined against those
/// retired, a split re-homing rather than adjoining; expansion the axes
/// opened and the strata split. Two readers of the same receipts and
/// covers read the same phase (13.7).
#[test]
fn the_phase_is_read_off_the_kinds_and_the_cover_at_the_windows_ends() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    for kind in [
        ActKind::List,
        ActKind::List,
        ActKind::Settled,
        ActKind::List,
        ActKind::Install,
        ActKind::Settled,
        ActKind::Ruling,
    ] {
        c.append(kind, [1u8], [9; 32], Vec::new()).expect("appends");
    }
    let mut peer = ReceiptChain::with_vk([2; 32]);
    peer.append(ActKind::List, [7u8], [9; 32], Vec::new()).expect("appends");
    let cited = peer.receipts()[0].clone();
    c.admit(cited.clone());
    c.append(
        ActKind::Install,
        [2u8],
        [9; 32],
        vec![crate::chronometry::Parent { vk: [2; 32], k: 0, hash: cited.chain_hash }],
    )
    .expect("an install reached through a lineage");
    let window = c.receipts();
    let h0 = 0;
    let h1 = c.head_height();

    let at_h0 = reading(&[(&[1], 2), (&[2], 1), (&[4], 1)], &[(&[1], 1), (&[2], 1)]);
    let at_h1 = reading(
        &[(&[1], 3), (&[2, 0], 1), (&[2, 1], 1), (&[3], 1)],
        &[(&[1], 2), (&[3], 1)],
    );
    let p = phase_over(window, &at_h0, &at_h1, h0, h1);
    assert_eq!((p.h0, p.h1), (0, h1));
    assert_eq!((p.deflated, p.held), (2, 3), "comprehension: settlements against listings");
    assert_eq!((p.sections, p.strata), (1, 4), "congruence: charts with two sections, against strata");
    assert_eq!(p.lineages, 1, "one install cites an independent lineage; the other cites none");
    assert_eq!((p.adjoined, p.retired), (3, 1), "growth: [1] +1, the split +1, the axis +1; [4] retired");
    assert_eq!((p.axes, p.splits), (1, 1), "expansion: [3] opened, [2] split into [2,0] and [2,1]");
    assert_eq!(
        phase_over(window, &at_h0, &at_h1, h0, h1),
        p,
        "the same receipts and covers read the same phase"
    );
    assert_eq!(
        phase_over(&[], &at_h1, &at_h1, h1, h1),
        Phase { h0: h1, h1, strata: 4, sections: 1, ..Phase::default() },
        "a still window reads only what stands"
    );
    assert_eq!(p.line(), format!("  court phase h 0..{h1} comprehension 2/3 congruence 1/4 lineages 1 growth +3 -1 expansion 1/1"));
}

/// The court writes the phase once per docket orbit and keeps only the
/// cover reading the next window starts from: the chain length and the
/// strata, never a phase.
#[test]
fn the_court_reads_the_phase_once_per_orbit_and_stores_no_phase() {
    let mut k = Kernel::new();
    assert!(k.phase_mark.is_none(), "nothing before the first orbit");
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
    k.read_phase();
    let (k0, at_h0) = k.phase_mark.clone().expect("the window's start is kept");
    assert_eq!(k0, k.chain().receipts().len(), "the next window starts at the chain's head");
    assert_eq!(at_h0.strata.len(), k.cover_reading().strata.len());
    let c = vec![(0, 0, 3u8), (0, 1, 3)];
    let d = vec![(0, 0, 4u8), (0, 1, 4)];
    assert_eq!(k.ingest(&c, &d, [2; 32]), Outcome::GermPending);
    k.read_phase();
    let (k1, _) = k.phase_mark.clone().expect("kept again");
    assert!(k1 > k0, "the window moved with the chain");
    let _: Keyed<usize> = at_h0.strata;
}
