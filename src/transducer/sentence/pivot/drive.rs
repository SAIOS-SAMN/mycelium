// A16 — THE PIVOT ORBIT: the one move that opens a chart's axis, and the
// candidates it reads off the form.
//
//! Candidates are read from the axes already occupied, never enumerated. A
//! trial is applied and unapplied exactly; what it measures is the class
//! vector before and after, per lane.

use super::*;

/// 78.47(6) — THE NEGATIVE'S KIND, read off why the residue did not spell
/// (78.9): no versor, or a null step (no reflection exists there), is P; no
/// residue asks nothing, N; modes that do not split over ℚ are unclassified;
/// a construction that does not compose or meets off its locus is a claim
/// that does not re-derive, I.
fn not_constructed_kind(why: crate::transducer::cga::NotConstructed) -> crate::transducer::sentence::ObstructionKind {
    use crate::transducer::cga::NotConstructed;
    use crate::transducer::sentence::ObstructionKind;
    match why {
        NotConstructed::NotVersor | NotConstructed::NullStep { .. } => ObstructionKind::P,
        NotConstructed::NoResidue => ObstructionKind::N,
        NotConstructed::Undecomposed(_) => ObstructionKind::Unclassified,
        NotConstructed::DoesNotCompose | NotConstructed::MeetOffLocus { .. } => ObstructionKind::I,
    }
}

/// WHAT A TRIAL IS MEASURED AGAINST: the form's standing before any
/// candidate is applied — the orbit rows and the rank they carry, the
/// crystal the axes are read in, the bundle and its T extent, and the μ⃗ the
/// bundle stood at. Constant across every candidate of one stroke, which is
/// why it is read once and lent to each trial rather than passed six times.
pub(crate) struct Prior<'a> {
    // `rows` stood here and is gone with the substitution it existed for.
    // The trial reads the rank AFTER over every row under its own carrier,
    // so the rows BEFORE are not a thing it needs lent to it — only the
    // rank they add up to, which `rank_before` already carries. A field no
    // reader reads is a second carrier for a fact (law 12).
    pub(crate) rank_before: Rank,
    pub(crate) crystal: &'a dyn Crystal,
    pub(crate) bundle: Option<&'a crate::transducer::family::Family>,
    pub(crate) t_extent: u64,
    pub(crate) mu_before: Option<(Vec<[u64; 4]>, [u64; 4])>,
    /// 78.46(4′) — the certified cells' circuits by their based rays before
    /// any candidate: which routes agree, so a trial can say whether its
    /// move split two of them.
    pub(crate) agreements: super::Agreements,
}

/// THE CLASS VECTORS OF ONE ORBIT, ONE PER MEMBER PATH, read off the faces it
/// runs over and the form's axis carrier — the two things an orbit's class is a
/// function of. A trial that has written one face reads its own answer here
/// without a form to read it from.
///
/// The PLANE and OFF-PLANE coordinates come from the holonomy word and are the
/// same on every path, because a word is a vertex coordinate. The PLACEMENT and
/// DILATION residues come from the carriage and the hue cycle from the gauge,
/// both member coordinates, so they are read per path (D10, item 7). An opened
/// axis zeroes the orbit on every path: the holonomy IS the transition on that
/// axis and no longer a defect (I6, 15.5, T14).
pub(crate) fn class_vectors_on(
    faces: &Faces,
    vertices: &Vertices,
    axes: &crate::network::nerve::Keyed<Option<coord::Overlap>, coord::Holonomy>,
    lp: &Orbit,
) -> crate::network::nerve::Keyed<ClassVector> {
    let mut out = crate::network::nerve::Keyed::new();
    // An opened axis zeroes the orbit on its one member path (AT1: a cycle is
    // one path, keyed by its charts).
    if crate::transducer::sentence::opened_axis_on(faces, vertices, axes, lp).is_some() {
        out.insert(lp.path_key(), ClassVector::zero());
        return out;
    }
    // THE HOLONOMIC COORDINATE IS THE EDGE COMPOSITE'S, NOT THE WORD'S, and
    // reading it off the word made 12.6's `𝒽` identically zero on every orbit
    // this engine has ever perceived. A face's word is the coboundary
    // `w_j ∘ w_i⁻¹` of its two vertices' own words, and a product of
    // coboundaries around a cycle telescopes to the identity for EVERY input
    // — `orbit_transport_on` says so in its own comment ("the words are vertex
    // coordinates and telescope identically whichever bodies the path runs
    // through"). Measured over 837 pair forms: 22 140 orbits, 22 140 of them
    // flat on the word and not one of finite non-identity order, while the
    // edge composites over the same orbits close at finite non-identity order
    // on 93 paths. The curvature was there; the coordinate that decides was
    // reading the one place it cannot appear.
    //
    // The carriage and the gauge were already the member path's own. Only the
    // rotor was taken from the orbit.
    let words = holonomy_words_on(faces, vertices, lp);
    for (k, t) in orbit_transport_on(faces, vertices, lp).scoped() {
        let Some(w) = words.get(&k) else {
            continue;
        };
        if let Some(cv) = ClassVector::read(w, t.carriage.clone(), CycleType::of(&t.gauge)) {
            out.insert(&k, cv);
        }
    }
    out
}

