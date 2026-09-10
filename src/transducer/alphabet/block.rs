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
    let root = gauss_sqrt(&arc)?;
    let mask = (1usize << plane.0) | (1usize << plane.1);
    let q = |n: &num_bigint::BigInt| num_rational::BigRational::from_integer(n.clone());
    Some(Rotor::scalar(q(&root.a)).add(&Rotor::blade(mask, -q(&root.b))))
}

/// The square root of a Gaussian integer when it has one in ℤ[i], by the
/// exact identity `√(a+bi) = ±(x + y·i)` with `x² = (a + |π|)/2`. Every
/// step is an exact integer square root or a refusal; nothing is rounded.
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
