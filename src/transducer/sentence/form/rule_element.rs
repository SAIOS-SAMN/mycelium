// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ACTS AS GEOMETRIC ELEMENTS (ruled 2026-10-26 — the English DSL trap:
//! the language is the source, the clause is its gloss).
//!
//! A clause's act names in English what is done to its selection; the
//! element is what is done, as geometry. Six of the acts are one element, a
//! CARRIAGE: a versor of `Cl(4,1)(ℚ)` read about the grid or about each
//! body, applied once or iterated to its meet, the source cleared or kept or
//! the path swept, the landed cells painted their own hue, a named one, or a
//! partner's, then mapped by a gauge. `Recolour` and `Repaint` are the
//! identity carriage, `Carry` and `Echo` one application clearing or keeping
//! its source, `Slide` a lattice translation iterated to the meet, `Extend`
//! the same translation sweeping each cell's path. A carriage no act names —
//! a slide that keeps its source, a carry painted a partner's hue — is said
//! all the same and has no gloss ([`gloss`] is `None`): the English is a
//! translation of what the language says, never the bound of it.
//!
//! The other acts are elements of their own: a 2-chain the selection bounds
//! (`Fill`), 1-chains between selected bodies straight or round one corner
//! (`Connect`, `Bend`), a body carried centre onto centre (`Copy`), and the
//! frame acts (`Crop`, `Scale`, `Panes`, `Tile`).

use super::clause::{
    put_act, put_gauge, put_paint, put_pred, put_stop, take_act, take_byte, take_gauge, take_paint, take_pred, take_stop, About, Act, Clause, Extent, Paint,
    Pred, Region, Stop,
};
use crate::core::Q;
use crate::transducer::address::{put_key_count, take_key_count};
use crate::transducer::base::Site;
use crate::transducer::cga::Mv;
use crate::transducer::element;

/// HOW FAR A CARRIAGE'S VERSOR IS APPLIED: once, or iterated until its next
/// application would leave the grid or, at `Stop::Contact`, meet a painted
/// cell. Iteration reads a lattice translation; any other versor iterated
/// is refused by name (it names no ray).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reach {
    Once,
    Meet(Stop),
}

/// WHAT A CARRIAGE LEAVES BEHIND: its source cleared, kept, or — iterated —
/// every cell's path to its meet swept (the source kept). A sweep of one
/// application is its landing, so `Once` sweeps as it keeps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trace {
    Clears,
    Keeps,
    Sweeps,
}

/// A CARRIAGE: the selected bodies carried by `motion`, about the grid or
/// about each body's own centre, as far as `reach`, leaving `trace`; each
/// landed cell painted `paint` (its own hue, a named one, or its partner's)
/// and mapped by `gauge`.
#[derive(Clone, Debug)]
pub struct Carriage {
    pub motion: Mv,
    pub about: About,
    pub reach: Reach,
    pub trace: Trace,
    pub paint: Paint,
    pub gauge: Vec<(u8, u8)>,
}

/// WHAT IS DONE TO A SELECTION, as geometry.
#[derive(Clone, Debug)]
pub enum RuleElement {
    Carriage(Carriage),
    /// The 2-chain the selection bounds, painted one hue.
    Chain { region: Region, hue: u8 },
    /// 1-chains between selected bodies through ground: straight along rows
    /// and columns, or round one corner to each body `corner` selects.
    Join { corner: Option<Pred>, paint: Paint },
    /// The one body `template` selects carried centre onto centre onto each
    /// selected body, which it replaces.
    Stamp { template: Pred },
    /// The output is the extent of the one selected body.
    Window(Extent),
    /// The output dilated by `up / down`, one of them 1.
    Dilate { up: u8, down: u8 },
    Panes { along_rows: bool, gap: bool, table: u8, paint: u8 },
    Tile { rows: u8, cols: u8, turns: Vec<u8> },
}

