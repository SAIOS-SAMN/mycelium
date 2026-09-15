// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! EXPANSION 2, 1.8, 15.3 — THE WORD AS A BUNDLE: `π : Word → B × F`, the
//! base generator it turns by and the fibre coordinate it turns at.
//!
//! A word observed twice in two contexts — the same quarter turn recolouring
//! blue here and green there — is ONE base and TWO sections, not two words.
//! Held as two monoliths the vocabulary grows with the contexts; held as a
//! bundle it grows with the BASES, and the contexts become coordinates on a
//! fibre over one of them.
//!
//! THE BASE IS THE CONJUGACY CLASS, and that is forced, not chosen. A gauge
//! transition acts by conjugation — `W ↦ γWγ⁻¹` — and the conjugacy
//! invariants `(a²/w, p²/w, sign p)` are exactly what conjugation preserves
//! (15.3). So two words related by ANY change of basepoint share a base by
//! construction, and the fibre is what the change of basepoint moved. There
//! is no freedom in where to cut: the invariants are the cut.
//!
//! WHAT THE FIBRE CARRIES is the rest, per lane and never summed (law 5):
//! the conjugator `γ` on `𝒢`, the placement and dilation of the carriage on
//! `𝒮`, the hue permutation on `𝒢`'s cover. The base carries `𝒽`.

use super::{Rotor, Word};
use crate::transducer::element::{self, Versor};

/// THE BASE of a word: the conjugacy class its rotor stands in.
///
/// Carried as the class key — the canonical 12.6 coordinate order, from
/// which `ClassVector::from_key` reads the vector back — so it is content
/// and reversible, never a digest of the rotor (law 2).
///
/// `None` where the rotor is not a versor: a non-versor has no conjugacy
/// class (15.3), so it is in no bundle, which is a refusal rather than a
/// base of zeros (rule 11).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base(pub crate::transducer::address::Address);

/// THE FIBRE COORDINATE: what a change of basepoint moved, lane by lane.
///
/// Four independent covers and never their sum (law 5). A section that
/// recolours AND translates is a coordinate on two of them, and a reduction
/// on one is not entitled to the other.
#[derive(Clone, Debug)]
pub struct Fibre {
    /// `𝒢` — the conjugator carrying the base representative to this word,
    /// where one exists in the alphabet. `None` is not "the identity": it
    /// is "no conjugator was found", and the two are different readings.
    pub gauge: Option<Word>,
    /// `𝒮` — the placement and dilation this section stands at.
    pub carriage: Versor,
    /// `𝒢`'s cover — the hue permutation the section carries.
    pub hue: Vec<(u8, u8)>,
}

/// A word projected onto its bundle: `π(W) = (B, F)`.
#[derive(Clone, Debug)]
pub struct Section {
    pub base: Base,
    pub fibre: Fibre,
}

impl Base {
    /// The base of a rotor. Two rotors share this exactly when they are
    /// conjugate, which is what makes it a base.
    pub fn of_rotor(r: &Rotor) -> Option<Base> {
        r.class_key().map(Base)
    }

