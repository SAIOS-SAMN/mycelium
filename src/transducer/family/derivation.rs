// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 50.2 step 2, 42.6, 37.7 — WHAT A TARGET IS AGAINST A MIND'S OWN
//! LANGUAGE: the lanes it occupies, the basis at the meet of those lanes,
//! the ordered peel over that basis, and A12 over what the peel left.
//!
//! These readers are only correct together. `occupied_lanes` says which
//! covers the target must reduce against; `adjoined_basis` fetches the
//! generators at the meet of exactly those; `peel_versor_with` divides by
//! them; `derivation_of` reduces the letters that remain against the same
//! meet. Reading the basis off one lane while the target occupies two is
//! the union L12 refuses, and it is a single literal `3` away in any of
//! them.
//!
//! Split from `perceive.rs` at the §1.1 600-line reading bound; the items
//! are moved, not rewritten (§1.6). `perceive.rs` keeps perception — a
//! grid becoming bodies and two bodies becoming a vertex — and calls in
//! here for what a mind makes of the carriage it measured.

use super::*;

/// 50.2 F STEP 2 — FACTOR THROUGH THE PRESENTATION, AND PEEL ONLY THE
/// RESIDUE. The sensor plane is the first block of 50.4 and the other
/// planes read the identity for a plane versor, so the ordered peel is
/// the sensor peel with its remainder typed. What the mind holds at the
/// meet of the target's lanes is then divided out by A12, and what those
/// generators do not reach stays as the derivation's residue. `None` when
/// the versor has no linear block; `Err` the demand it leaves.
///
/// THE ORDER OF THE TWO STEPS, named because it is not the order 50.2
/// step 2 writes. The doctrine says factor first and peel the residue.
/// `factor_on_plane` (A12) reduces in the LETTER-EXPONENT lattice, so its
/// target must already be a word: there is no plane form of a block, and
/// building one is the peel. So the peel runs first and the factorisation
/// runs on the word it produced. The derivation that comes out is the
/// same object in every case where the peel closes — the generators
/// cited, the residue what they did not reach — and it is no longer a
/// function of the versor alone, which is what made the old path a
/// cipher. The two orders differ in exactly one case: a block the ORDERED
/// peel cannot spell but which is a product of held generators. This
/// implementation types a demand there; the doctrinal order would derive
/// it. Reaching that case needs A12 over a basis of Gaussian primes wider
/// than the alphabet's own, which is 37.7's adjunction (L11) and not this
/// row.
///
/// THE GAUGE IS AN ARGUMENT (L12), because the lane a target occupies is
/// not a function of its versor alone: a carriage that recolours occupies
/// `𝒢`, and the recolouring lives in the carriage's hue map, not in the
/// versor. A caller that has no gauge passes none, and `𝒢` is then
/// unoccupied — which is the truth about a bare versor rather than an
/// absence of one.
pub fn factor_versor(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    gauge: &[(u8, u8)],
) -> Option<Result<crate::transducer::alphabet::Derivation, crate::transducer::sentence::Demand>> {
    factor_versor_target(crystal, budget, v, None, gauge)
}

/// 78.47 — WHAT A CANDIDATE ROTOR IS TO A MIND, read through that mind's OWN
/// held letters (its crystal, adjoined letters included), never the fixed
/// seventeen alone — so a class a mind could not spell reads DERIVABLE once
/// the court's adjoined letter reaches its crystal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LetterReading {
    /// The held letters spell it: derivable (78.32(c)), no new letter.
    Derivable,
    /// A versor the held letters do not spell: the letter demand (D).
    Unspelled,
    /// A versor with no plane reading to factor: unread by name.
    Unread,
    /// No versor at all (P).
    NoVersor,
}

