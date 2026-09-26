// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 3.8, 3.9 — THE BUNDLE SYNTHESIS ENGINE: an optimization search over the fiber bundle
//! E -> B over the chart complex.
//!
//! Rather than treating vocabulary as an unorganized heap of monolithic tokens and
//! issuing crude deletions, the bundle synthesis engine extracts the connection geometry:
//! 1. Change of Basepoint / Conjugate Overlay (Reanchor): groups sections sharing the same
//!    conjugacy class or orbit holonomies, solves for connecting gauge transitions γ ∈ Spin(4,0),
//!    and emits RewritePayload::Reanchor to earn metabolic yield on LANE_SPATIAL.
//! 2. Fiber Section Condensation (Condense): folds sibling forms projecting to the same
//!    base cycle into sections of a single punctured bundle, emitting RewritePayload::Condense
//!    to earn metabolic yield on LANE_GAUGE.
//! 3. Axiomatic Floor (Peel/Refine): factors adjoined words into the 17 Cl(4,0) generators
//!    via spell_peel / Gaussian factoring, dissolving factorable words into zero basis slots.

use std::io;
use crate::anastome::dialogue;
use crate::client::sdk::*;
use crate::transducer::address::Address;
use crate::transducer::alphabet::{PlaneForm, Rotor, Word};
use crate::transducer::element::Element;
use crate::transducer::rewrite::{RewriteKind, RewritePayload};

impl Entity {
    /// CHANGE OF BASEPOINT / CONJUGATE OVERLAY (RewriteKind::Reanchor)
    ///
    /// Probes the genome for gauge frame transitions γ ∈ Spin(4,0):
    /// 1. Sentence coboundary gauge transitions aligning local sections to basepoint.
    /// 2. Word conjugacy classes sharing `class_key()`, solving for connecting gauge rotor γ.
    ///
    /// Returns:
    /// - The minted `Rewrite` element.
    /// - Vector of addresses to retire from the genome on acceptance.
    /// - Optional new element to learn into the genome on acceptance (e.g. regauged sentence).
    pub fn probe_conjugate_reanchor(&mut self) -> Option<(Element, Vec<Address>, Option<Element>)> {
        // ONE STROKE'S WORTH OF CANDIDATES, and the cursor carries the rest
        // to the next stroke (rule 10 — a loop's bound is a Budget or one
        // simplex's boundary, and a genome is neither). `physics.stroke()` is
        // the engine's own unit of work — the same budget `pivot_on` spends
        // — so the probe is paced by the same arithmetic as every other
        // stroke and not by a number chosen here.
        let budget = self.physics.stroke();
        self.probe_conjugate_reanchor_under(budget)
    }

