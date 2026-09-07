// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! COGNITION — the kernel loop. A demand (scene A → scene B) enters as raw
//! cells; the percept is a pair form (`pair_form` → `extract_rule_with`) on
//! the court as on the mind. A closed form is the word; a withheld remainder
//! is the residual. A novel word installs ONLY on two-organ CONVERGENCE
//! (`court::convergence`): one vantage dockets the residual, and a witness
//! that re-derives the same address from the same demand installs it.
//! Convergence is not a count of mouths -- the same voice twice converges
//! on nothing. No digest-keyed cognition (Anti-Flattening Law).

use crate::transducer::a21::{put_addr, take_addr, Address};
use crate::transducer::blade::{components};
use crate::transducer::element::{self, Element};

mod book;
mod court;
mod germinate;
mod market;
mod snapshot;
pub use crate::transducer::sentence::{cited_pattern, replay, verify, Cite};

/// The verdict of ingesting one demand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The demand's substance DECOMPOSED into a canonical sentence of installed
    /// generators — the confluent normal form, reached by peeling under ≺_DM (no
    /// lookup, no search). The algebra spans it, nothing minted. An empty word is a
    /// pure transposition (no substance). This one outcome subsumes the old recall
    /// (length 1) and composition (length k): one reduction, not two search kinds.
    Decomposed(Vec<usize>),
    /// A novel generator docketed on one vantage, awaiting a witness organ's
    /// independent re-derivation of the same address (47.3).
    GermPending,
    /// An irreducible residual that CONVERGED and adjoined as a new generator
    /// (Knuth–Bendix completion — the alphabet grows).
    Installed(usize),
    /// A family member punctured as a section of held word `ti`. Not an install,
    /// and it does not mint.
    Sectioned(usize),
    /// The scene had no single motor; it fragmented into per-body sub-demands,
    /// each reduced independently and recomposed (the large-corpus path).
    Fragmented(Vec<Outcome>),
    /// A degenerate demand (empty, or mismatched body counts) — nothing to reduce.
    Pivot,
}

/// The Čech glue of a fetched word on an overlapping chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Glue {
    Admitted,
    Attached,
    Dialect,
    Slashed(&'static str),
}

/// The result of reducing a demand's substance against the generator basis: either
/// a canonical sentence (fully decomposed; indices into the language the
/// reduction ran over) or an irreducible residual (a critical pair to complete
/// by germination).
#[derive(Debug, Clone)]
pub enum NormalForm {
    Decomposed(Vec<usize>),
    Irreducible(Element),
}

impl PartialEq for NormalForm {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NormalForm::Decomposed(a), NormalForm::Decomposed(b)) => a == b,
            (NormalForm::Irreducible(a), NormalForm::Irreducible(b)) => a.same_vertex(b),
            _ => false,
        }
    }
}
impl Eq for NormalForm {}

/// THE LANGUAGE SNAPSHOT: the held words and their atlas, immutable and
/// shared by pointer with every reflex worker. Reduction, congruence, and
/// quoting run over it with no lock held.
#[derive(Debug, Clone, Default)]
pub struct Language {
    /// Shared snapshot: each word is its own Arc, so a snapshot shares
    /// pointers and never copies the basis. The vector and the index are
    /// owned so an admit MODIFIES this snapshot in place through
    /// `Arc::make_mut` rather than building a new one beside it; a copy is
    /// paid once, and only when a reader is actually holding the old one.
    pub words: Vec<std::sync::Arc<Element>>,
    pub stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// Each word's shape, lifted once on first use and shared by every
    /// lookup over this snapshot (a measurement memoized, never persisted).
    pub shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// The retirement generation of the basis this snapshot was taken at.
    pub retire_gen: u64,
    /// Each word reached by its address — a coordinate read, never a scan.
    pub index: crate::network::nerve::Keyed<usize>,
}

impl Language {
    pub fn get(&self, i: usize) -> Option<&Element> {
        self.words.get(i).map(|w| w.as_ref())
    }

    /// The shape of word `i`, lifted on first use.
    pub fn shape(&self, i: usize) -> Option<&crate::transducer::blade::Shape> {
        let cell = self.shapes.get(i)?;
        cell.get_or_init(|| element::shape_of_chain(self.words[i].sub())).as_ref()
    }

    /// The index of the word at an address.
    pub fn at(&self, addr: &[u8]) -> Option<usize> {
        self.index.get(addr).copied()
    }
}

/// The market as seen at one instant (see [`Kernel::market_open`]).
pub struct MarketView {
    pub language: std::sync::Arc<Language>,
    pub pending: std::sync::Arc<crate::network::nerve::Keyed<(Element, u64)>>,
    pub surface: [u64; 4],
    /// The open book's tension per lane — the denominator every price is a
    /// share of.
    pub pressure: [u64; 4],
    pub pool: Option<[u64; 4]>,
    pub k_step: u64,
    /// Receipt-chain head K — the kernel's own receipt index, packed on Quote.
    pub head_k: u64,
    /// Standing extent per listing, in heights.
    pub standing_t: std::sync::Arc<crate::network::nerve::Keyed<u64>>,
    pub reserve: [u64; 4],
    /// The (basis, book) generations this view was taken at.
    pub gens: (u64, u64),
    /// The sealed rank if it is current at `gens`; absent when either
    /// generation moved, so the reader measures it over the snapshot.
    pub closure: Option<std::sync::Arc<Rank>>,
    /// The last sealed rank regardless of currency — the memo a fresh
    /// measurement reuses closures from at the same basis generation.
    pub prior: std::sync::Arc<Rank>,
    /// Listings keyed by chart, so rank_over's moved set is a fiber.
    pub chart_listings: std::sync::Arc<crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>>,
}

/// THE RANK OF THE BOOK — a sealed carrier: for each listing its base point
/// in the language (closure) or none (coverage). There is no scalar order
/// over the book; quotes are incidence on charts (quote_incidence).
/// Identity is the listing address; there is no parallel Vec of positions.
#[derive(Debug, Clone, Default)]
pub struct Rank {
    pub closure: crate::network::nerve::Keyed<Option<usize>>,
    /// Per listing, its moduli — the coordinate that says whether a word
    /// added since could have closed it.
    pub invariants: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant>,
    /// The basis generation the closure was measured at.
    pub basis_gen: u64,
    /// The basis length and retirement generation at measurement: with no
    /// retirement since, every prior closure index still names its word.
    pub basis_len: usize,
    pub retire_gen: u64,
}

/// One chart a listing has been probed through, with its closure, the
/// remainder that still stands on that chart, and per-lane stagnation on
/// that chart. The closure is never a number.
#[derive(Debug, Clone)]
pub struct FieldEntry {
    pub chart: crate::transducer::stalk::Invariant,
    pub state: crate::transducer::stalk::Closure,
    pub remainder: Vec<(i64, i64, u8, u8)>,
    pub stagnant: [u32; 4],
}

/// One listing's coherence field: the charts and the non-Flat count, updated
/// in write_field so a read never scans the entries for closure.
#[derive(Debug, Clone, Default)]
pub struct ListingField {
    pub entries: Vec<FieldEntry>,
    pub nonflat: u32,
}

/// The holonomy of a fiber of transitions: the geometric product around it.
/// `None` when the fiber is not a loop — a singleton has nothing to close.
/// The loop closes iff this is projectively the identity (10.3); the value
/// itself is the obstruction when it is not, and it is what a germ is bred
/// from. Reading a member of the fiber instead made the obstruction depend
/// on which transition the fiber happened to list last, and two nodes
/// holding one loop in two insertion orders bred two different germs from
/// identical mathematics.
fn fiber_holonomy(vs: &[element::Versor]) -> Option<element::Versor> {
    // The fiber holds more than one transition; a singleton is not a loop.
    if vs.len() < 2 {
        return None;
    }
    let mut p = vs[0].clone();
    for v in &vs[1..] {
        p = p.geo(v);
    }
    Some(p)
}


fn rotor_versor(g: &crate::transducer::alphabet::Rotor) -> Option<element::Versor> {
    let word = g.spelled_word()?;
    crate::transducer::element::conformal_of(&word, &crate::transducer::element::identity_versor())
}