impl LetterReading {
    /// Its kind as a refusal (78.9); `None` where it is none — derivable.
    pub fn kind(self) -> Option<crate::transducer::sentence::ObstructionKind> {
        use crate::transducer::sentence::ObstructionKind;
        match self {
            LetterReading::Derivable => None,
            LetterReading::Unspelled => Some(ObstructionKind::D),
            LetterReading::Unread => Some(ObstructionKind::Unclassified),
            LetterReading::NoVersor => Some(ObstructionKind::P),
        }
    }
}

/// The one reader of [`LetterReading`], under the stroke's budget.
pub fn letter_reading(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    rotor: &crate::transducer::alphabet::Rotor,
) -> LetterReading {
    if !rotor.is_versor() {
        return LetterReading::NoVersor;
    }
    let Some(v) = crate::transducer::element::versor_of_word(&crate::transducer::alphabet::Word::formed(rotor.clone()))
    else {
        return LetterReading::Unread;
    };
    match factor_core(crystal, budget, &v, None, &[]) {
        Some(Ok(_)) => LetterReading::Derivable,
        Some(Err(_)) => LetterReading::Unspelled,
        None => LetterReading::Unread,
    }
}

/// Factor a versor with an explicit multi-plane Cl(4,0) target rotor, opening
/// kinematics to all six rotation planes.
pub fn factor_versor_target(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    target_rotor: Option<&crate::transducer::alphabet::Rotor>,
    gauge: &[(u8, u8)],
) -> Option<Result<crate::transducer::alphabet::Derivation, crate::transducer::sentence::Demand>> {
    Some(factor_core(crystal, budget, v, target_rotor, gauge)?.map_err(Unspelled::demand))
}

/// [`factor_versor_target`], what the peel left standing returned untyped:
/// a reader that asks only whether the mind's letters reach a word
/// ([`letter_reading`]) never types the demand (78.113).
fn factor_core(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    target_rotor: Option<&crate::transducer::alphabet::Rotor>,
    gauge: &[(u8, u8)],
) -> Option<Result<crate::transducer::alphabet::Derivation, Unspelled>> {
    let hue = crate::transducer::sentence::CycleType::of(gauge);
    let adjoined = adjoined_basis_target(crystal, budget, v, target_rotor, &hue)?;
    Some(match peel_core(v, target_rotor, adjoined.clone())? {
        Ok((cited, residue)) if cited.is_empty() => {
            let occupied = occupied_lanes(&residue.rotor(), &hue);
            Ok(derivation_of(crystal, budget, residue, &occupied)?)
        }
        Ok((cited, residue)) => {
            let composed = compose_citations(&adjoined, &cited, &residue)?;
            let target = match target_rotor {
                Some(r) => Some(r.clone()),
                None => rotor_of_versor(v),
            };
            match target.is_some_and(|t| composed.projective_eq(&t)) {
                true => Ok(crate::transducer::alphabet::Derivation::factored(
                    cited, residue.geodesic(), composed,
                )),
                // Named and counted where it forms (A.17, 78.9).
                false => {
                    let _ = crate::transducer::sentence::ObstructionRefusal::CitationsDoNotCompose.noted();
                    return None;
                }
            }
        }
        Err(demand) => Err(demand),
    })
}

