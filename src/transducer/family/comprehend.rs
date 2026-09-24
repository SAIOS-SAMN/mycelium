// 78.11, 78.14, CG1, CG1b, GR1, BD2 — EACH CERTIFIED CELL OF A FORM READ AS
// ITS ORBIT ALGEBRA: the circuits that cross it, their holonomies through the
// one composite reader (OR3), the cell's round, constructors and Gram form —
// and the census's reading of them by grade.
//
//! A circuit CROSSES a cell where one of its steps crosses a contact face
//! between two different bodies of the cell: it passes through the cell's
//! contact, where its transport meets the cell's round (78.13(3)). The
//! step's bodies are read off the face it crosses — the form's side carrier
//! of each contact face's body pair (BD2) — never derived from its member,
//! whose support can fall into several bodies (checklist (m)). One keyed
//! read per step, never a pairwise read.
//!
//! MG1 — each nucleus is read against the base's mirrors that bracket its
//! centre, and the form's mirrors — every distinct one bracketing any of its
//! cells' nuclei — are read as a reflection group by GR2: the grid's own
//! Coxeter reading beside the cells'. CO1 — the whole is a typed reading
//! the observer serves on demand ([`take_cells`]).

use crate::network::nerve::Keyed;
use crate::observe::{
    CellBracket, CellClosure, CellCoherence, CellDual, CellEnclosure, CellEntry, CellOrbit, CellPermutes, CellProduct,
    CellRound, Cells,
    GradeProgress, Progress, Spend, Viscosity,
};
use crate::transducer::cech::{CellAlgebra, Closure, Coherence, Congruence, CoxeterReading, Permutes, Refusal};
use crate::transducer::cga::{self, Containment, ConstructorOrbit, DualFrame, MirrorReading, Mv};
use crate::transducer::sentence::Form;

/// ONE FORM'S CELLS, COMPREHENDED: each cell's orbit algebra, each cell's
/// nuclei against their bracketing mirrors (MG1), and the form's mirrors
/// read as a group.
pub(crate) struct Comprehension {
    pub(crate) algebras: Vec<CellAlgebra>,
    pub(crate) mirrors: Vec<Vec<Vec<MirrorReading>>>,
    pub(crate) planes: usize,
    pub(crate) mirror_group: Option<CoxeterReading>,
    /// EN1 — each cell's enclosure, in the cells' key order.
    pub(crate) enclosures: Vec<Option<CellEnclosure>>,
    /// 78.26(2), CB1 — the cross-cell bracket; `None` for one cell.
    pub(crate) bracket: Option<CellBracket>,
    /// CS2 — each cell's held constructors, its nuclei, one per body; `None`
    /// where they did not read.
    pub(crate) constructors: Vec<Option<usize>>,
    /// CS2, 78.32(e) — the `n` of the base the form was read on, whose
    /// associative cap `(n+2)²` bounds the rank the capacity reads.
    pub(crate) n: u8,
}

/// 78.26(2), CB1 — THE CROSS-CELL BRACKET OF A SENTENCE SPANNING MORE THAN ONE
/// CELL: for every pair of crossing circuits — circuit `i` crossing cell `A`,
/// circuit `j` crossing cell `B ≠ A`, and NO ONE CELL crossed by both (a
/// pair one cell holds is that cell's own, its bracket in its closed algebra
/// by closure, and not cross-cell content) — whose actions do not commute,
/// the bracket `R_i R_j − R_j R_i` is HELD where a cell wrapping both (an
/// ancestor of both in EN1's nesting parse, or one of the two wrapping the
/// other) has a closed algebra containing it, LOST where no wrapping cell
/// does, UNREAD where a wrapping cell's algebra did not read — and UNREAD
/// where a generator of the pair did not read, or the two actions differ in
/// width: cross-cell content this reading cannot form is counted, never
/// skipped. Each pair of circuits is decided ONCE, against the UNION of the
/// cells wrapping every pair of cells the two cross (R1): a pair held by an
/// enclosure over any cells it spans is held. A reading: it spends nothing
/// of the stroke, two products per pair on actions the cells already read.
/// `None` where the sentence spans one cell.
pub(crate) fn read_bracket(
    form: &Form,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
    algebras: &[CellAlgebra],
    enclosures: &[Option<CellEnclosure>],
) -> Option<CellBracket> {
    if algebras.len() < 2 {
        return None;
    }
    // Each cell's crossing circuits by their place in the traversal, and its
    // read generators among them in the order its actions were read.
    let crossed: Vec<Vec<usize>> =
        form.cells.scoped().map(|(_, cell)| crossings(form, cell, traversed).into_iter().map(|(at, _)| at).collect()).collect();
    let generators: Vec<Vec<usize>> = crossed
        .iter()
        .map(|xs| xs.iter().copied().filter(|at| traversed.get(*at).is_some_and(|t| t.composite.is_some())).collect())
        .collect();
    let parents: Vec<Option<usize>> = (0..algebras.len())
        .map(|i| match enclosures.get(i) {
            Some(Some(CellEnclosure::Round { within: Some(p), .. })) => Some(*p as usize),
            _ => None,
        })
        .collect();
    Some(bracket_over(&crossed, &generators, algebras, &parents))
}

