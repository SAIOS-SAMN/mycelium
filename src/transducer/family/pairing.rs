// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 21 step 2 — THE PAIRING PASS: the scene's candidate bodies, the charts
//! they group under, the carriage that pairs one with another, and the
//! faces a bundle's supports name.
//!
//! Split from `pair.rs` at the §1.1 600-line reading bound; the items are
//! moved, not rewritten (§1.6). What stays there is what the pairing
//! leaves behind — the demands of the unpaired and the form they become.

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
/// Occupancy is keyed by cell; related pairs are a Keyed set. Cost is
/// occupied + ring cells, never n².
pub(crate) fn star_and_cell_faces(
    n: usize,
    frame: Option<usize>,
    supports: &[Vec<(i64, i64)>],
) -> Vec<(u16, u16)> {
    let mut related: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    if let Some(frame) = frame {
        for j in 0..n {
            if j == frame {
                continue;
            }
            related.insert(pair_key(frame as u16, j as u16), ());
        }
    }
    let mut occ: crate::network::nerve::Keyed<Vec<u16>> = crate::network::nerve::Keyed::new();
    for (vi, supp) in supports.iter().enumerate() {
        if Some(vi) == frame {
            continue;
        }
        let id = vi as u16;
        for &(r, c) in supp {
            note_face_work();
            let k = cell_pos_key(r, c);
            match occ.get_mut(&k) {
                Some(v) => {
                    if !v.contains(&id) {
                        v.push(id);
                    }
                }
                None => {
                    occ.insert(k, vec![id]);
                }
            }
        }
    }
    for (_k, verts) in occ.fiber(crate::network::nerve::ROOT) {
        if verts.len() < 2 {
            continue;
        }
        // Unordered pairs of this cell's own vertices, bounded by the cell.
        for a in 0..verts.len() {
            for b in (a + 1)..verts.len() {
                related.insert(pair_key(verts[a], verts[b]), ());
            }
        }
    }
    for (vi, supp) in supports.iter().enumerate() {
        if Some(vi) == frame {
            continue;
        }
        let v = vi as u16;
        let cells: Vec<(i64, i64, u8)> = supp.iter().map(|&(r, c)| (r, c, 0)).collect();
        for (r, c, _) in crate::transducer::blade::adjacent_ring(&cells) {
            note_face_work();
            let Some(others) = occ.get(&cell_pos_key(r, c)) else {
                continue;
            };
            for &u in others {
                if u != v {
                    related.insert(pair_key(v, u), ());
                }
            }
        }
    }
    related
        .fiber(crate::network::nerve::ROOT)
        .filter_map(|(k, _)| decode_pair_key(&k))
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

pub(crate) struct Cand {
    pub(crate) body: Body,
    pub(crate) kind: crate::transducer::peel::ShellKind,
    pub(crate) parent: usize,
}

pub(crate) fn scene_cands(grid: &Grid, scale: Option<((usize, usize), (usize, usize))>) -> Vec<Cand> {
    let mut out = Vec::new();
    // index: bodies of this scene; shells are peel fibre, not extra vertices
    for (p, b) in perceive(grid).into_iter().enumerate() {
        out.push(Cand { body: b.clone(), kind: crate::transducer::peel::ShellKind::Whole, parent: p });
        for shell in crate::transducer::peel::peel_body_scaled(&b.cells, scale) {
            if shell.kind == crate::transducer::peel::ShellKind::Whole || shell.cells.is_empty() || shell.cells == b.cells {
                continue;
            }
            out.push(Cand { body: body_of(shell.cells), kind: shell.kind, parent: p });
        }
    }
    out
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

pub(crate) fn mark_parent(used: &mut [bool], cands: &[Cand], idx: usize) {
    let p = cands[idx].parent;
    for (k, c) in cands.iter().enumerate() {
        if c.parent == p {
            used[k] = true;
        }
    }
}

/// Partition candidates by exact chart axes (`Invariant` equality), never
/// a digest of those axes. A hash of moduli isolates congruent bodies
/// whose packing or orientation differs; the axes themselves are the chart.
pub(crate) fn group_by_chart(cands: &[Cand], used: Option<&[bool]>, peel: bool) -> Vec<(Invariant, Vec<usize>)> {
    let mut by_key: crate::network::nerve::Keyed<(Invariant, Vec<usize>)> =
        crate::network::nerve::Keyed::new();
    for (i, c) in cands.iter().enumerate() {
        if used.is_some_and(|u| u[i]) {
            continue;
        }
        let inv = if peel { c.body.moduli.peel() } else { c.body.moduli.clone() };
        let k = inv.key();
        match by_key.get_mut(&k) {
            Some((_, v)) => v.push(i),
            None => {
                by_key.insert(k, (inv, vec![i]));
            }
        }
    }
    by_key
        .fiber(crate::network::nerve::ROOT)
        .map(|(_, (inv, ix))| (inv.clone(), ix.clone()))
        .collect()
}

/// THE STATE OF ONE PAIRING PASS: the candidates on each side, which of
/// them a pass has consumed, and the pairs it has made. A pass reads all
/// five or none of them — they are the same object at five names.
pub(crate) struct Pairing<'a> {
    pub(crate) ins: &'a [Cand],
    pub(crate) outs: &'a [Cand],
    pub(crate) used_in: &'a mut [bool],
    pub(crate) used_out: &'a mut [bool],
    pub(crate) pairs: &'a mut Vec<(usize, usize, Carriage)>,
}

pub(crate) fn greedy_within(p: &mut Pairing<'_>, ii: &[usize], oo: &[usize], want: u8) {
    let Pairing { ins, outs, used_in, used_out, pairs } = p;
    for &i in ii {
        if used_in[i] {
            continue;
        }
        // First in the support's own order for which a carriage exists.
        // Interchangeable under the class; no distance is measured to choose.
        let mut found: Option<(usize, Carriage)> = None;
        for &j in oo {
            if used_out[j] {
                continue;
            }
            let Some((r, car)) = pair_rank(&ins[i], &outs[j]) else {
                continue;
            };
            if r != want {
                continue;
            }
            found = Some((j, car));
            break;
        }
        if let Some((j, car)) = found {
            mark_parent(used_in, ins, i);
            mark_parent(used_out, outs, j);
            pairs.push((i, j, car));
        }
    }
}

pub(crate) fn pair_groups(
    p: &mut Pairing<'_>,
    in_g: &[(Invariant, Vec<usize>)],
    out_g: &[(Invariant, Vec<usize>)],
    want: u8,
) {
    let mut out_by: crate::network::nerve::Keyed<&[usize]> = crate::network::nerve::Keyed::new();
    for (inv, oo) in out_g {
        out_by.insert(inv.key(), oo.as_slice());
    }
    for (inv, ii) in in_g {
        let Some(oo) = out_by.get(&inv.key()) else {
            continue;
        };
        greedy_within(p, ii, oo, want);
    }
}

/// WHAT PAIRING TWO SCENES YIELDS.
pub(crate) struct PairedScene {
    /// The candidate bodies read off the input scene.
    pub(crate) ins: Vec<Cand>,
    /// The candidate bodies read off the output scene.
    pub(crate) outs: Vec<Cand>,
    /// For each pair that matched: the input candidate, the output
    /// candidate, and the carriage taking one to the other.
    pub(crate) pairs: Vec<(usize, usize, Carriage)>,
    /// Which input candidates a pass consumed. The UNCONSUMED are the
    /// residual — what the form does not speak.
    pub(crate) used_in: Vec<bool>,
    /// Which output candidates a pass consumed.
    pub(crate) used_out: Vec<bool>,
}

pub(crate) fn pair_scene(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> PairedScene {
    let ins = scene_cands(a, scale);
    let outs = scene_cands(b, scale);
    let mut used_in = vec![false; ins.len()];
    let mut used_out = vec![false; outs.len()];
    let mut pairs: Vec<(usize, usize, Carriage)> = Vec::new();
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
    // Leftover wholes: ∃V even when the chart axes disagree. Not a
    // cartesian of peel shells — unused wholes only.
    let ii: Vec<usize> = ins
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_in[*i] && c.kind == crate::transducer::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    let oo: Vec<usize> = outs
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_out[*i] && c.kind == crate::transducer::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    {
        let mut pass = Pairing {
            ins: &ins,
            outs: &outs,
            used_in: &mut used_in,
            used_out: &mut used_out,
            pairs: &mut pairs,
        };
        for want in 2u8..=3 {
            greedy_within(&mut pass, &ii, &oo, want);
        }
    }
    PairedScene { ins, outs, pairs, used_in, used_out }
}
