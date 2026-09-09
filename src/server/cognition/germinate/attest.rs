// 26, 47.3 — ATTESTATION: what a voice says it derived, what this court
// remembers of it, and where two organs converge.
//
//! A repeated demand is answered from the docket rather than re-docketed
//! (23.8). Convergence is on a witness's RE-DERIVATION by `same_vertex`
//! (M1) and never on the bytes it happens to be spelled with.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::{self, Element};

/// WHAT ONE UTTERANCE MEASURED: the words it cited, the residual it left,
/// and the length of the chain it spoke to. Three counts that are read
/// together, written to the receipt together, and mean nothing apart.
pub struct Uttered {
    pub cites: usize,
    pub residual: usize,
    pub chain_len: usize,
}

impl Kernel {
    /// QUOTE — a local reading of the page: listings still on_page, in
    /// atlas order, under one stroke Budget. Tests use this; the wire
    /// quote is an incidence query over the voice's charts.
    pub fn quote(&mut self) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        self.remeasure_pressure();
        let surface = crate::economy::yield_surface(&self.physics);
        let mut priced = Vec::new();
        let mut budget = self.physics.stroke();
        // Resumed from the last quote (rule 10, S13.3): the page is an
        // orbit, so a page beyond one budget is served whole across quotes
        // rather than its head on every one.
        let from = self.quote_cursor.clone();
        let mut cur = self.on_page.cursor(&from, &mut budget);
        for (addr, _) in &mut cur {
            let Some((p, _)) = self.pending.get(&addr) else { continue };
            let mut mag = p.magnitude();
            mag[2] = self.t_extent(&p.address(), self.height());
            let mu = crate::economy::price(&mag, &self.physics.pressure);
            priced.push((p.clone(), mu));
        }
        if let Some(k) = cur.last_key() {
            self.quote_cursor = k.to_vec();
        }
        (surface, priced)
    }

    /// ATTEST — a peer's INDEPENDENT derivation of an open residual arrives as a
    /// spoken `demand` act. The projective reduction runs first (a stale quote
    /// DECOMPOSES instead of double-minting); a genuine irreducible joins the
    /// admission as this voice's attestation — consilience through conversation.
    /// The local path has no voice K on the call: K = 0, same as a demand
    /// frame that omits it. A later K is `attest_at`.
    pub fn attest(&mut self, chain: &[(i64, i64, u8, u8)], voice: [u8; 32]) -> Outcome {
        self.attest_at(chain, voice, &[], 0)
    }

    /// Demand with the speaker's citations and the voice's K. Same (vk,
    /// address, K) is answered from the docket; a later K is a use.
    pub fn attest_at(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        voice: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
        k: u64,
    ) -> Outcome {
        self.ensure_bonded(voice);
        let germ = element::from_residual(&chain.to_vec());
        let listing = germ.address();
        if let Some(out) = self.answer_if_duplicate(&voice, &listing, k) {
            return out;
        }
        let Some((language, _budget)) = self.reflex_open(voice) else {
            return Outcome::GermPending;
        };
        let frame = crate::arc::Frame::default();
        let reduction = reduce_proven(&language, germ, cites, &frame, listing.clone());
        let residual = match &reduction.form {
            NormalForm::Decomposed(_) => 0,
            NormalForm::Irreducible(r) => r.sub().len(),
        };
        let out = self.commit_all(vec![reduction], voice);
        let said = Uttered { cites: cites.len(), residual, chain_len: chain.len() };
        self.record_utterance(voice, listing, k, said, out)
    }

    /// A repeated (vk, address, K) is the reply last given, if any.
    pub fn answer_if_duplicate(&self, voice: &[u8; 32], addr: &[u8], k: u64) -> Option<Outcome> {
        let out = self.utterance_of(voice, addr, k)?.clone();
        eprintln!(
            "  court attest duplicate {:02x}{:02x}… voice {:02x}{:02x}… K {k}",
            addr[0], addr[1], voice[0], voice[1]
        );
        Some(out)
    }

    pub(crate) fn utterance_of(&self, voice: &[u8; 32], addr: &[u8], k: u64) -> Option<&Outcome> {
        self.utterances.get(addr)?.get(voice)?.get(&utterance_k_key(k))
    }

    pub(crate) fn remember_utterance(&mut self, voice: [u8; 32], addr: Address, k: u64, out: Outcome) {
        let kk = utterance_k_key(k);
        let mut by_v = self.utterances.remove(&addr).unwrap_or_default();
        let mut by_k = by_v.remove(&voice).unwrap_or_default();
        by_k.insert(kk, out);
        by_v.insert(voice, by_k);
        self.utterances.insert(addr, by_v);
    }

    /// Seal the utterance receipt and log the first hearing.
    pub fn record_utterance(
        &mut self,
        voice: [u8; 32],
        addr: Address,
        k: u64,
        said: Uttered,
        out: Outcome,
    ) -> Outcome {
        let Uttered { cites, residual, chain_len } = said;
        eprintln!(
            "  court attest {:02x}{:02x}… voice {:02x}{:02x}… K {k} cites {cites} residual {residual} of {chain_len}",
            addr[0], addr[1], voice[0], voice[1]
        );
        self.remember_utterance(voice, addr, k, out.clone());
        out
    }

    /// Words taught on a decomposed sentence: each cited word the docket
    /// holds anchored. Sentence length does not decide.
    pub fn taught_anchored(&self, word: &[usize]) -> Vec<u8> {
        taught_anchored_words(word.iter().filter_map(|&i| self.language.words.get(i).map(|e| e.as_ref())))
    }

    /// The voices that uttered this object — the settle receipt's parties.
    /// A use, credited (24.1); never a vantage.
    pub fn voices_of(&self, germ: &[u8]) -> Vec<[u8; 32]> {
        self.convergence.parties(germ)
    }

    /// THIS COURT re-derived `object` and reached its address, and the voice
    /// that uttered it is a party to the settlement. The second vantage is a
    /// witness hypha's re-derivation of the same address (47.3), recorded by
    /// `converge_witness` when an Offer is glued.
    pub(crate) fn converge_here(&mut self, object: &Element, voice: [u8; 32]) {
        let addr = object.address();
        self.converge_here_at(&addr, object, voice);
    }

    /// The same, where the object is DOCKETED under a key that is not its
    /// own address -- a section, which is keyed by `section_address` over
    /// the held word and the assignment. The key is the docket's coordinate;
    /// the object is what convergence compares.
    pub(crate) fn converge_here_at(&mut self, addr: &Address, object: &Element, voice: [u8; 32]) {
        let me = self.identity;
        let (k, h) = (self.k_step(), self.height());
        self.convergence.rederived(
            addr.as_bytes(),
            me,
            crate::court::convergence::Rederivation { object: object.clone(), k, height: h },
        );
        self.convergence.used_by(addr.as_bytes(), voice);
    }

    /// A WITNESS HYPHA re-derived `object` and reached `reached`. Equal to
    /// this court's address, the object has converged and may install;
    /// different, it is inconsistent data and refuses (13.9 step 8).
    pub(crate) fn converge_witness(
        &mut self,
        object: &Address,
        witness: [u8; 32],
        reached: Element,
        k: u64,
        h: u64,
    ) -> crate::court::convergence::Converged {
        self.convergence.rederived(
            object.as_bytes(),
            witness,
            crate::court::convergence::Rederivation { object: reached, k, height: h },
        );
        let me = self.identity;
        let bond = |v: &[u8; 32]| self.treasury.bond_of(v);
        let room = |v: &[u8; 32]| self.lane_room_of(v);
        // 47.3: a second VANTAGE, not a second copy. The witness is another
        // organ, bonded outside this home's delegation closure, with room on
        // every lane. Delegation closure is task 5.3; until it lands the
        // test is identity and bond, and that limit is stated rather than
        // hidden behind a passing check.
        let independent = move |a: &[u8; 32], b: &[u8; 32]| -> bool {
            a != b && (*a == me || *b == me) && {
                let other = if *a == me { b } else { a };
                bond(other).is_some() && room(other).iter().any(|&r| r > 0)
            }
        };
        self.convergence.state(object.as_bytes(), &independent)
    }

}
