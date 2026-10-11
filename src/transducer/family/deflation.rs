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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Withheld {
    /// A class in `H¹` of THIS cover. Not permanent (78.9, amending what
    /// this said): under 74 a class in `H¹` is cover-relative — C — and
    /// closes on another mind's cover; no generator of this dialect retires
    /// it, and another vantage may. Named by the orbit's chord, the overlap
    /// it closes on (AT1).
    OrbitObstruction { chord: coord::Overlap, key: [u8; 2] },
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
    if let Some((chord, w)) = held {
        let k = w.key();
        return Withheld::OrbitObstruction { chord: chord.clone(), key: [k[0], k[1]] };
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
    let read = form_deflation_read(form, family, crystal, placement);
    (read.pay, read.remainders)
}

/// WHAT ONE SPEECH OF A FORM OVER ITS FAMILY MEASURES: the pay, the
/// remainder per pair, and the cited forms the crystal did not hold. Where a
/// citation is unresolved the pair's remainder is not the form's silence —
/// the citation would have spoken there — and a reader that judges closure
/// reads `unresolved` first: the refusal it owes is the citation's (D, the
/// word this crystal lacks), never the remainder's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeflationRead {
    pub pay: [u64; 4],
    /// 78.162 — THE DISCHARGE AS MODES: what the closing form moved on the
    /// pairs it closed, mode by mode with exact amounts (S boosts, G hue
    /// modes, T null fluxes, H turns), each pair's at the mass it carried.
    /// Empty where nothing closed or nothing was carried.
    pub modes: crate::economy::modes::Modes,
    pub remainders: Vec<crate::transducer::germinate::ResidualGerm>,
    pub unresolved: Vec<crate::transducer::address::Address>,
    /// 78.153 — cited forms reached again while being spoken: a cycle of
    /// citations, which no speech completes (`Speech::cycled`).
    pub cycled: Vec<crate::transducer::address::Address>,
}

/// [`form_deflation_at`]'s one computation, whole.
pub fn form_deflation_read(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    placement: &crate::transducer::alphabet::Word,
) -> DeflationRead {
    let mut unresolved: Vec<crate::transducer::address::Address> = Vec::new();
    let mut cycled: Vec<crate::transducer::address::Address> = Vec::new();
    let mut modes = crate::economy::modes::Modes::new();
    if family.is_empty() {
        return DeflationRead { pay: [0; 4], modes, remainders: Vec::new(), unresolved, cycled };
    }
    // The form's own motions and hue maps, read once: what each closed pair
    // moves, at that pair's mass.
    let moved = form_motions(form);
    let mut pay = [0u64; 4];
    let mut closed_count = 0u64;
    // A mass past its carrier leaves 𝒯 unread, never saturated (rule 11).
    let mut carried_mass: Option<u64> = Some(0);
    let mut remainders = Vec::with_capacity(family.len());
    for (pi, (x, y)) in family.iter().enumerate() {
        let spoken = apply_form_on(form, x, placement, crystal, Some(pi));
        if let Some(s) = spoken.as_ref() {
            unresolved.extend(s.unresolved.iter().cloned());
            cycled.extend(s.cycled.iter().cloned());
        }
        let (closed, after_chain, gated) = pair_measure_read(form, x, y, spoken.as_ref());
        for (p, d) in pay.iter_mut().zip(&closed) {
            #[expect(clippy::disallowed_methods, reason = "rule 11 — MEASURE_BOUND (element::carrier): a deflation is at most its pair's resident measure, and their sum cannot pass u64")]
            let next = p.saturating_add(*d);
            *p = next;
        }
        if gated {
            // The pair's own work: the links its before side demanded moved
            // (𝒮), the mass the rule's motions carried on it.
            if let Some(m) = pair_mass(x, y) {
                carried_mass = carried_mass.and_then(|c| c.checked_add(m));
                let mass = num_rational::BigRational::from_integer(m.into());
                for (motion, gauge) in &moved {
                    let mut budget = crate::physics::Physics::default().stroke();
                    if let Some(ms) = crate::economy::modes::Modes::of_motion(motion, &mass, &mut budget) {
                        modes.add(&ms);
                    }
                    if let Some(gs) = crate::economy::modes::Modes::of_gauge(gauge, &mass) {
                        modes.add(&gs);
                    }
                }
            }
            closed_count += 1;
        }
        remainders.push(after_chain);
    }
    // 𝒯 — THE GROUND THE CLOSURE COVERED: a chain carries no clock, so 𝒯 is
    // read off the rule's own motions — every word vertex's whole motion at
    // each member, admitted to the tubes (78.71, `transport_order`), its null
    // flux in quanta — once for each pair the form closed. Measured by whoever
    // replays the form; never the span anything stood (78.149 (a)).
    // Weighed by the mass the closed pairs carried: a pair with nothing to
    // carry covers no ground, whatever the rule would move.
    if closed_count > 0 {
        if let Some(t) = carried_mass.and_then(|m| rule_flux(form, m)) {
            pay[crate::transducer::sentence::presentation::LANE_T] = t;
        }
    }
    unresolved.sort_unstable();
    unresolved.dedup();
    cycled.sort_unstable();
    cycled.dedup();
    DeflationRead { pay, modes, remainders, unresolved, cycled }
}

