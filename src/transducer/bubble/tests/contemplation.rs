// 25 — the contemplation: what holding a form does to the complex, and
// what one thought costs whatever the mind already holds.

use crate::transducer::bubble::*;

#[test]
fn unmeasured_space_is_a_finite_boundary_norm() {
    let mut c = Contemplation::new();
    assert_eq!(c.boundary_norm(), 0, "no thought is zero, not MAX");
    c.hold(vec![(0, 0, 1, 2)]);
    let n = c.boundary_norm();
    assert!(n > 0 && n < usize::MAX, "∂g ≠ 0 is a finite cochain, got {n}");
}

#[test]
fn wrap_glues_crystallized_form() {
    use crate::transducer::element::{Element, FIBER_SPATIAL};
    let mut c = Contemplation::new();
    let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    c.hold(rem);
    let mut word = Element::identity();
    word.record_torsion(vec![(1, 2)], 2, FIBER_SPATIAL);
    let mut budget = crate::physics::Physics::default().stroke_n(32);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    assert!(
        c.glues_word(&word.address()),
        "a crystallized form is glued into the meeting idea"
    );
}

/// A form with Glued vertices and empty remainder is held as the form.
#[test]
fn hold_form_keeps_a_glued_open_loop() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{Form, FormFace, Vertex};
    let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let inner = {
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: crate::transducer::address::Address::from(vec![1]),
            word: w.clone(),
            carriage: Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
            gauge: vec![(1, 2)],
            class: crate::transducer::stalk::Invariant::default(),
            contact: None,
        });
        f.faces.push(FormFace { i: 0, j: 1, transport: f.face_of(0, 1).expect("vertices"), });
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f
    };
    let mut outer = Form::default();
    let _gi = outer
        .glue(&inner, crate::transducer::address::Address::from(vec![0xab]), Word::identity(), [1, 0, 0, 0])
        .expect("glue");
    assert!(outer.remainder.is_empty(), "no remainder to hold()");
    let mut c = Contemplation::new();
    c.hold_form(&outer, &[], &[]);
    assert_eq!(c.bubbles.len(), 1, "the glued form is an idea");
    let b = c.bubbles.fiber(ROOT).next().map(|(_, b)| b).expect("held");
    assert!(
        b.form.as_ref().is_some_and(|f| {
            f.vertices
                .iter()
                .any(|v| matches!(v, Vertex::Glued { .. }))
        }),
        "the bubble carries the Form"
    );
    let mut bytes = Vec::new();
    c.to_bytes(&mut bytes);
    let mut pos = 0usize;
    let back = Contemplation::from_bytes(&bytes, &mut pos).expect("FRM1");
    assert_eq!(back.bubbles.len(), 1, "the glued form survives the tape");

    let mut open = Form::default();
    open.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from(vec![2]),
        word: Word::identity(),
        carriage: Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    open.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from(vec![3]),
        word: Word::identity(),
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    open.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: open.face_of(0, 1).expect("star 1"),
    });
    open.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: open.face_of(0, 2).expect("star 2"),
    });
    open.faces.push(FormFace::new(1, 2, w.clone()));
    open.loops = Form::loops_of(&open.faces, open.vertices.len());
    assert!(
        open.loops.iter().any(|lp| !lp.holonomy.is_identity()),
        "the held form has an open loop"
    );
    let mut c2 = Contemplation::new();
    c2.hold_form(&open, &[], &[]);
    let k = c2.bubbles.fiber(ROOT).next().map(|(k, _)| k).expect("held");
    assert_eq!(k.as_slice(), crate::transducer::bubble::idea::idea_key(&open).as_bytes(), "hold, merge and Bubble::key share idea_key");
    struct HoldQ(Word);
    impl crate::transducer::family::Crystal for HoldQ {
        fn on_class(
            &self,
            _c: &crate::transducer::stalk::Invariant,
        ) -> Vec<(
            crate::transducer::element::Element,
            Form,
            crate::transducer::stalk::Standing,
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
        fn by_address(&self, _a: &[u8]) -> Option<(crate::transducer::element::Element, Form)> {
            None
        }
        fn on_axis(
            &self,
            _c: &crate::transducer::stalk::Invariant,
            axis: crate::transducer::sentence::Axis,
        ) -> Vec<Word> {
            let v = crate::transducer::sentence::ClassVector::of_word(&self.0);
            if v.is_some_and(|v| v.occupies(axis)) {
                vec![self.0.clone()]
            } else {
                Vec::new()
            }
        }
    }
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    if let Some(b) = c2.bubbles.get_mut(&k) {
        let atlas = Stalks::default();
        let _ = b.expand_with(&atlas, &Keyed::new(), 0, &HoldQ(w.clone()), &mut budget);
    }
    let b = c2.bubbles.get(&k).expect("still held");
    let form = b.form.as_ref().expect("form carrier");
    let discharged = !form.certificates.is_empty();
    let filed = !form.torsion.is_empty();
    assert!(
        discharged || filed,
        "the held form discharges or files, certificates {} torsion {}",
        form.certificates.len(),
        form.torsion.len()
    );
    let mag = crate::transducer::bubble::idea::form_mag(form);
    if mag.iter().any(|&x| x > 0) && form.is_cauterized() {
        let e = sentence::as_element_form(form, mag).expect("spoken tape");
        assert!(
            crate::transducer::sentence::carries_torsion(&e) || !form.certificates.is_empty(),
            "the spoken tape carries the certificate or the filed cycle"
        );
    }
}