/// A CLAUSE LOWERED: its selection, read among the scene's bodies or its
/// monochrome parts, and the element done to it.
#[derive(Clone, Debug)]
pub struct Lowered {
    pub select: Pred,
    pub parts: bool,
    pub element: RuleElement,
}

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

/// The identity versor.
pub(crate) fn still() -> Mv {
    Mv::scalar(q(1))
}

/// THE LATTICE TRANSLATION A VERSOR IS, `(rows, columns)`: read where the
/// sensor reads a cell — its centre — on three cells, and only where it
/// carries each by one integer vector. `None` for any other motion.
pub(crate) fn lattice_step(motion: &Mv) -> Option<(i64, i64)> {
    let land = |r: i64, c: i64| -> Option<(Q, Q)> {
        let p = element::carry_site_center(motion, &Site::of([r, c, 0]))?;
        let [pr, pc, pk] = p.as_slice() else { return None };
        (pk == &q(0)).then(|| (pr.clone(), pc.clone()))
    };
    let (o, a, b) = (land(0, 0)?, land(1, 0)?, land(0, 1)?);
    let half = Q::new(1.into(), 2.into());
    let (dr, dc) = (&o.0 - &half, &o.1 - &half);
    let moved = |p: &(Q, Q), r: i64, c: i64| p.0 == &dr + q(r) + &half && p.1 == &dc + q(c) + &half;
    if !(dr.is_integer() && dc.is_integer() && moved(&a, 1, 0) && moved(&b, 0, 1)) {
        return None;
    }
    Some((i64::try_from(dr.to_integer()).ok()?, i64::try_from(dc.to_integer()).ok()?))
}

fn translation(ray: (i8, i8)) -> Mv {
    crate::transducer::cga::translator_at(&[q(i64::from(ray.0)), q(i64::from(ray.1))])
}

fn carriage(motion: Mv, about: About, reach: Reach, trace: Trace, paint: Paint, gauge: Vec<(u8, u8)>) -> RuleElement {
    RuleElement::Carriage(Carriage { motion, about, reach, trace, paint, gauge })
}

fn lower_act(act: &Act) -> Option<RuleElement> {
    Some(match act {
        Act::Recolour(map) => carriage(still(), About::Grid, Reach::Once, Trace::Clears, Paint::Own, map.clone()),
        Act::Repaint { paint } => carriage(still(), About::Grid, Reach::Once, Trace::Clears, paint.clone(), Vec::new()),
        Act::Carry { motion, about, gauge } => carriage(motion.clone(), *about, Reach::Once, Trace::Clears, Paint::Own, gauge.clone()),
        Act::Echo { motion, about, gauge } => carriage(motion.clone(), *about, Reach::Once, Trace::Keeps, Paint::Own, gauge.clone()),
        Act::Slide { ray, stop } => carriage(translation(*ray), About::Grid, Reach::Meet(*stop), Trace::Clears, Paint::Own, Vec::new()),
        Act::Extend { ray, paint, stop } => carriage(translation(*ray), About::Grid, Reach::Meet(*stop), Trace::Sweeps, paint.clone(), Vec::new()),
        Act::Fill { region, hue } => RuleElement::Chain { region: *region, hue: *hue },
        Act::Connect { paint } => RuleElement::Join { corner: None, paint: paint.clone() },
        Act::Bend { to, paint } => RuleElement::Join { corner: Some(to.clone()), paint: paint.clone() },
        Act::Copy { template } => RuleElement::Stamp { template: template.clone() },
        Act::Crop(extent) => RuleElement::Window(*extent),
        Act::Scale { up, down } => RuleElement::Dilate { up: *up, down: *down },
        Act::Panes { along_rows, gap, table, paint } => RuleElement::Panes { along_rows: *along_rows, gap: *gap, table: *table, paint: *paint },
        Act::Tile { rows, cols, turns } => RuleElement::Tile { rows: *rows, cols: *cols, turns: turns.clone() },
        Act::Parts(_) => return None,
    })
}

