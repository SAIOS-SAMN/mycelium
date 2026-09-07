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
    /// ORDER-INVARIANT component selection: of the support's connected
    /// components, the one UNIQUELY extremizing a measured C₀ invariant.
    /// Code 0..4 = (max |K|, min |K|, max L1-bbox-diameter, min diameter).
    /// A tie selects NOTHING — no unique extremum, no selection (exactness,
    /// never a coin flip).
    ArgExt(Box<Sel>, u8),
    /// 1-cell TRANSPORT intersection: a ∩ Shift_d(b) where d is the exact
    /// bbox-min corner difference extrema(a) − extrema(b) — the offset is
    /// DERIVED from the two supports at evaluation time, never a parameter.
    Transport(Box<Sel>, Box<Sel>),
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
            Sel::ArgExt(e, code) => {
                let sup = e.eval(scene);
                let cells: Vec<(i64, i64, u8)> = sup.iter().map(|&(r, c)| (r, c, 0)).collect();
                let comps: Vec<Vec<(i64, i64)>> = components(&cells)
                    .into_iter()
                    .map(|c| sorted(c.into_iter().map(|(r, c, _)| (r, c)).collect()))
                    .collect();
                if comps.is_empty() {
                    return Vec::new();
                }
                let measure = |k: &Vec<(i64, i64)>| -> i64 {
                    match code {
                        0 | 1 => k.len() as i64,
                        _ => {
                            let (r0, r1) = (
                                k.iter().map(|(r, _)| *r).min().unwrap_or(0),
                                k.iter().map(|(r, _)| *r).max().unwrap_or(0),
                            );
                            let (c0, c1) = (
                                k.iter().map(|(_, c)| *c).min().unwrap_or(0),
                                k.iter().map(|(_, c)| *c).max().unwrap_or(0),
                            );
                            (r1 - r0) + (c1 - c0)
                        }
                    }
                };
                let vals: Vec<i64> = comps.iter().map(measure).collect();
                let target = match code {
                    0 | 2 => vals.iter().max(),
                    _ => vals.iter().min(),
                }
                .copied()
                .unwrap();
                let winners: Vec<&Vec<(i64, i64)>> = comps
                    .iter()
                    .zip(&vals)
                    .filter(|(_, v)| **v == target)
                    .map(|(k, _)| k)
                    .collect();
                match winners.as_slice() {
                    [only] => (*only).clone(),
                    _ => Vec::new(), // tie: no unique extremum, no selection
                }
            }
            Sel::Transport(a, b) => {
                let (sa, sb) = (a.eval(scene), b.eval(scene));
                if sa.is_empty() || sb.is_empty() {
                    return Vec::new();
                }
                let corner = |s: &[(i64, i64)]| {
                    (
                        s.iter().map(|(r, _)| *r).min().unwrap(),
                        s.iter().map(|(_, c)| *c).min().unwrap(),
                    )
                };
                let (ar, ac) = corner(&sa);
                let (br, bc) = corner(&sb);
                let (dr, dc) = (ar - br, ac - bc);
                let shifted: Vec<(i64, i64)> =
                    sorted(sb.iter().map(|(r, c)| (r + dr, c + dc)).collect());
                sa.into_iter().filter(|p| has(&shifted, *p)).collect()
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
            Sel::Inter(a, b) | Sel::Minus(a, b) | Sel::Flood(a, b) | Sel::Transport(a, b) => {
                1 + a.size() + b.size()
            }
            Sel::ArgExt(e, _) => 1 + e.size(),
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
        .filter(|(r, c, _)| has(fp, (*r, *c)))
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

    // ORDER-INVARIANT selections join the bases before the closure, so
    // "the boundary of the largest component" etc. compose naturally.
    let arg_bases: Vec<Sel> = terms
        .iter()
        .flat_map(|t| (0..4).map(|code| Sel::ArgExt(Box::new(t.clone()), code)))
        .collect();
    terms.extend(arg_bases);
    // 1-cell transports between the footprint's hues and the others.
    for &h in &hues {
        for &o in &others {
            if o != h {
                terms.push(Sel::Transport(Box::new(Sel::Hue(h)), Box::new(Sel::Hue(o))));
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

/// EVERY term CARRYING the footprint on this scene (eval ⊇ footprint), ranked
/// tightest-first (carrier size, then term size) and capped. Exact terms rank
/// first automatically (their eval IS the footprint). A single pair
/// underdetermines its anchor; the train FAMILY determines it jointly: the
/// binding term is the first member common to every pair's set — the tightest
/// common carrier, exact when an exact one is shared (deduction by
/// intersection, the same move consilience makes across voices). A carrier
/// that is not exact still confines the word to its subcomplex — the replay
/// gate downstream decides whether confined speech reproduces the evidence.
pub fn derive_carrying_all(footprint: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<Sel> {
    if footprint.is_empty() {
        return Vec::new();
    }
    let fp = sorted(footprint.to_vec());
    let mut ranked: Vec<(usize, usize, Sel)> = candidates(&fp, scene)
        .into_iter()
        .filter_map(|t| {
            let sup = t.eval(scene);
            if fp.iter().all(|p| has(&sup, *p)) {
                Some((sup.len(), t.size(), t))
            } else {
                None
            }
        })
        .collect();
    ranked.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    let mut out: Vec<Sel> = ranked.into_iter().map(|(_, _, t)| t).collect();
    out.dedup();
    out.truncate(32);
    out
}

impl Sel {
    /// SERIALIZE the term — prefix walk, one opcode byte per node, operands
    /// inline. Opcodes are the algebra's operators (wire form of the same
    /// constructors), never semantic labels.
    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        match self {
            Sel::Hue(h) => {
                out.push(0);
                out.push(*h);
            }
            Sel::Comp(e, k) => {
                out.push(1);
                out.push(*k);
                e.to_bytes(out);
            }
            Sel::Delta(e) => {
                out.push(2);
                e.to_bytes(out);
            }
            Sel::Star(e) => {
                out.push(3);
                e.to_bytes(out);
            }
            Sel::Ext(e, a) => {
                out.push(4);
                out.push(*a);
                e.to_bytes(out);
            }
            Sel::Inter(a, b) => {
                out.push(5);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::Minus(a, b) => {
                out.push(6);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::Flood(a, b) => {
                out.push(7);
                a.to_bytes(out);
                b.to_bytes(out);
            }
            Sel::ArgExt(e, c) => {
                out.push(8);
                out.push(*c);
                e.to_bytes(out);
            }
            Sel::Transport(a, b) => {
                out.push(9);
                a.to_bytes(out);
                b.to_bytes(out);
            }
        }
    }

    /// Parse a term (untrusted input: depth-capped at 16, every read guarded —
    /// a corrupt term is None, never a crash or an allocation bomb).
    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Sel> {
        Sel::parse(b, pos, 0)
    }

    fn parse(b: &[u8], pos: &mut usize, depth: u8) -> Option<Sel> {
        if depth > 16 {
            return None;
        }
        let op = *b.get(*pos)?;
        *pos += 1;
        Some(match op {
            0 => {
                let h = *b.get(*pos)?;
                *pos += 1;
                Sel::Hue(h)
            }
            1 => {
                let k = *b.get(*pos)?;
                *pos += 1;
                Sel::Comp(Box::new(Sel::parse(b, pos, depth + 1)?), k)
            }
            2 => Sel::Delta(Box::new(Sel::parse(b, pos, depth + 1)?)),
            3 => Sel::Star(Box::new(Sel::parse(b, pos, depth + 1)?)),
            4 => {
                let a = *b.get(*pos)?;
                *pos += 1;
                Sel::Ext(Box::new(Sel::parse(b, pos, depth + 1)?), a)
            }
            5 => Sel::Inter(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            6 => Sel::Minus(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            7 => Sel::Flood(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            8 => {
                let c = *b.get(*pos)?;
                *pos += 1;
                Sel::ArgExt(Box::new(Sel::parse(b, pos, depth + 1)?), c)
            }
            9 => Sel::Transport(
                Box::new(Sel::parse(b, pos, depth + 1)?),
                Box::new(Sel::parse(b, pos, depth + 1)?),
            ),
            _ => return None,
        })
    }
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

    /// Serde round-trips every operator exactly; hostile bytes refuse cleanly.
    #[test]
    fn sel_serde_round_trips() {
        let t = Sel::Minus(
            Box::new(Sel::ArgExt(Box::new(Sel::Hue(3)), 0)),
            Box::new(Sel::Flood(
                Box::new(Sel::Inter(Box::new(Sel::Hue(0)), Box::new(Sel::Delta(Box::new(Sel::Hue(255)))))),
                Box::new(Sel::Transport(Box::new(Sel::Hue(0)), Box::new(Sel::Star(Box::new(Sel::Ext(Box::new(Sel::Comp(Box::new(Sel::Hue(1)), 2)), 3)))))),
            )),
        );
        let mut buf = Vec::new();
        t.to_bytes(&mut buf);
        let mut pos = 0;
        assert_eq!(Sel::from_bytes(&buf, &mut pos), Some(t), "every operator survives the wire");
        assert_eq!(pos, buf.len(), "the parse consumes exactly the term");
        // Hostile: unknown opcode, truncation, and a depth bomb all refuse.
        assert_eq!(Sel::from_bytes(&[200], &mut 0), None);
        assert_eq!(Sel::from_bytes(&buf[..buf.len() - 1], &mut 0), None);
        let bomb = vec![2u8; 64]; // Delta(Delta(Delta(... unterminated
        assert_eq!(Sel::from_bytes(&bomb, &mut 0), None, "depth cap holds");
    }

    /// The LARGEST component selects by order invariant — uniquely, with ties
    /// selecting nothing — and the same term transfers to a rearranged scene.
    #[test]
    fn argmax_component_selects_uniquely_and_transfers() {
        let scene = vec![(0, 0, 3u8), (0, 1, 3), (0, 2, 3), (5, 5, 3)];
        let t = Sel::ArgExt(Box::new(Sel::Hue(3)), 0);
        assert_eq!(t.eval(&scene), vec![(0, 0), (0, 1), (0, 2)], "the 3-cell body wins");
        let small = Sel::ArgExt(Box::new(Sel::Hue(3)), 1);
        assert_eq!(small.eval(&scene), vec![(5, 5)], "the isolated cell is the argmin");
        let moved = vec![(9, 9, 3u8), (2, 0, 3), (2, 1, 3)];
        assert_eq!(t.eval(&moved), vec![(2, 0), (2, 1)], "the same term, the new largest");
        // A TIE selects nothing: two equal bodies have no unique extremum.
        let tie = vec![(0, 0, 3u8), (0, 1, 3), (5, 5, 3), (5, 6, 3)];
        assert!(t.eval(&tie).is_empty(), "no coin flips");
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
