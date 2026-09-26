// 16.4 — CARRY: the versor that carries one shape onto another, found in
// closed form and refused when none exists.

use super::*;

pub(crate) fn q(v: i64) -> BigRational {
    BigRational::from_integer(BigInt::from(v))
}
pub(crate) fn qn(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// The exact square root of a rational, if it is a rational square.
pub(crate) fn rational_sqrt(x: &BigRational) -> Option<BigRational> {
    if x.numer().sign() == num_bigint::Sign::Minus {
        return None;
    }
    let (n, d) = (x.numer().sqrt(), x.denom().sqrt());
    if &(&n * &n) == x.numer() && &(&d * &d) == x.denom() {
        Some(BigRational::new(n, d))
    } else {
        None
    }
}

/// Apply a versor to a point of the base's vertex coordinates and read the
/// base back at the point's own `n`; `None` at infinity.
pub fn carry(v: &Mv, p: &[i64]) -> Option<Vec<BigRational>> {
    let x: Vec<BigRational> = p.iter().map(|&a| q(a)).collect();
    cga::base_at(&v.sandwich(&cga::point_at(&x)), p.len())
}

/// A body's centroid in the sensor's VERTEX coordinates: the compartments'
/// centres' centroid plus one half on each grid axis (a compartment's square
/// spans the grid axes), and the offset alone on every other axis — a
/// compartment stands AT its frame (16.4).
pub(super) fn vertex_centroid(b: &Blade) -> Option<Vec<BigRational>> {
    Some(
        b.centroid()?
            .into_iter()
            .enumerate()
            .map(|(i, x)| match crate::transducer::base::is_grid_axis(i) {
                true => x + qn(1, 2),
                false => x,
            })
            .collect(),
    )
}

/// `b − a`, coordinate by coordinate; `None` where the two points stand on
/// bases of different `n`.
fn difference(b: &[BigRational], a: &[BigRational]) -> Option<Vec<BigRational>> {
    match a.len() == b.len() {
        true => Some(b.iter().zip(a).map(|(x, y)| x - y).collect()),
        false => None,
    }
}

/// THE TRANSPORT BETWEEN TWO BODIES, in closed form from their moments:
/// the canonical representative (50.2 step 1 as ruled 2026-09-09) of the
/// coset `versor_coset` measures. The refusal is the pose's, typed: no
/// rational versor carries `a` onto `b` by its named cause, or
/// [`NoPose::Unbounded`] where the stroke was spent first (65.5).
pub fn versor_between(a: &Shape, b: &Shape, budget: &mut crate::physics::Budget) -> Result<Mv, NoPose> {
    let coset = versor_coset(a, b, budget)?;
    coset_representative(&coset).map(|t| t.versor.clone()).ok_or(NoPose::Ungauged)
}

/// THE MOST CANDIDATES ONE TRANSITION VERIFIES, and so the most reads it
/// draws from the stroke (65.5, ruled 2026-09-26): two parities, each at most
/// the fourth harmonic's four 90° turns. The tube's pose tries fewer (a
/// proper and a mirrored solve, or two signs on two lines). What an
/// extraction's stroke is sized by per body pair (`extraction_spend`).
pub const CANDIDATES_PER_PAIR: usize = 8;

/// ONE MEMBER OF A TRANSITION BETWEEN TWO SHAPES (ruled 2026-09-26): the
/// rotation it was built from, the versor carrying `a` onto `b`, and the
/// RESIDUE — each order above the fit's where `b` is not the carried `a`,
/// exact. Empty where the fit is whole. The residue is carried with the
/// versor and never absorbed: it is what the fit costs, RM1's remainder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transition {
    pub rotation: Mv,
    pub versor: Mv,
    pub residue: Vec<Residue>,
    /// The base planes whose image is no base plane: their orders three and
    /// four are unread by the tower (`Covariance::unread`), named.
    pub unread: Vec<(usize, usize)>,
}

/// THE COSET OF TRANSITIONS carrying `a` onto `b` on the sensor's plane
/// (BQ4, named): the transition's members ([`transitions_between`]), each
/// with its rotation and its residue — never empty: a pair nothing carries
/// is refused by the pose's named cause, and a stroke spent before every
/// candidate was read is the typed cut [`NoPose::Unbounded`].
pub fn versor_coset(a: &Shape, b: &Shape, budget: &mut crate::physics::Budget) -> Result<Vec<Transition>, NoPose> {
    transitions_between(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE, budget)
}

