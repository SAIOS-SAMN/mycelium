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

    /// 78.71 (5) — THE TRANSPORT: per tube, every winding's magnitude weighed
    /// by its mass, `Σ |m|·mass`, summed over the tubes — how much moved, both
    /// ways. Beside [`Tubes::measure`], the net, which cancels flux in against
    /// flux out as a divergence does: forward and back on one tube is net 0
    /// and transport 2. Both are read; neither stands for the other.
    pub fn transport(&self) -> Option<u64> {
        let mut total = 0u64;
        for (_, windings) in self.windings() {
            let moved: BigInt = windings.iter().map(|(m, mass)| m.abs() * BigInt::from(*mass)).sum();
            total = total.checked_add(u64::try_from(moved).ok()?)?;
        }
        Some(total)
    }
}

/// 78.71 (1) — ONE TUBE HELD OVER TIME: its reference flux (the ray), the
/// generator of the field line that opened it (for helicity), its quantum as
/// a ratio to the reference, and — in quanta weighed by mass — the net flux
/// through it, the transport along it, and what is still open on it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tube {
    pub reference: Mv,
    pub generator: Mv,
    pub quantum: BigRational,
    pub net: BigInt,
    pub transport: BigInt,
    pub open: BigInt,
}

/// What admitting one flux did, in quanta weighed by mass: the flux that
/// closed on a known tube (order), the flux that opened a new one (pressure),
/// the open flux that closure relieved, and the transport it was.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Admitted {
    pub closed: u64,
    pub opened: u64,
    pub relieved: u64,
    pub transport: u64,
}

/// 78.71 (1) — THE TUBES HELD OVER TIME, keyed by their null ray: FROZEN IN.
/// A tube has an identity across readings — its ray — so the flux the flow
/// carries along it is conserved and read, not re-derived each time.
///
/// - A flux on a ray no tube holds OPENS one: an open field line, which
///   presses 𝒯 by its mass in quanta (78.71 (2): open flux is pressure).
/// - A flux on a held tube's ray CLOSES on it: the field returns along a known
///   line, which is order, and relieves exactly the open flux that tube held.
/// - A finer flux REFINES the tube's quantum (F4); every held amount is
///   rescaled exactly, so nothing is lost and nothing is declared.
#[derive(Clone, Debug, Default)]
pub struct TubeRegistry {
    tubes: Keyed<Tube>,
}

fn rational_gcd(a: &BigRational, b: &BigRational) -> BigRational {
    BigRational::new(a.numer().gcd(b.numer()), a.denom().lcm(b.denom()))
}

fn amount(winding: &BigInt, mass: u64) -> Option<u64> {
    u64::try_from(winding.abs() * BigInt::from(mass)).ok()
}

impl TubeRegistry {
    pub fn new() -> TubeRegistry {
        TubeRegistry { tubes: Keyed::new() }
    }

    pub fn len(&self) -> usize {
        self.tubes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tubes.is_empty()
    }

