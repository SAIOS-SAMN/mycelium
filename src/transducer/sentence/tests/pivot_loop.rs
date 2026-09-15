// 12.6, A20 — THE PIVOT LOOP: what a pivot discharges, what it holds as
// a vortex, and what it writes. Split from `pivot.rs` at the §1.1
// 600-line reading bound when the `Crystal` fixture grew the two reads
// of 50.2 step 2; the items are moved, not rewritten (§1.6).

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

/// The crystal both pivot files hand the loop: the words a mind holds on
/// an axis. It lives here with the tests that exercise the loop; `pivot.rs`
/// reads it across the split.
pub(super) struct Hold(pub Vec<Word>);
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
    fn on_body(&self, c: &stalk::Invariant) -> Vec<(Element, Form, Standing)> {
        self.on_class(c)
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
    /// Words on axes, which is the pivot's vocabulary and not a
    /// presentation: these words live in no form, so nothing here can be
    /// cited by (form, vertex).
    fn generators_under(
        &self,
        _chart: &stalk::Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        Word,
    )> {
        Vec::new()
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
    // S9.1: the certificate states the MOVE — one unit out of torsion —
    // rather than two audits for a reader to subtract for itself.
    assert_eq!(
        certificate.rank_move,
        RankMove::Discharged { from: RankAxis::Torsion },
        "the loop's finite order left torsion and an axis opened"
    );
    assert!(
        f.certificates.contains(&certificate.key()),
        "the form carries the discharge by key"
    );
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("the discharged form crystallizes");
    assert!(
        matches!(f.vertices.as_slice(), [Vertex::Frame, ..]),
        "Frame is vertex 0 of the crystallized form"
    );
    assert_eq!(
        crate::court::verdict::blind_replay(&e),
        None,
        "a certified discharge replays: every after-face is face_of, then the certificate"
    );
    let mut tampered =
        crate::transducer::sentence::from_element_form(&e).expect("v2 tape");
    let fi = tampered
        .certificates
        .scoped()
        .next()
        .expect("one certificate")
        .1
        .by
        .face() as usize;
    tampered.faces[fi].word = q.clone();
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

    // A19 ON A CLASS THE COURT HAS RULED: another mind can supply the word,
    // so this one neither demands it nor announces the filing. It still
    // files — the class is the form's own carrier — and it does so in
    // silence, which is what `Standing::Ruled` buys.
    let mut f = chord_form(std::slice::from_ref(&q));
    let mut horizon = Keyed::new();
    let r = q.rotor();
    horizon.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Ruled));
    let snaps = f.pivot_loop(
        [0x33; 32],
        &horizon,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(snaps.is_empty(), "a court-ruled class files silently: {snaps:?}");
    assert!(
        q.rotor().class_key().is_some_and(|k| f.torsion.contains(k.as_bytes())),
        "filed torsion is keyed by the class"
    );

    // AND THE SAME CLASS AT `Held` DOES NOT SUPPRESS IT. `Standing::Held` is
    // "an idea this mind is holding, not yet a word" — the mind's OWN
    // obstruction — and `hold_form` paints every standing loop's own class
    // onto the horizon at that standing through `paint_standing`. Read as
    // presence alone this made every loop defer to itself: no demand, no
    // announced filing, and no widening, all three closed by an entry the
    // mind wrote about its own defect. Measured at 7e73a1d — `held true` on
    // 108 of 108 loops against zero `horizon … seeded` lines in two hours.
    let mut f = chord_form(std::slice::from_ref(&q));
    let mut mine = Keyed::new();
    let r = q.rotor();
    mine.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Held));
    let snaps = f.pivot_loop(
        [0x34; 32],
        &mine,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { .. })),
        "a class this mind merely holds is announced, not deferred to: {snaps:?}"
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
            Some(face),
            "tree face {fi} re-derives, word and every member pair"
        );
    }
}

#[test]
fn pivot_discharges_only_its_axis_and_raises_none() {
    // T14: Placement.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        set_face_place(fa, |p| p.gauge.clear());
    }
    let chord = f.loops[0].chord as usize;
    set_face_place(&mut f.faces[chord], |p| {
        p.carriage = element::carriage(&q1(), &(q1(), q0()));
    });
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
        set_face_place(fa, |p| p.gauge.clear());
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
        set_face_place(fa, |p| p.gauge.clear());
    }
    let chord = f.loops[0].chord as usize;
    let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
    set_face_place(&mut f.faces[chord], |p| {
        p.carriage = element::carriage(&two, &(q0(), q0()));
    });
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

