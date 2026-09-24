// 55.3, 65.6, 68, A.12, 78.38 — THE TWO TOWERS OF A BODY ON A PLANE: its
// harmonics at orders two, three and four and its scale, read on the REGION
// (each lamina's unit square integrated) and on the SITES (each site the point
// it stands at); the chirality they compose; and the certificate that two
// bodies are dilates, naming which similarity of the lattice carries them.
//
//! 78.38 (C) — TWO SIMILARITIES OF THE LATTICE, ONE TOWER EACH. A body is
//! read two ways, one producer each (law 12: two readings of one body, not
//! two carriers of one reading):
//!
//! - the REGION tower (`moment2`, `quartic_on`, the lamina moments,
//!   `radius2`): each site's unit square integrated. A filled blow-up — each
//!   cell to a `k×k` block, ARC's scale tasks — is an EXACT similarity here,
//!   because a block of unit squares is one square of side `k`.
//! - the SITE tower ([`super::pose::centred`], the spread): each site the
//!   point it stands at. A site map `z ↦ λz` — the demand's Gaussian case,
//!   a body and its `(4 + i)` image — is an EXACT similarity here.
//!
//! Each tower has a floor the other lacks (the lamina's unit square, the
//! point tower's sub-site offsets), and a similarity is read on the tower
//! whose floor it scales. Both are always read and neither is chosen by a
//! score: a pair is dilates where EITHER tower certifies it, and the reading
//! names which.

use super::*;

/// WHICH TOWER A HARMONIC IS READ ON (78.38).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tower {
    /// Each lamina's unit square integrated: exact for filled blow-ups.
    Region,
    /// Each site the point it stands at: exact for site maps `z ↦ λz`.
    Sites,
}

/// WHICH SIMILARITY OF THE LATTICE CARRIES A PAIR (78.38 (C)), where one
/// does: of the region, of the site set, or of both.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dilation {
    Region,
    Sites,
    Both,
}

impl Dilation {
    /// The tower the pair's relation is read on: the one that certified it;
    /// the region's where both did, by the one fixed rule, never a score.
    pub fn tower(self) -> Tower {
        match self {
            Dilation::Sites => Tower::Sites,
            Dilation::Region | Dilation::Both => Tower::Region,
        }
    }
}

/// A.12 — THE ZEROTH MOMENT, the one place either tower is normalised: the
/// body's lamina count. `None` on an empty support (rule 11).
fn zeroth_moment(blade: &Blade) -> Option<BigRational> {
    match blade.support.len() {
        0 => None,
        n => Some(BigRational::from_integer(BigInt::from(n))),
    }
}

/// A harmonic that vanishes is no reading at that order (rule 11).
fn nonzero(h: [BigRational; 2]) -> Option<[BigRational; 2]> {
    match h[0].is_zero() && h[1].is_zero() {
        true => None,
        false => Some(h),
    }
}

/// THE SITE TOWER'S POINTS ON A PLANE: each site's two coordinates on `(i, j)`
/// about the sites' mean, with the count.
fn plane_points(blade: &Blade, plane: (usize, usize)) -> Option<(BigRational, Vec<(BigRational, BigRational)>)> {
    let (count, points) = super::pose::centred(blade, blade.n())?;
    let pts = points
        .iter()
        .map(|d| Some((d.get(plane.0)?.clone(), d.get(plane.1)?.clone())))
        .collect::<Option<Vec<_>>>()?;
    Some((count, pts))
}

/// A BODY'S SCALE ON ONE TOWER: the region's radius `radius2` (its unit squares
/// integrated), or the sites' spread `Σ(u² + v²)/M₀` on the plane.
pub fn scale2(blade: &Blade, plane: (usize, usize), tower: Tower) -> Option<BigRational> {
    match tower {
        Tower::Region => Some(blade.radius2()),
        Tower::Sites => {
            let (count, pts) = plane_points(blade, plane)?;
            Some(pts.iter().fold(BigRational::zero(), |acc, (u, v)| acc + u * u + v * v) / count)
        }
    }
}

