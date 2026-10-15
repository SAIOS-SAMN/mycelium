// 25 — the contemplation: what holding a form does to the complex, and
// what one thought costs whatever the mind already holds.

use crate::transducer::bubble::*;

/// Law 5, L7a: the boundary of the open thoughts is read on four lanes
/// and is the measure of the held remainders themselves — never their sum,
/// never a norm. No thought is `[0; 4]`; one held link measures as its
/// chain does, lane by lane.
#[test]
fn an_open_thought_measures_on_four_lanes_never_a_sum() {
    let mut c = Contemplation::new();
    assert_eq!(c.boundary_measure(), [0; 4], "no thought is zero on every lane, not MAX");
    let link = (0, 0, 1, 2);
    c.hold(vec![link]);
    let m = c.boundary_measure();
    assert_eq!(
        m,
        element::measure_lanes(&[link]),
        "the boundary is the remainder's own lane measure, lane for lane"
    );
    assert!(m.iter().any(|&x| x > 0), "∂g ≠ 0 is a non-zero cochain: {m:?}");
}

/// P9 — AN OPEN DEMAND IS AN IDEA THAT STANDS ON ITS OWN CHART. A demand
/// carries the chart its reading was typed at (50.5), and a chart is the
/// coordinate a bubble is filed under (`by_chart`). Held without it, an
/// idea whose form is a demand stood on nothing: `admit_incidence` wrote no
/// incidence and `link_new` found no neighbour, so nothing the mind held
/// could meet it. Measured before this on the one-body deformation P8
/// leaves standing: one bubble, cover 0.
#[test]
fn an_open_demand_is_held_as_an_idea_standing_on_its_chart() {
    use crate::transducer::sentence::{Demand, Form, Vertex};
    let bodies = vec![(0i64, 0i64, 1u8, 0u8), (0, 1, 1, 0)];
    let chart = crate::transducer::stalk::invariant(&bodies);
    // `Form::default()` holds the Frame.
    let mut f = Form::default();
    f.put_vertex(Vertex::demand(
        Demand::TopologicalDeformation {
            image: crate::transducer::stalk::Invariant::default(),
            axes: Vec::new(),
        },
        chart.clone(),
        bodies.clone(),
    ))
    .expect("a demand has a coordinate");
    f.reindex();
    let mut c = Contemplation::new();
    c.hold_form(f.clone(), &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.len(), 1, "the demand is one idea the mind holds");
    let held: Vec<&Bubble> = c.bubbles.scoped().map(|(_, b)| b).collect();
    let [b] = held.as_slice() else { panic!("one bubble, got {}", held.len()) };
    assert!(
        b.cover.contains(chart.key().as_bytes()),
        "and it stands on the chart its demand was typed at"
    );
    // THE COORDINATE IS WHAT A MEET IS READ AT, so the claim is that the
    // chart reaches the idea back — not merely that a cover is non-empty.
    let key = super::super::idea::idea_key(&f);
    // THE CLAIM IS UNCHANGED — the chart names this idea — and the index
    // now names it by its ORDINAL (Z4 item 2), so the check resolves the
    // ordinal to the address rather than looking for the address in the
    // index, where a 7.3 MB key used to sit once per chart.
    let at_chart: Vec<Vec<u8>> = c
        .by_chart
        .get(chart.key().as_bytes())
        .expect("the chart names the ideas standing on it")
        .scoped()
        .filter_map(|(k, _)| Contemplation::ordinal_of_key(&k))
        .filter_map(|o| c.address_at(o).map(<[u8]>::to_vec))
        .collect();
    assert!(
        at_chart.iter().any(|k| k.as_slice() == key.as_bytes()),
        "the chart names this idea, so a meet at it can be read"
    );
    // It carries no word, so it contributes no section and no rotor: what
    // it contributes is the coordinate (rule 11, nothing is invented).
    assert!(b.sections().is_empty(), "a demand speaks no section");
    assert!(b.rotor().is_none(), "and no rotor stands in for the one it lacks");
}

