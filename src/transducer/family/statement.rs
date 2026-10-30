// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.143 — SPEAKING A FORM'S CLAUSES ON A SCENE.
//!
//! The scene's bodies are the ones the court perceives (`perceive`): each a
//! maximal hue-blind connected region, its support and hues its own exact
//! coordinates. A clause SELECTS among them by a typed predicate, read on the
//! input as it stands — every clause selects on the same scene — and its act
//! paints the canvas, which opens as the input: a hue map or a carriage on
//! the selection, a region its boundary encloses filled, cells created
//! relative to it, or the output cropped to it. Nothing is chosen silently:
//! a rank ties only where the rule says what a tie does, a coordinate is read
//! exactly or the clause is refused by name ([`Unstated`]), and the court
//! refuses a form whose clause is unstated on a pair it replays.

use super::*;
use crate::core::Q;
use crate::transducer::sentence::form::clause::{exact_root, About, Clause, Cmp, Coord, Extent, Order, Paint, Pred, Region, Stop, Tie, Value};
use crate::transducer::sentence::form::rule_element::{self, Carriage, Lowered, Reach, RuleElement, Trace};

/// WHY A CLAUSE IS NOT STATED ON A SCENE, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unstated {
    /// A rank met a tie its order does not name.
    Tie,
    /// A predicate read a coordinate that is not exact on a body: a radius
    /// that is no rational, or a μ lane, which is never a rule's coordinate.
    Inexact,
    /// A crop, or a copy's template, selected no single body or extent.
    NotOne,
    /// A carried or copied cell landed off the lattice of cells.
    OffLattice,
    /// A created line names no ray.
    Still,
    /// An act on parts names another act on parts: parts are read one level.
    Nested,
    /// The input has not the frame the act names: no two equal panes it
    /// splits into, or a quarter turn of a tile that is not square.
    Unframed,
    /// A hue read off a partner (`Paint::Of`) found no touched body the
    /// predicate selects, or bodies of more than one hue; or a join, which
    /// has two ends and no one partner, was asked for one.
    Unpartnered,
}

/// THE SQUARE'S SYMMETRY `t` ON A GRID (`Act::Tile`'s codes): 0 itself, 1
/// mirrored across its columns, 2 across its rows, 3 a turn of 180°, 4 a turn
/// of 90°, 5 of 270°, 6 the transpose, 7 the anti-transpose. `None` for a
/// code past 7, or a quarter turn or transpose of a grid that is not square.
pub(crate) fn turn_of(g: &[Vec<u8>], t: u8) -> Option<Grid> {
    let h = g.len();
    let w = match g {
        [row, ..] => row.len(),
        [] => 0,
    };
    let at = |i: usize, j: usize| g.get(i).and_then(|row| row.get(j)).copied();
    let grid = |rows: usize, cols: usize, f: &dyn Fn(usize, usize) -> Option<u8>| -> Option<Grid> { (0..rows).map(|i| (0..cols).map(|j| f(i, j)).collect()).collect() };
    let last = |n: usize, i: usize| n.checked_sub(1).and_then(|m| m.checked_sub(i));
    match t {
        0 => Some(g.to_vec()),
        1 => grid(h, w, &|i, j| at(i, last(w, j)?)),
        2 => grid(h, w, &|i, j| at(last(h, i)?, j)),
        3 => grid(h, w, &|i, j| at(last(h, i)?, last(w, j)?)),
        4..=7 if h == w => match t {
            4 => grid(h, w, &|i, j| at(last(h, j)?, i)),
            5 => grid(h, w, &|i, j| at(j, last(w, i)?)),
            6 => grid(h, w, &|i, j| at(j, i)),
            _ => grid(h, w, &|i, j| at(last(h, j)?, last(w, i)?)),
        },
        _ => None,
    }
}

/// WHAT THE CLAUSES SPOKE: the output frame and its painted cells.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Spoken {
    pub frame: (usize, usize),
    pub painted: Vec<(i64, i64, u8)>,
}

/// THE SCENE AS THE CLAUSES READ IT, perceived once: each body's cells and
/// the body each cell belongs to.
struct Seen<'a> {
    x: &'a Grid,
    bodies: &'a [Body],
    rows: usize,
    cols: usize,
    cells: Vec<Vec<(usize, usize, u8)>>,
    owner: Vec<Vec<Option<usize>>>,
}

