// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21 step 2 — THE SCENE AS A COMPLEX, and the pairing over it: the
//! candidate bodies a grid presents, the charts they group under, the
//! carriage that pairs one with another, and the faces their supports
//! name.
//!
//! THE SCENE IS NOT FOUR ARRAYS. It was: `ins` and `outs` as `Vec<Cand>`
//! indexed by the order `perceive` happened to return bodies in, `used_in`
//! and `used_out` as `Vec<bool>` aligned to them by arithmetic, `pairs` as
//! index triples into both, and `group_by_chart` answering lists of
//! indices. Every one of those is a member named by its arrival position
//! (rule 15) and a presence held as a boolean rather than read from a
//! keyed set (rule 2) — and the parallel arrays are the shape that made
//! `usize::MAX` reachable for as "no member here".
//!
//! A candidate is addressed by its own CONTENT: the shell kind it came
//! from and its cells written out, which `body_of` has already sorted. The
//! scene is a `Keyed` of those; consumption is a keyed set; a pair is
//! keyed by its two ends the way a carriage is (`carriage_key`); and a
//! chart group is a list of addresses. Nothing in the scene is indexed by
//! when it was perceived.
//!
//! Split from `pair.rs` at the §1.1 600-line reading bound.

use super::*;

thread_local! {
    static FACE_WORK: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn note_face_work() {
    FACE_WORK.with(|c| c.set(c.get().saturating_add(1)));
}

#[cfg(test)]
pub(crate) fn reset_face_work() {
    FACE_WORK.with(|c| c.set(0));
}

#[cfg(test)]
pub(crate) fn face_work() -> u64 {
    FACE_WORK.with(|c| c.get())
}

/// Faces the bundle names: star from the Frame (when the form carries one),
/// plus pairs whose after-supports overlap or sit at ring distance one.
/// Occupancy is keyed by cell; related pairs are a Keyed set.
///
/// COST IS THE CELLS, NOT THE BODIES ON THEM. A cell with k occupants
/// contributes k−1 faces, not k(k−1)/2 — see the chain below for why the
/// chords are not faces the complex is missing. Every face this writes is
/// counted through `note_face_work`, so the gauge in
/// `face_work_is_linear_in_occupied_cells` reads the whole cost and not the
/// part that was already linear.
///
/// The supports arrive KEYED BY MEMBER — a vertex AND the chart it stands at
/// there (D10) — which is the coordinate a face is actually between: 21 step 2
/// relates two BODIES, and a vertex covering several bodies on several charts is
/// several occupants of the cells they sit on. Keyed by the vertex alone, the
/// pairs this returns named a relation between two vertices and left which of
/// their bodies met to be guessed by whatever built the face.
///
/// The pairs are member pairs, so the face is created already carrying which
/// member of `i` relates to which member of `j`. NO PRODUCT OF TWO FIBRES is
/// taken anywhere: the occupancy is keyed by CELL and a cell names its
/// occupants, so the chain over `windows(2)` yields incidences the scene has.
///
/// THE MEMBERS ARE READ FROM THE INDEX. This counted `for vi in 0..n` and asked
/// `supports` for each — a walk of the integers with a keyed read inside it,
/// which finds nothing for a vertex that has no support and says nothing about
/// one the index holds and the count does not reach. `supports` IS the member
/// index; walking it is reading it (rule 1).
pub(crate) fn star_and_cell_faces(
    frame: Option<&[u8]>,
    supports: &crate::network::nerve::Keyed<Vec<(i64, i64, i64)>>,
) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut related: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    if let Some(frame) = frame {
        for (mk, _) in supports.scoped() {
            if mk == frame {
                continue;
            }
            related.insert(member_pair_key(frame, &mk), ());
        }
    }
    let mut occ: crate::network::nerve::Keyed<Vec<Vec<u8>>> = crate::network::nerve::Keyed::new();
    for (mk, supp) in supports.scoped() {
        if Some(mk.as_slice()) == frame {
            continue;
        }
        for &(r, c, f) in supp {
            note_face_work();
            let k = support_key(r, c, f);
            match occ.get_mut(&k) {
                // The members ascend in key order and a member is offered its
                // own support once, so a repeat can only be the one just
                // pushed. This was `!v.contains(&id)`, a walk of the cell's
                // occupants on every offer — k² comparisons on the same cell
                // that the chain below was already paying k² for.
                Some(v) => match v.as_slice() {
                    [.., last] if *last == mk => {}
                    _ => v.push(mk.clone()),
                },
                None => {
                    occ.insert(k, vec![mk.clone()]);
                }
            }
        }
    }
    // A CELL'S OCCUPANTS ARE A SIMPLEX, NOT A CLIQUE. Every vertex on this
    // cell meets every other ON IT, so the subcomplex they span is a simplex
    // — contractible, and every cycle inside it is a boundary. This wrote
    // all k(k−1)/2 of those pairs as faces, and `loops_of` builds a cycle
    // basis of the 1-skeleton with no 2-cells to fill them with, so the form
    // carried k(k−1)/2 − (k−1) loops that bound by construction and the
    // pivot drive cloned the whole form once per candidate against each one.
    //
    // Measured on the fleet: 223 bodies on one cell gave a form of 24 920
    // loops, every one of them reported "closed by the tree gauge", and two
    // voices sat on 3 GB at 85% of a core for six minutes proving that
    // boundaries are boundaries. The doc above this claimed "never n²".
    //
    // The chain is that simplex's spanning tree: k−1 faces, the same
    // connectivity and the same components, and the only cycles that survive
    // are the ones spanning DISTINCT cells — which are the cycles that carry
    // a class. The occupants are read in the form's own vertex coordinate,
    // which is content order, so two minds holding one scene chain it alike.
    for (_k, members) in occ.scoped() {
        for w in members.windows(2) {
            let [a, b] = w else { continue };
            note_face_work();
            related.insert(member_pair_key(a, b), ());
        }
    }
    for (mk, supp) in supports.scoped() {
        if Some(mk.as_slice()) == frame {
            continue;
        }
        for (r, c, f) in ring_of_support(supp) {
            note_face_work();
            let Some(others) = occ.get(&support_key(r, c, f)) else {
                continue;
            };
            for u in others {
                if *u != mk {
                    related.insert(member_pair_key(&mk, u), ());
                }
            }
        }
    }
    related
        .scoped()
        .filter_map(|(k, _)| read_member_pair_key(&k))
        .collect()
}

