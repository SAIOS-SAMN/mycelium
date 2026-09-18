// 21 as ruled in 52.5 — EXTRACTION: the family's one form. Every chart's
// fiber is read frame by frame as a quotient in the frame's own
// coordinates; frames that read one versor are one vertex, frames that
// read another are sections along e₃, and what no quotient reaches is a
// cover or a demand. No correspondence, no candidate list, no search.
//
//! The percept — the peeled form, its sections, what move 6 reads and the
//! frames the budget did not reach — is memoised on the sensor's bytes; the
//! derivation against a mind's presentation runs per call (50.2 step 2),
//! and moves 3 to 7 run on the one form after it.

use super::cover::cover_deformation;
pub use super::finish::assignment_key;
use super::finish::finish;
use super::pair::{delta_links, meet_edge, Covered, Scenery};
use super::perceive::{
    at_frame_zero, demand_vertex, differing_axes, frame_of_body, perceive_at, support_at,
    word_or_demand_on_chart, Demanded, EdgeTally,
};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::{vertex_ix_key, Acting, Demand, FormFace, Transport};
use std::cell::RefCell;

type Ratio = ((usize, usize), (usize, usize));

/// THE PERCEPT OF A FAMILY (52.5): the peeled form, the sections along
/// `e₃`, what move 6 reads, and the frames the budget did not reach. The
/// derivation against a mind's presentation is not in it (50.2 step 2).
#[derive(Clone)]
pub(crate) struct Percept {
    pub(crate) form: Form,
    pub(crate) sections: Keyed<Vec<u16>>,
    pub(crate) varying: contact::Varying,
    pub(crate) unreached: Vec<usize>,
}

/// The percept, memoised on the sensor's bytes at the sensor boundary —
/// the one place bytes may be a key (rule 6). A percept the budget cut
/// short is not memoised: the next caller with a fuller stroke would be
/// handed this one's shortfall as if it were the scene (rule 11).
struct PerceptMemo {
    family: Vec<(Grid, Grid)>,
    scale: Ratio,
    percept: Percept,
}

thread_local! {
    static PERCEPT_MEMO: RefCell<Option<PerceptMemo>> = const { RefCell::new(None) };
}

fn frame_key(k: i64) -> [u8; 8] {
    k.to_le_bytes()
}

fn frame_of_key(k: &[u8]) -> Option<i64> {
    Some(i64::from_le_bytes(k.try_into().ok()?))
}

/// A body's address as content: its laminae written out (A21).
fn body_key(b: &Body) -> Vec<u8> {
    let mut k = Vec::with_capacity(b.laminae.len() * 25);
    for &(r, c, f, h) in &b.laminae {
        k.extend_from_slice(&r.to_le_bytes());
        k.extend_from_slice(&c.to_le_bytes());
        k.extend_from_slice(&f.to_le_bytes());
        k.push(h);
    }
    k
}

fn laminae_of(bodies: &[Body]) -> Vec<Lamina> {
    bodies.iter().flat_map(|b| b.laminae.iter().copied()).collect()
}

/// THE FIBERS OF A SIDE: chart key → the chart, and frame → the sections
/// of that chart standing there (Y4, 24.4, 52.5 item 2). Keyed reads all
/// the way down; nothing is walked to find a body.
fn fibers(bodies: &[Body]) -> Keyed<(Invariant, Keyed<Vec<Body>>)> {
    let mut out: Keyed<(Invariant, Keyed<Vec<Body>>)> = Keyed::new();
    for b in bodies {
        let Some(k) = frame_of_body(b) else { continue };
        let ck = b.moduli.key();
        if !out.contains(&ck) {
            out.insert(&ck, (b.moduli.clone(), Keyed::new()));
        }
        let Some((_, frames)) = out.get_mut(&ck) else { continue };
        match frames.get_mut(&frame_key(k)) {
            Some(v) => v.push(b.clone()),
            None => {
                frames.insert(frame_key(k), vec![b.clone()]);
            }
        }
    }
    out
}