/// THE HARMONIC OF AZIMUTHAL ORDER `m` (2, 3 or 4) ON ONE TOWER, normalised by
/// `M₀` so it carries degree `m` (A.12); turning at `m·φ`. `None` where it
/// vanishes, or for an order the towers do not carry.
///
/// - order 2, the DEVIATORIC `((Σu² − Σv²)/2, Σuv)/M₀` — the body's axis; the
///   lamina's isotropic `1/12` cancels in it, so the two towers agree here.
/// - order 3, `Σ(u + iv)³/M₀` — the odd moment that closes the half turn. On
///   the region each square's odd corrections cancel on the grid plane, and
///   a site off the grid is a point along that axis.
/// - order 4, `Σ(u + iv)⁴/M₀` — the orientation of a shape isotropic at order
///   two; on the region it carries each lamina's axis-aligned floor.
pub fn harmonic(blade: &Blade, plane: (usize, usize), tower: Tower, m: u8) -> Option<[BigRational; 2]> {
    let (two, three, four, six) = (
        BigRational::from_integer(BigInt::from(2)),
        BigRational::from_integer(BigInt::from(3)),
        BigRational::from_integer(BigInt::from(4)),
        BigRational::from_integer(BigInt::from(6)),
    );
    let (i, j) = plane;
    match (tower, m) {
        (Tower::Region, 2) => {
            let m0 = zeroth_moment(blade)?;
            let u1 = (blade.moment2(i, i)? - blade.moment2(j, j)?) / &two / &m0;
            let u2 = blade.moment2(i, j)? / &m0;
            nonzero([u1, u2])
        }
        (Tower::Region, 3) => {
            let centroid = blade.centroid()?;
            let (ci, cj) = (centroid.get(i)?, centroid.get(j)?);
            let m0 = zeroth_moment(blade)?;
            let (mut re, mut im) = (BigRational::zero(), BigRational::zero());
            for site in &blade.support {
                let a = BigRational::from_integer(BigInt::from(site.axis(i)?)) - ci;
                let b = BigRational::from_integer(BigInt::from(site.axis(j)?)) - cj;
                let [u1, u2, u3, _] = super::body::lamina_moments(&a, crate::transducer::base::is_grid_axis(i));
                let [v1, v2, v3, _] = super::body::lamina_moments(&b, crate::transducer::base::is_grid_axis(j));
                re += &u3 - &three * &u1 * &v2;
                im += &three * &u2 * &v1 - &v3;
            }
            nonzero([re / &m0, im / &m0])
        }
        (Tower::Region, 4) => {
            let m0 = zeroth_moment(blade)?;
            let [_, qx, qy] = blade.quartic_on(plane)?;
            nonzero([qx / &m0, qy / &m0])
        }
        (Tower::Sites, 2..=4) => {
            let (count, pts) = plane_points(blade, plane)?;
            let (mut a, mut b) = (BigRational::zero(), BigRational::zero());
            for (u, v) in &pts {
                let (u2, v2) = (u * u, v * v);
                match m {
                    2 => {
                        a += (&u2 - &v2) / &two;
                        b += u * v;
                    }
                    3 => {
                        a += u * (&u2 - &three * &v2);
                        b += v * (&three * &u2 - &v2);
                    }
                    _ => {
                        a += &u2 * &u2 - &six * &u2 * &v2 + &v2 * &v2;
                        b += &four * u * v * (&u2 - &v2);
                    }
                }
            }
            nonzero([a / &count, b / &count])
        }
        _ => None,
    }
}

/// THE DEVIATORIC VECTOR (55.3) on the region tower: `u = ((I_ii − I_jj)/2,
/// I_ij)/M₀`, turning at `2φ`; `None` where the second moment is isotropic
/// — a single lamina, a square, a cross — which [`quartic_harmonic`] orients.
pub fn deviatoric(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, 2)
}

/// THE FOURTH HARMONIC on the region tower, turning at `4φ`: the `D₄`
/// discriminator a square's second moment cannot give.
pub fn quartic_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, 4)
}

