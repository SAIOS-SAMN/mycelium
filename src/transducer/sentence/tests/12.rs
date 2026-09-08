// 12.6 — DESCENT and THE PIVOT: descent certifies and never selects, the
// pivot reads only occupied axes, and what cannot be ordered is held.

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Order, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

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

#[test]
fn incomparable_discharges_are_held_as_vortex() {
    let a = cert(Axis::Unit(0), [2, 0, 1, 0]);
    let b = cert(Axis::Hue, [0, 3, 0, 1]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "incomparable μ⃗ are a vortex, not a pick"
    );
}

/// Descent certifies after a trial; it never ranks candidates. A chord
/// write changes no vertex, so speech and μ⃗ are unmoved: two words that
/// both discharge tie and are a vortex. Per-candidate μ⃗ moves only when
/// a candidate changes what the form speaks (a glue pivot).
#[test]
fn descent_certifies_never_selects() {
    let loose = cert(Axis::Unit(0), [4, 4, 4, 4]);
    let tight = cert(Axis::Unit(0), [1, 1, 1, 1]);
    let Discharge::One(won) = select_discharge(&[loose.clone(), tight.clone()]) else {
        panic!("one μ⃗ dominates");
    };
    assert_eq!(won.mu, tight.mu, "the lane-wise minimum is the discharge");
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
    let Discharge::One(won_p) = select_discharge(&[lp, tp]) else {
        panic!("permuted lanes keep the winner");
    };
    assert_eq!(won_p.axis, tight.axis, "the same pivot after a lane permutation");
    let a = cert(Axis::Unit(0), [2, 0, 0, 0]);
    let b = cert(Axis::Hue, [0, 2, 0, 0]);
    assert!(
        matches!(select_discharge(&[a, b]), Discharge::Vortex(_)),
        "two incomparable discharges are a vortex"
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
    spoken.vertices.push(Vertex::Word {
        address: form_addr(1),
        word: q.clone(),
        carriage: Box::new(element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()))),
        gauge: vec![(1, 1)],
        class: class.clone(),
        contact: None,
    });
    let y = crate::transducer::family::apply_form(
        &spoken,
        &x,
        &Word::identity(),
        &crate::transducer::family::NoCrystal,
    )
    .map(|s| s.grid())
    .expect("q speaks the bar");
    assert!(
        y.iter().flatten().any(|&h| h != 0),
        "the spoken image is not the empty frame"
    );
    let mut f = chord_form(std::slice::from_ref(&q));
    if let Vertex::Word {
        class: c,
        gauge,
        carriage,
        ..
    } = &mut f.vertices[1]
    {
        *c = class;
        *gauge = vec![(1, 1)];
        **carriage = element::carriage(&q1(), &(q1() + q1() + q1(), q1() + q1() + q1()));
    }
    let bundle = vec![(x, y)];
    let snaps = f.pivot_loop(
        [0xde; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(32),
        Some(&bundle),
        3,
    );
    let Some(Snap::Vortex { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Vortex { .. })) else {
        panic!("a pivot that does not write vertices leaves μ⃗ unmoved, so two discharges are a vortex: {snaps:?}");
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
    assert_eq!(mu0, mu1, "equal μ⃗, neither dominates");
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
        let filed = f.torsion.fiber(crate::network::nerve::ROOT).any(|(_, filed)| filed.period == o);
        assert!(
            !still || filed,
            "order {o} stayed closed or filed; second {second:?}"
        );
    }
}

struct Hold(Vec<Word>);
impl crate::transducer::family::Crystal for Hold {
    fn on_class(
        &self,
        _class: &stalk::Invariant,
    ) -> Vec<(
        Element,
        Form,
        Standing,
    )> {
        Vec::new()
    }
    fn speak_form(
        &self,
        _f: &Form,
        _x: &crate::transducer::family::Grid,
        _p: &Word,
    ) -> Option<crate::transducer::family::Speech> {
        None
    }
    fn by_address(&self, _a: &[u8]) -> Option<(Element, Form)> {
        None
    }
    fn on_axis(&self, _chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
        self.0
            .iter()
            .filter(|w| ClassVector::of_word(w).is_some_and(|cv| cv.occupies(axis)))
            .cloned()
            .collect()
    }
}