/// EVERY WORD VERTEX'S WHOLE MOTION AND HUE MAP, member by member: its
/// carriage composed with its rotor's base action, and the member's gauge.
fn form_motions(form: &Form) -> Vec<(crate::transducer::cga::Mv, Vec<(u8, u8)>)> {
    let mut out = Vec::new();
    for (_, v) in form.vertices.scoped() {
        let crate::transducer::sentence::Vertex::Word { derivation, acts, .. } = v else { continue };
        for (_, a) in acts.scoped() {
            let Some(motion) = element::conformal_of_rotor(&derivation.rotor(), &a.carriage) else { continue };
            out.push((motion, a.gauge.clone()));
        }
    }
    out
}

/// THE MASS A PAIR CARRIED: the 𝒮 measure of what its before side demanded
/// moved (cells created or erased). `None` where the pair demanded nothing.
fn pair_mass(x: &Grid, y: &Grid) -> Option<u64> {
    let before_chain = match crate::transducer::family::refine::refine_delta(x, y) {
        Some(chain) => chain,
        None => residual_of(Some(x), y),
    };
    let m = element::measure_lanes(&before_chain)[crate::transducer::sentence::presentation::LANE_S];
    (m > 0).then_some(m)
}

/// THE NULL FLUX A FORM'S MOTIONS CARRY, in quanta, at the mass `mass` its
/// closed pairs carried: each word vertex's whole motion at each member (its
/// carriage composed with its rotor's base action) and its hue map, one field
/// line each, admitted to fresh tubes; the transport they moved on 𝒯. `None`
/// where no motion reads; nothing where nothing was carried.
fn rule_flux(form: &Form, mass: u64) -> Option<u64> {
    use super::pairing::FieldLine;
    if mass == 0 {
        return Some(0);
    }
    let mut lines = Vec::new();
    for (_, v) in form.vertices.scoped() {
        let crate::transducer::sentence::Vertex::Word { derivation, acts, .. } = v else { continue };
        for (_, a) in acts.scoped() {
            let Some(carriage) = element::conformal_of_rotor(&derivation.rotor(), &a.carriage) else { continue };
            lines.push(FieldLine { carriage, gauge: a.gauge.clone(), mass, free: false });
        }
    }
    if lines.is_empty() {
        return None;
    }
    let mut field = crate::transducer::cech::orbit::Field::new();
    let mut budget = crate::physics::Physics::default().stroke();
    let r = super::pairing::transport_order(&lines, &[], &mut field, &mut budget);
    Some(r.transport[crate::transducer::sentence::presentation::LANE_T])
}

