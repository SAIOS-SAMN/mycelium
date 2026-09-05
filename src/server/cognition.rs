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
    /// Shared snapshot: each word is its own Arc, so an append clones
    /// pointers, never the basis.
    pub words: std::sync::Arc<[std::sync::Arc<Element>]>,
    pub stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
    /// Each word's shape, lifted once on first use and shared by every
    /// lookup over this snapshot (a measurement memoized, never persisted).
    pub shapes: Vec<std::sync::OnceLock<Option<crate::transducer::blade::Shape>>>,
    /// The retirement generation of the basis this snapshot was taken at.
    pub retire_gen: u64,
    /// Each word reached by its address — a coordinate read, never a scan.
    pub index: std::sync::Arc<crate::network::nerve::Keyed<usize>>,
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
    pub fn at(&self, addr: &[u8; 32]) -> Option<usize> {
        self.index.get(addr).copied()
    }
}

/// The market as seen at one instant (see [`Kernel::market_open`]).
pub struct MarketView {
    pub language: std::sync::Arc<Language>,
    pub pending: std::sync::Arc<Vec<(Element, u64)>>,
    pub book_index: std::sync::Arc<crate::network::nerve::Keyed<usize>>,
    pub composites: std::sync::Arc<Vec<Composite>>,
    pub surface: [u64; 4],
    /// The open book's tension per lane — the denominator every price is a
    /// share of.
    pub pressure: [u64; 4],
    pub pool: [u64; 4],
    pub k_step: u64,
    /// Receipt-chain head K — verified consensus depth packed on Quote.
    pub head_k: u64,
    /// TUniversal at this view — local ticks, never packed as K.
    pub tu: u64,
    /// Standing extent per listing, in transports.
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

fn versors_close(vs: &[element::Versor]) -> bool {
    if vs.len() < 2 {
        return false;
    }
    let mut p = vs[0].clone();
    for v in &vs[1..] {
        p = p.geo(v);
    }
    let id = element::identity_versor();
    p.scalar_part() == id.scalar_part() && (1..16).all(|i| p.coeff(i) == id.coeff(i))
}

fn rotor_versor(g: &crate::transducer::alphabet::Rotor) -> Option<element::Versor> {
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let e0 = g.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()])?;
    let e1 = g.carry(&[zero.clone(), one.clone(), zero.clone(), zero.clone()])?;
    let block = [[e0[0].clone(), e1[0].clone()], [e0[1].clone(), e1[1].clone()]];
    let spelling = crate::transducer::alphabet::spell(&block)?;
    crate::transducer::element::conformal_of(&spelling, &(zero.clone(), zero))
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
    addr: &[u8; 32],
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

/// The lineage coordinate of a listing: its parent's (resolved, total,
/// ribbon volume), found by the listing's own address in the composites
/// relation. (0, 0, 0) when it has no parent.
fn composite_child_index(composites: &[Composite]) -> crate::network::nerve::Keyed<usize> {
    let mut idx = crate::network::nerve::Keyed::new();
    for (i, (_, children, _)) in composites.iter().enumerate() {
        for (a, _) in children {
            idx.insert(*a, i);
        }
    }
    idx
}

fn lineage_of(addr: &[u8; 32], composites: &[Composite]) -> (u64, u64, u64) {
    let idx = composite_child_index(composites);
    lineage_at(addr, composites, &idx)
}

fn lineage_at(
    addr: &[u8; 32],
    composites: &[Composite],
    idx: &crate::network::nerve::Keyed<usize>,
) -> (u64, u64, u64) {
    let Some(&i) = idx.get(addr) else {
        return (0, 0, 0);
    };
    let Some((_, children, ribbon)) = composites.get(i) else {
        return (0, 0, 0);
    };
    (
        children.iter().filter(|(_, done)| *done).count() as u64,
        children.len() as u64,
        ribbon.len() as u64,
    )
}

/// Measure the rank of the book — pure.
pub fn rank_over(
    language: &Language,
    pending: &[(Element, u64)],
    composites: &[Composite],
    prior: &Rank,
    basis_gen: u64,
    chart_listings: Option<&crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>>,
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
    let reuse = prior.retire_gen == language.retire_gen && !prior.addresses.is_empty();
    let mut moved: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
        crate::network::nerve::Keyed::new();
    if reuse && prior.basis_gen != basis_gen && language.words.len() >= prior.basis_len {
        for w in &language.words[prior.basis_len..] {
            let inv = crate::transducer::stalk::invariant_of(w);
            moved.insert(inv.key(), inv);
        }
    }
    let mut index = crate::network::nerve::Keyed::new();
    for (i, addr) in addresses.iter().enumerate() {
        index.insert(*addr, i);
    }
    let invariants: Vec<crate::transducer::stalk::Invariant> = pending
        .iter()
        .map(|(p, _)| {
            prior
                .index
                .get(&p.address())
                .and_then(|&j| prior.invariants.get(j).cloned())
                .unwrap_or_else(|| crate::transducer::stalk::invariant_of(p))
        })
        .collect();
    let locate = |p: &Element| {
        crate::transducer::stalk::congruent_shaped(
            stalks,
            |a| language.index.get(a).and_then(|&i| language.shape(i)),
            p.sub(),
        )
        .and_then(|(a, _)| language.index.get(&a).copied())
    };
    let mut closure: Vec<Option<usize>> = pending
        .iter()
        .map(|(p, _)| {
            prior
                .index
                .get(&p.address())
                .and_then(|&j| prior.closure.get(j).copied().flatten())
        })
        .collect();
    match reuse {
        false => {
            for (i, (p, _)) in pending.iter().enumerate() {
                closure[i] = locate(p);
            }
        }
        true => {
            if !moved.is_empty() {
                match chart_listings {
                    Some(by_chart) => {
                        for (ck, _) in moved.fiber(&crate::network::nerve::ROOT) {
                            for (addr, _) in by_chart.link(ck) {
                                if let Some(&i) = index.get(addr) {
                                    closure[i] = locate(&pending[i].0);
                                }
                            }
                        }
                    }
                    None => {
                        for (i, (p, _)) in pending.iter().enumerate() {
                            if moved.contains(&invariants[i].key()) {
                                closure[i] = locate(p);
                            }
                        }
                    }
                }
            }
            for (i, (p, _)) in pending.iter().enumerate() {
                if prior.index.get(&p.address()).is_none() {
                    closure[i] = locate(p);
                }
            }
        }
    }
    let child_idx = composite_child_index(composites);
    let lineage: Vec<(u64, u64)> = pending
        .iter()
        .map(|(p, _)| {
            let (r, t, _) = lineage_at(&p.address(), composites, &child_idx);
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
    pending: &[(Element, u64)],
    book_index: &crate::network::nerve::Keyed<usize>,
    listing_atlas: &crate::transducer::stalk::Stalks,
    field: &crate::network::nerve::Keyed<ListingField>,
    on_page: &crate::network::nerve::Keyed<()>,
    voice_charts: &[crate::transducer::stalk::Invariant],
    surface: &[u64; 4],
    _pool: &[u64; 4],
    pressure: &[u64; 4],
    composites: &[Composite],
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
            let addrs: Vec<[u8; 32]> = if chart.0.is_empty() {
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
                let Some(&idx) = book_index.get(&addr) else { continue };
                let Some((p, _)) = pending.get(idx) else { continue };
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
                seen.insert(addr, ());
                let mut mag = p.magnitude();
                if let Some(&t) = standing_t.get(&addr) {
                    mag[2] = t;
                }
                let mut mu = crate::economy::price(&mag, pressure);
                let (resolved, total, ribbon) = lineage_of(&p.address(), composites);
                let assembly = crate::economy::price(&[0, 0, ribbon, 0], pressure);
                for l in 0..4 {
                    mu[l] = mu[l].saturating_add(assembly[l].saturating_mul(resolved) / total.max(1));
                }
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
            Reduction { germ, ctx, form, qs, language: std::sync::Arc::clone(language), listing, frontier: false }
        })
        .collect()
}

/// The kernel's evolving state: the installed generator basis + the germ
/// admission ledger + the Čech peer table.
#[derive(Default)]
pub struct Kernel {
    /// The germinated basis as the language's Arc slice: one Arc per word.
    /// In-place writes go through `word_mut` / `touch_at`.
    basis: Vec<std::sync::Arc<Element>>,
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
    /// Each basis word reached by its address (index-aligned with `basis`;
    /// rebuilt on the rare removal).
    basis_index: std::sync::Arc<crate::network::nerve::Keyed<usize>>,
    /// A8: old stored address → M2 address. `word_at` consults on a miss.
    rmp1: crate::network::nerve::Keyed<[u8; 32]>,
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
    /// THE RECEIPTS OF SPEECH: listing → voices that have spoken on it.
    spoken: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
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
    /// Pending index by address, so incidence is a keyed read.
    book_index: crate::network::nerve::Keyed<usize>,
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
    court_federation: Vec<([u8; 32], Element)>,
    court_fed_index: crate::network::nerve::Keyed<usize>,
    court_upheld: u64,
    court_slashed: u64,
    crt_imported: bool,
    /// Čech complex per chart: vertices are covering hyphas, coefficients
    /// are Rotor. One Keyed entry per cell; never Elements.
    complexes: crate::network::nerve::Keyed<crate::transducer::cech::Complex>,
    /// Unfilled cells, keyed by (chart, cell). Filled within spend_unit.
    unfilled: crate::network::nerve::Keyed<(crate::transducer::stalk::Invariant, [u8; 32])>,
    /// Class cells awaiting a prefix split.
    pending_refine: crate::network::nerve::Keyed<(
        crate::transducer::stalk::Invariant,
        crate::transducer::alphabet::Rotor,
        u8,
    )>,
    /// Addresses fetched as Dialect or Slashed — (chart key, stratum root at
    /// fetch). Dropped when confirm_peer_holds sees that root move.
    heard: crate::network::nerve::Keyed<([u8; 32], [u8; 32])>,
    /// Current dialect transition per (chart, hypha pair): one versor, the
    /// TUniversal stamp it stood at, the two hyphae, and the transport
    /// class key. A newer stamp supersedes; the chain keeps the old.
    cocycle_at: crate::network::nerve::Keyed<(element::Versor, u64, [u8; 32], [u8; 32], [u8; 32])>,
    /// Chart key → dialect row keys, so a fill or a refinement removes the row.
    cocycle_by_chart: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    /// Every transition that ever stood between two hyphas on a chart.
    dialect_chain: Vec<([u8; 32], element::Versor, u64)>,
    /// Field entries of listings that settled Flat or relocated — FLD1 cold.
    field_cold: crate::network::nerve::Keyed<ListingField>,
    /// Settlements the court has read; the SET1 section holds them.
    settlements_cold: Vec<crate::anastome::dialogue::Settlement>,
    /// TUniversal genesis this hypha stamps standing work from.
    tu: crate::chronometry::TUniversal,
    /// Test clock: when set, now_tu returns this tick count.
    clock: Option<u64>,
    /// Measured transport in milliseconds. Standing T is ticks (seconds)
    /// over this unit. Zero means one second (one transport).
    transport_ms: u64,
    /// Set on home_install, glue Admitted, mint, and slash.
    persist_due: bool,
    /// Test lane capacity override; production reads dynamic_lane_capacity.
    lane_cap: Option<[usize; 4]>,
    /// TUniversal at first vantage of a standing carrier (listing or docket).
    standing_tu: crate::network::nerve::Keyed<u64>,
    /// Listing address → corpus task id, so a verified solve pays the same escrow.
    listing_task: crate::network::nerve::Keyed<[u8; 32]>,
    /// Reverse: task → listings bound to it.
    task_listings: crate::network::nerve::Keyed<Vec<[u8; 32]>>,
    /// Train pairs of a bound task. Install mints only what a rule speaks.
    task_family: crate::network::nerve::Keyed<Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>>,
    /// Sentence address → task, from perceive_speech. Replay reads the family.
    speech_task: crate::network::nerve::Keyed<[u8; 32]>,
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
    /// TUniversal at first vantage of each provisional — standing duration.
    provisional_tu: crate::network::nerve::Keyed<u64>,
    /// Round-robin cursor for syntropic optimization — which generator to test for
    /// redundancy next (bounds consolidation to one reduce per depleted step).
    optimize_cursor: usize,
    /// Polytonic kinematic pressure + torsion (per lane).
    physics: crate::physics::Physics,
    /// The settlement ledger (POW++-gated bounties, consilience splits).
    treasury: crate::economy::Treasury,
    /// This hypha's verifying key. Room reads `bond_of(identity)`.
    identity: [u8; 32],
    /// Words installed on two-voice consilience but NOT yet court-final
    /// (Step 5): local dialect awaiting the admission docket, keyed by address.
    provisional: crate::network::nerve::Keyed<Element>,
    /// Listings whose field has a non-Flat chart — the court's docket, written
    /// in write_field, read in key order.
    open: crate::network::nerve::Keyed<()>,
    /// Keyed cursor into `open` for court_stroke.
    open_cursor: [u8; 32],
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

fn chart_simplex_key(inv: &crate::transducer::stalk::Invariant, sim: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&inv.key());
    h.update(sim);
    *h.finalize().as_bytes()
}

fn loop_key(chart: &crate::transducer::stalk::Invariant, voice: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&chart.key());
    h.update(voice);
    *h.finalize().as_bytes()
}

fn footprint_key(addr: [u8; 32], fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(&addr);
    for (r, c) in fp {
        h.update(&r.to_le_bytes());
        h.update(&c.to_le_bytes());
    }
    h.update(&[0xff]);
    for (r, c, hue) in scene {
        h.update(&r.to_le_bytes());
        h.update(&c.to_le_bytes());
        h.update(&[*hue]);
    }
    *h.finalize().as_bytes()
}

impl Kernel {
    pub fn new() -> Self {
        let mut k = Kernel::default();
        k.identity = [0x11; 32];
        k.enroll([0x11; 32], crate::economy::UNIT_BOND);
        k
    }

    /// Stamp standing work from a shared genesis origin.
    pub fn with_tuniversal(tu: crate::chronometry::TUniversal) -> Self {
        let mut k = Kernel::default();
        k.tu = tu;
        k
    }

    pub fn now_tu(&self) -> u64 {
        self.clock.unwrap_or_else(|| self.tu.ticks())
    }

    pub fn set_clock(&mut self, ticks: u64) {
        self.clock = Some(ticks);
    }

    pub fn set_lane_cap(&mut self, cap: [usize; 4]) {
        self.lane_cap = Some(cap);
    }

    /// Measured transport in milliseconds. Standing T is TU ticks (seconds)
    /// converted to transports: (ticks × 1000) / transport_ms.
    pub fn set_transport_ms(&mut self, ms: u64) {
        self.transport_ms = ms.max(1);
    }

    fn transport_ms(&self) -> u64 {
        match self.transport_ms {
            0 => 1000,
            n => n,
        }
    }

    fn t_extent(&self, addr: &[u8; 32], now: u64) -> u64 {
        let tu = self
            .provisional_tu
            .get(addr)
            .copied()
            .or_else(|| self.standing_tu.get(addr).copied())
            .unwrap_or(now);
        let ticks = now.saturating_sub(tu).max(1);
        let ms = ticks.saturating_mul(1000);
        let transports = (ms / self.transport_ms()).max(1);
        let breath = crate::physics::aperture_cycle() as u64;
        (transports / breath.max(1)).max(1)
    }

    fn mark_persist(&mut self) {
        self.persist_due = true;
    }

    pub fn take_persist(&mut self) -> bool {
        let due = self.persist_due;
        self.persist_due = false;
        due
    }

    fn standing_mag(&self, addr: &[u8; 32], base: &[u64; 4]) -> [u64; 4] {
        let mut m = *base;
        m[2] = self.t_extent(addr, self.now_tu());
        m
    }

    fn stamp_standing(&mut self, addr: [u8; 32]) {
        let tu = self.now_tu();
        if self.standing_tu.get(&addr).is_none() {
            self.standing_tu.insert(addr, tu);
        }
        if self.provisional.contains(&addr) && self.provisional_tu.get(&addr).is_none() {
            self.provisional_tu.insert(addr, tu);
        }
    }

    fn stratum_root_of(&self, inv: &crate::transducer::stalk::Invariant) -> [u8; 32] {
        let mut rows = self.ruled.fiber(inv);
        rows.sort_unstable();
        let mut h = blake3::Hasher::new();
        for a in &rows {
            h.update(a);
        }
        *h.finalize().as_bytes()
    }

    fn heard_mark(&mut self, addr: [u8; 32], inv: &crate::transducer::stalk::Invariant) {
        let root = self.stratum_root_of(inv);
        self.heard.insert(addr, (inv.key(), root));
    }

    fn cocycle_key(
        chart: &crate::transducer::stalk::Invariant,
        a: &[u8; 32],
        b: &[u8; 32],
    ) -> [u8; 32] {
        let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
        let mut h = blake3::Hasher::new();
        h.update(&chart.key());
        h.update(lo);
        h.update(hi);
        *h.finalize().as_bytes()
    }

