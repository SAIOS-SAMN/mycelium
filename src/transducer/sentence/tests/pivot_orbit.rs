// 12.6, A20 — THE PIVOT ORBIT: what a pivot discharges, what it holds as
// a vortex, and what it writes.

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::Word;
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Standing};

/// O1, 70.7 — A FACE BETWEEN TWO CHARTS OF ONE VERTEX IS CROSSED THE WAY
/// ITS MEMBER PAIR RUNS.
///
/// One vertex acts on two charts, `a` and `b`, and the Frame's star reaches
/// both. A face from the vertex to itself relates `(v, a)` to `(v, b)` and
/// carries a translation. It is the chord, and the forest reaches its ends
/// through the Frame, so the cycle closes by crossing it from `b` to `a`,
/// against the face.
///
/// On the vertex pair, `f.i == v && f.j == v` holds whichever way the cycle
/// runs. The walk read the key `(b, a)`, which the face does not hold, and
/// refused the cycle as `no place`. Read off the member pair, the crossing
/// is backward on `(a, b)`, the translation is inverted, and the orbit
/// stands with nothing unread.
#[test]
fn a_face_between_two_charts_of_one_vertex_is_crossed_by_its_member_pair() {
    let (a, b) = (stalk::Invariant::counts(&[7]), stalk::Invariant::counts(&[9]));
    let mut f = Form::default();
    let mut v = word_vertex_on(1, Word::identity(), a.clone());
    if let Vertex::Word { acts, .. } = &mut v {
        let first = acts.scoped().next().map(|(_, x)| x.clone()).expect("one member");
        let member = Acting { chart: b.clone(), ..first };
        acts.put(&member.key(), member);
    }
    f.vertices.push(v);
    assert_eq!(f.vertices[1].charts().len(), 2, "the fixture must split, or nothing is tested");
    f.faces.push(f.face_of(0, 1).expect("the star reaches both charts"));
    let mut at = Keyed::new();
    at.insert(
        face_pair_key(&a, &b),
        FacePlace { carriage: element::translator_versor(3, 0), gauge: Vec::new(), edge: None },
    );
    f.faces.push(FormFace { i: 1, j: 1, word: Word::identity(), at, pairs: None, interface: Vec::new() });
    Form::reset_orbit_gates();
    let orbits = Form::orbits_of(&f.faces, &f.vertices);
    let (_, _, unread, _) = Form::orbit_gates();
    assert_eq!(unread, 0, "nothing is unread: the pair is held, one way round");
    assert_eq!(orbits.len(), 1, "the cycle through the self-face stands as an orbit");
    let carried = paths_carried(&f.faces, &f.vertices, &orbits[0]).expect("the walk reads the cycle");
    assert!(
        carried.scoped().all(|(_, c)| !element::same_versor(&c.face.carriage, &element::identity_versor())),
        "and it carries the translation, read across the face against its orientation"
    );
}

