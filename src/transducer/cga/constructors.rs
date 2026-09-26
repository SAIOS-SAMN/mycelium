// 78.12(3), CS1 — THE CONSTRUCTORS ARE THE FOUNDATION: a constructor is a
// reflection, a generator is a product of constructors, a generator carries a
// constructor to a constructor, and the orbit of the constructors under the
// generators they build closes or stays open by the mode that carries one off.
//
//! A CONSTRUCTOR is a rational vector `v` with `v² ∈ ℚ*` — a nucleus, a
//! section's sphere, a mirror (R1). A GENERATOR is a product of them
//! (`versor_of_reflections`, Cartan–Dieudonné; R2): the tether `V = s_b s_a`
//! is two, a circuit's holonomy the product of its members' constructors in
//! the circuit's order. Nothing is parameterised.
//!
//! GENERATORS BUILD CONSTRUCTORS: `Ω v Ω⁻¹` is a vector again with the same
//! square — a nucleus carried round a circuit is a nucleus — so the ORBIT of
//! a set of constructors under a set of generators is a set of constructors,
//! read as rays. It CLOSES when that set is closed under its generators — a
//! finite orbit — and stays OPEN BY NAME where a mode of a generator that
//! never closes (parabolic, hyperbolic, or a turn of no admitted order)
//! moves a constructor: that mode carries it off and the orbit never
//! returns. The closure is a keyed worklist under the stroke's budget, never
//! a search; a spent budget refuses by name.
//!
//! AND THE INVERSE (78.47): a generator SPELLED back into constructors —
//! constructive Cartan–Dieudonné in the algebra's own metric, one factorizer
//! for every caller (the relative pose's Euclidean turn is its positive-
//! definite case). [`construct`] spells a residue mode by mode into the
//! spheres and planes around its congruence factors, each mode certified by
//! its reflections meeting exactly on its fixed locus, the whole by T1.

use super::motion::motion_modes;
use super::multivector::{one, Mv};
use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use num_rational::BigRational;
use num_traits::Zero;

type Q = BigRational;

/// Is `v` a constructor: a pure vector whose square is a non-zero rational.
pub fn is_constructor(v: &Mv) -> bool {
    v.sub(&v.grade(1)).is_zero() && !v.geo(v).scalar_part().is_zero()
}

/// `Ω v Ω⁻¹`: the constructor `v` carried by the generator `Ω` (R2). THE
/// SCALE IS CONTENT (78.28(2), (4)): a constructor's square is its own
/// (`s² = r²` for a sphere), and the carried constructor keeps it, so the
/// sandwich `Ω v Ω̃ = w · Ω v Ω⁻¹` is divided by the generator's weight
/// here, named — never composed with an inverse. `None` where `Ω` is no
/// versor or `v` is not a constructor.
pub fn carry(generator: &Mv, v: &Mv) -> Option<Mv> {
    if !is_constructor(v) {
        return None;
    }
    let w = super::weight_of(generator)?;
    let carried = generator.sandwich(v).scale(&(super::multivector::one() / w));
    is_constructor(&carried).then_some(carried)
}

/// WHAT THE ORBIT OF A SET OF CONSTRUCTORS UNDER A SET OF GENERATORS READ.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstructorOrbit {
    /// Closed under the generators: the distinct constructor rays it holds.
    Closed { rays: usize },
    /// A mode of generator `generator` that never closes moves constructor
    /// `constructor`: it is carried off, by a mode of this class.
    Open { generator: usize, constructor: usize, kind: BivectorKind },
    /// The stroke's budget was spent before the orbit closed: the rays held
    /// when it was (rule 10).
    Unbounded { rays: usize },
    /// The orbit did not read, and why: a constructor no reflection, a
    /// generator no versor, a ray that does not read, a carrying refused.
    Unread(OrbitUnread),
    /// There are no constructors to carry: the cell's nuclei did not read.
    NoConstructors,
}

/// WHY THE CONSTRUCTORS' ORBIT DID NOT READ (CS1, 78.26(4)): a constructor
/// that is no reflection, a generator that is no versor, a constructor with
/// no canonical ray, or a carrying the algebra refused — the carried image
/// no constructor, or with no canonical ray.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrbitUnread {
    NotConstructor { constructor: usize },
    NotVersor { generator: usize },
    NoRay { constructor: usize },
    CarryRefused,
    CarriedNoRay,
}

fn ray_key(v: &Mv) -> Option<Vec<u8>> {
    let mut key = Vec::new();
    v.canonical()?.put_canonical(&mut key);
    Some(key)
}

/// THE ORBIT OF `constructors` UNDER `generators` (CS1). First the modes: a
/// generator's mode that never closes and moves a constructor opens the
/// orbit by name. Then the closure: every constructor carried by every
/// generator, new rays filed by their own coordinates and carried in turn,
/// one read of the stroke's budget per carrying.
pub fn constructor_orbit(
    constructors: &[Mv],
    generators: &[Mv],
    budget: &mut crate::physics::Budget,
) -> ConstructorOrbit {
    match orbit_or_why(constructors, generators, budget) {
        Ok(orbit) => orbit,
        Err(why) => ConstructorOrbit::Unread(why),
    }
}

