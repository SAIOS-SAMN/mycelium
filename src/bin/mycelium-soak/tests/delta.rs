use crate::delta::*;
use mycelium::client::sdk::Epigenome;

/// Y1 — THE INSTRUMENT MUST BE ABLE TO SEE A DIFFERENCE, or its zero
/// on the corpus says nothing about the engine and everything about
/// itself. The corpus reads `standable 0`, so the twelve exercise only
/// the arm where no warm mind can be built; this builds the other arm
/// by hand.
///
/// A mind is given one closed form and asked to read a family. The
/// same family is read by a mind holding nothing. `TaskWalk::of` is
/// the whole of what the delta compares, so proving it answers
/// differently for two different minds on one family is proving the
/// comparison is live — and proving it answers IDENTICALLY for two
/// identical minds is proving the reading is a function of the mind
/// and not of anything else in the walk.
#[test]
fn task_walk_of_is_function_of_given_mind() {
    // A recolour: one body, one hue changed. The simplest family there
    // is, built through the instrument's OWN public path so the test
    // exercises what the walk exercises and nothing else.
    let a = vec![vec![1u8, 1], vec![1, 1]];
    let b = vec![vec![2u8, 2], vec![2, 2]];
    let fam = vec![(a, b)];

    let empty = Epigenome::new();
    let cold = TaskWalk::of(&fam, &empty);
    // TWO EMPTY MINDS READ ALIKE. Without this, a difference below
    // could be noise in the walk rather than the mind.
    let cold_again = TaskWalk::of(&fam, &Epigenome::new());
    assert!(cold == cold_again, "one family, two identical minds, one reading");

    // A mind holding this family's own closed form — the same form a
    // warm mind is built out of on a real slice.
    let (stand, why) = standable(&fam, &empty);
    let Some(e) = stand else {
        panic!(
            "the simplest recolour yields a closed form, or nothing can ever be stood; \
             the chain stopped at {}",
            why.label()
        )
    };
    let mut warm = Epigenome::new();
    assert!(
        warm.stand_dialect_at(e, 1, 1).is_some(),
        "the mind stands the form it is given, or this test proves nothing"
    );
    let hot = TaskWalk::of(&fam, &warm);

    // THE INSTRUMENT SEES IT. What moves is the engine's business —
    // a word vertex where there was a demand, a glued citation, a
    // closure — and the assertion is that SOMETHING does, because a
    // reading that cannot move is not a measurement.
    assert!(
        hot != cold,
        "a mind holding the form reads the family differently from a mind holding nothing; \
         cold words {} glued {} demands {} closed {}, warm words {} glued {} demands {} closed {}",
        cold.words, cold.glued, cold.demands, cold.closed,
        hot.words, hot.glued, hot.demands, hot.closed
    );
}
