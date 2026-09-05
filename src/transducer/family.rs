// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! FAMILY TRANSPORT — a family of transitions comprehended as a connection.
//!
//! A training pair is two perceived scenes. Each output body is the image
//! of an input body under a transport: the versor solved from the two
//! bodies' moments and the gauge map read cell by cell. The family is
//! comprehended when, class by class (bodies of the same moduli), the
//! transports agree in at least one reading — the scene's own coordinates,
//! the coordinates about the scene bodies' centroids, or the coordinates
//! about the bodies' own centroids — with one gauge map. Every reading
//! that agrees is kept; none is preferred. Bodies created with no source
//! must be the same body in every pair; bodies removed must be removed in
//! every pair. What does not agree is the residual: the exact cells of the
//! observed output the connection does not speak. Nothing here searches:
//! every transport is a quotient, every agreement an equality in the
//! algebra. Where several input bodies of one class could be the source of
//! an output body, they are interchangeable under the class's own symmetry;
//! the nearest by centroid is the representative taken, and the replay on
//! every pair verifies the choice.
//!
//! Speaking a test input applies the connection: each body of the input is
//! carried by its class's transport read back out of every agreeing
//! reading, the readings must render the same cells, and the output is
//! rendered by carrying every output cell's center back into the body —
//! the serialization of the transported bodies onto the sensor's frame, the
//! one place the lattice is allowed.

use super::blade::{self, Shape};
use super::cga;
use super::element::{self, Versor};
use super::stalk::Invariant;
use num_bigint::BigInt;
use num_rational::BigRational;

pub type Grid = Vec<Vec<u8>>;
type Cell3 = (i64, i64, u8);

fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}
fn qn(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// A perceived body: its cells with hue, its shape, its moduli, and its
/// centroid in the scene's vertex coordinates.
#[derive(Clone, Debug)]
pub struct Body {
    pub cells: Vec<Cell3>,
    pub shape: Shape,
    pub moduli: Invariant,
    pub centroid: (BigRational, BigRational),
}

fn body_of(comp: Vec<Cell3>) -> Body {
    let mut pos: Vec<(i64, i64)> = comp.iter().map(|&(r, c, _)| (r, c)).collect();
    pos.sort_unstable();
    let shape = blade::shape_of(&pos);
    let moduli = Invariant(blade::moduli(&shape.blade));
    let centroid = shape.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2))).unwrap_or((q(0), q(0)));
    let mut cells = comp;
    cells.sort_unstable();
    Body { cells, shape, moduli, centroid }
}

/// Perceive a scene: its bodies (connected components of the non-background
/// cells; hue 0 is the background the eye does not see).
pub fn perceive(grid: &Grid) -> Vec<Body> {
    let mut cells: Vec<Cell3> = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            if h != 0 {
                cells.push((r as i64, c as i64, h));
            }
        }
    }
    blade::components(&cells).into_iter().map(body_of).collect()
}

fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), g.first().map_or(0, |r| r.len()))
}

/// The scene as a body: the whole sensor extent lifted, so its centroid is
/// a moment of a body, not a coordinate of a rectangle.
fn scene_center(f: (usize, usize)) -> (BigRational, BigRational) {
    let cells: Vec<Cell3> = (0..f.0 as i64).flat_map(|r| (0..f.1 as i64).map(move |c| (r, c, 1u8))).collect();
    if cells.is_empty() {
        return (q(0), q(0));
    }
    body_of(cells).centroid
}

/// The family's frame ratio ((h_num, h_den), (w_num, w_den)), identical
/// across pairs or refused.
pub fn family_ratio(family: &[(Grid, Grid)]) -> Option<((usize, usize), (usize, usize))> {
    let mut ratio = None;
    for (a, b) in family {
        let (fa, fb) = (frame_of(a), frame_of(b));
        if fa.0 == 0 || fa.1 == 0 || fb.0 == 0 || fb.1 == 0 {
            return None;
        }
        let (gh, gw) = (gcd(fb.0, fa.0).max(1), gcd(fb.1, fa.1).max(1));
        let r = ((fb.0 / gh, fa.0 / gh), (fb.1 / gw, fa.1 / gw));
        match &ratio {
            None => ratio = Some(r),
            Some(held) if *held != r => return None,
            _ => {}
        }
    }
    ratio
}

/// One body carried onto another: the transport and the gauge map.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub v: Versor,
    pub sigma: Vec<(u8, u8)>,
}

/// Carry `a` onto `b`: the versor from the moments, and the hue map read
/// cell by cell — functional, or refused.
pub fn carriage(a: &Body, b: &Body) -> Option<Carriage> {
    let v = blade::versor_between(&a.shape, &b.shape)?;
    let mut sigma: Vec<(u8, u8)> = Vec::new();
    for &(r, c, h) in &a.cells {
        let p = element::carry_center(&v, r, c)?;
        let (tr, tc) = element::cell_of(&p);
        let h2 = b.cells.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))).ok().map(|i| b.cells[i].2)?;
        match sigma.iter().find(|(f, _)| *f == h) {
            Some((_, t)) => {
                if *t != h2 {
                    return None;
                }
            }
            None => sigma.push((h, h2)),
        }
    }
    sigma.sort_unstable();
    Some(Carriage { v, sigma })
}

/// Merge two gauge maps: consistent on their shared hues, or refused.
fn merge_sigma(a: &[(u8, u8)], b: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut out = a.to_vec();
    for &(f, t) in b {
        match out.iter().find(|(x, _)| *x == f) {
            Some((_, held)) => {
                if *held != t {
                    return None;
                }
            }
            None => out.push((f, t)),
        }
    }
    out.sort_unstable();
    Some(out)
}

/// The readings a transport is taken in, by the point it is read about:
/// the scene's origin, the scene bodies' centroids, the bodies' own
/// centroids (with the centroid displacement carried alongside).
pub const READINGS: usize = 5;

/// The anchors of a pair in one reading: the scene's origin (0); the
/// scene bodies' centroids (1); the bodies' own centroids (2); mixed —
/// from the scene body to the body (3), from the body to the scene body
/// (4). The mixed readings are the compositions: a body gathered to the
/// scene's center, or released from it.
fn anchors(
    reading: usize,
    a: &Body,
    b: &Body,
    sa: &(BigRational, BigRational),
    sb: &(BigRational, BigRational),
) -> Option<((BigRational, BigRational), (BigRational, BigRational))> {
    match reading {
        0 => None,
        1 => Some((sa.clone(), sb.clone())),
        2 => Some((a.centroid.clone(), b.centroid.clone())),
        3 => Some((sa.clone(), b.centroid.clone())),
        _ => Some((a.centroid.clone(), sb.clone())),
    }
}

fn read_in(v: &Versor, pa: &(BigRational, BigRational), pb: &(BigRational, BigRational)) -> Versor {
    cga::translator(&-&pb.0, &-&pb.1).geo(v).geo(&cga::translator(&pa.0, &pa.1))
}

fn read_out(u: &Versor, px: &(BigRational, BigRational), py: &(BigRational, BigRational)) -> Versor {
    cga::translator(&py.0, &py.1).geo(u).geo(&cga::translator(&-&px.0, &-&px.1))
}

/// A class rule: bodies of these moduli are carried by the transport in
/// every reading that agreed across the family (reading index, versor in
/// that reading), with the centroid displacement `shift` for the body
/// reading, and recoloured by `sigma`.
#[derive(Clone, Debug)]
pub struct Rule {
    pub moduli: Invariant,
    pub readings: Vec<(usize, Versor)>,
    /// The family's anchor reading — a coordinate of the rule, not the first entry.
    pub anchor: usize,
    pub shift: (BigRational, BigRational),
    pub sigma: Vec<(u8, u8)>,
}

