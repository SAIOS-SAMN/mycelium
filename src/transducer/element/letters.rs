// 16.4, 1.4 — THE LETTERS: the versor and the alphabet it spells.
//
//! `Versor` is the conformal multivector the action carries: the versors built
//! by name (identity, translator, dilator, carriage), the projective equality,
//! the decomposition into rho and translation, and `Letters`, the M1 coordinates.

use crate::core::Q;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use super::*;

/// THE TRANSPORT of the bundle: a rational conformal versor V ∈ Spin(4,1)(ℚ)
/// (translation, rotation by any rational rotor, reflection, exact dilation,
/// and their products), acting by the sandwich V X Ṽ on the null points of
/// a support. The lattice isometries are the special case; no table of
/// images exists anywhere.
pub type Versor = crate::transducer::cga::Mv;

pub fn identity_versor() -> Versor {
    Versor::scalar(num_rational::BigRational::from_integer(BigInt::from(1)))
}

/// The word's ORIGIN-FIXING action as a CGA versor, no translation. The
/// identity translator is formed here, not at the call site. `None` when
/// the word carries the base off itself.
pub fn versor_of_word(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    base_action_of(&word.rotor())
}

/// THE CONFORMAL REALISATION OF A WORD: the word's base action composed
/// with the carriage by the geometric product. `None` when the word
/// carries the base off itself.
pub fn conformal_of(word: &crate::transducer::alphabet::Word, carriage: &Versor) -> Option<Versor> {
    conformal_of_rotor(&word.rotor(), carriage)
}

/// The same versor from the ACTION alone (L14): a word vertex carries its
/// canonical rotor and the sandwich needs nothing else, so no citation is
/// resolved to speak a body.
pub fn conformal_of_rotor(
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
) -> Option<Versor> {
    Some(carriage.geo(&base_action_of(rotor)?))
}

/// The rotor chart's blades that are the BASE's own even subalgebra
/// `⟨1, e₀₁, e₀₃, e₁₃⟩` — `e₀, e₁, e₃` are `r, c, k` (51.2) — and the
/// versor blades they are read as: `1, e1e2, e1e3, e2e3`.
const BASE_EVEN: [(usize, usize); 4] = [
    (0, 0),
    (0b0011, crate::transducer::cga::E1 | crate::transducer::cga::E2),
    (0b1001, crate::transducer::cga::E1 | crate::transducer::cga::E3),
    (0b1010, crate::transducer::cga::E2 | crate::transducer::cga::E3),
];

/// The half turn `e₀₂`: the orientation character (51.2), which flips `e₀`
/// on the base and nothing else there, and is realised on the base as the
/// reflector `e1`.
const ORIENTATION: usize = 0b0101;

/// THE BASE ACTION OF A ROTOR (51.2, A4): the rotor's action on the three
/// base axes of the chart — `e₀, e₁, e₃`, which are `r, c, k` — as the
/// versor of `Cl(4,1)` on `e1, e2, e3`. Exact, and complete on those axes:
/// an even element of `Cl(4,0)` that carries the base onto itself is, up
/// to scale, a rotor of the base's own even subalgebra `⟨1, e₀₁, e₀₃,
/// e₁₃⟩`, read off blade for blade; or that times the half turn `e₀₂` —
/// the orientation character — realised as the reflector `e1`. A rotor
/// that carries the base off itself (one turning `e₂` into a base axis)
/// has no base action, and says so: nothing of it is projected.
fn base_action_of(rotor: &crate::transducer::alphabet::Rotor) -> Option<Versor> {
    use crate::transducer::alphabet::Rotor;
    let even_on_base = |r: &Rotor| (0..16).all(|m| BASE_EVEN.iter().any(|&(rm, _)| rm == m) || r.coeff(m).is_zero());
    let read = |r: &Rotor| -> Versor {
        BASE_EVEN.iter().fold(Versor::zero(), |v, &(rm, vm)| v.add(&Versor::blade(vm, r.coeff(rm).clone())))
    };
    if rotor.is_identity() {
        return Some(identity_versor());
    }
    if even_on_base(rotor) {
        let v = read(rotor);
        return (!v.is_zero()).then_some(v);
    }
    // `R = R₃ · e₀₂` ⇒ `R₃ = R · e₀₂⁻¹ = −R · e₀₂`, and the base then reads
    // `R₃` with the reflector after it: the sandwich reflects first.
    let r3 = rotor.geo(&Rotor::blade(ORIENTATION, -Q::one()));
    if even_on_base(&r3) {
        let v = read(&r3);
        return (!v.is_zero()).then_some(v.geo(&crate::transducer::cga::reflector()));
    }
    None
}