/// 57.4, G2 — A CARRIAGE ON THE LIGHT CONE LEAVES ITS ORBIT UNREAD.
///
/// The Frame reaches `c` through one star face written from `c`, so the
/// forest runs against it and the transport across it is the INVERSE of
/// the face's carriage. A carriage with `MM̃ = 0` has none: `1 + e₁e₋`
/// squares its bivector to `+1`, so `(1 + B)(1 − B) = 1 − B² = 0`.
///
/// THE CYCLE NEVER CROSSES THAT EDGE. `a` and `b` hang from `c` and the
/// chord between them closes a triangle wholly beyond it, so the walk of
/// the cycle's own faces reads every transport it meets. What the cone
/// takes away is the BASING: an identity written for the refused inverse
/// would base the triangle as though the Frame's edge were flat, and the
/// orbit would stand. The tree refuses instead, counts the edge on the
/// `unread` gate, and the cycle with its ends beyond it is unread too. The
/// same fixture with the carriage the face derived keeps its orbit, so the
/// refusal is the cone's and nothing else's.
#[test]
fn a_carriage_on_the_light_cone_leaves_its_orbit_unread() {
    use crate::transducer::cga::{Mv, E1, EM};
    let build = |on_the_cone: bool| {
        let mut f = Form::default();
        for n in 1..=4u8 {
            f.vertices.push(word_vertex_on(n, Word::identity(), stalk::Invariant::counts(&[u64::from(n)])));
        }
        // THE FOREST'S ROOT IS ITS FIRST-DISCOVERED NODE, so a star face
        // from the Frame to a fourth body comes first and roots it at the
        // Frame. Then `c`'s star face, written from `c`, is walked against
        // its orientation.
        f.faces.push(f.face_of(0, 4).expect("the star face of d, from the Frame"));
        f.faces.push(f.face_of(3, 0).expect("the star face of c, oriented towards the Frame"));
        f.faces.push(f.face_carrying(3, 1, Word::identity()).expect("c to a"));
        f.faces.push(f.face_carrying(3, 2, Word::identity()).expect("c to b"));
        f.faces.push(
            f.face_carrying_with_edge(1, 2, Word::identity(), Some(Transport::from_word(order4())))
                .expect("a chord between two bodies"),
        );
        if on_the_cone {
            let one = num_rational::BigRational::from_integer(1.into());
            let null = Mv::scalar(one.clone()).add(&Mv::blade(E1 | EM, one));
            assert!(null.versor_inverse().is_none(), "MM̃ = 0: the carriage has no inverse");
            let [_, star_c, ..] = f.faces.as_mut_slice() else { panic!("five faces") };
            set_face_place(star_c, |p| p.carriage = null);
        }
        f
    };

    let flat = build(false);
    assert_eq!(
        Form::orbits_of(&flat.faces, &flat.vertices).len(),
        1,
        "with a readable carriage the chord closes one orbit"
    );

    let cone = build(true);
    Form::reset_orbit_gates();
    let orbits = Form::orbits_of(&cone.faces, &cone.vertices);
    assert!(orbits.is_empty(), "an orbit based through an unread transport is not an orbit: {orbits:?}");
    let (_, _, unread, trivial) = Form::orbit_gates();
    assert_eq!(unread, 2, "the refused tree edge and the cycle beyond it are both counted unread");
    assert_eq!(trivial, 0, "and nothing was read as flat");
}

/// 54.7 AXIOM A — A CYCLE IS BASED, AND BASING MOVES NO READING.
///
/// `Ad_θ` is an isometry of the algebra, so conjugation by the tree
/// transport preserves every grade projection and therefore both Casimirs:
///
/// ```text
///     ⟨(θΩθ⁻¹)²⟩₀ = ⟨θΩ²θ⁻¹⟩₀ = ⟨Ω²⟩₀   ⟹   I₁(ρ) = I₁(Ω)
/// ```
///
/// So the defect a cycle carries is the SAME defect read at the basepoint.
/// If basing changed a classification, the conjugation would be wrong — an
/// inverted orientation or an unnormalised versor in the tree — and this
/// is the assertion that would catch it.
#[test]
fn basing_a_cycle_conjugates_it_and_preserves_both_casimirs() {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let f = edge_chord_form(std::slice::from_ref(&w));
    let [lp] = f.orbits.as_slice() else { panic!("one chord closes one orbit") };

    // THE TRANSPORT IS A VERSOR, whatever it is — an unnormalised entry in
    // the tree is exactly the failure mode this guards.
    let theta = &lp.based_at;
    assert!(
        crate::transducer::cga::is_versor(theta).is_some(),
        "θ is a versor of Spin⁺(3,1)(ℚ), or the conjugation is not an isometry"
    );

    // AND CONJUGATION BY IT MOVES NEITHER CASIMIR. Read on the orbit's own
    // carriages, which is what `paths_carried` composed.
    let inv = theta.versor_inverse().expect("a versor inverts");
    for (_, omega) in lp.carriages.scoped() {
        let based = theta.geo(omega).geo(&inv);
        assert_eq!(
            based.class_invariants(),
            omega.class_invariants(),
            "Ad_θ preserves (I₁, I₂, kind): basing reads, it does not change"
        );
    }
}

