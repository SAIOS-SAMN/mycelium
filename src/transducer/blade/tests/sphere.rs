//! 78.188 — The chart is read on Sⁿ. A body's Betti numbers, read on the
//! sensor's cover ([`blade::topology`]), close on `S³ = ℝ³ ∪ {∞}` by two
//! readings that take no boundary rank:
//!
//! - Euler–Poincaré: `β₀ − β₁ + β₂ = C₀ − C₁ + C₂ − C₃`, the cells of
//!   the cover counted by dimension.
//! - Alexander duality on S³: `H̃₂(X) ≅ H̃⁰(S³∖X)`: the cavities are the
//!   components of the complement other than the one holding ∞. A complement
//!   component is a class of empty cells joined across shared faces; the
//!   cells that reach the edge of a neighbourhood one cell wider than the
//!   body all lie in the one component holding ∞.
//!
//! The chart then holds only what Sⁿ keeps: `n`, the Betti numbers and the
//! hue spectrum, never a lattice ratio.
use crate::transducer::base::Site;
use crate::transducer::blade;

type At = [i64; 3];

fn sites(cells: &[At]) -> Vec<Site> {
    cells.iter().map(|&a| Site::of(a)).collect()
}

/// `C₀ − C₁ + C₂ − C₃` of the cover: every closed cell of every site's
/// cell, shared cells counted once.
fn euler_of_cells(cells: &[At]) -> i64 {
    let mut all: Vec<(u8, At)> = Vec::new();
    for c in cells {
        for dirs in 0u8..8 {
            let fixed: Vec<usize> = (0..3).filter(|i| dirs & (1 << i) == 0).collect();
            for eps in 0u8..(1 << fixed.len()) {
                let mut at = *c;
                for (b, &i) in fixed.iter().enumerate() {
                    if eps & (1 << b) != 0 {
                        at[i] += 1;
                    }
                }
                all.push((dirs, at));
            }
        }
    }
    all.sort_unstable();
    all.dedup();
    all.iter().map(|(dirs, _)| if dirs.count_ones() % 2 == 0 { 1 } else { -1 }).sum()
}

fn root(parent: &mut [usize], mut i: usize) -> usize {
    while parent[i] != i {
        parent[i] = parent[parent[i]];
        i = parent[i];
    }
    i
}

/// The components of `S³∖X` other than the one holding ∞.
fn cavities(cells: &[At]) -> usize {
    let mut held = cells.to_vec();
    held.sort_unstable();
    let lo: At = std::array::from_fn(|i| cells.iter().map(|c| c[i]).fold(i64::MAX, i64::min) - 1);
    let hi: At = std::array::from_fn(|i| cells.iter().map(|c| c[i]).fold(i64::MIN, i64::max) + 1);
    let mut empty: Vec<At> = Vec::new();
    for x in lo[0]..=hi[0] {
        for y in lo[1]..=hi[1] {
            for z in lo[2]..=hi[2] {
                if held.binary_search(&[x, y, z]).is_err() {
                    empty.push([x, y, z]);
                }
            }
        }
    }
    let mut parent: Vec<usize> = (0..empty.len()).collect();
    for (i, e) in empty.iter().enumerate() {
        for axis in 0..3 {
            let mut next = *e;
            next[axis] += 1;
            if let Ok(j) = empty.binary_search(&next) {
                let (a, b) = (root(&mut parent, i), root(&mut parent, j));
                parent[a] = b;
            }
        }
    }
    let toward_infinity = |e: &At| (0..3).any(|i| e[i] == lo[i] || e[i] == hi[i]);
    let mut outside: Vec<usize> = empty.iter().enumerate().filter(|(_, e)| toward_infinity(e)).map(|(i, _)| root(&mut parent, i)).collect();
    outside.sort_unstable();
    outside.dedup();
    assert_eq!(outside.len(), 1, "the neighbourhood of ∞ is one component");
    let mut all: Vec<usize> = (0..empty.len()).map(|i| root(&mut parent, i)).collect();
    all.sort_unstable();
    all.dedup();
    all.len() - 1
}