/// A16 — A LOOP'S READINGS ARE A FUNCTION OF ITS FACES AND THE FORM'S AXIS
/// CARRIER, and of nothing else the form holds. That is why a trial reads
/// its own answer without copying a form for every candidate: it writes the
/// one face the move names, walks the loops those faces close into, and
/// reads the class and the row off them.
///
/// The two entry points must therefore answer alike on a live form — the
/// methods, which read the form's own faces and axes, and the free readers
/// the trial uses. A trial that answered differently from the form it
/// trials would certify a move the form does not make.
#[test]
fn a_loops_class_and_row_read_off_its_faces_and_the_axis_carrier() {
    use crate::transducer::sentence::pivot::drive::class_vectors_on;
    use crate::transducer::sentence::{loop_row_on, opened_axis_on};
    let f = chord_form(&[order4(), order2(), Word::identity()]);
    assert!(f.loops.len() >= 3, "three chords close three loops: {}", f.loops.len());
    for (i, lp) in f.loops.iter().enumerate() {
        // EVERY MEMBER PATH, compared path for path: the trial and the form
        // read the same paths and the same vector on each, so a trial that
        // agreed on one path and differed on another is caught.
        let cvs_a = class_vectors_on(&f.faces, &f.vertices, &f.axes, lp);
        let cvs_b = f.class_vectors(i);
        assert_eq!(
            cvs_a.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
            cvs_b.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
            "the classes the trial reads are the classes the form reads, loop {i}"
        );
        assert_eq!(
            loop_row_on(&f.faces, &f.vertices, &f.axes, lp),
            f.loop_row_at(i),
            "and so is the rank row, loop {i}"
        );
        assert_eq!(
            opened_axis_on(&f.faces, &f.vertices, &f.axes, lp),
            f.opened_axis_of(lp),
            "and the axis it reads as opened, loop {i}"
        );
    }
    // AND WITH AN AXIS OPEN. The carrier is what the readings consult, so
    // a trial holding its own copy of it reads the opened loop as opened.
    let mut opened = f.clone();
    let [lp, ..] = f.loops.as_slice() else { panic!("three chords close three loops") };
    let chord = lp.chord;
    if opened.open_axis_on(chord).is_some() {
        for (i, lp) in opened.loops.iter().enumerate() {
            let cvs_a = class_vectors_on(&opened.faces, &opened.vertices, &opened.axes, lp);
            let cvs_b = opened.class_vectors(i);
            assert_eq!(
                cvs_a.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
                cvs_b.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>()
            );
            assert_eq!(
                loop_row_on(&opened.faces, &opened.vertices, &opened.axes, lp),
                opened.loop_row_at(i)
            );
        }
    }
}

/// THE LOOP IS FILED UNDER THE CLASS IT IS JUDGED BY. A face's word is the
/// coboundary `w_j ∘ w_i⁻¹`, and a product of coboundaries around a cycle
/// telescopes to the identity for every input, so a loop over coboundary
/// faces is flat on the word whatever its bodies do. Its EDGE composites
/// (50.3) do not telescope — `edge_holonomies_on` says so in its own doc —
/// and they are where a cover's curvature lives.
///
/// `class_vector` has read the edge composite since Y5. The filing decision
/// did not: `pivot_loop` took `lp.holonomy` for the horizon key and for the
/// order that selects the `Discharge` branch, and `file_loop` filed under
/// `lp.holonomy.rotor().class_key()`. A loop was judged by one object and
/// filed under another, and on the fleet at e98e8a2 that meant 70311 loops
/// flat on the word, 284 finite edge composites over the same loops, and
/// zero torsion filed.
///
/// This fixture is exactly that disagreement: three coboundary faces, so
/// the word holonomy is the identity, and a quarter turn on the one face
/// that does not touch the Frame.
#[test]
fn a_loop_flat_on_the_word_files_under_its_edge_composite() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::stalk::Invariant;
    let chart = Invariant(vec![3, 5]);
    let pair = face_pair_key(&chart, &chart);
    let q = order4();
    let word_vertex = |b: u8| {
        Vertex::word_on(
            crate::transducer::address::Address::from_bytes([4u8, b]),
            Derivation::spelled(Word::identity()),
            chart.clone(),
            Box::new(element::identity_versor()),
            Vec::new(),
            None,
        )
    };
    let place = |edge: Option<Word>| FacePlace {
        carriage: element::identity_versor(),
        gauge: Vec::new(),
        edge: edge.map(|w| Transport {
            word: w,
            carriage: element::identity_versor(),
            gauge: Vec::new(),
        }),
    };
    // `Form::default` already seeds the Frame at 0; the words are 1 and 2.
    let mut f = Form::default();
    f.vertices.push(word_vertex(1));
    f.vertices.push(word_vertex(2));
    // Every face word is the identity, so the word holonomy telescopes to
    // the identity — which is what it does for every coboundary anyway.
    for (i, j, edge) in [(0u16, 1u16, None), (0, 2, None), (1, 2, Some(q.clone()))] {
        let mut face = FormFace::new(i, j, Word::identity());
        face.at.insert(pair.clone(), place(edge));
        f.faces.push(face);
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    assert_eq!(f.loops.len(), 1, "three faces on three vertices close one loop");
    assert!(
        f.loops[0].holonomy.rotor().is_identity(),
        "the word holonomy is flat, as a product of coboundaries always is"
    );

    let hw = f.holonomy_word(0).expect("the paths agree on one holonomy");
    // The loop runs the chord backward, so the composite is `q⁻¹`. It is the
    // same CLASS as `q` — conjugate, and one class key since a class is read
    // off the ray and not off a lift — and the class is what torsion is filed
    // under, so that is what is asserted.
    assert_eq!(
        hw.rotor().order(),
        Some(crate::transducer::alphabet::Order::Finite(4)),
        "the holonomy the paths carry is the edge composite's quarter turn"
    );
    assert_eq!(
        hw.rotor().class_key(),
        q.rotor().class_key(),
        "and it is the quarter turn's class"
    );

    // And the filing follows the judging: the class written is the edge's,
    // and the identity's class — which is what the word would have filed —
    // is not.
    assert!(f.file_loop(0, 4), "a path with a class files");
    let edge_class = q.rotor().class_key().expect("the quarter turn has a class");
    let flat_class = crate::transducer::alphabet::Rotor::identity()
        .class_key()
        .expect("the identity has a class");
    assert!(
        f.torsion.contains(edge_class.as_bytes()),
        "filed under the class the loop is judged by"
    );
    assert!(
        !f.torsion.contains(flat_class.as_bytes()),
        "and not under the identity the word holonomy reports"
    );

    // D2: `Loop.carriages` IS the transport composite's carriage, path for
    // path. `path_carriages` computed it in a second walk — the same
    // frontier, the same steps, the same closing rule, with the word and the
    // gauge discarded — so the walk ran once at `loops_of` and again at every
    // class reading. One carrier now; this asserts they were the same thing.
    let lp = &f.loops[0];
    let transport = loop_transport(&f, lp);
    assert_eq!(
        lp.carriages.len(),
        transport.len(),
        "one carriage per closed member path, and the same paths"
    );
    for (k, t) in transport.scoped() {
        let held = lp.carriages.get(&k).expect("the loop carries this path");
        assert!(
            element::same_versor(held, &t.carriage),
            "the stored carriage is the transport composite's"
        );
    }
}

