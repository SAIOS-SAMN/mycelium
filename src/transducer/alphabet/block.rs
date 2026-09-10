// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 37.7 P2, A3 — THE PLANE BLOCK AS A GAUSSIAN INTEGER, and the rotor a
//! block is the action of.
//!
//! A rotation block over ℚ is a Gaussian integer up to a rational scale,
//! and a rotor is that integer's square root, because a rotor is a
//! half-angle. Both readings are exact or refused; nothing is rounded and
//! nothing is searched for.
//!
//! Moved here from `family/ring.rs`, unchanged (§1.6). They were private
//! to the ring branch, which made them unreachable from the two other
//! places that need the same arithmetic: the versor readers in `element`,
//! which classify a meet's transition by its rotor part, and the court's
//! complex, which re-derives that classification. Arithmetic on a rotor's
//! block belongs beside the rotor, not inside one caller.

use crate::core::gaussian::Gauss;
use crate::core::Q;

/// A plane block `[[s, −t], [t, s]]` as the Gaussian integer `s + t·i`,
/// with the common denominator cleared EXACTLY. A rotation block over ℚ
/// is `π/ρ` for a Gaussian `π`; clearing the denominator recovers `π` up
/// to a rational scale, and scale is what the ring divides out anyway —
/// identity is projective (1.5). `None` when the block is not a rotation
/// block, which is not an arc at all.
pub(crate) fn gauss_of_block(block: &[[Q; 2]; 2]) -> Option<Gauss> {
    use num_integer::Integer;
    let (s, t) = (&block[0][0], &block[1][0]);
    if &block[1][1] != s || block[0][1] != -t.clone() {
        return None;
    }
    let d = s.denom().lcm(t.denom());
    let a = (s * num_rational::BigRational::from_integer(d.clone())).to_integer();
    let b = (t * num_rational::BigRational::from_integer(d)).to_integer();
    let g = Gauss::new(a, b);
    match g.is_zero() {
        true => None,
        false => Some(g),
    }
}

/// The inverse reading: `s + t·i` written back as its plane block.
pub(crate) fn block_of_gauss(g: &Gauss) -> [[Q; 2]; 2] {
    super::arc_block(&g.a, &g.b)
}

/// The rotor a plane block is the action of: the block's arc is the
/// rotor's SQUARE (a rotor is a half-angle), so the rotor is the arc's
/// square root in ℤ[i]. Found by the ring, never searched for. `None`
/// when the arc has no square root there, which is a block no rotor over
/// ℚ induces.
///
/// THE SIGN, measured rather than assumed: `rotor_block_on` writes the
/// rotor `x + y·e_ij` as `[[x²−y², 2xy], [−2xy, x²−y²]]`, and an arc is
/// read here as `s + t·i` with `t = block[1][0]`. So the arc of that
/// rotor is `(x²−y²) − 2xy·i = (x − y·i)²`: the square root's imaginary
/// part is `−y`, and the blade coefficient is its negation.
pub(crate) fn rotor_of_block(block: &[[Q; 2]; 2], plane: (u8, u8)) -> Option<super::Rotor> {
    use super::Rotor;
    let arc = gauss_of_block(block)?;
    let root = arc_half(&arc)?;
    let mask = (1usize << plane.0) | (1usize << plane.1);
    let q = |n: &num_bigint::BigInt| num_rational::BigRational::from_integer(n.clone());
    Some(Rotor::scalar(q(&root.a)).add(&Rotor::blade(mask, -q(&root.b))))
}

/// THE WORD A PLANE BLOCK SPELLS, where A22's peel closes on it: the
/// cores in reverse, which is the one shape three callers had each written
/// out for themselves (`peel_versor`, the ring's cofactor, and the cover's
/// deleted `rotor_of_transition`). `None` where the peel stands, which is
/// a block the seventeen do not reach — a refusal, never a shorter word.
pub(crate) fn word_of_block(block: &[[Q; 2]; 2], plane: (u8, u8)) -> Option<super::Word> {
    match super::spell_peel_on(plane, block)? {
        (crate::transducer::peel::PeelNest::Closed { cores }, _) => {
            let mut w = super::Word::identity();
            for syllable in cores.iter().rev() {
                for &(id, inv) in syllable {
                    w.push(id, inv);
                }
            }
            Some(w)
        }
        _ => None,
    }
}

