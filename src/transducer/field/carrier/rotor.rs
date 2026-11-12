// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! LAW-COORDINATE fixture 11b (proposal 78.173 §5, `FORMS-TURNING` v1),
//! ROADMAP track 26 S7 stage 3 — A ROTOR, A MOTION OF THE FIELD HELD BY ITS
//! RATE, and the composition of two turnings on planes that share a unit.
//!
//! A rotor holds an even generator `B`, its rate, with rational components
//! on its planes. Its motion `R = (1 + B)(1 − B)⁻¹` is read through
//! `forms::turning` whenever it is asked and never stored; two rotors
//! compose as `R₁R₂`, whole, and the rate is read back,
//! `B = (R − 1)(R + 1)⁻¹`. The planes are read from the rate's terms. A
//! rational rate names a Pythagorean angle, which is in general no rational
//! number of degrees (78.173 §8.3), so a rotor carries no degrees: its
//! angle on a plane is a reading of the rate, outside this carrier.
//!
//! A TURNING'S MOTION IS RATIONAL where its angle is a whole number of
//! quarter turns: `cos θ + B·sin θ` with both coefficients in ℚ and `θ` a
//! rational number of degrees holds only there (Niven). Two such turnings
//! on planes that share a unit compose by the law ([`shared`]): their
//! motions' product, read back as one turning on one plane, or as none.
//! Any other angle is refused `NotExact` with `Residue::IrrationalMotion`,
//! the plane and the angle. The degree convention is the carrier's own
//! (78.171: the motion `cos θ + B·sin θ`, one-sided); the factor of two
//! against `forms::turning::reading` is open in 78.173 §8.3.

use super::atlas::Chart;
use super::refusal::{FieldRefusal, Residue};
use super::turning::{Turning, TurningPlane};
use crate::core::Q;
use crate::transducer::cga::{base_axis, Clifford, Mv};
use forms::coordinate::{Axis, Key, Square};
use forms::turning::{compose, generator, motion, same};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// A ROTOR: a motion of the field held by its residual rate `B`, an even
/// generator of `Cl(p, q)` with exact rational components, and its signed
/// revolution count `K` per canonical invariant plane. One constructor reads
/// the rate, one the motion; both are checked by the law, so no rotor holds a
/// rate that names no motion.
#[derive(Clone, Debug)]
pub struct Rotor {
    rate: Clifford,
    windings: Vec<(Mv, BigInt)>,
}

impl PartialEq for Rotor {
    fn eq(&self, other: &Rotor) -> bool {
        same(&self.rate, &other.rate) && self.windings == other.windings
    }
}

impl Eq for Rotor {}

/// Canonical bytes of a plane `Mv`, used only to keep `Rotor::windings` in a
/// canonical order independent of composition order.
fn plane_order_key(plane: &Mv) -> Vec<u8> {
    let mut out = Vec::new();
    plane.put_canonical(&mut out);
    out
}

/// Add `delta` to `plane`'s signed winding count in `windings`, keeping only
/// non-zero counts in canonical plane order.
fn step_winding(windings: &mut Vec<(Mv, BigInt)>, plane: Mv, delta: BigInt) {
    if delta.is_zero() {
        return;
    }
    let key = plane_order_key(&plane);
    match windings.binary_search_by_key(&key, |(p, _)| plane_order_key(p)) {
        Ok(idx) => {
            windings[idx].1 += delta;
            if windings[idx].1.is_zero() {
                windings.remove(idx);
            }
        }
        Err(idx) => windings.insert(idx, (plane, delta)),
    }
}

/// If `b` is a non-zero simple elliptic grade-2 generator (`B² < 0` scalar),
/// returns its canonical plane `P` and its signed ratio `r` on `P` (`b = r P`).
fn simple_elliptic_rate(b: &Clifford) -> Option<(Mv, Q)> {
    if b.is_zero() || !b.terms().all(|(m, _)| m.count_ones() == 2) {
        return None;
    }
    let sq = b.product(b).ok()?;
    if !sq.terms().all(|(m, _)| m == 0) || !sq.multivector().scalar_part().is_negative() {
        return None;
    }
    let mv = b.multivector();
    let (_, first) = mv.terms().next()?;
    let canon = mv.canonical()?;
    Some((canon, first.clone()))
}

