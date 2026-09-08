// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! SPECTRAL PROJECTION OF FUTURE STATES (DC3, operator-ruled; discipline
//! imported from ipsal-autic v0.2's exact-rational interference/Laplacian
//! machinery, written natively for the ℤ² substrate).
//!
//! The beam holds candidate bindings — a bundle of PROJECTED FUTURE STATES.
//! Collapsing that bundle by raw cardinality (tightest-first) is counting,
//! not comprehension: two selections of equal size can be topologically
//! alien (one coherent region vs. scattered shrapnel). Here every candidate
//! selection carries an EXACT per-axis invariant signature — β₀ (connected
//! components), volume, boundary-cochain length, and the hue tally — and
//! candidates are ranked by their per-axis mismatch against the witnessed
//! footprint, worst witnessing pair governing (the joint criterion). Zero
//! mismatch = perfect resonance. All integers, all exact, no scalar
//! collapse: the axes compare lexicographically in structural order
//! (topology before volume before boundary before palette), never summed.

use crate::transducer::anchor::Sel;

/// The exact spectral signature of a selected region within a scene.
/// Vec-of-relations for the hue tally (Law II — never a keyed map).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    /// β₀ — connected components under 1-cell (4-)adjacency.
    pub components: u64,
    /// |S| — the region's cell volume.
    pub cells: u64,
    /// The boundary cochain length: exposed 1-cell faces (region↔outside).
    pub boundary: u64,
    /// The hue tally over the scene's cells inside the region, sorted.
    pub hues: Vec<(u8, u64)>,
}

pub(crate) fn find(parent: &mut Vec<usize>, i: usize) -> usize {
    let mut r = i;
    while parent[r] != r {
        r = parent[r];
    }
    let mut c = i;
    while parent[c] != c {
        let next = parent[c];
        parent[c] = r;
        c = next;
    }
    r
}

/// Measure the signature of `region` as it stands in `scene`.
pub fn signature(region: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Signature {
    let mut cells: Vec<(i64, i64)> = region.to_vec();
    cells.sort_unstable();
    cells.dedup();
    // The region's 1-skeleton, read off the ordered support: β₀ is its
    // component count and the boundary cochain is every cell face not
    // shared along an edge (4 faces per cell, 2 closed per edge).
    let edges = super::sentence::incidence_of(&cells);
    let mut roots = super::sentence::components(cells.len(), &edges);
    roots.sort_unstable();
    roots.dedup();
    let boundary = (4 * cells.len() as u64).saturating_sub(2 * edges.len() as u64);
    // Hue tally of the scene's cells inside the region: one merge walk over
    // the two ordered supports, never a lookup per cell.
    let mut ordered: Vec<(i64, i64, u8)> = scene.to_vec();
    ordered.sort_unstable();
    let mut hues: Vec<(u8, u64)> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < cells.len() && j < ordered.len() {
        let (r, c, h) = ordered[j];
        match cells[i].cmp(&(r, c)) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                match hues.iter_mut().find(|(hh, _)| *hh == h) {
                    Some((_, n)) => *n += 1,
                    None => hues.push((h, 1)),
                }
                j += 1;
            }
        }
    }
    hues.sort_unstable();
    Signature {
        components: roots.len() as u64,
        cells: cells.len() as u64,
        boundary,
        hues,
    }
}

/// The per-axis mismatch between two signatures — exact, never summed.
/// Axes in structural order: topology (β₀), volume, boundary, palette
/// (symmetric-difference mass of the hue tallies).
pub fn mismatch(a: &Signature, b: &Signature) -> [u64; 4] {
    let palette: u64 = {
        let mut d = 0u64;
        for &(h, n) in &a.hues {
            let m = b.hues.iter().find(|(hh, _)| *hh == h).map_or(0, |(_, m)| *m);
            d += n.abs_diff(m);
        }
        for &(h, m) in &b.hues {
            if !a.hues.iter().any(|(hh, _)| *hh == h) {
                d += m;
            }
        }
        d
    };
    [
        a.components.abs_diff(b.components),
        a.cells.abs_diff(b.cells),
        a.boundary.abs_diff(b.boundary),
        palette,
    ]
}