/// THE HOLONOMY WORD EACH MEMBER PATH OF AN ORBIT CARRIES: the edge
/// composite's word where the path has one, the orbit's own word where it
/// does not.
///
/// ONE CARRIER FOR THE CHOICE (law 12). `class_vectors_on` made it and the
/// filing decision did not: `pivot_orbit` read `lp.holonomy` directly for the
/// horizon key and for the order that selects the `Discharge` branch, and
/// `file_orbit` filed torsion under `lp.holonomy.rotor().class_key()`. So the
/// class an orbit was JUDGED by came off the edges and the class it was FILED
/// under came off the word, and those are different objects.
///
/// A face's word is the coboundary `w_j ∘ w_i⁻¹` of its two vertices' words,
/// and a product of coboundaries around a cycle telescopes to the identity,
/// so an orbit over coboundary faces alone is flat on the word for every
/// input. Measured on the fleet at e98e8a2: 6535 pair forms, 70311 orbits,
/// word holonomy flat 70311 — all of them — against 284 finite non-identity
/// edge composites over the same orbits. The filer was keyed to the one
/// place the curvature cannot appear, so nothing was ever filed.
///
/// The word is NOT a constant by construction and is not removable: a
/// section face (21 step 2, S12.2) carries a word that is deliberately not
/// that coboundary, and an orbit over one is curved on the word while its
/// edges compose to the identity. Both carriers are real; the path decides
/// which it has.
pub(crate) fn holonomy_words_on(faces: &Faces, vertices: &Vertices, lp: &Orbit) -> crate::network::nerve::Keyed<Word> {
    let mut out = crate::network::nerve::Keyed::new();
    // One traversal, the choice read per path by `path_word`.
    let Some(carried) = paths_carried(faces, vertices, lp) else {
        return out;
    };
    for (k, c) in carried.scoped() {
        out.insert(&k, path_word(c));
    }
    out
}

/// THE HOLONOMY WORD ONE MEMBER PATH CARRIES: its edge composite's word where
/// the path has one, its own word where it does not — the choice
/// [`holonomy_words_on`] makes, stated once (law 12).
pub(crate) fn path_word(c: &PathCarried) -> Word {
    // THE FALLBACK IS THE PATH'S OWN WORD, NOT THE ORBIT'S (54.5, as ruled
    // 2026-09-20). It read `None => lp.holonomy.clone()`, and `lp.holonomy`
    // is `form_named_holonomy` over the orbit's PROJECTED vertex path — a
    // base-space object composed without the members the transport lives
    // on. The face word is the composite this path actually crossed, member
    // by member.
    //
    // The fallback itself stays, and stripping it was wrong: a section face
    // (21 step 2, S12.2) carries a word that is DELIBERATELY not the
    // coboundary `w_j ∘ w_i⁻¹`, so an orbit over one is curved on the word
    // while its edges compose to the identity. Both carriers are real and
    // the path decides which it has — removing either deletes an
    // obstruction the engine measured.
    match &c.edge {
        Some(e) => e.word.clone(),
        None => c.face.word.clone(),
    }
}

/// THE ONE HOLONOMY AN ORBIT HAS, where every member path agrees on it: the
/// edge composite wherever a path carries one, the orbit's own word where it
/// does not (R8's reading, applied to the whole `OpenAxis` pipeline).
///
/// `lp.holonomy` is the WORD, and a face's word is the coboundary
/// `w_j ∘ w_i⁻¹` of its two vertices' words. A product of coboundaries
/// around a cycle telescopes to the identity for EVERY input — `∏ (w_j ∘
/// w_i⁻¹) ≡ 1`, an exact Čech coboundary, `δw = 0` — so an orbit over
/// coboundary faces alone is flat on the word whatever it actually turns by.
/// Measured at e98e8a2: 70311 of 70311 orbits flat on the word against 284
/// finite non-identity edge composites over those same orbits.
///
/// `file_orbit` was keyed to the word and filed nothing until R8 moved it
/// here. The `OpenAxis` pipeline was keyed to the word too and refused every
/// widening for the same reason: its gate is `!lp.holonomy.is_identity()`,
/// which is false by algebra on a coboundary cycle. Measured at f3a76ec:
/// `opens 1` on 133 orbits — the single one whose word did not cancel —
/// against 92 torsion classes filed off the edge composite in the same five
/// minutes. The curvature and the gate were reading different carriers.
///
/// Agreement is PROJECTIVE equality of the rotors (1.5), not equality of the
/// tapes: `q` and `q·q·q` are one vertex spelled twice, and an orbit is not
/// two obstructions for having been reached two ways. `None` where the paths
/// disagree — the orbit then has no single holonomy and a caller that keyed a
/// carrier by one path's answer would be asserting the rest (rule 11).
/// AN ORBIT CARRYING NO MEMBER PATH ANSWERS THE WORD. `holonomy_words_on`
/// already rules that a path with no edge composite carries the word — "the
/// only holonomy it has is the word's, which is the reading of an orbit whose
/// faces are all coboundaries, not a default standing in for one" — and an
/// orbit with no path at all has no edge composite anywhere, so the same
/// reading applies with nothing to read it over. Returning `None` there made
/// the RANK unmeasured on every such form: `rank_offplane_is_its_own_lane`
/// went from a measured `(offplane 1, free 0)` to no rank at all.
pub(crate) fn orbit_holonomy_on(faces: &Faces, vertices: &Vertices, lp: &Orbit) -> Option<Word> {
    let ws = holonomy_words_on(faces, vertices, lp);
    let mut held: Option<Word> = None;
    for (_, w) in ws.scoped() {
        match &held {
            None => held = Some(w.clone()),
            Some(h) if !h.rotor().projective_eq(&w.rotor()) => return None,
            Some(_) => {}
        }
    }
    // Written as the two readings they are, not as a fallback: an orbit with
    // member paths answers what those paths agree on, and an orbit with none
    // answers its word, because an orbit with no path has no edge composite to
    // answer instead (rule 11 — this is not a default for a missing value).
    match held {
        Some(w) => Some(w),
        None => crate::transducer::sentence::orbit_word_on(faces, vertices, lp),
    }
}