fn union_remainders(entries: &[FieldEntry]) -> Vec<(i64, i64, u8, u8)> {
    let mut u = Vec::new();
    for e in entries {
        if !e.state.is_flat() {
            u.extend_from_slice(&e.remainder);
        }
    }
    u.sort_unstable();
    u.dedup();
    u
}

fn open_charts_of(
    addr: &[u8],
    listing: &Element,
    field: &crate::network::nerve::Keyed<ListingField>,
) -> Vec<crate::transducer::stalk::Invariant> {
    match field.get(addr) {
        Some(f) if f.nonflat > 0 => f
            .entries
            .iter()
            .filter(|e| !e.state.is_flat())
            .map(|e| e.chart.clone())
            .collect(),
        Some(f) if !f.entries.is_empty() => Vec::new(),
        _ => vec![crate::transducer::stalk::invariant_of(listing)],
    }
}

/// Measure the rank of the book — pure.
pub fn rank_over(
    language: &Language,
    pending: &crate::network::nerve::Keyed<(Element, u64)>,
    prior: &Rank,
    basis_gen: u64,
    chart_listings: &crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
) -> Rank {
    let stalks = &language.stalks;
    // A listing's base point moves only when a word lands in its own
    // stratum. At the same basis generation every prior closure is reused
    // by the listing's address. When the basis grew with no retirement,
    // the words added since name the strata that moved, and only listings
    // in those strata are located again — a coordinate read, never the
    // whole book.
    let reuse = prior.retire_gen == language.retire_gen && !prior.closure.is_empty();
    let mut moved: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
        crate::network::nerve::Keyed::new();
    if reuse && prior.basis_gen != basis_gen && language.words.len() >= prior.basis_len {
        for w in &language.words[prior.basis_len..] {
            let inv = crate::transducer::stalk::invariant_of(w);
            moved.insert(inv.key(), inv);
        }
    }
    let locate = |p: &Element| {
        crate::transducer::stalk::congruent_shaped(
            stalks,
            |a| language.index.get(a).and_then(|&i| language.shape(i)),
            p.sub(),
        )
        .and_then(|(a, _)| language.index.get(&a).copied())
    };
    let mut closure = crate::network::nerve::Keyed::new();
    let mut invariants = crate::network::nerve::Keyed::new();
    match reuse {
        false => {
            for (addr, (p, _)) in pending.fiber(crate::network::nerve::ROOT) {
                invariants.insert(&addr, crate::transducer::stalk::invariant_of(p));
                closure.insert(&addr, locate(p));
            }
        }
        true => {
            for (addr, (p, _)) in pending.fiber(crate::network::nerve::ROOT) {
                let inv = prior
                    .invariants
                    .get(&addr)
                    .cloned()
                    .unwrap_or_else(|| crate::transducer::stalk::invariant_of(p));
                invariants.insert(&addr, inv);
                match prior.closure.get(&addr) {
                    Some(c) => {
                        closure.insert(&addr, *c);
                    }
                    None => {
                        closure.insert(&addr, locate(p));
                    }
                }
            }
            if !moved.is_empty() {
                for (ck, _) in moved.fiber(crate::network::nerve::ROOT) {
                    for (addr, _) in chart_listings.link(&ck) {
                        if let Some((p, _)) = pending.get(&addr) {
                            closure.insert(&addr, locate(p));
                        }
                    }
                }
            }
        }
    }
    Rank {
        closure,
        invariants,
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
    pub form: NormalForm,
    pub qs: Vec<(usize, Vec<(i64, i64)>)>,
    pub language: std::sync::Arc<Language>,
    /// The listing this reduction settles: the address of the whole
    /// utterance's coboundary (a percept's pair, a demand's chain), where
    /// the deflation an installed word earns is read.
    pub listing: crate::transducer::a21::Address,
    /// Empty-cites irreducible: remainder of a spoken connection or an
    /// unreached-moduli demand. Quoted. A percept-pair tape is not.
    pub frontier: bool,
}

/// QUOTE — pure, over a language snapshot and the open book. The yield
/// surface is read from genuine metabolic tension; each un-factored residual
/// is priced against it, μ⃗[l] = magnitude[l] · (1 + surface[l]). The (C, I)
/// pair differentiates the market (never summed): closure work (open demand
/// whose geometry a held word speaks, up to isometry, through the atlas)
/// carries the C premium; coverage work (unknown geometry) the I premium.
/// Top clusters by strongest lane; the escrow pool bounds the scale.
/// QUOTE as incidence: listings whose open charts meet `voice_charts`,
/// served in atlas order of those charts, within the body budget. No
/// total order over the book. Returns (surface, priced items with open charts).
pub fn quote_incidence(
    pending: &crate::network::nerve::Keyed<(Element, u64)>,
    listing_atlas: &crate::transducer::stalk::Stalks,
    field: &crate::network::nerve::Keyed<ListingField>,
    on_page: &crate::network::nerve::Keyed<()>,
    voice_charts: &[crate::transducer::stalk::Invariant],
    surface: &[u64; 4],
    _pool: &[u64; 4],
    pressure: &[u64; 4],
    budget: usize,
    spoken: &crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    voice: &[u8; 32],
    standing_t: &crate::network::nerve::Keyed<u64>,
) -> (
    [u64; 4],
    Vec<(Element, [u64; 4], Vec<crate::transducer::stalk::Invariant>)>,
) {
    let cell = crate::anastome::dialogue::cell_wire_width();
    let mut priced: Vec<(Element, [u64; 4], Vec<crate::transducer::stalk::Invariant>)> = Vec::new();
    let mut used = 0usize;
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let root_chart = crate::transducer::stalk::Invariant(Vec::new());
    let mut run = |charts: &[crate::transducer::stalk::Invariant]| {
        for chart in charts {
            let chart_key = chart.key();
            let addrs: Vec<Vec<u8>> = if chart.0.is_empty() {
                listing_atlas.fiber_under(chart)
            } else {
                listing_atlas.fiber(chart)
            };
            for addr in addrs {
                if !on_page.contains(&addr) || seen.contains(&addr) {
                    continue;
                }
                if spoken.get(&addr).is_some_and(|voices| voices.contains(voice)) {
                    continue;
                }
                let Some((p, _)) = pending.get(&addr) else { continue };
                let open = open_charts_of(&addr, p, field);
                let mut open_set: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                for c in &open {
                    open_set.insert(c.key(), ());
                }
                if !chart.0.is_empty() && !open_set.contains(&chart_key) {
                    continue;
                }
                let open_len: usize = 1 + open.iter().map(|c| 1 + 8 * c.0.len().min(255)).sum::<usize>();
                let wire = 68 + p.sub().len() * cell + open_len;
                if used + wire > budget {
                    continue;
                }
                used += wire;
                seen.insert(&addr, ());
                let mut mag = p.magnitude();
                if let Some(&t) = standing_t.get(&addr) {
                    mag[2] = t;
                }
                let mu = crate::economy::price(&mag, pressure);
                priced.push((p.clone(), mu, open));
            }
        }
    };
    // An empty voice atlas is incident to the root prefix and receives
    // the book's shallowest strata. A miss on the asked charts falls
    // through to the same root walk (seen skips duplicates).
    if voice_charts.is_empty() {
        run(std::slice::from_ref(&root_chart));
    } else {
        run(voice_charts);
        run(std::slice::from_ref(&root_chart));
    }
    (*surface, priced)
}

/// THE COHESION WITNESS — pure: the joint (C, I) state over the open book,
/// both axes exact and never traded. C counts open demands sitting on held
/// geometry (closure failing — collapse-side work); I counts open demands on
/// geometry no thought speaks (coverage failing — expansion-side work).
pub fn cohesion_over(rank: &Rank) -> crate::cohesion::CohesionWitness {
    let mut c = 0u64;
    let mut i = 0u64;
    for (_, closed) in rank.closure.fiber(crate::network::nerve::ROOT) {
        if closed.is_some() {
            c = c.saturating_add(1);
        } else {
            i = i.saturating_add(1);
        }
    }
    crate::cohesion::witness(
        crate::core::engine::Q::from(num_bigint::BigInt::from(c)),
        crate::core::engine::Q::from(num_bigint::BigInt::from(i)),
    )
}

/// A percept is the pair form: `pair_form` → `extract_rule_with` on the
/// court as on the mind. A family the engine closes is the minted word;
/// a withheld form is the remainder. No pairing by index, no composite tiles.
pub fn percept_elements(
    ca: &[Vec<(i64, i64, u8)>],
    cb: &[Vec<(i64, i64, u8)>],
    frame: &crate::arc::Frame,
    crystal: &dyn crate::transducer::family::Crystal,
) -> Option<Vec<Element>> {
    use crate::transducer::family::{
        cells_to_grid, extract_rule_with, family_ratio, form_descent, mint_sentence, pair_form,
    };
    let a: Vec<(i64, i64, u8)> = ca.iter().flatten().copied().collect();
    let b: Vec<(i64, i64, u8)> = cb.iter().flatten().copied().collect();
    if a.is_empty() && b.is_empty() {
        return None;
    }
    let family = vec![(cells_to_grid(&a), cells_to_grid(&b))];
    let from_remainders = |remainders: Vec<Vec<(i64, i64, u8, u8)>>| {
        let rem: Vec<(i64, i64, u8, u8)> = remainders.into_iter().flatten().collect();
        if rem.is_empty() {
            Element::identity()
        } else {
            element::from_residual_framed(&rem, frame)
        }
    };
    Some(vec![match extract_rule_with(&family, crystal) {
        Some(rule) => match mint_sentence(&rule, &family, crystal) {
            Some(e) => e,
            None => {
                let (_, rem) = form_descent(&rule.form, &family, crystal);
                from_remainders(rem)
            }
        },
        None => {
            let scale = family_ratio(&family);
            let f = pair_form(&family[0].0, &family[0].1, scale);
            let (_, rem) = form_descent(&f, &family, crystal);
            from_remainders(rem)
        }
    }])
}

/// THE NORMAL FORM of a chain over a language: its deconstruction into the
/// words that compose it (a sentence, when closed), or the residual no word
/// speaks (irreducible — a critical pair). One closed-form pass; no search.
pub fn normal_form(language: &Language, chain: &[(i64, i64, u8, u8)], exclude: Option<usize>) -> NormalForm {
    let sentence = crate::transducer::sentence::deconstruct_at(
        &|i| language.get(i),
        &language.stalks,
        &|a| language.index.get(a).copied(),
        &|i| language.shape(i),
        chain,
        exclude,
    );
    match (sentence.residual.as_slice(), sentence.punctures.as_slice()) {
        ([], []) => NormalForm::Decomposed(sentence.cites.iter().map(|(i, _, _)| *i).collect()),
        _ => NormalForm::Irreducible(element::from_residual(&sentence.open_at(&|i| language.get(i), chain))),
    }
}

/// Reduce a percept to normal form over the language snapshot — pure.
/// A closed form is the word (held, or a new residual). An empty residual
/// chain is a pure transposition.
pub fn reduce_germ(
    language: &Language,
    _budget: (usize, usize),
    g: &Element,
) -> (NormalForm, Vec<(usize, Vec<(i64, i64)>)>) {
    if g.is_sentence() {
        if let Some(i) = language.at(&g.address()) {
            return (NormalForm::Decomposed(vec![i]), Vec::new());
        }
        if let Some(f) = g.form() {
            let mut cited = Vec::new();
            let mut all = true;
            for v in &f.vertices {
                if let crate::transducer::sentence::Vertex::Glued { address, .. } = v {
                    match language.at(address) {
                        Some(i) => cited.push(i),
                        None => {
                            all = false;
                            break;
                        }
                    }
                }
            }
            if all && !cited.is_empty() {
                return (NormalForm::Decomposed(cited), Vec::new());
            }
        }
        return (NormalForm::Irreducible(g.clone()), Vec::new());
    }
    let sub = g.substance();
    if sub.sub().is_empty() {
        return (NormalForm::Decomposed(Vec::new()), Vec::new()); // a pure transposition
    }
    (normal_form(language, sub.sub(), None), Vec::new())
}

/// Cited words taught back: anchored ones only. Length of the sentence
/// does not decide.
pub fn taught_anchored_words<'a, I>(words: I) -> Vec<u8>
where
    I: IntoIterator<Item = &'a Element>,
{
    let mut taught = Vec::new();
    for g in words.into_iter().filter(|g| g.anchor().is_some()) {
        taught.extend_from_slice(&g.to_bytes());
    }
    taught
}

/// Reduce every germ over the snapshot — pure, off the lock.
pub fn reduce_all(
    language: &std::sync::Arc<Language>,
    budget: (usize, usize),
    germs: Vec<Element>,
) -> Vec<Reduction> {
    germs
        .into_iter()
        .map(|germ| {
            let (form, qs) = reduce_germ(language, budget, &germ);
            let listing = germ.address();
            Reduction { germ, form, qs, language: std::sync::Arc::clone(language), listing, frontier: false }
        })
        .collect()
}

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    /// The germinated basis, each word reached by its address. In-place
    /// writes go through `word_mut` / `touch_at`. Language snapshots still
    /// carry an insertion-order slice for Decomposed citations.
    basis: crate::network::nerve::Keyed<std::sync::Arc<Element>>,
    /// Empty-sub group words by M1 action key, written on admit.
    by_action: crate::network::nerve::Keyed<crate::transducer::a21::Address>,
    /// Chart+footprint → addresses on that base, written on admit.
    /// same_base reads the canonical member; condense reads the fiber.
    by_footprint: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// The discoverer of each basis germ, keyed by the word's address —
    /// retire must not shift a parallel Vec.
    discoverers: crate::network::nerve::Keyed<[u8; 32]>,
    /// The stalk space over `basis` (index-aligned): congruence lookup on
    /// the orbit quotient selects the fiber over a support's invariant and
    /// runs the exact orbit test there. Measured from the basis, never
    /// persisted; every basis mutation passes through `basis_push`,
    /// `basis_remove`, `basis_clear` so the alignment holds. Arc so a
    /// snapshot admits by pointer and copy-on-write, never a clone of the
    /// basis of elements.
    stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// THE RULED COVER: the atlas over the words the court has finalized
    /// here — admitted when a verdict lands or a court adoption installs,
    /// removed on slash or retirement, measured once at restore. It is the
    /// cover the court reconciles against, so a stratum is discordant only
    /// where this hypha lacks federation words, never where it holds a
    /// provisional or corpus germ the court has not seen.
    ruled: crate::transducer::stalk::Stalks,
    /// The court's voice, heard on its first verdict: the one asker whose
    /// digest request is answered with the ruled cover.
    court_voice: Option<[u8; 32]>,
    /// Held words keyed by (chart, axis). Filled at `ruled.admit`, emptied
    /// at cavity relocation and basis remove.
    axis_index: crate::transducer::sentence::AxisIndex,
    /// Court replay of A20 certificates: citer → (certificate key → (ok, K)).
    replay_memo: crate::network::nerve::Keyed<crate::network::nerve::Keyed<(bool, u64)>>,
    /// Cited form → citers. Slash of a cited form redockets each citer.
    cited_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// THE BOOK SNAPSHOT — the open listings as an immutable shared value
    /// sealed at a book generation; a market view shares them by pointer
    /// and the book is copied only when it moved.
    book: (u64, std::sync::Arc<crate::network::nerve::Keyed<(Element, u64)>>),
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
    /// What each ORGAN re-derived, per object (court::convergence). Two
    /// organs reaching one address is the admission condition; a voice is a
    /// use and a settlement party, never a vantage.
    convergence: crate::court::convergence::Convergence,
    /// THE OPEN-DEMAND MARKET: irreducible residuals still awaiting consilience —
    /// the un-factored chains the mesh QUOTES as work (Vec-of-relations, capped;
    /// ephemeral — the market is runtime tension, not persisted language).
    /// THE OPEN-DEMAND MARKET — (listing, the K-step it listed at). Dwell is
    /// measured in ALGEBRAIC time (K-steps, court-replayable), never a wall
    /// clock. Capacity is thermodynamic (dynamic_lane_capacity — live memory
    /// headroom × per-lane metabolic tension); eviction is by ACTION
    /// POTENTIAL, never FIFO. No static cap exists.
    pending: crate::network::nerve::Keyed<(Element, u64)>,
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
    /// THE RECEIPTS OF SPEECH: listing → voices that have spoken on it.
    spoken: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Transpose: voice → listings that voice holds. Written with `spoken`.
    spoken_by: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Idempotence of an utterance: address → voice → K → the reply last given.
    /// The receipt is (vk, M2 address, K); a second frame with the same
    /// triple is answered from here, never re-docketed or re-charged.
    utterances: crate::network::nerve::Keyed<crate::network::nerve::Keyed<crate::network::nerve::Keyed<Outcome>>>,
    /// THE QUOTE FRONTIER: remainder-of-g and unreached-moduli demands
    /// (empty-cites attest). Percept-pair coboundary tapes stay on the
    /// book for consilience but are not served on a quote page.
    quote_frontier: crate::network::nerve::Keyed<()>,
    /// THE REMAINING TENSION of a listing: derived from the union of
    /// remainders over the field's non-flat charts. Keyed, a vector, never
    /// primary — recomputed when a probe lands.
    tension: crate::network::nerve::Keyed<[u64; 4]>,
    /// THE COHERENCE FIELD: per listing, the charts it has been probed
    /// through and their closure. One entry per chart; a later probe on
    /// the same chart replaces its state. Never summed, never ordered.
    field: crate::network::nerve::Keyed<ListingField>,
    /// Listings still on the quote page (not relocated to the cavity atlas).
    on_page: crate::network::nerve::Keyed<()>,

    /// Voices that have closed a chart of a listing, keyed by chart then voice.
    flat_loops: crate::network::nerve::Keyed<crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, element::Versor)>>,
    /// Atlas of listings and cavities: admitted under the base chart at
    /// list and under every face in the field. Drain never removes from here.
    listing_atlas: crate::transducer::stalk::Stalks,
    /// Relocated cavity elements, keyed, reachable through congruence.
    cavity_book: crate::network::nerve::Keyed<Element>,
    /// Unreachable listings for the court docket: the cavity is the
    /// element itself. Drained as the court pulls.
    cavities: Vec<Element>,
    /// First-vantage words already Offered.
    offered: crate::network::nerve::Keyed<()>,
    /// The court organ's federation (ruled language) and index.
    court_federation: Vec<(Address, Element)>,
    court_fed_index: crate::network::nerve::Keyed<usize>,
    court_upheld: u64,
    court_slashed: u64,
    crt_imported: bool,
    /// Čech complex per chart: vertices are covering hyphas, coefficients
    /// are Rotor. One Keyed entry per cell; never Elements.
    complexes: crate::network::nerve::Keyed<crate::transducer::cech::Complex>,
    /// Unfilled cells, keyed by (chart, cell). Filled within spend_unit.
    unfilled: crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, crate::transducer::a21::Address)>,
    /// Class cells awaiting a prefix split.
    pending_refine: crate::network::nerve::Keyed<(
        crate::transducer::stalk::Invariant,
        crate::transducer::alphabet::Rotor,
        u8,
    )>,
    /// Addresses fetched as Dialect or Slashed — (chart key, stratum root at
    /// fetch). Dropped when confirm_peer_holds sees that root move.
    heard: crate::network::nerve::Keyed<(Vec<u8>, Vec<u8>)>,
    /// Current dialect transition per (chart, hypha pair): one versor, the
    /// TUniversal stamp it stood at, the two hyphae, and the transport
    /// class key. A newer stamp supersedes; the chain keeps the old.
    cocycle_at: crate::network::nerve::Keyed<(element::Versor, u64, [u8; 32], [u8; 32], Address)>,
    /// Chart key → dialect row keys, so a fill or a refinement removes the row.
    cocycle_by_chart: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Every transition that ever stood between two hyphas on a chart.
    dialect_chain: Vec<(Vec<u8>, element::Versor, u64)>,
    /// Field entries of listings that settled Flat or relocated — FLD1 cold.
    field_cold: crate::network::nerve::Keyed<ListingField>,
    /// Settlements the court has read; the SET1 section holds them.
    settlements_cold: Vec<crate::anastome::dialogue::Settlement>,
    /// Test height: when set, height() returns this instead of the chain head.
    /// C6: a TEST knob, `None` on any live fleet. A kernel that can be told
    /// its own height is a kernel whose time is an input (law 9), so the
    /// SETTER is behind `cfg(test)` and nothing in production can reach it.
    height_override: Option<u64>,
    /// Set on home_install, glue Admitted, mint, and slash.
    persist_due: bool,
    /// C6: a TEST knob, `None` on any live fleet. Production reads
    /// `dynamic_lane_capacity`, which is measured; a cap that can be set is
    /// a free constant (rule 5), so the SETTER is behind `cfg(test)`.
    lane_cap: Option<[usize; 4]>,
    /// Height at first vantage of a standing carrier (listing or docket).
    standing_h: crate::network::nerve::Keyed<u64>,
    /// Listing address → corpus task id, so a verified solve pays the same escrow.
    listing_task: crate::network::nerve::Keyed<crate::transducer::a21::Address>,
    /// Reverse: task → listings bound to it.
    task_listings: crate::network::nerve::Keyed<Vec<Address>>,
    /// Train pairs of a bound task. Install mints only what a rule speaks.
    task_family: crate::network::nerve::Keyed<Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>>,
    /// Sentence address → task, from perceive_speech. Replay reads the family.
    speech_task: crate::network::nerve::Keyed<Address>,
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
    /// coordinate a court's "since" is read against (order inside one kernel).
    provisional_k: crate::network::nerve::Keyed<u64>,
    /// Height at first vantage of each provisional — standing extent.
    provisional_h: crate::network::nerve::Keyed<u64>,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    /// Last key the syntropic cursor yielded; the next stroke resumes after it.
    optimize_cursor: Vec<u8>,
    /// Polytonic kinematic pressure + torsion (per lane).
    physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    treasury: crate::economy::Treasury,
    /// This hypha's verifying key.
    identity: [u8; 32],
    /// Covering envelope of the genesis corpus, per lane. None is unmeasured.
    genesis_cover: Option<[u64; 4]>,
    /// Listing address → the voice whose bond the draw occupies.
    drawers: crate::network::nerve::Keyed<[u8; 32]>,
    /// Per-voice occupancy of open draws, keyed by vk.
    drawn: crate::network::nerve::Keyed<[u64; 4]>,
    /// Words installed on two-voice consilience but NOT yet court-final
    /// (Step 5): local dialect awaiting the admission docket, keyed by address.
    provisional: crate::network::nerve::Keyed<Element>,
    /// Listings whose field has a non-Flat chart — the court's docket, written
    /// in write_field, read in key order.
    open: crate::network::nerve::Keyed<()>,
    /// Keyed cursor into `open` for court_stroke.
    open_cursor: Vec<u8>,
    /// This hypha's cover, admitted in write_field; own_cover_charts reads it.
    own_cover: crate::network::nerve::Cover,
    /// Listings keyed by chart, so rank_over's moved set is a fiber.
    chart_listings: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Ruled addresses beside the ruled cover trie.
    ruled_addrs: crate::network::nerve::Keyed<()>,
    /// Ruled cover root, maintained on admit/remove.
    ruled_root: [u8; 32],
    /// Per-lane occupancy of in-flight execution leases, not the book
    /// row count. Cold listings exert zero occupancy. Lane 2 is quote
    /// leases keyed in `t_leases`.
    occupancy: [u64; 4],
    /// Voices that currently hold a T-lease (ACT_QUOTE checkout).
    t_leases: crate::network::nerve::Keyed<()>,
    /// Per-lane magnitude sum of the open book, updated on list/unlist.
    pressure_sum: [u64; 4],
    /// Listings standing on each lane, for eviction candidates.
    lane_listings: [crate::network::nerve::Keyed<()>; 4],
    /// Anchor evidence/mismatch membership keyed by address+footprint.
    evidence_at: crate::network::nerve::Keyed<()>,
    mismatch_at: crate::network::nerve::Keyed<()>,
    /// SELECTION-FIBER CRITICAL PAIRS (Step 7): Q_anchor residuals — a word's
    /// substance matched a defect but its π refused the support. (word address,
    /// the symmetric difference, the scene it arose in). The court's admission
    /// docket refines these into rebound anchors. Bounded; in-memory work queue.
    anchor_mismatches: Vec<(crate::transducer::a21::Address, Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
    /// BIRTH EVIDENCE for the selection fiber: (word address, the footprint it
    /// spoke on, the scene) — every germination and every witnessed placement
    /// deposits one; the court's refinement derives π over the accumulated
    /// family (one evidence never binds). Bounded; in-memory.
    anchor_evidence: Vec<(crate::transducer::a21::Address, Vec<(i64, i64)>, Vec<(i64, i64, u8)>)>,
}