/// 54.7 AXIOM A — RE-ROOTING IS A PURE GAUGE TRANSFORMATION.
///
/// The sharp test. Two cycles of one form are based at their own nodes;
/// carrying both to the basepoint must be conjugation by ONE versor per
/// cycle, and the based operators must then live on one tangent space —
/// which is what makes `ρ(γ₁)·ρ(γ₂)` mean anything at all.
///
/// Concretely: `θ` is the identity exactly where the cycle is based at the
/// root, and every other cycle's `θ` is a genuine transport. A form whose
/// cycles all carried the identity would be one where the basing never
/// fired, and that is indistinguishable from Axiom A being absent unless
/// it is asserted.
#[test]
fn every_cycle_carries_a_transport_to_the_forms_basepoint() {
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let h = crate::transducer::alphabet::LetterId::quarter((0, 3)).expect("quarter on another plane");
    let f = edge_chord_form(&[
        Word::from_letters(&[(q, false)]),
        Word::from_letters(&[(h, false)]),
    ]);
    assert!(!f.orbits.is_empty(), "the fixture must close, or nothing is tested");
    for lp in &f.orbits {
        // EVERY cycle has a transport, and it composes: `θ · Ω · θ⁻¹` is a
        // versor of the same group, so the based operators multiply.
        let theta = &lp.based_at;
        let inv = theta
            .versor_inverse()
            .expect("a tree transport inverts, or the cycle cannot be based");
        for (_, omega) in lp.carriages.scoped() {
            let based = theta.geo(omega).geo(&inv);
            assert!(
                crate::transducer::cga::is_versor(&based).is_some(),
                "a based cycle is an operator of Spin⁺(3,1)(ℚ)"
            );
        }
    }
    // AND TWO BASED CYCLES COMPOSE. Before Axiom A this product was
    // meaningless — two operators on different tangent spaces — so the
    // assertion is that it is now a versor at all.
    let based: Vec<element::Versor> = f
        .orbits
        .iter()
        .filter_map(|lp| {
            let theta = &lp.based_at;
            let inv = theta.versor_inverse()?;
            let (_, omega) = lp.carriages.scoped().next()?;
            Some(theta.geo(omega).geo(&inv))
        })
        .collect();
    if let [g1, g2, ..] = based.as_slice() {
        let word = g1.geo(g2);
        assert!(
            crate::transducer::cga::is_versor(&word).is_some(),
            "γ₁γ₂ is an operator of the group: words compose at the basepoint"
        );
        // THE HOMOMORPHISM, exactly: ρ(γ₁γ₂) = ρ(γ₁)·ρ(γ₂) is the geometric
        // product and nothing else, so composition is associative and the
        // free group's multiplication is realised.
        let g3 = g1.geo(g2).geo(g1);
        assert!(
            g3.sub(&g1.geo(&g2.geo(g1))).is_zero(),
            "composition is associative under the geometric product"
        );
    }
}

