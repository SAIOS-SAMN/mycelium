// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ROADMAP track 26 S8 (stage 5), MU-SENSING §3.7 *the field's motions*,
//! LAW-FIELD v1 §2 (a motion) — THE FIELD'S MOTIONS AS ONE EXACT GROUP:
//! rotate (turnings on named planes), flip (the reflection in a frame unit,
//! of which the pole swap and the inversion in a plane's membrane are one),
//! deflate and re-inflate (the boost on a frame plane), and stretch and thin
//! (the re-period of one phase, which no versor carries). A motion is the
//! word of its generators, composed by concatenation and reduced only where
//! the reduction is exact; it acts on frame states, on turnings and orbits
//! (by conjugation) and on points, each exactly or refused `NotExact` with
//! the plane it left unheld.

use super::atlas::{Orbit, Transition};
use super::holonomy::Motion;
use super::lift::Point;
use super::refusal::{FieldRefusal, Residue};
use super::turning::{FramePlanes, Meeting, Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::cga::Clifford;
use forms::coordinate::{Axis, Key, Square, DEGREES_PER_TURN};
use forms::passage::{Frame, Poles};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed};

/// ONE FRAME UNIT, `e₊ₖ` or `e₋ₖ`: its square and its plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Unit {
    pub square: Square,
    pub plane: u32,
}

/// A UNIT'S IMAGE: the unit it is carried onto and whether its sign is kept.
type Image = (Unit, bool);

/// A GENERATOR OF THE FIELD'S MOTIONS (MU-SENSING §3.7, the field's motions).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Generator {
    /// ROTATE: turnings on pairwise disjoint turning planes.
    Turn(Motion),
    /// FLIP: the reflection in the frame unit `u`, `u ↦ −u` and every other
    /// unit kept. On `e₋ₖ` it is plane `k`'s pole swap, `x ↦ e₊ₖ·x·e₊ₖ`,
    /// which is the inversion in plane `k`'s membrane ([`Generator::swap`]).
    Flip(Unit),
    /// DEFLATE OR RE-INFLATE: the boost on frame plane `plane` by the exact
    /// gauge `scale`, its poles `n_Z ↦ λ·n_Z` and `n_Q ↦ n_Q/λ`; refused
    /// where it acts for `λ ≤ 0`.
    Boost { plane: u32, scale: Q },
    /// STRETCH OR THIN: the re-period of the phase on axis `axis` (from 0)
    /// from period `from` to period `to`, a site keeping its index. No
    /// versor carries it: it scales the phase's degrees by `from/to`.
    RePeriod { axis: usize, from: u64, to: u64 },
}

impl Generator {
    /// THE POLE SWAP ON FRAME PLANE `k`, the inversion in its membrane: the
    /// flip of `e₋ₖ` (78.172 §1; `forms::passage::Poles::swap`).
    pub fn swap(k: u32) -> Generator {
        Generator::Flip(Unit { square: Square::Negative, plane: k })
    }

    /// The generator undone.
    fn inverse(&self) -> Result<Generator, FieldRefusal> {
        Ok(match self {
            Generator::Turn(m) => Generator::Turn(Motion::of(
                m.turnings().iter().map(|t| Turning::of(t.plane().clone(), -t.degrees().clone())).collect::<Result<Vec<_>, _>>()?,
            )?),
            Generator::Flip(u) => Generator::Flip(*u),
            Generator::Boost { plane, scale } => Generator::Boost { plane: *plane, scale: Q::one() / scale },
            Generator::RePeriod { axis, from, to } => Generator::RePeriod { axis: *axis, from: *to, to: *from },
        })
    }

    /// Is the generator the identity?
    fn is_identity(&self) -> bool {
        match self {
            Generator::Turn(m) => m.turnings().is_empty(),
            Generator::Flip(_) => false,
            Generator::Boost { scale, .. } => scale.is_one(),
            Generator::RePeriod { from, to, .. } => from == to,
        }
    }