const EDGES: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const AROUND: [(i64, i64); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn qu(n: usize) -> Option<Q> {
    Some(q(i64::try_from(n).ok()?))
}

/// A body's extent `(top, left, bottom, right)`, read over every cell.
pub(crate) fn extent_of(cells: &[(usize, usize, u8)]) -> Option<(usize, usize, usize, usize)> {
    let [(r0, c0, _), ..] = cells else { return None };
    let (mut t, mut l, mut b, mut r) = (*r0, *c0, *r0, *c0);
    for &(rr, cc, _) in cells {
        if rr < t {
            t = rr;
        }
        if rr > b {
            b = rr;
        }
        if cc < l {
            l = cc;
        }
        if cc > r {
            r = cc;
        }
    }
    Some((t, l, b, r))
}

impl<'a> Seen<'a> {
    fn of(x: &'a Grid, bodies: &'a [Body]) -> Seen<'a> {
        let rows = x.len();
        let cols = match x.as_slice() {
            [row, ..] => row.len(),
            [] => 0,
        };
        let mut owner = vec![vec![None; cols]; rows];
        let mut cells = Vec::with_capacity(bodies.len());
        for (i, b) in bodies.iter().enumerate() {
            let mut owned = Vec::with_capacity(b.compartments.len());
            for l in &b.compartments {
                let Some((r, c)) = on_grid(&l.site) else { continue };
                let (Ok(r), Ok(c)) = (usize::try_from(r), usize::try_from(c)) else { continue };
                if let Some(o) = owner.get_mut(r).and_then(|row| row.get_mut(c)) {
                    *o = Some(i);
                    owned.push((r, c, l.hue));
                }
            }
            cells.push(owned);
        }
        Seen { x, bodies, rows, cols, cells, owner }
    }

    fn at(&self, r: usize, c: usize, dr: i64, dc: i64) -> Option<(usize, usize)> {
        let rr = usize::try_from(i64::try_from(r).ok()? + dr).ok()?;
        let cc = usize::try_from(i64::try_from(c).ok()? + dc).ok()?;
        (rr < self.rows && cc < self.cols).then_some((rr, cc))
    }

    fn hue(&self, r: usize, c: usize) -> u8 {
        match self.x.get(r).and_then(|row| row.get(c)) {
            Some(h) => *h,
            None => crate::arc::ground(),
        }
    }

    fn owner_at(&self, r: usize, c: usize) -> Option<usize> {
        self.owner.get(r).and_then(|row| row.get(c)).copied().flatten()
    }

    /// THE BACKGROUND CELLS THE BODIES OF `set` ENCLOSE between them: every
    /// background cell no edge-connected path reaches from the grid's border
    /// without crossing a cell of a body of `set`. A body standing inside the
    /// enclosure is inside it, not a breach of it.
    fn enclosed(&self, set: &[usize]) -> Vec<(usize, usize)> {
        let wall = |r: usize, c: usize| self.owner_at(r, c).is_some_and(|o| set.contains(&o));
        let mut reached = vec![vec![false; self.cols]; self.rows];
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for (r, row) in reached.iter_mut().enumerate() {
            for (c, cell) in row.iter_mut().enumerate() {
                let edge = r == 0 || c == 0 || r + 1 == self.rows || c + 1 == self.cols;
                if edge && !wall(r, c) {
                    *cell = true;
                    stack.push((r, c));
                }
            }
        }
        while let Some((a, b)) = stack.pop() {
            for (dr, dc) in EDGES {
                let Some((na, nb)) = self.at(a, b, dr, dc) else { continue };
                if !reached[na][nb] && !wall(na, nb) {
                    reached[na][nb] = true;
                    stack.push((na, nb));
                }
            }
        }
        let mut out = Vec::new();
        for (r, row) in reached.iter().enumerate() {
            for (c, &got) in row.iter().enumerate() {
                if !got && self.hue(r, c) == crate::arc::ground() {
                    out.push((r, c));
                }
            }
        }
        out
    }

    /// The other bodies `b` touches.
    fn touching(&self, b: usize) -> Vec<usize> {
        let mut out = Vec::new();
        for &(r, c, _) in &self.cells[b] {
            for (dr, dc) in AROUND {
                if let Some(o) = self.at(r, c, dr, dc).and_then(|(a, d)| self.owner_at(a, d)) {
                    if o != b && !out.contains(&o) {
                        out.push(o);
                    }
                }
            }
        }
        out
    }

    /// THE HUE OF `b`'S PARTNER (`Paint::Of`): the one hue every cell of the
    /// bodies `b` touches that `pred` selects carries. None selected, or
    /// cells of two hues among them, is refused by name.
    fn partner_hue(&self, b: usize, pred: &Pred) -> Result<u8, Unstated> {
        let partners = self.select(pred, &self.touching(b))?;
        let mut hue: Option<u8> = None;
        for &o in &partners {
            for &(_, _, h) in &self.cells[o] {
                match hue {
                    Some(k) if k != h => return Err(Unstated::Unpartnered),
                    _ => hue = Some(h),
                }
            }
        }
        hue.ok_or(Unstated::Unpartnered)
    }

    /// THE PAINT ON BODY `b`'s CELL OF HUE `h`.
    fn painted(&self, with: &Paint, b: usize, h: u8) -> Result<u8, Unstated> {
        match with {
            Paint::Own => Ok(h),
            Paint::Hue(k) => Ok(*k),
            Paint::Of(pred) => self.partner_hue(b, pred),
        }
    }

    /// THE COORDINATE, READ EXACTLY: `Ok(None)` where it does not read on
    /// this body (a tally of a body of several hues), refused where it is not
    /// exact.
    fn read(&self, coord: Coord, b: usize) -> Result<Option<Q>, Unstated> {
        let cells = &self.cells[b];
        let ext = extent_of(cells);
        let span = |lo: usize, hi: usize| hi.checked_sub(lo).and_then(|d| d.checked_add(1)).and_then(qu);
        let mut hues: Vec<u8> = cells.iter().map(|&(_, _, h)| h).collect();
        hues.sort_unstable();
        hues.dedup();
        Ok(match coord {
            Coord::Shell => qu(cells.len()),
            Coord::Height => ext.and_then(|(t, _, bt, _)| span(t, bt)),
            Coord::Width => ext.and_then(|(_, l, _, r)| span(l, r)),
            Coord::Top => ext.and_then(|(t, ..)| qu(t)),
            Coord::Left => ext.and_then(|(_, l, ..)| qu(l)),
            Coord::Bottom => ext.and_then(|(_, _, bt, _)| qu(bt)),
            Coord::Right => ext.and_then(|(.., r)| qu(r)),
            Coord::Hues => qu(hues.len()),
            Coord::Contacts => qu(self.touching(b).len()),
            Coord::Enclosed => qu(self.enclosed(&[b]).len()),
            Coord::Tally => match hues.as_slice() {
                [h] => qu(self.x.iter().flatten().filter(|&&v| v == *h).count()),
                _ => None,
            },
            Coord::Peers => qu(self.bodies.iter().filter(|o| o.moduli == self.bodies[b].moduli).count()),
            Coord::Radius => Some(exact_root(&self.bodies[b].shape.blade.radius2()).ok_or(Unstated::Inexact)?),
            Coord::Lane(_) => return Err(Unstated::Inexact),
        })
    }

    fn all(&self) -> Vec<usize> {
        (0..self.bodies.len()).collect()
    }

    /// THE BODIES OF `domain` THE PREDICATE SELECTS, in the domain's order.
    fn select(&self, pred: &Pred, domain: &[usize]) -> Result<Vec<usize>, Unstated> {
        Ok(match pred {
            Pred::All => domain.to_vec(),
            Pred::Hue(h) => domain.iter().copied().filter(|&b| self.cells[b].iter().any(|c| c.2 == *h)).collect(),
            Pred::Chart(inv) => domain.iter().copied().filter(|&b| self.bodies[b].moduli == *inv).collect(),
            Pred::Cmp { coord, cmp, to } => {
                let to = match to {
                    Value::Integer(n) => q(*n),
                    Value::Count(p) => qu(self.select(p, &self.all())?.len()).ok_or(Unstated::Inexact)?,
                };
                let mut out = Vec::new();
                for &b in domain {
                    let Some(v) = self.read(*coord, b)? else { continue };
                    let holds = match cmp {
                        Cmp::Eq => v == to,
                        Cmp::Less => v < to,
                        Cmp::Greater => v > to,
                    };
                    if holds {
                        out.push(b);
                    }
                }
                out
            }
            Pred::Rank { coord, order, place, tie } => self.rank(*coord, *order, *place, *tie, domain)?,
            Pred::Touches(p) => {
                let them = self.select(p, &self.all())?;
                domain.iter().copied().filter(|&b| self.touching(b).iter().any(|o| them.contains(o))).collect()
            }
            Pred::Not(p) => {
                let s = self.select(p, domain)?;
                domain.iter().copied().filter(|b| !s.contains(b)).collect()
            }
            Pred::And(ps) => {
                let mut d = domain.to_vec();
                for p in ps {
                    d = self.select(p, &d)?;
                }
                d
            }
            Pred::Any(ps) => {
                let mut hit = Vec::new();
                for p in ps {
                    hit.extend(self.select(p, domain)?);
                }
                domain.iter().copied().filter(|b| hit.contains(b)).collect()
            }
        })
    }

    /// COMPARE (Concordance 1.10): the bodies standing at the `place`-th
    /// distinct value of the coordinate in the order the rule names, a tie
    /// held, refused, or read by the further coordinate the rule names. A
    /// body on which the coordinate does not read stands at no place.
    fn rank(&self, coord: Coord, order: Order, place: u32, tie: Tie, domain: &[usize]) -> Result<Vec<usize>, Unstated> {
        let mut read: Vec<(Q, usize)> = Vec::new();
        for &b in domain {
            if let Some(v) = self.read(coord, b)? {
                read.push((v, b));
            }
        }
        let mut values: Vec<Q> = read.iter().map(|(v, _)| v.clone()).collect();
        values.sort_unstable();
        values.dedup();
        if order == Order::Greatest {
            values.reverse();
        }
        let Some(at) = usize::try_from(place).ok().and_then(|i| values.get(i)) else {
            return Ok(Vec::new());
        };
        let tied: Vec<usize> = read.iter().filter(|(v, _)| v == at).map(|(_, b)| *b).collect();
        if tied.len() < 2 {
            return Ok(tied);
        }
        match tie {
            Tie::Hold => Ok(tied),
            Tie::Refuse => Err(Unstated::Tie),
            Tie::Then(c, o) => self.rank(c, o, 0, Tie::Refuse, &tied),
        }
    }

    /// THE BACKGROUND CELLS A REGION NAMES about the selection, each once
    /// per body that names it (a fill paints them all one hue).
    fn region(&self, region: Region, sel: &[usize]) -> Vec<(usize, usize)> {
        match region {
            Region::Interior => self.enclosed(sel),
            Region::Halo => {
                let mut out = Vec::new();
                for &b in sel {
                    for &(r, c, _) in &self.cells[b] {
                        for (dr, dc) in AROUND {
                            if let Some((a, d)) = self.at(r, c, dr, dc) {
                                if self.hue(a, d) == crate::arc::ground() {
                                    out.push((a, d));
                                }
                            }
                        }
                    }
                }
                out
            }
            Region::Extent => {
                let mut out = Vec::new();
                for &b in sel {
                    let Some((t, l, bt, r)) = extent_of(&self.cells[b]) else { continue };
                    for a in t..=bt {
                        for d in l..=r {
                            if self.hue(a, d) == crate::arc::ground() {
                                out.push((a, d));
                            }
                        }
                    }
                }
                out
            }
        }
    }

    /// A body's centre in the sensor's coordinates (each index plus ½):
    /// the centre of its extent, exact.
    fn centre(&self, b: usize) -> Option<[Q; 2]> {
        let (t, l, bt, r) = extent_of(&self.cells[b])?;
        let two = q(2);
        Some([(qu(t)? + qu(bt)? + q(1)) / &two, (qu(l)? + qu(r)? + q(1)) / &two])
    }
}

