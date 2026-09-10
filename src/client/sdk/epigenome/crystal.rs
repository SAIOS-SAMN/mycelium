// 21, 31 — THE EPIGENOME AS A CRYSTAL: what a family extraction may ask a
// client for.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl crate::transducer::family::Crystal for Epigenome {
    fn on_class(
        &self,
        class: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        // 24.2: the closed forms this mind holds at a CITED FORM's chart,
        // which is `chart_of_form`. The glue's read is `on_body`.
        self.working_atlas()
            .fiber(class)
            .into_iter()
            .filter_map(|(a, st)| {
                let e = self.working_word(&a)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                if !f.is_closed() {
                    return None;
                }
                Some((e, f, st))
            })
            .collect()
    }

    fn on_body(
        &self,
        chart: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        // 21 step 3: the closed forms carrying a body on this chart, read
        // at the coordinate the perceived body supplies and nowhere else
        // (24.3, law 3). The chart routes; `speak_form` decides.
        let Some(bucket) = self.by_moduli.get(chart.key().as_bytes()) else {
            return Vec::new();
        };
        bucket
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(a, ())| {
                let st = self.standing_of(&a)?;
                let e = self.working_word(&a)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                if !f.is_closed() {
                    return None;
                }
                Some((e, f, st))
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
        let e = self.working_word(addr)?.clone();
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