/// Y2 (16.16, 12.6, 3.9, 25.1): A held loop with a held word on that axis discharges
/// and certifies on a constructed family.
#[test]
fn a_held_loop_with_a_held_word_discharges_and_certifies() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let family = vec![(vec![vec![0u8; 4]; 4], vec![vec![0u8; 4]; 4])];
    let snaps = f.pivot_loop(
        [0x11; 32],
        &Keyed::new(),
        &Hold(vec![q.clone()]),
        &mut crate::physics::Physics::default().stroke_n(16),
        Some(&family),
        0,
    );
    let Some(Snap::Discharged { certificate, .. }) =
        snaps.iter().find(|s| matches!(s, Snap::Discharged { .. }))
    else {
        panic!("a crystal holding q discharges on constructed family: {snaps:?}");
    };
    assert_eq!(
        certificate.before,
        ClassVector::of_word(&q).expect("q").class_key()
    );
    assert_eq!(
        certificate.after,
        ClassVector::of_word(&Word::identity()).expect("identity").class_key()
    );
    assert_eq!(
        certificate.rank_move,
        RankMove::Discharged { from: RankAxis::Torsion },
        "the loop's finite order left torsion and an axis opened"
    );
    assert!(
        matches!(certificate.mu, Mu::Measured { .. }),
        "pressure was measured on the constructed family"
    );
    assert!(
        f.certificates.contains(&certificate.key()),
        "the form carries the discharge by key"
    );
}

/// Y2 (16.16, 12.6, 3.9, 25.1): Two candidate words discharging the loop are held as a vortex
/// on a constructed family.
#[test]
fn two_certified_discharges_are_held_as_a_vortex() {
    let q = order4();
    let qinv = q.inverse();
    let mut f = chord_form(std::slice::from_ref(&q));
    let family = vec![(vec![vec![0u8; 4]; 4], vec![vec![0u8; 4]; 4])];
    let snaps = f.pivot_loop(
        [0x22; 32],
        &Keyed::new(),
        &Hold(vec![q.clone(), qinv]),
        &mut crate::physics::Physics::default().stroke_n(16),
        Some(&family),
        0,
    );
    let Some(Snap::Vortex { held, .. }) = snaps.iter().find(|s| matches!(s, Snap::Vortex { .. }))
    else {
        panic!("q and q⁻¹ on constructed family are held as a vortex: {snaps:?}");
    };
    assert_eq!(held.len(), 2, "two certificates held, got {held:?}");
    assert!(!f.vortex.is_empty(), "the form holds the vortex cells");
    for cert in held {
        assert!(
            matches!(cert.mu, Mu::Measured { .. }),
            "each candidate's pressure was measured on the constructed family"
        );
        assert_eq!(
            cert.rank_move,
            RankMove::Discharged { from: RankAxis::Torsion },
            "each candidate discharges torsion"
        );
    }
}
