// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 6, 24.1 — A READING of a chain against a language: locating one body by
//! its moduli, puncturing a held word the body deepens, and deconstructing
//! the whole chain into the words that speak it and the residual that stays.

use crate::network::nerve::Keyed;
use crate::transducer::blade::{self, Shape};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::chain::{
    gauge_of, pieces, section_site_key, replay_at, replay_operators, Cite, HeldCite, Link, Located, Puncture, Sentence,
};
use crate::transducer::sentence::form::Vertex;
use crate::transducer::stalk::{self, Stalks};

/// THE FOUR LOOKUPS A READING NEEDS, which always travel together: the word
/// AT AN ADDRESS, the fibre of a chart, the shape at an address, and the
/// held words read at a chart for a body (24.1 as ruled 2026-09-09: the
/// Held stratum is the contemplation's forms, read at the coordinate the
/// listing supplies, never a list of elements). This file sits BELOW the
/// language — the cognition book reads 18.x, never the other way — so a
/// reading takes the lookups rather than the language itself, and a caller
/// can point them at the genome, the dialect, a working atlas, or all three.
///
/// IT WAS FIVE, AND THE FIFTH WAS A CARTESIAN ROUND TRIP. `fiber_of`
/// answers a chart with ADDRESSES; `index_of` turned each address into an
/// offset into whatever vector the caller happened to hold, and `word_at`
/// turned the offset back into a word. The offsets existed only to be
/// undone, and the carriers that stored them — `Epigenome::index`,
/// `Epigenome::expressed_index`, `Language::index` — were pure functions of
/// the words they indexed, rebuilt from nothing by `reindex()` and costing
/// 11.5 MB on a voice eleven minutes into a soak. The address IS the
/// coordinate (law 2) and the trie reaches the word by its prefix directly, so the
/// round trip and all three carriers are gone.
pub struct Reading<'r, W, F, S, H> {
    pub word_at: &'r W,
    pub fiber_of: &'r F,
    pub shape_at: &'r S,
    pub held_at: &'r H,
}

/// A reading with no Held stratum: the genome, or the genome and the
/// dialect, alone.
pub fn no_held(_inv: &stalk::Invariant, _piece: &[Link], _budget: &mut crate::physics::Budget) -> Vec<HeldCite> {
    Vec::new()
}

/// THE WORD AT A VERTEX, EVALUATED ON A BODY (24.1 as ruled 2026-09-09;
/// 50.1 step 1). The vertex holds a rotor, a carriage and a gauge; the
/// caller holds the body. The rotor under the carriage's dilator is
/// carried onto the body's own frame by the translator its centroids
/// solve (`versor_carrying`), and the action is verified compartment by compartment:
/// every compartment of the body's before-support lands on a compartment of its
/// after-support, every after-compartment is reached, and each landed hue is
/// the gauge's image of the hue it left. Nothing is decoded from the address,
/// no element is minted to ask, and a vertex that reconstructs nothing
/// here is not a word for this body. The cite carries the body in its own
/// frame — the frame the action was evaluated in — and the vertex's
/// address, which names the word and never the chain.
pub fn vertex_cites(vertex: &Vertex, piece: &[Link], budget: &mut crate::physics::Budget) -> Option<HeldCite> {
    let Vertex::Word { acts, .. } = vertex else {
        return None;
    };
    // THE MEMBER THAT CARRIES THIS BODY, found by carrying it. The caller holds
    // a body and asks whether this vertex is a word for it; a vertex acting on
    // several charts answers with the member whose action RECONSTRUCTS the
    // body, and that is a verified match rather than a member taken by
    // position. Reading one member asked the question of one chart and
    // answered `None` for every body on the others.
    for (_, a) in acts.scoped() {
        if let Some(cite) = vertex_cites_by(vertex, &a.carriage, &a.gauge, piece, budget) {
            return Some(cite);
        }
    }
    None
}

/// The reconstruction test at ONE member: the rotor under that member's
/// dilator carried onto the body's frame, verified compartment by compartment.
fn vertex_cites_by(
    vertex: &Vertex,
    carriage: &crate::transducer::element::Versor,
    gauge: &[(u8, u8)],
    piece: &[Link],
    budget: &mut crate::physics::Budget,
) -> Option<HeldCite> {
    let Vertex::Word { address, derivation, .. } = vertex else {
        return None;
    };
    let rotor = derivation.rotor();
    let mut before: Vec<(i64, i64, u8)> = Vec::new();
    let mut after: Keyed<u8> = Keyed::new();
    let mut out_support: Vec<(i64, i64)> = Vec::new();
    let void = crate::arc::ground();
    for &(r, c, b, a) in piece {
        if b != void {
            before.push((r, c, b));
        }
        if a != void {
            after.insert(section_site_key(r, c), a);
            out_support.push((r, c));
        }
    }
    if before.is_empty() || out_support.is_empty() {
        return None;
    }
    before.sort_unstable();
    before.dedup();
    out_support.sort_unstable();
    out_support.dedup();
    let in_support: Vec<(i64, i64)> = before.iter().map(|&(r, c, _)| (r, c)).collect();
    let shape_in = blade::shape_of_section(&in_support);
    let shape_out = blade::shape_of_section(&out_support);
    let linear = element::conformal_of_rotor(&rotor, &element::dilator_of(carriage)?)?;
    let v = blade::versor_carrying(&shape_in, &shape_out, &linear, budget).ok()?.versor;
    for &(r, c, h) in &before {
        let p = element::carry_center(&v, r, c, 0)?;
        let (tr, tc, tk) = element::site_of(&p)?;
        if tk != 0 {
            return None;
        }
        let landed = *after.get(&section_site_key(tr, tc))?;
        if Some(landed) != super::hue_of(gauge, h) {
            return None;
        }
    }
    let mut body = piece.to_vec();
    body.sort_unstable();
    body.dedup();
    Some(HeldCite { address: address.clone(), body })
}

