// 21 — SPEAKING a rule: the form applied to a scene, and what it emits.
//
//! `apply_form` carries the rule's vertices onto the scene's bodies and
//! emits the cells they name. A hue no neighbour supplies is refused; a
//! spoken grid that does not grow to the frame is not a speech.

use super::*;

pub(crate) fn grows_to_frame(a: &Grid, b: &Grid) -> bool {
    let (fa, fb) = (frame_of(a), frame_of(b));
    if fa != fb || fa.0 == 0 || fa.1 == 0 {
        return false;
    }
    let ins: Vec<Cell3> = crate::arc::grid_to_cells(a);
    let outs: Vec<Cell3> = crate::arc::grid_to_cells(b);
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
        let ring = crate::transducer::blade::adjacent_ring(&occ);
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
pub(crate) fn output_frame(fx: (usize, usize), ratio: ((usize, usize), (usize, usize))) -> (usize, usize) {
    let ((hn, hd), (wn, wd)) = ratio;
    if hd == 0 || wd == 0 || fx.0 == 0 || fx.1 == 0 || (fx.0 * hn) % hd != 0 || (fx.1 * wn) % wd != 0 {
        return fx;
    }
    (fx.0 * hn / hd, fx.1 * wn / wd)
}

pub(crate) fn emit_carried(
    body: &Body,
    v: &Versor,
    map: &[(u8, u8)],
    fy: (usize, usize),
    cells: &mut Vec<Cell3>,
    remainder: &mut Vec<Cell3>,
) {
    let hue = hue_map_from_pairs(map);
    for &(r, c, k, h) in &body.cells {
        // A hue the map does not move keeps its own colour: that is what
        // the map says about it (rule 11).
        let spoken = match hue[h as usize] {
            Some(t) => t,
            None => h,
        };
        match element::carry_cell(v, r, c, k) {
            // The grid spoken onto is the body's own frame: a cell carried
            // into another frame is not spoken on it, and is remainder.
            Some(image) if image.iter().all(|&(_, _, f)| f == k) => {
                for (i, j, _) in image {
                    if i >= 0 && j >= 0 && (i as usize) < fy.0 && (j as usize) < fy.1 {
                        cells.push((i, j, spoken));
                    }
                }
            }
            _ => remainder.push((r, c, h)),
        }
    }
}

pub(crate) fn hue_from_neighbors(r: i64, c: i64, occ: &crate::network::nerve::Keyed<u8>) -> Option<u8> {
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
    placement: &crate::transducer::alphabet::Word,
    crystal: &dyn Crystal,
) -> Option<Speech> {
    apply_form_on(f, x, placement, crystal, None)
}

/// Speak a form onto a grid of a specific pair of a family: only members
/// active on that pair are spoken.
pub fn apply_form_on(
    f: &Form,
    x: &Grid,
    placement: &crate::transducer::alphabet::Word,
    crystal: &dyn Crystal,
    pair: Option<usize>,
) -> Option<Speech> {
    let fx = frame_of(x);
    let fy = output_frame(fx, f.ratio);
    let bodies = perceive(x);
    let mut atlas = crate::transducer::stalk::Stalks::default();
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
            // ONE VERTEX SPEAKS EVERY BODY IT ACTS ON. 21 step 1: two bodies
            // of different shapes carried by one transport correspond, and
            // each stands on its own chart. A vertex acting on two charts
            // spoke ONE of them while the other body went unreached and
            // unmentioned — on `05f2a901`, one of two, four cells emitted
            // where the pair needs both bodies carried.
            //
            // THE MEMBER IS THE UNIT, so the carriage and the gauge are read
            // at the chart whose bodies they are being applied to. Reading
            // one member's carriage and applying it to another chart's bodies
            // would carry a body by a versor measured on a different one.
            Vertex::Word { derivation, acts, .. } => {
                let tc_key = crate::transducer::family::transport_class(v).map(|tc| tc.key());
                if let (Some(k), Some(pi)) = (&tc_key, pair) {
                    if let Some(only) = f.sections.get(k) {
                        if !only.contains(&(pi as u16)) {
                            unspoken.push(vi as u16);
                            continue;
                        }
                    }
                }
                // The action composes: the placement's rotor times the
                // vertex's own (L14), so a cited vertex speaks its body
                // without resolving a citation. The rotor is the VERTEX's and
                // is one for all its members; only the placement differs.
                let r = placement.rotor().geo(&derivation.rotor());
                // A vertex is unspoken when NO member of it reached a body:
                // a member whose chart this scene carries nothing on is not a
                // fault, it is a chart this scene does not show. Naming that
                // absence as a demand is D8's move, not this one.
                let mut spoke = false;
                for (_, member) in acts.scoped() {
                    if let (Some(k), Some(pi)) = (&tc_key, pair) {
                        let mut mk = k.clone();
                        crate::transducer::address::put_bytes(&mut mk, &member.chart.key());
                        if let Some(mps) = f.sections.get(&mk) {
                            if !mps.contains(&(pi as u16)) {
                                continue;
                            }
                        }
                    }
                    let Some(versor) = element::conformal_of_rotor(&r, &member.carriage) else {
                        continue;
                    };

                    let hits = atlas.fiber(&member.chart);
                    if hits.is_empty() {
                        continue;
                    }
                    spoke = true;
                    for addr in hits {
                        let Ok(raw) = <[u8; 8]>::try_from(addr.as_slice()) else {
                            continue;
                        };
                        let i = u64::from_le_bytes(raw) as usize;
                        if let Some(body) = bodies.get(i) {
                            emit_carried(body, &versor, &member.gauge, fy, &mut cells, &mut remainder);
                        }
                    }
                }
                if !spoke {
                    unspoken.push(vi as u16);
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
            // A REBASED VERTEX SPEAKS EVERY CHART `of` ACTED ON, as a Word
            // vertex does: `by`'s word, the relative at that chart composed with
            // the base's placement, and that chart's own bodies and hue map.
            Vertex::Rebased { by, acts, .. } => {
                let Some(w @ Vertex::Word { derivation, .. }) = f.vertices.get(*by as usize)
                else {
                    unspoken.push(vi as u16);
                    continue;
                };
                // THE BASE'S CARRIAGE AT THE BASE'S OWN CHART. The placement is
                // `t · C_by` and `t` was measured as `C_of,c · C_by⁻¹`, so the
                // member read here is the base's — not the one at this vertex's
                // chart, which is a different body. The base stands at ONE chart
                // (`Form::rebase` refuses otherwise), so the slice pattern reads
                // it and a tape carrying a multi-member base speaks nothing
                // rather than composing against a body it was not measured on.
                let base_charts = w.charts();
                let [base_chart] = base_charts.as_slice() else {
                    unspoken.push(vi as u16);
                    continue;
                };
                let Some(base) = w.acting_on(base_chart) else {
                    unspoken.push(vi as u16);
                    continue;
                };
                let r = placement.rotor().geo(&derivation.rotor());
                let mut spoke = false;
                for (_, member) in acts.scoped() {
                    let composed = member.carriage.geo(&base.carriage);
                    let Some(versor) = element::conformal_of_rotor(&r, &composed) else {
                        continue;
                    };
                    let hits = atlas.fiber(&member.chart);
                    if hits.is_empty() {
                        continue;
                    }
                    spoke = true;
                    for addr in hits {
                        let Ok(raw) = <[u8; 8]>::try_from(addr.as_slice()) else {
                            continue;
                        };
                        let i = u64::from_le_bytes(raw) as usize;
                        if let Some(body) = bodies.get(i) {
                            emit_carried(body, &versor, &member.gauge, fy, &mut cells, &mut remainder);
                        }
                    }
                }
                if !spoke {
                    unspoken.push(vi as u16);
                }
            }
            Vertex::Unfolded {
                measure: crate::transducer::sentence::Measure::Occupied,
            } => {
                unfolded = true;
            }
            Vertex::Unfolded { .. } => {}
            // 50.5: a demand speaks nothing; its bodies are the remainder.
            Vertex::Demand { .. } => {}
        }
    }
    if unfolded {
        if cells.is_empty() {
            cells = bodies.iter().flat_map(|b| b.cells.iter().map(|&(r, c, _, h)| (r, c, h))).collect();
        }
        // A frame whose cell count does not fit the carrier is wider than
        // this speech fills; nothing is unfolded into it.
        let need = fy.0.checked_mul(fy.1)?;
        let mut guard = 0u32;
        while cells.len() < need && guard < need as u32 {
            guard += 1;
            let ring = crate::transducer::blade::adjacent_ring(&cells);
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
    ///
    /// `None` WHERE A CELL DOES NOT FIT THE FRAME. This wrote
    /// `out[r as usize][c as usize] = h` straight from `self.cells` with no
    /// bound on either coordinate, so a transported cell outside the output
    /// rectangle panicked the process: `index out of bounds: the len is 3
    /// but the index is 9`, four times in 47 minutes on the fleet at
    /// 820678a, and every voice death in that window.
    ///
    /// It is not an unexpected state. It is exactly the condition the court
    /// names `FAIL_PAST_FRAME` — a cell landing past the image frame's
    /// extent — which the court refuses and which a rule gets wrong often
    /// enough to be 186 of 488 body refusals. The court refused it and the
    /// voice indexed on it.
    ///
    /// A speech that cannot be laid on its own frame is not a speech, so it
    /// is refused here and every caller decides (rule 11). Dropping the cell
    /// instead would lay down a grid that silently is not what the form
    /// said, which is the fault this engine has spent the day removing.
    pub fn grid(&self) -> Option<Grid> {
        let mut out: Grid = vec![vec![0u8; self.frame.1]; self.frame.0];
        for &(r, c, h) in &self.cells {
            let (Ok(ur), Ok(uc)) = (usize::try_from(r), usize::try_from(c)) else {
                return None;
            };
            *out.get_mut(ur).and_then(|row| row.get_mut(uc))? = h;
        }
        Some(out)
    }
}

/// SPEAK A RULE onto a test input: match bodies to rule germs by chart,
/// instantiate transports from faces and the word's linear block, never
/// from a training pair's absolute versor. Several bodies on one germ is
/// a section; one body on two germs is a section whose face words to
/// already-placed anchors agree. None only when two anchors conflict on
/// the same output cells.
pub fn speak_rule(rule: &crate::transducer::sentence::SentenceRule, x: &Grid, crystal: &dyn Crystal) -> Option<Speech> {
    apply_form(&rule.form, x, &crate::transducer::alphabet::Word::identity(), crystal)
}

/// The residual between what was spoken and what was observed, as a chain
/// (row, col, spoken hue, observed hue) over every cell that differs.
pub(crate) fn residual_of(spoken: Option<&Grid>, observed: &Grid) -> Vec<(i64, i64, u8, u8)> {
    let mut out = Vec::new();
    for (r, row) in observed.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            // A cell the speech does not reach is unoccupied there, and
            // hue 0 IS absence in this encoding.
            let s = match spoken.and_then(|g| g.get(r)).and_then(|row| row.get(c)) {
                Some(h) => *h,
                None => 0,
            };
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
