// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.143 — THE RULE LANGUAGE GROWS: SELECT, REGION, CREATE, COUNT, COMPARE.
//!
//! A word vertex says "every body of this exact chart moves by one motion and
//! has its hues remapped" (MU-COGNITION.md §2.2). A CLAUSE says it of the bodies
//! a PREDICATE selects — a typed predicate over each perceived body's own
//! exact coordinates — and its act may RECOLOUR or CARRY them, FILL a region
//! the selected bodies' boundary encloses, CREATE cells relative to them (a line to the
//! border, a template stamped onto a seed), or CROP the output to one of
//! them. Counts and comparisons are predicates over a measured exact
//! coordinate, in the order the rule NAMES, ties included (Concordance 1.10):
//! a tie the rule's order does not name is held whole or refused by name,
//! never broken silently, and a coordinate that is not exact (a radius that
//! is no rational, a μ lane) is refused by name.
//!
//! The clauses are a coordinate of the FORM: written on its address after
//! the remainder's coordinates, and only where there are any, so every form
//! with none keeps the address it had. The court decodes them with the rest
//! of the form, speaks them on every pair it reconstructs (`family::statement`),
//! and judges the speech by closure as it judges any sentence.

use crate::core::Q;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use crate::transducer::alphabet::ALPHABET_SAMPLE_RANK;
use crate::transducer::cga::Mv;
pub use crate::transducer::perspective::PlaneAxis;
use crate::transducer::stalk;

/// AN EXACT COORDINATE OF A PERCEIVED BODY that a predicate may read. The
/// coordinates along the plane name an axis of the perspective's plane by
/// its role ([`PlaneAxis`], MU-SENSING §3.6), never a row or a column.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coord {
    /// Its cell count.
    Cells,
    /// The sites its support spans along the plane's axis, both ends
    /// included.
    Extent(PlaneAxis),
    /// Its frame position along the plane's axis: the least coordinate it
    /// stands on.
    Least(PlaneAxis),
    /// The greatest coordinate it stands on along the plane's axis.
    Greatest(PlaneAxis),
    /// How many distinct hues its gauge carries.
    Hues,
    /// How many other bodies it touches (two bodies meet at a corner, never
    /// along an edge, or they would be one body).
    Contacts,
    /// How many background cells its boundary alone encloses.
    Enclosed,
    /// How many cells of the scene carry its one hue; no reading on a body
    /// of several hues.
    Tally,
    /// How many bodies of the scene stand on its chart (its class), itself
    /// included.
    Peers,
    /// The radius of its dual sphere, `√(S·S)`: exact only where `S·S` is a
    /// rational square, and refused by name where it is not.
    Radius,
    /// A μ lane — never a coordinate a rule reads (Concordance 1.10's third
    /// guard); named so that a rule naming it is refused by name.
    Lane(u8),
}

/// A COMPARISON THE RULE NAMES.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    Eq,
    Less,
    Greater,
}

/// THE ORDER A RANK READS, named by the rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Greatest,
    Least,
}

/// WHAT A RANK DOES WITH A TIE, named by the rule: hold every tied body,
/// refuse, or read a further named coordinate in a further named order (and
/// refuse if that ties too). Never broken silently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    Hold,
    Refuse,
    Then(Coord, Order),
}

/// WHAT A COMPARISON IS AGAINST: an exact integer, or AS MANY AS the bodies
/// another predicate selects in the scene.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Integer(i64),
    Count(Box<Pred>),
}

/// A TYPED PREDICATE over a body's own coordinates. Evaluated over a domain
/// of bodies: `And` narrows it predicate by predicate (so a rank inside an
/// `And` ranks the bodies the predicates before it selected), `Not` takes
/// the domain's complement of its operand, `Any` the union.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pred {
    All,
    /// The body carries this hue.
    Hue(u8),
    /// The body stands on this chart — its class up to moduli.
    Chart(stalk::Invariant),
    /// COUNT: the coordinate compared with a value.
    Cmp { coord: Coord, cmp: Comparison, to: Value },
    /// COMPARE: the bodies at the `place`-th distinct value of the coordinate
    /// in the named order (0 the extremum), ties as named.
    Rank { coord: Coord, order: Order, place: u32, tie: Tie },
    /// The body touches a body the predicate selects (in the whole scene).
    Touches(Box<Pred>),
    Not(Box<Pred>),
    And(Vec<Pred>),
    Any(Vec<Pred>),
}

