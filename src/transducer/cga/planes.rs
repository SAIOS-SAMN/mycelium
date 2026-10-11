//! D1, 68, 71.2(3) — the bivector planes of `Cl(n+1, 1)`, GENERATED from `n`
//! and split into the harmonic and luminal halves, and the census of which
//! ones the engine populates.

use super::multivector::{axes, base_axis, Mv, EM, EP};
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

/// D1 — WHICH PLANES THE ENGINE ACTUALLY POPULATES (68).
///
/// One counter per bivector plane of the algebra the transitions live in,
/// so a recut can be read for DIMENSIONAL CAPACITY rather than for
/// throughput. A corpus that turns only on `e₁e₂` and boosts only on
/// `e₁e₋` is using two of its planes, and no aggregate count says so — it
/// reports a busy engine occupying a line.
///
/// This is the Anti-Flattening Law's clause 5 made observable: declaring
/// capacity and never populating it is the same defect as not declaring
/// it, and until now nothing could tell the two apart.
///
/// THE CENSUS IS AS WIDE AS THE ALGEBRA IT READS (71.2(3)): its `n` is the
/// largest of the transitions noted, and its planes are generated from it.
#[derive(Default, Clone, Debug)]
pub struct PlaneTally {
    /// The algebra the census is read in.
    pub n: u8,
    /// Non-zero occupancy per plane, keyed by the plane's own blade.
    pub planes: crate::network::nerve::Keyed<u64>,
    /// Occupancy per PARABOLIC plane (PC1), keyed by its base axis and
    /// pole ([`parabolic_key`]): a null bivector counted once.
    pub parabolic: crate::network::nerve::Keyed<u64>,
    /// Generators read, and those carrying content in BOTH halves — a
    /// screw, which is where `I₂ ≠ 0` can live (59.7).
    pub read: u64,
    pub coupled: u64,
}

thread_local! {
    static PLANES: std::cell::RefCell<PlaneTally> = std::cell::RefCell::new(PlaneTally::default());
}

/// A plane's key in the tally: its blade, written out.
fn plane_key(blade: usize) -> [u8; 8] {
    (blade as u64).to_be_bytes()
}

/// A parabolic plane's key: its base axis, written out, then its pole.
fn parabolic_key(axis: usize, pole: Pole) -> [u8; 9] {
    let mut k = [0u8; 9];
    k[..8].copy_from_slice(&(axis as u64).to_be_bytes());
    k[8] = pole.as_u8();
    k
}

/// THE CONFORMAL GENERATOR A SLOT OF THE LETTERS' CHART IS READ AS (51.2):
/// the base axis the slot stands for ([`crate::transducer::alphabet::axis_of_slot`]),
/// and `e₊` for the off-plane slot, which stands for no base axis. `None`
/// for a slot's axis past the algebra's masks.
fn carriage_generator(slot: u8) -> Option<usize> {
    match crate::transducer::alphabet::axis_of_slot(slot) {
        Some(axis) => base_axis(axis + 1),
        None => Some(EP),
    }
}

