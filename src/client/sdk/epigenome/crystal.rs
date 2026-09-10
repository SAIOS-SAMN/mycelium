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

    /// 1.8: THE PRESENTATION THIS MIND SPEAKS FROM on a versor's chart.
    /// The generator index is keyed by `versor_chart` — `[0,
    /// class_axes…]` — so a peeled key is a byte prefix and the read is
    /// one keyed subtree walk, never a scan of what the mind holds (law
    /// 3, 24.3). Each cited vertex's word is resolved through the form
    /// that carries it, by its own `by_address` index; a citation whose
    /// form the mind no longer holds is dropped rather than guessed
    /// (rule 11).
    fn generators_under(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        budget: &mut crate::physics::Budget,
    ) -> Vec<(
        crate::transducer::address::Address,
        crate::transducer::address::Address,
        crate::transducer::alphabet::Word,
    )> {
        crate::transducer::sentence::generators_under(&self.generators, chart, budget)
            .into_iter()
            .filter_map(|(form, vertex)| {
                let e = self.working_word(form.as_bytes())?;
                let f = crate::transducer::sentence::from_element_form(e)?;
                let w = crate::transducer::sentence::word_at_vertex(&f, vertex.as_bytes())?;
                Some((form, vertex, w))
            })
            .collect()
    }
}
