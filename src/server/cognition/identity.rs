// 21, 31 — the mind AS A CRYSTAL: what a family extraction may ask this
// kernel for, and the trajectory a rule measures against what it holds.
//
//! `Crystal` is the interface `family::extract_rule_with` speaks to. Every
//! answer here is a read of the ruled cover; a miss is zero, because
//! association with a listing is not a mint.

use super::*;

impl crate::transducer::family::Crystal for Kernel {
    fn on_class(
        &self,
        class: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        &Element,
        &crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        self.ruled
            .fiber(class)
            .into_iter()
            .filter_map(|addr| {
                // The court's ruled word, read off the word it HOLDS (L16):
                // the form is a field of the basis entry and this read
                // parses nothing.
                // Z4 item 3: references into the one held word.
                let held = self.held_word(&addr)?;
                Some((held.element(), held.form()?, crate::transducer::stalk::Standing::Ruled))
            })
            .collect()
    }

    /// 21 step 3, at the court: the ruled cover is filed by `chart_of_form`
    /// and carries no index by the moduli of the bodies its forms act on.
    /// It needs none yet — a court resolves a `Glued` vertex by address
    /// (32) and never re-derives a glue, so this read has no caller here;
    /// the kernel is never the crystal `extract_rule_with` speaks to. When
    /// a court extracts, it gains the index the mind has (`by_moduli`), and
    /// this answers from it. Row L8s.
    fn on_body(
        &self,
        _chart: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        &Element,
        &crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        Vec::new()
    }

    fn speak_form(
        &self,
        f: &crate::transducer::sentence::Form,
        x: &crate::transducer::family::Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }

    /// THE ONE READER OF THE WORDS THIS COURT REPLAYS AGAINST: a ruled
    /// address, installed, or a cavity address, which resolves for replay
    /// and is never taught (3.8, ch. 8). It read the basis alone while the
    /// citation check (`resolve_form`) admitted the cavity too, so a
    /// sentence citing a cavity word passed the check and then spoke nothing
    /// at the citation: the whole family read as undeclared remainder.
    fn by_address(
        &self,
        addr: &[u8],
    ) -> Option<(&Element, std::borrow::Cow<'_, crate::transducer::sentence::Form>)> {
        // An installed word holds its form, decoded once where it was placed.
        if let Some(held) = self.held_word(addr).filter(|_| self.ruled_contains(addr)) {
            return Some((held.element(), std::borrow::Cow::Borrowed(held.form()?)));
        }
        // A word the cavity retired is ruled and held there as its element
        // alone: its form is derived from its address here, where it is read.
        let e = self.cavity_book.get(addr)?;
        Some((e, std::borrow::Cow::Owned(crate::transducer::sentence::from_element_form(e)?)))
    }

    fn on_axis(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        crate::transducer::sentence::on_axis_of(&self.axis_index, chart, axis)
    }

    /// 50.2 step 2, at the court: a court does not speak, so it factors
    /// nothing through a presentation. F runs on the voice and nowhere
    /// else (law 8 as ruled 2026-09-10), and the court's reading of a
    /// derivation is `replay_form_at` resolving the citations it was
    /// SENT. Empty, and named rather than silent — the same standing as
    /// `on_body` here (row L8s).
    fn generators_under(
        &self,
        _chart: &crate::transducer::stalk::Invariant,
        _budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        Vec::new()
    }
}

impl Kernel {
    /// What a rule measures by speaking every train pair, its cited
    /// forms resolved through this kernel's ruled cover. Any miss is zero:
    /// association with a listing is not a mint.
    pub(crate) fn measured_replay(&self, form: &Element, family: &crate::transducer::family::Family) -> [u64; 4] {
        self.measured_replay_at(
            form,
            family,
            &crate::transducer::alphabet::Word::identity(),
        )
    }

    /// [`Kernel::measured_replay`] with the modes it discharged (78.162): what
    /// the closing form moved on the pairs it closed, mode by mode, exact.
    pub(crate) fn measured_replay_read(&self, form: &Element, family: &crate::transducer::family::Family) -> ([u64; 4], crate::economy::modes::Modes) {
        match crate::transducer::sentence::from_element_form(form) {
            Some(f) => {
                let r = crate::transducer::family::form_deflation_read(&f, family, self, &crate::transducer::alphabet::Word::identity());
                (r.pay, r.modes)
            }
            None => ([0; 4], crate::economy::modes::Modes::new()),
        }
    }

    /// The same measure at the placement a citer put the form at (26).
    pub(crate) fn measured_replay_at(
        &self,
        form: &Element,
        family: &crate::transducer::family::Family,
        placement: &crate::transducer::alphabet::Word,
    ) -> [u64; 4] {
        if let Some(f) = crate::transducer::sentence::from_element_form(form) {
            return crate::transducer::family::form_deflation_at(&f, family, self, placement).0;
        }
        [0; 4]
    }