/// THE ROTOR A VERSOR'S BASE ACTION IS (51.2; N2): `V = D(ρ)·T(t)·R` on
/// the base, so `R = (D·T)⁻¹ V`, read exactly off the carriage `dilator_of`
/// and `translator_of` measure, and mapped back blade for blade onto
/// `⟨1, e₀₁, e₀₃, e₁₃⟩`; a reflector `e1` in it is the orientation
/// character `e₀₂`. `None` when `V` is no versor of the base or `R` is not
/// a rotation or a reflection of it — nothing is projected. The inverse of
/// [`versor_of_word`] on every rotor that has a base action.
pub fn base_rotor_of(v: &Versor) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cga::E1;
    let carriage = dilator_of(v)?.geo(&translator_of(v)?);
    let r = carriage.versor_inverse()?.geo(v);
    let on_base = |m: &Versor| {
        (0..m.slot_count()).all(|b| BASE_EVEN.iter().any(|&(_, vm)| vm == b) || m.coeff(b).is_zero())
    };
    let read = |m: &Versor| -> Rotor {
        BASE_EVEN.iter().fold(Rotor::zero(), |acc, &(rm, vm)| acc.add(&Rotor::blade(rm, m.coeff(vm).clone())))
    };
    if on_base(&r) {
        let rotor = read(&r);
        return rotor.weight().map(|_| rotor);
    }
    // `r = r₃ · e1` ⇒ `r₃ = r · e1`, and the chart reads `r₃ · e₀₂`.
    let r3 = r.geo(&Versor::blade(E1, Q::one()));
    if on_base(&r3) {
        let rotor = read(&r3).geo(&Rotor::blade(ORIENTATION, Q::one()));
        return rotor.weight().map(|_| rotor);
    }
    None
}

/// Projective equality of two versors: the same transport up to a nonzero
/// rational scale (versors act projectively on the cone).
pub fn same_versor(a: &Versor, b: &Versor) -> bool {
    crate::transducer::cga::projective_ratio(a, b).is_some()
}

/// A4 carriage: dilator(ρ) · translator, of a body in frame `0` — its
/// `k` coordinate, not a default (16.4). Sensor pairs stay in `pair_form`,
/// `speak_rule`, and `orbit`.
pub fn carriage(rho: &Q, t: &(Q, Q)) -> Versor {
    carriage_at(rho, &[t.0.clone(), t.1.clone()])
}

/// A4 carriage on the base's own coordinates: dilator(ρ) · translator(d).
pub fn carriage_at(rho: &Q, t: &[Q]) -> Versor {
    crate::transducer::cga::dilator(rho).geo(&crate::transducer::cga::translator_at(t))
}

/// The origin's image under `v`, divided by the dilation `v` carries, on
/// the base axes of `v`'s own algebra: the displacement `v` places the
/// origin at (A4). `None` where the origin goes to infinity or `ρ` cannot
/// be read.
fn origin_image(v: &Versor) -> Option<Vec<Q>> {
    let rho = rho_of(v)?;
    let img = crate::transducer::cga::base_at(&v.sandwich(&crate::transducer::cga::point_at(&[])), usize::from(v.n()))?;
    Some(img.into_iter().map(|x| x / &rho).collect())
}

