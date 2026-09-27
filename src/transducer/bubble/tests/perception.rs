// N4, 78.11, 78.26(5), 78.48 — PERCEPTIONS ARE NOT THOUGHTS: a held
// perception persists through the unglued door, and a THOUGHT is the
// transition between two perceptions' bodies — its own orbit, a circuit of
// the cell their chart is — filed as the edge the meet complex fills, and
// decided by that fill.

use crate::chronometry::Cite;
use crate::transducer::alphabet::Order;
use crate::transducer::bubble::Contemplation;
use crate::transducer::cech::Fill;
use crate::transducer::family::{perceive_family, scene_form_on, Grid, NoCrystal};
use crate::transducer::rewrite::{Rewrite, RewritePayload};

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

/// The fill of the meet complex's edge between two ideas, where one stands.
fn thought(c: &Contemplation, a: &[u8], b: &[u8]) -> Option<Fill> {
    let mut verts = vec![a.to_vec(), b.to_vec()];
    verts.sort();
    c.complex.get(&verts).map(|s| s.fill.clone())
}

/// The periods the torsion rewrites pending on the contemplation claim.
fn torsions(c: &Contemplation) -> Vec<u64> {
    c.pending_rewrites
        .iter()
        .filter_map(Rewrite::from_element)
        .filter_map(|r| match r.payload {
            RewritePayload::Torse { period, .. } => Some(period),
            _ => None,
        })
        .collect()
}

/// The finite orders an idea's horizon holds.
fn horizon_orders(c: &Contemplation, k: &[u8]) -> Vec<u32> {
    match c.bubbles.get(k) {
        Some(b) => b
            .horizon
            .scoped()
            .filter_map(|(_, (_, o, _))| match o {
                Order::Finite(n) => Some(*n),
                Order::Infinite => None,
            })
            .collect(),
        None => Vec::new(),
    }
}

const L: [&str; 4] = ["10000", "10000", "11000", "00000"];
const L_OUT: [&str; 4] = ["01000", "01000", "01100", "00000"];
const L90: [&str; 4] = ["11100", "10000", "00000", "00000"];
const L90_OUT: [&str; 4] = ["01110", "01000", "00000", "00000"];

/// THE THOUGHT, DECIDED BY ITS FILL (78.48(5)). The L, and the L turned 90°:
/// the transition between their bodies is harmonic — elliptic, returning at
/// ℓ = 4 — so the chart's cell closes, the edge fills CONGRUENT, and the two
/// perceptions merge on the torsion axis into one thought: the torsion
/// minted at the turn's period, and the thought standing on both bodies. No
/// remainder volume was asked: a perfect match merges.
#[test]
fn two_perceptions_of_one_shape_turned_merge_into_one_thought() {
    let mut c = Contemplation::new();
    let a = perceive(&mut c, &L, &L_OUT);
    let b = perceive(&mut c, &L90, &L90_OUT);
    assert_eq!((c.resonance.closes, c.resonance.resonant, c.resonance.edges), (1, 1, 1), "{}", c.resonance.line());
    assert_eq!((c.torsion_merges, c.merge_reading().merged), (1, 1), "one torsion merge");
    assert_eq!(torsions(&c), vec![4], "its Torse claims the turn's period");
    assert!(c.bubbles.get(&a).is_none() && c.bubbles.get(&b).is_none(), "the two perceptions are one thought now");
    let [(_, thought)] = c.bubbles.scoped().collect::<Vec<_>>().try_into().expect("one thought held");
    let bodies: usize = thought.spores.scoped().map(|(_, bs)| bs.len()).sum();
    assert_eq!(bodies, 2, "the thought stands on both bodies it was formed of");
}

/// A THIRD PERCEPTION MEETS THE THOUGHT. The L at 180° meets the thought the
/// first two formed, on its two bodies: two orbits — 180° to the first body
/// (ℓ = 2), 90° to the second (ℓ = 4) — which differ, so the shear reads
/// CURVED and no one transition is chosen as the edge; each is kept by its
/// class in BOTH horizons, which the pivot reads.
#[test]
fn a_third_perception_meets_the_thought_by_both_its_orbits() {
    let mut c = Contemplation::new();
    perceive(&mut c, &L, &L_OUT);
    perceive(&mut c, &L90, &L90_OUT);
    let d = perceive(&mut c, &["11000", "01000", "01000", "00000"], &["01100", "00100", "00100", "00000"]);
    assert_eq!((c.resonance.several, c.resonance.kept_congruent, c.resonance.curved), (1, 2, 1), "{}", c.resonance.line());
    let held = horizon_orders(&c, &d);
    assert!(held.contains(&2) && held.contains(&4), "the perception's horizon holds both periods: {held:?}");
    let thought = c.bubbles.scoped().map(|(k, _)| k).find(|k| *k != d).expect("the thought");
    let held = horizon_orders(&c, &thought);
    assert!(held.contains(&2) && held.contains(&4), "and so does the thought's: {held:?}");
    assert!(c.dirty.contains(&d) && c.dirty.contains(&thought), "both are marked for the pivot");
}

