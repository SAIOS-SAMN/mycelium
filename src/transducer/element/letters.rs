// 16.4, 1.4 — THE LETTERS: the versor and the alphabet it spells.
//
//! `Versor` is the conformal multivector the action carries: the versors built
//! by name (identity, translator, dilator, carriage), the projective equality,
//! the decomposition into rho and translation, and `Letters`, the M1 coordinates.

use crate::core::Q;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use super::*;

/// THE TRANSPORT of the bundle: a rational conformal versor V ∈ Spin(4,1)(ℚ)
/// (translation, rotation by any rational rotor, reflection, exact dilation,
/// and their products), acting by the sandwich V X Ṽ on the null points of
/// a support. The lattice isometries are the special case; no table of
/// images exists anywhere.
pub type Versor = crate::transducer::cga::Mv;

pub fn identity_versor() -> Versor {
    Versor::scalar(num_rational::BigRational::from_integer(BigInt::from(1)))
}

/// The word's ORIGIN-FIXING action as a CGA versor, no translation. The
/// identity translator is formed here, not at the call site. `None` when
/// the word carries the base off itself.
pub fn versor_of_word(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    base_action_of(&word.rotor())
}

/// THE CONFORMAL REALISATION OF A WORD: the word's base action composed
/// with the carriage by the geometric product. `None` when the word
/// carries the base off itself.
pub fn conformal_of(word: &crate::transducer::alphabet::Word, carriage: &Versor) -> Option<Versor> {
    conformal_of_rotor(&word.rotor(), carriage)
}

/// The same versor from the ACTION alone (L14): a word vertex carries its
/// canonical rotor and the sandwich needs nothing else, so no citation is
/// resolved to speak a body.
pub fn conformal_of_rotor(
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
) -> Option<Versor> {
    Some(carriage.geo(&base_action_of(rotor)?))
}

/// The rotor chart's blades that are the BASE's own even subalgebra
/// `⟨1, e₀₁, e₀₃, e₁₃⟩` — `e₀, e₁, e₃` are `r, c, k` (51.2) — and the
/// versor blades they are read as: `1, e1e2, e1e3, e2e3`.
const BASE_EVEN: [(usize, usize); 4] = [
    (0, 0),
    (0b0011, crate::transducer::cga::E1 | crate::transducer::cga::E2),
    (0b1001, crate::transducer::cga::E1 | crate::transducer::cga::E3),
    (0b1010, crate::transducer::cga::E2 | crate::transducer::cga::E3),
];

/// The half turn `e₀₂`: the orientation character (51.2), which flips `e₀`
/// on the base and nothing else there, and is realised on the base as the
/// reflector `e1`.
const ORIENTATION: usize = 0b0101;

/// THE BASE ACTION OF A ROTOR (51.2, A4): the rotor's action on the three
/// base axes of the chart — `e₀, e₁, e₃`, which are `r, c, k` — as the
/// versor of `Cl(4,1)` on `e1, e2, e3`. Exact, and complete on those axes:
/// an even element of `Cl(4,0)` that carries the base onto itself is, up
/// to scale, a rotor of the base's own even subalgebra `⟨1, e₀₁, e₀₃,
/// e₁₃⟩`, read off blade for blade; or that times the half turn `e₀₂` —
/// the orientation character — realised as the reflector `e1`. A rotor
/// that carries the base off itself (one turning `e₂` into a base axis)
/// has no base action, and says so: nothing of it is projected.
fn base_action_of(rotor: &crate::transducer::alphabet::Rotor) -> Option<Versor> {
    use crate::transducer::alphabet::Rotor;
    let even_on_base = |r: &Rotor| (0..16).all(|m| BASE_EVEN.iter().any(|&(rm, _)| rm == m) || r.coeff(m).is_zero());
    let read = |r: &Rotor| -> Versor {
        BASE_EVEN.iter().fold(Versor::zero(), |v, &(rm, vm)| v.add(&Versor::blade(vm, r.coeff(rm).clone())))
    };
    if rotor.is_identity() {
        return Some(identity_versor());
    }
    if even_on_base(rotor) {
        let v = read(rotor);
        return (!v.is_zero()).then_some(v);
    }
    // `R = R₃ · e₀₂` ⇒ `R₃ = R · e₀₂⁻¹ = −R · e₀₂`, and the base then reads
    // `R₃` with the reflector after it: the sandwich reflects first.
    let r3 = rotor.geo(&Rotor::blade(ORIENTATION, -Q::one()));
    if even_on_base(&r3) {
        let v = read(&r3);
        return (!v.is_zero()).then_some(v.geo(&crate::transducer::cga::reflector()));
    }
    None
}

