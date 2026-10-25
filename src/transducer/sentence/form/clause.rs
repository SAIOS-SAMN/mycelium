// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.143 — THE RULE LANGUAGE GROWS: SELECT, REGION, CREATE, COUNT, COMPARE.
//!
//! A word vertex says "every body of this exact chart moves by one motion and
//! has its hues remapped" (COGNITION.md §1.2). A CLAUSE says it of the bodies
//! a PREDICATE selects — a typed predicate over each perceived body's own
//! exact coordinates — and its act may RECOLOUR or CARRY them, FILL a region
//! their boundary encloses, CREATE cells relative to them (a line to the
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
//! of the form, speaks them on every pair it replays (`family::statement`),
//! and judges the speech by closure as it judges any sentence.

use crate::core::Q;
use crate::transducer::address::{put_key_bytes, put_key_count, take_key_bytes, take_key_count};
use crate::transducer::cga::Mv;
use crate::transducer::stalk;

/// AN EXACT COORDINATE OF A PERCEIVED BODY that a predicate may read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coord {
    /// The shell: its cell count.
    Shell,
    /// The rows its support spans.
    Height,
    /// The columns its support spans.
    Width,
    /// Its frame position: the least row it stands on.
    Top,
    /// The least column.
    Left,
    /// The greatest row.
    Bottom,
    /// The greatest column.
    Right,
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
pub enum Cmp {
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
    Cmp { coord: Coord, cmp: Cmp, to: Value },
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
        Pred::Cmp { coord, cmp: Cmp::Eq, to: Value::Integer(n) }
    }

    /// The extremum of the coordinate in the named order, ties as named.
    pub fn extremum(coord: Coord, order: Order, tie: Tie) -> Pred {
        Pred::Rank { coord, order, place: 0, tie }
    }
}

/// A REGION READ FROM THE BLADE'S SUPPORT, addressable as a target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
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
    /// At the grid's border, painting every cell it crosses.
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

/// WHERE A CARRIAGE'S MOTION IS READ: about the grid's frame, or about each
/// selected body's own centre.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum About {
    Grid,
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

