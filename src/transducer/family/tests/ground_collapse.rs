// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// RULED 2026-11-06 ON #32 (3) — A GROUND'S VORTEX COLLAPSES BY A RELATION
// (MU-SENSING §3.4, §3.8): a candidate collapses it where a body stands on
// it on every train input (`Stands`), or every pair's residue changes only
// its sites (`PaintedOn`), and that reading holds for no other candidate.
// A closing rule never collapses it.

use super::fixtures::*;
use crate::transducer::family::transduce::{Candidacy, Collapse, Ground, GroundReading};
use crate::transducer::family::*;
use crate::transducer::sample::Sample;

/// The family's pairs as samples.
fn samples(family: &[(SectionSample, SectionSample)]) -> Vec<(Sample, Sample)> {
    family.iter().map(|(x, y)| (x.sample().clone(), y.sample().clone())).collect()
}

/// The symbols every condition of §3.4 keeps, in the family's order.
fn candidates(family: &[(SectionSample, SectionSample)]) -> Vec<Vec<u8>> {
    Ground::candidacies(&samples(family)).into_iter().filter(|(_, c)| *c == Candidacy::Candidate).map(|(s, _)| s).collect()
}

/// A frame of 7 holds an interior of 0 on every input; a dot stands on the
/// 0 in the first input and is recoloured, and nothing stands on the 0 in
/// the second, which is carried as it is. Both are candidates (the
/// interior holds a body in some input); a body stands on 7 on every input
/// (its interior), on 0 on one input alone, and no residue changes only one
/// candidate's sites, so 7 collapses the vortex by `Stands`.
#[test]
fn ground_vortex_collapses_where_bodies_stand_on_one_candidate() {
    let family = vec![
        (digits(&["77777", "70007", "70107", "70007", "77777"]), digits(&["77777", "70007", "70307", "70007", "77777"])),
        (digits(&["7777", "7007", "7007", "7777"]), digits(&["7777", "7007", "7007", "7777"])),
    ];
    assert_eq!(candidates(&family), vec![vec![7], vec![0]], "both are candidates of the vortex");
    let (read, collapse) = Ground::collapse(&samples(&family));
    assert_eq!(read[0].readings, vec![GroundReading::Stands], "a body stands on 7 on every input");
    assert_eq!(read[1].readings, Vec::new(), "on 0 on one input alone, and the recolouring changes neither's sites");
    assert_eq!(collapse, Collapse::Collapsed { candidate: 0, by: GroundReading::Stands });
    assert_eq!(crate::transducer::raster::ground_of(&family), Ground::One(7), "the vortex collapsed to 7");
}

/// A frame of 8 holds an interior of 0 on which a dot moves: a body stands
/// on both (the 0 inside the 8, the dot inside the 0), so `Stands` holds
/// alike; the residue changes only the 0's sites (the dot leaves 0 and
/// lands on 0), so 0 collapses the vortex by `PaintedOn`.
#[test]
fn ground_vortex_collapses_where_residue_paints_one_candidate() {
    let family = vec![(
        digits(&["888888", "800008", "801008", "800008", "800008", "888888"]),
        digits(&["888888", "800008", "800108", "800008", "800008", "888888"]),
    )];
    assert_eq!(candidates(&family), vec![vec![8], vec![0]], "both are candidates of the vortex");
    let (read, collapse) = Ground::collapse(&samples(&family));
    assert_eq!(read[0].readings, vec![GroundReading::Stands]);
    assert_eq!(read[1].readings, vec![GroundReading::Stands, GroundReading::PaintedOn]);
    assert_eq!(collapse, Collapse::Collapsed { candidate: 1, by: GroundReading::PaintedOn });
    assert_eq!(crate::transducer::raster::ground_of(&family), Ground::One(0), "the vortex collapsed to 0");
}

/// A canvas of two halves, 0 and 5: a dot moves on the 0 half in one pair
/// and on the 5 half in the other. No body stands on either half (neither
/// covers a seam), and no residue changes only one candidate's sites, so
/// the vortex stays superposed, whatever a rule closes under each
/// candidate: the closure is an observation per candidate, read here, and
/// never a collapse.
#[test]
fn closing_rule_never_collapses_the_ground() {
    let family = vec![
        (digits(&["00000555", "01000555", "00000555"]), digits(&["00000555", "00100555", "00000555"])),
        (digits(&["00000555", "00000525", "00000555"]), digits(&["00000555", "00000552", "00000555"])),
    ];
    assert_eq!(candidates(&family), vec![vec![0], vec![5]]);
    let (read, collapse) = Ground::collapse(&samples(&family));
    assert!(read.iter().all(|r| r.readings.is_empty()), "no relation holds for either: {read:?}");
    assert_eq!(collapse, Collapse::Superposed);
    assert_eq!(crate::transducer::raster::ground_of(&family), Ground::Vortex(vec![0, 5]), "the candidates stay superposed");
    // The observation per candidate: whether the family's rule, read and
    // minted under it, closes the family. Whatever it reads, the ground is
    // unchanged.
    let closes: Vec<bool> = [0u8, 5]
        .iter()
        .map(|&g| {
            crate::transducer::raster::on_ground(Ground::One(g), || {
                let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
                extract_rule(&family, &mut budget).is_some_and(|rule| mint_sentence(&rule, &family, &NoCrystal, 0).is_some())
            })
        })
        .collect();
    assert_eq!(crate::transducer::raster::ground_of(&family), Ground::Vortex(vec![0, 5]), "observed closures {closes:?} collapse nothing");
}
