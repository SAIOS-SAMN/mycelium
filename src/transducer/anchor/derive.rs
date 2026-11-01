// A9 — DERIVING the selection a family agrees on: the candidates a
// footprint admits, and the one that carries every scene.

use super::*;

/// DERIVE the minimal term carrying a footprint in a scene: bottom-up,
/// directed by the footprint itself (its hues, the components it touches),
/// never a blind sweep. Preference: exact evaluation (eval == footprint),
/// then the smallest strict carrier (footprint ⊂ eval, minimal support then
/// minimal term). None only when the footprint is empty.
pub fn derive(footprint: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Option<Selection> {
    if footprint.is_empty() {
        return None;
    }
    let fp = sorted(footprint.to_vec());
    let terms = candidates(&fp, scene);

    // Minimal exact term first; else the smallest strict carrier.
    let mut exact: Option<&Selection> = None;
    let mut carrier: Option<(&Selection, usize)> = None;
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
pub(crate) fn candidates(fp: &[(i64, i64)], scene: &[(i64, i64, u8)]) -> Vec<Selection> {
    // Level-1 bases, DIRECTED: the whole support plus exactly the hues the
    // footprint touches (measured from the data, not enumerated blindly).
    let mut hues: Vec<u8> = scene
        .iter()
        .filter(|(r, c, _)| has(fp, (*r, *c)))
        .map(|(_, _, h)| *h)
        .collect();
    hues.sort_unstable();
    hues.dedup();
    let mut terms: Vec<Selection> = vec![Selection::Hue(255)];
    terms.extend(hues.iter().map(|&h| Selection::Hue(h)));
    // Interfaces the footprint's hues form with the other hues present.
    let mut others: Vec<u8> = scene.iter().map(|(_, _, h)| *h).collect();
    others.sort_unstable();
    others.dedup();
    for &h in &hues {
        for &o in &others {
            if o != h {
                terms.push(Selection::Inter(
                    Box::new(Selection::Hue(h)),
                    Box::new(Selection::Star(Box::new(Selection::Hue(o)))),
                ));
            }
        }
    }

    // ORDER-INVARIANT selections join the bases before the closure, so
    // "the boundary of the largest component" etc. compose naturally.
    let arg_bases: Vec<Selection> = terms
        .iter()
        .flat_map(|t| (0..4).map(|code| Selection::ArgExt(Box::new(t.clone()), code)))
        .collect();
    terms.extend(arg_bases);
    // Site transports between the footprint's hues and the others.
    for &h in &hues {
        for &o in &others {
            if o != h {
                terms.push(Selection::Transport(Box::new(Selection::Hue(h)), Box::new(Selection::Hue(o))));
            }
        }
    }

    // Level-2 closure over each base: component projection (only components
    // meeting the footprint), coboundary, interior, extremal projections.
    let mut grown: Vec<Selection> = Vec::new();
    for t in &terms {
        let sup = t.eval(scene);
        if fp.iter().any(|p| has(&sup, *p)) {
            // `Selection::Comp` names a component in ONE BYTE, so a support with
            // more components than that has terms this algebra cannot
            // spell: they are not derived rather than folded onto index
            // 255, which would name a different component.
            let Ok(n_comps) = u8::try_from(super::selection::planar_components(&sup).len()) else {
                continue;
            };
            for k in 0..n_comps {
                let comp = Selection::Comp(Box::new(t.clone()), k);
                let cs = comp.eval(scene);
                if fp.iter().any(|p| has(&cs, *p)) {
                    grown.push(comp);
                }
            }
            grown.push(Selection::Delta(Box::new(t.clone())));
            grown.push(Selection::Minus(Box::new(t.clone()), Box::new(Selection::Delta(Box::new(t.clone())))));
            for axis in 0..4 {
                grown.push(Selection::Ext(Box::new(t.clone()), axis));
            }
            // The star and the HALO (star ∖ support): "the sites around".
            grown.push(Selection::Star(Box::new(t.clone())));
            grown.push(Selection::Minus(
                Box::new(Selection::Star(Box::new(t.clone()))),
                Box::new(t.clone()),
            ));
            // ENCLOSURE, composed: t ∖ Flood(t ∩ δ(whole), t) — the part of t
            // unreachable from the scene's own border (the topological holes).
            let border = Selection::Delta(Box::new(Selection::Hue(255)));
            let reach = Selection::Flood(
                Box::new(Selection::Inter(Box::new(t.clone()), Box::new(border))),
                Box::new(t.clone()),
            );
            grown.push(Selection::Minus(Box::new(t.clone()), Box::new(reach)));
        }
    }
    // Level-3: coboundary/interior/extrema of the component projections (the
    // recursive step — δ and ∖ applied to already-composed terms).
    let mut deeper: Vec<Selection> = Vec::new();
    for t in grown.iter().filter(|t| matches!(t, Selection::Comp(_, _))) {
        deeper.push(Selection::Delta(Box::new(t.clone())));
        deeper.push(Selection::Minus(Box::new(t.clone()), Box::new(Selection::Delta(Box::new(t.clone())))));
        for axis in 0..4 {
            deeper.push(Selection::Ext(Box::new(t.clone()), axis));
        }
    }
    terms.extend(grown);
    terms.extend(deeper);
    terms
}

/// WHY A CARRYING SET WAS NOT READ: the token spent before every candidate
/// term was evaluated on the scene, with how many were — the whole set is
/// refused, never a part of it (a part intersected would be a choice by
/// arrival order; MU-DERIVATIONS D3.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CarryingRefusal {
    Unread { terms_read: usize },
}

/// Whether `t` carries the footprint `fp` (sorted) on `scene`, its read
/// metered: one read per node of the term, each node reading its support of
/// the scene once. `None` where the token is spent.
pub(crate) fn carries(t: &Selection, fp: &[(i64, i64)], scene: &[(i64, i64, u8)], budget: &mut crate::physics::Budget) -> Option<bool> {
    for _ in 0..t.size() {
        if !budget.spend() {
            return None;
        }
    }
    let sup = t.eval(scene);
    Some(fp.iter().all(|p| has(&sup, *p)))
}

/// Whether `t` carries `footprint` on `scene` — the reconstruction a court
/// judges a held term by on a withheld pair — metered as [`carries`].
pub fn carries_footprint(t: &Selection, footprint: &[(i64, i64)], scene: &[(i64, i64, u8)], budget: &mut crate::physics::Budget) -> Option<bool> {
    carries(t, &sorted(footprint.to_vec()), scene, budget)
}

/// EVERY term CARRYING the footprint on this scene (eval ⊇ footprint),
/// keyed by its own A21 key: the closure's every member, none cut (rule 15;
/// MU-DERIVATIONS D3.3, ruled 2026-10-31). A single pair underdetermines its
/// anchor; the train FAMILY determines it jointly (`derive_pi`): the terms
/// common to every pair's set, read by key. The walk is the closure's
/// boundary, each evaluation metered; a spent token refuses the whole set.
pub fn derive_carrying_all(
    footprint: &[(i64, i64)],
    scene: &[(i64, i64, u8)],
    budget: &mut crate::physics::Budget,
) -> Result<crate::network::nerve::Keyed<Selection>, CarryingRefusal> {
    let mut out = crate::network::nerve::Keyed::new();
    if footprint.is_empty() {
        return Ok(out);
    }
    let fp = sorted(footprint.to_vec());
    for (terms_read, t) in candidates(&fp, scene).into_iter().enumerate() {
        match carries(&t, &fp, scene, budget) {
            Some(true) => {
                out.insert(t.key().as_bytes(), t);
            }
            Some(false) => {}
            None => return Err(CarryingRefusal::Unread { terms_read }),
        }
    }
    Ok(out)
}