/// 42.6 AS AMENDED, L12 — WHICH LANES A DERIVATION'S TARGET CARRIES
/// CONTENT ON, and the chart it carries it on in each.
///
/// `𝒽` IS THE ROTOR's own conjugacy axes. `𝒢` IS THE GAUGE: a target that
/// recolours while it turns is not derived by a generator that only turns
/// — the recolouring is content the citation does not account for — so the
/// hue cycle type is read and the `𝒢` cover is asked.
///
/// `𝒮` IS NOT A COORDINATE OF A DERIVATION, and the carriage is therefore
/// the IDENTITY here and not the target's own. A derivation claims a
/// ROTOR: `factor_versor` replays the citations against
/// `rotor_of_versor(v)` and against nothing else, and `word_vertex_with`
/// puts the placement and dilation on `Vertex::Word.carriage`, beside the
/// derivation rather than inside it. 18.2 quotients the body's absolute
/// position, so a body's own placement is not something a generator must
/// divide. Reading the target's carriage here — measured — refused the
/// 180° turn's citation of a 90°-turn generator in `generator_pay`,
/// because the target translated and no generator is held on `𝒮`: a
/// refusal by bookkeeping, of a claim the derivation never made.
///
/// THE READ AND THE ADMISSION USE ONE CLASS VECTOR. `class_of_vertex`
/// files a generator under the same rotor-and-gauge reading this asks
/// with; a generator filed under one key and looked up under another is
/// held and unreachable, which is indistinguishable from not held.
///
/// A target whose rotor has no class at all occupies nothing, and a basis
/// is not invented for it (rule 11).
pub(crate) fn occupied_lanes(
    rotor: &crate::transducer::alphabet::Rotor,
    hue: &crate::transducer::sentence::CycleType,
) -> Vec<(usize, crate::transducer::stalk::Invariant)> {
    let carriage = crate::transducer::element::identity_versor();
    match crate::transducer::sentence::ClassVector::of_rotor(rotor, carriage, hue.clone()) {
        Some(cv) => crate::transducer::sentence::lane_charts_of(&cv, rotor),
        None => Vec::new(),
    }
}

/// The rotor a versor's base action is (51.2): exact on the three base
/// planes, read whole and never off one plane's block.
fn rotor_of_versor(v: &Versor) -> Option<crate::transducer::alphabet::Rotor> {
    crate::transducer::element::base_rotor_of(v).ok()
}

/// 37.7, L23a, L12 — THE ADJOINED GENERATORS THIS MIND HOLDS at the meet
/// of the target's lanes, as primes the peel may divide by. A generator
/// that spells in letters is A12's and not the basis's; one whose rotor
/// has no arc on the sensor plane divides nothing here.
pub fn adjoined_basis(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    hue: &crate::transducer::sentence::CycleType,
) -> Option<Vec<crate::transducer::alphabet::AdjoinedPrime>> {
    adjoined_basis_target(crystal, budget, v, None, hue)
}

pub(crate) fn adjoined_basis_target(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    target_rotor: Option<&crate::transducer::alphabet::Rotor>,
    hue: &crate::transducer::sentence::CycleType,
) -> Option<Vec<crate::transducer::alphabet::AdjoinedPrime>> {
    use crate::transducer::alphabet::{gauss_of_block, rotor_block_on, AdjoinedPrime, SLOT_PLANES_SIX};
    let target = match target_rotor {
        Some(r) => r.clone(),
        None => match rotor_of_versor(v) {
            Some(r) => r,
            None => return Some(Vec::new()),
        },
    };
    // THE LANES THE TARGET OCCUPIES (42.6 as amended, L12), and the basis
    // at the lanes' MEET. `3` was written here as a literal, which asserted
    // that every target is holonomic and nothing else: a target that
    // RECOLOURS while it turns occupies `𝒢` too, and reducing it against a
    // generator held only on `𝒽` divides out the turn and leaves the
    // recolouring, asserting a bound the presentation does not give. The
    // meet admits a generator only where every occupied lane holds it.
    let occupied = occupied_lanes(&target, hue);
    let mut out = Vec::new();
    for (form, vertex, g) in crystal.generators_at_meet(&occupied, budget) {
        if !g.letters().is_empty() {
            continue;
        }
        for &plane in &SLOT_PLANES_SIX {
            let Some(block) = rotor_block_on(&g.rotor(), plane) else { continue };
            let Some(pi) = gauss_of_block(&block) else { continue };
            if pi.is_unit() || pi.is_zero() {
                continue;
            }
            out.push(AdjoinedPrime { pi, form: form.clone(), vertex: vertex.clone(), rotor: g.rotor() });
        }
    }
    Some(out)
}

