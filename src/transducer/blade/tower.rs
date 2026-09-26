// 55.3, 65.6, 68, A.12, 78.38 — THE TWO TOWERS OF A BODY ON A PLANE: its
// harmonics at orders two, three and four and its extent, read on the REGION
// (each compartment lifted to its round) and on the SITES (each site the point it
// stands at); the chirality they compose; and the certificate that two bodies
// are dilates, naming which tower carries them.
//
//! 78.38 (C) — TWO READINGS OF ONE BODY, one producer each (law 12):
//!
//! - the REGION tower (`moment2`, `quartic_on`, [`super::body::round_harmonic`],
//!   `radius2`): each compartment lifted at the sensor to its ROUND, its centre
//!   with the unit cube's extent (ruled 2026-09-26). A round is isotropic and
//!   holds nothing above order two, so on the grid plane the region's
//!   harmonics are its centres', and its extent carries the rounds' own
//!   `2/12` (A.12). No square enters: a square's own higher moments were the
//!   lattice's rigidity, and a filled `k×k` blow-up, which only a square
//!   carries onto itself, now reads its block's own structure at order four.
//! - the SITE tower ([`super::pose::centred`], the spread): each site the
//!   point it stands at. A site map `z ↦ λz` — the demand's Gaussian case, a
//!   body and its `(4 + i)` image — is an EXACT similarity here.
//!
//! The rounds' extent is the floor the site tower lacks, and a similarity is
//! read on the tower whose extent it scales. Both are always read and neither
//! is chosen by a score: a pair is dilates where EITHER tower certifies it,
//! and the reading names which.

use super::*;

/// WHICH TOWER A HARMONIC IS READ ON (78.38).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tower {
    /// Each compartment lifted to its round: its centre with its extent.
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

/// THE HARMONIC ORDERS THE TOWERS CARRY (A.12, 68): the deviatoric, the odd
/// moment and the quartic, turning at `2φ`, `3φ` and `4φ`. An order is one of
/// these by type, so no reader folds a number into range; ordered as a
/// FILTRATION, coarse to fine, so "exact through a higher order" is the finer
/// fit and never a score.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Harmonic {
    Two,
    Three,
    Four,
}

impl Harmonic {
    /// The orders in the order the certificate reads them.
    pub const ALL: [Harmonic; 3] = [Harmonic::Two, Harmonic::Three, Harmonic::Four];

    /// `m`: the multiple of `φ` the harmonic turns at, and its degree under a
    /// dilation.
    pub fn m(self) -> i32 {
        match self {
            Harmonic::Two => 2,
            Harmonic::Three => 3,
            Harmonic::Four => 4,
        }
    }
}

/// WHERE ONE TOWER REFUSES A PAIR AS DILATES (78.38 (C), R1): at the harmonic
/// order the two shapes part, or — before any order is read — because a body
/// has no points on the plane, so the tower carries no extent for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TowerRefusal {
    At(Harmonic),
    Unplaced,
}

impl TowerRefusal {
    /// Every refusal a tower can give, for a tally that names each.
    pub const ALL: [TowerRefusal; 4] = [
        TowerRefusal::At(Harmonic::Two),
        TowerRefusal::At(Harmonic::Three),
        TowerRefusal::At(Harmonic::Four),
        TowerRefusal::Unplaced,
    ];
}

/// A.12 — THE ZEROTH MOMENT, the one place either tower is normalised: the
/// body's compartment count. `None` on an empty support (rule 11).
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

/// THE POINT TOWER ON `n` AXES (78.38): each compartment the point it stands
/// at, about the points' mean, normalised by the count — the second moment
/// `M`, the odd moment `t = ⟨d |d|²⟩` and the fourth order's symmetric part
/// `N = ⟨d dᵀ |d|²⟩`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PointTower {
    pub m: Vec<Vec<BigRational>>,
    pub t: Vec<BigRational>,
    pub n4: Vec<Vec<BigRational>>,
}

