// 12.6, 26 — DESCENT: what a rule measures when it speaks a whole family,
// and the pressure a bundle is under.
//
//! Descent CERTIFIES: it says whether the lanes fell, never which candidate
//! to take. A miss is zero, because association with a listing is not a
//! mint.

use super::*;

/// Descent a form measures by speaking every train pair through the
/// crystal that resolves its cited forms. Per pair: deflation is
/// `measure_lanes(delta(x, y)) − measure_lanes(delta(spoken, y))`, accepted
/// only if the spoken residual descends on that pair, else the pair
/// contributes zero. Summed over pairs. The per-pair `residual_of(spoken, y)`
/// chains sit beside the sum. A form is spoken only when every remainder
/// is empty; `REFUSE_NO_DESCENT` means the form closes nothing.
pub fn form_descent(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> ([u64; 4], Vec<crate::transducer::germinate::ResidualGerm>) {
    if family.is_empty() {
        return ([0; 4], Vec::new());
    }
    let mut pay = [0u64; 4];
    let mut remainders = Vec::with_capacity(family.len());
    for (x, y) in family {
        let before_chain = crate::arc::refine_delta(x, y)
            .map(|(c, _, _)| c)
            .unwrap_or_else(|| residual_of(Some(x), y));
        let spoken = apply_form(form, x, &crate::transducer::alphabet::Word::identity(), crystal);
        let grid = spoken.as_ref().map(|s| s.grid());
        let after_chain = residual_of(grid.as_ref(), y);
        let before_mu = element::measure_lanes(&before_chain);
        let after_mu = element::measure_lanes(&after_chain);
        if crate::economy::descends(&after_mu, &before_mu) {
            for i in 0..4 {
                pay[i] = pay[i].saturating_add(before_mu[i].saturating_sub(after_mu[i]));
            }
        }
        remainders.push(after_chain);
    }
    (pay, remainders)
}

/// Mint the family's sentence: μ⃗ = [S, G, T, H] of the closed cochain.
/// The rule as the word a voice speaks and submits: the form's v3 tape
/// carrying the descent it measures over the bundle through the crystal
/// that resolves its citations. Lane 𝒯 is the scene-state span the bundle
/// stood over. A form with no measured descent is not a word.
pub fn mint_sentence(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> Option<element::Element> {
    let (mag, remainders) = form_descent(&rule.form, family, crystal);
    if remainders.iter().any(|r| !r.is_empty()) {
        return None;
    }
    crate::transducer::sentence::as_element_form(&rule.form, mag)
}

/// Pressure of a form spoken over its germinating bundle (A1, 12.6):
/// per pair, `measure_lanes` of that pair's own residual; over the bundle,
/// the lane-wise sum (the pairs are disjoint scenes). T is the extent
/// held, passed in. None when there is no bundle to measure.
pub fn bundle_pressure(
    form: &Form,
    bundle: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    t_extent: u64,
) -> Option<(Vec<[u64; 4]>, [u64; 4])> {
    if bundle.is_empty() {
        return None;
    }
    let mut pairs = Vec::with_capacity(bundle.len());
    let mut sum = [0u64; 4];
    for (x, y) in bundle {
        let spoken = apply_form(form, x, &crate::transducer::alphabet::Word::identity(), crystal);
        let grid = spoken.as_ref().map(|s| s.grid());
        let residual = residual_of(grid.as_ref(), y);
        let m = element::measure_lanes(&residual);
        for l in 0..4 {
            sum[l] = sum[l].saturating_add(m[l]);
        }
        pairs.push(m);
    }
    sum[2] = t_extent;
    Some((pairs, sum))
}

/// A candidate is refused if it raises any lane on any pair, even when the
/// summed μ⃗ descends. Equal or `descends` on every pair.
pub(crate) fn pairs_descend(before: &[[u64; 4]], after: &[[u64; 4]]) -> bool {
    if before.len() != after.len() {
        return false;
    }
    before
        .iter()
        .zip(after)
        .all(|(b, a)| *a == *b || crate::economy::descends(a, b))
}