/// THE BRACKET'S CORE, over its inputs alone: each cell's crossing circuits,
/// its read generators in the order of its actions, its algebra, and its
/// parent in EN1's parse.
pub(crate) fn bracket_over(
    crossed: &[Vec<usize>],
    generators: &[Vec<usize>],
    algebras: &[CellAlgebra],
    parents: &[Option<usize>],
) -> CellBracket {
    // A pair some one cell is crossed by both of is that cell's own.
    let one_cell_holds = |i: usize, j: usize| crossed.iter().any(|xs| xs.contains(&i) && xs.contains(&j));
    // The cells wrapping `i`, up EN1's parse; the parse is a forest, and the
    // walk is bounded by the cells' count whatever it holds.
    let ancestors = |i: usize| -> Vec<usize> {
        let mut out = Vec::new();
        let mut at = parents.get(i).copied().flatten();
        while let Some(p) = at {
            if out.contains(&p) || out.len() >= algebras.len() {
                break;
            }
            out.push(p);
            at = parents.get(p).copied().flatten();
        }
        out
    };
    // The cells wrapping both `a` and `c`: their common ancestors, and either
    // of the two where it wraps the other.
    let wrapping_both = |a: usize, c: usize| -> Vec<usize> {
        let (up_a, up_c) = (ancestors(a), ancestors(c));
        let mut out: Vec<usize> = up_a.iter().copied().filter(|x| up_c.contains(x)).collect();
        if up_c.contains(&a) {
            out.push(a);
        }
        if up_a.contains(&c) {
            out.push(c);
        }
        out
    };
    // A crossing circuit's action in one cell, where it read there.
    let action = |cell: usize, at: usize| -> Option<&Vec<Vec<num_rational::BigRational>>> {
        let k = generators.get(cell)?.iter().position(|x| *x == at)?;
        algebras.get(cell)?.actions.as_ref()?.get(k)
    };
    let cells_of = |at: usize| -> Vec<usize> { (0..crossed.len()).filter(|&k| crossed[k].contains(&at)).collect() };
    // Every crossing circuit once, in its place's order.
    let mut circuits: Vec<usize> = crossed.iter().flatten().copied().collect();
    circuits.sort_unstable();
    circuits.dedup();
    let mut b = CellBracket::default();
    for (x, &i) in circuits.iter().enumerate() {
        for &j in circuits.iter().skip(x + 1) {
            // No one cell holds both, so every cell `i` crosses differs from
            // every cell `j` crosses: the pair spans each of those pairs.
            if one_cell_holds(i, j) {
                continue;
            }
            let (at_i, at_j) = (cells_of(i), cells_of(j));
            // R1 — THE WRAPPING SET IS THE UNION over every pair of cells the
            // two circuits cross.
            let mut wrapping: Vec<usize> = Vec::new();
            for &a in &at_i {
                for &c in &at_j {
                    for w in wrapping_both(a, c) {
                        if !wrapping.contains(&w) {
                            wrapping.push(w);
                        }
                    }
                }
            }
            // The two actions at one width, from any cells they read in.
            let read_i: Vec<_> = at_i.iter().filter_map(|&a| action(a, i)).collect();
            let read_j: Vec<_> = at_j.iter().filter_map(|&c| action(c, j)).collect();
            let same_width = read_i.iter().find_map(|g| read_j.iter().find(|h| h.len() == g.len()).map(|h| (*g, *h)));
            // A pair whose generator did not read, or whose two actions read
            // at no one width, has no bracket this reading can form: unread,
            // never skipped.
            let Some((g, h)) = same_width else {
                b.unread += 1;
                continue;
            };
            let (gh, hg) = (cga::matrix_mul(g, h), cga::matrix_mul(h, g));
            if gh == hg {
                continue;
            }
            let bracket: Vec<Vec<num_rational::BigRational>> =
                gh.iter().zip(&hg).map(|(r, s)| r.iter().zip(s).map(|(x, y)| x - y).collect()).collect();
            let reads: Vec<Option<bool>> =
                wrapping.iter().map(|&w| crate::transducer::cech::algebra_contains(&algebras[w], &bracket)).collect();
            match (reads.contains(&Some(true)), reads.contains(&None)) {
                (true, _) => b.held += 1,
                (false, true) => b.unread += 1,
                (false, false) => b.lost += 1,
            }
        }
    }
    b
}