/// Two transports are one edge: the word projectively, the carriage as a
/// versor, the gauge as a listing (rule 3).
fn same_transport(a: &Transport, b: &Transport) -> bool {
    a.word.rotor().projective_eq(&b.word.rotor())
        && element::same_versor(&a.carriage, &b.carriage)
        && a.gauge == b.gauge
}

/// WHAT ONE FRAME'S QUOTIENT OF ONE CHART MEASURED: the frame, the versor
/// with its hue map, and the two union bodies in the frame's own
/// coordinates.
struct Quotient {
    frame: u16,
    car: Carriage,
    a0: Body,
    b0: Body,
}

/// ONE MEMBER OF A CLASS'S VERTEX (D7): the chart, what it acts by there,
/// the frames its versor holds on, the bodies at each of them, the output
/// support over those frames, and the words the other frames of the chart
/// spoke as sections (21 step 2).
struct Member {
    chart: Invariant,
    acting: Acting,
    frames: Vec<u16>,
    at: Vec<(u16, Body, Body, Versor)>,
    support: Vec<(i64, i64, i64)>,
    section_words: Vec<(crate::transducer::alphabet::Word, Vec<u16>)>,
}

/// A class's vertex before it is placed: the least address among its
/// members' readings, that member's derivation, and every member.
struct ClassRead {
    address: crate::transducer::address::Address,
    derivation: crate::transducer::alphabet::Derivation,
    members: Vec<Member>,
}

/// The demands a cover leaves on the union of a chart's fiber at one frame
/// (50.5, 50.8 case 4). An output lamina no piece reached is what a
/// `Source` over the chart stands for, with the number of input instances
/// standing there, every one of them standing and none first-found: the
/// input laminae no piece reached ride the same demand, since they are the
/// bodies the outputs may be the image of. Where every output lamina was
/// reached, an input instance no piece of which was reached vanished — the
/// empty chart on every axis — and what else was left on the input rides a
/// `Source` of its own. The first demand is the one the cover's standing
/// loops keep alive.
fn unreached_demands(
    instances: &[Body],
    a0: &Body,
    b0: &Body,
    reached: &Keyed<()>,
    chart: &Invariant,
    n_in: u32,
) -> Vec<Demanded> {
    let mut out_links: Vec<Link> = Vec::new();
    for &(r, c, f, h) in &b0.laminae {
        if !reached.contains(&base_key(r, c, f)) {
            out_links.push((r, c, 0, h));
        }
    }
    let mut out = Vec::new();
    let mut gone: Keyed<()> = Keyed::new();
    if out_links.is_empty() {
        for body in instances {
            let untouched = body.laminae.iter().all(|&(r, c, _, _)| !reached.contains(&base_key(r, c, 0)));
            if untouched {
                out.push(vanished(body));
                for &(r, c, _, _) in &body.laminae {
                    gone.insert(base_key(r, c, 0), ());
                }
            }
        }
    }
    let mut links: Vec<Link> = out_links;
    for &(r, c, f, h) in &a0.laminae {
        if !reached.contains(&base_key(r, c, f)) && !gone.contains(&base_key(r, c, f)) {
            links.push((r, c, h, 0));
        }
    }
    links.sort_unstable();
    links.dedup();
    if !links.is_empty() || out.is_empty() {
        out.push((Demand::Source { multiplicity: n_in }, chart.clone(), links));
    }
    out
}

/// A body that vanished: the empty chart on every axis (50.5).
fn vanished(body: &Body) -> Demanded {
    let axes: Vec<u16> = (0..body.moduli.0.len() as u16).collect();
    let links: Vec<Link> = body.laminae.iter().map(|&(r, c, _, h)| (r, c, h, 0u8)).collect();
    (Demand::TopologicalDeformation { image: Invariant(Vec::new()), axes }, body.moduli.clone(), links)
}

