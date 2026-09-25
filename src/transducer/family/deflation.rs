// 12.6, 26 — THE MEASURE OVER A BUNDLE: what a rule measures when it speaks
// a whole family, and the pressure a bundle is under.
//
//! Closure certifies (S9.5): a pair pays when the form closes it, and what
//! a form measures is priced by the economy per lane — never a direction
//! and never a choice among candidates (12.6). A miss is zero, because
//! association with a listing is not a mint.

use super::*;

/// WHY A FORM IS WITHHELD, AS A STRUCTURE RATHER THAN A STRING (69,
/// ruled 2026-09-22).
///
/// THE CLASSIFICATION ALREADY EXISTED AND WAS SPENT ON AN `eprintln!`.
/// `speak.rs`'s `why_open` read `Form::delta1()` — the Čech coboundary,
/// the exact non-abelian cocycle this engine computes over its cover —
/// formatted it into a log line, and then withheld the form identically
/// whatever it said. `delta1()` was called in ONE place in the tree, for
/// a diagnostic message.
///
/// Worse, it was not reached where it mattered: a form with a positional
/// residue was withheld on a branch ABOVE it, so the 113 forms per voice
/// that actually spoke and left links were never classified at all.
///
/// THE TWO ARMS DIFFER IN PERMANENCE, which is the same distinction V1
/// drew for `NoPose` one dimension down:
///
/// ```text
///   OrbitObstruction   δ¹ ≠ 0 — an orbit carries non-trivial holonomy, a
///                      genuine class in H¹. NO change of perspective
///                      removes it. PERMANENT: shed it at once.
///   CoboundaryResidue  δ¹ = 0 and the cochain still disagrees. The orbits
///                      all close; the form is off by a COBOUNDARY. The
///                      thought stands and the PERSPECTIVE does not.
///                      MUTABLE: Φ⁻¹ is the exact correction, and this is
///                      the one arm that must never age out.
/// ```
///
/// Order is not arbitrary: `δ¹` is asked FIRST, because a residue means
/// something different under an obstruction than under a closed cocycle.
/// A form whose orbits do not close has no perspective to shift into.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Withheld {
    /// A class in `H¹` of THIS cover. Not permanent (78.9, amending what
    /// this said): under 74 a class in `H¹` is cover-relative — C — and
    /// closes on another mind's cover; no generator of this dialect retires
    /// it, and another vantage may.
    OrbitObstruction { orbit_ix: u16, key: [u8; 2] },
    /// Off by a coboundary: the form CARRIES ORBITS, every one of them
    /// closes, and the cochain still disagrees. The thought stands and the
    /// perspective does not — `Φ⁻¹` is the correction. NOT A REFUSAL
    /// (78.9): work the mint owes, retired when the mint applies `Φ⁻¹`
    /// (rule 17); counted C by name until then, since it is cover-relative.
    CoboundaryResidue { links: usize, pairs: usize },
    /// THE QUESTION WAS NOT ASKABLE. The form carries NO ORBITS, so
    /// `delta1()` is empty and `H¹` has nothing to obstruct — the form is
    /// a tree, and every cochain on a tree is a coboundary TRIVIALLY.
    ///
    /// This arm is the vacuity fix (69, ruled 2026-09-22). The first cut
    /// folded it into `CoboundaryResidue`, which asserted "the orbits all
    /// close" of a form with no orbits — a vacuous truth wearing the
    /// stronger arm's name. Measured, it read `COBOUNDARY 34` on a corpus
    /// slice where 34 of 36 forms were acyclic, and a retention policy
    /// built on it would have called every one of them reachable.
    ///
    /// NEVER TRUE UNLESS PROVEN TRUE: an unaskable `δ¹` is not a closed
    /// `δ¹`, and the two must not answer to one name.
    Acyclic { links: usize, pairs: usize },
    /// The cocycle closes and nothing is left over — the form simply
    /// measured no trajectory to speak.
    NoTrajectory,
}

impl Withheld {
    /// Is this form PROVED still reachable by a change of perspective?
    /// The one bit a retention policy needs, and `Acyclic` is deliberately
    /// NOT it: on a tree the answer is unknown, not yes, and a policy that
    /// retains on an unasked question retains everything.
    pub fn is_mutable(&self) -> bool {
        matches!(self, Withheld::CoboundaryResidue { .. })
    }

    /// Was the coboundary question ASKABLE at all? `false` means the form
    /// carries no cycle and `δ¹` is silent — a reading that reports the
    /// arms without this cannot tell an empty arm from an unreachable one.
    pub fn is_askable(&self) -> bool {
        !matches!(self, Withheld::Acyclic { .. })
    }

