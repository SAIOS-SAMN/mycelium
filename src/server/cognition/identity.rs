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
                let e = self.word_at(&addr)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                Some((e, f, crate::transducer::stalk::Standing::Ruled))
            })
            .collect()
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
        let e = self.word_at(addr)?.clone();
        let f = crate::transducer::sentence::from_element_form(&e)?;
        Some((e, f))
    }

    fn on_axis(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        crate::transducer::sentence::on_axis_of(&self.axis_index, chart, axis)
    }
}

impl Kernel {
    /// Descent a rule measures by speaking every train pair, its cited
    /// forms resolved through this kernel's ruled cover. Any miss is zero:
    /// association with a listing is not a mint.
    pub(crate) fn measured_replay_descent(&self, form: &Element, family: &crate::transducer::family::Family) -> [u64; 4] {
        if let Some(f) = crate::transducer::sentence::from_element_form(form) {
            return crate::transducer::family::form_descent(&f, family, self).0;
        }
        [0; 4]
    }
}