/// If `b` is a non-zero simple grade-2 generator (`B²` a non-zero scalar),
/// returns its canonical plane `P`.
fn simple_plane(b: &Clifford) -> Option<Mv> {
    if b.is_zero() || !b.terms().all(|(m, _)| m.count_ones() == 2) {
        return None;
    }
    let sq = b.product(b).ok()?;
    if !sq.terms().all(|(m, _)| m == 0) || sq.multivector().scalar_part().is_zero() {
        return None;
    }
    b.multivector().canonical()
}

impl Rotor {
    /// THE ROTOR OF THE RATE `rate`. Refused as `forms::turning::motion`
    /// refuses: `NotGenerator` where a part is no generator's, `NoMotion`
    /// where `1 − B` has no inverse.
    pub fn of(rate: Clifford) -> Result<Rotor, FieldRefusal> {
        motion(&rate).map_err(FieldRefusal::Turning)?;
        Ok(Rotor { rate, windings: Vec::new() })
    }

    /// THE ROTOR OF THE MOTION `r`, its rate read back by the Cayley form.
    /// Refused as `forms::turning::generator` refuses: `NotEven` or
    /// `NotUnit` where `r` is no motion, `NoGenerator` where `R + 1` has no
    /// inverse (`R = −1` among them).
    pub fn of_motion(r: &Clifford) -> Result<Rotor, FieldRefusal> {
        Ok(Rotor { rate: generator(r).map_err(FieldRefusal::Turning)?, windings: Vec::new() })
    }

    /// THE IDENTITY ROTOR (`B = 0`, `K = 0`) under `negatives`.
    pub fn identity(negatives: usize) -> Result<Rotor, FieldRefusal> {
        let zero = Clifford::blade(negatives, 0, Q::zero()).map_err(FieldRefusal::Mismatched)?;
        Ok(Rotor { rate: zero, windings: Vec::new() })
    }

    /// THE ROTOR OF A VERSOR `v` (such as a `cech::orbit` flow increment
    /// `c + sP` or an even spinor ray): off the pole (`c ≠ 0` on a simple
    /// plane), its rate is `(s/c) P`; at the 180° pole (`c = 0` on a simple
    /// elliptic plane), its residual rate is `B = 0` and its winding `K` on
    /// `P` is `±1` in the generator's orientation.
    pub fn of_versor(v: &Mv) -> Result<Rotor, FieldRefusal> {
        let s = v.scalar_part();
        let g2 = v.grade(2);
        let g4 = v.grade(4);
        if g2.is_zero() && g4.is_zero() {
            if s.is_zero() {
                return Rotor::of_motion(&Clifford::of_conformal(v));
            }
            return Rotor::identity(v.negatives());
        }
        if g4.is_zero() {
            if !s.is_zero() {
                let rate_mv = g2.scale(&(Q::one() / s));
                return Rotor::of(Clifford::of_conformal(&rate_mv));
            }
            let pole = Clifford::of_conformal(&g2);
            if let Some((plane, ratio)) = simple_elliptic_rate(&pole) {
                let zero = Clifford::blade(v.negatives(), 0, Q::zero()).map_err(FieldRefusal::Mismatched)?;
                let sign = if ratio.is_positive() { BigInt::from(1) } else { BigInt::from(-1) };
                return Ok(Rotor { rate: zero, windings: vec![(plane, sign)] });
            }
            return Rotor::of_motion(&pole);
        }
        if v.negatives() == crate::transducer::cga::EM {
            if let Some(b) = crate::transducer::cga::cayley_inverse(v) {
                return Rotor::of(Clifford::of_conformal(&b.scale(&-Q::one())));
            }
        }
        Rotor::of_motion(&Clifford::of_conformal(v))
    }

    /// The residual rate `B`.
    pub fn rate(&self) -> &Clifford {
        &self.rate
    }

    /// The signed revolution counts `(P, K)` per canonical invariant plane `P`
    /// where `K ≠ 0`.
    pub fn windings(&self) -> &[(Mv, BigInt)] {
        &self.windings
    }

