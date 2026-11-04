// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE GROUND Ω, DEDUCED (MU-SENSING §3.4; ROADMAP track 26, S2): the
//! symbol a family's bodies stand on, read off the family's samples and its
//! pairs' transductions, never set to a code. A symbol is a CANDIDATE
//! exactly when it is
//! 1. **ambient**: present in every INPUT sample of the family (covered
//!    whole in an output, it is painted over and stays the ground);
//! 2. **enclosing**: in every sample that shows it, some site of it meets
//!    the sample's outer seam (a coordinate at `0` or at `extent − 1` on
//!    some axis), or the inner seam of a body covering that whole seam, the
//!    sample's MEMBRANE, level by level (`membrane::levels`). A membrane is
//!    never the ground: a body is read as one only where its symbol is no
//!    candidate on the outer seam;
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

    /// THE PERSPECTIVES THE READING IS READ IN (MU-SENSING §3.8, §4): one
    /// per candidate, each `Ground::One` of it, in the vortex's order;
    /// `One` and `None` are one perspective, themselves. A reader or writer
    /// that needs one ground runs once per perspective and keeps every
    /// result as the vortex's sections; nothing picks among them.
    pub fn perspectives(&self) -> Vec<Ground<S>>
    where
        S: Clone,
    {
        match self {
            Ground::One(g) => vec![Ground::One(g.clone())],
            Ground::Vortex(gs) => gs.iter().map(|g| Ground::One(g.clone())).collect(),
            Ground::None => vec![Ground::None],
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
        let candidates = Ground::candidacies(family)
            .into_iter()
            .filter_map(|(symbol, reading)| match reading {
                Candidacy::Candidate => Some(symbol),
                _ => None,
            })
            .collect();
        Ground::from_candidates(candidates)
    }

    /// EVERY SYMBOL OF THE FAMILY WITH ITS CANDIDACY, in the order the
    /// family's samples meet them (the family's own order, each sample in
    /// canonical address order): the first of the three conditions it
    /// fails, named with the sample or pair that fails it, or `Candidate`.
    pub fn candidacies(family: &[(Sample, Sample)]) -> Vec<(Vec<u8>, Candidacy)> {
        let samples: Vec<&Sample> = family.iter().flat_map(|(x, y)| [x, y]).collect();
        let mut symbols: Vec<Vec<u8>> = Vec::new();
        for s in samples.iter().flat_map(|x| x.symbols()) {
            if !symbols.iter().any(|c| c.as_slice() == s.bytes()) {
                symbols.push(s.bytes().to_vec());
            }
        }
        // Each sample's seams, peeled through its membranes, read once.
        let seams: Vec<Vec<super::membrane::Level>> = samples.iter().map(|x| super::membrane::levels(x)).collect();
        let readings: Vec<Seamed> = symbols
            .into_iter()
            .map(|c| {
                // 1. Present in every input sample: covered whole in an
                // output, it is painted over and stays the ground.
                let absent = family.iter().position(|(x, _)| !x.symbols().any(|s| s.bytes() == c.as_slice()));
                // 2. In every sample that shows it, on the outer seam
                // (`outer`), or else on the inner seam of the membranes
                // covering the seam outside it (`inner`, with the membranes
                // it was read through).
                let mut outer = None;
                let mut inner: Result<Vec<Vec<u8>>, usize> = Ok(Vec::new());
                // The first sample whose inner seam it meets but does not
                // fill: a framed interior is the symbol meeting the
                // membrane's ENTIRE inner seam.
                let mut partial = None;
                for (i, (x, levels)) in samples.iter().zip(&seams).enumerate() {
                    if !x.symbols().any(|s| s.bytes() == c.as_slice()) {
                        continue;
                    }
                    let depth = levels.iter().position(|l| l.seam.contains(&c));
                    if depth != Some(0) && outer.is_none() {
                        outer = Some(i);
                    }
                    if let Some(d) = depth.filter(|&d| d > 0) {
                        if partial.is_none() && levels.get(d).is_some_and(|l| l.seam.as_slice() != std::slice::from_ref(&c)) {
                            partial = Some(i);
                        }
                    }
                    match (depth, &mut inner) {
                        (None, Ok(_)) => inner = Err(i),
                        (Some(d), Ok(through)) => {
                            for l in levels.iter().take(d) {
                                if let Some(m) = &l.membrane {
                                    through.push(m.clone());
                                }
                            }
                        }
                        (_, Err(_)) => {}
                    }
                }
                // Something stands on it: in some input, its one body
                // covers a whole seam and a body lies inside it (a level
                // read past its membrane).
                let holds = family.iter().zip(seams.chunks(2)).position(|(_, of_pair)| {
                    of_pair.iter().take(1).any(|levels| {
                        levels.iter().enumerate().any(|(d, l)| l.membrane.as_ref() == Some(&c) && levels.len() > d + 1)
                    })
                });
                Seamed { symbol: c, absent, outer, inner, partial, holds }
            })
            .collect();
        // Condition 3, which needs each pair's transduction, is read only
        // where some symbol passes 1 and 2, each pair's transduction once.
        let carried: Vec<Vec<Vec<u8>>> = match readings.iter().any(|r| r.absent.is_none() && r.inner.is_ok()) {
            true => family.iter().map(|(x, y)| carried_as_body(x, y)).collect(),
            false => Vec::new(),
        };
        let moved = |c: &Vec<u8>| carried.iter().position(|of_pair| of_pair.contains(c));
        // The candidates on the outer seam alone. A body covering a whole
        // seam is read as a membrane only where its symbol is no such
        // candidate: a membrane is never the ground, and a candidate on the
        // outer seam is never peeled as a membrane, UNLESS the interior it
        // frames is a ground of its own: a symbol meeting its entire inner
        // seam in every sample that reads it there, on which a body stands
        // in some input (ruled 2026-11-04, "the ground is what bodies stand
        // on"). Then both are candidates, held as a vortex; a lone dot,
        // on which nothing stands, never is.
        let direct: Vec<Vec<u8>> = readings
            .iter()
            .filter(|r| r.absent.is_none() && r.outer.is_none() && moved(&r.symbol).is_none())
            .map(|r| r.symbol.clone())
            .collect();
        readings
            .into_iter()
            .map(|r| {
                let framed = r.partial.is_none() && r.holds.is_some();
                let reading = match (r.absent, r.outer, &r.inner) {
                    (Some(pair), _, _) => Candidacy::Absent { pair },
                    (None, Some(sample), Err(_)) => Candidacy::Unenclosing { sample },
                    (None, Some(sample), Ok(through)) if !framed && through.iter().any(|m| direct.contains(m)) => Candidacy::Unenclosing { sample },
                    (None, _, _) => match moved(&r.symbol) {
                        Some(pair) => Candidacy::CarriedAsBody { pair },
                        None => Candidacy::Candidate,
                    },
                };
                (r.symbol, reading)
            })
            .collect()
    }
}

