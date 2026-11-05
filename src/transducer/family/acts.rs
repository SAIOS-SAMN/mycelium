// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.167 C — THE INVERSION: THE ACT READ OFF ITS EFFECT.
//!
//! Perception names what changed between a pair's input and output — a body
//! that vanished, cells created, a body moved, the frame resized — and the
//! form's vertices say only the motions. This organ reads the ACT each effect
//! is and writes it in the rule language of 78.143 as a clause the form
//! carries: the motions as carriages, a vanished body as `Recolour(h → Ω)`
//! over the family's symbols onto its ground,
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
use crate::transducer::sentence::form::clause::{About, Act, Clause, Comparison, Coord, Extent, Order, Paint, PlaneAxis, Pred, RegionKind, Stop, Tie, Value};
use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
use crate::transducer::spelled::{PaintedSite, SectionSite};

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
    x: &'f SectionSample,
    y: &'f SectionSample,
    bodies: Vec<Body>,
}

/// THE READING over the family: its pairs, the fixed list of predicates
/// the pairs' bodies carry, and what each predicate selects on each pair.
struct PredicateTable<'f, 'b> {
    pairs: Vec<Pair<'f>>,
    preds: Vec<Pred>,
    /// `[pair][pred]`: the bodies selected, `None` where it is unstated.
    sels: Vec<Vec<Option<Vec<usize>>>>,
    budget: &'b mut crate::physics::Budget,
}

/// The coordinates a rank or a count is read at, in the list's fixed order.
const RANKED: [Coord; 12] = [
    Coord::Cells,
    Coord::Extent(PlaneAxis::First),
    Coord::Extent(PlaneAxis::Second),
    Coord::Least(PlaneAxis::First),
    Coord::Least(PlaneAxis::Second),
    Coord::Greatest(PlaneAxis::First),
    Coord::Greatest(PlaneAxis::Second),
    Coord::Hues,
    Coord::Contacts,
    Coord::Enclosed,
    Coord::Tally,
    Coord::Peers,
];

/// THE RAYS A CREATED LINE CAN RUN ALONG on the alphabet's section: every
/// neighbour of a site, generated at the section's rank by grade of contact
/// (`sentence::by_grade`: the faces first, then the corners), never a list
/// of offsets.
fn rays() -> Vec<(i8, i8)> {
    crate::transducer::sentence::on_section(crate::transducer::sentence::by_grade(crate::transducer::alphabet::ALPHABET_SAMPLE_RANK))
        .into_iter()
        .filter_map(|(a, b)| Some((i8::try_from(a).ok()?, i8::try_from(b).ok()?)))
        .collect()
}

/// A BODY'S PAINTED SITES OF THE SECTION, in its compartments' order.
fn painted_of(b: &Body) -> Vec<PaintedSite> {
    b.compartments.iter().filter_map(|l| Some(PaintedSite::new(SectionSite::of_family(&l.site)?, l.hue))).collect()
}

/// `site` STEPPED `by` along the plane, where it lands on `g`; `None` off
/// the sample, which holds no site there.
fn step(g: &SectionSample, site: SectionSite, by: [i64; ALPHABET_SAMPLE_RANK]) -> Option<SectionSite> {
    let s = site.moved(by)?;
    g.at(s).map(|_| s)
}

/// THE UNIT STEP along one axis of the plane, in the plane's order; `None`
/// for an axis the plane does not have.
fn unit_along(axis: usize) -> Option<[i64; ALPHABET_SAMPLE_RANK]> {
    let mut unit = [0i64; ALPHABET_SAMPLE_RANK];
    *unit.get_mut(axis)? = 1;
    Some(unit)
}

/// `site` WITH ITS COORDINATE along `axis` of the plane replaced by `value`.
fn with_along(site: SectionSite, axis: usize, value: i64) -> Option<SectionSite> {
    let mut c = site.coordinates();
    *c.get_mut(axis)? = value;
    Some(SectionSite::on_plane(c))
}

/// Whether two sites agree on every axis of the plane but `axis`: both
/// stand on one line along `axis`.
fn on_line(a: SectionSite, b: SectionSite, axis: usize) -> bool {
    a.coordinates().iter().zip(b.coordinates()).enumerate().all(|(k, (x, y))| k == axis || *x == y)
}

/// THE BODY OWNING EACH SITE of a pair's input, at the site's offset in
/// canonical address order (`SectionSample::offset_of`); a later body's
/// site is its own where two bodies name one.
struct Owners<'a> {
    x: &'a SectionSample,
    at: Vec<Option<usize>>,
}

