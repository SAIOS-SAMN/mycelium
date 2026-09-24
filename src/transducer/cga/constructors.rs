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

use super::motion::motion_modes;
use super::multivector::Mv;
use crate::network::nerve::coord::BivectorKind;
use crate::network::nerve::Keyed;
use num_traits::Zero;

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