    /// CV1 — THE KIND OF WHAT THE MINT WITHHELD (78.9): a class in `H¹` and
    /// a coboundary residue are cover-relative, C; an acyclic form and one
    /// with no trajectory asked no question, N.
    pub fn obstruction_kind(&self) -> crate::transducer::sentence::ObstructionKind {
        use crate::transducer::sentence::ObstructionKind;
        match self {
            Withheld::OrbitObstruction { .. } | Withheld::CoboundaryResidue { .. } => ObstructionKind::C,
            Withheld::Acyclic { .. } | Withheld::NoTrajectory => ObstructionKind::N,
        }
    }

    /// A stable token for the journal, so a reading counts arms rather
    /// than parsing prose.
    pub fn arm(&self) -> &'static str {
        match self {
            Withheld::OrbitObstruction { .. } => "H1-OBSTRUCTION",
            Withheld::CoboundaryResidue { .. } => "COBOUNDARY",
            Withheld::Acyclic { .. } => "ACYCLIC-UNASKABLE",
            Withheld::NoTrajectory => "NO-TRAJECTORY",
        }
    }
}

/// Classify a withheld form by its coboundary before its residue.
pub fn withheld_of(
    form: &Form,
    remainders: &[crate::transducer::germinate::ResidualGerm],
) -> Withheld {
    let cocycle = form.delta1();
    let held = cocycle
        .iter()
        .find(|(_, w)| !w.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()));
    if let Some((orbit_ix, w)) = held {
        let k = w.key();
        return Withheld::OrbitObstruction { orbit_ix: *orbit_ix, key: [k[0], k[1]] };
    }
    let links: usize = remainders.iter().map(|r| r.len()).sum::<usize>() + form.remainder.len();
    let pairs = remainders.iter().filter(|r| !r.is_empty()).count();
    if links == 0 {
        return Withheld::NoTrajectory;
    }
    // THE ASKABILITY GATE. `held` being `None` means "no orbit carries a
    // non-trivial holonomy" — which is a PROOF when there are orbits to
    // check and a VACUITY when there are none. A tree has `H¹ = 0` by
    // construction, so its cochain is a coboundary trivially and the
    // reading says nothing about whether a perspective exists.
    match cocycle.is_empty() {
        true => Withheld::Acyclic { links, pairs },
        false => Withheld::CoboundaryResidue { links, pairs },
    }
}

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
                #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a deflation is at most its pair's resident measure, and their sum cannot pass u64")]
                let next = p.saturating_add(*d);
                *p = next;
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
    // A form nothing in which acts is not uttered: the speaker's side of
    // `REFUSE_NOT_A_SENTENCE` (23.1 step 3, 50.6), reading the same
    // `Form::acts()` the court reads.
    if !rule.form.acts() {
        eprintln!("  mint refused: nothing in the form acts");
        return None;
    }
    if let Some(why) = crate::court::verdict::body::vertices_carry_frames(&rule.form, family) {
        eprintln!("  mint refused: a vertex does not carry its family ({why:?})");
        return None;
    }
    // 50.5 — what the form leaves unspoken on this family is its demand.
    // The remainders are measured once here and read by the measure below
    // (law 12): a demand vertex acts on nothing, so the declared form
    // speaks exactly what the undeclared one spoke and leaves the same
    // remainder, which is what `undeclared_remainder` reads at the court.
    let (pay, remainders) = form_deflation(&rule.form, family, crystal);
    let words = rule.form.vertices.iter().filter(|v| matches!(v, Vertex::Word { .. })).count();
    let demands = rule.form.demand_vertices().count();
    let unspoken: Vec<usize> = remainders.iter().map(|r| r.len()).collect();
    eprintln!(
        "  mint measured words {words} demands {demands} pairs {} pay {pay:?} remainders {unspoken:?}",
        family.len()
    );
    let declared = declare_unspoken(&rule.form, family, &remainders);
    let form = match declared.as_ref() {
        Some(f) => f,
        None => &rule.form,
    };
    let Some(mag) = sentence_measure_from(form, family, t_extent, pay, &remainders) else {
        eprintln!("  mint refused: the form speaks a lamina its family refuses");
        return None;
    };
    let out = crate::transducer::sentence::as_element_form(form, mag);
    if out.is_none() {
        eprintln!("  mint refused: the form has no tape at {mag:?}");
    }
    out
}