/// ONE PAIR OF THE DEFLATION, from the speech already taken on it: what the
/// pair pays (zero where the gate below withholds it), its remainder, and
/// whether the closure gate passed it.
fn pair_measure_read(
    form: &Form,
    x: &Grid,
    y: &Grid,
    spoken: Option<&Speech>,
) -> ([u64; 4], crate::transducer::germinate::ResidualGerm, bool) {
    // The exact cochain where the pair refines on K_lcm; the raw residual
    // of the two grids where it does not. Both are measured on the pair.
    let before_chain = match crate::transducer::family::refine::refine_delta(x, y) {
        Some(chain) => chain,
        None => residual_of(Some(x), y),
    };
    // A speech that does not fit its own frame lays down no grid, so the
    // residual is measured against what the form actually said: nothing.
    let grid = spoken.and_then(|s| s.grid());
    let after_chain = residual_of(grid.as_ref(), y);
    let before_mu = element::measure_lanes(&before_chain);
    let after_mu = element::measure_lanes(&after_chain);
    // S9.5 — THE PAY GATE IS CLOSURE. A pair pays when the form CLOSES
    // it: the remainder is empty and what the pair demanded is what the
    // form spoke. Under 50.5 (Y3), a sentence with demand vertices is
    // valid, never closed: its closed sub-complex pays for the deflation
    // its closed words achieved: `deflation_of(lane_moves(after, before))`.
    let closed = crate::economy::deflation_of(&crate::economy::lane_moves(&after_mu, &before_mu));
    match after_chain.is_empty() || form.demand_vertices().next().is_some() {
        true => (closed, after_chain, true),
        false => ([0; 4], after_chain, false),
    }
}

/// ONE PAIR OF THE DEFLATION, its pay and remainder alone.
fn pair_measure(
    form: &Form,
    x: &Grid,
    y: &Grid,
    spoken: Option<&Speech>,
) -> ([u64; 4], crate::transducer::germinate::ResidualGerm) {
    let (closed, after_chain, _) = pair_measure_read(form, x, y, spoken);
    (closed, after_chain)
}

/// 78.153 — ONE PAIR OF THE DEFLATION ON A STROKE: what the court's replay
/// reads of pair `pi` (its remainder, and the citations unresolved or
/// cycled there), or `Spent` where the stroke ran out inside the speech.
/// One read for the pair, and one per cited form its speech speaks afresh
/// (`apply_form_metered`).
pub(crate) enum PairRead {
    Read {
        remainder: crate::transducer::germinate::ResidualGerm,
        unresolved: Vec<crate::transducer::address::Address>,
        cycled: Vec<crate::transducer::address::Address>,
    },
    Spent,
}

pub(crate) fn deflation_pair_metered(
    form: &Form,
    family: &[(Grid, Grid)],
    pi: usize,
    crystal: &dyn Crystal,
    placement: &crate::transducer::alphabet::Word,
    budget: &mut crate::physics::Budget,
    bodies: Option<Vec<Body>>,
) -> PairRead {
    let Some((x, y)) = family.get(pi) else {
        return PairRead::Read { remainder: Vec::new(), unresolved: Vec::new(), cycled: Vec::new() };
    };
    if !budget.spend() {
        return PairRead::Spent;
    }
    let spoken = match apply_form_metered(form, x, placement, crystal, Some(pi), budget, bodies) {
        Spoken::Spent => return PairRead::Spent,
        Spoken::Unfit => None,
        Spoken::Said(s) => Some(s),
    };
    let (unresolved, cycled) = match spoken.as_ref() {
        Some(s) => (s.unresolved.clone(), s.cycled.clone()),
        None => (Vec::new(), Vec::new()),
    };
    let (_, remainder) = pair_measure(form, x, y, spoken.as_ref());
    PairRead::Read { remainder, unresolved, cycled }
}

/// Mint the family's sentence: μ⃗ = [S, G, 0, H] of the closed cochain.
/// The rule as the word a voice speaks and submits: the form's v3 tape
/// carrying the deflation it measures over the bundle through the crystal
/// that resolves its citations. A form with no measured deflation is not a
/// word.
///
/// LANE 𝒯 CARRIES NO SPAN (78.149 (a), the operator's ruling of
/// 2026-10-08): the span the bundle stood over is not μ — waiting is not
/// work — and no minted magnitude carries it. `_stood` is the caller's
/// reading of that span; the mint takes it and writes it nowhere. It stays
/// on the signature so the callers that withhold on an unmeasurable span
/// are not rewritten here (named in 78.149 as owed).
///
/// Under 50.5 (Y3): A sentence with word vertices and demand vertices mints
/// on its closed sub-complex: the amount is the deflation its closed words
/// achieved and nothing else. A sentence of zero word vertices mints nothing.
pub fn mint_sentence(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    _stood: u64,
) -> Option<element::Element> {
    mint_or_refuse(rule, family, crystal).map_err(crate::transducer::sentence::Refused::noted).ok()
}