/// A FILLED THOUGHT: one L seen in two scenes where it does not move — the
/// scenes differ in what the L becomes. The transition between the bodies is
/// the identity, the edge fills FILLED, and the fill decides: a merge is
/// asked of the two (the forms then merge or stand, as 13.12 reads them).
#[test]
fn the_same_body_in_two_scenes_is_a_filled_thought() {
    let mut c = Contemplation::new();
    perceive(&mut c, &L, &L_OUT);
    perceive(&mut c, &L, &["20000", "20000", "22000", "00000"]);
    assert_eq!((c.resonance.edges, c.fills), (1, 1), "{}", c.resonance.line());
    let m = c.merge_reading();
    assert_eq!(m.merged + m.stayed, 1, "the filled edge asked for a merge: {}", m.line());
}

/// An L and its 2× blow-up: the transition dilates, a BOOST — luminal, it
/// never returns — so the cell stays open, the edge between them is filled
/// as a CLASS (a standing thought, no period, each written into the other's
/// horizon), and the incoherence stands on the docket as a demand on the
/// chart by its class; a held word spelling that class answers it.
#[test]
fn a_blow_up_is_a_luminal_thought_and_its_class_is_demanded() {
    let mut c = Contemplation::new();
    let blown = perceive(
        &mut c,
        &["110000", "110000", "110000", "110000", "111100", "111100"],
        &["011000", "011000", "011000", "011000", "011110", "011110"],
    );
    let l = perceive(&mut c, &L, &L_OUT);
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

/// THE DOCKET'S FOLD (78.48(9)). The blow-up's demand stands on its chart.
/// Pending in its own orbit it is not folded; sealed, it is kept while the
/// perceptions stand on its chart. Re-seeded into a mind that holds nothing
/// on that chart — a restart whose orbit met nothing there — it has carried
/// no flux for a whole orbit, and the fold lapses it, counted.
#[test]
fn a_question_nothing_can_carry_lapses_at_the_fold() {
    let mut c = Contemplation::new();
    perceive(
        &mut c,
        &["110000", "110000", "110000", "110000", "111100", "111100"],
        &["011000", "011000", "011000", "011000", "011110", "011110"],
    );
    perceive(&mut c, &L, &L_OUT);
    c.docket_fold();
    assert_eq!((c.docket_reading().pending, c.docket_reading().lapsed), (1, 0), "a new question is given its orbit");
    c.docket_sealed();
    c.docket_fold();
    assert_eq!((c.docket_reading().open, c.docket_reading().lapsed), (1, 0), "ideas stand on its chart: kept");
    let mut fresh = Contemplation::new();
    fresh.docket_reseed(&c.docket_cites());
    fresh.docket_fold();
    assert_eq!((fresh.docket_reading().open, fresh.docket_reading().lapsed), (0, 1), "nothing stands there: it lapses");
    assert!(fresh.docket_cites().is_empty(), "and the close cites nothing for it");
}

/// BRANCHING — one L, then two Ls each turned 90° about its own place: the
/// one L's body is carried onto each by its own turn, two circuits of the
/// chart's cell, both harmonic, so the cell closes. The two perceptions are
/// related by two different transitions: none is chosen as their edge, and
/// both congruences are kept at their period in both horizons.
#[test]
fn a_body_carried_onto_two_keeps_both_congruences_and_chooses_no_edge() {
    let mut c = Contemplation::new();
    let one = perceive(&mut c, &["10000000", "10000000", "11000000", "00000000"], &["01000000", "01000000", "01100000", "00000000"]);
    let two = perceive(&mut c, &["11100000", "10000111", "00000100", "00000000"], &["01110000", "01000011", "00000010", "00000000"]);
    assert_eq!((c.resonance.circuits, c.resonance.closes, c.resonance.several, c.resonance.kept_congruent), (2, 1, 1, 2), "{}", c.resonance.line());
    assert!(thought(&c, &one, &two).is_none(), "no one transition is chosen as the edge");
    let periods: Vec<u32> = c.complex.congruent.scoped().map(|(_, (_, ell))| *ell).collect();
    assert!(!periods.is_empty() && periods.iter().all(|&l| l == 4), "each turn filed at its period: {periods:?}");
    assert!(horizon_orders(&c, &one).contains(&4) && horizon_orders(&c, &two).contains(&4), "both horizons hold it");
}
