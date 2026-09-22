// 54.7 — THE LOOP CENSUS: the charge each step carries, how closing cycles
// divide, the defect and spectrum of each kept cycle, and the operators its
// generators enact.

use super::*;

thread_local! {
    /// HOW THE CLOSING CYCLES DIVIDE — `(vacuum, resonant, the charges the
    /// resonant ones carried)` (54.7, ruled 2026-09-21).
    ///
    /// `gate(3)` counts every cycle whose boundary product is the
    /// identity, and two opposite facts land in that one number: a cycle
    /// over a subgraph where the corpus wrote no transport, and a cycle
    /// whose non-zero shears cancel around the perimeter. The first
    /// quotients nothing; the second is a relator of `π₁`.
    ///
    /// OBSERVATIONAL ONLY. `loop_is_trivial` keeps pruning exactly what it
    /// pruned before, so `loops.len()` and the tests asserting incidence
    /// through it are untouched. Both kinds genuinely belong to `ker(ρ)` —
    /// neither deposits net holonomic shear at `x₀` — so the gate is right
    /// and only the reporting was blind.
    ///
    /// The charges are kept whole rather than summed: the distribution is
    /// the content and a total cannot be un-added (rule 2).
    /// The last two count EVERY step of EVERY cycle, closing or not:
    /// `(steps whose charge was read, of those, steps carrying μ ≠ 0)`.
    /// Without them "all closures are vacuum" cannot be distinguished from
    /// "the charge was never computed" — a probe that never fired reporting
    /// zero as though it had measured one (rule 11).
    static RESONANCE: std::cell::RefCell<(u64, u64, Vec<num_rational::BigRational>, u64, u64)> =
        const { std::cell::RefCell::new((0, 0, Vec::new(), 0, 0)) };
}

/// THE KINEMATIC CHARGE A TRANSPORT CARRIES — a PROJECTION of the versor,
/// computed where it is read and never stored beside it (LAW-0).
///
/// ```text
///     μ = ⟨Ω⟩₀² / ⟨Ω Ω̃⟩₀ − 1
/// ```
///
/// Scale-free, so `Ω` and `λΩ` — one transport, since a versor is
/// projective — give one charge. `None` where the argument is not a
/// versor, which is a refusal and not a zero charge (rule 11).
pub fn step_charge(omega: &element::Versor) -> Option<num_rational::BigRational> {
    let w = crate::transducer::cga::is_versor(omega)?;
    let s = omega.scalar_part();
    let one = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1));
    Some(&s * &s / &w - one)
}

/// File one STEP's charge, on every cycle the walk reaches.
pub(super) fn note_step_charge(mu: &num_rational::BigRational) {
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    RESONANCE.with(|r| {
        let mut b = r.borrow_mut();
        b.3 += 1;
        if *mu != zero {
            b.4 += 1;
        }
    });
}

/// File one CLOSING cycle by whether it carried any charge at all.
pub(super) fn note_closure(carried: &Keyed<PathCarried>) {
    // The charge is taken HERE, off the stored versors, rather than read
    // back from a scalar written beside them (LAW-0, law 12: one producer).
    let mut charges: Vec<num_rational::BigRational> = Vec::new();
    for (_, c) in carried.scoped() {
        charges.extend(c.steps.iter().filter_map(step_charge));
    }
    let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
    let charged: Vec<num_rational::BigRational> =
        charges.into_iter().filter(|m| *m != zero).collect();
    RESONANCE.with(|r| {
        let mut b = r.borrow_mut();
        match charged.is_empty() {
            true => b.0 += 1,
            false => {
                b.1 += 1;
                b.2.extend(charged);
            }
        }
    });
}

/// `(vacuum closures, resonant closures, the resonant charges)`.
pub fn loop_resonance() -> (u64, u64, Vec<num_rational::BigRational>, u64, u64) {
    RESONANCE.with(|r| r.borrow().clone())
}