/// EN1 — EVERY CELL'S ENCLOSURE READ AGAINST THE FORM'S OTHER BODIES: the
/// round its bodies' centres join (flat by name where they span none), each
/// body of the form outside the cell by containment against it, and the
/// cells' enclosures parsed by nesting. A body is one piece, its nucleus
/// read once, whichever cells it stands in.
pub(crate) fn read_enclosures(form: &Form) -> Vec<Option<CellEnclosure>> {
    let mut nucleus_of: Keyed<Mv> = Keyed::new();
    for (_, cell) in form.cells.scoped() {
        if let Some(nuclei) = &cell.nuclei {
            for (piece, s) in cell.bodies.iter().zip(nuclei) {
                if !nucleus_of.contains(piece) {
                    nucleus_of.insert(piece, s.clone());
                }
            }
        }
    }
    let rounds: Vec<Option<Option<Mv>>> =
        form.cells.scoped().map(|(_, c)| c.nuclei.as_ref().map(|ns| cga::enclosure(ns))).collect();
    // The parse over the cells whose enclosure is a round, by cell index.
    let standing: Vec<(usize, Mv)> =
        rounds.iter().enumerate().filter_map(|(i, r)| Some((i, r.clone()??))).collect();
    let parents = cga::nesting(&standing.iter().map(|(_, e)| e.clone()).collect::<Vec<_>>());
    let within = |i: usize| -> Option<u32> {
        let at = standing.iter().position(|(j, _)| *j == i)?;
        parents[at].map(|p| standing[p].0 as u32)
    };
    form.cells
        .scoped()
        .zip(&rounds)
        .enumerate()
        .map(|(i, ((_, cell), round))| match round {
            None => None,
            Some(None) => Some(CellEnclosure::Flat),
            Some(Some(e)) => {
                let mut bodies = [0u32; 5];
                for (piece, s) in nucleus_of.scoped() {
                    if cell.bodies.contains(&piece) {
                        continue;
                    }
                    bodies[match cga::containment(s, e) {
                        Containment::Inside => 0,
                        Containment::On => 1,
                        Containment::Apart => 2,
                        Containment::Wraps => 3,
                        Containment::Unread => 4,
                    }] += 1;
                }
                Some(CellEnclosure::Round { bodies, within: within(i) })
            }
        })
        .collect()
}

