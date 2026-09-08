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
    locate_or_puncture_on(word_at, &|inv| atlas.fiber(inv), index_of, shape_at, piece, exclude)
}

/// Locate against an arbitrary fiber — the working atlas, not only the genome.
pub fn locate_or_puncture_on<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> Located {
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

fn place_located<'a>(
    out: &mut Sentence,
    located: Located,
    body: &[Cell],
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    exclude: Option<usize>,
) {
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
                        let loc = locate_or_puncture_on(
                            word_at, fiber_of, index_of, shape_at, &sub, exclude,
                        );
                        place_located(out, loc, &sub, word_at, fiber_of, index_of, shape_at, exclude);
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
    deconstruct_on(&|i| words.get(i), &|inv| atlas.fiber(inv), index_of, shape_at, chain, exclude)
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
    deconstruct_on(word_at, &|inv| atlas.fiber(inv), index_of, shape_at, chain, exclude)
}

/// Deconstruct over a fiber the caller names — genome, dialect, or both.
pub fn deconstruct_on<'a>(
    word_at: &impl Fn(usize) -> Option<&'a Element>,
    fiber_of: &impl Fn(&stalk::Invariant) -> Vec<Vec<u8>>,
    index_of: &impl Fn(&[u8]) -> Option<usize>,
    shape_at: &impl Fn(usize) -> Option<&'a Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> Sentence {
    let mut out = Sentence::default();
    if chain.is_empty() {
        return out;
    }
    let whole = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, chain, exclude);
    if let Some(k) = body_key(chain) {
        out.multiplicity.insert(k, whole.2);
    }
    match whole.0 {
        Some(_) => {
            place_located(&mut out, whole, chain, word_at, fiber_of, index_of, shape_at, exclude);
        }
        None => {
            for body in pieces(chain) {
                let loc = locate_or_puncture_on(word_at, fiber_of, index_of, shape_at, &body, exclude);
                place_located(&mut out, loc, &body, word_at, fiber_of, index_of, shape_at, exclude);
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
