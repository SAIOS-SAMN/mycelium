// 16.12, 23.8 — REDUCTION: what a spoken chain becomes against the
// language, and what the book is worth to a voice that asks.
//
//! A reduction runs over the `Language` SNAPSHOT with no lock held. `rank_over`
//! and `quote_incidence` read the book as a market view; `normal_form`,
//! `reduce_germ` and `reduce_all` carry a chain to what the language already
//! says; `reduce_proven` replays a proof a client already reduced.

use super::*;

/// The holonomy of a fiber of transitions: the geometric product around it.
/// `None` when the fiber is not a loop — a singleton has nothing to close.
/// The loop closes iff this is projectively the identity (10.3); the value
/// itself is the obstruction when it is not, and it is what a germ is bred
/// from. Reading a member of the fiber instead made the obstruction depend
/// on which transition the fiber happened to list last, and two nodes
/// holding one loop in two insertion orders bred two different germs from
/// identical mathematics.
pub(crate) fn fiber_holonomy(vs: &[element::Versor]) -> Option<element::Versor> {
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

pub(crate) fn rotor_versor(g: &crate::transducer::alphabet::Rotor) -> Option<element::Versor> {
    let word = g.spelled_word()?;
    crate::transducer::element::conformal_of(&word, &crate::transducer::element::identity_versor())
}

pub(crate) fn union_remainders(entries: &[FieldEntry]) -> Vec<(i64, i64, u8, u8)> {
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

pub(crate) fn open_charts_of(
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

/// THE COHESION READING — the joint (C, I) state over the open book, both
/// axes exact and never traded. C counts open demands sitting on held
/// geometry (closure failing — collapse-side work); I counts open demands on
/// geometry no thought speaks (coverage failing — expansion-side work).
///
/// The two counts ARE the reading, returned exactly, and whoever reads them
/// decides. Nothing compares them against a tolerance and nothing collapses
/// them to one state: C and I are two lanes of the same object, and a scalar
/// verdict over two lanes is the flattening law 5 refuses.
pub fn cohesion_over(rank: &Rank) -> (u64, u64) {
    let mut c = 0u64;
    let mut i = 0u64;
    for (_, closed) in rank.closure.fiber(crate::network::nerve::ROOT) {
        if closed.is_some() {
            c = c.saturating_add(1);
        } else {
            i = i.saturating_add(1);
        }
    }
    (c, i)
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
