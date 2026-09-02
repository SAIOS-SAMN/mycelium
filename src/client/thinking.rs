// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ENTITY'S THINKING — coordinate-free, in the algebra.
//!
//! A chain arriving from the sensorium or the book is not searched with
//! placements; it is LOCATED. Its support lifts to a shape; the shape's
//! similarity chart names the stratum of the genome's atlas it can sit on
//! (a precision call on the carrier's own coordinate); for each word in
//! that stratum the versor carrying the word onto the piece is solved in
//! closed form and verified by the algebraic replay; the gauge coordinate
//! is the word's assignment that closes the replay. The whole chain is
//! located first (a multi-component word), then each connected component.
//! What no word speaks is the residual — the entity's irreducible claim,
//! which germinates on the wire.
//!
//! ABDUCTION keeps the superposition: every verified word for a piece is a
//! hypothesis; the branches differ by which word speaks which piece, and
//! all share the residual the language cannot yet reach. Nothing here
//! iterates the genome, tests translations, or rounds a coordinate.

use crate::cognition::{replay, Cite};
use crate::transducer::element::{self, Element};
use crate::transducer::stalk::{self, Stalks};

type Cell = (i64, i64, u8, u8);

/// The connected components of a chain's support under 4-adjacency, each
/// as a sorted chain. The sensor's segmentation, not a search.
pub fn pieces(chain: &[Cell]) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Cell> = chain.to_vec();
    cells.sort_unstable();
    cells.dedup();
    let at = |r: i64, c: i64| cells.binary_search_by(|x| (x.0, x.1).cmp(&(r, c))).ok();
    let mut parent: Vec<usize> = (0..cells.len()).collect();
    for (i, &(r, c, _, _)) in cells.iter().enumerate() {
        for (dr, dc) in [(0i64, 1i64), (1, 0)] {
            if let Some(j) = at(r + dr, c + dc) {
                let (ri, rj) = (
                    crate::transducer::spectral::find(&mut parent, i),
                    crate::transducer::spectral::find(&mut parent, j),
                );
                if ri != rj {
                    parent[ri] = rj;
                }
            }
        }
    }
    let mut out: Vec<(usize, Vec<Cell>)> = Vec::new();
    for i in 0..cells.len() {
        let root = crate::transducer::spectral::find(&mut parent, i);
        match out.iter_mut().find(|(r, _)| *r == root) {
            Some((_, piece)) => piece.push(cells[i]),
            None => out.push((root, vec![cells[i]])),
        }
    }
    out.into_iter().map(|(_, p)| p).collect()
}

/// THE BASE POINT of a piece: every word in the piece's stratum that a
/// rational versor carries onto it, with the gauge coordinate under which
/// the replay closes — in stratum order. Empty when the language has no
/// word for this geometry.
pub fn base_points<'a>(
    genome: &[Element],
    atlas: &Stalks,
    shape_at: &impl Fn(usize) -> Option<&'a crate::transducer::blade::Shape>,
    piece: &[Cell],
    exclude: Option<usize>,
) -> Vec<Cite> {
    let chart = stalk::invariant(piece);
    let Some(shape) = element::shape_of_chain(piece) else { return Vec::new() };
    atlas
        .fiber(&chart)
        .iter()
        .copied()
        .filter(|&i| Some(i) != exclude)
        .filter_map(|i| {
            let word = genome.get(i)?;
            let to_word = crate::transducer::blade::versor_between(&shape, shape_at(i)?)?;
            let v = to_word.versor_inverse()?; // the word carried onto the piece
            (0..word.assignments().len()).find_map(|asg| {
                let cite: Cite = (i, v.clone(), asg);
                replay(genome, std::slice::from_ref(&cite), piece).is_empty().then_some(cite)
            })
        })
        .collect()
}

/// CITE a chain over the genome: the whole chain located first, then each
/// component; the residual is what no word speaks.
pub fn cite<'a>(
    genome: &[Element],
    atlas: &Stalks,
    shape_at: &impl Fn(usize) -> Option<&'a crate::transducer::blade::Shape>,
    chain: &[Cell],
    exclude: Option<usize>,
) -> (Vec<Cite>, Vec<Cell>) {
    if chain.is_empty() {
        return (Vec::new(), Vec::new());
    }
    if let Some(c) = base_points(genome, atlas, shape_at, chain, exclude).into_iter().next() {
        return (vec![c], Vec::new());
    }
    let mut cites = Vec::new();
    let mut residual: Vec<Cell> = Vec::new();
    for piece in pieces(chain) {
        match base_points(genome, atlas, shape_at, &piece, exclude).into_iter().next() {
            Some(c) => cites.push(c),
            None => residual.extend(piece),
        }
    }
    residual.sort_unstable();
    (cites, residual)
}