/// [`mint_sentence`] with its refusal named (A.17) and counted where it
/// forms, by the one refusal reader (78.9). `_stood` is not minted (78.149
/// (a)).
pub fn mint_sentence_why(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    _stood: u64,
) -> Result<element::Element, crate::transducer::sentence::Refused> {
    mint_or_refuse(rule, family, crystal).map_err(crate::transducer::sentence::Refused::noted)
}

/// THE MINT'S REFUSAL BY ARM (78.67): a vertex that does not carry its
/// family is `carry`; links the form's demands do not account for are
/// `demands`; a citation this mind does not hold is `citation`; a form
/// with nothing acting, no tape or an address that does not decode is
/// `integrity`. Exhaustive over the refusals the mint returns; any other is
/// named `other` rather than folded into one of these.
pub fn mint_arm(why: crate::transducer::sentence::Refused) -> &'static str {
    use crate::transducer::sentence::Refused;
    match why {
        Refused::MintVertexNotCarried(..) => "carry",
        Refused::MintLinksWithoutDemand | Refused::MintLinksUndeclared => "demands",
        Refused::CitedNotHeld | Refused::CitedSealed | Refused::OwedToEveryCourt => "citation",
        Refused::MintNothingActs | Refused::MintNoTape | Refused::MintAddressUndecoded => "integrity",
        _ => "other",
    }
}

/// 78.158a U1 — THE MINT'S REFUSAL NAMED WHOLE, on every path that mints:
/// its arm, its proof, its kind (78.9), and the KINDS OF THE DEMANDS the
/// refused form carries, in the form's own vertex order (`demand_vertices`,
/// each by `Demand::label`) — `[]` where it carries none. The solve paths
/// read "unmeasured span, or the mint refused" and the speech path named
/// the arm alone, so 456 refusals of "nothing in the form acts" could not
/// say that every vertex was a demand, nor which.
pub fn mint_refusal_line(why: crate::transducer::sentence::Refused, form: &crate::transducer::sentence::Form) -> String {
    let kinds: Vec<&'static str> = form.demand_vertices().map(|(_, kind, _, _)| kind.label()).collect();
    format!("mint refused ({}): {} [{:?}] demands [{}]", mint_arm(why), why.proof(), why.kind(), kinds.join(", "))
}

