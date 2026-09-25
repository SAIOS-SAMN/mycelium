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
use crate::transducer::cga;
use crate::network::nerve::coord::{ChartAt, Overlap};
use crate::transducer::sentence::{frame_key, Acting, Demand, FormFace, Producer};

pub(super) type Ratio = ((usize, usize), (usize, usize));

/// One member's frame-axis steps (MT1): its vertex, its chart, and each pair
/// of consecutive occupied frames with the motor its input body took.
type FrameSteps = (coord::VertexAt, Invariant, Vec<(u64, u64, Versor)>);

/// THE PERCEPT OF A FAMILY (52.5): the peeled form, what move 6 reads, and
/// the frames the budget did not reach. The sections along `e₃` are the
/// members' frames, read off the form ([`Form::sections`], AT1). The
/// derivation against a mind's presentation is not in it (50.2 step 2).
#[derive(Clone)]
pub(crate) struct Percept {
    pub(crate) form: Form,
    pub(crate) varying: contact::Varying,
    pub(crate) unreached: Vec<usize>,
}

/// WHAT ONE FRAME'S QUOTIENT OF ONE CHART MEASURED: the frame, the versor
/// with its hue map, the two union bodies in the frame's own coordinates,
/// and the same two bodies at their own `(r, c, k)` — the laminae the
/// fiber's union shape is built from (52.5).
struct Quotient {
    /// The frame, by its own coordinate (AT1 Q4): no width walls it.
    frame: u64,
    car: Carriage,
    a0: Body,
    b0: Body,
    a_k: Body,
    b_k: Body,
}

/// ONE MEMBER OF A CLASS'S VERTEX (D7): the chart, what it acts by there,
/// the frames its versor holds on, the bodies at each of them, the output
/// support over those frames, the union shape its fiber occupies along
/// `e₃` with the versor carrying it, and the frame-axis steps its input body
/// took between consecutive occupied frames (MT1), each the overlap the
/// atlas adds between the member's two charts (AT1 Q3).
///
/// A SECTION IS A MEMBER (AT1, ruled): a chart whose frames read another
/// versor of the class is a second section of the fibre — its own carriage,
/// its own frames — and never a face carrying a word beside its ends.
struct Member {
    chart: Invariant,
    acting: Acting,
    frames: Vec<u64>,
    at: Vec<(u64, Body, Body, Versor)>,
    support: Vec<Site>,
    union: Option<(Body, Body, Versor)>,
    steps: Vec<(u64, u64, Versor)>,
    /// The member's INPUT body at each frame it is claimed on, at that
    /// frame's own `k` — what the class's frame axis solves between when two
    /// consecutive frames of the class stand on two charts (AT1 A2).
    inputs: Vec<(u64, Body)>,
}

/// ONE STEP OF A CLASS'S FRAME AXIS ACROSS TWO CHARTS (AT1 A2): the vertex,
/// the chart and frame it leaves, the chart and frame it arrives at, and the
/// motor its input body took where `versor_between` solved — `None` where it
/// refused, the step then carrying the sections' relative `τ = C_B·C_A⁻¹`.
type CrossStep = (coord::VertexAt, Invariant, u64, Invariant, u64, Option<Versor>);

/// THE CLASS'S STEPS ACROSS TWO CHARTS, BY KIND (AT1 A2): carried by a
/// motor, carried by the sections' relative, refused where no place derives,
/// and not taken where a frame holds several charts.
#[derive(Default)]
struct AcrossCharts {
    motor: u64,
    relative: u64,
    unplaced: u64,
    several: u64,
}

impl AcrossCharts {
    fn line(&self) -> String {
        format!(
            "frame axis across charts: motor {} | sections' relative {} | no place {} | several charts at a frame {}",
            self.motor, self.relative, self.unplaced, self.several
        )
    }
}

/// MT1 (1) — THE FRAME-AXIS TRANSITIONS OF ONE MEMBER: a new solve by the
/// existing reader between the member's body at consecutive OCCUPIED frames,
/// on the inputs and on the outputs, two readings of one step typed by how
/// they agree, never averaged. Each solve is one transition under the stroke,
/// read by its action, with the frame its chain carries per lane. Each step
/// is also the overlap the atlas would add between the member's two charts,
/// `class ‖ Invariant ‖ k` and `… ‖ k′`, counted with its ruled key's bytes
/// (AT1 r5). THE STEPS ARE RETURNED — each pair of frames with the motor its
/// INPUT body took, where it solved — and the atlas files each as an overlap
/// between the member's two charts carrying that motor, producer `motor`
/// (AT1 Q3). A step whose input did not solve carries no transition and
/// makes no overlap (rule 11).
fn read_frame_axis(
    claimed: &mut [&Quotient],
    (class, chart): (&[u8], &[u8]),
    budget: &mut crate::physics::Budget,
) -> Vec<(u64, u64, Versor)> {
    let mut out = Vec::new();
    use cga::{note_motor, note_step, note_unread, StepClass, TransitionKind::FrameAxis};
    // Succession along e₃ is the frame coordinate's own order (51.1); the
    // fibre's keys are little-endian and do not carry it.
    #[expect(clippy::disallowed_methods, reason = "rule 4 — a sort by a canonical key: the frame, the corpus's e₃ coordinate, whose succession IS the frame-axis transition")]
    claimed.sort_by_key(|q| q.frame);
    let mut solve = |a: &Body, b: &Body| -> Option<Option<Versor>> {
        if !budget.spend() {
            note_unread(FrameAxis);
            return None;
        }
        let m = blade::versor_between(&a.shape, &b.shape);
        if let Some(m) = &m {
            let lanes = element::measure_lanes(&super::perceive::chain_of(Some(a), Some(b), m));
            note_motor(FrameAxis, m, Some(lanes));
        }
        Some(m)
    };
    for step in claimed.windows(2) {
        let [q, next] = step else { continue };
        let mut key = Vec::new();
        for k in [q.frame, next.frame] {
            crate::transducer::sentence::form::atlas::chart_coordinate(class, chart, k, &mut key);
        }
        crate::transducer::sentence::form::atlas::note_frame_axis_step(key.len());
        // BOTH SIDES ARE ASKED, and a side the stroke did not reach is
        // counted unread under its kind (A.17): nothing is dropped by a
        // return, so every later step of the member is counted too. The step
        // itself stands unclassed.
        let (input, output) = (solve(&q.a_k, &next.a_k), solve(&q.b_k, &next.b_k));
        let (Some(input), Some(output)) = (input, output) else { continue };
        // The planes turned: the solved side's, and on a DISAGREEMENT both
        // motors' — the disagreement is the content.
        let (class, turned): (StepClass, Vec<&Versor>) = match (&input, &output) {
            (Some(i), Some(o)) if cga::projective_ratio(i, o).is_some() => (StepClass::Agree, vec![i]),
            (Some(i), Some(o)) => (StepClass::Disagree, vec![i, o]),
            (Some(m), None) | (None, Some(m)) => (StepClass::OneSided, vec![m]),
            (None, None) => (StepClass::Neither, Vec::new()),
        };
        note_step(class, &turned);
        if let Some(m) = &input {
            out.push((q.frame, next.frame, m.clone()));
        }
    }
    out
}