/// COMPREHEND A FORM'S CELLS (CG1, MG1), under the stroke's budget.
pub(crate) fn comprehend(
    form: &Form,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
    budget: &mut crate::physics::Budget,
) -> Comprehension {
    let algebras = cell_algebras(form, traversed, budget);
    let n = form.n;
    let mirrors: Vec<Vec<Vec<MirrorReading>>> = form
        .cells
        .scoped()
        .map(|(_, c)| c.nuclei.iter().flatten().map(|s| cga::mirrors_between(s, n)).collect())
        .collect();
    // The form's mirrors: every distinct one bracketing a nucleus, keyed by
    // its own coordinates (axis, offset).
    let mut planes: Keyed<Mv> = Keyed::new();
    for m in mirrors.iter().flatten().flatten() {
        let mut key = vec![m.axis as u8];
        crate::transducer::address::put_bytes(&mut key, &m.k.numer().to_signed_bytes_le());
        crate::transducer::address::put_bytes(&mut key, &m.k.denom().to_signed_bytes_le());
        if !planes.contains(&key) {
            planes.insert(&key, cga::base_mirror(m.axis, &m.k));
        }
    }
    let plane_list: Vec<Mv> = planes.scoped().map(|(_, p)| p.clone()).collect();
    let mirror_group = match plane_list.is_empty() {
        true => None,
        false => Some(crate::transducer::cech::read_coxeter(&plane_list, &[], budget)),
    };
    let enclosures = read_enclosures(form);
    let bracket = read_bracket(form, traversed, &algebras, &enclosures);
    let constructors = form.cells.scoped().map(|(_, c)| c.nuclei.as_ref().map(Vec::len)).collect();
    Comprehension { algebras, mirrors, planes: plane_list.len(), mirror_group, enclosures, bracket, constructors, n }
}

fn cell_closure(r: &CoxeterReading) -> CellClosure {
    match &r.closure {
        Closure::Finite { by_theorem: true, .. } => CellClosure::FiniteByTheorem,
        Closure::Finite { by_theorem: false, .. } => CellClosure::FiniteByCheck,
        Closure::Infinite(Refusal::Pair(..)) => CellClosure::InfiniteByPair,
        Closure::Infinite(Refusal::Affine(_)) => CellClosure::Affine,
        Closure::Infinite(Refusal::Hyperbolic(_)) => CellClosure::Hyperbolic,
        Closure::Infinite(Refusal::Product { .. }) => CellClosure::InfiniteByProduct,
        Closure::Unbounded { .. } => CellClosure::Unbounded,
        Closure::Unread => CellClosure::Unread,
    }
}

/// CO1 — THE TYPED READING THE OBSERVER SERVES: every cell as its algebra
/// read it, SY1's blade per product in its canonical coordinates.
pub(crate) fn cells_reading(c: &Comprehension) -> Cells {
    let ray = |m: &Mv| -> Vec<(u16, num_rational::BigRational)> {
        match m.canonical() {
            Some(r) => (0..r.slot_count())
                .filter(|&b| !num_traits::Zero::is_zero(r.coeff(b)))
                .map(|b| (b as u16, r.coeff(b).clone()))
                .collect(),
            None => Vec::new(),
        }
    };
    let cells = c
        .algebras
        .iter()
        .zip(&c.mirrors)
        .zip(&c.enclosures)
        .zip(&c.constructors)
        .map(|(((a, ms), enclosure), held)| CellEntry {
            constructors: held.map(|k| k as u32),
            refusals: a.refusals.clone(),
            enclosure: enclosure.clone(),
            grade: a.grade as u8,
            round: match a.congruence {
                Congruence::Congruent => CellRound::Kept,
                Congruence::HeldZ2 { .. } => CellRound::HeldZ2,
                Congruence::Incongruent { .. } => CellRound::Carried,
                Congruence::Unread { .. } => CellRound::Unread,
                Congruence::NoRound => CellRound::NoRound,
                Congruence::NoCircuit => CellRound::NoCircuit,
            },
            permutes: match a.permutes {
                Permutes::All => CellPermutes::All,
                Permutes::Moves(_) => CellPermutes::Moves,
                Permutes::Unread { .. } => CellPermutes::Unread,
                Permutes::NoConstructors => CellPermutes::NoConstructors,
                Permutes::NoCircuit => CellPermutes::NoCircuit,
            },
            coherence: match a.coherence {
                Coherence::Closes(_) => CellCoherence::Closes,
                Coherence::Held => CellCoherence::Held,
                Coherence::Open(_) => CellCoherence::Open,
                Coherence::Unread => CellCoherence::Unread,
            },
            capacity: a.capacity.map(|k| k as u32),
            orbit: match a.constructors {
                ConstructorOrbit::Closed { .. } => CellOrbit::Closed,
                ConstructorOrbit::Open { .. } => CellOrbit::Open,
                ConstructorOrbit::Unbounded { .. } => CellOrbit::Unbounded,
                ConstructorOrbit::Unread => CellOrbit::Unread,
            },
            signature: a.gram.as_ref().map(|g| ((g.signature.0 as u8, g.signature.1 as u8, g.signature.2 as u8), g.rank as u8)),
            pairs: a.gram.iter().flat_map(|g| g.pairs.iter().map(|(_, _, _, r)| *r)).collect(),
            coxeter: a.coxeter.as_ref().map(cell_closure),
            polars: a.polars.as_ref().map(cell_closure),
            dual: a.dual.as_ref().map(|d| match d {
                DualFrame::Frame(_) => CellDual::Frame,
                DualFrame::Redundant(_) => CellDual::Redundant,
                DualFrame::Degenerate => CellDual::Degenerate,
            }),
            products: a
                .products
                .iter()
                .map(|p| CellProduct {
                    order: p.order.iter().map(|&i| i as u8).collect(),
                    grades: p.grades.iter().map(|&g| g as u8).collect(),
                    det: p.det,
                    classed: crate::transducer::cech::product_class(p).is_some(),
                    seal: ray(&p.seal),
                })
                .collect(),
            classes: a.classes as u8,
            mirrors: ms
                .iter()
                .map(|n| n.iter().map(|m| (m.axis as u8, m.k.clone(), m.delta2.clone(), m.regime)).collect())
                .collect(),
        })
        .collect();
    Cells {
        mirror_planes: c.planes as u32,
        mirror_group: c.mirror_group.as_ref().map(cell_closure),
        cells,
        bracket: c.bracket,
        // 78.32(e): the unital associative algebra on the `n + 2` vectors of
        // `Cl(n+1, 1)` is at most the full matrix algebra, `(n+2)²`.
        cap: (u32::from(c.n) + 2).pow(2),
    }
}

