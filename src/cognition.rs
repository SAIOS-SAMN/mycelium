// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COGNITION — the kernel loop. A demand (scene A → scene B) enters as raw
//! cells; the transducer lifts blades, the unified solve extracts one conformal
//! motor, conception classifies it against the installed basis, and a novel
//! germ installs ONLY on two-voice consilience (`consensus`). State is
//! Vec-of-relations throughout — the basis is a Vec of motors, admission and the
//! nerve are Vec-backed. No digest-keyed cognition (Anti-Flattening Law); the
//! motor digest below is a WIRE identity for matching receipts, never reasoned on.

use crate::network::consensus::{Admission, Receipt};
use crate::network::nerve::Nerve;
use crate::transducer::blade::{components, lift};
use crate::transducer::element::{self, Element};
use crate::transducer::germinate::germinate;

/// Mycelium's heritage floor: demands larger than this fragment (the 48% score
/// depended on decomposing large corpus). Here the topological trigger is
/// "no single motor closes the scene AND ≥2 bodies" — size-independent — with
/// the floor retained as the guard for whole-scene attempts.
pub const FRAGMENT_FLOOR: usize = 9;

/// The verdict of ingesting one demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The demand's substance DECOMPOSED into a canonical sentence of installed
    /// generators — the confluent normal form, reached by peeling under ≺_DM (no
    /// lookup, no search). The algebra spans it, nothing minted. An empty word is a
    /// pure transposition (no substance). This one outcome subsumes the old recall
    /// (length 1) and composition (length k): one reduction, not two search kinds.
    Decomposed(Vec<usize>),
    /// A novel generator awaiting a second kernel's voice (consilience pending).
    GermPending,
    /// An irreducible residual that reached quorum and adjoined as a new generator
    /// (Knuth–Bendix completion — the alphabet grows).
    Installed(usize),
    /// The scene had no single motor; it fragmented into per-body sub-demands,
    /// each reduced independently and recomposed (the large-corpus path).
    Fragmented(Vec<Outcome>),
    /// A degenerate demand (empty, or mismatched body counts) — nothing to reduce.
    Pivot,
}

