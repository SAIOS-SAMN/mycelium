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
    for (pi, (x, y)) in family.iter().enumerate() {
        // The exact cochain where the pair refines on K_lcm; the raw residual
        // of the two grids where it does not. Both are measured on the pair.
        let before_chain = match crate::transducer::family::refine::refine_delta(x, y) {
            Some(chain) => chain,
            None => residual_of(Some(x), y),
        };
        let spoken = apply_form_on(form, x, placement, crystal, Some(pi));

        // A speech that does not fit its own frame lays down no grid, so the
        // residual is measured against what the form actually said: nothing.
        let grid = spoken.as_ref().and_then(|s| s.grid());
        let after_chain = residual_of(grid.as_ref(), y);
        let before_mu = element::measure_lanes(&before_chain);
        let after_mu = element::measure_lanes(&after_chain);
        // S9.5 — THE PAY GATE IS CLOSURE. A pair pays when the form CLOSES
        // it: the remainder is empty and what the pair demanded is what the
        // form spoke. Under 50.5 (Y3), a sentence with demand vertices is
        // valid, never closed: its closed sub-complex pays for the deflation
        // its closed words achieved: `deflation_of(lane_moves(after, before))`.
        let closed =
            crate::economy::deflation_of(&crate::economy::lane_moves(&after_mu, &before_mu));
        if after_chain.is_empty() || form.demand_vertices().next().is_some() {
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
///
/// Under 50.5 (Y3): A sentence with word vertices and demand vertices mints
/// on its closed sub-complex: the amount is the deflation its closed words
/// achieved and nothing else. A sentence of zero word vertices mints nothing.
pub fn mint_sentence(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    t_extent: u64,
) -> Option<element::Element> {
    // A FORM NOTHING IN WHICH ACTS IS NOT UTTERED. The court's clause is
    // already exactly this — "a tape whose only vertices are the Frame and
    // demands is a chain wearing a form's tape" (23.1 step 3, 50.6/L7,
    // `REFUSE_NOT_A_SENTENCE`) — and this is the speaker's side of it, reading
    // the same `Form::acts()`.
    // FOUR REFUSALS, ONE MESSAGE (§8.1, measurement only — the gates are
    // unchanged). `mint refused (descent, carry or demands)` is the
    // dominant fate of a form on this corpus: measured over one recut,
    // 10799 of 13730 withheld forms and 79% of everything perceived, against
    // 45 forms spoken. Four different things reach that line and it names
    // none of them, so the largest reading on the wire says only that
    // something upstream of the court said no.
    if !rule.form.acts() {
        eprintln!("  mint refused: nothing in the form acts");
        return None;
    }
    if let Some(why) = crate::court::verdict::body::vertices_carry_family(&rule.form, family) {
        eprintln!("  mint refused: a vertex does not carry its family ({why:?})");
        return None;
    }
    // 50.5 — WHAT THE FORM LEAVES UNSPOKEN ON THIS FAMILY IS ITS DEMAND.
    // The remainders are measured once here and read by the measure below
    // (law 12): a demand vertex acts on nothing, so the declared form
    // speaks exactly what the undeclared one spoke and leaves the same
    // remainder — which is what `undeclared_remainder` reads at the court.
    let (pay, remainders) = form_deflation(&rule.form, family, crystal);
    let declared = declare_unspoken(&rule.form, family, &remainders);
    let form = match declared.as_ref() {
        Some(f) => f,
        None => &rule.form,
    };
    let Some(mag) = sentence_measure_from(form, family, t_extent, pay, &remainders) else {
        eprintln!("  mint refused: the form speaks a cell its family refuses");
        return None;
    };
    let out = crate::transducer::sentence::as_element_form(form, mag);
    if out.is_none() {
        eprintln!("  mint refused: the form has no tape at {mag:?}");
    }
    out
}

/// 50.5 — THE UNSPOKEN REMAINDER, DECLARED. A sentence is "the form saying
/// out loud what it could not close and asking the market to answer it",
/// and until now a form said it only for the pairs its budget never
/// reached (`finish`) and the multiplicities its correspondence could not
/// resolve. What the form's own words left unspoken on the family it was
/// minted over — the cells its vertices reach on no pair — was declared by
/// nothing, so `sentence_measure` refused the form and the voice withheld
/// it: 16,535 `the sentence measures nothing over its bundle` in 5½ hours
/// of 2cf578a, against 102 forms spoken, and a book that held one to three
/// listings because nothing reached it.
///
/// A remainder cell is a demand only where the form SPOKE NOTHING there
/// (`spoken == 0`, `residual_of`'s reading of absence): the court's own
/// `undeclared_remainder` admits exactly that cell against a demand naming
/// its observed hue, and refuses a cell the form spoke wrongly whatever a
/// demand says of it. So the declared cells are the unspoken ones, each
/// with the hue the input held there (absence is hue 0 in this encoding)
/// and the hue the pair asks; a cell the form spoke wrongly stays
/// undeclared and the form is refused by name.
///
/// Per pair, on the chart the unspoken cells stand on, as `Source` with
/// the multiplicity of its pieces — the same construction the unreached
/// arm of `finish` makes for a whole pair, so one kind of demand stands
/// for one kind of fact. A form already carrying that demand on that chart
/// extends it; a second vertex under one key would be a form that says one
/// thing twice.
///
/// `None` where nothing was unspoken: the form as it stands is the form.
pub fn declare_unspoken(
    form: &Form,
    family: &[(Grid, Grid)],
    remainders: &[crate::transducer::germinate::ResidualGerm],
) -> Option<Form> {
    use crate::transducer::sentence::{demand_key, pieces, Cell, Demand};
    // THE COURT'S OWN READER SAYS WHAT IS UNDECLARED (law 12): a cell a
    // demand the extraction already emitted names is declared, and naming
    // it again under `Source` would be one fact said twice. Sorted and
    // deduplicated by that reader, so membership is a search and not a
    // walk of the form.
    let undeclared = crate::court::verdict::body::undeclared_remainder(remainders, form);
    if undeclared.is_empty() {
        return None;
    }
    let mut declared = form.clone();
    let mut any = false;
    for ((x, _), rem) in family.iter().zip(remainders) {
        let cells: Vec<Cell> = rem
            .iter()
            .filter(|&&(r, c, spoken, observed)| {
                spoken == 0 && undeclared.binary_search(&(r, c, spoken, observed)).is_ok()
            })
            .map(|&(r, c, _, observed)| {
                // The hue the input holds at the cell. Outside the input
                // frame there is none, and absence is hue 0 in this
                // encoding — `residual_of`'s own reading, not a floor.
                let before = match (usize::try_from(r), usize::try_from(c)) {
                    (Ok(ur), Ok(uc)) => match x.get(ur).and_then(|row| row.get(uc)) {
                        Some(h) => *h,
                        None => 0,
                    },
                    _ => 0,
                };
                (r, c, before, observed)
            })
            .collect();
        if cells.is_empty() {
            continue;
        }
        let chart = crate::transducer::stalk::invariant(&cells);
        let kind = Demand::Source { multiplicity: pieces(&cells).len() as u32 };
        if kind.typed().is_err() {
            continue;
        }
        let key = demand_key(&kind, &chart);
        let held = declared.demands.get(&key).copied();
        match held.and_then(|i| declared.vertices.get_mut(i as usize)) {
            Some(Vertex::Demand { bodies, .. }) => {
                bodies.extend(cells);
                bodies.sort_unstable();
                bodies.dedup();
            }
            _ => declared.vertices.push(demand_vertex(kind, &chart, cells)),
        }
        declared.reindex();
        any = true;
    }
    any.then_some(declared)
}

/// What a sentence measures over its bundle (50.5). A sentence with no
/// demand vertex measures its deflation and is spoken only when every pair
/// closes; a sentence with demand vertices is spoken whole: its closed
/// sub-complex measures the deflation its words performed on every pair
/// — before less after per lane, whether or not the pair closed — and its
/// demands measure the cells they stand for, so a sentence that closes
/// nothing yet still carries what it asks for. `None` where the form leaves
/// a remainder no demand vertex declares: that is a withheld form, never a
/// word.
///
/// A DEMAND DECLARES CELLS, NOT AN EXEMPTION. This asked only whether the
/// form had ANY demand vertex, and a form that had one was measured whole
/// however much of its remainder that demand did not name — while the court
/// asks `undeclared_remainder`, which is whether the demands actually cover
/// what was left. So a form declaring one cell and leaving twenty was minted
/// by the voice and refused by the court, and `form leaves a remainder on the
/// family` is the commonest refusal on the fleet: 504 of 1018 at e98e8a2.
///
/// It is the COURT'S OWN reader that decides it here (law 12). A speaker
/// computing "is this remainder declared" by different arithmetic would agree
/// with the court by luck, which is the failure mode 23.1 step 3 exists to
/// catch — and the court still re-derives and does not take this for an
/// answer.
/// LANE 𝒯 IS THE SPAN, AND IT IS MEASURED HERE OR NOWHERE. `mint_sentence`'s
/// own line has always read "Lane 𝒯 is the scene-state span the bundle stood
/// over", and this computed μ⃗ through `measure_lanes` alone — which returns
/// a LITERAL ZERO on lane 2 under a comment deferring the measurement to the
/// mint. The mint did not take it either, so every word this engine has ever
/// spoken carried `[S, G, 0, H]`: 197 of 197 spoken magnitudes at 820678a,
/// none with a 𝒯 component.
///
/// The consequence is the whole lane vector. `word.lanes()[2]` is false, so
/// the court's `physics.spend(word.lanes())` never spends 𝒯, so `reserve[2]`
/// never depletes, so `reserve_deficit[2]` is zero, so `yield_surface[2]` is
/// zero, so the dwell's 𝒯 lane read 0 in 1247 of 1247 readings. Law 5 has
/// four lanes so they can carry independent content; three carried the same
/// reserve deficit and the fourth carried nothing.
///
/// `t_extent` is the span in RECEIPTS the bundle stood over — the same
/// quantity `pivot_loop` already takes and `bundle_pressure` already writes
/// to `sum[2]`. A form minted in the stroke it was extracted in stood over
/// no receipts and measures zero, which is a reading and not a placeholder.
pub fn sentence_measure(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    t_extent: u64,
) -> Option<[u64; 4]> {
    let (pay, remainders) = form_deflation(form, family, crystal);
    sentence_measure_from(form, family, t_extent, pay, &remainders)
}

/// The measure, from a deflation already taken: the mint measures the
/// remainders once, declares what was unspoken, and measures the declared
/// form against the same remainders (law 12) — a demand vertex acts on
/// nothing, so the deflation is the same form's.
pub fn sentence_measure_from(
    form: &Form,
    family: &[(Grid, Grid)],
    t_extent: u64,
    pay: [u64; 4],
    remainders: &[crate::transducer::germinate::ResidualGerm],
) -> Option<[u64; 4]> {
    if form.demand_vertices().next().is_none() {
        if !remainders.iter().all(|r| r.is_empty()) {
            return None;
        }
        let mut mag = pay;
        mag[2] = t_extent;
        return Some(mag);
    }
    if !crate::court::verdict::body::undeclared_remainder(remainders, form).is_empty() {
        return None;
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
    for ((x, y), after_chain) in family.iter().zip(remainders) {
        let before_chain = match crate::transducer::family::refine::refine_delta(x, y) {
            Some(chain) => chain,
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
    // The span is the bundle's, not a sum over its pairs: a family standing
    // three frames does not age three times (51.9). It is written, not added.
    mag[2] = t_extent;
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
    for (pi, (x, y)) in bundle.iter().enumerate() {
        let spoken = apply_form_on(form, x, &crate::transducer::alphabet::Word::identity(), crystal, Some(pi));

        // A speech that does not fit its own frame lays down no grid, so the
        // residual is measured against what the form actually said: nothing.
        let grid = spoken.as_ref().and_then(|s| s.grid());
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
