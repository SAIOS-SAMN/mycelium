// 16.1, 9 — THE ROTOR: an even versor over the exact rationals, and the
// arithmetic that composes two of them.
//
//! A rotor is the coefficients of the even subalgebra, held exactly. It is
//! compared PROJECTIVELY — a rotor and its negation are one element — and
//! keyed by its coordinates written out, never by a digest of them.

use crate::transducer::alphabet::{Word, Q};
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

use super::*;

pub(crate) fn q(v: i64) -> Q {
    Q::from_integer(BigInt::from(v))
}
pub(crate) fn qn(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

// `pack_axis` stood here: a rational cut into a numerator limb and a
// denominator limb for `class_axes` to write as two chart axes. Two limbs
// are not two coordinates — `peel` could stop between them — and both
// readers are gone (`stalk::rotor_axes` writes the rational whole as one
// `ChartAxis::Ratio`).

/// An exact multivector of `Cl(4,0)`: 16 blade coefficients indexed by a
/// 4-bit mask over `e0..e3`, every generator squaring to +1.
/// Vertex equality is [`Rotor::projective_eq`]; coefficient identity is
/// only the normal-form constructor's check.
#[derive(Debug, Clone)]
pub struct Rotor {
    c: [Q; 16],
}

impl PartialEq for Rotor {
    fn eq(&self, other: &Self) -> bool {
        self.projective_eq(other)
    }
}
impl Eq for Rotor {}

/// The parity of the transpositions sorting blade `a` before blade `b`.
pub(crate) fn reorder_sign(a: usize, b: usize) -> i64 {
    let mut hi = a >> 1;
    let mut swaps = 0u32;
    while hi != 0 {
        swaps += (hi & b).count_ones();
        hi >>= 1;
    }
    if swaps & 1 == 0 {
        1
    } else {
        -1
    }
}

impl Rotor {
    pub fn zero() -> Rotor {
        Rotor { c: std::array::from_fn(|_| Q::zero()) }
    }

    pub fn scalar(v: Q) -> Rotor {
        let mut r = Rotor::zero();
        r.c[0] = v;
        r
    }

    pub fn identity() -> Rotor {
        Rotor::scalar(Q::one())
    }

    pub fn blade(mask: usize, v: Q) -> Rotor {
        let mut r = Rotor::zero();
        r.c[mask & 15] = v;
        r
    }

    pub fn coeff(&self, mask: usize) -> &Q {
        &self.c[mask & 15]
    }

    pub fn add(&self, o: &Rotor) -> Rotor {
        Rotor { c: std::array::from_fn(|i| &self.c[i] + &o.c[i]) }
    }

    pub fn scale(&self, s: &Q) -> Rotor {
        Rotor { c: std::array::from_fn(|i| &self.c[i] * s) }
    }

    /// The geometric product, Euclidean metric.
    pub fn geo(&self, o: &Rotor) -> Rotor {
        #[cfg(test)]
        crate::network::nerve::work::count_geo();
        let mut r = Rotor::zero();
        // index: sixteen blade masks (fixed physical size)
        for i in 0..16 {
            if self.c[i].is_zero() {
                continue;
            }
            for j in 0..16 {
                if o.c[j].is_zero() {
                    continue;
                }
                let term = &self.c[i] * &o.c[j];
                let out = i ^ j;
                if reorder_sign(i, j) >= 0 {
                    r.c[out] = &r.c[out] + &term;
                } else {
                    r.c[out] = &r.c[out] - &term;
                }
            }
        }
        r
    }

    /// The reverse `Φ̃`.
    pub fn reverse(&self) -> Rotor {
        Rotor {
            c: std::array::from_fn(|i| {
                let g = (i as u32).count_ones();
                if (g * g.wrapping_sub(1) / 2) & 1 == 0 {
                    self.c[i].clone()
                } else {
                    -&self.c[i]
                }
            }),
        }
    }

    /// `Φ Φ̃` when it is a nonzero scalar — the rotor's own weight; `None`
    /// for anything that is not a versor.
    pub fn weight(&self) -> Option<Q> {
        let p = self.geo(&self.reverse());
        let s = p.c[0].clone();
        if s.is_zero() || (1..16).any(|i| !p.c[i].is_zero()) {
            return None;
        }
        Some(s)
    }

    /// Even-grade: only scalar, bivectors and pseudoscalar. An odd blade
    /// means the element is not a rotor.
    ///
    /// Public since the graded decomposition (`grade`, `grades`) is a
    /// reading callers take: "is this an element of `Cl⁺(4,0)`" is the same
    /// question as "are its occupied grades even", and two readers of one
    /// fact is what law 12 refuses.
    pub fn is_even(&self) -> bool {
        (0..16).all(|i| (i as u32).count_ones() % 2 == 0 || self.c[i].is_zero())
    }

    /// Canonical rotor (A1): divide by |first nonzero|, then negate so that
    /// coefficient is +1. −Φ and Φ are one vertex (the sandwich). Zero is
    /// not a vertex and stays zero.
    pub fn normalised(&self) -> Rotor {
        if !self.is_even() {
            return self.clone();
        }
        let Some(i) = (0..16).find(|&i| !self.c[i].is_zero()) else {
            return Rotor::zero();
        };
        let a = self.c[i].abs();
        let mut r = if a.is_one() {
            self.clone()
        } else {
            self.scale(&(Q::one() / a))
        };
        if r.c[i].is_negative() {
            r = r.scale(&-Q::one());
        }
        r
    }

    /// Coefficient identity: the normal-form constructor's check, never
    /// vertex equality.
    fn coeffs_eq(&self, other: &Rotor) -> bool {
        self.c == other.c
    }

    /// True when `self ∘ other⁻¹` is a nonzero scalar of either sign
    /// (`λ ∈ ℚ×`). The action is `R X R̃ / (R R̃)`, on which −R and R agree.
    pub fn projective_eq(&self, other: &Rotor) -> bool {
        match other.inverse() {
            Some(inv) => {
                let p = self.geo(&inv);
                !p.c[0].is_zero() && (1..16).all(|i| p.c[i].is_zero())
            }
            None => self.normalised().coeffs_eq(&other.normalised()),
        }
    }

    /// Conjugation-and-scale invariants `(a²/w, p²/w, sign p)` — exact
    /// rationals, never a digest. `a` the scalar, `p` the pseudoscalar,
    /// `w = ΦΦ̃`.
    ///
    /// READ OFF THE RAY, NOT OFF A LIFT. Conjugation `Φ ↦ V Φ V⁻¹` fixes
    /// both the scalar and the pseudoscalar, since both are central; scale
    /// by `λ ∈ ℚ×` multiplies each by `λ` and `w` by `λ²`. So `a²/w` and
    /// `p²/w` are invariant under conjugation and under any scale, and the
    /// SIGNS are invariant under conjugation and positive scale only — they
    /// flip under `λ < 0`. [`Rotor::projective_eq`] is this engine's
    /// identity and `−Φ` and `Φ` are one vertex there, because the sandwich
    /// divides the scale out. A sign must therefore be read off a canonical
    /// representative of the ray: negating so the first non-zero coefficient
    /// is positive names exactly one per ray, which is what [`normalised`]
    /// does for [`key`] and [`class_axes`].
    ///
    /// ONE SIGN, NOT TWO. Under that canonicalisation `sign a` is zero for
    /// every rotor — the scalar IS the first non-zero coefficient when it is
    /// non-zero, and is zero otherwise — so the byte is a constant and is
    /// not written. `sign p` is the pseudoscalar's sign relative to that
    /// representative, which does vary and is the chirality of the two
    /// quaternion factors `Φ (1 ± e₀e₁e₂e₃)/2`.
    ///
    /// AN ODD ELEMENT IS REFUSED. [`is_even`]'s rule is that an odd blade
    /// means the element is not a rotor, a rotation class is a class of
    /// rotations, and [`normalised`] returns an odd element untouched — so
    /// reading invariants off one would read its lift's sign. Exact or
    /// refused (rule 11), never a sign that depends on the caller.
    /// 54.7 R4 — THE CONJUGACY INVARIANTS, READ IN `Cl(4,1)` AND NOT
    /// PROJECTED.
    ///
    /// This was the SCALAR AND PSEUDOSCALAR PROJECTION: `a = c[0]`,
    /// `p = c[15]`, answering `(a²/w, p²/w, sign(p))`. It never touched the
    /// bivector — the part that says what the motion IS — so two rotations
    /// in different planes with the same scalar part were one class, and
    /// the "type" byte was the pseudoscalar's sign rather than anything
    /// about the motion. A projection cannot be the primary reading
    /// (MU-RECURSION checklist (m)); it is the defect this repository keeps
    /// re-deriving.
    ///
    /// It now [`Rotor::lift`]s into `Cl(4,1)` and reads
    /// [`crate::transducer::cga::Mv::class_invariants`] there: the two
    /// Casimirs `I₁ = ⟨B²⟩₀/w` and `I₂ = ‖⟨B²⟩₄‖²/w²` of the bivector
    /// generator, and the trichotomy byte from the sign of `I₁`.
    ///
    /// ONE PRODUCER, SO THE STRATA STAY ALIGNED (law 12). `class_key` is
    /// the `Conjugacy` segment of `ClassVector::class_schema` — the
    /// depth-one stratum that `Complex::congruent` reads as a FIBER — and
    /// `holonomy_key` is the same bytes declared as a coordinate. Moving
    /// one of the three and not the others is what desynchronised the
    /// coarse coordinate from the fine one; they are all this function.
    ///
    /// The evenness refusal stays here: an odd element is a reflection, not
    /// a rotor, and has no conjugacy class of this kind (rule 11).
    pub fn class_invariants(&self) -> Option<(Q, Q, u8)> {
        if !self.is_even() {
            return None;
        }
        let w = self.weight()?;
        if w.is_zero() {
            return None;
        }
        self.lift().class_invariants()
    }

    /// Same conjugacy-and-scale class: the invariants agree as rationals.
    /// A non-versor has no class.
    pub fn class_eq(&self, other: &Rotor) -> bool {
        match (self.class_invariants(), other.class_invariants()) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        }
    }

    /// Conjugacy invariants written sequentially as canonical coordinates (Rule 6).
    pub fn put_class_coordinates(&self, out: &mut Vec<u8>) -> Option<()> {
        let (x, y, sp) = self.class_invariants()?;
        put_q(out, &x);
        put_q(out, &y);
        out.push(sp);
        Some(())
    }

    /// Conjugacy invariants written out (A21). Identity of class is
    /// [`class_eq`]. None when the element is not a versor.
    pub fn class_key(&self) -> Option<crate::transducer::address::Address> {
        let mut buf = vec![crate::transducer::address::TAG_CLASS];
        self.put_class_coordinates(&mut buf)?;
        Some(crate::transducer::address::Address::new(buf))
    }

    /// 54.7 R4 — THIS ROTOR AS AN ELEMENT OF `Cl(4,1)(ℚ)`, EXACTLY.
    ///
    /// `Cl(4,0)` is generated by `e₀..e₃`, every one squaring to `+1`, and
    /// `{e₁, e₂, e₃, e₊}` of `Cl(4,1)` are four generators squaring to `+1`
    /// that anticommute. The subalgebra they generate is therefore
    /// isomorphic to `Cl(4,0)`, and since both sides index a blade by the
    /// same 4-bit mask and sort it with the same [`reorder_sign`], the
    /// isomorphism is the IDENTITY ON THE MASK: `metric_sign(i & j)` is
    /// `−1` only where `e₋` is shared, and no mask below bit 4 shares it.
    ///
    /// TOTAL, AND THAT IS WHY IT IS THIS AND NOT A BASE ACTION. The first
    /// cut routed the holonomy class through `conformal_of_rotor`, which
    /// goes through `base_action_of` — a PROJECTION onto the base's own
    /// even subalgebra `⟨1, e₀₁, e₀₃, e₁₃⟩`, four of the eight even blades.
    /// It is partial by design and answers `None` for a rotor carrying `e₂`
    /// onto a base axis — which is exactly the OFF-PLANE class. Every
    /// off-plane loop lost its class, `loop_row` refused, and `Form::rank`
    /// returned `None` for eight suites. A projection cannot be the primary
    /// reading (MU-RECURSION checklist (m)); an embedding can.
    ///
    /// WHAT THE IMAGE CAN SAY. Every bivector here lands in
    /// `span{eᵢ∧eⱼ, eᵢ∧e₊}` — the six SPACELIKE bivectors, all with
    /// `B² < 0`. So a lifted rotor is always elliptic, which is not a
    /// limitation of the lift but the statement that `Cl(4,0)` has no
    /// transvection to lift. The hyperbolic generators `eᵢ∧e₋` are reached
    /// only when a motion is BUILT in `Cl(4,1)`, which is what replacing
    /// `Rotor` at its source does.
    pub fn lift(&self) -> crate::transducer::cga::Mv {
        // index: the sixteen blade masks of Cl(4,0), each carried to the
        // identically-masked blade of Cl(4,1) (fixed physical size).
        (0..16).fold(crate::transducer::cga::Mv::zero(), |m, i| {
            m.add(&crate::transducer::cga::Mv::blade(i, self.c[i].clone()))
        })
    }

    /// 54.7 R4 — THE HOLONOMY CLASS, RE-ANCHORED TO `Cl(4,1)`.
    ///
    /// The accessor's type is unchanged and every carrier that takes it is
    /// untouched; what moved is the PRODUCER underneath. It read
    /// [`Rotor::class_key`] — the conjugacy coordinates of a `Cl(4,0)`
    /// rotor, where `⟨B²⟩₀ ≤ 0` for every bivector and every motion is
    /// therefore elliptic. A class read there cannot tell a rotation from a
    /// transvection because `Cl(4,0)` has no transvection to tell it from:
    /// the six bivectors `eᵢ∧eⱼ` of `⋀²ℚ⁴` are all spacelike.
    ///
    /// It is now [`Rotor::class_key`]'s own bytes declared as the
    /// coordinate, and those bytes are read in `Cl(4,1)` because
    /// [`Rotor::class_invariants`] is. One producer feeds the conjugacy
    /// segment, the transport class and this, so the coarse stratum stays
    /// a byte prefix of the fine one and `Complex::congruent`'s fiber read
    /// keeps working.
    ///
    /// §10 — IT IS A LAYOUT CHANGE. Every filed loop, opened axis and
    /// class vector is keyed by these bytes, so the corpus on disk is
    /// keyed by the old producer until a recut.
    pub fn holonomy_key(
        &self,
    ) -> Option<crate::network::nerve::Key<crate::network::nerve::coord::Holonomy>> {
        crate::network::nerve::Key::from_address(&self.class_key()?).ok()
    }

    /// The exact inverse `Φ̃ / (Φ Φ̃)`.
    pub fn inverse(&self) -> Option<Rotor> {
        let w = self.weight()?;
        Some(self.reverse().scale(&(Q::one() / w)))
    }

    /// The sandwich on a vector of the chart, scale divided out exactly:
    /// `Φ v Φ̃ / (Φ Φ̃)`. `None` when the result is not a pure vector (the
    /// element was not a versor).
    pub fn carry(&self, v: &[Q; 4]) -> Option<[Q; 4]> {
        let w = self.weight()?;
        let mut x = Rotor::zero();
        for (k, e) in v.iter().enumerate() {
            x.c[1 << k] = e.clone();
        }
        let out = self.geo(&x).geo(&self.reverse());
        if (0..16).any(|i| (i as u32).count_ones() != 1 && !out.c[i].is_zero()) {
            return None;
        }
        Some(std::array::from_fn(|k| &out.c[1 << k] / &w))
    }

    /// The identity of the RAY (1.5): a non-zero scalar, `λ · 1` for any
    /// `λ ∈ ℚ×`, which is `projective_eq(&Rotor::identity())` read
    /// without the inverse. `Φ Φ̃` is the weight and not the identity's
    /// coefficients, and a reader of the coefficients disagreed with `==`
    /// on it — two readers of one fact (law 12).
    pub fn is_identity(&self) -> bool {
        !self.c[0].is_zero() && (1..16).all(|i| self.c[i].is_zero())
    }

    /// EXPANSION 3 — THE GRADED DECOMPOSITION `W = ⟨W⟩₀ + ⟨W⟩₂ + ⟨W⟩₄`.
    ///
    /// A blade's grade is the population count of its mask: `Cl(4,0)` has
    /// one scalar (mask 0), four vectors, six bivectors, four trivectors and
    /// one pseudoscalar (mask 15). The EVEN subalgebra `Cl⁺(4,0)` is grades
    /// 0, 2 and 4 — eight coefficients — and that is what a rotor is.
    ///
    /// The grades are not four separate carriers to be summed. They are the
    /// projection of ONE multivector, and the whole point of holding it as a
    /// multivector is that a single geometric product composes all of them at
    /// once: a turn, a gauge transition and a dilation compose in one `geo`
    /// rather than in three lanes that must be re-mixed afterwards.
    ///
    /// What each grade carries, on this algebra:
    /// - `⟨W⟩₀` — the scalar weight. Identity is a pure grade 0.
    /// - `⟨W⟩₂` — the six bivectors. Every rotation lives here, one
    ///   coefficient per plane, and `Spin(4) = SU(2) × SU(2)` is exactly the
    ///   statement that `e₀∧e₁` and `e₂∧e₃` turn independently.
    /// - `⟨W⟩₄` — the pseudoscalar `e₀e₁e₂e₃`, the 4-volume, which a
    ///   double rotation carries and a single-plane turn does not.
    ///
    /// A reading, never a gate: it says which grades carry content, and the
    /// caller decides what that means.
    pub fn grade(&self, k: u32) -> Rotor {
        let mut out = Rotor::zero();
        for m in 0..16usize {
            if (m as u32).count_ones() == k {
                out.c[m] = self.c[m].clone();
            }
        }
        out
    }

    /// The grades this rotor actually occupies, ascending. Empty is the
    /// zero multivector, which is not a versor and carries nothing.
    pub fn grades(&self) -> Vec<u32> {
        let mut out = Vec::new();
        for k in 0..=4u32 {
            if (0..16usize).any(|m| (m as u32).count_ones() == k && !self.c[m].is_zero()) {
                out.push(k);
            }
        }
        out
    }

    /// THE PSEUDOSCALAR COEFFICIENT, `⟨W⟩₄` as a rational. A double rotation
    /// of `Spin(4)` carries it and a single-plane turn does not, so it is the
    /// exact reading that separates the two — and it is the coefficient
    /// `class_invariants` calls `p`.
    pub fn pseudoscalar(&self) -> &Q {
        &self.c[15]
    }

    /// THE BLADES THIS ROTOR CARRIES OUTSIDE ONE PLANE — the masks with a
    /// non-zero coefficient that are neither the scalar nor `plane`'s own
    /// bivector. Empty means the rotor lives wholly in the subalgebra
    /// `⟨1, e_ij⟩` and a single 2×2 block on that plane is the whole of it.
    ///
    /// Spin(4) is SU(2) × SU(2): a rotation of a rank-4 chart is TWO
    /// independent commuting rotations, in `e₀∧e₁` and in `e₂∧e₃`, with
    /// free and unrelated angles. A reader that takes one plane's block and
    /// calls it the rotor reports a partial rotation as the whole
    /// transition, and it loses the second angle exactly — including the
    /// case that matters, where the two turn in OPPOSITE senses and the
    /// pair's order is `lcm` of the two, an order neither plane has.
    ///
    /// This is a reading and never a gate (§2.3): it says which blades
    /// carry content, and the caller decides what that means.
    pub fn off_plane_blades(&self, plane: (u8, u8)) -> Vec<usize> {
        let keep = (1usize << plane.0) | (1usize << plane.1);
        (1..16)
            .filter(|&m| m != keep && !self.c[m].is_zero())
            .collect()
    }

    /// `by Φ by⁻¹`. A non-invertible conjugator leaves Φ unmoved.
    pub fn conjugate(&self, by: &Rotor) -> Rotor {
        match by.inverse() {
            Some(inv) => by.geo(self).geo(&inv),
            None => self.clone(),
        }
    }

    /// A1 key: the canonical coefficients written out (A21). Identity is
    /// [`projective_eq`].
    pub fn key(&self) -> crate::transducer::address::Address {
        let mut buf = vec![crate::transducer::address::TAG_ROTOR];
        self.put_canonical(&mut buf);
        crate::transducer::address::Address::new(buf)
    }

    /// Canonical coefficients written out sequentially as coordinates (Rule 6).
    pub fn put_canonical(&self, out: &mut Vec<u8>) {
        self.normalised().to_bytes(out);
    }

    pub fn from_key(k: &[u8]) -> Option<Rotor> {
        if crate::transducer::address::tag_of(k) != Some(crate::transducer::address::TAG_ROTOR) {
            return None;
        }
        let mut p = 1usize;
        Rotor::from_bytes(k, &mut p)
    }

    // `class_axes` stood here: the conjugacy invariants packed into
    // `Vec<u64>` through `pack_axis`, two limbs per rational. A limb is
    // not a coordinate — `peel` could cut between the halves of one
    // invariant — so the reading moved to `stalk::rotor_axes`, which
    // writes each invariant as one `ChartAxis` carrying its own exact
    // value (A21 §2 as ruled 2026-09-18).

    pub fn to_bytes(&self, out: &mut Vec<u8>) {
        for c in &self.c {
            put_q(out, c);
        }
    }

    pub fn from_bytes(b: &[u8], pos: &mut usize) -> Option<Rotor> {
        let mut c: [Q; 16] = std::array::from_fn(|_| Q::zero());
        for slot in &mut c {
            *slot = take_q(b, pos)?;
        }
        Some(Rotor { c })
    }

    /// The plane word this rotor spells. None when it does not — never identity.
    ///
    /// THE IMAGES MUST STAY IN THE SENSOR PLANE, and this did not check. It
    /// carried e₀ and e₁, took each image's FIRST TWO components into a 2×2
    /// block, and spelled that — so a rotor whose images leave the plane had
    /// its shadow on the plane spelled as though it were the whole motion.
    /// Two readings of the engine, not of the text: a quarter turn on plane
    /// (2,3) carries e₀ and e₁ unmoved, projects to the identity block, and
    /// returned the IDENTITY WORD against this doc line; and a legal
    /// two-letter word whose holonomy lives in (0,2)∧(1,3) returned a half
    /// turn on the sensor plane.
    ///
    /// The consequence is not local. `ClassVector::of_rotor` writes an
    /// occupied plane section from this, so a holonomy with no sensor-plane
    /// content reads as having one and `occupies(Axis::OffPlane)` reads
    /// false — and law 10's pivot takes its candidate moves off exactly
    /// those axes. A rotor that moves the plane out of itself has no plane
    /// word, which is what `None` here has always meant.
    pub fn spelled_word(&self) -> Option<Word> {
        let one = Q::from_integer(BigInt::from(1));
        let zero = Q::from_integer(BigInt::from(0));
        let e0 = self.carry(&[one.clone(), zero.clone(), zero.clone(), zero.clone()]);
        let e1 = self.carry(&[zero.clone(), one, zero.clone(), zero]);
        let (a, b) = (e0?, e1?);
        // The two directions the sensor plane is NOT. A component there is
        // the image leaving the plane, and no 2×2 block over the plane can
        // carry it — the block would be a projection, and a projection of a
        // motion is not that motion.
        if !a[2].is_zero() || !a[3].is_zero() || !b[2].is_zero() || !b[3].is_zero() {
            return None;
        }
        let block = [[a[0].clone(), b[0].clone()], [a[1].clone(), b[1].clone()]];
        Some(spell(&block)?.word)
    }

    /// THE TWO PERIODS OF A DOUBLE ROTATION, UNCOLLAPSED (54.5, as ruled
    /// 2026-09-20).
    ///
    /// A rotation in four dimensions is not a turn about an axis: the even
    /// subalgebra splits by the central idempotents `(1 ± I)/2` into two
    /// COMMUTING simple rotations, on two orthogonal planes, each with its
    /// own period. [`Self::order`] computes both exactly and then returns
    /// `lcm(m₁, m₂)` — one number for two independent quantities — and
    /// answers `Infinite` when EITHER factor is not a root of unity, which
    /// discards a compact half.
    ///
    /// That collapse is law 5's shape one level below the lanes: two
    /// coordinates summed into one, after which no reader can tell a
    /// `ℤ/6ℤ` on one plane from a `ℤ/2ℤ` and a `ℤ/3ℤ` on two. The
    /// projective order is a true and useful reading — `order` keeps it —
    /// but it is a reading OF this pair, and a spectrum needs the pair.
    ///
    /// Each factor turns by `2θ = 2kπ/m`, so `R^n` is real for that factor
    /// exactly when `n = m / gcd(k, m)`, which is its period. `None` for a
    /// non-versor: no weight, no class, no periods.
    pub fn plane_periods(&self) -> Option<[Order; 2]> {
        let w = self.weight()?;
        let (a, p) = (&self.c[0], &self.c[15]);
        let period = |s: Q| -> Order {
            match half_angle(&s, &w) {
                Some((k, m)) => Order::Finite(m / gcd_u32(k, m).max(1)),
                None => Order::Infinite,
            }
        };
        Some([period(a + p), period(a - p)])
    }

    /// THE ORDER OF A ROTOR, read from its class (15.3, ruled 2026-09-09).
    /// The even subalgebra splits by the central idempotents `(1 ± I)/2`
    /// into two commuting quaternion factors of one weight `w`, with
    /// scalar parts `a ± p`; each factor's half-angle has
    /// `cos² = (a ± p)²/w`, an exact rational, a finite period exactly at
    /// `{1, 3/4, 1/2, 1/4, 0}` (Niven), and its angle `kπ/m` is read off
    /// the sign. `R^n` is a scalar iff both factors' n-th powers are real
    /// and equal, so the projective order — the order of the rotation `R`
    /// induces — is the lcm of the two periods, or twice it when the two
    /// lcm-th powers are opposite scalars. `None` for a non-versor: no
    /// weight, no class, and no scalar power at all. Never a spelling, and
    /// never the sensor plane's shadow.
    /// THE TWO PLANE PERIODS AS THE SPECTRUM THEY ARE (54.7).
    ///
    /// [`Rotor::order`] returns `lcm(ℓ₁, ℓ₂)` and that is an envelope, not a
    /// period: a planar rotation `[ℓ, 1]` and an isoclinic Clifford
    /// translation `[ℓ, ℓ]` give one `lcm` and land in one class though
    /// their stabilisers and fixed-point sets differ; an incommensurate
    /// pair never closes and an integer envelope reports it as closed; and
    /// the beat `|ℓ₁ − ℓ₂|⁻¹` is thrown away, so no reader can compare two
    /// loops' harmonic ratio.
    ///
    /// `None` on a plane is that factor turning forever.
    pub fn toral_period(&self) -> crate::transducer::sentence::ToralPeriod {
        use crate::transducer::sentence::ToralPeriod;
        let of = |o: Order| match o {
            Order::Finite(n) => std::num::NonZeroU32::new(n),
            Order::Infinite => None,
        };
        match self.plane_periods() {
            Some([a, b]) => ToralPeriod { plane_1: of(a), plane_2: of(b) },
            None => ToralPeriod { plane_1: None, plane_2: None },
        }
    }

    pub fn order(&self) -> Option<Order> {
        let w = self.weight()?;
        let (a, p) = (&self.c[0], &self.c[15]);
        let (Some((k1, m1)), Some((k2, m2))) = (half_angle(&(a + p), &w), half_angle(&(a - p), &w)) else {
            return Some(Order::Infinite);
        };
        let l = m1 / gcd_u32(m1, m2) * m2;
        let same = (l / m1 * k1) % 2 == (l / m2 * k2) % 2;
        Some(Order::Finite(if same { l } else { 2 * l }))
    }
}