/// The rotor the citations and the residue compose to: each generator's
/// rotor at its exponent, then the residue's own.
fn compose_citations(
    basis: &[crate::transducer::alphabet::AdjoinedPrime],
    cited: &[crate::transducer::alphabet::Cited],
    residue: &crate::transducer::alphabet::Word,
) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::alphabet::{rotor_power, Rotor};
    let mut composed = Rotor::identity();
    for (form, vertex, e) in cited {
        let held = basis
            .iter()
            .find(|g| &g.form == form && &g.vertex == vertex)
            .map(|g| g.rotor.clone())?;
        composed = composed.geo(&rotor_power(&held, *e)?);
    }
    Some(composed.geo(&residue.rotor()))
}

/// 50.2 step 2 applied to an ALREADY PEELED vertex: the second step run
/// on the memo's percept, per mind. A derivation that already cites
/// something is not re-derived — it is another mind's proof and this one
/// does not overwrite it.
///
/// The `𝒢` lane comes from the VERTEX, not from its word: `gauge` is the
/// vertex's own hue map, and it is one of the two coordinates that decide
/// which covers this target must reduce against (L12). Its placement is
/// not one of them — see `occupied_lanes`.
pub(crate) fn derivation_of_spelled(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    d: &crate::transducer::alphabet::Derivation,
    gauge: &[(u8, u8)],
) -> Option<crate::transducer::alphabet::Derivation> {
    match d.is_spelled() {
        true => {
            let hue = crate::transducer::sentence::CycleType::of(gauge);
            let occupied = occupied_lanes(&d.residue.rotor(), &hue);
            derivation_of(crystal, budget, d.residue.clone(), &occupied)
        }
        false => Some(d.clone()),
    }
}

/// A22 alone on a measured carriage: the ordered peel with its remainder
/// typed, and no presentation in it. This is what an EDGE reads (50.3) —
/// an edge transport is the coboundary between two vertices' placements,
/// a relation and not a vertex, so it has no derivation to carry and none
/// is invented for it. Every vertex goes through `factor_versor`.
pub(crate) fn peel_versor(
    v: &Versor,
) -> Option<Result<crate::transducer::alphabet::Word, crate::transducer::sentence::Demand>> {
    Some(match peel_versor_with(v, Vec::new())? {
        Ok((_, w)) => Ok(w),
        Err(d) => Err(d),
    })
}

/// WHAT A CLOSED PEEL PRODUCED: the citations it divided out and the
/// letters that remain. A `Derivation`'s two halves.
pub(crate) type Peeled = (Vec<crate::transducer::alphabet::Cited>, crate::transducer::alphabet::Word);

/// A22 OVER A BASIS (37.7, L23a): the ordered peel with the alphabet's own
/// primes AND the adjoined generators the caller holds. What comes back is
/// a DERIVATION's two halves — the citations the peel divided out and the
/// letters that remain — or the demand the peel typed.
///
/// The ring branch was this, run a second time on a target this peel had
/// already refused, from a second basis, in another file. One loop, one
/// basis. A caller that passes no generators reads exactly what
/// `peel_versor` always read.
pub(crate) fn peel_versor_with(
    v: &Versor,
    adjoined: Vec<crate::transducer::alphabet::AdjoinedPrime>,
) -> Option<Result<Peeled, crate::transducer::sentence::Demand>> {
    peel_versor_with_target(v, None, adjoined)
}

