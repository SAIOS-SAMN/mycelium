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
/// under a half turn, so `u` cannot tell a body from its own 180°
/// rotation: `φ = 0` and `φ = π` both give `2φ ≡ 0`. The two differ as
/// rotors — `1` against `e₁₂` — and are NOT projectively equal, so this
/// returns the double angle and does not mint a rotor from it. Resolving
/// the residual half turn is a question about the SUPPORT, answered by
/// carrying a candidate through [`versor_coset_ordered`], which tests it
/// against the sites rather than guessing.
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

/// THE FOUR PROPER ROTATIONS OF THE SQUARE BASE, as PROJECTIVE rotors in
/// `Spin(2)(ℚ)` — no root taken and no field extension (56.4).
///
/// ```text
///     φ      R = cos(φ/2) + sin(φ/2) e₁₂     projective        ℓ
///     0      1                               1                 1
///     π/2    (1 + e₁₂)/√2                    1 + e₁₂           4
///     π      e₁₂                             e₁₂               2
///     3π/2   (−1 + e₁₂)/√2                   −1 + e₁₂          4
/// ```
///
/// A versor is projective, so `Ω` and `λΩ` are one transport and the
/// `1/√2` never has to be written: `(1 + e₁₂)(1 − e₁₂) = 1 − e₁₂² = 2 ∈ ℚ`.
///
/// REFLECTIONS ARE NOT HERE. `D₄` has eight elements; the four improper
/// ones lie in `O(2) \ SO(2)` and are not rotors. A reflection between two
/// charts is a different fact from a rotation and is not minted as one.
///
/// ON THE PLANE `(i, j)` BEING READ (BQ4): the bivector is that plane's own
/// `eᵢ ∧ eⱼ`, generated from the base axes, and `e₁₂` on the grid plane.
fn quarter_turns(plane: (usize, usize)) -> [(i64, i64, Mv); 4] {
    let q = |n: i64| BigRational::from_integer(BigInt::from(n));
    let e12 = cga::base_axis(plane.0 + 1) | cga::base_axis(plane.1 + 1);
    let rotor = |s: i64, b: i64| Mv::scalar(q(s)).add(&Mv::blade(e12, q(b)));
    // `(cos φ, sin φ)` beside the rotor that enacts it, so the test below
    // turns the support by the SAME angle the rotor carries.
    [
        (1, 0, rotor(1, 0)),
        (0, 1, rotor(1, 1)),
        (-1, 0, rotor(0, 1)),
        (0, -1, rotor(-1, 1)),
    ]
}

/// `conj(h(a)) · h(b)` — the relative phase at one harmonic order, whose
/// argument is `m·φ`. Scale-free as a ray: both sides carry the same
/// degree, so a dilation contributes a positive real factor.
fn relative_phase(ha: &[BigRational; 2], hb: &[BigRational; 2]) -> (BigRational, BigRational) {
    (
        &ha[0] * &hb[0] + &ha[1] * &hb[1],
        &ha[0] * &hb[1] - &ha[1] * &hb[0],
    )
}

