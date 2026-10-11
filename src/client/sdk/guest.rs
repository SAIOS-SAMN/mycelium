// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.117 P1 — THE GUEST MATH, STANDARD HALF: the exact constructors any
//! kernel needs to compose a transition the laws can read — motions, bodies
//! carried by them, the fluxes of continuity, and the transition itself,
//! checked before it is sent. Every one is the algebra's own (`cga`,
//! `physics::laws`, law 12): nothing re-derived, nothing approximated. What
//! narrows or overrides these — a domain's charts, its lanes, its own
//! constraints on L0 — is the kernel's, client-side (the operator's split,
//! 2026-10-07).

use crate::physics::laws::{verify_sized, BodyState, Flow, Flux, Transition, Verdict};
use crate::physics::tensegrity::{carried_body, Body};
use crate::transducer::cga::{self, Mv};
use num_rational::BigRational as Q;
use num_traits::{One, Zero};

/// A TRANSLATION by `d`, exact: the translator the fascicle of two parallel
/// planes composes (`cga::translator_at`).
pub fn translate(d: &[Q]) -> Mv {
    cga::translator_at(d)
}

/// A TURN on the plane of base axes `(i, j)`, `i < j` in `1..=3`: the rotor
/// `s + b·eᵢⱼ`, exact and unnormalised (the sandwich carries `s² + b²`, which
/// homogeneity absorbs). `None` for a plane that is no plane of the base, or
/// `s = b = 0`, which is no motion.
pub fn turn(plane: (usize, usize), s: &Q, b: &Q) -> Option<Mv> {
    let (i, j) = plane;
    if !(1 <= i && i < j && j <= 3) || (s.is_zero() && b.is_zero()) {
        return None;
    }
    Some(cga::rotator_on_blade(s, b, cga::base_axis(i)? | cga::base_axis(j)?))
}

/// `first`, then `then`: the motion `then · first`, whose sandwich is `then`'s
/// of `first`'s.
pub fn compose(first: &Mv, then: &Mv) -> Mv {
    then.geo(first)
}

/// The inverse `m̃ / (m m̃)`, where `m m̃` is a non-zero scalar; `None`
/// otherwise — no versor, no inverse.
pub fn inverse(m: &Mv) -> Option<Mv> {
    let n = m.geo(&m.reverse());
    let s = n.scalar_part();
    (!s.is_zero() && n.sub(&Mv::scalar(s.clone())).is_zero()).then(|| m.reverse().scale(&(Q::one() / s)))
}

/// A BODY at `centre` with its mass and second moments, its nucleus the unit
/// sphere there — a point is null, and its inversive distance unread, so the
/// laws read bodies on spheres.
pub fn body_at(centre: Vec<Q>, mass: Q, moments: Vec<Vec<Q>>) -> BodyState {
    let nucleus = cga::sphere_at(&centre, &Q::one());
    BodyState { nucleus, body: Body { centre, mass, moments } }
}

/// A body CARRIED by `motion`: its nucleus by the sandwich, its body by the
/// rigid carriage. `None` where the motion is not rigid — a dilation, a
/// transversion, an inversion carries no body unchanged.
pub fn carry(state: &BodyState, motion: &Mv) -> Option<BodyState> {
    Some(BodyState { nucleus: motion.sandwich(&state.nucleus), body: carried_body(&state.body, motion)? })
}

/// A SOURCE at `at` emitting `mass` into the composition (78.89).
pub fn source(at: &[Q], mass: Q) -> Flux {
    Flux { flow: Flow::Source, at: cga::point_at(at), mass }
}

/// A SINK at `at` absorbing `mass` from the composition (78.89).
pub fn sink(at: &[Q], mass: Q) -> Flux {
    Flux { flow: Flow::Sink, at: cga::point_at(at), mass }
}

/// WHY A BODY WAS NOT ADDED BY `carried`: the motion carries no body rigidly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotRigid;

/// ONE TRANSITION, BUILT: a motion from height `h` to `h + 1`, the bodies it
/// carries, the fluxes of continuity, the hue map. `checked` puts it to the
/// laws locally — the verdict the court gives it (`verify_sized`, 78.116),
/// read before it is sent (P1).
#[derive(Clone, Debug)]
pub struct TransitionBuilder {
    t: Transition,
}

impl TransitionBuilder {
    /// A transition of `motion` from height `height` to the next.
    pub fn new(motion: Mv, height: u64) -> Self {
        // A height past its carrier stands still, and the law of time
        // refuses it by name (rule 11) — never wrapped.
        let next = match height.checked_add(1) {
            Some(n) => n,
            None => height,
        };
        TransitionBuilder { t: Transition { motion, heights: (height, next), bodies: Vec::new(), hues: Vec::new(), fluxes: Vec::new() } }
    }

    /// A body as it stood before and as it stands after.
    pub fn body(mut self, before: BodyState, after: BodyState) -> Self {
        self.t.bodies.push((before, after));
        self
    }

    /// A body as it stood before, and after as the motion carries it.
    pub fn carried(mut self, before: BodyState) -> Result<Self, NotRigid> {
        let after = carry(&before, &self.t.motion).ok_or(NotRigid)?;
        self.t.bodies.push((before, after));
        Ok(self)
    }

    /// A source or sink the transition records.
    pub fn flux(mut self, f: Flux) -> Self {
        self.t.fluxes.push(f);
        self
    }

    /// The hue map the transition carries.
    pub fn hues(mut self, hues: Vec<(u8, u8)>) -> Self {
        self.t.hues = hues;
        self
    }

    /// The transition as built.
    pub fn build(self) -> Transition {
        self.t
    }

    /// The transition, admitted by the laws locally — or the verdict with
    /// every breach named.
    pub fn checked(self) -> Result<Transition, Box<Verdict>> {
        let v = verify_sized(&self.t);
        match v.admitted() {
            true => Ok(self.t),
            false => Err(Box::new(v)),
        }
    }
}
