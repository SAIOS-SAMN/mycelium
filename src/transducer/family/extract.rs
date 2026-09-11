// 21 steps 1-7 — EXTRACTION: the rule a family of train pairs already
// states, read off the pair forms rather than searched for.
//
//! A transport class names what a vertex does; the ends name where it does
//! it; the face word between two ends is constant across the bundle or the
//! rule refuses. Each step reads what the pair forms already agree on.

use super::*;


/// Step 4 of chapter 21: along each named face (i, j) with `w_i` not the
/// identity, `j` is `i` read in `j`'s frame when, on every pair of the
/// bundle, the two bodies carry one linear word and differ only by their
/// relative placement `t = p_j − p_i`. Conjugation by a translation keeps
/// the linear block and moves the centre, so `t · w_i · t⁻¹` is realised
/// by `apply_form` as `w_i` at `p_i + t`; nothing is spelled for `t`.
pub(crate) fn rebase_named_faces(form: &mut Form, pair_forms: &[Form]) {
    let named: Vec<FormFace> = form.faces.clone();
    // The face's own index IS the chord it closes on where it closes: this
    // is a clone of `form.faces` and the rebase adds no face, so the two
    // agree throughout. A caller that walked a face knows which one; it is
    // passed and never searched for (15.5).
    for (fi, face) in named.iter().enumerate() {
        let (i, j) = (face.i as usize, face.j as usize);
        if matches!(form.vertices.get(j), Some(Vertex::Rebased { .. })) {
            continue;
        }
        let (Some(Vertex::Word {
            derivation: di,
            carriage: ci,
            ..
        }), Some(Vertex::Word { carriage: cj, .. })) = (
            form.vertices.get(i).cloned(),
            form.vertices.get(j).cloned(),
        ) else {
            continue;
        };
        if di.rotor().projective_eq(&crate::transducer::alphabet::Rotor::identity()) {
            continue;
        }
        trace::trace_at("extract/4 face-i", i);
        trace::trace_at("extract/4 face-j", j);
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
            let _ = form.rebase(face.i, face.j, relative, fi as u16);
            trace::trace_at("extract/4 rebased", j);
        }
    }
    trace::trace_at("extract/4 reindex", form.vertices.len());
    form.reindex();
    for (_, &fi) in form.star.fiber(crate::network::nerve::ROOT) {
        let Some(face) = form.faces.get(fi as usize) else {
            continue;
        };
        let (i, j) = (face.i, face.j);
        trace::trace_at("extract/4 star-face", fi as usize);
        if let Some(t) = form.face_of(i, j) {
            form.faces[fi as usize].transport = t;
        }
        trace::trace_at("extract/4 star-face-done", fi as usize);
    }
}


/// The germ of one pair: what the input carries onto the output, as the
/// cells of the change. `None` for a pair with no scene.
fn germ_of_pair(a: &Grid, b: &Grid) -> Option<Vec<crate::transducer::sentence::Cell>> {
    let (ho, wo) = frame_of(b);
    if a.is_empty() || ho == 0 || wo == 0 {
        return None;
    }
    let x = crate::arc::grid_to_cells(&crate::arc::embed(a, ho, wo));
    let y = crate::arc::grid_to_cells(b);
    crate::transducer::germinate::germinate(&x, &y)
}

/// THE SPEND ONE EXTRACTION OF THIS FAMILY IS SIZED BY (rule 10, L8w).
///
/// A fixed `spend_unit()` — three on an eight-core box — is not a budget
/// for a family, it is a budget for nothing: step 0 spends one per pair
/// and `pair_form` spends its cover walk out of the same purse, so the
/// third pair of a three-pair family was never reached and on the fleet
/// 469 of 474 families extracted to nothing. Worse, the size of that
/// purse is `reserve_max()/8`, which is read off the machine's free
/// memory at boot: whether a family could be spelled at all depended on
/// what else the box was doing.
///
/// The spend is intrinsic instead — the pairs and the carrier each one
/// spans, which is the boundary the extraction has to walk. Two courts on
/// two machines therefore give one family the same budget, and a family
/// the mesh hands over is a family that can be measured.
pub fn extraction_spend(family: &[(Grid, Grid)]) -> usize {
    let mut n = 0usize;
    for (a, b) in family {
        for g in [a, b] {
            // `frame_of` is the one reader of a grid's carrier (law 12).
            let (h, w) = frame_of(g);
            n = n.saturating_add(h * w);
        }
    }
    n
}

