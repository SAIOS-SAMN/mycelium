// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! FAMILY TRANSPORT — pair forms extracted as a crystallized sentence.
//!
//! A training pair is two perceived scenes. Each output body is the image
//! of an input body under a versor solved from the two bodies' moments and
//! a gauge map read cell by cell. The form engine (`extract_rule_with`) is
//! the only extractor: correspondence by transport class, faces by star and
//! cell occupancy, glue, rebase, unfold, contact, tree gauge. A family it
//! does not close is a withheld form. Cells are spoken by sandwich and read
//! at the sensor (`pair_form`, `speak_rule`, `orbit`). No second extractor,
//! no centroid pairing, no coordinate pair outside the sensor.
//!

use super::blade::{self, Shape};
use super::element::{self, Element, Versor};
use super::sentence::{Form, FormFace, Vertex};
use super::stalk::{Invariant, Standing};
use std::cell::{Cell, RefCell};

/// Source of crystallized forms on a class. Standing is a coordinate on
/// the working atlas, never a gate.
pub trait Crystal {
    fn on_class(&self, class: &Invariant) -> Vec<(Element, Form, Standing)>;
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech>;
    /// Keyed read of a crystallized form by address. No scan.
    fn by_address(&self, addr: &[u8]) -> Option<(Element, Form)>;
    /// Held words on `chart` whose class vector occupies `axis` (12.6): the
    /// vocabulary a pivot may write onto a loop on that chart. One keyed read
    /// of the axis index the crystal filled when the word was admitted.
    fn on_axis(&self, chart: &Invariant, axis: super::sentence::Axis) -> Vec<super::alphabet::Word>;
}

/// A crystal that holds nothing. Glue is a no-op.
pub struct NoCrystal;

impl Crystal for NoCrystal {
    fn on_class(&self, _class: &Invariant) -> Vec<(Element, Form, Standing)> {
        Vec::new()
    }
    fn speak_form(&self, f: &Form, x: &Grid, placement: &super::alphabet::Word) -> Option<Speech> {
        apply_form(f, x, placement, self)
    }
    fn by_address(&self, _addr: &[u8]) -> Option<(Element, Form)> {
        None
    }
    fn on_axis(&self, _chart: &Invariant, _axis: super::sentence::Axis) -> Vec<super::alphabet::Word> {
        Vec::new()
    }
}
pub type Grid = Vec<Vec<u8>>;
type Cell3 = (i64, i64, u8);

/// A cell scene as a grid, origin at the scene's min-corner. The sensor
/// embedding of one scene; not a ranking over candidates.
pub fn cells_to_grid(cells: &[(i64, i64, u8)]) -> Grid {
    let Some(&(r0, c0, _)) = cells.first() else {
        return vec![vec![0]];
    };
    let (mut min_r, mut min_c, mut max_r, mut max_c) = (r0, c0, r0, c0);
    for &(r, c, _) in cells {
        if r < min_r {
            min_r = r;
        }
        if c < min_c {
            min_c = c;
        }
        if r > max_r {
            max_r = r;
        }
        if c > max_c {
            max_c = c;
        }
    }
    let h = (max_r - min_r + 1) as usize;
    let w = (max_c - min_c + 1) as usize;
    let mut g = vec![vec![0u8; w]; h];
    for &(r, c, hue) in cells {
        g[(r - min_r) as usize][(c - min_c) as usize] = hue;
    }
    g
}

#[cfg(test)]
fn q(v: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// A perceived body: its cells with hue, its shape, and its moduli.
#[derive(Clone, Debug)]
pub struct Body {
    pub cells: Vec<Cell3>,
    pub shape: Shape,
    pub moduli: Invariant,
}

fn body_of(comp: Vec<Cell3>) -> Body {
    let mut pos: Vec<(i64, i64)> = comp.iter().map(|&(r, c, _)| (r, c)).collect();
    pos.sort_unstable();
    let shape = blade::shape_of(&pos);
    let moduli = Invariant(blade::moduli(&shape.blade));
    let mut cells = comp;
    cells.sort_unstable();
    Body { cells, shape, moduli }
}

thread_local! {
    static CELL_READS: Cell<u64> = Cell::new(0);
    static PAIR_FORM_MEMO: RefCell<Option<PairFormMemo>> = RefCell::new(None);
}

struct PairFormMemo {
    a: Grid,
    b: Grid,
    scale: Option<((usize, usize), (usize, usize))>,
    form: Form,
}

/// Grid cells visited by [`perceive`] since the last reset.
#[cfg(test)]
pub fn cell_reads() -> u64 {
    CELL_READS.with(|c| c.get())
}

#[cfg(test)]
fn reset_cell_reads() {
    CELL_READS.with(|c| c.set(0));
}

fn note_cell_read() {
    CELL_READS.with(|c| c.set(c.get().saturating_add(1)));
}

/// Perceive a scene: its bodies (connected components of the non-background
/// cells; hue 0 is the background the eye does not see).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    let mut cells: Vec<Cell3> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            note_cell_read();
            if h != 0 {
                cells.push((r as i64, c as i64, h));
            }
        }
    }
    blade::components(&cells).into_iter().map(body_of).collect()
}

/// The scene frame as a form vertex.
pub fn frame_vertex() -> Vertex {
    Vertex::Frame
}

fn spelling_of_versor(v: &Versor) -> Option<super::alphabet::Spelling> {
    let qn = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let at = |r, c| {
        crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&qn(r), &qn(c))))
    };
    let o = at(0, 0)?;
    let er = at(1, 0)?;
    let ec = at(0, 1)?;
    let block = [
        [&er.0 - &o.0, &ec.0 - &o.0],
        [&er.1 - &o.1, &ec.1 - &o.1],
    ];
    super::alphabet::spell(&block)
}

fn chain_of(a: Option<&Body>, b: Option<&Body>, v: &Versor) -> Vec<(i64, i64, u8, u8)> {
    let mut chain = Vec::new();
    match (a, b) {
        (Some(src), Some(dst)) => {
            for &(r, c, h) in &src.cells {
                let h2 = element::carry_center(v, r, c)
                    .and_then(|p| {
                        let (tr, tc) = element::cell_of(&p);
                        dst.cells
                            .binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc)))
                            .ok()
                            .map(|i| dst.cells[i].2)
                    })
                    .unwrap_or(h);
                chain.push((r, c, h, h2));
            }
        }
        (None, Some(dst)) => {
            for &(r, c, h) in &dst.cells {
                chain.push((r, c, 0, h));
            }
        }
        (Some(src), None) => {
            for &(r, c, h) in &src.cells {
                chain.push((r, c, h, 0));
            }
        }
        (None, None) => {}
    }
    chain.sort_unstable();
    chain
}