/// THE DILATION A VERSOR CARRIES (A4, I12), read exactly off its action:
/// every translator and rotor fixes `e_∞` up to its own norm, and the
/// dilator alone scales it — `V e_∞ Ṽ = κ e_∞` with `κ / (V Ṽ) = ρ` for
/// `V = D(ρ) · T · R`. A reflection in `V` (the orientation character, 3.10)
/// negates the ratio and is not a scale, so the magnitude is read. `None`
/// when `V` is not a versor or sends `e_∞` off itself (no conformal action
/// of this form).
pub fn rho_of(v: &Versor) -> Option<Q> {
    let n = crate::transducer::cga::is_versor(v)?;
    let inf = crate::transducer::cga::e_inf();
    let kappa = crate::transducer::cga::projective_ratio(&v.sandwich(&inf), &inf)?;
    let rho = kappa / n;
    if rho.is_zero() {
        return None;
    }
    Some(if rho < Q::zero() { -rho } else { rho })
}

/// Translator component of a carriage `V = D(ρ) · T(t) · R`: the origin's
/// image under the sandwich is `X(ρ t)` — the dilator scales the
/// translation it follows — so `t` is that image divided by ρ. It read the
/// image as `t` itself, which is exact only at ρ = 1: at ρ ≠ 1 the
/// placement residue was ρ times the placement and `dilator_of` was not a
/// dilator (I12). `None` when the origin is sent to infinity or ρ cannot
/// be read.
/// THE DISPLACEMENT A VERSOR CARRIES, as exact coordinates on the base
/// axes of its own algebra (54.7 R3; PH2: no fixed arity).
///
/// [`translator_of`] answers the same reading already WRAPPED in a
/// conformal translator `T(d) = 1 − ½ d e_∞`, whose generator is null
/// (`e_∞² = 0`). A caller that wants the displacement in order to build a
/// HYPERBOLIC motion out of it cannot use that: it would have to unwrap a
/// parabolic to get at the number, and the wrapping is the defect.
///
/// So the displacement is read once, here, and the two constructors —
/// conformal for a lattice pose (51.2, derived), transvection for a
/// connection (51.2's asserted second reading) — are both built from it.
/// One reading, two realisations, rather than one realisation two readers
/// have to agree about (rule 12).
pub fn displacement_of(v: &Versor) -> Option<Vec<Q>> {
    origin_image(v)
}

/// THE SAME DISPLACEMENT AS A GEODESIC TRANSVECTION of `Spin⁺(3,1)(ℚ)`.
///
/// `B = Σ τᵢ (eᵢ ∧ e₋)` with `⟨B²⟩₀ > 0`, evolved to a finite versor by the
/// rational Cayley transform. No `e_∞` is formed, so no null direction
/// enters and the metric degeneracy that forbids an orthogonal eigenplane
/// split never arises.
///
/// A zero displacement is the identity and not a refusal: standing still is
/// a reading.
pub fn transvection_of(v: &Versor) -> Option<Versor> {
    let t = displacement_of(v)?;
    if t.iter().all(|x| x.is_zero()) {
        return Some(identity_versor());
    }
    // THE CAYLEY CHART ENDS AT `|u|² = 1`, AND THAT EDGE IS THE LIGHT CONE
    // (56, ruled 2026-09-22).
    //
    // The generator is `B = u ∧ e₋`, for which `B² = |u|²`, so
    // `(1 + B)(1 + B)~ = 1 − |u|²` and the transform is undefined exactly
    // on the unit sphere. That is NOT a numerical accident. Writing the
    // finite motion as `cosh θ − sinh θ B` gives
    //
    // ```text
    //     cosh θ = (1 + |u|²)/(1 − |u|²),   |u| = tanh(θ/2)
    // ```
    //
    // so `|u| → 1` is `θ → ∞`: the chart covers every FINITE boost and its
    // boundary is infinite rapidity — the luminal limit, where the
    // displacement leaves the timelike sector this reader can carry.
    //
    // Refusing there is right; refusing there ANONYMOUSLY is not. This
    // fell through `cayley`'s generic `None` into `EdgeRefusal::NoReading`,
    // where a displacement at the light cone read the same as a versor
    // that could not be parsed at all — two different facts under one
    // silence (rule 11). The boundary is now named where it is crossed.
    //
    // It is also the second half of why the corpus reads `parabolic 0`:
    // the luminal sector is not merely unbuilt (55.7), it is actively
    // refused here whenever a displacement reaches it.
    //
    // THE GENERATOR IS THE PRODUCT `u e₋` (`u ⊥ e₋`, so `u e₋ = u ∧ e₋`),
    // one boost per coordinate on its own base axis, and `B² = |u|²` is read
    // from it: the signature is the algebra's, never a sum of squares written
    // here (law 12, audit N-2). Written as a blade mask it took the stored
    // blade's orientation, which past the third axis is `e₋ eᵢ = −eᵢ ∧ e₋`.
    use crate::transducer::cga::EM;
    let generator = crate::transducer::cga::base_vector(&t).geo(&Versor::blade(EM, Q::one()));
    if generator.norm2() == Q::one() {
        return None;
    }
    crate::transducer::cga::cayley(&generator)
}

