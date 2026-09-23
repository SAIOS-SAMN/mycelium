// 45.9 — PHASE IS A READING: the four coordinates are read off the receipt
// kinds and the cover at the window's ends, and nothing stores them.

use super::common::speak_pair;
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
/// installs that cite a lineage; installed the words entering the cover
/// against those retired, a split re-homing rather than installing (I2);
/// adjoined-by-closure the generators a closure opened, read beside it and
/// never from it (I1); expansion the axes
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
        c.append(kind, [0; 4], [1u8], [9; 32], Vec::new()).expect("appends");
    }
    let mut peer = ReceiptChain::with_vk([2; 32]);
    peer.append(ActKind::List, [0; 4], [7u8], [9; 32], Vec::new()).expect("appends");
    let cited = peer.receipts()[0].clone();
    c.admit(cited.clone());
    c.append(
        ActKind::Install,
        [0; 4],
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
    assert_eq!((p.installed, p.retired), (3, 1), "installed: [1] +1, the split +1, the axis +1; [4] retired");
    assert_eq!(p.adjoined_by_closure, 0, "the window reads installs; adjunction is the court's, read at the court");
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
    assert_eq!(
        p.line(),
        format!(
            "  court phase h 0..{h1} comprehension 2/3 congruence 1/4 lineages 1 installed +3 -1 \
             adjoined-by-closure 0 expansion 1/1"
        )
    );
}

/// The court writes the phase once per docket circuit and keeps only the
/// cover reading the next window starts from: the chain length and the
/// strata, never a phase.
#[test]
fn the_court_reads_the_phase_once_per_circuit_and_stores_no_phase() {
    let mut k = Kernel::new();
    assert!(k.phase_mark.is_none(), "nothing before the first circuit");
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    k.read_phase();
    let (k0, at_h0) = k.phase_mark.clone().expect("the window's start is kept");
    assert_eq!(k0, k.chain().receipts().len(), "the next window starts at the chain's head");
    assert_eq!(at_h0.strata.len(), k.cover_reading().strata.len());
    let c = vec![(0, 0, 3u8), (0, 1, 3)];
    let d = vec![(0, 0, 4u8), (0, 1, 4)];
    assert_eq!(speak_pair(&mut k, &c, &d, [2; 32]), Outcome::GermPending);
    k.read_phase();
    let (k1, _) = k.phase_mark.clone().expect("kept again");
    assert!(k1 > k0, "the window moved with the chain");
    let _: Keyed<usize> = at_h0.strata;

    // A READING OVER AN EMPTY WINDOW IS NOT A READING, and `read_phase` says
    // so. Nothing has been admitted since the call above, so the window is
    // `k1..k1` and `phase_over` would report zero out of zero over `h N..N` —
    // the vacuous assertion at the instrument. Hypha a wrote 25 598 of those
    // in nine minutes at e98e8a2 because its caller paces on docket circuits
    // and a docket that fits one budget completes one every stroke.
    assert!(!k.read_phase(), "no receipt admitted, so nothing is measured");
    let (k2, _) = k.phase_mark.clone().expect("and the mark still advances");
    assert_eq!(k2, k1, "an empty window leaves the start where the chain is");
    // And a reading IS taken as soon as the chain moves again.
    let e = vec![(0, 0, 5u8), (0, 1, 5)];
    let f = vec![(0, 0, 6u8), (0, 1, 6)];
    assert_eq!(speak_pair(&mut k, &e, &f, [3; 32]), Outcome::GermPending);
    assert!(k.read_phase(), "a receipt in the window is a measurement");
    // PH5 — THE WINDOW WRITTEN IS HANDED TO THE OBSERVER, never kept by the
    // court: the observer's copy reads this window's heights (78.17). The
    // store is the process's, so another court's window may have landed in
    // between; the heights are this chain's own.
    let held = crate::observe::last_phase().expect("a window written is held for the observer");
    assert!(held.windows >= 1, "the windows are counted");
    assert!(held.h1 >= held.h0, "a window's heights run forward");
}

/// R7 (45.9): the window is an aperture of docket circuits. A docket that
/// fits one budget completes a circuit per stroke, and read per circuit
/// its window was one receipt or none; the court stroke takes the reading
/// when the circuit count crosses a multiple of `aperture_cycle()`.
#[test]
fn the_phase_window_is_an_aperture_of_docket_circuits() {
    use super::common::one_stroke_transport;
    let mut k = Kernel::new();
    let w = crate::transducer::element::from_connection(
        &crate::transducer::alphabet::Word::identity(),
        &crate::transducer::element::translator_versor(2, 0),
        &[],
        &Vec::new(),
        [1, 0, 0, 0],
    )
    .expect("spells");
    assert!(matches!(k.install_adopted(&w, [1; 32]), Adoption::Installed));
    let aperture = u64::from(crate::physics::aperture_cycle());
    let mut strokes = 0u64;
    loop {
        strokes += 1;
        let bound = aperture
            .checked_mul(2)
            .and_then(|n| n.checked_add(2))
            .expect("two apertures fit the clock this test runs on");
        assert!(strokes <= bound, "a one-listing docket circuits every stroke: {:?}", k.docket_circuits());
        k.court_stroke(one_stroke_transport);
        let c = k.docket_circuits().circuits();
        if c < aperture {
            assert!(k.phase_mark.is_none(), "no reading below the aperture: circuits {c} of {aperture}");
        } else {
            assert!(k.phase_mark.is_some(), "read when the circuit count crosses the aperture: circuits {c}");
            break;
        }
    }
}
