// N4, RX1–RX7, 0.7 — PERCEPTIONS ARE NOT THOUGHTS: a held perception persists
// through the unglued door, and a thought is its bodies resonating with
// another's — every body heard against every body, read from both sides —
// growing filaments that close rings order by order.

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
/// three perceptions whose input bodies are one shape turned. Each pair's
/// bodies resonate through a reciprocal filament, exact through order four
/// with no residue, and the three filaments close one ring coherent through
/// order four: three spores, three filaments, cycle rank one, all in the
/// 2-core. Once expanded, no perception is shed.
#[test]
fn three_perceptions_of_one_shape_close_a_coherent_ring() {
    let mut c = Contemplation::new();
    let a = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    let b = perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"]);
    let d = perceive(&mut c, &["11000", "01000", "01000", "00000"], &["01100", "00100", "00100", "00000"]);
    expanded(&mut c);
    assert_eq!(c.shed_unglued(), 0, "no perception passes the unglued door");
    assert_eq!(c.resonance.related, 3, "each pair resonates");
    for (x, y) in [(&a, &b), (&b, &d), (&d, &a)] {
        let [t] = c.filaments_between(x, y).try_into().expect("one filament between them");
        let [back] = c.filaments_between(y, x).try_into().expect("and read from the other side");
        assert!(back.versor.projective_eq(&t.versor.reverse()), "V_BA ∝ Ṽ_AB");
        assert_eq!((t.through, t.residue.len()), (Harmonic::Four, 0), "exact through order four, no residue");
    }
    let r = c.resonance_reading();
    assert_eq!((r.spores, r.held, r.components, r.cycle_rank, r.core), (3, 3, 1, 1, 3), "one ring of three filaments");
    assert_eq!(r.coherent, [0, 0, 1], "the ring coherent through order four");
    assert_eq!(r.split, [0, 0, 0]);
    // Each perception stands where its own key says, so the drive's rekey
    // leaves it there; and a perception refiled elsewhere takes its
    // filaments with it.
    for k in [&a, &b, &d] {
        assert_eq!(c.bubbles.get(k).expect("held").key().as_bytes(), k.as_slice(), "filed at its own key");
    }
    let moved = [a.as_slice(), b"'"].concat();
    c.rekey(&a, &moved);
    assert_eq!(c.filaments_between(&moved, &b).len(), 1, "the filament follows the perception");
    let r = c.resonance_reading();
    assert_eq!((r.spores, r.held, r.cycle_rank), (3, 3, 1), "the ring still stands");
}

/// RX6 AT THE EDGE — an L and its 2× blow-up: the filament between them
/// resonates through order three, its residue at four is the block's own
/// `S₄(2)/2² = −1/4`, and it is filed on the docket as a demand carrying that
/// residue exactly, the perception pass untouched.
#[test]
fn a_blow_up_is_a_thought_through_order_three_and_its_residue_is_demanded() {
    let mut c = Contemplation::new();
    let blown = perceive(
        &mut c,
        &["110000", "110000", "110000", "110000", "111100", "111100"],
        &["011000", "011000", "011000", "011000", "011110", "011110"],
    );
    let l = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    assert_eq!(c.resonance.related, 1, "the two meet on the chart they share");
    let [t] = c.filaments_between(&l, &blown).try_into().expect("one filament from the L to its blow-up");
    assert_eq!(t.through, Harmonic::Three, "resonant through order three");
    let grid_plane = crate::transducer::base::GRID_PLANE;
    let on_grid: Vec<_> = t.residue.iter().filter(|r| r.order == Harmonic::Four && r.plane == grid_plane).collect();
    assert_eq!(on_grid.iter().map(|r| r.vector.clone()).collect::<Vec<_>>(), vec![[q(-1, 4), q(0, 1)]]);
    assert_eq!((c.resonance.edge_split, c.resonance.undemanded), (1, 0), "the split is demanded");
    let expected = Mv::scalar(q(-1, 4));
    let filed = c.docket_cites().into_iter().any(|cite| match cite {
        Cite::Demand { residue: Some(r), .. } => r.sub(&expected).is_zero(),
        _ => false,
    });
    assert!(filed, "a demand stands carrying the residue −1/4 exactly");
}

/// A CONSTELLATION — three bodies, an L, a bar and a block, each its own
/// colour; then the L and the bar each turned 90° about its own centre and
/// the block gone. Nothing is refused for holding several bodies: the L
/// resonates with the turned L and the bar with the turned bar, each through
/// its own filament, and the block, which nothing resonates with, is a
/// foraging tip whose harmonics stand on the docket.
#[test]
fn a_constellation_grows_a_filament_per_resonant_body_and_forages_the_rest() {
    let mut c = Contemplation::new();
    let before = perceive(
        &mut c,
        &["10002220", "10000000", "11000000", "00000000", "00003300", "00003300"],
        &["01000222", "01000000", "01100000", "00000000", "00000330", "00000330"],
    );
    let after = perceive(
        &mut c,
        &["00000020", "00000020", "00000020", "11100000", "10000000", "00000000"],
        &["00000002", "00000002", "00000002", "01110000", "01000000", "00000000"],
    );
    let filaments = c.filaments_between(&before, &after);
    assert_eq!(filaments.len(), 2, "the L to the L, the bar to the bar: {}", c.resonance.line());
    assert!(filaments.iter().all(|t| t.through == Harmonic::Four), "each exact through order four");
    assert_eq!(c.resonance.related, 1);
    assert_eq!(c.resonance.foraging, 1, "the block is a foraging tip");
    let demands = c.docket_cites().into_iter().filter(|cite| matches!(cite, Cite::Demand { residue: Some(_), .. })).count();
    assert!(demands >= 1, "the tip's harmonics stand as demands");
}

/// BRANCHING — one L, then two Ls standing apart: the one resonates with
/// both, and both filaments are kept: the hypha branches, nothing is chosen
/// or pruned.
#[test]
fn a_body_resonating_with_two_grows_both_filaments() {
    let mut c = Contemplation::new();
    let one = perceive(&mut c, &["10000000", "10000000", "11000000", "00000000"], &["01000000", "01000000", "01100000", "00000000"]);
    let two = perceive(&mut c, &["10000100", "10000100", "11000110", "00000000"], &["01000010", "01000010", "01100011", "00000000"]);
    let filaments = c.filaments_between(&one, &two);
    assert_eq!(filaments.len(), 2, "one filament to each L: {}", c.resonance.line());
    assert_eq!(c.resonance.branching, 1, "the one L branches");
    assert_eq!(c.resonance.foraging, 0, "nothing is left unheard");
    let r = c.resonance_reading();
    assert_eq!((r.spores, r.held), (3, 2), "three spores, two filaments");
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
