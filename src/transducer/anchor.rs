// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE RECURSIVE SELECTION ALGEBRA (alpha ruling): selection is never a
//! predefined enum of spatial filters — it is a recursive composition of
//! 0-cell boundary markings. An anchored word is γ_substance ∘ π_anchor where
//! the projector π is a TERM in this algebra, derived bottom-up from the two
//! lattice invariants the sensor actually measures:
//!
//!   Level 0 — 0-cells (occupied lattice points) and 1-cell adjacency
//!             (‖x−y‖₁ = 1); nothing else enters.
//!   Level 1 — atomic supports and morphisms: K_h (hue support), the
//!             coboundary δK = {x ∈ K | ∃ y ∉ K adjacent}, the extremal
//!             projections onto the substrate axes.
//!   Level 2+ — algebraic composition: ∩, ∖, the star/dilation (1-cell
//!             neighbourhood), connected-component projection — from which
//!             interior (K ∖ δK), relative interface (K₁ ∩ star K₂), hulls
//!             and every deeper predicate DERIVE rather than being named.
//!
//! The constructors of `Sel` are the OPERATORS of this algebra (like compose/
//! bracket on Element) — not labels over semantic categories. A term is
//! re-evaluated exactly on any scene; deriving the minimal term whose
//! evaluation carries a word's domain is deduction over the algebra, verified
//! downstream by replay (a wrong term simply fails to re-derive its train
//! evidence). All carriers are Vec-of-relations (Law II); evaluation is exact.

use super::blade::components;

/// A term of the selection algebra. Constructors = operators, arguments =
/// measured coordinates (hue byte, substrate axis index, component ordinal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sel {
    /// K_h — the support of hue `h`; 255 is the whole occupied support.
    Hue(u8),
    /// The k-th connected component of the term's support (canonical order:
    /// by minimal cell) — connectivity read from 1-cells.
    Comp(Box<Sel>, u8),
    /// δ — the coboundary: cells of the support with a 1-cell neighbour
    /// outside it.
    Delta(Box<Sel>),
    /// The star (1-cell dilation): every cell adjacent to the support,
    /// support included.
    Star(Box<Sel>),
    /// Extremal projection onto a substrate axis: cells achieving the
    /// extremum. axis 0..4 = min-row, max-row, min-col, max-col.
    Ext(Box<Sel>, u8),
    /// ∩ — algebraic intersection of supports.
    Inter(Box<Sel>, Box<Sel>),
    /// ∖ — set difference of supports.
    Minus(Box<Sel>, Box<Sel>),
    /// Connected closure: the cells of the second support 1-cell-reachable
    /// from the first (seed ∩ within, grown through within) — pure Level-0
    /// connectivity, the operator that composes enclosure:
    /// enclosed(K) = K ∖ Flood(K ∩ δ(whole), K).
    Flood(Box<Sel>, Box<Sel>),
}

fn sorted(mut v: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    v.sort_unstable();
    v.dedup();
    v
}

fn has(s: &[(i64, i64)], p: (i64, i64)) -> bool {
    s.binary_search(&p).is_ok()
}

fn neighbours((r, c): (i64, i64)) -> [(i64, i64); 4] {
    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
}

