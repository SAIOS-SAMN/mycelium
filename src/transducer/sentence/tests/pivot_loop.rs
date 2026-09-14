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

/// 12.6, T21 (iii), S9.1 — A PIVOT MOVES ONE UNIT OR NONE. `Rank::moved`
/// names the move: the rank held, or exactly one unit left torsion, free or
/// off-plane for `axes`. Everything else is `Inadmissible` and carries its
/// four deltas so the refusal can say what happened.
///
/// The predicate this replaces was a four-coordinate monotone, and it
/// admitted `(−3, −2, −1, +7)` — three coordinates emptied and seven axes
/// opened — as readily as A18's one step. That is not a pivot that went
/// well; it is a different thing the audit could not name.
#[test]
fn a_rank_move_is_one_unit_or_none() {
    let before = Rank { torsion: 3, free: 2, offplane: 1, axes: 0 };

    // A18's step, from each coordinate it can leave: one unit, whether or
    // not the discharge opened a chart axis for it to land in. Today none
    // does — `rank_of_rows` writes `axes: 0` unconditionally — so both
    // shapes are asserted and the second is the one every live pivot makes.
    for (after, from) in [
        (Rank { torsion: 2, free: 2, offplane: 1, axes: 1 }, RankAxis::Torsion),
        (Rank { torsion: 2, free: 2, offplane: 1, axes: 0 }, RankAxis::Torsion),
        (Rank { torsion: 3, free: 1, offplane: 1, axes: 1 }, RankAxis::Free),
        (Rank { torsion: 3, free: 2, offplane: 0, axes: 0 }, RankAxis::OffPlane),
    ] {
        assert_eq!(Rank::moved(before, after), RankMove::Discharged { from });
    }
    // A pivot that leaves the audit alone is certified by its closure.
    assert_eq!(Rank::moved(before, before), RankMove::Equal);
    // Everything else is named, with its deltas: two units out of torsion,
    // however welcome the fall looks; the sprawl the old monotone admitted;
    // an axis closing, which is a coordinate lost and not a saving.
    for (a, b, delta) in [
        (before, Rank { torsion: 1, free: 2, offplane: 1, axes: 1 }, [-2, 0, 0, 1]),
        (before, Rank { torsion: 0, free: 0, offplane: 0, axes: 7 }, [-3, -2, -1, 7]),
        (Rank { axes: 2, ..before }, Rank { axes: 0, ..before }, [0, 0, 0, -2]),
    ] {
        assert_eq!(Rank::moved(a, b), RankMove::Inadmissible { delta });
    }
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
        assert_eq!(
            class_vectors_on(&f.faces, &f.vertices, &f.axes, lp),
            f.class_vectors(i),
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
            assert_eq!(
                class_vectors_on(&opened.faces, &opened.vertices, &opened.axes, lp),
                opened.class_vectors(i)
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

/// 16.20, A20, I6, R8 — THE WHOLE `OpenAxis` PIPELINE READS ONE HOLONOMY,
/// and it is the edge composite.
///
/// `lp.holonomy` is the WORD. A face's word is the coboundary `w_j ∘ w_i⁻¹`
/// of its two vertices' words, and `∏ (w_j ∘ w_i⁻¹) ≡ 1` around any cycle
/// by algebra — an exact Čech coboundary, `δw = 0`. So every reader keyed to
/// it asked whether a boundary was non-trivial, which it never is.
///
/// The consequences were not one bug but four, each in a different organ:
/// `candidates_for`'s gate refused the widening on every coboundary loop;
/// the seed was `versor_of_word(identity)`, an axis standing for nothing the
/// loop turns by; `open_axis_on` filed under the identity's class; and
/// `opened_axis_on` looked the axis up under the identity's class, so the
/// audit could not see an opening that HAD been made. Measured at f3a76ec:
/// `opens 1` on 133 loops, the single one whose word did not cancel, against
/// 92 torsion classes filed off the edge composite in the same five minutes.
///
/// This fixture is the disagreement itself: three coboundary faces, so the
/// word holonomy is the identity, and a quarter turn on the one face that
/// does not touch the Frame. Every site is asserted on that one loop — the
/// keying invariant is total or it is not an invariant.
#[test]
fn the_open_axis_pipeline_keys_on_the_edge_composite_not_the_flat_word() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::sentence::{loop_row_on, opened_axis_on};
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
    let mut f = Form::default();
    f.vertices.push(word_vertex(1));
    f.vertices.push(word_vertex(2));
    for (i, j, edge) in [(0u16, 1u16, None), (0, 2, None), (1, 2, Some(q.clone()))] {
        let mut face = FormFace::new(i, j, Word::identity());
        face.at.insert(pair.clone(), place(edge));
        f.faces.push(face);
    }
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let [lp, ..] = f.loops.as_slice() else { panic!("three faces close one loop") };
    let chord = lp.chord;

    // THE TWO CARRIERS DISAGREE, which is the precondition for the rest.
    assert!(
        lp.holonomy.rotor().is_identity(),
        "the word holonomy telescopes flat, as a product of coboundaries always does"
    );
    let composite = f.loop_holonomy(lp).expect("the paths agree on one holonomy");
    assert!(
        !composite.rotor().is_identity(),
        "the edge composite carries the quarter the word cancelled"
    );

    // THE SEED comes from the composite, so it is a versor that turns.
    let seed = f.axis_seed_of(lp).expect("the composite seeds the axis");
    assert!(
        !element::same_versor(&seed, &element::identity_versor()),
        "seeded from the word this was the identity versor, standing for nothing"
    );

    // THE WRITER AND THE READER AGREE. An axis opened over this loop is
    // found again by the reader that has to count it — keyed to the word,
    // `open_axis_on` filed under the identity's class and `opened_axis_on`
    // looked under it too, so the opening was invisible to the audit the one
    // time the classes differed.
    assert!(
        opened_axis_on(&f.faces, &f.vertices, &f.axes, lp).is_none(),
        "no axis is open yet"
    );
    let mut opened = f.clone();
    opened.open_axis_on(chord).expect("the axis opens over the composite's class");
    let [olp, ..] = opened.loops.as_slice() else { panic!("one loop") };
    assert_eq!(
        opened_axis_on(&opened.faces, &opened.vertices, &opened.axes, olp),
        Some(chord),
        "the reader finds the axis the writer filed"
    );
    assert_eq!(
        loop_row_on(&opened.faces, &opened.vertices, &opened.axes, olp),
        Some(LoopRow::Opened),
        "and the audit counts the loop as opened, not as the defect it was"
    );

    // AND THE INVERSE REMOVES WHAT THE OPENING WROTE. Keyed to the word,
    // `close_axis_on` removed nothing and the axis stood with no loop
    // claiming it.
    opened.close_axis_on(chord).expect("the axis closes under the same class");
    let [clp, ..] = opened.loops.as_slice() else { panic!("one loop") };
    assert!(
        opened_axis_on(&opened.faces, &opened.vertices, &opened.axes, clp).is_none(),
        "the un-pivot removes the axis the pivot opened"
    );
}

/// A18, T21 (iii) — AN OPENING'S RANK IS READ OVER EVERY ROW, because the
/// axis carrier is keyed by CLASS and not by loop.
///
/// `axes.insert(class, face)` files the opening under the loop's holonomy
/// class, and `opened_axis_on` answers `Opened` for ANY loop whose composite
/// class equals it. So a second loop of the same class is opened by the same
/// write, and a rank-after that substituted only the trialled loop's row
/// certified a move the form does not make. The drive catches the mismatch
/// at `apply_pivot` and holds the loop — which is right, and is also a
/// certificate that should never have been written.
///
/// Measured at 551c609, fifteen minutes: 11 `axis Open discharged` against 6
/// `held, the applied pivot did not make the move its certificate claims`.
/// The 6 appeared in the same window the widening started firing at rate —
/// the defect was always there and nothing reached it while the gate was
/// keyed to the flat word.
///
/// A `Word`, `Placement` or `Hue` move writes one face's transport, so
/// exactly one loop's holonomy moves and the substitution is the whole truth
/// for those. This asserts the certificate against the form for both kinds.
#[test]
fn an_openings_certified_rank_is_the_rank_the_form_moves_by() {
    let q = order4();
    let f = chord_form(&[q.clone(), q.clone()]);
    assert_eq!(f.loops.len(), 2, "two chords of one class close two loops");
    let [a, b] = f.loops.as_slice() else { panic!("two loops") };
    assert_eq!(
        f.loop_holonomy(a).and_then(|h| h.rotor().class_key()),
        f.loop_holonomy(b).and_then(|h| h.rotor().class_key()),
        "the two loops carry ONE class, which is what the axis is keyed by"
    );

    let rows = f.loop_rows().expect("rows");
    let rank_before = rank_of_rows(&rows).expect("rank");
    // Loop 0 by the slice pattern, not by `first()`: the index IS content
    // here, because `trial` is asked about that loop and no other (rule 15).
    let [lp, ..] = f.loops.as_slice() else { panic!("two loops") };
    let chord = lp.chord;
    let seed = f.axis_seed_of(lp).expect("the composite seeds the axis");
    let cand = Candidate {
        axis: Axis::Open,
        by: PivotBy::OpenAxis { face: chord, seed },
    };
    let prior = crate::transducer::sentence::pivot::drive::Prior {
        rows: &rows,
        rank_before,
        crystal: &crate::transducer::family::NoCrystal,
        bundle: None,
        t_extent: 0,
        mu_before: None,
    };
    // THE TRIAL REFUSES RATHER THAN CERTIFYING A MOVE THE FORM WILL NOT
    // MAKE. Reading the rank-after over every row, this opening is seen for
    // what it is: `axes` is keyed by class, both loops carry that class, so
    // both read `Opened` and the audit moves `[-2, 0, 0, +2]` — two units,
    // which T21 (iii) is not. Substituting only the trialled loop's row hid
    // the second unit and certified `[-1, 0, 0, +1]`, and the drive then
    // refused the applied move against its own certificate.
    //
    // THE REFUSAL IS THE CORRECT OUTCOME AND NOT THE END OF IT. Two loops of
    // one class are ONE obstruction seen twice — attaching one cell kills
    // both representatives, which is what `opened_axis_on` answering by
    // class means. `rank_of_rows` counts `LoopRow::Torsion` PER LOOP while
    // it counts `offplane` per CLASS through a `Keyed` set, so the same
    // object is double-counted on one coordinate and not on the other. That
    // is the reason a topologically single move reads as two units, and it
    // is a defect in the audit, not in this trial. Until it is settled the
    // trial refuses, which is rule 11 and costs nothing but the opening.
    assert!(
        f.trial(0, &cand, &f.class_vector(0).expect("class"), &prior).is_none(),
        "an opening that would move two units is refused, not certified"
    );

    // AND A ONE-LOOP FORM STILL CERTIFIES AND STILL MAKES ITS MOVE, so the
    // reading above is about the shared class and not about openings.
    let mut one = chord_form(std::slice::from_ref(&q));
    let rows = one.loop_rows().expect("rows");
    let rank_before = rank_of_rows(&rows).expect("rank");
    let [lp, ..] = one.loops.as_slice() else { panic!("one loop") };
    let cand = Candidate {
        axis: Axis::Open,
        by: PivotBy::OpenAxis {
            face: lp.chord,
            seed: one.axis_seed_of(lp).expect("seed"),
        },
    };
    let prior = crate::transducer::sentence::pivot::drive::Prior {
        rows: &rows,
        rank_before,
        crystal: &crate::transducer::family::NoCrystal,
        bundle: None,
        t_extent: 0,
        mu_before: None,
    };
    let cert = one
        .trial(0, &cand, &one.class_vector(0).expect("class"), &prior)
        .expect("one loop of one class opens in one unit");
    one.apply_pivot(&cert);
    assert_eq!(
        one.rank().map(|r| Rank::moved(rank_before, r)),
        Some(cert.rank_move),
        "the applied opening makes the move its certificate claims"
    );
}

/// 12.6, 16.16 step 3, A17, I6 — REDUCE, PUBLISH, THEN WIDEN.
///
/// A pivot is a REDUCTION: it divides the residual by a generator the mind
/// holds. Where the basis holds no word on the coordinate there is nothing
/// to divide by, every contraction candidate is terminal by algebra, and
/// asking again next stroke asks the same question of the same basis. The
/// move out of that is the reduction's inverse — widen the cover so the
/// class is re-read as a coordinate of the form rather than a defect in it
/// — but it is the THIRD move, not the second.
///
/// The second is 12.6 step 6: file the class, which puts it to the mesh for
/// a mind that CAN divide it. Widening first buries the defect under a
/// wider cover and asks nobody, and it also pre-empts the filing outright —
/// a discharging expansion wins `select_discharge` and the loop never
/// reaches the filing arm. Measured: offered on irreducibility alone, six
/// fixtures that file a finite holonomy against an empty crystal stopped
/// filing and the whole filed-torsion path went dead.
///
/// `kappa.free_part()` alone gated the widening, and a free part is ONE way
/// a cover can be too narrow, not the only one. A quarter turn is finite
/// order — torsion, no prime exponent, not off-plane — so it has no free
/// part and never saw the move at all. On the fleet at 769a17c every loop
/// with no candidate occupied `[Unit(0)]`, 60 of 60, every crystal read
/// answered 0, 123 of 123, and not one had a free part.
///
/// Every arm of the sequence is asserted, because the gate is the CRYSTAL'S
/// OWN ANSWER and the FORM'S OWN FILING, never a trial's failure.
#[test]
fn an_unanswered_irreducible_residual_is_widened_and_a_reducible_or_unasked_one_is_not() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let kappa = f.class_vector(0).expect("the chord closes one loop");
    assert!(
        !kappa.free_part(),
        "a quarter turn is torsion: no prime exponent and not off-plane, so \
         the free-part gate alone never offers it the widening — {:?}",
        kappa.coordinates()
    );

    // FIRST: the basis divides it. There is something to reduce by, so the
    // contraction is offered and the widening is not.
    let unpublished: Keyed<FiledLoop> = Keyed::new();
    let reducible = f.candidates_for(0, &kappa, &Hold(vec![q.clone()]), false, &unpublished);
    assert!(
        reducible.iter().any(|c| c.axis != Axis::Open),
        "a held word on the coordinate is offered as a contraction: {:?}",
        reducible.iter().map(|c| c.axis).collect::<Vec<_>>()
    );
    assert!(
        !reducible.iter().any(|c| c.axis == Axis::Open),
        "a residual the basis can divide is not widened: {:?}",
        reducible.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // SECOND: nothing divides it and it has not been put to the mesh. The
    // move is to file, not to widen, so no candidate is offered at all and
    // the drive falls through to 12.6 step 6.
    let unasked =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, false, &unpublished);
    assert!(
        !unasked.iter().any(|c| c.axis == Axis::Open),
        "an unpublished residual is filed, not widened: {:?}",
        unasked.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // THIRD: the class has been filed — put to the mesh — and a stroke has
    // passed with nothing coming back. Now there is no move left but the
    // reduction's inverse. The filing is read as the PREVIOUS stroke's,
    // which is what the drive hands down: a class filed by this stroke is
    // not a demand anyone has had a stroke to answer.
    assert!(f.file_loop(0, 4), "the finite holonomy files under its class");
    let published = f.torsion.clone();
    let asked =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, false, &published);
    assert!(
        asked.iter().any(|c| c.axis == Axis::Open),
        "a published, unanswered, irreducible residual is widened: {:?}",
        asked.iter().map(|c| c.axis).collect::<Vec<_>>()
    );

    // AND THE HORIZON STILL WITHHOLDS IT. A class the mesh already holds is
    // not this mind's to open an axis over (A19): it waits for the form.
    let on_the_horizon =
        f.candidates_for(0, &kappa, &crate::transducer::family::NoCrystal, true, &published);
    assert!(
        !on_the_horizon.iter().any(|c| c.axis == Axis::Open),
        "a class the mesh holds is not widened: {:?}",
        on_the_horizon.iter().map(|c| c.axis).collect::<Vec<_>>()
    );
}