fn hue_map_of_chain(chain: &[(i64, i64, u8, u8)]) -> Vec<(u8, u8)> {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &(_, _, b, a) in chain {
        seen.insert(super::element::hue_pair_key(b, a), ());
    }
    let mut m: Vec<(u8, u8)> = seen
        .fiber(crate::network::nerve::ROOT)
        .filter_map(|(k, _)| super::element::hue_pair_of(&k))
        .collect();
    m.sort_unstable();
    m
}

/// Carriage of two bodies spelled as a word form. Class is the target's chart.
pub fn word_form(a: &Body, b: &Body) -> Option<Vertex> {
    let car = carriage(a, b)?;
    let sp = spelling_of_versor(&car.v)?;
    let t = element::translator_of(&car.v)?;
    let d = element::dilator_of(&car.v)?;
    let carriage = d.geo(&t);
    let chain = chain_of(Some(a), Some(b), &car.v);
    let gauge = if car.sigma.is_empty() {
        hue_map_of_chain(&chain)
    } else {
        car.sigma
    };
    Some(Vertex::Word {
        address: element::from_residual(&chain).address(),
        word: sp.word,
        carriage,
        gauge,
        class: b.moduli.clone(),
        contact: None,
    })
}

fn body_only_vertex(body: &Body, created: bool) -> Vertex {
    let chain: Vec<(i64, i64, u8, u8)> = body
        .cells
        .iter()
        .map(|&(r, c, h)| if created { (r, c, 0, h) } else { (r, c, h, 0) })
        .collect();
    Vertex::Word {
        address: element::from_residual(&chain).address(),
        word: super::alphabet::Word::identity(),
        carriage: element::identity_versor(),
        gauge: hue_map_of_chain(&chain),
        class: body.moduli.clone(),
        contact: None,
    }
}

fn support_of(body: &Body) -> Vec<(i64, i64)> {
    let mut s: Vec<(i64, i64)> = body.cells.iter().map(|&(r, c, _)| (r, c)).collect();
    s.sort_unstable();
    s.dedup();
    s
}

pub(crate) fn cell_pos_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

pub(crate) fn pair_key(a: u16, b: u16) -> [u8; 32] {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    let mut k = [0u8; 32];
    k[..2].copy_from_slice(&lo.to_le_bytes());
    k[2..4].copy_from_slice(&hi.to_le_bytes());
    k
}

pub(crate) fn decode_pair_key(k: &[u8]) -> Option<(u16, u16)> {
    let lo = u16::from_le_bytes(k.get(..2)?.try_into().ok()?);
    let hi = u16::from_le_bytes(k.get(2..4)?.try_into().ok()?);
    Some((lo, hi))
}

thread_local! {
    static FACE_WORK: Cell<u64> = const { Cell::new(0) };
}

fn note_face_work() {
    FACE_WORK.with(|c| c.set(c.get().saturating_add(1)));
}

#[cfg(test)]
fn reset_face_work() {
    FACE_WORK.with(|c| c.set(0));
}

#[cfg(test)]
fn face_work() -> u64 {
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
        for (r, c, _) in super::blade::adjacent_ring(&cells) {
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

fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), g.first().map_or(0, |r| r.len()))
}

/// The family's frame ratio ((h_num, h_den), (w_num, w_den)), identical
/// across pairs or refused.
pub fn family_ratio(family: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in family {
        let (fa, fb) = (frame_of(a), frame_of(b));
        if fa.0 == 0 || fa.1 == 0 || fb.0 == 0 || fb.1 == 0 {
            return None;
        }
        let (gh, gw) = (gcd(fb.0, fa.0).max(1), gcd(fb.1, fa.1).max(1));
        let r = ((fb.0 / gh, fa.0 / gh), (fb.1 / gw, fa.1 / gw));
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}

/// One body carried onto another: the transport and the gauge map.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub v: Versor,
    pub sigma: Vec<(u8, u8)>,
}

/// Carry `a` onto `b`: the versor from the moments, and the hue map read
/// cell by cell — functional, or refused.
pub fn carriage(a: &Body, b: &Body) -> Option<Carriage> {
    let v = blade::versor_between(&a.shape, &b.shape)?;
    let mut sigma: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
    for &(r, c, h) in &a.cells {
        let p = element::carry_center(&v, r, c)?;
        let (tr, tc) = element::cell_of(&p);
        let h2 = b.cells.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))).ok().map(|i| b.cells[i].2)?;
        let k = super::element::hue_pair_key(h, 0);
        match sigma.get(&k) {
            Some(t) if *t != h2 => return None,
            Some(_) => {}
            None => {
                sigma.insert(k, h2);
            }
        }
    }
    let mut out: Vec<(u8, u8)> = sigma
        .fiber(crate::network::nerve::ROOT)
        .filter_map(|(k, t)| super::element::hue_pair_of(&k).map(|(h, _)| (h, *t)))
        .collect();
    out.sort_unstable();
    Some(Carriage { v, sigma: out })
}

/// Physical size of the hue carrier. Index by hue, never search.
pub const PALETTE: usize = 256;
pub type HueMap = [Option<u8>; PALETTE];

fn hue_map_from_pairs(pairs: &[(u8, u8)]) -> HueMap {
    let mut m = [None; PALETTE];
    for &(f, t) in pairs {
        m[f as usize] = Some(t);
    }
    m
}

struct Cand {
    body: Body,
    kind: super::peel::ShellKind,
    parent: usize,
}

fn scene_cands(grid: &Grid, scale: Option<((usize, usize), (usize, usize))>) -> Vec<Cand> {
    let mut out = Vec::new();
    // index: bodies of this scene; shells are peel fibre, not extra vertices
    for (p, b) in perceive(grid).into_iter().enumerate() {
        out.push(Cand { body: b.clone(), kind: super::peel::ShellKind::Whole, parent: p });
        for shell in super::peel::peel_body_scaled(&b.cells, scale) {
            if shell.kind == super::peel::ShellKind::Whole || shell.cells.is_empty() || shell.cells == b.cells {
                continue;
            }
            out.push(Cand { body: body_of(shell.cells), kind: shell.kind, parent: p });
        }
    }
    out
}



fn pair_rank(ic: &Cand, oc: &Cand) -> Option<(u8, Carriage)> {
    if oc.kind != super::peel::ShellKind::Whole {
        return None;
    }
    // Match is ∃V ∈ Spin(4,1)(ℚ) carrying the shape: carriage / versor_between.
    // The linear block need not spell — a parabolic translation is a versor.
    let car = carriage(&ic.body, &oc.body)?;
    let exact = ic.body.moduli == oc.body.moduli;
    let r = match (exact, ic.kind == super::peel::ShellKind::Whole) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    Some((r, car))
}