/// PG1 — THE PERCEPT'S PROGRESS toward E1's done-when: the circuits that
/// cross a contact face, those whose whole transport reads and is not
/// trivial, those unread (OR3); and the certified cells by grade, off the
/// typed cells.
pub(crate) fn progress(form: &Form, traversed: &[crate::transducer::sentence::TraversedCircuit], cells: &Cells) -> Progress {
    use crate::network::nerve::coord::BivectorKind;
    let mut p = Progress { percepts: 1, ..Progress::default() };
    for w in traversed {
        let contact =
            w.steps.iter().any(|&(f, _)| form.faces.get(usize::from(f)).is_some_and(|x| !x.interface.is_empty()));
        if !contact {
            continue;
        }
        p.contact_cycles += 1;
        if let Some(closure) = w.coset {
            use crate::transducer::sentence::circuit::CosetClosure;
            match closure {
                CosetClosure::Closed => p.contact_cycles_coset[0] += 1,
                CosetClosure::ClosedModuloHeld => p.contact_cycles_coset[1] += 1,
                CosetClosure::Open => p.contact_cycles_coset[2] += 1,
                // The stroke spent forming its candidates: unread by name.
                CosetClosure::Unbounded => p.contact_cycles_unread += 1,
            }
            continue;
        }
        match w.composite.as_ref().map(|m| m.class_invariants()) {
            None => p.contact_cycles_unread += 1,
            Some(Some((_, _, BivectorKind::Trivial))) => {}
            Some(Some(_)) => p.contact_cycles_curved += 1,
            // A composite whose class does not read is unread, never flat.
            Some(None) => p.contact_cycles_unread += 1,
        }
        if w.composite.is_some() && w.gauge_static {
            p.contact_cycles_static += 1;
        }
    }
    for c in &cells.cells {
        let at = match p.grades.binary_search_by_key(&c.grade, |g| g.grade) {
            Ok(at) => at,
            Err(at) => {
                p.grades.insert(at, GradeProgress { grade: c.grade, ..GradeProgress::default() });
                at
            }
        };
        let g = &mut p.grades[at];
        g.cells += 1;
        match c.round {
            CellRound::Kept => g.round_kept += 1,
            CellRound::HeldZ2 => g.held_z2 += 1,
            CellRound::Carried => g.refused += 1,
            CellRound::Unread | CellRound::NoRound | CellRound::NoCircuit => {}
        }
        match c.coxeter {
            Some(CellClosure::FiniteByTheorem | CellClosure::FiniteByCheck) => g.finite += 1,
            Some(CellClosure::Affine) => g.affine += 1,
            Some(CellClosure::InfiniteByPair | CellClosure::Hyperbolic | CellClosure::InfiniteByProduct) => {
                g.infinite += 1
            }
            Some(CellClosure::Unbounded | CellClosure::Unread) | None => {}
        }
        if c.dual == Some(CellDual::Frame) {
            g.dual_frames += 1;
        }
    }
    p
}

