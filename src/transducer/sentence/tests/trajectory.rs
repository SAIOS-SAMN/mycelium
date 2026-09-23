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
/// the circuit read off it.
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
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let _ = take_orbits(&mut stroke);
    finish_circuits(&mut stroke);
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

/// OR3 — AN ORBIT WHOSE EDGE HALF IS REFUSED IS UNREAD, NEVER TRIVIAL. The
/// chord turns by its word and carries no edge: the orbit is kept, its
/// line reads unread at the chord's step, not rigid, and no defect class is
/// filed off the face half alone.
#[test]
fn an_orbit_whose_edge_half_is_refused_reads_unread_at_its_step() {
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    f.faces.push(f.face_carrying(1, 2, order4()).expect("a chord carrying no edge"));
    reset_orbit_defects();
    arm_orbits();
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(orbits.len(), 1, "the word keeps the orbit, or nothing is tested");
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let lines = take_orbits(&mut stroke);
    finish_circuits(&mut stroke);
    let _ = take_circuits();
    let [OrbitLine::Unread { at: Some(at), why: Some(why) }] = lines.as_slice() else {
        panic!("one unread orbit: {lines:?}")
    };
    assert_eq!(why.label(), "not rigid");
    let chord = (f.faces.len() - 1) as u16;
    let carried = paths_carried(&f.faces, &f.vertices, &orbits[0]).expect("the circuit reads the cycle");
    assert!(
        carried.scoped().all(|(_, c)| c.edge_lost_at == Some(*at) && c.steps[*at - 1].face == chord),
        "lost at the chord's step, {at}"
    );
    assert_eq!(orbit_defects(), (0, 0, 0, 0, 0), "no class filed off the face half");
}

/// Two bodies on the Frame's star and a chord carrying no edge, with `extra`
/// written on the chord beside the pair the circuit crosses; the circuits read.
fn refused_circuits(extra: Option<(Vec<u8>, FacePlace)>) -> Vec<Circuit> {
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    let mut chord = f.face_carrying(1, 2, order4()).expect("a chord carrying no edge");
    if let Some((k, place)) = extra {
        chord.at.insert(k, place);
    }
    f.faces.push(chord);
    arm_orbits();
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(orbits.len(), 1, "the word keeps the orbit, or nothing is tested");
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let _ = take_orbits(&mut stroke);
    finish_circuits(&mut stroke);
    take_circuits().expect("the circuits were armed").circuits
}

/// CT1, 79.3 — THE LADDER RUNS ON A REFUSED CIRCUIT BY ITS REFUSAL'S TYPE:
/// a chord that is not rigid asks its other chart pairs and its tether; with
/// neither, every arm refuses by name and the circuit is P, a wall. With a
/// second chart pair on the chord carrying an edge, a reused fibre reads what
/// the circuit's own route could not: C. A circuit that reads a class of its
/// own is not tested at all.
#[test]
fn the_reuse_ladder_classes_a_refused_circuit_by_how_its_arms_read() {
    use crate::observe::{ArmRead, CircuitClass, TestArm};
    let [c] = refused_circuits(None).try_into().expect("one circuit");
    let t = c.test.expect("a refused circuit is tested");
    assert_eq!(t.arms, vec![(TestArm::OtherPairs, ArmRead::Empty), (TestArm::Tether, ArmRead::Empty)]);
    assert_eq!(t.class, CircuitClass::P, "every arm refuses: a wall");

    let other = FacePlace {
        carriage: crate::transducer::element::identity_versor(),
        gauge: Vec::new(),
        edge: Some(Transport::from_word(order4())),
        refraction: None,
    };
    let [c] = refused_circuits(Some((vec![9, 9], other))).try_into().expect("one circuit");
    let t = c.test.expect("tested");
    assert_eq!(t.arms[0], (TestArm::OtherPairs, ArmRead::Reads(1)), "the other chart pair carries an edge");
    assert_eq!(t.class, CircuitClass::C, "only a reused fibre reads it: cover-relative");

    let [c] = circuits(false).try_into().expect("one circuit");
    assert_eq!(c.test, None, "a circuit with a class of its own triggers nothing");
}

/// CT1 — HOW THE ARMS DISAGREE: two routes disagreeing certify curvature
/// whatever else ran; a trivial circuit whose other routes agree is flat.
#[test]
fn disagreeing_routes_are_curved_and_agreeing_trivial_ones_flat() {
    use crate::observe::{ArmRead, CircuitClass, CircuitTest, TestArm};
    let routes = |agree, disagree| vec![(TestArm::OtherCircuits, ArmRead::Routes { agree, disagree })];
    assert_eq!(CircuitTest::classify(routes(1, 1), true, Vec::new()).class, CircuitClass::Curved);
    assert_eq!(CircuitTest::classify(routes(2, 0), true, Vec::new()).class, CircuitClass::Flat);
    assert_eq!(CircuitTest::classify(vec![(TestArm::Cone, ArmRead::Permanent)], false, Vec::new()).class, CircuitClass::P);
    // CT2 — only a constructed route reads it: a candidate, whatever T1 says.
    let constructed = vec![
        (TestArm::OtherPairs, ArmRead::Empty),
        (TestArm::ConstructedTether, ArmRead::Routes { agree: 0, disagree: 1 }),
    ];
    assert_eq!(CircuitTest::classify(constructed, false, Vec::new()).class, CircuitClass::Candidate);
}

