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

/// Perceive a scene: its bodies (connected components of the non-background
/// cells; hue 0 is the background the eye does not see).
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

pub(crate) fn spelling_of_versor(v: &Versor) -> Option<crate::transducer::alphabet::Spelling> {
    let qn = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let at = |r, c| {
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&qn(r), &qn(c))))
    };
    let o = at(0, 0)?;
    let er = at(1, 0)?;
    let ec = at(0, 1)?;
    let block = [
        [&er.0 - &o.0, &ec.0 - &o.0],
        [&er.1 - &o.1, &ec.1 - &o.1],
    ];
    crate::transducer::alphabet::spell(&block)
}

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

/// Carriage of two bodies spelled as a word form. Class is the target's chart.
pub fn word_form(a: &Body, b: &Body) -> Option<Vertex> {
    let car = carriage(a, b)?;
    let sp = spelling_of_versor(&car.v)?;
    let t = element::translator_of(&car.v)?;
    let d = element::dilator_of(&car.v)?;
    let carriage = d.geo(&t);
    let chain = chain_of(Some(a), Some(b), &car.v);
    let gauge = if car.sigma.is_empty() {
        hue_map_of_chain(&chain)
    } else {
        car.sigma
    };
    Some(Vertex::Word {
        address: element::from_residual(&chain).address(),
        word: sp.word,
        carriage: Box::new(carriage),
        gauge,
        class: b.moduli.clone(),
        contact: None,
    })
}

pub(crate) fn body_only_vertex(body: &Body, created: bool) -> Vertex {
    let chain: Vec<(i64, i64, u8, u8)> = body
        .cells
        .iter()
        .map(|&(r, c, h)| if created { (r, c, 0, h) } else { (r, c, h, 0) })
        .collect();
    Vertex::Word {
        address: element::from_residual(&chain).address(),
        word: crate::transducer::alphabet::Word::identity(),
        carriage: Box::new(element::identity_versor()),
        gauge: hue_map_of_chain(&chain),
        class: body.moduli.clone(),
        contact: None,
    }
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

pub(crate) fn cells_subset_of_grid(cells: &[Cell3], g: &Grid) -> bool {
    cells.iter().all(|&(r, c, h)| {
        g.get(r as usize)
            .and_then(|row| row.get(c as usize))
            .copied()
            == Some(h)
    })
}
