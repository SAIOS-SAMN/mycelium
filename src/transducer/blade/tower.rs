// 55.3, 65.6, 68, A.12 — THE MOMENT TOWER: a body's harmonics at orders
// two, three and four, normalised by its zeroth moment, the chirality they
// compose, and the certificate that two bodies are dilates.

use super::*;

/// THE DEVIATORIC VECTOR OF A BODY'S SECOND MOMENT — its ORIENTATION,
/// exact in ℚ and taken with no root (55.3, ruled 2026-09-22).
///
/// `inertia` holds the whole tensor; `r2` already takes its TRACE and uses
/// it where a radius belongs. The trace is an `SO(2)` invariant, so it
/// says how big a body is and nothing about which way it faces — a body
/// read through its trace alone is an ISOTROPIC POINT MASS, two of them
/// have no relative angle, and the only relation left between them is the
/// geodesic through their centres:
///
/// ```text
///     R = 1 − v_b v_a,  ⟨R⟩₂ = v_a ∧ v_b,  (v_a∧v_b)∧(v_a∧v_b) = 0
/// ```
///
/// a simple blade, `I₂ ≡ 0`, `ℓ = 1`. What orients the body is the
/// ANISOTROPIC remainder, `M₂ − ½tr(M₂)I`, which in `Cl(2,0)` is a vector:
///
/// ```text
///     u = ((I_xx − I_yy)/2) e₁ + I_xy e₂
/// ```
///
/// A rank-2 symmetric tensor turns at TWICE the body's angle, so `u` turns
/// at `2φ`. `None` where `u = 0`: a body whose inertia is isotropic — a
/// single lamina, a square, a cross — has no principal axis, and refusing
/// is the honest answer rather than an arbitrary axis (rule 11). Such a
/// body is oriented by [`quartic_harmonic`] instead, which turns at `4φ`.
/// A.12 — THE ZEROTH MOMENT, and the one place the tower is normalised.
///
/// `M₀ = ∫ 1 dA` is the body's own area, which on this base is its lamina
/// count. Every central moment divides by it, so a moment of harmonic order
/// `m` carries homogeneous degree `m` and nothing downstream needs a table
/// of per-carrier weights.
///
/// `None` on an empty support: a body with no laminae has no moments to
/// normalise, and dividing by zero is not a reading (rule 11).
fn zeroth_moment(blade: &Blade) -> Option<BigRational> {
    match blade.support.len() {
        0 => None,
        n => Some(BigRational::from_integer(BigInt::from(n))),
    }
}

///
/// ON A PLANE `(i, j)` GENERATED FROM `n` (BQ4): `u = ((I_ii − I_jj)/2,
/// I_ij)/M₀`, read in that plane's own coordinates. The caller names the
/// plane it reads; `None` also for a plane the body's base does not have.
pub fn deviatoric(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    let two = BigRational::from_integer(BigInt::from(2));
    let (i, j) = plane;
    // A.12 — NORMALISED BY `M₀`, so the tower speaks one dimension.
    //
    // MEASURED BEFORE THIS (65.6): `radius2()` is `Tr(I)/M₀`, a length²
    // scaling as `ρ²`, while this returned RAW `inertia` combinations,
    // which carry the area's own weight — `∫x² ~ ρ²·ρ² = ρ⁴` — so
    // `‖D‖²` scaled as `ρ⁸` where the harmonic order `m = 2` predicts
    // `ρ⁴`. One tower at two normalisations is law 12 inside a single
    // carrier, and it forced every consumer to hold a private table of
    // degrees: `{r² ↦ 2, D ↦ 4, Q ↦ 6}`, a lookup masquerading as
    // geometry.
    //
    // Normalised, a moment of azimuthal order `m` carries degree `m`, and
    // the scale certificate reads its exponent straight off the gauge
    // order with no table at all:
    //
    // ```text
    //     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
    // ```
    let m0 = zeroth_moment(blade)?;
    let u1 = (blade.moment2(i, i)? - blade.moment2(j, j)?) / &two / &m0;
    let u2 = blade.moment2(i, j)? / &m0;
    match u1.is_zero() && u2.is_zero() {
        true => None,
        false => Some([u1, u2]),
    }
}

/// THE FOURTH HARMONIC OF A BODY — the orientation of a shape whose
/// inertia is isotropic, turning at FOUR times the body's angle.
///
/// `quartic` already holds `(∫(u⁴−6u²v²+v⁴), ∫(4u³v−4uv³))` beside the
/// isotropic part. A square's second moment cannot tell it from a disc;
/// its fourth moment can, and on the square base that is exactly the `D₄`
/// discriminator. `None` where this vanishes too, which is a body with no
/// orientation at either order. On the plane the caller names (BQ4).
pub fn quartic_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    // A.12 — normalised by `M₀`, as the deviatoric is. Raw, this carried
    // `∫x⁴ ~ ρ⁶` and `‖Q‖²` scaled as `ρ¹²`; normalised it is degree 4,
    // the harmonic order it is named for.
    let m0 = zeroth_moment(blade)?;
    let [_, qx, qy] = blade.quartic_on(plane)?;
    let (h4x, h4y) = (qx / &m0, qy / &m0);
    match h4x.is_zero() && h4y.is_zero() {
        true => None,
        false => Some([h4x, h4y]),
    }
}