/// THE ROTOR A VERSOR'S BASE ACTION IS (51.2; N2): `V = D(ρ)·T(t)·R` on
/// the base, so `R = (D·T)⁻¹ V`, read exactly off the carriage `dilator_of`
/// and `translator_of` measure, and mapped back blade for blade onto
/// `⟨1, e₀₁, e₀₃, e₁₃⟩`; a reflector `e1` in it is the orientation
/// character `e₀₂`. `None` when `V` is no versor of the base or `R` is not
/// a rotation or a reflection of it — nothing is projected. The inverse of
/// [`versor_of_word`] on every rotor that has a base action.
pub fn base_rotor_of(v: &Versor) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::alphabet::Rotor;
    use crate::transducer::cga::{E1, BLADES};
    let carriage = dilator_of(v)?.geo(&translator_of(v)?);
    let r = carriage.versor_inverse()?.geo(v);
    let on_base = |m: &Versor| (0..BLADES).all(|b| BASE_EVEN.iter().any(|&(_, vm)| vm == b) || m.coeff(b).is_zero());
    let read = |m: &Versor| -> Rotor {
        BASE_EVEN.iter().fold(Rotor::zero(), |acc, &(rm, vm)| acc.add(&Rotor::blade(rm, m.coeff(vm).clone())))
    };
    if on_base(&r) {
        let rotor = read(&r);
        return rotor.weight().map(|_| rotor);
    }
    // `r = r₃ · e1` ⇒ `r₃ = r · e1`, and the chart reads `r₃ · e₀₂`.
    let r3 = r.geo(&Versor::blade(E1, Q::one()));
    if on_base(&r3) {
        let rotor = read(&r3).geo(&Rotor::blade(ORIENTATION, Q::one()));
        return rotor.weight().map(|_| rotor);
    }
    None
}

/// Projective equality of two versors: the same transport up to a nonzero
/// rational scale (versors act projectively on the cone).
pub fn same_versor(a: &Versor, b: &Versor) -> bool {
    crate::transducer::cga::projective_ratio(a, b).is_some()
}

/// A4 carriage: dilator(ρ) · translator, of a body in frame `0` — its
/// `k` coordinate, not a default (16.4). Sensor pairs stay in `pair_form`,
/// `speak_rule`, and `orbit`.
pub fn carriage(rho: &Q, t: &(Q, Q)) -> Versor {
    carriage_at(rho, &(t.0.clone(), t.1.clone(), Q::zero()))
}

/// A4 carriage on the three-coordinate base: dilator(ρ) · translator(d).
pub fn carriage_at(rho: &Q, t: &(Q, Q, Q)) -> Versor {
    crate::transducer::cga::dilator(rho).geo(&crate::transducer::cga::translator(&t.0, &t.1, &t.2))
}

/// THE DILATION A VERSOR CARRIES (A4, I12), read exactly off its action:
/// every translator and rotor fixes `e_∞` up to its own norm, and the
/// dilator alone scales it — `V e_∞ Ṽ = κ e_∞` with `κ / (V Ṽ) = ρ` for
/// `V = D(ρ) · T · R`. A reflection in `V` (the orientation character, 3.10)
/// negates the ratio and is not a scale, so the magnitude is read. `None`
/// when `V` is not a versor or sends `e_∞` off itself (no conformal action
/// of this form).
pub fn rho_of(v: &Versor) -> Option<Q> {
    let n = crate::transducer::cga::is_versor(v)?;
    let inf = crate::transducer::cga::e_inf();
    let kappa = crate::transducer::cga::projective_ratio(&v.sandwich(&inf), &inf)?;
    let rho = kappa / n;
    if rho.is_zero() {
        return None;
    }
    Some(if rho < Q::zero() { -rho } else { rho })
}