    /// `self` THEN `next` AS ONE GENERATOR, where the composition is exact
    /// and one generator holds it; `None` where the two stay apart in the
    /// word. A flip twice is the identity; turnings compose where their
    /// planes are one or disjoint; boosts on one plane multiply; re-periods
    /// of one phase chain.
    fn joined(&self, next: &Generator) -> Option<Generator> {
        match (self, next) {
            (Generator::Turn(a), Generator::Turn(b)) => a.then(b).ok().map(Generator::Turn),
            (Generator::Flip(a), Generator::Flip(b)) if a == b => Some(Generator::Turn(Motion::identity())),
            (Generator::Boost { plane: a, scale: x }, Generator::Boost { plane: b, scale: y }) if a == b => {
                Some(Generator::Boost { plane: *a, scale: x * y })
            }
            (Generator::RePeriod { axis: a, from, to: mid }, Generator::RePeriod { axis: b, from: start, to }) if a == b && mid == start => {
                Some(Generator::RePeriod { axis: *a, from: *from, to: *to })
            }
            _ => None,
        }
    }
}

/// The refusal `NotExact`, naming the plane carried off the named planes.
fn unnamed(plane: &Key) -> FieldRefusal {
    FieldRefusal::NotExact { residue: Residue::Unnamed { plane: plane.clone() } }
}

/// THE NUMBER OF QUARTER TURNS `θ` IS, mod 4, where `θ` is a multiple of
/// 90°; `None` otherwise.
fn quarters(degrees: &Q) -> Option<u8> {
    let n = degrees / Q::from_integer(BigInt::from(DEGREES_PER_TURN / 4));
    match n.is_integer() {
        true => u8::try_from(n.to_integer().mod_floor(&BigInt::from(4))).ok(),
        false => None,
    }
}

/// A TURNING PLANE'S TWO UNITS in its written orientation.
fn named_units(plane: &TurningPlane) -> Result<(Unit, Unit), FieldRefusal> {
    let unit = |a: &Axis| a.square().map(|square| Unit { square, plane: a.index() });
    match plane.key().axes() {
        [a, b] => unit(a).zip(unit(b)).ok_or_else(|| unnamed(plane.key())),
        _ => Err(unnamed(plane.key())),
    }
}

/// WHERE THE TURNING `r` CARRIES THE UNIT `u`: kept where `u` is no unit of
/// `r`'s plane; by quarter turns, from the plane's first unit toward its
/// second, where it is; refused naming `carried` otherwise.
fn by_turning(r: &Turning, u: Unit, carried: &Key) -> Result<Image, FieldRefusal> {
    let (first, second) = named_units(r.plane())?;
    if u != first && u != second {
        return Ok((u, true));
    }
    let quarter = quarters(r.degrees()).ok_or_else(|| unnamed(carried))?;
    Ok(match (u == first, quarter) {
        (_, 0) => (u, true),
        (true, 1) => (second, true),
        (true, 2) => (first, false),
        (true, _) => (second, false),
        (false, 1) => (first, false),
        (false, 2) => (second, false),
        (false, _) => (first, true),
    })
}

/// WHERE A FLIP, A BOOST OR A RE-PERIOD CARRIES THE UNIT `u` (a turning
/// is read per turning, [`by_turning`]): a flip negates its own unit; a
/// boost mixes its plane's two units, so a unit of its plane is carried off
/// every named plane; a re-period moves no unit.
fn by_generator(g: &Generator, u: Unit, carried: &Key) -> Result<Image, FieldRefusal> {
    match g {
        Generator::Turn(m) => m.turnings().iter().try_fold((u, true), |(at, sign), r| {
            let (to, kept) = by_turning(r, at, carried)?;
            Ok((to, sign == kept))
        }),
        Generator::Flip(f) => Ok((u, *f != u)),
        Generator::Boost { plane, scale } => match (u.plane == *plane, scale.is_one()) {
            (true, false) => Err(unnamed(carried)),
            _ => Ok((u, true)),
        },
        Generator::RePeriod { .. } => Ok((u, true)),
    }
}

