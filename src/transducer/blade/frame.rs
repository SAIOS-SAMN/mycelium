// 55.3, 56.5, 66, 68 — THE FRAME READER: the relative turn between two
// bodies read off their harmonic tower, the four proper turns of the square
// base, and the typed refusal where no turn carries one onto the other.

use super::*;

/// THE DOUBLE ANGLE BETWEEN TWO BODIES' PRINCIPAL AXES, exact in ℚ and
/// with no root taken (55.3).
///
/// Diagonalising one tensor needs the roots of its characteristic
/// quadratic, which lie in `ℚ(√Δ)` — refused by rule 4. The RELATIVE
/// angle needs neither diagonalised. `u` turns at `2φ`, so for congruent
/// bodies `‖u_a‖² = ‖u_b‖² = Δ` and
///
/// ```text
///     cos 2φ = (u_a · u_b) / Δ,    sin 2φ = (u_a ∧ u_b) / Δ
/// ```
///
/// are both rational outright. On a square base congruent bodies are
/// related by `D₄`, so `φ ∈ {0, π/2, π, 3π/2}`, `2φ ∈ {0, π, 2π, 3π}` and
/// these take values in `{−1, 0, 1}` only.
///
/// `None` where either body is isotropic (no principal axis) or where the
/// two anisotropies DIFFER. That is not a policy: `cos 2φ = (u_a·u_b) /
/// (‖u_a‖‖u_b‖)` needs the PRODUCT of the norms, and only when
/// `‖u_a‖² = ‖u_b‖²` does that product equal a rational `Δ`. For unequal
/// norms the value is irrational in general and rule 4 refuses it.
///
/// Dropping the gate would not loosen this reading, it would BREAK it:
/// `(u_a·u_b)/‖u_a‖²` with unequal norms is `s⁴ cos 2φ`, a dilated
/// projection that can exceed `±1` and is not an angle.
///
/// [`relative_rotor`] is therefore NOT the same reading with a weaker
/// gate — the two live on different layers:
///
/// ```text
///   METRIC        relative_double_angle   O(2), an isometry invariant
///                 needs ‖u_a‖² = ‖u_b‖², answers an exact cosine
///   PROJECTIVE    relative_rotor          Spin(2)/ℤ₂ on ℙ¹(ℚ), a
///                 similitude invariant; never forms cos 2φ at all
/// ```
///
/// `relative_rotor` asks only WHICH of the four turns applies, which the
/// SIGNS of `u_a·u_b` and `u_a∧u_b` decide, and those survive any
/// dilation because `u ↦ s⁴u` with `s⁴ > 0`. One is the metric
/// restriction of the other's projective ray action. Exact angle values
/// need congruence; the turn itself needs only similarity.
///
/// IT DETERMINES `φ` ONLY MOD `π`. A rank-2 symmetric tensor is invariant
/// under a 180° turn, so `u` cannot tell a body from its own 180°
/// rotation: `φ = 0` and `φ = π` both give `2φ ≡ 0`. The two differ as
/// rotors — `1` against `e₁₂` — and are NOT projectively equal, so this
/// returns the double angle and does not mint a rotor from it. Resolving
/// the residual 180° turn is a question about the SUPPORT, answered by
/// carrying a candidate through [`versor_coset_ordered`], which tests it
/// against the sites rather than guessing.
///
/// `u·u` and `u·v` are THE METRIC ON VECTORS (78.28(4), VA2): each `u` is a
/// body's deviatoric harmonic, a 2-vector, and its square is its own
/// content, not a sum of squares deciding what an element is.
pub fn relative_double_angle(
    ua: &[BigRational; 2],
    ub: &[BigRational; 2],
) -> Option<(BigRational, BigRational)> {
    let da = &ua[0] * &ua[0] + &ua[1] * &ua[1];
    let db = &ub[0] * &ub[0] + &ub[1] * &ub[1];
    if da.is_zero() || da != db {
        return None;
    }
    let dot = &ua[0] * &ub[0] + &ua[1] * &ub[1];
    let wedge = &ua[0] * &ub[1] - &ua[1] * &ub[0];
    Some((dot / &da, wedge / &da))
}

