// 12.6 — DESCENT and THE PIVOT: descent certifies and never selects, the
// pivot reads only occupied axes, and what cannot be ordered is held.

use super::common::*;
use crate::network::nerve::Keyed;
use super::pivot_loop::Hold;
use crate::transducer::alphabet::{Order, Word};
use crate::transducer::element;
use crate::transducer::sentence::*;

#[test]
fn pivot_candidates_are_read_from_occupied_axes_only() {
    let id = ClassVector::of_word(&Word::identity()).expect("identity");
    assert!(id.occupied().is_empty() && id.is_zero());
    let k2 = ClassVector::of_word(&order2()).expect("order2");
    let occ = k2.coordinates();
    assert_eq!(occ, vec![Axis::Orientation(0)], "sheet parity occupies Sheet(0), got {occ:?}");
    let inf = ClassVector::of_word(&infinite()).expect("infinite");
    assert!(
        inf.free_part(),
        "a pythagorean-5 holonomy is free"
    );
    let mut seen = occ.clone();
    seen.extend(inf.coordinates());
    let mut declaration = seen.clone();
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — canonical key order: 12.6's own axis order, which is what a declaration is written in"
    )]
    declaration.sort_by_key(|a| match a {
        Axis::Orientation(i) => (0, *i),
        Axis::Unit(i) => (1, *i),
        Axis::Prime5(i) => (2, *i),
        Axis::Prime13(i) => (3, *i),
        Axis::Hue => (4, 0),
        Axis::Placement => (5, 0),
        Axis::Dilation => (6, 0),
        Axis::OffPlane => (7, 0),
        Axis::Open => (8, 0),
    });
    assert_eq!(seen, declaration, "occupied axes already in declaration order");
}

/// A16 step 5: SEVERAL discharges are a vortex. Not "several that cannot be
/// ordered" — several, whatever their μ⃗ says, because nothing orders them.
#[test]
fn several_discharges_are_held_as_vortex() {
    let a = cert(Axis::Unit(0), [2, 0, 1, 0]);
    let b = cert(Axis::Hue, [0, 3, 0, 1]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "two certified discharges are a vortex, not a pick"
    );
}

/// T13 — μ⃗ NEITHER CERTIFIES NOR SELECTS. Two candidates discharge and one
/// measures lower on every lane: it is still a vortex. Dominance used to
/// apply that one, which made the lower measurement a reason, which is a
/// score (law 1). μ⃗ enters a trial once, to refuse a candidate that RAISES
/// a lane, and never enters the comparison between certified discharges.
#[test]
fn mu_neither_certifies_nor_selects() {
    let loose = cert(Axis::Unit(0), [4, 4, 4, 4]);
    let tight = cert(Axis::Unit(0), [1, 1, 1, 1]);
    let Discharge::Vortex(cells) = select_discharge(&[loose.clone(), tight.clone()]) else {
        panic!("a lower μ⃗ on every lane is not a reason to apply it");
    };
    assert_eq!(cells.len(), 2, "both are held: {cells:?}");
    fn perm(m: [u64; 4]) -> [u64; 4] {
        [m[2], m[1], m[0], m[3]]
    }
    fn perm_mu(mu: Mu) -> Mu {
        match mu {
            Mu::Measured { before, after } => Mu::Measured {
                before,
                after: perm(after),
            },
            Mu::Unmeasured => Mu::Unmeasured,
        }
    }
    let mut lp = loose.clone();
    let mut tp = tight.clone();
    lp.mu = perm_mu(lp.mu);
    tp.mu = perm_mu(tp.mu);
    let permuted = select_discharge(&[lp, tp]);
    assert_eq!(
        permuted,
        select_discharge(&[loose, tight]),
        "permuting the lanes changes nothing, because nothing read them"
    );
    let a = cert(Axis::Unit(0), [2, 0, 0, 0]);
    let b = cert(Axis::Hue, [0, 2, 0, 0]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "and two on different axes are a vortex too"
    );

    let q = order4();
    let qinv = q.inverse();
    let x: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
    ];
    let bodies = crate::transducer::family::perceive(&x);
    assert!(!bodies.is_empty(), "the bar is a body");
    let class = bodies[0].moduli.clone();
    assert!(!class.0.is_empty(), "the body has a chart");
    let mut spoken = Form::default();
    spoken.vertices.push(Vertex::word_on(
        form_addr(1),
        crate::transducer::alphabet::Derivation::spelled(q.clone()),
        class.clone(),
        Box::new(element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()))),
        vec![(1, 1)],
        None,
    ));
    let y = crate::transducer::family::apply_form(
        &spoken,
        &x,
        &Word::identity(),
        &crate::transducer::family::NoCrystal,
    )
    .and_then(|s| s.grid())
    .expect("q speaks the bar and it fits the frame");
    assert!(
        y.iter().flatten().any(|&h| h != 0),
        "the spoken image is not the empty frame"
    );
    let mut f = chord_form(std::slice::from_ref(&q));
    // THE VERTEX KEEPS ITS IDENTITY AND CHANGES ITS MEMBER. The chart is the
    // key the member is filed under, so putting the vertex on `class` is
    // building the member on that chart — the address and the derivation are
    // read off the vertex that is there rather than restated here.
    let Vertex::Word { address, derivation, .. } = &f.vertices[1] else {
        panic!("the chord form's first body is a word vertex");
    };
    f.vertices[1] = Vertex::word_on(
        address.clone(),
        derivation.clone(),
        class,
        Box::new(element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()))),
        vec![(1, 1)],
        None,
    );
    let bundle = vec![(x, y)];
    let snaps = f.pivot_loop(
        [0xde; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(32),
        Some(&bundle),
        3,
    );
    let Some(Snap::Vortex { held, between, .. }) =
        snaps.iter().find(|s| matches!(s, Snap::Vortex { .. }))
    else {
        panic!("two discharges on one loop are a vortex: {snaps:?}");
    };
    assert!(held.len() >= 2, "q and q⁻¹ both discharge: {held:?}");
    let (mu0, mu1) = match (&held[0].mu, &held[1].mu) {
        (
            Mu::Measured { before: b0, after: a0 },
            Mu::Measured { after: a1, .. },
        ) => {
            assert_eq!(b0[2], 3, "T is the extent passed in");
            assert_eq!(a0[2], 3);
            (*a0, *a1)
        }
        _ => panic!("a measured vortex carries μ⃗, got {:?}", held[0].mu),
    };
    assert_eq!(mu0, mu1, "a chord write moves no vertex, so μ⃗ is unmoved");
    // S9.16 — THE VORTEX RECORDS THE OBSTRUCTION, and spends it on nothing.
    // Here the two cells measure the same μ⃗, so every lane HELD; the axes
    // they acted on are what separates them, and that is what a later
    // collapse resolves.
    assert_eq!(between.len(), held.len() - 1, "one reading per adjacency");
    let o = &between[0];
    assert_eq!(
        o.lanes,
        Some([crate::economy::LaneMove::Held; 4]),
        "equal μ⃗ reads as four held lanes, not as a tie score"
    );
    assert_eq!(o.axes, (held[0].axis, held[1].axis), "the axes are the obstruction");
}

