// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 12.6 — THE MAGNITUDE A FAMILY PRESENTS: the genesis bond is the lane-wise
// sum over the family's disjoint pairs, read at the samples' own rank
// (ruled 2026-11-06 on #32, track 26 S4).

use crate::transducer::family::magnitude::{family_cover, family_cover_of, pair_record, record_lanes, section_cover};
use crate::transducer::family::transduce::{Ground, TransductionRefusal};
use crate::transducer::sample::{Place, Sample};
use crate::transducer::spelled::SectionSample;

/// A sample of rank `extents.len()` and width 1, its symbols in canonical
/// address order.
fn sample(extents: Vec<usize>, symbols: Vec<u8>) -> Sample {
    Sample::new(extents, 1, symbols, Place::new(Vec::new())).expect("a sample")
}

/// 12.6: a bundle's pressure is the lane-wise SUM over its pairs, which
/// are disjoint scenes — never a maximum. A maximum would report this
/// two-pair family as no larger than its wider pair, so a bond measured
/// from it could not cover the family it was measured from.
#[test]
fn family_cover_sums_pairs_lane_wise() {
    type Pair = (SectionSample, SectionSample);
    let small: Pair = (vec![vec![1, 1]].into(), vec![vec![2, 2]].into());
    let wide: Pair = (vec![vec![1, 1, 1]].into(), vec![vec![2, 2, 2]].into());
    let lanes = |p: &Pair| record_lanes(&pair_record(p.0.sample(), p.1.sample()).expect("one enclosure"), &Ground::None).expect("measured");
    let cover = crate::transducer::raster::on_ground(Ground::None, || section_cover(&[small.clone(), wide.clone()])).and_then(|c| c.bond()).expect("measured");
    let (a, b) = (lanes(&small), lanes(&wide));
    for l in 0..4 {
        assert_eq!(cover[l], a[l] + b[l], "lane {l} sums the disjoint scenes");
        assert!(cover[l] >= a[l].max(b[l]), "and therefore covers each pair");
    }
    assert!(family_cover(&[]).and_then(|c| c.bond()).is_none(), "an empty family is unmeasured, not a zero bond");
}

/// RULED 2026-11-06 ON #32 (1, 3) — THE BOND IS READ AT RANK n: S and G from
/// the projected record, H from the boundary by grade `n − 1` and nothing
/// else. On a 2×2×2 volume on ground 0, one site painted is one spatial
/// count and a shell of one component and 6 free faces; one corner of a
/// painted pair recoloured is one gauge count and changes no support; no
/// chain length is added to H.
#[test]
fn magnitude_reads_counts_at_rank_n_and_h_by_grade_n_minus_one() {
    let ground = Ground::One(vec![0]);
    let empty = sample(vec![2, 2, 2], vec![0; 8]);
    let one = sample(vec![2, 2, 2], vec![1, 0, 0, 0, 0, 0, 0, 0]);
    let appear = record_lanes(&pair_record(&empty, &one).expect("one enclosure"), &ground).expect("measured");
    assert_eq!(appear, [1, 0, 0, 1 + 6], "S 1; H one component and 2n = 6 free faces");
    let pair = sample(vec![2, 2, 2], vec![1, 1, 0, 0, 0, 0, 0, 0]);
    let recoloured = sample(vec![2, 2, 2], vec![2, 1, 0, 0, 0, 0, 0, 0]);
    let gauge = record_lanes(&pair_record(&pair, &recoloured).expect("one enclosure"), &ground).expect("measured");
    assert_eq!(gauge, [0, 1, 0, 0], "G 1; the painted support is unchanged, so H is 0, never the chain's length");
    let cover = family_cover_of([(&empty, &one), (&pair, &recoloured)], &ground).and_then(|c| c.bond());
    assert_eq!(cover, Some([1, 1, 0, 7]), "the two pairs summed lane by lane");
}

/// RULED 2026-11-06 ON #32 (2), CONFIRMED — A SCALED PAIR IS MEASURED, A
/// REFUSED PAIR IS RESIDUE: 1×2 onto 2×4 re-periods each phase by 2 and
/// its projection lands, which is the reading that resolves the unequal
/// enclosure, so it is measured (two sites of 2 gone to ground: S 2, and H
/// the vanished support's one component and 6 free faces); 1×2 onto 1×3
/// does not land and is held as residue, contributing nothing.
#[test]
fn scaled_pair_is_measured_refused_pair_is_residue() {
    let ground = Ground::One(vec![0]);
    let (x, scaled) = (sample(vec![1, 2], vec![1, 2]), sample(vec![2, 4], vec![1, 1, 2, 2, 1, 1, 0, 0]));
    let off = sample(vec![1, 3], vec![1, 1, 2]);
    let measured = record_lanes(&pair_record(&x, &scaled).expect("an integer scaling lands"), &ground).expect("measured");
    assert_eq!(measured, [2, 0, 0, 7], "the scaled pair is measured on its projected record");
    let cover = family_cover_of([(&x, &scaled), (&x, &off)], &ground).expect("no lane overflows");
    assert_eq!(cover.bond(), Some(measured), "the scaled pair alone is the bond");
    assert_eq!(cover.residue().len(), 1, "the refused pair is held");
    assert_eq!(cover.residue()[0].pair, 1);
    assert!(matches!(cover.residue()[0].refusal, TransductionRefusal::OffSite { .. }));
}

/// RULED 2026-11-06 ON #32 (2) — A PAIR WHOSE RE-PERIOD DOES NOT LAND IS
/// HELD AS RESIDUE: 1×2 onto 1×3 lands the input's middle boundary between
/// two output sites, so the record refuses `OffSite` naming the turn and the
/// period; the pair is held by its index with that refusal, contributes
/// nothing, and the family's other pair is still measured. Two ranks are
/// held the same way. No pair is aligned at a corner and none is dropped.
#[test]
fn pair_of_unequal_enclosure_is_held_as_residue_citing_off_site() {
    let ground = Ground::One(vec![0]);
    let (two, three) = (sample(vec![1, 2], vec![1, 2]), sample(vec![1, 3], vec![1, 1, 2]));
    let (still, moved) = (sample(vec![1, 2], vec![0, 0]), sample(vec![1, 2], vec![0, 1]));
    let line = sample(vec![2], vec![0, 1]);
    let cover = family_cover_of([(&two, &three), (&still, &moved), (&still, &line)], &ground).expect("no lane overflows");
    let residue = cover.residue();
    assert_eq!(residue.len(), 2, "both unread pairs are held");
    assert_eq!(residue[0].pair, 0, "cited by the pair");
    assert!(matches!(residue[0].refusal, TransductionRefusal::OffSite { period: 3, .. }), "and where the re-period does not land: {:?}", residue[0].refusal);
    assert_eq!((residue[1].pair, residue[1].refusal), (2, TransductionRefusal::Rank { from: 2, to: 1 }));
    let alone = record_lanes(&pair_record(&still, &moved).expect("one enclosure"), &ground).expect("measured");
    assert_eq!(cover.bond(), Some(alone), "the residue contributes nothing; the measured pair alone is the bond");
}