/// THE PIECES OF ONE GENERATOR AS IT CONJUGATES A TURNING ON `plane`: a
/// motion's turnings one by one, those on `plane` itself or a disjoint plane
/// dropped (they commute with it); any other generator whole.
fn conjugating(g: &Generator, plane: &TurningPlane) -> Result<Vec<Generator>, FieldRefusal> {
    match g {
        Generator::Turn(m) => m
            .turnings()
            .iter()
            .filter_map(|r| match r.plane().meeting(plane) {
                Ok(Meeting::Sharing) => Some(Motion::of(vec![r.clone()]).map(Generator::Turn)),
                Ok(_) => None,
                Err(e) => Some(Err(e)),
            })
            .collect(),
        _ => Ok(vec![g.clone()]),
    }
}

/// A MOTION OF THE FIELD: the word of its generators, the first acting
/// first, reduced where adjacent generators compose exactly into one. The
/// identity is the empty word. Its inverse is the word reversed, each
/// generator undone, so a motion and its inverse compose to the identity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FieldMotion {
    word: Vec<Generator>,
}

impl FieldMotion {
    /// THE IDENTITY, the empty word.
    pub fn identity() -> FieldMotion {
        FieldMotion::default()
    }

    /// THE MOTION OF ONE GENERATOR.
    pub fn of(generator: Generator) -> FieldMotion {
        FieldMotion::identity().then(&FieldMotion { word: vec![generator] })
    }

    /// The reduced word.
    pub fn word(&self) -> &[Generator] {
        &self.word
    }

    /// THIS MOTION, THEN `next`: the words concatenated, each arriving
    /// generator joined to the last held where the two compose exactly into
    /// one, and an identity dropped.
    pub fn then(&self, next: &FieldMotion) -> FieldMotion {
        let mut word = self.word.clone();
        for g in &next.word {
            match word.pop() {
                Some(held) => match held.joined(g) {
                    Some(j) if j.is_identity() => {}
                    Some(j) => word.push(j),
                    None => {
                        word.push(held);
                        if !g.is_identity() {
                            word.push(g.clone());
                        }
                    }
                },
                None if g.is_identity() => {}
                None => word.push(g.clone()),
            }
        }
        FieldMotion { word }
    }

    /// THE MOTION UNDONE. Refused as a turning's constructor refuses.
    pub fn inverse(&self) -> Result<FieldMotion, FieldRefusal> {
        let word = self.word.iter().rev().map(Generator::inverse).collect::<Result<Vec<_>, _>>()?;
        Ok(FieldMotion::identity().then(&FieldMotion { word }))
    }

    /// THE TURNING `t` CARRIED BY THE MOTION, `g·t·g⁻¹`: its plane's two
    /// units carried, its angle kept, negated where the image plane is the
    /// named plane reversed (one unit's sign turned). Refused `NotExact`
    /// naming the plane where a unit is carried onto no frame unit or the
    /// two onto units of opposite squares.
    pub fn carry_turning(&self, frame: &FramePlanes, t: &Turning) -> Result<Turning, FieldRefusal> {
        self.word.iter().try_fold(t.clone(), |held, g| {
            conjugating(g, held.plane())?.iter().try_fold(held, |held, piece| {
                let key = held.plane().key().clone();
                let (first, second) = named_units(held.plane())?;
                let ((a, sa), (b, sb)) = (by_generator(piece, first, &key)?, by_generator(piece, second, &key)?);
                if a.square != b.square {
                    return Err(unnamed(&key));
                }
                let degrees = match sa == sb {
                    true => held.degrees().clone(),
                    false => -held.degrees().clone(),
                };
                Turning::of(TurningPlane::of(frame, a.square, a.plane, b.plane)?, degrees)
            })
        })
    }

    /// A MOTION OF TURNINGS CARRIED, each turning carried.
    pub fn carry_motion(&self, frame: &FramePlanes, m: &Motion) -> Result<Motion, FieldRefusal> {
        Motion::of(m.turnings().iter().map(|t| self.carry_turning(frame, t)).collect::<Result<Vec<_>, _>>()?)
    }