/// LOCATE one body: read its moduli, take the stratum, and for the word
/// there whose gauge charge is the body's, solve the transport from the
/// moments and verify it by replay. Returns the citation and the stratum's
/// multiplicity (the measurement, not a search width).
pub fn locate<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    atlas: &Stalks,
    shape_at: &impl Fn(&[u8]) -> Option<&'a Shape>,
    piece: &[Link],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> (Option<Cite>, usize) {
    let found = locate_or_puncture_at(word_at, atlas, shape_at, piece, exclude, budget);
    (found.cite, found.multiplicity)
}

/// LOCATE, and when a word carries the body by shape under a gauge it does
/// not yet hold, the PUNCTURE: the word's links under the body's own
/// transitions pulled back through the transport, verified by replay. A
/// citation is preferred; a puncture is returned only when no word speaks
/// the body as held.
pub fn locate_or_puncture_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    atlas: &Stalks,
    shape_at: &impl Fn(&[u8]) -> Option<&'a Shape>,
    piece: &[Link],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> Located {
    let fiber_of = |inv: &stalk::Invariant| atlas.fiber(inv);
    locate_or_puncture_on(
        &Reading { word_at, fiber_of: &fiber_of, shape_at, held_at: &no_held },
        piece,
        exclude,
        budget,
    )
}

/// Nothing located: no cite, no puncture, no held word, and the stratum's
/// width as measured.
fn unlocated(multiplicity: usize) -> Located {
    Located { cite: None, puncture: None, held: None, multiplicity }
}

/// Locate against an arbitrary fiber — the working atlas, not only the
/// genome. Ruled first, then Stood, then Held (24.1): the Held stratum is
/// read only where no ruled or stood word is on the body's chart, and
/// there exactly one held form's word must reconstruct the body; several
/// are sections and stand as multiplicity, never a choice.
pub fn locate_or_puncture_on<'a, W, F, S, H>(
    read: &Reading<'_, W, F, S, H>,
    piece: &[Link],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> Located
where
    W: Fn(&[u8]) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    S: Fn(&[u8]) -> Option<&'a Shape>,
    H: Fn(&stalk::Invariant, &[Link], &mut crate::physics::Budget) -> Vec<HeldCite>,
{
    let &Reading { word_at, fiber_of, shape_at, held_at } = read;
    let Some(shape) = element::shape_of_chain(piece) else {
        return unlocated(0);
    };
    let chart = stalk::invariant(piece);
    let key = element::gauge_key(&element::assignment(piece));
    // THE STRATUM IS ADDRESSES, WHICH IS WHAT THE ATLAS ANSWERS WITH. It
    // was converted to offsets here and converted back one line later.
    let stratum: Vec<Vec<u8>> = fiber_of(&chart)
        .into_iter()
        .filter(|a| Some(a.as_slice()) != exclude)
        .filter(|a| word_at(a).is_some_and(|w| !w.assignments_by_profile(&key).is_empty()))
        .collect();
    let m = stratum.len();
    if stratum.is_empty() {
        let held = held_at(&chart, piece, budget);
        let m = held.len();
        let [h] = held.as_slice() else { return unlocated(m) };
        return Located { cite: None, puncture: None, held: Some(h.clone()), multiplicity: m };
    }
    let [addr] = stratum.as_slice() else { return unlocated(m) };
    let Some(word) = word_at(addr) else { return unlocated(m) };
    let Some(held) = shape_at(addr) else { return unlocated(m) };
    let Ok(to_word) = blade::versor_between(&shape, held, budget) else { return unlocated(m) };
    // Back to the word by the reverse; a site is read by proportion (78.32(1)).
    if !crate::transducer::cga::is_versor(&to_word) {
        return unlocated(m);
    }
    let v = to_word.reverse();
    let agreeing = gauge_of(word, &v, piece);
    // The gauges agreeing on THIS piece under this transport: the reading
    // is whether there is exactly one, so the read takes two and stops.
    let mut asgs = agreeing.scoped();
    if let Some((_, asg)) = asgs.next() {
        if asgs.next().is_none() {
            // The cite carries the ASSIGNMENT the agreeing gauge IS — the
            // reading answers it, and nothing looks a position up (78.43(1)).
            let assignment = asg.clone();
            let cite = Cite {
                word: crate::transducer::address::Address::new(addr.clone()),
                transport: v.clone(),
                assignment,
            };
            if replay_at(word_at, std::slice::from_ref(&cite), piece).is_empty() {
                return Located { cite: Some(cite), puncture: None, held: None, multiplicity: m };
            }
        } else {
            return unlocated(agreeing.len());
        }
    }
    let pos = element::positions(word.sub());
    if let Some(asg) = element::pullback(&pos, piece, &v) {
        let mut body: Vec<Link> = pos.iter().zip(&asg).map(|(&(r, c), &(b, a))| (r, c, b, a)).collect();
        body.sort_unstable();
        if replay_operators(&[(v.clone(), body.clone())], piece).is_empty() {
            let puncture = Puncture {
                word: crate::transducer::address::Address::new(addr.clone()),
                transport: v,
                links: body,
            };
            return Located { cite: None, puncture: Some(puncture), held: None, multiplicity: m };
        }
    }
    unlocated(m)
}

