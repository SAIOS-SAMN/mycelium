//! D1, 68 — the ten bivector planes of `Cl(4,1)`, split into the harmonic
//! and luminal halves, and the census of which ones the engine populates.

use super::multivector::{Mv, E1, E2, E3, EM, EP};
use num_traits::Zero;

/// THE FOUR HYPERBOLIC GENERATORS `eₐ ∧ e₋`, the mixed bivectors `Cl(4,0)`
/// has none of. Each squares to `+1`:
///
/// ```text
///     (eₐe₋)² = eₐe₋eₐe₋ = −eₐeₐe₋e₋ = −(+1)(−1) = +1
/// ```
///
/// FOUR, NOT THREE. A first cut declared `e₁∧e₋, e₂∧e₋, e₃∧e₋` and dropped
/// `e₊∧e₋`. `Spin(4,1)` is the isometry group of HYPERBOLIC 4-SPACE whose
/// boundary at infinity is the three-coordinate base (51.2), and ℍ⁴ is four
/// dimensional: its boosts are `eₐ ∧ e₋` for every spacelike `a`, and `e₊`
/// is spacelike (`e₊² = +1`). Declaring three of four restricts dimensional
/// capacity, which the Anti-Flattening Law clause 5 refuses by name — the
/// same defect as collapsing `Λ²(ℝ⁴)` to one blade, one dimension up.
pub const TRANSVECTION_PLANES: [usize; 4] = [E1 | EM, E2 | EM, E3 | EM, EP | EM];

/// THE TEN BIVECTOR PLANES OF `Cl(4,1)`, IN ONE PLACE AND SPLIT BY HALF
/// (D1, 68 — ruled 2026-09-22).
///
/// `Λ²(ℚ^{4,1})` is ten dimensional and the engine reads it as two halves
/// that behave differently under `B²`:
///
/// ```text
///   HARMONIC   e₁e₂ e₁e₃ e₂e₃ e₁e₊ e₂e₊ e₃e₊      B² < 0, compact, turns
///   LUMINAL    e₁e₋ e₂e₋ e₃e₋ e₊e₋                B² > 0, non-compact, boosts
/// ```
///
/// Six and four. A census that counts "bivectors" without this split
/// cannot say whether a corpus turns or boosts, and the whole tetrachotomy
/// is a statement about which half a generator lives in.
///
/// THE NAMES ARE THE BLADE'S OWN BITS, so a reader gets the plane back
/// rather than an ordinal (rule 15).
pub const HARMONIC_PLANES: [usize; 6] =
    [E1 | E2, E1 | E3, E2 | E3, E1 | EP, E2 | EP, E3 | EP];

/// Every bivector plane, harmonic first then luminal, so an index into a
/// tally is stable and its half is read off the index.
pub const BIVECTOR_PLANES: [usize; 10] = [
    E1 | E2,
    E1 | E3,
    E2 | E3,
    E1 | EP,
    E2 | EP,
    E3 | EP,
    E1 | EM,
    E2 | EM,
    E3 | EM,
    EP | EM,
];

/// The plane's own name, for a line a reader can act on.
pub fn plane_name(blade: usize) -> &'static str {
    match blade {
        b if b == E1 | E2 => "e12",
        b if b == E1 | E3 => "e13",
        b if b == E2 | E3 => "e23",
        b if b == E1 | EP => "e1p",
        b if b == E2 | EP => "e2p",
        b if b == E3 | EP => "e3p",
        b if b == E1 | EM => "e1m",
        b if b == E2 | EM => "e2m",
        b if b == E3 | EM => "e3m",
        b if b == EP | EM => "epm",
        _ => "?",
    }
}

/// Is this plane compact — does its generator TURN rather than boost?
/// `B² < 0` for the six that avoid `e₋`, and `e₋² = −1` is what makes the
/// other four hyperbolic (54.7).
pub const fn plane_is_harmonic(blade: usize) -> bool {
    blade & EM == 0
}

/// D1 — WHICH PLANES THE ENGINE ACTUALLY POPULATES (68).
///
/// Ten counters, one per bivector plane, so a recut can be read for
/// DIMENSIONAL CAPACITY rather than for throughput. A corpus that turns
/// only on `e₁e₂` and boosts only on `e₁e₋` is using two of ten planes,
/// and no aggregate count says so — it reports a busy engine occupying a
/// line.
///
/// This is the Anti-Flattening Law's clause 5 made observable: declaring
/// capacity and never populating it is the same defect as not declaring
/// it, and until now nothing could tell the two apart.
#[derive(Default, Clone, Copy, Debug)]
pub struct PlaneTally {
    /// Non-zero occupancy per plane, indexed by [`BIVECTOR_PLANES`] — so
    /// `0..6` is the harmonic half and `6..10` the luminal one.
    pub planes: [u64; 10],
    /// Generators read, and those carrying content in BOTH halves — a
    /// screw, which is where `I₂ ≠ 0` can live (59.7).
    pub read: u64,
    pub coupled: u64,
}

