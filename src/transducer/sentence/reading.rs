// AI-GENERATED CONTENT — see DISCLAIMER.md.

//! Locating a body against a language, and deconstructing a chain into the
//! words that speak it. What `chain.rs` holds is the chain itself — its
//! cells, its pieces, its replay; what a READING of that chain is, against
//! some set of held words, is here.

use crate::network::nerve::ROOT;
use crate::transducer::blade::{self, Shape};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::chain::{
    gauge_of, pieces, replay_at, replay_operators, Cell, Cite, Located, Sentence,
};
use crate::transducer::stalk::{self, Stalks};

/// THE FOUR LOOKUPS A READING NEEDS, which always travel together: the word
/// at an index, the fibre of a chart, the index of an address, and the shape
/// at an index. This file sits BELOW the language — the cognition book reads
/// 18.x, never the other way — so a reading takes the lookups rather than the
/// language itself, and a caller can point them at the genome, the dialect, a
/// working atlas, or all three.
pub struct Reading<'r, W, F, I, S> {
    pub word_at: &'r W,
    pub fiber_of: &'r F,
    pub index_of: &'r I,
    pub shape_at: &'r S,
}

/// LOCATE one body: read its moduli, take the stratum, and for the word
/// there whose gauge charge is the body's, solve the transport from the
/// moments and verify it by replay. Returns the citation and the stratum's
/// multiplicity (the measurement, not a search width).
pub fn locate<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> (Option<Cite>, usize) {
    let (cite, _, m) = locate_or_puncture_at(&|i| words.get(i), atlas, index_of, shape_at, piece, exclude);
    (cite, m)
}

/// LOCATE, and when a word carries the body by shape under a gauge it does
/// not yet hold, the PUNCTURE: the word's cells under the body's own
/// transitions pulled back through the transport, verified by replay. A
/// citation is preferred; a puncture is returned only when no word speaks
/// the body as held.
pub fn locate_or_puncture<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> Located {
    locate_or_puncture_at(&|i| words.get(i), atlas, index_of, shape_at, piece, exclude)
}

pub fn locate_or_puncture_at<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    atlas: &Stalks,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> Located {
    let fiber_of = |inv: &stalk::Invariant| atlas.fiber(inv);
    locate_or_puncture_on(
        &Reading { word_at, fiber_of: &fiber_of, index_of, shape_at },
        piece,
        exclude,
    )
}

/// Locate against an arbitrary fiber — the working atlas, not only the genome.
pub fn locate_or_puncture_on<'a, W, F, I, S>(
    read: &Reading<'_, W, F, I, S>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> Located
where
    W: Fn(usize) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    I: Fn(&[u8]) -> Option<usize>,
    S: Fn(usize) -> Option<&'a Shape>,
{
    let &Reading { word_at, fiber_of, index_of, shape_at } = read;
    let Some(shape) = element::shape_of_chain(piece) else { return (None, None, 0) };
    let stratum: Vec<usize> = fiber_of(&stalk::invariant(piece)).iter().filter_map(|a| index_of(a)).collect();
    let key = element::gauge_key(&element::assignment(piece));
    let stratum: Vec<usize> = stratum
        .into_iter()
        .filter(|&i| Some(i) != exclude)
        .filter(|&i| word_at(i).is_some_and(|w| !w.assignments_by_profile(&key).is_empty()))
        .collect();
    let m = stratum.len();
    let [i] = stratum.as_slice() else { return (None, None, m) };
    let i = *i;
    let Some(word) = word_at(i) else { return (None, None, m) };
    let Some(held) = shape_at(i) else { return (None, None, m) };
    let Some(to_word) = blade::versor_between(&shape, held) else { return (None, None, m) };
    let Some(v) = to_word.versor_inverse() else { return (None, None, m) };
    let agreeing = gauge_of(word, &v, piece);
    let mut asgs = agreeing.fiber(ROOT);
    if let Some((_, &asg)) = asgs.next() {
        if asgs.next().is_none() {
            let cite: Cite = (i, v.clone(), asg);
            if replay_at(word_at, std::slice::from_ref(&cite), piece).is_empty() {
                return (Some(cite), None, m);
            }
        } else {
            return (None, None, agreeing.len());
        }
    }
    let pos = element::positions(word.sub());
    if let Some(asg) = element::pullback(&pos, piece, &v) {
        let mut body: Vec<Cell> = pos.iter().zip(&asg).map(|(&(r, c), &(b, a))| (r, c, b, a)).collect();
        body.sort_unstable();
        if replay_operators(&[(v.clone(), body.clone())], piece).is_empty() {
            return (None, Some((i, v, body)), m);
        }
    }
    (None, None, m)
}

