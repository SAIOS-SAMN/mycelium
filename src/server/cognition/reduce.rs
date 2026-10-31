// 16.12, 23.8 — THE BOOK AS A MARKET READS IT, and the normal form the
// redundancy probe reads over the language.
//
//! Everything here runs over the `Language` SNAPSHOT with no lock held.
//! `rank_over` and `quote_incidence` read the book as a market view;
//! `normal_form` says whether the held words span a chain. Nothing here
//! builds an element from a residual: the four sites that did were the
//! court's F on a voice's painted sites (50.6, seam 3; law 8, ruled 2026-09-10),
//! deleted with the percept and demand acts.

use super::*;

/// The holonomy of a fibre of transitions: the geometric product around it.
/// `None` when the fibre is not an orbit — a singleton has nothing to close.
/// The orbit closes iff this is projectively the identity (10.3); the value
/// itself is the obstruction when it is not, and it is what a germ is bred
/// from. Reading a member of the fibre instead made the obstruction depend
/// on which transition the fibre happened to list last, and two nodes
/// holding one orbit in two insertion orders bred two different germs from
/// identical mathematics.
pub(crate) fn fibre_holonomy(vs: &[element::Versor]) -> Option<element::Versor> {
    // The fibre holds more than one transition; a singleton is not an orbit.
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

/// A listing's open charts, or its base chart where it is unprobed. The
/// base chart is the one the listing was ADMITTED under (`atlas`), read by
/// its address; it is lifted again only for a listing the atlas does not
/// chart. It was lifted on every read — body, section moduli, towers — for
/// every unprobed listing of every quote: on the development deployment (epoch 106, hypha b
/// at 194% of a core, its standing frozen) every stack sample of the quote
/// workers stood in `invariant_of` under this line.
pub(crate) fn open_charts_of(
    address: &[u8],
    listing: &Element,
    field: &crate::network::nerve::Keyed<ListingField>,
    atlas: &crate::transducer::stalk::Stalks,
) -> Vec<crate::transducer::stalk::Invariant> {
    match field.get(address) {
        Some(f) if f.nonflat > 0 => f
            .entries
            .iter()
            .filter(|e| !e.state.is_flat())
            .map(|e| e.chart.clone())
            .collect(),
        Some(f) if !f.entries.is_empty() => Vec::new(),
        _ => match atlas.chart_of(address) {
            Some(base) => vec![base.clone()],
            None => vec![crate::transducer::stalk::invariant_of(listing)],
        },
    }
}

/// Measure the rank of the book — pure.
pub fn rank_over(
    language: &Language,
    pending: &crate::network::nerve::Keyed<Element>,
    prior: &Rank,
    basis_gen: u64,
    chart_listings: &crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    budget: &mut crate::physics::Budget,
) -> Rank {
    let stalks = &language.stalks;
    // A listing's base point moves only when a word lands in its own
    // stratum. At the same basis generation every prior closure is reused
    // by the listing's address. When the basis grew with no retirement,
    // the words added since name the strata that moved, and only listings
    // in those strata are located again — a coordinate read, never the
    // whole book.
    let reuse = prior.retire_gen == language.retire_gen && !prior.closure_is_empty();
    let mut moved: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
        crate::network::nerve::Keyed::new();
    // THE WORDS ADDED SINCE, read off each word's own admission ordinal.
    // It was `language.words[prior.basis_len..]` — a tail of a vector,
    // which named different words the moment a retirement shifted it.
    if reuse && prior.basis_gen != basis_gen {
        for w in language.words_since(prior.basis_seq) {
            let inv = crate::transducer::stalk::invariant_of(w);
            moved.insert(inv.key(), inv);
        }
    }
    let locate = |p: &Element, budget: &mut crate::physics::Budget| {
        crate::transducer::stalk::congruent_shaped(
            stalks,
            |a| language.shape_at(a),
            p.sub(),
            budget,
        )
        // THE BASE POINT IS THE COORDINATE the congruence landed on, which
        // is what `congruent_shaped` already returns; it was converted to
        // a slot here and the slot was what the rank stored.
        .and_then(|(a, _)| language.holds(a.as_bytes()).then_some(a))
    };
    // THE MEASUREMENT IS OVER THE OPEN BOOK, which is what a rank is; the
    // carrier is `pending` and its whole content is the object. What keeps
    // it from being a whole-carrier read per stroke is the generation gate above and the
    // `reuse` path, which locates again only the listings whose stratum a
    // new word moved -- a coordinate read, never the whole book.
    let mut out = Rank::default();
    let mut invariants = crate::network::nerve::Keyed::new();
    match reuse {
        false => {
            for (address, p) in pending.scoped() {
                invariants.insert(&address, crate::transducer::stalk::invariant_of(p));
                out.put_closure(&address, locate(p, budget));
            }
        }
        true => {
            for (address, p) in pending.scoped() {
                // The prior reading where it holds this listing; the
                // listing's own invariant where it does not. Both are
                // measured on the word.
                let inv = match prior.invariants.get(&address) {
                    Some(i) => i.clone(),
                    None => crate::transducer::stalk::invariant_of(p),
                };
                invariants.insert(&address, inv);
                match prior.closure_of(&address) {
                    Some(c) => out.put_closure(&address, c.clone()),
                    None => out.put_closure(&address, locate(p, budget)),
                }
            }
            if !moved.is_empty() {
                // The strata a new word moved, and under each the listings
                // filed on it: `link` follows the keyed prefix, so this is a
                // coordinate read and not a second pass over the book.
                for (ck, _) in moved.scoped() {
                    for (address, _) in chart_listings.link(&ck) {
                        if let Some(p) = pending.get(&address) {
                            out.put_closure(&address, locate(p, budget));
                        }
                    }
                }
            }
        }
    }
    out.invariants = invariants;
    out.basis_gen = basis_gen;
    out.basis_seq = language.next_seq;
    out.retire_gen = language.retire_gen;
    out
}

/// THE OPEN BOOK AS A QUOTE READS IT. Five keyed reads that all come off the
/// same lock, so they are one argument and not six.
pub struct BookView<'a> {
    /// The listings themselves, with the k-step each was listed at.
    pub pending: &'a crate::network::nerve::Keyed<Element>,
    /// The atlas the listings are filed under: incidence reads this.
    pub listing_atlas: &'a crate::transducer::stalk::Stalks,
    /// The field each listing stands in — what its open charts are read from.
    pub field: &'a crate::network::nerve::Keyed<ListingField>,
    /// What is on the page at all.
    pub on_page: &'a crate::network::nerve::Keyed<()>,
    /// Which voices each listing has already been spoken to.
    pub spoken: &'a crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    // The standing T of each listing rode here and replaced the 𝒯 lane of its
    // magnitude in the quote. Retired by 78.149 (a): the span a listing stood
    // is not μ, and no price carries it.
}