impl<'a> Owners<'a> {
    fn of(p: &Pair<'a>) -> Owners<'a> {
        let mut at = vec![None; p.x.symbols().len()];
        for (b, body) in p.bodies.iter().enumerate() {
            for c in painted_of(body) {
                if let Some(o) = p.x.offset_of(c.site).and_then(|k| at.get_mut(k)) {
                    *o = Some(b);
                }
            }
        }
        Owners { x: p.x, at }
    }

    /// The body owning `site`; `None` off the sample or on no body.
    fn at(&self, site: SectionSite) -> Option<usize> {
        self.x.offset_of(site).and_then(|k| self.at.get(k)).copied().flatten()
    }
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
            let cells = painted_of(b);
            let mut own: Vec<u8> = Vec::new();
            for c in &cells {
                insert_sorted(&mut hues, c.symbol);
                insert_sorted(&mut own, c.symbol);
            }
            if !charts.contains(&b.moduli) {
                charts.push(b.moduli.clone());
            }
            // The body's span along one axis of the plane, both corners
            // included.
            let corners = statement::corners_of(&cells);
            let span = |axis: PlaneAxis| -> Option<i64> {
                let (lo, hi) = corners?;
                let i = axis.index();
                hi.coordinates().get(i)?.checked_sub(*lo.coordinates().get(i)?)?.checked_add(1)
            };
            let read = [i64::try_from(cells.len()).ok(), span(PlaneAxis::First), span(PlaneAxis::Second), i64::try_from(own.len()).ok()];
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
    for (coord, list) in [Coord::Cells, Coord::Extent(PlaneAxis::First), Coord::Extent(PlaneAxis::Second), Coord::Hues].into_iter().zip(&counts) {
        for &n in list {
            out.push(Pred::Cmp { coord, cmp: Comparison::Eq, to: Value::Integer(n) });
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
/// - else THE TARGETS' OWN DESCRIPTION, read off them (ruled
///   2026-10-19: a composite derived from the difference between
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

/// A VANISHED BODY'S MAP: every symbol of the family's own set Σ sent to
/// the ground `void` (MU-SENSING §3.3), never every slot of a fixed palette.
/// Σ is the whole family's, its posted test inputs included
/// (`SymbolSet::of_family`).
/// Written as a gauge listing is, in the symbols' key order. Refused by the
/// symbol set's name where Σ is not read (a posted test input of another
/// symbol width), never an empty map.
fn onto_ground(family: &[(SectionSample, SectionSample)], void: u8) -> Result<Vec<(u8, u8)>, crate::transducer::symbols::SymbolRefusal> {
    let symbols = crate::transducer::symbols::SymbolSet::of_family(family)?;
    let mut map: Vec<(u8, u8)> = symbols
        .symbols()
        .filter_map(|s| match s {
            [h] if *h != void => Some((*h, void)),
            _ => None,
        })
        .collect();
    map.sort_unstable();
    Ok(map)
}

/// 78.167 Phase A — THE RECOLOURS READ ON THE SCENE'S PARTS, where a body
/// changed in part: each monochrome part (`perceive_parts`) of the bodies no
/// carriage moves is kept, sent to the ground, or recoloured by one map —
/// read off the pair — and the parts that showed one map are one clause on
/// parts, selected by the coordinate that separates them from the rest.
/// `None` where a part is itself changed in part (no part reading says it),
/// or a class separates on no coordinate.
fn part_clauses(pairs: &[Pair<'_>], moved_charts: &[&crate::transducer::stalk::Invariant]) -> Option<Vec<Clause>> {
    // A part sent to the ground is recoloured onto the scene's one ground;
    // with none (a vortex, no ground) that clause is unstated.
    let void = crate::transducer::raster::void();
    let parted: Vec<Pair<'_>> = pairs.iter().map(|p| Pair { x: p.x, y: p.y, bodies: perceive_parts(p.x) }).collect();
    let preds = predicates(&parted);
    let sels: Vec<Vec<Option<Vec<usize>>>> = parted.iter().map(|p| statement::select_each(&preds, p.x, &p.bodies)).collect();
    let mut scope: Vec<Vec<usize>> = Vec::new();
    let mut classes: Classes = crate::network::nerve::Keyed::new();
    for (k, (p, whole)) in parted.iter().zip(pairs).enumerate() {
        // A part of a body a carriage moves is the carriage's.
        let moved: Vec<SectionSite> = whole
            .bodies
            .iter()
            .filter(|b| moved_charts.contains(&&b.moduli))
            .flat_map(|b| painted_of(b).into_iter().map(|c| c.site))
            .collect();
        let mut s = Vec::new();
        for (i, part) in p.bodies.iter().enumerate() {
            let cells = painted_of(part);
            if cells.iter().any(|c| moved.contains(&c.site)) {
                continue;
            }
            s.push(i);
            let map = match fate(part, p.y) {
                Fate::Kept => continue,
                Fate::Erased => {
                    let void = void.ok()?;
                    cells.iter().map(|c| c.symbol).filter(|&h| h != void).map(|h| (h, void)).take(1).collect()
                }
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
    let Some(said) = statement::speak_clauses(std::slice::from_ref(clause), p.x, &p.bodies).ok().as_ref().and_then(canvas_of) else { return false };
    if said.extents() != p.y.extents() {
        return false;
    }
    let agrees = said.sites().all(|(site, s)| Some(s) == p.x.at(site) || Some(s) == p.y.at(site));
    agrees
}

/// The spoken clauses laid on the clauses' frame, an unpainted site on the
/// scene's one ground; `None` where such a site is left and the scope reads
/// none.
fn canvas_of(s: &statement::Spoken) -> Option<SectionSample> {
    // An unpainted site holds the scene's one ground; with none, every site
    // must be painted, and the canvas's opening symbol is overwritten at
    // every site before it is read.
    let void = crate::transducer::raster::void().ok();
    let opening = match void {
        Some(v) => v,
        None => s.painted.iter().map(|p| p.symbol).next()?,
    };
    let mut g = SectionSample::filled([s.frame.0, s.frame.1], opening).ok()?;
    let mut written: Vec<SectionSite> = Vec::with_capacity(s.painted.len());
    for p in &s.painted {
        g.paint(p.site, p.symbol)?;
        written.push(p.site);
    }
    if void.is_none() {
        written.sort_unstable();
        written.dedup();
        if written.len() != g.symbols().len() {
            return None;
        }
    }
    Some(g)
}

impl PredicateTable<'_, '_> {
    /// THE CLAUSES SPOKEN ON EVERY PAIR, one read each under the stroke;
    /// `None` where a clause is unstated on some pair.
    fn speak(&mut self, clauses: &[Clause]) -> Result<Option<Vec<SectionSample>>, Unread> {
        let mut out = Vec::with_capacity(self.pairs.len());
        for p in &self.pairs {
            if !self.budget.spend() {
                return Err(Unread::Spent);
            }
            match statement::speak_clauses(clauses, p.x, &p.bodies).ok().as_ref().and_then(canvas_of) {
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
/// leaves its bodies where the canvas holds them, and what became of the
/// bodies' hues is read off the effect body by body (78.167 Phase A) — one member's
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
        out.push(Clause { select: Pred::Chart(chart.clone()), act: Act::Carry { motion: v, about: About::Sample, gauge: gauge.to_vec() } });
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
    let Act::Carry { motion, about: About::Sample, gauge } = &c.act else { return None };
    if !gauge.is_empty() {
        return None;
    }
    // The translation, read off the cells it carries at the origin and one
    // step from it along each axis of the plane: one offset for all.
    let off = |from: SectionSite| -> Option<[i64; ALPHABET_SAMPLE_RANK]> { Some(statement::carried(motion, from).ok()?.offset_from(from)?.coordinates()) };
    let origin = SectionSite::on_plane([0; ALPHABET_SAMPLE_RANK]);
    let o = off(origin)?;
    for axis in 0..ALPHABET_SAMPLE_RANK {
        if off(origin.moved(unit_along(axis)?)?)? != o {
            return None;
        }
    }
    // Along one axis of the plane or a diagonal: every coordinate that
    // moves moves one length, the unit ray's.
    let mut n: Option<i64> = None;
    for c in o.iter().map(|c| c.abs()).filter(|&c| c != 0) {
        match n {
            None => n = Some(c),
            Some(m) if m == c => {}
            Some(_) => return None,
        }
    }
    let n = n?;
    let [first, second] = o.map(|c| i8::try_from(c / n).ok());
    let ray = (first?, second?);
    let slide = Clause { select: c.select.clone(), act: Act::Slide { ray, stop: Stop::Contact } };
    let same = pairs.iter().all(|p| {
        let carried = statement::speak_clauses(std::slice::from_ref(c), p.x, &p.bodies).ok();
        carried.is_some() && carried == statement::speak_clauses(std::slice::from_ref(&slide), p.x, &p.bodies).ok()
    });
    same.then_some(slide)
}

/// A REACTION READ OFF THE BODIES THAT LEFT A PLACE (ruled 2026-10-25):
/// each found again on the output — its own cells, its own hues — at an
/// offset along a unit ray. The ONE ray every one of them is found along;
/// `None` where none is shared, or more than one (no ray is chosen).
fn slide_ray(r: &PredicateTable<'_, '_>, gone: &[Vec<usize>]) -> Option<(i8, i8)> {
    let mut shared: Vec<(i8, i8)> = rays();
    for (p, g) in r.pairs.iter().zip(gone) {
        // The farthest a body can be found: the output's greatest extent
        // along the plane.
        let mut reach = 0usize;
        for e in p.y.extents() {
            if e > reach {
                reach = e;
            }
        }
        let reach = i64::try_from(reach).ok()?;
        for &b in g {
            let cells = painted_of(&p.bodies[b]);
            shared.retain(|&(dr, dc)| {
                (1..=reach).any(|k| {
                    let by = [k * i64::from(dr), k * i64::from(dc)];
                    cells.iter().all(|c| c.site.moved(by).and_then(|s| p.y.at(s)) == Some(c.symbol))
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
fn takes_partners_hue(r: &PredicateTable<'_, '_>, recoloured: &[Vec<usize>]) -> bool {
    if recoloured.iter().all(Vec::is_empty) {
        return false;
    }
    for (p, rc) in r.pairs.iter().zip(recoloured) {
        let partners = statement::partner_hues(p.x, &p.bodies);
        for &b in rc {
            let Some(want) = partners.get(b).copied().flatten() else { return false };
            if !painted_of(&p.bodies[b]).iter().all(|c| p.y.at(c.site) == Some(want)) {
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

fn fate(b: &Body, y: &SectionSample) -> Fate {
    let ground = crate::transducer::raster::ground();
    let cells = painted_of(b);
    let said: Vec<Option<u8>> = cells.iter().map(|c| y.at(c.site)).collect();
    if cells.iter().zip(&said).all(|(c, s)| *s == Some(c.symbol)) {
        return Fate::Kept;
    }
    if said.iter().all(|s| s.is_some_and(|t| ground.holds(&t))) {
        return Fate::Erased;
    }
    let mut map: Vec<(u8, u8)> = Vec::new();
    for (h, s) in cells.iter().map(|c| c.symbol).zip(&said) {
        let Some(t) = *s else { return Fate::Changed };
        if ground.holds(&t) {
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
pub(crate) fn read_acts(form: &Form, family: &[(SectionSample, SectionSample)], budget: &mut crate::physics::Budget) -> Result<Vec<Clause>, Unread> {
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
    let mut r = PredicateTable { pairs, preds, sels, budget };
    let every = r.every();
    if r.pairs.iter().any(|p| p.x.extents() != p.y.extents()) {
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
    // a place slid along one ray to a meet, and where the recoloured took
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
                    // Erased onto the scene's one ground; with none, the
                    // clearing is stated by no act (`Overwritten`).
                    // A family whose Σ is refused states no erasure either.
                    let void = crate::transducer::raster::void().map_err(|_| Unread::Overwritten)?;
                    reacted.push(Clause { select, act: Act::Recolour(onto_ground(family, void).map_err(|_| Unread::Overwritten)?) });
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
                    // The recolours as fixed maps, class by class.
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
            // C1 — A VANISHED BODY: every symbol of the family's set sent to
            // the ground, on the bodies the separating coordinate selects.
            if erased.iter().any(|e| !e.is_empty()) {
                let select = r.separate(&erased, &still).ok_or(Unread::Inseparable)?;
                // Erased onto the scene's one ground; with none, the
                // clearing is stated by no act (`Overwritten`).
                // A family whose Σ is refused states no erasure either.
                let void = crate::transducer::raster::void().map_err(|_| Unread::Overwritten)?;
                base.push(Clause { select, act: Act::Recolour(onto_ground(family, void).map_err(|_| Unread::Overwritten)?) });
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
/// the output (the residue), and the acts the cells' geometry is, read off it —
/// never a candidate tried and re-derived (ruled
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
///   pieces round one corner from a body on a line along the plane's second
///   axis to one on a line along its first (`read_bends`);
/// - then LINES on what remains, every cell on runs the effect drew from
///   bodies, each run's ray the displacement measured from the body cell it
///   leaves (`read_lines`).
///
/// The composed sentence stands where it re-derives every pair — one proof,
/// never a filter. A residue that clears a cell is no creation (OVERWRITTEN);
/// one no reading says is UNCREATED.
fn created(r: &mut PredicateTable<'_, '_>, base: Vec<Clause>, touched: &[Vec<usize>], every: &[Vec<usize>]) -> Result<Vec<Clause>, Unread> {
    // 78.167 Phase B, Step 1 — THE RESIDUE READ PIECE BY PIECE, each reading
    // composing its clauses onto the sentence and the next reading taking
    // what is left (ruled 2026-10-20: one act over all the
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
    // BENDS: pieces round one corner from a body on a line along the
    // plane's second axis to a body on a line along its first.
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
type Residue = (Vec<SectionSample>, PairCells);

/// On each pair, a set of painted sites of the section.
type PairCells = Vec<Vec<PaintedSite>>;

/// One fill class: on each pair its holders, and the cells the holders'
/// fill explains.
type FillClass = (Vec<Vec<usize>>, PairCells);

fn residue_of(r: &mut PredicateTable<'_, '_>, clauses: &[Clause]) -> Result<Option<Residue>, Unread> {
    let Some(canvas) = r.speak(clauses)? else { return Ok(None) };
    let ground = crate::transducer::raster::ground();
    let residue: PairCells = canvas
        .iter()
        .zip(&r.pairs)
        .map(|(g, p)| p.y.sites().filter(|&(site, h)| g.at(site) != Some(h)).map(|(site, h)| PaintedSite::new(site, h)).collect())
        .collect();
    if residue.iter().flatten().any(|c| ground.holds(&c.symbol)) {
        return Ok(None);
    }
    Ok(Some((canvas, residue)))
}

/// THE RESIDUE'S PIECES: its symbol-blind components connected through a
/// shared face, the contact of grade `n − 1` generated at the section's
/// rank (`sentence::faces_of`), never a list of offsets.
fn pieces_of(res: &[PaintedSite]) -> Vec<Vec<PaintedSite>> {
    let faces: Vec<Vec<i64>> = crate::transducer::sentence::faces_of(ALPHABET_SAMPLE_RANK);
    let touch = |a: SectionSite, b: SectionSite| b.offset_from(a).is_some_and(|d| faces.iter().any(|f| f.as_slice() == d.coordinates().as_slice()));
    let mut left: Vec<PaintedSite> = res.to_vec();
    let mut out = Vec::new();
    while let Some(seed) = left.pop() {
        let mut piece = vec![seed];
        let mut grew = true;
        while grew {
            grew = false;
            let (near, far): (Vec<_>, Vec<_>) = left.iter().partition(|c| piece.iter().any(|q| touch(q.site, c.site)));
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
/// region of the bodies whose region holds it — the bodies' interior, halo or
/// extent — covers it and lies within the created cells; the pieces of one
/// region and hue are one clause, the pieces' bodies separated by the list. What
/// no fill reads is left for the lines. A class whose bodies do not separate
/// is left for them too.
fn read_fills(r: &PredicateTable<'_, '_>, residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    // (region, hue) → its class.
    let mut classes: crate::network::nerve::Keyed<FillClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    let regions = [(0u8, RegionKind::Interior), (1, RegionKind::Halo), (2, RegionKind::Extent)];
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        // The residue's sites of the section.
        let sites_of = |cells: &[PaintedSite]| -> Vec<SectionSite> { cells.iter().map(|c| c.site).collect() };
        let mut all_cells: Vec<SectionSite> = sites_of(res);
        all_cells.sort_unstable();
        let by_region: Vec<Vec<Vec<SectionSite>>> = regions.iter().map(|&(_, region)| statement::regions_each(region, p.x, &p.bodies)).collect();
        for piece in pieces_of(res) {
            let mut hues: Vec<u8> = Vec::new();
            for c in &piece {
                insert_sorted(&mut hues, c.symbol);
            }
            let piece_sites = sites_of(&piece);
            let read = match hues.as_slice() {
                [hue] => regions
                    .iter()
                    .zip(&by_region)
                    .find_map(|(&(tag, _), bodies_regions)| {
                        let holders: Vec<usize> = bodies_regions
                            .iter()
                            .enumerate()
                            .filter(|(_, rc)| rc.iter().any(|c| piece_sites.contains(c)))
                            .map(|(b, _)| b)
                            .collect();
                        let union: Vec<SectionSite> = holders.iter().flat_map(|&b| bodies_regions[b].iter().copied()).collect();
                        let covers = piece_sites.iter().all(|s| union.contains(s));
                        let within = union.iter().all(|c| all_cells.binary_search(c).is_ok());
                        (!holders.is_empty() && covers && within).then_some(([tag, *hue], holders))
                    })
                    // AN INTERIOR SEVERAL BODIES WALL IN TOGETHER (read on the
                    // corpus 2026-10-26): a fill speaks the interior of its
                    // whole selection, and a hole only two bodies close was
                    // never read, each body's own interior measured alone. The
                    // bodies around the piece, together, where the bodies' joint
                    // interior covers it and lies within the created cells.
                    .or_else(|| {
                        let holders = statement::bodies_around(p.x, &p.bodies, &piece_sites);
                        if holders.len() < 2 {
                            return None;
                        }
                        let union = statement::region_of_set(RegionKind::Interior, p.x, &p.bodies, &holders);
                        let covers = piece_sites.iter().all(|s| union.contains(s));
                        let within = union.iter().all(|c| all_cells.binary_search(c).is_ok());
                        (covers && within).then_some(([0, *hue], holders))
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

/// C3 — LINES READ OFF THE EFFECT. Every residue cell beside a body cell
/// measures a ray — its displacement from that cell — and the walk from the
/// body cell along it must run through residue cells to a stop: the sample's
/// border, or a cell painted before (contact). Each ray so measured is one
/// clause: the bodies its runs leave (separated), each painted its own hue or
/// the one hue every run carried, stopping at contact where any run did.
/// `None` where the runs do not cover the residue, or a ray's bodies do not
/// separate, or its runs carry no one paint.
fn read_lines(r: &PredicateTable<'_, '_>, canvas: &[SectionSample], residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> Option<Vec<Clause>> {
    let ground = crate::transducer::raster::ground();
    let n = r.pairs.len();
    let unit_rays: Vec<[i64; ALPHABET_SAMPLE_RANK]> = rays().into_iter().map(|(a, b)| [i64::from(a), i64::from(b)]).collect();
    let mut rays: crate::network::nerve::Keyed<Rays> = crate::network::nerve::Keyed::new();
    for (k, ((p, res), before)) in r.pairs.iter().zip(residue).zip(canvas).enumerate() {
        let owner = Owners::of(p);
        let created = |s: SectionSite| res.iter().find(|c| c.site == s).map(|c| c.symbol);
        let mut covered: Vec<SectionSite> = Vec::new();
        for cell in res {
            for &ray in &unit_rays {
                // The body cell this residue cell stands beside, and the
                // displacement from it: the ray, measured.
                let Some(from) = step(p.x, cell.site, ray.map(|d| -d)) else { continue };
                let Some(b) = owner.at(from) else { continue };
                let Some(own) = p.x.at(from) else { continue };
                let (mut run, mut at_cell, mut stop): (Vec<PaintedSite>, SectionSite, Option<bool>) = (Vec::new(), from, None);
                while let Some(next) = step(p.x, at_cell, ray) {
                    match created(next) {
                        Some(h) => {
                            run.push(PaintedSite::new(next, h));
                            at_cell = next;
                        }
                        None => {
                            stop = before.at(next).map(|h| !ground.holds(&h));
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
                let whole = painted_of(&p.bodies[b]).into_iter().all(|c| {
                    let mut at_cell = c.site;
                    while let Some(next) = step(p.x, at_cell, ray) {
                        if before.at(next).is_none_or(|h| !ground.holds(&h)) {
                            break;
                        }
                        if created(next).is_none() {
                            return false;
                        }
                        at_cell = next;
                    }
                    true
                });
                if !whole {
                    continue;
                }
                // The ray's key: each step along the plane, one byte.
                let key = ray.map(|d| d.to_le_bytes()[0]);
                if rays.get(&key).is_none() {
                    rays.insert(key, Rays { starts: vec![Vec::new(); n], own: true, hue: None, contact: false });
                }
                let entry = rays.get_mut(&key)?;
                insert_sorted(&mut entry.starts[k], b);
                entry.contact |= contact;
                for c in &run {
                    entry.own &= c.symbol == own;
                    entry.hue = match entry.hue {
                        None => Some(Some(c.symbol)),
                        Some(Some(was)) if was == c.symbol => Some(Some(c.symbol)),
                        _ => Some(None),
                    };
                    insert_sorted(&mut covered, c.site);
                }
            }
        }
        if res.iter().any(|c| !covered.contains(&c.site)) {
            return None;
        }
    }
    if rays.is_empty() {
        return None;
    }
    rays.scoped()
        .map(|(key, rr)| {
            let [first, second] = <[u8; ALPHABET_SAMPLE_RANK]>::try_from(key.as_slice()).ok()?.map(|d| i8::from_le_bytes([d]));
            let ray = (first, second);
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

/// THE ENDS OF A STRAIGHT PIECE between its corners `lo` and `hi`: where they
/// agree on every axis of the plane but one and the piece spans `len` sites
/// along that one, the site before `lo` and the site past `hi` along it, the
/// plane's last axis read first; `None` for a piece that is no segment. An
/// end off `i64` is `None` within the pair.
fn ends_of(lo: SectionSite, hi: SectionSite, len: usize) -> Option<(Option<SectionSite>, Option<SectionSite>)> {
    for axis in (0..ALPHABET_SAMPLE_RANK).rev() {
        let span = hi.coordinates().get(axis)?.checked_sub(*lo.coordinates().get(axis)?)?.checked_add(1)?;
        if on_line(lo, hi, axis) && usize::try_from(span).ok() == Some(len) {
            let unit = unit_along(axis)?;
            return Some((lo.moved(unit.map(|u| -u)), hi.moved(unit)));
        }
    }
    None
}

/// C3′ — CONNECTORS READ OFF THE EFFECT (78.167 Phase B, Step 2): a piece
/// that is a straight segment along one axis of the plane, of one hue, whose two
/// ends abut — along its axis — cells of two different bodies, joins those
/// bodies. The pieces of one hue are one clause, `Connect` painted that hue,
/// on the bodies they join (separated by the list). What is no connector, or
/// whose class does not separate, is left for the lines.
fn read_connects(r: &PredicateTable<'_, '_>, residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut classes: crate::network::nerve::Keyed<FillClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    // The hues some piece of which, somewhere in the family, joins nothing:
    // a connector of that hue would leave it unsaid, so the hue is the
    // lines'.
    let mut unjoined: Vec<u8> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let owner = Owners::of(p);
        for piece in pieces_of(res) {
            let mut hues: Vec<u8> = Vec::new();
            for c in &piece {
                insert_sorted(&mut hues, c.symbol);
            }
            let ends = statement::corners_of(&piece)
                .and_then(|(lo, hi)| ends_of(lo, hi, piece.len()))
                .map(|(before, past)| (before.and_then(|s| owner.at(s)), past.and_then(|s| owner.at(s))));
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

/// A shape: its least corner, and its sites read from it, symbol-blind.
type Shape = (SectionSite, Vec<SectionSite>);

/// An echo offered: its offset and map as bytes, and the body offering it.
type Offer = (Vec<u8>, usize);

/// A piece the echo reader weighs: its pair, its cells, and its offers.
type Weighed = (usize, Vec<PaintedSite>, Vec<Offer>);

/// A shape read off its painted sites, the corner their least one.
fn shape_of(cells: &[PaintedSite]) -> Option<Shape> {
    let (least, _) = statement::corners_of(cells)?;
    let mut s: Vec<SectionSite> = cells.iter().map(|c| c.site.offset_from(least)).collect::<Option<_>>()?;
    s.sort_unstable();
    Some((least, s))
}

/// C2′ — ECHOES READ OFF THE EFFECT (78.167 Phase B, Step 3): a piece that
/// is an input body's own shape, its hues by one map, at an offset. Each such
/// piece offers the offsets (and maps) at which some body lands on it
/// exactly; the offset every echo-shaped piece offers, where exactly one is
/// offered by all of them, is the act's — set algebra on the effect, no
/// offset swept. Its sources are the bodies landing on a piece there,
/// separated by the list. `None` read (no clauses) where no one offset is
/// offered by every echo-shaped piece.
fn read_echoes(r: &PredicateTable<'_, '_>, residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    // Per piece: (pair, cells, the offers it makes) — an offer is the offset
    // and map as bytes, and the body making it.
    let mut pieces: Vec<Weighed> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        for piece in pieces_of(res) {
            let Some((piece_corner, ps)) = shape_of(&piece) else { continue };
            let mut offers = Vec::new();
            for (b, body) in p.bodies.iter().enumerate() {
                let cells = painted_of(body);
                let Some((body_corner, bs)) = shape_of(&cells) else { continue };
                if bs != ps {
                    continue;
                }
                // The offset from the body's corner to the piece's.
                let Some(offset) = piece_corner.offset_from(body_corner) else { continue };
                // The map, cell for cell; one hue going two ways is no echo.
                let mut map: Vec<(u8, u8)> = Vec::new();
                let mut whole = true;
                for c in &cells {
                    let landed = c.site.moved(offset.coordinates());
                    let Some(t) = piece.iter().find(|q| Some(q.site) == landed).map(|q| q.symbol) else {
                        whole = false;
                        break;
                    };
                    match map.iter().find(|(f, _)| *f == c.symbol) {
                        Some(&(_, was)) if was != t => whole = false,
                        Some(_) => {}
                        None => map.push((c.symbol, t)),
                    }
                }
                if !whole {
                    continue;
                }
                // A hue kept is no part of the map.
                map.retain(|(f, t)| f != t);
                map.sort_unstable();
                // The offer's key: the offset along the plane
                // (`SectionSite::put_canonical`), then the map.
                let mut key: Vec<u8> = Vec::new();
                offset.put_canonical(&mut key);
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
        let offered: Vec<Vec<u8>> = offers.iter().map(|(key, _)| key.clone()).collect();
        common = Some(match common {
            None => offered,
            Some(c) => c.into_iter().filter(|key| offered.contains(key)).collect(),
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
    let decoded = || -> Option<(SectionSite, Vec<(u8, u8)>)> {
        let mut at = 0usize;
        let offset = SectionSite::take_canonical(&key, &mut at)?;
        let gauge = key.get(at..)?.chunks_exact(2).map(|p| (p[0], p[1])).collect();
        Some((offset, gauge))
    };
    let (Some((offset, gauge)), Some(select)) = (decoded(), r.separate(&sources, every)) else {
        return (Vec::new(), rest);
    };
    let q = |v: i64| crate::core::Q::from_integer(v.into());
    let motion = crate::transducer::cga::translator_at(&offset.coordinates().map(q));
    (vec![Clause { select, act: Act::Echo { motion, about: About::Sample, gauge } }], rest)
}

/// The square's symmetry `t` (`Act::Tile`'s codes) on a site read from the
/// least corner of a span of `extents` along the plane: the site it lands on
/// in the turned span, read from that span's least corner. The square's
/// symmetries are the plane's by their mathematics: the coordinates are read
/// along the plane's first and second axes (`PlaneAxis`), never as rows and
/// columns. `None` for a code past 7, or a site off the span.
fn turned(t: u8, site: SectionSite, extents: [usize; ALPHABET_SAMPLE_RANK]) -> Option<SectionSite> {
    let [a, b] = site.coordinates();
    let [first, second] = extents;
    // The coordinate `i` read from the far end of an axis of extent `n`.
    let last = |n: usize, i: i64| -> Option<i64> { i64::try_from(n).ok()?.checked_sub(1)?.checked_sub(i).filter(|v| *v >= 0) };
    if a < 0 || b < 0 {
        return None;
    }
    let landed = match t {
        0 => [a, b],
        1 => [a, last(second, b)?],
        2 => [last(first, a)?, b],
        3 => [last(first, a)?, last(second, b)?],
        4 => [b, last(first, a)?],
        5 => [last(second, b)?, a],
        6 => [b, a],
        7 => [last(second, b)?, last(first, a)?],
        _ => return None,
    };
    Some(SectionSite::on_plane(landed))
}

/// THE SQUARE'S SYMMETRY `t` AS A VERSOR of `Cl(4,1)(ℚ)` about the origin of
/// the sensor's plane (`e1` along the plane's first axis, `e2` along its
/// second): a mirror is one reflector, a turn the product of two — `e1 − e2`
/// swaps the axes, and a 90° turn is that swap followed by one axis reversed.
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

/// A shape's sites, read from its least corner and sorted, as bytes: each
/// site written along the plane (`SectionSite::put_canonical`).
fn shape_bytes(rel: &[SectionSite]) -> Vec<u8> {
    let mut out = Vec::with_capacity(rel.len() * ALPHABET_SAMPLE_RANK * std::mem::size_of::<i64>());
    for s in rel {
        s.put_canonical(&mut out);
    }
    out
}

/// The echo an offer's bytes name: its symmetry, the displacement of the
/// centres (doubled), and its map. Itself is an echo about the sample; a
/// mirror or a turn is carried about each body's own centre.
fn echo_of(key: &[u8]) -> Option<Act> {
    let (&t, rest) = key.split_first()?;
    let mut at = 0usize;
    let doubled = SectionSite::take_canonical(rest, &mut at)?;
    let gauge = rest.get(at..)?.chunks_exact(2).map(|p| (p[0], p[1])).collect();
    let half = |v: i64| crate::core::Q::new(v.into(), 2.into());
    let slide = crate::transducer::cga::translator_at(&doubled.coordinates().map(half));
    Some(match t {
        0 => Act::Echo { motion: slide, about: About::Sample, gauge },
        _ => Act::Echo { motion: slide.geo(&symmetry(t)?), about: About::Body, gauge },
    })
}

/// THE CENTRES' DISPLACEMENT, DOUBLED to whole numbers: along each axis of
/// the plane, the sum of `to`'s corners less the sum of `from`'s; `None`
/// where a coordinate leaves `i64`.
fn doubled_displacement(to: (SectionSite, SectionSite), from: (SectionSite, SectionSite)) -> Option<SectionSite> {
    let sum = |(lo, hi): (SectionSite, SectionSite), k: usize| -> Option<i64> { lo.coordinates().get(k)?.checked_add(*hi.coordinates().get(k)?) };
    let mut out = [0i64; ALPHABET_SAMPLE_RANK];
    for (k, d) in out.iter_mut().enumerate() {
        *d = sum(to, k)?.checked_sub(sum(from, k)?)?;
    }
    Some(SectionSite::on_plane(out))
}

/// C2″ — ECHOES TURNED, AND AT SEVERAL OFFSETS (ruled
/// 2026-10-20), read on what the one-offset echo left: a piece that is an
/// input body's own shape under one of the square's symmetries — the
/// group's first that gives it, itself before any turn — its hues by one
/// map, offers that symmetry, the displacement of the centres and the map.
/// Each body's eight shapes are filed by the shapes' bytes, so a piece's offers
/// are one keyed read. The echoes are the offers EVERY pair showing one
/// makes — several where it shows several — each on the bodies offering
/// it (separated by the list), and each standing only where, spoken alone,
/// it paints nothing the output does not hold. What no echo stated covers
/// is left for the fills, the joins and the lines.
fn read_turned_echoes(r: &PredicateTable<'_, '_>, residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut pieces: Vec<Weighed> = Vec::new();
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let bodies: Vec<Vec<PaintedSite>> = p.bodies.iter().map(painted_of).collect();
        let mut by_shape: crate::network::nerve::Keyed<Vec<(usize, u8)>> = crate::network::nerve::Keyed::new();
        for (b, cells) in bodies.iter().enumerate() {
            let Some((least, extents)) = SectionSite::span_of(cells.iter().map(|c| c.site)) else { continue };
            for t in 0u8..8 {
                let mut rel: Vec<SectionSite> = cells.iter().filter_map(|c| turned(t, c.site.offset_from(least)?, extents)).collect();
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
            let (Some((piece_corner, ps)), Some((piece_lo, piece_hi))) = (shape_of(&piece), statement::corners_of(&piece)) else { continue };
            let mut offers = Vec::new();
            let filed = match by_shape.get(&shape_bytes(&ps)) {
                Some(at) => at.clone(),
                None => Vec::new(),
            };
            for (b, t) in filed {
                let cells = &bodies[b];
                let (Some((least, extents)), Some((body_lo, body_hi))) = (SectionSite::span_of(cells.iter().map(|c| c.site)), statement::corners_of(cells)) else { continue };
                // The map, cell for cell; one hue going two ways is no echo.
                let mut map: Vec<(u8, u8)> = Vec::new();
                let mut whole = true;
                for c in cells {
                    let landed = c.site.offset_from(least).and_then(|s| turned(t, s, extents)).and_then(|s| s.moved(piece_corner.coordinates()));
                    let Some(to) = piece.iter().find(|q| Some(q.site) == landed).map(|q| q.symbol) else {
                        whole = false;
                        break;
                    };
                    match map.iter().find(|(f, _)| *f == c.symbol) {
                        Some(&(_, was)) if was != to => {
                            whole = false;
                            break;
                        }
                        Some(_) => {}
                        None => map.push((c.symbol, to)),
                    }
                }
                if !whole {
                    continue;
                }
                map.retain(|(f, to)| f != to);
                map.sort_unstable();
                let Some(doubled) = doubled_displacement((piece_lo, piece_hi), (body_lo, body_hi)) else { continue };
                let mut key = vec![t];
                doubled.put_canonical(&mut key);
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

/// THE BEND A PIECE IS, where it is one (the act's plane: the bend leaves its
/// source along the plane's second axis and reaches its target along the
/// first, `PlaneAxis`): of one hue, a run along the second axis and a run
/// along the first meeting at a corner at an end of each; beyond the first
/// run's far end, along the second axis, a body standing on that line alone
/// — the source — and beyond the second run's far end, along the first axis,
/// another body standing on that line alone — the target. Its hue, source and
/// target.
fn bend_of(piece: &[PaintedSite], p: &Pair<'_>, owner: &Owners<'_>) -> Option<(u8, usize, usize)> {
    let [first_cell, more @ ..] = piece else { return None };
    let hue = first_cell.symbol;
    if more.iter().any(|c| c.symbol != hue) {
        return None;
    }
    let (leave, reach) = (PlaneAxis::Second.index(), PlaneAxis::First.index());
    let run = |v: &[i64]| v.windows(2).all(|w| w[0].checked_add(1) == Some(w[1]));
    // The coordinates along `axis` of the piece's sites on the line along
    // `axis` through `corner`: the piece is sorted, so each run reads in
    // order.
    let line = |corner: SectionSite, axis: usize| -> Vec<i64> {
        piece.iter().filter(|c| on_line(c.site, corner, axis)).filter_map(|c| c.site.coordinates().get(axis).copied()).collect()
    };
    // The end of a run past its far end from the corner's coordinate `at`:
    // the site before the run where the corner stands at its last, the site
    // past it where the corner stands at its first.
    let beyond = |corner: SectionSite, axis: usize, v: &[i64]| -> Option<Option<SectionSite>> {
        let ([low, .., high], Some(&at)) = (v, corner.coordinates().get(axis)) else { return None };
        match (at == *high, at == *low) {
            (true, _) => Some(low.checked_sub(1).and_then(|c| with_along(corner, axis, c))),
            (_, true) => Some(high.checked_add(1).and_then(|c| with_along(corner, axis, c))),
            _ => None,
        }
    };
    for corner in piece.iter().map(|c| c.site) {
        let (leaving, reaching) = (line(corner, leave), line(corner, reach));
        if leaving.len() < 2 || reaching.len() < 2 || leaving.len() + reaching.len() != piece.len() + 1 || !run(&leaving) || !run(&reaching) {
            continue;
        }
        let (Some(source), Some(target)) = (beyond(corner, leave, &leaving), beyond(corner, reach, &reaching)) else { continue };
        let (Some(f), Some(t)) = (source.and_then(|s| owner.at(s)), target.and_then(|s| owner.at(s))) else { continue };
        let on_leaving = painted_of(&p.bodies[f]).iter().all(|c| on_line(c.site, corner, leave));
        let on_reaching = painted_of(&p.bodies[t]).iter().all(|c| on_line(c.site, corner, reach));
        if f != t && on_leaving && on_reaching {
            return Some((hue, f, t));
        }
    }
    None
}

/// C3″ — BENT CONNECTORS READ OFF THE EFFECT (ruled
/// 2026-10-20: Manhattan connectors round an empty corner): each piece that
/// is a bend (`bend_of`) names its source and its target; the pieces of one
/// hue are one clause, `Bend` painted that hue, on the sources they leave,
/// `to` the targets they reach — each separated by the list. What is no
/// bend, or whose class does not separate, is left for the lines.
fn read_bends(r: &PredicateTable<'_, '_>, residue: &[Vec<PaintedSite>], every: &[Vec<usize>]) -> (Vec<Clause>, PairCells) {
    let n = r.pairs.len();
    let mut classes: crate::network::nerve::Keyed<BendClass> = crate::network::nerve::Keyed::new();
    let mut rest: PairCells = vec![Vec::new(); n];
    for (k, (p, res)) in r.pairs.iter().zip(residue).enumerate() {
        let owner = Owners::of(p);
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

/// `cells` READ FROM THE CENTRE OF `around`'s corners, doubled so every
/// centre is a whole number, sorted, as bytes: each site written along the
/// plane (`SectionSite::put_canonical`), then its symbol.
fn centred_key(cells: &[PaintedSite], around: &[PaintedSite]) -> Option<Vec<u8>> {
    let (least, greatest) = statement::corners_of(around)?;
    let mut centre = least.coordinates();
    for (c, g) in centre.iter_mut().zip(greatest.coordinates()) {
        *c = c.checked_add(g)?;
    }
    let mut rel: Vec<PaintedSite> = Vec::with_capacity(cells.len());
    for p in cells {
        let mut doubled = p.site.coordinates();
        for (x, c) in doubled.iter_mut().zip(centre) {
            *x = x.checked_mul(2)?.checked_sub(c)?;
        }
        rel.push(PaintedSite::new(SectionSite::on_plane(doubled), p.symbol));
    }
    rel.sort_unstable();
    let mut out = Vec::new();
    for p in rel {
        p.site.put_canonical(&mut out);
        out.push(p.symbol);
    }
    Some(out)
}

/// A body's painted sites relative to its centre: the pattern a copy
/// stamps, as bytes.
fn pattern_key(cells: &[PaintedSite]) -> Option<Vec<u8>> {
    centred_key(cells, cells)
}

/// The pattern the output stamped on a seed: the output's body standing on
/// the seed's sites, relative to the SEED's centre.
fn stamped_key(seed: &[PaintedSite], out_bodies: &[Body]) -> Option<Vec<u8>> {
    let over = out_bodies.iter().map(painted_of).find(|cells| cells.iter().any(|p| seed.iter().any(|s| s.site == p.site)))?;
    centred_key(&over, seed)
}

/// C2 — A COPY READ OFF ITS EFFECT: the bodies the effect touched are the
/// seeds; on each pair, what the output stamped over every seed, relative to
/// the seed's centre, is one pattern, and the template is the input body
/// whose own centred pattern is that pattern — read by its key, never a bank
/// of templates tried. The seeds and the template are each separated by the
/// list. `None` where the seeds stamp two patterns, or no body is the one
/// they stamp.
fn read_copy(r: &PredicateTable<'_, '_>, touched: &[Vec<usize>], every: &[Vec<usize>]) -> Option<Vec<Clause>> {
    let mut templates: Vec<Vec<usize>> = Vec::with_capacity(r.pairs.len());
    for (p, seeds) in r.pairs.iter().zip(touched) {
        let out_bodies = perceive(p.y);
        let mut by_pattern: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (b, body) in p.bodies.iter().enumerate() {
            if seeds.contains(&b) {
                continue;
            }
            if let Some(key) = pattern_key(&painted_of(body)) {
                if by_pattern.get(&key).is_none() {
                    by_pattern.insert(key, b);
                }
            }
        }
        let mut template: Option<usize> = None;
        for &s in seeds {
            let key = stamped_key(&painted_of(&p.bodies[s]), &out_bodies)?;
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
fn read_panes(r: &PredicateTable<'_, '_>) -> Option<Clause> {
    let ground = crate::transducer::raster::ground();
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
        // The second pane stands one pane and the gap further along the split.
        let pane = match along_rows {
            true => hy,
            false => wy,
        };
        let further = i64::try_from(pane).ok()?.checked_add(i64::from(gap))?;
        let across = match along_rows {
            true => [further, 0],
            false => [0, further],
        };
        for (site, y) in p.y.sites() {
            let a = p.x.at(site)?;
            let b = p.x.at(site.moved(across)?)?;
            let bit = 2 * usize::from(!ground.holds(&a)) + usize::from(!ground.holds(&b));
            let painted = !ground.holds(&y);
            match table[bit] {
                Some(was) if was != painted => return None,
                _ => table[bit] = Some(painted),
            }
            if painted {
                match hue {
                    Some(h) if h != y => return None,
                    _ => hue = Some(y),
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
fn read_tiles(r: &PredicateTable<'_, '_>) -> Option<Clause> {
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
                // The block of the output at `(bi, bj)`, read in place.
                let corner = [bi * hx, bj * wx];
                let block = |site: SectionSite| -> Option<u8> { p.y.at(site.moved([i64::try_from(corner[0]).ok()?, i64::try_from(corner[1]).ok()?])?) };
                statement::turn_of(p.x, t).is_some_and(|g| g.extents() == [hx, wx] && g.sites().all(|(site, h)| block(site) == Some(h)))
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
fn sized(r: &mut PredicateTable<'_, '_>, carried: Vec<Clause>, every: &[Vec<usize>]) -> Result<Vec<Clause>, Unread> {
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
                let spans: Vec<Vec<SectionSite>> = match extent {
                    Extent::Body => p.bodies.iter().map(|b| painted_of(b).into_iter().map(|q| q.site).collect()).collect(),
                    Extent::Interior => statement::regions_each(RegionKind::Interior, p.x, &p.bodies),
                };
                spans
                    .iter()
                    .enumerate()
                    .filter(|(_, sites)| SectionSite::span_of(sites.iter().copied()).is_some_and(|(_, e)| e == [yr, yc]))
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
