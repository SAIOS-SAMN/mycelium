// 20, 51.3 — PERCEPTION: a grid becomes bodies, and two bodies become a
// vertex.
//
//! A body is a connected component of one hue on the three coordinates of
//! the base. `word_or_demand` reads the versor that carries one body onto
//! another and the chain of hue transitions it leaves; `cell_reads` counts
//! what the sensor was asked for.

use super::*;

pub type Grid = Vec<Vec<u8>>;

/// A cell of one frame's grid, the sensor's planar token: `(row, col, hue)`.
pub(crate) type Cell3 = (i64, i64, u8);

/// A cell of a body's support, on the three coordinates of the base (51.3):
/// `(row, col, frame, hue)`.
pub type Cell4 = crate::transducer::blade::Painted4;

/// A cell scene as a grid, origin at the scene's min-corner. The sensor
/// embedding of one scene; not a ranking over candidates.
pub fn cells_to_grid(cells: &[(i64, i64, u8)]) -> Grid {
    // The head only SEEDS the bounding box; the box is folded over every
    // cell below, so no member is chosen here.
    let [(r0, c0, _), ..] = cells else {
        return vec![vec![0]];
    };
    let (r0, c0) = (*r0, *c0);
    let (mut min_r, mut min_c, mut max_r, mut max_c) = (r0, c0, r0, c0);
    for &(r, c, _) in cells {
        if r < min_r {
            min_r = r;
        }
        if c < min_c {
            min_c = c;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
    }
    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;
    let mut g = vec![vec![0u8; w]; h];
    for &(r, c, hue) in cells {
        g[(r - min_r) as usize][(c - min_c) as usize] = hue;
    }
    g
}

/// THE SCENE A CHAIN DESCRIBES, as the two grids of one frame (50.6 seam
/// 1): the cells before and the cells after, both at the chain's own
/// min-corner, so the motion between them is the chain's and never an
/// artefact of two origins. `None` for an empty chain, which describes
/// no scene.
pub fn scene_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<(Grid, Grid)> {
    scene_at_of_chain(chain).map(|(_, before, after)| (before, after))
}

/// The same scene, with THE CORNER IT WAS FRAMED AT. A caller that has to
/// put a spoken cell back where the chain had it needs the origin the
/// framing subtracted, and re-deriving that corner beside this reader is
/// the second carrier for one fact (law 12) — one of them would be the
/// one that drifts.
pub fn scene_at_of_chain(chain: &[(i64, i64, u8, u8)]) -> Option<((i64, i64), Grid, Grid)> {
    // As above: the head seeds the box the fold below measures.
    let [(r0, c0, _, _), ..] = chain else { return None };
    let (r0, c0) = (*r0, *c0);
    let (mut min_r, mut min_c, mut max_r, mut max_c) = (r0, c0, r0, c0);
    for &(r, c, _, _) in chain {
        if r < min_r {
            min_r = r;
        }
        if c < min_c {
            min_c = c;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
    }
    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;
    let mut before = vec![vec![0u8; w]; h];
    let mut after = vec![vec![0u8; w]; h];
    for &(r, c, b, a) in chain {
        before[(r - min_r) as usize][(c - min_c) as usize] = b;
        after[(r - min_r) as usize][(c - min_c) as usize] = a;
    }
    Some(((min_r, min_c), before, after))
}

#[cfg(test)]
pub(crate) fn q(v: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
}

/// A perceived body: its support with hue on the three coordinates of the
/// base — ONE carrier (51.3) — its shape, and its moduli.
#[derive(Clone, Debug)]
pub struct Body {
    pub cells: Vec<Cell4>,
    pub shape: Shape,
    pub moduli: Invariant,
}

/// The body of one connected component of one hue: its shape lifted from
/// its support, its moduli read off the shape, its cells sorted.
pub fn body_of(comp: Vec<Cell4>) -> Body {
    let mut pos: Vec<(i64, i64, i64)> = comp.iter().map(|&(r, c, k, _)| (r, c, k)).collect();
    pos.sort_unstable();
    pos.dedup();
    let shape = blade::shape_of(&pos);
    let moduli = Invariant(blade::moduli(&shape.blade));
    let mut cells = comp;
    cells.sort_unstable();
    cells.dedup();
    Body { cells, shape, moduli }
}

thread_local! {
    static CELL_READS: Cell<u64> = const { Cell::new(0) };
}

/// Grid cells visited by [`perceive`] since the last reset.
#[cfg(test)]
pub fn cell_reads() -> u64 {
    CELL_READS.with(|c| c.get())
}

#[cfg(test)]
pub(crate) fn reset_cell_reads() {
    CELL_READS.with(|c| c.set(0));
}

pub(crate) fn note_cell_read() {
    CELL_READS.with(|c| c.set(c.get().saturating_add(1)));
}

/// Perceive one scene: its bodies, each a 4-connected component of cells of
/// ONE hue (chapter 20, ruled 2026-09-09) standing in frame `0`; hue 0 is
/// the background the eye does not see. Two touching bodies of different
/// hues are two bodies sharing a boundary, and the pair form carries that
/// boundary as a contact face with the edge transport of 50.3, so sliding,
/// relative rotation and the evolution of a colour boundary are measured
/// on the edge. The court reads a two-hue percept as one sentence with two
/// body vertices and their edge, never as two detached listings (50.1).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    perceive_frames(std::slice::from_ref(grid))
}

/// Perceive a family's frames as bodies on `(r, c, k)`: 6-connected
/// components of one hue across the frames (51.3, 51.4), the frame of each
/// grid its place in the family.
pub fn perceive_frames(grids: &[Grid]) -> Vec<Body> {
    let mut cells: Vec<Cell4> = Vec::new();
    for (k, grid) in grids.iter().enumerate() {
        for (r, row) in grid.iter().enumerate() {
            for (c, &h) in row.iter().enumerate() {
                note_cell_read();
                if h != 0 {
                    cells.push((r as i64, c as i64, k as i64, h));
                }
            }
        }
    }
    blade::components(&cells).into_iter().map(body_of).collect()
}

// `linear_block_of_versor` was here, renamed to `sensor_plane_block` and
// then moved to `element::letters` beside the three other readers of a
// versor, which are only correct together (§1.6: moved, not rewritten).

// THE DERIVATION READERS were here and are now `family/derivation.rs`:
// `factor_versor`, `occupied_lanes`, `adjoined_basis`, `peel_versor`,
// `peel_versor_with` and `derivation_of`. They are what a MIND makes of
// a carriage this file measured, they are only correct together, and L12
// gave them a fifth reader — `occupied_lanes` — which put this file over
// the §1.1 reading bound. Moved, not rewritten (§1.6).

/// THE CHAIN OF ONE SECTION between two bodies: each cell of the source
/// carried by `v` and read on the image, with the hue it left and the hue
/// it landed on. The chain is the sensor's planar token (16.5); the frame
/// rides on the carriage. A body across frames puts two cells on one site
/// of the section, which A5 refuses downstream — its chain is its sections,
/// one per frame (51.4).
pub(crate) fn chain_of(a: Option<&Body>, b: Option<&Body>, v: &Versor) -> Vec<(i64, i64, u8, u8)> {
    let mut chain = Vec::new();
    match (a, b) {
        (Some(src), Some(dst)) => {
            for &(r, c, k, h) in &src.cells {
                let h2 = element::carry_center(v, r, c, k)
                    .and_then(|p| {
                        let (tr, tc, tk) = element::cell_of(&p)?;
                        dst.cells
                            .binary_search_by(|x| (x.0, x.1, x.2).cmp(&(tr, tc, tk)))
                            .ok()
                            .map(|i| dst.cells[i].3)
                    })
                    // A cell the carriage lands nowhere in the image keeps
                    // its own hue: nothing moved it.
                    .map_or(h, |found| found);
                chain.push((r, c, h, h2));
            }
        }
        (None, Some(dst)) => {
            for &(r, c, _, h) in &dst.cells {
                chain.push((r, c, 0, h));
            }
        }
        (Some(src), None) => {
            for &(r, c, _, h) in &src.cells {
                chain.push((r, c, h, 0));
            }
        }
        (None, None) => {}
    }
    chain.sort_unstable();
    chain
}

pub(crate) fn hue_map_of_chain(chain: &[(i64, i64, u8, u8)]) -> Vec<(u8, u8)> {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &(_, _, b, a) in chain {
        seen.insert(crate::transducer::element::hue_pair_key(b, a), ());
    }
    let mut m: Vec<(u8, u8)> = seen
        .scoped()
        .filter_map(|(k, _)| crate::transducer::element::hue_pair_of(&k))
        .collect();
    m.sort_unstable();
    m
}

/// What a demand vertex stands for: its kind, the chart it is listed under
/// and the cells of the bodies it carries (50.5).
pub type Demanded = (crate::transducer::sentence::Demand, Invariant, Vec<crate::transducer::sentence::Cell>);

/// 50.5: a demand vertex over the bodies it stands for.
pub fn demand_vertex(kind: crate::transducer::sentence::Demand, chart: &Invariant, bodies: Vec<crate::transducer::sentence::Cell>) -> Vertex {
    Vertex::Demand { kind, chart: chart.clone(), bodies }
}

/// Carriage of two bodies spelled as a word form (50.2 F steps 1–2), or the
/// demand its rotor leaves: the versor measured, its rotor spelled by A22.
/// Class is the target's chart. `None` when no versor carries `a` onto `b`.
pub(crate) fn word_or_demand(a: &Body, b: &Body) -> Option<Result<Vertex, Demanded>> {
    let car = carriage(a, b)?;
    let chain = chain_of(Some(a), Some(b), &car.v);
    // A22 alone: this is the PERCEPT, which is what the memo holds. The
    // presentation enters afterwards, per mind (50.2 step 2).
    let derivation = match peel_versor(&car.v)? {
        Ok(w) => crate::transducer::alphabet::Derivation::spelled(w),
        Err(demand) => return Some(Err((demand, b.moduli.clone(), chain))),
    };
    Some(Ok(word_vertex_with(a, b, &car, derivation)?))
}

/// THE WORD VERTEX OF A MEASURED CARRIAGE, given the derivation that
/// reaches it — the placement and dilation residues off the versor, the
/// gauge off the carriage or the chain, the address off the chain.
///
/// Split out because the derivation has TWO sources and the rest of the
/// vertex has one. A22's peel is the percept's; the ring's is a mind's,
/// found at 50.2 step 2 against a presentation the percept cannot hold.
/// Both build the same vertex around it.
pub(crate) fn word_vertex_with(
    a: &Body,
    b: &Body,
    car: &Carriage,
    derivation: crate::transducer::alphabet::Derivation,
) -> Option<Vertex> {
    let chain = chain_of(Some(a), Some(b), &car.v);
    let t = element::translator_of(&car.v)?;
    let d = element::dilator_of(&car.v)?;
    let carriage = d.geo(&t);
    let gauge = if car.sigma.is_empty() {
        hue_map_of_chain(&chain)
    } else {
        car.sigma.clone()
    };
    Some(Vertex::word_on(
        element::from_residual(&chain)?.address(),
        derivation,
        b.moduli.clone(),
        Box::new(carriage),
        gauge,
        None,
    ))
}

// `word_form` was here: `word_or_demand(a, b)?.ok()`, with no caller in
// the tree. It is deleted rather than threaded — a wrapper that discards
// the demand and now would also have to be handed a presentation and a
// budget to discard them too.

// `supports_overlap(a, b)` was here. It built a fresh cell index of `a` and
// asked whether any cell of `b` was in it — a correct answer to "do these
// two bodies meet", asked inside two Cartesian products in `pair.rs`, so the
// index was rebuilt once per PAIR and the walk ran over the whole of the
// other scene. Its callers now read the cell fibre directly: one index of a
// scene's bodies by cell, built once, and a body's overlaps are the union of
// the fibres at its own cells — which costs that body's own support instead
// of the other scene's cardinality (rule 1). There is no third caller and no
// reason to keep a second way to ask.

/// The axes on which two charts differ, by position; an axis one chart
/// lacks differs.
pub(crate) fn differing_axes(a: &Invariant, b: &Invariant) -> Vec<u16> {
    let n = a.0.len().max(b.0.len());
    (0..n).filter(|&i| a.0.get(i) != b.0.get(i)).map(|i| i as u16).collect()
}

/// WHY A PAIR CARRIES NO EDGE (50.3, 13.9): no single versor moves the two
/// bodies rigidly; a versor with no rotor, translator or dilator reading —
/// the `Cl(3,1)` bridge (51.2); or a stabilizer element whose rotor the
/// seventeen do not spell — 37.7's candidate, carried as the demand the
/// peel typed (50.4) and not an edge invented for it (rule 11).
#[derive(Debug)]
pub(crate) enum EdgeRefusal {
    NotRigid,
    NoReading,
    Unspelled(crate::transducer::sentence::Demand),
}

/// THE EDGES A PAIR FORM ASKED FOR, BY WHAT CAME BACK (§8.1, measurement
/// only): measured, or refused for one of `EdgeRefusal`'s three reasons.
/// Read on the `pair form` line, so the corpus says whether any
/// stabilizer element on it is 37.7's candidate.
#[derive(Default)]
pub(crate) struct EdgeTally {
    pub(crate) measured: usize,
    pub(crate) unspelled: usize,
    pub(crate) unread: usize,
    pub(crate) not_rigid: usize,
}

impl EdgeTally {
    /// Count one asked edge and hand the caller what it carries.
    pub(crate) fn take(
        &mut self,
        asked: Option<Result<crate::transducer::sentence::Transport, EdgeRefusal>>,
    ) -> Option<crate::transducer::sentence::Transport> {
        match asked {
            Some(Ok(t)) => {
                self.measured += 1;
                Some(t)
            }
            // 37.7's candidate at an edge, said with the kind the peel
            // typed: the one line on this path that names a generator the
            // language lacks.
            Some(Err(EdgeRefusal::Unspelled(d))) => {
                self.unspelled += 1;
                eprintln!("  edge unspelled: {}", d.label());
                None
            }
            Some(Err(EdgeRefusal::NoReading)) => {
                self.unread += 1;
                None
            }
            Some(Err(EdgeRefusal::NotRigid)) => {
                self.not_rigid += 1;
                None
            }
            None => None,
        }
    }
}

/// THE EDGE A TRANSITION `Γ` IS (50.3): its rotor spelled by A22's peel as
/// the word, its placement and dilation as the carriage, and no gauge —
/// the caller reads the hue transition where it has one.
pub(crate) fn edge_of(gamma: &Versor) -> Result<crate::transducer::sentence::Transport, EdgeRefusal> {
    let word = match peel_versor(gamma) {
        None => return Err(EdgeRefusal::NoReading),
        Some(Err(d)) => return Err(EdgeRefusal::Unspelled(d)),
        Some(Ok(w)) => w,
    };
    let t = element::translator_of(gamma).ok_or(EdgeRefusal::NoReading)?;
    let d = element::dilator_of(gamma).ok_or(EdgeRefusal::NoReading)?;
    Ok(crate::transducer::sentence::Transport { word, carriage: d.geo(&t), gauge: Vec::new() })
}

/// THE EDGE TRANSPORT of 50.3 between two carried bodies. `V_ij` carries
/// the pair-body `(a_i ∪ a_j)` onto `(b_i ∪ b_j)`; it exists iff the pair
/// moved rigidly, and then `Γ_ij = V_i⁻¹ · V_ij` lies in the stabilizer of
/// `a_i` or the exchange of the two bodies. A refusal is a remainder edge,
/// and says which (`EdgeRefusal`).
pub(crate) fn edge_transport(
    v_i: &Versor,
    a_i: &Body,
    a_j: &Body,
    b_i: &Body,
    b_j: &Body,
) -> Result<crate::transducer::sentence::Transport, EdgeRefusal> {
    let union = |x: &Body, y: &Body| -> Shape {
        let mut pos: Vec<(i64, i64, i64)> =
            x.cells.iter().chain(y.cells.iter()).map(|&(r, c, k, _)| (r, c, k)).collect();
        pos.sort_unstable();
        pos.dedup();
        blade::shape_of(&pos)
    };
    let Some(v_ij) = blade::versor_between(&union(a_i, a_j), &union(b_i, b_j)) else {
        return Err(EdgeRefusal::NotRigid);
    };
    let gamma = v_i.versor_inverse().ok_or(EdgeRefusal::NoReading)?.geo(&v_ij);
    edge_of(&gamma)
}

/// A body's support on the three coordinates of the base, sorted.
pub(crate) fn support_of(body: &Body) -> Vec<(i64, i64, i64)> {
    let mut s: Vec<(i64, i64, i64)> = body.cells.iter().map(|&(r, c, k, _)| (r, c, k)).collect();
    s.sort_unstable();
    s.dedup();
    s
}

/// THE RING OF A SUPPORT: the cells one 4-step from it within each of its
/// frames — the adjacent ring of each lamina, lifted back to its frame.
/// Adjacency along `e₃` is the corpus's own (51.3) and is not a ring.
pub(crate) fn ring_of_support(support: &[(i64, i64, i64)]) -> Vec<(i64, i64, i64)> {
    let mut out = Vec::new();
    for (k, section) in blade::sections(support) {
        let cells: Vec<Cell3> = section.iter().map(|&(r, c)| (r, c, 0)).collect();
        out.extend(blade::adjacent_ring(&cells).into_iter().map(|(r, c, _)| (r, c, k)));
    }
    out.sort_unstable();
    out.dedup();
    out
}

pub(crate) fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), crate::arc::grid_width(g))
}