    /// 78.149 — μ IS DISCHARGE (the operator's escrow rulings, 2026-10-08).
    /// THE ONE READER of what a word can be escrowed or paid, and every
    /// escrow and every release reads it, never the word's magnitude:
    ///
    /// - the μ a word can be escrowed or paid is its DISCHARGE — the lanes
    ///   it closed, the deflation this court measures by replaying it on
    ///   the family it is bound to (`measured_replay`), or a rewrite's yield
    ///   read on its carriers before less after (`settle_rewrite`);
    /// - (a) never the span it stood: `t_extent` is not μ, waiting is not
    ///   work, and no minted magnitude carries it;
    /// - (b) never what it asks: a demand's unclosed links are its ASK, the
    ///   listing's measure, which may CAP a price and never raise it — a
    ///   claim is paid lane-wise `min(escrow held, the closer's measured
    ///   discharge)` (`Treasury::release_discharged`), and nothing above the
    ///   discharge is minted;
    /// - (c) zero for a word that names no body: a letter or a callus stands
    ///   at no frame and closes nothing on any family, so it measures
    ///   nothing here. Its magnitude's holonomic unit is its EXISTENCE as a
    ///   word (`as_element_form` refuses an all-zero magnitude), never μ.
    ///
    /// The word's magnitude stays what it is — its measure and identity, the
    /// book's tension, the room it draws — and is read for those and for
    /// nothing priced. `family` is the family the word is heard or bound on;
    /// a sentence with none discharges nothing measurable here, which is a
    /// reading and not a floor.
    pub(crate) fn discharge_on(
        &self,
        addr: &[u8],
        word: &Element,
        family: Option<&crate::transducer::family::Family>,
    ) -> [u64; 4] {
        match (word.is_rewrite(), word.is_sentence(), family) {
            (true, _, _) => word.magnitude(),
            // MU-CLIENT §5.6 (C) — a conjugate is paid what its conjugator
            // added on the family: its own section beyond the cited word's.
            (false, true, Some(f)) if self.conjugate_of(word).is_some() => self.transport_on(word, f),
            (false, true, Some(f)) => self.measured_replay(word, f),
            (false, true, None) => [0; 4],
            // MU-CLIENT §5.6 (W) — a connection bound to a family is
            // decompressed onto it and measured on its own section; a colour
            // word's evidence is its fibre relation, never a section.
            (false, false, Some(f)) if word.is_connection() && !word.is_colour_word() => {
                crate::transducer::family::local::word_local(word, f).pay
            }
            // A connection bound to no family is measured by the listing it
            // stands for on the book, and by nothing where it stands for none
            // (26: association with a listing is not a mint).
            (false, false, _) => match self.book_elem(addr) {
                Some(e) => e.chain_magnitude(),
                None => [0; 4],
            },
        }
    }

    /// MU-CLIENT §5.6 (C) — THE WORD A CONJUGATE TRANSPORTS: the form its
    /// derivation cites, where that derivation carries a conjugator (`g · W ·
    /// g̃`, 78.166 1c) and the cite is one this court holds. `None` for any
    /// word that is not a conjugate of a held word.
    pub(crate) fn conjugate_of(&self, word: &Element) -> Option<crate::transducer::address::Address> {
        let env = word.receipts();
        let cite = env.derivations.iter().find(|d| d.conjugator.is_some())?;
        let [(form, _, _)] = cite.generators.as_slice() else { return None };
        crate::transducer::family::Crystal::by_address(self, form.as_bytes()).map(|_| form.clone())
    }

    /// C — WHAT A CONJUGATOR ADDED ON `family`: the conjugate's local section
    /// beyond the cited word's own, spoken in its own frame — the sites the
    /// transport closed that the word untransported does not, measured. A
    /// conjugate that closes nothing its word does not is paid nothing.
    pub(crate) fn transport_on(&self, word: &Element, family: &crate::transducer::family::Family) -> [u64; 4] {
        use crate::transducer::family::local::form_local;
        let here = crate::transducer::alphabet::Word::identity();
        let (Some(conj), Some(cited)) = (crate::transducer::sentence::from_element_form(word), self.conjugate_of(word)) else {
            return [0; 4];
        };
        let Some((_, base)) = crate::transducer::family::Crystal::by_address(self, cited.as_bytes()) else { return [0; 4] };
        let own = form_local(&base, family, self, &here);
        form_local(&conj, family, self, &here).beyond(&own, family).pay
    }

    /// S — A SENTENCE'S GLUING on the family it is bound to: the sites its
    /// closed rings close that no word it teaches closes on its own section,
    /// measured — the premium the voices that composed it take. Nothing
    /// where it is bound to no family, or does no more than its words.
    pub(crate) fn gluing_of(&self, addr: &[u8], sentence: &Element, words: &[Element]) -> [u64; 4] {
        use crate::transducer::family::local::{form_local, word_local, Local};
        let task = self.speech_task.get(addr).or_else(|| self.listing_task.get(addr));
        let Some(family) = task.and_then(|t| self.task_family.get(t.as_bytes())) else { return [0; 4] };
        let Some(form) = crate::transducer::sentence::from_element_form(sentence) else { return [0; 4] };
        let whole = form_local(&form, family, self, &crate::transducer::alphabet::Word::identity());
        let sections: Vec<Local> = words.iter().map(|w| word_local(w, family)).collect();
        whole.beyond(&Local::union(&sections, family.len()), family).pay
    }

    /// [`Kernel::discharge_on`] over the family the word is BOUND to — the
    /// task it was spoken for, or the listing it answers — which is the
    /// family its install measures.
    pub(crate) fn discharge_of(&self, addr: &[u8], word: &Element) -> [u64; 4] {
        let task = self.speech_task.get(addr).or_else(|| self.listing_task.get(addr));
        let family = task.and_then(|t| self.task_family.get(t.as_bytes()));
        self.discharge_on(addr, word, family.map(Vec::as_slice))
    }
}
