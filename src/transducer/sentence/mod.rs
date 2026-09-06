// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! THE SENTENCE — one algebra, two directions.
//!
//! Speaking forms a sentence from geometry: cited words, each carried by a
//! versor and read at a gauge coordinate, and the residual no word speaks.
//! Comprehending runs the same algebra the other way. A received chain is
//! deconstructed into the words that compose it: each body reads its
//! moduli, the stratum over those coordinates names the word, the transport
//! is the quotient of the two bodies' moments, the gauge coordinate is the
//! assignment whose transition profile is the body's own, and the residual
//! is what remains once the transported word is carried onto the body —
//! exactly, on the sensor's own cells. A received sentence is verified by
//! replay: the citations are applied and what remains is measured.
//!
//! Nothing here enumerates the language. A stratum names one word when the
//! moduli are fine enough; when it holds several, that multiplicity is
//! returned as the measurement that the coordinates want an axis. Entity,
//! hypha, and court run these same functions; they differ only in the
//! direction they speak.

pub mod chain;
pub mod form;
pub mod pivot;
pub mod tape;
pub mod chart;

pub use chain::*;
pub use form::*;
pub use pivot::*;
pub use tape::*;
pub use chart::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::nerve::{Keyed, ROOT};
    use crate::transducer::alphabet::{self, Order, Word};
    use crate::transducer::blade::Shape;
    use crate::transducer::element::{self, Element, from_residual, GERM_DIM};
    use crate::transducer::germinate::germinate;
    use crate::transducer::stalk::{self, Standing, Stalks};

    fn q0() -> num_rational::BigRational {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(0))
    }
    fn q1() -> num_rational::BigRational {
        num_rational::BigRational::from_integer(num_bigint::BigInt::from(1))
    }

    fn closed_word_form(word: Word, contact: Option<alphabet::Contact>) -> Form {
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [7u8; 32],
            word: word.clone(),
            rho: q1(),
            placement: (q0(), q1()),
            gauge: vec![(1, 2)],
            class: stalk::Invariant(vec![3, 5]),
            contact,
        });
        f.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: f.face_of(0, 1).expect("star"),
        });
        f.remainder.clear();
        f.depth = f.depth_of();
        f
    }

    /// SENTENCE_TAG v3 round-trips Frame, Word (with contact), Glued,
    /// Rebased, Unfolded, faces, and loops.
    #[test]
    fn form_tape_v2_round_trips() {
        let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let inner = closed_word_form(w.clone(), None);
        assert!(inner.is_closed(), "a tree of two vertices coheres");
        assert!(as_element_form(&inner, [0; 4]).is_none(), "zero descent is not spoken");
        let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
        let inner_addr = inner_e.address();

        let mut f = closed_word_form(
            w.clone(),
            Some(alphabet::Contact { anchor: 1, ring: 2 }),
        );
        let glued = f
            .glue(&inner, inner_addr, Word::identity(), [0, 0, 0, 1])
            .expect("glue depth 0 into depth 0");
        let rebased = f.vertices.len() as u16;
        f.vertices.push(Vertex::Rebased {
            by: 0,
            of: 1,
            shift: (q0(), q1()),
            rho: q1(),
            gauge: vec![(1, 2)],
            class: stalk::Invariant(vec![3, 5]),
        });
        let unfolded = f.unfold(Measure::Occupied);
        f.faces.push(FormFace::new(1, glued, w.clone(),));
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.remainder = vec![(9, 9, 1, 0)];
        assert!(matches!(f.vertices[glued as usize], Vertex::Glued { .. }));
        assert!(matches!(
            f.vertices[rebased as usize],
            Vertex::Rebased { by: 0, of: 1, .. }
        ));
        assert!(matches!(
            f.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        ));

        let e = as_element_form(&f, [1, 0, 0, 0]).expect("v3 tape");
        assert!(e.is_sentence());
        assert_eq!(e.description()[1], 3, "version 3");
        let back = from_element_form(&e).expect("decode v3");
        assert_eq!(back.vertices.len(), f.vertices.len());
        assert_eq!(back.faces.len(), f.faces.len());
        assert_eq!(back.loops.len(), f.loops.len());
        assert_eq!(back.remainder, f.remainder);
        assert_eq!(back.depth, f.depth);
        match (&f.vertices[1], &back.vertices[1]) {
            (
                Vertex::Word { word: a, gauge: ga, class: ca, contact: ca_c, .. },
                Vertex::Word { word: b, gauge: gb, class: cb, contact: cb_c, .. },
            ) => {
                assert_eq!(ca_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
                assert_eq!(cb_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
                assert_eq!(a, b, "M1 is the rotor, contact is beside it");
                assert_eq!(ga, gb);
                assert_eq!(ca, cb);
            }
            _ => panic!("word vertex"),
        }
        match (&f.vertices[glued as usize], &back.vertices[glued as usize]) {
            (Vertex::Glued { address: a, depth: da, .. }, Vertex::Glued { address: b, depth: db, .. }) => {
                assert_eq!(a, b);
                assert_eq!(da, db);
            }
            _ => panic!("glued vertex"),
        }
        match &back.vertices[rebased as usize] {
            Vertex::Rebased {
                by,
                of,
                shift,
                rho,
                gauge,
                class,
            } => {
                assert_eq!(*by, 0);
                assert_eq!(*of, 1);
                assert_eq!(shift, &(q0(), q1()));
                assert_eq!(rho, &q1());
                assert_eq!(gauge, &vec![(1, 2)]);
                assert_eq!(class, &stalk::Invariant(vec![3, 5]));
            }
            _ => panic!("rebased vertex"),
        }
        assert_eq!(
            back.vertices[unfolded as usize],
            Vertex::Unfolded {
                measure: Measure::Occupied
            }
        );
        assert_eq!(back.faces[0].i, 0);
        assert_eq!(back.faces[0].j, 1);
        assert_eq!(back.faces[0].transport.word, f.faces[0].transport.word);
        let e_plain = as_element_form(&closed_word_form(w.clone(), None), [1, 0, 0, 0]).expect("plain");
        let e_contact = as_element_form(
            &closed_word_form(
                w.clone(),
                Some(alphabet::Contact { anchor: 0, ring: 2 }),
            ),
            [1, 0, 0, 0],
        )
        .expect("contact");
        assert_ne!(
            e_plain.address(),
            e_contact.address(),
            "contact is on the form key"
        );
        assert_eq!(w, w, "M1 is the rotor; contact is not a letter");
    }

    #[test]
    fn face_of_a_missing_vertex_is_none() {
        let f = Form::default();
        assert!(
            f.face_of(0, 5).is_none(),
            "a missing vertex is not the identity word"
        );
        assert!(f.face_of(0, 0).is_some(), "the frame is present");
    }

    #[test]
    fn tape_v3_refuses_short_or_partial() {
        let f = closed_word_form(Word::identity(), None);
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("v3");
        assert_eq!(e.description()[1], 3);
        let mut short = e.description().to_vec();
        short.pop();
        let mut truncated = Element::identity(GERM_DIM);
        truncated.set_description(short);
        truncated.set_magnitude([1, 0, 0, 0]);
        assert!(
            from_element_form(&truncated).is_none(),
            "a short v3 tape is refused"
        );
        let mut v2 = e.description().to_vec();
        v2[1] = 2;
        let mut old = Element::identity(GERM_DIM);
        old.set_description(v2);
        old.set_magnitude([1, 0, 0, 0]);
        assert!(
            from_element_form(&old).is_none(),
            "a v2 tape is refused"
        );
    }

    #[test]
    fn placement_residue_is_the_affine_translation() {
        let mut f = chord_form(&[Word::identity()]);
        for fa in &mut f.faces {
            fa.transport.gauge.clear();
        }
        let chord = f.loops[0].chord as usize;
        f.faces[chord].transport.shift = (q1(), q0());
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        let kappa = f.class_vector(0).expect("loop");
        assert!(
            kappa.occupies(Axis::Placement),
            "a nonzero composed shift occupies Placement"
        );
        assert_ne!(
            kappa.placement,
            (q0(), q0()),
            "the residue is the composed translation, got {:?}",
            kappa.placement
        );
        let snaps = f.pivot_loop(
            [0xa4; 32],
            &Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Placement)),
            "the residue discharges on Placement: {snaps:?}"
        );
        let after = f.class_vector(0).expect("loop remains");
        assert!(
            !after.occupies(Axis::Placement),
            "Placement is zero after the pivot"
        );
    }

    #[test]
    fn hue_cycle_type_composes_around_the_loop() {
        let mut f = chord_form(&[Word::identity()]);
        for fa in &mut f.faces {
            fa.transport.gauge.clear();
        }
        let chord = f.loops[0].chord as usize;
        f.faces[chord].transport.gauge = vec![(3, 4), (4, 3)];
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        let kappa = f.class_vector(0).expect("loop");
        assert!(
            kappa.occupies(Axis::Hue),
            "a non-identity composed gauge occupies Hue, got {:?}",
            kappa.hue
        );
        let snaps = f.pivot_loop(
            [0xa5; 32],
            &Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Hue)),
            "the cycle type discharges on Hue: {snaps:?}"
        );
        let after = f.class_vector(0).expect("loop remains");
        assert!(!after.occupies(Axis::Hue), "Hue is identity after the pivot");
    }

    #[test]
    fn conjugate_hue_permutations_are_one_cycle_type() {
        let a = CycleType::of(&[(1, 2), (2, 1)]);
        let b = CycleType::of(&[(3, 4), (4, 3)]);
        assert_eq!(a, b, "cycle type is sorted lengths, not the listing");
        assert_eq!(a, CycleType::Type(vec![2]));
        let z = (q0(), q0());
        let ka = ClassVector::read(&Word::identity(), z.clone(), a);
        let kb = ClassVector::read(&Word::identity(), z, b);
        assert!(ka.occupies(Axis::Hue) && kb.occupies(Axis::Hue));
        assert_eq!(
            ka.class_key(),
            kb.class_key(),
            "(1 2) and (3 4) are one Hue coordinate"
        );
        assert_ne!(ka.class_key(), CLOSED_KEY, "a hue cycle is not closed");
    }

    #[test]
    fn coboundary_gauge_matches_by_address() {
        let q = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
        let mut a = Form::default();
        a.vertices.push(word_vertex(1, q.clone()));
        a.vertices.push(word_vertex(2, Word::identity()));
        a.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: a.face_of(0, 1).expect("a star 1"),
        });
        a.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: a.face_of(0, 2).expect("a star 2"),
        });
        a.reindex();
        let mut b = Form::default();
        b.vertices.push(word_vertex(2, Word::identity()));
        b.vertices.push(word_vertex(1, q.clone()));
        b.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: b.face_of(0, 1).expect("b star 1"),
        });
        b.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: b.face_of(0, 2).expect("b star 2"),
        });
        b.reindex();
        assert!(
            coboundary_gauge(&a, &b).is_some(),
            "the same words in a different vertex order are one coboundary"
        );
    }

    #[test]
    fn solve_coboundary_is_a_spanning_forest() {
        let mut f = Form::default();
        f.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: Word::identity(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        f.vertices.push(Vertex::Word {
            address: [2u8; 32],
            word: Word::identity(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        f.faces.push(FormFace::new(0, 1, Word::identity(),));
        let gamma = f.solve_coboundary().expect("an isolated vertex roots its own tree");
        assert_eq!(gamma.len(), 3);
        assert!(gamma[0].is_identity(), "the least index of a component is γ = 1");
        assert!(gamma[2].is_identity(), "vertex 2 is its own root");
        f.faces.push(FormFace::new(0, 9, Word::identity(),));
        assert!(
            f.solve_coboundary().is_none(),
            "a face that names a vertex outside the form refuses"
        );
        assert!(f.regauge(&[Word::identity()]).is_none(), "a short gauge refuses");
    }

    #[test]
    fn glue_retract_remaps_faces_and_placement_refuses() {
        let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
        let mut host = Form::default();
        host.vertices.push(Vertex::Word {
            address: [1u8; 32],
            word: w.clone(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        host.vertices.push(Vertex::Word {
            address: [2u8; 32],
            word: w.clone(),
            rho: q1(),
            placement: (q0(), q1()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        });
        host.faces.push(FormFace { i: 0, j: 1, transport: host.face_of(0, 1).expect("0-1"), });
        host.faces.push(FormFace { i: 1, j: 2, transport: host.face_of(1, 2).expect("1-2"), });
        host.faces.push(FormFace { i: 0, j: 2, transport: host.face_of(0, 2).expect("0-2"), });
        let inner = closed_word_form(w.clone(), None);
        let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner");
        let gi = host
            .glue(&inner, inner_e.address(), w.clone(), [1, 0, 0, 0])
            .expect("glue");
        assert_eq!(host.glued.get(&inner_e.address()).copied(), Some(gi));
        assert_eq!(
            host.faces.last().map(|f| (f.i, f.j)),
            Some((0, gi)),
            "glue writes the Frame→Glued placement face"
        );
        let mut drop = Keyed::new();
        drop.insert(vertex_ix_key(1), ());
        host.retract_vertices(&drop);
        assert_eq!(host.vertices.len(), 3, "Frame, remaining Word, Glued");
        for face in &host.faces {
            assert_eq!(
                host.face_of(face.i, face.j).as_ref(),
                Some(&face.transport),
                "every surviving face re-derives"
            );
        }
        let mut taped = host.clone();
        let mag = [1u64, 0, 0, 0];
        let clean = as_element_form(&taped, mag).expect("tape");
        assert_eq!(crate::court::verdict::blind_replay(&clean), Some(crate::court::verdict::REFUSE_GLUED_UNRULED));
        let resolve = |a: &[u8; 32]| (a == &inner_e.address()).then(|| inner.clone());
        assert_eq!(crate::court::verdict::blind_replay_with(&clean, &resolve), None);
        let Some(fi) = taped.faces.iter().position(|f| f.i == 0 && matches!(taped.vertices.get(f.j as usize), Some(Vertex::Glued { .. }))) else {
            panic!("placement face");
        };
        taped.faces[fi].transport.word = Word::identity();
        let bad = as_element_form(&taped, mag).expect("tampered placement");
        assert_eq!(
            crate::court::verdict::blind_replay_with(&bad, &resolve),
            Some(crate::court::verdict::REFUSE_FACE),
            "a tampered placement face refuses"
        );
    }

    /// The bundle's agreed shift is meaning: two translations of one linear
    /// block are two vertices.
    #[test]
    fn two_shifts_are_two_addresses() {
        let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(letter, false)]);
        let mut a = closed_word_form(w.clone(), None);
        let mut b = closed_word_form(w, None);
        if let Vertex::Word { placement, .. } = &mut a.vertices[1] {
            *placement = (q0(), q1());
        }
        if let Vertex::Word { placement, .. } = &mut b.vertices[1] {
            *placement = (
                q0(),
                num_rational::BigRational::from_integer(num_bigint::BigInt::from(3)),
            );
        }
        let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
        let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
        assert_ne!(ea.address(), eb.address(), "shift is not quotiented off the vertex");
        assert!(!ea.same_vertex(&eb));
    }

    /// The bundle's frame ratio and its sections are meaning: they are on
    /// the form key and on the tape, and a tape with a zero denominator is
    /// refused rather than repaired.
    #[test]
    fn ratio_and_sections_are_on_the_form_key() {
        let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(letter, false)]);
        let a = closed_word_form(w.clone(), None);
        let mut b = closed_word_form(w.clone(), None);
        b.ratio = ((2, 1), (2, 1));
        let mut c = closed_word_form(w, None);
        c.sections.insert([7u8; 32], vec![1]);
        let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
        let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
        let ec = as_element_form(&c, [1, 0, 0, 0]).expect("c");
        assert_ne!(ea.address(), eb.address(), "the frame ratio is on the key");
        assert_ne!(ea.address(), ec.address(), "a section is on the key");
        let back_b = from_element_form(&eb).expect("v2 tape");
        assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
        let back_c = from_element_form(&ec).expect("v2 tape");
        assert_eq!(back_c.sections.get(&[7u8; 32]), Some(&vec![1u16]));
        // Zero the height denominator on the tape: after certificates the
        // tail is ratio (8) + sections count (2) + torsion count (2). hd is
        // the second u16 of the ratio.
        let mut bytes = ea.description().to_vec();
        let n = bytes.len();
        bytes[n - 10] = 0;
        bytes[n - 9] = 0;
        let mut broken = ea.clone();
        broken.set_description(bytes);
        assert!(from_element_form(&broken).is_none(), "a zero denominator refuses the tape");
    }

    /// Depth is one plus the deepest glued form; gluing at equal depth is refused.
    #[test]
    fn depth_is_one_plus_max_glued() {
        let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
        let w = Word::from_letters(&[(q, false)]);
        let inner = closed_word_form(w.clone(), None);
        assert_eq!(inner.depth_of(), 0);
        let mut outer = Form::default();
        let a0 = [1u8; 32];
        outer.glue(&inner, a0, Word::identity(), [0, 0, 0, 1]).expect("glue into empty host");
        assert_eq!(outer.depth_of(), 1);
        assert_eq!(outer.depth, 1);
        let mut wrap = Form::default();
        wrap.glue(&outer, [2u8; 32], Word::identity(), [0, 0, 0, 1]).expect("glue depth 1 into depth 0");
        assert_eq!(wrap.depth_of(), 2);
        let outer_copy = outer.clone();
        assert!(
            outer.glue(&outer_copy, [3u8; 32], Word::identity(), [0, 0, 0, 1]).is_none(),
            "equal depth is refused"
        );
        assert!(
            outer.glue(&wrap, [4u8; 32], Word::identity(), [0, 0, 0, 1]).is_none(),
            "deeper form is refused"
        );
        let mut again = Form::default();
        again.glue(&inner, a0, Word::identity(), [0, 0, 0, 1]).unwrap();
        again.glue(&inner, [5u8; 32], Word::identity(), [0, 0, 0, 1]).expect("another depth-0 glue");
        assert_eq!(again.depth_of(), 1, "max glued depth, not a count");
    }

    /// A language of a bar and a dot deconstructs a rotated, translated,
    /// and dilated bar through its moduli with a verified transport, speaks
    /// a chain of two bodies as two words with their relative transport,
    /// leaves the unknown body as the residual, and refuses to cite a word
    /// with itself.
    #[test]
    fn a_chain_deconstructs_into_its_words_and_residual() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
        let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
        let words = vec![bar, dot, bar_other];
        let atlas = Stalks::over_elements(words.iter());
        let mut index = crate::network::nerve::Keyed::new();
        for (i, w) in words.iter().enumerate() {
            index.insert(w.address(), i);
        }
        let index_of = |a: &[u8; 32]| index.get(a).copied();
        let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
        let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
        // A vertical bar far away: the bar rotated and translated.
        let target = vec![(5, 5, 1, 2), (6, 5, 1, 2)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&target, None);
        assert_eq!(s.cites.len(), 1);
        assert_eq!(s.cites[0].0, 0, "the bar, not the other-hued bar");
        assert!(s.is_closed());
        assert!(replay(&words, &s.cites, &target).is_empty(), "the citation replays to closure");
        assert!(
            !s.multiplicity.is_empty(),
            "stratum width is recorded per body"
        );
        // A 2× dilated bar with the other hues: the other bar at ρ = 2.
        let big = vec![(0, 0, 7, 8), (0, 1, 7, 8), (0, 2, 7, 8), (0, 3, 7, 8), (1, 0, 7, 8), (1, 1, 7, 8), (1, 2, 7, 8), (1, 3, 7, 8)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&big, None);
        assert_eq!(s.cites.len(), 1);
        assert_eq!(s.cites[0].0, 2);
        assert!(s.is_closed());
        // Two bodies and an unknown: two citations with one relative
        // transport between them, one residual body.
        let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,&mixed, None);
        assert_eq!(s.cites.len(), 2);
        assert_eq!(
            s.multiplicity.len(),
            4,
            "the chain and each of bar, dot, residual have a width, never one maximum"
        );
        assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
        assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
        // Excluding the bar itself: a bar chain has no other word of hue 1→2.
        let s = deconstruct(&words, &atlas, &index_of, &shape_at,words[0].sub(), Some(0));
        assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
    }

    #[test]
    fn deconstruct_multiplicity_is_per_body() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
        let words = vec![bar, dot];
        let atlas = Stalks::over_elements(words.iter());
        let mut index = Keyed::new();
        for (i, w) in words.iter().enumerate() {
            index.insert(w.address(), i);
        }
        let index_of = |a: &[u8; 32]| index.get(a).copied();
        let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
        let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
        let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4)];
        let s = deconstruct(&words, &atlas, &index_of, &shape_at, &mixed, None);
        assert_eq!(s.cites.len(), 2);
        assert_eq!(
            s.multiplicity.len(),
            3,
            "the chain and each cited body have a width"
        );
        let widths: Vec<usize> = s
            .multiplicity
            .fiber(&ROOT)
            .map(|(_, w)| *w)
            .collect();
        assert_eq!(
            widths.iter().filter(|&&w| w >= 1).count(),
            2,
            "each cited body keeps its own stratum width; the chain records 0 when it is not a word: {widths:?}"
        );
    }

    /// The receiving direction: two citations carrying different words
    /// onto the same cells are a seam defect the replay alone would miss;
    /// a transport and its inverse are a flat loop, two translations are
    /// not.
    #[test]
    fn seams_and_holonomy_are_located_exactly() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
        let words = vec![bar, bar_other];
        let target = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
        let id = element::identity_versor();
        let cites: Vec<Cite> = vec![(0, id.clone(), 0), (1, id.clone(), 0)];
        assert!(replay(&words, &cites[..1], &target).is_empty());
        assert_eq!(seam_defect(&words, &cites, &target), target, "both cells spoken twice, differently");
        assert_eq!(verify(&words, &cites, &target), target);
        assert!(verify(&words, &cites[..1], &target).is_empty(), "one word, no seam");
        let t = element::translator_versor(3, 0);
        assert!(is_flat(&[t.clone(), t.versor_inverse().unwrap()]));
        assert!(!is_flat(&[t.clone(), element::translator_versor(0, 2)]));
        assert!(is_flat(&[]), "the empty loop is flat");
    }

    fn word_vertex(addr: u8, word: Word) -> Vertex {
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        Vertex::Word {
            address: [addr; 32],
            word,
            rho: one,
            placement: (zero.clone(), zero),
            gauge: vec![(1, 2)],
            class: stalk::Invariant::default(),
            contact: None,
        }
    }

    fn star_form(n_words: usize, extra: Vec<(u16, u16, Word)>) -> Form {
        let mut f = Form::default();
        for i in 0..n_words {
            f.vertices.push(word_vertex(i as u8 + 1, Word::identity()));
            let j = (i as u16) + 1;
            f.faces.push(FormFace {
                i: 0,
                j,
                transport: f.face_of(0, j).expect("star"),
            });
        }
        for (i, j, w) in extra {
            f.faces.push(FormFace::new(i, j, w));
        }
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        f
    }

    /// Two word vertices and the frame star are a tree; the coboundary is exact.
    #[test]
    fn a_two_germ_nerve_is_a_tree_and_closes() {
        let f = star_form(2, Vec::new());
        assert_eq!(f.vertices.len(), 3, "frame plus two words");
        assert!(!f.faces.is_empty(), "the frame star has faces");
        assert!(f.loops.is_empty(), "a tree has no independent cycle");
        assert!(f.solve_coboundary().is_some(), "a tree 1-cochain is exact");
        assert!(f.coheres(), "a tree coheres");
    }

    #[test]
    fn separated_bodies_meet_by_relative_word() {
        let f = star_form(2, Vec::new());
        let word_word = f.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
        assert_eq!(word_word, 0, "separated bodies have no adjacency face");
    }

    #[test]
    fn a_row_of_three_has_faces_by_constancy() {
        let tree = star_form(3, Vec::new());
        assert!(tree.faces.len() <= 2 * (tree.vertices.len() - 1));
        let touching = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity())]);
        let adj = touching.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
        assert_eq!(adj, 2, "two adjacency faces on a touching row, got {adj}");
        assert!(!touching.loops.is_empty(), "the 1-skeleton is not a star tree");
    }

    #[test]
    fn a_sentence_chart_leads_with_axis_two() {
        let f = star_form(2, Vec::new());
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
        assert!(e.is_sentence());
        let inv = stalk::invariant_of(&e);
        assert_eq!(inv.0.first().copied(), Some(2), "leading axis 2");
        assert!(inv.0.get(1).copied().unwrap_or(0) >= 2, "vertex count");
    }

    #[test]
    fn a_sentence_round_trips_its_nerve() {
        let f = star_form(2, Vec::new());
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
        assert!(e.is_sentence());
        let back = from_element_form(&e).expect("the form survives the tape");
        assert_eq!(back.vertices.len(), f.vertices.len());
        assert_eq!(back.faces.len(), f.faces.len());
        assert_eq!(back.loops.len(), f.loops.len());
    }

    #[test]
    fn face_constancy_is_the_word_not_the_versor() {
        let a = star_form(2, vec![(1, 2, Word::identity())]);
        let b = star_form(2, vec![(1, 2, Word::identity())]);
        let words1: Vec<_> = a.faces.iter().map(|fa| fa.transport.word.clone()).collect();
        let words2: Vec<_> = b.faces.iter().map(|fa| fa.transport.word.clone()).collect();
        assert_eq!(words1, words2, "constancy is the word");
        let _ = as_element_form(&a, [1, 0, 0, 0]).expect("rule element");
    }

    #[test]
    fn word_constant_triangle_keeps_its_cycle() {
        let f = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity()), (1, 3, Word::identity())]);
        assert!(!f.loops.is_empty(), "a triangle has a loop");
        assert!(f.loops.iter().all(|lp| lp.holonomy.is_identity()));
        let _ = as_element_form(&f, [1, 0, 0, 0]).expect("rule element");
    }

    #[test]
    fn placement_is_not_the_sentence_vertex() {
        let f = star_form(2, Vec::new());
        assert!(!f.faces.is_empty(), "the frame star relates the two words");
        let e1 = as_element_form(&f, [1, 0, 0, 0]).expect("e1");
        let e2 = as_element_form(&f, [1, 0, 0, 0]).expect("e2");
        assert!(e1.same_vertex(&e2));
        assert_eq!(e1.address(), e2.address());
    }

    #[test]
    fn infinite_class_is_never_filed() {
        let mut f = chord_form(&[infinite()]);
        let snaps = f.pivot_loop(
            [0x11; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "infinite is demanded: {snaps:?}"
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::TorsionFiled { .. })),
            "infinite is never filed: {snaps:?}"
        );
        assert!(
            f.torsion.is_empty(),
            "an infinite class is never a large-ℓ torsion cycle"
        );
    }

    #[test]
    fn torsion_filed_only_when_witnessed_at_period() {
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut t = from_residual(&crate::transducer::germinate::germinate(&a, &b).unwrap());
        t.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        assert_eq!(t.torse(), 1, "the hue cycle is one torsion generator");
        let ws = crate::transducer::toroid::witness_stored(&t);
        assert!(
            ws.iter().any(|(ell, w)| *ell == 3
                && matches!(w, crate::transducer::toroid::HolonomicLoopWitness::Closed { period: 3, .. })),
            "period 3 is witnessed closed: {ws:?}"
        );
        t.record_torsion(vec![(1, 9)], 8, crate::transducer::element::FIBER_SPATIAL);
        let ws = crate::transducer::toroid::witness_stored(&t);
        assert!(
            ws.iter().any(|(ell, w)| *ell == 8
                && matches!(w, crate::transducer::toroid::HolonomicLoopWitness::Open { .. })),
            "an unclosed walk is not believed: {ws:?}"
        );
    }

    fn order2() -> Word {
        alphabet::PlaneForm {
            parity: true,
            rot: [0; alphabet::CAYLEY_ALPHABET_LEN],
        }
        .to_word()
    }

    fn order4() -> Word {
        Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)])
    }

    fn infinite() -> Word {
        Word::from_letters(&[(alphabet::LetterId::pythagorean_5((0, 1)).unwrap(), false)])
    }

    fn chord_form(chords: &[Word]) -> Form {
        let mut f = Form::default();
        for (ci, w) in chords.iter().enumerate() {
            let a = (ci * 2 + 1) as u16;
            let b = (ci * 2 + 2) as u16;
            f.vertices.push(word_vertex(a as u8, Word::identity()));
            f.vertices.push(word_vertex(b as u8, Word::identity()));
            f.faces.push(FormFace {
                i: 0,
                j: a,
                transport: f.face_of(0, a).expect("star a"),
            });
            f.faces.push(FormFace {
                i: 0,
                j: b,
                transport: f.face_of(0, b).expect("star b"),
            });
            f.faces.push(FormFace::new(a, b, w.clone()));
        }
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        f
    }

    fn coboundary_chord_form() -> Form {
        let r = order4();
        let mut f = Form::default();
        f.vertices.push(word_vertex(1, Word::identity()));
        f.vertices.push(word_vertex(2, Word::identity()));
        f.faces.push(FormFace::new(0, 1, r.clone()));
        f.faces.push(FormFace::new(0, 2, Word::identity()));
        f.faces.push(FormFace::new(1, 2, r.inverse()));
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        f
    }

    #[test]
    fn pivot_candidates_are_read_from_occupied_axes_only() {
        let id = ClassVector::of_word(&Word::identity());
        assert!(id.occupied().is_empty() && id.is_zero());
        let k2 = ClassVector::of_word(&order2());
        let occ = k2.coordinates();
        assert_eq!(occ, vec![Axis::Sheet(0)], "sheet parity occupies Sheet(0), got {occ:?}");
        let inf = ClassVector::of_word(&infinite());
        assert!(
            inf.free_part(),
            "a pythagorean-5 holonomy is free"
        );
        let mut seen = occ.clone();
        seen.extend(inf.coordinates());
        let mut declaration = seen.clone();
        declaration.sort_by_key(|a| match a {
            Axis::Sheet(i) => (0, *i),
            Axis::Unit(i) => (1, *i),
            Axis::Prime5(i) => (2, *i),
            Axis::Prime13(i) => (3, *i),
            Axis::Hue => (4, 0),
            Axis::Placement => (5, 0),
            Axis::OffPlane => (6, 0),
            Axis::Open => (7, 0),
        });
        assert_eq!(seen, declaration, "occupied axes already in declaration order");
    }

    fn cert(axis: Axis, mu: [u64; 4]) -> PivotCertificate {
        PivotCertificate {
            chord: 0,
            axis,
            before: [0; 32],
            after: [0; 32],
            rank_before: Rank {
                torsion: 0,
                free: 0,
                axes: 0,
            },
            rank_after: Rank {
                torsion: 0,
                free: 0,
                axes: 0,
            },
            mu_before: [0; 4],
            mu_after: mu,
            by: PivotBy::Word {
                face: 0,
                word: Word::identity(),
                side: Side::Right,
            },
        }
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
        assert_eq!(won.mu_after, tight.mu_after, "the lane-wise minimum is the discharge");
        fn perm(m: [u64; 4]) -> [u64; 4] {
            [m[2], m[1], m[0], m[3]]
        }
        let mut lp = loose.clone();
        let mut tp = tight.clone();
        lp.mu_after = perm(lp.mu_after);
        tp.mu_after = perm(tp.mu_after);
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
            address: [1u8; 32],
            word: q.clone(),
            rho: q1(),
            placement: (q1() + q1() + q1(), q1() + q1() + q1()),
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
        let mut f = chord_form(&[q.clone()]);
        if let Vertex::Word {
            class: c,
            gauge,
            placement,
            ..
        } = &mut f.vertices[1]
        {
            *c = class;
            *gauge = vec![(1, 1)];
            *placement = (q1() + q1() + q1(), q1() + q1() + q1());
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
        assert_eq!(held[0].mu_before[2], 3, "T is the extent passed in");
        assert_eq!(held[0].mu_after[2], 3);
        assert_eq!(held[0].mu_after, held[1].mu_after, "equal μ⃗, neither dominates");
    }

    #[test]
    fn cauterize_filed_torsion_crystallizes() {
        let mut f = chord_form(&[order2()]);
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

    /// Chapter 23.1 step 4: a filed torsion chord replays; a chord that is
    /// not the holonomy refuses the face; a missing torsion entry refuses
    /// the loop.
    #[test]
    fn filed_torsion_chord_replays_or_refuses() {
        let mut f0 = chord_form(&[order2()]);
        let _ = f0.pivot_loop(
            [0xca; 32],
            &crate::network::nerve::Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(f0.is_cauterized(), "filed finite torsion with empty remainder cauterizes");
        let e = as_element_form(&f0, [1, 0, 0, 0]).expect("the filed form crystallizes");
        assert!(
            e.torsion().is_empty(),
            "v2 does not fill the element torsion section"
        );
        assert!(
            carries_torsion(&e),
            "the form carries the filed cycle"
        );
        let keyed = from_element_form(&e).expect("v2 tape");
        let (class, (_, _, chord)) = keyed
            .torsion
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .map(|(k, v)| (*k, v.clone()))
            .expect("filed class");
        assert_eq!(
            torsion_loop_rotor(&keyed, &class).map(|r| r.class_key()),
            Some(class),
            "the chord face is the keyed representative"
        );
        assert!(
            keyed.faces.get(chord as usize).is_some(),
            "the carrier stores a live chord face"
        );
        assert_eq!(
            crate::court::verdict::blind_replay(&e),
            None,
            "a filed torsion chord replays: the chord carries the holonomy"
        );
        let mut f = from_element_form(&e).expect("v2 tape");
        let mag = e.magnitude();
        let Some(ci) = f
            .faces
            .iter()
            .position(|face| f.face_of(face.i, face.j).as_ref() != Some(&face.transport))
        else {
            panic!("the filed form has a torsion chord");
        };
        f.faces[ci].transport.word = order4();
        let bad = as_element_form(&f, mag).expect("tampered chord");
        assert_eq!(
            crate::court::verdict::blind_replay(&bad),
            Some(crate::court::verdict::REFUSE_FACE),
            "a chord that does not carry the loop holonomy refuses the face"
        );
        let mut gone = from_element_form(&e).expect("v2 tape");
        gone.torsion = Keyed::new();
        let missing = as_element_form(&gone, mag).expect("torsion stripped");
        assert_eq!(
            crate::court::verdict::blind_replay(&missing),
            Some(crate::court::verdict::REFUSE_LOOP),
            "a filed chord without its torsion entry refuses the loop"
        );
    }

    #[test]
    fn horizon_seeded_infinite_is_not_demanded() {
        let mut f = chord_form(&[infinite()]);
        let r = infinite().rotor();
        let mut horizon = crate::network::nerve::Keyed::new();
        horizon.insert(r.class_key(), (r, Order::Infinite, Standing::Held));
        let snaps = f.pivot_loop(
            [0x5e; 32],
            &horizon,
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            !snaps.iter().any(|s| matches!(s, Snap::Demand { .. })),
            "a held infinite class is not spoken again: {snaps:?}"
        );
    }

    #[test]
    fn two_loops_of_one_class_are_both_filed_or_discharged() {
        let w = order2();
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
            let filed = f.torsion.fiber(&crate::network::nerve::ROOT).any(|(_, (_, ell, _))| *ell == o);
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
        fn by_address(&self, _a: &[u8; 32]) -> Option<(Element, Form)> {
            None
        }
        fn on_axis(&self, _chart: &stalk::Invariant, axis: Axis) -> Vec<Word> {
            self.0
                .iter()
                .filter(|w| ClassVector::of_word(w).occupies(axis))
                .cloned()
                .collect()
        }
    }

    #[test]
    fn pivot_loop_discharges_held_word_and_vortices_a_pair() {
        let q = order4();
        let qinv = q.inverse();
        let mut f = chord_form(&[q.clone()]);
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
        assert_eq!(certificate.before, ClassVector::of_word(&q).class_key());
        assert_eq!(certificate.after, CLOSED_KEY);
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
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .expect("one certificate")
            .1
            .by
            .face() as usize;
        tampered.faces[fi].transport.word = q.clone();
        let bad = as_element_form(&tampered, e.magnitude()).expect("tampered tape");
        assert_eq!(
            crate::court::verdict::blind_replay(&bad),
            Some(crate::court::verdict::REFUSE_FACE),
            "a word on a certified face that is not face_of of its vertices refuses"
        );

        let mut f = chord_form(&[q.clone()]);
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

        let mut f = chord_form(&[q.clone()]);
        let mut horizon = Keyed::new();
        let r = q.rotor();
        horizon.insert(r.class_key(), (r, q.order(), Standing::Held));
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
            f.torsion.contains(&q.rotor().class_key()),
            "filed torsion is keyed by the class"
        );
    }

    #[test]
    fn axis_index_admit_is_linear_on_axis_is_constant() {
        use crate::network::nerve::work;
        let inv = stalk::Invariant::default();
        let w = order2();
        let run = |n: usize| {
            work::reset();
            let mut index = AxisIndex::new();
            for _ in 0..n {
                admit_words(&mut index, &inv, std::slice::from_ref(&w));
            }
            let inserts = work::snapshot().1;
            work::reset();
            let _ = on_axis_of(&index, &inv, Axis::Sheet(0));
            let gets = work::snapshot().0;
            (inserts, gets)
        };
        let (i8, g8) = run(8);
        let (i32n, g32) = run(32);
        assert!(
            i8 > 0 && i32n >= i8 && i32n <= i8.saturating_mul(5),
            "4N admits cost ~4× N inserts: {i8} then {i32n}"
        );
        assert_eq!(g8, g32, "on_axis get cost is independent of N: {g8} vs {g32}");
    }

    #[test]
    fn holonomy_is_oriented() {
        let w = order4();
        let mut fwd = Form::default();
        fwd.vertices.push(word_vertex(1, Word::identity()));
        fwd.vertices.push(word_vertex(2, Word::identity()));
        fwd.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: fwd.face_of(0, 1).expect("star"),
        });
        fwd.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: fwd.face_of(0, 2).expect("star"),
        });
        fwd.faces.push(FormFace::new(1, 2, w.clone()));
        fwd.loops = Form::loops_of(&fwd.faces, fwd.vertices.len());
        let mut rev = Form::default();
        rev.vertices.push(word_vertex(1, Word::identity()));
        rev.vertices.push(word_vertex(2, Word::identity()));
        rev.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: rev.face_of(0, 1).expect("star"),
        });
        rev.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: rev.face_of(0, 2).expect("star"),
        });
        rev.faces.push(FormFace::new(2, 1, w.inverse()));
        rev.loops = Form::loops_of(&rev.faces, rev.vertices.len());
        assert_eq!(fwd.loops.len(), 1);
        assert_eq!(rev.loops.len(), 1);
        assert_eq!(
            fwd.loops[0].holonomy.class_key(),
            rev.loops[0].holonomy.class_key(),
            "a reversed chord stores the inverse word and the same class"
        );
    }

    #[test]
    fn chord_is_never_a_star_face() {
        let f = chord_form(&[order4(), order2()]);
        assert_eq!(f.loops.len(), 2, "two chords, two loops");
        for lp in &f.loops {
            let ch = &f.faces[lp.chord as usize];
            assert!(
                ch.i != 0 && ch.j != 0,
                "chord {} is a star face ({}, {})",
                lp.chord,
                ch.i,
                ch.j
            );
        }
    }

    #[test]
    fn pivot_writes_the_chord_transport_only() {
        let q = order4();
        let mut f = chord_form(&[q.clone()]);
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
    fn relative_gauge_composite_fixes_what_neither_fixes() {
        let t = vec![(1u8, 2), (2, 1)];
        assert!(
            compose_gauge(&t, &t).is_empty(),
            "two (1 2) transpositions compose to the identity"
        );
        let a = vec![(1u8, 2)];
        let b = vec![(2u8, 1)];
        let c = compose_gauge(&b, &a);
        assert!(
            c.iter().all(|&(h, k)| h != 1 || k != 1),
            "composite listing omits fixed points, got {c:?}"
        );
        assert_eq!(
            gauge_image(&gauge_keyed(&c), 1),
            1,
            "1→2 then 2→1 fixes 1, which neither map lists as fixed"
        );
    }

    #[test]
    fn offplane_word_refuses_to_act_on_a_translation() {
        let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 2)).unwrap(), false)]);
        assert!(w.plane_block().is_none(), "a (0,2) quarter leaves the sensor plane");
        let t = (q1(), q0());
        assert!(
            apply_linear(&w, &t).is_none(),
            "an off-plane word does not act as the identity on a translation"
        );
        assert!(
            Transport {
                word: w,
                shift: t,
                gauge: Vec::new(),
            }
            .inverse()
            .is_none(),
            "Transport::inverse refuses an off-plane word"
        );
    }

    /// A1: a (0,2) quarter to the fourth power is projectively identity and
    /// closed to coheres, pivot_loop, is_cauterized, and the court. The
    /// sensor-plane abelianisation does not see it.
    #[test]
    fn offplane_quarter_to_the_fourth_is_closed_to_all_four_readers() {
        let q = alphabet::LetterId::quarter((0, 2)).expect("quarter");
        let w = Word::from_letters(&[(q, false), (q, false), (q, false), (q, false)]);
        assert!(w.is_identity(), "Φ⁴ is projectively 1");
        let mut f = Form::default();
        let empty = |addr: u8| Vertex::Word {
            address: [addr; 32],
            word: Word::identity(),
            rho: q1(),
            placement: (q0(), q0()),
            gauge: Vec::new(),
            class: stalk::Invariant::default(),
            contact: None,
        };
        f.vertices.push(empty(1));
        f.vertices.push(empty(2));
        f.faces.push(FormFace {
            i: 0,
            j: 1,
            transport: f.face_of(0, 1).expect("star 1"),
        });
        f.faces.push(FormFace {
            i: 0,
            j: 2,
            transport: f.face_of(0, 2).expect("star 2"),
        });
        f.faces.push(FormFace::new(1, 2, w));
        f.loops = Form::loops_of(&f.faces, f.vertices.len());
        f.reindex();
        assert!(f.coheres(), "coheres reads projective identity");
        assert!(
            f.is_cauterized(),
            "is_cauterized reads projective identity"
        );
        assert!(
            f.loops.iter().all(|lp| lp.holonomy.is_identity()),
            "the chord holonomy is A1 identity"
        );
        let snaps = f.pivot_loop(
            [0xa1; 32],
            &Keyed::new(),
            &crate::transducer::family::NoCrystal,
            &mut crate::physics::Physics::default().stroke_n(8),
            None,
            0,
        );
        assert!(
            !snaps.iter().any(|s| matches!(
                s,
                Snap::Demand { .. } | Snap::TorsionOpen { .. } | Snap::TorsionFiled { .. }
            )),
            "pivot_loop does not demand a projective identity: {snaps:?}"
        );
        let e = as_element_form(&f, [1, 0, 0, 0]).expect("the identity chord crystallizes");
        assert_eq!(
            crate::court::verdict::blind_replay(&e),
            None,
            "the court reads projective identity"
        );
    }
}
