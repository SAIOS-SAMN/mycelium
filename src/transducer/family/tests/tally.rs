// 78.16, RC2 — THE CENSUS'S CELLS LINE, read off the typed cells the percept
// held for the observer: one reading, the line a test's own (C3).

use crate::observe::{CellClosure, CellCoherence, CellDual, CellEnclosure, CellOrbit, CellPermutes, CellRound, Cells};
use crate::transducer::cga::PairRegime;

/// ONE GRADE'S ROW OF THE CELLS' READING.
#[derive(Default)]
struct GradeRow {
    grade: usize,
    /// congruent, held ℤ₂, incongruent, unread, no round, no circuit.
    congruence: [u64; 6],
    /// all, moves, unread, no constructors, no circuit.
    permutes: [u64; 5],
    /// closes, held, open, unread.
    coherence: [u64; 4],
    capacity: Vec<usize>,
    /// closed, open, unbounded, unread.
    constructors: [u64; 4],
    /// The span signatures `(p, q, z)` read, with the vector rank.
    signatures: Vec<((usize, usize, usize), usize)>,
    /// Pairs by regime: finite, parabolic, boost, rosette, null.
    pairs: [u64; 5],
    /// GR2, by closure: finite by the theorem, finite by the check, infinite
    /// by a pair, affine, hyperbolic, by a product; unbounded; unread — the
    /// nuclei as mirrors, then the polars as mirrors.
    coxeter: [u64; 8],
    polars: [u64; 8],
    /// DF1: a dual frame, redundant, degenerate.
    dual: [u64; 3],
    /// CX1: per cell, the products kept and their distinct classes.
    products: Vec<(usize, usize)>,
    /// MG1: each nucleus against each bracketing mirror — through its
    /// centre, cutting it, tangent, apart.
    mirrors: [u64; 4],
    /// EN1: enclosures flat, round; the form's other bodies against the
    /// rounds — inside, on, apart, wrapping, unread; rounds nested in
    /// another cell's.
    enclosure: [u64; 8],
    /// CV1b — what the readers could not hold, by content: circuits unread
    /// by OR3's reason; modes unread (not a versor, irrational, defective,
    /// spent); the capacity's refusals (none read, not a versor, spent at
    /// these products, these entries overflowing, the elimination at these
    /// products); ZM1's zero meets; nuclei unread; modes of no admitted
    /// order; the orbit's refusals (no reflection, no versor, no ray, carry
    /// refused, carried with no ray).
    circuits_unread: [u64; 8],
    modes_unread: [u64; 4],
    capacity_refused: [u64; 2],
    capacity_spent: Vec<usize>,
    capacity_overflow: Vec<(usize, usize, u64)>,
    capacity_elimination: Vec<usize>,
    zero_meets: u64,
    nuclei_unread: u64,
    unordered: u64,
    orbit_unread: [u64; 5],
}

/// The tally's axis for a group's closure; an unread reading on the last
/// axis.
fn closure_axis(r: Option<CellClosure>) -> usize {
    match r {
        Some(CellClosure::FiniteByTheorem) => 0,
        Some(CellClosure::FiniteByCheck) => 1,
        Some(CellClosure::InfiniteByPair) => 2,
        Some(CellClosure::Affine) => 3,
        Some(CellClosure::Hyperbolic) => 4,
        Some(CellClosure::InfiniteByProduct) => 5,
        Some(CellClosure::Unbounded) => 6,
        Some(CellClosure::Unread) | None => 7,
    }
}

/// An arm's axis: its place in its own list of arms.
fn axis<T: PartialEq + Copy>(all: &[T], x: T) -> usize {
    match all.iter().position(|&a| a == x) {
        Some(i) => i,
        None => all.len(),
    }
}

/// THE CELLS' READING BY GRADE, one row per grade, ascending, and the
/// form's mirrors.
#[derive(Default)]
pub(super) struct CellTally {
    rows: Vec<GradeRow>,
    planes: usize,
    mirror_group: Option<CellClosure>,
}

