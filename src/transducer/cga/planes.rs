//! D1, 68, 71.2(3) — the bivector planes of `Cl(n+1, 1)`, GENERATED from `n`
//! and split into the harmonic and luminal halves.

use super::multivector::{axes, Mv, EM, EP};
use num_traits::Zero;

/// THE HYPERBOLIC GENERATORS `eₐ ∧ e₋`, one for every spacelike generator:
/// each base axis and `e₊`. Each squares to `+1`:
///
/// ```text
///     (eₐe₋)² = eₐe₋eₐe₋ = −eₐeₐe₋e₋ = −(+1)(−1) = +1
/// ```
///
/// `n + 1` OF THEM, NOT `n`. `Spin(n+1,1)` is the isometry group of
/// hyperbolic `(n+1)`-space whose boundary at infinity is the base, and its
/// boosts are `eₐ ∧ e₋` for every spacelike `a` — `e₊` included, since
/// `e₊² = +1`. At the corpus's `n = 3` that is four, and a first cut that
/// declared three restricted dimensional capacity (Anti-Flattening clause
/// 5).
pub fn transvection_planes(n: u8) -> Vec<usize> {
    let mut out: Vec<usize> = axes(n).map(|a| a | EM).collect();
    out.push(EP | EM);
    out
}

/// IS THIS BLADE A HYPERBOLIC GENERATOR — two generators, one of them `e₋`?
/// Read off the blade's own bits, at every `n`.
pub fn is_transvection_plane(blade: usize) -> bool {
    blade.count_ones() == 2 && blade & EM != 0
}

/// THE HARMONIC HALF: the planes avoiding `e₋` — every pair of base axes,
/// then each base axis with `e₊`. `B² < 0`, compact, they turn.
pub fn harmonic_planes(n: u8) -> Vec<usize> {
    let held: Vec<usize> = axes(n).collect();
    let mut out = Vec::new();
    for (k, a) in held.iter().enumerate() {
        for b in &held[k + 1..] {
            out.push(a | b);
        }
    }
    for a in &held {
        out.push(a | EP);
    }
    out
}

/// THE BIVECTOR PLANES OF `Cl(n+1, 1)`, IN ONE PLACE AND SPLIT BY HALF
/// (D1, 68 — ruled 2026-09-22; generated from `n`, 71.2(3)).
///
/// `Λ²(ℚ^{n+1,1})` has `C(n+2, 2) = C(n+1, 2) + (n+1)` planes, read as two
/// halves that behave differently under `B²`:
///
/// ```text
///   HARMONIC   eᵢeⱼ, eᵢe₊            B² < 0, compact, turns
///   LUMINAL    eᵢe₋, e₊e₋            B² > 0, non-compact, boosts
/// ```
///
/// Harmonic first, then luminal, so an index into a tally is stable and its
/// half is read off the blade. At `n = 3` these are the ten the engine has
/// always read, in the order it has always read them.
///
/// THE NAMES ARE THE BLADE'S OWN BITS, so a reader gets the plane back
/// rather than an ordinal (rule 15).
pub fn bivector_planes(n: u8) -> Vec<usize> {
    let mut out = harmonic_planes(n);
    out.extend(transvection_planes(n));
    out
}

/// A generator's place in the algebra's own order — the base axes by index,
/// then `e₊`, then `e₋` — and its name: the axis index, `p`, or `m`.
fn generator(bit: usize) -> (usize, String) {
    match bit {
        0..=2 => (bit + 1, format!("{}", bit + 1)),
        3 => (usize::MAX - 1, "p".into()),
        4 => (usize::MAX, "m".into()),
        b => (b - 1, format!("{}", b - 1)),
    }
}

/// The plane's own name, read off its two generators in the algebra's
/// order, for a line a reader can act on: `e12`, `e1p`, `e3m`, `epm`. An
/// axis index of two digits is set apart, `e1_10`, so a name never reads as
/// another plane's.
pub fn plane_name(blade: usize) -> String {
    match blade.count_ones() {
        2 => blade_name(blade),
        _ => "?".into(),
    }
}

/// ANY BLADE'S OWN NAME, read off its generators in the algebra's order:
/// `1` for the scalar, `e12`, `e1p`, `e123pm`; an axis index of two digits
/// sets every name apart, `e1_10`, so a name never reads as another
/// blade's. The one producer of a blade's name, [`plane_name`] included.
pub fn blade_name(blade: usize) -> String {
    let mut gens: Vec<(usize, String)> =
        (0..usize::BITS as usize).filter(|b| blade >> b & 1 == 1).map(generator).collect();
    gens.sort_unstable();
    let names: Vec<String> = gens.into_iter().map(|(_, name)| name).collect();
    match (names.is_empty(), names.iter().any(|x| x.len() > 1)) {
        (true, _) => "1".into(),
        (false, true) => format!("e{}", names.join("_")),
        (false, false) => format!("e{}", names.concat()),
    }
}

