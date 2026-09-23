// 52.5 items 1–4 — THE PERCEPT of a family: every frame the stroke reaches
// is perceived once, every chart's fiber is read frame by frame as a
// quotient, and what no quotient reaches is a cover or a demand.

use super::chord::dispiration;
use super::cover::cover_deformation;
use super::fiber::{body_key, fiber_frame_of_key, fiber_key, fibers, laminae_of, unreached_demands, vanished};
use super::pair::{delta_links, Covered, Scenery};
use super::perceive::{
    at_frame_zero, demand_vertex, differing_axes, frame_of_body, perceive_at, support_at,
    word_or_demand_on_chart, Demanded, EdgeTally,
};
use super::witness::{cover_nerves, phase_spectrum};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::{section_of_frames, vertex_ix_key, Acting, Demand, FormFace, Transport};

pub(super) type Ratio = ((usize, usize), (usize, usize));

/// THE PERCEPT OF A FAMILY (52.5): the peeled form, the sections along
/// `e₃`, what move 6 reads, and the frames the budget did not reach. The
/// derivation against a mind's presentation is not in it (50.2 step 2).
#[derive(Clone)]
pub(crate) struct Percept {
    pub(crate) form: Form,
    pub(crate) sections: Keyed<Keyed<()>, coord::SectionAt>,
    pub(crate) varying: contact::Varying,
    pub(crate) unreached: Vec<usize>,
}

/// WHAT ONE FRAME'S QUOTIENT OF ONE CHART MEASURED: the frame, the versor
/// with its hue map, the two union bodies in the frame's own coordinates,
/// and the same two bodies at their own `(r, c, k)` — the laminae the
/// fiber's union shape is built from (52.5).
struct Quotient {
    frame: u16,
    car: Carriage,
    a0: Body,
    b0: Body,
    a_k: Body,
    b_k: Body,
}

/// ONE MEMBER OF A CLASS'S VERTEX (D7): the chart, what it acts by there,
/// the frames its versor holds on, the bodies at each of them, the output
/// support over those frames, the union shape its fiber occupies along
/// `e₃` with the versor carrying it, and the words the other frames of the
/// chart spoke as sections (21 step 2).
struct Member {
    chart: Invariant,
    acting: Acting,
    frames: Vec<u16>,
    at: Vec<(u16, Body, Body, Versor)>,
    support: Vec<(i64, i64, i64)>,
    union: Option<(Body, Body, Versor)>,
    section_words: Vec<(crate::transducer::alphabet::Word, Vec<u16>)>,
}


/// A class's vertex before it is placed: the least address among its
/// members' readings, that member's derivation, and every member.
struct ClassRead {
    address: crate::transducer::address::Address,
    derivation: crate::transducer::alphabet::Derivation,
    members: Vec<Member>,
}

