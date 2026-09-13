// 12.6, 26 — THE MEASURE OVER A BUNDLE: what a rule measures when it speaks
// a whole family, and the pressure a bundle is under.
//
//! Closure certifies (S9.5): a pair pays when the form closes it, and what
//! a form measures is priced by the economy per lane — never a direction
//! and never a choice among candidates (12.6). A miss is zero, because
//! association with a listing is not a mint.

use super::*;

/// THE DEFLATION a form measures by speaking every train pair through the
/// crystal that resolves its cited forms. Per pair it is
/// `deflation_of(lane_moves(after, before))` on the lane vectors of the two
/// residuals, taken where the form CLOSES the pair and zero otherwise.
/// Summed over pairs. The per-pair `residual_of(spoken, y)` chains sit
/// beside the sum. `REFUSE_NOT_CLOSED` means a remainder was left.
///
/// IT WAS `form_descent`, AND 12.6 FORBIDS THE MOTION THAT NAME ASSERTS.
/// A lane does not descend: `lane_moves` reads what each lane did and
/// `deflation_of` prices it, which is the word the arithmetic inside this
/// function already used. The file's header was corrected at I13 and the
/// identifiers were not; this is the correction, owed since 2026-09-12.
/// §2.2 — the name is the claim.
pub fn form_deflation(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> ([u64; 4], Vec<crate::transducer::germinate::ResidualGerm>) {
    form_deflation_at(form, family, crystal, &crate::transducer::alphabet::Word::identity())
}

/// The same deflation spoken AT A PLACEMENT (26 as ruled 2026-09-10): what
/// a cited form closes where the citer put it, which is the share it
/// takes of the citer's mint. `Word::identity()` is the form speaking in
/// its own frame, which is what `form_deflation` asks for.
pub fn form_deflation_at(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    placement: &crate::transducer::alphabet::Word,
) -> ([u64; 4], Vec<crate::transducer::germinate::ResidualGerm>) {
    if family.is_empty() {
        return ([0; 4], Vec::new());
    }
    let mut pay = [0u64; 4];
    let mut remainders = Vec::with_capacity(family.len());
    for (x, y) in family {
        // The refined delta where the pair refines; the raw residual of
        // the two grids where it does not. Both are measured on the pair.
        let before_chain = match crate::arc::refine_delta(x, y) {
            Some(r) => r.delta,
            None => residual_of(Some(x), y),
        };
        let spoken = apply_form(form, x, placement, crystal);
        let grid = spoken.as_ref().map(|s| s.grid());
        let after_chain = residual_of(grid.as_ref(), y);
        let before_mu = element::measure_lanes(&before_chain);
        let after_mu = element::measure_lanes(&after_chain);
        // S9.5 — THE PAY GATE IS CLOSURE. A pair pays when the form CLOSES
        // it: the remainder is empty and what the pair demanded is what the
        // form spoke. A pair that merely descended — less remainder than
        // before, but still a remainder — pays nothing, because a partial
        // speaking is not a closed pair and the court refuses the form on
        // that remainder anyway (S9.4).
        //
        // The deflation of a closed pair is the whole delta it named:
        // `after` is empty, so `before − after` is `before`.
        if after_chain.is_empty() {
            let closed =
                crate::economy::deflation_of(&crate::economy::lane_moves(&after_mu, &before_mu));
            for (p, d) in pay.iter_mut().zip(&closed) {
                *p = p.saturating_add(*d);
            }
        }
        remainders.push(after_chain);
    }
    (pay, remainders)
}

/// Mint the family's sentence: μ⃗ = [S, G, T, H] of the closed cochain.
/// The rule as the word a voice speaks and submits: the form's v3 tape
/// carrying the deflation it measures over the bundle through the crystal
/// that resolves its citations. Lane 𝒯 is the scene-state span the bundle
/// stood over. A form with no measured deflation is not a word.
pub fn mint_sentence(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> Option<element::Element> {
    // A FORM NOTHING IN WHICH ACTS IS NOT UTTERED. The court's clause is
    // already exactly this — "a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape" (23.1 step 3, 50.6/L7,
    // `REFUSE_NOT_A_SENTENCE`) — and this is the speaker's side of it, reading
    // the same `Form::acts()`.
    //
    // Without it the voice utters the chain, the court refuses it, and the
    // refusal SEALS the address (50.9) — against a shape that was never
    // spoken, which is the fault the extraction path's own comments warn
    // about. Withholding costs the family nothing: its demands go to the
    // market to be answered, and a rule that can act again is minted then.
    //
    // D8 made this reachable: a class whose whole content is an unresolved
    // multiplicity carries no word, so a family with nothing else in it has a
    // form of Frame and demands. Measured on the 60-task slice, that is 11
    // families, up from 3.
    if !rule.form.acts() {
        return None;
    }
    let mag = sentence_measure(&rule.form, family, crystal)?;
    crate::transducer::sentence::as_element_form(&rule.form, mag)
}

/// What a sentence measures over its bundle (50.5). A sentence with no
/// demand vertex measures its deflation and is spoken only when every pair
/// closes; a sentence with demand vertices is spoken whole: its closed
/// sub-complex measures the deflation its words performed on every pair
/// — before less after per lane, whether or not the pair closed — and its
/// demands measure the cells they stand for, so a sentence that closes
/// nothing yet still carries what it asks for. `None` when a form with no
/// demands leaves a remainder: that is a withheld form, never a word.
pub fn sentence_measure(form: &Form, family: &[(Grid, Grid)], crystal: &dyn Crystal) -> Option<[u64; 4]> {
    let (pay, remainders) = form_deflation(form, family, crystal);
    if form.demand_vertices().next().is_none() {
        return remainders.iter().all(|r| r.is_empty()).then_some(pay);
    }
    let mut mag = [0u64; 4];
    // D2 — `after_chain` HAS A CARRIER AND BOTH BRANCHES READ IT. This branch
    // called `apply_form` a second time on every pair and re-derived the
    // residual the call above had already produced and returned in
    // `remainders`: one fact, two producers, and the expensive one of the two
    // (law 12). The pairs and the remainders are in the family's own order, so
    // the zip is the pairing and not a lookup.
    //
    // `before_chain` stays computed here: it is a fact about the PAIR and not
    // about the form, which is what `form_deflation` returns, so it is not the
    // same fact arriving twice.
    for ((x, y), after_chain) in family.iter().zip(&remainders) {
        let before_chain = match crate::arc::refine_delta(x, y) {
            Some(r) => r.delta,
            None => residual_of(Some(x), y),
        };
        let closed = crate::economy::deflation_of(&crate::economy::lane_moves(
            &element::measure_lanes(after_chain),
            &element::measure_lanes(&before_chain),
        ));
        for (m, d) in mag.iter_mut().zip(&closed) {
            *m = m.saturating_add(*d);
        }
    }
    let mut demanded: Vec<crate::transducer::sentence::Cell> = Vec::new();
    for (_, _, _, cells) in form.demand_vertices() {
        demanded.extend(cells.iter().copied().filter(|c| c.2 != c.3));
    }
    let asked = element::measure_lanes(&demanded);
    for (m, a) in mag.iter_mut().zip(&asked) {
        *m = m.saturating_add(*a);
    }
    Some(mag)
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

/// THE PAIRS ON WHICH A LANE ROSE, by index — the reading a trial refuses
/// on. Summing first would hide it: a bundle's total can deflate while one
/// pair pays for another, and 12.6 refuses that trade whatever the sum says.
/// The name is already the lane's own move (`lanes_risen`, `lane_moves`) and
/// keeps it; it moves here because the file does.
///
/// `None` when the two readings do not describe the same bundle (different
/// lengths). That is not "every pair rose"; it is not a comparison at all,
/// and the caller is told so rather than handed a `false` that reads like
/// one.
pub(crate) fn pairs_raised(before: &[[u64; 4]], after: &[[u64; 4]]) -> Option<Vec<usize>> {
    if before.len() != after.len() {
        return None;
    }
    Some(
        before
            .iter()
            .zip(after)
            .enumerate()
            .filter(|(_, (b, a))| {
                !crate::economy::lanes_risen(&crate::economy::lane_moves(a, b)).is_empty()
            })
            .map(|(i, _)| i)
            .collect(),
    )
}
