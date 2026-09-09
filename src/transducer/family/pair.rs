// 21 step 2 — THE PAIR FORM: the scene of one train pair, grouped by chart
// and ranked by carriage.
//
//! `pair_form` is the memoised entry: the scene's candidate bodies grouped
//! by exact chart, paired where a carriage exists, and the unpaired left as
//! the residual the form does not speak.

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

/// THE DEMANDS OF THE UNPAIRED (50.5): an output body no pairing consumed
/// is a `Source` when input charts carry onto it (every one standing,
/// their number the multiplicity) and a `Creation` when none does; an
/// input body no pairing consumed is a `TopologicalDeformation` toward
/// each unpaired output body its support overlaps, on the axes their
/// charts differ, and toward the empty chart on every axis when nothing
/// overlaps it — it vanished.
fn unpaired_demands(ins: &[Cand], outs: &[Cand], used_in: &[bool], used_out: &[bool]) -> Vec<Demanded> {
    use crate::transducer::peel::ShellKind::Whole;
    use crate::transducer::sentence::Demand;
    let mut out = Vec::new();
    // The unpaired input bodies first: what each overlaps decides its
    // reading. An output body of the SAME chart it overlaps is its image
    // in place — carried on the shape, its hues not a map (a section over
    // the same base, 25): the input is the SOURCE of that output. An
    // output body of another chart it overlaps is the image of its
    // DEFORMATION, and is consumed as such. Nothing overlapping: vanished.
    let mut sourced_in: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut imaged_out: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for (i, ic) in ins.iter().enumerate() {
        if used_in[i] || ic.kind != Whole {
            continue;
        }
        let mut same: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut images: crate::network::nerve::Keyed<(Invariant, usize)> = crate::network::nerve::Keyed::new();
        for (j, oc) in outs.iter().enumerate() {
            if used_out[j] || oc.kind != Whole || !supports_overlap(&ic.body, &oc.body) {
                continue;
            }
            if differing_axes(&ic.body.moduli, &oc.body.moduli).is_empty() {
                same.insert(j.to_le_bytes(), ());
            } else {
                images.insert(oc.body.moduli.key(), (oc.body.moduli.clone(), j));
            }
        }
        if same.fiber(crate::network::nerve::ROOT).next().is_some() {
            sourced_in.insert(i.to_le_bytes(), ());
            continue;
        }
        match images.fiber(crate::network::nerve::ROOT).next() {
            None => {
                let axes: Vec<u16> = (0..ic.body.moduli.0.len() as u16).collect();
                let cells: Vec<_> = ic.body.cells.iter().map(|&(r, c, h)| (r, c, h, 0u8)).collect();
                out.push((Demand::TopologicalDeformation { image: Invariant(Vec::new()), axes }, ic.body.moduli.clone(), cells));
            }
            Some(_) => {
                for (_, (image, j)) in images.fiber(crate::network::nerve::ROOT) {
                    imaged_out.insert(j.to_le_bytes(), ());
                    let axes = differing_axes(&ic.body.moduli, image);
                    let cells = delta_cells(&ic.body, &outs[*j].body);
                    out.push((Demand::TopologicalDeformation { image: image.clone(), axes }, ic.body.moduli.clone(), cells));
                }
            }
        }
    }
    // The unpaired output bodies no deformation imaged: a SOURCE when input
    // charts carry onto it by shape (every one standing, their number the
    // multiplicity; an in-place source's hues ride the cells), a CREATION
    // when none does.
    for (j, oc) in outs.iter().enumerate() {
        if used_out[j] || oc.kind != Whole || imaged_out.contains(&j.to_le_bytes()) {
            continue;
        }
        let mut charts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut cells: Vec<crate::transducer::sentence::Cell> = Vec::new();
        for (i, ic) in ins.iter().enumerate() {
            if ic.kind != Whole {
                continue;
            }
            let Some(v) = blade::versor_between(&ic.body.shape, &oc.body.shape) else {
                continue;
            };
            charts.insert(ic.body.moduli.key(), ());
            if sourced_in.contains(&i.to_le_bytes()) && supports_overlap(&ic.body, &oc.body) {
                cells.extend(chain_of(Some(&ic.body), Some(&oc.body), &v));
            }
        }
        if cells.is_empty() {
            cells = oc.body.cells.iter().map(|&(r, c, h)| (r, c, 0u8, h)).collect();
        }
        let kind = match charts.fiber(crate::network::nerve::ROOT).next() {
            None => Demand::Creation,
            Some(_) => Demand::Source { multiplicity: charts.len() as u32 },
        };
        out.push((kind, oc.body.moduli.clone(), cells));
    }
    out
}