/// THE DOUBLE-ANGLE POINT of a harmonic of degree `m` between two bodies:
/// `conj(h_a)·h_b / (ρ^m ‖h_a‖²)`, the point `(cos mφ, sin mφ)` — rational,
/// no root taken, because the order's certificate is `‖h_b‖² = ρ^{2m}‖h_a‖²`
/// (A.12). `scale` is `ρ^m`.
fn phase_point(ha: &[BigRational; 2], hb: &[BigRational; 2], scale: &BigRational) -> Option<(BigRational, BigRational)> {
    let norm = (&ha[0] * &ha[0] + &ha[1] * &ha[1]) * scale;
    if norm.is_zero() {
        return None;
    }
    Some((
        (&ha[0] * &hb[0] + &ha[1] * &hb[1]) / &norm,
        (&ha[0] * &hb[1] - &ha[1] * &hb[0]) / &norm,
    ))
}

/// THE EXACT HALF-ANGLE STEP (DG1, 78.45): from the rational point of `2θ`
/// to the rational point of `θ`, the representative with `cos θ ≥ 0` — the
/// other is its antipode, which the coset carries. `2(1 + C) = r²` is the
/// condition, `cos θ = (1 + C)/r`, `sin θ = S/r`; at `C = −1` (`2θ` is 180°)
/// the point is `(0, 1)`, 90°. `None` where `2(1 + C)` is no rational square:
/// `θ` is no rational point, never rounded.
fn half_point(p: &(BigRational, BigRational)) -> Option<(BigRational, BigRational)> {
    let one_c = q(1) + &p.0;
    if one_c.is_zero() {
        return Some((q(0), q(1)));
    }
    let r = rational_sqrt(&(q(2) * &one_c))?;
    Some((one_c / &r, &p.1 / &r))
}

/// THE ROTOR RAY TURNING BY `θ` from its rational point, `cos θ ≥ 0`: `1 −
/// t·e_ij` with `t = sin θ/(1 + cos θ)`, the half-angle tangent, exact. The
/// sign is the algebra's: `(1 + t e_ij) e_i (1 − t e_ij) = (1 − t²) e_i −
/// 2t e_j`, so `1 + t·e_ij` turns axis `i` AWAY from axis `j`, by `−θ` in
/// the harmonics' convention `u + i·v`; the turn by `+θ` is its reverse.
fn rotor_at(p: &(BigRational, BigRational), e_ij: &Mv) -> Mv {
    let t = &p.1 / (q(1) + &p.0);
    Mv::scalar(q(1)).sub(&e_ij.scale(&t))
}

/// THE TRANSITION BETWEEN TWO SHAPES ON ONE PLANE (55.3, 78.38(C), 68, A.12,
/// DG1; one edge of a cover — congruence is the cover's reading over many
/// shapes, 24.4): every versor carrying `a` onto `b`, each with its rotation.
///
/// ```text
///   fit      the certificate's (`certify`): the tower, and the order it fits
///            the pair through; both parting at order two is Incongruent(Order)
///   scale    ρ² = r²_b / r²_a, the two extents on that tower; no rational
///            ρ, no member
///   turn     the second harmonic's point of 2φ, one exact half-angle step;
///            isotropic there, the fourth's point of 4φ and two steps
///   coset    {R, R·e_ij} from the second; {R·Q_k}, the four 90° turns,
///            from the fourth; none chosen (L7c)
///   odd      the same against a's harmonics conjugated — `a` mirrored across
///            the plane's first axis — and the member R·e_j
///   place    T(c_b − V_lin(c_a)) · V_lin,  V_lin = D(ρ)·R  (·e_j)
///   verify   the tower carried by V through the fit's order, every order
///            above it returned as the member's residue (`carries_linear`)
/// ```
///
/// No member lands a compartment: the verification is the covariance of the
/// tower, and a filled blow-up is carried with the sensor block's own
/// `S₄(k)/k²` as its order-four residue. An empty list is a pair no rational similarity carries on this
/// plane — its reason the pose's to name (`unturned`): where the half-angle
/// point or `ρ` is not rational, Oblique, the RotorPrime demand. Refused
/// before any candidate: a bodiless pair, a pair neither tower certifies, a
/// pair where one body orients and the other does not.
pub fn transitions_between(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    budget: &mut crate::physics::Budget,
) -> Result<Vec<Transition>, NoPose> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    match certify(a, b, plane) {
        Ok(fit) => transitions_on(a, b, plane, fit, budget),
        Err((region, sites)) => Err(NoPose::Incongruent(Incongruence::Order { region, sites })),
    }
}