/// COLLAPSE THE BEAM SPECTRALLY: among candidate terms (all already carrying
/// the footprint on every witnessing pair), each candidate's future state —
/// its selection on each witness scene — is measured against that witness's
/// footprint signature; the WORST pair governs (per-axis max — the joint
/// criterion), and candidates rank lexicographically on the governed
/// mismatch. The minimum is the collapse; ties keep the incoming
/// (tightest-first) order, so the prior discipline is the tiebreak, not the
/// judge. Empty input → None (no candidate, no invention).
pub fn collapse_beam(
    candidates: &[Sel],
    witnesses: &[crate::transducer::anchor::Evidence],
) -> Option<Sel> {
    rank_beam(candidates, witnesses).into_iter().next()
}

/// The full spectral ordering of the beam: every candidate measured by its
/// governed per-axis mismatch (worst witnessing pair, lexicographic in
/// structural order), best first, stable on ties (the incoming
/// tightest-first order is the tiebreak). The head is the collapse; the
/// tail is the retained optionality a coherent multi-reading projection
/// draws from.
pub fn rank_beam(
    candidates: &[Sel],
    witnesses: &[crate::transducer::anchor::Evidence],
) -> Vec<Sel> {
    if candidates.len() <= 1 || witnesses.is_empty() {
        return candidates.to_vec();
    }
    let fp_sigs: Vec<Signature> =
        witnesses.iter().map(|(fp, scene)| signature(fp, scene)).collect();
    let mut measured: Vec<([u64; 4], usize)> = candidates
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let mut governed = [0u64; 4];
            for ((_, scene), fp_sig) in witnesses.iter().zip(&fp_sigs) {
                let sel_sig = signature(&t.eval(scene), scene);
                let m = mismatch(&sel_sig, fp_sig);
                for l in 0..4 {
                    governed[l] = governed[l].max(m[l]);
                }
            }
            (governed, i)
        })
        .collect();
    measured.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    measured.into_iter().map(|(_, i)| candidates[i].clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One coherent domino vs. the same volume scattered: topology tells
    /// them apart (β₀, boundary), volume cannot.
    #[test]
    fn topology_distinguishes_what_cardinality_cannot() {
        let scene = vec![(0, 0, 1), (0, 1, 1), (5, 5, 1), (7, 7, 2)];
        let coherent = signature(&[(0, 0), (0, 1)], &scene);
        let scattered = signature(&[(0, 0), (5, 5)], &scene);
        assert_eq!(coherent.cells, scattered.cells, "equal volume");
        assert_eq!(coherent.components, 1);
        assert_eq!(scattered.components, 2);
        assert_eq!(coherent.boundary, 6);
        assert_eq!(scattered.boundary, 8);
        assert_eq!(mismatch(&coherent, &coherent), [0; 4], "self-resonance is exact zero");
        assert!(mismatch(&coherent, &scattered)[0] > 0, "β₀ axis names the difference");
    }

    /// The spectral collapse picks the candidate whose future state RESONATES
    /// with the footprint — not the one that merely arrived first.
    #[test]
    fn the_beam_collapses_to_the_resonant_future_state() {
        // Scene: a hue-1 domino and a lone hue-2 cell.
        let scene = vec![(0, 0, 1), (0, 1, 1), (4, 4, 2)];
        let footprint = vec![(0i64, 0i64), (0, 1)];
        // Candidate A selects EVERYTHING occupied (carries fp, but drags the
        // alien cell); candidate B selects exactly the hue-1 support.
        let a = Sel::Hue(255);
        let b = Sel::Hue(1);
        assert_eq!(a.eval(&scene).len(), 3);
        assert_eq!(b.eval(&scene).len(), 2);
        let witnesses = vec![(footprint, scene)];
        // A listed FIRST: cardinality order would keep it; the spectrum
        // overrules — B's future state is the exact resonance.
        let chosen = collapse_beam(&[a, b.clone()], &witnesses).expect("a candidate survives");
        assert_eq!(chosen, b, "resonance beats arrival order");
        // A single candidate collapses trivially; an empty beam is honest None.
        assert_eq!(collapse_beam(std::slice::from_ref(&b), &[]), Some(b));
        assert_eq!(collapse_beam(&[], &[]), None);
    }
}
