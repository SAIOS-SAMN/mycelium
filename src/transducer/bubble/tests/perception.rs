// N4, 78.11, 78.26(5) — PERCEPTIONS ARE NOT THOUGHTS: a held perception
// persists through the unglued door, and a THOUGHT is the transition between
// two perceptions' bodies — a circuit of the cell their chart is, read by the
// cell reader — filed as the edge the meet complex fills and classifies.

use crate::chronometry::Cite;
use crate::transducer::bubble::Contemplation;
use crate::transducer::cech::Fill;
use crate::transducer::family::{perceive_family, scene_form_on, Grid, NoCrystal};

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
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

/// The fill of the meet complex's edge between two perceptions, where one
/// stands.
fn thought(c: &Contemplation, a: &[u8], b: &[u8]) -> Option<Fill> {
    let mut verts = vec![a.to_vec(), b.to_vec()];
    verts.sort();
    c.complex.get(&verts).map(|s| s.fill.clone())
}

/// The L, and the L turned about its own corner and carried one column: two
/// perceptions of one shape. The transition between their bodies is a
/// harmonic circuit — elliptic, returning — so the chart's cell closes, and
/// the two are joined by an edge the complex fills CONGRUENT at the turn's
/// period: a 90° turn returns at ℓ = 4. That edge is the thought: both
/// perceptions stay held, glued by it.
#[test]
fn two_perceptions_of_one_shape_turned_form_a_congruent_thought() {
    let mut c = Contemplation::new();
    let a = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    let b = perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"]);
    assert_eq!((c.resonance.cells, c.resonance.closes, c.resonance.resonant), (1, 1, 1), "{}", c.resonance.line());
    match thought(&c, &a, &b) {
        Some(Fill::Congruent(_, _, ell)) => assert_eq!(ell, 4, "a 90° turn returns at its fourth application"),
        other => panic!("a congruent thought between the two, not {other:?}"),
    }
    assert_eq!(c.meet_graph().edges[1], 1, "the meet graph holds one congruent edge");
}

/// One L at 0°, 90° and 180°: every pair is a congruent thought, each at its
/// own turn's period (90° → 4, 180° → 2), and the three edges close one
/// cycle of the meet graph.
#[test]
fn three_perceptions_of_one_shape_close_a_cycle_of_thoughts() {
    let mut c = Contemplation::new();
    let a = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    let b = perceive(&mut c, &["11100", "10000", "00000", "00000"], &["01110", "01000", "00000", "00000"]);
    let d = perceive(&mut c, &["11000", "01000", "01000", "00000"], &["01100", "00100", "00100", "00000"]);
    let period = |x: &[u8], y: &[u8]| match thought(&c, x, y) {
        Some(Fill::Congruent(_, _, ell)) => Some(ell),
        _ => None,
    };
    assert_eq!((period(&a, &b), period(&b, &d), period(&a, &d)), (Some(4), Some(4), Some(2)), "{}", c.resonance.line());
    let m = c.meet_graph();
    assert_eq!((m.vertices, m.edges[1], m.cycle_rank), (3, 3, 1), "three congruent thoughts close one cycle");
    assert_eq!(c.shed_unglued(), 0, "no perception passes the unglued door");
}

/// An L and its 2× blow-up: the transition dilates, a BOOST — luminal, it
/// never returns — so the cell stays open, the edge between them is filled
/// as a CLASS (a standing thought, no period), and the incoherence stands on
/// the docket as a demand on the chart by its class; a held word spelling
/// that class answers it.
#[test]
fn a_blow_up_is_a_luminal_thought_and_its_class_is_demanded() {
    let mut c = Contemplation::new();
    let blown = perceive(
        &mut c,
        &["110000", "110000", "110000", "110000", "111100", "111100"],
        &["011000", "011000", "011000", "011000", "011110", "011110"],
    );
    let l = perceive(&mut c, &["10000", "10000", "11000", "00000"], &["01000", "01000", "01100", "00000"]);
    assert_eq!((c.resonance.boost, c.resonance.open, c.resonance.demanded), (1, 1, 1), "{}", c.resonance.line());
    assert!(matches!(thought(&c, &l, &blown), Some(Fill::Class(..))), "a standing class between them");
    let class = c
        .docket_cites()
        .into_iter()
        .find_map(|cite| match cite {
            Cite::Demand { class, .. } => Some(class),
            _ => None,
        })
        .expect("the incoherence stands as a demand");
    c.docket_discharged(class.class_key().as_bytes());
    assert!(
        !c.docket_cites().into_iter().any(|cite| matches!(cite, Cite::Demand { .. })),
        "a word spelling its class answers it"
    );
}

/// BRANCHING — one L, then two Ls each turned 90° about its own place: the
/// one L's body is carried onto each by its own turn, two circuits of the
/// chart's cell, both harmonic, so the cell closes. The two perceptions are
/// related by two different transitions: none is chosen as their edge, and
/// both congruences stand filed by class at their period.
#[test]
fn a_body_carried_onto_two_keeps_both_congruences_and_chooses_no_edge() {
    let mut c = Contemplation::new();
    let one = perceive(&mut c, &["10000000", "10000000", "11000000", "00000000"], &["01000000", "01000000", "01100000", "00000000"]);
    let two = perceive(&mut c, &["11100000", "10000111", "00000100", "00000000"], &["01110000", "01000011", "00000010", "00000000"]);
    assert_eq!((c.resonance.circuits, c.resonance.closes, c.resonance.several), (2, 1, 1), "{}", c.resonance.line());
    assert!(thought(&c, &one, &two).is_none(), "no one transition is chosen as the edge");
    let periods: Vec<u32> = c.complex.congruent.scoped().map(|(_, (_, ell))| *ell).collect();
    assert!(!periods.is_empty() && periods.iter().all(|&l| l == 4), "each turn filed at its period: {periods:?}");
}
