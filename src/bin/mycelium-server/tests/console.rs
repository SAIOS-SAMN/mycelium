// §8.1 — the console renders each reading whole: every declared plane, the
// flattened census named, every non-zero loss by its axis.

use mycelium::observe::{Loss, Planes, Reading, Source};
use mycelium::transducer::cga;

fn rendered(readings: Vec<Reading>) -> String {
    let src = Source::Voice([7; 32]);
    let held: Vec<(Vec<u8>, Reading)> = readings.into_iter().map(|r| (src.key(r.tag()), r)).collect();
    crate::console::render("test", held.len() as u32, &held).join("\n")
}

/// A census whose every turn stands on the grid plane is named FLATTENED,
/// and every plane its algebra declares is printed with its count.
#[test]
fn a_census_turning_only_on_the_grid_plane_is_named_flattened() {
    let grid = (cga::E1 | cga::E2) as u64;
    // PC1: nine translations are one parabolic plane, `e1∞`, and do not
    // count as turns — the census still reads FLATTENED.
    let out = rendered(vec![Reading::Planes(Planes {
        n: 3,
        read: 18,
        coupled: 0,
        counts: vec![(grid, 9)],
        parabolic: vec![(1, cga::Pole::Infinity.as_u8(), 9)],
    })]);
    assert!(out.contains("FLATTENED"), "{out}");
    assert!(out.contains("e1∞ 9") && out.contains("parabolic 1/6"), "{out}");
    for p in cga::bivector_planes(3) {
        assert!(out.contains(&cga::plane_name(p)), "every declared plane is shown: {}", cga::plane_name(p));
    }
    let two = rendered(vec![Reading::Planes(Planes {
        n: 3,
        read: 9,
        coupled: 0,
        counts: vec![(grid, 5), ((cga::E1 | cga::E3) as u64, 4)],
        parabolic: Vec::new(),
    })]);
    assert!(!two.contains("FLATTENED") && two.contains("turns on 2 of 6"), "{two}");
}

/// CV5 — the console names each signer's admissions and each refusal by
/// its kind's own name.
#[test]
fn replication_and_adjudication_are_read_by_signer_and_by_kind() {
    use mycelium::observe::{rep, Adjudication, Replication, REP_AXES};
    let mut axes = [0u64; REP_AXES];
    axes[rep::BACKFILL] = 12;
    let out = rendered(vec![
        Reading::Replication(Replication { signers: vec![([9; 32], axes)], unattributed: [0; REP_AXES] }),
        Reading::Adjudication(Adjudication {
            ratified: 4,
            stood: 7,
            awaiting: 0,
            deferred: 1,
            refused: vec![(0, 2)],
            obstructed: [0, 0, 1, 1, 0, 0],
        }),
    ]);
    assert!(out.contains("admitted 12") && out.contains("backfill 12"), "{out}");
    let vertex = mycelium::transducer::sentence::RefusalKind::VertexBody.label();
    assert!(out.contains("ratified 4") && out.contains(&format!("{vertex} 2")), "{out}");
}

/// Every non-zero axis of loss is named with its meaning; the zero ones are
/// named too, on one line, so their absence is read.
#[test]
fn every_axis_of_loss_is_read_by_name() {
    let mut l = Loss::default();
    l.axes[6] = 3;
    let out = rendered(vec![Reading::Loss(l)]);
    assert!(out.contains("tau_refused 3"), "{out}");
    assert!(out.contains("zero:") && out.contains("unheard"), "{out}");
}

/// ACT1, 78.21(4) — EVERY SOURCE ON TUNIVERSAL FIRST: a court by its head's
/// height and a voice by its court's height as last heard, each with its
/// own `K`, the spread in heights, all before any other reading; and the
/// court's three counts named under it.
#[test]
fn every_source_stands_on_tuniversal_before_any_reading() {
    use mycelium::observe::{Clock, CourtClock, VoiceClock};
    let court = Clock::Court(CourtClock {
        k: 13,
        head: [40, 1, 0, 2, 0, 12],
        unframed: 2,
        unstamped: [1, 0, 0, 0, 0],
        rulings: [3, 1],
        ..CourtClock::default()
    });
    let voice = Clock::Voice(VoiceClock { k: 7, heard: 35, persisted: Some((30, 5)) });
    let held = vec![
        (Source::Court([1; 32]).key(Reading::Clock(court.clone()).tag()), Reading::Clock(court)),
        (Source::Voice([2; 32]).key(Reading::Loss(Loss::default()).tag()), Reading::Loss(Loss::default())),
        (Source::Voice([2; 32]).key(Reading::Clock(voice.clone()).tag()), Reading::Clock(voice)),
    ];
    let out = crate::console::render("test", held.len() as u32, &held).join("\n");
    let header = out.find("TUNIVERSAL").expect("the header is there");
    assert!(header < out.find("── COURT").expect("the court's block"), "TUniversal first: {out}");
    assert!(out.contains("h 40") && out.contains("K 13") && out.contains("stamp 40:1:0:2:0:12"), "{out}");
    assert!(out.contains("h 35") && out.contains("K 7"), "{out}");
    assert!(out.contains("h 35..40  (5 heights)"), "the spread, in heights only: {out}");
    assert!(out.contains("2 acts sealed on a word nothing measured"), "{out}");
    assert!(out.contains("basis 1") && out.contains("rulings admitted 3  applied 1"), "{out}");
    assert!(out.contains("at h 30 K 5  (5 heights, 2 K since)"), "the genome's persist, by height and K: {out}");
}