    /// The signed revolution count `K` on `plane` (matched against its
    /// canonical plane `P`).
    pub fn winding_on(&self, plane: &Mv) -> BigInt {
        let Some(canon) = plane.grade(2).canonical() else { return BigInt::zero() };
        let key = plane_order_key(&canon);
        match self.windings.binary_search_by_key(&key, |(p, _)| plane_order_key(p)) {
            Ok(idx) => self.windings[idx].1.clone(),
            Err(_) => BigInt::zero(),
        }
    }

    /// Whether this rotor is the identity (`B = 0` and `K = 0` on every plane).
    pub fn is_identity(&self) -> bool {
        self.rate.is_zero() && self.windings.is_empty()
    }

    /// Whether this rotor's residual rate vanishes (`R = 1`, resonant when
    /// `K ≠ 0`).
    pub fn is_resonant(&self) -> bool {
        self.rate.is_zero()
    }

    /// THE MOTION `R = (1 + B)(1 − B)⁻¹`, read from the rate.
    pub fn motion(&self) -> Result<Clifford, FieldRefusal> {
        motion(&self.rate).map_err(FieldRefusal::Turning)
    }

    /// THIS ROTOR, THEN `other`: the motions composed as `R₁R₂`, whole and
    /// never plane by plane, with the residual rate read back and the signed
    /// revolution count `K` stepped on the invariant plane whenever the Cayley
    /// pole `1 + ⟨B₁ B₂⟩₀ ≤ 0` is reached or crossed.
    pub fn then(&self, other: &Rotor) -> Result<Rotor, FieldRefusal> {
        self.rate.product(&other.rate).map_err(FieldRefusal::Mismatched)?;
        let mut windings = self.windings.clone();
        for (plane, count) in &other.windings {
            step_winding(&mut windings, plane.clone(), count.clone());
        }
        if let (Some((p1, r1)), Some((p2, r2))) = (simple_elliptic_rate(&self.rate), simple_elliptic_rate(&other.rate)) {
            if plane_order_key(&p1) == plane_order_key(&p2) {
                let prod = self.rate.product(&other.rate).map_err(FieldRefusal::Mismatched)?;
                let denom = Q::one() + prod.multivector().scalar_part();
                if denom.is_zero() {
                    let delta = if r1.is_positive() { BigInt::from(1) } else { BigInt::from(-1) };
                    step_winding(&mut windings, p1, delta);
                    let zero = Clifford::blade(self.rate.negatives(), 0, Q::zero()).map_err(FieldRefusal::Mismatched)?;
                    return Ok(Rotor { rate: zero, windings });
                }
                if denom.is_negative() {
                    let sum = &r1 + &r2;
                    let delta = if sum.is_positive() { BigInt::from(1) } else { BigInt::from(-1) };
                    step_winding(&mut windings, p1, delta);
                }
            }
        }
        let composite = compose(&self.motion()?, &other.motion()?).map_err(FieldRefusal::Turning)?;
        let rate = generator(&composite).map_err(FieldRefusal::Turning)?;
        Ok(Rotor { rate, windings })
    }

    /// THE PLANES OF THE RATE: each part of grade 2 as the key of its two
    /// units, ascending (`e_+1+3`), with its exact component, in the
    /// rate's canonical order. A boost plane's key names its two units of
    /// opposite squares. Refused `NotPlanes` with the masks of the parts on
    /// no plane (grade 6 and above, in six or more frame units), and with
    /// the masks whose units the frame does not name.
    pub fn planes(&self) -> Result<Vec<(Key, Q)>, FieldRefusal> {
        let negatives = self.rate.negatives();
        let mut planes = Vec::new();
        let mut unplaned = Vec::new();
        for (mask, c) in self.rate.terms() {
            match key_of(negatives, mask) {
                Some(key) => planes.push((key, c.clone())),
                None => unplaned.push(mask),
            }
        }
        match unplaned.is_empty() {
            true => Ok(planes),
            false => Err(FieldRefusal::NotPlanes { masks: unplaned }),
        }
    }
}

