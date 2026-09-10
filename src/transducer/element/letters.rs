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

/// Linear word as a CGA versor, no translation. The identity translator is
/// formed here, not at the call site. `None` when the word does not keep
/// the sensor plane.
pub fn versor_of_word(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    linear_versor(word)
}

/// THE CONFORMAL REALISATION OF A WORD: the linear versor of the word
/// composed with the carriage by the geometric product. `None` when the
/// word does not preserve the plane.
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
    Some(carriage.geo(&linear_versor_of(rotor)?))
}

fn linear_versor(word: &crate::transducer::alphabet::Word) -> Option<Versor> {
    linear_versor_of(&word.rotor())
}

fn linear_versor_of(rotor: &crate::transducer::alphabet::Rotor) -> Option<Versor> {
    use crate::transducer::cga;
    let block = crate::transducer::alphabet::rotor_block_on(
        rotor,
        (crate::transducer::alphabet::PLANE_ROW, crate::transducer::alphabet::PLANE_COL),
    )?;
    let rho2 = &block[0][0] * &block[0][0] + &block[1][0] * &block[1][0];
    if rho2.is_zero() {
        return None;
    }
    let r: [[Q; 2]; 2] = std::array::from_fn(|i| {
        std::array::from_fn(|j| {
            // The linear word is the rotation/orientation; dilation lives
            // on the carriage.
            &block[i][j] / Q::from_integer(BigInt::from(1))
        })
    });
    let det = &r[0][0] * &r[1][1] - &r[0][1] * &r[1][0];
    let flipped = if det.is_one() {
        false
    } else if (-&det).is_one() {
        true
    } else {
        return None;
    };
    let (c, s) = (&r[0][0], &r[1][0]);
    let rot = if (-c).is_one() {
        cga::rotator(&Q::zero(), &Q::one())
    } else {
        cga::rotator(&(Q::one() + c), &(-s))
    };
    let mut v = rot;
    if flipped {
        // The half-turn on a third axis is the orientation character;
        // there is no reflection letter.
        v = v.geo(&cga::rotator(&Q::zero(), &Q::one())).geo(&cga::reflector());
    }
    Some(v)
}

/// Projective equality of two versors: the same transport up to a nonzero
/// rational scale (versors act projectively on the cone).
pub fn same_versor(a: &Versor, b: &Versor) -> bool {
    crate::transducer::cga::projective_ratio(a, b).is_some()
}