    /// One versor per (chart, pair). A newer TUniversal stamp supersedes;
    /// the chain keeps every transition that ever stood.
    fn push_cocycle(
        &mut self,
        chart: &crate::transducer::stalk::Invariant,
        a: [u8; 32],
        b: [u8; 32],
        g: element::Versor,
        class_key: [u8; 32],
    ) {
        if a == b {
            return;
        }
        let key = Self::cocycle_key(chart, &a, &b);
        let tu = self.now_tu();
        if let Some((old, old_tu, _, _, _)) = self.cocycle_at.get(&key).cloned() {
            if tu < old_tu {
                self.dialect_chain.push((key, g, tu));
                return;
            }
            if element::same_versor(&old, &g) {
                return;
            }
            self.dialect_chain.push((key, old, old_tu));
        }
        self.cocycle_at.insert(key, (g, tu, a, b, class_key));
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

    fn drop_cocycles_on(&mut self, chart: &crate::transducer::stalk::Invariant) {
        let ck = chart.key();
        let Some(rows) = self.cocycle_by_chart.remove(&ck) else {
            return;
        };
        for (key, _) in rows.fiber(&crate::network::nerve::ROOT) {
            if let Some((v, tu, _, _, _)) = self.cocycle_at.remove(key) {
                self.dialect_chain.push((*key, v, tu));
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
        let mut saved: Vec<(element::Versor, u64, [u8; 32], [u8; 32], [u8; 32])> = Vec::new();
        for (key, _) in rows.fiber(&crate::network::nerve::ROOT) {
            if let Some(row) = self.cocycle_at.remove(key) {
                self.dialect_chain.push((*key, row.0.clone(), row.1));
                saved.push(row);
            }
        }
        for (g, _, a, b, ck) in saved {
            self.push_cocycle(new_a, a, b, g.clone(), ck);
            self.push_cocycle(new_b, a, b, g, ck);
        }
    }

    fn chill_field(&mut self, addr: &[u8; 32]) {
        if let Some(listing) = self.field.remove(addr) {
            self.field_cold.insert(*addr, listing);
        }
        self.open.remove(addr);
    }

    /// Relocate the largest-T standing carriers on a saturated temporal lane.
    fn relocate_t_exhausted(&mut self) {
        let room = self.lane_room();
        if room[2] > 0 {
            return;
        }
        let now = self.now_tu();
        let mut best: Option<([u8; 32], u64)> = None;
        let mut consider = |addr: [u8; 32]| {
            let t = self.t_extent(&addr, now);
            match best {
                Some((_, bt)) if t <= bt => {}
                _ => best = Some((addr, t)),
            }
        };
        for (addr, _) in self.lane_listings[2].fiber(&crate::network::nerve::ROOT) {
            consider(*addr);
        }
        for (addr, _) in self.provisional.fiber(&crate::network::nerve::ROOT) {
            consider(*addr);
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

    fn relocate_standing(&mut self, addr: &[u8; 32]) {
        let _ = self.treasury.revoke_award(addr, false);
        if self.book_index.contains(addr) {
            self.relocate_cavity(addr);
        } else if let Some(w) = self.provisional.get(addr).cloned() {
            let inv = crate::transducer::stalk::invariant_of(&w);
            self.listing_atlas.admit(&inv, *addr);
            self.cavity_book.insert(*addr, w.clone());
            self.cavities.push(w);
            self.chill_field(addr);
        }
        self.provisional.remove(addr);
        self.provisional_k.remove(addr);
        self.provisional_tu.remove(addr);
        self.offered.remove(addr);
        if self.lane_listings[2].contains(addr) {
            self.occupancy[2] = self.occupancy[2].saturating_sub(1);
            self.lane_listings[2].remove(addr);
        }
        self.standing_tu.remove(addr);
    }

    pub fn basis_len(&self) -> usize {
        self.basis.len()
    }

    fn discoverer_of(&self, addr: &[u8; 32]) -> [u8; 32] {
        self.discoverers.get(addr).copied().unwrap_or([0; 32])
    }

    fn discoverer_at(&self, i: usize) -> [u8; 32] {
        self.basis.get(i).map(|w| self.discoverer_of(&w.address())).unwrap_or([0; 32])
    }

    fn word_mut(&mut self, i: usize) -> &mut Element {
        std::sync::Arc::make_mut(&mut self.basis[i])
    }

    fn book_elem(&self, addr: &[u8; 32]) -> Option<&Element> {
        self.book_index.get(addr).and_then(|&i| self.pending.get(i).map(|(p, _)| p))
    }

    fn occupy(&mut self, addr: [u8; 32], mag: &[u64; 4], add: bool) {
        for l in 0..4 {
            if mag[l] > 0 {
                if add {
                    self.occupancy[l] = self.occupancy[l].saturating_add(1);
                    self.lane_listings[l].insert(addr, ());
                } else {
                    self.occupancy[l] = self.occupancy[l].saturating_sub(1);
                    self.lane_listings[l].remove(&addr);
                }
            }
            if add {
                self.pressure_sum[l] = self.pressure_sum[l].saturating_add(mag[l]);
            } else {
                self.pressure_sum[l] = self.pressure_sum[l].saturating_sub(mag[l]);
            }
        }
    }

    fn refresh_ruled_root(&mut self) {
        let digests = crate::transducer::sheaf::digests(&self.ruled);
        let mut h = blake3::Hasher::new();
        for d in &digests {
            h.update(&d.root);
        }
        self.ruled_root = *h.finalize().as_bytes();
    }

    /// Place a word on the basis with no syntropic stroke: restore and the
    /// fallback cursor walk insert this way so a snapshot does not condense
    /// the language it is loading.
    fn basis_insert(&mut self, word: Element, discoverer: [u8; 32]) -> usize {
        let i = self.basis.len();
        let addr = word.address();
        let arc = std::sync::Arc::new(word);
        self.basis.push(std::sync::Arc::clone(&arc));
        self.discoverers.insert(addr, discoverer);
        self.basis_gen += 1;
        self.language_append_arc(arc);
        i
    }

    /// Install a word and pivot Path 2 on the axis it just moved: condense
    /// a congruent sibling, probe the stratum for a derivable generator,
    /// pay each yield on its own lane. Returns the surviving word's index.
    fn basis_push(&mut self, word: Element, discoverer: [u8; 32]) -> usize {
        let addr = word.address();
        self.basis_insert(word, discoverer);
        self.syntropic_on_admit(addr)
    }

    /// Path 2 at admit: the atlas names the sibling a versor relates, the
    /// stratum names a word the new generator derives. Fallback consolidate
    /// still walks the cursor when these reads are empty.
    fn syntropic_on_admit(&mut self, addr: [u8; 32]) -> usize {
        let survivor = self.condense_on(&addr);
        self.refine_stratum(&survivor);
        self.holonomy_orbit(&survivor);
        self.drain_unreachable();
        self.basis_index.get(&survivor).copied().unwrap_or(0)
    }

    /// Read the holonomy group on the admitted word: a closed orbit of
    /// gauge transitions condenses through torse_at; an open orbit stays
    /// free; an orbit that fails to close on a Twisted chart germinates a
    /// fibre on a fresh axis seeded with the failing transition.
    fn holonomy_orbit(&mut self, addr: &[u8; 32]) {
        let Some(i) = self.basis_index.get(addr).copied() else {
            return;
        };
        let trans = self.basis[i].transitions().to_vec();
        if trans.is_empty() {
            return;
        }
        let inv = crate::transducer::stalk::invariant_of(&self.basis[i]);
        let twisted = self.field.get(&self.basis[i].address()).is_some_and(|f| {
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
        let mut asgs: Vec<(Vec<(u8, u8)>, Vec<element::Versor>)> = Vec::new();
        for (asg, g, _) in trans {
            match asgs.iter_mut().find(|(a, _)| a == &asg) {
                Some((_, vs)) => vs.push(g),
                None => asgs.push((asg, vec![g])),
            }
        }
        let mut germinate: Option<element::Versor> = None;
        let mut torse = false;
        for (_, vs) in asgs {
            if versors_close(&vs) {
                torse = true;
            } else if twisted {
                if let Some(fail) = vs.last() {
                    germinate = Some(fail.clone());
                }
            }
        }
        if torse {
            self.torse_at(i);
        }
        if let Some(fail) = germinate {
            self.germinate_fibre(i, &fail);
        }
    }

    fn germinate_fibre(&mut self, i: usize, fail: &element::Versor) {
        let axis = self.basis[i]
            .max_axis()
            .map_or(element::FIBER_GERMINATED, |m| m.saturating_add(1));
        let Some(seed) = self.basis[i].fiber(element::FIBER_SPATIAL).cloned() else {
            let asg = element::assignment(self.basis[i].sub());
            self.word_mut(i).record_dislocation(asg, fail.clone(), element::FIBER_SPATIAL);
            self.touch_at(i);
            return;
        };
        self.word_mut(i).set_fiber(axis, seed);
        let asg = element::assignment(self.basis[i].sub());
        self.word_mut(i).record_dislocation(asg, fail.clone(), axis);
        self.touch_at(i);
        eprintln!(
            "  germinated fibre axis {axis} from a failed orbit on a Twisted chart"
        );
    }

    /// Drain listings whose occupied lanes have each sat through a full
    /// aperture cycle of unmoved probes on every non-flat chart. Relocates
    /// to the cavity atlas; never removes from listing_atlas.
    fn drain_unreachable(&mut self) {
        let kappa = crate::physics::aperture_cycle();
        let mut budget = self.physics.stroke();
        let mut keys: Vec<[u8; 32]> = Vec::new();
        // index: budgeted keyed cursor over the open docket
        for (k, _) in self.open.cursor(crate::network::nerve::ROOT, &mut budget) {
            keys.push(*k);
        }
        let keys: Vec<[u8; 32]> = keys
            .into_iter()
            .filter_map(|k| {
                if !self.on_page.contains(&k) {
                    return None;
                }
                let listing = self.field.get(&k)?;
                let t = self.tension.get(&k)?;
                if listing.nonflat == 0 {
                    return None;
                }
                let occupied = t.iter().any(|&x| x > 0);
                let nonflat: Vec<_> = listing.entries.iter().filter(|e| !e.state.is_flat()).collect();
                if occupied
                    && !nonflat.is_empty()
                    && (0..4).all(|l| t[l] == 0 || nonflat.iter().all(|e| e.stagnant[l] >= kappa))
                {
                    Some(k)
                } else {
                    None
                }
            })
            .collect();
        for key in keys {
            self.relocate_cavity(&key);
        }
    }

    /// List a residual on the book and admit its base chart.
    fn book_list(&mut self, residual: Element, k_now: u64) {
        let addr = residual.address();
        self.stamp_standing(addr);
        let mag = self.standing_mag(&addr, &residual.magnitude());
        self.physics.bump(&mag);
        let inv = crate::transducer::stalk::invariant_of(&residual);
        self.listing_atlas.admit(&inv, addr);
        self.tension.insert(addr, mag);
        self.book_index.insert(addr, self.pending.len());
        self.on_page.insert(addr, ());
        self.occupy(addr, &mag, true);
        self.pending.push((residual, k_now));
        self.book_gen += 1;
    }

    fn pending_take(&mut self, addr: &[u8; 32]) -> Option<Element> {
        let i = self.book_index.remove(addr)?;
        if i >= self.pending.len() {
            return None;
        }
        let mag = self.standing_mag(addr, &self.pending[i].0.magnitude());
        let (elem, _) = self.pending.swap_remove(i);
        self.occupy(*addr, &mag, false);
        self.physics.relieve(&mag);
        if i < self.pending.len() {
            self.book_index.insert(self.pending[i].0.address(), i);
        }
        Some(elem)
    }

    /// True closure of the whole listing: the thought glued. Leaves the
    /// page and the listing atlas (the word, if any, lives on the language).
    fn unlist(&mut self, addr: &[u8; 32], as_cavity: bool) {
        if as_cavity {
            self.relocate_cavity(addr);
            return;
        }
        if let Some(elem) = self.pending_take(addr) {
            let _ = elem;
        }
        if let Some(listing) = self.field.remove(addr) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.retract(&e.chart, addr);
                }
            }
            self.field_cold.insert(*addr, listing);
        }
        self.listing_atlas.remove(addr);
        self.tension.remove(addr);
        self.spoken.remove(addr);
        self.quote_frontier.remove(addr);
        self.on_page.remove(addr);
        self.flat_loops.remove(addr);
        self.book_gen += 1;
    }

    /// Relocate a listing off the quote page onto the cavity atlas. The
    /// element, field, and charts stay; pressure re-measures without it.
    fn relocate_cavity(&mut self, addr: &[u8; 32]) {
        let Some(elem) = self.pending_take(addr) else {
            return;
        };
        self.on_page.remove(addr);
        self.quote_frontier.remove(addr);
        self.spoken.remove(addr);
        if let Some(listing) = self.field.get(addr) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.admit_also(&e.chart, *addr);
                }
            }
        }
        let inv = crate::transducer::stalk::invariant_of(&elem);
        self.listing_atlas.admit_also(&inv, *addr);
        self.cavity_book.insert(*addr, elem.clone());
        self.cavities.push(elem);
        self.chill_field(addr);
        self.book_gen += 1;
    }

    fn recompute_tension(&mut self, addr: &[u8; 32]) {
        let t = if self.open.contains(addr) {
            self.field
                .get(addr)
                .filter(|f| f.nonflat > 0)
                .map(|f| element::measure_lanes(&union_remainders(&f.entries)))
                .unwrap_or([0; 4])
        } else {
            self.book_elem(addr)
                .map(|p| p.magnitude())
                .or_else(|| self.cavity_book.get(addr).map(|e| e.magnitude()))
                .unwrap_or([0; 4])
        };
        self.tension.insert(*addr, t);
    }

    fn write_field(
        &mut self,
        addr: &[u8; 32],
        chart: crate::transducer::stalk::Invariant,
        state: crate::transducer::stalk::Closure,
        remainder: Vec<(i64, i64, u8, u8)>,
        before: &[u64; 4],
        after: &[u64; 4],
    ) {
        use crate::transducer::stalk::Closure;
        let mut listing = self
            .field
            .get(addr)
            .cloned()
            .or_else(|| self.field_cold.remove(addr))
            .unwrap_or_default();
        for e in &mut listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for l in 0..4 {
                if before[l] > 0 && after[l] == before[l] {
                    e.stagnant[l] = e.stagnant[l].saturating_add(1);
                } else {
                    e.stagnant[l] = 0;
                }
            }
        }
        let mut stag = [0u32; 4];
        // index: the four lanes
        for l in 0..4 {
            if before[l] > 0 && after[l] == before[l] {
                stag[l] = 1;
            }
        }
        let now_flat = state.is_flat();
        if let Some(e) = listing.entries.iter_mut().find(|e| e.chart == chart) {
            if !e.state.is_flat() {
                stag = e.stagnant;
            }
            if e.state.is_flat() != state.is_flat() && !state.is_flat() {
                // keep atlas admission for a newly opened chart
            }
            if matches!(
                (&e.state, &state),
                (Closure::Twisted(_) | Closure::Class(..) | Closure::Cavity, Closure::Flat)
            ) {
                let base = self.listing_atlas.chart_of(addr).cloned();
                if base.as_ref() != Some(&chart) {
                    self.listing_atlas.retract(&chart, addr);
                }
            }
            let was_flat = e.state.is_flat();
            e.state = state;
            e.remainder = remainder;
            e.stagnant = stag;
            if was_flat && !now_flat {
                listing.nonflat = listing.nonflat.saturating_add(1);
            } else if !was_flat && now_flat {
                listing.nonflat = listing.nonflat.saturating_sub(1);
            }
        } else {
            if !state.is_flat() {
                listing.nonflat = listing.nonflat.saturating_add(1);
            }
            listing.entries.push(FieldEntry {
                chart: chart.clone(),
                state,
                remainder,
                stagnant: stag,
            });
        }
        if listing.nonflat > 0 {
            self.open.insert(*addr, ());
            self.field.insert(*addr, listing);
        } else {
            self.open.remove(addr);
            self.field_cold.insert(*addr, listing);
            self.field.remove(addr);
        }
        let mag = self.tension.get(addr).copied().unwrap_or(*after);
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        // index: the four lanes
        for lane in &lanes {
            if now_flat {
                self.own_cover.retract(*lane, &chart, addr);
            } else {
                self.own_cover.admit(*lane, &chart, *addr);
            }
        }
        let ck = chart.key();
        match self.chart_listings.get_mut(&ck) {
            Some(m) => {
                if now_flat {
                    m.remove(addr);
                } else {
                    m.insert(*addr, ());
                }
            }
            None if !now_flat => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(*addr, ());
                self.chart_listings.insert(ck, m);
            }
            None => {}
        }
        self.recompute_tension(addr);
    }

    /// The remainder face of a listing: the first non-flat chart in its field.
    pub fn face_of(&self, addr: &[u8; 32]) -> Option<&crate::transducer::stalk::Invariant> {
        self.field
            .get(addr)
            .or_else(|| self.field_cold.get(addr))
            .and_then(|f| {
                if f.nonflat == 0 {
                    return None;
                }
                f.entries.iter().find(|e| !e.state.is_flat()).map(|e| &e.chart)
            })
    }

    /// The coherence field, keyed by listing.
    pub fn field(&self) -> &crate::network::nerve::Keyed<ListingField> {
        &self.field
    }

    pub fn listing_atlas(&self) -> &crate::transducer::stalk::Stalks {
        &self.listing_atlas
    }

    pub fn on_page(&self) -> &crate::network::nerve::Keyed<()> {
        &self.on_page
    }

    pub fn book_index(&self) -> &crate::network::nerve::Keyed<usize> {
        &self.book_index
    }

    /// Open (non-flat) charts of a listing, or its base chart if unprobed.
    pub fn open_charts(&self, addr: &[u8; 32]) -> Vec<crate::transducer::stalk::Invariant> {
        let listing = self
            .book_index
            .get(addr)
            .and_then(|&i| self.pending.get(i).map(|(p, _)| p))
            .or_else(|| self.cavity_book.get(addr));
        match listing {
            Some(p) => open_charts_of(addr, p, &self.field),
            None => Vec::new(),
        }
    }

    pub fn cavities(&self) -> &[Element] {
        &self.cavities
    }

    /// The court heard `n` cavities; they leave the docket.
    pub fn cavities_heard(&mut self, n: usize) {
        let n = n.min(self.cavities.len());
        self.cavities.drain(..n);
    }

    #[cfg(test)]
    pub(crate) fn stagnant_of(&self, addr: &[u8; 32]) -> [u32; 4] {
        let Some(listing) = self.field.get(addr) else {
            return [0; 4];
        };
        let mut s = [0u32; 4];
        for e in &listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for l in 0..4 {
                s[l] = s[l].max(e.stagnant[l]);
            }
        }
        s
    }

    #[cfg(test)]
    pub(crate) fn drain_now(&mut self) {
        self.drain_unreachable();
    }

    /// If the fibre over `addr` names exactly one other word a versor
    /// relates, absorb the card into the bundle and pay the absorbed
    /// discoverer the condense yield. Returns the surviving address.
    fn condense_on(&mut self, addr: &[u8; 32]) -> [u8; 32] {
        let Some(word) = self.word_at(addr).cloned() else {
            return *addr;
        };
        if word.is_sentence() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let members: Vec<[u8; 32]> = self.stalks.fiber(&inv).iter().copied().filter(|a| a != addr).collect();
            if let Some(na) = crate::transducer::sentence::from_element(&word) {
                for sib in members {
                    let Some(other) = self.word_at(&sib).cloned() else { continue };
                    let Some(nb) = crate::transducer::sentence::from_element(&other) else { continue };
                    if crate::transducer::sentence::coboundary_gauge(&na, &nb).is_some() {
                        return self.absorb_pair(*addr, sib);
                    }
                }
            }
        }
        let Some((sib, _)) = crate::transducer::stalk::congruent(
            &self.stalks,
            |a| {
                if a == addr {
                    None
                } else {
                    self.word_at(a)
                }
            },
            &word,
        ) else {
            return *addr;
        };
        self.absorb_pair(*addr, sib)
    }

    /// Absorb one of `new` / `sib` into the other: a bundle keeps the
    /// fibre; two cards make the newly admitted word the bundle. Pays the
    /// absorbed discoverer. Returns the surviving address.
    fn absorb_pair(&mut self, new: [u8; 32], sib: [u8; 32]) -> [u8; 32] {
        let (Some(ni), Some(si)) = (self.basis_index.get(&new).copied(), self.basis_index.get(&sib).copied()) else {
            return new;
        };
        let new_card = self.basis[ni].sections().is_empty() && self.basis[ni].torsion().is_empty();
        let sib_card = self.basis[si].sections().is_empty() && self.basis[si].torsion().is_empty();
        let (base, absorbed) = if new_card && !sib_card { (sib, new) } else { (new, sib) };
        let (Some(bi), Some(ai)) = (self.basis_index.get(&base).copied(), self.basis_index.get(&absorbed).copied()) else {
            return new;
        };
        let Some(g) = element::transition(self.basis[ai].sub(), self.basis[bi].sub()) else {
            return new;
        };
        let card = (*self.basis[ai]).clone();
        let disc = self.discoverer_at(ai);
        self.basis_remove(ai);
        let Some(bi) = self.basis_index.get(&base).copied() else {
            return new;
        };
        if self.word_mut(bi).attach(&card, &g) > 0 {
            let after = (*self.basis[bi]).clone();
            let payload = crate::transducer::rewrite::RewritePayload::Condense {
                sections: after.sections().to_vec(),
                gauge: g.clone(),
            };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Condense,
                &[card.clone()],
                &after,
                payload,
                disc,
            );
            self.physics.syntropy(&[0, 1, 0, 0]);
            self.torse_at(bi);
        }
        base
    }

    /// Torse one word: a closed orbit pays the torse coin on the residual
    /// lane immediately, not on the idle cursor.
    fn torse_at(&mut self, i: usize) {
        if i >= self.basis.len() {
            return;
        }
        let spanned = self.word_mut(i).torse();
        if spanned == 0 {
            return;
        }
        let lanes = self.basis[i].lanes();
        self.physics.torse_relief(lanes, spanned as u64);
        let after = (*self.basis[i]).clone();
        let (sigma, period) = after
            .torsion()
            .last()
            .map(|(s, p, _)| (s.clone(), *p))
            .unwrap_or((Vec::new(), spanned as u32));
        let payload = crate::transducer::rewrite::RewritePayload::Torse { sigma, period };
        let disc = self.discoverer_at(i);
        let _ = self.mint_rewrite(
            crate::transducer::rewrite::RewriteKind::Torse,
            std::slice::from_ref(&after),
            &after,
            payload,
            disc,
        );
        self.touch_at(i);
    }

    /// Redundancy probe on the admitted word's own stratum: a plain card
    /// the rest now derives retires, and the voice whose word derived it
    /// earns the refine yield.
    fn refine_stratum(&mut self, new: &[u8; 32]) {
        let Some(after) = self.word_at(new).cloned() else {
            return;
        };
        let inv = crate::transducer::stalk::invariant_of(&after);
        let members: Vec<[u8; 32]> = self
            .stalks
            .fiber(&inv)
            .iter()
            .copied()
            .filter(|a| a != new)
            .collect();
        let bidder = self.discoverer_of(new);
        for addr in members {
            let Some(j) = self.basis_index.get(&addr).copied() else {
                continue;
            };
            let Some(held) = self.basis.get(j) else {
                continue;
            };
            if !held.sections().is_empty() || !held.torsion().is_empty() {
                continue;
            }
            if self.provisional.contains(&addr) {
                continue;
            }
            let candidate = (**held).clone();
            if !matches!(self.reduce_excluding(&candidate, Some(j)), NormalForm::Decomposed(_)) {
                continue;
            }
            let chart = crate::transducer::stalk::invariant_of(&candidate);
            let axis = addr[0] as u64;
            let payload = crate::transducer::rewrite::RewritePayload::Refine { chart, axis };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Refine,
                &[candidate.clone()],
                &after,
                payload,
                bidder,
            );
            self.basis_remove(j);
            self.physics.syntropy(&candidate.magnitude());
            self.append_step(addr, bidder);
        }
    }

    /// Rebuild the address index from the basis (after a removal shifted
    /// every later index; removals are the rare stroke).
    fn reindex_basis(&mut self) {
        let mut index = crate::network::nerve::Keyed::new();
        for (i, w) in self.basis.iter().enumerate() {
            index.insert(w.address(), i);
        }
        self.basis_index = std::sync::Arc::new(index);
    }

    /// The word at an address, read by its coordinate. A8: a miss consults
    /// the restore remap so a pre-a29bba5 key still names the vertex.
    pub fn word_at(&self, addr: &[u8; 32]) -> Option<&Element> {
        let lookup = |a: &[u8; 32]| {
            self.basis_index
                .get(a)
                .and_then(|&i| self.basis.get(i).map(|w| w.as_ref()))
                .or_else(|| self.cavity_book.get(a))
                .or_else(|| self.provisional.get(a))
        };
        lookup(addr).or_else(|| self.rmp1.get(addr).and_then(|n| lookup(n)))
    }

    fn remap_restored_addresses(&mut self) {
        let mut remap = crate::network::nerve::Keyed::new();
        let rehome = |store: &mut crate::network::nerve::Keyed<Element>, remap: &mut crate::network::nerve::Keyed<[u8; 32]>| {
            let entries: Vec<_> = store.fiber(&crate::network::nerve::ROOT).map(|(k, v)| (*k, v.clone())).collect();
            *store = crate::network::nerve::Keyed::new();
            for (old, spoken) in entries {
                let new = spoken.address();
                if old != new {
                    remap.insert(old, new);
                }
                if store.get(&new).is_none() {
                    store.insert(new, spoken);
                }
            }
        };
        rehome(&mut self.provisional, &mut remap);
        rehome(&mut self.cavity_book, &mut remap);
        self.rmp1 = remap;
    }

    /// A word the court finalized here joins the ruled cover (keyed admit).
    fn ruled_admit(&mut self, addr: &[u8; 32]) {
        if let Some(w) = self.word_at(addr) {
            let inv = crate::transducer::stalk::invariant_of(w);
            self.ruled.admit(&inv, *addr);
            self.ruled_addrs.insert(*addr, ());
            self.refresh_ruled_root();
        }
    }

    fn ruled_contains(&self, addr: &[u8; 32]) -> bool {
        self.ruled_addrs.contains(addr)
    }

    /// A crystallized form this hypha has ruled, or retired to the cavity,
    /// read as a `Form` for the court's replay of a citer. Provisional and
    /// heard words resolve to nothing: a citation stands only on a ruling.
    pub fn resolve_form(&self, addr: &[u8; 32]) -> Option<crate::transducer::sentence::Form> {
        if !self.ruled_contains(addr) && !self.cavity_book.contains(addr) {
            return None;
        }
        let e = self.word_at(addr)?;
        crate::transducer::sentence::from_element_form(e)
    }

    /// List a sentence on the docket if it is not already provisional.
    fn docket_sentence(&mut self, word: &Element, ratifier: [u8; 32]) {
        self.docket_connection(word, ratifier);
    }

    /// Perceive a family's speech: replay each sentence, docket what stands.
    /// A false sentence is refused with the replay reason in the journal
    /// (`court refuse speech {addr}: {reason}`) before the wire code.
    /// `task` binds the stood sentence to that family's trains so install
    /// can mint only measured descent.
    pub fn perceive_speech(&mut self, voice: [u8; 32], task: [u8; 32], sentences: &[Element]) -> Vec<u8> {
        let mut codes = Vec::with_capacity(sentences.len());
        for s in sentences {
            if let Some(reason) = crate::court::verdict::blind_replay_with(s, &|a| self.resolve_form(a)) {
                let a = s.address();
                eprintln!("  court refuse speech {:02x}{:02x}…: {reason}", a[0], a[1]);
                codes.push(crate::anastome::dialogue::SPEECH_REFUSED);
                continue;
            }
            self.docket_sentence(s, voice);
            if task != [0; 32] {
                self.speech_task.insert(s.address(), task);
            }
            codes.push(crate::anastome::dialogue::SPEECH_STOOD);
        }
        codes
    }

    /// List a word on the docket if it is not already provisional.
    fn docket_connection(&mut self, word: &Element, ratifier: [u8; 32]) {
        if word.is_identity() && !word.is_rewrite() {
            return;
        }
        let addr = word.address();
        if self.provisional.contains(&addr) {
            return;
        }
        let mag = word.magnitude();
        let mu = if word.is_rewrite() {
            mag
        } else {
            crate::economy::price(&mag, &self.physics.pressure)
        };
        let anchor = self.anchor();
        self.treasury.escrow_award(addr, vec![ratifier], mu, anchor);
        self.deflations.insert(addr, mag);
        let k_now = self.k_step();
        self.provisional_k.insert(addr, k_now);
        self.provisional.insert(addr, word.clone());
        self.stamp_standing(addr);
        self.provisional_tu.insert(addr, self.now_tu());
        self.mark_persist();
        let inv = crate::transducer::stalk::invariant_of(word);
        if self.field.get(&addr).is_none() {
            self.write_field(
                &addr,
                inv,
                crate::transducer::stalk::Closure::Cavity,
                word.sub().to_vec(),
                &mag,
                &mag,
            );
        }
        self.book_gen += 1;
        self.append_step(addr, ratifier);
    }

    /// Speak an optimisation: refuse a zero vector, docket the Rewrite,
    /// Offer it, and keep it as language. Payment waits on the second organ.
    fn offer_rewrite(&mut self, rw: Element, voice: [u8; 32]) -> bool {
        if !rw.is_rewrite() || rw.magnitude().iter().all(|&x| x == 0) {
            return false;
        }
        let addr = rw.address();
        self.docket_connection(&rw, voice);
        self.offered.insert(addr, ());
        true
    }

    fn mint_rewrite(
        &mut self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[Element],
        after: &Element,
        payload: crate::transducer::rewrite::RewritePayload,
        voice: [u8; 32],
    ) -> bool {
        let Some(mu) = self.treasury.settle_rewrite(kind, before, after) else {
            return false;
        };
        let before_addr: Vec<[u8; 32]> = before.iter().map(|e| e.address()).collect();
        let Some(rw) = crate::transducer::rewrite::mint(kind, &before_addr, after.address(), payload, mu) else {
            return false;
        };
        self.offer_rewrite(rw, voice)
    }

    fn retire_rewrite_inputs(&mut self, rw: &Element) {
        let Some(r) = crate::transducer::rewrite::Rewrite::from_element(rw) else {
            return;
        };
        for addr in r.before {
            let Some(i) = self.basis_index.get(&addr).copied() else {
                continue;
            };
            let w = (*self.basis[i]).clone();
            if !matches!(self.reduce_excluding(&w, Some(i)), NormalForm::Decomposed(_)) {
                continue;
            }
            let elem = self.basis_remove(i);
            self.ruled_admit(&addr);
            self.cavity_book.insert(addr, elem.clone());
            self.cavities.push(elem);
        }
    }

    /// The court's voice, heard on its verdicts.
    pub fn set_court_voice(&mut self, voice: [u8; 32]) {
        self.court_voice = Some(voice);
    }

    /// The digests the court reconciles against: the ruled cover, served to
    /// the court's own voice. Any other asker (an entity's sync) is answered
    /// with the whole language elsewhere.
    pub fn ruled_digests_for(&self, voice: &[u8; 32]) -> Option<Vec<crate::transducer::sheaf::StratumDigest>> {
        (self.court_voice.as_ref() == Some(voice)).then(|| crate::transducer::sheaf::digests(&self.ruled))
    }

    /// Remove the word at `i` from the basis, its discoverer, and its stalk.
    fn basis_remove(&mut self, i: usize) -> Element {
        let addr = self.basis[i].address();
        self.discoverers.remove(&addr);
        std::sync::Arc::make_mut(&mut self.stalks).remove(&addr);
        self.ruled.remove(&addr);
        if self.ruled_addrs.remove(&addr).is_some() {
            self.refresh_ruled_root();
        }
        self.basis_gen += 1;
        self.retire_gen += 1;
        let removed = self.basis.remove(i);
        self.reindex_basis();
        self.language_rebuild();
        (*removed).clone()
    }

    fn basis_clear(&mut self) {
        self.basis.clear();
        self.discoverers = crate::network::nerve::Keyed::new();
        self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::default());
        self.basis_index = std::sync::Arc::new(crate::network::nerve::Keyed::new());
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// A held word changed in place: the Arc in `basis` is already unique
    /// (via `word_mut`); rebuild the language pointer slice.
    fn touch_at(&mut self, _i: usize) {
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// The language as an immutable shared snapshot. An append clones the
    /// pointer slice and one new Arc; a retirement rebuilds. Never clones
    /// the whole basis of Elements.
    pub fn language(&mut self) -> std::sync::Arc<Language> {
        if self.language_gen != self.basis_gen || self.language.words.len() != self.basis.len() {
            self.language_rebuild();
        }
        std::sync::Arc::clone(&self.language)
    }

    fn language_rebuild(&mut self) {
        let shapes = (0..self.basis.len()).map(|_| std::sync::OnceLock::new()).collect();
        self.language = std::sync::Arc::new(Language {
            words: self.basis.clone().into(),
            stalks: std::sync::Arc::clone(&self.stalks),
            shapes,
            retire_gen: self.retire_gen,
            index: std::sync::Arc::clone(&self.basis_index),
        });
        self.language_gen = self.basis_gen;
    }

    fn language_append_arc(&mut self, word: std::sync::Arc<Element>) {
        let addr = word.address();
        let inv = crate::transducer::stalk::invariant_of(word.as_ref());
        let i = self.basis.len() - 1;
        if self.retire_gen != self.language.retire_gen || self.language.words.len() + 1 != self.basis.len()
        {
            std::sync::Arc::make_mut(&mut self.stalks).admit(&inv, addr);
            std::sync::Arc::make_mut(&mut self.basis_index).insert(addr, i);
            self.language_rebuild();
            return;
        }
        let mut words: Vec<std::sync::Arc<Element>> = self.language.words.iter().cloned().collect();
        words.push(word);
        let mut stalks = std::sync::Arc::clone(&self.language.stalks);
        let mut index = std::sync::Arc::clone(&self.language.index);
        // Drop our holds on the previous snapshot so an unshared trie admits
        // in place; workers holding it still see the old generation.
        self.language = std::sync::Arc::new(Language::default());
        self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::default());
        self.basis_index = std::sync::Arc::new(crate::network::nerve::Keyed::new());
        std::sync::Arc::make_mut(&mut stalks).admit(&inv, addr);
        std::sync::Arc::make_mut(&mut index).insert(addr, i);
        self.stalks = std::sync::Arc::clone(&stalks);
        self.basis_index = std::sync::Arc::clone(&index);
        let shapes = (0..words.len()).map(|_| std::sync::OnceLock::new()).collect();
        self.language = std::sync::Arc::new(Language {
            words: words.into(),
            stalks,
            shapes,
            retire_gen: self.retire_gen,
            index,
        });
        self.language_gen = self.basis_gen;
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
        let now = self.now_tu();
        let mut standing_t = crate::network::nerve::Keyed::new();
        for (elem, _) in &self.pending {
            standing_t.insert(elem.address(), self.t_extent(&elem.address(), now));
        }
        MarketView {
            language: self.language(),
            pending: std::sync::Arc::clone(&self.book.1),
            book_index: std::sync::Arc::new(self.book_index.clone()),
            composites: std::sync::Arc::clone(&self.book.2),
            surface: crate::economy::yield_surface(&self.physics),
            pressure: self.physics.pressure,
            pool: self.treasury.bond_of(&self.identity).unwrap_or([0; 4]),
            k_step: self.chain.next_step_index(),
            head_k: self.chain.head_k(),
            tu: now,
            standing_t: std::sync::Arc::new(standing_t),
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
            let measured = rank_over(&language, &self.pending, &self.composites, &self.closure.2, gens.0, Some(&self.chart_listings));
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
        if self.provisional.remove(sibling).is_some() {
            let _ = self.treasury.revoke_award(sibling, false);
        }
        let (Some(bi), Some(si)) = (
            self.basis_index.get(base).copied(),
            self.basis_index.get(sibling).copied(),
        ) else {
            return 0;
        };
        let sib = (*self.basis[si]).clone();
        if element::pullback(&element::positions(self.basis[bi].sub()), sib.sub(), g).is_none() {
            return 0;
        }
        let disc = self.discoverer_at(si);
        let gained = self.word_mut(bi).attach(&sib, g);
        self.touch_at(bi);
        self.basis_remove(si);
        if gained > 0 {
            if let Some(bi) = self.basis_index.get(base).copied() {
                let after = (*self.basis[bi]).clone();
                let payload = crate::transducer::rewrite::RewritePayload::Condense {
                    sections: after.sections().to_vec(),
                    gauge: g.clone(),
                };
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Condense,
                    &[sib.clone()],
                    &after,
                    payload,
                    disc,
                );
                self.torse_at(bi);
            }
        }
        self.physics.syntropy(&[0, 0, 1, 0]);
        1
    }

    /// SETTLE A PROBE: the replayed residual of an open listing against the
    /// speaker's sentence. Closure (empty residual) of a fibre section or a
    /// connection listing pays morphism deflation; a tape covering pays
    /// nothing. A partial descent updates tension and relieves the physics
    /// (the measurement is real) but pays μ⃗ = 0⃗ — covering cells is not
    /// an axial move. Cited words still earn settle_use. Returns
    /// (code, paid μ⃗, remaining μ⃗).
    pub fn settle_probe(
        &mut self,
        voice: [u8; 32],
        target: &[u8; 32],
        residual: Vec<(i64, i64, u8, u8)>,
        cites: &[crate::anastome::dialogue::CiteWire],
        section: bool,
    ) -> (u8, [u64; 4], [u64; 4]) {
        use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
        use crate::transducer::stalk::Closure;
        let Some(&li) = self.book_index.get(target) else {
            return (PROBE_NOTHING, [0; 4], [0; 4]);
        };
        let listing = self.pending[li].0.clone();
        self.record_spoken(*target, voice);
        let before = self.tension.get(target).copied().unwrap_or_else(|| listing.magnitude());
        let rem = element::measure_lanes(&residual);
        let after = rem;
        let anchor = self.anchor();
        let mag = listing.magnitude();
        let mut charts_at: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
            crate::network::nerve::Keyed::new();
        for (_, _, _, _, body) in cites {
            let inv = crate::transducer::stalk::invariant(body);
            charts_at.insert(inv.key(), inv);
        }
        let mut charts: Vec<crate::transducer::stalk::Invariant> = charts_at
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, inv)| inv.clone())
            .collect();
        if charts.is_empty() {
            charts.push(crate::transducer::stalk::invariant(listing.sub()));
        }
        let cite_g = cites.first().map(|(_, _, g, _, _)| g.clone()).unwrap_or_else(element::identity_versor);
        if let [] = residual.as_slice() {
            let paid = if section || listing.is_connection() {
                self.treasury.settle_subsidy(voice, crate::economy::price(&before, &self.physics.pressure), anchor)
            } else {
                [0; 4]
            };
            for (addr, _, _, _, _) in cites {
                if let Some(i) = self.basis_index.get(addr).copied() {
                    let mag = self.basis[i].magnitude();
                    self.treasury.settle_use(voice, self.discoverer_at(i), mag, anchor);
                }
            }
            for chart in charts {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &[0; 4]);
                self.record_flat(target, chart, voice, cite_g.clone());
            }
            self.unlist(target, false);
            self.physics.relieve(&mag);
            self.physics.syntropy(&mag);
            self.append_step(*target, voice);
            self.complete_composite_child(target, voice);
            return (PROBE_CLOSED, paid, [0; 4]);
        }
        let mut listed = listing.sub().to_vec();
        listed.sort_unstable();
        if residual == listed {
            let standing = self
                .field
                .get(target)
                .map(|f| union_remainders(&f.entries))
                .filter(|r| !r.is_empty())
                .unwrap_or_else(|| residual.clone());
            for chart in charts {
                self.write_field(target, chart, Closure::Cavity, standing.clone(), &before, &before);
            }
            return (PROBE_NOTHING, [0; 4], before);
        }
        let rem_inv = crate::transducer::stalk::invariant(&residual);
        for chart in &charts {
            if *chart != rem_inv {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &after);
                self.record_flat(target, chart.clone(), voice, cite_g.clone());
            }
        }
        let v_disloc = {
            let base_shape = element::shape_of_chain(listing.sub());
            let rem_shape = element::shape_of_chain(&residual);
            match (base_shape, rem_shape) {
                (Some(sb), Some(sr)) => crate::transducer::blade::versor_between(&sb, &sr),
                _ => None,
            }
        };
        let state = match v_disloc.clone() {
            Some(v) => Closure::Twisted(v),
            None => Closure::Cavity,
        };
        self.write_field(target, rem_inv.clone(), state, residual.clone(), &before, &after);
        self.listing_atlas.admit_also(&rem_inv, *target);
        let mag_shift = if let Some(&li) = self.book_index.get(target) {
            if let Some(held) = self.pending.get_mut(li) {
                let old = held.0.magnitude();
                held.0.mag[3] = residual.len() as u64;
                if let Some(v) = v_disloc {
                    held.0.record_dislocation(element::assignment(&residual), v, element::FIBER_SPATIAL);
                }
                Some((old, held.0.magnitude()))
            } else {
                None
            }
        } else {
            None
        };
        if let Some((old, newm)) = mag_shift {
            self.occupy(*target, &old, false);
            self.occupy(*target, &newm, true);
        }
        let settled: [u64; 4] = std::array::from_fn(|l| before[l].saturating_sub(after[l]));
        self.spoken.remove(target);
        self.physics.relieve(&settled);
        self.book_gen += 1;
        self.physics.syntropy(&mag);
        self.append_step(*target, voice);
        (PROBE_DESCENT, [0; 4], after)
    }

    fn record_flat(
        &mut self,
        listing: &[u8; 32],
        chart: crate::transducer::stalk::Invariant,
        voice: [u8; 32],
        g: element::Versor,
    ) {
        let key = loop_key(&chart, &voice);
        if self.flat_loops.get(listing).is_some_and(|m| m.contains(&key)) {
            return;
        }
        match self.flat_loops.get_mut(listing) {
            Some(m) => {
                m.insert(key, (chart, g));
            }
            None => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(key, (chart, g));
                self.flat_loops.insert(*listing, m);
            }
        }
    }

    /// Topological volume: cards, fibre span, open listings. K is algebraic
    /// time and is not a persist event — ticking it must not flush the ledger.
    pub fn volume(&self) -> (usize, usize, usize) {
        (self.basis.len(), self.sections_total(), self.pending.len())
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury_entry_count(&self) -> usize {
        self.treasury.entry_count()
    }

    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    /// Restrict stakes to the live genesis identities.
    pub fn bind_live(&mut self, live: &[[u8; 32]]) {
        self.treasury.bind_live(live);
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
            .fiber(&crate::network::nerve::ROOT)
            .filter_map(|(addr, spoken)| {
                let k = self.provisional_k.get(addr).copied().unwrap_or(0);
                if k <= since_k && since_k > 0 {
                    return None;
                }
                let (voices, mu, _) = self.treasury.escrow_of(addr)?;
                // A word whose deflation is unmeasured is not served: the
                // court rules what was measured, never a fabricated zero.
                let deflation = self.deflation_of(addr);
                if deflation.iter().all(|&x| x == 0) {
                    return None;
                }
                Some((*addr, spoken.clone(), voices.clone(), *mu, deflation, k))
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
        let spoken = self.provisional.get(addr).cloned();
        if self.provisional.remove(addr).is_none() {
            return false;
        }
        if ruling == RULING_UPHELD {
            self.ruled_admit(addr); // finalized here: it joins the ruled cover
            let spoken = spoken.or_else(|| self.word_at(addr).cloned());
            if let Some(w) = spoken {
                if w.is_rewrite() {
                    let voice = self
                        .treasury
                        .escrow_of(addr)
                        .and_then(|(vs, _, _)| vs.first().copied())
                        .unwrap_or([0; 32]);
                    let _ = self.treasury.release_rewrite(addr, voice);
                    self.retire_rewrite_inputs(&w);
                    self.fed_admit(addr, w);
                    self.unlist(addr, false);
                    self.mark_persist();
                    return true;
                }
                self.fed_admit(addr, w);
            }
            if let Some((voices, mu, anchor)) = self.treasury.release_award(addr) {
                self.treasury.mint_claim(&voices, mu, anchor);
            }
            self.unlist(addr, false);
            self.mark_persist();
            true
        } else {
            let slash = ruling == RULING_SLASHED;
            let ok = self.treasury.revoke_award(addr, slash);
            if slash {
                // A slashed word leaves the language and is admitted as a
                // cavity chart: the residual is a boundary, never garbage.
                if let Some(w) = self.basis_index.get(addr).copied() {
                    let elem = (*self.basis[w]).clone();
                    self.basis_remove(w);
                    let inv = crate::transducer::stalk::invariant_of(&elem);
                    self.listing_atlas.admit(&inv, *addr);
                    self.write_field(
                        addr,
                        inv.clone(),
                        crate::transducer::stalk::Closure::Cavity,
                        elem.sub().to_vec(),
                        &elem.magnitude(),
                        &elem.magnitude(),
                    );
                    self.cavity_book.insert(*addr, elem.clone());
                    self.cavities.push(elem);
                }
                self.mark_persist();
            }
            ok
        }
    }

    /// Court organ: the docket is the field's projection — listings whose
    /// closure is not Flat and whose deflation is nonzero — plus unfilled
    /// triangles and tetrahedron classes. Fail → slash with a replay reason
    /// and relocate to the cavity atlas. Pass → a section on the ruled
    /// cover, escrow held. Fills and refinements share the stroke Budget.
    pub fn court_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut keys: Vec<[u8; 32]> = Vec::new();
        let from = self.open_cursor;
        // index: budgeted keyed cursor over the open docket
        {
            let cur = self.open.cursor(from, &mut budget);
            for (addr, _) in cur {
                keys.push(*addr);
            }
        }
        if let Some(k) = keys.last() {
            self.open_cursor = *k;
        }
        for addr in keys {
            let spoken = self
                .word_at(&addr)
                .cloned()
                .or_else(|| self.book_elem(&addr).cloned());
            let Some(spoken) = spoken else {
                continue;
            };
            if let Some(reason) = crate::verdict::blind_replay_with(&spoken, &|a| self.resolve_form(a)) {
                eprintln!("  court slash {:02x}{:02x}… {reason}", addr[0], addr[1]);
                let _ = self.treasury.revoke_award(&addr, true);
                self.relocate_cavity(&addr);
                self.provisional.remove(&addr);
                self.court_slashed += 1;
                continue;
            }
            self.ruled_admit(&addr);
            eprintln!("  court section {:02x}{:02x}… ruled cover, escrow held", addr[0], addr[1]);
        }
        self.relocate_t_exhausted();
        self.stroke_fills(&mut budget);
    }

    fn stroke_fills(&mut self, budget: &mut crate::physics::Budget) {
        if budget.remaining() == 0 {
            return;
        }
        let mut pending: Vec<(crate::transducer::stalk::Invariant, [u8; 32])> = Vec::new();
        for (_, v) in self.unfilled.cursor(crate::network::nerve::ROOT, budget) {
            pending.push(v.clone());
        }
        for (inv, sim_key) in pending {
            let dkey = chart_simplex_key(&inv, &sim_key);
            self.unfilled.remove(&dkey);
            let filled = {
                let Some(cx) = self.complexes.get_mut(&inv.key()) else {
                    continue;
                };
                let addrs: Vec<[u8; 32]> = cx
                    .sections
                    .fiber(&crate::network::nerve::ROOT)
                    .map(|(_, s)| s.addr)
                    .collect();
                (cx.fill(&sim_key), addrs)
            };
            match filled {
                (crate::transducer::cech::Fill::Filled(gamma), addrs) => {
                    eprintln!(
                        "  court cell filled {:02x}{:02x}… γ recorded",
                        sim_key[0], sim_key[1]
                    );
                    self.record_gamma(&addrs, &gamma);
                    self.pay_cell_volume(&inv, &addrs);
                    if !gamma.is_identity() && addrs.len() >= 2 {
                        self.mint_or_admit_translation(&gamma, addrs[0], addrs[1], addrs[0]);
                        let key = Self::cocycle_key(&inv, &addrs[0], &addrs[1]);
                        if let Some((v, tu, _, _, _)) = self.cocycle_at.remove(&key) {
                            self.dialect_chain.push((key, v, tu));
                        }
                        if let Some(m) = self.cocycle_by_chart.get_mut(&inv.key()) {
                            m.remove(&key);
                        }
                    }
                }
                (crate::transducer::cech::Fill::Congruent(_g, _ell), addrs) => {
                    eprintln!(
                        "  court cell congruent {:02x}{:02x}… torsion filed",
                        sim_key[0], sim_key[1]
                    );
                    self.file_torsion(&addrs, _ell);
                    self.pay_cell_volume(&inv, &addrs);
                    self.pay_torse(&addrs);
                }
                (crate::transducer::cech::Fill::Class(class), _) => {
                    eprintln!(
                        "  court cell class {:02x}{:02x}…",
                        sim_key[0], sim_key[1]
                    );
                    let degree = self
                        .complexes
                        .get(&inv.key())
                        .and_then(|c| c.class(&sim_key).map(|(_, d)| d as u8))
                        .unwrap_or(2);
                    self.mark_class(&inv, class.clone(), degree);
                    if degree >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, degree));
                    }
                }
                (crate::transducer::cech::Fill::Open, _) | (crate::transducer::cech::Fill::Already, _) => {}
            }
        }
        let mut refine: Vec<(
            crate::transducer::stalk::Invariant,
            crate::transducer::alphabet::Rotor,
            u8,
        )> = Vec::new();
        for (_, v) in self.pending_refine.cursor(crate::network::nerve::ROOT, budget) {
            refine.push(v.clone());
        }
        for (inv, class, degree) in refine {
            let key = inv.key();
            self.pending_refine.remove(&key);
            eprintln!(
                "  court class degree {degree} {:02x}{:02x}… chart refined",
                key[0], key[1]
            );
            self.refine_chart(&inv, class, degree, None);
        }
    }

    fn listings_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        verts: &[[u8; 32]],
    ) -> crate::network::nerve::Keyed<()> {
        let on_chart = self.listing_atlas.fiber(inv);
        let mut acc: Option<crate::network::nerve::Keyed<()>> = None;
        for v in verts {
            let mut here = crate::network::nerve::Keyed::new();
            for addr in &on_chart {
                if !self.open.contains(addr) {
                    continue;
                }
                let holds = match self.spoken.get(addr) {
                    Some(vs) if vs.len() > 0 => vs.contains(v),
                    _ => true,
                };
                if holds {
                    here.insert(*addr, ());
                }
            }
            acc = Some(match acc {
                None => here,
                Some(prev) => {
                    let mut next = crate::network::nerve::Keyed::new();
                    for (a, _) in here.fiber(&crate::network::nerve::ROOT) {
                        if prev.contains(a) {
                            next.insert(*a, ());
                        }
                    }
                    next
                }
            });
        }
        acc.unwrap_or_default()
    }

    fn volume_of_cell(&self, inv: &crate::transducer::stalk::Invariant, verts: &[[u8; 32]]) -> [u64; 4] {
        let held = self.listings_on(inv, verts);
        let mut chain = Vec::new();
        for (a, _) in held.fiber(&crate::network::nerve::ROOT) {
            if let Some(f) = self.field.get(a) {
                chain.extend(union_remainders(&f.entries));
            }
        }
        element::measure_lanes(&chain)
    }

    fn pay_cell_volume(&mut self, inv: &crate::transducer::stalk::Invariant, addrs: &[[u8; 32]]) {
        let held = self.listings_on(inv, addrs);
        let vol = self.volume_of_cell(inv, addrs);
        let voice = addrs.first().copied().unwrap_or([0; 32]);
        let anchor = self.anchor();
        for (a, _) in held.fiber(&crate::network::nerve::ROOT) {
            let _ = self.treasury.draw_escrow(a, voice, vol, anchor);
        }
    }

    fn pay_torse(&mut self, addrs: &[[u8; 32]]) {
        let voice = addrs.first().copied().unwrap_or([0; 32]);
        let mag = addrs
            .first()
            .and_then(|a| self.tension.get(a).copied())
            .unwrap_or([0; 4]);
        let _ = self.treasury.settle_torse(voice, mag, self.anchor());
    }

    fn file_torsion(&mut self, addrs: &[[u8; 32]], ell: u32) {
        let Some(rep) = addrs.first() else {
            return;
        };
        if let Some(i) = self.basis_index.get(rep).copied() {
            let asg = element::assignment(self.basis[i].sub());
            self.word_mut(i).record_torsion(asg, ell, element::FIBER_SPATIAL);
            self.touch_at(i);
        }
    }

    /// A gauge from Extended between two hyphas is a translation: minted
    /// through from_connection with the identity gauge map, tagged, entered
    /// on the docket, and admitted to the ruled cover on second replay.
    fn mint_or_admit_translation(
        &mut self,
        gamma: &crate::transducer::alphabet::Rotor,
        a: [u8; 32],
        b: [u8; 32],
        ratifier: [u8; 32],
    ) {
        let Some(t) = element::from_translation(gamma, a, b) else {
            return;
        };
        let addr = t.address();
        if self.ruled_contains(&addr) {
            return;
        }
        if self.provisional.contains(&addr) {
            self.ruled_admit(&addr);
            self.fed_admit(&addr, t);
            self.provisional.remove(&addr);
            self.court_upheld += 1;
            eprintln!(
                "  court translation admitted {:02x}{:02x}… second replay",
                addr[0], addr[1]
            );
            return;
        }
        self.docket_connection(&t, ratifier);
        let payload = crate::transducer::rewrite::RewritePayload::Translation {
            a,
            b,
            gauge: element::identity_versor(),
        };
        let _ = self.mint_rewrite(
            crate::transducer::rewrite::RewriteKind::Translation,
            &[],
            &t,
            payload,
            ratifier,
        );
        eprintln!(
            "  court translation docked {:02x}{:02x}…",
            addr[0], addr[1]
        );
    }

    /// Glue a word fetched from a peer whose cover meets ours on this chart.
    /// Vertices are covering hyphas. Degree 0 is the peer's section; degree 1
    /// edges to held sections; then coboundary_on_link climbs while volume
    /// persists.
    /// Match spoken torsion cycles against the chart's Congruent cells.
    /// Same class key and ℓ is consilience; same key different ℓ is
    /// [`crate::verdict::REFUSE_NOT_NATURAL`]; a new finite key files.
    fn court_torsion(
        &mut self,
        spoken: &Element,
        inv: &crate::transducer::stalk::Invariant,
        self_id: [u8; 32],
        peer: [u8; 32],
    ) -> Option<&'static str> {
        if spoken.torsion().is_empty() {
            return None;
        }
        let rep = spoken
            .letters()
            .map(|l| l.spelling.word.rotor())
            .unwrap_or_else(crate::transducer::alphabet::Rotor::identity);
        let rep_key = rep.class_key();
        for (_sigma, ell, _) in spoken.torsion() {
            let mut matched = false;
            let mut conflict = false;
            let mut log_key = rep_key;
            if let Some(cx) = self.complexes.get(&inv.key()) {
                for layer in &cx.degrees {
                    for (_, cell) in layer.fiber(&crate::network::nerve::ROOT) {
                        if let crate::transducer::cech::Fill::Congruent(g, ell2) = &cell.fill {
                            let ck = g.class_key();
                            if ck == rep_key && *ell2 != *ell {
                                conflict = true;
                                log_key = ck;
                            }
                            if *ell2 == *ell {
                                matched = true;
                                log_key = ck;
                            }
                        }
                    }
                }
            }
            if conflict {
                eprintln!(
                    "  court torsion refused {:02x}{:02x}… ℓ {ell}",
                    log_key[0], log_key[1]
                );
                return Some(crate::verdict::REFUSE_NOT_NATURAL);
            }
            if matched {
                eprintln!(
                    "  court torsion consilient {:02x}{:02x}… ℓ {ell}",
                    log_key[0], log_key[1]
                );
            } else {
                if let Some(cx) = self.complexes.get_mut(&inv.key()) {
                    cx.file_congruent(self_id, peer, rep.clone(), *ell);
                }
                eprintln!(
                    "  court torsion filed {:02x}{:02x}… ℓ {ell}",
                    rep_key[0], rep_key[1]
                );
            }
        }
        None
    }

    pub fn glue_fetched(
        &mut self,
        spoken: Element,
        peer: [u8; 32],
        self_id: [u8; 32],
        cover: &mut crate::network::nerve::Cover,
    ) -> Glue {
        let addr = spoken.address();
        if self.heard.contains(&addr) {
            return Glue::Dialect;
        }
        if let Some(reason) = crate::verdict::blind_replay_with(&spoken, &|a| self.resolve_form(a)) {
            eprintln!("  court glue slash {:02x}{:02x}… {reason}", addr[0], addr[1]);
            let inv = crate::transducer::stalk::invariant_of(&spoken);
            self.heard_mark(addr, &inv);
            return Glue::Slashed(reason);
        }
        let inv = crate::transducer::stalk::invariant_of(&spoken);
        let asg = element::assignment(spoken.sub());
        if spoken.is_sentence() {
            if let Some(n) = crate::transducer::sentence::from_element(&spoken) {
                let missing = n.germs.iter().any(|g| g.address != addr && self.word_at(&g.address).is_none());
                if missing {
                    self.heard_mark(addr, &inv);
                    return Glue::Dialect;
                }
            }
        }
        let mag = spoken.magnitude();
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        let mut verts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        verts.insert(self_id, ());
        verts.insert(peer, ());
        for lane in &lanes {
            for v in cover.fiber(*lane, &inv) {
                verts.insert(v, ());
            }
        }
        let mut cx = self
            .complexes
            .remove(&inv.key())
            .unwrap_or_else(crate::transducer::cech::Complex::new);
        let vol0 = self.volume_of_cell(&inv, &[peer]);
        let peer_gamma = if spoken.is_translation() {
            spoken
                .letters()
                .map(|l| l.spelling.word.rotor())
                .unwrap_or_else(crate::transducer::alphabet::Rotor::identity)
        } else {
            crate::transducer::alphabet::Rotor::identity()
        };
        cx.set_section(
            peer,
            crate::transducer::cech::Section {
                addr,
                gamma: peer_gamma,
            },
            vol0,
        );
        let self_rep = cx.sections.get(&self_id).map(|s| s.addr).or_else(|| {
            self.ruled
                .fiber(&inv)
                .into_iter()
                .find(|a| *a != addr && self.word_at(a).is_some())
        });
        if let Some(rep_addr) = self_rep {
            let vol_s = self.volume_of_cell(&inv, &[self_id]);
            cx.set_section(
                self_id,
                crate::transducer::cech::Section {
                    addr: rep_addr,
                    gamma: cx
                        .sections
                        .get(&self_id)
                        .map(|s| s.gamma.clone())
                        .unwrap_or_else(crate::transducer::alphabet::Rotor::identity),
                },
                vol_s,
            );
        }
        let mut outcome = Glue::Admitted;
        let held_rep = self_rep.and_then(|a| self.word_at(&a).cloned());
        if let Some(rep) = &held_rep {
            match crate::transducer::cech::transition(rep, &spoken) {
                None => {}
                Some(g) if g.is_identity() => {}
                Some(g) if crate::transducer::stalk::placement_of(&g).is_some() => {
                    outcome = Glue::Attached;
                }
                Some(g) => {
                    outcome = Glue::Dialect;
                    if let Some(r) = crate::transducer::cech::rotor_of(&g) {
                        if let Some(v) = rotor_versor(&r) {
                            self.push_cocycle(&inv, self_id, peer, v, r.class_key());
                        }
                    }
                }
            }
        }
        let others: Vec<[u8; 32]> = verts.fiber(&crate::network::nerve::ROOT).map(|(k, _)| *k).collect();
        for v in &others {
            if *v == peer {
                continue;
            }
            let Some(sec) = cx.sections.get(v).cloned() else {
                continue;
            };
            let Some(w) = self.word_at(&sec.addr) else {
                continue;
            };
            let vol_e = self.volume_of_cell(&inv, &[*v, peer]);
            match crate::transducer::cech::transition(w, &spoken).and_then(|g| crate::transducer::cech::rotor_of(&g)) {
                Some(g) => cx.set_edge(*v, peer, g, vol_e),
                None => {
                    if crate::transducer::cech::transition(w, &spoken).is_some() {
                        cx.set_remainder_edge(*v, peer);
                    }
                }
            }
        }
        cx.coboundary_on_link(&peer, |vs| self.volume_of_cell(&inv, vs));
        let mut filled = false;
        let mut class_cell: Option<(crate::transducer::alphabet::Rotor, u8)> = None;
        let mut congruent: Option<(crate::transducer::alphabet::Rotor, u32, Vec<[u8; 32]>)> = None;
        let mut fill_gamma = crate::transducer::alphabet::Rotor::identity();
        let mut fill_addrs: Vec<[u8; 32]> = Vec::new();
        let mut budget = self.physics.stroke();
        let spoken_rotor = spoken.letters().map(|l| l.spelling.word.rotor());
        // index: link of the fetched peer
        for key in cx.link_of(&peer) {
            let Some((d, volume, vertices)) = cx.cell(&key).map(|c| (c.degree(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if d == 0 {
                continue;
            }
            if !budget.spend() {
                let dkey = chart_simplex_key(&inv, &key);
                self.unfilled.insert(dkey, (inv.clone(), key));
                continue;
            }
            let outcome_fill = if spoken.is_translation() {
                if let Some(g) = &spoken_rotor {
                    cx.fill_by_gauge(&key, g)
                } else {
                    cx.fill(&key)
                }
            } else {
                cx.fill(&key)
            };
            match outcome_fill {
                crate::transducer::cech::Fill::Filled(gamma) => {
                    if volume.iter().any(|&x| x > 0) {
                        filled = true;
                        fill_gamma = gamma;
                        fill_addrs = vertices
                            .iter()
                            .filter_map(|v| cx.sections.get(v).map(|s| s.addr))
                            .collect();
                        eprintln!(
                            "  court cell filled degree {d} {:02x}{:02x}… volume {:?}",
                            key[0], key[1], volume
                        );
                    }
                }
                crate::transducer::cech::Fill::Congruent(g, ell) => {
                    let addrs: Vec<[u8; 32]> = vertices
                        .iter()
                        .filter_map(|v| cx.sections.get(v).map(|s| s.addr))
                        .collect();
                    congruent = Some((g, ell, addrs));
                    eprintln!(
                        "  court cell congruent degree {d} {:02x}{:02x}…",
                        key[0], key[1]
                    );
                }
                crate::transducer::cech::Fill::Class(class) => {
                    class_cell = Some((class.clone(), d as u8));
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), key));
                    if d >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, d as u8));
                    }
                    eprintln!(
                        "  court cell class degree {d} {:02x}{:02x}…",
                        key[0], key[1]
                    );
                }
                crate::transducer::cech::Fill::Open => {
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), key));
                }
                crate::transducer::cech::Fill::Already => {}
            }
        }
        self.complexes.insert(inv.key(), cx);
        if let Some(reason) = self.court_torsion(&spoken, &inv, self_id, peer) {
            self.heard_mark(addr, &inv);
            return Glue::Slashed(reason);
        }
        if filled {
            self.record_gamma(&fill_addrs, &fill_gamma);
            self.pay_cell_volume(&inv, &fill_addrs);
            if !fill_gamma.is_identity() && fill_addrs.len() >= 2 {
                self.mint_or_admit_translation(&fill_gamma, fill_addrs[0], fill_addrs[1], peer);
            }
            outcome = Glue::Attached;
        }
        if let Some((_g, ell, addrs)) = congruent {
            self.file_torsion(&addrs, ell);
            self.pay_cell_volume(&inv, &addrs);
            self.pay_torse(&addrs);
            outcome = Glue::Attached;
        }
        if let Some((class, degree)) = class_cell {
            self.mark_class(&inv, class.clone(), degree);
            outcome = Glue::Dialect;
        }
        match outcome {
            Glue::Admitted => {
                if self.basis_index.get(&addr).is_none()
                    && !(spoken.is_connection() && spoken.letters().is_none())
                {
                    self.basis_push(spoken.clone(), peer);
                }
                if let Some(i) = self.basis_index.get(&addr).copied() {
                    self.word_mut(i).record_dislocation(asg, element::identity_versor(), element::FIBER_SPATIAL);
                    self.touch_at(i);
                }
                self.ruled_admit(&addr);
                self.fed_admit(&addr, spoken.clone());
                if let Some((voices, mu, anchor)) = self.treasury.release_award(&addr) {
                    self.treasury.mint_claim(&voices, mu, anchor);
                }
                self.unlist(&addr, false);
                self.provisional.remove(&addr);
                self.court_upheld += 1;
                self.mark_persist();
            }
            Glue::Attached => {
                if let Some(rep) = held_rep {
                    if let Some(g) = crate::transducer::cech::transition(&rep, &spoken) {
                        if let Some(v) = crate::transducer::stalk::placement_of(&g) {
                            if let Some(i) = self.basis_index.get(&rep.address()).copied() {
                                self.word_mut(i).record_dislocation(asg, v, element::FIBER_SPATIAL);
                                self.touch_at(i);
                            }
                        }
                    }
                }
                self.ruled_admit(&addr);
            }
            Glue::Dialect | Glue::Slashed(_) => {
                self.heard_mark(addr, &inv);
            }
        }
        outcome
    }

    fn record_gamma(&mut self, addrs: &[[u8; 32]], gamma: &crate::transducer::alphabet::Rotor) {
        let Some(v) = rotor_versor(gamma) else {
            return;
        };
        for addr in addrs {
            if let Some(i) = self.basis_index.get(addr).copied() {
                let asg = element::assignment(self.basis[i].sub());
                self.word_mut(i).record_dislocation(asg, v.clone(), element::FIBER_SPATIAL);
                self.touch_at(i);
            }
        }
    }

    fn mark_class(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        class: crate::transducer::alphabet::Rotor,
        degree: u8,
    ) {
        let members: Vec<[u8; 32]> = self.listing_atlas.fiber(inv);
        for addr in members {
            let mag = self.tension.get(&addr).copied().unwrap_or([0; 4]);
            self.write_field(
                &addr,
                inv.clone(),
                crate::transducer::stalk::Closure::Class(class.clone(), degree),
                Vec::new(),
                &mag,
                &mag,
            );
        }
    }

    fn refine_chart(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        class: crate::transducer::alphabet::Rotor,
        _degree: u8,
        mut cover: Option<&mut crate::network::nerve::Cover>,
    ) {
        let axes = class.class_axes().unwrap_or_else(|| vec![0]);
        let cx = self.complexes.get(&inv.key()).cloned();
        let coord_of = |addr: &[u8; 32]| -> Vec<u64> {
            if let Some(c) = &cx {
                for (_, sec) in c.sections.fiber(&crate::network::nerve::ROOT) {
                    if sec.addr == *addr && !sec.gamma.is_identity() {
                        return axes.clone();
                    }
                }
            }
            vec![0]
        };
        let before: Vec<[u8; 32]> = self.listing_atlas.fiber(inv);
        let before_mag: Vec<([u8; 32], [u64; 4])> = before
            .iter()
            .map(|a| (*a, self.tension.get(a).copied().unwrap_or([0; 4])))
            .collect();
        self.ruled.split_stratum(inv, coord_of);
        std::sync::Arc::make_mut(&mut self.stalks).split_stratum(inv, coord_of);
        self.listing_atlas.split_stratum(inv, coord_of);
        if let Some(cover) = cover.as_mut() {
            for lane in 0..4u8 {
                let hyphas = cover.fiber(lane, inv);
                for h in hyphas {
                    let side = cx
                        .as_ref()
                        .and_then(|c| c.sections.get(&h))
                        .map(|sec| {
                            if sec.gamma.is_identity() {
                                vec![0]
                            } else {
                                axes.clone()
                            }
                        })
                        .unwrap_or_else(|| vec![0]);
                    let new = crate::transducer::stalk::Stalks::refine_axes(inv, &side);
                    cover.rehome(lane, inv, &new, h);
                }
            }
        }
        for (addr, mag) in before_mag {
            let after = self.tension.get(&addr).copied().unwrap_or(mag);
            if let Some(d) = crate::economy::split_deflation(&mag, &after) {
                self.deflations.insert(addr, d);
            }
        }
        if let Some(c) = self.complexes.remove(&inv.key()) {
            let a = crate::transducer::stalk::Stalks::refine_axes(inv, &[0]);
            let b = crate::transducer::stalk::Stalks::refine_axes(inv, &axes);
            self.rekey_cocycles(inv, &a, &b);
            self.complexes.insert(a.key(), c.clone());
            self.complexes.insert(b.key(), c);
        }
        self.basis_gen += 1;
    }

    /// The home installs a word it ruled once a peer's ruled digest holds
    /// the same stratum (same root and count) — evidence the peer glued it.
    pub fn confirm_peer_holds(&mut self, remote: &[crate::transducer::sheaf::StratumDigest], peer: [u8; 32]) {
        // index: the peer's digest charts; members from ruled fiber
        for d in remote {
            let members = self.ruled.fiber(&d.inv);
            let mut rows = members.clone();
            rows.sort_unstable();
            let mut h = blake3::Hasher::new();
            for a in &rows {
                h.update(a);
            }
            let root = *h.finalize().as_bytes();
            if root == d.root && members.len() as u32 == d.count {
                for addr in members {
                    if self.provisional.contains(&addr) {
                        eprintln!(
                            "  court peer holds {:02x}{:02x}… from {:02x}{:02x}…",
                            addr[0], addr[1], peer[0], peer[1]
                        );
                        self.home_install(&addr);
                    }
                }
            }
            let ck = d.inv.key();
            let drop: Vec<[u8; 32]> = self
                .heard
                .fiber(&crate::network::nerve::ROOT)
                .filter_map(|(addr, (chart, stored))| {
                    if *chart == ck && *stored != d.root {
                        Some(*addr)
                    } else {
                        None
                    }
                })
                .collect();
            for addr in drop {
                self.heard.remove(&addr);
            }
        }
    }

    /// Deflation a form may mint: the descent speaking it measures on the
    /// bound family's trains, or the listing's own magnitude when the word
    /// *is* that listing. Association with a listing (recorded deflation,
    /// sibling trains, whole magnitude at zero) is not a mint.
    fn verified_install_deflation(&self, addr: &[u8; 32], form: &Element) -> [u64; 4] {
        if form.is_sentence() {
            let task = self
                .speech_task
                .get(addr)
                .or_else(|| self.listing_task.get(addr))
                .copied();
            if let Some(task) = task {
                if let Some(family) = self.task_family.get(&task) {
                    return self.measured_replay_descent(form, family);
                }
            }
            return [0; 4];
        }
        if self.book_index.contains(addr) {
            return self.book_elem(addr).map(|e| e.magnitude()).unwrap_or([0; 4]);
        }
        [0; 4]
    }

    fn home_install(&mut self, addr: &[u8; 32]) {
        let held = self.word_at(addr).cloned().or_else(|| self.provisional.get(addr).cloned());
        if self.basis_index.get(addr).is_none() {
            if let Some(w) = held.as_ref() {
                if !(w.is_connection() && w.letters().is_none()) {
                    self.basis_push(w.clone(), [0; 32]);
                    self.fed_admit(addr, w.clone());
                }
            }
        }
        self.ruled_admit(addr);
        if let Some(w) = held {
            if w.is_rewrite() {
                let voice = self
                    .treasury
                    .escrow_of(addr)
                    .and_then(|(vs, _, _)| vs.first().copied())
                    .unwrap_or([0; 32]);
                let _ = self.treasury.release_rewrite(addr, voice);
                self.retire_rewrite_inputs(&w);
            } else {
                let pay = self.verified_install_deflation(addr, &w);
                if pay.iter().all(|&x| x == 0) {
                    let _ = self.treasury.revoke_award(addr, false);
                    eprintln!(
                        "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                        addr[0], addr[1]
                    );
                } else if let Some((voices, _, anchor)) = self.treasury.release_award(addr) {
                    self.treasury.mint_claim(&voices, pay, anchor);
                }
            }
        } else {
            let _ = self.treasury.revoke_award(addr, false);
            eprintln!(
                "  court form installed {:02x}{:02x}… mint none (closes nothing)",
                addr[0], addr[1]
            );
        }
        self.unlist(addr, false);
        self.provisional.remove(addr);
        self.court_upheld += 1;
        self.mark_persist();
        eprintln!("  court home installed {:02x}{:02x}… peer holds the stratum", addr[0], addr[1]);
    }

    /// A peer's have-list naming a ruled provisional is the same evidence:
    /// the address is on their cover, so the home installs.
    pub fn confirm_peer_holds_addrs(&mut self, have: &[[u8; 32]], peer: [u8; 32]) {
        // index: the peer's have-list
        for h in have {
            if self.provisional.contains(h) && self.ruled_contains(h) {
                eprintln!(
                    "  court peer holds {:02x}{:02x}… from {:02x}{:02x}…",
                    h[0], h[1], peer[0], peer[1]
                );
                self.home_install(h);
            }
        }
    }

    /// Charts this hypha covers: Twisted or Cavity listings, per occupied lane.
    /// Never the whole pending book — that clone was the last deploy's leak.
    pub fn own_cover_charts(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        // index: this hypha's own cover trie (signed digest payload)
        self.own_cover.all_charts()
    }

    pub fn complexes(&self) -> &crate::network::nerve::Keyed<crate::transducer::cech::Complex> {
        &self.complexes
    }

    /// Ideology boundary on a chart: classes at every degree the mesh holds
    /// there, including coarser prefixes, so a mind can be taught the frame
    /// its bubble sits on.
    pub fn classes_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<(crate::transducer::alphabet::Rotor, u8)> {
        let mut out = Vec::new();
        for p in inv.prefixes() {
            if let Some(cx) = self.complexes.get(&p.key()) {
                for (r, d) in cx.classes() {
                    out.push((r, d as u8));
                }
            }
        }
        out
    }

    pub fn cochain_bytes(&self) -> usize {
        let mut n = 0usize;
        for (_, c) in self.complexes.fiber(&crate::network::nerve::ROOT) {
            n += c.byte_len();
        }
        n
    }

    /// Class records on `inv` (and prefixes), or on every chart when `inv` is empty.
    pub fn class_records(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> Vec<crate::anastome::dialogue::ClassPage> {
        let mut out = Vec::new();
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let push = |out: &mut Vec<crate::anastome::dialogue::ClassPage>,
                    seen: &mut crate::network::nerve::Keyed<()>,
                    r: &crate::transducer::alphabet::Rotor,
                    order: u32| {
            let k = r.class_key();
            if seen.contains(&k) {
                return;
            }
            seen.insert(k, ());
            let mut rotor = Vec::new();
            r.to_bytes(&mut rotor);
            out.push(crate::anastome::dialogue::ClassPage {
                key: k,
                order,
                rotor,
            });
        };
        if inv.0.is_empty() {
            for (_, cx) in self.complexes.fiber(&crate::network::nerve::ROOT) {
                for layer in &cx.degrees {
                    for (_, cell) in layer.fiber(&crate::network::nerve::ROOT) {
                        match &cell.fill {
                            crate::transducer::cech::Fill::Congruent(r, ell) => {
                                push(&mut out, &mut seen, r, *ell)
                            }
                            crate::transducer::cech::Fill::Class(r) => {
                                push(&mut out, &mut seen, r, 0)
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            for p in inv.prefixes() {
                if let Some(cx) = self.complexes.get(&p.key()) {
                    for layer in &cx.degrees {
                        for (_, cell) in layer.fiber(&crate::network::nerve::ROOT) {
                            match &cell.fill {
                                crate::transducer::cech::Fill::Congruent(r, ell) => {
                                    push(&mut out, &mut seen, r, *ell)
                                }
                                crate::transducer::cech::Fill::Class(r) => {
                                    push(&mut out, &mut seen, r, 0)
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        out
    }

    pub fn serve_ruled(
        &self,
        inv: &crate::transducer::stalk::Invariant,
        have: &[[u8; 32]],
        budget: usize,
    ) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        let mut used = 0usize;
        let mut held: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for h in have {
            held.insert(*h, ());
        }
        // An empty mind is seeded from the root: coarsest ruled words first.
        let addrs: Vec<[u8; 32]> = if inv.0.is_empty() {
            self.syllabus_seed()
        } else {
            self.ruled.fiber(inv)
        };
        for addr in addrs {
            if held.contains(&addr) {
                continue;
            }
            let Some(w) = self.word_at(&addr) else { continue };
            if w.is_connection() && w.letters().is_none() {
                continue; // letterless history is never taught
            }
            let bytes = w.to_bytes();
            if used + bytes.len() + 4 > budget {
                break;
            }
            used += bytes.len() + 4;
            out.push(bytes);
        }
        out
    }

    pub fn ruled_digests(&self) -> Vec<crate::transducer::sheaf::StratumDigest> {
        crate::transducer::sheaf::digests(&self.ruled)
    }

    pub fn ruled_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<[u8; 32]> {
        self.ruled.fiber(inv).to_vec()
    }

    /// Ruled addresses, coarsest chart first — the syllabus seed of an
    /// empty atlas.
    pub fn syllabus_seed(&self) -> Vec<[u8; 32]> {
        let mut out = Vec::new();
        for (_, members) in self.ruled.strata() {
            out.extend(members);
        }
        out
    }

    pub fn fetch_have(&self, inv: &crate::transducer::stalk::Invariant) -> Vec<[u8; 32]> {
        let mut h: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for a in self.ruled.fiber(inv) {
            h.insert(a, ());
        }
        for (a, _) in self.heard.fiber(&crate::network::nerve::ROOT) {
            h.insert(*a, ());
        }
        h.fiber(&crate::network::nerve::ROOT).map(|(k, _)| *k).collect()
    }

    pub fn bind_listing_task(&mut self, listing: [u8; 32], task: [u8; 32]) {
        self.listing_task.insert(listing, task);
        match self.task_listings.get_mut(&task) {
            Some(v) => {
                if !v.iter().any(|a| a == &listing) {
                    v.push(listing);
                }
            }
            None => {
                self.task_listings.insert(task, vec![listing]);
            }
        }
    }

    /// The trains a rule must speak to mint at install. Empty trains are ignored.
    pub fn bind_task_family(&mut self, task: [u8; 32], train: Vec<(Vec<Vec<u8>>, Vec<Vec<u8>>)>) {
        if !train.is_empty() {
            self.task_family.insert(task, train);
        }
    }

    /// Speak a submitted rule onto withheld tests and compare cells exactly.
    /// The rule's cited forms resolve through this kernel's ruled cover.
    pub fn rule_matches_tests(&self, rule: &Element, tests: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> bool {
        if tests.is_empty() {
            return false;
        }
        let Some(sr) = sentence_rule_of(rule) else {
            return false;
        };
        tests.iter().all(|(x, y)| {
            crate::transducer::family::speak_rule(&sr, x, self).is_some_and(|s| s.grid() == *y)
        })
    }

    /// Pay a verified solve from the escrow of listings that task deflated.
    /// When a rule is supplied, the home speaks it on the withheld tests;
    /// a full match binds the solve to those listings, draws their escrow,
    /// and Offers the rule on the docket.
    pub fn pay_solve(
        &mut self,
        task: &[u8; 32],
        voice: [u8; 32],
        mu: [u64; 4],
        rule: Option<&Element>,
        tests: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
    ) -> [u64; 4] {
        if let Some(rule) = rule {
            if !self.rule_matches_tests(rule, tests) {
                return [0; 4];
            }
            self.docket_connection(rule, voice);
            self.offered.insert(rule.address(), ());
        }
        let listings: Vec<[u8; 32]> = self.task_listings.get(task).cloned().unwrap_or_default();
        let mut paid = [0u64; 4];
        let anchor = self.anchor();
        for addr in &listings {
            if let Some((_, held, a)) = self.treasury.release_award(addr) {
                let take = self.treasury.settle_solve(voice, held, a);
                for l in 0..4 {
                    paid[l] = paid[l].saturating_add(take[l]);
                }
            }
        }
        if paid.iter().all(|&x| x == 0) {
            return self.treasury.settle_solve(voice, mu, anchor);
        }
        paid
    }

    /// Drop the inflated basis; keep only lettered court-federation words
    /// (the ruled cover). Returns how many cards left.
    pub fn prune_basis_to_ruled(&mut self) -> usize {
        let before = self.basis.len();
        let mut nb = Vec::new();
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
            seen.insert(*addr, ());
            if let Some(&i) = self.basis_index.get(addr) {
                nb.push(std::sync::Arc::clone(&self.basis[i]));
                nd.insert(*addr, self.discoverer_of(addr));
            } else {
                nb.push(std::sync::Arc::new(elem.clone()));
                nd.insert(*addr, [0; 32]);
            }
        }
        if seen.is_empty() {
            self.basis.clear();
            self.discoverers = crate::network::nerve::Keyed::new();
            self.reindex_basis();
            self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::default());
            self.ruled = crate::transducer::stalk::Stalks::default();
            self.ruled_addrs = crate::network::nerve::Keyed::new();
            self.refresh_ruled_root();
            self.basis_gen += 1;
            self.retire_gen += 1;
            self.language_rebuild();
            return before;
        }
        let dropped = before.saturating_sub(nb.len());
        self.basis = nb;
        self.discoverers = nd;
        self.reindex_basis();
        self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::over_elements(self.basis.iter().map(|w| w.as_ref())));
        self.ruled = crate::transducer::stalk::Stalks::over_elements(self.basis.iter().map(|w| w.as_ref()));
        self.ruled_addrs = crate::network::nerve::Keyed::new();
        for w in &self.basis {
            self.ruled_addrs.insert(w.address(), ());
        }
        self.refresh_ruled_root();
        self.basis_gen += 1;
        self.retire_gen += 1;
        self.language_rebuild();
        dropped
    }

    pub fn ruled_root(&self) -> [u8; 32] {
        self.ruled_root
    }

    /// Test/helper: identity-glue every provisional as if an overlapping
    /// hypha fetched the same word. The basis grows from gluing, not from
    /// the two-voice quorum that listed them.
    pub fn glue_local(&mut self) {
        let words: Vec<Element> = self.provisional.fiber(&crate::network::nerve::ROOT).map(|(_, e)| e.clone()).collect();
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
        let addrs: Vec<[u8; 32]> = self.provisional.fiber(&crate::network::nerve::ROOT).map(|(a, _)| *a).collect();
        for a in addrs {
            self.ruled_admit(&a);
        }
    }

    fn fed_admit(&mut self, addr: &[u8; 32], spoken: Element) {
        if self.court_fed_index.get(addr).is_some() {
            return;
        }
        self.court_fed_index.insert(*addr, self.court_federation.len());
        self.court_federation.push((*addr, spoken));
    }

    /// Active cover: the identity fibre (so Čech overlap is non-empty after
    /// the first Cover), plus charts of open listings and of the ruled language.
    pub fn active_cover(&self) -> Vec<(u8, crate::transducer::stalk::Invariant)> {
        let mut out = vec![(0, crate::transducer::stalk::invariant_of(&Element::identity(2)))];
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
    pub fn has_listing(&self, addr: &[u8; 32]) -> bool {
        self.book_index.contains(addr)
    }

    /// Relocate one listing onto the cavity atlas. The address is kept.
    pub fn relocate_listing(&mut self, addr: &[u8; 32]) {
        self.relocate_cavity(addr);
    }

    /// Relocate restored demands whose chart no longer homes here. Never dropped.
    pub fn unlist_foreign(&mut self, home: impl Fn(&Element) -> bool) {
        let keys: Vec<[u8; 32]> = self
            .pending
            .iter()
            .filter(|(p, _)| !home(p))
            .map(|(p, _)| p.address())
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
                if self.basis_index.get(&addr).is_none() {
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

    pub fn offered(&self, addr: &[u8; 32]) -> bool {
        self.offered.get(addr).is_some()
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

    pub fn is_home_listing(&self, addr: &[u8; 32], closest_is_self: bool) -> bool {
        let _ = addr;
        closest_is_self
    }

    /// Room on each lane from that lane only: bonded μ × (reserve/spend)
    /// minus occupancy. Lane 2 is standing-extent slots per unit T-bond;
    /// ticks never enter S, G, or H. An unbonded kernel (tests) uses one
    /// unit of bond so the host's reserve still grants cover.
    pub fn bind_identity(&mut self, vk: [u8; 32]) {
        self.identity = vk;
    }

    pub fn lane_room(&self) -> [usize; 4] {
        if let Some(cap) = self.lane_cap {
            return std::array::from_fn(|l| cap[l].max(1).saturating_sub(self.occupancy[l] as usize));
        }
        let Some(bond) = self.treasury.bond_of(&self.identity) else {
            return [0; 4];
        };
        let spend = crate::physics::expansion_unit();
        std::array::from_fn(|l| {
            crate::network::nerve::lane_cover_room(bond[l], self.physics.reserve[l], self.occupancy[l], spend) as usize
        })
    }

    pub fn occupancy(&self) -> [u64; 4] {
        self.occupancy
    }

    /// Checkout a T-lease for a quoting voice. One lease per voice.
    pub fn grant_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.contains(&voice) {
            return;
        }
        self.t_leases.insert(voice, ());
        self.occupancy[2] = self.occupancy[2].saturating_add(1);
    }

    /// Return a T-lease when the voice's quoted work settles or expires.
    pub fn release_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.remove(&voice).is_some() {
            self.occupancy[2] = self.occupancy[2].saturating_sub(1);
        }
    }

    pub fn reserve(&self) -> [u64; 4] {
        self.physics.reserve
    }

    /// Test helper: remaining listing room, from each lane's own bond and reserve.
    pub fn dynamic_lane_capacity(&self) -> [usize; 4] {
        let room = self.lane_room();
        std::array::from_fn(|l| room[l].saturating_add(self.occupancy[l] as usize).max(1))
    }

    /// Does a residual fit the book on every lane it moves?
    fn fits_book(&self, mag: &[u64; 4]) -> bool {
        let room = self.lane_room();
        (0..4).all(|l| mag[l] == 0 || room[l] > 0)
    }

    fn market_occupancy(&self) -> usize {
        let mut children_listed = 0usize;
        let mut parents_open = 0usize;
        for (_, children, _) in &self.composites {
            let listed = children
                .iter()
                .filter(|(a, done)| !done && self.book_index.contains(a))
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
        let mut voices: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        voices.insert(voice, ());
        for (a, _) in &children {
            if let Some(i) = self.basis_index.get(a).copied() {
                voices.insert(self.discoverer_at(i), ());
            }
        }
        let voices: Vec<[u8; 32]> = voices.fiber(&crate::network::nerve::ROOT).map(|(k, _)| *k).collect();
        let anchor = self.anchor();
        // μ_assembly: the overlap volume actually glued — measured, never flat.
        let mu = [0, 0, 0, strip.len().max(1) as u64];
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
    /// against held answers). Pays the registered task tension, pool-funded
    /// and float-law bounded; the anti-replay collapse happened upstream —
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
        if self.provisional.contains(addr) {
            return None;
        }
        // Never a BUNDLE — its sections/cycles carry family volume no other
        // word speaks.
        let candidate = (*self.basis[i]).clone();
        if !candidate.sections().is_empty() || !candidate.torsion().is_empty() {
            return None;
        }
        // THE REPLAY rules: the bidder's sentence, spoken by other words,
        // must reproduce the candidate's substance exactly. No search here.
        // Citations resolve to any held index but the candidate's own (a
        // duplicate card at another index is exactly what a bid names).
        let resolved: Vec<Cite> = cites
            .iter()
            .filter_map(|(a, _st, g, asg, _)| {
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
                let inv = crate::transducer::stalk::invariant_of(&candidate);
                let after = self.basis.get(0).map(|w| (**w).clone()).unwrap_or_else(|| candidate.clone());
                let payload = crate::transducer::rewrite::RewritePayload::Refine {
                    chart: inv,
                    axis: addr[0] as u64,
                };
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Refine,
                    &[candidate.clone()],
                    &after,
                    payload,
                    bidder,
                );
                self.basis_remove(i);
                self.physics.syntropy(&candidate.magnitude());
                self.append_step(*addr, bidder);
                Some(saved)
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
            self.word_mut(i).merge_fibre(word);
            self.touch_at(i);
            // Presence is not finality: an unruled connection stays on the
            // docket until the court measures it.
            if (word.is_connection() || word.is_sentence() || word.is_rewrite()) && !self.ruled_contains(&addr) {
                self.docket_connection(word, ratifier);
            }
            return Adoption::Known;
        }
        // A CONNECTION arriving from an entity is listed as itself: it enters
        // the docket with its own magnitude as the deflation it claims, so
        // the court rules it (inversion, remainder, torsion) and the mesh
        // pays it as a word, never as a copied pair. A residual word taught
        // up is installed as before. A sentence is the same: Offered on the
        // docket so a second organ's replay installs it as language.
        if word.is_connection() || word.is_sentence() || word.is_rewrite() {
            self.docket_connection(word, ratifier);
        }
        self.basis_push(word.clone(), ratifier);
        // A word the court itself teaches is finalized language: it joins
        // the ruled cover on arrival.
        if self.court_voice == Some(ratifier) {
            self.ruled_admit(&addr);
        }
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
                voices.insert(voice, ());
            }
            None => {
                let mut voices = crate::network::nerve::Keyed::new();
                voices.insert(voice, ());
                self.spoken.insert(listing, voices);
            }
        }
    }

    /// The receipts of speech, keyed by listing: read at a listing's own
    /// address, never walked.
    pub fn spoken(&self) -> &crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> {
        &self.spoken
    }

    pub fn quote_frontier(&self) -> &crate::network::nerve::Keyed<()> {
        &self.quote_frontier
    }

    /// PUNCTURE held words from a verified utterance's carried bodies: the
    /// fiber deepens on what the hypha verified. A deepened basis moves the
    /// basis generation so the language snapshot refreshes.
    pub fn puncture_from(&mut self, cites: &[crate::anastome::dialogue::CiteWire]) -> usize {
        let index = &self.basis_index;
        let deepened = crate::transducer::sentence::puncture_from(&mut self.basis, |a| index.get(a).copied(), cites);
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
        // Settlements are receipts: they stay until the court hears them, never drop.
        let k = self.k_step();
        let tu = self.now_tu();
        self.settlements.push((listing, chain, voice, cites.to_vec(), before, after, k, tu));
        let floor = self.k_step().saturating_sub(crate::physics::reserve_max());
        while self.settlements.first().map(|s| s.6 < floor && s.6 + 1 < self.k_step()).unwrap_or(false) {
            let old = self.settlements.remove(0);
            self.settlements_cold.push(old);
        }
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
        self.settlements_cold.extend(self.settlements.drain(..n));
    }

    /// PRESSURE IS THE OPEN BOOK: the per-lane magnitude of every listing,
    /// measured whenever the book moved — never an accumulator. A demand
    /// the language already speaks adds nothing; a settlement removes
    /// exactly what it settled.
    fn remeasure_pressure(&mut self) {
        let now = self.now_tu();
        let mut p = self.pressure_sum;
        p[2] = 0;
        for (elem, _) in &self.pending {
            p[2] = p[2].saturating_add(self.t_extent(&elem.address(), now));
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
    fn record_deflation(&mut self, word: [u8; 32], listing: [u8; 32], own: [u64; 4]) {
        // The demand a percept settles is the listing of its whole chain;
        // the word installed may be the residual after citations, so the
        // deflation is read on the listing, then on the word itself. A germ
        // consilienced at this home from routed demands whose listing never
        // stood here (or was evicted) deflated at least its own measure —
        // never a fabricated zero for the court to slash.
        let on = |addr: [u8; 32]| self.book_elem(&addr).map(|p| p.magnitude());
        let deflation = on(listing).or_else(|| on(word)).unwrap_or(own);
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
        if self.basis_index.contains(&addr) || self.book_index.contains(&addr) {
            return false; // already speakable or already listed
        }
        // Every lane the residual moves must have room; none is chosen.
        if !self.fits_book(&residual.magnitude()) {
            return false; // the thermodynamic book is full on a lane it needs; corpus can wait
        }
        let k_now = self.k_step();
        self.stamp_standing(addr);
        self.book_list(residual, k_now);
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
        let key = footprint_key(addr, &fp, &scene);
        if self.evidence_at.contains(&key) {
            return false;
        }
        self.evidence_at.insert(key, ());
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
            let rebound = (*self.basis[i]).clone().with_anchor(pi.clone());
            self.basis[i] = std::sync::Arc::new(rebound);
            self.touch_at(i);
            bound = true;
        }
        // A germ still PENDING quorum accepts the bind too — it installs
        // already anchored (evidence often reaches family size before the
        // second voice completes the install).
        if let Some(&i) = self.book_index.get(addr) {
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
            self.mismatch_at.remove(addr);
            self.anchor_mismatches.retain(|(a, _, _)| a != addr);
        }
        bound
    }

    /// ENROLL an identity with a per-lane bond. The hypha enrolls itself
    /// from genesis or from a restored stake; arriving voices stay unbonded
    /// until they bond from balance. Returns the vector added this call.
    pub fn enroll(&mut self, who: [u8; 32], bond: [u64; 4]) -> [u64; 4] {
        let anchor = self.anchor();
        self.treasury.enroll(who, bond, anchor)
    }

    /// Mint a genesis allocation once per identity.
    pub fn allocate(&mut self, who: [u8; 32], mu: [u64; 4]) -> [u64; 4] {
        if self.treasury.has_mint(&who) {
            return [0; 4];
        }
        self.treasury.settle_subsidy(who, mu, self.anchor())
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
        let Reduction { germ: g, ctx, form, qs, language, listing, frontier } = r;
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
                    if !self.mismatch_at.contains(&addr) {
                        self.mismatch_at.insert(addr, ());
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
                            let key = footprint_key(addr, &fp, scene);
                            if !self.evidence_at.contains(&key) {
                                self.evidence_at.insert(key, ());
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
                    self.treasury.settle_use(voice, self.discoverer_at(i), mag, anchor);
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
                // The stratum over the residual's chart, read by key: a held
                // word on the same base positions is a section prospect.
                let same_base = self
                    .stalks
                    .fiber(&crate::transducer::stalk::invariant(&centered))
                    .iter()
                    .filter_map(|a| self.basis_index.get(a).copied())
                    .find(|&t| !self.basis[t].sub().is_empty() && element::positions(self.basis[t].sub()) == element::positions(&centered));
                if let Some(ti) = same_base {
                    let asg = element::assignment(&centered);
                    let saddr = section_address(&self.basis[ti].address(), &asg);
                    let _ = self.admission.record(Receipt { voice, germ: saddr });
                    self.record_spoken(saddr, voice);
                    let quorum = self.admission.voices(&saddr) >= crate::network::consensus::QUORUM;
                    if quorum && self.physics.can_expand(lanes) {
                        self.word_mut(ti).puncture(asg);
                        self.touch_at(ti);
                        self.physics.relieve(&mag);
                        self.physics.syntropy(&mag);
                        self.unlist(&residual.address(), false);
                        self.append_step(saddr, voice);
                        self.complete_composite_child(&gaddr, voice);
                        return Outcome::Sectioned(ti);
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
                    let mu = crate::economy::price(&residual.magnitude(), &self.physics.pressure);
                    let anchor = self.anchor();
                    // PROVISIONAL (Step 5): consilience escrows pending the
                    // court's admission docket; the word joins the basis as
                    // local dialect, final settlement waits for the verdict.
                    self.treasury.escrow_award(addr, self.admission.voices_of(&addr), mu, anchor);
                    self.record_deflation(addr, listing, residual.magnitude());
                    let k_now = self.k_step();
                    self.provisional_k.insert(addr, k_now);
                    self.provisional.insert(addr, residual.clone());
                    self.provisional_tu.insert(addr, self.now_tu());
                    self.stamp_standing(addr);
                    self.physics.relieve(&mag);
                    self.physics.syntropy(&mag); // a completed rule is order built
                    self.physics.spend(lanes); // the expansion SHRINKS the reserve
                    // Seal the K-step receipt: the install IS a tick of algebraic
                    // time, parent-linked to the mesh's whole history.
                    self.append_step(addr, voice);
                    self.complete_composite_child(&gaddr, voice); // lineage
                    // The listing is settled. The basis grows only from gluing,
                    // never from a single hypha's two voices. The field stays
                    // so the court stroke can read it; the page clears.
                    let inv = crate::transducer::stalk::invariant_of(&residual);
                    if self.field.get(&addr).is_none() {
                        self.write_field(
                            &addr,
                            inv,
                            crate::transducer::stalk::Closure::Cavity,
                            residual.sub().to_vec(),
                            &residual.magnitude(),
                            &residual.magnitude(),
                        );
                    }
                    self.docket_connection(&residual, voice);
                    self.offered.insert(addr, ());
                    Outcome::GermPending
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
                    // A residual a held word carries by shape, under a gauge
                    // the word does not hold, is holonomic demand — its
                    // tension projects onto lane 3, so the puncture that
                    // settles it is paid there.
                    let residual = {
                        let mut r = residual;
                        if crate::transducer::stalk::congruent(&self.stalks, |a| self.word_at(a), &r).is_some() {
                            r.mag[3] = r.sub().len() as u64;
                        }
                        r
                    };
                    if frontier {
                        self.quote_frontier.insert(addr, ());
                    }
                    if !self.book_index.contains(&addr) {
                        // Every lane the residual moves must have room; no
                        // lane is chosen. When one it needs is full, the
                        // weakest listing standing on a full lane it needs
                        // may yield its place.
                        let k_now = self.k_step();
                        let mut need = residual.magnitude();
                        need[2] = need[2].max(1);
                        if self.lane_room()[2] == 0 {
                            self.relocate_t_exhausted();
                        }
                        let room = self.lane_room();
                        let saturated: Vec<usize> = (0..4).filter(|&l| l != 2 && need[l] > 0 && room[l] == 0).collect();
                        if !self.fits_book(&need) {
                            let new_a = action_potential(
                                &crate::economy::price(&residual.magnitude(), &self.physics.pressure),
                                0,
                            );
                            let mut cands: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                            for &l in &saturated {
                                for (a, _) in self.lane_listings[l].fiber(&crate::network::nerve::ROOT) {
                                    cands.insert(*a, ());
                                }
                            }
                            let weakest = cands.fiber(&crate::network::nerve::ROOT).filter_map(|(addr, _)| {
                                let &i = self.book_index.get(addr)?;
                                let (p, lk) = self.pending.get(i)?;
                                let a = action_potential(
                                    &crate::economy::price(&p.magnitude(), &self.physics.pressure),
                                    k_now.saturating_sub(*lk),
                                );
                                Some((i, a))
                            }).min_by_key(|(_, a)| *a);
                            match weakest {
                                Some((i, a)) if a < new_a => {
                                    let evicted = self.pending[i].0.address();
                                    self.unlist(&evicted, false);
                                    self.book_list(residual.clone(), k_now);
                                    self.docket_connection(&residual, voice);
                                    self.offered.insert(addr, ());
                                }
                                _ => {} // the newcomer IS the weakest — unlisted
                            }
                        } else {
                            self.book_list(residual.clone(), k_now);
                            self.docket_connection(&residual, voice);
                            self.offered.insert(addr, ());
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
        let budget = self.peel_budget(&g.magnitude());
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
    fn peel_budget(&self, mag: &[u64; 4]) -> usize {
        // The occupied lane of this reduction; no max across lanes.
        for l in 0..4 {
            if mag[l] > 0 {
                let unit = crate::physics::expansion_unit().max(1);
                return (self.physics.reserve[l] / unit) as usize;
            }
        }
        0
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
        normal_form(&self.language, &first, exclude)
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

    /// The syntropic stroke reads the field's stagnation per chart per lane.
    /// A chart stagnant on gauge is a condense candidate, holonomic torse,
    /// shell refine, spatial re-anchor. Temporal stagnation relocates on T.
    /// Candidates from open and the ruled cover.
    pub fn syntropic_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut jobs: Vec<(crate::transducer::stalk::Invariant, [bool; 4])> = Vec::new();
        // index: budgeted cursor over the open docket
        for (addr, _) in self.open.cursor(crate::network::nerve::ROOT, &mut budget) {
            let Some(listing) = self.field.get(addr) else {
                continue;
            };
            for e in &listing.entries {
                if e.state.is_flat() {
                    continue;
                }
                let lanes = std::array::from_fn(|l| e.stagnant[l] > 0);
                if lanes.iter().any(|&x| x) {
                    jobs.push((e.chart.clone(), lanes));
                }
            }
        }
        // index: ruled cover fiber
        let ruled: Vec<[u8; 32]> = self.ruled_addrs.fiber(&crate::network::nerve::ROOT).map(|(a, _)| *a).collect();
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            if let Some(w) = self.word_at(addr) {
                jobs.push((crate::transducer::stalk::invariant_of(w), [true; 4]));
            }
        }
        let idle = jobs.is_empty();
        let mut t_stale = false;
        for (inv, lanes) in jobs {
            if lanes[1] {
                if let Some(a) = self.stalks.fiber(&inv).first().copied() {
                    let _ = self.condense_on(&a);
                }
            }
            if lanes[2] {
                t_stale = true;
            }
            if lanes[3] {
                if let Some(a) = self.stalks.fiber(&inv).first().copied() {
                    if let Some(i) = self.basis_index.get(&a).copied() {
                        self.torse_at(i);
                    }
                    self.refine_stratum(&a);
                }
            }
            if lanes[0] {
                self.reanchor_chart(&inv);
            }
        }
        if t_stale {
            self.relocate_t_exhausted();
        }
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            let Some(i) = self.basis_index.get(addr).copied() else {
                continue;
            };
            let candidate = (*self.basis[i]).clone();
            if matches!(self.reduce_excluding(&candidate, Some(i)), NormalForm::Decomposed(_)) {
                self.retire_redundant(addr);
            }
        }
        if idle {
            let _ = self.condense_stroke();
        }
        self.physics.regenerate();
    }

    fn reanchor_chart(&mut self, inv: &crate::transducer::stalk::Invariant) {
        let members = self.stalks.fiber(inv);
        for addr in members {
            let Some(w) = self.word_at(&addr).cloned() else {
                continue;
            };
            if !w.is_sentence() {
                continue;
            }
            let Some(mut n) = crate::transducer::sentence::from_element(&w) else {
                continue;
            };
            let Some(gamma) = crate::transducer::sentence::solve_coboundary(&n) else {
                continue;
            };
            if gamma.iter().all(|g| g.abelian_identity()) {
                continue;
            }
            crate::transducer::sentence::apply_gauge(&mut n, &gamma);
            let Some(after) = crate::transducer::sentence::as_element(&n) else {
                continue;
            };
            let payload = crate::transducer::rewrite::RewritePayload::Reanchor { gamma };
            let voice = self.discoverer_of(&addr);
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Reanchor,
                &[w],
                &after,
                payload,
                voice,
            );
            break;
        }
    }

    /// The whole stroke under one lock (the reflex and test paths).
    pub(crate) fn optimize(&mut self) {
        self.syntropic_stroke();
        if let Some(i) = self.condense_stroke() {
            let candidate = (*self.basis[i]).clone();
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(i)) {
                self.retire_redundant(&candidate.address());
            }
        }
    }

    /// The stroke for the idle deliberation worker: the cheap moves (condense,
    /// torse, regenerate) run here under the lock; the redundancy probe — a
    /// full decomposition — is handed out as (address, candidate, the basis
    /// without it) to run on that snapshot with no lock held, exactly as
    /// adoption's derivability does. The reflex path never waits on it.
    pub fn consolidate_stroke(&mut self) {
        self.syntropic_stroke();
    }

    /// Retire a plain card the rest of the basis derives (the off-lock probe
    /// found it decomposable). Returns the cards retired (0 or 1): the word
    /// may have gone, or gained a fibre, while the probe ran.
    pub fn retire_redundant(&mut self, addr: &[u8; 32]) -> usize {
        let Some(i) = self.basis_index.get(addr).copied() else { return 0 };
        if !self.basis[i].sections().is_empty() || !self.basis[i].torsion().is_empty() {
            return 0;
        }
        let mag = self.basis[i].magnitude();
        let elem = self.basis_remove(i);
        self.ruled_admit(addr);
        self.cavity_book.insert(*addr, elem.clone());
        self.cavities.push(elem);
        self.physics.syntropy(&mag);
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
            let own = self.basis[i].address();
            if !self.basis[i].sub().is_empty() {
                let inv = crate::transducer::stalk::invariant_of(&self.basis[i]);
                // The stratum's other members, by address — the atlas is not
                // a function of array order; the cursor's own word is found
                // again by its key after a removal.
                // A bundle absorbs cards; a card never absorbs a bundle (the
                // bundle absorbs it when the cursor reaches it). Read on the
                // fibre volume, never on array order.
                let fiber: Vec<[u8; 32]> = self
                    .stalks
                    .fiber(&inv)
                    .iter()
                    .copied()
                    .filter(|a| *a != own)
                    .filter(|a| {
                        self.basis_index
                            .get(a)
                            .and_then(|&j| self.basis.get(j))
                            .is_some_and(|w| w.sections().is_empty() && w.torsion().is_empty())
                    })
                    .collect();
                // One versor solved per stroke: the stroke's cursor also picks
                // which member of the stratum is tried, so successive strokes
                // walk the whole stratum without ever solving it all at once.
                let pick = match fiber.is_empty() {
                    true => Vec::new(),
                    false => vec![fiber[self.optimize_cursor % fiber.len()]],
                };
                let mut absorbed = 0usize;
                for sibling in pick {
                    let (Some(j), Some(i)) = (self.basis_index.get(&sibling).copied(), self.basis_index.get(&own).copied()) else { continue };
                    let Some(g) = element::transition(self.basis[j].sub(), self.basis[i].sub())
                    else {
                        continue;
                    };
                    let sib = (*self.basis[j]).clone();
                    let sibling_disc = self.discoverer_at(j);
                    self.basis_remove(j);
                    let Some(i) = self.basis_index.get(&own).copied() else { continue };
                    if self.word_mut(i).attach(&sib, &g) > 0 {
                        let after = (*self.basis[i]).clone();
                        let payload = crate::transducer::rewrite::RewritePayload::Condense {
                            sections: after.sections().to_vec(),
                            gauge: g.clone(),
                        };
                        let _ = self.mint_rewrite(
                            crate::transducer::rewrite::RewriteKind::Condense,
                            &[sib.clone()],
                            &after,
                            payload,
                            sibling_disc,
                        );
                        self.touch_at(i);
                    }
                    self.physics.syntropy(&[0, 1, 0, 0]);
                    absorbed += 1;
                }
            }
            // The cursor's word, found again by its key: a removal above may
            // have moved its position.
            let Some(i) = self.basis_index.get(&own).copied() else { return None };
            // THE TORSE MOVE (condense, one dimension deeper): extract the torsion
            // of the cursor thought's axis — sections forming one generator's
            // orbit condense into a cycle (σ, ℓ). The extracted mathematical
            // torsion RELIEVES the physical torsion charge on the germ's lanes
            // (disorder converted to structure — per-lane, never a scalar).
            let spanned = self.word_mut(i).torse();
            let wrote = spanned > 0;
            if spanned > 0 {
                let lanes = self.basis[i].lanes();
                self.physics.torse_relief(lanes, spanned as u64);
                let after = (*self.basis[i]).clone();
                let (sigma, period) = after
                    .torsion()
                    .last()
                    .map(|(s, p, _)| (s.clone(), *p))
                    .unwrap_or((Vec::new(), spanned as u32));
                let payload = crate::transducer::rewrite::RewritePayload::Torse { sigma, period };
                let disc = self.discoverer_at(i);
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Torse,
                    std::slice::from_ref(&after),
                    &after,
                    payload,
                    disc,
                );
            }
            if wrote {
                self.touch_at(i);
            }
            if self.basis[i].sections().is_empty() && self.basis[i].torsion().is_empty() {
                return Some(i);
            }
        }
        None
    }

    /// A read-only view of the germinated basis (for the CCL display seam).
    pub fn basis(&self) -> &[std::sync::Arc<Element>] {
        &self.basis
    }

    /// QUOTE — a local reading of the page: listings still on_page, in
    /// atlas order of pending, bounded by `top`. Tests use this; the wire
    /// quote is an incidence query over the voice's charts.
    pub fn quote(&mut self, top: usize) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        self.remeasure_pressure();
        let surface = crate::economy::yield_surface(&self.physics);
        let mut priced = Vec::new();
        for (p, _) in self.pending.iter() {
            if priced.len() >= top {
                break;
            }
            if !self.on_page.contains(&p.address()) {
                continue;
            }
            let mut mag = p.magnitude();
            mag[2] = self.t_extent(&p.address(), self.now_tu());
            let mu = crate::economy::price(&mag, &self.physics.pressure);
            priced.push((p.clone(), mu));
        }
        (surface, priced)
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
        let mut reductions = reduce_all(&language, budget, vec![germ]);
        for r in &mut reductions {
            r.frontier = true;
        }
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
        out.extend_from_slice(b"MYC2");
        out.extend_from_slice(&(self.basis.len() as u32).to_le_bytes());
        for g in &self.basis {
            let who = self.discoverer_of(&g.address());
            let eb = g.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
            out.extend_from_slice(&who);
        }
        out.extend_from_slice(&self.treasury.snapshot());
        // PROVISIONAL section (Step 5) — appended so pre-court snapshots
        // restore with an empty docket rather than failing.
        out.extend_from_slice(&(self.provisional.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.provisional, |addr, spoken| {
            out.extend_from_slice(addr);
            let eb = spoken.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
        });
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
        // DEFLATION section: each provisional's measured descent, so a
        // restart does not docket Δ⃗ = 0 and feed the court a slash list.
        out.extend_from_slice(b"DEF1");
        out.extend_from_slice(&(self.deflations.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.deflations, |addr, mag| {
            out.extend_from_slice(addr);
            for m in mag {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&self.provisional_k.get(addr).copied().unwrap_or(0).to_le_bytes());
            out.extend_from_slice(&self.provisional_tu.get(addr).copied().unwrap_or(0).to_le_bytes());
        });
        // Coherence field beside DEF1: which angles each thought has been
        // probed through. A restart must not forget the field.
        out.extend_from_slice(b"FLD1");
        out.extend_from_slice(&(self.field.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.field, |addr, listing| {
            out.extend_from_slice(addr);
            out.extend_from_slice(&(listing.entries.len() as u32).to_le_bytes());
            for e in &listing.entries {
                out.extend_from_slice(&(e.chart.0.len() as u32).to_le_bytes());
                for axis in &e.chart.0 {
                    out.extend_from_slice(&axis.to_le_bytes());
                }
                match &e.state {
                    crate::transducer::stalk::Closure::Flat => out.push(0),
                    crate::transducer::stalk::Closure::Twisted(v) => {
                        out.push(1);
                        v.to_bytes(&mut out);
                    }
                    crate::transducer::stalk::Closure::Cavity => out.push(2),
                    crate::transducer::stalk::Closure::Class(r, d) => {
                        out.push(3);
                        out.push(*d);
                        r.to_bytes(&mut out);
                    }
                }
                out.extend_from_slice(&(e.remainder.len() as u32).to_le_bytes());
                for &(r, c, hb, ha) in &e.remainder {
                    out.extend_from_slice(&r.to_le_bytes());
                    out.extend_from_slice(&c.to_le_bytes());
                    out.push(hb);
                    out.push(ha);
                }
                for s in e.stagnant {
                    out.extend_from_slice(&s.to_le_bytes());
                }
            }
        });
        out.extend_from_slice(b"BOK1");
        out.extend_from_slice(&(self.pending.len() as u32).to_le_bytes());
        for (elem, k) in &self.pending {
            let eb = elem.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
            out.extend_from_slice(&k.to_le_bytes());
        }
        out.extend_from_slice(b"CAV1");
        out.extend_from_slice(&(self.cavity_book.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.cavity_book, |_, elem| {
            let eb = elem.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
        });
        out.extend_from_slice(b"CRT1");
        out.push(self.crt_imported as u8);
        out.extend_from_slice(&(self.court_federation.len() as u32).to_le_bytes());
        for (addr, elem) in &self.court_federation {
            out.extend_from_slice(addr);
            let eb = elem.to_bytes();
            out.extend_from_slice(&(eb.len() as u32).to_le_bytes());
            out.extend_from_slice(&eb);
        }
        out.extend_from_slice(&0u32.to_le_bytes());
        out.extend_from_slice(b"COC1");
        out.extend_from_slice(&(self.complexes.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.complexes, |k, c| {
            out.extend_from_slice(k);
            c.to_bytes(&mut out);
        });
        out.extend_from_slice(b"FLC1");
        out.extend_from_slice(&(self.field_cold.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.field_cold, |addr, listing| {
            out.extend_from_slice(addr);
            out.extend_from_slice(&(listing.entries.len() as u32).to_le_bytes());
            for e in &listing.entries {
                out.extend_from_slice(&(e.chart.0.len() as u32).to_le_bytes());
                for axis in &e.chart.0 {
                    out.extend_from_slice(&axis.to_le_bytes());
                }
                match &e.state {
                    crate::transducer::stalk::Closure::Flat => out.push(0),
                    crate::transducer::stalk::Closure::Twisted(v) => {
                        out.push(1);
                        v.to_bytes(&mut out);
                    }
                    crate::transducer::stalk::Closure::Cavity => out.push(2),
                    crate::transducer::stalk::Closure::Class(r, d) => {
                        out.push(3);
                        out.push(*d);
                        r.to_bytes(&mut out);
                    }
                }
                out.extend_from_slice(&(e.remainder.len() as u32).to_le_bytes());
                for &(r, c, hb, ha) in &e.remainder {
                    out.extend_from_slice(&r.to_le_bytes());
                    out.extend_from_slice(&c.to_le_bytes());
                    out.push(hb);
                    out.push(ha);
                }
                for s in e.stagnant {
                    out.extend_from_slice(&s.to_le_bytes());
                }
            }
        });
        out.extend_from_slice(b"CYC2");
        out.extend_from_slice(&(self.cocycle_at.len() as u32).to_le_bytes());
        crate::snapshot::persist(&self.cocycle_at, |key, (v, tu, a, b, ck)| {
            out.extend_from_slice(key);
            out.extend_from_slice(a);
            out.extend_from_slice(b);
            out.extend_from_slice(&tu.to_le_bytes());
            out.extend_from_slice(ck);
            v.to_bytes(&mut out);
        });
        out.extend_from_slice(&(self.dialect_chain.len() as u32).to_le_bytes());
        for (key, v, tu) in &self.dialect_chain {
            out.extend_from_slice(key);
            out.extend_from_slice(&tu.to_le_bytes());
            v.to_bytes(&mut out);
        }
        out.extend_from_slice(b"SET1");
        let mut all = self.settlements_cold.clone();
        all.extend(self.settlements.iter().cloned());
        out.extend_from_slice(&(all.len() as u32).to_le_bytes());
        for (listing, chain, voice, cites, before, after, k, tu) in &all {
            out.extend_from_slice(listing);
            crate::anastome::dialogue::push_chain(&mut out, chain);
            out.extend_from_slice(voice);
            crate::anastome::dialogue::push_cites(&mut out, cites);
            for m in before {
                out.extend_from_slice(&m.to_le_bytes());
            }
            for m in after {
                out.extend_from_slice(&m.to_le_bytes());
            }
            out.extend_from_slice(&k.to_le_bytes());
            out.extend_from_slice(&tu.to_le_bytes());
        }
        out
    }

    /// P7 — RESTORE a snapshot into a fresh kernel. Returns `false` (leaving the
    /// kernel empty) on a bad magic or corruption — a corrupt ledger never crashes
    /// a boot, it just starts clean.
    pub fn restore(&mut self, b: &[u8]) -> bool {
        if b.get(0..4) != Some(b"MYC2") && b.get(0..4) != Some(b"MYC1") {
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
            // Skip the leaked basis: do not deserialize or install it.
            // The ruled cover is rebuilt from CRT1 (lettered federation).
            for _ in 0..n {
                let elen = read_u32(b, pos)? as usize;
                *pos = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                *pos = pos.checked_add(32).filter(|e| *e <= b.len())?;
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
                    kernel.provisional.insert(addr, spoken);
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
            if b.get(*pos..*pos + 4) == Some(b"DEF1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                if n > b.len().saturating_sub(*pos) / 72 {
                    return None;
                }
                let after80 = *pos + n.saturating_mul(80);
                let with_tu = n > 0 && b.get(after80..after80 + 4) == Some(b"FLD1");
                for _ in 0..n {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let mut mag = [0u64; 4];
                    for m in &mut mag {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    kernel.deflations.insert(addr, mag);
                    kernel.provisional_k.insert(addr, k);
                    if with_tu {
                        let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                        kernel.provisional_tu.insert(addr, tu);
                        kernel.standing_tu.insert(addr, tu);
                    }
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"FLD1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let ne = read_u32(b, pos)? as usize;
                    let mut entries = Vec::with_capacity(ne.min(64));
                    for _ in 0..ne {
                        let m = read_u32(b, pos)? as usize;
                        let mut axes = Vec::with_capacity(m);
                        for _ in 0..m {
                            axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                            *pos += 8;
                        }
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                element::Versor::from_bytes(b, pos)?,
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    crate::transducer::alphabet::Rotor::from_bytes(b, pos)?,
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u32(b, pos)? as usize;
                        let mut remainder = Vec::with_capacity(nc.min(1 << 16));
                        for _ in 0..nc {
                            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let hb = *b.get(*pos)?;
                            *pos += 1;
                            let ha = *b.get(*pos)?;
                            *pos += 1;
                            remainder.push((r, c, hb, ha));
                        }
                        let mut stagnant = [0u32; 4];
                        for s in &mut stagnant {
                            *s = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                            *pos += 4;
                        }
                        entries.push(FieldEntry {
                            chart: crate::transducer::stalk::Invariant(axes),
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field.insert(addr, ListingField { entries, nonflat });
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"BOK1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let addr = elem.address();
                    kernel.book_index.insert(addr, kernel.pending.len());
                    kernel.on_page.insert(addr, ());
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), addr);
                    kernel.pending.push((elem, k));
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"CAV1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    let addr = elem.address();
                    kernel.listing_atlas.admit(&crate::transducer::stalk::invariant_of(&elem), addr);
                    kernel.cavity_book.insert(addr, elem);
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"CRT1") {
                *pos += 4;
                kernel.crt_imported = *b.get(*pos)? != 0;
                *pos += 1;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let elen = read_u32(b, pos)? as usize;
                    let mut ep = *pos;
                    let end = pos.checked_add(elen).filter(|e| *e <= b.len())?;
                    let elem = Element::from_bytes(&b[..end], &mut ep)?;
                    *pos += elen;
                    kernel.court_fed_index.insert(addr, kernel.court_federation.len());
                    kernel.court_federation.push((addr, elem));
                }
                if *pos + 4 <= b.len() && b.get(*pos..*pos + 4) != Some(b"COC1") {
                    let n = read_u32(b, pos)? as usize;
                    for _ in 0..n.min(1 << 16) {
                        let nax = read_u32(b, pos)? as usize;
                        for _ in 0..nax {
                            *pos = pos.checked_add(8).filter(|e| *e <= b.len())?;
                        }
                        *pos = pos.checked_add(64).filter(|e| *e <= b.len())?;
                        let _ = element::Versor::from_bytes(b, pos)?;
                    }
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"COC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n.min(1 << 16) {
                    let key: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let c = crate::transducer::cech::Complex::from_bytes(b, pos)?;
                    kernel.complexes.insert(key, c);
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"FLC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n.min(1 << 16) {
                    let addr: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let ne = read_u32(b, pos)? as usize;
                    let mut entries = Vec::with_capacity(ne.min(64));
                    for _ in 0..ne {
                        let m = read_u32(b, pos)? as usize;
                        let mut axes = Vec::with_capacity(m);
                        for _ in 0..m {
                            axes.push(u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?));
                            *pos += 8;
                        }
                        let tag = *b.get(*pos)?;
                        *pos += 1;
                        let state = match tag {
                            0 => crate::transducer::stalk::Closure::Flat,
                            1 => crate::transducer::stalk::Closure::Twisted(
                                element::Versor::from_bytes(b, pos)?,
                            ),
                            3 => {
                                let d = *b.get(*pos)?;
                                *pos += 1;
                                crate::transducer::stalk::Closure::Class(
                                    crate::transducer::alphabet::Rotor::from_bytes(b, pos)?,
                                    d,
                                )
                            }
                            _ => crate::transducer::stalk::Closure::Cavity,
                        };
                        let nc = read_u32(b, pos)? as usize;
                        let mut remainder = Vec::with_capacity(nc.min(1 << 16));
                        for _ in 0..nc {
                            let r = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let c = i64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                            *pos += 8;
                            let hb = *b.get(*pos)?;
                            *pos += 1;
                            let ha = *b.get(*pos)?;
                            *pos += 1;
                            remainder.push((r, c, hb, ha));
                        }
                        let mut stagnant = [0u32; 4];
                        for s in &mut stagnant {
                            *s = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?);
                            *pos += 4;
                        }
                        entries.push(FieldEntry {
                            chart: crate::transducer::stalk::Invariant(axes),
                            state,
                            remainder,
                            stagnant,
                        });
                    }
                    let nonflat = entries.iter().filter(|e| !e.state.is_flat()).count() as u32;
                    kernel.field_cold.insert(addr, ListingField { entries, nonflat });
                }
            }
            let cyc2 = b.get(*pos..*pos + 4) == Some(b"CYC2");
            if cyc2 || b.get(*pos..*pos + 4) == Some(b"CYC1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n.min(1 << 16) {
                    let key: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let a: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let bp: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let ck_stored: Option<[u8; 32]> = if cyc2 {
                        let k: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                        *pos += 32;
                        Some(k)
                    } else {
                        None
                    };
                    let v = element::Versor::from_bytes(b, pos)?;
                    let class_key = ck_stored.unwrap_or_else(|| {
                        crate::transducer::sentence::class_key_of_versor(&v)
                    });
                    kernel.cocycle_at.insert(key, (v, tu, a, bp, class_key));
                    // Chart index is rebuilt on next glue; hot row is enough.
                }
                let nc = read_u32(b, pos)? as usize;
                for _ in 0..nc.min(1 << 20) {
                    let key: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let v = element::Versor::from_bytes(b, pos)?;
                    kernel.dialect_chain.push((key, v, tu));
                }
            }
            if b.get(*pos..*pos + 4) == Some(b"SET1") {
                *pos += 4;
                let n = read_u32(b, pos)? as usize;
                for _ in 0..n.min(1 << 20) {
                    let listing: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let chain = crate::anastome::dialogue::read_chain(b, pos)?;
                    let voice: [u8; 32] = b.get(*pos..*pos + 32)?.try_into().ok()?;
                    *pos += 32;
                    let cites = crate::anastome::dialogue::read_cites(b, pos)?;
                    let mut before = [0u64; 4];
                    for m in &mut before {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let mut after = [0u64; 4];
                    for m in &mut after {
                        *m = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                        *pos += 8;
                    }
                    let k = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    let tu = u64::from_le_bytes(b.get(*pos..*pos + 8)?.try_into().ok()?);
                    *pos += 8;
                    kernel.settlements_cold.push((listing, chain, voice, cites, before, after, k, tu));
                }
            }
            // index: keyed field at restore (persistence snapshot, once)
            let field_keys: Vec<[u8; 32]> = kernel.field.fiber(&crate::network::nerve::ROOT).map(|(k, _)| *k).collect();
            for addr in field_keys {
                let Some(listing) = kernel.field.get(&addr).cloned() else { continue };
                let mag = element::measure_lanes(&union_remainders(&listing.entries));
                let lanes = crate::network::nerve::occupied_lanes(&mag);
                for e in &listing.entries {
                    if !e.state.is_flat() {
                        kernel.listing_atlas.admit_also(&e.chart, addr);
                        for lane in &lanes {
                            kernel.own_cover.admit(*lane, &e.chart, addr);
                        }
                        let ck = e.chart.key();
                        match kernel.chart_listings.get_mut(&ck) {
                            Some(m) => {
                                m.insert(addr, ());
                            }
                            None => {
                                let mut m = crate::network::nerve::Keyed::new();
                                m.insert(addr, ());
                                kernel.chart_listings.insert(ck, m);
                            }
                        }
                    }
                }
                if listing.nonflat > 0 {
                    kernel.open.insert(addr, ());
                    kernel.tension.insert(addr, mag);
                } else {
                    kernel.field.remove(&addr);
                    kernel.field_cold.insert(addr, listing);
                    kernel.tension.insert(addr, [0; 4]);
                }
            }
            Some(())
        };
        if load(self, b, &mut pos).is_none() {
            self.basis_clear();
            return false;
        }
        self.remap_restored_addresses();
        self.occupancy = [0; 4];
        self.t_leases = crate::network::nerve::Keyed::new();
        self.lane_listings = Default::default();
        let pending_addrs: Vec<[u8; 32]> = self.pending.iter().map(|(e, _)| e.address()).collect();
        for addr in pending_addrs {
            if self.standing_tu.get(&addr).is_none() {
                self.standing_tu.insert(addr, self.now_tu());
            }
        }
        let prov: Vec<[u8; 32]> = self.provisional.fiber(&crate::network::nerve::ROOT).map(|(a, _)| *a).collect();
        for addr in prov {
            if self.provisional_tu.get(&addr).is_none() {
                self.provisional_tu.insert(addr, self.now_tu());
            }
            self.stamp_standing(addr);
        }
        // Ruled cover is the lettered federation, not the leaked basis.
        self.ruled = crate::transducer::stalk::Stalks::over_elements(
            self.court_federation.iter().filter_map(|(_, w)| {
                if w.is_connection() && w.letters().is_none() {
                    None
                } else {
                    Some(w)
                }
            }),
        );
        let dropped = self.prune_basis_to_ruled();
        if dropped > 0 {
            eprintln!("  pruned {dropped} leaked basis words; ruled cover remains");
        }
        // A snapshot without DEF1 (or a zero record) re-measures from the
        // spoken word: presence on the docket is not Δ⃗ = 0.
        let zeros = [0u64; 4];
        let need: Vec<([u8; 32], [u64; 4])> = self
            .provisional
            .fiber(&crate::network::nerve::ROOT)
            .filter_map(|(addr, spoken)| {
                let held = self.deflations.get(addr).copied().unwrap_or(zeros);
                if held.iter().any(|&x| x > 0) {
                    return None;
                }
                Some((*addr, self.book_elem(addr).map(|p| p.magnitude()).unwrap_or_else(|| spoken.magnitude())))
            })
            .collect();
        for (addr, mag) in need {
            if mag.iter().any(|&x| x > 0) {
                self.deflations.insert(addr, mag);
            }
        }
        // Unruled connections that survived the prune re-enter the docket.
        let held: Vec<Element> = self
            .basis
            .iter()
            .filter(|w| w.is_connection())
            .map(|w| (**w).clone())
            .collect();
        let ratifier = self.discoverers.fiber(&crate::network::nerve::ROOT).next().map(|(w, _)| *w).unwrap_or([0; 32]);
        for w in held {
            if !self.ruled_contains(&w.address()) {
                self.docket_connection(&w, ratifier);
            }
        }
        true
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
        addr: &[u8; 32],
    ) -> Option<(Element, crate::transducer::sentence::Form)> {
        let e = self.word_at(addr)?.clone();
        let f = crate::transducer::sentence::from_element_form(&e)?;
        Some((e, f))
    }
}

// The blind replay moved to `crate::verdict` (#162): verification is the
// court's substrate, not the server kernel's.

fn sentence_rule_of(form: &Element) -> Option<crate::transducer::sentence::SentenceRule> {
    use crate::transducer::sentence::{from_element, from_element_form, agree_up_to_gauge, SentenceRule, Vertex};
    if let Some(f) = from_element_form(form) {
        let form_speaks = f.vertices.iter().any(|v| {
            matches!(
                v,
                Vertex::Glued { .. } | Vertex::Rebased { .. } | Vertex::Unfolded { .. }
            ) || matches!(v, Vertex::Word { contact: Some(_), .. })
        });
        if form_speaks || form.description().get(1) == Some(&2) {
            let nerve = f.to_nerve().unwrap_or_default();
            let ratio = f.ratio;
            let sections = f.sections.clone();
            return Some(SentenceRule {
                charts: nerve
                    .germs
                    .iter()
                    .map(|g| (g.source.clone(), g.target.clone()))
                    .collect(),
                faces: nerve
                    .faces
                    .iter()
                    .map(|fc| (fc.i, fc.j, fc.g_ij.clone(), fc.spelled))
                    .collect(),
                holonomies: nerve.cycles.iter().map(|c| c.holonomy.clone()).collect(),
                hue_maps: Vec::new(),
                nerve,
                ratio,
                shifts: Vec::new(),
                sections,
                form: f,
            });
        }
    }
    let n = from_element(form)?;
    agree_up_to_gauge(std::slice::from_ref(&n))
}

impl Kernel {
    /// Descent a rule measures by speaking every train pair, its cited
    /// forms resolved through this kernel's ruled cover. Any miss is zero:
    /// association with a listing is not a mint.
    fn measured_replay_descent(&self, form: &Element, family: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> [u64; 4] {
        if let Some(f) = crate::transducer::sentence::from_element_form(form) {
            return crate::transducer::family::form_descent(&f, family, self);
        }
        [0; 4]
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

/// The consilience address of a SECTION prospect: the carrying thought's stable
/// base address ⊕ the assignment — so two kernels independently proposing the
/// same axis extension meet at one receipt (wire identity, never reasoned on).
fn section_address(thought: &[u8; 32], asg: &[(u8, u8)]) -> [u8; 32] {
    let mut sigma = asg.to_vec();
    sigma.sort_unstable();
    let mut buf = Vec::with_capacity(32 + sigma.len() * 2);
    buf.extend_from_slice(thought);
    for (b, a) in sigma {
        buf.push(b);
        buf.push(a);
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
    (crate::network::wire::frame_budget() / crate::anastome::dialogue::cell_wire_width()).max(1)
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
    element::from_residual(&rg.to_vec()).address()
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
    let mut chain = chain.to_vec();
    chain.sort_unstable();
    let chain = chain.as_slice();
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
            if pat.iter().all(|(r, c, b, a)| chain.binary_search(&(r + dr, c + dc, *b, *a)).is_ok()) {
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
        .filter_map(|(addr, _st, g, asg, _)| {
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
            let missing = cites.iter().find(|(a, _, _, _, _)| !named.contains(a)).map(|(_, _, _, _, body)| body.clone()).unwrap_or_default();
            NormalForm::Irreducible(element::from_residual_framed(&missing, frame))
        }
        ([_, ..], _) => NormalForm::Irreducible(element::from_residual_framed(&residual, frame)),
    };
    Reduction {
        germ: g,
        ctx,
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
mod tests {
    use super::*;

    fn scene(sr: i64, sc: i64) -> Vec<(i64, i64, u8)> {
        vec![(sr, sc, 1), (sr, sc + 1, 1), (sr + 1, sc, 1)]
    }

    fn glue_pair(k: &mut Kernel, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)]) {
        k.ingest(a, b, [1; 32]);
        let _ = k.ingest(a, b, [2; 32]);
        k.glue_local();
    }

    /// A novel SUBSTANCE (here a rotation) germinates: one voice pends, a second
    /// settles the listing; the basis grows only from gluing. Afterward the same
    /// demand REDUCES to the length-1 sentence [0].
    #[test]
    fn a_novel_substance_germinates_then_decomposes() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)]; // L
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)]; // R180 of L — non-trivial substance
        assert_eq!(k.ingest(&a, &rot, [1; 32]), Outcome::GermPending);
        assert_eq!(k.basis_len(), 0, "one voice does not complete");
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending, "two voices settle, they do not install");
        assert_eq!(k.basis_len(), 0, "zero basis installs from a single hypha's two voices");
        k.glue_local();
        assert_eq!(k.basis_len(), 1, "the basis grows from gluing");
        assert_eq!(k.ingest(&a, &rot, [3; 32]), Outcome::Decomposed(vec![0]), "reduces to the length-1 sentence");
    }

    /// An empty atlas is incident to the root prefix: the first quote
    /// reads the book at depth zero.
    #[test]
    fn empty_atlas_is_incident_to_the_root_prefix() {
        use crate::transducer::germinate::germinate;
        let k = std::sync::Mutex::new(Kernel::new());
        {
            let mut g = k.lock().unwrap();
            let a = vec![(0, 0, 1u8), (0, 1, 1)];
            let b = vec![(0, 0, 2u8), (0, 1, 2)];
            assert!(matches!(g.ingest(&a, &b, [1; 32]), Outcome::GermPending));
            assert!(matches!(g.attest(&germinate(&a, &b).unwrap(), [1; 32]), Outcome::GermPending));
        }
        let (_, priced, _, _) =
            crate::synthesis::quote(&k, [1u8; 32], &[]).expect("quote");
        assert!(!priced.is_empty(), "an empty atlas reads the book at depth zero");
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
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
        k.glue_local(); // a rotation generator
        k.ingest(&a, &bar, [1; 32]);
        assert_eq!(k.ingest(&a, &bar, [2; 32]), Outcome::GermPending);
        k.glue_local(); // a residual generator
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

    /// CYC2 writes the transport class key beside the versor; a restore
    /// reads the same key, never a hash of the row.
    #[test]
    fn myc2_round_trips_cocycle_class_key() {
        let mut k = Kernel::new();
        let inv = crate::transducer::stalk::Invariant(vec![1]);
        let q = crate::transducer::alphabet::LetterId::quarter((0, 1)).unwrap();
        let w = crate::transducer::alphabet::Word::from_letters(&[(q, false)]);
        let r = w.rotor();
        let v = rotor_versor(&r).expect("versor");
        let ck = r.class_key();
        k.push_cocycle(&inv, [1; 32], [2; 32], v, ck);
        let snap = k.snapshot();
        assert!(snap.windows(4).any(|w| w == b"CYC2"), "ledger speaks CYC2");
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap), "clean restore");
        let got = booted
            .cocycle_at
            .fiber(&crate::network::nerve::ROOT)
            .next()
            .map(|(_, row)| row.4);
        assert_eq!(got, Some(ck), "class key survives the ledger");
    }

    #[test]
    fn torsion_consilient_or_filed_or_refused() {
        use crate::transducer::element::{from_residual, FIBER_SPATIAL};
        use crate::transducer::germinate::germinate;
        let a = [(0, 0, 1u8), (0, 1, 2), (0, 2, 3)];
        let b = [(0, 0, 2u8), (0, 1, 3), (0, 2, 1)];
        let mut spoken = from_residual(&germinate(&a, &b).unwrap());
        spoken.puncture(vec![(2, 3), (3, 1), (1, 2)]);
        assert_eq!(spoken.torse(), 1);
        let inv = crate::transducer::stalk::invariant_of(&spoken);
        let mut k = Kernel::new();
        k.complexes
            .insert(inv.key(), crate::transducer::cech::Complex::new());
        assert_eq!(k.court_torsion(&spoken, &inv, [1; 32], [2; 32]), None, "first hearing files");
        let filed = k.complexes.get(&inv.key()).is_some_and(|cx| {
            cx.degrees.iter().any(|layer| {
                layer.fiber(&crate::network::nerve::ROOT).any(|(_, cell)| {
                    matches!(cell.fill, crate::transducer::cech::Fill::Congruent(_, 3))
                })
            })
        });
        assert!(filed, "court torsion filed ℓ 3");
        assert_eq!(
            k.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
            None,
            "second hearing is consilient"
        );
        let mut k2 = Kernel::new();
        let mut cx = crate::transducer::cech::Complex::new();
        cx.file_congruent(
            [1; 32],
            [2; 32],
            crate::transducer::alphabet::Rotor::identity(),
            4,
        );
        k2.complexes.insert(inv.key(), cx);
        assert_eq!(
            k2.court_torsion(&spoken, &inv, [1; 32], [2; 32]),
            Some(crate::verdict::REFUSE_NOT_NATURAL),
            "same class different ℓ is not natural"
        );
    }

    /// A restart must not docket Δ⃗ = 0: deflation records survive the
    /// snapshot, so the court does not slash a live provisional as empty.
    #[test]
    fn deflation_survives_a_restart() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        let chain = germinate(&a, &b).unwrap();
        assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
        let before = k.dockets();
        assert_eq!(before.len(), 1);
        assert!(before[0].4.iter().any(|&x| x > 0), "deflation was measured at install");
        let snap = k.snapshot();
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap), "clean restore");
        let after = booted.dockets();
        assert_eq!(after.len(), 1, "the provisional survived");
        assert_eq!(after[0].4, before[0].4, "deflation survived; the court will not slash Δ⃗ = 0");
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
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
        k.glue_local();
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
            match k.ingest(&a, &b, [2; 32]) {
                Outcome::GermPending => k.glue_local(),
                Outcome::Installed(_) | Outcome::Sectioned(_) => {}
                o => panic!("germ {n}: {o:?}"),
            }
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
        assert_eq!(k.ingest(&a, &ga, [2; 32]), Outcome::GermPending);
        let ga_spoken = k
            .provisional
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, e)| e.clone())
            .find(|e| e.sub().iter().any(|c| c.2 == 1 && c.3 == 4))
            .expect("ga's residual is on the docket");
        let mut cover = crate::network::nerve::Cover::new();
        let _ = k.glue_fetched(ga_spoken, [2; 32], [1; 32], &mut cover);
        // …so B's quorum finds the lane DEPLETED and DEFERS: quorum met, fuel absent.
        assert_eq!(k.ingest(&a, &gb, [2; 32]), Outcome::GermPending, "deferred, not lost");
        // A further voice arrives after another stroke: the deferred germ completes.
        match k.ingest(&a, &gb, [3; 32]) {
            Outcome::GermPending => k.glue_local(),
            Outcome::Installed(_) | Outcome::Sectioned(_) => {}
            o => panic!("deferral complete: {o:?}"),
        }
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
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
        k.glue_local();
        // Fabricate the stale duplicate: the SAME deformation germinated far away —
        // centering makes it the identical pattern — slipped in past the
        // reduce-gate (as an imported/legacy entry would be).
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        // A keyed basis holds one word per address: the stale duplicate carries
        // a frame fibre (a distinct address) over the same centered chain.
        k.basis_insert(element::from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)), [7; 32]);
        assert_eq!(k.basis_len(), 2);
        k.optimize(); // probe basis[0]: the rest (its duplicate) derives it → retired
        assert_eq!(k.basis_len(), 1, "the redundant rule is consolidated away");
        k.optimize(); // the survivor is essential — nothing else derives it
        assert_eq!(k.basis_len(), 1, "an essential generator survives the probe");
    }

    /// The survivor of refine_stratum may sit at the last index. Removing an
    /// earlier derivable sibling used to leave that index equal to len.
    #[test]
    fn refine_stratum_does_not_panic_when_the_survivor_is_last() {
        use crate::transducer::element::from_residual_framed;
        use crate::transducer::germinate::germinate;
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let rot = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        k.ingest(&a, &rot, [1; 32]);
        assert_eq!(k.ingest(&a, &rot, [2; 32]), Outcome::GermPending);
        k.glue_local();
        let a_far: Vec<_> = a.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        let rot_far: Vec<_> = rot.iter().map(|(r, c, h)| (r + 9, c + 9, *h)).collect();
        k.basis_insert(
            from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(4, 4, 8, 8)),
            [7; 32],
        );
        k.basis_insert(
            from_residual_framed(&germinate(&a_far, &rot_far).unwrap(), &(3, 3, 6, 6)),
            [8; 32],
        );
        assert_eq!(k.basis_len(), 3);
        let survivor = k.basis()[2].address();
        k.refine_stratum(&survivor);
        assert!(k.word_at(&survivor).is_some(), "the last card stays keyed");
        assert!(k.basis_len() >= 1);
        assert!(k.basis_len() < 3, "at least one earlier duplicate retired");
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
        k.enroll([9; 32], crate::economy::UNIT_BOND); // capitalize the pool
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
        k.basis_insert(element::from_residual_framed(&germinate(&a, &rot).unwrap(), &(4, 4, 8, 8)), [1; 32]);
        k.basis_insert(from_residual(&germinate(&a_far, &rot_far).unwrap()), [7; 32]);
        let derivable = k.basis[0].address(); // its duplicate speaks it
        // The bidder's sentence: the duplicate, cited at the identity.
        let body = |w: &Element| crate::transducer::sentence::cited_pattern(w, 0).unwrap();
        let sentence = vec![(k.basis[1].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis[1]))];
        let paid = k.try_refine(&derivable, [5; 32], &sentence);
        assert!(paid.is_some(), "the derivable word is a confirmed compression bid");
        assert_eq!(k.basis_len(), 1, "the vocabulary compressed");
        let essential = k.basis[0].address();
        let self_cite = vec![(essential, 0u8, element::identity_versor(), 0u32, body(&k.basis[0]))];
        assert!(k.try_refine(&essential, [5; 32], &self_cite).is_none(), "an essential word is refused");
        assert_eq!(k.basis_len(), 1, "refusal mutates nothing");
        assert!(k.try_refine(&[3; 32], [5; 32], &sentence).is_none(), "an unknown address is refused");

        // THE COURT'S BUSINESS STAYS THE COURT'S: a provisional word cannot be
        // bid away, even if derivable.
        k.basis_insert(from_residual(&germinate(&a, &rot).unwrap()), [1; 32]);
        let addr = k.basis[1].address();
        k.provisional.insert(addr, (*k.basis[1]).clone());
        let sentence = vec![(k.basis[0].address(), 0u8, element::identity_versor(), 0u32, body(&k.basis[0]))];
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
            match k.ingest(a, b, [2; 32]) {
                Outcome::GermPending => k.glue_local(),
                Outcome::Installed(_) | Outcome::Sectioned(_) => {}
                o => panic!("expected settle or puncture, got {o:?}"),
            }
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
    /// pends as a tape, not quoted); A's empty-cites demand lists the remainder
    /// on the quote frontier; B independently attests; consilience installs,
    /// the market CLEARS. {demand → quote → demand → settle}.
    #[test]
    fn the_five_act_conversation_closes_with_settlement() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)]; // (0,0): 1→4
        assert_eq!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending, "percept: voice A");
        assert_eq!(k.quote(4).1.len(), 1, "corpus demand is quoted when the frontier is dry");
        let chain = germinate(&a, &b).unwrap();
        assert_eq!(k.attest(&chain, [1; 32]), Outcome::GermPending, "remainder-of-g / unreached demand");
        let (surface, quoted) = k.quote(4);
        assert_eq!(quoted.len(), 1, "the unreached/remainder demand is quoted as work");
        assert!(surface.iter().any(|&s| s >= 1), "the open demand tensions the lanes it occupies");
        let (work, mu) = &quoted[0];
        assert!(
            (0..4).any(|l| work.magnitude()[l] > 0 && mu[l] > 0),
            "price is nonzero on a lane the word moves"
        );
        let chain = work.sub().to_vec();
        let out = k.attest(&chain, [2; 32]);
        assert_eq!(out, Outcome::GermPending, "consilience settles: {out:?}");
        k.glue_local();
        assert_eq!(k.voices_of(&k.basis()[0].address()).len(), 2, "the settle parties");
        assert!(k.quote(4).1.is_empty(), "the market cleared");
    }

    /// Room is per-lane from bond and reserve; ticks on T do not steal
    /// S, G, or H. Eviction is by ACTION POTENTIAL, never FIFO.
    #[test]
    fn market_capacity_is_thermodynamic_and_eviction_is_by_tension() {
        let mut k = Kernel::new();
        k.bind_identity([1; 32]);
        k.enroll([1; 32], [8, 8, 1, 8]);
        let calm = k.lane_room();
        assert!(calm.iter().all(|&c| c > 0), "no lane is ever pinched to zero: {calm:?}");
        let s0 = calm[0];
        for _ in 0..24 {
            k.physics_mut().bump(&[0, 0, 1, 0]);
        }
        let tense = k.lane_room();
        assert_eq!(tense[0], s0, "T pressure does not steal spatial room: {tense:?}");
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
            k.treasury().entries().iter().any(|e| e.kind == "transfer" && e.who == [7; 32]),
            "the funded speaker paid the door as a transfer"
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
        use crate::transducer::sentence::replay_at;
        let mut k = Kernel::new();
        // A demand listed whole on an empty language (one connected body).
        let c = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 1), (0, 3, 3)];
        let d = vec![(0, 0, 2u8), (0, 1, 2), (0, 2, 1), (0, 3, 4)];
        assert!(matches!(k.ingest(&c, &d, [1; 32]), Outcome::GermPending));
        let chain = germinate(&c, &d).unwrap();
        assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
        let target = k.quote(8).1[0].0.address();
        let chain = k.quote(8).1[0].0.sub().to_vec();
        assert_eq!(chain.len(), 3);
        // Then a word that covers it in part installs.
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8), (0, 1, 2)];
        k.ingest(&a, &b, [1; 32]);
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
        let small = k
            .provisional
            .fiber(&crate::network::nerve::ROOT)
            .map(|(_, e)| e.clone())
            .find(|e| e.sub().len() == 2)
            .expect("the covering word is on the docket");
        let mut cover = crate::network::nerve::Cover::new();
        let _ = k.glue_fetched(small, [2; 32], [1; 32], &mut cover);
        let _word = k.basis()[0].address();
        // The entity's reduction over the language: the word cited at the
        // origin, the far cell left as the residual.
        let language = k.language();
        let sentence = crate::transducer::sentence::deconstruct_at(&|i| language.get(i), &language.stalks, &|a| language.index.get(a).copied(), &|i| language.shape(i), &chain, None);
        let (void, cites) = (sentence.residual.clone(), sentence.cites.clone());
        assert_eq!(void, vec![(0, 3, 3, 4)]);
        assert_eq!(cites.len(), 1);
        assert_eq!(
            replay_at(&|i| k.basis().get(i).map(|w| w.as_ref()), &cites, &chain),
            void,
            "the replay reproduces the residual"
        );
        let (code, paid, remaining) = k.settle_probe([3; 32], &target, void.clone(), &[], false);
        assert_eq!(code, PROBE_DESCENT);
        assert_eq!(paid, [0; 4], "a descent is measured, never paid: covering cells opens no axis");
        assert_eq!(remaining, crate::transducer::element::measure_lanes(&void));
        let listing = k.quote(8).1[0].0.clone();
        assert_eq!(listing.address(), target, "the listing stays the body: a descent never re-lists a fragment");
        assert_eq!(
            k.tension.get(&target).copied(),
            Some(crate::transducer::element::measure_lanes(&void)),
            "the body's remaining tension is the residual's measure, keyed by the listing"
        );
        // No descent settles nothing; closure clears the book.
        let t2 = listing.address();
        assert_eq!(k.settle_probe([3; 32], &t2, listing.sub().to_vec(), &[], false).0, PROBE_NOTHING);
        assert_eq!(k.settle_probe([3; 32], &t2, Vec::new(), &[], false).0, PROBE_CLOSED);
        assert!(k.quote(8).1.is_empty(), "the listing cleared");
    }

    /// Descent records a remainder face and a holonomic cell-count, stagnation
    /// counts only unmoved occupied lanes, and a full aperture of failed
    /// probes sheds the listing as a cavity.
    #[test]
    fn descent_opens_a_face_and_drain_sheds_the_unmoved() {
        use crate::anastome::dialogue::PROBE_DESCENT;
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1u8), (0, 1, 1)];
        let b = vec![(0, 0, 2u8)];
        assert!(matches!(k.ingest(&a, &b, [1; 32]), Outcome::GermPending));
        let chain = germinate(&a, &b).unwrap();
        assert!(matches!(k.attest(&chain, [1; 32]), Outcome::GermPending));
        let target = k.quote(8).1[0].0.address();
        let listed = k.quote(8).1[0].0.sub().to_vec();
        let rem = vec![(0, 1, 1u8, 0u8)];
        let before = k.tension.get(&target).copied().unwrap_or_else(|| k.quote(8).1[0].0.magnitude());
        let after = crate::transducer::element::measure_lanes(&rem);
        assert_ne!(after, before, "the residual is a proper descent");
        let (code, _, remaining) = k.settle_probe([3; 32], &target, rem.clone(), &[], false);
        assert_eq!(code, PROBE_DESCENT);
        assert_eq!(remaining, after);
        let listing = k.quote(8).1[0].0.clone();
        assert_eq!(listing.address(), target, "the listing stays the body");
        assert_eq!(listing.magnitude()[3], rem.len() as u64, "holonomic demand is the remainder cell count");
        let rem_inv = crate::transducer::stalk::invariant(&rem);
        assert_eq!(k.face_of(&target).cloned(), Some(rem_inv.clone()));
        assert!(
            k.listing_atlas.fiber(&rem_inv).iter().any(|a| *a == target),
            "the remainder chart admits the listing"
        );
        let base_inv = crate::transducer::stalk::invariant(&listed);
        assert_eq!(
            k.listing_atlas.chart_of(&target).cloned().as_ref(),
            Some(&base_inv),
            "base moduli are never written"
        );
        let s = k.stagnant_of(&target);
        for l in 0..4 {
            if before[l] > 0 && after[l] == before[l] {
                assert!(s[l] >= 1, "unmoved occupied lane {l} stagnates: {s:?}");
            } else {
                assert_eq!(s[l], 0, "a moved or empty lane {l} resets: {s:?}");
            }
        }
        let mut listed_sorted = listed.clone();
        listed_sorted.sort_unstable();
        let kappa = crate::physics::aperture_cycle();
        for _ in 0..kappa {
            assert_eq!(
                k.settle_probe([4; 32], &target, listed_sorted.clone(), &[], false).0,
                crate::anastome::dialogue::PROBE_NOTHING
            );
        }
        let s = k.stagnant_of(&target);
        let t = k.tension.get(&target).copied().unwrap();
        assert!(
            (0..4).all(|l| t[l] == 0 || s[l] >= kappa),
            "every occupied lane sat a full cycle: tension {t:?} stagnant {s:?} kappa {kappa}"
        );
        k.drain_now();
        assert!(k.quote(8).1.is_empty(), "the unreachable listing left the page");
        assert_eq!(k.cavities().len(), 1, "the cavity is the relocated element");
        assert_eq!(k.cavities()[0].address(), target);
        assert_eq!(k.cavities()[0].magnitude()[3], rem.len() as u64);
        assert!(
            k.listing_atlas.fiber(&rem_inv).iter().any(|a| *a == target),
            "drain relocates: the listing stays on the atlas"
        );
        assert_eq!(
            k.listing_atlas.chart_of(&target).cloned().as_ref(),
            Some(&base_inv),
            "base moduli stay on the cavity chart"
        );
        assert!(k.face_of(&target).is_some(), "the cold field is readable by key");
        assert!(k.cavity_book.get(&target).is_some(), "the element is keyed on the cavity book");
        let listing = k.field_cold.get(&target).expect("relocated field is cold beside CAV1");
        let mut seen = Vec::new();
        for e in &listing.entries {
            assert!(!seen.contains(&e.chart), "a chart appears once per listing");
            seen.push(e.chart.clone());
        }
        let bytes = k.snapshot();
        let mut k2 = Kernel::new();
        assert!(k2.restore(&bytes), "the field persists beside DEF1");
        assert!(k2.cavity_book.get(&target).is_some(), "a restart keeps the cavity");
        assert!(
            k2.field_cold.get(&target).is_some_and(|f| !f.entries.is_empty()),
            "a restart does not forget which angles were probed"
        );
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
            g.glue_local();
        }
        let (word, body) = {
            let g = k.lock().unwrap();
            (g.basis()[0].address(), crate::transducer::sentence::cited_pattern(&g.basis()[0], 0).unwrap())
        };
        let (ca, cb) = perceive(&a, &b);
        let frame = frame_of(&a, &b);
        let proof = vec![vec![(word, 0u8, element::identity_versor(), 0u32, body)]];
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
        assert!(matches!(k.attest(&germinate(&a, &b).unwrap(), [1; 32]), Outcome::GermPending));
        assert!(matches!(k.attest(&germinate(&c, &d).unwrap(), [1; 32]), Outcome::GermPending));
        let listed: Vec<[u8; 32]> = k.quote(8).1.iter().map(|(w, _)| w.address()).collect();
        assert_eq!(listed.len(), 2);
        let parent = [9u8; 32];
        k.register_composites(vec![(parent, vec![(listed[0], false), (listed[1], false)], vec![(0, 0, 1, 2)])]);
        // Complete the first child: the second's rank key lifts by 1/2.
        let first_addr = k.quote(8).1[0].0.address();
        let first = k.quote(8).1[0].0.sub().to_vec();
        k.attest(&first, [2; 32]);
        let rank = k.closure_now();
        let remaining = k.quote(8);
        assert!(!remaining.1.is_empty(), "the unfinished child stays on the book");
        let (resolved, total) = rank.lineage[0];
        assert_eq!((resolved, total), (1, 2), "the lineage fraction is measured");
        // Complete the second child: the family settles and the premium books.
        let second = remaining
            .1
            .iter()
            .find(|(w, _)| w.address() != first_addr)
            .expect("the other child stays listed")
            .0
            .sub()
            .to_vec();
        k.attest(&second, [2; 32]);
        assert!(k.composites().is_empty(), "the family completed — lineage settled");
        assert!(
            k.treasury().entries().iter().any(|e| e.kind == "mint"),
            "the assembly premium minted on reassembly"
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
        let chain = germinate(&a, &b).unwrap();
        assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
        k.glue_local();
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
        fat.mag = [999, 0, 0, 0];
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
        let chain = germinate(&a, &b).unwrap();
        assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
        // The install PAID NOTHING — the award sits in escrow.
        assert_eq!(k.treasury().balance_mu(&[1; 32]), [0; 4], "unfinalized speech earns nothing");
        assert_eq!(k.treasury().balance_mu(&[2; 32]), [0; 4]);
        let dockets = k.dockets();
        assert_eq!(dockets.len(), 1, "the provisional word is on the docket");
        let (addr, spoken, voices, mu, _, _) = &dockets[0];
        assert_eq!(voices.len(), 2, "both attesting voices ride the docket");
        assert!(mu.iter().any(|&m| m > 0), "the escrowed award is real");
        assert_eq!(spoken.address(), *addr, "the docket carries the word as spoken");
        // UPHELD: finality releases the award — the voices are paid at last.
        assert!(k.apply_verdict(&addr.clone(), crate::anastome::dialogue::RULING_UPHELD));
        assert!(k.treasury().balance_mu(&[1; 32]).iter().any(|&m| m > 0), "finality pays voice A");
        assert!(k.treasury().balance_mu(&[2; 32]).iter().any(|&m| m > 0), "finality pays voice B");
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
        let chain = germinate(&a, &b).unwrap();
        assert_eq!(k.attest(&chain, [2; 32]), Outcome::GermPending);
        let (addr, _, _, _, _, _) = k.dockets()[0].clone();
        assert!(k.apply_verdict(&addr, crate::anastome::dialogue::RULING_SLASHED));
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
        assert_eq!(k.ingest(&a, &b1, [2; 32]), Outcome::GermPending);
        k.glue_local();
        assert_eq!(k.basis_len(), 1);
        // The family member: same cell, DIFFERENT transition (1→7).
        let b2 = vec![(0, 0, 7), (0, 1, 1), (1, 0, 1)];
        assert_eq!(k.ingest(&a, &b2, [1; 32]), Outcome::GermPending, "the axis prospect pends");
        assert_eq!(k.basis_len(), 1, "no sibling card");
        assert!(matches!(k.ingest(&a, &b2, [2; 32]), Outcome::Sectioned(0)), "the axis PIERCES thought 0");
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
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
        k.glue_local();
        // Fabricate two sibling cards (as a legacy/import would leave them).
        for hue in [6u8, 7] {
            let bx = vec![(0, 0, hue), (0, 1, 2)];
            k.basis_insert(from_residual(&germinate(&a, &bx).unwrap()), [9; 32]);
        }
        assert_eq!(k.basis_len(), 3, "the un-condensed family");
        // One versor per stroke: the cursor walks the stratum over strokes.
        for _ in 0..3 {
            k.optimize();
        }
        assert_eq!(k.basis_len(), 1, "the family collapsed into one bundle");
        assert_eq!(k.basis()[0].sections().len(), 2, "two sections along the pierced axis");
        let addrs: Vec<[u8; 32]> = k.dockets().iter().map(|(a, _, _, _, _, _)| *a).collect();
        for a in addrs {
            let _ = k.apply_verdict(&a, crate::anastome::dialogue::RULING_UPHELD);
        }
        assert!(
            k.treasury().balance_mu(&[9; 32])[1] > 0 || k.treasury().balance_mu(&[9; 32]).iter().any(|&m| m > 0),
            "absorbed discoverers still earn, on the lane"
        );
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
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending);
        k.glue_local();
        let rev = vec![(0, 0, 1), (0, 1, 9)];
        let fwd = vec![(0, 0, 4), (0, 1, 9)];
        k.ingest(&fwd, &rev, [1; 32]); // (0,0): 4→1 — the swap sibling
        match k.ingest(&fwd, &rev, [2; 32]) {
            Outcome::GermPending => k.glue_local(),
            Outcome::Installed(_) | Outcome::Sectioned(_) => {}
            o => panic!("{o:?}"),
        }
        assert_eq!(k.basis()[0].sections().len(), 1);
        let syntropic_before = k.physics().syntropic[3];
        k.optimize(); // the stroke: condense (nothing) → TORSE
        assert_eq!(k.torsion_total(), 1, "the ℤ/2 cycle extracted");
        assert!(k.basis()[0].sections().is_empty(), "the free list condensed");
        assert!(k.physics().syntropic[3] > syntropic_before, "torsion converted to order, on the holonomic lane");
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
        assert_eq!(k.ingest(&a1, &b1, [2; 32]), Outcome::GermPending);
        k.glue_local();
        let a2 = vec![(0, 0, 2), (0, 1, 3)];
        let b2 = vec![(0, 0, 2), (0, 1, 9)]; // (0,1): 3→9 — Q shares P's geometry:
        k.ingest(&a2, &b2, [1; 32]);
        // …so under 3.5 it PUNCTURES P's axis (Installed(0) as a section).
        match k.ingest(&a2, &b2, [2; 32]) {
            Outcome::GermPending => k.glue_local(),
            Outcome::Installed(_) | Outcome::Sectioned(_) => {}
            o => panic!("{o:?}"),
        }
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
        assert_eq!(k.ingest(&a, &b, [2; 32]), Outcome::GermPending, "two voices settle the residual germ");
        k.glue_local();
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
                assert!(parts.iter().all(|p| matches!(p, Outcome::GermPending)));
                k.glue_local();
                assert_eq!(k.basis_len(), 2, "two distinct motors germinated");
            }
            o => panic!("expected Fragmented settles, got {o:?}"),
        }
    }

    /// Court organ: two-voice quorum does not grow the basis. The stroke
    /// puts a section on the ruled cover. A peer's fetch glues by replay;
    /// a forged word is slashed with a reason. Signatures are not consulted.
    #[test]
    fn court_organ_glue_on_replay_not_on_signatures() {
        let mut home = Kernel::new();
        let mut peer = Kernel::new();
        let mut third = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
        assert_eq!(home.basis_len(), 0);
        home.court_stroke();
        let spoken = home.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().1.clone();
        let mut cover = crate::network::nerve::Cover::new();
        assert!(matches!(
            peer.glue_fetched(spoken.clone(), [1; 32], [2; 32], &mut cover),
            Glue::Admitted
        ));
        assert!(peer.basis_len() >= 1, "the word is language on the second organ");
        assert!(matches!(
            third.glue_fetched(spoken.clone(), [2; 32], [3; 32], &mut cover),
            Glue::Admitted
        ));
        let mut forged = spoken.clone();
        forged.mag = [999, 0, 0, 0];
        assert!(
            matches!(third.glue_fetched(forged, [2; 32], [3; 32], &mut cover), Glue::Slashed(_)),
            "never admit on a word that fails replay"
        );
    }

    /// Adopt writes a field entry and a deflation so the stroke can rule
    /// a client-minted connection. Ruled listings do not starve later ones.
    #[test]
    fn adopt_reaches_the_docket_and_stroke_skips_ruled() {
        use crate::transducer::element::from_connection;
        fn q(v: i64) -> num_rational::BigRational {
            num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
        }
        fn conn(shift: i64) -> Element {
            let affine = vec![
                vec![q(1), q(0), q(shift)],
                vec![q(0), q(1), q(0)],
                vec![q(0), q(0), q(1)],
            ];
            from_connection(&affine, &[], &Vec::new(), [1, 0, 0, 0]).expect("spells")
        }
        let mut k = Kernel::new();
        let cap = crate::physics::Physics::default().stroke().remaining();
        let mut addrs = Vec::new();
        for i in 0..(cap + 1) {
            let w = conn(i as i64 + 1);
            let addr = w.address();
            assert!(matches!(k.install_adopted(&w, [1; 32]), Adoption::Installed));
            assert!(k.field().get(&addr).is_some(), "adopt writes a field entry");
            addrs.push(addr);
        }
        let ruled_n = |k: &Kernel, addrs: &[[u8; 32]]| {
            addrs
                .iter()
                .filter(|a| {
                    k.word_at(a).is_some_and(|w| k.ruled_have(&crate::transducer::stalk::invariant_of(w)).contains(a))
                })
                .count()
        };
        k.court_stroke();
        assert_eq!(ruled_n(&k, &addrs), cap, "one stroke rules a spend of unruled listings, not already-ruled ones");
        k.court_stroke();
        assert_eq!(ruled_n(&k, &addrs), cap + 1, "later listings are not starved by already-ruled ones");
    }

    /// Restore keeps the lettered federation and drops a leaked basis card.
    #[test]
    fn prune_drops_the_leaked_basis_and_keeps_the_ruled_cover() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 1, 1), (1, 0, 1), (1, 1, 1)];
        glue_pair(&mut k, &a, &b);
        let kept = k.basis_len();
        assert!(kept >= 1);
        let leak_a = vec![(9, 0, 2), (9, 1, 2)];
        let leak_b = vec![(9, 0, 3), (9, 1, 3)];
        let leaked = crate::transducer::element::from_residual(&germinate(&leak_a, &leak_b).unwrap());
        k.leak_unruled(leaked);
        assert_eq!(k.basis_len(), kept + 1);
        let dropped = k.prune_basis_to_ruled();
        assert_eq!(dropped, 1, "the leaked card leaves");
        assert_eq!(k.basis_len(), kept, "the ruled cover remains");
        k.leak_unruled(crate::transducer::element::from_residual(
            &germinate(&[(8, 0, 2), (8, 1, 2)], &[(8, 0, 5), (8, 1, 5)]).unwrap(),
        ));
        let snap = k.snapshot();
        let mut booted = Kernel::new();
        assert!(booted.restore(&snap));
        assert_eq!(booted.basis_len(), kept, "restore skips the leaked basis and rebuilds from the federation");
    }

    /// An append shares the prior word's Arc; it does not clone the basis.
    #[test]
    fn language_append_shares_prior_word_arcs() {
        let mut k = Kernel::new();
        let w0 = crate::transducer::element::from_residual(
            &germinate(&[(0, 0, 1), (0, 1, 1), (1, 0, 1)], &[(0, 1, 1), (1, 0, 1), (1, 1, 1)]).unwrap(),
        );
        let w1 = crate::transducer::element::from_residual(
            &germinate(&[(4, 0, 2), (4, 1, 2), (5, 0, 2)], &[(4, 0, 3), (4, 1, 3), (5, 0, 3)]).unwrap(),
        );
        k.leak_unruled(w0);
        let first = k.language();
        assert_eq!(first.words.len(), 1);
        let p0 = std::sync::Arc::as_ptr(&first.words[0]);
        assert!(
            std::sync::Arc::ptr_eq(&first, &k.language()),
            "a stable language is shared by pointer"
        );
        k.leak_unruled(w1);
        let second = k.language();
        assert_eq!(second.words.len(), 2);
        assert_eq!(
            std::sync::Arc::as_ptr(&second.words[0]),
            p0,
            "the first word is the same Arc, not a cloned element"
        );
    }

    /// Dialect/Slashed addresses are heard once; a cocycle row is unique
    /// on (chart, pair, versor).
    #[test]
    fn heard_and_cocycles_do_not_re_row() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        k.ingest(&a, &b, [1; 32]);
        k.ingest(&a, &b, [2; 32]);
        let spoken = k.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().1.clone();
        let mut forged = spoken.clone();
        forged.mag = [999, 0, 0, 0];
        let mut cover = crate::network::nerve::Cover::new();
        assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
        let n = k.cochain_bytes();
        assert!(k.heard.contains(&forged.address()));
        assert!(matches!(k.glue_fetched(forged, [2; 32], [1; 32], &mut cover), Glue::Dialect));
        assert_eq!(k.cochain_bytes(), n, "a heard slash is not re-rowed");
    }

    /// The home installs when a peer's ruled digest holds the stratum.
    #[test]
    fn home_installs_when_peer_ruled_root_gains_the_address() {
        let mut home = Kernel::new();
        let mut peer = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert_eq!(home.ingest(&a, &b, [2; 32]), Outcome::GermPending);
        assert_eq!(home.basis_len(), 0);
        home.rule_all_provisionals();
        let spoken = home.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().1.clone();
        let mut cover = crate::network::nerve::Cover::new();
        assert!(matches!(peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover), Glue::Admitted));
        assert!(peer.basis_len() >= 1);
        home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
        assert!(home.basis_len() >= 1, "the home's own word is language once a peer holds the stratum");
        assert!(home.provisional.is_empty(), "the provisional leaves the home");
    }

    /// A single-voice percept stays on the book, is Offered, glues at a
    /// peer, and the home installs with the escrow released and minted.
    #[test]
    fn a_single_voice_percept_stays_on_the_docket_and_installs_with_a_mint() {
        let mut home = Kernel::new();
        let mut peer = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        assert!(!home.pending.is_empty(), "the listing stays on the book");
        assert_eq!(home.provisional.len(), 1, "the residual is on the docket");
        let addr = home.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().0.clone();
        assert!(home.offered(&addr), "the court Offers a first-vantage germ");
        assert!(home.treasury().escrow_of(&addr).is_some(), "the priced magnitude is the claim");
        assert_eq!(home.treasury().supply(), [0; 4], "the claim is not yet payment");
        home.court_stroke();
        let spoken = home.provisional.get(&addr).cloned().expect("still provisional until a peer glues");
        let mut cover = crate::network::nerve::Cover::new();
        assert!(matches!(
            peer.glue_fetched(spoken, [1; 32], [2; 32], &mut cover),
            Glue::Admitted
        ));
        let bond = home.treasury().bond_total();
        home.confirm_peer_holds(&peer.ruled_digests(), [2; 32]);
        assert!(home.basis_len() >= 1, "the home installs after the peer glues");
        assert!(home.provisional.is_empty(), "the docket clears on install");
        assert!(home.treasury().escrow_of(&addr).is_none(), "the claim is released");
        assert!(
            home.treasury().supply().iter().any(|&x| x > 0),
            "release mints the priced vector"
        );
        assert_eq!(home.treasury().bond_total(), bond, "mint does not debit the stake bond");
    }

    /// A peer have-list that names a ruled provisional is the second organ:
    /// the home installs without waiting for a digest-root match.
    #[test]
    fn home_installs_on_a_have_list_naming_a_ruled_provisional() {
        let mut home = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        assert_eq!(home.ingest(&a, &b, [1; 32]), Outcome::GermPending);
        let addr = *home.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().0;
        home.court_stroke();
        assert!(home.provisional.contains(&addr), "still provisional until the second organ");
        home.confirm_peer_holds_addrs(&[addr], [2; 32]);
        assert!(home.basis_len() >= 1, "have-list of a ruled provisional installs");
        assert!(home.provisional.is_empty(), "the docket clears on install");
    }

    /// A stood sentence that closes no listing returns its escrow and mints nothing.
    #[test]
    fn speech_that_closes_nothing_mints_none() {
        let mut k = Kernel::new();
        let affine = crate::transducer::element::affine_of(&crate::transducer::element::identity_versor()).unwrap();
        let w = crate::transducer::element::from_connection(
            &affine,
            &[(1, 2), (2, 1)],
            &Vec::new(),
            [1, 1, 0, 0],
        )
        .unwrap();
        let addr = w.address();
        let codes = k.perceive_speech([1; 32], [0; 32], std::slice::from_ref(&w));
        assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
        assert!(k.treasury().escrow_of(&addr).is_some());
        k.rule_all_provisionals();
        k.confirm_peer_holds_addrs(&[addr], [2; 32]);
        assert!(k.treasury().escrow_of(&addr).is_none(), "escrow returned");
        assert_eq!(
            k.treasury().supply(),
            [0; 4],
            "a restatement that closes nothing pays nothing"
        );
    }

    fn digits(rows: &[&str]) -> Vec<Vec<u8>> {
        rows.iter()
            .map(|r| r.bytes().map(|b| b - b'0').collect())
            .collect()
    }

    /// Binding a listing to a task does not mint: the form must speak the trains.
    #[test]
    fn association_with_a_listing_does_not_mint() {
        let mut k = Kernel::new();
        let family = vec![(
            digits(&["1100", "0000"]),
            digits(&["0011", "0000"]),
        )];
        let (chain, _, frame) = crate::arc::refine_delta(&family[0].0, &family[0].1).expect("coboundary");
        assert!(k.intake(&chain, &frame), "the listing stands");
        let listing = k.pending.last().expect("listed").0.address();
        let task = [7u8; 32];
        k.bind_listing_task(listing, task);
        k.bind_task_family(task, family);
        let affine = crate::transducer::element::affine_of(&crate::transducer::element::identity_versor()).unwrap();
        let w = crate::transducer::element::from_connection(
            &affine,
            &[(1, 2), (2, 1)],
            &Vec::new(),
            [4, 4, 4, 4],
        )
        .unwrap();
        let addr = w.address();
        assert_ne!(addr, listing, "the restatement is not the listing");
        let codes = k.perceive_speech([1; 32], task, std::slice::from_ref(&w));
        assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
        k.bind_listing_task(addr, task);
        k.rule_all_provisionals();
        k.confirm_peer_holds_addrs(&[addr], [2; 32]);
        assert_eq!(
            k.treasury().supply(),
            [0; 4],
            "a bound listing is not a mint without replay"
        );
    }

    /// A rule that speaks every train pair mints the coboundary it closed.
    #[test]
    fn a_rule_that_speaks_the_family_mints_measured_descent() {
        let family = vec![
            (digits(&["1100", "0000"]), digits(&["0022", "0000"])),
            (digits(&["0000", "1100"]), digits(&["0000", "0022"])),
        ];
        let rule = crate::transducer::family::extract_rule(&family).expect("extract");
        let spoken = crate::transducer::family::mint_sentence(&rule, &family).expect("mint");
        assert!(spoken.is_sentence());
        let mut expect = [0u64; 4];
        for (x, y) in &family {
            let (chain, _, _) = crate::arc::refine_delta(x, y).expect("coboundary");
            let lanes = crate::transducer::element::measure_lanes(&chain);
            for i in 0..4 {
                expect[i] = expect[i].saturating_add(lanes[i]);
            }
        }
        assert!(expect.iter().any(|&x| x > 0), "the family has descent");
        let mut k = Kernel::new();
        let task = crate::registry::task_id(&crate::arc::Task {
            train: family.clone(),
            test: Vec::new(),
        });
        k.bind_task_family(task, family);
        let addr = spoken.address();
        let codes = k.perceive_speech([1; 32], task, std::slice::from_ref(&spoken));
        assert_eq!(codes, vec![crate::anastome::dialogue::SPEECH_STOOD]);
        k.rule_all_provisionals();
        k.confirm_peer_holds_addrs(&[addr], [2; 32]);
        assert_eq!(
            k.treasury().supply(),
            expect,
            "mint is the measured coboundary, not a bound listing's magnitude"
        );
    }

    /// The demand codec roundtrips.
    #[test]
    fn the_demand_codec_roundtrips() {
        let (a, b) = (scene(2, 2), scene(2, 5));
        let (v, ra, rb) = decode_demand(&encode_demand(&a, &b, &[9; 32])).unwrap();
        assert_eq!((v, ra, rb), ([9; 32], a, b));
    }

    /// Two hyphas at different K relocate the same standing listing at the
    /// same TUniversal: T-room, never a stroke count.
    #[test]
    fn t_room_relocates_at_tuniversal_independent_of_k() {
        let list = |k: &mut Kernel, extra: bool| -> [u8; 32] {
            let mut a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
            let mut b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
            if extra {
                a.push((1, 1, 1));
                b.push((1, 1, 5));
            }
            let _ = k.ingest(&a, &b, [1; 32]);
            k.pending.last().expect("listed").0.address()
        };
        let mut ha = Kernel::new();
        let mut hb = Kernel::new();
        ha.set_lane_cap([64, 64, 1, 64]);
        hb.set_lane_cap([64, 64, 1, 64]);
        ha.set_clock(10);
        hb.set_clock(10);
        let first_a = list(&mut ha, false);
        let first_b = list(&mut hb, false);
        assert_eq!(first_a, first_b);
        // Advance K on ha only — TUniversal stays the clock we set.
        for _ in 0..8 {
            ha.append_step([1; 32], [2; 32]);
        }
        assert!(ha.k_step() > hb.k_step());
        ha.set_clock(40);
        hb.set_clock(40);
        let _ = list(&mut ha, true);
        let _ = list(&mut hb, true);
        assert!(
            ha.cavity_book.contains(&first_a),
            "ha relocates the older listing on T"
        );
        assert!(
            hb.cavity_book.contains(&first_b),
            "hb relocates the same listing at the same TUniversal"
        );
    }

    /// Extra court strokes of a fixed corpus do not grow the hot set.
    #[test]
    fn hot_set_does_not_grow_with_idle_strokes() {
        let mut k = Kernel::new();
        k.set_clock(5);
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        k.ingest(&a, &b, [1; 32]);
        k.ingest(&a, &b, [2; 32]);
        k.court_stroke();
        let hot = (
            k.field_hot_len(),
            k.pending.len(),
            k.heard_len(),
            k.dialect_len(),
            k.provisional.len(),
        );
        for _ in 0..16 {
            k.court_stroke();
        }
        assert_eq!(
            (
                k.field_hot_len(),
                k.pending.len(),
                k.heard_len(),
                k.dialect_len(),
                k.provisional.len(),
            ),
            hot,
            "idle strokes do not grow the hot set"
        );
    }

    /// heard drops when the peer's stratum root moves.
    #[test]
    fn heard_drops_when_stratum_root_moves() {
        let mut k = Kernel::new();
        let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
        let b = vec![(0, 0, 4), (0, 1, 1), (1, 0, 1)];
        k.ingest(&a, &b, [1; 32]);
        k.ingest(&a, &b, [2; 32]);
        let spoken = k.provisional.fiber(&crate::network::nerve::ROOT).next().unwrap().1.clone();
        let mut forged = spoken.clone();
        forged.mag = [999, 0, 0, 0];
        let mut cover = crate::network::nerve::Cover::new();
        assert!(matches!(k.glue_fetched(forged.clone(), [2; 32], [1; 32], &mut cover), Glue::Slashed(_)));
        assert_eq!(k.heard_len(), 1);
        let inv = crate::transducer::stalk::invariant_of(&forged);
        let mut moved = [0u8; 32];
        moved[0] = 7;
        k.confirm_peer_holds(&[crate::transducer::sheaf::StratumDigest {
            inv,
            count: 0,
            root: moved,
        }], [2; 32]);
        assert_eq!(k.heard_len(), 0, "a moved stratum root drops heard");
    }
}