/// A body created with no source, the same in every pair: its cells in
/// the scene's coordinates (reading 0) or as doubled offsets from the
/// scene body's centroid (reading 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Creation {
    pub reading: usize,
    pub cells: Vec<Cell3>,
}

/// The comprehended connection of a family.
#[derive(Clone, Debug)]
pub struct Transport {
    pub ratio: ((usize, usize), (usize, usize)),
    pub rules: Vec<Rule>,
    pub removed: Vec<Invariant>,
    pub created: Vec<Creation>,
}

impl Transport {
    /// The class rule for these moduli. Rules are sorted by moduli at
    /// comprehend; one read, not a scan of the family.
    pub fn rule(&self, moduli: &Invariant) -> Option<&Rule> {
        self.rules.binary_search_by(|r| r.moduli.0.cmp(&moduli.0)).ok().map(|i| &self.rules[i])
    }
}

/// What comprehending a family produced: the connection when every pair
/// replays, and the residual — the observed output cells the connection
/// does not speak, as a chain (row, col, spoken hue, observed hue) on the
/// first pair that fails.
#[derive(Clone, Debug, Default)]
pub struct Comprehended {
    pub transport: Option<Transport>,
    pub residual: Vec<(i64, i64, u8, u8)>,
    /// Whether the pairs share one frame ratio. A family whose frames
    /// disagree has no single connection to read; its pairs are spoken as
    /// percepts carrying their frames, for the court to relate.
    pub framed: bool,
    /// The family's sentence: one germ per carried body, faces on meeting
    /// supports. Absent when every body merged into a single class rule.
    pub sentence: Option<super::sentence::Nerve>,
    /// Cross-pair invariant cochain, when every pair's nerve agrees up to gauge.
    pub rule: Option<super::sentence::SentenceRule>,
}

struct Pairing {
    pair: usize,
    input: usize,
    output: usize,
    carriage: Carriage,
}

struct Scene {
    bodies_in: Vec<Body>,
    bodies_out: Vec<Body>,
    frame_in: (usize, usize),
    frame_out: (usize, usize),
    center_in: (BigRational, BigRational),
    center_out: (BigRational, BigRational),
}

/// COMPREHEND a family: pair every output body with its source, read each
/// class's transport in every reading the pairs agree in, settle creations
/// and removals, then replay the connection on every pair. The residual of
/// the first pair that does not replay is the family's demand.
pub fn comprehend(family: &[(Grid, Grid)]) -> Comprehended {
    let mut out = Comprehended::default();
    let Some(ratio) = family_ratio(family) else { return out };
    out.framed = true;
    let scenes: Vec<Scene> = family
        .iter()
        .map(|(a, b)| {
            let (frame_in, frame_out) = (frame_of(a), frame_of(b));
            Scene {
                bodies_in: perceive(a),
                bodies_out: perceive(b),
                frame_in,
                frame_out,
                center_in: scene_center(frame_in),
                center_out: scene_center(frame_out),
            }
        })
        .collect();
    // 1. PAIR each output body with its source: the input body of the same
    // moduli that a carriage exists for. The class is the pairing and the
    // carriage is the versor; several such bodies are interchangeable under
    // the class's symmetry, so the first in the support's own order is the
    // representative taken, and the replay below verifies it. No distance
    // is measured to choose.
    let mut pairings: Vec<Pairing> = Vec::new();
    let mut created_by_pair: Vec<Vec<Vec<Cell3>>> = vec![Vec::new(); family.len()];
    let mut unmatched_inputs: Vec<(usize, usize)> = Vec::new();
    for (p, s) in scenes.iter().enumerate() {
        let mut used = vec![false; s.bodies_in.len()];
        let mut used_out = vec![false; s.bodies_out.len()];
        for (j, ob) in s.bodies_out.iter().enumerate() {
            let best: Option<(usize, Carriage)> = s
                .bodies_in
                .iter()
                .enumerate()
                .filter(|(i, ab)| !used[*i] && ab.moduli == ob.moduli)
                .find_map(|(i, ab)| carriage(ab, ob).map(|car| (i, car)));
            if let Some((i, car)) = best {
                used[i] = true;
                used_out[j] = true;
                pairings.push(Pairing { pair: p, input: i, output: j, carriage: car });
            }
        }
        // Leftover: ∃V even when the chart axes disagree.
        for (j, ob) in s.bodies_out.iter().enumerate() {
            if used_out[j] {
                continue;
            }
            let best: Option<(usize, Carriage)> = s
                .bodies_in
                .iter()
                .enumerate()
                .filter(|(i, _)| !used[*i])
                .find_map(|(i, ab)| carriage(ab, ob).map(|car| (i, car)));
            match best {
                Some((i, car)) => {
                    used[i] = true;
                    used_out[j] = true;
                    pairings.push(Pairing { pair: p, input: i, output: j, carriage: car });
                }
                None => created_by_pair[p].push(ob.cells.clone()),
            }
        }
        for (i, u) in used.iter().enumerate() {
            if !u {
                unmatched_inputs.push((p, i));
            }
        }
    }
    // 2. RULES: per class, every reading the pairings agree in.
    let mut classes: Vec<Invariant> = Vec::new();
    for pr in &pairings {
        let m = &scenes[pr.pair].bodies_in[pr.input].moduli;
        if !classes.contains(m) {
            classes.push(m.clone());
        }
    }
    let mut rules: Vec<Rule> = Vec::new();
    for class in &classes {
        let members: Vec<&Pairing> =
            pairings.iter().filter(|pr| scenes[pr.pair].bodies_in[pr.input].moduli == *class).collect();
        let mut readings: Vec<(usize, Versor)> = Vec::new();
        let mut sigma: Option<Vec<(u8, u8)>> = None;
        let mut shift: Option<(BigRational, BigRational)> = None;
        // The gauge map and the displacement are read once for the class.
        let mut gauge_ok = true;
        for pr in &members {
            let s = &scenes[pr.pair];
            let (a, b) = (&s.bodies_in[pr.input], &s.bodies_out[pr.output]);
            let d = (&b.centroid.0 - &a.centroid.0, &b.centroid.1 - &a.centroid.1);
            sigma = match &sigma {
                None => Some(pr.carriage.sigma.clone()),
                Some(held) => match merge_sigma(held, &pr.carriage.sigma) {
                    Some(m) => Some(m),
                    None => {
                        gauge_ok = false;
                        break;
                    }
                },
            };
            if shift.is_none() {
                shift = Some(d);
            }
        }
        if !gauge_ok {
            continue;
        }
        for reading in 0..READINGS {
            let mut agreed: Option<Versor> = None;
            let mut ok = true;
            for pr in &members {
                let s = &scenes[pr.pair];
                let (a, b) = (&s.bodies_in[pr.input], &s.bodies_out[pr.output]);
                let u = match anchors(reading, a, b, &s.center_in, &s.center_out) {
                    None => pr.carriage.v.clone(),
                    Some((pa, pb)) => read_in(&pr.carriage.v, &pa, &pb),
                };
                let d = (&b.centroid.0 - &a.centroid.0, &b.centroid.1 - &a.centroid.1);
                let same_shift = !matches!(reading, 2 | 3) || shift.as_ref() == Some(&d);
                match &agreed {
                    None => agreed = Some(u),
                    Some(held) => {
                        if !(element::same_versor(held, &u) && same_shift) {
                            ok = false;
                            break;
                        }
                    }
                }
            }
            if let (true, Some(u)) = (ok, agreed) {
                readings.push((reading, u));
            }
        }
        if !readings.is_empty() {
            let anchor = readings.first().map(|(r, _)| *r).unwrap_or(0);
            rules.push(Rule {
                moduli: class.clone(),
                readings,
                anchor,
                shift: shift.unwrap_or((q(0), q(0))),
                sigma: sigma.unwrap_or_default(),
            });
        }
    }
    // 3. REMOVALS: a class never carried, unmatched wherever it appears.
    let mut removed: Vec<Invariant> = Vec::new();
    for (p, i) in &unmatched_inputs {
        let m = &scenes[*p].bodies_in[*i].moduli;
        if !removed.contains(m) && !classes.contains(m) {
            removed.push(m.clone());
        }
    }
    // 4. CREATIONS: the same created bodies in every pair, in the scene's
    // coordinates or about the scene body's centroid.
    let mut creations: Vec<Creation> = Vec::new();
    let about_center = |cells: &[Cell3], center: &(BigRational, BigRational)| -> Vec<Cell3> {
        let (cr, cc) = (center.0.clone() * q(2), center.1.clone() * q(2));
        cells
            .iter()
            .map(|&(r, c, h)| {
                let dr = (q(2 * r) - &cr).to_integer();
                let dc = (q(2 * c) - &cc).to_integer();
                (dr.try_into().unwrap_or(i64::MAX), dc.try_into().unwrap_or(i64::MAX), h)
            })
            .collect()
    };
    if let Some(first) = created_by_pair.first() {
        for cells in first {
            let absolute_everywhere = created_by_pair.iter().all(|cp| cp.iter().any(|c| c == cells));
            let centered = about_center(cells, &scenes[0].center_out);
            let centered_everywhere = created_by_pair
                .iter()
                .enumerate()
                .all(|(p, cp)| cp.iter().any(|c| about_center(c, &scenes[p].center_out) == centered));
            match (absolute_everywhere, centered_everywhere) {
                (true, _) => creations.push(Creation { reading: 0, cells: cells.clone() }),
                (false, true) => creations.push(Creation { reading: 1, cells: centered }),
                _ => {}
            }
        }
    }
    // One rule per class, keyed by moduli: speak is a read, not a scan.
    rules.sort_by(|a, b| a.moduli.0.cmp(&b.moduli.0));
    let transport = Transport { ratio, rules, removed, created: creations };
    // 5. REPLAY the connection on every pair. The classes that agree are a
    // connection in their own right: a body of a class no rule reaches is
    // the REMAINDER of the family, not a refutation of it. The connection
    // is refuted only where a cell it does speak disagrees with the
    // observation; where it speaks nothing, the unreached body's cells are
    // the residual carried alongside the partial transport.
    let mut remainder: Vec<(i64, i64, u8, u8)> = Vec::new();
    for (a, b) in family {
        let Some(s) = speak_total(&transport, a) else {
            out.residual = residual_of(None, b);
            return out;
        };
        let grid = s.grid();
        let unreached: Vec<(i64, i64)> = s.remainder.iter().map(|&(r, c, _)| (r, c)).collect();
        let diff = residual_of(Some(&grid), b);
        let (unspoken, refuted): (Vec<_>, Vec<_>) = diff.into_iter().partition(|(r, c, _, _)| unreached.contains(&(*r, *c)));
        if !refuted.is_empty() {
            out.residual = refuted;
            return out;
        }
        remainder.extend(unspoken);
    }
    remainder.sort_unstable();
    remainder.dedup();
    out.residual = remainder;
    out.transport = Some(transport);
    out.rule = extract_rule(family);
    out.sentence = out.rule.as_ref().map(|r| r.nerve.clone());
    out
}

