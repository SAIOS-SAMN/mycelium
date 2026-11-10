use super::super::*;
use super::fixtures::*;
use crate::court::admission::{Boundary, UniverseAct};
use crate::physics::laws::clauses::{AlignmentRefusal, ClauseReading, ConcordanceClauseId};
use crate::physics::laws::Law;

/// THE BOUNDARY READING TRAVELS WHOLE (MU-HOSTING §5a.7, B4): a row of each
/// name, the clause and the reserve's reading of it, the vetoed clause, the
/// stroke spent and the laws a fold broke, written as a query reply and read
/// back as themselves; and the console names each row's domain whole, its
/// act, its name and what decided it.
#[test]
fn boundary_reading_reads_back_each_name() {
    let clause = ConcordanceClauseId::of(forms::coordinate::LAW_ID, forms::coordinate::LAW_VERSION);
    let row = |last, times| TubeRow { last, height: 70, times };
    let reading = BoundaryReading {
        rows: vec![
            row(AtTube::Refused(Boundary::NotThroughCore { act: UniverseAct::Map, domain: party(1) }), 3),
            row(AtTube::Refused(Boundary::NotSignatory { act: UniverseAct::Fold, domain: party(1), clause: clause.clone(), reading: Box::new(ClauseReading::Unread) }), 1),
            row(AtTube::Refused(Boundary::Bounded { act: UniverseAct::Anchor, domain: party(2), clause: clause.clone() }), 2),
            row(AtTube::Refused(Boundary::Unaligned { act: UniverseAct::Statement, domain: party(2), why: AlignmentRefusal::StrokeSpent }), 1),
            row(AtTube::Refused(Boundary::Crossing { act: UniverseAct::Mouth, domain: party(3), why: crate::anastome::dialogue::crossing_record::CrossingRefusal::Absent }), 5),
            row(AtTube::NotLawful { domain: party(3), laws: vec![Law::Momentum, Law::Time] }, 4),
        ],
    };
    assert_eq!(reading.rows.iter().map(|r| r.last.name()).collect::<Vec<_>>(), TUBE_REFUSALS, "one row of each name, at its code");
    reads_back(&[Reading::Boundary(Box::new(reading.clone()))]);
    let lines = reading.lines();
    assert!(lines[0].contains("NotThroughCore 3 · NotSignatory 1 · Bounded 2 · Unaligned 1 · Crossing 5 · NotLawful 4"), "counted per name: {}", lines[0]);
    assert!(lines.iter().any(|l| l.contains(&"03".repeat(32)) && l.contains("NotLawful ×4") && l.contains("broke momentum, temporal monotonicity")), "the laws broken, named");
    assert!(lines.iter().any(|l| l.contains("vetoed FORMS-COORDINATE")), "the vetoed clause, named");
    assert!(lines.iter().any(|l| l.contains("Crossing ×5") && l.contains("crossing Absent")), "the crossing refusal, named");
}
