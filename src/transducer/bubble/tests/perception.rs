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

/// A scene built from its cells: the input a `rows × cols` grid holding the
/// cells, its output the same carried one column along.
fn scene(c: &mut Contemplation, cells: &[(usize, usize)], rows: usize, cols: usize) -> Vec<u8> {
    let draw = |shift: usize| -> Vec<String> {
        (0..rows)
            .map(|r| (0..cols).map(|col| if col >= shift && cells.contains(&(r, col - shift)) { '1' } else { '0' }).collect())
            .collect()
    };
    let (input, output) = (draw(0), draw(1));
    let i: Vec<&str> = input.iter().map(String::as_str).collect();
    let o: Vec<&str> = output.iter().map(String::as_str).collect();
    perceive(c, &i, &o)
}

/// The cells turned 90° about the grid point (3, 3): `(r, c) ↦ (c, 6 − r)`.
fn turned(cells: &[(usize, usize)]) -> Vec<(usize, usize)> {
    cells.iter().map(|&(r, c)| (c, 6 - r)).collect()
}

/// The cells carried `dc` columns along.
fn slid(cells: &[(usize, usize)], dc: usize) -> Vec<(usize, usize)> {
    cells.iter().map(|&(r, c)| (r, c + dc)).collect()
}

/// An L, and a second L of the same shape standing apart from it.
const LA: [(usize, usize); 4] = [(0, 1), (1, 1), (2, 1), (2, 2)];
const LB: [(usize, usize); 4] = [(4, 4), (5, 4), (6, 4), (6, 5)];

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
    // THE EDGE CARRIES WHAT THE TRANSITION CARRIED, lane by lane (78.49(5)):
    // the L's four sites all stand inside the blow-up's sixteen, so twelve
    // appear — 𝒮 12 — and a hue-blind body and a clockless chain read
    // nothing on 𝒢 and 𝒯.
    let mut verts = vec![l.clone(), blown.clone()];
    verts.sort();
    let volume = c.complex.get(&verts).expect("the edge").volume;
    assert_eq!((volume[0], volume[1], volume[2]), (12, 0, 0), "the lanes the transition carried: {volume:?}");
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
/// perceptions stand on its chart. Open in a mind that holds nothing on that
/// chart, it has carried no flux for a whole orbit, and the fold lapses it,
/// counted.
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
    for cite in c.docket_cites() {
        fresh.docket_put(cite);
    }
    fresh.docket_sealed();
    fresh.docket_fold();
    assert_eq!((fresh.docket_reading().open, fresh.docket_reading().lapsed), (0, 1), "nothing stands there: it lapses");
    assert!(fresh.docket_cites().is_empty(), "and the close cites nothing for it");
}

/// ONE ORBIT: the L turned 90° at a time about the grid's point (2, 2) —
/// `(r, c) ↦ (c, 4 − r)` — each carried one column in its output.
const O0: [&str; 5] = ["010000", "010000", "011000", "000000", "000000"];
const O0_OUT: [&str; 5] = ["001000", "001000", "001100", "000000", "000000"];
const O1: [&str; 5] = ["000000", "001110", "001000", "000000", "000000"];
const O1_OUT: [&str; 5] = ["000000", "000111", "000100", "000000", "000000"];
const O2: [&str; 5] = ["000000", "000000", "001100", "000100", "000100"];
const O2_OUT: [&str; 5] = ["000000", "000000", "000110", "000010", "000010"];
const O3: [&str; 5] = ["000000", "000000", "001000", "111000", "000000"];
const O3_OUT: [&str; 5] = ["000000", "000000", "000100", "011100", "000000"];