/// THE PERCEPT'S READERS AFTER ITS FORM, each held typed for the observer
/// and none of it printed but the orbits' lines (RC2): each kept orbit
/// (OR1), each certified cell as its orbit algebra off the circuits this
/// percept traversed (CG1, CO1), the circuits step by step (PL1), each held
/// loop's faces bent per pair of planes per lane (VS1, VO1), and the
/// percept's progress (PG1). SP1 — each reader's spend is the stroke's own
/// count before and after it, never a clock (law 9); perception's spend and
/// the frames it left unreached are the caller's.
pub(crate) fn read_and_hold(form: &Form, budget: &mut crate::physics::Budget, perceived: u64, unreached: u64) {
    use crate::transducer::cga::Undecomposed;
    use crate::transducer::sentence::{ModesRead, OrbitLine};
    let mut spend = Spend { percepts: 1, ..Spend::default() };
    spend.reads[0] = perceived;
    spend.unbounded[0] = unreached;
    let at = budget.remaining();
    for orbit in crate::transducer::sentence::take_orbits(budget) {
        if let OrbitLine::Read(r) | OrbitLine::Static(r) = &orbit {
            if let ModesRead::Undecomposed(Undecomposed::Unbounded(_)) = r.modes {
                spend.unbounded[1] += 1;
            }
        }
        eprintln!("  {}", orbit.line());
    }
    spend.reads[1] = (at - budget.remaining()) as u64;
    let at = budget.remaining();
    let traversed = crate::transducer::sentence::traversed_circuits(budget);
    let cells = cells_reading(&comprehend(form, &traversed, budget));
    spend.reads[2] = (at - budget.remaining()) as u64;
    spend.unbounded[2] = cells_unbounded(&cells);
    let at = budget.remaining();
    // CT2 — where every reuse arm was empty, the constructed routes (79.3).
    crate::transducer::sentence::construct_routes(form, budget);
    spend.unbounded[3] = crate::transducer::sentence::finish_circuits(budget);
    spend.reads[3] = (at - budget.remaining()) as u64;
    let at = budget.remaining();
    let viscosity = crate::transducer::sentence::read_viscosity(form, budget);
    spend.reads[4] = (at - budget.remaining()) as u64;
    spend.unbounded[4] = viscosity.iter().map(|p| p.unread).sum();
    let progress = progress(form, &traversed, &cells);
    hold(cells, crate::transducer::sentence::viscosity_reading(&viscosity), spend, progress);
}

/// The readings a cell's comprehension left unbounded by a spent stroke: a
/// group whose closure the check did not finish, an orbit of constructors
/// not closed within the stroke.
pub(crate) fn cells_unbounded(cells: &Cells) -> u64 {
    cells
        .cells
        .iter()
        .map(|c| {
            u64::from(c.coxeter == Some(CellClosure::Unbounded))
                + u64::from(c.polars == Some(CellClosure::Unbounded))
                + u64::from(c.orbit == CellOrbit::Unbounded)
        })
        .sum::<u64>()
        + u64::from(cells.mirror_group == Some(CellClosure::Unbounded))
}

/// THE PERCEPT'S TYPED READINGS, held until the observer takes them: the
/// last percept's cells (each percept's replaces the last), and the
/// viscosity, spend and progress summed over every percept since the last
/// take (78.16, RC2 — the percept prints none of them).
#[derive(Default)]
struct Held {
    cells: Option<Cells>,
    viscosity: Option<Viscosity>,
    spend: Option<Spend>,
    progress: Option<Progress>,
}

thread_local! {
    static HELD: std::cell::RefCell<Held> = std::cell::RefCell::new(Held::default());
}

