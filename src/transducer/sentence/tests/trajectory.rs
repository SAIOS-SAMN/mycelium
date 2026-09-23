// 78.12(4), PL1 — THE CIRCUIT'S TRAJECTORY: a kept circuit reads as its
// steps, each oriented the way the circuit runs, and the first step whose
// face refracts is named.

use super::common::*;
use crate::network::nerve::coord::BivectorKind;
use crate::observe::{Carried, Circuit};
use crate::transducer::alphabet::Word;
use crate::transducer::base::Site;
use crate::transducer::sentence::*;
use crate::transducer::stalk;

/// Two bodies on the Frame's star and a chord between them whose edge turns
/// by the Pythagorean `3 + 4e₁₂` — its weight a rational square, so its
/// modes read — the chord carrying an interface or not, and the circuits
/// the walk read off it.
fn circuits(contact: bool) -> Vec<Circuit> {
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    let turn = Transport { word: order4(), carriage: crate::transducer::cga::rotator(&q(3), &q(4)), gauge: Vec::new() };
    let mut chord = f.face_carrying_with_edge(1, 2, Word::identity(), Some(turn)).expect("a chord between two bodies");
    if contact {
        chord.interface = [(1i64, 1i64, 0i64), (3, 3, 0)].map(Site::from).to_vec();
    }
    f.faces.push(chord);
    arm_orbits();
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(orbits.len(), 1, "the fixture must close one orbit, or nothing is tested");
    let _ = take_orbits();
    take_circuits().expect("the circuits were armed").circuits
}

/// A KEPT CIRCUIT READS AS ITS STEPS: the two star steps and the chord, each
/// read, the stars flat and the chord a turn with its one elliptic mode, and
/// the class composed
/// through the last step is the orbit's own holonomy class — the house read
/// off its bricks, not beside them.
#[test]
fn a_kept_circuit_reads_as_its_steps_and_composes_to_its_holonomy() {
    let [c] = circuits(false).try_into().expect("one circuit");
    assert_eq!(c.steps.len(), 3, "the triangle through the Frame is three steps");
    let kinds: Vec<(Option<BivectorKind>, Option<BivectorKind>)> = c
        .steps
        .iter()
        .map(|s| match &s.carried {
            Carried::Read { kind, after, .. } => (*kind, *after),
            other => panic!("every step is read: {other:?}"),
        })
        .collect();
    let turns = kinds.iter().filter(|(k, _)| *k == Some(BivectorKind::Elliptic)).count();
    assert_eq!(turns, 1, "one step turns — the chord — and the stars carry no turn: {kinds:?}");
    let modes: Vec<&ModesRead> = c
        .steps
        .iter()
        .filter_map(|s| match &s.carried {
            Carried::Read { kind: Some(BivectorKind::Elliptic), modes, .. } => Some(modes),
            _ => None,
        })
        .collect();
    assert_eq!(
        modes,
        vec![&ModesRead::Split(vec![(BivectorKind::Elliptic, 1, None)])],
        "the turning step reads its one mode, a rosette that never closes"
    );
    let [.., (_, last)] = kinds.as_slice() else { panic!("three steps") };
    assert_eq!(*last, Some(BivectorKind::Elliptic), "the steps compose to the orbit's class");
    assert_eq!(c.first_refraction(), None, "no face carries an interface, so none refracts");
}

/// THE FIRST REFRACTION IS NAMED: the chord face carrying an interface is
/// the step where a refraction enters, by its position in the circuit and
/// its face.
#[test]
fn the_first_step_whose_face_refracts_is_named() {
    let [c] = circuits(true).try_into().expect("one circuit");
    let at = c.first_refraction().expect("the contact face refracts");
    assert!(c.steps[at].refracted);
    assert!(
        matches!(&c.steps[at].carried, Carried::Read { kind: Some(BivectorKind::Elliptic), .. }),
        "the refracting step is the chord's turn: {:?}",
        c.steps[at]
    );
    assert_eq!(c.steps.iter().filter(|s| s.refracted).count(), 1, "and it is the only one");
}

/// A READING NOT TAKEN IS NOT AN EMPTY ONE: once taken, the circuits are
/// gone until the next percept arms them.
#[test]
fn circuits_taken_twice_are_not_read_the_second_time() {
    let _ = circuits(false);
    assert!(take_circuits().is_none(), "nothing armed since the take");
}