/// The tail of an extraction: the demands ride the rule and the form is
/// closed out with its depth, loops, ratio and sections.
///
/// `unreached` is the part of the bundle the budget did not get to. IT IS
/// NOT DROPPED. Step 0 returned `None` the moment the purse emptied,
/// discarding every pair form it had already assembled along with the
/// whole boundary of the family — 469 families a pass, conserving
/// nothing, and re-pulled on the next stroke to be discarded identically.
/// An unreached pair is cells of the family this form does not account
/// for, which is exactly the charge `contract` emits for an undeclared
/// remainder, and it rides the rule as one.
fn finish(
    mut form: Form,
    pair_forms: &[Form],
    scale: ((usize, usize), (usize, usize)),
    sections: crate::network::nerve::Keyed<Vec<u16>>,
    unreached: &[(Grid, Grid)],
) -> Option<crate::transducer::sentence::SentenceRule> {
    // 50.5 (F step 5): the demands of every pair ride the rule, one vertex
    // per (kind, chart) over the bundle, carrying every pair's cells.
    let mut by_demand: crate::network::nerve::Keyed<Demanded> = crate::network::nerve::Keyed::new();
    for pf in pair_forms {
        for (_, kind, chart, cells) in pf.demand_vertices() {
            let key = crate::transducer::sentence::demand_key(kind, chart);
            match by_demand.get_mut(&key) {
                Some((_, _, held)) => held.extend_from_slice(cells),
                None => {
                    by_demand.insert(key, (kind.clone(), chart.clone(), cells.to_vec()));
                }
            }
        }
    }
    for (a, b) in unreached {
        let Some(germ) = germ_of_pair(a, b) else { continue };
        let chart = crate::transducer::stalk::invariant(&germ);
        let multiplicity = crate::transducer::sentence::pieces(&germ).len() as u32;
        let kind = crate::transducer::sentence::Demand::Source { multiplicity };
        if kind.typed().is_err() {
            continue;
        }
        let key = crate::transducer::sentence::demand_key(&kind, &chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend_from_slice(&germ),
            None => {
                by_demand.insert(key, (kind, chart, germ));
            }
        }
    }
    for (_, (kind, chart, cells)) in by_demand.fiber(crate::network::nerve::ROOT) {
        let mut cells = cells.clone();
        cells.sort_unstable();
        cells.dedup();
        form.vertices.push(demand_vertex(kind.clone(), chart, cells));
    }
    form.depth = form.depth_of();
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.ratio = scale;
    form.sections = sections;
    form.hue_maps = hue_maps_of_form(&form);
    Some(crate::transducer::sentence::rule_of_form(form))
}

