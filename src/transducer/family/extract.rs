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
        let (Some(vi), Some(vj)) = (form.vertices.get(i), form.vertices.get(j)) else {
            continue;
        };
        let (Vertex::Word { derivation: di, .. }, Vertex::Word { .. }) = (vi, vj) else {
            continue;
        };
        let di = di.clone();
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
        // THE RELATIVE IS READ PER CHART PAIR, at the pairs THIS FACE relates
        // (D10 item 9). It was `vi.carriage_of()` against `vj.carriage_of()` —
        // two representatives, which on a face between multi-chart vertices
        // compares one body's placement against a body on a different chart and
        // calls the difference the relative.
        //
        // A rebase asserts ONE relative for the whole vertex, so it fires only
        // where every pair this face relates agrees on it AND every pair form
        // measures that same relative between the two classes. Where the pairs
        // disagree there is no one relative to rebase by (rule 11).
        let mut relative: Option<Versor> = None;
        let mut agree = true;
        for (pk, _) in face.at.scoped() {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(cci), Some(ccj)) = (vi.chart_keyed(&ik), vj.chart_keyed(&jk)) else {
                agree = false;
                break;
            };
            let (Some(a), Some(b)) = (vi.acting_on(&cci), vj.acting_on(&ccj)) else {
                agree = false;
                break;
            };
            let Some(inv) = a.carriage.versor_inverse() else {
                agree = false;
                break;
            };
            let here = b.carriage.geo(&inv);
            match &relative {
                None => relative = Some(here),
                Some(held) if !element::same_versor(held, &here) => {
                    agree = false;
                    break;
                }
                Some(_) => {}
            }
        }
        let (true, Some(relative)) = (agree, relative) else {
            continue;
        };
        // EVERY PAIR FORM MEASURES THE SAME RELATIVE, at every member pair its
        // own face between the two classes records. The incidence is the pair
        // form's face, so the bodies compared are the bodies the scene relates
        // and never a product of the two classes' placements.
        let constant = pair_forms.iter().all(|pf| {
            let rs = relatives_between(pf, &tc_i, &tc_j);
            !rs.is_empty()
                && rs
                    .scoped()
                    .all(|(_, (_, r))| element::same_versor(r, &relative))
        });
        if constant {
            let _ = form.rebase(face.i, face.j, relative, fi as u16);
            trace::trace_at("extract/4 rebased", j);
        }
    }
    trace::trace_at("extract/4 reindex", form.vertices.len());
    form.reindex();
    for (_, &fi) in form.star.scoped() {
        let Some(face) = form.faces.get(fi as usize) else {
            continue;
        };
        let (i, j) = (face.i, face.j);
        trace::trace_at("extract/4 star-face", fi as usize);
        // THE RE-DERIVATION KEEPS THE MEMBER PAIRS THE FACE ALREADY CARRIES
        // and keeps their edges: the rebase moved the vertices' placements, so
        // the carriage at each pair is stale, but WHICH pairs this face relates
        // is the scene's fact and re-deriving it from the vertices would take
        // the product of their fibres. The pairs are read off the face and the
        // placement at each is re-derived at its own coordinate.
        let pairs: Vec<(Vec<u8>, Option<crate::transducer::sentence::Transport>)> = face
            .at
            .scoped()
            .map(|(k, p)| (k, p.edge.clone()))
            .collect();
        let mut at = crate::network::nerve::Keyed::new();
        for (pk, edge) in pairs {
            let Some((ik, jk)) = crate::transducer::sentence::read_face_pair_key(&pk) else {
                continue;
            };
            let (Some(ci), Some(cj)) = (
                form.vertices.get(i as usize).and_then(|v| v.chart_keyed(&ik)),
                form.vertices.get(j as usize).and_then(|v| v.chart_keyed(&jk)),
            ) else {
                continue;
            };
            if let Some(p) = form.face_place(i, &ci, j, &cj, edge) {
                at.insert(&pk, p);
            }
        }
        if let Some(word) = form.face_word(i, j) {
            form.faces[fi as usize].word = word;
            form.faces[fi as usize].at = at;
        }
        trace::trace_at("extract/4 star-face-done", fi as usize);
    }
}