/// A BODY'S TWO TOWERS, READ ONCE, AT THE LIFT (A.11: one producer, no
/// consumer recomputes). They are the body's coordinates like its inertia
/// and its quartic, derived from the same compartments in the same pass,
/// never a cache: on every plane of the base, the harmonics of orders two,
/// three and four on the region (the rounds) and on the sites (the points),
/// the sites' spread, and the point tower on `n` axes. Every reader — the
/// certificate, the chirality, the orientation, the transition, the tube's
/// pose — reads these, so a body is read once however many pairs it stands
/// in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Towers {
    /// Per plane, in [`crate::transducer::base::planes`] order: orders 2, 3, 4.
    pub region: Vec<[Option<[BigRational; 2]>; 3]>,
    /// Per plane: orders 2, 3, 4 on the points.
    pub sites: Vec<[Option<[BigRational; 2]>; 3]>,
    /// Per plane: the points' spread `Σ(u² + v²)/M₀`.
    pub spread: Vec<Option<BigRational>>,
    /// The point tower on the body's `n` axes; `None` for no support.
    pub points: Option<PointTower>,
}

/// THE ONE READING OF BOTH TOWERS, at the lift: every plane's harmonics and
/// spread, and the point tower, from the compartments the lift has just
/// read. Read by nothing but `lift`.
pub(super) fn read_towers(blade: &Blade) -> Towers {
    let planes = crate::transducer::base::planes(blade.n());
    let orders = |tower: Tower| -> Vec<[Option<[BigRational; 2]>; 3]> {
        planes.iter().map(|&p| Harmonic::ALL.map(|m| harmonic_read(blade, p, tower, m))).collect()
    };
    Towers {
        region: orders(Tower::Region),
        sites: orders(Tower::Sites),
        spread: planes.iter().map(|&p| extent_read(blade, p, Tower::Sites)).collect(),
        points: super::pose::point_tower(blade, blade.n()),
    }
}

/// An order's place in the towers' rows.
fn order_index(m: Harmonic) -> usize {
    match m {
        Harmonic::Two => 0,
        Harmonic::Three => 1,
        Harmonic::Four => 2,
    }
}

/// A BODY'S EXTENT ON ONE TOWER (A.12's `r²`, the inversive extent), read
/// off the body's towers: the region's `radius2` — its rounds' centres and
/// their own extent — or the sites' spread on the plane. `None` where the
/// body has no points on the plane — the plane off its base, or no support —
/// on either tower alike.
pub fn extent(blade: &Blade, plane: (usize, usize), tower: Tower) -> Option<BigRational> {
    let spread = blade.towers.spread.get(crate::transducer::base::plane_index(plane)?)?.clone()?;
    match tower {
        Tower::Region => Some(blade.radius2()),
        Tower::Sites => Some(spread),
    }
}

/// THE HARMONIC OF AZIMUTHAL ORDER `m` ON ONE TOWER, read off the body's
/// towers ([`read_towers`]); `None` where it vanishes, or on a plane the
/// body's base does not have.
pub fn harmonic(blade: &Blade, plane: (usize, usize), tower: Tower, m: Harmonic) -> Option<[BigRational; 2]> {
    let ix = crate::transducer::base::plane_index(plane)?;
    let row = match tower {
        Tower::Region => blade.towers.region.get(ix)?,
        Tower::Sites => blade.towers.sites.get(ix)?,
    };
    row[order_index(m)].clone()
}

/// The producer of a body's extent on one tower, from its compartments.
fn extent_read(blade: &Blade, plane: (usize, usize), tower: Tower) -> Option<BigRational> {
    match tower {
        Tower::Region => plane_points(blade, plane).map(|_| blade.radius2()),
        Tower::Sites => {
            let (count, pts) = plane_points(blade, plane)?;
            Some(pts.iter().fold(BigRational::zero(), |acc, (u, v)| acc + u * u + v * v) / count)
        }
    }
}

