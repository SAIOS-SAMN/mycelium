// 21 steps 1-7 — EXTRACTION: the rule a family of train pairs already
// states, read off the pair forms rather than searched for.
//
//! A transport class names what a vertex does; the ends name where it does
//! it; the face word between two ends is constant across the family or the
//! rule refuses. Nothing here ranks candidates: each step reads what the
//! pair forms already agree on.

use super::*;

pub(crate) fn class_of_vertex(v: &Vertex) -> Option<Invariant> {
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
pub(crate) struct TransportClass {
    kind: crate::transducer::peel::ShellKind,
    word_class: Vec<u8>,
}

impl TransportClass {
    pub(crate) fn key(&self) -> Vec<u8> {
        let mut buf = vec![self.kind.as_u8()];
        crate::transducer::a21::put_bytes(&mut buf, &self.word_class);
        buf
    }
}

pub(crate) fn transport_class(v: &Vertex) -> Option<TransportClass> {
    match v {
        Vertex::Word { word, .. } => Some(TransportClass {
            kind: crate::transducer::peel::ShellKind::Whole,
            word_class: word.class_key()?.into_vec(),
        }),
        _ => None,
    }
}

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum End {
    Frame,
    Class(TransportClass),
}

pub(crate) fn end_of(v: &Vertex) -> Option<End> {
    match v {
        Vertex::Frame => Some(End::Frame),
        Vertex::Word { .. } => Some(End::Class(transport_class(v)?)),
        _ => None,
    }
}

pub(crate) fn one_vertex_of_class<'a>(f: &'a Form, class: &TransportClass) -> Option<&'a Vertex> {
    match_vertex(f, class)
}

pub(crate) fn match_vertex<'a>(f: &'a Form, class: &TransportClass) -> Option<&'a Vertex> {
    f.vertices
        .iter()
        .find(|v| transport_class(v).as_ref() == Some(class))
}

/// The unique relative word between two ends on a pair form, read from
/// that form's named faces. Several instances of a class are a section:
/// every such face must be one M1 vertex.
pub(crate) fn face_word_between(pf: &Form, a: &End, b: &End) -> Option<crate::transducer::alphabet::Word> {
    let mut held: Option<crate::transducer::alphabet::Word> = None;
    for f in &pf.faces {
        let ea = end_of(pf.vertices.get(f.i as usize)?)?;
        let eb = end_of(pf.vertices.get(f.j as usize)?)?;
        let (w, ok) = if &ea == a && &eb == b {
            (f.transport.word.clone(), true)
        } else if &ea == b && &eb == a {
            (f.transport.word.inverse(), true)
        } else {
            (crate::transducer::alphabet::Word::identity(), false)
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
pub(crate) fn rebase_named_faces(form: &mut Form, pair_forms: &[Form]) {
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

pub(crate) fn word_place_of_class(
    pf: &Form,
    tc: &TransportClass,
) -> Option<(crate::transducer::alphabet::Word, Versor)> {
    match match_vertex(pf, tc) {
        Some(Vertex::Word { word, carriage, .. }) => Some((word.clone(), carriage.clone())),
        _ => None,
    }
}

pub(crate) fn placement_of_class(pair_forms: &[Form], class: &TransportClass) -> Option<crate::transducer::alphabet::Word> {
    let mut held: Option<crate::transducer::alphabet::Word> = None;
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
) -> Option<crate::transducer::sentence::SentenceRule> {
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
                if let Some(hk) = crate::transducer::sentence::hue_section_key(word, gauge) {
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
        let mut held: Option<crate::transducer::alphabet::Word> = None;
        let mut constant = true;
        for pf in &pair_forms {
            match face_word_between(pf, ea, eb) {
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
                        drop.insert(crate::transducer::sentence::vertex_ix_key(i as u16), ());
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
        form.unfold(crate::transducer::sentence::Measure::Occupied);
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
                    *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
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
    Some(crate::transducer::sentence::rule_of_form(form))
}

pub(crate) fn pair_idx_key(i: u16) -> Vec<u8> {
    let mut k = vec![0u8];
    k.extend_from_slice(&i.to_le_bytes());
    k
}

pub(crate) fn end_key(e: &End) -> Vec<u8> {
    match e {
        End::Frame => vec![0xff],
        End::Class(c) => c.key(),
    }
}

pub(crate) fn ends_pair_key(a: &End, b: &End) -> Vec<u8> {
    let (ka, kb) = (end_key(a), end_key(b));
    let (lo, hi) = if ka <= kb { (ka, kb) } else { (kb, ka) };
    let mut buf = Vec::new();
    crate::transducer::a21::put_bytes(&mut buf, &lo);
    crate::transducer::a21::put_bytes(&mut buf, &hi);
    buf
}

pub(crate) fn index_ends(form: &Form) -> crate::network::nerve::Keyed<u16> {
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

pub(crate) fn hue_maps_of_form(form: &Form) -> crate::network::nerve::Keyed<Vec<(u8, u8)>> {
    let mut m = form.hue_maps.clone();
    for v in &form.vertices {
        if let Vertex::Word { word, gauge, .. } = v {
            if let Some(k) = crate::transducer::sentence::hue_section_key(word, gauge) {
                if !m.contains(&k) {
                    m.insert(k, gauge.clone());
                }
            }
        }
    }
    m
}

/// Re-derive the family's rule and check it agrees with `rule`.
pub fn rule_replays_family(rule: &crate::transducer::sentence::SentenceRule, family: &[(Grid, Grid)]) -> bool {
    let Some(again) = extract_rule(family) else {
        return false;
    };
    again.form.vertices == rule.form.vertices && again.form.faces == rule.form.faces
}

/// Extract, memoized by family id and working-atlas generation. A hit
/// spends nothing; a miss spends one Budget and stores the rule.
pub fn extract_rule_cached(
    cache: &mut crate::network::nerve::Keyed<(crate::transducer::a21::Address, Option<crate::transducer::sentence::SentenceRule>)>,
    fam: &[u8],
    family: &[(Grid, Grid)],
    atlas_root: crate::transducer::a21::Address,
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
    let rule = extract_rule_with(family, crystal);
    cache.insert(fam, (atlas_root, rule.clone()));
    rule
}

/// Extract the invariant cochain of a family. The live path is the form
/// engine (`extract_rule_with`); the nerve extractor is retired.
pub fn extract_rule(family: &[(Grid, Grid)]) -> Option<crate::transducer::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal)
}