/// 50.5 — THE UNSPOKEN REMAINDER, DECLARED. A sentence is "the form saying
/// out loud what it could not close and asking the market to answer it".
/// `finish` says it for the pairs the budget never reached and the
/// multiplicities the correspondence could not resolve; this says it for
/// what the form's own words left unspoken on the family it was minted
/// over — the sites its vertices reach on no pair — which declared by
/// nothing would have `sentence_measure` refuse the form and the voice
/// withhold it.
///
/// A remainder link is a demand only where the form SPOKE NOTHING there
/// (`spoken == 0`, `residual_of`'s reading of absence): the court's own
/// `undeclared_remainder` admits exactly that link against a demand naming
/// its observed hue, and refuses a link the form spoke wrongly whatever a
/// demand says of it. So the declared links are the unspoken ones, each
/// with the hue the input held there (absence is hue 0 in this encoding)
/// and the hue the pair asks; a link the form spoke wrongly stays
/// undeclared and the form is refused by name.
///
/// Per pair, on the chart the unspoken links stand on, as `Source` with
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
    use crate::transducer::sentence::{pieces, Demand, Link};
    // THE COURT'S OWN READER SAYS WHAT IS UNDECLARED (law 12): a link a
    // demand the extraction already emitted names is declared, and naming
    // it again under `Source` would be one fact said twice. Sorted and
    // deduplicated by that reader, so membership is a search and not a
    // read of the whole form.
    let undeclared = crate::court::verdict::body::undeclared_remainder(remainders, form);
    if undeclared.is_empty() {
        return None;
    }
    let mut declared = form.clone();
    let mut any = false;
    for ((x, _), rem) in family.iter().zip(remainders) {
        let links: Vec<Link> = rem
            .iter()
            .filter(|&&(r, c, spoken, observed)| {
                spoken == 0 && undeclared.binary_search(&(r, c, spoken, observed)).is_ok()
            })
            .map(|&(r, c, _, observed)| {
                // The hue the input holds at the site. Outside the input
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
        if links.is_empty() {
            continue;
        }
        let chart = crate::transducer::stalk::invariant(&links);
        let kind = Demand::Source { multiplicity: pieces(&links).len() };
        if kind.typed().is_err() {
            continue;
        }
        declared.stand_demand(kind, &chart, links);
        any = true;
    }
    any.then_some(declared)
}

/// What a sentence measures over its bundle (50.5). A sentence with no
/// demand vertex measures its deflation and is spoken only when every pair
/// closes; a sentence with demand vertices is spoken whole: its closed
/// sub-complex measures the deflation its words performed on every pair
/// — before less after per lane, whether or not the pair closed — and its
/// demands measure the links they stand for, so a sentence that closes
/// nothing yet still carries what it asks for. `None` where the form leaves
/// a remainder no demand vertex declares: that is a withheld form, never a
/// word.
///
/// A demand declares links, not an exemption: whether the demands cover
/// what was left is the court's own reader, `undeclared_remainder` (law
/// 12, 23.1 step 3), and the court still re-derives it.
///
/// Lane 𝒯 is the span in receipts the bundle stood over — the quantity
/// `pivot_orbit` takes and `bundle_pressure` writes to `sum[2]` — and it is
/// measured here (law 5). A form minted in the stroke it was extracted in
/// stood over no receipts and measures zero, which is a reading.
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
        let left: usize = remainders.iter().map(|r| r.len()).sum();
        if left != 0 {
            eprintln!("  mint refused: {left} links remain and the form declares no demand");
            return None;
        }
        let mut mag = pay;
        mag[2] = t_extent;
        return Some(mag);
    }
    let undeclared = crate::court::verdict::body::undeclared_remainder(remainders, form);
    if !undeclared.is_empty() {
        eprintln!("  mint refused: {} links undeclared by the form's demands", undeclared.len());
        return None;
    }
    let mut mag = [0u64; 4];
    // The remainders are the ones `form_deflation` measured (law 12); the
    // pairs and the remainders are in the family's own order, so the zip is
    // the pairing. `before_chain` is a fact about the pair, not the form.
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
            #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a deflation is at most its pair's resident measure, and their sum cannot pass u64")]
            let next = m.saturating_add(*d);
            *m = next;
        }
    }
    let mut demanded: Vec<Link> = Vec::new();
    for (_, _, _, links) in form.demand_vertices() {
        demanded.extend(links.iter().copied().filter(|c| c.2 != c.3));
    }
    let asked = element::measure_lanes(&demanded);
    for (m, a) in mag.iter_mut().zip(&asked) {
        #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): resident deflations plus a resident demand's measure cannot pass u64")]
        let next = m.saturating_add(*a);
        *m = next;
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
            #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a sum of chain measures over resident chains cannot pass u64")]
            let next = sum[l].saturating_add(m[l]);
            sum[l] = next;
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