/// A4 carriage: dilator(ρ) · translator. Sensor pairs stay in `pair_form`,
/// `speak_rule`, and `orbit`.
pub fn carriage(rho: &Q, t: &(Q, Q)) -> Versor {
    crate::transducer::cga::dilator(rho).geo(&crate::transducer::cga::translator(&t.0, &t.1))
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
    let img = crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&z, &z)))?;
    let (t1, t2) = (img.0 / &rho, img.1 / &rho);
    if t1.is_zero() && t2.is_zero() {
        Some(identity_versor())
    } else {
        Some(crate::transducer::cga::translator(&t1, &t2))
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

/// A3 — THE SENSOR-PLANE BLOCK A VERSOR PROJECTS TO: the images of the
/// row and column steps with the origin's image subtracted. It is the one
/// matrix rule 11 allows, and it is a PROJECTION, not the versor: the
/// placement residue is subtracted away by construction and a dilation
/// survives only as the scalar a peel divides out.
///
/// It lives here, beside `rho_of`, `translator_of` and `dilator_of`,
/// because it is the fourth reader of a versor and the three of them are
/// only correct together — `V = D(ρ)·T(t)·R` and this returns the block of
/// the `R`. It was in `family::perceive`, which put a versor reader inside
/// one of its callers and left the court unable to reach it.
///
/// Ask it about a ROTATION and it answers exactly. Ask it to classify a
/// versor — its order, whether it is the identity, what class it carries —
/// and it answers wrongly, because a translation and a dilation both have
/// the identity for a block. `meet_class` is that question's reader.
pub fn sensor_plane_block(v: &Versor) -> Option<[[Q; 2]; 2]> {
    let qn = |n: i64| num_rational::BigRational::from_integer(BigInt::from(n));
    let at = |r, c| crate::transducer::cga::base(&v.sandwich(&crate::transducer::cga::point(&qn(r), &qn(c))));
    let o = at(0, 0)?;
    let er = at(1, 0)?;
    let ec = at(0, 1)?;
    Some([[&er.0 - &o.0, &ec.0 - &o.0], [&er.1 - &o.1, &ec.1 - &o.1]])
}

/// WHAT A MEET'S TRANSITION IS (12.6, 15.5, 50.8 case 3). Three readings
/// and none of them is a choice.
#[derive(Debug, Clone)]
pub enum MeetClass {
    /// The two charts agree across the seam. Nothing to fill.
    Flat,
    /// A rotor of FINITE order about a point, with no placement or
    /// dilation residue left over: a torsion class of this form, filed at
    /// the period it closes at (15.5, A17). The sentence then carries the
    /// class as its theorem and the demand does not stand on it.
    Finite { ell: u32, rotor: Box<crate::transducer::alphabet::Rotor> },
    /// Everything else the seventeen DO spell: an infinite-order rotor, a
    /// placement residue, a dilation residue, or any product of them.
    /// Nothing is filed and the demand stands CARRYING THIS CLASS, which
    /// is what lets an arriving generator of the conjugate class close it
    /// without a coordinator.
    Standing { kappa: Box<crate::transducer::sentence::ClassVector> },
    /// THE ROTOR PART EXISTS AND THE ALPHABET DOES NOT SPELL IT (37.7).
    ///
    /// This is not a worse `Standing`, and it is emphatically not `Flat`.
    /// It is the adjunction's own condition met at a seam: a rotation the
    /// seventeen cannot reach, standing on a prime outside them. It gets
    /// its own arm because the alternative is to hand `ClassVector::read`
    /// the identity word and file a class that says "this seam turns by
    /// nothing", which is the invented value rule 11 refuses — and which
    /// would silently consume every adjunction candidate the cover finds.
    Unspellable { rotor: Box<crate::transducer::alphabet::Rotor>, carriage: Box<Versor> },
}

/// CLASSIFY A MEET'S TRANSITION, ON THE VERSOR (12.6).
///
/// The versor is factored exactly, `V = D(ρ)·T(t)·R`, by the three readers
/// above; the rotor part is `R = T⁻¹·D⁻¹·V` and its order is read from the
/// rotor, not from a matrix. `Finite` is returned ONLY when both residues
/// are the identity, so a rotation carried along a translation is not
/// mistaken for a pure rotation of the same order.
///
/// A rotation about a point away from the origin is `T_p R T_p⁻¹ =
/// T(p − Rp)·R`: the placement residue is `p − Rp` and the rotor part is
/// `R`, so this reads its order correctly where a block reading either
/// mis-orders it or, for a pure translation or dilation, reads it as the
/// identity and files nothing at all. That was measured: sixteen covers
/// over twelve tasks filed nothing, and the last of them had a meet whose
/// Γ was a translation and whose block was the identity.
///
/// `None` is not a class of transition: it is a versor these readers
/// cannot factor — one that sends `e_∞` off itself or the origin to
/// infinity — and an unfactored versor is unmeasured, never `Flat`
/// (rule 11).
pub fn meet_class(
    gamma: &Versor,
    hue: crate::transducer::sentence::CycleType,
) -> Option<MeetClass> {
    use crate::transducer::alphabet::{rotor_of_block, Order, PLANE_COL, PLANE_ROW};
    use crate::transducer::sentence::ClassVector;
    // Projective, because versors act projectively on the cone (1.5): a
    // nonzero rational multiple of the identity is the identity.
    if same_versor(gamma, &identity_versor()) {
        return Some(MeetClass::Flat);
    }
    let t = translator_of(gamma)?;
    let d = dilator_of(gamma)?;
    let carriage = d.geo(&t);
    let r = t.versor_inverse()?.geo(&d.versor_inverse()?).geo(gamma);
    let flat_carriage = same_versor(&carriage, &identity_versor());
    let rotor = rotor_of_block(&sensor_plane_block(&r)?, (PLANE_ROW, PLANE_COL));
    // The rotor part is asked its ORDER, and the block is only how the
    // rotor is recovered from the action. `Finite` needs both residues to
    // be the identity: a quarter turn carried along a translation has the
    // same rotor order as one about a fixed point and is not the same
    // class, so filing it at period 4 would put a theorem on the form that
    // the seam does not satisfy.
    // Infinite order, or a rotor with no order over ℚ, falls through to
    // the standing reading below, which writes the residues out even
    // though they are the identity here — a class vector that omitted
    // them would not be comparable with one that carries them.
    if let (true, Some(Order::Finite(ell))) =
        (flat_carriage, rotor.as_ref().and_then(|r| r.order()))
    {
        if let Some(rotor) = rotor {
            return Some(MeetClass::Finite { ell, rotor: Box::new(rotor) });
        }
    }
    // 12.6's order: the rotor's conjugacy class first, then the placement
    // and dilation residues, which ride on the carriage. The word is the
    // rotor's OWN spelling, and where the seventeen do not reach it there
    // is no word to read a class from — that is 37.7's arm, not a class of
    // the identity.
    // A rotor part the rationals do not induce is one this reader could
    // not factor. That is UNMEASURED and returns `None`: writing a class
    // with the identity word and the true residues would assert the seam
    // turns by nothing, which is the same invented value one line down.
    // A pure translation does not land here — its rotor part is the
    // identity versor, whose block is the identity and whose rotor is the
    // identity rotor — so it stands with its placement residue, which is
    // the reading the block collapse could not produce.
    let r = rotor?;
    match crate::transducer::alphabet::word_of_rotor(&r) {
        Some(word) => Some(MeetClass::Standing {
            kappa: Box::new(ClassVector::read(&word, carriage, hue)?),
        }),
        None => Some(MeetClass::Unspellable { rotor: Box::new(r), carriage: Box::new(carriage) }),
    }
}

/// The translator by an integer lattice offset — the lattice peel's transport
/// as a rational versor.
pub fn translator_versor(r: i64, c: i64) -> Versor {
    crate::transducer::cga::translator(
        &num_rational::BigRational::from_integer(BigInt::from(r)),
        &num_rational::BigRational::from_integer(BigInt::from(c)),
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
        let b = &self.word;
        let tag = b.first().copied()?;
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
        if self.word.first() != Some(&CONNECTION_TAG) {
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

    /// A sentence: its own tape tag, never merely an opened-axis connection.
    pub fn is_sentence(&self) -> bool {
        self.word.first().copied() == Some(SENTENCE_TAG)
    }

    /// A translation: the gauge between two sections.
    pub fn is_translation(&self) -> bool {
        self.word.first().copied() == Some(TRANSLATION_TAG)
    }

    /// An optimisation spoken as a word.
    pub fn is_rewrite(&self) -> bool {
        self.word.first().copied() == Some(REWRITE_TAG)
    }

    /// Mint a Rewrite: the address measures the tape.
    pub fn from_rewrite_tape(word: Vec<u8>, mag: [u64; 4]) -> Element {
        let mut e = Element::identity();
        e.word = word;
        e.mag = mag;
        e.address.take();
        e.form.take();
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