/// THE TRANSITION ON THE FIT ALREADY CERTIFIED: the pose reads the
/// certificate once and hands its fit here, so no pair is certified twice.
pub(super) fn transitions_on(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    fit: super::tower::Fit,
    budget: &mut crate::physics::Budget,
) -> Result<Vec<Transition>, NoPose> {
    use super::tower::{extent, harmonic, Harmonic};
    let tower = fit.dilation.tower();
    let (Some(sa), Some(sb)) = (ContinuousSupport::of(a, tower), ContinuousSupport::of(b, tower)) else {
        return Err(NoPose::Bodiless);
    };
    let (Some(ra), Some(rb)) = (extent(a, plane, tower), extent(b, plane, tower)) else {
        return Err(NoPose::Bodiless);
    };
    if ra.is_zero() || rb.is_zero() {
        return Err(NoPose::Unoriented);
    }
    let mut out: Vec<Transition> = Vec::new();
    // Why nothing carried, where nothing did: the first candidate's
    // covariance refusal in the candidates' constructed order, which the
    // pair's content fixes (the proper parity first).
    let mut uncarried: Option<super::continuous::NotCarried> = None;
    let Some(rho) = rational_sqrt(&(&rb / &ra)) else {
        return Err(NoPose::Irrational);
    };
    let rho2 = &rho * &rho;
    let dilator = cga::dilator(&rho);
    let e_ij = Mv::blade(cga::base_axis(plane.0 + 1) | cga::base_axis(plane.1 + 1), q(1));
    let mirror = Mv::blade(cga::base_axis(plane.1 + 1), q(1));
    // The four 90° turns of the plane as rays: 0°, 90°, 180°, 270°.
    let one = Mv::scalar(q(1));
    let turns_90 = [one.clone(), one.add(&e_ij), e_ij.clone(), e_ij.sub(&one)];
    // Each body's orienting harmonics, read once for both parities. ONLY AN
    // ORDER THE FIT HOLDS THROUGH ORIENTS: above it the harmonic is part
    // residue, and a phase read off it is not the pair's.
    let orienting = |x: &Blade| {
        let quartic = match fit.through {
            Harmonic::Four => harmonic(x, plane, tower, Harmonic::Four),
            Harmonic::Two | Harmonic::Three => None,
        };
        (harmonic(x, plane, tower, Harmonic::Two), quartic)
    };
    let ((a2, a4), (b2, b4)) = (orienting(a), orienting(b));
    for odd in [false, true] {
        // The odd parity reads `a` mirrored across the plane's first axis.
        let parity = |h: &Option<[BigRational; 2]>| {
            h.as_ref().map(|h| match odd {
                true => [h[0].clone(), -&h[1]],
                false => h.clone(),
            })
        };
        let rotors: Vec<Mv> = match ((parity(&a2), b2.clone()), (parity(&a4), b4.clone())) {
            ((Some(ua), Some(ub)), _) => match phase_point(&ua, &ub, &rho2).and_then(|p| half_point(&p)) {
                Some(p) => {
                    let r = rotor_at(&p, &e_ij);
                    vec![r.clone(), r.geo(&e_ij)]
                }
                // 2φ has no rational half: no rational member at this parity.
                None => Vec::new(),
            },
            ((None, None), (Some(qa), Some(qb))) => {
                match phase_point(&qa, &qb, &(&rho2 * &rho2)).and_then(|p| half_point(&p)).and_then(|p| half_point(&p)) {
                    Some(p) => {
                        let r = rotor_at(&p, &e_ij);
                        turns_90.iter().map(|k| r.geo(k)).collect()
                    }
                    None => Vec::new(),
                }
            }
            // Isotropic at both orders on both: nothing orients the pair, and
            // the identity is the one member the tower can verify.
            ((None, None), (None, None)) => vec![one.clone()],
            _ => return Err(NoPose::Isotropy),
        };
        for rotor in rotors {
            // AT1 (r4) — the carry's candidate loop, counted at its site.
            crate::transducer::sentence::form::atlas::note_carry_candidate();
            // EACH CANDIDATE VERIFIED DRAWS ONE READ FROM THE STROKE (65.5,
            // rule 10), so `Spend` reads the transition and a short stroke is
            // a typed cut, never a silent overrun.
            if !budget.spend() {
                return Err(NoPose::Unbounded);
            }
            let rotation = match odd {
                true => rotor.geo(&mirror),
                false => rotor,
            };
            // The candidate's rotation and scale are in hand: its covariance is
            // read on them, and only a member that carries is placed — the
            // placement sends `c_a` to `c_b` by construction.
            let covariance = match sa.carries_linear(&sb, &rotation, &rho, fit.through) {
                Ok(c) => c,
                Err(why) => {
                    uncarried.get_or_insert(why);
                    continue;
                }
            };
            let Some(versor) = placed(a, b, &dilator.geo(&rotation)) else {
                uncarried.get_or_insert(super::continuous::NotCarried::Centre);
                continue;
            };
            out.push(Transition { rotation, versor, residue: covariance.residue, unread: covariance.unread });
        }
    }
    // A parity with no rational turn tries no candidate; every other arm of
    // the turn builds one, so an empty coset with no candidate's refusal is
    // the oblique turn's.
    match (out.is_empty(), uncarried) {
        (false, _) => Ok(out),
        (true, Some(why)) => Err(NoPose::Uncarried(why)),
        (true, None) => Err(NoPose::Oblique),
    }
}