/// The result of reducing a demand's substance against the generator basis: either
/// a canonical sentence (fully decomposed) or an irreducible residual (a critical
/// pair to complete by germination).
enum NormalForm {
    Decomposed(Vec<usize>),
    Irreducible(Element),
}

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    /// The germinated basis: anonymous ANASTOME elements — one uniform species
    /// (block-diagonal descriptions with a coordinate-exact word), whether a solved
    /// motion∘recolour or a residual coboundary. The kernel never branches on their
    /// "kind"; they are opaque, addressed by canonical form. English lives only in
    /// the `ccl` seam.
    basis: Vec<Element>,
    /// The discoverer (installing voice) of each basis germ, parallel to `basis`
    /// — so novelty yield flows to who germinated it, on real use.
    discoverers: Vec<[u8; 32]>,
    admission: Admission,
    /// THE OPEN-DEMAND MARKET: irreducible residuals still awaiting consilience —
    /// the un-factored chains the mesh QUOTES as work (Vec-of-relations, capped;
    /// ephemeral — the market is runtime tension, not persisted language).
    pending: Vec<Element>,
    /// THE CHRONOMETRIC RECEIPT CHAIN — the K-step index is the mesh's temporal
    /// authority (algebraic time, no wall clocks); every install seals a
    /// parent-linked receipt (LAW-5 continuity). The Step-4 ledger's spine.
    chain: crate::chronometry::ReceiptChain,
    pub nerve: Nerve,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    optimize_cursor: usize,
    /// Polytonic kinematic pressure + torsion (per lane).
    physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    treasury: crate::economy::Treasury,
    /// Words installed on two-voice consilience but NOT yet court-final
    /// (Step 5): local dialect awaiting the admission docket — (address, the
    /// element as spoken at install). Vec-of-relations.
    provisional: Vec<([u8; 32], Element)>,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel::default()
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    /// THE DOCKET (Step 5): every provisional word, exported for the court's
    /// blind replay + confluence audit — (element, attesting voices, escrowed μ⃗).
    pub fn dockets(&self) -> Vec<([u8; 32], Element, Vec<[u8; 32]>, [u64; 4])> {
        self.provisional
            .iter()
            .filter_map(|(addr, spoken)| {
                let (_, voices, mu, _) = self
                    .treasury
                    .escrowed_awards()
                    .iter()
                    .find(|(a, _, _, _)| a == addr)?;
                Some((*addr, spoken.clone(), voices.clone(), *mu))
            })
            .collect()
    }

    /// APPLY the court's verdict on a provisional word.
    /// upheld → release the escrowed award (finality: full settlement books).
    /// rejected, slash=false → demote: stays local dialect, award evaporates.
    /// rejected, slash=true → false speech: word excised from the basis and
    /// each voice's would-be share debited back into the pool.
    pub fn apply_verdict(&mut self, addr: &[u8; 32], upheld: bool, slash: bool) -> bool {
        let Some(i) = self.provisional.iter().position(|(a, _)| a == addr) else {
            return false;
        };
        self.provisional.remove(i);
        if upheld {
            self.treasury.release_award(addr)
        } else {
            let ok = self.treasury.revoke_award(addr, slash);
            if slash {
                // Excise a slashed BASE word. A slashed section puncture has no
                // separate card to excise (growth was +axis) — the economic
                // slash applies; the fiber coordinate stays local dialect.
                if let Some(w) = self.basis.iter().position(|t| t.address() == *addr) {
                    self.basis.remove(w);
                    self.discoverers.remove(w);
                }
            }
            ok
        }
    }

    /// ENROLL an identity at a tier: its stake capitalizes the escrow pool —
    /// the one genesis issuance per identity (idempotent, C9: one stake admits).
    /// The hypha enrolls ITSELF at boot with its own tier; arriving voices are
    /// enrolled as participants on first speech. Returns the staked μ⃗.
    pub fn enroll(&mut self, who: [u8; 32], tier: crate::economy::Tier) -> [u64; 4] {
        let anchor = self.anchor();
        self.treasury.enroll(who, tier, anchor)
    }

    /// Ingest a demand from `voice`: transduce → solve → conceive → admit.
    /// A whole-scene coherent motor settles directly; otherwise the demand
    /// FRAGMENTS into per-body sub-demands (bodies corresponded by invariant),
    /// each solved and admitted independently, then recomposed — the large-corpus
    /// path. An irreducible body makes the whole a pivot.
    /// Convenience: PERCEIVE (segment) then ADJUDICATE. In the mesh, perception
    /// belongs to the client (`client::perceive`) — this path is for local use
    /// and tests. The server calls `adjudicate` on the client's reported percept.
    pub fn ingest(&mut self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
        self.adjudicate(&components(a), &components(b), voice)
    }

    /// THE JUDGE — mathematics + ANASTOME only. Given a client's PERCEPT (already
    /// segmented bodies of A and B), correspond by invariant, derive the exact
    /// element per body, and answer valid (Recognised/Installed/Fragmented) or
    /// invalid (Pivot). No sensing here — the client perceived; the judge reasons.
    pub fn adjudicate(
        &mut self,
        ca: &[Vec<(i64, i64, u8)>],
        cb: &[Vec<(i64, i64, u8)>],
        voice: [u8; 32],
    ) -> Outcome {
        self.enroll(voice, crate::economy::Tier::Participant); // first speech enrolls
        self.physics.relax(); // the metabolism dissipates its backlog as it works
        // THE SYNTROPIC STROKE fires in BOTH resting states of the two-stroke:
        // DEPLETED (expansion spent its fuel — optimize to regenerate) and IDLE
        // (no backlog — free energy condenses the basis). A mesh that is neither
        // expanding nor optimizing is wasting its metabolism.
        let idle = self.physics.metabolism_rate().iter().all(|&p| p <= 1);
        if self.physics.depleted() || idle {
            self.optimize();
        }
        if ca.is_empty() || ca.len() != cb.len() {
            self.physics.pivot([false; 4]);
            return Outcome::Pivot;
        }
        // Correspond bodies by rigid-motion invariant (zero search) — the pairing,
        // keeping the raw cells so the full description can be read per body.
        let mut ai: Vec<usize> = (0..ca.len()).collect();
        let mut bi: Vec<usize> = (0..cb.len()).collect();
        ai.sort_by(|&x, &y| lift(&ca[x], 0).invariant().cmp(&lift(&ca[y], 0).invariant()));
        bi.sort_by(|&x, &y| lift(&cb[x], 0).invariant().cmp(&lift(&cb[y], 0).invariant()));
        // POST-EXCISION DERIVATION — perception measures, never names. Each paired
        // body is framed to its canonical origin by the SUBSTRATE (the hypha's
        // transposition projection, per the ruling); the demand's substance is the
        // coboundary between the framed scenes. Identical framed scenes = a pure
        // transposition (no substance). Everything else GERMINATES as a centered
        // coboundary germ — "rotation", "recolour", "deformation" are patterns the
        // mesh must learn and consolidate, never solver verdicts.
        let mut germs: Vec<Element> = Vec::with_capacity(ca.len());
        for k in 0..ca.len() {
            let fa = frame_to_origin(&ca[ai[k]]);
            let fb = frame_to_origin(&cb[bi[k]]);
            let e = match germinate(&fa, &fb) {
                Some(rg) => element::from_residual(&rg),
                None => Element::identity(element::GERM_DIM), // pure transposition
            };
            germs.push(e);
        }
        // One element closing every body is a single germ; otherwise the demand
        // fragmented into independent per-body germs.
        if germs.iter().all(|g| *g == germs[0]) {
            self.conceive_germ(germs.swap_remove(0), voice)
        } else {
            Outcome::Fragmented(germs.into_iter().map(|g| self.conceive_germ(g, voice)).collect())
        }
    }

    /// Classify one anonymous germ against the basis and run the two-voice gate.
    /// Opaque: matched by canonical address, never by "kind".
    fn conceive_germ(&mut self, g: Element, voice: [u8; 32]) -> Outcome {
        let lanes = g.lanes();
        // CONFLUENT DECOMPOSITION — reduce the demand's substance to normal form by
        // peeling installed generators under ≺_DM. No lookup, no pair-search: a
        // demand either decomposes into a canonical sentence, or leaves an
        // irreducible residual (a Knuth–Bendix critical pair) to complete.
        match self.reduce(&g) {
            NormalForm::Decomposed(word) => {
                let anchor = self.anchor();
                self.treasury.settle_recall(voice, lanes, anchor); // reader's fee → the pool recharges
                for &i in &word {
                    // PoUW circulation: the demander pays each cited word's
                    // discoverer the structure exercised, from earned balance.
                    let mag = self.basis[i].magnitude();
                    self.treasury.settle_use(voice, self.discoverers[i], mag, anchor);
                }
                self.physics.syntropy(lanes); // a spanned sentence builds order
                Outcome::Decomposed(word)
            }
            NormalForm::Irreducible(residual) => {
                // PUNCTURE-BEFORE-MINT: if this residual sits on an EXISTING
                // thought's base positions with a new transition assignment, it is
                // a SECTION prospect — the germinated axis extends (quorum-gated),
                // and no sibling card is minted. Growth as +axis, not +card.
                // (A residual matching a held section never reaches here — the
                // reduction's direct terminal speaks the family.)
                let centered = residual.sub().to_vec();
                if let Some(ti) = self.basis.iter().position(|t| {
                    !t.sub().is_empty() && element::positions(t.sub()) == element::positions(&centered)
                }) {
                    let asg = element::assignment(&centered);
                    let saddr = section_address(&self.basis[ti].address(), &asg);
                    let _ = self.admission.record(Receipt { voice, germ: saddr });
                    let quorum = self.admission.voices(&saddr) >= crate::network::consensus::QUORUM;
                    if quorum && self.physics.can_expand(lanes) {
                        self.basis[ti].puncture(asg);
                        let mu = crate::economy::price(
                            &residual.magnitude(),
                            &crate::economy::yield_surface(&self.physics),
                        );
                        let anchor = self.anchor();
                        // PROVISIONAL (Step 5): the award escrows until the
                        // court's finality verdict — unfinalized speech earns
                        // nothing; the puncture itself is local dialect.
                        self.treasury.escrow_award(saddr, self.admission.voices_of(&saddr), mu, anchor);
                        self.provisional.push((saddr, residual.clone()));
                        self.physics.relieve(lanes);
                        self.physics.syntropy(lanes); // volume built through the axis
                        self.physics.spend(lanes); // still an expansion — fuel is spent
                        self.pending.retain(|p| p.address() != residual.address());
                        self.chain.append(saddr, voice); // the puncture is a K-step too
                        return Outcome::Installed(ti); // installed AS A SECTION of thought ti
                    }
                    self.physics.bump(lanes);
                    return Outcome::GermPending;
                }
                // KB COMPLETION — a genuinely new base thought adjoins on
                // two-kernel consilience. Confluence is the gate: two kernels
                // reducing the same demand reach the SAME residual (same address),
                // so agreement is a theorem about the algebra, not a matched card.
                let addr = residual.address();
                let _ = self.admission.record(Receipt { voice, germ: addr });
                // ENTROPIC EXPANSION needs BOTH consilience (quorum of distinct
                // kernels) AND metabolic fuel (reserve above the floor on the germ's
                // lanes). A germ at quorum with a depleted lane DEFERS — it stays
                // pending while the mesh optimizes/regenerates, and installs on a
                // later voice's submission once the reserve recovers (the recheck
                // below, not record()'s one-shot crossing, so deferral never
                // strands a germ).
                let quorum = self.admission.voices(&addr) >= crate::network::consensus::QUORUM;
                if quorum && self.physics.can_expand(lanes) {
                    let _ = self.treasury.settle_install(&residual, voice); // POW++ gate only
                    let mu = crate::economy::price(
                        &residual.magnitude(),
                        &crate::economy::yield_surface(&self.physics),
                    );
                    let anchor = self.anchor();
                    // PROVISIONAL (Step 5): consilience escrows pending the
                    // court's admission docket; the word joins the basis as
                    // local dialect, final settlement waits for the verdict.
                    self.treasury.escrow_award(addr, self.admission.voices_of(&addr), mu, anchor);
                    self.provisional.push((addr, residual.clone()));
                    self.physics.relieve(lanes);
                    self.physics.syntropy(lanes); // a completed rule is order built
                    self.physics.spend(lanes); // the expansion SHRINKS the reserve
                    self.pending.retain(|p| p.address() != addr); // the market CLEARS
                    // Seal the K-step receipt: the install IS a tick of algebraic
                    // time, parent-linked to the mesh's whole history.
                    self.chain.append(addr, voice);
                    self.basis.push(residual);
                    self.discoverers.push(voice);
                    Outcome::Installed(self.basis.len() - 1)
                } else {
                    // THE GEOMETRIC TORSION SOURCE (Einstein-Cartan import): the open
                    // demand's pressure is COMPUTED from its δ-field's torsion
                    // residue — kinematic energy over the relaxation rate — not a
                    // flat event count. Bounded per demand; per-lane as ever.
                    let residue = crate::torsion::extract_torsion_residue(
                        &element::germ_delta(&residual.sub().to_vec()),
                        8,
                        8,
                        residual.torsion().first().map(|(_, l)| *l),
                    );
                    let k = crate::torsion::required_k_steps(&residue).min(8);
                    for _ in 0..k.max(1) {
                        self.physics.bump(lanes);
                    }
                    // LIST the un-factored residual on the open-demand market so the
                    // mesh can QUOTE it as work (bounded; oldest listing lapses).
                    if !self.pending.iter().any(|p| p.address() == addr) {
                        const PENDING_CAP: usize = 512;
                        if self.pending.len() >= PENDING_CAP {
                            self.pending.remove(0);
                        }
                        self.pending.push(residual);
                    }
                    Outcome::GermPending
                }
            }
        }
    }

    /// REDUCE a demand to its confluent normal form. (1) TRANSPOSE to substance
    /// (quotient the ℤ² position). (2) PEEL installed generators while the reduction
    /// magnitude strictly descends under ≺_DM — a terminating rewrite (the order is
    /// well-founded). A demand whose substance directly IS a generator terminates at
    /// length 1 (the old "recall"); a compound one peels to a longer sentence (the
    /// old "composition"); one no generator lowers is IRREDUCIBLE — a critical pair
    /// to germinate.
    fn reduce(&self, g: &Element) -> NormalForm {
        self.reduce_excluding(g, None)
    }

    /// The reduction with one generator EXCLUDED from the alphabet — the syntropic
    /// stroke's redundancy probe: "does the REST of the basis already derive this
    /// generator?" (`exclude = None` is the ordinary full reduction.)
    fn reduce_excluding(&self, g: &Element, exclude: Option<usize>) -> NormalForm {
        // Post-excision, a perceived demand is a CENTERED COBOUNDARY CHAIN — the
        // chain reduction decomposes it over the germinated patterns. The matrix
        // path below remains for pure-algebra (group) elements.
        let sub = g.substance();
        if !sub.sub().is_empty() {
            return self.reduce_chain(sub.sub().to_vec(), exclude);
        }
        let mut s = sub;
        let mut word = Vec::new();
        let budget = self.peel_budget(); // the syntropic/entropic metabolism bounds compute
        // ≺_DM is well-founded so this terminates; the 64 is only a safety net.
        for step in 0..64 {
            if s.is_identity() {
                return NormalForm::Decomposed(word); // fully reduced (empty = pure transposition)
            }
            // Terminal: the substance IS an installed generator (length-1 sentence).
            // (A cheap scan — no matrix inverse; always affordable.)
            if let Some(i) = self
                .basis
                .iter()
                .enumerate()
                .position(|(j, p)| Some(j) != exclude && p.substance().same_action(&s))
            {
                word.push(i);
                return NormalForm::Decomposed(word);
            }
            // METABOLIC THROTTLE: peeling is the expensive arm (a matrix inverse per
            // generator). Under backlog the budget is spent, so we stop and germinate
            // the compound rather than brute-force — the mesh metabolizes first.
            if step >= budget {
                return NormalForm::Irreducible(s);
            }
            match self.best_peel(&s, exclude) {
                Some((i, next)) => {
                    word.push(i);
                    s = next;
                }
                None => return NormalForm::Irreducible(s),
            }
        }
        NormalForm::Irreducible(s)
    }

    /// The metabolic PEEL BUDGET — how many expensive peels the confluent reduction
    /// may spend, governed by the current per-lane PRESSURE (the entropic backlog),
    /// never a monotone scalar (#34). Polytonic: the most-pressured lane governs
    /// (Liebig). Calm → full reduction; loaded → shallow; saturated → direct-match
    /// only, then germinate the compound (refined later, when the mesh is calm).
    fn peel_budget(&self) -> usize {
        match self.physics.metabolism_rate().iter().copied().max().unwrap_or(0) {
            0..=7 => 64,   // calm — reduce to the full normal form
            8..=31 => 8,   // loaded — a few peels, then defer
            _ => 0,        // saturated — recognise cheaply or germinate; do not brute-force
        }
    }

    /// Peel the MAXIMAL ≺_DM-descending generator from substance `s`: try each
    /// invertible (group) generator γ, form `γ⁻¹ ∘ s`, and keep the one whose
    /// reduction magnitude descends most (tie-broken by basis order — deterministic,
    /// so the reduction is confluent). `None` when no generator lowers `s`.
    fn best_peel(&self, s: &Element, exclude: Option<usize>) -> Option<(usize, Element)> {
        let before = s.reduction_magnitude();
        let mut best: Option<(usize, Element, [u64; 4])> = None;
        for (i, gen) in self.basis.iter().enumerate() {
            if Some(i) == exclude {
                continue;
            }
            let gs = gen.substance();
            let Some(inv) = gs.solution() else { continue }; // only group generators peel
            let cand = inv.compose(s);
            let m = cand.reduction_magnitude();
            if crate::economy::descends(&m, &before) {
                let take = match &best {
                    Some((_, _, bm)) => crate::economy::descends(&m, bm),
                    None => true,
                };
                if take {
                    best = Some((i, cand, m));
                }
            }
        }
        best.map(|(i, cand, _)| (i, cand))
    }

    /// THE SPECULATIVE PROJECTION BUDGET — the metabolism prices IMAGINATION: how
    /// wide a superposition (uncommitted branches held in tension) and how deep a
    /// virtual rollout the reduction may hold before collapse. Calm → wide and
    /// deep; loaded → narrow; saturated → near-reactive (terminals only).
    fn projection_budget(&self) -> (usize, usize) {
        match self.physics.metabolism_rate().iter().copied().max().unwrap_or(0) {
            0..=7 => (4, 16),
            8..=31 => (2, 8),
            _ => (1, 2),
        }
    }

    /// The kernel's chain reduction — delegates to the shared projective
    /// decomposition ([`decompose_over`]) at the metabolic projection budget.
    fn reduce_chain(&self, first: Vec<(i64, i64, u8, u8)>, exclude: Option<usize>) -> NormalForm {
        let (beam_width, depth) = self.projection_budget();
        match decompose_over(&self.basis, &first, exclude, beam_width, depth) {
            Ok(word) => NormalForm::Decomposed(word),
            Err(void) => NormalForm::Irreducible(element::from_residual(&void)),
        }
    }

    /// THE SYNTROPIC STROKE — one bounded consolidation step, run when a lane's
    /// reserve is depleted. Probe the cursor generator for REDUNDANCY: if the rest
    /// of the basis already derives it (its substance decomposes with it excluded),
    /// it is a closed critical pair wearing a generator's name — RETIRE it, and the
    /// basis compresses toward a minimal confluent generating set. Either way the
    /// reserve REGENERATES: optimization is the recovery stroke, so entropic
    /// expansion resumes only at the pace the syntropic counterpart earns.
    fn optimize(&mut self) {
        if !self.basis.is_empty() {
            let i = self.optimize_cursor % self.basis.len();
            self.optimize_cursor = self.optimize_cursor.wrapping_add(1);
            // THE CONDENSE MOVE: absorb position-identical siblings into the
            // cursor thought as SECTIONS along its punctured axis — the family
            // collapses into one bundle. The basis CONDENSES: cardinality falls,
            // dimensional volume rises. Bounded per stroke; the absorbed word's
            // discoverer still earns (its language lives on as a section).
            let pos_i = element::positions(self.basis[i].sub());
            if !pos_i.is_empty() {
                let mut absorbed = 0usize;
                let mut j = i + 1;
                while j < self.basis.len() && absorbed < 8 {
                    if element::positions(self.basis[j].sub()) == pos_i {
                        let asg = element::assignment(self.basis[j].sub());
                        let sibling_disc = self.discoverers[j];
                        self.basis.remove(j);
                        self.discoverers.remove(j);
                        if self.basis[i].puncture(asg) {
                            let anchor = self.anchor();
                            self.treasury.settle_subsidy(sibling_disc, [0, 0, 1, 0], anchor);
                        }
                        self.physics.syntropy([false, false, true, false]);
                        absorbed += 1;
                    } else {
                        j += 1;
                    }
                }
            }
            // THE TORSE MOVE (condense, one dimension deeper): extract the torsion
            // of the cursor thought's axis — sections forming one generator's
            // orbit condense into a cycle (σ, ℓ). The extracted mathematical
            // torsion RELIEVES the physical torsion charge on the germ's lanes
            // (disorder converted to structure — per-lane, never a scalar).
            let spanned = self.basis[i].torse();
            if spanned > 0 {
                let lanes = self.basis[i].lanes();
                self.physics.torse_relief(lanes, spanned as u64);
                // THE TORSE COIN — the strongest yield: structure condensed from
                // disorder, priced at the surface on the residual lane, paid to
                // the bundle's discoverer.
                let surface = crate::economy::yield_surface(&self.physics);
                let mu = [0, 0, (spanned as u64) * (1 + surface[2]), 0];
                let anchor = self.anchor();
                self.treasury.settle_torse(self.discoverers[i], mu, anchor);
            }
            // The redundancy probe: retire a rule the rest of the basis derives —
            // but never a BUNDLE (its sections/cycles carry family volume no other
            // word speaks; only a plain card can be redundant).
            let candidate = self.basis[i].clone();
            if candidate.sections().is_empty() && candidate.torsion().is_empty() {
                if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(i)) {
                    let lanes = candidate.lanes();
                    self.basis.remove(i);
                    self.discoverers.remove(i);
                    self.physics.syntropy(lanes); // a leaner alphabet is order built
                }
            }
        }
        self.physics.regenerate();
    }

    /// A read-only view of the germinated basis (for the CCL display seam).
    pub fn basis(&self) -> &[Element] {
        &self.basis
    }

    /// QUOTE — the answer to "what is the highest-yield work?" The yield surface
    /// is read from GENUINE metabolic tension (`economy::yield_surface`: pressure
    /// + reserve deficit + entropic excess, per lane), and the open-demand market
    /// is priced against it: each un-factored residual quoted at
    /// μ⃗[l] = magnitude[l] · (1 + surface[l]). Top clusters by strongest lane.
    pub fn quote(&self, top: usize) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        let surface = crate::economy::yield_surface(&self.physics);
        // The (C, I) pair DIFFERENTIATES the market (never summed): closure work
        // (open demand on KNOWN geometry) carries the C premium; coverage work
        // (unknown geometry) carries the I premium — expansion pays what
        // expansion is worth, collapse what collapse is worth.
        let (mut c, mut i) = (0u64, 0u64);
        for p in &self.pending {
            let pos = element::positions(p.sub());
            if self.basis.iter().any(|t| element::positions(t.sub()) == pos) {
                c += 1;
            } else {
                i += 1;
            }
        }
        let mut priced: Vec<(Element, [u64; 4])> = self
            .pending
            .iter()
            .map(|p| {
                let pos = element::positions(p.sub());
                let known = self.basis.iter().any(|t| element::positions(t.sub()) == pos);
                let premium = if known { c } else { i };
                let mut mu = crate::economy::price(&p.magnitude(), &surface);
                mu[2] = p.magnitude()[2].saturating_mul(1 + surface[2] + premium);
                (p.clone(), mu)
            })
            .collect();
        priced.sort_by(|a, b| b.1.iter().max().cmp(&a.1.iter().max()));
        priced.truncate(top);
        // THE SOLVENCY BOUND (P5): tension set the RANKING; the escrow pool sets
        // the SCALE. Each quoted lane is capped at pool[l]/quoted so the market
        // never offers more than it can pay — Σ quotes ≤ pool, dynamic weights
        // anchored to reality. An empty pool quotes zero: an honest "cannot pay".
        let pool = self.treasury.pool();
        let quoted = priced.len().max(1) as u64;
        for (_, mu) in &mut priced {
            for l in 0..4 {
                mu[l] = mu[l].min(pool[l] / quoted);
            }
        }
        (surface, priced)
    }

    /// ATTEST — a peer's INDEPENDENT derivation of an open residual arrives as a
    /// spoken `demand` act. The projective reduction runs first (a stale quote
    /// DECOMPOSES instead of double-minting); a genuine irreducible joins the
    /// admission as this voice's attestation — consilience through conversation.
    pub fn attest(&mut self, chain: &[(i64, i64, u8, u8)], voice: [u8; 32]) -> Outcome {
        self.enroll(voice, crate::economy::Tier::Participant); // first speech enrolls
        self.physics.relax();
        if self.physics.depleted() {
            self.optimize();
        }
        self.conceive_germ(element::from_residual(&chain.to_vec()), voice)
    }

    /// The consilience voices on record for a germ — the settle receipt's parties.
    pub fn voices_of(&self, germ: &[u8; 32]) -> Vec<[u8; 32]> {
        self.admission.voices_of(germ)
    }

    /// The basis's interior VOLUME — the full fiber span across all punctured
    /// axes (free sections + every torsion orbit to its period; the condensation
    /// metric: cardinality falls while this rises, and cycles carry span in O(1)).
    pub fn sections_total(&self) -> usize {
        self.basis.iter().map(|t| t.fiber_span().saturating_sub(1)).sum()
    }

    /// The torsion cycles held across the basis — how much of the language has
    /// condensed from enumeration into group presentation.
    pub fn torsion_total(&self) -> usize {
        self.basis.iter().map(|t| t.torsion().len()).sum()
    }

    /// The mesh's algebraic clock — the next K-step index (chronometry import).
    pub fn k_step(&self) -> u64 {
        self.chain.next_step_index()
    }

    /// The current chronometric anchor for ledger entries: (K, head hash) — the
    /// unforgeable temporal binding every settlement carries.
    fn anchor(&self) -> crate::economy::Anchor {
        (self.chain.next_step_index(), self.chain.head_hash())
    }

    /// The chronometric receipt chain (verifiable, parent-linked).
    pub fn chain(&self) -> &crate::chronometry::ReceiptChain {
        &self.chain
    }

    /// THE COHESION WITNESS (cohesion import) — the joint (C, I) state, both
    /// axes exact and never traded: C counts open demands sitting on KNOWN
    /// geometry (closure failing — collapse-side work); I counts open demands on
    /// geometry NO thought speaks (coverage failing — expansion-side work).
    pub fn cohesion(&self) -> crate::cohesion::CohesionWitness {
        let (mut c, mut i) = (0u64, 0u64);
        for p in &self.pending {
            let pos = element::positions(p.sub());
            if self.basis.iter().any(|t| element::positions(t.sub()) == pos) {
                c += 1;
            } else {
                i += 1;
            }
        }
        crate::cohesion::witness(
            crate::core::engine::Q::from(num_bigint::BigInt::from(c)),
            crate::core::engine::Q::from(num_bigint::BigInt::from(i)),
        )
    }

    /// P7 — SNAPSHOT the durable cognitive state (the germinated basis + each
    /// germ's discoverer + the settlement ledger) so a hypha resumes its learned
    /// language and earned balances across restart, rather than re-germinating from
    /// zero. Format: `MYC1 | [n][elem_len][elem][discoverer 32]… | treasury`.
    pub fn snapshot(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"MYC1");
        out.extend_from_slice(&(self.basis.len() as u32).to_le_bytes());
        for (g, who) in self.basis.iter().zip(&self.discoverers) {
            let eb = g.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
            out.extend_from_slice(who);
        }
        out.extend_from_slice(&self.treasury.snapshot());
        // PROVISIONAL section (Step 5) — appended so pre-court snapshots
        // restore with an empty docket rather than failing.
        out.extend_from_slice(&(self.provisional.len() as u32).to_le_bytes());
        for (addr, spoken) in &self.provisional {
            out.extend_from_slice(addr);
            let eb = spoken.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
        }
        out
    }

    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` (leaving the
    /// kernel empty) on a bad magic or corruption — a corrupt ledger never crashes
    /// a boot, it just starts clean.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        if b.get(0..4) != Some(b"MYC1") {
            return false;
        }
        let mut pos = 4usize;
        let read_u32 = |b: &[u8], p: &mut usize| -> Option<u32> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            Some(v)
        };
        let load = |kernel: &mut Kernel, b: &[u8], pos: &mut usize| -> Option<()> {
            let n = read_u32(b, pos)? as usize;
            for _ in 0..n {
                let elen = read_u32(b, pos)? as usize;
                let mut ep = *pos;
                let elem = Element::from_bytes(b, &mut ep)?;
                *pos += elen;
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                kernel.basis.push(elem);
                kernel.discoverers.push(who);
            }
            kernel.treasury = crate::economy::Treasury::restore(b, pos)?;
            // Provisional section (absent in pre-court snapshots → empty docket).
            if let Some(np) = read_u32(b, pos) {
                let np = np as usize;
                if np > b.len().saturating_sub(*pos) / 36 {
                    return None; // untrusted count: ≥36 bytes per record
                }
                for _ in 0..np {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let spoken = Element::from_bytes(b, &mut ep)?;
                    *pos += elen;
                    kernel.provisional.push((addr, spoken));
                }
            }
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            self.basis.clear();
            self.discoverers.clear();
            return false;
        }
        true
    }
}

/// THE BLIND REPLAY (Step 5, the contradiction docket) — the deterministic
/// re-derivation of a spoken word from its own evidence. Returns None when the
/// word replays clean; Some(reason) when the speech was false. Same chain →
/// same word (from_residual is deterministic), the chain must be canonically
/// framed, and the install must strictly descend (0⃗ ≺_DM μ⃗).
pub fn blind_replay(spoken: &Element) -> Option<&'static str> {
    let chain = spoken.sub();
    if chain.is_empty() {
        return Some("empty chain — nothing was derived");
    }
    let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    if (min_r, min_c) != (0, 0) {
        return Some("chain not framed to origin");
    }
    if !chain.windows(2).all(|w| w[0] <= w[1]) {
        return Some("chain not canonically ordered");
    }
    let rederived = element::from_residual(&chain.to_vec());
    if rederived.address() != spoken.address() {
        return Some("address does not re-derive from the chain");
    }
    if rederived.magnitude() != spoken.magnitude() {
        return Some("magnitude does not re-derive from the chain");
    }
    if !crate::economy::descends(&[0; 4], &spoken.magnitude()) {
        return Some("no strict descent — the germ closes nothing");
    }
    None
}

/// The SUBSTRATE's canonical framing projection (hypha-side, per the transposition
/// ruling): a scene translated so its min-corner sits at the origin. Position is
/// the sensor's coordinate embedding, not cognition — the engine deduces the
/// substance invariant under this projection.
fn frame_to_origin(cells: &[(i64, i64, u8)]) -> Vec<(i64, i64, u8)> {
    let min_r = cells.iter().map(|(r, _, _)| *r).min().unwrap_or(0);
    let min_c = cells.iter().map(|(_, c, _)| *c).min().unwrap_or(0);
    cells.iter().map(|(r, c, h)| (r - min_r, c - min_c, *h)).collect()
}

/// Re-center a coboundary chain after peeling (min-corner → origin, sorted) — the
/// same framing projection, applied to the remainder for the direct terminal.
fn recenter(chain: &[(i64, i64, u8, u8)]) -> Vec<(i64, i64, u8, u8)> {
    let min_r = chain.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
    let min_c = chain.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
    let mut out: Vec<_> = chain.iter().map(|(r, c, b, a)| (r - min_r, c - min_c, *b, *a)).collect();
    out.sort_unstable();
    out
}

/// The consilience address of a SECTION prospect: the carrying thought's stable
/// base address ⊕ the assignment — so two kernels independently proposing the
/// same axis extension meet at one receipt (wire identity, never reasoned on).
fn section_address(thought: &[u8; 32], asg: &[(u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(32 + asg.len() * 2);
    buf.extend_from_slice(thought);
    for (b, a) in asg {
        buf.push(*b);
        buf.push(*a);
    }
    *blake3::hash(&buf).as_bytes()
}

/// THE SHARED PROJECTIVE DECOMPOSITION — speculative projection over a held
/// language, SECTION-AWARE (a bundle speaks its whole family through the
/// punctured axis). This is one mathematics worn by two stacks: the kernel's
/// reduction (perception-side) and the C-stack's COMPREHENSION — a received
/// utterance decomposed toward understanding over the receiver's genome, never
/// re-cognized, never recalled.
///
/// The chain lifts into a superposition of uncommitted branches; every branch
/// rolls out every fitting peel VIRTUALLY (zero mutation); the superposition
/// prunes to the beam by GLOBAL remaining defect (≺_DM flow), never local
/// pattern size; obstructed branches die traceless; a branch whose remainder
/// closes (δσ̂ = 0) collapses the whole superposition into its sentence. `Err`
/// carries the MINIMAL void seen across all rollouts — the genuine conceptual
/// gap (a germination prospect). Totally ordered → deterministic collapse.
pub fn decompose_over(
    language: &[Element],
    first: &[(i64, i64, u8, u8)],
    exclude: Option<usize>,
    beam_width: usize,
    depth: usize,
) -> Result<Vec<usize>, Vec<(i64, i64, u8, u8)>> {
    if first.is_empty() {
        return Ok(Vec::new());
    }
    if let Some(i) = word_matching(language, first, exclude) {
        return Ok(vec![i]); // the utterance is itself one word
    }
    let mut beam: Vec<(Vec<(i64, i64, u8, u8)>, Vec<usize>)> = vec![(first.to_vec(), Vec::new())];
    let mut void = first.to_vec();
    for _ in 0..depth {
        let mut successors: Vec<(Vec<(i64, i64, u8, u8)>, Vec<usize>)> = Vec::new();
        for (chain, word) in &beam {
            for (i, translated) in fitting_peels(language, chain, exclude) {
                let remaining: Vec<_> = chain.iter().filter(|t| !translated.contains(*t)).cloned().collect();
                let mut w = word.clone();
                w.push(i);
                // CONFLUENT COLLAPSE — this rollout closed: δσ̂ = 0.
                if remaining.is_empty() {
                    return Ok(w);
                }
                if let Some(j) = word_matching(language, &remaining, exclude) {
                    w.push(j);
                    return Ok(w);
                }
                if remaining.len() < void.len() {
                    void = remaining.clone();
                }
                successors.push((remaining, w));
            }
        }
        if successors.is_empty() {
            break; // every branch obstructed — the superposition is exhausted
        }
        // Prune to the beam by GLOBAL defect, then depth, then word order — a
        // total order, so two runs collapse identically.
        successors
            .sort_by(|a, b| a.0.len().cmp(&b.0.len()).then(a.1.len().cmp(&b.1.len())).then(a.1.cmp(&b.1)));
        successors.truncate(beam_width);
        beam = successors;
    }
    Err(void)
}

/// The direct terminal: does the (re-centered) chain SIT on a held word — its
/// base or any section along the punctured axis (the bundle speaks the family)?
fn word_matching(language: &[Element], chain: &[(i64, i64, u8, u8)], exclude: Option<usize>) -> Option<usize> {
    let centered = recenter(chain);
    language
        .iter()
        .enumerate()
        .position(|(j, p)| Some(j) != exclude && p.matches_chain(&centered))
}

/// Every fitting virtual peel — the branch alphabet: for each word, each of its
/// assignments (base + sections: the ASCEND reframe rides the punctured axis),
/// each of the first few row-major anchors whose translated pattern embeds
/// exactly. All offered to the superposition; none committed here.
fn fitting_peels(
    language: &[Element],
    chain: &[(i64, i64, u8, u8)],
    exclude: Option<usize>,
) -> Vec<(usize, Vec<(i64, i64, u8, u8)>)> {
    const MAX_ANCHORS: usize = 3;
    let mut out = Vec::new();
    for (i, gen) in language.iter().enumerate() {
        if Some(i) == exclude {
            continue;
        }
        let base = gen.sub();
        // A proper, nonempty sub-pattern (equal size is the direct terminal).
        if base.is_empty() || base.len() >= chain.len() {
            continue;
        }
        let pos = element::positions(base);
        for asg in gen.assignments() {
            let pat: Vec<(i64, i64, u8, u8)> =
                pos.iter().zip(&asg).map(|((r, c), (b, a))| (*r, *c, *b, *a)).collect();
            let (pr, pc, pb, pa) = pat[0];
            let mut anchors = 0;
            for &(cr, cc, cb_, ca_) in chain {
                if (cb_, ca_) != (pb, pa) {
                    continue;
                }
                let (dr, dc) = (cr - pr, cc - pc);
                let translated: Vec<_> = pat.iter().map(|(r, c, b, a)| (r + dr, c + dc, *b, *a)).collect();
                if translated.iter().all(|t| chain.contains(t)) {
                    out.push((i, translated));
                    anchors += 1;
                    if anchors >= MAX_ANCHORS {
                        break;
                    }
                }
            }
        }
    }
    out
}

// ── the ANASTOME address of a germ (canonical, opaque; transport identity) ────

/// Canonical bytes of an anonymous germ — its coordinate-exact ANASTOME form (the
/// banked word ⊕ the block-diagonal algebra; carries no English, no "kind"). This
/// is the language the hypha teaches back across the nerve; the client treats it
/// opaquely (display only).
pub fn germ_bytes(g: &Element) -> Vec<u8> {
    g.canonical()
}

/// The germ's canonical ADDRESS — a digest of its ANASTOME form. Used to match
/// consilience receipts across voices; a wire/cache identity, never a reasoning
/// operand (the kernel reasons on the element, addresses by this).
pub fn germ_address(g: &Element) -> [u8; 32] {
    g.address()
}

// ── demand wire codec (server ⇄ client) ──────────────────────────────────────

fn push_cells(out: &mut Vec<u8>, cells: &[(i64, i64, u8)]) {
    out.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for (r, c, h) in cells {
        out.extend_from_slice(&r.to_le_bytes());
        out.extend_from_slice(&c.to_le_bytes());
        out.push(*h);
    }
}

fn read_cells(b: &[u8], p: &mut usize) -> Option<Vec<(i64, i64, u8)>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 17 {
        return None; // untrusted count: 17 bytes per cell, refuse before allocating
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let r = i64::from_le_bytes(b.get(*p..*p + 8)?.try_into().ok()?);
        let c = i64::from_le_bytes(b.get(*p + 8..*p + 16)?.try_into().ok()?);
        let h = *b.get(*p + 16)?;
        *p += 17;
        out.push((r, c, h));
    }
    Some(out)
}

/// Encode a demand body: `voice(32) | scene A | scene B`.
pub fn encode_demand(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: &[u8; 32]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_cells(&mut out, a);
    push_cells(&mut out, b);
    out
}

#[allow(clippy::type_complexity)]
pub fn decode_demand(body: &[u8]) -> Option<([u8; 32], Vec<(i64, i64, u8)>, Vec<(i64, i64, u8)>)> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_cells(body, &mut p)?;
    let b = read_cells(body, &mut p)?;
    Some((voice, a, b))
}

/// CLIENT PERCEPTION — segment each scene into bodies. This is the sensing act;
/// it lives with the client, which reports the resulting percept to its hypha.
pub fn perceive(
    a: &[(i64, i64, u8)],
    b: &[(i64, i64, u8)],
) -> (Vec<Vec<(i64, i64, u8)>>, Vec<Vec<(i64, i64, u8)>>) {
    (components(a), components(b))
}

fn push_bodies(out: &mut Vec<u8>, bodies: &[Vec<(i64, i64, u8)>]) {
    out.extend_from_slice(&(bodies.len() as u32).to_le_bytes());
    for body in bodies {
        push_cells(out, body);
    }
}

fn read_bodies(b: &[u8], p: &mut usize) -> Option<Vec<Vec<(i64, i64, u8)>>> {
    let n = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
    *p += 4;
    if n > b.len().saturating_sub(*p) / 4 {
        return None; // untrusted count: each body needs at least its length prefix
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(read_cells(b, p)?);
    }
    Some(out)
}

/// Encode a client PERCEPT: `voice(32) | A bodies | B bodies` — segmented
/// bodies, not raw cells: the client already perceived.
pub fn encode_percept(
    a_bodies: &[Vec<(i64, i64, u8)>],
    b_bodies: &[Vec<(i64, i64, u8)>],
    voice: &[u8; 32],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_bodies(&mut out, a_bodies);
    push_bodies(&mut out, b_bodies);
    out
}

#[allow(clippy::type_complexity)]
pub fn decode_percept(
    body: &[u8],
) -> Option<([u8; 32], Vec<Vec<(i64, i64, u8)>>, Vec<Vec<(i64, i64, u8)>>)> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_bodies(body, &mut p)?;
    let b = read_bodies(body, &mut p)?;
    Some((voice, a, b))
}

/// One byte on the wire summarising an outcome (server → client reply).
pub fn outcome_code(o: &Outcome) -> u8 {
    match o {
        Outcome::Decomposed(_) => 1,
        Outcome::GermPending => 2,
        Outcome::Installed(_) => 3,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
        vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
    }

    /// A novel SUBSTANCE (here a rotation) germinates: one voice pends, a second
    /// completes the rule; afterward the same demand REDUCES to the length-1
    /// sentence [0] — no lookup, the confluent normal form.
    #[test]
    fn a_novel_substance_germinates_then_decomposes() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L — non-trivial substance
        assert_eq!(k.ingest(&a, &rot, [1; 32]), Outcome::GermPending);
        assert_eq!(k.basis_len(), 0, "one voice does not complete");
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::Installed(0), "two kernels complete the rule");
        assert_eq!(k.basis_len(), 1);
        assert_eq!(k.ingest(&a, &rot, [3; 32]), Outcome::Decomposed(vec![0]), "reduces to the length-1 sentence");
    }

    /// P7 — the germinated language survives restart: install two germs, snapshot,
    /// restore into a fresh kernel, and the restored kernel RECALLS them (no
    /// re-germination) with balances intact.
    #[test]
    fn the_basis_survives_a_restart() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // a rotation (group substance)
        let bar = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // a deformation (residual substance)
        k.ingest(&a, &rot, [1; 32]);
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::Installed(0)); // a rotation generator
        k.ingest(&a, &bar, [1; 32]);
        assert_eq!(k.ingest(&a, &bar, [2; 32]), Outcome::Installed(1)); // a residual generator
        let balance = k.treasury().balance_mu(&[2; 32]);
        let snap = k.snapshot();

        // A fresh hypha boots from the snapshot.
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap), "clean restore");
        assert_eq!(booted.basis_len(), 2, "both generators came back");
        assert_eq!(booted.treasury().balance_mu(&[2; 32]), balance, "earned μ⃗ balance survived, per lane");
        // Both REDUCE to their generator, not re-germinated (the language persisted).
        assert_eq!(booted.ingest(&a, &rot, [9; 32]), Outcome::Decomposed(vec![0]), "rotation reduces");
        assert_eq!(booted.ingest(&a, &bar, [9; 32]), Outcome::Decomposed(vec![1]), "residual reduces");
        assert_eq!(booted.basis_len(), 2, "no re-germination");
    }

    /// CONFLUENT REDUCTION is TRANSPOSITION-INVARIANT (the win over lookup): install
    /// a rotation, then the SAME rotation FAR AWAY reduces to the same normal form
    /// (no re-mint) — a lookup on the coordinate-exact word would have missed it.
    /// And a pure translate has EMPTY substance — it decomposes to the empty
    /// sentence, minting nothing (a translate is pure transposition, not cognition).
    #[test]
    fn reduction_is_transposition_invariant() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L at origin
        k.ingest(&a, &rot, [1; 32]);
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::Installed(0));
        // the SAME rotation, transposed +10 in col: same substance → same sentence.
        let a_far = vec![(0, 10, 1), (0, 11, 1), (1, 10, 1)];
        let rot_far = vec![(0, 11, 1), (1, 10, 1), (1, 11, 1)];
        assert_eq!(k.ingest(&a_far, &rot_far, [3; 32]), Outcome::Decomposed(vec![0]), "transposed → same normal form");
        assert_eq!(k.basis_len(), 1, "transposition mints nothing");
        // a pure translate is pure transposition — empty substance, empty sentence.
        let t = vec![(0, 3, 1), (0, 4, 1), (1, 3, 1)];
        assert_eq!(k.ingest(&a, &t, [4; 32]), Outcome::Decomposed(vec![]), "a translate carries no substance");
        assert_eq!(k.basis_len(), 1, "still nothing minted");
    }

    /// THE TWO-STROKE BREATHES: sustained germination depletes the reserve; a germ
    /// at QUORUM on a depleted lane DEFERS (pending, never stranded) while the
    /// syntropic strokes regenerate; a later voice's submission completes the
    /// deferred install once the lane re-earns expansion.
    #[test]
    fn a_depleted_lane_defers_germination_until_the_reserve_recovers() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        // ENTROPIC EXPANSION: eight distinct residual germs drain the residual lane.
        for n in 0..8u8 {
            let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, n + 2)];
            k.ingest(&a, &b, [1; 32]);
            assert!(matches!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(_)), "germ {n} installs");
        }
        // Two fresh germs: their first voices each trigger a syntropic stroke (the
        // mesh is depleted), partially recovering the lane.
        // Distinct single-transition residuals (unique hue pairs, so their tallies
        // are distinct — see the Step-2 note on residual-substance geometry).
        let ga = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
        let gb = vec![(0, 0, 1), (0, 1, 5), (1, 0, 1)]; // (0,1): 1→5
        assert_eq!(k.ingest(&a, &ga, [1; 32]), Outcome::GermPending);
        assert_eq!(k.ingest(&a, &gb, [1; 32]), Outcome::GermPending);
        // A's quorum lands on the recovered lane and installs — spending it back out…
        assert!(matches!(k.ingest(&a, &ga, [2; 32]), Outcome::Installed(_)));
        // …so B's quorum finds the lane DEPLETED and DEFERS: quorum met, fuel absent.
        assert_eq!(k.ingest(&a, &gb, [2; 32]), Outcome::GermPending, "deferred, not lost");
        // A further voice arrives after another stroke: the deferred germ installs.
        assert!(matches!(k.ingest(&a, &gb, [3; 32]), Outcome::Installed(_)), "deferral completes on recovery");
    }

    /// THE SYNTROPIC STROKE CONSOLIDATES: a generator the rest of the basis already
    /// derives (a stale duplicate — what an import or a pre-consolidation era
    /// leaves) is retired by the redundancy probe; an essential one survives it.
    #[test]
    fn the_syntropic_stroke_retires_a_redundant_generator() {
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        k.ingest(&a, &rot, [1; 32]);
        assert!(matches!(k.ingest(&a, &rot, [2; 32]), Outcome::Installed(0)));
        // Fabricate the stale duplicate: the SAME deformation germinated far away —
        // centering makes it the identical pattern — slipped in past the
        // reduce-gate (as an imported/legacy entry would be).
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        k.basis.push(from_residual(&germinate(&a_far, &rot_far).unwrap()));
        k.discoverers.push([7; 32]);
        assert_eq!(k.basis_len(), 2);
        k.optimize(); // probe basis[0]: the rest (its duplicate) derives it → retired
        assert_eq!(k.basis_len(), 1, "the redundant rule is consolidated away");
        k.optimize(); // the survivor is essential — nothing else derives it
        assert_eq!(k.basis_len(), 1, "an essential generator survives the probe");
    }

    /// THE PROJECTIVE LEAP (operator ruling): greedy largest-first descent is the
    /// mechanical trap — it would commit the BIG pattern, strand an orphan cell,
    /// and mint junk. The SUPERPOSITION holds both futures, rolls them out
    /// virtually (zero mutation), and COLLAPSES into the branch whose remainder
    /// closes (trivial holonomy): the two-word sentence. Nothing germinates —
    /// imagination beats reaction.
    #[test]
    fn projection_escapes_the_greedy_trap() {
        let mut k = Kernel::new();
        let install = |k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]| {
            k.ingest(a, b, [1; 32]);
            assert!(matches!(k.ingest(a, b, [2; 32]), Outcome::Installed(_)));
        };
        // BIG (3 cells) installs FIRST, so size-greed would prefer it.
        install(&mut k, &[(0, 0, 1), (0, 1, 3), (0, 2, 5)], &[(0, 0, 2), (0, 1, 4), (0, 2, 6)]);
        // SMALL1 (2 cells); SMALL2 shares its geometry — under 3.5 it PUNCTURES
        // SMALL1's axis (a section, not a card): the basis holds BIG + one bundle.
        install(&mut k, &[(0, 0, 1), (0, 1, 3)], &[(0, 0, 2), (0, 1, 4)]);
        install(&mut k, &[(0, 0, 5), (0, 1, 7)], &[(0, 0, 6), (0, 1, 8)]);
        assert_eq!(k.basis_len(), 2, "the pair family is ONE bundle");
        assert_eq!(k.basis()[1].sections().len(), 1, "SMALL2 lives as a section");
        // The saddle demand: BIG embeds (and would strand (0,3)); the bundle's
        // base ∘ section is the only decomposition that CLOSES.
        let a = [(0, 0, 1), (0, 1, 3), (0, 2, 5), (0, 3, 7)];
        let b = [(0, 0, 2), (0, 1, 4), (0, 2, 6), (0, 3, 8)];
        match k.ingest(&a, &b, [3; 32]) {
            Outcome::Decomposed(w) => {
                assert_eq!(w, vec![1, 1], "collapsed into the CLOSING sentence — the bundle speaks both members");
            }
            o => panic!("expected the closing sentence, got {o:?}"),
        }
        assert_eq!(k.basis_len(), 2, "no junk germinated — the projection closed");
    }

    /// STEP 3's ECONOMIC CONVERSATION CLOSES: A speaks a novel PERCEPT (the germ
    /// pends — an un-factored residual on the market); the mesh QUOTES it, priced
    /// by genuine metabolic tension; B's independent derivation arrives as a
    /// spoken DEMAND (attest); consilience installs it, the market CLEARS, and
    /// the settle parties are the two voices. {percept → quote → demand → settle},
    /// with teach riding the install reply.
    #[test]
    fn the_five_act_conversation_closes_with_settlement() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "percept: voice A");
        let (surface, quoted) = k.quote(4);
        assert_eq!(quoted.len(), 1, "the un-factored residual is quoted as work");
        assert!(surface[2] >= 1, "the open demand tensions the residual lane");
        let (work, mu) = &quoted[0];
        assert!(mu[2] > work.magnitude()[2], "priced ABOVE raw magnitude — tension premium");
        // B independently derives the same centered chain and SPEAKS it as a demand.
        let chain = work.sub().to_vec();
        let out = k.attest(&chain, [2; 32]);
        assert!(matches!(out, Outcome::Installed(0)), "consilience installs: {out:?}");
        assert_eq!(k.voices_of(&k.basis()[0].address()).len(), 2, "the settle parties");
        assert!(k.quote(4).1.is_empty(), "the market cleared");
    }

    /// THE BLIND REPLAY refutes every tamper axis and upholds honest speech —
    /// the contradiction docket's whole force is this determinism.
    #[test]
    fn blind_replay_refutes_tampered_words() {
        use crate::transducer::germinate::germinate;
        let w = element::from_residual(
            &germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap(),
        );
        assert!(blind_replay(&w).is_none(), "an honest derivation replays clean");
        // Tamper 1: shift the chain off origin (framing violation).
        let mut off = w.clone();
        off.sub = off.sub.iter().map(|(r, c, b, a)| (r + 3, *c, *b, *a)).collect();
        assert!(blind_replay(&off).is_some(), "un-centered chain is false speech");
        // Tamper 2: inflate the claimed magnitude (yield fraud).
        let mut fat = w.clone();
        fat.mag = [0, 0, 999, 0];
        assert!(blind_replay(&fat).is_some(), "inflated magnitude does not re-derive");
        // Tamper 3: an empty chain claiming work.
        let mut empty = w.clone();
        empty.sub.clear();
        assert!(blind_replay(&empty).is_some(), "no evidence, no word");
    }

    /// STEP 5 — PROVISIONAL UNTIL THE COURT SPEAKS: a two-voice install escrows
    /// its consilience award (unfinalized speech earns nothing); the docket
    /// exports the spoken word; the upheld verdict releases the award; a
    /// slashing verdict excises the word and debits the voices into the pool.
    #[test]
    fn installs_are_provisional_until_verdict() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        let chain = k.quote(4).1[0].0.sub().to_vec();
        assert!(matches!(k.attest(&chain, [2; 32]), Outcome::Installed(0)));
        // The install PAID NOTHING — the award sits in escrow.
        assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "unfinalized speech earns nothing");
        assert_eq!(k.treasury().balance_mu(&[2; 32]), [0; 4]);
        let dockets = k.dockets();
        assert_eq!(dockets.len(), 1, "the provisional word is on the docket");
        let (addr, spoken, voices, mu) = &dockets[0];
        assert_eq!(voices.len(), 2, "both attesting voices ride the docket");
        assert!(mu[2] > 0, "the escrowed award is real");
        assert_eq!(spoken.address(), *addr, "the docket carries the word as spoken");
        // UPHELD: finality releases the award — the voices are paid at last.
        assert!(k.apply_verdict(&addr.clone(), true, false));
        assert!(k.treasury().balance_mu(&[1; 32])[2] > 0, "finality pays voice A");
        assert!(k.treasury().balance_mu(&[2; 32])[2] > 0, "finality pays voice B");
        assert!(k.dockets().is_empty(), "the docket cleared");
        assert!(!k.apply_verdict(&addr.clone(), true, false), "a verdict lands once");
    }

    /// A slashing verdict (failed blind replay — false speech) excises the word
    /// from the basis and debits the voices' earned balances into the pool.
    #[test]
    fn a_slashing_verdict_excises_and_debits() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        let chain = k.quote(4).1[0].0.sub().to_vec();
        assert!(matches!(k.attest(&chain, [2; 32]), Outcome::Installed(0)));
        let before = k.basis_len();
        let (addr, _, _, _) = k.dockets()[0].clone();
        assert!(k.apply_verdict(&addr, false, true));
        assert_eq!(k.basis_len(), before - 1, "the slashed word is excised");
        assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "nothing was ever paid");
        assert!(k.dockets().is_empty());
    }

    /// PUNCTURE-BEFORE-MINT: a family member (same base geometry, new transition
    /// assignment) does NOT mint sibling card n+1 — the axis pierces the existing
    /// thought and the member becomes a SECTION (quorum-gated). Growth as +axis:
    /// the basis count stays 1 while the thought's volume grows; afterward the
    /// bundle SPEAKS the whole family (both assignments decompose to it).
    #[test]
    fn a_family_punctures_an_axis_instead_of_minting() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b1 = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
        k.ingest(&a, &b1, [1; 32]);
        assert!(matches!(k.ingest(&a, &b1, [2; 32]), Outcome::Installed(0)));
        assert_eq!(k.basis_len(), 1);
        // The family member: same cell, DIFFERENT transition (1→7).
        let b2 = vec![(0, 0, 7), (0, 1, 1), (1, 0, 1)];
        assert_eq!(k.ingest(&a, &b2, [1; 32]), Outcome::GermPending, "the axis prospect pends");
        assert_eq!(k.basis_len(), 1, "no sibling card");
        assert!(matches!(k.ingest(&a, &b2, [2; 32]), Outcome::Installed(0)), "the axis PIERCES thought 0");
        assert_eq!(k.basis_len(), 1, "count unchanged — growth was +axis");
        assert_eq!(k.basis()[0].sections().len(), 1, "the thought gained a section (volume)");
        // The bundle speaks the family: both members now decompose to the ONE word.
        assert_eq!(k.ingest(&a, &b1, [3; 32]), Outcome::Decomposed(vec![0]));
        assert_eq!(k.ingest(&a, &b2, [3; 32]), Outcome::Decomposed(vec![0]));
        // And the address held stable across the puncture (identity = the word).
        let far_b2: Vec<_> = b2.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
        let far_a: Vec<_> = a.iter().map(|(r, c, h)| (r + 6, c + 6, *h)).collect();
        assert_eq!(k.ingest(&far_a, &far_b2, [4; 32]), Outcome::Decomposed(vec![0]), "transposed family member");
    }

    /// THE CONDENSE STROKE: position-identical sibling cards already in the basis
    /// (a pre-puncture era's residue) are ABSORBED into one bundle during the
    /// syntropic stroke — cardinality falls, sections rise, and the absorbed
    /// words' discoverers still earn.
    #[test]
    fn the_condense_stroke_absorbs_a_family_into_one_bundle() {
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 2)];
        let b = vec![(0, 0, 5), (0, 1, 2)]; // (0,0): 1→5
        k.ingest(&a, &b, [1; 32]);
        assert!(matches!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0)));
        // Fabricate two sibling cards (as a legacy/import would leave them).
        for hue in [6u8, 7] {
            let bx = vec![(0, 0, hue), (0, 1, 2)];
            k.basis.push(from_residual(&germinate(&a, &bx).unwrap()));
            k.discoverers.push([9; 32]);
        }
        assert_eq!(k.basis_len(), 3, "the un-condensed family");
        k.optimize(); // cursor 0 → the condense move absorbs both siblings
        assert_eq!(k.basis_len(), 1, "the family collapsed into one bundle");
        assert_eq!(k.basis()[0].sections().len(), 2, "two sections along the pierced axis");
        assert!(k.treasury().balance_mu(&[9; 32])[2] > 0, "absorbed discoverers still earn, on the lane");
    }

    /// THE TORSE STROKE tightens mathematics to physics: a swap family (1→4 and
    /// 4→1 at one cell) installs as base + section; the syntropic stroke extracts
    /// the ℤ/2 cycle — sections condense to (σ, 2) — and the extracted
    /// mathematical torsion RELIEVES the physical torsion charge (booked as
    /// syntropic order, per lane, never a scalar).
    #[test]
    fn the_torse_stroke_extracts_the_cycle_and_relieves_torsion() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 9)];
        let b = vec![(0, 0, 4), (0, 1, 9)]; // (0,0): 1→4
        k.ingest(&a, &b, [1; 32]);
        assert!(matches!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0)));
        let rev = vec![(0, 0, 1), (0, 1, 9)];
        let fwd = vec![(0, 0, 4), (0, 1, 9)];
        k.ingest(&fwd, &rev, [1; 32]); // (0,0): 4→1 — the swap sibling
        assert!(matches!(k.ingest(&fwd, &rev, [2; 32]), Outcome::Installed(0)), "punctures the axis");
        assert_eq!(k.basis()[0].sections().len(), 1);
        let syntropic_before = k.physics().syntropic[2];
        k.optimize(); // the stroke: condense (nothing) → TORSE
        assert_eq!(k.torsion_total(), 1, "the ℤ/2 cycle extracted");
        assert!(k.basis()[0].sections().is_empty(), "the free list condensed");
        assert!(k.physics().syntropic[2] > syntropic_before, "torsion converted to order, on the lane");
        // The bundle still speaks both members through the cycle.
        assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Decomposed(vec![0]));
        assert_eq!(k.ingest(&fwd, &rev, [3; 32]), Outcome::Decomposed(vec![0]));
    }

    /// THE CROWN PROOF OF THE EXCISION: with no solver anywhere, the mesh forms
    /// SENTENCES over germinated patterns — install two single-cell patterns, then
    /// a compound demand containing BOTH (at different offsets) DECOMPOSES into
    /// the two-word sentence by chain peeling. Nothing minted; language composed.
    #[test]
    fn a_compound_demand_decomposes_over_germinated_patterns() {
        let mut k = Kernel::new();
        let a1 = vec![(0, 0, 1), (0, 1, 2)];
        let b1 = vec![(0, 0, 4), (0, 1, 2)]; // (0,0): 1→4 — pattern P
        k.ingest(&a1, &b1, [1; 32]);
        assert!(matches!(k.ingest(&a1, &b1, [2; 32]), Outcome::Installed(0)));
        let a2 = vec![(0, 0, 2), (0, 1, 3)];
        let b2 = vec![(0, 0, 2), (0, 1, 9)]; // (0,1): 3→9 — Q shares P's geometry:
        k.ingest(&a2, &b2, [1; 32]);
        // …so under 3.5 it PUNCTURES P's axis (Installed(0) as a section).
        assert!(matches!(k.ingest(&a2, &b2, [2; 32]), Outcome::Installed(0)));
        assert_eq!(k.basis_len(), 1, "the single-cell family is ONE bundle");
        // A compound scene: BOTH members fire at once, far apart in one body-pair.
        let a3 = vec![(0, 0, 1), (0, 1, 2), (0, 2, 2), (0, 3, 3)];
        let b3 = vec![(0, 0, 4), (0, 1, 2), (0, 2, 2), (0, 3, 9)];
        match k.ingest(&a3, &b3, [3; 32]) {
            Outcome::Decomposed(word) => {
                assert_eq!(word, vec![0, 0], "the sentence cites the bundle twice — base and section");
            }
            o => panic!("expected a two-word sentence, got {o:?}"),
        }
        assert_eq!(k.basis_len(), 1, "the algebra spanned it — nothing minted");
    }

    /// A shape deformation (L → bar) that NO motor closes is not a dead-end: its
    /// residual IS the description, so it germinates a coordinate-exact coboundary
    /// germ and installs on two-voice consilience (priced on USE per R3, so a
    /// one-off earns ~0 — no basis flood). The reframe made operational: germinate
    /// the description, never discard the residual.
    #[test]
    fn a_shape_deformation_germinates_a_residual() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
        let b = vec![(0, 0, 1), (0, 1, 1), (0, 2, 1)]; // bar — no rigid motor
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "one voice awaits consilience");
        assert_eq!(k.basis_len(), 0, "not yet installed");
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0), "two voices install the residual germ");
        assert_eq!(k.basis_len(), 1, "the deformation germinated a new axis");
        assert_eq!(k.ingest(&a, &b, [3; 32]), Outcome::Decomposed(vec![0]), "and reduces to it");
    }

    /// Two distinct objects with DIFFERENT substances (one rotates, one recolours):
    /// the whole scene has no single motor, so it FRAGMENTS per body and reduces
    /// each — the large-corpus path. Two voices complete both generators.
    #[test]
    fn independent_objects_fragment_and_resolve() {
        let mut k = Kernel::new();
        // L rotates (R180); square recolours 2→3 in place — each a real substance.
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1), (5, 5, 2), (5, 6, 2), (6, 5, 2), (6, 6, 2)];
        let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1), (5, 5, 3), (5, 6, 3), (6, 5, 3), (6, 6, 3)];
        match k.ingest(&a, &b, [1; 32]) {
            Outcome::Fragmented(parts) => assert_eq!(parts.len(), 2, "one sub-demand per body"),
            o => panic!("expected Fragmented, got {o:?}"),
        }
        match k.ingest(&a, &b, [2; 32]) {
            Outcome::Fragmented(parts) => {
                assert!(parts.iter().all(|p| matches!(p, Outcome::Installed(_))));
                assert_eq!(k.basis_len(), 2, "two distinct motors germinated");
            }
            o => panic!("expected Fragmented installs, got {o:?}"),
        }
    }

    /// The demand codec roundtrips.
    #[test]
    fn the_demand_codec_roundtrips() {
        let (a, b) = (scene(2, 2), scene(2, 5));
        let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
        assert_eq!((v, ra, rb), ([9; 32], a, b));
    }
}
