// 78.142 — THE SDK RULE SEAM: every rule the seam builds is judged by the
// REAL court (`Kernel::judge_solve`) on pairs it never saw, and the seam's
// own `would_judge` agrees with that verdict, solved or refused.

use crate::arc::Grid;
use crate::client::sdk::rule::{sentence, would_judge, NoRule, Step, Symmetry};
use crate::cognition::{Kernel, SolveVerdict};
use crate::transducer::cga::{self, Mv};
use crate::transducer::element::Element;
use crate::transducer::family::NoCrystal;

fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The symmetry read off by index, as a grid is read — the fixture's
/// expectation, never the seam's construction.
fn apply(kind: Symmetry, g: &Grid) -> Grid {
    let (h, w) = (g.len(), g[0].len());
    match kind {
        Symmetry::Rotate90 => (0..w).map(|c| (0..h).rev().map(|r| g[r][c]).collect()).collect(),
        Symmetry::Rotate180 => g.iter().rev().map(|row| row.iter().rev().copied().collect()).collect(),
        Symmetry::Rotate270 => (0..w).rev().map(|c| (0..h).map(|r| g[r][c]).collect()).collect(),
        Symmetry::FlipH => g.iter().map(|row| row.iter().rev().copied().collect()).collect(),
        Symmetry::FlipV => g.iter().rev().cloned().collect(),
        Symmetry::Transpose => (0..w).map(|c| (0..h).map(|r| g[r][c]).collect()).collect(),
    }
}

fn recolour(map: &[(u8, u8)], g: &Grid) -> Grid {
    g.iter()
        .map(|row| row.iter().map(|&h| map.iter().find(|(f, _)| *f == h).map_or(h, |(_, t)| *t)).collect())
        .collect()
}

fn shape(g: &Grid) -> (usize, usize) {
    (g.len(), g[0].len())
}

/// The court's verdict and the seam's, which must agree.
fn judged(rule: &Element, withheld: &[(Grid, Grid)]) -> (SolveVerdict, Result<(), NoRule>) {
    let mut k = Kernel::new();
    let verdict = k.judge_solve(rule, withheld);
    let court: &Kernel = &k;
    let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
    (verdict, seam)
}

fn assert_solved(rule: &Element, withheld: &[(Grid, Grid)], what: &str) {
    let (verdict, seam) = judged(rule, withheld);
    assert!(matches!(verdict, SolveVerdict::Solved { .. }), "{what}: the court solves it: {verdict:?}");
    assert!(seam.is_ok(), "{what}: the seam agrees: {seam:?}");
}

/// Two L-shaped trominoes' worth of scene on a 3 × 3 grid, two hues, and
/// the same L placed elsewhere and painted otherwise for the withheld pair.
fn square_train() -> Grid {
    grid(&["120", "100", "000"])
}
fn square_withheld() -> Grid {
    grid(&["000", "034", "033"])
}

#[test]
fn every_grid_symmetry_of_a_square_is_solved_on_a_pair_it_never_saw() {
    for kind in [
        Symmetry::Rotate90,
        Symmetry::Rotate180,
        Symmetry::Rotate270,
        Symmetry::FlipH,
        Symmetry::FlipV,
        Symmetry::Transpose,
    ] {
        let x = square_train();
        let family = vec![(x.clone(), apply(kind, &x))];
        let step = Step::grid_motion(kind, (3, 3), (3, 3)).expect("a square fits every symmetry");
        let rule = sentence(&[step], &family, &NoCrystal).unwrap_or_else(|e| panic!("{kind:?} mints: {e:?}"));
        let w = square_withheld();
        assert_solved(&rule, &[(w.clone(), apply(kind, &w))], &format!("{kind:?}"));
    }
}

#[test]
fn a_turn_and_the_transpose_take_a_non_square_grid_to_its_exchange() {
    for kind in [Symmetry::Rotate90, Symmetry::Transpose] {
        let x = grid(&["120", "100"]);
        let y = apply(kind, &x);
        assert_eq!(shape(&y), (3, 2), "R × C goes to C × R");
        let step = Step::grid_motion(kind, (2, 3), (3, 2)).expect("the exchange fits");
        let rule = sentence(&[step], &[(x, y)], &NoCrystal).unwrap_or_else(|e| panic!("{kind:?} mints: {e:?}"));
        let w = grid(&["034", "033"]);
        assert_solved(&rule, &[(w.clone(), apply(kind, &w))], &format!("{kind:?} on 2 × 3"));
    }
    // A SHAPE THE SYMMETRY DOES NOT FIT is no motion at all.
    assert!(Step::grid_motion(Symmetry::Rotate90, (2, 3), (2, 3)).is_none());
    assert!(Step::grid_motion(Symmetry::Transpose, (2, 3), (2, 3)).is_none());
    assert!(Step::grid_motion(Symmetry::Rotate180, (2, 3), (3, 2)).is_none());
    assert!(Step::grid_motion(Symmetry::FlipH, (2, 3), (3, 2)).is_none());
}