/// Physical size of the hue carrier. Index by hue, never search.
pub const PALETTE: usize = 256;
pub type HueMap = [Option<u8>; PALETTE];

pub(crate) fn hue_map_from_pairs(pairs: &[(u8, u8)]) -> HueMap {
    let mut m = [None; PALETTE];
    for &(f, t) in pairs {
        m[f as usize] = Some(t);
    }
    m
}

/// ONE CANDIDATE BODY of a scene: a perceived body, or a shell of one.
/// `parent` is the ADDRESS of the whole body it came from, so consuming a
/// body consumes its shells without either naming the other by position.
pub(crate) struct Cand {
    pub(crate) body: Body,
    pub(crate) kind: crate::transducer::peel::ShellKind,
    pub(crate) parent: Vec<u8>,
}

/// THE ADDRESS OF A CANDIDATE, as content (A21): the shell kind it came
/// from and its cells written out, length-prefixed. `body_of` has sorted
/// them, so the address is canonical. No digest and no arrival index — a
/// hash would key perception by bytes, and a position would make the
/// scene's order part of its meaning (rule 2, rule 6, rule 15).
pub(crate) fn cand_key(kind: crate::transducer::peel::ShellKind, body: &Body) -> Vec<u8> {
    let mut k = vec![kind.as_u8()];
    k.extend_from_slice(&(body.cells.len() as u32).to_le_bytes());
    for &(r, c, f, h) in &body.cells {
        k.extend_from_slice(&r.to_le_bytes());
        k.extend_from_slice(&c.to_le_bytes());
        k.extend_from_slice(&f.to_le_bytes());
        k.push(h);
    }
    k
}

/// ONE SCENE AS A COMPLEX: its candidates by their own addresses, and the
/// shells each whole body owns, so a consumption reaches them without a
/// scan.
pub(crate) struct Scene {
    pub(crate) cands: crate::network::nerve::Keyed<Cand>,
    pub(crate) by_parent: crate::network::nerve::Keyed<Vec<Vec<u8>>>,
}