/// The half-angle of one quaternion factor as `(k, m)` with `θ = kπ/m`,
/// from its scalar part `s` and the weight `w`: `cos² θ = s²/w`. The five
/// rationals are the only ones a root of unity has (Niven), and a
/// negative cosine is the supplement. `None`: not a root of unity, the
/// factor turns forever.
fn half_angle(s: &Q, w: &Q) -> Option<(u32, u32)> {
    let c = &(s * s) / w;
    let (k, m) = if c.is_one() {
        (0, 1)
    } else if c == qn(3, 4) {
        (1, 6)
    } else if c == qn(1, 2) {
        (1, 4)
    } else if c == qn(1, 4) {
        (1, 3)
    } else if c.is_zero() {
        (1, 2)
    } else {
        return None;
    };
    Some(if s.is_negative() { (m - k, m) } else { (k, m) })
}

/// The rotor's action on the pair plane `(i, j)` as a 2×2 block, columns
/// the images of `e_i` and `e_j`; `None` when the rotor is not a versor,
/// the plane is not a pair, or the images leave the plane (50.4).
pub fn rotor_block_on(r: &Rotor, plane: (u8, u8)) -> Option<[[Q; 2]; 2]> {
    let (i, j) = (plane.0 as usize, plane.1 as usize);
    if i >= 4 || j >= 4 || i == j {
        return None;
    }
    let unit = |k: usize| -> [Q; 4] { std::array::from_fn(|t| if t == k { Q::one() } else { Q::zero() }) };
    let a = r.carry(&unit(i))?;
    let b = r.carry(&unit(j))?;
    let closed = (0..4).filter(|&t| t != i && t != j).all(|t| a[t].is_zero() && b[t].is_zero());
    if !closed {
        return None;
    }
    Some([[a[i].clone(), b[i].clone()], [a[j].clone(), b[j].clone()]])
}