/// Seven moves on pair forms (chapter 21), under the stroke (L8w, rule 10).
///
/// Extraction from cells SEARCHES for correspondence, so its cost grows
/// with the scene and not with what the mind holds: on
/// `0607ce86.json` — three pairs at 21×22 — it did not return in 900s,
/// where twelve smaller families take six seconds together. A voice on the
/// fleet was therefore stalled on one listing with no way out. Every step
/// below spends the caller's `Budget`, and an extraction the budget cuts
/// off MEASURED NOTHING: it returns `None` and says nothing at all. A
/// truncated bundle is not a smaller rule — a rule read off two pairs of
/// four states what those two agree on, which the other two may refuse —
/// and minting it would utter as complete a shape the mind never
/// measured, have the court refuse it, and seal the address against the
/// shape that was never spoken (50.9). "A budget that ran out is
/// UNMEASURED, and a refusal is not a demand" (rule 11, 50.2). The voice
/// withholds and the next stroke resumes. This is the STOP; the
/// resolution is L8x, where F factors through the presentation and the
/// search is gone (50.2 step 2).
pub fn extract_rule_with(
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    let scale = family_ratio(family)?;
    trace::trace_n("extract/0 pair-forms", family.len(), budget);
    let mut pair_forms: Vec<Form> = Vec::with_capacity(family.len());
    // WHAT THE BUDGET DID NOT REACH, kept (radical conservation). Both
    // this loop's arms discarded the whole family: the spend check
    // returned `None`, and `pair_form(...)?` did the same for a pair it
    // could not factor. Every pair form already assembled went with them.
    // A pair that was not reached is not a refusal and not a smaller rule
    // — it is cells of the family this form does not account for, and it
    // rides out as the demand `finish` emits for it.
    let mut unreached: Vec<(Grid, Grid)> = Vec::new();
    for (i, (a, b)) in family.iter().enumerate() {
        trace::trace_n("extract/0 pair", i, budget);
        if !budget.spend() {
            unreached.push((a.clone(), b.clone()));
            continue;
        }
        // 50.2 step 2: the extraction's own crystal is the presentation
        // its word vertices derive against, so a mind extracts in its own
        // language and `NoCrystal` extracts in the cipher's. `None` is a
        // vertex the budget could not factor — unmeasured, not spelled —
        // and the pair is carried out as a demand rather than taking the
        // bundle with it (rule 11, L8w).
        match pair_form(crystal, budget, a, b, Some(scale)) {
            Some(pf) => pair_forms.push(pf),
            None => unreached.push((a.clone(), b.clone())),
        }
    }
    let n_pairs = pair_forms.len();
    trace::trace_n("extract/1 correspondence", n_pairs, budget);
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
            if let Vertex::Word { derivation, gauge, .. } = v {
                if let Some(hk) =
                    crate::transducer::sentence::hue_section_key_of(&derivation.rotor(), gauge)
                {
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
    trace::trace_n("extract/2 faces", form.vertices.len(), budget);
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
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached);
        }
        // 21 step 2, 25 (Stage 12 S12.2): the face word between two ends,
        // read on every pair of the bundle. Constant, it is the rule's face.
        // Not constant, it is KEPT as sections over the pairs and never
        // dropped: the rule's face carries the first word, and every other
        // word the bundle speaks between the same ends is a second face
        // between the same vertices — two carriages on one class are two
        // sections (18.2) — so the form has a loop whose holonomy is the
        // transition between the sections, held with its class vector (12.6)
        // for the pivot loop to read. Dropped, every rule was a star with no
        // loop and the pivot had nothing to act on (I34). The words of one
        // face are keyed by M1 (`Word::key`), never scanned.
        let mut words: Vec<(crate::transducer::alphabet::Word, Vec<u16>)> = Vec::new();
        let mut at: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (pi, pf) in pair_forms.iter().enumerate() {
            let Some(w) = face_word_between(pf, ea, eb) else {
                continue;
            };
            let k = w.key();
            match at.get(k.as_bytes()).copied() {
                Some(n) => {
                    if let Some((_, pairs)) = words.get_mut(n) {
                        pairs.push(pi as u16);
                    }
                }
                None => {
                    at.insert(k.as_bytes(), words.len());
                    words.push((w, vec![pi as u16]));
                }
            }
        }
        let Some((i, j)) = ends
            .get(&end_key(ea))
            .copied()
            .zip(ends.get(&end_key(eb)).copied())
        else {
            continue;
        };
        // A face whose two ends are ONE end lands as `i == j`: two bodies
        // of one transport class, related on a pair form, map to one rule
        // vertex. It is NOT dropped. It is a closed orbit at that vertex —
        // an element of its stabilizer — and `Form::rebase` evaluates its
        // boundary holonomy when move 4 reaches it (L20).
        let mut sections_of_face = words.into_iter();
        let Some((_, first_pairs)) = sections_of_face.next() else {
            continue;
        };
        let Some(t) = form.face_of(i, j) else {
            continue;
        };
        form.faces.push(FormFace { i, j, transport: t, edge: None });
        let mut rest = sections_of_face.peekable();
        if rest.peek().is_some() {
            sections.insert(ends_pair_key(ea, eb), first_pairs);
        }
        for (w, pairs) in rest {
            let mut key = ends_pair_key(ea, eb);
            crate::transducer::address::put_bytes(&mut key, w.key().as_bytes());
            form.faces.push(FormFace::new(i, j, w));
            sections.insert(key, pairs);
        }
    }
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.reindex();
    trace::trace_n("extract/3 glue", form.faces.len(), budget);
    // 3. Glue: placement is the relative word Frame → class, constant across the bundle.
    // Crystal is indexed by the section coordinate (moduli); correspondence is transport class.
    let glued_classes: Vec<(TransportClass, Invariant)> = form
        .vertices
        .iter()
        .filter_map(|v| Some((transport_class(v)?, class_of_vertex(v)?)))
        .collect();
    for (tc, moduli) in glued_classes {
        // The glue is what the mind already holds; a budget that ends here
        // leaves the form it has, ungLued, rather than unsaying the pairs
        // it measured to get here.
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached);
        }
        let Some(placement) = placement_of_class(&pair_forms, &tc) else {
            continue;
        };
        for (e, s, _st) in crystal.on_body(&moduli) {
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
                        drop.insert(crate::transducer::sentence::vertex_ix_key(i as u16), ());
                    }
                }
                form.retract_vertices(&drop);
            }
        }
    }
    trace::trace_n("extract/4 rebase", form.vertices.len(), budget);
    // 4. Rebase: w_j = t · w_i · t⁻¹ with t the relative placement, on
    // every pair. Named faces only; Frame is not a conjugator.
    rebase_named_faces(&mut form, &pair_forms);
    trace::trace("extract/5 unfold", budget);
    // 5. Unfold grow-to-frame.
    if family.iter().all(|(a, b)| grows_to_frame(a, b)) {
        form.unfold(crate::transducer::sentence::Measure::Occupied);
    }
    // 6. Contact: placement differs across pairs, peel ring to Frame is one value.
    let nverts = form.vertices.len();
    trace::trace_n("extract/6 contact", nverts, budget);
    for vi in 0..nverts {
        // A contact ring not read is a vertex that keeps its carriage,
        // which is what it had; it is not a reason to unsay the form.
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached);
        }
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
                    *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
                    **carriage = element::identity_versor();
                }
            }
        }
    }
    trace::trace_n("extract/7 coherence", form.loops.len(), budget);
    // 7. Coherence: tree gauge, else hold the loops.
    if !form.coheres() {
        if let Some(gamma) = form.solve_coboundary() {
            let _ = form.regauge(&gamma);
        }
    }
    trace::trace("extract/8 finish", budget);
    finish(form, &pair_forms, scale, sections, &unreached)
}


