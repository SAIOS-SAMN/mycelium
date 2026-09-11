// 21, 31 — the mind AS A CRYSTAL: what a family extraction may ask this
// kernel for, and the descent a rule measures against what it holds.
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
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        self.ruled
            .fiber(class)
            .into_iter()
            .filter_map(|addr| {
                // The court's ruled word, read off the element it HOLDS
                // (L16), so the tape is parsed once per word and not once
                // per read of it.
                let held = self.word_at(&addr)?;
                let f = held.form()?.clone();
                Some((held.clone(), f, crate::transducer::stalk::Standing::Ruled))
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
        Element,
        crate::transducer::sentence::Form,
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

    fn by_address(
        &self,
        addr: &[u8],
    ) -> Option<(Element, crate::transducer::sentence::Form)> {
        let held = self.word_at(addr)?;
        let f = held.form()?.clone();
        Some((held.clone(), f))
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
    /// Descent a rule measures by speaking every train pair, its cited
    /// forms resolved through this kernel's ruled cover. Any miss is zero:
    /// association with a listing is not a mint.
    pub(crate) fn measured_replay_descent(&self, form: &Element, family: &crate::transducer::family::Family) -> [u64; 4] {
        self.measured_replay_descent_at(
            form,
            family,
            &crate::transducer::alphabet::Word::identity(),
        )
    }

    /// The same descent at the placement a citer put the form at (26).
    pub(crate) fn measured_replay_descent_at(
        &self,
        form: &Element,
        family: &crate::transducer::family::Family,
        placement: &crate::transducer::alphabet::Word,
    ) -> [u64; 4] {
        if let Some(f) = form.form() {
            return crate::transducer::family::form_descent_at(f, family, self, placement).0;
        }
        [0; 4]
    }
}