impl Scene {
    pub(crate) fn get(&self, addr: &[u8]) -> Option<&Cand> {
        self.cands.get(addr)
    }
}

pub(crate) fn scene_cands(grid: &Grid, scale: Option<((usize, usize), (usize, usize))>) -> Scene {
    let mut cands: crate::network::nerve::Keyed<Cand> = crate::network::nerve::Keyed::new();
    let mut by_parent: crate::network::nerve::Keyed<Vec<Vec<u8>>> =
        crate::network::nerve::Keyed::new();
    for b in perceive(grid) {
        let whole = crate::transducer::peel::ShellKind::Whole;
        let parent = cand_key(whole, &b);
        let mut own = vec![parent.clone()];
        cands.insert(&parent, Cand { body: b.clone(), kind: whole, parent: parent.clone() });
        for shell in crate::transducer::peel::peel_body_scaled(&b.cells, scale) {
            if shell.kind == whole || shell.laminae.is_empty() || shell.laminae == b.cells {
                continue;
            }
            let body = body_of(shell.laminae);
            let k = cand_key(shell.kind, &body);
            own.push(k.clone());
            cands.insert(k, Cand { body, kind: shell.kind, parent: parent.clone() });
        }
        by_parent.insert(parent, own);
    }
    Scene { cands, by_parent }
}

pub(crate) fn pair_rank(ic: &Cand, oc: &Cand) -> Option<(u8, Carriage)> {
    if oc.kind != crate::transducer::peel::ShellKind::Whole {
        return None;
    }
    // Match is ∃V ∈ Spin(4,1)(ℚ) carrying the shape: carriage / versor_between.
    // The linear block need not spell — a parabolic translation is a versor.
    let car = carriage(&ic.body, &oc.body)?;
    let exact = ic.body.moduli == oc.body.moduli;
    let r = match (exact, ic.kind == crate::transducer::peel::ShellKind::Whole) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    Some((r, car))
}

/// Consume a candidate AND every shell of the body it came from: a body is
/// spoken once, whole or by a shell, never twice.
pub(crate) fn consume(used: &mut crate::network::nerve::Keyed<()>, scene: &Scene, addr: &[u8]) {
    let Some(c) = scene.get(addr) else {
        return;
    };
    let Some(siblings) = scene.by_parent.get(&c.parent) else {
        used.insert(addr, ());
        return;
    };
    for k in siblings.clone() {
        used.insert(k, ());
    }
}

/// Partition candidates by exact chart axes (`Invariant` equality), never
/// a digest of those axes. A hash of moduli isolates congruent bodies
/// whose packing or orientation differs; the axes themselves are the chart.
pub(crate) fn group_by_chart(
    scene: &Scene,
    used: Option<&crate::network::nerve::Keyed<()>>,
    peel: bool,
) -> Vec<(Invariant, Vec<Vec<u8>>)> {
    let mut by_key: crate::network::nerve::Keyed<(Invariant, Vec<Vec<u8>>)> =
        crate::network::nerve::Keyed::new();
    for (addr, c) in scene.cands.scoped() {
        if used.is_some_and(|u| u.contains(&addr)) {
            continue;
        }
        let inv = if peel { c.body.moduli.peel() } else { c.body.moduli.clone() };
        let k = inv.key();
        match by_key.get_mut(&k) {
            Some((_, v)) => v.push(addr.clone()),
            None => {
                by_key.insert(k, (inv, vec![addr.clone()]));
            }
        }
    }
    by_key
        .scoped()
        .map(|(_, (inv, ix))| (inv.clone(), ix.clone()))
        .collect()
}

/// ONE PAIR: the two candidates by their own addresses and the carriage
/// taking one to the other.
pub(crate) struct Paired {
    pub(crate) input: Vec<u8>,
    pub(crate) image: Vec<u8>,
    pub(crate) carriage: Carriage,
}