impl Form {
    /// The class vector of orbit `idx` (12.6): plane coordinates from the
    /// oriented holonomy; placement and hue from the carriage and gauge
    /// composed around `lp.vertices` (A4). An orbit whose class is an axis
    /// the form opened reads as the zero vector: its holonomy is the
    /// transition on that axis and no longer a defect (I6, 15.5, T14).
    /// THE TRANSPORT COMPOSITE, which is one of an orbit's two obstructions
    /// and not the only one. `FormFace.edge` carries the other — 50.3's
    /// stabilizer element — and it is read by [`Form::edge_class_vector`].
    /// Measured: reading the edge HERE instead zeroed the class of the one
    /// orbit in the tree the pivot actually fires on (a turned L across two
    /// pairs, occupying `Unit(0)`), because an extraction's faces carry an
    /// edge whose composite is the identity while their transports carry
    /// the half turn. They are two objects and neither replaces the other.
    /// ONE PER MEMBER PATH (D10 item 7), keyed by the path.
    pub fn class_vectors(&self, idx: usize) -> crate::network::nerve::Keyed<ClassVector> {
        match self.orbits.get(idx) {
            Some(lp) => class_vectors_on(&self.faces, &self.vertices, &self.axes, lp),
            None => crate::network::nerve::Keyed::new(),
        }
    }

    /// THE ORBIT'S CLASS WHERE EVERY MEMBER PATH AGREES ON IT, which is what a
    /// caller asking "is this orbit a defect" is asking: an orbit is a defect if
    /// ANY path of it carries an unfiled class, and the readings differ only in
    /// the placement and dilation residues.
    ///
    /// `None` when the paths disagree — the orbit then has no single class and
    /// a caller that treated one path's answer as the orbit's would be making
    /// exactly the assertion D10 removes — or when no path closes.
    pub fn class_vector(&self, idx: usize) -> Option<ClassVector> {
        self.class_vector_of(self.orbits.get(idx)?)
    }

    /// [`Form::class_vector`] of an orbit held by the caller — the one it
    /// reached by its chord's coordinates (AT1), never by a position.
    pub fn class_vector_of(&self, lp: &Orbit) -> Option<ClassVector> {
        let vs = class_vectors_on(&self.faces, &self.vertices, &self.axes, lp);
        let mut held: Option<ClassVector> = None;
        for (_, cv) in vs.scoped() {
            match &held {
                None => held = Some(cv.clone()),
                Some(h) if h.class_key() != cv.class_key() => return None,
                Some(_) => {}
            }
        }
        held
    }

    /// THE HOLONOMY WORD EVERY MEMBER PATH OF ORBIT `idx` AGREES ON, by the
    /// same rule [`Form::class_vector`] collapses the class vectors: the
    /// paths' own readings, and absence where they differ.
    ///
    /// Agreement is projective equality of the rotors (1.5) and not equality
    /// of the tapes, because `q` and `q·q·q` are one vertex spelled twice and
    /// an orbit is not two obstructions for having been reached two ways.
    pub fn holonomy_word(&self, idx: usize) -> Option<Word> {
        orbit_holonomy_on(&self.faces, &self.vertices, self.orbits.get(idx)?)
    }

    /// Write a certified move onto the form — the chord face's transport,
    /// or an axis opened over the orbit — and re-read. Vertices are not
    /// written.
    pub fn apply_pivot(&mut self, cert: &PivotCertificate) {
        self.apply_move(&cert.by);
        self.orbits = Self::orbits_of(&self.faces, &self.vertices);
        self.reindex();
    }