/// THE ROTOR'S SIX PLANES, WHICH THE CARRIAGE CANNOT CARRY.
///
/// A transition is `D(ρ)·T(t)·R` and the two halves live in DIFFERENT
/// ALGEBRAS (L11e): the carriage in the conformal algebra, the rotor over
/// the six planes of the letters' `Cl(4,0)` chart (law 6: the seventeen are
/// written on it and do not change with `n`). A census of the carriage
/// alone therefore reads ZERO on the entire harmonic half, structurally.
///
/// 51.2 fixes the rotor chart's correspondence onto the carriage's planes,
/// and it is READ from the alphabet's own slots ([`carriage_generator`]),
/// never tabled beside them: `(0,1)`, `(0,3)`, `(1,3)` are `e₁e₂`, `e₁e₃`,
/// `e₂e₃`, and the chart's three off-plane channels, through `e2`, land on
/// `e₁e₊`, `e₂e₊`, `e₃e₊` — which is where orders outside `{1,2,4}` live
/// (65.6). A hand table stood here and filed `(0,3)` as `e₂e₃`, `(1,3)` as
/// `e₂e₊` and `(0,2)` as `e₁e₃` (law 12: two producers of one
/// correspondence, and they disagreed).
///
/// ONE TRANSITION, BOTH HALVES, ONE COUNT, so `read` increments once and
/// `coupled` can see both halves at once.
pub fn note_transition(r: &crate::transducer::alphabet::Rotor, carriage: &Mv) {
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    let mut harmonic = false;
    for &(s, t) in &crate::transducer::alphabet::SLOT_PLANES_SIX {
        let rotor_mask = (1usize << s) | (1usize << t);
        if !r.coeff(rotor_mask).is_zero() {
            if let (Some(a), Some(b)) = (carriage_generator(s), carriage_generator(t)) {
                seen.insert(plane_key(a | b), ());
            }
            harmonic = true;
        }
    }
    // The carriage contributes the luminal planes, the `eᵢ∧e₊` the rotor
    // chart does not reach, and — PC1 — its null bivectors, each ONE
    // parabolic plane: all read by `occupancy`, at the carriage's own `n`. A
    // translator's `eᵢ ∧ e_∞` was counted as a turn on `eᵢe₊` and a boost on
    // `eᵢe₋`, and epoch 97's census read every translation twice.
    let occupied = occupancy(carriage, carriage.n());
    let mut luminal = false;
    for &plane in &occupied.planes {
        seen.insert(plane_key(plane), ());
        if !plane_is_harmonic(plane) {
            luminal = true;
        }
    }
    let mut par: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for &(axis, pole) in &occupied.parabolic {
        par.insert(parabolic_key(axis, pole), ());
    }
    let bump = |carrier: &mut crate::network::nerve::Keyed<u64>, k: Vec<u8>| {
        let now = match carrier.get(&k) {
            Some(&v) => v + 1,
            None => 1,
        };
        carrier.insert(k, now);
    };
    PLANES.with(|c| {
        let mut t = c.borrow_mut();
        t.read += 1;
        t.n = t.n.max(carriage.n());
        for (k, ()) in seen.scoped() {
            bump(&mut t.planes, k);
        }
        for (k, ()) in par.scoped() {
            bump(&mut t.parabolic, k);
        }
        // A SCREW: a turn beside a motion off the harmonic half — a boost
        // or a slip — which is where `I₂ ≠ 0` can live (59.7). Neither half
        // alone can produce it.
        if harmonic && (luminal || !occupied.parabolic.is_empty()) {
            t.coupled += 1;
        }
    });
}

/// Read the plane census and clear it.
pub fn take_plane_tally() -> PlaneTally {
    PLANES.with(|c| c.replace(PlaneTally::default()))
}

/// The census without clearing, for a per-form line beside a running
/// total (law 12: only one reader may take).
pub fn plane_tally() -> PlaneTally {
    PLANES.with(|c| c.borrow().clone())
}

impl PlaneTally {
    /// THE ALGEBRA THE CENSUS IS READ IN: the largest carriage noted, and
    /// never less than the corpus's own — the letters' chart turns planes
    /// of the grid's three axes whatever `n` a carriage narrowed to, and a
    /// census that has read nothing names the planes a busy one would. The
    /// one reader of it, for the line and for the observer's census.
    pub fn algebra(&self) -> u8 {
        self.n.max(crate::transducer::element::GRID_N)
    }

    /// How often one plane was populated.
    pub fn count(&self, blade: usize) -> u64 {
        match self.planes.get(&plane_key(blade)) {
            Some(&v) => v,
            None => 0,
        }
    }

    /// How often one parabolic plane was populated (PC1).
    pub fn count_parabolic(&self, axis: usize, pole: Pole) -> u64 {
        match self.parabolic.get(&parabolic_key(axis, pole)) {
            Some(&v) => v,
            None => 0,
        }
    }

    /// THE CAPACITY LINE — every plane of the census's algebra by its own
    /// name, the two halves kept apart, the parabolic planes beside them,
    /// and how many generators coupled a turn to anything else.
    pub fn line(&self) -> String {
        let half = |planes: Vec<usize>| {
            planes
                .into_iter()
                .map(|p| format!("{} {}", plane_name(p), self.count(p)))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let n = self.algebra();
        let parabolic: Vec<String> = (1..=usize::from(n))
            .flat_map(|i| [Pole::Infinity, Pole::Origin].map(|p| (i, p)))
            .map(|(i, p)| format!("{} {}", parabolic_name(i, p), self.count_parabolic(i, p)))
            .collect();
        format!(
            "read {} | harmonic: {} | luminal: {} | parabolic: {} | coupled {}",
            self.read,
            half(harmonic_planes(n)),
            half(transvection_planes(n)),
            parabolic.join(" "),
            self.coupled
        )
    }

    /// How many planes carry anything at all, parabolic ones among them.
    /// The single number that says whether declared capacity is being used.
    pub fn occupied(&self) -> usize {
        self.planes.scoped().filter(|(_, &n)| n > 0).count()
            + self.parabolic.scoped().filter(|(_, &n)| n > 0).count()
    }
}