/// AN ASSIGNMENT'S OWN KEY: the carriage's bytes and the sorted gauge, which
/// is what the member acts BY. Content, so two encodings of one assignment
/// are one key (law 2), and sorted for the same reason `form_vertex_key`
/// sorts it — a gauge is a map and not a list.
pub(super) fn assignment_key(a: &crate::transducer::sentence::Acting) -> Vec<u8> {
    let mut buf = Vec::new();
    // A4 output (17.2): raw coefficients made a carriage and its negation two assignments.
    a.carriage.put_canonical(&mut buf);
    let mut g = a.gauge.clone();
    g.sort_unstable();
    buf.extend_from_slice(&(g.len() as u16).to_le_bytes());
    for (x, y) in g {
        buf.push(x);
        buf.push(y);
    }
    buf
}

/// 50.5 — THE DEMAND A MULTIPLICITY NAMES: `Source { multiplicity }` on the
/// chart the bodies stand on, carrying every one of their cells with the hue
/// the pair asks of it. "Every one standing as a section, none first-found."
///
/// `None` when the demand would carry nothing — no body of that chart on any
/// pair, or no cell whose hue the pair changes. A demand with no cells stands
/// for nothing and `Demand::typed` would be the only thing holding it up.
///
/// The bodies of one grid are walked because `perceive` is their producer and
/// there is no index over them; this is that carrier's own boundary, and it is
/// the same read the court makes in `fiber_on`.
fn multiplicity_demand(
    family: &[(Grid, Grid)],
    chart: &Invariant,
    multiplicity: u32,
) -> Option<(
    crate::transducer::sentence::Demand,
    Invariant,
    Vec<crate::transducer::sentence::Cell>,
)> {
    let key = chart.key();
    let mut cells: Vec<crate::transducer::sentence::Cell> = Vec::new();
    for (x, y) in family {
        for body in perceive(x) {
            if body.moduli.key() != key {
                continue;
            }
            for &(r, c, h) in &body.cells {
                // The hue the pair asks of this cell. A cell outside the
                // output frame is asked for nothing, and hue 0 IS absence in
                // this encoding — `residual_of`'s own reading.
                let (Ok(ur), Ok(uc)) = (usize::try_from(r), usize::try_from(c)) else {
                    continue;
                };
                let observed = match y.get(ur).and_then(|row| row.get(uc)) {
                    Some(h) => *h,
                    None => 0,
                };
                if observed != h {
                    cells.push((r, c, h, observed));
                }
            }
        }
    }
    cells.sort_unstable();
    cells.dedup();
    if cells.is_empty() {
        return None;
    }
    let kind = crate::transducer::sentence::Demand::Source { multiplicity };
    kind.typed().ok()?;
    Some((kind, chart.clone(), cells))
}