/// A.12 — IS THIS PAIR RELATED BY A DILATION, OR BY A SHEAR?
///
/// A trace ratio cannot tell: a 2×2 square and a 1×4 bar of equal `Tr(I)`
/// have the same `r²` and are not dilates of one another. The whole
/// normalised tower must correspond, and with the tower at one
/// normalisation a moment of azimuthal order `m` carries degree `m`, so
/// the certificate reads its exponent STRAIGHT OFF THE GAUGE ORDER:
///
/// ```text
///     ‖M̃_b^(m)‖² · (r_a²)^m  =  ‖M̃_a^(m)‖² · (r_b²)^m
/// ```
///
/// checked at `m = 2` (deviatoric) and `m = 4` (quartic), by exact cross
/// multiplication in ℚ with no root taken and no table of weights. Each
/// `‖M̃^(m)‖²` is THE METRIC ON A VECTOR (78.28(4), VA2): a harmonic moment
/// is a 2-vector and its square is its own content.
///
/// A VANISHING HARMONIC IS PART OF THE READING, not a gap in it. A `C₄`
/// body has no deviatoric at all, so a pair where one vanishes and the
/// other does not is REFUSED — they cannot be dilates, since a dilation
/// preserves every harmonic's vanishing. Both vanishing is agreement.
///
/// ON THE PLANE ITS CALLER READS (BQ4). The norms of a plane's harmonics are
/// invariant under turns IN that plane and not under turns out of it, so a
/// certificate taken over every plane would refuse a pair a turn in one
/// plane relates; the certificate belongs to the plane the relation is read
/// on, as the turn does.
pub fn dilation_certified(a: &Blade, b: &Blade, plane: (usize, usize)) -> bool {
    let (ra2, rb2) = (a.radius2(), b.radius2());
    let norm2 = |v: [BigRational; 2]| &v[0] * &v[0] + &v[1] * &v[1];
    let zero = BigRational::from_integer(BigInt::from(0));
    let order = |ma: Option<[BigRational; 2]>, mb: Option<[BigRational; 2]>, m: usize| {
        match (ma, mb) {
            // Both isotropic at this order: they agree, and nothing is
            // asserted about a harmonic neither body carries.
            (None, None) => true,
            // One carries the harmonic and the other does not. A dilation
            // preserves vanishing, so this is a shape difference (rule 11:
            // the refusal is the reading).
            (None, Some(_)) | (Some(_), None) => false,
            (Some(x), Some(y)) => {
                norm2(y) * ra2.pow(m as i32) == norm2(x) * rb2.pow(m as i32)
            }
        }
    };
    let _ = zero;
    order(deviatoric(a, plane), deviatoric(b, plane), 2)
        && order(quartic_harmonic(a, plane), quartic_harmonic(b, plane), 4)
}

/// THE CHIRALITY OF A BODY — AN ODD INVARIANT OF ITS OWN HARMONIC TOWER
/// (V1 recut, 68 — ruled 2026-09-22).
///
/// THE FIRST CUT OF V1 ANSWERED THIS WITH A LATTICE, AND THAT WAS THE
/// DEFECT. It carried a table of the four improper elements of `D₄`,
/// applied each to the CELLS about the centroid, and compared coordinate
/// lists. Three separate flattenings in one helper: a hardcoded generator
/// set, raw coordinate scraping in place of geometry, and a permanence
/// argument resting on the crystallographic restriction of `ℤ²` — none of
/// which mean anything in `Cl(4,1)`, where the base is conformal and a
/// body is its moment tower, not a set of squares.
///
/// THE INTRINSIC READING. Under a rotation by `φ` the harmonic of
/// azimuthal order `m` carries `h_m ↦ e^{imφ}·h_m`; under a reflection in
/// the axis at `α` it carries `h_m ↦ e^{imα}·conj(h_m)`. So the
/// combination
///
/// ```text
///     Z = h₂² · conj(h₄)
/// ```
///
/// is ROTATION-INVARIANT — `e^{4iφ}·e^{−4iφ} = 1` — and CONJUGATED by
/// every reflection, whatever axis it is taken about. That is chirality
/// itself, read off the body, with no group table to consult, no cells to
/// compare, and no lattice to restrict.
///
/// IT IS ODD WHERE THE TOWER IS EVEN, which is why it can do what neither
/// `u` nor `h₄` can alone. Both are even-rank and satisfy `M(−x) = M(x)`,
/// so each is blind to a reflection taken on its own axis; their PRODUCT
/// at mismatched orders is not, because the two phases cancel against
/// each other rather than against themselves.
///
/// SCALE-FREE AS A RAY, not as a value. Normalised (A.12), `h₂` has
/// degree 2 and `h₄` degree 4, so `Z` has degree 8 and a dilation by `ρ`
/// sends `Z ↦ ρ⁸·Z` — a POSITIVE real factor, which is exactly what
/// [`same_ray`] divides out. Nothing is normalised away and no root is
/// taken.
///
/// `None` where the body is isotropic at either order — a square, a disc,
/// a cross. That is not a gap: such a body IS achiral at these orders,
/// its mirror image is one of its own turns, and a chirality read off it
/// would be invented (rule 11).
pub fn chirality(b: &Blade, plane: (usize, usize)) -> Option<(BigRational, BigRational)> {
    let u = deviatoric(b, plane)?;
    let q = quartic_harmonic(b, plane)?;
    // `u²` then `· conj(q)`, written out over ℚ.
    let (s1, s2) = (&u[0] * &u[0] - &u[1] * &u[1], &u[0] * &u[1] * BigRational::from_integer(BigInt::from(2)));
    Some((&s1 * &q[0] + &s2 * &q[1], &s2 * &q[0] - &s1 * &q[1]))
}

