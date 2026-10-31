// 78.75 IN n DIMENSIONS — THE WHITE-HOLE SOURCE AT ANY OBSERVED DIMENSION (ruled
// 2026-10-17: an n-dimensional source is valid, limited at the source only by
// the observed dimensional capacity). The organ `source`
// states in `Cl(4,1)`, read on the conformal model of ℚⁿ for whatever `n` the
// caller observed — S⁶⁰, S³⁶⁰ — and its outflow answered plane by plane, so a
// flow staged on many planes gives each its own founding phase (with one phase
// shared on two planes, the founding's capture splits kinds by parity, and no
// world forms).
//
//! EVERY OBJECT OF THE LAW IS OF GRADE ONE OR TWO: the pivot `P`, the geoid's
//! vector `σ` and the velocity `v` are vectors; the lift `L`, its keeping part
//! `K` and the source `N` are bivectors. So the law is carried on those two
//! grades alone — `n + 2` exact coordinates a vector (the `n` base axes, then
//! `e₊` of square +1 and `e₋` of square −1), a bivector its non-zero
//! components by axis pair — and never on the algebra's `2ⁿ⁺²` blades, which
//! no observed dimension of this size could hold. No dimension is a constant:
//! `n` is read off the coordinates given.
//!
//! THE LAW, EXACT:
//! - the part of `L` keeping the geoid, `K = ½(L + σLσ⁻¹)`; reflection in `σ`
//!   carries each factor `x` to `2(x·σ)σ/σ² − x`, so `σLσ⁻¹ = L −
//!   2σ∧(σ·L)/σ²` and `K = L − σ∧(σ·L)/σ²` — verified to commute with `σ`
//!   (`σ·K = 0`);
//! - at the pivot `P` on the geoid (`P·σ = 0`, `P² = 0`), its velocity
//!   `v = P·K`;
//! - the source `N = P ∧ v`, verified null (`N² = (P·v)² − P²v² = 0`), fixing
//!   `P` (`P·N = 0`) and keeping `σ` (`σ·N = 0`);
//! - ITS FLOW BY PLANE: on each base plane `(e₂ᵢ₋₁, e₂ᵢ)`, `N`'s component
//!   `P₂ᵢ₋₁v₂ᵢ − P₂ᵢv₂ᵢ₋₁` — how much outflow the plane carries — and `v`'s
//!   in-plane components `(v₂ᵢ₋₁, v₂ᵢ)` — where in it the matter leaves, the
//!   direction a phase reading decides (L9). An odd last base axis pairs with
//!   no base axis; its flow is in `N` whole, on the planes it shares with `e₊`
//!   and `e₋`.
//!
//! Every claim is checked on the value before it is returned and refused by
//! name otherwise (rule 11): nothing is asserted the algebra does not hold, and
//! nothing is approximated. On `n = 3` the organ is the `Cl(4,1)` source
//! (`source::Source::of`), coefficient for coefficient.

use num_rational::BigRational;
use num_traits::{One, Zero};

type Q = BigRational;

/// WHY THE LAW REFUSED, BY NAME.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutflowRefusal {
    /// Fewer than three coordinates: no base axis beside `e₊`, `e₋`.
    NoBase,
    /// Two objects of different dimension.
    Dimension,
    /// A bivector names an axis past its coordinates.
    Axis,
    /// The pivot is the zero vector, or not null.
    NotNull,
    /// `σ² = 0`: a point, not a sphere.
    NoSphere,
    /// The pivot is not on the geoid: `P·σ ≠ 0`.
    OffGeoid,
    /// The kept generator does not move the pivot: `v = 0`.
    Still,
    /// A check of the law failed on the value: the part kept does not
    /// commute with `σ`, or `N` is not null, moves `P`, or leaves `σ`.
    Unverified,
}

/// THE SQUARE OF AXIS `a` IN `n` BASE AXES: +1 for a base axis and `e₊`
/// (index `n`), −1 for `e₋` (index `n + 1`).
fn square(n: usize, a: usize) -> Q {
    match a == n + 1 {
        true => -Q::one(),
        false => Q::one(),
    }
}

/// A VECTOR OF THE CONFORMAL MODEL OF ℚⁿ: its `n + 2` exact coordinates, the
/// base axes first, then `e₊`, then `e₋`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vector {
    coords: Vec<Q>,
}

impl Vector {
    /// The vector of `coords`; `n = coords.len() − 2` read off them. Refused
    /// where there is no base axis.
    pub fn new(coords: Vec<Q>) -> Result<Vector, OutflowRefusal> {
        match coords.len() >= 3 {
            true => Ok(Vector { coords }),
            false => Err(OutflowRefusal::NoBase),
        }
    }

    /// The number of base axes.
    pub fn n(&self) -> usize {
        self.coords.len() - 2
    }

    /// The coordinates, base axes first, then `e₊`, `e₋`.
    pub fn coords(&self) -> &[Q] {
        &self.coords
    }

