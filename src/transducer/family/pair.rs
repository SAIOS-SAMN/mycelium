// 21 step 2 — THE PAIR FORM: the scene of one train pair, grouped by chart
// and ranked by carriage.
//
//! `pair_form` is the memoised entry: the scene's candidate bodies grouped
//! by exact chart, paired where a carriage exists, and the unpaired left as
//! the residual the form does not speak.

use super::pairing::{Paired, PairedScene, Scene};
use crate::transducer::sentence::vertex_ix_key;
use super::*;

/// WHAT CARRIED ONE VERTEX (50.3): the versor, and the two SCENE
/// CANDIDATES it was carried between where there are two — by their own
/// addresses, never by where they arrived. A cover piece is a piece of ONE
/// body and names no pair, so 50.3's edge transport — the stabilizer
/// element a pair's rigid motion picks out — is not asked for it.
pub(crate) struct Carried {
    pub(crate) ends: Option<(Vec<u8>, Vec<u8>)>,
    pub(crate) versor: Versor,
}

/// THE SCENERY OF A PAIR FORM AS IT IS BUILT: the vertices, and — keyed by
/// each vertex's OWN coordinate, the index the tape carries as `FormFace.i`
/// and `.j` — the support it stands on and what carried it.
///
/// THESE WERE THREE PARALLEL ARRAYS, index-aligned by discipline and by
/// nothing else, and then one `Vec<Placing>` of three fields, which fixed
/// the alignment but kept `Option` for two readings that are simply absent
/// on most vertices. A support and a carriage are not properties every
/// vertex has: a Frame has neither, a demand has a support and no carriage,
/// a cover piece has both. Keyed, absence is absence — nothing to encode,
/// and no `usize::MAX` reachable as "no member here".
struct Scenery {
    vertices: Vec<Vertex>,
    supports: crate::network::nerve::Keyed<Vec<(i64, i64)>>,
    carried: crate::network::nerve::Keyed<Carried>,
}

impl Scenery {
    fn new() -> Self {
        Scenery {
            vertices: Vec::new(),
            supports: crate::network::nerve::Keyed::new(),
            carried: crate::network::nerve::Keyed::new(),
        }
    }

    /// THE ONE SITE THAT ADDS A VERTEX. It returns the vertex's coordinate,
    /// so a caller that needs to speak about what it placed — the cover, to
    /// map a piece address onto a form vertex — is given it rather than
    /// computing it from a length.
    fn place(
        &mut self,
        vertex: Vertex,
        support: Vec<(i64, i64)>,
        carried: Option<Carried>,
    ) -> u16 {
        let ix = self.vertices.len() as u16;
        // THE SUPPORT IS KEYED BY MEMBER — vertex AND chart (D10) — because a
        // face relates two BODIES and the support is a body's cells. A vertex
        // standing at several charts has a support at each; the vertices this
        // builds stand at one, so each contributes one member, and the pairing
        // now names which body met which instead of leaving it to be guessed.
        // A VERTEX STANDING AT NO CHART GETS NO SUPPORT ENTRY. The support is a
        // BODY's cells and the member key is where it is filed, so a vertex with
        // no member is not an occupant of any cell — absence, not a chart
        // defaulted in for it (rule 11).
        let member = vertex.face_chart().map(|c| member_key(ix, c.key().as_bytes()));
        self.vertices.push(vertex);
        if let (Some(m), false) = (member, support.is_empty()) {
            self.supports.insert(&m, support);
        }
        if let Some(c) = carried {
            self.carried.insert(vertex_ix_key(ix), c);
        }
        ix
    }

    fn carried_at(&self, ix: u16) -> Option<&Carried> {
        self.carried.get(&vertex_ix_key(ix))
    }
}

