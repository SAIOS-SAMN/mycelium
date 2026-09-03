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
use crate::transducer::blade::{components, lift};
use crate::transducer::element::{self, Element};
use crate::transducer::germinate::germinate;
pub use crate::transducer::sentence::{cited_pattern, replay, verify, Cite};

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
/// a canonical sentence (fully decomposed; indices into the language the
/// reduction ran over) or an irreducible residual (a critical pair to complete
/// by germination).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NormalForm {
    Decomposed(Vec<usize>),
    Irreducible(Element),
}

/// A germ ready for reduction: the element and its anchor context (the framed
/// input body and the offset of the centered chain within it).
pub type Germ = (Element, Option<(Vec<(i64, i64, u8)>, (i64, i64))>);

/// A composite lineage to register: (parent id, children (address, done), the
/// overlap strips the reassembly glues on).
pub type Composite = ([u8; 32], Vec<([u8; 32], bool)>, Vec<(i64, i64, u8, u8)>);

/// THE LANGUAGE SNAPSHOT: the held words and their atlas, immutable and
/// shared by pointer with every reflex worker. Reduction, congruence, and
/// quoting run over it with no lock held.
#[derive(Debug, Clone, Default)]
pub struct Language {
    pub words: Vec<Element>,
    pub stalks: crate::transducer::stalk::Stalks,
    /// Each word's shape, lifted once on first use and shared by every
    /// lookup over this snapshot (a measurement memoized, never persisted).
    pub shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// The retirement generation of the basis this snapshot was taken at.
    pub retire_gen: u64,
    /// Each word reached by its address — a coordinate read, never a scan.
    pub index: crate::network::nerve::Keyed<usize>,
}

impl Language {
    /// The shape of word `i`, lifted on first use.
    pub fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let cell = self.shapes.get(i)?;
        cell.get_or_init(|| element::shape_of_chain(self.words[i].sub())).as_ref()
    }

    /// The index of the word at an address.
    pub fn at(&self, addr: &[u8; 32]) -> Option<usize> {
        self.index.get(addr).copied()
    }
}

/// The market as seen at one instant (see [`Kernel::market_open`]).
pub struct MarketView {
    pub language: std::sync::Arc<Language>,
    pub pending: std::sync::Arc<Vec<(Element, u64)>>,
    pub composites: std::sync::Arc<Vec<Composite>>,
    pub surface: [u64; 4],
    pub pool: [u64; 4],
    pub k_step: u64,
    pub reserve: [u64; 4],
    /// The (basis, book) generations this view was taken at.
    pub gens: (u64, u64),
    /// The sealed rank if it is current at `gens`; absent when either
    /// generation moved, so the reader measures it over the snapshot.
    pub closure: Option<std::sync::Arc<Rank>>,
    /// The last sealed rank regardless of currency — the memo a fresh
    /// measurement reuses closures from at the same basis generation.
    pub prior: std::sync::Arc<Rank>,
}

/// THE RANK OF THE BOOK — a sealed carrier: for each listing its base point
/// in the language (closure) or none (coverage), and the book's order by
/// homological curvature, κ⃗ = μ⃗ / |supp| per lane, lifted by the lineage
/// fraction (resolved / total children of its parent). The order is
/// surface-free, so it holds across strokes; a quote takes its head.
#[derive(Debug, Clone, Default)]
pub struct Rank {
    pub closure: Vec<Option<usize>>,
    pub order: Vec<usize>,
    /// Per listing, the lineage fraction as (resolved, total) — (0, 0) when
    /// the listing has no parent.
    pub lineage: Vec<(u64, u64)>,
    /// Per listing, its address — the coordinate a later rank reuses the
    /// closure by when the basis has not moved.
    pub addresses: Vec<[u8; 32]>,
    /// Per listing, its moduli — the coordinate that says whether a word
    /// added since could have closed it.
    pub invariants: Vec<crate::transducer::stalk::Invariant>,
    /// Each listing's position reached by its address.
    pub index: crate::network::nerve::Keyed<usize>,
    /// The basis generation the closure was measured at.
    pub basis_gen: u64,
    /// The basis length and retirement generation at measurement: with no
    /// retirement since, every prior closure index still names its word.
    pub basis_len: usize,
    pub retire_gen: u64,
}

/// The lineage coordinate of a listing: its parent's (resolved, total,
/// ribbon volume), found by the listing's own address in the composites
/// relation. (0, 0, 0) when it has no parent.
fn lineage_of(addr: &[u8; 32], composites: &[Composite]) -> (u64, u64, u64) {
    composites
        .iter()
        .find(|(_, children, _)| children.iter().any(|(a, _)| a == addr))
        .map(|(_, children, ribbon)| {
            (
                children.iter().filter(|(_, done)| *done).count() as u64,
                children.len() as u64,
                ribbon.len() as u64,
            )
        })
        .unwrap_or((0, 0, 0))
}

/// Measure the rank of the book — pure.
pub fn rank_over(
    language: &Language,
    pending: &[(Element, u64)],
    composites: &[Composite],
    prior: &Rank,
    basis_gen: u64,
) -> Rank {
    let stalks = &language.stalks;
    use crate::core::engine::Q;
    use num_bigint::BigInt;
    use num_traits::{One, Zero};
    // A listing's base point moves only when a word lands in its own
    // stratum. At the same basis generation every prior closure is reused
    // by the listing's address. When the basis grew with no retirement,
    // the words added since name the strata that moved, and only listings
    // in those strata are located again — a coordinate read, never the
    // whole book.
    let addresses: Vec<[u8; 32]> = pending.iter().map(|(p, _)| p.address()).collect();
    let incremental = prior.basis_gen != basis_gen
        && prior.retire_gen == language.retire_gen
        && language.words.len() >= prior.basis_len
        && prior.invariants.len() == prior.addresses.len();
    let mut moved: Vec<crate::transducer::stalk::Invariant> = Vec::new();
    if incremental {
        for w in &language.words[prior.basis_len..] {
            let inv = crate::transducer::stalk::invariant(w.sub());
            if !moved.contains(&inv) {
                moved.push(inv);
            }
        }
    }
    let prior_at: Vec<Option<usize>> = addresses.iter().map(|addr| prior.index.get(addr).copied()).collect();
    let mut index = crate::network::nerve::Keyed::new();
    for (i, addr) in addresses.iter().enumerate() {
        index.insert(*addr, i);
    }
    let invariants: Vec<crate::transducer::stalk::Invariant> = pending
        .iter()
        .zip(&prior_at)
        .map(|((p, _), j)| match j.and_then(|j| prior.invariants.get(j)) {
            Some(inv) => inv.clone(),
            None => crate::transducer::stalk::invariant(p.sub()),
        })
        .collect();
    let closure: Vec<Option<usize>> = pending
        .iter()
        .enumerate()
        .map(|(i, (p, _))| {
            let held = prior_at[i].and_then(|j| {
                let same_basis = prior.basis_gen == basis_gen;
                let untouched = incremental && !moved.contains(&invariants[i]);
                (same_basis || untouched).then(|| prior.closure[j])
            });
            held.unwrap_or_else(|| {
                crate::transducer::stalk::congruent_shaped(stalks, |i| language.shape(i), p.sub()).map(|(i, _)| i)
            })
        })
        .collect();
    let lineage: Vec<(u64, u64)> = pending
        .iter()
        .map(|(p, _)| {
            let (r, t, _) = lineage_of(&p.address(), composites);
            (r, t)
        })
        .collect();
    // κ⃗ lifted by the lineage fraction; the key is the strongest lane, exact.
    let mut keyed: Vec<(Q, [u8; 32], usize)> = pending
        .iter()
        .enumerate()
        .map(|(i, (p, _))| {
            let supp = BigInt::from(p.sub().len().max(1) as u64);
            let (r, t) = lineage[i];
            let lift = Q::one() + Q::new(BigInt::from(r), BigInt::from(t.max(1)));
            let key = p
                .magnitude()
                .iter()
                .map(|&m| Q::new(BigInt::from(m), supp.clone()) * lift.clone())
                .max()
                .unwrap_or_else(Q::zero);
            (key, p.address(), i)
        })
        .collect();
    keyed.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    Rank {
        closure,
        order: keyed.into_iter().map(|(_, _, i)| i).collect(),
        lineage,
        addresses,
        invariants,
        index,
        basis_gen,
        basis_len: language.words.len(),
        retire_gen: language.retire_gen,
    }
}

/// A reduction computed off the lock, ready to commit: the germ, its context,
/// the normal form over `language` (the snapshot it reduced over, so cited
/// indices resolve to addresses at commit), and the selection critical pairs.
pub struct Reduction {
    pub germ: Element,
    pub ctx: Option<(Vec<(i64, i64, u8)>, (i64, i64))>,
    pub form: NormalForm,
    pub qs: Vec<(usize, Vec<(i64, i64)>)>,
    pub language: std::sync::Arc<Language>,
    /// The listing this reduction settles: the address of the whole
    /// utterance's coboundary (a percept's pair, a demand's chain), where
    /// the deflation an installed word earns is read.
    pub listing: [u8; 32],
}

/// QUOTE — pure, over a language snapshot and the open book. The yield
/// surface is read from genuine metabolic tension; each un-factored residual
/// is priced against it, μ⃗[l] = magnitude[l] · (1 + surface[l]). The (C, I)
/// pair differentiates the market (never summed): closure work (open demand
/// whose geometry a held word speaks, up to isometry, through the atlas)
/// carries the C premium; coverage work (unknown geometry) the I premium.
/// Top clusters by strongest lane; the escrow pool bounds the scale.
pub fn quote_over(
    rank: &Rank,
    composites: &[Composite],
    pending: &[(Element, u64)],
    surface: &[u64; 4],
    pool: &[u64; 4],
    top: usize,
    budget: usize,
    spoken: &crate::network::nerve::Keyed<Vec<[u8; 32]>>,
    voice: &[u8; 32],
) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
    let c = rank.closure.iter().filter(|k| k.is_some()).count() as u64;
    let i = rank.closure.len() as u64 - c;
    let cell = crate::anastome::dialogue::cell_wire_width();
    // THE CONTACT'S CARRIER: the head of the sealed rank, up to `top` items
    // whose wire form fits the remaining byte budget (address, μ⃗, count,
    // cells). An item too wide for the frame is passed over, not served as
    // a frame the wire refuses. Only the served head is priced.
    let mut priced: Vec<(Element, [u64; 4])> = Vec::with_capacity(top);
    let mut used = 0usize;
    // Listings no voice has spoken on come first in rank order, then the
    // ones other voices are already on: every voice reads an open page
    // before it joins a conversation in progress.
    // Each listing's receipts are read at its own address.
    let open = rank.order.iter().copied().filter(|&i| pending.get(i).is_some_and(|(p, _)| !spoken.contains(&p.address())));
    let joined = rank.order.iter().copied().filter(|&i| pending.get(i).is_some_and(|(p, _)| spoken.contains(&p.address())));
    for idx in open.chain(joined) {
        if priced.len() >= top {
            break;
        }
        let Some((p, _)) = pending.get(idx) else { continue };
        if spoken.get(&p.address()).is_some_and(|voices| voices.contains(voice)) {
            continue; // this voice has spoken on it in this state
        }
        let wire = 68 + p.sub().len() * cell;
        if used + wire > budget {
            continue;
        }
        used += wire;
        let base = rank.closure.get(idx).copied().flatten();
        let premium = if base.is_some() { c } else { i };
        let mut mu = crate::economy::price(&p.magnitude(), surface);
        mu[2] = p.magnitude()[2].saturating_mul(1 + surface[2] + premium);
        // Lineage-aware assembly pricing: a child of a partly resolved
        // parent carries the parent's assembly premium (its ribbon volume,
        // priced at the surface) scaled by the resolved fraction, so the
        // closing pieces of an assembly quote highest without assignment.
        let (resolved, total, ribbon) = lineage_of(&p.address(), composites);
        let assembly = crate::economy::price(&[0, 0, ribbon, 0], surface);
        for l in 0..4 {
            mu[l] = mu[l].saturating_add(assembly[l].saturating_mul(resolved) / total.max(1));
        }
        priced.push((p.clone(), mu));
    }
    // THE SOLVENCY BOUND (P5): tension set the RANKING; the escrow pool sets
    // the SCALE. Each quoted lane is capped at pool[l]/quoted so the market
    // never offers more than it can pay — Σ quotes ≤ pool. An empty pool
    // quotes zero: an honest "cannot pay".
    let quoted = priced.len().max(1) as u64;
    for (_, mu) in &mut priced {
        for l in 0..4 {
            mu[l] = mu[l].min(pool[l] / quoted);
        }
    }
    (*surface, priced)
}

/// THE COHESION WITNESS — pure: the joint (C, I) state over the open book,
/// both axes exact and never traded. C counts open demands sitting on held
/// geometry (closure failing — collapse-side work); I counts open demands on
/// geometry no thought speaks (coverage failing — expansion-side work).
pub fn cohesion_over(closure: &[Option<usize>]) -> crate::cohesion::CohesionWitness {
    let c = closure.iter().filter(|k| k.is_some()).count() as u64;
    let i = closure.len() as u64 - c;
    crate::cohesion::witness(
        crate::core::engine::Q::from(num_bigint::BigInt::from(c)),
        crate::core::engine::Q::from(num_bigint::BigInt::from(i)),
    )
}