/// THE STATE OF ONE PAIRING PASS: the two scenes, which candidates a pass
/// has consumed, and the pairs it has made. A pass reads all five or none
/// of them — they are the same object at five names.
pub(crate) struct Pairing<'a> {
    pub(crate) ins: &'a Scene,
    pub(crate) outs: &'a Scene,
    pub(crate) used_in: &'a mut crate::network::nerve::Keyed<()>,
    pub(crate) used_out: &'a mut crate::network::nerve::Keyed<()>,
    pub(crate) pairs: &'a mut crate::network::nerve::Keyed<Paired>,
}

pub(crate) fn greedy_within(p: &mut Pairing<'_>, ii: &[Vec<u8>], oo: &[Vec<u8>], want: u8) {
    for i in ii {
        if p.used_in.contains(i) {
            continue;
        }
        let Some(ic) = p.ins.get(i) else { continue };
        // First in the candidates' own KEY order for which a carriage
        // exists. Key order is content order; interchangeable under the
        // class, and no distance is measured to choose.
        let mut found: Option<(Vec<u8>, Carriage)> = None;
        for j in oo {
            if p.used_out.contains(j) {
                continue;
            }
            let Some(oc) = p.outs.get(j) else { continue };
            let Some((r, car)) = pair_rank(ic, oc) else {
                continue;
            };
            if r != want {
                continue;
            }
            found = Some((j.clone(), car));
            break;
        }
        if let Some((j, car)) = found {
            consume(p.used_in, p.ins, i);
            consume(p.used_out, p.outs, &j);
            let key = super::pieces::carriage_key(i, &j);
            p.pairs.insert(key, Paired { input: i.clone(), image: j, carriage: car });
        }
    }
}

pub(crate) fn pair_groups(
    p: &mut Pairing<'_>,
    in_g: &[(Invariant, Vec<Vec<u8>>)],
    out_g: &[(Invariant, Vec<Vec<u8>>)],
    want: u8,
) {
    let mut out_by: crate::network::nerve::Keyed<&[Vec<u8>]> = crate::network::nerve::Keyed::new();
    for (inv, oo) in out_g {
        out_by.insert(inv.key(), oo.as_slice());
    }
    for (inv, ii) in in_g {
        let Some(oo) = out_by.get(&inv.key()).copied() else {
            continue;
        };
        greedy_within(p, ii, oo, want);
    }
}

/// WHAT PAIRING TWO SCENES YIELDS.
pub(crate) struct PairedScene {
    pub(crate) ins: Scene,
    pub(crate) outs: Scene,
    /// The pairs that matched, by the carriage's own address.
    pub(crate) pairs: crate::network::nerve::Keyed<Paired>,
    /// Which input candidates a pass consumed — a keyed SET, not a boolean
    /// per arrival slot. The unconsumed are the residual: what the form
    /// does not speak.
    pub(crate) used_in: crate::network::nerve::Keyed<()>,
    /// Which output candidates a pass consumed.
    pub(crate) used_out: crate::network::nerve::Keyed<()>,
}

pub(crate) fn pair_scene(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> PairedScene {
    let ins = scene_cands(a, scale);
    let outs = scene_cands(b, scale);
    let mut used_in: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut used_out: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut pairs: crate::network::nerve::Keyed<Paired> = crate::network::nerve::Keyed::new();
    let in_exact = group_by_chart(&ins, None, false);
    let out_exact = group_by_chart(&outs, None, false);
    {
        let mut pass = Pairing {
            ins: &ins,
            outs: &outs,
            used_in: &mut used_in,
            used_out: &mut used_out,
            pairs: &mut pairs,
        };
        for want in 0u8..=1 {
            pair_groups(&mut pass, &in_exact, &out_exact, want);
        }
    }
    let in_peel = group_by_chart(&ins, Some(&used_in), true);
    let out_peel = group_by_chart(&outs, Some(&used_out), true);
    {
        let mut pass = Pairing {
            ins: &ins,
            outs: &outs,
            used_in: &mut used_in,
            used_out: &mut used_out,
            pairs: &mut pairs,
        };
        for want in 2u8..=3 {
            pair_groups(&mut pass, &in_peel, &out_peel, want);
        }
    }
    PairedScene { ins, outs, pairs, used_in, used_out }
}
