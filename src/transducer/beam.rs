// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A9 — THE BEAM of candidate selections a family's evidence admits, and
//! the exact invariant signature each carries: components, volume, boundary
//! length, hue tally. Candidates are read against the witnessed footprint
//! per axis, in structural order and never summed; two selections of equal
//! size are not equal readings (law 5).

use crate::transducer::anchor::Selection;

/// The exact spectral signature of a selected region within a scene.
/// Vec-of-relations for the hue tally (Law II — never a keyed map).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    /// β₀ — connected components under edge (4-)adjacency.
    pub components: u64,
    /// |S| — the region's site count.
    pub sites: u64,
    /// The boundary cochain length: exposed edges (region↔outside).
    pub boundary: u64,
    /// The hue tally over the scene's painted sites inside the region,
    /// sorted.
    pub hues: Vec<(u8, u64)>,
}

pub(crate) fn find(parent: &mut [usize], i: usize) -> usize {
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
    let mut sites: Vec<(i64, i64)> = region.to_vec();
    sites.sort_unstable();
    sites.dedup();
    // The region's 1-skeleton, read off the ordered support: β₀ is its
    // component count and the boundary cochain is every site's side not
    // shared along an edge (4 sides per site, 2 closed per edge).
    let edges = super::sentence::incidence_of(&sites);
    let mut roots = super::sentence::components(sites.len(), &edges);
    roots.sort_unstable();
    roots.dedup();
    // Every site has four sides and every interior edge consumes two of
    // them. `incidence_of` emits at most one row-edge and one column-edge
    // per adjacent pair of an ordered support, so edges ≤ 2n − 2 and the
    // sides always outnumber them: the boundary is exact and total.
    let boundary = 4 * sites.len() as u64 - 2 * edges.len() as u64;
    // Hue tally of the scene's painted sites inside the region: one merge
    // walk over the two ordered supports, never a lookup per site.
    let mut ordered: Vec<(i64, i64, u8)> = scene.to_vec();
    ordered.sort_unstable();
    let mut hues: Vec<(u8, u64)> = Vec::new();
    let (mut i, mut j) = (0usize, 0usize);
    while i < sites.len() && j < ordered.len() {
        let (r, c, h) = ordered[j];
        match sites[i].cmp(&(r, c)) {
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
        sites: sites.len() as u64,
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
        a.sites.abs_diff(b.sites),
        a.boundary.abs_diff(b.boundary),
        palette,
    ]
}

/// THE BEAM'S MINIMA, HELD: every candidate no other candidate beats on all
/// four lanes at once.
///
/// Each candidate's future state — its selection on each witness scene — is
/// measured against that witness's footprint signature, and the WORST
/// witnessing pair governs (per-axis max, the joint criterion). What is done
/// with those four lanes afterwards is the whole question.
///
/// `collapse_beam` and `rank_beam` stood here. They sorted the governed
/// mismatch LEXICOGRAPHICALLY and returned the head, discarding the rest —
/// which orders a polytonic vector (law 5: the lanes are never ordered) and
/// destroys a vortex at the moment it forms. Two candidates that each beat
/// the other on a different lane are not better and worse; they are two
/// readings, and 12.6 holds both until a perception, a glue or a ruling
/// collapses the superposition. A lexicographic sort silently makes 𝒮 senior
/// to 𝒢 senior to 𝒽 and calls the result a winner.
///
/// So the reading is the ANTICHAIN: a candidate is out only when another
/// beats or equals it on every lane and strictly beats it somewhere — a
/// comparison each lane makes for itself, with no lane ranked against
/// another and nothing summed. One survivor is a discharge; several are a
/// vortex and are all returned. Empty input → empty (no candidate, no
/// invention).
pub fn beam_minima(
    candidates: &[Selection],
    witnesses: &[crate::transducer::anchor::Evidence],
) -> Vec<Selection> {
    if candidates.len() <= 1 || witnesses.is_empty() {
        return candidates.to_vec();
    }
    let fp_sigs: Vec<Signature> =
        witnesses.iter().map(|(fp, scene)| signature(fp, scene)).collect();
    let governed: Vec<[u64; 4]> = candidates
        .iter()
        .map(|t| {
            let mut g = [0u64; 4];
            for ((_, scene), fp_sig) in witnesses.iter().zip(&fp_sigs) {
                let sel_sig = signature(&t.eval(scene), scene);
                let m = mismatch(&sel_sig, fp_sig);
                // index: the four lanes
                for l in 0..4 {
                    g[l] = g[l].max(m[l]);
                }
            }
            g
        })
        .collect();
    // `a` beats `b` when no lane of `a` is worse and some lane is better.
    // Read lane by lane; neither vector is reduced to anything.
    let beats = |a: &[u64; 4], b: &[u64; 4]| {
        a.iter().zip(b).all(|(x, y)| x <= y) && a.iter().zip(b).any(|(x, y)| x < y)
    };
    candidates
        .iter()
        .enumerate()
        .filter(|(i, _)| !governed.iter().enumerate().any(|(j, g)| j != *i && beats(g, &governed[*i])))
        .map(|(_, t)| t.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One coherent domino vs. the same volume scattered: topology tells
    /// them apart (β₀, boundary), volume cannot.
    #[test]
    fn signature_separates_equal_volumes_by_topology() {
        let scene = vec![(0, 0, 1), (0, 1, 1), (5, 5, 1), (7, 7, 2)];
        let coherent = signature(&[(0, 0), (0, 1)], &scene);
        let scattered = signature(&[(0, 0), (5, 5)], &scene);
        assert_eq!(coherent.sites, scattered.sites, "equal volume");
        assert_eq!(coherent.components, 1);
        assert_eq!(scattered.components, 2);
        assert_eq!(coherent.boundary, 6);
        assert_eq!(scattered.boundary, 8);
        assert_eq!(mismatch(&coherent, &coherent), [0; 4], "self-resonance is exact zero");
        assert!(mismatch(&coherent, &scattered)[0] > 0, "β₀ axis names the difference");
    }

    /// A candidate beaten on every lane is out; arrival order decides
    /// nothing. One survivor is a discharge.
    #[test]
    fn beam_minima_drops_candidate_beaten_on_every_lane() {
        // Scene: a hue-1 domino and a lone hue-2 site.
        let scene = vec![(0, 0, 1), (0, 1, 1), (4, 4, 2)];
        let footprint = vec![(0i64, 0i64), (0, 1)];
        // Candidate A selects EVERYTHING painted (carries fp, but drags the
        // alien site); candidate B selects exactly the hue-1 support, so B
        // is no worse on any lane and better on some.
        let a = Selection::Hue(255);
        let b = Selection::Hue(1);
        assert_eq!(a.eval(&scene).len(), 3);
        assert_eq!(b.eval(&scene).len(), 2);
        let witnesses = vec![(footprint, scene)];
        // A is listed FIRST: arrival order would keep it.
        assert_eq!(
            beam_minima(&[a, b.clone()], &witnesses),
            vec![b.clone()],
            "the dominated candidate leaves and arrival order decides nothing"
        );
        assert_eq!(beam_minima(std::slice::from_ref(&b), &[]), vec![b]);
        assert_eq!(beam_minima(&[], &[]), Vec::new());
    }

    /// TWO INCOMPARABLE READINGS ARE HELD, not ranked. Each beats the other
    /// on a different lane, so neither is out — and the lexicographic sort
    /// this replaced would have returned one of them and dropped the other,
    /// which is the vortex destroyed at the moment it forms.
    #[test]
    fn beam_minima_holds_incomparable_candidates() {
        // Two scenes. On the first, hue 1 is the exact footprint and hue 2
        // misses; on the second the footprint is hue 2 and hue 1 misses. The
        // worst pair governs each candidate, so each carries a mismatch the
        // other does not, on different content.
        let s1 = vec![(0, 0, 1), (0, 1, 1), (4, 4, 2)];
        let s2 = vec![(0, 0, 2), (0, 1, 2), (7, 7, 1)];
        let witnesses = vec![
            (vec![(0i64, 0i64), (0, 1)], s1),
            (vec![(0i64, 0i64), (0, 1)], s2),
        ];
        let one = Selection::Hue(1);
        let two = Selection::Hue(2);
        let held = beam_minima(&[one.clone(), two.clone()], &witnesses);
        assert_eq!(
            held.len(),
            2,
            "neither reading beats the other on every lane, so both stand: {held:?}"
        );
        assert!(held.contains(&one) && held.contains(&two));
    }
}