fn block(w: [i64; 3]) -> Vec<At> {
    (0..w[0]).flat_map(|x| (0..w[1]).flat_map(move |y| (0..w[2]).map(move |z| [x, y, z]))).collect()
}

fn closes(name: &str, cells: &[At], betti: [u64; 3]) {
    let read = blade::topology(&sites(cells)).expect("topology at n = 3");
    assert_eq!(read, betti.to_vec(), "{name}: the Betti numbers");
    let [b0, b1, b2] = betti.map(|b| i64::try_from(b).expect("small"));
    assert_eq!(b0 - b1 + b2, euler_of_cells(cells), "{name}: Euler–Poincaré");
    assert_eq!(usize::try_from(betti[2]).expect("small"), cavities(cells), "{name}: Alexander duality on S³");
}

/// The Betti numbers `topology` reads close on S³: for a solid, a hollow, a
/// ring, two handles, two hollows apart, an opened hollow, a core inside a
/// shell and two cells meeting on an edge or a corner, the alternating sum
/// is the Euler characteristic of the cover's cells, and β₂ is the count of
/// complement components other than the one holding ∞.
#[test]
fn topology_betti_numbers_close_on_s3() {
    closes("a solid", &block([2, 2, 2]), [1, 0, 0]);
    let hollow: Vec<At> = block([3, 3, 3]).into_iter().filter(|&c| c != [1, 1, 1]).collect();
    closes("a hollow", &hollow, [1, 0, 1]);
    let ring: Vec<At> = block([3, 3, 1]).into_iter().filter(|&c| c != [1, 1, 0]).collect();
    closes("a ring", &ring, [1, 1, 0]);
    let handles: Vec<At> = block([5, 3, 1]).into_iter().filter(|&c| c != [1, 1, 0] && c != [3, 1, 0]).collect();
    closes("two handles", &handles, [1, 2, 0]);
    let two: Vec<At> = hollow.iter().copied().chain(hollow.iter().map(|c| [c[0] + 5, c[1], c[2]])).collect();
    closes("two hollows apart", &two, [2, 0, 2]);
    let opened: Vec<At> = hollow.iter().copied().filter(|&c| c != [1, 1, 0]).collect();
    closes("a hollow opened on a face", &opened, [1, 0, 0]);
    let core_in_shell: Vec<At> = block([7, 7, 7])
        .into_iter()
        .filter(|c| !c.iter().all(|&v| (1..=5).contains(&v)) || c.iter().all(|&v| v == 3))
        .collect();
    closes("a core inside a shell", &core_in_shell, [2, 0, 1]);
    closes("two cells on one edge", &[[0, 0, 0], [1, 1, 0]], [1, 0, 0]);
    closes("two cells on one corner", &[[0, 0, 0], [1, 1, 1]], [1, 0, 0]);
}

/// A ring and the same ring stretched along one axis file on one chart, and
/// a disc does not: on Sⁿ nothing metric of one body is filed.
#[test]
fn moduli_holds_only_what_the_sphere_keeps() {
    let ring: Vec<At> = block([3, 3, 1]).into_iter().filter(|&c| c != [1, 1, 0]).collect();
    // The same ring, stretched to 5 × 3: a different lattice shape, the
    // same body on S³.
    let stretched: Vec<At> = block([5, 3, 1]).into_iter().filter(|&c| !(1..=3).contains(&c[0]) || c[1] != 1).collect();
    let chart = |cells: &[At]| blade::moduli(&blade::shape_of(&sites(cells)).blade).expect("a body");
    assert_eq!(chart(&ring), chart(&stretched), "a ring and a stretched ring share a chart on S³");
    let solid = block([3, 3, 1]);
    assert_ne!(chart(&ring), chart(&solid), "a ring and a disc do not");
}
