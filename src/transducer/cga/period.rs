//! 54.7 — the toral period of a motion: the cyclotomic orders a versor can
//! close at, and the three ways a motion sounds no period at all.

use super::group::is_versor;
use super::multivector::{one, Mv};

/// THE TORAL PERIOD OF A MOTION OF `Cl(4,1)` — the acoustic reading (54.7).
///
/// A toral period is the order of a ROTATION: a compact invariant. So it is
/// asked of the COMPACT PART of the generator and of nothing else.
///
/// WHY NOT OF THE WHOLE MOTION. `Rotor::toral_period` reads the `Cl(4,0)`
/// alphabet chart, where every letter is a Euclidean rotation and therefore
/// every word is compact and periodic — 545 of 545 corpus cycles read as
/// CLOSING while 541 of them classified Hyperbolic. That reader interrogates
/// the ALPHABET; this one interrogates the GEOMETRY.
///
/// AN OPEN ORBIT HAS TWO CAUSES AND BOTH ARE READ HERE (the Niven trap):
///
///   1. `B_ell = 0` — no rotation at all. A pure Burgers dislocation slips
///      along a geodesic and never returns.
///   2. `B_ell ≠ 0` and the winding is IRRATIONAL. `cos θ` is rational by
///      construction for a rational rotor, and Niven's theorem says a
///      rational cosine with `θ/π ∈ ℚ` forces `cos θ ∈ {0, ±½, ±1}`. The
///      `(3,4,5)` rotor has `cos θ = −7/25`, which is rational and not one
///      of those — so its orbit is dense on the circle and closes never.
///      `half_angle` is exactly that test and answers `None` there.
///
/// So `B_ell ≠ 0` does NOT imply a finite period, and a finite period is
/// always one of `{1, 2, 3, 4, 6}` — the crystallographic restriction,
/// arriving from ℚ rather than from a convention. A period of 5 or 7 is
/// not merely absent; it is unreachable.
///
/// `None` is the projective boundary `[1:0]`: an open orbit, charged as
/// shear rather than sounding a pitch.
/// THE CYCLOTOMIC CANDIDATES — every order a versor of this algebra can
/// close at (54.7, ruled 2026-09-21).
///
/// A finite-order motion has eigenvalues that are roots of unity, so its
/// minimal polynomial is a product of cyclotomics. Acting on the
/// `(n+1)`-dimensional space of `Cl(n+1, 1)`'s compact part bounds that
/// degree, `φ(ℓ) ≤ n + 1`, so the candidates are GENERATED from `n`
/// (71.2(3)) and never listed. At the corpus's `n = 3`:
///
/// ```text
///     φ(ℓ) ≤ 4   ⟹   ℓ ∈ {1, 2, 3, 4, 5, 6, 8, 10, 12}
/// ```
///
/// `φ(7) = 6`, `φ(9) = 6`, `φ(11) = 10` — excluded there, so an order of 7
/// is not merely absent from that corpus, it is unreachable.
///
/// `{1,2,3,4,6}` are the crystallographic orders, reachable when the
/// discriminant is a rational square (`φ(ℓ) ≤ 2`, Niven). `{5,8,10,12}`
/// need `φ(ℓ) = 4` — the minimal polynomial staying irreducible over ℚ —
/// which happens exactly when `√Δ ∉ ℚ`. Pentagonal and dodecagonal
/// symmetry are therefore not forbidden; they live in `ℚ(√Δ)`.
///
/// THE ORDER IS THE LCM OF THE EIGENVALUES' ORDERS (audit N-5). The minimal
/// polynomial is a product of DISTINCT cyclotomics `Φ_ℓᵢ`, their degrees
/// summing to at most `d = n + 1`, and the motion closes at
/// `lcm(ℓ₁, ℓ₂, …)` — not at an `ℓ` of its own with `φ(ℓ) ≤ d`. The two
/// readings agree up to `d = 5`; at `d = 6` the eigenvalues of orders 3
/// and 5 fit (`φ = 2 + 4`) and the motion closes at 15, where `φ(15) = 8`:
/// a reader over single `ℓ` would call it Open, a false reading and not a
/// refusal (rule 11). So the candidates are every lcm of a set of atoms
/// within the degree, generated from `n`.
///
/// THE ATOMS ARE BOUNDED BY A THEOREM, not a constant: `φ(ℓ) ≥ √(ℓ/2)`, so
/// `φ(ℓ) ≤ d` forces `ℓ ≤ 2d²`, and every `ℓ` up to that bound is read once.
/// Each atom enters a set once (a repeated factor adds degree and no
/// order), and a set is kept at the least degree its lcm needs.
pub fn cyclotomic_orders(n: u8) -> Vec<u32> {
    let d = u32::from(n) + 1;
    let totient = |l: u32| (1..=l).filter(|&k| gcd(k, l) == 1).count() as u32;
    // `(order, degree)`: the lcm a set of atoms closes at and the degree it
    // spends. The empty set closes at 1 and spends nothing.
    let mut reach: Vec<(u32, u32)> = vec![(1, 0)];
    for l in 2..=2 * d * d {
        let phi = totient(l);
        if phi > d {
            continue;
        }
        let grown: Vec<(u32, u32)> = reach
            .iter()
            .filter(|&&(_, used)| used + phi <= d)
            .map(|&(order, used)| (order / gcd(order, l) * l, used + phi))
            .collect();
        reach.extend(grown);
        // The canonical order: by order, then by degree, so the first of
        // each order is its least degree and the rest are dropped.
        reach.sort_unstable();
        reach.dedup_by_key(|&mut (order, _)| order);
    }
    reach.into_iter().map(|(order, _)| order).collect()
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// DOES THIS MOTION CLOSE, AND AT WHAT ORDER — asked of `Ω` WHOLE.
///
/// ```text
///     Ω^ℓ ~ ±1  in  Spin⁺(3,1)(ℚ)
/// ```
///
/// PROJECTIVELY, because `Ω` and `−Ω` are one element of the ray: a `2π`
/// spatial turn evaluates to `−1` and acts identically on every vector, so
/// testing `Ω^ℓ = 1` would levy shear on a gauge sign.
///
/// NO SPLIT, NO ROOT, NO SCALAR DISCRIMINANT. This replaces
/// [`bivector_split`] as the gatekeeper of periodicity, and the reason is
/// that the split was never the question. Decomposing `B` to measure a
/// period projected a coupled six-parameter group action onto isolated 2D
/// tangent slices in order to run a scalar angle formula — and then
/// refused whenever `√Δ ∉ ℚ`, reporting an arithmetic decline as if it were
/// a property of the cycle. Iterated Clifford multiplication asks the
/// motion directly.
///
/// The iteration is over [`cyclotomic_orders`] at the motion's own `n`, a
/// set fixed by `φ(ℓ) ≤ n + 1` — a theorem about the algebra, not a read
/// over a carrier (rule 3).
///
/// `None` is an open orbit. A motion with any hyperbolic content cannot
/// close at all: `Ω = Ω_ell Ω_hyp` commuting gives
/// `Ω^k = Ω_ell^k Ω_hyp^k`, and `Ω_hyp^k = cosh(kβ) + sinh(kβ)B_hyp/β` is
/// never `±1` for `β > 0` — so even a dispiration whose rotation closes at
/// a fifth root of unity drifts along its screw axis forever.
pub fn finite_order(m: &Mv) -> Option<std::num::NonZeroU32> {
    is_versor(m)?;
    let unit = Mv::scalar(one());
    let orders = cyclotomic_orders(m.n());
    let mut acc = m.clone();
    // index: the algebra's own cyclotomic candidates, bounded by
    // φ(ℓ) ≤ n + 1, in ascending order, so the first that closes is the
    // order.
    for k in 1..=orders.iter().copied().fold(1, u32::max) {
        if orders.contains(&k) && acc.projective_eq(&unit) {
            return std::num::NonZeroU32::new(k);
        }
        acc = acc.geo(m);
    }
    None
}

pub fn compact_period(m: &Mv) -> Spectrum {
    use crate::transducer::sentence::ToralPeriod;
    // 54.7 — ASKED OF THE MOTION WHOLE, NOT OF A DECOMPOSITION OF IT.
    match finite_order(m) {
        // ONE PLANE TURNS AND THE ORTHOGONAL ONE IS IDLE, which is a period
        // of one and not an absence: an idle plane closes trivially and
        // earns no yield, where absence would say it turns forever.
        Some(plane) => Spectrum::Closes(ToralPeriod {
            plane_1: Some(plane),
            plane_2: std::num::NonZeroU32::new(1),
        }),
        None => Spectrum::Open,
    }
}

/// WHAT A MOTION'S ACOUSTIC READING IS — and the three ways there is none,
/// kept apart (rule 11).
///
/// A first cut returned `Option<ToralPeriod>` and every `None` was counted
/// as an open orbit: 545 of 545 "open", which folded a genuine dense
/// winding together with two algebraic refusals and reported the sum as a
/// measurement. An orbit that does not close and a split that does not
/// exist are different facts about different things.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spectrum {
    /// The compact part closes at a crystallographic order.
    Closes(crate::transducer::sentence::ToralPeriod),
    /// The orbit genuinely never returns: either there is no compact part
    /// (a pure dislocation) or its winding is irrational (Niven). This is
    /// the projective boundary `[1:0]`, charged as shear.
    Open,
    /// `√Δ ∉ ℚ`: the commuting eigenplanes live in `ℚ(√Δ)` and the motion
    /// cannot be decomposed into rational simple parts without extending
    /// the field. An obstruction, not an orbit.
    Undecomposed,
    /// The compact part turns in two planes at once. It has a spectrum;
    /// this reader does not take it, and says so rather than reporting one
    /// plane's order as the whole.
    Isoclinic,
}

// `gcd_u32` stood here and is deleted: `alphabet::word::gcd_u32` is the
// one producer and the one `alphabet::rotor` already reads (law 12). Two
// spellings of one arithmetic is one too many, and this copy had no
// caller at all.