    pub fn of_word(w: &Word) -> Option<Base> {
        Base::of_rotor(&w.rotor())
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

/// THE PROJECTION `π : Word → B × F`.
///
/// The base is read off the rotor's conjugacy invariants; the fibre takes
/// the carriage and hue the caller measured the word acting by, and the
/// conjugator is SOLVED rather than stored — see [`solve_conjugator`].
pub fn project(w: &Word, carriage: &Versor, hue: &[(u8, u8)], base_rep: Option<&Word>) -> Option<Section> {
    let base = Base::of_word(w)?;
    let gauge = base_rep.and_then(|rep| solve_conjugator(rep, w));
    Some(Section {
        base,
        fibre: Fibre {
            gauge,
            carriage: carriage.clone(),
            hue: hue.to_vec(),
        },
    })
}

/// SOLVE `γ` WITH `γ·a·γ⁻¹ = b`, over the seventeen.
///
/// Conjugacy is necessary and not sufficient: two rotors of one class are
/// related by SOME element of the group, and whether the alphabet spells
/// that element is a separate question. This answers the second — a
/// conjugator the language can write — and refuses otherwise rather than
/// returning an element nobody can say (37.7's condition, read from this
/// side).
///
/// It is not a search over the group. The candidates are the seventeen
/// letters and their inverses — the alphabet's own generators, which is a
/// fixed reading of a stated table and not an enumeration of a carrier —
/// and each is verified by the exact conjugation. A conjugator needing more
/// than one letter is not found here and is refused, which is the honest
/// bound: an unbounded word search is what law 3 forbids.
pub fn solve_conjugator(a: &Word, b: &Word) -> Option<Word> {
    let (ra, rb) = (a.rotor(), b.rotor());
    if ra.projective_eq(&rb) {
        return Some(Word::identity());
    }
    // NO CLASS PRECONDITION, and the first cut of this had one. `class_eq`
    // compares `(a²/w, p²/w, sign p)` — and the SIGN is the oriented
    // class's, not the conjugacy class's. A sheet conjugates a rotation to
    // its INVERSE, which is a stated relator of this alphabet, and
    // inversion flips `sign p`. So `q` and `s·q·s⁻¹` are conjugate by
    // construction and compare unequal under `class_eq`, and refusing on it
    // threw away the very case the sheet exists to provide — measured: the
    // conjugator for the one relation the alphabet states outright was not
    // found.
    //
    // `class_eq` is a necessary condition for ORIENTED conjugacy and not for
    // conjugacy. The exact conjugation below is the decision, so nothing is
    // lost by asking it directly: a γ that verifies IS the proof, and no
    // prefilter can be more certain than the arithmetic it stands in for.
    // EVERY CONJUGATOR THE SOLVE FOUND, not one of them. The kernel of the
    // linearised relation is a COSET OF THE CENTRALISER — if γ conjugates,
    // so does γ·z for every z commuting with `a` — so it is a whole
    // subspace of solutions and they are not interchangeable here: the
    // alphabet spells some and not others. Taking one and asking whether it
    // was spellable answered "no" for a relation the alphabet states
    // outright (the sheet), because the one taken was a different member of
    // the same coset.
    //
    // The alphabet must be able to SAY it, or the language holds a relation
    // it cannot write down (37.7, from this side). `factor_rotor` answers
    // with the spelling or with the demand that stopped it; a demand is a
    // refusal here, not a conjugator.
    match conjugation(a, b)? {
        Conjugation::Spelled(w) => Some(w),
        Conjugation::Unspellable(_) => None,
    }
}

/// WHAT A CHANGE OF BASEPOINT IS, once it has been solved: a transition the
/// alphabet can SAY, or one it cannot.
#[derive(Clone, Debug)]
pub enum Conjugation {
    /// The seventeen spell it, so the language can write the relation down.
    Spelled(Word),
    /// 37.7 EXACTLY. The engine proved `a` and `b` are related by a gauge
    /// `γ ∈ Spin(4,0)`, and proved the existing generators cannot spell it.
    /// That is not a failure and not a silence — it is the measured
    /// obstruction a generator is made of, and it is carried out to be
    /// adjoined.
    Unspellable(Rotor),
}

/// Solve the change of basepoint and say which kind it is.
///
/// `solve_conjugator` answers `None` for both "no γ exists" and "γ exists
/// and cannot be spelled", and those are different facts: the first is a
/// refusal about the two words, the second is a candidate generator. This
/// separates them.
pub fn conjugation(a: &Word, b: &Word) -> Option<Conjugation> {
    let (ra, rb) = (a.rotor(), b.rotor());
    if ra.projective_eq(&rb) {
        return Some(Conjugation::Spelled(Word::identity()));
    }
    let found = conjugators(&ra, &rb);
    for g in &found {
        if let Some(Ok(w)) = super::factor_rotor(g) {
            if w.rotor().conjugate_holds(&ra, &rb) {
                return Some(Conjugation::Spelled(w));
            }
        }
    }
    // A γ exists and no member of its coset is spellable: the transition is
    // real and the language has no letter for it.
    found.into_iter().next().map(Conjugation::Unspellable)
}

/// The first conjugator the solve finds, for a caller that wants the rotor
/// rather than a spelling.
pub fn solve_conjugator_rotor(a: &Rotor, b: &Rotor) -> Option<Rotor> {
    conjugators(a, b).into_iter().next()
}

/// SOLVE `γ` WITH `γ·a = b·γ` IN THE ALGEBRA, exactly and without searching.
///
/// Conjugacy `γaγ⁻¹ = b` is not a question to be answered by trying tapes.
/// Multiply through: `γa = bγ`, which is LINEAR in γ. The map
/// `γ ↦ γ·a − b·γ` is a linear operator on the sixteen blade coefficients,
/// so its kernel is a subspace over ℚ and any INVERTIBLE element of that
/// kernel is a conjugator. Exact Gaussian elimination over the rationals —
/// no rounding, no candidate tapes, no bound on word length to apologise
/// for (law 3: this is a solve, not an enumeration).
///
/// The kernel can contain singular elements — `γ = 0` always lies in it —
/// so a member is accepted only when `weight` says it is a versor. The
/// basis vectors are tried, then their pairwise sums, which suffices to
/// avoid a degenerate choice: a kernel whose every basis vector is singular
/// but which contains an invertible element has one in the span of two.
///
/// `None` is "no conjugator exists in this algebra", which for two rotors
/// that are genuinely conjugate cannot happen, and for two that are not is
/// the correct refusal.
pub fn conjugators(a: &Rotor, b: &Rotor) -> Vec<Rotor> {
    // THE EQUATION IS PROJECTIVE, and the first cut of this was not.
    // Conjugation preserves the element up to scale — `Φ` and `λΦ` are one
    // vertex (1.5) — so the relation is `γ·a = λ·b·γ` and not `γ·a = b·γ`.
    // Measured on the alphabet's own sheet relation: `a = 1 + e₀₁` against
    // `b = (1 − e₀₁)/2`, and the literal system has only the zero solution
    // because of that ½. Normalising both to the canonical representative
    // removes λ up to sign, and the sign is what the two attempts below are
    // for: `Φ` and `−Φ` are one vertex, so a conjugator may solve either.
    let (a, b) = (a.normalised(), b.normalised());
    let minus = num_rational::BigRational::from_integer(num_bigint::BigInt::from(-1));
    let mut out: Vec<Rotor> = Vec::new();
    for signed in [b.clone(), b.scale(&minus)] {
        let zero = num_rational::BigRational::from_integer(num_bigint::BigInt::from(0));
        // Column j is the image of basis blade j: `e_j·a − b·e_j`.
        let mut m: Vec<Vec<num_rational::BigRational>> = vec![vec![zero.clone(); 16]; 16];
        for j in 0..16usize {
            let e = Rotor::blade(
                j,
                num_rational::BigRational::from_integer(num_bigint::BigInt::from(1)),
            );
            let img = e.geo(&a).add(&signed.geo(&e).scale(&minus));
            for (i, row) in m.iter_mut().enumerate() {
                row[j] = img.coeff(i).clone();
            }
        }
        for cand in nullspace(&m) {
            let r = (0..16usize).fold(Rotor::zero(), |acc, i| {
                acc.add(&Rotor::blade(i, cand[i].clone()))
            });
            // Invertible, and it must actually conjugate: the kernel is of
            // the linearised relation, and the verification is the exact
            // sandwich (rule 11 — a solution is proved, not presumed).
            if r.weight().is_some() && r.conjugate_holds(&a, &b) {
                // THE COSET IS REACHED FROM ANY MEMBER OF IT. `K = g·C(a)`,
                // and `a` commutes with itself, so `a^k ∈ C(a)` and every
                // `g·a^k` conjugates exactly as `g` does:
                // `(ga)a(ga)⁻¹ = g·a·a·a⁻¹·g⁻¹ = gag⁻¹ = b`.
                //
                // They are offered because the alphabet spells SOME members
                // of a coset and not others, and a solve that returns one
                // arbitrary element answers "unspellable" for a relation the
                // seventeen can say in a different basepoint. Four powers,
                // because a quarter turn has order four and `a⁴` is the
                // identity — past that the multiples repeat and nothing new
                // is offered.
                let mut acc = r.clone();
                out.push(r);
                for _ in 0..3 {
                    acc = acc.geo(&a);
                    if acc.weight().is_some() && acc.conjugate_holds(&a, &b) {
                        out.push(acc.clone());
                    }
                }
            }
        }
    }
    out
}

/// The kernel of a 16×16 rational matrix, as basis vectors and then their
/// pairwise sums. Exact row reduction over ℚ — every pivot is a rational
/// division and nothing is rounded or thresholded.
fn nullspace(m: &[Vec<num_rational::BigRational>]) -> Vec<Vec<num_rational::BigRational>> {
    use num_traits::{One, Zero};
    let n = 16usize;
    let mut a: Vec<Vec<num_rational::BigRational>> = m.to_vec();
    let mut pivot_of: Vec<Option<usize>> = vec![None; n];
    let mut row = 0usize;
    for col in 0..n {
        // The pivot is the first row at or below `row` with a non-zero entry
        // in this column — a position in the elimination, which is the
        // algorithm's own order and not a choice among members.
        let Some(p) = (row..n).find(|&r| !a[r][col].is_zero()) else {
            continue;
        };
        a.swap(row, p);
        let lead = a[row][col].clone();
        for x in a[row].iter_mut() {
            *x = &*x / &lead;
        }
        for r in 0..n {
            if r == row || a[r][col].is_zero() {
                continue;
            }
            let f = a[r][col].clone();
            let pivot_row = a[row].clone();
            for (x, p) in a[r].iter_mut().zip(pivot_row.iter()) {
                *x = &*x - &(&f * p);
            }
        }
        pivot_of[col] = Some(row);
        row += 1;
        if row == n {
            break;
        }
    }
    let free: Vec<usize> = (0..n).filter(|c| pivot_of[*c].is_none()).collect();
    let mut basis: Vec<Vec<num_rational::BigRational>> = Vec::new();
    for &f in &free {
        let mut v = vec![num_rational::BigRational::zero(); n];
        v[f] = num_rational::BigRational::one();
        for c in 0..n {
            if let Some(r) = pivot_of[c] {
                v[c] = -a[r][f].clone();
            }
        }
        basis.push(v);
    }
    // THE SINGULAR ELEMENTS ARE A HYPERSURFACE, and coordinate axes land on
    // it easily. Gaussian elimination returns a basis aligned to the
    // coordinate axes, and a non-versor — `γγ̃` not a non-zero scalar — is a
    // proper algebraic subvariety of the kernel, so individual basis vectors
    // frequently have no weight. Pairwise SUMS alone are the 2-dimensional
    // diagonals, and where `dim K > 2` those can lie on the singular cone
    // too (a commuting complementary plane gives `dim K` of 4 or 8).
    //
    // ONE CANONICAL COMBINATION, NOT A SEARCH OVER PAIRS. The first cut
    // pushed every `v_i + v_j` and `v_i − v_j`, which is a Cartesian sweep
    // of the basis against itself dressed as linear algebra — and it is not
    // even complete, because for `dim K > 2` the 2-dimensional diagonals can
    // lie on the singular cone as readily as the axes do.
    //
    // The basis vectors themselves are the object's own coordinates and
    // walking them is bounded by it (rule 10). What follows them is a SINGLE
    // determined element: `Σ 2ⁱ·v_i`, one point of the kernel fixed by the
    // basis and nothing else. The singular elements — where `γγ̃` is not a
    // non-zero scalar — are a proper algebraic subvariety of `K`, so a point
    // whose coordinates are distinct powers is off it except by coincidence,
    // and coincidence here costs a refusal and never a wrong answer: an
    // element that fails `weight` is dropped, and one that passes is still
    // verified by the exact sandwich before it is returned.
    let mut out = basis.clone();
    if basis.len() > 1 {
        let mut scale = num_rational::BigRational::one();
        let two = num_rational::BigRational::from_integer(num_bigint::BigInt::from(2));
        let mut combined = vec![num_rational::BigRational::zero(); n];
        for v in &basis {
            for (c, x) in combined.iter_mut().zip(v.iter()) {
                *c = &*c + &(&scale * x);
            }
            scale = &scale * &two;
        }
        out.push(combined);
    }
    out
}

impl Rotor {
    /// `self · a · self⁻¹ ≟ b`, projectively. Exact rational arithmetic and
    /// one comparison — the conjugation is computed, never approximated.
    pub fn conjugate_holds(&self, a: &Rotor, b: &Rotor) -> bool {
        a.conjugate(self).projective_eq(b)
    }
}

/// THE SECTIONS OF ONE BASE, grouped. Words sharing a base are one bundle
/// and the vocabulary holds the base once; words of different bases are
/// different bundles and nothing here merges them.
///
/// Keyed, never a scan: the base key IS the coordinate, so this is a read at
/// it (law 3).
pub fn by_base(sections: &[(Word, Section)]) -> crate::network::nerve::Keyed<Vec<Word>> {
    let mut out: crate::network::nerve::Keyed<Vec<Word>> = crate::network::nerve::Keyed::new();
    for (w, s) in sections {
        match out.get_mut(s.base.as_bytes()) {
            Some(v) => v.push(w.clone()),
            None => {
                out.insert(s.base.as_bytes(), vec![w.clone()]);
            }
        }
    }
    out
}

/// IS THIS WORD A SECTION OF A BUNDLE THE LANGUAGE ALREADY HOLDS?
///
/// The expansion's whole point: a word is redundant not only when its cells
/// tile, but when it is a CONJUGATE of something held — same base, and a
/// conjugator the alphabet can spell. Then the language keeps the base and
/// the transition, and the second monolith is a section.
///
/// The answer carries the conjugator, because "it is redundant" without the
/// γ that shows it is an assertion; with γ it is a re-derivation anyone can
/// replay.
pub fn section_of(held: &[Word], w: &Word) -> Option<(Word, Word)> {
    // NOT GATED ON EQUAL BASES, for the reason `solve_conjugator` states: a
    // sheet carries a rotation to its inverse, and the two stand in
    // different ORIENTED classes while being conjugate. Gating here would
    // refuse exactly the sections the alphabet's own relator provides.
    //
    // `Base` stays the oriented class because that is what `class_key` reads
    // and what `by_base` should group by — a finer grouping than conjugacy,
    // and the right one for a vocabulary that must write the tape down. The
    // conjugator is what relates two of them, and it is returned so the
    // relation travels with the claim.
    Base::of_word(w)?;
    for h in held {
        if let Some(g) = solve_conjugator(h, w) {
            return Some((h.clone(), g));
        }
    }
    None
}

/// THE UNSPELLABLE GAUGE TRANSITIONS between a word and what the language
/// holds — 37.7's candidates, from the bundle's side.
///
/// A word conjugate to a held word is a section of its bundle. Where the
/// seventeen can spell the transition the language writes it down and the
/// second monolith retires; where they CANNOT, the engine has proved a gauge
/// exists that it has no letter for, and that rotor is exactly what an
/// adjoined generator is made of.
///
/// Returning nothing there — which `solve_conjugator` does, because `None`
/// covers both "no γ" and "γ nobody can say" — drops the one measurement
/// that grows the alphabet.
pub fn unspellable_gauges(held: &[Word], w: &Word) -> Vec<(Word, Rotor)> {
    let mut out = Vec::new();
    for h in held {
        if let Some(Conjugation::Unspellable(g)) = conjugation(h, w) {
            out.push((h.clone(), g));
        }
    }
    out
}

/// The identity fibre: no conjugator, identity carriage, no recolour. The
/// coordinate a base stands at over itself.
pub fn trivial_fibre() -> Fibre {
    Fibre {
        gauge: Some(Word::identity()),
        carriage: element::identity_versor(),
        hue: Vec::new(),
    }
}