/// THE EDGE OF A MEET (50.3, 13.9 step 7): the transition between two
/// carried pieces' OWN frames, `V_b ∘ V_a⁻¹`, split into the three
/// coordinates a `Transport` carries — the rotor as a word by A22's peel,
/// the placement and dilation as the carriage, and the hue transition
/// `σ_b ∘ σ_a⁻¹` as the gauge.
///
/// THE THREE ARE THE THREE LANES a class vector reads (42.6): the word is
/// `𝒽`, the carriage is `𝒮`, the gauge is `𝒢`. Dropping any of them makes
/// the loop above blind on that lane before it is classified.
///
/// `None` where the transition has no sensor block or A22 cannot spell it:
/// that meet is a remainder edge and its loop is not evaluated, which is
/// what `edge_holonomy` answers `None` for. A rotor the seventeen cannot
/// spell is 37.7's business and not an edge to invent one for (rule 11).
fn meet_edge(
    va: &Vertex,
    sa: &Versor,
    vb: &Vertex,
    sb: &Versor,
) -> Option<crate::transducer::sentence::Transport> {
    let gamma = sb.geo(&sa.versor_inverse()?);
    let word = peel_versor(&gamma)?.ok()?;
    let t = element::translator_of(&gamma)?;
    let d = element::dilator_of(&gamma)?;
    // The two bodies' own hue maps, each read at the chart its vertex stands
    // at: a cover vertex stands at one, so these are keyed reads and not
    // representatives.
    // Each body's own hue map at the chart its vertex stands at. A cover vertex
    // stands at exactly one, and a vertex standing at none carries no map —
    // which is the empty map, a map that moves no hue, and not a default
    // standing in for an absent reading (rule 11).
    let hue_at = |v: &Vertex| match v.face_chart() {
        Some(c) => crate::transducer::sentence::vertex_gauge_at(v, &c),
        None => Vec::new(),
    };
    let gauge = crate::transducer::sentence::relative_gauge(&hue_at(va), &hue_at(vb));
    Some(crate::transducer::sentence::Transport { word, carriage: d.geo(&t), gauge })
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

#[expect(
    clippy::too_many_arguments,
    reason = "law 8 — unpaired demand generation requires scenes, pair maps, scale, and covers buffer"
)]
fn unpaired_demands(
    ins: &Scene,
    outs: &Scene,
    pairs: &crate::network::nerve::Keyed<Paired>,
    used_in: &crate::network::nerve::Keyed<()>,
    used_out: &crate::network::nerve::Keyed<()>,
    scale: Option<((usize, usize), (usize, usize))>,
    budget: &mut crate::physics::Budget,
    covered: &mut Vec<Covered>,
) -> Vec<Demanded> {
    use crate::transducer::peel::ShellKind::Whole;
    use crate::transducer::sentence::Demand;
    let mut out = Vec::new();
    let in_bodies: Vec<crate::transducer::family::perceive::Body> = ins
        .cands
        .scoped()
        .filter_map(|(_, c)| if c.kind == Whole { Some(c.body.clone()) } else { None })
        .collect();
    let selected_chart = crate::transducer::family::perceive::chart_holding_one(&in_bodies);
    let is_selection_scene = match &selected_chart {
        Some(u) => {
            let mut any_pair = false;
            let mut all_from_u = true;
            for (_k, p) in pairs.scoped() {
                any_pair = true;
                if let Some(ic) = ins.get(&p.input) {
                    if ic.body.moduli != *u {
                        all_from_u = false;
                        break;
                    }
                } else {
                    all_from_u = false;
                    break;
                }
            }
            if any_pair && all_from_u {
                for (ja, oc) in outs.cands.scoped() {
                    if oc.kind == Whole && !used_out.contains(&ja) {
                        all_from_u = false;
                        break;
                    }
                }
            }
            any_pair && all_from_u
        }
        None => false,
    };
    // The unpaired input bodies first: what each overlaps decides its
    // reading. An output body of the SAME chart it overlaps is its image
    // in place — carried on the shape, its hues not a map (a section over
    // the same base, 25): the input is the SOURCE of that output. An
    // output body of another chart it overlaps is the image of its
    // DEFORMATION, and is consumed as such. Nothing overlapping: vanished.
    //
    // Both scenes are walked in their candidates' own KEY order, which is
    // content order: two minds that perceived the same pair of grids read
    // the same demands in the same order, whatever order `perceive`
    // happened to return the bodies in.
    let mut sourced_in: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut imaged_out: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // WHICH OUTPUT BODIES STAND AT EACH CELL, read once.
    //
    // This walked EVERY output candidate for EVERY input candidate and asked
    // `supports_overlap`, which itself rebuilt the input body's whole cell
    // index on each of those passes — a product search across two scenes
    // with a third walk inside it, where the question is only "what stands
    // where I stand". A body's overlap is a keyed read at its own cells and
    // costs its own support, not the other scene's cardinality (rule 1).
    //
    // Built in the outputs' own key order, so the fibre at a cell is in
    // content order and two minds that perceived the same pair read the same
    // demands in the same order.
    let mut at_cell: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> =
        crate::network::nerve::Keyed::new();
    for (ja, oc) in outs.cands.scoped() {
        if used_out.contains(&ja) || oc.kind != Whole {
            continue;
        }
        for &(r, c, _) in &oc.body.cells {
            at_cell.nested_insert(crate::transducer::family::cell_pos_key(r, c), ja.clone(), ());
        }
    }
    for (ia, ic) in ins.cands.scoped() {
        if used_in.contains(&ia) || ic.kind != Whole {
            continue;
        }
        // The outputs this body meets: the union of the fibres at its own
        // cells. A body meets nothing where it stands on nothing.
        let mut meets: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for &(r, c, _) in &ic.body.cells {
            if let Some(here) = at_cell.get(&crate::transducer::family::cell_pos_key(r, c)) {
                for (ja, ()) in here.scoped() {
                    meets.insert(&ja, ());
                }
            }
        }
        let mut same: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut images: crate::network::nerve::Keyed<(Invariant, Vec<u8>)> =
            crate::network::nerve::Keyed::new();
        for (ja, ()) in meets.scoped() {
            let Some(oc) = outs.cands.get(&ja) else {
                continue;
            };
            if differing_axes(&ic.body.moduli, &oc.body.moduli).is_empty() {
                same.insert(&ja, ());
            } else {
                images.insert(oc.body.moduli.key(), (oc.body.moduli.clone(), ja.clone()));
            }
        }
        if same.scoped().next().is_some() {
            sourced_in.insert(&ia, ());
            continue;
        }
        match images.scoped().next() {
            None => {
                if is_selection_scene && selected_chart.as_ref().is_some_and(|u| ic.body.moduli != *u) {
                    continue;
                }
                let axes: Vec<u16> = (0..ic.body.moduli.0.len() as u16).collect();
                let cells: Vec<_> = ic.body.cells.iter().map(|&(r, c, h)| (r, c, h, 0u8)).collect();
                out.push((Demand::TopologicalDeformation { image: Invariant(Vec::new()), axes }, ic.body.moduli.clone(), cells));
            }
            Some(_) => {
                for (_, (image, ja)) in images.scoped() {
                    let Some(oc) = outs.get(ja) else { continue };
                    imaged_out.insert(ja, ());
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
                        &oc.body,
                        scale,
                        budget,
                    )
                    .ok();
                    // A cover that did not complete files nothing. Filing
                    // what a half-run cover happened to reach would put a
                    // class on the form that no measurement stands behind
                    // (rule 11).
                    let axes = differing_axes(&ic.body.moduli, image);
                    let cells = delta_cells(&ic.body, &oc.body);
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
                    for (_piece, d) in cover.refused.scoped() {
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
    // THE SECOND PRODUCT, KEYED THE SAME WAY. This walked every input
    // candidate for every output candidate and asked `versor_between`, which
    // builds a coset and verifies it cell by cell — the most expensive
    // question in the file, asked across the whole Cartesian product.
    //
    // A VERSOR IS A BIJECTION ON THE SUPPORT: `versor_coset` verifies by
    // carrying every cell of one body into a cell of the other and back, so
    // two shapes of different cell counts can never be related by one. The
    // count is a coordinate the blade already carries and it is exactly
    // versor-invariant, so it strata the inputs and the coset is built only
    // within a stratum. It is a NECESSARY condition and not a sufficient
    // one, so `versor_between` still decides — nothing is admitted by the
    // key that the versor does not carry.
    let mut ins_by_support: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> =
        crate::network::nerve::Keyed::new();
    let mut in_at_cell: crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> =
        crate::network::nerve::Keyed::new();
    for (ia, ic) in ins.cands.scoped() {
        if ic.kind != Whole {
            continue;
        }
        let n = ic.body.shape.blade.cells.len() as u32;
        ins_by_support.nested_insert(n.to_le_bytes(), ia.clone(), ());
        for &(r, c, _) in &ic.body.cells {
            in_at_cell.nested_insert(crate::transducer::family::cell_pos_key(r, c), ia.clone(), ());
        }
    }
    for (ja, oc) in outs.cands.scoped() {
        if used_out.contains(&ja) || oc.kind != Whole || imaged_out.contains(&ja) {
            continue;
        }
        // The inputs standing where this body stands, read at its own cells.
        let mut met_by: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for &(r, c, _) in &oc.body.cells {
            if let Some(here) = in_at_cell.get(&crate::transducer::family::cell_pos_key(r, c)) {
                for (ia, ()) in here.scoped() {
                    met_by.insert(&ia, ());
                }
            }
        }
        let mut charts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        let mut cells: Vec<crate::transducer::sentence::Cell> = Vec::new();
        let n = (oc.body.shape.blade.cells.len() as u32).to_le_bytes();
        let congruent = match ins_by_support.get(&n) {
            Some(c) => c.scoped().map(|(ia, ())| ia).collect::<Vec<_>>(),
            None => Vec::new(),
        };
        for ia in congruent {
            let Some(ic) = ins.cands.get(&ia) else {
                continue;
            };
            let Some(v) = blade::versor_between(&ic.body.shape, &oc.body.shape) else {
                continue;
            };
            charts.insert(ic.body.moduli.key(), ());
            if sourced_in.contains(&ia) && met_by.contains(&ia) {
                cells.extend(chain_of(Some(&ic.body), Some(&oc.body), &v));
            }
        }
        if cells.is_empty() {
            cells = oc.body.cells.iter().map(|&(r, c, h)| (r, c, 0u8, h)).collect();
        }
        let kind = match charts.scoped().next() {
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
        // A cell the image does not hold is absent there, and hue 0 IS
        // absence in this encoding.
        let gone = match after.get(&cell_pos_key(r, c)) {
            Some(h2) => *h2,
            None => 0,
        };
        cells.push((r, c, h, gone));
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
    trace::trace("pair/0 bodies+pairing", budget);
    let PairedScene { ins, outs, pairs, used_in, used_out } = pair_scene(a, b, scale);
    trace::trace_n("pair/1 paired", pairs.len(), budget);
    let mut scenery = Scenery::new();
    let frame = scenery.place(frame_vertex(), Vec::new(), None);
    let mut demanded: Vec<Demanded> = Vec::new();
    for (_k, Paired { input, image, carriage }) in pairs.scoped() {
        let (Some(ic), Some(oc)) = (ins.get(input), outs.get(image)) else {
            continue;
        };
        match word_or_demand(&ic.body, &oc.body) {
            Some(Ok(v)) => {
                scenery.place(
                    v,
                    support_of(&oc.body),
                    Some(Carried {
                        ends: Some((input.clone(), image.clone())),
                        versor: carriage.v.clone(),
                    }),
                );
            }
            Some(Err(d)) => demanded.push(d),
            None => {}
        }
    }
    trace::trace_n("pair/2 versors", scenery.vertices.len(), budget);
    let mut covers: Vec<Covered> = Vec::new();
    demanded.extend(unpaired_demands(&ins, &outs, &pairs, &used_in, &used_out, scale, budget, &mut covers));
    trace::trace_n("pair/3 covers", covers.len(), budget);
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
    // Vertex coordinate → the cover that placed it, so a loop's class is
    // attributed to the deformation it obstructs and to no other.
    let mut owner: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for (ci, c) in covers.iter().enumerate() {
        for (key, (v, versor, codomain)) in c.cover.carried.scoped() {
            let ix = scenery.place(
                v.clone(),
                codomain.clone(),
                Some(Carried { ends: None, versor: versor.clone() }),
            );
            cover_vertex.insert(&key, ix);
            owner.insert(vertex_ix_key(ix), ci);
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
        vertices: scenery.vertices.clone(),
        faces: Vec::new(),
        loops: Vec::new(),
        remainder: crate::transducer::element::PlacedChain::default(),
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
        by_transport: crate::network::nerve::Keyed::new(),
        star: crate::network::nerve::Keyed::new(),
        carriages: crate::network::nerve::Keyed::new(),
        by_rebased: crate::network::nerve::Keyed::new(),
        by_unfolded: crate::network::nerve::Keyed::new(),
        demands: crate::network::nerve::Keyed::new(),
    };
    // THE COVER'S MEETS ARE FACES. A meet is a 1-simplex of the nerve;
    // the transport across it is `face_of`'s to compute from the two
    // vertices, and a meet with no transport gets no face (13.9 step 7).
    //
    // AND THE FACE CARRIES ITS EDGE (50.3). `face_of` gives the COBOUNDARY
    // of the two pieces' placements, which telescopes around a cycle over
    // words that commute — and every word here is a sensor-plane word — so
    // a loop read from transports alone is flat however curved the scene
    // is. The edge is the other object: the transition between the two
    // pieces' OWN frames, `V_b ∘ V_a⁻¹`, which the cover already holds
    // both halves of in `carried`. `FormFace::new` left it `None`, so
    // `edge_holonomy` answered `None` for every loop in the tree and the
    // reader has never been fed.
    for c in &covers {
        for (mk, ()) in c.cover.meets.scoped() {
            // The meet's address is its two ends; nothing is stored beside
            // it to be read here instead (law 12).
            let Some((ka, kb)) = crate::transducer::family::pieces::meet_ends(&mk) else {
                continue;
            };
            let (Some(&i), Some(&j)) = (cover_vertex.get(ka), cover_vertex.get(kb)) else {
                continue;
            };
            // THE MEET NAMES THE TWO PIECES, so it names the member pair: a
            // cover vertex stands at one chart, and the meet's two ends are
            // those two bodies and no others. The incidence is read, not a
            // product of the two vertices' fibres (D10 item 5).
            let (Some(vi), Some(vj)) = (form.vertices.get(i as usize), form.vertices.get(j as usize))
            else {
                continue;
            };
            let (Some(ci), Some(cj)) = (vi.face_chart(), vj.face_chart()) else {
                continue;
            };
            let edge = match (c.cover.carried.get(ka), c.cover.carried.get(kb)) {
                (Some((va, sa, _)), Some((vb, sb, _))) => meet_edge(va, sa, vb, sb),
                _ => None,
            };
            if edge.is_some() {
                crate::transducer::family::cover::tally(|t| t.edges += 1);
            }
            let Some(word) = form.face_word(i, j) else { continue };
            let Some(place) = form.face_place(i, &ci, j, &cj, edge) else {
                continue;
            };
            let mut at = crate::network::nerve::Keyed::new();
            at.insert(crate::transducer::sentence::face_pair_key(&ci, &cj), place);
            form.faces.push(FormFace { i, j, word, at });
        }
    }
    let meet_faces = form.faces.len();
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
        // THE EDGE HOLONOMY IS WHAT IS CLASSIFIED (50.3), not the loop's
        // transport composite. A loop the cover did not evaluate — a
        // remainder edge anywhere on it — answers `None` and is not
        // classified at all, which is 13.9 step 7 and not a flat reading.
        // ONE PER MEMBER PATH, and each classified on its own. A cover's
        // vertices stand at one chart each, so today this is one path per loop;
        // where a loop runs through a vertex acting on several, each path is a
        // cycle of its own and each carries its own obstruction.
        let Some(es) = form.edge_holonomies(&lp) else {
            continue;
        };
        for (_, e) in es.scoped() {
        // AND THE HUE IS THE COMPOSED GAUGE. This read
        // `lp.vertices.windows(2).map(|p| (p[0] as u8, p[1] as u8))` — the
        // loop's VERTEX INDICES cast to bytes — and handed them to
        // `CycleType::of` as if they were a hue permutation. A member's
        // position in a vector stood in for the colours it carries (rule
        // 15), so the `𝒢` coordinate of every cover loop was a function of
        // where its vertices happened to sit, and `Form::class_vector`
        // beside it has always read the composed gauge instead: the cover
        // and the pivot disagreed about what a loop's hue is.
        let hue = crate::transducer::sentence::CycleType::of(&e.gauge);
        let sigma = e.gauge.clone();
        let fill = crate::transducer::cech::classify(&e.word.rotor(), &e.carriage, hue);
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
    for (_, (kind, chart, cells)) in by_demand.scoped() {
        let mut cells = cells.clone();
        cells.sort_unstable();
        cells.dedup();
        let mut support: Vec<(i64, i64)> = cells.iter().map(|&(r, c, _, _)| (r, c)).collect();
        support.sort_unstable();
        support.dedup();
        scenery.place(demand_vertex(kind.clone(), chart, cells), support, None);
    }
    // ONE TRUTH: the form's vertices are the scenery's, and the supports
    // and carriages are keyed by the same coordinate the form's faces use,
    // so a vertex and what it stands on cannot drift apart between here and
    // `star_and_cell_faces`.
    form.vertices = scenery.vertices.clone();
    let n = form.vertices.len();
    // THE INCIDENCE IS THE OCCUPANCY, AND IT NAMES THE MEMBER PAIR (D10 item
    // 5). `star_and_cell_faces` keys supports by member — vertex AND chart — and
    // a cell names its occupants, so each pair it yields says which BODY of `i`
    // met which body of `j`. Nothing here enumerates and nothing takes the
    // product of two fibres: the two charts are read straight off the two member
    // keys the pairing returned.
    // The Frame's own member: it stands at the root stratum, which is what
    // `face_chart` answers for it. A slot that is not a vertex, or one standing
    // at no chart, is not a frame to star from — `None` there means this
    // pairing writes no star faces, which is the reading and not a frame
    // invented for it (rule 11).
    let frame_member = scenery
        .vertices
        .get(frame as usize)
        .and_then(|v| v.face_chart())
        .map(|c| member_key(frame, c.key().as_bytes()));
    for (ak, bk) in star_and_cell_faces(frame_member.as_deref(), &scenery.supports) {
        let (Some((i, cik)), Some((j, cjk))) = (read_member_key(&ak), read_member_key(&bk)) else {
            continue;
        };
        // The edge transport between two carried bodies (50.3); a face to
        // the Frame or to an uncarried body carries none.
        let edge = match (scenery.carried_at(i), scenery.carried_at(j)) {
            (
                Some(Carried { ends: Some((ai, bi)), versor }),
                Some(Carried { ends: Some((aj, bj)), .. }),
            ) => match (ins.get(ai), ins.get(aj), outs.get(bi), outs.get(bj)) {
                (Some(ia), Some(ja), Some(ib), Some(jb)) => {
                    edge_transport(versor, &ia.body, &ja.body, &ib.body, &jb.body)
                }
                _ => None,
            },
            _ => None,
        };
        let (Some(vi), Some(vj)) = (form.vertices.get(i as usize), form.vertices.get(j as usize))
        else {
            continue;
        };
        let (Some(ci), Some(cj)) = (vi.chart_keyed(&cik), vj.chart_keyed(&cjk)) else {
            continue;
        };
        let Some(word) = form.face_word(i, j) else { continue };
        let Some(place) = form.face_place(i, &ci, j, &cj, edge) else {
            continue;
        };
        let mut at = crate::network::nerve::Keyed::new();
        at.insert(crate::transducer::sentence::face_pair_key(&ci, &cj), place);
        form.faces.push(FormFace { i, j, word, at });
    }
    form.loops = Form::loops_of(&form.faces, n);
    form.reindex();
    // WHAT A PAIR FORM COST, WHEN IT IS BIG ENOUGH TO MATTER. A held form
    // is pivoted every stroke and `trial` clones the whole of it once per
    // candidate, so a form's loop count is its price in both cores and
    // bytes — 24 920 loops on one form held two voices at 3 GB and 85% of
    // a core. The count alone does not say WHO named that many faces, and
    // three namers meet here: the cover's own meets, the intra-cell chain,
    // and the ring adjacency. The reading separates them, so the next
    // explosion is attributed instead of hunted.
    //
    // The line is unconditional: a threshold on the loop count would be a
    // count deciding a branch (rule 2), and the gate refuses it — rightly,
    // because the size at which a form becomes news is exactly the judgment
    // that was wrong last time. One line per pair form, which is one per
    // scene the mind perceives.
    //
    // `meet_faces` is a prefix of `faces`, which only grows here, so the
    // difference is exact and not a floor.
    // AND WHAT THE LOOPS CARRY, because the count alone cannot say whether
    // any of them is curved. Torsion is filed by `file_loop` under
    // `lp.holonomy.rotor().class_key()`, and `lp.holonomy` is
    // `form_named_holonomy`, which composes each face's OWN WORD. A face made
    // by `face_of` carries the coboundary `w_j ∘ w_i⁻¹`, and a product of
    // coboundaries around a cycle telescopes to the identity ALGEBRAICALLY —
    // no data makes it curve. Only a section face (`face_carrying`, 21 step 2,
    // S12.2), whose word is deliberately not that coboundary, can leave a
    // loop with holonomy to file.
    //
    // So the three counts separate the only three states a loop can be in,
    // and the corpus-wide zero on `finite` is what says whether the engine
    // has ever had a cycle to file at all.
    let (mut flat, mut finite, mut infinite) = (0usize, 0usize, 0usize);
    // AND THE SAME READING ON THE EDGE TRANSPORTS, which is the other
    // holonomy this form carries: `edge_holonomies` composes `place.edge`,
    // the 50.3 transport `meet_edge` measures on a cover meet, and its own
    // doc says "stabilizer cosets do not telescope, which is what makes it a
    // reading". If the words are flat by algebra and the edges are not, the
    // curvature is present and the filing instrument is looking at the wrong
    // carrier.
    let (mut e_flat, mut e_fin, mut e_inf) = (0usize, 0usize, 0usize);
    for lp in &form.loops {
        match lp.holonomy.is_identity() {
            true => flat += 1,
            false => match lp.holonomy.order() {
                crate::transducer::alphabet::Order::Finite(_) => finite += 1,
                crate::transducer::alphabet::Order::Infinite => infinite += 1,
            },
        }
        if let Some(hs) = form.edge_holonomies(lp) {
            for (_, h) in hs.scoped() {
                match h.word.is_identity() {
                    true => e_flat += 1,
                    false => match h.word.order() {
                        crate::transducer::alphabet::Order::Finite(_) => e_fin += 1,
                        crate::transducer::alphabet::Order::Infinite => e_inf += 1,
                    },
                }
            }
        }
    }
    // A FORM WITH NOTHING TO REPORT ABOUT ITS HOLONOMY REPORTS NOTHING. The
    // line's own argument for being unconditional is that a threshold on the
    // loop COUNT would be a count deciding a branch (rule 2), and that is
    // right about a threshold on magnitude. This is not one: it is the
    // presence of the object the line is about. A form with no loop and no
    // cover meet has no holonomy, and `flat 0, finite 0, infinite 0` over
    // nothing is the vacuous assertion at the instrument.
    //
    // Measured on the fleet at e98e8a2: 6535 pair forms in twelve minutes
    // across five voices, of which 90% had ZERO cover meets and the median
    // had zero loops. The tail is where every reading came from — one form
    // carried 677 vertices, 4030 faces, 1305 cover meets and 3361 loops —
    // and the tail is what this now prints.
    if !form.loops.is_empty() || meet_faces > 0 {
        eprintln!(
            "  pair form {n} vertices, {} faces ({meet_faces} cover meets, {} cell+ring), {} loops \
             (word holonomy: flat {flat}, finite {finite}, infinite {infinite} \
             | edge holonomy paths: flat {e_flat}, finite {e_fin}, infinite {e_inf})",
            form.faces.len(),
            form.faces.len() - meet_faces,
            form.loops.len()
        );
    }
    trace::trace_n("pair/4 done", n, budget);
    form
}