/// The crystal both pivot files hand the orbit: the words a mind holds on
/// an axis. It lives here with the tests that exercise the orbit; `pivot.rs`
/// reads it across the split.
pub(super) struct Hold(pub Vec<Word>);
impl crate::transducer::family::Crystal for Hold {
    fn on_class(
        &self,
        _class: &stalk::Invariant,
    ) -> Vec<(
        &Element,
        &Form,
        Standing,
    )> {
        Vec::new()
    }
    fn on_body(&self, c: &stalk::Invariant) -> Vec<(&Element, &Form, Standing)> {
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
    fn by_address(&self, _a: &[u8]) -> Option<(&Element, &Form)> {
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
fn pivot_orbit_discharges_held_word_and_vortices_a_pair() {
    let q = order4();
    let qinv = q.inverse();
    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_orbit(
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
        "the orbit's finite order left torsion and an axis opened"
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
        crate::transducer::sentence::from_element_form(&e).expect("the address decodes");
    // THE CERTIFICATE RIDES THE ENVELOPE (54.5 Road (d), B3). It read
    // `tampered.certificates` — the DECODED form's — and a form recovered
    // from its address carries none: a certificate is what one mind's
    // stroke did against one budget, which does not factor through the
    // geometry, so `evidence_is_not_identity` keeps it out of the key.
    // ONE CERTIFICATE, and the slice pattern says so rather than taking the
    // first of several (rule 15).
    let [cert] = e.receipts().certificates.as_slice() else {
        panic!("one certificate, on the word that made it")
    };
    let fi = cert.by.face() as usize;
    tampered.faces[fi].word = q.clone();
    let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
    // TAMPERING A CERTIFIED FACE NAMES A DIFFERENT WORD (54.5, as ruled
    // 2026-09-20), and that is the stronger property.
    //
    // This expected `REFUSE_ORBIT` — the tamper "re-opens an UNFILED orbit" —
    // which is the permit reading: the orbit was refused for lacking an
    // entry rather than for its geometry. Coherence no longer asks for a
    // filing, so a tampered face whose orbit is still a compact resonance
    // still coheres, and the court says so honestly.
    //
    // What the forger cannot do is keep the word's NAME. A face's word is
    // in the key (A21), so moving it moves the address: the tampered form
    // is a different word, and the certificate it carries is for the one it
    // is no longer.
    assert_ne!(
        bad.address(),
        e.address(),
        "a face's word is in the key, so tampering with it names another word"
    );

    let mut f = chord_form(std::slice::from_ref(&q));
    let snaps = f.pivot_orbit(
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
    assert!(!f.vortex.is_empty(), "the form holds the vortex candidates");

    // A19 ON A CLASS THE COURT HAS RULED: another mind can supply the word,
    // so this one neither demands it nor announces the filing. It still
    // files — the class is the form's own carrier — and it does so in
    // silence, which is what `Standing::Ruled` buys.
    let mut f = chord_form(std::slice::from_ref(&q));
    let mut horizon = Keyed::new();
    let r = q.rotor();
    horizon.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Ruled));
    let snaps = f.pivot_orbit(
        [0x33; 32],
        &horizon,
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    assert!(snaps.is_empty(), "a court-ruled class files silently: {snaps:?}");
    assert!(
        q.rotor().holonomy_key().is_some_and(|k| f.torsion.holds(&k)),
        "filed torsion is keyed by the class"
    );

    // AND THE SAME CLASS AT `Held` DOES NOT SUPPRESS IT. `Standing::Held` is
    // "an idea this mind is holding, not yet a word" — the mind's OWN
    // obstruction — and `hold_form` paints every standing orbit's own class
    // onto the horizon at that standing through `paint_standing`. Read as
    // presence alone this made every orbit defer to itself: no demand, no
    // announced filing, and no widening, all three closed by an entry the
    // mind wrote about its own defect. Measured at 7e73a1d — `held true` on
    // 108 of 108 orbits against zero `horizon … seeded` lines in two hours.
    let mut f = chord_form(std::slice::from_ref(&q));
    let mut mine = Keyed::new();
    let r = q.rotor();
    mine.insert(r.class_key().expect("q class"), (r, q.order(), Standing::Held));
    let snaps = f.pivot_orbit(
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
    let chord = f.orbits[0].chord;
    let snaps = f.pivot_orbit(
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
    let chord = f.orbits[0].chord as usize;
    set_face_place(&mut f.faces[chord], |p| {
        p.carriage = element::carriage(&q1(), &(q1(), q0()));
    });
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // The orbit the fixture wrote, by its chord — a trivial cycle no longer
    // stands in `orbits`, so position is not identity (rule 15).
    let at = f.orbit_of_chord(chord as u16).expect("the chord's orbit");
    let before = f.class_vector(at).expect("orbit");
    assert!(before.occupies(Axis::Placement));
    let _ = f.pivot_orbit(
        [0xb4; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(at).expect("orbit remains");
    assert!(!after.occupies(Axis::Placement), "Placement zeroes");
    assert!(
        !after.rises_against(&before, Axis::Placement),
        "and no other coordinate rose (law 10)"
    );

    // T14: Orientation.
    let mut f = chord_form(&[order2()]);
    for fa in &mut f.faces {
        set_face_place(fa, |p| p.gauge.clear());
    }
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // The orbit by its chord, and the chord read by destructuring rather
    // than by position: one orbit stands here and the pattern says so.
    let [only, ..] = f.orbits.as_slice() else { panic!("one orbit stands") };
    let at = f.orbit_of_chord(only.chord).expect("its index");
    let before = f.class_vector(at).expect("ori");
    assert!(before.occupies(Axis::Orientation(0)), "got {:?}", before.coordinates());
    let _ = f.pivot_orbit(
        [0xb5; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(at).expect("ori after");
    assert!(
        !after.occupies(Axis::Orientation(0)),
        "Orientation zeroes, got {:?}",
        after.coordinates()
    );
    assert!(!after.rises_against(&before, Axis::Orientation(0)));

    // T14: Dilation.
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        set_face_place(fa, |p| p.gauge.clear());
    }
    let chord = f.orbits[0].chord as usize;
    let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
    set_face_place(&mut f.faces[chord], |p| {
        p.carriage = element::carriage(&two, &(q0(), q0()));
    });
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let at = f.orbit_of_chord(chord as u16).expect("the chord's orbit");
    let before = f.class_vector(at).expect("dil");
    assert!(
        before.occupies(Axis::Dilation),
        "a dilator residue occupies Dilation, got {:?}",
        before.coordinates()
    );
    let _ = f.pivot_orbit(
        [0xb6; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    let after = f.class_vector(at).expect("dil after");
    assert!(!after.occupies(Axis::Dilation), "Dilation zeroes, got {:?}", after.coordinates());
    assert!(!after.rises_against(&before, Axis::Dilation));
}

/// A16 — AN ORBIT'S READINGS ARE A FUNCTION OF ITS FACES AND THE FORM'S AXIS
/// CARRIER, and of nothing else the form holds. That is why a trial reads
/// its own answer without copying a form for every candidate: it writes the
/// one face the move names, walks the orbits those faces close into, and
/// reads the class and the row off them.
///
/// The two entry points must therefore answer alike on a live form — the
/// methods, which read the form's own faces and axes, and the free readers
/// the trial uses. A trial that answered differently from the form it
/// trials would certify a move the form does not make.
#[test]
fn an_orbits_class_and_row_read_off_its_faces_and_the_axis_carrier() {
    use crate::transducer::sentence::pivot::drive::class_vectors_on;
    use crate::transducer::sentence::{orbit_row_on, opened_axis_on};
    let f = chord_form(&[order4(), order2(), Word::identity()]);
    assert!(f.orbits.len() >= 3, "three chords close three orbits: {}", f.orbits.len());
    for (i, lp) in f.orbits.iter().enumerate() {
        // EVERY MEMBER PATH, compared path for path: the trial and the form
        // read the same paths and the same vector on each, so a trial that
        // agreed on one path and differed on another is caught.
        let cvs_a = class_vectors_on(&f.faces, &f.vertices, &f.axes, lp);
        let cvs_b = f.class_vectors(i);
        assert_eq!(
            cvs_a.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
            cvs_b.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
            "the classes the trial reads are the classes the form reads, orbit {i}"
        );
        assert_eq!(
            orbit_row_on(&f.faces, &f.vertices, &f.axes, lp),
            f.orbit_row_at(i),
            "and so is the rank row, orbit {i}"
        );
        assert_eq!(
            opened_axis_on(&f.faces, &f.vertices, &f.axes, lp),
            f.opened_axis_of(lp),
            "and the axis it reads as opened, orbit {i}"
        );
    }
    // AND WITH AN AXIS OPEN. The carrier is what the readings consult, so
    // a trial holding its own copy of it reads the opened orbit as opened.
    let mut opened = f.clone();
    let [lp, ..] = f.orbits.as_slice() else { panic!("three chords close three orbits") };
    let chord = lp.chord;
    if opened.open_axis_on(chord).is_some() {
        for (i, lp) in opened.orbits.iter().enumerate() {
            let cvs_a = class_vectors_on(&opened.faces, &opened.vertices, &opened.axes, lp);
            let cvs_b = opened.class_vectors(i);
            assert_eq!(
                cvs_a.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>(),
                cvs_b.scoped().map(|(k, v)| (k, v.class_key())).collect::<Vec<_>>()
            );
            assert_eq!(
                orbit_row_on(&opened.faces, &opened.vertices, &opened.axes, lp),
                opened.orbit_row_at(i)
            );
        }
    }
}

/// THE ORBIT IS FILED UNDER THE CLASS IT IS JUDGED BY. A face's word is the
/// coboundary `w_j ∘ w_i⁻¹`, and a product of coboundaries around a cycle
/// telescopes to the identity for every input, so an orbit over coboundary
/// faces is flat on the word whatever its bodies do. Its EDGE composites
/// (50.3) do not telescope — `edge_holonomies_on` says so in its own doc —
/// and they are where a cover's curvature lives.
///
/// `class_vector` has read the edge composite since Y5. The filing decision
/// did not: `pivot_orbit` took `lp.holonomy` for the horizon key and for the
/// order that selects the `Discharge` branch, and `file_orbit` filed under
/// `lp.holonomy.rotor().class_key()`. An orbit was judged by one object and
/// filed under another, and on the fleet at e98e8a2 that meant 70311 orbits
/// flat on the word, 284 finite edge composites over the same orbits, and
/// zero torsion filed.
///
/// This fixture is exactly that disagreement: three coboundary faces, so
/// the word holonomy is the identity, and a quarter turn on the one face
/// that does not touch the Frame.
#[test]
fn an_orbit_flat_on_the_word_files_under_its_edge_composite() {
    use crate::transducer::alphabet::Derivation;
    use crate::transducer::stalk::Invariant;
    let chart = Invariant::counts(&[3, 5]);
    let pair = face_pair_key(&chart, &chart);
    let q = order4();
    // The ends are two SITES: one chart each, so they are two vertices and
    // not one named twice (54.2). The faces carry their own pair key and
    // their own edges, so the holonomy this test reads is unmoved.
    let word_vertex = |b: u8| {
        Vertex::word_on(
            Derivation::spelled(Word::identity()),
            Invariant::counts(&[u64::from(b)]),
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
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    assert_eq!(f.orbits.len(), 1, "three faces on three vertices close one orbit");
    assert!(
        f.orbits[0].holonomy.rotor().is_identity(),
        "the word holonomy is flat, as a product of coboundaries always is"
    );

    let hw = f.holonomy_word(0).expect("the paths agree on one holonomy");
    // The orbit runs the chord backward, so the composite is `q⁻¹`. It is the
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
    // 54.7 — THE YIELD IS BOTH PLANES. `file_orbit` returns the labour the
    // filing earns, and that is `(ℓ₁ − 1) + (ℓ₂ − 1)` across the two
    // invariant planes of the double rotation, not `lcm − 1`. A quarter
    // turn is isoclinic here — `[4, 4]` — so it earns `3 + 3 = 6`, where
    // the envelope said `4`. The two planes are doing the work; the
    // envelope was never either plane's.
    assert_eq!(f.file_orbit(0), vec![6], "a path with a class files, at its own spectrum");
    let edge_class = q.rotor().holonomy_key().expect("the quarter turn has a class");
    let flat_class = crate::transducer::alphabet::Rotor::identity()
        .holonomy_key()
        .expect("the identity has a class");
    assert!(
        f.torsion.holds(&edge_class),
        "filed under the class the orbit is judged by"
    );
    assert!(
        !f.torsion.holds(&flat_class),
        "and not under the identity the word holonomy reports"
    );

    // D2: `Orbit.carriages` IS the transport composite's carriage, path for
    // path. `path_carriages` computed it in a second walk — the same
    // frontier, the same steps, the same closing rule, with the word and the
    // gauge discarded — so the walk ran once at `orbits_of` and again at every
    // class reading. One carrier now; this asserts they were the same thing.
    let lp = &f.orbits[0];
    let transport = orbit_transport(&f, lp);
    assert_eq!(
        lp.carriages.len(),
        transport.len(),
        "one carriage per closed member path, and the same paths"
    );
    for (k, t) in transport.scoped() {
        let held = lp.carriages.get(&k).expect("the orbit carries this path");
        assert!(
            element::same_versor(held, &t.carriage),
            "the stored carriage is the transport composite's"
        );
    }
}

/// Y2 (16.16, 12.6, 3.9, 25.1): A held orbit with a held word on that axis discharges
/// and certifies on a constructed family.
#[test]
fn a_held_orbit_with_a_held_word_discharges_and_certifies() {
    let q = order4();
    let mut f = chord_form(std::slice::from_ref(&q));
    let family = vec![(vec![vec![0u8; 4]; 4], vec![vec![0u8; 4]; 4])];
    let snaps = f.pivot_orbit(
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
        "the orbit's finite order left torsion and an axis opened"
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

/// Y2 (16.16, 12.6, 3.9, 25.1): Two candidate words discharging the orbit are held as a vortex
/// on a constructed family.
#[test]
fn two_certified_discharges_are_held_as_a_vortex() {
    let q = order4();
    let qinv = q.inverse();
    let mut f = chord_form(std::slice::from_ref(&q));
    let family = vec![(vec![vec![0u8; 4]; 4], vec![vec![0u8; 4]; 4])];
    let snaps = f.pivot_orbit(
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
    assert!(!f.vortex.is_empty(), "the form holds the vortex candidates");
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

/// A21 §2, R6 — THE BASE IS THE RANK, AND ITS WIDTH DOES NOT GROW WITH THE
/// FORM. π: P → B, so a form's chart is five coordinates whatever it holds
/// and the orbits inhabit the fiber over it.
///
/// The chart appended one packed coordinate per distinct orbit class. Key
/// width grew with the orbit count, every `Keyed` entry naming the chart
/// copied that width (R6), and `on_axis_of` re-entered the trie at the root
/// once per coordinate it coarsened through — `n·d` descents to answer one
/// axis, over an index whose own type is the bundle.
#[test]
fn a_forms_base_chart_is_five_coordinates_however_many_orbits_it_holds() {
    use crate::transducer::sentence::chart_of_form;
    let many = chord_form(&[order4(), order2(), Word::identity()]);
    assert!(many.orbits.len() >= 3, "a form carrying several orbits: {}", many.orbits.len());
    let one = chord_form(&[order4()]);
    let base = chart_of_form(&many).expect("a measured rank is a base point");
    assert_eq!(base.0.len(), 5, "the tag and the four rank coordinates, and nothing else");
    assert!(
        base.0.iter().all(|a| matches!(a, crate::transducer::stalk::ChartAxis::Count(_))),
        "every one of them a count: {:?}",
        base.0
    );
    if let Some(single) = chart_of_form(&one) {
        assert_eq!(
            single.0.len(),
            base.0.len(),
            "and a form with fewer orbits is charted at the same width, not a shorter key"
        );
    }
    // `peel` reaches the root in five steps, bounded by the base and never
    // by the orbit count: that is the coarsening walk `on_axis_of` makes.
    let mut at = base.clone();
    let mut steps = 0usize;
    while !at.0.is_empty() {
        at = at.peel();
        steps += 1;
    }
    assert_eq!(steps, 5, "five strata from the base to the root");
}

/// LAW 10 — A COORDINATE MAY FALL. The discharge condition is the target
/// summand zeroing "with no other rising"; `agrees_except` demanded that
/// no other coordinate MOVE — it cleared the target from both sides and
/// compared the rest for equality. Its own doc line said "a coordinate may
/// fall to zero", so the reader and its description disagreed and the
/// reader won.
///
/// A pivot writes a face carrier that every orbit through that face reads,
/// so a move zeroing its target routinely carries other coordinates down
/// with it. The first assertion here is exactly that move: admitted now,
/// refused before.
#[test]
fn a_falling_coordinate_is_not_a_rise() {
    use crate::transducer::sentence::{Axis, ClassVector};
    let of = |w: &Word| ClassVector::of_word(w).expect("a word carries a class");
    // Measured, not assumed: a quarter turn stands on `Unit(0)`, a half
    // turn on `Orientation(0)`, and the identity on nothing.
    let (four, two, flat) = (of(&order4()), of(&order2()), ClassVector::zero());
    assert_eq!(four.coordinates(), vec![Axis::Unit(0)]);
    assert_eq!(two.coordinates(), vec![Axis::Orientation(0)]);
    assert!(flat.coordinates().is_empty());

    // THE DISCRIMINATING CASE. Target `Orientation(0)`, and `Unit(0)` —
    // which is not the target — falls to zero. Nothing rose, so law 10
    // admits it; `agrees_except` compared the planes, found them unequal,
    // and refused.
    assert!(
        !flat.rises_against(&four, Axis::Orientation(0)),
        "a NON-TARGET coordinate falling to zero is not a rise: {:?} → {:?}",
        four.coordinates(),
        flat.coordinates()
    );

    // The target's own fall, with nothing beside it.
    assert!(!flat.rises_against(&four, Axis::Unit(0)), "the target zeroing is not a rise");

    // And the moves that DO rise, each on a coordinate that is not the
    // target: a unit count appearing, an orientation character appearing.
    assert!(
        four.rises_against(&flat, Axis::Orientation(0)),
        "a unit count appearing out of nothing is a rise"
    );
    assert!(
        two.rises_against(&flat, Axis::Unit(0)),
        "an orientation character appearing out of nothing is a rise"
    );
}