impl CellTally {
    /// The tally of the typed cells the percept held (RC2): the census line
    /// is read off the observer's own reading, never a second one.
    pub(super) fn of(cells: &Cells) -> CellTally {
        let mut t =
            CellTally { planes: cells.mirror_planes as usize, mirror_group: cells.mirror_group, ..CellTally::default() };
        for c in &cells.cells {
            let grade = usize::from(c.grade);
            let at = match t.rows.binary_search_by_key(&grade, |r| r.grade) {
                Ok(at) => at,
                Err(at) => {
                    t.rows.insert(at, GradeRow { grade, ..GradeRow::default() });
                    at
                }
            };
            let row = &mut t.rows[at];
            row.congruence[axis(&CellRound::ALL, c.round)] += 1;
            row.permutes[axis(&CellPermutes::ALL, c.permutes)] += 1;
            row.coherence[axis(&CellCoherence::ALL, c.coherence)] += 1;
            if let Some(k) = c.capacity {
                row.capacity.push(k as usize);
            }
            row.constructors[axis(&CellOrbit::ALL, c.orbit)] += 1;
            row.coxeter[closure_axis(c.coxeter)] += 1;
            if c.polars.is_some() {
                row.polars[closure_axis(c.polars)] += 1;
            }
            match c.dual {
                Some(CellDual::Frame) => row.dual[0] += 1,
                Some(CellDual::Redundant) => row.dual[1] += 1,
                Some(CellDual::Degenerate) => row.dual[2] += 1,
                None => {}
            }
            row.products.push((c.products.len(), usize::from(c.classes)));
            // CV1b — the refusals by content.
            use crate::observe::{CapacityRefused, CircuitUnread, ModesUnread, OrbitUnread};
            let r = &c.refusals;
            for w in &r.circuits {
                row.circuits_unread[axis(&CircuitUnread::ALL, *w)] += 1;
            }
            for m in &r.modes {
                row.modes_unread[match m {
                    ModesUnread::NotVersor => 0,
                    ModesUnread::Irrational(_) => 1,
                    ModesUnread::Defective => 2,
                    ModesUnread::Unbounded(_) => 3,
                }] += 1;
            }
            match r.capacity {
                Some(CapacityRefused::NoneRead) => row.capacity_refused[0] += 1,
                Some(CapacityRefused::NotVersor { .. }) => row.capacity_refused[1] += 1,
                Some(CapacityRefused::Spent { product }) => row.capacity_spent.push(product),
                Some(CapacityRefused::Overflow { product, entry, bits }) => {
                    row.capacity_overflow.push((product, entry, bits))
                }
                Some(CapacityRefused::Elimination { product }) => row.capacity_elimination.push(product),
                None => {}
            }
            row.zero_meets += u64::from(r.zero_meet);
            row.nuclei_unread += u64::from(r.nuclei);
            row.unordered += r.unordered as u64;
            if let Some(o) = r.orbit {
                row.orbit_unread[match o {
                    OrbitUnread::NotConstructor { .. } => 0,
                    OrbitUnread::NotVersor { .. } => 1,
                    OrbitUnread::NoRay { .. } => 2,
                    OrbitUnread::CarryRefused => 3,
                    OrbitUnread::CarriedNoRay => 4,
                }] += 1;
            }
            match &c.enclosure {
                Some(CellEnclosure::Flat) => row.enclosure[0] += 1,
                Some(CellEnclosure::Round { bodies, within }) => {
                    row.enclosure[1] += 1;
                    for (slot, n) in row.enclosure[2..7].iter_mut().zip(bodies) {
                        *slot += u64::from(*n);
                    }
                    row.enclosure[7] += u64::from(within.is_some());
                }
                None => {}
            }
            for (_, _, _, regime) in c.mirrors.iter().flatten() {
                row.mirrors[match regime {
                    PairRegime::Finite(2) => 0,
                    PairRegime::Parabolic => 2,
                    PairRegime::Boost => 3,
                    _ => 1,
                }] += 1;
            }
            if let Some(((p, q, z), rank)) = c.signature {
                row.signatures.push(((usize::from(p), usize::from(q), usize::from(z)), usize::from(rank)));
                for r in &c.pairs {
                    row.pairs[match r {
                        PairRegime::Finite(_) => 0,
                        PairRegime::Parabolic => 1,
                        PairRegime::Boost => 2,
                        PairRegime::Rosette => 3,
                        PairRegime::Null => 4,
                    }] += 1;
                }
            }
        }
        t
    }