/// Are two plane quantities the SAME RAY — equal up to a strictly
/// positive rational? Exact, and the one comparison a scale-free reading
/// of a normalised tower is allowed to make: `p ∼ q` iff `p ∧ q = 0` and
/// `p · q > 0`. The wedge alone would identify a ray with its opposite.
fn same_ray(p: &(BigRational, BigRational), q: &(BigRational, BigRational)) -> bool {
    let wedge = &p.0 * &q.1 - &p.1 * &q.0;
    let dot = &p.0 * &q.0 + &p.1 * &q.1;
    wedge.is_zero() && dot > BigRational::from_integer(BigInt::from(0))
}

/// HOW TWO BODIES STAND TO EACH OTHER UNDER THE ODD INVARIANT (V1 recut).
///
/// Read off [`chirality`] alone, so it holds in `Cl(4,1)` without any
/// appeal to a base lattice, and it answers for bodies of DIFFERENT SIZE
/// — the ray comparison divides out the dilation, which is what the old
/// cardinality test could not do and refused instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Handedness {
    /// The invariants agree: a PROPER motion is consistent with them.
    /// Also where both bodies are achiral (`Z` real), which is correct —
    /// an achiral body's mirror image is one of its own turns, so there
    /// is no improper-only relation to find.
    Proper,
    /// The invariants are CONJUGATE and not real: no proper motion can
    /// relate these bodies, in any vocabulary, ever.
    Improper,
    /// The invariants are neither: the bodies differ in a quantity NO
    /// conformal motion changes.
    Unrelated,
    /// One body or both is isotropic at an order the invariant needs, so
    /// there is no reading. Never treated as any of the other three.
    Unread,
}

fn handedness(a: &Blade, b: &Blade, plane: (usize, usize), tower: Tower) -> Handedness {
    let (Some(za), Some(zb)) = (chirality_on(a, plane, tower), chirality_on(b, plane, tower)) else {
        return Handedness::Unread;
    };
    if same_ray(&za, &zb) {
        return Handedness::Proper;
    }
    match same_ray(&za, &(zb.0.clone(), -zb.1)) {
        true => Handedness::Improper,
        false => Handedness::Unrelated,
    }
}

/// A RELATIVE POSE, and whether its 180° turn is settled (56.5; named
/// `Frame` until FR1 — 72.1c keeps "frame" for 1.4's sense, the sensor's
/// plane with a carriage, and a body's orientation with its placement is
/// its POSE).
///
/// `Turn` is a rotor the SUPPORT confirmed. `Open180` is one the even
/// moments fixed only mod `π`: `u` and `h₄` satisfy `u(−x) = u(x)`, so no
/// even-rank moment distinguishes `φ` from `φ + π`, and where the two
/// bodies differ in cardinality — a transmuted carry — no compartment test can
/// settle it either. EVERY CANDIDATE is carried — the held stabilizer's
/// coset, never a representative (78.6(1), checklist (r)) — so a reader
/// cannot mistake a projective class for a determined rotor. Two
/// candidates are the held `ℤ₂`; the quartic's four the held `ℤ₄` the
/// same carriage names (FR1c).
///
/// WHY NOT JUST PICK ONE. Choosing a sign per edge injects EXTRINSIC
/// PARITY into the 1-skeleton: three edges of a cycle guessed
/// independently manufacture a `−1` holonomy out of nothing, and the
/// complex then reports a phantom obstruction it invented. Left open, the
/// edge carries a `ℤ₂` gauge freedom and the COBOUNDARY is the arbiter —
/// a 2-cell either locks, in which case `∂σ` fixes the signs globally
/// across it, or it does not, in which case the obstruction is real.
/// Closing it locally instead needs an odd invariant (a third harmonic, a
/// directed contour), which is not carried here.
#[derive(Clone, Debug)]
pub enum Pose {
    Turn(Mv),
    /// The candidates, in the reader's own order; never fewer than two.
    Open180(Vec<Mv>),
}