fn germ_of_body(
    source: Option<&Body>,
    target: Option<&Body>,
    v: Versor,
    word: super::alphabet::Word,
    kind: super::peel::ShellKind,
) -> super::sentence::Germ {
    let mut chain = Vec::new();
    match (source, target) {
        (Some(a), Some(b)) => {
            for &(r, c, h) in &a.cells {
                let h2 = element::carry_center(&v, r, c)
                    .and_then(|p| {
                        let (tr, tc) = element::cell_of(&p);
                        b.cells.binary_search_by(|x| (x.0, x.1).cmp(&(tr, tc))).ok().map(|i| b.cells[i].2)
                    })
                    .unwrap_or(h);
                chain.push((r, c, h, h2));
            }
        }
        (None, Some(b)) => {
            for &(r, c, h) in &b.cells {
                chain.push((r, c, 0, h));
            }
        }
        (Some(a), None) => {
            for &(r, c, h) in &a.cells {
                chain.push((r, c, h, 0));
            }
        }
        (None, None) => {}
    }
    chain.sort_unstable();
    let addr = element::from_residual(&chain).address();
    super::sentence::Germ {
        address: addr,
        versor: v,
        word,
        chain,
        source: source.map(|b| b.moduli.clone()),
        target: target.map(|b| b.moduli.clone()),
        kind,
        shift: None,
    }
}

fn word_of_versor(v: &Versor) -> super::alphabet::Word {
    element::affine_of(v)
        .and_then(|aff| {
            (aff.len() >= 2 && aff[0].len() >= 2).then(|| {
                [[aff[0][0].clone(), aff[0][1].clone()], [aff[1][0].clone(), aff[1][1].clone()]]
            })
        })
        .and_then(|block| super::alphabet::spell(&block).map(|s| s.word))
        .unwrap_or_else(super::alphabet::Word::identity)
}

fn frame_chart(f: (usize, usize)) -> Invariant {
    Invariant(vec![u64::MAX, f.0 as u64, f.1 as u64])
}

fn frame_border(f: (usize, usize)) -> Vec<(i64, i64, u8, u8)> {
    let (h, w) = (f.0 as i64, f.1 as i64);
    if h <= 0 || w <= 0 {
        return Vec::new();
    }
    let mut chain = Vec::new();
    for r in 0..h {
        chain.push((r, 0, 0, 0));
        if w > 1 {
            chain.push((r, w - 1, 0, 0));
        }
    }
    for c in 0..w {
        chain.push((0, c, 0, 0));
        if h > 1 {
            chain.push((h - 1, c, 0, 0));
        }
    }
    chain.sort_unstable();
    chain.dedup();
    chain
}

fn frame_germ(a: &Grid, b: &Grid) -> super::sentence::Germ {
    let (fa, fb) = (frame_of(a), frame_of(b));
    let chain = frame_border(fb);
    let addr = element::from_residual(&chain).address();
    super::sentence::Germ {
        address: addr,
        versor: element::identity_versor(),
        word: super::alphabet::Word::identity(),
        chain,
        source: Some(frame_chart(fa)),
        target: Some(frame_chart(fb)),
        kind: super::peel::ShellKind::Frame,
        shift: None,
    }
}

struct Cand {
    body: Body,
    kind: super::peel::ShellKind,
    parent: usize,
}

fn scene_cands(grid: &Grid, scale: Option<((usize, usize), (usize, usize))>) -> Vec<Cand> {
    let mut out = Vec::new();
    // index: bodies of this scene; shells are peel fibre, not extra vertices
    for (p, b) in perceive(grid).into_iter().enumerate() {
        out.push(Cand { body: b.clone(), kind: super::peel::ShellKind::Whole, parent: p });
        for shell in super::peel::peel_body_scaled(&b.cells, scale) {
            if shell.kind == super::peel::ShellKind::Whole || shell.cells.is_empty() || shell.cells == b.cells {
                continue;
            }
            out.push(Cand { body: body_of(shell.cells), kind: shell.kind, parent: p });
        }
    }
    out
}

fn centroid_dist2(a: &Body, b: &Body) -> BigRational {
    let dr = &a.centroid.0 - &b.centroid.0;
    let dc = &a.centroid.1 - &b.centroid.1;
    &dr * &dr + &dc * &dc
}