/// THE HOUR PREDICTED (78.48(7)). The L at hour 0 and hour 1 merge into one
/// thought holding both, in the order they arrived; its orbit's step is the
/// 90° turn about (2, 2), period 4. The L at hour 2 is exactly the orbit's
/// next hour: predicted, it joins the thought by the orbit's step — a second
/// torsion merge — and the thought stands on all three bodies.
#[test]
fn the_next_hour_of_an_orbit_is_predicted_and_joins_the_thought() {
    let mut c = Contemplation::new();
    perceive(&mut c, &O0, &O0_OUT);
    perceive(&mut c, &O1, &O1_OUT);
    assert_eq!(c.torsion_merges, 1, "hours 0 and 1 are one thought");
    perceive(&mut c, &O2, &O2_OUT);
    assert_eq!((c.resonance.predicted, c.resonance.phase_slips, c.resonance.off_orbit), (1, 0, 0), "{}", c.resonance.line());
    assert_eq!(c.torsion_merges, 2, "the predicted hour joins by the orbit's step");
    assert_eq!(torsions(&c), vec![4, 4], "each join a torsion at the orbit's period");
    let [(_, thought)] = c.bubbles.scoped().collect::<Vec<_>>().try_into().expect("one thought held");
    let bodies: usize = thought.spores.scoped().map(|(_, bs)| bs.len()).sum();
    assert_eq!(bodies, 3, "the thought stands on the three hours");
}

/// A PHASE SLIP. After hours 0 and 1 the orbit predicts hour 2; the L at
/// hour 3 is on the orbit but not where it was predicted. It is not joined;
/// its slip is a demand at the chart whose residue is the phase it slipped
/// by — one step of the orbit, a congruence of period 4.
#[test]
fn an_hour_out_of_turn_is_a_phase_slip_demanded() {
    let mut c = Contemplation::new();
    perceive(&mut c, &O0, &O0_OUT);
    perceive(&mut c, &O1, &O1_OUT);
    perceive(&mut c, &O3, &O3_OUT);
    assert_eq!((c.resonance.predicted, c.resonance.phase_slips, c.resonance.off_orbit), (0, 1, 0), "{}", c.resonance.line());
    assert_eq!(c.torsion_merges, 1, "the slipped hour does not join");
    let period = |r: &crate::transducer::cga::Mv| {
        let f = crate::transducer::element::conformal_factors(r).ok()?;
        crate::transducer::element::rotor_of_factors(&f).ok()?.order()
    };
    let slipped = c.docket_cites().into_iter().any(|cite| match cite {
        Cite::Demand { residue: Some(r), .. } => period(&r) == Some(Order::Finite(4)),
        _ => false,
    });
    assert!(slipped, "a demand at the chart, its residue one step of the orbit");
}

/// OFF THE ORBIT. After hours 0 and 1, the L of hour 0 carried one row down
/// is on no hour of the orbit: its slip is a demand at the chart, counted
/// apart from a phase slip.
#[test]
fn a_body_off_the_orbit_slips_and_is_demanded() {
    let mut c = Contemplation::new();
    perceive(&mut c, &O0, &O0_OUT);
    perceive(&mut c, &O1, &O1_OUT);
    perceive(
        &mut c,
        &["000000", "010000", "010000", "011000", "000000"],
        &["000000", "001000", "001000", "001100", "000000"],
    );
    assert_eq!((c.resonance.predicted, c.resonance.phase_slips, c.resonance.off_orbit), (0, 0, 1), "{}", c.resonance.line());
    assert!(
        c.docket_cites().into_iter().any(|cite| matches!(cite, Cite::Demand { residue: Some(_), .. })),
        "the slip stands as a demand carrying its residue"
    );
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
    // One class, two orbits (78.49(4)): the two turns stand about different
    // points, so each is filed under the class at where it stands — neither
    // overwrites the other.
    assert_eq!(periods, vec![4, 4], "each turn filed at its own coordinate, at its period");
    assert!(horizon_orders(&c, &one).contains(&4) && horizon_orders(&c, &two).contains(&4), "both horizons hold it");
}