impl Pred {
    /// The coordinate equals `n` exactly.
    pub fn count(coord: Coord, n: i64) -> Pred {
        Pred::Cmp { coord, cmp: Comparison::Eq, to: Value::Integer(n) }
    }

    /// The extremum of the coordinate in the named order, ties as named.
    pub fn extremum(coord: Coord, order: Order, tie: Tie) -> Pred {
        Pred::Rank { coord, order, place: 0, tie }
    }
}

/// A REGION READ FROM THE BLADE'S SUPPORT, addressable as a target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionKind {
    /// The background cells the selection's boundary encloses: every
    /// background cell no edge-connected path reaches from the border
    /// without crossing a selected body (a body inside stays inside).
    Interior,
    /// The contact neighbourhood: background cells touching a selected
    /// body, corners included.
    Halo,
    /// The background cells inside a selected body's extent.
    Extent,
}

/// WHERE A CREATED LINE STOPS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    /// At the sample's outer seam, painting every cell it crosses.
    Border,
    /// At the first painted cell it meets.
    Contact,
}

/// THE HUE A CREATED CELL TAKES: the cell it grows from, one named, or the
/// hue of the body it touches that a predicate selects (ruled 2026-10-25 —
/// a REACTION's hue: read off the partner, never fixed, so it holds where
/// the partner's hue differs pair to pair). None selected, or bodies of two
/// hues, is refused by name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Paint {
    Own,
    Hue(u8),
    Of(Pred),
}

/// WHERE A CARRIAGE'S MOTION IS READ: about the sample's own frame, its
/// sites as the sample writes them, or about each selected body's own
/// centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum About {
    Sample,
    Body,
}

/// WHAT A CROP KEEPS of its one selected body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extent {
    /// The body's own extent.
    Body,
    /// The extent of the cells its boundary encloses.
    Interior,
}

/// THE RAY OF A SLIDE OR AN EXTEND (ruled 2026-11-06 on #32 (5)): a
/// translation of the sample's own rank, one step per axis, `n` components
/// for a sample of rank `n`. No plane is assumed: a ray of the alphabet's
/// section has the section's rank of steps because the section has that
/// rank, never because a ray is a pair. Its versor is the translator by its
/// steps ([`Ray::versor`]), which is what the lowered element carries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ray {
    steps: Vec<i8>,
}

impl Ray {
    /// The ray of these steps, one per axis.
    pub fn new(steps: Vec<i8>) -> Ray {
        Ray { steps }
    }

    /// The steps, one per axis.
    pub fn steps(&self) -> &[i8] {
        &self.steps
    }

    /// The rank the ray is of: its count of steps.
    pub fn rank(&self) -> usize {
        self.steps.len()
    }

    /// THE TRANSLATION VERSOR of the ray, of its own rank: the translator by
    /// its steps (`cga::translator_at`).
    pub fn versor(&self) -> Mv {
        let d: Vec<Q> = self.steps.iter().map(|&s| Q::from_integer(i64::from(s).into())).collect();
        crate::transducer::cga::translator_at(&d)
    }

    /// Written: the count of steps by its own coordinate, then each step a
    /// byte.
    fn put(&self, buf: &mut Vec<u8>) {
        put_key_count(buf, self.steps.len());
        buf.extend(self.steps.iter().map(|s| s.to_le_bytes()[0]));
    }

    /// Read back, or `None` where the bytes do not carry the count.
    fn take(k: &[u8], p: &mut usize) -> Option<Ray> {
        let n = take_key_count(k, p)?;
        if n > k.len().checked_sub(*p)? {
            return None;
        }
        let mut steps = Vec::with_capacity(n);
        for _ in 0..n {
            steps.push(i8::from_le_bytes([take_byte(k, p)?]));
        }
        Some(Ray { steps })
    }
}

