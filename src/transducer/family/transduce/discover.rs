// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! DYNAMIC ENCLOSURE DISCOVERY (MU-SENSING §0, #32 S4): an unranked stream of
//! symbols carries no hardcoded "2D" or dataset header. Dimension is an
//! emergent topological invariant discovered by folding the stream's length
//! `L = p₀ · p₁ · … · pₙ₋₁` into the phase-circle enclosure that:
//!
//! 1. minimizes same-symbol connected components `β₀(P)` (folding shattered
//!    lower-rank segments into closed higher-rank bodies),
//! 2. maximizes exact rational interior coherence `C(P) = M(P) / E(P)` via
//!    integer cross-multiplication (`u128`, zero floating point), and
//! 3. prefers the lowest rank on exact ties (never inventing a redundant axis).
//!
//! Every candidate factorisation is evaluated under the caller's `Budget`
//! without `HashMap`s or floating-point arithmetic.

use super::{Enclosure, TransductionRefusal};
use crate::physics::Budget;
use crate::transducer::sample::{Place, Sample};

/// THE ENCLOSURE DISCOVERED FROM AN UNRANKED STREAM: the inferred phase
/// periods, its `β₀` component count, and its exact interior coherence
/// `(matches, edges)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Discovered {
    pub enclosure: Enclosure,
    pub components: usize,
    pub matches: usize,
    pub edges: usize,
}

impl Discovered {
    /// LIFT THE UNRANKED STREAM INTO A RANK-`n` `Sample` using the discovered
    /// phase periods, with symbol width `1` at `place`.
    pub fn into_sample(self, symbols: Vec<u8>, place: Place) -> Result<Sample, TransductionRefusal> {
        Sample::new(self.enclosure.periods().to_vec(), 1, symbols, place).map_err(TransductionRefusal::Sample)
    }
}

/// Union-find root with path halving.
fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Union two indices; returns `true` when two distinct components merged.
fn union(parent: &mut [usize], a: usize, b: usize) -> bool {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra == rb {
        return false;
    }
    parent[rb] = ra;
    true
}

/// Measure `(β₀, matches, edges)` for a candidate enclosure `periods` over
/// `symbols` (length `∏ periods`).
fn measure_enclosure(symbols: &[u8], periods: &[usize], budget: &mut Budget) -> Result<(usize, usize, usize), TransductionRefusal> {
    let n = symbols.len();
    let rank = periods.len();
    let mut strides = vec![1usize; rank];
    if let Some(last) = rank.checked_sub(1) {
        for k in (0..last).rev() {
            strides[k] = strides[k + 1].checked_mul(periods[k + 1]).ok_or(TransductionRefusal::Empty)?;
        }
    }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut components = n;
    let mut matches = 0usize;
    let mut edges = 0usize;

    for (axis, (&period, &stride)) in periods.iter().zip(&strides).enumerate() {
        let block = stride.checked_mul(period).ok_or(TransductionRefusal::Empty)?;
        let _ = axis;
        for i in 0..n {
            if !budget.spend() {
                return Err(TransductionRefusal::Spent { need: None });
            }
            let coord = (i % block) / stride;
            if coord + 1 < period {
                let j = i + stride;
                edges += 1;
                if symbols[i] == symbols[j] {
                    matches += 1;
                    if union(&mut parent, i, j) {
                        components -= 1;
                    }
                }
            }
        }
    }

    Ok((components, matches, edges))
}

/// Whether candidate `(comp_a, m_a, e_a, rank_a)` strictly improves on
/// `(comp_b, m_b, e_b, rank_b)`:
/// 1. fewer connected components `β₀`,
/// 2. higher exact rational interior coherence `m / e`,
/// 3. lower rank (no redundant dimension).
fn better(a: (usize, usize, usize, usize), b: (usize, usize, usize, usize)) -> bool {
    let (ca, ma, ea, ra) = a;
    let (cb, mb, eb, rb) = b;
    if ca != cb {
        return ca < cb;
    }
    let lhs = num_bigint::BigUint::from(ma) * num_bigint::BigUint::from(eb);
    let rhs = num_bigint::BigUint::from(mb) * num_bigint::BigUint::from(ea);
    if lhs != rhs {
        return lhs > rhs;
    }
    ra < rb
}

/// Enumerate all ordered factorisations `∏ periods = rem` with each factor ≥ 2
/// up to `max_rank` axes.
fn factorisations(rem: usize, max_rank: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>, budget: &mut Budget) -> Result<(), TransductionRefusal> {
    if !budget.spend() {
        return Err(TransductionRefusal::Spent { need: None });
    }
    if prefix.len() < max_rank {
        let mut full = prefix.clone();
        full.push(rem);
        out.push(full);
    }
    if prefix.len() + 1 >= max_rank {
        return Ok(());
    }
    let mut d = 2usize;
    while d * 2 <= rem {
        if !budget.spend() {
            return Err(TransductionRefusal::Spent { need: None });
        }
        if rem % d == 0 {
            prefix.push(d);
            factorisations(rem / d, max_rank, prefix, out, budget)?;
            prefix.pop();
        }
        d += 1;
    }
    Ok(())
}

/// DISCOVER THE ENCLOSURE OF AN UNRANKED STREAM `symbols` up to `max_rank`
/// orthogonal phase axes (each period ≥ 2). A stream of length 1 stays `[1]`.
pub fn discover_enclosure(symbols: &[u8], max_rank: usize, budget: &mut Budget) -> Result<Discovered, TransductionRefusal> {
    if symbols.is_empty() || max_rank == 0 {
        return Err(TransductionRefusal::Empty);
    }
    if symbols.len() == 1 {
        return Ok(Discovered {
            enclosure: Enclosure::new(vec![1])?,
            components: 1,
            matches: 0,
            edges: 0,
        });
    }
    let mut candidates = Vec::new();
    let mut prefix = Vec::new();
    factorisations(symbols.len(), max_rank, &mut prefix, &mut candidates, budget)?;

    let mut best: Option<(Vec<usize>, usize, usize, usize)> = None;
    for periods in candidates {
        let (components, matches, edges) = measure_enclosure(symbols, &periods, budget)?;
        let rank = periods.len();
        let replace = match &best {
            None => true,
            Some((bp, bc, bm, be)) => better((components, matches, edges, rank), (*bc, *bm, *be, bp.len())),
        };
        if replace {
            best = Some((periods, components, matches, edges));
        }
    }

    let Some((periods, components, matches, edges)) = best else {
        return Err(TransductionRefusal::Empty);
    };
    Ok(Discovered {
        enclosure: Enclosure::new(periods)?,
        components,
        matches,
        edges,
    })
}