/// 78.6(2) — A CIRCUIT THROUGH A STATIC CONTACT READS ITS TETHER HOLONOMY,
/// its gauge half static by name: the orbit line reads the geometric class
/// and names the gauge static, never the identity.
#[test]
fn a_circuit_through_a_static_contact_reads_its_tether_holonomy_with_the_gauge_static() {
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    let mut chord = f.face_carrying_with_edge(1, 2, Word::identity(), None).expect("a chord");
    chord.interface = [(1i64, 1i64, 0i64)].map(Site::from).to_vec();
    let a = crate::transducer::cga::sphere_at(&[q(0), q(0), q(0)], &q(1));
    let b = crate::transducer::cga::sphere_at(&[q(3), q(0), q(0)], &q(1));
    set_face_place(&mut chord, |p| {
        p.refraction = Some(Refraction::Static { tether: b.geo(&a), poses: [HeldPose::OneFrame, HeldPose::OneFrame] })
    });
    f.faces.push(chord);
    arm_orbits();
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(orbits.len(), 1, "a static edge keeps its loop: its gauge is never flat");
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let lines = take_orbits(&mut stroke);
    finish_circuits(&mut stroke);
    let _ = take_circuits();
    let [OrbitLine::Static(r)] = lines.as_slice() else { panic!("one orbit, static: {lines:?}") };
    assert_eq!(r.class, Some(BivectorKind::Hyperbolic), "two spheres apart compose a boost: the tether's class");
}

/// The single-route cell of 79.3, by hand: two bodies on the Frame's star
/// and a CONTACT chord between them whose refraction refused — no edge, no
/// coset, no static tether, one chart pair — so every reuse arm is empty;
/// the chord's bodies `p`, `r` and a third body `s` stand in one cell, each
/// with its nucleus. The circuits read with the routes CT2 constructs.
fn single_route_cell(nuclei: [crate::transducer::cga::Mv; 3]) -> Vec<Circuit> {
    let mut f = Form::default();
    for n in 1..=2u8 {
        f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
    }
    f.faces.push(f.face_of(0, 1).expect("star a"));
    f.faces.push(f.face_of(0, 2).expect("star b"));
    let mut chord = f.face_carrying(1, 2, order4()).expect("a chord carrying no edge");
    chord.interface = [(1i64, 1i64, 0i64)].map(Site::from).to_vec();
    let chord_ix = f.faces.len() as u32;
    f.faces.push(chord);
    let (p, r, s) = (vec![1u8], vec![2u8], vec![3u8]);
    f.face_bodies.insert(chord_ix.to_be_bytes(), (p.clone(), r.clone()));
    f.cells.insert([0u8], FormCell { bodies: vec![p, r, s], witnesses: Vec::new(), nuclei: Some(nuclei.to_vec()) });
    arm_orbits();
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(f.orbits.len(), 1, "the word keeps the orbit, or nothing is tested");
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    let _ = take_orbits(&mut stroke);
    construct_routes(&f, &mut stroke);
    finish_circuits(&mut stroke);
    take_circuits().expect("the circuits were armed").circuits
}

/// CT2, 79.3 — WHERE EVERY REUSE ARM IS EMPTY, CONSTRUCT, AND READ T1: the
/// tether between the lost face's two bodies completes the circuit, and so
/// does that tether carried by the cell's third constructor; two spheres
/// apart compose a boost, so the completed loop is not the identity and T1
/// reads the routes as disagreeing — curved — while only a constructed route
/// reads it: the circuit is a CANDIDATE, each construction offered with its
/// constructor, verdict, holonomy and modes, nothing installed. Where the two
/// bodies' nuclei coincide, the tether is the identity ray and T1 reads flat.
#[test]
fn every_reuse_arm_empty_constructs_a_candidate_and_reads_t1() {
    use crate::observe::{ArmRead, CircuitClass, TestArm};
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let sphere = |x: i64| crate::transducer::cga::sphere_at(&[q(x), q(0), q(0)], &q(1));
    let [c] = single_route_cell([sphere(0), sphere(3), sphere(0)]).try_into().expect("one circuit");
    let t = c.test.expect("a refused circuit is tested");
    assert!(
        t.arms.iter().filter(|(a, _)| !a.constructs()).all(|(_, r)| *r == ArmRead::Empty),
        "every reuse arm is empty: {:?}",
        t.arms
    );
    assert!(t.arms.contains(&(TestArm::ConstructedTether, ArmRead::Routes { agree: 0, disagree: 1 })), "{:?}", t.arms);
    assert!(t.arms.contains(&(TestArm::ConstructedFibre, ArmRead::Routes { agree: 0, disagree: 1 })), "{:?}", t.arms);
    assert_eq!(t.class, CircuitClass::Candidate, "only a constructed route reads it");
    assert_eq!(t.offers.len(), 2, "each construction offered");
    assert!(t.offers.iter().all(|o| !o.flat && !o.holonomy.is_empty()), "curved, with its holonomy");

    let [c] = single_route_cell([sphere(0), sphere(0), sphere(5)]).try_into().expect("one circuit");
    let t = c.test.expect("tested");
    let tether = t.offers.iter().find(|o| o.arm == TestArm::ConstructedTether).expect("the tether offered");
    assert!(tether.flat, "coincident nuclei: the tether is the identity ray, the routes agree");
    assert_eq!(t.class, CircuitClass::Candidate, "still a candidate: only a construction reads it");
}

/// A READING NOT TAKEN IS NOT AN EMPTY ONE: once taken, the circuits are
/// gone until the next percept arms them.
#[test]
fn circuits_taken_twice_are_not_read_the_second_time() {
    let _ = circuits(false);
    assert!(take_circuits().is_none(), "nothing armed since the take");
}