pub fn reset_loop_resonance() {
    RESONANCE.with(|r| *r.borrow_mut() = (0, 0, Vec::new(), 0, 0));
}

thread_local! {
    /// THE DEFECT EACH KEPT CYCLE CARRIES, read off its transport
    /// composite: `(trivial, elliptic, hyperbolic, parabolic, non-simple)`.
    ///
    /// A cycle's class is not a label the builder assigns; it is
    /// `class_invariants` of the versor the loop's own faces compose, so
    /// the distribution is a reading of the corpus and not of this code.
    static DEFECTS: std::cell::Cell<(u64, u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0, 0)) };

    /// THE TORAL SPECTRUM OF EACH KEPT CYCLE:
    /// `(closes, open, undecomposed over ℚ, isoclinic)`.
    ///
    /// A toral period is a COMPACT invariant — the order of a rotation. A
    /// pure transvection generates a non-compact one-parameter subgroup and
    /// its geodesic flow has no returning orbit, so it has no finite period
    /// at all: it lands on the projective boundary `[1:0]` and charges
    /// shear rather than sounding a pitch.
    ///
    /// FOUR COUNTERS, NOT TWO. An orbit that does not close and a split
    /// that does not EXIST are different facts about different things, and
    /// folding them together reported 545 "open" when the truth was 541
    /// open and 4 undecomposed (rule 11).
    pub(super) static SPECTRA: std::cell::Cell<(u64, u64, u64, u64)> =
        const { std::cell::Cell::new((0, 0, 0, 0)) };
}

thread_local! {
    /// `ker(ρ)` ON THE CYCLE BASIS — the based generators, counted by the
    /// OPERATOR each enacts (54.7, ruled 2026-09-21).
    ///
    /// Now that Axiom A puts every cycle in the basepoint's frame, two
    /// generators can be compared: `ρ(γ_i) ~ ±ρ(γ_j)` means two distinct
    /// cellular paths enact the identical physical dislocation — literal
    /// synonyms, which 13.10 predicts ("one theorem has as many proofs as
    /// the form has synonyms of one class") and which nothing has ever
    /// counted.
    ///
    /// `trivial` counts `ρ(γ) ~ ±1`: a chord whose transport the spanning
    /// tree's return path cancelled exactly. Such a generator is in
    /// `ker(ρ)` outright — a real cycle of `π₁` carrying no geometry.
    ///
    /// COUNTED BY A KEYED READ, never a pairwise walk (rule 3): the
    /// operator's own canonical bytes are its coordinate, so uniqueness is
    /// a trie insert and `|im(ρ)|` is the carrier's length.
    /// THE QUOTIENT TOWER, MEASURED AT EACH LEVEL (54.7, ruled
    /// 2026-09-21):
    ///
    /// ```text
    ///   words in π₁(X¹) ↠ π₁(X, x₀) ↠ im(ρ) ↠ Conj(im ρ)
    ///        level 0          level 1      level 2
    /// ```
    ///
    /// The two axioms project onto ORTHOGONAL factors and neither sees the
    /// other's level:
    ///
    ///   AXIOM B is spatial — it fixes `(I₁, I₂, kind)`, the eigenvalues
    ///   and plane signatures, and is completely blind to which tree or
    ///   basepoint was chosen. It determines LEVEL 2.
    ///
    ///   AXIOM A is compositional — it orients the operator inside the
    ///   shared tangent space at `x₀`, acting along the fibre of level 1
    ///   over level 2. Re-rooting applies one inner automorphism `Ad_Θ`,
    ///   which moves the level-1 representative and leaves its level-2
    ///   projection fixed.
    ///
    /// `Ad_θ` is therefore a CONGRUENCE: an equivalence compatible with
    /// the group operation, and the reason basing had to move no reading.
    ///
    /// Held: `(operators by exact projective equality, classes by Casimir,
    /// generators seen, generators with ρ ~ ±1)`.
    static SYNONYMS: std::cell::RefCell<(Keyed<u64>, Keyed<u64>, u64, u64)> =
        std::cell::RefCell::new((Keyed::new(), Keyed::new(), 0, 0));

    /// THE OPERATORS OF EACH CASIMIR CLASS — level 1 grouped by level 2.
    ///
    /// The antipodal hypothesis needs the operators WITHIN a class, not
    /// just how many there are: if a class holds exactly two and they are
    /// mutually inverse, the pair is one undirected geodesic channel
    /// traversed both ways, and the corpus vocabulary is a set of
    /// bidirectional dislocation lines rather than scattered directions.
    ///
    /// Keyed by the class so the question is a keyed read (rule 3); the
    /// value carries each distinct operator's own bytes so the product can
    /// be re-derived and tested rather than assumed.
    static CLASS_OPS: std::cell::RefCell<Keyed<Vec<Vec<u8>>>> =
        std::cell::RefCell::new(Keyed::new());
}

