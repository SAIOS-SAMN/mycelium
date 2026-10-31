// D3.1, D3.2 — ACT: the eight signed swaps as the pair motions, and the one
// obstruction they meet, at projection.

use super::grid;
use crate::transducer::family::perceive::Grid;
use crate::transducer::family::transduce::*;

/// The frame turned by 90° counterclockwise as drawn, site by site: the
/// independent oracle the swap is held to.
fn turned_90(g: &Grid) -> Grid {
    let (h, w) = (g.len(), g[0].len());
    (0..w).map(|r| (0..h).map(|c| g[c][w - 1 - r]).collect()).collect()
}

/// The frame transposed, site by site.
fn transposed(g: &Grid) -> Grid {
    let (h, w) = (g.len(), g[0].len());
    (0..w).map(|r| (0..h).map(|c| g[c][r]).collect()).collect()
}

/// THE EIGHT ARE THE SEAM-KEEPING ACTIONS (D3.2 §D.2): each is its own
/// matrix read back, the eight matrices are distinct and each a signed
/// permutation; a shear and a scale move the seam set and refuse
/// `Oblique`.
#[test]
fn signed_swaps_are_the_seam_keeping_actions() {
    let mut seen = Vec::new();
    for s in SignedSwap::ALL {
        let m = s.matrix();
        assert_eq!(SignedSwap::of_matrix(m), Ok(s));
        for row in m {
            assert_eq!(row.iter().filter(|&&x| x != 0).count(), 1, "one signed generator per row: {m:?}");
        }
        assert!(!seen.contains(&m), "the eight are distinct");
        seen.push(m);
    }
    assert_eq!(SignedSwap::of_matrix([[1, 1], [0, 1]]), Err(TransductionRefusal::Oblique), "a shear moves the seam");
    assert_eq!(SignedSwap::of_matrix([[2, 0], [0, 1]]), Err(TransductionRefusal::Oblique), "a scale is a re-period, not a motion");
}

/// THE ALPHABET'S 90° LETTER ON THE SENSOR'S PLANE IS A SIGNED SWAP, read
/// off its images of `e₀` and `e₁`; the `(3, 4, 5)` letter on the same
/// plane is not, and has no action on turns.
#[test]
fn signed_swap_reads_the_alphabet_letters() {
    use crate::transducer::alphabet::{LetterId, Word, PLANE_COL, PLANE_ROW};
    let plane = (PLANE_ROW, PLANE_COL);
    let quarter = Word::from_letters(&[(LetterId::turn_90(plane).expect("a letter"), false)]);
    let read = SignedSwap::of_word(&quarter).expect("a 90° letter is a signed swap");
    assert!(read == SignedSwap::TURN_90 || read == SignedSwap::TURN_270, "{read:?}");
    let twice = Word::from_letters(&[(LetterId::turn_90(plane).expect("a letter"), false); 2]);
    assert_eq!(SignedSwap::of_word(&twice), Ok(SignedSwap::TURN_180));
    let pythagorean = Word::from_letters(&[(LetterId::pythagorean_5(plane).expect("a letter"), false)]);
    assert_eq!(SignedSwap::of_word(&pythagorean), Err(TransductionRefusal::Oblique));
}

/// A 90° TURN BETWEEN FRAMES OF DIFFERENT EXTENT IS EXACT ON TURNS: a `3 × 5`
/// frame turned lands on a `5 × 3` output site for site, and the same turn
/// followed by the re-period onto `10 × 6` lands too (every moved boundary
/// `k/5` and `k/3` lands on periods 10 and 6), each output compartment the
/// turned site's block.
#[test]
fn turn_90_between_unequal_frames_is_exact() {
    let x = grid(&["12000", "10300", "00004"]);
    let e = Enclosed::of(&x).expect("the frame stands");
    assert_eq!(SignedSwap::TURN_90.spoken_onto(&e, Enclosure { rows: 5, cols: 3 }), Ok(turned_90(&x)));
    assert_eq!(SignedSwap::TRANSPOSE.spoken_onto(&e, Enclosure { rows: 5, cols: 3 }), Ok(transposed(&x)));
    let doubled = SignedSwap::TURN_90.spoken_onto(&e, Enclosure { rows: 10, cols: 6 }).expect("the turns land");
    let turned = turned_90(&x);
    for (r, row) in doubled.iter().enumerate() {
        for (c, &hue) in row.iter().enumerate() {
            assert_eq!(hue, turned[r / 2][c / 2], "site ({r}, {c})");
        }
    }
    let moved = SignedSwap::TURN_90.moved(&e).expect("the frame moves");
    assert_eq!(moved.enclosure, Enclosure { rows: 5, cols: 3 });
    assert_eq!(moved, Enclosed::of(&turned_90(&x)).expect("the turned frame stands"));
}

/// THE ONE OBSTRUCTION IS AT PROJECTION, NAMED: the same `3 × 5` frame
/// turned onto `4 × 3` moves its first band's column arc `[0, 1/5)` onto
/// the row circle negated, `[4/5, 1)`, and the turn `4/5` lands at `16/5` of
/// the output's row circle — `OffSite`, never rounded. The record in turns
/// stands regardless.
#[test]
fn turn_90_onto_unequal_periods_refuses_off_site() {
    let x = grid(&["12000", "10300", "00004"]);
    let e = Enclosed::of(&x).expect("the frame stands");
    assert_eq!(
        SignedSwap::TURN_90.spoken_onto(&e, Enclosure { rows: 4, cols: 3 }),
        Err(TransductionRefusal::OffSite { turn: Turn::new(4, 5), period: 4 })
    );
    let moved = SignedSwap::TURN_90.moved(&e).expect("the frame moves");
    let y = grid(&["000", "040", "300", "021"]);
    let need = need(5, 3, 4, 3).expect("periods stand");
    let t = Transduction::of(&moved.spoken_onto(moved.enclosure).expect("its own periods"), &y, &mut super::stroke(need));
    assert!(t.is_ok(), "the incidences in turns read: {t:?}");
}
