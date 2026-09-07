// 13.12, 25.3 — THE MERGE: when two held thoughts are one, and what the
// one that remains carries.
//
//! Two bubbles merge when their forms are one section of the same bundle:
//! congruent up to a re-gauge, or congruent with a torsion axis. A merge
//! absorbs the other's incidence, its remainder and its vortex; a class
//! neighbour that is neither keeps both and writes each into the other's
//! horizon. Zero volume never merges.

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
        let i = if face.i == 0 { 0 } else { face.i.saturating_add(off.saturating_sub(1)) };
        let j = if face.j == 0 { 0 } else { face.j.saturating_add(off.saturating_sub(1)) };
        f.faces.push(sentence::FormFace {
            i,
            j,
            transport: face.transport.clone(),
        });
    }
    f.remainder = union_rem(&a.remainder, &b.remainder);
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    match f.solve_coboundary() {
        Some(gamma) => {
            let _ = f.regauge(&gamma);
            f.coheres()
        }
        None => false,
    }
}

pub(crate) fn absorb_keyed<V: Clone>(dst: &mut Keyed<V>, src: &Keyed<V>) {
    for (k, v) in src.fiber(ROOT) {
        dst.insert(&k, v.clone());
    }
}

pub(crate) fn absorb_vortex(
    dst: &mut Keyed<Keyed<sentence::PivotCertificate>>,
    src: &Keyed<Keyed<sentence::PivotCertificate>>,
) {
    for (after, inner) in src.fiber(ROOT) {
        match dst.get_mut(&after) {
            Some(d) => absorb_keyed(d, inner),
            None => {
                dst.insert(after, inner.clone());
            }
        }
    }
}

pub(crate) fn intersect(a: &[RemCell], b: &[RemCell]) -> Vec<RemCell> {
    let mut kb = Keyed::new();
    for &(r, c, _, _) in b {
        kb.insert(rem_cell_key(r, c), ());
    }
    a.iter().copied().filter(|&(r, c, _, _)| kb.contains(&rem_cell_key(r, c))).collect()
}

impl Contemplation {
    /// Two bubbles on one cell: extend decides. Extended merges and
    /// re-gauges; Congruent merges with a torsion axis; Class keeps both
    /// and writes each into the other's horizon. Zero volume never merges.
    pub(crate) fn decide_neighbors(&mut self, key: &[u8]) {
        let keys = self.complex.link(key, 1);
        for ck in keys {
            let Some((fill, volume, vertices)) = self.complex.cell(&ck).map(|c| (c.fill.clone(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if !matches!(fill, Fill::Open) {
                continue;
            }
            let vol_nz = volume.iter().any(|&x| x > 0);
            let outcome = self.complex.fill(&ck);
            match outcome {
                Fill::Filled(gamma) if vol_nz => {
                    self.fills += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            let keys: Vec<Vec<u8>> =
                                b.nerve.sections.fiber(ROOT).map(|(k, _)| k).collect();
                            for sk in keys {
                                if let Some(sec) = b.nerve.sections.get_mut(&sk) {
                                    sec.gamma = gamma.geo(&sec.gamma);
                                }
                            }
                        }
                    }
                    let _ = self.merge(&vertices);
                    return;
                }
                Fill::Congruent(g, ell) if vol_nz => {
                    self.torsion_merges += 1;
                    let ends = [
                        vertices[0].clone(),
                        vertices.get(1).cloned().unwrap_or_else(|| vertices[0].clone()),
                    ];
                    if let Some(nk) = self.merge(&vertices) {
                        if let Some(b) = self.bubbles.get_mut(nk.as_bytes()) {
                            b.nerve.file_congruent(&ends[0], &ends[1], g, ell);
                        }
                        let before: Vec<crate::transducer::a21::Address> = vertices
                            .iter()
                            .map(|v| crate::transducer::a21::Address::from(v.clone()))
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
                            [0, 0, (ell as u64).saturating_sub(1).max(1), 0],
                        ) {
                            self.pending_rewrites.push(rw);
                        }
                    }
                    return;
                }
                Fill::Already | Fill::Open => {}
                Fill::Filled(_) | Fill::Congruent(_, _) => {}
                Fill::Class(r) => {
                    self.classes += 1;
                    for v in &vertices {
                        if let Some(b) = self.bubbles.get_mut(v) {
                            if r.class_key().is_none_or(|ck| !b.horizon.contains(ck.as_bytes())) {
                                horizon_put(&mut b.horizon, r.clone(), Standing::Held);
                            }
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn merge(&mut self, verts: &[Vec<u8>]) -> Option<crate::transducer::a21::Address> {
        if verts.len() < 2 {
            return verts.first().cloned().map(crate::transducer::a21::Address::from);
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
                    return verts.first().cloned().map(crate::transducer::a21::Address::from);
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
        let mut remainders: Keyed<Vec<RemCell>> = Keyed::new();
        let mut old_rems: Keyed<Vec<RemCell>> = Keyed::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v.as_slice()).cloned() else {
                continue;
            };
            old_rems.insert(v, b.remainder.clone());
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
            cursor: Vec::new(),
            moved_k: 0,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion,
            certificates,
            vortex,
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
            let before: Vec<crate::transducer::a21::Address> = verts
                .iter()
                .map(|v| crate::transducer::a21::Address::from(v.clone()))
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
                [0, verts.len().saturating_sub(1) as u64, 0, 0],
            ) {
                self.pending_rewrites.push(rw);
            }
        }
        Some(key)
    }

}