    /// A16 on the form. Certificates and vortex live here.
    /// `bundle` is the pairs the idea germinated from; `t_extent` is how
    /// long the orbit has been held on the bubble's clock.
    pub fn pivot_orbit(
        &mut self,
        idea: impl AsRef<[u8]>,
        horizon: &Keyed<(Rotor, Order, Standing)>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
        bundle: Option<&crate::transducer::family::Family>,
        t_extent: u64,
    ) -> Vec<Snap> {
        self.pivot_orbit_read(idea, horizon, crystal, budget, bundle, t_extent, None, &mut super::PivotTally::default())
    }

    /// The same pivot, each orbit's outcome noted against the cells its
    /// circuit crosses where the caller read them (CS2, note only): the
    /// pivot applies exactly as it does without them.
    #[expect(clippy::too_many_arguments, reason = "the pivot's six inputs and CS2's reading beside them")]
    pub fn pivot_orbit_read(
        &mut self,
        idea: impl AsRef<[u8]>,
        horizon: &Keyed<(Rotor, Order, Standing)>,
        crystal: &dyn Crystal,
        budget: &mut Budget,
        bundle: Option<&crate::transducer::family::Family>,
        t_extent: u64,
        cells: Option<&super::PivotCells>,
        tally: &mut super::PivotTally,
    ) -> Vec<Snap> {
        let idea = idea.as_ref();
        // The meet is re-derived, never asserted: `by_address` is how A6 and
        // Mayer–Vietoris find the vertex carrying an address (law 2).
        self.reindex();
        // WHAT THIS FORM HAD ALREADY PUT TO THE MESH when the stroke began.
        // Read once, here, and not from `self.torsion` at each candidate
        // read: this stroke's own filings must not count as demands that
        // have gone unanswered, because nothing has had a stroke in which to
        // answer them. See `candidates_for`.
        let published = self.torsion.clone();
        let mut snaps = Vec::new();
        let mut handled: Keyed<(), coord::Overlap> = Keyed::new();
        loop {
            if budget.remaining() == 0 {
                break;
            }
            let cycles = Self::cycles_of(&self.faces, &self.vertices);
            self.orbits = cycles.kept.clone();
            if self.rank().is_none() {
                break;
            }
            // THE TREE GAUGE RETIRED (AT1, ruled (a)), AND WITH IT THE ORBITS
            // IT CLOSED. `Snap::Closed` read an orbit whose chord word the
            // gauge had flattened; nothing regauges a form now, a face's word
            // is derived from its ends, and an orbit is read as its overlaps
            // compose — a flat one is trivial at `orbits_of` and never reaches
            // here.
            let open: Vec<usize> = self
                .orbits
                .iter()
                .enumerate()
                .filter(|(i, _)| self.class_vector(*i).is_some_and(|k| !k.is_zero()))
                .map(|(i, _)| i)
                .collect();
            if open.is_empty() {
                break;
            }
            // An orbit whose holonomy is not a versor has no row (15.3): the
            // audit is unmeasured and the form is held, never counted
            // under a key of the rotor's bytes.
            let Some(rows) = self.orbit_rows() else {
                break;
            };
            let Some(rank_before) = rank_of_rows(&rows) else {
                break;
            };
            let mu_before: Option<(Vec<[u64; 4]>, [u64; 4])> = match bundle {
                None => None,
                Some(b) => crate::transducer::family::bundle_pressure(self, b, crystal, t_extent),
            };
            let prior = Prior {
                rank_before,
                crystal,
                bundle,
                t_extent,
                mu_before,
                agreements: self.read_agreements(&cycles, false),
            };
            let mut applied = false;
            for idx in open {
                if !budget.spend() {
                    break;
                }
                let Some(kappa) = self.class_vector(idx) else {
                    continue;
                };
                // AN ORBIT IS HANDLED BY ITS CHORD'S OVERLAP (AT1), never by
                // its position in the orbit list.
                let Some(orbit_k) = self.orbits.get(idx).map(|lp| lp.chord.clone()) else {
                    continue;
                };
                if handled.holds(&orbit_k) {
                    continue;
                }
                // The holonomy the member paths carry — the edge composite
                // wherever a path has one — is the class the orbit is judged
                // and filed by (law 12); `open` holds only orbits whose paths
                // agree on it, so `holonomy_word` answers one class.
                let Some(hw) = self.holonomy_word(idx) else {
                    continue;
                };
                let Some(rotor_key) = hw.rotor().class_key() else {
                    continue;
                };
                // A19 is about what the MESH has ruled: `Ruled` on the
                // horizon is the one standing on which another mind supplies
                // the word; `Held` is this mind's own obstruction (L24) and
                // suppresses nothing.
                let held = horizon
                    .get(&rotor_key)
                    .is_some_and(|(_, _, st)| matches!(st, Standing::Ruled));
                let order = hw.order();
                let cands = self.candidates_for(idx, &kappa, crystal, held, &published);
                // THE BASIS'S REACH on the orbit's occupied axes: the words
                // `crystal.on_axis` answered. Where none did, the residue is
                // outside the span and 37.7 adjoins from it below.
                let held_gens = cands
                    .iter()
                    .filter(|c| matches!(c.axis, Axis::Unit(_) | Axis::Prime5(_) | Axis::Prime13(_) | Axis::OffPlane))
                    .count();
                let mut starved = false;
                let mut discharging: Vec<PivotCertificate> = Vec::new();
                for cand in &cands {
                    // The zero-spend eliminator: whether a held word can
                    // cancel this orbit's rotor is exact arithmetic in the
                    // even subalgebra and spends nothing. It eliminates and
                    // never decides — a candidate it admits still runs the
                    // whole trial, which reads the carriage and the gauge no
                    // rotor product sees (12.6, 16.16 step 4). Only the
                    // `Word` arm is eliminable.
                    if let PivotBy::Word { word, side, .. } = &cand.by {
                        // The test is conjugacy (15.3): writing `w` on either
                        // side closes `A·e·B` only if `w` is conjugate to `hw`
                        // or its inverse, and a rotor and its inverse stand at
                        // one conjugacy stratum, so one keyed compare serves
                        // both sides.
                        let _ = side;
                        if let (Some(want), Some(got)) =
                            (hw.rotor().class_key(), word.rotor().class_key())
                        {
                            if want != got {
                                continue;
                            }
                        }
                    }
                    if !budget.spend() {
                        starved = true;
                        break;
                    }
                    if let Ok(cert) = self.trial(idx, cand, &kappa, &prior) {
                        discharging.push(cert);
                    }
                }
                // 37.7 from the orbit's own residue: the basis reached no
                // coordinate this orbit occupies, the class was published and
                // nothing answered, so the rotor the orbit measured is carried
                // out to be adjoined — beside the widening, never instead of
                // it. Minting is the entity's, since a form does not hold the
                // language; it stands in the dialect, `Stood` not `Ruled`, so
                // it suppresses no demand the mesh may still answer.
                let adjoining = held_gens == 0
                    && !held
                    && !hw.rotor().is_identity()
                    && hw.rotor().holonomy_key().is_some_and(|k| published.at(&k).is_some());
                if adjoining {
                    // Every chart the orbit stands on (rule 15): an orbit through
                    // vertices on two charts is an obstruction on both.
                    for chart in self.charts_of_orbit(idx) {
                        snaps.push(Snap::Adjoin {
                            chord: idx,
                            rotor: Box::new(hw.rotor()),
                            chart,
                        });
                    }
                }
                // One certificate per move (`one_per_move`): a word
                // discharging on both sides of the chord is one move spelled
                // twice, never a vortex of itself.
                let moves = one_per_move(&discharging);
                let outcome = outcome_of(&moves, starved);
                // CS2 — the outcome read against the cells its circuit
                // crosses, noted and never branched on.
                if let Some(cells) = cells {
                    super::cells::note_outcome(cells, &orbit_k, &outcome, adjoining, tally);
                }
                // THE RESIDUE: the orbit's whole transport, read once where no
                // candidate discharged it — the demand's content and the
                // undischarged orbit's kind are both read off it.
                let residue = match outcome {
                    Discharge::NoneDischarged => self.orbits.get(idx).and_then(|lp| {
                        paths_carried_quiet(&self.faces, &self.vertices, lp)?.scoped().find_map(|(_, c)| c.composite())
                    }),
                    _ => None,
                };
                // 78.37 — THE DEMAND ARM'S DEMAND, carried to the docket: the
                // orbit's undischarged class and its cover, where no held word
                // discharged it and none adjoins — and, A19, where the court
                // has not ruled the class: another mind can supply that word,
                // so this one does not demand it. 78.47 — DEDUCTION CONSTRUCTS:
                // the demand carries its residue, and the constructors around
                // its congruence factors are read off it here only to name
                // what does not spell — the negative, refused with the class
                // and counted by kind. They are never carried beside the
                // residue that derives them (law 12).
                if matches!(outcome, Discharge::NoneDischarged) && !adjoining && !held {
                    let spelled = match &residue {
                        Some(r) => crate::transducer::cga::construct(r, budget).map(|_| ()),
                        None => Err(crate::transducer::cga::NotConstructed::NotVersor),
                    };
                    if let Err(why) = spelled {
                        crate::observe::note_obstruction(not_constructed_kind(why));
                    }
                    snaps.push(Snap::Demanded {
                        chord: idx,
                        class: Box::new(kappa.clone()),
                        charts: self.charts_of_orbit(idx),
                        residue: residue.clone(),
                    });
                }
                match outcome {
                    Discharge::One(cert) => {
                        self.apply_pivot(&cert);
                        // A18, T21 (iii): the applied pivot makes the move
                        // its certificate claims, certified here (law 10).
                        // On disagreement the orbit holds — no snap, no
                        // certificate — and nothing downstream rests on an
                        // uncertified move.
                        if self.rank().map(|r| Rank::moved(rank_before, r)) != Some(cert.rank_move) {
                            eprintln!(
                                "  pivot {:02x}{:02x}… orbit {idx} held, the applied pivot did not make the move its certificate claims",
                                idea[0], idea[1]
                            );
                            break;
                        }
                        let ck = cert.key();
                        self.certificates.insert(ck, (*cert).clone());
                        snaps.push(Snap::Discharged {
                            chord: idx,
                            certificate: cert,
                        });
                        handled.put(&orbit_k, ());
                        applied = true;
                        break;
                    }
                    Discharge::Vortex(held_certs) => {
                        for c in &held_certs {
                            vortex_put(&mut self.vortex, c.clone());
                        }
                        if prior.mu_before.is_some() {
                            let between = obstructions(&held_certs);
                            snaps.push(Snap::Vortex {
                                chord: idx,
                                held: held_certs,
                                between,
                            });
                        } else {
                            snaps.push(Snap::Unmeasured {
                                chord: idx,
                                held: held_certs,
                            });
                        }
                        handled.put(&orbit_k, ());
                    }
                    // THE STROKE ENDED, NOT THE SEARCH. Nothing is spoken,
                    // filed or widened on an evaluation that did not
                    // happen: the orbit is held and the next stroke reaches
                    // the candidates this one could not.
                    Discharge::Starved => {
                        handled.put(&orbit_k, ());
                    }
                    Discharge::NoneDischarged => {
                        // A.17, 78.9 — NO CANDIDATE DISCHARGES THE ORBIT: one
                        // fact, counted once, its kind read off the orbit's
                        // own holonomy by the one reader — no versor P, a
                        // versor the dialect does not spell D, a residue on
                        // the cover C. The candidates' own refusals were the
                        // trial's returns, never counted (law 12).
                        crate::observe::note_undischarged(
                            crate::transducer::sentence::read_obstruction(crate::transducer::sentence::Content::Holonomy {
                                versor: residue.as_ref(),
                                chart: None,
                                proof: "no candidate discharges the orbit",
                            })
                            .kind(),
                        );
                        let infinite = matches!(order, Order::Infinite);
                        if infinite && !held {
                            // One standing demand per chart the orbit
                            // stands on, carrying the holonomy rotor as
                            // its witness (16.16 step 6): the language's
                            // want of a generator, spoken beside the
                            // filing below and never instead of it (52.3).
                            for chart in self.charts_of_orbit(idx) {
                                snaps.push(Snap::Demand {
                                    chord: idx,
                                    key: rotor_key.clone(),
                                    rotor: Box::new(hw.rotor()),
                                    chart,
                                });
                            }
                        }
                        // 54.5, DIRECTIVE 4 — A CIRCUIT THAT NEVER CLOSES MOUNTS
                        // ITS SHEAR AS A BOUNDARY CELL.
                        //
                        // A screw composes to `g = T(τ)·R` with
                        // `τ ∈ ker(I − R)`, so `g^ℓ = T(ℓτ)` and no power
                        // returns: there is no period that closes it and no
                        // filing that can. `edge_coheres` refuses such a
                        // orbit unless the form DECLARES it, and this is
                        // what declares it — the obstruction converted from
                        // something hidden in a side-table into a vertex in
                        // the key, saying what the shear is and what would
                        // close it.
                        //
                        // ONE PER CHART, AND THE SLIP IS THE CHART'S OWN. A
                        // Burgers vector is coordinatized by the local
                        // frame: under a transition `g_AB = D(ρ)·T(t)·R`
                        // the slip transforms as `τ_B = ρ·R(τ_A)`, so it is
                        // not an ambient invariant and a declaration on one
                        // chart does not answer for another. 50.5 already
                        // stands one demand per `(kind, chart)`, and
                        // `demand_key` keys on both, so each boundary the
                        // orbit fails to close gets its own cell.
                        //
                        // A ZERO TWIST MOUNTS NOTHING: that circuit RETURNS
                        // and is a congruence, and a demand for it would be
                        // a coordinate standing for no obstruction
                        // (T21(iii), `Demand::typed`).
                        for (chart, tau) in self.orbit_slips(idx) {
                            self.stand_demand(
                                crate::transducer::sentence::Demand::Dislocation {
                                    chart: chart.clone(),
                                    tau: Box::new(tau.clone()),
                                },
                                &chart,
                                Vec::new(),
                            );
                        }
                        // An orbit of any order files under its class at its
                        // period, `0` for infinite order (52.3, A19).
                        //
                        // THE PERIOD IS THE FILER'S, NOT THIS CALLER'S. It
                        // passed `order.to_u32()` — the ROTOR'S order — and
                        // a quarter turn carried along a translation has
                        // the same rotor order as one about a fixed point
                        // while closing at neither: the screw slips. The
                        // filer solves for the fixed point exactly and
                        // answers what each member path actually closes at,
                        // and the snap now carries THAT (54.3).
                        let periods = self.file_orbit(idx);
                        if !periods.is_empty() {
                            if !held {
                                for ell in &periods {
                                    snaps.push(Snap::TorsionFiled { chord: idx, ell: *ell });
                                }
                            }
                        } else {
                            snaps.push(Snap::TorsionOpen { chord: idx });
                        }
                        handled.put(&orbit_k, ());
                    }
                }
            }
            if !applied {
                break;
            }
            handled = Keyed::new();
        }
        snaps
    }