fn body_key(body: &[Link]) -> Option<crate::transducer::address::Address> {
    if body.is_empty() {
        return None;
    }
    // A body with no canonical chain has no key, and a reading keyed by
    // nothing is not taken (U14).
    Some(element::from_residual(&body.to_vec())?.address())
}

fn place_located<'a, W, F, S, H>(
    out: &mut Sentence,
    located: Located,
    body: &[Link],
    read: &Reading<'_, W, F, S, H>,
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) where
    W: Fn(&[u8]) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    S: Fn(&[u8]) -> Option<&'a Shape>,
    H: Fn(&stalk::Invariant, &[Link], &mut crate::physics::Budget) -> Vec<HeldCite>,
{
    if let Some(k) = body_key(body) {
        out.multiplicity.insert(k, located.multiplicity);
    }
    match (located.cite, located.puncture, located.held) {
        (Some(c), _, _) => out.cites.push(c),
        (None, Some(p), _) => out.punctures.push(p),
        (None, None, Some(h)) => out.held.push(h),
        (None, None, None) => match element::gauge_fibers(body) {
            None => {}
            Some(element::GaugeFibers::One(_, links)) => {
                out.residual.extend(links);
            }
            Some(element::GaugeFibers::Many(idx)) => {
                // One body's gauge fibers, each read in turn: a piece left
                // out is a piece nothing places.
                for (_, fiber) in idx.scoped() {
                    for sub in pieces(fiber) {
                        let loc = locate_or_puncture_on(read, &sub, exclude, budget);
                        place_located(out, loc, &sub, read, exclude, budget);
                    }
                }
            }
        },
    }
}

/// DECONSTRUCT a chain over a language: the whole chain located first (a
/// word of several bodies), then each body; the connection between
/// consecutive cited bodies is the bodies' relative transport; the residual is
/// what no word speaks. `exclude` keeps a word from citing itself.
/// Deconstruct over a getter, so a language snapshot of `Arc<Element>`
/// never flattens into a cloned slice.
pub fn deconstruct_at<'a>(
    word_at: &impl Fn(&[u8]) -> Option<&'a Element>,
    atlas: &Stalks,
    shape_at: &impl Fn(&[u8]) -> Option<&'a Shape>,
    chain: &[Link],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> Sentence {
    let fiber_of = |inv: &stalk::Invariant| atlas.fiber(inv);
    deconstruct_on(
        &Reading { word_at, fiber_of: &fiber_of, shape_at, held_at: &no_held },
        chain,
        exclude,
        budget,
    )
}

/// Deconstruct over a fiber the caller names — genome, dialect, or both —
/// and over the held words the caller reads at each body's chart.
pub fn deconstruct_on<'a, W, F, S, H>(
    read: &Reading<'_, W, F, S, H>,
    chain: &[Link],
    exclude: Option<&[u8]>,
    budget: &mut crate::physics::Budget,
) -> Sentence
where
    W: Fn(&[u8]) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    S: Fn(&[u8]) -> Option<&'a Shape>,
    H: Fn(&stalk::Invariant, &[Link], &mut crate::physics::Budget) -> Vec<HeldCite>,
{
    let mut out = Sentence::default();
    if chain.is_empty() {
        return out;
    }
    let whole = locate_or_puncture_on(read, chain, exclude, budget);
    if let Some(k) = body_key(chain) {
        out.multiplicity.insert(k, whole.multiplicity);
    }
    match (whole.cite.is_some(), whole.held.is_some()) {
        (true, _) | (_, true) => {
            place_located(&mut out, whole, chain, read, exclude, budget);
        }
        (false, false) => {
            for body in pieces(chain) {
                let loc = locate_or_puncture_on(read, &body, exclude, budget);
                place_located(&mut out, loc, &body, read, exclude, budget);
            }
        }
    }
    out.residual.sort_unstable();
    out.connection = out
        .cites
        .windows(2)
        // `w̃₀ w₁`, the connection as a ray (78.32(1)).
        .filter(|w| crate::transducer::cga::is_versor(&w[0].transport))
        .map(|w| w[0].transport.reverse().geo(&w[1].transport))
        .collect();
    out
}