/// THE PRODUCER OF THE HARMONIC OF AZIMUTHAL ORDER `m` ON ONE TOWER, from the
/// compartments, normalised by `M₀` so it carries degree `m` (A.12); turning
/// at `m·φ`. `None` where it vanishes. Read once per body, by the lift.
///
/// - order 2, the DEVIATORIC `((Σu² − Σv²)/2, Σuv)/M₀` — the body's axis; the
///   round's isotropic extent cancels in it on the grid plane, so the two
///   towers agree there.
/// - order 3, `Σ(u + iv)³/M₀` — the odd moment that closes the 180° turn; on
///   the region each round's own `(z + w)³` ([`super::body::round_harmonic`]).
/// - order 4, `Σ(u + iv)⁴/M₀` — the orientation of a shape isotropic at order
///   two; on the region each round's, held by the lift (`quartic_on`).
fn harmonic_read(blade: &Blade, plane: (usize, usize), tower: Tower, m: Harmonic) -> Option<[BigRational; 2]> {
    let (two, three, four, six) = (
        BigRational::from_integer(BigInt::from(2)),
        BigRational::from_integer(BigInt::from(3)),
        BigRational::from_integer(BigInt::from(4)),
        BigRational::from_integer(BigInt::from(6)),
    );
    let (i, j) = plane;
    match (tower, m) {
        (Tower::Region, Harmonic::Two) => {
            let m0 = zeroth_moment(blade)?;
            let u1 = (blade.moment2(i, i)? - blade.moment2(j, j)?) / &two / &m0;
            let u2 = blade.moment2(i, j)? / &m0;
            nonzero([u1, u2])
        }
        (Tower::Region, Harmonic::Three) => {
            let centroid = blade.centroid()?;
            let (ci, cj) = (centroid.get(i)?, centroid.get(j)?);
            let m0 = zeroth_moment(blade)?;
            let (mut re, mut im) = (BigRational::zero(), BigRational::zero());
            for site in &blade.support {
                let a = BigRational::from_integer(BigInt::from(site.axis(i)?)) - ci;
                let b = BigRational::from_integer(BigInt::from(site.axis(j)?)) - cj;
                let [h0, h1] = super::body::round_harmonic(&a, &b, plane, Harmonic::Three);
                re += h0;
                im += h1;
            }
            nonzero([re / &m0, im / &m0])
        }
        (Tower::Region, Harmonic::Four) => {
            let m0 = zeroth_moment(blade)?;
            let [_, qx, qy] = blade.quartic_on(plane)?;
            nonzero([qx / &m0, qy / &m0])
        }
        (Tower::Sites, m) => {
            let (count, pts) = plane_points(blade, plane)?;
            let (mut a, mut b) = (BigRational::zero(), BigRational::zero());
            for (u, v) in &pts {
                let (u2, v2) = (u * u, v * v);
                match m {
                    Harmonic::Two => {
                        a += (&u2 - &v2) / &two;
                        b += u * v;
                    }
                    Harmonic::Three => {
                        a += u * (&u2 - &three * &v2);
                        b += v * (&three * &u2 - &v2);
                    }
                    Harmonic::Four => {
                        a += &u2 * &u2 - &six * &u2 * &v2 + &v2 * &v2;
                        b += &four * u * v * (&u2 - &v2);
                    }
                }
            }
            nonzero([a / &count, b / &count])
        }
    }
}

/// THE DEVIATORIC VECTOR (55.3) on the region tower: `u = ((I_ii − I_jj)/2,
/// I_ij)/M₀`, turning at `2φ`; `None` where the second moment is isotropic
/// — a single compartment, a square, a cross — which [`quartic_harmonic`] orients.
pub fn deviatoric(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, Harmonic::Two)
}

/// THE FOURTH HARMONIC on the region tower, turning at `4φ`: the `D₄`
/// discriminator a square's second moment cannot give.
pub fn quartic_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, Harmonic::Four)
}

/// THE THIRD HARMONIC on the region tower (68), turning at `3φ`: odd where
/// `u` and `h₄` are even, so it tells `φ` from `φ + π`. `None` exactly on a
/// body with 2-fold symmetry, whose `ℤ₂` gauge freedom is real.
pub fn third_harmonic(blade: &Blade, plane: (usize, usize)) -> Option<[BigRational; 2]> {
    harmonic(blade, plane, Tower::Region, Harmonic::Three)
}

/// 78.38 — THE SITES' SPREAD on a plane, the site tower's extent.
pub fn spread_on(blade: &Blade, plane: (usize, usize)) -> Option<BigRational> {
    extent(blade, plane, Tower::Sites)
}

