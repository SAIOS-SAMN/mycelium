// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 27, 3.8; MU-CLIENT §5.4 — A WORD READ OVER THE LANGUAGE WITHOUT ITSELF.
//! The optimisation is a relation, never a size: a word whose every word
//! vertex the rest of the presentation composes is a second derivation of
//! the same object, and the face between the two derivations is a `Retire`
//! rewrite (MU-ANASTOME §4.1). This module reads that relation; it decides
//! nothing for a court and retires nothing from the presentation.

use super::{gauge_modes, Generator, Presentation};
use crate::network::nerve::Keyed;
use crate::transducer::address::Address;

impl Presentation {
    /// THE GENERATORS ONE FORM GAVE THE PRESENTATION: one keyed subtree read
    /// under the form's own key prefix (`generator_key` writes the form
    /// first, length-delimited), never a walk of what is held.
    fn generators_of(&self, form: &Address) -> Vec<&Generator> {
        let mut prefix = Vec::new();
        crate::transducer::address::put_key_bytes(&mut prefix, form.as_bytes());
        match self.held.fibre_at(&prefix) {
            Some(fibre) => fibre.map(|(_, g)| g).filter(|g| g.form == *form).collect(),
            None => Vec::new(),
        }
    }

    /// 27, 3.8 — THE FORMS THAT DERIVE `form` WITHOUT IT: each of its word
    /// vertices' whole motion and hue map read over every other form's
    /// generators ([`Presentation::read_word`], with `form`'s own refused),
    /// only the ruled where `ruled_only`. `Some` with the cited forms, in key
    /// order, only where every vertex composes exactly; `None` where the
    /// presentation holds no generator of `form`, where `ruled_only` and
    /// `form` is not itself ruled, or where any vertex leaves a residue.
    /// Exact: no derivation is compared with another by length or count.
    pub fn derived_without(&self, form: &Address, ruled_only: bool) -> Option<Vec<Address>> {
        let own = self.generators_of(form);
        if own.is_empty() || (ruled_only && own.iter().any(|g| !g.ruled)) {
            return None;
        }
        let mut cited: Keyed<()> = Keyed::new();
        for g in own {
            let read = self.read_word_where(&g.motion, &g.gauge, |h| (h.ruled || !ruled_only) && h.form != *form)?;
            if !read.composes {
                return None;
            }
            for (f, ..) in read.cites {
                cited.insert(f.as_bytes(), ());
            }
        }
        // A read composes only where it cites something, so this is never
        // empty here.
        Some(cited.scoped().map(|(k, ())| Address::from(k)).collect())
    }

    /// THE FORMS STANDING WHERE `form` STANDS: every other form holding a
    /// generator on a hole one of `form`'s generators winds about, or on a
    /// mode of its hue map — one keyed read per hole and per mode, in key
    /// order. These are the words a newly held `form` can be a derivation
    /// of, and the only ones: a word sharing no hole and no mode with it is
    /// composed by nothing it brings.
    pub fn standing_with(&self, form: &Address) -> Vec<Address> {
        let mut out: Keyed<()> = Keyed::new();
        let mut take = |fibre: Option<&Keyed<()>>| {
            for (k, ()) in fibre.into_iter().flat_map(|f| f.scoped()) {
                if let Some(h) = self.held.get(&k).filter(|h| h.form != *form) {
                    out.insert(h.form.as_bytes(), ());
                }
            }
        };
        for g in self.generators_of(form) {
            for hole in &g.coords.holes {
                take(self.holes.fibre(&hole.key));
            }
            for m in gauge_modes(&g.gauge).into_iter().flatten() {
                take(self.gauges.get(&m.key()));
            }
        }
        out.scoped().map(|(k, ())| Address::from(k)).collect()
    }
}
