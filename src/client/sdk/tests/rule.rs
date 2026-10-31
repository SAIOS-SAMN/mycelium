// 78.142 — THE SDK RULE SEAM: every rule the seam builds is judged by the
// REAL court (`Court::judge_solve`) on pairs it never saw, and the seam's
// own `would_judge` agrees with that verdict, solved or refused.

use crate::arc::Grid;
use crate::client::sdk::rule::{sentence, would_judge, NoRule, Step, Symmetry};
use crate::cognition::{Court, SolveVerdict};
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
    let mut k = Court::new();
    let verdict = k.judge_solve(rule, withheld);
    let court: &Court = &k;
    let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
    (verdict, seam)
}

fn assert_solved(rule: &Element, withheld: &[(Grid, Grid)], what: &str) {
    let (verdict, seam) = judged(rule, withheld);
    assert!(matches!(verdict, SolveVerdict::Solved { .. }), "{what}: the court solves it: {verdict:?}");
    assert!(seam.is_ok(), "{what}: the seam agrees: {seam:?}");
}

/// An L-shaped tromino on a 3 × 3 grid in two hues, and the same L placed
/// elsewhere and painted otherwise for the withheld pair: one chart, a
/// scene the rule never saw.
fn square_train() -> Grid {
    grid(&["120", "100", "000"])
}
fn square_withheld() -> Grid {
    grid(&["000", "034", "030"])
}

/// THE MOTION CARRIES EVERY SITE WHERE THE GRID READS IT: each symmetry's
/// versor, applied by the sensor's own carrier to every site of a 2 × 3 grid
/// (a 3 × 3 for the shape-keeping ones), lands exactly on the site the
/// index map names.
#[test]
fn grid_motion_carries_every_site_to_index_image() {
    use crate::transducer::base::Site;
    for kind in [
        Symmetry::Rotate90,
        Symmetry::Rotate180,
        Symmetry::Rotate270,
        Symmetry::FlipH,
        Symmetry::FlipV,
        Symmetry::Transpose,
    ] {
        let x = grid(&["123", "456"]);
        let y = apply(kind, &x);
        let Some(Step::Motion(v)) = Step::grid_motion(kind, shape(&x), shape(&y)) else { panic!("{kind:?} fits") };
        for (r, row) in x.iter().enumerate() {
            for (c, &h) in row.iter().enumerate() {
                let site = Site::of([r as i64, c as i64, 0]);
                let to = crate::transducer::element::carry_site(&v, &site).expect("a site lands");
                let [tr, tc, _] = to.axes() else { panic!("three axes") };
                let (tr, tc) = (usize::try_from(*tr).expect("a row on the grid"), usize::try_from(*tc).expect("a column on the grid"));
                assert_eq!(y[tr][tc], h, "{kind:?}: ({r}, {c}) lands on its image");
            }
        }
    }
}

/// Every grid symmetry of a square, minted as a rule, is solved by the court
/// on a pair the rule never saw.
#[test]
fn sentence_of_solves_unseen_pair_by_square_symmetry() {
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
        let rule = match sentence(&[step], &family, &NoCrystal) {
            Ok(rule) => rule,
            Err(e) => panic!("{kind:?} mints: {e:?}"),
        };
        let w = square_withheld();
        assert_solved(&rule, &[(w.clone(), apply(kind, &w))], &format!("{kind:?}"));
    }
}