/// THE ANTIPODAL TEST, run over the classes (54.7).
///
/// `(operators in the class, whether the class is exactly a mutually
/// inverse pair)` for every Casimir class, in the carrier's own key order.
/// A class of two whose product is projectively the identity is one
/// geodesic line read forward and backward.
pub fn loop_class_report() -> Vec<(usize, bool)> {
    CLASS_OPS.with(|c| {
        c.borrow()
            .scoped()
            .map(|(_, ops)| {
                let inverse = match ops.as_slice() {
                    [a, b] => {
                        let decode = |v: &Vec<u8>| {
                            let mut p = 0usize;
                            crate::transducer::cga::Mv::from_bytes(v, &mut p)
                        };
                        match (decode(a), decode(b)) {
                            (Some(x), Some(y)) => {
                                x.geo(&y).projective_eq(&element::identity_versor())
                            }
                            _ => false,
                        }
                    }
                    _ => false,
                };
                (ops.len(), inverse)
            })
            .collect()
    })
}

pub fn reset_loop_class_ops() {
    CLASS_OPS.with(|c| *c.borrow_mut() = Keyed::new());
}

/// THE OPERATORS OF ONE FORM — the univalence test.
///
/// `|im(ρ_F)|` over this form's own cycle basis. A form with one operator
/// is an elementary token: however many internal cycles it carries, they
/// are that many geometric proofs of ONE theorem. A form with several is
/// compound — an unresolved sentence rather than a letter — and the
/// distinction between alphabet and grammar depends on which it is.
///
/// Computed where it is reported, from the form's own loops, so there is
/// no cumulative state to reset and no way for one form's reading to leak
/// into another's.
pub fn form_operators(f: &Form) -> usize {
    let mut ops: Keyed<()> = Keyed::new();
    for lp in &f.loops {
        let Some(inv) = lp.based_at.versor_inverse() else { continue };
        for (_, omega) in lp.carriages.scoped() {
            let based = lp.based_at.geo(omega).geo(&inv);
            let Some(canonical) = based.canonical() else { continue };
            let mut bytes = Vec::new();
            canonical.to_bytes(&mut bytes);
            ops.insert(&bytes, ());
        }
    }
    ops.len()
}

/// `(|im(ρ)|, |Conj|, generators seen, generators with ρ ~ ±1)`.
///
/// `b₁ − |im(ρ)|` is the semantic compression: distinct cellular paths
/// enacting one physical displacement. `|im(ρ)| / |Conj|` is the
/// directional dispersion — how many distinct Burgers vectors the corpus
/// spans within each kinematic class.
pub fn loop_synonyms() -> (u64, u64, u64, u64) {
    SYNONYMS.with(|s| {
        let b = s.borrow();
        (b.0.len() as u64, b.1.len() as u64, b.2, b.3)
    })
}

pub fn reset_loop_synonyms() {
    SYNONYMS.with(|s| *s.borrow_mut() = (Keyed::new(), Keyed::new(), 0, 0));
}