/// THE CLAUSE AS ITS ELEMENT. `None` for parts named inside parts: parts are
/// read one level.
pub fn lower(clause: &Clause) -> Option<Lowered> {
    let (parts, act) = match &clause.act {
        Act::Parts(inner) => (true, inner.as_ref()),
        act => (false, act),
    };
    Some(Lowered { select: clause.select.clone(), parts, element: lower_act(act)? })
}

/// A RUN OF CLAUSES AS ITS ELEMENTS, in the rule's order; `None` where one
/// names parts inside parts.
pub fn lower_all(clauses: &[Clause]) -> Option<Vec<Lowered>> {
    clauses.iter().map(lower).collect()
}

fn gloss_carriage(t: &Carriage) -> Option<Act> {
    let own = t.paint == Paint::Own;
    let fixed = t.motion.projective_eq(&still());
    let ray = || -> Option<(i8, i8)> {
        let (dr, dc) = lattice_step(&t.motion)?;
        Some((i8::try_from(dr).ok()?, i8::try_from(dc).ok()?))
    };
    match (t.reach, t.trace) {
        (Reach::Once, Trace::Clears) if fixed && own => Some(Act::Recolour(t.gauge.clone())),
        (Reach::Once, Trace::Clears) if fixed && t.gauge.is_empty() => Some(Act::Repaint { paint: t.paint.clone() }),
        (Reach::Once, Trace::Clears) if own => Some(Act::Carry { motion: t.motion.clone(), about: t.about, gauge: t.gauge.clone() }),
        (Reach::Once, Trace::Keeps | Trace::Sweeps) if own => Some(Act::Echo { motion: t.motion.clone(), about: t.about, gauge: t.gauge.clone() }),
        (Reach::Meet(stop), Trace::Clears) if own && t.gauge.is_empty() => Some(Act::Slide { ray: ray()?, stop }),
        (Reach::Meet(stop), Trace::Sweeps) if t.gauge.is_empty() => Some(Act::Extend { ray: ray()?, paint: t.paint.clone(), stop }),
        _ => None,
    }
}

/// THE CARRIAGE AS ONE TAPE WRITES IT: where the frame it is read about does
/// not move what it says — the identity, or a translation iterated, which
/// every body's centre conjugates to itself — it is read about the grid;
/// and one application sweeps as it keeps.
fn canonical(t: &Carriage) -> Carriage {
    let mut t = t.clone();
    if matches!(t.reach, Reach::Meet(_)) || t.motion.projective_eq(&still()) {
        t.about = About::Grid;
    }
    if t.reach == Reach::Once && t.trace == Trace::Sweeps {
        t.trace = Trace::Keeps;
    }
    t
}

/// THE TAG AN ELEMENT NO ACT NAMES IS WRITTEN UNDER, in the act's place: past
/// every act's tag, so a reader that knows only the acts refuses it.
const UNGLOSSED: u8 = 16;

fn put_unglossed(buf: &mut Vec<u8>, l: &Lowered, t: &Carriage) {
    put_pred(buf, &l.select);
    buf.push(UNGLOSSED);
    buf.push(u8::from(l.parts));
    let t = canonical(t);
    t.motion.put_canonical(buf);
    buf.push(match t.about {
        About::Grid => 0,
        About::Body => 1,
    });
    match t.reach {
        Reach::Once => buf.push(0),
        Reach::Meet(stop) => {
            buf.push(1);
            put_stop(buf, stop);
        }
    }
    buf.push(match t.trace {
        Trace::Clears => 0,
        Trace::Keeps => 1,
        Trace::Sweeps => 2,
    });
    put_paint(buf, &t.paint);
    put_gauge(buf, &t.gauge);
}