/// THE PERCEPT (52.5 items 1 to 4), under the stroke (rule 10): every
/// frame the budget reaches is perceived once; every chart's fiber is read
/// frame by frame; a budget that ends leaves the frames it did not measure
/// as the demands of `finish` and says so.
fn family_percept(family: &[(Grid, Grid)], scale: Ratio, budget: &mut crate::physics::Budget) -> Percept {
    use crate::transducer::sentence::face_pair_key;
    trace::trace_n("extract/0 frames", family.len(), budget);
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

    trace::trace_n("extract/1 fibers", charts.len(), budget);
    let mut reads: Keyed<ClassRead> = Keyed::new();
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
            let Some(k) = frame_of_key(&fk) else { continue };
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
            ak.extend_from_slice(&frame_key(k));
            attempted.insert(&ak, ());
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
                    quotients.push(Quotient { frame: k as u16, car, a0, b0 });
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
                    demanded.push(d);
                    continue;
                }
                None => continue,
            };
            let Vertex::Word { address, derivation, acts, .. } = &vertex else { continue };
            let Some(tc) = transport_class(&vertex) else { continue };
            let Some(acting) = acts.get(c.key().as_bytes()).cloned() else { continue };
            let rotor = derivation.rotor();
            // THE HUE MAP IS A SECTION COORDINATE (21 step 1): every frame's
            // map is a hue section on the form, the vertex carries the least
            // frame's, and a frame whose map differs is a frame the member
            // is not claimed on — its placement varies (move 6).
            let mut frames: Vec<u16> = Vec::new();
            let mut at: Vec<(u16, Body, Body, Versor)> = Vec::new();
            let mut support: Vec<(i64, i64, i64)> = Vec::new();
            for q in rep {
                if let Some(hk) = crate::transducer::sentence::hue_section_key_of(&rotor, &q.car.sigma) {
                    hue_maps.insert(hk, q.car.sigma.clone());
                }
                if !crate::transducer::sentence::gauge_agrees(&q.car.sigma, &acting.gauge) {
                    varying.place_varies.insert(tc.key(), ());
                    continue;
                }
                frames.push(q.frame);
                at.push((q.frame, q.a0.clone(), q.b0.clone(), q.car.v.clone()));
                support.extend(support_at(&q.b0.laminae, i64::from(q.frame)));
            }
            support.sort_unstable();
            support.dedup();
            // THE OTHER VERSORS THE CHART READ ON OTHER FRAMES: the placement
            // varies (move 6's reading), and a word that differs is a face the
            // family spoke between the Frame and this vertex on those frames
            // (21 step 2, S12.2).
            let mut section_words = Vec::new();
            for (vk, group) in by_versor.scoped() {
                if vk == rep_key {
                    continue;
                }
                varying.place_varies.insert(tc.key(), ());
                let [g, ..] = group.as_slice() else { continue };
                let g_frames: Vec<u16> = group.iter().map(|q| q.frame).collect();
                if let Some(Ok(w)) = peel_versor(&g.car.v) {
                    if !w.rotor().projective_eq(&rotor) {
                        section_words.push((w, g_frames));
                    }
                }
            }
            let member = Member { chart: c.clone(), acting, frames, at, support, section_words };
            match reads.get_mut(&tc.key()) {
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
                    reads.insert(tc.key(), ClassRead { address: address.clone(), derivation: derivation.clone(), members: vec![member] });
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
            let fk = frame_key(k as i64);
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
            let fk = frame_key(k as i64);
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

    // THE VERTICES: a class's quotient with a member per chart (D7), and a
    // cover's pieces, each a vertex of the nerve it is a piece of (50.8),
    // claimed on the frame its cover was read on. The sections along e₃
    // are filed by member and by class (21 step 1) once every vertex
    // stands.
    let mut scenery = Scenery::new();
    let frame_ix = scenery.place(frame_vertex(), Vec::new());
    let mut sections: Keyed<Vec<u16>> = Keyed::new();
    let mut bodies_at: Keyed<(Body, Body, Versor)> = Keyed::new();
    let mut section_faces: Vec<(u16, crate::transducer::alphabet::Word, Vec<u16>)> = Vec::new();
    // Member section key → the frames that member is claimed on, and the
    // member the key names. Two members whose every coordinate agrees —
    // one action placed twice — are one key and claim the union.
    let mut member_frames: Keyed<Vec<u16>> = Keyed::new();
    let mut member_of: Keyed<(u16, Invariant)> = Keyed::new();
    let claim_member = |scenery: &Scenery,
                        member_frames: &mut Keyed<Vec<u16>>,
                        member_of: &mut Keyed<(u16, Invariant)>,
                        ix: u16,
                        chart: &Invariant,
                        frames: &[u16]| {
        let Some(k) = scenery.vertices.get(ix as usize).and_then(|v| member_section_key(v, chart)) else {
            return;
        };
        match member_frames.get_mut(&k) {
            Some(v) => v.extend_from_slice(frames),
            None => {
                member_frames.insert(&k, frames.to_vec());
                member_of.insert(&k, (ix, chart.clone()));
            }
        }
    };
    for (_, read) in reads.scoped() {
        let mut acts: Keyed<Acting> = Keyed::new();
        for m in &read.members {
            acts.insert(m.acting.key(), m.acting.clone());
        }
        let vertex = Vertex::Word { address: read.address.clone(), derivation: read.derivation.clone(), acts, contact: None };
        let ix = scenery.place(vertex, Vec::new());
        for m in &read.members {
            let ck = m.chart.key();
            scenery.supports.insert(member_key(ix, ck.as_bytes()), m.support.clone());
            for (k, a0, b0, v) in &m.at {
                let mut bk = vertex_ix_key(ix);
                crate::transducer::address::put_bytes(&mut bk, ck.as_bytes());
                bk.extend_from_slice(&k.to_le_bytes());
                bodies_at.insert(bk, (a0.clone(), b0.clone(), v.clone()));
            }
            claim_member(&scenery, &mut member_frames, &mut member_of, ix, &m.chart, &m.frames);
            for (w, fr) in &m.section_words {
                section_faces.push((ix, w.clone(), fr.clone()));
            }
        }
    }

    // THE COVER'S PIECES ARE VERTICES (50.8, L23c), at the frame the cover
    // was read on; their meets are faces below.
    let mut cover_vertex: Keyed<u16> = Keyed::new();
    let mut owner: Keyed<Keyed<()>> = Keyed::new();
    for (ci, c) in covers.iter().enumerate() {
        let frame = c.frame as u16;
        for (key, (v, _versor, codomain)) in c.cover.carried.scoped() {
            let Some(chart) = v.face_chart() else { continue };
            let support: Vec<(i64, i64, i64)> = codomain.iter().map(|&(r, cc, _)| (r, cc, c.frame)).collect();
            let ix = scenery.place(v.clone(), support);
            claim_member(&scenery, &mut member_frames, &mut member_of, ix, &chart, &[frame]);
            cover_vertex.insert(&key, ix);
            let mut set = Keyed::new();
            set.insert(ci.to_le_bytes(), ());
            owner.insert(vertex_ix_key(ix), set);
        }
    }
    // THE SECTIONS: a member claimed on fewer frames than the family
    // reached is filed at its own key, and a class claimed on fewer at its
    // — the union over every vertex of the class.
    let mut class_union: Keyed<Vec<u16>> = Keyed::new();
    for (mk, frames) in member_frames.scoped() {
        let Some((ix, _)) = member_of.get(&mk) else { continue };
        let Some(v) = scenery.vertices.get(*ix as usize) else { continue };
        let mut frames = frames.clone();
        frames.sort_unstable();
        frames.dedup();
        if let Some(tc) = transport_class(v) {
            match class_union.get_mut(&tc.key()) {
                Some(u) => u.extend_from_slice(&frames),
                None => {
                    class_union.insert(tc.key(), frames.clone());
                }
            }
        }
        if frames.len() < n_reached {
            sections.insert(&mk, frames);
        }
    }
    for (tk, frames) in class_union.scoped() {
        let mut frames = frames.clone();
        frames.sort_unstable();
        frames.dedup();
        if frames.len() < n_reached {
            sections.insert(&tk, frames.clone());
        }
        varying.class_frames.insert(&tk, frames);
    }
    let mut form = Form { vertices: scenery.vertices.clone(), ..Form::default() };
    // THE COVER'S MEETS ARE FACES carrying the edge between the two pieces'
    // own frames (50.3); a meet with no transport gets no face (13.9).
    let mut edges = EdgeTally::default();
    for c in &covers {
        for (mk, ()) in c.cover.meets.scoped() {
            let Some((ka, kb)) = crate::transducer::family::pieces::meet_ends(&mk) else { continue };
            let (Some(&i), Some(&j)) = (cover_vertex.get(ka), cover_vertex.get(kb)) else { continue };
            let (Some(vi), Some(vj)) = (form.vertices.get(i as usize), form.vertices.get(j as usize)) else { continue };
            let (Some(ci), Some(cj)) = (vi.face_chart(), vj.face_chart()) else { continue };
            let edge = edges.take(match (c.cover.carried.get(ka), c.cover.carried.get(kb)) {
                (Some((va, sa, _)), Some((vb, sb, _))) => Some(meet_edge(va, sa, vb, sb)),
                _ => None,
            });
            if edge.is_some() {
                crate::transducer::family::cover::tally(|t| t.edges += 1);
            }
            let Some(word) = form.face_word(i, j) else { continue };
            let Some(place) = form.face_place(i, &ci, j, &cj, edge) else { continue };
            let mut at = Keyed::new();
            at.insert(face_pair_key(&ci, &cj), place);
            form.faces.push(FormFace { i, j, word, at, pairs: None });
        }
    }
    let meet_faces = form.faces.len();
    form.loops = Form::loops_of(&form.faces, form.vertices.len());
    // EACH LOOP OVER CARRIED PIECES, CLASSIFIED ONCE (50.8 case 3), by the
    // one classifier both organs use, on its edge holonomy per member path.
    let mut stood: Keyed<Vec<crate::transducer::sentence::ClassVector>> = Keyed::new();
    for lp in form.loops.clone() {
        // THE LOOP'S COVER: the one cover that placed every vertex of it —
        // a vertex two covers' pieces became is in both — and none where a
        // vertex belongs to no cover or the vertices to different ones.
        let mut common: Option<Keyed<()>> = None;
        for v in &lp.vertices {
            let Some(set) = owner.get(&vertex_ix_key(*v)) else {
                common = None;
                break;
            };
            common = Some(match common {
                None => set.clone(),
                Some(held) => {
                    let mut meet = Keyed::new();
                    for (k, ()) in held.scoped() {
                        if set.contains(&k) {
                            meet.insert(&k, ());
                        }
                    }
                    meet
                }
            });
        }
        let Some(common) = common else { continue };
        let owners: Vec<Vec<u8>> = common.scoped().map(|(k, _)| k).collect();
        let [ok] = owners.as_slice() else { continue };
        let Ok(ob) = <[u8; 8]>::try_from(ok.as_slice()) else { continue };
        let ci = usize::from_le_bytes(ob);
        let Some(es) = form.edge_holonomies(&lp) else { continue };
        for (_, e) in es.scoped() {
            let hue = crate::transducer::sentence::CycleType::of(&e.gauge);
            let sigma = e.gauge.clone();
            let fill = crate::transducer::cech::classify(&e.word.rotor(), &e.carriage, hue);
            crate::transducer::family::cover::tally_loop(&fill);
            match fill {
                crate::transducer::cech::Fill::Congruent(g, ell) => {
                    let Some(class) = g.class_key() else { continue };
                    eprintln!("  simplex filled {:02x}{:02x}… class {ell}", class.as_bytes()[0], class.as_bytes()[1]);
                    form.torsion.insert(class, crate::transducer::sentence::FiledLoop { sigma, period: ell, chord: lp.chord });
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
    // THE DEMANDS, one vertex per (kind, chart) (50.5), in key order. A
    // deformation dissolves when every lamina of both bodies is carried AND
    // no loop over its pieces stands (50.8).
    let mut by_demand: Keyed<Demanded> = Keyed::new();
    let mut declare = |kind: Demand, chart: Invariant, links: Vec<Link>| {
        let key = crate::transducer::sentence::demand_key(&kind, &chart);
        match by_demand.get_mut(&key) {
            Some((_, _, held)) => held.extend(links),
            None => {
                by_demand.insert(key, (kind, chart, links));
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
    for (ak, bk) in star_and_site_faces(frame_member.as_deref(), &scenery.supports) {
        let (Some((i, cik)), Some((j, cjk))) = (read_member_key(&ak), read_member_key(&bk)) else { continue };
        let (edge, pairs) = edge_between(&bodies_at, n_reached, i, &cik, j, &cjk, &mut edges);
        let (Some(vi), Some(vj)) = (form.vertices.get(i as usize), form.vertices.get(j as usize)) else { continue };
        let (Some(ci), Some(cj)) = (vi.chart_keyed(&cik), vj.chart_keyed(&cjk)) else { continue };
        let Some(word) = form.face_word(i, j) else { continue };
        let Some(place) = form.face_place(i, &ci, j, &cj, edge) else { continue };
        let mut at = Keyed::new();
        at.insert(face_pair_key(&ci, &cj), place);
        form.faces.push(FormFace { i, j, word, at, pairs });
    }
    // THE WORDS THE FAMILY SPOKE ON OTHER FRAMES between the Frame and a
    // vertex are faces of their own, each on the frames it was spoken on
    // (21 step 2, S12.2).
    for (ix, w, fr) in section_faces {
        let Some(mut face) = form.face_carrying_with_edge(frame_ix, ix, w, None) else { continue };
        face.pairs = Some(fr);
        form.faces.push(face);
    }
    form.loops = Form::loops_of(&form.faces, nv);
    form.hue_maps = hue_maps;
    form.reindex();
    if !form.loops.is_empty() || meet_faces > 0 {
        eprintln!(
            "  family form {nv} vertices, {} faces ({meet_faces} cover meets, {} site+ring), {} loops \
             | edges asked: measured {}, unspelled {}, unread {}, not rigid {}",
            form.faces.len(),
            form.faces.len() - meet_faces,
            form.loops.len(),
            edges.measured,
            edges.unspelled,
            edges.unread,
            edges.not_rigid
        );
    }
    trace::trace_n("extract/2 faces", form.faces.len(), budget);
    Percept { form, sections, varying, unreached }
}

/// THE FRAMES A MEMBER OCCUPIES, read at its own coordinate (52.5 reading
/// 6). `bodies_at` is keyed `vertex ++ chart ++ frame`, so the frames this
/// member stands on are the fiber under its own `vertex ++ chart` prefix
/// and the remaining key IS the frame. Read in coordinate order, because
/// succession along `e₃` is the next frame OCCUPIED and the set may skip,
/// repeat or reorder in ℤ; the trie's own order is the key's bytes, which
/// for a little-endian frame is not the coordinate's.
///
/// Re-derived from the carrier that already holds the bodies, never stored
/// beside them (law 12).
pub(crate) fn occupied_frames(bodies_at: &Keyed<(Body, Body, Versor)>, ix: u16, ck: &[u8]) -> Vec<u16> {
    let mut prefix = vertex_ix_key(ix);
    crate::transducer::address::put_bytes(&mut prefix, ck);
    let Some(fiber) = bodies_at.fiber_at(&prefix) else {
        return Vec::new();
    };
    let mut out: Vec<u16> = fiber
        .filter_map(|(k, _)| {
            let tail = k.get(prefix.len()..)?;
            Some(u16::from_le_bytes(tail.try_into().ok()?))
        })
        .collect();
    out.sort_unstable();
    out
}

/// THE EDGE BETWEEN TWO MEMBERS (50.3): measured on the union of their two
/// bodies at the least frame both stand on, and the frames on which that
/// edge re-derives — the face's section. A pair with no common frame, or
/// no edge on the least one, carries none.
fn edge_between(
    bodies_at: &Keyed<(Body, Body, Versor)>,
    n_reached: usize,
    i: u16,
    cik: &[u8],
    j: u16,
    cjk: &[u8],
    tally: &mut EdgeTally,
) -> (Option<Transport>, Option<Vec<u16>>) {
    let at = |ix: u16, ck: &[u8], k: u16| -> Option<&(Body, Body, Versor)> {
        let mut bk = vertex_ix_key(ix);
        crate::transducer::address::put_bytes(&mut bk, ck);
        bk.extend_from_slice(&k.to_le_bytes());
        bodies_at.get(&bk)
    };
    let mut edge: Option<Transport> = None;
    let mut agreeing: Vec<u16> = Vec::new();
    let mut common = 0usize;
    // SUCCESSION ALONG e₃ IS THE NEXT FRAME OCCUPIED, never `k + 1` (52.5
    // reading 6, ruled 2026-09-18). This read `for k in 0..n`, probing every
    // integer up to the family's pair count and `continue`-ing on each miss:
    // a linear scan over a range, where the frames a member actually stands
    // on are named by its own fiber of `bodies_at`. The occupied set may
    // skip, repeat or reorder in ℤ, and the integers between it are not
    // coordinates of anything.
    for k in occupied_frames(bodies_at, i, cik) {
        let (Some((a_i, b_i, v_i)), Some((a_j, b_j, _))) = (at(i, cik, k), at(j, cjk, k)) else {
            continue;
        };
        common += 1;
        let measured = edge_transport(v_i, a_i, a_j, b_i, b_j);
        match &edge {
            None => {
                edge = tally.take(Some(measured));
                if edge.is_some() {
                    agreeing.push(k);
                }
            }
            Some(e) => {
                if measured.as_ref().is_ok_and(|t| same_transport(t, e)) {
                    agreeing.push(k);
                }
            }
        }
    }
    let pairs = match (&edge, common) {
        (Some(_), _) if agreeing.len() < n_reached => Some(agreeing),
        _ => None,
    };
    (edge, pairs)
}

/// Seven moves on the family's one form (chapter 21 as ruled in 52.5),
/// under the stroke (L8w, rule 10). An extraction the budget cuts off keeps
/// what it measured and says what it did not reach (rule 11): the frames it
/// never read leave as the demands of `finish`.
pub fn extract_rule_with(
    family: &[(Grid, Grid)],
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    let scale = family_ratio(family)?;
    let hit = PERCEPT_MEMO.with(|m| {
        m.borrow().as_ref().and_then(|held| {
            (held.family.as_slice() == family && held.scale == scale).then(|| held.percept.clone())
        })
    });
    let percept = match hit {
        Some(p) => p,
        None => {
            let p = family_percept(family, scale, budget);
            if budget.remaining() > 0 && p.unreached.is_empty() {
                PERCEPT_MEMO.with(|m| {
                    *m.borrow_mut() = Some(PerceptMemo { family: family.to_vec(), scale, percept: p.clone() });
                });
            }
            p
        }
    };
    let Percept { form, mut sections, varying, unreached } = percept;
    let unreached_pairs: Vec<(Grid, Grid)> = unreached.iter().filter_map(|&k| family.get(k).cloned()).collect();
    trace::trace_n("extract/1 derive_against", form.vertices.len(), budget);
    let mut form = derive_against(form, crystal, budget)?;
    trace::trace_n("extract/3 glue", form.faces.len(), budget);
    let measured = trace::carriage_of(&form);
    // 3. Glue: the placement is the vertex's own word from the Frame; the
    // crystal is asked at every chart the class acts on (21 step 3).
    let glued_classes: Vec<(TransportClass, Invariant, crate::transducer::alphabet::Word)> = form
        .vertices
        .iter()
        .filter_map(|v| {
            let tc = transport_class(v)?;
            let acts = v.acts()?;
            let placement = v.word_in(&form)?;
            Some(acts.scoped().map(|(_, a)| (tc.clone(), a.chart.clone(), placement.clone())).collect::<Vec<_>>())
        })
        .flatten()
        .collect();
    for (tc, moduli, placement) in glued_classes {
        if !budget.spend() {
            return finish(form, scale, sections, &unreached_pairs);
        }
        for (e, s, _st) in crystal.on_body(&moduli) {
            if !s.is_closed() {
                continue;
            }
            let reproduces = family.iter().all(|(x, y)| {
                crystal
                    .speak_form(&s, x, &placement)
                    .is_some_and(|sp| painted_within_grid(&sp.painted, y))
            });
            if reproduces {
                let _ = form.glue(&s, e.address(), placement.clone(), e.magnitude());
                let mut drop = Keyed::new();
                for (i, v) in form.vertices.iter().enumerate() {
                    if matches!(v, Vertex::Word { .. }) && transport_class(v).as_ref() == Some(&tc) {
                        drop.insert(vertex_ix_key(i as u16), ());
                    }
                }
                form.retract_vertices(&drop);
            }
        }
    }
    trace::trace_carriage("extract/3 glue", &measured, &form);
    trace::trace_n("extract/4 rebase", form.vertices.len(), budget);
    // 4. Rebase: w_j = t · w_i · t⁻¹ with t the relative placement, read on
    // the form's own faces. Named faces only; Frame is not a conjugator.
    let before = form.clone();
    rebase_named_faces(&mut form, std::slice::from_ref(&before));
    trace::trace("extract/5 unfold", budget);
    // 5. Unfold grow-to-frame.
    if family.iter().all(|(a, b)| grows_to_frame(a, b)) {
        form.unfold(crate::transducer::sentence::Measure::Occupied);
    }
    trace::trace_carriage("extract/4 rebase", &measured, &form);
    // 6. Contact: placement differs across frames, the ring to the Frame is
    // one value, and the word orbits (22).
    if !contact::contact_named_vertices(&mut form, family, &varying, &mut sections, budget) {
        return finish(form, scale, sections, &unreached_pairs);
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
    finish(form, scale, sections, &unreached_pairs)
}

/// Extract the invariant cochain of a family.
pub fn extract_rule(
    family: &[(Grid, Grid)],
    budget: &mut crate::physics::Budget,
) -> Option<crate::transducer::sentence::SentenceRule> {
    extract_rule_with(family, &NoCrystal, budget)
}

/// THE FORM OF ONE SCENE — a family of one pair — derived against a mind's
/// presentation under a stroke. `None` is a vertex the stroke could not
/// factor: unmeasured, never a form held with the presentation's reach
/// asserted unread (rule 11).
pub fn scene_form(
    crystal: &dyn Crystal,
    budget: &mut crate::physics::Budget,
    a: &Grid,
    b: &Grid,
) -> Option<Form> {
    let pair = [(a.clone(), b.clone())];
    extract_rule_with(&pair, crystal, budget).map(|r| r.form)
}

/// The percept memo, emptied: a test that asks whether two courts reach one
/// address with no shared runtime memory clears it between them (50.9).
#[cfg(test)]
pub(crate) fn clear_percept_memo() {
    PERCEPT_MEMO.with(|m| *m.borrow_mut() = None);
}
