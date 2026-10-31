use super::super::*;
use super::common::*;
use crate::physics::tensegrity::{metric_defect, similarity};
use crate::transducer::cga::{point_at, sphere_at};

/// The register holds every law, each at its own coordinate with a
/// statement.
#[test]
fn laws_reads_each_law_at_own_coordinate() {
    let addresses: Vec<_> = LAWS.iter().map(|l| l.address()).collect();
    for (law, a) in LAWS.iter().zip(&addresses) {
        assert_eq!(a.as_bytes(), [crate::transducer::address::TAG_LAW, law.index(), LAW_VERSION].as_slice());
        assert!(!law.statement().is_empty());
    }
    for (i, a) in addresses.iter().enumerate() {
        assert!(addresses.iter().skip(i + 1).all(|b| b != a), "every law at its own coordinate");
    }
}

/// A turn that carries its bodies is admitted under every law, split into
/// its modes and exchanging no torque.
#[test]
fn verify_admits_turn_carrying_bodies() {
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let v = verify(&t, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert!(v.modes >= 1, "the turn split into its modes");
    assert!(v.exchanged.is_empty(), "an unforced turn exchanges no torque");
}

/// 78.116 — THE VERDICT IS THE TRANSITION'S, NOT THE STROKE'S: under seven
/// reads (the court's default stroke on the laptop the first kernel uplink
/// ran against) the reference turn reads a false `Conformal` breach — its
/// modes unsplit for want of stroke; under the stroke its own modes need it
/// is admitted, as it is under any stroke that suffices.
#[test]
fn verify_sized_leaves_turn_unread_on_short_stroke() {
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let short = verify(&t, &mut crate::physics::Physics::default().stroke_n(7));
    // 78.121 — a short stroke leaves the law UNREAD, never broken: no
    // breach, the stroke it needs named, and no admission.
    assert!(short.breaches.is_empty(), "nothing breached: {:?}", short.breaches);
    let need = crate::transducer::cga::modes_need(&t.motion).and_then(|n| u64::try_from(n).ok());
    assert_eq!(short.unmeasured, vec![Unmeasured { law: Law::Conformal, needed_stroke: need }]);
    assert!(!short.admitted() && !short.measured());
    let sized = verify_sized(&t);
    assert!(sized.admitted(), "{:?}", sized.breaches);
    assert_eq!(sized, verify(&t, &mut budget()), "the verdict of the transition alone");
}

/// Each law refuses the transition that breaks it, and every breach is
/// named beside the others, not only the first.
#[test]
fn verify_names_every_breach_by_law() {
    let laws_of = |v: &Verdict| v.breaches.iter().map(|(l, _)| *l).collect::<Vec<_>>();
    // TIME: the height stands still.
    let still = Transition { motion: turn(), heights: (8, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    assert_eq!(laws_of(&verify(&still, &mut budget())), vec![Law::Time]);
    // CONFORMAL: a body's nucleus is not where the motion sends it.
    let mut moved = carried(&turn());
    if let [(_, after), ..] = moved.as_mut_slice() {
        after.nucleus = sphere_at(&[q(5), q(5), q(0)], &q(1));
    }
    let v = verify(&Transition { motion: turn(), heights: (1, 2), bodies: moved, hues: Vec::new(), fluxes: Vec::new() }, &mut budget());
    assert!(v.breaches.contains(&(Law::Conformal, Breach::NotCarried { body: 0 })), "{:?}", v.breaches);
    assert!(laws_of(&v).contains(&Law::Rigidity), "and the pair's distance moved: {:?}", v.breaches);
    // MOMENTUM: a body's inertia changed with no body to answer for it.
    let mut spun = carried(&turn());
    if let [(_, after), ..] = spun.as_mut_slice() {
        after.body.moments[1][1] = q(3);
    }
    let v = verify(&Transition { motion: turn(), heights: (1, 2), bodies: spun, hues: Vec::new(), fluxes: Vec::new() }, &mut budget());
    assert!(matches!(v.breaches.as_slice(), [(Law::Momentum, Breach::MomentumNotConserved { .. })]), "{:?}", v.breaches);
    // RESERVOIR, beside TIME: a merge whose moved preimage is not recorded.
    let lossy = Transition { motion: turn(), heights: (3, 3), bodies: carried(&turn()), hues: vec![(1, 2), (2, 2)], fluxes: Vec::new() };
    assert_eq!(laws_of(&verify(&lossy, &mut budget())), vec![Law::Time, Law::Reservoir], "every breach named, not the first");
}

/// 78.59 — A POINT IS READ, AT UNIT WEIGHT: two point nuclei carried by the
/// turn keep the nuclei's product (no squared ratio was ever needed). A uniform
/// dilation with no motion moves every pair — rigidity is strict — and reads
/// as one ratio, the dilation lane; a drift of one body moves the pairs it
/// touches, by no one ratio.
#[test]
fn verify_admits_points_and_similarity_reads_dilation() {
    let mut bodies = carried(&turn());
    for (before, after) in bodies.iter_mut() {
        before.nucleus = point_at(&before.body.centre);
        after.nucleus = turn().sandwich(&before.nucleus);
    }
    let v = verify(&Transition { motion: turn(), heights: (1, 2), bodies: bodies.clone(), hues: Vec::new(), fluxes: Vec::new() }, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert_eq!(v.similarity, Some(q(1)), "an isometry");
    let pts = |cs: &[[i64; 3]]| cs.iter().map(|c| point_at(&c.iter().map(|&x| q(x)).collect::<Vec<_>>())).collect::<Vec<_>>();
    let before = pts(&[[1, 0, 0], [0, 2, 0], [0, 0, 3]]);
    let doubled = pts(&[[2, 0, 0], [0, 4, 0], [0, 0, 6]]);
    let pairs: Vec<(Mv, Mv)> = before.iter().cloned().zip(doubled).collect();
    assert_eq!(metric_defect(&pairs).len(), 3, "rigidity is strict: a dilation with no motion to answer for it moves every pair");
    assert_eq!(similarity(&pairs), Some(q(4)), "and reads as one ratio: every pair's product × 4, lengths × 2");
    let drifted = pts(&[[1, 0, 0], [0, 2, 0], [0, 0, 5]]);
    let pairs: Vec<(Mv, Mv)> = before.into_iter().zip(drifted).collect();
    assert_eq!(metric_defect(&pairs).len(), 2, "a drift moves the two pairs it touches, each named");
    assert_eq!(similarity(&pairs), None, "and no one ratio");
}

/// LV2 (law version 2, ruled 2026-10-03) — A DILATION CONSERVES MOMENTUM UP TO
/// ITS RATIO. A body at the origin dilated by 2 — its nucleus the dilator's
/// image, its mass four times (a lattice body's sites) — holds momentum four
/// times what the dilator carried: one ratio, docketed as the expansion
/// lane, admitted. Two bodies whose momenta move by two ratios under one
/// dilation stand to no one ratio: refused, as version 1 refused every
/// dilation. A rigid motion is held to exact conservation as before
/// (`verify_names_every_breach_by_law`).
#[test]
fn verify_admits_dilation_momentum_up_to_one_ratio() {
    let d = crate::transducer::cga::dilator(&q(2));
    let at_origin = |mass: i64, radius: i64| {
        let mut b = body([0, 0, 0]);
        b.mass = q(mass);
        BodyState { nucleus: sphere_at(&b.centre, &q(radius)), body: b }
    };
    let dilated = |s: &BodyState, mass: i64| BodyState { nucleus: d.sandwich(&s.nucleus), body: Body { mass: q(mass), ..s.body.clone() } };
    let one = at_origin(1, 1);
    let t = Transition { motion: d.clone(), heights: (1, 2), bodies: vec![(one.clone(), dilated(&one, 4))], hues: Vec::new(), fluxes: Vec::new() };
    let v = verify(&t, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert_eq!(v.expansion, Some(q(4)), "the momentum's ratio, docketed as the expansion lane");
    let two = at_origin(1, 2);
    let split = Transition {
        motion: d.clone(),
        heights: (1, 2),
        bodies: vec![(one.clone(), dilated(&one, 4)), (two.clone(), dilated(&two, 9))],
        hues: Vec::new(),
        fluxes: Vec::new(),
    };
    let v = verify(&split, &mut budget());
    assert!(v.breaches.iter().any(|(l, _)| *l == Law::Momentum), "two ratios are no expansion lane: {:?}", v.breaches);
    assert_eq!(v.expansion, None);
}

/// LAW VERSION 3 (78.89) — CONTINUITY: THE WHITE HOLE AND THE BLACK HOLE
/// PUBLISHED. Under a turn, which does not dilate, a body that gains two of
/// mass with nothing recorded is refused — two made, named exactly; the
/// same gain admitted where a source at a finite place emits it; a loss
/// admitted where a sink absorbs it; a flux of no mass, or at no finite
/// place, refused by its index. A dilation's change of mass stays its
/// expansion lane (`verify_admits_dilation_momentum_up_to_one_ratio`).
#[test]
fn verify_refuses_mass_change_without_source_or_sink() {
    let heavier = |m: i64| {
        let mut b = carried(&turn());
        if let [(_, after), ..] = b.as_mut_slice() {
            after.body.mass = q(m);
        }
        b
    };
    // The flux stands where the mass lands: at the body, after the turn.
    let at = {
        let b = carried(&turn());
        let [(_, after), ..] = b.as_slice() else { panic!("two bodies carried") };
        point_at(&after.body.centre)
    };
    let source = |m: i64| Flux { flow: Flow::Source, at: at.clone(), mass: q(m) };
    let sink = |m: i64| Flux { flow: Flow::Sink, at: at.clone(), mass: q(m) };
    let of = |bodies, fluxes| Transition { motion: turn(), heights: (1, 2), bodies, hues: Vec::new(), fluxes };
    // Unrecorded, the gain breaks both laws, each named: mass was made, and
    // the momentum it carries came from nowhere.
    let v = verify(&of(heavier(3), Vec::new()), &mut budget());
    let laws: Vec<Law> = v.breaches.iter().map(|(l, _)| *l).collect();
    assert_eq!(laws, vec![Law::Momentum, Law::Continuity], "{:?}", v.breaches);
    assert!(v.breaches.contains(&(Law::Continuity, Breach::MassNotConserved { made: q(2) })));
    assert!(verify(&of(heavier(3), vec![source(2)]), &mut budget()).admitted(), "a source emitted it");
    // A lighter body is carried as the turn carries it, its momentum read
    // on the spatial plane its mass does not weigh; the sink took the rest.
    let lighter = {
        let mut b = carried(&turn());
        if let [(before, after), ..] = b.as_mut_slice() {
            before.body.mass = q(3);
            after.body.mass = q(1);
        }
        b
    };
    assert!(verify(&of(lighter, vec![sink(2)]), &mut budget()).admitted(), "a sink absorbed it");
    let nowhere = Flux { flow: Flow::Source, at: crate::transducer::cga::e_inf(), mass: q(2) };
    let v = verify(&of(heavier(3), vec![nowhere, source(0)]), &mut budget());
    assert!(v.breaches.contains(&(Law::Continuity, Breach::FluxUnplaced { index: 0 })), "at no finite place: {:?}", v.breaches);
    assert!(v.breaches.contains(&(Law::Continuity, Breach::FluxUnplaced { index: 1 })), "of no mass: {:?}", v.breaches);
}

/// 78.91 — A CHANGE THAT IS NO MOTION READ AS THE FLUX ITS LINKS CARRY. A
/// creation's gained sites are a source at the sites' barycentre, its mass the
/// count gained, exact over ℚ (here a half); a vanishing is a sink at the
/// lost sites'; a reshaping at constant mass is no flux; and the lone flux
/// put to the laws is admitted where it stands at a finite place.
#[test]
fn flux_of_links_reads_creation_and_vanishing() {
    let half = Q::new(1.into(), 2.into());
    // Two sites gained on row 1, at columns 0 and 1; a site held throughout.
    let made = [(1, 0, 0, 3), (1, 1, 0, 3), (0, 0, 2, 2)];
    let f = flux_of_links(&made).expect("mass was made");
    assert_eq!(f.flow, Flow::Source);
    assert_eq!(f.mass, q(2));
    assert_eq!(f.at, point_at(&[q(1), half.clone()]), "at the gained sites' barycentre");
    assert!(verify_flux(&f, &mut budget()).admitted(), "{:?}", verify_flux(&f, &mut budget()).breaches);
    // One site lost at (2, 3): a sink there, of one.
    let lost = [(2, 3, 4, 0), (0, 0, 2, 2)];
    let f = flux_of_links(&lost).expect("mass was lost");
    assert_eq!((f.flow, f.mass.clone(), f.at.clone()), (Flow::Sink, q(1), point_at(&[q(2), q(3)])));
    assert!(verify_flux(&f, &mut budget()).admitted());
    // A site moved and one recoloured: the mass kept, no flux.
    assert_eq!(flux_of_links(&[(0, 0, 1, 0), (0, 1, 0, 1), (1, 1, 2, 5)]), None);
    assert_eq!(flux_of_links(&[]), None);
}
