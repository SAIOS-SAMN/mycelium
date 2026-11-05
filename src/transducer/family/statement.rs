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
use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
use crate::transducer::sentence::form::clause::{exact_root, About, Clause, Comparison, Coord, Extent, Order, Paint, Pred, RegionKind, Stop, Tie, Value};
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
    /// A carried or copied cell landed off the substrate's sites.
    OffSubstrate,
    /// A created line names no ray.
    Still,
    /// An act on parts names another act on parts: parts are read one level.
    Nested,
    /// The input has not the frame the act names: no two equal panes it
    /// splits into, or a 90° turn of a tile that is not square; or the
    /// family reads no one ground for the act to clear cells onto or lay a
    /// new canvas on (a vortex, or no ground; MU-SENSING §3.4).
    Unframed,
    /// A hue read off a partner (`Paint::Of`) found no touched body the
    /// predicate selects, or bodies of more than one hue; or a join, which
    /// has two ends and no one partner, was asked for one.
    Unpartnered,
}

/// THE SQUARE'S SYMMETRY `t` ON A SECTION SAMPLE (`Act::Tile`'s codes): 0
/// itself, 1 mirrored across its columns, 2 across its rows, 3 a turn of
/// 180°, 4 a turn of 90°, 5 of 270°, 6 the transpose, 7 the anti-transpose.
/// `None` for a code past 7, or a 90° turn or transpose of a sample that is
/// not square.
pub(crate) fn turn_of(g: &SectionSample, t: u8) -> Option<SectionSample> {
    let [h, w] = g.extents();
    let at = |i: usize, j: usize| read(g, i, j);
    // The turned sample opens as the symbol at the origin and is written at
    // every one of its sites before it is read.
    let grid = |rows: usize, cols: usize, f: &dyn Fn(usize, usize) -> Option<u8>| -> Option<SectionSample> {
        let mut out = SectionSample::filled([rows, cols], at(0, 0)?).ok()?;
        let mut writes = Vec::with_capacity(rows * cols);
        for (site, _) in out.sites() {
            let [i, j] = site.indices()?;
            writes.push((site, f(i, j)?));
        }
        for (site, h) in writes {
            out.paint(site, h)?;
        }
        Some(out)
    };
    let last = |n: usize, i: usize| n.checked_sub(1).and_then(|m| m.checked_sub(i));
    match t {
        0 => Some(g.clone()),
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
    pub painted: Vec<PaintedSite>,
}

/// THE SCENE AS THE CLAUSES READ IT, perceived once: each body's painted
/// sites of the section, and the body each site of the sample belongs to.
struct Seen<'a> {
    x: &'a SectionSample,
    bodies: &'a [Body],
    /// Each body's painted sites, in its compartments' order.
    cells: Vec<Vec<PaintedSite>>,
    /// The body owning each site of the sample, at the site's offset in
    /// canonical address order (`SectionSample::offset_of`).
    owner: Vec<Option<usize>>,
}

/// The steps of the section's rank `directions` names, each a step along the
/// plane; a direction of another rank names none.
fn on_plane(directions: Vec<Vec<i64>>) -> Vec<[i64; ALPHABET_SAMPLE_RANK]> {
    directions.into_iter().filter_map(|d| <[i64; ALPHABET_SAMPLE_RANK]>::try_from(d).ok()).collect()
}

/// THE FACE NEIGHBOURS of a site of the alphabet's section, both senses:
/// the contact of grade `n − 1`, generated at the section's rank
/// (`sentence::faces_of`), never a list of offsets.
fn edges() -> Vec<[i64; ALPHABET_SAMPLE_RANK]> {
    on_plane(crate::transducer::sentence::faces_of(ALPHABET_SAMPLE_RANK))
}

/// EVERY NEIGHBOUR of a site of the alphabet's section, corners included,
/// in lexicographic order (`sentence::surrounding`), generated.
fn around() -> Vec<[i64; ALPHABET_SAMPLE_RANK]> {
    on_plane(crate::transducer::sentence::surrounding(ALPHABET_SAMPLE_RANK))
}

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn qu(n: usize) -> Option<Q> {
    Some(q(i64::try_from(n).ok()?))
}

/// THE CORNERS OF PAINTED SITES along the plane: the least coordinate and
/// the greatest on each axis, read over every site (`SectionSite::span_of`).
/// `None` for no site.
pub(crate) fn corners_of(cells: &[PaintedSite]) -> Option<(SectionSite, SectionSite)> {
    let (least, extents) = SectionSite::span_of(cells.iter().map(|p| p.site))?;
    let mut by = [0i64; ALPHABET_SAMPLE_RANK];
    for (d, e) in by.iter_mut().zip(extents) {
        *d = i64::try_from(e.checked_sub(1)?).ok()?;
    }
    Some((least, least.moved(by)?))
}