/// File one BASED generator by the operator it enacts.
pub(super) fn note_synonym(based: &element::Versor) {
    let Some(canonical) = based.canonical() else { return };
    let mut bytes = Vec::new();
    canonical.put_canonical(&mut bytes);
    let trivial = based.projective_eq(&element::identity_versor());
    // LEVEL 2 — the conjugacy class, which Axiom A cannot move. A
    // generator whose class cannot be read is counted at level 1 and not
    // at level 2, rather than filed under a class it does not have.
    let class = based.class_key().map(|a| a.as_bytes().to_vec());
    SYNONYMS.with(|s| {
        let mut b = s.borrow_mut();
        b.2 += 1;
        if trivial {
            b.3 += 1;
        }
        let seen = b.0.get(&bytes).copied().unwrap_or(0);
        b.0.insert(&bytes, seen + 1);
        if let Some(c) = class.clone() {
            let seen = b.1.get(&c).copied().unwrap_or(0);
            b.1.insert(&c, seen + 1);
        }
    });
    // AND THE OPERATOR FILED UNDER ITS CLASS, so the antipodal test can ask
    // which operators share a kinematic scale.
    let Some(c) = class else { return };
    let mut wire = Vec::new();
    canonical.to_bytes(&mut wire);
    CLASS_OPS.with(|m| {
        let mut m = m.borrow_mut();
        match m.get_mut(&c) {
            Some(held) => {
                if !held.contains(&wire) {
                    held.push(wire);
                }
            }
            None => {
                m.insert(&c, vec![wire]);
            }
        }
    });
}

/// `(closes, open, undecomposed over ℚ, isoclinic)`.
pub fn loop_spectra() -> (u64, u64, u64, u64) {
    SPECTRA.with(|c| c.get())
}

pub fn reset_loop_spectra() {
    SPECTRA.with(|c| c.set((0, 0, 0, 0)));
}

/// The defect distribution of every cycle kept since the last reset.
pub fn loop_defects() -> (u64, u64, u64, u64, u64) {
    DEFECTS.with(|c| c.get())
}

pub fn reset_loop_defects() {
    DEFECTS.with(|c| c.set((0, 0, 0, 0, 0)));
}

/// Classify one kept cycle by THE WHOLE TRANSPORT IT COMPOSES.
///
/// A first cut read `PathCarried::face.carriage` alone and every one of 545
/// cycles came back Trivial — while `loop_is_trivial` had KEPT them, which
/// can only happen when something non-flat is present. The chord's
/// dislocation rides in 50.3's EDGE transport, not in the member carriage:
/// two sections under one carriage have `φ_b φ_a⁻¹ = 1` by construction,
/// so the carriage is exactly where the Burgers vector is not.
///
/// The cycle's holonomy is the composite of both, which is what
/// `Ω_∂e² = 𝒫 ∏ Ω_e` means.
pub(super) fn note_defect(carried: &PathCarried) {
    use crate::network::nerve::coord::BivectorKind;
    let composite = match &carried.edge {
        Some(e) => e.carriage.geo(&carried.face.carriage),
        None => carried.face.carriage.clone(),
    };
    let Some((_, _, kind)) = composite.class_invariants() else { return };
    DEFECTS.with(|c| {
        let mut t = c.get();
        match kind {
            BivectorKind::Trivial => t.0 += 1,
            BivectorKind::Elliptic => t.1 += 1,
            BivectorKind::Hyperbolic => t.2 += 1,
            BivectorKind::Parabolic => t.3 += 1,
        }
        // A DISPIRATION IS NOT A FOURTH TYPE, it is a coupling: the
        // generator is non-simple, so the cycle turns and slips at once.
        // Counted beside the type rather than instead of it.
        if !crate::transducer::cga::bivector_is_simple(&composite) {
            t.4 += 1;
        }
        c.set(t);
    });
}
