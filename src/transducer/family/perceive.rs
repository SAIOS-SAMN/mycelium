// 20 — PERCEPTION: a grid becomes bodies, and two bodies become a vertex.
//
//! A body is a connected component of one hue. `word_form` reads the versor
//! that carries one body onto another and the chain of hue transitions it
//! leaves; `cell_reads` counts what the sensor was asked for.

use super::*;

pub type Grid = Vec<Vec<u8>>;
pub(crate) type Cell3 = (i64, i64, u8);

/// A cell scene as a grid, origin at the scene's min-corner. The sensor
/// embedding of one scene; not a ranking over candidates.
pub fn cells_to_grid(cells: &[(i64, i64, u8)]) -> Grid {
    let Some(&(r0, c0, _)) = cells.first() else {
        return vec![vec![0]];
    };
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
    let &(r0, c0, _, _) = chain.first()?;
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
    Some((before, after))
}

#[cfg(test)]
pub(crate) fn q(v: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
}
pub(crate) fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// A perceived body: its cells with hue, its shape, and its moduli.
#[derive(Clone, Debug)]
pub struct Body {
    pub cells: Vec<Cell3>,
    pub shape: Shape,
    pub moduli: Invariant,
}

pub(crate) fn body_of(comp: Vec<Cell3>) -> Body {
    let mut pos: Vec<(i64, i64)> = comp.iter().map(|&(r, c, _)| (r, c)).collect();
    pos.sort_unstable();
    let shape = blade::shape_of(&pos);
    let moduli = Invariant(blade::moduli(&shape.blade));
    let mut cells = comp;
    cells.sort_unstable();
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

/// Perceive a scene: its bodies, each a 4-connected component of cells of
/// ONE hue (chapter 20, ruled 2026-09-09); hue 0 is the background the
/// eye does not see. Two touching bodies of different hues are two bodies
/// sharing a boundary, and the pair form carries that boundary as a
/// contact face with the edge transport of 50.3, so sliding, relative
/// rotation and the evolution of a colour boundary are measured on the
/// edge. The court reads a two-hue percept as one sentence with two body
/// vertices and their edge, never as two detached listings (50.1).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    let mut cells: Vec<Cell3> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            note_cell_read();
            if h != 0 {
                cells.push((r as i64, c as i64, h));
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


pub(crate) fn chain_of(a: Option<&Body>, b: Option<&Body>, v: &Versor) -> Vec<(i64, i64, u8, u8)> {
    let mut chain = Vec::new();
    match (a, b) {
        (Some(src), Some(dst)) => {
            for &(r, c, h) in &src.cells {
                let h2 = element::carry_center(v, r, c)
                    .and_then(|p| {
                        let (tr, tc) = element::cell_of(&p);
                        dst.cells
                            .binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc)))
                            .ok()
                            .map(|i| dst.cells[i].2)
                    })
                    .unwrap_or(h);
                chain.push((r, c, h, h2));
            }
        }
        (None, Some(dst)) => {
            for &(r, c, h) in &dst.cells {
                chain.push((r, c, 0, h));
            }
        }
        (Some(src), None) => {
            for &(r, c, h) in &src.cells {
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
        .fiber(crate::network::nerve::ROOT)
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
    Some(Vertex::Word {
        address: element::from_residual(&chain).address(),
        derivation,
        carriage: Box::new(carriage),
        gauge,
        class: b.moduli.clone(),
        contact: None,
    })
}

// `word_form` was here: `word_or_demand(a, b)?.ok()`, with no caller in
// the tree. It is deleted rather than threaded — a wrapper that discards
// the demand and now would also have to be handed a presentation and a
// budget to discard them too.

/// Whether two bodies' supports share a cell: the overlap the eyes admit
/// as a correspondence when no versor carries one onto the other.
pub(crate) fn supports_overlap(a: &Body, b: &Body) -> bool {
    let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &(r, c, _) in &a.cells {
        have.insert(cell_pos_key(r, c), ());
    }
    b.cells.iter().any(|&(r, c, _)| have.contains(&cell_pos_key(r, c)))
}

/// The axes on which two charts differ, by position; an axis one chart
/// lacks differs.
pub(crate) fn differing_axes(a: &Invariant, b: &Invariant) -> Vec<u16> {
    let n = a.0.len().max(b.0.len());
    (0..n).filter(|&i| a.0.get(i) != b.0.get(i)).map(|i| i as u16).collect()
}

/// THE EDGE TRANSPORT of 50.3 between two carried bodies. `V_ij` carries
/// the pair-body `(a_i ∪ a_j)` onto `(b_i ∪ b_j)`; it exists iff the pair
/// moved rigidly, and then `Γ_ij = V_i⁻¹ · V_ij` lies in the stabilizer of
/// `a_i` or the exchange of the two bodies. `None` is a remainder edge: no
/// single versor carries the pair, and the two bodies are two charts.
pub(crate) fn edge_transport(v_i: &Versor, a_i: &Body, a_j: &Body, b_i: &Body, b_j: &Body) -> Option<crate::transducer::sentence::Transport> {
    let union = |x: &Body, y: &Body| -> Shape {
        let mut pos: Vec<(i64, i64)> = x.cells.iter().chain(y.cells.iter()).map(|&(r, c, _)| (r, c)).collect();
        pos.sort_unstable();
        pos.dedup();
        blade::shape_of(&pos)
    };
    let v_ij = blade::versor_between(&union(a_i, a_j), &union(b_i, b_j))?;
    let gamma = v_i.versor_inverse()?.geo(&v_ij);
    let word = peel_versor(&gamma)?.ok()?;
    let t = element::translator_of(&gamma)?;
    let d = element::dilator_of(&gamma)?;
    Some(crate::transducer::sentence::Transport { word, carriage: d.geo(&t), gauge: Vec::new() })
}

pub(crate) fn support_of(body: &Body) -> Vec<(i64, i64)> {
    let mut s: Vec<(i64, i64)> = body.cells.iter().map(|&(r, c, _)| (r, c)).collect();
    s.sort_unstable();
    s.dedup();
    s
}

pub(crate) fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), g.first().map_or(0, |r| r.len()))
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
