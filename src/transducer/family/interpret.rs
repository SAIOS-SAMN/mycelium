// 78.81, stripped by 78.90 — THE REMAINDER INVERTED, ON THE VOICE: what a
// form leaves unsaid of its family is perceived as a scene of its own, in
// the act that spoke it, and the generators that produce it join the form
// where speech then leaves less unsaid (78.76). That is a FACULTY of the
// mind — noise turned into signal by its own perception, judged by its own
// speech — and it is the whole of this module.
//
//! The readers that once rode here — light, confluence, single changes, the
//! recolour keys, the reconstructions by holonomy — were the builder's
//! instruments over the corpus, reporting to the builder (78.90: the
//! progenitor builds the mind's faculties, never his measuring tools into
//! it). They stand in `mycelium-soak`, read off the same library functions,
//! and no longer ride the mind's stroke.

use super::invert::{invert_measured, Inversion, Painted};
use super::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

/// WHAT THE MIND'S INVERSION DID WITH ONE FAMILY'S REMAINDER: none where
/// nothing was unsaid.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Interpretation {
    pub inversion: Option<Inversion>,
}

/// A form the voice speaks from here on, with the remainders it leaves.
pub type Spoken = (Form, Vec<Vec<(i64, i64, u8, u8)>>);

/// INVERT A SPOKEN FORM'S REMAINDER: the joined form with its remainders
/// where the speech accepted the inversion, and what the inversion read.
/// Paid from the voice's stroke, through the remainder scene's extraction.
pub fn interpret(
    form: &Form,
    family: &[(Grid, Grid)],
    remainders: &[Vec<(i64, i64, u8, u8)>],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> (Option<Spoken>, Interpretation) {
    let Some((inversion, joined)) = invert_measured(form, family, remainders, crystal, budget, Painted::Bodies) else {
        return (None, Interpretation::default());
    };
    note_read(&inversion);
    (joined, Interpretation { inversion: Some(inversion) })
}

const SLOTS: usize = 7;
static TALLY: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];

fn note(slot: usize, by: u64) {
    if by > 0 && TALLY[slot].fetch_update(Relaxed, Relaxed, |held| held.checked_add(by)).is_err() {
        let mut past = u64::MAX;
        let _ = crate::physics::metabolism::step_or_refuse(&mut past, by);
    }
}

fn note_read(i: &Inversion) {
    let flat = [1, u64::from(i.accepted), u64::from(i.closed), u64::from(i.unextracted), i.before, i.after, i.joined];
    for (slot, by) in flat.into_iter().enumerate() {
        note(slot, by);
    }
}

/// THE MIND'S INVERSIONS since the process began: families whose remainder
/// it inverted, the inversions its speech accepted and those that closed the
/// family, the remainder scenes that extracted nothing, the links before and
/// after, and the generators joined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct InterpretTally {
    pub families: u64,
    pub accepted: u64,
    pub closed: u64,
    pub unextracted: u64,
    pub before: u64,
    pub after: u64,
    pub joined: u64,
}

pub fn interpret_tally() -> InterpretTally {
    let r = |i: usize| TALLY[i].load(Relaxed);
    InterpretTally { families: r(0), accepted: r(1), closed: r(2), unextracted: r(3), before: r(4), after: r(5), joined: r(6) }
}