/// WHAT A CLAUSE DOES TO ITS SELECTION.
#[derive(Clone, Debug)]
pub enum Act {
    /// A hue map on the selected bodies' cells; a hue sent to 0 is erased.
    Recolour(Vec<(u8, u8)>),
    /// The selected bodies carried by a versor of `Cl(4,1)(ℚ)` — taken from
    /// where they stand, painted where the bodies' compartments land — with a hue
    /// map on what lands.
    Carry { motion: Mv, about: About, gauge: Vec<(u8, u8)> },
    /// CREATE: the region's cells painted.
    Fill { region: RegionKind, hue: u8 },
    /// CREATE: from every cell of each selected body, a line along the ray
    /// (a translation of the sample's rank) to its stop.
    Extend { ray: Ray, paint: Paint, stop: Stop },
    /// CREATE: the ONE body `template` selects, carried centre onto centre
    /// onto each selected body, which it replaces.
    Copy { template: Pred },
    /// SIZE: the output is the extent of the one selected body.
    Crop(Extent),
    /// SIZE: the output scaled by `up / down`, one of them 1 — each cell an
    /// `up × up` block, or each uniform `down × down` block one cell (a
    /// block that is not uniform is no cell, refused by name).
    Scale { up: u8, down: u8 },
    /// ON THE SCENE'S PARTS: the clause's selection read among the scene's
    /// monochrome parts — each hue's 4-connected run within a body — and the
    /// act done to the parts it selects (78.167 Phase A: a body recoloured
    /// in part is parts recoloured whole). One level: an act on parts is not
    /// itself on parts.
    Parts(Box<Act>),
    /// CREATE, BETWEEN (78.167 Phase B, Step 2): along every row and every
    /// column, two consecutive cells of selected bodies, of two different
    /// bodies, are joined through the cells between them where every one of
    /// those was ground before the clause — painted the one hue, or the hue
    /// the two ends share (`Paint::Own`; ends of two hues are not joined).
    Connect { paint: Paint },
    /// CREATE, AGAIN (78.167 Phase B, Step 3): the selected bodies carried by
    /// a versor of `Cl(4,1)(ℚ)` onto the canvas with a hue map on what lands,
    /// the bodies KEPT where they stood — a carriage that leaves its source.
    /// About the sample, one motion for every body; about each body's own
    /// centre, a mirror or a turn of the body's own frame carried along
    /// (ruled 2026-10-20: rotated or mirrored echoes).
    Echo { motion: Mv, about: About, gauge: Vec<(u8, u8)> },
    /// FRAME, THE PANES (78.167 Phase B, Step 4): the input split along the
    /// perspective's plane axis `along` (MU-SENSING §3.6) into two equal
    /// panes — a separator line of one cell between them where `gap` — and
    /// the output the pane's frame: a cell painted `paint` where `table`
    /// holds of whether each pane is painted there (bit `2a + b` for panes
    /// painted `a`, `b`), the ground otherwise. The truth table is the act;
    /// AND, OR, XOR and NOR are names for four of them.
    Panes { along: PlaneAxis, gap: bool, table: u8, paint: u8 },
    /// FRAME, THE TILES (78.167 Phase B, Step 4): the output a lattice of
    /// blocks of the input's frame, `blocks` of them along each axis of the
    /// perspective's plane (one count per plane axis, in the plane's order,
    /// [`PlaneAxis::index`]): block `i` (in the lattice's canonical order,
    /// the plane's last axis fastest) the input under the square's symmetry
    /// `turns[i]`: 0 itself, 1 reversed along the plane's second axis, 2
    /// along its first, 3 a turn of 180°, 4 a turn of 90°, 5 of 270°, 6 the
    /// transpose, 7 the anti-transpose. A 90° turn, a transpose of an input
    /// that is not square is no block of its frame. The square's symmetries
    /// are the plane's by their mathematics, read through the perspective.
    Tile { blocks: [u8; ALPHABET_SAMPLE_RANK], turns: Vec<u8> },
    /// CREATE, BETWEEN, ROUND A CORNER (ruled 2026-10-20:
    /// Manhattan bent connectors): from each selected body standing on one
    /// row to each body `to` selects standing on one column, the path along
    /// the first body's row to the second's column — the corner — and along
    /// that column to it, painted `paint`, where every cell of it was ground
    /// before the clause. A pair already facing along a row or a column is no
    /// bend (that is `Connect`'s).
    Bend { to: Pred, paint: Paint },
    /// A REACTION — THE IMPULSE (ruled 2026-10-25): each selected body
    /// carried along the unit ray `ray` (a translation of the sample's own
    /// rank) until its next step would leave the
    /// sample or, at `Stop::Contact`, meet a painted cell; the bodies nearest
    /// the stop first, so a stack settles as it lands. Its extent is read
    /// off the meet, so it holds where the distance differs pair to pair —
    /// a fixed carriage fits each pair and fails the held-out one.
    Slide { ray: Ray, stop: Stop },
    /// A REACTION'S HUE: every cell of each selected body painted `paint`
    /// (`Paint::Of`: the hue of the body it touches).
    Repaint { paint: Paint },
}

