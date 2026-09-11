// 21 step 2 — THE PAIR FORM: the scene of one train pair, grouped by chart
// and ranked by carriage.
//
//! `pair_form` is the memoised entry: the scene's candidate bodies grouped
//! by exact chart, paired where a carriage exists, and the unpaired left as
//! the residual the form does not speak.

use super::pairing::{Cand, PairedScene};
use crate::transducer::sentence::vertex_ix_key;
use super::*;

/// THE DEMANDS OF THE UNPAIRED (50.5): an output body no pairing consumed
/// is a `Source` when input charts carry onto it (every one standing,
/// their number the multiplicity) and a `Creation` when none does; an
/// input body no pairing consumed is a `TopologicalDeformation` toward
/// each unpaired output body its support overlaps, on the axes their
/// charts differ, and toward the empty chart on every axis when nothing
/// overlaps it — it vanished.
/// ONE VERTEX OF A PAIR FORM AS IT IS BUILT: the vertex, the support it
/// stands on, and what carried it.
///
/// THE THREE WERE THREE PARALLEL ARRAYS, index-aligned by discipline and
/// by nothing else. Every site that added a vertex had to remember to push
/// to all three, and R2 added two such sites; the first bug that produced
/// was a vertex whose "no pair" had to be ENCODED as `usize::MAX` because
/// there was no way to have a vertex without an entry. One struct, one
/// push, and absence is absence.
pub(crate) struct Placing {
    pub(crate) vertex: Vertex,
    pub(crate) support: Vec<(i64, i64)>,
    /// The versor that carried it and the two scene bodies it was carried
    /// between, where there are two. A cover piece is a piece of ONE body
    /// and names no pair, so 50.3's edge transport — the stabilizer
    /// element a pair's rigid motion picks out — is not asked for it.
    pub(crate) carried: Option<Carried>,
}

/// WHAT CARRIED ONE VERTEX (50.3).
pub(crate) struct Carried {
    pub(crate) pair: Option<(usize, usize)>,
    pub(crate) versor: Versor,
}

/// A DEFORMATION AND THE COVER THAT READ IT. The two travel together
/// because the demand's fate is not decided here: whether it dissolves
/// depends on what the FORM reads off the cover's loops, and the form is
/// built by the caller. `unpaired_demands` used to decide it on the
/// cover's own classification, which is the reading L23(c) moves.
pub(crate) struct Covered {
    pub(crate) demand: Demanded,
    pub(crate) cover: crate::transducer::family::cover::Cover,
}

