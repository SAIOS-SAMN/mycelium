// N4, RX1–RX7, 0.7 — PERCEPTIONS ARE NOT THOUGHTS: a held perception persists
// through the unglued door, and a thought is the transition that relates two
// of them — read from both sides — closing rings order by order.

use crate::chronometry::Cite;
use crate::transducer::blade::{self, Harmonic};
use crate::transducer::bubble::{ring_reading, Contemplation, RingOrder};
use crate::transducer::cga::Mv;
use crate::transducer::family::{perceive_family, scene_form_on, Grid, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

fn q(n: i64, d: i64) -> num_rational::BigRational {
    num_rational::BigRational::new(n.into(), d.into())
}

/// Perceive the scene `input → output` and hold it with its bodies, as a
/// voice does (`hold_perceived`): the frames perceived once, the stroke the
/// act's own.
fn perceive(c: &mut Contemplation, input: &[&str], output: &[&str]) -> Vec<u8> {
    let (a, b) = (grid(input), grid(output));
    let frames = perceive_family(&[(a.clone(), b.clone())]);
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let f = scene_form_on(&NoCrystal, &mut budget, &a, &b, &frames).expect("the scene reads");
    c.hold_perceived(f, &frames, &Vec::new(), &[], [0; 4], &mut budget).expect("held").as_bytes().to_vec()
}

/// Every held idea read through its first expansion: nothing is dirty.
fn expanded(c: &mut Contemplation) {
    let keys: Vec<Vec<u8>> = c.bubbles.scoped().map(|(k, _)| k).collect();
    for k in keys {
        if let Some(b) = c.bubbles.get_mut(&k) {
            b.dirty = false;
        }
    }
}

/// One L at three turns — 0°, 90°, 180° — each carried one column along:
/// three perceptions whose input bodies are one shape turned. Each pair is
/// related by a reciprocal transition, exact through order four with no
/// residue, and the three close one ring coherent through order four. Once
/// expanded, no perception is shed.
#[test]
fn three_perceptions_of_one_shape_close_a_coherent_ring() {
    let mut c = Contemplation::new();
    let a = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    let b = perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"]);
    let d = perceive(&mut c, &["11000", "01000", "01000", "00000"], &["01100", "00100", "00100", "00000"]);
    expanded(&mut c);
    assert_eq!(c.shed_unglued(), 0, "no perception passes the unglued door");
    assert_eq!(c.sight.related, 3, "three thoughts: each pair related");
    for (x, y) in [(&a, &b), (&b, &d), (&d, &a)] {
        let t = c.transition_of(x, y).expect("a thought between them");
        let back = c.transition_of(y, x).expect("and read from the other side");
        assert!(back.versor.projective_eq(&t.versor.reverse()), "V_BA ∝ Ṽ_AB");
        assert_eq!((t.through, t.residue.len()), (Harmonic::Four, 0), "exact through order four, no residue");
    }
    let m = c.meet_graph();
    assert_eq!((m.vertices, m.edges.iter().sum::<u64>(), m.cycle_rank), (3, 3, 1), "one cycle of three thoughts");
    assert_eq!(c.sight.coherent, [0, 0, 1], "the ring coherent through order four");
    assert_eq!(c.sight.split, [0, 0, 0]);
}

/// RX6 AT THE EDGE — an L and its 2× blow-up: the thought between them fits
/// through order three, its residue at four is the block's own `S₄(2)/2² =
/// −1/4`, and it is filed on the docket as a demand carrying that residue
/// exactly, the perception pass untouched.
#[test]
fn a_blow_up_is_a_thought_through_order_three_and_its_residue_is_demanded() {
    let mut c = Contemplation::new();
    let blown = perceive(
        &mut c,
        &["110000", "110000", "110000", "110000", "111100", "111100"],
        &["011000", "011000", "011000", "011000", "011110", "011110"],
    );
    let l = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    assert_eq!(c.sight.related, 1, "the two meet on the chart they share");
    let t = c.transition_of(&l, &blown).expect("the thought from the L to its blow-up");
    assert_eq!(t.through, Harmonic::Three, "congruent through order three");
    let grid_plane = crate::transducer::base::GRID_PLANE;
    let on_grid: Vec<_> = t.residue.iter().filter(|r| r.order == Harmonic::Four && r.plane == grid_plane).collect();
    assert_eq!(on_grid.iter().map(|r| r.vector.clone()).collect::<Vec<_>>(), vec![[q(-1, 4), q(0, 1)]]);
    assert_eq!((c.sight.edge_split, c.sight.undemanded), (1, 0), "the split is demanded");
    let expected = Mv::scalar(q(-1, 4));
    let filed = c.docket_cites().into_iter().any(|cite| match cite {
        Cite::Demand { residue: Some(r), .. } => r.sub(&expected).is_zero(),
        _ => false,
    });
    assert!(filed, "a demand stands carrying the residue −1/4 exactly");
}

/// T3 AND THE GROUPOID — A RING OF REAL SHAPES IS READ THE SAME FROM EVERY
/// VERTEX, and it never splits: the transitions carry each shape onto the
/// next, so around the ring the holonomy carries the first onto itself — a
/// symmetry of it, of finite order. A bar, the bar turned, the bar again:
/// the ring's holonomy may be the bar's 180° turn, filled at orders two and
/// four and congruent at three, never split.
#[test]
fn t3_a_ring_reads_the_same_from_every_vertex_and_real_shapes_never_split() {
    let bar = blade::sensor_plane::shape_of(&[(0, 0, 0), (0, 1, 0), (0, 2, 0)]);
    let upright = blade::sensor_plane::shape_of(&[(0, 0, 0), (1, 0, 0), (2, 0, 0)]);
    let along = blade::sensor_plane::shape_of(&[(3, 1, 0), (3, 2, 0), (3, 3, 0)]);
    let rep = |x: &blade::Shape, y: &blade::Shape| {
        let coset = blade::versor_coset(x, y, &mut crate::physics::Physics::default().stroke_n(1 << 20)).expect("carried");
        blade::coset_representative(&coset).expect("named").clone()
    };
    let (t1, t2, t3) = (rep(&bar, &upright), rep(&upright, &along), rep(&along, &bar));
    let from_bar = ring_reading(&[&t1, &t2, &t3], 3);
    assert_eq!(from_bar, ring_reading(&[&t2, &t3, &t1], 3), "read from the upright bar");
    assert_eq!(from_bar, ring_reading(&[&t3, &t1, &t2], 3), "read from the bar along");
    assert!(
        from_bar.iter().all(|(_, o)| matches!(o, RingOrder::Filled | RingOrder::Congruent(_))),
        "a ring of real shapes closes at every order: {from_bar:?}"
    );
}