impl Pose {
    /// The rotor either way — the caller that only needs to act takes
    /// this, the first candidate of a held coset; the caller that must not
    /// confuse the two matches instead, and a contact edge never takes it
    /// (78.6(1)).
    pub fn rotor(&self) -> &Mv {
        match self {
            Pose::Turn(r) => r,
            Pose::Open180(c) => &c[0],
        }
    }
}

/// THE RELATIVE FRAME ROTOR BETWEEN TWO BODIES — `θ_ba`, exact in `ℚ`
/// (56.5, ruled 2026-09-22).
///
/// The deviatoric and the quartic say at what DOUBLE or QUADRUPLE angle
/// two bodies differ, but neither fixes `φ` itself: a rank-2 symmetric
/// tensor is invariant under a 180° turn, so `u` gives `φ` only mod `π`,
/// and `1` against `e₁₂` are not projectively equal. The residue is a
/// question about the SUPPORT, so the support answers it: each of the four
/// proper turns is applied to `a`'s compartments about their centroid and
/// compared with `b`'s. This is a finite group's four elements checked
/// exactly, as `versor_coset_ordered` checks its coset — not a search.
///
/// The harmonics are still what REFUSES: bodies whose anisotropies differ
/// are not congruent, and no turn can carry one onto the other, so the
/// cheap exact test runs first and the support comparison never happens
/// for them.
///
/// WHY TWO BODIES CARRY NO DETERMINED FRAME (66, ruled 2026-09-22).
///
/// `relative_rotor` answered `None` for FIVE different facts, and a
/// refusal that does not say which refusal it is is rule 11's defect at
/// the producer rather than at the call site. The census made it concrete:
/// 89 contacting pairs of 118 came back `None`, and one number could not
/// say whether the corpus is full of unspellable turns — 37.7's condition,
/// live — or of bodies that simply do not orient.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoPose {
    /// A body with no support. Nothing to orient.
    Bodiless,
    /// One body has a principal axis and the other does not, so no turn of
    /// one is the other whatever their sizes.
    Isotropy,
    /// Neither body orients at any order this carrier holds — isotropic at
    /// the second harmonic AND at the fourth.
    Unoriented,
    /// THE AXES SIT AT AN ANGLE THE SQUARE BASE DOES NOT TURN BY, THE
    /// BODIES DIFFER IN SIZE, AND SO NO COMPARTMENT TEST CAN SETTLE IT. The
    /// relative orientation is WELL DEFINED and is not in `D₄`: `2φ =
    /// ±π/2` is the 45° relation whose half-angle `cos(π/8)` is irrational
    /// and refused by rule 4, and anything else is not a lattice turn.
    ///
    /// THE ONLY ARM NOT PROVED PERMANENT. A dilation is in play, so the
    /// crystallographic restriction does not close the question the way it
    /// does at equal cardinality: `z ↦ (1+i)z` turns 45° at scale `√2` and
    /// maps `ℤ[i]` into itself, so similarities the square base cannot
    /// spell genuinely exist. This is 37.7's condition, and D3 is what
    /// supplies or withholds the `ℤ[i]` witness that would settle it.
    ///
    /// It is NOT a claim that a generator exists — only that nothing here
    /// has ruled one out.
    Oblique,
    /// THE ODD INVARIANT CALLS THE PAIR IMPROPER AND NO REFLECTION CARRIES IT.
    /// A mirrored pair the reflection does carry is not refused: it is the
    /// same signal read orientation-reversed, its pose the odd versor
    /// (`settle`, AL1 as amended), spelled through the letter chart's
    /// orientation character. What stays here is an improper reading no
    /// reflected member of the coset verifies.
    Mirrored,
    /// NO MOTION CARRIES IT, PROPER OR IMPROPER: a genuine difference of
    /// shape, which no turn and no mirror repairs — and WHERE, by content
    /// (78.38(2)): the harmonic order at which the two point towers disagree
    /// on the spread, the chirality where every order agrees and `Z` does
    /// not, or the base and the tube the pose reads on. A pair that agrees
    /// on the spread and not on the nucleus radius is one shape at two
    /// scales and is not incongruent.
    ///
    /// Permanent, like [`NoPose::Mirrored`], but for a different reason,
    /// and the two are kept apart because a census that merges them cannot
    /// say whether a corpus is full of handed pairs or of unlike ones.
    Incongruent(Incongruence),
    /// FR1 — THE TUBES' SCALE IS NOT RATIONAL. On the tube the scale is
    /// read from the two bodies' second moments, `ρ² = tr M_b / tr M_a`,
    /// and where `ρ²` is not a rational square no versor over ℚ carries one
    /// tube onto the other: a dilator needs `ρ`. Refused by its cause (rule
    /// 4), and NOT a claim that the tubes are unlike — whether a similarity
    /// stands at that scale is not read, so this is neither `Incongruent`
    /// nor `Oblique` (whose question is the square base's turns, one
    /// section's).
    Irrational,
    /// 78.47 — THE TURN READ DOES NOT SPELL AS CONSTRUCTORS: the constructors'
    /// one Cartan–Dieudonné factorizer, in the algebra's own metric, met a
    /// null step — no reflection exists there, so no versor is built of it.
    /// Carried by name, never assumed away.
    NullStep,
    /// THE STROKE WAS SPENT before every candidate was read (65.5, rule 10):
    /// each candidate a transition verifies draws one read from the stroke,
    /// and a short stroke is this typed cut — never a silent overrun, and
    /// never a claim about the pair, which a longer stroke reads.
    Unbounded,
    /// EVERY CANDIDATE WAS READ AND NONE CARRIES: the covariance refused
    /// each — the first candidate's refusal, in the order the pair's content
    /// builds them, named. Includes a versor that moves infinity (A.13 not
    /// built) and a plane turned off the base, never passed unchecked.
    Uncarried(super::continuous::NotCarried),
    /// THE COSET CARRIES AND NO MEMBER READS A WEIGHT OR A KEY: no
    /// representative names the class, and none is picked in its place.
    Ungauged,
}

