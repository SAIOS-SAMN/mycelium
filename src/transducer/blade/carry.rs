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

/// A body's centroid in the sensor's VERTEX coordinates: the laminae's
/// centres' centroid plus one half on each grid axis (a lamina's square
/// spans the grid axes), and the offset alone on every other axis — a
/// lamina stands AT its frame (16.4).
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

/// The unnormalized rotor carrying direction `a` onto direction `b` in the
/// e₁e₂ plane: `|a||b| + b·a + (b∧a) e₁₂`. Exact when `|a||b|` is rational
/// (the two lengths' product is a rational square), else refused.
pub(crate) fn rotor_between(a: &[BigRational; 2], b: &[BigRational; 2]) -> Option<(BigRational, BigRational)> {
    let aa = &a[0] * &a[0] + &a[1] * &a[1];
    let bb = &b[0] * &b[0] + &b[1] * &b[1];
    let ab = rational_sqrt(&(&aa * &bb))?;
    let s = &ab + &b[0] * &a[0] + &b[1] * &a[1];
    let bv = &b[0] * &a[1] - &b[1] * &a[0];
    Some((s, bv))
}

/// The rotor of half the angle of `s + b e₁₂`: `(|r| + s) + b e₁₂`, exact
/// when `|r|` is rational. A half turn (`s < 0`, `b = 0`) halves to the
/// quarter turn `1 + e₁₂`... whose own half-angle form is `e₁₂` itself when
/// the rotor is the pure bivector.
pub(crate) fn half_rotor(s: &BigRational, b: &BigRational) -> Option<(BigRational, BigRational)> {
    let m2 = s * s + b * b;
    if m2.is_zero() {
        return None;
    }
    let m = rational_sqrt(&m2)?;
    let s2 = &m + s;
    match (s2.is_zero(), b.is_zero()) {
        (true, true) => Some((q(0), q(1))), // the half of a half turn: e₁₂
        _ => Some((s2, b.clone())),
    }
}

/// THE TRANSPORT BETWEEN TWO BODIES, in closed form from their moments:
/// the canonical representative (50.2 step 1 as ruled 2026-09-09) of the
/// coset `versor_coset` measures. `None` when no rational versor carries
/// `a` onto `b`.
pub fn versor_between(a: &Shape, b: &Shape) -> Option<Mv> {
    coset_representative(&versor_coset(a, b))
}

/// The rotation measured as a Cl(4,0) Rotor across turned planes from the
/// deviatoric tensors, and the translation placement as a CGA translator.
pub fn rotor_and_placement_between(
    a: &Shape,
    b: &Shape,
) -> Option<(crate::transducer::alphabet::Rotor, Mv)> {
    use crate::transducer::alphabet::Rotor;
    let _ = versor_between(a, b)?;
    let t = cga::translator_at(&difference(&b.blade.centroid()?, &a.blade.centroid()?)?);

    let mut total_rotor = Rotor::identity();
    // THE BODY'S OWN PLANES, generated from its `n` (BQ4), each read in the
    // rotor chart's slots (51.2).
    //
    // NOTHING IS PASSED OVER UNREAD (audit N-4). A plane `b` does not have is
    // no rotor between the two, and refuses. A plane on which either body is
    // isotropic has no orientation to turn, which is a reading. A plane that
    // orients both and has no slot in the letters' chart carries a turn the
    // seventeen cannot write (71.2(3)), and REFUSES: this `filter_map`ped
    // such planes away and minted the rotor of the rest, a turn dropped and
    // the result presented as whole.
    for plane in a.blade.planes() {
        let ha = a.blade.harmonic_on_plane(plane, false)?;
        let hb = b.blade.harmonic_on_plane(plane, false)?;
        if (ha[0].is_zero() && ha[1].is_zero()) || (hb[0].is_zero() && hb[1].is_zero()) {
            continue;
        }
        let slot = crate::transducer::alphabet::slot_plane(plane)?;
        if let Some((s, bv)) = rotor_between(&ha, &hb) {
            let (s, bv) = match (s.is_zero(), bv.is_zero()) {
                (true, true) => (q(0), q(1)),
                _ => (s, bv),
            };
            if let Some((hs, hb)) = half_rotor(&s, &bv) {
                let mask = (1usize << slot.0) | (1usize << slot.1);
                let r_plane = Rotor::scalar(hs).add(&Rotor::blade(mask, hb));
                total_rotor = total_rotor.geo(&r_plane);
            }
        }
    }
    Some((total_rotor, t))
}

