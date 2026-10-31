//! A CLOSED SPHERE'S POINTS:
//! the 3-sphere `S³` of `span(e₁, e₂, e₃, e₊)`, the vectors with `v² = +1`,
//! its pole `e₊` — the mirror of the hyperboloid sheet (`v² = −1` about `e₋`,
//! 54.7), whose motions are `Spin(4)`, the even versors fixing `e₋`.

use super::group::weight_of;
use super::multivector::{one, Mv, EP};
use num_traits::Zero;

/// THE POLE OF THE SPHERE — `e₊`, the point a frame's base point is the
/// image of.
pub fn sphere_pole() -> Mv {
    Mv::blade(EP, one())
}

/// THE POINT A FRAME STANDS AT ON THE SPHERE: its versor applied to the
/// pole, renormalised onto `v² = +1` — `V e₊ Ṽ` has `v² = (V Ṽ)²`, so one
/// exact rational division by the weight puts it on the sphere; the scale
/// is the point's own coordinate, as on the sheet (`frame_base_point`).
/// `None` where the frame has no weight or its image is not a unit vector.
pub fn sphere_base_point(v: &Mv) -> Option<Mv> {
    let w = weight_of(v)?;
    if w.is_zero() {
        return None;
    }
    let out = v.sandwich(&sphere_pole()).scale(&(one() / w));
    let s = out.geo(&out);
    (s.scalar_part() == one() && s.sub(&Mv::scalar(s.scalar_part())).is_zero()).then_some(out)
}