/// AT WHICH ORDER ONE TOWER REFUSES THE PAIR AS DILATES, or `None` where it
/// certifies them. A moment of order `m` carries degree `m`, so
///
/// ```text
///     ‖h_b^(m)‖² · (r_a²)^m  =  ‖h_a^(m)‖² · (r_b²)^m
/// ```
///
/// with `r²` that tower's extent, by exact cross multiplication at `m = 2,
/// 3, 4`; a harmonic vanishing on one and not the other is a difference of
/// shape at that order, on both is agreement. Each `‖h‖²` is the metric on a
/// vector (78.28(4)). A body with no points on the plane carries no extent on
/// either tower and is refused by that name, before any order is read.
fn refused_at(a: &Blade, b: &Blade, plane: (usize, usize), tower: Tower) -> Option<TowerRefusal> {
    let (Some(sa), Some(sb)) = (extent(a, plane, tower), extent(b, plane, tower)) else {
        return Some(TowerRefusal::Unplaced);
    };
    let norm2 = |v: &[BigRational; 2]| &v[0] * &v[0] + &v[1] * &v[1];
    Harmonic::ALL.into_iter().find_map(|m| {
        let agrees = match (harmonic(a, plane, tower, m), harmonic(b, plane, tower, m)) {
            (None, None) => true,
            (None, Some(_)) | (Some(_), None) => false,
            (Some(x), Some(y)) => norm2(&y) * sa.pow(m.m()) == norm2(&x) * sb.pow(m.m()),
        };
        (!agrees).then_some(TowerRefusal::At(m))
    })
}

/// THE ORDER THROUGH WHICH ONE TOWER CARRIES THE PAIR (ruled 2026-09-26, the
/// cost of a fit): the pair fits through `m` and its residue begins at `m +
/// 1`. Order four is a fit with no residue on the tower. A pair that parts at
/// the deviatoric has no orientation in common and fits through nothing, and
/// a body with no points on the plane is unplaced: both refused by name.
pub fn fits_through(a: &Blade, b: &Blade, plane: (usize, usize), tower: Tower) -> Result<Harmonic, TowerRefusal> {
    match refused_at(a, b, plane, tower) {
        None => Ok(Harmonic::Four),
        Some(TowerRefusal::At(Harmonic::Four)) => Ok(Harmonic::Three),
        Some(TowerRefusal::At(Harmonic::Three)) => Ok(Harmonic::Two),
        Some(refusal @ (TowerRefusal::At(Harmonic::Two) | TowerRefusal::Unplaced)) => Err(refusal),
    }
}

/// A PAIR'S FIT: the tower it is read on and the order through which that
/// tower carries it — the residue, if any, begins one order above.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fit {
    pub dilation: Dilation,
    pub through: Harmonic,
}

/// A.12, 78.38 (C) — THE PAIR'S FIT, or where each tower refuses it. Both
/// towers are always read. The pair is carried where EITHER fits it through
/// the deviatoric, on the tower that fits it through the finer order (a
/// filtration, never a score) — both where they fit through the same order;
/// where both refuse, each tower's refusal, `(region, sites)`.
pub fn certify(a: &Blade, b: &Blade, plane: (usize, usize)) -> Result<Fit, (TowerRefusal, TowerRefusal)> {
    let fit = |dilation, through| Ok(Fit { dilation, through });
    match (fits_through(a, b, plane, Tower::Region), fits_through(a, b, plane, Tower::Sites)) {
        (Ok(r), Ok(s)) if r == s => fit(Dilation::Both, r),
        (Ok(r), Ok(s)) if r > s => fit(Dilation::Region, r),
        (Ok(_), Ok(s)) => fit(Dilation::Sites, s),
        (Ok(r), Err(_)) => fit(Dilation::Region, r),
        (Err(_), Ok(s)) => fit(Dilation::Sites, s),
        (Err(r), Err(s)) => Err((r, s)),
    }
}

/// WHICH TOWER CARRIES THE PAIR, the certificate's first half.
pub fn dilation(a: &Blade, b: &Blade, plane: (usize, usize)) -> Result<Dilation, (TowerRefusal, TowerRefusal)> {
    certify(a, b, plane).map(|f| f.dilation)
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
    let u = harmonic(b, plane, tower, Harmonic::Two)?;
    let q = harmonic(b, plane, tower, Harmonic::Four)?;
    // `u²` then `· conj(q)`, written out over ℚ.
    let (s1, s2) = (&u[0] * &u[0] - &u[1] * &u[1], &u[0] * &u[1] * BigRational::from_integer(BigInt::from(2)));
    Some((&s1 * &q[0] + &s2 * &q[1], &s2 * &q[0] - &s1 * &q[1]))
}

/// The chirality on the region tower.
pub fn chirality(b: &Blade, plane: (usize, usize)) -> Option<(BigRational, BigRational)> {
    chirality_on(b, plane, Tower::Region)
}
