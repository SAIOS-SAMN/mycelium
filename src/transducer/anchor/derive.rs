// A9 — DERIVING the selection a family agrees on: the candidates a
// footprint admits, and the one that carries every scene.

use super::*;

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
            if exact.is_none_or(|e| t.size() < e.size()) {
                exact = Some(t);
            }
        } else if fp.iter().all(|p| has(&sup, *p)) {
            let better = carrier.is_none_or(|(c, n)| {
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
pub(crate) fn candidates(fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<Sel> {
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
    // Site transports between the footprint's hues and the others.
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
            // `Sel::Comp` names a component in ONE BYTE, so a support with
            // more components than that has terms this algebra cannot
            // spell: they are not derived rather than folded onto index
            // 255, which would name a different component.
            let Ok(n_comps) = u8::try_from(super::sel::planar_components(&sup).len()) else {
                continue;
            };
            for k in 0..n_comps {
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
            // The star and the HALO (star ∖ support): "the sites around".
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

/// EVERY term CARRYING the footprint on this scene (eval ⊇ footprint),
/// ordered by the TERM — node count, then its own A21 key — and capped. Exact terms rank
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
    let mut ranked: Vec<(usize, crate::transducer::address::Address, Sel)> = candidates(&fp, scene)
        .into_iter()
        .filter_map(|t| {
            let sup = t.eval(scene);
            if fp.iter().all(|p| has(&sup, *p)) {
                Some((t.size(), t.key(), t))
            } else {
                None
            }
        })
        .collect();
    // ORDERED BY THE TERM, NEVER BY THE SCENE — and the order is still
    // CONSUMED. The rank was `(sup.len(), t.size())`, the carrier's size on
    // this scene first, which ranked candidates by a measurement; it is the
    // term's own coordinates now, so the order is the same on every scene
    // and every node. But `truncate(32)` below keeps the thirty-two
    // smallest and drops the rest, which is a choice by the sort key
    // however canonical the key is. That cut is L9a's row, open since
    // Stage 14, and this reason says so rather than calling the site
    // settled.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — a choice by node count then key, consumed by truncate(32); owed to L9a"
    )]
    ranked.sort_by_key(|a| (a.0, a.1.clone()));
    let mut out: Vec<Sel> = ranked.into_iter().map(|(_, _, t)| t).collect();
    out.dedup();
    // AT1 (r4) — the anchor's sort-and-cut, counted at its site with what
    // it dropped.
    if out.len() > 32 {
        crate::transducer::sentence::form::atlas::note_anchor_cut(out.len() - 32);
    }
    out.truncate(32);
    out
}