fn chart_simplex_key(inv: &crate::transducer::stalk::Invariant, sim: &[u8]) -> Vec<u8> {
    let mut buf = vec![crate::transducer::a21::TAG_CELL];
    crate::transducer::a21::put_bytes(&mut buf, &inv.key());
    crate::transducer::a21::put_bytes(&mut buf, sim);
    buf
}

/// Canonical encoding of a voice's K: the index itself.
fn utterance_k_key(k: u64) -> Vec<u8> {
    k.to_le_bytes().to_vec()
}

fn loop_key(chart: &crate::transducer::stalk::Invariant, voice: &[u8; 32]) -> Vec<u8> {
    let mut buf = Vec::new();
    crate::transducer::a21::put_bytes(&mut buf, &chart.key());
    buf.extend_from_slice(voice);
    buf
}

/// Canonical key of one evidence row: address, sorted footprint, sorted scene.
fn footprint_key(addr: &[u8], fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<u8> {
    let mut points = fp.to_vec();
    points.sort_unstable();
    let mut cells = scene.to_vec();
    cells.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::a21::put_bytes(&mut buf, addr);
    buf.extend_from_slice(&(points.len() as u32).to_le_bytes());
    for &(r, c) in &points {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    buf.extend_from_slice(&(cells.len() as u32).to_le_bytes());
    for &(r, c, h) in &cells {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
        buf.push(h);
    }
    buf
}

/// M1 action key of an empty-sub group element: canonical versor plus gauge.
fn action_index_key(e: &Element) -> Option<Vec<u8>> {
    if !e.sub().is_empty() {
        return None;
    }
    let mut buf = Vec::new();
    match e.versor() {
        Some(v) => {
            let k = v.key()?;
            crate::transducer::a21::put_bytes(&mut buf, k.as_bytes());
        }
        None => buf.push(0),
    }
    for &(a, b) in e.gauge() {
        buf.push(a);
        buf.push(b);
    }
    Some(buf)
}

/// Positions of a chain — same_base and the condense family are a get.
fn chart_footprint_key(_inv: &crate::transducer::stalk::Invariant, chain: &[(i64, i64, u8, u8)]) -> Vec<u8> {
    let mut pos = element::positions(chain);
    pos.sort_unstable();
    let mut buf = Vec::new();
    buf.extend_from_slice(&(pos.len() as u32).to_le_bytes());
    for (r, c) in pos {
        buf.extend_from_slice(&r.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    buf
}

/// A 1-simplex is exactly two vertices. Any other fiber is not an edge.
fn edge_pair(addrs: &[Address]) -> Option<(Address, Address)> {
    match addrs {
        [a, b] => Some((a.clone(), b.clone())),
        _ => None,
    }
}

impl Kernel {
    pub fn new() -> Self {
        let mut k = Kernel::default();
        k.identity = [0x11; 32];
        if let Some(cover) = Self::measured_test_cover() {
            k.set_genesis_cover(cover);
            k.enroll([0x11; 32], cover);
        }
        k
    }

    /// Bootstrap genesis: the lane-wise measure of a named genesis family,
    /// which a server replaces the moment a corpus registers (the cover is
    /// measured from the registered tasks and re-staked,
    /// `mycelium-server.rs` `set_genesis_cover`). It is a starting corpus,
    /// not a measurement of the live one, and it is a bond a voice may work
    /// within before any corpus arrives. It is measured by `family_cover`,
    /// so it is the lane-wise SUM over these pairs (12.6) and covers each of
    /// them; an envelope over them would not.
    fn measured_test_cover() -> Option<[u64; 4]> {
        crate::arc::family_cover(&[
            (vec![vec![1, 1], vec![1, 0]], vec![vec![0, 1], vec![1, 1]]),
            (vec![vec![1, 1]], vec![vec![2, 2]]),
            (vec![vec![1, 1, 0, 0], vec![0, 0, 0, 0]], vec![vec![0, 0, 1, 1], vec![0, 0, 0, 0]]),
            (vec![vec![1, 1, 1, 3]], vec![vec![2, 2, 1, 4]]),
            (
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
                vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]],
            ),
            (vec![vec![1, 1, 1, 1, 1, 1]], vec![vec![2, 2, 2, 2, 2, 2]]),
        ])
    }

    /// The genesis covering bond, if measured.
    pub fn set_genesis_cover(&mut self, bond: [u64; 4]) {
        self.genesis_cover = Some(bond);
    }

    pub fn genesis_cover(&self) -> Option<[u64; 4]> {
        self.genesis_cover
    }

    fn ensure_bonded(&mut self, voice: [u8; 32]) {
        if self.treasury.bond_of(&voice).is_some() {
            return;
        }
        let Some(cover) = self.genesis_cover else {
            return;
        };
        self.enroll(voice, cover);
    }

    pub fn height(&self) -> u64 {
        self.height_override.unwrap_or_else(|| self.chain.head_height())
    }

    /// C6: test only. Time is not an input to a kernel (law 9).
    #[cfg(test)]
    pub fn set_height(&mut self, h: u64) {
        self.height_override = Some(h);
    }

    /// C6: test only. Capacity is measured, never set (rule 5).
    #[cfg(test)]
    pub fn set_lane_cap(&mut self, cap: [usize; 4]) {
        self.lane_cap = Some(cap);
    }

    fn t_extent(&self, addr: &[u8], now: u64) -> u64 {
        let Some(then) = self
            .provisional_h
            .get(addr)
            .copied()
            .or_else(|| self.standing_h.get(addr).copied())
        else {
            return 0;
        };
        now.saturating_sub(then)
    }

    fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    fn stamp_standing(&mut self, addr: impl AsRef<[u8]>) {
        let addr = addr.as_ref();
        let h = self.height();
        if self.standing_h.get(&addr).is_none() {
            self.standing_h.insert(addr, h);
        }
        if self.provisional.contains(&addr) && self.provisional_h.get(&addr).is_none() {
            self.provisional_h.insert(addr, h);
        }
    }

    fn stratum_root_of(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<u8> {
        let mut rows = self.ruled.fiber(inv);
        rows.sort();
        let mut buf = Vec::new();
        for a in &rows {
            crate::transducer::a21::put_bytes(&mut buf, a);
        }
        buf
    }

    fn heard_mark(&mut self, addr: impl AsRef<[u8]>, inv: &crate::transducer::stalk::Invariant) {
        let root = self.stratum_root_of(inv);
        self.heard.insert(addr, (inv.key().into_vec(), root));
    }

    fn cocycle_key(
        chart: &crate::transducer::stalk::Invariant,
        a: &[u8],
        b: &[u8],
    ) -> Vec<u8> {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let mut buf = vec![crate::transducer::a21::TAG_COCYCLE];
        crate::transducer::a21::put_bytes(&mut buf, &chart.key());
        crate::transducer::a21::put_bytes(&mut buf, lo);
        crate::transducer::a21::put_bytes(&mut buf, hi);
        buf
    }

    /// One versor per (chart, pair). A newer TUniversal stamp supersedes;
    /// the chain keeps every transition that ever stood.
    fn push_cocycle(
        &mut self,
        chart: &crate::transducer::stalk::Invariant,
        a: [u8; 32],
        b: [u8; 32],
        g: element::Versor,
        class_key: Address,
    ) {
        if a == b {
            return;
        }
        let key = Self::cocycle_key(chart, &a, &b);
        let tu = self.height();
        if let Some((old, old_tu, _, _, _)) = self.cocycle_at.get(&key).cloned() {
            if tu < old_tu {
                self.dialect_chain.push((key.clone(), g, tu));
                return;
            }
            if element::same_versor(&old, &g) {
                return;
            }
            self.dialect_chain.push((key.clone(), old, old_tu));
        }
        self.cocycle_at.insert(key.clone(), (g, tu, a, b, class_key));
        let ck = chart.key();
        match self.cocycle_by_chart.get_mut(&ck) {
            Some(m) => {
                m.insert(key, ());
            }
            None => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(key, ());
                self.cocycle_by_chart.insert(ck, m);
            }
        }
    }

    fn rekey_cocycles(
        &mut self,
        old: &crate::transducer::stalk::Invariant,
        new_a: &crate::transducer::stalk::Invariant,
        new_b: &crate::transducer::stalk::Invariant,
    ) {
        let ck = old.key();
        let Some(rows) = self.cocycle_by_chart.remove(&ck) else {
            return;
        };
        let mut saved: Vec<(element::Versor, u64, [u8; 32], [u8; 32], Address)> = Vec::new();
        for (key, _) in rows.fiber(crate::network::nerve::ROOT) {
            if let Some(row) = self.cocycle_at.remove(&key) {
                self.dialect_chain.push((key, row.0.clone(), row.1));
                saved.push(row);
            }
        }
        for (g, _, a, b, ck) in saved {
            self.push_cocycle(new_a, a, b, g.clone(), ck.clone());
            self.push_cocycle(new_b, a, b, g, ck);
        }
    }

    fn chill_field(&mut self, addr: &[u8]) {
        if let Some(listing) = self.field.remove(addr) {
            self.field_cold.insert(addr, listing);
        }
        self.open.remove(addr);
    }

    /// Relocate the largest-T standing carriers on a saturated temporal lane.
    /// T-room is standing extent in heights, never a stored occupancy floor.
    fn relocate_t_exhausted(&mut self) {
        if self.lane_cap.is_none()
            && self.occupancy[2] == 0
            && self.t_leases.fiber(crate::network::nerve::ROOT).next().is_none()
        {
            return;
        }
        let now = self.height();
        let mut tot = 0u64;
        let mut best: Option<(Vec<u8>, u64)> = None;
        let mut consider = |addr: Vec<u8>| {
            let t = self.t_extent(&addr, now);
            tot = tot.saturating_add(t);
            match best {
                Some((_, bt)) if t <= bt => {}
                _ => best = Some((addr, t)),
            }
        };
        for (addr, _) in self.lane_listings[2].fiber(crate::network::nerve::ROOT) {
            consider(addr);
        }
        for (addr, _) in self.provisional.fiber(crate::network::nerve::ROOT) {
            consider(addr);
        }
        let cap = self
            .lane_cap
            .map(|c| c[2] as u64)
            .unwrap_or_else(|| tot.saturating_add(self.lane_room()[2] as u64));
        if tot < cap {
            return;
        }
        let Some((addr, t)) = best else {
            return;
        };
        eprintln!(
            "  cavity relocate {:02x}{:02x}… T={t} (T-room exhausted)",
            addr[0], addr[1]
        );
        self.relocate_standing(&addr);
    }

    fn relocate_standing(&mut self, addr: &[u8]) {
        let _ = self.treasury.revoke_award(addr, false);
        if self.pending.contains(addr) {
            self.relocate_cavity(addr);
        } else if let Some(w) = self.provisional.get(addr).cloned() {
            let inv = crate::transducer::stalk::invariant_of(&w);
            self.listing_atlas.admit(&inv, addr);
            self.cavity_book.insert(&addr, w.clone());
            self.cavities.push(w);
            self.chill_field(addr);
        }
        self.provisional.remove(addr);
        self.provisional_k.remove(addr);
        self.provisional_h.remove(addr);
        self.offered.remove(addr);
        if self.lane_listings[2].contains(addr) {
            self.occupancy[2] = self.occupancy[2].saturating_sub(1);
            self.lane_listings[2].remove(addr);
        }
        self.standing_h.remove(addr);
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    fn discoverer_of(&self, addr: &[u8]) -> Option<[u8; 32]> {
        self.discoverers.get(addr).copied()
    }

    fn word_mut(&mut self, addr: &[u8]) -> Option<&mut Element> {
        self.basis.get_mut(addr).map(std::sync::Arc::make_mut)
    }

    fn book_elem(&self, addr: &[u8]) -> Option<&Element> {
        self.pending.get(addr).map(|(p, _)| p)
    }

    fn live_slot(&self, addr: &[u8]) -> Option<usize> {
        self.language.index.get(addr).copied()
    }

    fn index_word(&mut self, w: &Element) {
        let addr = w.address();
        if let Some(k) = action_index_key(w) {
            self.by_action.insert(k, addr.clone());
        }
        if !w.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(w);
            self.by_footprint.nested_insert(chart_footprint_key(&inv, w.sub()), addr, ());
        }
    }

    fn unindex_word(&mut self, w: &Element) {
        let addr = w.address();
        if let Some(k) = action_index_key(w) {
            if self.by_action.get(&k).is_some_and(|a| a.as_bytes() == addr.as_bytes()) {
                self.by_action.remove(&k);
            }
        }
        if !w.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(w);
            let fk = chart_footprint_key(&inv, w.sub());
            if let Some(inner) = self.by_footprint.get_mut(&fk) {
                inner.remove(addr.as_bytes());
            }
        }
    }

    fn forget_spoken(&mut self, listing: &[u8]) {
        if let Some(voices) = self.spoken.remove(listing) {
            for (v, _) in voices.fiber(crate::network::nerve::ROOT) {
                if let Some(inner) = self.spoken_by.get_mut(&v) {
                    inner.remove(listing);
                }
            }
        }
    }

    fn occupy(&mut self, addr: impl AsRef<[u8]>, mag: &[u64; 4], add: bool) {
        let addr = addr.as_ref();
        let voice = self.drawers.get(addr).copied();
        for l in 0..4 {
            if add {
                self.occupancy[l] = self.occupancy[l].saturating_add(mag[l]);
                if mag[l] > 0 {
                    self.lane_listings[l].insert(addr, ());
                }
            } else {
                self.occupancy[l] = self.occupancy[l].saturating_sub(mag[l]);
                if mag[l] > 0 {
                    self.lane_listings[l].remove(&addr);
                }
            }
            if add {
                self.pressure_sum[l] = self.pressure_sum[l].saturating_add(mag[l]);
            } else {
                self.pressure_sum[l] = self.pressure_sum[l].saturating_sub(mag[l]);
            }
        }
        if let Some(v) = voice {
            let mut d = self.drawn.get(&v).copied().unwrap_or([0; 4]);
            for l in 0..4 {
                if add {
                    d[l] = d[l].saturating_add(mag[l]);
                } else {
                    d[l] = d[l].saturating_sub(mag[l]);
                }
            }
            self.drawn.insert(v, d);
        }
    }

    fn refresh_ruled_root(&mut self) {
        let digests = crate::transducer::sheaf::digests(&self.ruled);
        let mut buf = Vec::new();
        for d in &digests {
            crate::transducer::a21::put_bytes(&mut buf, &d.root);
        }
        self.ruled_root = crate::chronometry::payload_hash(&buf);
    }

    /// Place a word on the basis with no syntropic stroke: restore and the
    /// fallback cursor walk insert this way so a snapshot does not condense
    /// the language it is loading.
    fn basis_insert(&mut self, word: Element, discoverer: [u8; 32]) {
        let addr = word.address();
        self.index_word(&word);
        let arc = std::sync::Arc::new(word);
        self.basis.insert(&addr, std::sync::Arc::clone(&arc));
        self.discoverers.insert(&addr, discoverer);
        self.basis_gen += 1;
        self.language_append_arc(arc);
    }

    /// Install a word and pivot Path 2 on the axis it just moved: condense
    /// a congruent sibling, probe the stratum for a derivable generator,
    /// pay each yield on its own lane. A word is named by its address; the
    /// slot index these returned was a position in the language snapshot,
    /// read back through `live_slot` and defaulted to 0 when the snapshot
    /// lagged the carrier -- slot 0 is a real word, and it is not this one.
    /// No caller read either return (rule 11).
    fn basis_push(&mut self, word: Element, discoverer: [u8; 32]) {
        let addr = word.address();
        self.basis_insert(word, discoverer);
        self.syntropic_on_admit(addr);
    }

    /// Path 2 at admit: the atlas names the sibling a versor relates, the
    /// stratum names a word the new generator derives. Fallback consolidate
    /// still walks the cursor when these reads are empty.
    fn syntropic_on_admit(&mut self, addr: Address) {
        let survivor = self.condense_on(&addr);
        self.refine_stratum(&survivor);
        self.holonomy_orbit(&survivor);
        self.drain_unreachable();
    }

    /// Read the holonomy group on the admitted word: a closed orbit of
    /// gauge transitions condenses through torse_at; an open orbit stays
    /// free; an orbit that fails to close on a Twisted chart germinates a
    /// fibre on a fresh axis seeded with the failing transition.
    fn holonomy_orbit(&mut self, addr: &[u8]) {
        let Some(word) = self.basis.get(addr) else {
            return;
        };
        let trans = word.transitions().to_vec();
        if trans.is_empty() {
            return;
        }
        let inv = crate::transducer::stalk::invariant_of(word);
        let twisted = self.field.get(&word.address()).is_some_and(|f| {
            f.nonflat > 0
                && f.entries.iter().any(|e| {
                    e.chart == inv
                        && matches!(
                            e.state,
                            crate::transducer::stalk::Closure::Twisted(_)
                                | crate::transducer::stalk::Closure::Class(..)
                        )
                })
        });
        let mut asgs: crate::network::nerve::Keyed<Vec<element::Versor>> =
            crate::network::nerve::Keyed::new();
        for (asg, g, _) in trans {
            let mut k = Vec::with_capacity(asg.len() * 2);
            for &(a, b) in &asg {
                k.push(a);
                k.push(b);
            }
            match asgs.get_mut(&k) {
                Some(vs) => vs.push(g),
                None => {
                    asgs.insert(k, vec![g]);
                }
            }
        }
        let mut germinate: Option<element::Versor> = None;
        let mut torse = false;
        for (_, vs) in asgs.fiber(crate::network::nerve::ROOT) {
            match fiber_holonomy(&vs) {
                // Closed: the loop is torsion the language already carries.
                Some(h) if h.projective_eq(&element::identity_versor()) => torse = true,
                // Held: the germ is the holonomy itself, the exact thing the
                // loop failed to close by, not a transition read off the end
                // of the fiber.
                Some(h) if twisted => germinate = Some(h),
                _ => {}
            }
        }
        if torse {
            self.torse_at(addr);
        }
        if let Some(fail) = germinate {
            self.germinate_fibre(addr, &fail);
        }
    }

    fn germinate_fibre(&mut self, addr: &[u8], fail: &element::Versor) {
        let Some(word) = self.basis.get(addr).map(|w| (**w).clone()) else {
            return;
        };
        let Some(seed) = word.versor() else {
            let asg = element::assignment(word.sub());
            if let Some(w) = self.word_mut(addr) {
                w.record_dislocation(asg, fail.clone(), element::FIBER_SPATIAL);
            }
            self.touch_at(addr);
            return;
        };
        let asg = element::assignment(word.sub());
        let axis = match self.word_mut(addr) {
            Some(w) => {
                let axis = w.open_axis(seed);
                w.record_dislocation(asg, fail.clone(), axis);
                axis
            }
            None => return,
        };
        self.touch_at(addr);
        eprintln!(
            "  germinated fibre axis {axis} from a failed orbit on a Twisted chart"
        );
    }

    /// Drain listings whose occupied lanes have each sat through a full
    /// aperture cycle of unmoved probes on every non-flat chart. Relocates
    /// to the cavity atlas; never removes from listing_atlas.

    pub fn prune_basis_to_ruled(&mut self) -> usize {
        let before = self.basis.len();
        let mut nb = crate::network::nerve::Keyed::new();
        let mut nd = crate::network::nerve::Keyed::new();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        // Walk the federation once through the fed index; peel the live word.
        for (addr, elem) in &self.court_federation {
            if elem.is_connection() && elem.letters().is_none() {
                continue;
            }
            if seen.contains(addr) {
                continue;
            }
            seen.insert(&addr, ());
            if let Some(w) = self.basis.get(addr) {
                nb.insert(addr, std::sync::Arc::clone(w));
                if let Some(d) = self.discoverer_of(addr) {
                    nd.insert(&addr, d);
                }
            } else {
                nb.insert(addr, std::sync::Arc::new(elem.clone()));
            }
        }
        if seen.is_empty() {
            // No court cover: the restored or local language stands.
            return 0;
        }
        let dropped = before.saturating_sub(nb.len());
        self.basis = nb;
        self.discoverers = nd;
        self.by_action = crate::network::nerve::Keyed::new();
        self.by_footprint = crate::network::nerve::Keyed::new();
        let mut stalks = crate::transducer::stalk::Stalks::default();
        let mut ruled = crate::transducer::stalk::Stalks::default();
        self.ruled_addrs = crate::network::nerve::Keyed::new();
        let held: Vec<(Vec<u8>, std::sync::Arc<Element>)> = self
            .basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, w)| (a, std::sync::Arc::clone(w)))
            .collect();
        for (addr, w) in &held {
            self.index_word(w);
            let inv = crate::transducer::stalk::invariant_of(w);
            stalks.admit(&inv, addr);
            ruled.admit(&inv, addr);
            self.ruled_addrs.insert(addr, ());
        }
        self.stalks = std::sync::Arc::new(stalks);
        self.ruled = ruled;
        self.refill_axis_index();
        self.refresh_ruled_root();
        self.basis_gen += 1;
        self.retire_gen += 1;
        self.language_rebuild();
        dropped
    }

    /// Restore fills the axis index through the same admits as `ruled_admit`.
    fn refill_axis_index(&mut self) {
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        let strata = self.ruled.strata();
        let mut held = Vec::new();
        for (inv, addrs) in strata {
            for addr in addrs {
                if let Some(w) = self.word_at(&addr) {
                    held.push((
                        inv.clone(),
                        crate::transducer::sentence::words_of_element(w),
                    ));
                }
            }
        }
        for (inv, words) in held {
            crate::transducer::sentence::admit_words(&mut self.axis_index, &inv, &words);
        }
    }

    pub fn ruled_root(&self) -> [u8; 32] {
        self.ruled_root
    }

    /// Test/helper: identity-glue every provisional as if an overlapping
    /// hypha fetched the same word. The basis grows from gluing, not from
    /// the vantage that docketed them.
    pub fn glue_local(&mut self) {
        let words: Vec<Element> = self.provisional.fiber(crate::network::nerve::ROOT).map(|(_, e)| e.clone()).collect();
        let mut cover = crate::network::nerve::Cover::new();
        for w in words {
            let _ = self.glue_fetched(w, [2; 32], [1; 32], &mut cover);
        }
    }

    #[cfg(test)]
    pub(crate) fn leak_unruled(&mut self, w: Element) {
        self.basis_insert(w, [0; 32]);
    }

    #[cfg(test)]
    pub(crate) fn rule_all_provisionals(&mut self) {
        let addrs: Vec<Vec<u8>> = self.provisional.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
        for a in addrs {
            self.ruled_admit(&a);
        }
    }

    fn fed_admit(&mut self, addr: &[u8], spoken: Element) {
        if self.court_fed_index.get(addr).is_some() {
            return;
        }
        self.court_fed_index.insert(&addr, self.court_federation.len());
        self.court_federation.push((Address::from(addr), spoken));
    }

    /// Active cover: the identity fibre (so Čech overlap is non-empty after
    /// the first Cover), plus charts of open listings and of the ruled language.
    pub fn active_cover(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        let mut out = vec![(0, crate::transducer::stalk::invariant_of(&Element::identity()))];
        for (inv, _) in self.listing_atlas.strata() {
            out.push((0, inv));
        }
        for (inv, _) in self.ruled.strata() {
            out.push((0, inv));
        }
        out.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));
        out.dedup();
        out
    }

    /// Whether this address still stands on the quote book.
    pub fn has_listing(&self, addr: &[u8]) -> bool {
        self.pending.contains(addr)
    }

    /// Relocate one listing onto the cavity atlas. The address is kept.
    pub fn relocate_listing(&mut self, addr: &[u8]) {
        self.relocate_cavity(addr);
    }

    /// Relocate restored demands whose chart no longer homes here. Never dropped.
    pub fn unlist_foreign(&mut self, home: impl Fn(&Element) -> bool) {
        let keys: Vec<Address> = self
            .pending
            .fiber(crate::network::nerve::ROOT)
            .filter(|(_, (p, _))| !home(p))
            .map(|(_, (p, _))| p.address())
            .collect();
        for k in keys {
            self.relocate_cavity(&k);
        }
    }

    /// Import a court.federation once as the initial ruled cover.
    /// Letterless words are history and are never taught (not on the basis).
    pub fn import_crt(&mut self, state: crate::court::ledger::CourtState) {
        if self.crt_imported {
            return;
        }
        self.crt_imported = true;
        for (addr, elem) in state.federation {
            let letterless = elem.is_connection() && elem.letters().is_none();
            if !letterless {
                if self.basis.get(&addr).is_none() {
                    self.basis_insert(elem.clone(), [0; 32]);
                }
                self.ruled_admit(&addr);
            }
            self.fed_admit(&addr, elem);
        }
        if !state.chain.receipts().is_empty() && self.chain.receipts().is_empty() {
            self.chain = state.chain;
        }
    }

    pub fn court_federation_len(&self) -> usize {
        self.court_federation.len()
    }

    pub fn crt_imported(&self) -> bool {
        self.crt_imported
    }

    pub fn offered(&self, addr: &[u8]) -> bool {
        self.offered.get(addr).is_some()
    }

    /// The germs this court has Offered and still holds provisionally, with
    /// their tapes — what a second organ needs in order to replay them.
    /// Bounded by the caller's `Budget`: an offer is work like any other and
    /// a stroke sends what it can pay for, never the whole docket.
    pub fn offers_pending(
        &self,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(crate::transducer::a21::Address, Vec<u8>)> {
        let mut out = Vec::new();
        for (addr, _) in self.offered.fiber(crate::network::nerve::ROOT) {
            if !budget.spend() {
                break;
            }
            let Some(w) = self.provisional.get(&addr) else {
                continue;
            };
            out.push((crate::transducer::a21::Address::from(addr), w.to_bytes()));
        }
        out
    }

    pub fn dialect_len(&self) -> usize {
        self.cocycle_at.len()
    }

    pub fn dialect_chain_len(&self) -> usize {
        self.dialect_chain.len()
    }

    pub fn heard_len(&self) -> usize {
        self.heard.len()
    }

    pub fn field_hot_len(&self) -> usize {
        self.field.len()
    }

    pub fn field_cold_len(&self) -> usize {
        self.field_cold.len()
    }

    pub fn settlements_hot_len(&self) -> usize {
        self.settlements.len()
    }

    pub fn settlements_cold_len(&self) -> usize {
        self.settlements_cold.len()
    }

}