/// Re-derive the family's rule and check it agrees with `rule`. The court
/// replays under its own stroke, as the voice extracted under its (L8w).
pub fn rule_replays_family(
    rule: &crate::transducer::sentence::SentenceRule,
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> bool {
    let Some(again) = extract_rule(family, budget) else {
        return false;
    };
    again.form.vertices == rule.form.vertices && again.form.faces == rule.form.faces
}

/// Extract, memoized by family id and working-atlas generation. A hit
/// spends nothing; a miss spends one Budget and stores the rule.
pub fn extract_rule_cached(
    cache: &mut crate::network::nerve::Keyed<(crate::transducer::address::Address, Option<crate::transducer::sentence::SentenceRule>)>,
    fam: &[u8],
    family: &[(Grid, Grid)],
    atlas_root: crate::transducer::address::Address,
    budget: &mut crate::physics::Budget,
    crystal: &dyn Crystal,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if let Some((g, rule)) = cache.get(fam) {
        if *g == atlas_root {
            return rule.clone();
        }
    }
    if !budget.spend() {
        return cache.get(fam).and_then(|(_, r)| r.clone());
    }
    let rule = extract_rule_with(family, crystal, budget);
    // THE PARTIAL IS CACHED WITH THE WHOLE. An extraction that ended on
    // its budget used to measure nothing, so nothing was cached and the
    // next stroke asked again — the same family, the same purse, the same
    // point of exhaustion, the same nothing, for as long as the node ran.
    // That was the groundhog cycle behind 3763 idle strokes in a row.
    //
    // An extraction now always says what it measured, and what it did not
    // reach rides out as a demand. Caching that is not caching "this
    // family has no rule": it is holding the form the mind HAS, whose
    // demands go to the market to be answered. The entry is keyed by the
    // atlas root, so the moment this mind's language moves — which is the
    // only thing that could spell more of it — the extraction runs again
    // and reaches further.
    cache.insert(fam, (atlas_root, rule.clone()));
    rule
}

/// Extract the invariant cochain of a family. The live path is the form
/// engine (`extract_rule_with`); the nerve extractor is retired.
pub fn extract_rule(
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal, budget)
}
