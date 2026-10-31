// 21 — SPEAKING a rule: the form applied to a scene, and what it emits.
//
//! `apply_form` carries the rule's vertices onto the scene's bodies and
//! emits the painted sites they name. A hue no neighbour supplies is
//! refused; a spoken grid that does not grow to the frame is not a speech.

use super::*;
use crate::transducer::sentence::frame_key;

pub(crate) fn grows_to_frame(a: &Grid, b: &Grid) -> bool {
    let (fa, fb) = (frame_of(a), frame_of(b));
    if fa != fb || fa.0 == 0 || fa.1 == 0 {
        return false;
    }
    let ins: Vec<Painted> = crate::arc::painted_of_grid(a);
    let outs: Vec<Painted> = crate::arc::painted_of_grid(b);
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
    // The guard in the frame's own count: a ring can grow at most once per
    // site, so `frame_n` passes bound the fill.
    let mut guard = 0usize;
    while occ.len() < frame_n && guard < frame_n {
        guard += 1;
        let ring = crate::transducer::blade::adjacent_ring(&occ);
        let keyed = occupy_keys(&occ);
        let mut grew = false;
        for (r, c, _) in ring {
            #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: a site one ring out from sites painted_of_grid delivered or this check admitted, so within the sensor's grid plus one, checked non-negative first")]
            if r < 0 || c < 0 || (r as usize) >= fa.0 || (c as usize) >= fa.1 {
                continue;
            }
            if keyed.contains(&site_key(r, c)) {
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
    painted: &mut Vec<Painted>,
    remainder: &mut Vec<Painted>,
) {
    let hue = hue_map_from_pairs(map);
    for l in &body.compartments {
        let h = l.hue;
        let Some((r, c)) = on_grid(&l.site) else { continue };
        // A hue the map does not move keeps its own colour: that is what
        // the map says about it (rule 11).
        let spoken = match hue[h as usize] {
            Some(t) => t,
            None => h,
        };
        let frame = l.site.axis(crate::transducer::base::FRAME_AXIS);
        match element::carry_compartment_site(v, &l.site) {
            // The grid spoken onto is the body's own frame: a compartment
            // carried into another frame is not spoken on it, and is
            // remainder.
            Some(image) if image.iter().all(|s| s.axis(crate::transducer::base::FRAME_AXIS) == frame) => {
                // A carried site can be any i64: inside the frame by
                // `try_from` and a comparison, exact at every width — a cast
                // could wrap a far site into the frame.
                let within = |x: i64, n: usize| usize::try_from(x).is_ok_and(|x| x < n);
                for (i, j) in image.iter().filter_map(on_grid) {
                    if within(i, fy.0) && within(j, fy.1) {
                        painted.push((i, j, spoken));
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
        let Some(&h) = occ.get(&site_key(nr, nc)) else {
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
    speak_on_scene(f, x, &Scene::of(x), placement, crystal, pair, &mut Speaking::free())
}

/// 78.153 — THE SAME SPEECH ON A STROKE: one read per cited form spoken
/// afresh on the scene (a citation read back from the scene's memo costs
/// nothing more). `Spoken::Spent` where the stroke ran out first: nothing
/// was said, and a caller reads that as unread, never as a speech.
/// `bodies` is `x` as the sensor already read it, where the caller holds
/// that perception (the court's replay cursor); `None` perceives `x` here.
pub(crate) fn apply_form_metered(
    f: &Form,
    x: &Grid,
    placement: &crate::transducer::alphabet::Word,
    crystal: &dyn Crystal,
    pair: Option<usize>,
    budget: &mut crate::physics::Budget,
    bodies: Option<Vec<Body>>,
) -> Spoken {
    let scene = match bodies {
        Some(b) => Scene::of_bodies(b),
        None => Scene::of(x),
    };
    let mut ctx = Speaking::metered(budget);
    let said = speak_on_scene(f, x, &scene, placement, crystal, pair, &mut ctx);
    match (said, ctx.spent) {
        (_, true) => Spoken::Spent,
        (Some(s), false) => Spoken::Said(s),
        (None, false) => Spoken::Unfit,
    }
}

/// What a metered speech came to (78.153).
#[expect(clippy::large_enum_variant, reason = "the speech is the common outcome and is moved out at once; the bare refusals carry nothing to box against")]
pub(crate) enum Spoken {
    Said(Speech),
    /// The speech does not fit its own frame (`speak_on_scene`'s `None`).
    Unfit,
    /// The stroke was spent before the speech ended: nothing was said.
    Spent,
}

/// 78.153 — WHAT A SPEECH CARRIES DOWN ITS CITATIONS: the cited forms on
/// the speaking stack, and the stroke it spends where it is metered.
///
/// THE STACK IS THE CYCLE GUARD (defect 4 of the lock audit). Mutual
/// citation is admitted by the nerve (`replay_form_on`'s citation reading:
/// a posetal height forbade every 2-simplex), and speaking it recursed
/// without end: `A` cites `B`, `B` cites `A`, and each speech spoke the
/// other before its own memo entry was written. A citation already on the
/// stack is therefore a fact about the speech, named in `Speech::cycled` by
/// the cited address, and the court refuses it as
/// [`crate::verdict::REFUSE_CITATION_CYCLE`] — never a stack overflow.
pub(crate) struct Speaking<'b> {
    stack: Vec<crate::transducer::address::Address>,
    budget: Option<&'b mut crate::physics::Budget>,
    spent: bool,
}

impl<'b> Speaking<'b> {
    /// Unmetered: the voice's own speech and every reader that is not under
    /// a court's lock.
    pub(crate) fn free() -> Speaking<'static> {
        Speaking { stack: Vec::new(), budget: None, spent: false }
    }

    pub(crate) fn metered(budget: &'b mut crate::physics::Budget) -> Speaking<'b> {
        Speaking { stack: Vec::new(), budget: Some(budget), spent: false }
    }

    /// One read, where the speech is metered; false once the stroke is
    /// spent, and the speech then says nothing.
    fn spend(&mut self) -> bool {
        let Some(b) = self.budget.as_deref_mut() else {
            return true;
        };
        if b.spend() {
            return true;
        }
        self.spent = true;
        false
    }
}

/// THE SCENE A FORM IS SPOKEN ONTO, PERCEIVED ONCE (law 12): the grid's
/// bodies and the atlas that files them by chart. A cited form spoken
/// inside this speech speaks onto the same scene and perceives nothing
/// itself: the bodies are one fact of the grid, not of the vertex citing
/// them.
pub(crate) struct Scene {
    bodies: Vec<Body>,
    atlas: crate::transducer::stalk::Stalks,
    /// WHAT A CITED FORM SPEAKS ON THIS GRID, by the citation's own
    /// coordinate (Z2 item 3): the cited form's address and the COMPOSED
    /// placement, written out (rule 6). What a citation emits is a function
    /// of the cited form, the placement it is spoken at, and the grid — and
    /// of nothing else, so a rule citing one form 578 times at one
    /// placement speaks it once and reads the painted set back 577 times.
    ///
    /// It is on the SCENE because the scene is the grid: a memo held
    /// anywhere wider would answer for a grid it was not measured on, and
    /// one held narrower could not outlive the citation that filled it.
    /// `RefCell` because `speak_on_scene` recurses through `&Scene` and the
    /// memo is written by the recursion, never read by the engine as a
    /// decision.
    spoken: std::cell::RefCell<crate::network::nerve::Keyed<Cited>>,
}

/// What one citation spoke on this scene: its painted sites, its remainder,
/// the citations inside it this crystal could not resolve, and those it
/// reached back onto its own speaking stack (78.153).
type Cited = (
    Vec<Painted>,
    Vec<Painted>,
    Vec<crate::transducer::address::Address>,
    Vec<crate::transducer::address::Address>,
    Vec<usize>,
    [u64; 6],
);

impl Scene {
    pub(crate) fn of(x: &Grid) -> Scene {
        Scene::of_bodies(perceive(x))
    }

    /// The scene over bodies already perceived (78.153): the same scene
    /// `of` builds, without the sensor reading the grid again.
    pub(crate) fn of_bodies(bodies: Vec<Body>) -> Scene {
        let mut atlas = crate::transducer::stalk::Stalks::default();
        for (i, body) in bodies.iter().enumerate() {
            atlas.admit(&body.moduli, (i as u64).to_le_bytes());
        }
        Scene { bodies, atlas, spoken: std::cell::RefCell::new(crate::network::nerve::Keyed::new()) }
    }

    /// The citation's coordinate: the cited address and the composed
    /// placement's canonical bytes, length-delimited (A21).
    fn citation_key(address: &[u8], composed: &crate::transducer::alphabet::Word) -> Vec<u8> {
        let mut k = Vec::new();
        crate::transducer::address::put_key_bytes(&mut k, address);
        let mut w = Vec::new();
        composed.put_canonical(&mut w);
        crate::transducer::address::put_key_bytes(&mut k, &w);
        k
    }
}

/// The speech itself, on a scene already perceived: each vertex kind
/// emits onto the scene's bodies, read by chart from its atlas.
fn speak_on_scene(
    f: &Form,
    x: &Grid,
    scene: &Scene,
    placement: &crate::transducer::alphabet::Word,
    crystal: &dyn Crystal,
    pair: Option<usize>,
    ctx: &mut Speaking<'_>,
) -> Option<Speech> {
    let fx = frame_of(x);
    let mut fy = output_frame(fx, f.ratio);
    let Scene { bodies, atlas, .. } = scene;
    let mut painted: Vec<Painted> = Vec::new();
    let mut remainder: Vec<Painted> = Vec::new();
    let mut unspoken: Vec<crate::network::nerve::coord::VertexAt> = Vec::new();
    let mut unresolved: Vec<crate::transducer::address::Address> = Vec::new();
    let mut cycled: Vec<crate::transducer::address::Address> = Vec::new();
    let mut unfolded = false;
    // 78.167 A2 — WHAT THE SPEECH REACHED, so what it did not is named: each
    // scene body a vertex (or a cited form) spoke, and the demand vertices it
    // could speak nothing for, by kind.
    let mut reached = vec![false; bodies.len()];
    let mut demanded = [0u64; 6];
    for (vi, v) in f.vertices.entries() {
        match v {
            Vertex::Frame => {}
            // ONE VERTEX SPEAKS EVERY BODY IT ACTS ON. 21 step 1: two bodies
            // of different shapes carried by one transport correspond, and
            // each stands on its own chart. A vertex acting on two charts
            // that spoke ONE of them would leave the other body unreached
            // and unmentioned where the pair needs both bodies carried.
            //
            // THE MEMBER IS THE UNIT, so the carriage and the gauge are read
            // at the chart whose bodies they are being applied to. Reading
            // one member's carriage and applying it to another chart's bodies
            // would carry a body by a versor measured on a different one.
            Vertex::Word { derivation, acts, .. } => {
                // THE CLASS'S SECTION IS ITS MEMBERS' FRAMES (AT1): a class
                // none of whose members holds a frame is claimed whole, and
                // one whose members hold frames speaks only on those.
                let pi = pair.and_then(|pi| u64::try_from(pi).ok());
                if let Some(k) = pi {
                    let held = acts.scoped().any(|(_, a)| !a.frames.is_empty());
                    let here = acts.scoped().any(|(_, a)| a.frames.contains(&frame_key(k)));
                    if held && !here {
                        unspoken.push(vi);
                        continue;
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
                    // A member holding frames speaks on them alone.
                    if let Some(k) = pi {
                        if !member.frames.is_empty() && !member.frames.contains(&frame_key(k)) {
                            continue;
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
                        let Ok(i) = usize::try_from(u64::from_le_bytes(raw)) else { continue };
                        if let Some(body) = bodies.get(i) {
                            emit_carried(body, &versor, &member.gauge, fy, &mut painted, &mut remainder);
                            reached[i] = true;
                        }
                    }
                }
                if !spoke {
                    unspoken.push(vi);
                }
            }
            // A CITED FORM SPEAKS ONTO THE SCENE THIS SPEECH ALREADY
            // PERCEIVED, at the composed placement: the crystal resolves the
            // citation by address, and the cited form's bodies are this
            // grid's bodies (law 12).
            v @ Vertex::Glued { placement: q, .. } => {
                let composed = placement.compose(q);
                // 54.7 B5a — EVERY FORM THIS BOUNDARY MATES WITH, off the
                // cover. It read the target off the vertex, and a vertex no
                // longer names the form it cites: the boundary is the
                // chart's fact, the incidence is `N(𝒰)`'s. One site may
                // carry several edges (gauge universality), and each of
                // them speaks.
                //
                // A DECODED FORM CARRIES ITS COVER OR SPEAKS NOTHING HERE,
                // and that is why `from_element_form` fills it from the
                // word's envelope: the court reaches this through
                // `rule_matches_tests`, and a rule whose cover it did not
                // receive would paint nothing while the client painted it,
                // failing every citing solve on the verification.
                let site = crate::transducer::sentence::vertex_bytes(v);
                let mut spoke_any = false;
                for edge in f.edges_at(&site) {
                    spoke_any = true;
                    let key = Scene::citation_key(edge.target.as_bytes(), &composed);
                    // READ THE CITATION AT ITS OWN COORDINATE. This rule
                    // cites one form 578 times at one placement and spoke it
                    // 578 times onto the same grid; what comes back is the
                    // same painted set every time, because nothing in the
                    // citation varies.
                    if let Some((p, r, u, c, left, d)) = scene.spoken.borrow().get(&key) {
                        painted.extend_from_slice(p);
                        remainder.extend_from_slice(r);
                        unresolved.extend_from_slice(u);
                        cycled.extend_from_slice(c);
                        // A body the cited form reached is reached by this one.
                        for (i, hit) in reached.iter_mut().enumerate() {
                            if !left.contains(&i) {
                                *hit = true;
                            }
                        }
                        for (n, k) in demanded.iter_mut().zip(d) {
                            *n += k;
                        }
                        continue;
                    }
                    // 78.153 — A CITATION ALREADY BEING SPOKEN is a cycle of
                    // the cover, named by the cited address: speaking it again
                    // would never end (`Speaking`).
                    if ctx.stack.contains(&edge.target) {
                        cycled.push(edge.target.clone());
                        continue;
                    }
                    // A CITATION THIS CRYSTAL DOES NOT HOLD is a fact about the
                    // crystal, named by the cited address (78.22: a coordinate
                    // locates, it never reconstructs), and the rest of the
                    // form still speaks. It emptied the whole speech, and
                    // every site of the family then read as unspoken.
                    let Some((_e, s)) = crystal.by_address(&edge.target) else {
                        unresolved.push(edge.target.clone());
                        continue;
                    };
                    // 78.153 — ONE READ PER CITED FORM SPOKEN AFRESH, where
                    // the speech is metered; a spent stroke says nothing.
                    if !ctx.spend() {
                        return None;
                    }
                    ctx.stack.push(edge.target.clone());
                    let sp = speak_on_scene(&s, x, scene, &composed, crystal, None, ctx);
                    ctx.stack.pop();
                    let sp = sp?;
                    scene.spoken.borrow_mut().insert(
                        &key,
                        (
                            sp.painted.clone(),
                            sp.remainder.clone(),
                            sp.unresolved.clone(),
                            sp.cycled.clone(),
                            sp.unreached.clone(),
                            sp.demanded,
                        ),
                    );
                    for (i, hit) in reached.iter_mut().enumerate() {
                        if !sp.unreached.contains(&i) {
                            *hit = true;
                        }
                    }
                    for (n, k) in demanded.iter_mut().zip(sp.demanded) {
                        *n += k;
                    }
                    painted.extend(sp.painted);
                    remainder.extend(sp.remainder);
                    unresolved.extend(sp.unresolved);
                    cycled.extend(sp.cycled);
                }
                // A BOUNDARY WITH NO INCIDENCE IS UNSPOKEN, NOT SILENT
                // (rule 11). The form states a citation and nothing told it
                // which form; that is a gap in what reached this reader and
                // it is reported, never painted over as an empty citation.
                if !spoke_any {
                    unspoken.push(vi);
                }
            }
            // A REBASED VERTEX SPEAKS EVERY CHART `of` ACTED ON, as a Word
            // vertex does: `by`'s word, the relative at that chart composed with
            // the base's placement, and that chart's own bodies and hue map.
            Vertex::Rebased { by, acts, .. } => {
                let Some(w @ Vertex::Word { derivation, .. }) = f.vertices.at(by)
                else {
                    unspoken.push(vi);
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
                    unspoken.push(vi);
                    continue;
                };
                let Some(base) = w.acting_on(base_chart) else {
                    unspoken.push(vi);
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
                        let Ok(i) = usize::try_from(u64::from_le_bytes(raw)) else { continue };
                        if let Some(body) = bodies.get(i) {
                            emit_carried(body, &versor, &member.gauge, fy, &mut painted, &mut remainder);
                            reached[i] = true;
                        }
                    }
                }
                if !spoke {
                    unspoken.push(vi);
                }
            }
            Vertex::Unfolded {
                measure: crate::transducer::sentence::Measure::Occupied,
            } => {
                unfolded = true;
            }
            Vertex::Unfolded { .. } => {}
            // 50.5: a demand speaks nothing — 78.167 A2: and the speech says
            // which, by kind, so a wrong grid is read for what it could not
            // say.
            Vertex::Demand { kind, .. } => {
                if let Some(n) = demanded.get_mut(usize::from(kind.kind_byte())) {
                    *n += 1;
                }
            }
        }
    }
    if unfolded {
        if painted.is_empty() {
            painted = bodies
                .iter()
                .flat_map(|b| b.compartments.iter().filter_map(|l| on_grid(&l.site).map(|(r, c)| (r, c, l.hue))))
                .collect();
        }
        // A frame whose site count does not fit the carrier is wider than
        // this speech fills; nothing is unfolded into it.
        let need = fy.0.checked_mul(fy.1)?;
        let mut guard = 0usize;
        while painted.len() < need && guard < need {
            guard += 1;
            let ring = crate::transducer::blade::adjacent_ring(&painted);
            let mut hues: crate::network::nerve::Keyed<u8> = crate::network::nerve::Keyed::new();
            for &(r, c, h) in &painted {
                hues.insert(site_key(r, c), h);
            }
            let mut grew = false;
            for (r, c, _) in ring {
                #[expect(clippy::cast_possible_truncation, reason = "STD1 C1 — the sensor's: a site one ring out from painted sites already inside the frame fy or on the sensor's grid, checked non-negative first")]
                if r < 0 || c < 0 || (r as usize) >= fy.0 || (c as usize) >= fy.1 {
                    continue;
                }
                if hues.contains(&site_key(r, c)) {
                    continue;
                }
                match hue_from_neighbors(r, c, &hues) {
                    Some(h) => {
                        painted.push((r, c, h));
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
    // 78.143 — A FORM THAT STATES CLAUSES SPEAKS THEM: each a selection over
    // this scene's bodies and what is done to it, and the output frame is the
    // one they speak (a crop's, or the input's). A clause unstated here — a
    // tie the rule's order does not name, a coordinate that is not exact —
    // lays down no speech; the court refuses it by name before it reads one.
    if !f.elements.is_empty() {
        let spoken = super::statement::speak_elements(&f.elements, x, bodies).ok()?;
        fy = spoken.frame;
        painted = spoken.painted;
        // 78.167 C — the clauses are the whole speech: what the vertices left
        // as remainder is not what was said, and a demand is stated by the
        // clause that reads its act.
        remainder.clear();
        demanded = [0; 6];
    }
    painted.sort_unstable();
    painted.dedup();
    unresolved.sort_unstable();
    unresolved.dedup();
    cycled.sort_unstable();
    cycled.dedup();
    // A body the speech did not reach is named, never silent (rule 11). An
    // unfolded form fills the frame, and clauses paint a canvas that opens as
    // the input, so under either every body is spoken for.
    let unreached: Vec<usize> = match unfolded || !f.elements.is_empty() {
        true => Vec::new(),
        false => reached.iter().enumerate().filter(|(_, hit)| !**hit).map(|(i, _)| i).collect(),
    };
    Some(Speech {
        frame: fy,
        painted,
        remainder,
        unspoken,
        unresolved,
        cycled,
        unreached,
        demanded,
    })
}

/// What a form says about a new input: the spoken painted sites on the
/// output frame, and the remainder — input sites the form does not speak.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speech {
    pub frame: (usize, usize),
    pub painted: Vec<Painted>,
    pub remainder: Vec<Painted>,
    /// The vertices, by coordinates, whose class fiber was empty on
    /// this grid.
    pub unspoken: Vec<crate::network::nerve::coord::VertexAt>,
    /// The cited forms, by address, the speaking crystal does not hold: the
    /// sites they would have spoken are not the form's silence.
    pub unresolved: Vec<crate::transducer::address::Address>,
    /// 78.153 — the cited forms this speech reached while they were still
    /// being spoken: a cycle of citations, which no speech completes.
    pub cycled: Vec<crate::transducer::address::Address>,
    /// 78.167 A2 — the scene's bodies, by index, no vertex or cited form
    /// reached: absent from the speech, and named so (rule 11).
    pub unreached: Vec<usize>,
    /// 78.167 A2 — the demand vertices the form carries that spoke nothing
    /// on this scene, by `Demand::kind_byte`.
    pub demanded: [u64; 6],
}

impl Speech {
    /// The wire form: the spoken set laid on the frame's rectangle. Only
    /// here does a rectangle exist; an unspoken site reads as the ground.
    ///
    /// `None` WHERE A SITE DOES NOT FIT THE FRAME. A transported site
    /// outside the output rectangle is exactly the condition the court
    /// names `FAIL_PAST_FRAME` — a site landing past the image frame's
    /// extent — and a speech that cannot be laid on its own frame is not a
    /// speech, so it is refused here and every caller decides (rule 11).
    /// Dropping the site instead would lay down a grid that silently is
    /// not what the form said, and indexing on it would kill the process
    /// where the court refuses.
    pub fn grid(&self) -> Option<Grid> {
        // A speech missing a citation is not the form's whole speech, and no
        // grid stands for it; the measure reads the citation by name.
        if !self.unresolved.is_empty() || !self.cycled.is_empty() {
            return None;
        }
        let mut out: Grid = vec![vec![crate::arc::ground(); self.frame.1]; self.frame.0];
        for &(r, c, h) in &self.painted {
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
/// the same output sites.
pub fn speak_rule(rule: &crate::transducer::sentence::SentenceRule, x: &Grid, crystal: &dyn Crystal) -> Option<Speech> {
    apply_form(&rule.form, x, &crate::transducer::alphabet::Word::identity(), crystal)
}

/// The residual between what was spoken and what was observed, as a chain
/// (row, col, spoken hue, observed hue) over every site that differs.
pub(crate) fn residual_of(spoken: Option<&Grid>, observed: &Grid) -> Vec<(i64, i64, u8, u8)> {
    let mut out = Vec::new();
    for (r, row) in observed.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            // A site the speech does not reach is unoccupied there, and
            // the scene's ground IS absence in this encoding.
            let s = match spoken.and_then(|g| g.get(r)).and_then(|row| row.get(c)) {
                Some(h) => *h,
                None => crate::arc::ground(),
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
                let void = crate::arc::ground();
                if !inside && s != void {
                    out.push((r as i64, c as i64, s, void));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}