/// ONE STATEMENT: a selection and what is done to it.
#[derive(Clone, Debug)]
pub struct Clause {
    pub select: Pred,
    pub act: Act,
}

/// THE DEEPEST PREDICATE THE LANGUAGE MINTS, in levels (MU-DERIVATIONS D4.4
/// (iii)): the vocabulary (`family::acts`) is atoms whose deepest is
/// `Not(Touches(leaf))` (three levels), and a description is at most one
/// meet (`And`) of atoms, one level more. A decoded predicate deeper than any
/// the producer mints is no word of the language and is refused, which
/// bounds every walker of a decoded tree — `Drop`, `Clone`, the codec, the
/// statement's evaluation — at once (§2.4: a court that does not come back
/// is a court an adversary stops).
/// EXACT: take_pred_refuses_past_language_depth
pub(crate) const PRED_HEIGHT: usize = 3 + 1;

/// The clause codec's byte for each coordinate along the plane: one byte per
/// coordinate and axis, the bytes the planar names were written with, so no
/// clause or word address moves.
fn put_coord(buf: &mut Vec<u8>, c: Coord) {
    let tag = match c {
        Coord::Cells => 0,
        Coord::Extent(PlaneAxis::First) => 1,
        Coord::Extent(PlaneAxis::Second) => 2,
        Coord::Least(PlaneAxis::First) => 3,
        Coord::Least(PlaneAxis::Second) => 4,
        Coord::Greatest(PlaneAxis::First) => 5,
        Coord::Greatest(PlaneAxis::Second) => 6,
        Coord::Hues => 7,
        Coord::Contacts => 8,
        Coord::Enclosed => 9,
        Coord::Tally => 10,
        Coord::Peers => 11,
        Coord::Radius => 12,
        Coord::Lane(l) => {
            buf.push(13);
            buf.push(l);
            return;
        }
    };
    buf.push(tag);
}

fn take_coord(k: &[u8], p: &mut usize) -> Option<Coord> {
    let tag = *k.get(*p)?;
    *p += 1;
    Some(match tag {
        0 => Coord::Cells,
        1 => Coord::Extent(PlaneAxis::First),
        2 => Coord::Extent(PlaneAxis::Second),
        3 => Coord::Least(PlaneAxis::First),
        4 => Coord::Least(PlaneAxis::Second),
        5 => Coord::Greatest(PlaneAxis::First),
        6 => Coord::Greatest(PlaneAxis::Second),
        7 => Coord::Hues,
        8 => Coord::Contacts,
        9 => Coord::Enclosed,
        10 => Coord::Tally,
        11 => Coord::Peers,
        12 => Coord::Radius,
        13 => {
            let l = *k.get(*p)?;
            *p += 1;
            Coord::Lane(l)
        }
        _ => return None,
    })
}

/// The panes' codec byte for the plane axis they split along: the byte the
/// split's planar flag was written with (1 along the plane's first axis, 0
/// along its second), so no clause or word address moves.
pub(super) fn panes_byte(along: PlaneAxis) -> u8 {
    match along {
        PlaneAxis::First => 1,
        PlaneAxis::Second => 0,
    }
}

/// The plane axis a panes byte names; `None` for any other byte.
pub(super) fn panes_of_byte(byte: u8) -> Option<PlaneAxis> {
    match byte {
        1 => Some(PlaneAxis::First),
        0 => Some(PlaneAxis::Second),
        _ => None,
    }
}

fn put_order(buf: &mut Vec<u8>, o: Order) {
    buf.push(match o {
        Order::Greatest => 0,
        Order::Least => 1,
    });
}

fn take_order(k: &[u8], p: &mut usize) -> Option<Order> {
    let o = match *k.get(*p)? {
        0 => Order::Greatest,
        1 => Order::Least,
        _ => return None,
    };
    *p += 1;
    Some(o)
}

pub(super) fn take_byte(k: &[u8], p: &mut usize) -> Option<u8> {
    let b = *k.get(*p)?;
    *p += 1;
    Some(b)
}

