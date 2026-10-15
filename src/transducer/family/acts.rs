// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.167 C — THE INVERSION: THE ACT READ OFF ITS EFFECT.
//!
//! Perception names what changed between a pair's input and output — a body
//! that vanished, cells created, a body moved, the frame resized — and the
//! form's vertices say only the motions. This organ reads the ACT each effect
//! is and writes it in the rule language of 78.143 as a clause the form
//! carries: the motions as carriages, a vanished body as `Recolour(h → 0)`,
//! a recoloured one by its map, created cells as `Fill`, `Extend` or `Copy`,
//! a resized frame as `Scale` or `Crop`.
//!
//! NO SEARCH. Each effect's bodies are read off the pair (which bodies
//! vanished, which regions the created cells lie in, which rays reach them),
//! and the clause's selection is the predicate on which those bodies and
//! the rest SEPARATE exactly on every pair — read from a fixed list of the
//! coordinates the family's bodies carry (78.41(2′): compaction, never a
//! tree). The acts tried are the vocabulary's own fixed list. A clause set
//! stands only where it re-derives EVERY training pair exactly — the
//! perspective's own test — and otherwise it is refused by name ([`Unread`]).

use super::*;
use crate::transducer::sentence::form::clause::{About, Act, Clause, Cmp, Coord, Extent, Order, Paint, Pred, Region, Stop, Tie, Value};

/// WHY NO ACT WAS READ, by name (rule 11).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unread {
    /// The form cites another form: its speech is the cited form's, which a
    /// clause does not restate.
    Cited,
    /// The form unfolds to its frame: a growth no clause states.
    Unfolded,
    /// Cells the input holds are cleared, or changed, by no act read.
    Overwritten,
    /// No coordinate of the list separates the bodies an act touches from
    /// the rest on every pair.
    Inseparable,
    /// Cells were created by no act read.
    Uncreated,
    /// The frame changed by no scale and no crop.
    Unsized,
    /// The stroke was spent before the reading ended.
    Spent,
}

impl Unread {
    /// The slot of the mind's reading ([`crate::observe::ACTS`]).
    pub fn slot(self) -> usize {
        match self {
            Unread::Cited => 1,
            Unread::Unfolded => 2,
            Unread::Overwritten => 3,
            Unread::Inseparable => 4,
            Unread::Uncreated => 5,
            Unread::Unsized => 6,
            Unread::Spent => 7,
        }
    }
}

/// One training pair, its input perceived once.
struct Pair<'f> {
    x: &'f Grid,
    y: &'f Grid,
    bodies: Vec<Body>,
}

/// THE READING over the family: its pairs, the fixed list of predicates
/// their bodies carry, and what each predicate selects on each pair.
struct Reading<'f, 'b> {
    pairs: Vec<Pair<'f>>,
    preds: Vec<Pred>,
    /// `[pair][pred]`: the bodies selected, `None` where it is unstated.
    sels: Vec<Vec<Option<Vec<usize>>>>,
    budget: &'b mut crate::physics::Budget,
}

/// The coordinates a rank or a count is read at, in the list's fixed order.
const RANKED: [Coord; 12] = [
    Coord::Shell,
    Coord::Height,
    Coord::Width,
    Coord::Top,
    Coord::Left,
    Coord::Bottom,
    Coord::Right,
    Coord::Hues,
    Coord::Contacts,
    Coord::Enclosed,
    Coord::Tally,
    Coord::Peers,
];

/// The eight rays a created line can run along.
const RAYS: [(i8, i8); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];

fn cells_of(b: &Body) -> Vec<(usize, usize, u8)> {
    b.compartments
        .iter()
        .filter_map(|l| {
            let (r, c) = on_grid(&l.site)?;
            Some((usize::try_from(r).ok()?, usize::try_from(c).ok()?, l.hue))
        })
        .collect()
}