/// A TRANSITION ON A LOCAL ORBIT: the ordered pair of charts `(from, to)` and
/// the rate-held [`Rotor`] carrying a sample from `from` to `to`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RotorTransition {
    pub from: Chart,
    pub to: Chart,
    pub motion: Rotor,
}

/// THE PRECESSION OF A LOCAL ORBIT PER REVOLUTION: `Resonant` where the
/// revolution's residual rate vanishes (`R = 1`, its signed winding `K` kept
/// on its planes), `Advances` where a non-zero residual rate remains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RotorPrecession {
    Advances(Rotor),
    Resonant(Rotor),
}

/// A LOCAL ORBIT ATTACHED TO A WELL: the admission-ordered sequence of
/// `(Chart, Rotor)` steps threading a `cech::orbit` orbit. A single-line orbit
/// stands attached to its well and refuses revolution readings with
/// `FieldRefusal::NotRevolution { at: 1 }` until a second line winds onto it;
/// an orbit of two or more lines forms a closed revolution whose transitions
/// compose in admission order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalOrbit {
    steps: Vec<(Chart, Rotor)>,
    revolution: Option<Vec<RotorTransition>>,
}

impl LocalOrbit {
    /// A LOCAL ORBIT FROM ITS ADMISSION-ORDERED STEPS `(chart, motion)`.
    /// Refused `NotRevolution { at: 0 }` when `steps` is empty, and `Unbound`
    /// when every step is the identity (`B = 0, K = 0`). A single non-identity
    /// step is kept awaiting its second line.
    pub fn of_steps(steps: Vec<(Chart, Rotor)>) -> Result<LocalOrbit, FieldRefusal> {
        if steps.is_empty() {
            return Err(FieldRefusal::NotRevolution { at: 0 });
        }
        if steps.iter().all(|(_, m)| m.is_identity()) {
            return Err(FieldRefusal::Unbound { charts: steps.into_iter().map(|(c, _)| c).collect() });
        }
        if steps.len() == 1 {
            return Ok(LocalOrbit { steps, revolution: None });
        }
        let n = steps.len();
        let revolution = (0..n)
            .map(|k| RotorTransition {
                from: steps[k].0.clone(),
                to: steps[(k + 1) % n].0.clone(),
                motion: steps[(k + 1) % n].1.clone(),
            })
            .collect();
        Ok(LocalOrbit { steps, revolution: Some(revolution) })
    }

    /// A LOCAL ORBIT FROM EXPLICIT [`RotorTransition`]s around one revolution.
    /// Refused `NotRevolution` where empty (`at: 0`) or where consecutive
    /// transitions do not meet (`at: step + 1`), and `Unbound` where every
    /// transition's rotor is the identity.
    pub fn of(revolution: Vec<RotorTransition>) -> Result<LocalOrbit, FieldRefusal> {
        let steps_len = revolution.len();
        if steps_len == 0 {
            return Err(FieldRefusal::NotRevolution { at: 0 });
        }
        if let Some(at) = (0..steps_len).find(|&i| match (revolution.get(i), revolution.get((i + 1) % steps_len)) {
            (Some(a), Some(b)) => a.to != b.from,
            _ => true,
        }) {
            return Err(FieldRefusal::NotRevolution { at: at + 1 });
        }
        if revolution.iter().all(|t| t.motion.is_identity()) {
            return Err(FieldRefusal::Unbound { charts: revolution.into_iter().map(|t| t.from).collect() });
        }
        let steps = (0..steps_len)
            .map(|k| {
                let prev = (k + steps_len - 1) % steps_len;
                (revolution[k].from.clone(), revolution[prev].motion.clone())
            })
            .collect();
        Ok(LocalOrbit { steps, revolution: Some(revolution) })
    }

    /// The admission-ordered `(chart, motion)` steps of the orbit.
    pub fn steps(&self) -> &[(Chart, Rotor)] {
        &self.steps
    }

    /// The closed revolution's transitions. Refused `NotRevolution { at: 1 }`
    /// when the orbit holds a single line awaiting its second line.
    pub fn revolution(&self) -> Result<&[RotorTransition], FieldRefusal> {
        self.revolution.as_deref().ok_or(FieldRefusal::NotRevolution { at: self.steps.len() })
    }

