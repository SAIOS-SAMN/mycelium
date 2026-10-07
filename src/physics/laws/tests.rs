// 78.62 — the register and the verifier, tested by the algebra: a turn that
// carries its bodies is admitted under every law, and each law refuses the
// transition that breaks it, by name, with every other breach beside it.

use super::*;
use crate::physics::tensegrity::{metric_defect, similarity};
use crate::physics::tensegrity::carried_body;
use crate::transducer::cga::{point_at, rotator, sphere_at};

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A body at `centre` with a second moment on its first axis.
fn body(centre: [i64; 3]) -> Body {
    Body {
        centre: centre.iter().map(|&c| q(c)).collect(),
        mass: q(1),
        moments: vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]],
    }
}

/// The (3,4,5) turn in (e₁, e₂) — `2 + e₁₂`, a rational rotor, no angle.
fn turn() -> Mv {
    rotator(&q(2), &q(1))
}

/// Two bodies carried by `motion` exactly: nucleus (a unit sphere — a point
/// is null and its inversive distance unread) by the sandwich, body by the
/// rigid carriage.
fn carried(motion: &Mv) -> Vec<(BodyState, BodyState)> {
    [[1, 0, 0], [0, 2, 0]]
        .iter()
        .map(|&c| {
            let b = body(c);
            let nucleus = sphere_at(&b.centre, &q(1));
            let after = BodyState { nucleus: motion.sandwich(&nucleus), body: carried_body(&b, motion).expect("a rigid carriage") };
            (BodyState { nucleus, body: b }, after)
        })
        .collect()
}

#[test]
fn the_register_holds_every_law_each_at_its_own_coordinate() {
    let addresses: Vec<_> = LAWS.iter().map(|l| l.address()).collect();
    for (law, a) in LAWS.iter().zip(&addresses) {
        assert_eq!(a.as_bytes(), [crate::transducer::address::TAG_LAW, law.index(), LAW_VERSION].as_slice());
        assert!(!law.statement().is_empty());
    }
    for (i, a) in addresses.iter().enumerate() {
        assert!(addresses.iter().skip(i + 1).all(|b| b != a), "every law at its own coordinate");
    }
}

#[test]
fn a_turn_that_carries_its_bodies_is_admitted_under_every_law() {
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let v = verify(&t, &mut budget());
    assert!(v.admitted(), "{:?}", v.breaches);
    assert!(v.modes >= 1, "the turn split into its modes");
    assert!(v.exchanged.is_empty(), "an unforced turn exchanges no torque");
}

/// 78.116 — THE VERDICT IS THE TRANSITION'S, NOT THE STROKE'S: under seven
/// reads (the court's default stroke on the laptop the first xylarium uplink
/// ran against) the reference turn reads a false `Conformal` breach — its
/// modes unsplit for want of stroke; under the stroke its own modes need it
/// is admitted, as it is under any stroke that suffices.
#[test]
fn a_lawful_turn_is_admitted_under_the_stroke_its_modes_need() {
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let short = verify(&t, &mut crate::physics::Physics::default().stroke_n(7));
    assert!(
        matches!(short.breaches.as_slice(), [(Law::Conformal, Breach::Undecomposed(Undecomposed::Unbounded(_)))]),
        "the short stroke's false breach: {:?}",
        short.breaches
    );
    let sized = verify_sized(&t);
    assert!(sized.admitted(), "{:?}", sized.breaches);
    assert_eq!(sized, verify(&t, &mut budget()), "the verdict of the transition alone");
}

