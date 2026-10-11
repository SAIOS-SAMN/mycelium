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

/// THE INVERSE OF COMPOSING: CONSTRUCTIVE CARTAN–DIEUDONNÉ, on the vectors.
/// An isometry of the algebra's metric, held as its image of each generator,
/// spelled as reflecting vectors `u₁ … u_m`, `r = H_{u₁} ∘ ⋯ ∘ H_{u_m}`: at
/// generator `e_k` its current image `x` is reflected home by `u = x − e_k`,
/// every image reflected with it (`y ↦ y − 2⟨u y⟩₀ u / u²`, the vector part of
/// `−u y u⁻¹`), which fixes the generators already home because `r` is an
/// isometry. Each step forced, no branch; the vectors rational rays.
///
/// 78.114 — IN THE INDEFINITE METRIC, THE FIRST NON-NULL STEP. `u = x − e` is
/// null exactly where `x·e = e²`, and a translation's and a dilation-about-a-
/// point's images do it at once (read live: 64–317 residues a voice refused
/// here). Cartan–Dieudonné's own induction is the remedy: any `y` with
/// `u = (R − 1)y` non-null is fixed by reflecting in `u`, every vector `R`
/// already fixes stays fixed (`⟨u, z⟩ = ⟨Ry, Rz⟩ − ⟨y, z⟩ = 0`), so the fixed
/// space grows each step, and every `u` lies in `im(R − 1)` — whose
/// orthogonal complement IS the fixed locus, so the reflecting hyperplanes
/// meet exactly there (the certificate) and the spelling is minimal
/// (Scherk). `y` is read in the generators' own order, then their pairs:
/// the first whose step is non-null — an order by coordinates, never a
/// choice. `Err(k)` where every remaining step is null: `im(R − 1)` totally
/// isotropic there, which no reflection of it spells — refused by name.
fn reflections_of(r: &super::LinearFunction) -> Result<Vec<Mv>, usize> {
    let mut rest: Vec<Mv> = r.images().to_vec();
    let gens: Vec<Mv> = r.masks().iter().map(|&m| Mv::blade(m, one())).collect();
    let mut us = Vec::new();
    loop {
        let steps: Vec<Mv> = rest.iter().zip(&gens).map(|(x, e)| x.sub(e)).collect();
        let Some(open) = steps.iter().position(|u| !u.is_zero()) else { return Ok(us) };
        let non_null = |u: &Mv| !u.is_zero() && !u.geo(u).scalar_part().is_zero();
        let single = steps.iter().find(|u| non_null(u)).cloned();
        let pair = || {
            (0..steps.len())
                .flat_map(|i| (i + 1..steps.len()).map(move |j| (i, j)))
                .map(|(i, j)| steps[i].add(&steps[j]))
                .find(|u| non_null(u))
        };
        let Some(u) = single.or_else(pair) else { return Err(open) };
        let uu = u.geo(&u).scalar_part();
        reflect_all(&mut rest, &u, &uu);
        us.push(u);
    }
}

/// Every image reflected in `s` (of square `ss`): `y ↦ y − 2⟨s y⟩₀ s / s²`.
fn reflect_all(rest: &mut [Mv], s: &Mv, ss: &Q) {
    let two = Q::from_integer(2.into());
    for y in rest.iter_mut() {
        let f = s.geo(y).scalar_part() * &two / ss;
        *y = y.sub(&s.scale(&f));
    }
}

/// A TURN OF THE BASE — its image of each base axis `e₁ … eₙ`, vectors of the
/// base — as the conformal motion it is: `e₊`, `e₋` held fixed, and spelled
/// in the algebra by the one factorizer. No flat case.
pub(crate) fn constructors_of_base_turn(images: &[Mv], n: u8) -> Result<Vec<Mv>, TurnUnspelled> {
    if images.len() > usize::from(n) {
        return Err(TurnUnspelled::Axis(usize::from(n)));
    }
    let turn = super::LinearFunction::on_generators(n, |mask| {
        match (0..images.len()).find(|&i| super::base_axis(i + 1) == Some(mask)) {
            Some(i) => images[i].clone(),
            None => Mv::blade(mask, one()),
        }
    });
    reflections_of(&turn).map_err(TurnUnspelled::NullStep)
}

/// WHY A TURN OF THE BASE DOES NOT SPELL AS CONSTRUCTORS, each by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TurnUnspelled {
    /// A row of the turn names an axis the algebra at `n` does not have.
    Axis(usize),
    /// A forced step would reflect in a null vector, at this column.
    NullStep(usize),
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
    let mut shapes = Vec::with_capacity(ms.len());
    for (mode, m) in ms.iter().enumerate() {
        let r = super::LinearFunction::of_versor(&m.ray).ok_or(NotConstructed::NotVersor)?;
        let us = reflections_of(&r).map_err(|column| NotConstructed::NullStep { mode, column })?;
        // THE CERTIFICATE: the meet of the reflecting hyperplanes — every
        // vector orthogonal to each `uᵢ`, `⟨uᵢ x⟩₀ = 0` — is exactly the
        // mode's fixed locus `ker(R − 1)`, and every vector of it is held.
        let meet_equations: Vec<Vec<Q>> =
            us.iter().map(|u| r.masks().iter().map(|&k| u.geo(&Mv::blade(k, one())).scalar_part()).collect()).collect();
        let meet: Vec<Mv> =
            crate::core::span::solutions(meet_equations, r.masks().len()).iter().map(|v| r.vector(v)).collect();
        let fixed = r.shifted(&one()).kernel();
        let on_locus = meet.iter().all(|v| r.at(v).sub(v).is_zero());
        if meet.len() != fixed.len() || !on_locus {
            return Err(NotConstructed::MeetOffLocus { mode });
        }
        shapes.push(us);
    }
    // A CONGRUENT THOUGHT: the constructors compose back to the residue (T1).
    let product = shapes.iter().flatten().fold(Mv::scalar(one()), |acc, u| acc.geo(u));
    if !product.projective_eq(residue) {
        return Err(NotConstructed::DoesNotCompose);
    }
    let all: Vec<Mv> = shapes.iter().flatten().cloned().collect();
    Ok(Constructed { gram: super::read_gram(&all), modes: ms, shapes })
}