/// THE THIRD HARMONIC on the region tower (68), turning at `3φ`: odd where
/// `u` and `h₄` are even, so it tells `φ` from `φ + π`. `None` exactly on a
/// body with 2-fold symmetry, whose `ℤ₂` gauge freedom is real.
pub fn third_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, 3)
}

/// 78.38 — THE SITES' SPREAD on a plane, the site tower's scale.
pub fn spread2_on(blade: &Blade, plane: (usize, usize)) -> Option<BigRational> {
    scale2(blade, plane, Tower::Sites)
}

/// AT WHICH ORDER ONE TOWER REFUSES THE PAIR AS DILATES, or `None` where it
/// certifies them. A moment of order `m` carries degree `m`, so
///
/// ```text
///     ‖h_b^(m)‖² · (s_a²)^m  =  ‖h_a^(m)‖² · (s_b²)^m
/// ```
///
/// with `s²` that tower's scale, by exact cross multiplication at `m = 2, 3,
/// 4`; a harmonic vanishing on one and not the other is a difference of
/// shape at that order, on both is agreement. Each `‖h‖²` is the metric on a
/// vector (78.28(4)).
fn refused_at(a: &Blade, b: &Blade, plane: (usize, usize), tower: Tower) -> Option<u8> {
    let (Some(sa), Some(sb)) = (scale2(a, plane, tower), scale2(b, plane, tower)) else {
        return Some(2);
    };
    let norm2 = |v: &[BigRational; 2]| &v[0] * &v[0] + &v[1] * &v[1];
    for m in [2u8, 3, 4] {
        let agrees = match (harmonic(a, plane, tower, m), harmonic(b, plane, tower, m)) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(x), Some(y)) => norm2(&y) * sa.pow(i32::from(m)) == norm2(&x) * sb.pow(i32::from(m)),
        };
        if !agrees {
            return Some(m);
        }
    }
    None
}

/// A.12, 78.38 (C) — WHICH SIMILARITY CARRIES THE PAIR, or where each tower
/// refuses it. Both towers are always read: the pair is dilates where EITHER
/// certifies, and the answer names which; where both refuse, the order at
/// which each did, `(region, sites)`.
pub fn dilation(a: &Blade, b: &Blade, plane: (usize, usize)) -> Result<Dilation, (u8, u8)> {
    match (refused_at(a, b, plane, Tower::Region), refused_at(a, b, plane, Tower::Sites)) {
        (None, None) => Ok(Dilation::Both),
        (None, Some(_)) => Ok(Dilation::Region),
        (Some(_), None) => Ok(Dilation::Sites),
        (Some(r), Some(s)) => Err((r, s)),
    }
}

/// Whether some similarity of the lattice carries the pair.
pub fn dilation_certified(a: &Blade, b: &Blade, plane: (usize, usize)) -> bool {
    dilation(a, b, plane).is_ok()
}

/// THE CHIRALITY OF A BODY ON ONE TOWER — AN ODD INVARIANT OF ITS OWN HARMONIC
/// TOWER (V1 recut, 68): `Z = h₂² · conj(h₄)`, rotation-invariant and
/// conjugated by every reflection, scale-free as a ray (degree 8). `None`
/// where the body is isotropic at either order: achiral there.
pub fn chirality_on(b: &Blade, plane: (usize, usize), tower: Tower) -> Option<(BigRational, BigRational)> {
    let u = harmonic(b, plane, tower, 2)?;
    let q = harmonic(b, plane, tower, 4)?;
    // `u²` then `· conj(q)`, written out over ℚ.
    let (s1, s2) = (&u[0] * &u[0] - &u[1] * &u[1], &u[0] * &u[1] * BigRational::from_integer(BigInt::from(2)));
    Some((&s1 * &q[0] + &s2 * &q[1], &s2 * &q[0] - &s1 * &q[1]))
}

/// The chirality on the region tower.
pub fn chirality(b: &Blade, plane: (usize, usize)) -> Option<(BigRational, BigRational)> {
    chirality_on(b, plane, Tower::Region)
}
