// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! FAMILY TRANSPORT — a family of transitions comprehended as a connection.
//!
//! A training pair is two perceived scenes. Each output body is the image
//! of an input body under a transport: the versor solved from the two
//! bodies' moments and the gauge map read cell by cell. The family is
//! comprehended when, class by class (bodies of the same moduli), the
//! transports agree in one chart — the scene's own coordinates, the
//! coordinates about the frame centers, or the coordinates about the
//! bodies' own centroids — with one gauge map. Bodies created with no
//! source must be the same body in every pair; bodies removed must be
//! removed in every pair. What does not agree is the residual: the exact
//! cells of the observed output the connection does not speak, which is
//! the demand the family raises. Nothing here searches: every transport
//! is a quotient, every agreement an equality in the algebra.
//!
//! Speaking a test input applies the connection: each body of the input is
//! carried by its class's transport read back out of its chart, and the
//! output is rendered by carrying every output cell's center back into the
//! body — the serialization of the transported bodies onto the sensor's
//! frame, the one place the lattice is allowed.

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
    blade::components(&cells)
        .into_iter()
        .map(|comp| {
            let mut pos: Vec<(i64, i64)> = comp.iter().map(|&(r, c, _)| (r, c)).collect();
            pos.sort_unstable();
            let shape = blade::shape_of(&pos);
            let moduli = Invariant(blade::moduli(&shape.blade));
            let centroid =
                shape.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2))).unwrap_or((q(0), q(0)));
            let mut cells = comp;
            cells.sort_unstable();
            Body { cells, shape, moduli, centroid }
        })
        .collect()
}

fn frame_of(g: &Grid) -> (usize, usize) {
    (g.len(), g.first().map_or(0, |r| r.len()))
}