pub fn translator_of(v: &Versor) -> Option<Versor> {
    let t = origin_image(v)?;
    if t.iter().all(|x| x.is_zero()) {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::translator_at(&t))
    }
}

/// Dilator component of a carriage: `D(ρ)` for the ρ the versor carries,
/// the identity at ρ = 1. Exact at every ρ; it was `v · T⁻¹` with `T` read
/// wrong at ρ ≠ 1, which left a translation inside the "dilator".
pub fn dilator_of(v: &Versor) -> Option<Versor> {
    let rho = rho_of(v)?;
    if rho.is_one() {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::dilator(&rho))
    }
}

/// The base's three axes in the rotor chart (51.2): `e₀, e₁, e₃` are `r,
/// c, k`, and `e₂` is off the base. Index 2 of a rotor's action is the row
/// that says a turn leaves the base, and it is a row of the system here,
/// never dropped.
const BASE_AXES: [usize; 3] = [0, 1, 3];

/// THE BASE POINT OF A TURN (12.6; `cech::classify`'s standing note): a
/// rotation about `p` rather than the origin is `T_p R T_p⁻¹ = T(p − R p)
/// · R`, so its carriage is not a residue — it is WHERE the turn stands,
/// and `p` is determined by `R` and the translation together. `Some(p)`
/// when the carriage is exactly `T(p − R p)` for some rational `p`; `None`
/// when it is not — a dilation, a walk, or a turn that leaves the base.
///
/// It stood unread, and that is why a turning body never filed a period.
/// `classify` will only read a finite order off a FLAT carriage, and every
/// transport a voice measures arrives turning about its body's own
/// centroid: `versor_coset` builds each member as `T(c_b)·D(ρ)·R·T(−c_a)`,
/// so the translation is always there and every turn classified as a
/// standing class instead. Reading `p` loses nothing law 12 would keep —
/// it is a FUNCTION of `R` and `t`, not a coordinate beside them — while
/// refusing to read it lost the turn itself.
pub fn turn_base_point(
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
) -> Option<[Q; 3]> {
    // ONE READER (law 12): the base point is the screw whose twist is zero.
    // It answered `None` for a walk, and the walk's SLIP — the thing that
    // makes it a walk — went with it.
    let s = screw_of(rotor, carriage)?;
    match s.twist.iter().all(num_traits::Zero::is_zero) {
        true => Some(s.centre),
        false => None,
    }
}