impl crate::transducer::family::Crystal for Kernel {
    fn on_class(
        &self,
        class: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        self.ruled
            .fiber(class)
            .into_iter()
            .filter_map(|addr| {
                let e = self.word_at(&addr)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                Some((e, f, crate::transducer::stalk::Standing::Ruled))
            })
            .collect()
    }

    fn speak_form(
        &self,
        f: &crate::transducer::sentence::Form,
        x: &crate::transducer::family::Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }

    fn by_address(
        &self,
        addr: &[u8],
    ) -> Option<(Element, crate::transducer::sentence::Form)> {
        let e = self.word_at(addr)?.clone();
        let f = crate::transducer::sentence::from_element_form(&e)?;
        Some((e, f))
    }

    fn on_axis(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        crate::transducer::sentence::on_axis_of(&self.axis_index, chart, axis)
    }
}

// The blind replay moved to `crate::verdict` (#162): verification is the
// court's substrate, not the server kernel's.

fn sentence_rule_of(form: &Element) -> Option<crate::transducer::sentence::SentenceRule> {
    use crate::transducer::sentence::{from_element_form, rule_of_form};
    Some(rule_of_form(from_element_form(form)?))
}

impl Kernel {
    /// Descent a rule measures by speaking every train pair, its cited
    /// forms resolved through this kernel's ruled cover. Any miss is zero:
    /// association with a listing is not a mint.
    fn measured_replay_descent(&self, form: &Element, family: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> [u64; 4] {
        if let Some(f) = crate::transducer::sentence::from_element_form(form) {
            return crate::transducer::family::form_descent(&f, family, self).0;
        }
        [0; 4]
    }
}

/// The consilience address of a SECTION prospect: the carrying thought's stable
/// base address ⊕ the assignment — so two kernels independently proposing the
/// same axis extension meet at one receipt (wire identity, never reasoned on).
fn section_address(thought: &[u8], asg: &[(u8, u8)]) -> Vec<u8> {
    let mut sigma = asg.to_vec();
    sigma.sort_unstable();
    let mut buf = Vec::new();
    crate::transducer::a21::put_bytes(&mut buf, thought);
    buf.extend_from_slice(&(sigma.len() as u32).to_le_bytes());
    for (b, a) in sigma {
        buf.push(b);
        buf.push(a);
    }
    buf
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

/// THE ACTION POTENTIAL of a listing — the thermodynamic tension deciding
/// eviction order: 𝒜 = ‖μ⃗_quoted‖₁ · R / (R + Δk_dwell), with dwell in
/// K-STEPS (algebraic time) and R = RESERVE_MAX as λ_decay's scale (the
/// metabolism contract, not a new knob). Exact integer arithmetic; higher =
/// hotter = kept. The market sheds argmin 𝒜 — its coldest stale residual —
/// never its oldest.
#[cfg(test)]
fn action_potential(mu_quoted: &[u64; 4], dwell_k: u64) -> u128 {
    let l1: u128 = mu_quoted.iter().map(|&m| m as u128).sum();
    let r = crate::physics::reserve_max() as u128;
    l1 * r / (r + dwell_k as u128)
}

/// The machine's live breath — MemAvailable from /proc/meminfo, in bytes.
/// A platform without the sensor reports half of a modest measured floor
/// rather than pretending to know (the capacity formula degrades gracefully).
#[allow(dead_code)]
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
pub fn germ_address(g: &Element) -> crate::transducer::a21::Address {
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
        .filter_map(|(addr, _st, g, _asg, _)| {
            let i = language.at(addr)?;
            // The gauge coordinate is read from the target, never trusted
            // as an index into this copy of the word.
            let agreeing = crate::transducer::sentence::gauge_of(&language.words[i], g, target);
            let n = agreeing.len();
            let mut it = agreeing.fiber(crate::network::nerve::ROOT);
            let (_, &gauge) = it.next()?;
            if it.next().is_some() {
                eprintln!(
                    "  court cite gauge ambiguous {:02x}{:02x}… {n}",
                    addr[0], addr[1]
                );
                return None;
            }
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
    germ: Element,
    cites: &[crate::anastome::dialogue::CiteWire],
    frame: &crate::arc::Frame,
    listing: crate::transducer::a21::Address,
) -> Reduction {
    if germ.is_sentence() {
        let (form, qs) = reduce_germ(language, (0, 0), &germ);
        return Reduction {
            germ,
            form,
            qs,
            language: std::sync::Arc::clone(language),
            listing,
            frontier: cites.is_empty(),
        };
    }
    let sub = germ.substance();
    // The utterance is verified as spoken: the operators it carries are
    // replayed against the germ; the lexicon is consulted only to name
    // what was verified. A closing operator the language does not hold is
    // the missing generator, listed as the demand.
    let residual = crate::transducer::sentence::verify_utterance(cites, sub.sub());
    let resolved = resolve_cites(language, cites, sub.sub());
    let form = match (residual.as_slice(), resolved.len() == cites.len()) {
        ([], true) => NormalForm::Decomposed(resolved.iter().map(|(i, _, _)| *i).collect()),
        ([], false) => {
            let named: Vec<Address> = resolved.iter().map(|(i, _, _)| language.words[*i].address()).collect();
            let missing = cites.iter().find(|(a, _, _, _, _)| !named.contains(a)).map(|(_, _, _, _, body)| body.clone()).unwrap_or_default();
            NormalForm::Irreducible(element::from_residual_framed(&missing, frame))
        }
        ([_, ..], _) => NormalForm::Irreducible(element::from_residual_framed(&residual, frame)),
    };
    Reduction {
        germ,
        form,
        qs: Vec::new(),
        language: std::sync::Arc::clone(language),
        listing,
        frontier: cites.is_empty(),
    }
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
        Outcome::Sectioned(_) => 6,
        Outcome::Fragmented(_) => 5,
        Outcome::Pivot => 4,
    }
}


#[cfg(test)]
mod tests;
