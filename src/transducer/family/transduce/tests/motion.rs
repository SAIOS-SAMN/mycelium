// D3.1, D3.2 — ACT: the signed permutations of the phases as the pair
// motions, generated at every rank, and the one obstruction they meet, at
// projection.

use super::{at, extents, sample, sample_by};
use crate::transducer::family::transduce::*;
use crate::transducer::sample::Sample;

/// `(t₀, t₁) ↦ (−t₁, t₀)`: the frame turned by a quarter, counterclockwise
/// as drawn with axis 0 down and axis 1 across.
fn quarter() -> SignedPermutation {
    SignedPermutation::new(vec![(1, Sense::Reversed), (0, Sense::Kept)]).expect("a signed permutation")
}

/// `(t₀, t₁) ↦ (t₁, −t₀)`.
fn three_quarters() -> SignedPermutation {
    SignedPermutation::new(vec![(1, Sense::Kept), (0, Sense::Reversed)]).expect("a signed permutation")
}

/// `(t₀, t₁) ↦ (−t₀, −t₁)`.
fn half() -> SignedPermutation {
    SignedPermutation::new(vec![(0, Sense::Reversed), (1, Sense::Reversed)]).expect("a signed permutation")
}

/// `(t₀, t₁) ↦ (t₁, t₀)`.
fn exchange() -> SignedPermutation {
    SignedPermutation::new(vec![(1, Sense::Kept), (0, Sense::Kept)]).expect("a signed permutation")
}

/// The frame turned by a quarter counterclockwise as drawn, site by site:
/// the independent oracle the motion is held to.
fn turned_quarter(s: &Sample) -> Sample {
    let [h, w] = extents(s)[..] else { panic!("rank 2") };
    sample_by(&[w, h], |t| at(s, &[t[1], w as i64 - 1 - t[0]]))
}

/// The frame with its two axes exchanged, site by site.
fn exchanged(s: &Sample) -> Sample {
    let [h, w] = extents(s)[..] else { panic!("rank 2") };
    sample_by(&[w, h], |t| at(s, &[t[1], t[0]]))
}

/// THE EIGHT OF RANK 2 ARE THE SEAM-KEEPING ACTIONS (D3.2 §D.2): each is its
/// own matrix read back, the eight matrices are distinct and each a signed
/// permutation; a shear and a scale move the seam set and refuse `Oblique`.
#[test]
fn signed_permutations_are_the_seam_keeping_actions() {
    let mut seen = Vec::new();
    for s in SignedPermutation::all(2) {
        let m = s.matrix();
        assert_eq!(SignedPermutation::of_matrix(&m), Ok(s.clone()));
        for line in &m {
            assert_eq!(line.iter().filter(|&&x| x != 0).count(), 1, "one signed generator per line: {m:?}");
        }
        assert!(!seen.contains(&m), "the eight are distinct");
        seen.push(m);
    }
    assert_eq!(seen.len(), 8);
    assert_eq!(SignedPermutation::of_matrix(&[vec![1, 1], vec![0, 1]]), Err(TransductionRefusal::Oblique), "a shear moves the seam");
    assert_eq!(SignedPermutation::of_matrix(&[vec![2, 0], vec![0, 1]]), Err(TransductionRefusal::Oblique), "a scale is a re-period, not a motion");
    assert_eq!(SignedPermutation::of_matrix(&[vec![1, 0], vec![1, 0]]), Err(TransductionRefusal::Oblique), "two phases onto one");
}

/// THE GROUP OF RANK `n` IS GENERATED, `2ⁿ·n!` ELEMENTS (the hyperoctahedral
/// group), for `n = 1..4`: the identity first, every element distinct and
/// its own matrix read back, the inverse composing to the identity, and each
/// keeps the seams: every turn at a seam (`0` or the whole turn) of every
/// phase is carried to a seam, and the turn of order two to itself.
#[test]
fn signed_permutations_generated_for_rank() {
    for (n, count) in [(1usize, 2usize), (2, 8), (3, 48), (4, 384)] {
        let all: Vec<SignedPermutation> = SignedPermutation::all(n).collect();
        assert_eq!(all.len(), count, "2ⁿ·n! at rank {n}");
        assert_eq!(SignedPermutation::all(n).next(), Some(SignedPermutation::identity(n)));
        for (i, g) in all.iter().enumerate() {
            assert!(!all[..i].contains(g), "distinct at rank {n}: {g:?}");
            assert_eq!(SignedPermutation::of_matrix(&g.matrix()).as_ref(), Ok(g));
            assert_eq!(g.then(&g.inverse()), Ok(SignedPermutation::identity(n)));
            // Periods 2, 3, 4, 5 on the phases: the seams, then the middle.
            let periods: Vec<usize> = (0..n).map(|k| k + 2).collect();
            for corner in 0..(1usize << n) {
                let turns: Vec<Turn> =
                    periods.iter().enumerate().map(|(k, &p)| Turn::of(((corner >> k) & 1) * p, p)).collect();
                let moved = g.carry(&turns).expect("one rank");
                assert!(moved.iter().all(Turn::is_seam), "{g:?} keeps the seams");
            }
            let middle: Vec<Turn> = (0..n).map(|_| Turn::of(1, 2)).collect();
            assert_eq!(g.carry(&middle), Ok(middle.clone()), "the turn of order two stands");
        }
    }
}