/// One symbol's reading against conditions 1 and 2: the pair whose input
/// lacks it, the first sample whose outer seam it misses, and the
/// membranes it was read through on inner seams (or the first sample where
/// no seam shows it).
struct Seamed {
    symbol: Vec<u8>,
    absent: Option<usize>,
    outer: Option<usize>,
    inner: Result<Vec<Vec<u8>>, usize>,
    /// The first sample whose inner seam it meets without filling it.
    partial: Option<usize>,
    /// The first pair whose input it holds a body in.
    holds: Option<usize>,
}

/// HOW ONE SYMBOL STANDS AGAINST THE THREE CONDITIONS (MU-SENSING §3.4):
/// a candidate, or the first condition it fails with where it fails it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Candidacy {
    /// Ambient, enclosing and never carried as a body.
    Candidate,
    /// Condition 1: absent from the input of the family's `pair`-th pair.
    Absent { pair: usize },
    /// Condition 2: in the `sample`-th sample (input and output of each pair
    /// in turn) that shows it, no site of it meets the outer seam, and it
    /// meets no inner seam of a membrane in every sample (or the body it
    /// would be read through is itself a candidate on the outer seam).
    Unenclosing { sample: usize },
    /// Condition 3: the `pair`-th pair carries a body of it elsewhere.
    CarriedAsBody { pair: usize },
}