/// Translator component of a carriage `V = D(ρ) · T(t) · R`: the origin's
/// image under the sandwich is `X(ρ t)` — the dilator scales the
/// translation it follows — so `t` is that image divided by ρ. It read the
/// image as `t` itself, which is exact only at ρ = 1: at ρ ≠ 1 the
/// placement residue was ρ times the placement and `dilator_of` was not a
/// dilator (I12). `None` when the origin is sent to infinity or ρ cannot
/// be read.
pub fn translator_of(v: &Versor) -> Option<Versor> {
    let rho = rho_of(v)?;
    let z = Q::zero();
    let img = crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&z, &z, &z)))?;
    let (t1, t2, t3) = (img.0 / &rho, img.1 / &rho, img.2 / &rho);
    if t1.is_zero() && t2.is_zero() && t3.is_zero() {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::translator(&t1, &t2, &t3))
    }
}

/// Dilator component of a carriage: `D(ρ)` for the ρ the versor carries,
/// the identity at ρ = 1. Exact at every ρ; it was `v · T⁻¹` with `T` read
/// wrong at ρ ≠ 1, which left a translation inside the "dilator".
pub fn dilator_of(v: &Versor) -> Option<Versor> {
    let rho = rho_of(v)?;
    if rho.is_one() {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::dilator(&rho))
    }
}

/// The base's three axes in the rotor chart (51.2): `e₀, e₁, e₃` are `r,
/// c, k`, and `e₂` is off the base. Index 2 of a rotor's action is the row
/// that says a turn leaves the base, and it is a row of the system here,
/// never dropped.
const BASE_AXES: [usize; 3] = [0, 1, 3];

/// THE BASE POINT OF A TURN (12.6; `cech::classify`'s standing note): a
/// rotation about `p` rather than the origin is `T_p R T_p⁻¹ = T(p − R p)
/// · R`, so its carriage is not a residue — it is WHERE the turn stands,
/// and `p` is determined by `R` and the translation together. `Some(p)`
/// when the carriage is exactly `T(p − R p)` for some rational `p`; `None`
/// when it is not — a dilation, a walk, or a turn that leaves the base.
///
/// It stood unread, and that is why a turning body never filed a period.
/// `classify` will only read a finite order off a FLAT carriage, and every
/// transport a voice measures arrives turning about its body's own
/// centroid: `versor_coset` builds each member as `T(c_b)·D(ρ)·R·T(−c_a)`,
/// so the translation is always there and every turn classified as a
/// standing class instead. Reading `p` loses nothing law 12 would keep —
/// it is a FUNCTION of `R` and `t`, not a coordinate beside them — while
/// refusing to read it lost the turn itself.
pub fn turn_base_point(
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
) -> Option<[Q; 3]> {
    // A dilation is a residue and not a base point, and a carriage that is
    // not exactly the translator by the origin's image carries something
    // this reading does not name: refused whole rather than approximated
    // (rule 11).
    if !rho_of(carriage)?.is_one() {
        return None;
    }
    let z = Q::zero();
    let t = crate::transducer::cga::base(
        &carriage.sandwich(&crate::transducer::cga::point(&z, &z, &z)),
    )?;
    if !same_versor(carriage, &crate::transducer::cga::translator(&t.0, &t.1, &t.2)) {
        return None;
    }
    // `(I − R) p = t`, four rows in three unknowns: the base's three and
    // the off-base row that must come out zero, so a turn carrying the
    // base off itself refuses here instead of being solved on a projection
    // (checklist (m)).
    let mut rows: [[Q; 4]; 4] = std::array::from_fn(|_| std::array::from_fn(|_| Q::zero()));
    for (col, &axis) in BASE_AXES.iter().enumerate() {
        let mut e: [Q; 4] = std::array::from_fn(|_| Q::zero());
        e[axis] = Q::one();
        let image = rotor.carry(&e)?;
        for (row, &r_axis) in BASE_AXES.iter().enumerate() {
            rows[row][col] = match row == col {
                true => Q::one() - &image[r_axis],
                false => -image[r_axis].clone(),
            };
        }
        rows[3][col] = -image[2].clone();
    }
    for (row, rhs) in [t.0, t.1, t.2, Q::zero()].into_iter().enumerate() {
        rows[row][3] = rhs;
    }
    solve_base_point(rows)
}