fn pair_rank(ic: &Cand, oc: &Cand) -> Option<(u8, Carriage)> {
    if oc.kind != super::peel::ShellKind::Whole {
        return None;
    }
    // Match is ∃V ∈ Spin(4,1)(ℚ) carrying the shape: carriage / versor_between.
    // The linear block need not spell — a parabolic translation is a versor.
    let car = carriage(&ic.body, &oc.body)?;
    let exact = ic.body.moduli == oc.body.moduli;
    let r = match (exact, ic.kind == super::peel::ShellKind::Whole) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };
    Some((r, car))
}

fn mark_parent(used: &mut [bool], cands: &[Cand], idx: usize) {
    let p = cands[idx].parent;
    for (k, c) in cands.iter().enumerate() {
        if c.parent == p {
            used[k] = true;
        }
    }
}

/// Partition candidates by exact chart axes (`Invariant` equality), never
/// a digest of those axes. A hash of moduli isolates congruent bodies
/// whose packing or orientation differs; the axes themselves are the chart.
fn group_by_chart(cands: &[Cand], used: Option<&[bool]>, peel: bool) -> Vec<(Invariant, Vec<usize>)> {
    let mut groups: Vec<(Invariant, Vec<usize>)> = Vec::new();
    for (i, c) in cands.iter().enumerate() {
        if used.is_some_and(|u| u[i]) {
            continue;
        }
        let inv = if peel { c.body.moduli.peel() } else { c.body.moduli.clone() };
        match groups.iter_mut().find(|(m, _)| *m == inv) {
            Some((_, v)) => v.push(i),
            None => groups.push((inv, vec![i])),
        }
    }
    groups
}

fn greedy_within(
    ins: &[Cand],
    outs: &[Cand],
    ii: &[usize],
    oo: &[usize],
    used_in: &mut [bool],
    used_out: &mut [bool],
    pairs: &mut Vec<(usize, usize, Carriage)>,
    want: u8,
) {
    for &i in ii {
        if used_in[i] {
            continue;
        }
        let mut best: Option<(usize, BigRational, Carriage)> = None;
        for &j in oo {
            if used_out[j] {
                continue;
            }
            let Some((r, car)) = pair_rank(&ins[i], &outs[j]) else {
                continue;
            };
            if r != want {
                continue;
            }
            let d = centroid_dist2(&ins[i].body, &outs[j].body);
            match &best {
                None => best = Some((j, d, car)),
                Some((_, bd, _)) if d < *bd => best = Some((j, d, car)),
                _ => {}
            }
        }
        if let Some((j, _, car)) = best {
            mark_parent(used_in, ins, i);
            mark_parent(used_out, outs, j);
            pairs.push((i, j, car));
        }
    }
}

fn pair_groups(
    ins: &[Cand],
    outs: &[Cand],
    in_g: &[(Invariant, Vec<usize>)],
    out_g: &[(Invariant, Vec<usize>)],
    used_in: &mut [bool],
    used_out: &mut [bool],
    pairs: &mut Vec<(usize, usize, Carriage)>,
    want: u8,
) {
    for (inv, ii) in in_g {
        let Some((_, oo)) = out_g.iter().find(|(m, _)| m == inv) else {
            continue;
        };
        greedy_within(ins, outs, ii, oo, used_in, used_out, pairs, want);
    }
}

fn pair_nerve(a: &Grid, b: &Grid, scale: Option<((usize, usize), (usize, usize))>) -> super::sentence::Nerve {
    use super::sentence::{assemble, close};
    let ins = scene_cands(a, scale);
    let outs = scene_cands(b, scale);
    let mut used_in = vec![false; ins.len()];
    let mut used_out = vec![false; outs.len()];
    let mut pairs: Vec<(usize, usize, Carriage)> = Vec::new();
    let in_exact = group_by_chart(&ins, None, false);
    let out_exact = group_by_chart(&outs, None, false);
    for want in 0u8..=1 {
        pair_groups(&ins, &outs, &in_exact, &out_exact, &mut used_in, &mut used_out, &mut pairs, want);
    }
    let in_peel = group_by_chart(&ins, Some(&used_in), true);
    let out_peel = group_by_chart(&outs, Some(&used_out), true);
    for want in 2u8..=3 {
        pair_groups(&ins, &outs, &in_peel, &out_peel, &mut used_in, &mut used_out, &mut pairs, want);
    }
    // Leftover wholes: ∃V even when the chart axes disagree. Not a
    // cartesian of peel shells — unused wholes only.
    let ii: Vec<usize> = ins
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_in[*i] && c.kind == super::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    let oo: Vec<usize> = outs
        .iter()
        .enumerate()
        .filter(|(i, c)| !used_out[*i] && c.kind == super::peel::ShellKind::Whole)
        .map(|(i, _)| i)
        .collect();
    for want in 2u8..=3 {
        greedy_within(&ins, &outs, &ii, &oo, &mut used_in, &mut used_out, &mut pairs, want);
    }
    let mut germs = vec![frame_germ(a, b)];
    for (i, j, car) in pairs {
        let kind = ins[i].kind;
        let word = word_of_versor(&car.v);
        germs.push(germ_of_body(
            Some(&ins[i].body),
            Some(&outs[j].body),
            car.v,
            word,
            kind,
        ));
    }
    // unmatched shells are not germs; unmatched wholes are created or erased
    for (j, oc) in outs.iter().enumerate() {
        if !used_out[j] && oc.kind == super::peel::ShellKind::Whole {
            germs.push(germ_of_body(
                None,
                Some(&oc.body),
                element::identity_versor(),
                super::alphabet::Word::identity(),
                super::peel::ShellKind::Whole,
            ));
        }
    }
    for (i, ic) in ins.iter().enumerate() {
        if !used_in[i] && ic.kind == super::peel::ShellKind::Whole {
            germs.push(germ_of_body(
                Some(&ic.body),
                None,
                element::identity_versor(),
                super::alphabet::Word::identity(),
                super::peel::ShellKind::Whole,
            ));
        }
    }
    let mut n = assemble(germs, Vec::new());
    let _ = close(&mut n);
    n
}

/// Re-derive the family's rule and check it agrees with `rule`.
pub fn rule_replays_family(rule: &super::sentence::SentenceRule, family: &[(Grid, Grid)]) -> bool {
    let Some(again) = extract_rule(family) else {
        return false;
    };
    again.charts == rule.charts && again.faces == rule.faces && again.holonomies == rule.holonomies
}

/// Extract the invariant cochain of a family: one nerve per pair, then
/// agree_up_to_gauge. None when the nerves do not agree up to gauge.
pub fn extract_rule(family: &[(Grid, Grid)]) -> Option<super::sentence::SentenceRule> {
    if family.is_empty() {
        return None;
    }
    let scale = family_ratio(family);
    let nerves: Vec<super::sentence::Nerve> = family.iter().map(|(a, b)| pair_nerve(a, b, scale)).collect();
    let mut rule = super::sentence::agree_up_to_gauge(&nerves)?;
    if let Some(r) = scale {
        rule.ratio = r;
    }
    Some(rule)
}

fn ratio_rho(ratio: ((usize, usize), (usize, usize))) -> BigRational {
    let ((hn, hd), (wn, wd)) = ratio;
    if hd == 0 || wd == 0 {
        return q(1);
    }
    if hn * wd == wn * hd {
        BigRational::new(BigInt::from(hn), BigInt::from(hd.max(1)))
    } else {
        q(1)
    }
}

fn realize_word(word: &super::alphabet::Word, rho: &BigRational, shift: &(BigRational, BigRational)) -> Versor {
    element::conformal_of(
        &super::alphabet::Spelling { word: word.clone(), rho: rho.clone() },
        shift,
    )
    .unwrap_or_else(element::identity_versor)
}

fn germ_idx_key(i: usize) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&(i as u64).to_le_bytes());
    k
}