    /// The same probe under a STATED budget. The stroke is calibrated from
    /// the machine (`reserve_max`, 51.11), so a claim about what the probe
    /// finds is a claim under a budget the claim names, never under
    /// whatever the box had free when the test ran.
    pub fn probe_conjugate_reanchor_under(
        &mut self,
        mut budget: crate::physics::Budget,
    ) -> Option<(Element, Vec<Address>, Option<Element>)> {
        // THE CIRCUIT AS COORDINATES (45.7): every genome word once from
        // where this circuit last stood, wrapping to close on it.
        let circuit = self.epigenome.genome_circuit_from(self.bundle_cursor.as_deref());
        // Where this circuit now stands: the circuit's opening coordinate, which
        // is the trie's own successor to the last and is content, not a
        // member picked out of an arrival order.
        let [from, ..] = circuit.as_slice() else { return None };
        self.bundle_cursor = Some(from.clone());

        // 1. THE SENTENCE REGAUGE STOOD HERE AND RETIRED (AT1 step 2, ruled
        // (a)): it solved a sentence's tree gauge and re-minted the regauged
        // form. A face's word is derived from its ends now, so there is no
        // gauge to solve and no regauged form to mint.

        // 2. Word pairs sharing the same conjugacy class (class_key):
        // For words w1, w2 with w1.class_key() == w2.class_key(),
        // solve for connecting gauge rotor γ such that w2 = γ * w1 * γ^-1.
        for at in &circuit {
            if !budget.spend() {
                break;
            }
            let Some(h2) = self.epigenome.word_at(at) else { continue };
            let Some(letters2) = h2.letters() else { continue };
            let w2 = letters2.spelling.word.clone();
            let Some(ck2) = w2.class_key() else { continue };

            // THE SIBLINGS ARE READ AT A COORDINATE, not filtered out of the
            // genome. This read every word the mind holds and kept the ones
            // whose `class_key` matched — the whole carrier per candidate,
            // which is rule 1 and law 3, and unbounded by any Budget.
            //
            // The conjugacy class IS the coordinate. `versor_chart` is
            // `[0, class_axes…]` (M4 step 2, 15.3) and the atlas is keyed by
            // `invariant_of`, which for a connection is `connection_chart` —
            // `[0, class_axes…, cycle lengths…]`, of which the versor chart
            // is a PREFIX.
            //
            // `fiber_under`, NOT `fiber`. `Stratum::fiber` reaches the
            // node at that exact path and answers the members standing
            // THERE; `fiber_under` collects the subtree below it. Two words
            // of one conjugacy class at different hues sit at different
            // depths under the same versor chart, so the exact read answers
            // the empty fibre for both — measured: the word-pair fixture
            // found no conjugate at all.
            //
            // The chart PREFILTERS and the class key still decides: the
            // subtree is the stratum, and `ck1 != ck2` below is the exact
            // re-derivation that confirms a member of it (R15's ruling —
            // a key narrows the search and never certifies a match).
            let Some(vc) = crate::transducer::sentence::versor_chart(&w2) else { continue };
            for addr in self.epigenome.atlas().fiber_under(&vc) {
                let Some(h1) = self.epigenome.word_at(&addr) else { continue };
                let Some(letters1) = h1.letters() else { continue };
                let w1 = letters1.spelling.word.clone();
                let Some(ck1) = w1.class_key() else { continue };
                if ck1 != ck2 || h1.address() == h2.address() {
                    continue;
                }

                if let Some(gamma) = solve_gauge_between(&w1, &w2) {
                    let payload = RewritePayload::Reanchor { gamma: vec![gamma] };
                    let claimed = [1, 0, 0, 0];
                    if let Some(rw) = crate::transducer::rewrite::mint(
                        RewriteKind::Reanchor,
                        std::slice::from_ref(&h2.address()),
                        h1.address(),
                        payload,
                        claimed,
                    ) {
                        return Some((rw, vec![h2.address()], None));
                    }
                }
            }
        }

        None
    }

    /// FIBER SECTION CONDENSATION (RewriteKind::Condense)
    ///
    /// Folds sibling forms projecting to the same base cycle into sections of
    /// a single punctured bundle over the chart complex.
    ///
    /// Returns:
    /// - The minted `Rewrite` element with non-zero yield on LANE_GAUGE.
    /// - Vector of addresses to retire from the genome on acceptance (absorbed sibling).
    /// - Optional updated base bundle element to learn on acceptance.
    pub fn probe_bundle_condensation(&mut self) -> Option<(Element, Vec<Address>, Option<Element>)> {
        // One stroke's worth of candidates, as `probe_conjugate_reanchor`
        // takes: the cursor carries the rest, and a genome is neither a
        // Budget nor one simplex's boundary (rule 10).
        let budget = self.physics.stroke();
        self.probe_bundle_condensation_under(budget)
    }