/// Exact rational elimination on the four rows of `(I − R) p = t`. Any
/// solution answers, because every point of a turn's axis is a base point
/// of the same turn and the reading asks whether one exists, not which.
/// `None` is a row that is zero on the left and not on the right: the
/// system saying no point turns this way.
fn solve_base_point(mut rows: [[Q; 4]; 4]) -> Option<[Q; 3]> {
    let mut pivot_of: [Option<usize>; 3] = [None, None, None];
    let mut pivot = 0usize;
    for col in 0..3 {
        let Some(found) = (pivot..4).find(|&r| !rows[r][col].is_zero()) else { continue };
        rows.swap(pivot, found);
        let lead = rows[pivot][col].clone();
        for cell in rows[pivot].iter_mut().skip(col) {
            *cell = &*cell / &lead;
        }
        let lead_row = rows[pivot].clone();
        for (r, row) in rows.iter_mut().enumerate() {
            if r == pivot || row[col].is_zero() {
                continue;
            }
            let f = row[col].clone();
            for (cell, above) in row.iter_mut().zip(lead_row.iter()).skip(col) {
                *cell = &*cell - &(above * &f);
            }
        }
        pivot_of[col] = Some(pivot);
        pivot += 1;
    }
    if (pivot..4).any(|r| !rows[r][3].is_zero()) {
        return None;
    }
    Some(std::array::from_fn(|c| match pivot_of[c] {
        Some(r) => rows[r][3].clone(),
        None => Q::zero(),
    }))
}

// `sensor_plane_block` — the 2×2 projection of a versor's rotation onto
// the sensor plane — stood here. `base_rotor_of` reads the rotation whole
// on the base's three planes (51.2), and a projection of it is what
// checklist (m) refuses.

/// `by V by⁻¹` on versors — the same conjugation `Rotor::conjugate` does,
/// one algebra up. A conjugator that does not invert leaves `v` unmoved,
/// as the rotor's does: a non-invertible transport is not a change of
/// frame and cannot move a class.
pub fn versor_conjugate(v: &Versor, by: &Versor) -> Versor {
    match by.versor_inverse() {
        Some(inv) => by.geo(v).geo(&inv),
        None => v.clone(),
    }
}

/// The translator by an integer lattice offset within a frame — the lattice
/// peel's transport as a rational versor.
pub fn translator_versor(r: i64, c: i64) -> Versor {
    crate::transducer::cga::translator(
        &num_rational::BigRational::from_integer(BigInt::from(r)),
        &num_rational::BigRational::from_integer(BigInt::from(c)),
        &Q::zero(),
    )
}

/// A GROUP ELEMENT: one versor, no chain. This is the thing 3.x calls a
/// group element and `same_action` describes -- an action on the sensor
/// with no boundary of its own.
pub fn of_versor(v: Versor) -> Element {
    let mut e = Element::identity();
    e.action.spatial = Some(v);
    e.address.take();
    e
}

/// THE LETTERS OF A CONNECTION as read off its description: the word,
/// the A4 carriage, and the gauge map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Letters {
    pub spelling: crate::transducer::alphabet::Spelling,
    pub carriage: Versor,
    pub sigma: Vec<(u8, u8)>,
}

impl Element {
    /// The letters of this WORD — `None` for a residual germ or a word
    /// spoken before the alphabet (no tape: not a word of the language).
    ///
    /// Two tags, and they are the two the parser below reads: a connection
    /// and a translation are words. A SENTENCE is not — it is a nerve one
    /// level above a word (18.2), its tape is `[tag, version, form…]`, and
    /// byte 1 is the version where a spelling would be. It used to be
    /// admitted by this guard and then fail silently on that byte, which
    /// read as "this sentence has no letters" when the truth is that the
    /// question does not apply to it. A sentence's words are its loop
    /// holonomies: `Form::loop_words`.
    pub fn letters(&self) -> Option<Letters> {
        use crate::transducer::alphabet::Spelling;
        let b = &self.description;
        let tag = crate::transducer::address::tag_of(b)?;
        if tag != CONNECTION_TAG && tag != TRANSLATION_TAG {
            return None;
        }
        let mut pos = 1usize;
        let spelling = Spelling::from_bytes(b, &mut pos)?;
        let carriage = Versor::from_bytes(b, &mut pos)?;
        let n = u16::from_le_bytes(b.get(pos..pos + 2)?.try_into().ok()?) as usize;
        pos += 2;
        let mut sigma = Vec::with_capacity(n);
        for _ in 0..n {
            sigma.push((*b.get(pos)?, *b.get(pos + 1)?));
            pos += 2;
        }
        let len = u32::from_le_bytes(b.get(pos..pos + 4)?.try_into().ok()?) as usize;
        pos += 4;
        if pos + len > b.len() {
            return None;
        }
        Some(Letters { spelling, carriage, sigma })
    }