impl Sel {
    /// EVALUATE the term over a scene (the percept's occupied 0-cells) to its
    /// support set — exact, deterministic, replayable on any scene.
    pub fn eval(&self, scene: &[(i64, i64, u8)]) -> Vec<(i64, i64)> {
        match self {
            Sel::Hue(h) => sorted(
                scene
                    .iter()
                    .filter(|(_, _, x)| *h == 255 || *x == *h)
                    .map(|(r, c, _)| (*r, *c))
                    .collect(),
            ),
            Sel::Comp(e, k) => {
                let sup = e.eval(scene);
                let cells: Vec<(i64, i64, u8)> = sup.iter().map(|&(r, c)| (r, c, 0)).collect();
                let mut comps: Vec<Vec<(i64, i64)>> = components(&cells)
                    .into_iter()
                    .map(|c| sorted(c.into_iter().map(|(r, c, _)| (r, c)).collect()))
                    .collect();
                comps.sort_by_key(|c| c.first().copied());
                comps.into_iter().nth(*k as usize).unwrap_or_default()
            }
            Sel::Delta(e) => {
                let sup = e.eval(scene);
                sup.iter().copied().filter(|&p| neighbours(p).iter().any(|q| !has(&sup, *q))).collect()
            }
            Sel::Star(e) => {
                let sup = e.eval(scene);
                let mut out = sup.clone();
                for &p in &sup {
                    out.extend(neighbours(p));
                }
                sorted(out)
            }
            Sel::Ext(e, axis) => {
                let sup = e.eval(scene);
                if sup.is_empty() {
                    return sup;
                }
                match axis {
                    0 => {
                        let m = sup.iter().map(|(r, _)| *r).min().unwrap();
                        sup.into_iter().filter(|(r, _)| *r == m).collect()
                    }
                    1 => {
                        let m = sup.iter().map(|(r, _)| *r).max().unwrap();
                        sup.into_iter().filter(|(r, _)| *r == m).collect()
                    }
                    2 => {
                        let m = sup.iter().map(|(_, c)| *c).min().unwrap();
                        sup.into_iter().filter(|(_, c)| *c == m).collect()
                    }
                    _ => {
                        let m = sup.iter().map(|(_, c)| *c).max().unwrap();
                        sup.into_iter().filter(|(_, c)| *c == m).collect()
                    }
                }
            }
            Sel::Inter(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                sa.into_iter().filter(|p| has(&sb, *p)).collect()
            }
            Sel::Minus(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                sa.into_iter().filter(|p| !has(&sb, *p)).collect()
            }
            Sel::Flood(seed, within) => {
                let (sd, w) = (seed.eval(scene), within.eval(scene));
                let mut reached: Vec<(i64, i64)> =
                    sd.into_iter().filter(|p| has(&w, *p)).collect();
                reached.sort_unstable();
                let mut frontier = reached.clone();
                while let Some(p) = frontier.pop() {
                    for q in neighbours(p) {
                        if has(&w, q) && !has(&reached, q) {
                            let i = reached.binary_search(&q).unwrap_err();
                            reached.insert(i, q);
                            frontier.push(q);
                        }
                    }
                }
                reached
            }
        }
    }

    /// Node count — the term's cost (minimality prefers shallow derivations).
    pub fn size(&self) -> usize {
        match self {
            Sel::Hue(_) => 1,
            Sel::Comp(e, _) | Sel::Delta(e) | Sel::Star(e) | Sel::Ext(e, _) => 1 + e.size(),
            Sel::Inter(a, b) | Sel::Minus(a, b) | Sel::Flood(a, b) => 1 + a.size() + b.size(),
        }
    }
}

