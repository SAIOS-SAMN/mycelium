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
    pub nerve: Nerve,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    optimize_cursor: usize,
    /// Polytonic kinematic pressure + torsion (per lane).
    physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    treasury: crate::economy::Treasury,
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
        self.physics.relax(); // the metabolism dissipates its backlog as it works
        // THE SYNTROPIC STROKE: when a lane is depleted (the entropic expansion has
        // spent its fuel), the mesh OPTIMIZES — consolidate one generator the rest
        // already derive — which regenerates the reserve. Expansion and optimization
        // are the two strokes of one metabolism.
        if self.physics.depleted() {
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
                self.treasury.settle_recall(voice); // the reader's small yield
                for &i in &word {
                    self.treasury.settle_use(self.discoverers[i]); // each cited generator, used
                }
                self.physics.syntropy(lanes); // a spanned sentence builds order
                Outcome::Decomposed(word)
            }
            NormalForm::Irreducible(residual) => {
                // KB COMPLETION — the irreducible residual adjoins as a NEW generator
                // on two-kernel consilience. Confluence is the gate: two kernels
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
                    self.treasury.settle_consilience(&self.admission.voices_of(&addr));
                    self.physics.relieve(lanes);
                    self.physics.syntropy(lanes); // a completed rule is order built
                    self.physics.spend(lanes); // the expansion SHRINKS the reserve
                    self.basis.push(residual);
                    self.discoverers.push(voice);
                    Outcome::Installed(self.basis.len() - 1)
                } else {
                    self.physics.bump(lanes); // an open critical pair raises pressure
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

    /// CHAIN REDUCTION — the post-excision decomposition. Peel basis patterns that
    /// occur as TRANSLATED sub-coboundaries of the demand's centered chain: each
    /// peel removes the pattern's cells, so the support strictly descends (≺_DM on
    /// the residual lane — termination is structural, no matrix inverse anywhere).
    /// Deterministic choice (largest pattern, then basis order, then the chain's
    /// row-major anchor) → one normal form. Empty ⇒ Decomposed(sentence); nothing
    /// fits ⇒ the leftover, re-centered, is the irreducible critical pair.
    fn reduce_chain(&self, first: Vec<(i64, i64, u8, u8)>, exclude: Option<usize>) -> NormalForm {
        let budget = self.peel_budget();
        let mut chain = first;
        let mut word: Vec<usize> = Vec::new();
        loop {
            if chain.is_empty() {
                return NormalForm::Decomposed(word);
            }
            // Direct terminal (always affordable): the re-centered remainder IS a
            // basis pattern — the sentence's last letter.
            let centered = recenter(&chain);
            if let Some(i) = self
                .basis
                .iter()
                .enumerate()
                .position(|(j, p)| Some(j) != exclude && p.sub() == centered.as_slice())
            {
                word.push(i);
                return NormalForm::Decomposed(word);
            }
            // The metabolic budget bounds the expensive sub-pattern scan.
            if word.len() >= budget {
                return NormalForm::Irreducible(element::from_residual(&chain));
            }
            match self.best_chain_peel(&chain, exclude) {
                Some((i, remaining)) => {
                    word.push(i);
                    chain = remaining;
                }
                None => return NormalForm::Irreducible(element::from_residual(&chain)),
            }
        }
    }

    /// The best basis pattern embedding as a translated sub-coboundary of `chain`:
    /// anchor each generator's first cell on every same-transition chain cell and
    /// test exact embedding. The LARGEST pattern wins (maximal ≺_DM descent),
    /// tie-broken by basis order then the first (row-major) anchor — deterministic.
    fn best_chain_peel(
        &self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
    ) -> Option<(usize, Vec<(i64, i64, u8, u8)>)> {
        let mut best: Option<(usize, usize, Vec<(i64, i64, u8, u8)>)> = None; // (|pat|, i, remaining)
        for (i, gen) in self.basis.iter().enumerate() {
            if Some(i) == exclude {
                continue;
            }
            let pat = gen.sub();
            // Strict descent: a proper, nonempty sub-pattern (the equal-size case
            // is the direct terminal in reduce_chain).
            if pat.is_empty() || pat.len() >= chain.len() {
                continue;
            }
            let (pr, pc, pb, pa) = pat[0];
            for &(cr, cc, cb_, ca_) in chain {
                if (cb_, ca_) != (pb, pa) {
                    continue;
                }
                let (dr, dc) = (cr - pr, cc - pc);
                let translated: Vec<_> = pat.iter().map(|(r, c, b, a)| (r + dr, c + dc, *b, *a)).collect();
                if translated.iter().all(|t| chain.contains(t)) {
                    let better = match &best {
                        Some((blen, _, _)) => pat.len() > *blen,
                        None => true,
                    };
                    if better {
                        let remaining: Vec<_> =
                            chain.iter().filter(|t| !translated.contains(*t)).cloned().collect();
                        best = Some((pat.len(), i, remaining));
                    }
                    break; // first row-major anchor for this generator (chain is sorted)
                }
            }
        }
        best.map(|(_, i, remaining)| (i, remaining))
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
            let candidate = self.basis[i].clone();
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(i)) {
                // The rest of the basis spells it — a redundant rule: retire it.
                let lanes = candidate.lanes();
                self.basis.remove(i);
                self.discoverers.remove(i);
                self.physics.syntropy(lanes); // a leaner alphabet is order built
            }
        }
        self.physics.regenerate();
    }

    /// A read-only view of the germinated basis (for the CCL display seam).
    pub fn basis(&self) -> &[Element] {
        &self.basis
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
        let balance = k.treasury().balance(&[2; 32]);
        let snap = k.snapshot();

        // A fresh hypha boots from the snapshot.
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap), "clean restore");
        assert_eq!(booted.basis_len(), 2, "both generators came back");
        assert_eq!(booted.treasury().balance(&[2; 32]), balance, "earned balance survived");
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
        let b2 = vec![(0, 0, 2), (0, 1, 9)]; // (0,1): 3→9 — pattern Q
        k.ingest(&a2, &b2, [1; 32]);
        assert!(matches!(k.ingest(&a2, &b2, [2; 32]), Outcome::Installed(1)));
        // A compound scene: BOTH patterns fire at once, far apart in one body-pair.
        let a3 = vec![(0, 0, 1), (0, 1, 2), (0, 2, 2), (0, 3, 3)];
        let b3 = vec![(0, 0, 4), (0, 1, 2), (0, 2, 2), (0, 3, 9)];
        match k.ingest(&a3, &b3, [3; 32]) {
            Outcome::Decomposed(word) => {
                let mut w = word.clone();
                w.sort_unstable();
                assert_eq!(w, vec![0, 1], "the compound is the SENTENCE of both patterns");
            }
            o => panic!("expected a two-word sentence, got {o:?}"),
        }
        assert_eq!(k.basis_len(), 2, "the algebra spanned it — nothing minted");
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
