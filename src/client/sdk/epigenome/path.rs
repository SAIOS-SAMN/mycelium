// 50.2 step 2 — THE PATH: the generators this mind reached a word vertex
// through, read by factoring the measured versor against what it holds.
//
//! A22 from the identity is a function of the versor alone, so a mind that
//! spells from the identity what it already holds has not comprehended
//! (chapter 6) and the duplicate it mints is removed by a retirement E2
//! pays for. The order is 24.1's and it is a law (1.8): factor through
//! what is held; spell only the residue. The vertex is unchanged either
//! way — identity is the rotor (1.5) — so two minds reach ONE address by
//! two paths, and the path is what tells them apart and what the ledger
//! pays.

use crate::client::sdk::*;
use crate::transducer::sentence::Path;

impl Epigenome {
    /// The path to `word` through the generators this mind holds on the
    /// stratum of `chart` (A12 over `WorkingAtlas::stratum`, 42.6). The
    /// factoriser is `transducer::factor`, the same one the court runs;
    /// nothing is moved and nothing is duplicated.
    ///
    /// `None` where the mind holds no generator that factors any part of
    /// the word: there is no path, and A22's spelling from the identity
    /// stands alone. A path with an empty generator list is never
    /// returned — the identity is what every mind already speaks.
    pub fn path_of(
        &self,
        vertex: &crate::transducer::address::Address,
        word: &crate::transducer::alphabet::Word,
        budget: &mut crate::physics::Budget,
    ) -> Option<Path> {
        // 50.2: the stratum is the MEASURED VERSOR's, not the body's, and
        // the generators are the WORD VERTICES of the forms this mind holds
        // (1.8). Read off `Element::letters` — which answers `None` for
        // every sentence — the basis saw only the v1 connection words the
        // utterance path stopped producing at L5, so a mind holding a
        // language of forms factored through nothing.
        let chart = crate::transducer::sentence::versor_chart(word)?;
        let mut basis = Vec::new();
        let mut forms = Vec::new();
        let mut at: Vec<(
            crate::transducer::address::Address,
            crate::transducer::address::Address,
        )> = Vec::new();
        for (form_addr, v) in
            crate::transducer::sentence::generators_under(&self.generators, &chart, budget)
        {
            // A word is not a generator of itself.
            if v.as_bytes() == vertex.as_bytes() {
                continue;
            }
            let Some(held) = self.working_word(form_addr.as_bytes()) else {
                continue;
            };
            let Some(f) = crate::transducer::sentence::from_element_form(held) else {
                continue;
            };
            let Some(w) = crate::transducer::sentence::word_at_vertex(&f, v.as_bytes()) else {
                continue;
            };
            let Some(pf) = w.form() else {
                continue;
            };
            basis.push(w.rotor());
            forms.push(pf);
            at.push((form_addr, v));
        }
        let g = word.rotor();
        let gf = g.spelled_word().and_then(|w| w.form())?;
        // A22 over the seventeen is the residue's peel and never the
        // first read (1.8). An empty stratum is nothing to factor
        // through, not a refusal.
        let (taken, residue) = match crate::transducer::factor::factor_on_plane(&gf, &forms) {
            crate::transducer::factor::Factored::Understood(tape) => {
                (tape, crate::transducer::alphabet::Word::identity())
            }
            // The canonical remainder is what the presentation did not
            // reach (A12 step 5). Carried, a PARTIAL path is a use of the
            // generators that did reach; dropped, only a whole path could
            // ever compose and a partial one paid nobody.
            crate::transducer::factor::Factored::Novel { understood, remainder } => {
                (understood, remainder.to_word())
            }
            // A budget that ran out is unmeasured, and a refusal is not a
            // path (rule 11).
            crate::transducer::factor::Factored::Unmeasured => return None,
        };
        let mut generators_of_path = Vec::with_capacity(taken.len());
        for (i, e) in &taken {
            let (f, v) = at.get(*i)?;
            generators_of_path.push((f.clone(), v.clone(), *e));
        }
        if generators_of_path.is_empty() {
            return None;
        }
        // Verified by re-derivation, never trusted from the algebra that
        // produced it (16.12 step 4): the generators composed, then the
        // residue, reach the vertex. A path that does not replay is not a
        // path this mind may speak.
        if !crate::transducer::factor::replays_with_residue(
            &taken,
            &basis,
            &residue.rotor(),
            &g,
        ) {
            return None;
        }
        Some(Path {
            vertex: vertex.clone(),
            generators: generators_of_path,
            residue,
        })
    }

    /// Every word vertex of a form, with the path this mind reached it
    /// through. Vertices the presentation does not touch carry none.
    pub fn paths_of_form(
        &self,
        f: &crate::transducer::sentence::Form,
        budget: &mut crate::physics::Budget,
    ) -> Vec<Path> {
        let mut out = Vec::new();
        for v in &f.vertices {
            let crate::transducer::sentence::Vertex::Word { address, derivation, .. } = v else {
                continue;
            };
            if let Some(p) = self.path_of(address, derivation.word(), budget) {
                out.push(p);
            }
        }
        out
    }
}