pub(super) fn put_pred(buf: &mut Vec<u8>, pred: &Pred) {
    match pred {
        Pred::All => buf.push(0),
        Pred::Hue(h) => {
            buf.push(1);
            buf.push(*h);
        }
        Pred::Chart(c) => {
            buf.push(2);
            put_key_bytes(buf, c.key().as_bytes());
        }
        Pred::Cmp { coord, cmp, to } => {
            buf.push(3);
            put_coord(buf, *coord);
            buf.push(match cmp {
                Comparison::Eq => 0,
                Comparison::Less => 1,
                Comparison::Greater => 2,
            });
            match to {
                Value::Integer(n) => {
                    buf.push(0);
                    buf.extend_from_slice(&n.to_le_bytes());
                }
                Value::Count(q) => {
                    buf.push(1);
                    put_pred(buf, q);
                }
            }
        }
        Pred::Rank { coord, order, place, tie } => {
            buf.push(4);
            put_coord(buf, *coord);
            put_order(buf, *order);
            buf.extend_from_slice(&place.to_le_bytes());
            match tie {
                Tie::Hold => buf.push(0),
                Tie::Refuse => buf.push(1),
                Tie::Then(c, o) => {
                    buf.push(2);
                    put_coord(buf, *c);
                    put_order(buf, *o);
                }
            }
        }
        Pred::Touches(q) => {
            buf.push(5);
            put_pred(buf, q);
        }
        Pred::Not(q) => {
            buf.push(6);
            put_pred(buf, q);
        }
        Pred::And(qs) | Pred::Any(qs) => {
            buf.push(if matches!(pred, Pred::And(_)) { 7 } else { 8 });
            put_key_count(buf, qs.len());
            for q in qs {
                put_pred(buf, q);
            }
        }
    }
}

pub(super) fn take_pred(k: &[u8], p: &mut usize, depth: usize) -> Option<Pred> {
    if depth >= PRED_HEIGHT {
        return None;
    }
    let tag = take_byte(k, p)?;
    Some(match tag {
        0 => Pred::All,
        1 => Pred::Hue(take_byte(k, p)?),
        2 => Pred::Chart(stalk::Invariant::from_key(&take_key_bytes(k, p)?)?),
        3 => {
            let coord = take_coord(k, p)?;
            let cmp = match take_byte(k, p)? {
                0 => Comparison::Eq,
                1 => Comparison::Less,
                2 => Comparison::Greater,
                _ => return None,
            };
            let to = match take_byte(k, p)? {
                0 => {
                    let n = i64::from_le_bytes(k.get(*p..*p + 8)?.try_into().ok()?);
                    *p += 8;
                    Value::Integer(n)
                }
                1 => Value::Count(Box::new(take_pred(k, p, depth + 1)?)),
                _ => return None,
            };
            Pred::Cmp { coord, cmp, to }
        }
        4 => {
            let coord = take_coord(k, p)?;
            let order = take_order(k, p)?;
            let place = u32::from_le_bytes(k.get(*p..*p + 4)?.try_into().ok()?);
            *p += 4;
            let tie = match take_byte(k, p)? {
                0 => Tie::Hold,
                1 => Tie::Refuse,
                2 => Tie::Then(take_coord(k, p)?, take_order(k, p)?),
                _ => return None,
            };
            Pred::Rank { coord, order, place, tie }
        }
        5 => Pred::Touches(Box::new(take_pred(k, p, depth + 1)?)),
        6 => Pred::Not(Box::new(take_pred(k, p, depth + 1)?)),
        7 | 8 => {
            let n = take_key_count(k, p)?;
            // Each operand is at least one byte: a count past the bytes left
            // is no run of predicates.
            if n > k.len().checked_sub(*p)? {
                return None;
            }
            let mut qs = Vec::with_capacity(n);
            for _ in 0..n {
                qs.push(take_pred(k, p, depth + 1)?);
            }
            match tag {
                7 => Pred::And(qs),
                _ => Pred::Any(qs),
            }
        }
        _ => return None,
    })
}

pub(super) fn put_gauge(buf: &mut Vec<u8>, gauge: &[(u8, u8)]) {
    let mut g = gauge.to_vec();
    g.sort_unstable();
    put_key_count(buf, g.len());
    for (a, b) in g {
        buf.push(a);
        buf.push(b);
    }
}