    /// ADMIT ONE FLUX carried by the field line whose generator is
    /// `generator`, with `mass` compartments: open a tube, or close on one.
    /// `None` where the flux has no ray, its ratio does not read, or an amount
    /// passes its carrier — nothing is admitted then.
    pub fn admit(&mut self, flux: &Mv, generator: &Mv, mass: u64) -> Option<Admitted> {
        let key = tube_key(flux)?;
        let Some(t) = self.tubes.get(&key).cloned() else {
            let one = BigInt::from(1);
            let opened = amount(&one, mass)?;
            self.tubes.insert(
                &key,
                Tube {
                    reference: flux.clone(),
                    generator: generator.clone(),
                    quantum: BigRational::from_integer(1.into()),
                    net: BigInt::from(mass),
                    transport: BigInt::from(mass),
                    open: BigInt::from(mass),
                },
            );
            return Some(Admitted { closed: 0, opened, relieved: 0, transport: opened });
        };
        let r = cga::projective_ratio(flux, &t.reference).filter(|r| !r.is_zero())?;
        // THE QUANTUM REFINES: the generator of ⟨q, r⟩, and every held amount
        // rescaled by q / q', an exact integer since q' divides q.
        let q2 = rational_gcd(&t.quantum, &r);
        let scale = (&t.quantum / &q2).to_integer();
        let m = (&r / &q2).to_integer();
        let moved = amount(&m, mass)?;
        let open = &t.open * &scale;
        let relieved = (&open).min(&BigInt::from(moved)).clone();
        let tube = Tube {
            reference: t.reference,
            generator: t.generator,
            quantum: q2,
            net: &t.net * &scale + &m * BigInt::from(mass),
            transport: &t.transport * &scale + BigInt::from(moved),
            open: &open - &relieved,
        };
        self.tubes.insert(&key, tube);
        Some(Admitted { closed: moved, opened: 0, relieved: u64::try_from(relieved).ok()?, transport: moved })
    }

    /// 78.71 (4) — THE HELICITY of the tubes held: `Σ_{a ≤ b} net_a · net_b ·
    /// ⟨G_a G_b⟩₀`, each tube's net flux in its quanta and `G` the generator
    /// of the field line that opened it, paired by the scalar part of their
    /// product (the algebra's own invariant form). Pure translations pair to
    /// zero — `e_∞² = 0`, parallel field lines do not link — and a screw's
    /// coupled turn does not. Exact, a rational; never a norm.
    pub fn helicity(&self) -> BigRational {
        let held: Vec<&Tube> = self.tubes.scoped().map(|(_, t)| t).collect();
        let mut h = BigRational::zero();
        for (i, a) in held.iter().enumerate() {
            for b in held.iter().skip(i) {
                let pair = a.generator.geo(&b.generator).scalar_part();
                h += pair * BigRational::from_integer(&a.net * &b.net);
            }
        }
        h
    }

    /// 78.71 (4) — THE LINKS: the pairs of distinct tubes whose field lines
    /// link (`⟨G_a G_b⟩₀ ≠ 0`) while both carry net flux. Helicity moves
    /// whenever flux is injected into a linked tube; the links move only when
    /// the field's topology does — a tube opening across another, or a tube's
    /// net flux cancelling to nothing. A change in this is a reconnection.
    pub fn links(&self) -> usize {
        let held: Vec<&Tube> = self.tubes.scoped().map(|(_, t)| t).filter(|t| !t.net.is_zero()).collect();
        held.iter()
            .enumerate()
            .map(|(i, a)| held.iter().skip(i + 1).filter(|b| !a.generator.geo(&b.generator).scalar_part().is_zero()).count())
            .sum()
    }

    /// The totals over the tubes held, in quanta weighed by mass: net flux,
    /// transport, and what stands open.
    pub fn totals(&self) -> (BigInt, BigInt, BigInt) {
        self.tubes.scoped().fold((BigInt::zero(), BigInt::zero(), BigInt::zero()), |(n, t, o), (_, tube)| {
            (n + &tube.net, t + &tube.transport, o + &tube.open)
        })
    }

    /// F3 — FROZEN IN: the registry carried by the flow `g`, each tube's ray
    /// and generator conjugated, `g X g̃`, its quantum and its amounts kept.
    /// A tube whose carried ray does not read is left where it stood.
    pub fn carried(&self, g: &Mv) -> TubeRegistry {
        let carry = |x: &Mv| g.geo(x).geo(&g.reverse());
        let mut out = TubeRegistry::new();
        for (key, t) in self.tubes.scoped() {
            let reference = carry(&t.reference);
            let k = match tube_key(&reference) {
                Some(k) => k,
                None => key,
            };
            out.tubes.insert(&k, Tube { reference, generator: carry(&t.generator), ..t.clone() });
        }
        out
    }
}