/// THE CELL A VERSOR CARRIES `(r, c)` ONTO, read where the sensor reads it —
/// the compartment's centre — and only where it lands on a cell's centre
/// in the frame it left: a centre between cells is off the lattice.
pub(crate) fn carried(v: &Versor, r: usize, c: usize) -> Result<(i64, i64), Unstated> {
    let site = Site::of([
        i64::try_from(r).map_err(|_| Unstated::OffLattice)?,
        i64::try_from(c).map_err(|_| Unstated::OffLattice)?,
        0,
    ]);
    let p = element::carry_site_center(v, &site).ok_or(Unstated::OffLattice)?;
    let half = Q::new(1.into(), 2.into());
    let [pr, pc, pk] = p.as_slice() else { return Err(Unstated::OffLattice) };
    let (ir, ic) = (pr - &half, pc - &half);
    if !ir.is_integer() || !ic.is_integer() || pk != &q(0) {
        return Err(Unstated::OffLattice);
    }
    let whole = |x: &Q| i64::try_from(x.to_integer()).map_err(|_| Unstated::OffLattice);
    Ok((whole(&ir)?, whole(&ic)?))
}

fn mapped(map: &[(u8, u8)], h: u8) -> u8 {
    map.iter().find(|(f, _)| *f == h).map_or(h, |(_, t)| *t)
}

