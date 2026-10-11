// 78.99 (4), M8, 13.9 — GLUING PRODUCES MEANING: the non-abelian connecting
// map `δ : H⁰(U ∩ V) → H¹(U ∪ V)`, read on two constructions glued along the
// vertices and tethers they share.
//
//! The coefficients are rotors and do not commute, so no abelian long exact
//! sequence stands above degree one, and computing one would abelianize — the
//! flattening (78.99 (4)). What stands exactly is the connecting map. Each
//! part placed by its own gauge (12.2's tree walk, exact on that part), a
//! section that agrees on every component of the overlap but does not extend
//! is a loop through the overlap — out through `U` from the base component
//! to component `i`, back through `V` — and its holonomy
//!
//! ```text
//!     Ω_i = (γᵁ_i (γᵁ_0)⁻¹) · (γⱽ_i (γⱽ_0)⁻¹)⁻¹        on both halves
//! ```
//!
//! is a class NEITHER PART HELD: classified alone and never multiplied with
//! another (every route kept, 78.44). A connected overlap makes nothing. A
//! part that is not exact on its own refuses by name — its own holonomy
//! would stand under every loop through it, and the new class is not read
//! beneath it (rule 11). A tether both parts hold with different transitions
//! is the Mayer–Vietoris inexactness the court refuses (`REFUSE_MV_INEXACT`).
//!
//! GLUING ALSO CAPS. A hole of one part whose cell the other holds sealed is
//! capped in the union: its class is killed, read beside the classes made.

use super::*;
use crate::network::nerve::Keyed;

/// WHAT A GLUE READ.
#[derive(Clone, Debug)]
pub struct Glued {
    /// The overlap's components: the vertices the two parts share, joined by
    /// the tethers both hold.
    pub components: usize,
    /// THE CLASSES THE GLUE MADE: per loop through the overlap, the base
    /// component's vertex, the other component's vertex, and the class of
    /// its holonomy.
    pub made: Vec<(Vec<u8>, Vec<u8>, Fill)>,
    /// Loops through the overlap whose holonomy is the identity: flat, they
    /// make nothing.
    pub flat: usize,
    /// THE HOLES THE GLUE CAPPED: a hole of one part sealed in the other, and
    /// the class it kept.
    pub capped: Vec<(Vec<u8>, Fill)>,
}

/// WHY A GLUE WAS NOT READ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Unglued {
    /// The parts share no vertex.
    NoOverlap,
    /// A tether both hold carries two transitions: Mayer–Vietoris inexact.
    Inexact(Vec<u8>),
    /// One part carries holonomy of its own (`0` for `U`, `1` for `V`).
    PartNotExact(u8),
    /// A transition has no reverse: unread.
    Unread,
}

/// Union-find over `n` indices, by path halving.
fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// ONE PART PLACED: per vertex key, its component's root and its section
/// relative to it — each component gauged by the tree walk on its own edges.
/// `Err(())` where a component's chords carry holonomy.
fn placed(cx: &Complex) -> Result<Keyed<(usize, Rotor, Versor)>, ()> {
    let verts: Vec<Vec<u8>> = cx.sections.scoped().map(|(k, _)| k).collect();
    let mut index: Keyed<usize> = Keyed::new();
    for (i, v) in verts.iter().enumerate() {
        index.insert(v, i);
    }
    let edges: Vec<(usize, usize, Rotor, Versor)> = match cx.degrees.get(1) {
        Some(layer) => layer
            .scoped()
            .filter_map(|(_, s)| match s.vertices.as_slice() {
                [a, b] => Some((*index.get(a)?, *index.get(b)?, s.coefficient.clone(), s.carriage.clone())),
                _ => None,
            })
            .collect(),
        None => Vec::new(),
    };
    let mut parent: Vec<usize> = (0..verts.len()).collect();
    for (i, j, _, _) in &edges {
        let (a, b) = (find(&mut parent, *i), find(&mut parent, *j));
        if a != b {
            parent[a] = b;
        }
    }
    // EACH COMPONENT GAUGED ON ITS OWN: its members and its edges grouped by
    // root in one pass each, re-indexed from zero, so the walk's root is the
    // component's first vertex in key order.
    let roots: Vec<usize> = (0..verts.len()).map(|i| find(&mut parent, i)).collect();
    let mut members: Vec<Vec<usize>> = vec![Vec::new(); verts.len()];
    let mut local: Vec<usize> = vec![0; verts.len()];
    for (i, r) in roots.iter().enumerate() {
        local[i] = members[*r].len();
        members[*r].push(i);
    }
    let mut sub: Vec<Vec<(usize, usize, Rotor, Versor)>> = vec![Vec::new(); verts.len()];
    for (i, j, g, c) in &edges {
        sub[roots[*i]].push((local[*i], local[*j], g.clone(), c.clone()));
    }
    let mut out: Keyed<(usize, Rotor, Versor)> = Keyed::new();
    for (root, group) in members.iter().enumerate().filter(|(_, g)| !g.is_empty()) {
        let gauge = solve_gauge(group.len(), &sub[root]).ok_or(())?;
        if !gauge.holonomy.is_empty() {
            return Err(());
        }
        for (m, (g, c)) in group.iter().zip(gauge.sections) {
            out.insert(&verts[*m], (root, g, c));
        }
    }
    Ok(out)
}