/// ONE MEMBER FROM THE QUOTIENTS IT IS CLAIMED ON: its frames, the bodies at
/// each, the output support over them, the union shape of its fibre (52.5) —
/// the laminae gathered at their own frames, so a frame not claimed is not in
/// it. Its frame-axis steps are the CHART's, read once over every frame the
/// class holds on it ([`read_frame_axis`] at the caller), not the member's.
fn member_of(chart: &Invariant, acting: Acting, claimed: &mut [&Quotient], union_by: &Versor) -> Member {
    let mut frames: Vec<u64> = Vec::new();
    let mut at: Vec<(u64, Body, Body, Versor)> = Vec::new();
    let mut support: Vec<Site> = Vec::new();
    let mut union_in: Vec<Lamina> = Vec::new();
    let mut union_out: Vec<Lamina> = Vec::new();
    let mut inputs: Vec<(u64, Body)> = Vec::new();
    for q in claimed.iter() {
        frames.push(q.frame);
        inputs.push((q.frame, q.a_k.clone()));
        at.push((q.frame, q.a0.clone(), q.b0.clone(), q.car.v.clone()));
        if let Ok(k) = i64::try_from(q.frame) {
            support.extend(support_at(&q.b0.laminae, k));
        }
        union_in.extend_from_slice(&q.a_k.laminae);
        union_out.extend_from_slice(&q.b_k.laminae);
    }
    support.sort_unstable();
    support.dedup();
    frames.sort_unstable();
    frames.dedup();
    let union = union_quotient(union_in, union_out, union_by);
    Member { chart: chart.clone(), acting, frames, at, support, union, steps: Vec::new(), inputs }
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
    let entry = budget.remaining();
    trace::trace("extract/0 frames", trace::Wrote::Nothing, Some(budget));
    // OR1 — THE ORBITS THIS PERCEPT READS are kept for its cycles line. A
    // percept that returns before the line leaves the list to the next arm,
    // which replaces it: at most one percept's orbits are ever held.
    crate::transducer::sentence::arm_orbits();
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
        for l in &b.laminae {
            out_at_site.insert(l.site.key(), bk.clone());
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
            // The fiber's bodies were heard when they were perceived, and a
            // union of heard sections at their `n` is heard; a union that is
            // not is no quotient to read (38.2, BQ4).
            let (Some(a_k), Some(b_k), Some(a0), Some(b0)) = (
                body_of(laminae_of(ai)),
                body_of(laminae_of(bo)),
                body_of(at_frame_zero(&laminae_of(ai))),
                body_of(at_frame_zero(&laminae_of(bo))),
            ) else {
                continue;
            };
            match carriage(&a0, &b0) {
                Some(car) => {
                    // MT1 — THE IN→OUT TRANSITION READ BY ITS ACTION, the
                    // whole matrix once under the stroke, with the frame it
                    // carries per lane.
                    match budget.spend() {
                        true => {
                            let lanes = element::measure_lanes(&super::perceive::chain_of(Some(&a0), Some(&b0), &car.v));
                            cga::note_motor(cga::TransitionKind::InOut, &car.v, Some(lanes));
                        }
                        false => cga::note_unread(cga::TransitionKind::InOut),
                    }
                    if let Some(qc) = usize::try_from(k).ok().and_then(|k| quotient_charts.get_mut(k)) {
                        qc.insert(&ck, ());
                    }
                    for b in bo {
                        out_reached.insert(body_key(b), ());
                    }
                    // The frame by its own coordinate (AT1 Q4); a frame
                    // before the family's first is no frame of it.
                    let Ok(frame) = u64::try_from(k) else { continue };
                    quotients.push(Quotient { frame, car, a0, b0, a_k, b_k });
                }
                // 52.5 item 3, 50.8: no quotient at this frame. The cover
                // reads what the pieces carry, and what it leaves is the
                // demand; a cover the stroke cut short leaves the demand as
                // typed (rule 11).
                None => match cover_deformation(&a0, &b0, Some(scale), budget) {
                    Ok(cover) => {
                        for b in bo {
                            if at_frame_zero(&b.laminae).iter().all(|l| cover.reached.contains(&l.site.key())) {
                                out_reached.insert(body_key(b), ());
                            }
                        }
                        for (_, d) in cover.refused.scoped() {
                            demanded.push(d.clone());
                        }
                        let mut demands = unreached_demands(ai, &a0, &b0, &cover.reached, c, ai.len());
                        let first = demands.split_off(1);
                        demanded.extend(first);
                        let [demand] = demands.as_slice() else { continue };
                        covers.push(Covered { demand: demand.clone(), cover, frame: k });
                    }
                    Err(_) => {
                        demanded.extend(unreached_demands(ai, &a0, &b0, &Keyed::new(), c, ai.len()));
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
            let mut claimed: Vec<&Quotient> = Vec::new();
            for q in rep {
                if let Some(hk) = crate::transducer::sentence::hue_section_key_of(&rotor, &q.car.sigma) {
                    hue_maps.insert(hk, q.car.sigma.clone());
                }
                if !crate::transducer::sentence::gauge_agrees(&q.car.sigma, &acting.gauge) {
                    varying.place_varies.put(&tc.key(), ());
                    continue;
                }
                claimed.push(q);
            }
            let class_key = tc.key().encode();
            let chart_key = c.key();
            let coords = (class_key.as_bytes(), chart_key.as_bytes());
            // EVERY FRAME THE CLASS HOLDS ON THIS CHART, whichever section
            // stands there: the frame-axis steps are read over all of them
            // (AT1 Q3 — consecutive frames of one vertex's chart are two
            // charts of it, `class ‖ Invariant ‖ k` and `… ‖ k′`, whichever
            // member each frame is), never within one section only, which
            // left the step between two sections' frames without an overlap.
            let mut on_chart: Vec<&Quotient> = claimed.clone();
            let mut members = vec![member_of(c, acting, &mut claimed, &first.car.v)];
            // THE OTHER VERSORS THE CHART READ ON OTHER FRAMES: the placement
            // varies (move 6's reading), and a word that differs is ANOTHER
            // SECTION OF THE FIBRE (21 step 2, S12.2; AT1, ruled) — a member
            // of this class with its own carriage and its own frames, never a
            // face carrying a word beside its ends.
            for (vk, group) in by_versor.scoped() {
                if vk == rep_key {
                    continue;
                }
                varying.place_varies.put(&tc.key(), ());
                let [g, ..] = group.as_slice() else { continue };
                let Some(Ok(w)) = peel_versor(&g.car.v) else { continue };
                if w.rotor().projective_eq(&rotor) {
                    continue;
                }
                // THE SECTION'S CARRIAGE IS RELATIVE TO THE VERTEX'S ACTION.
                // A member speaks `carriage · A`, `A` the vertex rotor's base
                // action (`conformal_of_rotor`), so the member whose frames
                // read `V_s` carries `V_s · Ã` — the conjugator and placement
                // taking the class's action to this section's — and never
                // `V_s` itself, which would speak `V_s · A`. The action's
                // reverse is its inverse as a ray (78.32(1)).
                let Some(action) = element::conformal_of_rotor(&rotor, &element::identity_versor()) else { continue };
                if !cga::is_versor(&action) {
                    continue;
                }
                let section = Acting {
                    chart: c.clone(),
                    carriage: Box::new(g.car.v.geo(&action.reverse())),
                    gauge: g.car.sigma.clone(),
                    frames: Keyed::new(),
                };
                let mut group: Vec<&Quotient> = group.clone();
                on_chart.extend(group.iter().copied());
                members.push(member_of(c, section, &mut group, &g.car.v));
            }
            // The chart's frame-axis steps (MT1), read once, filed with the
            // class's first member: the overlaps they make name the charts by
            // their frames, and the frames name the member standing there.
            let steps = read_frame_axis(&mut on_chart, coords, budget);
            if let [first, ..] = members.as_mut_slice() {
                first.steps = steps;
            }
            match reads.at_mut(&tc.key()) {
                Some(read) => {
                    // ONE VERTEX PER TRANSPORT CLASS, its address the least of
                    // its members' (content order, law 2), its derivation
                    // that member's.
                    if address < &read.address {
                        read.address = address.clone();
                        read.derivation = derivation.clone();
                    }
                    read.members.extend(members);
                }
                None => {
                    reads.put(&tc.key(), ClassRead { address: address.clone(), derivation: derivation.clone(), members });
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
                for l in &body.laminae {
                    if let Some(bk) = out_at_site.get(&l.site.key()) {
                        meets.insert(bk, ());
                    }
                }
                let mut any = false;
                for (bk, ()) in meets.scoped() {
                    let Some(image) = out_bodies.get(&bk) else { continue };
                    any = true;
                    imaged.insert(&bk, ());
                    // Both were heard as perceived; moved to frame zero they
                    // are the same bodies (38.2, BQ4 for any other reading).
                    let (Some(a0), Some(b0)) =
                        (body_of(at_frame_zero(&body.laminae)), body_of(at_frame_zero(&image.laminae)))
                    else {
                        continue;
                    };
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
                let links: Vec<Link> = body
                    .laminae
                    .iter()
                    .filter_map(|l| on_grid(&l.site).map(|(r, c)| (r, c, 0u8, l.hue)))
                    .collect();
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
    let frame_at = coord::VertexAt::Frame;
    let _ = scenery.place(frame_vertex(), Vec::new());
    // THE BODIES EACH MEMBER STANDS ON AT EACH FRAME, keyed member ‖ frame:
    // the member by its vertex's coordinate and its chart, the frame by its
    // own spelling (AT1).
    let mut bodies_at: Keyed<(Body, Body, Versor)> = Keyed::new();
    // THE UNION SHAPE OF EACH MEMBER'S FIBER, keyed as its support is —
    // `member_key(vertex, chart)` — and holding no frame, because the union
    // is what the member occupies ACROSS its frames and not a reading at any
    // one of them. `bodies_at` keys the same prefix with the frame after it,
    // so a union filed there would answer `occupied_frames` with a frame
    // nothing stands on.
    let mut unions: Keyed<(Body, Body, Versor)> = Keyed::new();
    // THE FRAME-AXIS STEPS OF EACH MEMBER (MT1), kept until the vertices
    // stand, then filed as overlaps between the member's charts (AT1 Q3).
    let mut frame_steps: Vec<FrameSteps> = Vec::new();
    // AT1 A2 — the class's steps across two charts, and their count by kind.
    let mut cross_steps: Vec<CrossStep> = Vec::new();
    let mut across_charts = AcrossCharts::default();
    // 53.6 — A COVER'S CARRIED PIECES ARE GROUPED BY TRANSPORT CLASS,
    // exactly as a chart's quotients are: ONE Word vertex per class present
    // in the cover, one member per chart the class carries a piece on.
    //
    // Each piece was its own vertex. That is what made a form of thousands:
    // `484b58aa` read 2 055 vertices, 11 167 faces and 3 986 orbits, and
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
        let Ok(frame) = u64::try_from(c.frame) else { continue };
        for (_, (v, _versor, sup)) in c.cover.carried.scoped() {
            let codomain = &sup.codomain;
            let (Some(chart), Some(tc)) = (v.face_chart(), transport_class(v)) else { continue };
            let Some(acting) = v.acting_on(&chart) else { continue };
            let Vertex::Word { address, derivation, .. } = v else { continue };
            let support: Vec<Site> = codomain
                .iter()
                .map(|s| match s.with_axis(crate::transducer::base::FRAME_AXIS, c.frame) {
                    Some(at) => at,
                    None => s.clone(),
                })
                .collect();
            // One class, one chart, two carriages: the multiplicity.
            // The class opens a member coordinate — class ‖ chart — and the
            // transport coordinate is left behind where the extension begins.
            let mut mk = tc.key().encode().as_bytes().to_vec();
            crate::transducer::address::put_key_bytes(&mut mk, chart.key().as_bytes());
            let mut carriage = Vec::new();
            acting.carriage.put_canonical(&mut carriage);
            match piece_seen.get(&mk) {
                Some(held) if held != &carriage => {
                    let links: Vec<Link> = support
                        .iter()
                        .filter_map(|s| on_grid(s).map(|(r, cc)| (r, cc, 0u8, 0u8)))
                        .collect();
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
                steps: Vec::new(),
                // A cover's piece is read at its cover's one frame, with no
                // body of its own at that frame's `k`: it takes no step.
                inputs: Vec::new(),
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

    for (_, read) in reads.entries() {
        // EACH MEMBER HOLDS ITS FRAMES (AT1, ruled): the member's charts are
        // its chart at each of them. Two members of one section key — one
        // action placed twice, at two frames — are one member standing on
        // the union of their frames.
        let mut acts: Keyed<Acting, coord::Section> = Keyed::new();
        for m in &read.members {
            let key = m.acting.key();
            let mut acting = match acts.drop_at(&key) {
                Some(held) => held,
                None => m.acting.clone(),
            };
            for &k in &m.frames {
                acting.frames.insert(frame_key(k), ());
            }
            acts.put(&key, acting);
        }
        // THE VERTEX'S ADDRESS IS DERIVED FROM WHAT IT HOLDS (54.2): its
        // derivation — the least member's — and every member, as
        // `from_address` derives it; the least member's own address chose
        // the derivation and is not the vertex's.
        let address = crate::transducer::sentence::word_vertex_address(&read.derivation, &acts, None);
        let vertex = Vertex::Word { address, derivation: read.derivation.clone(), acts, contact: None };
        let Some(at) = scenery.place(vertex, Vec::new()) else { continue };
        for m in &read.members {
            let ck = m.chart.key();
            // TWO SECTIONS OF ONE CHART ARE TWO MEMBERS AT ONE MEMBER KEY
            // (AT1): the key is `(vertex, chart)` and the frames tell them
            // apart. Their supports are one occupancy, the union of both; the
            // union shape is the first section's — the class's
            // representative, read first — and a later section's does not
            // replace it.
            let mk = member_key(&at, ck.as_bytes());
            match scenery.supports.get_mut(&mk) {
                Some(held) => {
                    held.extend(m.support.iter().cloned());
                    held.sort_unstable();
                    held.dedup();
                }
                None => {
                    scenery.supports.insert(&mk, m.support.clone());
                }
            }
            if let (Some((ua, ub, uv)), false) = (&m.union, unions.contains(&mk)) {
                unions.insert(&mk, (ua.clone(), ub.clone(), uv.clone()));
            }
            for (k, a0, b0, v) in &m.at {
                // The member opens a body coordinate — member ‖ frame — and
                // is left behind where the frame begins.
                let mut bk = member_key(&at, ck.as_bytes());
                bk.extend_from_slice(&frame_key(*k));
                bodies_at.insert(bk, (a0.clone(), b0.clone(), v.clone()));
            }
            if !m.steps.is_empty() {
                frame_steps.push((at.clone(), m.chart.clone(), m.steps.clone()));
            }
        }
        // AT1 A2 — THE CLASS'S FRAME AXIS ACROSS ITS CHARTS. The steps above
        // join a chart to itself; a body that CHANGES CHART between two
        // consecutive frames of its class — one vertex standing on chart A at
        // `k` and on chart B at `k′` — was joined by nothing, and the cycles
        // through it were lost (the audit of 8309b6e1). The class's occupied
        // frames in their own order, each with the charts the class stands on
        // there; a step where each of two consecutive frames holds ONE chart
        // and the two differ. A frame holding several charts pairs with no
        // one of them over another (rule 15: no representative): counted.
        let mut succession: Keyed<Vec<(Invariant, Body)>> = Keyed::new();
        for m in &read.members {
            for (k, body) in &m.inputs {
                let fk = frame_key(*k);
                match succession.get_mut(&fk) {
                    Some(on) => {
                        if !on.iter().any(|(c, _)| c == &m.chart) {
                            on.push((m.chart.clone(), body.clone()));
                        }
                    }
                    None => {
                        succession.insert(&fk, vec![(m.chart.clone(), body.clone())]);
                    }
                }
            }
        }
        let frames: Vec<(u64, Vec<(Invariant, Body)>)> = succession
            .scoped()
            .filter_map(|(fk, on)| Some((crate::transducer::sentence::frame_of_key(&fk)?, on.clone())))
            .collect();
        for pair in frames.windows(2) {
            let [(k, here), (k2, there)] = pair else { continue };
            match (here.as_slice(), there.as_slice()) {
                ([(a, body_a)], [(b, body_b)]) if a != b => {
                    // BOTH READINGS ARE MADE by the type of the answer: the
                    // input body's motor where one carries it, else the
                    // sections' relative, which `face_place` re-derives at
                    // each end's own frame (54.5 grade 0's vertical
                    // transition).
                    let motor = match budget.spend() {
                        true => blade::versor_between(&body_a.shape, &body_b.shape),
                        false => {
                            cga::note_unread(cga::TransitionKind::FrameAxis);
                            continue;
                        }
                    };
                    cross_steps.push((at.clone(), a.clone(), *k, b.clone(), *k2, motor));
                }
                ([_], [_]) => {}
                _ => across_charts.several += 1,
            }
        }
    }
    let mut form = Form { vertices: scenery.vertices.clone(), ..Form::default() };
    trace::trace("percept/3 meets", trace::Wrote::Nothing, Some(budget));
    // 53.6 items 3 and 4, in their own organ: the pieces' form is built,
    // its orbits are classified, and it is discarded. What comes back is
    // what the SCENE keeps — the torsion each orbit filed, awaiting a chord
    // this form can name, and the classes standing per cover.
    let mut edges = EdgeTally::default();
    let (meet_chords, meet_faces, pending_torsion, stood) =
        crate::transducer::family::covered::classify_cover(&covers, &mut edges);
    trace::trace("percept/4 orbits+classify", trace::Wrote::Nothing, Some(budget));
    // THE DEMANDS, one vertex per (kind, chart) (50.5), in key order. A
    // deformation dissolves when every lamina of both bodies is carried AND
    // no orbit over its pieces stands (50.8).
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
        let _ = scenery.place(demand_vertex(kind.clone(), chart, links), Vec::new());
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
        .at(&frame_at)
        .and_then(|v| v.face_chart())
        .map(|c| member_key(&frame_at, c.key().as_bytes()));
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
    // identically. `0 orbits` everywhere, and with no cycle there is no
    // held obstruction, so `pivot_orbit` had nothing to discharge:
    // `pivoted [0,0,0,0]` on all five voices at epoch 94, `stood` and
    // `closed` likewise, and `court basis 0` on all three courts. The residual
    // probe worked because it asks whether the genome already holds a word
    // that reproduces the demand, which needs no cycle; everything above
    // grade 0 consumes H₁ and H₁ was empty.
    //
    // The chords are the meets lifted to classes by `transport_class`, the
    // same lift the orbits' ENDS already used one screen down. They are
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
    //
    // AT1, ruled (a): each end is a PIECE-CHART of that member at the
    // cover's frame — the piece by its nucleus and shape — so a chord is an
    // overlap between two piece-charts, offered to the same builder.
    type OfferedChord = (ChartAt, ChartAt, Option<crate::transducer::sentence::Transport>);

    // AXIOM B's CHART FOR THIS FORM — one origin and one quantum for every
    // chord it carries (`chord::form_chart`, where the construction is read).
    let (origin, quantum) = super::chord::form_chart(&meet_chords);
    let (source, image) = cover_nerves(&covers);
    let spectrum = phase_spectrum(&meet_chords);
    let chord_pairs: Vec<OfferedChord> = meet_chords
        .iter()
        .filter_map(|((ta, ca, _, da), (tb, cb, _, db), e, frame)| {
            let at = |t: &TransportClass, c: &ChartAt| -> Option<ChartAt> {
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
                //
                // AT1, ruled (a): the footprint is the piece's fixed-size
                // coordinates on the chart, read off the cover's piece-chart
                // and re-read here at the scene's vertex of the same class.
                let v = match form.by_transport.at(&t.key())?.as_slice() {
                    [v, ..] => v.clone(),
                    [] => return None,
                };
                let ChartAt::At(m) = c else { return None };
                let piece = m.piece.clone()?;
                Some(ChartAt::piece(v, m.chart.clone(), *frame, piece))
            };
            let (ma, mb) = (at(ta, ca)?, at(tb, cb)?);
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
        hows: Vec<Incidence>,
        contact: Option<super::pairing::ContactPair>,
    }
    let mut offers: Vec<Offer> = Vec::new();
    // NB1 — the faces and the contact complex's certified cells, one read of
    // the occupancy; the cells ride the form in memory (78.6, choice (c)).
    let (named, cells) = named_incidence(frame_member.as_deref(), &scenery.supports);
    form.cells = cells;
    for nf in named {
        let (a, b, hows) = (nf.a, nf.b, nf.named.hows);
        match nf.named.interfaces.as_slice() {
            [] => offers.push(Offer { a, b, hows, contact: None }),
            each => offers.extend(each.iter().map(|c| Offer {
                a: a.clone(),
                b: b.clone(),
                hows: hows.clone(),
                contact: Some(c.clone()),
            })),
        }
    }
    // THE CHORDS, each one overlap between two piece-charts at the cover's
    // frame (AT1, ruled (a)), subject to the same place reading as every
    // face and counted beside the others (§8.1).
    for (from, to, edge) in chord_pairs {
        drops.offered += 1;
        let Some(place) = form.face_place(&from, &to, edge) else {
            drops.place += 1;
            continue;
        };
        form.put_face(Overlap { from, to }, FormFace::placed(place, Producer::Place));
        drops.built += 1;
    }
    // CV1b F3 — the demands the contacts' oblique poses leave, stood below.
    let mut pole_demanded: Vec<Demanded> = Vec::new();
    // 78.46(5)(d) — the named pairs no frame places, each with its content.
    let mut frameless: Vec<super::incidence::Frameless> = Vec::new();
    for Offer { a: ak, b: bk, hows: how, contact: contacted } in offers {
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
            edge_between(&bodies_at, &unions, n_reached, (&i, &cik), (&j, &cjk), &mut edges);
        // A CONTACT carries its refraction and nothing
        // else — the rigidity of the two bodies' union is not what their
        // contact is (BQ5a), so where the refraction was refused the face
        // stands with no edge and the contact line names why. Every other
        // face reads its edge off the bodies.
        // A contact face's body pair, in hand, the body at its `i` end first,
        // with the two nuclei whose tether the contact carries (78.18(c)).
        let body_pair = contacted.as_ref().map(|p| match p.left.contains(&ak) {
            true => (p.bodies.clone(), p.nuclei.clone()),
            false => ((p.bodies.1.clone(), p.bodies.0.clone()), (p.nuclei.1.clone(), p.nuclei.0.clone())),
        });
        // BQ5b — the interface, whole; an unread WHAT still a face, counted.
        let (interface, unnucleated) =
            super::pairing::interface_of(contacted.as_ref(), body_pair.as_ref().map(|(_, n)| n));
        drops.unnucleated += usize::from(unnucleated);
        let (edge, refraction) = match contact {
            true => {
                let (edge, refraction, demands) = super::pairing::members_contact_edge(
                    &unions,
                    body_pair.as_ref().and_then(|(_, (x, y))| Some((x.as_ref()?, y.as_ref()?))),
                    member_key(&i, &cik),
                    member_key(&j, &cjk),
                );
                pole_demanded.extend(demands);
                (edge, refraction)
            }
            false => (measured, None),
        };
        let (Some(vi), Some(vj)) = (form.vertices.at(&i), form.vertices.at(&j)) else {
            drops.vertex += 1;
            continue;
        };
        let (Some(ci), Some(cj)) = (vi.chart_keyed(&cik), vj.chart_keyed(&cjk)) else {
            drops.chart += 1;
            continue;
        };
        if form.coboundary(&i, &j).is_none() {
            drops.word += 1;
            continue;
        }
        // ONE OVERLAP PER FRAME THE PAIR STANDS ON TOGETHER (AT1, Q2): the
        // frames both members hold, or a star member's own where one end is
        // the Frame — whose chart is its tag alone and stands first — and,
        // where the edge re-derived on some of them only, those (52.5: the
        // face's section along `e₃`).
        let star_i = matches!(i, coord::VertexAt::Frame);
        let star_j = matches!(j, coord::VertexAt::Frame);
        let frames: Vec<u64> = match (star_i, star_j) {
            (true, _) => form.member_frames(&j, &cj),
            (_, true) => form.member_frames(&i, &ci),
            _ => {
                let there = crate::transducer::sentence::section_of_frames(form.member_frames(&j, &cj));
                form.member_frames(&i, &ci).into_iter().filter(|k| there.contains(&frame_key(*k))).collect()
            }
        };
        let common = !frames.is_empty();
        let frames: Vec<u64> = match &pairs {
            Some(agreeing) => frames.into_iter().filter(|k| agreeing.contains(&frame_key(*k))).collect(),
            None => frames,
        };
        if frames.is_empty() {
            // A REFUSAL THAT CARRIES ITS CONTENT (78.46(5)(d)): counted, and
            // held with the pair, both members' frames, and why.
            drops.frameless += 1;
            frameless.push(super::incidence::Frameless {
                a: (i.clone(), ci.clone(), form.member_frames(&i, &ci)),
                b: (j.clone(), cj.clone(), form.member_frames(&j, &cj)),
                why: match common {
                    true => super::incidence::FramelessWhy::EdgeOnNone,
                    false => super::incidence::FramelessWhy::NoCommonFrame,
                },
            });
            continue;
        }
        let mut stood = false;
        for k in frames {
            let chart = |v: &coord::VertexAt, c: &Invariant| match v {
                coord::VertexAt::Frame => ChartAt::Frame,
                _ => ChartAt::at(v.clone(), c.clone(), k),
            };
            let o = match star_j {
                true => Overlap { from: ChartAt::Frame, to: chart(&i, &ci) },
                false => Overlap { from: chart(&i, &ci), to: chart(&j, &cj) },
            };
            let Some(mut place) = form.face_place(&o.from, &o.to, edge.clone()) else {
                drops.place += 1;
                continue;
            };
            place.refraction = refraction.clone();
            // BD2 — and the pair on the form's side carrier, by the overlap.
            if let Some((pair, _)) = &body_pair {
                form.face_bodies.put(&o, pair.clone());
            }
            let mut face = FormFace::placed(place, Producer::Place);
            face.interface = interface.clone();
            form.put_face(o, face);
            stood = true;
        }
        if stood {
            drops.built += 1;
            if contact {
                drops.contact_built += 1;
            }
        }
    }
    // THE FRAME-AXIS OVERLAPS (AT1 Q3, MT1): each step a member's input body
    // took between two consecutive occupied frames is an overlap between the
    // member's two charts, carrying the motor it measured, producer `motor`.
    // Its edge is the motor read as a transport where it spells one; a motor
    // that does not is an overlap with no edge, counted as any edge is.
    for (at, chart, steps) in frame_steps {
        for (k, k2, motor) in steps {
            let o = Overlap { from: ChartAt::at(at.clone(), chart.clone(), k), to: ChartAt::at(at.clone(), chart.clone(), k2) };
            let edge = edges.take(Some(super::perceive::edge_of(&motor).map_err(|e| vec![e])));
            let Some(mut place) = form.face_place(&o.from, &o.to, edge) else { continue };
            place.carriage = motor;
            form.put_face(o, FormFace::placed(place, Producer::Motor));
        }
    }
    // AT1 A2 — AND ACROSS TWO CHARTS: the motor where the input body's
    // solved (producer Motor, its edge the motor read as a transport), the
    // sections' relative `τ = C_B·C_A⁻¹` where it refused — `face_place`
    // at each end's own frame, producer Place. Each counted by kind.
    for (at, a, k, b, k2, motor) in cross_steps {
        let o = Overlap { from: ChartAt::at(at.clone(), a, k), to: ChartAt::at(at, b, k2) };
        match motor {
            Some(motor) => {
                let edge = edges.take(Some(super::perceive::edge_of(&motor).map_err(|e| vec![e])));
                let Some(mut place) = form.face_place(&o.from, &o.to, edge) else {
                    across_charts.unplaced += 1;
                    continue;
                };
                place.carriage = motor;
                form.put_face(o, FormFace::placed(place, Producer::Motor));
                across_charts.motor += 1;
            }
            None => {
                let Some(place) = form.face_place(&o.from, &o.to, None) else {
                    across_charts.unplaced += 1;
                    continue;
                };
                form.put_face(o, FormFace::placed(place, Producer::Place));
                across_charts.relative += 1;
            }
        }
    }
    // CV1b F3 — the pole's demands stand once each, by their coordinates.
    super::perceive::stand_demands(&mut form.vertices, pole_demanded);
    let cycles = Form::cycles_of(&form.faces, &form.vertices);
    form.orbits = cycles.kept.clone();
    form.hue_maps = hue_maps;
    form.reindex();
    // 78.46(4′) — the certified cells read by their circuits' pair
    // composites, the vacuum circuits among them (79.3).
    let _ = form.read_agreements(&cycles, true);
    // 53.6 item 4 — THE COVER'S FILED ORBITS TAKE A CHORD OF THE SCENE.
    // Each was classified inside the cover; its chord there indexed the
    // pieces' own form. Here it takes the face between the two CLASS
    // vertices its ends belong to, which is a coordinate this form has. A
    // orbit whose classes the scene relates by no face is NOT filed: there
    // is no chord to name and none is invented (rule 11).
    let mut standing_demands: Vec<(Demand, Invariant)> = Vec::new();
    for (class, (sigma, periods), ends, rotor) in pending_torsion {
        let ats: Vec<coord::VertexAt> = ends
            .iter()
            .filter_map(|tc| form.by_transport.at(&tc.key()))
            // A class stands at one or more vertices; the orbit's end is
            // the class, so any vertex of it names the same class and the
            // slice pattern reads one rather than choosing among them.
            .filter_map(|on| match on.as_slice() {
                [v, ..] => Some(v.clone()),
                [] => None,
            })
            .collect();
        // THE CHORD IS AN OVERLAP BETWEEN THE TWO CLASS VERTICES (AT1): the
        // first the fibres read, in key order, which is content (law 2).
        let chord = match ats.as_slice() {
            [a, b, ..] => match form.overlaps_between_vertices(a, b).as_slice() {
                [o, ..] => Some(o.clone()),
                [] => None,
            },
            _ => None,
        };
        match chord {
            Some(chord) => {
                form.torsion.put(&class, crate::transducer::sentence::FiledOrbit {
                    sigma,
                    periods,
                    chord: crate::transducer::sentence::Closes::On(chord),
                });
            }
            // A MEASURED CLASS IS NEVER DISCARDED (law 7, rule 11). The
            // orbit was classified inside the cover and its class is a
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
                        [v, ..] => form.vertices.at(v),
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
        // A standing demand stands at its own coordinate; two orbits
        // standing the same `(kind, chart)` are one demand, counted twice.
        let _ = form.put_vertex(demand_vertex(kind, &chart, Vec::new()));
    }
    form.reindex();
    // READ WHETHER OR NOT ANYTHING CLOSED. It was gated on
    // `!form.orbits.is_empty() || meet_faces > 0`, so the one row that says
    // why a form is empty was printed only when the form was not empty:
    // fifteen forms crystallized at `orbits 0` and this line never fired
    // once. A reading suppressed in the failure case is not a reading.
    let unread_why = crate::transducer::sentence::Form::unread_gates();
    // NB1 — THE CONTACT COMPLEX'S CERTIFIED CELLS BY GRADE, off the form's
    // own carrier by the nerve's one reader (law 12).
    let contact_cells_by_grade =
        crate::transducer::cech::cells_by_grade(form.cells.scoped().map(|(_, c)| (c.grade(), c.meet())));
    // BQ5a's guard (77, 78.4 E1): the circuits that cross a contact face, by
    // arm.
    let contact_circuits = super::pairing::contact_circuits(&form, budget);
    // BQ1, audit N-3 — WHAT PERCEPTION PASSED OVER, always printed: bodies
    // `body_of` refused (counted at `body_of`, whoever asked), laminae with
    // no section (counted at `on_grid`), and sections filed at their `n`
    // alone (counted at `section_moduli`). Zero while intake reads the
    // grid's three.
    eprintln!(
        "  perceive unheard {}: components whose moduli are not read at their n | sectionless {} | coarse sections {}",
        crate::transducer::family::perceive::take_unheard(),
        crate::transducer::family::perceive::take_sectionless(),
        crate::transducer::blade::take_coarse(),
    );
    eprintln!(
        "  family form {nv} vertices, {} faces ({meet_faces} cover meets, {} site+ring), {} orbits \
         | edges asked: measured {}, unspelled {}, unread {}, not rigid {} \
         | chords {} offered {} | cycles: reflexive {} no-tree-path {} unread {} \
         (cone {} not rigid {} no face {}; edge refused: cone {} not rigid {} unrefracted {}) \
         trivial {} (vacuum {} resonant {} over {} charged edges; \
         μ read on {} steps, {} of them non-zero) \
         | defects: trivial {} elliptic {} hyperbolic {} parabolic {} non-simple {} \
         | spectra: closes {} open {} undecomposed {} isoclinic {} \
         | ρ: |im| {} |conj| {} of {} generators, {} in ker | this form: b₁ {} |im| {} \
         | contacts: {} \
         | planes: {} \
         | mint: {} \
         | metabolism: {} syntropy {} \
         | nerve/source: {} charts over {} cells, {} shared by two or more, \
         {} \
         | contact complex: certified cells (grade cells meet≠0) {:?} \
         | nerve/image: {} charts over {} cells, {} shared, {} \
         | phase: {} finite {} infinite {} unread, periods {:?} \
         | faces offered {} built {} dropped: member_key {} vertex {} chart {} word {} place {} frameless {} \
         | contact faces offered {} built {} unnucleated {} | contact circuits: curved {} (gauge static {}) \
         coset closed {} modulo-held {} open {} unread {}",
        form.faces.len(),
        // The site+ring faces COUNTED as they were built, not the total
        // less the meets: a subtraction floored at zero invents a value
        // where the two carriers disagree (rule 11), and `drops.built` is
        // the measurement this label was reaching for.
        drops.built,
        form.orbits.len(),
        edges.measured,
        edges.unspelled,
        edges.unread,
        edges.not_rigid,
        meet_chords.len(),
        n_chords,
        crate::transducer::sentence::Form::orbit_gates().0,
        crate::transducer::sentence::Form::orbit_gates().1,
        crate::transducer::sentence::Form::orbit_gates().2,
        // H2c — the unread gate by what refused it, and the read cycles
        // whose edge half was refused.
        unread_why.0.cone,
        unread_why.0.not_rigid,
        unread_why.0.no_face,
        unread_why.1.cone,
        unread_why.1.not_rigid,
        unread_why.1.unrefracted,
        crate::transducer::sentence::Form::orbit_gates().3,
        // HOW THE CLOSURES DIVIDE. `trivial` above counts every cycle whose
        // boundary product is the identity; these two say whether that
        // identity was a cancellation or an absence (54.7).
        crate::transducer::sentence::orbit_resonance().0,
        crate::transducer::sentence::orbit_resonance().1,
        crate::transducer::sentence::orbit_resonance().2.len(),
        crate::transducer::sentence::orbit_resonance().3,
        crate::transducer::sentence::orbit_resonance().4,
        crate::transducer::sentence::orbit_defects().0,
        crate::transducer::sentence::orbit_defects().1,
        crate::transducer::sentence::orbit_defects().2,
        crate::transducer::sentence::orbit_defects().3,
        crate::transducer::sentence::orbit_defects().4,
        crate::transducer::sentence::orbit_spectra().0,
        crate::transducer::sentence::orbit_spectra().1,
        crate::transducer::sentence::orbit_spectra().2,
        crate::transducer::sentence::orbit_spectra().3,
        crate::transducer::sentence::orbit_synonyms().0,
        crate::transducer::sentence::orbit_synonyms().1,
        crate::transducer::sentence::orbit_synonyms().2,
        crate::transducer::sentence::orbit_synonyms().3,
        form.orbits.len(),
        crate::transducer::sentence::form_operators(&form),
        // THE CONTACTS, ON THE FLEET'S OWN LINE (66, BQ5a): what each
        // contact the topology named carried, and why an edge did not form.
        // Read WITHOUT clearing — the corpus test takes the total, this
        // prints the running state, and only one reader may take (law 12).
        crate::transducer::family::pairing::contact_tally().line(),
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
        // per-lane draw. SYNTROPY IS READ FROM THE CONTACTS' COUNTER, not
        // duplicated: condensations are already counted there, and a
        // second counter for one fact is law 12 inside one carrier.
        crate::physics::metabolism::metabolism().line(),
        crate::transducer::family::pairing::contact_tally().condensed,
        source.charts,
        source.cells,
        source.shared,
        source.cells_line(),
        contact_cells_by_grade,
        image.charts,
        image.cells,
        image.shared,
        image.cells_line(),
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
        drops.frameless,
        drops.contact_offered,
        drops.contact_built,
        drops.unnucleated,
        contact_circuits[0],
        contact_circuits[1],
        contact_circuits[2],
        contact_circuits[3],
        contact_circuits[4],
        contact_circuits[5]
    );
    // OR1, 73.1 — EACH ORBIT THIS PERCEPT READ, beside the cycles line: its
    // type, its class, the planes its precession turns in, the precession's
    // canonical coordinates, and its centre read or refused by name — the
    // modes under this stroke's budget (BD1), and the circuits' with them.
    // Then its cells, circuits and bends, each held typed (78.16).
    // 78.46(5)(d) — EACH FRAMELESS PAIR, its content on its own line; the
    // count is the family form line's `frameless`.
    for refused in &frameless {
        eprintln!("  {}", refused.line());
    }
    // AT1 A2 — the class's steps across two charts, by kind.
    eprintln!("  {}", across_charts.line());
    super::comprehend::read_and_hold(&form, budget, (entry - budget.remaining()) as u64, unreached.len() as u64);
    // FR2, 72.1c(3) — AND WHERE EACH CYCLE STANDS ALONG `e₃`: its span, the
    // meet of its members' sections, read here from the members' frames and
    // never stored beside them.
    eprintln!("  {}", super::span::SpanTally::of(&form).line());
    // MT1 — EVERY OVERLAP'S CARRIAGE READ BY ITS ACTION, under the stroke;
    // then the motor line, read without clearing (the corpus test takes it).
    for (_, face) in form.faces.entries() {
        match budget.spend() {
            true => cga::note_motor(cga::TransitionKind::Face, &face.place.carriage, None),
            false => cga::note_unread(cga::TransitionKind::Face),
        }
    }
    eprintln!("  running {}", cga::motor_tally().line());
    // AT1's READING (r1–r6): this form read as the table it is, and the
    // searches and frame-axis steps counted so far (running).
    let reached: Vec<u64> = (0..n).filter(|k| !unreached.contains(k)).filter_map(|k| u64::try_from(k).ok()).collect();
    let atlas = super::atlas::read_form_atlas(&form, &reached);
    crate::transducer::sentence::form::atlas::note_form(atlas);
    eprintln!(
        "  atlas: {} | running {}",
        atlas.line(),
        crate::transducer::sentence::form::atlas::atlas_tally().searches_line()
    );
    trace::trace("extract/2 faces", trace::Wrote::Nothing, Some(budget));
    Percept { form, varying, unreached }
}