/// A landed cell past the frame is no cell of this grid and is not spoken on
/// it, as a carried compartment past the frame is not.
fn lay(canvas: &mut [Vec<u8>], r: i64, c: i64, h: u8) {
    if let (Ok(r), Ok(c)) = (usize::try_from(r), usize::try_from(c)) {
        if let Some(cell) = canvas.get_mut(r).and_then(|row| row.get_mut(c)) {
            *cell = h;
        }
    }
}

/// SPEAK A CARRIAGE on the selection `sel` of `seen`, onto `canvas`.
fn carriage(seen: &Seen, t: &Carriage, sel: &[usize], canvas: &mut [Vec<u8>]) -> Result<(), Unstated> {
    let void = crate::arc::ground();
    let colour = |b: usize, h: u8| -> Result<u8, Unstated> { Ok(mapped(&t.gauge, seen.painted(&t.paint, b, h)?)) };
    match t.reach {
        Reach::Once => {
            // The identity carries every cell onto itself: no versor is
            // applied to a cell it cannot move.
            let fixed = t.motion.projective_eq(&rule_element::still());
            let mut landed: Vec<(i64, i64, u8)> = Vec::new();
            for &b in sel {
                let v = match (fixed, t.about) {
                    (true, _) => None,
                    (false, About::Grid) => Some(t.motion.clone()),
                    (false, About::Body) => {
                        let [cr, cc] = seen.centre(b).ok_or(Unstated::OffLattice)?;
                        let to = crate::transducer::cga::translator_at(&[cr.clone(), cc.clone()]);
                        let from = crate::transducer::cga::translator_at(&[-cr, -cc]);
                        Some(to.geo(&t.motion).geo(&from))
                    }
                };
                for &(r, c, h) in &seen.cells[b] {
                    let (tr, tc) = match &v {
                        Some(v) => carried(v, r, c)?,
                        None => (i64::try_from(r).map_err(|_| Unstated::OffLattice)?, i64::try_from(c).map_err(|_| Unstated::OffLattice)?),
                    };
                    landed.push((tr, tc, colour(b, h)?));
                }
            }
            if t.trace == Trace::Clears {
                for &b in sel {
                    for &(r, c, _) in &seen.cells[b] {
                        canvas[r][c] = void;
                    }
                }
            }
            for (r, c, h) in landed {
                lay(canvas, r, c, h);
            }
        }
        Reach::Meet(stop) => {
            let (dr, dc) = rule_element::lattice_step(&t.motion).filter(|&s| s != (0, 0)).ok_or(Unstated::Still)?;
            match t.trace {
                Trace::Sweeps => {
                    // A line stops at what was painted BEFORE this clause, so
                    // no line of the clause stops another and the order the
                    // bodies are read in is no part of what it says.
                    let before = canvas.to_vec();
                    for &b in sel {
                        for &(r, c, h) in &seen.cells[b] {
                            let colour = colour(b, h)?;
                            let (mut a, mut d) = (r, c);
                            while let Some((na, nd)) = seen.at(a, d, dr, dc) {
                                if stop == Stop::Contact && before[na][nd] != void {
                                    break;
                                }
                                canvas[na][nd] = colour;
                                (a, d) = (na, nd);
                            }
                        }
                    }
                }
                Trace::Clears | Trace::Keeps => {
                    // THE NEAREST THEIR STOP FIRST: a body's reach is its
                    // leading cell's along the ray. Each reach lands on the
                    // canvas as the nearer ones left it, so a stack settles
                    // as it lands; bodies of ONE reach land together, against
                    // the canvas as it stood before them — no order among
                    // them is chosen — and two of them landing on one cell is
                    // a tie, refused by name.
                    let reach = |b: usize| -> Option<i64> {
                        seen.cells[b].iter().try_fold(None, |most: Option<i64>, &(r, c, _)| {
                            let at = i64::try_from(r).ok()?.checked_mul(dr)?.checked_add(i64::try_from(c).ok()?.checked_mul(dc)?)?;
                            Some(Some(match most {
                                Some(m) if m >= at => m,
                                _ => at,
                            }))
                        })?
                    };
                    let mut order: Vec<(i64, usize)> = Vec::with_capacity(sel.len());
                    for &b in sel {
                        order.push((reach(b).ok_or(Unstated::OffLattice)?, b));
                    }
                    #[expect(clippy::disallowed_methods, reason = "rule 4 — the slide's own order, content: the reach read off each body along the ray, nearest its stop first; one reach lands together")]
                    order.sort_by_key(|&(reach, _)| std::cmp::Reverse(reach));
                    let mut at = 0usize;
                    while let Some(&(reach_here, _)) = order.get(at) {
                        let group: Vec<usize> = order[at..].iter().take_while(|(r, _)| *r == reach_here).map(|&(_, b)| b).collect();
                        at += group.len();
                        for &b in &group {
                            for &(r, c, _) in &seen.cells[b] {
                                canvas[r][c] = void;
                            }
                        }
                        let mut landed: Vec<(usize, usize, u8)> = Vec::new();
                        for &b in &group {
                            let cells = &seen.cells[b];
                            // The furthest step every cell takes onto the
                            // grid — and, at a contact stop, onto ground: the
                            // meet ends it.
                            let lands = |k: i64| -> bool {
                                cells.iter().all(|&(r, c, _)| match seen.at(r, c, k * dr, k * dc) {
                                    Some((a, d)) => stop == Stop::Border || canvas[a][d] == void,
                                    None => false,
                                })
                            };
                            let mut k: i64 = 0;
                            while lands(k + 1) {
                                k += 1;
                            }
                            for &(r, c, h) in cells {
                                let (a, d) = seen.at(r, c, k * dr, k * dc).ok_or(Unstated::OffLattice)?;
                                if landed.iter().any(|&(la, ld, _)| (la, ld) == (a, d)) {
                                    return Err(Unstated::Tie);
                                }
                                landed.push((a, d, colour(b, h)?));
                            }
                        }
                        for (a, d, h) in landed {
                            canvas[a][d] = h;
                        }
                        // A carriage that keeps its source leaves it where
                        // nothing landed on it.
                        if t.trace == Trace::Keeps {
                            for &b in &group {
                                for &(r, c, h) in &seen.cells[b] {
                                    if canvas[r][c] == void {
                                        canvas[r][c] = h;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// SPEAK THE CLAUSES on `x`, whose bodies `bodies` are (the scene the form
/// is spoken onto, perceived once): each clause lowered to its element
/// ([`rule_element::lower`]) and the elements spoken. A run naming parts
/// inside parts is refused by name.
pub fn speak_clauses(clauses: &[Clause], x: &Grid, bodies: &[Body]) -> Result<Spoken, Unstated> {
    speak_elements(&rule_element::lower_all(clauses).ok_or(Unstated::Nested)?, x, bodies)
}

/// SPEAK THE ELEMENTS on `x`. The canvas opens as the input; each element
/// selects on the input and paints the canvas; a window sets the output's
/// extent. An element on parts selects among the scene's monochrome parts
/// (`perceive_parts`), read once where one asks.
pub fn speak_elements(said: &[Lowered], x: &Grid, bodies: &[Body]) -> Result<Spoken, Unstated> {
    let whole = Seen::of(x, bodies);
    let parts: Vec<Body> = match said.iter().any(|l| l.parts) {
        true => perceive_parts(x),
        false => Vec::new(),
    };
    let on_parts = Seen::of(x, &parts);
    let mut canvas: Vec<Vec<u8>> = x.clone();
    let mut window: Option<(usize, usize, usize, usize)> = None;
    let mut scale: Option<(u8, u8)> = None;
    // A frame element's output stands in place of the canvas (`Panes`, `Tile`).
    let mut framed: Option<Grid> = None;
    for l in said {
        let seen = match l.parts {
            true => &on_parts,
            false => &whole,
        };
        let all = seen.all();
        let sel = seen.select(&l.select, &all)?;
        match &l.element {
            RuleElement::Carriage(t) => carriage(seen, t, &sel, &mut canvas)?,
            RuleElement::Chain { region, hue } => {
                for (r, c) in seen.region(*region, &sel) {
                    canvas[r][c] = *hue;
                }
            }
            RuleElement::Stamp { template } => {
                let chosen = seen.select(template, &all)?;
                let [t] = chosen.as_slice() else { return Err(Unstated::NotOne) };
                let t = *t;
                let [tr, tc] = seen.centre(t).ok_or(Unstated::NotOne)?;
                let mut landed: Vec<(i64, i64, u8)> = Vec::new();
                let seeds: Vec<usize> = sel.iter().copied().filter(|&b| b != t).collect();
                for &b in &seeds {
                    let [sr, sc] = seen.centre(b).ok_or(Unstated::OffLattice)?;
                    let v = crate::transducer::cga::translator_at(&[sr - &tr, sc - &tc]);
                    for &(r, c, h) in &seen.cells[t] {
                        let (lr, lc) = carried(&v, r, c)?;
                        landed.push((lr, lc, h));
                    }
                }
                for &b in &seeds {
                    for &(r, c, _) in &seen.cells[b] {
                        canvas[r][c] = crate::arc::ground();
                    }
                }
                for (r, c, h) in landed {
                    lay(&mut canvas, r, c, h);
                }
            }
            RuleElement::Window(extent) => {
                let [b] = sel.as_slice() else { return Err(Unstated::NotOne) };
                let cells: Vec<(usize, usize, u8)> = match extent {
                    Extent::Body => seen.cells[*b].clone(),
                    Extent::Interior => seen.enclosed(&[*b]).into_iter().map(|(r, c)| (r, c, 0)).collect(),
                };
                window = Some(extent_of(&cells).ok_or(Unstated::NotOne)?);
            }
            RuleElement::Dilate { up, down } => scale = Some((*up, *down)),
            RuleElement::Panes { along_rows, gap, table, paint: hue } => {
                let void = crate::arc::ground();
                let (n, other) = match along_rows {
                    true => (whole.rows, whole.cols),
                    false => (whole.cols, whole.rows),
                };
                let k = n.checked_sub(usize::from(*gap)).filter(|m| m % 2 == 0).map(|m| m / 2).ok_or(Unstated::Unframed)?;
                if k == 0 {
                    return Err(Unstated::Unframed);
                }
                let cell = |a: usize, b: usize| -> u8 { if *along_rows { canvas[a][b] } else { canvas[b][a] } };
                // The pane's cell at `a` along the split, `b` across it.
                let at = |a: usize, b: usize| -> u8 {
                    let first = cell(a, b) != void;
                    let second = cell(a + k + usize::from(*gap), b) != void;
                    let bit = 2 * usize::from(first) + usize::from(second);
                    if (table >> bit) & 1 == 1 { *hue } else { void }
                };
                let out: Grid = match along_rows {
                    true => (0..k).map(|a| (0..other).map(|b| at(a, b)).collect()).collect(),
                    false => (0..other).map(|b| (0..k).map(|a| at(a, b)).collect()).collect(),
                };
                framed = Some(out);
            }
            RuleElement::Tile { rows, cols, turns } => {
                let (h, w) = (whole.rows, whole.cols);
                let (rows, cols) = (usize::from(*rows), usize::from(*cols));
                if turns.len() != rows * cols || rows == 0 || cols == 0 {
                    return Err(Unstated::Unframed);
                }
                let mut out = vec![vec![crate::arc::ground(); w * cols]; h * rows];
                for (i, &t) in turns.iter().enumerate() {
                    let block = turn_of(&canvas, t).ok_or(Unstated::Unframed)?;
                    let (bi, bj) = (i / cols, i % cols);
                    for (r, row) in block.iter().enumerate() {
                        for (c, &v) in row.iter().enumerate() {
                            out[bi * h + r][bj * w + c] = v;
                        }
                    }
                }
                framed = Some(out);
            }
            RuleElement::Join { corner: None, paint: with } => {
                // A join has two ends, and no one partner to take a hue off.
                if matches!(with, Paint::Of(_)) {
                    return Err(Unstated::Unpartnered);
                }
                // Joined through what was ground BEFORE this clause, so no
                // join of the clause blocks another.
                let before = canvas.clone();
                let void = crate::arc::ground();
                let chosen = |r: usize, c: usize| -> Option<(usize, u8)> {
                    let o = seen.owner_at(r, c)?;
                    sel.contains(&o).then(|| (o, seen.hue(r, c)))
                };
                let mut joins: Vec<(usize, usize, u8)> = Vec::new();
                // Each row read along its columns, each column along its rows.
                for (along_rows, outer, inner) in [(true, seen.rows, seen.cols), (false, seen.cols, seen.rows)] {
                    for a in 0..outer {
                        let cell = |i: usize| if along_rows { (a, i) } else { (i, a) };
                        let mut last: Option<(usize, usize, u8)> = None;
                        for i in 0..inner {
                            let (r, c) = cell(i);
                            let Some((o, h)) = chosen(r, c) else {
                                if before[r][c] != void {
                                    last = None;
                                }
                                continue;
                            };
                            if let Some((li, lo, lh)) = last {
                                let hue = match with {
                                    Paint::Hue(k) => Some(*k),
                                    Paint::Own => (lh == h).then_some(h),
                                    Paint::Of(_) => None,
                                };
                                if let (true, Some(hue)) = (lo != o && i > li + 1, hue) {
                                    for m in li + 1..i {
                                        let (mr, mc) = cell(m);
                                        joins.push((mr, mc, hue));
                                    }
                                }
                            }
                            last = Some((i, o, h));
                        }
                    }
                }
                for (r, c, h) in joins {
                    canvas[r][c] = h;
                }
            }
            RuleElement::Join { corner: Some(to), paint: with } => {
                // A bend has two ends, and no one partner to take a hue off.
                if matches!(with, Paint::Of(_)) {
                    return Err(Unstated::Unpartnered);
                }
                // Bent through what was ground BEFORE this clause, so no bend
                // of the clause blocks another.
                let before = canvas.clone();
                let void = crate::arc::ground();
                let targets = seen.select(to, &all)?;
                let one_hue = |b: usize| -> Option<u8> {
                    let [(_, _, h), rest @ ..] = seen.cells[b].as_slice() else { return None };
                    rest.iter().all(|c| c.2 == *h).then_some(*h)
                };
                let mut joins: Vec<(usize, usize, u8)> = Vec::new();
                for &f in &sel {
                    for &t in targets.iter().filter(|&&t| t != f) {
                        let (Some((ft, fl, fb, fr)), Some((tt, tl, tb, tr))) = (extent_of(&seen.cells[f]), extent_of(&seen.cells[t])) else {
                            continue;
                        };
                        // The source stands on one row, the target on one
                        // column: otherwise no one corner is named.
                        if ft != fb || tl != tr {
                            return Err(Unstated::NotOne);
                        }
                        let (row, col) = (ft, tl);
                        // Facing along a row or a column: no corner.
                        if (fl..=fr).contains(&col) || (tt..=tb).contains(&row) {
                            continue;
                        }
                        let hue = match with {
                            Paint::Hue(k) => *k,
                            Paint::Own => match (one_hue(f), one_hue(t)) {
                                (Some(a), Some(b)) if a == b => a,
                                _ => continue,
                            },
                            Paint::Of(_) => return Err(Unstated::Unpartnered),
                        };
                        let across: Vec<usize> = match col > fr {
                            true => (fr + 1..=col).collect(),
                            false => (col..fl).collect(),
                        };
                        let along: Vec<usize> = match tt > row {
                            true => (row + 1..tt).collect(),
                            false => (tb + 1..row).collect(),
                        };
                        let path: Vec<(usize, usize)> = across.into_iter().map(|c| (row, c)).chain(along.into_iter().map(|r| (r, col))).collect();
                        if path.iter().all(|&(r, c)| before[r][c] == void) {
                            joins.extend(path.into_iter().map(|(r, c)| (r, c, hue)));
                        }
                    }
                }
                for (r, c, h) in joins {
                    canvas[r][c] = h;
                }
            }
        }
    }
    let out: Vec<Vec<u8>> = match framed {
        // A frame act's output is the output; a crop of the input's frame
        // does not cut it.
        Some(g) => g,
        None => {
            let (t, l, b, r) = match window {
                Some(w) => w,
                None => (0, 0, whole.rows.checked_sub(1).ok_or(Unstated::NotOne)?, whole.cols.checked_sub(1).ok_or(Unstated::NotOne)?),
            };
            canvas.iter().take(b + 1).skip(t).map(|row| row.iter().take(r + 1).skip(l).copied().collect()).collect()
        }
    };
    let out = match scale {
        None => out,
        Some((up, down)) => scaled(&out, up, down)?,
    };
    let void = crate::arc::ground();
    let mut painted = Vec::new();
    for (i, row) in out.iter().enumerate() {
        for (j, &h) in row.iter().enumerate() {
            if h != void {
                let (Ok(pi), Ok(pj)) = (i64::try_from(i), i64::try_from(j)) else {
                    return Err(Unstated::OffLattice);
                };
                painted.push((pi, pj, h));
            }
        }
    }
    let cols = match out.as_slice() {
        [row, ..] => row.len(),
        [] => 0,
    };
    Ok(Spoken { frame: (out.len(), cols), painted })
}

/// 78.167 C — WHAT EACH PREDICATE SELECTS on one scene, as a clause reads
/// it (the scene seen once): `None` where the predicate is unstated there.
/// The act reading separates an effect's bodies by these and nothing else.
pub(crate) fn select_each(preds: &[Pred], x: &Grid, bodies: &[Body]) -> Vec<Option<Vec<usize>>> {
    let seen = Seen::of(x, bodies);
    let all = seen.all();
    preds.iter().map(|p| seen.select(p, &all).ok()).collect()
}

/// THE REGION SEVERAL BODIES NAME TOGETHER, as a fill selecting them speaks
/// it (`Seen::region` over the set): their joint interior is every ground
/// cell the set walls in, which no one of them may.
pub(crate) fn region_of_set(region: Region, x: &Grid, bodies: &[Body], set: &[usize]) -> Vec<(usize, usize)> {
    Seen::of(x, bodies).region(region, set)
}

/// The bodies of the scene owning a cell beside one of `cells`, corners
/// included, each once, in the scene's order.
pub(crate) fn bodies_around(x: &Grid, bodies: &[Body], cells: &[(usize, usize, u8)]) -> Vec<usize> {
    let seen = Seen::of(x, bodies);
    let mut out: Vec<usize> = Vec::new();
    for &(r, c, _) in cells {
        for (dr, dc) in AROUND {
            if let Some(o) = seen.at(r, c, dr, dc).and_then(|(a, d)| seen.owner_at(a, d)) {
                if !out.contains(&o) {
                    out.push(o);
                }
            }
        }
    }
    out.sort_unstable();
    out
}

/// THE HUE EACH BODY'S PARTNERS CARRY, as `Paint::Of(Pred::All)` reads it on
/// one scene: the one hue of every body it touches, `None` where it touches
/// none or bodies of two hues. The act reading asks whether a recoloured
/// body took its partner's hue.
pub(crate) fn partner_hues(x: &Grid, bodies: &[Body]) -> Vec<Option<u8>> {
    let seen = Seen::of(x, bodies);
    (0..bodies.len()).map(|b| seen.partner_hue(b, &Pred::All).ok()).collect()
}

/// 78.167 C — the cells a region names about each ONE body of the scene, as
/// a fill would paint them: the act reading asks which bodies' regions the
/// created cells lie in.
pub(crate) fn regions_each(region: Region, x: &Grid, bodies: &[Body]) -> Vec<Vec<(usize, usize)>> {
    let seen = Seen::of(x, bodies);
    (0..bodies.len()).map(|b| seen.region(region, &[b])).collect()
}

/// THE GRID SCALED BY `up / down`, one of them 1: each cell an `up × up`
/// block, or each `down × down` block, uniform, one cell. A block that is
/// not uniform, an extent `down` does not divide, a zero or a ratio of two
/// factors is no scaling of cells, refused by name.
fn scaled(g: &[Vec<u8>], up: u8, down: u8) -> Result<Vec<Vec<u8>>, Unstated> {
    let (up, down) = (usize::from(up), usize::from(down));
    let cols = match g {
        [row, ..] => row.len(),
        [] => 0,
    };
    match (up, down) {
        (1, 1) => Ok(g.to_vec()),
        (u, 1) if u > 1 => Ok((0..g.len() * u)
            .map(|i| (0..cols * u).map(|j| g[i / u][j / u]).collect())
            .collect()),
        (1, d) if d > 1 && g.len() % d == 0 && cols % d == 0 => {
            let mut out = Vec::with_capacity(g.len() / d);
            for bi in 0..g.len() / d {
                let mut row = Vec::with_capacity(cols / d);
                for bj in 0..cols / d {
                    let h = g[bi * d][bj * d];
                    let uniform = (0..d).all(|i| (0..d).all(|j| g[bi * d + i][bj * d + j] == h));
                    if !uniform {
                        return Err(Unstated::OffLattice);
                    }
                    row.push(h);
                }
                out.push(row);
            }
            Ok(out)
        }
        _ => Err(Unstated::OffLattice),
    }
}
