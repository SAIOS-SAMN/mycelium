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
//! development builds the mind's faculties, never the builder's measuring
//! tools into it). They stand in `mycelium-soak`, read off the same library functions,
//! and no longer ride the mind's stroke.

use super::invert::{invert_measured, Inversion, Painted};
use super::*;

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
    (joined, Interpretation { inversion: Some(inversion) })
}