/// The coordinate of a site along one axis of the plane.
fn along(s: SectionSite, axis: usize) -> Option<i64> {
    s.coordinates().get(axis).copied()
}

/// Whether a site stands on the sample's outer seam: some coordinate at `0`
/// or at its extent's last site (MU-SENSING §3.4).
fn on_seam(site: SectionSite, extents: [usize; ALPHABET_SAMPLE_RANK]) -> bool {
    site.coordinates().iter().zip(extents).any(|(&c, e)| c == 0 || usize::try_from(c).ok().and_then(|c| c.checked_add(1)) == Some(e))
}

impl<'a> Seen<'a> {
    fn of(x: &'a SectionSample, bodies: &'a [Body]) -> Seen<'a> {
        let mut owner = vec![None; x.symbols().len()];
        let mut cells = Vec::with_capacity(bodies.len());
        for (i, b) in bodies.iter().enumerate() {
            let mut owned = Vec::with_capacity(b.compartments.len());
            for l in &b.compartments {
                let Some(site) = SectionSite::of_family(&l.site) else { continue };
                if let Some(o) = x.offset_of(site).and_then(|k| owner.get_mut(k)) {
                    *o = Some(i);
                    owned.push(PaintedSite::new(site, l.hue));
                }
            }
            cells.push(owned);
        }
        Seen { x, bodies, cells, owner }
    }

    /// The site `site` stepped `by` along the plane, where it lands on the
    /// sample; `None` off it.
    fn step(&self, site: SectionSite, by: [i64; ALPHABET_SAMPLE_RANK]) -> Option<SectionSite> {
        let s = site.moved(by)?;
        self.x.at(s).map(|_| s)
    }

    /// The symbol at a site; `None` off the sample, which holds none.
    fn hue(&self, site: SectionSite) -> Option<u8> {
        self.x.at(site)
    }

    /// Is the site on the sample and read as ground?
    fn grounded(&self, site: SectionSite) -> bool {
        self.hue(site).is_some_and(crate::transducer::raster::is_ground)
    }

    fn owner_at(&self, site: SectionSite) -> Option<usize> {
        self.x.offset_of(site).and_then(|k| self.owner.get(k)).copied().flatten()
    }

    /// THE BACKGROUND SITES THE BODIES OF `set` ENCLOSE between them: every
    /// background site no face-connected path reaches from the sample's
    /// outer seam without crossing a site of a body of `set`, in canonical
    /// address order. A body standing inside the enclosure is inside it, not
    /// a breach of it.
    fn enclosed(&self, set: &[usize]) -> Vec<SectionSite> {
        let wall = |s: SectionSite| self.owner_at(s).is_some_and(|o| set.contains(&o));
        let extents = self.x.extents();
        let mut reached = vec![false; self.owner.len()];
        let mut stack: Vec<SectionSite> = Vec::new();
        for (site, _) in self.x.sites() {
            if on_seam(site, extents) && !wall(site) {
                if let Some(slot) = self.x.offset_of(site).and_then(|k| reached.get_mut(k)) {
                    *slot = true;
                    stack.push(site);
                }
            }
        }
        let edges = edges();
        while let Some(s) = stack.pop() {
            for &d in &edges {
                let Some(n) = self.step(s, d) else { continue };
                let Some(slot) = self.x.offset_of(n).and_then(|k| reached.get_mut(k)) else { continue };
                if !*slot && !wall(n) {
                    *slot = true;
                    stack.push(n);
                }
            }
        }
        self.x
            .sites()
            .zip(&reached)
            .filter(|((site, _), &got)| !got && self.grounded(*site))
            .map(|((site, _), _)| site)
            .collect()
    }

    /// The other bodies `b` touches.
    fn touching(&self, b: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let around = around();
        for p in &self.cells[b] {
            for &d in &around {
                if let Some(o) = self.step(p.site, d).and_then(|s| self.owner_at(s)) {
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
            for p in &self.cells[o] {
                match hue {
                    Some(k) if k != p.symbol => return Err(Unstated::Unpartnered),
                    _ => hue = Some(p.symbol),
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
        let corners = corners_of(cells);
        // The least and the greatest coordinate along one axis of the plane,
        // and the span between them, both ends included.
        let least = |axis: usize| corners.and_then(|(lo, _)| along(lo, axis)).map(q);
        let greatest = |axis: usize| corners.and_then(|(_, hi)| along(hi, axis)).map(q);
        let span = |axis: usize| corners.and_then(|(lo, hi)| along(hi, axis)?.checked_sub(along(lo, axis)?)?.checked_add(1)).map(q);
        let mut hues: Vec<u8> = cells.iter().map(|p| p.symbol).collect();
        hues.sort_unstable();
        hues.dedup();
        Ok(match coord {
            Coord::Cells => qu(cells.len()),
            Coord::Extent(axis) => span(axis.index()),
            Coord::Least(axis) => least(axis.index()),
            Coord::Greatest(axis) => greatest(axis.index()),
            Coord::Hues => qu(hues.len()),
            Coord::Contacts => qu(self.touching(b).len()),
            Coord::Enclosed => qu(self.enclosed(&[b]).len()),
            Coord::Tally => match hues.as_slice() {
                [h] => qu(self.x.symbols().iter().filter(|&&v| v == *h).count()),
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
            Pred::Hue(h) => domain.iter().copied().filter(|&b| self.cells[b].iter().any(|p| p.symbol == *h)).collect(),
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
                        Comparison::Eq => v == to,
                        Comparison::Less => v < to,
                        Comparison::Greater => v > to,
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

    /// THE BACKGROUND SITES A REGION NAMES about the selection, each once
    /// per body that names it (a fill paints them all one hue).
    fn region(&self, region: RegionKind, sel: &[usize]) -> Vec<SectionSite> {
        match region {
            RegionKind::Interior => self.enclosed(sel),
            RegionKind::Halo => {
                let mut out = Vec::new();
                let around = around();
                for &b in sel {
                    for p in &self.cells[b] {
                        for &d in &around {
                            if let Some(s) = self.step(p.site, d) {
                                if self.grounded(s) {
                                    out.push(s);
                                }
                            }
                        }
                    }
                }
                out
            }
            RegionKind::Extent => {
                // The sites between a body's corners on every axis, in
                // canonical address order.
                let mut out = Vec::new();
                for &b in sel {
                    let Some((lo, hi)) = corners_of(&self.cells[b]) else { continue };
                    let inside = |s: SectionSite| s.coordinates().iter().zip(lo.coordinates().iter().zip(hi.coordinates())).all(|(c, (l, h))| (*l..=h).contains(c));
                    out.extend(self.x.sites().map(|(s, _)| s).filter(|&s| inside(s) && self.grounded(s)));
                }
                out
            }
        }
    }

    /// A body's centre in the sensor's coordinates (each index plus ½):
    /// the centre between its corners on every axis of the plane, exact.
    fn centre(&self, b: usize) -> Option<[Q; ALPHABET_SAMPLE_RANK]> {
        let (lo, hi) = corners_of(&self.cells[b])?;
        let two = q(2);
        Some(std::array::from_fn(|k| (q(lo.coordinates()[k]) + q(hi.coordinates()[k]) + q(1)) / &two))
    }
}

/// THE SITE A VERSOR CARRIES `site` ONTO, read where the sensor reads it —
/// the compartment's centre on the family's base, in the frame the scene
/// stands in — and only where it lands on a compartment's centre in that
/// frame: a centre between sites is off the lattice.
pub(crate) fn carried(v: &Versor, site: SectionSite) -> Result<SectionSite, Unstated> {
    let base = site.on_family(0).ok_or(Unstated::OffSubstrate)?;
    let p = element::carry_site_centre(v, &base).ok_or(Unstated::OffSubstrate)?;
    let half = Q::new(1.into(), 2.into());
    let [pr, pc, pk] = p.as_slice() else { return Err(Unstated::OffSubstrate) };
    let (ir, ic) = (pr - &half, pc - &half);
    if !ir.is_integer() || !ic.is_integer() || pk != &q(0) {
        return Err(Unstated::OffSubstrate);
    }
    let whole = |x: &Q| i64::try_from(x.to_integer()).map_err(|_| Unstated::OffSubstrate);
    Ok(SectionSite::on_plane([whole(&ir)?, whole(&ic)?]))
}

fn mapped(map: &[(u8, u8)], h: u8) -> u8 {
    map.iter().find(|(f, _)| *f == h).map_or(h, |(_, t)| *t)
}

/// A landed site past the frame is no site of this sample and is not spoken
/// on it, as a carried compartment past the frame is not.
fn lay(canvas: &mut SectionSample, p: PaintedSite) {
    // A site off the canvas is not written.
    let _ = canvas.paint(p.site, p.symbol);
}

/// THE SYMBOL AT INDICES `(r, c)` ALONG THE PERSPECTIVE'S PLANE of a
/// section sample; `None` off it.
fn read(g: &SectionSample, r: usize, c: usize) -> Option<u8> {
    g.at(SectionSite::of_indices([r, c])?)
}

/// Is the canvas's site read as ground? Never off the canvas.
fn grounded_on(canvas: &SectionSample, ground: &crate::transducer::family::transduce::Ground<u8>, site: SectionSite) -> bool {
    canvas.at(site).is_some_and(|h| ground.holds(&h))
}

/// WRITE `h` AT `site` of the canvas, in place: the sample is painted where
/// it stands, never copied into a raster. A site off the canvas is not
/// written.
fn paint(canvas: &mut SectionSample, site: SectionSite, h: u8) {
    let _ = canvas.paint(site, h);
}

/// SPEAK A CARRIAGE on the selection `sel` of `seen`, onto `canvas`.
fn carriage(seen: &Seen, t: &Carriage, sel: &[usize], canvas: &mut SectionSample) -> Result<(), Unstated> {
    // A cleared cell is laid on the scene's one ground, read where a cell is
    // cleared; a stop reads the ground as membership.
    let ground = crate::transducer::raster::ground();
    let void = || crate::transducer::raster::void().map_err(|_| Unstated::Unframed);
    let colour = |b: usize, h: u8| -> Result<u8, Unstated> { Ok(mapped(&t.gauge, seen.painted(&t.paint, b, h)?)) };
    match t.reach {
        Reach::Once => {
            // The identity carries every cell onto itself: no versor is
            // applied to a cell it cannot move.
            let fixed = t.motion.projective_eq(&rule_element::still());
            let mut landed: Vec<PaintedSite> = Vec::new();
            for &b in sel {
                let v = match (fixed, t.about) {
                    (true, _) => None,
                    (false, About::Grid) => Some(t.motion.clone()),
                    (false, About::Body) => {
                        let centre = seen.centre(b).ok_or(Unstated::OffSubstrate)?;
                        let to = crate::transducer::cga::translator_at(&centre);
                        let from = crate::transducer::cga::translator_at(&centre.map(|x| -x));
                        Some(to.geo(&t.motion).geo(&from))
                    }
                };
                for p in &seen.cells[b] {
                    let site = match &v {
                        Some(v) => carried(v, p.site)?,
                        None => p.site,
                    };
                    landed.push(PaintedSite::new(site, colour(b, p.symbol)?));
                }
            }
            if t.trace == Trace::Clears {
                for &b in sel {
                    for p in &seen.cells[b] {
                        paint(canvas, p.site, void()?);
                    }
                }
            }
            for p in landed {
                lay(canvas, p);
            }
        }
        Reach::Meet(stop) => {
            let (dr, dc) = rule_element::substrate_step(&t.motion).filter(|&s| s != (0, 0)).ok_or(Unstated::Still)?;
            let ray = [dr, dc];
            // The ray taken `k` steps along the plane.
            let steps = |k: i64| -> Option<[i64; ALPHABET_SAMPLE_RANK]> {
                let mut out = ray;
                for x in out.iter_mut() {
                    *x = x.checked_mul(k)?;
                }
                Some(out)
            };
            match t.trace {
                Trace::Sweeps => {
                    // A line stops at what was painted BEFORE this clause, so
                    // no line of the clause stops another and the order the
                    // bodies are read in is no part of what it says.
                    let before = canvas.clone();
                    for &b in sel {
                        for p in &seen.cells[b] {
                            let colour = colour(b, p.symbol)?;
                            let mut at = p.site;
                            while let Some(next) = seen.step(at, ray) {
                                if stop == Stop::Contact && !grounded_on(&before, &ground, next) {
                                    break;
                                }
                                paint(canvas, next, colour);
                                at = next;
                            }
                        }
                    }
                }
                Trace::Clears | Trace::Keeps => {
                    // THE BODIES NEAREST A STOP FIRST: a body's reach is its
                    // leading cell's along the ray. Each reach lands on the
                    // canvas as the nearer ones left it, so a stack settles
                    // as it lands; bodies of ONE reach land together, against
                    // the canvas as it stood before them — no order among
                    // them is chosen — and two of them landing on one cell is
                    // a tie, refused by name.
                    let reach = |b: usize| -> Option<i64> {
                        seen.cells[b].iter().try_fold(None, |most: Option<i64>, p| {
                            // The site's coordinate along the ray: each
                            // coordinate times the ray's step on that axis.
                            let mut at = 0i64;
                            for (c, d) in p.site.coordinates().into_iter().zip(ray) {
                                at = at.checked_add(c.checked_mul(d)?)?;
                            }
                            Some(Some(match most {
                                Some(m) if m >= at => m,
                                _ => at,
                            }))
                        })?
                    };
                    let mut order: Vec<(i64, usize)> = Vec::with_capacity(sel.len());
                    for &b in sel {
                        order.push((reach(b).ok_or(Unstated::OffSubstrate)?, b));
                    }
                    #[expect(clippy::disallowed_methods, reason = "rule 4 — the slide's own order, content: the reach read off each body along the ray, nearest its stop first; one reach lands together")]
                    order.sort_by_key(|&(reach, _)| std::cmp::Reverse(reach));
                    let mut at = 0usize;
                    while let Some(&(reach_here, _)) = order.get(at) {
                        let group: Vec<usize> = order[at..].iter().take_while(|(r, _)| *r == reach_here).map(|&(_, b)| b).collect();
                        at += group.len();
                        for &b in &group {
                            for p in &seen.cells[b] {
                                paint(canvas, p.site, void()?);
                            }
                        }
                        let mut landed: Vec<PaintedSite> = Vec::new();
                        for &b in &group {
                            let cells = &seen.cells[b];
                            // The furthest step every site takes onto the
                            // sample — and, at a contact stop, onto ground:
                            // the meet ends it.
                            let lands = |k: i64| -> bool {
                                cells.iter().all(|p| match steps(k).and_then(|by| seen.step(p.site, by)) {
                                    Some(s) => stop == Stop::Border || grounded_on(canvas, &ground, s),
                                    None => false,
                                })
                            };
                            let mut k: i64 = 0;
                            while lands(k + 1) {
                                k += 1;
                            }
                            for p in cells {
                                let s = steps(k).and_then(|by| seen.step(p.site, by)).ok_or(Unstated::OffSubstrate)?;
                                if landed.iter().any(|l| l.site == s) {
                                    return Err(Unstated::Tie);
                                }
                                landed.push(PaintedSite::new(s, colour(b, p.symbol)?));
                            }
                        }
                        for l in landed {
                            paint(canvas, l.site, l.symbol);
                        }
                        // A carriage that keeps its source leaves it where
                        // nothing landed on it.
                        if t.trace == Trace::Keeps {
                            for &b in &group {
                                for p in &seen.cells[b] {
                                    if grounded_on(canvas, &ground, p.site) {
                                        paint(canvas, p.site, p.symbol);
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
pub fn speak_clauses(clauses: &[Clause], x: &SectionSample, bodies: &[Body]) -> Result<Spoken, Unstated> {
    speak_elements(&rule_element::lower_all(clauses).ok_or(Unstated::Nested)?, x, bodies)
}

/// SPEAK THE ELEMENTS on `x`. The canvas opens as the input; each element
/// selects on the input and paints the canvas; a window sets the output's
/// extent. An element on parts selects among the scene's monochrome parts
/// (`perceive_parts`), read once where one asks.
pub fn speak_elements(said: &[Lowered], x: &SectionSample, bodies: &[Body]) -> Result<Spoken, Unstated> {
    let whole = Seen::of(x, bodies);
    let parts: Vec<Body> = match said.iter().any(|l| l.parts) {
        true => perceive_parts(x),
        false => Vec::new(),
    };
    let on_parts = Seen::of(x, &parts);
    // The canvas opens as the input sample and is painted in place, site by
    // site through the perspective's plane, into the output the clauses
    // write; no raster is laid (MU-SENSING §2).
    let mut canvas: SectionSample = x.clone();
    let mut window: Option<(SectionSite, SectionSite)> = None;
    let mut scale: Option<(u8, u8)> = None;
    // A frame element's output stands in place of the canvas (`Panes`, `Tile`).
    let mut framed: Option<SectionSample> = None;
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
                for site in seen.region(*region, &sel) {
                    paint(&mut canvas, site, *hue);
                }
            }
            RuleElement::Stamp { template } => {
                let chosen = seen.select(template, &all)?;
                let [t] = chosen.as_slice() else { return Err(Unstated::NotOne) };
                let t = *t;
                let at = seen.centre(t).ok_or(Unstated::NotOne)?;
                let mut landed: Vec<PaintedSite> = Vec::new();
                let seeds: Vec<usize> = sel.iter().copied().filter(|&b| b != t).collect();
                for &b in &seeds {
                    let seed = seen.centre(b).ok_or(Unstated::OffSubstrate)?;
                    let by: Vec<Q> = seed.iter().zip(&at).map(|(s, a)| s - a).collect();
                    let v = crate::transducer::cga::translator_at(&by);
                    for p in &seen.cells[t] {
                        landed.push(PaintedSite::new(carried(&v, p.site)?, p.symbol));
                    }
                }
                for &b in &seeds {
                    for p in &seen.cells[b] {
                        paint(&mut canvas, p.site, crate::transducer::raster::void().map_err(|_| Unstated::Unframed)?);
                    }
                }
                for p in landed {
                    lay(&mut canvas, p);
                }
            }
            RuleElement::Window(extent) => {
                let [b] = sel.as_slice() else { return Err(Unstated::NotOne) };
                let cells: Vec<PaintedSite> = match extent {
                    Extent::Body => seen.cells[*b].clone(),
                    Extent::Interior => seen.enclosed(&[*b]).into_iter().map(|s| PaintedSite::new(s, 0)).collect(),
                };
                window = Some(corners_of(&cells).ok_or(Unstated::NotOne)?);
            }
            RuleElement::Dilate { up, down } => scale = Some((*up, *down)),
            RuleElement::Panes { along_rows, gap, table, paint: hue } => {
                // The panes' unpainted cells are laid on the scene's one
                // ground; with none, no pane is spoken.
                let ground = crate::transducer::raster::ground();
                let void = crate::transducer::raster::void().map_err(|_| Unstated::Unframed)?;
                let [rows, cols] = x.extents();
                let (n, other) = match along_rows {
                    true => (rows, cols),
                    false => (cols, rows),
                };
                let k = n.checked_sub(usize::from(*gap)).filter(|m| m % 2 == 0).map(|m| m / 2).ok_or(Unstated::Unframed)?;
                if k == 0 {
                    return Err(Unstated::Unframed);
                }
                let cell = |a: usize, b: usize| -> Option<u8> {
                    match *along_rows {
                        true => read(&canvas, a, b),
                        false => read(&canvas, b, a),
                    }
                };
                // The pane's cell at `a` along the split, `b` across it.
                let at = |a: usize, b: usize| -> Option<u8> {
                    let first = !ground.holds(&cell(a, b)?);
                    let second = !ground.holds(&cell(a + k + usize::from(*gap), b)?);
                    let bit = 2 * usize::from(first) + usize::from(second);
                    Some(if (table >> bit) & 1 == 1 { *hue } else { void })
                };
                let extents = match along_rows {
                    true => [k, other],
                    false => [other, k],
                };
                let mut out = SectionSample::filled(extents, void).map_err(|_| Unstated::Unframed)?;
                let mut writes = Vec::with_capacity(out.symbols().len());
                for (site, _) in out.sites() {
                    let [i, j] = site.indices().ok_or(Unstated::Unframed)?;
                    let (a, b) = match along_rows {
                        true => (i, j),
                        false => (j, i),
                    };
                    writes.push((site, at(a, b).ok_or(Unstated::Unframed)?));
                }
                for (site, h) in writes {
                    out.paint(site, h).ok_or(Unstated::Unframed)?;
                }
                framed = Some(out);
            }
            RuleElement::Tile { rows, cols, turns } => {
                let [h, w] = x.extents();
                let (rows, cols) = (usize::from(*rows), usize::from(*cols));
                if turns.len() != rows * cols || rows == 0 || cols == 0 {
                    return Err(Unstated::Unframed);
                }
                // Every cell is a block's: each block holds the canvas's
                // extents, so the blocks lay every cell of the tiling once,
                // and a block of other extents is unframed, never a ground
                // written in for a cell it leaves. The tiling opens as the
                // canvas's symbol at the origin, written over at every cell.
                let laid = canvas.clone();
                let origin = read(&laid, 0, 0).ok_or(Unstated::Unframed)?;
                let mut out = SectionSample::filled([h * rows, w * cols], origin).map_err(|_| Unstated::Unframed)?;
                for (i, &t) in turns.iter().enumerate() {
                    let block = turn_of(&laid, t).ok_or(Unstated::Unframed)?;
                    if block.extents() != [h, w] {
                        return Err(Unstated::Unframed);
                    }
                    let corner = [i64::try_from((i / cols) * h), i64::try_from((i % cols) * w)];
                    let [Ok(br), Ok(bc)] = corner else { return Err(Unstated::Unframed) };
                    for (site, v) in block.sites() {
                        out.paint(site.moved([br, bc]).ok_or(Unstated::Unframed)?, v).ok_or(Unstated::Unframed)?;
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
                let ground = crate::transducer::raster::ground();
                let chosen = |site: SectionSite| -> Option<(usize, u8)> {
                    let o = seen.owner_at(site)?;
                    sel.contains(&o).then(|| seen.hue(site).map(|h| (o, h))).flatten()
                };
                let mut joins: Vec<PaintedSite> = Vec::new();
                // Each line of the plane read along its axis, the plane's last
                // axis first: a line is every site that shares the other
                // coordinates, walked from the outer seam where the axis
                // starts, the lines in canonical address order.
                let extents = x.extents();
                for axis in (0..ALPHABET_SAMPLE_RANK).rev() {
                    let mut unit = [0i64; ALPHABET_SAMPLE_RANK];
                    let (Some(d), Some(&inner)) = (unit.get_mut(axis), extents.get(axis)) else { continue };
                    *d = 1;
                    let starts = x.sites().map(|(s, _)| s).filter(|&s| along(s, axis) == Some(0));
                    for start in starts {
                        // The site `i` steps along the line.
                        let cell = |i: usize| -> Option<SectionSite> {
                            let i = i64::try_from(i).ok()?;
                            start.moved(unit.map(|u| u * i))
                        };
                        let mut last: Option<(usize, usize, u8)> = None;
                        for i in 0..inner {
                            let Some(site) = cell(i) else { continue };
                            let Some((o, h)) = chosen(site) else {
                                if !grounded_on(&before, &ground, site) {
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
                                        if let Some(between) = cell(m) {
                                            joins.push(PaintedSite::new(between, hue));
                                        }
                                    }
                                }
                            }
                            last = Some((i, o, h));
                        }
                    }
                }
                for p in joins {
                    paint(&mut canvas, p.site, p.symbol);
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
                let ground = crate::transducer::raster::ground();
                let targets = seen.select(to, &all)?;
                let one_hue = |b: usize| -> Option<u8> {
                    let [first, rest @ ..] = seen.cells[b].as_slice() else { return None };
                    rest.iter().all(|c| c.symbol == first.symbol).then_some(first.symbol)
                };
                let mut joins: Vec<PaintedSite> = Vec::new();
                for &f in &sel {
                    for &t in targets.iter().filter(|&&t| t != f) {
                        // The bend runs along the plane's second axis from the
                        // source, then along its first axis to the target: each
                        // body's corners, read as the planar act names them.
                        let (Some((f_lo, f_hi)), Some((t_lo, t_hi))) = (corners_of(&seen.cells[f]), corners_of(&seen.cells[t])) else {
                            continue;
                        };
                        let ([ft, fl], [fb, fr], [tt, tl], [tb, tr]) = (f_lo.coordinates(), f_hi.coordinates(), t_lo.coordinates(), t_hi.coordinates());
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
                        let across: Vec<i64> = match col > fr {
                            true => (fr + 1..=col).collect(),
                            false => (col..fl).collect(),
                        };
                        let down: Vec<i64> = match tt > row {
                            true => (row + 1..tt).collect(),
                            false => (tb + 1..row).collect(),
                        };
                        let path: Vec<SectionSite> = across
                            .into_iter()
                            .map(|c| SectionSite::on_plane([row, c]))
                            .chain(down.into_iter().map(|r| SectionSite::on_plane([r, col])))
                            .collect();
                        if path.iter().all(|&s| grounded_on(&before, &ground, s)) {
                            joins.extend(path.into_iter().map(|s| PaintedSite::new(s, hue)));
                        }
                    }
                }
                for p in joins {
                    paint(&mut canvas, p.site, p.symbol);
                }
            }
        }
    }
    let out: SectionSample = match (framed, window) {
        // A frame act's output is the output; a crop of the input's frame
        // does not cut it.
        (Some(g), _) => g,
        (None, None) => canvas,
        (None, Some((low, high))) => cropped(&canvas, low, high)?,
    };
    let out = match scale {
        None => out,
        Some((up, down)) => scaled(&out, up, down)?,
    };
    let painted = crate::transducer::raster::painted_sites(&out);
    let [rows, cols] = out.extents();
    Ok(Spoken { frame: (rows, cols), painted })
}

/// THE CANVAS CROPPED to the window from `low` to `high`, both corners
/// included, read in place. A window that reaches past the canvas keeps the
/// part on it; one that holds no site is refused by name.
fn cropped(canvas: &SectionSample, low: SectionSite, high: SectionSite) -> Result<SectionSample, Unstated> {
    // Each axis's extent from the low corner to the high one, the high one
    // held on the canvas's last site along that axis.
    let mut extents = [0usize; ALPHABET_SAMPLE_RANK];
    for (((e, lo), hi), whole) in extents.iter_mut().zip(low.coordinates()).zip(high.coordinates()).zip(canvas.extents()) {
        let last = i64::try_from(whole.checked_sub(1).ok_or(Unstated::NotOne)?).map_err(|_| Unstated::NotOne)?;
        let hi = if hi > last { last } else { hi };
        *e = hi.checked_sub(lo).and_then(|d| usize::try_from(d).ok()).and_then(|d| d.checked_add(1)).ok_or(Unstated::NotOne)?;
    }
    let origin = canvas.at(low).ok_or(Unstated::NotOne)?;
    let mut out = SectionSample::filled(extents, origin).map_err(|_| Unstated::NotOne)?;
    let corner = low.coordinates();
    let mut writes = Vec::with_capacity(out.symbols().len());
    for (site, _) in out.sites() {
        let from = site.moved(corner).ok_or(Unstated::OffSubstrate)?;
        writes.push((site, canvas.at(from).ok_or(Unstated::OffSubstrate)?));
    }
    for (site, h) in writes {
        out.paint(site, h).ok_or(Unstated::OffSubstrate)?;
    }
    Ok(out)
}

/// 78.167 C — WHAT EACH PREDICATE SELECTS on one scene, as a clause reads
/// it (the scene seen once): `None` where the predicate is unstated there.
/// The act reading separates an effect's bodies by these and nothing else.
pub(crate) fn select_each(preds: &[Pred], x: &SectionSample, bodies: &[Body]) -> Vec<Option<Vec<usize>>> {
    let seen = Seen::of(x, bodies);
    let all = seen.all();
    preds.iter().map(|p| seen.select(p, &all).ok()).collect()
}

/// THE REGION SEVERAL BODIES NAME TOGETHER, as a fill selecting them speaks
/// it (`Seen::region` over the set): the set's joint interior is every ground
/// cell the set walls in, which no one of them may.
pub(crate) fn region_of_set(region: RegionKind, x: &SectionSample, bodies: &[Body], set: &[usize]) -> Vec<SectionSite> {
    Seen::of(x, bodies).region(region, set)
}

/// The bodies of the scene owning a site beside one of `sites`, corners
/// included, each once, in the scene's order.
pub(crate) fn bodies_around(x: &SectionSample, bodies: &[Body], sites: &[SectionSite]) -> Vec<usize> {
    let seen = Seen::of(x, bodies);
    let mut out: Vec<usize> = Vec::new();
    let around = around();
    for &site in sites {
        for &d in &around {
            if let Some(o) = seen.step(site, d).and_then(|s| seen.owner_at(s)) {
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
pub(crate) fn partner_hues(x: &SectionSample, bodies: &[Body]) -> Vec<Option<u8>> {
    let seen = Seen::of(x, bodies);
    (0..bodies.len()).map(|b| seen.partner_hue(b, &Pred::All).ok()).collect()
}

/// 78.167 C — the cells a region names about each ONE body of the scene, as
/// a fill would paint them: the act reading asks which bodies' regions the
/// created cells lie in.
pub(crate) fn regions_each(region: RegionKind, x: &SectionSample, bodies: &[Body]) -> Vec<Vec<SectionSite>> {
    let seen = Seen::of(x, bodies);
    (0..bodies.len()).map(|b| seen.region(region, &[b])).collect()
}

/// THE GRID SCALED BY `up / down`, one of them 1: each cell an `up × up`
/// block, or each `down × down` block, uniform, one cell. A block that is
/// not uniform, an extent `down` does not divide, a zero or a ratio of two
/// factors is no scaling of cells, refused by name.
fn scaled(g: &SectionSample, up: u8, down: u8) -> Result<SectionSample, Unstated> {
    let (up, down) = (usize::from(up), usize::from(down));
    let [rows, cols] = g.extents();
    // Each site of the scaled sample reads the site of `g` it stands in (a
    // block of `up`), or each block of `down` reads one uniform symbol.
    let extents = match (up, down) {
        (1, 1) => return Ok(g.clone()),
        (u, 1) if u > 1 => [rows * u, cols * u],
        (1, d) if d > 1 && rows % d == 0 && cols % d == 0 => [rows / d, cols / d],
        _ => return Err(Unstated::OffSubstrate),
    };
    let from = |bi: usize, bj: usize| -> Option<u8> {
        match down {
            1 => read(g, bi / up, bj / up),
            d => {
                let h = read(g, bi * d, bj * d)?;
                let uniform = (0..d).all(|i| (0..d).all(|j| read(g, bi * d + i, bj * d + j) == Some(h)));
                uniform.then_some(h)
            }
        }
    };
    let origin = read(g, 0, 0).ok_or(Unstated::OffSubstrate)?;
    let mut out = SectionSample::filled(extents, origin).map_err(|_| Unstated::OffSubstrate)?;
    let mut writes = Vec::with_capacity(out.symbols().len());
    for (site, _) in out.sites() {
        let [i, j] = site.indices().ok_or(Unstated::OffSubstrate)?;
        writes.push((site, from(i, j).ok_or(Unstated::OffSubstrate)?));
    }
    for (site, h) in writes {
        out.paint(site, h).ok_or(Unstated::OffSubstrate)?;
    }
    Ok(out)
}