/// The Cl(4,0) Rotor carrying body a's orientation onto body b's
/// orientation: the base rotor of the transition's geodesic (51.2), read
/// back from the versor the transition verified.
pub fn rotor_between_shapes(a: &Shape, b: &Shape) -> Option<crate::transducer::alphabet::Rotor> {
    crate::transducer::element::base_rotor_of(&versor_between(a, b)?)
}

/// THE COSET OF VERSORS carrying `a` onto `b` on the sensor's plane (BQ4,
/// named), each with the rotation it was built from: the transition's
/// members ([`transitions_between`]); none where the pair is refused, and
/// the refusal is the pose's to name.
pub fn versor_coset(a: &Shape, b: &Shape) -> Vec<(Mv, Mv)> {
    transitions_between(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE).into_iter().flatten().collect()
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
///   tower    the one `dilation` certifies; both refusing is Incongruent(Order)
///   scale    ρ² = s_b / s_a on that tower; no rational ρ, no member
///   turn     the second harmonic's point of 2φ, one exact half-angle step;
///            isotropic there, the fourth's point of 4φ and two steps
///   coset    {R, R·e_ij} from the second; {R·Q_k}, the four quarter turns,
///            from the fourth; none chosen (L7c)
///   odd      the same against a's harmonics conjugated — `a` mirrored across
///            the plane's first axis — and the member R·e_j
///   place    T(c_b − V_lin(c_a)) · V_lin,  V_lin = D(ρ)·R  (·e_j)
///   verify   the tower carried by V (`ContinuousSupport::matches`)
/// ```
///
/// No member lands a lamina: the verification is the covariance of the
/// tower. An empty list is a pair no rational similarity carries on this
/// plane — its reason the pose's to name (`unturned`): where the half-angle
/// point or `ρ` is not rational, Oblique, the RotorPrime demand. Refused
/// before any candidate: a bodiless pair, a pair neither tower certifies, a
/// pair where one body orients and the other does not.
pub fn transitions_between(a: &Blade, b: &Blade, plane: (usize, usize)) -> Result<Vec<(Mv, Mv)>, NoPose> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    match dilation(a, b, plane) {
        Ok(d) => transitions_on(a, b, plane, d.tower()),
        Err((region, sites)) => Err(NoPose::Incongruent(Incongruence::Order { region, sites })),
    }
}

