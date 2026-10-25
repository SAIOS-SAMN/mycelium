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
/// hue and its complement, its contact and its absence, each chart, the extremum of each
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
    out.extend(hues.iter().map(|&h| Pred::Not(Box::new(Pred::Touches(Box::new(Pred::Hue(h)))))));
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

/// What `sel` selects within `scope`, sorted.
fn within(sel: &[usize], scope: &[usize]) -> Vec<usize> {
    let mut w: Vec<usize> = sel.iter().copied().filter(|b| scope.contains(b)).collect();
    w.sort_unstable();
    w
}

/// THE PREDICATE ON WHICH THE TARGETS SEPARATE — `sels[pair][pred]` what
/// each predicate of the list selects there, `None` where it is unstated:
/// - the first of the list that, on every pair, selects within `scope`
///   exactly that pair's targets;
/// - else THE TARGETS' OWN DESCRIPTION, read off them (the auditor's
///   ruling, 2026-10-19: a composite derived from the difference between
///   targets and the rest, never iterated): of the predicates holding of
///   every target on every pair, each one, in the list's order, that
///   excludes something the ones before it kept — the meet of what the
///   targets share, every conjunct earning its place. It stands where it is
///   exactly the targets, read by speaking it on each pair (`select_each`:
///   a conjunction narrows in order, so a rank in it ranks the survivors).
///
/// `None` where neither separates. Bodies or parts alike: the list, its
/// selections and the pairs are the domain's own.
fn separate_on(
    preds: &[Pred],
    sels: &[Vec<Option<Vec<usize>>>],
    pairs: &[Pair<'_>],
    targets: &[Vec<usize>],
    scope: &[Vec<usize>],
) -> Option<Pred> {
    let exact = |i: usize| {
        sels.iter().zip(targets.iter().zip(scope)).all(|(sels, (t, s))| match &sels[i] {
            Some(sel) => within(sel, s) == *t,
            None => false,
        })
    };
    if let Some(i) = (0..preds.len()).find(|&i| exact(i)) {
        return Some(preds[i].clone());
    }
    let (meet, kept) = meet_on(preds, sels, targets, scope);
    if kept.as_slice() != targets || meet.len() < 2 {
        return None;
    }
    let and = Pred::And(meet);
    let spoken = pairs.iter().zip(targets.iter().zip(scope)).all(|(p, (t, s))| match statement::select_each(std::slice::from_ref(&and), p.x, &p.bodies).as_slice() {
        [Some(sel)] => within(sel, s) == *t,
        _ => false,
    });
    spoken.then_some(and)
}

/// THE MEET OF WHAT THE TARGETS SHARE: of the predicates holding of every
/// target on every pair, each one, in the list's order, that excludes what
/// the ones before kept — and what the meet keeps within `scope`.
fn meet_on(preds: &[Pred], sels: &[Vec<Option<Vec<usize>>>], targets: &[Vec<usize>], scope: &[Vec<usize>]) -> (Vec<Pred>, Vec<Vec<usize>>) {
    let covers = |i: usize| {
        sels.iter().zip(targets).all(|(sels, t)| match &sels[i] {
            Some(sel) => t.iter().all(|b| sel.contains(b)),
            None => false,
        })
    };
    let mut kept: Vec<Vec<usize>> = scope.to_vec();
    let mut meet: Vec<Pred> = Vec::new();
    for i in (0..preds.len()).filter(|&i| covers(i)) {
        // A predicate covering the targets is stated on every pair.
        let narrowed: Vec<Vec<usize>> = sels
            .iter()
            .zip(&kept)
            .map(|(sels, k)| match &sels[i] {
                Some(sel) => within(sel, k),
                None => Vec::new(),
            })
            .collect();
        if narrowed != kept {
            kept = narrowed;
            meet.push(preds[i].clone());
        }
    }
    (meet, kept)
}

/// THE TARGETS' OWN DESCRIPTION for an act that leaves untouched the bodies
/// it cannot reach (a join with no partner): the one predicate that
/// separates them, else the meet of what they share — never required to
/// exclude the bodies the act would leave as they stand; the proof decides.
fn described_on(preds: &[Pred], sels: &[Vec<Option<Vec<usize>>>], pairs: &[Pair<'_>], targets: &[Vec<usize>], scope: &[Vec<usize>]) -> Pred {
    if let Some(p) = separate_on(preds, sels, pairs, targets, scope) {
        return p;
    }
    let (mut meet, _) = meet_on(preds, sels, targets, scope);
    match meet.len() {
        0 => Pred::All,
        1 => meet.remove(0),
        _ => Pred::And(meet),
    }
}

/// THE RECOLOUR CLASSES, keyed by the exact map an effect showed (its pairs'
/// bytes, so the classes are read in the map's own order): on each pair the
/// bodies (or parts) that showed it, sorted.
type Classes = crate::network::nerve::Keyed<Vec<Vec<usize>>>;

/// The bodies (or parts) `i` of pair `k` showing `map`, filed in its class.
fn file_class(classes: &mut Classes, map: &[(u8, u8)], k: usize, i: usize, pairs: usize) {
    let key: Vec<u8> = map.iter().flat_map(|&(a, b)| [a, b]).collect();
    match classes.get_mut(&key) {
        Some(at) => insert_sorted(&mut at[k], i),
        None => {
            let mut at = vec![Vec::new(); pairs];
            at[k].push(i);
            classes.insert(&key, at);
        }
    }
}

/// The map a class is keyed by.
fn map_of_class(key: &[u8]) -> Vec<(u8, u8)> {
    key.chunks_exact(2).map(|p| (p[0], p[1])).collect()
}

/// 78.167 Phase A — THE RECOLOURS READ ON THE SCENE'S PARTS, where a body
/// changed in part: each monochrome part (`perceive_parts`) of the bodies no
/// carriage moves is kept, sent to the ground, or recoloured by one map —
/// read off the pair — and the parts that showed one map are one clause on
/// parts, selected by the coordinate that separates them from the rest.
/// `None` where a part is itself changed in part (no part reading says it),
/// or a class separates on no coordinate.
fn part_clauses(pairs: &[Pair<'_>], moved_charts: &[&crate::transducer::stalk::Invariant]) -> Option<Vec<Clause>> {
    let void = crate::arc::ground();
    let parted: Vec<Pair<'_>> = pairs.iter().map(|p| Pair { x: p.x, y: p.y, bodies: perceive_parts(p.x) }).collect();
    let preds = predicates(&parted);
    let sels: Vec<Vec<Option<Vec<usize>>>> = parted.iter().map(|p| statement::select_each(&preds, p.x, &p.bodies)).collect();
    let mut scope: Vec<Vec<usize>> = Vec::new();
    let mut classes: Classes = crate::network::nerve::Keyed::new();
    for (k, (p, whole)) in parted.iter().zip(pairs).enumerate() {
        // A part of a body a carriage moves is the carriage's.
        let moved: Vec<(usize, usize)> = whole
            .bodies
            .iter()
            .filter(|b| moved_charts.contains(&&b.moduli))
            .flat_map(|b| cells_of(b).into_iter().map(|(r, c, _)| (r, c)))
            .collect();
        let mut s = Vec::new();
        for (i, part) in p.bodies.iter().enumerate() {
            let cells = cells_of(part);
            if cells.iter().any(|&(r, c, _)| moved.contains(&(r, c))) {
                continue;
            }
            s.push(i);
            let map = match fate(part, p.y) {
                Fate::Kept => continue,
                Fate::Erased => cells.iter().map(|&(_, _, h)| h).filter(|&h| h != void).map(|h| (h, void)).take(1).collect(),
                Fate::Recoloured(map) => map,
                Fate::Changed => return None,
            };
            file_class(&mut classes, &map, k, i, parted.len());
        }
        scope.push(s);
    }
    classes
        .scoped()
        .map(|(key, at)| {
            separate_on(&preds, &sels, &parted, at, &scope).map(|select| Clause { select, act: Act::Parts(Box::new(Act::Recolour(map_of_class(&key)))) })
        })
        .collect()
}

/// WHETHER A CLAUSE AGREES WITH A PAIR'S EFFECT: spoken alone on the input,
/// every cell it changes is the output's there.
fn agrees(clause: &Clause, p: &Pair<'_>) -> bool {
    let Some(said) = statement::speak_clauses(std::slice::from_ref(clause), p.x, &p.bodies).ok().as_ref().and_then(grid_of) else { return false };
    if frame_of(&said) != frame_of(p.y) {
        return false;
    }
    said.iter().zip(p.x).zip(p.y).all(|((s, x), y)| s.iter().zip(x).zip(y).all(|((s, x), y)| s == x || s == y))
}

/// The spoken clauses laid on their frame.
fn grid_of(s: &statement::Spoken) -> Option<Grid> {
    let mut g: Grid = vec![vec![crate::arc::ground(); s.frame.1]; s.frame.0];
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

    /// THE PREDICATE ON WHICH THE TARGETS SEPARATE among the scene's bodies
    /// (`separate_on`).
    fn separate(&self, targets: &[Vec<usize>], scope: &[Vec<usize>]) -> Option<Pred> {
        separate_on(&self.preds, &self.sels, &self.pairs, targets, scope)
    }

    fn every(&self) -> Vec<Vec<usize>> {
        self.pairs.iter().map(|p| (0..p.bodies.len()).collect()).collect()
    }
}

/// THE FORM'S MOTIONS AS CARRIAGES: each word member's versor (its rotor
/// composed with its carriage, as the vertex speaks it) on the bodies of its
/// chart, with its hue map. A member that moves nothing is no motion: it
/// leaves its bodies where the canvas holds them, and what became of their
/// hues is read off the effect body by body (78.167 Phase A) — one member's
/// one map over a chart whose bodies were recoloured two ways was an
/// over-claim the effect refutes.
fn carriages(form: &Form) -> Result<Vec<Clause>, Unread> {
    let one = crate::transducer::cga::Mv::scalar(crate::core::Q::from_integer(1.into()));
    let mut out = Vec::new();
    let mut carry = |chart: &crate::transducer::stalk::Invariant, v: Option<Versor>, gauge: &[(u8, u8)]| {
        let Some(v) = v else { return };
        if v.projective_eq(&one) {
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

/// A REACTION READ OFF A CARRIAGE (ruled 2026-10-25): a carriage that is one
/// translation along a unit ray, with no hue map, whose bodies land where a
/// slide to contact lands them on every pair, is the slide — its distance
/// read off the meet, not fixed, so a pair whose distance differs is said
/// too. A fixed carriage fits each pair and fails the held-out one.
fn slide_of(c: &Clause, pairs: &[Pair<'_>]) -> Option<Clause> {
    let Act::Carry { motion, about: About::Grid, gauge } = &c.act else { return None };
    if !gauge.is_empty() {
        return None;
    }
    // The translation, read off three cells it carries: one offset for all.
    let off = |r: usize, cc: usize| -> Option<(i64, i64)> {
        let (a, b) = statement::carried(motion, r, cc).ok()?;
        Some((a.checked_sub(i64::try_from(r).ok()?)?, b.checked_sub(i64::try_from(cc).ok()?)?))
    };
    let o = off(0, 0)?;
    if off(0, 1)? != o || off(1, 0)? != o {
        return None;
    }
    // Along a row, a column or a diagonal: the unit ray and its length.
    let n = match (o.0.abs(), o.1.abs()) {
        (0, 0) => return None,
        (a, 0) => a,
        (0, b) => b,
        (a, b) if a == b => a,
        _ => return None,
    };
    let ray = (i8::try_from(o.0 / n).ok()?, i8::try_from(o.1 / n).ok()?);
    let slide = Clause { select: c.select.clone(), act: Act::Slide { ray, stop: Stop::Contact } };
    let same = pairs.iter().all(|p| {
        let carried = statement::speak_clauses(std::slice::from_ref(c), p.x, &p.bodies).ok();
        carried.is_some() && carried == statement::speak_clauses(std::slice::from_ref(&slide), p.x, &p.bodies).ok()
    });
    same.then_some(slide)
}

/// A REACTION READ OFF THE BODIES THAT LEFT THEIR PLACE (ruled 2026-10-25):
/// each found again on the output — its own cells, its own hues — at an
/// offset along a unit ray. The ONE ray every one of them is found along;
/// `None` where none is shared, or more than one (no ray is chosen).
fn slide_ray(r: &Reading<'_, '_>, gone: &[Vec<usize>]) -> Option<(i8, i8)> {
    let mut shared: Vec<(i8, i8)> = RAYS.to_vec();
    for (p, g) in r.pairs.iter().zip(gone) {
        let rows = p.y.len();
        let cols = match p.y.as_slice() {
            [row, ..] => row.len(),
            [] => 0,
        };
        let reach = i64::try_from(if rows > cols { rows } else { cols }).ok()?;
        for &b in g {
            let cells = cells_of(&p.bodies[b]);
            shared.retain(|&(dr, dc)| {
                (1..=reach).any(|k| {
                    let by = (k * i64::from(dr), k * i64::from(dc));
                    cells.iter().all(|&(i, j, h)| step(i, j, by, rows, cols).is_some_and(|(a, d)| at(p.y, a, d) == Some(h)))
                })
            });
        }
    }
    let [ray] = shared.as_slice() else { return None };
    Some(*ray)
}

/// A REACTION'S HUE READ OFF THE RECOLOURED (ruled 2026-10-25): on every
/// pair, every recoloured body painted whole the one hue the bodies it
/// touches carry (`statement::partner_hues`). A fixed map fits each pair and
/// fails a held-out pair whose partner is another hue.
fn takes_partners_hue(r: &Reading<'_, '_>, recoloured: &[Vec<usize>]) -> bool {
    if recoloured.iter().all(Vec::is_empty) {
        return false;
    }
    for (p, rc) in r.pairs.iter().zip(recoloured) {
        let partners = statement::partner_hues(p.x, &p.bodies);
        for &b in rc {
            let Some(want) = partners.get(b).copied().flatten() else { return false };
            if !cells_of(&p.bodies[b]).iter().all(|&(i, j, _)| at(p.y, i, j) == Some(want)) {
                return false;
            }
        }
    }
    true
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
    let void = crate::arc::ground();
    let cells = cells_of(b);
    let said: Vec<Option<u8>> = cells.iter().map(|&(r, c, _)| at(y, r, c)).collect();
    if cells.iter().zip(&said).all(|(c, s)| *s == Some(c.2)) {
        return Fate::Kept;
    }
    if said.iter().all(|s| *s == Some(void)) {
        return Fate::Erased;
    }
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (&(_, _, h), s) in cells.iter().zip(&said) {
        let Some(t) = *s else { return Fate::Changed };
        if t == void {
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
    let pairs: Vec<Pair<'_>> = family.iter().map(|(x, y)| Pair { x, y, bodies: perceive(x) }).collect();
    // A CARRIAGE THE EFFECT REFUTES IS NO ACT: one the form's vertices give
    // stands only where every cell it changes, spoken alone on each pair, is
    // the output's — a motion read for a growth paints what the output never
    // shows, and the effect is read instead.
    // A carriage that is a slide to contact on every pair is stated as the
    // slide (`slide_of`): the relation, where the fixed distance is a fit.
    let carried: Vec<Clause> = carriages(form)?
        .into_iter()
        .filter(|c| pairs.iter().all(|p| agrees(c, p)))
        .map(|c| match slide_of(&c, &pairs) {
            Some(slide) => slide,
            None => c,
        })
        .collect();
    let preds = predicates(&pairs);
    let sels = pairs.iter().map(|p| statement::select_each(&preds, p.x, &p.bodies)).collect();
    let mut r = Reading { pairs, preds, sels, budget };
    let every = r.every();
    if r.pairs.iter().any(|p| frame_of(p.x) != frame_of(p.y)) {
        // 78.167 Phase B, Step 4 — A NEW FRAME read off the frames: the panes
        // the input splits into, or the tiles the output is made of.
        if let Some(clause) = read_panes(&r).or_else(|| read_tiles(&r)) {
            let all = vec![clause];
            if r.rederives(&all)? {
                return Ok(all);
            }
        }
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
    let n = r.pairs.len();
    let mut still: Vec<Vec<usize>> = Vec::new();
    let mut erased: Vec<Vec<usize>> = Vec::new();
    let mut recoloured: Vec<Vec<usize>> = Vec::new();
    let mut changed: Vec<Vec<usize>> = Vec::new();
    // 78.167 Phase A — each body's map read off the pair, the bodies that
    // showed one map one class (never one map forced over all of them).
    let mut classes: Classes = crate::network::nerve::Keyed::new();
    for (k, p) in r.pairs.iter().enumerate() {
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
                    file_class(&mut classes, &m, k, i, n);
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
    // A BODY CHANGED IN PART is read on its parts: where every part of the
    // still bodies is kept, erased or recoloured whole, the recolours (and
    // erasures) are the parts' own clauses.
    let on_parts = match changed.iter().any(|c| !c.is_empty()) {
        true => part_clauses(&r.pairs, &moved_charts),
        false => None,
    };
    // THE REACTIONS FIRST (ruled 2026-10-25): where the bodies that left
    // their place slid along one ray to a meet, and where the recoloured took
    // the hue of what they touch, the relations are stated — and stand where
    // they re-derive every pair. A fixed erasure and a fixed map fit each
    // pair too, and fail the held-out one; they are read after.
    // A selection that does not separate leaves the relation unread, and the
    // fixed reading below goes on as it did.
    if on_parts.is_none() {
        let mut reacted: Vec<Clause> = Vec::new();
        let mut related = Some(false);
        if erased.iter().any(|e| !e.is_empty()) {
            related = match (r.separate(&erased, &still), slide_ray(&r, &erased)) {
                (Some(select), Some(ray)) => {
                    reacted.push(Clause { select, act: Act::Slide { ray, stop: Stop::Contact } });
                    Some(true)
                }
                (Some(select), None) => {
                    let void = crate::arc::ground();
                    let ground = (0..crate::transducer::element::PALETTE).filter_map(|h| u8::try_from(h).ok()).filter(|&h| h != void).map(|h| (h, void)).collect();
                    reacted.push(Clause { select, act: Act::Recolour(ground) });
                    related
                }
                (None, _) => None,
            };
        }
        if related.is_some() {
            related = match (takes_partners_hue(&r, &recoloured), r.separate(&recoloured, &still)) {
                (true, Some(select)) => {
                    reacted.push(Clause { select, act: Act::Repaint { paint: Paint::Of(Pred::All) } });
                    Some(true)
                }
                _ => {
                    // The recolours as their fixed maps, class by class.
                    let mut fixed = related;
                    for (key, at) in classes.scoped() {
                        match r.separate(at, &still) {
                            Some(select) => reacted.push(Clause { select, act: Act::Recolour(map_of_class(&key)) }),
                            None => fixed = None,
                        }
                    }
                    fixed
                }
            };
        }
        if related == Some(true) {
            reacted.extend(carried.iter().cloned());
            if r.rederives(&reacted)? {
                return Ok(reacted);
            }
        }
    }
    let mut base: Vec<Clause> = Vec::new();
    match on_parts {
        Some(clauses) => base.extend(clauses),
        None => {
            // C1 — A VANISHED BODY: every hue of the palette sent to the
            // ground, on the bodies the separating coordinate selects.
            if erased.iter().any(|e| !e.is_empty()) {
                let select = r.separate(&erased, &still).ok_or(Unread::Inseparable)?;
                let void = crate::arc::ground();
                let ground = (0..crate::transducer::element::PALETTE).filter_map(|h| u8::try_from(h).ok()).filter(|&h| h != void).map(|h| (h, void)).collect();
                base.push(Clause { select, act: Act::Recolour(ground) });
            }
            for (key, at) in classes.scoped() {
                let select = r.separate(at, &still).ok_or(Unread::Inseparable)?;
                base.push(Clause { select, act: Act::Recolour(map_of_class(&key)) });
            }
        }
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

/// 78.167 Phase B — CELLS CREATED: the cells the clauses so far leave unlike
/// the output (the residue), and the acts their geometry is, read off it —
/// never a candidate tried and re-derived (the auditor's rulings,
/// 2026-10-19/20) — each reading composing its clauses and the next taking
/// what is left:
/// - where the bodies an act touched are its seeds, a COPY — its template the
///   input body whose own pattern is what the output stamped on each seed
///   (`read_copy`);
/// - then ECHOES: pieces that are a body's own shape at the one offset every
///   such piece offers (`read_echoes`), then on what is left a body's shape
///   mirrored or turned, at each offset every pair showing one offers
///   (`read_turned_echoes`);
/// - then FILLS, piece by piece: a piece of one hue that a region of the
///   bodies holding it covers (`read_fills`);
/// - then JOINS: straight pieces between two bodies (`read_connects`), and
///   pieces round one corner from a body on its row to one on its column
///   (`read_bends`);
/// - then LINES on what remains, every cell on runs the effect drew from
///   bodies, each run's ray the displacement measured from the body cell it
///   leaves (`read_lines`).
///
/// The composed sentence stands where it re-derives every pair — one proof,
/// never a filter. A residue that clears a cell is no creation (OVERWRITTEN);
/// one no reading says is UNCREATED.
fn created(r: &mut Reading<'_, '_>, base: Vec<Clause>, touched: &[Vec<usize>], every: &[Vec<usize>]) -> Result<Vec<Clause>, Unread> {
    // 78.167 Phase B, Step 1 — THE RESIDUE READ PIECE BY PIECE, each reading
    // composing its clauses onto the sentence and the next reading taking
    // what is left (the auditor's ruling, 2026-10-20: one act over all the
    // created cells left 223 of 372 families unstateable by definition).
    let mut all = base;
    // A COPY onto the seeds the effect touched.
    if touched.iter().any(|c| !c.is_empty()) {
        if let Some(clauses) = read_copy(r, touched, every) {
            all.extend(clauses);
        }
    }
    let Some((_, residue)) = residue_of(r, &all)? else { return Err(Unread::Overwritten) };
    // ECHOES: pieces that are an input body's own shape at one offset, the
    // source kept.
    let (echoes, _) = read_echoes(r, &residue, every);
    all.extend(echoes);
    let Some((_, residue)) = residue_of(r, &all)? else { return Err(Unread::Overwritten) };
    // ECHOES TURNED OR MIRRORED, and at several offsets, on what the one
    // offset left.
    let (turned, _) = read_turned_echoes(r, &residue, every);
    let residue = match turned.is_empty() {
        true => residue,
        false => {
            all.extend(turned);
            let Some((_, residue)) = residue_of(r, &all)? else { return Err(Unread::Overwritten) };
            residue
        }
    };
    // FILLS, piece by piece — the stricter reading: a region, cell for
    // cell, before a line, which a region's cells each beside its boundary
    // would also read as.
    let (fills, rest) = read_fills(r, &residue, every);
    all.extend(fills);
    // CONNECTORS: straight pieces whose two ends abut two bodies.
    let (connects, rest) = read_connects(r, &rest, every);
    all.extend(connects);
    // BENDS: pieces round one corner from a body on its row to a body on
    // its column.
    let (bends, rest) = read_bends(r, &rest, every);
    all.extend(bends);
    // LINES on what no fill or connector read, on the canvas they left (a
    // line stops at what was painted before it).
    if rest.iter().any(|res| !res.is_empty()) {
        let Some((canvas, _)) = residue_of(r, &all)? else { return Err(Unread::Overwritten) };
        let Some(lines) = read_lines(r, &canvas, &rest, every) else { return Err(Unread::Uncreated) };
        all.extend(lines);
    }
    // THE ONE PROOF of the composed sentence.
    match r.rederives(&all)? {
        true => Ok(all),
        false => Err(Unread::Uncreated),
    }
}

/// THE CANVAS THE CLAUSES SPEAK on each pair, and the RESIDUE: the cells
/// left unlike the output. `None` where a clause is unstated, or the residue
/// clears a cell — no creation says that.
type Residue = (Vec<Grid>, PairCells);

/// On each pair, a set of cells with their hues.
type PairCells = Vec<Vec<(usize, usize, u8)>>;

/// One fill class: on each pair its holders, and the cells their fill
/// explains.
type FillClass = (Vec<Vec<usize>>, PairCells);

fn residue_of(r: &mut Reading<'_, '_>, clauses: &[Clause]) -> Result<Option<Residue>, Unread> {
    let Some(canvas) = r.speak(clauses)? else { return Ok(None) };
    let void = crate::arc::ground();
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
    if residue.iter().flatten().any(|&(_, _, h)| h == void) {
        return Ok(None);
    }
    Ok(Some((canvas, residue)))
}

/// The residue's pieces: its hue-blind 4-connected components.
fn pieces_of(res: &[(usize, usize, u8)]) -> Vec<Vec<(usize, usize, u8)>> {
    let mut left: Vec<(usize, usize, u8)> = res.to_vec();
    let mut out = Vec::new();
    while let Some(seed) = left.pop() {
        let mut piece = vec![seed];
        let mut grew = true;
        while grew {
            grew = false;
            let (near, far): (Vec<_>, Vec<_>) = left.iter().partition(|&&(i, j, _)| {
                piece.iter().any(|&(a, b, _)| (a == i && b.abs_diff(j) == 1) || (b == j && a.abs_diff(i) == 1))
            });
            if !near.is_empty() {
                piece.extend(near);
                left = far;
                grew = true;
            }
        }
        piece.sort_unstable();
        out.push(piece);
    }
    out
}

/// C4 — FILLS READ PIECE BY PIECE: a piece of one hue is a fill where the
/// region of the bodies whose region holds it — their interior, halo or
/// extent — covers it and lies within the created cells; the pieces of one
/// region and hue are one clause, their bodies separated by the list. What
/// no fill reads is left for the lines. A class whose bodies do not separate
/// is left for them too.
fn read_fills(r: &Reading<'_, '_>, residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    // (region, hue) → its class.
    let mut classes: crate::network::nerve::Keyed<FillClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    let regions = [(0u8, Region::Interior), (1, Region::Halo), (2, Region::Extent)];
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let mut all_cells: Vec<(usize, usize)> = res.iter().map(|&(i, j, _)| (i, j)).collect();
        all_cells.sort_unstable();
        let by_region: Vec<Vec<Vec<(usize, usize)>>> = regions.iter().map(|&(_, region)| statement::regions_each(region, p.x, &p.bodies)).collect();
        for piece in pieces_of(res) {
            let mut hues: Vec<u8> = Vec::new();
            for &(_, _, h) in &piece {
                insert_sorted(&mut hues, h);
            }
            let read = match hues.as_slice() {
                [hue] => regions.iter().zip(&by_region).find_map(|(&(tag, _), bodies_regions)| {
                    let holders: Vec<usize> = bodies_regions
                        .iter()
                        .enumerate()
                        .filter(|(_, rc)| rc.iter().any(|c| piece.iter().any(|&(i, j, _)| (i, j) == *c)))
                        .map(|(b, _)| b)
                        .collect();
                    let union: Vec<(usize, usize)> = holders.iter().flat_map(|&b| bodies_regions[b].iter().copied()).collect();
                    let covers = piece.iter().all(|&(i, j, _)| union.contains(&(i, j)));
                    let within = union.iter().all(|c| all_cells.binary_search(c).is_ok());
                    (!holders.is_empty() && covers && within).then_some(([tag, *hue], holders))
                }),
                _ => None,
            };
            match read {
                Some((key, holders)) => {
                    if classes.get(&key).is_none() {
                        classes.insert(key, (vec![Vec::new(); n], vec![Vec::new(); n]));
                    }
                    if let Some((at, cells)) = classes.get_mut(&key) {
                        for b in holders {
                            insert_sorted(&mut at[k], b);
                        }
                        cells[k].extend(piece);
                    }
                }
                None => rest[k].extend(piece),
            }
        }
    }
    let mut clauses = Vec::new();
    for (key, (holders, cells)) in classes.scoped() {
        let read = match key.as_slice() {
            [tag, hue] => regions.iter().find(|(t, _)| t == tag).map(|&(_, region)| (region, *hue)),
            _ => None,
        };
        match (read, r.separate(holders, every)) {
            (Some((region, hue)), Some(select)) => clauses.push(Clause { select, act: Act::Fill { region, hue } }),
            _ => {
                for (k, c) in cells.iter().enumerate() {
                    rest[k].extend(c.iter().copied());
                }
            }
        }
    }
    (clauses, rest)
}

/// Each body's cell owner on a pair's input grid.
fn owners(p: &Pair<'_>) -> Vec<Vec<Option<usize>>> {
    let cols = match p.x.as_slice() {
        [row, ..] => row.len(),
        [] => 0,
    };
    let mut owner = vec![vec![None; cols]; p.x.len()];
    for (b, body) in p.bodies.iter().enumerate() {
        for (i, j, _) in cells_of(body) {
            if let Some(o) = owner.get_mut(i).and_then(|row| row.get_mut(j)) {
                *o = Some(b);
            }
        }
    }
    owner
}

/// One cell `(i, j)` stepped by `(dr, dc)`, inside a grid of `rows × cols`.
fn step(i: usize, j: usize, (dr, dc): (i64, i64), rows: usize, cols: usize) -> Option<(usize, usize)> {
    let ni = usize::try_from(i64::try_from(i).ok()?.checked_add(dr)?).ok()?;
    let nj = usize::try_from(i64::try_from(j).ok()?.checked_add(dc)?).ok()?;
    (ni < rows && nj < cols).then_some((ni, nj))
}

/// WHAT THE RUNS ALONG ONE RAY SHOWED: on each pair the bodies they leave;
/// whether every run carried its body's own hue; the one hue every run
/// carried, where they all carried one; whether a run stopped at a painted
/// cell.
struct Rays {
    starts: Vec<Vec<usize>>,
    own: bool,
    hue: Option<Option<u8>>,
    contact: bool,
}

/// C3 — LINES READ OFF THEIR EFFECT. Every residue cell beside a body cell
/// measures a ray — its displacement from that cell — and the walk from the
/// body cell along it must run through residue cells to a stop: the grid's
/// border, or a cell painted before (contact). Each ray so measured is one
/// clause: the bodies its runs leave (separated), painted their own hue or
/// the one hue every run carried, stopping at contact where any run did.
/// `None` where the runs do not cover the residue, or a ray's bodies do not
/// separate, or its runs carry no one paint.
fn read_lines(r: &Reading<'_, '_>, canvas: &[Grid], residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> Option<Vec<Clause>> {
    let void = crate::arc::ground();
    let n = r.pairs.len();
    let mut rays: crate::network::nerve::Keyed<Rays> = crate::network::nerve::Keyed::new();
    for (k, ((p, res), before)) in r.pairs.iter().zip(residue).zip(canvas).enumerate() {
        let (rows, cols) = frame_of(p.x);
        let owner = owners(p);
        let created = |i: usize, j: usize| res.iter().find(|&&(a, b, _)| (a, b) == (i, j)).map(|&(_, _, h)| h);
        let mut covered: Vec<(usize, usize)> = Vec::new();
        for &(i, j, _) in res {
            for (dr, dc) in RAYS.map(|(a, b)| (i64::from(a), i64::from(b))) {
                // The body cell this residue cell stands beside, and the
                // displacement from it: the ray, measured.
                let Some((bi, bj)) = step(i, j, (-dr, -dc), rows, cols) else { continue };
                let Some(b) = owner[bi][bj] else { continue };
                let own = p.x[bi][bj];
                let (mut run, mut at_cell, mut stop) = (Vec::new(), (bi, bj), None);
                while let Some((ni, nj)) = step(at_cell.0, at_cell.1, (dr, dc), rows, cols) {
                    match created(ni, nj) {
                        Some(h) => {
                            run.push((ni, nj, h));
                            at_cell = (ni, nj);
                        }
                        None => {
                            stop = Some(before[ni][nj] != void);
                            break;
                        }
                    }
                }
                // A run to the border stops there; one before a ground cell
                // the effect left is no line.
                let contact = match stop {
                    None => false,
                    Some(true) => true,
                    Some(false) => continue,
                };
                // A LINE IS DRAWN FROM EVERY CELL OF ITS BODY: the reading
                // stands only where each of the body's cells, walked along
                // the ray to what was painted before, lands on created cells
                // alone — a ray read off one run that the rest of the body
                // would paint onto ground the effect left is no line.
                let whole = cells_of(&p.bodies[b]).into_iter().all(|(ci, cj, _)| {
                    let mut at_cell = (ci, cj);
                    while let Some((ni, nj)) = step(at_cell.0, at_cell.1, (dr, dc), rows, cols) {
                        if before[ni][nj] != void {
                            break;
                        }
                        if created(ni, nj).is_none() {
                            return false;
                        }
                        at_cell = (ni, nj);
                    }
                    true
                });
                if !whole {
                    continue;
                }
                let key = [dr.to_le_bytes()[0], dc.to_le_bytes()[0]];
                if rays.get(&key).is_none() {
                    rays.insert(key, Rays { starts: vec![Vec::new(); n], own: true, hue: None, contact: false });
                }
                let entry = rays.get_mut(&key)?;
                insert_sorted(&mut entry.starts[k], b);
                entry.contact |= contact;
                for &(a, c, h) in &run {
                    entry.own &= h == own;
                    entry.hue = match entry.hue {
                        None => Some(Some(h)),
                        Some(Some(was)) if was == h => Some(Some(h)),
                        _ => Some(None),
                    };
                    insert_sorted(&mut covered, (a, c));
                }
            }
        }
        if res.iter().any(|&(i, j, _)| !covered.contains(&(i, j))) {
            return None;
        }
    }
    if rays.is_empty() {
        return None;
    }
    rays.scoped()
        .map(|(key, rr)| {
            let [dr, dc] = <[u8; 2]>::try_from(key.as_slice()).ok()?;
            let ray = (i8::from_le_bytes([dr]), i8::from_le_bytes([dc]));
            let paint = match (rr.own, rr.hue) {
                (true, _) => Paint::Own,
                (false, Some(Some(h))) => Paint::Hue(h),
                _ => return None,
            };
            let stop = if rr.contact { Stop::Contact } else { Stop::Border };
            let select = r.separate(&rr.starts, every)?;
            Some(Clause { select, act: Act::Extend { ray, paint, stop } })
        })
        .collect()
}

/// C3′ — CONNECTORS READ OFF THEIR EFFECT (78.167 Phase B, Step 2): a piece
/// that is a straight segment along a row or a column, of one hue, whose two
/// ends abut — along its axis — cells of two different bodies, joins those
/// bodies. The pieces of one hue are one clause, `Connect` painted that hue,
/// on the bodies they join (separated by the list). What is no connector, or
/// whose class does not separate, is left for the lines.
fn read_connects(r: &Reading<'_, '_>, residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut classes: crate::network::nerve::Keyed<FillClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    // The hues some piece of which, somewhere in the family, joins nothing:
    // a connector of that hue would leave it unsaid, so the hue is the
    // lines'.
    let mut unjoined: Vec<u8> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let (rows, cols) = frame_of(p.x);
        let owner = owners(p);
        let body_at = |i: Option<usize>, j: Option<usize>| -> Option<usize> {
            let (i, j) = (i?, j?);
            (i < rows && j < cols).then(|| owner[i][j]).flatten()
        };
        for piece in pieces_of(res) {
            let mut hues: Vec<u8> = Vec::new();
            for &(_, _, h) in &piece {
                insert_sorted(&mut hues, h);
            }
            let ends = match statement::extent_of(&piece) {
                Some((r0, c0, r1, c1)) if r0 == r1 && c1 - c0 + 1 == piece.len() => {
                    Some((body_at(Some(r0), c0.checked_sub(1)), body_at(Some(r0), Some(c1 + 1))))
                }
                Some((r0, c0, r1, c1)) if c0 == c1 && r1 - r0 + 1 == piece.len() => {
                    Some((body_at(r0.checked_sub(1), Some(c0)), body_at(Some(r1 + 1), Some(c0))))
                }
                _ => None,
            };
            if let [hue] = hues.as_slice() {
                if !matches!(ends, Some((Some(a), Some(b))) if a != b) {
                    insert_sorted(&mut unjoined, *hue);
                }
            }
            match (hues.as_slice(), ends) {
                ([hue], Some((Some(a), Some(b)))) if a != b => {
                    let key = [*hue];
                    if classes.get(&key).is_none() {
                        classes.insert(key, (vec![Vec::new(); n], vec![Vec::new(); n]));
                    }
                    if let Some((at, cells)) = classes.get_mut(&key) {
                        insert_sorted(&mut at[k], a);
                        insert_sorted(&mut at[k], b);
                        cells[k].extend(piece);
                    }
                }
                _ => rest[k].extend(piece),
            }
        }
    }
    let mut clauses = Vec::new();
    for (key, (ends, cells)) in classes.scoped() {
        match key.as_slice() {
            // A body with no partner is never joined: the ends' own
            // description selects them, the inert ones beside them or not.
            [hue] if !unjoined.contains(hue) => {
                let select = described_on(&r.preds, &r.sels, &r.pairs, ends, every);
                clauses.push(Clause { select, act: Act::Connect { paint: Paint::Hue(*hue) } });
            }
            _ => {
                for (k, c) in cells.iter().enumerate() {
                    rest[k].extend(c.iter().copied());
                }
            }
        }
    }
    (clauses, rest)
}

/// A shape: its least corner, and its cells relative to it, hue-blind.
type Shape = (usize, usize, Vec<(usize, usize)>);

/// An echo offered: its offset and map as bytes, and the body offering it.
type Offer = (Vec<u8>, usize);

/// A piece the echo reader weighs: its pair, its cells, and its offers.
type Weighed = (usize, Vec<(usize, usize, u8)>, Vec<Offer>);

/// An echo read: its offset, and its map.
type EchoRead = (i64, i64, Vec<(u8, u8)>);

/// A shape read off its cells, the corner its extent's own.
fn shape_of(cells: &[(usize, usize, u8)]) -> Option<Shape> {
    let (r0, c0, _, _) = statement::extent_of(cells)?;
    let mut s: Vec<(usize, usize)> = cells.iter().map(|&(i, j, _)| (i - r0, j - c0)).collect();
    s.sort_unstable();
    Some((r0, c0, s))
}

/// C2′ — ECHOES READ OFF THEIR EFFECT (78.167 Phase B, Step 3): a piece that
/// is an input body's own shape, its hues by one map, at an offset. Each such
/// piece offers the offsets (and maps) at which some body lands on it
/// exactly; the offset every echo-shaped piece offers, where exactly one is
/// offered by all of them, is the act's — set algebra on the effect, no
/// offset swept. Its sources are the bodies landing on a piece there,
/// separated by the list. `None` read (no clauses) where no one offset is
/// offered by every echo-shaped piece.
fn read_echoes(r: &Reading<'_, '_>, residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    // Per piece: (pair, cells, the offers it makes) — an offer is the offset
    // and map as bytes, and the body making it.
    let mut pieces: Vec<Weighed> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        for piece in pieces_of(res) {
            let Some((pr, pc, ps)) = shape_of(&piece) else { continue };
            let mut offers = Vec::new();
            for (b, body) in p.bodies.iter().enumerate() {
                let cells = cells_of(body);
                let Some((br, bc, bs)) = shape_of(&cells) else { continue };
                if bs != ps {
                    continue;
                }
                // The map, cell for cell; one hue going two ways is no echo.
                let mut map: Vec<(u8, u8)> = Vec::new();
                let mut whole = true;
                for &(i, j, h) in &cells {
                    let Some(&(_, _, t)) = piece.iter().find(|&&(a, c, _)| a + br == i + pr && c + bc == j + pc) else {
                        whole = false;
                        break;
                    };
                    match map.iter().find(|(f, _)| *f == h) {
                        Some(&(_, was)) if was != t => whole = false,
                        Some(_) => {}
                        None => map.push((h, t)),
                    }
                }
                if !whole {
                    continue;
                }
                // A hue kept is no part of the map.
                map.retain(|(f, t)| f != t);
                map.sort_unstable();
                let (dr, dc) = (i64::try_from(pr).ok().zip(i64::try_from(br).ok()), i64::try_from(pc).ok().zip(i64::try_from(bc).ok()));
                let (Some((a, b0)), Some((c, d0))) = (dr, dc) else { continue };
                let mut key: Vec<u8> = Vec::new();
                key.extend_from_slice(&(a - b0).to_le_bytes());
                key.extend_from_slice(&(c - d0).to_le_bytes());
                for (f, t) in &map {
                    key.extend_from_slice(&[*f, *t]);
                }
                offers.push((key, b));
            }
            pieces.push((k, piece, offers));
        }
    }
    let mut rest: PairCells = vec![Vec::new(); n];
    // THE OFFSET EVERY ECHO-SHAPED PIECE OFFERS.
    let shaped: Vec<&Weighed> = pieces.iter().filter(|(_, _, o)| !o.is_empty()).collect();
    let mut common: Option<Vec<Vec<u8>>> = None;
    for (_, _, offers) in &shaped {
        let mine: Vec<Vec<u8>> = offers.iter().map(|(key, _)| key.clone()).collect();
        common = Some(match common {
            None => mine,
            Some(c) => c.into_iter().filter(|key| mine.contains(key)).collect(),
        });
    }
    let read = match common.as_deref() {
        Some([key]) => Some(key.clone()),
        _ => None,
    };
    let Some(key) = read else {
        for (k, piece, _) in pieces {
            rest[k].extend(piece);
        }
        return (Vec::new(), rest);
    };
    let mut sources: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (k, piece, offers) in pieces {
        let mut echoed = false;
        for (o, b) in &offers {
            if *o == key {
                insert_sorted(&mut sources[k], *b);
                echoed = true;
            }
        }
        if !echoed {
            rest[k].extend(piece);
        }
    }
    let decoded = || -> Option<EchoRead> {
        let dr = i64::from_le_bytes(key.get(0..8)?.try_into().ok()?);
        let dc = i64::from_le_bytes(key.get(8..16)?.try_into().ok()?);
        let gauge = key.get(16..)?.chunks_exact(2).map(|p| (p[0], p[1])).collect();
        Some((dr, dc, gauge))
    };
    let (Some((dr, dc, gauge)), Some(select)) = (decoded(), r.separate(&sources, every)) else {
        return (Vec::new(), rest);
    };
    let q = |v: i64| crate::core::Q::from_integer(v.into());
    let motion = crate::transducer::cga::translator_at(&[q(dr), q(dc)]);
    (vec![Clause { select, act: Act::Echo { motion, about: About::Grid, gauge } }], rest)
}

/// The square's symmetry `t` (`Act::Tile`'s codes) on a cell `(a, b)` of an
/// extent of `h` rows and `w` columns, read from its least corner: the cell
/// it lands on in the turned extent, read from that extent's least corner.
fn turned(t: u8, (a, b): (usize, usize), (h, w): (usize, usize)) -> Option<(usize, usize)> {
    let last = |n: usize, i: usize| -> Option<usize> { n.checked_sub(1)?.checked_sub(i) };
    Some(match t {
        0 => (a, b),
        1 => (a, last(w, b)?),
        2 => (last(h, a)?, b),
        3 => (last(h, a)?, last(w, b)?),
        4 => (b, last(h, a)?),
        5 => (last(w, b)?, a),
        6 => (b, a),
        7 => (last(w, b)?, last(h, a)?),
        _ => return None,
    })
}

/// THE SQUARE'S SYMMETRY `t` AS A VERSOR of `Cl(4,1)(ℚ)` about the origin of
/// the sensor's plane (`e1` the rows, `e2` the columns): a mirror is one
/// reflector, a turn the product of two — `e1 − e2` swaps the axes, and a
/// quarter turn is that swap followed by one axis reversed.
fn symmetry(t: u8) -> Option<crate::transducer::cga::Mv> {
    use crate::transducer::cga::{Mv, E1, E2};
    let one = || crate::core::Q::from_integer(1.into());
    let (e1, e2) = (Mv::blade(E1, one()), Mv::blade(E2, one()));
    let swap = e1.sub(&e2);
    Some(match t {
        0 => Mv::scalar(one()),
        1 => e2,
        2 => e1,
        3 => e1.geo(&e2),
        4 => e2.geo(&swap),
        5 => e1.geo(&swap),
        6 => swap,
        7 => e1.add(&e2),
        _ => return None,
    })
}

/// A shape's cells, relative to its least corner and sorted, as bytes.
fn shape_bytes(rel: &[(usize, usize)]) -> Vec<u8> {
    rel.iter().flat_map(|&(a, b)| a.to_le_bytes().into_iter().chain(b.to_le_bytes())).collect()
}

/// The echo an offer's bytes name: its symmetry, the displacement of the
/// centres (doubled), and its map. Itself is an echo about the grid; a
/// mirror or a turn is carried about each body's own centre.
fn echo_of(key: &[u8]) -> Option<Act> {
    let (&t, rest) = key.split_first()?;
    let dr = i64::from_le_bytes(rest.get(0..8)?.try_into().ok()?);
    let dc = i64::from_le_bytes(rest.get(8..16)?.try_into().ok()?);
    let gauge = rest.get(16..)?.chunks_exact(2).map(|p| (p[0], p[1])).collect();
    let half = |v: i64| crate::core::Q::new(v.into(), 2.into());
    let slide = crate::transducer::cga::translator_at(&[half(dr), half(dc)]);
    Some(match t {
        0 => Act::Echo { motion: slide, about: About::Grid, gauge },
        _ => Act::Echo { motion: slide.geo(&symmetry(t)?), about: About::Body, gauge },
    })
}

/// C2″ — ECHOES TURNED, AND AT SEVERAL OFFSETS (the auditor's directive,
/// 2026-10-20), read on what the one-offset echo left: a piece that is an
/// input body's own shape under one of the square's symmetries — the
/// group's first that gives it, itself before any turn — its hues by one
/// map, offers that symmetry, the displacement of the centres and the map.
/// Each body's eight shapes are filed by their bytes, so a piece's offers
/// are one keyed read. The echoes are the offers EVERY pair showing one
/// makes — several where it shows several — each on the bodies offering
/// it (separated by the list), and each standing only where, spoken alone,
/// it paints nothing the output does not hold. What no echo stated covers
/// is left for the fills, the joins and the lines.
fn read_turned_echoes(r: &Reading<'_, '_>, residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut pieces: Vec<Weighed> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let bodies: Vec<Vec<(usize, usize, u8)>> = p.bodies.iter().map(cells_of).collect();
        let mut by_shape: crate::network::nerve::Keyed<Vec<(usize, u8)>> = crate::network::nerve::Keyed::new();
        for (b, cells) in bodies.iter().enumerate() {
            let Some((t0, l0, b0, r0)) = statement::extent_of(cells) else { continue };
            let hw = (b0 - t0 + 1, r0 - l0 + 1);
            for t in 0u8..8 {
                let mut rel: Vec<(usize, usize)> = cells.iter().filter_map(|&(i, j, _)| turned(t, (i - t0, j - l0), hw)).collect();
                if rel.len() != cells.len() {
                    continue;
                }
                rel.sort_unstable();
                let key = shape_bytes(&rel);
                match by_shape.get_mut(&key) {
                    // The group's first symmetry giving the shape is the
                    // body's offer of it.
                    Some(at) => {
                        if !at.iter().any(|(o, _)| *o == b) {
                            at.push((b, t));
                        }
                    }
                    None => {
                        by_shape.insert(&key, vec![(b, t)]);
                    }
                }
            }
        }
        for piece in pieces_of(res) {
            let (Some((pr, pc, ps)), Some((pt, pl, pb, pright))) = (shape_of(&piece), statement::extent_of(&piece)) else { continue };
            let mut offers = Vec::new();
            let filed = match by_shape.get(&shape_bytes(&ps)) {
                Some(at) => at.clone(),
                None => Vec::new(),
            };
            for (b, t) in filed {
                let cells = &bodies[b];
                let Some((t0, l0, b0, r0)) = statement::extent_of(cells) else { continue };
                let hw = (b0 - t0 + 1, r0 - l0 + 1);
                // The map, cell for cell; one hue going two ways is no echo.
                let mut map: Vec<(u8, u8)> = Vec::new();
                let mut whole = true;
                for &(i, j, h) in cells {
                    let landed = turned(t, (i - t0, j - l0), hw).map(|(a, c)| (pr + a, pc + c));
                    let Some(&(_, _, to)) = landed.and_then(|at| piece.iter().find(|&&(a, c, _)| (a, c) == at)) else {
                        whole = false;
                        break;
                    };
                    match map.iter().find(|(f, _)| *f == h) {
                        Some(&(_, was)) if was != to => {
                            whole = false;
                            break;
                        }
                        Some(_) => {}
                        None => map.push((h, to)),
                    }
                }
                if !whole {
                    continue;
                }
                map.retain(|(f, to)| f != to);
                map.sort_unstable();
                // The centres' displacement, doubled to whole numbers.
                let twice = |a: usize, b: usize, c: usize, d: usize| -> Option<i64> { i64::try_from(a + b).ok()?.checked_sub(i64::try_from(c + d).ok()?) };
                let (Some(dr), Some(dc)) = (twice(pt, pb, t0, b0), twice(pl, pright, l0, r0)) else { continue };
                let mut key = vec![t];
                key.extend_from_slice(&dr.to_le_bytes());
                key.extend_from_slice(&dc.to_le_bytes());
                for (f, to) in &map {
                    key.extend_from_slice(&[*f, *to]);
                }
                offers.push((key, b));
            }
            pieces.push((k, piece, offers));
        }
    }
    // THE OFFERS EVERY PAIR SHOWING AN ECHO MAKES.
    let mut shown: Vec<Option<Vec<Vec<u8>>>> = vec![None; n];
    for (k, _, offers) in &pieces {
        for (key, _) in offers {
            let at = shown[*k].get_or_insert_with(Vec::new);
            if !at.contains(key) {
                at.push(key.clone());
            }
        }
    }
    let showing: Vec<&Vec<Vec<u8>>> = shown.iter().flatten().collect();
    let mut keys: Vec<Vec<u8>> = match showing.split_first() {
        Some((head, tail)) => head.iter().filter(|key| tail.iter().all(|o| o.contains(key))).cloned().collect(),
        None => Vec::new(),
    };
    keys.sort_unstable();
    let mut clauses = Vec::new();
    let mut kept: Vec<Vec<u8>> = Vec::new();
    for key in keys {
        let mut sources: Vec<Vec<usize>> = vec![Vec::new(); n];
        for (k, _, offers) in &pieces {
            for (o, b) in offers {
                if *o == key {
                    insert_sorted(&mut sources[*k], *b);
                }
            }
        }
        let (Some(select), Some(act)) = (r.separate(&sources, every), echo_of(&key)) else { continue };
        let clause = Clause { select, act };
        // AN ECHO THE EFFECT REFUTES IS NO ACT: spoken alone, it paints only
        // what the output holds.
        if r.pairs.iter().all(|p| agrees(&clause, p)) {
            clauses.push(clause);
            kept.push(key);
        }
    }
    // THE ECHO-SHAPED PIECES ARE READ WHOLE OR NOT AT ALL (as the one offset
    // is the one every such piece offers, and the lines cover the residue or
    // nothing): a piece offering an echo that no echo stated covers says the
    // echoes are not the act — a ray's cells, each one cell, each offer a
    // one-cell body's echo at its distance, and the offsets every pair shares
    // stop where the shortest ray does — and all is left to the readings
    // after.
    let whole = pieces.iter().all(|(_, _, offers)| offers.is_empty() || offers.iter().any(|(o, _)| kept.contains(o)));
    let mut rest: PairCells = vec![Vec::new(); n];
    for (k, piece, offers) in pieces {
        if !whole || !offers.iter().any(|(o, _)| kept.contains(o)) {
            rest[k].extend(piece);
        }
    }
    match whole {
        true => (clauses, rest),
        false => (Vec::new(), rest),
    }
}

/// A bend class: on each pair the sources its pieces leave, the targets
/// they reach, and the cells they explain.
type BendClass = (Vec<Vec<usize>>, Vec<Vec<usize>>, PairCells);

/// THE BEND A PIECE IS, where it is one: of one hue, a run along a row and a
/// run along a column meeting at a corner at an end of each; beyond the row
/// run's far end, along the row, a body standing on that row alone — the
/// source — and beyond the column run's far end, along the column, another
/// body standing on that column alone — the target. Its hue, source and
/// target.
fn bend_of(piece: &[(usize, usize, u8)], p: &Pair<'_>, owner: &[Vec<Option<usize>>]) -> Option<(u8, usize, usize)> {
    let [(_, _, hue), more @ ..] = piece else { return None };
    if more.iter().any(|c| c.2 != *hue) {
        return None;
    }
    let (rows, cols) = frame_of(p.x);
    let at = |cell: Option<(usize, usize)>| cell.filter(|&(a, b)| a < rows && b < cols).and_then(|(a, b)| owner[a][b]);
    let run = |v: &[usize]| v.windows(2).all(|w| w[1] == w[0] + 1);
    for &(cr, cc, _) in piece {
        // The piece is sorted, so each run reads in order.
        let row: Vec<usize> = piece.iter().filter(|c| c.0 == cr).map(|c| c.1).collect();
        let col: Vec<usize> = piece.iter().filter(|c| c.1 == cc).map(|c| c.0).collect();
        let ([c0, .., c1], [r0, .., r1]) = (row.as_slice(), col.as_slice()) else { continue };
        if row.len() + col.len() != piece.len() + 1 || !run(&row) || !run(&col) {
            continue;
        }
        let source = match (cc == *c1, cc == *c0) {
            (true, _) => c0.checked_sub(1).map(|c| (cr, c)),
            (_, true) => Some((cr, c1 + 1)),
            _ => continue,
        };
        let target = match (cr == *r1, cr == *r0) {
            (true, _) => r0.checked_sub(1).map(|r| (r, cc)),
            (_, true) => Some((r1 + 1, cc)),
            _ => continue,
        };
        let (Some(f), Some(t)) = (at(source), at(target)) else { continue };
        let on_row = cells_of(&p.bodies[f]).iter().all(|c| c.0 == cr);
        let on_col = cells_of(&p.bodies[t]).iter().all(|c| c.1 == cc);
        if f != t && on_row && on_col {
            return Some((*hue, f, t));
        }
    }
    None
}

/// C3″ — BENT CONNECTORS READ OFF THEIR EFFECT (the auditor's directive,
/// 2026-10-20: Manhattan connectors round an empty corner): each piece that
/// is a bend (`bend_of`) names its source and its target; the pieces of one
/// hue are one clause, `Bend` painted that hue, on the sources they leave,
/// `to` the targets they reach — each separated by the list. What is no
/// bend, or whose class does not separate, is left for the lines.
fn read_bends(r: &Reading<'_, '_>, residue: &[Vec<(usize, usize, u8)>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut classes: crate::network::nerve::Keyed<BendClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let owner = owners(p);
        for piece in pieces_of(res) {
            let Some((hue, f, t)) = bend_of(&piece, p, &owner) else {
                rest[k].extend(piece);
                continue;
            };
            let key = [hue];
            if classes.get(&key).is_none() {
                classes.insert(key, (vec![Vec::new(); n], vec![Vec::new(); n], vec![Vec::new(); n]));
            }
            if let Some((sources, targets, cells)) = classes.get_mut(&key) {
                insert_sorted(&mut sources[k], f);
                insert_sorted(&mut targets[k], t);
                cells[k].extend(piece);
            }
        }
    }
    let mut clauses = Vec::new();
    for (key, (sources, targets, cells)) in classes.scoped() {
        match (key.as_slice(), r.separate(sources, every), r.separate(targets, every)) {
            ([hue], Some(select), Some(to)) => clauses.push(Clause { select, act: Act::Bend { to, paint: Paint::Hue(*hue) } }),
            _ => {
                for (k, c) in cells.iter().enumerate() {
                    rest[k].extend(c.iter().copied());
                }
            }
        }
    }
    (clauses, rest)
}

/// A body's cells relative to its centre, doubled so every centre is a whole
/// number: the pattern a copy stamps, as bytes.
fn pattern_key(cells: &[(usize, usize, u8)]) -> Option<Vec<u8>> {
    let (t, l, b, r) = statement::extent_of(cells)?;
    let (cr, cc) = (i64::try_from(t + b).ok()?, i64::try_from(l + r).ok()?);
    let mut rel: Vec<(i64, i64, u8)> = Vec::with_capacity(cells.len());
    for &(i, j, h) in cells {
        rel.push((2 * i64::try_from(i).ok()? - cr, 2 * i64::try_from(j).ok()? - cc, h));
    }
    rel.sort_unstable();
    Some(rel.iter().flat_map(|&(a, c, h)| a.to_le_bytes().into_iter().chain(c.to_le_bytes()).chain([h])).collect())
}

/// The pattern the output stamped on a seed: the output's body standing on
/// the seed's cells, relative to the SEED's centre.
fn stamped_key(seed: &[(usize, usize, u8)], out_bodies: &[Body]) -> Option<Vec<u8>> {
    let (t, l, b, r) = statement::extent_of(seed)?;
    let over = out_bodies
        .iter()
        .map(cells_of)
        .find(|cells| cells.iter().any(|&(i, j, _)| seed.iter().any(|&(si, sj, _)| (i, j) == (si, sj))))?;
    let (cr, cc) = (i64::try_from(t + b).ok()?, i64::try_from(l + r).ok()?);
    let mut rel: Vec<(i64, i64, u8)> = Vec::with_capacity(over.len());
    for &(i, j, h) in &over {
        rel.push((2 * i64::try_from(i).ok()? - cr, 2 * i64::try_from(j).ok()? - cc, h));
    }
    rel.sort_unstable();
    Some(rel.iter().flat_map(|&(a, c, h)| a.to_le_bytes().into_iter().chain(c.to_le_bytes()).chain([h])).collect())
}

/// C2 — A COPY READ OFF ITS EFFECT: the bodies the effect touched are the
/// seeds; on each pair, what the output stamped over every seed, relative to
/// the seed's centre, is one pattern, and the template is the input body
/// whose own centred pattern is that pattern — read by its key, never a bank
/// of templates tried. The seeds and the template are each separated by the
/// list. `None` where the seeds stamp two patterns, or no body is the one
/// they stamp.
fn read_copy(r: &Reading<'_, '_>, touched: &[Vec<usize>], every: &[Vec<usize>]) -> Option<Vec<Clause>> {
    let mut templates: Vec<Vec<usize>> = Vec::with_capacity(r.pairs.len());
    for (p, seeds) in r.pairs.iter().zip(touched) {
        let out_bodies = perceive(p.y);
        let mut by_pattern: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (b, body) in p.bodies.iter().enumerate() {
            if seeds.contains(&b) {
                continue;
            }
            if let Some(key) = pattern_key(&cells_of(body)) {
                if by_pattern.get(&key).is_none() {
                    by_pattern.insert(key, b);
                }
            }
        }
        let mut template: Option<usize> = None;
        for &s in seeds {
            let key = stamped_key(&cells_of(&p.bodies[s]), &out_bodies)?;
            let &t = by_pattern.get(&key)?;
            match template {
                Some(was) if was != t => return None,
                _ => template = Some(t),
            }
        }
        templates.push(template.into_iter().collect());
    }
    let template = r.separate(&templates, every)?;
    let seeds = r.separate(touched, every)?;
    Some(vec![Clause { select: seeds, act: Act::Copy { template } }])
}

/// C6 — THE PANES READ OFF THE FRAMES AND THE CELLS (78.167 Phase B, Step 4):
/// where every pair's input is two equal panes along its rows (or its
/// columns) — a separator line of one cell between them or none, the same on
/// every pair — and the output is one pane's frame, the truth table is read
/// cell by cell: for each pair of pane cells painted or not, whether the
/// output cell is painted. `None` where an entry is unseen on every pair (no
/// table is read past what the pairs show), two pairs disagree on one, or
/// the output is painted in more than one hue.
fn read_panes(r: &Reading<'_, '_>) -> Option<Clause> {
    let void = crate::arc::ground();
    let split = |along_rows: bool| -> Option<bool> {
        let gaps: Vec<Option<bool>> = r
            .pairs
            .iter()
            .map(|p| {
                let ((hx, wx), (hy, wy)) = (frame_of(p.x), frame_of(p.y));
                let (n, m, ok) = match along_rows {
                    true => (hx, hy, wx == wy),
                    false => (wx, wy, hx == hy),
                };
                match (ok, n == 2 * m, n == 2 * m + 1) {
                    (true, true, _) => Some(false),
                    (true, _, true) => Some(true),
                    _ => None,
                }
            })
            .collect();
        match gaps.as_slice() {
            [Some(g), rest @ ..] if rest.iter().all(|x| *x == Some(*g)) => Some(*g),
            _ => None,
        }
    };
    let (along_rows, gap) = match (split(true), split(false)) {
        (Some(g), None) => (true, g),
        (None, Some(g)) => (false, g),
        _ => return None,
    };
    let mut table: [Option<bool>; 4] = [None; 4];
    let mut hue: Option<u8> = None;
    for p in &r.pairs {
        let (hy, wy) = frame_of(p.y);
        for i in 0..hy {
            for j in 0..wy {
                let (a, b) = match along_rows {
                    true => (p.x[i][j], p.x[i + hy + usize::from(gap)][j]),
                    false => (p.x[i][j], p.x[i][j + wy + usize::from(gap)]),
                };
                let bit = 2 * usize::from(a != void) + usize::from(b != void);
                let painted = p.y[i][j] != void;
                match table[bit] {
                    Some(was) if was != painted => return None,
                    _ => table[bit] = Some(painted),
                }
                if painted {
                    match hue {
                        Some(h) if h != p.y[i][j] => return None,
                        _ => hue = Some(p.y[i][j]),
                    }
                }
            }
        }
    }
    let mut bits = 0u8;
    for (i, entry) in table.iter().enumerate() {
        // An entry no pair shows is unread: no table is read past the pairs.
        if (*entry)? {
            bits |= 1 << i;
        }
    }
    Some(Clause { select: Pred::All, act: Act::Panes { along_rows, gap, table: bits, paint: hue? } })
}

/// C7 — THE TILES READ OFF THE FRAMES AND THE BLOCKS (78.167 Phase B, Step 4):
/// where every pair's output is the same whole number of the input's frames
/// along each axis, each block is read as the symmetries of the square that
/// carry the input onto it — the group's elements tested against the block,
/// never a tiling tried — and the block's symmetry is the one every pair's
/// block admits; where a symmetric input admits several on every pair, the
/// group's first in its code order (itself before any turn), which is every
/// pair's block alike. `None` where a block admits none on some pair, or the
/// pairs share none.
fn read_tiles(r: &Reading<'_, '_>) -> Option<Clause> {
    let factors: Vec<Option<(usize, usize)>> = r
        .pairs
        .iter()
        .map(|p| {
            let ((hx, wx), (hy, wy)) = (frame_of(p.x), frame_of(p.y));
            (hx > 0 && wx > 0 && hy % hx == 0 && wy % wx == 0 && (hy, wy) != (hx, wx)).then(|| (hy / hx, wy / wx))
        })
        .collect();
    let (rows, cols) = match factors.as_slice() {
        [Some(f), rest @ ..] if rest.iter().all(|x| *x == Some(*f)) => *f,
        _ => return None,
    };
    let mut turns = Vec::with_capacity(rows * cols);
    for i in 0..rows * cols {
        let (bi, bj) = (i / cols, i % cols);
        let fits = |t: u8| -> bool {
            r.pairs.iter().all(|p| {
                let (hx, wx) = frame_of(p.x);
                let block: Grid = (0..hx).map(|a| (0..wx).map(|b| p.y[bi * hx + a][bj * wx + b]).collect()).collect();
                statement::turn_of(p.x, t).is_some_and(|g| g == block)
            })
        };
        turns.push((0u8..8).find(|&t| fits(t))?);
    }
    Some(Clause {
        select: Pred::All,
        act: Act::Tile { rows: u8::try_from(rows).ok()?, cols: u8::try_from(cols).ok()?, turns },
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