    /// Tape re-derives vertex: the geodesic of the stored word is the same
    /// transport — projective equality of the two rotors (M1, A1), never
    /// equality of two matrices (rule 16) — and V reconstructed from the
    /// tape agrees with the action. `None` when there are no letters.
    pub fn letters_rederive(&self) -> Option<bool> {
        let letters = self.letters()?;
        let v = conformal_of(&letters.spelling.word, &letters.carriage)?;
        let word = &letters.spelling.word;
        let same_word = word.rotor().projective_eq(&word.geodesic().rotor());
        Some(same_word && self.versor().is_some_and(|held| same_versor(&held, &v)))
    }

    /// The axes opened above the four strata: germinated versors.
    pub fn opened_axes(&self) -> Vec<u16> {
        self.action.germinated.iter().map(|(i, _)| *i).collect()
    }

    /// Is this word a CONNECTION — letters on the tape and an invertible
    /// versor. A residual chain is never a connection.
    pub fn is_connection(&self) -> bool {
        let Some(letters) = self.letters() else {
            return false;
        };
        if self.description_tag() != Some(CONNECTION_TAG) {
            return false;
        }
        conformal_of(&letters.spelling.word, &letters.carriage)
            .and_then(|v| v.versor_inverse())
            .is_some()
            && gauge_is_perm(&letters.sigma)
            && !(letters.spelling.word.is_identity()
                && same_versor(&letters.carriage, &identity_versor())
                && letters.sigma.is_empty()
                && self.sub.is_empty())
    }

    /// THE DESCRIPTION TAG this word opens with — 17.2's own offset, read
    /// through the one carrier for it. The tag is a coordinate of the
    /// description, so the reader is named for the description and not for the
    /// word, whose tag it is not.
    pub(crate) fn description_tag(&self) -> Option<u8> {
        crate::transducer::address::tag_of(&self.description)
    }

    /// A sentence: its own tape tag, never merely an opened-axis connection.
    pub fn is_sentence(&self) -> bool {
        self.description_tag() == Some(SENTENCE_TAG)
    }

    /// A translation: the gauge between two sections.
    pub fn is_translation(&self) -> bool {
        self.description_tag() == Some(TRANSLATION_TAG)
    }

    /// An optimisation spoken as a word.
    pub fn is_rewrite(&self) -> bool {
        self.description_tag() == Some(REWRITE_TAG)
    }

    /// Mint a Rewrite: the address measures the tape.
    pub fn from_rewrite_tape(description: Vec<u8>, mag: [u64; 4]) -> Element {
        let mut e = Element::identity();
        e.description = description;
        e.mag = mag;
        e.address.take();
        e
    }

    /// The two sections a translation gauges, if the tape carries them.
    pub fn translation_sections(&self) -> Option<(crate::transducer::address::Address, crate::transducer::address::Address)> {
        if !self.is_translation() {
            return None;
        }
        let b = self.description();
        // Walk back two length-delimited addresses at the tail.
        // The tape is prefix + addr_a + addr_b; decode from the start of the pair
        // by scanning: find the first put_addr that consumes the suffix.
        let mut q = 0usize;
        while q + 2 <= b.len() {
            let mut t = q;
            if let (Some(a), Some(c)) = (
                crate::transducer::address::take_addr(b, &mut t),
                crate::transducer::address::take_addr(b, &mut t),
            ) {
                if t == b.len() {
                    return Some((a, c));
                }
            }
            q += 1;
        }
        None
    }

}
