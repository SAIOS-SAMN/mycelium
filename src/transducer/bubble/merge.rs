// 13.12, 25 — THE MERGE: when two held thoughts are one, and what the
// one that remains carries.
//
//! Two bubbles merge when the bubbles' forms are one section of the same bundle:
//! congruent up to a re-gauge, or with a torsion axis. A merge absorbs the
//! other's incidence, remainder and vortex; zero volume never merges.

use super::*;

/// 13.12: two forms are one section when the forms' join coheres.
///
/// THE JOIN IS BY COORDINATES (AT1): a vertex both forms hold is one vertex
/// — the same class, its members the union of both forms' — and every
/// overlap of either stands at its own key, since it names its charts by
/// the charts' coordinates and never by a position that would have to be shifted.
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
    /// Two bubbles on one simplex: the fill decides (78.48(5)). Filled
    /// merges and re-gauges; Congruent merges with a torsion axis; Class
    /// keeps both and writes each into the other's horizon. Remainder volume
    /// is what two thoughts share unexplained — a 2-cell's measure, never
    /// the edge's decision: under a volume gate a perfect match never merged.
    pub(crate) fn decide_neighbors(&mut self, key: &[u8]) {
        let keys = self.complex.link(key, 1);
        for ck in keys {
            let Some((fill, vertices)) = self.complex.simplex(&ck).map(|c| (c.fill.clone(), c.vertices.clone())) else {
                continue;
            };
            if !fill.unsettled() {
                continue;
            }
            let outcome = self.fill_meet(&ck);
            match outcome {
                CoboundaryOutcome::Attached(Fill::Filled(gamma, carriage)) => {
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
                    // truncate to the 2-core.
                    let mut touched = self.touched_by(&vertices);
                    if let Some(m) = self.merge(&vertices) {
                        touched.push(m.as_bytes().to_vec());
                    }
                    self.truncate(&touched);
                    return;
                }
                CoboundaryOutcome::Attached(Fill::Congruent(kappa, g, ell)) => {
                    self.torsion_merges += 1;
                    // RM1 — the residue's chart, read before the merge takes
                    // the thoughts: where the remainder was perceived.
                    let at = self.shared_chain(&vertices);
                    // 78.52: does this merge close the thought's return? Read
                    // before the merge takes the thoughts and the thoughts' grounds.
                    let closing = self.closing_ground(&vertices);
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
                        if let Some(ground) = closing {
                            self.close_thought(ground, nk.as_bytes());
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

    /// 78.52 — THE GROUNDS OF THE THOUGHTS THAT CLOSED, drained for the voice
    /// to transcribe and speak in the act that closed them.
    pub(crate) fn take_closed(&mut self) -> Vec<(crate::physics::tensegrity::Ground, Vec<u8>)> {
        std::mem::take(&mut self.closed)
    }

    /// MU-CLIENT §4.4(1) — THE TWISTS the thoughts that came round again
    /// carry, drained for the voice to transcribe at its return.
    pub(crate) fn take_twists(&mut self) -> Vec<crate::transducer::cga::Mv> {
        std::mem::take(&mut self.twists)
    }

    /// 78.52, MU-CLIENT §4.4(2) — DOES THIS MERGE COMPLETE A RETURN? The
    /// pair's ground, read at the gate, carries every hour the merged thought
    /// will have seen; its returns are those hours over its period. Where the
    /// merge completes a return no thought merged had completed — the
    /// constellation came round, every hour admitted balanced with its
    /// momentum carried — the ground goes to the voice: its FIRST return is
    /// the thought's first close, new structure; each later one is a
    /// TRAVERSAL the winding completed, where the voice says only what changed
    /// (a word its letters now spell that they did not before).
    fn closing_ground(&self, vertices: &[Vec<u8>]) -> Option<(super::idea::Grounded, bool)> {
        let [a, b] = vertices else { return None };
        let pair = self.grounds.get(&super::resonance::pair_key(a, b))?;
        let period = pair.ground.period;
        let returns = pair.hours.checked_div(period)?;
        if returns == 0 {
            return None;
        }
        let returned = |v: &Vec<u8>| self.bubbles.get(v).and_then(|t| t.ground.as_ref()).and_then(|g| g.hours.checked_div(period));
        if vertices.iter().any(|v| returned(v).is_some_and(|r| r >= returns)) {
            return None;
        }
        let first = vertices.iter().all(|v| returned(v).is_none_or(|r| r == 0));
        Some((pair.clone(), first))
    }

    /// 78.52 — A THOUGHT THAT CAME ROUND WAITS FOR ITS VOICE: its ground stays
    /// the mind's; the voice transcribes it against its own letters at the
    /// return (`Entity::transcribe_closed`), and its slips are worked there
    /// (`work_returned`). Counted as a first close or as a later return of
    /// its winding.
    fn close_thought(&mut self, (grounded, first): (super::idea::Grounded, bool), thought: &[u8]) {
        // MU-CLIENT §4.4(1) — a later return says what changed: the twist its
        // pull has put on the orbit, transcribed by the voice beside the
        // thought's own word.
        if let (false, Some(twist)) = (first, &grounded.twist) {
            self.twists.push(twist.clone());
        }
        // E1 — the scene the thought was perceived in rides with its ground,
        // so its word is spoken there.
        let family = match self.bubbles.get(thought) {
            Some(b) => b.family.clone(),
            None => Vec::new(),
        };
        self.closed.push((grounded.ground, family));
        self.came_round.push(thought.to_vec());
        match first {
            true => self.closed_thoughts += 1,
            false => self.thought_returns += 1,
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
                    self.merges.stayed += 1;
                    #[expect(
                        clippy::disallowed_methods,
                        reason = "rule 15 — canonical key order: the simplex's vertices are sorted, so the least key stands where the merge is refused"
                    )]
                    return verts.first().cloned().map(crate::transducer::address::Address::from);
                }
            }
        }
        // 78.99 (4) — THE GLUE MAKES MEANING: the two thoughts' stars on the
        // meet graph glued by the connecting map, read before the thoughts' vertices
        // are vented. Each loop through a neighbour both thoughts meet is a
        // class neither star held alone, and it enters the merged thought's
        // horizon as a class the meet stands — every route kept, none
        // multiplied with another.
        let mut made: Vec<Rotor> = Vec::new();
        if let [a, b] = verts {
            if let Ok(g) = crate::transducer::cech::connecting(&self.complex.star_of(a), &self.complex.star_of(b)) {
                made.extend(g.made.into_iter().filter_map(|(_, _, fill)| match fill {
                    Fill::Class(_, r) => Some(*r),
                    _ => None,
                }));
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
        let mut ground: Option<super::idea::Grounded> = None;
        let mut slips: Vec<(Vec<u8>, crate::transducer::cga::Mv)> = Vec::new();
        let mut bundle = None;
        let mut remainders: Keyed<Vec<Link>> = Keyed::new();
        let mut old_rems: Keyed<Vec<Link>> = Keyed::new();
        let mut merged_volume = [0u64; 4];
        // ESCROW CONSERVED (MU-CLIENT §7.1 Phase 3): what each merged thought
        // holds is carried whole into the merge, never re-drawn and never its
        // volume.
        let mut escrow = [0u64; 4];
        // THE THOUGHT STANDS ON THE BODIES IT WAS FORMED OF (78.48(5)): its
        // perceptions' bodies by chart, IN THE ORDER THEY ARRIVED — each
        // idea's ordinal counts admissions, written big-endian so the keyed
        // read is that order — which is the orbit's order of hours (78.48(7)).
        let mut arrived: Keyed<super::resonance::Spores> = Keyed::new();
        // 78.56 III — the thought is of the family its perceptions were seen
        // in: they met only within it, so they share it.
        let mut family: Vec<u8> = Vec::new();
        for v in verts {
            let Some(b) = self.bubbles.get(v.as_slice()).cloned() else {
                continue;
            };
            family.clone_from(&b.family);
            if let Some(o) = self.ordinals.get(v.as_slice()) {
                arrived.insert([&o.to_be_bytes()[..], v.as_slice()].concat(), b.spores.clone());
            }
            old_rems.insert(v, b.remainder.to_vec());
            for (m, x) in merged_volume.iter_mut().zip(&b.volume) {
                *m = (*m).max(*x);
            }
            if !crate::core::lanes::lanes_step(&mut escrow, &b.escrow) {
                self.overflowed += 1;
            }
            self.unthreaded.remove(v);
            self.unheld.remove(v);
            self.retract_incidence(v, &b);
            remainder = union_rem(&remainder, &b.remainder);
            absorb_keyed(&mut cover, &b.cover);
            absorb_keyed(&mut horizon, &b.horizon);
            absorb_keyed(&mut torsion, &b.torsion);
            absorb_keyed(&mut certificates, &b.certificates);
            absorb_vortex(&mut vortex, &b.vortex);
            absorb_keyed(&mut remainders, &b.remainders);
            // 78.52(6) Stage C — THE FIRST-ARRIVAL FORM IS PURGED: a thought
            // is not the form of whichever perception came first. It holds
            // its ground, adopted below, and is filed by it.
            if ground.is_none() {
                ground = b.ground.clone();
            }
            if bundle.is_none() {
                bundle = b.bundle.clone();
            }
            // The slips of what merged are the merged thought's, still waiting
            // on its return.
            slips.extend(b.slips.iter().cloned());
            nerve = b.nerve.clone();
            self.bubbles.remove(v);
        }
        for r in made {
            if r.class_key().is_none_or(|ck| !horizon.contains(ck.as_bytes())) {
                horizon_put(&mut horizon, r, Standing::Held);
            }
        }
        self.paint_seeded(&mut horizon);
        // 78.52(6) — THE PAIR'S GROUND, read at the gate with the hours the
        // merged thought has now seen, is the thought's ground from here on.
        if let [a, b] = verts {
            let pk = super::resonance::pair_key(a, b);
            if let Some(pair) = self.grounds.get(&pk).cloned() {
                self.grounds.remove(&pk);
                ground = Some(pair);
            }
        }
        // A.10.1 — THE CONSTELLATION JOINED: a frame the gate admitted joins
        // the thought's frames, in the order it arrived. A body the thought
        // already holds — the same coordinates, as an orbit's hour `ℓ` is its
        // hour 0 — is the same frame and is never held twice: a thought's
        // frames on an orbit are bounded by one return (78.43), and its
        // address with them (the epoch-105 growth).
        let mut spores: super::resonance::Spores = Keyed::new();
        let mut held: Keyed<()> = Keyed::new();
        for (_, bodies_by_chart) in arrived.scoped() {
            for (chart, bodies) in bodies_by_chart.scoped() {
                for b in bodies {
                    let mut at = Vec::new();
                    crate::transducer::address::put_key_bytes(&mut at, &chart);
                    crate::transducer::address::put_key_bytes(&mut at, &super::resonance::body_coordinates(b));
                    if held.contains(&at) {
                        continue;
                    }
                    held.insert(&at, ());
                    match spores.get_mut(&chart) {
                        Some(on) => on.push(b.clone()),
                        None => {
                            spores.insert(&chart, vec![b.clone()]);
                        }
                    }
                }
            }
        }
        if remainder.is_empty() && spores.is_empty() && ground.is_none() {
            self.merges.unnamed += 1;
            self.return_escrow(&escrow);
            return None;
        }
        // THE THOUGHT'S VOLUME is the largest of what merged, lane by lane —
        // never the first form's (purged), never a sum that grows with every
        // hour joined.
        let volume = merged_volume;
        // A union of two remainders can say two things at one site, and
        // such a remainder has no A5 chain. The merge yields no bubble
        // rather than one that cannot be named (U14).
        let Some(remainder) = element::place_chain(&remainder) else {
            self.merges.unnamed += 1;
            self.return_escrow(&escrow);
            return None;
        };
        let bubble = Bubble {
            remainder,
            nerve,
            cover,
            horizon,
            potential: volume,
            volume,
            dirty: false,
            // A merge is a thought, formed of what it merged (N4).
            percept: false,
            spores,
            family,
            moved_k: 0,
            glues: Keyed::new(),
            glued_by: Keyed::new(),
            torsion,
            certificates,
            vortex,
            candidates: Keyed::new(),
            escrow,
            rotor: None,
            form: None,
            bundle,
            remainders,
            cells_read: None,
            ground,
            slips,
        };
        // THE ONE DERIVATION of where a bubble is filed (`Bubble::key`): a
        // grounded thought at its word's coordinates, else by its bodies.
        let key = bubble.key();
        self.admit_incidence(&key.shared(), &bubble);
        self.bubbles.insert(&key, bubble);
        self.dirty.insert(&key, ());
        // A merged thought that had come round is the thought that came
        // round, now under the merge's key (MU-CLIENT §4.4(4)).
        for v in verts {
            self.follow_came_round(v, key.as_bytes());
        }
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