    /// The same probe under a stated budget (see `probe_conjugate_reanchor_under`).
    pub fn probe_bundle_condensation_under(
        &mut self,
        mut budget: crate::physics::Budget,
    ) -> Option<(Element, Vec<Address>, Option<Element>)> {
        if self.epigenome.genome_len() < 2 {
            return None;
        }
        let circuit = self.epigenome.genome_circuit_from(self.bundle_cursor.as_deref());
        // Where this circuit now stands: the circuit's opening coordinate, which
        // is the trie's own successor to the last and is content, not a
        // member picked out of an arrival order.
        let [from, ..] = circuit.as_slice() else { return None };
        self.bundle_cursor = Some(from.clone());

        for at in &circuit {
            if !budget.spend() {
                break;
            }
            let Some(h1) = self.epigenome.word_at(at) else { continue };
            if h1.sub().is_empty() {
                continue;
            }
            let inv1 = crate::transducer::stalk::invariant_of(h1);

            // THE SIBLINGS OF A CHART ARE READ AT THAT CHART. This read the
            // whole genome and kept the words whose `invariant_of` matched
            // the one already in hand — the chart was computed, then used as
            // a filter over everything instead of as the coordinate it is.
            // `Stalks::fiber` answers exactly that stratum in one keyed
            // read by the prefix (rule 1, law 3).
            for addr in self.epigenome.atlas().fiber(&inv1) {
                let Some(h2) = self.epigenome.word_at(&addr) else { continue };
                if h2.sub().is_empty() || h1.address() == h2.address() {
                    continue;
                }

                // Solve transition versor between sibling forms projecting to same chart
                let Some(trans) = crate::transducer::element::transition(h2.sub(), h1.sub()) else {
                    continue;
                };

                let mut base = (**h1).clone();
                let card = (**h2).clone();
                let gained = base.attach(&card, &trans);
                if gained > 0 {
                    let payload = RewritePayload::Condense {
                        sections: base.sections().to_vec(),
                        gauge: trans,
                    };
                    let joins = 1u64;
                    let gained_sec = base.sections().len() as u64;
                    let claimed = [0, gained_sec.max(joins), 0, 0];
                    if let Some(rw) = crate::transducer::rewrite::mint(
                        RewriteKind::Condense,
                        std::slice::from_ref(&card.address()),
                        base.address(),
                        payload,
                        claimed,
                    ) {
                        return Some((rw, vec![card.address()], Some(base)));
                    }
                }
            }
        }

        None
    }

    /// SUBMIT BUNDLE REWRITE TO THE COURT
    ///
    /// Speaks the rewrite element to the court. When stood or ratified,
    /// retires the absorbed/reanchored addresses locally, learns any updated
    /// target section, and credits the syntropic yield on the entity's physics.
    pub fn bid_bundle_rewrite(
        &mut self,
        rw: &Element,
        retires: &[Address],
        after: Option<&Element>,
    ) -> io::Result<[u64; 4]> {
        let codes = self.speak(std::slice::from_ref(rw))?;
        if let [code, ..] = codes.as_slice() {
            if *code == dialogue::SPEECH_STOOD || *code == dialogue::SPEECH_RATIFIED {
                for b in retires {
                    let _ = self.epigenome.retire(b.as_bytes());
                }
                // THE UPDATED SECTION IS ADMITTED BY AN ACT ON THIS VOICE'S
                // LINE (78.27, LD3-c): learned on the court's standing of
                // the rewrite, it is sealed as Learned, which admits it; a
                // section the genome refuses is said, never discarded.
                if let Some(after_elem) = after {
                    match after_elem.to_bytes() {
                        None => eprintln!(
                            "  bundle rewrite: the updated section's record passes a length its field writes"
                        ),
                        Some(record) => match self.epigenome.learn(&record) {
                            Some(_) => {
                                self.seal_word(crate::chronometry::ActKind::Learned, after_elem);
                            }
                            None => eprintln!("  bundle rewrite: the updated section was not learned"),
                        },
                    }
                }
                let mu = rw.magnitude();
                self.physics.syntropy(&mu);
                return Ok(mu);
            }
        }
        Err(io::Error::other("bundle rewrite not accepted by court"))
    }
}