#[test]
fn hold_form_carries_bundle_and_remainder() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::family::pair_idx_key;
    use crate::transducer::sentence::{Form, FormFace, Vertex};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: crate::transducer::address::Address::from(vec![9]),
        word: Word::identity(),
        carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
        gauge: vec![(1, 2)],
        class: crate::transducer::stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: f.face_of(0, 1).expect("star"),
    });
    let train = vec![(vec![vec![1u8]], vec![vec![2u8]])];
    let rem = vec![vec![(0i64, 0i64, 1u8, 2u8)]];
    let mut c = Contemplation::new();
    c.hold_form(&f, &train, &rem);
    assert_eq!(c.bubbles.len(), 1);
    let (k, b) = c.bubbles.fiber(ROOT).next().expect("held");
    assert_eq!(k.as_slice(), crate::transducer::bubble::idea::idea_key(&f).as_bytes(), "idea_key stays the form's vertex key");
    assert!(
        b.bundle.as_ref().is_some_and(|t| t.len() == 1),
        "the idea carries the train"
    );
    let cells = b.remainders.get(&pair_idx_key(0)).expect("pair remainder");
    assert_eq!(cells.len(), 1, "remainder is keyed by pair");
    assert!(
        b.remainder.is_empty(),
        "Bubble.remainder is the form's own remainder, not concatenated pairs"
    );
    assert_eq!(
        b.volume,
        crate::transducer::element::measure_lanes(cells),
        "volume is the lane-wise sum of per-pair remainder measures"
    );
}

/// Hold, expand, and the quote-chart peel of one thought do not grow
/// with N unrelated atlas words.
#[test]
fn work_of_one_thought_is_independent_of_n() {
    use crate::network::nerve::work;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;
    let run = |n: usize| -> (u64, u64, u64) {
        let mut atlas = Stalks::default();
        for i in 2..n + 2 {
            let before: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 1u8)).collect();
            let after: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 2u8)).collect();
            let w = from_residual(&germinate(&before, &after).unwrap());
            atlas.admit(&stalk::invariant_of(&w), w.address());
        }
        let mut c = Contemplation::new();
        work::reset();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        let hold = work::total();
        work::reset();
        let mut budget = crate::physics::Physics::default().stroke_n(4);
        let _ = c.expand_all(&atlas, &Keyed::new(), &crate::transducer::family::NoCrystal, &mut budget);
        let expand = work::total();
        work::reset();
        let mut charts = 0usize;
        for (_, b) in c.bubbles.fiber(ROOT) {
            let mut cur = stalk::invariant(&b.remainder);
            loop {
                if b.cover.contains(&cur.key()) {
                    charts += 1;
                }
                if cur.0.is_empty() {
                    break;
                }
                cur = cur.peel();
            }
        }
        let quote = work::total();
        assert!(charts > 0);
        (hold, expand, quote)
    };
    let a = run(32);
    let b = run(128);
    eprintln!("work thought N=32 {a:?} N=128 {b:?}");
    assert_eq!(a, b, "4N differs by zero: {a:?} vs {b:?}");
}