fn body_key(body: &[Cell]) -> Option<crate::transducer::a21::Address> {
    if body.is_empty() {
        return None;
    }
    Some(element::from_residual(&body.to_vec()).address())
}

fn place_located<'a, W, F, I, S>(
    out: &mut Sentence,
    located: Located,
    body: &[Cell],
    read: &Reading<'_, W, F, I, S>,
    exclude: Option<usize>,
) where
    W: Fn(usize) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    I: Fn(&[u8]) -> Option<usize>,
    S: Fn(usize) -> Option<&'a Shape>,
{
    if let Some(k) = body_key(body) {
        out.multiplicity.insert(k, located.2);
    }
    match located {
        (Some(c), _, _) => out.cites.push(c),
        (None, Some(p), _) => out.punctures.push(p),
        (None, None, _) => match element::gauge_fibers(body) {
            None => {}
            Some(element::GaugeFibers::One(_, cells)) => {
                out.residual.extend(cells);
            }
            Some(element::GaugeFibers::Many(idx)) => {
                for (_, fiber) in idx.fiber(ROOT) {
                    for sub in pieces(fiber) {
                        let loc = locate_or_puncture_on(read, &sub, exclude);
                        place_located(out, loc, &sub, read, exclude);
                    }
                }
            }
        },
    }
}

/// DECONSTRUCT a chain over a language: the whole chain located first (a
/// word of several bodies), then each body; the connection between
/// consecutive cited bodies is their relative transport; the residual is
/// what no word speaks. `exclude` keeps a word from citing itself.
pub fn deconstruct<'a>(
    words: &'a [Element],
    atlas: &Stalks,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence {
    let word_at = |i: usize| words.get(i);
    let fiber_of = |inv: &stalk::Invariant| atlas.fiber(inv);
    deconstruct_on(
        &Reading { word_at: &word_at, fiber_of: &fiber_of, index_of, shape_at },
        chain,
        exclude,
    )
}

/// Deconstruct over a getter, so a language snapshot of `Arc<Element>`
/// never flattens into a cloned slice.
pub fn deconstruct_at<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    atlas: &Stalks,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence {
    let fiber_of = |inv: &stalk::Invariant| atlas.fiber(inv);
    deconstruct_on(
        &Reading { word_at, fiber_of: &fiber_of, index_of, shape_at },
        chain,
        exclude,
    )
}

/// Deconstruct over a fiber the caller names — genome, dialect, or both.
pub fn deconstruct_on<'a, W, F, I, S>(
    read: &Reading<'_, W, F, I, S>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence
where
    W: Fn(usize) -> Option<&'a Element>,
    F: Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    I: Fn(&[u8]) -> Option<usize>,
    S: Fn(usize) -> Option<&'a Shape>,
{
    let mut out = Sentence::default();
    if chain.is_empty() {
        return out;
    }
    let whole = locate_or_puncture_on(read, chain, exclude);
    if let Some(k) = body_key(chain) {
        out.multiplicity.insert(k, whole.2);
    }
    match whole.0 {
        Some(_) => {
            place_located(&mut out, whole, chain, read, exclude);
        }
        None => {
            for body in pieces(chain) {
                let loc = locate_or_puncture_on(read, &body, exclude);
                place_located(&mut out, loc, &body, read, exclude);
            }
        }
    }
    out.residual.sort_unstable();
    out.connection = out
        .cites
        .windows(2)
        .filter_map(|w| Some(w[0].1.versor_inverse()?.geo(&w[1].1)))
        .collect();
    out
}