/// WHAT A CLAUSE DOES TO ITS SELECTION.
#[derive(Clone, Debug)]
pub enum Act {
    /// A hue map on the selected bodies' cells; a hue sent to 0 is erased.
    Recolour(Vec<(u8, u8)>),
    /// The selected bodies carried by a versor of `Cl(4,1)(ℚ)` — taken from
    /// where they stand, painted where their compartments land — with a hue
    /// map on what lands.
    Carry { motion: Mv, about: About, gauge: Vec<(u8, u8)> },
    /// CREATE: the region's cells painted.
    Fill { region: Region, hue: u8 },
    /// CREATE: from every cell of each selected body, a line along the ray
    /// `(dr, dc)` to its stop.
    Extend { ray: (i8, i8), paint: Paint, stop: Stop },
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
    /// About the grid, one motion for every body; about each body's own
    /// centre, a mirror or a turn of the body's own frame carried along
    /// (the auditor's directive, 2026-10-20: rotated or mirrored echoes).
    Echo { motion: Mv, about: About, gauge: Vec<(u8, u8)> },
    /// FRAME, THE PANES (78.167 Phase B, Step 4): the input split along its
    /// rows (`along_rows`) or its columns into two equal panes — a separator
    /// line of one cell between them where `gap` — and the output the pane's
    /// frame: a cell painted `paint` where `table` holds of whether each pane
    /// is painted there (bit `2a + b` for panes painted `a`, `b`), the ground
    /// otherwise. The truth table is the act; AND, OR, XOR and NOR are names
    /// for four of them.
    Panes { along_rows: bool, gap: bool, table: u8, paint: u8 },
    /// FRAME, THE TILES (78.167 Phase B, Step 4): the output `rows × cols`
    /// blocks of the input's frame, block `i` (in the lattice's order) the
    /// input under the square's symmetry `turns[i]`: 0 itself, 1 mirrored
    /// across its columns, 2 across its rows, 3 a turn of 180°, 4 a turn of
    /// 90°, 5 of 270°, 6 the transpose, 7 the anti-transpose. A quarter turn,
    /// a transpose of an input that is not square is no block of its frame.
    Tile { rows: u8, cols: u8, turns: Vec<u8> },
    /// CREATE, BETWEEN, ROUND A CORNER (the auditor's directive, 2026-10-20:
    /// Manhattan bent connectors): from each selected body standing on one
    /// row to each body `to` selects standing on one column, the path along
    /// the first body's row to the second's column — the corner — and along
    /// that column to it, painted `paint`, where every cell of it was ground
    /// before the clause. A pair already facing along a row or a column is no
    /// bend (that is `Connect`'s).
    Bend { to: Pred, paint: Paint },
    /// A REACTION — THE IMPULSE (ruled 2026-10-25): each selected body
    /// carried along the unit ray `ray` until its next step would leave the
    /// grid or, at `Stop::Contact`, meet a painted cell; the bodies nearest
    /// their stop first, so a stack settles as it lands. Its extent is read
    /// off the meet, so it holds where the distance differs pair to pair —
    /// a fixed carriage fits each pair and fails the held-out one.
    Slide { ray: (i8, i8), stop: Stop },
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

/// THE DEEPEST NESTING A DECODED PREDICATE MAY HAVE: a tape arriving from a
/// peer is bounded by its own bytes, and a recursion on it by this (§2.4 — a
/// court that does not come back is a court an adversary stops).
const NESTING: usize = 32;

fn put_coord(buf: &mut Vec<u8>, c: Coord) {
    let tag = match c {
        Coord::Shell => 0,
        Coord::Height => 1,
        Coord::Width => 2,
        Coord::Top => 3,
        Coord::Left => 4,
        Coord::Bottom => 5,
        Coord::Right => 6,
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
        0 => Coord::Shell,
        1 => Coord::Height,
        2 => Coord::Width,
        3 => Coord::Top,
        4 => Coord::Left,
        5 => Coord::Bottom,
        6 => Coord::Right,
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

fn take_byte(k: &[u8], p: &mut usize) -> Option<u8> {
    let b = *k.get(*p)?;
    *p += 1;
    Some(b)
}

fn put_pred(buf: &mut Vec<u8>, pred: &Pred) {
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
                Cmp::Eq => 0,
                Cmp::Less => 1,
                Cmp::Greater => 2,
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

fn take_pred(k: &[u8], p: &mut usize, depth: usize) -> Option<Pred> {
    if depth > NESTING {
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
                0 => Cmp::Eq,
                1 => Cmp::Less,
                2 => Cmp::Greater,
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

fn put_gauge(buf: &mut Vec<u8>, gauge: &[(u8, u8)]) {
    let mut g = gauge.to_vec();
    g.sort_unstable();
    put_key_count(buf, g.len());
    for (a, b) in g {
        buf.push(a);
        buf.push(b);
    }
}

fn take_gauge(k: &[u8], p: &mut usize) -> Option<Vec<(u8, u8)>> {
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

fn put_act(buf: &mut Vec<u8>, act: &Act) {
    match act {
        Act::Recolour(map) => {
            buf.push(0);
            put_gauge(buf, map);
        }
        Act::Carry { motion, about, gauge } => {
            buf.push(1);
            motion.put_canonical(buf);
            buf.push(match about {
                About::Grid => 0,
                About::Body => 1,
            });
            put_gauge(buf, gauge);
        }
        Act::Fill { region, hue } => {
            buf.push(2);
            buf.push(match region {
                Region::Interior => 0,
                Region::Halo => 1,
                Region::Extent => 2,
            });
            buf.push(*hue);
        }
        // The paint in the bytes `put_paint` writes, which are the bytes an
        // extend was first written with: no form already minted moves.
        Act::Extend { ray, paint, stop } => {
            buf.push(3);
            buf.extend_from_slice(&ray.0.to_le_bytes());
            buf.extend_from_slice(&ray.1.to_le_bytes());
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
        // An echo about the grid keeps the tag and bytes it was first
        // written with, so no form already minted moves its address; one
        // about each body is its own tag.
        Act::Echo { motion, about, gauge } => {
            buf.push(match about {
                About::Grid => 9,
                About::Body => 12,
            });
            motion.put_canonical(buf);
            put_gauge(buf, gauge);
        }
        Act::Panes { along_rows, gap, table, paint } => {
            buf.push(10);
            buf.push(u8::from(*along_rows));
            buf.push(u8::from(*gap));
            buf.push(*table);
            buf.push(*paint);
        }
        Act::Tile { rows, cols, turns } => {
            buf.push(11);
            buf.push(*rows);
            buf.push(*cols);
            buf.extend_from_slice(turns);
        }
        Act::Bend { to, paint } => {
            buf.push(13);
            put_pred(buf, to);
            put_paint(buf, paint);
        }
        Act::Slide { ray, stop } => {
            buf.push(14);
            buf.extend_from_slice(&ray.0.to_le_bytes());
            buf.extend_from_slice(&ray.1.to_le_bytes());
            put_stop(buf, *stop);
        }
        Act::Repaint { paint } => {
            buf.push(15);
            put_paint(buf, paint);
        }
    }
}

fn put_paint(buf: &mut Vec<u8>, paint: &Paint) {
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

fn take_paint(k: &[u8], p: &mut usize) -> Option<Paint> {
    match take_byte(k, p)? {
        0 => Some(Paint::Own),
        1 => Some(Paint::Hue(take_byte(k, p)?)),
        2 => Some(Paint::Of(take_pred(k, p, 0)?)),
        _ => None,
    }
}

fn put_stop(buf: &mut Vec<u8>, stop: Stop) {
    buf.push(match stop {
        Stop::Border => 0,
        Stop::Contact => 1,
    });
}

fn take_stop(k: &[u8], p: &mut usize) -> Option<Stop> {
    match take_byte(k, p)? {
        0 => Some(Stop::Border),
        1 => Some(Stop::Contact),
        _ => None,
    }
}

fn take_act(k: &[u8], p: &mut usize) -> Option<Act> {
    Some(match take_byte(k, p)? {
        0 => Act::Recolour(take_gauge(k, p)?),
        1 => {
            let motion = Mv::from_bytes(k, p)?;
            let about = match take_byte(k, p)? {
                0 => About::Grid,
                1 => About::Body,
                _ => return None,
            };
            Act::Carry { motion, about, gauge: take_gauge(k, p)? }
        }
        2 => {
            let region = match take_byte(k, p)? {
                0 => Region::Interior,
                1 => Region::Halo,
                2 => Region::Extent,
                _ => return None,
            };
            Act::Fill { region, hue: take_byte(k, p)? }
        }
        3 => {
            let dr = i8::from_le_bytes([take_byte(k, p)?]);
            let dc = i8::from_le_bytes([take_byte(k, p)?]);
            let paint = take_paint(k, p)?;
            Act::Extend { ray: (dr, dc), paint, stop: take_stop(k, p)? }
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
                9 => About::Grid,
                _ => About::Body,
            };
            Act::Echo { motion, about, gauge: take_gauge(k, p)? }
        }
        10 => {
            let along_rows = match take_byte(k, p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let gap = match take_byte(k, p)? {
                0 => false,
                1 => true,
                _ => return None,
            };
            let table = take_byte(k, p)?;
            if table > 0b1111 {
                return None;
            }
            Act::Panes { along_rows, gap, table, paint: take_byte(k, p)? }
        }
        11 => {
            let (rows, cols) = (take_byte(k, p)?, take_byte(k, p)?);
            let n = usize::from(rows).checked_mul(usize::from(cols))?;
            let mut turns = Vec::with_capacity(n);
            for _ in 0..n {
                let t = take_byte(k, p)?;
                if t > 7 {
                    return None;
                }
                turns.push(t);
            }
            Act::Tile { rows, cols, turns }
        }
        13 => Act::Bend { to: take_pred(k, p, 0)?, paint: take_paint(k, p)? },
        14 => {
            let dr = i8::from_le_bytes([take_byte(k, p)?]);
            let dc = i8::from_le_bytes([take_byte(k, p)?]);
            Act::Slide { ray: (dr, dc), stop: take_stop(k, p)? }
        }
        15 => Act::Repaint { paint: take_paint(k, p)? },
        _ => return None,
    })
}

/// THE CLAUSES AS THE FORM'S ADDRESS WRITES THEM, in the rule's own order
/// (the order is content: a later clause paints over an earlier one).
pub(crate) fn put_clauses(buf: &mut Vec<u8>, clauses: &[Clause]) {
    put_key_count(buf, clauses.len());
    for c in clauses {
        put_pred(buf, &c.select);
        put_act(buf, &c.act);
    }
}

/// The inverse of [`put_clauses`]. `None` where the bytes are no run of
/// clauses: a form whose statements do not read is no form (rule 11).
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