    pub fn is_zero(&self) -> bool {
        self.coords.iter().all(Zero::is_zero)
    }

    /// THE INNER PRODUCT, in the model's metric. Refused across dimensions.
    pub fn dot(&self, o: &Vector) -> Result<Q, OutflowRefusal> {
        if o.n() != self.n() {
            return Err(OutflowRefusal::Dimension);
        }
        let n = self.n();
        Ok(self.coords.iter().zip(&o.coords).enumerate().fold(Q::zero(), |acc, (a, (x, y))| acc + x * y * square(n, a)))
    }

    fn scaled(&self, s: &Q) -> Vector {
        Vector { coords: self.coords.iter().map(|x| x * s).collect() }
    }
}

/// A NULL POINT: a non-zero vector of square zero, checked on entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NullPoint(Vector);

impl NullPoint {
    pub fn new(coords: Vec<Q>) -> Result<NullPoint, OutflowRefusal> {
        let v = Vector::new(coords)?;
        match !v.is_zero() && v.dot(&v)?.is_zero() {
            true => Ok(NullPoint(v)),
            false => Err(OutflowRefusal::NotNull),
        }
    }

    pub fn vector(&self) -> &Vector {
        &self.0
    }
}

/// A BIVECTOR OF THE CONFORMAL MODEL OF ℚⁿ: its non-zero components on
/// `eᵢ ∧ eⱼ`, `i < j`, ascending by the pair — axes counted from 0, the base
/// axes first, then `e₊` (`n`) and `e₋` (`n + 1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bivector {
    n: usize,
    terms: Vec<(usize, usize, Q)>,
}

impl Bivector {
    /// The bivector `Σ c eᵢ ∧ eⱼ` over `n` base axes: a pair given `i > j` is
    /// `−c eⱼ ∧ eᵢ`, one with `i = j` is zero, pairs given twice add. Refused
    /// where an axis is past the coordinates or there is no base axis.
    pub fn new(n: usize, terms: Vec<(usize, usize, Q)>) -> Result<Bivector, OutflowRefusal> {
        if n == 0 {
            return Err(OutflowRefusal::NoBase);
        }
        let mut out: Vec<(usize, usize, Q)> = Vec::with_capacity(terms.len());
        for (i, j, c) in terms {
            if i > n + 1 || j > n + 1 {
                return Err(OutflowRefusal::Axis);
            }
            let (i, j, c) = match i < j {
                true => (i, j, c),
                false => (j, i, -c),
            };
            if i == j {
                continue;
            }
            match out.binary_search_by_key(&(i, j), |(a, b, _)| (*a, *b)) {
                Ok(at) => out[at].2 += c,
                Err(at) => out.insert(at, (i, j, c)),
            }
        }
        out.retain(|(_, _, c)| !c.is_zero());
        Ok(Bivector { n, terms: out })
    }

    /// The number of base axes.
    pub fn n(&self) -> usize {
        self.n
    }

    /// The non-zero components, ascending by `(i, j)`, `i < j`.
    pub fn terms(&self) -> &[(usize, usize, Q)] {
        &self.terms
    }