fn output_frame(fx: (usize, usize), ratio: ((usize, usize), (usize, usize))) -> (usize, usize) {
    let ((hn, hd), (wn, wd)) = ratio;
    let hd = hd.max(1);
    let wd = wd.max(1);
    if fx.0 == 0 || fx.1 == 0 || (fx.0 * hn) % hd != 0 || (fx.1 * wn) % wd != 0 {
        return fx;
    }
    (fx.0 * hn / hd, fx.1 * wn / wd)
}

fn test_transport(
    rule: &super::sentence::SentenceRule,
    g: &super::sentence::Germ,
    gi: usize,
    test_c: &(BigRational, BigRational),
    placed: &crate::network::nerve::Keyed<(BigRational, BigRational)>,
) -> Versor {
    let rho = ratio_rho(rule.ratio);
    if let Some((dr, dc)) = g.shift.clone().or_else(|| rule.shift_of(g)) {
        return realize_word(&g.word, &rho, &(dr, dc));
    }
    let linear = realize_word(&g.word, &rho, &(q(0), q(0)));
    let t_body = cga::translator(&test_c.0, &test_c.1);
    let t_inv = cga::translator(&-&test_c.0, &-&test_c.1);
    let placed_linear = t_body.geo(&linear).geo(&t_inv);
    let frame_i = rule.nerve.germs.iter().position(|h| h.kind == super::peel::ShellKind::Frame);
    let face = rule.nerve.faces.iter().find(|f| {
        (f.i == gi || f.j == gi)
            && (rule.nerve.germs.get(f.i).is_some_and(|h| h.kind == super::peel::ShellKind::Frame)
                || rule.nerve.germs.get(f.j).is_some_and(|h| h.kind == super::peel::ShellKind::Frame))
    });
    let Some(f) = face else {
        return placed_linear;
    };
    let anchor = if rule.nerve.germs.get(f.i).is_some_and(|h| h.kind == super::peel::ShellKind::Frame) {
        f.i
    } else {
        f.j
    };
    let t_anchor = placed
        .get(&germ_idx_key(anchor))
        .cloned()
        .or_else(|| frame_i.and_then(|i| placed.get(&germ_idx_key(i)).cloned()))
        .unwrap_or((q(0), q(0)));
    let face_v = f.versor.clone();
    cga::translator(&t_anchor.0, &t_anchor.1).geo(&face_v).geo(&placed_linear)
}

fn emit_carried(
    body: &Body,
    v: &Versor,
    map: &[(u8, u8)],
    fy: (usize, usize),
    cells: &mut Vec<Cell3>,
    remainder: &mut Vec<Cell3>,
) {
    for &(r, c, h) in &body.cells {
        let hue = map.iter().find(|(f, _)| *f == h).map(|(_, t)| *t).unwrap_or(h);
        if let Some(image) = element::carry_cell(v, r, c) {
            for (i, j) in image {
                if i >= 0 && j >= 0 && (i as usize) < fy.0 && (j as usize) < fy.1 {
                    cells.push((i, j, hue));
                }
            }
        } else {
            remainder.push((r, c, h));
        }
    }
}

fn speak_created(
    rule: &super::sentence::SentenceRule,
    g: &super::sentence::Germ,
    i: usize,
    placed: &crate::network::nerve::Keyed<(BigRational, BigRational)>,
    fy: (usize, usize),
    cells: &mut Vec<Cell3>,
) {
    let rho = ratio_rho(rule.ratio);
    let frame_i = rule.nerve.germs.iter().position(|h| h.kind == super::peel::ShellKind::Frame);
    let face = rule.nerve.faces.iter().find(|f| f.i == i || f.j == i);
    let anchor = face
        .map(|f| if f.i == i { f.j } else { f.i })
        .or(frame_i)
        .unwrap_or(0);
    let test_anchor = placed.get(&germ_idx_key(anchor)).cloned().unwrap_or((q(0), q(0)));
    let train_anchor = {
        let support = super::sentence::after_support(&rule.nerve.germs[anchor]);
        if support.is_empty() {
            (q(0), q(0))
        } else {
            let cells: Vec<Cell3> = support.iter().map(|&(r, c)| (r, c, 1)).collect();
            body_of(cells).centroid
        }
    };
    let face_word = face.map(|f| &f.g_ij).unwrap_or(&g.word);
    let linear = realize_word(face_word, &rho, &(q(0), q(0)));
    let t_test = cga::translator(&test_anchor.0, &test_anchor.1);
    let t_train_inv = cga::translator(&-&train_anchor.0, &-&train_anchor.1);
    let v = t_test.geo(&linear).geo(&t_train_inv);
    for &(r, c, _, a) in &g.chain {
        if a == 0 {
            continue;
        }
        if let Some(image) = element::carry_cell(&v, r, c) {
            for (ir, ic) in image {
                if ir >= 0 && ic >= 0 && (ir as usize) < fy.0 && (ic as usize) < fy.1 {
                    cells.push((ir, ic, a));
                }
            }
        }
    }
}

fn cell3_key(r: i64, c: i64) -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..8].copy_from_slice(&r.to_le_bytes());
    k[8..16].copy_from_slice(&c.to_le_bytes());
    k
}

fn hue_conflict(cells: &[Cell3]) -> bool {
    let mut at = crate::network::nerve::Keyed::new();
    for &(r, c, h) in cells {
        let k = cell3_key(r, c);
        if let Some(&old) = at.get(&k) {
            if old != h {
                return true;
            }
        } else {
            at.insert(k, h);
        }
    }
    false
}

fn hue_conflict_against(prior: &[Cell3], next: &[Cell3]) -> bool {
    let mut at = crate::network::nerve::Keyed::new();
    for &(r, c, h) in prior {
        at.insert(cell3_key(r, c), h);
    }
    for &(r, c, h) in next {
        if let Some(&old) = at.get(&cell3_key(r, c)) {
            if old != h {
                return true;
            }
        }
    }
    false
}

fn support_centroid(g: &super::sentence::Germ) -> (BigRational, BigRational) {
    let s = super::sentence::after_support(g);
    if s.is_empty() {
        return (q(0), q(0));
    }
    let cells: Vec<Cell3> = s.iter().map(|&(r, c)| (r, c, 1)).collect();
    body_of(cells).centroid
}

fn section_agrees(
    rule: &super::sentence::SentenceRule,
    gi: usize,
    body_c: &(BigRational, BigRational),
    placed: &crate::network::nerve::Keyed<(BigRational, BigRational)>,
) -> bool {
    let Some(g) = rule.nerve.germs.get(gi) else {
        return false;
    };
    let this_t = support_centroid(g);
    for f in &rule.nerve.faces {
        let other = if f.i == gi {
            f.j
        } else if f.j == gi {
            f.i
        } else {
            continue;
        };
        let Some(anchor) = placed.get(&germ_idx_key(other)) else {
            continue;
        };
        let other_t = support_centroid(&rule.nerve.germs[other]);
        let expected = (
            anchor.0.clone() + &this_t.0 - &other_t.0,
            anchor.1.clone() + &this_t.1 - &other_t.1,
        );
        let dr = &expected.0 - &body_c.0;
        let dc = &expected.1 - &body_c.1;
        if &dr * &dr + &dc * &dc > qn(1, 4) {
            return false;
        }
    }
    true
}

/// Mint the family's sentence: μ⃗ = [S, G, T, H] of the closed cochain.
pub fn mint_sentence(rule: &super::sentence::SentenceRule, family: &[(Grid, Grid)]) -> Option<element::Element> {
    let mut e = super::sentence::as_element(&rule.nerve)?;
    let mut mag = super::sentence::cochain_magnitude(&rule.nerve);
    mag[2] = mag[2].max((family.len() as u64).saturating_sub(1).max(1));
    e.set_magnitude(mag);
    Some(e)
}