/// Does everything a form spoke lie inside the grid it was spoken against?
/// The test extraction's step 3 decides a glue by, and the same test a
/// voice answers a demand by (L8q), so the two organs read one thing.
pub fn cells_subset_of_grid(cells: &[Cell3], g: &Grid) -> bool {
    cells.iter().all(|&(r, c, h)| {
        g.get(r as usize)
            .and_then(|row| row.get(c as usize))
            .copied()
            == Some(h)
    })
}

/// The scene's bodies indexed by their own chart (Y4, 24.4, 20).
/// A chart whose fiber holds exactly one body is "the unique one", read as
/// `fiber.iter().nth(1).is_none()` (W10), without a count compared to a literal.
pub fn bodies_by_chart(bodies: &[Body]) -> crate::network::nerve::Keyed<Vec<&Body>> {
    let mut fibers: crate::network::nerve::Keyed<Vec<&Body>> = crate::network::nerve::Keyed::new();
    for b in bodies {
        let k = b.moduli.key();
        match fibers.get_mut(&k) {
            Some(v) => v.push(b),
            None => {
                fibers.insert(k, vec![b]);
            }
        }
    }
    fibers
}

/// The chart whose fiber in the scene holds exactly one body (Y4, 24.4, 21 step 1, W10).
/// Answers "the chart whose fiber holds one" without a count compared to a literal.
pub fn chart_holding_one(bodies: &[Body]) -> Option<crate::transducer::stalk::Invariant> {
    let fibers = bodies_by_chart(bodies);
    let mut singletons = fibers
        .scoped()
        .filter_map(|(_, fiber)| {
            let mut it = fiber.iter();
            let first = it.next()?;
            if it.next().is_none() {
                Some(first.moduli.clone())
            } else {
                None
            }
        });
    let unique = singletons.next()?;
    if singletons.next().is_none() {
        Some(unique)
    } else {
        None
    }
}
