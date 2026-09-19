// 13.12, 25 — THE MERGE: when two held thoughts are one, and what the
// one that remains carries.
//
//! Two bubbles merge when their forms are one section of the same bundle:
//! congruent up to a re-gauge, or with a torsion axis. A merge absorbs the
//! other's incidence, remainder and vortex; zero volume never merges.

use super::*;

/// 13.12: two forms are one section when a coboundary on their join coheres.
pub(crate) fn form_one_section(a: &Form, b: &Form) -> bool {
    if idea_key(a) == idea_key(b) {
        return true;
    }
    let mut f = a.clone();
    let off = f.vertices.len() as u16;
    if off == 0 {
        return b.coheres();
    }
    for v in b.vertices.iter().skip(1) {
        f.vertices.push(v.clone());
    }
    for face in &b.faces {
        // The join's own vertices start where this form's end, less the
        // Frame the two share. `off` is a vertex count of at least one —
        // the Frame — so the shift is exact.
        let Some(shift) = off.checked_sub(1) else {
            return false;
        };
        let i = if face.i == 0 { 0 } else { face.i.saturating_add(shift) };
        let j = if face.j == 0 { 0 } else { face.j.saturating_add(shift) };
        f.faces.push(sentence::FormFace {
            i,
            j,
            word: face.word.clone(),
            at: face.at.clone(),
            // The section is the BUNDLE's, and the shift moves vertices,
            // so it crosses the merge unchanged with the face it names.
            pairs: face.pairs.clone(),
        });
    }
    // The join is taken in the SCENE, where the two chains actually sit,
    // and A5 decides it: two chains naming one site with different
    // transitions are not one section (rule 11 — keeping either would
    // invent an agreement they do not have).
    let mut both = a.remainder.placed();
    both.extend_from_slice(&b.remainder.placed());
    let Some(joined) = element::place_chain(&both) else {
        return false;
    };
    f.remainder = joined;
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    match f.solve_coboundary() {
        Some(gamma) => {
            let _ = f.regauge(&gamma);
            f.coheres()
        }
        None => false,
    }
}

/// ABSORB IS THE WHOLE OF THE SOURCE, by the meaning of the word: two
/// bubbles merging carry every entry of each other's carriers across, and
/// an entry left behind is a fact the merged bubble has lost.
pub(crate) fn absorb_keyed<V: Clone>(dst: &mut Keyed<V>, src: &Keyed<V>) {
    for (k, v) in src.scoped() {
        dst.insert(&k, v.clone());
    }
}

pub(crate) fn absorb_vortex(
    dst: &mut Keyed<Keyed<sentence::PivotCertificate>>,
    src: &Keyed<Keyed<sentence::PivotCertificate>>,
) {
    for (after, inner) in src.scoped() {
        match dst.get_mut(&after) {
            Some(d) => absorb_keyed(d, inner),
            None => {
                dst.insert(after, inner.clone());
            }
        }
    }
}

pub(crate) fn intersect(a: &[Link], b: &[Link]) -> Vec<Link> {
    let mut kb = Keyed::new();
    for &(r, c, _, _) in b {
        kb.insert(remainder_site_key(r, c), ());
    }
    a.iter().copied().filter(|&(r, c, _, _)| kb.contains(&remainder_site_key(r, c))).collect()
}

