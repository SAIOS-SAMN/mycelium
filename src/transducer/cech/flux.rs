// 78.71 — THE TRANSLATION MEASURE AS FLUX: the null part of a cell's
// curvature, grouped into tubes by the ray it fixes, counted in each tube's
// own quantum. No length, no lattice, no straight line.
//
//! A translation's generator `d ∧ e_∞` is null (`B² = 0`). On the conformal
//! sphere it is a rotation about the point at infinity and it has NO length
//! of its own: no conjugation-invariant size, only a direction — the ray it
//! lies on — and ratios to the other generators on that ray. Measuring it by
//! steps on the sensor's grid laid a lattice over the hypersphere (§76.1).
//!
//! So 𝒯 is read the way magnetohydrodynamics reads a field:
//!
//! - **Flux, by Stokes.** A cell's holonomy is its curvature (79.1 R4); its
//!   PARABOLIC modes are the null flux through it (MR1, [`null_fluxes`]).
//!   Elliptic modes are 𝒽's, hyperbolic 𝒮's — never 𝒯.
//! - **Tubes, frozen in.** Fluxes on one null ray are one tube (keyed by the
//!   ray's canonical coordinates); the flow carries a tube whole by
//!   conjugation, so its identity is conserved (79.1 C2a).
//! - **The quantum, read.** A tube's fluxes are rational multiples of one
//!   another (`projective_ratio`); the tube's quantum is the generator of
//!   the ℤ-module they span — the largest flux every one is a whole multiple
//!   of — and each flux is an integer winding of it. Read off the tube's own
//!   fluxes, never declared, and refined, never refused, when a new flux is
//!   finer.

use crate::network::nerve::Keyed;
use crate::transducer::cga::{self, Mv};
use num_bigint::BigInt;
use num_integer::Integer;
use num_rational::BigRational;
use num_traits::{Signed, Zero};

/// THE NULL FLUX THROUGH A CELL: each parabolic mode of its holonomy, read
/// as its generator on §52's chart (`B = ⟨ray⟩₂ / ⟨ray⟩₀`), and kept only
/// where that generator is null as a whole multivector. `None` where the
/// holonomy's modes did not read.
pub fn null_fluxes(holonomy: &Mv, budget: &mut crate::physics::Budget) -> Option<Vec<Mv>> {
    use crate::network::nerve::coord::BivectorKind;
    let Ok(modes) = cga::motion_modes(holonomy, budget)? else { return None };
    Some(
        modes
            .iter()
            .filter(|m| m.kind == BivectorKind::Parabolic)
            .filter_map(|m| crate::transducer::cayley::bivector_of(&m.ray))
            .filter(|b| !b.is_zero() && b.geo(b).is_zero())
            .collect(),
    )
}

/// A TUBE'S KEY: its null ray's canonical coordinates — two fluxes on one
/// ray, at any scale or sign, write one key (A21).
pub fn tube_key(flux: &Mv) -> Option<Vec<u8>> {
    let mut key = Vec::new();
    flux.canonical()?.put_canonical(&mut key);
    Some(key)
}

/// THE TUBES OF A SET OF CELLS: per tube, its reference flux, and each
/// held flux as its rational ratio to that reference, with the mass carried
/// beside it.
#[derive(Clone, Debug, Default)]
pub struct Tubes {
    tubes: Keyed<(Mv, Vec<(BigRational, u64)>)>,
}

impl Tubes {
    pub fn new() -> Tubes {
        Tubes { tubes: Keyed::new() }
    }

    /// ADMIT ONE FLUX carrying `mass` compartments: onto its tube, as its
    /// exact ratio to the tube's reference. A flux on no ray (zero) or one
    /// whose ratio does not read is not admitted, and says so.
    pub fn admit(&mut self, flux: &Mv, mass: u64) -> bool {
        let Some(key) = tube_key(flux) else { return false };
        match self.tubes.get_mut(&key) {
            Some((reference, held)) => match cga::projective_ratio(flux, reference) {
                Some(r) if !r.is_zero() => {
                    held.push((r, mass));
                    true
                }
                _ => false,
            },
            None => {
                self.tubes.insert(&key, (flux.clone(), vec![(BigRational::from_integer(1.into()), mass)]));
                true
            }
        }
    }

    /// How many tubes.
    pub fn len(&self) -> usize {
        self.tubes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tubes.is_empty()
    }

    /// EACH TUBE'S QUANTUM AND WINDINGS: the quantum as a ratio to the tube's
    /// reference — `gcd(numerators) / lcm(denominators)`, the generator of the
    /// ℤ-module the ratios span — and each flux's integer winding of it.
    pub fn windings(&self) -> Vec<(BigRational, Vec<(BigInt, u64)>)> {
        self.tubes
            .scoped()
            .map(|(_, (_, held))| {
                let num = held.iter().fold(BigInt::zero(), |g, (r, _)| g.gcd(r.numer()));
                let den = held.iter().fold(BigInt::from(1), |l, (r, _)| l.lcm(r.denom()));
                let q = BigRational::new(num, den);
                let w = held.iter().map(|(r, m)| ((r / &q).to_integer(), *m)).collect();
                (q, w)
            })
            .collect()
    }

    /// 𝒯 — THE MASS FLUX IN QUANTA: per tube, the net winding through it
    /// (the signed sum: flux in and flux out cancel, as the field's
    /// divergence does) weighed by the mass each flux carried, summed over
    /// the tubes. A reading of the tubes held; `None` past the carrier.
    pub fn measure(&self) -> Option<u64> {
        let mut total = 0u64;
        for (_, windings) in self.windings() {
            let net: BigInt = windings.iter().map(|(m, mass)| m * BigInt::from(*mass)).sum();
            let net = u64::try_from(net.abs()).ok()?;
            total = total.checked_add(net)?;
        }
        Some(total)
    }
}