/// Hold one percept's readings for the observer.
pub(crate) fn hold(cells: Cells, viscosity: Viscosity, spend: Spend, progress: Progress) {
    HELD.with(|h| {
        let mut h = h.borrow_mut();
        h.cells = Some(cells);
        match h.viscosity.as_mut() {
            Some(v) => v.absorb(&viscosity),
            None => h.viscosity = Some(viscosity),
        }
        match h.spend.as_mut() {
            Some(s) => s.absorb(&spend),
            None => h.spend = Some(spend),
        }
        match h.progress.as_mut() {
            Some(p) => p.absorb(&progress),
            None => h.progress = Some(progress),
        }
    });
}

/// The last percept's cells, and stop holding them. `None` where no percept
/// read any since the last take — a reading not taken, never an empty one.
pub fn take_cells() -> Option<Cells> {
    HELD.with(|h| h.borrow_mut().cells.take())
}

/// VO1 — the viscosity read since the last take.
pub fn take_viscosity() -> Option<Viscosity> {
    HELD.with(|h| h.borrow_mut().viscosity.take())
}

/// SP1 — the stroke by reader since the last take.
pub fn take_spend() -> Option<Spend> {
    HELD.with(|h| h.borrow_mut().spend.take())
}

/// PG1 — the progress read since the last take.
pub fn take_progress() -> Option<Progress> {
    HELD.with(|h| h.borrow_mut().progress.take())
}

/// EVERY CERTIFIED CELL OF `form`, READ AS ITS ORBIT ALGEBRA, under the
/// stroke's budget, in the cells' own key order. The circuits are the ones
/// the percept's own traversal kept — `(faces crossed, composite)` — never
/// traversed again.
/// THE CIRCUITS CROSSING ONE CELL, by their place in the traversal, each with
/// its order of the cell's bodies (CX1): each body as the circuit's crossings
/// first reach it. A step crosses where the face it crosses joins two
/// different bodies of the cell, read off the face's oriented pair and the
/// step's way (BD2).
fn crossings(
    form: &Form,
    cell: &crate::transducer::sentence::FormCell,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
) -> Vec<(usize, Vec<usize>)> {
    let mut body_of: Keyed<usize> = Keyed::new();
    for (b, piece) in cell.bodies.iter().enumerate() {
        body_of.insert(piece, b);
    }
    let crossing = |face: u16, forward: bool| -> Option<(usize, usize)> {
        let (x, y) = form.face_bodies.get(&u32::from(face).to_be_bytes())?;
        let (a, b) = (*body_of.get(x)?, *body_of.get(y)?);
        match (a == b, forward) {
            (true, _) => None,
            (false, true) => Some((a, b)),
            (false, false) => Some((b, a)),
        }
    };
    let mut out = Vec::new();
    for (at, t) in traversed.iter().enumerate() {
        let mut order: Vec<usize> = Vec::new();
        for &(face, forward) in &t.steps {
            if let Some((from, to)) = crossing(face, forward) {
                for b in [from, to] {
                    if !order.contains(&b) {
                        order.push(b);
                    }
                }
            }
        }
        if !order.is_empty() {
            out.push((at, order));
        }
    }
    out
}

pub(crate) fn cell_algebras(
    form: &Form,
    traversed: &[crate::transducer::sentence::TraversedCircuit],
    budget: &mut crate::physics::Budget,
) -> Vec<CellAlgebra> {
    form.cells
        .scoped()
        .map(|(_, cell)| {
            let mut circuits: Vec<Result<Mv, crate::transducer::cech::CircuitUnread>> = Vec::new();
            let mut orders: Vec<Option<Vec<usize>>> = Vec::new();
            for (at, order) in crossings(form, cell, traversed) {
                let Some(crate::transducer::sentence::TraversedCircuit { composite, unread, .. }) = traversed.get(at)
                else {
                    continue;
                };
                // OR3's reason rides the circuit into the cell; a composite
                // missing with no reason recorded is the unnamed refusal, by
                // its own name.
                use crate::transducer::cech::CircuitUnread;
                circuits.push(match (composite, unread) {
                    (Some(h), _) => Ok(h.clone()),
                    (None, Some(why)) => Err(*why),
                    (None, None) => Err(CircuitUnread::Unnamed),
                });
                orders.push((order.len() == cell.bodies.len()).then_some(order));
            }
            crate::transducer::cech::read_cell(
                cell.grade(),
                cell.meet().as_ref(),
                cell.nuclei.as_deref(),
                &circuits,
                &orders,
                budget,
            )
        })
        .collect()
}