/// The germ of one pair: what the input carries onto the output, as the
/// cells of the change. `None` for a pair with no scene.
fn germ_of_pair(a: &Grid, b: &Grid) -> Option<crate::transducer::element::PlacedChain> {
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
/// `multiplicities` are the demands correspondence itself raised (D8, 50.5):
/// a transport class carrying several incompatible assignments under one
/// chart, which no coordinate available to the matching selects among. They
/// union into `by_demand` beside the pair forms' own, under the same key, so
/// the rule still carries one vertex per (kind, chart) over the bundle.
fn finish(
    mut form: Form,
    pair_forms: &[Form],
    scale: ((usize, usize), (usize, usize)),
    sections: crate::network::nerve::Keyed<Vec<u16>>,
    unreached: &[(Grid, Grid)],
    multiplicities: &[(
        crate::transducer::sentence::Demand,
        Invariant,
        Vec<crate::transducer::sentence::Cell>,
    )],
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
    for (kind, chart, cells) in multiplicities {
        let key = crate::transducer::sentence::demand_key(kind, chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend_from_slice(cells),
            None => {
                by_demand.insert(key, (kind.clone(), chart.clone(), cells.clone()));
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
                by_demand.insert(key, (kind, chart, germ.to_vec()));
            }
        }
    }
    for (_, (kind, chart, cells)) in by_demand.scoped() {
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
    let mut hue_maps: crate::network::nerve::Keyed<Vec<(u8, u8)>> = crate::network::nerve::Keyed::new();
    // EVERY CORRESPONDING BODY'S MEMBER, filed under the class by its chart,
    // READ THROUGH EACH PAIR FORM'S TRANSPORT FIBRE and not by walking its
    // vertices. `reindex` is the indexer (D9 step 1); this is a reader.
    //
    // Two faults have stood here in turn. `one_vertex_of_class` —
    // `vertices.iter().find(...)` on the least pair — dropped the second body
    // of a class that spans two charts, which is `05f2a901`. D7b replaced it
    // with a walk of every pair form's vertices accumulating the
    // correspondence, which is a SECOND PRODUCER of a fact the indexer's job
    // it is: "once" is not the criterion, "by the indexer" is (law 12, 13.1).
    //
    // Now every vertex reached here was ASKED FOR by its class key. A vertex
    // of no transport class is not passed over silently — it is not in the
    // relation at all — and the classes arrive grouped, which is the shape
    // correspondence wants.
    //
    // What each later step reads: `class_acts` for the members, `class_rep`
    // for the vertex's identity, `class_place_varies` for step 6.
    let mut class_acts: crate::network::nerve::Keyed<
        crate::network::nerve::Keyed<crate::transducer::sentence::Acting>,
    > = crate::network::nerve::Keyed::new();
    let mut class_rep: crate::network::nerve::Keyed<Vertex> = crate::network::nerve::Keyed::new();
    // THE PLACEMENT VARIES WHEN ONE CHART'S BODY IS PLACED TWO WAYS ON TWO
    // PAIRS — which is the question step 6 asks, and the only variation a
    // contact ring can answer. Two distinctions have to be kept apart here
    // and collapsing them is a fault either way:
    //
    //   across PAIRS at one chart — the rule cannot state one carriage for
    //     the bundle, and a stop ring may be what places it instead;
    //   across BODIES at one chart within one pair — two bodies of one shape
    //     placed differently, which is a MULTIPLICITY and not variation at
    //     all. It is consistent on every pair and no ring explains it.
    //
    // Step 6 used to read the first by comparing the rule vertex's carriage
    // against each pair's FIRST vertex of the class, which answers the wrong
    // question once a class spans several charts: it compares one body's
    // placement against a different body's. Reading it per chart is the fix —
    // but per chart WITHOUT the multiplicity kept apart turns every
    // multiplicity into variation, and
    // `two_hues_that_meet_are_two_bodies_with_a_contact_face` is exactly
    // that: two bars of one shape, one moving and one still, the same way on
    // both pairs. It refused with `orbit does not move`, because a vertex
    // whose rotor is the identity has no orbit for a ring to carry.
    //
    // Both facts are recorded per member and the DECISION IS TAKEN AFTER THE
    // WALK, so it does not depend on which pair the walk met first: a chart
    // some pair carries several incompatible assignments on is a MULTIPLICITY
    // and makes no claim about variation, because "the first body of this
    // chart on pair p" is identified with "the first on pair q" by nothing but
    // position (rule 15).
    let mut class_place_varies: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut chart_varies: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // 50.5 — HOW MANY ASSIGNMENTS STAND AT ONE CHART WITHIN ONE PAIR, as the
    // running maximum over the bundle. Keyed by class ++ chart.
    //
    // Two bodies of one chart carrying the SAME assignment are not a
    // multiplicity: one member speaks a whole fibre, which is what
    // `atlas.fiber(chart)` is for. Two carrying DIFFERENT assignments are
    // several incompatible assignments and no coordinate the correspondence
    // has selects among them — the chart is what it matches by and the chart
    // is the same. That is the demand of D8.
    //
    // The assignment's own key is its content (law 2): the carriage's bytes
    // and the sorted gauge, so two encodings of one assignment are one key.
    let mut chart_mult: crate::network::nerve::Keyed<u32> = crate::network::nerve::Keyed::new();
    let mut assign_seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut assign_n: crate::network::nerve::Keyed<u32> = crate::network::nerve::Keyed::new();
    // The (class, chart) coordinates whose assignments cannot be reconciled,
    // and the pair that first wrote each member, so "within one pair" and
    // "across pairs" stay two different readings.
    let mut chart_incompatible: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // The assignment one pair reconciles at one chart, keyed by (class, chart,
    // pair): the carrier that keeps "within one pair" apart from "across the
    // bundle" without either reading being positional.
    let mut pair_member: crate::network::nerve::Keyed<crate::transducer::sentence::Acting> =
        crate::network::nerve::Keyed::new();
    for (pi, f) in pair_forms.iter().enumerate() {
        // THE UNION, class by class: each pair form's fibre contributes the
        // pairs it is present on. One entry per class per pair form, so the
        // `seen` guard D7b needed against repeated vertices is gone with the
        // walk that needed it.
        for (k, on) in f.by_transport.scoped() {
            match class_pairs.get_mut(&k) {
                Some(ps) => ps.push(pi as u16),
                None => {
                    class_pairs.insert(&k, vec![pi as u16]);
                }
            }
            for &i in on {
                let Some(v) = f.vertices.get(i as usize) else {
                    continue;
                };
                let Vertex::Word { derivation, acts, .. } = v else {
                    continue;
                };
                if !class_rep.contains(&k) {
                    class_rep.insert(&k, v.clone());
                }
                if !class_acts.contains(&k) {
                    class_acts.insert(&k, crate::network::nerve::Keyed::new());
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
                    // The member's own coordinate: the class it corresponds
                    // by, and the chart it acts on; and that coordinate at
                    // one pair, which is where multiplicity shows.
                    let mut mk = k.clone();
                    crate::transducer::address::put_bytes(&mut mk, &ck);
                    let mut pk = mk.clone();
                    pk.extend_from_slice(&(pi as u16).to_le_bytes());
                    // THE DISTINCT ASSIGNMENTS AT THIS CHART IN THIS PAIR,
                    // counted by their own content and carried as the running
                    // maximum over the bundle.
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
                    // WITHIN THIS PAIR FIRST. A second body of this chart on
                    // THIS pair either reconciles with the first or it is the
                    // multiplicity; that is the boundary D7b drew and it is
                    // the pair index that draws it, so the reconciliation is
                    // keyed by (class, chart, pair) and not by (class, chart).
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
                    let Some(members) = class_acts.get_mut(&k) else {
                        continue;
                    };
                    // ACROSS THE BUNDLE. A differing carriage here is step 6's
                    // question — the placement is not family-constant and a
                    // stop ring may be what places it — and NOT a
                    // multiplicity; a differing gauge is a hue SECTION, kept
                    // on `Form.hue_maps` where both already go, and not a
                    // multiplicity either. That is the boundary: variation
                    // across pairs is a section, multiplicity within a pair
                    // under one chart is a demand.
                    //
                    // THE HUE MAPS UNION hue by hue, because a gauge is a
                    // PARTIAL map and absence is the identity (18.2) — which
                    // is how `emit_carried` applies it: "a hue the map does not
                    // move keeps its own colour". `[(1,4)]` on one body and
                    // `[(9,9)]` on another are not two irreconcilable claims;
                    // one member carrying `[(1,4),(9,9)]` speaks both. The
                    // held entry stands where the two send one hue two ways,
                    // exactly as it did before, and `hue_maps` carries both
                    // sections.
                    match members.get_mut(&ck) {
                        None => {
                            members.insert(&ck, a.clone());
                        }
                        Some(held) => {
                            // AN ASSIGNMENT IS THE CARRIAGE AND THE GAUGE, and
                            // this asked only the carriage — one coordinate of
                            // a two-coordinate object, against the definition
                            // stated at `assign_seen` above: "the assignment's
                            // own key is its content (law 2): the carriage's
                            // bytes AND the sorted gauge".
                            //
                            // The gauge was merged by walking `a.gauge` and
                            // pushing every hue `held` did not already name.
                            // Where both name one hue and send it two ways,
                            // the second was DROPPED and no variance was
                            // recorded — so the vertex went on to claim one
                            // map across every pair and the court found the
                            // other colour. Measured over 36 minutes: 880 `a
                            // cell lands on the wrong hue`, the largest single
                            // vertex-body fault.
                            //
                            // Worse, the drop is what hid it. `hue_of` refuses
                            // a listing that sends one hue two ways — the
                            // court's `FAIL_GAUGE_AMBIGUOUS` — and the merge
                            // deduped on `from`, so that listing was never
                            // built. The court saw an unambiguous WRONG map
                            // instead of an ambiguous one, and the guard read
                            // 0 on the wire: not because gauges agree but
                            // because the disagreement could not reach it.
                            //
                            // Two gauges COMPOSE where their hues do not meet
                            // or where they meet and agree — one member speaks
                            // both listings, which is what the union was
                            // written for and remains right. Where they meet
                            // and DISAGREE they are two assignments, and the
                            // class varies across the bundle exactly as it
                            // does when the carriage differs. That is the
                            // boundary this block already states for the
                            // placement.
                            let carriage_varies =
                                !element::same_versor(&held.carriage, &a.carriage);
                            let joined = match (
                                crate::transducer::sentence::hue_table(&held.gauge),
                                crate::transducer::sentence::hue_table(&a.gauge),
                            ) {
                                (Some(h), Some(x)) => {
                                    crate::transducer::sentence::hue_agreement(&h, &x)
                                }
                                // A listing that is no map on either side is
                                // not something to reconcile (rule 11).
                                _ => crate::transducer::sentence::HueAgreement::Apart {
                                    hue: 0,
                                    here: 0,
                                    there: 0,
                                },
                            };
                            match joined {
                                crate::transducer::sentence::HueAgreement::Agreed(t) => {
                                    held.gauge = crate::transducer::sentence::hue_listing(&t);
                                    if carriage_varies {
                                        chart_varies.insert(&mk, ());
                                    }
                                }
                                crate::transducer::sentence::HueAgreement::Apart { .. } => {
                                    // The held listing stands unchanged — the
                                    // arriving one is not merged into it,
                                    // because the two are not one map — and
                                    // the class is marked varying so the pair
                                    // filter sections them apart.
                                    chart_varies.insert(&mk, ());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // THE MEET: the classes EVERY pair form carries (13.1, A13). `Keyed::meet`
    // is simultaneous PATRICIA descent — subtrees sharing no prefix are pruned
    // and the cost is the shared structure, never a product of the two fibres
    // — so this is COMPUTED and not enumerated.
    //
    // It replaces `all_pairs.scoped().any(|pk| !have.contains(&pk))`, which
    // built a keyed set of every pair index and asked, per class, whether some
    // index was missing from a second keyed set. That is the overlap
    // enumerated, and its `all_pairs`/`have` pair existed for nothing else.
    let every_pair: crate::network::nerve::Keyed<()> = match pair_forms.split_first() {
        None => crate::network::nerve::Keyed::new(),
        // The first fibre's own keys as a `Keyed<()>`: the meet of a carrier
        // with itself is its key set, which is the seed the fold needs and is
        // the same descent as every other step of it.
        Some((first, rest)) => rest.iter().fold(
            first.by_transport.meet(&first.by_transport),
            |acc, f| acc.meet(&f.by_transport),
        ),
    };
    // THE DECISIONS, after the walk, so neither depends on which pair the walk
    // met first.
    //
    // A class's PLACEMENT VARIES where some chart of it was placed two ways
    // over the bundle and that chart carries ONE assignment per pair. A chart
    // with several is a multiplicity and says nothing about variation.
    //
    // A chart with several incompatible assignments in one pair is a
    // MULTIPLICITY THE CORRESPONDENCE CANNOT RESOLVE (50.5, D8): every one of
    // them stands as a section and none is first-found, so the form says so
    // with a demand and DOES NOT KEEP ONE AND SPEAK IT. `mult_charts` is the
    // (class, chart) coordinates that fall out of the fibre for that reason.
    let mut mult_charts: crate::network::nerve::Keyed<u32> = crate::network::nerve::Keyed::new();
    let mut mult_demands: Vec<(
        crate::transducer::sentence::Demand,
        Invariant,
        Vec<crate::transducer::sentence::Cell>,
    )> = Vec::new();
    for (k, members) in class_acts.scoped() {
        for (ck, a) in members.scoped() {
            let mut mk = k.clone();
            crate::transducer::address::put_bytes(&mut mk, &ck);
            // Rule 11: the count is what the walk recorded, and a chart the
            // walk recorded nothing for carried no assignment at all — which
            // is not a multiplicity of zero, it is a chart this loop cannot
            // have reached, since `class_acts` and `chart_mult` are written
            // from the same member.
            let Some(&multiplicity) = chart_mult.get(&mk) else {
                continue;
            };
            // THE COUNT DOES NOT DECIDE (rule 2). Several assignments at one
            // chart are a multiplicity only where they cannot be reconciled;
            // where they can, the member above already carries their union and
            // speaks every body. The count is what the demand NAMES once the
            // incompatibility has decided that there is one.
            if chart_incompatible.contains(&mk) {
                mult_charts.insert(&mk, multiplicity);
                // The demand names the CLASS by the chart it stands on — which
                // is the coordinate 50.5 files a demand under — and the
                // multiplicity it could not resolve. Its bodies are the cells
                // of the bodies on that chart, with the hue each pair asks of
                // them, so the demand carries what it stands for and is not a
                // claim with nothing in it.
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
    let matching_pairs_for = |k: &[u8], pairs: &[u16], acts: &crate::network::nerve::Keyed<crate::transducer::sentence::Acting>| -> Vec<u16> {
        pairs
            .iter()
            .copied()
            .filter(|&pi| {
                // EVERY CHART THE VERTEX ACTS ON, not any of them. This was
                // `any`, so a pair was kept whenever ONE of the vertex's
                // charts matched there — and the vertex then acts on ALL its
                // charts over that pair, including the ones whose carriage
                // does not match. Forcing an unmatched carriage onto a cell
                // is what carries it out of the image: measured over 36
                // minutes, 443 `past the image frame's extent` and 347
                // `before the image frame's origin`, together the second
                // fault after the hue.
                //
                // A section says the pairs it was MEASURED on. A pair where
                // one chart agrees and another does not is not a pair this
                // vertex was measured on; it is a pair it would have to
                // guess at.
                //
                // AND THE WHOLE ASSIGNMENT, not the carriage alone. `Acting`
                // carries a carriage AND a gauge and `pair_member` holds
                // both; comparing one of them admits pairs that agree on the
                // placement and demand a different hue map, which is a hue
                // clash guaranteed at replay.
                acts.scoped().all(|(ck, a)| {
                    let mut pk = k.to_vec();
                    crate::transducer::address::put_bytes(&mut pk, &ck);
                    pk.extend_from_slice(&pi.to_le_bytes());
                    match pair_member.get(&pk) {
                        // A chart this pair has no member for is a chart the
                        // vertex does not act on there, and it contradicts
                        // nothing (the `Silent` reading the court makes).
                        None => true,
                        Some(pm) => {
                            element::same_versor(&a.carriage, &pm.carriage)
                                && crate::transducer::sentence::gauge_eq(&a.gauge, &pm.gauge)
                        }
                    }
                })
            })
            .collect()
    };
    let mut sections: crate::network::nerve::Keyed<Vec<u16>> = crate::network::nerve::Keyed::new();
    let mut rule_vertices = vec![frame_vertex()];
    for (k, pairs) in class_pairs.scoped() {
        // ONE VERTEX PER CLASS, ACTING ON EVERY CHART THE CLASS REACHES.
        // The identity — address, derivation, contact — is the class's
        // representative, which is the least pair's first vertex of the
        // class and therefore the same on every node; the FIBRE is the union
        // over the bundle, filed by chart, so nothing the correspondence saw
        // is dropped for arriving second.
        let (Some(Vertex::Word { address, derivation, contact, .. }), Some(members)) =
            (class_rep.get(&k), class_acts.get(&k))
        else {
            continue;
        };
        // A CHART WHOSE MULTIPLICITY THE CORRESPONDENCE COULD NOT RESOLVE IS
        // NOT IN THE FIBRE. The demand above stands for it; keeping a member
        // here would speak one of several assignments and assert it for the
        // rest, which is the first-found 50.5 forbids.
        let mut acts: crate::network::nerve::Keyed<crate::transducer::sentence::Acting> =
            crate::network::nerve::Keyed::new();
        for (ck, a) in members.scoped() {
            let mut mk = k.clone();
            crate::transducer::address::put_bytes(&mut mk, &ck);
            if mult_charts.contains(&mk) {
                continue;
            }
            acts.insert(&ck, a.clone());
        }
        // A CLASS LEFT WITH NO CHART SPEAKS NO WORD. Its whole content was
        // multiplicities, the demands carry them, and a word vertex acting on
        // nothing would be a vertex that reconstructs nothing while claiming
        // to be a word (2.2, 50.1: a sentence with demands is valid, never
        // closed).
        if acts.is_empty() {
            continue;
        }
        // THE DIFFERENCE IS THE SECTION: a class in the union and not in the
        // meet is a class some pair form does not carry, and the rule states
        // the pairs it was measured on rather than claiming the rest. So is a
        // class one of whose charts was placed two ways over the bundle. The
        // court asks a sectioned vertex for exactly those pairs
        // (`vertices_carry_family`).
        //
        // A class IN the meet makes the unrestricted claim, which is what
        // every class on every pair has always made here. Every class still
        // gets a vertex — the meet decides whether its claim is restricted,
        // not whether the rule says anything about it at all: 18.2 keeps a
        // section AS a section, and a class with no vertex would leave the
        // pairs it was measured on unspoken with nothing recording it.
        let section = class_place_varies.contains(&k) || !every_pair.contains(&k);
        if section {
            let ps = if class_place_varies.contains(&k) {
                matching_pairs_for(&k, pairs, &acts)
            } else {
                pairs.clone()
            };
            sections.insert(&k, ps);
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
        let mut sections_of_face = words.into_iter();
        let Some((_, first_pairs)) = sections_of_face.next() else {
            continue;
        };
        // The face carries its word and edge transport from the pair forms.
        let edge = pair_forms.iter().find_map(|pf| face_edge_between(pf, ea, eb));
        let Some(word) = form.face_word(i, j) else {
            continue;
        };
        let Some(face) = form.face_carrying_with_edge(i, j, word, edge.clone()) else {
            continue;
        };
        form.faces.push(face);
        let mut rest = sections_of_face.peekable();
        if rest.peek().is_some() {
            sections.insert(ends_pair_key(ea, eb), first_pairs);
        }
        for (w, pairs) in rest {
            let mut key = ends_pair_key(ea, eb);
            crate::transducer::address::put_bytes(&mut key, w.key().as_bytes());
            let Some(section) = form.face_carrying_with_edge(i, j, w, edge.clone()) else {
                continue;
            };
            form.faces.push(section);
            sections.insert(key, pairs);
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
    let nverts = form.vertices.len();
    trace::trace_carriage("extract/4 rebase", &measured, &form);
    trace::trace_n("extract/6 contact", nverts, budget);
    for vi in 0..nverts {
        // A contact ring not read is a vertex that keeps its carriage,
        // which is what it had; it is not a reason to unsay the form.
        if !budget.spend() {
            return finish(form, &pair_forms, scale, sections, &unreached, &mult_demands);
        }
        let Some(tc) = transport_class(&form.vertices[vi]) else {
            continue;
        };
        let Some(Vertex::Word { contact: None, .. }) = form.vertices.get(vi) else {
            continue;
        };
        // THE PLACEMENT VARIES WHERE ONE CHART CARRIES TWO CARRIAGES, which
        // step 1's walk already read and filed; this reads the index rather
        // than walking every pair form again for a fact already held (rule
        // 1). It is also the corrected reading: comparing the rule vertex's
        // carriage against each pair's FIRST vertex of the class compared two
        // different bodies' placements once a class spanned two charts.
        let place_varies = class_place_varies.contains(&tc.key());
        let mut rings: Option<u8> = None;
        let mut ring_one = true;
        for (_x, y) in family.iter() {
            let fy = frame_of(y);
            let outs: Vec<Cell3> = perceive(y).into_iter().flat_map(|b| b.cells).collect();
            // The frame's own border. A frame with no rows or no columns
            // has no far side, and the near side is the whole of it.
            let (Some(last_row), Some(last_col)) =
                (fy.0.checked_sub(1), fy.1.checked_sub(1))
            else {
                continue;
            };
            let frame_anchor: Vec<(i64, i64)> = (0..fy.1 as i64)
                .map(|c| (0, c))
                .chain((0..fy.1 as i64).map(|c| (last_row as i64, c)))
                .chain((0..fy.0 as i64).map(|r| (r, 0)))
                .chain((0..fy.0 as i64).map(|r| (r, last_col as i64)))
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
        // A PLACEMENT THAT VARIES AND CANNOT BE CONTACTED IS A SECTION.
        //
        // `place_varies && ring_one` puts the vertex on its orbit and drops
        // the carriage, and the court then reads it as placed-by-orbit and
        // passes over it. `place_varies && !ring_one` did NOTHING: the rule
        // kept whichever pair's carriage it was read from and asserted it
        // for the whole bundle, and the court — which checks every word
        // vertex against every pair — refused it on the pairs it was never
        // measured against. Measured at epoch 13: 80 of 87 vertex-body
        // refusals, all on pair 0, one sentence repeating.
        //
        // Two carriages on one class are two sections (18.2), and the face
        // path already says so. The class path now says it too: the vertex
        // states the pairs it was actually measured on, and nothing is
        // claimed about the rest.
        if place_varies && !ring_one {
            if let Some(tc) = transport_class(&form.vertices[vi]) {
                let tk = tc.key();
                if let (Some(pairs), Vertex::Word { acts, .. }) = (class_pairs.get(&tk), &form.vertices[vi]) {
                    sections.insert(tk.clone(), matching_pairs_for(&tk, pairs, acts));
                }
            }
        }
        if place_varies && ring_one {
            if let Some(r) = rings {
                // THE CONTACT REPLACES THE PLACEMENT at every chart this
                // vertex acts on: a stop ring is the vertex's, not one
                // member's, so each member's carriage becomes the identity
                // and the ring carries the placement instead.
                if let Vertex::Word { contact, acts, .. } = &mut form.vertices[vi] {
                    *contact = Some(crate::transducer::alphabet::Contact { anchor: 0, ring: r });
                    let charts: Vec<Vec<u8>> = acts.scoped().map(|(k, _)| k).collect();
                    for k in charts {
                        if let Some(a) = acts.get_mut(&k) {
                            *a.carriage = element::identity_versor();
                        }
                    }
                }
            }
        }
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