/// A22 OVER A BASIS, BY THE COMMUTING PAIRS OF 50.4. The target rotor is
/// split by the central idempotents into its two quaternion factors, each
/// peeled in Tait–Bryan order, so every block of `BLOCK_ORDER` yields two
/// commuting plane rotors; each is spelled on its own plane by the peel
/// over the alphabet's primes and the generators this mind adjoined. A
/// block the factors do not decompose, or a plane whose arc the basis does
/// not spell, is the demand it leaves (50.5); a target that is not even is
/// no rotor and answers nothing (16.1). A target that turns no plane is the
/// identity word: a carriage that only places and dilates spells nothing.
pub(crate) fn peel_versor_with_target(
    v: &Versor,
    target_rotor: Option<&crate::transducer::alphabet::Rotor>,
    adjoined: Vec<crate::transducer::alphabet::AdjoinedPrime>,
) -> Option<Result<Peeled, crate::transducer::sentence::Demand>> {
    Some(peel_core(v, target_rotor, adjoined)?.map_err(Unspelled::demand))
}

/// 78.113 — WHAT A PEEL LEFT STANDING, BEFORE IT IS TYPED AS A DEMAND. A
/// standing arc's demand names the least prime of its norm outside the
/// alphabet, found by trial division up to `√n` — and a reader that asks
/// only whether a word spells dropped that prime unread: 76% of a family's
/// hold on the development deployment's stalled task (`50846271`, sampled) was spent finding
/// primes no caller kept. The arc is held here as its exact coordinates
/// (its plane, `p`, `q`, the block, the target) and typed only by a caller
/// that keeps the demand (law 12: never computed to be dropped).
pub(crate) enum Unspelled {
    /// A demand already typed, at no search: a standing block.
    Typed(crate::transducer::sentence::Demand),
    /// A plane whose arc the basis did not spell, its prime not yet read.
    Arc {
        plane: (u8, u8),
        p: num_bigint::BigInt,
        q: num_bigint::BigInt,
        block: Box<[[num_rational::BigRational; 2]; 2]>,
        target: Box<crate::transducer::alphabet::Rotor>,
    },
}

impl Unspelled {
    /// The demand it leaves (37.7 P2): the least prime outside the alphabet
    /// with the residue block the shells left as its witness, or the
    /// standing block where only units or half letters remain.
    pub(crate) fn demand(self) -> crate::transducer::sentence::Demand {
        use crate::transducer::sentence::Demand;
        match self {
            Unspelled::Typed(d) => d,
            Unspelled::Arc { plane, p, q, block, target } => {
                let norm = &p * &p + &q * &q;
                match crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane) {
                    Some(prime) => Demand::RotorPrime {
                        plane,
                        prime,
                        witness: Box::new(crate::transducer::alphabet::arc_block(&p, &q)),
                    },
                    None => Demand::Standing { block, rotor: target },
                }
            }
        }
    }
}

/// 78.113 — WHETHER A VERSOR SPELLS in the alphabet's own letters, and
/// nothing else: the peel, its residue never typed. `None` where it reads
/// nothing (no rotor, no linear block).
#[cfg(test)]
pub(crate) fn spells(v: &Versor) -> Option<bool> {
    Some(peel_core(v, None, Vec::new())?.is_ok())
}