    /// The census line's segment.
    pub(super) fn line(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .map(|r| {
                let (cg, pm, ch, co, pr) = (r.congruence, r.permutes, r.coherence, r.constructors, r.pairs);
                let axes = |x: &[u64; 8]| {
                    format!(
                        "finite(theorem) {} finite(check) {} infinite(pair) {} affine {} hyperbolic {} infinite(product) {} unbounded {} unread {}",
                        x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]
                    )
                };
                let en = r.enclosure;
                let head = format!(
                    "grade {}: round kept {} held-z2 {} carried {} unread {} no-round {} no-circuit {} | constructors kept {} moved {} unread {} | coherence closes {} held {} open {} unread {} | capacity {:?} | orbit closed {} open {} unbounded {} unread {} | gram (sig, rank) {:?} pairs finite {} parabolic {} boost {} rosette {} null {} | coxeter {} | polars {} | dual frame {} redundant {} degenerate {} | products (kept, classes) {:?} | nuclei on mirrors: through {} cut {} tangent {} apart {}",
                    r.grade, cg[0], cg[1], cg[2], cg[3], cg[4], cg[5], pm[0], pm[1], pm[2], ch[0], ch[1], ch[2], ch[3],
                    r.capacity, co[0], co[1], co[2], co[3], r.signatures, pr[0], pr[1], pr[2], pr[3], pr[4],
                    axes(&r.coxeter), axes(&r.polars), r.dual[0], r.dual[1], r.dual[2], r.products,
                    r.mirrors[0], r.mirrors[1], r.mirrors[2], r.mirrors[3]
                );
                let cu = r.circuits_unread;
                let (mu, ou) = (r.modes_unread, r.orbit_unread);
                format!(
                    "{head} | enclosures flat {} round {}: bodies inside {} on {} apart {} wraps {} unread {}, nested {} | refused: circuits unread cone {} no-place {} not-rigid {} unrefracted {} no-face {} two-ways {} coset {} unnamed {} | modes unread not-versor {} irrational {} defective {} spent {} | capacity none-read {} not-versor {} spent at {:?} overflow (product, entry, bits) {:?} elimination at {:?} | zero meets {} | nuclei unread {} | modes of no admitted order {} | orbit no-reflection {} no-versor {} no-ray {} carry-refused {} carried-no-ray {}",
                    en[0], en[1], en[2], en[3], en[4], en[5], en[6], en[7],
                    cu[0], cu[1], cu[2], cu[3], cu[4], cu[5], cu[6], cu[7],
                    mu[0], mu[1], mu[2], mu[3],
                    r.capacity_refused[0], r.capacity_refused[1], r.capacity_spent, r.capacity_overflow, r.capacity_elimination,
                    r.zero_meets, r.nuclei_unread, r.unordered,
                    ou[0], ou[1], ou[2], ou[3], ou[4]
                )
            })
            .collect();
        let mirrors = match self.mirror_group {
            Some(g) => format!("mirrors: {} planes, group {}", self.planes, g.label()),
            None => "mirrors: none".into(),
        };
        match rows.is_empty() {
            true => "cells: none certified".into(),
            false => format!("cells: {} | {mirrors}", rows.join(" ; ")),
        }
    }
}
