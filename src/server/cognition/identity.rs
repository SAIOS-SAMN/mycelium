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
    ) -> Option<(&Element, &crate::transducer::sentence::Form)> {
        if !self.ruled_contains(addr) && !self.cavity_book.contains(addr) {
            return None;
        }
        // A word the cavity retired is ruled and held there, not installed.
        let held = self.held_word(addr).or_else(|| self.cavity_book.get(addr))?;
        Some((held.element(), held.form()?))
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
}