/// PERCEPTION TO GERMS — pure. Correspond bodies by rigid-motion invariant
/// (zero search), frame each pair to its canonical origin, and germinate the
/// coboundary between the framed scenes; identical framed scenes are a pure
/// transposition (no substance). A coboundary wider than the atomic frame
/// fragments into overlapping tiles carrying a composite lineage. None when
/// the percept is degenerate (no bodies, or mismatched counts). Identical
/// germs across every body collapse to one.
pub fn germinate_bodies(
    ca: &[Vec<(i64, i64, u8)>],
    cb: &[Vec<(i64, i64, u8)>],
    frame: &crate::arc::Frame,
) -> Option<(Vec<Germ>, Vec<Composite>)> {
    if ca.is_empty() || ca.len() != cb.len() {
        return None;
    }
    let mut ai: Vec<usize> = (0..ca.len()).collect();
    let mut bi: Vec<usize> = (0..cb.len()).collect();
    ai.sort_by(|&x, &y| lift(&ca[x], 0).invariant().cmp(&lift(&ca[y], 0).invariant()));
    bi.sort_by(|&x, &y| lift(&cb[x], 0).invariant().cmp(&lift(&cb[y], 0).invariant()));
    let mut germs: Vec<Germ> = Vec::with_capacity(ca.len());
    let mut composites: Vec<Composite> = Vec::new();
    for k in 0..ca.len() {
        let fa = frame_to_origin(&ca[ai[k]]);
        let fb = frame_to_origin(&cb[bi[k]]);
        match germinate(&fa, &fb) {
            Some(rg) => {
                let (tiles, ribbon) = cover(&rg, carrier_capacity());
                if tiles.len() > 1 {
                    let parent = composite_parent_id(&rg);
                    let mut children: Vec<([u8; 32], bool)> = Vec::new();
                    for tile in &tiles {
                        let elem = element::from_residual_framed(tile, frame);
                        children.push((elem.address(), false));
                        let off_r = tile.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
                        let off_c = tile.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
                        germs.push((elem, Some((fa.clone(), (off_r, off_c)))));
                    }
                    composites.push((parent, children, ribbon));
                    continue;
                }
                let off_r = rg.iter().map(|(r, _, _, _)| *r).min().unwrap_or(0);
                let off_c = rg.iter().map(|(_, c, _, _)| *c).min().unwrap_or(0);
                germs.push((element::from_residual_framed(&rg, frame), Some((fa, (off_r, off_c)))));
            }
            None => germs.push((Element::identity(element::GERM_DIM), None)),
        }
    }
    if germs.iter().all(|(g, _)| *g == germs[0].0) {
        germs.truncate(1);
    }
    Some((germs, composites))
}

/// THE NORMAL FORM of a chain over a language: its deconstruction into the
/// words that compose it (a sentence, when closed), or the residual no word
/// speaks (irreducible — a critical pair). One closed-form pass; no search.
pub fn normal_form(language: &Language, chain: &[(i64, i64, u8, u8)], exclude: Option<usize>) -> NormalForm {
    let sentence = crate::transducer::sentence::deconstruct(
        &language.words,
        &language.stalks,
        &|i| language.shape(i),
        chain,
        exclude,
    );
    // The remainder decides as chains: an empty residual and no fiber to
    // deepen is the closed arm; anything left over is the demand.
    match (sentence.residual.as_slice(), sentence.punctures.as_slice()) {
        ([], []) => NormalForm::Decomposed(sentence.cites.iter().map(|(i, _, _)| *i).collect()),
        _ => NormalForm::Irreducible(element::from_residual(&sentence.open(&language.words, chain))),
    }
}

/// Reduce a germ's substance to normal form over the language snapshot —
/// pure. The anchor context rides along unused: placement is a coordinate
/// of the transport now, not a selection rule.
pub fn reduce_germ(
    language: &Language,
    _budget: (usize, usize),
    g: &Element,
    _ctx: Option<&(Vec<(i64, i64, u8)>, (i64, i64))>,
) -> (NormalForm, Vec<(usize, Vec<(i64, i64)>)>) {
    let sub = g.substance();
    if sub.sub().is_empty() {
        return (NormalForm::Decomposed(Vec::new()), Vec::new()); // a pure transposition
    }
    (normal_form(language, sub.sub(), None), Vec::new())
}