#[test]
fn cauterize_filed_torsion_crystallizes() {
    let mut f = chord_form(&[order4()]);
    assert!(!f.is_closed());
    let _ = f.pivot_loop(
        [0xca; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(f.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("crystallized form");
    assert!(
        carries_torsion(&e),
        "the tape carries the filed cycle"
    );
}

#[test]
fn two_loops_of_one_class_are_both_filed_or_discharged() {
    let w = order4();
    fn two_chords(w: &Word) -> Form {
        chord_form(&[w.clone(), w.clone()])
    }
    let mut f = two_chords(&w);
    assert_eq!(f.loops.len(), 2, "two independent chords");
    let snaps = f.pivot_loop(
        [0xa3; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let filed = snaps
        .iter()
        .filter(|s| matches!(s, Snap::TorsionFiled { .. }))
        .count();
    assert_eq!(filed, 2, "both loops of one class file: {snaps:?}");

    let q = order4();
    let mut f = two_chords(&q);
    let snaps = f.pivot_loop(
        [0xa3; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(32),
        None,
        0,
    );
    let disc = snaps
        .iter()
        .filter(|s| matches!(s, Snap::Discharged { .. }))
        .count();
    assert_eq!(disc, 2, "both loops discharge: {snaps:?}");
}

#[test]
fn pivot_never_reopens_a_closed_chord() {
    let mut cob = coboundary_chord_form();
    let cob_snaps = cob.pivot_loop(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        cob_snaps.iter().any(|s| matches!(s, Snap::Closed { .. })),
        "a coboundary chord closes by the tree gauge: {cob_snaps:?}"
    );
    assert!(
        cob.loops.iter().all(|lp| lp.holonomy.is_identity()),
        "T2: the closed chord stays closed"
    );
    let again = cob.pivot_loop(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        !again.iter().any(|s| matches!(s, Snap::Demand { .. })),
        "T2: a closed chord is not demanded: {again:?}"
    );

    let mut f = chord_form(&[order2(), order4()]);
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    let first = f.pivot_loop(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut budget,
        None,
        0,
    );
    let closed: Vec<u32> = first
        .iter()
        .filter_map(|s| match s {
            Snap::Closed {
                order: Order::Finite(e),
                ..
            } => Some(*e),
            Snap::TorsionFiled { ell, .. } => Some(*ell),
            _ => None,
        })
        .collect();
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    let second = f.pivot_loop(
        [0xcd; 32],
        &crate::network::nerve::Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut budget,
        None,
        0,
    );
    for o in closed {
        assert!(
            !second.iter().any(|s| matches!(s, Snap::Demand { .. })) || o > 0,
            "a closed finite chord is not demanded"
        );
        let still = f.loops.iter().any(|lp| {
            !lp.holonomy.is_identity() && matches!(lp.holonomy.order(), Order::Finite(ell) if ell == o)
        });
        let filed = f.torsion.scoped().any(|(_, filed)| filed.period == o);
        assert!(
            !still || filed,
            "order {o} stayed closed or filed; second {second:?}"
        );
    }
}