/// THE SCENE MOTION (78.50). Two Ls, then both carried one column along: each
/// L's body relates to each of the other's, and the pair's spectrum holds four
/// rays — but ONE carries the whole configuration. It is the thought: the
/// edge the complex fills, a standing class (a slide never returns).
#[test]
fn one_motion_carrying_every_body_is_the_scene_and_the_thought() {
    let mut c = Contemplation::new();
    let a = scene(&mut c, &[&LA[..], &LB[..]].concat(), 7, 9);
    let b = scene(&mut c, &[slid(&LA, 1), slid(&LB, 1)].concat(), 7, 9);
    assert_eq!((c.resonance.scene_motions, c.resonance.several, c.resonance.appeared), (1, 0, 0), "{}", c.resonance.line());
    assert!(matches!(thought(&c, &a, &b), Some(Fill::Class(..))), "the scene's motion is the thought");
}

/// A TURNED SCENE (78.50, 78.51 tier 2): the two Ls both turned 90° about one
/// point — one motion carries the whole configuration, elliptic. The two
/// bodies co-rotate about their barycentre in synchronous lock, their
/// attitudes balanced, and the turn keeps their inversive distance: the gate
/// consolidates it, and the thought is congruent at ℓ = 4 — the two
/// perceptions merge on its torsion axis.
#[test]
fn a_scene_turned_whole_is_one_congruent_thought() {
    let mut c = Contemplation::new();
    scene(&mut c, &[&LA[..], &LB[..]].concat(), 7, 9);
    scene(&mut c, &turned(&[&LA[..], &LB[..]].concat()), 7, 9);
    assert_eq!(
        (c.resonance.scene_motions, c.resonance.consolidated, c.resonance.unbalanced, c.resonance.nonrigid),
        (1, 1, 0, 0),
        "{}",
        c.resonance.line()
    );
    assert_eq!(torsions(&c), vec![4], "one torsion at the turn's period");
}

/// A SYMMETRIC SCENE (78.50, 78.51 tier 2): an L and its 180° turn about the
/// point form a configuration the 180° turn carries onto itself; turned 90°
/// whole, TWO motions carry it — the 90° and the 270° — landing on the same
/// bodies: one coset. Its representative co-rotates the pair in lock and
/// keeps their distance: consolidated, the edge carries it, and the
/// quotient, the configuration's own symmetry, stands filed at ℓ = 2.
#[test]
fn a_symmetric_scene_is_one_coset_and_its_symmetry_is_filed() {
    let lb: Vec<(usize, usize)> = LA.iter().map(|&(r, c)| (6 - r, 6 - c)).collect();
    let mut c = Contemplation::new();
    let before = [&LA[..], &lb[..]].concat();
    scene(&mut c, &before, 7, 9);
    scene(&mut c, &turned(&before), 7, 9);
    assert_eq!(
        (c.resonance.cosets, c.resonance.symmetries, c.resonance.scene_motions, c.resonance.consolidated, c.resonance.unbalanced),
        (1, 1, 1, 1, 0),
        "{}",
        c.resonance.line()
    );
    let periods: Vec<u32> = c.complex.congruent.scoped().map(|(_, (_, ell))| *ell).collect();
    assert!(periods.contains(&2), "the symmetry at its period: {periods:?}");
}

/// NO ONE MOTION (78.50): one L carried along, the other turned in its place —
/// no ray carries both, so no scene motion stands; the spectrum is kept, each
/// ray by its class.
#[test]
fn bodies_moving_apart_keep_the_spectrum() {
    let turned_b: Vec<(usize, usize)> = [(4, 4), (4, 5), (4, 6), (5, 4)].to_vec();
    let mut c = Contemplation::new();
    scene(&mut c, &[&LA[..], &LB[..]].concat(), 7, 9);
    scene(&mut c, &[slid(&LA, 1), turned_b].concat(), 7, 9);
    assert_eq!((c.resonance.scene_motions, c.resonance.several), (0, 1), "{}", c.resonance.line());
    assert!(c.resonance.kept_class + c.resonance.kept_congruent > 0, "each ray kept by its class");
}