/// THE CARTAN SPLIT OF A TRANSPORT: where it BENDS, and how far it SLIPS.
///
/// `(I − R) p = t` was read as a yes-or-no — solvable and the turn has a
/// base point, unsolvable and it is "a screw", filed as an undifferentiated
/// free generator. The unsolvable part of `t` is not noise. It is the
/// BURGERS VECTOR of a dislocation, and throwing it away is law 7 at the
/// one place the engine measures torsion in Cartan's sense.
///
/// For an ORTHOGONAL `R` the two halves are exact and orthogonal
/// complements: `ker(I − R) = ker(I − R⁻¹) = ker((I − R)ᵀ)`, so
/// `Im(I − R) ⊥ ker(I − R)` and `ℝ⁴ = Im(I − R) ⊕ ker(I − R)`. Therefore
///
///   CENTRE — `p` solving `(I − R) p = t` on `Im(I − R)`. The rotation is
///   invertible there, and `p` is the point the bend holds still. This is
///   CURVATURE: `R` turning about `p`, angular defect around the loop.
///
///   TWIST — `t − (I − R) p`, which lies in `ker(I − R)`: the plane the
///   rotor leaves POINTWISE FIXED, along which no choice of origin can
///   absorb the translation. The seam slips by exactly this much per
///   circuit. This is TORSION in Cartan's sense, `Θ = Dθ`.
///
/// A screw is neither of these alone — it is curvature on the turning
/// plane TENSORED with twist on the fixed plane, and both coordinates are
/// returned rather than one being collapsed into the other's absence.
///
/// THE SPLIT IS TAKEN BY THE NORMAL EQUATIONS, `AᵀA p = Aᵀ t`, which over
/// ℚ is exact elimination and nothing else: the residual `t − A p` is
/// orthogonal to `Im(A)` by construction, hence lies in `ker(A)`, which is
/// precisely the canonical twist. No tolerance, no rounding, no choice of
/// free variable leaking into the answer (law 4, rule 11).
///
/// `None` is still returned for a reading that CANNOT BE TAKEN in this
/// chart: a carriage that is not `D(ρ)·T(t)` at all, a dilation, or a turn
/// that carries the base off itself — the last being the off-base row of
/// the system, which `Axis::OffPlane` already carries and which is not a
/// slip within the base (rule 11: unread is refused, never assumed zero).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Screw {
    /// The point the bend holds still: `p` on `Im(I − R)`.
    pub centre: [Q; 3],
    /// The Burgers vector: the translation no point absorbs, in
    /// `ker(I − R)`.
    pub twist: [Q; 3],
}