/// P10 (1.5) — TWO PROJECTIVELY EQUAL CANDIDATES ARE ONE CANDIDATE, and
/// that is structural rather than a pass that runs. `candidates` is keyed
/// by `Rotor::key`, which writes `normalised()` — the ray's canonical
/// coefficients, scaled so the first non-zero one is positive — so `V` and
/// `−V` reach the same key and cannot both stand. A condensation pass over
/// this carrier would find nothing to condense; the carrier's own key does
/// it (rule 3, rule 6).
#[test]
fn a_candidate_and_its_negation_are_one_candidate() {
    use crate::transducer::alphabet::{Rotor, Q};
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    let v = Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b0101, q(1)));
    let minus_v = v.scale(&-Q::from_integer(num_bigint::BigInt::from(1)));
    assert!(v.projective_eq(&minus_v), "the two are one ray");
    assert_eq!(v.key(), minus_v.key(), "so they are one coordinate");
    let mut over_chart: Keyed<Rotor> = Keyed::new();
    over_chart.insert(v.key(), v.clone());
    over_chart.insert(minus_v.key(), minus_v);
    assert_eq!(over_chart.len(), 1, "and one candidate stands, not two");
    // A TRANSPORT ON ANOTHER PLANE IS ANOTHER CANDIDATE. The second rotor
    // differs from the first ONLY in which bivector plane it turns in —
    // same scalar, same weight — because that is the coordinate that has to
    // stay distinct. Separating them by their scalar instead would prove
    // nothing about the planes: `normalised` divides through by the leading
    // coefficient, so a scalar-led pair is told apart by a ratio and the
    // plane capacity the candidates exist to carry is never exercised.
    let other = Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b1001, q(1)));
    assert_ne!(other.key(), v.key(), "a turn in another plane keeps its own coordinate");
    assert!(!other.projective_eq(&v), "and the two are not one ray");
}

#[test]
fn wrap_glues_crystallized_form() {
    use crate::transducer::element::FIBER_SPATIAL;
    let mut c = Contemplation::new();
    let rem = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    c.hold(rem);
    // A chain-free word carrying an honest torsion cycle: a HOLONOMY, the
    // 90° turn, at its READ period 4 (PD1). The fixture filed σ = {1↦2}
    // at period 2, which permutes nothing and has no period — a claim the
    // record now refuses by name; a chain-free word's torsion is a
    // holonomy's, as the court's Congruent fill files it.
    let mut word = crate::transducer::element::Element::identity();
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).expect("the sensor plane");
    let turn_90 = crate::transducer::alphabet::Word::from_letters(&[(q, false)]).rotor();
    assert_eq!(
        word.record_cycle(crate::transducer::element::TorsionCycle {
            generator: crate::transducer::element::Generator::Holonomy(Box::new(turn_90)),
            period: 4,
            fibre: FIBER_SPATIAL,
            remainder: None,
        }),
        crate::transducer::element::Filed::New,
        "the 90° turn stands at its read period"
    );
    let mut budget = crate::physics::Physics::default().stroke_n(32);
    let _ = c.pivot_on(&word, &crate::transducer::family::NoCrystal, &mut budget);
    assert!(
        c.glues_word(&word.address()),
        "a crystallized form is glued into the meeting idea"
    );
}