#[test]
fn each_law_refuses_what_breaks_it_and_every_breach_is_named() {
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
/// turn keep their product (no squared ratio was ever needed). A uniform
/// dilation with no motion moves every pair — rigidity is strict — and reads
/// as one ratio, the dilation lane; a drift of one body moves the pairs it
/// touches, by no one ratio.
#[test]
fn points_are_read_at_unit_weight_and_a_dilation_is_a_similarity() {
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
/// (`each_law_refuses_what_breaks_it_and_every_breach_is_named`).
#[test]
fn a_dilation_conserves_momentum_up_to_its_ratio() {
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
/// expansion lane (`a_dilation_conserves_momentum_up_to_its_ratio`).
#[test]
fn mass_is_made_or_lost_only_at_a_recorded_source_or_sink() {
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
/// creation's gained sites are a source at their barycentre, its mass the
/// count gained, exact over ℚ (here a half); a vanishing is a sink at the
/// lost sites'; a reshaping at constant mass is no flux; and the lone flux
/// put to the laws is admitted where it stands at a finite place.
#[test]
fn a_change_that_is_no_motion_is_read_as_the_flux_its_links_carry() {
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

/// 78.64 H4 — THE CONTRACT'S BYTES ARE THE OBJECTS. Every law read back from
/// its coordinate; a transition the kernel sends and the verdict the court
/// answers decode to themselves exactly, so the court verifies what was sent
/// and the kernel reads what was ruled; and a verdict carrying every kind of
/// breach, each law beside it, survives its bytes whole. A truncated payload
/// decodes to nothing, never to a default.
#[test]
fn the_contracts_bytes_decode_to_the_objects_they_carry() {
    use super::codec::{law_at, put_transition, put_verdict, take_transition, take_verdict};
    for law in LAWS {
        assert_eq!(law_at(&law.address()), Some(law));
    }
    let t = Transition {
        motion: turn(),
        heights: (7, 8),
        bodies: carried(&turn()),
        hues: vec![(1, 2), (2, 3)],
        fluxes: vec![
            Flux { flow: Flow::Source, at: point_at(&[q(1), q(2), q(0)]), mass: q(3) },
            Flux { flow: Flow::Sink, at: sphere_at(&[q(0), q(0), q(0)], &q(1)), mass: Q::new(1.into(), 2.into()) },
        ],
    };
    let mut bytes = Vec::new();
    put_transition(&mut bytes, &t);
    let mut p = 0;
    assert_eq!(take_transition(&bytes, &mut p), Some(t.clone()));
    assert_eq!(p, bytes.len(), "every byte read, none left over");
    let v = verify(&t, &mut budget());
    let ruled = Verdict {
        breaches: vec![
            (Law::Conformal, Breach::NotAVersor),
            (Law::Conformal, Breach::Undecomposed(crate::transducer::cga::Undecomposed::Irrational(3))),
            (Law::Conformal, Breach::NotCarried { body: 1 }),
            (Law::Momentum, Breach::MomentumUnread { body: 0 }),
            (Law::Momentum, Breach::MomentumNotConserved { net: turn() }),
            (Law::Rigidity, Breach::NonRigid { i: 0, j: 1, delta: Q::new(5.into(), 4.into()) }),
            (Law::Rigidity, Breach::RigidityUnmeasured { i: 0, j: 2 }),
            (Law::Time, Breach::HeightNotAdvanced { before: 9, after: 9 }),
            (Law::Reservoir, Breach::NotUnitary),
            (Law::Continuity, Breach::MassNotConserved { made: Q::new((-3).into(), 2.into()) }),
            (Law::Continuity, Breach::FluxUnplaced { index: 1 }),
        ],
        ..v
    };
    let mut bytes = Vec::new();
    put_verdict(&mut bytes, &ruled);
    let mut p = 0;
    assert_eq!(take_verdict(&bytes, &mut p), Some(ruled));
    assert_eq!(p, bytes.len());
    let mut p = 0;
    assert_eq!(take_verdict(&bytes[..bytes.len() - 1], &mut p), None, "a truncated verdict is no verdict");
}

/// 78.62 steps 2 and 3 — THE ACT A KERNEL SPEAKS. The register request and
/// the verify request read back as what they ask; a verify request with a
/// byte left over is no request. The register's reply carries every law at
/// its coordinate with its statement, in the register's order, at the
/// register's version.
#[test]
fn a_kernel_asks_the_register_and_puts_a_transition_to_it() {
    use super::codec::{decode_laws_request, decode_register_reply, encode_register_reply, encode_register_request, encode_verify_request, LawsRequest};
    assert_eq!(decode_laws_request(&encode_register_request()), Some(LawsRequest::Register));
    let t = Transition { motion: turn(), heights: (7, 8), bodies: carried(&turn()), hues: Vec::new(), fluxes: Vec::new() };
    let mut asked = encode_verify_request(&t);
    assert_eq!(decode_laws_request(&asked), Some(LawsRequest::Verify(t)));
    asked.push(0);
    assert_eq!(decode_laws_request(&asked), None, "a byte left over is no request");
    let served = decode_register_reply(&encode_register_reply()).expect("the register reads back");
    assert_eq!(served.iter().map(|s| s.law).collect::<Vec<_>>(), LAWS.to_vec());
    assert!(served.iter().all(|s| s.statement == s.law.statement()));
}