/// `φ` ITSELF, FROM TWO CONSECUTIVE HARMONIC ORDERS (68).
///
/// The even tower fixes `m·φ` and never `φ`. But two orders that differ by
/// one give it directly: `z_m` has argument `m·φ` and `z_{m+1}` has
/// `(m+1)·φ`, so
///
/// ```text
///     z₃ · conj(z₂)     has argument   3φ − 2φ = φ
/// ```
///
/// exactly, over ℚ, with no root taken and no candidate applied to
/// anything. This is what the support's cell test was standing in for.
///
/// `None` when either order vanishes on either body — the phase is not
/// there to read, and the caller keeps the `ℤ₂` freedom rather than
/// choosing a sign it cannot see (which is what manufactures a phantom
/// holonomy around a cycle).
fn relative_turn_ray(a: &Blade, b: &Blade, plane: (usize, usize)) -> Option<(BigRational, BigRational)> {
    let (ua, ub) = (deviatoric(a, plane)?, deviatoric(b, plane)?);
    let (ta, tb) = (third_harmonic(a, plane)?, third_harmonic(b, plane)?);
    let z2 = relative_phase(&ua, &ub);
    let z3 = relative_phase(&ta, &tb);
    // `z₃ · conj(z₂)`.
    Some((
        &z3.0 * &z2.0 + &z3.1 * &z2.1,
        &z3.1 * &z2.0 - &z3.0 * &z2.1,
    ))
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

fn handedness(a: &Blade, b: &Blade, plane: (usize, usize)) -> Handedness {
    let (Some(za), Some(zb)) = (chirality(a, plane), chirality(b, plane)) else {
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

/// A CHART'S FRAME, and whether its half turn is settled (56.5).
///
/// `Turn` is a rotor the SUPPORT confirmed. `HalfOpen` is one the even
/// moments fixed only mod `π`: `u` and `h₄` satisfy `u(−x) = u(x)`, so no
/// even-rank moment distinguishes `φ` from `φ + π`, and where the two
/// bodies differ in cardinality — a transmuted carry — no cell test can
/// settle it either. The representative is carried and the openness is
/// carried WITH it, so a reader cannot mistake a projective class for a
/// determined rotor.
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
pub enum Frame {
    Turn(Mv),
    HalfOpen(Mv),
}

impl Frame {
    /// The rotor either way — the caller that only needs to act takes
    /// this; the caller that must not confuse the two matches instead.
    pub fn rotor(&self) -> &Mv {
        match self {
            Frame::Turn(r) | Frame::HalfOpen(r) => r,
        }
    }
}

/// THE RELATIVE FRAME ROTOR BETWEEN TWO BODIES — `θ_ba`, exact in `ℚ`
/// (56.5, ruled 2026-09-22).
///
/// The deviatoric and the quartic say at what DOUBLE or QUADRUPLE angle
/// two bodies differ, but neither fixes `φ` itself: a rank-2 symmetric
/// tensor is invariant under a half turn, so `u` gives `φ` only mod `π`,
/// and `1` against `e₁₂` are not projectively equal. The residue is a
/// question about the SUPPORT, so the support answers it: each of the four
/// proper turns is applied to `a`'s cells about their centroid and
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
pub enum NoFrame {
    /// A body with no support. Nothing to orient.
    Bodiless,
    /// One body has a principal axis and the other does not, so no turn of
    /// one is the other whatever their sizes.
    Isotropy,
    /// Neither body orients at any order this carrier holds — isotropic at
    /// the second harmonic AND at the fourth.
    Unoriented,
    /// THE AXES SIT AT AN ANGLE THE SQUARE BASE DOES NOT TURN BY, THE
    /// BODIES DIFFER IN SIZE, AND SO NO CELL TEST CAN SETTLE IT. The
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
    /// A REFLECTION CARRIES IT: a named element of `D₄ ∖ SO(2)` maps one
    /// body's cells exactly onto the other's, proved by [`mirror_carries`].
    ///
    /// THE OBSTRUCTION IS THE GEOMETRY, AND GEOMETRY IS IMMUTABLE. An
    /// improper map has determinant `−1` and is a rotation at no angle in
    /// no vocabulary, so no adjoined generator can ever spell it and no
    /// demand should ever be raised for it. Before V1 this arm was reached
    /// only by exhaustion — "a reflection, OR a genuine difference of
    /// shape" — which is rule 11's defect at the producer: two facts of
    /// opposite permanence answering to one name.
    Mirrored,
    /// NO ELEMENT OF `D₄` CARRIES IT, PROPER OR IMPROPER. The moments
    /// agreed far enough to get here and the cells then disagreed: a
    /// genuine difference of shape, which no turn and no mirror repairs.
    ///
    /// Permanent, like [`NoFrame::Mirrored`], but for a different reason,
    /// and the two are kept apart because a census that merges them cannot
    /// say whether a corpus is full of handed pairs or of unlike ones.
    Incongruent,
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
) -> Result<(u8, [BigRational; 2], [BigRational; 2]), NoFrame> {
    match (deviatoric(a, plane), deviatoric(b, plane)) {
        (Some(ua), Some(ub)) => Ok((2, ua, ub)),
        (None, None) => match (quartic_harmonic(a, plane), quartic_harmonic(b, plane)) {
            (Some(qa), Some(qb)) => Ok((4, qa, qb)),
            _ => Err(NoFrame::Unoriented),
        },
        // One has a principal axis and the other does not: no turn of one
        // is the other, whatever their sizes.
        _ => Err(NoFrame::Isotropy),
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
/// [`relative_rotor`]; everything else lands on [`NoFrame::Oblique`].
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
    let (order, oa, ob) = orientation_of(a, b, plane).ok()?;
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
    if order != 2 {
        return None;
    }
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
/// is FR1's reading; this answers one plane.
pub fn relative_rotor(a: &Blade, b: &Blade, plane: (usize, usize)) -> Result<Frame, NoFrame> {
    if a.support.is_empty() || b.support.is_empty() {
        return Err(NoFrame::Bodiless);
    }
    let (order, oa, ob) = orientation_of(a, b, plane)?;
    // SIGNS ONLY, WHICH IS WHAT MAKES IT SCALE-FREE. Under a dilation by
    // `s > 0` the moments scale as `s⁴`, so `u ↦ s⁴ · R(2φ) u` and every
    // sign survives. Taking `cos 2φ = (u_a·u_b)/(‖u_a‖‖u_b‖)` would need
    // that product of norms — the one place a root could enter — and it
    // is never formed. Demanding `‖u_a‖² = ‖u_b‖²` instead, as this did,
    // demanded CONGRUENCE: it refused a dilated carry whose rotation is
    // perfectly well defined and often the identity. A transformation
    // here carries a dilator `D(ρ)`, so similarity is the right group and
    // isometry was too small.
    let dot = &oa[0] * &ob[0] + &oa[1] * &ob[1];
    let wedge = &oa[0] * &ob[1] - &oa[1] * &ob[0];
    let zero = BigRational::from_integer(BigInt::from(0));
    // THE CANDIDATES DEPEND ON THE ORDER THAT PRODUCED THE READING, and
    // reading them off `2φ` unconditionally was a bug (68): a moment of
    // azimuthal order `m` turns at `m·φ`, so the SAME `(wedge, dot)` means
    // different things at `m = 2` and `m = 4`.
    //
    //   m = 2   wedge 0, dot > 0   2φ ≡ 0   φ ∈ {0, π}
    //           wedge 0, dot < 0   2φ ≡ π   φ ∈ {π/2, 3π/2}
    //   m = 4   wedge 0, dot > 0   4φ ≡ 0   φ ∈ {0, π/2, π, 3π/2}  ALL FOUR
    //           wedge 0, dot < 0   4φ ≡ π   φ = π/4 + kπ/2, NO lattice turn
    //
    // The quartic branch tried `[0, 2]` and so refused two of the four
    // turns available to it, which reported as `Incongruent` on exactly
    // the bodies the quartic exists to orient — squares and crosses,
    // isotropic at second order.
    let turn_pair: &[usize] = if wedge == zero && dot > zero {
        match order {
            4 => &[0usize, 1, 2, 3],
            _ => &[0usize, 2],
        }
    } else if wedge == zero && dot < zero && order == 2 {
        // 2φ = π — the axes are at a right angle, so φ is π/2 or 3π/2.
        &[1usize, 3]
    } else {
        // 2φ = ±π/2 is a 45° relation whose half-angle `cos(π/8)` is
        // irrational: refused rather than approximated (rule 4). Anything
        // else is not a turn of the square base at all.
        //
        // V1 — WHICH KIND OF REFUSAL IS THIS (68, ruled 2026-09-22).
        //
        // The deviatoric CANNOT tell these apart on its own: a reflection
        // about an axis at `α` maps `u(2φ)` to `u(2α − 2φ)`, which is a
        // perfectly good rotation OF THE DEVIATORIC and lands here looking
        // exactly like an oblique turn. That is why an L and a J tetromino
        // — mirror images, no rotation between them at any angle — read
        // `Oblique` and would have raised a demand for a generator that
        // cannot exist.
        //
        // So the BODY'S OWN ODD INVARIANT answers it — `Z = h₂²·conj(h₄)`,
        // rotation-invariant and conjugated by every reflection:
        //
        //   Improper   the invariants are conjugate and not real. There is
        //              no rotor at any angle in any vocabulary. PERMANENT.
        //   Unrelated  the invariants are neither equal nor conjugate, so
        //              the bodies differ in a quantity NO conformal motion
        //              changes. PERMANENT, and proved by an invariant
        //              rather than by exhausting a finite group.
        //
        // AND CHIRALITY ALONE IS NOT ENOUGH, which the first recut got
        // wrong. `Z` is REAL for every achiral body, so all of them share
        // one ray and `Proper` degenerates to "handedness does not forbid
        // it" — true, and far too weak: a domino and an L-tromino passed
        // it, and would have asked the court to adjoin a generator
        // relating them. Chirality answers the ODD question only.
        //
        // The EVEN question is A.12's, already built: a similarity must
        // carry the whole normalised tower, `‖M̃_b^(m)‖²·(r_a²)^m =
        // ‖M̃_a^(m)‖²·(r_b²)^m` at every order. A pair the certificate
        // refuses is not similar at ANY angle — permanent, proved by an
        // invariant, and nothing about a lattice is assumed.
        //
        // Both gates must pass for the question to stay open. `Unread` is
        // no chirality reading at all, so it proves nothing on its own and
        // leans entirely on the certificate.
        return Err(match (handedness(a, b, plane), dilation_certified(a, b, plane)) {
            (Handedness::Improper, _) => NoFrame::Mirrored,
            (Handedness::Unrelated, _) | (_, false) => NoFrame::Incongruent,
            _ => NoFrame::Oblique,
        });
    };
    // A BODY WITH NO CENTROID HAS NO MOMENTS, so the odd harmonic below
    // cannot be read and the pair is bodiless rather than unturned.
    if a.centroid().is_none() || b.centroid().is_none() {
        return Err(NoFrame::Bodiless);
    }
    // THE HALF TURN IS A PARITY OBSTRUCTION, AND AN ODD HARMONIC CLOSES
    // IT (68, ruled 2026-09-22).
    //
    // `u` and `h₄` are EVEN-rank: `u(−x) = u(x)`, so no even moment can
    // ever tell `φ` from `φ + π`. This stood in for the missing odd
    // invariant by CARRYING EACH CANDIDATE THROUGH THE CELLS and comparing
    // coordinate lists — and gated that on `support.len()` equality, so a
    // pair of different size was refused a reading altogether and returned
    // mod `π`. Both halves were Euclidean: a coordinate comparison where a
    // geometric one belongs, and a cell count standing in for scale.
    //
    // `h₃` is the odd invariant the comment above used to say was not
    // carried. Two consecutive orders give `φ` outright — `z₃·conj(z₂)`
    // has argument `3φ − 2φ = φ` — exactly, over ℚ, at any relative scale,
    // with nothing applied to anything.
    //
    // WHEN IT VANISHES, THE FREEDOM IS REAL. `h₃ = 0` exactly when the
    // body has 2-fold symmetry, because the half turn maps it to itself
    // and sends `h₃ ↦ −h₃`. Both candidates then act IDENTICALLY on the
    // body, so there is no fact to recover and `HalfOpen` is the honest
    // reading — the `ℤ₂` gauge freedom the coboundary arbitrates, not a
    // measurement this reader failed to take.
    let turns = quarter_turns(plane);
    let Some(phi) = relative_turn_ray(a, b, plane) else {
        return Ok(Frame::HalfOpen(turns[turn_pair[0]].2.clone()));
    };
    let q = |n: i64| BigRational::from_integer(BigInt::from(n));
    for &ix in turn_pair {
        let (cos, sin, rotor) = &turns[ix];
        if same_ray(&phi, &(q(*cos), q(*sin))) {
            return Ok(Frame::Turn(rotor.clone()));
        }
    }
    // `φ` is determined and is not one of the candidates: the bodies are
    // not related by a turn of the base. WHICH fact that is, the body's
    // own invariants say — the same question asked on the oblique branch
    // above, asked here too, because a ruling applies to every site of its
    // shape and not only to the one that reported it.
    Err(match (handedness(a, b, plane), dilation_certified(a, b, plane)) {
        (Handedness::Improper, _) => NoFrame::Mirrored,
        (Handedness::Unrelated, _) | (_, false) => NoFrame::Incongruent,
        _ => NoFrame::Oblique,
    })
}
