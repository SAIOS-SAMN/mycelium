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
    ) -> Option<Path> {
        // 50.2: the stratum is the MEASURED VERSOR's, not the body's. A
        // `Vertex::Word` states its body's moduli, which is a section
        // coordinate (rule 7) in a different keyspace from the charts
        // held generators are filed under; read there, the stratum is the
        // wrong fibre. The word's own chart is `connection_chart`'s
        // shape — the leading 0 that keeps connection charts disjoint
        // from residual charts, then the rotor's conjugacy axes (15.3).
        let mut axes = vec![0u64];
        axes.extend(word.rotor().class_axes()?);
        let chart = crate::transducer::stalk::Invariant(axes);
        let (_, generators) = self.working_atlas().stratum(&chart);
        let mut basis = Vec::new();
        let mut forms = Vec::new();
        let mut at: Vec<crate::transducer::address::Address> = Vec::new();
        for (a, _st) in &generators {
            let Some(held) = self.working_word(a) else {
                continue;
            };
            let Some(l) = held.letters() else {
                continue;
            };
            let w = l.spelling.word.clone();
            let Some(pf) = w.form() else {
                continue;
            };
            basis.push(w.rotor());
            forms.push(pf);
            at.push(held.address());
        }
        let g = word.rotor();
        let gf = g.spelled_word().and_then(|w| w.form())?;
        // A22 over the seventeen is the residue's peel and never the
        // first read (1.8). An empty stratum is nothing to factor
        // through, not a refusal.
        let taken = match crate::transducer::factor::factor_on_plane(&gf, &forms) {
            crate::transducer::factor::Factored::Understood(tape) => tape,
            crate::transducer::factor::Factored::Novel { understood, .. } => understood,
            // A budget that ran out is unmeasured, and a refusal is not a
            // path (rule 11).
            crate::transducer::factor::Factored::Unmeasured => return None,
        };
        let mut generators_of_path = Vec::with_capacity(taken.len());
        for (i, e) in &taken {
            let a = at.get(*i)?;
            generators_of_path.push((a.clone(), *e));
        }
        if generators_of_path.is_empty() {
            return None;
        }
        // Verified by re-derivation, never trusted from the algebra that
        // produced it (16.12 step 4). A path that does not replay is not
        // a path this mind may speak.
        let whole = crate::transducer::factor::replays(&taken, &basis, &g);
        Some(Path {
            vertex: vertex.clone(),
            generators: generators_of_path,
            whole,
        })
    }

    /// Every word vertex of a form, with the path this mind reached it
    /// through. Vertices the presentation does not touch carry none.
    pub fn paths_of_form(&self, f: &crate::transducer::sentence::Form) -> Vec<Path> {
        let mut out = Vec::new();
        for v in &f.vertices {
            let crate::transducer::sentence::Vertex::Word { address, word, .. } = v else {
                continue;
            };
            if let Some(p) = self.path_of(address, word) {
                out.push(p);
            }
        }
        out
    }
}