/// THE TRANSITION ON THE TOWER ALREADY CERTIFIED: the pose reads the
/// certificate once and hands its tower here, so no pair is certified twice.
pub(super) fn transitions_on(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    tower: super::tower::Tower,
) -> Result<Vec<(Mv, Mv)>, NoPose> {
    use super::tower::{harmonic, scale2, Harmonic};
    let (Some(sa), Some(sb)) = (ContinuousSupport::of(a, tower), ContinuousSupport::of(b, tower)) else {
        return Err(NoPose::Bodiless);
    };
    let (Some(ra), Some(rb)) = (scale2(a, plane, tower), scale2(b, plane, tower)) else {
        return Err(NoPose::Bodiless);
    };
    if ra.is_zero() || rb.is_zero() {
        return Err(NoPose::Unoriented);
    }
    let mut out: Vec<(Mv, Mv)> = Vec::new();
    let Some(rho) = rational_sqrt(&(&rb / &ra)) else {
        return Ok(out);
    };
    let rho2 = &rho * &rho;
    let dilator = cga::dilator(&rho);
    let e_ij = Mv::blade(cga::base_axis(plane.0 + 1) | cga::base_axis(plane.1 + 1), q(1));
    let mirror = Mv::blade(cga::base_axis(plane.1 + 1), q(1));
    // The four quarter turns of the plane as rays: 0°, 90°, 180°, 270°.
    let one = Mv::scalar(q(1));
    let quarters = [one.clone(), one.add(&e_ij), e_ij.clone(), e_ij.sub(&one)];
    for odd in [false, true] {
        let read = |m: Harmonic| {
            let ha = harmonic(a, plane, tower, m).map(|h| match odd {
                true => [h[0].clone(), -&h[1]],
                false => h,
            });
            (ha, harmonic(b, plane, tower, m))
        };
        let rotors: Vec<Mv> = match (read(Harmonic::Two), read(Harmonic::Four)) {
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
                        quarters.iter().map(|k| r.geo(k)).collect()
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
            let rotation = match odd {
                true => rotor.geo(&mirror),
                false => rotor,
            };
            // The candidate's rotation and scale are in hand: its covariance is
            // read on them, and only a member that carries is placed — the
            // placement sends `c_a` to `c_b` by construction.
            if !sa.carries_linear(&sb, &rotation, &rho) {
                continue;
            }
            let Some(v) = placed(a, b, &dilator.geo(&rotation)) else { continue };
            out.push((rotation, v));
        }
    }
    Ok(out)
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
/// dilation and the bar's own half turn.
pub fn coset_representative(coset: &[(Mv, Mv)]) -> Option<Mv> {
    let nearness = |rotation: &Mv| -> Option<BigRational> {
        // `a²/w`, scale-free, read as the class's order (Axiom C).
        let w = cga::weight_of(rotation)?;
        let a = rotation.scalar_part();
        Some(&a * &a / w)
    };
    let mut geodesic: Option<(BigRational, Vec<u8>, &Mv)> = None;
    for (rotation, v) in coset {
        let Some(n) = nearness(rotation) else { continue };
        let Some(key) = rotation.key() else { continue };
        let key = key.into_vec();
        let nearer = match &geodesic {
            None => true,
            Some((held, held_key, _)) => n > *held || (n == *held && key < *held_key),
        };
        if nearer {
            geodesic = Some((n, key, v));
        }
    }
    geodesic.map(|(_, _, v)| v.clone())
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
pub fn versor_coset_ordered(a: &Shape, b: &Shape) -> Vec<Mv> {
    let coset = versor_coset(a, b);
    let geodesic = coset_representative(&coset);
    // The key IS the member's coordinates written inline (A21), so sorting
    // the keys is sorting by content and the member rides along by the
    // position it was measured at — never the other way round.
    let mut rest: Vec<(Vec<u8>, usize)> = coset
        .iter()
        .enumerate()
        .filter(|(_, (_, v))| geodesic.as_ref() != Some(v))
        .filter_map(|(at, (_, v))| v.key().map(|k| (k.into_vec(), at)))
        .collect();
    rest.sort();
    rest.dedup_by(|x, y| x.0 == y.0);
    geodesic.into_iter().chain(rest.into_iter().map(|(_, at)| coset[at].1.clone())).collect()
}

/// DOES `v` CARRY `a` ONTO `b` ON ONE TOWER: the transition's covariance
/// (`ContinuousSupport::matches`) — the centre, the second moment on every
/// plane, the odd and quartic harmonics on the base's planes — never where a
/// carried lamina lands. The verification every member of the coset passes,
/// and the one a held word's action passes on a listing body (24.1, 50.1
/// step 1).
pub fn carries_by_covariance(v: &Mv, a: &Blade, b: &Blade, tower: super::tower::Tower) -> bool {
    match (ContinuousSupport::of(a, tower), ContinuousSupport::of(b, tower)) {
        (Some(sa), Some(sb)) => sa.matches(&sb, v),
        _ => false,
    }
}

/// A LINEAR ACTION PLACED AND VERIFIED on one tower: the versor where it
/// carries `a` onto `b`, `None` where it does not.
pub(super) fn carried_on(a: &Blade, b: &Blade, linear: &Mv, tower: super::tower::Tower) -> Option<Mv> {
    let v = placed(a, b, linear)?;
    carries_by_covariance(&v, a, b, tower).then_some(v)
}

/// THE CARRIAGE OF A GIVEN LINEAR ACTION between two bodies (24.1 as ruled
/// 2026-09-09): `linear` is a rotation with its dilation composed — a held
/// word's rotor under its dilator — and the one freedom a listing's frame
/// quotients is the translation, solved from the two centres exactly as
/// the transition solves it. No candidate is tried: the word says which
/// rotation, the bodies say where, and the result is verified by the
/// transition's covariance on the tower `dilation` certifies. `None` when
/// the action does not carry `a` onto `b`: the word is not a word for this
/// body.
pub fn versor_carrying(a: &Shape, b: &Shape, linear: &Mv) -> Option<Mv> {
    if a.blade.support.is_empty() || b.blade.support.is_empty() {
        return None;
    }
    let tower = dilation(&a.blade, &b.blade, crate::transducer::base::GRID_PLANE).ok()?.tower();
    carried_on(&a.blade, &b.blade, linear, tower)
}