/// WHERE TWO SHAPES DISAGREE (78.38(2), (C)): where neither similarity of the
/// lattice carries them, by each tower's refusal (the region's, the sites') —
/// the harmonic order it parted them at, or a body with no points on the
/// plane; in chirality alone where one does; on bases of different `n`; or on
/// the tube, where no orthogonal map carries the one tower onto the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Incongruence {
    Order { region: TowerRefusal, sites: TowerRefusal },
    Chirality,
    Base,
    Tube,
}

impl NoPose {
    /// PHY1 — THE LANE THAT REFUSED: the degree of freedom the pose could not
    /// carry, where the refusal names one. A turn the pair would not take —
    /// isotropic, mirrored, oblique, a null step, an ungauged coset — is the
    /// rotor's lane, 𝒽 (42.6's table: the rotor's coordinates are holonomic,
    /// `Axis::lane`; 𝒢 is hue). It read 𝒢, so a refused turn pressed the
    /// lane recolour work is measured on (78.69). A shape or a scale no
    /// motion carries — incongruent, irrational — is the spatial lane, 𝒮. A
    /// refusal that names no lane — the covariance refusing every candidate,
    /// a held stabilizer, a cut stroke, nothing to pose — is `None`: no
    /// pressure is invented for it.
    pub fn lane(self) -> Option<usize> {
        use crate::transducer::rewrite::{LANE_SHELL, LANE_SPATIAL};
        match self {
            NoPose::Isotropy | NoPose::Mirrored | NoPose::Oblique | NoPose::NullStep | NoPose::Ungauged => Some(LANE_SHELL),
            NoPose::Incongruent(_) | NoPose::Irrational => Some(LANE_SPATIAL),
            NoPose::Uncarried(_) | NoPose::Unoriented | NoPose::Unbounded | NoPose::Bodiless => None,
        }
    }

