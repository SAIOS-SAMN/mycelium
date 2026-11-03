// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE GROUND Ω, DEDUCED (MU-SENSING §3.4; ROADMAP track 26, S2): the
//! symbol a family's bodies stand on, read off the family's samples and its
//! pairs' transductions, never set to a code. A symbol is a CANDIDATE
//! exactly when it is
//! 1. **ambient**: present in every sample of the family;
//! 2. **enclosing**: in every sample, some site of it meets the sample's
//!    seam (a coordinate at `0` or at `extent − 1` on some axis), so the
//!    bodies stand inside it;
//! 3. **never carried as a body**: no pair carries a body of it, by a signed
//!    permutation of the phases and a non-zero offset, onto a body of it
//!    elsewhere in the output that the pair's transduction does not hold in
//!    place. A ground carries no structure the family's motions preserve.
//!    Sites of it covered or uncovered (a body painted onto it, or lifted
//!    off it) and re-periods (crops, scalings) read nothing here.
//!
//! One candidate is [`Ground::One`]; several are held as a vortex
//! ([`Ground::Vortex`], MU-SENSING §3.8), never one picked by a scalar, an
//! order or a count; none is [`Ground::None`], every symbol a body.
//!
//! ORDER OF READING. The transductions read no ground (their incidences and
//! bodies are symbol-blind but for equality), so the order is: the samples,
//! then each pair's transduction and moved frames, then the ground, and only
//! then the readings that take the ground (`Transduction::lane_shares`).

use super::relation::{centred_shape, Held};
use super::{need_of, Enclosed, SignedPermutation, Transduction, Turn};
use crate::transducer::sample::{Sample, Symbol};

/// THE GROUND OF A FAMILY, a typed reading over the symbol carrier `S` (a
/// sample's owned symbol bytes, or a raster's one byte).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ground<S> {
    /// Exactly one candidate.
    One(S),
    /// Several candidates, held in the order the family's samples meet
    /// them (an order of content, never a rank among them).
    Vortex(Vec<S>),
    /// No candidate: every symbol is a body, the support the whole sample.
    None,
}

/// WHY A WRITER HAS NO ONE SYMBOL OF GROUND TO WRITE, named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unground {
    /// The ground is a vortex: none of its candidates is written for it.
    Vortex,
    /// The family reads no ground.
    None,
    /// No family is in scope, so no ground is read at all.
    Unscoped,
}

impl<S: PartialEq> Ground<S> {
    /// IS `symbol` READ AS GROUND under this reading: the one candidate, any
    /// candidate of a vortex (each is ambient, enclosing and never carried
    /// as a body, so the support is what no candidate claims), nothing under
    /// `None`.
    pub fn holds(&self, symbol: &S) -> bool {
        match self {
            Ground::One(g) => g == symbol,
            Ground::Vortex(gs) => gs.contains(symbol),
            Ground::None => false,
        }
    }

    /// THE ONE SYMBOL A WRITER WRITES FOR THE GROUND, or why there is none.
    pub fn symbol(&self) -> Result<&S, Unground> {
        match self {
            Ground::One(g) => Ok(g),
            Ground::Vortex(_) => Err(Unground::Vortex),
            Ground::None => Err(Unground::None),
        }
    }

    /// The reading over another symbol carrier, each candidate mapped; a
    /// candidate `f` cannot carry is left out of the reading.
    pub fn map<T: PartialEq>(&self, f: impl Fn(&S) -> Option<T>) -> Ground<T> {
        let candidates: Vec<T> = match self {
            Ground::One(g) => f(g).into_iter().collect(),
            Ground::Vortex(gs) => gs.iter().filter_map(&f).collect(),
            Ground::None => Vec::new(),
        };
        Ground::from_candidates(candidates)
    }

    /// The reading the candidates make: none, one, or a vortex.
    fn from_candidates(mut candidates: Vec<S>) -> Ground<S> {
        match candidates.len() {
            0 => Ground::None,
            1 => match candidates.pop() {
                Some(g) => Ground::One(g),
                None => Ground::None,
            },
            _ => Ground::Vortex(candidates),
        }
    }
}

/// Does some site of `symbol` in `x` meet the sample's seam?
fn meets_seam(x: &Sample, symbol: &[u8]) -> bool {
    let extents: Vec<usize> = (0..x.rank())
        .filter_map(|axis| match x.extent(axis) {
            crate::transducer::sample::AxisReading::Read(e) => Some(e),
            crate::transducer::sample::AxisReading::Unread => None,
        })
        .collect();
    x.sites().zip(x.symbols()).any(|(site, s)| {
        s.bytes() == symbol
            && site.axes().iter().zip(&extents).any(|(&c, &e)| c == 0 || usize::try_from(c).is_ok_and(|c| c + 1 == e))
    })
}

