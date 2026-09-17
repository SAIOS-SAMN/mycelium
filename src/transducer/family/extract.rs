// 21 steps 1-7 — EXTRACTION: the rule a family of train pairs already
// states, read off the pair forms rather than searched for.
//
//! A transport class names what a vertex does; the ends name where it does
//! it; the face word between two ends is constant across the bundle or the
//! rule refuses. Each step reads what the pair forms already agree on.

use super::*;
use super::finish::{finish, multiplicity_demand};
pub use super::finish::assignment_key;
use crate::network::nerve::Keyed;

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
    let mut unreached: Vec<(Grid, Grid)> = Vec::new();
    for (i, (a, b)) in family.iter().enumerate() {
        trace::trace_n("extract/0 pair", i, budget);
        if !budget.spend() {
            unreached.push((a.clone(), b.clone()));
            continue;
        }
        match pair_form(crystal, budget, a, b, Some(scale)) {
            Some(pf) => pair_forms.push(pf),
            None => unreached.push((a.clone(), b.clone())),
        }
    }
    let n_pairs = pair_forms.len();
    trace::trace_n("extract/1 correspondence", n_pairs, budget);

    let mut class_pairs: Keyed<Vec<u16>> = Keyed::new();
    let mut hue_maps: Keyed<Vec<(u8, u8)>> = Keyed::new();
    let mut class_acts: Keyed<Keyed<crate::transducer::sentence::Acting>> = Keyed::new();
    let mut class_rep: Keyed<Vertex> = Keyed::new();
    let mut class_place_varies: Keyed<()> = Keyed::new();
    let mut chart_varies: Keyed<()> = Keyed::new();
    let mut chart_mult: Keyed<u32> = Keyed::new();
    let mut assign_seen: Keyed<()> = Keyed::new();
    let mut assign_n: Keyed<u32> = Keyed::new();
    let mut chart_incompatible: Keyed<()> = Keyed::new();
    let mut pair_member: Keyed<crate::transducer::sentence::Acting> = Keyed::new();

    for (pi, f) in pair_forms.iter().enumerate() {
        for (k, on) in f.by_transport.scoped() {
            match class_pairs.get_mut(&k) {
                Some(ps) => ps.push(pi as u16),
                None => {
                    class_pairs.insert(&k, vec![pi as u16]);
                }
            }
            for &i in on {
                let Some(v) = f.vertices.get(i as usize) else { continue };
                let Vertex::Word { derivation, acts, .. } = v else { continue };
                if !class_rep.contains(&k) {
                    class_rep.insert(&k, v.clone());
                }
                if !class_acts.contains(&k) {
                    class_acts.insert(&k, Keyed::new());
                }
                for (ck, a) in acts.scoped() {
                    if let Some(hk) = crate::transducer::sentence::hue_section_key_of(
                        &derivation.rotor(),
                        &a.gauge,
                    ) {
                        if !hue_maps.contains(&hk) {
                            hue_maps.insert(hk, a.gauge.clone());
                        }
                    }
                    let mut mk = k.clone();
                    crate::transducer::address::put_bytes(&mut mk, &ck);
                    let mut pk = mk.clone();
                    pk.extend_from_slice(&(pi as u16).to_le_bytes());
                    let mut ak = pk.clone();
                    crate::transducer::address::put_bytes(&mut ak, &assignment_key(a));
                    if !assign_seen.contains(&ak) {
                        assign_seen.insert(&ak, ());
                        let n = match assign_n.get_mut(&pk) {
                            Some(n) => {
                                *n = n.saturating_add(1);
                                *n
                            }
                            None => {
                                assign_n.insert(&pk, 1);
                                1
                            }
                        };
                        match chart_mult.get_mut(&mk) {
                            Some(m) => *m = (*m).max(n),
                            None => {
                                chart_mult.insert(&mk, n);
                            }
                        }
                    }
                    match pair_member.get_mut(&pk) {
                        None => {
                            pair_member.insert(&pk, a.clone());
                        }
                        Some(held) => {
                            if !element::same_versor(&held.carriage, &a.carriage) {
                                chart_incompatible.insert(&mk, ());
                            }
                            for &(from, to) in &a.gauge {
                                match held.gauge.iter().find(|&&(f, _)| f == from) {
                                    Some(&(_, t)) if t != to => {
                                        chart_incompatible.insert(&mk, ());
                                    }
                                    Some(_) => {}
                                    None => held.gauge.push((from, to)),
                                }
                            }
                        }
                    }
                    let Some(members) = class_acts.get_mut(&k) else { continue };
                    match members.get_mut(&ck) {
                        None => {
                            members.insert(&ck, a.clone());
                        }
                        Some(held) => {
                            let carriage_varies = !element::same_versor(&held.carriage, &a.carriage);
                            let joined = match (
                                crate::transducer::sentence::hue_table(&held.gauge),
                                crate::transducer::sentence::hue_table(&a.gauge),
                            ) {
                                (Some(h), Some(x)) => crate::transducer::sentence::hue_agreement(&h, &x),
                                _ => crate::transducer::sentence::HueAgreement::Apart { hue: 0, here: 0, there: 0 },
                            };
                            match joined {
                                crate::transducer::sentence::HueAgreement::Agreed(t) => {
                                    held.gauge = crate::transducer::sentence::hue_listing(&t);
                                    if carriage_varies {
                                        chart_varies.insert(&mk, ());
                                    }
                                }
                                crate::transducer::sentence::HueAgreement::Apart { .. } => {
                                    chart_varies.insert(&mk, ());
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let every_pair: Keyed<()> = match pair_forms.split_first() {
        None => Keyed::new(),
        Some((first, rest)) => rest.iter().fold(
            first.by_transport.meet(&first.by_transport),
            |acc, f| acc.meet(&f.by_transport),
        ),
    };

    let mut mult_charts: Keyed<u32> = Keyed::new();
    let mut mult_demands: Vec<(crate::transducer::sentence::Demand, Invariant, Vec<crate::transducer::sentence::Cell>)> = Vec::new();
    for (k, members) in class_acts.scoped() {
        for (ck, a) in members.scoped() {
            let mut mk = k.clone();
            crate::transducer::address::put_bytes(&mut mk, &ck);
            let Some(&multiplicity) = chart_mult.get(&mk) else { continue };
            if chart_incompatible.contains(&mk) {
                mult_charts.insert(&mk, multiplicity);
                if let Some(d) = multiplicity_demand(family, &a.chart, multiplicity) {
                    mult_demands.push(d);
                }
                continue;
            }
            if chart_varies.contains(&mk) {
                class_place_varies.insert(&k, ());
            }
        }
    }

    let pair_charts: Vec<Keyed<()>> = family
        .iter()
        .map(|(x, _)| {
            let mut charts = Keyed::new();
            for b in perceive(x) {
                charts.insert(b.moduli.key(), ());
            }
            charts
        })
        .collect();

    let matching_pairs_for = |k: &[u8], pairs: &[u16], acts: &Keyed<crate::transducer::sentence::Acting>| -> Vec<u16> {
        pairs
            .iter()
            .copied()
            .filter(|&pi| {
                acts.scoped().all(|(ck, a)| {
                    let mut pk = k.to_vec();
                    crate::transducer::address::put_bytes(&mut pk, &ck);
                    pk.extend_from_slice(&pi.to_le_bytes());
                    match pair_member.get(&pk) {
                        None => {
                            let has_chart = pair_charts
                                .get(pi as usize)
                                .is_some_and(|pc| pc.contains(&ck));
                            !has_chart
                        }
                        Some(pm) => {
                            element::same_versor(&a.carriage, &pm.carriage)
                                && crate::transducer::sentence::gauge_agrees(&a.gauge, &pm.gauge)
                        }
                    }
                })
            })
            .collect()
    };

    let mut sections: Keyed<Vec<u16>> = Keyed::new();
    let mut rule_vertices = vec![frame_vertex()];
    for (k, pairs) in class_pairs.scoped() {
        let (Some(Vertex::Word { address, derivation, contact, .. }), Some(members)) =
            (class_rep.get(&k), class_acts.get(&k))
        else {
            continue;
        };
        let mut acts: Keyed<crate::transducer::sentence::Acting> = Keyed::new();
        for (ck, a) in members.scoped() {
            let mut mk = k.clone();
            crate::transducer::address::put_bytes(&mut mk, &ck);
            if mult_charts.contains(&mk) {
                continue;
            }
            acts.insert(&ck, a.clone());
        }
        if acts.is_empty() {
            continue;
        }
        let section = class_place_varies.contains(&k) || !every_pair.contains(&k);
        if section {
            let ps = if class_place_varies.contains(&k) {
                matching_pairs_for(&k, pairs, &acts)
            } else {
                pairs.clone()
            };
            sections.insert(&k, ps);
        }
        for (ck, a) in acts.scoped() {
            let mut mk = k.clone();
            crate::transducer::address::put_bytes(&mut mk, &ck);
            let mps: Vec<u16> = pairs
                .iter()
                .copied()
                .filter(|&pi| {
                    let mut pk = mk.clone();
                    pk.extend_from_slice(&pi.to_le_bytes());
                    match pair_member.get(&pk) {
                        None => false,
                        Some(pm) => {
                            element::same_versor(&a.carriage, &pm.carriage)
                                && crate::transducer::sentence::gauge_agrees(&a.gauge, &pm.gauge)
                        }
                    }
                })
                .collect();
            if mps.len() < n_pairs {
                sections.insert(&mk, mps);
            }
        }

        let v = Vertex::Word {
            address: address.clone(),
            derivation: derivation.clone(),
            acts,
            contact: *contact,
        };
        rule_vertices.push(v);
    }
    let mut form = Form {
        vertices: rule_vertices,
        faces: Vec::new(),
        loops: Vec::new(),
        remainder: crate::transducer::element::PlacedChain::default(),
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
        by_transport: crate::network::nerve::Keyed::new(),
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
    for (_, (ea, eb)) in candidates.scoped() {
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached, &mult_demands);
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
        if words.is_empty() {
            continue;
        }
        // The face carries its word and edge transport from the pair forms.
        let edge = pair_forms.iter().find_map(|pf| face_edge_between(pf, ea, eb));
        let Some(word) = form.face_word(i, j) else {
            continue;
        };
        // THE PAIRS THIS FACE'S WORD WAS MEASURED ON, ASKED BY THAT WORD'S
        // OWN KEY. The rule's face carries `w_j ∘ w_i⁻¹` off its two
        // vertices, and `words` holds each word the BUNDLE spoke between
        // these ends with the pairs it spoke it on. Which list belongs to
        // this face is therefore a keyed question, and the answer used to
        // be `words[0]` — the first entry of a list, taken for a face built
        // from a different word source (rule 15). A word the bundle did not
        // speak here is measured on no pair and the face says so, rather
        // than inheriting a claim it cannot support (rule 11).
        let measured = |w: &crate::transducer::alphabet::Word| -> Vec<u16> {
            match at.get(w.key().as_bytes()).and_then(|&n| words.get(n)) {
                Some((_, ps)) => ps.clone(),
                None => Vec::new(),
            }
        };
        let claimed = measured(&word);
        let Some(mut face) = form.face_carrying_with_edge(i, j, word.clone(), edge.clone())
        else {
            continue;
        };
        face.pairs = Some(claimed);
        form.faces.push(face);
        // EVERY OTHER WORD THE BUNDLE SPOKE between these two ends is a
        // second face between the same vertices — two carriages on one
        // class are two sections (18.2) — each carrying the pairs it was
        // spoken on. This walked `words[1..]`, so a bundle whose first
        // spoken word is the rule's own coboundary lost nothing, and one
        // whose first spoken word is NOT lost that word's face entirely.
        for (w, ps) in &words {
            if w.key() == word.key() {
                continue;
            }
            let Some(mut section) = form.face_carrying_with_edge(i, j, w.clone(), edge.clone())
            else {
                continue;
            };
            section.pairs = Some(ps.clone());
            form.faces.push(section);
        }
    }
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    form.reindex();
    trace::trace_n("extract/3 glue", form.faces.len(), budget);
    // The carriages as the pair forms measured them, by address, so the
    // three steps that alter a vertex can each be asked whether they did.
    let measured = trace::carriage_of(&form);
    // 3. Glue: placement is the relative word Frame → class, constant across the bundle.
    // Crystal is indexed by the section coordinate (moduli); correspondence is transport class.
    // EVERY CHART THE CLASS ACTS ON IS OFFERED TO THE CRYSTAL, because the
    // crystal is indexed by the section coordinate and a vertex acting on two
    // charts has two bodies for the mind to recognise. `class_of_vertex`
    // answers one chart — the least member's — so a held sentence for the
    // second body was never asked for.
    let glued_classes: Vec<(TransportClass, Invariant)> = form
        .vertices
        .iter()
        .filter_map(|v| {
            let tc = transport_class(v)?;
            let acts = v.acts()?;
            Some(
                acts.scoped()
                    .map(|(_, a)| (tc.clone(), a.chart.clone()))
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect();
    for (tc, moduli) in glued_classes {
        // The glue is what the mind already holds; a budget that ends here
        // leaves the form it has, ungLued, rather than unsaying the pairs
        // it measured to get here.
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached, &mult_demands);
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
    trace::trace_carriage("extract/3 glue", &measured, &form);
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
    trace::trace_carriage("extract/4 rebase", &measured, &form);
    let ctx = contact::ContactContext {
        class_place_varies: &class_place_varies,
        class_pairs: &class_pairs,
        pair_member: &pair_member,
        pair_charts: &pair_charts,
        n_pairs,
    };
    if !contact::contact_named_vertices(&mut form, family, &ctx, &mut sections, budget) {
        return finish(form, &pair_forms, scale, sections, &unreached, &mult_demands);
    }
    trace::trace_carriage("extract/6 contact", &measured, &form);
    trace::trace_n("extract/7 coherence", form.loops.len(), budget);
    // 7. Coherence: tree gauge, else hold the loops.
    if !form.coheres() {
        if let Some(gamma) = form.solve_coboundary() {
            let _ = form.regauge(&gamma);
        }
    }
    trace::trace("extract/8 finish", budget);
    finish(form, &pair_forms, scale, sections, &unreached, &mult_demands)
}

/// Extract the invariant cochain of a family.
pub fn extract_rule(
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal, budget)
}