pub fn put_q(out: &mut Vec<u8>, v: &Q) {
    let nb = v.numer().to_signed_bytes_le();
    let db = v.denom().to_signed_bytes_le();
    out.extend_from_slice(&(nb.len() as u32).to_le_bytes());
    out.extend_from_slice(&nb);
    out.extend_from_slice(&(db.len() as u32).to_le_bytes());
    out.extend_from_slice(&db);
}

pub fn take_q(b: &[u8], pos: &mut usize) -> Option<Q> {
    let take_int = |pos: &mut usize| -> Option<BigInt> {
        let n = u32::from_le_bytes(b.get(*pos..*pos + 4)?.try_into().ok()?) as usize;
        *pos += 4;
        let bytes = b.get(*pos..*pos + n)?;
        *pos += n;
        Some(BigInt::from_signed_bytes_le(bytes))
    };
    let num = take_int(pos)?;
    let den = take_int(pos)?;
    if den.is_zero() {
        return None;
    }
    Some(Q::new(num, den))
}

pub(crate) fn rational_sqrt(x: &Q) -> Option<Q> {
    if x.is_negative() {
        return None;
    }
    let (n, d) = (x.numer().sqrt(), x.denom().sqrt());
    if &(&n * &n) == x.numer() && &(&d * &d) == x.denom() {
        Some(Q::new(n, d))
    } else {
        None
    }
}