    /// The form says out loud what it could not close (50.5): one demand
    /// vertex per (kind, chart), the bodies it stands for gathered under
    /// it. A vertex already standing for that kind on that chart takes the
    /// bodies; a second vertex under one key would say one thing twice.
    pub fn stand_demand(
        &mut self,
        kind: crate::transducer::sentence::Demand,
        chart: &stalk::Invariant,
        links: Vec<crate::transducer::sentence::Link>,
    ) {
        // The one filer of a demand's bodies (AT1): the vertex re-files at
        // its new coordinates when its chain grows.
        let mut links = links;
        links.sort_unstable();
        links.dedup();
        self.file_demand(kind, chart, &links);
        self.reindex();
    }

    /// 54.5, Directive 4 — THE SHEARS AN ORBIT LEAVES, ONE PER CHART.
    ///
    /// `screw_of` splits a transport exactly over ℚ by the normal equations
    /// `AᵀA p = Aᵀt`: the centre is where the bend holds still, on
    /// `Im(I − R)`, and the twist is what no choice of origin absorbs, on
    /// `ker(I − R)`. The twist is the Burgers vector — the circuit slips by
    /// exactly that much per turn — and it is the quantity a
    /// `Demand::Dislocation` carries.
    ///
    /// A TWIST IS READ IN A FRAME, so this pairs each with the chart it was
    /// measured on. Under a transition `g_AB = D(ρ)·T(t)·R` a slip
    /// transforms as `τ_B = ρ·R(τ_A)`; carrying one τ for an orbit that
    /// crosses several charts would assert an ambient vector the geometry
    /// does not have.
    ///
    /// Only NON-ZERO twists are returned. A zero twist is a circuit that
    /// returns — a congruence, filed at its period — and it owes no
    /// declaration (T21(iii): a coordinate is earned by retiring an
    /// obstruction, never by naming one that is not there).
    fn orbit_slips(&self, idx: usize) -> Vec<(stalk::Invariant, [crate::core::Q; 3])> {
        let Some(lp) = self.orbits.get(idx) else {
            return Vec::new();
        };
        let Some(hs) = self.edge_holonomies(lp) else {
            return Vec::new();
        };
        let charts = self.charts_of_orbit(idx);
        let mut out = Vec::new();
        for (_, h) in hs.scoped() {
            let Some(screw) =
                crate::transducer::element::screw_of(&h.word.rotor(), &h.carriage)
            else {
                continue;
            };
            if screw.twist.iter().all(num_traits::Zero::is_zero) {
                continue;
            }
            for chart in &charts {
                out.push((chart.clone(), screw.twist.clone()));
            }
        }
        out
    }