/// Reduce every germ over the snapshot — pure, off the lock.
pub fn reduce_all(
    language: &std::sync::Arc<Language>,
    budget: (usize, usize),
    germs: Vec<Germ>,
) -> Vec<Reduction> {
    germs
        .into_iter()
        .map(|(germ, ctx)| {
            let (form, qs) = reduce_germ(language, budget, &germ, ctx.as_ref());
            let listing = germ.address();
            Reduction { germ, ctx, form, qs, language: std::sync::Arc::clone(language), listing }
        })
        .collect()
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
    /// The stalk space over `basis` (index-aligned): congruence lookup on
    /// the orbit quotient selects the fiber over a support's invariant and
    /// runs the exact orbit test there. Measured from the basis, never
    /// persisted; every basis mutation passes through `basis_push`,
    /// `basis_remove`, `basis_clear` so the alignment holds.
    stalks: crate::transducer::stalk::Stalks,
    /// Each basis word reached by its address (index-aligned with `basis`;
    /// rebuilt on the rare removal).
    basis_index: crate::network::nerve::Keyed<usize>,
    /// THE BOOK SNAPSHOT — the open listings and composites as immutable
    /// shared values sealed at a book generation; a market view shares
    /// them by pointer and the book is copied only when it moved.
    book: (u64, std::sync::Arc<Vec<(Element, u64)>>, std::sync::Arc<Vec<Composite>>),
    /// THE LANGUAGE SNAPSHOT — the basis as an immutable shared value the
    /// reflex path reduces over with no lock held. `basis_gen` counts every
    /// mutation of the basis (cards or fibres); the snapshot is rebuilt only
    /// when its generation lags, so a stable language is shared by pointer.
    basis_gen: u64,
    language_gen: u64,
    language: std::sync::Arc<Language>,
    /// THE MARKET CLOSURE — for every listing, the base point (an index into
    /// the language) its geometry sits on, or none (coverage work). A
    /// measured carrier sealed at (basis generation, book generation); a
    /// quote reads it, and it is recomputed off the lock only when either
    /// generation moved. `book_gen` counts every mutation of the open book.
    book_gen: u64,
    closure: (u64, u64, std::sync::Arc<Rank>),
    admission: Admission,
    /// THE OPEN-DEMAND MARKET: irreducible residuals still awaiting consilience —
    /// the un-factored chains the mesh QUOTES as work (Vec-of-relations, capped;
    /// ephemeral — the market is runtime tension, not persisted language).
    /// THE OPEN-DEMAND MARKET — (listing, the K-step it listed at). Dwell is
    /// measured in ALGEBRAIC time (K-steps, court-replayable), never a wall
    /// clock. Capacity is thermodynamic (dynamic_lane_capacity — live memory
    /// headroom × per-lane metabolic tension); eviction is by ACTION
    /// POTENTIAL, never FIFO. No static cap exists.
    pending: Vec<(Element, u64)>,
    /// THE CHRONOMETRIC RECEIPT CHAIN — the K-step index is the mesh's temporal
    /// authority (algebraic time, no wall clocks); every install seals a
    /// parent-linked receipt (LAW-5 continuity). The Step-4 ledger's spine.
    chain: crate::chronometry::ReceiptChain,
    /// The head hash of the court's chain as last ruled to this hypha —
    /// every settlement receipt here links to it, so the hypha's k-steps
    /// are causally ordered against the mesh's temporal authority.
    court_head: [u8; 32],
    /// Retirements of basis words, counted: while it stands still, every
    /// index into the basis names the same word it did.
    retire_gen: u64,
    /// THE RECEIPTS OF SPEECH: (listing address, voice) for every listing a
    /// voice has probed or attested in its current state. A listing that
    /// closes or descends changes address and leaves this relation; a quote
    /// never serves a voice a listing it has already spoken on.
    spoken: crate::network::nerve::Keyed<Vec<[u8; 32]>>,
    /// THE SETTLEMENTS awaiting the court: (listing, its chain, the voice,
    /// the utterance, magnitude before, magnitude after). The court replays
    /// the utterance and measures the deflation itself — proof of
    /// convergence on the work that actually happens. Bounded by the
    /// reserve ceiling; drained as the court pulls the docket.
    settlements: Vec<crate::anastome::dialogue::Settlement>,
    /// The book generation the pressure was last measured at.
    pressure_gen: u64,
    /// THE DEFLATIONS: per provisional word, the magnitude of the listed
    /// demand its install settled — zero when the germ settled nothing on
    /// the book. The court's proof of convergence rides with the word.
    deflations: crate::network::nerve::Keyed<[u64; 4]>,
    /// The k-step at which each provisional word was installed — the
    /// coordinate a court's "since" is read against.
    provisional_k: crate::network::nerve::Keyed<u64>,
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
    /// SELECTION-FIBER CRITICAL PAIRS (Step 7): Q_anchor residuals — a word's
    /// substance matched a defect but its π refused the support. (word address,
    /// the symmetric difference, the scene it arose in). The court's admission
    /// docket refines these into rebound anchors. Bounded; in-memory work queue.
    anchor_mismatches: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
    /// BIRTH EVIDENCE for the selection fiber: (word address, the footprint it
    /// spoke on, the scene) — every germination and every witnessed placement
    /// deposits one; the court's refinement derives π over the accumulated
    /// family (one evidence never binds). Bounded; in-memory.
    anchor_evidence: Vec<([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
    /// COMPOSITE LINEAGE (7d): oversized coboundaries tiled into sub-demands —
    /// (parent puzzle id, children (germ address, completed), the overlap
    /// strips the reassembly must glue on). Full parent settlement pays the
    /// assembly premium only when every child completes and the strips agree.
    /// Bounded; in-memory.
    composites: Vec<([u8; 32], Vec<([u8; 32], bool)>, Vec<(i64, i64, u8, u8)>)>,
}

impl Kernel {
    pub fn new() -> Self {
        Kernel::default()
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    /// Install a word at the end of the basis with its discoverer; the stalk
    /// space admits its support at the same index.
    fn basis_push(&mut self, word: Element, discoverer: [u8; 32]) {
        self.stalks.admit(&crate::transducer::stalk::invariant(word.sub()), self.basis.len());
        self.basis_index.insert(word.address(), self.basis.len());
        self.basis.push(word);
        self.discoverers.push(discoverer);
        self.basis_gen += 1;
    }

    /// Rebuild the address index from the basis (after a removal shifted
    /// every later index; removals are the rare stroke).
    fn reindex_basis(&mut self) {
        let mut index = crate::network::nerve::Keyed::new();
        for (i, w) in self.basis.iter().enumerate() {
            index.insert(w.address(), i);
        }
        self.basis_index = index;
    }

    /// The word at an address, read by its coordinate.
    pub fn word_at(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.basis_index.get(addr).and_then(|&i| self.basis.get(i))
    }

    /// Remove the word at `i` from the basis, its discoverer, and its stalk.
    fn basis_remove(&mut self, i: usize) -> Element {
        self.discoverers.remove(i);
        self.stalks.remove(i);
        self.basis_gen += 1;
        self.retire_gen += 1;
        let removed = self.basis.remove(i);
        self.reindex_basis();
        removed
    }

    fn basis_clear(&mut self) {
        self.basis.clear();
        self.discoverers.clear();
        self.stalks.clear();
        self.basis_index = crate::network::nerve::Keyed::new();
        self.basis_gen += 1;
    }

    /// A held word changed in place (a section, a connection, an anchor):
    /// the language moved.
    fn touch(&mut self) {
        self.basis_gen += 1;
    }

    /// The language as an immutable shared snapshot (words and their atlas)
    /// for off-lock reduction and congruence, rebuilt only when the basis
    /// generation moved since the last one.
    pub fn language(&mut self) -> std::sync::Arc<Language> {
        if self.language_gen != self.basis_gen || self.language.words.len() != self.basis.len() {
            let shapes = (0..self.basis.len()).map(|_| std::sync::OnceLock::new()).collect();
            self.language = std::sync::Arc::new(Language {
                words: self.basis.clone(),
                stalks: self.stalks.clone(),
                shapes,
                retire_gen: self.retire_gen,
                index: self.basis_index.clone(),
            });
            self.language_gen = self.basis_gen;
        }
        std::sync::Arc::clone(&self.language)
    }

    /// THE MARKET VIEW — under a brief lock: the language snapshot, the open
    /// book, the yield surface, the escrow pool, the clock, and the reserve.
    /// Quoting and the cohesion witness compute over this with no lock held.
    pub fn market_open(&mut self) -> MarketView {
        self.remeasure_pressure();
        let gens = (self.basis_gen, self.book_gen);
        let closure = match (self.closure.0, self.closure.1) == gens {
            true => Some(std::sync::Arc::clone(&self.closure.2)),
            false => None,
        };
        if self.book.0 != self.book_gen || self.book.1.len() != self.pending.len() || self.book.2.len() != self.composites.len() {
            self.book = (
                self.book_gen,
                std::sync::Arc::new(self.pending.clone()),
                std::sync::Arc::new(self.composites.clone()),
            );
        }
        MarketView {
            language: self.language(),
            pending: std::sync::Arc::clone(&self.book.1),
            composites: std::sync::Arc::clone(&self.book.2),
            surface: crate::economy::yield_surface(&self.physics),
            pool: self.treasury.pool(),
            k_step: self.chain.next_step_index(),
            reserve: self.physics.reserve,
            gens,
            closure,
            prior: std::sync::Arc::clone(&self.closure.2),
        }
    }

    /// Seal a rank measured off the lock, if the generations it was
    /// measured at still hold. Returns the rank now in force.
    pub fn seal_closure(&mut self, gens: (u64, u64), rank: Rank) -> std::sync::Arc<Rank> {
        // The measurement is served and kept with the generations it was
        // measured at: the market view reads it as current only when those
        // generations still hold, and as the prior the next measurement
        // continues from otherwise — so a book that never stands still is
        // still measured incrementally, never from nothing.
        let measured = std::sync::Arc::new(rank);
        self.closure = (gens.0, gens.1, std::sync::Arc::clone(&measured));
        measured
    }

    /// The rank in force, measured now under the lock (tests and local use;
    /// the server measures off the lock through the market view).
    fn closure_now(&mut self) -> std::sync::Arc<Rank> {
        let gens = (self.basis_gen, self.book_gen);
        if (self.closure.0, self.closure.1) != gens {
            let language = self.language();
            let measured = rank_over(&language, &self.pending, &self.composites, &self.closure.2, gens.0);
            self.closure = (gens.0, gens.1, std::sync::Arc::new(measured));
        }
        std::sync::Arc::clone(&self.closure.2)
    }

    /// APPLY an attach ruling: the court connected `sibling` to `base` by
    /// the transport g. A provisional listing of the sibling clears with its
    /// award evaporating (it is not a base word). When both words are held
    /// here and g carries the sibling onto the base, the sibling transports
    /// into the bundle as a section and its card retires — its discoverer
    /// earns the section subsidy when the fibre gained volume. Returns the
    /// cards retired (0 or 1).
    pub fn apply_attach(
        &mut self,
        sibling: &[u8; 32],
        base: &[u8; 32],
        g: &element::Versor,
    ) -> usize {
        if let Some(i) = self.provisional.iter().position(|(a, _)| a == sibling) {
            self.provisional.remove(i);
            let _ = self.treasury.revoke_award(sibling, false);
        }
        let (Some(bi), Some(si)) = (
            self.basis_index.get(base).copied(),
            self.basis_index.get(sibling).copied(),
        ) else {
            return 0;
        };
        let sib = self.basis[si].clone();
        if element::pullback(&element::positions(self.basis[bi].sub()), sib.sub(), g).is_none() {
            return 0;
        }
        let disc = self.discoverers[si];
        let gained = self.basis[bi].attach(&sib, g);
        self.touch();
        self.basis_remove(si);
        if gained > 0 {
            let anchor = self.anchor();
            self.treasury.settle_subsidy(disc, [0, 0, 1, 0], anchor);
        }
        self.physics.syntropy(&[0, 0, 1, 0]);
        1
    }

    /// SETTLE A PROBE: the replayed residual of an open listing against the
    /// speaker's sentence. Closure (empty residual) clears the listing: the
    /// closer earns the listing's quoted tension from the pool, the cited
    /// words' discoverers earn their use, the lineage advances. Graded
    /// descent (every lane ≤, one <) earns the reduction per lane and the
    /// residual replaces the listing under its own frame fiber; the book's
    /// tension falls monotonically. No descent settles nothing. Returns
    /// (code, paid μ⃗, remaining μ⃗).
    pub fn settle_probe(
        &mut self,
        voice: [u8; 32],
        target: &[u8; 32],
        residual: Vec<(i64, i64, u8, u8)>,
        cited: &[[u8; 32]],
    ) -> (u8, [u64; 4], [u64; 4]) {
        use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
        let Some(li) = self.pending.iter().position(|(p, _)| p.address() == *target) else {
            return (PROBE_NOTHING, [0; 4], [0; 4]);
        };
        let listing = self.pending[li].0.clone();
        self.record_spoken(*target, voice);
        let before = listing.magnitude();
        let after = element::measure_lanes(&residual);
        let surface = crate::economy::yield_surface(&self.physics);
        let anchor = self.anchor();
        let lanes = listing.lanes();
        let mag = listing.magnitude();
        if let [] = residual.as_slice() {
            let paid = self.treasury.settle_subsidy(voice, crate::economy::price(&before, &surface), anchor);
            for addr in cited {
                if let Some(i) = self.basis_index.get(addr).copied() {
                    let mag = self.basis[i].magnitude();
                    self.treasury.settle_use(voice, self.discoverers[i], mag, anchor);
                }
            }
            self.pending.remove(li);
            self.spoken.remove(target);
            self.book_gen += 1;
            self.physics.relieve(&mag);
            self.physics.syntropy(&mag);
            self.append_step(*target, voice);
            self.complete_composite_child(target, voice);
            return (PROBE_CLOSED, paid, [0; 4]);
        }
        // Replay and list: a sentence is a factorization, not a descent to
        // reward. What the citations covered leaves the listing; what
        // remains is listed as the next demand whatever its magnitude. Only
        // a sentence that covered nothing is answered with nothing.
        let mut listed = listing.sub().to_vec();
        listed.sort_unstable();
        if residual == listed {
            return (PROBE_NOTHING, [0; 4], before);
        }
        let delta: [u64; 4] = std::array::from_fn(|l| before[l].saturating_sub(after[l]));
        let paid = self.treasury.settle_subsidy(voice, crate::economy::price(&delta, &surface), anchor);
        // The residual inherits the listing's frame fiber (the scale
        // generators of its spatial block), never its hue tally.
        let mut next = element::from_residual(&residual);
        if let (Some(old), Some(new)) =
            (listing.fiber(element::FIBER_SPATIAL), next.fiber(element::FIBER_SPATIAL))
        {
            let mut m = new.clone();
            m[0][0] = old[0][0].clone();
            m[1][1] = old[1][1].clone();
            next.set_fiber(element::FIBER_SPATIAL, m);
        }
        let listed_at = self.pending[li].1;
        self.pending[li] = (next, listed_at);
        self.spoken.remove(target);
        self.book_gen += 1;
        self.physics.syntropy(&mag);
        self.append_step(*target, voice);
        (PROBE_DESCENT, paid, after)
    }

    /// The durable state's measurement: (cards, sections + connection
    /// relations, provisional docket size, K). A change in any coordinate is
    /// a changed durable language — the persistence trigger reads this, never
    /// an event.
    pub fn volume(&self) -> (usize, usize, usize, u64) {
        let fibre = self.basis.iter().map(|w| w.sections().len() + w.transitions().len()).sum();
        (self.basis.len(), fibre, self.provisional.len(), self.chain.next_step_index())
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    #[cfg(test)]
    pub(crate) fn treasury_mut(&mut self) -> &mut crate::economy::Treasury {
        &mut self.treasury
    }

    /// Crate-internal: the deliberative arm (synthesis::contemplate) spends
    /// the reserve through here — thought is metabolically real.
    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    /// THE DOCKET (Step 5): every provisional word, exported for the court's
    /// blind replay + confluence audit — (element, attesting voices, escrowed μ⃗).
    pub fn dockets(&self) -> Vec<([u8; 32], Element, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64)> {
        self.dockets_since(0)
    }

    /// The provisional words installed past a court's "since" k-step.
    pub fn dockets_since(&self, since_k: u64) -> Vec<([u8; 32], Element, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64)> {
        self.provisional
            .iter()
            .filter_map(|(addr, spoken)| {
                let k = self.provisional_k.get(addr).copied().unwrap_or(0);
                if k <= since_k && since_k > 0 {
                    return None;
                }
                let (_, voices, mu, _) = self
                    .treasury
                    .escrowed_awards()
                    .iter()
                    .find(|(a, _, _, _)| a == addr)?;
                Some((*addr, spoken.clone(), voices.clone(), *mu, self.deflation_of(addr), k))
            })
            .collect()
    }

    /// APPLY the court's verdict on a provisional word. The ruling is the
    /// closed 3-point vocabulary from the wire (never a product of booleans —
    /// bool×bool spans four states for a three-point space):
    /// RULING_UPHELD → release the escrowed award (finality books).
    /// RULING_DEMOTED → stays local dialect, the award evaporates.
    /// RULING_SLASHED → false speech: word excised, shares debited to the pool.
    /// Any other byte is refused — an unknown ruling rules nothing.
    pub fn apply_verdict(&mut self, addr: &[u8; 32], ruling: u8) -> bool {
        use crate::anastome::dialogue::{RULING_DEMOTED, RULING_SLASHED, RULING_UPHELD};
        if !matches!(ruling, RULING_UPHELD | RULING_DEMOTED | RULING_SLASHED) {
            return false;
        }
        let Some(i) = self.provisional.iter().position(|(a, _)| a == addr) else {
            return false;
        };
        self.provisional.remove(i);
        if ruling == RULING_UPHELD {
            self.treasury.release_award(addr)
        } else {
            let slash = ruling == RULING_SLASHED;
            let ok = self.treasury.revoke_award(addr, slash);
            if slash {
                // Excise a slashed BASE word. A slashed section puncture has no
                // separate card to excise (growth was +axis) — the economic
                // slash applies; the fiber coordinate stays local dialect.
                if let Some(w) = self.basis_index.get(addr).copied() {
                    self.basis_remove(w);
                }
            }
            ok
        }
    }

    /// THE THERMODYNAMIC CAPACITY VECTOR — no static cap exists. Per lane:
    ///   C_L = C_total · (ΣP + P_L + 1) / (4 · (ΣP + 1))
    /// where C_total = (machine's live MemAvailable / κ) ÷ the MEASURED mean
    /// listing footprint — the machine's real breath divided by real entry
    /// weight, so the book grows on an idle box and contracts under memory
    /// pressure with no invented budget (α_mem is intrinsic: MemAvailable
    /// already IS the headroom). High-tension lanes expand their share;
    /// slack lanes cede it. κ = 16, the engine's fractional-contract family
    /// (cohesion tolerance κ/16, the float law's pool/16, the use tithe /8).
    pub fn dynamic_lane_capacity(&self) -> [usize; 4] {
        let avail_bytes = mem_available_bytes().max(1);
        let budget = avail_bytes / 16;
        // Measured mean listing footprint: 18 bytes per chain cell, floored
        // at one cell so an empty book still prices honestly.
        let cells: usize = self.pending.iter().map(|(p, _)| p.sub().len().max(1)).sum();
        let mean = (cells * 18 / self.pending.len().max(1)).max(18);
        let c_total = (budget as usize / mean).max(4);
        let p = &self.physics.pressure;
        let sum: u64 = p.iter().sum();
        std::array::from_fn(|l| {
            ((c_total as u128 * (sum + p[l] + 1) as u128) / (4 * (sum + 1)) as u128) as usize
        })
    }

    /// Market occupancy under LINEAGE COHESION: a composite's listed children
    /// share the parent's budget envelope — the family counts ONCE, so a
    /// fractured scene cannot flood the book with its own shards.
    fn market_occupancy(&self) -> usize {
        let mut children_listed = 0usize;
        let mut parents_open = 0usize;
        for (_, children, _) in &self.composites {
            let listed = children
                .iter()
                .filter(|(a, done)| !done && self.pending.iter().any(|(p, _)| p.address() == *a))
                .count();
            if listed > 0 {
                parents_open += 1;
                children_listed += listed;
            }
        }
        self.pending.len() - children_listed + parents_open
    }

    /// D12 — charge the congestion door and decide deferral: fee[l] =
    /// reserve_deficit[l]/8 (0 on a calm mesh, up to 4 at an empty lane); the
    /// speaker pays what their balance holds; a zero payment on a FULLY
    /// exhausted lane defers the demand. Returns true when the door defers.
    fn door_defers(&mut self, voice: [u8; 32]) -> bool {
        let deficit = self.physics.reserve_deficit();
        let fee: [u64; 4] = std::array::from_fn(|l| deficit[l] / 8);
        if fee.iter().all(|&f| f == 0) {
            return false; // a calm mesh keeps a free door
        }
        let anchor = self.anchor();
        let paid = self.treasury.settle_door(voice, fee, anchor);
        (0..4).any(|l| {
            deficit[l] >= crate::physics::reserve_max() && fee[l] > 0 && paid[l] == 0
        })
    }

    /// A child sub-demand COMPLETED (7d): mark its lineage; when the whole
    /// family is done, the parent settles — the gluing strips price the
    /// ASSEMBLY PREMIUM (per-lane overlap volume, float-bounded), split across
    /// the completing voice and the children's discoverers: distributed
    /// reassembly pays MORE than the sum of its parts.
    fn complete_composite_child(&mut self, child: &[u8; 32], voice: [u8; 32]) {
        let mut ready: Option<usize> = None;
        for (ci, (_, children, _)) in self.composites.iter_mut().enumerate() {
            if let Some(entry) = children.iter_mut().find(|(a, done)| a == child && !*done) {
                entry.1 = true;
                if children.iter().all(|(_, d)| *d) {
                    ready = Some(ci);
                }
                break;
            }
        }
        let Some(ci) = ready else { return };
        let (parent, children, strip) = self.composites.remove(ci);
        let mut voices: Vec<[u8; 32]> = vec![voice];
        for (a, _) in &children {
            if let Some(i) = self.basis_index.get(a).copied() {
                let d = self.discoverers[i];
                if !voices.contains(&d) {
                    voices.push(d);
                }
            }
        }
        let anchor = self.anchor();
        // μ_assembly: the overlap volume actually glued — measured, never flat.
        let mu = [0, 0, strip.len().max(1) as u64, 0];
        self.treasury.settle_assembly(&voices, mu, anchor);
        self.append_step(parent, voice); // the reassembly is a K-step
    }

    /// The SELECTION-FIBER critical pairs awaiting Knuth–Bendix anchor
    /// refinement — (word address, Q_anchor, scene). Draining is the court's
    /// move (7c); the queue is bounded and in-memory.
    pub fn anchor_mismatch_dockets(&self) -> &[([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)] {
        &self.anchor_mismatches
    }

    /// The selection fiber's birth evidence (word address, footprint, scene) —
    /// what the court's refinement derives π over.
    pub fn anchor_evidence(&self) -> &[([u8; 32], Vec<(i64, i64)>, Vec<(i64, i64, u8)>)] {
        &self.anchor_evidence
    }

    /// The chronometric anchor, public (settlement callers outside the
    /// kernel — the registry's solve settlements — bind to the same chain).
    pub fn anchor_pub(&self) -> crate::economy::Anchor {
        self.anchor()
    }

    /// SETTLE a verified SOLVE (PoUW: the registry confirmed the prediction
    /// against held answers). Pool-funded, float-law bounded like every
    /// award; the anti-replay collapse happened upstream in the registry —
    /// this pays whatever the registry ruled (bounty or floor).
    pub fn treasury_settle_solve(
        &mut self,
        voice: [u8; 32],
        mu: [u64; 4],
        anchor: crate::economy::Anchor,
    ) -> [u64; 4] {
        self.treasury.settle_solve(voice, mu, anchor)
    }

    /// THE REFINEMENT BID (0x09 — compression IS learning): a client claims
    /// basis word W is DERIVABLE — the rest of the basis already speaks it.
    /// Verification is the redundancy probe's own deterministic replay
    /// (decompose W's substance with W excluded); no trust, no vote. On
    /// success: W retires (the vocabulary compresses — ontological
    /// optimization, halting geometric bloat), the bidder earns the
    /// COMPRESSION YIELD Δ‖·‖_DM × tension (float-bounded), and the rewrite
    /// seals a K-step. Provisional words are the court's business — refused.
    pub fn try_refine(
        &mut self,
        addr: &[u8; 32],
        bidder: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
    ) -> Option<[u64; 4]> {
        let i = self.basis_index.get(addr).copied()?;
        if self.provisional.iter().any(|(a, _)| a == addr) {
            return None;
        }
        // Never a BUNDLE — its sections/cycles carry family volume no other
        // word speaks.
        let candidate = self.basis[i].clone();
        if !candidate.sections().is_empty() || !candidate.torsion().is_empty() {
            return None;
        }
        // THE REPLAY rules: the bidder's sentence, spoken by other words,
        // must reproduce the candidate's substance exactly. No search here.
        // Citations resolve to any held index but the candidate's own (a
        // duplicate card at another index is exactly what a bid names).
        let resolved: Vec<Cite> = cites
            .iter()
            .filter_map(|(a, g, asg, _)| {
                self.basis_index.get(a).copied().filter(|j| *j != i).map(|j| (j, g.clone(), *asg as usize))
            })
            .collect();
        let sub = candidate.substance();
        // A bid citing the candidate itself names nothing; the utterance is
        // verified as spoken.
        if resolved.len() != cites.len() {
            return None;
        }
        let residual = crate::transducer::sentence::verify_utterance(cites, sub.sub());
        match (residual.as_slice(), resolved.len()) {
            ([], 1..) => {
                let saved = candidate.magnitude();
                let surface = crate::economy::yield_surface(&self.physics);
                let mu = crate::economy::price(&saved, &surface);
                let anchor = self.anchor();
                let paid = self.treasury.settle_refine(bidder, mu, anchor);
                self.basis_remove(i);
                // Compression is ORDER BUILT — the syntropic ledger credits it.
                self.physics.syntropy(&candidate.magnitude());
                self.append_step(*addr, bidder); // the global rewrite is a K-step
                Some(paid)
            }
            _ => None,
        }
    }

    /// GENOME SYNCHRONIZATION, commit step (operator-ruled: servers hold no
    /// dialects — one universal genome on the server plane, authored only
    /// by the court's finality). The caller (the deliberation worker) has
    /// already verified the blind replay and derivability OFF the lock on a
    /// basis snapshot; this brief step re-checks presence and installs. The
    /// ratifying voice is the discoverer of record — real provenance, no
    /// sentinel identity.
    pub fn install_adopted(&mut self, word: &Element, ratifier: [u8; 32]) -> Adoption {
        // The loop check at the mint: an arriving word's torsion cycles are
        // walked before they enter the genome. Only a witnessed closed loop
        // at the spoken period is believed.
        for (stored, witness) in crate::transducer::toroid::witness_stored(word) {
            match witness {
                crate::transducer::toroid::HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
                crate::transducer::toroid::HolonomicLoopWitness::Closed { .. } => {
                    return Adoption::Refused("torsion period does not re-derive — the winding was misspoken")
                }
                crate::transducer::toroid::HolonomicLoopWitness::Open { .. } => {
                    return Adoption::Refused("torsion cycle does not close — the period was a filing")
                }
            }
        }
        let addr = word.address();
        if let Some(i) = self.basis_index.get(&addr).copied() {
            // The address is held: the arriving placement's fibre merges into
            // the bundle (sections, connection) — the genome converges in
            // volume, not only in vocabulary.
            self.basis[i].merge_fibre(word);
            self.touch();
            return Adoption::Known;
        }
        // A CONNECTION arriving from an entity is listed as itself: it enters
        // the docket with its own magnitude as the deflation it claims, so
        // the court rules it (inversion, remainder, torsion) and the mesh
        // pays it as a word, never as a copied pair. A residual word taught
        // up is installed as before.
        if word.is_connection() {
            let mag = word.magnitude();
            let surface = crate::economy::yield_surface(&self.physics);
            let mu = crate::economy::price(&mag, &surface);
            let anchor = self.anchor();
            self.treasury.escrow_award(addr, vec![ratifier], mu, anchor);
            self.deflations.insert(addr, mag);
            let k_now = self.k_step();
            self.provisional_k.insert(addr, k_now);
            self.provisional.push((addr, word.clone()));
            self.book_gen += 1;
            self.append_step(addr, ratifier);
        }
        self.basis_push(word.clone(), ratifier);
        self.physics.syntropy(&word.magnitude()); // convergence is order built
        Adoption::Installed
    }

    /// A K-STEP on this hypha's chain, linked to the court: the operator
    /// digest folds the acting voice with the court head last ruled here,
    /// so the receipt is causally ordered against the mesh's chain of
    /// record and verifiable end to end.
    fn append_step(&mut self, fingerprint: [u8; 32], voice: [u8; 32]) {
        let mut buf = [0u8; 64];
        buf[..32].copy_from_slice(&voice);
        buf[32..].copy_from_slice(&self.court_head);
        let op = *blake3::hash(&buf).as_bytes();
        self.chain.append(fingerprint, op);
    }

    /// A voice spoke on a listing in its current state.
    fn record_spoken(&mut self, listing: [u8; 32], voice: [u8; 32]) {
        match self.spoken.get_mut(&listing) {
            Some(voices) => {
                if !voices.contains(&voice) {
                    voices.push(voice);
                }
            }
            None => {
                self.spoken.insert(listing, vec![voice]);
            }
        }
    }

    /// The receipts of speech, keyed by listing: read at a listing's own
    /// address, never walked.
    pub fn spoken(&self) -> &crate::network::nerve::Keyed<Vec<[u8; 32]>> {
        &self.spoken
    }

    /// PUNCTURE held words from a verified utterance's carried bodies: the
    /// fiber deepens on what the hypha verified. A deepened basis moves the
    /// basis generation so the language snapshot refreshes.
    pub fn puncture_from(&mut self, cites: &[crate::anastome::dialogue::CiteWire]) -> usize {
        let deepened = crate::transducer::sentence::puncture_from(&mut self.basis, cites);
        if deepened > 0 {
            self.basis_gen += 1;
        }
        deepened
    }

    /// RECORD a settlement for the court, bounded by the reserve ceiling
    /// (the oldest leaves when the docket is full and the court has not
    /// pulled).
    pub fn record_settlement(
        &mut self,
        listing: [u8; 32],
        chain: Vec<(i64, i64, u8, u8)>,
        voice: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
        before: [u64; 4],
        after: [u64; 4],
    ) {
        let cap = crate::physics::reserve_max() as usize;
        if self.settlements.len() >= cap {
            self.settlements.remove(0);
        }
        let k = self.k_step();
        self.settlements.push((listing, chain, voice, cites.to_vec(), before, after, k));
    }

    /// The settlements past a court's "since": the relation is in k-step
    /// order, so the walk is over the delta only, from the newest back.
    pub fn settlements_since(&self, since_k: u64) -> &[crate::anastome::dialogue::Settlement] {
        let mut start = self.settlements.len();
        while start > 0 && self.settlements[start - 1].6 > since_k {
            start -= 1;
        }
        &self.settlements[start..]
    }

    /// The court stands at `since_k`: everything at or before it is heard
    /// and leaves the docket.
    pub fn settlements_heard_to(&mut self, since_k: u64) {
        let mut n = 0usize;
        while n < self.settlements.len() && self.settlements[n].6 <= since_k {
            n += 1;
        }
        self.settlements.drain(..n);
    }

    /// PRESSURE IS THE OPEN BOOK: the per-lane magnitude of every listing,
    /// measured whenever the book moved — never an accumulator. A demand
    /// the language already speaks adds nothing; a settlement removes
    /// exactly what it settled.
    fn remeasure_pressure(&mut self) {
        if self.pressure_gen == self.book_gen && !self.pending.is_empty() {
            return;
        }
        let mut p = [0u64; 4];
        for (listing, _) in &self.pending {
            let m = listing.magnitude();
            for l in 0..4 {
                p[l] = p[l].saturating_add(m[l]);
            }
        }
        self.physics.pressure = p;
        self.pressure_gen = self.book_gen;
    }

    /// The deflation a provisional word's install measured on the book.
    pub fn deflation_of(&self, addr: &[u8; 32]) -> [u64; 4] {
        self.deflations.get(addr).copied().unwrap_or([0; 4])
    }

    /// Measure and record what installing `addr` settles on the book: the
    /// magnitude of the listed demand at that address, or nothing.
    fn record_deflation(&mut self, word: [u8; 32], listing: [u8; 32]) {
        // The demand a percept settles is the listing of its whole chain;
        // the word installed may be the residual after citations, so the
        // deflation is read on the listing, then on the word itself.
        let on = |addr: [u8; 32]| self.pending.iter().find(|(p, _)| p.address() == addr).map(|(p, _)| p.magnitude());
        let deflation = on(listing).or_else(|| on(word)).unwrap_or([0; 4]);
        self.deflations.insert(word, deflation);
    }

    /// The court's chain head as ruled to this hypha.
    pub fn set_court_head(&mut self, head: [u8; 32]) {
        self.court_head = head;
    }

    pub fn court_head(&self) -> [u8; 32] {
        self.court_head
    }

    /// THE CORPUS INTAKE DOOR (server-side ingestion authority): an
    /// un-factored demand from the hypha's OWN registered corpus lists on
    /// the open market under the same thermodynamic discipline as every
    /// spoken demand — no voice spoke it (the corpus is the source), so it
    /// germinates only when client voices independently attest it. The
    /// server decides what is work; the entities do the cognition.
    pub fn intake(&mut self, chain: &[(i64, i64, u8, u8)], frame: &crate::arc::Frame) -> bool {
        if chain.is_empty() {
            return false;
        }
        let residual = element::from_residual_framed(&chain.to_vec(), frame);
        let addr = residual.address();
        if self.basis_index.contains(&addr)
            || self.pending.iter().any(|(p, _)| p.address() == addr)
        {
            return false; // already speakable or already listed
        }
        let lane = (0..4).max_by_key(|&l| residual.magnitude()[l]).unwrap_or(2);
        let cap = self.dynamic_lane_capacity()[lane].max(1);
        if self.market_occupancy() >= cap {
            return false; // the thermodynamic book is full; corpus can wait
        }
        let k_now = self.k_step();
        self.pending.push((residual, k_now));
        self.book_gen += 1;
        true
    }

    /// DEPOSIT evidence from the wire (an entity's verified dialect finding):
    /// local thought seeds the court's global refinement — what tasks
    /// repeatedly confirm locally is what the mesh promotes. Bounded, deduped.
    pub fn deposit_evidence(
        &mut self,
        addr: [u8; 32],
        fp: Vec<(i64, i64)>,
        scene: Vec<(i64, i64, u8)>,
    ) -> bool {
        if fp.is_empty() || self.anchor_evidence.len() >= 128 {
            return false;
        }
        if self.anchor_evidence.iter().any(|(a, f, s)| *a == addr && *f == fp && *s == scene) {
            return false;
        }
        self.anchor_evidence.push((addr, fp, scene));
        true
    }

    /// Open composite lineages (7d): parents whose child sub-demands are still
    /// completing — (parent id, children (address, done), overlap strips).
    pub fn composites(&self) -> &[([u8; 32], Vec<([u8; 32], bool)>, Vec<(i64, i64, u8, u8)>)] {
        &self.composites
    }

    /// REBIND (Step 7c): the court's family-derived π* upgrades γ = (⊤, Δ) →
    /// (π*, Δ) in place — canonical substance identity untouched (the address
    /// is BASE-only), the dual condition activates for this word, and the
    /// word's spent evidence/mismatch queues clear. Returns false for an
    /// unknown address.
    pub fn apply_rebind(&mut self, addr: &[u8; 32], pi: crate::transducer::anchor::Sel) -> bool {
        let mut bound = false;
        if let Some(i) = self.basis_index.get(addr).copied() {
            self.basis[i] = self.basis[i].clone().with_anchor(pi.clone());
            self.touch();
            bound = true;
        }
        // A germ still PENDING quorum accepts the bind too — it installs
        // already anchored (evidence often reaches family size before the
        // second voice completes the install).
        if let Some(i) = self.pending.iter().position(|(t, _)| t.address() == *addr) {
            let listed = self.pending[i].1;
            self.pending[i] = (self.pending[i].0.clone().with_anchor(pi), listed);
            self.book_gen += 1;
            bound = true;
        }
        if bound {
            // Spent critical pairs clear (they argued against the OLD π);
            // EVIDENCE PERSISTS — the family keeps growing so the court can
            // re-refine iteratively: loose early carriers tighten as more
            // witnessed placements accumulate (rebinding is never one-shot).
            self.anchor_mismatches.retain(|(a, _, _)| a != addr);
        }
        bound
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
        let frame = frame_of(a, b);
        self.adjudicate(&components(a), &components(b), voice, &frame)
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
        frame: &crate::arc::Frame,
    ) -> Outcome {
        // The whole path under one lock (tests and local use); the server
        // runs the same three phases with the lock dropped around the
        // reduction (synthesis::adjudicate).
        let Some((language, budget)) = self.reflex_open(voice) else {
            return Outcome::GermPending;
        };
        let Some((germs, composites)) = germinate_bodies(ca, cb, frame) else {
            return self.pivot();
        };
        let reductions = reduce_all(&language, budget, germs);
        self.register_composites(composites);
        self.commit_all(reductions, voice)
    }

    /// THE REFLEX OPENING — under a brief lock: the voice enrols on first
    /// speech, the metabolism dissipates its backlog, the cheap syntropic
    /// moves fire in both resting states of the two-stroke (depleted or
    /// idle; the redundancy probe stays with the idle worker), and the
    /// escrow-priced door prices congestion. Returns the language snapshot
    /// and the projection budget the off-lock reduction runs with; None when
    /// the door defers (the honest "come back", which clients retry).
    pub fn reflex_open(
        &mut self,
        voice: [u8; 32],
    ) -> Option<(std::sync::Arc<Language>, (usize, usize))> {
        self.enroll(voice, crate::economy::Tier::Participant);
        self.physics.relax();
        let idle = self.physics.metabolism_rate().iter().all(|&p| p <= 1);
        if self.physics.depleted() || idle {
            self.condense_stroke();
            self.physics.regenerate();
        }
        if self.door_defers(voice) {
            return None;
        }
        let budget = (0usize, 0usize); // reduction is closed-form: no search budget
        Some((self.language(), budget))
    }

    /// A degenerate percept: nothing to reduce.
    pub fn pivot(&mut self) -> Outcome {
        self.physics.pivot([false; 4]);
        Outcome::Pivot
    }

    /// Register composite lineages from a fragmented percept (bounded).
    pub fn register_composites(&mut self, composites: Vec<Composite>) {
        for c in composites {
            if self.composites.len() < 32 {
                self.composites.push(c);
            }
        }
    }

    /// COMMIT every reduction under the lock: one reduction is the demand's
    /// outcome; several are a fragmented demand recomposed per body.
    pub fn commit_all(&mut self, mut reductions: Vec<Reduction>, voice: [u8; 32]) -> Outcome {
        match reductions.len() {
            1 => self.commit_germ(reductions.swap_remove(0), voice),
            _ => Outcome::Fragmented(
                reductions.into_iter().map(|r| self.commit_germ(r, voice)).collect(),
            ),
        }
    }

    /// COMMIT a reduction: the state changes a normal form implies, under the
    /// lock. Cited words resolve from the snapshot's indices to addresses and
    /// then to the live basis; a citation that left the basis while the
    /// reduction ran defers the demand (it re-reduces on the client's retry).
    /// Opaque: matched by canonical address, never by "kind".
    pub fn commit_germ(&mut self, r: Reduction, voice: [u8; 32]) -> Outcome {
        let Reduction { germ: g, ctx, form, qs, language, listing } = r;
        let gaddr = g.address(); // composite lineage completes by this identity
        let lanes = g.lanes();
        let mag = g.magnitude();
        // Queue the selection critical pairs (bounded, deduped by word).
        if let Some((scene, _)) = &ctx {
            for (wi, q) in qs {
                if self.anchor_mismatches.len() >= 64 {
                    break;
                }
                if let Some(w) = language.words.get(wi) {
                    let addr = w.address();
                    if !self.anchor_mismatches.iter().any(|(a, _, _)| *a == addr) {
                        self.anchor_mismatches.push((addr, q, scene.clone()));
                    }
                }
            }
        }
        let normal_form = match form {
            NormalForm::Decomposed(cited) => {
                let mut word = Vec::with_capacity(cited.len());
                for wi in cited {
                    let live = language
                        .words
                        .get(wi)
                        .map(|w| w.address())
                        .and_then(|a| self.basis_index.get(&a).copied());
                    match live {
                        Some(i) => word.push(i),
                        None => {
                            self.physics.bump(&mag);
                            return Outcome::GermPending; // the language moved under it
                        }
                    }
                }
                NormalForm::Decomposed(word)
            }
            NormalForm::Irreducible(res) => {
                // A germ is born ⊤ — ONE evidence underdetermines its
                // selection (the same law as anchor consensus: the family
                // decides, never a single pair). Its birth footprint+scene are
                // RECORDED as evidence; the court's KB refinement (7c) derives
                // and binds π over the accumulated family.
                if let Some((scene, (or, oc))) = &ctx {
                    let fp: Vec<(i64, i64)> =
                        res.sub().iter().map(|(r, c, _, _)| (r + or, c + oc)).collect();
                    if self.anchor_evidence.len() < 128 {
                        self.anchor_evidence.push((res.address(), fp, scene.clone()));
                    }
                }
                NormalForm::Irreducible(res)
            }
        };
        match normal_form {
            NormalForm::Decomposed(word) => {
                // USE-EVIDENCE (Step 7c): a witnessed placement is a family
                // member — every word this demand decomposed through deposits
                // (address, footprint, scene), so families reach the court's
                // ≥2 threshold through ordinary speech, not only births.
                if let Some((scene, (or, oc))) = &ctx {
                    let chain = g.substance().sub().to_vec();
                    for &i in &word {
                        if self.anchor_evidence.len() >= 128 {
                            break;
                        }
                        let fp = word_footprint_in(&self.basis[i], &chain, (*or, *oc));
                        if !fp.is_empty() {
                            let addr = self.basis[i].address();
                            if !self
                                .anchor_evidence
                                .iter()
                                .any(|(a, f, s)| *a == addr && *f == fp && s == scene)
                            {
                                self.anchor_evidence.push((addr, fp, scene.clone()));
                            }
                        }
                    }
                }
                let anchor = self.anchor();
                self.treasury.settle_recall(voice, lanes, anchor); // reader's fee → the pool recharges
                for &i in &word {
                    // PoUW circulation: the demander pays each cited word's
                    // discoverer the structure exercised, from earned balance.
                    let mag = self.basis[i].magnitude();
                    self.treasury.settle_use(voice, self.discoverers[i], mag, anchor);
                }
                self.physics.syntropy(&mag); // a spanned sentence builds order
                self.complete_composite_child(&gaddr, voice); // lineage: a solved tile
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
                    self.record_spoken(saddr, voice);
                    let quorum = self.admission.voices(&saddr) >= crate::network::consensus::QUORUM;
                    if quorum && self.physics.can_expand(lanes) {
                        self.basis[ti].puncture(asg);
                        self.touch();
                        let mu = crate::economy::price(
                            &residual.magnitude(),
                            &crate::economy::yield_surface(&self.physics),
                        );
                        let anchor = self.anchor();
                        // PROVISIONAL (Step 5): the award escrows until the
                        // court's finality verdict — unfinalized speech earns
                        // nothing; the puncture itself is local dialect.
                        self.treasury.escrow_award(saddr, self.admission.voices_of(&saddr), mu, anchor);
                        self.record_deflation(saddr, listing);
                        let k_now = self.k_step();
                        self.provisional_k.insert(saddr, k_now);
                        self.provisional.push((saddr, residual.clone()));
                        self.physics.relieve(&mag);
                        self.physics.syntropy(&mag); // volume built through the axis
                        // A puncture deepens a held word: order built, no
                        // reserve spent — the fiber is the cheaper direction.
                        self.pending.retain(|(p, _)| p.address() != residual.address());
                        self.book_gen += 1;
                        self.append_step(saddr, voice); // the puncture is a K-step too
                        self.complete_composite_child(&gaddr, voice); // lineage
                        return Outcome::Installed(ti); // installed AS A SECTION of thought ti
                    }
                    self.physics.bump(&mag);
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
                    self.record_deflation(addr, listing);
                    let k_now = self.k_step();
                    self.provisional_k.insert(addr, k_now);
                    self.provisional.push((addr, residual.clone()));
                    self.physics.relieve(&mag);
                    self.physics.syntropy(&mag); // a completed rule is order built
                    self.physics.spend(lanes); // the expansion SHRINKS the reserve
                    self.pending.retain(|(p, _)| p.address() != addr); // the market CLEARS
                    self.book_gen += 1;
                    // Seal the K-step receipt: the install IS a tick of algebraic
                    // time, parent-linked to the mesh's whole history.
                    self.append_step(addr, voice);
                    self.basis_push(residual, voice);
                    self.complete_composite_child(&gaddr, voice); // lineage
                    Outcome::Installed(self.basis.len() - 1)
                } else {
                    // THE TORSION SOURCE: the open demand's pressure is the
                    // torsion the word actually carries. Every stored cycle
                    // is walked from the base assignment: an Open walk is
                    // the anholonomity — its residual span charges the
                    // lanes; a Closed loop is proven monodromy and charges
                    // nothing. A word with no cycle falls back to the
                    // δ-field's kinematic residue. Bounded per demand;
                    // per-lane as ever, never a summed winding.
                    let witnesses = crate::transducer::toroid::witness_thought(&residual);
                    let k = match witnesses.is_empty() {
                        true => {
                            let residue = crate::torsion::extract_torsion_residue(
                                &element::germ_delta(&residual.sub().to_vec()),
                                8,
                                8,
                                None,
                            );
                            crate::torsion::required_k_steps(&residue)
                        }
                        false => witnesses
                            .iter()
                            .map(|w| match w {
                                crate::transducer::toroid::HolonomicLoopWitness::Open { walked } => {
                                    // A failed loop is the torsion charge the economy
                                    // already names: it pivots the lanes the demand
                                    // occupies, and the listing below carries it to
                                    // the market as the demand for a new fiber.
                                    self.physics.pivot(crate::physics::Physics::lanes_of(&mag));
                                    (*walked as u64).max(1)
                                }
                                crate::transducer::toroid::HolonomicLoopWitness::Closed { .. } => 0,
                            })
                            .sum::<u64>(),
                    }
                    .min(8);
                    for _ in 0..k.max(1) {
                        self.physics.bump(&mag);
                    }
                    // LIST the un-factored residual on the open-demand market.
                    // Capacity is THERMODYNAMIC (live memory headroom × lane
                    // tension, recomputed every listing) and eviction is by
                    // ACTION POTENTIAL — the market sheds its lowest-tension
                    // stale listing, never its oldest (FIFO is banned). A new
                    // demand weaker than everything already listed is not
                    // listed at all: the book holds only what the machine can
                    // honestly carry, ordered by what the network most needs.
                    if !self.pending.iter().any(|(p, _)| p.address() == addr) {
                        let lane = (0..4)
                            .max_by_key(|&l| residual.magnitude()[l])
                            .unwrap_or(2);
                        let cap = self.dynamic_lane_capacity()[lane].max(1);
                        let k_now = self.k_step();
                        if self.market_occupancy() >= cap {
                            let surface = crate::economy::yield_surface(&self.physics);
                            let new_a = action_potential(
                                &crate::economy::price(&residual.magnitude(), &surface),
                                0,
                            );
                            let weakest = self
                                .pending
                                .iter()
                                .enumerate()
                                .min_by(|(_, (p, lk)), (_, (q, mk))| {
                                    let ap = action_potential(
                                        &crate::economy::price(&p.magnitude(), &surface),
                                        k_now.saturating_sub(*lk),
                                    );
                                    let aq = action_potential(
                                        &crate::economy::price(&q.magnitude(), &surface),
                                        k_now.saturating_sub(*mk),
                                    );
                                    ap.cmp(&aq)
                                })
                                .map(|(i, (p, lk))| {
                                    (
                                        i,
                                        action_potential(
                                            &crate::economy::price(&p.magnitude(), &surface),
                                            k_now.saturating_sub(*lk),
                                        ),
                                    )
                                });
                            match weakest {
                                Some((i, a)) if a < new_a => {
                                    self.pending.remove(i);
                                    self.pending.push((residual, k_now));
                                    self.book_gen += 1;
                                }
                                _ => {} // the newcomer IS the weakest — unlisted
                            }
                        } else {
                            self.pending.push((residual, k_now));
                            self.book_gen += 1;
                        }
                    }
                    Outcome::GermPending
                }
            }
        }
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


    /// The kernel's chain reduction — delegates to the shared projective
    /// decomposition ([`decompose_over`]) at the metabolic projection budget.
    fn reduce_chain(&self, first: Vec<(i64, i64, u8, u8)>, exclude: Option<usize>) -> NormalForm {
        let fresh;
        let language: &Language = match self.language_gen == self.basis_gen && self.language.words.len() == self.basis.len() {
            true => &self.language,
            false => {
                fresh = Language {
                    words: self.basis.clone(),
                    stalks: self.stalks.clone(),
                    shapes: (0..self.basis.len()).map(|_| std::sync::OnceLock::new()).collect(),
                    retire_gen: self.retire_gen,
                    index: self.basis_index.clone(),
                };
                &fresh
            }
        };
        normal_form(language, &first, exclude)
    }

    /// THE SYNTROPIC STROKE — one bounded consolidation step, run when a lane's
    /// reserve is depleted. Probe the cursor generator for REDUNDANCY: if the rest
    /// of the basis already derives it (its substance decomposes with it excluded),
    /// it is a closed critical pair wearing a generator's name — RETIRE it, and the
    /// basis compresses toward a minimal confluent generating set. Either way the
    /// reserve REGENERATES: optimization is the recovery stroke, so entropic
    /// expansion resumes only at the pace the syntropic counterpart earns.
    /// One bounded consolidation stroke, public for the server's idle
    /// deliberation worker: condense, torse, probe redundancy, regenerate.
    pub fn consolidate(&mut self) {
        self.optimize();
    }

    /// The whole stroke under one lock (the reflex and test paths).
    pub(crate) fn optimize(&mut self) {
        if let Some(i) = self.condense_stroke() {
            let candidate = self.basis[i].clone();
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(i)) {
                self.retire_redundant(&candidate.address());
            }
        }
        self.physics.regenerate();
    }

    /// The stroke for the idle deliberation worker: the cheap moves (condense,
    /// torse, regenerate) run here under the lock; the redundancy probe — a
    /// full decomposition — is handed out as (address, candidate, the basis
    /// without it) to run on that snapshot with no lock held, exactly as
    /// adoption's derivability does. The reflex path never waits on it.
    pub fn consolidate_stroke(&mut self) {
        // The cheap topological moves only: condense siblings through their
        // transports, extract torsion, regenerate. Redundancy is discovered
        // by the entities (the refinement bid) and replayed here — the
        // hypha runs no decomposition of its own.
        self.condense_stroke();
        self.physics.regenerate();
    }

    /// Retire a plain card the rest of the basis derives (the off-lock probe
    /// found it decomposable). Returns the cards retired (0 or 1): the word
    /// may have gone, or gained a fibre, while the probe ran.
    pub fn retire_redundant(&mut self, addr: &[u8; 32]) -> usize {
        let Some(i) = self.basis_index.get(addr).copied() else { return 0 };
        if !self.basis[i].sections().is_empty() || !self.basis[i].torsion().is_empty() {
            return 0;
        }
        let lanes = self.basis[i].lanes();
        let mag = self.basis[i].magnitude();
        self.basis_remove(i);
        self.physics.syntropy(&mag); // a leaner alphabet is order built
        1
    }

    /// Condense and torse at the cursor; returns the cursor index when the
    /// cursor word is a plain card (a redundancy-probe candidate — never a
    /// bundle: its sections/cycles carry family volume no other word speaks).
    fn condense_stroke(&mut self) -> Option<usize> {
        if !self.basis.is_empty() {
            let i = self.optimize_cursor % self.basis.len();
            self.optimize_cursor = self.optimize_cursor.wrapping_add(1);
            // THE CONDENSE MOVE: connect the bundle — every later sibling on
            // the cursor thought's base point (any placement: translated,
            // rotated, reflected) transports into it as a SECTION along its
            // punctured axis, the transport g joining the connection. The
            // basis CONDENSES: cardinality falls, dimensional volume rises.
            // Candidates come from the atlas (the stratum over the cursor's
            // invariant), walked in descending index so a removal never
            // shifts an index still to visit. Bounded per stroke; the absorbed
            // word's discoverer still earns (its language lives on as a section).
            if !self.basis[i].sub().is_empty() {
                let inv = crate::transducer::stalk::invariant(self.basis[i].sub());
                let mut fiber: Vec<usize> =
                    self.stalks.fiber(&inv).iter().copied().filter(|&j| j > i).collect();
                fiber.sort_unstable_by(|a, b| b.cmp(a));
                // One versor solved per stroke: the stroke's cursor also picks
                // which member of the stratum is tried, so successive strokes
                // walk the whole stratum without ever solving it all at once.
                let pick = match fiber.is_empty() {
                    true => Vec::new(),
                    false => vec![fiber[self.optimize_cursor % fiber.len()]],
                };
                let mut absorbed = 0usize;
                for j in pick {
                    let Some(g) = element::transition(self.basis[j].sub(), self.basis[i].sub())
                    else {
                        continue;
                    };
                    let sib = self.basis[j].clone();
                    let sibling_disc = self.discoverers[j];
                    self.basis_remove(j);
                    if self.basis[i].attach(&sib, &g) > 0 {
                        let anchor = self.anchor();
                        self.treasury.settle_subsidy(sibling_disc, [0, 0, 1, 0], anchor);
                    }
                    self.physics.syntropy(&[0, 0, 1, 0]);
                    absorbed += 1;
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
        let mag = self.basis[i].magnitude();
                self.physics.torse_relief(lanes, spanned as u64);
                // THE TORSE COIN — the strongest yield: structure condensed from
                // disorder, priced at the surface on the residual lane, paid to
                // the bundle's discoverer.
                let surface = crate::economy::yield_surface(&self.physics);
                let mu = [0, 0, (spanned as u64) * (1 + surface[2]), 0];
                let anchor = self.anchor();
                self.treasury.settle_torse(self.discoverers[i], mu, anchor);
            }
            self.touch(); // sections and cycles may have moved at the cursor
            if self.basis[i].sections().is_empty() && self.basis[i].torsion().is_empty() {
                return Some(i);
            }
        }
        None
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
    pub fn quote(&mut self, top: usize) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        let surface = crate::economy::yield_surface(&self.physics);
        let rank = self.closure_now();
        quote_over(&rank, &self.composites, &self.pending, &surface, &self.treasury.pool(), top, usize::MAX, &self.spoken, &[0; 32])
    }

    /// ATTEST — a peer's INDEPENDENT derivation of an open residual arrives as a
    /// spoken `demand` act. The projective reduction runs first (a stale quote
    /// DECOMPOSES instead of double-minting); a genuine irreducible joins the
    /// admission as this voice's attestation — consilience through conversation.
    pub fn attest(&mut self, chain: &[(i64, i64, u8, u8)], voice: [u8; 32]) -> Outcome {
        let Some((language, budget)) = self.reflex_open(voice) else {
            return Outcome::GermPending;
        };
        let germ = (element::from_residual(&chain.to_vec()), None);
        let reductions = reduce_all(&language, budget, vec![germ]);
        self.commit_all(reductions, voice)
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
    pub fn cohesion(&mut self) -> crate::cohesion::CohesionWitness {
        let rank = self.closure_now();
        cohesion_over(&rank.closure)
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
        // THE CHAIN section: the court head and every k-step receipt, so
        // the hypha's clock continues across boots instead of restarting.
        out.extend_from_slice(b"KCH1");
        out.extend_from_slice(&self.court_head);
        let receipts = self.chain.receipts();
        out.extend_from_slice(&(receipts.len() as u32).to_le_bytes());
        for r in receipts {
            out.extend_from_slice(&r.step_index.to_le_bytes());
            out.extend_from_slice(&r.delta_fingerprint);
            out.extend_from_slice(&r.operator_hash);
            out.extend_from_slice(&r.prev_receipt_hash);
            out.extend_from_slice(&r.chain_hash);
        }
        // THE RANK section: each listing's closure and moduli as last
        // measured, so a boot continues the measurement instead of walking
        // the book from nothing.
        let rank = &self.closure.2;
        out.extend_from_slice(b"RNK1");
        out.extend_from_slice(&(rank.basis_len as u64).to_le_bytes());
        out.extend_from_slice(&(rank.addresses.len() as u32).to_le_bytes());
        for (j, addr) in rank.addresses.iter().enumerate() {
            out.extend_from_slice(addr);
            let closed = rank.closure.get(j).copied().flatten().map_or(u64::MAX, |i| i as u64);
            out.extend_from_slice(&closed.to_le_bytes());
            let inv = rank.invariants.get(j).map(|i| i.0.as_slice()).unwrap_or(&[]);
            out.extend_from_slice(&(inv.len() as u32).to_le_bytes());
            for axis in inv {
                out.extend_from_slice(&axis.to_le_bytes());
            }
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
                // The element reads a BOUNDED slice: its trailing relations
                // (anchor, connection) are taken from the bytes that remain.
                let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                let elem = Element::from_bytes(&b[..end], &mut ep)?;
                *pos += elen;
                let who: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                kernel.basis_push(elem, who);
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
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let spoken = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    kernel.provisional.push((addr, spoken));
                }
            }
            // Chain section (absent in earlier snapshots → the clock starts).
            if b.get(*pos..*pos + 4) == Some(b"KCH1") {
                *pos += 4;
                let head: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                *pos += 32;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) / 136 {
                    return None;
                }
                let mut receipts = Vec::with_capacity(n);
                for _ in 0..n {
                    let step = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let mut field = |p: &mut usize| -> Option<[u8; 32]> {
                        let f: [u8; 32] = b.get(*p..*p + 32)?.try_into().ok()?;
                        *p += 32;
                        Some(f)
                    };
                    let (fp, op, prev, hash) = (field(pos)?, field(pos)?, field(pos)?, field(pos)?);
                    receipts.push(crate::chronometry::KStepReceipt {
                        step_index: step,
                        delta_fingerprint: fp,
                        operator_hash: op,
                        prev_receipt_hash: prev,
                        chain_hash: hash,
                    });
                }
                kernel.court_head = head;
                kernel.chain = crate::chronometry::ReceiptChain::from_receipts(receipts)?;
            }
            // Rank section: the prior measurement, continued incrementally
            // (never read as current: its generations are unknowable here).
            if b.get(*pos..*pos + 4) == Some(b"RNK1") {
                *pos += 4;
                let basis_len = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?) as usize;
                *pos += 8;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) / 44 {
                    return None;
                }
                let mut rank = Rank { basis_gen: u64::MAX, basis_len, retire_gen: kernel.retire_gen, ..Rank::default() };
                for _ in 0..n {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let closed = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let m = read_u32(b, pos)? as usize;
                    if m > b.len().saturating_sub(*pos) / 8 {
                        return None;
                    }
                    let mut axes = Vec::with_capacity(m);
                    for _ in 0..m {
                        axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                        *pos += 8;
                    }
                    rank.index.insert(addr, rank.addresses.len());
                    rank.addresses.push(addr);
                    rank.closure.push((closed != u64::MAX).then_some(closed as usize));
                    rank.invariants.push(crate::transducer::stalk::Invariant(axes));
                }
                kernel.closure = (u64::MAX, u64::MAX, std::sync::Arc::new(rank));
            }
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            self.basis_clear();
            return false;
        }
        true
    }
}

// The blind replay moved to `crate::verdict` (#162): verification is the
// court's substrate, not the server kernel's.

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
/// The adoption outcome — a deterministic status, never a boolean gate.
/// Every arm names what actually happened; the caller maps states, it does
/// not branch on success.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Adoption {
    /// The word entered the universal genome here.
    Installed,
    /// Already present — the genome had converged at this address.
    Known,
    /// The current language already derives it — adoption would add
    /// redundancy, not vocabulary.
    Derivable,
    /// The word does not replay from its own evidence — not ratifiable
    /// speech; the reason names the failed axis.
    Refused(&'static str),
}

/// THE CARRIER'S CAPACITY in cells: what one contact's frame carries, read
/// from the wire's own budget and its measured cell width — a physical
/// limit of the transport, never a size chosen for cognition.
pub fn carrier_capacity() -> usize {
    (crate::network::wire::BODY_BUDGET / crate::anastome::dialogue::cell_wire_width()).max(1)
}

/// THE COVER of a coboundary. A chain the carrier holds whole is one piece.
/// Wider than the carrier, it opens along its own structure: first the
/// connected components of its support (disjoint — zero cut defect), then
/// any component still wider than the carrier along its gauge classes (the
/// cells grouped by transition before→after), whose shared 1-cell
/// boundaries are the overlap ribbons the reassembly glues on. No bounding
/// box, no tile count. Returns (pieces, ribbon cells).
pub fn cover(
    rg: &[(i64, i64, u8, u8)],
    capacity: usize,
) -> (Vec<Vec<(i64, i64, u8, u8)>>, Vec<(i64, i64, u8, u8)>) {
    if rg.len() <= capacity {
        return (vec![rg.to_vec()], Vec::new());
    }
    let mut cells: Vec<(i64, i64, u8, u8)> = rg.to_vec();
    cells.sort_unstable();
    cells.dedup();
    // The support's 1-skeleton, read off the support (sentence::incidence);
    // its components are the disjoint pieces.
    let edges = crate::transducer::sentence::incidence(&cells);
    let roots = crate::transducer::sentence::components(cells.len(), &edges);
    let mut components: Vec<(usize, Vec<usize>)> = Vec::new();
    for i in 0..cells.len() {
        match components.iter_mut().find(|(r, _)| *r == roots[i]) {
            Some((_, piece)) => piece.push(i),
            None => components.push((roots[i], vec![i])),
        }
    }
    // A component wider than the carrier opens along its gauge classes.
    let mut pieces: Vec<Vec<usize>> = Vec::new();
    for (_, component) in components {
        if component.len() <= capacity {
            pieces.push(component);
            continue;
        }
        let mut classes: Vec<((u8, u8), Vec<usize>)> = Vec::new();
        for i in component {
            let key = (cells[i].2, cells[i].3);
            match classes.iter_mut().find(|(k, _)| *k == key) {
                Some((_, piece)) => piece.push(i),
                None => classes.push((key, vec![i])),
            }
        }
        classes.sort_by_key(|(k, _)| *k);
        pieces.extend(classes.into_iter().map(|(_, p)| p));
    }
    // The ribbons: every cell sharing an edge of the 1-skeleton with a cell
    // of another piece — read off the incidence, never by probing neighbours.
    let mut owner: Vec<usize> = vec![0; cells.len()];
    for (pi, piece) in pieces.iter().enumerate() {
        for &i in piece {
            owner[i] = pi;
        }
    }
    let mut crosses: Vec<bool> = vec![false; cells.len()];
    for &(i, j) in &edges {
        if owner[i] != owner[j] {
            crosses[i] = true;
            crosses[j] = true;
        }
    }
    let ribbon: Vec<(i64, i64, u8, u8)> = cells.iter().zip(&crosses).filter(|(_, &x)| x).map(|(c, _)| *c).collect();
    let pieces: Vec<Vec<(i64, i64, u8, u8)>> = pieces.into_iter().map(|p| p.into_iter().map(|i| cells[i]).collect()).collect();
    (pieces, ribbon)
}

/// The parent puzzle id — the content address of the WHOLE coboundary
/// (intrinsic authenticity: a digest cannot be forged).
fn composite_parent_id(rg: &[(i64, i64, u8, u8)]) -> [u8; 32] {
    let mut buf = Vec::with_capacity(rg.len() * 18);
    for (r, c, b, a) in rg {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
        buf.push(*b);
        buf.push(*a);
    }
    *blake3::hash(&buf).as_bytes()
}

/// A word's witnessed placements inside a demand chain (union of translated
/// variant embeddings), mapped into scene coordinates by the centering offset.
fn word_footprint_in(
    gen: &Element,
    chain: &[(i64, i64, u8, u8)],
    (or, oc): (i64, i64),
) -> Vec<(i64, i64)> {
    let base = gen.sub();
    if base.is_empty() {
        return Vec::new();
    }
    let pos = element::positions(base);
    let mut fp: Vec<(i64, i64)> = Vec::new();
    for asg in gen.assignments() {
        let pat: Vec<(i64, i64, u8, u8)> =
            pos.iter().zip(&asg).map(|((r, c), (b, a))| (*r, *c, *b, *a)).collect();
        let Some(&(pr, pc, pb, pa)) = pat.first() else { continue };
        for &(cr, cc, cb_, ca_) in chain {
            if (cb_, ca_) != (pb, pa) {
                continue;
            }
            let (dr, dc) = (cr - pr, cc - pc);
            if pat.iter().all(|(r, c, b, a)| chain.contains(&(r + dr, c + dc, *b, *a))) {
                fp.extend(pat.iter().map(|(r, c, _, _)| (r + dr + or, c + dc + oc)));
            }
        }
    }
    fp.sort_unstable();
    fp.dedup();
    fp
}

/// THE ACTION POTENTIAL of a listing — the thermodynamic tension deciding
/// eviction order: 𝒜 = ‖μ⃗_quoted‖₁ · R / (R + Δk_dwell), with dwell in
/// K-STEPS (algebraic time) and R = RESERVE_MAX as λ_decay's scale (the
/// metabolism contract, not a new knob). Exact integer arithmetic; higher =
/// hotter = kept. The market sheds argmin 𝒜 — its coldest stale residual —
/// never its oldest.
fn action_potential(mu_quoted: &[u64; 4], dwell_k: u64) -> u128 {
    let l1: u128 = mu_quoted.iter().map(|&m| m as u128).sum();
    let r = crate::physics::reserve_max() as u128;
    l1 * r / (r + dwell_k as u128)
}

/// The machine's live breath — MemAvailable from /proc/meminfo, in bytes.
/// A platform without the sensor reports half of a modest measured floor
/// rather than pretending to know (the capacity formula degrades gracefully).
fn mem_available_bytes() -> u64 {
    std::fs::read_to_string("/proc/meminfo")
        .ok()
        .and_then(|text| {
            text.lines().find(|l| l.starts_with("MemAvailable:")).and_then(|l| {
                l.split_whitespace().nth(1).and_then(|kb| kb.parse::<u64>().ok()).map(|kb| kb * 1024)
            })
        })
        .unwrap_or(64 * 1024 * 1024)
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
/// `[voice 32][a bodies][b bodies][frame 4×u32]` — the measured frame
/// (h_in, w_in, h_out, w_out) rides with the bodies so the word germinated
/// from a cross-frame pair carries its frame fiber ρ.
pub fn encode_percept(
    a_bodies: &[Vec<(i64, i64, u8)>],
    b_bodies: &[Vec<(i64, i64, u8)>],
    voice: &[u8; 32],
    frame: &crate::arc::Frame,
    proof: &[Vec<crate::anastome::dialogue::CiteWire>],
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(voice);
    push_bodies(&mut out, a_bodies);
    push_bodies(&mut out, b_bodies);
    for d in [frame.0, frame.1, frame.2, frame.3] {
        out.extend_from_slice(&(d as u32).to_le_bytes());
    }
    // THE PROOF: per germ (in germination order), the speaker's citations.
    out.extend_from_slice(&(proof.len() as u32).to_le_bytes());
    for cites in proof {
        crate::anastome::dialogue::push_cites(&mut out, cites);
    }
    out
}

/// A decoded percept: (voice, bodies A, bodies B, frame, proof per germ).
pub type Percept = (
    [u8; 32],
    Vec<Vec<(i64, i64, u8)>>,
    Vec<Vec<(i64, i64, u8)>>,
    crate::arc::Frame,
    Vec<Vec<crate::anastome::dialogue::CiteWire>>,
);

pub fn decode_percept(body: &[u8]) -> Option<Percept> {
    let voice: [u8; 32] = body.get(0..32)?.try_into().ok()?;
    let mut p = 32;
    let a = read_bodies(body, &mut p)?;
    let b = read_bodies(body, &mut p)?;
    let mut d = [0usize; 4];
    for slot in d.iter_mut() {
        *slot = u32::from_le_bytes(body.get(p..p + 4)?.try_into().ok()?) as usize;
        p += 4;
    }
    let mut proof = Vec::new();
    if let Some(n) = body.get(p..p + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap()) as usize) {
        p += 4;
        for _ in 0..n {
            proof.push(crate::anastome::dialogue::read_cites(body, &mut p)?);
        }
    }
    Some((voice, a, b, (d[0], d[1], d[2], d[3]), proof))
}

/// Resolve citations spoken by address into the language snapshot's
/// coordinates — each by its own address, a bounded find; a citation naming
/// nothing held is dropped (it speaks nothing in the replay).
pub fn resolve_cites(
    language: &Language,
    cites: &[crate::anastome::dialogue::CiteWire],
    target: &[(i64, i64, u8, u8)],
) -> Vec<Cite> {
    cites
        .iter()
        .filter_map(|(addr, g, asg, _)| {
            let i = language.at(addr)?;
            // The gauge coordinate is read from the target, never trusted
            // as an index into this copy of the word.
            let gauge = crate::transducer::sentence::gauge_of(&language.words[i], g, target).unwrap_or(*asg as usize);
            Some((i, g.clone(), gauge))
        })
        .collect()
}

/// THE PROVEN REDUCTION of one germ: the replay of the speaker's citations
/// over the language snapshot decides the normal form — closure cites the
/// sentence, a residual is the speaker's irreducible claim, carried as the
/// germ to complete. No search runs here.
pub fn reduce_proven(
    language: &std::sync::Arc<Language>,
    germ: Germ,
    cites: &[crate::anastome::dialogue::CiteWire],
    frame: &crate::arc::Frame,
    listing: [u8; 32],
) -> Reduction {
    let (g, ctx) = germ;
    let sub = g.substance();
    // The utterance is verified as spoken: the operators it carries are
    // replayed against the germ; the lexicon is consulted only to name
    // what was verified. A closing operator the language does not hold is
    // the missing generator, listed as the demand.
    let residual = crate::transducer::sentence::verify_utterance(cites, sub.sub());
    let resolved = resolve_cites(language, cites, sub.sub());
    let form = match (residual.as_slice(), resolved.len() == cites.len()) {
        ([], true) => NormalForm::Decomposed(resolved.iter().map(|(i, _, _)| *i).collect()),
        ([], false) => {
            let named: Vec<[u8; 32]> = resolved.iter().map(|(i, _, _)| language.words[*i].address()).collect();
            let missing = cites.iter().find(|(a, _, _, _)| !named.contains(a)).map(|(_, _, _, body)| body.clone()).unwrap_or_default();
            NormalForm::Irreducible(element::from_residual_framed(&missing, frame))
        }
        ([_, ..], _) => NormalForm::Irreducible(element::from_residual_framed(&residual, frame)),
    };
    Reduction { germ: g, ctx, form, qs: Vec::new(), language: std::sync::Arc::clone(language), listing }
}

/// The frame measured from two cell scenes (their 0-cell extents) — the
/// identity frame when both share dims; used where no grid is at hand.
pub fn frame_of(a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) -> crate::arc::Frame {
    let extent = |s: &[(i64, i64, u8)]| {
        (
            s.iter().map(|(r, _, _)| *r + 1).max().unwrap_or(0).max(0) as usize,
            s.iter().map(|(_, c, _)| *c + 1).max().unwrap_or(0).max(0) as usize,
        )
    };
    let (hi, wi) = extent(a);
    let (ho, wo) = extent(b);
    (hi, wi, ho, wo)
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
        k.basis_push(from_residual(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
        assert_eq!(k.basis_len(), 2);
        k.optimize(); // probe basis[0]: the rest (its duplicate) derives it → retired
        assert_eq!(k.basis_len(), 1, "the redundant rule is consolidated away");
        k.optimize(); // the survivor is essential — nothing else derives it
        assert_eq!(k.basis_len(), 1, "an essential generator survives the probe");
    }

    /// THE REFINEMENT BID (0x09): a bidder claims a word the rest of the
    /// vocabulary already speaks; the kernel's deterministic replay confirms,
    /// the word retires (compression), the bidder earns the yield. A bid on
    /// an ESSENTIAL word is refused — nothing else derives it.
    #[test]
    fn a_refinement_bid_retires_a_derivable_word_and_pays_the_bidder() {
        use crate::transducer::element::from_residual;
        use crate::transducer::germinate::germinate;
        let mut k = Kernel::new();
        k.enroll([9; 32], crate::economy::Tier::Validator); // capitalize the pool
        // Two stale duplicates of the same deformation germinated far apart —
        // identical substance after centering (what an import or a
        // pre-consolidation era leaves). Neither is provisional: this is
        // final vocabulary carrying a redundancy.
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        // Two words, one centered chain: the first carries a frame fibre (a
        // distinct address, since a keyed basis holds one word per address)
        // and the second speaks its chain exactly.
        k.basis_push(element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(4, 4, 8, 8)), [1; 32]);
        k.basis_push(from_residual(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
        let derivable = k.basis[0].address(); // its duplicate speaks it
        // The bidder's sentence: the duplicate, cited at the identity.
        let body = |w: &Element| crate::transducer::sentence::cited_pattern(w, 0).unwrap();
        let sentence = vec![(k.basis[1].address(), element::identity_versor(), 0u32, body(&k.basis[1]))];
        let paid = k.try_refine(&derivable, [5; 32], &sentence);
        assert!(paid.is_some(), "the derivable word is a confirmed compression bid");
        assert_eq!(k.basis_len(), 1, "the vocabulary compressed");
        let essential = k.basis[0].address();
        let self_cite = vec![(essential, element::identity_versor(), 0u32, body(&k.basis[0]))];
        assert!(k.try_refine(&essential, [5; 32], &self_cite).is_none(), "an essential word is refused");
        assert_eq!(k.basis_len(), 1, "refusal mutates nothing");
        assert!(k.try_refine(&[3; 32], [5; 32], &sentence).is_none(), "an unknown address is refused");

        // THE COURT'S BUSINESS STAYS THE COURT'S: a provisional word cannot be
        // bid away, even if derivable.
        k.basis_push(from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
        let addr = k.basis[1].address();
        k.provisional.push((addr, k.basis[1].clone()));
        let sentence = vec![(k.basis[0].address(), element::identity_versor(), 0u32, body(&k.basis[0]))];
        assert!(k.try_refine(&addr, [5; 32], &sentence).is_none(), "provisional words are refused");
        assert_eq!(k.basis_len(), 2);
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
        // The saddle is one body no held word carries whole: it is not
        // peeled by search into two words, it pends as a germ — the
        // composite is the lineage's work on the book, and a second voice
        // decides it.
        assert!(matches!(k.ingest(&a, &b, [3; 32]), Outcome::GermPending), "a fused body pends as a germ");
        assert_eq!(k.basis_len(), 2, "nothing installs on one voice");
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

    /// THE MARKET HAS NO STATIC CAP: capacity is thermodynamic (live machine
    /// headroom × lane tension) and eviction is by ACTION POTENTIAL — the
    /// coldest listing goes, never the oldest. FIFO is banned.
    #[test]
    fn market_capacity_is_thermodynamic_and_eviction_is_by_tension() {
        let mut k = Kernel::new();
        // Capacity: alive on every lane, and a pressured lane EXPANDS its share.
        let calm = k.dynamic_lane_capacity();
        assert!(calm.iter().all(|&c| c > 0), "no lane is ever pinched to zero: {calm:?}");
        for _ in 0..24 {
            k.physics_mut().bump(&[0, 0, 1, 0]);
        }
        let tense = k.dynamic_lane_capacity();
        assert!(
            tense[2] > tense[0],
            "the strained lane out-budgets the slack one: {tense:?}"
        );
        // Action potential: a VALUABLE stale listing outranks a worthless
        // fresh one — eviction is argmin tension, not argmax age (anti-FIFO).
        let rich_old = action_potential(&[0, 0, 900, 0], 64);
        let poor_new = action_potential(&[0, 0, 3, 0], 0);
        assert!(rich_old > poor_new, "value dominates age: {rich_old} vs {poor_new}");
        // And dwell genuinely decays tension at equal value.
        let fresh = action_potential(&[0, 0, 40, 0], 0);
        let stale = action_potential(&[0, 0, 40, 0], 128);
        assert!(fresh > stale, "dwell cools a listing: {fresh} vs {stale}");
    }

    /// D12 — THE ESCROW-PRICED DOOR: a calm mesh keeps a free door; a
    /// saturated mesh charges congestion (fee → the pool) and DEFERS a
    /// speaker who can pay nothing on a fully exhausted lane — spam costs
    /// exactly when strain is real, and paying members pass.
    #[test]
    fn the_door_charges_strain_and_defers_the_broke() {
        let mut k = Kernel::new();
        // Calm: the door is free — a normal percept adjudicates untouched.
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
        let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "calm door admits");
        assert!(
            !k.treasury().entries().iter().any(|e| e.kind == "fee"),
            "no fee on a calm mesh"
        );
        // Exhaust the metabolism entirely (every lane to zero reserve).
        for _ in 0..16 {
            k.physics_mut().spend([true; 4]);
        }
        // A broke speaker (no earned balance) is DEFERRED at the door.
        let a2 = vec![(5, 0, 2), (5, 1, 2)];
        let b2 = vec![(5, 0, 6), (5, 1, 6)];
        let broke = k.ingest(&a2, &b2, [9; 32]);
        assert_eq!(broke, Outcome::GermPending, "saturated door defers the broke: {broke:?}");
        assert!(k.basis().is_empty() || k.quote(8).1.iter().all(|(w, _)| w.sub() != [(0i64, 0i64, 2u8, 6u8)]),
            "the deferred demand was never adjudicated");
        // A FUNDED speaker pays the congestion fee and passes the door.
        k.treasury_mut().settle_subsidy([7; 32], [8, 8, 8, 8], (0, [0; 32]));
        for _ in 0..16 {
            k.physics_mut().spend([true; 4]); // re-exhaust (the deferral relaxed some)
        }
        let _ = k.ingest(&a2, &b2, [7; 32]);
        assert!(
            k.treasury().entries().iter().any(|e| e.kind == "fee" && e.who == [7; 32]),
            "the funded speaker paid the door into the pool"
        );
    }

    /// THE CONVERSATION: a listing is probed with a cited sentence. Graded
    /// descent settles the reduction and the residual replaces the listing
    /// under its frame; closure clears it and advances the lineage; a
    /// sentence that names nothing settles nothing. The replay is linear in
    /// the proof and the hypha runs no search.
    #[test]
    fn a_probe_settles_descent_then_closure_without_search() {
        use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
        let mut k = Kernel::new();
        // A demand listed whole on an empty language (one connected body).
        let c = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 1), (0, 3, 3)];
        let d = vec![(0, 0, 2u8), (0, 1, 2), (0, 2, 1), (0, 3, 4)];
        assert!(matches!(k.ingest(&c, &d, [1; 32]), Outcome::GermPending));
        let target = k.quote(8).1[0].0.address();
        let chain = k.quote(8).1[0].0.sub().to_vec();
        assert_eq!(chain.len(), 3);
        // Then a word that covers it in part installs.
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8), (0, 1, 2)];
        k.ingest(&a, &b, [1; 32]);
        assert!(matches!(k.ingest(&a, &b, [2; 32]), Outcome::Installed(0)));
        let word = k.basis()[0].address();
        // The entity's reduction over the language: the word cited at the
        // origin, the far cell left as the residual.
        let language = k.language();
        let sentence = crate::transducer::sentence::deconstruct(&language.words, &language.stalks, &|i| language.shape(i), &chain, None);
        let (void, cites) = (sentence.residual.clone(), sentence.cites.clone());
        assert_eq!(void, vec![(0, 3, 3, 4)]);
        assert_eq!(cites.len(), 1);
        assert_eq!(replay(k.basis(), &cites, &chain), void, "the replay reproduces the residual");
        let (code, paid, remaining) = k.settle_probe([3; 32], &target, void.clone(), &[word]);
        assert_eq!(code, PROBE_DESCENT);
        assert!(paid.iter().any(|&p| p > 0), "graded descent earns per lane");
        assert_eq!(remaining, crate::transducer::element::measure_lanes(&void));
        let listing = k.quote(8).1[0].0.clone();
        assert_eq!(listing.sub(), &[(0, 0, 3, 4)], "the residual replaced the listing, re-centered");
        // No descent settles nothing; closure clears the book.
        let t2 = listing.address();
        assert_eq!(k.settle_probe([3; 32], &t2, listing.sub().to_vec(), &[]).0, PROBE_NOTHING);
        assert_eq!(k.settle_probe([3; 32], &t2, Vec::new(), &[word]).0, PROBE_CLOSED);
        assert!(k.quote(8).1.is_empty(), "the listing cleared");
    }


    /// THE PROOF: on the wire the judge replays; a proven percept commits its
    /// sentence, an unproven one is deferred, never searched.
    #[test]
    fn the_wire_judge_replays_proofs_and_defers_the_unproven() {
        let k = std::sync::Mutex::new(Kernel::new());
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8), (0, 1, 2)];
        {
            let mut g = k.lock().unwrap();
            g.ingest(&a, &b, [1; 32]);
            g.ingest(&a, &b, [2; 32]);
        }
        let (word, body) = {
            let g = k.lock().unwrap();
            (g.basis()[0].address(), crate::transducer::sentence::cited_pattern(&g.basis()[0], 0).unwrap())
        };
        let (ca, cb) = perceive(&a, &b);
        let frame = frame_of(&a, &b);
        let proof = vec![vec![(word, element::identity_versor(), 0u32, body)]];
        let out = crate::synthesis::adjudicate(&k, &ca, &cb, [3; 32], &frame, &proof);
        assert!(matches!(out, Outcome::Decomposed(ref w) if w == &vec![0]), "{out:?}");
        let out = crate::synthesis::adjudicate(&k, &ca, &cb, [3; 32], &frame, &[]);
        assert!(matches!(out, Outcome::GermPending), "unproven: deferred, {out:?}");
    }

    /// THE COVER: a chain the carrier holds is one piece; wider, it opens
    /// along its components, and a component still wider than the carrier
    /// along its gauge classes, with the shared 1-cell boundaries as the
    /// ribbon. No bounding box, no count.
    #[test]
    fn the_cover_opens_along_components_then_gauge_classes() {
        // Two components: a 1→2 bar of 4 and a far 3→4 bar of 2.
        let mut rg: Vec<(i64, i64, u8, u8)> = (0..4).map(|c| (0, c, 1u8, 2u8)).collect();
        rg.extend((0..2).map(|c| (5, c, 3u8, 4u8)));
        let (whole, ribbon) = cover(&rg, 6);
        assert_eq!(whole.len(), 1, "the carrier holds it whole");
        assert!(ribbon.is_empty());
        let (pieces, ribbon) = cover(&rg, 5);
        assert_eq!(pieces.len(), 2, "wider than the carrier: the components");
        assert!(ribbon.is_empty(), "disjoint components share no boundary");
        // One component of 6 with two gauge classes, carrier of 5: the
        // classes open it, and the ribbon is the class boundary (both sides).
        let mixed: Vec<(i64, i64, u8, u8)> =
            (0..3).map(|c| (0, c, 1u8, 2u8)).chain((3..6).map(|c| (0, c, 1u8, 3u8))).collect();
        let (pieces, ribbon) = cover(&mixed, 5);
        assert_eq!(pieces.len(), 2);
        assert_eq!(ribbon.len(), 2, "the two cells across the class boundary");
        assert!(ribbon.contains(&(0, 2, 1, 2)) && ribbon.contains(&(0, 3, 1, 3)));
    }

    /// 7d — COMPOSITE LINEAGE: children of a registered parent quote and
    /// settle independently; the assembly premium books when the family
    /// completes, and the rank lifts a partly resolved parent's remaining
    /// children by the resolved fraction.
    #[test]
    fn a_lineage_lifts_its_closing_pieces_and_the_assembly_premium_books() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8), (0, 1, 2)];
        let c = vec![(0, 0, 5u8), (0, 1, 5), (0, 2, 5)];
        let d = vec![(0, 0, 7u8), (0, 1, 7), (0, 2, 7)];
        // Two listings (one voice each: pending), registered as one lineage.
        assert!(matches!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending));
        assert!(matches!(k.ingest(&c, &d, [1; 32]), Outcome::GermPending));
        let listed: Vec<[u8; 32]> = k.quote(8).1.iter().map(|(w, _)| w.address()).collect();
        assert_eq!(listed.len(), 2);
        let parent = [9u8; 32];
        k.register_composites(vec![(parent, vec![(listed[0], false), (listed[1], false)], vec![(0, 0, 1, 2)])]);
        // Complete the first child: the second's rank key lifts by 1/2.
        let first = k.quote(8).1[0].0.sub().to_vec();
        k.attest(&first, [2; 32]);
        let rank = k.closure_now();
        let remaining = k.quote(8);
        assert_eq!(remaining.1.len(), 1, "one child remains listed");
        let (resolved, total) = rank.lineage[0];
        assert_eq!((resolved, total), (1, 2), "the lineage fraction is measured");
        // Complete the second child: the family settles and the premium books.
        let second = remaining.1[0].0.sub().to_vec();
        k.attest(&second, [2; 32]);
        assert!(k.composites().is_empty(), "the family completed — lineage settled");
        assert!(
            k.treasury().entries().iter().any(|e| e.kind == "assembly"),
            "the assembly premium booked on reassembly"
        );
    }

    /// STEP 7c END-TO-END: birth deposits evidence; the court's rebind
    /// upgrades (⊤, Δ) → (π*, Δ) in place; the dual condition then bites in
    /// live adjudication — the same demand in an off-region scene stops
    /// decomposing through the word and pends as new coverage instead.
    #[test]
    fn rebind_activates_the_dual_condition_in_adjudication() {
        use crate::transducer::anchor::Sel;
        let mut k = Kernel::new();
        // Germinate a 1→4 recolour word next to hue 3 (two voices).
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 3)];
        let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 3)];
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert!(!k.anchor_evidence().is_empty(), "birth deposited evidence");
        let chain = k.quote(4).1[0].0.sub().to_vec();
        assert!(matches!(k.attest(&chain, [2; 32]), Outcome::Installed(0)));
        assert!(k.basis()[0].anchor().is_none(), "born ⊤ — one evidence never binds");
        // The court's family-derived π*: hue-1 cells touching hue 3.
        let addr = k.basis()[0].address();
        let pi = Sel::Inter(Box::new(Sel::Hue(1)), Box::new(Sel::Star(Box::new(Sel::Hue(3)))));
        assert!(k.apply_rebind(&addr, pi.clone()));
        assert_eq!(k.basis()[0].anchor(), Some(&pi), "γ = (π*, Δ) in place");
        assert!(
            !k.anchor_evidence().is_empty(),
            "evidence PERSISTS through rebind — the family keeps growing for iterative refinement"
        );
        // ON the interface: the anchored word still speaks.
        let a2 = vec![(5, 0, 1), (5, 1, 1), (6, 0, 3), (6, 1, 3)];
        let b2 = vec![(5, 0, 4), (5, 1, 4), (6, 0, 3), (6, 1, 3)];
        assert!(
            matches!(k.ingest(&a2, &b2, [3; 32]), Outcome::Decomposed(_)),
            "π covers the interface placement"
        );
        // Placement is a coordinate of the transport, not a selection rule:
        // the same body elsewhere is the same word carried there.
        let a3 = vec![(0, 0, 1), (0, 1, 1)];
        let b3 = vec![(0, 0, 4), (0, 1, 4)];
        assert!(matches!(k.ingest(&a3, &b3, [3; 32]), Outcome::Decomposed(_)), "the word speaks wherever it is carried");
    }


    /// THE BLIND REPLAY refutes every tamper axis and upholds honest speech —
    /// the contradiction docket's whole force is this determinism.
    #[test]
    fn blind_replay_refutes_tampered_words() {
        use crate::transducer::germinate::germinate;
        use crate::verdict::blind_replay;
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
        let (addr, spoken, voices, mu, _, _) = &dockets[0];
        assert_eq!(voices.len(), 2, "both attesting voices ride the docket");
        assert!(mu[2] > 0, "the escrowed award is real");
        assert_eq!(spoken.address(), *addr, "the docket carries the word as spoken");
        // UPHELD: finality releases the award — the voices are paid at last.
        assert!(k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD));
        assert!(k.treasury().balance_mu(&[1; 32])[2] > 0, "finality pays voice A");
        assert!(k.treasury().balance_mu(&[2; 32])[2] > 0, "finality pays voice B");
        assert!(k.dockets().is_empty(), "the docket cleared");
        assert!(!k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD), "a verdict lands once");
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
        let (addr, _, _, _, _, _) = k.dockets()[0].clone();
        assert!(k.apply_verdict(&addr, crate::anastome::dialogue::RULING_SLASHED));
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
            k.basis_push(from_residual(&germinate(&a, &bx).unwrap()), [9; 32]);
        }
        assert_eq!(k.basis_len(), 3, "the un-condensed family");
        // One versor per stroke: the cursor walks the stratum over strokes.
        for _ in 0..3 {
            k.optimize();
        }
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