/// The peel itself ([`peel_versor_with_target`]), what it left standing
/// returned untyped ([`Unspelled`]).
fn peel_core(
    v: &Versor,
    target_rotor: Option<&crate::transducer::alphabet::Rotor>,
    adjoined: Vec<crate::transducer::alphabet::AdjoinedPrime>,
) -> Option<Result<Peeled, Unspelled>> {
    use crate::transducer::alphabet::{
        factors, is_identity_block, pair_rotors, rotor_block_on, spell_peel_rotor_with, standing,
        tait_bryan, BLOCK_ORDER,
    };
    use crate::transducer::peel::PeelNest;
    use crate::transducer::sentence::Demand;

    // THE DE-COMPACTION'S INVERSION (AL1 as amended): an odd versor is the
    // signal read orientation-reversed — its even part `V·e1`, the reflector
    // undone, times the chart's orientation character `e₀₂` — never a letter
    // the language lacks. The even part is peeled as any versor is, and the
    // orientation character is the alphabet's own 180° turn through the third
    // axis; the word is read back against the versor's own rotor as one ray,
    // and one that does not compose back is no reading.
    if target_rotor.is_none() && v.parity() == Some(true) {
        let even = v.geo(&crate::transducer::cga::reflector());
        let orientation = crate::transducer::alphabet::LetterId::turn_90((0, 2))?;
        let flip = crate::transducer::alphabet::Word::from_letters(&[(orientation, false), (orientation, false)]);
        let whole = rotor_of_versor(v)?;
        return Some(match peel_core(&even, None, adjoined)? {
            Ok((cited, w)) => {
                let word = w.compose(&flip);
                if !word.rotor().projective_eq(&whole) {
                    return None;
                }
                Ok((cited, word))
            }
            Err(d) => Err(d),
        });
    }
    let target = match target_rotor {
        Some(r) => r.clone(),
        None => rotor_of_versor(v)?,
    };
    // D2 — THE ORBIT STRUCTURE, at the same site and for the same reason:
    // the cyclotomic period `μ` is CARRIED BY, rather than a scalar it would
    // be flattened into. READ OFF THE MOTION'S OWN MODES (PHY1), never the
    // rotation's order alone: a transvection or a null shear in the motion
    // never returns, and the rotor's order counted it closed. Read before
    // the gates so an orbit that never closes is counted as the reading it
    // is rather than lost as a refusal; the census reads on a stroke of its
    // own.
    let mut census = crate::physics::Physics::default().stroke();
    let period = match crate::transducer::cga::motion_modes(v, &mut census) {
        Some(Ok(modes)) => crate::transducer::cga::period_of(&modes),
        Some(Err(_)) | None => None,
    };
    crate::physics::metabolism::note_orbit(period);
    target.is_versor().then_some(())?;
    if !target.is_even() {
        return None;
    }
    let (qp, qm) = factors(&target);
    let (Some(tp), Some(tm)) = (tait_bryan(&qp), tait_bryan(&qm)) else {
        return Some(Err(Unspelled::Typed(standing(&target, BLOCK_ORDER[0].0)?)));
    };
    let mut cited_all: Vec<crate::transducer::alphabet::Cited> = Vec::new();
    let mut word = crate::transducer::alphabet::Word::identity();

    for (k, (pa, pb)) in BLOCK_ORDER.iter().enumerate() {
        let Some((ra, rb)) = pair_rotors(k, &tp[k], &tm[k]) else {
            return Some(Err(Unspelled::Typed(standing(&target, *pa)?)));
        };
        for (plane, rotor) in [(*pa, ra), (*pb, rb)] {
            let block = rotor_block_on(&rotor, plane)?;
            if is_identity_block(&block) {
                continue;
            }
            match spell_peel_rotor_with(plane, &block, adjoined.clone()) {
                Some((PeelNest::Closed { cores }, _rho)) => {
                    let (cited, w) = crate::transducer::alphabet::cited_of_cores(&cores)?;
                    cited_all.extend(cited);
                    word = word.compose(&w);
                }
                Some((PeelNest::Standing { arc, .. }, _)) => {
                    // 37.7 P2: the witness is the residue block the shells
                    // left, not the block the peel started from — typed by
                    // the caller that keeps it (`Unspelled::demand`).
                    return Some(Err(Unspelled::Arc {
                        plane,
                        p: arc.p,
                        q: arc.q,
                        block: Box::new(block),
                        target: Box::new(target.clone()),
                    }));
                }
                None => {
                    return Some(Err(Unspelled::Typed(Demand::Standing {
                        block: Box::new(block),
                        rotor: Box::new(target.clone()),
                    })))
                }
            }
        }
    }

    Some(Ok((cited_all, word)))
}

