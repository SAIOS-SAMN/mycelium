// MU-SENSING §3.1, §3.7; MU-ANTI-FLATTENING 12 — a section's chart read
// from its sites as turns of the section's own periods, with no conformal
// lift, reading every chart the lift read.

use crate::transducer::blade;
use crate::transducer::blade::harmonics::Harmonics;
use crate::transducer::spelled::SectionSite;

fn sites(at: &[[i64; 2]]) -> Vec<SectionSite> {
    at.iter().map(|&c| SectionSite::on_plane(c)).collect()
}

/// THE CHART READS A SECTION AS TURNS: every section of a `3 × 4` window
/// (each of its 4095 non-empty supports), a ring, a bar along each axis and
/// a section standing at negative coordinates lift by enclosure into turns
/// of the section's own periods, each turn landing on a site of its period;
/// and the chart read from those turns is, axis for axis, the chart the
/// conformal lift read (`Harmonics::of` on `shape_of_section`), so no
/// chart address moves. A section translated reads the same turns and the
/// same chart.
#[test]
fn chart_reads_sections_as_turns() {
    let mut cases: Vec<Vec<SectionSite>> = (1u32..(1 << 12))
        .map(|bits| {
            let at: Vec<[i64; 2]> = (0..12i64).filter(|k| bits & (1 << k) != 0).map(|k| [k / 4, k % 4]).collect();
            sites(&at)
        })
        .collect();
    cases.push(sites(&[[0, 0], [0, 1], [0, 2], [1, 0], [1, 2], [2, 0], [2, 1], [2, 2]]));
    cases.push(sites(&[[0, 0], [0, 1], [0, 2], [0, 3], [0, 4], [0, 5], [0, 6]]));
    cases.push(sites(&[[0, 0], [1, 0], [2, 0], [3, 0], [4, 0]]));
    cases.push(sites(&[[-9, -4], [-9, -3], [-8, -3], [-7, -3], [-7, -2]]));
    for section in &cases {
        let read = blade::SectionTurns::of(section).expect("a section with sites");
        for turns in read.turns() {
            assert_eq!(turns.len(), read.enclosure().rank(), "one turn per phase");
            for (t, &p) in turns.iter().zip(read.enclosure().periods()) {
                assert!(t.lands(p).is_ok_and(|i| i < p), "{t:?} lands on a site of period {p}");
            }
        }
        let lifted = Harmonics::of(&blade::shape_of_section(section).blade).expect("a section's topology").axes();
        assert_eq!(blade::section_moduli(section), lifted, "{section:?}");
        let moved: Vec<SectionSite> = section.iter().map(|s| s.moved([-7, 11]).expect("in range")).collect();
        let shifted = blade::SectionTurns::of(&moved).expect("a section with sites");
        assert_eq!(shifted.turns(), read.turns(), "the turns are the section's own");
        assert_eq!(blade::section_moduli(&moved), lifted, "{section:?} moved");
    }
    assert_eq!(blade::section_moduli(&[]), vec![crate::transducer::stalk::ChartAxis::Count(0)], "no site, no base");
}