/// THE THIRD HARMONIC OF A BODY — THE ODD MOMENT THAT CLOSES THE HALF
/// TURN (68, ruled 2026-09-22).
///
/// `h₃ = Σ(x + iy)³ / M₀`, turning at THREE times the body's angle. It is
/// the first ODD-order harmonic the tower carries, and odd is the whole
/// point: `u` and `h₄` are even-rank, satisfy `M(−x) = M(x)`, and
/// therefore CANNOT tell `φ` from `φ + π` — which is why the half turn was
/// left open and answered with a cell test instead.
///
/// EXACT ON LAMINAE, WITH NO CORRECTION TERM. A body is unit squares, and
/// a square centred at `(a,b)` has `∫x³ = a³ + a/4` and
/// `∫xy² = a(b² + 1/12)`, so
///
/// ```text
///     ∫(x³ − 3xy²) = Σ(a³ + a/4 − 3ab² − a/4) = Σ(a³ − 3ab²)
///     ∫(3x²y − y³) = Σ(3a²b + b/4 − b³ − b/4) = Σ(3a²b − b³)
/// ```
///
/// The square's own odd moments cancel identically, so the point-mass
/// formula IS the integral — not an approximation of it.
///
/// `None` where it vanishes, and that is a reading rather than a gap: a
/// body with 2-fold symmetry has `h₃ = 0` exactly, because the half turn
/// maps it to itself and sends `h₃ ↦ −h₃`. Such a body's two candidate
/// turns act IDENTICALLY on it, so there is nothing to choose — and the
/// `ℤ₂` gauge freedom `HalfOpen` carries is real, not a missing
/// measurement.
///
/// Normalised by `M₀` like the rest of the tower (A.12), so it carries
/// degree 3 and the ray comparison divides any dilation out.
///
/// ON THE PLANE THE CALLER NAMES (BQ4), from each lamina's own moments
/// along the plane's two axes ([`super::body::lamina_moments`], the lift's
/// one producer): `∫(u³ − 3uv²)` and `∫(3u²v − v³)`. On the grid plane the
/// square's corrections cancel as above; on a plane with an axis off the
/// grid the lamina is a point along it and the same product formula holds.
pub fn third_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    let (i, j) = plane;
    let centroid = blade.centroid()?;
    let (ci, cj) = (centroid.get(i)?, centroid.get(j)?);
    let m0 = zeroth_moment(blade)?;
    let three = BigRational::from_integer(BigInt::from(3));
    let (mut re, mut im) = (
        BigRational::from_integer(BigInt::from(0)),
        BigRational::from_integer(BigInt::from(0)),
    );
    for site in &blade.support {
        let a = BigRational::from_integer(BigInt::from(site.axis(i)?)) - ci;
        let b = BigRational::from_integer(BigInt::from(site.axis(j)?)) - cj;
        let [u1, u2, u3, _] = super::body::lamina_moments(&a, crate::transducer::base::is_grid_axis(i));
        let [v1, v2, v3, _] = super::body::lamina_moments(&b, crate::transducer::base::is_grid_axis(j));
        re += &u3 - &three * &u1 * &v2;
        im += &three * &u2 * &v1 - &v3;
    }
    let (re, im) = (re / &m0, im / &m0);
    match re.is_zero() && im.is_zero() {
        true => None,
        false => Some([re, im]),
    }
}