pub fn screw_of(
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
) -> Option<Screw> {
    // A dilation is a residue and not a base point, and a carriage that is
    // not exactly the translator by the origin's image carries something
    // this reading does not name: refused whole rather than approximated
    // (rule 11).
    if !rho_of(carriage)?.is_one() {
        return None;
    }
    // THE SCREW IS THE ALPHABET'S, and the alphabet stands on three base
    // axes (51.2, law 6: `BASE_AXES` below), so the translation is read on
    // those three; a carriage with a component past them is no screw of
    // this rotor and refuses at the same-versor check.
    let t = crate::transducer::cga::base_at(&carriage.sandwich(&crate::transducer::cga::point_at(&[])), 3)?;
    if !same_versor(carriage, &crate::transducer::cga::translator_at(&t)) {
        return None;
    }
    let t = (t[0].clone(), t[1].clone(), t[2].clone());
    // `(I − R) p = t`, four rows in three unknowns: the base's three and
    // the off-base row that must come out zero, so a turn carrying the
    // base off itself refuses here instead of being solved on a projection
    // (checklist (m)).
    let mut rows: [[Q; 4]; 4] = std::array::from_fn(|_| std::array::from_fn(|_| Q::zero()));
    for (col, &axis) in BASE_AXES.iter().enumerate() {
        let mut e: [Q; 4] = std::array::from_fn(|_| Q::zero());
        e[axis] = Q::one();
        let image = rotor.carry(&e)?;
        for (row, &r_axis) in BASE_AXES.iter().enumerate() {
            rows[row][col] = match row == col {
                true => Q::one() - &image[r_axis],
                false => -image[r_axis].clone(),
            };
        }
        rows[3][col] = -image[2].clone();
    }
    for (row, rhs) in [t.0, t.1, t.2, Q::zero()].into_iter().enumerate() {
        rows[row][3] = rhs;
    }
    // THE CENTRE, by the normal equations on the four rows. `AᵀA` is 3×3
    // over ℚ; where it is singular the rotor fixes a direction of the base
    // pointwise, the free coordinate is set to zero, and adding a kernel
    // vector to `p` changes neither `A p` nor the residual below.
    let a: [[Q; 3]; 4] = std::array::from_fn(|r| std::array::from_fn(|c| rows[r][c].clone()));
    let rhs: [Q; 4] = std::array::from_fn(|r| rows[r][3].clone());
    let mut normal: [[Q; 4]; 3] = std::array::from_fn(|_| std::array::from_fn(|_| Q::zero()));
    for i in 0..3 {
        for j in 0..3 {
            normal[i][j] = (0..4).map(|r| &a[r][i] * &a[r][j]).fold(Q::zero(), |s, x| s + x);
        }
        normal[i][3] = (0..4).map(|r| &a[r][i] * &rhs[r]).fold(Q::zero(), |s, x| s + x);
    }
    let centre = solve_base_point(square_to_rows(normal))?;
    // THE RESIDUAL, exact: `t − A p`. Orthogonal to `Im(A)` by the normal
    // equations, hence in `ker(A)`.
    let residual: [Q; 4] = std::array::from_fn(|r| {
        (0..3).map(|c| &a[r][c] * &centre[c]).fold(Q::zero(), |s, x| s + x)
    });
    let residual: [Q; 4] = std::array::from_fn(|r| &rhs[r] - &residual[r]);
    // THE OFF-BASE ROW IS NOT A SLIP. Row three says whether the turn
    // carries the base off itself; a residual there is a reading this chart
    // cannot take, which `Axis::OffPlane` carries, and it refuses rather
    // than being counted as a translation within the base (rule 11).
    if !residual[3].is_zero() {
        return None;
    }
    Some(Screw {
        centre,
        twist: [residual[0].clone(), residual[1].clone(), residual[2].clone()],
    })
}

/// The 3×4 normal system as the four-row shape `solve_base_point` reads,
/// its fourth row all zeros — a row that is zero on both sides, which the
/// elimination passes over.
fn square_to_rows(n: [[Q; 4]; 3]) -> [[Q; 4]; 4] {
    let [a, b, c] = n;
    [a, b, c, std::array::from_fn(|_| Q::zero())]
}

/// Exact rational elimination on the four rows of `(I − R) p = t`. Any
/// solution answers, because every point of a turn's axis is a base point
/// of the same turn and the reading asks whether one exists, not which.
/// `None` is a row that is zero on the left and not on the right: the
/// system saying no point turns this way.
fn solve_base_point(mut rows: [[Q; 4]; 4]) -> Option<[Q; 3]> {
    let mut pivot_of: [Option<usize>; 3] = [None, None, None];
    let mut pivot = 0usize;
    for col in 0..3 {
        let Some(found) = (pivot..4).find(|&r| !rows[r][col].is_zero()) else { continue };
        rows.swap(pivot, found);
        let lead = rows[pivot][col].clone();
        for cell in rows[pivot].iter_mut().skip(col) {
            *cell = &*cell / &lead;
        }
        let lead_row = rows[pivot].clone();
        for (r, row) in rows.iter_mut().enumerate() {
            if r == pivot || row[col].is_zero() {
                continue;
            }
            let f = row[col].clone();
            for (cell, above) in row.iter_mut().zip(lead_row.iter()).skip(col) {
                *cell = &*cell - &(above * &f);
            }
        }
        pivot_of[col] = Some(pivot);
        pivot += 1;
    }
    if (pivot..4).any(|r| !rows[r][3].is_zero()) {
        return None;
    }
    Some(std::array::from_fn(|c| match pivot_of[c] {
        Some(r) => rows[r][3].clone(),
        None => Q::zero(),
    }))
}