    /// AN ORBIT CARRIED: each transition's motion carried, its charts kept
    /// (a chart is an invariant, which an isometry does not move). Refused
    /// as [`FieldMotion::carry_turning`] and [`Orbit::of`] refuse.
    pub fn carry_orbit(&self, frame: &FramePlanes, orbit: &Orbit) -> Result<Orbit, FieldRefusal> {
        Orbit::of(
            orbit
                .revolution()
                .iter()
                .map(|t| Ok(Transition { from: t.from.clone(), to: t.to.clone(), motion: self.carry_motion(frame, &t.motion)? }))
                .collect::<Result<Vec<_>, FieldRefusal>>()?,
        )
    }

    /// THE FRAME STATE `x` ON FRAME PLANE `k` (`poles` its pole basis)
    /// CARRIED, held as its whole `H + R` on `e₊ₖ` and its span `H − R` on
    /// `e₋ₖ`: a flip of `e₋ₖ`, the pole swap, turns the span's sign
    /// (`n_Z ↔ n_Q`); a flip of `e₊ₖ` the whole's; a boost on `k` scales
    /// the poles, `(H, R) ↦ (λH, R/λ)`; a turning keeps it where it moves
    /// neither unit and turns a unit's sign by a half turn. Refused
    /// `NotExact` naming `e_+k-k` where a unit is carried off the plane or
    /// a boost's gauge is not positive, and as the pole basis refuses `x`.
    pub fn carry_state(&self, k: u32, poles: &Poles, x: &Clifford) -> Result<Clifford, FieldRefusal> {
        let axis = |s| Axis::unit(s, k).map_err(FieldRefusal::NotTurningPlane);
        let key = Key::generator(axis(Square::Positive)?, axis(Square::Negative)?).map_err(FieldRefusal::NotTurningPlane)?;
        let read = poles.read(x).map_err(FieldRefusal::Passage)?;
        let (mut whole, mut span) = (read.whole(), read.span());
        for g in &self.word {
            match g {
                Generator::Boost { plane, scale } if *plane == k => {
                    if !scale.is_positive() {
                        return Err(unnamed(&key));
                    }
                    let half = algebra::multivector::half();
                    let (hold, reach) = ((&whole + &span) * &half * scale, (&whole - &span) * &half / scale);
                    (whole, span) = (&hold + &reach, hold - reach);
                }
                _ => {
                    let positive = by_generator(g, Unit { square: Square::Positive, plane: k }, &key)?;
                    let negative = by_generator(g, Unit { square: Square::Negative, plane: k }, &key)?;
                    if positive.0 != (Unit { square: Square::Positive, plane: k }) || negative.0 != (Unit { square: Square::Negative, plane: k }) {
                        return Err(unnamed(&key));
                    }
                    if !positive.1 {
                        whole = -whole;
                    }
                    if !negative.1 {
                        span = -span;
                    }
                }
            }
        }
        let half = algebra::multivector::half();
        poles.state(&Frame { hold: (&whole + &span) * &half, reach: (whole - span) * half }).map_err(FieldRefusal::Passage)
    }

    /// A POINT CARRIED (78.171 §2, a point): each phase a vector at its angle
    /// `θ` on its plane, the plane's second unit turned by `θ` (the phase
    /// origin, `carry_vector`), carried by every generator in turn. A turning on its own plane adds
    /// its angle (subtracts, reversed), a disjoint one keeps it; any other
    /// carries the plane's two units; a re-period of its axis scales its
    /// degrees by `from/to`. The image stands on one of the point's own
    /// planes, the phase moving to that axis; refused `NotExact` naming the
    /// plane otherwise.
    pub fn carry_point(&self, point: &Point) -> Result<Point, FieldRefusal> {
        let planes: Vec<TurningPlane> = point.phases.iter().map(|t| t.plane().clone()).collect();
        let slots = planes.iter().map(named_units).collect::<Result<Vec<_>, _>>()?;
        let mut phases = point.phases.clone();
        for g in &self.word {
            let mut next: Vec<Option<Turning>> = vec![None; phases.len()];
            for (axis, phase) in phases.iter().enumerate() {
                let (slot, degrees) = match g {
                    Generator::RePeriod { axis: a, from, to } if *a == axis => {
                        (axis, phase.degrees() * Q::new(BigInt::from(*from), BigInt::from(*to)))
                    }
                    _ => carry_vector(g, phase, &slots)?,
                };
                let plane = planes.get(slot).ok_or(FieldRefusal::Malformed { at: slot })?;
                if let Some(cell) = next.get_mut(slot) {
                    *cell = Some(Turning::of(plane.clone(), degrees)?);
                }
            }
            phases = next.into_iter().enumerate().map(|(i, p)| p.ok_or(FieldRefusal::Malformed { at: i })).collect::<Result<_, _>>()?;
        }
        Ok(Point { phases })
    }
}

