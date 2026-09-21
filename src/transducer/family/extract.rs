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
use super::pair::{delta_links, Covered, Scenery};
use super::perceive::{
    at_frame_zero, demand_vertex, differing_axes, frame_of_body, perceive_at, support_at,
    word_or_demand_on_chart, Demanded, EdgeTally,
};
use super::*;
use crate::network::nerve::Keyed;
use crate::transducer::sentence::{section_of_frames, vertex_ix_key, Acting, Demand, FormFace, Transport};
use std::cell::RefCell;

type Ratio = ((usize, usize), (usize, usize));

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

/// THE FIBERS CARRIER'S OWN KEY: a frame as `i64`, little-endian, keying
/// `fibers`'s inner trie. It is NOT [`crate::transducer::sentence::frame_key`],
/// which is the frame COORDINATE a section is read at — big-endian, `u16`,
/// and in succession order. Two keys of one frame for two carriers, named
/// apart so a reader cannot take one for the other.
fn fiber_key(k: i64) -> [u8; 8] {
    k.to_le_bytes()
}

fn fiber_frame_of_key(k: &[u8]) -> Option<i64> {
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
        match frames.get_mut(&fiber_key(k)) {
            Some(v) => v.push(b.clone()),
            None => {
                frames.insert(fiber_key(k), vec![b.clone()]);
            }
        }
    }
    out
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
        for (_, (v, _versor, codomain)) in c.cover.carried.scoped() {
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
            sections.put(&coord::SectionAt::Member(mk.clone()), section_of_frames(frames));
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
    // `star_and_site_faces` offers the Frame's star, a spanning CHAIN of
    // each site's occupants and the ring pairs. Its own comment says what
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
    // A face is between two members and `star_and_site_faces` already
    // returns member pairs, so the chord is offered at the same granularity
    // the builder consumes. Both ends may sit at ONE vertex; what must
    // differ is the member key, which is the pair `(vertex, chart)`.
    type OfferedChord = (Vec<u8>, Vec<u8>, Option<crate::transducer::sentence::Transport>);

    /// THE BURGERS VECTOR BETWEEN TWO FOOTPRINTS, as a geodesic on the
    /// sheet (54.7, ruled 2026-09-21).
    ///
    /// This read `d.iter().min()` for a footprint's basepoint and defended
    /// it as "content order, never a centroid, which would be an average
    /// standing in for a place". That defence was backwards. A lexicographic
    /// minimum over `(i64, i64, i64)` is EXTRINSIC: it is decided by which
    /// axis the tuple happens to name first, so renaming the axes moves the
    /// basepoint while the scene stands still. The centre of mass is
    /// equivariant — `p(g·D) = g·p(D)` for any affine `g` — so it is the
    /// intrinsic representative and `min` was the rule 15 violation.
    ///
    /// The displacement was then `τ = b − a`, an affine difference in `ℝ³`
    /// injected as `B = Σ τᵢ (eᵢ ∧ e₋) = u ∧ e₋`. A single wedge of two
    /// 1-vectors is a simple 2-blade by construction:
    ///
    /// ```text
    ///     B ∧ B = (u ∧ e₋) ∧ (u ∧ e₋) = 0  ⟹  ⟨B²⟩₄ = 0  ⟹  I₂ ≡ 0
    /// ```
    ///
    /// so a flat translation injected as an `e₋` boost CANNOT generate a
    /// dispiration, no matter what the scene does. Both cells are now
    /// carried onto the upper sheet by the form's quantum `M(X)` and the
    /// transport is Axiom B's own transition:
    ///
    /// ```text
    ///     v_a = lamina(p(D_a), M),  v_b = lamina(p(D_b), M),  v² = −1
    ///     R   = 1 − v_b v_a,        ⟨R⟩₂ = v_a ∧ v_b
    /// ```
    ///
    /// `None` where the two footprints share a centre, since a zero
    /// displacement is not a dislocation and none is invented (rule 11).
    /// The 4 non-simple readings are therefore NOT from `τ` — they are the
    /// composition `Ω_rot · Ω_trans` in `dispiration` below, where a face
    /// word's rotation axis has a component along the boost direction, the
    /// two bivectors fail to commute, and `B_rot ∧ B_trans ≠ 0`.
    /// THE DEFECT A CHORD CARRIES — a DISPIRATION, not one of its halves.
    ///
    /// A first cut wrote `measured.or_else(transvection)`: the rotational
    /// reading OR the translational one, whichever was present. That is the
    /// projection into uncoupled sub-actions — a chord whose two sections
    /// differ in BOTH carriage and footprint has a rotation about an axis
    /// and a slip along it at once, and choosing one discards the coupling
    /// that makes it helicoidal.
    ///
    /// ```text
    ///     Ω_ab = (φ_b φ_a⁻¹) · T(τ),   τ = p(D_b) − p(D_a)
    /// ```
    ///
    /// The composite's generator is non-simple where both parts are
    /// present, so `I₂ = ⟨B∧B⟩₄ ≠ 0` and the screw survives. Where the
    /// carriages agree it degenerates to a pure Volterra dislocation —
    /// uniform Burgers vector, `I₂ = 0` honestly — and where the footprints
    /// agree it is the measured rotation alone. Each is a reading of what
    /// the two sections differ in, never a shape imposed on all of them.
    fn dispiration(
        origin: &[num_rational::BigRational; 3],
        quantum: &num_rational::BigRational,
        measured: Option<&crate::transducer::sentence::Transport>,
        da: &[(i64, i64, i64)],
        db: &[(i64, i64, i64)],
    ) -> Option<crate::transducer::sentence::Transport> {
        let slip = basepoint_transvection(origin, quantum, da, db);
        crate::transducer::family::cover::tally(|t| {
            t.chords_offered += 1;
            if slip.is_some() {
                t.tau_active += 1;
            }
        });
        match (measured, slip) {
            (None, s) => s,
            (Some(m), None) => Some(m.clone()),
            // BOTH: compose, and the product is the helicoidal transport.
            (Some(m), Some(s)) => Some(crate::transducer::sentence::Transport {
                word: m.word.clone(),
                carriage: m.carriage.geo(&s.carriage),
                gauge: m.gauge.clone(),
            }),
        }
    }

    fn basepoint_transvection(
        origin: &[num_rational::BigRational; 3],
        quantum: &num_rational::BigRational,
        da: &[(i64, i64, i64)],
        db: &[(i64, i64, i64)],
    ) -> Option<crate::transducer::sentence::Transport> {
        // THE INTRINSIC CENTRE of a support domain: its centre of mass,
        // exact over ℚ and invariant under the order the cells arrive in.
        let centre = |d: &[(i64, i64, i64)]| -> Option<[num_rational::BigRational; 3]> {
            if d.is_empty() {
                return None;
            }
            let n = num_bigint::BigInt::from(u64::try_from(d.len()).ok()?);
            let mut sum = [
                num_bigint::BigInt::from(0),
                num_bigint::BigInt::from(0),
                num_bigint::BigInt::from(0),
            ];
            for (r, c, k) in d {
                sum[0] += num_bigint::BigInt::from(*r);
                sum[1] += num_bigint::BigInt::from(*c);
                sum[2] += num_bigint::BigInt::from(*k);
            }
            Some(sum.map(|s| num_rational::BigRational::new(s, n.clone())))
        };
        let (a, b) = match (centre(da), centre(db)) {
            (Some(a), Some(b)) => (a, b),
            _ => {
                crate::transducer::family::cover::tally(|t| t.tau_unfooted += 1);
                return None;
            }
        };
        if a == b {
            // THE TWO ENDS MET. Not "collinear" — they do not lie on a
            // line — and not a coincidence, which would make a structural
            // fact sound contingent. Two footprints sharing a centre stand
            // over ONE base point, so they are two sections of one fibre
            // and their difference is VERTICAL. There is no horizontal
            // displacement for a 1-cell to carry, so none is invented
            // (rule 11); what differs between them is higher-moment shape,
            // which is a 2-cell's business.
            crate::transducer::family::cover::tally(|t| t.tau_met += 1);
            return None;
        }
        // Read from the form's own origin, so the transport does not depend
        // on where in the grid the scene happened to be drawn (rule 15).
        let lamina = |p: &[num_rational::BigRational; 3]| {
            crate::transducer::cga::hyperbolic_lamina(
                &(&p[0] - &origin[0]),
                &(&p[1] - &origin[1]),
                &(&p[2] - &origin[2]),
                quantum,
            )
        };
        let carriage = match (lamina(&a), lamina(&b)) {
            (Some(va), Some(vb)) => crate::transducer::cga::geodesic_transvection(&va, &vb),
            // Off the stereographic chart: the quantum did not cover this
            // form's extent, which is a defect in `M(X)` and not a reading.
            _ => None,
        };
        let carriage = match carriage {
            Some(c) => c,
            None => {
                crate::transducer::family::cover::tally(|t| t.tau_refused += 1);
                return None;
            }
        };
        Some(crate::transducer::sentence::Transport {
            word: crate::transducer::alphabet::Word::identity(),
            carriage,
            gauge: Vec::new(),
        })
    }
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
    let (origin, quantum) = {
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
        let mut cells = 0u64;
        let mut sum = [
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(0),
            num_bigint::BigInt::from(0),
        ];
        for ((_, _, _, da), (_, _, _, db), _) in meet_chords.iter() {
            for (r, c, k) in da.iter().chain(db.iter()) {
                sum[0] += num_bigint::BigInt::from(*r);
                sum[1] += num_bigint::BigInt::from(*c);
                sum[2] += num_bigint::BigInt::from(*k);
                cells += 1;
            }
        }
        if cells == 0 {
            ([zero.clone(), zero.clone(), zero], one)
        } else {
            let n = num_bigint::BigInt::from(cells);
            let centre = sum.map(|s| num_rational::BigRational::new(s, n.clone()));
            let mut moment = zero;
            for ((_, _, _, da), (_, _, _, db), _) in meet_chords.iter() {
                for (r, c, k) in da.iter().chain(db.iter()) {
                    let q = |v: i64| {
                        num_rational::BigRational::from_integer(num_bigint::BigInt::from(v))
                    };
                    for (axis, value) in [q(*r), q(*c), q(*k)].iter().enumerate() {
                        let d = value - &centre[axis];
                        moment += &d * &d;
                    }
                }
            }
            (centre, one + moment)
        }
    };
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
    // A STAR OR SITE PAIR BRINGS NO EDGE OF ITS OWN — `edge_between` reads
    // it off the two bodies — and a CHORD brings the one its meet measured,
    // because `edge_between` is keyed by `(vertex, chart)` and cannot
    // re-derive a transition between two sections of one member.
    for (ak, bk, chord_edge) in star_and_site_faces(frame_member.as_deref(), &scenery.supports)
        .into_iter()
        .map(|(a, b)| (a, b, None))
        .chain(chord_pairs)
    {
        drops.offered += 1;
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
        form.faces.push(FormFace { i, j, word, at, pairs });
        drops.built += 1;
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
    eprintln!(
        "  family form {nv} vertices, {} faces ({meet_faces} cover meets, {} site+ring), {} loops \
         | edges asked: measured {}, unspelled {}, unread {}, not rigid {} \
         | chords {} offered {} | cycles: reflexive {} no-tree-path {} unread {} trivial {} \
         | defects: trivial {} elliptic {} hyperbolic {} parabolic {} non-simple {} \
         | spectra: closes {} open {} undecomposed {} isoclinic {} \
         | ρ: |im| {} |conj| {} of {} generators, {} in ker | this form: b₁ {} |im| {} \
         | faces offered {} built {} dropped: member_key {} vertex {} chart {} word {} place {}",
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
        crate::transducer::sentence::Form::loop_gates().3,
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
        drops.offered,
        drops.built,
        drops.member_key,
        drops.vertex,
        drops.chart,
        drops.word,
        drops.place
    );
    trace::trace("extract/2 faces", trace::Wrote::Nothing, Some(budget));
    Percept { form, sections, varying, unreached }
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
    trace::trace("extract/1 derive_against", trace::Wrote::Nothing, Some(budget));
    let mut form = derive_against(form, crystal, budget)?;
    trace::trace("extract/3 glue", trace::Wrote::Nothing, Some(budget));
    let measured = trace::carriage_of(&form);
    // 3. Glue: the placement is the vertex's own word from the Frame; the
    // crystal is asked at every chart the class acts on (21 step 3).
    //
    // THE CLASSES BY THE CHART THEY ACT ON, in chart-key order, which is
    // content: the crystal is asked once per chart, and every class
    // standing on that chart reads the one answer.
    let mut by_chart: Keyed<(Invariant, Vec<(TransportClass, crate::transducer::alphabet::Word)>)> =
        Keyed::new();
    for v in &form.vertices {
        let (Some(tc), Some(acts), Some(placement)) = (transport_class(v), v.acts(), v.word_in(&form))
        else {
            continue;
        };
        for (ck, a) in acts.scoped() {
            match by_chart.get_mut(&ck) {
                Some((_, on)) => on.push((tc.clone(), placement.clone())),
                None => {
                    by_chart.insert(&ck, (a.chart.clone(), vec![(tc.clone(), placement.clone())]));
                }
            }
        }
    }
    // WHAT A CITED FORM SPEAKS OVER THE FAMILY is a function of that form,
    // the placement it is spoken at and the family, and of nothing else:
    // neither the class nor the chart the reading is made for enters
    // `speak_form`. So the reading is keyed by the cited form's address and
    // the placement written out (rule 6), measured once, and read back by
    // every class that asks at that form and that placement: the classes
    // are not a coordinate of the reading, so they are not a factor of it.
    let mut spoken: Keyed<bool> = Keyed::new();
    // THE RETRACTIONS OF THE WHOLE PASS, in one carrier (Z2 item 1). Each
    // citation retracted its class immediately, and `retract_vertices`
    // rebuilds the vertices, the faces, the cycle basis and every index of
    // the form — once per citation, 578 times on `1d0a4b61`. The vertices a
    // citation retracts are the same vertices whatever order the pass
    // reaches them in, so the set is the pass's and the walk is taken once
    // after it.
    let mut retract: Keyed<(), coord::VertexIx> = Keyed::new();
    for (_, (moduli, classes)) in by_chart.scoped() {
        let held = crystal.on_body(moduli);
        for (tc, placement) in classes {
            if !budget.spend() {
                return finish(form, scale, sections, &unreached_pairs);
            }
            let mut placement_key = Vec::new();
            placement.put_canonical(&mut placement_key);
            for (e, s, _st) in &held {
                if !s.is_closed() {
                    continue;
                }
                let mut key = e.address().as_bytes().to_vec();
                crate::transducer::address::put_bytes(&mut key, &placement_key);
                let reproduces = match spoken.get(&key) {
                    Some(&r) => r,
                    None => {
                        let r = family.iter().all(|(x, y)| {
                            crystal
                                .speak_form(s, x, placement)
                                .is_some_and(|sp| painted_within_grid(&sp.painted, y))
                        });
                        spoken.insert(&key, r);
                        r
                    }
                };
                if reproduces {
                    let _ = form.glue_append(s, e.address(), placement.clone(), e.magnitude());
                    // THE VERTICES OF THE CLASS, read off the transport index
                    // the form keeps current (rule 1), never a walk of every
                    // vertex asking each its class. The index is the one the
                    // form carried into the pass: an append writes no
                    // transport class, so nothing here reads a stale entry.
                    if let Some(on) = form.by_transport.at(&tc.key()) {
                        for &i in on {
                            retract.put(&vertex_ix_key(i), ());
                        }
                    }
                }
            }
        }
    }
    // ONCE, AFTER THE PASS. `retract_vertices` rebuilds the cycle basis and
    // reindexes at its end, so a pass that retracted anything needs no
    // second walk; a pass that cited without retracting still appended
    // faces and walks them here.
    if retract.is_empty() {
        form.loops = Form::loops_of(&form.faces, &form.vertices);
        form.reindex();
    } else {
        form.retract_vertices(&retract);
    }
    trace::trace("extract/3 glue walked", trace::Wrote::Nothing, Some(budget));
    trace::trace_carriage("extract/3 glue", &measured, &form);
    trace::trace("extract/4 rebase", trace::Wrote::Nothing, Some(budget));
    // 4. Rebase: w_j = t · w_i · t⁻¹ with t the relative placement, read on
    // the form's own faces. Named faces only; Frame is not a conjugator.

    rebase_named_faces(&mut form);
    trace::trace("extract/5 unfold", trace::Wrote::Nothing, Some(budget));
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
    trace::trace("extract/7 coherence", trace::Wrote::Nothing, Some(budget));
    // 7. Coherence: tree gauge, else hold the loops.
    if !form.coheres() {
        if let Some(gamma) = form.solve_coboundary() {
            let _ = form.regauge(&gamma);
        }
    }
    trace::trace("extract/8 finish", trace::Wrote::Nothing, Some(budget));
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