/// DERIVE the minimal term carrying a footprint in a scene: bottom-up,
/// directed by the footprint itself (its hues, the components it touches),
/// never a blind sweep. Preference: exact evaluation (eval == footprint),
/// then the smallest strict carrier (footprint ⊂ eval, minimal support then
/// minimal term). None only when the footprint is empty.
pub fn derive(footprint: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Option<Sel> {
    if footprint.is_empty() {
        return None;
    }
    let fp = sorted(footprint.to_vec());
    let terms = candidates(&fp, scene);

    // Minimal exact term first; else the smallest strict carrier.
    let mut exact: Option<&Sel> = None;
    let mut carrier: Option<(&Sel, usize)> = None;
    for t in &terms {
        let sup = t.eval(scene);
        if sup == fp {
            if exact.map_or(true, |e| t.size() < e.size()) {
                exact = Some(t);
            }
        } else if fp.iter().all(|p| has(&sup, *p)) {
            let better = carrier.map_or(true, |(c, n)| {
                sup.len() < n || (sup.len() == n && t.size() < c.size())
            });
            if better {
                carrier = Some((t, sup.len()));
            }
        }
    }
    exact.or(carrier.map(|(t, _)| t)).cloned()
}

/// The DIRECTED candidate closure for a footprint: Level-1 bases measured from
/// the footprint's own hues, then the recursive Level-2/3 composition (δ, ∖,
/// component projection, extremal projection, hue interfaces).
fn candidates(fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<Sel> {
    // Level-1 bases, DIRECTED: the whole support plus exactly the hues the
    // footprint touches (measured from the data, not enumerated blindly).
    let mut hues: Vec<u8> = scene
        .iter()
        .filter(|(r, c, _)| has(&fp, (*r, *c)))
        .map(|(_, _, h)| *h)
        .collect();
    hues.sort_unstable();
    hues.dedup();
    let mut terms: Vec<Sel> = vec![Sel::Hue(255)];
    terms.extend(hues.iter().map(|&h| Sel::Hue(h)));
    // Interfaces the footprint's hues form with the other hues present.
    let mut others: Vec<u8> = scene.iter().map(|(_, _, h)| *h).collect();
    others.sort_unstable();
    others.dedup();
    for &h in &hues {
        for &o in &others {
            if o != h {
                terms.push(Sel::Inter(
                    Box::new(Sel::Hue(h)),
                    Box::new(Sel::Star(Box::new(Sel::Hue(o)))),
                ));
            }
        }
    }

    // Level-2 closure over each base: component projection (only components
    // meeting the footprint), coboundary, interior, extremal projections.
    let mut grown: Vec<Sel> = Vec::new();
    for t in &terms {
        let sup = t.eval(scene);
        if fp.iter().any(|p| has(&sup, *p)) {
            let cells: Vec<(i64, i64, u8)> = sup.iter().map(|&(r, c)| (r, c, 0)).collect();
            let n_comps = components(&cells).len().min(255);
            for k in 0..n_comps as u8 {
                let comp = Sel::Comp(Box::new(t.clone()), k);
                let cs = comp.eval(scene);
                if fp.iter().any(|p| has(&cs, *p)) {
                    grown.push(comp);
                }
            }
            grown.push(Sel::Delta(Box::new(t.clone())));
            grown.push(Sel::Minus(Box::new(t.clone()), Box::new(Sel::Delta(Box::new(t.clone())))));
            for axis in 0..4 {
                grown.push(Sel::Ext(Box::new(t.clone()), axis));
            }
            // The star and the HALO (star ∖ support): "the cells around".
            grown.push(Sel::Star(Box::new(t.clone())));
            grown.push(Sel::Minus(
                Box::new(Sel::Star(Box::new(t.clone()))),
                Box::new(t.clone()),
            ));
            // ENCLOSURE, composed: t ∖ Flood(t ∩ δ(whole), t) — the part of t
            // unreachable from the scene's own border (the topological holes).
            let border = Sel::Delta(Box::new(Sel::Hue(255)));
            let reach = Sel::Flood(
                Box::new(Sel::Inter(Box::new(t.clone()), Box::new(border))),
                Box::new(t.clone()),
            );
            grown.push(Sel::Minus(Box::new(t.clone()), Box::new(reach)));
        }
    }
    // Level-3: coboundary/interior/extrema of the component projections (the
    // recursive step — δ and ∖ applied to already-composed terms).
    let mut deeper: Vec<Sel> = Vec::new();
    for t in grown.iter().filter(|t| matches!(t, Sel::Comp(_, _))) {
        deeper.push(Sel::Delta(Box::new(t.clone())));
        deeper.push(Sel::Minus(Box::new(t.clone()), Box::new(Sel::Delta(Box::new(t.clone())))));
        for axis in 0..4 {
            deeper.push(Sel::Ext(Box::new(t.clone()), axis));
        }
    }
    terms.extend(grown);
    terms.extend(deeper);
    terms
}

/// EVERY term that evaluates EXACTLY to the footprint on this scene — the full
/// ambiguity set of one evidence. A single pair underdetermines its anchor;
/// the train FAMILY determines it jointly: the binding term is the minimal
/// member common to every pair's exact set (deduction by intersection, the
/// same move consilience makes across voices).
pub fn derive_exact_all(footprint: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<Sel> {
    if footprint.is_empty() {
        return Vec::new();
    }
    let fp = sorted(footprint.to_vec());
    let mut out: Vec<Sel> = candidates(&fp, scene)
        .into_iter()
        .filter(|t| t.eval(scene) == fp)
        .collect();
    out.sort_by_key(|t| t.size());
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block() -> Vec<(i64, i64, u8)> {
        (0..3).flat_map(|r| (0..3).map(move |c| (r, c, 5u8))).collect()
    }

    /// Interior and coboundary are DERIVED terms (K ∖ δK and δK), never tags:
    /// the center of a block derives to Minus(Hue, Delta(Hue)) and re-evaluates
    /// exactly on a larger scene of the same species.
    #[test]
    fn interior_is_a_derived_term_that_transfers() {
        let scene = block();
        let t = derive(&[(1, 1)], &scene).expect("derivable");
        assert_eq!(t.eval(&scene), vec![(1, 1)], "exact on its own evidence");
        assert!(matches!(t, Sel::Minus(_, _)), "the interior DERIVES as K ∖ δK: {t:?}");
        // Transfer: a 5×5 block of the same hue — the SAME TERM selects the
        // new interior with no re-derivation (selection is language).
        let big: Vec<(i64, i64, u8)> =
            (0..5).flat_map(|r| (0..5).map(move |c| (r, c, 5u8))).collect();
        let inner = t.eval(&big);
        assert_eq!(inner.len(), 9, "the 3×3 interior of the 5×5 block");
        assert!(inner.contains(&(2, 2)));
    }

    /// The boundary ring derives as δ of the hue support.
    #[test]
    fn coboundary_derives_as_delta() {
        let scene = block();
        let ring: Vec<(i64, i64)> =
            scene.iter().map(|(r, c, _)| (*r, *c)).filter(|&(r, c)| !(r == 1 && c == 1)).collect();
        let t = derive(&ring, &scene).expect("derivable");
        assert_eq!(t.eval(&scene).len(), 8);
        assert!(matches!(t, Sel::Delta(_)), "δK, derived: {t:?}");
    }

    /// The hue-interface TERM K_h ∩ star(K_o) — the relative frontier composed
    /// from primitives — transfers across rearranged scenes (algebra law).
    /// (Which term `derive` picks from ONE pair is honestly ambiguous — a
    /// single exact evidence underdetermines; consensus across train pairs
    /// disambiguates in the pipeline.)
    #[test]
    fn hue_interface_term_transfers() {
        let t = Sel::Inter(Box::new(Sel::Hue(1)), Box::new(Sel::Star(Box::new(Sel::Hue(2)))));
        let scene = vec![(0, 0, 1u8), (0, 1, 1), (0, 2, 2), (0, 3, 2)];
        assert_eq!(t.eval(&scene), vec![(0, 1)], "the hue-1 cell touching hue 2");
        let moved = vec![(5, 5, 1u8), (6, 5, 1), (7, 5, 2)];
        assert_eq!(t.eval(&moved), vec![(6, 5)], "the same term finds the new frontier");
        // And derive() does produce SOME exact term from the one pair.
        let d = derive(&[(0, 1)], &scene).expect("derivable");
        assert_eq!(d.eval(&scene), vec![(0, 1)], "exact on the evidence");
    }

    /// ENCLOSURE composes from primitives: on a full-complex scene (background
    /// included), the ring's hole is Hue(0) ∖ Flood(border∩Hue(0), Hue(0)) —
    /// and the derived term transfers to a different ring.
    #[test]
    fn enclosure_composes_and_transfers() {
        // A 3×3 ring of hue 5 with a hue-0 hole, on a 5×5 full grid.
        let mut scene: Vec<(i64, i64, u8)> = Vec::new();
        for r in 0..5 {
            for c in 0..5 {
                let on_ring = (1..=3).contains(&r) && (1..=3).contains(&c) && !(r == 2 && c == 2);
                scene.push((r, c, if on_ring { 5 } else { 0 }));
            }
        }
        let t = derive(&[(2, 2)], &scene).expect("the hole is derivable");
        assert_eq!(t.eval(&scene), vec![(2, 2)], "exactly the enclosed cell");
        // Transfer: a wider ring elsewhere on a 7×7 grid — same term, new hole.
        let mut big: Vec<(i64, i64, u8)> = Vec::new();
        for r in 0..7 {
            for c in 0..7 {
                let on_ring = (2..=5).contains(&r) && (2..=5).contains(&c)
                    && !((3..=4).contains(&r) && (3..=4).contains(&c));
                big.push((r, c, if on_ring { 5 } else { 0 }));
            }
        }
        let hole = t.eval(&big);
        assert_eq!(hole.len(), 4, "the 2×2 hole of the big ring: {hole:?}");
        assert!(hole.contains(&(3, 3)));
    }

    /// Extremal projection: the top row of a component derives via Ext.
    #[test]
    fn extremal_projection_derives() {
        let scene = block();
        let top: Vec<(i64, i64)> = vec![(0, 0), (0, 1), (0, 2)];
        let t = derive(&top, &scene).expect("derivable");
        assert_eq!(t.eval(&scene), top);
    }
}