/// SPEAK A RULE onto a test input: match bodies to rule germs by chart,
/// instantiate transports from faces and the word's linear block, never
/// from a training pair's absolute versor. Several bodies on one germ is
/// a section; one body on two germs is a section whose face words to
/// already-placed anchors agree. None only when two anchors conflict on
/// the same output cells.
pub fn speak_rule(rule: &super::sentence::SentenceRule, x: &Grid) -> Option<Speech> {
    let fx = frame_of(x);
    let fy = output_frame(fx, rule.ratio);
    let mut atlas = super::stalk::Stalks::default();
    // index: germs of this rule
    for (i, g) in rule.nerve.germs.iter().enumerate() {
        if g.kind == super::peel::ShellKind::Frame {
            continue;
        }
        let mut addr = germ_idx_key(i);
        if let Some(inv) = g.source.as_ref() {
            atlas.admit(inv, addr);
        } else if let Some(inv) = g.target.as_ref() {
            addr[8] = 1;
            atlas.admit(inv, addr);
        }
    }
    let mut claimed: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    let mut placed: crate::network::nerve::Keyed<(BigRational, BigRational)> = crate::network::nerve::Keyed::new();
    if let Some(i) = rule.nerve.germs.iter().position(|g| g.kind == super::peel::ShellKind::Frame) {
        placed.insert(germ_idx_key(i), scene_center(fy));
    }
    let mut remainder: Vec<Cell3> = Vec::new();
    let mut cells: Vec<Cell3> = Vec::new();
    let scale = Some(rule.ratio).filter(|r| *r != ((1, 1), (1, 1)));
    for body in perceive(x) {
        let mut hits: Vec<usize> = Vec::new();
        let src_hits = atlas.fiber(&body.moduli);
        for addr in src_hits {
            let idx = u64::from_le_bytes(addr[..8].try_into().ok()?) as usize;
            if !hits.contains(&idx) {
                hits.push(idx);
            }
        }
        if hits.is_empty() {
            for shell in super::peel::peel_body_scaled(&body.cells, scale) {
                if shell.cells.is_empty() {
                    continue;
                }
                let sb = body_of(shell.cells);
                let sh = atlas.fiber(&sb.moduli);
                for addr in sh {
                    let idx = u64::from_le_bytes(addr[..8].try_into().ok()?) as usize;
                    if !hits.contains(&idx) {
                        hits.push(idx);
                    }
                }
            }
        }
        let speak_idx: Vec<usize> = {
            let free: Vec<usize> = hits
                .iter()
                .copied()
                .filter(|&idx| !claimed.contains(&germ_idx_key(idx)))
                .collect();
            match free.len() {
                0 => {
                    remainder.extend(body.cells.iter().copied());
                    continue;
                }
                1 => free,
                _ => {
                    let mut ranked = free;
                    ranked.sort_by(|&a, &b| {
                        let ca = support_centroid(&rule.nerve.germs[a]);
                        let cb = support_centroid(&rule.nerve.germs[b]);
                        let da = {
                            let dr = &ca.0 - &body.centroid.0;
                            let dc = &ca.1 - &body.centroid.1;
                            &dr * &dr + &dc * &dc
                        };
                        let db = {
                            let dr = &cb.0 - &body.centroid.0;
                            let dc = &cb.1 - &body.centroid.1;
                            &dr * &dr + &dc * &dc
                        };
                        da.cmp(&db)
                    });
                    vec![ranked[0]]
                }
            }
        };
        let start = cells.len();
        let mut body_remainder: Vec<Cell3> = Vec::new();
        for &idx in &speak_idx {
            let addr = germ_idx_key(idx);
            if !claimed.contains(&addr) {
                claimed.insert(addr, idx);
            }
            let Some(g) = rule.nerve.germs.get(idx) else {
                body_remainder.extend(body.cells.iter().copied());
                continue;
            };
            placed.insert(addr, body.centroid.clone());
            let map = g.target.as_ref().map(|t| rule.hue_for(t)).unwrap_or_default();
            if g.source.is_none() {
                speak_created(rule, g, idx, &placed, fy, &mut cells);
                continue;
            }
            let v = test_transport(rule, g, idx, &body.centroid, &placed);
            emit_carried(&body, &v, &map, fy, &mut cells, &mut body_remainder);
        }
        if hue_conflict(&cells[start..]) || hue_conflict_against(&cells[..start], &cells[start..]) {
            return None;
        }
        remainder.extend(body_remainder);
    }
    // index: created germs of this rule
    for (i, g) in rule.nerve.germs.iter().enumerate() {
        if g.source.is_some() || g.kind == super::peel::ShellKind::Frame {
            continue;
        }
        if claimed.contains(&germ_idx_key(i)) {
            continue;
        }
        speak_created(rule, g, i, &placed, fy, &mut cells);
    }
    cells.sort_unstable();
    cells.dedup();
    remainder.sort_unstable();
    Some(Speech { frame: fy, cells, remainder })
}