#[test]
fn a_hue_permutation_is_solved_on_a_pair_it_never_saw() {
    let map = [(1u8, 2u8), (2, 1)];
    let x = square_train();
    let rule = sentence(&[Step::Hue(map.to_vec())], &[(x.clone(), recolour(&map, &x))], &NoCrystal).expect("mints");
    let w = grid(&["000", "012", "011"]);
    assert_solved(&rule, &[(w.clone(), recolour(&map, &w))], "hue permutation");
}

#[test]
fn a_motion_then_a_recolour_is_solved_as_one_action() {
    let map = [(1u8, 5u8)];
    let steps = [Step::grid_motion(Symmetry::Rotate180, (3, 3), (3, 3)).expect("fits"), Step::Hue(map.to_vec())];
    let then = |g: &Grid| recolour(&map, &apply(Symmetry::Rotate180, g));
    let x = square_train();
    let rule = sentence(&steps, &[(x.clone(), then(&x))], &NoCrystal).expect("mints");
    let w = grid(&["000", "014", "011"]);
    assert_solved(&rule, &[(w.clone(), then(&w))], "motion then recolour");
}

#[test]
fn a_step_with_no_inverse_is_no_rule() {
    use crate::transducer::cga::{E1, E2};
    let one = || num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    // Mixed parity: a scalar plus a vector has no versor action.
    let mixed = Mv::scalar(one()).add(&Mv::blade(E1, one())).add(&Mv::blade(E1 | E2, one()));
    let x = square_train();
    let got = sentence(&[Step::Motion(mixed)], &[(x.clone(), x)], &NoCrystal);
    assert!(matches!(got, Err(NoRule::NotAVersor)), "refused by name: {got:?}");
}

/// THE EXACT-COMPARISON NEGATIVE CONTROL: a dilation is a versor, and
/// composed with a motion it is not that motion — by exact comparison its
/// action differs (`a.sub(&b)` is not zero), and the seam and the court
/// refuse it where the plain motion is solved.
#[test]
fn a_dilation_composed_with_a_motion_is_refused_where_the_motion_is_solved() {
    let x = square_train();
    let y = apply(Symmetry::Rotate180, &x);
    let Some(Step::Motion(m)) = Step::grid_motion(Symmetry::Rotate180, (3, 3), (3, 3)) else { panic!("fits") };
    let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
    let dilated = cga::dilator(&two).geo(&m);
    assert!(cga::is_versor(&dilated), "a dilation is a versor");
    assert!(!dilated.sub(&m).is_zero(), "and by exact comparison it is not the motion");
    let family = vec![(x.clone(), y.clone())];
    let plain = sentence(&[Step::Motion(m)], &family, &NoCrystal).expect("the motion mints");
    let w = square_withheld();
    let withheld = vec![(w.clone(), apply(Symmetry::Rotate180, &w))];
    assert_solved(&plain, &withheld, "the plain motion");
    match sentence(&[Step::Motion(dilated)], &family, &NoCrystal) {
        Err(NoRule::Mint(_)) => {}
        Err(other) => panic!("refused, but by the mint: {other:?}"),
        Ok(rule) => {
            let (verdict, seam) = judged(&rule, &withheld);
            assert!(matches!(verdict, SolveVerdict::Refused(_)), "the court refuses the dilation: {verdict:?}");
            assert!(matches!(seam, Err(NoRule::Refused(_))), "and the seam agrees: {seam:?}");
        }
    }
}

/// THE I-DEFECT, reproduced: a rule EXTRACTED from a family of two pairs
/// carries the frame-axis motor faces between its training frames; the
/// client's own replay on the training pairs passes, and the court, judging
/// it on one withheld pair, must not refuse it for a training frame the
/// withheld family does not hold.
#[test]
fn an_extracted_rule_judged_on_one_withheld_pair_is_not_refused_for_a_training_frame() {
    let train = vec![
        (grid(&["1100", "0000"]), grid(&["0022", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0022"])),
    ];
    let rule = crate::transducer::family::extract_rule(&train, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let spoken = crate::transducer::family::mint_sentence(&rule, &train, &NoCrystal, 0).expect("and mints");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    assert!(
        crate::verdict::blind_replay_at(&spoken, &|_| None, Some(&train), &NoCrystal, &mut budget).is_none(),
        "the client's own replay on its training pairs passes"
    );
    let withheld = vec![(grid(&["1100", "0000"]), grid(&["0022", "0000"]))];
    let (verdict, seam) = judged(&spoken, &withheld);
    assert!(matches!(verdict, SolveVerdict::Solved { .. }), "the court solves the withheld pair: {verdict:?}");
    assert!(seam.is_ok(), "the seam agrees: {seam:?}");
}