// `sensor_plane_block` — the 2×2 projection of a versor's rotation onto
// the sensor plane — stood here. `base_rotor_of` reads the rotation whole
// on the base's three planes (51.2), and a projection of it is what
// checklist (m) refuses.

/// `by V by⁻¹` on versors — the same conjugation `Rotor::conjugate` does,
/// one algebra up. A conjugator that does not invert leaves `v` unmoved,
/// as the rotor's does: a non-invertible transport is not a change of
/// frame and cannot move a class.
pub fn versor_conjugate(v: &Versor, by: &Versor) -> Versor {
    match by.versor_inverse() {
        Some(inv) => by.geo(v).geo(&inv),
        None => v.clone(),
    }
}

/// The translator by an integer lattice offset within a frame — the lattice
/// peel's transport as a rational versor.
pub fn translator_versor(r: i64, c: i64) -> Versor {
    crate::transducer::cga::translator_at(&[
        num_rational::BigRational::from_integer(BigInt::from(r)),
        num_rational::BigRational::from_integer(BigInt::from(c)),
    ])
}

/// A GROUP ELEMENT: one versor, no chain. This is the thing 3.x calls a
/// group element and `same_action` describes -- an action on the sensor
/// with no boundary of its own.
pub fn of_versor(v: Versor) -> Element {
    let mut e = Element::identity();
    e.action.spatial = Some(v);
    e.address.take();
    e
}

/// THE LETTERS OF A CONNECTION as read off its description: the word,
/// the A4 carriage, and the gauge map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letters {
    pub spelling: crate::transducer::alphabet::Spelling,
    pub carriage: Versor,
    pub sigma: Vec<(u8, u8)>,
}