fn frame_center(f: (usize, usize)) -> (BigRational, BigRational) {
    (qn(f.0 as i64, 2), qn(f.1 as i64, 2))
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

/// The charts a transport is read in: the scene's own coordinates, the
/// coordinates about the frame centers, the coordinates about the bodies'
/// centroids (with the centroid displacement carried alongside).
pub const CHARTS: usize = 3;

fn in_chart(v: &Versor, chart: usize, a: &Body, b: &Body, fa: (usize, usize), fb: (usize, usize)) -> Versor {
    let (pa, pb) = match chart {
        0 => return v.clone(),
        1 => (frame_center(fa), frame_center(fb)),
        _ => (a.centroid.clone(), b.centroid.clone()),
    };
    cga::translator(&-pb.0, &-pb.1).geo(v).geo(&cga::translator(&pa.0, &pa.1))
}

fn from_chart(u: &Versor, chart: usize, x: &Body, fx: (usize, usize), fy: (usize, usize), shift: &(BigRational, BigRational)) -> Versor {
    let (px, py) = match chart {
        0 => return u.clone(),
        1 => (frame_center(fx), frame_center(fy)),
        _ => (x.centroid.clone(), (&x.centroid.0 + &shift.0, &x.centroid.1 + &shift.1)),
    };
    cga::translator(&py.0, &py.1).geo(u).geo(&cga::translator(&-px.0, &-px.1))
}

/// A class rule: bodies of these moduli are carried by `u` read in `chart`
/// (with the centroid displacement `shift` in the body chart) and recoloured
/// by `sigma`.
#[derive(Clone, Debug)]
pub struct Rule {
    pub moduli: Invariant,
    pub chart: usize,
    pub u: Versor,
    pub shift: (BigRational, BigRational),
    pub sigma: Vec<(u8, u8)>,
}

/// A body created with no source, the same in every pair: its cells in
/// the scene's coordinates (chart 0) or as doubled offsets from the frame
/// center (chart 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Creation {
    pub chart: usize,
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
/// first pair that fails — the family's demand.
#[derive(Clone, Debug, Default)]
pub struct Comprehended {
    pub transport: Option<Transport>,
    pub residual: Vec<(i64, i64, u8, u8)>,
}

struct Pairing {
    pair: usize,
    input: usize,
    output: usize,
    carriage: Carriage,
}

/// COMPREHEND a family: pair every output body with the input body it is
/// the transport of, read each class's transport in the chart the pairs
/// agree in, settle creations and removals, then replay the connection on
/// every pair. The residual of the first pair that does not replay is the
/// family's demand.
pub fn comprehend(family: &[(Grid, Grid)]) -> Comprehended {
    let mut out = Comprehended::default();
    let Some(ratio) = family_ratio(family) else { return out };
    let scenes: Vec<(Vec<Body>, Vec<Body>, (usize, usize), (usize, usize))> = family
        .iter()
        .map(|(a, b)| (perceive(a), perceive(b), frame_of(a), frame_of(b)))
        .collect();
    // 1. PAIR each output body with its source: the input body of the same
    // moduli that a carriage exists for, nearest by centroid displacement.
    let mut pairings: Vec<Pairing> = Vec::new();
    let mut created: Vec<Vec<Cell3>> = Vec::new(); // per pair: unmatched output cells, body by body
    let mut created_by_pair: Vec<Vec<(usize, Vec<Cell3>)>> = vec![Vec::new(); family.len()];
    let mut unmatched_inputs: Vec<(usize, usize)> = Vec::new();
    for (p, (ia, ib, _, _)) in scenes.iter().enumerate() {
        let mut used = vec![false; ia.len()];
        for (j, ob) in ib.iter().enumerate() {
            let mut best: Option<(BigRational, usize, Carriage)> = None;
            for (i, ab) in ia.iter().enumerate() {
                if used[i] || ab.moduli != ob.moduli {
                    continue;
                }
                let Some(car) = carriage(ab, ob) else { continue };
                let (dr, dc) = (&ob.centroid.0 - &ab.centroid.0, &ob.centroid.1 - &ab.centroid.1);
                let d2 = &dr * &dr + &dc * &dc;
                if best.as_ref().map_or(true, |(bd, _, _)| d2 < *bd) {
                    best = Some((d2, i, car));
                }
            }
            match best {
                Some((_, i, car)) => {
                    used[i] = true;
                    pairings.push(Pairing { pair: p, input: i, output: j, carriage: car });
                }
                None => created_by_pair[p].push((j, ob.cells.clone())),
            }
        }
        for (i, u) in used.iter().enumerate() {
            if !u {
                unmatched_inputs.push((p, i));
            }
        }
        created.push(created_by_pair[p].iter().flat_map(|(_, c)| c.clone()).collect());
    }
    // 2. RULES: per class, the chart every pairing agrees in.
    let mut classes: Vec<Invariant> = Vec::new();
    for pr in &pairings {
        let m = &scenes[pr.pair].0[pr.input].moduli;
        if !classes.contains(m) {
            classes.push(m.clone());
        }
    }
    let mut rules: Vec<Rule> = Vec::new();
    let mut disagreeing: Vec<Invariant> = Vec::new();
    for class in &classes {
        let members: Vec<&Pairing> =
            pairings.iter().filter(|pr| scenes[pr.pair].0[pr.input].moduli == *class).collect();
        let mut settled: Option<Rule> = None;
        for chart in 0..CHARTS {
            let mut agreed: Option<Rule> = None;
            let mut ok = true;
            for pr in &members {
                let (ia, ib, fa, fb) = &scenes[pr.pair];
                let (a, b) = (&ia[pr.input], &ib[pr.output]);
                let u = in_chart(&pr.carriage.v, chart, a, b, *fa, *fb);
                let shift = (&b.centroid.0 - &a.centroid.0, &b.centroid.1 - &a.centroid.1);
                match &mut agreed {
                    None => {
                        agreed = Some(Rule { moduli: class.clone(), chart, u, shift, sigma: pr.carriage.sigma.clone() })
                    }
                    Some(rule) => {
                        let same = element::same_versor(&rule.u, &u) && (chart != 2 || rule.shift == shift);
                        match (same, merge_sigma(&rule.sigma, &pr.carriage.sigma)) {
                            (true, Some(sigma)) => rule.sigma = sigma,
                            _ => {
                                ok = false;
                                break;
                            }
                        }
                    }
                }
            }
            if ok {
                settled = agreed;
                break;
            }
        }
        match settled {
            Some(rule) => rules.push(rule),
            None => disagreeing.push(class.clone()),
        }
    }
    // 3. REMOVALS: a class removed in every pair it appears unmatched in,
    // and never carried.
    let mut removed: Vec<Invariant> = Vec::new();
    for (p, i) in &unmatched_inputs {
        let m = &scenes[*p].0[*i].moduli;
        if !removed.contains(m) && !classes.contains(m) {
            removed.push(m.clone());
        }
    }
    // 4. CREATIONS: the same created bodies in every pair, in the scene's
    // coordinates or about the frame center.
    let mut creations: Vec<Creation> = Vec::new();
    if let Some(first) = created_by_pair.first() {
        for (_, cells) in first {
            let fb0 = scenes[0].3;
            let about_center =
                |cells: &[Cell3], fb: (usize, usize)| -> Vec<Cell3> { cells.iter().map(|&(r, c, h)| (2 * r - fb.0 as i64, 2 * c - fb.1 as i64, h)).collect() };
            let absolute_everywhere = created_by_pair.iter().all(|cp| cp.iter().any(|(_, c)| c == cells));
            let centered = about_center(cells, fb0);
            let centered_everywhere = created_by_pair
                .iter()
                .enumerate()
                .all(|(p, cp)| cp.iter().any(|(_, c)| about_center(c, scenes[p].3) == centered));
            match (absolute_everywhere, centered_everywhere) {
                (true, _) => creations.push(Creation { chart: 0, cells: cells.clone() }),
                (false, true) => creations.push(Creation { chart: 1, cells: centered }),
                _ => {}
            }
        }
    }
    let transport = Transport { ratio, rules, removed, created: creations };
    // 5. REPLAY the connection on every pair; the first failure's residual
    // is the demand.
    for (p, (a, b)) in family.iter().enumerate() {
        let spoken = speak(&transport, a);
        let matches = spoken.as_ref() == Some(b);
        if !matches {
            out.residual = residual_of(spoken.as_ref(), b);
            let _ = (p, &created, &disagreeing);
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

/// The image of a hue under a gauge map. A hue the family showed is
/// carried as observed; a hue it did not show is carried only when the
/// map's form is total — the identity on every observed hue, or one
/// constant target — else refused.
fn gauge_image(sigma: &[(u8, u8)], h: u8) -> Option<u8> {
    if let Some((_, t)) = sigma.iter().find(|(f, _)| *f == h) {
        return Some(*t);
    }
    let (first, rest) = sigma.split_first()?;
    if sigma.iter().all(|(f, t)| f == t) {
        return Some(h);
    }
    if rest.iter().all(|(_, t)| *t == first.1) {
        return Some(first.1);
    }
    None
}

/// SPEAK a test input through the connection. None when the output frame
/// is not integral, a body has no rule, a hue has no image under its rule,
/// or two carried bodies disagree on a cell — refusal, never a guess.
pub fn speak(t: &Transport, x: &Grid) -> Option<Grid> {
    let fx = frame_of(x);
    let ((hn, hd), (wn, wd)) = t.ratio;
    if fx.0 == 0 || fx.1 == 0 || (fx.0 * hn) % hd.max(1) != 0 || (fx.1 * wn) % wd.max(1) != 0 {
        return None;
    }
    let fy = (fx.0 * hn / hd.max(1), fx.1 * wn / wd.max(1));
    let mut out: Grid = vec![vec![0u8; fy.1]; fy.0];
    let paint = |out: &mut Grid, r: i64, c: i64, h: u8| -> bool {
        if r < 0 || c < 0 || r as usize >= fy.0 || c as usize >= fy.1 {
            return false;
        }
        let cell = &mut out[r as usize][c as usize];
        if *cell != 0 && *cell != h {
            return false;
        }
        *cell = h;
        true
    };
    for body in perceive(x) {
        if t.removed.contains(&body.moduli) {
            continue;
        }
        let rule = t.rules.iter().find(|r| r.moduli == body.moduli)?;
        let v = from_chart(&rule.u, rule.chart, &body, fx, fy, &rule.shift);
        let inv = v.versor_inverse()?;
        // Render by carrying every output cell's center back into the body.
        for r in 0..fy.0 as i64 {
            for c in 0..fy.1 as i64 {
                let Some(p) = element::carry_center(&inv, r, c) else { continue };
                let (br, bc) = element::cell_of(&p);
                let Ok(i) = body.cells.binary_search_by(|x| (x.0, x.1).cmp(&(br, bc))) else { continue };
                let h = body.cells[i].2;
                let h2 = gauge_image(&rule.sigma, h)?;
                if !paint(&mut out, r, c, h2) {
                    return None;
                }
            }
        }
    }
    for creation in &t.created {
        for &(r, c, h) in &creation.cells {
            let (rr, cc) = match creation.chart {
                0 => (r, c),
                _ => {
                    let (dr, dc) = (r + fy.0 as i64, c + fy.1 as i64);
                    if dr % 2 != 0 || dc % 2 != 0 {
                        return None;
                    }
                    (dr / 2, dc / 2)
                }
            };
            if !paint(&mut out, rr, cc, h) {
                return None;
            }
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(rows: &[&str]) -> Grid {
        rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
    }

    /// A family that shifts every body two columns right and recolours
    /// 1→2 is comprehended in the body chart with a constant displacement;
    /// a new input is spoken exactly; a family that dilates the scene
    /// twofold is comprehended in the scene's own chart.
    #[test]
    fn a_shift_and_a_dilation_are_comprehended_and_spoken() {
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
        assert_eq!(t.rules[0].chart, 0, "a constant shift is one versor in the scene's own coordinates");
        assert_eq!(t.rules[0].sigma, vec![(1, 2)]);
        let x = grid(&["0000", "0000", "1100"]);
        assert_eq!(speak(&t, &x), Some(grid(&["0000", "0000", "0022"])));
        // A body mirrored in place: the reflection axis moves with the body,
        // so the transport agrees only about the bodies' own centroids.
        let family = vec![
            (grid(&["100", "110", "000"]), grid(&["010", "110", "000"])),
            (grid(&["000", "010", "011"]), grid(&["000", "001", "011"])),
        ];
        let c = comprehend(&family);
        assert!(c.residual.is_empty(), "the mirror replays: {:?}", c.residual);
        let t = c.transport.expect("comprehended");
        assert_eq!(t.rules[0].chart, 2, "read about the body's centroid");
        assert_eq!(speak(&t, &grid(&["000", "100", "110"])), Some(grid(&["000", "010", "110"])));
        // Dilation: a 2×2 scene becomes 4×4, every body doubled about the origin.
        let family = vec![
            (grid(&["10", "00"]), grid(&["1100", "1100", "0000", "0000"])),
            (grid(&["00", "03"]), grid(&["0000", "0000", "0033", "0033"])),
        ];
        let c = comprehend(&family);
        assert!(c.residual.is_empty(), "the dilation replays: {:?}", c.residual);
        let t = c.transport.expect("comprehended");
        assert_eq!(t.ratio, ((2, 1), (2, 1)));
        assert_eq!(t.rules[0].chart, 0, "read in the scene's own coordinates");
        assert_eq!(speak(&t, &grid(&["05", "00"])), Some(grid(&["0055", "0055", "0000", "0000"])));
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
        assert!(!c.residual.is_empty());
        assert!(c.residual.iter().all(|&(_, _, s, h)| s != h));
    }
}
