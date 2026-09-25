// 13.12, 25 — THE MERGE: when two held thoughts are one, and what the
// one that remains carries.
//
//! Two bubbles merge when their forms are one section of the same bundle:
//! congruent up to a re-gauge, or with a torsion axis. A merge absorbs the
//! other's incidence, remainder and vortex; zero volume never merges.

use super::*;

/// 13.12: two forms are one section when their join coheres.
///
/// THE JOIN IS BY COORDINATES (AT1): a vertex both forms hold is one vertex
/// — the same class, its members the union of both forms' — and every
/// overlap of either stands at its own key, since it names its charts by
/// their coordinates and never by a position that would have to be shifted.
/// The tree gauge that re-solved the join's coboundary retired (AT1 step 2,
/// ruled (a)); an overlap's word is derived from its ends, so the join
/// coheres or it does not.
pub(crate) fn form_one_section(a: &Form, b: &Form) -> bool {
    if idea_key(a) == idea_key(b) {
        return true;
    }
    let mut f = a.clone();
    for (k, v) in b.vertices.entries() {
        match (f.vertices.at_mut(&k), v) {
            (Some(Vertex::Word { acts: held, .. }), Vertex::Word { acts, .. }) => {
                for (sk, member) in acts.entries() {
                    let mut member = member.clone();
                    if let Some(prior) = held.drop_at(&sk) {
                        for (fk, ()) in prior.frames.scoped() {
                            member.frames.insert(&fk, ());
                        }
                    }
                    held.put(&sk, member);
                }
            }
            (Some(_), _) => {}
            (None, _) => {
                f.vertices.put(&k, v.clone());
            }
        }
    }
    for (o, face) in b.faces.entries() {
        // The interface is the two bodies', and it crosses the merge with
        // the overlap that runs along it.
        f.faces.put(&o, face.clone());
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
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.reindex();
    f.coheres()
}

/// ABSORB IS THE WHOLE OF THE SOURCE, by the meaning of the word: two
/// bubbles merging carry every entry of each other's carriers across, and
/// an entry left behind is a fact the merged bubble has lost.
pub(crate) fn absorb_keyed<V: Clone, C: crate::network::nerve::Coordinate>(
    dst: &mut Keyed<V, C>,
    src: &Keyed<V, C>,
) {
    // BOTH CARRIERS ANSWER THE SAME COORDINATE OR THIS DOES NOT TYPECHECK
    // (54.7 R5). An absorb between two systems is what the type refuses,
    // and the key stays the coordinate it was across the move.
    for (k, v) in src.entries() {
        dst.put(&k, v.clone());
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
            if !fill.unsettled() {
                continue;
            }
            let vol_nz = volume.iter().any(|&x| x > 0);
            let outcome = self.fill_meet(&ck);
            match outcome {
                CoboundaryOutcome::Attached(Fill::Filled(gamma, carriage)) if vol_nz => {
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
                    // TC1 — the merge moves degrees: the changed components
                    // truncate to their 2-core.
                    let mut touched = self.touched_by(&vertices);
                    if let Some(m) = self.merge(&vertices) {
                        touched.push(m.as_bytes().to_vec());
                    }
                    self.truncate(&touched);
                    return;
                }
                CoboundaryOutcome::Attached(Fill::Congruent(kappa, g, ell)) if vol_nz => {
                    self.torsion_merges += 1;
                    // RM1 — the residue's chart, read before the merge takes
                    // the thoughts: where the remainder was perceived.
                    let at = self.shared_chain(&vertices);
                    // KB1 (C) — THE CLASS BETWEEN TWO THOUGHTS LIVES ON THIS
                    // COMPLEX AND ON THE TORSE REWRITE IT MINTS. It was filed a
                    // second time onto the merged idea's nerve between two
                    // bubble addresses that are no vertices after the merge,
                    // with no section to key by and read by nothing (law 12).
                    if let Some(nk) = self.merge(&vertices) {
                        let before: Vec<crate::transducer::address::Address> = vertices
                            .iter()
                            .map(|v| crate::transducer::address::Address::from(v.clone()))
                            .collect();
                        // A HOLONOMY Torse: its generator is the deficit ray
                        // `g`, whose order the fill read as ℓ (PD1).
                        let payload = crate::transducer::rewrite::RewritePayload::Torse {
                            generator: crate::transducer::element::Generator::Holonomy(g.clone()),
                            period: u64::from(ell),
                        };
                        // RM1 — THE REMAINDER ON THE REWRITE'S RECORD: the
                        // class it closed up to, the deficit as its ray, where
                        // it was perceived. The payload says σ and ℓ; the
                        // record is its evidence (law 12). Thoughts sharing no
                        // chart place no residue: counted.
                        let unsited = at.is_none();
                        let cycle = crate::transducer::element::TorsionCycle {
                            generator: crate::transducer::element::Generator::Holonomy(g),
                            period: u64::from(ell),
                            fibre: crate::transducer::element::FIBER_SPATIAL,
                            remainder: at.map(|at| crate::transducer::element::Remainder { class: kappa, at, answered: None }),
                        };
                        // STD1 (PH1) — the claim is the Torse's one yield,
                        // derived from the cycle it carries (`torse_yield`): one
                        // kind priced on one lane, as the court prices it.
                        let claimed = crate::transducer::rewrite::torse_yield(std::slice::from_ref(&cycle));
                        if let Some(mut rw) = crate::transducer::rewrite::mint(
                            crate::transducer::rewrite::RewriteKind::Torse,
                            &before,
                            nk.clone(),
                            payload,
                            claimed,
                        ) {
                            match unsited {
                                true => self.remainders.unsited += 1,
                                false => {
                                    match rw.record_cycle(cycle) {
                                        crate::transducer::element::Filed::Refused => self.remainders.refused += 1,
                                        crate::transducer::element::Filed::PastCarrier => {
                                            self.remainders.past_carrier += 1
                                        }
                                        crate::transducer::element::Filed::Misspoken => {
                                            self.remainders.misspoken += 1
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            self.pending_rewrites.push(rw);
                        } else {
                            // A name past its wall: no rewrite to record on.
                            self.remainders.unminted += 1;
                        }
                    } else {
                        // The merge named no thought (`merges.unnamed`): no
                        // `after` for the rewrite, the remainder unrecorded.
                        self.remainders.unmerged += 1;
                    }
                    return;
                }
                // Evaluated and still open, evaluated and not measured (by
                // its reason, on the simplex), already settled, or not there
                // to evaluate — four facts, none of which merges anything.
                CoboundaryOutcome::Attached(Fill::Open)
                | CoboundaryOutcome::Attached(Fill::Unread(_))
                | CoboundaryOutcome::AlreadyFilled
                | CoboundaryOutcome::Absent => {}
                // 37.7 at a simplex: a rotation the seventeen do not spell,
                // which merges nothing and seeds no horizon.
                CoboundaryOutcome::Attached(Fill::Unspellable(_)) => {}
                // The volume-zero halves of the two arms above.
                CoboundaryOutcome::Attached(Fill::Filled(..))
                | CoboundaryOutcome::Attached(Fill::Congruent(..)) => {}
                CoboundaryOutcome::Attached(Fill::Class(_, r)) => {
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
                    self.merges.stayed += 1;
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
        let mut torsion: Keyed<sentence::FiledOrbit, crate::network::nerve::coord::Holonomy> =
            Keyed::new();
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
            self.merges.unnamed += 1;
            return None;
        }
        let volume = match &form {
            Some(f) => form_mag(f),
            None => Some(element::measure_lanes(&remainder)),
        };
        // STD1 — a volume past its carrier names no bubble, counted.
        let Some(volume) = volume else {
            self.overflowed += 1;
            return None;
        };
        // A union of two remainders can say two things at one site, and
        // such a remainder has no A5 chain. The merge yields no bubble
        // rather than one that cannot be named (U14).
        let Some(remainder) = element::place_chain(&remainder) else {
            self.merges.unnamed += 1;
            return None;
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
            moved_k: 0,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion,
            certificates,
            vortex,
            candidates: Keyed::new(),
            escrow: volume,
            until: None,
            paid_from: None,
            rotor: form.as_ref().and_then(Bubble::rotor_of_form),
            form,
            bundle,
            remainders,
            cells_read: None,
        };
        self.admit_incidence(&key.shared(), &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        // The merged thought re-links with its re-gauged sections: the GAUGE
        // moved (78.26(6)).
        self.link_new(&key, super::meet::Adjunct::Gauge);
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
        self.merges.merged += 1;
        Some(key)
    }

}

/// 13.12, 78.26(6) — WHAT THE CONTEMPLATION'S MERGES CAME TO, counted where
/// each is decided: MERGED into one thought; STAYED two, the forms refusing
/// to be one section (the class the coordinate between them); UNNAMED, the
/// union naming no bubble (no remainder and no form, or a remainder with no
/// A5 chain). TP1 extends the same line with decohered and collapsed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MergeTally {
    pub merged: u64,
    pub stayed: u64,
    pub unnamed: u64,
}

impl MergeTally {
    pub fn line(&self) -> String {
        format!("thoughts merged {} stayed {} unnamed {}", self.merged, self.stayed, self.unnamed)
    }
}