    /// THE HOLONOMY OF ONE REVOLUTION: the revolution's [`Rotor`]s composed in
    /// order. Refused `NotRevolution { at: 1 }` when the orbit holds a single
    /// line awaiting its second line.
    pub fn holonomy(&self) -> Result<Rotor, FieldRefusal> {
        let rev = self.revolution()?;
        let mut iter = rev.iter();
        let Some(first) = iter.next() else {
            return Err(FieldRefusal::NotRevolution { at: 0 });
        };
        iter.try_fold(first.motion.clone(), |acc, t| acc.then(&t.motion))
    }

    /// THE PRECESSION PER REVOLUTION: `Resonant` where the revolution's
    /// holonomy has zero residual rate (`R = 1`, with its signed `K` kept),
    /// `Advances` where a non-zero residual rate remains. Refused
    /// `NotRevolution { at: 1 }` on a single-line orbit.
    pub fn precession(&self) -> Result<RotorPrecession, FieldRefusal> {
        let h = self.holonomy()?;
        match h.is_resonant() {
            true => Ok(RotorPrecession::Resonant(h)),
            false => Ok(RotorPrecession::Advances(h)),
        }
    }

    /// The orbit's primary simple plane and a representative rate on it when
    /// every non-identity step turns on one common plane; `None` when steps
    /// span multiple planes.
    fn primary_rate(&self) -> Option<(Mv, Clifford)> {
        let mut iter = self.steps.iter().filter(|(_, m)| !m.rate().is_zero());
        let (_, first_rotor) = iter.next()?;
        let first_plane = simple_plane(first_rotor.rate())?;
        let first_key = plane_order_key(&first_plane);
        for (_, m) in iter {
            let p = simple_plane(m.rate())?;
            if plane_order_key(&p) != first_key {
                return None;
            }
        }
        if let Ok(h) = self.holonomy() {
            if !h.rate().is_zero() && simple_plane(h.rate()).is_some_and(|p| plane_order_key(&p) == first_key) {
                return Some((first_plane, h.rate().clone()));
            }
        }
        Some((first_plane, first_rotor.rate().clone()))
    }

    /// AXIOMATIC GRAVITY SHEAR (MU-SENSING §3.7): the rate drag experienced by
    /// `other` from `self` when the two orbits stand on orthogonal simple
    /// planes (`B₁ · B₂ = 0` and `B₁ ∧ B₂ ≠ 0`, commuting). Returns
    /// `Some(drag)` on the orthogonal class and `None` when the planes share a
    /// unit or coincide.
    pub fn rate_shear(&self, other: &LocalOrbit) -> Result<Option<Mv>, FieldRefusal> {
        let (Some((_, b_local)), Some((_, b_neighbour))) = (self.primary_rate(), other.primary_rate()) else {
            return Ok(None);
        };
        let prod = b_local.product(&b_neighbour).map_err(FieldRefusal::Mismatched)?;
        let mv = prod.multivector();
        let orthogonal = mv.grade(0).is_zero() && mv.grade(2).is_zero() && !mv.grade(4).is_zero();
        match orthogonal {
            true => Ok(Some(b_neighbour.multivector().grade(2))),
            false => Ok(None),
        }
    }
}

/// THE FRAME UNIT OF ONE GENERATOR BIT under `negatives`: plane `k`'s
/// `e₊ₖ` is base generator `2k − 1` and `e₋ₖ` base generator `2k`
/// (`FramePlanes`); its square read from the value's negative mask must
/// agree. `None` for a bit no frame unit holds.
fn unit_of(negatives: usize, bit: usize) -> Option<Axis> {
    let g = (1..).map_while(|g| base_axis(g).map(|m| (g, m))).find(|(_, m)| *m == bit)?.0;
    let (square, k) = match g % 2 {
        1 => (Square::Positive, g.div_ceil(2)),
        _ => (Square::Negative, g / 2),
    };
    let held = match negatives & bit {
        0 => Square::Positive,
        _ => Square::Negative,
    };
    (held == square).then(|| Axis::unit(square, u32::try_from(k).ok()?).ok()).flatten()
}