    /// Every chart an orbit stands on, once each, in key order: the charts of
    /// its vertices, never one of them by position (rule 15).
    pub(crate) fn charts_of_orbit_at(&self, idx: usize) -> Vec<stalk::Invariant> {
        self.charts_of_orbit(idx)
    }

    fn charts_of_orbit(&self, idx: usize) -> Vec<stalk::Invariant> {
        let mut at: Keyed<stalk::Invariant> = Keyed::new();
        if let Some(lp) = self.orbits.get(idx) {
            for vi in lp.vertices() {
                let Some(v) = self.vertices.at(&vi) else {
                    continue;
                };
                for chart in v.charts() {
                    at.insert(chart.key(), chart);
                }
            }
        }
        at.scoped().map(|(_, c)| c.clone()).collect()
    }

    /// The candidate pivots of a held orbit (16.16 step 3), read off its
    /// class vector in the fixed order, each instantiated from what the
    /// crystal holds. `held` is the horizon's word: the mesh holds this
    /// class already (A19), so the mind neither demands it nor opens an
    /// axis on it; it is still discharged by a held form when one arrives.
    /// `published` is what this form had FILED BEFORE THIS STROKE BEGAN — a
    /// parameter and not a read of `self.torsion`, because `file_orbit`
    /// files by class and a second orbit of one class would otherwise read
    /// its own stroke's filing as an unanswered demand and widen on it. A
    /// demand nobody has had a stroke to answer is not an unanswered demand.
    pub(crate) fn candidates_for(
        &self,
        idx: usize,
        kappa: &ClassVector,
        crystal: &dyn Crystal,
        held: bool,
        published: &Keyed<FiledOrbit, crate::network::nerve::coord::Holonomy>,
    ) -> Vec<Candidate> {
        let Some(lp) = self.orbits.get(idx) else {
            return Vec::new();
        };
        let chord = lp.chord.clone();
        if !self.faces.holds(&chord) {
            return Vec::new();
        }
        let mut charts: Keyed<stalk::Invariant> = Keyed::new();
        for s in &lp.steps {
            for vi in [s.overlap.from_vertex(), s.overlap.to_vertex()] {
                if let Some(v) = self.vertices.at(&vi) {
                    // EVERY CHART THE VERTEX STANDS AT, so a vertex acting on
                    // two contributes both and the orbit's chart set is the
                    // whole of what the orbit passes through.
                    for chart in v.charts() {
                        charts.insert(chart.key(), chart);
                    }
                }
            }
        }
        fn side_slot(side: Side) -> [u8; 32] {
            let mut k = [0u8; 32];
            k[0] = match side {
                Side::Left => 0,
                Side::Right => 1,
            };
            k
        }
        let mut seen: Keyed<Keyed<()>> = Keyed::new();
        let mut out = Vec::new();
        let mut reached = false;
        for axis in kappa.coordinates() {
            match axis {
                Axis::Placement => {
                    // THE SLIP, NOT THE WHOLE TRANSLATION (54.4, as ruled
                    // 2026-09-20). This read `translator_of(&kappa.carriage)`
                    // — `t` entire — and `t = t_im ⊕ τ` where `t_im` is the
                    // part a choice of origin absorbs. Writing `t_im` onto a
                    // face drags the CENTRE of the rotation across the chart
                    // and moves the invariant plane with it; the obstruction
                    // is `τ` alone, which acts on `ker(I − R)` and leaves the
                    // centre standing. The aim was unsplittable while the
                    // placement was an enum.
                    let Some(translator) = kappa.slip_translator() else {
                        continue;
                    };
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Placement {
                            face: chord.clone(),
                            translator,
                        },
                    });
                }
                Axis::Dilation => {
                    let Some(translator) = element::dilator_of(&kappa.carriage) else {
                        continue;
                    };
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Placement {
                            face: chord.clone(),
                            translator,
                        },
                    });
                }
                Axis::Orientation(_) => {
                    out.push(Candidate {
                        axis,
                        by: PivotBy::Word {
                            face: chord.clone(),
                            word: Box::new(crate::transducer::alphabet::PlaneForm::sheet_word()),
                            side: Side::Right,
                        },
                    });
                }
                // ONE HUE CANDIDATE PER MEMBER PATH. The gauge the move
                // carries is the orbit's composite hue map, which is a member
                // coordinate, so an orbit running through two pairs of bodies
                // offers the move each of them names. Offering one and
                // skipping the rest is a move never offered on the others.
                Axis::Hue => {
                    for (_, t) in orbit_transport(self, lp).scoped() {
                        out.push(Candidate {
                            axis,
                            by: PivotBy::Hue {
                                face: chord.clone(),
                                gauge: t.gauge.clone(),
                            },
                        });
                    }
                }
                _ => {
                    // THE BASIS REACHED THIS COORDINATE, recorded because the
                    // expansion below is offered on exactly its negation.
                    // Set where the crystal ANSWERS, not where it is asked: a
                    // chart with no word on this axis has not reached it.
                    // `on_axis_of` is an EXACT keyed read, asked under the
                    // vertices' `acts` charts.
                    for (_, chart) in charts.scoped() {
                        for w in crystal.on_axis(chart, axis) {
                            reached = true;
                            for side in [Side::Right, Side::Left] {
                                let wk = w.key();
                                let sk = side_slot(side);
                                if seen.get(&wk).is_some_and(|s| s.contains(&sk)) {
                                    continue;
                                }
                                seen.nested_insert(wk, sk, ());
                                out.push(Candidate {
                                    axis,
                                    by: PivotBy::Word {
                                        face: chord.clone(),
                                        word: Box::new(w.clone()),
                                        side,
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
        // THE OPENING AXIS (12.6, 16.16 step 3, 3.7; I6): an orbit whose
        // class has a free part, or whose class was published and not
        // answered while the basis reaches none of its coordinates, opens a
        // chart axis seeded by its own holonomy, and the class becomes a
        // coordinate of the form the court re-derives from the orbit. The
        // condition is the fibre's, `kappa.free_part()`, never another
        // candidate's absence — 12.6 offers the candidates and orders
        // nothing. The order is reduce, publish, then widen: `asked` reads
        // `published`, the filings as they stood before this stroke, keyed
        // by the member paths' own classes (law 12); a finite-order orbit no
        // word reaches files first (step 6, A17) and widens once the mesh
        // has not answered. The gate and the seed are the edge composite's
        // (`orbit_holonomy_on`), never the word's, which telescopes on any
        // coboundary cycle. The rank audit admits the move as it stands:
        // `OrbitRow::Opened` moves `torsion −1, axes +1`.
        let asked = |h: &Word| {
            h.rotor()
                .holonomy_key()
                .is_some_and(|k| published.at(&k).is_some())
        };
        if let Some(h) = orbit_holonomy_on(&self.faces, &self.vertices, lp) {
            if !held && !h.is_identity() && (kappa.free_part() || (!reached && asked(&h))) {
                // The seed is the holonomy's sensor action, one plane by
                // construction (51.2), while the axis is keyed by the whole
                // rotor's class.
                if let Some(seed) = element::versor_of_word(&h) {
                    out.push(Candidate {
                        axis: Axis::Open,
                        by: PivotBy::OpenAxis {
                            face: chord.clone(),
                            seed,
                        },
                    });
                }
            }
        }
        out
    }

}
