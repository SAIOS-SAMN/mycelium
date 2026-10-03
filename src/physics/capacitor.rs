// 78.74 — THE FIBRE AS A CAPACITOR, AND TORSION, WHICH DOES NO WORK (ruled
// 2026-10-02, the operator's word: "record 78.74 and build it").
//
//! A FIBRE joins two pivots `Pᵢ`, `Pⱼ` (conformal points, exact) carrying
//! masses `mᵢ`, `mⱼ`. Nothing about its shape is declared; everything is
//! read off the pair:
//!
//! - **ITS SWITCH** `S = mⱼPᵢ − mᵢPⱼ` — a real sphere whose reflection swaps
//!   the two ends; its points are where the weighed pulls balance,
//!   `mⱼ X·Pᵢ = mᵢ X·Pⱼ`, the fibre's own equipotential. Flat (through `e_∞`)
//!   only where the masses balance.
//! - **ITS LEVELS** `S(λ) = λPᵢ − Pⱼ` — the capacitor's equipotentials. Two
//!   compose an exact boost `S(λ₂)S(λ₁)` fixing both pivots (source and
//!   sink) that carries level `μ` to `μ(λ₂/λ₁)²`: charge moves by rational
//!   ratios, never by a logarithm.
//! - **THE SPLIT.** `B = Pᵢ∧Pⱼ` squares to a rational `β² > 0`, so any
//!   generator `G` splits exactly: the FIELD `cB` (`c = ⟨GB⟩₀/β²`, charge
//!   between levels), the TORSION `T` (commutes with `B`, annihilates both
//!   pivots and every level — it does NO WORK across them), and the LIFT `L`
//!   (anticommutes with `B`, moves the pivots — the only part that reshapes
//!   the capacitor).
//! - **THE SHAPE** a set of pivots holds still: the round through them, read
//!   as their outer product, which every fibre switch among them keeps
//!   whatever the masses. Where a further pivot leaves it, no round shape
//!   stands and the shape is what the switches generate — emergent.

use crate::transducer::cga::Mv;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

type Q = BigRational;

/// A FIBRE between two pivots with their masses.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fibre {
    pub pi: Mv,
    pub pj: Mv,
    pub mi: Q,
    pub mj: Q,
    blade: Mv,
    beta2: Q,
}

/// THE THREE PARTS OF A GENERATOR against a fibre, exact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Split {
    /// `c`, the field's coefficient on `B`: charge moved between levels.
    pub field: Q,
    /// Commutes with the fibre and fixes both pivots and every level.
    pub torsion: Mv,
    /// Anticommutes with the fibre: moves the pivots.
    pub lift: Mv,
}

impl Split {
    /// Work is the field and the lift; torsion does none.
    pub fn does_work(&self) -> bool {
        !self.field.is_zero() || !self.lift.is_zero()
    }
}

/// HOW A RELATIVE MOTION TREATS A FIBRE, read on the finite versor itself
/// (no generator taken, so a flip — an odd versor — reads as well as a
/// turn).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carried {
    /// The motion is the identity on the fibre: carried rigidly, no work.
    Rigid,
    /// Both pivots and every level kept, the fibre twisted about its axis
    /// (or mirrored across it): torsion only, no work.
    Torsion,
    /// Both pivots kept, the levels moved: charge moved along the fibre.
    Field,
    /// A pivot moved: the capacitor reshaped.
    Lift,
}

impl Fibre {
    /// The fibre between `pi` and `pj` with their masses. `None` where a mass
    /// is not positive or the pivots do not span a fibre (`β² ≤ 0`: one point,
    /// or not points).
    pub fn new(pi: Mv, pj: Mv, mi: Q, mj: Q) -> Option<Fibre> {
        if !mi.is_positive() || !mj.is_positive() {
            return None;
        }
        let blade = pi.wedge_vector(1, &pj);
        let beta2 = blade.geo(&blade).scalar_part();
        beta2.is_positive().then_some(Fibre { pi, pj, mi, mj, blade, beta2 })
    }

    /// `B = Pᵢ ∧ Pⱼ`.
    pub fn blade(&self) -> &Mv {
        &self.blade
    }

    /// THE SWITCH `mⱼPᵢ − mᵢPⱼ`: the fibre's own equipotential and polarity
    /// switch.
    pub fn switch(&self) -> Mv {
        self.pi.scale(&self.mj).sub(&self.pj.scale(&self.mi))
    }

    /// The level `S(λ) = λPᵢ − Pⱼ`.
    pub fn level(&self, l: &Q) -> Mv {
        self.pi.scale(l).sub(&self.pj)
    }

    /// The step from level `from` to level `to`: an exact boost fixing both
    /// pivots, carrying level `μ` to `μ (to/from)²`.
    pub fn step(&self, from: &Q, to: &Q) -> Mv {
        self.level(to).geo(&self.level(from))
    }