/// THE PERCEPT (52.5 items 1 to 4), under the stroke (rule 10): every
/// frame the budget reaches is perceived once; every chart's fiber is read
/// frame by frame; a budget that ends leaves the frames it did not measure
/// as the demands of `finish` and says so.
pub(super) fn family_percept(family: &[(Grid, Grid)], scale: Ratio, budget: &mut crate::physics::Budget) -> Percept {
    use crate::transducer::sentence::face_pair_key;
    trace::trace("extract/0 frames", trace::Wrote::Nothing, Some(budget));
    let n = family.len();
    let mut unreached: Vec<usize> = Vec::new();
    let mut ins: Vec<Body> = Vec::new();
    let mut outs: Vec<Body> = Vec::new();
    for (k, (a, b)) in family.iter().enumerate() {
        if !budget.spend() {
            unreached.push(k);
            continue;
        }
        ins.extend(perceive_at(a, k as i64));
        outs.extend(perceive_at(b, k as i64));
    }
    let n_reached = n - unreached.len();
    let fib_in = fibers(&ins);
    let fib_out = fibers(&outs);
    let mut charts: Keyed<Invariant> = Keyed::new();
    for (k, (c, _)) in fib_in.scoped() {
        charts.insert(&k, c.clone());
    }
    for (k, (c, _)) in fib_out.scoped() {
        charts.insert(&k, c.clone());
    }
    // WHICH OUTPUT BODY STANDS AT EACH SITE, read once (rule 1): the
    // reading an input body with no image is typed by.
    let mut out_bodies: Keyed<Body> = Keyed::new();
    let mut out_at_site: Keyed<Vec<u8>> = Keyed::new();
    for b in &outs {
        let bk = body_key(b);
        for &(r, c, f, _) in &b.laminae {
            out_at_site.insert(base_key(r, c, f), bk.clone());
        }
        out_bodies.insert(bk, b.clone());
    }

    trace::trace("extract/1 fibers", trace::Wrote::Nothing, Some(budget));
    let mut reads: Keyed<ClassRead, coord::Transport> = Keyed::new();
    let mut demanded: Vec<Demanded> = Vec::new();
    let mut covers: Vec<Covered> = Vec::new();
    let mut hue_maps: Keyed<Vec<(u8, u8)>> = Keyed::new();
    let mut varying = contact::Varying::new();
    let mut attempted: Keyed<()> = Keyed::new();
    // The charts with a quotient at each frame, and the output bodies a
    // quotient or a cover reached: the selection reading (Y4) is made of
    // these.
    let mut quotient_charts: Vec<Keyed<()>> = vec![Keyed::new(); n];
    let mut out_reached: Keyed<()> = Keyed::new();
    let mut cut = false;
    // Z1b — THE MOVES OF THE PERCEPT ARE LINES OF THEIR OWN. `extract/1
    // fibers` held one span over four moves and a reader of the timeline
    // could not tell which of them cost the seconds; a line that names four
    // moves at once is a reading of the instrument, not of the engine.
    //
    // EVERY LINE IS AT ITS MOVE'S ENTRY, because the timeline's `held` is
    // the gap to the NEXT line: a line at a move's exit reports the move
    // after it under the name of the move before it. The regions BETWEEN
    // the four are named too — otherwise their cost lands on whichever of
    // the four happens to precede them.
    trace::trace("percept/1 covers", trace::Wrote::Nothing, Some(budget));
    'charts: for (ck, c) in charts.scoped() {
        let in_frames = fib_in.get(&ck).map(|(_, f)| f);
        let out_frames = fib_out.get(&ck).map(|(_, f)| f);
        let mut frames: Keyed<()> = Keyed::new();
        if let Some(f) = in_frames {
            for (fk, _) in f.scoped() {
                frames.insert(&fk, ());
            }
        }
        if let Some(f) = out_frames {
            for (fk, _) in f.scoped() {
                frames.insert(&fk, ());
            }
        }
        let mut quotients: Vec<Quotient> = Vec::new();
        for (fk, ()) in frames.scoped() {
            let Some(k) = fiber_frame_of_key(&fk) else { continue };
            let (Some(ai), Some(bo)) = (
                in_frames.and_then(|f| f.get(&fk)),
                out_frames.and_then(|f| f.get(&fk)),
            ) else {
                continue;
            };
            // ONE READ PER QUOTIENT ATTEMPT (rule 10).
            if !budget.spend() {
                cut = true;
                break 'charts;
            }
            let mut ak = ck.clone();
            ak.extend_from_slice(&fiber_key(k));
            attempted.insert(&ak, ());
            let a_k = body_of(laminae_of(ai));
            let b_k = body_of(laminae_of(bo));
            let a0 = body_of(at_frame_zero(&laminae_of(ai)));
            let b0 = body_of(at_frame_zero(&laminae_of(bo)));
            match carriage(&a0, &b0) {
                Some(car) => {
                    if let Some(qc) = quotient_charts.get_mut(k as usize) {
                        qc.insert(&ck, ());
                    }
                    for b in bo {
                        out_reached.insert(body_key(b), ());
                    }
                    quotients.push(Quotient { frame: k as u16, car, a0, b0, a_k, b_k });
                }
                // 52.5 item 3, 50.8: no quotient at this frame. The cover
                // reads what the pieces carry, and what it leaves is the
                // demand; a cover the stroke cut short leaves the demand as
                // typed (rule 11).
                None => match cover_deformation(&a0, &b0, Some(scale), budget) {
                    Ok(cover) => {
                        for b in bo {
                            if b.laminae.iter().all(|&(r, cc, _, _)| cover.reached.contains(&base_key(r, cc, 0))) {
                                out_reached.insert(body_key(b), ());
                            }
                        }
                        for (_, d) in cover.refused.scoped() {
                            demanded.push(d.clone());
                        }
                        let mut demands = unreached_demands(ai, &a0, &b0, &cover.reached, c, ai.len() as u32);
                        let first = demands.split_off(1);
                        demanded.extend(first);
                        let [demand] = demands.as_slice() else { continue };
                        covers.push(Covered { demand: demand.clone(), cover, frame: k });
                    }
                    Err(_) => {
                        demanded.extend(unreached_demands(ai, &a0, &b0, &Keyed::new(), c, ai.len() as u32));
                    }
                },
            }
        }
        if quotients.is_empty() {
            continue;
        }
        // THE CLASSES THE CHART'S FRAMES READ (21 step 1): a rotor class per
        // versor, the frames grouped by the versor they read, in frame order.
        let mut by_class: Keyed<Vec<Quotient>> = Keyed::new();
        for q in quotients {
            let Some(class) = element::base_rotor_of(&q.car.v).and_then(|r| r.class_key()) else {
                continue;
            };
            match by_class.get_mut(class.as_bytes()) {
                Some(v) => v.push(q),
                None => {
                    by_class.insert(class.as_bytes(), vec![q]);
                }
            }
        }
        for (_, qs) in by_class.scoped() {
            let mut by_versor: Keyed<Vec<&Quotient>> = Keyed::new();
            let mut rep_key: Option<Vec<u8>> = None;
            for q in qs {
                let Some(vk) = q.car.v.key() else { continue };
                if rep_key.is_none() {
                    rep_key = Some(vk.as_bytes().to_vec());
                }
                match by_versor.get_mut(vk.as_bytes()) {
                    Some(v) => v.push(q),
                    None => {
                        by_versor.insert(vk.as_bytes(), vec![q]);
                    }
                }
            }
            let Some(rep_key) = rep_key else { continue };
            let Some(rep) = by_versor.get(&rep_key) else { continue };
            let [first, ..] = rep.as_slice() else { continue };
            // THE VERTEX IS THE LEAST FRAME'S READING (51.1: the frame is the
            // corpus's coordinate); every other frame reading the same versor
            // is the frames it holds on.
            let vertex = match word_or_demand_on_chart(&first.a0, &first.b0, c) {
                Some(Ok(v)) => v,
                Some(Err(d)) => {
                    // A reader that returns a typed demand has answered
                    // (rule 17), and the timeline says so rather than
                    // showing a gap where the vertex would have been.
                    trace::trace("extract/1 chart", trace::Wrote::Demand, Some(budget));
                    demanded.push(d);
                    continue;
                }
                None => continue,
            };
            let Vertex::Word { address, derivation, acts, .. } = &vertex else { continue };
            let Some(tc) = transport_class(&vertex) else { continue };
            // A FIBRE READ AT THE CHART'S STRATUM (54.7 R1). `Acting::key`
            // is `chart ‖ carriage`, so an exact lookup on the chart key
            // misses every section — and this `continue` then skipped every
            // Word vertex, leaving a form in which nothing acts.
            let prefix = crate::transducer::sentence::Acting::chart_prefix(c);
            let Some(acting) = acts
                .scoped()
                .find(|(k, _)| k.starts_with(&prefix))
                .map(|(_, a)| a.clone())
            else {
                continue;
            };
            let rotor = derivation.rotor();
            // THE HUE MAP IS A SECTION COORDINATE (21 step 1): every frame's
            // map is a hue section on the form, the vertex carries the least
            // frame's, and a frame whose map differs is a frame the member
            // is not claimed on — its placement varies (move 6).
            let mut frames: Vec<u16> = Vec::new();
            let mut at: Vec<(u16, Body, Body, Versor)> = Vec::new();
            let mut support: Vec<(i64, i64, i64)> = Vec::new();
            // THE LAMINAE OF THE MEMBER'S UNION SHAPE (52.5), gathered at
            // their own frames as the frames are claimed: the union is the
            // fiber the member actually stands on, so a frame the gauge
            // refuses is not in it.
            let mut union_in: Vec<Lamina> = Vec::new();
            let mut union_out: Vec<Lamina> = Vec::new();
            for q in rep {
                if let Some(hk) = crate::transducer::sentence::hue_section_key_of(&rotor, &q.car.sigma) {
                    hue_maps.insert(hk, q.car.sigma.clone());
                }
                if !crate::transducer::sentence::gauge_agrees(&q.car.sigma, &acting.gauge) {
                    varying.place_varies.put(&tc.key(), ());
                    continue;
                }
                frames.push(q.frame);
                at.push((q.frame, q.a0.clone(), q.b0.clone(), q.car.v.clone()));
                support.extend(support_at(&q.b0.laminae, i64::from(q.frame)));
                union_in.extend_from_slice(&q.a_k.laminae);
                union_out.extend_from_slice(&q.b_k.laminae);
            }
            support.sort_unstable();
            support.dedup();
            let union = union_quotient(union_in, union_out, &first.car.v);
            // THE OTHER VERSORS THE CHART READ ON OTHER FRAMES: the placement
            // varies (move 6's reading), and a word that differs is a face the
            // family spoke between the Frame and this vertex on those frames
            // (21 step 2, S12.2).
            let mut section_words = Vec::new();
            for (vk, group) in by_versor.scoped() {
                if vk == rep_key {
                    continue;
                }
                varying.place_varies.put(&tc.key(), ());
                let [g, ..] = group.as_slice() else { continue };
                let mut g_frames: Vec<u16> = group.iter().map(|q| q.frame).collect();
                // A coordinate set, read by `claims_frame` at the frame.
                g_frames.sort_unstable();
                g_frames.dedup();
                if let Some(Ok(w)) = peel_versor(&g.car.v) {
                    if !w.rotor().projective_eq(&rotor) {
                        section_words.push((w, g_frames));
                    }
                }
            }
            let member =
                Member { chart: c.clone(), acting, frames, at, support, union, section_words };
            match reads.at_mut(&tc.key()) {
                Some(read) => {
                    // ONE VERTEX PER TRANSPORT CLASS, its address the least of
                    // its members' (content order, law 2), its derivation
                    // that member's.
                    if address < &read.address {
                        read.address = address.clone();
                        read.derivation = derivation.clone();
                    }
                    read.members.push(member);
                }
                None => {
                    reads.put(&tc.key(), ClassRead { address: address.clone(), derivation: derivation.clone(), members: vec![member] });
                }
            }
        }
    }

    // THE FRAMES THE BUDGET DID NOT MEASURE are unreached (rule 11): every
    // frame with a chart on both sides whose quotient was not attempted.
    if cut {
        for k in 0..n {
            if unreached.contains(&k) {
                continue;
            }
            let fk = fiber_key(k as i64);
            let mut short = false;
            for (ck, _) in charts.scoped() {
                let both = fib_in.get(&ck).is_some_and(|(_, f)| f.contains(&fk))
                    && fib_out.get(&ck).is_some_and(|(_, f)| f.contains(&fk));
                let mut ak = ck.clone();
                ak.extend_from_slice(&fk);
                if both && !attempted.contains(&ak) {
                    short = true;
                }
            }
            if short {
                unreached.push(k);
            }
        }
        unreached.sort_unstable();
        unreached.dedup();
    }

    // THE INPUT BODIES WITH NO CHART ON THE OUTPUT AT THEIR FRAME, and the
    // output bodies with none on the input (50.5, 50.8). An input body
    // standing under an output body of another chart is that body's
    // deformation and is covered; one standing under nothing vanished; an
    // output body no reading imaged is a creation. On a selection frame —
    // one chart holding one body, that chart's quotient the only one, and
    // every output body reached (Y4) — the bodies the selection dropped are
    // what the rule says, not a demand.
    let mut imaged: Keyed<()> = Keyed::new();
    if !cut {
        for k in 0..n {
            if unreached.contains(&k) {
                continue;
            }
            let fk = fiber_key(k as i64);
            let ins_k: Vec<Body> = fib_in
                .scoped()
                .filter_map(|(_, (_, f))| f.get(&fk).cloned())
                .flatten()
                .collect();
            let outs_k: Vec<&Body> = outs.iter().filter(|b| frame_of_body(b) == Some(k as i64)).collect();
            let selected = chart_holding_one(&ins_k);
            let selection = selected.as_ref().is_some_and(|u| {
                let Some(qc) = quotient_charts.get(k) else {
                    return false;
                };
                let only_u = qc.scoped().all(|(ck, _)| ck == u.key().as_bytes()) && qc.contains(u.key().as_bytes());
                only_u && outs_k.iter().all(|b| out_reached.contains(&body_key(b)))
            });
            for body in &ins_k {
                let ck = body.moduli.key();
                if fib_out.get(&ck).is_some_and(|(_, f)| f.contains(&fk)) {
                    continue;
                }
                if selection {
                    continue;
                }
                // The outputs this body stands under, of other charts.
                let mut meets: Keyed<()> = Keyed::new();
                for &(r, c, f, _) in &body.laminae {
                    if let Some(bk) = out_at_site.get(&base_key(r, c, f)) {
                        meets.insert(bk, ());
                    }
                }
                let mut any = false;
                for (bk, ()) in meets.scoped() {
                    let Some(image) = out_bodies.get(&bk) else { continue };
                    any = true;
                    imaged.insert(&bk, ());
                    let a0 = body_of(at_frame_zero(&body.laminae));
                    let b0 = body_of(at_frame_zero(&image.laminae));
                    let axes = differing_axes(&body.moduli, &image.moduli);
                    let demand = (
                        Demand::TopologicalDeformation { image: image.moduli.clone(), axes },
                        body.moduli.clone(),
                        delta_links(&a0, &b0),
                    );
                    match cover_deformation(&a0, &b0, Some(scale), budget) {
                        Ok(cover) => {
                            for (_, d) in cover.refused.scoped() {
                                demanded.push(d.clone());
                            }
                            covers.push(Covered { demand, cover, frame: k as i64 });
                        }
                        Err(_) => demanded.push(demand),
                    }
                }
                if !any {
                    demanded.push(vanished(body));
                }
            }
            for body in outs_k {
                let bk = body_key(body);
                if imaged.contains(&bk) || out_reached.contains(&bk) {
                    continue;
                }
                let ck = body.moduli.key();
                if fib_in.get(&ck).is_some_and(|(_, f)| f.contains(&fk)) {
                    continue;
                }
                let links: Vec<Link> = body.laminae.iter().map(|&(r, c, _, h)| (r, c, 0u8, h)).collect();
                demanded.push((Demand::Creation, body.moduli.clone(), links));
            }
        }
    }

    trace::trace("percept/2 vertices", trace::Wrote::Nothing, Some(budget));
    // THE VERTICES: a class's quotient with a member per chart (D7), and a
    // cover's pieces, each a vertex of the nerve it is a piece of (50.8),
    // claimed on the frame its cover was read on. The sections along e₃
    // are filed by member and by class (21 step 1) once every vertex
    // stands.
    let mut scenery = Scenery::new();
    let frame_ix = scenery.place(frame_vertex(), Vec::new());
    let mut sections: Keyed<Keyed<()>, coord::SectionAt> = Keyed::new();
    let mut bodies_at: Keyed<(Body, Body, Versor)> = Keyed::new();
    // THE UNION SHAPE OF EACH MEMBER'S FIBER, keyed as its support is —
    // `member_key(vertex, chart)` — and holding no frame, because the union
    // is what the member occupies ACROSS its frames and not a reading at any
    // one of them. `bodies_at` keys the same prefix with the frame after it,
    // so a union filed there would answer `occupied_frames` with a frame
    // nothing stands on.
    let mut unions: Keyed<(Body, Body, Versor)> = Keyed::new();
    let mut section_faces: Vec<(u16, crate::transducer::alphabet::Word, Vec<u16>, Option<Transport>)> =
        Vec::new();
    // Member section key → the frames that member is claimed on, and the
    // member the key names. Two members whose every coordinate agrees —
    // one action placed twice — are one key and claim the union.
    let mut member_frames: Keyed<Vec<u16>, coord::MemberSection> = Keyed::new();
    let mut member_of: Keyed<(u16, Invariant), coord::MemberSection> = Keyed::new();
    let claim_member = |scenery: &Scenery,
                        member_frames: &mut Keyed<Vec<u16>, coord::MemberSection>,
                        member_of: &mut Keyed<(u16, Invariant), coord::MemberSection>,
                        ix: u16,
                        chart: &Invariant,
                        frames: &[u16]| {
        let Some(k) = scenery.vertices.get(ix as usize).and_then(|v| member_section_key(v, chart)) else {
            return;
        };
        match member_frames.at_mut(&k) {
            Some(v) => v.extend_from_slice(frames),
            None => {
                member_frames.put(&k, frames.to_vec());
                member_of.put(&k, (ix, chart.clone()));
            }
        }
    };
    // 53.6 — A COVER'S CARRIED PIECES ARE GROUPED BY TRANSPORT CLASS,
    // exactly as a chart's quotients are: ONE Word vertex per class present
    // in the cover, one member per chart the class carries a piece on.
    //
    // Each piece was its own vertex. That is what made a form of thousands:
    // `484b58aa` read 2 055 vertices, 11 167 faces and 3 986 loops, and
    // every carrier the engine has is multiplied by that count — the
    // address, the indexes, the genome entry, the resident hold. A piece is
    // not an idea; the CLASS that carries the pieces is, and 21 step 1
    // already says two bodies carried by one transport are one vertex.
    //
    // MULTIPLICITY IS A DEMAND, NOT A SECOND VERTEX (50.5, item 2 of the
    // ruling). Where one class carries two pieces on one chart by two
    // DIFFERENT carriages, there is no single member to write; that is the
    // chart's multiplicity and it leaves as a `Source` on that chart, never
    // as two vertices and never as two members.
    let mut piece_seen: Keyed<Vec<u8>> = Keyed::new();
    for c in &covers {
        let frame = c.frame as u16;
        for (_, (v, _versor, sup)) in c.cover.carried.scoped() {
            let codomain = &sup.codomain;
            let (Some(chart), Some(tc)) = (v.face_chart(), transport_class(v)) else { continue };
            let Some(acting) = v.acting_on(&chart) else { continue };
            let Vertex::Word { address, derivation, .. } = v else { continue };
            let support: Vec<(i64, i64, i64)> =
                codomain.iter().map(|&(r, cc, _)| (r, cc, c.frame)).collect();
            // One class, one chart, two carriages: the multiplicity.
            // The class opens a member coordinate — class ‖ chart — and the
            // transport coordinate is left behind where the extension begins.
            let mut mk = tc.key().encode().as_bytes().to_vec();
            crate::transducer::address::put_bytes(&mut mk, chart.key().as_bytes());
            let mut carriage = Vec::new();
            acting.carriage.put_canonical(&mut carriage);
            match piece_seen.get(&mk) {
                Some(held) if held != &carriage => {
                    let links: Vec<Link> =
                        support.iter().map(|&(r, cc, _)| (r, cc, 0u8, 0u8)).collect();
                    demanded.push((Demand::Source { multiplicity: 2 }, chart.clone(), links));
                    continue;
                }
                Some(_) => {}
                None => {
                    piece_seen.insert(&mk, carriage);
                }
            }
            let member = Member {
                chart: chart.clone(),
                acting: acting.clone(),
                frames: vec![frame],
                at: Vec::new(),
                support,
                union: None,
                section_words: Vec::new(),
            };
            match reads.at_mut(&tc.key()) {
                Some(read) => {
                    if address < &read.address {
                        read.address = address.clone();
                        read.derivation = derivation.clone();
                    }
                    read.members.push(member);
                }
                None => {
                    reads.put(
                        &tc.key(),
                        ClassRead {
                            address: address.clone(),
                            derivation: derivation.clone(),
                            members: vec![member],
                        },
                    );
                }
            }
        }
    }

    for (_, read) in reads.scoped() {
        let mut acts: Keyed<Acting, coord::Section> = Keyed::new();
        for m in &read.members {
            acts.put(&m.acting.key(), m.acting.clone());
        }
        let vertex = Vertex::Word { address: read.address.clone(), derivation: read.derivation.clone(), acts, contact: None };
        let ix = scenery.place(vertex, Vec::new());
        for m in &read.members {
            let ck = m.chart.key();
            scenery.supports.insert(member_key(ix, ck.as_bytes()), m.support.clone());
            if let Some((ua, ub, uv)) = &m.union {
                unions.insert(member_key(ix, ck.as_bytes()), (ua.clone(), ub.clone(), uv.clone()));
            }
            for (k, a0, b0, v) in &m.at {
                // The vertex ordinal opens a body coordinate — ordinal ‖
                // chart ‖ body — and is left behind where that begins.
                let mut bk = vertex_ix_key(ix).encode().as_bytes().to_vec();
                crate::transducer::address::put_bytes(&mut bk, ck.as_bytes());
                bk.extend_from_slice(&k.to_le_bytes());
                bodies_at.insert(bk, (a0.clone(), b0.clone(), v.clone()));
            }
            claim_member(&scenery, &mut member_frames, &mut member_of, ix, &m.chart, &m.frames);
            for (w, fr) in &m.section_words {
                section_faces.push((ix, w.clone(), fr.clone(), None));
            }
        }
    }

    // THE SECTIONS: a member claimed on fewer frames than the family
    // reached is filed at its own key, and a class claimed on fewer at its
    // — the union over every vertex of the class.
    let mut class_union: Keyed<Vec<u16>, coord::Transport> = Keyed::new();
    for (mk, frames) in member_frames.entries() {
        let Some((ix, _)) = member_of.at(&mk) else { continue };
        let Some(v) = scenery.vertices.get(*ix as usize) else { continue };
        let mut frames = frames.clone();
        frames.sort_unstable();
        frames.dedup();
        if let Some(tc) = transport_class(v) {
            match class_union.at_mut(&tc.key()) {
                Some(u) => u.extend_from_slice(&frames),
                None => {
                    class_union.put(&tc.key(), frames.clone());
                }
            }
        }
        if frames.len() < n_reached {
            // THE MEMBER DEPTH of the section system (54.7 R5) — `mk` is a
            // member key and the class-level write below is the coarse
            // stratum of the same system, not a second carrier.
            sections.put(&coord::SectionAt::Member(Box::new(mk.clone())), section_of_frames(frames));
        }
    }
    // The key stays a transport coordinate from the walk to both writes.
    for (tk, frames) in class_union.entries() {
        let mut frames = frames.clone();
        frames.sort_unstable();
        frames.dedup();
        if frames.len() < n_reached {
            // THE CLASS DEPTH — the coarse stratum, written through the one
            // conversion that says so.
            sections.put(
                &coord::SectionAt::Class(tk.clone()),
                section_of_frames(frames.iter().copied()),
            );
        }
        varying.class_frames.put(&tk, frames);
    }
    let mut form = Form { vertices: scenery.vertices.clone(), ..Form::default() };
    trace::trace("percept/3 meets", trace::Wrote::Nothing, Some(budget));
    // 53.6 items 3 and 4, in their own organ: the pieces' form is built,
    // its loops are classified, and it is discarded. What comes back is
    // what the SCENE keeps — the torsion each loop filed, awaiting a chord
    // this form can name, and the classes standing per cover.
    let mut edges = EdgeTally::default();
    let (meet_chords, meet_faces, pending_torsion, stood) =
        crate::transducer::family::covered::classify_cover(&covers, &mut edges);
    trace::trace("percept/4 loops+classify", trace::Wrote::Nothing, Some(budget));
    // THE DEMANDS, one vertex per (kind, chart) (50.5), in key order. A
    // deformation dissolves when every lamina of both bodies is carried AND
    // no loop over its pieces stands (50.8).
    let mut by_demand: Keyed<Demanded, coord::DemandVertex> = Keyed::new();
    let mut declare = |kind: Demand, chart: Invariant, links: Vec<Link>| {
        let key = crate::transducer::sentence::demand_key(&kind, &chart);
        match by_demand.at_mut(&key) {
            Some((_, _, held)) => held.extend(links),
            None => {
                by_demand.put(&key, (kind, chart, links));
            }
        }
    };
    for (kind, chart, links) in demanded {
        declare(kind, chart, links);
    }
    for (ci, c) in covers.iter().enumerate() {
        let standing = stood.get(&ci.to_le_bytes()).is_some();
        if c.cover.uncarried == [0; 4] && !standing {
            continue;
        }
        let (kind, chart, links) = c.demand.clone();
        declare(kind, chart, links);
    }
    for (_, (kind, chart, links)) in by_demand.scoped() {
        let mut links = links.clone();
        links.sort_unstable();
        links.dedup();
        if kind.typed().is_err() {
            continue;
        }
        scenery.place(demand_vertex(kind.clone(), chart, links), Vec::new());
    }
    form.vertices = scenery.vertices.clone();
    let nv = form.vertices.len();

    trace::trace("percept/6 site faces", trace::Wrote::Nothing, Some(budget));
    // THE INCIDENCE IS THE OCCUPANCY (21 step 2): star faces from the Frame
    // and faces between members whose supports meet or sit at ring distance
    // one, each carrying 50.3's edge measured on the union of the two
    // bodies at the least frame both stand on, and claiming the frames that
    // edge re-derives on.
    let frame_member = scenery
        .vertices
        .get(frame_ix as usize)
        .and_then(|v| v.face_chart())
        .map(|c| member_key(frame_ix, c.key().as_bytes()));
    // WHAT WAS OFFERED AND WHAT WAS DROPPED, at the step that dropped it.
    // Five `continue`s stood here and none of them counted: a form came out
    // with two faces on nine vertices and nothing said which reader
    // discarded the other six. A face offered and refused is a measurement
    // (§8.1) and it was being thrown away silently.
    let mut drops = FaceDrops::default();
    // 53.6 item 3 — THE COVER'S MEETS ARE OFFERED AS FACES, BETWEEN THE
    // CLASSES THEY RELATE.
    //
    // `named_faces` (then `star_and_site_faces`) offered the Frame's star, a
    // spanning CHAIN of each site's occupants and the ring pairs. Its own comment says what
    // that leaves: "the only cycles that survive are the ones spanning
    // DISTINCT sites". Nothing offered such a pair, so every form came out
    // a forest — `faces == site+ring` on every row of every recut — and
    //
    //     b₁ = E − V + C = 0
    //
    // identically. `0 loops` everywhere, and with no cycle there is no
    // held obstruction, so `pivot_loop` had nothing to discharge:
    // `pivoted [0,0,0,0]` on all five voices at epoch 94, `stood` and
    // `closed` likewise, and `court basis 0` on all three courts. Descent
    // worked because descent asks whether the genome already holds a word
    // that reproduces the demand, which needs no cycle; everything above
    // grade 0 consumes H₁ and H₁ was empty.
    //
    // The chords are the meets lifted to classes by `transport_class`, the
    // same lift the loops' ENDS already used one screen down. They are
    // offered to THIS builder and not pushed as faces directly, so a meet
    // is subject to every reading a face is — the word must re-derive, the
    // place must exist — and a chord that cannot be built is counted in
    // `drops` beside the others rather than forced in (§8.1).
    //
    // `by_transport` is the index that resolves a class to its vertex, and
    // it is built by `reindex`; the scene's vertices are final above, so it
    // is built here rather than after the faces that need it.
    form.reindex();
    // THE CHORD IS BETWEEN TWO MEMBERS, AND THE CHART IS THE HALF THAT
    // DIFFERS (53.6 item 3, corrected by the lift's own reading).
    //
    // Measured over the 36 corpus forms: 186 meets, 186 of them carrying an
    // edge, `unclassed` 0 — and 186 of 186 WITHIN ONE CLASS. A first cut
    // kept only meets whose CLASSES differed and therefore kept none.
    //
    // That is what a `TransportClass` is. 21 step 1 excludes the carriage
    // deliberately — "never the hue map and never the body's moduli" — and
    // that exclusion is what makes two copies of one shape at two places
    // CORRESPOND, so a rule generalises. A deformation that moves a rigid
    // shape carries every piece by the same rotation and differs only in
    // TRANSLATION, so every piece lands at one class vertex by construction.
    //
    // The cycle therefore does not span two class vertices; it spans two
    // MEMBERS of one — `(V, chart A)` and `(V, chart B)`, each with its own
    // carriage — and the relative transport between them is
    // `τ = φ_B · φ_A⁻¹`, a Burgers vector, which is what
    // `Demand::Dislocation { chart, tau }` was built to carry. 18.2 says it
    // in one line: two carriages on one class are two sections.
    //
    // A face is between two members and `named_faces` already
    // returns member pairs, so the chord is offered at the same granularity
    // the builder consumes. Both ends may sit at ONE vertex; what must
    // differ is the member key, which is the pair `(vertex, chart)`.
    type OfferedChord = (Vec<u8>, Vec<u8>, Option<crate::transducer::sentence::Transport>);

    // AXIOM B's QUANTUM `M(X)` — THE FORM'S OWN LATTICE SCALE, and one
    // chart for every chord of this percept.
    //
    // `M` must be a property of the form, not of a chord: two edges of one
    // cycle computing their own `M` would be transitions between DIFFERENT
    // stereographic charts, and composing them around the loop would be
    // composing maps with no common domain — the holonomy would read as
    // curvature that is only a change of ruler.
    //
    // The scene's extent bounds every centre of mass it can hold: a cell
    // stands at `0 ≤ r < rows`, `0 ≤ c < cols`, `0 ≤ k < hues`, so any
    // centroid has `|p|² < rows² + cols² + hues²`. Taking
    // `M = 1 + rows + cols + hues` gives `M² > rows² + cols² + hues² > |p|²`
    // strictly, so `|u| = |p|/M < 1` and every footprint of this form lands
    // inside the stereographic image of the sheet. Derived from the scene
    // rather than chosen, so a bigger scene gets a bigger ruler and the
    // hyperbolic distances stay comparable within the form.
    // AXIOM B's CHART FOR THIS FORM — its own origin `P(X)` and quantum
    // `M(X)`, one stereographic chart for every chord it carries.
    //
    // Two earlier cuts were both flat. The first took `M = 1 + rows + cols
    // + 256`; the second replaced the 256 with the scene's hue extent. Both
    // are the AXIS-ALIGNED BOUNDING BOX of the corpus — `max` along each
    // axis in turn — carried past the sensor boundary into the connection,
    // and both SUM INCOMMENSURABLE AXES, adding a palette index to two
    // lattice positions as though a hue were a length.
    //
    // Worse, the lamina was fed ABSOLUTE `(r, c, k)`. The same form drawn
    // at the origin and at the far corner of a grid then lands on different
    // sheet points and reads as different geometry — position standing in
    // for coordinate (rule 15), inside the connection itself.
    //
    // Both dissolve into one construction. The form's centre and its total
    // second moment:
    //
    // ```text
    //     P = (1/N) Σ pᵢ            translation-equivariant: P(g·X) = g·P(X)
    //     S = Σ |pᵢ − P|²           one scalar, rotation-invariant, no max
    //     M = 1 + S
    // ```
    //
    // `S` is a MOMENT, not an EXTENT: every cell enters symmetrically and
    // no axis is privileged, so there is no box. The chart bound comes
    // free — a sub-domain's centre lies in the convex hull of the whole, so
    // `|c − P|² ≤ maxᵢ|pᵢ − P|² ≤ S`, and `M ≥ 1` gives `M² ≥ M > S`
    // strictly. Every footprint of this form therefore lands inside the
    // stereographic image of the sheet, and feeding `c(D) − P` rather than
    // `c(D)` makes the reading invariant under where the scene was drawn.
    //
    // OPEN, and flagged rather than silently decided: the third axis is
    // HUE, and a hue is not a length. The `(row, col, hue)` lamina is the
    // sensor's existing convention (`blade::shells`), so the moment mixes a
    // palette index into a spatial scale. This predates the chart and is
    // not repaired here; it wants a ruling on whether hue is a coordinate
    // of the base or of the fibre.
    // A.11's SINGLE PRODUCER (56.3, ruled 2026-09-22). This summed the
    // cells a second time — a SHADOW PRODUCER of moments `blade::lift`
    // already computes exactly, and a lossy one: it took each cell as a
    // POINT MASS at integer coordinates, where `lift` integrates the
    // lamina's own unit square and carries the parallel-axis term
    // `N/12` at every order. Two producers of one quantity, and the
    // second disagreed with the first (law 12).
    //
    // The form's chart is one body — the union of every cell its chords
    // stand on — so `centroid()` is `P` and the trace of `inertia` is the
    // scale. The trace is used HERE and only here, as a radius: it is an
    // `SO(2)` invariant and carries no orientation (A.11), which is what
    // the frame rotor is for and not this.
    let (origin, quantum) = {
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let mut cells: Vec<(i64, i64, i64)> = Vec::new();
        for ((_, _, _, da), (_, _, _, db), _) in meet_chords.iter() {
            cells.extend(da.iter().copied());
            cells.extend(db.iter().copied());
        }
        cells.sort_unstable();
        cells.dedup();
        match crate::transducer::blade::shape_of(&cells).blade {
            // A form whose chords stand on nothing has no chart to speak
            // of; the pole and unit scale are the honest empty reading.
            b if b.support.is_empty() => ([zero.clone(), zero.clone(), zero], one),
            b => {
                let (c1, c2, c3) = match b.centroid() {
                    Some(p) => p,
                    // A body whose centroid does not read back is refused
                    // rather than defaulted (rule 11).
                    None => (zero.clone(), zero.clone(), zero.clone()),
                };
                ([c1, c2, c3], one + (&b.inertia[0] + &b.inertia[1]))
            }
        }
    };
    let (source, image) = cover_nerves(&covers);
    let spectrum = phase_spectrum(&meet_chords);
    let chord_pairs: Vec<OfferedChord> = meet_chords
        .iter()
        .filter_map(|((ta, ca, pa, da), (tb, cb, pb, db), e)| {
            let at = |t: &TransportClass,
                      c: &Invariant,
                      phi: &crate::transducer::element::Versor,
                      d: &[(i64, i64, i64)]|
             -> Option<Vec<u8>> {
                // A class stands at one or more vertices and any of them
                // names the same class, so the slice pattern reads one
                // rather than choosing among them (rule 15). The SECTION is
                // that member and the carriage it acts by, which is the
                // coordinate separating two sheets of one fibre.
                //
                // 54.7 — AND THE FOOTPRINT, which is what separates two
                // sections carried by ONE versor. This key omitted it while
                // `classify_cover`'s gate included it, so 184 of 186 chords
                // arrived here with both ends identical and were dropped by
                // the `ma != mb` below. The two ends of a chord are told
                // apart by the same coordinates at both places or they are
                // not told apart at all (rule 12: one construction).
                let ix = match form.by_transport.at(&t.key())?.as_slice() {
                    [ix, ..] => *ix,
                    [] => return None,
                };
                let mut k = section_key(ix, c.key().as_bytes(), phi);
                for (r, col, f) in d {
                    k.extend_from_slice(&r.to_le_bytes());
                    k.extend_from_slice(&col.to_le_bytes());
                    k.extend_from_slice(&f.to_le_bytes());
                }
                Some(k)
            };
            let (ma, mb) = (at(ta, ca, pa, da)?, at(tb, cb, pb, db)?);
            // ONE SECTION MEETING ITSELF IS NOT A CHORD — same class, same
            // chart, same carriage AND same footprint. Anything else is two
            // sections and closes a cycle.
            if ma == mb {
                return None;
            }
            // THE TRANSPORT ACROSS THE INTERFACE. Where the meet measured
            // one, that stands. Where it did not — two disjoint bodies under
            // the identical transformation, `φ_a = φ_b`, so the relative
            // carriage is the identity and says nothing — the transport is
            // the SOLDER FORM displacement between the two basepoints:
            //
            //     τ = p(D_b) − p(D_a) ≠ 0
            //
            // a hyperbolic transvection, `B = Σ τᵢ (eᵢ∧e₋)`, evolved to a
            // finite versor by the rational Cayley transform. A chord whose
            // holonomy is the identity carries no obstruction and would
            // leave the cycle unreadable; this is the Burgers vector the
            // dislocation actually has.
            let edge = dispiration(&origin, &quantum, e.as_ref(), da, db);
            Some((ma, mb, edge))
        })
        .collect();
    let n_chords = chord_pairs.len();
    // A STAR, SITE OR CONTACT PAIR BRINGS NO EDGE OF ITS OWN — `edge_between`
    // reads it off the two bodies, as it reads any meet's (50.3) — and a
    // CHORD brings the one its meet measured, because `edge_between` is
    // keyed by `(vertex, chart)` and cannot re-derive a transition between
    // two sections of one member.
    //
    // A PAIR BECOMES ONE FACE PER INTERFACE IT RUNS ALONG (C2, 70.6): a flat
    // interface and a ridged one between one pair are two contacts. A pair
    // no contact named carries no interface and is one face.
    struct Offer {
        a: Vec<u8>,
        b: Vec<u8>,
        edge: Option<crate::transducer::sentence::Transport>,
        hows: Vec<Incidence>,
        interface: Vec<(i64, i64, i64)>,
    }
    let mut offers: Vec<Offer> = Vec::new();
    for nf in named_faces(frame_member.as_deref(), &scenery.supports) {
        let (a, b, hows) = (nf.a, nf.b, nf.named.hows);
        match nf.named.interfaces.as_slice() {
            [] => offers.push(Offer { a, b, edge: None, hows, interface: Vec::new() }),
            each => offers.extend(each.iter().map(|i| Offer {
                a: a.clone(),
                b: b.clone(),
                edge: None,
                hows: hows.clone(),
                interface: i.clone(),
            })),
        }
    }
    offers.extend(chord_pairs.into_iter().map(|(a, b, edge)| Offer {
        a,
        b,
        edge,
        hows: Vec::new(),
        interface: Vec::new(),
    }));
    for Offer { a: ak, b: bk, edge: chord_edge, hows: how, interface } in offers {
        drops.offered += 1;
        let contact = how.contains(&Incidence::Contact);
        if contact {
            drops.contact_offered += 1;
        }
        let (Some((i, cik)), Some((j, cjk))) = (read_member_key(&ak), read_member_key(&bk)) else {
            drops.member_key += 1;
            continue;
        };
        let (measured, pairs) =
            edge_between(&bodies_at, &unions, n_reached, (i, &cik), (j, &cjk), &mut edges);
        // THE CHORD'S OWN TRANSITION WINS where it has one: the meet
        // measured `V_b ∘ V_a⁻¹` between the two pieces' frames, and the
        // bodies at one member cannot say what the transition between two
        // of its sections is. Where the chord carries none, the reading off
        // the bodies stands.
        let edge = chord_edge.or(measured);
        let (Some(vi), Some(vj)) = (form.vertices.get(i as usize), form.vertices.get(j as usize))
        else {
            drops.vertex += 1;
            continue;
        };
        let (Some(ci), Some(cj)) = (vi.chart_keyed(&cik), vj.chart_keyed(&cjk)) else {
            drops.chart += 1;
            continue;
        };
        let Some(word) = form.face_word(i, j) else {
            drops.word += 1;
            continue;
        };
        let Some(place) = form.face_place(i, &ci, j, &cj, edge) else {
            drops.place += 1;
            continue;
        };
        let mut at = Keyed::new();
        at.insert(face_pair_key(&ci, &cj), place);
        form.faces.push(FormFace { i, j, word, at, pairs, interface });
        drops.built += 1;
        if contact {
            drops.contact_built += 1;
        }
    }
    // THE WORDS THE FAMILY SPOKE ON OTHER FRAMES between the Frame and a
    // vertex are faces of their own, each on the frames it was spoken on
    // (21 step 2, S12.2).
    for (ix, w, fr, edge) in section_faces {
        let Some(mut face) = form.face_carrying_with_edge(frame_ix, ix, w, edge) else { continue };
        face.pairs = Some(section_of_frames(fr));
        form.faces.push(face);
    }
    form.loops = Form::loops_of(&form.faces, &form.vertices);
    form.hue_maps = hue_maps;
    form.reindex();
    // 53.6 item 4 — THE COVER'S FILED LOOPS TAKE A CHORD OF THE SCENE.
    // Each was classified inside the cover; its chord there indexed the
    // pieces' own form. Here it takes the face between the two CLASS
    // vertices its ends belong to, which is a coordinate this form has. A
    // loop whose classes the scene relates by no face is NOT filed: there
    // is no chord to name and none is invented (rule 11).
    let mut standing_demands: Vec<(Demand, Invariant)> = Vec::new();
    for (class, mut filed, ends, rotor) in pending_torsion {
        let ixs: Vec<u16> = ends
            .iter()
            .filter_map(|tc| form.by_transport.at(&tc.key()))
            // A class stands at one or more vertices; the loop's end is
            // the class, so any vertex of it names the same class and the
            // slice pattern reads one rather than choosing among them.
            .filter_map(|on| match on.as_slice() {
                [ix, ..] => Some(*ix),
                [] => None,
            })
            .collect();
        let chord = match ixs.as_slice() {
            [a, b, ..] => form.faces.iter().position(|f| {
                (f.i == *a && f.j == *b) || (f.i == *b && f.j == *a)
            }),
            _ => None,
        };
        match chord {
            Some(c) => {
                filed.chord = c as u16;
                form.torsion.put(&class, filed);
            }
            // A MEASURED CLASS IS NEVER DISCARDED (law 7, rule 11). The
            // loop was classified inside the cover and its class is a
            // reading the engine made; the scene simply has no face
            // between the two classes to name a chord in, which is a fact
            // about this form's incidence and not about the class. So it
            // leaves as the demand it IS — `Standing`, with the holonomy's
            // own rotor as the witness — and the line below counts it.
            //
            // The first cut of 53.6 dropped these, which is the fault
            // 53.6's own item 2 refuses one paragraph earlier: a thing
            // measured and then not written down.
            None => {
                let chart = ends
                    .iter()
                    .filter_map(|tc| form.by_transport.at(&tc.key()))
                    .filter_map(|on| match on.as_slice() {
                        [ix, ..] => form.vertices.get(*ix as usize),
                        [] => None,
                    })
                    .find_map(crate::transducer::sentence::Vertex::face_chart);
                if let (Some(chart), Some(d)) =
                    (chart, crate::transducer::alphabet::standing_of(&rotor))
                {
                    standing_demands.push((d, chart));
                }
            }
        }
    }
    for (kind, chart) in standing_demands {
        crate::transducer::family::cover::tally(|t| t.standing += 1);
        let ix = form.vertices.len() as u16;
        form.vertices.push(demand_vertex(kind, &chart, Vec::new()));
        let _ = ix;
    }
    form.reindex();
    // READ WHETHER OR NOT ANYTHING CLOSED. It was gated on
    // `!form.loops.is_empty() || meet_faces > 0`, so the one row that says
    // why a form is empty was printed only when the form was not empty:
    // fifteen forms crystallized at `loops 0` and this line never fired
    // once. A reading suppressed in the failure case is not a reading.
    let unread_why = crate::transducer::sentence::Form::unread_gates();
    eprintln!(
        "  family form {nv} vertices, {} faces ({meet_faces} cover meets, {} site+ring), {} loops \
         | edges asked: measured {}, unspelled {}, unread {}, not rigid {} \
         | chords {} offered {} | cycles: reflexive {} no-tree-path {} unread {} \
         (cone {} no place {} not rigid {} no face {}; edge refused: cone {} not rigid {}) \
         trivial {} (vacuum {} resonant {} over {} charged edges; \
         μ read on {} steps, {} of them non-zero) \
         | defects: trivial {} elliptic {} hyperbolic {} parabolic {} non-simple {} \
         | spectra: closes {} open {} undecomposed {} isoclinic {} \
         | ρ: |im| {} |conj| {} of {} generators, {} in ker | this form: b₁ {} |im| {} \
         | field: {} \
         | planes: {} \
         | mint: {} \
         | metabolism: {} syntropy {} \
         | nerve/source: {} charts over {} cells, {} shared by two or more, \
         {} certified triples, {} refused \
         | nerve/image: {} charts over {} cells, {} shared, {} triples \
         | phase: {} finite {} infinite {} unread, periods {:?} \
         | faces offered {} built {} dropped: member_key {} vertex {} chart {} word {} place {} \
         | contact faces offered {} built {}",
        form.faces.len(),
        // The site+ring faces COUNTED as they were built, not the total
        // less the meets: a subtraction floored at zero invents a value
        // where the two carriers disagree (rule 11), and `drops.built` is
        // the measurement this label was reaching for.
        drops.built,
        form.loops.len(),
        edges.measured,
        edges.unspelled,
        edges.unread,
        edges.not_rigid,
        meet_chords.len(),
        n_chords,
        crate::transducer::sentence::Form::loop_gates().0,
        crate::transducer::sentence::Form::loop_gates().1,
        crate::transducer::sentence::Form::loop_gates().2,
        // H2c — the unread gate by what refused it, and the read cycles
        // whose edge half was refused.
        unread_why.0.cone,
        unread_why.0.no_place,
        unread_why.0.not_rigid,
        unread_why.0.no_face,
        unread_why.1.cone,
        unread_why.1.not_rigid,
        crate::transducer::sentence::Form::loop_gates().3,
        // HOW THE CLOSURES DIVIDE. `trivial` above counts every cycle whose
        // boundary product is the identity; these two say whether that
        // identity was a cancellation or an absence (54.7).
        crate::transducer::sentence::loop_resonance().0,
        crate::transducer::sentence::loop_resonance().1,
        crate::transducer::sentence::loop_resonance().2.len(),
        crate::transducer::sentence::loop_resonance().3,
        crate::transducer::sentence::loop_resonance().4,
        crate::transducer::sentence::loop_defects().0,
        crate::transducer::sentence::loop_defects().1,
        crate::transducer::sentence::loop_defects().2,
        crate::transducer::sentence::loop_defects().3,
        crate::transducer::sentence::loop_defects().4,
        crate::transducer::sentence::loop_spectra().0,
        crate::transducer::sentence::loop_spectra().1,
        crate::transducer::sentence::loop_spectra().2,
        crate::transducer::sentence::loop_spectra().3,
        crate::transducer::sentence::loop_synonyms().0,
        crate::transducer::sentence::loop_synonyms().1,
        crate::transducer::sentence::loop_synonyms().2,
        crate::transducer::sentence::loop_synonyms().3,
        form.loops.len(),
        crate::transducer::sentence::form_operators(&form),
        // THE VALVES, ON THE FLEET'S OWN LINE (66). The field tally was
        // printed only by a corpus test, so a recut would have run blind:
        // every voice would have condensed or starved with nothing in the
        // journal to say which valve held. Read WITHOUT clearing — the
        // corpus test takes the total, this prints the running state, and
        // only one reader may take (law 12).
        crate::transducer::family::pairing::field_tally().line(),
        // D1 — DIMENSIONAL CAPACITY, PER FORM. Ten planes by name, the
        // harmonic six and the luminal four kept apart, and how many
        // generators coupled the halves (a screw, where `I₂ ≠ 0` lives).
        crate::transducer::cga::plane_tally().line(),
        // D3 — THE MINT, PER FORM. Where the unspellable turns stop: the
        // three gates by name and what reached the court. 37.7's pipeline
        // measured `adjoin 0` with nothing to say whether it was starved
        // or shut.
        crate::transducer::family::perceive::demand_tally().line(),
        // D2 — THE THERMODYNAMIC LEDGER. Entropy is the cauterizations,
        // the orbit structure is `ℓ`, and the rate is the reserve's own
        // per-lane draw. SYNTROPY IS READ FROM THE FIELD'S COUNTER, not
        // duplicated: condensations are already counted there, and a
        // second counter for one fact is law 12 inside one carrier.
        crate::physics::metabolism::metabolism().line(),
        crate::transducer::family::pairing::field_tally().condensed,
        source.charts,
        source.cells,
        source.shared,
        source.triples.len(),
        source.refused.len(),
        image.charts,
        image.cells,
        image.shared,
        image.triples.len(),
        spectrum.0,
        spectrum.1,
        spectrum.2,
        spectrum.3,
        drops.offered,
        drops.built,
        drops.member_key,
        drops.vertex,
        drops.chart,
        drops.word,
        drops.place,
        drops.contact_offered,
        drops.contact_built
    );
    trace::trace("extract/2 faces", trace::Wrote::Nothing, Some(budget));
    Percept { form, sections, varying, unreached }
}