/// A12 over the mind's own generators (50.2 step 2, 42.6, L12). The basis
/// is the presentation AT THE MEET of the lanes this target occupies —
/// `occupied`, computed by the caller from the vertex's own class vector —
/// so the read is the stratum this word acts on and nothing else.
/// `Understood` is a word wholly in the span — the citations alone reach
/// it, and the residue is the identity. `Novel` keeps the part that
/// factored and leaves A12's canonical remainder, which is unique and has
/// no choice in it (rule 4), as the residue in letters. `Unmeasured` is a
/// budget that ended: it is not a demand and not a spelling but `None`,
/// because "this word cites nothing" is a claim about the presentation and
/// a budget that ended made no such claim (rule 11).
///
/// A word with no plane form IS measured: it is off the plane A12 reduces
/// on, so the presentation provably reached none of it and `spelled` is
/// the true answer. So is a word occupying no lane — the identity, which
/// has nothing to reduce — and `generators_at_meet` answers empty there.
/// `versor_chart` is not read here any more: it is the `𝒽` lane's chart
/// alone, and reading the basis off it was the union L12 refuses.
fn derivation_of(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    w: crate::transducer::alphabet::Word,
    occupied: &[(usize, crate::transducer::stalk::Invariant)],
) -> Option<crate::transducer::alphabet::Derivation> {
    use crate::transducer::alphabet::{Derivation, Word};
    use crate::transducer::factor::Factored;
    // EVERY RESIDUE LEAVES HERE AS ITS GEODESIC (Expansion 1). The word a
    // vertex carries on the tape is the one the court replays by rotor
    // and the one every other mind reads; two minds deriving one rotor
    // write one tape when both reduce under the same completed
    // presentation. A12's plane remainder is already canonical on its
    // plane; the reduction reaches what A12 does not — the commutators
    // and the sheet across planes — and leaves a single-plane word as it
    // is.
    let spelled = || Some(Derivation::spelled(w.geodesic()));
    if w.is_identity() {
        return spelled();
    }
    let held = crystal.generators_at_meet(occupied, budget);
    let target_rotor = w.rotor();

    // 1. DIRECT GENERATOR FACTORIZATION: check whether the word projectively
    // matches a held generator across ANY plane (including off-plane), without
    // flattening to the sensor plane.
    for (form, vertex, g) in &held {
        let gr = g.rotor();
        if gr.projective_eq(&target_rotor) {
            return Some(Derivation::compose(
                &[((*form).clone(), (*vertex).clone(), 1, (*g).clone())],
                Word::identity(),
            ));
        }
        if gr.is_versor() && gr.reverse().projective_eq(&target_rotor) {
            return Some(Derivation::compose(
                &[((*form).clone(), (*vertex).clone(), -1, (*g).clone())],
                Word::identity(),
            ));
        }
    }

    let Some(target) = w.form() else {
        return spelled();
    };
    // A generator with no plane form is off the plane A12 reduces on and
    // is not in the basis. The citation and its plane form are taken in
    // ONE pass, so the index A12 returns names the same generator in both
    // lists by construction — never by two lists agreeing on a length.
    let cited: Vec<_> = held
        .iter()
        .filter_map(|(form, vertex, g)| g.form().map(|pf| (form, vertex, g, pf)))
        .collect();
    let basis: Vec<_> = cited.iter().map(|(_, _, _, pf)| *pf).collect();
    let one = |tape: &[(usize, i64)], residue: Word| -> Derivation {
        let mut with_words = Vec::new();
        for (i, e) in tape {
            let Some((form, vertex, g, _)) = cited.get(*i) else {
                return Derivation::spelled(w.geodesic());
            };
            with_words.push(((*form).clone(), (*vertex).clone(), *e, (*g).clone()));
        }
        Derivation::compose(&with_words, residue.geodesic())
    };
    let derived = match crate::transducer::factor::factor_on_plane(&target, &basis) {
        Factored::Understood(tape) => one(&tape, Word::identity()),
        Factored::Novel { remainder, understood } => one(&understood, remainder.to_word()),
        Factored::Unmeasured => return None,
    };
    // ROTOR REPLAY VERIFICATION: ensure the plane factorisation projectively
    // reproduces the target word's 4D Clifford rotor action. A non-abelian
    // product or twist that diverges from the true rotor is refused and
    // falls back to the geodesic spelling.
    if !derived.rotor().projective_eq(&target_rotor) {
        return spelled();
    }
    Some(derived)
}