/// The residual between what was spoken and what was observed, as a chain
/// (row, col, spoken hue, observed hue) over every cell that differs.
fn residual_of(spoken: Option<&Grid>, observed: &Grid) -> Vec<(i64, i64, u8, u8)> {
    let mut out = Vec::new();
    for (r, row) in observed.iter().enumerate() {
        for (c, &h) in row.iter().enumerate() {
            let s = spoken.and_then(|g| g.get(r)).and_then(|row| row.get(c)).copied().unwrap_or(0);
            if s != h {
                out.push((r as i64, c as i64, s, h));
            }
        }
    }
    if let Some(g) = spoken {
        for (r, row) in g.iter().enumerate() {
            for (c, &s) in row.iter().enumerate() {
                let inside = observed.get(r).and_then(|row| row.get(c)).is_some();
                if !inside && s != 0 {
                    out.push((r as i64, c as i64, s, 0));
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// The image of a hue under a gauge map: the hue the family showed is
/// carried as observed; a hue it never showed has no image — refused, not
/// guessed.
fn gauge_image(sigma: &[(u8, u8)], h: u8) -> Option<u8> {
    sigma.iter().find(|(f, _)| *f == h).map(|(_, t)| *t)
}

/// Render one body through one transport onto the output frame: the cells
/// (row, col, hue) it speaks, or None when a hue has no image.
/// Carry the body's support forward by V, each cell as a blade, and read
/// the hue of every image cell through the gauge rule. Only the support is
/// visited; the output frame is the wire's boundary, so an image cell
/// outside it is not spoken. The cells come back in canonical order so two
/// readings compare as sets.
/// Carry the body's support forward by V, each cell as a blade, and read
/// the hue of every image cell through the gauge rule. Only the support is
/// visited; the output frame is the wire's boundary, so an image cell
/// outside it is not spoken. A cell whose hue has no image under the rule
/// is not dropped: it rides out as the remainder (in the input's own
/// coordinates). The spoken cells come back in canonical order so two
/// readings compare as sets.
fn render(body: &Body, v: &Versor, sigma: &[(u8, u8)], fy: (usize, usize), total: bool) -> Option<(Vec<Cell3>, Vec<Cell3>)> {
    let mut out: Vec<Cell3> = Vec::new();
    let mut remainder: Vec<Cell3> = Vec::new();
    for &(r, c, h) in &body.cells {
        // THE GAUGE AS A TOTAL MAP: under `total` a hue the family never
        // showed rides as itself — the identity section the minted
        // connection already carries on its hue rows (apply g). Under the
        // partial map it is the remainder.
        let image = match (gauge_image(sigma, h), total) {
            (Some(hue), _) => Some(hue),
            (None, true) => Some(h),
            (None, false) => None,
        };
        let Some(hue) = image else {
            remainder.push((r, c, h));
            continue;
        };
        for (i, j) in element::carry_cell(v, r, c)? {
            if i < 0 || j < 0 || i as usize >= fy.0 || j as usize >= fy.1 {
                continue;
            }
            out.push((i, j, hue));
        }
    }
    out.sort_unstable();
    out.dedup();
    Some((out, remainder))
}

/// What a transport says about a new input: the spoken set of cells on
/// the output frame, and the remainder — the input cells whose hue the
/// rules give no image, carried out unspoken rather than dropped. The
/// frame is the wire's rectangle; the speech itself is a set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Speech {
    pub frame: (usize, usize),
    pub cells: Vec<Cell3>,
    pub remainder: Vec<Cell3>,
}

impl Speech {
    /// The wire form: the spoken set laid on the frame's rectangle. Only
    /// here does a rectangle exist; an unspoken cell reads as the ground.
    pub fn grid(&self) -> Grid {
        let mut out: Grid = vec![vec![0u8; self.frame.1]; self.frame.0];
        for &(r, c, h) in &self.cells {
            out[r as usize][c as usize] = h;
        }
        out
    }
}

/// SPEAK a test input through the connection. None when the output frame
/// is not integral, a body has no rule, the rule's readings render
/// differently, or two carried bodies disagree on a cell — refusal, never
/// a guess. A hue with no image is not a refusal: it is the remainder on
/// the speech.
pub fn speak(t: &Transport, x: &Grid) -> Option<Speech> {
    speak_with(t, x, false)
}

/// SPEAK THROUGH THE MINTED CONNECTION: the same transport applied as the
/// total map its word carries — a hue outside the family's palette is its
/// own image (the identity section). This is apply g: the close-test for a
/// thought, and the gauge puncture that lets an unseen hue ride.
pub fn speak_total(t: &Transport, x: &Grid) -> Option<Speech> {
    speak_with(t, x, true)
}

fn speak_with(t: &Transport, x: &Grid, total: bool) -> Option<Speech> {
    let fx = frame_of(x);
    let ((hn, hd), (wn, wd)) = t.ratio;
    if fx.0 == 0 || fx.1 == 0 || (fx.0 * hn) % hd.max(1) != 0 || (fx.1 * wn) % wd.max(1) != 0 {
        return None;
    }
    let fy = (fx.0 * hn / hd.max(1), fx.1 * wn / wd.max(1));
    let (cx, cy) = (scene_center(fx), scene_center(fy));
    let mut cells: Vec<Cell3> = Vec::new();
    let mut remainder: Vec<Cell3> = Vec::new();
    for body in perceive(x) {
        if t.removed.contains(&body.moduli) {
            continue;
        }
        let Some(rule) = t.rule(&body.moduli) else {
            // Unreached body: do not drop the scene. Under the total map,
            // identity-carry as a trivial section of g (1, σ₀); always flag
            // the body's cells as remainder so the closed sub-morphism can
            // still be uttered.
            if total {
                if let Some((said, _)) = render(&body, &element::identity_versor(), &[], fy, true) {
                    cells.extend(said);
                }
            }
            remainder.extend(body.cells.iter().copied());
            continue;
        };
        let mut spoken: Option<(Vec<Cell3>, Vec<Cell3>)> = None;
        for (reading, u) in &rule.readings {
            let shifted = (&body.centroid.0 + &rule.shift.0, &body.centroid.1 + &rule.shift.1);
            let v = match *reading {
                0 => u.clone(),
                1 => read_out(u, &cx, &cy),
                2 => read_out(u, &body.centroid, &shifted),
                3 => read_out(u, &cx, &shifted),
                _ => read_out(u, &body.centroid, &cy),
            };
            let rendered = render(&body, &v, &rule.sigma, fy, total)?;
            match &spoken {
                None => spoken = Some(rendered),
                Some(held) => {
                    if *held != rendered {
                        return None; // the readings disagree on this body: no speech
                    }
                }
            }
        }
        let (said, unsaid) = spoken?;
        cells.extend(said);
        remainder.extend(unsaid);
    }
    for creation in &t.created {
        for &(r, c, h) in &creation.cells {
            let (rr, cc) = match creation.reading {
                0 => (r, c),
                _ => {
                    let (dr, dc) = (q(r) + cy.0.clone() * q(2), q(c) + cy.1.clone() * q(2));
                    let (dr, dc) = (dr / q(2), dc / q(2));
                    if !dr.is_integer() || !dc.is_integer() {
                        return None;
                    }
                    (dr.to_integer().try_into().unwrap_or(i64::MAX), dc.to_integer().try_into().unwrap_or(i64::MAX))
                }
            };
            if rr < 0 || cc < 0 || rr as usize >= fy.0 || cc as usize >= fy.1 {
                return None; // a creation outside the frame: no speech
            }
            cells.push((rr, cc, h));
        }
    }
    // Two bodies speaking different hues on one cell contradict: refusal.
    cells.sort_unstable();
    cells.dedup();
    if cells.windows(2).any(|w| (w[0].0, w[0].1) == (w[1].0, w[1].1)) {
        return None;
    }
    remainder.sort_unstable();
    Some(Speech { frame: fy, cells, remainder })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Grid {
        rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
    }

    /// A family that shifts every body two columns right and recolours
    /// 1→2 agrees in every reading; a body mirrored in place agrees only
    /// about the bodies' own centroids; a family that dilates the scene
    /// twofold agrees in the scene's own coordinates. Each speaks a new
    /// input exactly.
    #[test]
    fn a_shift_a_mirror_and_a_dilation_are_comprehended_and_spoken() {
        let family = vec![
            (grid(&["1100", "0000", "0000"]), grid(&["0011", "0000", "0000"])),
            (grid(&["0000", "1100", "0000"]), grid(&["0000", "0011", "0000"])),
        ];
        let family: Vec<(Grid, Grid)> = family
            .into_iter()
            .map(|(a, b)| (a, b.into_iter().map(|row| row.into_iter().map(|h| if h == 1 { 2 } else { h }).collect()).collect()))
            .collect();
        let c = comprehend(&family);
        assert!(c.residual.is_empty(), "the family replays: {:?}", c.residual);
        let t = c.transport.expect("comprehended");
        assert_eq!(t.rules.len(), 1);
        let readings: Vec<usize> = t.rules[0].readings.iter().map(|(r, _)| *r).collect();
        assert_eq!(readings, vec![0, 1, 2], "a constant shift agrees in the scene, frame, and body readings; the mixed anchors move with the body");
        assert_eq!(t.rules[0].sigma, vec![(1, 2)]);
        let x = grid(&["0000", "0000", "1100"]);
        assert_eq!(speak(&t, &x).map(|s| s.grid()), Some(grid(&["0000", "0000", "0022"])));
        // A hue the family never showed has no image: it rides out as the
        // remainder of the speech, never a guess and never dropped.
        let unsaid = speak(&t, &grid(&["0000", "0000", "3300"])).expect("spoken with a remainder");
        assert!(unsaid.cells.is_empty());
        assert_eq!(unsaid.remainder, vec![(2, 0, 3), (2, 1, 3)]);
        // An extra body of unseen moduli (a lone cell, not the family's bar)
        // does not refuse the scene: identity-carry plus remainder of that
        // body, the rest still spoken.
        let mixed = speak_total(&t, &grid(&["1100", "0000", "0030"])).expect("partial g");
        assert!(mixed.cells.iter().any(|&(r, c, h)| r == 0 && c == 2 && h == 2), "the reached body speaks");
        assert!(mixed.remainder.iter().any(|&(r, c, h)| r == 2 && c == 2 && h == 3), "the unreached body is remainder, not a dropped scene");
        let reached = perceive(&grid(&["1100", "0000", "0000"]));
        assert_eq!(reached.len(), 1);
        assert!(t.rule(&reached[0].moduli).is_some(), "the class is a keyed read, not a scan");
        let lone = perceive(&grid(&["0030"]));
        assert_eq!(lone.len(), 1);
        assert!(t.rule(&lone[0].moduli).is_none(), "unreached moduli have no rule");
        // A body mirrored in place: the reflection axis moves with the body,
        // so the transport agrees only about the bodies' own centroids.
        let family = vec![
            (grid(&["100", "110", "000"]), grid(&["010", "110", "000"])),
            (grid(&["000", "010", "011"]), grid(&["000", "001", "011"])),
        ];
        let c = comprehend(&family);
        assert!(c.residual.is_empty(), "the mirror replays: {:?}", c.residual);
        let t = c.transport.expect("comprehended");
        let readings: Vec<usize> = t.rules[0].readings.iter().map(|(r, _)| *r).collect();
        assert_eq!(readings, vec![2], "read about the body's centroid only");
        assert_eq!(speak(&t, &grid(&["000", "100", "110"])).map(|s| s.grid()), Some(grid(&["000", "010", "110"])));
        // Dilation: a 2×2 scene becomes 4×4, every body doubled about the origin.
        let family = vec![
            (grid(&["10", "00"]), grid(&["1100", "1100", "0000", "0000"])),
            (grid(&["00", "03"]), grid(&["0000", "0000", "0033", "0033"])),
        ];
        let c = comprehend(&family);
        assert!(c.residual.is_empty(), "the dilation replays: {:?}", c.residual);
        let t = c.transport.expect("comprehended");
        assert_eq!(t.ratio, ((2, 1), (2, 1)));
        assert!(t.rules[0].readings.iter().any(|(r, _)| *r == 0), "read in the scene's own coordinates");
        assert_eq!(speak(&t, &grid(&["30", "00"])).map(|s| s.grid()), Some(grid(&["3300", "3300", "0000", "0000"])));
    }

    /// A family whose rule depends on something the connection does not
    /// carry leaves a residual: the exact observed cells not spoken.
    #[test]
    fn an_uncomprehended_family_leaves_its_residual_as_the_demand() {
        let family = vec![
            (grid(&["100", "000"]), grid(&["001", "000"])),
            (grid(&["100", "000"]), grid(&["000", "001"])),
        ];
        let c = comprehend(&family);
        assert!(c.transport.is_none());
        assert!(c.framed);
        assert!(!c.residual.is_empty());
        assert!(c.residual.iter().all(|&(_, _, s, h)| s != h));
        let disagree = vec![
            (grid(&["10", "00"]), grid(&["1100", "1100", "0000", "0000"])),
            (grid(&["10", "00"]), grid(&["110", "110", "000"])),
        ];
        let c = comprehend(&disagree);
        assert!(!c.framed, "frames that disagree are not one connection");
    }

    #[test]
    fn a_shift_family_extracts_a_rule_and_speaks_the_test() {
        let family = vec![
            (grid(&["1100", "0000"]), grid(&["0011", "0000"])),
            (grid(&["0000", "1100"]), grid(&["0000", "0011"])),
        ];
        let family: Vec<(Grid, Grid)> = family
            .into_iter()
            .map(|(a, b)| {
                (
                    a,
                    b.into_iter()
                        .map(|row| row.into_iter().map(|h| if h == 1 { 2 } else { h }).collect())
                        .collect(),
                )
            })
            .collect();
        let c = comprehend(&family);
        let rule = extract_rule(&family).expect("the family extracts a rule");
        let bodies = rule
            .nerve
            .germs
            .iter()
            .filter(|g| g.kind != crate::transducer::peel::ShellKind::Frame)
            .count();
        assert_eq!(bodies, 1, "one body is one germ, got {}", rule.nerve.germ_count());
        assert!(
            rule.nerve.germs.iter().any(|g| g.kind == crate::transducer::peel::ShellKind::Frame),
            "a frame germ is on the nerve"
        );
        let frame_face = rule.nerve.faces.iter().any(|f| {
            rule.nerve.germs.get(f.i).is_some_and(|g| g.kind == crate::transducer::peel::ShellKind::Frame)
                || rule.nerve.germs.get(f.j).is_some_and(|g| g.kind == crate::transducer::peel::ShellKind::Frame)
        });
        let shifted = rule.nerve.germs.iter().any(|g| g.shift.is_some());
        assert!(frame_face || shifted, "placement is a frame face or a family-constant shift");
        let spoken = speak_rule(&rule, &grid(&["1100", "0000"])).expect("the test is spoken");
        assert!(spoken.remainder.is_empty(), "the test is reached in full: {:?}", spoken.remainder);
        assert_eq!(spoken.grid(), grid(&["0022", "0000"]));
        let t = c.transport.expect("still a connection");
        assert_eq!(speak_total(&t, &grid(&["1100", "0000"])).map(|s| s.remainder.is_empty()), Some(true));
    }

    /// Pairing is ∃V on the shapes, not a digest of the moduli axes.
    #[test]
    fn pairing_is_carriage_not_a_moduli_digest() {
        let family = vec![(grid(&["1100", "0000"]), grid(&["0011", "0000"]))];
        let rule = extract_rule(&family).expect("a translated bar extracts");
        let bodies = rule
            .nerve
            .germs
            .iter()
            .filter(|g| g.kind != crate::transducer::peel::ShellKind::Frame)
            .count();
        assert_eq!(bodies, 1, "one body is one germ under ∃V, got {}", rule.nerve.germ_count());
        let a = perceive(&family[0].0);
        let b = perceive(&family[0].1);
        assert_eq!(a[0].moduli, b[0].moduli, "the chart is exact axes, not a hash");
        assert!(carriage(&a[0], &b[0]).is_some(), "a versor carries the bar");
    }

    /// Three identical squares in one family are a section: the test
    /// recolours all three.
    #[test]
    fn three_identical_squares_recolour_all_three_on_the_test() {
        let train_in = grid(&[
            "11011",
            "11011",
            "00000",
            "11000",
            "11000",
        ]);
        let train_out: Grid = train_in
            .iter()
            .map(|row| row.iter().map(|&h| if h == 1 { 2 } else { h }).collect())
            .collect();
        let family = vec![(train_in.clone(), train_out.clone())];
        let rule = extract_rule(&family).expect("three squares extract as a section");
        let spoken = speak_rule(&rule, &train_in).expect("the test is spoken");
        assert!(spoken.remainder.is_empty(), "all three squares are reached: {:?}", spoken.remainder);
        assert_eq!(spoken.grid(), train_out);
        let minted = mint_sentence(&rule, &family).expect("the family mints");
        let mu = minted.magnitude();
        assert!(mu[2] >= 1, "lane 2 carries the scene-state span");
        assert!(mu[1] >= 1, "recolour is gauge content, got {mu:?}");
        assert_ne!(mu, [0, 0, 0, 1], "mint is the unflattened 4-vector");
    }

    /// Correspondence across pairs is transport class (kind, word, hue),
    /// not body moduli. A square and a bar recoloured 1→2 are one class;
    /// frames of different sizes match by kind.
    #[test]
    fn correspondence_is_transport_class_not_shape() {
        let family = vec![
            (grid(&["1100", "1100"]), grid(&["2200", "2200"])),
            (grid(&["1110", "0000"]), grid(&["2220", "0000"])),
        ];
        let rule = extract_rule(&family).expect("different shapes, same recolour");
        let bodies = rule
            .nerve
            .germs
            .iter()
            .filter(|g| g.kind != crate::transducer::peel::ShellKind::Frame)
            .count();
        assert!(bodies >= 1, "a body germ of the recolour class survives, got {}", rule.nerve.germ_count());
        assert!(
            rule.nerve.germs.iter().any(|g| g.kind == crate::transducer::peel::ShellKind::Frame),
            "frames match by kind, not [MAX,h,w]"
        );
        assert!(
            rule.nerve.germ_count() >= 2,
            "frame + body, got {} germs {} faces",
            rule.nerve.germ_count(),
            rule.nerve.faces.len()
        );
    }
}