/// The key of a grade-2 mask, its units ascending; `None` for any other
/// grade, or a bit no frame unit holds.
fn key_of(negatives: usize, mask: usize) -> Option<Key> {
    if mask.count_ones() != 2 {
        return None;
    }
    let low = mask & mask.wrapping_neg();
    Key::generator(unit_of(negatives, low)?, unit_of(negatives, mask ^ low)?).ok()
}

/// THE TURNING'S MOTION AS A VALUE, `cos θ + B·sin θ` on its plane, where
/// it is rational: `θ` a whole number of quarter turns. Refused `NotExact`
/// with `IrrationalMotion`, the plane and the angle, at any other angle.
fn rational_motion(t: &Turning) -> Result<Clifford, FieldRefusal> {
    let quarters = t.degrees() / Q::from_integer(BigInt::from(90));
    let irrational = || FieldRefusal::NotExact { residue: Residue::IrrationalMotion { plane: t.plane().key().clone(), degrees: t.degrees().clone() } };
    if !quarters.is_integer() {
        return Err(irrational());
    }
    let b = t.plane().bivector();
    let (cos, sin) = match quarters.to_integer().mod_floor(&BigInt::from(4)).to_u8() {
        Some(0) => (1, 0),
        Some(1) => (0, 1),
        Some(2) => (-1, 0),
        Some(3) => (0, -1),
        _ => return Err(irrational()),
    };
    let scalar = Clifford::blade(b.negatives(), 0, Q::from_integer(BigInt::from(cos))).map_err(FieldRefusal::Mismatched)?;
    let turned = b.product(&Clifford::blade(b.negatives(), 0, Q::from_integer(BigInt::from(sin))).map_err(FieldRefusal::Mismatched)?).map_err(FieldRefusal::Mismatched)?;
    scalar.sum(&turned).map_err(FieldRefusal::Mismatched)
}

/// TWO TURNINGS ON PLANES THAT SHARE A UNIT, `held` then `arriving`,
/// composed by the law: their rational motions' product `R₁R₂`, the rate
/// read back as a [`Rotor`] and its one plane read, as one turning on that
/// plane by the composite's angle (the carrier's `cos θ + B·sin θ`, so a
/// rate `±1` is `±90°`), or as none where the composite is the identity.
/// The plane is one of `candidates` (the two planes and the plane between
/// them), in the orientation the key names; reversed, the angle reads with
/// the opposite sign.
///
/// Refused `NotExact`: with `IrrationalMotion` where either angle is no
/// whole number of quarter turns; with `Uncommuted` naming both planes
/// where either carries a winding, which a composite on another plane does
/// not keep (78.173 §8.3: how `K` is held beside `R` is open).
pub(super) fn shared(held: &Turning, arriving: &Turning, candidates: &[TurningPlane]) -> Result<Option<Turning>, FieldRefusal> {
    let first = rational_motion(held)?;
    let then = rational_motion(arriving)?;
    let uncommuted = || FieldRefusal::NotExact { residue: Residue::Uncommuted { held: held.plane().key().clone(), arriving: arriving.plane().key().clone() } };
    if !held.winding().is_zero() || !arriving.winding().is_zero() {
        return Err(uncommuted());
    }
    let composite = compose(&first, &then).map_err(FieldRefusal::Turning)?;
    let planes = Rotor::of_motion(&composite)?.planes()?;
    let (key, rate) = match planes.as_slice() {
        [] => return Ok(None),
        [one] => one,
        _ => return Err(uncommuted()),
    };
    let quarter = Q::from_integer(BigInt::from(90));
    let one = Q::one();
    let angle = match (rate == &one, rate == &-one.clone()) {
        (true, _) => quarter,
        (_, true) => -quarter,
        _ => return Err(uncommuted()),
    };
    for plane in candidates {
        let [x, y] = plane.key().axes() else { return Err(uncommuted()) };
        let reversed = Key::generator(y.clone(), x.clone()).map_err(FieldRefusal::NotTurningPlane)?;
        match (plane.key() == key, &reversed == key) {
            (true, _) => return Turning::of(plane.clone(), angle.clone()).map(Some),
            (_, true) => return Turning::of(plane.clone(), -angle.clone()).map(Some),
            _ => {}
        }
    }
    Err(uncommuted())
}
