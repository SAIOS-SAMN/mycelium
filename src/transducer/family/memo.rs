// 21 step 2, 50.2 step 2 — THE PERCEPT MEMO AND THE SECOND STEP. The
// peeled pair form is memoised on sensor bytes; the derivation against a
// mind's presentation runs per call and is never cached.
//
//! Split from `pair.rs` at the §1.1 600-line reading bound; the items are
//! moved, not rewritten (§1.6).

use super::*;
use std::cell::RefCell;

thread_local! {
    static PAIR_FORM_MEMO: RefCell<Option<PairFormMemo>> = const { RefCell::new(None) };
}

/// THE PERCEPT, MEMOISED — and nothing else. The grids are sensor bytes
/// at the sensor boundary, which is the one place bytes may be a key
/// (rule 6); what the memo holds is the PEELED form, every word vertex
/// spelled from the identity by A22 alone. The derivation is not in here
/// and must not be: it is a function of the mind that is speaking, and a
/// presentation is content, never a cache key. Keyed on a root of the
/// atlas, a digest decided whether F re-derives.
pub(crate) struct PairFormMemo {
    a: Grid,
    b: Grid,
    scale: Option<((usize, usize), (usize, usize))>,
    form: Form,
}

/// Perception of one pair as a form. Cells are read once; a second call on
/// the same grids spends none. Shells pair and are never vertices.
pub fn pair_form(
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Option<Form> {
    let hit = PAIR_FORM_MEMO.with(|m| {
        m.borrow().as_ref().and_then(|held| {
            if held.a == *a && held.b == *b && held.scale == scale {
                Some(held.form.clone())
            } else {
                None
            }
        })
    });
    let peeled = match hit {
        Some(f) => f,
        None => {
            let form = pair_form_uncached(a, b, scale, budget);
            // 50.8: the cover is part of the PERCEPT and is built under
            // this budget. A budget that ended carried fewer pieces, so
            // the reading is incomplete — and an incomplete percept is
            // not memoised, or the next caller with a fuller stroke would
            // be handed this one's shortfall as if it were the scene
            // (rule 11: unmeasured is not a smaller answer).
            if budget.remaining() > 0 {
                PAIR_FORM_MEMO.with(|m| {
                    *m.borrow_mut() = Some(PairFormMemo {
                        a: a.clone(),
                        b: b.clone(),
                        scale,
                        form: form.clone(),
                    });
                });
            }
            form
        }
    };
    derive_against(peeled, crystal, budget)
}

/// 50.2 step 2 — THE SECOND STEP, RUN PER CALL AND PER MIND. The peeled
/// form's word vertices are spelled from the identity; each is factored
/// through the presentation this crystal holds, and the vertex carries
/// what came back. No root and no key: the mind IS the argument.
///
/// `None` is a vertex the budget could not measure. It is not a form
/// with that vertex spelled — that would assert the presentation reached
/// nothing when nothing was looked at (rule 11) — so the whole form is
/// unmeasured and the caller withholds and resumes next stroke (L8w).
fn derive_against(
    mut peeled: Form,
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<Form> {
    for v in &mut peeled.vertices {
        let Vertex::Word { derivation, .. } = v else {
            continue;
        };
        *derivation = derivation_of_spelled(crystal, budget, derivation)?;
    }
    // `reindex` reads the composed word of every word vertex, so it runs
    // after the derivations land and not before.
    peeled.reindex();
    Some(peeled)
}