/// Solves for connecting gauge rotor γ in Spin(4,0) (as a Word) connecting two
/// conjugate words w1 and w2 such that w2 = γ · w1 · γ⁻¹.
/// Solves for connecting gauge rotor γ in Spin(4,0) (as a Word) connecting two
/// conjugate words w1 and w2 such that w2 = γ · w1 · γ⁻¹.
///
/// Exact closed-form in Cl(4,0) coordinates — zero Cartesian enumeration or search loops:
/// 1. Sheet plane 180°-turn conjugates a rotation to its inverse (w2 = w1⁻¹ in-plane).
/// 2. For bivectors B₁ and B₂ in Cl(4,0), the exact connecting rotor is:
///    γ = 1 + B₂ · B₁⁻¹  (where B₁⁻¹ = -B₁ / ||B₁||²).
///    Then γ · B₁ = B₁ + B₂ = B₂ · γ, guaranteeing γ · B₁ · γ⁻¹ = B₂ in closed form.
/// 3. The rotor γ is factored into exact generator letters via ordered Tait–Bryan/Gaussian peel.
pub(crate) fn solve_gauge_between(w1: &Word, w2: &Word) -> Option<Word> {
    let r1 = w1.rotor();
    let r2 = w2.rotor();
    if r1.projective_eq(&r2) {
        return None;
    }

    // 1. Sheet plane 180°-turn (inverts orientation on the sensor plane)
    let sheet = PlaneForm::sheet_word();
    if w1.rebase(&sheet).rotor().projective_eq(&r2) {
        return Some(sheet);
    }

    // 2. Extract exact bivector coordinates B₁ and B₂ from the rotors.
    //
    // FROM THE CANONICAL RAYS, NOT FROM WHATEVER SCALE THE TAPE CARRIED.
    // `γ = 1 + B₂·B₁⁻¹` is not projective in `B₂`: scaling that bivector
    // by λ gives `1 + λ·B₂B₁⁻¹`, a different rotor, and the conjugation
    // check below then refuses it. So the answer depended on how the word
    // handed in happened to be spelled — two tapes of one vertex (1.5),
    // one of which composed to a multiple of the other — and a change of
    // normal form anywhere upstream silently turned a solvable gauge into
    // an unsolvable one. `normalised` is A1's ray, which is what a rotor
    // IS.
    let (n1, n2) = (r1.normalised(), r2.normalised());
    let mut b1 = Rotor::zero();
    let mut b2 = Rotor::zero();
    for mask in 0..16 {
        #[expect(
            clippy::cast_possible_truncation,
            reason = "STD1 C1 — the algebra's range: a blade mask below the 2^4 = 16 slots of Cl(4,0)'s rotor"
        )]
        if (mask as u32).count_ones() == 2 {
            b1 = b1.add(&Rotor::blade(mask, n1.coeff(mask).clone()));
            b2 = b2.add(&Rotor::blade(mask, n2.coeff(mask).clone()));
        }
    }

    // 3. Closed-form Geometric Algebra rotor: γ = 1 + B₂ · B₁⁻¹
    // THE SCALE OF `B₁⁻¹` IS CONTENT HERE (78.28(2)): `γ = 1 + B₂ B₁⁻¹` adds
    // the product to the identity, so its scale is not a ray's; it is
    // divided by `B₁`'s weight, named, and nothing else is.
    let one = crate::transducer::alphabet::Q::from_integer(1.into());
    if let Some(b1_inv) = b1.weight().map(|w| b1.reverse().scale(&(one / w))) {
        let gamma_rotor = Rotor::identity().add(&b2.geo(&b1_inv));
        if gamma_rotor.is_versor() {
            {
                let conj = gamma_rotor.geo(&r1).geo(&gamma_rotor.reverse());
                if conj.projective_eq(&r2) {
                    if let Some(Ok(w)) = crate::transducer::alphabet::factor_rotor(&gamma_rotor) {
                        return Some(w);
                    }
                    if let Some(w) = gamma_rotor.spelled_word() {
                        return Some(w);
                    }
                }
            }
        }
    }

    None
}