/// ABDUCE: the superposition of hypotheses for a chain, at most `width`
/// alternatives per piece (the width is the thinker's reserve on the
/// lane, never a constant here). The head is the located sentence; the
/// others differ by one piece's word. All share the residual.
pub fn abduce<'a>(
    genome: &[Element],
    atlas: &Stalks,
    shape_at: &impl Fn(usize) -> Option<&'a crate::transducer::blade::Shape>,
    chain: &[Cell],
    width: usize,
) -> Vec<(Vec<Cite>, Vec<Cell>)> {
    let width = width.max(1);
    if chain.is_empty() {
        return vec![(Vec::new(), Vec::new())];
    }
    let whole = base_points(genome, atlas, shape_at, chain, None);
    if !whole.is_empty() {
        return whole.into_iter().take(width).map(|c| (vec![c], Vec::new())).collect();
    }
    let mut alternatives: Vec<Vec<Cite>> = Vec::new();
    let mut residual: Vec<Cell> = Vec::new();
    for piece in pieces(chain) {
        let alts = base_points(genome, atlas, shape_at, &piece, None);
        match alts.is_empty() {
            true => residual.extend(piece),
            false => alternatives.push(alts.into_iter().take(width).collect()),
        }
    }
    residual.sort_unstable();
    let head: Vec<Cite> = alternatives.iter().map(|a| a[0].clone()).collect();
    let mut branches = vec![(head.clone(), residual.clone())];
    for (p, alts) in alternatives.iter().enumerate() {
        for alt in alts.iter().skip(1) {
            let mut branch = head.clone();
            branch[p] = alt.clone();
            branches.push((branch, residual.clone()));
        }
    }
    branches
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transducer::element::from_residual;
    use crate::transducer::germinate::germinate;

    /// A genome of a bar and a dot locates a rotated, translated, and
    /// dilated bar through its chart with a verified versor, cites a chain
    /// of two pieces as two words, leaves the unknown piece as the
    /// residual, and abduces alternatives when two words share a geometry.
    #[test]
    fn the_entity_locates_words_through_the_chart_and_the_versor() {
        let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
        let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
        let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
        let genome = vec![bar, dot, bar_other];
        let atlas = Stalks::over(genome.iter().map(|w| w.sub()));
        let shapes: Vec<Option<crate::transducer::blade::Shape>> =
            genome.iter().map(|w| element::shape_of_chain(w.sub())).collect();
        let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
        // A vertical bar far away: the bar rotated and translated.
        let target = vec![(5, 5, 1, 2), (6, 5, 1, 2)];
        let (cites, residual) = cite(&genome, &atlas, &shape_at, &target, None);
        assert_eq!(cites.len(), 1);
        assert_eq!(cites[0].0, 0, "the bar, not the other-hued bar");
        assert!(residual.is_empty());
        assert!(replay(&genome, &cites, &target).is_empty(), "the citation replays to closure");
        // A 2× dilated bar with the other hues: the other bar at ρ = 2.
        let big = vec![(0, 0, 7, 8), (0, 1, 7, 8), (0, 2, 7, 8), (0, 3, 7, 8), (1, 0, 7, 8), (1, 1, 7, 8), (1, 2, 7, 8), (1, 3, 7, 8)];
        let (cites, residual) = cite(&genome, &atlas, &shape_at, &big, None);
        assert_eq!(cites.len(), 1);
        assert_eq!(cites[0].0, 2);
        assert!(residual.is_empty());
        // Two pieces and an unknown: two citations, one residual piece.
        let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
        let (cites, residual) = cite(&genome, &atlas, &shape_at, &mixed, None);
        assert_eq!(cites.len(), 2);
        assert_eq!(residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
        // Excluding the bar itself: a bar chain has no other word of hue 1→2.
        let (cites, residual) = cite(&genome, &atlas, &shape_at, genome[0].sub(), Some(0));
        assert!(cites.is_empty() && !residual.is_empty(), "no word but itself speaks it");
        // Abduction: nothing alternative for a bar of hue 1→2 (the other bar
        // has other hues), so one branch; width bounds alternatives.
        let branches = abduce(&genome, &atlas, &shape_at, &target, 4);
        assert_eq!(branches.len(), 1);
    }
}