impl Element {
    /// The letters of this WORD — `None` for a residual germ or a word
    /// spoken before the alphabet (no tape: not a word of the language).
    ///
    /// Two tags, and they are the two the parser below reads: a connection
    /// and a translation are words. A SENTENCE is not — it is a nerve one
    /// level above a word (18.2), its tape is `[tag, version, form…]`, and
    /// byte 1 is the version where a spelling would be. It used to be
    /// admitted by this guard and then fail silently on that byte, which
    /// read as "this sentence has no letters" when the truth is that the
    /// question does not apply to it. A sentence's words are its loop
    /// holonomies: `Form::loop_words`.
    pub fn letters(&self) -> Option<Letters> {
        use crate::transducer::alphabet::Spelling;
        let b = &self.description;
        let tag = crate::transducer::address::tag_of(b)?;
        if tag != CONNECTION_TAG && tag != TRANSLATION_TAG {
            return None;
        }
        let mut pos = 1usize;
        let spelling = Spelling::from_bytes(b, &mut pos)?;
        let carriage = Versor::from_bytes(b, &mut pos)?;
        let n = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let mut sigma = Vec::with_capacity(n);
        for _ in 0..n {
            sigma.push((*b.get(pos)?, *b.get(pos + 1)?));
            pos += 2;
        }
        let len = u32::from_le_bytes(b.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        if pos + len > b.len() {
            return None;
        }
        Some(Letters { spelling, carriage, sigma })
    }

    /// Tape re-derives vertex: the geodesic of the stored word is the same
    /// transport — projective equality of the two rotors (M1, A1), never
    /// equality of two matrices (rule 16) — and V reconstructed from the
    /// tape agrees with the action. `None` when there are no letters.
    pub fn letters_rederive(&self) -> Option<bool> {
        let letters = self.letters()?;
        let v = conformal_of(&letters.spelling.word, &letters.carriage)?;
        let word = &letters.spelling.word;
        let same_word = word.rotor().projective_eq(&word.geodesic().rotor());
        Some(same_word && self.versor().is_some_and(|held| same_versor(&held, &v)))
    }

    /// The axes opened above the four strata: germinated versors.
    pub fn opened_axes(&self) -> Vec<u16> {
        self.action.germinated.iter().map(|(i, _)| *i).collect()
    }

    /// Is this word a CONNECTION — letters on the tape and an invertible
    /// versor. A residual chain is never a connection.
    pub fn is_connection(&self) -> bool {
        let Some(letters) = self.letters() else {
            return false;
        };
        if self.description_tag() != Some(CONNECTION_TAG) {
            return false;
        }
        conformal_of(&letters.spelling.word, &letters.carriage)
            .and_then(|v| v.versor_inverse())
            .is_some()
            && gauge_is_perm(&letters.sigma)
            && !(letters.spelling.word.is_identity()
                && same_versor(&letters.carriage, &identity_versor())
                && letters.sigma.is_empty()
                && self.sub.is_empty())
    }

    /// THE DESCRIPTION TAG this word opens with — 17.2's own offset, read
    /// through the one carrier for it. The tag is a coordinate of the
    /// description, so the reader is named for the description and not for the
    /// word, whose tag it is not.
    pub(crate) fn description_tag(&self) -> Option<u8> {
        crate::transducer::address::tag_of(&self.description)
    }

    /// A sentence: its own tape tag, never merely an opened-axis connection.
    /// 54.7 B6 — A SENTENCE IS KNOWN BY ITS COORDINATE, not by a magic
    /// byte at the head of a tape.
    ///
    /// This read `description()[0] == SENTENCE_TAG`, which made the kind of
    /// a word a property of a byte buffer. It is a property of the word's
    /// ADDRESS: `TAG_FORM` opens the canonical A21 coordinates of a closed
    /// form and `TAG_RESIDUAL` opens everything else, and the two were
    /// separated for exactly this reason — one tag over both meant
    /// `from_address` could not tell which layout it held.
    ///
    /// THE CYCLE IS BROKEN AT `address()`, not here. That reader used to
    /// ask `is_sentence()` and then parse the tape for the key, so the two
    /// were mutually recursive through the description. The mint sets the
    /// address (54.5) and the record carries it, refusing an empty one, so
    /// a word always knows its own name without a tape to read it from.
    pub fn is_sentence(&self) -> bool {
        self.address().tag() == Some(crate::transducer::address::TAG_FORM)
    }

    /// A translation: the gauge between two sections.
    pub fn is_translation(&self) -> bool {
        self.description_tag() == Some(TRANSLATION_TAG)
    }

    /// An optimisation spoken as a word.
    pub fn is_rewrite(&self) -> bool {
        self.description_tag() == Some(REWRITE_TAG)
    }

    /// Mint a Rewrite: the address measures the tape.
    pub fn from_rewrite_tape(description: Vec<u8>, mag: [u64; 4]) -> Element {
        let mut e = Element::identity();
        e.description = description;
        e.mag = mag;
        e.address.take();
        e
    }

    /// The two sections a translation gauges, if the tape carries them.
    pub fn translation_sections(&self) -> Option<(crate::transducer::address::Address, crate::transducer::address::Address)> {
        if !self.is_translation() {
            return None;
        }
        let b = self.description();
        // Walk back two length-delimited addresses at the tail.
        // The tape is prefix + addr_a + addr_b; decode from the start of the pair
        // by scanning: find the first put_addr that consumes the suffix.
        let mut q = 0usize;
        while q + 2 <= b.len() {
            let mut t = q;
            if let (Some(a), Some(c)) = (
                crate::transducer::address::take_addr(b, &mut t),
                crate::transducer::address::take_addr(b, &mut t),
            ) {
                if t == b.len() {
                    return Some((a, c));
                }
            }
            q += 1;
        }
        None
    }

}