fn mark_parent(used: &mut [bool], cands: &[Cand], idx: usize) {
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
fn group_by_chart(cands: &[Cand], used: Option<&[bool]>, peel: bool) -> Vec<(Invariant, Vec<usize>)> {
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

fn greedy_within(
    ins: &[Cand],
    outs: &[Cand],
    ii: &[usize],
    oo: &[usize],
    used_in: &mut [bool],
    used_out: &mut [bool],
    pairs: &mut Vec<(usize, usize, Carriage)>,
    want: u8,
) {
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

fn pair_groups(
    ins: &[Cand],
    outs: &[Cand],
    in_g: &[(Invariant, Vec<usize>)],
    out_g: &[(Invariant, Vec<usize>)],
    used_in: &mut [bool],
    used_out: &mut [bool],
    pairs: &mut Vec<(usize, usize, Carriage)>,
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
        greedy_within(ins, outs, ii, oo, used_in, used_out, pairs, want);
    }
}

fn pair_scene(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> (Vec<Cand>, Vec<Cand>, Vec<(usize, usize, Carriage)>, Vec<bool>, Vec<bool>) {
    let ins = scene_cands(a, scale);
    let outs = scene_cands(b, scale);
    let mut used_in = vec![false; ins.len()];
    let mut used_out = vec![false; outs.len()];
    let mut pairs: Vec<(usize, usize, Carriage)> = Vec::new();
    let in_exact = group_by_chart(&ins, None, false);
    let out_exact = group_by_chart(&outs, None, false);
    for want in 0u8..=1 {
        pair_groups(&ins, &outs, &in_exact, &out_exact, &mut used_in, &mut used_out, &mut pairs, want);
    }
    let in_peel = group_by_chart(&ins, Some(&used_in), true);
    let out_peel = group_by_chart(&outs, Some(&used_out), true);
    for want in 2u8..=3 {
        pair_groups(&ins, &outs, &in_peel, &out_peel, &mut used_in, &mut used_out, &mut pairs, want);
    }
    // Leftover wholes: ∃V even when the chart axes disagree. Not a
    // cartesian of peel shells — unused wholes only.
    let ii: Vec<usize> = ins
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_in[*i] && c.kind == super::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    let oo: Vec<usize> = outs
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_out[*i] && c.kind == super::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    for want in 2u8..=3 {
        greedy_within(&ins, &outs, &ii, &oo, &mut used_in, &mut used_out, &mut pairs, want);
    }
    (ins, outs, pairs, used_in, used_out)
}

fn pair_form_uncached(
    a: &Grid,
    b: &Grid,
    scale: Option<((usize, usize), (usize, usize))>,
) -> Form {
    let (ins, outs, pairs, used_in, used_out) = pair_scene(a, b, scale);
    let mut vertices = vec![frame_vertex()];
    let mut supports: Vec<Vec<(i64, i64)>> = vec![Vec::new()];
    for (i, j, _car) in &pairs {
        if let Some(v) = word_form(&ins[*i].body, &outs[*j].body) {
            vertices.push(v);
            supports.push(support_of(&outs[*j].body));
        }
    }
    for (j, oc) in outs.iter().enumerate() {
        if !used_out[j] && oc.kind == super::peel::ShellKind::Whole {
            vertices.push(body_only_vertex(&oc.body, true));
            supports.push(support_of(&oc.body));
        }
    }
    for (i, ic) in ins.iter().enumerate() {
        if !used_in[i] && ic.kind == super::peel::ShellKind::Whole {
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

/// End of an orbit (chapter 22). `steps` is instance, not the form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrbitEnd {
    Contact { cells: Vec<Cell3>, steps: u32 },
    Torsion { steps: u32 },
    LeftFrame { steps: u32 },
    NoOrbit,
}

fn positions_of(cells: &[Cell3]) -> Vec<(i64, i64)> {
    let mut p: Vec<(i64, i64)> = cells.iter().map(|&(r, c, _)| (r, c)).collect();
    p.sort_unstable();
    p.dedup();
    p
}

fn occupy_keys(cells: &[(i64, i64, u8)]) -> crate::network::nerve::Keyed<()> {
    let mut occ = crate::network::nerve::Keyed::new();
    for &(r, c, _) in cells {
        occ.insert(cell_pos_key(r, c), ());
    }
    occ
}

/// Smallest peel-shell index at which every image cell sits in the
/// `adjacent_ring` expansion of the anchor. Not a metric.
fn ring_dist_peel(image: &[(i64, i64)], anchor: &[(i64, i64)], bound: u32) -> Option<u8> {
    if image.is_empty() || anchor.is_empty() {
        return None;
    }
    let mut have: Vec<(i64, i64, u8)> = anchor.iter().map(|&(r, c)| (r, c, 0)).collect();
    for k in 0..=bound {
        let occ = occupy_keys(&have);
        if image.iter().all(|&(r, c)| occ.contains(&cell_pos_key(r, c))) {
            return Some(k as u8);
        }
        let ring = super::blade::adjacent_ring(&have);
        if ring.is_empty() {
            break;
        }
        have.extend(ring);
    }
    None
}

fn apply_versor_cells(v: &Versor, cells: &[Cell3], frame: (usize, usize)) -> Vec<Cell3> {
    let mut out = Vec::new();
    for &(r, c, h) in cells {
        if let Some(image) = element::carry_cell(v, r, c) {
            for (ir, ic) in image {
                if ir >= 0 && ic >= 0 && (ir as usize) < frame.0 && (ic as usize) < frame.1 {
                    out.push((ir, ic, h));
                } else {
                    out.push((ir, ic, h));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn vertex_versor(v: &Vertex) -> Option<Versor> {
    match v {
        Vertex::Word { word, carriage, .. } => element::conformal_of(word, carriage),
        _ => None,
    }
}

/// Apply a word until the image's ring distance to `anchor` equals the
/// contact ring (or 0). Bound by the frame. `NoOrbit` when the first step
/// does not move the distance.
pub fn orbit(v: &Vertex, x_cells: &[Cell3], anchor: &[(i64, i64)], frame: (usize, usize)) -> OrbitEnd {
    let Some(versor) = vertex_versor(v) else {
        return OrbitEnd::NoOrbit;
    };
    let want = match v {
        Vertex::Word { contact: Some(c), .. } => c.ring,
        _ => 0,
    };
    let bound = (frame.0.saturating_add(frame.1)).max(1) as u32;
    let start = ring_dist_peel(&positions_of(x_cells), anchor, bound);
    let next = apply_versor_cells(&versor, x_cells, frame);
    let d1 = ring_dist_peel(&positions_of(&next), anchor, bound);
    if d1 == start {
        return OrbitEnd::NoOrbit;
    }
    // Canonical form of a placed occupancy: sorted cells as bytes, then
    // blake3. Position is meaning (orbit instance); this is not the
    // translation-free residual address.
    let state_key = |cs: &[Cell3]| -> Vec<u8> {
        let mut cells = cs.to_vec();
        cells.sort_unstable();
        let mut buf = Vec::with_capacity(4 + cells.len().saturating_mul(17));
        buf.extend_from_slice(&(cells.len() as u32).to_le_bytes());
        for &(r, c, h) in &cells {
            buf.extend_from_slice(&r.to_le_bytes());
            buf.extend_from_slice(&c.to_le_bytes());
            buf.push(h);
        }
        buf
    };
    let mut cells = next;
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    seen.insert(state_key(x_cells), ());
    seen.insert(state_key(&cells), ());
    for step in 1..=bound {
        let d = ring_dist_peel(&positions_of(&cells), anchor, bound);
        let left = cells.iter().any(|&(r, c, _)| r < 0 || c < 0 || (r as usize) >= frame.0 || (c as usize) >= frame.1);
        if left {
            return OrbitEnd::LeftFrame { steps: step };
        }
        if d == Some(want) {
            let inside: Vec<Cell3> = cells
                .into_iter()
                .filter(|&(r, c, _)| r >= 0 && c >= 0 && (r as usize) < frame.0 && (c as usize) < frame.1)
                .collect();
            return OrbitEnd::Contact { cells: inside, steps: step };
        }
        let nxt = apply_versor_cells(&versor, &cells, frame);
        let nk = state_key(&nxt);
        if nxt == cells || seen.contains(&nk) {
            return OrbitEnd::Torsion { steps: step };
        }
        seen.insert(nk, ());
        cells = nxt;
    }
    OrbitEnd::LeftFrame { steps: bound }
}

fn cells_subset_of_grid(cells: &[Cell3], g: &Grid) -> bool {
    cells.iter().all(|&(r, c, h)| {
        g.get(r as usize)
            .and_then(|row| row.get(c as usize))
            .copied()
            == Some(h)
    })
}

fn grows_to_frame(a: &Grid, b: &Grid) -> bool {
    let (fa, fb) = (frame_of(a), frame_of(b));
    if fa != fb || fa.0 == 0 || fa.1 == 0 {
        return false;
    }
    let ins: Vec<Cell3> = perceive(a).into_iter().flat_map(|b| b.cells).collect();
    let outs: Vec<Cell3> = perceive(b).into_iter().flat_map(|b| b.cells).collect();
    if ins.is_empty() || outs.len() <= ins.len() {
        return false;
    }
    let target: Vec<(i64, i64)> = {
        let mut t = positions_of(&outs);
        t.sort_unstable();
        t
    };
    let frame_n = fa.0 * fa.1;
    if target.len() != frame_n {
        return false;
    }
    let mut occ: Vec<(i64, i64, u8)> = ins.clone();
    let mut guard = 0u32;
    while occ.len() < frame_n && guard < frame_n as u32 {
        guard += 1;
        let ring = super::blade::adjacent_ring(&occ);
        let keyed = occupy_keys(&occ);
        let mut grew = false;
        for (r, c, _) in ring {
            if r < 0 || c < 0 || (r as usize) >= fa.0 || (c as usize) >= fa.1 {
                continue;
            }
            if keyed.contains(&cell_pos_key(r, c)) {
                continue;
            }
            occ.push((r, c, 0));
            grew = true;
        }
        if !grew {
            return false;
        }
    }
    let mut got = positions_of(&occ);
    got.sort_unstable();
    got == target
}

/// The output frame of a scaled bundle. A ratio that does not divide the
/// input frame exactly, or that is not a ratio at all, leaves the frame
/// where it is; nothing is rounded.
fn output_frame(fx: (usize, usize), ratio: ((usize, usize), (usize, usize))) -> (usize, usize) {
    let ((hn, hd), (wn, wd)) = ratio;
    if hd == 0 || wd == 0 || fx.0 == 0 || fx.1 == 0 || (fx.0 * hn) % hd != 0 || (fx.1 * wn) % wd != 0 {
        return fx;
    }
    (fx.0 * hn / hd, fx.1 * wn / wd)
}

fn emit_carried(
    body: &Body,
    v: &Versor,
    map: &[(u8, u8)],
    fy: (usize, usize),
    cells: &mut Vec<Cell3>,
    remainder: &mut Vec<Cell3>,
) {
    let hue = hue_map_from_pairs(map);
    for &(r, c, h) in &body.cells {
        let spoken = hue[h as usize].unwrap_or(h);
        if let Some(image) = element::carry_cell(v, r, c) {
            for (i, j) in image {
                if i >= 0 && j >= 0 && (i as usize) < fy.0 && (j as usize) < fy.1 {
                    cells.push((i, j, spoken));
                }
            }
        } else {
            remainder.push((r, c, h));
        }
    }
}

fn hue_from_neighbors(r: i64, c: i64, occ: &crate::network::nerve::Keyed<u8>) -> Option<u8> {
    let mut held: Option<u8> = None;
    for (nr, nc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
        let Some(&h) = occ.get(&cell_pos_key(nr, nc)) else {
            continue;
        };
        match held {
            None => held = Some(h),
            Some(old) if old != h => return None,
            _ => {}
        }
    }
    held
}

/// Speak a form onto a grid: each vertex kind emits; the class fiber
/// names the bodies. An empty fiber is unspoken, never every body.
pub fn apply_form(
    f: &Form,
    x: &Grid,
    placement: &super::alphabet::Word,
    crystal: &dyn Crystal,
) -> Option<Speech> {
    let fx = frame_of(x);
    let fy = output_frame(fx, f.ratio);
    let bodies = perceive(x);
    let mut atlas = super::stalk::Stalks::default();
    for (i, body) in bodies.iter().enumerate() {
        atlas.admit(&body.moduli, (i as u64).to_le_bytes());
    }
    let mut cells: Vec<Cell3> = Vec::new();
    let mut remainder: Vec<Cell3> = Vec::new();
    let mut unspoken: Vec<u16> = Vec::new();
    let mut unfolded = false;
    for (vi, v) in f.vertices.iter().enumerate() {
        match v {
            Vertex::Frame => {}
            Vertex::Word {
                word,
                carriage,
                gauge,
                class,
                ..
            } => {
                let w = placement.compose(word);
                let Some(versor) = element::conformal_of(&w, carriage) else {
                    unspoken.push(vi as u16);
                    continue;
                };
                let hits = atlas.fiber(class);
                if hits.is_empty() {
                    unspoken.push(vi as u16);
                    continue;
                }
                for addr in hits {
                    let Ok(raw) = <[u8; 8]>::try_from(addr.as_slice()) else {
                        continue;
                    };
                    let i = u64::from_le_bytes(raw) as usize;
                    if let Some(body) = bodies.get(i) {
                        emit_carried(body, &versor, gauge, fy, &mut cells, &mut remainder);
                    }
                }
            }
            Vertex::Glued {
                address,
                placement: q,
                ..
            } => {
                let (_e, s) = crystal.by_address(address)?;
                let composed = placement.compose(q);
                let sp = crystal.speak_form(&s, x, &composed)?;
                cells.extend(sp.cells);
                remainder.extend(sp.remainder);
            }
            Vertex::Rebased {
                by,
                carriage,
                gauge,
                class,
                ..
            } => {
                // `by`'s word, carriage composed with the relative carriage;
                // the bodies and hue map are the rebased vertex's own.
                let Some(Vertex::Word { word, carriage: c, .. }) = f.vertices.get(*by as usize) else {
                    unspoken.push(vi as u16);
                    continue;
                };
                let composed = carriage.geo(c);
                let Some(versor) = element::conformal_of(&placement.compose(word), &composed) else {
                    unspoken.push(vi as u16);
                    continue;
                };
                let hits = atlas.fiber(class);
                if hits.is_empty() {
                    unspoken.push(vi as u16);
                    continue;
                }
                for addr in hits {
                    let Ok(raw) = <[u8; 8]>::try_from(addr.as_slice()) else {
                        continue;
                    };
                    let i = u64::from_le_bytes(raw) as usize;
                    if let Some(body) = bodies.get(i) {
                        emit_carried(body, &versor, gauge, fy, &mut cells, &mut remainder);
                    }
                }
            }
            Vertex::Unfolded {
                measure: super::sentence::Measure::Occupied,
            } => {
                unfolded = true;
            }
            Vertex::Unfolded { .. } => {}
        }
    }
    if unfolded {
        if cells.is_empty() {
            cells = bodies.iter().flat_map(|b| b.cells.iter().copied()).collect();
        }
        let need = fy.0.saturating_mul(fy.1);
        let mut guard = 0u32;
        while cells.len() < need && guard < need as u32 {
            guard += 1;
            let ring = super::blade::adjacent_ring(&cells);
            let mut hues: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
            for &(r, c, h) in &cells {
                hues.insert(cell_pos_key(r, c), h);
            }
            let mut grew = false;
            for (r, c, _) in ring {
                if r < 0 || c < 0 || (r as usize) >= fy.0 || (c as usize) >= fy.1 {
                    continue;
                }
                if hues.contains(&cell_pos_key(r, c)) {
                    continue;
                }
                match hue_from_neighbors(r, c, &hues) {
                    Some(h) => {
                        cells.push((r, c, h));
                        grew = true;
                    }
                    None => remainder.push((r, c, 0)),
                }
            }
            if !grew {
                break;
            }
        }
    }
    cells.sort_unstable();
    cells.dedup();
    Some(Speech {
        frame: fy,
        cells,
        remainder,
        unspoken,
    })
}

fn class_of_vertex(v: &Vertex) -> Option<Invariant> {
    match v {
        Vertex::Word { class, .. } => Some(class.clone()),
        Vertex::Glued { chart, .. } => Some(chart.clone()),
        _ => None,
    }
}

/// Correspondence class of a vertex: kind and conjugacy of the word.
/// The hue map is a section coordinate on `Form.hue_maps`, never a matching
/// key. The body's moduli stay on the vertex as a section coordinate.
/// Frame matches by kind alone and is not a class.
#[derive(Clone, Debug, PartialEq, Eq)]
struct TransportClass {
    kind: super::peel::ShellKind,
    word_class: Vec<u8>,
}

impl TransportClass {
    fn key(&self) -> Vec<u8> {
        let mut buf = vec![self.kind.as_u8()];
        crate::transducer::a21::put_bytes(&mut buf, &self.word_class);
        buf
    }
}

fn transport_class(v: &Vertex) -> Option<TransportClass> {
    match v {
        Vertex::Word { word, .. } => Some(TransportClass {
            kind: super::peel::ShellKind::Whole,
            word_class: word.class_key()?.into_vec(),
        }),
        _ => None,
    }
}

#[derive(Clone, PartialEq, Eq)]
enum End {
    Frame,
    Class(TransportClass),
}

fn end_of(v: &Vertex) -> Option<End> {
    match v {
        Vertex::Frame => Some(End::Frame),
        Vertex::Word { .. } => Some(End::Class(transport_class(v)?)),
        _ => None,
    }
}

fn one_vertex_of_class<'a>(f: &'a Form, class: &TransportClass) -> Option<&'a Vertex> {
    match_vertex(f, class)
}

fn match_vertex<'a>(f: &'a Form, class: &TransportClass) -> Option<&'a Vertex> {
    f.vertices
        .iter()
        .find(|v| transport_class(v).as_ref() == Some(class))
}

/// The unique relative word between two ends on a pair form, read from
/// that form's named faces. Several instances of a class are a section:
/// every such face must be one M1 vertex.
fn face_word_between(pf: &Form, a: &End, b: &End) -> Option<super::alphabet::Word> {
    let mut held: Option<super::alphabet::Word> = None;
    for f in &pf.faces {
        let ea = end_of(pf.vertices.get(f.i as usize)?)?;
        let eb = end_of(pf.vertices.get(f.j as usize)?)?;
        let (w, ok) = if &ea == a && &eb == b {
            (f.transport.word.clone(), true)
        } else if &ea == b && &eb == a {
            (f.transport.word.inverse(), true)
        } else {
            (super::alphabet::Word::identity(), false)
        };
        if !ok {
            continue;
        }
        match &held {
            None => held = Some(w),
            Some(h) if h != &w => return None,
            _ => {}
        }
    }
    held
}

/// Step 4 of chapter 21: along each named face (i, j) with `w_i` not the
/// identity, `j` is `i` read in `j`'s frame when, on every pair of the
/// bundle, the two bodies carry one linear word and differ only by their
/// relative placement `t = p_j − p_i`. Conjugation by a translation keeps
/// the linear block and moves the centre, so `t · w_i · t⁻¹` is realised
/// by `apply_form` as `w_i` at `p_i + t`; nothing is spelled for `t`.
fn rebase_named_faces(form: &mut Form, pair_forms: &[Form]) {
    let named: Vec<FormFace> = form.faces.clone();
    for face in &named {
        let (i, j) = (face.i as usize, face.j as usize);
        if matches!(form.vertices.get(j), Some(Vertex::Rebased { .. })) {
            continue;
        }
        let (Some(Vertex::Word {
            word: wi,
            carriage: ci,
            ..
        }), Some(Vertex::Word { carriage: cj, .. })) = (
            form.vertices.get(i).cloned(),
            form.vertices.get(j).cloned(),
        ) else {
            continue;
        };
        if wi.is_identity() {
            continue;
        }
        let Some(tc_i) = transport_class(&form.vertices[i]) else {
            continue;
        };
        let Some(tc_j) = transport_class(&form.vertices[j]) else {
            continue;
        };
        let Some(inv) = ci.versor_inverse() else {
            continue;
        };
        let relative = cj.geo(&inv);
        let constant = pair_forms.iter().all(|pf| {
            match (word_place_of_class(pf, &tc_i), word_place_of_class(pf, &tc_j)) {
                (Some((wa, ca)), Some((wb, cb))) => {
                    match ca.versor_inverse() {
                        Some(ainv) => wb == wa && element::same_versor(&cb.geo(&ainv), &relative),
                        None => false,
                    }
                }
                _ => false,
            }
        });
        if constant {
            let _ = form.rebase(face.i, face.j, relative);
        }
    }
    form.reindex();
    for (_, &fi) in form.star.fiber(crate::network::nerve::ROOT) {
        let Some(face) = form.faces.get(fi as usize) else {
            continue;
        };
        let (i, j) = (face.i, face.j);
        if let Some(t) = form.face_of(i, j) {
            form.faces[fi as usize].transport = t;
        }
    }
}

fn word_place_of_class(
    pf: &Form,
    tc: &TransportClass,
) -> Option<(super::alphabet::Word, Versor)> {
    match match_vertex(pf, tc) {
        Some(Vertex::Word { word, carriage, .. }) => Some((word.clone(), carriage.clone())),
        _ => None,
    }
}

fn placement_of_class(pair_forms: &[Form], class: &TransportClass) -> Option<super::alphabet::Word> {
    let mut held: Option<super::alphabet::Word> = None;
    for pf in pair_forms {
        let w = face_word_between(pf, &End::Frame, &End::Class(class.clone()))?;
        match &held {
            None => held = Some(w),
            Some(h) if h != &w => return None,
            _ => {}
        }
    }
    held
}

/// Seven moves on pair forms (chapter 21). `extract_rule` keeps the nerve path.
pub fn extract_rule_with(
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> Option<super::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    let Some(scale) = family_ratio(family) else {
        return None;
    };
    let pair_forms: Vec<Form> = family
        .iter()
        .map(|(a, b)| pair_form(a, b, Some(scale)))
        .collect();
    let n_pairs = pair_forms.len();
    // 1. Correspondence by transport class (kind, word class).
    // The hue map is a section on Form.hue_maps. Moduli stay a section
    // coordinate. Presence is a keyed set of pair indices, never a scan
    // of groups and never a count-as-branch.
    let mut class_pairs: crate::network::nerve::Keyed<Vec<u16>> = crate::network::nerve::Keyed::new();
    let mut class_of: crate::network::nerve::Keyed<TransportClass> = crate::network::nerve::Keyed::new();
    let mut hue_maps: crate::network::nerve::Keyed<Vec<(u8, u8)>> = crate::network::nerve::Keyed::new();
    for (pi, f) in pair_forms.iter().enumerate() {
        let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for v in &f.vertices {
            let Some(c) = transport_class(v) else {
                continue;
            };
            let k = c.key();
            if let Vertex::Word { word, gauge, .. } = v {
                if let Some(hk) = super::sentence::hue_section_key(word, gauge) {
                    if !hue_maps.contains(&hk) {
                        hue_maps.insert(hk, gauge.clone());
                    }
                }
            }
            if seen.contains(&k) {
                continue;
            }
            seen.insert(&k, ());
            match class_pairs.get_mut(&k) {
                Some(ps) => ps.push(pi as u16),
                None => {
                    class_pairs.insert(&k, vec![pi as u16]);
                    class_of.insert(&k, c);
                }
            }
        }
    }
    let mut all_pairs: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for i in 0..n_pairs {
        all_pairs.insert(pair_idx_key(i as u16), ());
    }
    let mut sections: crate::network::nerve::Keyed<Vec<u16>> = crate::network::nerve::Keyed::new();
    let mut rule_vertices = vec![frame_vertex()];
    for (k, pairs) in class_pairs.fiber(crate::network::nerve::ROOT) {
        let Some(class) = class_of.get(&k) else {
            continue;
        };
        let Some(&pi) = pairs.first() else {
            continue;
        };
        let Some(v) = pair_forms
            .get(pi as usize)
            .and_then(|f| one_vertex_of_class(f, class).cloned())
        else {
            continue;
        };
        let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for &p in pairs {
            have.insert(pair_idx_key(p), ());
        }
        let section = all_pairs
            .fiber(crate::network::nerve::ROOT)
            .any(|(pk, _)| !have.contains(&pk));
        if section {
            sections.insert(&k, pairs.clone());
        }
        rule_vertices.push(v);
    }
    let mut form = Form {
        vertices: rule_vertices,
        faces: Vec::new(),
        loops: Vec::new(),
        remainder: Vec::new(),
        depth: 0,
        hue_maps,
        ratio: ((1, 1), (1, 1)),
        sections: crate::network::nerve::Keyed::new(),
        certificates: crate::network::nerve::Keyed::new(),
        torsion: crate::network::nerve::Keyed::new(),
        glued: crate::network::nerve::Keyed::new(),
        vortex: crate::network::nerve::Keyed::new(),
        by_address: crate::network::nerve::Keyed::new(),
        star: crate::network::nerve::Keyed::new(),
        carriages: crate::network::nerve::Keyed::new(),
        by_rebased: crate::network::nerve::Keyed::new(),
        by_unfolded: crate::network::nerve::Keyed::new(),
    };
    // 2. Faces named by the pair forms (star + cell overlap). Constancy is
    // over the pairs where both ends exist; a missing end is not a violation.
    let mut candidates: crate::network::nerve::Keyed<(End, End)> = crate::network::nerve::Keyed::new();
    for pf in &pair_forms {
        for f in &pf.faces {
            let Some(va) = pf.vertices.get(f.i as usize) else {
                continue;
            };
            let Some(vb) = pf.vertices.get(f.j as usize) else {
                continue;
            };
            let Some(ea) = end_of(va) else {
                continue;
            };
            let Some(eb) = end_of(vb) else {
                continue;
            };
            candidates.insert(ends_pair_key(&ea, &eb), (ea, eb));
        }
    }
    let ends = index_ends(&form);
    for (_, (ea, eb)) in candidates.fiber(crate::network::nerve::ROOT) {
        let mut held: Option<super::alphabet::Word> = None;
        let mut constant = true;
        for pf in &pair_forms {
            match face_word_between(pf, &ea, &eb) {
                None => {}
                Some(w) => match &held {
                    None => held = Some(w),
                    Some(h) if h != &w => {
                        constant = false;
                        break;
                    }
                    _ => {}
                },
            }
        }
        if !constant {
            continue;
        }
        if held.is_none() {
            continue;
        }
        let i = ends.get(&end_key(ea)).copied();
        let j = ends.get(&end_key(eb)).copied();
        if let (Some(i), Some(j)) = (i, j) {
            let Some(t) = form.face_of(i, j) else {
                continue;
            };
            form.faces.push(FormFace { i, j, transport: t });
        }
    }
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.reindex();
    // 3. Glue: placement is the relative word Frame → class, constant across the bundle.
    // Crystal is indexed by the section coordinate (moduli); correspondence is transport class.
    let glued_classes: Vec<(TransportClass, Invariant)> = form
        .vertices
        .iter()
        .filter_map(|v| Some((transport_class(v)?, class_of_vertex(v)?)))
        .collect();
    for (tc, moduli) in glued_classes {
        let Some(placement) = placement_of_class(&pair_forms, &tc) else {
            continue;
        };
        for (e, s, _st) in crystal.on_class(&moduli) {
            if !s.is_closed() {
                continue;
            }
            let reproduces = family.iter().all(|(x, y)| {
                crystal
                    .speak_form(&s, x, &placement)
                    .is_some_and(|sp| cells_subset_of_grid(&sp.cells, y))
            });
            if reproduces {
                let _ = form.glue(&s, e.address(), placement.clone(), e.magnitude());
                let mut drop = crate::network::nerve::Keyed::new();
                for (i, v) in form.vertices.iter().enumerate() {
                    if matches!(v, Vertex::Word { .. }) && transport_class(v).as_ref() == Some(&tc)
                    {
                        drop.insert(super::sentence::vertex_ix_key(i as u16), ());
                    }
                }
                form.retract_vertices(&drop);
            }
        }
    }
    // 4. Rebase: w_j = t · w_i · t⁻¹ with t the relative placement, on
    // every pair. Named faces only; Frame is not a conjugator.
    rebase_named_faces(&mut form, &pair_forms);
    // 5. Unfold grow-to-frame.
    if family.iter().all(|(a, b)| grows_to_frame(a, b)) {
        form.unfold(super::sentence::Measure::Occupied);
    }
    // 6. Contact: placement differs across pairs, peel ring to Frame is one value.
    let nverts = form.vertices.len();
    for vi in 0..nverts {
        let Some(tc) = transport_class(&form.vertices[vi]) else {
            continue;
        };
        let Some(Vertex::Word { contact: None, carriage, .. }) = form.vertices.get(vi) else {
            continue;
        };
        let home = carriage.clone();
        let mut rings: Option<u8> = None;
        let mut place_varies = false;
        let mut ring_one = true;
        for (pf, (_x, y)) in pair_forms.iter().zip(family.iter()) {
            if let Some(Vertex::Word { carriage: p, .. }) = match_vertex(pf, &tc) {
                if !element::same_versor(p, &home) {
                    place_varies = true;
                }
            }
            let fy = frame_of(y);
            let outs: Vec<Cell3> = perceive(y).into_iter().flat_map(|b| b.cells).collect();
            let frame_anchor: Vec<(i64, i64)> = (0..fy.1 as i64)
                .map(|c| (0, c))
                .chain((0..fy.1 as i64).map(|c| (fy.0.saturating_sub(1) as i64, c)))
                .chain((0..fy.0 as i64).map(|r| (r, 0)))
                .chain((0..fy.0 as i64).map(|r| (r, fy.1.saturating_sub(1) as i64)))
                .collect();
            let bound = (fy.0.saturating_add(fy.1)) as u32;
            match ring_dist_peel(&positions_of(&outs), &frame_anchor, bound) {
                Some(r) => match rings {
                    None => rings = Some(r),
                    Some(h) if h != r => ring_one = false,
                    _ => {}
                },
                None => ring_one = false,
            }
        }
        if place_varies && ring_one {
            if let Some(r) = rings {
                if let Vertex::Word { contact, carriage, .. } = &mut form.vertices[vi] {
                    *contact = Some(super::alphabet::Contact { anchor: 0, ring: r });
                    *carriage = element::identity_versor();
                }
            }
        }
    }
    // 7. Coherence: tree gauge, else hold the loops.
    if !form.coheres() {
        if let Some(gamma) = form.solve_coboundary() {
            let _ = form.regauge(&gamma);
        }
    }
    form.depth = form.depth_of();
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.ratio = scale;
    form.sections = sections.clone();
    form.hue_maps = hue_maps_of_form(&form);
    Some(super::sentence::rule_of_form(form))
}

pub(crate) fn pair_idx_key(i: u16) -> Vec<u8> {
    let mut k = vec![0u8];
    k.extend_from_slice(&i.to_le_bytes());
    k
}

fn end_key(e: &End) -> Vec<u8> {
    match e {
        End::Frame => vec![0xff],
        End::Class(c) => c.key(),
    }
}

fn ends_pair_key(a: &End, b: &End) -> Vec<u8> {
    let (ka, kb) = (end_key(a), end_key(b));
    let (lo, hi) = if ka <= kb { (ka, kb) } else { (kb, ka) };
    let mut buf = Vec::new();
    crate::transducer::a21::put_bytes(&mut buf, &lo);
    crate::transducer::a21::put_bytes(&mut buf, &hi);
    buf
}

fn index_ends(form: &Form) -> crate::network::nerve::Keyed<u16> {
    let mut idx = crate::network::nerve::Keyed::new();
    for (i, v) in form.vertices.iter().enumerate() {
        if let Some(e) = end_of(v) {
            let k = end_key(&e);
            if !idx.contains(&k) {
                idx.insert(k, i as u16);
            }
        }
    }
    idx
}

fn hue_maps_of_form(form: &Form) -> crate::network::nerve::Keyed<Vec<(u8, u8)>> {
    let mut m = form.hue_maps.clone();
    for v in &form.vertices {
        if let Vertex::Word { word, gauge, .. } = v {
            if let Some(k) = super::sentence::hue_section_key(word, gauge) {
                if !m.contains(&k) {
                    m.insert(k, gauge.clone());
                }
            }
        }
    }
    m
}

/// Re-derive the family's rule and check it agrees with `rule`.
pub fn rule_replays_family(rule: &super::sentence::SentenceRule, family: &[(Grid, Grid)]) -> bool {
    let Some(again) = extract_rule(family) else {
        return false;
    };
    again.form.vertices == rule.form.vertices && again.form.faces == rule.form.faces
}

/// Extract, memoized by family id and working-atlas generation. A hit
/// spends nothing; a miss spends one Budget and stores the rule.
pub fn extract_rule_cached(
    cache: &mut crate::network::nerve::Keyed<(crate::transducer::a21::Address, Option<super::sentence::SentenceRule>)>,
    fam: &[u8],
    family: &[(Grid, Grid)],
    atlas_root: crate::transducer::a21::Address,
    budget: &mut crate::physics::Budget,
    crystal: &dyn Crystal,
) -> Option<super::sentence::SentenceRule> {
    if let Some((g, rule)) = cache.get(fam) {
        if *g == atlas_root {
            return rule.clone();
        }
    }
    if !budget.spend() {
        return cache.get(fam).and_then(|(_, r)| r.clone());
    }
    let rule = extract_rule_with(family, crystal);
    cache.insert(fam, (atlas_root, rule.clone()));
    rule
}

/// Descent a form measures by speaking every train pair through the
/// crystal that resolves its cited forms. Per pair: deflation is
/// `measure_lanes(delta(x, y)) − measure_lanes(delta(spoken, y))`, accepted
/// only if the spoken residual descends on that pair, else the pair
/// contributes zero. Summed over pairs. The per-pair `residual_of(spoken, y)`
/// chains sit beside the sum. A form is spoken only when every remainder
/// is empty; `REFUSE_NO_DESCENT` means the form closes nothing.
pub fn form_descent(
    form: &Form,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> ([u64; 4], Vec<Vec<(i64, i64, u8, u8)>>) {
    if family.is_empty() {
        return ([0; 4], Vec::new());
    }
    let mut pay = [0u64; 4];
    let mut remainders = Vec::with_capacity(family.len());
    for (x, y) in family {
        let before_chain = crate::arc::refine_delta(x, y)
            .map(|(c, _, _)| c)
            .unwrap_or_else(|| residual_of(Some(x), y));
        let spoken = apply_form(form, x, &super::alphabet::Word::identity(), crystal);
        let grid = spoken.as_ref().map(|s| s.grid());
        let after_chain = residual_of(grid.as_ref(), y);
        let before_mu = element::measure_lanes(&before_chain);
        let after_mu = element::measure_lanes(&after_chain);
        if crate::economy::descends(&after_mu, &before_mu) {
            for i in 0..4 {
                pay[i] = pay[i].saturating_add(before_mu[i].saturating_sub(after_mu[i]));
            }
        }
        remainders.push(after_chain);
    }
    (pay, remainders)
}

/// Extract the invariant cochain of a family. The live path is the form
/// engine (`extract_rule_with`); the nerve extractor is retired.
pub fn extract_rule(family: &[(Grid, Grid)]) -> Option<super::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal)
}


/// Mint the family's sentence: μ⃗ = [S, G, T, H] of the closed cochain.
/// The rule as the word a voice speaks and submits: the form's v3 tape
/// carrying the descent it measures over the bundle through the crystal
/// that resolves its citations. Lane 𝒯 is the scene-state span the bundle
/// stood over. A form with no measured descent is not a word.
pub fn mint_sentence(
    rule: &super::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
) -> Option<element::Element> {
    let (mag, remainders) = form_descent(&rule.form, family, crystal);
    if remainders.iter().any(|r| !r.is_empty()) {
        return None;
    }
    super::sentence::as_element_form(&rule.form, mag)
}

/// What a form says about a new input: the spoken set of cells on the
/// output frame, and the remainder — input cells the form does not speak.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speech {
    pub frame: (usize, usize),
    pub cells: Vec<Cell3>,
    pub remainder: Vec<Cell3>,
    /// Vertex indices whose class fiber was empty on this grid.
    pub unspoken: Vec<u16>,
}

impl Speech {
    /// The wire form: the spoken set laid on the frame's rectangle. Only
    /// here does a rectangle exist; an unspoken cell reads as the ground.
    pub fn grid(&self) -> Grid {
        let mut out: Grid = vec![vec![0u8; self.frame.1]; self.frame.0];
        for &(r, c, h) in &self.cells {
            out[r as usize][c as usize] = h;
        }
        out
    }
}

/// SPEAK A RULE onto a test input: match bodies to rule germs by chart,
/// instantiate transports from faces and the word's linear block, never
/// from a training pair's absolute versor. Several bodies on one germ is
/// a section; one body on two germs is a section whose face words to
/// already-placed anchors agree. None only when two anchors conflict on
/// the same output cells.
pub fn speak_rule(rule: &super::sentence::SentenceRule, x: &Grid, crystal: &dyn Crystal) -> Option<Speech> {
    apply_form(&rule.form, x, &super::alphabet::Word::identity(), crystal)
}

/// The residual between what was spoken and what was observed, as a chain
/// (row, col, spoken hue, observed hue) over every cell that differs.
pub(crate) fn residual_of(spoken: Option<&Grid>, observed: &Grid) -> Vec<(i64, i64, u8, u8)> {
    let mut out = Vec::new();
    for (r, row) in observed.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            let s = spoken.and_then(|g| g.get(r)).and_then(|row| row.get(c)).copied().unwrap_or(0);
            if s != h {
                out.push((r as i64, c as i64, s, h));
            }
        }
    }
    if let Some(g) = spoken {
        for (r, row) in g.iter().enumerate() {
            for (c, &s) in row.iter().enumerate() {
                let inside = observed.get(r).and_then(|row| row.get(c)).is_some();
                if !inside && s != 0 {
                    out.push((r as i64, c as i64, s, 0));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Pressure of a form spoken over its germinating bundle (A1, 12.6):
/// per pair, `measure_lanes` of that pair's own residual; over the bundle,
/// the lane-wise sum (the pairs are disjoint scenes). T is the extent
/// held, passed in. None when there is no bundle to measure.
pub fn bundle_pressure(
    form: &Form,
    bundle: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    t_extent: u64,
) -> Option<(Vec<[u64; 4]>, [u64; 4])> {
    if bundle.is_empty() {
        return None;
    }
    let mut pairs = Vec::with_capacity(bundle.len());
    let mut sum = [0u64; 4];
    for (x, y) in bundle {
        let spoken = apply_form(form, x, &super::alphabet::Word::identity(), crystal);
        let grid = spoken.as_ref().map(|s| s.grid());
        let residual = residual_of(grid.as_ref(), y);
        let m = element::measure_lanes(&residual);
        for l in 0..4 {
            sum[l] = sum[l].saturating_add(m[l]);
        }
        pairs.push(m);
    }
    sum[2] = t_extent;
    Some((pairs, sum))
}

/// A candidate is refused if it raises any lane on any pair, even when the
/// summed μ⃗ descends. Equal or `descends` on every pair.
pub(crate) fn pairs_descend(before: &[[u64; 4]], after: &[[u64; 4]]) -> bool {
    if before.len() != after.len() {
        return false;
    }
    before
        .iter()
        .zip(after)
        .all(|(b, a)| *a == *b || crate::economy::descends(a, b))
}


#[cfg(test)]
mod tests;
