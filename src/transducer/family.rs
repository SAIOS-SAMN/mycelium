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
        for (j, ob) in s.bodies_out.iter().enumerate() {
            let best: Option<(usize, Carriage)> = s
                .bodies_in
                .iter()
                .enumerate()
                .filter(|(i, ab)| !used[*i] && ab.moduli == ob.moduli)
                .find_map(|(i, ab)| carriage(ab, ob).map(|car| (i, car)));
            match best {
                Some((i, car)) => {
                    used[i] = true;
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
            rules.push(Rule {
                moduli: class.clone(),
                readings,
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
    let transport = Transport { ratio, rules, removed, created: creations };
    // 5. REPLAY the connection on every pair; the first failure's residual
    // is the demand.
    for (a, b) in family {
        let spoken = speak(&transport, a).and_then(|s| match s.remainder.as_slice() {
            [] => Some(s.grid()),
            _ => None,
        });
        if spoken.as_ref() != Some(b) {
            out.residual = residual_of(spoken.as_ref(), b);
            return out;
        }
    }
    out.transport = Some(transport);
    out
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
        let rule = t.rules.iter().find(|r| r.moduli == body.moduli)?;
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
}