/// THE PLACEMENT OF A LINEAR ACTION between two bodies: `T(c_b − V(c_a))·V`,
/// the one freedom a listing's frame quotients solved from the two centres
/// exactly (24.1). `None` where the centre goes to infinity.
fn placed(a: &Blade, b: &Blade, linear: &Mv) -> Option<Mv> {
    let (ca, cb) = (vertex_centroid(a)?, vertex_centroid(b)?);
    let image = cga::base_at(&linear.sandwich(&cga::point_at(&ca)), ca.len())?;
    Some(cga::translator_at(&difference(&cb, &image)?).geo(linear))
}

/// THE CANONICAL REPRESENTATIVE OF A COSET (50.2 step 1 as ruled, law 6):
/// the member nearest the identity in the Cayley graph — the coset's
/// geodesic — read as the greatest `a²/w` of its rotation part (15.3's
/// class invariant, an exact rational; a reflected member reads zero),
/// ties broken by the least A21 key of the canonical rotation. A gauge
/// fixed by content, A1's sign step at the altitude of a coset: two voices
/// measuring one pair return one versor, never the first that verified.
/// Not a choice among candidates by a scalar (rule 4) — every member
/// carries the body; what is fixed is which name the language gives the
/// class, and a dilated bar is carried by the dilation, not by the
/// dilation and the bar's own 180° turn.
pub fn coset_representative(coset: &[Transition]) -> Option<&Transition> {
    coset_geodesic(coset).map(|at| &coset[at])
}

/// Where the coset's geodesic stands in it (see [`coset_representative`]);
/// `None` where no member's rotation reads a weight or a key.
fn coset_geodesic(coset: &[Transition]) -> Option<usize> {
    let nearness = |rotation: &Mv| -> Option<BigRational> {
        // `a²/w`, scale-free, read as the class's order (Axiom C).
        let w = cga::weight_of(rotation)?;
        let a = rotation.scalar_part();
        Some(&a * &a / w)
    };
    let mut geodesic: Option<(BigRational, Vec<u8>, usize)> = None;
    for (at, Transition { rotation, .. }) in coset.iter().enumerate() {
        let Some(n) = nearness(rotation) else { continue };
        let Some(key) = rotation.key() else { continue };
        let key = key.into_vec();
        let nearer = match &geodesic {
            None => true,
            Some((held, held_key, _)) => n > *held || (n == *held && key < *held_key),
        };
        if nearer {
            geodesic = Some((n, key, at));
        }
    }
    geodesic.map(|(_, _, at)| at)
}