pub(super) fn take_gauge(k: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
    let n = take_key_count(k, p)?;
    if n > k.len().checked_sub(*p)? {
        return None;
    }
    let mut g = Vec::with_capacity(n);
    for _ in 0..n {
        g.push((take_byte(k, p)?, take_byte(k, p)?));
    }
    Some(g)
}

pub(super) fn put_act(buf: &mut Vec<u8>, act: &Act) {
    match act {
        Act::Recolour(map) => {
            buf.push(0);
            put_gauge(buf, map);
        }
        Act::Carry { motion, about, gauge } => {
            buf.push(1);
            motion.put_canonical(buf);
            buf.push(match about {
                About::Sample => 0,
                About::Body => 1,
            });
            put_gauge(buf, gauge);
        }
        Act::Fill { region, hue } => {
            buf.push(2);
            buf.push(match region {
                RegionKind::Interior => 0,
                RegionKind::Halo => 1,
                RegionKind::Extent => 2,
            });
            buf.push(*hue);
        }
        // The paint in the bytes `put_paint` writes; the ray its count of
        // steps and each step (track 26, ruled on #32 (5)). A form writes its
        // elements (the ray's translator), so no form already minted moves.
        Act::Extend { ray, paint, stop } => {
            buf.push(3);
            ray.put(buf);
            put_paint(buf, paint);
            put_stop(buf, *stop);
        }
        Act::Copy { template } => {
            buf.push(4);
            put_pred(buf, template);
        }
        Act::Crop(extent) => {
            buf.push(5);
            buf.push(match extent {
                Extent::Body => 0,
                Extent::Interior => 1,
            });
        }
        Act::Scale { up, down } => {
            buf.push(6);
            buf.push(*up);
            buf.push(*down);
        }
        Act::Parts(inner) => {
            buf.push(7);
            put_act(buf, inner);
        }
        Act::Connect { paint } => {
            buf.push(8);
            put_paint(buf, paint);
        }
        // An echo about the sample keeps the tag and bytes it was first
        // written with, so no form already minted moves its address; one
        // about each body is its own tag.
        Act::Echo { motion, about, gauge } => {
            buf.push(match about {
                About::Sample => 9,
                About::Body => 12,
            });
            motion.put_canonical(buf);
            put_gauge(buf, gauge);
        }
        Act::Panes { along, gap, table, paint } => {
            buf.push(10);
            buf.push(panes_byte(*along));
            buf.push(u8::from(*gap));
            buf.push(*table);
            buf.push(*paint);
        }
        // The counts along the plane in the plane's order, one byte each:
        // the bytes the planar pair of counts was written with.
        Act::Tile { blocks, turns } => {
            buf.push(11);
            buf.extend_from_slice(blocks);
            buf.extend_from_slice(turns);
        }
        Act::Bend { to, paint } => {
            buf.push(13);
            put_pred(buf, to);
            put_paint(buf, paint);
        }
        Act::Slide { ray, stop } => {
            buf.push(14);
            ray.put(buf);
            put_stop(buf, *stop);
        }
        Act::Repaint { paint } => {
            buf.push(15);
            put_paint(buf, paint);
        }
    }
}

pub(super) fn put_paint(buf: &mut Vec<u8>, paint: &Paint) {
    match paint {
        Paint::Own => buf.push(0),
        Paint::Hue(h) => {
            buf.push(1);
            buf.push(*h);
        }
        Paint::Of(pred) => {
            buf.push(2);
            put_pred(buf, pred);
        }
    }
}

pub(super) fn take_paint(k: &[u8], p: &mut usize) -> Option<Paint> {
    match take_byte(k, p)? {
        0 => Some(Paint::Own),
        1 => Some(Paint::Hue(take_byte(k, p)?)),
        2 => Some(Paint::Of(take_pred(k, p, 0)?)),
        _ => None,
    }
}

pub(super) fn put_stop(buf: &mut Vec<u8>, stop: Stop) {
    buf.push(match stop {
        Stop::Border => 0,
        Stop::Contact => 1,
    });
}

pub(super) fn take_stop(k: &[u8], p: &mut usize) -> Option<Stop> {
    match take_byte(k, p)? {
        0 => Some(Stop::Border),
        1 => Some(Stop::Contact),
        _ => None,
    }
}

