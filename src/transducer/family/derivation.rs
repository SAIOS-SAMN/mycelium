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
use crate::transducer::element::sensor_plane_block;

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
    let hue = crate::transducer::sentence::CycleType::of(gauge);
    // THE BASIS THIS MIND PEELS WITH (L23a): the seventeen, and whatever
    // it has adjoined at the meet of the lanes this target occupies. There
    // is no second branch after the peel any more — the ring WAS that
    // branch, running a second division loop over a second basis on a
    // target this peel had already refused, and it is the same loop.
    let adjoined = adjoined_basis(crystal, budget, v, &hue)?;
    Some(match peel_versor_with(v, adjoined.clone())? {
        // `None` from the factorisation is the budget ending, which is
        // UNMEASURED and not a spelling: answering `spelled(w)` there
        // would assert "no held generator reached this" without having
        // looked, which is exactly the invented value rule 11 refuses.
        // It propagates as `None` — the same refusal a versor with no
        // sensor block gives — and the caller withholds (L8w).
        //
        // A peel that cited nothing is the letter branch exactly as it
        // was: A12 over the letter exponents of the word it produced.
        Ok((cited, residue)) if cited.is_empty() => {
            let occupied = occupied_lanes(&residue.rotor(), &hue);
            Ok(derivation_of(crystal, budget, residue, &occupied)?)
        }
        // A peel that CITED is a derivation through an adjoined generator.
        // The composition is replayed against the target before it is
        // returned: the arithmetic is exact, but the map from a block to a
        // rotor is a half-angle, and a mismatch is not a guess to ship
        // (rule 11).
        Ok((cited, residue)) => {
            let composed = compose_citations(&adjoined, &cited, &residue)?;
            match rotor_of_versor(v).is_some_and(|t| composed.projective_eq(&t)) {
                true => Ok(crate::transducer::alphabet::Derivation::factored(
                    cited, residue, composed,
                )),
                false => {
                    eprintln!("  derivation refused: the citations do not compose to the target");
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
/// half turn's citation of a quarter-turn generator in `generator_pay`,
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

/// The rotor a versor's sensor block is the action of.
fn rotor_of_versor(v: &Versor) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::alphabet::{rotor_of_block, PLANE_COL, PLANE_ROW};
    rotor_of_block(&sensor_plane_block(v)?, (PLANE_ROW, PLANE_COL))
}

/// 37.7, L23a, L12 — THE ADJOINED GENERATORS THIS MIND HOLDS at the meet
/// of the target's lanes, as primes the peel may divide by. A generator
/// that spells in letters is A12's and not the basis's; one whose rotor
/// has no arc on the sensor plane divides nothing here.
fn adjoined_basis(
    crystal: &dyn crate::transducer::family::Crystal,
    budget: &mut crate::physics::Budget,
    v: &Versor,
    hue: &crate::transducer::sentence::CycleType,
) -> Option<Vec<crate::transducer::alphabet::AdjoinedPrime>> {
    use crate::transducer::alphabet::{gauss_of_block, rotor_block_on, AdjoinedPrime, PLANE_COL, PLANE_ROW};
    let plane = (PLANE_ROW, PLANE_COL);
    let Some(target) = rotor_of_versor(v) else {
        return Some(Vec::new());
    };
    // THE LANES THE TARGET OCCUPIES (42.6 as amended, L12), and the basis
    // at their MEET. `3` was written here as a literal, which asserted
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
        let Some(block) = rotor_block_on(&g.rotor(), plane) else { continue };
        let Some(pi) = gauss_of_block(&block) else { continue };
        if pi.is_unit() || pi.is_zero() {
            continue;
        }
        out.push(AdjoinedPrime { pi, form, vertex, rotor: g.rotor() });
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
    use crate::transducer::alphabet::{spell_peel_with, PLANE_COL, PLANE_ROW};
    use crate::transducer::peel::PeelNest;
    use crate::transducer::sentence::Demand;
    let block = sensor_plane_block(v)?;
    let plane = (PLANE_ROW, PLANE_COL);
    Some(match spell_peel_with(plane, &block, adjoined) {
        Some((PeelNest::Closed { cores }, _rho)) => {
            let (cited, w) = crate::transducer::alphabet::cited_of_cores(&cores)?;
            Ok((cited, w))
        }
        Some((PeelNest::Standing { arc, .. }, _)) => {
            let norm = &arc.p * &arc.p + &arc.q * &arc.q;
            match crate::transducer::alphabet::prime_outside_the_alphabet(&norm, plane) {
                // 37.7 P2: the residue block the shells left, not the
                // block the peel started from.
                Some(prime) => Err(Demand::RotorPrime {
                    plane,
                    prime,
                    witness: Box::new(crate::transducer::alphabet::arc_block(&arc.p, &arc.q)),
                }),
                None => Err(Demand::Standing { block: Box::new(block) }),
            }
        }
        None => Err(Demand::Standing { block: Box::new(block) }),
    })
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
    let spelled = || Some(Derivation::spelled(w.clone()));
    let held = crystal.generators_at_meet(occupied, budget);
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
                return Derivation::spelled(w.clone());
            };
            with_words.push(((*form).clone(), (*vertex).clone(), *e, (*g).clone()));
        }
        Derivation::compose(&with_words, residue)
    };
    match crate::transducer::factor::factor_on_plane(&target, &basis) {
        Factored::Understood(tape) => Some(one(&tape, Word::identity())),
        Factored::Novel { remainder, understood } => Some(one(&understood, remainder.to_word())),
        Factored::Unmeasured => None,
    }
}