/// The orbit, or why it does not read.
fn orbit_or_why(
    constructors: &[Mv],
    generators: &[Mv],
    budget: &mut crate::physics::Budget,
) -> Result<ConstructorOrbit, OrbitUnread> {
    if let Some(constructor) = constructors.iter().position(|v| !is_constructor(v)) {
        return Err(OrbitUnread::NotConstructor { constructor });
    }
    if let Some(generator) = generators.iter().position(|g| !super::is_versor(g)) {
        return Err(OrbitUnread::NotVersor { generator });
    }
    for (gi, g) in generators.iter().enumerate() {
        let Some(Ok(modes)) = motion_modes(g, budget) else { continue };
        for m in modes.iter().filter(|m| m.order.is_none()) {
            for (ci, v) in constructors.iter().enumerate() {
                if carry(&m.ray, v).is_some_and(|w| w != *v) {
                    return Ok(ConstructorOrbit::Open { generator: gi, constructor: ci, kind: m.kind });
                }
            }
        }
    }
    let mut held: Keyed<()> = Keyed::new();
    let mut work: Vec<Mv> = Vec::new();
    for (constructor, v) in constructors.iter().enumerate() {
        let k = ray_key(v).ok_or(OrbitUnread::NoRay { constructor })?;
        if !held.contains(&k) {
            held.insert(&k, ());
            work.push(v.clone());
        }
    }
    while let Some(v) = work.pop() {
        for g in generators {
            if !budget.spend() {
                return Ok(ConstructorOrbit::Unbounded { rays: held.len() });
            }
            let w = carry(g, &v).ok_or(OrbitUnread::CarryRefused)?;
            let k = ray_key(&w).ok_or(OrbitUnread::CarriedNoRay)?;
            if !held.contains(&k) {
                held.insert(&k, ());
                work.push(w);
            }
        }
    }
    Ok(ConstructorOrbit::Closed { rays: held.len() })
}

fn eta(metric: &[Q], a: &[Q], b: &[Q]) -> Q {
    metric.iter().zip(a).zip(b).fold(Q::zero(), |acc, ((s, x), y)| acc + s * x * y)
}

/// THE INVERSE OF COMPOSING: CONSTRUCTIVE CARTAN–DIEUDONNÉ. An isometry `r`
/// of the algebra's metric (columns the images of the generators) spelled
/// as reflecting vectors `u₁ … u_m`, `r = H_{u₁} ∘ ⋯ ∘ H_{u_m}`: at column `k`
/// the current image `x` of `e_k` is reflected home by `u = x − e_k`, which
/// fixes the columns already home because `r` is an isometry. Each step
/// forced, no branch; the vectors rational rays. `Err(column)` where a step
/// is NULL (`η(u, u) = 0`): no reflection exists there, refused by name.
fn reflections_of(r: &[Vec<Q>], metric: &[Q]) -> Result<Vec<Vec<Q>>, usize> {
    let d = r.len();
    let mut rest: Vec<Vec<Q>> = (0..d).map(|k| (0..d).map(|i| r[i][k].clone()).collect()).collect();
    let mut us = Vec::new();
    for k in 0..d {
        let mut u = rest[k].clone();
        u[k] -= one();
        if u.iter().all(Zero::is_zero) {
            continue;
        }
        let uu = eta(metric, &u, &u);
        if uu.is_zero() {
            return Err(k);
        }
        for col in rest.iter_mut() {
            let f = eta(metric, &u, col) * Q::from_integer(2.into()) / &uu;
            for (y, ui) in col.iter_mut().zip(&u) {
                *y = &*y - &f * ui;
            }
        }
        us.push(u);
    }
    Ok(us)
}

/// The metric on the generators, read off the geometric product (`e² = ±1`)
/// — never written in: the conformal model's own signature.
fn metric_of(masks: &[usize]) -> Vec<Q> {
    masks.iter().map(|&m| Mv::blade(m, one()).geo(&Mv::blade(m, one())).scalar_part()).collect()
}

/// A MOTION'S ACTION ON THE VECTORS OF `Cl(n+1, 1)` (columns over the
/// algebra's generators, as `action_of` reads them) SPELLED AS CONSTRUCTORS,
/// in composition order, in the algebra's own metric. `Err(column)` at a
/// null step.
fn constructors_of_action(r: &[Vec<Q>], n: u8) -> Result<Vec<Mv>, usize> {
    let masks = super::modes::generators(n);
    let us = reflections_of(r, &metric_of(&masks))?;
    Ok(us.iter().map(|u| masks.iter().zip(u).fold(Mv::zero(), |acc, (&k, x)| acc.add(&Mv::blade(k, x.clone())))).collect())
}