/// 78.161 — A WORD SAID THROUGH THE WORDS THE MIND HOLDS: its whole motion
/// (its carriage composed with its rotor's base action, Cl(4,1)) and its hue
/// map read over the ruled words, lane by lane (`Crystal::read_word`), and
/// failing that what the mind's court answered it for the motion
/// (`Crystal::presented`). The citation stands only where this mind
/// re-derives it: one exponent per cited generator, the powers of the cited
/// motions multiplying to the vertex's motion exactly, the cited rotors to its
/// rotor, and the cited hue maps composing to its hue map. Two words reach one
/// vertex by different derivations (1.5): what is checked is the mathematics
/// adding up, never that the words are the same.
pub(crate) fn heard_citation(
    crystal: &dyn crate::transducer::family::Crystal,
    rotor: &crate::transducer::alphabet::Rotor,
    carriage: &Versor,
    gauge: &[(u8, u8)],
) -> Option<crate::transducer::alphabet::Derivation> {
    use crate::transducer::sentence::presentation::{gauge_power, motion_power, vertex_motion};
    let motion = crate::transducer::element::conformal_of_rotor(rotor, carriage)?;
    let heard = crystal.read_word(&motion, gauge).filter(|r| r.composes);
    let cites: Vec<(crate::transducer::address::Address, crate::transducer::address::Address, i64)> = match heard {
        Some(r) => {
            // ONE EXPONENT PER GENERATOR across its lanes: a generator cited
            // at two exponents on two lanes is no one power of it.
            let mut one: Vec<(crate::transducer::address::Address, crate::transducer::address::Address, i64)> = Vec::new();
            for (f, v, _, e) in r.cites {
                match one.iter().find(|(g, w, _)| *g == f && *w == v) {
                    Some((_, _, held)) if *held == e => {}
                    Some(_) => return None,
                    None => one.push((f, v, e)),
                }
            }
            one
        }
        None if gauge.is_empty() => crystal.presented(&motion)?,
        None => return None,
    };
    let mut by_motion = Versor::scalar(num_rational::BigRational::from_integer(1.into()));
    let mut by_rotor = crate::transducer::alphabet::Rotor::identity();
    let mut by_gauge: Vec<(u8, u8)> = Vec::new();
    let mut generators = Vec::with_capacity(cites.len());
    for (form, vertex, e) in &cites {
        let (_, f) = crystal.by_address(form.as_bytes())?;
        let v = crate::transducer::sentence::vertex_cited(&f, vertex.as_bytes())?;
        let Vertex::Word { derivation, acts, .. } = v else { return None };
        by_motion = by_motion.geo(&motion_power(&vertex_motion(v)?, *e));
        by_rotor = by_rotor.geo(&crate::transducer::alphabet::rotor_power(&derivation.rotor(), *e)?);
        let held_gauge = match acts.scoped().next() {
            Some((_, a)) => a.gauge.clone(),
            None => Vec::new(),
        };
        by_gauge = crate::transducer::sentence::compose_gauge(&gauge_power(&held_gauge, *e), &by_gauge);
        generators.push((form.clone(), vertex.clone(), *e));
    }
    if !by_motion.projective_eq(&motion) || !by_rotor.projective_eq(rotor) || !crate::transducer::sentence::gauge_eq(&by_gauge, gauge) {
        return None;
    }
    Some(crate::transducer::alphabet::Derivation::factored(
        generators,
        crate::transducer::alphabet::Word::identity(),
        rotor.clone(),
    ))
}