fn unpaired_demands(
    ins: &[Cand],
    outs: &[Cand],
    used_in: &[bool],
    used_out: &[bool],
    scale: Option<((usize, usize), (usize, usize))>,
    budget: &mut crate::physics::Budget,
    covered: &mut Vec<Covered>,
) -> Vec<Demanded> {
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
                    // 50.8, L11c — THE COVER IS BUILT BEFORE THE DEMAND IS
                    // TYPED. No single versor carries this body onto its
                    // image, but its PIECES may each be carried; what the
                    // pieces leave at their meets is the class the form
                    // files, and the demand stands only on what the cover
                    // did not reach.
                    // THE COVER IS ATOMIC BY ROLLBACK OF ITS RESULT, and
                    // it spends the CALLER'S budget. A cover that does not
                    // reach the end of its pieces contributes nothing —
                    // `Unmeasured` has no fields to read — and the demand
                    // then stands exactly as F typed it, which is what it
                    // did before any cover existed.
                    //
                    // NO SHARE AND NO RATIO: `cover_share` and
                    // `Budget::halve` are both struck, and `physics/budget`
                    // records why so a third is not written. The cost
                    // belongs in `exhausted` and `unmeasured` where it can
                    // be read; the cursor is what turns it into strokes
                    // (L20).
                    let read = crate::transducer::family::cover::cover_deformation(
                        &ic.body,
                        &outs[*j].body,
                        scale,
                        budget,
                    )
                    .ok();
                    // A cover that did not complete files nothing. Filing
                    // what a half-run cover happened to reach would put a
                    // class on the form that no measurement stands behind
                    // (rule 11).
                    let axes = differing_axes(&ic.body.moduli, image);
                    let cells = delta_cells(&ic.body, &outs[*j].body);
                    let demand = (
                        Demand::TopologicalDeformation { image: image.clone(), axes },
                        ic.body.moduli.clone(),
                        cells,
                    );
                    let Some(cover) = read else {
                        // An UNMEASURED cover leaves the demand exactly as
                        // F typed it, which is what it was before any cover
                        // existed (rule 11).
                        out.push(demand);
                        continue;
                    };
                    // A PIECE THE COVER CARRIED AND COULD NOT SPELL is
                    // 37.7's condition with the carriage already confirmed,
                    // and it enters the form as the demand the peel typed.
                    for (_piece, d) in cover.refused.fiber(crate::network::nerve::ROOT) {
                        out.push(d.clone());
                    }
                    // The demand's fate is the FORM's to decide: it
                    // dissolves when every cell of both bodies is carried
                    // AND no loop over the carried pieces stands (50.8).
                    // The second half is a reading of the form's loops and
                    // cannot be taken here.
                    covered.push(Covered { demand, cover });
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
    budget: &mut crate::physics::Budget,
) -> Form {
    let PairedScene { ins, outs, pairs, used_in, used_out } = pair_scene(a, b, scale);
    let mut placing = vec![Placing {
        vertex: frame_vertex(),
        support: Vec::new(),
        carried: None,
    }];
    let mut demanded: Vec<Demanded> = Vec::new();
    for (i, j, car) in &pairs {
        match word_or_demand(&ins[*i].body, &outs[*j].body) {
            Some(Ok(v)) => {
                placing.push(Placing {
                    vertex: v,
                    support: support_of(&outs[*j].body),
                    carried: Some(Carried { pair: Some((*i, *j)), versor: car.v.clone() }),
                });
            }
            Some(Err(d)) => demanded.push(d),
            None => {}
        }
    }
    let mut covers: Vec<Covered> = Vec::new();
    demanded.extend(unpaired_demands(&ins, &outs, &used_in, &used_out, scale, budget, &mut covers));
    // 50.8, L23(c) — THE COVER'S PIECES ARE VERTICES OF THIS FORM. Each
    // carried piece is a word vertex like any other, at the support it
    // landed on; the cover's meets are the faces between them; and the
    // class of a cycle of those faces is the form's loop holonomy, read
    // once over the whole cycle rather than edge by edge inside the cover.
    //
    // The cover used to classify its own meets into carriers nothing read,
    // so 161 carriages and 3 964 meets over twelve tasks reached no form
    // and left only a tally.
    let mut cover_vertex: crate::network::nerve::Keyed<u16> = crate::network::nerve::Keyed::new();
    // Vertex index → the cover that placed it, so a loop's class is
    // attributed to the deformation it obstructs and to no other.
    let mut owner: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for (ci, c) in covers.iter().enumerate() {
        for (key, (v, versor, codomain)) in c.cover.carried.fiber(crate::network::nerve::ROOT) {
            let ix = placing.len() as u16;
            cover_vertex.insert(&key, ix);
            owner.insert(vertex_ix_key(ix), ci);
            placing.push(Placing {
                vertex: v.clone(),
                support: codomain.clone(),
                carried: Some(Carried { pair: None, versor: versor.clone() }),
            });
        }
    }
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
    let mut form = Form {
        vertices: placing.iter().map(|p| p.vertex.clone()).collect(),
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
        by_class: crate::network::nerve::Keyed::new(),
        star: crate::network::nerve::Keyed::new(),
        carriages: crate::network::nerve::Keyed::new(),
        by_rebased: crate::network::nerve::Keyed::new(),
        by_unfolded: crate::network::nerve::Keyed::new(),
        demands: crate::network::nerve::Keyed::new(),
    };
    // THE COVER'S MEETS ARE FACES. A meet is a 1-simplex of the nerve;
    // the transport across it is `face_of`'s to compute from the two
    // vertices, and a meet with no transport gets no face (13.9 step 7).
    for c in &covers {
        for (_k, (ka, kb)) in c.cover.meets.fiber(crate::network::nerve::ROOT) {
            let (Some(&i), Some(&j)) = (cover_vertex.get(ka), cover_vertex.get(kb)) else {
                continue;
            };
            if let Some(t) = form.face_of(i, j) {
                form.faces.push(FormFace::new(i, j, t.word));
            }
        }
    }
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    // EACH LOOP OVER CARRIED PIECES, CLASSIFIED ONCE (50.8 case 3). The
    // holonomy is the loop's own, composed around the whole cycle, and it
    // is read by the one classifier both organs use.
    let mut stood: crate::network::nerve::Keyed<Vec<crate::transducer::sentence::ClassVector>> =
        crate::network::nerve::Keyed::new();
    for lp in form.loops.clone() {
        // THE LOOP'S OWNER, read off EVERY vertex and never off one of
        // them by position (rule 15): a loop belongs to the cover that
        // placed all of its vertices, and a loop touching a vertex no
        // cover placed, or two covers, belongs to neither.
        let mut which: Option<usize> = None;
        let mut mixed = false;
        for v in &lp.vertices {
            match owner.get(&vertex_ix_key(*v)).copied() {
                Some(ci) => match which {
                    None => which = Some(ci),
                    Some(w) if w == ci => {}
                    Some(_) => mixed = true,
                },
                None => mixed = true,
            }
        }
        let (Some(ci), false) = (which, mixed) else {
            continue;
        };
        let sigma: Vec<(u8, u8)> =
            lp.vertices.windows(2).map(|p| (p[0] as u8, p[1] as u8)).collect();
        let hue = crate::transducer::sentence::CycleType::of(&sigma);
        let fill = crate::transducer::cech::classify(&lp.holonomy.rotor(), &lp.carriage, hue);
        crate::transducer::family::cover::tally_loop(&fill);
        match fill {
            crate::transducer::cech::Fill::Congruent(g, ell) => {
                let Some(class) = g.class_key() else { continue };
                eprintln!("  cell filled {:02x}{:02x}… class {ell}", class.as_bytes()[0], class.as_bytes()[1]);
                form.torsion.insert(
                    class,
                    crate::transducer::sentence::FiledLoop { sigma, period: ell, chord: lp.chord },
                );
            }
            crate::transducer::cech::Fill::Class(kappa, _) => {
                let k = ci.to_le_bytes();
                match stood.get_mut(&k) {
                    Some(v) => v.push(*kappa),
                    None => {
                        stood.insert(k, vec![*kappa]);
                    }
                }
            }
            _ => {}
        }
    }
    // THE DEMAND VERTICES, AFTER THE LOOPS. A deformation dissolves when
    // every cell of both bodies is carried AND no loop over its pieces
    // stands (50.8): a finite-order holonomy is filed as torsion and the
    // sentence carries the class as its theorem, so a demand kept alive on
    // it would be a demand for what the form already says.
    for (ci, c) in covers.iter().enumerate() {
        let standing = stood.get(&ci.to_le_bytes()).is_some();
        if c.cover.uncarried == [0; 4] && !standing {
            continue;
        }
        let (kind, chart, cells) = c.demand.clone();
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
        placing.push(Placing {
            vertex: demand_vertex(kind.clone(), chart, cells),
            support,
            carried: None,
        });
    }
    // ONE TRUTH: the form's vertices are a projection of `placing`, so a
    // vertex without a support or a carriage cannot exist, and the two
    // cannot drift apart between here and `star_and_cell_faces`.
    form.vertices = placing.iter().map(|p| p.vertex.clone()).collect();
    let supports: Vec<Vec<(i64, i64)>> = placing.iter().map(|p| p.support.clone()).collect();
    let n = form.vertices.len();
    let frame = 0usize;
    for (i, j) in star_and_cell_faces(n, Some(frame), &supports) {
        let Some(g) = form.face_of(i, j) else {
            continue;
        };
        // The edge transport between two carried bodies (50.3); a face to
        // the Frame or to an uncarried body carries none.
        let edge = match (
            placing.get(i as usize).and_then(|p| p.carried.as_ref()),
            placing.get(j as usize).and_then(|p| p.carried.as_ref()),
        ) {
            (
                Some(Carried { pair: Some((ai, bi)), versor }),
                Some(Carried { pair: Some((aj, bj)), .. }),
            ) => edge_transport(
                versor,
                &ins[*ai].body,
                &ins[*aj].body,
                &outs[*bi].body,
                &outs[*bj].body,
            ),
            _ => None,
        };
        form.faces.push(FormFace { i, j, transport: g, edge });
    }
    form.loops = Form::loops_of(&form.faces, n);
    form.reindex();
    form
}