impl Contemplation {
    /// Two bubbles on one simplex: extend decides. Extended merges and
    /// re-gauges; Congruent merges with a torsion axis; Class keeps both
    /// and writes each into the other's horizon. Zero volume never merges.
    pub(crate) fn decide_neighbors(&mut self, key: &[u8]) {
        let keys = self.complex.link(key, 1);
        for ck in keys {
            let Some((fill, volume, vertices)) = self.complex.simplex(&ck).map(|c| (c.fill.clone(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if !matches!(fill, Fill::Open) {
                continue;
            }
            let vol_nz = volume.iter().any(|&x| x > 0);
            let outcome = self.complex.fill(&ck);
            match outcome {
                Fill::Filled(gamma, carriage) if vol_nz => {
                    self.fills += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            let keys: Vec<Vec<u8>> =
                                b.nerve.sections.scoped().map(|(k, _)| k).collect();
                            for sk in keys {
                                if let Some(sec) = b.nerve.sections.get_mut(&sk) {
                                    sec.gamma = gamma.geo(&sec.gamma);
                                    sec.carriage = carriage.geo(&sec.carriage);
                                }
                            }
                        }
                    }
                    let _ = self.merge(&vertices);
                    return;
                }
                Fill::Congruent(_, g, ell) if vol_nz => {
                    self.torsion_merges += 1;
                    // A simplex of one vertex is a loop at that vertex: both
                    // ends are it, which is what the simplex says and not a
                    // second end invented for it. The vertices are the
                    // simplex's own sorted keys, read as a slice pattern.
                    let ends = match vertices.as_slice() {
                        [a, b, ..] => [a.clone(), b.clone()],
                        [a] => [a.clone(), a.clone()],
                        [] => continue,
                    };
                    if let Some(nk) = self.merge(&vertices) {
                        if let Some(b) = self.bubbles.get_mut(nk.as_bytes()) {
                            b.nerve.file_congruent(&ends[0], &ends[1], *g, ell);
                        }
                        let before: Vec<crate::transducer::address::Address> = vertices
                            .iter()
                            .map(|v| crate::transducer::address::Address::from(v.clone()))
                            .collect();
                        let payload = crate::transducer::rewrite::RewritePayload::Torse {
                            sigma: Vec::new(),
                            period: ell,
                        };
                        if let Some(rw) = crate::transducer::rewrite::mint(
                            crate::transducer::rewrite::RewriteKind::Torse,
                            &before,
                            nk.clone(),
                            payload,
                            // What the period measures (L7b): a torsion of
                            // period ℓ closes ℓ − 1. `classify` files a
                            // congruence only at an order that closes, so
                            // ℓ ≥ 1 and the subtraction is total.
                            [0, 0, ell as u64 - 1, 0],
                        ) {
                            self.pending_rewrites.push(rw);
                        }
                    }
                    return;
                }
                Fill::Already | Fill::Open => {}
                // 37.7 at a simplex: a rotation the seventeen do not spell,
                // which merges nothing and seeds no horizon.
                Fill::Unspellable(_) => {}
                Fill::Filled(..) | Fill::Congruent(..) => {}
                Fill::Class(_, r) => {
                    self.classes += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            if r.class_key().is_none_or(|ck| !b.horizon.contains(ck.as_bytes())) {
                                horizon_put(&mut b.horizon, r.as_ref().clone(), Standing::Held);
                            }
                        }
                    }
                }
            }
        }
    }

    /// A merge of fewer than two bubbles is the bubble itself, and a merge
    /// the forms refuse leaves the LEAST key of the simplex standing — the
    /// vertices arrive sorted, so it is the simplex's own coordinate and
    /// not the bubble that was reached first.
    pub(crate) fn merge(&mut self, verts: &[Vec<u8>]) -> Option<crate::transducer::address::Address> {
        // One vertex is that vertex; none is nothing to merge.
        if let [only] = verts {
            return Some(crate::transducer::address::Address::from(only.clone()));
        }
        if verts.is_empty() {
            return None;
        }
        let forms: Vec<Form> = verts
            .iter()
            .filter_map(|v| self.bubbles.get(v.as_slice()).and_then(|b| b.form.clone()))
            .collect();
        if forms.len() >= 2 {
            for w in forms.windows(2) {
                if !form_one_section(&w[0], &w[1]) {
                    eprintln!(
                        "  thought merge refused {:02x}{:02x}… not one section",
                        verts[0][0], verts[0][1]
                    );
                    #[expect(
                        clippy::disallowed_methods,
                        reason = "rule 15 — canonical key order: the simplex's vertices are sorted, so the least key stands where the merge is refused"
                    )]
                    return verts.first().cloned().map(crate::transducer::address::Address::from);
                }
            }
        }
        let mut remainder = Vec::new();
        let mut cover = Keyed::new();
        let mut nerve = Complex::new();
        let mut horizon: Keyed<(Rotor, Order, Standing)> = Keyed::new();
        let mut torsion = Keyed::new();
        let mut certificates = Keyed::new();
        let mut vortex = Keyed::new();
        let mut form = None;
        let mut bundle = None;
        let mut remainders: Keyed<Vec<Link>> = Keyed::new();
        let mut old_rems: Keyed<Vec<Link>> = Keyed::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v.as_slice()).cloned() else {
                continue;
            };
            old_rems.insert(v, b.remainder.to_vec());
            self.retract_incidence(v, &b);
            remainder = union_rem(&remainder, &b.remainder);
            absorb_keyed(&mut cover, &b.cover);
            absorb_keyed(&mut horizon, &b.horizon);
            absorb_keyed(&mut torsion, &b.torsion);
            absorb_keyed(&mut certificates, &b.certificates);
            absorb_vortex(&mut vortex, &b.vortex);
            absorb_keyed(&mut remainders, &b.remainders);
            if form.is_none() {
                form = b.form.clone();
            }
            if bundle.is_none() {
                bundle = b.bundle.clone();
            }
            nerve = b.nerve.clone();
            self.bubbles.remove(v);
        }
        self.paint_seeded(&mut horizon);
        if remainder.is_empty() && form.is_none() {
            return None;
        }
        let volume = match &form {
            Some(f) => form_mag(f),
            None => element::measure_lanes(&remainder),
        };
        // A union of two remainders can say two things at one site, and
        // such a remainder has no A5 chain. The merge yields no bubble
        // rather than one that cannot be named (U14).
        let remainder = element::place_chain(&remainder)?;
        let key = match &form {
            Some(f) => idea_key(f),
            None => remainder_addr(&remainder),
        };
        let bubble = Bubble {
            remainder,
            nerve,
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: false,
            moved_k: 0,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion,
            certificates,
            vortex,
            candidates: Keyed::new(),
            escrow: volume,
            until: None,
            rotor: form.as_ref().and_then(Bubble::rotor_of_form),
            form,
            bundle,
            remainders,
        };
        self.admit_incidence(&key, &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        self.link_new(&key);
        if verts.len() >= 2 {
            for v in verts {
                if v.as_slice() != key.as_bytes() {
                    if let Some(rem) = old_rems.get(v) {
                        self.cavity.insert(v, rem.clone());
                    }
                }
            }
            let before: Vec<crate::transducer::address::Address> = verts
                .iter()
                .map(|v| crate::transducer::address::Address::from(v.clone()))
                .collect();
            let payload = crate::transducer::rewrite::RewritePayload::Condense {
                sections: vec![vec![]],
                gauge: element::identity_versor(),
            };
            if let Some(rw) = crate::transducer::rewrite::mint(
                crate::transducer::rewrite::RewriteKind::Condense,
                &before,
                key.clone(),
                payload,
                // Condensing n bubbles closes n − 1 joins; condensing none
                // closes none.
                [
                    0,
                    match verts.len().checked_sub(1) {
                        Some(joins) => joins as u64,
                        None => 0,
                    },
                    0,
                    0,
                ],
            ) {
                self.pending_rewrites.push(rw);
            }
        }
        Some(key)
    }

}
