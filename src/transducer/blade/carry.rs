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

/// Apply a versor to a point of the sensor's vertex coordinates and read
/// the base back; `None` at infinity.
pub fn carry(v: &Mv, r: i64, c: i64) -> Option<(BigRational, BigRational)> {
    cga::base(&v.sandwich(&cga::point(&q(r), &q(c))))
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

/// THE TRANSPORT BETWEEN TWO BODIES, in closed form from their moments.
/// Translation from the body centroids; the dilation ρ from the per-cell
/// spread (a rational square, or the bodies are not similar in ℚ — a
/// refusal, never a rounding); the rotation from the moment harmonics: the
/// inertia's anisotropy turns at twice the body's angle, so the rotor
/// carrying `a`'s anisotropy onto `b`'s is halved once; an isotropic body
/// is oriented by its fourth harmonic, halved twice. Each harmonic leaves a
/// finite ambiguity — the body's own stabilizer (a half turn for the
/// second harmonic, quarter turns for the fourth) — and a reflection is
/// tried as the odd versor; every candidate is verified by carrying every
/// cell center of `a` into a cell of `b` and every cell center of `b` back
/// into a cell of `a`. `None` when no rational versor carries `a` onto `b`.
pub fn versor_between(a: &Shape, b: &Shape) -> Option<Mv> {
    if a.blade.cells.is_empty() || b.blade.cells.is_empty() {
        return None;
    }
    // Body centroids in the sensor's vertex coordinates: the cell-center
    // centroid plus one half on each axis.
    let (ca1, ca2) = a.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let (cb1, cb2) = b.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let spread = |s: &Shape| (&s.blade.inertia[0] + &s.blade.inertia[1]) / &s.blade.shell;
    let (sa, sb) = (spread(a), spread(b));
    let rho = match sa.is_zero() {
        true => match sb.is_zero() {
            true => q(1),
            false => return None,
        },
        false => rational_sqrt(&(&sb / &sa))?,
    };
    if b.blade.shell != &a.blade.shell * &rho * &rho {
        return None; // the cell counts do not scale as ρ²: not similar
    }
    let to_origin = cga::translator(&(-&ca1), &(-&ca2));
    let to_target = cga::translator(&cb1, &cb2);
    let dil = cga::dilator(&rho);
    // The stabilizer turns and the orientation sheet are WORDS of the
    // alphabet realised as conformal versors — the quarter letter on the
    // sensor's plane, its square, and the half turn through the third axis
    // (the plane's flip). Nothing here is tabulated.
    use crate::transducer::alphabet::{LetterId, Word, PLANE_COL, PLANE_ROW};
    let realise = |tape: &[(LetterId, bool)]| -> Option<Mv> {
        crate::transducer::element::versor_of_word(&Word::from_letters(tape))
    };
    let quarter = realise(&[(LetterId::quarter((PLANE_ROW, PLANE_COL))?, false)])?;
    let half_turn = quarter.geo(&quarter);
    let flip_letter = LetterId::quarter((PLANE_COL, 2))?;
    let sheet = realise(&[(flip_letter, false), (flip_letter, false)])?;
    let verify = |v: &Mv| -> bool { carries(v, a, b) };
    for reflect in [false, true] {
        let a2 = a.blade.harmonic2(reflect);
        let b2 = b.blade.harmonic2(false);
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
                let a4 = a.blade.harmonic4(reflect);
                let b4 = b.blade.harmonic4(false);
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
            let mut v = rotor.geo(&to_origin);
            if reflect {
                v = rotor.geo(&sheet).geo(&to_origin);
            }
            v = to_target.geo(&dil).geo(&v);
            if verify(&v) {
                return Some(v);
            }
        }
    }
    None
}

/// EVERY CELL CARRIED, BOTH WAYS: `v` carries each cell center of `a` into
/// a cell of `b`, and its inverse carries each cell center of `b` back into
/// a cell of `a`. The verification every candidate of `versor_between`
/// passes, and the one a held word's action passes on a listing body
/// (24.1, 50.1 step 1). A versor with no inverse carries nothing.
pub fn carries(v: &Mv, a: &Shape, b: &Shape) -> bool {
    let lands = |v: &Mv, r: i64, c: i64, held: &[(i64, i64)]| -> bool {
        match crate::transducer::element::carry_center(v, r, c) {
            Some(p) => held.binary_search(&crate::transducer::element::cell_of(&p)).is_ok(),
            None => false,
        }
    };
    let Some(inv) = v.versor_inverse() else { return false };
    a.blade.cells.iter().all(|&(r, c)| lands(v, r, c, &b.blade.cells))
        && b.blade.cells.iter().all(|&(r, c)| lands(&inv, r, c, &a.blade.cells))
}

/// THE CARRIAGE OF A GIVEN LINEAR ACTION between two bodies (24.1 as ruled
/// 2026-09-09): `linear` is a rotation with its dilation composed — a held
/// word's rotor under its dilator — and the one freedom a listing's frame
/// quotients is the translation, solved from the two centroids exactly as
/// `versor_between` solves it. Nothing is measured from the harmonics and
/// no candidate is tried: the word says which rotation, the bodies say
/// where, and the result is verified cell by cell both ways. `None` when
/// the action does not carry `a` onto `b`: the word is not a word for
/// this body.
pub fn versor_carrying(a: &Shape, b: &Shape, linear: &Mv) -> Option<Mv> {
    if a.blade.cells.is_empty() || b.blade.cells.is_empty() {
        return None;
    }
    let (ca1, ca2) = a.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let (cb1, cb2) = b.blade.centroid().map(|(x, y)| (x + qn(1, 2), y + qn(1, 2)))?;
    let (ia, ib) = cga::base(&linear.sandwich(&cga::point(&ca1, &ca2)))?;
    let v = cga::translator(&(cb1 - ia), &(cb2 - ib)).geo(linear);
    carries(&v, a, b).then_some(v)
}