pub(super) fn take_act(k: &[u8], p: &mut usize) -> Option<Act> {
    Some(match take_byte(k, p)? {
        0 => Act::Recolour(take_gauge(k, p)?),
        1 => {
            let motion = Mv::from_bytes(k, p)?;
            let about = match take_byte(k, p)? {
                0 => About::Sample,
                1 => About::Body,
                _ => return None,
            };
            Act::Carry { motion, about, gauge: take_gauge(k, p)? }
        }
        2 => {
            let region = match take_byte(k, p)? {
                0 => RegionKind::Interior,
                1 => RegionKind::Halo,
                2 => RegionKind::Extent,
                _ => return None,
            };
            Act::Fill { region, hue: take_byte(k, p)? }
        }
        3 => {
            let ray = Ray::take(k, p)?;
            let paint = take_paint(k, p)?;
            Act::Extend { ray, paint, stop: take_stop(k, p)? }
        }
        4 => Act::Copy { template: take_pred(k, p, 0)? },
        5 => Act::Crop(match take_byte(k, p)? {
            0 => Extent::Body,
            1 => Extent::Interior,
            _ => return None,
        }),
        6 => Act::Scale { up: take_byte(k, p)?, down: take_byte(k, p)? },
        // One level: a tape nesting parts in parts is no act.
        7 => match take_act(k, p)? {
            Act::Parts(_) => return None,
            inner => Act::Parts(Box::new(inner)),
        },
        8 => Act::Connect { paint: take_paint(k, p)? },
        tag @ (9 | 12) => {
            let motion = Mv::from_bytes(k, p)?;
            let about = match tag {
                9 => About::Sample,
                _ => About::Body,
            };
            Act::Echo { motion, about, gauge: take_gauge(k, p)? }
        }
        10 => {
            let along = panes_of_byte(take_byte(k, p)?)?;
            let gap = match take_byte(k, p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let table = take_byte(k, p)?;
            if table > 0b1111 {
                return None;
            }
            Act::Panes { along, gap, table, paint: take_byte(k, p)? }
        }
        11 => {
            let mut blocks = [0u8; ALPHABET_SAMPLE_RANK];
            let mut n = 1usize;
            for b in blocks.iter_mut() {
                *b = take_byte(k, p)?;
                n = n.checked_mul(usize::from(*b))?;
            }
            let mut turns = Vec::with_capacity(n);
            for _ in 0..n {
                let t = take_byte(k, p)?;
                if t > 7 {
                    return None;
                }
                turns.push(t);
            }
            Act::Tile { blocks, turns }
        }
        13 => Act::Bend { to: take_pred(k, p, 0)?, paint: take_paint(k, p)? },
        14 => {
            let ray = Ray::take(k, p)?;
            Act::Slide { ray, stop: take_stop(k, p)? }
        }
        15 => Act::Repaint { paint: take_paint(k, p)? },
        _ => return None,
    })
}

/// THE CLAUSES AS THE FORM'S ADDRESS WROTE THEM, in the rule's own order
/// (the order is content: a later clause paints over an earlier one). The
/// form writes its elements now (`rule_element::put_elements`), an element
/// an act names in exactly these bytes; this is the reference the claims
/// hold them to.
#[cfg(test)]
pub(crate) fn put_clauses(buf: &mut Vec<u8>, clauses: &[Clause]) {
    put_key_count(buf, clauses.len());
    for c in clauses {
        put_pred(buf, &c.select);
        put_act(buf, &c.act);
    }
}

/// The inverse of [`put_clauses`]. `None` where the bytes are no run of
/// clauses: a form whose statements do not read is no form (rule 11).
#[cfg(test)]
pub(crate) fn take_clauses(k: &[u8], p: &mut usize) -> Option<Vec<Clause>> {
    let n = take_key_count(k, p)?;
    if n > k.len().checked_sub(*p)? {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let select = take_pred(k, p, 0)?;
        let act = take_act(k, p)?;
        out.push(Clause { select, act });
    }
    Some(out)
}

/// The radius `√r²` exactly, where `r²` is a rational square; `None` where
/// it is not — the coordinate is then not exact, and a predicate reading it
/// is refused.
pub(crate) fn exact_root(r2: &Q) -> Option<Q> {
    if r2 < &Q::from_integer(0.into()) {
        return None;
    }
    let (n, d) = (r2.numer(), r2.denom());
    let (rn, rd) = (n.sqrt(), d.sqrt());
    (&rn * &rn == *n && &rd * &rd == *d).then(|| Q::new(rn, rd))
}