#[test]
fn pivot_loop_discharges_held_word_and_vortices_a_pair() {
    let q = order4();
    let qinv = q.inverse();
    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_loop(
        [0x11; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let Some(Snap::Discharged { certificate, .. }) = snaps.iter().find(|s| matches!(s, Snap::Discharged { .. })) else {
        panic!("a crystal holding q discharges: {snaps:?}");
    };
    assert_eq!(
        certificate.before,
        ClassVector::of_word(&q).expect("q").class_key()
    );
    assert_eq!(
        certificate.after,
        ClassVector::of_word(&Word::identity()).expect("identity").class_key()
    );
    assert_eq!(certificate.rank_before.torsion, 1);
    assert_eq!(certificate.rank_after.torsion, 0);
    assert!(
        f.certificates.contains(&certificate.key()),
        "the form carries the discharge by key"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("the discharged form crystallizes");
    assert!(
        matches!(f.vertices.first(), Some(Vertex::Frame)),
        "Frame is vertex 0 of the crystallized form"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "a certified discharge replays: every after-face is face_of, then the certificate"
    );
    let mut tampered = from_element_form(&e).expect("v2 tape");
    let fi = tampered
        .certificates
        .fiber(crate::network::nerve::ROOT)
        .next()
        .expect("one certificate")
        .1
        .by
        .face() as usize;
    tampered.faces[fi].transport.word = q.clone();
    let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
    assert_eq!(
        crate::court::verdict::blind_replay(&bad),
        Some(crate::court::verdict::REFUSE_LOOP),
        "tampering a certified face re-opens an unfiled loop"
    );

    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_loop(
        [0x22; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let Some(Snap::Unmeasured { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Unmeasured { .. })) else {
        panic!("q and q⁻¹ unmeasured are a vortex: {snaps:?}");
    };
    assert_eq!(held.len(), 2, "two certificates, got {held:?}");
    assert!(!f.vortex.is_empty(), "the form holds the vortex cells");

    let mut f = chord_form(std::slice::from_ref(&q));
    let mut horizon = Keyed::new();
    let r = q.rotor();
    horizon.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Held));
    let snaps = f.pivot_loop(
        [0x33; 32],
        &horizon,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(snaps.is_empty(), "a horizon holding q files silently: {snaps:?}");
    assert!(
        q.rotor().class_key().is_some_and(|k| f.torsion.contains(k.as_bytes())),
        "filed torsion is keyed by the class"
    );
}

#[test]
fn pivot_writes_the_chord_transport_only() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let verts_before = f.vertices.clone();
    let chord = f.loops[0].chord;
    let snaps = f.pivot_loop(
        [0xc0; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { .. })),
        "a held quarter discharges: {snaps:?}"
    );
    assert_eq!(
        f.vertices, verts_before,
        "a pivot writes no vertex placement or gauge"
    );
    for (fi, face) in f.faces.iter().enumerate() {
        if fi as u16 == chord {
            continue;
        }
        assert_eq!(
            f.face_of(face.i, face.j).as_ref(),
            Some(&face.transport),
            "tree face {fi} re-derives"
        );
    }
}

#[test]
fn pivot_discharges_only_its_axis_and_raises_none() {
    // T14: Placement.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    f.faces[chord].transport.carriage = element::carriage(&q1(), &(q1(), q0()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("loop");
    assert!(before.occupies(Axis::Placement));
    let _ = f.pivot_loop(
        [0xb4; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("loop remains");
    assert!(!after.occupies(Axis::Placement), "Placement zeroes");
    assert!(
        before.agrees_except(&after, Axis::Placement),
        "no other coordinate moved"
    );

    // T14: Orientation.
    let mut f = chord_form(&[order2()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("ori");
    assert!(before.occupies(Axis::Orientation(0)), "got {:?}", before.coordinates());
    let _ = f.pivot_loop(
        [0xb5; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("ori after");
    assert!(
        !after.occupies(Axis::Orientation(0)),
        "Orientation zeroes, got {:?}",
        after.coordinates()
    );
    assert!(before.agrees_except(&after, Axis::Orientation(0)));

    // T14: Dilation.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
    f.faces[chord].transport.carriage = element::carriage(&two, &(q0(), q0()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let before = f.class_vector(0).expect("dil");
    assert!(
        before.occupies(Axis::Dilation),
        "a dilator residue occupies Dilation, got {:?}",
        before.coordinates()
    );
    let _ = f.pivot_loop(
        [0xb6; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(0).expect("dil after");
    assert!(!after.occupies(Axis::Dilation), "Dilation zeroes, got {:?}", after.coordinates());
    assert!(before.agrees_except(&after, Axis::Dilation));
}