/// WHAT APPEARED AND WHAT VANISHED (78.50): both Ls carried along and a third
/// L standing new — the motion still carries the whole of what was there, and
/// the new L is what appeared. The other way, three Ls and then two of them
/// carried along: the motion lands on the whole of what is there, and the
/// third is what vanished.
#[test]
fn a_body_that_appears_or_vanishes_is_counted_and_the_scene_still_moves() {
    let third = vec![(0, 5), (1, 5), (2, 5), (2, 6)];
    let mut c = Contemplation::new();
    scene(&mut c, &[&LA[..], &LB[..]].concat(), 7, 9);
    scene(&mut c, &[slid(&LA, 1), slid(&LB, 1), third.clone()].concat(), 7, 9);
    assert_eq!((c.resonance.scene_motions, c.resonance.appeared, c.resonance.vanished), (1, 1, 0), "{}", c.resonance.line());
    let mut c = Contemplation::new();
    scene(&mut c, &[&LA[..], &LB[..], &third[..]].concat(), 7, 9);
    scene(&mut c, &[slid(&LA, 1), slid(&LB, 1)].concat(), 7, 9);
    assert_eq!((c.resonance.scene_motions, c.resonance.appeared, c.resonance.vanished), (1, 0, 1), "{}", c.resonance.line());
}

/// THE HOURS OF A SLIDE (78.50): three Ls three columns apart are an orbit's
/// samples whose step is a slide; the L three columns on is its next hour,
/// predicted and joined; the L six columns on stands at the hour after —
/// a phase slip; the L one column on stands at no hour — off the orbit.
#[test]
fn a_slides_next_hour_is_predicted_and_its_slips_are_read_exactly() {
    let l = [(0usize, 0usize), (1, 0), (2, 0), (2, 1)];
    let samples = [slid(&l, 0), slid(&l, 3), slid(&l, 6)].concat();
    for (at, expect) in [(9usize, (1u64, 0u64, 0u64)), (12, (0, 1, 0)), (7, (0, 0, 1))] {
        let mut c = Contemplation::new();
        scene(&mut c, &samples, 4, 16);
        scene(&mut c, &slid(&l, at), 4, 16);
        let r = &c.resonance;
        assert_eq!((r.predicted, r.phase_slips, r.off_orbit), expect, "the L at column {at}: {}", r.line());
    }
}

/// THE STRATA ABOVE A CHART (78.49(4)): a solid block and the block with its
/// middle cut out stand on charts that PART at the first Betti number the
/// ring has and the block has not (β₁). Their deepest shared stratum is read,
/// the surgery that carries one onto the other factors, and the deformation
/// stands as a demand on both charts, its residue the core's ray.
#[test]
fn a_hole_cut_is_a_deformation_read_at_the_stratum_where_the_charts_part() {
    let mut c = Contemplation::new();
    perceive(&mut c, &["11100", "11100", "11100", "00000"], &["01110", "01110", "01110", "00000"]);
    perceive(&mut c, &["11100", "10100", "11100", "00000"], &["01110", "01010", "01110", "00000"]);
    assert_eq!((c.resonance.deformed, c.resonance.unfactored), (1, 0), "{}", c.resonance.line());
    let on_both = c.docket_cites().into_iter().any(|cite| match cite {
        Cite::Demand { cover: Some(cv), residue: Some(_), .. } => cv.len() == 2,
        _ => false,
    });
    assert!(on_both, "the deformation a demand on both charts, carrying its core");
}

/// A PARTING NO MOTION CARRIES (78.49(4)): an L and a bar have one topology
/// and part at their turns; every axis past the Betti numbers is invariant
/// under every versor, so no conformal motion carries one onto the other —
/// counted apart, and nothing filed.
#[test]
fn bodies_parting_past_their_topology_are_apart() {
    let mut c = Contemplation::new();
    perceive(&mut c, &L, &L_OUT);
    perceive(&mut c, &["11100", "00000", "00000", "00000"], &["01110", "00000", "00000", "00000"]);
    assert_eq!((c.resonance.apart, c.resonance.deformed), (1, 0), "{}", c.resonance.line());
}