fn mint_or_refuse(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> Result<element::Element, crate::transducer::sentence::Refused> {
    use crate::transducer::sentence::Refused;
    // A form nothing in which acts is not uttered: the speaker's side of
    // `REFUSE_NOT_A_SENTENCE` (23.1 step 3, 50.6), reading the same
    // `Form::acts()` the court reads.
    if !rule.form.acts() {
        return Err(Refused::MintNothingActs);
    }
    if let Some((r, why)) = crate::court::verdict::body::vertices_carry_frames(&rule.form, family) {
        return Err(Refused::MintVertexNotCarried(r.obstruction.kind(), why));
    }
    // 50.5 — what the form leaves unspoken on this family is its demand. A
    // citation this mind's crystal does not hold leaves nothing to declare:
    // the sites it would speak are not the form's silence.
    let here = crate::transducer::alphabet::Word::identity();
    let spoken = form_deflation_read(&rule.form, family, crystal, &here);
    // 78.153 — a cycle of citations is no composition: no speech of it ends.
    if !spoken.cycled.is_empty() {
        return Err(Refused::CitationsDoNotCompose);
    }
    if !spoken.unresolved.is_empty() {
        return Err(Refused::CitedNotHeld);
    }
    let declared = declare_unspoken(&rule.form, family, &spoken.remainders);
    let form = match declared.as_ref() {
        Some(f) => f,
        None => &rule.form,
    };
    // THE FORM THE COURT WILL HOLD, measured — never the form in hand (34.2,
    // 54.3). The word travels as its address and its receipts envelope, and
    // the court reads its form from exactly those by the one decoder; the
    // mint reads the same two before it mints, so the remainder the mint
    // admits and the remainder the court re-derives are one reading of one
    // object. It measured the undeclared form and shipped the declared one,
    // on the unchecked word that a demand vertex acts on nothing.
    let held = crate::transducer::sentence::held_form(
        &crate::transducer::sentence::form_vertex_key(form),
        &crate::transducer::element::ReceiptsEnvelope::of_form(form),
    )
    .ok_or(Refused::MintAddressUndecoded)?;
    let read = form_deflation_read(&held, family, crystal, &here);
    if !read.cycled.is_empty() {
        return Err(Refused::CitationsDoNotCompose);
    }
    if !read.unresolved.is_empty() {
        return Err(Refused::CitedNotHeld);
    }
    let mag = sentence_measure_why(&held, family, read.pay, &read.remainders)?;
    crate::transducer::sentence::as_element_form(form, mag).ok_or(Refused::MintNoTape)
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
/// Lane 𝒯 carries no span (78.149 (a)): the span in receipts the bundle
/// stood over is the caller's reading (`_stood`), and it is not μ and not
/// minted. The temporal lane of a measure is what its chains measure there.
pub fn sentence_measure(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    _stood: u64,
) -> Option<[u64; 4]> {
    let (pay, remainders) = form_deflation(form, family, crystal);
    sentence_measure_why(form, family, pay, &remainders).ok()
}

/// The measure, from a deflation already taken: the mint measures the
/// remainders once, declares what was unspoken, and measures the declared
/// form against the same remainders (law 12) — a demand vertex acts on
/// nothing, so the deflation is the same form's. `_stood` is not minted
/// (78.149 (a)).
pub fn sentence_measure_from(
    form: &Form,
    family: &[(Grid, Grid)],
    _stood: u64,
    pay: [u64; 4],
    remainders: &[crate::transducer::germinate::ResidualGerm],
) -> Option<[u64; 4]> {
    sentence_measure_why(form, family, pay, remainders).ok()
}

/// [`sentence_measure_from`] with its refusal named (A.17) and not counted:
/// a reader that only measures does not refuse, and the mint that does
/// counts it where it forms.
///
/// 78.149 — WHAT THIS MEASURES IS THE WORD'S MAGNITUDE, ITS MEASURE AND
/// IDENTITY, AND ITS μ IS THE DISCHARGE ALONE. With no demand vertex the two
/// are one vector: the magnitude IS the deflation the form closed, and the
/// span it stood is never written into it (a). With demand vertices the
/// magnitude is the closed sub-complex's deflation AND the measure of the
/// links its demands ask for: that sum is the listing's ASK, its tension on
/// the book, and the court never prices it — every escrow and release reads
/// the discharge (`Kernel::discharge_of`), the ask only capping it (b).
pub fn sentence_measure_why(
    form: &Form,
    family: &[(Grid, Grid)],
    pay: [u64; 4],
    remainders: &[crate::transducer::germinate::ResidualGerm],
) -> Result<[u64; 4], crate::transducer::sentence::Refused> {
    use crate::transducer::sentence::Refused;
    if form.demand_vertices().next().is_none() {
        let left: usize = remainders.iter().map(|r| r.len()).sum();
        if left != 0 {
            return Err(Refused::MintLinksWithoutDemand);
        }
        // NO DISCHARGE, NO WORD (the operator, 2026-10-08: never give μ away
        // for the possibility of falsification). A span written into T made
        // a form that discharged nothing a non-zero magnitude, escrowed at
        // its price; no span is written now (78.149 (a)), and the work is
        // still read by name.
        if pay.iter().all(|&x| x == 0) {
            return Err(Refused::MintNoTape);
        }
        return Ok(pay);
    }
    let undeclared = crate::court::verdict::body::undeclared_remainder(remainders, form);
    if !undeclared.is_empty() {
        return Err(Refused::MintLinksUndeclared);
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
    // THE ASK (78.149 (b)): what the demands stand for, added to the
    // measure so a sentence that closes nothing still exists as the listing
    // it is. It is the word's measure and the book's tension, a CAP on what
    // a closer is paid; it is never escrowed and never minted.
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
    // No span is written over 𝒯 (78.149 (a)): the lane holds what the chains
    // measure there, the span the bundle stood is not μ.
    Ok(mag)
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