/// A form with Glued vertices and empty remainder is held as the form.
#[test]
fn hold_form_keeps_a_glued_open_orbit() {
    use crate::transducer::alphabet::Word;
    use crate::transducer::sentence::{Form, Vertex};
    let q = crate::transducer::alphabet::LetterId::turn_90((0, 1)).unwrap();
    let w = Word::from_letters(&[(q, false)]);
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    use crate::transducer::sentence::tests::common::{chord, stand_with_star, FRAME};
    let inner = {
        let mut f = Form::default();
        let v = Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(w.clone()),
            crate::transducer::stalk::Invariant::default(),
            Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
            vec![(1, 2)],
            [FRAME],
            None,
        );
        stand_with_star(&mut f, &v);
        f.orbits = Form::orbits_of(&f.faces, &f.vertices);
        f
    };
    let mut outer = Form::default();
    let _gi = outer
        .glue(&inner, crate::transducer::address::Address::from(vec![0xab]), Word::identity(), [1, 0, 0, 0])
        .expect("glue");
    assert!(outer.remainder.is_empty(), "no remainder to hold()");
    let mut c = Contemplation::new();
    c.hold_form(outer.clone(), &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.bubbles.len(), 1, "the glued form is an idea");
    let b = c.bubbles.scoped().next().map(|(_, b)| b).expect("held");
    assert!(
        b.form.as_ref().is_some_and(|f| {
            f.vertices
                .scoped()
                .any(|(_, v)| matches!(v, Vertex::Glued { .. }))
        }),
        "the bubble carries the Form"
    );

    let mut open = Form::default();
    // Two members, two charts: one vertex — the identity's class (AT1, Q1)
    // — standing at each, each with its star, and a chord between them.
    let mut charts = Vec::new();
    for tag in [2u8, 3u8] {
        let v = Vertex::word_on(
            crate::transducer::alphabet::Derivation::spelled(Word::identity()),
            crate::transducer::stalk::Invariant::counts(&[u64::from(tag)]),
            Box::new(element::carriage(&one.clone(), &(zero.clone(), zero.clone()))),
            vec![(1, 2)],
            [FRAME],
            None,
        );
        charts.push(stand_with_star(&mut open, &v));
    }
    let [c2, c3] = charts.as_slice() else { panic!("two members stood") };
    chord(&mut open, c2, c3, w.clone());
    open.orbits = Form::orbits_of(&open.faces, &open.vertices);
    assert!(
        open.orbits.iter().any(|lp| {
            crate::transducer::sentence::orbit_word_on(&open.faces, &open.vertices, lp).is_some_and(|h| !h.is_identity())
        }),
        "the held form has an open orbit"
    );
    let mut c2 = Contemplation::new();
    c2.hold_form(open.clone(), &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    let k = c2.bubbles.scoped().next().map(|(k, _)| k).expect("held");
    assert_eq!(k.as_slice(), crate::transducer::bubble::idea::idea_key(&open).as_bytes(), "hold, merge and Bubble::key share idea_key");
    struct HoldQ(Word);
    impl crate::transducer::family::Crystal for HoldQ {
        fn on_class(
            &self,
            _c: &crate::transducer::stalk::Invariant,
        ) -> Vec<(
            &crate::transducer::element::Element,
            &Form,
            crate::transducer::stalk::Standing,
        )> {
            Vec::new()
        }
        fn on_body(
            &self,
            c: &crate::transducer::stalk::Invariant,
        ) -> Vec<(
            &crate::transducer::element::Element,
            &Form,
            crate::transducer::stalk::Standing,
        )> {
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
        fn by_address(&self, _a: &[u8]) -> Option<(&crate::transducer::element::Element, std::borrow::Cow<'_, Form>)> {
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
        /// One word on one axis: this fixture is the pivot's vocabulary,
        /// not a presentation to derive against.
        fn generators_under(
            &self,
            _c: &crate::transducer::stalk::Invariant,
            _budget: &mut crate::physics::Budget,
        ) -> Vec<(
            crate::transducer::address::Address,
            crate::transducer::address::Address,
            Word,
        )> {
            Vec::new()
        }
    }
    let mut budget = crate::physics::Physics::default().stroke_n(16);
    if let Some(b) = c2.bubbles.get_mut(&k) {
        let atlas = crate::transducer::stalk::Stalks::default();
        let _ = b.expand_with(&atlas, &|_| None, 0, &HoldQ(w.clone()), &mut budget, &mut Default::default());
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
    let mag = crate::transducer::bubble::idea::form_mag(form).expect("the form's measure reads");
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
    use crate::transducer::sentence::{Form, Vertex};
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let mut f = Form::default();
    let v = Vertex::word_on(
        crate::transducer::alphabet::Derivation::spelled(Word::identity()),
        crate::transducer::stalk::Invariant::default(),
        Box::new(element::carriage(&one, &(zero.clone(), zero))),
        vec![(1, 2)],
        [crate::transducer::sentence::tests::common::FRAME],
        None,
    );
    crate::transducer::sentence::tests::common::stand_with_star(&mut f, &v);
    let train = vec![(vec![vec![1u8]], vec![vec![2u8]])];
    let rem = vec![vec![(0i64, 0i64, 1u8, 2u8)]];
    let mut c = Contemplation::new();
    c.hold_form(f.clone(), &train, &rem, [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.bubbles.len(), 1);
    let (k, b) = c.bubbles.scoped().next().expect("held");
    assert_eq!(k.as_slice(), crate::transducer::bubble::idea::idea_key(&f).as_bytes(), "idea_key stays the form's vertex key");
    assert!(
        b.bundle.as_ref().is_some_and(|t| t.len() == 1),
        "the idea carries the train"
    );
    let links = b.remainders.get(&pair_idx_key(0)).expect("pair remainder");
    assert_eq!(links.len(), 1, "remainder is keyed by pair");
    assert!(
        b.remainder.is_empty(),
        "Bubble.remainder is the form's own remainder, not concatenated pairs"
    );
    assert_eq!(
        b.volume,
        crate::transducer::element::measure_lanes(links),
        "volume is the lane-wise sum of per-pair remainder measures"
    );
}

/// 25.2 — CAUTERIZE. An idea whose form is closed on its bundle
/// crystallizes on the stroke: the drive returns the minted sentence, its
/// magnitude is the trajectory the form measures over the bundle (26, the
/// same word `mint_sentence` mints), the tape reads back closed, the court
/// replays it against the bundle, and the idea leaves the contemplation.
#[test]
fn a_closed_idea_crystallizes_with_its_trajectory_and_leaves_the_contemplation() {
    use crate::transducer::family::{extract_rule, mint_sentence, Grid, NoCrystal};
    let grid = |rows: &[&str]| -> Grid {
        rows.iter()
            .map(|r| r.bytes().map(|b| b - b'0').collect())
            .collect()
    };
    let train_in = grid(&["11011", "11011", "00000", "11000", "11000"]);
    let train_out: Grid = train_in
        .iter()
        .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
        .collect();
    let family = vec![(train_in, train_out)];
    let rule = extract_rule(&family, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("the recolour family extracts");
    let minted = mint_sentence(&rule, &family, &NoCrystal, 0).expect("the family mints");
    let mut c = Contemplation::new();
    c.hold_form(rule.form.clone(), &family, &[Vec::new()], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.bubbles.len(), 1, "the closed form is held as an idea");
    let atlas = crate::transducer::stalk::Stalks::default();
    let mut budget = crate::physics::Physics::default().stroke_n(8);
    let crate::transducer::bubble::drive::Expanded { crystallized, .. } =
        c.expand_all(&atlas, &|_| None, &NoCrystal, &mut budget);
    let [(e, _)] = crystallized.as_slice() else {
        panic!("one idea crystallizes, got {}", crystallized.len());
    };
    assert!(e.is_sentence(), "the crystallized idea is a sentence tape");
    assert_eq!(e.address(), minted.address(), "the crystallized idea is the minted word");
    assert_eq!(e.magnitude(), minted.magnitude(), "its magnitude is the trajectory over the bundle");
    assert!(
        crate::transducer::sentence::from_element_form(e).is_some_and(|f| f.is_closed()),
        "the tape reads back closed"
    );
    assert_eq!(
        crate::court::verdict::replay_form_with(e, &|_| None, Some(&family), &NoCrystal, &mut crate::physics::Physics::default().stroke_n(1 << 20)),
        None,
        "the court replays it closed on its bundle"
    );
    assert!(c.bubbles.is_empty(), "the idea left the contemplation");
    assert_eq!(c.formed, 1, "one thought formed");
}

/// Hold, expand, and the quote-chart peel of one thought do not grow
/// with N unrelated atlas words.
#[test]
fn work_of_one_thought_is_independent_of_n() {
    use crate::network::nerve::work;
    use crate::transducer::element::from_canonical;
    use crate::transducer::germinate::germinate;
    let run = |n: usize| -> (u64, u64, u64) {
        let mut atlas = crate::transducer::stalk::Stalks::default();
        for i in 2..n + 2 {
            let before: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 1u8)).collect();
            let after: Vec<(i64, i64, u8)> = (0..i as i64).map(|c| (0, c, 2u8)).collect();
            let w = from_canonical(&germinate(&before, &after).unwrap());
            atlas.admit(&stalk::invariant_of(&w), w.address());
        }
        let mut c = Contemplation::new();
        work::reset();
        c.hold(vec![(0, 0, 1, 2), (0, 1, 1, 2)]);
        let hold = work::total();
        work::reset();
        let mut budget = crate::physics::Physics::default().stroke_n(4);
        let _ = c.expand_all(&atlas, &|_| None, &crate::transducer::family::NoCrystal, &mut budget);
        let expand = work::total();
        work::reset();
        let mut charts = 0usize;
        for (_, b) in c.bubbles.scoped() {
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

/// 45.7, S13.3 — THE DIRTY RING IS A CIRCUIT: three held thoughts, one
/// bubble per stroke under a budget of one read, and the fourth stroke
/// continues past the last bubble from the first and reports one circuit
/// with a period of four strokes — where the cursor that stopped at the
/// end of the ring left every later stroke expanding nothing.
#[test]
fn the_dirty_ring_circuits_and_reports_its_period() {
    use crate::transducer::bubble::drive::Expanded;
    use crate::transducer::family::NoCrystal;
    let mut c = Contemplation::new();
    // Three shapes on three rows: disjoint, so no two merge into one thought.
    c.hold(vec![(0, 0, 1, 2)]);
    c.hold(vec![(3, 0, 1, 2), (3, 1, 1, 2)]);
    c.hold(vec![(6, 0, 1, 2), (6, 1, 1, 2), (6, 2, 1, 2)]);
    assert_eq!(c.bubbles.len(), 3, "three thoughts held apart");
    assert_eq!(c.dirty.len(), 3, "three thoughts on the ring");
    let atlas = crate::transducer::stalk::Stalks::default();
    // The ring's cursor after each stroke. `None` is a ring that has not
    // started, and every reading here is after one stroke, so each is Some.
    let mut visited: Vec<Option<Vec<u8>>> = Vec::new();
    for stroke in 1..=3 {
        let mut budget = crate::physics::Physics::default().stroke_n(1);
        let Expanded { circuit, .. } = c.expand_all(&atlas, &|_| None, &NoCrystal, &mut budget);
        assert_eq!(circuit, None, "stroke {stroke}: three bubbles at one per stroke is no circuit yet");
        visited.push(c.expand_cursor.clone());
    }
    assert!(
        visited[0] != visited[1] && visited[1] != visited[2] && visited[0] != visited[2],
        "each stroke read a different bubble: {visited:?}"
    );
    assert_eq!(c.dirty.len(), 3, "a bubble the budget did not reach stays on the ring");
    let mut budget = crate::physics::Physics::default().stroke_n(1);
    let Expanded { circuit, .. } = c.expand_all(&atlas, &|_| None, &NoCrystal, &mut budget);
    assert_eq!(circuit, Some((1, 4)), "the fourth stroke passes the end of the ring: one circuit in four strokes");
    assert_eq!(c.expand_cursor, visited[0], "and continues from the first bubble");
    assert_eq!(c.ring.circuits(), 1);
}

/// P10(c), §25.12 — THE READING IS THE ORDER, NOT AN EQUALITY, and only an
/// INFINITE monodromy is a rupture.
///
/// This asserted `!carried.projective_eq(&v)` and called the non-identity
/// result the rupture. That is the flattened predicate: a connection with
/// torsion does not close on the first circuit, it closes at its PERIOD, so
/// a candidate returning after ℓ trips reads "not the identity" after one
/// and would be dissolved for closing. `Rotor::order` answers the three
/// readings exactly over ℚ, and they are three different facts.
#[test]
fn a_candidate_closes_flat_closes_at_a_period_or_ruptures() {
    use crate::transducer::alphabet::{Order, Rotor, Q};
    use crate::transducer::bubble::idea::monodromy;
    let q = |n: i64| Q::from_integer(num_bigint::BigInt::from(n));
    let v = Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(4)));

    // NO CONNECTION IS NO READING. The empty product is the identity by
    // ABSENCE, which is `Finite(1)` — and `dissolve_candidates` counts such
    // a chart neither for nor against (rule 11).
    let none = monodromy(&v, &[]).expect("the empty product is measured");
    assert_eq!(none.order(), Some(Order::Finite(1)), "nothing carried, nothing turned");

    // A 90° TURN CLOSES AT FOUR. Carried around a flat connection it is
    // itself, and its order is its period — not `1`, and not a rupture.
    let turn_90 = Rotor::blade(0b0011, q(1)).add(&Rotor::scalar(q(1)));
    let flat = monodromy(&turn_90, std::slice::from_ref(&Rotor::identity())).expect("measured");
    assert!(flat.projective_eq(&turn_90),"a flat connection returns the candidate");
    assert!(
        matches!(flat.order(), Some(Order::Finite(n)) if n >= 2),
        "and it closes at a period: {:?} — under an equality test this read as a rupture",
        flat.order()
    );

    // THE THREE READINGS ARE DISTINCT, which is what the dissolution reads.
    assert_eq!(Rotor::identity().order(), Some(Order::Finite(1)), "flat");
    let infinite = Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1)));
    assert_eq!(infinite.order(), Some(Order::Infinite), "and a rupture has no period");
    // Only the last dissolves: a candidate standing at ANY finite period on
    // ANY measured chart keeps its sub-cover and is held.
}

/// 52.5 READING 6 — THE FRAME KEY CARRIES THE ORDER. `frame_connections`
/// reads the frames a chart's fiber occupies from a `Keyed`, whose walk is
/// its key bytes. A frame written little-endian sorts by its low byte, so
/// 256 (`[0, 1, …]`) would read before 1 (`[1, 0, …]`) and the succession
/// would not be the family's. Written big-endian with the sign bit flipped,
/// byte order IS order in ℤ — negatives before zero before positives — so
/// the carrier walks the coordinate and nothing is sorted into it (rule 4).
#[test]
fn the_frame_key_orders_by_coordinate_and_not_by_byte() {
    let key = |k: i64| ((k as u64) ^ (1u64 << 63)).to_be_bytes();
    let mut at: Keyed<i64> = Keyed::new();
    for k in [256i64, 1, -3, 0, 2, -1] {
        at.insert(key(k), k);
    }
    let walked: Vec<i64> = at.scoped().map(|(_, v)| *v).collect();
    assert_eq!(walked, vec![-3, -1, 0, 1, 2, 256], "the trie walks ℤ in order");
    // Little-endian would not: 256 is [0, 1, ..] and reads before 1.
    let le = |k: i64| k.to_le_bytes();
    let mut wrong: Keyed<i64> = Keyed::new();
    for k in [256i64, 1] {
        wrong.insert(le(k), k);
    }
    let wrong_walk: Vec<i64> = wrong.scoped().map(|(_, v)| *v).collect();
    assert_eq!(wrong_walk, vec![256, 1], "which is why the key is not little-endian");
}

/// 25 DOOR 1 — A HOLD LAPSES WITH ITS DOCKET, AND ITS CITERS LAPSE WITH IT.
///
/// A bubble is pinned by `glues`, which is written at the hold and emptied
/// nowhere, so a pin outlives every reason for it. Measured on the 60-task
/// slice before this door was written: 21 of 51 held bubbles cite, all 21
/// pins are LIVE and none is dead — so a dead-pin collector retires none of
/// them and the cited address lapsing is the only signal that reaches them.
///
/// The three readings are the door. The cited bubble goes when its address
/// lapses. The citer goes WITH it, on the same call and not by a later
/// sweep that would hold a dead reference until it came round. And the
/// lapse is TRANSITIVE along the fibration: a citer of a citer goes too, in
/// one call, because the frontier is traversed until it adds nothing.
/// A form that stands on a chart and cites nothing.
fn standing_form(hue: u8) -> crate::transducer::sentence::Form {
    use crate::transducer::sentence::{Demand, Form, Vertex};
    // AT1: a demand's key carries its kind, chart, μ⃗ and piece count, never
    // its bodies, and the chart is the shape's — so two rows of one length
    // in two hues are ONE demand and one form. Each fixture's row is `hue`
    // bodies long, so the forms differ on a coordinate the key carries.
    let bodies: Vec<(i64, i64, u8, u8)> = (0..i64::from(hue)).map(|c| (0, c, hue, 0)).collect();
    let chart = crate::transducer::stalk::invariant(&bodies);
    let mut f = Form::default();
    f.put_vertex(Vertex::demand(
        Demand::TopologicalDeformation {
            image: crate::transducer::stalk::Invariant::default(),
            axes: Vec::new(),
        },
        chart,
        bodies,
    ))
    .expect("a demand has a coordinate");
    f.reindex();
    f
}

/// The same, plus a `Glued` vertex citing `cited` — which is what writes
/// `glues` at the hold and pins the bubble.
fn citing_form(hue: u8, cited: &crate::transducer::address::Address) -> crate::transducer::sentence::Form {
    use crate::transducer::sentence::Vertex;
    let mut f = standing_form(hue);
    let chart = crate::transducer::stalk::invariant(&[(0i64, 0i64, hue, 0u8)]);
    f.put_vertex(Vertex::Glued {
        chart: chart.clone(),
        magnitude: [1, 0, 0, 0],
        depth: 0,
        placement: crate::transducer::alphabet::Word::identity(),
        carriage: Box::new(crate::transducer::element::identity_versor()),
    })
    .expect("a glued vertex has a coordinate");
    // 54.7 B5a — AND THE INCIDENCE IT STANDS FOR. The vertex is the
    // boundary; `glues` at the hold is written off the cover.
    f.cover.push(crate::transducer::element::NerveEdge {
        target: cited.clone(),
        chart,
        magnitude: [1, 0, 0, 0],
        depth: 0,
        placement: crate::transducer::alphabet::Word::identity(),
        carriage: Box::new(crate::transducer::element::identity_versor()),
    });
    f.reindex();
    f
}

#[test]
fn a_hold_lapses_with_its_docket() {
    use crate::transducer::bubble::contemplation::Contemplation;
    let mut c = Contemplation::new();
    // Three bubbles: `inner` stands alone, `mid` cites it, `outer` cites
    // `mid`. Built by hand so the citation is the only relation between
    // them and nothing else can retire them.
    let inner = standing_form(7);
    let inner_key = crate::transducer::sentence::form_vertex_key(&inner);
    c.hold_form(inner, &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    let mid = citing_form(8, &inner_key);
    let mid_key = crate::transducer::sentence::form_vertex_key(&mid);
    c.hold_form(mid, &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    let outer = citing_form(9, &mid_key);
    let outer_key = crate::transducer::sentence::form_vertex_key(&outer);
    c.hold_form(outer, &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.bubbles.len(), 3, "three bubbles stand");
    assert!(
        c.bubbles.get(&mid_key).is_some_and(|b| !b.glues.is_empty()),
        "the middle bubble is pinned by its citation"
    );

    // The inner address lapses from the atlas. Everything pinned to it
    // goes on THIS call.
    let n = c.lapse_with(std::slice::from_ref(&inner_key.clone().into_vec()));
    assert_eq!(n, 3, "the cited bubble and both citers lapse together, transitively");
    assert_eq!(c.bubbles.len(), 0, "and none is left standing");
    for k in [&inner_key, &mid_key, &outer_key] {
        assert!(c.bubbles.get(k).is_none(), "retired: {:02x}{:02x}…", k[0], k[1]);
    }
    // An address nothing holds lapses nothing: the door is not a sweep.
    assert_eq!(c.lapse_with(&[vec![0xde, 0xad]]), 0, "an unheld address retires nothing");
}

/// 25 DOOR 2 — A PROMOTED THOUGHT IS NOT HELD TWICE.
///
/// A form ratified into the genome is held by the atlas. The bubble still
/// standing for it in the contemplation is a second carrier for one fact
/// (law 12), and it is retired AT the promotion rather than left for a
/// later sweep to notice.
///
/// The door goes through door 1's `lapse_with`, and the second reading is
/// why that is right rather than merely convenient: a promoted address has
/// left the contemplation's keeping, so whatever CITED it is pinned to a
/// bubble that no longer stands here and lapses on the same stroke.
#[test]
fn a_promoted_thought_is_not_held_twice() {
    use crate::transducer::bubble::contemplation::Contemplation;
    let mut c = Contemplation::new();
    let stood = standing_form(4);
    let stood_key = crate::transducer::sentence::form_vertex_key(&stood);
    c.hold_form(stood, &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    let citer = citing_form(5, &stood_key);
    c.hold_form(citer, &[], &[], [0; 4], &mut crate::physics::Physics::default().stroke_n(1 << 20));
    assert_eq!(c.bubbles.len(), 2, "the thought and a thought citing it");

    // It is ratified: the address is promoted into the genome.
    let n = c.lapse_with(std::slice::from_ref(&stood_key.clone().into_vec()));
    assert_eq!(n, 2, "the promoted thought and its citer both leave the contemplation");
    assert_eq!(c.bubbles.len(), 0, "nothing is held twice");
    assert_eq!(c.cavity_len(), 2, "and law 7 keeps both remainders");
}