/// A body's least boundary turn on each phase: its place in the frame.
fn place_of(e: &Enclosed, body: usize) -> Vec<Turn> {
    let mut least: Option<Vec<usize>> = None;
    for (_, b) in e.body_bands(body) {
        least = Some(match least {
            None => b.spans.iter().map(|&(lo, _)| lo).collect(),
            Some(held) => held.iter().zip(&b.spans).map(|(&l, &(lo, _))| l.min(lo)).collect(),
        });
    }
    match least {
        Some(least) => least.iter().zip(e.enclosure.periods()).map(|(&i, &p)| Turn::new(i, p)).collect(),
        None => Vec::new(),
    }
}

/// THE SYMBOLS PAIR `(x, y)` CARRIES AS A BODY: a body of `x`, moved by a
/// signed permutation of the phases, whose shape (its boundary turns in the
/// frame's periods, centred by its least turn on each phase) is the shape
/// of a body of `y` of its symbol standing at another place, where that
/// body of `y` is not the carriage under the re-period of a body of `x` of
/// its symbol (it is not held in place). A pair of two ranks, or whose
/// transduction does not read, relates no body and carries none.
fn carried_as_body(x: &Sample, y: &Sample) -> Vec<Vec<u8>> {
    let from = Enclosed::of(x);
    let to = Enclosed::of(y);
    let Some(need) = need_of(x, y) else { return Vec::new() };
    let Ok(t) = Transduction::of(x, y, &mut crate::physics::Physics::default().stroke_n(need)) else { return Vec::new() };
    // The output bodies held in place: the carriage of an input body of one
    // symbol with them.
    let held: Vec<usize> = t
        .relations()
        .into_iter()
        .enumerate()
        .filter_map(|(b, onto)| onto.ok().filter(|&q| from.body_symbol(b).is_some() && from.body_symbol(b) == to.body_symbol(q)))
        .collect();
    let loose: Vec<(usize, Vec<u8>, Vec<Turn>)> = (0..to.bodies)
        .filter(|q| !held.contains(q))
        .map(|q| (q, centred_shape(&to, q, Held::InFrame), place_of(&to, q)))
        .collect();
    let mut out: Vec<Vec<u8>> = Vec::new();
    if loose.is_empty() {
        return out;
    }
    for s in SignedPermutation::all(from.enclosure.rank()) {
        let Ok(moved) = s.moved(&from) else { continue };
        for b in 0..moved.bodies {
            let Some(symbol) = moved.body_symbol(b) else { continue };
            let shape = centred_shape(&moved, b, Held::InFrame);
            let place = place_of(&moved, b);
            let carried = loose
                .iter()
                .any(|(q, s, p)| to.body_symbol(*q) == Some(symbol) && *s == shape && *p != place);
            if carried && !out.iter().any(|o| o.as_slice() == symbol.bytes()) {
                out.push(symbol.bytes().to_vec());
            }
        }
    }
    out
}

impl Ground<Vec<u8>> {
    /// IS THE SAMPLE'S `symbol` READ AS GROUND ([`Ground::holds`] on the
    /// symbol's bytes).
    pub fn holds_symbol(&self, symbol: Symbol<'_>) -> bool {
        match self {
            Ground::One(g) => g.as_slice() == symbol.bytes(),
            Ground::Vortex(gs) => gs.iter().any(|g| g.as_slice() == symbol.bytes()),
            Ground::None => false,
        }
    }

    /// THE GROUND OF A FAMILY OF PAIRS (MU-SENSING §3.4): the candidates
    /// that are ambient, enclosing and never carried as a body. A family
    /// with no sample has no ambient symbol and reads `None`.
    pub fn of(family: &[(Sample, Sample)]) -> Ground<Vec<u8>> {
        let samples: Vec<&Sample> = family.iter().flat_map(|(x, y)| [x, y]).collect();
        // Ambient: the family's symbols in the order its samples meet them
        // (the family's own order, each sample in canonical address order),
        // kept where every sample shows them. A family with no sample has
        // no symbol, and reads `None`.
        let mut candidates: Vec<Vec<u8>> = Vec::new();
        for s in samples.iter().flat_map(|x| x.symbols()) {
            if !candidates.iter().any(|c| c.as_slice() == s.bytes()) {
                candidates.push(s.bytes().to_vec());
            }
        }
        candidates.retain(|c| samples.iter().all(|x| x.symbols().any(|s| s.bytes() == c.as_slice())));
        // Enclosing: meeting every sample's seam.
        candidates.retain(|c| samples.iter().all(|x| meets_seam(x, c)));
        // Never carried as a body by any pair.
        for (x, y) in family {
            if candidates.is_empty() {
                break;
            }
            let carried = carried_as_body(x, y);
            candidates.retain(|c| !carried.contains(c));
        }
        Ground::from_candidates(candidates)
    }
}
