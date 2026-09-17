// A9 — THE SELECTION TERM: what a footprint selects on a scene, and how it
// reads it.
//
//! `Sel` is a recursive selection over a scene: a hue, a delta, a
//! difference, a ring. It is EVALUATED against a scene and never guessed
//! at; where it selects nothing it says so.

use super::*;

/// A term of the selection algebra. Constructors = operators, arguments =
/// measured coordinates (hue byte, substrate axis index, component ordinal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sel {
    /// K_h — the support of hue `h`; 255 is the whole occupied support.
    Hue(u8),
    /// The k-th connected component of the term's support (canonical order:
    /// by least site) — connectivity read along edges.
    Comp(Box<Sel>, u8),
    /// δ — the coboundary: sites of the support with an edge-neighbour
    /// outside it.
    Delta(Box<Sel>),
    /// The star (the edge dilation): every site adjacent to the support,
    /// support included.
    Star(Box<Sel>),
    /// Extremal projection onto a substrate axis: sites achieving the
    /// extremum. axis 0..4 = min-row, max-row, min-col, max-col.
    Ext(Box<Sel>, u8),
    /// ∩ — algebraic intersection of supports.
    Inter(Box<Sel>, Box<Sel>),
    /// ∖ — set difference of supports.
    Minus(Box<Sel>, Box<Sel>),
    /// Connected closure: the sites of the second support edge-reachable
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
    /// SITE TRANSPORT intersection: a ∩ Shift_d(b) where d is the exact
    /// bbox-min corner difference extrema(a) − extrema(b) — the offset is
    /// DERIVED from the two supports at evaluation time, never a parameter.
    Transport(Box<Sel>, Box<Sel>),
}

pub(crate) fn sorted(mut v: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    v.sort_unstable();
    v.dedup();
    v
}

pub(crate) fn has(s: &[(i64, i64)], p: (i64, i64)) -> bool {
    s.binary_search(&p).is_ok()
}

pub(crate) fn neighbours((r, c): (i64, i64)) -> [(i64, i64); 4] {
    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
}

impl Sel {
    /// EVALUATE the term over a scene (the percept's painted sites) to its
    /// support set — exact, deterministic, replayable on any scene.
    ///
    /// `Ext` IS an extremum on a substrate axis and `ArgExt` IS the
    /// component uniquely extremizing a measured invariant: the reading is
    /// the operator's definition, not a candidate picked by a scalar, and
    /// `ArgExt` selects nothing on a tie rather than breaking one (rule 4).
    /// `Transport` derives its offset from the two bbox corners.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — an extremum on an axis is this algebra's operator, and a tie selects nothing"
    )]
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
                // The scene is one grid's: its sites stand in frame 0.
                let laminae: Vec<(i64, i64, i64, u8)> = sup.iter().map(|&(r, c)| (r, c, 0, 0)).collect();
                let mut comps: Vec<Vec<(i64, i64)>> = components(&laminae)
                    .into_iter()
                    .map(|c| sorted(c.into_iter().map(|(r, c, _, _)| (r, c)).collect()))
                    .collect();
                // A component IS its least site, in the coordinate order the
                // support is already written in; a component with no site
                // has no least one and sorts after those that do.
                #[expect(
                    clippy::disallowed_methods,
                    reason = "rule 4 — canonical key order: the component's least site, the support's own coordinate"
                )]
                comps.sort_by_key(|c| match c.as_slice() {
                    [least, ..] => Some(*least),
                    [] => None,
                });
                // A term naming a component the support does not have
                // selects nothing (§2.4), which is the empty support and
                // not a default standing in for one (rule 11).
                #[expect(
                    clippy::manual_unwrap_or_default,
                    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; the empty support is measured here, not defaulted to"
                )]
                match comps.into_iter().nth(*k as usize) {
                    Some(c) => c,
                    None => Vec::new(),
                }
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
                // The extremum of a non-empty support exists; an empty one
                // was returned above, and selects nothing here (§2.4).
                let rows = || sup.iter().map(|(r, _)| *r);
                let cols = || sup.iter().map(|(_, c)| *c);
                let extremum = match axis {
                    0 => rows().min(),
                    1 => rows().max(),
                    2 => cols().min(),
                    _ => cols().max(),
                };
                let Some(m) = extremum else { return Vec::new() };
                match axis {
                    0 | 1 => sup.into_iter().filter(|(r, _)| *r == m).collect(),
                    _ => sup.into_iter().filter(|(_, c)| *c == m).collect(),
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
                let laminae: Vec<(i64, i64, i64, u8)> = sup.iter().map(|&(r, c)| (r, c, 0, 0)).collect();
                let comps: Vec<Vec<(i64, i64)>> = components(&laminae)
                    .into_iter()
                    .map(|c| sorted(c.into_iter().map(|(r, c, _, _)| (r, c)).collect()))
                    .collect();
                if comps.is_empty() {
                    return Vec::new();
                }
                // A component with no sites has no diameter to measure, so
                // the measure refuses rather than reading zero for it
                // (rule 11); `components` returns none such, and a term
                // whose measure refuses selects nothing below.
                let measure = |k: &Vec<(i64, i64)>| -> Option<i64> {
                    match code {
                        0 | 1 => Some(k.len() as i64),
                        _ => {
                            let rows = || k.iter().map(|(r, _)| *r);
                            let cols = || k.iter().map(|(_, c)| *c);
                            let (r0, r1) = (rows().min()?, rows().max()?);
                            let (c0, c1) = (cols().min()?, cols().max()?);
                            Some((r1 - r0) + (c1 - c0))
                        }
                    }
                };
                let Some(vals) = comps.iter().map(measure).collect::<Option<Vec<i64>>>() else {
                    return Vec::new();
                };
                let target = match code {
                    0 | 2 => vals.iter().max(),
                    _ => vals.iter().min(),
                };
                let Some(target) = target.copied() else { return Vec::new() };
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
                    Some((s.iter().map(|(r, _)| *r).min()?, s.iter().map(|(_, c)| *c).min()?))
                };
                let (Some((ar, ac)), Some((br, bc))) = (corner(&sa), corner(&sb)) else {
                    return Vec::new();
                };
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
                            // Either arm is the seat `q` belongs at, so the
                            // search answers a position and never a failure
                            // to be unwrapped (§2.4).
                            let (Ok(i) | Err(i)) = reached.binary_search(&q);
                            reached.insert(i, q);
                            frontier.push(q);
                        }
                    }
                }
                reached
            }
        }
    }

    /// A21 KEY of the term: its tape under the selection tag. The term's
    /// own canonical coordinates, so two nodes order a set of terms alike
    /// without measuring any of them against a scene.
    pub fn key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_SEL];
        self.to_bytes(&mut buf);
        crate::transducer::address::Address::new(buf)
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