/// A turn and the transpose take a non-square grid to its exchange, and a
/// shape the symmetry does not fit is no motion.
#[test]
fn grid_motion_exchanges_non_square_shape() {
    for kind in [Symmetry::Rotate90, Symmetry::Transpose] {
        let x = grid(&["120", "100"]);
        let y = apply(kind, &x);
        assert_eq!(shape(&y), (3, 2), "R × C goes to C × R");
        let step = Step::grid_motion(kind, (2, 3), (3, 2)).expect("the exchange fits");
        let rule = match sentence(&[step], &[(x, y)], &NoCrystal) {
            Ok(rule) => rule,
            Err(e) => panic!("{kind:?} mints: {e:?}"),
        };
        let w = grid(&["034", "030"]);
        assert_solved(&rule, &[(w.clone(), apply(kind, &w))], &format!("{kind:?} on 2 × 3"));
    }
    // A SHAPE THE SYMMETRY DOES NOT FIT is no motion at all.
    assert!(Step::grid_motion(Symmetry::Rotate90, (2, 3), (2, 3)).is_none());
    assert!(Step::grid_motion(Symmetry::Transpose, (2, 3), (2, 3)).is_none());
    assert!(Step::grid_motion(Symmetry::Rotate180, (2, 3), (3, 2)).is_none());
    assert!(Step::grid_motion(Symmetry::FlipH, (2, 3), (3, 2)).is_none());
}

/// A hue permutation, minted as a rule, is solved by the court on a pair the
/// rule never saw.
#[test]
fn sentence_of_solves_unseen_pair_by_hue_permutation() {
    let map = [(1u8, 2u8), (2, 1)];
    let x = square_train();
    let rule = sentence(&[Step::Hue(map.to_vec())], &[(x.clone(), recolour(&map, &x))], &NoCrystal).expect("mints");
    let w = grid(&["000", "012", "010"]);
    assert_solved(&rule, &[(w.clone(), recolour(&map, &w))], "hue permutation");
}

/// A motion followed by a recolour is minted and solved as one action.
#[test]
fn sentence_of_solves_motion_then_recolour_as_one() {
    let map = [(1u8, 5u8)];
    let steps = [Step::grid_motion(Symmetry::Rotate180, (3, 3), (3, 3)).expect("fits"), Step::Hue(map.to_vec())];
    let then = |g: &Grid| recolour(&map, &apply(Symmetry::Rotate180, g));
    let x = square_train();
    let rule = sentence(&steps, &[(x.clone(), then(&x))], &NoCrystal).expect("mints");
    let w = grid(&["000", "014", "010"]);
    assert_solved(&rule, &[(w.clone(), then(&w))], "motion then recolour");
}