/// `γ_x γ_y⁻¹` on both halves, as rays.
fn relative(x: &(usize, Rotor, Versor), y: &(usize, Rotor, Versor)) -> Option<(Rotor, Versor)> {
    let yr = y.1.is_versor().then(|| y.1.reverse())?;
    Some((x.1.geo(&yr), x.2.geo(&y.2.reverse())))
}


impl Complex {
    /// THE CONSTRUCTION ONE VERTEX STANDS ON: its section, its tethers, and
    /// the sections at their far ends — a star, a tree, exact by
    /// construction. Two vertices' stars glued by [`connecting`] read the
    /// loops through what the two share: each common neighbour a component of
    /// the overlap, each loop the holonomy round the two and that neighbour.
    pub fn star_of(&self, v: &[u8]) -> Complex {
        let mut out = Complex::new();
        let Some(at) = self.sections.get(v) else { return out };
        out.sections.insert(v, at.clone());
        for key in self.link(v, 1) {
            let Some(s) = self.simplex(&key) else { continue };
            for u in &s.vertices {
                if let Some(sec) = self.sections.get(u) {
                    out.sections.insert(u, sec.clone());
                }
            }
            out.remember(s.clone());
        }
        out
    }
}

/// THE CONNECTING MAP on two constructions glued along what they share: the
/// reading is the outcome, kept whole, never counted.
pub fn connecting(u: &Complex, v: &Complex) -> Result<Glued, Unglued> {
    use crate::transducer::element::{identity_versor, same_versor};
    // THE OVERLAP: the vertices both parts section, joined by the tethers both
    // hold — which must carry one transition (Mayer–Vietoris exactness).
    let shared: Vec<Vec<u8>> = u.sections.scoped().filter(|(k, _)| v.sections.contains(k)).map(|(k, _)| k).collect();
    if shared.is_empty() {
        return Err(Unglued::NoOverlap);
    }
    let mut at: Keyed<usize> = Keyed::new();
    for (i, s) in shared.iter().enumerate() {
        at.insert(s, i);
    }
    let mut parent: Vec<usize> = (0..shared.len()).collect();
    // `U`'s tethers on two shared vertices, each read in `V` by its key: the
    // overlap's tethers, never a walk over pairs of shared vertices.
    if let Some(layer) = u.degrees.get(1) {
        for (key, s) in layer.scoped() {
            let [a, b] = s.vertices.as_slice() else { continue };
            let (Some(&i), Some(&j)) = (at.get(a), at.get(b)) else { continue };
            let Some(other) = v.simplex(&key) else { continue };
            if !(s.coefficient.projective_eq(&other.coefficient) && same_versor(&s.carriage, &other.carriage)) {
                return Err(Unglued::Inexact(key));
            }
            let (x, y) = (find(&mut parent, i), find(&mut parent, j));
            if x != y {
                parent[x] = y;
            }
        }
    }
    // Each overlap component by its first vertex in key order.
    let mut first: Vec<Option<usize>> = vec![None; shared.len()];
    let mut reps: Vec<usize> = Vec::new();
    for i in 0..shared.len() {
        let r = find(&mut parent, i);
        if first[r].is_none() {
            first[r] = Some(i);
            reps.push(i);
        }
    }
    let pu = placed(u).map_err(|()| Unglued::PartNotExact(0))?;
    let pv = placed(v).map_err(|()| Unglued::PartNotExact(1))?;
    let mut made = Vec::new();
    let mut flat = 0usize;
    // A LOOP through the overlap runs between two of its components lying in
    // one component of `U` and one of `V`; each such pair is read against
    // the first component of its class, once.
    let mut bases: Keyed<usize> = Keyed::new();
    for &i in &reps {
        let (Some(ui), Some(vi)) = (pu.get(&shared[i]), pv.get(&shared[i])) else { continue };
        // The pair of components it lies in, as its coordinate: the first
        // overlap component there is the base, the rest loop against it.
        let mut class = Vec::new();
        crate::transducer::address::put_key_count(&mut class, ui.0);
        crate::transducer::address::put_key_count(&mut class, vi.0);
        let Some(&base) = bases.get(&class) else {
            bases.insert(&class, i);
            continue;
        };
        let (Some(ub), Some(vb)) = (pu.get(&shared[base]), pv.get(&shared[base])) else { continue };
        let (Some((ru, cu)), Some((rv, cv))) = (relative(ui, ub), relative(vi, vb)) else { return Err(Unglued::Unread) };
        let rv_inv = rv.is_versor().then(|| rv.reverse()).ok_or(Unglued::Unread)?;
        let (omega_r, omega_c) = (ru.geo(&rv_inv), cu.geo(&cv.reverse()));
        if omega_r.is_identity() && same_versor(&omega_c, &identity_versor()) {
            flat += 1;
            continue;
        }
        let class = classify(&omega_r, &omega_c, crate::transducer::sentence::CycleType::Identity);
        made.push((shared[base].clone(), shared[i].clone(), class));
    }
    // A hole of one part whose cell the other holds sealed: capped.
    let mut capped = Vec::new();
    for (holder, other) in [(u, v), (v, u)] {
        for (k, fill) in holder.holes() {
            if other.simplex(&k).is_some_and(|s| !s.fill.unsettled()) {
                capped.push((k, fill.clone()));
            }
        }
    }
    Ok(Glued { components: reps.len(), made, flat, capped })
}