    /// CV1 — THE KIND OF A POSE REFUSAL (78.9), on the REFRACTION only: a
    /// static contact still carries its tether (78.6(2)). No rational versor
    /// relates the two towers — mirrored, incongruent, isotropic, irrational
    /// — is P; the one arm a demand retires (D3, `RotorPrime`), oblique, is
    /// D; a bodiless pair has nothing to pose, I. An unoriented pair is a
    /// held stabilizer beyond `ℤ₂` (FR1c), NOT a refusal: `None`.
    pub fn obstruction_kind(self) -> Option<crate::transducer::sentence::ObstructionKind> {
        use crate::transducer::sentence::ObstructionKind;
        match self {
            NoPose::Mirrored
            | NoPose::Incongruent(_)
            | NoPose::Isotropy
            | NoPose::Irrational
            | NoPose::NullStep
            | NoPose::Uncarried(_) => Some(ObstructionKind::P),
            NoPose::Ungauged => Some(ObstructionKind::Unclassified),
            NoPose::Oblique => Some(ObstructionKind::D),
            NoPose::Bodiless => Some(ObstructionKind::I),
            // A cut by the stroke is no reading of the pair at all.
            NoPose::Unoriented | NoPose::Unbounded => None,
        }
    }
}

/// WHICH HARMONIC ORIENTS A PAIR (R1): the deviatoric, turning at `2φ`, where
/// both bodies have a principal axis; the quartic, at `4φ`, where neither does.
/// The odd order never orients — it only settles the 180° turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Orienting {
    Deviatoric,
    Quartic,
}

/// THE ORIENTATION VECTORS OF A PAIR. The second harmonic where the body
/// has a principal axis; the fourth where it does not — a square or a
/// cross is isotropic at second order and the quartic still orients it.
/// Isotropic at both orders is a body with no orientation to compare.
///
/// Split out so [`relative_rotor`] and [`oblique_arc`] read the SAME
/// orientation. Two readers deriving it separately is how the two would
/// drift onto different harmonics and disagree about the angle they are
/// each describing.
///
/// THE ORDER COMES BACK WITH THE VECTORS, and it is not decoration: a
/// moment of azimuthal order `m` turns at `m·φ`, so the deviatoric's
/// reading is `2φ` and the quartic's is `4φ`. A caller that reads an
/// angle off these without knowing which order produced them is off by a
/// factor of two and cannot tell.
fn orientation_of(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    tower: Tower,
) -> Result<(Orienting, [BigRational; 2], [BigRational; 2]), NoPose> {
    match (harmonic(a, plane, tower, Harmonic::Two), harmonic(b, plane, tower, Harmonic::Two)) {
        (Some(ua), Some(ub)) => Ok((Orienting::Deviatoric, ua, ub)),
        (None, None) => match (harmonic(a, plane, tower, Harmonic::Four), harmonic(b, plane, tower, Harmonic::Four)) {
            (Some(qa), Some(qb)) => Ok((Orienting::Quartic, qa, qb)),
            _ => Err(NoPose::Unoriented),
        },
        // One has a principal axis and the other does not: no turn of one
        // is the other, whatever their sizes.
        _ => Err(NoPose::Isotropy),
    }
}

