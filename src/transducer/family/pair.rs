// 21 step 2 — THE PAIR FORM: the scene of one train pair, grouped by chart
// and ranked by carriage.
//
//! `pair_form` is the memoised entry; everything under it reads the scene's
//! own candidates and their carriages. `greedy_within` and `pair_rank` are
//! the two sites the enumeration sweep still watches, and they are here so
//! that watch is one file.

use super::*;
use std::cell::RefCell;

thread_local! {
    static PAIR_FORM_MEMO: RefCell<Option<PairFormMemo>> = const { RefCell::new(None) };
}

pub(crate) struct PairFormMemo {
    a: Grid,
    b: Grid,
    scale: Option<((usize, usize), (usize, usize))>,
    form: Form,
}

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
    body: Body,
    kind: crate::transducer::peel::ShellKind,
    parent: usize,
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

pub(crate) fn pair_form_uncached(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Form {
    let PairedScene { ins, outs, pairs, used_in, used_out } = pair_scene(a, b, scale);
    let mut vertices = vec![frame_vertex()];
    let mut supports: Vec<Vec<(i64, i64)>> = vec![Vec::new()];
    for (i, j, _car) in &pairs {
        if let Some(v) = word_form(&ins[*i].body, &outs[*j].body) {
            vertices.push(v);
            supports.push(support_of(&outs[*j].body));
        }
    }
    for (j, oc) in outs.iter().enumerate() {
        if !used_out[j] && oc.kind == crate::transducer::peel::ShellKind::Whole {
            vertices.push(body_only_vertex(&oc.body, true));
            supports.push(support_of(&oc.body));
        }
    }
    for (i, ic) in ins.iter().enumerate() {
        if !used_in[i] && ic.kind == crate::transducer::peel::ShellKind::Whole {
            vertices.push(body_only_vertex(&ic.body, false));
            supports.push(support_of(&ic.body));
        }
    }
    let mut form = Form {
        vertices,
        faces: Vec::new(),
        loops: Vec::new(),
        remainder: Vec::new(),
        depth: 0,
        hue_maps: crate::network::nerve::Keyed::new(),
        ratio: ((1, 1), (1, 1)),
        sections: crate::network::nerve::Keyed::new(),
        certificates: crate::network::nerve::Keyed::new(),
        torsion: crate::network::nerve::Keyed::new(),
        axes: crate::network::nerve::Keyed::new(),
        glued: crate::network::nerve::Keyed::new(),
        vortex: crate::network::nerve::Keyed::new(),
        by_address: crate::network::nerve::Keyed::new(),
        star: crate::network::nerve::Keyed::new(),
        carriages: crate::network::nerve::Keyed::new(),
        by_rebased: crate::network::nerve::Keyed::new(),
        by_unfolded: crate::network::nerve::Keyed::new(),
    };
    let n = form.vertices.len();
    let frame = 0usize;
    for (i, j) in star_and_cell_faces(n, Some(frame), &supports) {
        let Some(g) = form.face_of(i, j) else {
            continue;
        };
        form.faces.push(FormFace { i, j, transport: g });
    }
    form.loops = Form::loops_of(&form.faces, n);
    form.reindex();
    form
}

/// Perception of one pair as a form. Cells are read once; a second call on
/// the same grids spends none. Shells pair and are never vertices.
pub fn pair_form(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Form {
    let hit = PAIR_FORM_MEMO.with(|m| {
        m.borrow().as_ref().and_then(|held| {
            if held.a == *a && held.b == *b && held.scale == scale {
                Some(held.form.clone())
            } else {
                None
            }
        })
    });
    if let Some(f) = hit {
        return f;
    }
    let form = pair_form_uncached(a, b, scale);
    PAIR_FORM_MEMO.with(|m| {
        *m.borrow_mut() = Some(PairFormMemo {
            a: a.clone(),
            b: b.clone(),
            scale,
            form: form.clone(),
        });
    });
    form
}