thread_local! {
    static PLANES: std::cell::Cell<PlaneTally> =
        const { std::cell::Cell::new(PlaneTally { planes: [0; 10], read: 0, coupled: 0 }) };
}

/// THE ROTOR'S SIX PLANES, WHICH THE CARRIAGE CANNOT CARRY.
///
/// A transition is `D(ρ)·T(t)·R` and the two halves live in DIFFERENT
/// ALGEBRAS (L11e): the carriage in `Cl(3,1)`, the rotor over all six
/// planes of `Cl(4,0)`. A census of the carriage alone therefore reads
/// ZERO on the entire harmonic half, structurally — which is what the
/// first cut of this did, and the corpus obligingly reported every
/// harmonic plane empty.
///
/// `Cl(4,0)`'s six bivector blades are the pairs of `{e₀,e₁,e₂,e₃}`, and
/// 51.2 fixes the rotor chart's correspondence: `(0,1)`, `(0,3)`, `(1,3)`
/// are `e₁e₂`, `e₁e₃`, `e₂e₃`. The remaining three are the chart's
/// off-plane channels, which is where orders outside `{1,2,4}` live (65.6)
/// — so a corpus using them is turning off the sensor plane, and one that
/// never does is flat however busy it looks.
/// ONE TRANSITION, BOTH HALVES, ONE COUNT. A transition is `D(ρ)·T(t)·R`
/// and it is ONE generator however many carriers hold it, so `read`
/// increments once and `coupled` can see both halves at once — which a
/// per-carrier call could not.
pub fn note_transition(r: &crate::transducer::alphabet::Rotor, carriage: &Mv) {
    // The six grade-2 masks of `Cl(4,0)`, in the chart's own order.
    const ROTOR_PLANES: [usize; 6] = [0b0011, 0b0101, 0b1001, 0b0110, 0b1010, 0b1100];
    let mut seen = [0u64; 10];
    let mut harmonic = false;
    for (i, &mask) in ROTOR_PLANES.iter().enumerate() {
        if !r.coeff(mask).is_zero() {
            seen[i] = 1;
            harmonic = true;
        }
    }
    // The carriage contributes the LUMINAL four and the three `e_i∧e₊`
    // the rotor chart does not reach; both are read off its own blades.
    let b = carriage.grade(2);
    let mut luminal = false;
    for (i, &plane) in BIVECTOR_PLANES.iter().enumerate() {
        if !b.c[plane].is_zero() {
            seen[i] = 1;
            if !plane_is_harmonic(plane) {
                luminal = true;
            }
        }
    }
    PLANES.with(|c| {
        let mut t = c.get();
        t.read += 1;
        for (slot, s) in t.planes.iter_mut().zip(seen.iter()) {
            *slot += s;
        }
        // A SCREW: content in both halves at once, which is where `I₂ ≠ 0`
        // can live (59.7). Neither half alone can produce it.
        if harmonic && luminal {
            t.coupled += 1;
        }
        c.set(t);
    });
}

/// Read the plane census and clear it.
pub fn take_plane_tally() -> PlaneTally {
    PLANES.with(|c| c.replace(PlaneTally::default()))
}

/// The census without clearing, for a per-form line beside a running
/// total (law 12: only one reader may take).
pub fn plane_tally() -> PlaneTally {
    PLANES.with(|c| c.get())
}

impl PlaneTally {
    /// THE CAPACITY LINE — every plane by its own name, the two halves
    /// kept apart, and how many generators coupled them.
    pub fn line(&self) -> String {
        let half = |r: std::ops::Range<usize>| {
            r.map(|i| format!("{} {}", plane_name(BIVECTOR_PLANES[i]), self.planes[i]))
                .collect::<Vec<_>>()
                .join(" ")
        };
        format!(
            "read {} | harmonic: {} | luminal: {} | coupled {}",
            self.read,
            half(0..6),
            half(6..10),
            self.coupled
        )
    }

    /// How many of the ten planes carry anything at all. The single number
    /// that says whether declared capacity is being used.
    pub fn occupied(&self) -> usize {
        self.planes.iter().filter(|&&n| n > 0).count()
    }
}