/// THE PRIMITIVE GAUSSIAN ARC OF AN OBLIQUE PAIR'S DOUBLE ANGLE
/// (D3, 68 — ruled 2026-09-22).
///
/// `z = (u_a·u_b) + (u_a∧u_b)·i`, whose argument is `2φ` — the relative
/// orientation the deviatoric fixes, which is `φ` only mod `π` because
/// every even moment satisfies `u(−x) = u(x)`.
///
/// A ROTOR IS A RAY, so it is a Gaussian integer `w` with `arg(w) = φ/2`,
/// and therefore `w⁴ ∝ z` up to a POSITIVE rational. That is what makes
/// the spellable set exactly two directions and not four:
///
/// ```text
///   z ~ +1   w = 1        (1)⁴ = 1        φ = 0 or π
///   z ~ −1   w = 1 + i    (1+i)⁴ = −4     φ = π/2 or 3π/2
///   z ~ ±i   —            arg w = π/8, and tan(π/8) = √2 − 1 is
///                         irrational, so NO Gaussian integer has it
/// ```
///
/// Those two are precisely the `wedge == 0` branches of
/// [`relative_rotor`]; everything else lands on [`NoPose::Oblique`].
///
/// PRIMITIVE, AND THAT IS NOT COSMETIC. The court reads the factor as
/// `π_p = gcd_{ℤ[i]}(s + t·i, p)` and refuses a witness whose gcd is not
/// of norm exactly `p` (`REFUSE_LETTER_WITNESS`). If `g = gcd(s,t) > 1`
/// and `p | g`, that gcd comes back with norm `p²` and the letter is
/// thrown out — so a composite arc would mint demands the court can only
/// refuse, which is worse than minting none.
///
/// SCALE-FREE, so a dilated pair reads the same arc: `u ↦ s⁴u` scales `z`
/// by `s⁸ > 0`, and dividing out the content removes it.
pub fn oblique_arc(a: &Blade, b: &Blade, plane: (usize, usize)) -> Option<(BigInt, BigInt)> {
    use num_integer::Integer;
    // Read on the tower whose similarity carries the pair (78.38 (C)).
    let (order, oa, ob) = orientation_of(a, b, plane, reading_tower(a, b, plane).0).ok()?;
    // ORDER 2 ONLY, AND THAT IS A REFUSAL RATHER THAN A GAP. The quartic
    // orients a body the deviatoric cannot, but it turns at `4φ`, so its
    // arc answers `w⁸ ∝ z` and a witness minted from it would name a
    // rotation at half the angle the pair actually stands at. The court
    // cannot tell the two apart from the block alone — it reads `s + t·i`
    // and nothing says which harmonic produced it — so a wrong-order
    // witness is not a weaker demand, it is a false one.
    //
    // Refused here rather than corrected, because `w⁸` is a DIFFERENT
    // spellability question from `w⁴` and answering it is its own ruling.
    let Orienting::Deviatoric = order else {
        return None;
    };
    let dot = &oa[0] * &ob[0] + &oa[1] * &ob[1];
    let wedge = &oa[0] * &ob[1] - &oa[1] * &ob[0];
    // `z = 0` is no direction at all: an orientation vector vanished, and
    // an angle read off it would be invented (rule 11).
    if dot.is_zero() && wedge.is_zero() {
        return None;
    }
    // Clear denominators, then divide out the content. Any factor the
    // clearing introduced is common to both parts and the gcd removes it,
    // so the result is primitive however it was scaled.
    let d = dot.denom() * wedge.denom();
    let (s, t) = (dot.numer() * (&d / dot.denom()), wedge.numer() * (&d / wedge.denom()));
    let g = s.gcd(&t);
    match g.is_zero() {
        true => None,
        false => Some((&s / &g, &t / &g)),
    }
}