/// A TURN OF THE BASE — an `n × n` isometry on `e₁ … eₙ` — as the conformal
/// motion it is: embedded in the action on `Cl(n+1, 1)` with `e₊`, `e₋` held
/// fixed, and spelled there. One factorizer, one metric, no flat case.
pub(crate) fn constructors_of_base_turn(r: &[Vec<Q>], n: u8) -> Result<Vec<Mv>, usize> {
    let masks = super::modes::generators(n);
    let at = |axis: usize| masks.iter().position(|&m| m == super::base_axis(axis + 1));
    let d = masks.len();
    let mut full: Vec<Vec<Q>> = (0..d).map(|i| (0..d).map(|j| if i == j { one() } else { Q::zero() }).collect()).collect();
    for (i, row) in r.iter().enumerate() {
        for (j, x) in row.iter().enumerate() {
            let (Some(fi), Some(fj)) = (at(i), at(j)) else { return Err(i) };
            full[fi][fj] = x.clone();
        }
    }
    constructors_of_action(&full, n)
}

/// WHY A RESIDUE DOES NOT SPELL AS CONSTRUCTORS — the negative, refused with
/// the class as its content by the caller (78.47(6)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotConstructed {
    /// The residue is no versor: it has no action to spell.
    NotVersor,
    /// The residue is the identity: nothing to construct (a filament
    /// generates nothing).
    NoResidue,
    /// Its modes do not split over ℚ, by name (MR1).
    Undecomposed(super::Undecomposed),
    /// A forced step would reflect in a null vector: the mode and column.
    NullStep { mode: usize, column: usize },
    /// The constructors do not compose back to the residue as one ray (T1).
    DoesNotCompose,
    /// A mode's constructors meet off its fixed locus.
    MeetOffLocus { mode: usize },
}

/// ONE RESIDUE SPELLED AS CONSTRUCTORS: per mode, the reflections that spell
/// it in composition order, and the Gram form of all of them together.
#[derive(Clone, Debug)]
pub struct Constructed {
    pub modes: Vec<super::MotionMode>,
    pub shapes: Vec<Vec<Mv>>,
    pub gram: super::GramReading,
}

impl Constructed {
    /// Every constructor, mode by mode, in composition order.
    pub fn reflections(&self) -> Vec<Mv> {
        self.shapes.iter().flatten().cloned().collect()
    }
}

/// 78.47(1) — A RESIDUE SPELLED AS CONSTRUCTORS AROUND ITS CONGRUENCE
/// FACTORS, under the stroke's budget: each mode (MR1) by
/// [`reflections_of`] in the algebra's metric; each mode CERTIFIED by its
/// constructors meeting exactly on its fixed locus (`∩ u_i^⊥ = ker(R_j − 1)`);
/// the whole a congruent thought where they compose back to the residue as
/// one ray (T1); their Gram form read (GR1).
pub fn construct(residue: &Mv, budget: &mut crate::physics::Budget) -> Result<Constructed, NotConstructed> {
    let ms = match motion_modes(residue, budget) {
        None => return Err(NotConstructed::NotVersor),
        Some(Err(why)) => return Err(NotConstructed::Undecomposed(why)),
        Some(Ok(ms)) => ms,
    };
    if ms.is_empty() {
        return Err(NotConstructed::NoResidue);
    }
    let masks = super::modes::generators(residue.n());
    let metric = metric_of(&masks);
    let d = masks.len();
    let mut shapes = Vec::with_capacity(ms.len());
    for (mode, m) in ms.iter().enumerate() {
        let r = super::action_of(&m.ray).ok_or(NotConstructed::NotVersor)?;
        let us = reflections_of(&r, &metric).map_err(|column| NotConstructed::NullStep { mode, column })?;
        // THE CERTIFICATE: the meet is exactly the mode's fixed locus.
        let meet_rows: Vec<Vec<Q>> = us.iter().map(|u| u.iter().zip(&metric).map(|(x, s)| x * s).collect()).collect();
        let meet = super::nullspace(meet_rows, d);
        let fixed_rows: Vec<Vec<Q>> =
            (0..d).map(|i| (0..d).map(|j| if i == j { &r[i][j] - one() } else { r[i][j].clone() }).collect()).collect();
        let fixed = super::nullspace(fixed_rows, d);
        let on_locus = meet.iter().all(|v| {
            (0..d).all(|i| r[i].iter().zip(v).fold(Q::zero(), |acc, (a, x)| acc + a * x) == v[i])
        });
        if meet.len() != fixed.len() || !on_locus {
            return Err(NotConstructed::MeetOffLocus { mode });
        }
        shapes.push(
            us.iter()
                .map(|u| masks.iter().zip(u).fold(Mv::zero(), |acc, (&k, x)| acc.add(&Mv::blade(k, x.clone()))))
                .collect::<Vec<Mv>>(),
        );
    }
    // A CONGRUENT THOUGHT: the constructors compose back to the residue (T1).
    let product = shapes.iter().flatten().fold(Mv::scalar(one()), |acc, u| acc.geo(u));
    if !product.projective_eq(residue) {
        return Err(NotConstructed::DoesNotCompose);
    }
    let all: Vec<Mv> = shapes.iter().flatten().cloned().collect();
    Ok(Constructed { gram: super::read_gram(&all), modes: ms, shapes })
}