/// ONE PHASE CARRIED BY `g`: the slot (the axis whose plane holds its
/// image) and the image's degrees on that slot's plane.
///
/// THE PHASE ORIGIN (78.171 §8, the convention agreed 2026-11-06 with
/// netstratum docs/117 §11.6): a phase `θ` on the plane `u₁u₂` is the
/// plane's second unit turned by `θ`, `cos θ·u₂ − sin θ·u₁`. On a lifted
/// axis `i` the plane is `e₊ᵢe₊₍ₙ₊₁₎`, so phase 0 lies on the lift's own
/// unit `e₊₍ₙ₊₁₎`, shared by every axis, and the flip of `e₊ᵢ` alone is the
/// reversal `θ ↦ −θ`, leaving every other axis fixed.
///
/// With `u₁ ↦ s₁·U₁` and `u₂ ↦ s₂·U₂` onto a slot `(A, L)`: where `U₁ = A`
/// the image is `θ`, `−θ`, `180° − θ` or `180° + θ` by the signs
/// `(s₁, s₂)`; where `U₁ = L` the two units exchange and it is `270° − θ`,
/// `90° + θ`, `θ − 90°` or `90° − θ`. Exact in degrees, the winding kept.
fn carry_vector(g: &Generator, phase: &Turning, slots: &[(Unit, Unit)]) -> Result<(usize, Q), FieldRefusal> {
    let key = phase.plane().key().clone();
    let theta = phase.degrees().clone();
    let deg = |n: i64| Q::from_integer(BigInt::from(n));
    let (u1, u2) = named_units(phase.plane())?;
    // A turning on the phase's own plane adds its angle; a disjoint one
    // keeps it; one sharing a unit carries the units by quarter turns.
    let mut along = deg(0);
    let mut moved: Vec<Generator> = Vec::new();
    match g {
        Generator::Turn(m) => {
            for r in m.turnings() {
                match r.plane().meeting(phase.plane())? {
                    Meeting::Along => along += r.degrees(),
                    Meeting::Against => along -= r.degrees(),
                    Meeting::Disjoint => {}
                    Meeting::Sharing => moved.push(Generator::Turn(Motion::of(vec![r.clone()])?)),
                }
            }
        }
        _ => moved.push(g.clone()),
    }
    let theta = theta + along;
    let ((a, s1), (b, s2)) = moved.iter().try_fold(((u1, true), (u2, true)), |((a, sa), (b, sb)), piece| {
        let ((a2, ka), (b2, kb)) = (by_generator(piece, a, &key)?, by_generator(piece, b, &key)?);
        Ok::<_, FieldRefusal>(((a2, sa == ka), (b2, sb == kb)))
    })?;
    match slots.iter().position(|s| *s == (a, b)).zip(Some(true)).or_else(|| slots.iter().position(|s| *s == (b, a)).zip(Some(false))) {
        Some((slot, true)) => Ok((
            slot,
            match (s1, s2) {
                (true, true) => theta,
                (true, false) => deg(180) - theta,
                (false, true) => -theta,
                (false, false) => deg(180) + theta,
            },
        )),
        Some((slot, false)) => Ok((
            slot,
            match (s1, s2) {
                (true, true) => deg(270) - theta,
                (true, false) => deg(90) + theta,
                (false, true) => theta - deg(90),
                (false, false) => deg(90) - theta,
            },
        )),
        None => Err(unnamed(&key)),
    }
}