fn take_unglossed(k: &[u8], p: &mut usize, select: Pred) -> Option<Lowered> {
    let parts = match take_byte(k, p)? {
        0 => false,
        1 => true,
        _ => return None,
    };
    let motion = Mv::from_bytes(k, p)?;
    let about = match take_byte(k, p)? {
        0 => About::Grid,
        1 => About::Body,
        _ => return None,
    };
    let reach = match take_byte(k, p)? {
        0 => Reach::Once,
        1 => Reach::Meet(take_stop(k, p)?),
        _ => return None,
    };
    let trace = match take_byte(k, p)? {
        0 => Trace::Clears,
        1 => Trace::Keeps,
        2 => Trace::Sweeps,
        _ => return None,
    };
    let t = Carriage { motion, about, reach, trace, paint: take_paint(k, p)?, gauge: take_gauge(k, p)? };
    // ONE TAPE PER ELEMENT: a carriage written other than canonically, or
    // one an act names — whose tape is the act's — is no element's tape.
    let c = canonical(&t);
    let gauge_written = {
        let mut g = t.gauge.clone();
        g.sort_unstable();
        g == t.gauge
    };
    if c.about != t.about || c.trace != t.trace || !gauge_written {
        return None;
    }
    let l = Lowered { select, parts, element: RuleElement::Carriage(t) };
    gloss(&l).is_none().then_some(l)
}

/// THE ELEMENTS AS THE FORM'S ADDRESS WRITES THEM, in the rule's own order
/// (the order is content: a later element paints over an earlier one). An
/// element an act names is written as that clause, byte for byte — so no
/// form already minted moves its address — and one no act names under its
/// own tag.
pub(crate) fn put_elements(buf: &mut Vec<u8>, elements: &[Lowered]) {
    put_key_count(buf, elements.len());
    for l in elements {
        match named(l) {
            Ok(c) => {
                put_pred(buf, &c.select);
                put_act(buf, &c.act);
            }
            Err(t) => put_unglossed(buf, l, t),
        }
    }
}

/// The inverse of [`put_elements`]: an act's tape lowered to its element,
/// an element's own tape read where no act names it. `None` where the bytes
/// are no run of elements: a form whose statements do not read is no form
/// (rule 11).
pub(crate) fn take_elements(k: &[u8], p: &mut usize) -> Option<Vec<Lowered>> {
    let n = take_key_count(k, p)?;
    if n > k.len().checked_sub(*p)? {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let select = take_pred(k, p, 0)?;
        let l = match k.get(*p) {
            Some(&UNGLOSSED) => {
                *p += 1;
                take_unglossed(k, p, select)?
            }
            _ => lower(&Clause { select, act: take_act(k, p)? })?,
        };
        out.push(l);
    }
    Some(out)
}

/// THE ENGLISH OF AN ELEMENT: the clause that names what it does, `None`
/// where no act names it. A gloss is a translation for readers; the
/// element is what is said.
pub fn gloss(l: &Lowered) -> Option<Clause> {
    named(l).ok()
}

/// The clause naming the element, or the carriage no act names.
fn named(l: &Lowered) -> Result<Clause, &Carriage> {
    let act = match &l.element {
        RuleElement::Carriage(t) => gloss_carriage(t).ok_or(t)?,
        RuleElement::Chain { region, hue } => Act::Fill { region: *region, hue: *hue },
        RuleElement::Join { corner: None, paint } => Act::Connect { paint: paint.clone() },
        RuleElement::Join { corner: Some(to), paint } => Act::Bend { to: to.clone(), paint: paint.clone() },
        RuleElement::Stamp { template } => Act::Copy { template: template.clone() },
        RuleElement::Window(extent) => Act::Crop(*extent),
        RuleElement::Dilate { up, down } => Act::Scale { up: *up, down: *down },
        RuleElement::Panes { along_rows, gap, table, paint } => Act::Panes { along_rows: *along_rows, gap: *gap, table: *table, paint: *paint },
        RuleElement::Tile { rows, cols, turns } => Act::Tile { rows: *rows, cols: *cols, turns: turns.clone() },
    };
    let act = match l.parts {
        true => Act::Parts(Box::new(act)),
        false => act,
    };
    Ok(Clause { select: l.select.clone(), act })
}