/// The word a ROTOR spells on the sensor plane, through its own block.
/// `None` where the seventeen do not reach it, which is exactly 37.7's
/// condition and not an error.
pub(crate) fn word_of_rotor(rotor: &super::Rotor) -> Option<super::Word> {
    let plane = (super::PLANE_ROW, super::PLANE_COL);
    word_of_block(&super::rotor_block_on(rotor, plane)?, plane)
}

/// THE HALF-ANGLE OF AN ARC, PROJECTIVELY — the rotor's own square root.
///
/// A rotor is identified up to a nonzero rational scale (1.5), so the root
/// of the arc `g` is wanted only up to one: any `r` with `r² = λ·g` for a
/// positive rational `λ`. Writing `g = ρ·e^{iθ}` with `ρ = |g|`, the number
/// `g + ρ = ρ(1 + e^{iθ})` has argument `θ/2` exactly, so `g + ρ` IS that
/// root — `(g + ρ)² = 2ρ·(g + ρ)·cos(θ/2)²`, a positive rational multiple
/// of `g`. It needs `ρ` to be an integer, which is the condition that the
/// rotation is rational at all, and nothing else.
///
/// The exact root was here instead, and it refused every arc that is not a
/// perfect square in ℤ[i]. A quarter turn's arc is `i`, whose exact root is
/// `(1+i)/√2` and does not exist there — so `rotor_of_block` returned
/// `None` for the commonest rotation in the corpus, and every caller that
/// needed a rotor from a block, the ring branch included, was refused with
/// it. `i + 1 = 1 + i` squares to `2i`, which is the same rotor.
///
/// The integer content is divided out to keep the coordinates small; it is
/// a square scale and changes no rotor.
pub(crate) fn arc_half(g: &Gauss) -> Option<Gauss> {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::{Signed, Zero};
    let n = g.norm();
    let m = n.sqrt();
    if &m * &m != n {
        // |g| is not an integer: this block is not a rational rotation and
        // no rotor over ℚ induces it (rule 11, refused rather than nearest).
        return None;
    }
    let (a, b) = (&g.a + &m, g.b.clone());
    if a.is_zero() && b.is_zero() {
        // g = −ρ, the half turn: `g + ρ` degenerates to zero because the
        // half-angle is a quarter turn, whose arc is `i` and whose root is
        // the blade alone. Written out rather than reached by the formula,
        // which has no argument to halve at θ = π.
        return Some(Gauss::new(BigInt::from(0), BigInt::from(1)));
    }
    let content = a.gcd(&b);
    match content.is_zero() {
        true => Some(Gauss::new(a, b)),
        false => Some(Gauss::new(&a / content.abs(), &b / content.abs())),
    }
}

/// The square root of a Gaussian integer when it has one in ℤ[i], by the
/// exact identity `√(a+bi) = ±(x + y·i)` with `x² = (a + |π|)/2`. Every
/// step is an exact integer square root or a refusal; nothing is rounded.
/// This is the root ON THE NOSE, wanted where an equality must hold in
/// ℤ[i] itself; a ROTOR wants `arc_half` instead, because a rotor is
/// projective and this refuses most rotations.
#[allow(dead_code)]
pub(crate) fn gauss_sqrt(g: &Gauss) -> Option<Gauss> {
    use num_bigint::BigInt;
    use num_integer::Integer;
    use num_traits::Zero;
    let n = g.norm();
    let m = n.sqrt();
    if &m * &m != n {
        return None;
    }
    let two = BigInt::from(2);
    let (xx, r1) = (&g.a + &m).div_rem(&two);
    if !r1.is_zero() {
        return None;
    }
    let x = xx.sqrt();
    if &x * &x != xx {
        return None;
    }
    if x.is_zero() {
        return None;
    }
    let (y, r2) = g.b.div_rem(&(&two * &x));
    if !r2.is_zero() {
        return None;
    }
    Some(Gauss::new(x, y))
}