/// WHO IS ASKING: the charts this voice holds open, the surface it is
/// quoted against, the lane pressure the price is read at, the body budget
/// of one page, and the voice itself.
pub struct Ask<'a> {
    pub charts: &'a [crate::transducer::stalk::Invariant],
    pub surface: &'a [u64; 4],
    pub pressure: &'a [u64; 4],
    pub budget: usize,
    pub voice: &'a [u8; 32],
}

/// QUOTE — pure, over a language snapshot and the open book. The yield
/// surface is read from genuine metabolic tension; each un-factored residual
/// is priced against it, μ⃗[l] = magnitude[l] · (1 + surface[l]). The (C, I)
/// pair differentiates the market (never summed): closure work (open demand
/// whose geometry a held word speaks, up to isometry, through the atlas)
/// carries the C premium; coverage work (unknown geometry) the I premium.
///
/// QUOTE as incidence: listings whose open charts meet the ask's charts,
/// served in atlas order of those charts, within the body budget. No
/// total order over the book. Returns (surface, priced items with open charts).
pub fn quote_incidence(book: &BookView<'_>, ask: &Ask<'_>) -> ([u64; 4], Vec<Priced>) {
    let BookView { pending, listing_atlas, field, on_page, spoken } = book;
    let &Ask { charts: voice_charts, surface, pressure, budget, voice } = ask;
    let link = crate::anastome::dialogue::link_wire_width();
    let mut priced: Vec<Priced> = Vec::new();
    let mut used = 0usize;
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let root_chart = crate::transducer::stalk::Invariant(Vec::new());
    let mut run = |charts: &[crate::transducer::stalk::Invariant]| {
        for chart in charts {
            let chart_key = chart.key();
            let addrs: Vec<Vec<u8>> = if chart.0.is_empty() {
                listing_atlas.fibre_under(chart)
            } else {
                listing_atlas.fibre(chart)
            };
            for address in addrs {
                if !on_page.contains(&address) || seen.contains(&address) {
                    continue;
                }
                if spoken.get(&address).is_some_and(|voices| voices.contains(voice)) {
                    continue;
                }
                let Some(p) = pending.get(&address) else { continue };
                let open = open_charts_of(&address, p, field, listing_atlas);
                let mut open_set: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                for c in &open {
                    open_set.insert(c.key(), ());
                }
                if !chart.0.is_empty() && !open_set.contains(&chart_key) {
                    continue;
                }
                // The width is the width the encoder writes. It mirrored
                // the encoder's truncation at 255 axes, so a chart wider
                // than that was measured short here and then refused on
                // the wire; the encoder refuses it now and this measures
                // what it would write.
                let open_len: usize = 1 + open.iter().map(|c| 1 + 8 * c.0.len()).sum::<usize>();
                let wire = 68 + p.sub().len() * link + open_len;
                // THE PAGE IS FULL, SO THE READ IS OVER. This was
                // `continue`: a listing that did not fit was skipped and
                // the read carried on through the WHOLE BOOK looking for a
                // smaller one, on every quote. The book grows with demand
                // arriving from every peer, and the root run below is
                // unconditional, so this was a per-request whole read of an
                // append-only carrier (law 3) that the frame budget was
                // supposed to bound.
                //
                // A page serves what fits in rank order and stops. What
                // did not fit is not lost: it is on the book, and the next
                // quote resumes from the cursor that paces the page.
                if used + wire > budget {
                    return;
                }
                used += wire;
                seen.insert(&address, ());
                // The listing's own measure, its ask (78.149 (b)): a cap on
                // what a closer is paid, never the span it stood (a).
                let mu = crate::economy::price(&p.magnitude(), pressure);
                priced.push(Priced { word: p.clone(), mu, open });
            }
        }
    };
    // An empty voice atlas is incident to the root prefix and receives
    // the book's shallowest strata. A miss on the asked charts falls
    // through to the same root run (seen skips duplicates).
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
///
/// BOTH ARE CARRIED WHERE THE CLOSURE IS BOUND (`Rank::put_closure`), so
/// this is a read of two integers. It read the whole closure map, once
/// per call, for two numbers the write already knew -- the same shape as
/// `ListingField::nonflat`, which is kept by `write_field` for exactly
/// this reason.
pub fn cohesion_over(rank: &Rank) -> (u64, u64) {
    let (c, i) = rank.counts();
    (c as u64, i as u64)
}

/// THE NORMAL FORM of a chain over a language: whether the held words
/// compose it (the citation set, a sentence when closed), or leave a
/// remainder. One closed-form pass; no search. The remainder is not
/// composed into an element here: what the language does not span is the
/// speaker's to say (law 8), and the redundancy probe asks only whether
/// the rest of the basis derives a held word.
pub fn normal_form(
    language: &Language,
    chain: &[(i64, i64, u8, u8)],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> NormalForm {
    let sentence = crate::transducer::sentence::deconstruct_at(
        &|a| language.word_at(a),
        &language.stalks,
        &|a| language.shape_at(a),
        chain,
        exclude,
        budget,
    );
    match (sentence.residual.as_slice(), sentence.punctures.as_slice()) {
        ([], []) => NormalForm::Decomposed(sentence.cites.iter().map(|c| c.word.clone()).collect()),
        _ => NormalForm::Irreducible,
    }
}