/// A step with no inverse is no rule: a mixed-parity element is refused by
/// name as not a versor.
#[test]
fn sentence_refuses_step_with_no_inverse() {
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
fn judge_solve_refuses_dilated_motion_and_solves_plain() {
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

/// THE I-DEFECT, reproduced and crossed: a rule EXTRACTED from a family of
/// two pairs carries the frame-axis motor faces between its training
/// frames; the client's own replay on the training pairs passes, and the
/// court, judging it on one withheld pair, refuses it with kind I — a
/// training frame the withheld family does not hold. The word that crosses
/// (`rule::crossing`) carries no overlap between two training frames, still
/// replays on its training pairs, and is solved on the withheld pair.
#[test]
fn crossing_drops_training_frames_so_withheld_pair_solves() {
    use crate::transducer::sentence::ObstructionKind;
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
    // THE DEFECT: the extracted word, judged as it stands, is refused I.
    let (verdict, seam) = judged(&spoken, &withheld);
    match &verdict {
        SolveVerdict::Refused(r) => assert_eq!(r.obstruction.kind(), ObstructionKind::Integrity, "refused for a training frame: {r:?}"),
        other => panic!("the extracted word carries its training frames: {other:?}"),
    }
    assert!(matches!(seam, Err(NoRule::Refused(_))), "the seam agrees: {seam:?}");
    // THE CROSSING: no overlap between two training frames rides the word.
    let crossed = crate::client::sdk::rule::crossing(&spoken, &train, &NoCrystal).expect("the rule crosses");
    assert_ne!(crossed.address(), spoken.address(), "the training frames' overlaps are gone from the word");
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    assert!(
        crate::verdict::blind_replay_at(&crossed, &|_| None, Some(&train), &NoCrystal, &mut budget).is_none(),
        "the crossed word still replays on its training pairs"
    );
    assert_solved(&crossed, &withheld, "the crossed rule");
}

/// 78.158a U2 — UNREAD AT HOME IS UNJUDGED, NEVER REFUTED. A home replay's
/// `REFUSE_UNREAD` (a unit no stroke it ran on reads) is `NoRule::Unread`,
/// told apart from every refusal of the judge's; the solve it stands for is
/// the court's to rule, and the court's `judge_solve` — resuming its replay
/// on its own strokes by its cursor — rules the same word.
#[test]
fn of_refusal_reads_unread_as_unjudged() {
    let train = vec![
        (grid(&["1100", "0000"]), grid(&["0022", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0022"])),
    ];
    let rule = crate::transducer::family::extract_rule(&train, &mut crate::physics::Physics::default().stroke_n(1 << 20))
        .expect("the family extracts");
    let spoken = crate::transducer::family::mint_sentence(&rule, &train, &NoCrystal, 0).expect("and mints");
    let crossed = crate::client::sdk::rule::crossing(&spoken, &train, &NoCrystal).expect("the rule crosses");
    let unread = crate::verdict::Refusal::of_tape(crate::verdict::REFUSE_UNREAD, &crossed);
    assert!(matches!(NoRule::of_refusal(unread), NoRule::Unread), "unread is unjudged");
    let refused = crate::verdict::Refusal::of_tape(crate::verdict::REFUSE_NOT_A_SENTENCE, &crossed);
    assert!(matches!(NoRule::of_refusal(refused), NoRule::Refused(r) if r.reason == crate::verdict::REFUSE_NOT_A_SENTENCE));
    let withheld = vec![(grid(&["1100", "0000"]), grid(&["0022", "0000"]))];
    let mut k = Court::new();
    assert!(matches!(k.judge_solve(&crossed, &withheld), SolveVerdict::Solved { .. }), "the court rules the word");
}

/// 78.157 §3a — THE HOME CHECK RESOLVES COURT-STAMPED WORDS ONLY. A rule
/// that cites a word this mind stood in its dialect (no court admitted it)
/// is CITES UNHELD at home, by the cited address — never a pass; once a
/// court's stamp files the same word, the citation resolves and the home
/// check goes on to the replay.
#[test]
fn would_judge_refuses_citation_no_court_admitted() {
    use crate::client::sdk::Epigenome;
    use crate::transducer::family::{cipher_form, sentence_measure};
    let family = vec![
        (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
        (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
    ];
    let held = cipher_form(&family[0].0, &family[0].1);
    let mag = sentence_measure(&held, &family[..1], &NoCrystal, 0).expect("the body is spoken");
    let cited = crate::transducer::sentence::as_element_form(&held, mag).expect("a tape");
    let mut dialect = Epigenome::new();
    dialect.stand_dialect_at(cited.clone(), 1, 1).expect("stood");
    let rule = crate::transducer::family::extract_rule_with(&family, &dialect, &mut super::common::budget())
        .expect("the family extracts");
    let spoken = crate::transducer::family::mint_sentence_why(&rule, &family, &dialect, 0).expect("and mints");
    assert!(
        spoken.receipts().cited_addresses().iter().any(|a| *a == cited.address()),
        "the word cites the form the mind stood"
    );
    let court_words = |a: &[u8]| dialect.court_word(a).and_then(|w| w.form().cloned());
    let home = would_judge(&spoken, &family, &court_words, &dialect);
    assert!(
        matches!(&home, Err(NoRule::CitesUnheld(a)) if a.as_slice() == cited.address().as_bytes()),
        "a dialect word is no citation the court holds: {home:?}"
    );
    // The same word, admitted at a court's stamp: the citation resolves.
    let mut admitted = Epigenome::new();
    admitted.learn(&cited.to_bytes().expect("the record writes"), &mut super::common::budget()).expect("learned");
    admitted.admit_all_at(crate::chronometry::Stamp::new(1, [7; 32], crate::chronometry::Place::GENESIS));
    let court_words = |a: &[u8]| admitted.court_word(a).and_then(|w| w.form().cloned());
    let home = would_judge(&spoken, &family, &court_words, &admitted);
    assert!(!matches!(home, Err(NoRule::CitesUnheld(_))), "a court-stamped word resolves: {home:?}");
}