/// ON THE PLANE THE CALLER NAMES (BQ4, 71.2(3)). The turn, its harmonics,
/// its odd invariant and its refusal are all read in that plane's own
/// coordinates, and the rotor is that plane's bivector. Which planes a
/// relation between two bodies turns across — the pose over their tubes —
/// is [`super::relative_pose`]'s reading (FR1); this answers one plane, and
/// is what the pose reads where both bodies stand in one section.
pub fn relative_rotor(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    budget: &mut crate::physics::Budget,
) -> Result<Pose, NoPose> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoPose::Bodiless);
    }
    // 78.38 (C) — BOTH TOWERS ARE READ, and the relation is read on the tower
    // whose similarity carries the pair: the sites' where only a site map
    // does, the region's otherwise.
    let (tower, fit) = reading_tower(a, b, plane);
    let certified = fit.map(|f| f.dilation);
    orientation_of(a, b, plane, tower)?;
    // WHERE NEITHER TOWER FITS THE PAIR THROUGH ITS DEVIATORIC, it has no
    // orientation in common: refused with the order from each tower, the
    // mirror named first.
    let Ok(fit) = fit else {
        return Err(unturned(a, b, plane, tower, certified));
    };
    // THE TRANSITION'S MEMBERS (`transitions_between`): the turn read off the
    // harmonics' phases by the exact half-angle step, its coset, the odd
    // members against the mirrored harmonics, each verified by the tower's
    // covariance — no lattice turn is listed and no compartment lands.
    //
    // THE 180° TURN IS A PARITY OBSTRUCTION (68): `u` and `h₄` are even-rank
    // and cannot tell `φ` from `φ + π`; the odd harmonic `h₃` does, and the
    // verification carries it, so where the body has one exactly one of the
    // pair carries. Where `h₃ = 0` the body has 2-fold symmetry, both carry,
    // and `Open180` is the honest reading — the `ℤ₂` gauge freedom the
    // coboundary arbitrates, never a sign chosen per edge.
    //
    // A PAIR IS READ ORIENTATION-REVERSED ONLY WHERE NO TURN CARRIES IT (AL1
    // as amended): a body symmetric under a mirror is carried by its
    // reflections as well as its turns, and its pose is the turns; a mirrored
    // pair is the same signal read orientation-reversed, its pose the odd
    // rotation, spelled through the letter chart's orientation character.
    //
    // THE POSE IS THE ROTATION; the residue a fit leaves above its order is
    // the transition's (`transitions_between`), read there with its versor.
    // A spent stroke is the stroke's cut. A coset nothing fills is named by
    // the bodies' own invariants below — the pose's reading of WHICH fact
    // no member carries (`unturned`), finer than the transition's.
    let members = match super::carry::transitions_on(a, b, plane, fit, budget) {
        Ok(members) => members,
        Err(NoPose::Unbounded) => return Err(NoPose::Unbounded),
        Err(NoPose::Uncarried(_) | NoPose::Oblique | NoPose::Irrational) => Vec::new(),
        Err(other) => return Err(other),
    };
    let (turns, mirrors): (Vec<Mv>, Vec<Mv>) =
        members.into_iter().map(|t| t.rotation).partition(|r| r.parity() == Some(false));
    let read = match turns.is_empty() {
        true => mirrors,
        false => turns,
    };
    match read.as_slice() {
        // No rational similarity carries the pair on this plane. WHICH fact
        // that is, the bodies' own invariants say (V1, 68): conjugate
        // chirality is Mirrored, unrelated chirality Incongruent, and a
        // proper relation at no rational turn or scale Oblique — the one arm
        // a demand retires (D3).
        [] => Err(unturned(a, b, plane, tower, certified)),
        [one] => Ok(Pose::Turn(one.clone())),
        _ => Ok(Pose::Open180(read)),
    }
}

/// 78.38 (C) — THE TOWER A PAIR'S RELATION IS READ ON, and the certificate's
/// fit that named it: the tower that fits the pair through the finer order,
/// the region's where none fits it (the relation is then refused on it, at
/// the order each tower names).
fn reading_tower(a: &Blade, b: &Blade, plane: (usize, usize)) -> (Tower, Result<Fit, (TowerRefusal, TowerRefusal)>) {
    let fit = certify(a, b, plane);
    let tower = match fit {
        Ok(f) => f.dilation.tower(),
        Err(_) => Tower::Region,
    };
    (tower, fit)
}

/// WHICH FACT A PAIR NO TURN OF THE BASE RELATES IS: mirrored where the odd
/// invariants are conjugate on the tower read; incongruent where both towers
/// refuse the pair as dilates — at the order each names — or where a
/// similarity carries it and the chirality alone differs; oblique where a
/// similarity carries it and a proper motion is consistent — the one arm a
/// demand retires (D3). One statement for both of `relative_rotor`'s
/// branches (law 12).
fn unturned(
    a: &Blade,
    b: &Blade,
    plane: (usize, usize),
    tower: Tower,
    certified: Result<Dilation, (TowerRefusal, TowerRefusal)>,
) -> NoPose {
    match (handedness(a, b, plane, tower), certified) {
        (Handedness::Improper, _) => NoPose::Mirrored,
        (_, Err((region, sites))) => NoPose::Incongruent(Incongruence::Order { region, sites }),
        (Handedness::Unrelated, Ok(_)) => NoPose::Incongruent(Incongruence::Chirality),
        (Handedness::Proper | Handedness::Unread, Ok(_)) => NoPose::Oblique,
    }
}
