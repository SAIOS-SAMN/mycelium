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
fn vertex_centroid(b: &Blade) -> Option<Vec<BigRational>> {
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

/// The Cl(4,0) Rotor carrying body a's orientation onto body b's orientation.
pub fn rotor_between_shapes(a: &Shape, b: &Shape) -> Option<crate::transducer::alphabet::Rotor> {
    let (r, _) = rotor_and_placement_between(a, b)?;
    if let (Some(sa), Some(sb)) = (
        crate::transducer::blade::continuous::ContinuousSupport::of(a),
        crate::transducer::blade::continuous::ContinuousSupport::of(b),
    ) {
        if !sa.matches_rotor(&sb, &r) {
            return None;
        }
    }
    Some(r)
}

/// THE COSET OF VERSORS carrying `a` onto `b`, each with the rotation it
/// was built from. Translation from the body centroids; the dilation ρ
/// from the per-lamina spread (a rational square, or the bodies are not
/// similar in ℚ — a refusal, never a rounding); the rotation from the
/// moment harmonics: the inertia's anisotropy turns at twice the body's
/// angle, so the rotor carrying `a`'s anisotropy onto `b`'s is halved
/// once; an isotropic body is oriented by its fourth harmonic, halved
/// twice. Each harmonic leaves a finite ambiguity — the body's own
/// stabilizer (a half turn for the second harmonic, quarter turns for the
/// fourth) — and a reflection is tried as the odd versor; every candidate
/// is verified by carrying every lamina's center of `a` onto a lamina of
/// `b` and every lamina's center of `b` back onto a lamina of `a`. The
/// ones that carry are one coset of the stabilizer acting on one
/// transport, and none is chosen here: the coset is what is measured
/// (L7c), its representative is `coset_representative`'s.
pub fn versor_coset(a: &Shape, b: &Shape) -> Vec<(Mv, Mv)> {
    let mut coset: Vec<(Mv, Mv)> = Vec::new();
    if a.blade.support.is_empty() || b.blade.support.is_empty() {
        return coset;
    }
    // Body centroids in the sensor's vertex coordinates: the centroid of
    // the laminae's centers plus one half on each plane axis; a lamina
    // stands AT its frame (16.4).
    let (Some(ca), Some(cb)) = (vertex_centroid(&a.blade), vertex_centroid(&b.blade)) else {
        return coset;
    };
    let spread = |s: &Shape| {
        // The trace on every axis the body has (PH2).
        (0..s.blade.n())
            .filter_map(|i| s.blade.moment2(i, i))
            .fold(BigRational::zero(), |acc, x| acc + x)
            / &s.blade.shell
    };
    let (sa, sb) = (spread(a), spread(b));
    // ρ is ONE rational from the per-lamina spread, so a homothety with
    // different horizontal and vertical factors — a stretch — is not a
    // similarity and is refused exactly. Carrying an anisotropic scale would
    // need a carrier that holds two factors, which is a ruling and not a
    // repair.
    let rho = match sa.is_zero() {
        true => match sb.is_zero() {
            true => q(1),
            false => return coset,
        },
        false => match rational_sqrt(&(&sb / &sa)) {
            Some(r) => r,
            None => return coset,
        },
    };
    // SQ1-b5: no scalar prefilter stands between ρ and the versor. With ρ in
    // hand, D(ρ) is built and applied below, and `carries` reads whether the
    // support lands; the lamina count scaling as ρ² is the sensor's unit
    // squares, a reading (M₀), never a gate (rule 2, checklist (q)).
    // The bodies stand in their frames: the transport returns the frame
    // coordinate as it does the two of the plane.
    let to_origin = cga::translator_at(&ca.iter().map(|x| -x).collect::<Vec<_>>());
    let to_target = cga::translator_at(&cb);
    let dil = cga::dilator(&rho);
    // The stabilizer turns and the orientation sheet are WORDS of the
    // alphabet realised as conformal versors — the quarter letter on the
    // sensor's plane, its square, and the half turn through the third axis
    // (the plane's flip). Nothing here is tabulated.
    use crate::transducer::alphabet::{LetterId, Word, PLANE_COL, PLANE_ROW};
    let realise = |tape: &[(LetterId, bool)]| -> Option<Mv> {
        crate::transducer::element::versor_of_word(&Word::from_letters(tape))
    };
    let Some(quarter) = LetterId::quarter((PLANE_ROW, PLANE_COL)).and_then(|q| realise(&[(q, false)])) else {
        return coset;
    };
    let half_turn = quarter.geo(&quarter);
    let Some(sheet) = LetterId::quarter((PLANE_COL, 2)).and_then(|f| realise(&[(f, false), (f, false)])) else {
        return coset;
    };
    // THE COSET IS READ ON THE SENSOR'S PLANE, where its quarter letter and
    // its rotator stand (BQ4: named, never assumed).
    let plane = crate::transducer::base::GRID_PLANE;
    for reflect in [false, true] {
        let (Some(a2), Some(b2)) =
            (a.blade.harmonic_on_plane(plane, reflect), b.blade.harmonic_on_plane(plane, false))
        else {
            continue;
        };
        let iso_a = a2[0].is_zero() && a2[1].is_zero();
        let iso_b = b2[0].is_zero() && b2[1].is_zero();
        // The rotation candidates: the rotor of the body's angle and its
        // stabilizer under the harmonic that oriented it.
        let candidates: Vec<Mv> = match (iso_a, iso_b) {
            (false, false) => {
                let Some((s, bv)) = rotor_between(&a2, &b2) else { continue };
                let (s, bv) = match (s.is_zero(), bv.is_zero()) {
                    (true, true) => (q(0), q(1)), // antiparallel anisotropy: a half turn of the harmonic
                    _ => (s, bv),
                };
                let Some((hs, hb)) = half_rotor(&s, &bv) else { continue };
                let r = cga::rotator(&hs, &hb);
                vec![r.clone(), r.geo(&half_turn)]
            }
            (true, true) => {
                let (Some(a4), Some(b4)) =
                    (a.blade.harmonic4_on(plane, reflect), b.blade.harmonic4_on(plane, false))
                else {
                    continue;
                };
                let iso4 = (a4[0].is_zero() && a4[1].is_zero()) || (b4[0].is_zero() && b4[1].is_zero());
                match iso4 {
                    true => vec![Mv::scalar(q(1))],
                    false => {
                        let Some((s, bv)) = rotor_between(&a4, &b4) else { continue };
                        let (s, bv) = match (s.is_zero(), bv.is_zero()) {
                            (true, true) => (q(0), q(1)),
                            _ => (s, bv),
                        };
                        let Some((s1, b1)) = half_rotor(&s, &bv) else { continue };
                        let Some((s2, b2)) = half_rotor(&s1, &b1) else { continue };
                        let r = cga::rotator(&s2, &b2);
                        let r90 = r.geo(&quarter);
                        let r180 = r90.geo(&quarter);
                        let r270 = r180.geo(&quarter);
                        vec![r, r90, r180, r270]
                    }
                }
            }
            _ => continue, // one body isotropic, the other not: not similar
        };
        for rotor in candidates {
            if cga::is_versor(&rotor).is_none() {
                continue;
            }
            let rotation = if reflect { rotor.geo(&sheet) } else { rotor };
            let v = to_target.geo(&dil).geo(&rotation).geo(&to_origin);
            if carries(&v, a, b) {
                coset.push((rotation, v));
            }
        }
    }
    coset
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
        let w = cga::is_versor(rotation)?;
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

/// EVERY LAMINA CARRIED, BOTH WAYS: `v` carries each lamina's center of
/// `a` onto a lamina of `b`, and its inverse carries each lamina's center
/// of `b` back onto a lamina of `a`. The verification every candidate of
/// `versor_between` passes, and the one a held word's action passes on a
/// listing body (24.1, 50.1 step 1). A versor with no inverse carries
/// nothing.
pub fn carries(v: &Mv, a: &Shape, b: &Shape) -> bool {
    use crate::transducer::base::Site;
    // A centre that lands at infinity or off the base lands on no held lamina.
    let lands = |v: &Mv, site: &Site, held: &[Site]| -> bool {
        match crate::transducer::element::carry_site(v, site) {
            Some(s) => held.binary_search(&s).is_ok(),
            None => false,
        }
    };
    let Some(inv) = v.versor_inverse() else { return false };
    a.blade.support.iter().all(|site| lands(v, site, &b.blade.support))
        && b.blade.support.iter().all(|site| lands(&inv, site, &a.blade.support))
}

/// THE CARRIAGE OF A GIVEN LINEAR ACTION between two bodies (24.1 as ruled
/// 2026-09-09): `linear` is a rotation with its dilation composed — a held
/// word's rotor under its dilator — and the one freedom a listing's frame
/// quotients is the translation, solved from the two centroids exactly as
/// `versor_between` solves it. Nothing is measured from the harmonics and
/// no candidate is tried: the word says which rotation, the bodies say
/// where, and the result is verified lamina by lamina both ways. `None`
/// when the action does not carry `a` onto `b`: the word is not a word for
/// this body.
pub fn versor_carrying(a: &Shape, b: &Shape, linear: &Mv) -> Option<Mv> {
    if a.blade.support.is_empty() || b.blade.support.is_empty() {
        return None;
    }
    let (ca, cb) = (vertex_centroid(&a.blade)?, vertex_centroid(&b.blade)?);
    let image = cga::base_at(&linear.sandwich(&cga::point_at(&ca)), ca.len())?;
    let v = cga::translator_at(&difference(&cb, &image)?).geo(linear);
    carries(&v, a, b).then_some(v)
}