    /// The component on `eᵢ ∧ eⱼ` (`i < j`; the other order is its negative).
    pub fn component(&self, i: usize, j: usize) -> Q {
        let (a, b, sign) = match i < j {
            true => (i, j, Q::one()),
            false => (j, i, -Q::one()),
        };
        match self.terms.binary_search_by_key(&(a, b), |(x, y, _)| (*x, *y)) {
            Ok(at) => &self.terms[at].2 * sign,
            Err(_) => Q::zero(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    /// THE OUTER PRODUCT `a ∧ b`: on each pair `i < j`, `aᵢbⱼ − aⱼbᵢ`.
    pub fn wedge(a: &Vector, b: &Vector) -> Result<Bivector, OutflowRefusal> {
        if a.n() != b.n() {
            return Err(OutflowRefusal::Dimension);
        }
        let d = a.coords.len();
        let mut terms = Vec::new();
        for i in 0..d {
            for j in (i + 1)..d {
                let c = &a.coords[i] * &b.coords[j] - &a.coords[j] * &b.coords[i];
                if !c.is_zero() {
                    terms.push((i, j, c));
                }
            }
        }
        Ok(Bivector { n: a.n(), terms })
    }

    /// THE LEFT CONTRACTION `x·B`: `x·(eᵢ ∧ eⱼ) = (x·eᵢ) eⱼ − (x·eⱼ) eᵢ`.
    pub fn contracted_by(&self, x: &Vector) -> Result<Vector, OutflowRefusal> {
        if x.n() != self.n {
            return Err(OutflowRefusal::Dimension);
        }
        let n = self.n;
        let mut out = vec![Q::zero(); n + 2];
        for (i, j, c) in &self.terms {
            out[*j] += c * &x.coords[*i] * square(n, *i);
            out[*i] -= c * &x.coords[*j] * square(n, *j);
        }
        Ok(Vector { coords: out })
    }

    fn sub(&self, o: &Bivector) -> Result<Bivector, OutflowRefusal> {
        let mut terms = self.terms.clone();
        terms.extend(o.terms.iter().map(|(i, j, c)| (*i, *j, -c.clone())));
        Bivector::new(self.n, terms)
    }
}

/// THE PART OF `l` THAT KEEPS THE GEOID `sigma`: `½(L + σLσ⁻¹) = L −
/// σ∧(σ·L)/σ²`, verified to commute with `σ` (`σ·K = 0`). Refused where `σ`
/// is null (a point, not a sphere) or the dimensions differ.
pub fn keeping(sigma: &Vector, l: &Bivector) -> Result<Bivector, OutflowRefusal> {
    let s2 = sigma.dot(sigma)?;
    if s2.is_zero() {
        return Err(OutflowRefusal::NoSphere);
    }
    let across = Bivector::wedge(sigma, &l.contracted_by(sigma)?.scaled(&(Q::one() / s2)))?;
    let k = l.sub(&across)?;
    match k.contracted_by(sigma)?.is_zero() {
        true => Ok(k),
        false => Err(OutflowRefusal::Unverified),
    }
}

/// A WHITE-HOLE SOURCE IN n DIMENSIONS: the pivot it emanates from, the
/// velocity the kept generator gives it (`v = P·K`), and its null generator
/// `N = P ∧ v`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceN {
    pub at: NullPoint,
    pub velocity: Vector,
    pub generator: Bivector,
}

/// ONE BASE PLANE'S FLOW: the plane, the outflow the source carries on it
/// (`N`'s component), and where in it the matter leaves — the pivot's
/// velocity in the plane, `(v₂ᵢ₋₁, v₂ᵢ)`, exact, for a phase reading to
/// decide its sector (L9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlaneFlow {
    pub plane: Plane,
    pub outflow: Q,
    pub direction: (Q, Q),
}

/// ONE BASE PLANE `(e₂ᵢ₋₁, e₂ᵢ)`, its axes counted from 1 as the law writes
/// them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Plane {
    pub first: usize,
    pub second: usize,
}

impl SourceN {
    /// THE SOURCE AT `p` ON THE GEOID `sigma`, pushed by the kept generator
    /// `k`: `N = P ∧ (P·K)`, verified non-zero, null, fixing `P` and keeping
    /// `σ`; refused by name otherwise.
    pub fn of(sigma: &Vector, p: &NullPoint, k: &Bivector) -> Result<SourceN, OutflowRefusal> {
        let pv = p.vector();
        if sigma.dot(sigma)?.is_zero() {
            return Err(OutflowRefusal::NoSphere);
        }
        if !pv.dot(sigma)?.is_zero() {
            return Err(OutflowRefusal::OffGeoid);
        }
        let v = k.contracted_by(pv)?;
        if v.is_zero() {
            return Err(OutflowRefusal::Still);
        }
        let n = Bivector::wedge(pv, &v)?;
        // N² = (P·v)² − P²v² for the blade P ∧ v; P² = 0 was checked at entry.
        let pdv = pv.dot(&v)?;
        let null = !n.is_zero() && (&pdv * &pdv - pv.dot(pv)? * v.dot(&v)?).is_zero();
        let fixes = n.contracted_by(pv)?.is_zero();
        let keeps = n.contracted_by(sigma)?.is_zero();
        match null && fixes && keeps {
            true => Ok(SourceN { at: p.clone(), velocity: v, generator: n }),
            false => Err(OutflowRefusal::Unverified),
        }
    }

    /// ITS FLOW BY PLANE: on each base plane `(e₂ᵢ₋₁, e₂ᵢ)`, the generator's
    /// component `P₂ᵢ₋₁v₂ᵢ − P₂ᵢv₂ᵢ₋₁` and the velocity's in-plane
    /// components `(v₂ᵢ₋₁, v₂ᵢ)`, every plane in order; a plane the source
    /// does not turn reads zero.
    pub fn planes(&self) -> Vec<PlaneFlow> {
        let v = self.velocity.coords();
        (0..self.generator.n / 2)
            .map(|i| PlaneFlow {
                plane: Plane { first: 2 * i + 1, second: 2 * i + 2 },
                outflow: self.generator.component(2 * i, 2 * i + 1),
                direction: (v[2 * i].clone(), v[2 * i + 1].clone()),
            })
            .collect()
    }
}

/// THE FLOW BY PLANE OF THE SOURCE A LIFT `l` OPENS AT `p` ON THE GEOID
/// `sigma` (the module's law): the part of `l` keeping the geoid, the source
/// it opens at `p`, and that source's outflow and direction on every base
/// plane.
pub fn outflow_planes(sigma: &Vector, p: &NullPoint, l: &Bivector) -> Result<Vec<PlaneFlow>, OutflowRefusal> {
    let k = keeping(sigma, l)?;
    Ok(SourceN::of(sigma, p, &k)?.planes())
}