/// The cells a deformation stands for: the union of both bodies' supports,
/// each with the hue before and after (0 where a body has no cell).
fn delta_cells(a: &Body, b: &Body) -> Vec<crate::transducer::sentence::Cell> {
    let mut before: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, h) in &a.cells {
        before.insert(cell_pos_key(r, c), h);
    }
    let mut after: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, h) in &b.cells {
        after.insert(cell_pos_key(r, c), h);
    }
    let mut cells: Vec<crate::transducer::sentence::Cell> = Vec::new();
    for &(r, c, h) in &a.cells {
        cells.push((r, c, h, after.get(&cell_pos_key(r, c)).copied().unwrap_or(0)));
    }
    for &(r, c, h) in &b.cells {
        if !before.contains(&cell_pos_key(r, c)) {
            cells.push((r, c, 0, h));
        }
    }
    cells.sort_unstable();
    cells.dedup();
    cells
}

pub(crate) fn pair_form_uncached(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Form {
    let PairedScene { ins, outs, pairs, used_in, used_out } = pair_scene(a, b, scale);
    let mut vertices = vec![frame_vertex()];
    let mut supports: Vec<Vec<(i64, i64)>> = vec![Vec::new()];
    // Per vertex, the pair it carries and the versor that carried it (50.3):
    // the Frame and the demand vertices carry none.
    let mut carried: Vec<Option<(usize, usize, Versor)>> = vec![None];
    let mut demanded: Vec<Demanded> = Vec::new();
    for (i, j, car) in &pairs {
        match word_or_demand(&ins[*i].body, &outs[*j].body) {
            Some(Ok(v)) => {
                vertices.push(v);
                supports.push(support_of(&outs[*j].body));
                carried.push(Some((*i, *j, car.v.clone())));
            }
            Some(Err(d)) => demanded.push(d),
            None => {}
        }
    }
    demanded.extend(unpaired_demands(&ins, &outs, &used_in, &used_out));
    // One demand vertex per (kind, chart) (50.5), the bodies it stands for
    // gathered under it; key order, never arrival order.
    let mut by_demand: crate::network::nerve::Keyed<Demanded> = crate::network::nerve::Keyed::new();
    for (kind, chart, cells) in demanded {
        let key = crate::transducer::sentence::demand_key(&kind, &chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend(cells),
            None => {
                by_demand.insert(key, (kind, chart, cells));
            }
        }
    }
    for (_, (kind, chart, cells)) in by_demand.fiber(crate::network::nerve::ROOT) {
        let mut cells = cells.clone();
        cells.sort_unstable();
        cells.dedup();
        let mut support: Vec<(i64, i64)> = cells.iter().map(|&(r, c, _, _)| (r, c)).collect();
        support.sort_unstable();
        support.dedup();
        vertices.push(demand_vertex(kind.clone(), chart, cells));
        supports.push(support);
        carried.push(None);
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
        demands: crate::network::nerve::Keyed::new(),
    };
    let n = form.vertices.len();
    let frame = 0usize;
    for (i, j) in star_and_cell_faces(n, Some(frame), &supports) {
        let Some(g) = form.face_of(i, j) else {
            continue;
        };
        // The edge transport between two carried bodies (50.3); a face to
        // the Frame or to an uncarried body carries none.
        let edge = match (carried.get(i as usize), carried.get(j as usize)) {
            (Some(Some((ai, bi, v_i))), Some(Some((aj, bj, _)))) => {
                edge_transport(v_i, &ins[*ai].body, &ins[*aj].body, &outs[*bi].body, &outs[*bj].body)
            }
            _ => None,
        };
        form.faces.push(FormFace { i, j, transport: g, edge });
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