fn at(g: &Grid, r: usize, c: usize) -> Option<u8> {
    g.get(r).and_then(|row| row.get(c)).copied()
}

fn insert_sorted<T: Ord>(v: &mut Vec<T>, x: T) {
    if let Err(i) = v.binary_search(&x) {
        v.insert(i, x);
    }
}

/// THE FIXED LIST OF PREDICATES the family's bodies carry: every body, each
/// hue and its complement and its contact, each chart, the extremum of each
/// coordinate in each order (ties held), and each exact count of the
/// coordinates a body reads off its own cells.
fn predicates(pairs: &[Pair<'_>]) -> Vec<Pred> {
    let mut hues: Vec<u8> = Vec::new();
    let mut charts: Vec<crate::transducer::stalk::Invariant> = Vec::new();
    let mut counts: [Vec<i64>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for p in pairs {
        for b in &p.bodies {
            let cells = cells_of(b);
            let mut own: Vec<u8> = Vec::new();
            for &(_, _, h) in &cells {
                insert_sorted(&mut hues, h);
                insert_sorted(&mut own, h);
            }
            if !charts.contains(&b.moduli) {
                charts.push(b.moduli.clone());
            }
            let ext = statement::extent_of(&cells);
            let span = |lo: usize, hi: usize| -> Option<i64> { i64::try_from(hi.checked_sub(lo)?.checked_add(1)?).ok() };
            let read = [
                i64::try_from(cells.len()).ok(),
                ext.and_then(|(t, _, b, _)| span(t, b)),
                ext.and_then(|(_, l, _, r)| span(l, r)),
                i64::try_from(own.len()).ok(),
            ];
            for (list, v) in counts.iter_mut().zip(read) {
                if let Some(v) = v {
                    insert_sorted(list, v);
                }
            }
        }
    }
    let mut out = vec![Pred::All];
    out.extend(hues.iter().map(|&h| Pred::Hue(h)));
    out.extend(charts.into_iter().map(Pred::Chart));
    out.extend(hues.iter().map(|&h| Pred::Not(Box::new(Pred::Hue(h)))));
    out.extend(hues.iter().map(|&h| Pred::Touches(Box::new(Pred::Hue(h)))));
    for coord in RANKED {
        for order in [Order::Greatest, Order::Least] {
            out.push(Pred::extremum(coord, order, Tie::Hold));
        }
    }
    for (coord, list) in [Coord::Shell, Coord::Height, Coord::Width, Coord::Hues].into_iter().zip(&counts) {
        for &n in list {
            out.push(Pred::Cmp { coord, cmp: Cmp::Eq, to: Value::Integer(n) });
        }
    }
    out
}

/// The spoken clauses laid on their frame.
fn grid_of(s: &statement::Spoken) -> Option<Grid> {
    let mut g: Grid = vec![vec![0u8; s.frame.1]; s.frame.0];
    for &(r, c, h) in &s.painted {
        *g.get_mut(usize::try_from(r).ok()?)?.get_mut(usize::try_from(c).ok()?)? = h;
    }
    Some(g)
}

impl Reading<'_, '_> {
    /// THE CLAUSES SPOKEN ON EVERY PAIR, one read each under the stroke;
    /// `None` where a clause is unstated on some pair.
    fn speak(&mut self, clauses: &[Clause]) -> Result<Option<Vec<Grid>>, Unread> {
        let mut out = Vec::with_capacity(self.pairs.len());
        for p in &self.pairs {
            if !self.budget.spend() {
                return Err(Unread::Spent);
            }
            match statement::speak_clauses(clauses, p.x, &p.bodies).ok().as_ref().and_then(grid_of) {
                Some(g) => out.push(g),
                None => return Ok(None),
            }
        }
        Ok(Some(out))
    }

    /// Whether the clauses re-derive every training pair exactly.
    fn rederives(&mut self, clauses: &[Clause]) -> Result<bool, Unread> {
        Ok(match self.speak(clauses)? {
            Some(said) => said.iter().zip(&self.pairs).all(|(s, p)| s == p.y),
            None => false,
        })
    }

    /// THE PREDICATE ON WHICH THE TARGETS SEPARATE: the first of the list
    /// that, on every pair, selects within `scope` exactly that pair's
    /// targets (sorted). `None` where no coordinate of the list separates.
    fn separate(&self, targets: &[Vec<usize>], scope: &[Vec<usize>]) -> Option<Pred> {
        (0..self.preds.len())
            .find(|&i| {
                self.sels.iter().zip(targets.iter().zip(scope)).all(|(sels, (t, s))| match &sels[i] {
                    Some(sel) => {
                        let mut within: Vec<usize> = sel.iter().copied().filter(|b| s.contains(b)).collect();
                        within.sort_unstable();
                        within == *t
                    }
                    None => false,
                })
            })
            .map(|i| self.preds[i].clone())
    }

    fn every(&self) -> Vec<Vec<usize>> {
        self.pairs.iter().map(|p| (0..p.bodies.len()).collect()).collect()
    }
}

/// THE FORM'S MOTIONS AS CARRIAGES: each word member's versor (its rotor
/// composed with its carriage, as the vertex speaks it) on the bodies of its
/// chart, with its hue map. A member that moves nothing and maps no hue is
/// no act, and leaves its bodies where the canvas holds them.
fn carriages(form: &Form) -> Result<Vec<Clause>, Unread> {
    let one = crate::transducer::cga::Mv::scalar(crate::core::Q::from_integer(1.into()));
    let mut out = Vec::new();
    let mut carry = |chart: &crate::transducer::stalk::Invariant, v: Option<Versor>, gauge: &[(u8, u8)]| {
        let Some(v) = v else { return };
        if v.projective_eq(&one) && gauge.iter().all(|(a, b)| a == b) {
            return;
        }
        out.push(Clause { select: Pred::Chart(chart.clone()), act: Act::Carry { motion: v, about: About::Grid, gauge: gauge.to_vec() } });
    };
    for (_, v) in form.vertices.entries() {
        match v {
            Vertex::Frame | Vertex::Demand { .. } => {}
            Vertex::Word { derivation, acts, .. } => {
                for (_, m) in acts.scoped() {
                    carry(&m.chart, element::conformal_of_rotor(&derivation.rotor(), &m.carriage), &m.gauge);
                }
            }
            Vertex::Rebased { by, acts, .. } => {
                let Some(w @ Vertex::Word { derivation, .. }) = form.vertices.at(by) else { return Err(Unread::Cited) };
                let charts = w.charts();
                let [base_chart] = charts.as_slice() else { return Err(Unread::Cited) };
                let Some(base) = w.acting_on(base_chart) else { return Err(Unread::Cited) };
                for (_, m) in acts.scoped() {
                    carry(&m.chart, element::conformal_of_rotor(&derivation.rotor(), &m.carriage.geo(&base.carriage)), &m.gauge);
                }
            }
            Vertex::Glued { .. } => return Err(Unread::Cited),
            Vertex::Unfolded { .. } => return Err(Unread::Unfolded),
        }
    }
    Ok(out)
}

/// WHAT BECAME OF ONE BODY no carriage moves: kept where it stood, erased
/// (every cell ground), recoloured (every cell painted, its hues moved by
/// one map), or otherwise changed.
enum Fate {
    Kept,
    Erased,
    Recoloured(Vec<(u8, u8)>),
    Changed,
}

fn fate(b: &Body, y: &Grid) -> Fate {
    let cells = cells_of(b);
    let said: Vec<Option<u8>> = cells.iter().map(|&(r, c, _)| at(y, r, c)).collect();
    if cells.iter().zip(&said).all(|(c, s)| *s == Some(c.2)) {
        return Fate::Kept;
    }
    if said.iter().all(|s| *s == Some(0)) {
        return Fate::Erased;
    }
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (&(_, _, h), s) in cells.iter().zip(&said) {
        let Some(t) = *s else { return Fate::Changed };
        if t == 0 {
            return Fate::Changed;
        }
        match map.iter().find(|(f, _)| *f == h) {
            Some(&(_, old)) if old != t => return Fate::Changed,
            Some(_) => {}
            None => map.push((h, t)),
        }
    }
    Fate::Recoloured(map.into_iter().filter(|(a, b)| a != b).collect())
}

/// READ THE FAMILY'S ACTS as the clauses its form carries, re-deriving every
/// training pair exactly, or refused by name.
pub(crate) fn read_acts(form: &Form, family: &[(Grid, Grid)], budget: &mut crate::physics::Budget) -> Result<Vec<Clause>, Unread> {
    let carried = carriages(form)?;
    let pairs: Vec<Pair<'_>> = family.iter().map(|(x, y)| Pair { x, y, bodies: perceive(x) }).collect();
    let preds = predicates(&pairs);
    let sels = pairs.iter().map(|p| statement::select_each(&preds, p.x, &p.bodies)).collect();
    let mut r = Reading { pairs, preds, sels, budget };
    let every = r.every();
    if r.pairs.iter().any(|p| frame_of(p.x) != frame_of(p.y)) {
        return sized(&mut r, carried, &every);
    }
    // THE BODIES NO CARRIAGE MOVES, and what became of each.
    let moved_charts: Vec<&crate::transducer::stalk::Invariant> = carried
        .iter()
        .filter_map(|c| match &c.select {
            Pred::Chart(inv) => Some(inv),
            _ => None,
        })
        .collect();
    let mut still: Vec<Vec<usize>> = Vec::new();
    let mut erased: Vec<Vec<usize>> = Vec::new();
    let mut recoloured: Vec<Vec<usize>> = Vec::new();
    let mut changed: Vec<Vec<usize>> = Vec::new();
    let mut map: Vec<(u8, u8)> = Vec::new();
    for p in &r.pairs {
        let (mut s, mut e, mut rc, mut ch) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for (i, b) in p.bodies.iter().enumerate() {
            if moved_charts.contains(&&b.moduli) {
                continue;
            }
            s.push(i);
            match fate(b, p.y) {
                Fate::Kept => {}
                Fate::Erased => e.push(i),
                Fate::Recoloured(m) => {
                    for (h, t) in m {
                        match map.iter().find(|(f, _)| *f == h) {
                            Some(&(_, old)) if old != t => return Err(Unread::Inseparable),
                            Some(_) => {}
                            None => map.push((h, t)),
                        }
                    }
                    rc.push(i);
                }
                Fate::Changed => ch.push(i),
            }
        }
        still.push(s);
        erased.push(e);
        recoloured.push(rc);
        changed.push(ch);
    }
    let mut base: Vec<Clause> = Vec::new();
    // C1 — A VANISHED BODY: every hue of the palette sent to the ground, on
    // the bodies the separating coordinate selects.
    if erased.iter().any(|e| !e.is_empty()) {
        let select = r.separate(&erased, &still).ok_or(Unread::Inseparable)?;
        let ground = (1..crate::transducer::element::PALETTE).filter_map(|h| u8::try_from(h).ok()).map(|h| (h, 0)).collect();
        base.push(Clause { select, act: Act::Recolour(ground) });
    }
    if recoloured.iter().any(|e| !e.is_empty()) {
        let select = r.separate(&recoloured, &still).ok_or(Unread::Inseparable)?;
        base.push(Clause { select, act: Act::Recolour(map) });
    }
    base.extend(carried);
    if r.rederives(&base)? {
        return Ok(base);
    }
    // A seed a copy stamps over reads as erased, recoloured or changed,
    // as the template's cell at its centre is ground, a hue, or several.
    let touched: Vec<Vec<usize>> = erased
        .iter()
        .zip(&recoloured)
        .zip(&changed)
        .map(|((e, rc), ch)| {
            let mut t: Vec<usize> = e.iter().chain(rc).chain(ch).copied().collect();
            t.sort_unstable();
            t
        })
        .collect();
    created(&mut r, base, &touched, &every)
}

/// C2–C4 — CELLS CREATED: the cells the clauses so far leave unlike the
/// output, read as a copy onto the changed bodies, a fill of a region, or a
/// line along a ray.
fn created(r: &mut Reading<'_, '_>, base: Vec<Clause>, touched: &[Vec<usize>], every: &[Vec<usize>]) -> Result<Vec<Clause>, Unread> {
    let Some(canvas) = r.speak(&base)? else { return Err(Unread::Overwritten) };
    let residue: Vec<Vec<(usize, usize, u8)>> = canvas
        .iter()
        .zip(&r.pairs)
        .map(|(g, p)| {
            let mut out = Vec::new();
            for (i, row) in p.y.iter().enumerate() {
                for (j, &h) in row.iter().enumerate() {
                    if at(g, i, j) != Some(h) {
                        out.push((i, j, h));
                    }
                }
            }
            out
        })
        .collect();
    let mut created_hues: Vec<u8> = Vec::new();
    for &(_, _, h) in residue.iter().flatten() {
        insert_sorted(&mut created_hues, h);
    }
    let cleared = matches!(created_hues.as_slice(), [0, ..]);
    let attempt = |r: &mut Reading<'_, '_>, clause: Clause| -> Result<Option<Vec<Clause>>, Unread> {
        let mut all = base.clone();
        all.push(clause);
        Ok(r.rederives(&all)?.then_some(all))
    };
    // C2 — COPY: the bodies an act touched are seeds, and the one template
    // a coordinate of the list names is stamped onto each.
    if touched.iter().any(|c| !c.is_empty()) {
        if let Some(seeds) = r.separate(touched, every) {
            let templates: Vec<Pred> = (0..r.preds.len())
                .filter(|&i| r.sels.iter().all(|s| s[i].as_ref().is_some_and(|sel| sel.len() == 1)))
                .map(|i| r.preds[i].clone())
                .collect();
            for template in templates {
                if let Some(c) = attempt(r, Clause { select: seeds.clone(), act: Act::Copy { template } })? {
                    return Ok(c);
                }
            }
        }
    }
    // C4 — FILL: one hue created, in the region of the bodies whose own
    // region holds created cells.
    if let [hue] = created_hues.as_slice() {
        for region in [Region::Interior, Region::Halo, Region::Extent] {
            let holders: Vec<Vec<usize>> = r
                .pairs
                .iter()
                .zip(&residue)
                .map(|(p, res)| {
                    statement::regions_each(region, p.x, &p.bodies)
                        .iter()
                        .enumerate()
                        .filter(|(_, cells)| cells.iter().any(|&(a, b)| res.iter().any(|&(i, j, _)| (i, j) == (a, b))))
                        .map(|(b, _)| b)
                        .collect()
                })
                .collect();
            if holders.iter().all(Vec::is_empty) {
                continue;
            }
            let Some(select) = r.separate(&holders, every) else { continue };
            if let Some(c) = attempt(r, Clause { select, act: Act::Fill { region, hue: *hue } })? {
                return Ok(c);
            }
        }
    }
    // C3 — EXTEND: for each ray, the bodies one of whose cells has a created
    // cell next along it; the line painted its own hue or the one created.
    for ray in RAYS {
        let (dr, dc) = (i64::from(ray.0), i64::from(ray.1));
        let starts: Vec<Vec<usize>> = r
            .pairs
            .iter()
            .zip(&residue)
            .map(|(p, res)| {
                p.bodies
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| {
                        cells_of(b).iter().any(|&(i, j, _)| {
                            let (Ok(i), Ok(j)) = (i64::try_from(i), i64::try_from(j)) else { return false };
                            let (Ok(ni), Ok(nj)) = (usize::try_from(i + dr), usize::try_from(j + dc)) else { return false };
                            res.iter().any(|&(a, b, _)| (a, b) == (ni, nj))
                        })
                    })
                    .map(|(b, _)| b)
                    .collect()
            })
            .collect();
        if starts.iter().all(Vec::is_empty) {
            continue;
        }
        let Some(select) = r.separate(&starts, every) else { continue };
        let paints: Vec<Paint> = match created_hues.as_slice() {
            [h] => vec![Paint::Own, Paint::Hue(*h)],
            _ => vec![Paint::Own],
        };
        for paint in paints {
            for stop in [Stop::Border, Stop::Contact] {
                if let Some(c) = attempt(r, Clause { select: select.clone(), act: Act::Extend { ray, paint, stop } })? {
                    return Ok(c);
                }
            }
        }
    }
    Err(match cleared {
        true => Unread::Overwritten,
        false => Unread::Uncreated,
    })
}

/// C5 — THE FRAME RESIZED: every cell scaled by the one exact ratio the
/// frames read, or the output cropped to the extent of the one body (or
/// the one enclosure) whose extent is the output's frame.
fn sized(r: &mut Reading<'_, '_>, carried: Vec<Clause>, every: &[Vec<usize>]) -> Result<Vec<Clause>, Unread> {
    let ratio = |p: &Pair<'_>| -> Option<(u8, u8)> {
        let ((xr, xc), (yr, yc)) = (frame_of(p.x), frame_of(p.y));
        if xr == 0 || xc == 0 || yr == 0 || yc == 0 {
            return None;
        }
        if yr % xr == 0 && yc % xc == 0 && yr / xr == yc / xc {
            return Some((u8::try_from(yr / xr).ok()?, 1));
        }
        if xr % yr == 0 && xc % yc == 0 && xr / yr == xc / yc {
            return Some((1, u8::try_from(xr / yr).ok()?));
        }
        None
    };
    let ratios: Vec<Option<(u8, u8)>> = r.pairs.iter().map(ratio).collect();
    if let [Some(first), rest @ ..] = ratios.as_slice() {
        if rest.iter().all(|q| q.as_ref() == Some(first)) {
            let (up, down) = *first;
            for lead in [Vec::new(), carried.clone()] {
                let mut all = lead;
                all.push(Clause { select: Pred::All, act: Act::Scale { up, down } });
                if r.rederives(&all)? {
                    return Ok(all);
                }
            }
        }
    }
    for extent in [Extent::Body, Extent::Interior] {
        let holders: Vec<Vec<usize>> = r
            .pairs
            .iter()
            .map(|p| {
                let (yr, yc) = frame_of(p.y);
                let spans: Vec<Vec<(usize, usize)>> = match extent {
                    Extent::Body => p.bodies.iter().map(|b| cells_of(b).into_iter().map(|(i, j, _)| (i, j)).collect()).collect(),
                    Extent::Interior => statement::regions_each(Region::Interior, p.x, &p.bodies),
                };
                spans
                    .iter()
                    .enumerate()
                    .filter(|(_, cells)| {
                        let marked: Vec<(usize, usize, u8)> = cells.iter().map(|&(i, j)| (i, j, 0)).collect();
                        statement::extent_of(&marked).is_some_and(|(t, l, b, rr)| (b - t + 1, rr - l + 1) == (yr, yc))
                    })
                    .map(|(b, _)| b)
                    .collect()
            })
            .collect();
        if holders.iter().any(|h| h.len() != 1) {
            continue;
        }
        let Some(select) = r.separate(&holders, every) else { continue };
        for lead in [Vec::new(), carried.clone()] {
            let mut all = lead;
            all.push(Clause { select: select.clone(), act: Act::Crop(extent) });
            if r.rederives(&all)? {
                return Ok(all);
            }
        }
    }
    Err(Unread::Unsized)
}