/// THE COSET AS CANDIDATES, GEODESIC FIRST (50.2 step 1, L7c): the
/// representative [`coset_representative`] names, then every other member
/// by its own A21 key — a content-fixed order, so two voices reading one
/// pair walk the same list in the same order and nothing here is chosen by
/// a scalar (rule 4).
///
/// Every member carries the body; they differ in which word the alphabet
/// spells them with. [`versor_between`] answers the geodesic alone, and a
/// caller that needs a reading — not a name — had no way to reach the
/// rest, so a pair whose geodesic the seventeen could not spell was filed
/// as carrying no transport at all while a turning member of the same
/// coset spelled fine. The geodesic stays first, so naming and replay are
/// unchanged.
pub fn versor_coset_ordered(
    a: &Shape,
    b: &Shape,
    budget: &mut crate::physics::Budget,
) -> Result<Vec<Transition>, NoPose> {
    let coset = versor_coset(a, b, budget)?;
    let geodesic = coset_geodesic(&coset).ok_or(NoPose::Ungauged)?;
    // Each member filed by its own coordinates (A21): the keyed carrier IS
    // the content order, and a member standing twice is one key. The
    // geodesic is set apart by its place, never compared by bytes.
    let first_key = coset[geodesic].versor.key().map(|k| k.into_vec());
    let mut order: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
    for (at, t) in coset.iter().enumerate() {
        let Some(key) = t.versor.key().map(|k| k.into_vec()) else { continue };
        if at == geodesic || first_key.as_deref() == Some(key.as_slice()) {
            continue;
        }
        order.insert(&key, at);
    }
    let rest: Vec<usize> = order.scoped().map(|(_, &at)| at).collect();
    let mut slots: Vec<Option<Transition>> = coset.into_iter().map(Some).collect();
    Ok(std::iter::once(geodesic).chain(rest).filter_map(|at| slots.get_mut(at).and_then(Option::take)).collect())
}

/// DOES `v` CARRY `a` ONTO `b` ON ONE TOWER, THROUGH AN ORDER: the
/// transition's covariance (`ContinuousSupport::matches`) — the centre, the
/// second moment on every plane, the odd and quartic harmonics on the base's
/// planes — never where a carried compartment lands. The residue above `through`
/// where it carries; where it does not, why, by name. The verification every
/// member of the coset passes, and the one a held word's action passes on a
/// listing body (24.1, 50.1 step 1).
pub fn carries_by_covariance(
    v: &Mv,
    a: &Blade,
    b: &Blade,
    tower: super::tower::Tower,
    through: super::tower::Harmonic,
) -> Result<super::continuous::Covariance, NoPose> {
    let (Some(sa), Some(sb)) = (ContinuousSupport::of(a, tower), ContinuousSupport::of(b, tower)) else {
        return Err(NoPose::Bodiless);
    };
    sa.matches(&sb, v, through).map_err(NoPose::Uncarried)
}

/// A LINEAR ACTION PLACED AND VERIFIED on one tower through an order: the
/// transition where it carries `a` onto `b`; where it does not, why.
pub(super) fn carried_on(
    a: &Blade,
    b: &Blade,
    linear: &Mv,
    tower: super::tower::Tower,
    through: super::tower::Harmonic,
) -> Result<Transition, NoPose> {
    let versor = placed(a, b, linear).ok_or(NoPose::Uncarried(super::continuous::NotCarried::Centre))?;
    let covariance = carries_by_covariance(&versor, a, b, tower, through)?;
    Ok(Transition { rotation: linear.clone(), versor, residue: covariance.residue, unread: covariance.unread })
}

/// THE CARRIAGE OF A GIVEN LINEAR ACTION between two bodies (24.1 as ruled
/// 2026-09-09): `linear` is a rotation with its dilation composed — a held
/// word's rotor under its dilator — and the one freedom a listing's frame
/// quotients is the translation, solved from the two centres exactly as
/// the transition solves it. No candidate is tried: the word says which
/// rotation, the bodies say where, and the result is verified by the
/// transition's covariance through the fit the certificate reads, its
/// residue carried with it. Where the action does not carry `a` onto `b` —
/// the word is not a word for this body — the refusal names why; a spent
/// stroke is the typed cut.
pub fn versor_carrying(
    a: &Shape,
    b: &Shape,
    linear: &Mv,
    budget: &mut crate::physics::Budget,
) -> Result<Transition, NoPose> {
    if a.blade.support.is_empty() || b.blade.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    let fit = certify(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE)
        .map_err(|(region, sites)| NoPose::Incongruent(Incongruence::Order { region, sites }))?;
    // One candidate verified: one read from the stroke (65.5).
    if !budget.spend() {
        return Err(NoPose::Unbounded);
    }
    carried_on(&a.blade, &b.blade, linear, fit.dilation.tower(), fit.through)
}
