// 74, 78.9, CV1 — every refusal carries its kind: P, D, C by content through
// one reader, I for integrity, N for no question, unclassified a defect.

use crate::transducer::cga;
use crate::transducer::sentence::{read_obstruction, ClosureObstruction, Content, ObstructionKind};
use crate::verdict::*;

fn q(n: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(n.into())
}

/// THE ONE READER, BY CONTENT: no versor is P with the refusing reader's
/// proof; a versor the dialect's peel does not spell — the turn by the
/// Gaussian prime `2 + i` — is D with its coset; a versor it spells whose
/// closure still refused is C; a residue carried is C with its cocycle.
#[test]
fn read_obstruction_reads_kind_off_content() {
    let none = read_obstruction(Content::Holonomy { versor: None, chart: None, proof: "no versor" });
    assert_eq!(none, ClosureObstruction::Permanent { proof: "no versor" });

    let prime = cga::rotator(&q(2), &q(1));
    let chart = [7u8, 7];
    let d = read_obstruction(Content::Holonomy { versor: Some(&prime), chart: Some(&chart), proof: "unused" });
    let ClosureObstruction::Dialect { coset, chart: Some(c) } = &d else { panic!("an unspelled turn is D: {d:?}") };
    assert!(!coset.is_empty(), "the coset is the versor's ray");
    assert_eq!(c.as_slice(), &chart, "and the chart it acts on");

    let spelled = cga::rotator(&q(0), &q(1));
    let c = read_obstruction(Content::Holonomy { versor: Some(&spelled), chart: None, proof: "unused" });
    assert_eq!(c.kind(), ObstructionKind::Cover, "a spelled versor whose closure refused is cover-relative");

    let links = [(1, 2, 0, 3)];
    assert_eq!(read_obstruction(Content::Residue(&links)), ClosureObstruction::Cover { cocycle: links.to_vec() });
}

/// THE RULED ARMS (78.9): the integrity codes are I; a cited word not ruled
/// is D held by the court; a class that does not survive restriction is C;
/// NOT_NATURAL's other half — a spoken period that does not re-derive — is
/// its own code and I. A closure code reaching the table without its
/// content is UNCLASSIFIED, a defect count and never a home.
#[test]
fn ruled_obstruction_carries_ruled_kind_per_code() {
    for code in [
        REFUSE_FORM_TAPE,
        REFUSE_SIGNATURE,
        REFUSE_ROOM,
        REFUSE_FOLD_INEXACT,
        REFUSE_ORBIT_STILL,
        REFUSE_DERIVATION,
        REFUSE_ORPHAN_PORT,
        REFUSE_PERIOD_UNDERIVED,
        REFUSE_MV_INEXACT,
    ] {
        assert_eq!(ruled_obstruction(code).kind(), ObstructionKind::Integrity, "{code}");
    }
    assert_eq!(ruled_obstruction(REFUSE_GLUED_UNRULED).kind(), ObstructionKind::Dialect);
    assert_eq!(ruled_obstruction(REFUSE_NOT_NATURAL).kind(), ObstructionKind::Cover);
    for code in [REFUSE_ORBIT, REFUSE_NOT_CLOSED, REFUSE_VERTEX_BODY, REFUSE_EDGE] {
        assert_eq!(ruled_obstruction(code).kind(), ObstructionKind::Unclassified, "{code} is read by content");
    }
    assert_ne!(REFUSE_NOT_NATURAL, REFUSE_PERIOD_UNDERIVED, "NOT_NATURAL is split");
}

/// THE MINT'S AND THE POSE'S ARMS (78.9): a class in `H¹` and a coboundary
/// residue are C; acyclic and no trajectory are N; mirrored, incongruent,
/// isotropic and irrational poses are P on the refraction, oblique D,
/// bodiless I, and an unoriented pair — a held stabilizer — no refusal.
#[test]
fn obstruction_kind_classifies_withheld_and_pose_arms() {
    use crate::transducer::blade::NoPose;
    use crate::transducer::family::Withheld;
    let chord = crate::transducer::sentence::tests::common::some_overlap();
    assert_eq!(Withheld::OrbitObstruction { chord, key: [0, 0] }.obstruction_kind(), ObstructionKind::Cover);
    assert_eq!(Withheld::CoboundaryResidue { links: 1, pairs: 1 }.obstruction_kind(), ObstructionKind::Cover);
    assert_eq!(Withheld::Acyclic { links: 0, pairs: 0 }.obstruction_kind(), ObstructionKind::NoQuestion);
    assert_eq!(Withheld::NoTrajectory.obstruction_kind(), ObstructionKind::NoQuestion);
    use crate::transducer::blade::{Harmonic, Incongruence, TowerRefusal};
    let at = TowerRefusal::At;
    for p in [
        NoPose::Mirrored,
        NoPose::Incongruent(Incongruence::Order { region: at(Harmonic::Two), sites: at(Harmonic::Two) }),
        NoPose::Incongruent(Incongruence::Order { region: at(Harmonic::Three), sites: at(Harmonic::Four) }),
        NoPose::Incongruent(Incongruence::Order { region: at(Harmonic::Four), sites: TowerRefusal::Unplaced }),
        NoPose::Incongruent(Incongruence::Chirality),
        NoPose::Incongruent(Incongruence::Base),
        NoPose::Incongruent(Incongruence::Tube),
        NoPose::Isotropy,
        NoPose::Irrational,
    ] {
        assert_eq!(p.obstruction_kind(), Some(ObstructionKind::Permanent), "{p:?}");
    }
    assert_eq!(NoPose::Oblique.obstruction_kind(), Some(ObstructionKind::Dialect));
    assert_eq!(NoPose::Bodiless.obstruction_kind(), Some(ObstructionKind::Integrity));
    assert_eq!(NoPose::Unoriented.obstruction_kind(), None, "a held stabilizer, not a refusal");
}