/// THE ALPHABET'S 90° LETTER ON THE SENSOR'S PLANE IS A SIGNED PERMUTATION,
/// read off its images of `e₀` and `e₁`; the `(3, 4, 5)` letter on the same
/// plane is not, and has no action on turns.
#[test]
fn signed_permutation_reads_the_alphabet_letters() {
    use crate::transducer::alphabet::{LetterId, Word};
    // The plane of the generators `e₀` and `e₁`, which `Word::plane_block`
    // reads.
    let plane = (0, 1);
    let turn = Word::from_letters(&[(LetterId::turn_90(plane).expect("a letter"), false)]);
    let read = SignedPermutation::of_word(&turn).expect("a 90° letter is a signed permutation");
    assert!(read == quarter() || read == three_quarters(), "{read:?}");
    let twice = Word::from_letters(&[(LetterId::turn_90(plane).expect("a letter"), false); 2]);
    assert_eq!(SignedPermutation::of_word(&twice), Ok(half()));
    let pythagorean = Word::from_letters(&[(LetterId::pythagorean_5(plane).expect("a letter"), false)]);
    assert_eq!(SignedPermutation::of_word(&pythagorean), Err(TransductionRefusal::Oblique));
}

/// A QUARTER TURN BETWEEN FRAMES OF DIFFERENT EXTENT IS EXACT ON TURNS: a
/// `3 × 5` frame turned lands on a `5 × 3` output site for site, and the same
/// turn followed by the re-period onto `10 × 6` lands too (every moved
/// boundary `k/5` and `k/3` lands on periods 10 and 6), each output
/// compartment the turned site's block.
#[test]
fn turn_90_between_unequal_frames_is_exact() {
    let x = sample(&["12000", "10300", "00004"]);
    let e = Enclosed::of(&x);
    let periods = |p: Vec<usize>| Enclosure::new(p).expect("periods stand");
    assert_eq!(quarter().spoken_onto(&e, &periods(vec![5, 3])), Ok(turned_quarter(&x)));
    assert_eq!(exchange().spoken_onto(&e, &periods(vec![5, 3])), Ok(exchanged(&x)));
    let doubled = quarter().spoken_onto(&e, &periods(vec![10, 6])).expect("the turns land");
    let turned = turned_quarter(&x);
    for site in doubled.sites() {
        let t = site.axes();
        assert_eq!(at(&doubled, t), at(&turned, &[t[0] / 2, t[1] / 2]), "site {t:?}");
    }
    let moved = quarter().moved(&e).expect("the frame moves");
    assert_eq!(moved.enclosure, periods(vec![5, 3]));
    assert_eq!(moved, Enclosed::of(&turned_quarter(&x)));
}

/// THE ONE OBSTRUCTION IS AT PROJECTION, NAMED: the same `3 × 5` frame
/// turned onto `4 × 3` moves its first band's arc `[0, 1/5)` of the second
/// phase onto the first phase reversed, `[4/5, 1)`, and the turn `4/5` lands
/// at `16/5` of the output's first phase: `OffSite`, never rounded. The
/// record in turns stands regardless.
#[test]
fn turn_90_onto_unequal_periods_refuses_off_site() {
    let x = sample(&["12000", "10300", "00004"]);
    let e = Enclosed::of(&x);
    let periods = |p: Vec<usize>| Enclosure::new(p).expect("periods stand");
    assert_eq!(
        quarter().spoken_onto(&e, &periods(vec![4, 3])),
        Err(TransductionRefusal::OffSite { turn: Turn::of(4, 5), period: 4 })
    );
    let moved = quarter().moved(&e).expect("the frame moves");
    let y = sample(&["000", "040", "300", "021"]);
    let need = need(&periods(vec![5, 3]), &periods(vec![4, 3])).expect("periods stand");
    let t = Transduction::of(&moved.spoken_onto(&moved.enclosure).expect("its own periods"), &y, &mut super::stroke(need));
    assert!(t.is_ok(), "the incidences in turns read: {t:?}");
}