/// A RAY'S OWN NAME, blade by blade with its exact coefficient: `e12 3/5,
/// e1p -1`. The one namer of a ray a reading carries ([`Mv::ray`]); a mask
/// this target's word cannot hold is named `?`, never truncated into another.
pub fn ray_name(ray: &[(u64, num_rational::BigRational)]) -> String {
    ray.iter()
        .map(|(b, q)| match usize::try_from(*b) {
            Ok(m) => format!("{} {q}", blade_name(m)),
            Err(_) => format!("? {q}"),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// THE TWO NULL DIRECTIONS OF THE CONFORMAL PAIR: `e_∞ = e₋ + e₊` and
/// `e_o = ½(e₋ − e₊)`. A bivector `eᵢ ∧ e_∞` is a translation's, `eᵢ ∧ e_o`
/// a transversion's; both square to zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pole {
    Infinity,
    Origin,
}

impl Pole {
    pub fn as_u8(self) -> u8 {
        match self {
            Pole::Infinity => 0,
            Pole::Origin => 1,
        }
    }

    pub fn from_u8(b: u8) -> Option<Pole> {
        match b {
            0 => Some(Pole::Infinity),
            1 => Some(Pole::Origin),
            _ => None,
        }
    }
}

/// A parabolic plane's name: its base axis and its pole, `e1∞`, `e2o`.
pub fn parabolic_name(axis: usize, pole: Pole) -> String {
    let p = match pole {
        Pole::Infinity => "∞",
        Pole::Origin => "o",
    };
    match axis > 9 {
        true => format!("e{axis}_{p}"),
        false => format!("e{axis}{p}"),
    }
}

/// WHICH PLANES A BIVECTOR OCCUPIES (PC1, D1, 68.1, 70.3), the one reader
/// the plane census and the orbit's precession both read (law 12).
///
/// A NULL BIVECTOR IS ONE PARABOLIC PLANE. On each base axis `eᵢ` the pair
/// `p eᵢe₊ + m eᵢe₋` squares to `m² − p²`. Where that is zero and the pair is
/// not, it is `eᵢ ∧ e_∞` (`p = m`, a translator's `−½ d e_∞`) or `eᵢ ∧ e_o`
/// (`p = −m`, a transversion's): ONE null plane, counted once, never as a
/// turn on `eᵢe₊` beside a boost on `eᵢe₋`. Every other pair, and every
/// other plane, is counted blade by blade as the algebra writes it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Occupancy {
    /// The planes, by blade, in [`bivector_planes`] order.
    pub planes: Vec<usize>,
    /// The parabolic planes, by base axis (counted from 1) and pole.
    pub parabolic: Vec<(usize, Pole)>,
}

/// The planes `b`'s bivector part occupies in the algebra of `n` base axes.
pub fn occupancy(b: &Mv, n: u8) -> Occupancy {
    let b = b.grade(2);
    let mut null_pairs: Vec<usize> = Vec::new();
    let mut parabolic: Vec<(usize, Pole)> = Vec::new();
    for (k, a) in axes(n).enumerate() {
        let (p, m) = (b.coeff(a | EP), b.coeff(a | EM));
        if p.is_zero() || p * p != m * m {
            continue;
        }
        // The stored blades past the third axis are `e₊eᵢ` and `e₋eᵢ`, both
        // the negatives of `eᵢe₊`, `eᵢe₋`; the ratio `p/m` is unchanged.
        parabolic.push((k + 1, if p == m { Pole::Infinity } else { Pole::Origin }));
        null_pairs.push(a | EP);
        null_pairs.push(a | EM);
    }
    let planes = bivector_planes(n)
        .into_iter()
        .filter(|p| !b.coeff(*p).is_zero() && !null_pairs.contains(p))
        .collect();
    Occupancy { planes, parabolic }
}

/// Is this plane compact — does its generator TURN rather than boost?
/// `B² < 0` for the planes that avoid `e₋`, and `e₋² = −1` is what makes the
/// others hyperbolic (54.7).
pub const fn plane_is_harmonic(blade: usize) -> bool {
    blade & EM == 0
}