    /// THE SQUARED LENGTH between the pivots, `d² = −2 Pᵢ·Pⱼ` at the points'
    /// own weights — read off the inner product, never a sum of squares.
    pub fn length2(&self) -> Q {
        let weight = |p: &Mv| -p.geo(&crate::transducer::cga::e_inf()).scalar_part();
        let (wi, wj) = (weight(&self.pi), weight(&self.pj));
        -Q::from_integer(2.into()) * self.pi.geo(&self.pj).scalar_part() / (wi * wj)
    }

    /// 78.74 — THE TETHER'S STRENGTH: its binding, `mᵢ mⱼ / d²`, the
    /// inverse-square law, exact. Of two tethers to one node, the stronger
    /// is read by comparing two rationals — never a bool, connected or not.
    pub fn strength(&self) -> Q {
        &self.mi * &self.mj / self.length2()
    }

    /// 78.74 — THE TETHER'S CAPACITY: the reach of its levels, the switch's
    /// own square `S² = mᵢ mⱼ d²`. Strength × capacity = `(mᵢ mⱼ)²`: the two
    /// trade length against binding at fixed masses.
    pub fn capacity(&self) -> Q {
        let s = self.switch();
        s.geo(&s).scalar_part()
    }

    /// 78.74 — THE TORQUE A GENERATOR PUTS ON THIS TETHER, squared: its
    /// torsion part's own quadratic form `⟨T T̃⟩₀`. Torsion lives in the
    /// definite complement of the fibre's plane, so this is a true square —
    /// zero only for no twist — exact, where the whole metric is indefinite
    /// and no other part has a size.
    pub fn torque2(&self, g: &Mv) -> Q {
        let t = self.split(g).torsion;
        t.geo(&t.reverse()).scalar_part()
    }

    /// THE SPLIT of a generator's bivector part into field, torsion and lift.
    pub fn split(&self, g: &Mv) -> Split {
        let g = g.grade(2);
        let unit = Q::from_integer(1.into()) / &self.beta2;
        let mirrored = self.blade.geo(&g).geo(&self.blade).scale(&unit);
        let half = Q::new(1.into(), 2.into());
        let commuting = g.add(&mirrored).scale(&half);
        let lift = g.sub(&commuting);
        let field = g.geo(&self.blade).scalar_part() * &unit;
        let torsion = commuting.sub(&self.blade.scale(&field));
        Split { field, torsion, lift }
    }

    /// HOW THE FINITE MOTION `r` TREATS THIS FIBRE: whether it keeps both
    /// pivots, and if so whether it keeps the levels. Exact, by the sandwich
    /// read projectively — even and odd versors alike.
    pub fn carried_by(&self, r: &Mv) -> Carried {
        let keeps = |x: &Mv| r.sandwich(x).projective_eq(x);
        if r.projective_eq(&Mv::scalar(Q::from_integer(1.into()))) {
            return Carried::Rigid;
        }
        if !keeps(&self.pi) || !keeps(&self.pj) {
            return Carried::Lift;
        }
        match keeps(&self.level(&Q::from_integer(1.into()))) && keeps(&self.level(&Q::from_integer(2.into()))) {
            true => Carried::Torsion,
            false => Carried::Field,
        }
    }
}

/// THE SHAPE A SET OF PIVOTS HOLDS, at base dimension `n`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shape {
    /// Fewer than `n + 1` independent pivots: no round is determined yet.
    Open,
    /// Exactly enough independent pivots to fix a round, and none left to
    /// ask whether it holds: determined, not yet tested — never called
    /// stable, which would be vacuous.
    Determined(Mv),
    /// Every pivot lies on the round through the first `n + 1` independent
    /// ones, and at least one further pivot was asked and lay on it: the
    /// shape stands, read off the pivots, never set.
    Stable(Mv),
    /// A pivot leaves that round: no round shape stands; the shape is what
    /// the fibres' switches generate.
    Emergent,
}

/// THE SHAPE OF `pivots` at base dimension `n`: their outer product built
/// pivot by pivot to the round of grade `n + 1`, and every remaining pivot
/// asked whether it lies on it (`round ∧ X = 0`). Exact, no search.
pub fn shape_of(pivots: &[Mv], n: usize) -> Shape {
    let Ok(top) = u32::try_from(n + 1) else { return Shape::Open };
    let mut round: Option<Mv> = None;
    let mut grade = 0u32;
    let mut rest = pivots.iter();
    for x in rest.by_ref() {
        let next = match &round {
            None => x.clone(),
            Some(r) => r.wedge_vector(grade, x),
        };
        if next.is_zero() {
            continue;
        }
        round = Some(next);
        grade += 1;
        if grade == top {
            break;
        }
    }
    let Some(round) = round.filter(|_| grade == top) else { return Shape::Open };
    let mut tested = false;
    for x in rest {
        if !round.wedge_vector(top, x).is_zero() {
            return Shape::Emergent;
        }
        tested = true;
    }
    match tested {
        true => Shape::Stable(round),
        false => Shape::Determined(round),
    }
}
